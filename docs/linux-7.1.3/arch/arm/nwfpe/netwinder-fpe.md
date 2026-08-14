## Current State


下面描述了 NetWinder 浮点仿真器的当前状态。

下面的命名法用于描述浮点指令。它遵循 ARM 手册中的约定。

```

  <S|D|E> = <single|double|extended>, no default
  {P|M|Z} = {round to +infinity,round to -infinity,round to zero},
            default = round to nearest

```
注意：{} 括起来的项是可选的。

### Floating Point Coprocessor Data Transfer Instructions (CPDT)


LDF/STF - 加载与存储浮点数据

<LDF|STF>{cond}<S|D|E> Fd, Rn
<LDF|STF>{cond}<S|D|E> Fd, [Rn, #<expression>]{!}
<LDF|STF>{cond}<S|D|E> Fd, [Rn], #<expression>

这些指令已完整实现。

LFM/SFM - 加载与存储多个浮点数据

Form 1 语法：
<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn]
<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn, #<expression>]{!}
<LFM|SFM>{cond}<S|D|E> Fd, <count>, [Rn], #<expression>

Form 2 语法：
<LFM|SFM>{cond}<FD,EA> Fd, <count>, [Rn]{!}

这些指令已完整实现。它们为每个浮点寄存器向指令给定的内存位置存储/加载三个字。内存中的格式不太可能与其他实现（尤其是实际硬件）兼容。ARM 手册中对此有特别说明。

### Floating Point Coprocessor Register Transfer Instructions (CPRT)


转换、读/写状态/控制寄存器指令

FLT{cond}<S,D,E>{P,M,Z} Fn, Rd          Convert integer to floating point
FIX{cond}{P,M,Z} Rd, Fn                 Convert floating point to integer
WFS{cond} Rd                            Write floating point status register
RFS{cond} Rd                            Read floating point status register
WFC{cond} Rd                            Write floating point control register
RFC{cond} Rd                            Read floating point control register

FLT/FIX 已完整实现。

RFS/WFS 已完整实现。

RFC/WFC 已完整实现。RFC/WFC 是仅 supervisor 指令，目前会检查 CPU 模式，若不是从 supervisor 模式调用则产生非法指令陷阱。

Compare 指令

CMF{cond} Fn, Fm        Compare floating
CMFE{cond} Fn, Fm       Compare floating with exception
CNF{cond} Fn, Fm        Compare negated floating
CNFE{cond} Fn, Fm       Compare negated floating with exception

这些均已完整实现。

### Floating Point Coprocessor Data Instructions (CPDT)


双目运算：

ADF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - add
SUF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - subtract
RSF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse subtract
MUF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - multiply
DVF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - divide
RDV{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse divide

这些均已完整实现。

FML{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast multiply
FDV{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast divide
FRD{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - fast reverse divide

这些也都已完整实现。它们使用与非快速版本相同的算法。因此，在本实现中它们的性能等同于 MUF/DVF/RDV 指令。这符合 ARM 手册的规定。手册指出这些仅针对单精度操作数定义，在实际的 FPA11 硬件上它们对双精度或扩展精度操作数无效。仿真器目前不检查所请求的权限条件，而是直接执行所请求的操作。

RMF{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - IEEE remainder

这已完整实现。

单目运算：

MVF{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - move
MNF{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - move negated

这些均已完整实现。

ABS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - absolute value
SQT{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - square root
RND{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - round

这些均已完整实现。

URD{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - unnormalized round
NRM{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - normalize

这些已实现。URD 使用与 RND 指令相同的代码实现。由于 URD 不能返回非规格化数，NRM 变成了空操作（NOP）。

库调用：

POW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - power
RPW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse power
POL{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - polar angle (arctan2)

LOG{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base 10
LGN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base e
EXP{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - exponent
SIN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - sine
COS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - cosine
TAN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - tangent
ASN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arcsine
ACS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arccosine
ATN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arctangent

这些尚未实现。编译器当前不会发出它们，而是由 libc 中的例程处理。FPA11 硬件也未实现它们，而是由浮点支持代码处理。它们应在未来的版本中实现。

Signalling：

信号已实现。然而当前由 Rebel.com 生成的 ELF 内核中存在一个 bug，导致该模块无法生成 SIGFPE。这是由于未能正确地把 fp_current 别名到内核变量 current_set[^0^]。

本发行版自带的内核（vmlinux-nwfpe-0.93）包含针对该问题的修复，并且直接集成了当前版本的仿真器。使用该内核可以不加载任何浮点模块运行。它作为该技术的演示，以及为那些依赖信号进行浮点工作的人而提供。使用模块并非严格要求。

一个模块（由 Russell King 提供的，或本发行版中的）可以被加载以替换内建于内核的仿真器功能。
