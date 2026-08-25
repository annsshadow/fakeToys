## Yama


Yama 是一Linux 安全模块（Linux Security Module），它收集核心内核本身未处理的、系统范围的
DAC 安全保护。它可以在构建时通过 `CONFIG_SECURITY_YAMA` 选择，并可在运行时通过
`/proc/sys/kernel/yama` 中的 sysctl 进行控制
## ptrace_scope


随着 Linux 日益流行，它将成为恶意软件更大的目标。Linux 进程接口一个特别令人担忧的弱点是，
单个用户可以检查其任何进程的内存和运行状态。例如，如果某个应用程序（如 Pidgin）被攻破，攻者就可以附加到其他正在运行的进程（如 Firefox、SSH 会话、GPG agent 等），以提取额外的凭据，
并在不借助用户协助的网络钓鱼的情况下继续扩大攻击范围
这并非一个理论问题。`SSH session hijacking
<https://www.blackhat.com/presentations/bh-usa-05/bh-us-05-boileau.pdf>`_
鍜?`arbitrary code injection
<https://c-skills.blogspot.com/2007/05/injectso.html>`_
攻击已经存在，并且如ptrace 被允许像以前一样运行，它们仍然可能发生。由ptrace 很少被非
开发者和非管理员使用，应允许系统构建者选择禁用此调试系统
作为解决方案，一些应用程序使`prctl(PR_SET_DUMPABLE, ...)` 专门禁止此类 ptrace 附加
（例ssh-agent），但许多应用程序没有这样做。一个更通用的解决方案是只允许从父进程直接对进程进行 ptrace（即直接“gdb EXE“strace EXE仍然有效），或者需`CAP_SYS_PTRACE`
（即 “gdb --pid=PID“strace -p PID作为 root 仍然有效）
在模1 下，定义了调试进程与其下级（inferior）之间应用特定关系的软件（崩溃处理程序等）可使用 `prctl(PR_SET_PTRACER, pid, ...)`。一个下级可以声明允许哪些其他进程（及其后代）对其调`PTRACE_ATTACH`。每个下级一次只能存在一个这样的已声明调试进程。例如，KDE、Chromium Firefox
的崩溃处理程序，以及 Wine（用于只允许 Wine 进程之间相互 ptrace）使用了它。如果一个进程希完全禁用这些 ptrace 限制，它可以调用 `prctl(PR_SET_PTRACER, PR_SET_PTRACER_ANY, ...)`，以任何其他本应被允许的进程（即使是外部 pid 命名空间中的进程）都可以附加
sysctl 设置（只`CAP_SYS_PTRACE` 才能写入）为
0 - 经典 ptrace 权限    一个进程可`PTRACE_ATTACH` 到任何在相同 uid 下运行的其他进程，只要它是可转储的（    没有切换uid、没有以特权启动，或没有已经调用`prctl(PR_SET_DUMPABLE...)`）。类似地    `PTRACE_TRACEME` 不变
1 - 受限 ptrace    一个进程必须与其想要调`PTRACE_ATTACH` 的下级有预定义的关系。默认情况下，这种关系是
    仅当上述经典条件也满足时的后代关系。要更改关系，下级可以调    `prctl(PR_SET_PTRACER, debugger, ...)` 来声明一个被允许的调试器 PID 对该下级调用
    `PTRACE_ATTACH`。使`PTRACE_TRACEME` 不变
2 - 仅管理员附加    只有带有 `CAP_SYS_PTRACE` 的进程可以使ptrace，无论是通过 `PTRACE_ATTACH` 还是通过
    子进程调`PTRACE_TRACEME`
3 - 禁止附加    没有任何进程可以使用 `PTRACE_ATTACH` 或通过 `PTRACE_TRACEME` 使用 ptrace。一旦设置，    sysctl 值无法更改
最初的仅子进程逻辑基于 grsecurity 中的限制