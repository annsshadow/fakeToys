
## PROPER CARE AND FEEDING OF RETURN VALUES FROM rcu_dereference()

合理地维护和使用来自 rcu_dereference() 的返回值

正确维护和使用地址依赖与数据依赖，对正确使用 RCU 这类机制至关重要。为此，
rcu_dereference() 系列原语返回的指针会携带地址依赖与数据依赖。这些依赖从
rcu_dereference() 宏加载该指针开始，一直延伸到后续使用该指针去计算某次内存
访问的地址（对应地址依赖）或计算某次内存访问所写入的值（对应数据依赖）。

大多数情况下，这些依赖会被保留，使你可以自由地使用 rcu_dereference() 的返回值。
例如，解引用（前缀 `*`）、字段选择（`->`）、赋值（`=`）、取地址（`&`）、类型转换、
以及对常量做加法或减法，都能很自然、很安全地进行。但是，由于当前的编译器
并不会考虑地址依赖或数据依赖，你仍有可能陷入麻烦。

请遵循以下规则来保留源自 rcu_dereference() 及其相关调用的地址依赖与数据依赖，
从而保证你的 RCU 读者正确运行：

- 你必须使用 rcu_dereference() 系列原语之一来加载受 RCU 保护的指针，否则
	CONFIG_PROVE_RCU 会发出警告。更糟糕的是，由于编译器和 DEC Alpha 可能
	玩的一些把戏，你的代码会出现随机的内存破坏错误。如果没有 rcu_dereference()
	系列原语，编译器可以重新加载该值，而你的代码面对同一个指针的两个不同值
	岂不是会乱套！如果没有 rcu_dereference()，DEC Alpha 可以加载一个指针、
	解引用该指针，并返回该指针存储之前、初始化之前的旧数据。（如前文所述，
	在近期的核中，READ_ONCE() 也能阻止 DEC Alpha 玩这些把戏。）

	此外，rcu_dereference() 中的 volatile 强制转换可阻止编译器推导出所得指针的值。
	请参见标题为"EXAMPLE WHERE THE COMPILER KNOWS TOO MUCH"的小节，其中给出了
	编译器确实能够推导出指针精确值、从而造成乱序的示例。

- 在这样一种特殊情况中：数据只会被添加、而在读者访问该结构期间永远不会被移除，
	可以使用 READ_ONCE() 来代替 rcu_dereference()。在此情况下，使用 READ_ONCE()
	扮演了 v4.15 中被移除的 lockless_dereference() 原语的角色。

- 你只能在指针值上使用 rcu_dereference()。编译器对整型值知道得太多了，不值得
	信任它会通过整数运算承载依赖。有极少数例外，即你可以临时将指针转换为
	uintptr_t，以便：

 - 在该指针必须为零的低位比特中置位和清零。这显然意味着该指针必须具有对齐
		约束，例如，这通常对 char* 指针是**不**适用的。

 - 对指针做异或运算来转换指针，正如一些经典的伙伴分配器算法中所做的那样。

	在执行任何其他操作之前，将值转换回指针是很重要的。

- 使用 `+` 和 `-` 中缀算术运算符时避免相消。例如，对于给定变量 "x"，对 char*
	指针避免使用 `(x-(uintptr_t)x)`。编译器有权用零替换此类表达式，从而使得
	后续访问不再依赖 rcu_dereference()，由此可能再次因乱序导致错误。

	当然，如果 "p" 是来自 rcu_dereference() 的指针，而 "a" 和 "b" 是恰好相等的
	整数，那么表达式 "p+a-b" 是安全的，因为其值必然仍依赖 rcu_dereference()，
	从而维持了正确的顺序。

- 如果你使用 RCU 来保护 JIT 编译的函数，使得 `()` 函数调用运算符被应用到
	（直接地或间接地）从 rcu_dereference() 获得的值上，你可能需要直接与硬件
	交互以刷新指令缓存。当新 JIT 的函数使用了早先某个 JIT 函数所用的同一块
	内存时，某些系统上会出现此问题。

- 解引用时不要使用关系运算符（`==`、`!=`、`>`、`>=`、`<` 或 `<=`）的结果。例如，

```
		int *p;
		int *q;

		...

		p = rcu_dereference(gp)
		q = &global_q;
		q += p > &oom_p;
		r1 = *q;  /* BUGGY!!! */

```

	如前所述，这种做法有 bug 的原因是关系运算符通常被编译成分支。同样如前文所述，
	虽然像 ARM 或 PowerPC 这样的弱内存机器会对这类分支之后的存储排序，但可以
	对加载做投机执行，从而再次可能造成乱序错误。

```
- 将来自 rcu_dereference() 的指针与非 NULL 值进行比较时要非常小心。正如 Linus
	Torvalds 所解释的那样，如果两个指针相等，编译器可以用你拿来比较的那个指针
	替换被比较的指针

		p = rcu_dereference(gp);
		if (p == &default_struct)
			do_default(p->a);

	由于编译器现在知道 "p" 的值恰好是变量 "default_struct" 的地址，它可以自由地
	将这段代码转换为如下形式：

		p = rcu_dereference(gp);
		if (p == &default_struct)
			do_default(default_struct.a);

	在 ARM 和 Power 硬件上，对 "default_struct.a" 的加载现在可能被投机执行，
	从而可能发生在 rcu_dereference() 之前。这可能会因乱序而导致错误。

	但是，在以下情况下进行比较是安全的：

	-	与 NULL 指针进行比较。如果编译器知道该指针为 NULL，你本来就不该
		去解引用它。如果比较结果是非相等，编译器也不更聪明。因此，
		将来自 rcu_dereference() 的指针与 NULL 指针进行比较是安全的。

	-	该指针在被比较之后永远不会被解引用。由于不存在后续的解引用，编译器
		无法利用它从比较中学到的任何信息来重排那些并不存在的后续解引用。
		这种比较在扫描受 RCU 保护的循环链表时经常发生。

		注意，如果指针比较是在 RCU 读者临界区之外完成的，且该指针从未被解引用，
		则应当使用 rcu_access_pointer() 代替 rcu_dereference()。在大多数情况下，
		最好直接测试 rcu_access_pointer() 的返回值，而不将其赋给变量，以避免
		意外的解引用。

		在 RCU 读者临界区内部，几乎没有理由使用 rcu_access_pointer()。

	-	被比较的指针所引用的是"很久以前"就已初始化的内存。这种情况安全的
		原因是，即使发生了乱序，这种乱序也不会影响比较之后的那些访问。那么
		"很久以前"到底是多久？以下是一些可能性：

		-	编译期。

		-	启动期。

		-	模块代码的模块初始化期。

		-	在 kthread 创建之前、针对 kthread 代码的时期。

		-	在我们当前持有的某个锁的某次较早获取期间。

		-	在定时器处理函数的 mod_timer() 之前。

		Linux 内核有大量的原语会导致代码在稍后某个时刻被调用，还有许多其他
		可能性。

	-	被比较的那个指针同样来自于 rcu_dereference()。在这种情况下，两个指针
		都依赖某个 rcu_dereference() 或另一个，因此无论哪种方式你都能获得
		正确的顺序。

		话虽如此，这种情况可能使某些 RCU 使用错误更有可能发生。若这些错误
		发生在测试期间，这倒可能是件好事。此类 RCU 使用错误的一个示例见标题为
		"EXAMPLE OF AMPLIFIED RCU-USAGE BUG"的小节。

	-	比较之后的所有访问都是存储，从而控制依赖保留了所需的顺序。话虽如此，
		控制依赖很容易用错。更多细节请参阅 Documentation/memory-barriers.txt
		中的"CONTROL DEPENDENCIES"小节。

	-	指针不相等*且*编译器没有足够信息推导出该指针的值。注意，rcu_dereference()
		中的 volatile 强制转换通常会阻止编译器知道太多信息。

		但是，请注意，如果编译器知道该指针只取两个值之一，那么一个非相等比较
		恰好会提供编译器推导出该指针值所需的信息。

```

- 关闭编译器可能提供的任何值投机优化，特别是当你使用了基于反馈的、从先前
	运行收集数据的优化时。这类值投机优化在设计上就是重排操作的。

	这条规则有一个例外：利用分支预测硬件的值投机优化在强序系统（如 x86）上是
	安全的，但在弱序系统（如 ARM 或 Power）上不安全。请明智地选择你的编译器
	命令行选项！


### EXAMPLE OF AMPLIFIED RCU-USAGE BUG

RCU 使用错误被放大的示例

由于更新者可以与 RCU 读者并发运行，RCU 读者可能看到陈旧和不一致的值。如果 RCU
读者需要新鲜或一致的值（有时确实需要），它们需要正确地进行

```

	struct foo {
		int a;
		int b;
		int c;
	};
	struct foo *gp1;
	struct foo *gp2;

	void updater(void)
	{
		struct foo *p;

		p = kmalloc(...);
		if (p == NULL)
			deal_with_it();
		p->a = 42;  /* Each field in its own cache line. */
		p->b = 43;
		p->c = 44;
		rcu_assign_pointer(gp1, p);
		p->b = 143;
		p->c = 144;
		rcu_assign_pointer(gp2, p);
	}

	void reader(void)
	{
		struct foo *p;
		struct foo *q;
		int r1, r2;

		rcu_read_lock();
		p = rcu_dereference(gp2);
		if (p == NULL)
			return;
		r1 = p->b;  /* Guaranteed to get 143. */
		q = rcu_dereference(gp1);  /* Guaranteed non-NULL. */
		if (p == q) {
			/* The compiler decides that q->c is same as p->c. */
			r2 = p->c; /* Could get 44 on weakly order system. */
		} else {
			r2 = p->c - r1; /* Unconditional access to p->c. */
		}
		rcu_read_unlock();
		do_something_with(r1, r2);
	}

```

你可能会对结果 (r1 == 143 && r2 == 44) 是可能的感到惊讶，但你不应惊讶。毕竟，
更新者可能在 reader() 把值载入 "r1" 与载入 "r2" 之间被第二次调用。由于编译器
和 CPU 的某些重排，同样的结果也可能出现，这一点倒无关紧要。

但如果读者需要一致的视图呢？

```

	struct foo {
		int a;
		int b;
		int c;
		spinlock_t lock;
	};
	struct foo *gp1;
	struct foo *gp2;

	void updater(void)
	{
		struct foo *p;

		p = kmalloc(...);
		if (p == NULL)
			deal_with_it();
		spin_lock(&p->lock);
		p->a = 42;  /* Each field in its own cache line. */
		p->b = 43;
		p->c = 44;
		spin_unlock(&p->lock);
		rcu_assign_pointer(gp1, p);
		spin_lock(&p->lock);
		p->b = 143;
		p->c = 144;
		spin_unlock(&p->lock);
		rcu_assign_pointer(gp2, p);
	}

	void reader(void)
	{
		struct foo *p;
		struct foo *q;
		int r1, r2;

		rcu_read_lock();
		p = rcu_dereference(gp2);
		if (p == NULL)
			return;
		spin_lock(&p->lock);
		r1 = p->b;  /* Guaranteed to get 143. */
		q = rcu_dereference(gp1);  /* Guaranteed non-NULL. */
		if (p == q) {
			/* The compiler decides that q->c is same as p->c. */
			r2 = p->c; /* Locking guarantees r2 == 144. */
		} else {
			spin_lock(&q->lock);
			r2 = q->c - r1;
			spin_unlock(&q->lock);
		}
		rcu_read_unlock();
		spin_unlock(&p->lock);
		do_something_with(r1, r2);
	}

```

一如既往，选用合适的工具来完成工作！


### EXAMPLE WHERE THE COMPILER KNOWS TOO MUCH

编译器知道得太多的示例

如果从 rcu_dereference() 获得的指针与某个其他指针比较为非相等，编译器通常
无从得知第一个指针的值可能是什么。这种信息缺失阻止了编译器执行那些本来可能
破坏 RCU 所依赖的顺序保证的优化。而 rcu_dereference() 中的 volatile 强制转换
应当能阻止编译器猜测该值。

但是，如果没有 rcu_dereference()，编译器知道的可能比你想象的更多

```

	struct foo {
		int a;
		int b;
	};
	static struct foo variable1;
	static struct foo variable2;
	static struct foo *gp = &variable1;

	void updater(void)
	{
		initialize_foo(&variable2);
		rcu_assign_pointer(gp, &variable2);
		/*
		 * The above is the only store to gp in this translation unit,
		 * and the address of gp is not exported in any way.
		 */
	}

	int reader(void)
	{
		struct foo *p;

		p = gp;
		barrier();
		if (p == &variable1)
			return p->a; /* Must be variable1.a. */
		else
			return p->b; /* Must be variable2.b. */
	}

```

由于编译器能看到对 "gp" 的所有存储，它知道 "gp" 可能的值只有 variable1 和
variable2 这两种。因此 reader() 中的比较即便在非相等的情况下，也告诉了编译器
"p" 的精确值。这使得编译器能够令返回值不依赖于从 "gp" 的加载，进而破坏了这次
加载与那些返回值的加载之间的顺序关系。这会导致 "p->b" 在弱序系统上返回
初始化之前的垃圾值。

简而言之，当你要去解引用所得指针时，rcu_dereference() **不是**可有可无的。


### WHICH MEMBER OF THE rcu_dereference() FAMILY SHOULD YOU USE?

你应该使用 rcu_dereference() 家族中的哪一个成员？

首先，请避免使用 rcu_dereference_raw()，也请避免使用带有常量参数值 1（或
true）的 rcu_dereference_check() 和 rcu_dereference_protected()。在给出这一
警告之后，以下是一些关于在各种情形下使用 rcu_dereference() 哪个成员的指导：

1. 如果访问需要位于 RCU 读者临界区之内，使用 rcu_dereference()。在合并后的
	新 RCU 变体中，进入 RCU 读者临界区是通过 rcu_read_lock()、任何禁用底半部
	的操作、任何禁用中断的操作，或任何禁用抢占的操作来实现的。请注意，自旋锁
	临界区也隐含为 RCU 读者临界区，即使它们是可抢占的（在使用 CONFIG_PREEMPT_RT=y
	构建的内核中也是如此）。

2. 如果访问可能位于 RCU 读者临界区之内（一方面），或者受（比如说）my_lock 保护
	（另一方面），使用

```

		p1 = rcu_dereference_check(p->rcu_protected_pointer,
					   lockdep_is_held(&my_lock));

```

3. 如果访问可能位于 RCU 读者临界区之内（一方面），或者受 my_lock 或 your_lock
	二者之一保护（另一方面），使用

```

		p1 = rcu_dereference_check(p->rcu_protected_pointer,
					   lockdep_is_held(&my_lock) ||
					   lockdep_is_held(&your_lock));

```

4. 如果访问位于更新侧，因而始终受到保护，使用

```

		p1 = rcu_dereference_protected(p->rcu_protected_pointer,
					       lockdep_is_held(&my_lock));

```

	这可以像上面的 #3 那样扩展到处理多个锁，两者也都能扩展为检查其他条件。

	5. 如果保护是由调用者提供的、因此本代码无从得知，那就是极少需要使用
	rcu_dereference_raw() 的情形。此外，当 lockdep 表达式会过分复杂时，
	rcu_dereference_raw() 可能是合适的，不过这种情况下更好的办法也许是好好
	审视一下你的同步设计。尽管如此，还是存在这样的数据加锁情形：极大数量的
	锁或引用计数中的任意一个都足以保护该指针，因此 rcu_dereference_raw() 确有
	其用武之地。

	不过，它的用武之地可能比你依据当前内核中的使用次数所预期的要小得多。
	它的同义词 rcu_dereference_check( ... , 1)，以及它的近亲
	rcu_dereference_protected(... , 1)，也是如此。


### SPARSE CHECKING OF RCU-PROTECTED POINTERS

对受 RCU 保护的指针做 sparse 检查

sparse 静态分析工具会检查对受 RCU 保护指针的非 RCU 访问，这类访问可能因涉及
编译器发明加载、或许还有加载分裂（load tearing）的优化而导致"有趣"的 bug。

```

	p = q->rcu_protected_pointer;
	do_something_with(p->a);
	do_something_else_with(p->b);

```

如果寄存器压力很高，编译器可能会把 "p" 优化掉

```

	do_something_with(q->rcu_protected_pointer->a);
	do_something_else_with(q->rcu_protected_pointer->b);

```

如果 q->rcu_protected_pointer 在此期间发生了改变，这可能会致命地令你的代码失望。
而且这并非理论问题：恰恰这类 bug 在 1990 年代初让 Paul E. McKenney（以及他的
几位无辜同事）搭上了一个三天的周末。

加载分裂当然可能导致解引用一对指针被糅合的结果，这同样可能致命地令你的代码失望。

这些问题本可以通过简单地让代码改为如下形式来避免

```

	p = rcu_dereference(q->rcu_protected_pointer);
	do_something_with(p->a);
	do_something_else_with(p->b);

```

遗憾的是，这类 bug 在评审时极难发现。这正是 sparse 工具以及 "__rcu" 标记的
用武之地。如果你给一个指针声明（无论是在结构体中还是作为形参）加上 "__rcu"，
就告诉 sparse 在该指针被直接访问时发出警告。如果某个未标记 "__rcu" 的指针被
rcu_dereference() 及其相关原语访问，它也会让 sparse 发出警告。例如，
->rcu_protected_pointer 可能被声明为

```

	struct foo __rcu *rcu_protected_pointer;

```

使用 "__rcu" 是选择加入（opt-in）的。如果你选择不使用它，那么你应该忽略 sparse
的警告。
