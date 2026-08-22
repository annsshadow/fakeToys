

## Linux 套接字过滤即伯克利数据包过滤器（BPF

### 注意


本文件曾经记录了 eBPF 的格式与机制，即使这些内容与套接字过滤无关。关eBPF 的更多细节请参阅 ../bpf/index.rst
### 简

Linux 套接字过滤（LSF）派生自 Berkeley Packet Filter。尽BSD Linux
内核过滤之间存在一些明显差异，但当我们Linux 语境中谈BPF LSF 时，
指的Linux 内核中完全相同的过滤机制
BPF 允许用户空间程序将过滤器附加到任何套接字上，并允许或禁止某些类型数据通过该套接字。LSF 遵循BSD BPF 完全相同的过滤器代码结构，因参BSD bpf.4 手册页对创建过滤器非常有帮助
Linux 上，BPF 比在 BSD 上简单得多。你不必担心设备之类的事情。你只需
创建你的过滤器代码，通过 SO_ATTACH_FILTER 选项将其发送到内核，如果你过滤器代码通过了内核的检查，你就可以立即开始在该套接字上过滤数据
你也可以通过 SO_DETACH_FILTER 选项从套接字上分离过滤器。这可能不太常用因为当你关闭带有过滤器的套接字时，过滤器会被自动移除。另一种不太常见的
情况可能是，在已经运行着另一个过滤器的同一套接字上添加不同的过滤器：内负责移除旧的并放置你的新过滤器，前提是你的过滤器通过了检查，否则如果失败旧的过滤器将保留在该套接字上
SO_LOCK_FILTER 选项允许锁定附加到套接字的过滤器。一旦设置，过滤器就无法
被移除或更改。这允许一个进程建立套接字、附加过滤器、锁定它，然后放弃特权，
并确信该过滤器会一直保留到套接字关闭
此构造最大的使用者可能就libpcap。发出像 `tcpdump -i em1 port 22` 这样
的高级过滤器命令，会经过 libpcap 内部编译器，生成一个最终可以通过
SO_ATTACH_FILTER 加载到内核的结构。`tcpdump -i em1 port 22 -ddd` 会显正被放入该结构的内容
尽管我们这里只谈论套接字，但 Linux 中的 BPF 还被用于更多地方。netfilter xt_bpf，内qdisc 层有 cls_bpf，还SECCOMP-BPF（SECure COMPuting
[^1^]_），以及许多其他地方，例team 驱动、PTP 代码等都在使BPF

Original BPF paper:

Steven McCanne and Van Jacobson. 1993. The BSD packet filter: a new
architecture for user-level packet capture. In Proceedings of the
USENIX Winter 1993 Conference Proceedings on USENIX Winter 1993
Conference Proceedings (USENIX'93). USENIX Association, Berkeley,
CA, USA, 2-2. [http://www.tcpdump.org/papers/bpf-usenix93.pdf]

### 结构


用户空间应用程序包含 <linux/filter.h>，其中含```

	struct sock_filter {	/* Filter block */
		__u16	code;   /* Actual filter code */
		__u8	jt;	/* Jump true */
		__u8	jf;	/* Jump false */
		__u32	k;      /* Generic multiuse field */
	};

```
这样的结构体被组装成一个由 4 元组组成的数组，包含 code、jt、jf k 值jt jf 是跳转偏移，k 是一个通用```

	struct sock_fprog {			/* Required for SO_ATTACH_FILTER. */
		unsigned short		   len;	/* Number of filter blocks */
		struct sock_filter __user *filter;
	};

```
对于套接字过滤，指向该结构体的指针（如后续示例所示）通过 setsockopt(2)
传递给内核
### 示例


```

    #include <sys/socket.h>
    #include <sys/types.h>
    #include <arpa/inet.h>
    #include <linux/if_ether.h>
    /* ... */

    /* From the example above: tcpdump -i em1 port 22 -dd */
    struct sock_filter code[] = {
	    { 0x28,  0,  0, 0x0000000c },
	    { 0x15,  0,  8, 0x000086dd },
	    { 0x30,  0,  0, 0x00000014 },
	    { 0x15,  2,  0, 0x00000084 },
	    { 0x15,  1,  0, 0x00000006 },
	    { 0x15,  0, 17, 0x00000011 },
	    { 0x28,  0,  0, 0x00000036 },
	    { 0x15, 14,  0, 0x00000016 },
	    { 0x28,  0,  0, 0x00000038 },
	    { 0x15, 12, 13, 0x00000016 },
	    { 0x15,  0, 12, 0x00000800 },
	    { 0x30,  0,  0, 0x00000017 },
	    { 0x15,  2,  0, 0x00000084 },
	    { 0x15,  1,  0, 0x00000006 },
	    { 0x15,  0,  8, 0x00000011 },
	    { 0x28,  0,  0, 0x00000014 },
	    { 0x45,  6,  0, 0x00001fff },
	    { 0xb1,  0,  0, 0x0000000e },
	    { 0x48,  0,  0, 0x0000000e },
	    { 0x15,  2,  0, 0x00000016 },
	    { 0x48,  0,  0, 0x00000010 },
	    { 0x15,  0,  1, 0x00000016 },
	    { 0x06,  0,  0, 0x0000ffff },
	    { 0x06,  0,  0, 0x00000000 },
    };

    struct sock_fprog bpf = {
	    .len = ARRAY_SIZE(code),
	    .filter = code,
    };

    sock = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
    if (sock < 0)
	    /* ... bail out ... */

    ret = setsockopt(sock, SOL_SOCKET, SO_ATTACH_FILTER, &bpf, sizeof(bpf));
    if (ret < 0)
	    /* ... bail out ... */

    /* ... */
    close(sock);

```
上面的示例代码为 PF_PACKET 套接字附加了一个套接字过滤器，以让所有端口为 22
IPv4/IPv6 数据包通过。其余的数据包将被该套接字丢弃
SO_DETACH_FILTER setsockopt(2) 调用不需要任何参数，而用于防止过滤器
被分离的 SO_LOCK_FILTER 接受一个取值为 0 1 的整数值
请注意，套接字过滤器并不仅限PF_PACKET 套接字，也可以用于其他套接字族
系统调用总结
 - setsockopt(sockfd, SOL_SOCKET, SO_ATTACH_FILTER, &val, sizeof(val));
 - setsockopt(sockfd, SOL_SOCKET, SO_DETACH_FILTER, &val, sizeof(val));
 - setsockopt(sockfd, SOL_SOCKET, SO_LOCK_FILTER,   &val, sizeof(val));

通常，在包套接字上进行套接字过滤的大部分用例都会libpcap 以高级语法覆盖，
因此作为应用程序开发者，你应该坚持使用它。libpcap 在所有这些之上封装了自己层
除非 i) 不使不链libpcap 不可行，ii) 所需BPF 过滤器使用了 libpcap
编译器不支持Linux 扩展，iii) 过滤器可能更复杂且无法用 libpcap 编译器干净
地实现，iv) 特定的过滤器代码需要以不同libpcap 内部编译器的方式进行
优化；那么在这样的情形下，手动“手写”这样的过滤器可以作为一种替代方案。例如，
xt_bpf cls_bpf 用户可能有会产生更复杂过滤器代码的需求，或者产生无法用
libpcap 表达的代码（例如不同代码路径有不同返回码）。此外，BPF JIT 实现可能希望手动编写测试用例，因此也需要对 BPF 代码的底层访问
### BPF 引擎与指令集


tools/bpf/ 下有一个名bpf_asm 的小辅助工具，可用于为上一节提到的示例
场景编写底层过滤器。这里提到的类汇编语法已bpf_asm 中实现，并将用于进一的解释（而不是直接处理可读性较差的操作码，原理是相同的）。该语法紧密模仿
Steven McCanne Van Jacobson BPF 论文
BPF 架构由以下基本元素组成：

  =======          ====================================================
  Element          Description
  =======          ====================================================
  A                32 bit wide accumulator
  X                32 bit wide X register
  M[]              16 x 32 bit wide misc registers aka "scratch memory
		   store", addressable from 0 to 15
  =======          ====================================================

一个由 bpf_asm 翻译成“opcodes”的程序是一个数组，```

  op:16, jt:8, jf:8, k:32

```
元素 op 是一16 位宽的操作码，其中编码了特定的指令。jt jf 是两8 宽的跳转目标，一个用于“条件为真时跳转”，另一个用于“条件为假时跳转”。最后，
元素 k 包含一个杂项参数，可以根据 op 中给定的指令以不同方式解释
指令集由加载、存储、分支、alu、杂项和返回指令组成，这些也bpf_asm 语法表示。下表列出了所有可用的 bpf_asm 指令，以及它们在 linux/filter.h 中定义的
基础操作码的含义
  ===========      ===================  =====================
  Instruction      Addressing mode      Description
  ===========      ===================  =====================
  ld               1, 2, 3, 4, 12       Load word into A
  ldi              4                    Load word into A
  ldh              1, 2                 Load half-word into A
  ldb              1, 2                 Load byte into A
  ldx              3, 4, 5, 12          Load word into X
  ldxi             4                    Load word into X
  ldxb             5                    Load byte into X

  st               3                    Store A into M[]
  stx              3                    Store X into M[]

  jmp              6                    Jump to label
  ja               6                    Jump to label
  jeq              7, 8, 9, 10          Jump on A == <x>
  jneq             9, 10                Jump on A != <x>
  jne              9, 10                Jump on A != <x>
  jlt              9, 10                Jump on A <  <x>
  jle              9, 10                Jump on A <= <x>
  jgt              7, 8, 9, 10          Jump on A >  <x>
  jge              7, 8, 9, 10          Jump on A >= <x>
  jset             7, 8, 9, 10          Jump on A &  <x>

  add              0, 4                 A + <x>
  sub              0, 4                 A - <x>
  mul              0, 4                 A * <x>
  div              0, 4                 A / <x>
  mod              0, 4                 A % <x>
  neg                                   !A
  and              0, 4                 A & <x>
  or               0, 4                 A | <x>
  xor              0, 4                 A ^ <x>
  lsh              0, 4                 A << <x>
  rsh              0, 4                 A >> <x>

  tax                                   Copy A into X
  txa                                   Copy X into A

  ret              4, 11                Return
  ===========      ===================  =====================

下一表显示了2 列中的寻址格式
  ===============  ===================  ===============================================
  Addressing mode  Syntax               Description
  ===============  ===================  ===============================================
   0               x/%x                 Register X
   1               [k]                  BHW at byte offset k in the packet
   2               [x + k]              BHW at the offset X + k in the packet
   3               M[k]                 Word at offset k in M[]
   4               #k                   Literal value stored in k
   5               4**([k]&0xf)          Lower nibble ** 4 at byte offset k in the packet
   6               L                    Jump label L
   7               #k,Lt,Lf             Jump to Lt if true, otherwise jump to Lf
   8               x/%x,Lt,Lf           Jump to Lt if true, otherwise jump to Lf
   9               #k,Lt                Jump to Lt if predicate is true
  10               x/%x,Lt              Jump to Lt if predicate is true
  11               a/%a                 Accumulator A
  12               extension            BPF extension
  ===============  ===================  ===============================================

Linux 内核还有几个 BPF 扩展，它们通过k 参数“重载”为负偏移加上特定的扩展
偏移，与加载指令这一类一起使用。此BPF 扩展的结果被加载A 中
可能BPF 扩展如下表所示：

  ===================================   =================================================
  Extension                             Description
  ===================================   =================================================
  len                                   skb->len
  proto                                 skb->protocol
  type                                  skb->pkt_type
  poff                                  Payload start offset
  ifidx                                 skb->dev->ifindex
  nla                                   Netlink attribute of type X with offset A
  nlan                                  Nested Netlink attribute of type X with offset A
  mark                                  skb->mark
  queue                                 skb->queue_mapping
  hatype                                skb->dev->type
  rxhash                                skb->hash
  cpu                                   raw_smp_processor_id()
  vlan_tci                              skb_vlan_tag_get(skb)
  vlan_avail                            skb_vlan_tag_present(skb)
  vlan_tpid                             skb->vlan_proto
  rand                                  get_random_u32()
  ===================================   =================================================

这些扩展也可以加'#' 前缀底层 BPF 示例
```

  ldh [12]
  jne #0x806, drop
  ret #-1
  drop: ret #0

```
```

  ldh [12]
  jne #0x800, drop
  ldb [23]
  jneq #6, drop
  ret #-1
  drop: ret #0

```
```

  ldh [12]
  jne #0x800, drop
  ldb [23]
  jneq #1, drop
  # get a random uint32 number
  ld rand
  mod #4
  jneq #1, drop
  ret #-1
  drop: ret #0

```
```

  ld [4]                  /* offsetof(struct seccomp_data, arch) */
  jne #0xc000003e, bad    /* AUDIT_ARCH_X86_64 */
  ld [0]                  /* offsetof(struct seccomp_data, nr) */
  jeq #15, good           /* __NR_rt_sigreturn */
  jeq #231, good          /* __NR_exit_group */
  jeq #60, good           /* __NR_exit */
  jeq #0, good            /* __NR_read */
  jeq #1, good            /* __NR_write */
  jeq #5, good            /* __NR_fstat */
  jeq #9, good            /* __NR_mmap */
  jeq #14, good           /* __NR_rt_sigprocmask */
  jeq #13, good           /* __NR_rt_sigaction */
  jeq #35, good           /* __NR_nanosleep */
  bad: ret #0             /* SECCOMP_RET_KILL_THREAD */
  good: ret #0x7fff0000   /* SECCOMP_RET_ALLOW */

```
底层 BPF 扩展示例
```

  ld ifidx
  jneq #13, drop
  ret #-1
  drop: ret #0

```
```

  ld vlan_tci
  jneq #10, drop
  ret #-1
  drop: ret #0

```
上面的示例代码可以放入一个文件（这里称为“foo”），然后传递给 bpf_asm 工具
以生成操作码，其输出xt_bpf cls_bpf 能够理解并可直接加载的。使用上示例```

    $ ./bpf_asm foo
    4,40 0 0 12,21 0 1 2054,6 0 0 4294967295,6 0 0 0,

```
```

    $ ./bpf_asm -c foo
    { 0x28,  0,  0, 0x0000000c },
    { 0x15,  0,  1, 0x00000806 },
    { 0x06,  0,  0, 0xffffffff },
    { 0x06,  0,  0, 0000000000 },

```
特别是，由于xt_bpf cls_bpf 一起使用可能导致更复杂BPF 过滤器，一
开始可能并不明显，因此在附加到真实系统之前测试过滤器是很好的做法。为此，
内核源代码目录下tools/bpf/ 中有一个名bpf_dbg 的小工具。该调试器允针对给定pcap 文件测试 BPF 过滤器，pcap 数据包上BPF 代码进行单步
执行，并进行 BPF 机器寄存器转储
```

    # ./bpf_dbg

```
如果输入和输出不等于 stdin/stdout，bpf_dbg 会将替代stdin 源作为第一个参数，
将替代的 stdout 接收器作为第二个参数，例`./bpf_dbg test_in.txt test_out.txt`
除此之外，可以通过文件 "~/.bpf_dbg_init" 设置特定libreadline 配置，命历史存储在文"~/.bpf_dbg_history" 中
bpf_dbg 中的交互通过一个同样支持自动补全的 shell 进行（后续以 '>' 开头的
示例命令表示 bpf_dbg shell）。通常的工作流程是…
- load bpf 6,40 0 0 12,21 0 3 2048,48 0 0 23,21 0 1 1,6 0 0 65535,6 0 0 0
  bpf_asm 的标准输出加BPF 过滤器，或经由例`tcpdump -iem1 -ddd port 22 | tr '\n' ','` 转换而来。请注意，为JIT 调试（下一节），此命令会创建一个临时套接字并将 BPF 代码加载到内核中。因此，它对 JIT 开发者也很有用
- load pcap foo.pcap

  加载标准tcpdump pcap 文件
- run [<n>]

bpf passes:1 fails:9
  遍历 pcap 中的所有数据包，统计过滤器将产生多少次通过（pass）和失败（fail）  可以给定要遍历的数据包数量上限
```

	l0:	ldh [12]
	l1:	jeq #0x800, l2, l5
	l2:	ldb [23]
	l3:	jeq #0x1, l4, l5
	l4:	ret #0xffff
	l5:	ret #0

  Prints out BPF code disassembly.

```
```

	/* { op, jt, jf, k }, */
	{ 0x28,  0,  0, 0x0000000c },
	{ 0x15,  0,  3, 0x00000800 },
	{ 0x30,  0,  0, 0x00000017 },
	{ 0x15,  0,  1, 0x00000001 },
	{ 0x06,  0,  0, 0x0000ffff },
	{ 0x06,  0,  0, 0000000000 },

  Prints out C-style BPF code dump.

```
```

	breakpoint at: l0:	ldh [12]

```
```

	breakpoint at: l1:	jeq #0x800, l2, l5

  ...

  Sets breakpoints at particular BPF instructions. Issuing a `run` command
  will walk through the pcap file continuing from the current packet and
  break when a breakpoint is being hit (another `run` will continue from
  the currently active breakpoint executing next instructions):

  * run::

	-- register dump --
	pc:       [0]                       <-- program counter
	code:     [40] jt[0] jf[0] k[12]    <-- plain BPF code of current instruction
	curr:     l0:	ldh [12]              <-- disassembly of current instruction
	A:        [00000000][0]             <-- content of A (hex, decimal)
	X:        [00000000][0]             <-- content of X (hex, decimal)
	M[0,15]:  [00000000][0]             <-- folded content of M (hex, decimal)
	-- packet dump --                   <-- Current packet from pcap (hex)
	len: 42
	    0: 00 19 cb 55 55 a4 00 14 a4 43 78 69 08 06 00 01
	16: 08 00 06 04 00 01 00 14 a4 43 78 69 0a 3b 01 26
	32: 00 00 00 00 00 00 0a 3b 01 01
	(breakpoint)
	>

  * breakpoint::

	breakpoints: 0 1

    Prints currently set breakpoints.

```
- step [-<n>, +<n>]

  从当pc 偏移量开始对 BPF 程序进行单步执行。因此，每次调用 step 时，都会
  输出上面的寄存器转储。这可以在时间上向前和向后移动，单纯`step` 会在  一BPF 指令处中断，+1。（这里不需要发`run`。）

- select <n>

  pcap 文件中选择一个给定的数据包以继续。因此，在下一`run` `step`
  时，BPF 程序将针对用户预先选择的数据包进行求值。编号与 Wireshark 一样从
  索引 1 开始
- quit

  退bpf_dbg
### JIT 编译

Linux 内核内置了一个用x86_64、SPARC、PowerPC、ARM、ARM64、MIPS、RISC-Vs390 ARC BPF JIT 编译器，可通过 CONFIG_BPF_JIT 启用。如果设置了
```

  echo 1 > /proc/sys/net/core/bpf_jit_enable

```
对于 JIT 开发者，进行审计等用途，每次编译运行都可以输出生成的
```

  echo 2 > /proc/sys/net/core/bpf_jit_enable

```
```

    [ 3389.935842] flen=6 proglen=70 pass=3 image=ffffffffa0069c8f
    [ 3389.935847] JIT code: 00000000: 55 48 89 e5 48 83 ec 60 48 89 5d f8 44 8b 4f 68
    [ 3389.935849] JIT code: 00000010: 44 2b 4f 6c 4c 8b 87 d8 00 00 00 be 0c 00 00 00
    [ 3389.935850] JIT code: 00000020: e8 1d 94 ff e0 3d 00 08 00 00 75 16 be 17 00 00
    [ 3389.935851] JIT code: 00000030: 00 e8 28 94 ff e0 83 f8 01 75 07 b8 ff ff 00 00
    [ 3389.935852] JIT code: 00000040: eb 02 31 c0 c9 c3

```
CONFIG_BPF_JIT_ALWAYS_ON 启用时，bpf_jit_enable 被永久设1，设置其任何值都会返回失败。即使将 bpf_jit_enable 设为 2 也是如此，因为将最JIT
映像转储到内核日志是不推荐的，一般建议改用通过 bpftool（位tools/bpf/bpftool/
下）进行自省
在内核源代码树的 tools/bpf/ 下，bpf_jit_disasm 用于
```

	# ./bpf_jit_disasm
	70 bytes emitted from JIT compiler (pass:3, flen:6)
	ffffffffa0069c8f + <x>:
	0:	push   %rbp
	1:	mov    %rsp,%rbp
	4:	sub    $0x60,%rsp
	8:	mov    %rbx,-0x8(%rbp)
	c:	mov    0x68(%rdi),%r9d
	10:	sub    0x6c(%rdi),%r9d
	14:	mov    0xd8(%rdi),%r8
	1b:	mov    $0xc,%esi
	20:	callq  0xffffffffe0ff9442
	25:	cmp    $0x800,%eax
	2a:	jne    0x0000000000000042
	2c:	mov    $0x17,%esi
	31:	callq  0xffffffffe0ff945e
	36:	cmp    $0x1,%eax
	39:	jne    0x0000000000000042
	3b:	mov    $0xffff,%eax
	40:	jmp    0x0000000000000044
	42:	xor    %eax,%eax
	44:	leaveq
	45:	retq

	Issuing option `-o` will "annotate" opcodes to resulting assembler
	.instructions, which can be very useful for JIT developers:

	# ./bpf_jit_disasm -o
	70 bytes emitted from JIT compiler (pass:3, flen:6)
	ffffffffa0069c8f + <x>:
	0:	push   %rbp
		55
	1:	mov    %rsp,%rbp
		48 89 e5
	4:	sub    $0x60,%rsp
		48 83 ec 60
	8:	mov    %rbx,-0x8(%rbp)
		48 89 5d f8
	c:	mov    0x68(%rdi),%r9d
		44 8b 4f 68
	10:	sub    0x6c(%rdi),%r9d
		44 2b 4f 6c
	14:	mov    0xd8(%rdi),%r8
		4c 8b 87 d8 00 00 00
	1b:	mov    $0xc,%esi
		be 0c 00 00 00
	20:	callq  0xffffffffe0ff9442
		e8 1d 94 ff e0
	25:	cmp    $0x800,%eax
		3d 00 08 00 00
	2a:	jne    0x0000000000000042
		75 16
	2c:	mov    $0x17,%esi
		be 17 00 00 00
	31:	callq  0xffffffffe0ff945e
		e8 28 94 ff e0
	36:	cmp    $0x1,%eax
		83 f8 01
	39:	jne    0x0000000000000042
		75 07
	3b:	mov    $0xffff,%eax
		b8 ff ff 00 00
	40:	jmp    0x0000000000000044
		eb 02
	42:	xor    %eax,%eax
		31 c0
	44:	leaveq
		c9
	45:	retq
		c3

```
对于 BPF JIT 开发者，bpf_jit_disasm、bpf_asm bpf_dbg 提供了一个有用的工具
链，用于开发和测试内核JIT 编译器
### BPF 内核内部机制


在内核解释器内部，使用的是一种不同的指令集格式，其底层原理与前面段落描述
BPF 相似。然而，该指令集格式更贴近底层架构进行建模，以模仿原生指令集从而可以获得更好的性能（详见后文）。这个新ISA 被称eBPF。详情请参阅
../bpf/index.rst。（注意：源[e]xtended BPF eBPF BPF 扩展并不相同eBPF 是一ISA，BPF 扩展可以追溯到经BPF BPF_LD | BPF_{B,H,W} |
BPF_ABS 指令的“重载”。）

新指令集最初设计时的可能目标是用“受C（restricted C）”编写程序，并通过
可选的 GCC/LLVM 后端编译eBPF，从而能够以最小的性能开销分两步即时映到现64 CPU，即 C -> eBPF -> 原生代码
目前，新格式被用于运行用BPF 程序，其中包seccomp BPF、经典套接字
过滤器、cls_bpf 流量分类器、team 驱动用于其负载均衡模式的分类器、netfilter
xt_bpf 扩展、PTP 解析分类器等等。它们全部由内核内部转换为新的指令集
表示，并eBPF 解释器中运行。对于内核内处理程序，这一切通过 bpf_prog_create()
建立过滤器、通过 bpf_prog_destroy() 销毁过滤器来透明地工作。函bpf_prog_run(filter, ctx) 透明地调eBPF 解释器或 JIT 编译后的代码来运过滤器filter' 是来bpf_prog_create() 的指struct bpf_prog 的指针，
'ctx' 是给定的上下文（例如 skb 指针）。在后台转换为新布局之前bpf_check_classic() 的所有约束和限制都适用
目前，经BPF 格式用于大多32 位架构上JIT 编译，x86-64、aarch64s390x、powerpc64、sparc64、arm32、riscv64、riscv32、loongarch64、arc eBPF 指令集进JIT 编译
### 测试


除了 BPF 工具链之外，内核还附带一个测试模块，其中包含针对经典eBPF 各种测试用例，可以对 BPF 解释器和 JIT 编译器执行。它位于 lib/test_bpf.c 中，
并且
```

  CONFIG_TEST_BPF=m

```
在模块构建并安装后，可以通过 insmod modprobe 针对 'test_bpf' 模块执行
测试套件。测试用例的结果（包括以纳秒为单位的时间）可以在内核日志（dmesg中找到
### 杂项


此外，trinity（Linux 系统调用模糊测试器）也内置了BPF SECCOMP-BPF
内核模糊测试的支持
### 作

撰写本文档是希望它能有所助益，并为潜在的 BPF 黑客或安全审计人员提供对底层
架构更好的概览
- Jay Schulist <jschlst@samba.org>
- Daniel Borkmann <daniel@iogearbox.net>
- Alexei Starovoitov <ast@kernel.org>
