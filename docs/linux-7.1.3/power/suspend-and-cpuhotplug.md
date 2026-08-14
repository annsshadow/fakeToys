锘?# Interaction 鐨?Suspend code (S3) 涓?the CPU hotplug infrastructure


(C) 2011 - 2014 Srivatsa S. Bhat <srivatsa.bhat@linux.vnet.ibm.com>


## I. Differences 涔嬮棿 CPU hotplug 鍜?Suspend-to-RAM


濡備綍 鎵ц the regular CPU hotplug code differ 鏉ヨ嚜 濡備綍 the Suspend-to-RAM
infrastructure uses 瀹?internally? 鍜?浣曞 鎵ц 瀹冧滑 share 閫氱敤 code?

Well, 涓€涓?picture 鏄?worth 涓€涓?thousand words... 鍥犳 ASCII art follows :-)

[姝?depicts the 鐢垫祦 design 鍦?the 鍐呮牳, 鍜?focuses 浠?鍦?the
interactions involving the freezer 鍜?CPU hotplug 鍜?涔?tries 鍒?explain
the locking involved. 瀹?outlines the notifications involved 浣滀负 well.
浣?璇?娉ㄦ剰 璇?姝ゅ, 浠?the call paths 鏄?illustrated, 涓?the aim
鐨?describing 浣曞 瀹冧滑 take 涓嶅悓 paths 鍜?浣曞 瀹冧滑 share code.
浠€涔?happens 褰?regular CPU hotplug 鍜?Suspend-to-RAM race 涓?姣忎釜 鍏朵粬
鏄?涓?depicted 姝ゅ.]

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
Resuming back 鏄?likewise, 涓?the counterparts 姝ｅ湪 (鍦?the order 鐨?
execution 鏈熼棿 resume):

```

   |  Acquire cpu_add_remove_lock
   |  Decrease cpu_hotplug_disabled, thereby enabling regular cpu hotplug
   |  Call _cpu_up() [for all those cpus in the frozen_cpus mask, in a loop]
   |  Release cpu_add_remove_lock
   v

```
- thaw tasks
- send PM_POST_SUSPEND notifications
- 閲婃斁 绯荤粺_transition_浜掓枼浣?閿?


瀹冩槸 鍒?涓?noted 姝ゅ 璇?the 绯荤粺_transition_浜掓枼浣?閿?鏄?acquired 鍦?the
very beginning, 褰?鎴戜滑 鏄?just starting out 鍒?suspend, 鍜?鐒跺悗 released 浠?
涔嬪悗 the entire cycle 鏄?complete (i.e., suspend + resume).

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
鍥犳, 浣滀负 鍙?涓?seen 鏉ヨ嚜 the two diagrams (the parts marked 浣滀负 "閫氱敤 code"),
regular CPU hotplug 鍜?the suspend code path converge 鍦?the _CPU_down() 鍜?
_CPU_up() 鍑芥暟. 瀹冧滑 differ 鍦?the arguments passed 鍒?杩欎簺 鍑芥暟,
鍦?璇?鏈熼棿 regular CPU hotplug, 0 鏄?passed 鐢ㄤ簬 the 'tasks_frozen'
鍙傛暟. 浣?鏈熼棿 suspend, since the tasks 鏄?宸茬粡 frozen 鐢?the time
the non-boot CPUs 鏄?offlined 鎴?onlined, the _CPU_*() 鍑芥暟 鏄?called
涓?the 'tasks_frozen' 鍙傛暟 set 鍒?1.
[鍙傝 涓嬫枃 鐢ㄤ簬 涓€浜?known issues regarding 姝?]


### 閲嶈 鏂囦欢 鍜?鍑芥暟/鏉＄洰 points:


- 鍐呮牳/鐢垫簮/杩涚▼.c : freeze_杩涚▼(), thaw_杩涚▼()
- 鍐呮牳/鐢垫簮/suspend.c : suspend_prepare(), suspend_enter(), suspend_finish()
- 鍐呮牳/CPU.c: CPU_[up|down](), _CPU_[up|down](),
  [绂佺敤|鍚敤]_nonboot_cpus()



### II. 浠€涔?鏄?the issues involved 鍦?CPU hotplug?


瀛樺湪 涓€浜?interesting situations involving CPU hotplug 鍜?microcode
鏇存柊 鍦?the CPUs, 浣滀负 discussed 涓嬫枃:

[璇?bear 鍦?mind 璇?the 鍐呮牳 requests the microcode images 鏉ヨ嚜
userspace, 浣跨敤 the 璇锋眰_鍥轰欢() 鍑芥暟 瀹氫箟 鍦?
椹卞姩/base/鍥轰欢_loader/涓昏.c]


涓€涓? 褰?鍏ㄩ儴 the CPUs 鏄?identical:

   杩欐槸 the 澶у鏁?閫氱敤 situation 鍜?瀹冩槸 quite straightforward: 鎴戜滑 甯屾湜
   鍒?apply the 鐩稿悓 microcode revision 鍒?姣忎釜 鐨?the CPUs.
   鍒?give 涓€涓?绀轰緥 鐨?x86, the collect_CPU_info() 鍑芥暟 瀹氫箟 鍦?
   arch/x86/鍐呮牳/microcode_鏍稿績.c helps 鍦?discovering the 绫诲瀷 鐨?the CPU
   鍜?thereby 鍦?applying the correct microcode revision 鍒?瀹?
   浣?娉ㄦ剰 璇?the 鍐呮牳 鎵ц 涓?maintain 涓€涓?閫氱敤 microcode image 鐢ㄤ簬 the
   鍏ㄩ儴 CPUs, 涓轰簡 handle case 'b' 鎻忚堪 涓嬫枃.


b. 褰?涓€浜?鐨?the CPUs 鏄?涓嶅悓 姣?the rest:

   鍦?姝?case since 鎴戜滑 probably 闇€瑕?鍒?apply 涓嶅悓 microcode revisions
   鍒?涓嶅悓 CPUs, the 鍐呮牳 maintains 涓€涓?copy 鐨?the correct microcode
   image 鐢ㄤ簬 姣忎釜 CPU (涔嬪悗 appropriate CPU 绫诲瀷/鍨嬪彿 discovery 浣跨敤
   鍑芥暟 渚嬪 collect_CPU_info()).


c. 褰?涓€涓?CPU 鏄?physically hot-unplugged 鍜?涓€涓?鏂?(鍜?possibly 涓嶅悓
   绫诲瀷 鐨? CPU 鏄?hot-plugged 杩涘叆 the 绯荤粺:

   鍦?the 鐢垫祦 design 鐨?the 鍐呮牳, whenever 涓€涓?CPU 鏄?taken offline 鏈熼棿
   涓€涓?regular CPU hotplug 鎿嶄綔, upon receiving the CPU_DEAD notification
   (鍏?鏄?sent 鐢?the CPU hotplug code), the microcode 鏇存柊 椹卞姩's
   鍥炶皟鍑芥暟 鐢ㄤ簬 璇?浜嬩欢 reacts 鐢?freeing the 鍐呮牳's copy 鐨?the
   microcode image 鐢ㄤ簬 璇?CPU.

   Hence, 褰?涓€涓?鏂?CPU 鏄?brought online, since the 鍐呮牳 finds 璇?瀹?
   doesn't 鍏锋湁 the microcode image, 瀹?鎵ц the CPU 绫诲瀷/鍨嬪彿 discovery
   afresh 鍜?鐒跺悗 requests the userspace 鐢ㄤ簬 the appropriate microcode image
   鐢ㄤ簬 璇?CPU, 鍏?鏄?subsequently applied.

   渚嬪, 鍦?x86, the mc_CPU_鍥炶皟鍑芥暟() 鍑芥暟 (鍏?鏄?the microcode
   鏇存柊 椹卞姩's 鍥炶皟鍑芥暟 registered 鐢ㄤ簬 CPU hotplug 浜嬩欢) calls
   microcode_鏇存柊_CPU() 鍏?灏嗕細 call microcode_鍒濆鍖朹CPU() 鍦?姝?case,
   鑰岄潪 microcode_resume_CPU() 褰?瀹?finds 璇?the 鍐呮牳 doesn't
   鍏锋湁 涓€涓?valid microcode image. 姝?ensures 璇?the CPU 绫诲瀷/鍨嬪彿
   discovery 鏄?performed 鍜?the right microcode 鏄?applied 鍒?the CPU 涔嬪悗
   getting 瀹?鏉ヨ嚜 userspace.


d. Handling microcode 鏇存柊 鏈熼棿 suspend/hibernate:

   Strictly speaking, 鏈熼棿 涓€涓?CPU hotplug 鎿嶄綔 鍏?鎵ц 涓?involve
   physically removing 鎴?inserting CPUs, the CPUs 鏄?涓?actually powered
   off 鏈熼棿 涓€涓?CPU offline. 瀹冧滑鏄?just put 鍒?the lowest C-states 鍙兘.
   Hence, 鍦?姝ょ被 涓€涓?case, 瀹冩槸 涓?really 蹇呰 鍒?re-apply microcode
   褰?the CPUs 鏄?brought back online, since 瀹冧滑 wouldn't 鍏锋湁 lost the
   image 鏈熼棿 the CPU offline 鎿嶄綔.

   杩欐槸 the usual scenario encountered 鏈熼棿 涓€涓?resume 涔嬪悗 涓€涓?suspend.
   鐒惰€? 鍦ㄨ鎯呭喌涓?鐨?hibernation, since 鍏ㄩ儴 the CPUs 鏄?completely
   powered off, 鏈熼棿 restore 瀹?becomes 蹇呰 鍒?apply the microcode
   images 鍒?鍏ㄩ儴 the CPUs.

   [娉ㄦ剰 璇?鎴戜滑 don't expect someone 鍒?physically pull out nodes 鍜?insert
   nodes 涓?涓€涓?涓嶅悓 绫诲瀷 鐨?CPUs in-between 涓€涓?suspend-resume 鎴?涓€涓?
   hibernate/restore cycle.]

   鍦?the 鐢垫祦 design 鐨?the 鍐呮牳 鐒惰€? 鏈熼棿 涓€涓?CPU offline 鎿嶄綔
   浣滀负 part 鐨?the suspend/hibernate cycle (cpuhp_tasks_frozen 鏄?set),
   the existing copy 鐨?microcode image 鍦?the 鍐呮牳 鏄?涓?freed up.
   鍜?鏈熼棿 the CPU online 鎿嶄綔 (鏈熼棿 resume/restore), since the
   鍐呮牳 finds 璇?瀹?宸茬粡 鍏锋湁 copies 鐨?the microcode images 鐢ㄤ簬 鍏ㄩ儴 the
   CPUs, 瀹?just applies them 鍒?the CPUs, avoiding 浠讳綍 re-discovery 鐨?CPU
   绫诲瀷/鍨嬪彿 鍜?the 闇€瑕?鐢ㄤ簬 validating 鏄惁 the microcode revisions 鏄?
   right 鐢ㄤ簬 the CPUs 鎴?涓?(鐢变簬 the 涓婃枃 assumption 璇?鐗╃悊 CPU
   hotplug 灏?涓?涓?宸插畬鎴?in-between suspend/resume 鎴?hibernate/restore
   cycles).


## III. Known problems


鏄?閭ｉ噷 浠讳綍 known problems 褰?regular CPU hotplug 鍜?suspend race
涓?姣忎釜 鍏朵粬.

Yes, 瀹冧滑鏄?listed 涓嬫枃:

1. 褰?invoking regular CPU hotplug, the 'tasks_frozen' 鍙傛暟 passed 鍒?
   the _CPU_down() 鍜?_CPU_up() 鍑芥暟 鏄?**濮嬬粓** 0.
   姝?鍙兘 涓?reflect the true 鐢垫祦 鐘舵€?鐨?the 绯荤粺, since the
   tasks 鍙互 鍏锋湁 宸茬粡 frozen 鐢?涓€涓?out-of-band 浜嬩欢 渚嬪 涓€涓?suspend
   鎿嶄綔 鍦?progress. Hence, the cpuhp_tasks_frozen variable 灏?涓?
   reflect the frozen 鐘舵€?鍜?the CPU hotplug callbacks 鍏?evaluate
   璇?variable 鍙兘 execute the wrong code path.

2. 鑻?涓€涓?regular CPU hotplug stress test happens 鍒?race 涓?the freezer due
   鍒?涓€涓?suspend 鎿嶄綔 鍦?progress 鍚屾椂, 鐒跺悗 鎴戜滑 鍙互 hit the
   situation 鎻忚堪 涓嬫枃:

    - 涓€涓?regular CPU online 鎿嶄綔 continues 鍏?journey 鏉ヨ嚜 userspace
      杩涘叆 the 鍐呮牳, since the freezing 鍏锋湁 涓?灏氭湭 begun.
    - 鐒跺悗 freezer gets 鍒?work 鍜?freezes userspace.
    - 鑻?CPU online 鍏锋湁 涓?灏氭湭 completed the microcode 鏇存柊 stuff 鐢?鐜板湪,
      瀹?灏?鐜板湪 鍚姩 waiting 鍦?the frozen userspace 鍦?the
      TASK_UNINTERRUPTIBLE 鐘舵€? 涓轰簡 get the microcode image.
    - 鐜板湪 the freezer continues 鍜?tries 鍒?freeze the remaining tasks. 浣?
      鐢变簬 姝?wait mentioned 涓婃枃, the freezer won't 涓?able 鍒?freeze
      the CPU online hotplug task 鍜?hence freezing 鐨?tasks fails.

   鍥犳 鐨?姝?task freezing failure, the suspend 鎿嶄綔 gets
   aborted.
