
## RISC-V Linux 的向量（Vector）扩展支持


本文档简要概述了 Linux 提供给用户空间、用于支持 RISC-V 向量扩展使用的接口。

### 1. prctl() 接口


新增了两个 prctl() 调用，用于让程序管理在用户空间使用 Vector 的启用状态。这些接口的预期使用准则是为 init 系统提供一种方式，用于修改其域下运行的进程对 V 的可用性。不建议在库例程中调用这些接口，因为库不应覆盖由父进程配置的策略。此外，用户必须注意这些接口不可移植到非 Linux 以及非 RISC-V 环境，因此不鼓励在可移植代码中使用。要获取 ELF 程序中 V 的可用性，请读取辅助向量中 `ELF_HWCAP` 的 `COMPAT_HWCAP_ISA_V` 位。

- prctl(PR_RISCV_V_SET_CONTROL, unsigned long arg)

    设置调用线程的 Vector 启用状态，其中控制参数由两个 2 位的启用状态和一个用于继承模式的位组成。调用进程的其他线程不受影响。

    启用状态是一个三态值，各占用控制参数中的 2 位空间：

    - `PR_RISCV_V_VSTATE_CTRL_DEFAULT`：在 execve() 时使用系统范围的默认启用状态。系统范围的默认设置可以通过 sysctl 接口控制（见下文 sysctl 小节）。

    - `PR_RISCV_V_VSTATE_CTRL_ON`：允许该线程运行 Vector。

    - `PR_RISCV_V_VSTATE_CTRL_OFF`：禁止 Vector。在此情况下执行 Vector 指令会触发陷阱并导致线程终止。

    arg：控制参数是一个由 3 部分组成的 5 位值，分别通过 3 个掩码访问。

    这 3 个掩码 PR_RISCV_V_VSTATE_CTRL_CUR_MASK、PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 和 PR_RISCV_V_VSTATE_CTRL_INHERIT 分别表示 bit[1:0]、bit[3:2] 和 bit[4]。bit[1:0] 对应调用线程的启用状态，bit[3:2] 的设置发生在下一次 execve() 时。bit[4] 定义 bit[3:2] 中设置的继承模式。

        - `PR_RISCV_V_VSTATE_CTRL_CUR_MASK`：bit[1:0]：对应调用线程的 Vector 启用状态。一旦启用，调用线程无法关闭 Vector。如果该掩码中的值为 PR_RISCV_V_VSTATE_CTRL_OFF，但当前启用状态不是 off，则 prctl() 调用将以 EPERM 失败。在此处设置 PR_RISCV_V_VSTATE_CTRL_DEFAULT 没有效果，只是将原始启用状态设回。

        - `PR_RISCV_V_VSTATE_CTRL_NEXT_MASK`：bit[3:2]：对应调用线程在下一次 execve() 系统调用时的 Vector 启用设置。如果在此掩码中使用 PR_RISCV_V_VSTATE_CTRL_DEFAULT，则启用状态将在 execve() 发生时由系统范围的启用状态决定。

        - `PR_RISCV_V_VSTATE_CTRL_INHERIT`：bit[4]：PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 中设置的继承模式。如果设置了该位，则后续的 execve() 不会清除 PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 和 PR_RISCV_V_VSTATE_CTRL_INHERIT 中的设置。该设置跨系统范围默认值的更改而持续存在。

    返回值：
        - 成功时返回 0；
        - EINVAL：不支持 Vector，或当前/下一个掩码的启用状态无效；
        - EPERM：在 PR_RISCV_V_VSTATE_CTRL_CUR_MASK 中关闭 Vector，而调用线程的 Vector 已启用。

    成功时：
        - 对 PR_RISCV_V_VSTATE_CTRL_CUR_MASK 的有效设置会立即生效。PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 中指定的启用状态发生在下一次 execve() 调用时，或者如果设置了 PR_RISCV_V_VSTATE_CTRL_INHERIT 位，则发生在所有后续的 execve() 调用时。
        - 每次成功的调用都会覆盖调用线程之前的一次设置。

- prctl(PR_RISCV_V_GET_CONTROL)

    获取调用线程相同的 Vector 启用状态。下一次 execve() 调用的设置和继承位都会被 OR 在一起。

    注意，ELF 程序能够通过读取辅助向量中 `ELF_HWCAP` 的 `COMPAT_HWCAP_ISA_V` 位来获取自身 V 的可用性。

    返回值：
        - 成功时返回非负值；
        - EINVAL：不支持 Vector。

### 2. 系统运行时配置（sysctl）


为了缓解信号栈扩展对 ABI 的影响，提供了一个策略机制，供管理员、发行版维护者和开发者以 sysctl 旋钮的形式控制用户空间进程默认的 Vector 启用状态：

- /proc/sys/abi/riscv_v_default_allow

    向该文件写入 0 或 1 的文本表示，可设置新启动的用户空间程序的默认系统启用状态。有效值为：

    - 0：默认不允许新进程执行 Vector 代码。
    - 1：默认允许新进程执行 Vector 代码。

    读取该文件会返回当前的系统默认启用状态。

    在每次 execve() 调用时，新进程的启用状态被设为系统默认值，除非：

      - 调用进程设置了 PR_RISCV_V_VSTATE_CTRL_INHERIT，且 PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 中的设置不是 PR_RISCV_V_VSTATE_CTRL_DEFAULT。或者，

      - PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 中的设置不是 PR_RISCV_V_VSTATE_CTRL_DEFAULT。

    修改系统默认启用状态不会影响任何未发起 execve() 调用的现有进程或线程的启用状态。

### 3. 系统调用间的向量寄存器状态


正如 V 扩展的 1.0 版本 [^1^] 所指出的，向量寄存器会被系统调用破坏。

1: https://github.com/riscv/riscv-v-spec/blob/master/calling-convention.adoc
