
## 系统调用用户态分发（Syscall User Dispatch

### 背景


Wine 这样的兼容性层需要一种高效的方法来仅模拟其进程中某一部分（即包含
不兼容代码的那部分）的系统调用，同时能够在本进程的原生部分上执行原生系统调用
而不会产生很高的性能损耗。Seccomp 在这项任务上力有不逮，因为它对基于内存区域
高效过滤系统调用的支持有限，并且不支持移除过滤器。因此有必要引入一种新的机制
Syscall User Dispatch 将系统调用分发器地址的过滤交还给用户空间。应用程序控一个拨动开关，用于指示进程当前的“人格”（personality）。一个多人格应用程序
因而可以在跨越兼容性层 API 边界时，无需调用内核即可拨动开关，以启禁用系统
调用重定向，并直接执行系统调用（禁用）或通过 SIGSYS 将其发送到用户空间进行模拟
本设计的目标是提供非常快速的兼容性层边界穿越，这是通过不在每次兼容性层执行都发起一个系统调用来改变人格来实现的。相反，一个暴露给内核的用户空间内存区指示当前的人格，应用程序只需修改该变量即可配置该机制
在大多数架构（如 x86）上，处理信号的成本相对较高，但至少对于 Wine 而言，由
原生 Windows 代码发起的系统调用目前并不构成性能问题，因为它们相当罕见，至少
对现代游戏应用是如此
由于该机制旨在捕获由非原生应用程序发起的系统调用，它必须能够作用于那些调ABI Linux 完全未知的系统调用。因此，Syscall User Dispatch 不依赖任何系调用 ABI 来做过滤。它只使用系统调用分发器地址和用户空间密钥
由于这些被拦截系统调用的 ABI Linux 未知，它们无法通过 ptrace 或系统调跟踪点进行插桩
### 接口


线程可以通过在受支持的内核上执行以下 prctl 来建立该机制
  prctl(PR_SET_SYSCALL_USER_DISPATCH, <op>, <offset>, <length>, [selector])

<op> PR_SYS_DISPATCH_EXCLUSIVE_ON/PR_SYS_DISPATCH_INCLUSIVE_ON PR_SYS_DISPATCH_OFF，用于全局启用和禁用该线程的机制。使PR_SYS_DISPATCH_OFF
时，其它字段必须为零
对于 PR_SYS_DISPATCH_EXCLUSIVE_ON，[<offset>, <offset>+<length>) 划定一内存区域区间，无论用户空间选择器如何，来自该区间的系统调用总是被直接执行这为 C 库提供了一条快速路径，C 库包含了原生代码应用程序中最常见的系统调分发器，并且也为信号处理程序提供了一种在不触发嵌SIGSYS 的情况下(rt\_)sigreturn 返回的途径。该接口的使用者应确保至少信号跳板代码包含在该
区域内。此外，对于vDSO 上实现跳板代码的系统调用，该跳板永远不会被拦截
对于 PR_SYS_DISPATCH_INCLUSIVE_ON，[<offset>, <offset>+<length>) 划定一内存区域区间，来自该区间的系统调用根据用户输入选择器进行分发。来自该范围之外
的系统调用总是被直接执行
[selector] 是指向进程内存区域中一char 大小区域的指针，它提供了一种无需直接
调用内核即可在线程范围内快速启禁用系统调用重定向的方法。selector 可以设置SYSCALL_DISPATCH_FILTER_ALLOW SYSCALL_DISPATCH_FILTER_BLOCK。任何其它值都
应以 SIGSYS 终止程序
此外，任务的系统调用用户态分发配置可以通过 PTRACE_(GET|SET)_SYSCALL_USER_DISPATCH_CONFIG
ptrace 请求进行查看和修改。这对于检查点/重启软件很有用
### 安全注意事项


Syscall User Dispatch 为兼容性层提供了一种功能，用于快速捕获应用程序中非原部分发起的系统调用，同时不影响进程中Linux 原生区域。它并非用于沙箱化系调用的机制，也不应被视为一种安全机制，因为一个恶意应用程序很容易通过在执系统调用之前跳转到允许的 dispatch 区域，或者发现地址并修改选择器值，从绕过该机制。如果用例需要任何形式的系统调用沙箱化，应当改用 Seccomp
现有进程的任fork exec 都会将该机制重置PR_SYS_DISPATCH_OFF