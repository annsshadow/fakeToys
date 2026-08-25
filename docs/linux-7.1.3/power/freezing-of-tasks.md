锘?# Freezing 鐨?tasks


(C) 2007 Rafael J. Wysocki <rjw@sisk.pl>, GPL

## I. 什??the freezing ?tasks?


The freezing ?tasks ?一?mechanism ??用户空间 进程 ?一。
内核 线程 ?controlled 期间 hibernation ?system-wide suspend (?一。
architectures).

## II. 如何 执行 ?work?


存在 one per-task 标志 (PF_NOFREEZE) ?three per-task states
(TASK_FROZEN, TASK_FREEZABLE ?__TASK_FREEZABLE_UNSAFE) 使用 用于 ?
The tasks ?具有 PF_NOFREEZE unset (全部 用户空间 tasks ?一，内核
线程) ?regarded 作为 'freezable' ?treated ?一，特殊 way 之前 the
系统 enters 一?sleep 状，以及 之前 一?hibernation image ?已创。
(hibernation ?directly covered ?什?follows, ?the description applies
鍒?system-wide suspend too).

Namely, 作为 the 第一 step ?the hibernation procedure the 函数
freeze_进程() (定义 ?内核/电源/进程.c) ?called.  一?system-wide
静?key freezer_active (相对，一?per-task 标志 ?状? ?使用 ?
indicate 是否 the 系统 ??undergo 一?freezing 操作. ?
freeze_进程() sets ?静?key.  之后 ? ?executes
try_到_freeze_tasks() ?sends 一?fake 信号 ?全部 用户空间 进程, ?
wakes up 全部 the 内核 线程. 全部 freezable tasks 必须 react ???
calling try_到_freeze(), ?results ?一?call ?__refrigerator() (定义
?内核/freezer.c), ?changes the task's 状??TASK_FROZEN, ?makes
?loop 直到 它是 woken ?一?explicit TASK_FROZEN wakeup. 然后, ?task
?regarded 作为 'frozen' ?因此 the set ?函数 handling ?mechanism ?
referred ?作为 'the freezer' (这些 函数 ?定义 ?
内核/电源/进程.c, 内核/freezer.c & 包含/linux/freezer.h). 用户空间
tasks ?generally frozen 之前 内核 线程.

__refrigerator() 必须 ??called directly.  改为, 使用 the
try_到_freeze() 函数 (定义 ?包含/linux/freezer.h), ?checks
鑻?the task 鏄，鍒，涓?frozen 鍜?makes the task enter __refrigerator().

用于 用户空间 进程 try_到_freeze() ?called automatically 来自 the
signal-handling code, ?the freezable 内核 线程 需??call ?
explicitly ?suitable places ?使用 the wait_事件_freezable() ?
wait_事件_freezable_超时() macros (定义 ?包含/linux/wait.h)
璇?put the task 鍒?sleep (TASK_INTERRUPTIBLE) 鎴?freeze 瀹?(TASK_FROZEN) 鑻。
freezer_active ?set. The 主要 loop ?一?freezable 内核 线程 ?look
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

?一?freezable 内核 线程 ??put ?the frozen 状，之后 the freezer
具有 initiated 一?freezing 操作, the freezing ?tasks ?fail ?the
entire system-wide transition ??cancelled.  用于 ?reason, freezable
内核 线程 必须 call try_到_freeze() somewhere ?使用 one ?the
wait_事件_freezable() ?wait_事件_freezable_超时() macros.

之后 the 系统 内存 状，具有 已经 restored 来自 一?hibernation image ?
设备 具有 已经 reinitialized, the 函数 thaw_进程() ?called ?
order ?wake up 每个 frozen task.  然后, the tasks ?具有 已经 frozen leave
__refrigerator() ?continue 运行。


### Rationale behind the 函数 dealing ?freezing ?thawing ?tasks


freeze_进程():
  - freezes 浠?userspace tasks

freeze_内核_线程():
  - freezes 全部 tasks (including 内核 线程) 因为 我们可以't freeze
    内核 线程 ?freezing userspace tasks

thaw_内核_线程():
  - thaws ?内核 线程; 这是 particularly useful ?我们 需??执行
    anything 特殊 ?之间 thawing ?内核 线程 ?thawing ?
    userspace tasks, ??我们 希望 ?postpone the thawing ?userspace tasks

thaw_进程():
  - thaws 全部 tasks (including 内核 线程) 因为 我们可以't thaw userspace
    tasks ?thawing 内核 线程


## III. ?内核 线程 ?freezable?


内核 线程 ??freezable 默认情况?  然? 一，内核 线程 ?clear
PF_NOFREEZE 用于 itself ?calling set_freezable() (the resetting ?PF_NOFREEZE
directly ??allowed).  来自 ?point 它是 regarded 作为 freezable
?必须 call try_到_freeze() ?variants ?wait_事件_freezable() ?一。
suitable place.

## IV. 为何 执行 我们 执行 ?


Generally speaking, 存在 一?couple ?reasons ?使用 the freezing ?tasks:

1. The principal reason ??prevent 文件系统 来自 正在 damaged 之后
   hibernation.  ?the moment 我们 具有 ?简?means ?checkpointing
   文件系统, 因此 ?存在 任何 modifications made ?文件系统 数据 ??
   metadata ?disks, 我们 cannot bring them back ?the 状，来自 之前 the
   modifications.  同时 每个 hibernation image 包含 一。
   filesystem-related information ?必须 ?consistent ?the 状??the
   on-disk 数据 ?metadata 之后 the 系统 内存 状，具有 已经 restored
   来自 the image (否则 the 文件系统 ??damaged ?一?nasty way,
   通常 making them almost impossible ?repair).  我们 因此 freeze
   tasks ?可能 cause the on-disk 文件系统' 数据 ?metadata ??
   modified 之后 the hibernation image 具有 已经 已创??之前 the
   系统 ?finally powered off. The majority ?这些 ?用户空间
   进程, ??任何 ?the 内核 线程 ?cause something 类似 ?
   ?happen, 它们 具有 ??freezable.

2. 接下? ?创建 the hibernation image 我们 需??free 一?sufficient amount ?
   内存 (approximately 50% ?可用 RAM) ?我们 需??执行 ?之前
   设备 ?deactivated, 因为 我们 generally 需?them 用于 swapping out.
   然后, 之后 the 内存 用于 the image 具有 已经 freed, 我们 don't 希望 tasks
   ?allocate 额外 内存 ?我们 prevent them 来自 doing ??
   freezing them 更早. [?course, ??means ?设备 驱动
   应当 ?allocate substantial amounts ?内存 来自 它们?.suspend()
   callbacks 之前 hibernation, ?这是 一?separate issue.]

3. The third reason ??prevent 用户空间 进程 ?一，内核 线程
   来自 interfering ?the suspending ?resuming ?设备.  一，用户空间
   进程 运行??一?second CPU 同时 我们 ?suspending 设备 ? 用于
   示例, ?troublesome ??the freezing ?tasks 我们 将会 需，一。
   safeguards against race conditions ?可能 occur ?此类 一?case.

尽管 Linus Torvalds doesn't 类似 the freezing ?tasks, he said ??one
鐨?the discussions 鍦?LKML (https://lore.kernel.org/r/alpine.LFD.0.98.0704271801020.9964@woody.linux-foundation.org):

"RJW:> 为何 我们 freeze tasks ?全部 ?为何 我们 freeze 内核 线程。

Linus: ?许多 ways, '?全部'.

I **执行** realize the IO 请求 队列 issues, ??我们 cannot actually 执行
s2ram ?一，设备 ?the middle ?一?DMA.  因此 我们 希望 ??able ?
avoid **?*, 那里's ?question 关于 ?  ?I suspect ?stopping
用户 线程 ?然后 waiting 用于 一?sync ?practically one ?the easier
ways ?执行 因此.

因此 ?practice, the '?全部' ?become 一?'为何 freeze 内核 线程? ?
freezing 用户 线程 I don't find really objectionable."

仍然, 存在 内核 线程 ??希望 ??freezable.  例如, ?
一，内核 线程 ?belongs ?一，设备 驱动 accesses the 设备 directly, ?
?principle needs ?know ?the 设备 ?suspended, 因此 ??doesn't try
?access ???time.  然? ?the 内核 线程 ?freezable, ??
?frozen 之前 the 驱动's .suspend() 回调函数 ?executed ????
thawed 之后 the 驱动's .resume() 回调函数 具有 运行, 因此 ?won't ?accessing
the 设备 同时 ?s suspended.

4. Another reason 用于 freezing tasks ??prevent 用户空间 进程 来自
   realizing ?hibernation (?suspend) 操作 takes place.  Ideally, 用户
   space 进程 应当 ?notice ?此类 一?system-wide 操作 具有
   occurred ?应当 continue 运行??任何 problems 之后 the restore
   (?resume 来自 suspend).  Unfortunately, ?the 大多，通用 case ?
   鏄?quite difficult 鍒?achieve 鏃?the freezing 鐨?tasks.  Consider,
   例如, 一，进程 ?depends ?全部 CPUs 正在 online 同时 ?s
   运行?  Since 我们 需??禁用 nonboot CPUs 期间 the hibernation,
   ??进程 ??frozen, ??notice ?the 数字 ?CPUs 具有
   changed ??启动 ?work incorrectly 因为 ??

## V. ?那里 任何 problems related ?the freezing ?tasks?


Yes, 存在.

第一 ?全部, the freezing ?内核 线程 ??tricky ?它们 depend one
?another.  例如, ?内核 线程 一?waits 用于 一?completion (?the
TASK_UNINTERRUPTIBLE 状? ?needs ??已完??freezable 内核 线程 B
?B ?frozen ?the meantime, 然后 一???blocked 直到 B ?thawed, ?
??undesirable.  ?s 为何 内核 线程 ??freezable 默认情况。

Second, 存在 the 以下 two problems related ?the freezing ?用户
space 进程:

1. Putting 进程 进入 一?uninterruptible sleep distorts the 加载 average.
2. 现在 ?我们 具有 FUSE, 增强?the framework 用于 doing 设备 驱动 ?
   userspace, ?gets even 更多 complicated 因为 一?userspace 进程 ?
   现在 doing the sorts ?things ?内核 线程 执行
   (https://lists.linux-foundation.org/pipermail/linux-pm/2007-May/012309.html).

The problem 1. seems ??fixable, 尽管 ?hasn't 已经 fixed 因此 far.  The
其他 one ?更多 serious, ??seems ?我们可以 work around ??使用
hibernation (?suspend) notifiers (??case, though, 我们 won't ?able ?
avoid the realization ?the 用户空间 进程 ?the hibernation ?taking
place).

存在 ?problems ?the freezing ?tasks tends ?expose, 尽管
它们??directly related ??  例如, ?请求_固件() ?
called 来自 一，设备 驱动's .resume() routine, ??超时 ?eventually
fail, 因为 the 用户 land 进程 ?应当 respond ?the 请求 ?frozen
??point.  因此, seemingly, the failure ?由于 the freezing ?tasks.
Suppose, 然? ?the 固件 文件 ?located ?一，文件系统 accessible
?through another 设备 ?hasn't 已经 resumed 尚未.  ??case,
请求_固件() ?fail regardless ?是否 ??the freezing ?tasks
?使用.  Consequently, the problem ??really related ?the freezing ?
tasks, since 瀹?generally exists anyway.

一，驱动 必须 具有 全部 firmwares ??需??RAM 之前 suspend() ?called.
?keeping them ??practical, 例如 由于 它们，大小, 它们 必须 ?
requested early enough 使用 the suspend notifier API 描述 ?
Documentation/driver-api/pm/notifiers.rst.

## VI. ?那里 任何 precautions ??taken ?prevent freezing failures?


Yes, 存在.

第一 ?全部, grabbing the '系统_transition_互斥? ??mutually exclude 一。
piece ?code 来自 system-wide sleep 例如 suspend/hibernation ??
encouraged.  ?可能, ?piece ?code 必须 改为 hook onto the
suspend/hibernation notifiers 鍒?achieve mutual exclusion. Look 鍦?the
CPU-Hotplug code (内核/CPU.c) 用于 一，示例.

然? ???feasible, ?grabbing '系统_transition_互斥? ?
deemed 必要, 它是 strongly discouraged ?directly call
互斥体_[un]?&系统_transition_互斥? since ?可以 lead ?freezing
failures, 因为 ?the suspend/hibernate code successfully acquired the
'系统_transition_互斥? ? ?hence ?其他 entity failed ?acquire
the ? 然后 ?task 将会 get blocked ?TASK_UNINTERRUPTIBLE 状? 作为 一。
consequence, the freezer 将会 ??able ?freeze ?task, leading ?
freezing failure.

然? the [un]锁_系统_sleep() APIs ?safe ?使用 ??scenario,
since 它们 ask the freezer ?skip freezing ?task, since 它是 anyway
"frozen enough" 作为 它是 blocked ?'系统_transition_互斥?, ???
released ?之后 the entire suspend/hibernation sequence ?complete.  因此, ?
summarize, 使用 [un]锁_系统_sleep() 而非 directly 使用
互斥体_[un]?&系统_transition_互斥?. ?将会 prevent freezing failures.

## V. Miscellaneous


/sys/电源/pm_freeze_超时 controls 如何 long ??cost 至多 ?freeze
全部 用户空间 进程 ?全部 freezable 内核 线程, ?unit ?
millisecond.  The 默认 ??20000, ?range ?unsigned integer.
