
## tip 树手册

### 什么是 tip 树？

tip 树（tip tree）是若干子系统与开发领域的集合。tip 树既是一个直接的开发树，
也是若干子维护者（sub-maintainer）树的聚合树。tip 树的 gitweb URL 为：
https://git.kernel.org/pub/scm/linux/kernel/git/tip/tip.git

tip 树包含以下子系统：

   - **x86 架构**

     x86 架构的开发在 tip 树中进行，但 x86 KVM 和 XEN 的特定部分除外，它们由
     相应的子系统维护，并直接从那里合并到主线（mainline）。在 x86 特定的 KVM 和
     XEN 补丁上 Cc x86 维护者仍然是好习惯。

     除了整体的 x86 维护者外，一些 x86 子系统还有它们自己的维护者。即使 MAINTAINER
     文件没有特别点名，在修改 arch/x86 下文件的补丁上也请 Cc 整体 x86 维护者。

     注意，`x86@kernel.org` 并不是一个邮件列表。它仅仅是一个邮件别名，将邮件分发
     给 x86 顶层维护者团队。请始终 Cc Linux 内核邮件列表（LKML）
     `linux-kernel@vger.kernel.org`，否则你的邮件只会进入维护者的私人收件箱。

   - **调度器（Scheduler）**

     调度器开发在 -tip 树中进行，位于 sched/core 分支——偶尔会有用于进行中补丁集的
     子主题树。

   - **锁（Locking）与原子操作（atomics）**

     锁的开发（包括原子操作以及与锁相关的其它同步原语）在 -tip 树中进行，位于
     locking/core 分支——偶尔会有用于进行中补丁集的子主题树。

   - **通用中断子系统与中断芯片驱动**：

     - 中断核心开发发生在 irq/core 分支

     - 中断芯片驱动开发也发生在 irq/core 分支，但补丁通常先应用到单独的维护者树，
       然后再聚合到 irq/core

   - **时间、定时器、时间保持（timekeeping）、NOHZ 以及相关芯片驱动**：

     - 时间保持、clocksource 核心、NTP 和 alarmtimer 的开发发生在 timers/core 分支，
       但补丁通常先应用到单独的维护者树，然后再聚合到 timers/core

     - clocksource/event 驱动开发发生在 timers/core 分支，但补丁大多先应用到单独的
       维护者树，然后再聚合到 timers/core

   - **性能计数器（Performance counters）核心、架构支持以及工具（tooling）**：

     - perf 核心和架构支持开发发生在 perf/core 分支

     - perf 工具开发发生在 perf tools 维护者树，并聚合到 tip 树。

   - **CPU 热插拔（CPU hotplug）核心**

   - **RAS 核心**

     大部分 x86 特定的 RAS 补丁被收集在 tip 的 ras/core 分支。

   - **EFI 核心**

     EFI 开发在 efi git 树中进行。收集的补丁被聚合到 tip 的 efi/core 分支。

   - **RCU**

     RCU 开发发生在 linux-rcu 树。产生的改动被聚合到 tip 的 core/rcu 分支。

   - **各种核心代码组件**：

       - debugobjects

       - objtool

       - 零散的零碎代码

### 补丁提交说明

##### 选择树/分支

一般来说，针对 tip 树 master 分支的头部进行开发是可以的，但对于单独维护、拥有
自己的 git 树并且只是聚合到 tip 树的那些子系统，开发应该针对相关的子系统树或
分支进行。

针对主线的缺陷修复（bug fix）应该始终可以应用到主线内核树上。与已经排队在 tip
树中的改动之间潜在的冲突由维护者处理。

##### 补丁主题（subject）

tip 树偏好的补丁主题前缀格式是“subsys/component:”，例如“x86/apic:”、“x86/mm/fault:”、
“sched/fair:”、“genirq/core:”。请不要使用文件名或完整文件路径作为前缀。“git log
path/to/file” 在多数情况下应该能给你一个合理的提示。

主题行中凝练的补丁描述应该以大写字母开头，并且使用祈使语气书写。

##### 变更日志（Changelog）

:ref:`提交补丁指南 <describe_changes>` 中关于变更日志的一般规则同样适用。

tip 树维护者非常重视遵循这些规则，尤其是要求以祈使语气书写变更日志、不要以代码
或其执行的口吻来叙述。这并非维护者的一时兴起。用抽象措辞写成的变更日志比小说
形式的变更日志更精确、也更不容易引起混淆。

把变更日志组织成若干段落、而不是把所有内容堆在一个段落里也很有用。一个好的结构是
按“背景、问题、解决方案”的顺序，用独立的段落分别解释。

示例说明：

```

    x86/intel_rdt/mbm: Fix MBM overflow handler during hot cpu

    When a CPU is dying, we cancel the worker and schedule a new worker on a
    different CPU on the same domain. But if the timer is already about to
    expire (say 0.99s) then we essentially double the interval.

    We modify the hot cpu handling to cancel the delayed work on the dying
    cpu and run the worker immediately on a different cpu in same domain. We
    do not flush the worker because the MBM overflow worker reschedules the
    worker on same CPU and scans the domain->cpu_mask to get the domain
    pointer.

  Improved version::

    x86/intel_rdt/mbm: Fix MBM overflow handler during CPU hotplug

    When a CPU is dying, the overflow worker is canceled and rescheduled on a
    different CPU in the same domain. But if the timer is already about to
    expire this essentially doubles the interval which might result in a non
    detected overflow.

    Cancel the overflow worker and reschedule it immediately on a different CPU
    in the same domain. The work could be flushed as well, but that would
    reschedule it on the same CPU.

  Example 2::

    time: POSIX CPU timers: Ensure that variable is initialized

    If cpu_timer_sample_group returns -EINVAL, it will not have written into
    *sample. Checking for cpu_timer_sample_group's return value precludes the
    potential use of an uninitialized value of now in the following block.
    Given an invalid clock_idx, the previous code could otherwise overwrite
    *oldval in an undefined manner. This is now prevented. We also exploit
    short-circuiting of && to sample the timer only if the result will
    actually be used to update *oldval.

  Improved version::

    posix-cpu-timers: Make set_process_cpu_timer() more robust

    Because the return value of cpu_timer_sample_group() is not checked,
    compilers and static checkers can legitimately warn about a potential use
    of the uninitialized variable 'now'. This is not a runtime issue as all
    call sites hand in valid clock ids.

    Also cpu_timer_sample_group() is invoked unconditionally even when the
    result is not used because *oldval is NULL.

    Make the invocation conditional and check the return value.

  Example 3::

    The entity can also be used for other purposes.

    Let's rename it to be more generic.

  Improved version::

    The entity can also be used for other purposes.

    Rename it to be more generic.


```

对于复杂场景，尤其是竞态条件（race condition）和内存排序（memory ordering）问题，
用一张表来描绘场景很有价值，例如：

```

    CPU0                            CPU1
    free_irq(X)                     interrupt X
                                    spin_lock(desc->lock)
                                    wake irq thread()
                                    spin_unlock(desc->lock)
    spin_lock(desc->lock)
    remove action()
    shutdown_irq()
    release_resources()             thread_handler()
    spin_unlock(desc->lock)           access released resources.
                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
    synchronize_irq()

```

Lockdep 提供了类似的有助于描绘可能死锁的输出：

```

    CPU0                                    CPU1
    rtmutex_lock(&rcu->rt_mutex)
      spin_lock(&rcu->rt_mutex.wait_lock)
                                            local_irq_disable()
                                            spin_lock(&timer->it_lock)
                                            spin_lock(&rcu->mutex.wait_lock)
    --> Interrupt
        spin_lock(&timer->it_lock)


```

##### 变更日志中的函数引用

当变更日志中提到一个函数时（无论是在正文还是主题行中），请使用“function_name()”
格式。省略“()”是错误的，例如：

```

  Subject: subsys/component: Make reservation_count static

  reservation_count is only used in reservation_stats. Make it static.

```

```

  Subject: subsys/component: Make reservation_count() static

  reservation_count() is only called from reservation_stats(). Make it
  static.


```

##### 变更日志中的回溯（backtrace）

参见 backtraces。

##### 提交标签（commit tag）的顺序

为了统一查看提交标签，tip 维护者使用以下标签排序方案：

 - Fixes: 12+字符-SHA1（“sub/sys: 原始主题行”）

   即使对于不需要回移植（backport）到稳定（stable）内核的改动，也应添加 Fixes 标签，
   即当处理一个最近引入的、只影响 tip 或主线当前头部的问题时。这些标签有助于识别
   原始提交，其价值远高于在变更日志正文中醒目地提及引入问题的提交，因为它们可以
   被自动提取。

```

     Commit

       abcdef012345678 ("x86/xxx: Replace foo with bar")

     left an unused instance of variable foo around. Remove it.

     Signed-off-by: J.Dev <j.dev@mail>

   Please say instead::

     The recent replacement of foo with bar left an unused instance of
     variable foo around. Remove it.

     Fixes: abcdef012345678 ("x86/xxx: Replace foo with bar")
     Signed-off-by: J.Dev <j.dev@mail>

   The latter puts the information about the patch into the focus and
   amends it with the reference to the commit which introduced the issue
   rather than putting the focus on the original commit in the first place.

 - Reported-by: ``Reporter <reporter@mail>``

 - Closes: ``URL 或此修复所对应的缺陷报告的 Message-ID``

 - Originally-by: ``Original author <original-author@mail>``

 - Suggested-by: ``Suggester <suggester@mail>``

 - Co-developed-by: ``Co-author <co-author@mail>``

   Signed-off-by: ``Co-author <co-author@mail>``

   注意，Co-developed-by 与合著者（co-author）的 Signed-off-by 必须成对出现。

 - Signed-off-by: ``Author <author@mail>``

   在最后一个 Co-developed-by/SOB 对之后的第一个 Signed-off-by（SOB）是作者 SOB，
   即被 git 标记为作者的人。

 - Signed-off-by: ``Patch handler <handler@mail>``

   作者 SOB 之后的 SOB 来自处理和传送该补丁、但未参与开发的人。SOB 链应反映补丁
   传播到我们这里所经过的**真实**路径，其中第一个 SOB 条目表示该补丁单一的
   主要作者。Ack 应以 Acked-by 行给出，审阅批准应以 Reviewed-by 行给出。

   如果处理者（handler）对补丁或变更日志做了修改，那么应该在变更日志文本**之后**、
   所有提交标签**之上**，以下列格式提及::

     ... changelog text ends.

     [ handler: Replaced foo by bar and updated changelog ]

     First-tag: .....

   注意用两个空行将该提示与变更日志文本及提交标签分隔开。

   如果补丁由处理者发送到邮件列表，那么作者必须在变更日志的第一行用以下方式注明::

     From: Author <author@mail>

     Changelog text starts here....

   以便保留作者身份。'From:' 行之后必须跟一个空行。如果缺少该 'From:' 行，那么补丁
   会被归于发送（传送、处理）它的人。'From:' 行在补丁被应用时会被自动移除，不会
   出现在最终的 git 变更日志中。它仅影响最终 Git 提交的作者信息。

 - Tested-by: ``Tester <tester@mail>``

 - Reviewed-by: ``Reviewer <reviewer@mail>``

 - Acked-by: ``Acker <acker@mail>``

 - Cc: ``cc-ed-person <person@mail>``

   如果补丁应该回移植到 stable，请添加“``Cc: stable@vger.kernel.org``”标签，但在
   发送邮件时不要 Cc stable。

 - Link: ``https://link/to/information``

   对于引用发布到内核邮件列表的邮件，请使用 lore.kernel.org 重定向器 URL::

     Link: https://lore.kernel.org/email-message-id@here

   该 URL 应用于引用相关的邮件列表主题、相关的补丁集或其它值得注意的讨论线程。将
   ``Link:`` 预告（trailer）与提交信息关联起来的一个便捷方法是使用类 Markdown 的
   方括号记法，例如::

     A similar approach was attempted before as part of a different
     effort [1], but the initial implementation caused too many
     regressions [2], so it was backed out and reimplemented.

     Link: https://lore.kernel.org/some-msgid@here # [1]
     Link: https://bugzilla.example.org/bug/12345  # [2]

   你也可以使用 ``Link:`` 预告来标示将补丁应用到你的 git 树时的来源。在这种情况下，
   请使用专用的 ``patch.msgid.link`` 域名，而不是 ``lore.kernel.org``。这种做法使
   自动化工具能够识别使用哪个链接来取回原始补丁提交。例如::

     Link: https://patch.msgid.link/patch-source-message-id@here

```

请不要使用组合标签，例如 `Reported-and-tested-by`，因为它们只会使标签的自动提取
变得复杂。

##### 文档链接

在变更日志中提供文档链接对日后调试和分析是极大的帮助。遗憾的是，URL 往往很快就
失效，因为公司频繁地重构其网站。非“易变（volatile）”的例外包括 Intel SDM 和
AMD APM。

因此，对于“易变”文档，请在 kernel bugzilla https://bugzilla.kernel.org 创建一个
条目，并将这些文档的副本附到该 bugzilla 条目上。最后，在变更日志中提供该 bugzilla
条目的 URL。

##### 补丁重发或提醒

参见 resend_reminders。

##### 合并窗口（Merge window）

请不要在合并窗口期间或临近合并窗口时期望 tip 维护者会审阅或合并补丁。在此期间，
除了紧急修复外，这些树都是关闭的。一旦合并窗口关闭并发布新的 -rc1 内核，它们会
重新开放。

大型补丁系列（series）应该在合并窗口开启**至少**一周之前以可合并状态提交。对于
缺陷修复以及**有时**针对新硬件的小型独立驱动或侵入性极小的硬件支持补丁，可以有
例外。

在合并窗口期间，维护者转而专注于跟踪上游改动、修复合并窗口产生的问题、收集缺陷
修复，并让自己喘口气。请尊重这一点。

所谓的*紧急（urgent）*分支会在每个发布版本的 stabilization 阶段被合并到主线。

##### Git

tip 维护者接受来自维护者的 git pull 请求，这些维护者提供要在 tip 树中聚合的子系统
改动。

针对新补丁提交的 pull 请求通常不被接受，也不能取代向邮件列表的正确补丁提交。主要
原因是审阅工作流程是基于邮件的。

如果你提交一个较大的补丁系列，提供一个私有仓库中的 git 分支会很有帮助，使感兴趣
的人可以轻松拉取该系列进行测试。通常的做法是在补丁系列的封面信（cover letter）中
提供 git URL。

##### 测试

代码在提交给 tip 维护者之前应当经过测试。除了微小的改动之外，任何改动都应该构建、
启动，并在启用了全面（且重量级）的内核调试选项的情况下进行测试。

这些调试选项可以在 kernel/configs/x86_debug.config 中找到，并可通过运行以下命令
添加到已有的内核配置中：

	make x86_debug.config

其中一些选项是 x86 特定的，在其它架构上测试时可以省去。

### 编码风格说明

##### 注释风格

注释中的句子以大写字母开头。

```

	/* This is a single line comment */

```

```

	/*
	 * This is a properly formatted
	 * multi-line comment.
	 *
	 * Larger multi-line comments should be split into paragraphs.
	 */

```

不要使用尾随注释（tail comment）（见下）：

  Please refrain from using tail comments. Tail comments disturb the
```

	if (somecondition_is_true) /* Don't put a comment here */
		dostuff(); /* Neither here */

	seed = MAGIC_CONSTANT; /* Nor here */

  Use freestanding comments instead::

	/* This condition is not obvious without a comment */
	if (somecondition_is_true) {
		/* This really needs to be documented */
		dostuff();
	}

	/* This magic initialization needs a comment. Maybe not? */
	seed = MAGIC_CONSTANT;

  Use C++ style, tail comments when documenting structs in headers to
  achieve a more compact layout and better readability::

        // eax
        u32     x2apic_shift    :  5, // Number of bits to shift APIC ID right
                                      // for the topology ID at the next level
                                : 27; // Reserved
        // ebx
        u32     num_processors  : 16, // Number of processors at current level
                                : 16; // Reserved

  versus::

	/* eax */
	        /*
	         * Number of bits to shift APIC ID right for the topology ID
	         * at the next level
	         */
         u32     x2apic_shift    :  5,
		 /* Reserved */
				 : 27;

	/* ebx */
		/* Number of processors at current level */
	u32     num_processors  : 16,
		/* Reserved */
				: 16;

```

注释重要的东西：

  Comments should be added where the operation is not obvious. Documenting
```

	/* Decrement refcount and check for zero */
	if (refcount_dec_and_test(&p->refcnt)) {
		do;
		lots;
		of;
		magic;
		things;
	}

  Instead, comments should explain the non-obvious details and document
  constraints::

	if (refcount_dec_and_test(&p->refcnt)) {
		/*
		 * Really good explanation why the magic things below
		 * need to be done, ordering and locking constraints,
		 * etc..
		 */
		do;
		lots;
		of;
		magic;
		/* Needs to be the last operation because ... */
		things;
	}

```

函数文档注释：

  To document functions and their arguments please use kernel-doc format
```

	/**
	 * magic_function - Do lots of magic stuff
	 * @magic:	Pointer to the magic data to operate on
	 * @offset:	Offset in the data array of @magic
	 *
	 * Deep explanation of mysterious things done with @magic along
         * with documentation of the return values.
	 *
	 * Note, that the argument descriptors above are arranged
	 * in a tabular fashion.
	 */

  This applies especially to globally visible functions and inline
  functions in public header files. It might be overkill to use kernel-doc
  format for every (static) function which needs a tiny explanation. The
  usage of descriptive function names often replaces these tiny comments.
  Apply common sense as always.


```

##### 记录锁的要求

  Documenting locking requirements is a good thing, but comments are not
```

	/* Caller must hold foo->lock */
	void func(struct foo *foo)
	{
		...
	}

  Please use::

	void func(struct foo *foo)
	{
		lockdep_assert_held(&foo->lock);
		...
	}

  In PROVE_LOCKING kernels, lockdep_assert_held() emits a warning
  if the caller doesn't hold the lock.  Comments can't do that.

```

##### 括号规则

只有在跟随“if”、“for”等之后的语句是单行时，才可以省略括号，例如：

```

	if (foo)
		do_something();

```

即使如下情况也不被视为单行语句：

```

	for (i = 0; i < end; i++)
		if (foo[i])
			do_something(foo[i]);

```

```

	for (i = 0; i < end; i++) {
		if (foo[i])
			do_something(foo[i]);
	}


```

##### 变量声明

变量声明在块开头时的首选顺序如下：

```

	struct long_struct_name *descriptive_name;
	unsigned long foo, bar;
	unsigned int tmp;
	int ret;

```

```

	int ret;
	unsigned int tmp;
	unsigned long foo, bar;
	struct long_struct_name *descriptive_name;

```

```

	unsigned long foo, bar;
	int ret;
	struct long_struct_name *descriptive_name;
	unsigned int tmp;

```

另外，请尽量将同一类型的变量聚合到一行：

```

	unsigned long a;
	unsigned long b;
	unsigned long c;
	unsigned long d;

```

```

	unsigned long a, b, c, d;

```

```

	struct long_struct_name *descriptive_name = container_of(bar,
						      struct long_struct_name,
	                                              member);
	struct foobar foo;

```

将初始化移到声明之后的单独一行会更好：

```

	struct long_struct_name *descriptive_name;
	struct foobar foo;

	descriptive_name = container_of(bar, struct long_struct_name, member);


```

##### 变量类型

对于旨在描述硬件或作为访问硬件的函数参数的变量，请使用适当的 u8、u16、u32、u64
类型。这些类型清晰地定义了位宽，并避免了截断、扩展以及 32/64 位混淆。

在如果使用“unsigned long”会对 32 位内核产生歧义的代码中也推荐使用 u64。虽然
在这种情况下也可以使用“unsigned long long”，但 u64 更短，并且也清楚地表明该操作
要求为 64 位宽，与目标 CPU 无关。

请使用“unsigned int”而不是“unsigned”。

##### 常量

请不要在代码或初始化器中使用字面（十六进制/十进制）数字。要么使用具有描述性名称
的适当 define，要么考虑使用 enum。

##### 结构体声明与初始化器

结构体声明应该将结构体成员名以表格形式对齐：

```

	struct bar_order {
		unsigned int	guest_id;
		int		ordered_item;
		struct menu	*menu;
	};

```

请避免在声明中记录结构体成员，因为这常常导致格式奇怪的注释，而且结构体成员：

```

	struct bar_order {
		unsigned int	guest_id; /* Unique guest id */
		int		ordered_item;
		/* Pointer to a menu instance which contains all the drinks */
		struct menu	*menu;
	};

```

相反，请考虑在结构体声明之前的注释中使用 kernel-doc 格式，这样做更易读，并且还
有一个额外的好处，即把信息纳入内核文档中，例如：

```

	/**
	 * struct bar_order - Description of a bar order
	 * @guest_id:		Unique guest id
	 * @ordered_item:	The item number from the menu
	 * @menu:		Pointer to the menu from which the item
	 *  			was ordered
	 *
	 * Supplementary information for using the struct.
	 *
	 * Note, that the struct member descriptors above are arranged
	 * in a tabular fashion.
	 */
	struct bar_order {
		unsigned int	guest_id;
		int		ordered_item;
		struct menu	*menu;
	};

```

静态结构体初始化器必须使用 C99 初始化器，并且也应该：

```

	static struct foo statfoo = {
		.a		= 0,
		.plain_integer	= CONSTANT_DEFINE_OR_ENUM,
		.bar		= &statbar,
	};

```

注意，虽然 C99 语法允许省略最后的逗号，但我们建议在最后一行使用逗号，因为这使得
重新排序和添加新行更容易，也让这类未来的补丁稍微更易读。

##### 换行

将行长限制在 80 个字符会使深度缩进的代码难以阅读。考虑将代码提取到辅助函数中，
以避免过度换行。

80 字符规则并非硬性规则，因此在换行时请运用常识。尤其是格式字符串绝不应被拆开。

拆分函数声明或函数调用时，请将第二行中的第一个参数与第一行中的第一个参数对齐：

```

  static int long_function_name(struct foobar *barfoo, unsigned int id,
				unsigned int offset)
  {

	if (!id) {
		ret = longer_function_name(barfoo, DEFAULT_BARFOO_ID,
					   offset);
	...

```

##### 命名空间（Namespaces）

函数/变量命名空间提高了可读性并便于搜索（grep）。这些命名空间是全局可见的函数和
变量名（包括内联函数）的字符串前缀。这些前缀应结合子系统名与组件名，例如
“x86_comp\_”、“sched\_”、“irq\_”和“mutex\_”。

这也包括被立即放入全局可见驱动模板的静态文件作用域函数——对于这些符号，带上一个
好的前缀也很有用，以便回溯（backtrace）时可读。

对于局部静态函数和变量，可以省略命名空间前缀。真正局部的函数，只被其它局部函数
调用，可以有更短的描述性名称——我们主要关心的是可搜索性和回溯可读性。

请注意，“xxx_vendor\_”和“vendor_xxx_”前缀对于厂商特定文件中的静态函数并无帮助。
毕竟，代码是厂商特定的这一点已经很清楚了。此外，厂商名只应用于真正厂商特定的功能。

一如既往，运用常识，以一致性和可读性为目标。

### 提交通知

tip 树由一个机器人监控新提交。该机器人为每次新提交向一个专用邮件列表
（`linux-tip-commits@vger.kernel.org`）发送邮件，并 Cc 在其中一个提交标签中被
提及的所有人。它使用标签列表末尾 Link 标签中的邮件 Message-ID 来设置 In-Reply-To
邮件头，从而使该邮件与补丁提交邮件正确地形成线程。

tip 维护者和子维护者会尽量在合并补丁时回复提交者，但他们有时忘记，或者不符合
当下的工作流程。虽然机器人消息纯粹是机械性的，但它也意味着“谢谢！已应用。”。
