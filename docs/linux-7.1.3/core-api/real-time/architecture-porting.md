
## Porting an architecture to support PREEMPT_RT


:Author: Sebastian Andrzej Siewior <bigeasy@linutronix.de>

下面列出了为了启PREEMPT_RT 而必须实现的、与体系架构相关的需求。一旦所有必需的特性都实现完毕，就可以在该体系架构Kconfig 中选择 ARCH_SUPPORTS_RT，从而使 PREEMPT_RT 可选许多前置条件（例genirq 支持）由公共代码强制要求，此处不再赘述
可选特性并非严格必需，但仍值得考虑
### Requirements


强制线程化中断（Forced threaded interrupts  CONFIG_IRQ_FORCED_THREADING 必须被选中。任何必须保留在IRQ（hard-IRQ）上下文中的中断，必须用 IRQF_NO_THREAD 标记。例如，这一要求适用于时钟源事件中断、perf 中断以及级联中断控制器处理程序
抢占（PREEMPTION）支  必须支持内核抢占，并且要CONFIG_ARCH_NO_PREEMPT 保持未选中状态。调度请求（例如从中断或其他异常处理器发出的请求）必须被立即处理
POSIX CPU 定时器与 KVM
  POSIX CPU 定时器必须从线程上下文到期，而不是直接在定时器中断内部到期。通过设置配置选项 CONFIG_HAVE_POSIX_CPU_TIMERS_TASK_WORK 可以启用这一行为  当启用虚拟化支持（例KVM）时，还必须设置 CONFIG_VIRT_XFER_TO_GUEST_WORK，以确保任何待处理的工作（例POSIX 定时器到期）在进入客户机模式之前得到处理
IRQ 与软 IRQ   软中断在引发它们的线程上下文中被处理。如果软中断是从IRQ 上下文触发的，它的执行会被推迟到 ksoftirqd 线程。在软中断处理期间，抢占永远不会被禁用，这使得软中断是可抢占的  如果某个体系架构提供了使用独立栈的自定义 __do_softirq() 实现，它必须选择 CONFIG_HAVE_SOFTIRQ_ON_OWN_STACK。该功能只应在设置了 CONFIG_SOFTIRQ_ON_OWN_STACK 时启用
内核模式下的 FPU SIMD 访问
  FPU SIMD 寄存器通常不会在内核模式中使用，因此在内核抢占时不会被保存。因此，任何使用这些寄存器的内核代码都必须包含在 kernel_fpu_begin() kernel_fpu_end() 区段之内  kernel_fpu_begin() 函数通常会调local_bh_disable()，以防止来自软中断的打断，并禁用常规抢占。这允许受保护的代码在线程和软中断两种上下文中都安全运行  然而，PREEMPT_RT 内核上，kernel_fpu_begin() 不能调用 local_bh_disable()。相反，它应该使preempt_disable()，因为在 PREEMPT_RT 下，软中断总是在线程上下文中处理。在这种情况下，仅禁用抢占就足够了  crypto 子系统操作内存页，并要求用户在处理请求时对这些页进行"遍历和映。这一操作必须发生kernel_fpu_begin()/kernel_fpu_end() 区段之外，因为它需要启用抢占。这些抢占点通常足以避免过度的调度延迟
异常处理器（Exception handlers  异常处理器（例如缺页处理器）通常会提前启用中断，然后再调用任何用于处理该异常的通用代码。这是必要的，因为处理缺页可能涉及可能休眠的操作。在 PREEMPT_RT 上启用中断尤为重要，因为PREEMPT_RT 下，某些锁（例如 spinlock_t）变成了可睡眠的。例如，处理非法操作码可能会导致向用户任务发SIGILL 信号。调试异常会发SIGTRAP 信号  在这两种情况下，如果异常发生在用户空间，提前启用中断是安全的。发送信号需要同时启用中断和内核抢占
### Optional features


定时器与时钟  建议使用高分辨率时钟源和 clock event 设备。clock event 设备应支CLOCK_EVT_FEAT_ONESHOT 特性，以获得最佳的定时器行为。在大多数情况下，微秒级的精度已经足够
惰性抢占（Lazy preemption  该机制允许将针对非实时任务的内核调度请求延迟到该任务即将返回用户空间时。它有助于避免在发出调度请求时抢占一个持有睡眠锁的任务  在启CONFIG_GENERIC_IRQ_ENTRY 的情况下，支持该特性需要为 TIF_NEED_RESCHED_LAZY 定义一个位，最好放TIF_NEED_RESCHED 附近
使用 NBCON 的串口控制台
  在启PREEMPT_RT 的情况下，所有控制台输出都由专用线程处理，而不是直接在调用 printk() 的上下文中进行。这种设计允printk() 在原子上下文中安全使用  然而，这也意味着，如果内核崩溃且无法切换到打印线程，则不会有任何输出可见，从而使系统无法打印其最终的消息  对于立即输出存在例外情况，例如在 panic() 处理期间。为了支持这一点，控制台驱动必须实现新式的锁处理。这涉及console::flags 中设CON_NBCON 标志，并提供 write_atomic、write_thread、device_lock device_unlock 回调的实现