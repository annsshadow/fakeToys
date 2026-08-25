## 实时应用程序监视

- Name: rtapp
- Type: 多个监视器的容器
- Author: Nam Cao <namcao@linutronix.de>

### 描述


实时应用程序可能存在设计缺陷，导致它们经历意外的延迟并无法满足其时间要求。通常，这些缺陷遵循几种模式：

  - 页错误（Page faults）：实时线程可能访问没有映射的物理后备、或必须首先被复制（例如写时复制）的内存。因此会引发页错误，内核必须先执行代价高昂的操作。这会导致实时线程出现显著延  - 优先级反转（Priority inversion）：实时线程阻塞等待一个较低优先级的线程。这导致实时线程实际上采用了较低优先级线程的调度优先级。例如，实时线程需要访问一个由pi-mutex 保护的共享资源，但该互斥体当前由非实时线程持有
`rtapp` 监视器检测这些模式。它帮助开发人员识别实时应用程序出现意外延迟的原因。它是以下各节中描述的多个子监视器的容器
Monitor pagefault
+++++++++++++++++

`pagefault` 监视器报告引发页错误的实时任务。其
```

  RULE = always (RT imply not PAGEFAULT)

```
要修复此监视器报告的警告，可以使`mlockall()` `mlock()` 来确保内存的物理后备
此监视器可能存在漏报（false negatives），因为实时线程使用的页在测试期间可能恰好直接可用。为了尽量减少这种情况，可以让系统处于内存压力下（例如使用一个执`ptr = malloc(SIZE_OF_RAM); memset(ptr, 0, SIZE_OF_RAM);` 的程序来调用 OOM killer），以便内核执行激进策略以回收尽可能多的物理内存
Monitor sleep
+++++++++++++

`sleep` 监视器报告以可能导致不良延迟的方式睡眠的实时线程。实时应用程序只应出于以下原因之一让实时线程睡眠：

  - 周期性工作：实时线程睡眠等待下一个周期。对于这种情况，只应使用`TIMER_ABSTIME`（以避免时间漂移）和 `CLOCK_MONOTONIC`（以避免时钟被更改）`clock_nanosleep` 系统调用。其他方法对实时都不安全。例如，等待 timerfd 的线程可能被 softirq 唤醒，softirq 不提供任何实时保证  - 实时线程等待某件事情发生（例如另一个线程释放共享资源，或来自另一个线程的完成信号）。在这种情况下，只应使用 futexes（FUTEX_LOCK_PI、FUTEX_LOCK_PI2 FUTEX_WAIT_* 之一）。应用程序通常不直接使futexes，而是使用构建futexes 之上PI 互斥体和 PI 条件变量。请注意，C 库可能没有将条件变量实现为对实时安全。作为替代，librtpi 库提供了一个对 Linux 实时应用程序正确的条件变量实现
除了睡眠的原因外，最终的唤醒者也应该对实时安全。即，以下之一
  - 相等或更高优先级的线  - 硬中断处理程  - 不可屏蔽中断处理程序

此监视器的警告通常意味着以下之一
  - 实时线程被非实时线程阻塞（例如由于对不带优先级继承的互斥体的争用）。这就是优先级反转  - 时间关键的工作在等待某个对实时不安全的东西（例如 timerfd）  - 实时线程执行的工作根本不需要以实时优先级运行。这对实时线程本身不是问题，但它可能会把 CPU 从其他重要的实时工作中抢走
应用程序开发人员可能会有意选择以对实时不安全的方式让其实时应用程序睡眠。这是否是一个问题尚有争议。应用程序开发人员必须分析警告以做出恰当评估
```

  RULE = always ((RT and SLEEP) imply (RT_FRIENDLY_SLEEP or ALLOWLIST))

  RT_FRIENDLY_SLEEP = (RT_VALID_SLEEP_REASON or KERNEL_THREAD)
                  and ((not WAKE) until RT_FRIENDLY_WAKE)

  RT_VALID_SLEEP_REASON = FUTEX_WAIT
                       or RT_FRIENDLY_NANOSLEEP

  RT_FRIENDLY_NANOSLEEP = CLOCK_NANOSLEEP
                      and NANOSLEEP_TIMER_ABSTIME
                      and NANOSLEEP_CLOCK_MONOTONIC

  RT_FRIENDLY_WAKE = WOKEN_BY_EQUAL_OR_HIGHER_PRIO
                  or WOKEN_BY_HARDIRQ
                  or WOKEN_BY_NMI
                  or KTHREAD_SHOULD_STOP

  ALLOWLIST = BLOCK_ON_RT_MUTEX
           or FUTEX_LOCK_PI
           or TASK_IS_RCU
           or TASK_IS_MIGRATION

```
除了上述场景外，本规范还处理一些特殊情况：

  - `KERNEL_THREAD`：内核任务没有任何可以被识别为有效实时睡眠原因的模式。因此不对内核任务的睡眠原因进行检查  - `KTHREAD_SHOULD_STOP`：非实时线程可以通过唤醒实时内核线程并等待其退出（`kthread_stop()`）来停止它。这种唤醒对实时是安全的  - `ALLOWLIST`：用于处理内核中已知的误报  - `BLOCK_ON_RT_MUTEX` 因其实现而被包含在允许列表中。在 rt_mutex 的释放路径中，被提升优先级的任务在唤rt_mutex 的等待者之前被取消提升。因此，监视器可能看到一个对实时不安全的唤醒（例如非实时任务唤醒实时任务）。这实际上是实时安全的，因为在此期间禁用了抢占  - `FUTEX_LOCK_PI` 因与 `BLOCK_ON_RT_MUTEX` 相同的原因而被包含在允许列表中