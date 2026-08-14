## 内核锁 torture 测试操作

## CONFIG_LOCK_TORTURE_TEST

CONFIG_LOCK_TORTURE_TEST 配置选项提供了一个内核模块，它会对核心内核的锁原语运行 torture 测试。如果需要，可以在被测的正在运行的内核上事后构建名为 'locktorture' 的内核模块。测试会周期性地通过 printk() 输出状态消息，可以通过 dmesg（也许用 grep "torture"）查看。测试在模块加载时启动，在模块卸载时停止。本程序基于 RCU 如何被 torture 的方式，即通过 rcutorture。

这个 torture 测试通过创建若干内核线程来模拟不同的临界区行为，这些线程获取锁并将其持有特定的一段时间。锁上的争用程度可以通过延长这个临界区的持有时间和/或创建更多的 kthread 来模拟。

## 模块参数

本模块具有以下参数：

### Locktorture 专用

nwriters_stress
		  用于对独占锁所有权（写者）施加压力的核线程数量。默认值是在线 CPU 数量的两倍。

nreaders_stress
		  用于对共享锁所有权（读者）施加压力的核线程数量。默认与写者锁数量相同。如果用户未指定 nwriters_stress，那么读者和写者都为在线 CPU 的数量。

torture_type
		  要 torture 的锁类型。默认只 torture 自旋锁。本模块可以用如下字符串值 torture 以下锁：

       - "lock_busted":
				模拟一个有缺陷的锁实现。

       - "spin_lock":
				spin_lock() 与 spin_unlock() 对。

       - "spin_lock_irq":
				spin_lock_irq() 与 spin_unlock_irq() 对。

       - "rw_lock":
				read/write lock() 与 unlock() rwlock 对。

       - "rw_lock_irq":
				read/write lock_irq() 与 unlock_irq()
				rwlock 对。

       - "mutex_lock":
				mutex_lock() 与 mutex_unlock() 对。

       - "rtmutex_lock":
				rtmutex_lock() 与 rtmutex_unlock() 对。
				内核必须配置 CONFIG_RT_MUTEXES=y。

       - "rwsem_lock":
				read/write down() 与 up() 信号量对。

### Torture 框架（RCU + 锁）

shutdown_secs
		  在终止测试并关闭系统之前运行测试的秒数。默认为零，即禁用测试终止与系统关机。此能力对自动化测试很有用。

onoff_interval
		  每次尝试执行随机选择的 CPU 热插拔操作之间的秒数。默认为零，即禁用 CPU 热插拔。在 CONFIG_HOTPLUG_CPU=n 的内核中，无论为 onoff_interval 指定什么值，locktorture 都会静默地拒绝执行任何 CPU 热插拔操作。

onoff_holdoff
		  在开始 CPU 热插拔操作之前等待的秒数。这通常只在内核内置了 locktorture 并在启动时自动启动时才有用，此时它有助于避免让启动时代码被来来去去的 CPU 搞糊涂。此参数仅在启用了 CONFIG_HOTPLUG_CPU 时才有用。

stat_interval
		  统计相关 printk() 之间的秒数。默认情况下，locktorture 每 60 秒报告一次统计。把间隔设为零会导致统计只在该模块卸载时才被打印。

stutter
		  运行测试然后在相同长度时间内暂停的时长。默认为 "stutter=5"，即大约以五秒的间隔运行和暂停。指定 "stutter=0" 会使测试持续运行而不暂停。

shuffle_interval
		  将测试线程亲和到特定 CPU 子集保持的秒数，默认为 3 秒。与 test_no_idle_hz 配合使用。

verbose
		  通过 printk() 启用详细调试打印。默认启用。这些额外信息大多与来自主 'torture' 框架的高层错误和报告有关。

## 统计

```

  spin_lock-torture: Writes:  Total: 93746064  Max/Min: 0/0   Fail: 0
     (A)		    (B)		   (C)		  (D)	       (E)

  (A): Lock type that is being tortured -- torture_type parameter.

  (B): Number of writer lock acquisitions. If dealing with a read/write
       primitive a second "Reads" statistics line is printed.

  (C): Number of times the lock was acquired.

  (D): Min and max number of times threads failed to acquire the lock.

  (E): true/false values if there were errors acquiring the lock. This should
       -only- be positive if there is a bug in the locking primitive's
       implementation. Otherwise a lock should never fail (i.e., spin_lock()).
       Of course, the same applies for (C), above. A dummy example of this is
       the "lock_busted" type.

```
## 用法

```

	#!/bin/sh

	modprobe locktorture
	sleep 3600
	rmmod locktorture
	dmesg | grep torture:

```
输出可以手动检查 "!!!" 的错误标志。当然，也可以创建一个更精巧的脚本来自动检查此类错误。 "rmmod" 命令会强制 printk() 打印一个 "SUCCESS"、"FAILURE" 或 "RCU_HOTPLUG" 指示。前两个不言自明，而最后一个表示虽然没有锁定失败，但检测到了 CPU 热插拔问题。

另见：Documentation/RCU/torture.rst
