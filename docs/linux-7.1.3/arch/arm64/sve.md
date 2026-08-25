## AArch64 Linux 对可扩展向量扩展（SVE）的支持


作者：Dave Martin <Dave.Martin@arm.com>

日期 August 2017

本文档简要概述了 Linux 提供给用户空间的接口，用以支ARM 可扩展向量扩展（SVE，Scalable Vector Extension）的使用，包括由可扩展矩阵扩展（SME，Scalable Matrix Extension）引入的 Streaming SVE 模式的交互

本文仅概述最重要的特性与问题，并非详尽无遗

本文档无意描SVE 架构或编程模型。为帮助理解，附A 中包含了 SVE 相关编程模型特性的一份最小化描述


### 1. 概述


- SVE 寄存Z0..Z31、P0..P15 以及 FFR，连同当前的向量长度 VL，均按线程进行跟踪

- streaming 模式下，除非系统中存HWCAP2_SME_FA64，否则无法访FFR；当不支FFR 时，使用这些接口访问 streaming 模式下的 FFR 会将其读写为零

- SVE 的存在通过辅助向量 AT_HWCAP 条目中的 HWCAP_SVE 报告给用户空间。该标志的存在意味着 SVE 指令和寄存器以及本文档描述的 Linux 专用系统接口均存在。SVE /proc/cpuinfo 中报告为 "sve"

- 用户空间也可以通过使用 MRS 指令读取 CPU ID 寄存ID_AA64PFR0_EL1，并检SVE 字段的值是否非零，来检测对 SVE 指令执行的支持。[^3^]

  这并不能保证后续各节描述的系统接口的存在：需要验证这些接口是否存在的软件必须改而检HWCAP_SVE

- 在支SVE2 扩展的硬件上，HWCAP2_SVE2 也会AT_HWCAP2 辅助向量条目中报告。此外，SVE2 的可选扩展可通过以下标志的存在来报告

	HWCAP2_SVE2
	HWCAP2_SVEAES
	HWCAP2_SVEPMULL
	HWCAP2_SVEBITPERM
	HWCAP2_SVESHA3
	HWCAP2_SVESM4
	HWCAP2_SVE2P1

  随着 SVE 架构的演进，此列表可能会随时间扩展

  这些扩展也通过 CPU ID 寄存ID_AA64ZFR0_EL1 报告，用户空间可使用 MRS 指令读取该寄存器。详elf_hwcaps.txt cpu-feature-registers.txt

- 在支SME 扩展的硬件上，HWCAP2_SME 也会AT_HWCAP2 辅助向量条目中报告。除其他特性外，SME 引入streaming 模式，该模式使用独立SME 向量长度和相同的 Z/V 寄存器，提供 SVE 特性集的一个子集。更多细节参sme.rst

- 调试器应将自身限制在通过 NT_ARM_SVE regset 与目标交互。检测对regset 支持的建议方式是：首先连接到一个目标进程，然后尝试 ptrace(PTRACE_GETREGSET, pid, NT_ARM_SVE, &iov)。注意，SME 存在且正在使streaming SVE 模式时，FPSIMD 寄存器子集将通过 NT_ARM_SVE 读取，而对 NT_ARM_SVE 的写入将使目标退streaming 模式

- 每当 SVE 可扩展寄存器值（Zn、Pn、FFR）在用户空间与内核之间于内存中交换时，寄存器值以与字节序无关的布局编码到内存中，其中位 [(8 ** i + 7) : (8 ** i)] 编码在内存表示起始位置的i 个字节偏移处。例如，这会影响信号帧（struct sve_context）和 ptrace 接口（struct user_sve_header）及其关联数据

  注意，在大端系统上，这导致与 FPSIMD V 寄存器不同的字节顺序，后者作为单个主机字节序128 位值存储，寄存器的[(127 - 8 ** i) : (120 - 8 ** i)] 编码在第 i 个字节偏移处。（struct fpsimd_context、struct user_fpsimd_state


### 2. 向量长度术语


一SVE 向量（Z）寄存器的大小被称为“向量长度”

为避免关于用于表示向量长度的单位产生混淆，内核采用以下约定：

- 向量长度（VL Z 寄存器的大小，以字节

- 向量四字（VQ Z 寄存器的大小，以 128 位为单位

（即 VL = 16 * VQ。）

在底层粒度很重要的场合（例如数据结构定义中）使用 VQ 约定。在大多数其他情况下，使VL 约定。这SVE 指令集架构中“VL”伪寄存器的含义一致


### 3. 系统调用行为


- 在系统调用时，V0..V31 被保留（与无 SVE 时相同）。因此，Z0..Z31 的位 [127:0] 被保留。Z0..Z31 的所有其他位，以P0..P15 FFR 的全部，在系统调用返回时变为零

- SVE 寄存器不用于向任何系统调用传递参数或接收其结果

- 线程的所有其SVE 状态，包括当前配置的向量长度、PR_SVE_VL_INHERIT 标志的状态，以及延迟向量长度（若有），在所有系统调用之间均被保留，但第 6 节对 execve() 描述的特定例外除外

  特别地，fork() clone() 返回时，父进程与新创建的子进程或线程共享完全相同SVE 配置，与调用前父进程的配置一致

### 4. 信号处理


- 一个新的信号帧记录 sve_context 在信号递送时编码 SVE 寄存器。[^1^]

- 该记录是fpsimd_context 的补充。FPSR FPCR 寄存器仅存在fpsimd_context 中。为方便起见，V0..V31 的内容在 sve_context fpsimd_context 之间重复

- 该记录包含一个标志字段，其中有标SVE_SIG_FLAG_SM，若被置位则指示该线程处streaming 模式，且向量长度和寄存器数据（若存在）描述的streaming SVE 数据和向量长度

- 用于 SVE 的信号帧记录始终包含基本元数据，特别是线程的向量长度（位sve_context.vl）

- SVE 寄存器可能会也可能不会包含在记录中，取决于这些寄存器对该线程是否为活跃的（live）。寄存器存在当且仅当

  sve_context.head.size >= SVE_SIG_CONTEXT_SIZE(sve_vq_from_vl(sve_context.vl))銆。

- 若寄存器存在，记录的其余部分具有依赖vl 的大小和布局。定义了SVE_SIG_* [^1^] 以方便访问各成员

- 每个可扩展寄存器（Zn、Pn、FFR）以与字节序无关的布局存储，位 [(8 ** i + 7) : (8 ** i)] 存储在寄存器内存表示起始位置的第 i 个字节偏移处

- SVE 上下文过大以至于无法放入 sigcontext.__reserved[]，则在栈上分配额外的空间，并__reserved[] 中写入一extra_context 记录引用该空间。随sve_context 被写入该额外空间。关于此机制的更多细节参[^1^]


### 5. 信号返回


从信号处理函数返回时

- 若信号帧中没sve_context 记录，或者该记录存在但不包含上一节所述的寄存器数据，SVE 寄存位变为非活跃（non-live）并取未指定的值

- sve_context 存在于信号帧中且包含完整的寄存器数据，则 SVE 寄存器变为活跃并被指定数据填充。然而，出于向后兼容的原因，Z0..Z31 的位 [127:0] 始终fpsimd_context.vregs[] 的对应成员恢复，而非sve_context 恢复。其余位sve_context 恢复

- 无论 sve_context 是否存在，信号帧中包fpsimd_context 仍是强制要求

- 向量长度不能通过信号返回改变。若信号帧中sve_context.vl 与当前向量长度不匹配，则该信号返回尝试被视为非法，导致强制的 SIGSEGV

- 允许通过设置或清SVE_SIG_FLAG_SM 标志来进入或离开 streaming 模式，但应用程序应注意在这样做时，sve_context.vl 以及任何寄存器数据应适合新模式的向量长度


### 6. prctl 扩展


新增了一prctl() 调用，以允许程序管理 SVE 向量长度

prctl(PR_SVE_SET_VL, unsigned long arg)

    设置调用线程的向量长度及相关标志，其arg == vl | flags。调用进程的其他线程不受影响

    vl 是期望的向量长度，其sve_vl_valid(vl) 必须为真

    flags:

	PR_SVE_VL_INHERIT

	    execve() 继承当前向量长度。否则，向量长度execve() 时重置为系统默认值。（见第 9 节。）

	PR_SVE_SET_VL_ONEXEC

	    将请求的向量长度改变延迟到该线程执行的下一execve()

	    其效果等同于在该线程下一execve()（若有）之后立即隐式执行以下调用

		prctl(PR_SVE_SET_VL, arg & ~PR_SVE_SET_VL_ONEXEC)

	    这允许以不同的向量长度启动一个新程序，同时避免对调用方产生运行时副作用

	    若无 PR_SVE_SET_VL_ONEXEC，所请求的改变立即生效

    返回值：成功时为非负值，出错时为负值：
	EINVAL：不支持 SVE、请求了无效的向量长度，或无效的标志

    成功时：

    - 调用线程的向量长度，或该线程将在下一execve() 时应用的延迟向量长度（取决于 arg 中是否存PR_SVE_SET_VL_ONEXEC），被设置为系统支持的小于或等于 vl 的最大值。若 vl == SVE_VL_MAX，所设置的值将是系统支持的最大值

    - 调用线程中任何先前未决的延迟向量长度改变被取消

    - 返回的值描述了结果配置，其编码方式PR_SVE_GET_VL 相同。若该值中 PR_SVE_SET_VL_ONEXEC 不存在，则其中报告的向量长度是该线程的新的当前向量长度；否则，报告的向量长度是调用线程将在下一execve() 时应用的延迟向量长度

    - 改变向量长度会导P0..P15、FFR 以及 Z0..Z31 中除 Z0 [127:0] .. Z31 [127:0] 之外的所有位变为未指定。为此目的，vl 等于线程当前向量长度调用 PR_SVE_SET_VL，或PR_SVE_SET_VL_ONEXEC 标志调用 PR_SVE_SET_VL，均不构成对向量长度的改变


prctl(PR_SVE_GET_VL)

    获取调用线程的向量长度

    以下标志可被 OR 进结果中

	PR_SVE_VL_INHERIT

	    向量长度将在 execve() 之间被继承

    无法判定是否存在未决的延迟向量长度改变（在典型使用中，这一般只会出现在 fork() vfork() 与相应的 execve() 之间）

    要从结果中提取向量长度，将其PR_SVE_VL_LEN_MASK 做按位与

    返回值：成功时为非负值，出错时为负值：
	EINVAL：不支持 SVE

### 7. ptrace 扩展


- 定义了新regset NT_ARM_SVE NT_ARM_SSVE，用PTRACE_GETREGSET PTRACE_SETREGSET。NT_ARM_SSVE 描述 streaming 模式SVE 寄存器，NT_ARM_SVE 描述streaming 模式SVE 寄存器

  在此描述中，当目标处于适当streaming 或非 streaming 模式，并且正在使用超出与 FPSIMD Vn 寄存器共享的子集之外的数据时，称该寄存器集为“live”（活跃）

  定义参见 [^2^]

regset 数据struct user_sve_header 开头，包含

    size

	完整 regset 的大小，以字节计
	这取决于 vl，未来可能还取决于其他因素

	若对 PTRACE_GETREGSET 的调用请求的数据少于 size 的值，调用方可分配更大的缓冲区并重试，以读取完整的 regset

    max_size

	regset 对目标线程能够增长到的最大大小（字节）。即使目标线程改变其向量长度等，regset 也不会增长到超过此值

    vl

	目标线程当前的向量长度，以字节计

    max_vl

	目标线程可能的最大向量长度

    flags

	至多以下之一

	    SVE_PT_REGS_FPSIMD

		SVE 寄存器非活跃（GETREGSET），或将被设为非活跃（SETREGSET）

		payload 的类型为 struct user_fpsimd_state，其含义NT_PRFPREG 相同，从 user_sve_header 起始位置偏移 SVE_PT_FPSIMD_OFFSET 处开始

		未来可能会追加额外数据：payload 的大小应使用 SVE_PT_FPSIMD_SIZE(vq, flags) 获取

		vq 应使sve_vq_from_vl(vl) 获取

		鎴。

	    SVE_PT_REGS_SVE

		SVE 寄存器为活跃（GETREGSET），或将被设为活跃（SETREGSET）

		payload 包含 SVE 寄存器数据，user_sve_header 起始位置偏移 SVE_PT_SVE_OFFSET 处开始，大小SVE_PT_SVE_SIZE(vq, flags)

	... 与以下零个或多个标志OR 运算，这些标志的含义和行为与对应PR_SET_VL_* 标志相同

	    SVE_PT_VL_INHERIT

	    SVE_PT_VL_ONEXEC（仅 SETREGSET）

	若既未提FPSIMD 也未提供 SVE 标志，则没有可用的寄存器 payload，这仅当实现SME 时才可能

- 改变向量长度或标志的效果等同于为 PR_SVE_SET_VL 所记录的效果

  若调用方需要知SETREGSET 实际设置的是哪个 VL，它必须再做一GETREGSET 调用，除非事先已知所请求VL 受支持

- SVE_PT_REGS_SVE 情况下，payload 的大小和布局取决于头部字段。提供了 SVE_PT_SVE_*() 宏以方便访问各成员

- 在两种情况下，对SETREGSET，允许省payload，此时仅改变向量长度和标志（连同这些改变带来的任何后果）

- 在支SME 的系统中，当处于 streaming 模式时，NT_REG_SVE GETREGSET 将只返回 user_sve_header 而无寄存器数据；类似地，当不处于 streaming 模式时，NT_REG_SSVE GETREGSET 将不返回任何寄存器数据

- NT_ARM_SSVE GETREGSET 永远不会返回 SVE_PT_REGS_FPSIMD

- 对于 SETREGSET，若提供SVE_PT_REGS_SVE payload 但所请求VL 不受支持，效果将如同省略payload，只是会报告一EIO 错误。不会尝试将 payload 数据转换为实际设置的向量长度所对应的正确布局。线程的 FPSIMD 状态被保留，但 SVE 寄存器的其余位变为未指定。由调用方负责为实际 VL 转换 payload 布局并重试

- 在实现了 SME 的情况下，当处于 streaming 模式时，无法通过 GETREGSET 获取普SVE 的寄存器状态，也无法在正常模式下获streaming 模式的寄存器状态，无论硬件在两种模式之间共享数据的实现定义行为如何

- 任何NT_ARM_SVE SETREGSET 若目标处streaming 模式都将退streaming 模式；任何对 NT_ARM_SSVE SETREGSET 若目标不处于 streaming 模式都将进入 streaming 模式

- 在不支持 SVE 的系统上，允许使SETREGSET 通过 NT_ARM_SVE 写入 SVE_PT_REGS_FPSIMD 格式的数据，此时向量长度应指定为 0。这允许在具SME 但不具有 SVE 的系统上禁用 streaming 模式

- 若任何寄存器数据SVE_PT_VL_ONEXEC 一起提供，则寄存器数据将以当前向量长度解释，而非以配置为exec 时使用的向量长度解释

- 写入部分、不完整payload 的效果是未指定的


### 8. ELF coredump 扩展

- NT_ARM_SVE NT_ARM_SSVE notes 将被添加到被转储进程每个线程的每coredump 中。其内容等价于：若在生成 coredump 时为每个线程执行相应类型PTRACE_GETREGSET 时本应读取的数据

### 9. 系统运行时配


- 为缓解信号帧扩展带来ABI 影响，提供了一种策略机制，供管理员、发行版维护者和开发者设置用户空间进程的默认向量长度

/proc/sys/abi/sve_default_vector_length

    将整数的文本表示写入此文件，会将系统默认向量长度设置为指定值，并依据与通过 PR_SVE_SET_VL 设置向量长度相同的规则四舍五入为一个受支持的值

    可通过重新打开该文件并读取其内容来确定结果

    在启动时，默认向量长度初始设64 或最大支持向量长度中较小的一个。这决定init 进程（PID 1）的初始向量长度

    读取此文件返回当前的系统默认向量长度

- 在每execve() 调用时，新进程的新向量长度被设置为系统默认向量长度，除非

    - 为调用线程设置了 PR_SVE_VL_INHERIT（或等价SVE_PT_VL_INHERIT），

    - 存在通过 PR_SVE_SET_VL_ONEXEC 标志（或 SVE_PT_VL_ONEXEC）建立的延迟向量长度改变

- 修改系统默认向量长度不会影响任何不进execve() 调用的现有进程或线程的向量长度

### 10. Perf 扩展


- arm64 特定DWARF 标准 [^5^] 在索46 处增加了 VG（Vector Granule，向量粒度）寄存器。当变长 SVE 寄存器被压入栈时，该寄存器用DWARF 展开

- 其值等效于当前 SVE 向量长度（VL，以位计）除64

- 若设置了 PERF_SAMPLE_REGS_USER sample_regs_user 掩码的第 46 位被置位，则该值会包含Perf 采样regs[^46^] 字段中

- 该值为采样时刻的当前值，并可能随时间改变

- 若系统不支持 SVE，却以这些设置调perf_event_open，则该事件将无法打开

## Appendix A. SVE 编程模型（资料性）


本节提供SVE 对与本文档相关的 ARMv8-A 编程模型所做增补的一份最小化描述

注意：本节仅用于提供信息，无意完整，也不打算替代任何架构规范

### A.1. 瀵勫瓨鍣。


A64 状态下，SVE 新增了以下内容：

- 32 8VL 位向量寄存器 Z0..Z31
  对于每个 Zn，Zn 的位 [127:0] ARMv8-A 向量寄存Vn 的别名

  使用 Vn 寄存器名进行的寄存器写操作会将对Zn 除位 [127:0] 之外的所有位清零

- 16 VL 位谓词寄存器 P0..P15

- 1 VL 位专用谓词寄存器 FFR（“first-fault register”，首错寄存器）

- 一个决定每个向量寄存器大小VL“伪寄存器

  SVE 指令集架构没有提供直接写VL 的方式。相反，它只能由 EL1 及更高特权级通过写入适当的系统寄存器来修改

- VL 的值可在运行时EL1 及更高特权级配置
  16 <= VL <= VLmax，其VL 必须16 的倍数

- 最大向量长度由硬件决定
  16 <= VLmax <= 256銆。

  （SVE 架构规定256，但允许未来的架构修订提高此上限。）

- FPSR FPCR ARMv8-A 保留下来，并SVE 浮点操作交互，方式类似于它们ARMv8
```
         8VL-1                       128               0  bit index
        +----          ////            -----------------+
     Z0 |                               :       V0      |
      :                                          :
     Z7 |                               :       V7      |
     Z8 |                               :     * V8      |
      :                                       :  :
    Z15 |                               :     *V15      |
    Z16 |                               :      V16      |
      :                                          :
    Z31 |                               :      V31      |
        +----          ////            -----------------+
                                                 31    0
         VL-1                  0                +-------+
        +----       ////      --+          FPSR |       |
     P0 |                       |               +-------+
      : |                       |         *FPCR |       |
    P15 |                       |               +-------+
        +----       ////      --+
    FFR |                       |               +-----+
        +----       ////      --+            VL |     |
                                                +-----+
```
(*) callee-save锛。
    这仅适用Z-/V-寄存器的[63:0]
    FPCR 包含 callee-save caller-save 位。详[^4^]


### A.2. 过程调用标准

ARMv8-A 基础过程调用标准针对额外SVE 寄存器状态扩展如下：

- 所有不FP/SIMD 共享SVE 寄存器位均为 caller-save（调用方保存）

- Z8 [63:0] .. Z15 [63:0] callee-save（被调用方保存）

  这源于这些位映射V8..V15 的方式，V8..V15 在基础过程调用标准中是 caller-save

## Appendix B. ARMv8-A FP/SIMD 编程模型


注意：本节仅用于提供信息，无意完整，也不打算替代任何架构规范

更多信息参见 [^4^]

ARMv8-A 定义了以下浮/ SIMD 寄存器状态：

- 32 128 位向量寄存器 V0..V31
- 2 32 位状控制寄存FPSR、FPCR

```
         127           0  bit index
        +---------------+
     V0 |               |
      : :               :
     V7 |               |
   * V8 |               |
   :  : :               :
   *V15 |               |
    V16 |               |
      : :               :
    V31 |               |
        +---------------+

                 31    0
                +-------+
           FPSR |       |
                +-------+
          *FPCR |       |
                +-------+
```
(*) callee-save锛。
    这仅适用V-寄存器的[63:0]
    FPCR 包含 callee-save caller-save 位的混合


## References


[^1^] arch/arm64/include/uapi/asm/sigcontext.h
    AArch64 Linux signal ABI definitions

[^2^] arch/arm64/include/uapi/asm/ptrace.h
    AArch64 Linux ptrace ABI definitions

[^3^] Documentation/arch/arm64/cpu-feature-registers.rst

[^4^] ARM IHI0055C
    http://infocenter.arm.com/help/topic/com.arm.doc.ihi0055c/IHI0055C_beta_aapcs64.pdf
    http://infocenter.arm.com/help/topic/com.arm.doc.subset.swdev.abi/index.html
    Procedure Call Standard for the ARM 64-bit Architecture (AArch64)

[^5^] https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst

