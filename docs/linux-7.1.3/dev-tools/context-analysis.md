

## 鍩轰簬缂栬瘧鍣ㄧ殑涓婁笅鏂囧垎鏋愶紙Context Analysis锛?


涓婁笅鏂囧垎鏋愶紙Context Analysis锛夋槸涓€绉嶈瑷€鎵╁睍锛屽畠閫氳繃鑾峰彇涓庨噴鏀剧敤鎴峰彲瀹氫箟鐨勨€滀笂涓嬫枃閿佲€濓紙context lock锛夋潵闈欐€佹鏌ユ墍闇€鐨勪笂涓嬫枃鏄惁澶勪簬婵€娲伙紙鎴栭潪婵€娲伙級鐘舵€併€備竴涓槑鏄剧殑搴旂敤鏄鍐呮牳鍚勭鍚屾鍘熻锛堟瘡涓€涓兘浠ｈ〃涓€涓€滀笂涓嬫枃閿佲€濓級杩涜閿佸畨鍏ㄦ€ф鏌ワ紝骞舵鏌ユ槸鍚﹁繚鍙嶄簡鍔犻攣瑙勫垯銆?

Clang 缂栬瘧鍣ㄧ洰鍓嶆敮鎸佸畬鏁寸殑涓婁笅鏂囧垎鏋?

```

    CONFIG_WARN_CONTEXT_ANALYSIS=y

```
璇ョ壒鎬ч渶瑕?Clang 22 鎴栨洿楂樼増鏈€?

璇ュ垎鏋愰粯璁ゆ槸**閫夋嫨鎬у惎鐢紙opt-in锛?*鐨勶紝骞朵笖闇€瑕佸０鏄庡摢浜涙ā鍧椾互鍙?

```

    CONTEXT_ANALYSIS_mymodule.o := y

```
```

    CONTEXT_ANALYSIS := y

```
涓嶈繃锛屼篃鍙互鍦ㄦ暣涓唬鐮佹爲涓婂惎鐢ㄨ鍒嗘瀽锛岃繖灏嗗鑷?

```

    CONFIG_WARN_CONTEXT_ANALYSIS_ALL=y

```
### 缂栫▼妯″瀷


涓嬮潰鎻忚堪鍥寸粫浣跨敤涓婁笅鏂囬攣绫诲瀷鐨勭紪绋嬫ā鍨嬨€?

   鍚敤涓婁笅鏂囧垎鏋愬彲浠ョ湅浣滄槸鍦ㄥ惎鐢ㄤ竴绉嶅甫鏈夆€滀笂涓嬫枃绯荤粺锛圕ontext System锛夆€濈殑 Linux C 鏂硅█銆備竴浜涙秹鍙婂鏉傛帶鍒舵祦鐨勬湁鏁堟ā寮忎細鍙楀埌绾︽潫锛堜緥濡傚湪鍚屼竴鍑芥暟鍐呰繘琛屾潯浠惰幏鍙栦互鍙婇殢鍚庢潯浠堕噴鏀撅級銆?

涓婁笅鏂囧垎鏋愭槸涓€绉嶅皢鎿嶄綔鐨勫彲鍏佽鎬ф寚瀹氫负渚濊禆浜庢槸鍚︽寔鏈夛紙鎴栨湭鎸佹湁锛変笂涓嬫枃閿佺殑鏂规硶銆傞€氬父锛屾垜浠殑鐩爣鏄€氳繃瑕佹眰鏌愪釜鐗瑰畾涓婁笅鏂囧浜庢縺娲荤姸鎬佹潵淇濇姢涓寸晫鍖轰腑鐨勬暟鎹笌浠ｇ爜锛屼緥濡傛寔鏈夋煇涓壒瀹氱殑閿併€傝鍒嗘瀽纭繚璋冪敤鑰呭湪娌℃湁鎵€闇€涓婁笅鏂囧浜庢縺娲荤姸鎬佺殑鎯呭喌涓嬫棤娉曟墽琛屾煇椤规搷浣溿€?

涓婁笅鏂囬攣涓庡叿鍚嶇殑 struct 鐩稿叧鑱旓紝鍚屾椂涔熶笌閭ｄ簺鎿嶄綔 struct 瀹炰緥浠ヨ幏鍙栧拰閲婃斁鐩稿簲涓婁笅鏂囬攣鐨勫嚱鏁扮浉鍏宠仈銆?

涓婁笅鏂囬攣鏃㈠彲浠ヨ鐙崰鎸佹湁锛屼篃鍙互琚叡浜寔鏈夈€傝繖绉嶆満鍒跺厑璁稿湪涓婁笅鏂囨縺娲绘椂璧嬩簣鏇寸簿纭殑鏉冮檺锛岄€氬父鐢ㄤ簬鍖哄垎绾跨▼鍦ㄦ煇涓笂涓嬫枃涓彧鑳借鍙栵紙鍏变韩锛夎繕鏄篃鑳藉啓鍏ワ紙鐙崰锛夊彈淇濇姢鐨勬暟鎹€?

鍦ㄧ粰瀹氱殑绾跨▼涓€佸湪绋嬪簭鎵ц鐨勬煇涓壒瀹氭椂鍒诲疄闄呭浜庢縺娲荤姸鎬佺殑涓婁笅鏂囬泦鍚堬紝鏄竴涓繍琛屾椂姒傚康銆傞潤鎬佸垎鏋愰€氳繃璁＄畻璇ラ泦鍚堢殑涓€涓繎浼硷紙绉颁负涓婁笅鏂囩幆澧冿紝context environment锛夋潵宸ヤ綔銆備笂涓嬫枃鐜閽堝姣忎竴涓▼搴忕偣杩涜璁＄畻锛屽苟鎻忚堪鍦ㄨ鐗瑰畾鐐逛笂闈欐€佸凡鐭ヤ负婵€娲绘垨闈炴縺娲荤殑涓婁笅鏂囬泦鍚堛€傝繖涓幆澧冩槸瀵圭嚎绋嬪湪杩愯鏃跺疄闄呬細婵€娲荤殑瀹屾暣涓婁笅鏂囬泦鍚堢殑涓€涓繚瀹堣繎浼笺€?

鏇村缁嗚妭涔熻褰曞湪 `here
<https://clang.llvm.org/docs/ThreadSafetyAnalysis.html>`_銆?

   Clang 鐨勫垎鏋愭槑纭湴涓嶄細鎺ㄦ柇鐢卞唴鑱斿嚱鏁拌幏鍙栨垨閲婃斁鐨勪笂涓嬫枃閿併€傚畠闇€瑕佹樉寮忔敞瑙ｆ潵 (a) 鏂█褰撴煇涓笂涓嬫枃閿佽閲婃斁鎴栬幏鍙栨椂杩欎笉鏄竴涓?bug锛屼互鍙?(b) 淇濇寔鍐呰仈涓庨潪鍐呰仈鍑芥暟澹版槑涔嬮棿鐨勪竴鑷存€с€?

#### 鍐呮牳鏀寔鐨勫悓姝ュ師璇?


鐩墠鏀寔浠ヤ笅鍚屾鍘熻锛?
`raw_spinlock_t`, `spinlock_t`, `rwlock_t`, `mutex`, `seqlock_t`,
`bit_spinlock`, RCU, SRCU (`srcu_struct`), `rw_semaphore`, `local_lock_t`,
`ww_mutex`銆?

瑕佷娇鐢ㄥ垵濮嬪寲鍑芥暟锛坄type_init(&lock)`锛夊垵濮嬪寲鍙椾笂涓嬫枃閿佷繚鎶ょ殑鍙橀噺锛屽缓璁紭鍏堜娇鐢?`guard(type_init)(&lock)` 鎴?
`scoped_guard(type_init, &lock) { ... }` 鍦ㄥ灞備綔鐢ㄥ煙涓垵濮嬪寲姝ょ被鍙椾繚鎶ょ殑鎴愬憳鎴栧叏灞€鍙橀噺銆傝繖浼氬垵濮嬪寲涓婁笅鏂囬攣锛屽苟灏嗚涓婁笅鏂囪涓哄湪鍒濆鍖栦綔鐢ㄥ煙鍐呭浜庢縺娲荤姸鎬侊紙鍒濆鍖栨剰鍛崇潃瀵瑰簳灞傚璞℃嫢鏈夌嫭鍗犺闂潈锛夈€?

```

    struct my_data {
            spinlock_t lock;
            int counter __guarded_by(&lock);
    };

    void init_my_data(struct my_data *d)
    {
            ...
            guard(spinlock_init)(&d->lock);
            d->counter = 0;
            ...
    }

```
鍙﹀锛屽垵濮嬪寲鍙椾繚鎶ゅ彉閲忎篃鍙互鍦ㄧ鐢ㄤ笂涓嬫枃鍒嗘瀽鐨勬儏鍐典笅杩涜锛屾渶濂芥槸鍦ㄥ敖鍙兘灏忕殑浣滅敤鍩熷唴锛堝洜涓虹己灏戜换浣曞叾浠栨鏌ワ級锛氭棦鍙互浣跨敤 `context_unsafe(var = init)` 琛ㄨ揪寮忥紝涔熷彲浠ラ€氳繃 `__context_unsafe(init)` 灞炴€ф潵鏍囪灏忓瀷鍒濆鍖栧嚱鏁般€?

Lockdep 鏂█锛堜緥濡?`lockdep_assert_held()`锛変細鍛婄煡缂栬瘧鍣ㄧ殑涓婁笅鏂囧垎鏋愶細鍦ㄦ柇瑷€涔嬪悗锛岀浉鍏崇殑鍚屾鍘熻宸茶鎸佹湁銆傝繖鍙互閬垮厤鍦ㄥ鏉傛帶鍒舵祦鍦烘櫙涓嚭鐜拌鎶ワ紝骞跺湪闈欐€佸垎鏋愯兘鍔涙湁闄愮殑鍦版柟榧撳姳浣跨敤 Lockdep銆備緥濡傦紝褰撲竴涓嚱鏁板苟闈?*鎬绘槸**闇€瑕佹寔閿佹椂锛岃繖灏卞緢鏈夌敤锛屽洜涓烘鏃?`__must_hold()` 骞朵笉鍚堥€傘€?

#### 鍏抽敭瀛?


   :identifiers: context_lock_struct
                 token_context_lock token_context_lock_instance
                 __guarded_by __pt_guarded_by
                 __must_hold
                 __must_not_hold
                 __acquires
                 __cond_acquires
                 __releases
                 __must_hold_shared
                 __acquires_shared
                 __cond_acquires_shared
                 __releases_shared
                 __acquire
                 __release
                 __acquire_shared
                 __release_shared
                 __acquire_ret
                 __acquire_shared_ret
                 context_unsafe
                 __context_unsafe
                 disable_context_analysis enable_context_analysis

   `__no_context_analysis` 鍑芥暟灞炴€т繚鐣欑粰涓婁笅鏂囬攣绫诲瀷鐨勫唴閮ㄥ疄鐜颁娇鐢紝鍦ㄦ櫘閫氫唬鐮佷腑搴旈伩鍏嶄娇鐢ㄣ€?

### 鑳屾櫙


Clang 鏈€鍒濆皢杩欎竴鐗规€хО涓?`Thread Safety Analysis
<https://clang.llvm.org/docs/ThreadSafetyAnalysis.html>`_锛岄儴鍒嗗叧閿瓧涓庢枃妗ｄ粛鍦ㄤ娇鐢ㄤ粎閽堝绾跨▼瀹夊叏鐨勬湳璇€傚悗鏉ヨ繖涓€鐗规€ц鏀瑰姩骞跺彉寰楁洿鍔犵伒娲伙紝鑾峰緱浜嗗畾涔夎嚜瀹氫箟鈥滆兘鍔涳紙capabilities锛夆€濈殑鑳藉姏銆傚叾鍩虹鍙互鍦?`Capability
Systems <https://www.cs.cornell.edu/talc/papers/capabilities.pdf>`_ 涓壘鍒帮紝瀹冪敤浜庢寚瀹氭搷浣滅殑鍙厑璁告€т緷璧栦簬鏌愪釜鈥滆兘鍔涒€濊鎸佹湁锛堟垨鏈寔鏈夛級銆?

鐢变簬璇ョ壒鎬т笉浠呰兘琛ㄨ揪涓庡悓姝ュ師璇浉鍏崇殑鑳藉姏锛岃€屸€渃apability鈥濆湪鍐呮牳涓凡鏈夊叾浠栧惈涔夛紝鍐呮牳鎵€閫夌殑鍛藉悕鍥犳鍋忕浜?Clang 鏈€鍒濃€淭hread Safety鈥濅笌鈥渃apability鈥濈殑鏈锛涙垜浠皢鍏剁О涓衡€淐ontext Analysis鈥濅互閬垮厤娣锋穯銆傚唴閮ㄥ疄鐜颁粛浼氬湪灏戞暟鍦版柟寮曠敤 Clang 鐨勬湳璇紝渚嬪 `-Wthread-safety` 浠嶆槸鍚屾牱浼氬嚭鐜板湪璇婃柇淇℃伅涓殑璀﹀憡閫夐」銆?
