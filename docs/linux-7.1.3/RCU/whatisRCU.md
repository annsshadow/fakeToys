## What is RCU?  --  "Read, Copy, Update"


请注意，"What is RCU" LWN 系列是学RCU 的绝佳起点：

| 1.	What is RCU, Fundamentally?  https://lwn.net/Articles/262464/
| 2.	What is RCU? Part 2: Usage   https://lwn.net/Articles/263130/
| 3.	RCU part 3: the RCU API      https://lwn.net/Articles/264090/
| 4.	The RCU API, 2010 Edition    https://lwn.net/Articles/418853/
| 	2010 Big API Table           https://lwn.net/Articles/419086/
| 5.	The RCU API, 2014 Edition    https://lwn.net/Articles/609904/
|	2014 Big API Table           https://lwn.net/Articles/609973/
| 6.	The RCU API, 2019 Edition    https://lwn.net/Articles/777036/
|	2019 Big API Table           https://lwn.net/Articles/777165/
| 7.	The RCU API, 2024 Edition    https://lwn.net/Articles/988638/
|       2024 Background Information  https://lwn.net/Articles/988641/
|	2024 Big API Table           https://lwn.net/Articles/988666/

对于偏好视频的读者：

| 1.	Unraveling RCU Mysteries: Fundamentals          https://www.linuxfoundation.org/webinars/unraveling-rcu-usage-mysteries
| 2.	Unraveling RCU Mysteries: Additional Use Cases  https://www.linuxfoundation.org/webinars/unraveling-rcu-usage-mysteries-additional-use-cases


什么是 RCU

RCU 是一种在 2.5 开发周期中加入 Linux 内核的同步机制，针对"读多"场景进行了优化。尽RCU 实际上相当简单，但要有效使用它，你需要以不同的方式思考你的代码。问题的另一部分在于一种错误假设，即存在描述和使用 RCU 唯一正确方式"。相反，经验表明，不同的人必须走不同的路径，才能达成RCU 的理解，这取决于他们的经验和用例。本文档提供了如下几条不同的路径

1.	RCU 概述 <1_whatisRCU>

2.	RCU 的核API 是什么？ <2_whatisRCU>

3.	核心 RCU API 的一些示例用<3_whatisRCU>

4.	如果我的更新线程不能阻塞怎么办？ <4_whatisRCU>

5.	RCU 的一些简单实<5_whatisRCU>

6.	与读写锁定的类比 <6_whatisRCU>

7.	与引用计数的类比 <7_whatisRCU>

8.	RCU API 完整列表 <8_whatisRCU>

9.	快速测验答<9_whatisRCU>

偏好从概念性概述入手的人应重点关注1 节，不过大多数读者在某个时刻阅读本节都会有所收获。偏好从可以试验API 入手的人应重点关注第 2 节。偏好从示例用法入手的人应重点关注第 3 节和4 节。需要理RCU 实现的人应重点关注第 5 节，然后再深入内核源代码。最擅长通过类比来思考的人应重点关注6 节和7 节。第 8 节作docbook API 文档的索引，9 节是传统的答案密钥

因此，从对你和你的学习习惯最有意义的节开始。如果你想知道关于一切的的一切，尽管通读全文——但如果你真的是这种人，你早已翻阅过源代码，因而根本不需要本文档-)


### 1.  RCU 概述


RCU 背后的基本思想是将更新拆分移除"回收"两个阶段。移除阶段移除数据结构中对数据项的引用（可能通过将它们替换为这些数据项的新版本的引用来实现），并且可以与读者并发运行。移除阶段能够与读者并发运行的原因是，现代 CPU 的语义保证读者将看到数据结构的旧版本或新版本，而不会看到部分更新的引用。回收阶段完成回收（例如释放）在移除阶段从数据结构中移除的数据项的工作。由于回收数据项会干扰任何正在并发引用这些数据项的读者，回收阶段必须等到读者不再持有对这些数据项的引用后才能开始

将更新拆分为移除和回收两个阶段，使得更新者可以立即执行移除阶段，并将回收阶段推迟到移除阶段期间活跃的所有读者都完成之后，或者通过阻塞直到它们完成，或者通过注册一个在它们完成之后被调用的回调函数。只需考虑在移除阶段期间活跃的读者，因为任何在移除阶段之后开始的读者都将无法获得对已移除数据项的引用，因此不会被回收阶段干扰

因此典型RCU 更新序列大致如下

a.	移除指向某个数据结构的指针，使得后续的读者无法获得对它的引用

b.	等待所有先前的读者完成其 RCU 读端临界区

c.	此时，不可能有任何持有对该数据结构引用的读者，因此现在可以安全地回收它（例kfree()）

上述步骤 (b) RCU 延迟销毁背后核心思想的体现。能够等待所有读者完成，使得 RCU 读者可以使用更轻量级的同步，在某些情况下，完全不需要任何同步。相比之下，在更传统的基于锁的方案中，读者必须使用重量级同步，以防止更新者将数据结构从他们脚下删除。这是因为基于锁的更新者通常在原地更新数据项，因此必须排除读者。相比之下，基于 RCU 的更新者通常利用这样一个事实：在现CPU 上，对单个对齐指针的写入是原子的，从而可以在不干扰读者的情况下，原子地插入、移除和替换链表结构中的数据项。并发的 RCU 读者可以继续访问旧版本，并且可以省去在当今 SMP 计算机系统上代价高昂的原子操作、内存屏障和通信缓存未命中，即便在没有锁竞争的情况下也是如此

在上面所示的三步流程中，更新者同时执行移除和回收步骤，但让一个完全不同的线程来执行回收通常很有帮助，Linux 内核的目录项缓存（dcache）实际上就是这种情况。即使同一个线程执行更新步骤（上面的步(a)）和回收步骤（上面的步骤 (c)），将二者分开思考通常也很有帮助。例如，RCU 读者和更新者根本不需要通信，但 RCU 在读者和回收者之间提供了隐式的低开销通信，即上面的步(b)

那么，既然读者没有执行任何同步操作，回收者到底怎么知道读者何时完成呢？请继续阅读，了RCU API 如何让这一切变得简单


### 2.  RCU 的核API 是什么？


核心 RCU API 非常小：

a.	rcu_read_lock()
b.	rcu_read_unlock()
c.	synchronize_rcu() / call_rcu()
d.	rcu_assign_pointer()
e.	rcu_dereference()

RCU API 还有许多其他成员，但其余成员都可以用这五个来表示，不过大多数实现转而用 call_rcu() 回调 API 来表synchronize_rcu()

下面描述这五个核RCU API，另18 个稍后列举。更多信息请参阅内核 docbook 文档，或直接查看函数头注释

##### rcu_read_lock()

	void rcu_read_lock(void);

	这一时态原语由读者用来告知回收者，该读者正在进入一RCU 读端临界区。在 RCU 读端临界区内阻塞是非法的，不过使CONFIG_PREEMPT_RCU 构建的内核可以抢RCU 读端临界区。在 RCU 读端临界区内访问的任RCU 保护的数据结构，都保证在该临界区的整个持续期间保持未被回收。引用计数可以与 RCU 结合使用，以维护对数据结构的更长期引用

	请注意，任何禁用底半部、抢占或中断的操作，也同样进入了一RCU 读端临界区。获取自旋锁也同样进入一RCU 读端临界区，即便是那些不禁用抢占的自旋锁也是如此，在使用 CONFIG_PREEMPT_RT=y 构建的内核中就是这种情况。睡眠锁 **并不** 进入 RCU 读端临界区

##### rcu_read_unlock()

	void rcu_read_unlock(void);

	这一时态原语由读者用来告知回收者，该读者正在退出一RCU 读端临界区。任何启用底半部、抢占或中断的操作，也同样退出了一RCU 读端临界区。释放自旋锁也同样退出了一RCU 读端临界区

	请注意，RCU 读端临界区可以是嵌套的和/或重叠的

##### synchronize_rcu()

	void synchronize_rcu(void);

	这一时态原语标记了更新者代码的结束和回收者代码的开始。它通过阻塞，直到所CPU 上所有预先存在的 RCU 读端临界区都已完成来实现这一点。请注意，synchronize_rcu() **不一* 会等待任何后续的 RCU 读端临界区完成。例如，考虑下面的代
```

	         CPU 0                  CPU 1                 CPU 2
	     ----------------- ------------------------- ---------------
	 1.  rcu_read_lock()
	 2.                    enters synchronize_rcu()
	 3.                                               rcu_read_lock()
	 4.  rcu_read_unlock()
	 5.                     exits synchronize_rcu()
	 6.                                              rcu_read_unlock()

	To reiterate, synchronize_rcu() waits only for ongoing RCU
	read-side critical sections to complete, not necessarily for
	any that begin after synchronize_rcu() is invoked.

	Of course, synchronize_rcu() does not necessarily return
	**immediately** after the last pre-existing RCU read-side critical
	section completes.  For one thing, there might well be scheduling
	delays.  For another thing, many RCU implementations process
	requests in batches in order to improve efficiencies, which can
	further delay synchronize_rcu().

	Since synchronize_rcu() is the API that must figure out when
	readers are done, its implementation is key to RCU.  For RCU
	to be useful in all but the most read-intensive situations,
	synchronize_rcu()'s overhead must also be quite small.

	The call_rcu() API is an asynchronous callback form of
	synchronize_rcu(), and is described in more detail in a later
	section.  Instead of blocking, it registers a function and
	argument which are invoked after all ongoing RCU read-side
	critical sections have completed.  This callback variant is
	particularly useful in situations where it is illegal to block
	or where update-side performance is critically important.

	However, the call_rcu() API should not be used lightly, as use
	of the synchronize_rcu() API generally results in simpler code.
	In addition, the synchronize_rcu() API has the nice property
	of automatically limiting update rate should grace periods
	be delayed.  This property results in system resilience in face
	of denial-of-service attacks.  Code using call_rcu() should limit
	update rate in order to gain this same sort of resilience.  See
	checklist.rst for some approaches to limiting the update rate.

```

##### rcu_assign_pointer()

	void rcu_assign_pointer(p, typeof(p) v);

	是的，rcu_assign_pointer() **确实** 是作为一个宏实现的，尽管如果能以这种方式声明一个函数会很酷。（并且曾经有过关于C 语言添加重载函数的讨论，所以谁知道呢？

	更新者使用这一空间宏来为受 RCU 保护的指针赋一个新值，以便将值的变化安全地从更新者传达给读者。这是一个空间（相对于时态）宏。它不会求值为一个右值，但它确实提供了给定编译器CPU 架构所需的任何编译器指令和内存屏障指令。它的排序特性相当于一store-release（存释放）操作，也就是说，用于初始化该结构的任何先前的加载和存储，都被排序在该结构发布指针的存储之前

	也许同样重要的是，rcu_assign_pointer() 用于记录）哪些指针受 RCU 保护，以及（2）给定结构对其他 CPU 变得可访问的时间点。话虽如此，rcu_assign_pointer() 最经常是通过 _rcu 链表操作原语（例list_add_rcu()）间接使用的

##### rcu_dereference()

	typeof(p) rcu_dereference(p);

	rcu_assign_pointer() 一样，rcu_dereference() 必须作为一个宏来实现

	读者使用空间宏 rcu_dereference() 来获取一个受 RCU 保护的指针，它返回一个随后可以安全解引用的指针值。请注意，rcu_dereference() 实际上并不解引用该指针，相反，它保护该指针以便后续解引用。它还为给定CPU 架构执行任何所需的内存屏障指令。目前，只有 Alpha 需要在 rcu_dereference() 内部使用内存屏障——在其他 CPU 上，它被编译为一volatile 加载。然而，没有主流 C 编译器尊重地址依赖，因rcu_dereference() 使用 volatile 强制转换，结rcu_dereference.rst 中列出的编码准则，可以防止当前编译器破坏这些依赖关系

	常见的编码实践是使用 rcu_dereference() 将受 RCU 保护的指针复制到一个局部变量，然后解引
```

		p = rcu_dereference(head.next);
		return p->data;

	However, in this case, one could just as easily combine these
	into one statement::

		return rcu_dereference(head.next)->data;

	If you are going to be fetching multiple fields from the
	RCU-protected structure, using the local variable is of
	course preferred.  Repeated rcu_dereference() calls look
	ugly, do not guarantee that the same pointer will be returned
	if an update happened while in the critical section, and incur
	unnecessary overhead on Alpha CPUs.

	Note that the value returned by rcu_dereference() is valid
	only within the enclosing RCU read-side critical section [1]_.
	For example, the following is **not** legal::

		rcu_read_lock();
		p = rcu_dereference(head.next);
		rcu_read_unlock();
		x = p->address;	/* BUG!!! */
		rcu_read_lock();
		y = p->data;	/* BUG!!! */
		rcu_read_unlock();

	Holding a reference from one RCU read-side critical section
	to another is just as illegal as holding a reference from
	one lock-based critical section to another!  Similarly,
	using a reference outside of the critical section in which
	it was acquired is just as illegal as doing so with normal
	locking.

	As with rcu_assign_pointer(), an important function of
	rcu_dereference() is to document which pointers are protected by
	RCU, in particular, flagging a pointer that is subject to changing
	at any time, including immediately after the rcu_dereference().
	And, again like rcu_assign_pointer(), rcu_dereference() is
	typically used indirectly, via the _rcu list-manipulation
	primitives, such as list_for_each_entry_rcu() [2]_.

```
	RCU 读端临界区保护，只要该用法受到更新侧代码所获取锁的保护即可。这一变体避免了在使用（例如）没有 rcu_read_lock() 保护rcu_dereference() 时会发生lockdep 警告
	使用 rcu_dereference_protected() 还有一个优点，即允rcu_dereference() 必须禁止的编译器优化。rcu_dereference_protected() 变体接受一lockdep 表达式，用以指示调用者必须获取哪些锁。如果没有提供所指明的保护，就会发出一lockdep splat。更多细节和示例用法请参Design/Requirements/Requirements.rst 以及API 的代码注释

	如果某个指针既被更新侧代码使用，也被 RCU 读者使用，那么可以向它的参数列表中添加一个额外的 lockdep 表达式。例如，给定一个额外的 "lock_is_held(&mylock)" 参数，RCU lockdep 代码将仅在该实例RCU 读端临界区之外且没有 mylock 保护的情况下被调用时才会报警

下图展示了每API 如何在读者、更新者和回收者之间进行通信
```


	    rcu_assign_pointer()
	                            +--------+
	    +---------------------->| reader |---------+
	    |                       +--------+         |
	    |                           |              |
	    |                           |              | Protect:
	    |                           |              | rcu_read_lock()
	    |                           |              | rcu_read_unlock()
	    |        rcu_dereference()  |              |
	    +---------+                 |              |
	    | updater |<----------------+              |
	    +---------+                                V
	    |                                    +-----------+
	    +----------------------------------->| reclaimer |
	                                         +-----------+
	      Defer:
	      synchronize_rcu() & call_rcu()


```
RCU 基础设施观察 rcu_read_lock()、rcu_read_unlock()、synchronize_rcu() call_rcu() 调用的时态序列，以确定（1）synchronize_rcu() 调用何时可以返回给其调用者，以及）call_rcu() 回调何时可以被调用。RCU 基础设施的高效实现大量使用批处理，以便将开销分摊到相API 的多次使用之上。rcu_assign_pointer() rcu_dereference() 调用通过对相关受 RCU 保护的指针进行存储和加载来传达空间变化

Linux 内核中至少有三种 RCU 用法风格。上图展示了最常见的一种。在更新者一侧，所使用rcu_assign_pointer()、synchronize_rcu() call_rcu() 原语对这三种风格都是相同的。然而对于保护（读者一侧），所使用的原语因风格而异

a.	rcu_read_lock() / rcu_read_unlock()
	rcu_dereference()

b.	rcu_read_lock_bh() / rcu_read_unlock_bh()
	local_bh_disable() / local_bh_enable()
	rcu_dereference_bh()

c.	rcu_read_lock_sched() / rcu_read_unlock_sched()
	preempt_disable() / preempt_enable()
	local_irq_save() / local_irq_restore()
	hardirq enter / hardirq exit
	NMI enter / NMI exit
	rcu_dereference_sched()

这三种风格的使用方式如下

a.	应用于普通数据结构的 RCU

b.	应用于可能遭受远程拒绝服务攻击的网络数据结构RCU

c.	应用于调度器以及中断/NMI 处理程序任务RCU

同样，大多数用法属于 (a)b) (c) 的情况对于专门用途很重要，但相对少见。SRCU、RCU-Tasks、RCU-Tasks-Rude RCU-Tasks-Trace 在其各种原语之间具有类似的关系


### 3.  核心 RCU API 的一些示例用


本节展示了一个使用核RCU API 来保护指向动态分配结构的全局指针的简单示例。更典型RCU 用法可以listRCU.rst NMI-RCU.rst 中找到
```

	struct foo {
		int a;
		char b;
		long c;
	};
	DEFINE_SPINLOCK(foo_mutex);

	struct foo __rcu *gbl_foo;

	/*
	 * Create a new struct foo that is the same as the one currently
	 * pointed to by gbl_foo, except that field "a" is replaced
	 * with "new_a".  Points gbl_foo to the new structure, and
	 * frees up the old structure after a grace period.
	 *
	 * Uses rcu_assign_pointer() to ensure that concurrent readers
	 * see the initialized version of the new structure.
	 *
	 * Uses synchronize_rcu() to ensure that any readers that might
	 * have references to the old structure complete before freeing
	 * the old structure.
	 */
	void foo_update_a(int new_a)
	{
		struct foo *new_fp;
		struct foo *old_fp;

		new_fp = kmalloc(sizeof(*new_fp), GFP_KERNEL);
		spin_lock(&foo_mutex);
		old_fp = rcu_dereference_protected(gbl_foo, lockdep_is_held(&foo_mutex));
		*new_fp = *old_fp;
		new_fp->a = new_a;
		rcu_assign_pointer(gbl_foo, new_fp);
		spin_unlock(&foo_mutex);
		synchronize_rcu();
		kfree(old_fp);
	}

	/*
	 * Return the value of field "a" of the current gbl_foo
	 * structure.  Use rcu_read_lock() and rcu_read_unlock()
	 * to ensure that the structure does not get deleted out
	 * from under us, and use rcu_dereference() to ensure that
	 * we see the initialized version of the structure (important
	 * for DEC Alpha and for people reading the code).
	 */
	int foo_get_a(void)
	{
		int retval;

		rcu_read_lock();
		retval = rcu_dereference(gbl_foo)->a;
		rcu_read_unlock();
		return retval;
	}

```
综上所述：

- 使用 rcu_read_lock() rcu_read_unlock() 来保RCU 读端临界区

- RCU 读端临界区内，使rcu_dereference() 来解引用RCU 保护的指针

- 使用某种稳妥的设计（例如锁或信号量）来防止并发更新相互干扰

- 使用 rcu_assign_pointer() 来更新受 RCU 保护的指针。这一原语保护并发读者免受更新者的影响*并不**保护并发更新彼此之间不受影响！因此你仍然需要使用锁（或类似机制）来防止并发rcu_assign_pointer() 原语相互干扰

- 在从RCU 保护的数据结构中移除数据元素**之后**，但在回释放该数据元*之前**，使synchronize_rcu() 来等待所有可能正在引用该数据项的所RCU 读端临界区完成

更多使用 RCU 时需要遵循的规则，请参阅 checklist.rst。再次说明，更典型的 RCU 用法可以listRCU.rst NMI-RCU.rst 中找到


### 4.  如果我的更新线程不能阻塞怎么办？


在上面的示例中，foo_update_a() 会阻塞，直到宽限期过去。这非常简单，但在某些情况下，人们无法承受等待这么久——可能还有其他高优先级的工作要做

在这种情况下，应该使call_rcu() 而不synchronize_rcu()
```

	void call_rcu(struct rcu_head *head, rcu_callback_t func);

```
该函数在宽限期过去之后调func(head)。这一调用可能发生在软中断或进程上下文中，因此该函数不允许阻塞。foo 结构体需
```

	struct foo {
		int a;
		char b;
		long c;
		struct rcu_head rcu;
	};

```
```

	/*
	 * Create a new struct foo that is the same as the one currently
	 * pointed to by gbl_foo, except that field "a" is replaced
	 * with "new_a".  Points gbl_foo to the new structure, and
	 * frees up the old structure after a grace period.
	 *
	 * Uses rcu_assign_pointer() to ensure that concurrent readers
	 * see the initialized version of the new structure.
	 *
	 * Uses call_rcu() to ensure that any readers that might have
	 * references to the old structure complete before freeing the
	 * old structure.
	 */
	void foo_update_a(int new_a)
	{
		struct foo *new_fp;
		struct foo *old_fp;

		new_fp = kmalloc(sizeof(*new_fp), GFP_KERNEL);
		spin_lock(&foo_mutex);
		old_fp = rcu_dereference_protected(gbl_foo, lockdep_is_held(&foo_mutex));
		*new_fp = *old_fp;
		new_fp->a = new_a;
		rcu_assign_pointer(gbl_foo, new_fp);
		spin_unlock(&foo_mutex);
		call_rcu(&old_fp->rcu, foo_reclaim);
	}

```
```

	void foo_reclaim(struct rcu_head *rp)
	{
		struct foo *fp = container_of(rp, struct foo, rcu);

		foo_cleanup(fp->a);

		kfree(fp);
	}

```
container_of() 原语是一个宏，给定一个指向结构体内部的指针、结构体的类型以及结构体内被指向的字段，它返回指向该结构体起始位置的指针

使用 call_rcu() 允许 foo_update_a() 的调用者立即重新获得控制权，而不必进一步担心新更新元素的旧版本。它还清楚地展示RCU 在更新者（foo_update_a()）和回收者（foo_reclaim()）之间的区别

建议总结与上一节相同，只是我们现在使用 call_rcu() 而不synchronize_rcu()

- 在从RCU 保护的数据结构中移除数据元素**之后**使用 call_rcu()，以注册一个回调函数，该回调函数将在所有可能正在引用该数据项的所RCU 读端临界区完成之后被调用

如果 call_rcu() 的回调所做的无非是对该结构调kfree()，那么可以使kfree_rcu() 来代call_rcu()
```

	kfree_rcu(old_fp, rcu);

```
如果允许偶尔睡眠，则可以使用单参数形式，从而从 struct foo 中省rcu_head 结构

	kfree_rcu_mightsleep(old_fp);

这一变体几乎从不阻塞，但可能因内存分配失败而通过调用 synchronize_rcu() 来阻塞

同样，请参阅 checklist.rst 了解使用 RCU 的其他规则


### 5.  RCU 有哪些简单的实现


RCU 的好处之一在于，它有极其简单的"玩具"实现，是理解 Linux 内核中生产级实现的一个良好第一步。本节给RCU 的两个这样的"玩具"实现，一个基于熟悉的锁定原语，另一个更接近经典"RCU。二者都过于简单，无法用于真实世界，既缺乏功能也缺乏性能。不过，它们有助于体RCU 的工作方式。有关生产级实现，请参阅 kernel/rcu/update.c，并参阅

	https://docs.google.com/document/d/1X0lThx8OK0ZgLMqVoXiR4ZrGURHrXK6NyLRbeXe3Xac/edit

以了解描Linux 内核 RCU 实现的论文。OLS'01 OLS'02 论文是很好的入门，而学位论文提供了截至 2004 年初当前实现的更多细节


##### 5A.  "玩具"实现 #1：锁


本节给出一个基于熟悉锁定原语的"玩具"RCU 实现。它的开销使得它无法用于实际场景，缺乏可扩展性也是一样。它也不适合实时使用，因为它允许调度延迟从一个读端临界区"渗到另一个。它还假设了递归的读写锁：如果你在非递归锁上尝试这样做，并且允许嵌套rcu_read_lock() 调用，就可能发生死锁

不过，它可能是最容易与之建立联系的实现了，因此是一个良好的起点
```

	static DEFINE_RWLOCK(rcu_gp_mutex);

	void rcu_read_lock(void)
	{
		read_lock(&rcu_gp_mutex);
	}

	void rcu_read_unlock(void)
	{
		read_unlock(&rcu_gp_mutex);
	}

	void synchronize_rcu(void)
	{
		write_lock(&rcu_gp_mutex);
		smp_mb__after_spinlock();
		write_unlock(&rcu_gp_mutex);
	}

```
（你可以忽略 rcu_assign_pointer() rcu_dereference() 而不会错过太多。不过这里还是给出简化版本。而且无论你做什么，
```

	#define rcu_assign_pointer(p, v) \
	({ \
		smp_store_release(&(p), (v)); \
	})

	#define rcu_dereference(p) \
	({ \
		typeof(p) _________p1 = READ_ONCE(p); \
		(_________p1); \
	})


```
rcu_read_lock() rcu_read_unlock() 原语读取-获取并释放一个全局读写锁。synchronize_rcu() 原语写入-获取同一个锁，然后释放它。这意味着，一synchronize_rcu() 退出，synchronize_rcu() 被调用之前所有正在进行的 RCU 读端临界区都保证已经完成——否synchronize_rcu() 不可能写获取该锁。smp_mb__after_spinlock() synchronize_rcu() 提升为一个完全内存屏障，以符合以下文档中列出内存屏障保证"

	Design/Requirements/Requirements.rst

由于读写锁可以被递归获取，因此嵌rcu_read_lock() 是可能的。还要注意，rcu_read_lock() 不会死锁（这RCU 的一个重要属性）。原因在于，唯一能够阻塞 rcu_read_lock() 的是 synchronize_rcu()。但 synchronize_rcu() 在持rcu_gp_mutex 时不会获取任何锁，因此不可能形成死锁环


快速测#1
		为什么这个论点很天真？在真实世界Linux 内核中使用该算法时，死锁是如何发生的？又该如何避免这种死锁？

快速测验答<9_whatisRCU>


##### 5B.  "玩具"示例 #2：经RCU


本节给出一个基经典 RCU"玩具"RCU 实现。它在性能上（但仅针对更新）以及诸如热插拔 CPU 和在 CONFIG_PREEMPTION 内核中运行等特性方面也很欠缺。rcu_dereference() rcu_assign_pointer() 的定义与前一节所示相同，因此此处省略
```

	void rcu_read_lock(void) { }

	void rcu_read_unlock(void) { }

	void synchronize_rcu(void)
	{
		int cpu;

		for_each_possible_cpu(cpu)
			run_on(cpu);
	}

```
请注意，rcu_read_lock() rcu_read_unlock() 绝对什么都不做。这是经RCU 在非抢占式内核中的巨大优势：读端开销恰好为零，至少在Alpha CPU 上是如此。而且 rcu_read_lock() 绝对不可能参与死锁环

synchronize_rcu() 的实现只是依次在每个 CPU 上调度自身。run_on() 原语可以利用 sched_setaffinity() 原语直接实现。当然，一个不那么"玩具"的实现会在完成时恢复亲和性，而不是让所有任务都留在最后一CPU 上运行，但当我说"玩具"时，我是*玩具**

那么，这到底是怎么工作的？？？

请记住，RCU 读端临界区内阻塞是非法的。因此，如果某个 CPU 执行了一次上下文切换，我们就知道它必定已经完成了所有先前的 RCU 读端临界区。一*所* CPU 都执行了一次上下文切换，那*所*先前RCU 读端临界区就都已完成了

因此，假设我们从其结构中移除一个数据项，然后调synchronize_rcu()。一synchronize_rcu() 返回，我们就保证没有任何 RCU 读端临界区持有对该数据项的引用，因此我们可以安全地回收它


快速测#2
		给出一个经RCU 读端开销**的例子

快速测验答<9_whatisRCU>


快速测#3
		如果RCU 读端临界区内阻塞是非法的，那么在 CONFIG_PREEMPT_RT 中你到底该怎么做？在那里普通的自旋锁也会阻塞？？？

快速测验答<9_whatisRCU>


### 6.  与读写锁定的类比


尽管 RCU 可以以许多不同的方式使用，但 RCU 的一种非常常见的用法类似于读写锁定。下面的统一 diff 展示RCU 与读写锁定可以有多么密切的关系
```

	@@ -5,5 +5,5 @@ struct el {
	 	int data;
	 	/* Other data fields */
	 };
	-rwlock_t listmutex;
	+spinlock_t listmutex;
	 struct el head;

	@@ -13,15 +14,15 @@
		struct list_head *lp;
		struct el *p;

	-	read_lock(&listmutex);
	-	list_for_each_entry(p, head, lp) {
	+	rcu_read_lock();
	+	list_for_each_entry_rcu(p, head, lp) {
			if (p->key == key) {
				*result = p->data;
	-			read_unlock(&listmutex);
	+			rcu_read_unlock();
				return 1;
			}
		}
	-	read_unlock(&listmutex);
	+	rcu_read_unlock();
		return 0;
	 }

	@@ -29,15 +30,16 @@
	 {
		struct el *p;

	-	write_lock(&listmutex);
	+	spin_lock(&listmutex);
		list_for_each_entry(p, head, lp) {
			if (p->key == key) {
	-			list_del(&p->list);
	-			write_unlock(&listmutex);
	+			list_del_rcu(&p->list);
	+			spin_unlock(&listmutex);
	+			synchronize_rcu();
				kfree(p);
				return 1;
			}
		}
	-	write_unlock(&listmutex);
	+	spin_unlock(&listmutex);
		return 0;
	 }

```
```

 1 struct el {                          1 struct el {
 2   struct list_head list;             2   struct list_head list;
 3   long key;                          3   long key;
 4   spinlock_t mutex;                  4   spinlock_t mutex;
 5   int data;                          5   int data;
 6   /* Other data fields */            6   /* Other data fields */
 7 };                                   7 };
 8 rwlock_t listmutex;                  8 spinlock_t listmutex;
 9 struct el head;                      9 struct el head;

```
```

  1 int search(long key, int *result)    1 int search(long key, int *result)
  2 {                                    2 {
  3   struct list_head *lp;              3   struct list_head *lp;
  4   struct el *p;                      4   struct el *p;
  5                                      5
  6   read_lock(&listmutex);             6   rcu_read_lock();
  7   list_for_each_entry(p, head, lp) { 7   list_for_each_entry_rcu(p, head, lp) {
  8     if (p->key == key) {             8     if (p->key == key) {
  9       *result = p->data;             9       *result = p->data;
 10       read_unlock(&listmutex);      10       rcu_read_unlock();
 11       return 1;                     11       return 1;
 12     }                               12     }
 13   }                                 13   }
 14   read_unlock(&listmutex);          14   rcu_read_unlock();
 15   return 0;                         15   return 0;
 16 }                                   16 }

```
```

  1 int delete(long key)                 1 int delete(long key)
  2 {                                    2 {
  3   struct el *p;                      3   struct el *p;
  4                                      4
  5   write_lock(&listmutex);            5   spin_lock(&listmutex);
  6   list_for_each_entry(p, head, lp) { 6   list_for_each_entry(p, head, lp) {
  7     if (p->key == key) {             7     if (p->key == key) {
  8       list_del(&p->list);            8       list_del_rcu(&p->list);
  9       write_unlock(&listmutex);      9       spin_unlock(&listmutex);
                                        10       synchronize_rcu();
 10       kfree(p);                     11       kfree(p);
 11       return 1;                     12       return 1;
 12     }                               13     }
 13   }                                 14   }
 14   write_unlock(&listmutex);         15   spin_unlock(&listmutex);
 15   return 0;                         16   return 0;
 16 }                                   17 }

```
无论哪种方式，差别都很小。读端锁定转移到 rcu_read_lock() rcu_read_unlock，更新端锁定从读写锁转移到一个简单的自旋锁，并且 kfree() 之前有一synchronize_rcu()

不过，有一个潜在的陷阱：读端和更新的临界区现在可以并发运行。在许多情况下，这不会成为问题，但无论如何都必须仔细检查。例如，如果多个独立的链表更新必须被视为单个原子更新，那么转换为 RCU 将需要特别小心

此外，synchronize_rcu() 的存在意味着 delete() RCU 版本现在可能会阻塞。如果这是个问题，可以使用基于回调、永不阻塞的机制，即 call_rcu() kfree_rcu()，来代替 synchronize_rcu()


### 7.  与引用计数的类比


读写类比（由上一节说明）并不总是思考如何使RCU 的最佳方式。另一个有用的类比是，RCU 视为对受 RCU 保护的一切事物的一种有效引用计数

引用计数通常并不阻止被引用对象的值发生变化，但确实阻止类型发生变化——特别是当该对象的内存被释放并重新分配给其他用途时发生的那种整体类型变更。一旦获得对该对象的类型安全引用，就需要某种其他机制来确保对该对象中数据的一致访问。这可能涉及获取一个自旋锁，但RCU 中，典型的方法是使用具备 SMP 感知的操作（例如 smp_load_acquire()）来执行读取，使用原子读-修改-写操作来执行更新，并提供必要的排序。RCU 提供了许多内嵌了所需操作和排序的支持函数，例如上一节中使用list_for_each_entry_rcu() 宏

对引用计数行为更聚焦的看法是：在 rcu_read_lock() rcu_read_unlock() 之间，使rcu_dereference() 在标记为 `__rcu` 的指针上获取的任何引用，都可以被视为该对象的引用计数被临时增加了。这防止了对象改变类型。这究竟意味着什么，将取决于对该类型对象的一般预期，但它通常包括：自旋锁仍然可以安全加锁、普通引用计数器可以安全操作，以`__rcu` 指针可以安全解引用

人们可能期望在持RCU 引用的对象上看到的一些操作包括：

 - 复制出由对象类型保证稳定的数据
 - 使用 kref_get_unless_zero() 或类似方法获取更长期的引用。当然，这可能会失败
 - 获取对象中的自旋锁，并检查该对象是否仍然是期望的对象，如果是，则自由地操作它

RCU 提供的引用仅防止类型变化这一理解，在使用从标记为 `SLAB_TYPESAFE_BY_RCU` slab 缓存分配的对象时尤为明显。RCU 操作可能产生对来自此类缓存的对象的引用，该对象已被并发释放，并且内存被重新分配给一个完全不同的对象，尽管是同一类型。在这种情况下，RCU 甚至不保护对象的身份免遭改变，只保护其类型。因此找到的对象可能不是期望的那个，但它会是一个可以安全获取引用（以及随后可能获取自旋锁）的对象，从而允许后续代码检查身份是否符合预期。人们很容易想在不先获取引用的情况下直接获取自旋锁，但不幸的是，`SLAB_TYPESAFE_BY_RCU` 对象中的任何自旋锁都必须在每次调kmem_cache_alloc() 之后重新初始化，这使得无引用的自旋锁获取完全不安全。因此，当使`SLAB_TYPESAFE_BY_RCU` 时，要正确地使用引用计数器。如果使refcount_t，则应使用专门的 refcount_{add|inc}_not_zero_acquire() refcount_set_release() API，以确保在验证对象身份和初始化新分配对象时操作顺序的正确性。refcount_{add|inc}_not_zero_acquire() 中的获取栅栏确保身份检查发生在引用计数被获*之后**。refcount_set_release() 应当在一个新分配的对象被完全初始化之后调用，其释放栅栏确保新值在引用计数能被其他用户成功获取**之前**可见。一旦调用了 refcount_set_release()，该对象就应被视为对其他任务可见
（那些愿意在 kmem_cache 构造函数中初始化其锁的人，也可以使用锁定，包括缓存友好的顺序锁。）

对于传统的引用计数——例Linux 中由 kref 库实现的那种——通常会有在对象的最后一个引用被丢弃时运行的代码。对kref，这就是传给 kref_put() 的函数。当使用 RCU 时，这样的终结代码必须等到所有引用该对象`__rcu` 指针都已被更新，并且宽限期已经过去之后才能运行。每一个剩余的对该对象的全局可见指针都必须被视为一个潜在的计数引用，而终结代码通常只在所有那些指针都被更改之后，使用 call_rcu() 来运行

要弄清楚如何在这两个类比之间——将 RCU 视为读写锁，以及RCU 视为引用计数系统——做出选择，反思被保护事物的规模会有所帮助。读写锁类比着眼于更大的多部分对象（例如链表），并展示 RCU 如何在元素被添加进链表以及从链表中移除时促进并发。引用计数类比着眼于单个对象，并考察它们在其所属的整体中如何被安全访问


### 8.  RCU API 完整列表


RCU API 记录Linux 内核源代码中docbook 格式头注释里，但有一份完整的 API 列表会很有帮助，因为似乎无法docbook 中对它们进行分类。以下是按类别排列的列表
```

	list_entry_rcu
	list_entry_lockless
	list_first_entry_rcu
	list_first_or_null_rcu
	list_tail_rcu
	list_next_rcu
	list_next_or_null_rcu
	list_for_each_entry_rcu
	list_for_each_entry_continue_rcu
	list_for_each_entry_from_rcu
	list_for_each_entry_lockless
	hlist_first_rcu
	hlist_next_rcu
	hlist_pprev_rcu
	hlist_for_each_entry_rcu
	hlist_for_each_entry_rcu_notrace
	hlist_for_each_entry_rcu_bh
	hlist_for_each_entry_from_rcu
	hlist_for_each_entry_continue_rcu
	hlist_for_each_entry_continue_rcu_bh
	hlist_nulls_first_rcu
	hlist_nulls_next_rcu
	hlist_nulls_for_each_entry_rcu
	hlist_nulls_for_each_entry_safe
	hlist_bl_first_rcu
	hlist_bl_for_each_entry_rcu

```
```

	rcu_assign_pointer
	rcu_replace_pointer
	INIT_LIST_HEAD_RCU
	list_add_rcu
	list_add_tail_rcu
	list_del_rcu
	list_replace_rcu
	list_splice_init_rcu
	list_splice_tail_init_rcu
	hlist_add_behind_rcu
	hlist_add_before_rcu
	hlist_add_head_rcu
	hlist_add_tail_rcu
	hlist_del_rcu
	hlist_del_init_rcu
	hlist_replace_rcu
	hlist_nulls_del_init_rcu
	hlist_nulls_del_rcu
	hlist_nulls_add_head_rcu
	hlist_nulls_add_tail_rcu
	hlist_nulls_add_fake
	hlists_swap_heads_rcu
	hlist_bl_add_head_rcu
	hlist_bl_del_rcu
	hlist_bl_set_first_rcu

```
```

	Critical sections		Grace period		Barrier

	rcu_read_lock			synchronize_net		rcu_barrier
	rcu_read_unlock			synchronize_rcu
	guard(rcu)()			synchronize_rcu_expedited
	scoped_guard(rcu)		synchronize_rcu_mult
	rcu_dereference			call_rcu
	rcu_dereference_check		call_rcu_hurry
	rcu_dereference_protected	kfree_rcu
	rcu_read_lock_held		kvfree_rcu
	rcu_read_lock_any_held		kfree_rcu_mightsleep
	rcu_pointer_handoff		cond_synchronize_rcu
	unrcu_pointer			cond_synchronize_rcu_full
					cond_synchronize_rcu_expedited
					cond_synchronize_rcu_expedited_full
					get_completed_synchronize_rcu
					get_completed_synchronize_rcu_full
					get_state_synchronize_rcu
					get_state_synchronize_rcu_full
					poll_state_synchronize_rcu
					poll_state_synchronize_rcu_full
					same_state_synchronize_rcu
					same_state_synchronize_rcu_full
					start_poll_synchronize_rcu
					start_poll_synchronize_rcu_full
					start_poll_synchronize_rcu_expedited
					start_poll_synchronize_rcu_expedited_full

```
```

	Critical sections	Grace period		Barrier

	rcu_read_lock_bh	[Same as RCU]		[Same as RCU]
	rcu_read_unlock_bh
	[local_bh_disable]
	[and friends]
	rcu_dereference_bh
	rcu_dereference_bh_check
	rcu_dereference_bh_protected
	rcu_read_lock_bh_held

```
```

	Critical sections	Grace period		Barrier

	rcu_read_lock_sched	[Same as RCU]		[Same as RCU]
	rcu_read_unlock_sched
	[preempt_disable]
	[and friends]
	rcu_read_lock_sched_notrace
	rcu_read_unlock_sched_notrace
	rcu_dereference_sched
	rcu_dereference_sched_check
	rcu_dereference_sched_protected
	rcu_read_lock_sched_held


```
```

	RCU_INIT_POINTER
	RCU_INITIALIZER
	RCU_POINTER_INITIALIZER
	init_rcu_head
	destroy_rcu_head
	init_rcu_head_on_stack
	destroy_rcu_head_on_stack
	SLAB_TYPESAFE_BY_RCU


```
```

	cond_resched_tasks_rcu_qs
	rcu_all_qs
	rcu_softirq_qs_periodic
	rcu_end_inkernel_boot
	rcu_expedite_gp
	rcu_gp_is_expedited
	rcu_unexpedite_gp
	rcu_cpu_stall_reset
	rcu_head_after_call_rcu
	rcu_is_watching


```
```

	rcu_sync_is_idle
	rcu_sync_init
	rcu_sync_enter
	rcu_sync_exit
	rcu_sync_dtor


```
```

	Critical sections	Grace period			Barrier

	N/A			call_rcu_tasks			rcu_barrier_tasks
				synchronize_rcu_tasks


```
```

	Critical sections	Grace period			Barrier

	N/A			synchronize_rcu_tasks_rude	rcu_barrier_tasks_rude
				call_rcu_tasks_rude


```
```

	Critical sections	Grace period			Barrier

	rcu_read_lock_trace	call_rcu_tasks_trace		rcu_barrier_tasks_trace
	rcu_read_unlock_trace	synchronize_rcu_tasks_trace
	guard(rcu_tasks_trace)()
	scoped_guard(rcu_tasks_trace)


```
```
	list_for_each_entry_srcu
	hlist_for_each_entry_srcu


```
```

	Critical sections		Grace period		Barrier

	srcu_read_lock			call_srcu		srcu_barrier
	srcu_read_unlock		synchronize_srcu
	srcu_read_lock_fast		synchronize_srcu_expedited
	srcu_read_unlock_fast		get_state_synchronize_srcu
	srcu_read_lock_nmisafe		start_poll_synchronize_srcu
	srcu_read_unlock_nmisafe	start_poll_synchronize_srcu_expedited
	srcu_read_lock_notrace		poll_state_synchronize_srcu
	srcu_read_unlock_notrace
	srcu_down_read
	srcu_up_read
	srcu_down_read_fast
	srcu_up_read_fast
	guard(srcu)()
	scoped_guard(srcu)
	srcu_read_lock_held
	srcu_dereference
	srcu_dereference_check
	srcu_dereference_notrace
	srcu_read_lock_held


```
```

	DEFINE_SRCU
	DEFINE_STATIC_SRCU
	DEFINE_SRCU_FAST        // for srcu_read_lock_fast() and friends
	DEFINE_STATIC_SRCU_FAST // for srcu_read_lock_fast() and friends
	init_srcu_struct
	init_srcu_struct_fast
	cleanup_srcu_struct
	smp_mb__after_srcu_read_unlock

```
```

	RCU_LOCKDEP_WARN
	rcu_sleep_check

```
```

	rcu_dereference_raw

```
```

	rcu_access_pointer

```
有关更多信息，请参阅源代码中的注释头（或从中生成docbook）

然而，鉴于 Linux 内核中至少有四个系列RCU API，你该如何选择使用哪一个？以下列表可能会有帮助

a.	读者是否需要阻塞？如果是，你需SRCU

b.	读者是否需要阻塞，并且你是在做跟踪（例ftrace BPF）？如果是，你需RCU-tasks、RCU-tasks-rude RCU-tasks-trace

c.	那么 -rt 补丁集呢？如果读者在rt 内核中需要阻塞，你需SRCU。如果读者在 -rt 内核中获取自旋锁时会阻塞，而在rt 内核中不会，则不需SRCU。（-rt 补丁集将自旋锁转变为睡眠锁，因此有了这种区分。）

d.	你是否需要将 NMI 处理程序、hardirq 处理程序，以及禁用了抢占的代码段（无论是通过 preempt_disable()、local_irq_save()、local_bh_disable() 还是其他某种机制）视为显式的 RCU 读者？如果是，RCU-sched 读者是唯一可行的选择，但自大v4.20 起，你可以使用原生的 RCU 更新原语

e.	你是否需RCU 宽限期在一个或多个 CPU 被软中断垄断的情况下也能完成？例如，你的代码是否会受到基于网络的拒绝服务攻击？如果是，你应该跨读者禁用软中断，例如通过使用 rcu_read_lock_bh()。自大约 v4.20 起，你可以使用原生的 RCU 更新原语

f.	你的工作负载是否RCU 的普通使用而言更新过于密集，但又不适合其他同步机制？如果是，考虑 SLAB_TYPESAFE_BY_RCU（它最初名SLAB_DESTROY_BY_RCU）。但请务必小心！

g.	你是否需要在那些深陷空闲循环、在用户态执行的进入或退出期间、或在离CPU 上的 CPU 上，读端临界区也受到尊重？如果是，SRCU RCU Tasks Trace 是唯一可行的选择，其SRCU 在几乎所有情况下都被强烈优先推荐

h.	否则，使RCU

当然，这一切都假设你已经确RCU 确实是你工作的正确工具


### 9.  快速测验答


快速测#1
		为什么这个论点很天真？在真实世界Linux 内核中使用该算法时，死锁是如何发生的？[指的是基于锁玩具"RCU 算法。]

答案
		考虑以下事件序列

  1. CPU 0 获取某个不相关的锁，称之
			"problematic_lock"，通过
			spin_lock_irqsave() 禁用 irq

  2. CPU 1 进入 synchronize_rcu()，写获取
			rcu_gp_mutex銆。

  3. CPU 0 进入 rcu_read_lock()，但必须等待
			因为 CPU 1 持有 rcu_gp_mutex

  4. CPU 1 被中断，而该 irq 处理程序
			试图获取 problematic_lock

		系统现在发生了死锁

		避免这种死锁的一种方法是采用类似CONFIG_PREEMPT_RT 的做法，即所有普通自旋锁都变成阻塞锁，并且所irq 处理程序都在特殊任务的上下文中执行。在这种情况下，在上述第 4 步中，irq 处理程序会阻塞，从而允CPU 1 释放 rcu_gp_mutex，避免死锁

		即使没有死锁，这RCU 实现也允许延迟通过 synchronize_rcu() 从读渗到其他读者。要看出这一点，考虑处于 RCU 读端临界区中的任A（因而读持有 rcu_gp_mutex）、试图写获取 rcu_gp_mutex 而被阻塞的任B，以及试图读获取 rcu_gp_mutex 而在 rcu_read_lock() 中阻塞的任务 C。任A RCU 读端延迟正在拖住任务 C，尽管是通过任务 B 间接做到的

		实时 RCU 实现因此使用了一种基于计数器的方法，其中处于 RCU 读端临界区中的任务不会被执行 synchronize_rcu() 的任务阻塞

回到快速测#1 <quiz_1>

快速测#2
		给出一个经RCU 读端开销**的例子

答案
		设想一个单 CPU 系统，运行非 CONFIG_PREEMPTION 内核，其中路由表由进程上下文代码使用，但可以irq 上下文代码更新（例如，通过一ICMP REDIRECT"包）。通常的处理方式是让进程上下文代码在查找路由表时禁用中断。使RCU 则可以省去这种禁用中断的操作。因此，没有 RCU 时，你要付出禁用中断的代价；而有RCU，你则不需要

		有人可以争辩说，在这种情况下 RCU 的开销相对于单 CPU 的禁用中断方案是负的。其他人可能会争辩说，RCU 的开销仅仅是零，而用零开销RCU 方案取代正开销的禁用中断方案并不构成负开销

		当然，在现实生活中，事情要复杂得多。但即便是一个同步原语开销可能为负的理论可能性，也有些出人意料-)

回到快速测#2 <quiz_2>

快速测#3
		如果RCU 读端临界区内阻塞是非法的，那么在 CONFIG_PREEMPT_RT 中你到底该怎么做？在那里普通的自旋锁也会阻塞？？？

答案
		正如 CONFIG_PREEMPT_RT 允许抢占自旋锁临界区一样，它也允许抢占 RCU 读端临界区。它还允许在 RCU 读端临界区内自旋锁阻塞

		为什么存在这种明显的不一致？因为如果有需要（例如内存短缺时），有可能使用优先级提升来保持 RCU 宽限期较短。相比之下，如果阻塞等待（比如说）网络接收，则无法知道应该提升什么。特别是考虑到我们需要提升的进程很可能是一个刚出去买披萨或什么的活人。而且尽管计算机操作的赶牛棒可能引起严重兴趣，它也可能招致严重反对。此外，计算机怎么知道那个人去了哪家披萨店？？

回到快速测#3 <quiz_3>

致谢

感谢那些帮助使本文具备可读性的人员，包Jon Walpole、Josh Triplett、Serge Hallyn、Suzanne Wood Alan Stern


更多信息，请参阅 http://www.rdrop.com/users/paulmck/RCU
