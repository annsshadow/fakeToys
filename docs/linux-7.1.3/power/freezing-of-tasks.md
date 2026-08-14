锘?# Freezing 鐨?tasks


(C) 2007 Rafael J. Wysocki <rjw@sisk.pl>, GPL

## I. 浠€涔?鏄?the freezing 鐨?tasks?


The freezing 鐨?tasks 鏄?涓€涓?mechanism 鐢?鍏?鐢ㄦ埛绌洪棿 杩涚▼ 鍜?涓€浜?
鍐呮牳 绾跨▼ 鏄?controlled 鏈熼棿 hibernation 鎴?system-wide suspend (鍦?涓€浜?
architectures).

## II. 濡備綍 鎵ц 瀹?work?


瀛樺湪 one per-task 鏍囧織 (PF_NOFREEZE) 鍜?three per-task states
(TASK_FROZEN, TASK_FREEZABLE 鍜?__TASK_FREEZABLE_UNSAFE) 浣跨敤 鐢ㄤ簬 璇?
The tasks 璇?鍏锋湁 PF_NOFREEZE unset (鍏ㄩ儴 鐢ㄦ埛绌洪棿 tasks 鍜?涓€浜?鍐呮牳
绾跨▼) 鏄?regarded 浣滀负 'freezable' 鍜?treated 鍦?涓€涓?鐗规畩 way 涔嬪墠 the
绯荤粺 enters 涓€涓?sleep 鐘舵€?浠ュ強 涔嬪墠 涓€涓?hibernation image 鏄?宸插垱寤?
(hibernation 鏄?directly covered 鐢?浠€涔?follows, 浣?the description applies
鍒?system-wide suspend too).

Namely, 浣滀负 the 绗竴 step 鐨?the hibernation procedure the 鍑芥暟
freeze_杩涚▼() (瀹氫箟 鍦?鍐呮牳/鐢垫簮/杩涚▼.c) 鏄?called.  涓€涓?system-wide
闈欐€?key freezer_active (鐩稿浜?涓€涓?per-task 鏍囧織 鎴?鐘舵€? 鏄?浣跨敤 鍒?
indicate 鏄惁 the 绯荤粺 鏄?鍒?undergo 涓€涓?freezing 鎿嶄綔. 鍜?
freeze_杩涚▼() sets 姝?闈欐€?key.  涔嬪悗 姝? 瀹?executes
try_鍒癬freeze_tasks() 璇?sends 涓€涓?fake 淇″彿 鍒?鍏ㄩ儴 鐢ㄦ埛绌洪棿 杩涚▼, 鍜?
wakes up 鍏ㄩ儴 the 鍐呮牳 绾跨▼. 鍏ㄩ儴 freezable tasks 蹇呴』 react 鍒?璇?鐢?
calling try_鍒癬freeze(), 鍏?results 鍦?涓€涓?call 鍒?__refrigerator() (瀹氫箟
鍦?鍐呮牳/freezer.c), 鍏?changes the task's 鐘舵€?鍒?TASK_FROZEN, 鍜?makes
瀹?loop 鐩村埌 瀹冩槸 woken 鐢?涓€涓?explicit TASK_FROZEN wakeup. 鐒跺悗, 璇?task
鏄?regarded 浣滀负 'frozen' 鍜?鍥犳 the set 鐨?鍑芥暟 handling 姝?mechanism 鏄?
referred 鍒?浣滀负 'the freezer' (杩欎簺 鍑芥暟 鏄?瀹氫箟 鍦?
鍐呮牳/鐢垫簮/杩涚▼.c, 鍐呮牳/freezer.c & 鍖呭惈/linux/freezer.h). 鐢ㄦ埛绌洪棿
tasks 鏄?generally frozen 涔嬪墠 鍐呮牳 绾跨▼.

__refrigerator() 蹇呴』 涓?涓?called directly.  鏀逛负, 浣跨敤 the
try_鍒癬freeze() 鍑芥暟 (瀹氫箟 鍦?鍖呭惈/linux/freezer.h), 璇?checks
鑻?the task 鏄?鍒?涓?frozen 鍜?makes the task enter __refrigerator().

鐢ㄤ簬 鐢ㄦ埛绌洪棿 杩涚▼ try_鍒癬freeze() 鏄?called automatically 鏉ヨ嚜 the
signal-handling code, 浣?the freezable 鍐呮牳 绾跨▼ 闇€瑕?鍒?call 瀹?
explicitly 鍦?suitable places 鎴?浣跨敤 the wait_浜嬩欢_freezable() 鎴?
wait_浜嬩欢_freezable_瓒呮椂() macros (瀹氫箟 鍦?鍖呭惈/linux/wait.h)
璇?put the task 鍒?sleep (TASK_INTERRUPTIBLE) 鎴?freeze 瀹?(TASK_FROZEN) 鑻?
freezer_active 鏄?set. The 涓昏 loop 鐨?涓€涓?freezable 鍐呮牳 绾跨▼ 鍙?look
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
**(鏉ヨ嚜 mm/oom_kill.c**
: oom_reaper()).

鑻?涓€涓?freezable 鍐呮牳 绾跨▼ 鏄?涓?put 鍒?the frozen 鐘舵€?涔嬪悗 the freezer
鍏锋湁 initiated 涓€涓?freezing 鎿嶄綔, the freezing 鐨?tasks 灏?fail 鍜?the
entire system-wide transition 灏?涓?cancelled.  鐢ㄤ簬 姝?reason, freezable
鍐呮牳 绾跨▼ 蹇呴』 call try_鍒癬freeze() somewhere 鎴?浣跨敤 one 鐨?the
wait_浜嬩欢_freezable() 鍜?wait_浜嬩欢_freezable_瓒呮椂() macros.

涔嬪悗 the 绯荤粺 鍐呭瓨 鐘舵€?鍏锋湁 宸茬粡 restored 鏉ヨ嚜 涓€涓?hibernation image 鍜?
璁惧 鍏锋湁 宸茬粡 reinitialized, the 鍑芥暟 thaw_杩涚▼() 鏄?called 鍦?
order 鍒?wake up 姣忎釜 frozen task.  鐒跺悗, the tasks 璇?鍏锋湁 宸茬粡 frozen leave
__refrigerator() 鍜?continue 杩愯涓?


### Rationale behind the 鍑芥暟 dealing 涓?freezing 鍜?thawing 鐨?tasks


freeze_杩涚▼():
  - freezes 浠?userspace tasks

freeze_鍐呮牳_绾跨▼():
  - freezes 鍏ㄩ儴 tasks (including 鍐呮牳 绾跨▼) 鍥犱负 鎴戜滑鍙互't freeze
    鍐呮牳 绾跨▼ 鏃?freezing userspace tasks

thaw_鍐呮牳_绾跨▼():
  - thaws 浠?鍐呮牳 绾跨▼; 杩欐槸 particularly useful 鑻?鎴戜滑 闇€瑕?鍒?鎵ц
    anything 鐗规畩 鍦?涔嬮棿 thawing 鐨?鍐呮牳 绾跨▼ 鍜?thawing 鐨?
    userspace tasks, 鎴?鑻?鎴戜滑 甯屾湜 鍒?postpone the thawing 鐨?userspace tasks

thaw_杩涚▼():
  - thaws 鍏ㄩ儴 tasks (including 鍐呮牳 绾跨▼) 鍥犱负 鎴戜滑鍙互't thaw userspace
    tasks 鏃?thawing 鍐呮牳 绾跨▼


## III. 鍏?鍐呮牳 绾跨▼ 鏄?freezable?


鍐呮牳 绾跨▼ 鏄?涓?freezable 榛樿鎯呭喌涓?  鐒惰€? 涓€涓?鍐呮牳 绾跨▼ 鍙?clear
PF_NOFREEZE 鐢ㄤ簬 itself 鐢?calling set_freezable() (the resetting 鐨?PF_NOFREEZE
directly 鏄?涓?allowed).  鏉ヨ嚜 姝?point 瀹冩槸 regarded 浣滀负 freezable
鍜?蹇呴』 call try_鍒癬freeze() 鎴?variants 鐨?wait_浜嬩欢_freezable() 鍦?涓€涓?
suitable place.

## IV. 涓轰綍 鎵ц 鎴戜滑 鎵ц 璇?


Generally speaking, 瀛樺湪 涓€涓?couple 鐨?reasons 鍒?浣跨敤 the freezing 鐨?tasks:

1. The principal reason 鏄?鍒?prevent 鏂囦欢绯荤粺 鏉ヨ嚜 姝ｅ湪 damaged 涔嬪悗
   hibernation.  鍦?the moment 鎴戜滑 鍏锋湁 鏃?绠€鍗?means 鐨?checkpointing
   鏂囦欢绯荤粺, 鍥犳 鑻?瀛樺湪 浠讳綍 modifications made 鍒?鏂囦欢绯荤粺 鏁版嵁 鍜?鎴?
   metadata 鍦?disks, 鎴戜滑 cannot bring them back 鍒?the 鐘舵€?鏉ヨ嚜 涔嬪墠 the
   modifications.  鍚屾椂 姣忎釜 hibernation image 鍖呭惈 涓€浜?
   filesystem-related information 璇?蹇呴』 涓?consistent 涓?the 鐘舵€?鐨?the
   on-disk 鏁版嵁 鍜?metadata 涔嬪悗 the 绯荤粺 鍐呭瓨 鐘舵€?鍏锋湁 宸茬粡 restored
   鏉ヨ嚜 the image (鍚﹀垯 the 鏂囦欢绯荤粺 灏?涓?damaged 鍦?涓€涓?nasty way,
   閫氬父 making them almost impossible 鍒?repair).  鎴戜滑 鍥犳 freeze
   tasks 璇?鍙兘 cause the on-disk 鏂囦欢绯荤粺' 鏁版嵁 鍜?metadata 鍒?涓?
   modified 涔嬪悗 the hibernation image 鍏锋湁 宸茬粡 宸插垱寤?鍜?涔嬪墠 the
   绯荤粺 鏄?finally powered off. The majority 鐨?杩欎簺 鏄?鐢ㄦ埛绌洪棿
   杩涚▼, 浣?鑻?浠讳綍 鐨?the 鍐呮牳 绾跨▼ 鍙?cause something 绫讳技 姝?
   鍒?happen, 瀹冧滑 鍏锋湁 鍒?涓?freezable.

2. 鎺ヤ笅鏉? 鍒?鍒涘缓 the hibernation image 鎴戜滑 闇€瑕?鍒?free 涓€涓?sufficient amount 鐨?
   鍐呭瓨 (approximately 50% 鐨?鍙敤 RAM) 鍜?鎴戜滑 闇€瑕?鍒?鎵ц 璇?涔嬪墠
   璁惧 鏄?deactivated, 鍥犱负 鎴戜滑 generally 闇€瑕?them 鐢ㄤ簬 swapping out.
   鐒跺悗, 涔嬪悗 the 鍐呭瓨 鐢ㄤ簬 the image 鍏锋湁 宸茬粡 freed, 鎴戜滑 don't 甯屾湜 tasks
   鍒?allocate 棰濆 鍐呭瓨 鍜?鎴戜滑 prevent them 鏉ヨ嚜 doing 璇?鐢?
   freezing them 鏇存棭. [鐨?course, 姝?涔?means 璇?璁惧 椹卞姩
   搴斿綋 涓?allocate substantial amounts 鐨?鍐呭瓨 鏉ヨ嚜 瀹冧滑鐨?.suspend()
   callbacks 涔嬪墠 hibernation, 浣?杩欐槸 涓€涓?separate issue.]

3. The third reason 鏄?鍒?prevent 鐢ㄦ埛绌洪棿 杩涚▼ 鍜?涓€浜?鍐呮牳 绾跨▼
   鏉ヨ嚜 interfering 涓?the suspending 鍜?resuming 鐨?璁惧.  涓€涓?鐢ㄦ埛绌洪棿
   杩涚▼ 杩愯涓?鍦?涓€涓?second CPU 鍚屾椂 鎴戜滑 鏄?suspending 璁惧 鍙? 鐢ㄤ簬
   绀轰緥, 涓?troublesome 鍜?鏃?the freezing 鐨?tasks 鎴戜滑 灏嗕細 闇€瑕?涓€浜?
   safeguards against race conditions 璇?鍙兘 occur 鍦?姝ょ被 涓€涓?case.

灏界 Linus Torvalds doesn't 绫讳技 the freezing 鐨?tasks, he said 姝?鍦?one
鐨?the discussions 鍦?LKML (https://lore.kernel.org/r/alpine.LFD.0.98.0704271801020.9964@woody.linux-foundation.org):

"RJW:> 涓轰綍 鎴戜滑 freeze tasks 鍦?鍏ㄩ儴 鎴?涓轰綍 鎴戜滑 freeze 鍐呮牳 绾跨▼搴?

Linus: 鍦?璁稿 ways, '鍦?鍏ㄩ儴'.

I **鎵ц** realize the IO 璇锋眰 闃熷垪 issues, 鍜?璇?鎴戜滑 cannot actually 鎵ц
s2ram 涓?涓€浜?璁惧 鍦?the middle 鐨?涓€涓?DMA.  鍥犳 鎴戜滑 甯屾湜 鍒?涓?able 鍒?
avoid **璇?*, 閭ｉ噷's 鏃?question 鍏充簬 璇?  鍜?I suspect 璇?stopping
鐢ㄦ埛 绾跨▼ 鍜?鐒跺悗 waiting 鐢ㄤ簬 涓€涓?sync 鏄?practically one 鐨?the easier
ways 鍒?鎵ц 鍥犳.

鍥犳 鍦?practice, the '鍦?鍏ㄩ儴' 鍙?become 涓€涓?'涓轰綍 freeze 鍐呮牳 绾跨▼搴? 鍜?
freezing 鐢ㄦ埛 绾跨▼ I don't find really objectionable."

浠嶇劧, 瀛樺湪 鍐呮牳 绾跨▼ 璇?鍙?甯屾湜 鍒?涓?freezable.  渚嬪, 鑻?
涓€涓?鍐呮牳 绾跨▼ 璇?belongs 鍒?涓€涓?璁惧 椹卞姩 accesses the 璁惧 directly, 瀹?
鍦?principle needs 鍒?know 褰?the 璁惧 鏄?suspended, 鍥犳 璇?瀹?doesn't try
鍒?access 瀹?鍦?璇?time.  鐒惰€? 鑻?the 鍐呮牳 绾跨▼ 鏄?freezable, 瀹?灏?
涓?frozen 涔嬪墠 the 椹卞姩's .suspend() 鍥炶皟鍑芥暟 鏄?executed 鍜?瀹?灏?涓?
thawed 涔嬪悗 the 椹卞姩's .resume() 鍥炶皟鍑芥暟 鍏锋湁 杩愯, 鍥犳 瀹?won't 涓?accessing
the 璁惧 鍚屾椂 瀹?s suspended.

4. Another reason 鐢ㄤ簬 freezing tasks 鏄?鍒?prevent 鐢ㄦ埛绌洪棿 杩涚▼ 鏉ヨ嚜
   realizing 璇?hibernation (鎴?suspend) 鎿嶄綔 takes place.  Ideally, 鐢ㄦ埛
   space 杩涚▼ 搴斿綋 涓?notice 璇?姝ょ被 涓€涓?system-wide 鎿嶄綔 鍏锋湁
   occurred 鍜?搴斿綋 continue 杩愯涓?鏃?浠讳綍 problems 涔嬪悗 the restore
   (鎴?resume 鏉ヨ嚜 suspend).  Unfortunately, 鍦?the 澶у鏁?閫氱敤 case 姝?
   鏄?quite difficult 鍒?achieve 鏃?the freezing 鐨?tasks.  Consider,
   渚嬪, 涓€涓?杩涚▼ 璇?depends 鍦?鍏ㄩ儴 CPUs 姝ｅ湪 online 鍚屾椂 瀹?s
   杩愯涓?  Since 鎴戜滑 闇€瑕?鍒?绂佺敤 nonboot CPUs 鏈熼棿 the hibernation,
   鑻?姝?杩涚▼ 鏄?涓?frozen, 瀹?鍙?notice 璇?the 鏁板瓧 鐨?CPUs 鍏锋湁
   changed 鍜?鍙?鍚姩 鍒?work incorrectly 鍥犱负 鐨?璇?

## V. 鏄?閭ｉ噷 浠讳綍 problems related 鍒?the freezing 鐨?tasks?


Yes, 瀛樺湪.

绗竴 鐨?鍏ㄩ儴, the freezing 鐨?鍐呮牳 绾跨▼ 鍙?涓?tricky 鑻?瀹冧滑 depend one
鍦?another.  渚嬪, 鑻?鍐呮牳 绾跨▼ 涓€涓?waits 鐢ㄤ簬 涓€涓?completion (鍦?the
TASK_UNINTERRUPTIBLE 鐘舵€? 璇?needs 鍒?涓?宸插畬鎴?鐢?freezable 鍐呮牳 绾跨▼ B
鍜?B 鏄?frozen 鍦?the meantime, 鐒跺悗 涓€涓?灏?涓?blocked 鐩村埌 B 鏄?thawed, 鍏?
鍙?涓?undesirable.  璇?s 涓轰綍 鍐呮牳 绾跨▼ 鏄?涓?freezable 榛樿鎯呭喌涓?

Second, 瀛樺湪 the 浠ヤ笅 two problems related 鍒?the freezing 鐨?鐢ㄦ埛
space 杩涚▼:

1. Putting 杩涚▼ 杩涘叆 涓€涓?uninterruptible sleep distorts the 鍔犺浇 average.
2. 鐜板湪 璇?鎴戜滑 鍏锋湁 FUSE, 澧炲己鐗?the framework 鐢ㄤ簬 doing 璁惧 椹卞姩 鍦?
   userspace, 瀹?gets even 鏇村 complicated 鍥犱负 涓€浜?userspace 杩涚▼ 鏄?
   鐜板湪 doing the sorts 鐨?things 璇?鍐呮牳 绾跨▼ 鎵ц
   (https://lists.linux-foundation.org/pipermail/linux-pm/2007-May/012309.html).

The problem 1. seems 鍒?涓?fixable, 灏界 瀹?hasn't 宸茬粡 fixed 鍥犳 far.  The
鍏朵粬 one 鏄?鏇村 serious, 浣?瀹?seems 璇?鎴戜滑鍙互 work around 瀹?鐢?浣跨敤
hibernation (鍜?suspend) notifiers (鍦?璇?case, though, 鎴戜滑 won't 涓?able 鍒?
avoid the realization 鐢?the 鐢ㄦ埛绌洪棿 杩涚▼ 璇?the hibernation 鏄?taking
place).

瀛樺湪 涔?problems 璇?the freezing 鐨?tasks tends 鍒?expose, 灏界
瀹冧滑鏄?涓?directly related 鍒?瀹?  渚嬪, 鑻?璇锋眰_鍥轰欢() 鏄?
called 鏉ヨ嚜 涓€涓?璁惧 椹卞姩's .resume() routine, 瀹?灏?瓒呮椂 鍜?eventually
fail, 鍥犱负 the 鐢ㄦ埛 land 杩涚▼ 璇?搴斿綋 respond 鍒?the 璇锋眰 鏄?frozen
鍦?姝?point.  鍥犳, seemingly, the failure 鏄?鐢变簬 the freezing 鐨?tasks.
Suppose, 鐒惰€? 璇?the 鍥轰欢 鏂囦欢 鏄?located 鍦?涓€涓?鏂囦欢绯荤粺 accessible
浠?through another 璁惧 璇?hasn't 宸茬粡 resumed 灏氭湭.  鍦?璇?case,
璇锋眰_鍥轰欢() 灏?fail regardless 鐨?鏄惁 鎴?涓?the freezing 鐨?tasks
鏄?浣跨敤.  Consequently, the problem 鏄?涓?really related 鍒?the freezing 鐨?
tasks, since 瀹?generally exists anyway.

涓€涓?椹卞姩 蹇呴』 鍏锋湁 鍏ㄩ儴 firmwares 瀹?鍙?闇€瑕?鍦?RAM 涔嬪墠 suspend() 鏄?called.
鑻?keeping them 鏄?涓?practical, 渚嬪 鐢变簬 瀹冧滑鐨?澶у皬, 瀹冧滑 蹇呴』 涓?
requested early enough 浣跨敤 the suspend notifier API 鎻忚堪 鍦?
Documentation/driver-api/pm/notifiers.rst.

## VI. 鏄?閭ｉ噷 浠讳綍 precautions 鍒?涓?taken 鍒?prevent freezing failures?


Yes, 瀛樺湪.

绗竴 鐨?鍏ㄩ儴, grabbing the '绯荤粺_transition_浜掓枼浣? 閿?鍒?mutually exclude 涓€涓?
piece 鐨?code 鏉ヨ嚜 system-wide sleep 渚嬪 suspend/hibernation 鏄?涓?
encouraged.  鑻?鍙兘, 璇?piece 鐨?code 蹇呴』 鏀逛负 hook onto the
suspend/hibernation notifiers 鍒?achieve mutual exclusion. Look 鍦?the
CPU-Hotplug code (鍐呮牳/CPU.c) 鐢ㄤ簬 涓€涓?绀轰緥.

鐒惰€? 鑻?鍗?涓?feasible, 鍜?grabbing '绯荤粺_transition_浜掓枼浣? 鏄?
deemed 蹇呰, 瀹冩槸 strongly discouraged 鍒?directly call
浜掓枼浣揰[un]閿?&绯荤粺_transition_浜掓枼浣? since 璇?鍙互 lead 鍒?freezing
failures, 鍥犱负 鑻?the suspend/hibernate code successfully acquired the
'绯荤粺_transition_浜掓枼浣? 閿? 鍜?hence 璇?鍏朵粬 entity failed 鍒?acquire
the 閿? 鐒跺悗 璇?task 灏嗕細 get blocked 鍦?TASK_UNINTERRUPTIBLE 鐘舵€? 浣滀负 涓€涓?
consequence, the freezer 灏嗕細 涓?涓?able 鍒?freeze 璇?task, leading 鍒?
freezing failure.

鐒惰€? the [un]閿乢绯荤粺_sleep() APIs 鏄?safe 鍒?浣跨敤 鍦?姝?scenario,
since 瀹冧滑 ask the freezer 鍒?skip freezing 姝?task, since 瀹冩槸 anyway
"frozen enough" 浣滀负 瀹冩槸 blocked 鍦?'绯荤粺_transition_浜掓枼浣?, 鍏?灏?涓?
released 浠?涔嬪悗 the entire suspend/hibernation sequence 鏄?complete.  鍥犳, 鍒?
summarize, 浣跨敤 [un]閿乢绯荤粺_sleep() 鑰岄潪 directly 浣跨敤
浜掓枼浣揰[un]閿?&绯荤粺_transition_浜掓枼浣?. 璇?灏嗕細 prevent freezing failures.

## V. Miscellaneous


/sys/鐢垫簮/pm_freeze_瓒呮椂 controls 濡備綍 long 瀹?灏?cost 鑷冲 鍒?freeze
鍏ㄩ儴 鐢ㄦ埛绌洪棿 杩涚▼ 鎴?鍏ㄩ儴 freezable 鍐呮牳 绾跨▼, 鍦?unit 鐨?
millisecond.  The 榛樿 鍊?鏄?20000, 涓?range 鐨?unsigned integer.
