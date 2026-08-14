## Interaction 的 Suspend code (S3) 与 the CPU hotplug infrastructure


(C) 2011 - 2014 Srivatsa S. Bhat <srivatsa.bhat@linux.vnet.ibm.com>


## I. Differences 之间 CPU hotplug 和 Suspend-to-RAM


如何 执行 the regular CPU hotplug code differ 来自 如何 the Suspend-to-RAM
infrastructure uses 它 internally? 和 何处 执行 它们 share 通用 code?

Well, 一个 picture 是 worth 一个 thousand words... 因此 ASCII art follows :-)

[此 depicts the 电流 design 在 the 内核, 和 focuses 仅 在 the
interactions involving the freezer 和 CPU hotplug 和 也 tries 到 explain
the locking involved. 它 outlines the notifications involved 作为 well.
但 请 注意 该 此处, 仅 the call paths 是 illustrated, 与 the aim
的 describing 何处 它们 take 不同 paths 和 何处 它们 share code.
什么 happens 当 regular CPU hotplug 和 Suspend-to-RAM race 与 每个 其他
是 不 depicted 此处.]

```

  |Freeze| -> |Disable nonboot| -> |Do suspend| -> |Enable nonboot| -> |Thaw |
  |tasks |    |     cpus      |    |          |    |     cpus     |    |tasks|


```
```

                                Suspend call path
                                -----------------

                                  Write 'mem' to
                                /sys/power/state
                                    sysfs file
                                        |
                                        v
                               Acquire system_transition_mutex lock
                                        |
                                        v
                             Send PM_SUSPEND_PREPARE
                                   notifications
                                        |
                                        v
                                   Freeze tasks
                                        |
                                        |
                                        v
                              freeze_secondary_cpus()
                                   /* start */
                                        |
                                        v
                            Acquire cpu_add_remove_lock
                                        |
                                        v
                             Iterate over CURRENTLY
                                   online CPUs
                                        |
                                        |
                                        |                ----------
                                        v                          | L
             ======>               _cpu_down()                     |
            |              [This takes cpuhotplug.lock             |
  Common    |               before taking down the CPU             |
   code     |               and releases it when done]             | O
            |            While it is at it, notifications          |
            |            are sent when notable events occur,       |
             ======>     by running all registered callbacks.      |
                                        |                          | O
                                        |                          |
                                        |                          |
                                        v                          |
                            Note down these cpus in                | P
                                frozen_cpus mask         ----------
                                        |
                                        v
                           Disable regular cpu hotplug
                        by increasing cpu_hotplug_disabled
                                        |
                                        v
                            Release cpu_add_remove_lock
                                        |
                                        v
                       /* freeze_secondary_cpus() complete */
                                        |
                                        v
                                   Do suspend



```
Resuming back 是 likewise, 与 the counterparts 正在 (在 the order 的
execution 期间 resume):

```

   |  Acquire cpu_add_remove_lock
   |  Decrease cpu_hotplug_disabled, thereby enabling regular cpu hotplug
   |  Call _cpu_up() [for all those cpus in the frozen_cpus mask, in a loop]
   |  Release cpu_add_remove_lock
   v

```
- thaw tasks
- send PM_POST_SUSPEND notifications
- 释放 系统_transition_互斥体 锁.


它是 到 为 noted 此处 该 the 系统_transition_互斥体 锁 是 acquired 在 the
very beginning, 当 我们 是 just starting out 到 suspend, 和 然后 released 仅
之后 the entire cycle 是 complete (i.e., suspend + resume).

```



                          Regular CPU hotplug call path
                          -----------------------------

                                Write 0 (or 1) to
                       /sys/devices/system/cpu/cpu*/online
                                    sysfs file
                                        |
                                        |
                                        v
                                    cpu_down()
                                        |
                                        v
                           Acquire cpu_add_remove_lock
                                        |
                                        v
                          If cpu_hotplug_disabled > 0
                                return gracefully
                                        |
                                        |
                                        v
             ======>                _cpu_down()
            |              [This takes cpuhotplug.lock
  Common    |               before taking down the CPU
   code     |               and releases it when done]
            |            While it is at it, notifications
            |           are sent when notable events occur,
             ======>    by running all registered callbacks.
                                        |
                                        |
                                        v
                          Release cpu_add_remove_lock
                               [That's it!, for
                              regular CPU hotplug]



```
因此, 作为 可 为 seen 来自 the two diagrams (the parts marked 作为 "通用 code"),
regular CPU hotplug 和 the suspend code path converge 在 the _CPU_down() 和
_CPU_up() 函数. 它们 differ 在 the arguments passed 到 这些 函数,
在 该 期间 regular CPU hotplug, 0 是 passed 用于 the 'tasks_frozen'
参数. 但 期间 suspend, since the tasks 是 已经 frozen 由 the time
the non-boot CPUs 是 offlined 或 onlined, the _CPU_*() 函数 是 called
与 the 'tasks_frozen' 参数 set 到 1.
[参见 下文 用于 一些 known issues regarding 此.]


### 重要 文件 和 函数/条目 points:


- 内核/电源/进程.c : freeze_进程(), thaw_进程()
- 内核/电源/suspend.c : suspend_prepare(), suspend_enter(), suspend_finish()
- 内核/CPU.c: CPU_[up|down](), _CPU_[up|down](),
  [禁用|启用]_nonboot_cpus()



### II. 什么 是 the issues involved 在 CPU hotplug?


存在 一些 interesting situations involving CPU hotplug 和 microcode
更新 在 the CPUs, 作为 discussed 下文:

[请 bear 在 mind 该 the 内核 requests the microcode images 来自
userspace, 使用 the 请求_固件() 函数 定义 在
驱动/base/固件_loader/主要.c]


一个. 当 全部 the CPUs 是 identical:

   这是 the 大多数 通用 situation 和 它是 quite straightforward: 我们 希望
   到 apply the 相同 microcode revision 到 每个 的 the CPUs.
   到 give 一个 示例 的 x86, the collect_CPU_info() 函数 定义 在
   arch/x86/内核/microcode_核心.c helps 在 discovering the 类型 的 the CPU
   和 thereby 在 applying the correct microcode revision 到 它.
   但 注意 该 the 内核 执行 不 maintain 一个 通用 microcode image 用于 the
   全部 CPUs, 为了 handle case 'b' 描述 下文.


b. 当 一些 的 the CPUs 是 不同 比 the rest:

   在 此 case since 我们 probably 需要 到 apply 不同 microcode revisions
   到 不同 CPUs, the 内核 maintains 一个 copy 的 the correct microcode
   image 用于 每个 CPU (之后 appropriate CPU 类型/型号 discovery 使用
   函数 例如 collect_CPU_info()).


c. 当 一个 CPU 是 physically hot-unplugged 和 一个 新 (和 possibly 不同
   类型 的) CPU 是 hot-plugged 进入 the 系统:

   在 the 电流 design 的 the 内核, whenever 一个 CPU 是 taken offline 期间
   一个 regular CPU hotplug 操作, upon receiving the CPU_DEAD notification
   (其 是 sent 由 the CPU hotplug code), the microcode 更新 驱动's
   回调函数 用于 该 事件 reacts 由 freeing the 内核's copy 的 the
   microcode image 用于 该 CPU.

   Hence, 当 一个 新 CPU 是 brought online, since the 内核 finds 该 它
   doesn't 具有 the microcode image, 它 执行 the CPU 类型/型号 discovery
   afresh 和 然后 requests the userspace 用于 the appropriate microcode image
   用于 该 CPU, 其 是 subsequently applied.

   例如, 在 x86, the mc_CPU_回调函数() 函数 (其 是 the microcode
   更新 驱动's 回调函数 registered 用于 CPU hotplug 事件) calls
   microcode_更新_CPU() 其 将会 call microcode_初始化_CPU() 在 此 case,
   而非 microcode_resume_CPU() 当 它 finds 该 the 内核 doesn't
   具有 一个 valid microcode image. 此 ensures 该 the CPU 类型/型号
   discovery 是 performed 和 the right microcode 是 applied 到 the CPU 之后
   getting 它 来自 userspace.


d. Handling microcode 更新 期间 suspend/hibernate:

   Strictly speaking, 期间 一个 CPU hotplug 操作 其 执行 不 involve
   physically removing 或 inserting CPUs, the CPUs 是 不 actually powered
   off 期间 一个 CPU offline. 它们是 just put 到 the lowest C-states 可能.
   Hence, 在 此类 一个 case, 它是 不 really 必要 到 re-apply microcode
   当 the CPUs 是 brought back online, since 它们 wouldn't 具有 lost the
   image 期间 the CPU offline 操作.

   这是 the usual scenario encountered 期间 一个 resume 之后 一个 suspend.
   然而, 在该情况下 的 hibernation, since 全部 the CPUs 是 completely
   powered off, 期间 restore 它 becomes 必要 到 apply the microcode
   images 到 全部 the CPUs.

   [注意 该 我们 don't expect someone 到 physically pull out nodes 和 insert
   nodes 与 一个 不同 类型 的 CPUs in-between 一个 suspend-resume 或 一个
   hibernate/restore cycle.]

   在 the 电流 design 的 the 内核 然而, 期间 一个 CPU offline 操作
   作为 part 的 the suspend/hibernate cycle (cpuhp_tasks_frozen 是 set),
   the existing copy 的 microcode image 在 the 内核 是 不 freed up.
   和 期间 the CPU online 操作 (期间 resume/restore), since the
   内核 finds 该 它 已经 具有 copies 的 the microcode images 用于 全部 the
   CPUs, 它 just applies them 到 the CPUs, avoiding 任何 re-discovery 的 CPU
   类型/型号 和 the 需要 用于 validating 是否 the microcode revisions 是
   right 用于 the CPUs 或 不 (由于 the 上文 assumption 该 物理 CPU
   hotplug 将 不 为 已完成 in-between suspend/resume 或 hibernate/restore
   cycles).


## III. Known problems


是 那里 任何 known problems 当 regular CPU hotplug 和 suspend race
与 每个 其他?

Yes, 它们是 listed 下文:

1. 当 invoking regular CPU hotplug, the 'tasks_frozen' 参数 passed 到
   the _CPU_down() 和 _CPU_up() 函数 是 **始终** 0.
   此 可能 不 reflect the true 电流 状态 的 the 系统, since the
   tasks 可以 具有 已经 frozen 由 一个 out-of-band 事件 例如 一个 suspend
   操作 在 progress. Hence, the cpuhp_tasks_frozen variable 将 不
   reflect the frozen 状态 和 the CPU hotplug callbacks 其 evaluate
   该 variable 可能 execute the wrong code path.

2. 若 一个 regular CPU hotplug stress test happens 到 race 与 the freezer due
   到 一个 suspend 操作 在 progress 同时, 然后 我们 可以 hit the
   situation 描述 下文:

    - 一个 regular CPU online 操作 continues 其 journey 来自 userspace
      进入 the 内核, since the freezing 具有 不 尚未 begun.
    - 然后 freezer gets 到 work 和 freezes userspace.
    - 若 CPU online 具有 不 尚未 completed the microcode 更新 stuff 由 现在,
      它 将 现在 启动 waiting 在 the frozen userspace 在 the
      TASK_UNINTERRUPTIBLE 状态, 为了 get the microcode image.
    - 现在 the freezer continues 和 tries 到 freeze the remaining tasks. 但
      由于 此 wait mentioned 上文, the freezer won't 为 able 到 freeze
      the CPU online hotplug task 和 hence freezing 的 tasks fails.

   因此 的 此 task freezing failure, the suspend 操作 gets
   aborted.
