## Freezing 的 tasks


(C) 2007 Rafael J. Wysocki <rjw@sisk.pl>, GPL

## I. 什么 是 the freezing 的 tasks?


The freezing 的 tasks 是 一个 mechanism 由 其 用户空间 进程 和 一些
内核 线程 是 controlled 期间 hibernation 或 system-wide suspend (在 一些
architectures).

## II. 如何 执行 它 work?


存在 one per-task 标志 (PF_NOFREEZE) 和 three per-task states
(TASK_FROZEN, TASK_FREEZABLE 和 __TASK_FREEZABLE_UNSAFE) 使用 用于 该.
The tasks 该 具有 PF_NOFREEZE unset (全部 用户空间 tasks 和 一些 内核
线程) 是 regarded 作为 'freezable' 和 treated 在 一个 特殊 way 之前 the
系统 enters 一个 sleep 状态 以及 之前 一个 hibernation image 是 已创建
(hibernation 是 directly covered 由 什么 follows, 但 the description applies
到 system-wide suspend too).

Namely, 作为 the 第一 step 的 the hibernation procedure the 函数
freeze_进程() (定义 在 内核/电源/进程.c) 是 called.  一个 system-wide
静态 key freezer_active (相对于 一个 per-task 标志 或 状态) 是 使用 到
indicate 是否 the 系统 是 到 undergo 一个 freezing 操作. 和
freeze_进程() sets 此 静态 key.  之后 此, 它 executes
try_到_freeze_tasks() 该 sends 一个 fake 信号 到 全部 用户空间 进程, 和
wakes up 全部 the 内核 线程. 全部 freezable tasks 必须 react 到 该 由
calling try_到_freeze(), 其 results 在 一个 call 到 __refrigerator() (定义
在 内核/freezer.c), 其 changes the task's 状态 到 TASK_FROZEN, 和 makes
它 loop 直到 它是 woken 由 一个 explicit TASK_FROZEN wakeup. 然后, 该 task
是 regarded 作为 'frozen' 和 因此 the set 的 函数 handling 此 mechanism 是
referred 到 作为 'the freezer' (这些 函数 是 定义 在
内核/电源/进程.c, 内核/freezer.c & 包含/linux/freezer.h). 用户空间
tasks 是 generally frozen 之前 内核 线程.

__refrigerator() 必须 不 为 called directly.  改为, 使用 the
try_到_freeze() 函数 (定义 在 包含/linux/freezer.h), 该 checks
若 the task 是 到 为 frozen 和 makes the task enter __refrigerator().

用于 用户空间 进程 try_到_freeze() 是 called automatically 来自 the
signal-handling code, 但 the freezable 内核 线程 需要 到 call 它
explicitly 在 suitable places 或 使用 the wait_事件_freezable() 或
wait_事件_freezable_超时() macros (定义 在 包含/linux/wait.h)
该 put the task 到 sleep (TASK_INTERRUPTIBLE) 或 freeze 它 (TASK_FROZEN) 若
freezer_active 是 set. The 主要 loop 的 一个 freezable 内核 线程 可 look
```

	set_freezable();

	while (true) {
		struct task_struct *tsk = NULL;

		wait_event_freezable(oom_reaper_wait, oom_reaper_list != NULL);
		spin_lock_irq(&oom_reaper_lock);
		if (oom_reaper_list != NULL) {
			tsk = oom_reaper_list;
			oom_reaper_list = tsk->oom_reaper_list;
		}
		spin_unlock_irq(&oom_reaper_lock);

		if (tsk)
			oom_reap_task(tsk);
	}

```
**(来自 mm/oom_kill.c**
: oom_reaper()).

若 一个 freezable 内核 线程 是 不 put 到 the frozen 状态 之后 the freezer
具有 initiated 一个 freezing 操作, the freezing 的 tasks 将 fail 和 the
entire system-wide transition 将 为 cancelled.  用于 此 reason, freezable
内核 线程 必须 call try_到_freeze() somewhere 或 使用 one 的 the
wait_事件_freezable() 和 wait_事件_freezable_超时() macros.

之后 the 系统 内存 状态 具有 已经 restored 来自 一个 hibernation image 和
设备 具有 已经 reinitialized, the 函数 thaw_进程() 是 called 在
order 到 wake up 每个 frozen task.  然后, the tasks 该 具有 已经 frozen leave
__refrigerator() 和 continue 运行中.


### Rationale behind the 函数 dealing 与 freezing 和 thawing 的 tasks


freeze_进程():
  - freezes 仅 userspace tasks

freeze_内核_线程():
  - freezes 全部 tasks (including 内核 线程) 因为 我们可以't freeze
    内核 线程 无 freezing userspace tasks

thaw_内核_线程():
  - thaws 仅 内核 线程; 这是 particularly useful 若 我们 需要 到 执行
    anything 特殊 在 之间 thawing 的 内核 线程 和 thawing 的
    userspace tasks, 或 若 我们 希望 到 postpone the thawing 的 userspace tasks

thaw_进程():
  - thaws 全部 tasks (including 内核 线程) 因为 我们可以't thaw userspace
    tasks 无 thawing 内核 线程


## III. 其 内核 线程 是 freezable?


内核 线程 是 不 freezable 默认情况下.  然而, 一个 内核 线程 可 clear
PF_NOFREEZE 用于 itself 由 calling set_freezable() (the resetting 的 PF_NOFREEZE
directly 是 不 allowed).  来自 此 point 它是 regarded 作为 freezable
和 必须 call try_到_freeze() 或 variants 的 wait_事件_freezable() 在 一个
suitable place.

## IV. 为何 执行 我们 执行 该?


Generally speaking, 存在 一个 couple 的 reasons 到 使用 the freezing 的 tasks:

1. The principal reason 是 到 prevent 文件系统 来自 正在 damaged 之后
   hibernation.  在 the moment 我们 具有 无 简单 means 的 checkpointing
   文件系统, 因此 若 存在 任何 modifications made 到 文件系统 数据 和/或
   metadata 在 disks, 我们 cannot bring them back 到 the 状态 来自 之前 the
   modifications.  同时 每个 hibernation image 包含 一些
   filesystem-related information 该 必须 为 consistent 与 the 状态 的 the
   on-disk 数据 和 metadata 之后 the 系统 内存 状态 具有 已经 restored
   来自 the image (否则 the 文件系统 将 为 damaged 在 一个 nasty way,
   通常 making them almost impossible 到 repair).  我们 因此 freeze
   tasks 该 可能 cause the on-disk 文件系统' 数据 和 metadata 到 为
   modified 之后 the hibernation image 具有 已经 已创建 和 之前 the
   系统 是 finally powered off. The majority 的 这些 是 用户空间
   进程, 但 若 任何 的 the 内核 线程 可 cause something 类似 此
   到 happen, 它们 具有 到 为 freezable.

2. 接下来, 到 创建 the hibernation image 我们 需要 到 free 一个 sufficient amount 的
   内存 (approximately 50% 的 可用 RAM) 和 我们 需要 到 执行 该 之前
   设备 是 deactivated, 因为 我们 generally 需要 them 用于 swapping out.
   然后, 之后 the 内存 用于 the image 具有 已经 freed, 我们 don't 希望 tasks
   到 allocate 额外 内存 和 我们 prevent them 来自 doing 该 由
   freezing them 更早. [的 course, 此 也 means 该 设备 驱动
   应当 不 allocate substantial amounts 的 内存 来自 它们的 .suspend()
   callbacks 之前 hibernation, 但 这是 一个 separate issue.]

3. The third reason 是 到 prevent 用户空间 进程 和 一些 内核 线程
   来自 interfering 与 the suspending 和 resuming 的 设备.  一个 用户空间
   进程 运行中 在 一个 second CPU 同时 我们 是 suspending 设备 可, 用于
   示例, 为 troublesome 和 无 the freezing 的 tasks 我们 将会 需要 一些
   safeguards against race conditions 该 可能 occur 在 此类 一个 case.

尽管 Linus Torvalds doesn't 类似 the freezing 的 tasks, he said 此 在 one
的 the discussions 在 LKML (https://lore.kernel.org/r/alpine.LFD.0.98.0704271801020.9964@woody.linux-foundation.org):

"RJW:> 为何 我们 freeze tasks 在 全部 或 为何 我们 freeze 内核 线程?

Linus: 在 许多 ways, '在 全部'.

I **执行** realize the IO 请求 队列 issues, 和 该 我们 cannot actually 执行
s2ram 与 一些 设备 在 the middle 的 一个 DMA.  因此 我们 希望 到 为 able 到
avoid **该**, 那里's 无 question 关于 该.  和 I suspect 该 stopping
用户 线程 和 然后 waiting 用于 一个 sync 是 practically one 的 the easier
ways 到 执行 因此.

因此 在 practice, the '在 全部' 可 become 一个 '为何 freeze 内核 线程?' 和
freezing 用户 线程 I don't find really objectionable."

仍然, 存在 内核 线程 该 可 希望 到 为 freezable.  例如, 若
一个 内核 线程 该 belongs 到 一个 设备 驱动 accesses the 设备 directly, 它
在 principle needs 到 know 当 the 设备 是 suspended, 因此 该 它 doesn't try
到 access 它 在 该 time.  然而, 若 the 内核 线程 是 freezable, 它 将
为 frozen 之前 the 驱动's .suspend() 回调函数 是 executed 和 它 将 为
thawed 之后 the 驱动's .resume() 回调函数 具有 运行, 因此 它 won't 为 accessing
the 设备 同时 它's suspended.

4. Another reason 用于 freezing tasks 是 到 prevent 用户空间 进程 来自
   realizing 该 hibernation (或 suspend) 操作 takes place.  Ideally, 用户
   space 进程 应当 不 notice 该 此类 一个 system-wide 操作 具有
   occurred 和 应当 continue 运行中 无 任何 problems 之后 the restore
   (或 resume 来自 suspend).  Unfortunately, 在 the 大多数 通用 case 此
   是 quite difficult 到 achieve 无 the freezing 的 tasks.  Consider,
   例如, 一个 进程 该 depends 在 全部 CPUs 正在 online 同时 它's
   运行中.  Since 我们 需要 到 禁用 nonboot CPUs 期间 the hibernation,
   若 此 进程 是 不 frozen, 它 可 notice 该 the 数字 的 CPUs 具有
   changed 和 可 启动 到 work incorrectly 因为 的 该.

## V. 是 那里 任何 problems related 到 the freezing 的 tasks?


Yes, 存在.

第一 的 全部, the freezing 的 内核 线程 可 为 tricky 若 它们 depend one
在 another.  例如, 若 内核 线程 一个 waits 用于 一个 completion (在 the
TASK_UNINTERRUPTIBLE 状态) 该 needs 到 为 已完成 由 freezable 内核 线程 B
和 B 是 frozen 在 the meantime, 然后 一个 将 为 blocked 直到 B 是 thawed, 其
可 为 undesirable.  该's 为何 内核 线程 是 不 freezable 默认情况下.

Second, 存在 the 以下 two problems related 到 the freezing 的 用户
space 进程:

1. Putting 进程 进入 一个 uninterruptible sleep distorts the 加载 average.
2. 现在 该 我们 具有 FUSE, 增强版 the framework 用于 doing 设备 驱动 在
   userspace, 它 gets even 更多 complicated 因为 一些 userspace 进程 是
   现在 doing the sorts 的 things 该 内核 线程 执行
   (https://lists.linux-foundation.org/pipermail/linux-pm/2007-May/012309.html).

The problem 1. seems 到 为 fixable, 尽管 它 hasn't 已经 fixed 因此 far.  The
其他 one 是 更多 serious, 但 它 seems 该 我们可以 work around 它 由 使用
hibernation (和 suspend) notifiers (在 该 case, though, 我们 won't 为 able 到
avoid the realization 由 the 用户空间 进程 该 the hibernation 是 taking
place).

存在 也 problems 该 the freezing 的 tasks tends 到 expose, 尽管
它们是 不 directly related 到 它.  例如, 若 请求_固件() 是
called 来自 一个 设备 驱动's .resume() routine, 它 将 超时 和 eventually
fail, 因为 the 用户 land 进程 该 应当 respond 到 the 请求 是 frozen
在 此 point.  因此, seemingly, the failure 是 由于 the freezing 的 tasks.
Suppose, 然而, 该 the 固件 文件 是 located 在 一个 文件系统 accessible
仅 through another 设备 该 hasn't 已经 resumed 尚未.  在 该 case,
请求_固件() 将 fail regardless 的 是否 或 不 the freezing 的 tasks
是 使用.  Consequently, the problem 是 不 really related 到 the freezing 的
tasks, since 它 generally exists anyway.

一个 驱动 必须 具有 全部 firmwares 它 可 需要 在 RAM 之前 suspend() 是 called.
若 keeping them 是 不 practical, 例如 由于 它们的 大小, 它们 必须 为
requested early enough 使用 the suspend notifier API 描述 在
Documentation/driver-api/pm/notifiers.rst.

## VI. 是 那里 任何 precautions 到 为 taken 到 prevent freezing failures?


Yes, 存在.

第一 的 全部, grabbing the '系统_transition_互斥体' 锁 到 mutually exclude 一个
piece 的 code 来自 system-wide sleep 例如 suspend/hibernation 是 不
encouraged.  若 可能, 该 piece 的 code 必须 改为 hook onto the
suspend/hibernation notifiers 到 achieve mutual exclusion. Look 在 the
CPU-Hotplug code (内核/CPU.c) 用于 一个 示例.

然而, 若 即 不 feasible, 和 grabbing '系统_transition_互斥体' 是
deemed 必要, 它是 strongly discouraged 到 directly call
互斥体_[un]锁(&系统_transition_互斥体) since 该 可以 lead 到 freezing
failures, 因为 若 the suspend/hibernate code successfully acquired the
'系统_transition_互斥体' 锁, 和 hence 该 其他 entity failed 到 acquire
the 锁, 然后 该 task 将会 get blocked 在 TASK_UNINTERRUPTIBLE 状态. 作为 一个
consequence, the freezer 将会 不 为 able 到 freeze 该 task, leading 到
freezing failure.

然而, the [un]锁_系统_sleep() APIs 是 safe 到 使用 在 此 scenario,
since 它们 ask the freezer 到 skip freezing 此 task, since 它是 anyway
"frozen enough" 作为 它是 blocked 在 '系统_transition_互斥体', 其 将 为
released 仅 之后 the entire suspend/hibernation sequence 是 complete.  因此, 到
summarize, 使用 [un]锁_系统_sleep() 而非 directly 使用
互斥体_[un]锁(&系统_transition_互斥体). 该 将会 prevent freezing failures.

## V. Miscellaneous


/sys/电源/pm_freeze_超时 controls 如何 long 它 将 cost 至多 到 freeze
全部 用户空间 进程 或 全部 freezable 内核 线程, 在 unit 的
millisecond.  The 默认 值 是 20000, 与 range 的 unsigned integer.
