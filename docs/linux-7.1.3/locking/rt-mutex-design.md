## RT-mutex 实现设计


Copyright (c) 2006 Steven Rostedt

Licensed under the GNU Free Documentation License, Version 1.2


本文档试图描rtmutex.c 实现的design。它不描rtmutex.c 存在的原因。关于那一点，
请参Documentation/locking/rt-mutex.rst。尽管本文档确实解释了在没有这段代码时会
发生的问题，但那是为了理解代码实际在做什么而给出的概念
本文档的目标是帮助他人理解所使用（且已实现的 PI）的优先级继承（PI）算法，以及
以那种方式实PI 所做决策的原因

### 鏃犵晫浼樺厛绾у弽杞。

当较低优先级的进程执行、而较高优先级的进程想要运行时，就发生了优先级反转。这出于
若干原因而发生，并且大多数时候是无可奈何的。任何时候，当一个高优先级进程想要使一个低优先级进程持有的资源（例如一mutex）时，高优先级进程必须等到低优先级进用完该资源。这就是优先级反转。我们想要防止的是所谓的无界优先级反转。即高优先级
进程被低优先级进程阻止运行一段不确定的时间
无界优先级反转的经典例子是，你有三个进程，我们称它们为进A、B C，其A 最高优先级进程，C 是最低优先级，B 介于两者之间。A 试图获取 C 拥有的锁，必须等并让 C 运行以释放锁。但与此同时，B 执行了，并且由于 B 优先级比 C 高，它抢C，但
这样做实际上也抢占了一个更高优先级进程 A。现在无法知A 要睡多久来等C 释放锁，
因为据我们所知，B 是个 CPU 霸占者，永远不会C 机会去释放锁。这被称为无界优先级
反转
```

     grab lock L1 (owned by C)
       |
  A ---+
          C preempted by B
            |
  C    +----+

  B         +-------->
                  B now keeps A from running.


```

### 优先级继承（PI

有几种方法可以解决这个问题，但其他方法不在本文档范围内。这里我们只讨论 PI
PI 是指，如果一个进程阻塞在“当前进程拥有的锁”上，那么该进程就继承另一个进程的
优先级。为了更容易理解，让我们再次使用前面的例子，即进A、B C
这一次，A 阻塞C 拥有的锁上时，C 将继A 的优先级。所以现在如B 变得可运行，
它不会抢C，因C 现在拥有 A 的高优先级。一C 释放锁，它就失去其继承的优先级，
A 随后可以继续使用 C 曾持有的资源

### 术语


这里我解释本文档中使用的一些术语，以帮助描述用于实PI 的设计
PI chain
         - PI chain 是一系列有序的锁和进程，它们导致进程从一个先前阻塞在
           其某个锁上的进程继承优先级。这在本文档后面有更详细的描述
mutex
         - 在本文档中，为了区别于实PI 的锁以及 PI 代码中使用的自旋锁，
           从现在起，PI 锁将被称mutex
lock
         - 在本文档中，从现在起，当我提到用于保PI 算法部分所用的
           自旋锁时，我将使用术lock。这些锁UP（当启用 CONFIG_PREEMPT 时）
           上禁用抢占，SMP 上防止多CPU 同时进入临界区
spin lock
         - 与上面的 lock 相同
waiter
         - waiter 是一个存储在被阻塞进程栈上的结构体。由waiter 的作用域
           在被阻塞mutex 上的进程的代码之内，waiter 分配在进程的栈上
           （局部变量）是没问题的。该结构体持有一个指task 的指针，以及
           task 所阻塞在其上的 mutex。它还有 rbtree 节点结构，用于将 task
           放入一mutex waiters rbtree 中，以及放入 mutex 属主 task            pi_waiters rbtree 中（如下所述）
           waiter 有时也用来指代正在等待一mutex task。这waiter->task
           相同
waiters
         - 阻塞在一mutex 上的一组进程
top waiter
         - 等待在一个特mutex 上的最高优先级进程
top pi waiter
              - 等待在一个特定进程拥有的某个 mutex 上的最高优先级进程
Note:
       task process 在本文档中可互换使用，主要是为了区分两个被一起描述的进程

### PI chain


PI chain 是一个可能导致发生优先级继承的进程和 mutex 的列表。多个链可能汇聚，但一链永远不会分叉，因为一个进程一次不能阻塞在多于一mutex 上
```

   Process:  A, B, C, D, E
   Mutexes:  L1, L2, L3, L4

   A owns: L1
           B blocked on L1
           B owns L2
                  C blocked on L2
                  C owns L3
                         D blocked on L3
                         D owns L4
                                E blocked on L4

```

```

   E->L4->D->L3->C->L2->B->L1->A

```

为了展示两条链在何处合并，我们可以添加另一个进F 和另一mutex L5，其B 拥有
L5，F 阻塞mutex L5 上
```

   F->L5->B->L1->A

```

由于一个进程可能拥有多于一mutex，但永远不会阻塞在多于一个之上，链就发生了合并
```

   E->L4->D->L3->C->L2-+
                       |
                       +->B->L1->A
                       |
                 F->L5-+

```

为了PI 工作，这些链的右端（或者我们也可以称之为链的顶端）的进程，其优先级必须
等于或高于链中左侧或下方的进程
同样，由于一mutex 上可能阻塞有不止一个进程，我们可以有多个链mutex 处合并如果我们添加另一个进G，它
```

  G->L2->B->L1->A

```

而再次，为了展示这如何增长，我将展示合并的链
```

   E->L4->D->L3->C-+
                   +->L2-+
                   |     |
                 G-+     +->B->L1->A
                         |
                   F->L5-+

```

如果进程 G 在链中具有最高优先级，那么链上游的所task（本例中A B）都必须其优先级提升G 的优先级

### Mutex 等待者树


每个 mutex 都跟踪所有阻塞在其自身上waiter。该 mutex 有一rbtree，按优先存储这些 waiter。这棵树由一个位mutex 结构中的自旋锁保护。这个锁称为 wait_lock

### Task PI 鏍。

为了跟踪 PI chain，每个进程都有自己的 PI rbtree。这是一棵由该进程拥有的所mutex
top waiter 组成的树。注意，这棵树只持有 top waiter，而不是所有阻塞在该进程拥有的
mutex 上的 waiter
task PI 树的顶端始终是正在等待该 task 拥有的某mutex 的最高优先级 task。所如果task 继承了某个优先级，它将始终是这棵树顶端那task 的优先级
这棵树作为一棵名pi_waiters rbtree 存储在进程的 task 结构中。它由同样位task 结构中的一个自旋锁保护，称pi_lock。这个锁也可能在中断上下文中被获取，所在锁pi_lock 时必须禁用中断

### PI 链的深度


PI 链的最大深度不是动态的，实际上可以被定义。但要弄清楚它非常复杂，因为它取决于
所mutex 的嵌套情况。让我们看一个例子，其中3 mutex，L1、L2 L3，以及四独立的函func1、func2、func3 func4。下面显示了 L1->L2->L3 的加锁顺序，但可并非实际如此
```

  void func1(void)
  {
	mutex_lock(L1);

	/* do anything */

	mutex_unlock(L1);
  }

  void func2(void)
  {
	mutex_lock(L1);
	mutex_lock(L2);

	/* do something */

	mutex_unlock(L2);
	mutex_unlock(L1);
  }

  void func3(void)
  {
	mutex_lock(L2);
	mutex_lock(L3);

	/* do something else */

	mutex_unlock(L3);
	mutex_unlock(L2);
  }

  void func4(void)
  {
	mutex_lock(L3);

	/* do something again */

	mutex_unlock(L3);
  }

```

现在我们添加 4 个进程，分别单独运行这些函数。进A、B、C D 分别运行函数 func1func2、func3 func4，并且使D 先运行、A 最后运行。D 被抢占时
```

  D owns L3
         C blocked on L3
         C owns L2
                B blocked on L2
                B owns L1
                       A blocked on L1

  And thus we have the chain A->L1->B->L2->C->L3->D.

```

这给了我们一PI 深度4（四个进程），但单独看这些函数中的任何一个，似乎它们最只有加锁深度 2。所以，尽管加锁深度在编译时定义，要找出达到那个深度的可能性仍然非困难
现在由于 mutex 可以由用户态应用程序定义，我们不希望一个嵌套大mutex DOS 应用程序创建出一条巨大的 PI 链，并且让代码在查看大量数据时持有着自旋锁。所以为防止这一点，该实现不仅实现了最大加锁深度，而且在沿 PI 链行走时也最多只同时持有个不同的锁。更多内容见下文

### Mutex 属主与标

mutex 结构包含一个指mutex 属主的指针。如mutex 未被拥有，该属主被设NULL由于所有体系结构的 task 结构都至少有两字节对齐（如果这不成立，rtmutex.c 代码就会
坏掉！），这就允许将最低有效位用作一个标志。Bit 0 被用作“Has Waiters（有等待者）标志。只mutex 上有等待者，它就被置位
更多细节参见 Documentation/locking/rt-mutex.rst

### cmpxchg 技

有些体系结构实现了原子的 cmpxchg（比较并交换）。这（在适用时）被用来保持获取和释放
mutex 的快速路径简短
```

  unsigned long _cmpxchg(unsigned long *A, unsigned long *B, unsigned long *C)
  {
	unsigned long T = *A;
	if (*A == *B) {
		*A = *C;
	}
	return T;
  }
  #define cmpxchg(a,b,c) _cmpxchg(&a,&b,&c)

```

这真的很好用，因为它允许你只在变量等于你期望的值时才更新它。如果返回值（A 的旧值）
等于 B，你就知道它成功了
rt_mutex_cmpxchg 被用来尝试锁定和解锁 mutex。如果体系结构不支持 CMPXCHG，那么这宏就简单地被设为每次都失败。但如果支持 CMPXCHG，那么它将极大地有助于保持快速路简短
rt_mutex_cmpxchg 与属主字段中的标志一起使用，有助于为支持它的体系结构优化系统这也将在本文档后面解释

### 优先级调

rtmutex.c PI 代码的实现有几处地方，进程必须调整其优先级。在 pi_waiters 的帮助下要知道需要调整什么就相当容易了
实现 task 调整的函数是 rt_mutex_adjust_prio rt_mutex_setprio。rt_mutex_setprio
只在 rt_mutex_adjust_prio 中使用
rt_mutex_adjust_prio 检task 的优先级，以及正在等待该 task 拥有的任意一mutex 最高优先级进程。由task pi_waiters 持有由该 task 拥有的所mutex 的所top
waiter 按优先级的排序，我们只需比较 top pi waiter 与该 task 自身normal/deadline
优先级，并取较高的那个。然后调rt_mutex_setprio 将该 task 的优先级调整为新的优先级注意 rt_mutex_setprio 定义kernel/sched/core.c 中，用于实现实际的优先级变更
Note:
	对于 task_struct 中的 "prio" 字段，数值越小，优先级越高prio" 5
	"prio" 10 优先级更高
有趣的是，rt_mutex_adjust_prio 既可以提高也可以降低 task 的优先级。在较高优先级的
进程刚刚阻塞在该 task 拥有mutex 上的情况下，rt_mutex_adjust_prio 会提提升（boosttask 的优先级。但如果某个较高优先级的 task 由于某种原因离开mutex（超时或信号），
这同一个函数会降低/取消提升（unboost）该 task 的优先级。这是因pi_waiters 总是包含
正在等待task 拥有的某mutex 的最高优先级 task，所以我们只需要将top pi waiter
的优先级与该给定 task 的正常优先级进行比较

### PI 链行走的高层概览


PI 链行走由函数 rt_mutex_adjust_prio_chain 实现
该实现经历了几次迭代，最终得到了我们认为是最好的版本。它PI 链行走时一次最多只
获取两个锁，并且非常高效
rt_mutex_adjust_prio_chain 既可用于提升也可用于降低进程优先级
rt_mutex_adjust_prio_chain 被调用时，传入一个待检PI（提取消提升）的 task（某进程正阻塞在其上mutex 的属主）、一个用于死锁检测的标志、该 task 拥有mutex、一指向 waiter 的指针（即阻塞在mutex 上的进程waiter 结构体，尽管这个参数在取提升时可能为 NULL）、一个指向该 task 所阻塞mutex 的指针，以及一top_task 作为
璇?mutex 鐨?top waiter銆。
对于这个解释，我将不提及死锁检测。这个解释将尽量保持在高层次
当这个函数被调用时，没有持有任何锁。这也意味着属主和锁的状态在进入这个函数时可发生变化
在这个函数被调用之前，该 task 已经执行rt_mutex_adjust_prio。这意味着task 已被
设置为它应当处于的优先级，但task waiter rbtree 节点还没有用新优先级更新并且task 可能不在它阻塞于其上pi_waiters waiters 树中的正确位置。这个函解决了所有这些问题
这个函数的主要操作由 Thomas Gleixner rtmutex.c 中总结。更多细节参'Chain walk basics and protection scope' 注释

### 获取一mutex（详细过程）


好，现在让我们详细看看获取一mutex 时发生了什么
首先尝试的是快速获mutex。这在我们启用了 CMPXCHG 时才进行（否则快速获取自动失败）只有mutex 的属主字段为 NULL 时，才能CMPXCHG 获取锁，且无需做其他任何事
如果对锁存在争用，我们就走慢速路径（rt_mutex_slowlock）
慢速路径函数是创建 task waiter 结构（在栈上）的地方。这是因waiter 结构只在函数作用域内需要。waiter 结构持有将该 task 存入 mutex waiters 树的节点，以及（如果
需要）存入属主pi_waiters 树的节点
由于 mutex 解锁的慢速路径也获取这个锁，因此获取 mutex wait_lock
然后我们调用 try_to_take_rt_mutex。这是没有实CMPXCHG 的体系结构总会获取锁的地方
（如果没有争用）
try_to_take_rt_mutex task 于慢速路径中尝试获取一mutex 时每次都会使用。这里首做的是原子地设置 mutex 属主字段的“Has Waiters”标志。通过现在设置这个标志，正在被
争用mutex 的当前属主就不能在不进入慢速解锁路径的情况下释mutex，而那样它就需
要获取当前这段代码持有的 wait_lock。所以设置“Has Waiters”标志强制当前属主与这段
代码同步
如果满足以下条件，则获取锁：

   1) 锁没有属   2) 当前 task 相对于锁的所有其他等待者具有最高优先级

如果 task 成功获取锁，那么task 被设为锁的属主，并且如果锁仍有等待者，top_waiter
（等待在锁上的最高优先级 task）会被加入该 task pi_waiters 树
如果锁没有被 try_to_take_rt_mutex() 获取，那么就调用 task_blocks_on_rt_mutex() 函数这将把该 task 加入锁的 waiter 树，并传播锁pi 链以及锁的属主的 pi_waiters 树。这
在下一节中描述

### Task 阻塞mutex 

mutex 和进程的记账是通过进程waiter 结构完成的task" 字段被设为该进程lock"
字段被设为该 mutex。waiter rbtree 节点被初始化为该进程当前优先级
由于 wait_lock 在慢速加锁入口处已被获取，我们可以安全地waiter 加入 task waiter 树如果当前进程是正在等待该 mutex 的当前最高优先级进程，那么我们就从属主的 pi_waiters
中移除先前的 top waiter 进程（如果它存在），并将当前进程加入那棵树。由于属主的
pi_waiter 发生了变化，我们调用 rt_mutex_adjust_prio 检查属主是否应相应地调整其优先级
如果属主也阻塞在某个锁上，并且其 pi_waiters 发生了变化（或者死锁检测开启），我们就
释放 mutex wait_lock，然后继续进行属主的 rt_mutex_adjust_prio_chain，如前所述
现在所有锁都已释放，如果当前进程仍然阻塞在某个 mutex 上（waiter "task" 字段不为
NULL），那么我们就去睡眠（调schedule）

### 在循环中唤醒


task 可能因几个原因被唤醒  1) 先前的锁属主释放了锁，并且该 task 现在成为top_waiter
  2) 我们收到了信号或超时

在这两种情况下，task 都会再次尝试获取锁。如果获取到了，它就会将自己waiters 树中
移除，并将自己设TASK_RUNNING 状态
在第一种情况下，如果在task 能获取锁之前锁已被另一task 获取，那么它会回到睡并等待再次被唤醒
第二种情况只适用于那些可以在获取锁之前被唤醒task，这要么由于信号，要么由于超（即 rt_mutex_timed_futex_lock()）。被唤醒时，它会尝试再次获取锁，如果成功，则task
将持有锁返回，否则如task 被信号唤醒则返回 -EINTR，如果超时则返回 -ETIMEDOUT

### 解锁 Mutex


解锁一mutex 对于带有 CMPXCHG 的体系结构也有一条快速路径。由于争用时获取 mutex 总是
设置 mutex 属主的“Has Waiters”标志，我们利用这一点来知道在解mutex 时是否需要走
慢速路径。如mutex 没有任何等待者，mutex 的属主字段将等于当前进程，并且只需将属字段替换NULL 即可解锁 mutex
如果属主字段设置了“Has Waiters”位（或CMPXCHG 不可用），则走慢速解锁路径
慢速解锁路径中做的第一件事是获mutex wait_lock。这mutex 的加锁和解锁同步
做一个检查，mutex 是否有等待者。在没有 CMPXCHG 的体系结构上，这mutex 的属确定是否需要唤醒一个等待者的地方。在CMPXCHG 的体系结构上，该检查在快速路径中完成但在慢速路径中仍然需要。如果一mutex 的等待者因为信号或超时而在属主快速路CMPXCHG
检查失败与获取 wait_lock 之间被唤醒，mutex 可能没有任何等待者，因此属主仍然需做这个检查。如果没有等待者，那么 mutex 属主字段被设NULL，wait_lock 被释放，不需再做其他事
如果有等待者，那么我们需要唤醒其中一个
在唤醒代码中，获取当前属主的 pi_lock。找到锁top waiter，并将其mutex waiters
树以及当前属主的 pi_waiters 树中移除。“Has Waiters”位被标记，以防止较低优先级task
窃取锁
最后我们释放待定属主的 pi_lock 并唤醒它

### 联系


关于本文档的更新，请发邮件给 Steven Rostedt <rostedt@goodmis.org>


### 致谢


Author:  Steven Rostedt <rostedt@goodmis.org>

Updated: Alex Shi <alex.shi@linaro.org>	- 7/6/2017

Original Reviewers:
		     Ingo Molnar, Thomas Gleixner, Thomas Duetsch, and
		     Randy Dunlap

Update (7/6/2017) Reviewers: Steven Rostedt and Sebastian Siewior


### 更新


本文档最初为 2.6.17-rc3-mm1 而写，于 4.12 更新
