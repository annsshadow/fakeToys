## Linux 日志（Journalling）API


### 概述


#### 细节


日志层使用起来很简单。首先，你需要创建一个 journal_t 数据结构。根据
你决定在何处分配日志所在物理介质的不同，有两种调用来完成这件事。
jbd2_journal_init_inode() 用于存储在文件系统 inode 中的日志；而
jbd2_journal_init_dev() 可用于存储在裸设备（一段连续的块区间）上的日志。
journal_t 是一个指向结构体的指针的 typedef，所以当你最终结束时，务必
对其调用 jbd2_journal_destroy() 以释放任何已使用的内核内存。

一旦你拥有了 journal_t 对象，就需要“挂载”或加载日志文件。日志层期望
日志所需的空间已经由用户态工具分配并正确初始化。在加载日志时，你必须
调用 jbd2_journal_load() 来处理日志内容。如果客户端文件系统检测到日志
内容无需被处理（甚至不需要有有效内容），它可以调用
jbd2_journal_wipe() 在调用 jbd2_journal_load() 之前清空日志内容。

注意，jbd2_journal_wipe(..,0) 在检测到日志中存在任何未决事务时会为你
调用 jbd2_journal_skip_recovery()；类似地，jbd2_journal_load() 会在
必要时调用 jbd2_journal_recover()。我建议阅读 fs/ext4/super.c 中的
ext4_load_journal() 作为此阶段的示例。

现在你可以着手开始修改底层文件系统了。差不多了。

你仍然需要真正地把你对文件系统的修改记入日志，这是通过将它们包裹进
事务（transaction）中完成的。此外，你还需要把对每个缓冲区的修改也用
对日志层的调用来包裹，这样它才知道你实际做出的修改是什么。为此要使用
jbd2_journal_start()，它会返回一个事务句柄。

jbd2_journal_start() 及其对应物 jbd2_journal_stop()（表示事务结束）是
可嵌套的调用，所以如有必要你可以重新进入一个事务；但请记住，在事务
完成（更准确地说，离开更新阶段）之前，你必须调用 jbd2_journal_stop()
的次数与调用 jbd2_journal_start() 的次数相同。Ext4/VFS 利用了这个特性
来简化 inode 置脏、配额支持等的处理。

在每个事务内部，你需要把对各个缓冲区（块）的修改包裹起来。在开始修改
一个缓冲区之前，你需要视情况调用 jbd2_journal_get_create_access() /
jbd2_journal_get_write_access() /
jbd2_journal_get_undo_access()，这让日志层能够在需要时复制未被修改的
数据——毕竟该缓冲区可能属于某个先前尚未提交的事务。此时你终于可以
修改缓冲区了，一旦完成修改，你需要调用
jbd2_journal_dirty_metadata()。或者，如果你请求访问的某个缓冲区现在
已知不再需要写回设备，你可以调用 jbd2_journal_forget()，其用法与你
过去可能用过的 bforget() 大致相同。

可以随时调用 jbd2_journal_flush() 来提交并对所有事务执行检查点。

然后在卸载时，在你的 put_super() 中调用 jbd2_journal_destroy() 来
清理你在内存中的日志对象。

遗憾的是，日志层有几种方式会导致死锁。首先要注意到的是，每个任务在
任一时刻只能有一个未完成的事务；请记住，在 outermost 的
jbd2_journal_stop() 之前没有任何东西会被提交。这意味着你必须在所执行的
每个文件/inode/address 等操作的末尾完成该事务，这样日志系统才不会被
重新进入另一个日志。由于事务不能跨不同的日志嵌套/批处理，而另一个
文件系统（比如 ext4）可能在后续的系统调用中被修改。

需要牢记的第二种情况是，如果没有足够的日志空间容纳你的事务（基于传入
的 nblocks 参数），jbd2_journal_start() 会阻塞——当它阻塞时，它只是
（！）需要等待来自其他任务的事务完成并被提交，所以本质上我们是在等待
jbd2_journal_stop()。因此，为避免死锁，你必须把 jbd2_journal_start() /
jbd2_journal_stop() 当作信号量一样对待，并将它们纳入你的信号量排序规则
以防止死锁。注意 jbd2_journal_extend() 与 jbd2_journal_start() 有
相似的阻塞行为，所以你在这里同样容易像在 jbd2_journal_start() 上一样
发生死锁。

一开始就预留正确数量的块。;-)。这将是你在此事务中要触碰的最大块数。
我建议至少看一下 ext4_jbd.h，了解 ext4 据以做出这些决策的基础。

另一个需要注意的问题是你的磁盘块分配策略。为什么？因为，如果你执行
删除操作，你需要确保在释放这些块的事务提交之前，没有重用其中任何
被释放的块。如果你重用了这些块并发生了崩溃，就无法在上一个完全提交的
事务末尾恢复被重新分配块的内容。一种简单的做法是，仅在释放它们的
事务提交之后，才在内部内存块分配结构中将这些块标记为空闲。Ext4 为此
使用了日志提交回调。

借助日志提交回调，你可以要求日志层在事务最终提交到磁盘时调用一个回调
函数，这样你就能做一些自己的管理工作。你只需设置
`journal->j_commit_callback` 函数指针，即可让日志层调用该回调，该函数在
每次事务提交后被调用。

JBD2 还提供了一种通过 jbd2_journal_lock_updates() /
jbd2_journal_unlock_updates() 阻塞所有事务更新的方式。Ext4 在需要一段
干净且稳定的文件系统时，会用到它。例如：

```

        jbd2_journal_lock_updates() //stop new stuff happening..
        jbd2_journal_flush()        // checkpoint everything.
        ..do stuff on stable fs
        jbd2_journal_unlock_updates() // carry on with filesystem use.

```
如果你允许非特权用户态触发包含这些调用的代码路径，那么其中可被滥用于
DoS 攻击的机会应当是不言而喻的。

#### 快速提交（Fast commits）


JBD2 还允许你执行被称为快速提交的文件系统特定的增量提交。为了使用快速
提交，你需要设置以下执行相应工作的回调：

`journal->j_fc_cleanup_cb`：在每次完整提交和快速提交之后调用的清理函数。

`journal->j_fc_replay_cb`：用于重放快速提交块的回调重放函数。

文件系统可以在它想要的时候自由地执行快速提交，只要它通过调用
`jbd2_fc_begin_commit()` 函数获得 JBD2 的许可即可。一旦快速提交完成，
客户端文件系统应通过调用 `jbd2_fc_end_commit()` 告知 JBD2。如果文件系统
希望 JBD2 在停止快速提交后立即执行一次完整提交，它可以通过调用
`jbd2_fc_end_commit_fallback()` 来做到。当快速提交操作由于某种原因失败，
而保证一致性的唯一办法是让 JBD2 执行传统完整提交时，这很有用。

JBD2 提供了管理快速提交缓冲区的辅助函数。文件系统可以使用
`jbd2_fc_get_buf()` 和 `jbd2_fc_wait_bufs()` 来分配快速提交缓冲区并
等待其 IO 完成。

目前，仅有 Ext4 实现了快速提交。有关其快速提交实现的细节，请参阅
fs/ext4/fast_commit.c 顶部的注释。

#### 总结


使用日志的关键在于把不同的上下文变化包裹起来：每次挂载、每次修改
（事务）以及每个被修改的缓冲区，都要告知日志层。

### 数据类型


日志层使用 typedef 来“隐藏”所使用结构的具体定义。作为 JBD2 层的客户端，
你可以只依赖把该指针当作某种魔法 cookie 来使用。显然，这种隐藏并未被
强制实施，因为这是“C”。

#### 结构


   :internal:

### 函数


这里的函数被分为两组：一组影响整个日志，另一组用于管理事务。

#### 日志级别（Journal Level）


   :export:

   :internal:

#### 事务级别（Transaction Level）



### 另请参阅


`Journaling the Linux ext2fs Filesystem, LinuxExpo 98, Stephen
Tweedie <http://kernel.org/pub/linux/kernel/people/sct/ext3/journal-design.ps.gz>`__

`Ext3 Journalling FileSystem, OLS 2000, Dr. Stephen
Tweedie <http://olstrans.sourceforge.net/release/OLS2000-ext3/OLS2000-ext3.html>`__
