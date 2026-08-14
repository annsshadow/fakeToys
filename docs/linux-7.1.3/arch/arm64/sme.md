## Scalable Matrix Extension support for AArch64 Linux

AArch64 Linux 对可伸缩矩阵扩展（SME）的支持


This document outlines briefly the interface provided to userspace by Linux in
order to support use of the ARM Scalable Matrix Extension (SME).

本文档简要概述了 Linux 为支持使用 ARM 可伸缩矩阵扩展（SME）而提供给用户空间的接口。

This is an outline of the most important features and issues only and not
intended to be exhaustive.  It should be read in conjunction with the SVE
documentation in sve.rst which provides details on the Streaming SVE mode
included in SME.

本文档仅概述最重要的特性和问题，并非面面俱到。它应与 sve.rst 中的 SVE 文档一并阅读，
后者提供了关于 SME 中所包含流式 SVE（Streaming SVE）模式的细节。

This document does not aim to describe the SME architecture or programmer's
model.  To aid understanding, a minimal description of relevant programmer's
model features for SME is included in Appendix A.

本文档无意描述 SME 架构或编程模型。为便于理解，附录 A 中包含了 SME 相关编程模型特性
的简要说明。


### 1.  General

### 1. 概述


- PSTATE.SM, PSTATE.ZA, the streaming mode vector length, the ZA and (when
  present) ZTn register state and TPIDR2_EL0 are tracked per thread.

- PSTATE.SM、PSTATE.ZA、流式模式向量长度、ZA 以及（在存在时）ZTn 寄存器状态和
  TPIDR2_EL0 均按线程进行跟踪。

- The presence of SME is reported to userspace via HWCAP2_SME in the aux vector
  AT_HWCAP2 entry.  Presence of this flag implies the presence of the SME
  instructions and registers, and the Linux-specific system interfaces
  described in this document.  SME is reported in /proc/cpuinfo as "sme".

- SME 的存在通过辅助向量 AT_HWCAP2 条目中的 HWCAP2_SME 报告给用户空间。该标志的
  存在意味着 SME 指令和寄存器以及本文档所描述的 Linux 特定系统接口的存在。SME 在
  /proc/cpuinfo 中以 "sme" 形式报告。

- The presence of SME2 is reported to userspace via HWCAP2_SME2 in the
  aux vector AT_HWCAP2 entry.  Presence of this flag implies the presence of
  the SME2 instructions and ZT0, and the Linux-specific system interfaces
  described in this document.  SME2 is reported in /proc/cpuinfo as "sme2".

- SME2 的存在通过辅助向量 AT_HWCAP2 条目中的 HWCAP2_SME2 报告给用户空间。该标志的
  存在意味着 SME2 指令和 ZT0 以及本文档所描述的 Linux 特定系统接口的存在。SME2 在
  /proc/cpuinfo 中以 "sme2" 形式报告。

- Support for the execution of SME instructions in userspace can also be
  detected by reading the CPU ID register ID_AA64PFR1_EL1 using an MRS
  instruction, and checking that the value of the SME field is nonzero. [^3^]

- 用户空间也可通过 MRS 指令读取 CPU ID 寄存器 ID_AA64PFR1_EL1，并检查 SME 字段的
  值是否非零，来检测对用户空间中执行 SME 指令的支持。[^3^]

  It does not guarantee the presence of the system interfaces described in the
  following sections: software that needs to verify that those interfaces are
  present must check for HWCAP2_SME instead.

  这并不保证以下小节所描述的系统接口的存在：需要确认这些接口存在的软件必须转而
  检查 HWCAP2_SME。

- There are a number of optional SME features, presence of these is reported
  through AT_HWCAP2 through:

- 存在若干可选的 SME 特性，它们的存在通过 AT_HWCAP2 报告如下：

	HWCAP2_SME_I16I64
	HWCAP2_SME_F64F64
	HWCAP2_SME_I8I32
	HWCAP2_SME_F16F32
	HWCAP2_SME_B16F32
	HWCAP2_SME_F32F32
	HWCAP2_SME_FA64
        HWCAP2_SME2

  This list may be extended over time as the SME architecture evolves.

  此列表可能随着 SME 架构的演进而扩展。

  These extensions are also reported via the CPU ID register ID_AA64SMFR0_EL1,
  which userspace can read using an MRS instruction.  See elf_hwcaps.txt and
  cpu-feature-registers.txt for details.

  这些扩展也通过 CPU ID 寄存器 ID_AA64SMFR0_EL1 报告，用户空间可使用 MRS 指令读取
  该寄存器。详见 elf_hwcaps.txt 和 cpu-feature-registers.txt。

- Debuggers should restrict themselves to interacting with the target via the
  NT_ARM_SVE, NT_ARM_SSVE, NT_ARM_ZA and NT_ARM_ZT regsets.  The recommended
  way of detecting support for these regsets is to connect to a target process
  first and then attempt a

- 调试器应仅限于通过 NT_ARM_SVE、NT_ARM_SSVE、NT_ARM_ZA 和 NT_ARM_ZT regset 与目标
  交互。检测对这些 regset 支持情况的推荐方式是先连接到一个目标进程，然后尝试一个

	ptrace(PTRACE_GETREGSET, pid, NT_ARM_<regset>, &iov).

- Whenever ZA register values are exchanged in memory between userspace and
  the kernel, the register value is encoded in memory as a series of horizontal
  vectors from 0 to VL/8-1 stored in the same endianness invariant format as is
  used for SVE vectors.

- 每当 ZA 寄存器值在用户空间与内核之间通过内存交换时，寄存器值以一系列水平向量的
  形式编码在内存中，从 0 到 VL/8-1，采用与 SVE 向量相同的字节序无关格式存储。

- On thread creation PSTATE.ZA and TPIDR2_EL0 are preserved unless CLONE_VM
  is specified, in which case PSTATE.ZA is set to 0 and TPIDR2_EL0 is set to 0.

- 在线程创建时，PSTATE.ZA 和 TPIDR2_EL0 会被保留，除非指定了 CLONE_VM，在那种情况下
  PSTATE.ZA 被设为 0，TPIDR2_EL0 被设为 0。

### 2.  Vector lengths

### 2. 向量长度


SME defines a second vector length similar to the SVE vector length which
controls the size of the streaming mode SVE vectors and the ZA matrix array.
The ZA matrix is square with each side having as many bytes as a streaming
mode SVE vector.

SME 定义了第二个向量长度，类似于 SVE 向量长度，它控制流式模式 SVE 向量和 ZA 矩阵
数组的大小。ZA 矩阵是方形的，每边的长度等于一个流式模式 SVE 向量的字节数。


### 3.  System call behaviour

### 3. 系统调用行为


- On syscall PSTATE.ZA is preserved, if PSTATE.ZA==1 then the contents of the
  ZA matrix and ZTn (if present) are preserved.

- 在系统调用时 PSTATE.ZA 被保留，如果 PSTATE.ZA==1，则 ZA 矩阵和 ZTn（若存在）的
  内容被保留。

- On syscall PSTATE.SM will be cleared and the SVE registers will be handled
  as per the standard SVE ABI.

- 在系统调用时 PSTATE.SM 会被清除，SVE 寄存器将按标准 SVE ABI 处理。

- None of the SVE registers, ZA or ZTn are used to pass arguments to
  or receive results from any syscall.

- SVE 寄存器、ZA 或 ZTn 都不用于向任何系统调用传递参数或从任何系统调用接收结果。

- On process creation (eg, clone()) the newly created process will have
  PSTATE.SM cleared.

- 在进程创建时（例如 clone()），新创建的进程其 PSTATE.SM 会被清除。

- All other SME state of a thread, including the currently configured vector
  length, the state of the PR_SME_VL_INHERIT flag, and the deferred vector
  length (if any), is preserved across all syscalls, subject to the specific
  exceptions for execve() described in section 6.

- 线程的所有其他 SME 状态，包括当前配置的向量长度、PR_SME_VL_INHERIT 标志的状态，
  以及延迟向量长度（若有），在所有系统调用之间都会被保留，但须遵循第 6 节中针对
  execve() 描述的特定例外。


### 4.  Signal handling

### 4. 信号处理


- Signal handlers are invoked with PSTATE.SM=0, PSTATE.ZA=0, and TPIDR2_EL0=0.

- 信号处理函数被调用时 PSTATE.SM=0、PSTATE.ZA=0，且 TPIDR2_EL0=0。

- A new signal frame record TPIDR2_MAGIC is added formatted as a struct
  tpidr2_context to allow access to TPIDR2_EL0 from signal handlers.

- 新增了一个信号帧记录 TPIDR2_MAGIC，其格式为 struct tpidr2_context，以便从信号处理
  函数访问 TPIDR2_EL0。

- A new signal frame record za_context encodes the ZA register contents on
  signal delivery. [^1^]

- 一个新的信号帧记录 za_context 在信号递交时编码 ZA 寄存器内容。[^1^]

- The signal frame record for ZA always contains basic metadata, in particular
  the thread's vector length (in za_context.vl).

- ZA 的信号帧记录始终包含基本元数据，特别是线程的向量长度（在 za_context.vl 中）。

- The ZA matrix may or may not be included in the record, depending on
  the value of PSTATE.ZA.  The registers are present if and only if:
  za_context.head.size >= ZA_SIG_CONTEXT_SIZE(sve_vq_from_vl(za_context.vl))
  in which case PSTATE.ZA == 1.

- ZA 矩阵是否包含在记录中，取决于 PSTATE.ZA 的值。当且仅当
  za_context.head.size >= ZA_SIG_CONTEXT_SIZE(sve_vq_from_vl(za_context.vl))
  时寄存器才存在，此时 PSTATE.ZA == 1。

- If matrix data is present, the remainder of the record has a vl-dependent
  size and layout.  Macros ZA_SIG_* are defined [^1^] to facilitate access to
  them.

- 如果存在矩阵数据，记录的其余部分具有依赖于 vl 的大小和布局。宏 ZA_SIG_* 已被定义
  [^1^] 以便于访问它们。

- The matrix is stored as a series of horizontal vectors in the same format as
  is used for SVE vectors.

- 矩阵以一系列水平向量的形式存储，采用与 SVE 向量相同的格式。

- If the ZA context is too big to fit in sigcontext.__reserved[], then extra
  space is allocated on the stack, an extra_context record is written in
  __reserved[] referencing this space.  za_context is then written in the
  extra space.  Refer to [^1^] for further details about this mechanism.

- 如果 ZA 上下文太大而无法放入 sigcontext.__reserved[]，则在栈上分配额外空间，并在
  __reserved[] 中写入一个 extra_context 记录来引用该空间。随后 za_context 被写入
  该额外空间。有关此机制的更多细节请参阅 [^1^]。

- If ZTn is supported and PSTATE.ZA==1 then a signal frame record for ZTn will
  be generated.

- 如果支持 ZTn 且 PSTATE.ZA==1，则会生成 ZTn 的信号帧记录。

- The signal record for ZTn has magic ZT_MAGIC (0x5a544e01) and consists of a
  standard signal frame header followed by a struct zt_context specifying
  the number of ZTn registers supported by the system, then zt_context.nregs
  blocks of 64 bytes of data per register.

- ZTn 的信号记录具有魔数 ZT_MAGIC（0x5a544e01），由一个标准信号帧头，后跟一个
  struct zt_context（指定系统支持的 ZTn 寄存器数量），然后是每个寄存器 64 字节数据
  的 zt_context.nregs 个块组成。


### 5.  Signal return

### 5. 信号返回


When returning from a signal handler:

从信号处理函数返回时：

- If there is no za_context record in the signal frame, or if the record is
  present but contains no register data as described in the previous section,
  then ZA is disabled.

- 如果信号帧中没有 za_context 记录，或者该记录存在但不包含上一节所述的寄存器数据，
  则 ZA 被禁用。

- If za_context is present in the signal frame and contains matrix data then
  PSTATE.ZA is set to 1 and ZA is populated with the specified data.

- 如果信号帧中存在 za_context 且包含矩阵数据，则 PSTATE.ZA 被设为 1，并且 ZA 被填入
  指定的数据。

- The vector length cannot be changed via signal return.  If za_context.vl in
  the signal frame does not match the current vector length, the signal return
  attempt is treated as illegal, resulting in a forced SIGSEGV.

- 向量长度不能通过信号返回来改变。如果信号帧中的 za_context.vl 与当前向量长度不匹配，
  则信号返回尝试被视为非法，导致强制产生 SIGSEGV。

- If ZTn is not supported or PSTATE.ZA==0 then it is illegal to have a
  signal frame record for ZTn, resulting in a forced SIGSEGV.

- 如果不支持 ZTn 或 PSTATE.ZA==0，则拥有 ZTn 的信号帧记录是非法的，会导致强制产生
  SIGSEGV。


### 6.  prctl extensions

### 6.  prctl 扩展


Some new prctl() calls are added to allow programs to manage the SME vector
length:

新增了一些 prctl() 调用来允许程序管理 SME 向量长度：

prctl(PR_SME_SET_VL, unsigned long arg)

    Sets the vector length of the calling thread and related flags, where
    arg == vl | flags.  Other threads of the calling process are unaffected.

    vl 是调用线程的向量长度及相关标志，其中 arg == vl | flags。调用进程的
    其他线程不受影响。

    vl is the desired vector length, where sve_vl_valid(vl) must be true.

    vl 是期望的向量长度，其中 sve_vl_valid(vl) 必须为真。

    flags:

    PR_SME_VL_INHERIT

        Inherit the current vector length across execve().  Otherwise, the
        vector length is reset to the system default at execve().  (See
        Section 9.)

        在 execve() 期间继承当前向量长度。否则，向量长度在 execve() 时被重置为
        系统默认值。（参见第 9 节。）

    PR_SME_SET_VL_ONEXEC

        Defer the requested vector length change until the next execve()
        performed by this thread.

        将所请求的向量长度变更推迟到本线程执行的下一次 execve()。

        The effect is equivalent to implicit execution of the following
        call immediately after the next execve() (if any) by the thread:

        其效果等同于在本次（若有）execve() 之后由该线程隐式执行以下调用：

        prctl(PR_SME_SET_VL, arg & ~PR_SME_SET_VL_ONEXEC)

        This allows launching of a new program with a different vector
        length, while avoiding runtime side effects in the caller.

        这允许以不同的向量长度启动一个新程序，同时避免对调用者产生运行时副作用。

        Without PR_SME_SET_VL_ONEXEC, the requested change takes effect
        immediately.

        若没有 PR_SME_SET_VL_ONEXEC，所请求的变更会立即生效。


    Return value: a nonnegative on success, or a negative value on error:
        EINVAL: SME not supported, invalid vector length requested, or
            invalid flags.


    返回值：成功时为非负数，出错时为负值：
        EINVAL：不支持 SME、请求了无效的向量长度，或无效的标志。


    On success:

    成功时：

    - Either the calling thread's vector length or the deferred vector length
      to be applied at the next execve() by the thread (dependent on whether
      PR_SME_SET_VL_ONEXEC is present in arg), is set to the largest value
      supported by the system that is less than or equal to vl.  If vl ==
      SVE_VL_MAX, the value set will be the largest value supported by the
      system.

    - 调用线程的向量长度，或将在下一次 execve() 时由该线程应用的延迟向量长度（取决于
      arg 中是否含有 PR_SME_SET_VL_ONEXEC），被设为系统支持的小于或等于 vl 的最大值。
      如果 vl == SVE_VL_MAX，所设的值为系统支持的最大值。

    - Any previously outstanding deferred vector length change in the calling
      thread is cancelled.

    - 调用线程中任何先前未决的延迟向量长度变更被取消。

    - The returned value describes the resulting configuration, encoded as for
      PR_SME_GET_VL.  The vector length reported in this value is the new
      current vector length for this thread if PR_SME_SET_VL_ONEXEC was not
      present in arg; otherwise, the reported vector length is the deferred
      vector length that will be applied at the next execve() by the calling
      thread.

    - 返回值描述了结果配置，按 PR_SME_GET_VL 的方式编码。如果 arg 中不含
      PR_SME_SET_VL_ONEXEC，此值中报告的向量长度是本线程新的当前向量长度；否则，
      报告的向量长度是将在调用线程的下一次 execve() 时应用的延迟向量长度。

    - Changing the vector length causes all of ZA, ZTn, P0..P15, FFR and all
      bits of Z0..Z31 except for Z0 bits [127:0] .. Z31 bits [127:0] to become
      unspecified, including both streaming and non-streaming SVE state.
      Calling PR_SME_SET_VL with vl equal to the thread's current vector
      length, or calling PR_SME_SET_VL with the PR_SME_SET_VL_ONEXEC flag,
      does not constitute a change to the vector length for this purpose.

    - 改变向量长度会使 ZA、ZTn、P0..P15、FFR 以及 Z0..Z31 中除 Z0 比特 [127:0] ..
      Z31 比特 [127:0] 之外的所有比特变为未指定状态，包括流式和非流式 SVE 状态。
      以等于线程当前向量长度的 vl 调用 PR_SME_SET_VL，或以 PR_SME_SET_VL_ONEXEC
      标志调用 PR_SME_SET_VL，在此意义上不构成对向量长度的更改。

    - Changing the vector length causes PSTATE.ZA to be cleared.
      Calling PR_SME_SET_VL with vl equal to the thread's current vector
      length, or calling PR_SME_SET_VL with the PR_SME_SET_VL_ONEXEC flag,
      does not constitute a change to the vector length for this purpose.

    - 改变向量长度会导致 PSTATE.ZA 被清除。以等于线程当前向量长度的 vl 调用
      PR_SME_SET_VL，或以 PR_SME_SET_VL_ONEXEC 标志调用 PR_SME_SET_VL，在此意义上
      不构成对向量长度的更改。


prctl(PR_SME_GET_VL)

    Gets the vector length of the calling thread.

    获取调用线程的向量长度。

    The following flag may be OR-ed into the result:

    以下标志可被 OR 进结果中：

        PR_SME_VL_INHERIT

        Vector length will be inherited across execve().

        向量长度将在 execve() 期间被继承。

    There is no way to determine whether there is an outstanding deferred
    vector length change (which would only normally be the case between a
    fork() or vfork() and the corresponding execve() in typical use).

    无法判断是否存在未决的延迟向量长度变更（通常只会在典型的 fork() 或 vfork() 与
    相应的 execve() 之间出现）。

    To extract the vector length from the result, bitwise and it with
    PR_SME_VL_LEN_MASK.

    要从结果中提取向量长度，对其按位与 PR_SME_VL_LEN_MASK。

    Return value: a nonnegative value on success, or a negative value on error:
        EINVAL: SME not supported.

    返回值：成功时为非负值，出错时为负值：
        EINVAL：不支持 SME。


### 7.  ptrace extensions

### 7.  ptrace 扩展


- A new regset NT_ARM_SSVE is defined for access to streaming mode SVE
  state via PTRACE_GETREGSET and  PTRACE_SETREGSET, this is documented in
  sve.rst.

- 定义了一个新的 regset NT_ARM_SSVE，用于通过 PTRACE_GETREGSET 和 PTRACE_SETREGSET
  访问流式模式 SVE 状态，这在 sve.rst 中有记载。

- A new regset NT_ARM_ZA is defined for ZA state for access to ZA state via
  PTRACE_GETREGSET and PTRACE_SETREGSET.

- 定义了一个新的 regset NT_ARM_ZA，用于通过 PTRACE_GETREGSET 和 PTRACE_SETREGSET
  访问 ZA 状态。

  Refer to [^2^] for definitions.

  定义请参阅 [^2^]。

The regset data starts with struct user_za_header, containing:

regset 数据以 struct user_za_header 开头，其中包含：

    size

        Size of the complete regset, in bytes.
        This depends on vl and possibly on other things in the future.

        regset 的完整大小，以字节为单位。
        这取决于 vl，将来也可能取决于其他因素。

        If a call to PTRACE_GETREGSET requests less data than the value of
        size, the caller can allocate a larger buffer and retry in order to
        read the complete regset.

        如果对 PTRACE_GETREGSET 的调用请求的数据少于 size 的值，调用者可以分配
        更大的缓冲区并重试，以读取完整的 regset。

    max_size

        Maximum size in bytes that the regset can grow to for the target
        thread.  The regset won't grow bigger than this even if the target
        thread changes its vector length etc.

        regset 能为目标线程增长到的最大字节数。即使目标线程改变其向量长度等，
        regset 也不会增长到超过此值。

    vl

        Target thread's current streaming vector length, in bytes.

        目标线程当前的流式向量长度，以字节为单位。

    max_vl

        Maximum possible streaming vector length for the target thread.

        目标线程可能的最大流式向量长度。

    flags

        Zero or more of the following flags, which have the same
        meaning and behaviour as the corresponding PR_SET_VL_* flags:

        以下一个或多个标志，其含义和行为与对应的 PR_SET_VL_* 标志相同：

            SME_PT_VL_INHERIT

            SME_PT_VL_ONEXEC (SETREGSET only).

- The effects of changing the vector length and/or flags are equivalent to
  those documented for PR_SME_SET_VL.

- 改变向量长度和/或标志的效果等同于 PR_SME_SET_VL 中记载的效果。

  The caller must make a further GETREGSET call if it needs to know what VL is
  actually set by SETREGSET, unless is it known in advance that the requested
  VL is supported.

  如果调用者需要知道 SETREGSET 实际设置的 VL，则必须进行进一步的 GETREGSET 调用，
  除非事先已知所请求的 VL 受支持。

- The size and layout of the payload depends on the header fields.  The
  ZA_PT_ZA*() macros are provided to facilitate access to the data.

- 负载的大小和布局取决于头部字段。提供了 ZA_PT_ZA*() 宏以便于访问数据。

- In either case, for SETREGSET it is permissible to omit the payload, in which
  case the vector length and flags are changed and PSTATE.ZA is set to 0
  (along with any consequences of those changes).  If a payload is provided
  then PSTATE.ZA will be set to 1.

- 无论哪种情况，对于 SETREGSET，可以省略负载，此时向量长度和标志会被改变且
  PSTATE.ZA 被设为 0（以及这些变更带来的任何后果）。如果提供了负载，则 PSTATE.ZA
  将被设为 1。

- For SETREGSET, if the requested VL is not supported, the effect will be the
  same as if the payload were omitted, except that an EIO error is reported.
  No attempt is made to translate the payload data to the correct layout
  for the vector length actually set.  It is up to the caller to translate the
  payload layout for the actual VL and retry.

- 对于 SETREGSET，如果所请求的 VL 不受支持，其效果与省略负载相同，只是会报告一个
  EIO 错误。不会尝试将负载数据转换为实际设置的向量长度所对应的正确布局。由调用者
  负责为实际的 VL 转换负载布局并重试。

- The effect of writing a partial, incomplete payload is unspecified.

- 写入不完整的、部分负载的效果是未指定的。

- A new regset NT_ARM_ZT is defined for access to ZTn state via
  PTRACE_GETREGSET and PTRACE_SETREGSET.

- 定义了一个新的 regset NT_ARM_ZT，用于通过 PTRACE_GETREGSET 和 PTRACE_SETREGSET
  访问 ZTn 状态。

- The NT_ARM_ZT regset consists of a single 512 bit register.

- NT_ARM_ZT regset 由一个单独的 512 位寄存器组成。

- When PSTATE.ZA==0 reads of NT_ARM_ZT will report all bits of ZTn as 0.

- 当 PSTATE.ZA==0 时，对 NT_ARM_ZT 的读取会将 ZTn 的所有比特报告为 0。

- Writes to NT_ARM_ZT will set PSTATE.ZA to 1.

- 对 NT_ARM_ZT 的写入会将 PSTATE.ZA 设为 1。

- If any register data is provided along with SME_PT_VL_ONEXEC then the
  registers data will be interpreted with the current vector length, not
  the vector length configured for use on exec.

- 如果提供了任何寄存器数据并同时带有 SME_PT_VL_ONEXEC，则寄存器数据将使用当前向量
  长度来解释，而非为 exec 配置使用的向量长度。


### 8.  ELF coredump extensions

### 8.  ELF coredump 扩展


- NT_ARM_SSVE notes will be added to each coredump for
  each thread of the dumped process.  The contents will be equivalent to the
  data that would have been read if a PTRACE_GETREGSET of the corresponding
  type were executed for each thread when the coredump was generated.

- 将为被 dump 进程的每一个线程，在每个 coredump 中添加 NT_ARM_SSVE 备注。其内容等同于
  在生成 coredump 时，如果对每个线程执行了相应类型的 PTRACE_GETREGSET 所会读到的数据。

- A NT_ARM_ZA note will be added to each coredump for each thread of the
  dumped process.  The contents will be equivalent to the data that would have
  been read if a PTRACE_GETREGSET of NT_ARM_ZA were executed for each thread
  when the coredump was generated.

- 将为被 dump 进程的每一个线程，在每个 coredump 中添加 NT_ARM_ZA 备注。其内容等同于
  在生成 coredump 时，如果对每个线程执行了 NT_ARM_ZA 的 PTRACE_GETREGSET 所会读到的
  数据。

- A NT_ARM_ZT note will be added to each coredump for each thread of the
  dumped process.  The contents will be equivalent to the data that would have
  been read if a PTRACE_GETREGSET of NT_ARM_ZT were executed for each thread
  when the coredump was generated.

- 将为被 dump 进程的每一个线程，在每个 coredump 中添加 NT_ARM_ZT 备注。其内容等同于
  在生成 coredump 时，如果对每个线程执行了 NT_ARM_ZT 的 PTRACE_GETREGSET 所会读到的
  数据。

- The NT_ARM_TLS note will be extended to two registers, the second register
  will contain TPIDR2_EL0 on systems that support SME and will be read as
  zero with writes ignored otherwise.

- NT_ARM_TLS 备注将扩展到两个寄存器，第二个寄存器在支持 SME 的系统上将包含
  TPIDR2_EL0，否则将被读作零且写入被忽略。

### 9.  System runtime configuration

### 9. 系统运行时配置


- To mitigate the ABI impact of expansion of the signal frame, a policy
  mechanism is provided for administrators, distro maintainers and developers
  to set the default vector length for userspace processes:

- 为减轻信号帧扩展带来的 ABI 影响，提供了一种策略机制，供管理员、发行版维护者和
  开发者为用户空间进程设置默认向量长度：

/proc/sys/abi/sme_default_vector_length

    Writing the text representation of an integer to this file sets the system
    default vector length to the specified value rounded to a supported value
    using the same rules as for setting vector length via PR_SME_SET_VL.

    向此文件写入一个整数的文本表示，会将系统默认向量长度设为指定的值（按与通过
    PR_SME_SET_VL 设置向量长度相同的规则取整到受支持的值）。

    The result can be determined by reopening the file and reading its
    contents.

    可通过重新打开文件并读取其内容来确定结果。

    At boot, the default vector length is initially set to 32 or the maximum
    supported vector length, whichever is smaller and supported.  This
    determines the initial vector length of the init process (PID 1).

    在启动时，默认向量长度最初被设为 32 或最大受支持的向量长度，取其中较小且受支持者。
    这决定了 init 进程（PID 1）的初始向量长度。

    Reading this file returns the current system default vector length.

    读取此文件返回当前系统默认向量长度。

- At every execve() call, the new vector length of the new process is set to
  the system default vector length, unless

- 在每次 execve() 调用时，新进程的新向量长度被设为系统默认向量长度，除非

    - PR_SME_VL_INHERIT (or equivalently SME_PT_VL_INHERIT) is set for the
      calling thread, or

    - 为调用线程设置了 PR_SME_VL_INHERIT（或等价的 SME_PT_VL_INHERIT），或

    - a deferred vector length change is pending, established via the
      PR_SME_SET_VL_ONEXEC flag (or SME_PT_VL_ONEXEC).

    - 存在一个通过 PR_SME_SET_VL_ONEXEC 标志（或 SME_PT_VL_ONEXEC）建立的待定
      延迟向量长度变更。

- Modifying the system default vector length does not affect the vector length
  of any existing process or thread that does not make an execve() call.

- 修改系统默认向量长度不会影响任何不进行 execve() 调用的已有进程或线程的向量长度。


## Appendix A.  SME programmer's model (informative)

## 附录 A.  SME 编程模型（仅供参考）


This section provides a minimal description of the additions made by SME to the
ARMv8-A programmer's model that are relevant to this document.

本节简要描述了 SME 对与本文档相关的 ARMv8-A 编程模型所做的补充。

Note: This section is for information only and not intended to be complete or
to replace any architectural specification.

注意：本节仅供参考，无意完整，也不取代任何架构规范。

### A.1.  Registers

### A.1.  寄存器


In A64 state, SME adds the following:

在 A64 状态下，SME 增加了以下内容：

- A new mode, streaming mode, in which a subset of the normal FPSIMD and SVE
  features are available.  When supported EL0 software may enter and leave
  streaming mode at any time.

- 一种新模式，即流式模式（streaming mode），在该模式下可用正常 FPSIMD 和 SVE 特性的
  一个子集。当受支持时，EL0 软件可以随时进入和离开流式模式。

  For best system performance it is strongly encouraged for software to enable
  streaming mode only when it is actively being used.

  为了获得最佳系统性能，强烈建议软件仅在主动使用流式模式时才启用它。

- A new vector length controlling the size of ZA and the Z registers when in
  streaming mode, separately to the vector length used for SVE when not in
  streaming mode.  There is no requirement that either the currently selected
  vector length or the set of vector lengths supported for the two modes in
  a given system have any relationship.  The streaming mode vector length
  is referred to as SVL.

- 一个新的向量长度，用于控制处于流式模式时 ZA 和 Z 寄存器的大小，与不在流式模式时
  用于 SVE 的向量长度相互独立。对于某个给定系统，当前选择的向量长度，或两种模式所
  支持的向量长度集合，都不要求有任何关系。流式模式的向量长度被称为 SVL。

- A new ZA matrix register.  This is a square matrix of SVLxSVL bits.  Most
  operations on ZA require that streaming mode be enabled but ZA can be
  enabled without streaming mode in order to load, save and retain data.

- 一个新的 ZA 矩阵寄存器。这是一个 SVLxSVL 比特的方形矩阵。对 ZA 的大多数操作要求
  启用流式模式，但 ZA 可以在不启用流式模式的情况下被启用，以便加载、保存和保留数据。

  For best system performance it is strongly encouraged for software to enable
  ZA only when it is actively being used.

  为了获得最佳系统性能，强烈建议软件仅在主动使用 ZA 时才启用它。

- A new ZT0 register is introduced when SME2 is present. This is a 512 bit
  register which is accessible when PSTATE.ZA is set, as ZA itself is.

- 当 SME2 存在时引入一个新的 ZT0 寄存器。这是一个 512 位寄存器，在 PSTATE.ZA 被
  设置时可访问，正如 ZA 本身一样。

- Two new 1 bit fields in PSTATE which may be controlled via the SMSTART and
  SMSTOP instructions or by access to the SVCR system register:

- PSTATE 中两个新的 1 比特字段，可通过 SMSTART 和 SMSTOP 指令，或通过对 SVCR 系统
  寄存器的访问来控制：

  - PSTATE.ZA, if this is 1 then the ZA matrix is accessible and has valid
    data while if it is 0 then ZA can not be accessed.  When PSTATE.ZA is
    changed from 0 to 1 all bits in ZA are cleared.

  - PSTATE.ZA，如果为 1，则 ZA 矩阵可访问且含有有效数据；如果为 0，则 ZA 不可访问。
    当 PSTATE.ZA 从 0 变为 1 时，ZA 中的所有比特被清除。

  - PSTATE.SM, if this is 1 then the PE is in streaming mode.  When the value
    of PSTATE.SM is changed then it is implementation defined if the subset
    of the floating point register bits valid in both modes may be retained.
    Any other bits will be cleared.

  - PSTATE.SM，如果为 1，则 PE 处于流式模式。当 PSTATE.SM 的值改变时，两种模式都
    有效的浮点寄存器比特子集是否可保留是由具体实现定义的。任何其他比特都将被清除。


## References

## 参考


[^1^] arch/arm64/include/uapi/asm/sigcontext.h
    AArch64 Linux signal ABI definitions

[^2^] arch/arm64/include/uapi/asm/ptrace.h
    AArch64 Linux ptrace ABI definitions

[^3^] Documentation/arch/arm64/cpu-feature-registers.rst
