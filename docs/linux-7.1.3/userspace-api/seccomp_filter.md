## Seccomp BPF（SECure COMPuting with filters，带过滤器的安全计算

## 简

大量系统调用对每个用户态进程都是开放的，但其中许多在进程的整个生命周期中都不会被使用。随着系统调用的演进与成熟，bug 被发现并被消除。某些用户态应用受益于拥有一组更少可用系统调用的集合。由此得到的集合减小了暴露给应用程序的内核总攻击面。系统调用过滤正是为这类应用程序而设
Seccomp 过滤提供了一种机制，使进程能够为传入的系统调用指定过滤器。该过滤器以 Berkeley Packet Filter（BPF）程序的形式表达，与套接字过滤器类似，区别在于所操作的数据与正在进行的系统调用相关：系统调用号和系统调用参数。这使得能够以富于表达力的方式过滤系统调用，使用一种早已对用户态开放、且数据集直观的过滤程序语言
此外，BPF 使得 seccomp 的使用者不会沦为检查时使用时间（TOCTOU）攻击的受害者，这类攻击在系统调用拦截框架中很常见。BPF 程序不能解引用指针，这就将所有过滤器限制为只能直接对系统调用参数求值
## 瀹冨苟闈炰粈涔。

系统调用过滤并非沙箱。它提供了一种定义清晰的机制，用于最小化暴露的内核攻击面。它是供沙箱开发者使用的工具。除此之外，针对逻辑行为与信息流的策略应当结合其他系统加固技术、以及（可能的话）你所选择LSM 来管理。富于表达力、动态的过滤器沿此路径提供了更多选项（例如避免病态规模，或选择允许 socketcall() 中哪些多路复用系统调用），这些可能被错误地理解为更完整的沙箱解决方案
## 用法


新增了一seccomp 模式，并使用与严seccomp 相同prctl(2) 调用来启用。如果架构具`CONFIG_HAVE_ARCH_SECCOMP_FILTER`，则可按如下方式添加过滤器：

`PR_SET_SECCOMP`	现在接受一个额外的参数，用于通过 BPF 程序指定一个新过滤器。该 BPF 程序将在反映系统调用号、参数及其他元数据的 struct seccomp_data 上执行。随BPF 程序必须返回某个可接受的值，以告知内核应当采取哪个动作
```
		prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, prog);

	The 'prog' argument is a pointer to a struct sock_fprog which
	will contain the filter program.  If the program is invalid, the
	call will return -1 and set errno to ``EINVAL``.

	If ``fork``/``clone`` and ``execve`` are allowed by @prog, any child
	processes will be constrained to the same filters and system
	call ABI as the parent.

	Prior to use, the task must call ``prctl(PR_SET_NO_NEW_PRIVS, 1)`` or
	run with ``CAP_SYS_ADMIN`` privileges in its namespace.  If these are not
	true, ``-EACCES`` will be returned.  This requirement ensures that filter
	programs cannot be applied to child processes with greater privileges
	than the task that installed them.

	Additionally, if ``prctl(2)`` is allowed by the attached filter,
	additional filters may be layered on which will increase evaluation
	time, but allow for further decreasing the attack surface during
	execution of a process.
```

上述调用成功时返0，出错时返回非零值
## 杩斿洖鍊。

seccomp 过滤器可以返回下列任意值。如果存在多个过滤器，对给定系统调用求值得到的返回值将始终采用优先级最高的值。（例如，`SECCOMP_RET_KILL_PROCESS` 总是优先。）

按其优先级顺序如下：

`SECCOMP_RET_KILL_PROCESS`	导致整个进程立即退出，且不执行该系统调用。任务的退出状态（`status & 0x7f`）将`SIGSYS`，而非 `SIGKILL`
`SECCOMP_RET_KILL_THREAD`	导致该任务立即退出，且不执行该系统调用。任务的退出状态（`status & 0x7f`）将`SIGSYS`，而非 `SIGKILL`
`SECCOMP_RET_TRAP`	导致内核向触发该调用的任务发`SIGSYS` 信号，且不执行该系统调用。`siginfo->si_call_addr` 将显示系统调用指令的地址，`siginfo->si_syscall` `siginfo->si_arch` 将指示尝试了哪个系统调用。程序计数器将表现得如同系统调用已经发生（即它不会指向系统调用指令）。返回值寄存器将包含一个与架构相关的值——若恢复执行，请将其设为合理的值。（之所以与架构相关，是因为`-ENOSYS` 替换它可能会覆盖一些有用信息。）

	返回值的 `SECCOMP_RET_DATA` 部分将作`si_errno` 传递
	seccomp 触发`SIGSYS` si_code `SYS_SECCOMP`
`SECCOMP_RET_ERRNO`	导致返回值的16 位作errno 传递给用户态，且不执行该系统调用
`SECCOMP_RET_USER_NOTIF`	导致在用户态通知 fd 上发送一`struct seccomp_notif` 消息（若已附加），否则发`-ENOSYS`。关于如何处理用户通知，参见下文讨论
`SECCOMP_RET_TRACE`	当返回此值时，会导致内核在执行系统调用之前尝试通知一个基`ptrace()` 的跟踪器。若不存在跟踪器，则向用户态返`-ENOSYS`，且不执行该系统调用
	如果跟踪器使`ptrace(PTRACE_SETOPTIONS)` 请求`PTRACE_O_TRACESECCOMP`，它就会收到通知。跟踪器将收`PTRACE_EVENT_SECCOMP` 通知，且 BPF 程序返回值的 `SECCOMP_RET_DATA` 部分可通过 `PTRACE_GETEVENTMSG` 供跟踪器获取
	跟踪器可以通过将系统调用号改为 -1 来跳过该系统调用。或者，跟踪器可以通过将系统调用改为一个有效的系统调用号来改变所请求的系统调用。若跟踪器要求跳过该系统调用，则系统调用将表现得如同返回跟踪器放入返回值寄存器中的值
	在通知跟踪器之后，不会再运seccomp 检查。（这意味着基于 seccomp 的沙箱在允许使用 ptrace 时必须极为谨慎，即便是对其他已沙箱化的进程也是如此；ptrace 跟踪器可利用此机制逃逸。）

`SECCOMP_RET_LOG`	导致系统调用在被记录之后执行。应用开发者应使用它来了解其应用程序需要哪些系统调用，而无需反复经历多次测试与开发周期来构建该列表
	仅当 actions_logged sysctl 字符串中包含 "log" 时，该动作才会被记录
`SECCOMP_RET_ALLOW`	导致系统调用被执行
如果存在多个过滤器，对给定系统调用求值得到的返回值将始终采用优先级最高的值
优先级仅`SECCOMP_RET_ACTION` 掩码决定。当多个过滤器返回相同优先级的值时，只会返回最近安装的过滤器所提供`SECCOMP_RET_DATA`
## 陷阱


使用过程中最需避免的陷阱是：仅依据系统调用号进行过滤而不检查架构值。为什么？在任何支持多种系统调用调用约定的架构上，系统调用号可能随具体调用方式而不同。如果不同调用约定中的编号发生重叠，过滤器中的检查就可能被滥用。务必检arch 值！

## 示例


`samples/seccomp/` 目录中既包含一x86 特定的示例，也包含一个更通用的、用BPF 程序生成的高级宏接口示例
## 用户态通知


`SECCOMP_RET_USER_NOTIF` 返回码使 seccomp 过滤器能够将特定的系统调用传递给用户态处理。这对于容器管理器等应用可能很有用，它们希望拦截特定的系统调用（`mount()`、`finit_module()` 等）并改变其行为
要获取通知 fd，可`seccomp()` 系统调用使用 `SECCOMP_FILTER_FLAG_NEW_LISTENER` 参数
    fd = seccomp(SECCOMP_SET_MODE_FILTER, SECCOMP_FILTER_FLAG_NEW_LISTENER, &prog);

该调用成功时会返回一个针对该过滤器的 listener fd，随后可通过 `SCM_RIGHTS` 或类似机制传递。注意，filter fd 对应于特定的过滤器，而非特定的任务。因此，如果该任务随fork，两个任务的 notifications 都会出现在同一filter fd 上。对 filter fd 的读写也是同步的，因此一filter fd 可以安全地拥有多个读取者
seccomp 通知 fd 的接口由两个结构体组成：

    struct seccomp_notif_sizes {
        __u16 seccomp_notif;
        __u16 seccomp_notif_resp;
        __u16 seccomp_data;
    };

    struct seccomp_notif {
        __u64 id;
        __u32 pid;
        __u32 flags;
        struct seccomp_data data;
    };

    struct seccomp_notif_resp {
        __u64 id;
        __s64 val;
        __s32 error;
        __u32 flags;
    };

`struct seccomp_notif_sizes` 结构体可用于确定 seccomp 通知中所用各种结构体的大小。`struct seccomp_data` 的大小未来可能会改变，因此代码应使用
    struct seccomp_notif_sizes sizes;
    seccomp(SECCOMP_GET_NOTIF_SIZES, 0, &sizes);

来确定要分配的各种结构体的大小。示例参samples/seccomp/user-trap.c
用户可通过seccomp 通知 fd 上调`ioctl(SECCOMP_IOCTL_NOTIF_RECV)`（或 `poll()`）来读取并接收一`struct seccomp_notif`，它包含五个成员：结构体的输入长度、每个过滤器唯一`id`、触发该请求的任务的 `pid`（若该任务处于监听pid 命名空间不可见的 pid ns 中，则可能为 0）。该通知还包含传递给 seccomp `data`，以及一个过滤器标志。在调用 ioctl 之前，应将该结构体清零
然后用户态可基于这些信息做出决定，并通过 `ioctl(SECCOMP_IOCTL_NOTIF_SEND)` 发送一个响应，指示应返回给用户态的内容。`struct seccomp_notif_resp` `id` 成员应与 `struct seccomp_notif` 中的 `id` 相同
用户态还可以通过 `ioctl(SECCOMP_IOCTL_NOTIF_ADDFD)` 向通知进程添加文件描述符。`struct seccomp_notif_addfd` `id` 成员应与 `struct seccomp_notif` 中的 `id` 相同。`newfd_flags` 标志可用于在通知进程的文件描述符上设置诸O_CLOEXEC 之类的标志。如果监管者（supervisor）希望以特定编号注入文件描述符，可以使用 `SECCOMP_ADDFD_FLAG_SETFD` 标志，并`newfd` 成员设为要使用的特定编号。如果该文件描述符已在通知进程中打开，则会被替换。监管者也可以添加一FD，并通过使用 `SECCOMP_ADDFD_FLAG_SEND` 标志原子地作出响应，此时返回值将是被注入的文件描述符编号
通知进程可能被抢占，导致通知被中止。当试图代表通知进程执行耗时较长、且通常可重试的操作（例如挂载文件系统）时，这可能带来问题。作为替代，在过滤器安装时，可以设置 `SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV` 标志。该标志的作用是：当监管者收到用户通知时，通知进程将忽略非致命信号，直到响应被发送。在通知被用户态接收之前发送的信号则照常处理
值得注意的是，`struct seccomp_data` 包含系统调用寄存器参数的值，但不包含指向内存的指针。任务的內存可通过 `ptrace()` `/proc/pid/mem` 供拥有相应权限的跟踪器访问。但是，应注意避免本文档前述提到TOCTOU：在做出任何策略决定之前，从被跟踪者内存中读取的所有参数都应先读入跟踪器的内存。这使得对系统调用参数能够做出原子性决定
## Sysctl 参数


Seccomp sysctl 文件位于 `/proc/sys/kernel/seccomp/` 目录中。下面描述该目录中的每个文件
`actions_avail`	以字符串形式给出的、只读且有序seccomp 返回值列表（参见上文 `SECCOMP_RET_*` 宏）。其从左到右的排列顺序为从最不宽松的返回值到最宽松的返回值
	该列表表示内核支持的 seccomp 返回值集合。用户态程序可使用该列表来判断：程序构建时 `seccomp.h` 中的动作，与当前运行内核实际支持的动作集合是否不同
`actions_logged`	一个可读写的、有序的 seccomp 返回值列表（参见上文 `SECCOMP_RET_*` 宏），表示允许被记录的返回值。写入该文件时无需有序，但读取时将以与 actions_avail sysctl 相同的方式排序
	`actions_logged` sysctl 不接`allow` 字符串，因为无法记录 `SECCOMP_RET_ALLOW` 动作。尝试向sysctl 写入 `allow` 会导致返EINVAL
## 添加架构支持


权威要求参见 `arch/Kconfig`。一般来说，如果某个架构同时支持 ptrace_event seccomp，它就能以少量修补支seccomp 过滤器：`SIGSYS` 支持seccomp 返回值检查。然后它只需在其架构特定Kconfig 中添`CONFIG_HAVE_ARCH_SECCOMP_FILTER`
## 注意事项


vDSO 可能导致某些系统调用完全在用户态运行，当你在不同机器上运行程序、而这些程序回退到真实系统调用时，会造成意外。为了在 x86 上尽量减少这类意外，务必在测试时`/sys/devices/system/clocksource/clocksource0/current_clocksource` 设为类似 `acpi_pm` 的值
x86-64 上，vsyscall 模拟默认是启用的。（vsyscall vDSO 调用的旧式变体。）目前，被模拟vsyscall 会遵seccomp，但有一些怪异之处
- `SECCOMP_RET_TRAP` 的返回值会`si_call_addr` 设为指向给定调用vsyscall 入口，而非 'syscall' 指令之后的地址。任何希望重启该调用的代码都应意识到a) 一ret 指令已被模拟b) 尝试恢复系统调用会再次触发标准的 vsyscall 模拟安全检査，使得恢复系统调用基本没有意义
- `SECCOMP_RET_TRACE` 的返回值会像往常一样向跟踪器发信号，但无法使用 orig_rax 寄存器将系统调用改为另一个系统调用。只能将其改-1 以跳过当前被模拟的调用。任何其他改动都可能终止进程。跟踪器看到rip 值将是系统调用入口地址；这与正常行为不同。跟踪器绝不可修rip rsp。（不要依赖其他改动来终止进程。它们或许能生效。例如，在某些内核上，选择一个仅在未来内核中存在的系统调用会被正确模拟（通过返回 `-ENOSYS`）。）

要检测这种古怪行为，请检``addr & ~0x0C00 == 0xFFFFFFFFFF600000`。（对于 `SECCOMP_RET_TRACE`，使rip；对`SECCOMP_RET_TRAP`，使`siginfo->si_call_addr`。）不要检查任何其他条件：未来内核可能会改vsyscall 模拟，而当前内核在 vsyscall=native 模式下行为也会不同，但在这些情况`0xF...F600{0,4,8,C}00` 处的指令不会是系统调用
注意，现代系统几乎不可能使用 vsyscall——它们是遗留特性，且比标准系统调用慢得多。新代码会使vDSO，而由 vDSO 发起的系统调用与正常的系统调用无法区分