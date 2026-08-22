
## BPF 指令集架构（ISA

eBPF（通常也称BPF）是一项源Linux 内核的技术，可以在特权上下文（如操作系统内核）中运行不可信程序。本文档规定BPF 指令集架构（ISA）
作为历史注记，BPF 最初代表伯克利数据包过滤器（Berkeley Packet Filter），但既然它能做的远超数据包过滤，该缩写已不再合理。BPF 现在被视为一个独立的术语，不代表任何含义。原始的 BPF 有时被称cBPF（经BPF），以区别于现在广泛部署eBPF（扩BPF）
## 文档约定


本文档中的关键词"MUST"MUST NOT"REQUIRED"SHALL"SHALL NOT"SHOULD"SHOULD NOT"RECOMMENDED"NOT RECOMMENDED"MAY"OPTIONAL"在且仅在它们以全大写形式出现时，应按
BCP 14 `<https://www.rfc-editor.org/info/rfc2119>`_
`<https://www.rfc-editor.org/info/rfc8174>`_
中的描述进行解释
为简洁和一致，本文档使用简写语法引用类型族，并在描述指令语义时引用若干说明性的助记函数。这些类型的有效值范围以及这些函数的语义在下小节中定义
### 类型

本文档使用符`SN` 来分别指定类型的符号性（`S`）和位宽（`N`）
  ==== =========
  S    Meaning
  ==== =========
  u    unsigned
  s    signed
  ==== =========


  ===== =========
  N     Bit width
  ===== =========
  8     8 bits
  16    16 bits
  32    32 bits
  64    64 bits
  128   128 bits
  ===== =========

例如，`u32` 是有效值为所32 位无符号数的类型，`s16` 是有效值为所16 位有符号数的类型
### 函数


以下字节交换函数与方向无关。即，同一个函数用于下面讨论的两个方向的转换
- be16：取一个无符号 16 位数，在主机字节序与大端
  (`IEN137 <https://www.rfc-editor.org/ien/ien137.txt>`_) 字节序之间转换- be32：取一个无符号 32 位数，在主机字节序与大端字节序之间转换- be64：取一个无符号 64 位数，在主机字节序与大端字节序之间转换- bswap16：取一个无符号 16 位数（大端或小端格式均可），返回位宽相同但字节序相反的数值- bswap32：取一个无符号 32 位数（大端或小端格式均可），返回位宽相同但字节序相反的数值- bswap64：取一个无符号 64 位数（大端或小端格式均可），返回位宽相同但字节序相反的数值- le16：取一个无符号 16 位数，在主机字节序与小端字节序之间转换- le32：取一个无符号 32 位数，在主机字节序与小端字节序之间转换- le64：取一个无符号 64 位数，在主机字节序与小端字节序之间转换
### 定义



  Sign Extend
    `X` 位的`A` **符号扩展**`Y` 位的`B`，意味着

    #. `A` 的所`X` 位复制到 `B` 的低 `X` 位    #. `B` 剩余`Y` - `X` 位设`A` 的最高有效位的值

  在大端平台上，将一8 位数 `A` 符号扩展16 位数 `B````

    A:          10000110
    B: 11111111 10000110

```
### 一致性组


实现不需要支持本文档中规定的所有指令（例如已弃用的指令）。相反，规定了一组一致性组。实现必须支base32 一致性组，并可以支持额外的一致性组，其中支持某个一致性组意味着必须支持该组中的所有指令
命名一致性组的使用实现了执行指令的运行时与生成指令供运行时使用的工具（如编译器）之间的互操作性。因此，基于一致性组的能力发现可以由用户手动完成，或由工具自动完成
每个一致性组有一个短 ASCII 标签（例"base32"），对应一组必须实现的指令。即，每条指令是一个或多个一致性组的成员
本文档定义了以下一致性组
- base32：包含本规范中定义的所有指令，除非另有说明- base64：包base32，加上明确注明属base64 一致性组的指令- atomic32：包32 位原子操作指令（`Atomic operations`_）- atomic64：包atomic32，加64 位原子操作指令- divmul32：包32 位除法、乘法和取模指令- divmul64：包divmul32，加64 位除法、乘法和取模指令- packet：已弃用的数据包访问指令
## 指令编码


BPF 有两种指令编码：

- 基本指令编码，使64 位编码一条指- 宽指令编码，在基本指令之后附加第二个 64 位，总共 128 位
### 基本指令编码


```

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |    opcode     |     regs      |            offset             |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |                              imm                              |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
**opcode**
```

    +-+-+-+-+-+-+-+-+
    |specific |class|
    +-+-+-+-+-+-+-+-+

  **specific**
    这些位的格式随指令类而变

  **class**
    指令类（`Instruction classes`_
```
**regs**
  源和目的寄存器编号，编码如下
```

    +-+-+-+-+-+-+-+-+
    |src_reg|dst_reg|
    +-+-+-+-+-+-+-+-+

  在小端主机上如下::

    +-+-+-+-+-+-+-+-+
    |dst_reg|src_reg|
    +-+-+-+-+-+-+-+-+

  **src_reg**
    源寄存器编号-10），除非另有说明
   （`64-bit immediate instructions`_ 将此字段另作他用
  **dst_reg**
    目的寄存器编号（0-10），除非另有说明
    （未来的指令可能将此字段另作他用
```
**offset**
  带符号整数偏移，用于指针运算，除非另有说明（某些算术指令将此字段另作他用
**imm**
  带符号整数立即
注意，多字节字段offset' 'imm'）的内容在大端主机上以大端字节序存储，在小端主机上以小端字节序存储
```

  opcode                  offset imm          assembly
         src_reg dst_reg
  07     0       1        00 00  44 33 22 11  r1 += 0x11223344 // little
         dst_reg src_reg
  07     1       0        00 00  11 22 33 44  r1 += 0x11223344 // big

```
注意，大多数指令不使用所有字段。未使用的字段应清零
### 宽指令编

某些指令定义为使用宽指令编码，它使用两个 32 位立即值。基本指令格式之后的 64 位包含一个伪指令，其 'opcode'dst_reg'src_reg' 'offset' 都设为零
```

  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |    opcode     |     regs      |            offset             |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |                              imm                              |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |                           reserved                            |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |                           next_imm                            |
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
**opcode**
  要执行的操作，如上所述编
**regs**
  源和目的寄存器编号（除非另有说明），如上所述编
**offset**
  带符号整数偏移，用于指针运算，除非另有说
**imm**
  带符号整数立即
**reserved**
  未使用，设为
**next_imm**
  第二个带符号整数立即
### 指令

'opcode' 字段的最低三位存储指令类
  =====  =====  ===============================  ===================================
  class  value  description                      reference
  =====  =====  ===============================  ===================================
  LD     0x0    非标准加载操                  `Load and store instructions`_
  LDX    0x1    加载到寄存器的操              `Load and store instructions`_
  ST     0x2    从立即值存储操                `Load and store instructions`_
  STX    0x3    从寄存器存储操作                 `Load and store instructions`_
  ALU    0x4    32 位算术操                   `Arithmetic and jump instructions`_
  JMP    0x5    64 位跳转操                   `Arithmetic and jump instructions`_
  JMP32  0x6    32 位跳转操                   `Arithmetic and jump instructions`_
  ALU64  0x7    64 位算术操                   `Arithmetic and jump instructions`_
  =====  =====  ===============================  ===================================

## 算术与跳转指

对于算术和跳转指令（`ALU`、`ALU64`、`JMP` ```

  +-+-+-+-+-+-+-+-+
  |  code |s|class|
  +-+-+-+-+-+-+-+-+

```
**code**
  操作码，其含义随指令类而变

**s（源*
  源操作数位置，除非另有说明，是以下之一
  .. table:: Source operand location

    ======  =====  ==============================================
    source  value  description
    ======  =====  ==============================================
    K       0      使用 32 'imm' 值作为源操作    X       1      使用 'src_reg' 寄存器值作为源操作    ======  =====  ==============================================

**instruction class**
  指令类（`Instruction classes`_
### 算术指令


对于其他方面相同的操作，`ALU` 使用 32 位宽操作数，`ALU64` 使用 64 位宽操作数。`ALU64` 指令属于 base64 一致性组，除非另有说明'code' 字段按如下方式编码操作，其中 'src' 指源操作数，'dst' 指目的寄存器的值
  =====  =====  =======  ===================================================================================
  name   code   offset   description
  =====  =====  =======  ===================================================================================
  ADD    0x0    0        dst += src
  SUB    0x1    0        dst -= src
  MUL    0x2    0        dst \*= src
  DIV    0x3    0        dst = (src != 0) ? (dst / src) : 0
  SDIV   0x3    1        dst = (src == 0) ? 0 : ((src == -1 && dst == LLONG_MIN) ? LLONG_MIN : (dst s/ src))
  OR     0x4    0        dst \|= src
  AND    0x5    0        dst &= src
  LSH    0x6    0        dst <<= (src & mask)
  RSH    0x7    0        dst >>= (src & mask)
  NEG    0x8    0        dst = -dst
  MOD    0x9    0        dst = (src != 0) ? (dst % src) : dst
  SMOD   0x9    1        dst = (src == 0) ? dst : ((src == -1 && dst == LLONG_MIN) ? 0: (dst s% src))
  XOR    0xa    0        dst ^= src
  MOV    0xb    0        dst = src
  MOVSX  0xb    8/16/32  dst = (s8,s16,s32)src
  ARSH   0xc    0        **符号扩展<Sign Extend>** dst >>= (src & mask)
  END    0xd    0        字节交换操作（见下文 `Byte swap instructions`_  =====  =====  =======  ===================================================================================

算术操作允许下溢和上溢，64 位或 32 位值会回绕。如BPF 程序执行将导致除以零，则目的寄存器改设为零。否则，对于 `ALU64`，如果执行将导致 `LLONG_MIN` 除以 -1，目的寄存器改设`LLONG_MIN`。对`ALU`，如果执行将导致 `INT_MIN` 除以 -1，目的寄存器改设`INT_MIN`
如果执行将导致对零取模，对于 `ALU64`，目的寄存器的值不变；而对`ALU`，目的寄存器的高 32 位被清零。否则，对于 `ALU64`，如果执行将导致 `LLONG_MIN` -1 取模，目的寄存器改设为零。对`ALU`，如果执行将导致 `INT_MIN` -1 取模，目的寄存器改设为零
```

  dst = (u32) ((u32) dst + (u32) src)

```
其中 '(u32)' 表示32 位被清零
```

  dst = dst + src

```
```

  dst = (u32) dst ^ (u32) imm

```
```

  dst = dst ^ imm

```
注意，大多数算术指令'offset' 设为零。只有三条指（`SDIV`、`SMOD`、`MOVSX`）的 'offset' 为非零
`ALU` 的除法、乘法和取模操作属于 "divmul32" 一致性组，`ALU64` 的除法、乘法和取模操作属于 "divmul64" 一致性组。除法和取模操作同时支持无符号和有符号形式
对于无符号操作（`DIV` `MOD`），对于 `ALU`imm' 被解释为 32 位无符号值。对`ALU64`imm' 首先 **符号扩展<Sign Extend>** 32 位到 64 位，然后解释64 位无符号值
对于有符号操作（`SDIV` `SMOD`），对于 `ALU`imm' 被解释为 32 位有符号值。对`ALU64`imm' 首先 **符号扩展<Sign Extend>** 32 位到 64 位，然后解释64 位有符号值
注意，当被除数或除数为负数时，有符号取模操作有多种定义，不同实现常因语言而异，例Python、Ruby 等与 C、Go、Java 等不同。本规范规定有符号取模必须使用截断除```

   a % n = a - n * trunc(a / n)

```
`MOVSX` 指令执行带符号扩展的传送操作。`{MOVSX, X, ALU}` **符号扩展<Sign Extend>** 8 位和 16 位操作数32 位操作数，并将剩余的32 位清零。`{MOVSX, X, ALU64}` **符号扩展<Sign Extend>** 8 位6 位和 32 位操作数64 位操作数。与其他算术指令不同，`MOVSX` 仅对寄存器源操作数（`X`）定义
```

  dst = (s64)imm

```
```

  dst = (u32)src

```
```

  dst = (u32)(s32)(s8)src



```
`NEG` 指令仅在源位清除（`K`）时定义
移位操作64 位操作使用掩0x3F3），32 位操作使用掩0x1F1）
### 字节交换指令


字节交换指令使用指令`ALU` `ALU64`，以4 'code' 字段 `END`
字节交换指令仅对目的寄存器操作，不使用单独的源寄存器或立即值
对于 `ALU`，opcode 中的 1 位源操作数字段用于选择操作转换自或转换到的字节序。对`ALU64` 位源操作数字段保留，必须设为零
  =====  ========  =====  =================================================
  class  source    value  description
  =====  ========  =====  =================================================
  ALU    LE        0      在主机字节序与小端之间转  ALU    BE        1      在主机字节序与大端之间转  ALU64  Reserved  0      无条件进行字节交  =====  ========  =====  =================================================

'imm' 字段编码交换操作的宽度。支持以下宽度：162 64。宽64 的操作属base64 一致性组，其他交换操作属base32 一致性组
示例
```

  dst = le16(dst)
  dst = le32(dst)
  dst = le64(dst)

```
```

  dst = be16(dst)
  dst = be32(dst)
  dst = be64(dst)

```
```

  dst = bswap16(dst)
  dst = bswap32(dst)
  dst = bswap64(dst)

```
### 跳转指令


`JMP32` 使用 32 位宽操作数并表示 base32 一致性组，`JMP` 对其他方面相同的操作使用 64 位宽操作数，并表base64 一致性组，除非另有说明code' 字段按如下方式编码操作：

  ========  =====  =======  =================================  ===================================================
  code      value  src_reg  description                        notes
  ========  =====  =======  =================================  ===================================================
  JA        0x0    0x0      PC += offset                       {JA, K, JMP} only
  JA        0x0    0x0      PC += imm                          {JA, K, JMP32} only
  JEQ       0x1    any      PC += offset if dst == src
  JGT       0x2    any      PC += offset if dst > src          unsigned
  JGE       0x3    any      PC += offset if dst >= src         unsigned
  JSET      0x4    any      PC += offset if dst & src
  JNE       0x5    any      PC += offset if dst != src
  JSGT      0x6    any      PC += offset if dst > src          signed
  JSGE      0x7    any      PC += offset if dst >= src         signed
  CALL      0x8    0x0      call helper function by static ID  {CALL, K, JMP} only, see `Helper functions`_
  CALL      0x8    0x1      call PC += imm                     {CALL, K, JMP} only, see `Program-local functions`_
  CALL      0x8    0x2      call helper function by BTF ID     {CALL, K, JMP} only, see `Helper functions`_
  EXIT      0x9    0x0      return                             {CALL, K, JMP} only
  JLT       0xa    any      PC += offset if dst < src          unsigned
  JLE       0xb    any      PC += offset if dst <= src         unsigned
  JSLT      0xc    any      PC += offset if dst < src          signed
  JSLE      0xd    any      PC += offset if dst <= src         signed
  ========  =====  =======  =================================  ===================================================

其中 'PC' 表示程序计数器，要递增的偏移是相对于跳转指令之后那条指令的 64 位指令为单位。因'PC += 1' 会跳过下一条指令的执行（如果它是基本指令），或者如果下一条指令是 128 位宽指令，则导致未定义行为
示例
```

  if (s32)dst s>= (s32)src goto +offset

```
其中 's>=' 表示有符号的 '>=' 比较
```

  if dst <= (u64)(s64)imm goto +offset

```
```

  gotol +imm

```
其中 'imm' 表示分支偏移来自 'imm' 字段
注意，有两种风格`JA` 指令。`JMP` 类允许由 'offset' 字段指定16 位跳转偏移，`JMP32` 类允许由 'imm' 字段指定32 位跳转偏移。大16 位的条件跳转可以转换为小16 位的条件跳转加上 32 位无条件跳转
所`CALL` `JA` 指令都属base32 一致性组
#### 辅助函数


辅助函数是一个概念，通过BPF 程序可以调用底层平台暴露的一组函数调用
历史上，每个辅助函数由编码在 'imm' 字段中的静ID 标识。辅助函数的进一步文档不在本文档范围内，标准化留待未来工作，但其使用已广泛部署，更多信息可在平台特定文档（例Linux 内核文档）中找到
支持 BPF 类型格式（BTF）的平台支持通过编码'imm' 字段中的 BTF ID 来标识辅助函数，其中 BTF ID 标识辅助函数的名称和类型。BTF 的进一步文档不在本文档范围内，标准化留待未来工作，但其使用已广泛部署，更多信息可在平台特定文档（例Linux 内核文档）中找到
#### 程序局部函
程序局部函数是由与调用者相同的 BPF 程序暴露的函数，并通过相对于调用指令之后那条指令的偏移引用，类似于 `JA`。偏移编码在调用指令'imm' 字段中。程序局部函数内`EXIT` 会返回到调用者
## 加载与存储指

对于加载和存储指令（`LD`、`LDX`、`ST` `STX`），
```

  +-+-+-+-+-+-+-+-+
  |mode |sz |class|
  +-+-+-+-+-+-+-+-+

```
**mode**
  模式修饰符是以下之一
  .. table:: Mode modifier

    =============  =====  ====================================  =============
    mode modifier  value  description                           reference
    =============  =====  ====================================  =============
    IMM            0      64 位立即值指                      `64-bit immediate instructions`_
    ABS            1      传统 BPF 数据包访问（绝对          `Legacy BPF Packet access instructions`_
    IND            2      传统 BPF 数据包访问（间接          `Legacy BPF Packet access instructions`_
    MEM            3      常规加载和存储操                    `Regular load and store operations`_
    MEMSX          4      符号扩展加载操作                      `Sign-extension load operations`_
    ATOMIC         6      原子操作                              `Atomic operations`_
    =============  =====  ====================================  =============

**sz（大小）**
  大小修饰符是以下之一
  .. table:: Size modifier

    ====  =====  =====================
    size  value  description
    ====  =====  =====================
    W     0      word        (4 bytes)
    H     1      half word   (2 bytes)
    B     2      byte
    DW    3      double word (8 bytes)
    ====  =====  =====================

  使用 `DW` 的指令属base64 一致性组
**class**
  指令类（`Instruction classes`_
### 常规加载与存储操

`MEM` 模式修饰符用于编码在寄存器与内存之间传输数据的常规加载和存储指令
```

  *(size *) (dst + offset) = src

```
```

  *(size *) (dst + offset) = imm

```
```

  dst = *(unsigned size *) (src + offset)

```
其中 '<size>' 是以下之一：`B`、`H`、`W` `DW`，且
'unsigned size' 是以下之一：u8、u16、u32 u64
### 符号扩展加载操作


`MEMSX` 模式修饰符用于编**符号扩展<Sign Extend>** 的加载指令，在寄存器与内存之间传输数据
```

  dst = *(signed size *) (src + offset)

```
其中 '<size>' 是以下之一：`B`、`H` `W`，且
'signed size' 是以下之一：s8、s16 s32
### 原子操作


原子操作是对内存进行操作、且不能被其BPF 程序或本规范之外的方式对相同内存区域的访问打断或破坏的操作
BPF 支持的所有原子操作都编码为使`ATOMIC` 模式修饰符的存储操作，如下所示：

- `{ATOMIC, W, STX}` 用于 32 位操作，属于 "atomic32" 一致性组- `{ATOMIC, DW, STX}` 用于 64 位操作，属于 "atomic64" 一致性组- 不支8 位和 16 位宽的原子操作
'imm' 字段用于编码实际的原子操作。简单原子操作使用为'imm' 字段中编码算术操作所定义的值的一个子集来编码原子操作
  ========  =====  ===========
  imm       value  description
  ========  =====  ===========
  ADD       0x00   atomic add
  OR        0x40   atomic or
  AND       0x50   atomic and
  XOR       0xa0   atomic xor
  ========  =====  ===========


```

  *(u32 *)(dst + offset) += src

```
```

  *(u64 *)(dst + offset) += src

```
除了简单原子操作外，还有一个修饰符和两个复杂原子操作：

  ===========  ================  ===========================
  imm          value             description
  ===========  ================  ===========================
  FETCH        0x01              modifier: return old value
  XCHG         0xe0 | FETCH      atomic exchange
  CMPXCHG      0xf0 | FETCH      atomic compare and exchange
  ===========  ================  ===========================

`FETCH` 修饰符对简单原子操作是可选的，对复杂原子操作总是设置。如果设置了 `FETCH` 标志，则操作还会用修改前内存中的值覆`src`
`XCHG` 操作原子地将 `src` `dst + offset` 寻址的值交换
`CMPXCHG` 操作`dst + offset` 寻址的值与 `R0` 比较。如果匹配，`dst + offset` 寻址的值被替换`src`。无论哪种情况，`dst + offset` 处的值在操作前会被零扩展并加载回 `R0`
### 64 位立即值指

带有 `IMM` 'mode' 修饰符的指令使用 `Instruction encoding`_ 中定义的宽指令编码，并使用基本指令的 'src_reg' 字段保存操作码子类型
下表定义了一`{IMM, DW, LD}` 指令，在 'src_reg' 字段中使用操作码子类型，使用如下进一步定义的"map"等新术语
  =======  =========================================  ===========  ==============
  src_reg  pseudocode                                 imm type     dst type
  =======  =========================================  ===========  ==============
  0x0      dst = (next_imm << 32) | imm               integer      integer
  0x1      dst = map_by_fd(imm)                       map fd       map
  0x2      dst = map_val(map_by_fd(imm)) + next_imm   map fd       data address
  0x3      dst = var_addr(imm)                        variable id  data address
  0x4      dst = code_addr(imm)                       integer      code address
  0x5      dst = map_by_idx(imm)                      map index    map
  0x6      dst = map_val(map_by_idx(imm)) + next_imm  map index    data address
  =======  =========================================  ===========  ==============

其中

- map_by_fd(imm) 表示32 位文件描述符转换map 的地址（见 `Maps`_- map_by_idx(imm) 表示32 位索引转换为 map 的地址
- map_val(map) 获取给定 map 中第一个值的地址
- var_addr(imm) 获取具有给定 id 的平台变量（`Platform Variables`_）的地址
- code_addr(imm) 获取以（64 位）指令数为单位、指定相对偏移处指令的地址
- 'imm type' 可供反汇编器用于显示
- 'dst type' 可用于验证和 JIT 编译目的

#### Maps


Map 是某些平台上 BPF 程序可访问的共享内存区域。一map 可以具有单独文档中定义的多种语义，可能有也可能没有单一的连续内存区域，'map_val(map)' 目前仅对已具有单一连续内存区域map 定义
每个 map 可以有一个文件描述符（fd）（如果平台支持），其中 'map_by_fd(imm)' 表示获取具有指定文件描述符的 map。每BPF 程序也可以定义为在加载时使用一组与该程序关联的 mapmap_by_idx(imm)' 表示获取包含该指令的 BPF 程序所关联集合中给定索引的 map
#### 平台变量


平台变量是由运行时暴露、由整数 id 标识的内存区域，在某些平台上可供 BPF 程序访问var_addr(imm)' 操作表示获取由给id 标识的内存区域的地址
### 传统 BPF 数据包访问指

BPF 之前引入了用于访问数据包数据的特殊指令，这些指令从经BPF 沿用而来。这些指令使用指令类 `LD`、大小修饰符 `W`、`H` `B`，以及模式修饰符 `ABS` `IND`dst_reg' 'offset' 字段设为零，'src_reg' `ABS` 设为零。然而，这些指令已弃用，不应再使用。所有传统数据包访问指令属于 "packet" 一致性组