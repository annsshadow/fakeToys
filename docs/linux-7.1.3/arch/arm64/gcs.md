## AArch64 Linux 的受保护控制栈（Guarded Control Stack）支持


本文档简要概述了 Linux 提供给用户空间的接口，用以支持 ARM 受保护控制栈
（GCS，Guarded Control Stack）特性的使用。

这仅是对最重要特性和问题的概述，并非穷尽所有内容。



### 1.  概述


- GCS 是一项架构特性，旨在提供针对面向返回编程（ROP）攻击更强的保护，
  并简化亟需收集栈回溯（如性能剖析）的特性的实现。

- 当启用 GCS 时，PE 会维护一个独立的受保护控制栈，该栈只能通过特定的
  GCS 操作写入。它只存储调用栈：当执行过程调用指令时，当前 PC 被压入
  GCS；而在 RET 时，LR 中的地址会与控制栈顶部的地址进行核对。

- 处于活动状态时，当前 GCS 指针存储在系统寄存器 GCSPR_EL0 中。该寄存器
  对用户空间可读，但只能通过特定的 GCS 指令更新。

- 架构提供了在受保护控制栈之间切换的指令，并带有检查以确保新栈是有效的
  切换目标。

- GCS 的功能类似于 x86 Shadow Stack（影子栈）特性所提供的功能，由于用户
  空间接口的共享，ABI 中称其为 shadow stack 而非 GCS。

- 对 GCS 的支持通过 aux vector 中 AT_HWCAP 条目的 HWCAP_GCS 报告给用户空间。

- GCS 是按线程启用的。虽然支持在运行时禁用 GCS，但这应极为谨慎地进行。

- GCS 内存访问错误被作为普通内存访问错误报告。

- GCS 特定的错误（以 EC 0x2d 报告的那些）将被报告为 SIGSEGV，其 si_code
  为 SEGV_CPERR（控制保护错误）。

- GCS 仅支持 AArch64。

- 在支持 GCS 的系统上，无论该线程的 GCS 配置如何，GCSPR_EL0 始终对 EL0
  可读。

- 架构支持在未核对 LR 中的返回值与 GCS 中的值是否匹配的情况下启用 GCS
  （此时 LR 将被忽略）。Linux 不支持此模式。



### 2.  启用与禁用受保护控制栈


- 通过 PR_SET_SHADOW_STACK_STATUS prctl() 为线程启用和禁用 GCS，它接受
  单个 flags 参数，指定应使用哪些 GCS 特性。

- 设置 PR_SHADOW_STACK_ENABLE 标志会分配一个受保护控制栈并为线程启用
  GCS，从而启用由 GCSCRE0_EL1.{nTR, RVCHKEN, PCRSEL} 控制的功能。

- 设置 PR_SHADOW_STACK_PUSH 标志会启用由 GCSCRE0_EL1.PUSHMEn 控制的
  功能，允许显式的 GCS 压栈。

- 设置 PR_SHADOW_STACK_WRITE 标志会启用由 GCSCRE0_EL1.STREn 控制的
  功能，允许对受保护控制栈进行显式存储。

- 任何未知标志都会导致 PR_SET_SHADOW_STACK_STATUS 返回 -EINVAL。

- PR_LOCK_SHADOW_STACK_STATUS 传入的位掩码特性与用于
  PR_SET_SHADOW_STACK_STATUS 的取值相同。对指定 GCS 模式位的任何未来
  状态变更都将被拒绝。

- PR_LOCK_SHADOW_STACK_STATUS 允许锁定任意比特，这使得用户空间能够阻止
  对任何未来特性的变更。

- 不支持进程移除为其设置的锁。

- PR_SET_SHADOW_STACK_STATUS 和 PR_LOCK_SHADOW_STACK_STATUS 只影响调用
  它们的线程，任何其他正在运行的线程都不受影响。

- 新线程继承创建它们的线程的 GCS 配置。

- 在 exec() 时 GCS 被禁用。

- 可以通过 PR_GET_SHADOW_STACK_STATUS prctl() 读取线程当前的 GCS 配置，
  它返回与传给 PR_SET_SHADOW_STACK_STATUS 相同的标志。

- 如果某个线程此前曾启用 GCS，之后又被禁用，则该栈会在线程的整个生存期内
  保持分配。目前，任何重新启用该线程 GCS 的尝试都会被拒绝，这一点未来
  可能会重新考虑。

- 应当注意，由于启用 GCS 会导致 GCS 立即变为活动状态，通常无法从调用了
  启用 GCS 的 prctl() 的那个函数返回。预期的正常用法是，在程序执行的
  很早阶段就启用 GCS。



### 3.  受保护控制栈的分配


- 当为一个线程启用 GCS 时，会为其分配一个新的受保护控制栈，大小为标准
  栈的一半或 2 GB 中的较小者。

- 当由已启用 GCS 的线程创建一个新线程时，会为新线程分配一个新的受保护
  控制栈，大小为标准栈的一半。

- 当通过启用 GCS 或在创建线程期间分配栈时，栈顶 8 字节将被初始化为 0，
  并且 GCSPR_EL0 将被设置为指向这个 0 值的地址，这可用于检测栈顶。

- 可以使用 map_shadow_stack() 系统调用分配额外的受保护控制栈。

- 使用 map_shadow_stack() 分配的栈可以选择在栈顶放置一个栈结束标记和
  封顶（cap）。如果指定了 SHADOW_STACK_SET_TOKEN 标志，则会在栈上放置一个
  封顶；如果没有指定 SHADOW_STACK_SET_MARKER，则该封顶将是栈顶 8 字节；
  如果指定了它，则封顶将是接下来的 8 字节。虽然仅单独指定
  SHADOW_STACK_SET_MARKER 是有效的（因为标记是全 0 比特），但它没有可
  观察到的效果。

- 使用 map_shadow_stack() 分配的栈的大小必须是 8 字节的倍数且大于 8 字节，
  并且必须 8 字节对齐。

- 可以向 map_shadow_stack() 指定一个地址，如果提供了地址，则它必须按页
  边界对齐。

- 当线程被释放时，最初为该线程分配的受保护控制栈会被释放。请特别注意，
  如果栈已被切换，这可能不是该线程当前正在使用的栈。



### 4.  信号处理


- 一个新的信号帧记录 gcs_context 对信号递送时被中断上下文的当前 GCS 模式
  和指针进行编码。在支持 GCS 的系统上，它将始终存在。

- 该记录包含一个标志字段，报告被中断上下文的当前 GCS 配置，方式与
  PR_GET_SHADOW_STACK_STATUS 相同。

- 信号处理函数以与被中断上下文相同的 GCS 配置运行。

- 当被中断线程启用了 GCS 时，一个特定于信号处理的 GCS 封顶令牌将被写入
  GCS，这是一个体系结构 GCS 封顶，其令牌类型（比特 0..11）全部清零。信号
  帧中报告的 GCSPR_EL0 将指向这个封顶令牌。

- 信号处理函数将使用与被中断上下文相同的 GCS。

- 在信号进入时若启用了 GCS，则带有信号返回处理函数地址的帧会被压入 GCS，
  从而允许像平常一样通过 RET 从信号处理函数返回。这不会在信号帧的
  gcs_context 中报告。



### 5.  信号返回


从信号处理函数返回时：

- 如果信号帧中存在 gcs_context 记录，则在进行进一步验证之前，GCS 标志
  和 GCSPR_EL0 将从该上下文恢复。

- 如果信号帧中不存在 gcs_context 记录，则 GCS 配置保持不变。

- 如果从信号处理函数返回时启用了 GCS，则 GCSPR_EL0 必须指向一个有效的
  GCS 信号封顶记录，该记录会在信号返回之前从 GCS 中弹出。

- 如果在信号返回时 GCS 配置被锁定，则任何更改 GCS 配置的尝试都将被视作
  错误。即使 GCS 在信号进入之前未启用，这一点也成立。

- 可以通过信号返回禁用 GCS，但任何通过信号返回启用 GCS 的尝试都将被
  拒绝。



### 6.  ptrace 扩展


- 定义了一个新的 regset NT_ARM_GCS，用于 PTRACE_GETREGSET 和
  PTRACE_SETREGSET。

- GCS 模式（包括启用和禁用）可以通过 ptrace 配置。如果通过 ptrace 启用了
  GCS，则不会为该线程分配新的 GCS。

- 通过 ptrace 进行的配置会忽略 GCS 模式位的锁定。



### 7.  ELF coredump 扩展


- 对于被转储进程的每个线程，NT_ARM_GCS 注释将被添加到每个 coredump 中。
  其内容等价于：如果在生成 coredump 时对每个线程执行相应类型的
  PTRACE_GETREGSET 所读取到的数据。



### 8.  /proc 扩展


- 受保护控制栈页面将在 /proc/<pid>/smaps 的 VmFlags 中包含 "ss"。
