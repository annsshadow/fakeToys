
## RCU 补丁审查清单

本文档包含一份用于制作和审查使用RCU 的补丁的清单。违反下面列出的任何规则，都会导致与遗漏某个锁原语（locking primitive）相同类型的问题。这份清单基于在相当长一段时间内审查此类补丁的经验，但我们也随时欢迎改进
0. RCU 是否被应用到一个以读取为主（read-mostly）的场景？如果数据结构更新的频率超过10%，那么你应该认真考虑其他一些方法，除非详细的性能测量表明 RCU 仍然是适合这项工作的工具。是的，RCU 确实通过增加写侧开销来降低读侧开销，这正是为什RCU 的正常使用会有比更新多得多的读取
	另一个例外是性能不是问题，RCU 提供了更简单的实现。这种情况的一个例子是 Linux 2.6 内核中的动NMI 代码，至少是NMI 罕见的一些架构上
	还有一个例外是，RCU 读侧原语的低实时延迟至关重要
	最后一个例外是，RCU 读者被用来防止无锁更新ABA 问题（https://en.wikipedia.org/wiki/ABA-problem）。这确实会导致一种略显反直觉的情况：rcu_read_lock() rcu_read_unlock() 被用来保护更新，然而，这种方法可以为某些类型的无锁算法提供与垃圾收集器相同的简化
1. 更新代码是否具备适当的互斥（mutual exclusion）？

	RCU 确实允许**读*几乎赤裸地运行，*写*仍然必须使用某种互斥，例如：

	a.	加锁	b.	原子操作，或
	c.	将更新限制为单个任务
	如果你选择 #b，就要准备好说明你是如何在弱序（weakly ordered）机器上处理内存屏障的（几乎全都是弱序机器——即x86 也允许后续的加载被重排到更早的存储之前），并准备好解释为什么增加的这种复杂性是值得的。如果你选择 #c，就要准备好解释这个单一任务为什么不会成为大型系统上的主要瓶颈（例如，如果该任务正在更新与自身相关的、其他任务可以读取的信息，那么按定义就不会有瓶颈）。注意“大型”的定义已经发生了显著变化：2000 年时 8 CPU 就算“大型”，2017 年一百个 CPU 都不足为奇了
2. RCU 读侧临界区是否正确使用了 rcu_read_lock() 及其同类？需要这些原语来防止宽限期（grace period）过早结束，否则可能导致数据在你的读侧代码之下被毫无礼节地释放，这会大大增加你内核的精算风险
	作为一条粗略的经验法则，任何对 RCU 保护的指针的解引用，都必须被 rcu_read_lock()、rcu_read_lock_bh()、rcu_read_lock_sched() 或相应的更新侧锁覆盖。显式禁用抢占（例如 preempt_disable()）可以充rcu_read_lock_sched()，但可读性较差，并且会阻lockdep 检测锁问题。获取一raw spinlock 同样会进入一RCU 读侧临界区
	guard(rcu)() scoped_guard(rcu) 原语分别将当前作用域的剩余部分或下一条语句指定为 RCU 读侧临界区。使用这guard 可以rcu_read_lock()、rcu_read_unlock() 及其同类更不容易出错
	请注意，*不能**依赖已知只在不可抢占内核中构建的代码。这样的代码可能且终将崩溃，尤其是在CONFIG_PREEMPT_COUNT=y 构建的内核中
	RCU 保护的指针从 RCU 读侧临界区“泄漏”出去，与让它们从锁保护下泄漏出去一样糟糕。除非，当然，你安排了其他某种保护方式，例如在让它们离开 RCU 读侧临界*之前**，使用一把锁或一个引用计数
3. 更新代码是否能容忍并发访问？

	RCU 的全部意义在于允许读者在没有锁或原子操作的情况下运行。这意味着读者会在更新进行期间运行。根据情况，有若干种方式来处理这种并发：

	a.	使用 list hlist 更新原语RCU 变体，来RCU 保护的列表上添加、删除和替换元素。或者，使用已经加入 Linux 内核的其RCU 保护的数据结构
		这几乎总是最好的方法
	b.	按上(a) 的方式进行，同时维护每元素锁（由读者和写者都获取），以保护每元素状态。读者不去访问的字段，如果愿意，可以由仅由更新者获取的某把其他锁来保护
		这也相当好用
	c.	使更新对读者而言看起来是原子的。例如，对正确对齐的字段的指针更新看起来将是原子的，单个原子原语也是如此。在锁下执行的一系列操作RCU 读*不会**看起来是原子的，多个原子原语组成的序列也不会。一种替代方案是将多个单独的字段移到一个单独的结构中，从而通过引入额外一层间接来化解多字段问题
		这可以奏效，但开始有点棘手了
	d.	仔细地对更新和读取排序，使读者在更新的所有阶段都能看到有效数据。这通常比听起来更困难，尤其是考虑到现CPU 倾向于重排内存引用。通常必须在代码中大量散布内存排序操作，使其难以理解和测试。在可行的地方，最好使smp_store_release() smp_load_acquire() 之类的东西，但在某些情况下需smp_mb() 全内存屏障
		如前所述，通常更好的做法是将变化的数据分组到一个单独的结构中，这样通过更新一个指向包含更新值的新结构的指针，可以使变更看起来是原子的
4. 弱序 CPU 带来了特殊的挑战。几乎所CPU 都是弱序的——即x86 CPU 也允许后续的加载被重排到更早的存储之前。RCU 代码必须采取以下所有措施来防止内存损坏问题
	a.	读者必须保持其内存访问的正确排序。rcu_dereference() 原语确保 CPU 在获取指针所指向的数据之前先获取该指针。这Alpha CPU 上确实是必要的
		rcu_dereference() 原语也是一个极好的文档辅助，让读代码的人确切知道哪些指针受 RCU 保护。请注意编译器也可以重排代码，而且它们在这方面正变得日益激进。因rcu_dereference() 原语也能防止破坏性的编译器优化。然而，用一点狡猾的创造力，是有可能误rcu_dereference() 的返回值的。更多信息请参阅 rcu_dereference.rst
		rcu_dereference() 原语被各"_rcu()" 列表遍历原语使用，例list_for_each_entry_rcu()。注意，更新侧代码使rcu_dereference() "_rcu()" 列表遍历原语是完全合法（尽管多余）的。这在读者与更新者共用的代码中特别有用。然而，如果你在 RCU 读侧临界区之外访rcu_dereference()，lockdep 会报警。请参阅 lockdep.rst 了解该如何处理
		当然，无论是 rcu_dereference() 还是 "_rcu()" 列表遍历原语，都无法替代一个协调多个更新者之间良好并发设计
	b.	如果正在使用列表宏，必须使用 list_add_tail_rcu() list_add_rcu() 原语，以防止弱序机器错排结构初始化与指针植入。类似地，如果正在使hlist 宏，则需hlist_add_head_rcu() 原语
	c.	如果正在使用列表宏，必须使用 list_del_rcu() 原语，以防止 list_del() 的指针“投毒”对并发读者造成毒性影响。类似地，如果正在使hlist 宏，则需hlist_del_rcu() 原语
		list_replace_rcu() hlist_replace_rcu() 原语可用于在各自类型RCU 保护列表中用新结构替换旧结构
	d.	类似(4b) (4c) 的规则适用"hlist_nulls" 类型RCU 保护链表
	e.	更新必须确保对某结构的初始化发生在指向该结构的指针被公开之前。在公开一个可以被 RCU 读侧临界区遍历的结构的指针时使用 rcu_assign_pointer() 原语
5. 如果使用call_rcu()、call_srcu()、call_rcu_tasks() call_rcu_tasks_trace() 中的任何一个，回调函数的调用可能来softirq 上下文，并且无论如何都是在禁用底半部（bottom halves）的情况下。特别是，这个回调函数不能阻塞。如果你需要回调阻塞，就在从回调调度的 workqueue 处理程序中运行那段代码。在 call_rcu() 的情况下，queue_rcu_work() 函数为你做这件事
6. 由于 synchronize_rcu() 可能阻塞，所以不能在任意 irq 上下文中调用它。同样的规则适用synchronize_srcu()、synchronize_rcu_expedited()、synchronize_srcu_expedited()、synchronize_rcu_tasks()、synchronize_rcu_tasks_rude() synchronize_rcu_tasks_trace()
	这些原语expedited（加速）形式与非 expedited 形式语义相同，但加速会更消CPU。expedited 原语的使用应限制在罕见的配置变更操作上，这类操作通常不会在实时工作负载运行期间进行。注意，IPI 敏感的实时工作负载可以使rcupdate.rcu_normal 内核启动参数来完全禁用加速宽限期，尽管这可能会有性能方面的影响
	特别是，如果你发现自己在一个循环里反复调用某一expedited 原语，请帮大家一个忙：重构你的代码，让它对更新进行批处理，从而允许用单个expedited 原语覆盖整个批次。这极有可能比包expedited 原语的循环更快，并且对系统其余部分（尤其是运行在系统其余部分的实时工作负载）要友好得多。或者，改用 call_rcu() 之类的异步原语
7. v4.20 起，一个给定的内核只实现一RCU 风格，即 PREEMPTION=n 时为 RCU-sched，PREEMPTION=y 时为 RCU-preempt。如果更新者使call_rcu() synchronize_rcu()，那么相应的读者可以使用：(1) rcu_read_lock() rcu_read_unlock()2) 任何禁用并重新启softirq 的原语对，例rcu_read_lock_bh() rcu_read_unlock_bh()，或 (3) 任何禁用并重新启用抢占的原语对，例如 rcu_read_lock_sched() rcu_read_unlock_sched()。如果更新者使synchronize_srcu() call_srcu()，那么相应的读者必须使srcu_read_lock() srcu_read_unlock()，并且使用同一srcu_struct。expedited RCU 宽限期等待原语的规则与其expedited 对应物相同
	类似地，有必要正确使RCU Tasks 风格
	a.	如果更新者使synchronize_rcu_tasks() call_rcu_tasks()，那么读者必须避免执行自愿的上下文切换，即避免阻塞
	b.	如果更新者使call_rcu_tasks_trace() synchronize_rcu_tasks_trace()，那么相应的读者必须使rcu_read_lock_trace() rcu_read_unlock_trace()
	c.	如果更新者使synchronize_rcu_tasks_rude()，那么相应的读者必须使用任何禁用抢占的方式，例preempt_disable() preempt_enable()
	混淆这些会导致混乱和崩溃的内核，甚至已经导致过可利用的安全问题。因此，当使用不明显的原语对时，加注释当然是必要的。一个不明显配对的例子是网络中的 XDP 特性，它从网络驱动 NAPI（softirq）上下文调用 BPF 程序。BPF 严重依赖 RCU 保护其数据结构，但由BPF 程序的调用完全发生在 NAPI 轮询周期中的一个本local_bh_disable() 段内，这种用法是安全的。这种用法之所以安全，是因为当更新者使call_rcu() synchronize_rcu() 时，读者可以使用任何禁BH 的方式
8. 虽然 synchronize_rcu() call_rcu() 慢，但它通常带来更简单的代码。所以，除非更新性能至关重要、更新者无法阻塞，或synchronize_rcu() 的延迟从用户空间可见，否则应优先使用 synchronize_rcu() 而非 call_rcu()。此外，kfree_rcu() kvfree_rcu() 通常synchronize_rcu() 带来更简单的代码，且没有 synchronize_rcu() 的几毫秒级延迟。所以请在适用之处利用 kfree_rcu() kvfree_rcu() 的“发射即忘”（fire and forget）内存释放能力
	synchronize_rcu() 原语一个特别重要的特性是它会自动自我限制：如果宽限期由于某种原因被延迟，那么 synchronize_rcu() 原语会相应地延迟更新。相比之下，在宽限期被延迟的情况下，使用 call_rcu() 的代码应当显式限制更新速率，因为不这样做可能导致过度的实时延迟甚至 OOM 状况
	在使call_rcu()、kfree_rcu() kvfree_rcu() 时获得这种自限制特性的方式包括
	a.	保持一个由 RCU 保护的数据结构所使用的（包括那些正在等待宽限期过去的）数据元素的计数。对这个数量强制执行一个限制，按需暂停更新以允许先前延迟的释放完成。或者，只限制等待延迟释放的数量，而不是元素的总数
		暂停更新的一种方式是获取更新侧互斥体。（不要试图用自旋锁做这件事——其CPU 在该锁上自旋可能阻止宽限期结束。）另一种暂停更新的方式是让更新使用一个围绕内存分配器的包装函数，这样当等RCU 宽限期的待释放内存过多时，该包装函数模拟 OOM。当然还有许多其他变化
	b.	限制更新速率。例如，如果更新每小时只发生一次，那么不需要显式的速率限制，除非你的系统已经严重损坏。旧版本dcache 子系统采用了这种方法，用一个全局锁保护更新，限制其速率
	c.	受信任的更新——如果更新只能由超级用户或其他受信任用户手动完成，那么可能没必要自动限制它们。这里理论认为超级用户已经有大量让机器崩溃的手段
	d.	周期性调rcu_barrier()，每个宽限期允许有限数量的更新
	同样的注意事项适用call_srcu()、call_rcu_tasks() call_rcu_tasks_trace()。这就是为什么分别存srcu_barrier()、rcu_barrier_tasks() rcu_barrier_tasks_trace()
	注意，尽管这些原语确实会采取行动以避免在任意给定 CPU 拥有过多回调时内存耗尽，但一个坚定的用户或管理员仍然可能耗尽内存。如果拥有大CPU 的系统被配置为将其所有的 RCU 回调卸载到单CPU 上，或者系统相对空闲内存很少，尤其如此
9. 所RCU 列表遍历原语，包rcu_dereference()、list_for_each_entry_rcu() list_for_each_safe_rcu()，必须要么位RCU 读侧临界区内，要么必须被适当的更新侧锁保护。RCU 读侧临界区由 rcu_read_lock() rcu_read_unlock() 界定，或由类似的原语rcu_read_lock_bh() rcu_read_unlock_bh() 界定，在后一种情况下必须使用匹配rcu_dereference() 原语（即 rcu_dereference_bh()）来lockdep 满意
	之所以允许在持有更新侧锁时使RCU 列表遍历原语，是因为这样做在读者与更新者共享公共代码时，对减少代码膨胀很有帮助。针对这种情况提供了额外的原语，详见 lockdep.rst
	这条规则的一个例外是：当数据只会被添加到链表数据结构，并且在读者可能访问该结构的任何时间内都不会被移除时。在这种情况下，可以READ_ONCE() 代替 rcu_dereference()，并且可以省略读侧标记（例如 rcu_read_lock() rcu_read_unlock()）
10. 反过来，如果你处RCU 读侧临界区中，并且没有持有适当的更新侧锁，你就**必须**使用列表宏的 "_rcu()" 变体。不这样做会破坏 Alpha，导致激进的编译器生成糟糕的代码，并让试图理解你代码的人感到困惑
11. RCU 回调获取的任何锁，都必须以禁softirq 的方式在其他地方获取，例如通过 spin_lock_bh()。未能在该锁的某次获取中禁用 softirq，一RCU softirq 处理程序碰巧在中断那次获取的关键区时运行你的 RCU 回调，就会导致死锁
12. RCU 回调可以并且确实会并行执行。在许多情况下，回调代码只是kfree() 的简单包装，所以这不是问题（或者更准确地说，就它是问题的程度而言，内存分配器的锁机制会处理它）。然而，如果回调确实操作一个共享数据结构，它们就必须使用访问和/或修改该数据结构所需的任何锁或其他同步机制
	不要假设 RCU 回调会在执行了相call_rcu()、call_srcu()、call_rcu_tasks() call_rcu_tasks_trace() 的同一 CPU 上执行。例如，如果某个给定 CPU 在有待处理的 RCU 回调时离线，那么RCU 回调将在某个幸存下来CPU 上执行。（如果不是这样，一个自我繁衍的 RCU 回调会阻止受CPU 永远下线。）此外，由 rcu_nocbs= 指定CPU 很可*总是**在另一CPU 上执行它们的 RCU 回调，事实上，对某些实时工作负载来说，这正是使用 rcu_nocbs= 内核启动参数的全部意义所在
	另外，不要假设以给定顺序入队的回调会按该顺序被调用，即使它们全都在同一CPU 上入队。此外，不要假设CPU 的回调会被串行调用。例如，在近期的内核中，CPU 可以在卸载（offloaded）与去卸载（de-offloaded）回调调用之间切换，而当某个给定 CPU 正在进行这种切换时，它的回调可能由该 CPU softirq 处理程序与该 CPU rcuo kthread 并发调用。在这种时候，CPU 的回调可能既被并发执行又被乱序执行
13. 与大多数 RCU 风格不同，在 SRCU 读侧临界区（srcu_read_lock() srcu_read_unlock() 界定）中阻塞***允许的，因此才有“SRCU”：“可睡眠 RCU”（sleepable RCU）。与 RCU 一样，guard(srcu)() scoped_guard(srcu) 形式也可用，并且通常提供更大的易用性。请注意，如果你不需要在读侧临界区中睡眠，你应该使用 RCU 而不SRCU，因RCU 几乎总是SRCU 更快且更易用
	也与其他形式RCU 不同，SRCU 需要显式的初始化和清理，要么在构建时通过 DEFINE_SRCU()、DEFINE_STATIC_SRCU()、DEFINE_SRCU_FAST() DEFINE_STATIC_SRCU_FAST()，要么在运行时通过 init_srcu_struct() init_srcu_struct_fast() 以及 cleanup_srcu_struct()。最后这三个传入一`struct srcu_struct`，它定义给定 SRCU 域的范围。一旦初始化，srcu_struct 就被传给 srcu_read_lock()、srcu_read_unlock()、synchronize_srcu()、synchronize_srcu_expedited() call_srcu()。一个给定的 synchronize_srcu() 只等待由传入了同一srcu_struct srcu_read_lock() srcu_read_unlock() 调用所管辖SRCU 读侧临界区。这一特性使得可睡眠的读侧临界区可以容忍——一个给定的子系统只延迟它自己的更新，而不是那些使SRCU 的其他子系统的更新。因此，SRCU 让系统陷OOM 的倾向比如RCU 的读侧临界区被允许睡眠时要小
	在读侧临界区中睡眠的能力并非没有代价。首先，相应srcu_read_lock() srcu_read_unlock() 调用必须传入同一srcu_struct。其次，宽限期检测的开销只在共享给定 srcu_struct 的那些更新之间摊销，而不是像其他 RCU 形式那样全局摊销。因此，SRCU 应只在极度读密集的情况下，或在需SRCU 的读侧死锁免疫或低读侧实时延迟的情况下，才优先于 rw_semaphore 使用。当你需要轻量级读者时，也应该考虑 percpu_rw_semaphore
	SRCU expedited 原语（synchronize_srcu_expedited()）从不向其他 CPU 发IPI，因此它对实时工作负载比 synchronize_rcu_expedited() 更友好
	RCU Tasks Trace 读侧临界区中睡眠也是允许的，它由 rcu_read_lock_trace() rcu_read_unlock_trace() 界定。然而，这是一种专门化RCU 风格，不应在未先与其当前使用者确认的情况下使用。在大多数情况下，你应该改用 SRCU。与 RCU SRCU 一样，guard(rcu_tasks_trace)() scoped_guard(rcu_tasks_trace) 也可用，并且通常提供更大的易用性
	注意，rcu_assign_pointer() SRCU 的关系与对其RCU 形式的关系一样，但你应该使用 srcu_dereference() 而不rcu_dereference()，以避免 lockdep 报错
14. call_rcu()、synchronize_rcu() 及其同类的全部意义，在于在执行某个原本具有破坏性的操作之前，等待所有已有的读者完成。因此，关键是要***移除读者可能遵循的、会受到该破坏性操作影响的任何路径，并*只有在那之后**才调call_rcu()、synchronize_rcu() 或其同类
	因为这些原语只等待已有的读者，所以保证任何后续读者都能安全执行是调用者的责任
15. 各种 RCU 读侧原语*不一*包含内存屏障。因此你应该预期 CPU 和编译器会自由地将代码重排进和重排出 RCU 读侧临界区。处理这件事RCU 更新侧原语的责任
	对于 SRCU 读者，你可以在 srcu_read_unlock() 之后立即使用 smp_mb__after_srcu_read_unlock() 来获得一个完整的屏障
16. 使用 CONFIG_PROVE_LOCKING、CONFIG_DEBUG_OBJECTS_RCU_HEAD __rcu sparse 检查来验证你的 RCU 代码。它们可以如下帮助发现问题：

	CONFIG_PROVE_LOCKING		检查对 RCU 保护的数据结构的访问是否在适当RCU 读侧临界区内、在持有正确的锁组合的情况下，或在其他任何适当的条件下进行
	CONFIG_DEBUG_OBJECTS_RCU_HEAD		检查你不会在自上次将同一对象传给 call_rcu()（或其同类）以来的一RCU 宽限期过去之前，把同一个对象再次传call_rcu()（或其同类）
	CONFIG_RCU_STRICT_GRACE_PERIOD		KASAN 结合使用，以检查从 RCU 读侧临界区泄漏出去的指针。这Kconfig 选项对性能和可扩展性都很苛刻，因此限制在四 CPU 的系统上
	__rcu sparse 检查：
		__rcu 标记指向 RCU 保护数据结构的指针，如果你在没有 rcu_dereference() 某一变体服务的情况下访问该指针，sparse 会警告你
	这些调试辅助可以帮助你发现否则极难发现的问题
17. 如果你将一个在模块内定义的回调函数传给 call_rcu()、call_srcu()、call_rcu_tasks() call_rcu_tasks_trace() 中的某一个，那么在卸载该模块之前，有必要等待所有待处理的回调都被调用。注意，仅仅等待一个宽限期是绝*不够**的！例如，synchronize_rcu() 的实**保证会等待通过 call_rcu() 在其CPU 上注册的回调。即使在当前 CPU 上，如果CPU 最近离线后又重新上线，也不保证
	你反而需要使用以下屏障函数之一
 - call_rcu() -> rcu_barrier()
 - call_srcu() -> srcu_barrier()
 - call_rcu_tasks() -> rcu_barrier_tasks()
 - call_rcu_tasks_trace() -> rcu_barrier_tasks_trace()

	然而，这些屏障函数绝对***保证会等待一个宽限期。例如，如果系统中任何地方都没有排队 call_rcu() 回调，rcu_barrier() 可以且将会立即返回
	所以如果你需要既等待一个宽限期、又等待所有已有的回调，你将需要同时调用这两个函数，具体配对取决于 RCU 的风格：

 - synchronize_rcu() synchronize_rcu_expedited() 之一，与 rcu_barrier() 一 - synchronize_srcu() synchronize_srcu_expedited() 之一，与 srcu_barrier() 一 - synchronize_rcu_tasks() rcu_barrier_tasks()
 - synchronize_tasks_trace() 鍜?rcu_barrier_tasks_trace()

	如果有必要，你可以使用类workqueue 的东西来并发执行所需的那一对函数
	更多信息请参rcubarrier.rst