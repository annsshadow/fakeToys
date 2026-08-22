锘?# Interaction 鐨?Suspend code (S3) 涓?the CPU hotplug infrastructure


(C) 2011 - 2014 Srivatsa S. Bhat <srivatsa.bhat@linux.vnet.ibm.com>


## I. Differences 涔嬮棿 CPU hotplug 鍜?Suspend-to-RAM


如何 执行 the regular CPU hotplug code differ 来自 如何 the Suspend-to-RAM
infrastructure uses internally 何处 执行 它们 share 通用 code

Well, 一picture worth 一thousand words... 因此 ASCII art follows :-)

[depicts the 电流 design the 内核, focuses the
interactions involving the freezer 鍜?CPU hotplug 鍜，涔?tries 鍒?explain
the locking involved. outlines the notifications involved 作为 well.
注意 此处, the call paths illustrated, the aim
describing 何处 它们 take 不同 paths 何处 它们 share code.
什happens regular CPU hotplug Suspend-to-RAM race 每个 其他
depicted 此处.]

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
Resuming back likewise, the counterparts 正在 (the order 
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
- 释放 系统_transition_互斥


它是 noted 此处 the 系统_transition_互斥acquired the
very beginning, 我们 just starting out suspend, 然后 released 
之后 the entire cycle complete (i.e., suspend + resume).

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
因此, 作为 seen 来自 the two diagrams (the parts marked 作为 "通用 code"),
regular CPU hotplug 鍜?the suspend code path converge 鍦?the _CPU_down() 鍜。
_CPU_up() 函数. 它们 differ the arguments passed 这些 函数,
期间 regular CPU hotplug, 0 passed 用于 the 'tasks_frozen'
参数. 期间 suspend, since the tasks 已经 frozen the time
the non-boot CPUs offlined onlined, the _CPU_*() 函数 called
the 'tasks_frozen' 参数 set 1.
[参见 下文 用于 一known issues regarding ]


### 重要 文件 函数/条目 points:


- 内核/电源/进程.c : freeze_进程(), thaw_进程()
- 内核/电源/suspend.c : suspend_prepare(), suspend_enter(), suspend_finish()
- 内核/CPU.c: CPU_[up|down](), _CPU_[up|down](),
  [禁用|启用]_nonboot_cpus()



### II. 什the issues involved CPU hotplug


存在 一interesting situations involving CPU hotplug microcode
更新 the CPUs, 作为 discussed 下文:

[bear mind the 内核 requests the microcode images 来自
userspace, 使用 the 请求_固件() 函数 定义 
驱动/base/固件_loader/主要.c]


一 全部 the CPUs identical:

   这是 the 大多通用 situation 它是 quite straightforward: 我们 希望
   apply the 相同 microcode revision 每个 the CPUs.
   give 一示例 x86, the collect_CPU_info() 函数 定义 
   arch/x86/内核/microcode_核心.c helps discovering the 类型 the CPU
   鍜?thereby 鍦?applying the correct microcode revision 鍒，瀹。
   注意 the 内核 执行 maintain 一通用 microcode image 用于 the
   全部 CPUs, 为了 handle case 'b' 描述 下文.


b. 一the CPUs 不同 the rest:

   case since 我们 probably 需apply 不同 microcode revisions
   不同 CPUs, the 内核 maintains 一copy the correct microcode
   image 用于 每个 CPU (之后 appropriate CPU 类型/型号 discovery 使用
   函数 例如 collect_CPU_info()).


c. 一CPU physically hot-unplugged 一(possibly 不同
   类型  CPU hot-plugged 进入 the 系统:

   the 电流 design the 内核, whenever 一CPU taken offline 期间
   一regular CPU hotplug 操作, upon receiving the CPU_DEAD notification
   (sent the CPU hotplug code), the microcode 更新 驱动's
   回调函数 用于 事件 reacts freeing the 内核's copy the
   microcode image 用于 CPU.

   Hence, 一CPU brought online, since the 内核 finds 
   doesn't 具有 the microcode image, 执行 the CPU 类型/型号 discovery
   afresh 然后 requests the userspace 用于 the appropriate microcode image
   用于 CPU, subsequently applied.

   例如, x86, the mc_CPU_回调函数() 函数 (the microcode
   更新 驱动's 回调函数 registered 用于 CPU hotplug 事件) calls
   microcode_更新_CPU() 将会 call microcode_初始化_CPU() case,
   而非 microcode_resume_CPU() finds the 内核 doesn't
   具有 一valid microcode image. ensures the CPU 类型/型号
   discovery performed the right microcode applied the CPU 之后
   getting 来自 userspace.


d. Handling microcode 更新 期间 suspend/hibernate:

   Strictly speaking, 期间 一CPU hotplug 操作 执行 involve
   physically removing 鎴?inserting CPUs, the CPUs 鏄，涓?actually powered
   off 期间 一CPU offline. 它们just put the lowest C-states 可能.
   Hence, 此类 一case, 它是 really 必要 re-apply microcode
   the CPUs brought back online, since 它们 wouldn't 具有 lost the
   image 期间 the CPU offline 操作.

   这是 the usual scenario encountered 期间 一resume 之后 一suspend.
   然 在该情况hibernation, since 全部 the CPUs completely
   powered off, 期间 restore becomes 必要 apply the microcode
   images 全部 the CPUs.

   [注意 我们 don't expect someone physically pull out nodes insert
   nodes 一不同 类型 CPUs in-between 一suspend-resume 一
   hibernate/restore cycle.]

   the 电流 design the 内核 然 期间 一CPU offline 操作
   作为 part the suspend/hibernate cycle (cpuhp_tasks_frozen set),
   the existing copy microcode image the 内核 freed up.
   鍜，鏈熼棿 the CPU online 鎿嶄綔 (鏈熼棿 resume/restore), since the
   内核 finds 已经 具有 copies the microcode images 用于 全部 the
   CPUs, just applies them the CPUs, avoiding 任何 re-discovery CPU
   类型/型号 the 需用于 validating 是否 the microcode revisions 
   right 用于 the CPUs (由于 the 上文 assumption 物理 CPU
   hotplug 已完in-between suspend/resume hibernate/restore
   cycles).


## III. Known problems


那里 任何 known problems regular CPU hotplug suspend race
每个 其他.

Yes, 它们listed 下文:

1. invoking regular CPU hotplug, the 'tasks_frozen' 参数 passed 
   the _CPU_down() _CPU_up() 函数 **始终** 0.
   可能 reflect the true 电流 状the 系统, since the
   tasks 可以 具有 已经 frozen 一out-of-band 事件 例如 一suspend
   操作 progress. Hence, the cpuhp_tasks_frozen variable 
   reflect the frozen 状the CPU hotplug callbacks evaluate
   variable 可能 execute the wrong code path.

2. 一regular CPU hotplug stress test happens race the freezer due
   一suspend 操作 progress 同时, 然后 我们 可以 hit the
   situation 描述 下文:

    - 一regular CPU online 操作 continues journey 来自 userspace
      进入 the 内核, since the freezing 具有 尚未 begun.
    - 然后 freezer gets work freezes userspace.
    - CPU online 具有 尚未 completed the microcode 更新 stuff 现在,
      现在 启动 waiting the frozen userspace the
      TASK_UNINTERRUPTIBLE 状 为了 get the microcode image.
    - 现在 the freezer continues tries freeze the remaining tasks. 
      由于 wait mentioned 上文, the freezer won't able freeze
      the CPU online hotplug task 鍜?hence freezing 鐨?tasks fails.

   因此 task freezing failure, the suspend 操作 gets
   aborted.
