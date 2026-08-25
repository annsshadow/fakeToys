
## 使用 RCU 保护以读为主的链
RCU 最常见的用途之一，是保护以读为主的链（`list.h` 中的 `struct list_head`）。这种方法的一大优势在于，
所有必需的内存排序都由链表宏来提供。本文档描述了若干基于链表的 RCU 用例
在持rcu_read_lock() 遍历链表的同时，写者可以修改该链表。读者保证能看到
他们在获rcu_read_lock() 之前就被加入链表、并且在释放 rcu_read_unlock()
时仍然留在链表上的所有元素。被加入或移出链表的元素可能会被看到，也可能不会被看到如果写者调list_replace_rcu()，读者可能看到旧元素，也可能看到新元素；
但既不会同时看到两者，也不会两者都看不到

### 示例 1：以读为主的链表：延迟销
内核RCU 链表一个广泛使用的场景，是对系统中**所有进*进行无锁遍历`task_struct` `tasks` 字段表示链接所有进程的链表节点。该链表可以与任何链的添加或删除操作并行地进行遍历
链表的遍历通过 `for_each_process()` 完成，其定义如下```

	#define next_task(p) \
		list_entry_rcu((p)->tasks.next, struct task_struct, tasks)

	#define for_each_process(p) \
		for (p = &init_task ; (p = next_task(p)) != &init_task ; )

```
```

	rcu_read_lock();
	for_each_process(p) {
		/* Do something with p */
	}
	rcu_read_unlock();

```
从链表中删除进程的简化且高度内联的代码如下：
```

	void release_task(struct task_struct *p)
	{
		write_lock(&tasklist_lock);
		list_del_rcu(&p->tasks);
		write_unlock(&tasklist_lock);
		call_rcu(&p->rcu, delayed_put_task_struct);
	}

```
当进程退出时，`release_task()` 会在 `tasklist_lock` 写者锁的保护下通过 __exit_signal() __unhash_process() 调用 `list_del_rcu(&p->tasks)`list_del_rcu() 调用将任务从所有任务的链表中移除。`tasklist_lock`
防止并发的链表添删除破坏链表。使`for_each_process()` 的读者并不受
`tasklist_lock` 保护。为了防止读者察觉到链表指针的变化，`task_struct`
对象只有在经过一个或多个宽限期之后才会被释放，这是借助 call_rcu() 实现的，
call_rcu() 通过 put_task_struct_rcu_user() 调用。这种销毁的延迟保证任何正在遍历链表的读者都能看到有效的 `p->tasks.next` 指针并且删除/释放可以与链表遍历并行进行。这种模式也被称*存在锁（existence lock*因为 RCU 会一直推迟调delayed_put_task_struct() 回调函数，直到所有现存的
读者都完成，从而保证相关的 `task_struct` 对象会一直存在，直到所有可能持该对象引用的 RCU 读者都执行完毕

### 示例 2：在锁之外执行读侧操作：无原地更
某些读写锁用例在持有读侧锁时计算一个值，但在释放该锁之后仍继续使用这个值这类用例通常很适合转换RCU。一个典型的例子是网络数据包路由由于数据包路由数据追踪的是计算机之外的设备状态，它有时会包含过期数据因此，一旦路由计算完毕，在数据包传输期间就没有必要保持路由表静止毕竟，你可以随心所欲地让路由表静止，但那并不能阻止外部互联网发生变化，
而真正重要的是外部互联网的状态。此外，路由项通常是被添加或删除，
而不是原地修改。这是一个罕见的例子，光速的有限性和原子的非零尺寸实际上
帮助降低了同步的开销
这类 RCU 用例的一个简单例子可以在系统调用审计支持中找到。例如，一读写锁保护的代码如下```

	static enum audit_state audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		read_lock(&auditsc_lock);
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				read_unlock(&auditsc_lock);
				return state;
			}
		}
		read_unlock(&auditsc_lock);
		return AUDIT_BUILD_CONTEXT;
	}

```
这里链表在锁的保护下进行搜索，但在返回对应的值之前就释放了锁等到这个值被使用时，链表很可能已经被修改。这是合理的，因为如果你正在关闭审计多审计几个系统调用也没关系
```

	static enum audit_state audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		rcu_read_lock();
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry_rcu(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				rcu_read_unlock();
				return state;
			}
		}
		rcu_read_unlock();
		return AUDIT_BUILD_CONTEXT;
	}

```
read_lock() read_unlock() 调用分别变成rcu_read_lock()
rcu_read_unlock()，list_for_each_entry() 变成list_for_each_entry_rcu()*_rcu()** 链表遍历原语增加READ_ONCE()
以及用于检测在 RCU 读侧临界区之外错误使用的诊断检查
更新侧的改动也很直接。在这些简化的代码中，读写锁可能像下面这样用于删除和插入：
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		write_lock(&auditsc_lock);
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				list_del(&e->list);
				write_unlock(&auditsc_lock);
				return 0;
			}
		}
		write_unlock(&auditsc_lock);
		return -EFAULT;		/* No matching rule */
	}

	static inline int audit_add_rule(struct audit_entry *entry,
					 struct list_head *list)
	{
		write_lock(&auditsc_lock);
		if (entry->rule.flags & AUDIT_PREPEND) {
			entry->rule.flags &= ~AUDIT_PREPEND;
			list_add(&entry->list, list);
		} else {
			list_add_tail(&entry->list, list);
		}
		write_unlock(&auditsc_lock);
		return 0;
	}

```
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		/* No need to use the _rcu iterator here, since this is the only
		 * deletion routine. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				list_del_rcu(&e->list);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

	static inline int audit_add_rule(struct audit_entry *entry,
					 struct list_head *list)
	{
		if (entry->rule.flags & AUDIT_PREPEND) {
			entry->rule.flags &= ~AUDIT_PREPEND;
			list_add_rcu(&entry->list, list);
		} else {
			list_add_tail_rcu(&entry->list, list);
		}
		return 0;
	}

```
通常，write_lock() write_unlock() 会被替换spin_lock() spin_unlock()。但在本例中，所有调用者都持有 `audit_filter_mutex`因此不需要额外的锁。于auditsc_lock 可以被移除，因为使用 RCU 消除写者需要排斥读者的需求
list_del()、list_add() list_add_tail() 原语被替换为
list_del_rcu()、list_add_rcu() list_add_tail_rcu()**_rcu()** 链表操作原语增加了在弱内存序 CPU 上所需的内存屏障list_del_rcu() 原语省略了指针毒化调试辅助代码，否则会导致并发读彻底失败
因此，当读者能够容忍过期数据、且表项只是被添加或删除而不进行原地修改时，
使用 RCU 就非常容易！


### 示例 3：处理原地更
系统调用审计代码并不原地更新审计规则。不过，如果它要这么做，用于实现此目的的
读写锁代码可能如下所示（假设只更`field_count`，否则新增的字段会……）```

	static inline int audit_upd_rule(struct audit_rule *rule,
					 struct list_head *list,
					 __u32 newaction,
					 __u32 newfield_count)
	{
		struct audit_entry *e;
		struct audit_entry *ne;

		write_lock(&auditsc_lock);
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				e->rule.action = newaction;
				e->rule.field_count = newfield_count;
				write_unlock(&auditsc_lock);
				return 0;
			}
		}
		write_unlock(&auditsc_lock);
		return -EFAULT;		/* No matching rule */
	}

```
RCU 版本创建一个副本，更新该副本，然后用新更新的表项替换旧表项。这一连串动作—在制作副本以执行更新时允许并发读取——正RCU（read-copy update，读-复制-更新名称的由来
```

	static inline int audit_upd_rule(struct audit_rule *rule,
					 struct list_head *list,
					 __u32 newaction,
					 __u32 newfield_count)
	{
		struct audit_entry *e;
		struct audit_entry *ne;

		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				ne = kmalloc(sizeof(*entry), GFP_ATOMIC);
				if (ne == NULL)
					return -ENOMEM;
				audit_copy_rule(&ne->rule, &e->rule);
				ne->rule.action = newaction;
				ne->rule.field_count = newfield_count;
				list_replace_rcu(&e->list, &ne->list);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

```
同样，这假设调用者持`audit_filter_mutex`。通常，在这种代码中写者锁会变成自旋锁
update_lsm_rule() 做的事情非常类似，如果想看真正的 Linux 内核代码可以参考它
这一模式的另一个用法可以在 openvswitch 驱动 `ct_limit_set()` 中的*连接跟踪
代码里找到。该表保存连接跟踪表项，并对最大表项数设有上限。每zone 有一个这样的
表，因此每个 zone 有一*限制（limit*。zone 通过哈希表映射到它们的限制，
哈希链使RCU 管理hlist。当设置新的限制时，会分配一个新的限制对象，
并调`ct_limit_set()` 使用 list_replace_rcu() 用新限制对象替换旧的限制对象旧的限制对象随后在宽限期之后通过 kfree_rcu() 释放

### 示例 4：消除过期数
上面的审计例子容忍过期数据，大多数追踪外部状态的算法也是如此毕竟，从外部状态变化到 Linux 察觉到这一变化之间存在延迟，因此如前所述，
少量的、由 RCU 引入的额外过期通常不成问题
然而，存在许多无法容忍过期数据的例子。Linux 内核中的一个例子是 System V IPC
（参ipc/shm.c 中的 shm_lock() 函数）。该代码在每表项自旋锁下检查一**deleted（已删除*标志，如果该**deleted**标志被置位，就假装该表项不存在为了让这起作用，搜索函数必须在持有每表项自旋锁的情况下返回，正如 shm_lock()
实际所做的那样

快速测验：
	要让 deleted 标志技术起作用，为什么必须在从搜索函数返回时持有每表项的锁？

快速测验答<quick_quiz_answer>

如果系统调用审计模块将来需要拒绝过期数据，一种实现方式是给审计表增加一`deleted` 标志和一`lock` 自旋锁，如下代码所示：
```

	static struct audit_entry *audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		rcu_read_lock();
		list_for_each_entry_rcu(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				spin_lock(&e->lock);
				if (e->deleted) {
					spin_unlock(&e->lock);
					rcu_read_unlock();
					return NULL;
				}
				rcu_read_unlock();
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				/* As long as e->lock is held, e is valid and
				 * its value is not stale */
				return e;
			}
		}
		rcu_read_unlock();
		return NULL;
	}

```
`audit_del_rule()` 函数需要在每表项锁下设`deleted` 标志，如下代码所示：
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		/* No need to use the _rcu iterator here, since this
		 * is the only deletion routine. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				spin_lock(&e->lock);
				list_del_rcu(&e->list);
				e->deleted = 1;
				spin_unlock(&e->lock);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

```
这也假设调用者持`audit_filter_mutex`
注意，本例假设表项只会被添加和删除。要正确处理 audit_upd_rule() 执行原地更新，还需要额外的机制。一方面，audit_upd_rule() 在执list_replace_rcu() 时需要同时持有旧`audit_entry` 及其替换项的锁

### 示例 5：跳过过期对
对某些用例而言，可以通过在读侧链表遍历期间跳过过期对象来提升读者性能这里的过期对象是指那些将在一个或多个宽限期之后被移除并销毁的对象。timerfd
子系统中可以找到这样一个例子。当 `CLOCK_REALTIME` 时钟被重新编程时
（例如由于设置了系统时间），所有依赖于此时钟的已编`timerfds` 都会被触发，
等待它们的进程会在预定到期时间之前被唤醒。为便于实现，所有这些定时器在通过
如下代码建立时都会被加入一个由 RCU 管理`cancel_list````

	static void timerfd_setup_cancel(struct timerfd_ctx *ctx, int flags)
	{
		spin_lock(&ctx->cancel_lock);
		if ((ctx->clockid == CLOCK_REALTIME ||
		     ctx->clockid == CLOCK_REALTIME_ALARM) &&
		    (flags & TFD_TIMER_ABSTIME) && (flags & TFD_TIMER_CANCEL_ON_SET)) {
			if (!ctx->might_cancel) {
				ctx->might_cancel = true;
				spin_lock(&cancel_lock);
				list_add_rcu(&ctx->clist, &cancel_list);
				spin_unlock(&cancel_lock);
			}
		} else {
			__timerfd_remove_cancel(ctx);
		}
		spin_unlock(&ctx->cancel_lock);
	}

```
当一timerfd 被释放（fd 被关闭）时，`might_cancel` 标志被清除，
对象`cancel_list` 中移除并销毁，如下简化且内联的代码所示：
```

	int timerfd_release(struct inode *inode, struct file *file)
	{
		struct timerfd_ctx *ctx = file->private_data;

		spin_lock(&ctx->cancel_lock);
		if (ctx->might_cancel) {
			ctx->might_cancel = false;
			spin_lock(&cancel_lock);
			list_del_rcu(&ctx->clist);
			spin_unlock(&cancel_lock);
		}
		spin_unlock(&ctx->cancel_lock);

		if (isalarm(ctx))
			alarm_cancel(&ctx->t.alarm);
		else
			hrtimer_cancel(&ctx->t.tmr);
		kfree_rcu(ctx, rcu);
		return 0;
	}

```
如果设置`CLOCK_REALTIME` 时钟（例如由时间服务器设置），hrtimer 框架会调`timerfd_clock_was_set()`，它遍历 `cancel_list` 并唤醒等待在timerfd 上的进程在遍`cancel_list` 时，会查`might_cancel` 标志以跳过过期的对象如下代码所示：
```

	void timerfd_clock_was_set(void)
	{
		ktime_t moffs = ktime_mono_to_real(0);
		struct timerfd_ctx *ctx;
		unsigned long flags;

		rcu_read_lock();
		list_for_each_entry_rcu(ctx, &cancel_list, clist) {
			if (!ctx->might_cancel)
				continue;
			spin_lock_irqsave(&ctx->wqh.lock, flags);
			if (ctx->moffs != moffs) {
				ctx->moffs = KTIME_MAX;
				ctx->ticks++;
				wake_up_locked_poll(&ctx->wqh, EPOLLIN);
			}
			spin_unlock_irqrestore(&ctx->wqh.lock, flags);
		}
		rcu_read_unlock();
	}

```
关键在于，由于对 `cancel_list` RCU 保护遍历与对象的添加和移除是并发进行的，
有时遍历会访问到已经从链表中移除的对象。在本例中，使用一个标志来跳过这类对象

### 总结

能够容忍过期数据的、以读为主的、基于链表的数据结构，最适合使用 RCU最简单的情况是表项被添加或删除（或原地原子修改），但非原子的原地修改可以
通过制作副本、更新副本、然后用副本替换原对象来处理。如果无法容忍过期数据，
则可以结合每表项自旋锁使用一**deleted** 标志，以允许搜索函数拒绝
新删除的数据

快速测验答案：
	要让 deleted 标志技术起作用，为什么必须在从搜索函数返回时持有每表项的锁？

	如果搜索函数在返回之前释放了每表项锁，那么调用者无论如何都在处理过期数据	如果处理过期数据确实可以接受，那么你不需**deleted** 标志。如果处理过期数	确实是个问题，那么你需要在使用所返回值的全部代码范围内持有每表项锁
返回快速测<quick_quiz>
