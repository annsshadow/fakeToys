## 漫游 TREE_RCU 的数据结[LWN.net]


2016 骞?12 鏈?18 鏃。

本文Paul E. McKenney 贡献

## 引言


本文档描RCU 的主要数据结构以及它们之间的相互关系

## 数据结构间的关系


就所有意图和目的而言，RCU 是一个大型状态机，其数据结构以维持状态的方式，让 RCU 读者能够极快地执行，同时还能以高效且极具可扩展的方式处理更新者所请求RCU 宽限期（grace period）。RCU 更新者的效率与可扩展性主要由一棵组合树（combining tree）提供，如下所示：


该图显示了一个外层的 `rcu_state` 结构，包含一`rcu_node` 结构树。该 `rcu_node` 树的每个叶节点最多关16 `rcu_data` 结构，因此共`NR_CPUS` `rcu_data` 结构，每个可能的 CPU 各一个。如果需要，这个结构会在引导时进行调整，以处`nr_cpu_ids` 远小`NR_CPUs` 的常见情况。例如，许多 Linux 发行版将 `NR_CPUs=4096`，这会生成一个三级的 `rcu_node` 树。如果实际硬件只16 CPU，RCU 会在引导时自我调整，从而生成一个只有单个节点的 `rcu_node` 树

这棵组合树的目的是让CPU 的事件（例如静止状态（quiescent state）、dyntick-idle 转换，以CPU 热插拔操作）能够被高效且可扩展地处理。静止状态由CPU `rcu_data` 结构记录，而其他事件由叶级`rcu_node` 结构记录。所有这些事件在树的每一层被合并，直到最终在树根`rcu_node` 结构处完成宽限期。一旦每CPU（或者在 `CONFIG_PREEMPT_RCU` 的情况下，每个任务）都经过了一次静止状态，根节点处的宽限期就可以完成。一旦宽限期完成，这一事实会被沿树向下传播

从图中可见，64 位系统上，一个具64 个叶子的两级树可以容1,024 CPU，根节点的扇出（fanout）为 64，叶子的扇出16

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 为什么叶子的扇出不也64                                           |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 因为影响叶级 `rcu_node` 结构的事件类型，比树上更高层更多。因此，如果|
| `rcu_node` 结构的扇出为 64，这些结构的 `->structures` 上的竞争    |
| 会变得过分严重。在各种系统上的实验表明，扇出为 16 `rcu_node` 树的  |
| 叶子来说效果很好                                                   |
|                                                                       |
| 当然，在拥有数百或数千个 CPU 的系统上的进一步经验可能表明，非叶       |
| `rcu_node` 结构的扇出也必须降低。如果有必要，这种降低可以很容易   |
| 进行。同时，如果你正在使用这样的系统，并且在非叶 `rcu_node` 结构  |
| 遇到了竞争问题，你可以使`CONFIG_RCU_FANOUT` 内核配置参数来按需    |
| 降低非叶扇出                                                       |
|                                                                       |
| 为具有强 NUMA 特性的系统构建的内核可能也需要调`CONFIG_RCU_FANOUT`，|
| 以使 `rcu_node` 结构的域与硬件边界对齐。然而，迄今为止尚无此必要  |
+-----------------------------------------------------------------------+

如果你的系统拥有超过 1,024 CPU（或32 位系统上超过 512 CPU），RCU 会自动为树添加更多层级。例如，如果你疯狂到去构建一个拥65,536 CPU 64 位系统，RCU 会将 `rcu_node` 树配置如下：


RCU 目前允许最多四级树，在 64 位系统上可容纳多4,194,304 CPU，而在 32 位系统上则只524,288 CPU。另一方面，你可以`CONFIG_RCU_FANOUT` `CONFIG_RCU_FANOUT_LEAF` 都设置为小到 2，这会使得一16-CPU 的测试使用四级树。这对于在小型测试机上测试大型系统的能力很有用

这种多级组合树让我们能够获得分区的绝大部分性能和可扩展性优势，即便 RCU 宽限期检测本质上是一个全局操作。其中的诀窍在于：只有最后一个向给定 `rcu_node` 结构报告静止状态的 CPU，才需要向上推进到树的上一`rcu_node` 结构。这意味着在叶`rcu_node` 结构上，16 次访问中只有 1 次会向上推进。对于内部的 `rcu_node` 结构，情况更极端4 次访问中只有 1 次会向上推进。由于绝大多CPU 都不会向上推进，锁竞争在树中保持大致恒定。无论系统中有多少个 CPU，每个宽限期最多只64 个静止状态报告会一路推进到`rcu_node` 结构，从而确保该`rcu_node` 结构上的锁竞争保持可接受地低

实际上，组合树就像一个大的减震器，无论系统负载如何，都能在所有树层级上控制锁竞争

RCU 更新者通过注册 RCU 回调来等待正常的宽限期，这些回调或者直接通过 `call_rcu()`，或者间接通过 `synchronize_rcu()` 及其相关函数。RCU 回调`rcu_head` 结构表示，在等待宽限期过去时被排队在 `rcu_data` 结构上，如下图所示：


该图展示`TREE_RCU` `PREEMPT_RCU` 的主要数据结构是如何关联的。次要的数据结构将随使用它们的算法一起介绍

请注意，上图中的每种数据结构都有其自身的同步方式

#. 每个 `rcu_state` 结构都有一个锁和一个互斥体，并且某些字段由相应的根 `rcu_node` 结构的锁保护
#. 每个 `rcu_node` 结构都有一个自旋锁
#. `rcu_data` 中的字段归相应的 CPU 私有，尽管有少数字段可被其他 CPU 读写

需要注意的是，不同的数据结构在任意给定时刻RCU 状态可能有非常不同的认识。仅举一例：对给RCU 宽限期开始或结束的感知是缓慢地在这些数据结构中传播的。这种缓慢的传播对于 RCU 拥有良好的读端性能是绝对必要的。如果这种各自为政的实现对你而言显得陌生，一个有用的技巧是把这些数据结构的每个实例都视为不同的人，每个人都对现实有着通常略有差异的看法

这些数据结构中每一种的大致角色如下

#. `rcu_state`：该结构形成 `rcu_node` `rcu_data` 结构之间的互连，跟踪宽限期，充当CPU 热插拔事件而变为孤儿的回调的短期存放处，维`rcu_barrier()` 状态，跟踪加速（expedited）宽限期状态，并维护用于在宽限期过长时强制静止状态的状态
#. `rcu_node`：该结构形成组合树，将静止状态信息从叶子传播到根，也将宽限期信息从根传播到叶子。它提供宽限期状态的本地副本，以便能够以同步方式访问这些信息，而免受否则会由全局锁带来的可扩展性限制。在 `CONFIG_PREEMPT_RCU` 内核中，它管理在其当RCU 读端临界区中阻塞的任务列表。在带有 `CONFIG_RCU_BOOST` `CONFIG_PREEMPT_RCU` 中，它管理每 `rcu_node` 的优先级提升（priority-boosting）内核线程（kthread）及其状态。最后，它记CPU 热插拔状态，以确定在给定的宽限期内应忽略哪些 CPU
#. `rcu_data`：这个每 CPU 结构是静止状态检测和 RCU 回调排队的焦点。它还跟踪其与相应叶 `rcu_node` 结构的关系，以便更高效地`rcu_node` 组合树向上传播静止状态。与 `rcu_node` 结构类似，它提供宽限期信息的本地副本，以便相CPU 能够免锁地同步访问该信息。最后，该结构记录相CPU 过去dyntick-idle 状态，并跟踪统计信息
#. `rcu_head`：该结构表示 RCU 回调，并且是唯一RCU 用户分配和管理结构。`rcu_head` 结构通常内嵌在受 RCU 保护的数据结构之中

如果你从本文中只想要一个关RCU 数据结构如何关联的总体概念，那么你已经读完了。否则，接下来的每一节都会更详细地介`rcu_state`、`rcu_node` `rcu_data` 数据结构

#### ``rcu_state`` 结构


`rcu_state` 结构是表示系统中 RCU 状态的基础结构。该结构形成 `rcu_node` `rcu_data` 结构之间的互连，跟踪宽限期，包含用于CPU 热插拔事件同步的锁，并维护在宽限期过长时用于强制静止状态的状态

`rcu_state` 结构的少数几个字段将在下面的各节中单独或成组地讨论。那些更专门的字段将在讨论其用法时涵盖

####### rcu_node rcu_data 结构的关


`rcu_state` 结构的这一部分是这样声明的

```

     1   struct rcu_node node[NUM_RCU_NODES];
     2   struct rcu_node *level[NUM_RCU_LVLS + 1];
     3   struct rcu_data __percpu *rda;

```
+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 等一下！你说`rcu_node` 结构形成的是一棵树，但它们却被声明为一|
| 扁平数组！这是怎么回事                                             |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 这棵树是布局在数组中的。数组中的第一个节点是头节点，数组中接下来  |
| 一组节点是头节点的子节点，依此类推，直到数组中的最后一组节点是叶子|
| 见下面的图示以了解其工作原理                                       |
+-----------------------------------------------------------------------+

`rcu_node` 树以内嵌方式放入 `->node[]` 数组中，如下图所示：


这种映射一个有趣的后果是，对树的广度优先遍历被实现为对数组的简单线性扫描，而这实际上正`rcu_for_each_node_breadth_first()` 宏所做的。该宏在宽限期的开始和结束处被使用

`->level` 数组的每个条目都引用树中相应层级的第一`rcu_node` 结构，例如如下所示：


该数组的第零（`th`）个元素引用`rcu_node` 结构，第一个元素引用根 `rcu_node` 的第一个子节点，最后第二个元素引用第一个叶 `rcu_node` 结构

无论如何，如果你把树画成树形而非数组形，很容易画出一个平面表示：


最后，`->rda` 字段引用一个每 CPU 指针，指向相CPU `rcu_data` 结构

初始化完成后，所有这些字段都是常量，因此不需要保护

####### 瀹介檺鏈熻窡韪。


`rcu_state` 结构的这一部分是这样声明的

```

     1   unsigned long gp_seq;

```
RCU 宽限期是编号的，`->gp_seq` 字段包含当前的宽限期序列号。最低的 2 位表示当前宽限期的状态，可以0（尚未开始）1（进行中）。换言之，如果 `->gp_seq` 的最2 位为 0，那RCU 是空闲的。最2 位的任何其他值都表示出了问题。该字段由根 `rcu_node` 结构`->lock` 字段保护

`rcu_node` `rcu_data` 结构中也存在 `->gp_seq` 字段。处`rcu_state` 结构中的字段代表最新的值，而其他结构中的字段会被拿来比较，以便以分布式的方式检测宽限期的开始和结束。这些值从 `rcu_state` 流向 `rcu_node`（从根到叶沿树向下）再到 `rcu_data`

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 既然rcu_node 结构有一gp_seq 字段，为什RCU 还要rcu_state   |
| 结构中维护一个单独的 gp_seq？为什么不直接把根 rcu_node gp_seq 当作 |
| 正式记录，并在开始新宽限期时直接更新它？                              |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 在单节点 RCU 树（根节点同时也是叶子）上，立即更新根节点的 gp_seq   |
| 造成不必要的锁竞争。原因如下：                                        |
|                                                                       |
| 如果我们直接在根节点gp_seq 上执rcu_seq_start()                |
|                                                                       |
| 1. 所CPU 会立即从各自 rdp gp_seq 中看到其节点gp_seq，在      |
|    rcu_pending() 中。它们随后都会调RCU 核心                     |
| 2. 这会调用 note_gp_changes() 并尝试获取节点锁                     |
| 3. rnp->qsmask 尚未初始化（稍后rcu_gp_init() 中进行）          |
| 4. 因此每个 CPU 都会获取锁，发现它无法确定是否需要报告静止状态（没有  |
|    qsmask），更新 rdp->gp_seq，然后释放锁                          |
| 5. 结果：大量锁获取，却没有宽限期进                                |
|                                                                       |
| 通过设置一个单独的 rcu_state.gp_seq，我们可以递增正式的宽限期计数器， |
| 而不立即影响CPU 在其节点中看到的内容。rcu_gp_init() 中的层级传播   |
| 随后在同一把锁的获取下，一起更新根节点gp_seq qsmask，从而避免了 |
| 这种无谓的竞争                                                     |
+-----------------------------------------------------------------------+

####### 杂项


`rcu_state` 结构的这一部分是这样声明的

```

     1   unsigned long gp_max;
     2   char abbr;
     3   char *name;

```
`->gp_max` 字段jiffies 为单位跟踪最长宽限期的持续时间。它由根 `rcu_node` `->lock` 保护

`->name` `->abbr` 字段用于区分可抢RCU（“rcu_preempt“p”）与不可抢RCU（“rcu_sched“s”）。这些字段用于诊断和跟踪目的

#### ``rcu_node`` 结构


`rcu_node` 结构形成组合树，将静止状态信息从叶子传播到根，也将宽限期信息从根向下传播到叶子。它们提供宽限期状态的本地副本，以便能够以同步方式访问这些信息，而免受否则会由全局锁带来的可扩展性限制。在 `CONFIG_PREEMPT_RCU` 内核中，它们管理在其当前 RCU 读端临界区中阻塞的任务列表。在带有 `CONFIG_RCU_BOOST` `CONFIG_PREEMPT_RCU` 中，它们管理`rcu_node` 的优先级提升（priority-boosting）内核线程（kthread）及其状态。最后，它们记录 CPU 热插拔状态，以确定在给定的宽限期内应忽略哪些 CPU

`rcu_node` 结构的字段将在下面的各节中单独或成组地讨论

####### 与组合树的连


`rcu_node` 结构的这一部分是这样声明的

```

     1   struct rcu_node *parent;
     2   u8 level;
     3   u8 grpnum;
     4   unsigned long grpmask;
     5   int grplo;
     6   int grphi;

```
`->parent` 指针引用树中上一层的 `rcu_node`，对于根 `rcu_node` 则为 `NULL`。RCU 实现大量使用此字段将静止状态沿树向上推送。`->level` 字段给出树中的层级，根为0 级，其子节点为第 1 级，依此类推。`->grpnum` 字段给出此节点在其父节点的子节点中的位置，因此该数字32 位系统上介于 0 31 之间，在 64 位系统上介于 0 63 之间。`->level` `->grpnum` 字段仅用于初始化和跟踪。`->grpmask` 字段`->grpnum` 的位掩码对应物，因此始终恰好设置一位。该掩码用于清除其父节点位掩码中对应于此 `rcu_node` 结构的那一位，这将在后面描述。最后，`->grplo` `->grphi` 字段分别包含`rcu_node` 结构所服务的最低和最高编号的 CPU

所有这些字段都是常量，因此不需要任何同步

####### 同步


`rcu_node` 结构的这一字段是这样声明的

```

     1   raw_spinlock_t lock;

```
除非另有说明，此字段用于保护本结构中其余的字段。话虽如此，出于跟踪目的，本结构中的所有字段都可以在不加锁的情况下访问。是的，这可能导致令人困惑的跟踪信息，但总比因为海森bug（heisenbug）而让这些信息消失要好


####### 瀹介檺鏈熻窡韪。


`rcu_node` 结构的这一部分是这样声明的

```

     1   unsigned long gp_seq;
     2   unsigned long gp_seq_needed;

```
`rcu_node` 结构`->gp_seq` 字段`rcu_state` 结构中同名字段相对应。它们各自可能滞后其 `rcu_state` 对应物至多一步。如果给`rcu_node` 结构`->gp_seq` 字段的最2 位为 0，则`rcu_node` 结构认为 RCU 是空闲的

每个 `rcu_node` 结构`>gp_seq` 字段在每个宽限期的开始和结束时被更新

`->gp_seq_needed` 字段记录相应 `rcu_node` 结构所见到的最遥远的未来宽限期请求。当 `->gp_seq` 字段的值等于或超过 `->gp_seq_needed` 字段的值时，该请求即被视为已满足

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 假设这个 `rcu_node` 结构很长一段时间都没有看到请求。难`->gp_seq`  |
| 字段回绕不会引起问题吗？                                              |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 不会，因为如`->gp_seq_needed` 字段滞后`->gp_seq` 字段，`       |
| ->gp_seq_needed` 字段会在宽限期结束时更新。因此，模算术比较总是   |
| 得到正确答案，即便发生回绕也是如此                                 |
+-----------------------------------------------------------------------+

####### 静止状态跟


这些字段管理静止状态沿组合树向上的传播

`rcu_node` 结构的这一部分包含如下字段

```

     1   unsigned long qsmask;
     2   unsigned long expmask;
     3   unsigned long qsmaskinit;
     4   unsigned long expmaskinit;

```
`->qsmask` 字段跟踪`rcu_node` 结构的哪些子节点仍需为当前正常的宽限期报告静止状态。这样的子节点在其对应位上会有1。注意，`rcu_node` 结构应被视为`rcu_data` 结构作为它们的子节点。类似地，`->expmask` 字段跟踪`rcu_node` 结构的哪些子节点仍需为当前加速（expedited）宽限期报告静止状态。加速宽限期在概念属性上与正常宽限期相同，但加速实现接受极高的 CPU 开销以换取低得多的宽限期延迟，例如，消耗数十微秒量级的 CPU 时间，将宽限期持续时间从毫秒级降低到数十微秒级。`->qsmaskinit` 字段跟踪`rcu_node` 结构的哪些子节点覆盖了至少一个在CPU。该掩码用于初始`->qsmask`，`->expmaskinit` 用于分别在正常和加速宽限期的开始时初始`->expmask`

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 为什么这些位掩码要用锁来保护？得了吧，你难道没听说过原子指令吗？？？  |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 无锁的宽限期计算！多么诱人的可能性！但请考虑以下事件序列           |
|                                                                       |
| #. CPU 0 已经处于 dyntick-idle 模式相当长一段时间。当它醒来时，它注意到|
|    当前RCU 宽限期需要它报告，于是设置一个标志，让调度时钟中断能 |
|    找到它                                                          |
| #. 与此同时，CPU 1 正在运行 `force_quiescent_state()`，并注意CPU 0 |
|    一直处dyntick idle 模式，这构成一个扩展的静止状态            |
| #. CPU 0 的调度时钟中断在 RCU 读端临界区中间触发，并注意到 RCU 核心   |
|    需要某些东西，于是开RCU softirq 处理                         |
| #. CPU 0 softirq 处理程序执行，并且正要准备将其静止状态沿         |
|    `rcu_node` 树向上报告                                          |
| #. CPU 1 抢先一步，完成了当前宽限期并开始了一个新的宽限期      |
| #. CPU 0 现在为错误的宽限期报告了其静止状态。那个宽限期现在可能在该   |
|    RCU 读端临界区结束之前就结束了。如果发生这种情况，灾难就会降临 |
|                                                                       |
| 因此，绝对需要加锁，以协调位的清除与 `->gp_seq` 中宽限期序列号的更新。|
+-----------------------------------------------------------------------+

####### 阻塞任务管理


`PREEMPT_RCU` 允许任务在其 RCU 读端临界区中间被抢占，这些任务必须被显式跟踪。关于它们为何、如何被跟踪的细节将在一篇关RCU 读端处理的独立文章中介绍。目前，只需知道 `rcu_node` 结构会跟踪它们就足够了

```

     1   struct list_head blkd_tasks;
     2   struct list_head *gp_tasks;
     3   struct list_head *exp_tasks;
     4   bool wait_blkd_tasks;

```
`->blkd_tasks` 字段是已阻塞和被抢占任务列表的链表头。当任务RCU 读端临界区内经历上下文切换时，它们的 `task_struct` 结构会被入队（经`task_struct` `->rcu_node_entry` 字段）到执行外出上下文切换的 CPU 所对应的叶 `rcu_node` 结构`->blkd_tasks` 链表头部。当这些任务后来退出其 RCU 读端临界区时，它们会将自己从列表中移除。因此该列表是按时间倒序排列的，所以如果其中一个任务正在阻塞当前宽限期，那么所有后续任务也必定在阻塞同一个宽限期。因此，指向该列表中的单个指针就足以跟踪阻塞给定宽限期的所有任务。对于正常的宽限期，该指针存放在 `->gp_tasks` 中；对于加速宽限期，存放在 `->exp_tasks` 中。如果当前没有正在进行的宽限期，或者没有阻塞该宽限期完成的阻塞任务，则最后这两个字段`NULL`。如果这两个指针中的任何一个引用了一个正将自己从 `->blkd_tasks` 列表中移除的任务，那么该任务必须将指针推进到列表中的下一个任务，或者如果列表中没有后续任务则将指针设为 `NULL`

例如，假设任T1、T2 T3 都硬亲和（hard-affinitied）到系统中编号最大的 CPU。那么如果任T1 在一RCU 读端临界区内阻塞，随后启动了一个加速宽限期，然后任T2 在一RCU 读端临界区内阻塞，然后启动了一个正常宽限期，最后任T3 在一RCU 读端临界区内阻塞，那么最后一个叶 `rcu_node` 结构的阻塞任务列表的状态将如下图所示：


任务 T1 正在阻塞两个宽限期，任务 T2 只阻塞正常宽限期，而任T3 两个宽限期都不阻塞。注意，这些任务在恢复执行时不会立即将自己从列表中移除。相反，它们会一直留在列表上，直到执行结束其 RCU 读端临界区的最外层 `rcu_read_unlock()`

`->wait_blkd_tasks` 字段指示当前宽限期是否正在等待一个被阻塞的任务

####### 调整 ``rcu_node`` 数组的大


`rcu_node` 数组的大小是通过一系列 C 预处理器表达式来确定的，如下所示：

```

    1 #ifdef CONFIG_RCU_FANOUT
    2 #define RCU_FANOUT CONFIG_RCU_FANOUT
    3 #else
    4 # ifdef CONFIG_64BIT
    5 # define RCU_FANOUT 64
    6 # else
    7 # define RCU_FANOUT 32
    8 # endif
    9 #endif
   10
   11 #ifdef CONFIG_RCU_FANOUT_LEAF
   12 #define RCU_FANOUT_LEAF CONFIG_RCU_FANOUT_LEAF
   13 #else
   14 # ifdef CONFIG_64BIT
   15 # define RCU_FANOUT_LEAF 64
   16 # else
   17 # define RCU_FANOUT_LEAF 32
   18 # endif
   19 #endif
   20
   21 #define RCU_FANOUT_1        (RCU_FANOUT_LEAF)
   22 #define RCU_FANOUT_2        (RCU_FANOUT_1 * RCU_FANOUT)
   23 #define RCU_FANOUT_3        (RCU_FANOUT_2 * RCU_FANOUT)
   24 #define RCU_FANOUT_4        (RCU_FANOUT_3 * RCU_FANOUT)
   25
   26 #if NR_CPUS <= RCU_FANOUT_1
   27 #  define RCU_NUM_LVLS        1
   28 #  define NUM_RCU_LVL_0        1
   29 #  define NUM_RCU_NODES        NUM_RCU_LVL_0
   30 #  define NUM_RCU_LVL_INIT    { NUM_RCU_LVL_0 }
   31 #  define RCU_NODE_NAME_INIT  { "rcu_node_0" }
   32 #  define RCU_FQS_NAME_INIT   { "rcu_node_fqs_0" }
   33 #  define RCU_EXP_NAME_INIT   { "rcu_node_exp_0" }
   34 #elif NR_CPUS <= RCU_FANOUT_2
   35 #  define RCU_NUM_LVLS        2
   36 #  define NUM_RCU_LVL_0        1
   37 #  define NUM_RCU_LVL_1        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_1)
   38 #  define NUM_RCU_NODES        (NUM_RCU_LVL_0 + NUM_RCU_LVL_1)
   39 #  define NUM_RCU_LVL_INIT    { NUM_RCU_LVL_0, NUM_RCU_LVL_1 }
   40 #  define RCU_NODE_NAME_INIT  { "rcu_node_0", "rcu_node_1" }
   41 #  define RCU_FQS_NAME_INIT   { "rcu_node_fqs_0", "rcu_node_fqs_1" }
   42 #  define RCU_EXP_NAME_INIT   { "rcu_node_exp_0", "rcu_node_exp_1" }
   43 #elif NR_CPUS <= RCU_FANOUT_3
   44 #  define RCU_NUM_LVLS        3
   45 #  define NUM_RCU_LVL_0        1
   46 #  define NUM_RCU_LVL_1        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_2)
   47 #  define NUM_RCU_LVL_2        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_1)
   48 #  define NUM_RCU_NODES        (NUM_RCU_LVL_0 + NUM_RCU_LVL_1 + NUM_RCU_LVL_2)
   49 #  define NUM_RCU_LVL_INIT    { NUM_RCU_LVL_0, NUM_RCU_LVL_1, NUM_RCU_LVL_2 }
   50 #  define RCU_NODE_NAME_INIT  { "rcu_node_0", "rcu_node_1", "rcu_node_2" }
   51 #  define RCU_FQS_NAME_INIT   { "rcu_node_fqs_0", "rcu_node_fqs_1", "rcu_node_fqs_2" }
   52 #  define RCU_EXP_NAME_INIT   { "rcu_node_exp_0", "rcu_node_exp_1", "rcu_node_exp_2" }
   53 #elif NR_CPUS <= RCU_FANOUT_4
   54 #  define RCU_NUM_LVLS        4
   55 #  define NUM_RCU_LVL_0        1
   56 #  define NUM_RCU_LVL_1        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_3)
   57 #  define NUM_RCU_LVL_2        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_2)
   58 #  define NUM_RCU_LVL_3        DIV_ROUND_UP(NR_CPUS, RCU_FANOUT_1)
   59 #  define NUM_RCU_NODES        (NUM_RCU_LVL_0 + NUM_RCU_LVL_1 + NUM_RCU_LVL_2 + NUM_RCU_LVL_3)
   60 #  define NUM_RCU_LVL_INIT    { NUM_RCU_LVL_0, NUM_RCU_LVL_1, NUM_RCU_LVL_2, NUM_RCU_LVL_3 }
   61 #  define RCU_NODE_NAME_INIT  { "rcu_node_0", "rcu_node_1", "rcu_node_2", "rcu_node_3" }
   62 #  define RCU_FQS_NAME_INIT   { "rcu_node_fqs_0", "rcu_node_fqs_1", "rcu_node_fqs_2", "rcu_node_fqs_3" }
   63 #  define RCU_EXP_NAME_INIT   { "rcu_node_exp_0", "rcu_node_exp_1", "rcu_node_exp_2", "rcu_node_exp_3" }
   64 #else
   65 # error "CONFIG_RCU_FANOUT insufficient for NR_CPUS"
   66 #endif

```
`rcu_node` 结构中的最大层级数目前被限制为四级，如21-24 行以及后续“if”语句的结构所规定。对32 位系统，这允16×32×32×32=524,288 CPU，至少在未来几年内应该是足够的。对64 位系统，允许 16×64×64×64=4,194,304 CPU，这应该能让我们撑过下一个十年左右。这四级树还允许使用 `CONFIG_RCU_FANOUT=8` 构建的内核支持多4096 CPU，这在每个插槽有八个 CPU 的非常大型系统中可能有用（但请注意，迄今还没有人证明过因插槽`rcu_node` 边界未对齐而导致的任何可测量的性能下降）。此外，构建具有完整四级 `rcu_node` 树的内核可以更好地测RCU 的组合树代码

`RCU_FANOUT` 符号控制 `rcu_node` 树中每个非叶层级允许有多少个子节点。如果未指定 `CONFIG_RCU_FANOUT` Kconfig 选项，它会根据系统的字长来设置，这也Kconfig 的默认值

`RCU_FANOUT_LEAF` 符号控制每个`rcu_node` 结构处理多少CPU。经验表明，允许给定的叶 `rcu_node` 结构处理 64 CPU（正64 位系统上 `->qsmask` 字段的位数所允许的那样）会导致叶 `rcu_node` 结构`->lock` 字段出现过度的竞争。因此，给定 `CONFIG_RCU_FANOUT_LEAF` 的默认值，每个`rcu_node` 结构CPU 数被限制16。如果未指定 `CONFIG_RCU_FANOUT_LEAF`，所选值基于系统的字长，就`CONFIG_RCU_FANOUT` 一样。第 11-19 行执行此计算

21-24 行计算由 `RCU_FANOUT` `RCU_FANOUT_LEAF` 指定的扇出分别支持的单级（包含单`rcu_node` 结构）、两级、三级和四级 `rcu_node` 树所能支持的最CPU 数。这CPU 数量分别保留`RCU_FANOUT_1`、`RCU_FANOUT_2`、`RCU_FANOUT_3` `RCU_FANOUT_4` C 预处理器变量中

这些变量用于控制跨越26-66 行的 C 预处理器 `#if` 语句，该语句计算树的每一层所需`rcu_node` 结构数量，以及所需的层级数。层级数由第 2754 54 行放`NUM_RCU_LVLS` C 预处理器变量。树最顶层`rcu_node` 结构数量始终恰好为一，此值由2865 55 行无条件放入 `NUM_RCU_LVL_0`。`rcu_node` 树的其余层级（如果有的话）是通过将最CPU 数除以从该层向下到当前层所支持的扇出来计算，并向上取整。此计算由第 376-47 56-58 行执行。第 31-330-420-52 62-63 行创lockdep 锁类名的初始化器。最后，64-66 行在最CPU 数对于指定扇出而言过大时产生一个错误

#### ``rcu_segcblist`` 结构


`rcu_segcblist` 结构维护一个分段（segmented）的回调列表，如下所示：

```

    1 #define RCU_DONE_TAIL        0
    2 #define RCU_WAIT_TAIL        1
    3 #define RCU_NEXT_READY_TAIL  2
    4 #define RCU_NEXT_TAIL        3
    5 #define RCU_CBLIST_NSEGS     4
    6
    7 struct rcu_segcblist {
    8   struct rcu_head *head;
    9   struct rcu_head **tails[RCU_CBLIST_NSEGS];
   10   unsigned long gp_seq[RCU_CBLIST_NSEGS];
   11   long len;
   12   long len_lazy;
   13 };

```
各段如下

#. `RCU_DONE_TAIL`：宽限期已经过去的回调。这些回调已准备好被调用
#. `RCU_WAIT_TAIL`：正在等待当前宽限期的回调。注意不同的 CPU 对于哪个宽限期是当前的可能有不同认识，因此有`->gp_seq` 字段
#. `RCU_NEXT_READY_TAIL`：正在等待下一个宽限期开始的回调
#. `RCU_NEXT_TAIL`：尚未与任何宽限期关联的回调

`->head` 指针引用第一个回调，如果列表不包含任何回调则`NULL`（这与为*不是**一回事）。`->tails[]` 数组的每个元素引用列表中相应段的最后一个回调的 `->next` 指针，或者如果该段及所有前面的段都为空，则引用列表`->head` 指针。如果相应的段为空但某些前面的段不为空，那么该数组元素与其前驱相同。较旧的回调更靠近列表头部，新回调被加在尾部。这`->head` 指针、`->tails[]` 数组与回调之间的关系如下图所示：


在此图中，`->head` 指针引用列表中的第一RCU 回调。`->tails[RCU_DONE_TAIL]` 数组元素引用 `->head` 指针本身，表明没有任何一个回调已准备好调用。`->tails[RCU_WAIT_TAIL]` 数组元素引用回调 CB 2 `->next` 指针，这表明 CB 1 CB 2 都在等待当前宽限期，对于具体哪个宽限期是当前的可能分歧暂且不论。`->tails[RCU_NEXT_READY_TAIL]` 数组元素引用`->tails[RCU_WAIT_TAIL]` 相同RCU 回调，这表明没有回调在等待下一RCU 宽限期。`->tails[RCU_NEXT_TAIL]` 数组元素引用 CB 4 `->next` 指针，表明所有剩余的 RCU 回调尚未被分配到某个 RCU 宽限期。注意，`->tails[RCU_NEXT_TAIL]` 数组元素总是引用最后一RCU 回调`->next` 指针，除非回调列表为空，在那种情况下它引`->head` 指针

对于 `->tails[RCU_NEXT_TAIL]` 数组元素还有一个额外的特殊情况：当此列表被**禁用**时，它可以为 `NULL`。当相应 CPU 离线，或者相CPU 的回调被卸载（offload）到一kthread 时，列表会被禁用，这两者都在别处描述

随着宽限期的推进，CPU 将其回调`RCU_NEXT_TAIL` 推进`RCU_NEXT_READY_TAIL`，再`RCU_WAIT_TAIL`，再`RCU_DONE_TAIL` 列表段

`->gp_seq[]` 数组记录与各列表段对应的宽限期编号。这允许不同CPU 对于哪个是当前宽限期有不同认识，同时仍避免过早调用它们的回调。特别是，这让长时间空闲CPU 能够在重新唤醒后确定它们的哪些回调已准备好被调用

`->len` 计数器包`->head` 中的回调数量，`->len_lazy` 包含那些已知仅释放内存、因此其调用可以被安全推迟的回调数量


   决定是否存在与本 `rcu_segcblist` 结构相关联的回调的是
   `->len` 字段*而非** `->head` 指针。原因是，所有已准备好调用的回调
   （即 `RCU_DONE_TAIL` 段中的那些）会在回调调用时（`rcu_do_batch`
   被一次性全部取出，由于这个原因，如果没有剩余未完成（not-done）的回调留在
   `rcu_segcblist` 中，`->head` 可能会被设为 NULL。如果回调调用必须被推迟
   例如因为一个高优先级进程刚刚在CPU 上唤醒，那么剩余的回调会被放
   `RCU_DONE_TAIL` 段，并且 `->head` 再次指向该段的开始。简而言之，即便 CPU
   始终都有回调存在，head 字段也可能短暂地`NULL`。因此，测试 `->head`
   指针是否`NULL` 是不合适的

相比之下，`->len` `->len_lazy` 计数仅在相应回调被调用之后才调整。这意味着 `->len` 计数只有`rcu_segcblist` 结构确实没有任何回调时才为零。当然，`->len` 计数的离 CPU 采样需要谨慎使用适当的同步，例如内存屏障。这种同步可能有点微妙，`rcu_barrier()` 的情况下尤为如此
#### ``rcu_data`` 结构


`rcu_data` 维护 RCU 子系统的CPU 状态。除非另有说明，本结构中的字段只能从相应CPU（以及从跟踪代码）访问。该结构是静止状态检测和 RCU 回调排队的焦点。它还跟踪其与相应叶 `rcu_node` 结构的关系，以便更高效地将静止状态沿 `rcu_node` 组合树向上传播。与 `rcu_node` 结构类似，它提供宽限期信息的本地副本，以便相CPU 能够免锁地同步访问该信息。最后，该结构记录相CPU 过去dyntick-idle 状态，并跟踪统计信息

`rcu_data` 结构的字段将在下面的各节中单独或成组地讨论

####### 与其他数据结构的关系


`rcu_data` 结构的这一部分是这样声明的

```

     1   int cpu;
     2   struct rcu_node *mynode;
     3   unsigned long grpmask;
     4   bool beenonline;

```
`->cpu` 字段包含相应 CPU 的编号，`->mynode` 字段引用相应`rcu_node` 结构。`->mynode` 用于沿组合树向上传播静止状态。这两个字段都是常量，因此不需要同步

`->grpmask` 字段指示 `->mynode->qsmask` 中对应于`rcu_data` 结构的位，并且在传播静止状态时也使用。`->beenonline` 标志在相CPU 上线时设置，这意味着 debugfs 跟踪无需转储出任何未设置此标志的 `rcu_data` 结构

####### 静止状态与宽限期跟


`rcu_data` 结构的这一部分是这样声明的

```

     1   unsigned long gp_seq;
     2   unsigned long gp_seq_needed;
     3   bool cpu_no_qs;
     4   bool core_needs_qs;
     5   bool gpwrap;

```
`->gp_seq` 字段`rcu_state` `rcu_node` 结构中同名字段的对应物。`->gp_seq_needed` 字段rcu_node 结构中同名字段的对应物。它们各自可能滞后其 `rcu_node` 对应物至多一步，但在 `CONFIG_NO_HZ_IDLE` `CONFIG_NO_HZ_FULL` 内核中，对于处于 dyntick-idle 模式CPU 可能滞后任意远（但这些计数器会在退dyntick-idle 模式时追上）。如果给`rcu_data` 结构`->gp_seq` 最2 位为 0，则`rcu_data` 结构认为 RCU 是空闲的

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 所有这些对宽限期编号的复制只会造成大规模的混乱。为什么不直接保留一|
| 全局序列号然后了事呢？？                                           |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 因为如果只有一个全局序列号，就需要一把全局锁才能安全地访问和更新它。|
| 如果我们不打算使用单一的全局锁，我们就需要在每节点（per-node）的基础|
| 仔细地管理这些编号。请回想前面某次快速测验的答案：将先前采样的静止状|
| 应用到错误的宽限期上，后果是相当严重的                             |
+-----------------------------------------------------------------------+

`->cpu_no_qs` 标志指示CPU 尚未经过一次静止状态，`->core_needs_qs` 标志指示 RCU 核心需要来自相CPU 的一次静止状态。`->gpwrap` 字段指示相应 CPU 已空闲了如此之久，以至于 `gp_seq` 计数器面临溢出危险，这将导致CPU 在下次退出空闲时忽略其计数器的值

####### RCU 回调处理


在没CPU 热插拔事件的情况下，RCU 回调由注册它们的同一CPU 调用。这严格来说是一种缓存局部性优化：回调可以、也确实会在非注册它们的 CPU 上被调用。毕竟，如果注册了给定回调的 CPU 在回调能被调用之前就已经离线，那确实别无选择

`rcu_data` 结构的这一部分是这样声明的

```

    1 struct rcu_segcblist cblist;
    2 long qlen_last_fqs_check;
    3 unsigned long n_cbs_invoked;
    4 unsigned long n_nocbs_invoked;
    5 unsigned long n_cbs_orphaned;
    6 unsigned long n_cbs_adopted;
    7 unsigned long n_force_qs_snap;
    8 long blimit;

```
`->cblist` 结构是前面描述过的分段回调列表。每当该 CPU 注意到另一RCU 宽限期已经完成时，它就会推进`rcu_data` 结构中的回调。CPU 通过注意到其 `rcu_data` 结构`->gp_seq` 字段的值与其叶 `rcu_node` 结构的值不同，来检测到 RCU 宽限期的完成。回想一下，每个 `rcu_node` 结构`->gp_seq` 字段在每个宽限期的开始和结束时都会被更新

`->qlen_last_fqs_check` `->n_force_qs_snap` 在回调列表变得过长时，协调来`call_rcu()` 及其相关函数的静止状态强制

`->n_cbs_invoked`、`->n_cbs_orphaned` `->n_cbs_adopted` 字段分别统计被调用的回调数量、此 CPU 离线时发送给其他 CPU 的回调数量，以及从其CPU 离线时接收的回调数量。`->n_nocbs_invoked` 在该 CPU 的回调被卸载kthread 时使用

最后，`->blimit` 计数器是给定时刻可被调用的最RCU 回调数量

####### Dyntick-Idle 处理


`rcu_data` 结构的这一部分是这样声明的

```

     1   int watching_snap;
     2   unsigned long dynticks_fqs;

```
`->watching_snap` 字段用于在强制静止状态时，对相应 CPU dyntick-idle 状态拍一个快照，因此会从其他 CPU 访问。最后，`->dynticks_fqs` 字段用于统计CPU 被判定为处于 dyntick-idle 状态的次数，并用于跟踪和调试目的

`rcu_data` 结构的这一部分是这样声明的

```

     1   long nesting;
     2   long nmi_nesting;
     3   atomic_t dynticks;
     4   bool rcu_need_heavy_qs;
     5   bool rcu_urgent_qs;

```
`rcu_data` 结构中的这些字段维护相应 CPU 的每 CPU dyntick-idle 状态。除非另有说明，这些字段只能从相应的 CPU（以及从跟踪代码）访问

`->nesting` 字段统计进程执行的嵌套深度，因此在正常情况下该计数器的值为 0 1。NMI、irq 和跟踪器`->nmi_nesting` 字段统计。因NMI 无法被屏蔽，对此变量的更改必须使Andy Lutomirski 提供的算法谨慎进行。从空闲状态的初始转换加一，嵌套转换加二，因此嵌套层级为五`->nmi_nesting` 值为九来表示。因此可以认为该计数器统计了除进程级转换之外，本 CPU 不容许进dyntick-idle 模式的原因数量

然而，事实证明，当运行在非空闲的内核上下文时，Linux 内核完全能够进入永不退出的中断处理程序，或许反之亦然。因此，每当 `->nesting` 字段从零递增时，`->nmi_nesting` 字段被设为一个大的正数；每当 `->nesting` 字段递减到零时，`->nmi_nesting` 字段被设为零。假设错误嵌套的中断数量不足以让计数器溢出，这种方法会在相应 CPU 每次从进程上下文进入空闲循环时，纠正 `->nmi_nesting` 字段

`->dynticks` 字段统计相应 CPU 进出 dyntick-idle 模式或用户模式的转换次数，因此当 CPU 处于 dyntick-idle 模式或用户模式时该计数器为偶数值，否则为奇数值。进出用户模式的转换需要被统计，以支持用户模式的自适应 ticks（adaptive-ticks）（Documentation/timers/no_hz.rst）

`->rcu_need_heavy_qs` 字段用于记录这样一个事实：RCU 核心代码非常希望看到来自相应 CPU 的静止状态，以至于它愿意调用重型dyntick 计数器操作。此标志RCU 的上下文切换`cond_resched()` 代码检查，它们会相应地提供一个短暂的空闲停留（sojourn）

最后，`->rcu_urgent_qs` 字段用于记录这样一个事实：RCU 核心代码非常希望看到来自相应 CPU 的静止状态，而其他各个字段则表明 RCU 对此次静止状态的渴望程度。此标志RCU 的上下文切换路径（`rcu_note_context_switch`）和 cond_resched 代码检查

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 为什么不干脆`->nesting` `->nmi_nesting` 计数器合并成单个   |
| 计数器，只统计相CPU 非空闲的原因数量                           |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 因为这会在存在永不返回的中断处理程序，以及设法从伪造中断中返回的处|
| 程序时出现失败                                                     |
+-----------------------------------------------------------------------+

某些特殊用途的构建中还存在额外的字段，将单独讨论

#### ``rcu_head`` 结构


每个 `rcu_head` 结构表示一RCU 回调。这些结构通常内嵌在使用异步宽限期的算法所涉及的、受 RCU 保护的数据结构之中。相比之下，当使用阻塞等RCU 宽限期的算法时，RCU 用户无需提供 `rcu_head` 结构

`rcu_head` 结构的字段如下：

```

     1   struct rcu_head *next;
     2   void (*func)(struct rcu_head *head);

```
`->next` 字段用于`rcu_head` 结构链接`rcu_data` 结构内部的列表中。`->func` 字段是一个指针，指向当回调准备好被调用时要调用的函数，并且该函数会被传入一个指`rcu_head` 结构的指针。不过，`kfree_rcu()` 使用 `->func` 字段来记`rcu_head` 结构在外层受 RCU 保护的数据结构中的偏移量

这两个字段都RCU 在内部使用。从 RCU 用户的角度看，这个结构是一个不透明的“cookie”

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 既然回调函数`->func` 被传入一个指`rcu_head` 结构的指针， |
| 个函数该如何找到外层RCU 保护的数据结构的起始位置               |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 在实际操作中，每种受 RCU 保护的数据结构都有一个单独的回调         |
| 数。因此该回调函数可以使用 Linux 内核中的 `container_of()` 宏（或在  |
| 其他软件环境中的其他指针操作设施）来找到外层结构的起始位置        |
+-----------------------------------------------------------------------+

#### ``task_struct`` 结构中的 RCU 特定字段


`CONFIG_PREEMPT_RCU` 实现`task_struct` 结构中使用了一些额外的字段

```

    1 #ifdef CONFIG_PREEMPT_RCU
    2   int rcu_read_lock_nesting;
    3   union rcu_special rcu_read_unlock_special;
    4   struct list_head rcu_node_entry;
    5   struct rcu_node *rcu_blocked_node;
    6 #endif /* #ifdef CONFIG_PREEMPT_RCU */
    7 #ifdef CONFIG_TASKS_RCU
    8   unsigned long rcu_tasks_nvcsw;
    9   bool rcu_tasks_holdout;
   10   struct list_head rcu_tasks_holdout_list;
   11   int rcu_tasks_idle_cpu;
   12 #endif /* #ifdef CONFIG_TASKS_RCU */

```
`->rcu_read_lock_nesting` 字段记录 RCU 读端临界区的嵌套层级，`->rcu_read_unlock_special` 字段是一个位掩码，记录需`rcu_read_unlock()` 做额外工作的特殊情况。`->rcu_node_entry` 字段用于形成在可抢占 RCU 读端临界区内阻塞的任务列表，`->rcu_blocked_node` 字段引用该任务作为其中一员的那个 `rcu_node` 结构列表，如果它并未在可抢占 RCU 读端临界区内阻塞则为 `NULL`

`->rcu_tasks_nvcsw` 字段跟踪该任务在当前 tasks-RCU 宽限期开始时已经历的自愿上下文切换次数，`->rcu_tasks_holdout` 在当tasks-RCU 宽限期正在等待此任务时设置，`->rcu_tasks_holdout_list` 是一个列表元素，将此任务入队holdout 列表中，`->rcu_tasks_idle_cpu` 跟踪此空闲任务运行在哪个 CPU 上，但仅当该任务当前正在运行，即CPU 当前空闲时

#### 访问器函


下面的列表展示了 `rcu_get_root()`、`rcu_for_each_node_breadth_first` `rcu_for_each_leaf_node()` 函数与宏

```

     1 static struct rcu_node *rcu_get_root(struct rcu_state *rsp)
     2 {
     3   return &rsp->node[0];
     4 }
     5
     6 #define rcu_for_each_node_breadth_first(rsp, rnp) \
     7   for ((rnp) = &(rsp)->node[0]; \
     8        (rnp) < &(rsp)->node[NUM_RCU_NODES]; (rnp)++)
     9
    10 #define rcu_for_each_leaf_node(rsp, rnp) \
    11   for ((rnp) = (rsp)->level[NUM_RCU_LVLS - 1]; \
    12        (rnp) < &(rsp)->node[NUM_RCU_NODES]; (rnp)++)

```
`rcu_get_root()` 简单地返回指定 `rcu_state` 结构`->node[]` 数组的第一个元素的指针，也就是`rcu_node` 结构

如前所述，`rcu_for_each_node_breadth_first()` 宏利用了 `rcu_node` 结构`rcu_state` 结构`->node[]` 数组中的布局，通过简单地按序遍历数组来执行广度优先遍历。类似地，`rcu_for_each_leaf_node()` 宏只遍历数组的最后一部分，从而只遍历`rcu_node` 结构

+-----------------------------------------------------------------------+
| **快速测验（Quick Quiz*                                         |
+-----------------------------------------------------------------------+
| 如果 `rcu_node` 树只包含一个节点，`rcu_for_each_leaf_node()` 会做  |
| 什么？                                                                 |
+-----------------------------------------------------------------------+
| **答案**                                                           |
+-----------------------------------------------------------------------+
| 在单节点情况下，`rcu_for_each_leaf_node()` 遍历那个单一节点       |
+-----------------------------------------------------------------------+

#### 总结


因此，RCU 的状态由一`rcu_state` 结构表示，它包含一个由 `rcu_node` `rcu_data` 结构组成的组合树。最后，`CONFIG_NO_HZ_IDLE` 内核中，每个 CPU dyntick-idle 状态由 `rcu_data` 结构中的 dynticks 相关字段跟踪。如果你读到了这里，你已为阅读本系列其他文章中的代码走查做好了充分准备

#### 致谢


我要感谢 Cyrill Gorcunov、Mathieu Desnoyers、Dhaval Giani、Paul Turner、Abhishek Srivastava、Matt Kowalczyk Serge Hallyn，他们帮助我将本文档整理成更具可读性的状态

#### 法律声明


本作品代表作者的观点，并不一定代IBM 的观点

Linux Linus Torvalds 的注册商标

其他公司、产品和服务的名称可能是其他方的商标或服务标志
