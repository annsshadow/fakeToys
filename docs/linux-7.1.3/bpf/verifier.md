
## eBPF 鏍￠獙鍣。
eBPF 程序的安全性分两步确定
第一步进DAG（有向无环图）检查，以禁止循环和其他 CFG（控制流图）校验。特是它会检测出含有不可达指令的程序。（尽管经典 BPF 校验器允许它们）

第二步从第一条指令开始，遍历所有可能路径。它模拟每条指令的执行并观察寄存器与
栈的状态变化
在程序开始时，寄存器 R1 包含一个指context 的指针，类型PTR_TO_CTX如果校验器看到一R2=R1 的指令，那么 R2 现在也具有类PTR_TO_CTX，并且可用在表达式的右侧。如R1=PTR_TO_CTX 且指令为 R2=R1+R1，那R2=SCALAR_VALUE因为两个有效指针相加会产生无效指针。（secure"模式下，校验器将拒绝任何类型
的指针算术，以确保内核地址不会泄漏给非特权用户
```
  bpf_mov R0 = R2
  bpf_exit
```

将被拒绝，因为在程序开始处 R2 是不可读的
在内核函数调用之后，R1-R5 被重置为不可读，R0 具有该函数的返回类型
由于 R6-R9 是被调用方保存（callee saved）的，它们的状态在调用之间是保留的
```
  bpf_mov R6 = 1
  bpf_call foo
  bpf_mov R0 = R6
  bpf_exit
```

是一个正确的程序。如果这里用的是 R1 而不R6，它就会被拒绝
load/store 指令只允许使用具有有效类型的寄存器，这些类型PTR_TO_CTXPTR_TO_MAP、PTR_TO_STACK。它们会经过边界和对其检查
```
 bpf_mov R1 = 1
 bpf_mov R2 = 2
 bpf_xadd *(u32 *)(R1 + 3) += R2
 bpf_exit
```

将被拒绝，因为在执行 bpf_xadd 指令R1 不具备有效的指针类型
在开始时 R1 的类型为 PTR_TO_CTX（指向通用 `struct bpf_context` 的指针）。使一个回调来自定义校验器，以eBPF 程序ctx 结构体内特定字段（具有指定大小和
对齐）的访问限制为仅允许这些字段
```
  bpf_ld R0 = *(u32 *)(R6 + 8)
```

意图从地址 R6 + 8 加载一个字并将其存R0。如R6=PTR_TO_CTX，通过
is_valid_access() 回调，校验器将知道偏移量 8、大小为 4 字节的区域可以被读取访问否则校验器将拒绝该程序。如R6=PTR_TO_STACK，那么访问应当是对齐的且位于栈边[-MAX_BPF_STACK, 0) 之内。在此例中偏移量8，因此它将无法通过校验，因为它越界了
校验器只允许 eBPF 程序在向栈中写入之后从栈中读取数据
经典 BPF 校验器对 M[0-15] 内存槽做类似的检查
```
  bpf_ld R0 = *(u32 *)(R10 - 4)
  bpf_exit
```

是无效的程序。尽R10 是正确的只读寄存器且类型PTR_TO_STACK，并R10 - 4
在栈边界之内，但那里从未被写入过
指针寄存器的溢出/填充（spill/fill）也被跟踪，因为四个（R6-R9）被调用方保存寄存器
对某些程序来说可能不够用
允许的函数调用通过 bpf_verifier_ops->get_func_proto() 自定义。eBPF 校验器将检寄存器是否匹配参数约束。调用之后，寄存R0 将被设置为该函数的返回类型
函数调用是扩eBPF 程序功能的主要机制。套接字过滤器可能允许程序调用一组函数，跟踪过滤器可能允许一组完全不同的函数
如果一个函数被开放给 eBPF 程序使用，那么从安全角度需要仔细考虑。校验器将保证该函数
以有效的参数被调用
seccomp 与套接字过滤器对经典 BPF 有不同的安全限制。Seccomp 通过两阶段校验器解决个问题：经典 BPF 校验器之后跟seccomp 校验器。而在 eBPF 中，一个可配置的校验器
被所有用例共享
eBPF 校验器的细节参见 kernel/bpf/verifier.c
## 寄存器值跟
为了确定 eBPF 程序的安全性，校验器必须跟踪每个寄存器以及每个栈槽中可能出现的值的
范围。这是通过 `struct bpf_reg_state` 完成的，它定义于 include/linux/bpf_verifier.h统一了对标量值和指针值的跟踪。每个寄存器状态有一个类型，它要么是 NOT_INIT（寄存器
尚未被写入），要么是 SCALAR_VALUE（某个不可用作指针的值），要么是一个指针类型。指的类型描述了其基址，如下：

    PTR_TO_CTX
			指向 bpf_context 的指针    CONST_PTR_TO_MAP
			指向 struct bpf_map 的指针Const"（常量）是因为禁止对这些
			指针进行算术运算    PTR_TO_MAP_VALUE
			指向存储map 元素中的值的指针    PTR_TO_MAP_VALUE_OR_NULL
			要么是指map 值的指针，要么是 NULL；map 访问（见 maps.rst			返回此类型，当被检!= NULL 时它变为 PTR_TO_MAP_VALUE。禁			对这些指针进行算术运算    PTR_TO_STACK
			帧指针（frame pointer）    PTR_TO_PACKET
			skb->data銆?    PTR_TO_PACKET_END
			skb->data + headlen；禁止算术运算    PTR_TO_SOCKET
			指向 struct bpf_sock_ops 的指针，隐式引用计数    PTR_TO_SOCKET_OR_NULL
			要么是指socket 的指针，要么NULL；socket 查找返回此类型，
			当被检!= NULL 时它变为 PTR_TO_SOCKET。PTR_TO_SOCKET 是引			计数的，因此程序必须在程序结束前通过 socket 释放函数释放该引用			禁止对这些指针进行算术运算
然而，一个指针可能相对于这个基址有偏移（作为指针算术的结果），这通过两部分跟踪：
'fixed offset'（固定偏移）'variable offset'（可变偏移）。前者在将一个确切已知的（例如一个立即数操作数）加到一个指针时使用，后者用于不完全确定已知的值。可变偏也用SCALAR_VALUE，以跟踪寄存器中可能出现的值的范围
校验器关于可变偏移的知识包括
- 作为无符号数的最小值和最大- 作为有符号数的最小值和最大
- 单个比特值的知识，形式为 'tnum'：一u64 'mask' 和一u64 'value'。mask
  中的 1 表示值未知的比特；value 中的 1 表示已知1 的比特。已知为 0 的比特在 mask
  value 中都0；没有任何比特应该在两者中都为 1。例如，如果一个字节从内存读入
  一个寄存器，该寄存器的56 位已知为 0，而低 8 位未知——这表示tnum (0x0; 0xff)  如果我们再将其与 0x40 OR 运算，得(0x40; 0xbf)，如果我们再1 则得(0x0;
  0x1ff)，因为可能有进位
除了算术运算，寄存器状态也可以被条件分支更新。例如，如果一SCALAR_VALUE 被比> 8，在 'true'（真）分支中它将有一umin_value（无符号最小值）9，而在 'false'
（假）分支中它将有一umax_value 8。有符号比较（使BPF_JSGT BPF_JSGE）将
改为更新有符号的最小最大值。来自有符号和无符号边界的信息可以组合；例如如果一值先被测< 8 然后被测s> 4，校验器将推断出该值也 > 4 s< 8，因为边界阻止跨符号位
带有可变偏移部分PTR_TO_PACKET 有一'id'，它对所有共享同一可变偏移的指针是
通用的。这对于包范围检查很重要：在给包指针寄存A 加上一个变量之后，如果你将其复到另一个寄存器 B，然后给 A 加上常量 4，两个寄存器将共享同一'id'，但 A 将有一固定偏移 +4。然后如A 经过边界检查并被发现小于一PTR_TO_PACKET_END，那么寄存器
B 现在就知道有至少 4 字节的安全范围。关PTR_TO_PACKET 范围的更多内容，参见下面"Direct packet access"（直接包访问）
'id' 字段也用PTR_TO_MAP_VALUE_OR_NULL，对所有从 map 查找返回的指针副本通用。这
意味着当一个副本被检查并发现NULL 时，所有副本都可以变为 PTR_TO_MAP_VALUE。除范围检查，被跟踪的信息也用于强制指针访问的对齐。例如，在大多数系统上包指针在一4 字节对齐之后 2 字节处。如果一个程序加14 字节以跳过以太网头部，然后读IHL 加上（IHL * 4），得到的指针将有一个已知为 4n+2（对n）的可变偏移，因此加上这 2 字节
（NET_IP_ALIGN）就得到一4 字节对齐，因此通过该指针进行的字（word）大小访问是安全的'id' 字段也用PTR_TO_SOCKET PTR_TO_SOCKET_OR_NULL，对所有从 socket 查找返回指针副本通用。其行为PTR_TO_MAP_VALUE_OR_NULL->PTR_TO_MAP_VALUE 的处理类似，但它
也处理指针的引用跟踪。PTR_TO_SOCKET 隐式地表示一个对相应 `struct sock` 的引用。为确保该引用不被泄漏，必须对该引用NULL 检查，并且在非 NULL 的情况下，将有效引用传递给
socket 释放函数
## 直接包访
cls_bpf act_bpf 程序中，校验器允许通过 skb->data skb->data_end 指针直接
访问包数据
```
    1:  r4 = *(u32 *)(r1 +80)  /* 加载 skb->data_end */
    2:  r3 = *(u32 *)(r1 +76)  /* 加载 skb->data */
    3:  r5 = r3
    4:  r5 += 14
    5:  if r5 > r4 goto pc+16
    R1=ctx R3=pkt(id=0,off=0,r=14) R4=pkt_end R5=pkt(id=0,off=14,r=14) R10=fp
    6:  r0 = *(u16 *)(r3 +12) /* 访问包的12 13 字节 */
```

这个从包2 字节加载是安全的，因为程序作者在5 条指令处确实检查了
`if (skb->data + 14 > skb->data_end) goto err`，这意味着fall-through 情况下，
寄存R3（指skb->data）至少有 14 个可直接访问的字节。校验器将其标记R3=pkt(id=0,off=0,r=14)。id=0 表示没有向该寄存器添加额外的变量。off=0 表示没有添加
额外的常量。r=14 是安全访问的范围，意味着字节 [R3, R3 + 14) 是没问题的。注R5 标记R5=pkt(id=0,off=14,r=14)。它也指向包数据，但是向寄存器加了常14，所以它现在
指向 `skb->data + 14`，可访问范围[R5, R5 + 14 - 14)，即零字节
```
    R0=inv1 R1=ctx R3=pkt(id=0,off=0,r=14) R4=pkt_end R5=pkt(id=0,off=14,r=14) R10=fp
    6:  r0 = *(u8 *)(r3 +7) /* 从包加载7 字节 */
    7:  r4 = *(u8 *)(r3 +12)
    8:  r4 *= 14
    9:  r3 = *(u32 *)(r1 +76) /* 加载 skb->data */
    10:  r3 += r4
    11:  r2 = r1
    12:  r2 <<= 48
    13:  r2 >>= 48
    14:  r3 += r2
    15:  r2 = r3
    16:  r2 += 8
    17:  r1 = *(u32 *)(r1 +80) /* 加载 skb->data_end */
    18:  if r2 > r1 goto pc+2
    R0=inv(id=0,umax_value=255,var_off=(0x0; 0xff)) R1=pkt_end R2=pkt(id=2,off=8,r=8) R3=pkt(id=2,off=0,r=8) R4=inv(id=0,umax_value=3570,var_off=(0x0; 0xfffe)) R5=pkt(id=0,off=14,r=14) R10=fp
    19:  r1 = *(u8 *)(r3 +4)
```

寄存R3 的状态是 R3=pkt(id=2,off=0,r=8)。id=2 表示看到了两`r3 += rX` 指令因此 r3 指向包内的某个偏移，并且由于程序作者在18 条指令处做了 `if (r3 + 8 > r1)
goto err`，安全范围是 [R3, R3 + 8)。校验器只允许对包寄存器进行 'add'/'sub'（加/减）
操作。任何其他操作都会将寄存器状态设置为 'SCALAR_VALUE'，它将不再可用于直接包访问
操作 `r3 += rX` 可能溢出并变得小于原始的 skb->data，因此校验器必须阻止这一点。所当它看到 `r3 += rX` 指令rX 是大16 位的值时，任何随后对 r3 针对 skb->data_end
的边界检查都不会给我范围"信息，因此对通过该指针的读取尝试将给invalid access
to packet"（对包的无效访问）错误
例如在第 7 条指`r4 = **(u8 **)(r3 +12)` 之后，r4 的状态是 R4=inv(id=0,
umax_value=255,var_off=(0x0; 0xff))，这意味着寄存器的56 位保证为零，而对8 一无所知。在`r4 *= 14` 条指令之后，状态变R4=inv(id=0,umax_value=3570,
var_off=(0x0; 0xfffe))，因为将一8 位值乘以常14 会保持高 52 位为零，且最低有位也将为零，因为 14 是偶数。类似地 `r2 >>= 48` 将使R2=inv(id=0,umax_value=65535,
var_off=(0x0; 0xffff))，因为移位不是符号扩展。这个逻辑adjust_reg_min_max_vals()
函数中实现，它对"指针加标（或反之）调adjust_ptr_min_max_vals()，对两个标量上的
运算调用 adjust_scalar_min_max_vals()
最终的结果bpf 程序作者可以直接访问包
```
  void *data = (void *)(long)skb->data;
  void *data_end = (void *)(long)skb->data_end;
  struct eth_hdr *eth = data;
  struct iphdr *iph = data + sizeof(*eth);
  struct udphdr *udp = data + sizeof(*eth) + sizeof(*iph);

  if (data + sizeof(*eth) + sizeof(*iph) + sizeof(*udp) > data_end)
	  return 0;
  if (eth->h_proto != htons(ETH_P_IP))
	  return 0;
  if (iph->protocol != IPPROTO_UDP || iph->ihl != 5)
	  return 0;
  if (udp->dest == 53 || udp->source == 9)
	  ...;
```

LD_ABS 指令相比，这使得此类程序更易于编写，并且显著更快
## 剪枝（Pruning
校验器实际上并不遍历程序中所有可能的路径。对于每一条要分析的新分支，校验器查看它此在这条指令处曾经处于的所有状态。如果其中任何一个包含当前状态作为子集，该分支就"pruned"（剪枝）——也就是说，先前状态被接受的事实意味着当前状态也会被接受。例如，如果先前状态中 r1 持有一个包指针，而在当前状态中 r1 持有一个范围至少同样长且对齐至少同严格的包指针，那r1 是安全的。类似地，如r2 之前NOT_INIT，那么从那一点出发的
任何路径都不可能使用过它，因r2 中的任何值（包括另一NOT_INIT）都是安全的。该实现
regsafe() 函数中。剪枝不仅考虑寄存器，也考虑栈（以及它可能持有的任何溢出的寄存器）它们必须全部安全，该分支才会被剪枝。这states_equal() 中实现
关于状态剪枝实现的一些技术细节可以在下面找到
### 寄存器活跃性跟
为了使状态剪枝有效，会对每个寄存器和栈槽跟踪活跃性（liveness）状态。基本思想是跟哪些寄存器和栈槽在程序的后续执行中（直到到达程序退出）实际被使用。从未被使用过的寄存
器和栈槽可以从缓存的状态中移除，从而使更多状态等价于一个缓存状态：

```
  0: call bpf_get_prandom_u32()
  1: r1 = 0
  2: if r0 == 0 goto +1
  3: r0 = 1
  --- checkpoint ---
  4: r0 = r1
  5: exit
```

假设在指#4 处创建了一个状态缓存条目（此类条目在下文中也称checkpoints"（检查点））校验器可能带着以下两种可能的寄存器状态之一到达该指令：

- r0 = 1, r1 = 0
- r0 = 0, r1 = 0

然而，只有寄存`r1` 的值对于成功完成校验才是重要的。活跃性跟踪算法的目标是发现这一
事实，并弄清这两种状态实际上是等价的
## 理解 eBPF 校验器消
以下是几个无eBPF 程序以及在校验器日志中看到的错误信息的示例：

```
  static struct bpf_insn prog[] = {
  BPF_EXIT_INSN(),
  BPF_EXIT_INSN(),
  };
```

```
  unreachable insn 1
```

```
  BPF_MOV64_REG(BPF_REG_0, BPF_REG_2),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r0 = r2
  R2 !read_ok
```

```
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_1),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r2 = r1
  1: (95) exit
  R0 !read_ok
```

```
    BPF_ST_MEM(BPF_DW, BPF_REG_10, 8, 0),
    BPF_EXIT_INSN(),
```

```
    0: (7a) *(u64 *)(r10 +8) = 0
    invalid stack off=8 size=8
```

```
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r2 = r10
  1: (07) r2 += -8
  2: (b7) r1 = 0x0
  3: (85) call 1
  invalid indirect read from stack off -8+0 size 8
```

```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 0x0
  4: (85) call 1
  fd 0 is not pointing to valid bpf_map
```

在访问之前不检map_lookup_elem() 返回值的程序
```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 0),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 0x0
  4: (85) call 1
  5: (7a) *(u64 *)(r0 +0) = 0
  R0 invalid mem access 'map_value_or_null'
```

正确检查了 map_lookup_elem() 返回值是否为 NULL，但
```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 1),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 4, 0),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 1
  4: (85) call 1
  5: (15) if r0 == 0x0 goto pc+1
   R0=map_ptr R10=fp
  6: (7a) *(u64 *)(r0 +4) = 0
  misaligned access off 4 size 8
```

正确检查了 map_lookup_elem() 返回值是否为 NULL，并'if' 分支的一侧以正确对齐访问
内存，但失败的程序：

```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 2),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 0),
  BPF_EXIT_INSN(),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 1),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 1
  4: (85) call 1
  5: (15) if r0 == 0x0 goto pc+2
   R0=map_ptr R10=fp
  6: (7a) *(u64 *)(r0 +0) = 0
  7: (95) exit

  from 5 to 8: R0=imm0 R10=fp
  8: (7a) *(u64 *)(r0 +0) = 1
  R0 invalid mem access 'imm'
```

执行 socket 查找然后将指针设NULL，却没有
```
  BPF_MOV64_IMM(BPF_REG_2, 0),
  BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_2, -8),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_MOV64_IMM(BPF_REG_3, 4),
  BPF_MOV64_IMM(BPF_REG_4, 0),
  BPF_MOV64_IMM(BPF_REG_5, 0),
  BPF_EMIT_CALL(BPF_FUNC_sk_lookup_tcp),
  BPF_MOV64_IMM(BPF_REG_0, 0),
  BPF_EXIT_INSN(),
```

```
  0: (b7) r2 = 0
  1: (63) *(u32 *)(r10 -8) = r2
  2: (bf) r2 = r10
  3: (07) r2 += -8
  4: (b7) r3 = 4
  5: (b7) r4 = 0
  6: (b7) r5 = 0
  7: (85) call bpf_sk_lookup_tcp#65
  8: (b7) r0 = 0
  9: (95) exit
  Unreleased reference id=1, alloc_insn=7
```

执行 socket 查找但未对返回的指针NULL 检查的程序
```
  BPF_MOV64_IMM(BPF_REG_2, 0),
  BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_2, -8),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_MOV64_IMM(BPF_REG_3, 4),
  BPF_MOV64_IMM(BPF_REG_4, 0),
  BPF_MOV64_IMM(BPF_REG_5, 0),
  BPF_EMIT_CALL(BPF_FUNC_sk_lookup_tcp),
  BPF_EXIT_INSN(),
```

```
  0: (b7) r2 = 0
  1: (63) *(u32 *)(r10 -8) = r2
  2: (bf) r2 = r10
  3: (07) r2 += -8
  4: (b7) r3 = 4
  5: (b7) r4 = 0
  6: (b7) r5 = 0
  7: (85) call bpf_sk_lookup_tcp#65
  8: (95) exit
  Unreleased reference id=1, alloc_insn=7
```
