
## BPF LLVM 重定

本文档描LLVM BPF 后端的重定位类型
## 重定位记

LLVM BPF 后端使用以下 16 字节记录每个重定```

  typedef struct
  {
    Elf64_Addr    r_offset;  // Offset from the beginning of section.
    Elf64_Xword   r_info;    // Relocation type and symbol index.
  } Elf64_Rel;

```
```

  int g1 __attribute__((section("sec")));
  int g2 __attribute__((section("sec")));
  static volatile int l1 __attribute__((section("sec")));
  static volatile int l2 __attribute__((section("sec")));
  int test() {
    return g1 + g2 + l1 + l2;
  }

```
使用 `clang --target=bpf -O2 -c test.c` 编译，以下是
```

       0:       18 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r1 = 0 ll
                0000000000000000:  R_BPF_64_64  g1
       2:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
       3:       18 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r2 = 0 ll
                0000000000000018:  R_BPF_64_64  g2
       5:       61 20 00 00 00 00 00 00 r0 = *(u32 *)(r2 + 0)
       6:       0f 10 00 00 00 00 00 00 r0 += r1
       7:       18 01 00 00 08 00 00 00 00 00 00 00 00 00 00 00 r1 = 8 ll
                0000000000000038:  R_BPF_64_64  sec
       9:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
      10:       0f 10 00 00 00 00 00 00 r0 += r1
      11:       18 01 00 00 0c 00 00 00 00 00 00 00 00 00 00 00 r1 = 12 ll
                0000000000000058:  R_BPF_64_64  sec
      13:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
      14:       0f 10 00 00 00 00 00 00 r0 += r1
      15:       95 00 00 00 00 00 00 00 exit

```
上面有四`LD_imm64` 指令的四个重定位。以`llvm-readelf -r test.o` 显示了这四个二进制```

  Relocation section '.rel.text' at offset 0x190 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000000  0000000600000001 R_BPF_64_64            0000000000000000 g1
  0000000000000018  0000000700000001 R_BPF_64_64            0000000000000004 g2
  0000000000000038  0000000400000001 R_BPF_64_64            0000000000000000 sec
  0000000000000058  0000000400000001 R_BPF_64_64            0000000000000000 sec

```
每个重定位由 `Offset` 字节）和 `Info` 字节）表示。例如，第一个重定位对应于第一指令（Offset 0x0），相应`Info` 指示`R_BPF_64_64`（类1）的重定位类型以及符表中的条目（条目 6）```

  Symbol table '.symtab' contains 8 entries:
     Num:    Value          Size Type    Bind   Vis       Ndx Name
       0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT   UND
       1: 0000000000000000     0 FILE    LOCAL  DEFAULT   ABS test.c
       2: 0000000000000008     4 OBJECT  LOCAL  DEFAULT     4 l1
       3: 000000000000000c     4 OBJECT  LOCAL  DEFAULT     4 l2
       4: 0000000000000000     0 SECTION LOCAL  DEFAULT     4 sec
       5: 0000000000000000   128 FUNC    GLOBAL DEFAULT     2 test
       6: 0000000000000000     4 OBJECT  GLOBAL DEFAULT     4 g1
       7: 0000000000000004     4 OBJECT  GLOBAL DEFAULT     4 g2

```
6 个条目是值为 0 的全局变量 `g1`
类似地，第二个重定位位于 `.text` 偏移 `0x18`，指3，类型为 `R_BPF_64_64`，并引用符号
表中的条7。第二个重定位解析为全局变量 `g2`，其符号值为 4。该符号值表示存储全局变量
`g2` 初始值的 `.data` 节起始处的偏移
第三和第四个重定位引用静态变`l1` `l2`。从上面`.rel.text` 节看，不清楚它们真正
引用哪些符号，因为它们都引用符号表条4，即符号 `sec`，它具有 `STT_SECTION` 类型并代一个节。因此对于静态变量或函数，节偏移被写入原insn 缓冲区，这称`A`（addend）查看上面insn `7` `11`，它们具有节偏移 `8` `12`。从符号表我们可以找到它们对应于
`l1` `l2` 的条`2` `3`
一般来说，对于全局变量和函数，`A` 0；对于静态变函数，`A` 是节偏移或基于节偏移某种计算结果。非节偏移的情况指的是函数调用。更多细节见下文
## 不同的重定位类型


支持六种重定位类型。以下是概述```

  Enum  ELF Reloc Type     Description      BitSize  Offset        Calculation
  0     R_BPF_NONE         None
  1     R_BPF_64_64        ld_imm64 insn    32       r_offset + 4  S + A
  2     R_BPF_64_ABS64     normal data      64       r_offset      S + A
  3     R_BPF_64_ABS32     normal data      32       r_offset      S + A
  4     R_BPF_64_NODYLD32  .BTF[.ext] data  32       r_offset      S + A
  10    R_BPF_64_32        call insn        32       r_offset + 4  (S + A) / 8 - 1

```
例如，`R_BPF_64_64` 重定位类型用`ld_imm64` 指令。实际待重定位的数据 或节偏移）存`r_offset + 4`，读/写数据位宽为 32 字节）。该重定位可以用符号值加上隐式加数来解析注意 `BitSize` 32，这意味着节偏移必须小于或等于 `UINT32_MAX`，这LLVM BPF 后端强制
执行
在另一种情况下，`R_BPF_64_ABS64` 重定位类型用于普通的 64 位数据。实际待重定位的数据存储
`r_offset`，读/写数据位宽为 64 字节）。该重定位可以用符号值加上隐式加数来解析
`R_BPF_64_ABS32` `R_BPF_64_NODYLD32` 类型都用32 位数据。但 `R_BPF_64_NODYLD32`
特指 `.BTF` `.BTF.ext` 节中的重定位。对于像 bcc 这样涉及 llvm `ExecutionEngine
RuntimeDyld` 的情况，`R_BPF_64_NODYLD32` 类型的重定位不应解析为实际的函数/变量地址。否则，
`.BTF` `.BTF.ext` 将变得对 bcc 和内核不可用
类型 `R_BPF_64_32` 用于 call 指令。call 目标的节偏移存储`r_offset + 4`2 位），并
计算`(S + A) / 8 - 1`
## 示例


类型 `R_BPF_64_64` `R_BPF_64_32` 用于解析 `ld_imm64`
```

  __attribute__((noinline)) __attribute__((section("sec1")))
  int gfunc(int a, int b) {
    return a * b;
  }
  static __attribute__((noinline)) __attribute__((section("sec1")))
  int lfunc(int a, int b) {
    return a + b;
  }
  int global __attribute__((section("sec2")));
  int test(int a, int b) {
    return gfunc(a, b) +  lfunc(a, b) + global;
  }

```
使用 `clang --target=bpf -O2 -c test.c` 编译，我们将得到
```

  Disassembly of section .text:

  0000000000000000 <test>:
         0:       bf 26 00 00 00 00 00 00 r6 = r2
         1:       bf 17 00 00 00 00 00 00 r7 = r1
         2:       85 10 00 00 ff ff ff ff call -1
                  0000000000000010:  R_BPF_64_32  gfunc
         3:       bf 08 00 00 00 00 00 00 r8 = r0
         4:       bf 71 00 00 00 00 00 00 r1 = r7
         5:       bf 62 00 00 00 00 00 00 r2 = r6
         6:       85 10 00 00 02 00 00 00 call 2
                  0000000000000030:  R_BPF_64_32  sec1
         7:       0f 80 00 00 00 00 00 00 r0 += r8
         8:       18 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r1 = 0 ll
                  0000000000000040:  R_BPF_64_64  global
        10:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
        11:       0f 10 00 00 00 00 00 00 r0 += r1
        12:       95 00 00 00 00 00 00 00 exit

  Disassembly of section sec1:

  0000000000000000 <gfunc>:
         0:       bf 20 00 00 00 00 00 00 r0 = r2
         1:       2f 10 00 00 00 00 00 00 r0 *= r1
         2:       95 00 00 00 00 00 00 00 exit

  0000000000000018 <lfunc>:
         3:       bf 20 00 00 00 00 00 00 r0 = r2
         4:       0f 10 00 00 00 00 00 00 r0 += r1
         5:       95 00 00 00 00 00 00 00 exit

```
第一个重定位对应`gfunc(a, b)`，其`gfunc` 的值为 0，因`call` 指令偏移`(0 + 0)/8 - 1 = -1`。第二个重定位对应于 `lfunc(a, b)`，其`lfunc` 的节偏移`0x18`因此 `call` 指令偏移`(0 + 0x18)/8 - 1 = 2`。第三个重定位对应于 `global` ld_imm64其节偏移`0````

  int global() { return 0; }
  struct t { void *g; } gbl = { global };

```
使用 `clang --target=bpf -O2 -g -c test.c` 编译，我们将通过命令`.data` 节中看到如下
重定```

  Relocation section '.rel.data' at offset 0x458 contains 1 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000000  0000000700000002 R_BPF_64_ABS64         0000000000000000 global

```
该重定位表示 `.data` 节的8 字节应填充为 `global` 变量的地址
通过 `llvm-readelf` 输出，我们可以看dwarf 节有一```

  Relocation section '.rel.debug_info' at offset 0x468 contains 13 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000006  0000000300000003 R_BPF_64_ABS32         0000000000000000 .debug_abbrev
  000000000000000c  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000012  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000016  0000000600000003 R_BPF_64_ABS32         0000000000000000 .debug_line
  000000000000001a  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  000000000000001e  0000000200000002 R_BPF_64_ABS64         0000000000000000 .text
  000000000000002b  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000037  0000000800000002 R_BPF_64_ABS64         0000000000000000 gbl
  0000000000000040  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  ......

```
```

  Relocation section '.rel.BTF' at offset 0x538 contains 1 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000084  0000000800000004 R_BPF_64_NODYLD32      0000000000000000 gbl

  Relocation section '.rel.BTF.ext' at offset 0x548 contains 2 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  000000000000002c  0000000200000004 R_BPF_64_NODYLD32      0000000000000000 .text
  0000000000000040  0000000200000004 R_BPF_64_NODYLD32      0000000000000000 .text

```

## CO-RE 重定

从目标文件的角度来看，CO-RE 机制是作为一CO-RE 特定的重定位记录实现的。这些重定位记录
ELF 重定位无关，并编码在 .BTF.ext 节中。有.BTF.ext 结构的更多信息，请参Documentation/bpf/btf.rst <BTF_Ext_Section>
CO-RE 重定位应用于 BPF 指令，以在加载时用与目标内核相关的信息更新指令的立即数或偏移字段
要打补丁的字段根据指令类选择
- 对于 BPF_ALU、BPF_ALU64、BPF_LD，`immediate` 字段被补丁；
- 对于 BPF_LDX、BPF_STX、BPF_ST，`offset` 字段被补丁；
- BPF_JMP、BPF_JMP32 指令**不应**被补丁
## 重定位种

有几CO-RE 重定位，可分为三组：

- 基于字段 - 用与字段相关的信息补丁指令，例如BPF_LDX 指令offset 字段更改为反  目标内核中特定结构体字段的偏移
- 基于类型 - 用与类型相关的信息补丁指令，例如BPF_ALU move 指令immediate 字段更改  0 1，以反映目标内核中是否存在特定类型
- 基于枚举 - 用与枚举相关的信息补丁指令，例如BPF_LD_IMM64 指令immediate 字段更改  反映目标内核中特定枚举字面量的值
重定位种类的完整列表由以enum 表示
enum bpf_core_relo_kind {
	BPF_CORE_FIELD_BYTE_OFFSET = 0,  /** field byte offset **/
	BPF_CORE_FIELD_BYTE_SIZE   = 1,  /** field size in bytes **/
	BPF_CORE_FIELD_EXISTS      = 2,  /** field existence in target kernel **/
	BPF_CORE_FIELD_SIGNED      = 3,  /** field signedness (0 - unsigned, 1 - signed) **/
	BPF_CORE_FIELD_LSHIFT_U64  = 4,  /** bitfield-specific left bitshift **/
	BPF_CORE_FIELD_RSHIFT_U64  = 5,  /** bitfield-specific right bitshift **/
	BPF_CORE_TYPE_ID_LOCAL     = 6,  /** type ID in local BPF object **/
	BPF_CORE_TYPE_ID_TARGET    = 7,  /** type ID in target kernel **/
	BPF_CORE_TYPE_EXISTS       = 8,  /** type existence in target kernel **/
	BPF_CORE_TYPE_SIZE         = 9,  /** type size in bytes **/
	BPF_CORE_ENUMVAL_EXISTS    = 10, /** enum value existence in target kernel **/
	BPF_CORE_ENUMVAL_VALUE     = 11, /** enum value integer value **/
	BPF_CORE_TYPE_MATCHES      = 12, /** type match in target kernel **/
 };

注意
- `BPF_CORE_FIELD_LSHIFT_U64` `BPF_CORE_FIELD_RSHIFT_U64` 应该用于使用以下算法读取
  位域值：

  .. code-block:: c

     // To read bitfield `f` from `struct s`
     is_signed = relo(s->f, BPF_CORE_FIELD_SIGNED)
     off = relo(s->f, BPF_CORE_FIELD_BYTE_OFFSET)
     sz  = relo(s->f, BPF_CORE_FIELD_BYTE_SIZE)
     l   = relo(s->f, BPF_CORE_FIELD_LSHIFT_U64)
     r   = relo(s->f, BPF_CORE_FIELD_RSHIFT_U64)
     // define `v` as signed or unsigned integer of size `sz`
     v = **({s|u}<sz> **)((void *)s + off)
     v <<= l
     v >>= r

- `BPF_CORE_TYPE_MATCHES` 查询匹配关系，定义如下：

  - 对于整数：类型和符号都匹配则类型匹配  - 对于数组和指针：目标类型被递归匹配  - 对于结构体和联合体：

    - 局部成员需要以相同名称存在于目标中
    - 对于每个成员，我们递归检查匹配，除非它已经在指针之后，在这种情况下我们只检查匹      的名称和兼容kind
  - 对于枚举
    - 局部变体必须按符号名称（而非数值）在目标中有匹配；

    - 大小必须匹配（但 enum 可以匹配 enum64，反之亦然）
  - 对于函数指针
    - 局部类型中参数的数量和位置必须匹配目标    - 对于每个参数和返回值，我们递归检查匹配
## CO-RE 重定位记

重定位记录编码为以下结构
struct bpf_core_relo {
	__u32 insn_off;
	__u32 type_id;
	__u32 access_str_off;
	enum bpf_core_relo_kind kind;
};

- `insn_off` - 与此重定位关联的代码节内的指令偏移（以字节为单位）；

- `type_id` - 可重定位类型或字段的"（包含）实体BTF 类型 ID
- `access_str_off` - 对应 .BTF 字符串节内的偏移。字符串的解释取决于具体的重定位种类
  - 对于基于字段的重定位，字符串使用字段和数组索引的序列（以冒号）分隔）来编码被访问
    的字段。它在概念上非常接近 LLVM `getelementptr <GEP_>`_ 指令用于标识字段偏移的参数    例如，考虑以下 C 代码
    .. code-block:: c

       struct sample {
           int a;
           int b;
           struct { int c[^10^]; };
       } __attribute__((preserve_access_index));
       struct sample *s;

    - `s[^0^].a` 的访问会被编码为 `0:0`
      - `0`：`s` 的第一个元素（如同 `s` 是一个数组）      - `0`：`struct sample` 中字`a` 的索引
    - `s->a` 的访问也会被编码`0:0`    - `s->b` 的访问会被编码为 `0:1`
      - `0`：`s` 的第一个元素；
      - `1`：`struct sample` 中字`b` 的索引
    - `s[^1^].c[^5^]` 的访问会被编码为 `1:2:0:5`
      - `1`：`s` 的第二个元素      - `2`：`struct sample` 中匿名结构体字段的索引；
      - `0`：匿名结构体中字`c` 的索引；
      - `5`：访问数组元#5
  - 对于基于类型的重定位，字符串应为 "0"
  - 对于基于枚举值的重定位，字符串包含其枚举类型内枚举值的索引
- `kind` - `enum bpf_core_relo_kind` 之一

## CO-RE 重定位示

对于以下 C 代码
struct foo {
   int a;
   int b;
   unsigned c:15;
 } __attribute__((preserve_access_index));

 enum bar { U, V };

使用以下 BTF 定义
...
[^2^] STRUCT 'foo' size=8 vlen=2
        'a' type_id=3 bits_offset=0
        'b' type_id=3 bits_offset=32
        'c' type_id=4 bits_offset=64 bitfield_size=15
[^3^] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
[^4^] INT 'unsigned int' size=4 bits_offset=0 nr_bits=32 encoding=(none)
...
[^16^] ENUM 'bar' encoding=UNSIGNED size=4 vlen=2
        'U' val=0
        'V' val=1

当使`__attribute__((preserve_access_index))` 时，字段偏移重定位会自动生成，例如：

  void alpha(struct foo **s, volatile unsigned long **g) {
    *g = s->a;
    s->a = 1;
  }

  00 <alpha>:
    0:  r3 = **(s32 **)(r1 + 0x0)
           00:  CO-RE <byte_off> [^2^] struct foo::a (0:0)
    1:  **(u64 **)(r2 + 0x0) = r3
    2:  **(u32 **)(r1 + 0x0) = 0x1
           10:  CO-RE <byte_off> [^2^] struct foo::a (0:0)
    3:  exit


所有重定位种类都可以通过内置函数请求。例如基于字段的重定位：

  void bravo(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_field_info(s->b, 0 /** field byte offset */);
    **g = __builtin_preserve_field_info(s->b, 1 /** field byte size */);
    **g = __builtin_preserve_field_info(s->b, 2 /** field existence */);
    **g = __builtin_preserve_field_info(s->b, 3 /** field signedness */);
    **g = __builtin_preserve_field_info(s->c, 4 /** bitfield left shift */);
    **g = __builtin_preserve_field_info(s->c, 5 /** bitfield right shift */);
  }

  20 <bravo>:
     4:     r1 = 0x4
            20:  CO-RE <byte_off> [^2^] struct foo::b (0:1)
     5:     **(u64 **)(r2 + 0x0) = r1
     6:     r1 = 0x4
            30:  CO-RE <byte_sz> [^2^] struct foo::b (0:1)
     7:     **(u64 **)(r2 + 0x0) = r1
     8:     r1 = 0x1
            40:  CO-RE <field_exists> [^2^] struct foo::b (0:1)
     9:     **(u64 **)(r2 + 0x0) = r1
    10:     r1 = 0x1
            50:  CO-RE <signed> [^2^] struct foo::b (0:1)
    11:     **(u64 **)(r2 + 0x0) = r1
    12:     r1 = 0x31
            60:  CO-RE <lshift_u64> [^2^] struct foo::c (0:2)
    13:     **(u64 **)(r2 + 0x0) = r1
    14:     r1 = 0x31
            70:  CO-RE <rshift_u64> [^2^] struct foo::c (0:2)
    15:     **(u64 **)(r2 + 0x0) = r1
    16:     exit


基于类型的重定位
  void charlie(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_type_info(**s, 0 /** type existence **/);
    **g = __builtin_preserve_type_info(**s, 1 /** type size **/);
    **g = __builtin_preserve_type_info(**s, 2 /** type matches **/);
    **g = __builtin_btf_type_id(**s, 0 /** type id in this object file **/);
    **g = __builtin_btf_type_id(**s, 1 /** type id in target kernel **/);
  }

  88 <charlie>:
    17:     r1 = 0x1
            88:  CO-RE <type_exists> [^2^] struct foo
    18:     **(u64 **)(r2 + 0x0) = r1
    19:     r1 = 0xc
            98:  CO-RE <type_size> [^2^] struct foo
    20:     **(u64 **)(r2 + 0x0) = r1
    21:     r1 = 0x1
            a8:  CO-RE <type_matches> [^2^] struct foo
    22:     **(u64 **)(r2 + 0x0) = r1
    23:     r1 = 0x2 ll
            b8:  CO-RE <local_type_id> [^2^] struct foo
    25:     **(u64 **)(r2 + 0x0) = r1
    26:     r1 = 0x2 ll
            d0:  CO-RE <target_type_id> [^2^] struct foo
    28:     **(u64 **)(r2 + 0x0) = r1
    29:     exit

基于枚举的重定位
  void delta(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_enum_value(**(enum bar **)U, 0 /** enum literal existence */);
    **g = __builtin_preserve_enum_value(**(enum bar **)V, 1 /** enum literal value */);
  }

  f0 <delta>:
    30:     r1 = 0x1 ll
            f0:  CO-RE <enumval_exists> [^16^] enum bar::U = 0
    32:     **(u64 **)(r2 + 0x0) = r1
    33:     r1 = 0x1 ll
            108:  CO-RE <enumval_value> [^16^] enum bar::V = 1
    35:     **(u64 **)(r2 + 0x0) = r1
    36:     exit
