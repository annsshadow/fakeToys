## vlocks 用于裸机互斥


投票锁（Voting Locks，或“vlocks”）提供了一种简单的底层互斥机制，对内存系统的要求合理且最小。

这些锁旨在用于协调那些本身不具一致性的 CPU 之间的关键活动，适用于硬件不提供其他支持机制、且无法使用普通自旋锁的情况。


vlocks 利用了内存系统对写入单一内存位置的原子性。为了仲裁，每个 CPU 通过向一个公共内存位置存储一个唯一编号来“为自己投票”。当所有投票都完成之后，该内存位置中所看到的最终值确定了胜者。

为了确保选举能在有限时间内产生明确的结果，只有当尚未选出胜者且选举看起来尚未开始时，CPU 才会首先进入选举。


### Algorithm


```


	int currently_voting[NR_CPUS] = { 0, };
	int last_vote = -1; /* no votes yet */

	bool vlock_trylock(int this_cpu)
	{
		/* signal our desire to vote */
		currently_voting[this_cpu] = 1;
		if (last_vote != -1) {
			/* someone already volunteered himself */
			currently_voting[this_cpu] = 0;
			return false; /* not ourself */
		}

		/* let's suggest ourself */
		last_vote = this_cpu;
		currently_voting[this_cpu] = 0;

		/* then wait until everyone else is done voting */
		for_each_cpu(i) {
			while (currently_voting[i] != 0)
				/* wait */;
		}

		/* result */
		if (last_vote == this_cpu)
			return true; /* we won */
		return false;
	}

	bool vlock_unlock(void)
	{
		last_vote = -1;
	}


```
currently_voting[] 数组为各 CPU 提供了一种判断选举是否正在进行的方式，其作用类似于 Lamport 面包店算法 [^1^] 中的“entering”数组。

然而，一旦选举开始，底层的、由内存系统提供的原子性就被用来挑选胜者。这避免了需要一个静态优先级规则作为决胜机制，也避免了任何可能溢出的计数器。

只要 last_vote 变量对所有 CPU 全局可见，它就只会包含一个值，并且在每个 CPU 都清除了其 currently_voting 标志之前不会改变。

```

	/* first level: local election */
	my_town = towns[(this_cpu >> 4) & 0xf];
	I_won = vlock_trylock(my_town, this_cpu & 0xf);
	if (I_won) {
		/* we won the town election, let's go for the state */
		my_state = states[(this_cpu >> 8) & 0xf];
		I_won = vlock_lock(my_state, this_cpu & 0xf));
		if (I_won) {
			/* and so on */
			I_won = vlock_lock(the_whole_country, this_cpu & 0xf];
			if (I_won) {
				/* ... */
			}
			vlock_unlock(the_whole_country);
		}
		vlock_unlock(my_state);
	}
	vlock_unlock(my_town);


```
### ARM 实现


当前的 ARM 实现 [^2^] 在基础算法之外还包含一些优化：

 - 通过将 currently_voting 数组的成员紧凑地排在一起，我们可以在一次事务中读取整个数组（前提是可能竞争该锁的 CPU 数量足够小）。这减少了访问外部内存所需的往返次数。

   在 ARM 实现中，这意味着我们可以使用一次加载

```

	LDR	Rt, [Rn]
	CMP	Rt, #0

   ...in place of code equivalent to::

	LDRB	Rt, [Rn]
	CMP	Rt, #0
	LDRBEQ	Rt, [Rn, #1]
	CMPEQ	Rt, #0
	LDRBEQ	Rt, [Rn, #2]
	CMPEQ	Rt, #0
	LDRBEQ	Rt, [Rn, #3]
	CMPEQ	Rt, #0

   This cuts down on the fast-path latency, as well as potentially
   reducing bus contention in contended cases.

   The optimisation relies on the fact that the ARM memory system
   guarantees coherency between overlapping memory accesses of
   different sizes, similarly to many other architectures.  Note that
   we do not care which element of currently_voting appears in which
   bits of Rt, so there is no need to worry about endianness in this
   optimisation.

   If there are too many CPUs to read the currently_voting array in
   one transaction then multiple transactions are still required.  The
   implementation uses a simple loop of word-sized loads for this
   case.  The number of transactions is still fewer than would be
   required if bytes were loaded individually.


   In principle, we could aggregate further by using LDRD or LDM, but
   to keep the code simple this was not attempted in the initial
   implementation.


 * vlocks are currently only used to coordinate between CPUs which are
   unable to enable their caches yet.  This means that the
   implementation removes many of the barriers which would be required
   when executing the algorithm in cached memory.

   packing of the currently_voting array does not work with cached
   memory unless all CPUs contending the lock are cache-coherent, due
   to cache writebacks from one CPU clobbering values written by other
   CPUs.  (Though if all the CPUs are cache-coherent, you should be
   probably be using proper spinlocks instead anyway).


 * The "no votes yet" value used for the last_vote variable is 0 (not
   -1 as in the pseudocode).  This allows statically-allocated vlocks
   to be implicitly initialised to an unlocked state simply by putting
   them in .bss.

   An offset is added to each CPU's ID for the purpose of setting this
   variable, so that no CPU uses the value 0 for its ID.


```
### Colophon


最初由 Dave Martin 为 Linaro Limited 创建并记录，用于基于 ARM 的 big.LITTLE 平台，并感激地接受了来自 Nicolas Pitre 与 Achin Gupta 的审阅与意见。感谢 Nicolas 从相关邮件讨论串中提取了大部分文本并编写了伪代码。

Copyright (C) 2012-2013  Linaro Limited
依据 linux/COPYING 中定义的 GNU General Public License 第 2 版的条款分发。


### References


[^1^] Lamport, L. "A New Solution of Dijkstra's Concurrent Programming
    Problem", Communications of the ACM 17, 8 (August 1974), 453-455.

    https://en.wikipedia.org/wiki/Lamport%27s_bakery_algorithm

[^2^] linux/arch/arm/common/vlock.S, www.kernel.org.
