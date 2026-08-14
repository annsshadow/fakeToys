## IRQ 标志状态追踪


:Author: 由 Ingo Molnar <mingo@redhat.com> 发起

“irq 标志追踪”（irq-flags tracing）特性会“追踪”硬中断（hardirq）与
软中断（softirq）状态，即它给感兴趣的子系统一个机会，以获知内核中
发生的每一次 hardirqs-off/hardirqs-on、softirqs-off/softirqs-on 事件。

通用锁调试代码需要 CONFIG_TRACE_IRQFLAGS_SUPPORT，才能提供
CONFIG_PROVE_SPIN_LOCKING 与 CONFIG_PROVE_RW_LOCKING。否则在某个
架构上只会提供 CONFIG_PROVE_MUTEX_LOCKING 与 CONFIG_PROVE_RWSEM_LOCKING —
这些是不会在 IRQ 上下文中使用的加锁 API。（rwsem 的唯一例外已被规避）

架构对此的支持当然不属于“平凡”一类，因为大量底层汇编代码会处理
irq 标志状态的改变。但一个架构可以通过一种相当直接且无风险的方式
启用 irq 标志追踪。

想要支持此特性的架构需要先做一些代码组织上的改动：

- 在其架构级 Kconfig 文件中添加并启用 TRACE_IRQFLAGS_SUPPORT

然后还需要做几处功能性改动以实现 irq 标志追踪支持：

- 在底层入口代码中添加（构建条件化的）对 trace_hardirqs_off()/
  trace_hardirqs_on() 函数的调用。锁验证器会严密守护“真实” irq 标志
  是否与“虚拟” irq 标志状态相匹配，若两者不匹配则会大声抱怨（并关闭
  自身）。架构支持 irq 标志追踪所花的时间通常大部分都处于这种状态：
  查看 lockdep 的抱怨，尝试找出我们尚未覆盖的汇编代码，修复并重复。
  一旦系统启动并能在 irq 标志追踪函数中无 lockdep 抱怨地工作，架构
  支持即告完成。
- 如果架构拥有不可屏蔽中断（NMI），那么需要通过 lockdep_off()/
  lockdep_on() 将它们排除在 irq 追踪 [以及锁验证] 机制之外。

一般而言，架构上拥有不完整的 irq 标志追踪实现并没有风险：lockdep 会
检测到这一点并自行关闭。也就是说，锁验证器依然是可靠的。不应出现因
irq 追踪缺陷导致的崩溃。（除非汇编改动通过修改本不应修改的条件或
寄存器而破坏了其它代码）
