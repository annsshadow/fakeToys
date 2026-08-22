
## 顺序计数器与顺序

## 简

顺序计数器是一种读写一致性机制，具有无锁的读者（只读重试循环），且不会出写者饥饿。它们用于很少被写入的数据（例如系统时间），此时读者希望获得一一致的信息，并愿意在该信息发生变化时重试
当读侧临界区开始时的顺序计数为偶数、且临界区结束时读到的顺序计数值相同时数据集是一致的。集合中的数据必须在读侧临界区内被复制出来。如果顺序计数在
临界区开始和结束之间发生了变化，读者必须重试
写者在写侧临界区的开始和结束处递增顺序计数。进入临界区后顺序计数为奇数向读者表明有更新正在进行。在写侧临界区结束时顺序计数再次变为偶数，使读得以继续推进
顺序计数器的写侧临界区绝不能被读侧临界区抢占或打断。否则，由于奇数的顺计数值和被打断的写者，读者会在整个调度节拍内自旋。如果该读者属于实时调类，它可能永远自旋，内核将发生活锁
如果受保护的数据包含指针，则不能使用此机制，因为写者可能使读者正在跟随的
指针失效


## 顺序计数器（``seqcount_t``

这是原始的计数机制，不防止多个写者。因此写侧临界区必须由外部锁进行串行化
如果写串行化原语没有隐式地禁用抢占，则必须在进入写侧临界区之前显式禁抢占。如果读侧临界区可以hardirq softirq 上下文调用，则在进入写侧临界之前还必须分别禁用中断或底半部
如果希望自动处理写者串行化和不可抢占性的顺序计数器要求，请改seqlock_t
```

	/* dynamic */
	seqcount_t foo_seqcount;
	seqcount_init(&foo_seqcount);

	/* static */
	static seqcount_t foo_seqcount = SEQCNT_ZERO(foo_seqcount);

	/* C99 struct init */
	struct {
		.seq   = SEQCNT_ZERO(foo.seq),
	} foo;

```
```

	/* Serialized context with disabled preemption */

	write_seqcount_begin(&foo_seqcount);

	/* ... [[write-side critical section]] ... */

	write_seqcount_end(&foo_seqcount);

```
```

	do {
		seq = read_seqcount_begin(&foo_seqcount);

		/* ... [[read-side critical section]] ... */

	} while (read_seqcount_retry(&foo_seqcount, seq));


```

### 带关联锁的顺序计数器（``seqcount_LOCKNAME_t``

正如seqcount_t 中所述，顺序计数的写侧临界区必须被串行化且不可抢占。这顺序计数器的变体在初始化时将用于写者串行化的锁关联起来，从而使 lockdep 能够
验证写侧临界区是否被正确串行化
如果禁用 lockdep，此锁关联是一个空操作，既没有存储开销也没有运行时开销如果启用 lockdep，锁指针被存储在 struct seqcount 中，并在写侧临界区开始时
注入 lockdep 锁已被持断言，以验证其受到正确保护
对于不会隐式禁用抢占的锁类型，写侧函数中会强制实施抢占保护
定义了以下带关联锁的顺序计数器：

  - `seqcount_spinlock_t`
  - `seqcount_raw_spinlock_t`
  - `seqcount_rwlock_t`
  - `seqcount_mutex_t`
  - `seqcount_ww_mutex_t`

顺序计数的读API 既可以接受普通的 seqcount_t，也可以接受上述任意
seqcount_LOCKNAME_t 变体
```

	/* dynamic */
	seqcount_LOCKNAME_t foo_seqcount;
	seqcount_LOCKNAME_init(&foo_seqcount, &lock);

	/* static */
	static seqcount_LOCKNAME_t foo_seqcount =
		SEQCNT_LOCKNAME_ZERO(foo_seqcount, &lock);

	/* C99 struct init */
	struct {
		.seq   = SEQCNT_LOCKNAME_ZERO(foo.seq, &lock),
	} foo;

```
写路径：seqcount_t 相同，但运行在已获取关联写串行化锁的上下文中
读路径：seqcount_t 相同


### 锁存顺序计数器（``seqcount_latch_t``

锁存顺序计数器是一种多版本并发控制机制，其中嵌入的 seqcount_t 计数偶数/奇数值用于在受保护数据的两份副本之间切换。这使得顺序计数器的读路能够安全地打断其自身的写侧临界区
当写侧临界区无法被读者打断保护时使用 seqcount_latch_t。当读侧可以NMI
处理程序调用时通常就是这种情况
请参`write_seqcount_latch()` 了解更多信息


## 顺序锁（``seqlock_t``

这包含前面讨论的 seqcount_t 机制，外加一个用于写者串行化和不可抢占性的
嵌入式自旋锁
如果读侧临界区可以从 hardirq softirq 上下文调用，请使用分别禁用中断或
底半部的写侧函数变体
```

	/* dynamic */
	seqlock_t foo_seqlock;
	seqlock_init(&foo_seqlock);

	/* static */
	static DEFINE_SEQLOCK(foo_seqlock);

	/* C99 struct init */
	struct {
		.seql   = __SEQLOCK_UNLOCKED(foo.seql)
	} foo;

```
```

	write_seqlock(&foo_seqlock);

	/* ... [[write-side critical section]] ... */

	write_sequnlock(&foo_seqlock);

```
读路径，三类
1. 普通顺序读者，从不阻塞写者，但如果检测到顺序变化、有写者正在进行，则必   重试
```

	do {
		seq = read_seqbegin(&foo_seqlock);

		/* ... [[read-side critical section]] ... */

	} while (read_seqretry(&foo_seqlock, seq));

```
2. 锁定读者，如果写者或另一个锁定读者正在进行，则会等待。进行中的锁定读   也会阻止写者进入其临界区。这个读锁为
```

	read_seqlock_excl(&foo_seqlock);

	/* ... [[read-side critical section]] ... */

	read_sequnlock_excl(&foo_seqlock);

```
3. 条件无锁读者（1）或锁定读者（2），取决于传入的标记。这用于避免
   无锁读者在写活动急剧飙升时出现饥饿（过多重试循环）。首先尝试无锁读
   （传入偶数标记）。如果该尝试失败（顺序计数器不匹配），则将标记变为奇   用于下一次迭代，无锁读被转换```

	/* marker; even initialization */
	int seq = 1;
	do {
		seq++; /* 1 无锁路径2，否则为奇数 */
		read_seqbegin_or_lock(&foo_seqlock, &seq);

		/* ... [[read-side critical section]] ... */

	} while (need_seqretry(&foo_seqlock, seq));
	done_seqretry(&foo_seqlock, seq);


```
## API 文档

