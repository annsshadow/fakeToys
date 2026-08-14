
## tip 鏍戞墜鍐?
### 浠€涔堟槸 tip 鏍戯紵

tip 鏍戯紙tip tree锛夋槸鑻ュ共瀛愮郴缁熶笌寮€鍙戦鍩熺殑闆嗗悎銆倀ip 鏍戞棦鏄竴涓洿鎺ョ殑寮€鍙戞爲锛?涔熸槸鑻ュ共瀛愮淮鎶よ€咃紙sub-maintainer锛夋爲鐨勮仛鍚堟爲銆倀ip 鏍戠殑 gitweb URL 涓猴細
https://git.kernel.org/pub/scm/linux/kernel/git/tip/tip.git

tip 鏍戝寘鍚互涓嬪瓙绯荤粺锛?
   - **x86 鏋舵瀯**

     x86 鏋舵瀯鐨勫紑鍙戝湪 tip 鏍戜腑杩涜锛屼絾 x86 KVM 鍜?XEN 鐨勭壒瀹氶儴鍒嗛櫎澶栵紝瀹冧滑鐢?     鐩稿簲鐨勫瓙绯荤粺缁存姢锛屽苟鐩存帴浠庨偅閲屽悎骞跺埌涓荤嚎锛坢ainline锛夈€傚湪 x86 鐗瑰畾鐨?KVM 鍜?     XEN 琛ヤ竵涓?Cc x86 缁存姢鑰呬粛鐒舵槸濂戒範鎯€?
     闄や簡鏁翠綋鐨?x86 缁存姢鑰呭锛屼竴浜?x86 瀛愮郴缁熻繕鏈夊畠浠嚜宸辩殑缁存姢鑰呫€傚嵆浣?MAINTAINER
     鏂囦欢娌℃湁鐗瑰埆鐐瑰悕锛屽湪淇敼 arch/x86 涓嬫枃浠剁殑琛ヤ竵涓婁篃璇?Cc 鏁翠綋 x86 缁存姢鑰呫€?
     娉ㄦ剰锛宍x86@kernel.org` 骞朵笉鏄竴涓偖浠跺垪琛ㄣ€傚畠浠呬粎鏄竴涓偖浠跺埆鍚嶏紝灏嗛偖浠跺垎鍙?     缁?x86 椤跺眰缁存姢鑰呭洟闃熴€傝濮嬬粓 Cc Linux 鍐呮牳閭欢鍒楄〃锛圠KML锛?     `linux-kernel@vger.kernel.org`锛屽惁鍒欎綘鐨勯偖浠跺彧浼氳繘鍏ョ淮鎶よ€呯殑绉佷汉鏀朵欢绠便€?
   - **璋冨害鍣紙Scheduler锛?*

     璋冨害鍣ㄥ紑鍙戝湪 -tip 鏍戜腑杩涜锛屼綅浜?sched/core 鍒嗘敮鈥斺€斿伓灏斾細鏈夌敤浜庤繘琛屼腑琛ヤ竵闆嗙殑
     瀛愪富棰樻爲銆?
   - **閿侊紙Locking锛変笌鍘熷瓙鎿嶄綔锛坅tomics锛?*

     閿佺殑寮€鍙戯紙鍖呮嫭鍘熷瓙鎿嶄綔浠ュ強涓庨攣鐩稿叧鐨勫叾瀹冨悓姝ュ師璇級鍦?-tip 鏍戜腑杩涜锛屼綅浜?     locking/core 鍒嗘敮鈥斺€斿伓灏斾細鏈夌敤浜庤繘琛屼腑琛ヤ竵闆嗙殑瀛愪富棰樻爲銆?
   - **閫氱敤涓柇瀛愮郴缁熶笌涓柇鑺墖椹卞姩**锛?
     - 涓柇鏍稿績寮€鍙戝彂鐢熷湪 irq/core 鍒嗘敮

     - 涓柇鑺墖椹卞姩寮€鍙戜篃鍙戠敓鍦?irq/core 鍒嗘敮锛屼絾琛ヤ竵閫氬父鍏堝簲鐢ㄥ埌鍗曠嫭鐨勭淮鎶よ€呮爲锛?       鐒跺悗鍐嶈仛鍚堝埌 irq/core

   - **鏃堕棿銆佸畾鏃跺櫒銆佹椂闂翠繚鎸侊紙timekeeping锛夈€丯OHZ 浠ュ強鐩稿叧鑺墖椹卞姩**锛?
     - 鏃堕棿淇濇寔銆乧locksource 鏍稿績銆丯TP 鍜?alarmtimer 鐨勫紑鍙戝彂鐢熷湪 timers/core 鍒嗘敮锛?       浣嗚ˉ涓侀€氬父鍏堝簲鐢ㄥ埌鍗曠嫭鐨勭淮鎶よ€呮爲锛岀劧鍚庡啀鑱氬悎鍒?timers/core

     - clocksource/event 椹卞姩寮€鍙戝彂鐢熷湪 timers/core 鍒嗘敮锛屼絾琛ヤ竵澶у鍏堝簲鐢ㄥ埌鍗曠嫭鐨?       缁存姢鑰呮爲锛岀劧鍚庡啀鑱氬悎鍒?timers/core

   - **鎬ц兘璁℃暟鍣紙Performance counters锛夋牳蹇冦€佹灦鏋勬敮鎸佷互鍙婂伐鍏凤紙tooling锛?*锛?
     - perf 鏍稿績鍜屾灦鏋勬敮鎸佸紑鍙戝彂鐢熷湪 perf/core 鍒嗘敮

     - perf 宸ュ叿寮€鍙戝彂鐢熷湪 perf tools 缁存姢鑰呮爲锛屽苟鑱氬悎鍒?tip 鏍戙€?
   - **CPU 鐑彃鎷旓紙CPU hotplug锛夋牳蹇?*

   - **RAS 鏍稿績**

     澶ч儴鍒?x86 鐗瑰畾鐨?RAS 琛ヤ竵琚敹闆嗗湪 tip 鐨?ras/core 鍒嗘敮銆?
   - **EFI 鏍稿績**

     EFI 寮€鍙戝湪 efi git 鏍戜腑杩涜銆傛敹闆嗙殑琛ヤ竵琚仛鍚堝埌 tip 鐨?efi/core 鍒嗘敮銆?
   - **RCU**

     RCU 寮€鍙戝彂鐢熷湪 linux-rcu 鏍戙€備骇鐢熺殑鏀瑰姩琚仛鍚堝埌 tip 鐨?core/rcu 鍒嗘敮銆?
   - **鍚勭鏍稿績浠ｇ爜缁勪欢**锛?
       - debugobjects

       - objtool

       - 闆舵暎鐨勯浂纰庝唬鐮?
### 琛ヤ竵鎻愪氦璇存槑

##### 閫夋嫨鏍?鍒嗘敮

涓€鑸潵璇达紝閽堝 tip 鏍?master 鍒嗘敮鐨勫ご閮ㄨ繘琛屽紑鍙戞槸鍙互鐨勶紝浣嗗浜庡崟鐙淮鎶ゃ€佹嫢鏈?鑷繁鐨?git 鏍戝苟涓斿彧鏄仛鍚堝埌 tip 鏍戠殑閭ｄ簺瀛愮郴缁燂紝寮€鍙戝簲璇ラ拡瀵圭浉鍏崇殑瀛愮郴缁熸爲鎴?鍒嗘敮杩涜銆?
閽堝涓荤嚎鐨勭己闄蜂慨澶嶏紙bug fix锛夊簲璇ュ缁堝彲浠ュ簲鐢ㄥ埌涓荤嚎鍐呮牳鏍戜笂銆備笌宸茬粡鎺掗槦鍦?tip
鏍戜腑鐨勬敼鍔ㄤ箣闂存綔鍦ㄧ殑鍐茬獊鐢辩淮鎶よ€呭鐞嗐€?
##### 琛ヤ竵涓婚锛坰ubject锛?
tip 鏍戝亸濂界殑琛ヤ竵涓婚鍓嶇紑鏍煎紡鏄€渟ubsys/component:鈥濓紝渚嬪鈥渪86/apic:鈥濄€佲€渪86/mm/fault:鈥濄€?鈥渟ched/fair:鈥濄€佲€済enirq/core:鈥濄€傝涓嶈浣跨敤鏂囦欢鍚嶆垨瀹屾暣鏂囦欢璺緞浣滀负鍓嶇紑銆傗€済it log
path/to/file鈥?鍦ㄥ鏁版儏鍐典笅搴旇鑳界粰浣犱竴涓悎鐞嗙殑鎻愮ず銆?
涓婚琛屼腑鍑濈粌鐨勮ˉ涓佹弿杩板簲璇ヤ互澶у啓瀛楁瘝寮€澶达紝骞朵笖浣跨敤绁堜娇璇皵涔﹀啓銆?
##### 鍙樻洿鏃ュ織锛圕hangelog锛?
:ref:`鎻愪氦琛ヤ竵鎸囧崡 <describe_changes>` 涓叧浜庡彉鏇存棩蹇楃殑涓€鑸鍒欏悓鏍烽€傜敤銆?
tip 鏍戠淮鎶よ€呴潪甯搁噸瑙嗛伒寰繖浜涜鍒欙紝灏ゅ叾鏄姹備互绁堜娇璇皵涔﹀啓鍙樻洿鏃ュ織銆佷笉瑕佷互浠ｇ爜
鎴栧叾鎵ц鐨勫彛鍚绘潵鍙欒堪銆傝繖骞堕潪缁存姢鑰呯殑涓€鏃跺叴璧枫€傜敤鎶借薄鎺緸鍐欐垚鐨勫彉鏇存棩蹇楁瘮灏忚
褰㈠紡鐨勫彉鏇存棩蹇楁洿绮剧‘銆佷篃鏇翠笉瀹规槗寮曡捣娣锋穯銆?
鎶婂彉鏇存棩蹇楃粍缁囨垚鑻ュ共娈佃惤銆佽€屼笉鏄妸鎵€鏈夊唴瀹瑰爢鍦ㄤ竴涓钀介噷涔熷緢鏈夌敤銆備竴涓ソ鐨勭粨鏋勬槸
鎸夆€滆儗鏅€侀棶棰樸€佽В鍐虫柟妗堚€濈殑椤哄簭锛岀敤鐙珛鐨勬钀藉垎鍒В閲娿€?
绀轰緥璇存槑锛?
```

    x86/intel_rdt/mbm: Fix MBM overflow handler during hot cpu

    When a CPU is dying, we cancel the worker and schedule a new worker on a
    different CPU on the same domain. But if the timer is already about to
    expire (say 0.99s) then we essentially double the interval.

    We modify the hot cpu handling to cancel the delayed work on the dying
    cpu and run the worker immediately on a different cpu in same domain. We
    do not flush the worker because the MBM overflow worker reschedules the
    worker on same CPU and scans the domain->cpu_mask to get the domain
    pointer.

  Improved version::

    x86/intel_rdt/mbm: Fix MBM overflow handler during CPU hotplug

    When a CPU is dying, the overflow worker is canceled and rescheduled on a
    different CPU in the same domain. But if the timer is already about to
    expire this essentially doubles the interval which might result in a non
    detected overflow.

    Cancel the overflow worker and reschedule it immediately on a different CPU
    in the same domain. The work could be flushed as well, but that would
    reschedule it on the same CPU.

  Example 2::

    time: POSIX CPU timers: Ensure that variable is initialized

    If cpu_timer_sample_group returns -EINVAL, it will not have written into
    *sample. Checking for cpu_timer_sample_group's return value precludes the
    potential use of an uninitialized value of now in the following block.
    Given an invalid clock_idx, the previous code could otherwise overwrite
    *oldval in an undefined manner. This is now prevented. We also exploit
    short-circuiting of && to sample the timer only if the result will
    actually be used to update *oldval.

  Improved version::

    posix-cpu-timers: Make set_process_cpu_timer() more robust

    Because the return value of cpu_timer_sample_group() is not checked,
    compilers and static checkers can legitimately warn about a potential use
    of the uninitialized variable 'now'. This is not a runtime issue as all
    call sites hand in valid clock ids.

    Also cpu_timer_sample_group() is invoked unconditionally even when the
    result is not used because *oldval is NULL.

    Make the invocation conditional and check the return value.

  Example 3::

    The entity can also be used for other purposes.

    Let's rename it to be more generic.

  Improved version::

    The entity can also be used for other purposes.

    Rename it to be more generic.


```

瀵逛簬澶嶆潅鍦烘櫙锛屽挨鍏舵槸绔炴€佹潯浠讹紙race condition锛夊拰鍐呭瓨鎺掑簭锛坢emory ordering锛夐棶棰橈紝
鐢ㄤ竴寮犺〃鏉ユ弿缁樺満鏅緢鏈変环鍊硷紝渚嬪锛?
```

    CPU0                            CPU1
    free_irq(X)                     interrupt X
                                    spin_lock(desc->lock)
                                    wake irq thread()
                                    spin_unlock(desc->lock)
    spin_lock(desc->lock)
    remove action()
    shutdown_irq()
    release_resources()             thread_handler()
    spin_unlock(desc->lock)           access released resources.
                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
    synchronize_irq()

```

Lockdep 鎻愪緵浜嗙被浼肩殑鏈夊姪浜庢弿缁樺彲鑳芥閿佺殑杈撳嚭锛?
```

    CPU0                                    CPU1
    rtmutex_lock(&rcu->rt_mutex)
      spin_lock(&rcu->rt_mutex.wait_lock)
                                            local_irq_disable()
                                            spin_lock(&timer->it_lock)
                                            spin_lock(&rcu->mutex.wait_lock)
    --> Interrupt
        spin_lock(&timer->it_lock)


```

##### 鍙樻洿鏃ュ織涓殑鍑芥暟寮曠敤

褰撳彉鏇存棩蹇椾腑鎻愬埌涓€涓嚱鏁版椂锛堟棤璁烘槸鍦ㄦ鏂囪繕鏄富棰樿涓級锛岃浣跨敤鈥渇unction_name()鈥?鏍煎紡銆傜渷鐣モ€?)鈥濇槸閿欒鐨勶紝渚嬪锛?
```

  Subject: subsys/component: Make reservation_count static

  reservation_count is only used in reservation_stats. Make it static.

```

```

  Subject: subsys/component: Make reservation_count() static

  reservation_count() is only called from reservation_stats(). Make it
  static.


```

##### 鍙樻洿鏃ュ織涓殑鍥炴函锛坆acktrace锛?
鍙傝 backtraces銆?
##### 鎻愪氦鏍囩锛坈ommit tag锛夌殑椤哄簭

涓轰簡缁熶竴鏌ョ湅鎻愪氦鏍囩锛宼ip 缁存姢鑰呬娇鐢ㄤ互涓嬫爣绛炬帓搴忔柟妗堬細

 - Fixes: 12+瀛楃-SHA1锛堚€渟ub/sys: 鍘熷涓婚琛屸€濓級

   鍗充娇瀵逛簬涓嶉渶瑕佸洖绉绘锛坆ackport锛夊埌绋冲畾锛坰table锛夊唴鏍哥殑鏀瑰姩锛屼篃搴旀坊鍔?Fixes 鏍囩锛?   鍗冲綋澶勭悊涓€涓渶杩戝紩鍏ョ殑銆佸彧褰卞搷 tip 鎴栦富绾垮綋鍓嶅ご閮ㄧ殑闂鏃躲€傝繖浜涙爣绛炬湁鍔╀簬璇嗗埆
   鍘熷鎻愪氦锛屽叾浠峰€艰繙楂樹簬鍦ㄥ彉鏇存棩蹇楁鏂囦腑閱掔洰鍦版彁鍙婂紩鍏ラ棶棰樼殑鎻愪氦锛屽洜涓哄畠浠彲浠?   琚嚜鍔ㄦ彁鍙栥€?
```

     Commit

       abcdef012345678 ("x86/xxx: Replace foo with bar")

     left an unused instance of variable foo around. Remove it.

     Signed-off-by: J.Dev <j.dev@mail>

   Please say instead::

     The recent replacement of foo with bar left an unused instance of
     variable foo around. Remove it.

     Fixes: abcdef012345678 ("x86/xxx: Replace foo with bar")
     Signed-off-by: J.Dev <j.dev@mail>

   The latter puts the information about the patch into the focus and
   amends it with the reference to the commit which introduced the issue
   rather than putting the focus on the original commit in the first place.

 - Reported-by: ``Reporter <reporter@mail>``

 - Closes: ``URL 鎴栨淇鎵€瀵瑰簲鐨勭己闄锋姤鍛婄殑 Message-ID``

 - Originally-by: ``Original author <original-author@mail>``

 - Suggested-by: ``Suggester <suggester@mail>``

 - Co-developed-by: ``Co-author <co-author@mail>``

   Signed-off-by: ``Co-author <co-author@mail>``

   娉ㄦ剰锛孋o-developed-by 涓庡悎钁楄€咃紙co-author锛夌殑 Signed-off-by 蹇呴』鎴愬鍑虹幇銆?
 - Signed-off-by: ``Author <author@mail>``

   鍦ㄦ渶鍚庝竴涓?Co-developed-by/SOB 瀵逛箣鍚庣殑绗竴涓?Signed-off-by锛圫OB锛夋槸浣滆€?SOB锛?   鍗宠 git 鏍囪涓轰綔鑰呯殑浜恒€?
 - Signed-off-by: ``Patch handler <handler@mail>``

   浣滆€?SOB 涔嬪悗鐨?SOB 鏉ヨ嚜澶勭悊鍜屼紶閫佽琛ヤ竵銆佷絾鏈弬涓庡紑鍙戠殑浜恒€係OB 閾惧簲鍙嶆槧琛ヤ竵
   浼犳挱鍒版垜浠繖閲屾墍缁忚繃鐨?*鐪熷疄**璺緞锛屽叾涓涓€涓?SOB 鏉＄洰琛ㄧず璇ヨˉ涓佸崟涓€鐨?   涓昏浣滆€呫€侫ck 搴斾互 Acked-by 琛岀粰鍑猴紝瀹￠槄鎵瑰噯搴斾互 Reviewed-by 琛岀粰鍑恒€?
   濡傛灉澶勭悊鑰咃紙handler锛夊琛ヤ竵鎴栧彉鏇存棩蹇楀仛浜嗕慨鏀癸紝閭ｄ箞搴旇鍦ㄥ彉鏇存棩蹇楁枃鏈?*涔嬪悗**銆?   鎵€鏈夋彁浜ゆ爣绛?*涔嬩笂**锛屼互涓嬪垪鏍煎紡鎻愬強::

     ... changelog text ends.

     [ handler: Replaced foo by bar and updated changelog ]

     First-tag: .....

   娉ㄦ剰鐢ㄤ袱涓┖琛屽皢璇ユ彁绀轰笌鍙樻洿鏃ュ織鏂囨湰鍙婃彁浜ゆ爣绛惧垎闅斿紑銆?
   濡傛灉琛ヤ竵鐢卞鐞嗚€呭彂閫佸埌閭欢鍒楄〃锛岄偅涔堜綔鑰呭繀椤诲湪鍙樻洿鏃ュ織鐨勭涓€琛岀敤浠ヤ笅鏂瑰紡娉ㄦ槑::

     From: Author <author@mail>

     Changelog text starts here....

   浠ヤ究淇濈暀浣滆€呰韩浠姐€?From:' 琛屼箣鍚庡繀椤昏窡涓€涓┖琛屻€傚鏋滅己灏戣 'From:' 琛岋紝閭ｄ箞琛ヤ竵
   浼氳褰掍簬鍙戦€侊紙浼犻€併€佸鐞嗭級瀹冪殑浜恒€?From:' 琛屽湪琛ヤ竵琚簲鐢ㄦ椂浼氳鑷姩绉婚櫎锛屼笉浼?   鍑虹幇鍦ㄦ渶缁堢殑 git 鍙樻洿鏃ュ織涓€傚畠浠呭奖鍝嶆渶缁?Git 鎻愪氦鐨勪綔鑰呬俊鎭€?
 - Tested-by: ``Tester <tester@mail>``

 - Reviewed-by: ``Reviewer <reviewer@mail>``

 - Acked-by: ``Acker <acker@mail>``

 - Cc: ``cc-ed-person <person@mail>``

   濡傛灉琛ヤ竵搴旇鍥炵Щ妞嶅埌 stable锛岃娣诲姞鈥渀`Cc: stable@vger.kernel.org``鈥濇爣绛撅紝浣嗗湪
   鍙戦€侀偖浠舵椂涓嶈 Cc stable銆?
 - Link: ``https://link/to/information``

   瀵逛簬寮曠敤鍙戝竷鍒板唴鏍搁偖浠跺垪琛ㄧ殑閭欢锛岃浣跨敤 lore.kernel.org 閲嶅畾鍚戝櫒 URL::

     Link: https://lore.kernel.org/email-message-id@here

   璇?URL 搴旂敤浜庡紩鐢ㄧ浉鍏崇殑閭欢鍒楄〃涓婚銆佺浉鍏崇殑琛ヤ竵闆嗘垨鍏跺畠鍊煎緱娉ㄦ剰鐨勮璁虹嚎绋嬨€傚皢
   ``Link:`` 棰勫憡锛坱railer锛変笌鎻愪氦淇℃伅鍏宠仈璧锋潵鐨勪竴涓究鎹锋柟娉曟槸浣跨敤绫?Markdown 鐨?   鏂规嫭鍙疯娉曪紝渚嬪::

     A similar approach was attempted before as part of a different
     effort [1], but the initial implementation caused too many
     regressions [2], so it was backed out and reimplemented.

     Link: https://lore.kernel.org/some-msgid@here # [1]
     Link: https://bugzilla.example.org/bug/12345  # [2]

   浣犱篃鍙互浣跨敤 ``Link:`` 棰勫憡鏉ユ爣绀哄皢琛ヤ竵搴旂敤鍒颁綘鐨?git 鏍戞椂鐨勬潵婧愩€傚湪杩欑鎯呭喌涓嬶紝
   璇蜂娇鐢ㄤ笓鐢ㄧ殑 ``patch.msgid.link`` 鍩熷悕锛岃€屼笉鏄?``lore.kernel.org``銆傝繖绉嶅仛娉曚娇
   鑷姩鍖栧伐鍏疯兘澶熻瘑鍒娇鐢ㄥ摢涓摼鎺ユ潵鍙栧洖鍘熷琛ヤ竵鎻愪氦銆備緥濡?:

     Link: https://patch.msgid.link/patch-source-message-id@here

```

璇蜂笉瑕佷娇鐢ㄧ粍鍚堟爣绛撅紝渚嬪 `Reported-and-tested-by`锛屽洜涓哄畠浠彧浼氫娇鏍囩鐨勮嚜鍔ㄦ彁鍙?鍙樺緱澶嶆潅銆?
##### 鏂囨。閾炬帴

鍦ㄥ彉鏇存棩蹇椾腑鎻愪緵鏂囨。閾炬帴瀵规棩鍚庤皟璇曞拰鍒嗘瀽鏄瀬澶х殑甯姪銆傞仐鎲剧殑鏄紝URL 寰€寰€寰堝揩灏?澶辨晥锛屽洜涓哄叕鍙搁绻佸湴閲嶆瀯鍏剁綉绔欍€傞潪鈥滄槗鍙橈紙volatile锛夆€濈殑渚嬪鍖呮嫭 Intel SDM 鍜?AMD APM銆?
鍥犳锛屽浜庘€滄槗鍙樷€濇枃妗ｏ紝璇峰湪 kernel bugzilla https://bugzilla.kernel.org 鍒涘缓涓€涓?鏉＄洰锛屽苟灏嗚繖浜涙枃妗ｇ殑鍓湰闄勫埌璇?bugzilla 鏉＄洰涓娿€傛渶鍚庯紝鍦ㄥ彉鏇存棩蹇椾腑鎻愪緵璇?bugzilla
鏉＄洰鐨?URL銆?
##### 琛ヤ竵閲嶅彂鎴栨彁閱?
鍙傝 resend_reminders銆?
##### 鍚堝苟绐楀彛锛圡erge window锛?
璇蜂笉瑕佸湪鍚堝苟绐楀彛鏈熼棿鎴栦复杩戝悎骞剁獥鍙ｆ椂鏈熸湜 tip 缁存姢鑰呬細瀹￠槄鎴栧悎骞惰ˉ涓併€傚湪姝ゆ湡闂达紝
闄や簡绱ф€ヤ慨澶嶅锛岃繖浜涙爲閮芥槸鍏抽棴鐨勩€備竴鏃﹀悎骞剁獥鍙ｅ叧闂苟鍙戝竷鏂扮殑 -rc1 鍐呮牳锛屽畠浠細
閲嶆柊寮€鏀俱€?
澶у瀷琛ヤ竵绯诲垪锛坰eries锛夊簲璇ュ湪鍚堝苟绐楀彛寮€鍚?*鑷冲皯**涓€鍛ㄤ箣鍓嶄互鍙悎骞剁姸鎬佹彁浜ゃ€傚浜?缂洪櫡淇浠ュ強**鏈夋椂**閽堝鏂扮‖浠剁殑灏忓瀷鐙珛椹卞姩鎴栦镜鍏ユ€ф瀬灏忕殑纭欢鏀寔琛ヤ竵锛屽彲浠ユ湁
渚嬪銆?
鍦ㄥ悎骞剁獥鍙ｆ湡闂达紝缁存姢鑰呰浆鑰屼笓娉ㄤ簬璺熻釜涓婃父鏀瑰姩銆佷慨澶嶅悎骞剁獥鍙ｄ骇鐢熺殑闂銆佹敹闆嗙己闄?淇锛屽苟璁╄嚜宸卞枠鍙ｆ皵銆傝灏婇噸杩欎竴鐐广€?
鎵€璋撶殑*绱ф€ワ紙urgent锛?鍒嗘敮浼氬湪姣忎釜鍙戝竷鐗堟湰鐨?stabilization 闃舵琚悎骞跺埌涓荤嚎銆?
##### Git

tip 缁存姢鑰呮帴鍙楁潵鑷淮鎶よ€呯殑 git pull 璇锋眰锛岃繖浜涚淮鎶よ€呮彁渚涜鍦?tip 鏍戜腑鑱氬悎鐨勫瓙绯荤粺
鏀瑰姩銆?
閽堝鏂拌ˉ涓佹彁浜ょ殑 pull 璇锋眰閫氬父涓嶈鎺ュ彈锛屼篃涓嶈兘鍙栦唬鍚戦偖浠跺垪琛ㄧ殑姝ｇ‘琛ヤ竵鎻愪氦銆備富瑕?鍘熷洜鏄闃呭伐浣滄祦绋嬫槸鍩轰簬閭欢鐨勩€?
濡傛灉浣犳彁浜や竴涓緝澶х殑琛ヤ竵绯诲垪锛屾彁渚涗竴涓鏈変粨搴撲腑鐨?git 鍒嗘敮浼氬緢鏈夊府鍔╋紝浣挎劅鍏磋叮
鐨勪汉鍙互杞绘澗鎷夊彇璇ョ郴鍒楄繘琛屾祴璇曘€傞€氬父鐨勫仛娉曟槸鍦ㄨˉ涓佺郴鍒楃殑灏侀潰淇★紙cover letter锛変腑
鎻愪緵 git URL銆?
##### 娴嬭瘯

浠ｇ爜鍦ㄦ彁浜ょ粰 tip 缁存姢鑰呬箣鍓嶅簲褰撶粡杩囨祴璇曘€傞櫎浜嗗井灏忕殑鏀瑰姩涔嬪锛屼换浣曟敼鍔ㄩ兘搴旇鏋勫缓銆?鍚姩锛屽苟鍦ㄥ惎鐢ㄤ簡鍏ㄩ潰锛堜笖閲嶉噺绾э級鐨勫唴鏍歌皟璇曢€夐」鐨勬儏鍐典笅杩涜娴嬭瘯銆?
杩欎簺璋冭瘯閫夐」鍙互鍦?kernel/configs/x86_debug.config 涓壘鍒帮紝骞跺彲閫氳繃杩愯浠ヤ笅鍛戒护
娣诲姞鍒板凡鏈夌殑鍐呮牳閰嶇疆涓細

	make x86_debug.config

鍏朵腑涓€浜涢€夐」鏄?x86 鐗瑰畾鐨勶紝鍦ㄥ叾瀹冩灦鏋勪笂娴嬭瘯鏃跺彲浠ョ渷鍘汇€?
### 缂栫爜椋庢牸璇存槑

##### 娉ㄩ噴椋庢牸

娉ㄩ噴涓殑鍙ュ瓙浠ュぇ鍐欏瓧姣嶅紑澶淬€?
```

	/* This is a single line comment */

```

```

	/*
	 * This is a properly formatted
	 * multi-line comment.
	 *
	 * Larger multi-line comments should be split into paragraphs.
	 */

```

涓嶈浣跨敤灏鹃殢娉ㄩ噴锛坱ail comment锛夛紙瑙佷笅锛夛細

  Please refrain from using tail comments. Tail comments disturb the
```

	if (somecondition_is_true) /* Don't put a comment here */
		dostuff(); /* Neither here */

	seed = MAGIC_CONSTANT; /* Nor here */

  Use freestanding comments instead::

	/* This condition is not obvious without a comment */
	if (somecondition_is_true) {
		/* This really needs to be documented */
		dostuff();
	}

	/* This magic initialization needs a comment. Maybe not? */
	seed = MAGIC_CONSTANT;

  Use C++ style, tail comments when documenting structs in headers to
  achieve a more compact layout and better readability::

        // eax
        u32     x2apic_shift    :  5, // Number of bits to shift APIC ID right
                                      // for the topology ID at the next level
                                : 27; // Reserved
        // ebx
        u32     num_processors  : 16, // Number of processors at current level
                                : 16; // Reserved

  versus::

	/* eax */
	        /*
	         * Number of bits to shift APIC ID right for the topology ID
	         * at the next level
	         */
         u32     x2apic_shift    :  5,
		 /* Reserved */
				 : 27;

	/* ebx */
		/* Number of processors at current level */
	u32     num_processors  : 16,
		/* Reserved */
				: 16;

```

娉ㄩ噴閲嶈鐨勪笢瑗匡細

  Comments should be added where the operation is not obvious. Documenting
```

	/* Decrement refcount and check for zero */
	if (refcount_dec_and_test(&p->refcnt)) {
		do;
		lots;
		of;
		magic;
		things;
	}

  Instead, comments should explain the non-obvious details and document
  constraints::

	if (refcount_dec_and_test(&p->refcnt)) {
		/*
		 * Really good explanation why the magic things below
		 * need to be done, ordering and locking constraints,
		 * etc..
		 */
		do;
		lots;
		of;
		magic;
		/* Needs to be the last operation because ... */
		things;
	}

```

鍑芥暟鏂囨。娉ㄩ噴锛?
  To document functions and their arguments please use kernel-doc format
```

	/**
	 * magic_function - Do lots of magic stuff
	 * @magic:	Pointer to the magic data to operate on
	 * @offset:	Offset in the data array of @magic
	 *
	 * Deep explanation of mysterious things done with @magic along
         * with documentation of the return values.
	 *
	 * Note, that the argument descriptors above are arranged
	 * in a tabular fashion.
	 */

  This applies especially to globally visible functions and inline
  functions in public header files. It might be overkill to use kernel-doc
  format for every (static) function which needs a tiny explanation. The
  usage of descriptive function names often replaces these tiny comments.
  Apply common sense as always.


```

##### 璁板綍閿佺殑瑕佹眰

  Documenting locking requirements is a good thing, but comments are not
```

	/* Caller must hold foo->lock */
	void func(struct foo *foo)
	{
		...
	}

  Please use::

	void func(struct foo *foo)
	{
		lockdep_assert_held(&foo->lock);
		...
	}

  In PROVE_LOCKING kernels, lockdep_assert_held() emits a warning
  if the caller doesn't hold the lock.  Comments can't do that.

```

##### 鎷彿瑙勫垯

鍙湁鍦ㄨ窡闅忊€渋f鈥濄€佲€渇or鈥濈瓑涔嬪悗鐨勮鍙ユ槸鍗曡鏃讹紝鎵嶅彲浠ョ渷鐣ユ嫭鍙凤紝渚嬪锛?
```

	if (foo)
		do_something();

```

鍗充娇濡備笅鎯呭喌涔熶笉琚涓哄崟琛岃鍙ワ細

```

	for (i = 0; i < end; i++)
		if (foo[i])
			do_something(foo[i]);

```

```

	for (i = 0; i < end; i++) {
		if (foo[i])
			do_something(foo[i]);
	}


```

##### 鍙橀噺澹版槑

鍙橀噺澹版槑鍦ㄥ潡寮€澶存椂鐨勯閫夐『搴忓涓嬶細

```

	struct long_struct_name *descriptive_name;
	unsigned long foo, bar;
	unsigned int tmp;
	int ret;

```

```

	int ret;
	unsigned int tmp;
	unsigned long foo, bar;
	struct long_struct_name *descriptive_name;

```

```

	unsigned long foo, bar;
	int ret;
	struct long_struct_name *descriptive_name;
	unsigned int tmp;

```

鍙﹀锛岃灏介噺灏嗗悓涓€绫诲瀷鐨勫彉閲忚仛鍚堝埌涓€琛岋細

```

	unsigned long a;
	unsigned long b;
	unsigned long c;
	unsigned long d;

```

```

	unsigned long a, b, c, d;

```

```

	struct long_struct_name *descriptive_name = container_of(bar,
						      struct long_struct_name,
	                                              member);
	struct foobar foo;

```

灏嗗垵濮嬪寲绉诲埌澹版槑涔嬪悗鐨勫崟鐙竴琛屼細鏇村ソ锛?
```

	struct long_struct_name *descriptive_name;
	struct foobar foo;

	descriptive_name = container_of(bar, struct long_struct_name, member);


```

##### 鍙橀噺绫诲瀷

瀵逛簬鏃ㄥ湪鎻忚堪纭欢鎴栦綔涓鸿闂‖浠剁殑鍑芥暟鍙傛暟鐨勫彉閲忥紝璇蜂娇鐢ㄩ€傚綋鐨?u8銆乽16銆乽32銆乽64
绫诲瀷銆傝繖浜涚被鍨嬫竻鏅板湴瀹氫箟浜嗕綅瀹斤紝骞堕伩鍏嶄簡鎴柇銆佹墿灞曚互鍙?32/64 浣嶆贩娣嗐€?
鍦ㄥ鏋滀娇鐢ㄢ€渦nsigned long鈥濅細瀵?32 浣嶅唴鏍镐骇鐢熸涔夌殑浠ｇ爜涓篃鎺ㄨ崘浣跨敤 u64銆傝櫧鐒?鍦ㄨ繖绉嶆儏鍐典笅涔熷彲浠ヤ娇鐢ㄢ€渦nsigned long long鈥濓紝浣?u64 鏇寸煭锛屽苟涓斾篃娓呮鍦拌〃鏄庤鎿嶄綔
瑕佹眰涓?64 浣嶅锛屼笌鐩爣 CPU 鏃犲叧銆?
璇蜂娇鐢ㄢ€渦nsigned int鈥濊€屼笉鏄€渦nsigned鈥濄€?
##### 甯搁噺

璇蜂笉瑕佸湪浠ｇ爜鎴栧垵濮嬪寲鍣ㄤ腑浣跨敤瀛楅潰锛堝崄鍏繘鍒?鍗佽繘鍒讹級鏁板瓧銆傝涔堜娇鐢ㄥ叿鏈夋弿杩版€у悕绉?鐨勯€傚綋 define锛岃涔堣€冭檻浣跨敤 enum銆?
##### 缁撴瀯浣撳０鏄庝笌鍒濆鍖栧櫒

缁撴瀯浣撳０鏄庡簲璇ュ皢缁撴瀯浣撴垚鍛樺悕浠ヨ〃鏍煎舰寮忓榻愶細

```

	struct bar_order {
		unsigned int	guest_id;
		int		ordered_item;
		struct menu	*menu;
	};

```

璇烽伩鍏嶅湪澹版槑涓褰曠粨鏋勪綋鎴愬憳锛屽洜涓鸿繖甯稿父瀵艰嚧鏍煎紡濂囨€殑娉ㄩ噴锛岃€屼笖缁撴瀯浣撴垚鍛橈細

```

	struct bar_order {
		unsigned int	guest_id; /* Unique guest id */
		int		ordered_item;
		/* Pointer to a menu instance which contains all the drinks */
		struct menu	*menu;
	};

```

鐩稿弽锛岃鑰冭檻鍦ㄧ粨鏋勪綋澹版槑涔嬪墠鐨勬敞閲婁腑浣跨敤 kernel-doc 鏍煎紡锛岃繖鏍峰仛鏇存槗璇伙紝骞朵笖杩?鏈変竴涓澶栫殑濂藉锛屽嵆鎶婁俊鎭撼鍏ュ唴鏍告枃妗ｄ腑锛屼緥濡傦細

```

	/**
	 * struct bar_order - Description of a bar order
	 * @guest_id:		Unique guest id
	 * @ordered_item:	The item number from the menu
	 * @menu:		Pointer to the menu from which the item
	 *  			was ordered
	 *
	 * Supplementary information for using the struct.
	 *
	 * Note, that the struct member descriptors above are arranged
	 * in a tabular fashion.
	 */
	struct bar_order {
		unsigned int	guest_id;
		int		ordered_item;
		struct menu	*menu;
	};

```

闈欐€佺粨鏋勪綋鍒濆鍖栧櫒蹇呴』浣跨敤 C99 鍒濆鍖栧櫒锛屽苟涓斾篃搴旇锛?
```

	static struct foo statfoo = {
		.a		= 0,
		.plain_integer	= CONSTANT_DEFINE_OR_ENUM,
		.bar		= &statbar,
	};

```

娉ㄦ剰锛岃櫧鐒?C99 璇硶鍏佽鐪佺暐鏈€鍚庣殑閫楀彿锛屼絾鎴戜滑寤鸿鍦ㄦ渶鍚庝竴琛屼娇鐢ㄩ€楀彿锛屽洜涓鸿繖浣垮緱
閲嶆柊鎺掑簭鍜屾坊鍔犳柊琛屾洿瀹规槗锛屼篃璁╄繖绫绘湭鏉ョ殑琛ヤ竵绋嶅井鏇存槗璇汇€?
##### 鎹㈣

灏嗚闀块檺鍒跺湪 80 涓瓧绗︿細浣挎繁搴︾缉杩涚殑浠ｇ爜闅句互闃呰銆傝€冭檻灏嗕唬鐮佹彁鍙栧埌杈呭姪鍑芥暟涓紝
浠ラ伩鍏嶈繃搴︽崲琛屻€?
80 瀛楃瑙勫垯骞堕潪纭€ц鍒欙紝鍥犳鍦ㄦ崲琛屾椂璇疯繍鐢ㄥ父璇嗐€傚挨鍏舵槸鏍煎紡瀛楃涓茬粷涓嶅簲琚媶寮€銆?
鎷嗗垎鍑芥暟澹版槑鎴栧嚱鏁拌皟鐢ㄦ椂锛岃灏嗙浜岃涓殑绗竴涓弬鏁颁笌绗竴琛屼腑鐨勭涓€涓弬鏁板榻愶細

```

  static int long_function_name(struct foobar *barfoo, unsigned int id,
				unsigned int offset)
  {

	if (!id) {
		ret = longer_function_name(barfoo, DEFAULT_BARFOO_ID,
					   offset);
	...

```

##### 鍛藉悕绌洪棿锛圢amespaces锛?
鍑芥暟/鍙橀噺鍛藉悕绌洪棿鎻愰珮浜嗗彲璇绘€у苟渚夸簬鎼滅储锛坓rep锛夈€傝繖浜涘懡鍚嶇┖闂存槸鍏ㄥ眬鍙鐨勫嚱鏁板拰
鍙橀噺鍚嶏紙鍖呮嫭鍐呰仈鍑芥暟锛夌殑瀛楃涓插墠缂€銆傝繖浜涘墠缂€搴旂粨鍚堝瓙绯荤粺鍚嶄笌缁勪欢鍚嶏紝渚嬪
鈥渪86_comp\_鈥濄€佲€渟ched\_鈥濄€佲€渋rq\_鈥濆拰鈥渕utex\_鈥濄€?
杩欎篃鍖呮嫭琚珛鍗虫斁鍏ュ叏灞€鍙椹卞姩妯℃澘鐨勯潤鎬佹枃浠朵綔鐢ㄥ煙鍑芥暟鈥斺€斿浜庤繖浜涚鍙凤紝甯︿笂涓€涓?濂界殑鍓嶇紑涔熷緢鏈夌敤锛屼互渚垮洖婧紙backtrace锛夋椂鍙銆?
瀵逛簬灞€閮ㄩ潤鎬佸嚱鏁板拰鍙橀噺锛屽彲浠ョ渷鐣ュ懡鍚嶇┖闂村墠缂€銆傜湡姝ｅ眬閮ㄧ殑鍑芥暟锛屽彧琚叾瀹冨眬閮ㄥ嚱鏁?璋冪敤锛屽彲浠ユ湁鏇寸煭鐨勬弿杩版€у悕绉扳€斺€旀垜浠富瑕佸叧蹇冪殑鏄彲鎼滅储鎬у拰鍥炴函鍙鎬с€?
璇锋敞鎰忥紝鈥渪xx_vendor\_鈥濆拰鈥渧endor_xxx_鈥濆墠缂€瀵逛簬鍘傚晢鐗瑰畾鏂囦欢涓殑闈欐€佸嚱鏁板苟鏃犲府鍔┿€?姣曠珶锛屼唬鐮佹槸鍘傚晢鐗瑰畾鐨勮繖涓€鐐瑰凡缁忓緢娓呮浜嗐€傛澶栵紝鍘傚晢鍚嶅彧搴旂敤浜庣湡姝ｅ巶鍟嗙壒瀹氱殑鍔熻兘銆?
涓€濡傛棦寰€锛岃繍鐢ㄥ父璇嗭紝浠ヤ竴鑷存€у拰鍙鎬т负鐩爣銆?
### 鎻愪氦閫氱煡

tip 鏍戠敱涓€涓満鍣ㄤ汉鐩戞帶鏂版彁浜ゃ€傝鏈哄櫒浜轰负姣忔鏂版彁浜ゅ悜涓€涓笓鐢ㄩ偖浠跺垪琛?锛坄linux-tip-commits@vger.kernel.org`锛夊彂閫侀偖浠讹紝骞?Cc 鍦ㄥ叾涓竴涓彁浜ゆ爣绛句腑琚?鎻愬強鐨勬墍鏈変汉銆傚畠浣跨敤鏍囩鍒楄〃鏈熬 Link 鏍囩涓殑閭欢 Message-ID 鏉ヨ缃?In-Reply-To
閭欢澶达紝浠庤€屼娇璇ラ偖浠朵笌琛ヤ竵鎻愪氦閭欢姝ｇ‘鍦板舰鎴愮嚎绋嬨€?
tip 缁存姢鑰呭拰瀛愮淮鎶よ€呬細灏介噺鍦ㄥ悎骞惰ˉ涓佹椂鍥炲鎻愪氦鑰咃紝浣嗕粬浠湁鏃跺繕璁帮紝鎴栬€呬笉绗﹀悎
褰撲笅鐨勫伐浣滄祦绋嬨€傝櫧鐒舵満鍣ㄤ汉娑堟伅绾补鏄満姊版€х殑锛屼絾瀹冧篃鎰忓懗鐫€鈥滆阿璋紒宸插簲鐢ㄣ€傗€濄€?