## struct taskstats 缁撴瀯浣?


鏈枃妗ｈВ閲婁簡 struct taskstats 鐨勫悇瀛楁銆?

struct taskstats 涓殑瀛楁鍒嗕负涓夌粍涓嶅悓鐨勭被鍒細

1) 閫氱敤涓庡熀纭€缁熻瀛楁
    鑻ヨ缃簡 CONFIG_TASKSTATS锛屽垯 taskstats 鎺ュ彛琚惎鐢紝骞朵笖閫氱敤瀛楁鍜屽熀纭€缁熻瀛楁浼氬湪浠诲姟鐨?do_exit() 鏃惰鏀堕泦骞朵紶閫掋€?
2) 寤惰繜缁熻瀛楁
```

	/* Delay accounting fields start */

    and::

	/* Delay accounting fields end */

    Their values are collected if CONFIG_TASK_DELAY_ACCT is set.
```
3) 鎵╁睍缁熻瀛楁
```

	/* Extended accounting fields start */

    and::

	/* Extended accounting fields end */

    Their values are collected if CONFIG_TASK_XACCT is set.

```
4) 姣忎换鍔′笌姣忕嚎绋嬬殑涓婁笅鏂囧垏鎹㈡鏁扮粺璁?

5) SMT 鏈哄櫒鐨勮鏃剁粺璁?

6) 鐢ㄤ簬鍐呭瓨鍥炴敹鐨勬墿灞曞欢杩熺粺璁″瓧娈?

鏈潵鐨勬墿灞曞簲褰撴妸瀛楁娣诲姞鍒?taskstats 缁撴瀯浣撶殑鏈熬锛屽苟涓斾笉搴旀敼鍙樼粨鏋勪綋鍐呮瘡涓瓧娈电殑鐩稿浣嶇疆銆?

```

  struct taskstats {

```
```

	/* The version number of this struct. This field is always set to
	 * TASKSTATS_VERSION, which is defined in <linux/taskstats.h>.
	 * Each time the struct is changed, the value should be incremented.
	 */
	__u16	version;

	/* The exit code of a task. */
	__u32	ac_exitcode;		/* Exit status */

	/* The accounting flags of a task as defined in <linux/acct.h>
	 * Defined values are AFORK, ASU, ACOMPAT, ACORE, and AXSIG.
	 */
	__u8	ac_flag;		/* Record flags */

	/* The value of task_nice() of a task. */
	__u8	ac_nice;		/* task_nice */

	/* The name of the command that started this task. */
	char	ac_comm[TS_COMM_LEN];	/* Command name */

	/* The scheduling discipline as set in task->policy field. */
	__u8	ac_sched;		/* Scheduling discipline */

	__u8	ac_pad[3];
	__u32	ac_uid;			/* User ID */
	__u32	ac_gid;			/* Group ID */
	__u32	ac_pid;			/* Process ID */
	__u32	ac_ppid;		/* Parent process ID */

	/* The time when a task begins, in [secs] since 1970. */
	__u32	ac_btime;		/* Begin time [sec since 1970] */

	/* The elapsed time of a task, in [usec]. */
	__u64	ac_etime;		/* Elapsed time [usec] */

	/* The user CPU time of a task, in [usec]. */
	__u64	ac_utime;		/* User CPU time [usec] */

	/* The system CPU time of a task, in [usec]. */
	__u64	ac_stime;		/* System CPU time [usec] */

	/* The minor page fault count of a task, as set in task->min_flt. */
	__u64	ac_minflt;		/* Minor Page Fault Count */

	/* The major page fault count of a task, as set in task->maj_flt. */
	__u64	ac_majflt;		/* Major Page Fault Count */


```
```

	/* Delay accounting fields start
	 *
	 * All values, until the comment "Delay accounting fields end" are
	 * available only if delay accounting is enabled, even though the last
	 * few fields are not delays
	 *
	 * xxx_count is the number of delay values recorded
	 * xxx_delay_total is the corresponding cumulative delay in nanoseconds
	 *
	 * xxx_delay_total wraps around to zero on overflow
	 * xxx_count incremented regardless of overflow
	 */

	/* Delay waiting for cpu, while runnable
	 * count, delay_total NOT updated atomically
	 */
	__u64	cpu_count;
	__u64	cpu_delay_total;

	/* Following four fields atomically updated using task->delays->lock */

	/* Delay waiting for synchronous block I/O to complete
	 * does not account for delays in I/O submission
	 */
	__u64	blkio_count;
	__u64	blkio_delay_total;

	/* Delay waiting for page fault I/O (swap in only) */
	__u64	swapin_count;
	__u64	swapin_delay_total;

	/* cpu "wall-clock" running time
	 * On some architectures, value will adjust for cpu time stolen
	 * from the kernel in involuntary waits due to virtualization.
	 * Value is cumulative, in nanoseconds, without a corresponding count
	 * and wraps around to zero silently on overflow
	 */
	__u64	cpu_run_real_total;

	/* cpu "virtual" running time
	 * Uses time intervals seen by the kernel i.e. no adjustment
	 * for kernel's involuntary waits due to virtualization.
	 * Value is cumulative, in nanoseconds, without a corresponding count
	 * and wraps around to zero silently on overflow
	 */
	__u64	cpu_run_virtual_total;
	/* Delay accounting fields end */
	/* version 1 ends here */


```
```

	/* Extended accounting fields start */

	/* Accumulated RSS usage in duration of a task, in MBytes-usecs.
	 * The current rss usage is added to this counter every time
	 * a tick is charged to a task's system time. So, at the end we
	 * will have memory usage multiplied by system time. Thus an
	 * average usage per system time unit can be calculated.
	 */
	__u64	coremem;		/* accumulated RSS usage in MB-usec */

	/* Accumulated virtual memory usage in duration of a task.
	 * Same as acct_rss_mem1 above except that we keep track of VM usage.
	 */
	__u64	virtmem;		/* accumulated VM usage in MB-usec */

	/* High watermark of RSS usage in duration of a task, in KBytes. */
	__u64	hiwater_rss;		/* High-watermark of RSS usage */

	/* High watermark of VM  usage in duration of a task, in KBytes. */
	__u64	hiwater_vm;		/* High-water virtual memory usage */

	/* The following four fields are I/O statistics of a task. */
	__u64	read_char;		/* bytes read */
	__u64	write_char;		/* bytes written */
	__u64	read_syscalls;		/* read syscalls */
	__u64	write_syscalls;		/* write syscalls */

	/* Extended accounting fields end */

```
```

	__u64	nvcsw;			/* Context voluntary switch counter */
	__u64	nivcsw;			/* Context involuntary switch counter */

```
```

	__u64	ac_utimescaled;		/* utime scaled on frequency etc */
	__u64	ac_stimescaled;		/* stime scaled on frequency etc */
	__u64	cpu_scaled_run_real_total; /* scaled cpu_run_real_total */

```
```

	/* Delay waiting for memory reclaim */
	__u64	freepages_count;
	__u64	freepages_delay_total;

```
```

  }

```