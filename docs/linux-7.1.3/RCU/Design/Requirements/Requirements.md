## RCU 需求之旅


Copyright IBM Corporation, 2015

Author: Paul E. McKenney

The initial version of this document appeared in the
`LWN <https://lwn.net/>`_ on those articles:
`part 1 <https://lwn.net/Articles/652156/>`_,
`part 2 <https://lwn.net/Articles/652677/>`_, and
`part 3 <https://lwn.net/Articles/653326/>`_.

### 引言


读-复制-更新（RCU）是一种同步机制，常被用作读写锁的替代方案。RCU 的不同寻常之处在于更新者不会阻塞读者，这意味着 RCU 的读端原语可以非常快且具备可扩展性。此外，更新者可以与读者并发地取得有用的前向进展。然而，RCU 读者与更新者之间的所有这些并发确实引出了一个疑问：RCU 读者究竟在做什么，进而又引出了 RCU 的需求究竟是什么的疑问。

因此，本文档总结了 RCU 的需求，可被视为 RCU 的一份非正式、高层次的规范。重要的是要理解，RCU 的规范本质上是经验性的；事实上，我是吃了不少苦头才了解到其中许多需求的。这种情况或许会让人有些不安，不过，这一学习过程不仅充满乐趣，而且能与众多愿意以有趣的新方式应用技术的人共事，也是极大的荣幸。

抛开这些不谈，以下是当前已知的 RCU 需求类别：

#. `Fundamental Requirements`_
#. `Fundamental Non-Requirements`_
#. `Parallelism Facts of Life`_
#. `Quality-of-Implementation Requirements`_
#. `Linux Kernel Complications`_
#. `Software-Engineering Requirements`_
#. `Other RCU Flavors`_
#. `Possible Future Changes`_

其后是一个 summary_，不过，每个快速测验的答案紧接在测验之后。用鼠标选中大片空白区域即可看到答案。

### 基本要求


RCU 的基本要求是 RCU 最接近硬性数学需求的东西。它们是：

#. `Grace-Period Guarantee`_
#. `Publish/Subscribe Guarantee`_
#. `Memory-Barrier Guarantees`_
#. `RCU Primitives Guaranteed to Execute Unconditionally`_
#. `Guaranteed Read-to-Write Upgrade`_

#### 宽限期保证


RCU 的宽限期保证之所以不同寻常，在于它是预先设想好的：Jack Slingwine 和我在 1990 年代初开始研究 RCU（当时称为“rclock”）时，就牢牢记着这一保证。话虽如此，过去二十年使用 RCU 的经验让我们对这一保证有了更为细致的理解。

RCU 的宽限期保证允许更新者等待所有既存的 RCU 读端临界区的完成。一个 RCU 读端临界区以标记 rcu_read_lock() 开始，以标记 rcu_read_unlock() 结束。这些标记可以嵌套，RCU 将一组嵌套的标记视为一个大的 RCU 读端临界区。生产质量的 rcu_read_lock() 和 rcu_read_unlock() 实现极其轻量，事实上在使用 `CONFIG_PREEMPTION=n` 构建的用于生产用途的 Linux 内核中开销为零。

这一保证使得能够以极低的开销对读者实施顺序约束，例如：

```

       1 int x, y;
       2
       3 void thread0(void)
       4 {
       5   rcu_read_lock();
       6   r1 = READ_ONCE(x);
       7   r2 = READ_ONCE(y);
       8   rcu_read_unlock();
       9 }
      10
      11 void thread1(void)
      12 {
      13   WRITE_ONCE(x, 1);
      14   synchronize_rcu();
      15   WRITE_ONCE(y, 1);
      16 }
      17
```

由于第 14 行的 synchronize_rcu() 会等待所有既存读者，任何从 `x` 加载到零值的 thread0() 实例都必须在 thread1() 向 `y` 存储之前完成，因此该实例也必须从 `y` 加载到零值。类似地，任何从 `y` 加载到一值的 thread0() 实例必定是在 synchronize_rcu() 开始之后才启动的，因此也必定会从 `x` 加载到一值。因此，如下结果：

```

     (r1 == 0 && r2 == 1)

```

不可能发生。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Wait a minute! You said that updaters can make useful forward         |
| progress concurrently with readers, but pre-existing readers will     |
| block synchronize_rcu()!!!                                            |
| Just who are you trying to fool???                                    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| First, if updaters do not wish to be blocked by readers, they can use |
| call_rcu() or kfree_rcu(), which will be discussed later.             |
| Second, even when using synchronize_rcu(), the other update-side      |
| code does run concurrently with readers, whether pre-existing or not. |
+-----------------------------------------------------------------------+

这一场景类似于 RCU 在 `DYNIX/ptx <https://en.wikipedia.org/wiki/DYNIX>`__ 中的最早用途之一，它管理一个分布式锁管理器向适合处理节点故障恢复的状态过渡，大致如下：

```

       1 #define STATE_NORMAL        0
       2 #define STATE_WANT_RECOVERY 1
       3 #define STATE_RECOVERING    2
       4 #define STATE_WANT_NORMAL   3
       5
       6 int state = STATE_NORMAL;
       7
       8 void do_something_dlm(void)
       9 {
      10   int state_snap;
      11
      12   rcu_read_lock();
      13   state_snap = READ_ONCE(state);
      14   if (state_snap == STATE_NORMAL)
      15     do_something();
      16   else
      17     do_something_carefully();
      18   rcu_read_unlock();
      19 }
      20
      21 void start_recovery(void)
      22 {
      23   WRITE_ONCE(state, STATE_WANT_RECOVERY);
      24   synchronize_rcu();
      25   WRITE_ONCE(state, STATE_RECOVERING);
      26   recovery();
      27   WRITE_ONCE(state, STATE_WANT_NORMAL);
      28   synchronize_rcu();
      29   WRITE_ONCE(state, STATE_NORMAL);
      30 }
      31
```

do_something_dlm() 中的 RCU 读端临界区与 start_recovery() 中的 synchronize_rcu() 配合，保证 do_something() 永远不会与 recovery() 并发运行，而在 do_something_dlm() 中几乎或完全没有同步开销。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why is the synchronize_rcu() on line 28 needed?                       |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Without that extra grace period, memory reordering could result in    |
| do_something_dlm() executing do_something() concurrently with         |
| the last bits of recovery().                                          |
+-----------------------------------------------------------------------+

为了避免死锁等致命问题，RCU 读端临界区不能包含对 synchronize_rcu() 的调用。类似地，RCU 读端临界区不能包含任何直接或者间接等待某个 synchronize_rcu() 调用完成的内容。

尽管 RCU 的宽限期保证本身很有用（有 `quite a few use cases <https://lwn.net/Articles/573497/>`__），但若是能用 RCU 来协调对链表数据结构的读端访问就好了。对此，宽限期保证并不充分，如下面的 add_gp_buggy() 函数所示。我们稍后会看到读者的代码，但在此期间，只需把读者看作无锁地取走 `gp` 指针，并且如果加载到的值不是 `NULL`，就无锁地访问 `->a` 和 `->b` 字段。

```

       1 bool add_gp_buggy(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   p->a = a;
      12   p->b = a;
      13   gp = p; /* ORDERING BUG */
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

问题在于，编译器和弱序 CPU 都有权将这段代码重排如下：

```

       1 bool add_gp_buggy_optimized(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   gp = p; /* ORDERING BUG */
      12   p->a = a;
      13   p->b = a;
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

如果某个 RCU 读者在 `add_gp_buggy_optimized` 执行第 11 行后立刻取走 `gp`，它会看到 `->a` 和 `->b` 字段中的垃圾数据。而这只是编译器和硬件优化可能造成麻烦的众多方式之一。因此，我们显然需要某种方式来阻止编译器和 CPU 以这种方式重排，这就引出了下一节讨论的发布-订阅保证。

#### 发布/订阅保证


RCU 的发布-订阅保证允许在不打扰 RCU 读者的情况下，将数据插入到链表数据结构中。更新者使用 rcu_assign_pointer() 插入新数据，读者使用 rcu_dereference() 访问数据（无论是新的还是旧的）。下面给出一个插入示例：

```

       1 bool add_gp(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   p->a = a;
      12   p->b = a;
      13   rcu_assign_pointer(gp, p);
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

第 13 行的 rcu_assign_pointer() 在概念上等价于一条简单的赋值语句，但同时也保证其赋值会发生在第 11 行和第 12 行两次赋值之后，类似于 C11 `memory_order_release` 存储操作。它还能阻止任何数量的“有趣”的编译器优化，例如，在赋值之前把 `gp` 用作临时存放位置。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But rcu_assign_pointer() does nothing to prevent the two              |
| assignments to `p->a` and `p->b` from being reordered. Can't that |    
| also cause problems?                                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| No, it cannot. The readers cannot see either of these two fields      |
| until the assignment to `gp`, by which time both fields are fully   |  
| initialized. So reordering the assignments to `p->a` and `p->b`   |    
| cannot possibly cause any problems.                                   |
+-----------------------------------------------------------------------+

人们很容易想当然地认为，读者无需做任何特殊处理来控制其对 RCU 保护数据的访问，如下面的 do_something_gp_buggy() 所示：

```

       1 bool do_something_gp_buggy(void)
       2 {
       3   rcu_read_lock();
       4   p = gp;  /* OPTIMIZATIONS GALORE!!! */
       5   if (p) {
       6     do_something(p->a, p->b);
       7     rcu_read_unlock();
       8     return true;
       9   }
      10   rcu_read_unlock();
      11   return false;
      12 }
      13
```

然而，必须抵制这种诱惑，因为编译器（或像 DEC Alpha 这样的弱序 CPU）有数量惊人之多的方式会让这段代码出错。仅举一例：如果编译器寄存器短缺，它可能会选择从 `gp` 重新获取，而不是在 `p` 中保留单独的一份副本，如下所示：

```

       1 bool do_something_gp_buggy_optimized(void)
       2 {
       3   rcu_read_lock();
       4   if (gp) { /* OPTIMIZATIONS GALORE!!! */
       5     do_something(gp->a, gp->b);
       6     rcu_read_unlock();
       7     return true;
       8   }
       9   rcu_read_unlock();
      10   return false;
      11 }
      12
```
如果这个函数与一系列将当前结构替换为新结构的更新并发运行，对 `gp->a` 和
`gp->b` 的取用很可能来自两个不同的结构，这会造成严重的混乱。为防止这种情况（以及许多其他情况），
do_something_gp() 使用 rcu_dereference() 从 `gp` 取用：

```

       1 bool do_something_gp(void)
       2 {
       3   rcu_read_lock();
       4   p = rcu_dereference(gp);
       5   if (p) {
       6     do_something(p->a, p->b);
       7     rcu_read_unlock();
       8     return true;
       9   }
      10   rcu_read_unlock();
      11   return false;
      12 }
      13
```

rcu_dereference() 在 Linux 内核中使用 volatile 类型转换以及（对于 DEC Alpha）内存屏障。倘若将来出现了 |high-quality implementation of C11 memory_order_consume [PDF]|_，那么 rcu_dereference() 就可以实现为一个 `memory_order_consume` 加载。无论具体实现如何，由 rcu_dereference() 取到的指针都不能在其所在的（最外层的）RCU 读端临界区之外使用，除非相应数据元素的保护已从 RCU 转交给其他某种同步机制，最常见的是锁或引用计数（see ../../rcuref.rst）。


简而言之，更新者使用 rcu_assign_pointer()，读者使用 rcu_dereference()，这两个 RCU API 元素协同工作，确保读者对新添加的数据元素有一致的视图。

当然，还需要从 RCU 保护的数据结构中移除元素，例如使用如下过程：

#. 将数据元素从外层结构中移除。
#. 等待所有既存的 RCU 读端临界区完成（因为只有既存读者才可能持有对新移除数据元素的引用）。
#. 此时，只有更新者持有对新移除数据元素的引用，因此它可以安全地回收该数据元素，例如将其传递给 kfree()。

这一过程由 remove_gp_synchronous() 实现：

```

       1 bool remove_gp_synchronous(void)
       2 {
       3   struct foo *p;
       4
       5   spin_lock(&gp_lock);
       6   p = rcu_access_pointer(gp);
       7   if (!p) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   rcu_assign_pointer(gp, NULL);
      12   spin_unlock(&gp_lock);
      13   synchronize_rcu();
      14   kfree(p);
      15   return true;
      16 }
      17
```

这个函数很简单，第 13 行在释放旧数据元素（第 14 行）之前等待一个宽限期。这一等待确保读者会在被 `p` 引用的数据元素被释放之前到达 do_something_gp() 的第 7 行。第 6 行的 rcu_access_pointer() 类似于 rcu_dereference()，区别在于：

#. rcu_access_pointer() 返回的值不能被解引用。如果你想同时访问所指向的值和指针本身，请使用 rcu_dereference() 而不是 rcu_access_pointer()。
#. 对 rcu_access_pointer() 的调用无需受到保护。相比之下，rcu_dereference() 必须位于 RCU 读端临界区内，或者位于指针不会变化的代码段中，例如受相应更新端锁保护的代码。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Without the rcu_dereference() or the rcu_access_pointer(),            |
| what destructive optimizations might the compiler make use of?        |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Let's start with what happens to do_something_gp() if it fails to     |
| use rcu_dereference(). It could reuse a value formerly fetched        |
| from this same pointer. It could also fetch the pointer from `gp`   |  
| in a byte-at-a-time manner, resulting in **load tearing**, in turn      |
| resulting a bytewise mash-up of two distinct pointer values. It might |
| even use value-speculation optimizations, where it makes a wrong      |
| guess, but by the time it gets around to checking the value, an       |
| update has changed the pointer to match the wrong guess. Too bad      |
| about any dereferences that returned pre-initialization garbage in    |
| the meantime!                                                         |
| For remove_gp_synchronous(), as long as all modifications to          |
| `gp` are carried out while holding `gp_lock`, the above           |    
| optimizations are harmless. However, `sparse` will complain if you  |  
| define `gp` with `__rcu` and then access it without using either  |    
| rcu_access_pointer() or rcu_dereference().                            |
+-----------------------------------------------------------------------+

简而言之，RCU 的发布-订阅保证由 rcu_assign_pointer() 和 rcu_dereference() 的组合提供。这一保证允许将数据元素安全地添加到 RCU 保护的链表数据结构中而不打扰 RCU 读者。这一保证可与宽限期保证结合使用，从而也允许将数据元素从 RCU 保护的链表数据结构中移除，同样不打扰 RCU 读者。

这一保证只是部分地预先设想过。DYNIX/ptx 在发布时使用了一条显式内存屏障，但在订阅时没有任何类似 rcu_dereference() 的东西，也没有任何类似后来被纳入 rcu_dereference()、再后来又被纳入 READ_ONCE() 的依赖顺序屏障的东西。对于这些操作的需求，是在 1990 年代末与 DEC Alpha 架构师的一次会议上突然显现出来的，那时 DEC 还是一家独立的公司。Alpha 架构师花了足足一个小时才让我相信竟然会需要任何种类的屏障，而我随后又花了足足**两**个小时才让他们相信他们的文档并没有把这一点讲清楚。近些年来与 C 和 C++ 标准委员会的合作，让人们从编译器那里学到了很多技巧和陷阱。简而言之，在 1990 年代初编译器还没那么刁钻，但在 2015 年，千万别想着省略 rcu_dereference()！

#### 内存屏障保证


上一节简单的链表数据结构场景清楚地展示了，在拥有多个 CPU 的系统上，为何需要 RCU 严格的内存顺序保证：

#. 每个拥有在某个 synchronize_rcu() 开始之前开始的 RCU 读端临界区的 CPU，都保证在该 RCU 读端临界区结束与该 synchronize_rcu() 返回之间的某个时刻执行一条完整内存屏障。没有这一保证，一个既存的 RCU 读端临界区可能会在 remove_gp_synchronous() 第 14 行的 kfree() 之后仍然持有对新移除的 `struct foo` 的引用。
#. 每个拥有在某个 synchronize_rcu() 返回之后结束的 RCU 读端临界区的 CPU，都保证在 synchronize_rcu() 开始与该 RCU 读端临界区开始之间的某个时刻执行一条完整内存屏障。没有这一保证，在 remove_gp_synchronous() 第 14 行的 kfree() 之后运行的、更晚的 RCU 读端临界区，稍后可能会运行 do_something_gp() 并找到新删除的 `struct foo`。
#. 如果调用 synchronize_rcu() 的任务停留在某个给定的 CPU 上，那么该 CPU 保证在 synchronize_rcu() 执行期间的某个时刻执行一条完整内存屏障。这一保证确保 remove_gp_synchronous() 第 14 行的 kfree() 确实在第 11 行的移除之后执行。
#. 如果调用 synchronize_rcu() 的任务在这次调用期间在一组 CPU 之间迁移，那么该组中的每个 CPU 都保证在 synchronize_rcu() 执行期间的某个时刻执行一条完整内存屏障。这一保证同样确保 remove_gp_synchronous() 第 14 行的 kfree() 确实在第 11 行的移除之后执行，而且涵盖了执行 synchronize_rcu() 的任务在此期间发生迁移的情况。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Given that multiple CPUs can start RCU read-side critical sections at |
| any time without any ordering whatsoever, how can RCU possibly tell   |
| whether or not a given RCU read-side critical section starts before a |
| given instance of synchronize_rcu()?                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| If RCU cannot tell whether or not a given RCU read-side critical      |
| section starts before a given instance of synchronize_rcu(), then     |
| it must assume that the RCU read-side critical section started first. |
| In other words, a given instance of synchronize_rcu() can avoid       |
| waiting on a given RCU read-side critical section only if it can      |
| prove that synchronize_rcu() started first.                           |
| A related question is “When rcu_read_lock() doesn't generate any      |
| code, why does it matter how it relates to a grace period?” The       |
| answer is that it is not the relationship of rcu_read_lock()          |
| itself that is important, but rather the relationship of the code     |
| within the enclosed RCU read-side critical section to the code        |
| preceding and following the grace period. If we take this viewpoint,  |
| then a given RCU read-side critical section begins before a given     |
| grace period when some access preceding the grace period observes the |
| effect of some access within the critical section, in which case none |
| of the accesses within the critical section may observe the effects   |
| of any access following the grace period.                             |
|                                                                       |
| As of late 2016, mathematical models of RCU take this viewpoint, for  |
| example, see slides 62 and 63 of the `2016 LinuxCon                   |
| EU <http://www2.rdrop.com/users/paulmck/scalability/paper/LinuxMM.201 |
| 6.10.04c.LCE.pdf>`__                                                  |
| presentation.                                                         |
+-----------------------------------------------------------------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| The first and second guarantees require unbelievably strict ordering! |
| Are all these memory barriers **really** required?                      |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Yes, they really are required. To see why the first guarantee is      |
| required, consider the following sequence of events:                  |
|                                                                       |
| #. CPU 1: rcu_read_lock()                                             |
| #. CPU 1: `q = rcu_dereference(gp); /** Very likely to return p. **/` |
| #. CPU 0: `list_del_rcu(p);`                                        |  
| #. CPU 0: synchronize_rcu() starts.                                   |
| #. CPU 1: `do_something_with(q->a);`                                |  
|    `/** No smp_mb(), so might happen after kfree(). **/`              |
| #. CPU 1: rcu_read_unlock()                                           |
| #. CPU 0: synchronize_rcu() returns.                                  |
| #. CPU 0: `kfree(p);`                                               |  
|                                                                       |
| Therefore, there absolutely must be a full memory barrier between the |
| end of the RCU read-side critical section and the end of the grace    |
| period.                                                               |
|                                                                       |
| The sequence of events demonstrating the necessity of the second rule |
| is roughly similar:                                                   |
|                                                                       |
| #. CPU 0: `list_del_rcu(p);`                                        |  
| #. CPU 0: synchronize_rcu() starts.                                   |
| #. CPU 1: rcu_read_lock()                                             |
| #. CPU 1: `q = rcu_dereference(gp);`                                |  
|    `/** Might return p if no memory barrier. **/`                     |
| #. CPU 0: synchronize_rcu() returns.                                  |
| #. CPU 0: `kfree(p);`                                               |  
| #. CPU 1: `do_something_with(q->a); /** Boom!!! **/`                  |
| #. CPU 1: rcu_read_unlock()                                           |
|                                                                       |
| And similarly, without a memory barrier between the beginning of the  |
| grace period and the beginning of the RCU read-side critical section, |
| CPU 1 might end up accessing the freelist.                            |
|                                                                       |
| The "as if" rule of course applies, so that any implementation that   |
| acts as if the appropriate memory barriers were in place is a correct |
| implementation. That said, it is much easier to fool yourself into    |
| believing that you have adhered to the as-if rule than it is to       |
| actually adhere to it!                                                |
+-----------------------------------------------------------------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| You claim that rcu_read_lock() and rcu_read_unlock() generate         |
| absolutely no code in some kernel builds. This means that the         |
| compiler might arbitrarily rearrange consecutive RCU read-side        |
| critical sections. Given such rearrangement, if a given RCU read-side |
| critical section is done, how can you be sure that all prior RCU      |
| read-side critical sections are done? Won't the compiler              |
| rearrangements make that impossible to determine?                     |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| In cases where rcu_read_lock() and rcu_read_unlock() generate         |
| absolutely no code, RCU infers quiescent states only at special       |
| locations, for example, within the scheduler. Because calls to        |
| schedule() had better prevent calling-code accesses to shared         |
| variables from being rearranged across the call to schedule(), if     |
| RCU detects the end of a given RCU read-side critical section, it     |
| will necessarily detect the end of all prior RCU read-side critical   |
| sections, no matter how aggressively the compiler scrambles the code. |
| Again, this all assumes that the compiler cannot scramble code across |
| calls to the scheduler, out of interrupt handlers, into the idle      |
| loop, into user-mode code, and so on. But if your kernel build allows |
| that sort of scrambling, you have broken far more than just RCU!      |
+-----------------------------------------------------------------------+

注意，这些内存屏障需求并不能取代 RCU 的基本需求，即宽限期要等待所有既存读者。恰恰相反，本节指出的内存屏障必须以**强制**这一基本需求的方式运作。当然，不同的实现以不同的方式强制这一需求，但它们必须强制。

#### 保证无条件执行的 RCU 原语


常见情形的 RCU 原语是无条件的。它们被调用、完成工作、然后返回，不可能出错，也无需重试。这是 RCU 的一项关键设计哲学。

然而，这一哲学是务实的而非固执的。如果有人能为某个特定的条件式 RCU 原语提出合理的理由，它很可能会被实现并加入。毕竟，这一保证是逆向推导出来的，而非预先设想。RCU 原语的无条件特性最初只是实现上的一个意外，后来与带有条件式原语的同步原语打交道的经验，促使我把这一偶然提升为保证。因此，向 RCU 添加条件式原语的理由需要建立在详尽且令人信服的用例之上。

#### 保证从读到写的升级


就 RCU 而言，在 RCU 读端临界区内执行一次更新总是可行的。例如，该 RCU 读端临界区可能会搜索某个给定的数据元素，然后获取更新端自旋锁以更新该元素，而整个过程都留在该 RCU 读端临界区内。当然，在调用 synchronize_rcu() 之前必须先退出 RCU 读端临界区，不过，这一不便可以通过使用本文档后面介绍的 call_rcu() 和 kfree_rcu() API 来避免。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But how does the upgrade-to-write operation exclude other readers?    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| It doesn't, just like normal RCU updates, which also do not exclude   |
| RCU readers.                                                          |
+-----------------------------------------------------------------------+

这一保证允许在只读端和更新端代码之间共享查找代码，并且是预先设想过的，出现在最早的 DYNIX/ptx RCU 文档中。

### 基本非需求


RCU 提供极其轻量的读者，它的读端保证虽然相当有用，但相应地也很轻量。因此，人们太容易假定 RCU 所保证的比它实际保证的更多。当然，RCU 不保证的事情的清单是无限长的，不过，以下几节列出了一些曾引起困惑的非保证。除另有说明外，这些非保证都是预先设想过的。

#. `Readers Impose Minimal Ordering`_
#. `Readers Do Not Exclude Updaters`_
#. `Updaters Only Wait For Old Readers`_
#. `Grace Periods Don't Partition Read-Side Critical Sections`_
#. `Read-Side Critical Sections Don't Partition Grace Periods`_
#### 读者施加最小的顺序约束


读端标记如 rcu_read_lock() 和 rcu_read_unlock() 除了通过与 synchronize_rcu() 这类宽限期 API 的交互之外，绝对不提供任何顺序保证。要明白这一点，请看下面这对线程：

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(x, 1);
       5   rcu_read_unlock();
       6   rcu_read_lock();
       7   WRITE_ONCE(y, 1);
       8   rcu_read_unlock();
       9 }
      10
      11 void thread1(void)
      12 {
      13   rcu_read_lock();
      14   r1 = READ_ONCE(y);
      15   rcu_read_unlock();
      16   rcu_read_lock();
      17   r2 = READ_ONCE(x);
      18   rcu_read_unlock();
      19 }
      20
```

在 thread0() 和 thread1() 并发执行之后，很可能出现

```

     (r1 == 1 && r2 == 0)

```

（也就是说，`y` 看起来是在 `x` 之前被赋值的），如果 rcu_read_lock() 和 rcu_read_unlock() 具有较多的顺序特性，这是不可能的。但它们没有，因此 CPU 完全有权进行显著的重新排序。这是设计使然：任何显著的顺序约束都会拖慢这些快速路径 API。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Can't the compiler also reorder this code?                            |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| No, the volatile casts in READ_ONCE() and WRITE_ONCE()                |
| prevent the compiler from reordering in this particular case.         |
+-----------------------------------------------------------------------+

#### 读者不排除更新者


rcu_read_lock() 和 rcu_read_unlock() 都不排除更新。它们所做的全部事情，只是阻止宽限期结束。下面的例子说明了这一点：

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   r1 = READ_ONCE(y);
       5   if (r1) {
       6     do_something_with_nonzero_x();
       7     r2 = READ_ONCE(x);
       8     WARN_ON(!r2); /* BUG!!! */
       9   }
      10   rcu_read_unlock();
      11 }
      12
      13 void thread1(void)
      14 {
      15   spin_lock(&my_lock);
      16   WRITE_ONCE(x, 1);
      17   WRITE_ONCE(y, 1);
      18   spin_unlock(&my_lock);
      19 }
      20
```

如果 thread0() 函数的 rcu_read_lock() 排除了 thread1() 函数的更新，那么 WARN_ON() 就永远不会触发。但事实是，除了后续的宽限期之外，rcu_read_lock() 几乎不排除任何东西，而 thread1() 没有任何宽限期，因此 WARN_ON() 能够并且确实会触发。

#### 更新者只等待旧读者


人们很容易想当然地认为，在 synchronize_rcu() 完成之后，就没有读者在执行了。必须抵制这种诱惑，因为新的读者可以在 synchronize_rcu() 开始之后立即启动，而 synchronize_rcu() 没有义务等待这些新读者。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Suppose that synchronize_rcu() did wait until **all** readers had       |
| completed instead of waiting only on pre-existing readers. For how    |
| long would the updater be able to rely on there being no readers?     |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| For no time at all. Even if synchronize_rcu() were to wait until      |
| all readers had completed, a new reader might start immediately after |
| synchronize_rcu() completed. Therefore, the code following            |
| synchronize_rcu() can **never** rely on there being no readers.         |
+-----------------------------------------------------------------------+

#### 宽限期不会分割读端临界区


人们很容易想当然地认为：如果某个 RCU 读端临界区的任何部分位于给定宽限期之前，而另一个 RCU 读端临界区的任何部分位于同一宽限期之后，那么整个第一个 RCU 读端临界区必定位于整个第二个之前。然而，事实并非如此：单个宽限期并不会对 RCU 读端临界区的集合进行分割。这种情况可以如下说明，其中 `x`、`y` 和 `z` 初始都为零：

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   rcu_read_lock();
      19   r2 = READ_ONCE(b);
      20   r3 = READ_ONCE(c);
      21   rcu_read_unlock();
      22 }
      23
```

结果

```

     (r1 == 1 && r2 == 0 && r3 == 1)

```

完全可能出现。下图展示了这是如何发生的，其中每个带圈 `QS` 表示 RCU 为该线程记录**静止状态**（quiescent state）的时刻，也就是 RCU 知道该线程不可能处于在当前宽限期之前开始的 RCU 读端临界区之中的状态：


如果确实有必要以这种方式分割 RCU 读端临界区，就必须使用两个宽限期，其中第一个宽限期已知在第二个宽限期开始之前结束：

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   r2 = READ_ONCE(c);
      19   synchronize_rcu();
      20   WRITE_ONCE(d, 1);
      21 }
      22
      23 void thread3(void)
      24 {
      25   rcu_read_lock();
      26   r3 = READ_ONCE(b);
      27   r4 = READ_ONCE(d);
      28   rcu_read_unlock();
      29 }
      30
```

这里，如果 `(r1 == 1)`，那么 thread0() 对 `b` 的写入必定发生在 thread1() 的宽限期结束之前。如果此外还有 `(r4 == 1)`，那么 thread3() 对 `b` 的读取必定发生在 thread2() 的宽限期开始之后。如果同时还有 `(r2 == 1)`，那么 thread1() 的宽限期结束必定早于 thread2() 的宽限期开始。这意味着两个 RCU 读端临界区不能重叠，从而保证 `(r3 == 1)`。因此，结果

```

     (r1 == 1 && r2 == 1 && r3 == 0 && r4 == 1)

```

不可能发生。

这一非需求同样不是预先设想的，而是在研究 RCU 与内存序的交互时变得明显的。

#### 读端临界区不会分割宽限期


人们同样很容易想当然地认为：如果一个 RCU 读端临界区位于一对宽限期之间，那么那些宽限期就不能重叠。然而，这种诱惑只会把人引向歧途，正如下面的例子所示，所有变量初始都为零：

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   rcu_read_lock();
      19   WRITE_ONCE(d, 1);
      20   r2 = READ_ONCE(c);
      21   rcu_read_unlock();
      22 }
      23
      24 void thread3(void)
      25 {
      26   r3 = READ_ONCE(d);
      27   synchronize_rcu();
      28   WRITE_ONCE(e, 1);
      29 }
      30
      31 void thread4(void)
      32 {
      33   rcu_read_lock();
      34   r4 = READ_ONCE(b);
      35   r5 = READ_ONCE(e);
      36   rcu_read_unlock();
      37 }
      38
```

在这种情况下，结果

```

     (r1 == 1 && r2 == 1 && r3 == 1 && r4 == 0 && r5 == 1)

```

完全可能出现，如下图所示：


同样，一个 RCU 读端临界区可以几乎与整个给定宽限期重叠，只要它不与整个宽限期完全重叠即可。因此，一个 RCU 读端临界区无法分割一对 RCU 宽限期。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| How long a sequence of grace periods, each separated by an RCU        |
| read-side critical section, would be required to partition the RCU    |
| read-side critical sections at the beginning and end of the chain?    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| In theory, an infinite number. In practice, an unknown number that is |
| sensitive to both implementation details and timing considerations.   |
| Therefore, even in practice, RCU users must abide by the theoretical  |
| rather than the practical answer.                                     |
+-----------------------------------------------------------------------+

### 并行生活的现实


这些并行生活的现实绝不仅限于 RCU，但 RCU 的实现必须遵守它们。因此它们值得重申：

#. 任何 CPU 或任务都可能在任何时候被延迟，而任何试图通过禁用抢占、中断或其他手段来避免这些延迟的做法都是完全徒劳的。这在可抢占的用户态环境以及虚拟化环境（其中给定客户机操作系统的 VCPU 随时可能被底层 hypervisor 抢占）中最明显，但也可能由于 ECC 错误、NMI 以及其他硬件事件而发生在裸机环境中。尽管超过约 20 秒的延迟可能导致 splat，但 RCU 实现有义务使用能够容忍极长延迟的算法，只不过这里的“极长”还不够长到让 64 位计数器在递增时发生回绕。
#. 编译器和 CPU 都可能对内存访问进行重排。在重要的地方，RCU 必须使用编译器指令和内存屏障指令来保持顺序。
#. 对任一给定缓存行中内存位置的相互冲突的写入会导致昂贵的缓存未命中。更多数量的并发写入以及更频繁的并发写入会导致更严重的减速。因此 RCU 有义务使用具备足够局部性的算法，以避免显著的性能和可扩展性问题。
#. 作为一条粗略的经验法则，在任何给定排他锁的保护下，只能执行相当于一个 CPU 的处理量。因此 RCU 必须使用可扩展的加锁设计。
#. 计数器是有限的，在 32 位系统上尤其如此。因此 RCU 对计数器的使用必须能够容忍计数器回绕，或者被设计成计数器回绕所需的时间远超过单个系统可能运行的时间。十年的正常运行时间相当可能，一个世纪的运行时间则远不可能。作为后者的一个例子，RCU 的 dyntick-idle 嵌套计数器为中断嵌套层级保留了 54 位（即使在 32 位系统上，该计数器也是 64 位）。让该计数器溢出需要某个 CPU 在不曾进入空闲的情况下发生 2\ `54` 次半中断。如果每微秒发生一次半中断，那么需要 570 年的运行时间才会让该计数器溢出，目前这被认为是一段可以接受的长久时间。
#. Linux 系统可以在单个共享内存环境中让数千个 CPU 运行同一个 Linux 内核。因此 RCU 必须密切关注高端的可扩展性。
这最后一条并行生活的现实意味着 RCU 必须特别留意前述那些现实。Linux 能够扩展到拥有数千个 CPU 的系统的想法，在 1990 年代或许会遭到一些怀疑，但除此之外，这些要求并不出人意料，即便在 1990 年代初也是如此。

### 实现质量需求


以下各节列出了实现质量方面的需求。尽管一个忽略这些需求的 RCU 实现仍然可以使用，但它很可能会受到种种限制，从而不适合工业级的生产经营使用。实现质量需求的类别如下：

#. `Specialization`_
#. `Performance and Scalability`_
#. `Forward Progress`_
#. `Composability`_
#. `Corner Cases`_

这些类别将在以下各节中分别介绍。

#### 专门化


RCU 过去是、现在也主要面向以读为主（read-mostly）的场景，这意味着 RCU 的读端原语经过了优化，往往以牺牲其更新端原语为代价。迄今的经验可由下面列出的情形概括：

#. 以读为主的数据，且过期和不一致的数据不成问题：RCU 表现极佳！
#. 以读为主的数据，且数据必须保持一致：RCU 表现良好。
#. 读写兼有的数据，且数据必须保持一致：RCU **可能**还行。也可能不行。
#. 以写为主的数据，且数据必须保持一致：RCU 极不可能是合适的工具，但有以下例外，在这些情况下 RCU 可以提供：

   a. 对更新友好的机制提供存在性保证。
   b. 为实时用途提供无等待（wait-free）的读端原语。

这种以读为主的取向意味着 RCU 必须与其他同步原语互通。例如，前面讨论的 add_gp() 和 remove_gp_synchronous() 示例使用 RCU 保护读者、用锁来协调更新者。然而，需求远不止于此，它要求各种各样的同步原语在 RCU 读端临界区内都是合法的，包括自旋锁、顺序锁、原子操作、引用计数器和内存屏障。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| What about sleeping locks?                                            |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| These are forbidden within Linux-kernel RCU read-side critical        |
| sections because it is not legal to place a quiescent state (in this  |
| case, voluntary context switch) within an RCU read-side critical      |
| section. However, sleeping locks may be used within userspace RCU     |
| read-side critical sections, and also within Linux-kernel sleepable   |
| RCU `(SRCU) <Sleepable RCU_>`__ read-side critical sections. In       |
| addition, the -rt patchset turns spinlocks into a sleeping locks so   |
| that the corresponding critical sections can be preempted, which also |
| means that these sleeplockified spinlocks (but not other sleeping     |
| locks!) may be acquire within -rt-Linux-kernel RCU read-side critical |
| sections.                                                             |
| Note that it **is** legal for a normal RCU read-side critical section   |
| to conditionally acquire a sleeping locks (as in                      |
| mutex_trylock()), but only as long as it does not loop                |
| indefinitely attempting to conditionally acquire that sleeping locks. |
| The key point is that things like mutex_trylock() either return       |
| with the mutex held, or return an error indication if the mutex was   |
| not immediately available. Either way, mutex_trylock() returns        |
| immediately without sleeping.                                         |
+-----------------------------------------------------------------------+

许多人会惊讶地发现，很多算法并不要求数据视图一致，但许多算法确实能在那种模式下工作，网络路由就是最典型的例子。互联网路由算法传播更新需要相当长的时间，因此当一次更新到达某个系统时，该系统已经把网络流量往错误的方向发送了相当长一段时间。让少数几个线程再往错误的方向多发几毫秒的流量显然不是问题：在最坏情况下，TCP 重传最终会把数据送到它该去的地方。一般而言，在追踪计算机之外的宇宙状态时，由于光速延迟（即使不考虑其他原因），某种程度的不一致是必须容忍的。

此外，对外部状态的不确定性在许多情况下是固有的。例如，一对兽医可能会用心跳来判断一只给定的猫是否还活着。但在最后一次心跳之后，他们应该等多久才断定这只猫确实死了？等待少于 400 毫秒毫无意义，因为那意味着一只放松的猫会被认为每分钟在死亡与存活之间反复超过 100 次。而且，就像人一样，猫的心脏可能会停跳一段时间，所以确切的等待时长是一个判断问题。我们这对兽医中的一位可能会在宣布猫死亡之前等 30 秒，而另一位可能坚持要等满一分钟。于是，在最后一次心跳之后那一分钟里的最后 30 秒中，两位兽医会对猫的状态意见不一。

有趣的是，同样的状况也适用于硬件。到了关键时刻，我们如何判断某个外部服务器是否已经故障？我们会周期性地向它发送消息，如果在给定时间内没有收到响应，就宣布它故障。策略决策通常能够容忍短时间的不一致。策略是一段时间之前定下的，现在才付诸实施，因此几毫秒的延迟通常无关紧要。

然而，有些算法绝对必须看到一致的数据。例如，用户态 SystemV 信号量 ID 到相应内核数据结构的转换由 RCU 保护，但绝对禁止更新一个刚刚被移除的信号量。在 Linux 内核中，这种一致性需求是通过在 RCU 读端临界区内获取位于内核数据结构中的自旋锁来满足的，这由上图中的绿框所标明。许多其他技术也可能、并且事实上在 Linux 内核中确实被使用。

简而言之，RCU 不负责维护一致性，当需要一致性时，可以与其他机制配合 RCU 一起使用。RCU 的专门化让它把本职工作做得极好，而它与其他同步机制互通的能力，使得针对给定任务使用恰当的同步工具组合成为可能。

#### 性能与可扩展性


能效是当今性能的一个关键组成部分，因此 Linux 内核的 RCU 实现必须避免不必要地唤醒空闲 CPU。我不能声称这一需求是预先设想过的。事实上，我是在一次电话交谈中了解到它的，在那次交谈中，我得到了关于电池供电系统中能效重要性、以及 Linux 内核 RCU 实现在具体能效缺陷方面“坦诚而开明”的反馈。据我的经验，电池供电的嵌入式社区会把任何不必要的唤醒视为极不友好的行为。以至于仅仅在 Linux 内核邮件列表上发帖都不足以宣泄他们的怒火。

内存在大多数情况下并非特别重要，并且随着内存容量扩大、内存价格暴跌，它的重要性还在下降。然而，正如我从 Matt Mackall 的 `bloatwatch <http://elinux.org/Linux_Tiny-FAQ>`__ 工作中所学到的，内存在带有非可抢占（`CONFIG_PREEMPTION=n`）内核的单 CPU 系统上至关重要，于是 `tiny RCU <https://lore.kernel.org/r/20090113221724.GA15307@linux.vnet.ibm.com>`__ 应运而生。此后，Josh Triplett 接过小型内存的大旗，发起他的 `Linux kernel tinification <https://tiny.wiki.kernel.org/>`__ 项目，这使得 `SRCU <Sleepable RCU_>`__ 对于那些不需要它的内核成为可选项。

其余的性能需求在大多数情况下都不出人意料。例如，与 RCU 的读端专门化相一致，rcu_dereference() 应当有可忽略的开销（例如，抑制少数轻微的编译器优化）。类似地，在非可抢占环境中，rcu_read_lock() 和 rcu_read_unlock() 应当有确切为零的开销。

在可抢占环境中，对于未被抢占的 RCU 读端临界区（最高优先级的实时进程就是这种情况），rcu_read_lock() 和 rcu_read_unlock() 应当有最小的开销。特别地，它们不应包含原子读-修改-写操作、内存屏障指令、禁用抢占、禁用中断或向后分支。然而，对于被抢占的 RCU 读端临界区，rcu_read_unlock() 可以获取自旋锁并禁用中断。这就是为什么至少在被抢占的实时延迟影响可控（即临界区足够短）的情况下，把 RCU 读端临界区嵌套在禁用抢占区域内、而非相反，是更好的做法。

synchronize_rcu() 宽限期等待原语是针对吞吐量优化的。因此，除了最长 RCU 读端临界区的持续时间之外，它可能还会带来若干毫秒的延迟。另一方面，多个并发的 synchronize_rcu() 调用必须运用批处理优化，使得它们能由一个底层宽限期等待操作来满足。例如，在 Linux 内核中，单次宽限期等待操作服务于超过 `1,000 separate invocations <https://www.usenix.org/conference/2004-usenix-annual-technical-conference/making-rcu-safe-deep-sub-millisecond-response>`__ 的 synchronize_rcu() 并不罕见，从而把每次调用的开销分摊到接近于零。然而，宽限期优化同时也必须避免实时调度和中断延迟出现可测量的退化。

在某些情况下，数毫秒级的 synchronize_rcu() 延迟是不可接受的。在这些情况下，可以使用 synchronize_rcu_expedited() 来替代，在小型系统上把宽限期延迟降低到几十微秒（至少在读端临界区较短的情况下）。目前对于大型系统上的 synchronize_rcu_expedited() 没有特殊的延迟需求，但是，与 RCU 规范的经验性本质一致，这一点将来可能会改变。不过，可扩展性需求是确凿无疑的：在 4096 个 CPU 上突如其来的一波 synchronize_rcu_expedited() 调用至少应当取得合理的向前进展。作为较短延迟的回报，synchronize_rcu_expedited() 被允许对非空闲的在线 CPU 施加适度的实时延迟退化。这里的“适度”大致等同于一次调度时钟中断带来的延迟退化。

还有些情况，连 synchronize_rcu_expedited() 降低后的宽限期延迟也不可接受。在这些情况下，可以使用异步的 call_rcu() 来替代 synchronize_rcu()，如下所示：

```

       1 struct foo {
       2   int a;
       3   int b;
       4   struct rcu_head rh;
       5 };
       6
       7 static void remove_gp_cb(struct rcu_head *rhp)
       8 {
       9   struct foo *p = container_of(rhp, struct foo, rh);
      10
      11   kfree(p);
      12 }
      13
      14 bool remove_gp_asynchronous(void)
      15 {
      16   struct foo *p;
      17
      18   spin_lock(&gp_lock);
      19   p = rcu_access_pointer(gp);
      20   if (!p) {
      21     spin_unlock(&gp_lock);
      22     return false;
      23   }
      24   rcu_assign_pointer(gp, NULL);
      25   call_rcu(&p->rh, remove_gp_cb);
      26   spin_unlock(&gp_lock);
      27   return true;
      28 }
      29
```

终于需要一个 `struct foo` 的定义了，它出现在第 1-5 行。函数 remove_gp_cb() 在第 25 行被传给 call_rcu()，并将在后续一个宽限期结束之后被调用。这达到了与 remove_gp_synchronous() 相同的效果，但无需强迫更新者等待宽限期过去。call_rcu() 函数可以用在 synchronize_rcu() 和 synchronize_rcu_expedited() 都不合法的许多情况下，包括在禁用抢占代码、local_bh_disable() 代码、禁用中断代码以及中断处理程序中。然而，即便 call_rcu() 在 NMI 处理程序中，以及来自空闲和离线 CPU 时也是非法的。回调函数（本例中的 remove_gp_cb()）将在 Linux 内核的软中断（software interrupt）环境中执行，既可能在真正的软中断处理程序中，也可能在 local_bh_disable() 的保护下。在 Linux 内核和用户态中，编写一个耗时过长的 RCU 回调函数都是不好的做法。长时间运行的操作应当交给单独的线程，或在 Linux 内核中交给工作队列（workqueue）。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why does line 19 use rcu_access_pointer()? After all,                 |
| call_rcu() on line 25 stores into the structure, which would          |
| interact badly with concurrent insertions. Doesn't this mean that     |
| rcu_dereference() is required?                                        |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Presumably the `->gp_lock` acquired on line 18 excludes any         |  
| changes, including any insertions that rcu_dereference() would        |
| protect against. Therefore, any insertions will be delayed until      |
| after `->gp_lock` is released on line 25, which in turn means that  |  
| rcu_access_pointer() suffices.                                        |
+-----------------------------------------------------------------------+

然而，remove_gp_cb() 所做的全部事情就是对数据元素调用 kfree()。这是一种常见惯用法，并得到 kfree_rcu() 的支持，它允许“发射后不管”（fire and forget）式的操作，如下所示：

```

       1 struct foo {
       2   int a;
       3   int b;
       4   struct rcu_head rh;
       5 };
       6
       7 bool remove_gp_faf(void)
       8 {
       9   struct foo *p;
      10
      11   spin_lock(&gp_lock);
      12   p = rcu_dereference(gp);
      13   if (!p) {
      14     spin_unlock(&gp_lock);
      15     return false;
      16   }
      17   rcu_assign_pointer(gp, NULL);
      18   kfree_rcu(p, rh);
      19   spin_unlock(&gp_lock);
      20   return true;
      21 }
      22
```

注意，remove_gp_faf() 只是简单地调用 kfree_rcu() 然后继续，无需再关注后续的宽限期和 kfree()。允许在与 call_rcu() 相同的环境中调用 kfree_rcu()。有趣的是，DYNIX/ptx 拥有 call_rcu() 和 kfree_rcu() 的等价物，却没有 synchronize_rcu()。这是因为 RCU 在 DYNIX/ptx 中用得不多，所以极少数需要类似 synchronize_rcu() 的地方干脆就内联实现了。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Earlier it was claimed that call_rcu() and kfree_rcu()                |
| allowed updaters to avoid being blocked by readers. But how can that  |
| be correct, given that the invocation of the callback and the freeing |
| of the memory (respectively) must still wait for a grace period to    |
| elapse?                                                               |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| We could define things this way, but keep in mind that this sort of   |
| definition would say that updates in garbage-collected languages      |
| cannot complete until the next time the garbage collector runs, which |
| does not seem at all reasonable. The key point is that in most cases, |
| an updater using either call_rcu() or kfree_rcu() can proceed         |
| to the next update as soon as it has invoked call_rcu() or            |
| kfree_rcu(), without having to wait for a subsequent grace            |
| period.                                                               |
+-----------------------------------------------------------------------+

但如果更新者必须等待在宽限期结束之后才执行的代码完成，而在此期间又有其他任务可以开展呢？轮询风格的 get_state_synchronize_rcu() 和 cond_synchronize_rcu() 函数可以用于此目的，如下所示：
```

       1 bool remove_gp_poll(void)
       2 {
       3   struct foo *p;
       4   unsigned long s;
       5
       6   spin_lock(&gp_lock);
       7   p = rcu_access_pointer(gp);
       8   if (!p) {
       9     spin_unlock(&gp_lock);
      10     return false;
      11   }
      12   rcu_assign_pointer(gp, NULL);
      13   spin_unlock(&gp_lock);
      14   s = get_state_synchronize_rcu();
      15   do_something_while_waiting();
      16   cond_synchronize_rcu(s);
      17   kfree(p);
      18   return true;
      19 }
      20
```

在第 14 行，get_state_synchronize_rcu() 从 RCU 取得一个“cookie”，然后第 15 行执行其他任务，最后，如果在此期间已经有宽限期过去，第 16 行会立即返回，否则会按需等待。`get_state_synchronize_rcu` 和 cond_synchronize_rcu() 的需求是最近才出现的，因此现在判断它们能否经受住时间的考验还为时过早。

RCU 因而提供了一系列工具，让更新者能够在延迟、灵活性和 CPU 开销之间做出所需的权衡。

#### 向前推进


理论上，延迟宽限期的完成和回调的调用是无害的。在实践中，不仅内存容量是有限的，而且回调有时确实会唤醒，而充分被推迟的唤醒可能很难与系统挂起区分开来。因此，RCU 必须提供若干机制来促进向前推进。

这些机制并非万无一失，也不可能万无一失。举一个简单的例子：RCU 读端临界区中的无限循环，按定义必然阻止后续宽限期永远无法完成。再举一个更复杂的例子，考虑一个用 `CONFIG_RCU_NOCB_CPU=y` 构建、并以 `rcu_nocbs=1-63` 引导的 64-CPU 系统，其中 CPU 1 到 63 在紧循环中自旋并调用 call_rcu()。即使这些紧循环还包含对 cond_resched() 的调用（从而允许宽限期完成），CPU 0 也根本无法以其他 63 个 CPU 注册回调的速度来调用回调，至少在该系统耗尽内存之前是这样。在这两个例子中，都适用“蜘蛛侠原则”：能力越大，责任越大。然而，只要不滥用到这种程度，RCU 就被要求保证宽限期的及时完成和回调的及时调用。

RCU 采取以下步骤来促使宽限期及时完成：

#. 如果某个宽限期未能在 100 毫秒内完成，RCU 会让那些 CPU 上后续对 cond_resched() 的调用提供一个 RCU 静止状态。RCU 还会让那些 CPU 的 need_resched() 调用返回 `true`，但只能在相应 CPU 的下一个调度时钟之后。
#. 在 `nohz_full` 内核引导参数中提到的 CPU 可以在内核中无限期运行而无需调度时钟中断，这会挫败上述 need_resched() 策略。因此 RCU 会在那些在 109 毫秒之后仍然坚持不放的 `nohz_full` CPU 上调用 resched_cpu()。
#. 在使用 `CONFIG_RCU_BOOST=y` 构建的内核中，如果某个在 RCU 读端临界区内被抢占的任务坚持不放超过 500 毫秒，RCU 将诉诸优先级提升。
#. 如果某个 CPU 在宽限期进行到 10 秒时仍然坚持不放，RCU 会调用 resched_cpu() 来调度它，而无论其 `nohz_full` 状态如何。

上述数值是运行在 `HZ=1000` 系统上的默认值。它们会随 `HZ` 值的变化而变化，也可以使用相关的 Kconfig 选项和内核引导参数来更改。RCU 目前对这些参数没有做太多合理性检查，因此更改时请务必小心。注意，这些向前推进措施只提供给 RCU，而不是 `SRCU <Sleepable RCU_>`__ 或 `Tasks RCU`_。

RCU 在 call_rcu() 中采取以下步骤，以促使在任何给定的非 `rcu_nocbs` CPU 拥有 10,000 个回调，或者比上次提供鼓励时多出 10,000 个回调时，及时调用回调：

#. 如果还没有宽限期在进行中，则启动一个宽限期。
#. 强制立即检查静止状态，而不是等待自宽限期开始起已过去三毫秒。
#. 立即用各自的宽限期完成编号给该 CPU 的回调打上标记，而不是等待 `RCU_SOFTIRQ` 处理程序腾出手来做这件事。
#. 提高回调执行的批处理上限，这能以退化实时响应为代价来加速回调调用。

同样，这些是运行在 `HZ=1000` 时的默认值，并且可以被覆盖。同样，这些向前推进措施只提供给 RCU，而不是 `SRCU <Sleepable RCU_>`__ 或 `Tasks RCU`_。即便对 RCU 而言，`rcu_nocbs` CPU 的回调调用向前推进也远未成熟，部分原因是受益于 `rcu_nocbs` CPU 的工作负载往往调用 call_rcu() 的频率相对较低。如果将来出现既需要 `rcu_nocbs` CPU 又需要高 call_rcu() 调用速率的工作负载，那么就需要额外的向前推进工作。

#### 可组合性


可组合性近年来受到了很多关注，或许部分是因为多核硬件与为单线程环境设计、用于单线程的面向对象技术发生了碰撞。理论上，RCU 读端临界区可以组合，事实上可以任意深度地嵌套。实践中，与所有可组合结构的现实实现一样，是有限制的。

对于那些 rcu_read_lock() 和 rcu_read_unlock() 不生成任何代码的 RCU 实现（例如当 `CONFIG_PREEMPTION=n` 时的 Linux 内核 RCU），可以任意深度地嵌套。毕竟没有开销。只是，如果所有这些 rcu_read_lock() 和 rcu_read_unlock() 的实例对编译器可见，编译最终会因耗尽内存、存储空间或用户耐心（视谁先发生）而失败。如果嵌套对编译器不可见，就像各自位于独立翻译单元中的互递归函数那样，就会导致栈溢出。如果嵌套采取循环的形式，或许伪装成尾递归，那么要么控制变量会溢出，要么（在 Linux 内核中）你会得到一个 RCU CPU 停顿（stall）警告。尽管如此，这类 RCU 实现仍是现存最具可组合性的构造之一。

显式跟踪嵌套深度的 RCU 实现受嵌套深度计数器的限制。例如，Linux 内核的可抢占 RCU 把嵌套限制为 `INT_MAX`。这对于几乎所有实际用途都足够了。话虽如此，一对前后相邻的两个 RCU 读端临界区，如果在它们之间有一个等待宽限期的操作，就不能被包含在另一个 RCU 读端临界区之中。这是因为不允许在 RCU 读端临界区内等待宽限期：那样做要么会导致死锁，要么会导致 RCU 隐式地拆分外层 RCU 读端临界区，这两者都不利于一个长寿且繁荣的内核。

值得一提的是，限制可组合性并非 RCU 独有。例如，许多事务内存实现禁止组合一对被一个不可撤销操作（例如网络接收操作）分隔的事务。再举一个例子，基于锁的临界区可以惊人自由地组合，但前提是必须避免死锁。

简而言之，尽管 RCU 读端临界区具有高度可组合性，但在某些情况下仍需要小心，就像任何其他可组合的同步机制一样。

#### 边界情况


某个给定的 RCU 工作负载可能有源源不断且密集的 RCU 读端临界区，甚至可能密集到在任一时刻都至少有一个 RCU 读端临界区在执行。RCU 不能允许这种情况阻塞宽限期：只要所有 RCU 读端临界区都是有限的，宽限期也必须是有限的。

话虽如此，可抢占 RCU 实现可能会导致 RCU 读端临界区被抢占很长时间，这就产生了一个长持续时间的 RCU 读端临界区。这种情况只可能出现在负载沉重的系统中，但使用实时优先级的系统当然更脆弱。因此，提供了 RCU 优先级提升来帮助应对这种情况。话虽如此，对 RCU 优先级提升的确切需求很可能随着经验的积累而演变。

其他工作负载可能有非常高的更新速率。尽管有人会辩称这样的工作负载应该使用 RCU 之外的其他东西，但事实是 RCU 必须优雅地处理这类工作负载。这一需求是推动宽限期批处理的另一个因素，但它也是 call_rcu() 代码路径中检查大量排队 RCU 回调的背后驱动力。最后，高更新速率不应延迟 RCU 读端临界区，尽管在使用 synchronize_rcu_expedited() 时（由于该函数使用了 smp_call_function_single()）可能会出现一些小的读端延迟。

尽管这三种边界情况在 1990 年代初就已被理解，但在 2000 年代初，一个由紧循环中的 `close(open(path))` 组成的简单用户态测试突然让人对高更新速率这种边界情况有了深刻得多的认识。这个测试也促使加入了一些 RCU 代码来应对高更新速率，例如，如果某个给定 CPU 发现自己排队的 RCU 回调超过 10,000 个，它会促使 RCU 采取规避行动，更积极地启动宽限期，并更积极地强制完成宽限期处理。这种规避行动使宽限期更快地完成，但代价是限制了 RCU 的批处理优化，从而增加了该宽限期带来的 CPU 开销。

### 软件工程需求


介于墨菲定律与“犯错是人之常情”之间，有必要防范意外事故和误用：

#. 人们太容易忘记在每一个需要的地方使用 rcu_read_lock()，因此用 `CONFIG_PROVE_RCU=y` 构建的内核会在 rcu_dereference() 被用于 RCU 读端临界区之外时发出 splat。更新端代码可以使用 rcu_dereference_protected()，它接受一个 `lockdep expression <https://lwn.net/Articles/371986/>`__ 来表明是什么提供了保护。如果所指示的保护没有提供，就会发出一个 lockdep splat。
   读写共享的代码可以使用 rcu_dereference_check()，它也接受一个 lockdep 表达式，并且如果 rcu_read_lock() 和所指示的保护都没有就位，就会发出 lockdep splat。此外，rcu_dereference_raw() 用于那些（希望很少见的）难以简便描述所需保护的场合。最后，提供 rcu_read_lock_held() 以允许某个函数验证自己是在 RCU 读端临界区内被调用的。我是在 Thomas Gleixner 审查了若干 RCU 用法之后不久才意识到这一组需求的。
#. 某个给定的函数可能希望在入口处、在使用任何其他 RCU API 之前，检查 RCU 相关的先决条件。rcu_lockdep_assert() 做这件事，它在启用了 lockdep 的内核中断言该表达式，否则什么都不做。
#. 人们同样容易忘记使用 rcu_assign_pointer() 和 rcu_dereference()，或许（错误地）用一个简单的赋值来替代。为了捕获这类错误，一个给定的 RCU 保护指针可以用 `__rcu` 标记，之后 sparse 就会抱怨对该指针的简单赋值访问。Arnd Bergmann 让我意识到了这一需求，并且还提供了所需的 `patch series <https://lwn.net/Articles/376011/>`__。
#. 用 `CONFIG_DEBUG_OBJECTS_RCU_HEAD=y` 构建的内核会在把一个数据元素连续两次传给 call_rcu()、而中间没有宽限期时发出 splat。（这个错误类似于双重释放。）那些动态分配的相应 `rcu_head` 结构会被自动跟踪，但分配在栈上的 `rcu_head` 结构必须用 init_rcu_head_on_stack() 初始化，并用 destroy_rcu_head_on_stack() 清理。类似地，静态分配的非栈 `rcu_head` 结构必须用 init_rcu_head() 初始化，并用 destroy_rcu_head() 清理。Mathieu Desnoyers 让我意识到了这一需求，并且还提供了所需的 `patch <https://lore.kernel.org/r/20100319013024.GA28456@Krystal>`__。
#. RCU 读端临界区中的无限循环最终会触发一个 RCU CPU 停顿警告 splat，而“最终”的时长由 `RCU_CPU_STALL_TIMEOUT` `Kconfig` 选项控制，或者，也可由 `rcupdate.rcu_cpu_stall_timeout` 引导/sysfs 参数控制。然而，除非有某个宽限期在等待那个特定的 RCU 读端临界区，否则 RCU 没有义务产生这个 splat。

   某些极端的工作负载可能有意延迟 RCU 宽限期，运行这些工作负载的系统可以用 `rcupdate.rcu_cpu_stall_suppress` 引导来抑制 splat。这个内核参数也可以通过 `sysfs` 设置。此外，RCU CPU 停顿警告在 sysrq dump 期间和 panic 期间会适得其反。因此 RCU 提供了 rcu_sysrq_start() 和 rcu_sysrq_end() API 成员，分别在长时间 sysrq dump 之前和之后调用。RCU 还提供了 rcu_panic() 通知器，它会在 panic 开始时自动被调用来抑制进一步的 RCU CPU 停顿警告。

   这一需求在 1990 年代初就显现出来了，差不多是第一次需要调试 CPU 停顿时。话虽如此，与 Linux 相比，DYNIX/ptx 中的初始实现是相当通用的。

#. 尽管能检测到指针从 RCU 读端临界区泄漏出来会非常好，但目前还没有好的方法可以做到这一点。一个难点是需要区分“指针泄漏”与“指针已从 RCU 移交给其他某种同步机制（例如引用计数）”这两种情况。
#. 在用 `CONFIG_RCU_TRACE=y` 构建的内核中，RCU 相关信息通过事件跟踪提供。
#. 直接使用 rcu_assign_pointer() 和 rcu_dereference() 来创建典型的链表数据结构可能惊人地容易出错。因此，提供了 RCU 保护的 `linked lists <https://lwn.net/Articles/609973/#RCU%20List%20APIs>`__，以及（更近期的）RCU 保护的 `hash tables <https://lwn.net/Articles/612100/>`__。许多其他专门用途的 RCU 保护数据结构在 Linux 内核和用户态 RCU 库中都有提供。
#. 有些链表结构是在编译时创建的，但依然需要 `__rcu` 检查。RCU_POINTER_INITIALIZER() 宏用于此目的。
#. 在创建要通过单个外部指针发布的链表结构时，没有必要使用 rcu_assign_pointer()。为此提供了 RCU_INIT_POINTER() 宏。

这不是一份硬性而固定的清单：RCU 的诊断能力将继续由真实世界 RCU 使用中发现的缺陷的数量和类型来指引。

### Linux 内核带来的复杂性


Linux 内核为包括 RCU 在内的各类软件提供了一个有趣的环境。一些相关的关注点如下：

#. `Configuration`_
#. `Firmware Interface`_
#. `Early Boot`_
#. `Interrupts and NMIs`_
#. `Loadable Modules`_
#. `Hotplug CPU`_
#. `Scheduler and RCU`_
#. `Tracing and RCU`_
#. `Accesses to User Memory and RCU`_
#. `Energy Efficiency`_
#. `Scheduling-Clock Interrupts and RCU`_
#. `Memory Efficiency`_
#. `Performance, Scalability, Response Time, and Reliability`_

这份清单可能并不完整，但它确实让人感受到了最显著的 Linux 内核复杂性。以下各节分别介绍上述主题之一。

#### 配置


RCU 的目标是自动配置，这样几乎没有人需要操心 RCU 的 `Kconfig` 选项。并且对于几乎所用的用户，RCU 确实“开箱即用”地工作良好。

然而，也有一些专门用途是由内核引导参数和 `Kconfig` 选项来处理的。不幸的是，`Kconfig` 系统会显式地就新的 `Kconfig` 选项询问用户，这就要求几乎所有选项都隐藏在一个 `CONFIG_RCU_EXPERT` `Kconfig` 选项之后。

这一切应当相当显而易见，但事实是，Linus Torvalds 最近不得不 `remind <https://lore.kernel.org/r/CA+55aFy4wcCwaL4okTs8wXhGZ5h-ibecy_Meg9C4MNQrUnwMcg@mail.gmail.com>`__ 我这一需求。

#### 固件接口


在许多情况下，内核从固件获取关于系统的信息，而有时信息在翻译过程中丢失了。或者翻译是准确的，但原始消息本身就是假的。
例如，某些系统的固件会高估 CPU 的数量，有时高出很多倍。如果 RCU 像过去那样天真地相信固件，它就会创建过多的每-CPU kthread。尽管由此得到的系统仍然能正确运行，但那些多余的 kthread 会不必要地消耗内存，并且在它们出现在 `ps` 列表时会令人困惑。

RCU 因此必须等待某个给定 CPU 真正上线之后，才能让自己相信该 CPU 确实存在。由此产生的“幽灵 CPU”（它们永远也不会上线）会造成若干 `interesting complications <https://paulmck.livejournal.com/37494.html>`__。

#### 早期启动


Linux 内核的启动过程是一个有趣的过程，RCU 用得很早，甚至在 rcu_init() 被调用之前。事实上，RCU 的许多原语在初始任务的 `task_struct` 可用、且引导 CPU 的每-CPU 变量设置好之后就可以使用。读端原语（rcu_read_lock()、rcu_read_unlock()、rcu_dereference() 和 rcu_access_pointer()）在很早的时候就会正常运行，rcu_assign_pointer() 也是如此。

尽管 call_rcu() 可以在启动期间的任何时刻被调用，但回调保证要到 RCU 的所有 kthread 都生成之后才会被调用，这发生在 early_initcall() 时刻。回调调用的这种延迟是由于 RCU 在完全初始化之前不会调用回调，而这一完全初始化要等到调度器把自己初始化到 RCU 能够生成并运行其 kthread 的程度之后才能发生。理论上，更早地调用回调是可能的，然而，这并非万灵药，因为那些回调能调用的操作会受到严格的限制。

也许令人惊讶的是，synchronize_rcu() 和 synchronize_rcu_expedited() 在非常早的启动阶段会正常运行，原因是那时只有一个 CPU 且抢占被禁用。这意味着对 synchronize_rcu()（或其同类）本身的调用就是一个静止状态，从而也就是一个宽限期，因此早期启动的实现可以是一个空操作。

然而，一旦调度器生成了它的第一个 kthread，这种早期启动技巧对于 `CONFIG_PREEMPTION=y` 内核中的 synchronize_rcu()（以及 synchronize_rcu_expedited()）就失效了。原因是 RCU 读端临界区可能会被抢占，这意味着后续的 synchronize_rcu() 确实必须等待某些东西，而不是简单地立即返回。不幸的是，synchronize_rcu() 在它的所有 kthread 都生成之前无法做到这一点，而这要到 early_initcalls() 期间的某个时刻才会发生。但这不能成为借口：RCU 仍然被要求在这一时间段内正确处理同步宽限期。一旦它的所有 kthread 都启动并运行，RCU 就开始正常运行。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| How can RCU possibly handle grace periods before all of its kthreads  |
| have been spawned???                                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Very carefully!                                                       |
| During the "dead zone" between the time that the scheduler spawns the |
| first task and the time that all of RCU's kthreads have been spawned, |
| all synchronous grace periods are handled by the expedited            |
| grace-period mechanism. At runtime, this expedited mechanism relies   |
| on workqueues, but during the dead zone the requesting task itself    |
| drives the desired expedited grace period. Because dead-zone          |
| execution takes place within task context, everything works. Once the |
| dead zone ends, expedited grace periods go back to using workqueues,  |
| as is required to avoid problems that would otherwise occur when a    |
| user task received a POSIX signal while driving an expedited grace    |
| period.                                                               |
|                                                                       |
| And yes, this does mean that it is unhelpful to send POSIX signals to |
| random tasks between the time that the scheduler spawns its first     |
| kthread and the time that RCU's kthreads have all been spawned. If    |
| there ever turns out to be a good reason for sending POSIX signals    |
| during that time, appropriate adjustments will be made. (If it turns  |
| out that POSIX signals are sent during this time for no good reason,  |
| other adjustments will be made, appropriate or otherwise.)            |
+-----------------------------------------------------------------------+

我是通过一系列系统挂起才了解到这些启动期需求的。

#### 中断与 NMI


Linux 内核有中断，RCU 读端临界区在中断处理程序内以及禁用中断的代码区域内都是合法的，call_rcu() 的调用也是如此。

某些 Linux 内核体系结构可以从非空闲的进程上下文进入一个中断处理程序，然后就再也不离开它，而是偷偷地转回进程上下文。这个技巧有时被用来从内核内部调用系统调用。这些“半中断”意味着 RCU 必须非常小心地计算中断嵌套层级。我是在重写 RCU 的 dyntick-idle 代码时吃了苦头才了解到这一需求的。

Linux 内核有不可屏蔽中断（NMI），RCU 读端临界区在 NMI 处理程序内是合法的。值得庆幸的是，RCU 更新端原语（包括 call_rcu()）在 NMI 处理程序内是被禁止的。

尽管名为不可屏蔽中断，某些 Linux 内核体系结构却可以有嵌套的 NMI，RCU 必须正确处理。Andy Lutomirski 用这一需求 `surprised me <https://lore.kernel.org/r/CALCETrXLq1y7e_dKFPgou-FKHB6Pu-r8+t-6Ds+8=va7anBWDA@mail.gmail.com>`__，他还好心地用 `an algorithm <https://lore.kernel.org/r/CALCETrXSY9JpW3uE6H8WYk81sg56qasA2aqmjMPsq5dOtzso=g@mail.gmail.com>`__ 让我惊喜，该算法满足了这一需求。

此外，NMI 处理程序可能被在 RCU 看来是普通中断的东西打断。发生这种情况的一种途径是，直接从 NMI 处理程序中调用 ct_irq_enter() 和 ct_irq_exit() 的代码。这一惊人现实促成了当前的代码结构，即让 ct_irq_enter() 调用 ct_nmi_enter()、ct_irq_exit() 调用 ct_nmi_exit()。没错，我也是吃了苦头才了解到这一需求的。

#### 可加载模块


Linux 内核有可加载模块，这些模块也可以被卸载。在给定模块被卸载之后，任何调用其函数的尝试都会导致段错误。模块的卸载函数因此必须取消对任何可加载模块函数的延迟调用，例如，任何未决的 mod_timer() 都必须通过 timer_shutdown_sync() 或类似方法来处理。

不幸的是，没有办法取消一个 RCU 回调；一旦你调用了 call_rcu()，该回调函数最终就会被执行，除非系统在此之前宕机。因为用让系统崩溃来回应一次模块卸载请求通常被视为不负社会责任的行为，我们需要其他方法来处理在途的 RCU 回调。

RCU 因而提供 rcu_barrier()，它等待所有在途的 RCU 回调都被调用。如果一个模块使用了 call_rcu()，它的退出函数因此应当阻止将来任何对 call_rcu() 的调用，然后调用 rcu_barrier()。理论上，底层的模块卸载代码可以无条件地调用 rcu_barrier()，但在实践中这会带来不可接受的延迟。

Nikita Danilov 针对一个类似的文件系统卸载场景指出了这一需求，而 Dipankar Sarma 把 rcu_barrier() 引入了 RCU。rcu_barrier() 用于模块卸载的需求是后来才显现出来的。


   rcu_barrier() 函数并不——重复一遍，**并不**——有义务等待一个宽限期。它只被要求等待那些已经提交的 RCU 回调。因此，如果系统中任何地方都没有提交 RCU 回调，rcu_barrier() 完全有权立即返回。即使确有回调被提交，rcu_barrier() 也不一定需要等待一个宽限期。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Wait a minute! Each RCU callbacks must wait for a grace period to     |
| complete, and rcu_barrier() must wait for each pre-existing           |
| callback to be invoked. Doesn't rcu_barrier() therefore need to       |
| wait for a full grace period if there is even one callback posted     |
| anywhere in the system?                                               |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Absolutely not!!!                                                     |
| Yes, each RCU callbacks must wait for a grace period to complete, but |
| it might well be partly (or even completely) finished waiting by the  |
| time rcu_barrier() is invoked. In that case, rcu_barrier()            |
| need only wait for the remaining portion of the grace period to       |
| elapse. So even if there are quite a few callbacks posted,            |
| rcu_barrier() might well return quite quickly.                        |
|                                                                       |
| So if you need to wait for a grace period as well as for all          |
| pre-existing callbacks, you will need to invoke both                  |
| synchronize_rcu() and rcu_barrier(). If latency is a concern,         |
| you can always use workqueues to invoke them concurrently.            |
+-----------------------------------------------------------------------+

#### 热插拔 CPU


Linux 内核支持 CPU 热插拔，这意味着 CPU 可以来来去去。当然，从离线 CPU 使用任何 RCU API 成员都是非法的，例外是 `SRCU <Sleepable RCU_>`__ 读端临界区。这一需求从 DYNIX/ptx 的第一天起就存在，但另一方面，Linux 内核的 CPU 热插拔实现“很有意思”。

Linux 内核的 CPU 热插拔实现带有通知器（notifier），用于让各个内核子系统（包括 RCU）对给定的 CPU 热插拔操作做出恰当响应。大多数 RCU 操作都可以从 CPU 热插拔通知器中调用，甚至包括像 synchronize_rcu() 和 synchronize_rcu_expedited() 这样的同步宽限期操作。然而，这些同步操作确实会阻塞，因此不能从通过 stop_machine() 执行的通知器中调用，具体来说就是那些处于 `CPUHP_AP_OFFLINE` 和 `CPUHP_AP_ONLINE` 状态之间的通知器。

此外，像 rcu_barrier() 这样的“等待所有回调”操作不能从任何 CPU 热插拔通知器中调用。这一限制是因为在 CPU 热插拔操作的某些阶段，离线的那个 CPU 的回调在 CPU 热插拔操作结束之前不会被调用，这同样可能导致死锁。而且，rcu_barrier() 在其执行期间会阻塞 CPU 热插拔操作，当从 CPU 热插拔通知器中调用时，这会导致另一种死锁。

最后，RCU 必须避免由于热插拔、定时器和宽限期处理之间的交互而导致的死锁。它通过对自己的那套账本进行维护来做到这一点，这些账本复制了集中维护的 `cpu_online_mask`，并且在 CPU 离线时显式地报告静止状态。这种对静止状态的显式报告，避免了强制静止状态循环（FQS）去为离线 CPU 报告静止状态的任何需要。不过，作为一种调试手段，如果离线 CPU 把 RCU 宽限期阻塞太久，FQS 循环确实会发出 splat。

一个离线 CPU 的静止状态会通过以下两种方式之一被报告：

1. 当该 CPU 通过 RCU 的热插拔通知器离线时（rcutree_report_cpu_dead()）。
2. 当宽限期初始化（rcu_gp_init()）检测到与 CPU 离线、或者与一个在叶子 `rcu_node` 结构（其所有 CPU 都已离线）上解除阻塞的任务之间的竞态时。

CPU 上线路径（rcutree_report_cpu_starting()）应当永远不需要为离线 CPU 报告静止状态。不过，作为一种调试手段，如果尚未为该 CPU 报告静止状态，它确实会发出一个警告。

在检查/修改 RCU 的热插拔账本期间，会持有相应 CPU 的叶子节点锁。这避免了 RCU 的热插拔通知器钩子、宽限期初始化代码和 FQS 循环之间的竞态条件，它们都引用或修改这套账本。

注意，宽限期初始化（rcu_gp_init()）必须仔细地为 CPU 热插拔扫描与宽限期状态变更排序。例如，如果 rcu_seq_start() 先发生，下面的竞态就可能在 rcu_gp_init() 中发生
```

   CPU0 (rcu_gp_init)                   CPU1                          CPU2
   ---------------------                ----                          ----
   // Hotplug scan first (WRONG ORDER)
   rcu_for_each_leaf_node(rnp) {
       rnp->qsmaskinit = rnp->qsmaskinitnext;
   }
                                        rcutree_report_cpu_starting()
                                            rnp->qsmaskinitnext |= mask;
                                        rcu_read_lock()
                                        r0 = *X;
                                                                      r1 = *X;
                                                                      X = NULL;
                                                                      cookie = get_state_synchronize_rcu();
                                                                      // cookie = 8 (future GP)
   rcu_seq_start(&rcu_state.gp_seq);
   // gp_seq = 5

   // CPU1 now invisible to this GP!
   rcu_for_each_node_breadth_first() {
       rnp->qsmask = rnp->qsmaskinit;
       // CPU1 not included!
   }

   // GP completes without CPU1
   rcu_seq_end(&rcu_state.gp_seq);
   // gp_seq = 8
                                                                      poll_state_synchronize_rcu(cookie);
                                                                      // Returns true!
                                                                      kfree(r1);
                                        r2 = *r0; // USE-AFTER-FREE!

```

通过先递增 `gp_seq`，就能保证 CPU1 的 RCU 读端临界区不会被 CPU2 漏掉。

##### 离线 CPU 的并发静止状态报告


RCU 必须确保离线的 CPU 报告静止状态，以避免阻塞宽限期。这需要仔细的同步来处理竞态条件

##### 导致离线 CPU 挂起 GP 的竞态条件


CPU 离线与新 GP 初始化（gp_init()）之间可能发生竞态，因为 rcutree_report_cpu_dead() 中的 rcu_report_qs_rnp() 必须临时
```

   CPU1 (going offline)                 CPU0 (GP kthread)
   --------------------                 -----------------
   rcutree_report_cpu_dead()
     rcu_report_qs_rnp()
       // Must release rnp->lock to wake GP kthread
       raw_spin_unlock_irqrestore_rcu_node()
                                        // Wakes up and starts new GP
                                        rcu_gp_init()
                                          // First loop:
                                          copies qsmaskinitnext->qsmaskinit
                                          // CPU1 still in qsmaskinitnext!

                                          // Second loop:
                                          rnp->qsmask = rnp->qsmaskinit
                                          mask = rnp->qsmask & ~rnp->qsmaskinitnext
                                          // mask is 0! CPU1 still in both masks
       // Reacquire lock (but too late)
     rnp->qsmaskinitnext &= ~mask       // Finally clears bit

```

如果没有 `ofl_lock`，新的宽限期就会包含离线的 CPU，并永远等待它的静止状态，从而导致 GP 挂起。

##### 使用 ofl_lock 的解决方案


`ofl_lock`（离线锁）阻止 rcu_gp_init() 在以下情况期间运行
```

   CPU0 (rcu_gp_init)                   CPU1 (rcutree_report_cpu_dead)
   ------------------                   ------------------------------
   rcu_for_each_leaf_node(rnp) {
       arch_spin_lock(&ofl_lock) -----> arch_spin_lock(&ofl_lock) [BLOCKED]

       // Safe: CPU1 can't interfere
       rnp->qsmaskinit = rnp->qsmaskinitnext

       arch_spin_unlock(&ofl_lock) ---> // Now CPU1 can proceed
   }                                    // But snapshot already taken

```

##### 导致 rcu_gp_init() 中 GP 挂起的另一种竞态：为现已离线的 CPU 报告 QS


在第一个循环对在线 CPU 取了原子快照之后（如上所示），rcu_gp_init() 中的第二个循环会检测在释放 `ofl_lock` 与获取每节点 `rnp->lock` 之间离线的 CPU。
这一检测至关重要，因为：

1. 该 CPU 可能在快照之后、第二个循环之前离线了
2. 离线的 CPU 如果已经“死亡”，就无法报告它自己的 QS
3. 没有这一检测，宽限期就会永远等待那些现已离线的 CPU
```

   rcu_for_each_node_breadth_first(rnp) {
       raw_spin_lock_irqsave_rcu_node(rnp, flags);
       rnp->qsmask = rnp->qsmaskinit;  // Apply the snapshot

       // Detect CPUs offline after snapshot
       mask = rnp->qsmask & ~rnp->qsmaskinitnext;

       if (mask && rcu_is_leaf_node(rnp))
           rcu_report_qs_rnp(mask, ...)  // Report QS for offline CPUs
   }

```

这种方法保证了原子性：对离线 CPU 的静止状态报告要么发生在 rcu_gp_init()（第二个循环）中，要么发生在 rcutree_report_cpu_dead() 中，永远不会两者都做，也永远不会两者都不做。整个序列期间持有的 `rnp->lock` 防止了竞态——rcutree_report_cpu_dead() 在清除 `qsmaskinitnext` 时也会获取这把锁，从而确保互斥。

#### 调度器与 RCU


RCU 使用了 kthread，并且必须避免这些 kthread 过度累积 CPU 时间。这一需求并不令人意外，但 RCU 在构建时带有 `CONFIG_NO_HZ_FULL=y`、并运行上下文切换繁重的工作负载时违反了它，这 `did come as a surprise [PDF] <http://www.rdrop.com/users/paulmck/scalability/paper/BareMetal.2015.01.15b.pdf>`__。RCU 在满足这一需求方面已经取得了良好进展，即便对上下文切换繁重的 `CONFIG_NO_HZ_FULL=y` 工作负载也是如此，但仍有进一步改进的空间。

不再有任何禁止在某个 rcu_read_unlock() 期间持有调度器的运行队列锁或优先级继承自旋锁，即使在相应 RCU 读端临界区内部的某处启用了中断和抢占也是如此。因此，现在完全合法地可以在启用抢占的情况下执行 rcu_read_lock()、获取其中一把调度器锁，并在与之匹配的 rcu_read_unlock() 期间持有该锁。

类似地，RCU 风格的整合已经消除了对负向嵌套的需求。禁用中断的代码区域隐式地充当 RCU 读端临界区这一事实，避免了早期那些会因中断处理程序使用 RCU 而导致破坏性递归的问题。

#### 跟踪与 RCU


可以在 RCU 代码上使用跟踪，但跟踪本身使用了 RCU。因此，提供了 rcu_dereference_raw_check() 供跟踪使用，它避免了原本可能发生的破坏性递归。这个 API 在某些体系结构的虚拟化中也被使用，那里 RCU 读者运行在无法使用跟踪的环境中。跟踪的开发者既定位到了这一需求，也提供了所需的修复，所以这个意外需求的代价相对较小。

#### 对用户内存的访问与 RCU


内核需要访问用户空间内存，例如，访问由系统调用参数所引用的数据。get_user() 宏做这件事。

然而，用户空间内存很可能被换出页，这意味着 get_user() 很可能会发生页错误，从而在等待由此产生的 I/O 完成时阻塞。如果编译器把 get_user() 的调用重排进一个 RCU 读端临界区，那将是非常糟糕的事情。

例如，假设源代码看起来像这样：

```

       1 rcu_read_lock();
       2 p = rcu_dereference(gp);
       3 v = p->value;
       4 rcu_read_unlock();
       5 get_user(user_v, user_p);
       6 do_something_with(v, user_v);

```

绝不能允许编译器把这段源代码变换成下面这样：

```

       1 rcu_read_lock();
       2 p = rcu_dereference(gp);
       3 get_user(user_v, user_p); // BUG: POSSIBLE PAGE FAULT!!!
       4 v = p->value;
       5 rcu_read_unlock();
       6 do_something_with(v, user_v);

```

如果编译器真的在 `CONFIG_PREEMPTION=n` 内核构建中做了这种变换，并且如果 get_user() 确实发生了页错误，结果就会是在一个 RCU 读端临界区中间出现一个静止状态。这个错位的静止状态可能导致第 4 行成为一个释放后使用（use-after-free）访问，这可能对你的内核的精算统计很不利。也可以用把 get_user() 调用放在 rcu_read_lock() 之前的情形构造出类似的例子。

不幸的是，get_user() 没有任何特定的顺序特性，并且在某些体系结构上，底层的 `asm` 甚至没有被标记为 `volatile`。而即便它被标记为 `volatile`，上面那个对 `p->value` 的访问也不是 volatile 的，所以编译器没有任何理由把那两次访问保持有序。

因此，rcu_read_lock() 和 rcu_read_unlock() 的 Linux 内核定义必须充当编译器屏障，至少对一组嵌套 RCU 读端临界区中最外层的 rcu_read_lock() 和 rcu_read_unlock() 实例而言如此。

#### 能效


打断空闲 CPU 被认为是不合社会公德的，尤其是对于那些使用电池供电的嵌入式系统的人而言。RCU 因此通过检测哪些 CPU 空闲（包括跟踪那些从空闲中被打断的 CPU）来节省能耗。这是能效需求的一大部分，所以我是通过一通愤怒的电话才了解到它的。

因为 RCU 避免打断空闲 CPU，所以在空闲 CPU 上执行 RCU 读端临界区是非法的。（如果你尝试这样做，用 `CONFIG_PROVE_RCU=y` 构建的内核会发出 splat。）

打断运行在用户态的 `nohz_full` CPU 同样被认为是不合社会公德的。RCU 因此必须跟踪 `nohz_full` 的用户态执行。RCU 因此必须能够在两个时间点上采样状态，并且能够判断其他某个 CPU 是否曾花费任何时间处于空闲和/或执行于用户态。

这些能效需求被证明相当难以理解和满足，例如，RCU 的能效代码已经被彻底重写过不止五次，其中最后一次终于能够在真实硬件上演示出 `real energy savings running on real hardware [PDF] <http://www.rdrop.com/users/paulmck/realtime/paper/AMPenergy.2013.04.19a.pdf>`__。如前所述，我是通过愤怒的电话了解到其中许多需求的：在 Linux 内核邮件列表上对我发火，显然不足以完全宣泄他们对 RCU 能效缺陷的怒火！

#### 调度时钟中断与 RCU


内核在内核内非空闲执行、用户态执行和空闲循环之间转换。取决于内核配置，RCU 对这些状态的处理方式不同：

+-----------------+------------------+------------------+-----------------+
| `HZ` Kconfig    | 内核中           | 用户态           | 空闲            |
+=================+==================+==================+=================+
| `HZ_PERIODIC` | 可以依赖         | 可以依赖         | 可以依赖        |
|                 | 调度时钟         | 调度时钟         | RCU 的          |
|                 | 中断。           | 中断及其         | dyntick-idle    |
|                 |                  | 对来自用户态的   | 检测。          |
|                 |                  | 中断的检测。     |                 |
+-----------------+------------------+------------------+-----------------+
| `NO_HZ_IDLE`  | 可以依赖         | 可以依赖         | 可以依赖        |
|                 | 调度时钟         | 调度时钟         | RCU 的          |
|                 | 中断。           | 中断及其         | dyntick-idle    |
|                 |                  | 对来自用户态的   | 检测。          |
|                 |                  | 中断的检测。     |                 |
+-----------------+------------------+------------------+-----------------+
| `NO_HZ_FULL`  | 只能有时         | 可以依赖         | 可以依赖        |
|                 | 依赖调度时钟     | RCU 的           | RCU 的          |
|                 | 中断。在其他     | dyntick-idle     | dyntick-idle    |
|                 | 情况下，有必要   | 检测。           | 检测。          |
|                 | 限制内核执行     |                  |                 |
|                 | 时间和/或使用    |                  |                 |
|                 | IPI。            |                  |                 |
+-----------------+------------------+------------------+-----------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why can't `NO_HZ_FULL` in-kernel execution rely on the              |  
| scheduling-clock interrupt, just like `HZ_PERIODIC` and             |  
| `NO_HZ_IDLE` do?                                                    |  
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Because, as a performance optimization, `NO_HZ_FULL` does not       |  
| necessarily re-enable the scheduling-clock interrupt on entry to each |
| and every system call.                                                |
+-----------------------------------------------------------------------+

然而，RCU 必须被可靠地告知任意给定 CPU 当前是否处于空闲循环，并且对于 `NO_HZ_FULL`，还要被告知该 CPU 是否正在执行于用户态，如 `earlier <Energy Efficiency_>`__ 所讨论。它还需要在 RCU 需要调度时钟中断时被启用：

#. 如果某个 CPU 要么空闲、要么执行于用户态，而 RCU 认为它是非空闲的，那么调度时钟滴答最好正在运行。否则，你会得到 RCU CPU 停顿警告。或者最好的情况下，是非常长（11 秒）的宽限期，伴随着一个无意义的 IPI 不时唤醒该 CPU。
#. 如果某个 CPU 处于内核中会执行 RCU 读端临界区的部分，而 RCU 认为该 CPU 空闲，你会得到随机的内存损坏。**不要这样做！！！** 这正是用 lockdep 做测试的一个理由，它会抱怨这类事情。
#. 如果某个 CPU 处于内核中绝对、肯定、绝不执行任何 RCU 读端临界区的部分，而 RCU 认为该 CPU 空闲，则没问题。某些体系结构把这种东西用于轻量级异常处理程序，从而可以避免在异常进入和退出时分别承受 ct_irq_enter() 和 ct_irq_exit() 的开销。有些走得更远，连 irq_enter() 和 irq_exit() 的整体都避免了。
   只要非常确定你用 `CONFIG_PROVE_RCU=y` 运行了部分测试，以防你的某条代码路径其实是在开玩笑地说自己不执行 RCU 读端临界区。
#. 如果某个 CPU 在内核中执行、调度时钟中断被禁用、且 RCU 认为该 CPU 非空闲，并且如果该 CPU 每隔几个 jiffies（从 RCU 角度看）就进入空闲，则没问题。空闲期之间偶尔出现长达一秒左右的间隔通常是可以的。
   如果间隔变得太长，你会得到 RCU CPU 停顿警告。
#. 如果某个 CPU 要么空闲、要么执行于用户态，而 RCU 认为它空闲，自然没问题。
#. 如果某个 CPU 在内核中执行，内核代码路径以合理的频率经过静止状态（最好大约每几个 jiffies 一次，但偶尔延伸到一秒左右通常也可以），并且调度时钟中断被启用，自然没问题。
   如果连续两个静止状态之间的间隔变得太长，你会得到 RCU CPU 停顿警告。

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But what if my driver has a hardware interrupt handler that can run   |
| for many seconds? I cannot invoke schedule() from an hardware         |
| interrupt handler, after all!                                         |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| One approach is to do `ct_irq_exit();ct_irq_enter();` every so      |  
| often. But given that long-running interrupt handlers can cause other |
| problems, not least for response time, shouldn't you work to keep     |
| your interrupt handler's runtime within reasonable bounds?            |
+-----------------------------------------------------------------------+

但只要 RCU 被正确告知内核态执行、用户态执行和空闲之间的内核状态转换，并且只要调度时钟中断在 RCU 需要它时被启用，你就可以放心，你遇到的 bug 会出现在 RCU 的其他部分、或者内核的其他部分！

#### 内存效率


尽管小型内存的非实时系统可以简单地使用 Tiny RCU，但代码大小只是内存效率的一个方面。另一个方面是 call_rcu() 和 kfree_rcu() 所使用的 `rcu_head` 结构的大小。尽管这个结构只包含一对指针，但它确实出现在许多 RCU 保护的数据结构中，包括一些对大小敏感的结构。`page` 结构就是一个恰当的例子，该结构内部大量出现 `union` 关键字便证明了这一点。

这种对内存效率的需求，是 RCU 使用手工打造的单向链表来跟踪那些等待宽限期过去的 `rcu_head` 结构的一个原因。这也是 `rcu_head` 结构不包含调试信息（例如跟踪提交了它们的 call_rcu() 或 kfree_rcu() 的文件和行的字段）的原因。尽管这类信息或许将来某个时候会出现在仅用于调试的内核构建中，但截至目前，`->func` 字段往往能提供所需的调试信息。

然而，在某些情况下，对内存效率的需求导致了更极端的措施。回到 `page` 结构，`rcu_head` 字段与许多其他结构共享存储，这些结构在该页生命周期中的不同时刻被使用。为了正确解决某些 `race conditions <https://lore.kernel.org/r/1439976106-137226-1-git-send-email-kirill.shutemov@linux.intel.com>`__，Linux 内核的内存管理子系统需要某个特定的位在宽限期处理的所有阶段都保持为零，而那个位恰好映射到 `rcu_head` 结构的 `->next` 字段的最低位。只要使用 call_rcu()（而不是 kfree_rcu() 或将来某天可能为能效目的而创建的 call_rcu() 的某个“惰性”变体）来提交回调，RCU 就提供这一保证。

话虽如此，还是有限度的。RCU 要求 `rcu_head` 结构按两字节边界对齐，把未对齐的 `rcu_head` 结构传给 call_rcu() 系列函数中的某一个会导致 splat。因此在打包包含 `rcu_head` 类型字段的结构时必须谨慎。为什么不是四字节甚至八字节的对齐要求？因为 m68k 体系结构只提供两字节对齐，因此它充当了内存对齐的最小公分母。

保留指向 `rcu_head` 结构的指针的最低位的理由，是为“惰性”回调留下大门，这类回调的调用可以被安全地推迟。推迟调用可能有潜在的能效收益，但前提是对于某个重要工作负载，非惰性回调的速率显著下降。在此之前，保留最低位让这个选项保持开放，以备将来某天变得有用。

#### 性能、可扩展性、响应时间与可靠性


扩展 `earlier discussion <Performance and Scalability_>`__，RCU 被 Linux 内核的网络、安全、虚拟化和调度等性能关键代码路径中的热点代码大量使用。RCU 因此必须使用高效的实现，尤其是在其读端原语中。为此，如果可抢占 RCU 的 rcu_read_lock() 实现能够被内联就好了，然而，这样做需要解决与 `task_struct` 结构的 `#include` 问题。

Linux 内核支持多达 4096 个 CPU 的硬件配置，这意味着 RCU 必须极具可扩展性。在 RCU 实现内部，涉及频繁获取全局锁或频繁对全局变量做原子操作的算法是绝对无法容忍的。RCU 因此大量使用基于 `rcu_node` 结构的组合树。RCU 必须能够容忍所有 CPU 持续地以任意组合调用 RCU 的运行时原语，且每次操作的开销极小。事实上，在许多情况下，负载的增加必须**降低**每次操作的开销，synchronize_rcu()、call_rcu()、synchronize_rcu_expedited() 和 rcu_barrier() 的批处理优化就是明证。作为一般规则，RCU 必须欣然接受 Linux 内核其余部分决定抛给它的任何东西。

Linux 内核被用于实时工作负载，尤其是与 `-rt patchset <https://wiki.linuxfoundation.org/realtime/>`__ 结合使用时。实时延迟响应需求使得那种在 RCU 读端临界区上跨区禁用抢占的传统做法变得不合适。用 `CONFIG_PREEMPTION=y` 构建的内核因此使用一种允许 RCU 读端临界区被抢占的 RCU 实现。这一需求是在用户明确表示早期的一个 `real-time patch <https://lwn.net/Articles/107930/>`__ 不满足他们的需求、并结合 -rt patchset 极早期版本遇到的某些 `RCU issues <https://lore.kernel.org/r/20050318031826.GA2693@us.ibm.com>`__ 之后才显现出来的。

此外，RCU 必须在一个小于 100 微秒的实时延迟预算内将就应付。事实上，在使用 -rt patchset 的小型系统上，
Linux 内核为整个内核（包括 RCU）提供亚 20 微秒的实时延迟。RCU 的可扩展性和延迟因此必须足以满足这类配置。令我惊讶的是，亚 100 微秒的实时延迟预算 `applies to even the largest systems [PDF] <http://www.rdrop.com/users/paulmck/realtime/paper/bigrt.2013.01.31a.LCA.pdf>`__，并且一直涵盖到拥有 4096 个 CPU 的系统。这一实时需求促使了宽限期 kthread 的诞生，它也简化了若干竞态条件的处理。

RCU 必须避免降低 CPU 密集型线程的实时响应，无论这些线程执行于用户态（这是 `CONFIG_NO_HZ_FULL=y` 的一个用例）还是在内核中。话虽如此，内核中的 CPU 密集型循环必须至少每几十毫秒执行一次 cond_resched()，以避免收到来自 RCU 的 IPI。

最后，RCU 作为同步原语的地位意味着，任何 RCU 故障都可能导致任意的内存损坏，而这可能极其难以调试。这意味着 RCU 必须极其可靠，这在实践中也意味着 RCU 必须有一个激进的压力测试套件。这个压力测试套件叫做 `rcutorture`。

尽管对 `rcutorture` 的需求并不令人意外，但 Linux 内核当前极高的普及度正在带来有趣——或许也是前所未有——的验证挑战。要理解这一点，请记住，鉴于 Android 智能手机、由 Linux 驱动的电视和服务器，当今正在运行的 Linux 内核实例远远超过十亿。随着声名显赫的物联网的到来，这个数字预计会急剧增加。

假设 RCU 含有一个平均每一百万年运行时长出现一次的竞态条件。在整个装机量上，这个 bug 大约每天会发生三次。RCU 当然可以躲藏在硬件错误率背后，毕竟没人真的指望自己的智能手机能用一百万年。然而，任何从这个想法中获得太多安慰的人，都应该考虑这样一个事实：在大多数司法管辖区，对一个给定机制（其中可能包括 Linux 内核）成功的多年测试，就足以满足若干类型的安全关键认证。事实上，有传言说 Linux 内核已经用于安全关键型的生产应用。我不知道你怎么想，但如果 RCU 中的一个 bug 害死了人，我会感到相当糟糕。这也许可以解释我最近对验证与确认的关注。

### 其他 RCU 风格


RCU 更令人惊讶的事情之一是，它现在至少有五**种风格**（flavor），或称 API 家族。此外，迄今为止一直是唯一焦点的主风格有两种不同的实现：不可抢占的和可抢占的。其他四种风格列在下面，每种的需求在单独的节中描述。

#. `Bottom-Half Flavor (Historical)`_
#. `Sched Flavor (Historical)`_
#. `Sleepable RCU`_
#. `Tasks RCU`_
#. `Tasks Trace RCU`_

#### 底半部风格（历史）


RCU 的 RCU-bh 风格此后已用其他 RCU 风格表达，作为将三种风格整合为单一风格的一部分。读端 API 保留了下来，并继续禁用软中断，且继续由 lockdep 记账。因此，本节中的大部分材料严格来说是历史性质的。

RCU 的软中断禁用（又称“底半部”，hence 缩写为“_bh”）风格，或称 **RCU-bh**，由 Dipankar Sarma 开发，用于提供一种能够承受 Robert Olsson 所研究的基于网络的拒绝服务攻击的 RCU 风格。这些攻击给系统施加了如此之大的网络负载，以至于某些 CPU 永远不退出软中断执行，而这反过来又阻止了那些 CPU 执行任何上下文切换，在当时的 RCU 实现中，这就阻止了宽限期永远结束。结果是内存耗尽和系统挂起。

解决方案是创建 RCU-bh，它在自己的读端临界区上做 local_bh_disable()，并且除了上下文切换、空闲、用户态和离线之外，还把从一种软中断处理到另一种软中断处理的转换当作一个静止状态。这意味着即便某些 CPU 无限期地执行于软中断中，RCU-bh 宽限期也能完成，从而让基于 RCU-bh 的算法能够承受基于网络的拒绝服务攻击。

因为 rcu_read_lock_bh() 和 rcu_read_unlock_bh() 禁用和重新启用软中断处理程序，任何在 RCU-bh 读端临界区期间启动软中断处理程序的尝试都会被推迟。在这种情况下，rcu_read_unlock_bh() 会调用软中断处理，这可能需要相当长的时间。当然可以争辩说，这个软中断开销应当归属于 RCU-bh 读端临界区之后的代码，而不是 rcu_read_unlock_bh()，但事实是，大多数性能分析工具无法做到这种精细的区分。例如，假设一个三毫秒长的 RCU-bh 读端临界区在网络负载沉重时执行。在那三毫秒内极有可能会尝试调用至少一个软中断处理程序，但任何此类调用都会被推迟到 rcu_read_unlock_bh() 的时刻。这当然会让人第一眼看上去以为是 rcu_read_unlock_bh() 执行得非常慢。

`RCU-bh API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 包括 rcu_read_lock_bh()、rcu_read_unlock_bh()、rcu_dereference_bh()、rcu_dereference_bh_check() 和 rcu_read_lock_bh_held()。然而，旧的 RCU-bh 更新端 API 现在已经没有了，取而代之的是 synchronize_rcu()、synchronize_rcu_expedited()、call_rcu() 和 rcu_barrier()。此外，任何禁用底半部的操作也都标记了一个 RCU-bh 读端临界区，包括 local_bh_disable() 和 local_bh_enable()、local_irq_save() 和 local_irq_restore() 等等。

#### 调度风格（历史）


RCU 的 RCU-sched 风格此后已用其他 RCU 风格表达，作为将三种风格整合为单一风格的一部分。读端 API 保留了下来，并继续禁用抢占，且继续由 lockdep 记账。因此，本节中的大部分材料严格来说是历史性质的。

在可抢占 RCU 之前，等待一个 RCU 宽限期还有一个副作用，就是也会等待所有既存的Interrupt和 NMI 处理程序。然而，存在一些合法的可抢占 RCU 实现并不具备这一性质，因为代码中点位于 RCU 读端临界区之外的任何位置都可以是一个静止状态。因此，创建了 **RCU-sched**，它遵循“经典”RCU，即一个 RCU-sched 宽限期会等待既存的Interrupt和 NMI 处理程序。在用 `CONFIG_PREEMPTION=n` 构建的内核中，RCU 和 RCU-sched 的 API 有着相同的实现，而用 `CONFIG_PREEMPTION=y` 构建的内核则为每个提供了单独的实现。

请注意，在 `CONFIG_PREEMPTION=y` 内核中，rcu_read_lock_sched() 和 rcu_read_unlock_sched() 分别禁用和重新启用抢占。这意味着在 RCU-sched 读端临界区期间如果有抢占尝试，rcu_read_unlock_sched() 将会进入调度器，带来随之而来的所有延迟和开销。正如 rcu_read_unlock_bh() 一样，这可能让人看起来像是 rcu_read_unlock_sched() 执行得很慢。然而，最高优先级的任务不会被抢占，因此该任务会享有低开销的 rcu_read_unlock_sched() 调用。

`RCU-sched API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 包括 rcu_read_lock_sched()、rcu_read_unlock_sched()、rcu_read_lock_sched_notrace()、rcu_read_unlock_sched_notrace()、rcu_dereference_sched()、rcu_dereference_sched_check() 和 rcu_read_lock_sched_held()。然而，旧的 RCU-sched 更新端 API 现在已经没有了，取而代之的是 synchronize_rcu()、synchronize_rcu_expedited()、call_rcu() 和 rcu_barrier()。此外，任何禁用抢占的操作也都标记了一个 RCU-sched 读端临界区，包括 preempt_disable() 和 preempt_enable()、local_irq_save() 和 local_irq_restore() 等等。

#### 可睡眠 RCU


十多年来，只要有人说“我需要在 RCU 读端临界区内阻塞”，这都是一个可靠的信号，表明这个人不懂 RCU。毕竟，如果你总是在 RCU 读端临界区内阻塞，那你大概可以负担得起使用一个开销更高的同步机制。然而，随着 Linux 内核通知器的出现，情况改变了，它们的 RCU 读端临界区几乎从不睡眠，但有时又需要睡眠。这导致了 `sleepable RCU <https://lwn.net/Articles/202847/>`__（即可睡眠 RCU，或称 **SRCU**）的引入。

SRCU 允许定义不同的域（domain），每个域由一个 `srcu_struct` 结构的实例定义。必须把这个结构的指针传入每个 SRCU 函数，例如 `synchronize_srcu(&ss)`，其中 `ss` 是 `srcu_struct` 结构。这些域的关键好处是，一个域中较慢的 SRCU 读者不会延迟另一个域中的 SRCU 宽限期。话虽如此，这些域的一个后果是，读端代码必须在 srcu_read_lock() 和 srcu_read_unlock() 之间传递一个“cookie”，例如如下：

```

       1 int idx;
       2
       3 idx = srcu_read_lock(&ss);
       4 do_something();
       5 srcu_read_unlock(&ss, idx);

```

如上所述，在 SRCU 读端临界区中阻塞是合法的，然而，能力越大，责任越大。如果你在某个给定域的 SRCU 读端临界区中永远阻塞，那么该域的宽限期也会永远被阻塞。当然，永远阻塞的一个好办法就是死锁，如果某个给定域的 SRCU 读端临界区中的任何操作能够直接或间接地等待该域的宽限期过去，死锁就可能发生。例如，这会导致自死锁：

```

       1 int idx;
       2
       3 idx = srcu_read_lock(&ss);
       4 do_something();
       5 synchronize_srcu(&ss);
       6 srcu_read_unlock(&ss, idx);

```

然而，如果第 5 行获取了一把在 `ss` 域的 synchronize_srcu() 期间被持有的互斥锁，仍然可能发生死锁。此外，如果第 5 行获取了一把在另一个域 `ss1` 的 synchronize_srcu() 期间被持有的互斥锁，并且如果某个 `ss1` 域的 SRCU 读端临界区获取了另一把在 `ss` 域的 synchronize_srcu() 期间被持有的互斥锁，死锁同样可能发生。这样的死锁环可以跨越任意多个不同的 SRCU 域。再次强调，能力越大，责任越大，不过 lockdep 现在能够检测这类死锁。

与其他 RCU 风格不同，SRCU 读端临界区可以运行在空闲、甚至离线的 CPU 上，srcu_read_lock_fast() 及其同类除外。这一能力要求 srcu_read_lock() 和 srcu_read_unlock() 包含内存屏障，这意味着 SRCU 读者的运行会比 RCU 读者稍慢一些。它也促成了 smp_mb__after_srcu_read_unlock() API，它与 srcu_read_unlock() 结合，保证一条完整内存屏障。

同样与其他 RCU 风格不同，synchronize_srcu() **不能**从 CPU 热插拔通知器中调用，原因是 SRCU 宽限期利用了定时器，以及定时器可能临时“滞留”在离线的 CPU 上的可能性。这种定时器的滞留意味着投递给离线 CPU 的定时器在 CPU 热插拔过程后期之前不会触发。问题在于，如果一个通知器正在等待一个 SRCU 宽限期，而那个宽限期正在等待一个定时器，而那个定时器滞留在该离线的 CPU 上，那么该通知器就永远不会被唤醒，换言之，发生了死锁。当然，同样的情况也禁止了从 CPU 热插拔通知器中调用 srcu_barrier()。

SRCU 与其他 RCU 风格的另一处不同在于，SRCU 的加速（expedited）和非加速宽限期是由同一机制实现的。这意味着在当前的 SRCU 实现中，加速一个未来的宽限期有一个副作用，就是加速了所有尚未完成的先前宽限期。（但请注意，这是当前实现的一个属性，未必是未来实现的属性。）此外，如果 SRCU 已经空闲了超过 `srcutree.exp_holdoff` 内核引导参数所指定的间隔（默认 25 微秒），并且如果一次 synchronize_srcu() 调用结束了这个空闲期，那么该调用会被自动加速。

自 v4.12 起，SRCU 的回调是每-CPU 维护的，消除了先前内核版本中存在的一个加锁瓶颈。尽管这将允许用户对 call_srcu() 施加更重的压力，但重要的是要注意，SRCU 尚未采取任何特殊步骤来应对回调洪泛。所以如果你每 CPU 每秒提交（比如）10,000 个 SRCU 回调，你大概完全没问题；但如果你打算每 CPU 每秒提交（比如）1,000,000 个 SRCU 回调，请先运行一些测试。SRCU 可能确实需要一些调整来应对那种负载。当然，具体效果会因你的 CPU 速度和内存大小而异。

`SRCU API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 包括 srcu_read_lock()、srcu_read_unlock()、srcu_dereference()、srcu_dereference_check()、synchronize_srcu()、synchronize_srcu_expedited()、call_srcu()、srcu_barrier() 和 srcu_read_lock_held()。它还包括用于定义和初始化 `srcu_struct` 结构的 DEFINE_SRCU()、DEFINE_STATIC_SRCU()、DEFINE_SRCU_FAST()、DEFINE_STATIC_SRCU_FAST()、init_srcu_struct() 和 init_srcu_struct_fast() API。

更近一些，SRCU API 增加了轮询接口：

#. start_poll_synchronize_srcu() 返回一个标识未来 SRCU 宽限期完成的 cookie，并确保这个宽限期会被启动。
#. poll_state_synchronize_srcu() 在指定 cookie 对应于一个已经完成 SRCU 宽限期时返回 `true`。
#. get_state_synchronize_srcu() 返回与 start_poll_synchronize_srcu() 一样的 cookie，但区别在于它不做任何事情来确保任何未来的 SRCU 宽限期会被启动。

这些函数用于在某些具有多级老化机制的缓冲区缓存算法中避免不必要的 SRCU 宽限期。其思路是，等到该块完全从缓存中老化掉时，一个 SRCU 宽限期极有可能已经过去。

#### 任务 RCU


某些形式的跟踪使用“trampoline（跳板）”来处理安装不同类型探针所需的二进制重写。能够释放旧的 trampoline 会很好，这听起来像是某种形式 RCU 的活儿。然而，因为必须能够在代码中的任何位置安装跟踪，所以不可能使用像 rcu_read_lock() 和 rcu_read_unlock() 这样的读端标记。此外，把这些标记放在 trampoline 本身里也不行，因为 rcu_read_unlock() 之后需要有指令跟随。尽管 synchronize_rcu() 会保证执行到达了 rcu_read_unlock()，但它无法保证执行已经完全离开了 trampoline。更糟的是，在某些情况下，trampoline 的保护必须延伸到执行到达 trampoline **之前**的几条指令。例如，这几条指令可能会计算 trampoline 的地址，从而进入 trampoline 会在执行实际到达 trampoline 本身之前很久就被预先注定了。

解决方案以 `Tasks RCU <https://lwn.net/Articles/607117/>`__ 的形式出现，即拥有由自愿上下文切换所界定的隐式读端临界区，也就是对 schedule()、cond_resched() 和 synchronize_rcu_tasks() 的调用。此外，进出用户态执行的转换也界定了任务 RCU 读端临界区。空闲任务被 Tasks RCU 忽略，Tasks Rude RCU 可以用来与它们交互。

请注意，非自愿的上下文切换**不是** Tasks-RCU 静止状态。毕竟，在可抢占内核中，执行 trampoline 中代码的任务可能被抢占。在这种情况下，Tasks-RCU 宽限期在该任务恢复并且其执行离开该 trampoline 之前显然无法结束。这意味着，除其他外，cond_resched() 并不提供 Tasks RCU 静止状态。（取而代之，在软中断中使用 rcu_softirq_qs()，否则使用 rcu_tasks_classic_qs()。）

任务 RCU 的 API 相当紧凑，只包含 call_rcu_tasks()、synchronize_rcu_tasks() 和 rcu_barrier_tasks()。在 `CONFIG_PREEMPTION=n` 内核中，trampoline 不能被抢占，因此这些 API 分别映射为 call_rcu()、synchronize_rcu() 和 rcu_barrier()。在 `CONFIG_PREEMPTION=y` 内核中，trampoline 可以被抢占，因此这三个 API 由单独的函数实现，这些函数检查自愿上下文切换。

#### Tasks Rude RCU


某些形式的跟踪需要等待运行在任何在线 CPU 上的所有禁用抢占的代码区域，包括那些在 RCU 没有观察时执行的区域。这意味着 synchronize_rcu() 是不够的，必须改用 Tasks Rude RCU。这种 RCU 风格通过强制在每个在线 CPU 上调度一个工作队列来完成它的工作，hence 得了“Rude（粗鲁）”这个绰号。而实时工作负载（不想让它们的 `nohz_full` CPU 收到 IPI）以及电池供电的系统（不想让它们的空闲 CPU 被唤醒）都认为这个操作相当粗鲁。

一旦内核的进入/退出和深空闲函数被正确标记为 `noinstr`，Tasks RCU 就可以开始关注空闲任务（RCU 视角下空闲的那些除外），然后 Tasks Rude RCU 就可以从内核中移除了。
任务粗鲁 RCU 的 API 同样没有读端标记，因此相当紧凑，仅由 synchronize_rcu_tasks_rude() 组成。

#### 任务跟踪 RCU


某些形式的跟踪需要在读者中睡眠，但又无法容忍 SRCU 的读端开销，后者在 srcu_read_lock() 和 srcu_read_unlock() 中都包含一条完整内存屏障。这一需求由任务跟踪 RCU API 处理，它被实现为围绕 SRCU-fast 的轻量包装，从而避免了读端内存屏障，至少对于那些对内核进入/退出代码应用了 noinstr 的体系结构（或者构建时带有 `CONFIG_TASKS_TRACE_RCU_NO_MB=y` 的体系结构）而言如此。

既然实现基于 SRCU-fast，对 synchronize_rcu_tasks_trace() 的一次调用就隐含了至少一次对 synchronize_rcu() 的调用，也就是说，每一个任务跟踪 RCU 宽限期都至少包含一个普通的 RCU 宽限期。如果将来出现 synchronize_rcu_tasks_trace_expedited()，这一保证**不一定**适用于这个假想的 API 成员。

任务跟踪 RCU 的 API 也相当紧凑，由 rcu_read_lock_trace()、rcu_read_unlock_trace()、rcu_read_lock_trace_held()、call_rcu_tasks_trace()、synchronize_rcu_tasks_trace() 和 rcu_barrier_tasks_trace() 组成。

### 可能的未来变更


RCU 用来获得更新端可扩展性的一个技巧是，随着 CPU 数量的增加而提高宽限期延迟。如果这成为一个严重问题，就有必要重新设计宽限期状态机，以避免对这种额外延迟的需求。

RCU 在少数地方禁用了 CPU 热插拔，最著名的或许是在 rcu_barrier() 操作中。如果有强烈理由要在 CPU 热插拔通知器中使用 rcu_barrier()，就有必要避免禁用 CPU 热插拔。这会引入一些复杂性，所以最好有一个**非常**好的理由。

宽限期延迟与对其他 CPU 的打扰这两者之间的权衡可能需要重新审视。当然，理想情况是既有零宽限期延迟，又在加速宽限期操作期间产生零处理器间中断。尽管这个理想不太可能实现，但进一步改进是相当有可能的。

RCU 的多处理器实现使用一棵组合树来对 CPU 分组，以减少锁竞争并增加缓存局部性。然而，这棵组合树并没有把它的内存分散到 NUMA 节点上，也没有把 CPU 组与插槽或核这样的硬件特性对齐。目前认为这种分散和对齐是不必要的，因为热路径上的读端原语并不访问组合树，常见情况下的 call_rcu() 也不会。如果你认为你的体系结构需要这种分散和对齐，那么你的体系结构也应该能从 `rcutree.rcu_fanout_leaf` 引导参数受益，它可以设置为一个插槽、NUMA 节点或随便什么中的 CPU 数量。如果 CPU 数量太大，就使用 CPU 数量的一个分数。如果 CPU 数量是一个很大的素数，嗯，那绝对是一个“有趣”的体系结构选择！更灵活的安排或许会被考虑，但前提是 `rcutree.rcu_fanout_leaf` 已被证明不够用，并且这种不够用已经通过一个仔细运行且现实的系统级工作负载得到证实。

请注意，要求 RCU 重新映射 CPU 编号的安排，需要极好地证明需求的存在，并充分探索替代方案。

RCU 的各种 kthread 是相当近期的添加。很可能需要进行调整，以更优雅地应对极端负载。可能还需要能够把 RCU 的 kthread 和软中断处理程序造成的 CPU 占用，归咎到引发这一 CPU 占用的代码头上。例如，RCU 回调开销或许会被回溯记到发起的 call_rcu() 实例头上，尽管在生产内核中大概不会这样做。

可能需要额外的工作，以在重负载下为宽限期和回调调用提供合理的向前推进保证。

### 总结


本文档呈现了超过二十年的 RCU 需求。鉴于这些需求一直在变化，这不会是有关这个主题的最后定论，但至少它有助于把一个重要的需求子集阐述清楚。

### 致谢


我感谢 Steven Rostedt、Lai Jiangshan、Ingo Molnar、Oleg Nesterov、Borislav Petkov、Peter Zijlstra、Boqun Feng 和 Andy Lutomirski 在把这篇文章变得人类可读方面提供的帮助，也感谢 Michelle Rankin 对这项工作的支持。其他的贡献在 Linux 内核的 git 归档中得到致谢。
