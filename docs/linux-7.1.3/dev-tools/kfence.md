
## 鍐呮牳 Electric-Fence (KFENCE)


Kernel Electric-Fence锛圞FENCE锛夋槸涓€绉嶄綆寮€閿€銆佸熀浜庨噰鏍风殑璁板繂浣撳畨鍏ㄩ敊璇娴嬪櫒銆侹FENCE 鍙娴嬪爢瓒婄晫璁块棶銆侀噴鏀惧悗浣跨敤锛坲se-after-free锛変互鍙婃棤鏁堥噴鏀撅紙invalid-free锛夐敊璇€?
KFENCE 璁捐涓哄彲鍦ㄧ敓浜у唴鏍镐腑鍚敤锛屽苟涓旀€ц兘寮€閿€鎺ヨ繎浜庨浂銆備笌 KASAN 鐩告瘮锛孠FENCE 浠ョ簿搴︽崲鍙栨€ц兘銆侹FENCE 璁捐鐨勪富瑕佸姩鏈哄湪浜庯細鍙鎬昏繍琛屾椂闂磋冻澶熼暱锛孠FENCE 灏辫兘妫€娴嬪嚭閭ｄ簺閫氬父涓嶄細琚潪鐢熶骇娴嬭瘯璐熻浇鎵€鎵ц鍒扮殑浠ｇ爜璺緞涓殑缂洪櫡銆傚揩閫熺疮绉冻澶熼暱鎬昏繍琛屾椂闂寸殑涓€绉嶆柟寮忔槸锛氬皢璇ュ伐鍏烽儴缃插埌澶ц妯℃満鍣ㄩ泦缇や腑銆?
### 鐢ㄦ硶


```
    CONFIG_KFENCE=y

```

瑕佹瀯寤哄甫 KFENCE 鏀寔浣嗛粯璁ょ鐢ㄧ殑鍐呮牳锛堣鍚敤鍒欒缃?```
    CONFIG_KFENCE=y
    CONFIG_KFENCE_SAMPLE_INTERVAL=0

```

KFENCE 杩樻彁渚涜嫢骞插叾浠栭厤缃€夐」鐢ㄤ簬瀹氬埗琛屼负锛堟洿澶氫俊鎭鍙傞槄 `lib/Kconfig.kfence` 涓浉搴旂殑甯姪鏂囨湰锛夈€?
#### 璋冧紭鎬ц兘


鏈€閲嶈鐨勫弬鏁版槸 KFENCE 鐨勯噰鏍烽棿闅旓紝瀹冨彲浠ラ€氳繃鍐呮牳寮曞鍙傛暟 `kfence.sample_interval`锛堝崟浣嶄负姣锛夋潵璁剧疆銆傞噰鏍烽棿闅斿喅瀹氫簡鍫嗗垎閰嶈 KFENCE 淇濇姢璧锋潵鐨勯鐜囥€傞粯璁ゅ€煎彲閫氳繃 Kconfig 閫夐」 `CONFIG_KFENCE_SAMPLE_INTERVAL` 閰嶇疆銆傝缃?`kfence.sample_interval=0` 灏嗙鐢?KFENCE銆?
閲囨牱闂撮殧鎺у埗涓€涓畾鏃跺櫒锛岃瀹氭椂鍣ㄨ礋璐ｅ缓绔?KFENCE 鍒嗛厤銆傞粯璁ゆ儏鍐典笅锛屼负浜嗕繚鎸佸疄闄呴噰鏍烽棿闅旂殑鍙娴嬫€э紝鏅€氬畾鏃跺櫒鍦ㄧ郴缁熷畬鍏ㄧ┖闂叉椂涔熶細鍞ら啋 CPU銆傝繖鍦ㄥ姛鑰楀彈闄愮殑绯荤粺涓婂彲鑳藉苟涓嶇悊鎯炽€傚紩瀵煎弬鏁?`kfence.deferrable=1` 鍒欎細鏀圭敤鈥滃彲寤惰繜锛坉eferrable锛夆€濆畾鏃跺櫒锛屽畠涓嶄細鍦ㄧ┖闂茬郴缁熶笂寮哄埗鍞ら啋 CPU锛屼絾浠ｄ环鏄噰鏍烽棿闅斿彉寰椾笉鍙娴嬨€傞粯璁ゅ€煎彲閫氳繃 Kconfig 閫夐」 `CONFIG_KFENCE_DEFERRABLE` 閰嶇疆銆?
   KUnit 娴嬭瘯濂椾欢鍦ㄤ娇鐢ㄥ彲寤惰繜瀹氭椂鍣ㄦ椂鏋佹湁鍙兘澶辫触锛屽洜涓哄畠鐩墠浼氶€犳垚闈炲父涓嶅彲棰勬祴鐨勯噰鏍烽棿闅斻€?
榛樿鎯呭喌涓嬶紝KFENCE 鍦ㄦ瘡涓噰鏍烽棿闅斿唴鍙 1 涓爢鍒嗛厤杩涜閲囨牱銆?*绐佸彂妯″紡锛圔urst mode锛?* 鍏佽瀵硅繛缁殑鍫嗗垎閰嶈繘琛岄噰鏍凤紝鍏朵腑鍐呮牳寮曞鍙傛暟 `kfence.burst` 鍙涓轰竴涓潪闆跺€硷紝琛ㄧず鍦ㄤ竴涓噰鏍烽棿闅斿唴鐨?*棰濆**杩炵画鍒嗛厤鏁帮紱璁剧疆 `kfence.burst=N` 鎰忓懗鐫€姣忎釜閲囨牱闂撮殧鍐呬細閫氳繃 KFENCE 灏濊瘯 `1 + N` 涓繛缁垎閰嶃€?
KFENCE 鍐呭瓨姹犲ぇ灏忓浐瀹氾紝濡傛灉鍐呭瓨姹犺€楀敖锛屽垯涓嶅啀杩涜杩涗竴姝ョ殑 KFENCE 鍒嗛厤銆傞€氳繃 `CONFIG_KFENCE_NUM_OBJECTS`锛堥粯璁?255锛夊彲浠ユ帶鍒跺彲鐢ㄥ彈淇濇姢瀵硅薄鐨勬暟閲忋€傛瘡涓璞￠渶瑕?2 涓〉锛屼竴涓敤浜庡璞℃湰韬紝鍙︿竴涓敤浣滀繚鎶ら〉锛坓uard page锛夛紱瀵硅薄椤典笌淇濇姢椤典氦閿欐帓鍒楋紝鍥犳姣忎釜瀵硅薄椤甸兘琚袱涓繚鎶ら〉鎵€鍖呭洿銆?
```
    ( #objects + 1 ) * 2 * PAGE_SIZE

```

浣跨敤榛樿閰嶇疆锛屽苟鍋囪椤靛ぇ灏忎负 4 KiB锛屽垯 KFENCE 鍐呭瓨姹犲崰鐢?2 MiB銆?
娉ㄦ剰锛氬湪鏀寔澶ч〉锛坔uge pages锛夌殑鏋舵瀯涓婏紝KFENCE 浼氱‘淇濆唴瀛樻睜浣跨敤澶у皬涓?`PAGE_SIZE` 鐨勯〉銆傝繖灏嗗鑷村垎閰嶉澶栫殑椤佃〃銆?
#### 閿欒鎶ュ憡


寮曞鍙傛暟 `kfence.fault` 鍙敤浜庢帶鍒舵娴嬪埌 KFENCE 閿欒鏃剁殑琛屼负锛?
- `kfence.fault=report`锛氭墦鍗伴敊璇姤鍛婂苟缁х画锛堥粯璁わ級銆?- `kfence.fault=oops`锛氭墦鍗伴敊璇姤鍛婂苟瑙﹀彂 oops銆?- `kfence.fault=panic`锛氭墦鍗伴敊璇姤鍛婂苟瑙﹀彂 panic銆?
```
    ==================================================================
    BUG: KFENCE: out-of-bounds read in test_out_of_bounds_read+0xa6/0x234

    Out-of-bounds read at 0xffff8c3f2e291fff (1B left of kfence-#72):
     test_out_of_bounds_read+0xa6/0x234
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#72: 0xffff8c3f2e292000-0xffff8c3f2e29201f, size=32, cache=kmalloc-32

    allocated by task 484 on cpu 0 at 32.919330s:
     test_alloc+0xfe/0x738
     test_out_of_bounds_read+0x9b/0x234
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 0 PID: 484 Comm: kunit_try_catch Not tainted 5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

鎶ュ憡澶撮儴鎻愪緵浜嗘墍娑夊強璁块棶鍑芥暟鐨勭畝瑕佹憳瑕併€傚叾鍚庤窡闅忓叧浜庤璁块棶鍙婂叾鏉ユ簮鐨勬洿璇︾粏淇℃伅銆傛敞鎰忥紝鍙湁鍦ㄤ娇鐢ㄤ簡鍐呮牳鍛戒护琛岄€夐」 `no_hash_pointers` 鏃舵墠浼氭樉绀虹湡瀹炵殑鍐呮牳鍦板潃銆?
```
    ==================================================================
    BUG: KFENCE: use-after-free read in test_use_after_free_read+0xb3/0x143

    Use-after-free read at 0xffff8c3f2e2a0000 (in kfence-#79):
     test_use_after_free_read+0xb3/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#79: 0xffff8c3f2e2a0000-0xffff8c3f2e2a001f, size=32, cache=kmalloc-32

    allocated by task 488 on cpu 2 at 33.871326s:
     test_alloc+0xfe/0x738
     test_use_after_free_read+0x76/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    freed by task 488 on cpu 2 at 33.871358s:
     test_use_after_free_read+0xa8/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 2 PID: 488 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

```
    ==================================================================
    BUG: KFENCE: invalid free in test_double_free+0xdc/0x171

    Invalid free of 0xffff8c3f2e2a4000 (in kfence-#81):
     test_double_free+0xdc/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#81: 0xffff8c3f2e2a4000-0xffff8c3f2e2a401f, size=32, cache=kmalloc-32

    allocated by task 490 on cpu 1 at 34.175321s:
     test_alloc+0xfe/0x738
     test_double_free+0x76/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    freed by task 490 on cpu 1 at 34.175348s:
     test_double_free+0xa8/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 1 PID: 490 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

KFENCE 杩樺湪瀵硅薄淇濇姢椤电殑鍙︿竴渚т娇鐢ㄥ熀浜庢ā寮忕殑绾㈠尯锛坮edzone锛夛紝浠ユ娴嬪璞℃湭鍙椾繚鎶や竴渚х殑瓒婄晫鍐欏叆銆?```
    ==================================================================
    BUG: KFENCE: memory corruption in test_kmalloc_aligned_oob_write+0xef/0x184

    Corrupted memory at 0xffff8c3f2e33aff9 [ 0xac . . . . . . ] (in kfence-#156):
     test_kmalloc_aligned_oob_write+0xef/0x184
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#156: 0xffff8c3f2e33afb0-0xffff8c3f2e33aff8, size=73, cache=kmalloc-96

    allocated by task 502 on cpu 7 at 42.159302s:
     test_alloc+0xfe/0x738
     test_kmalloc_aligned_oob_write+0x57/0x184
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 7 PID: 502 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

瀵逛簬姝ょ被閿欒锛屼細鏄剧ず鍙戠敓鎹熷潖鐨勫湴鍧€浠ュ強琚棤鏁堝啓鍏ョ殑瀛楄妭锛堢浉瀵逛簬鍦板潃鐨勫亸绉伙級锛涘湪璇ヨ〃绀轰腑锛?.' 琛ㄧず鏈瑙︾鐨勫瓧鑺傘€傚湪涓婇潰绀轰緥涓?`0xac` 鏄啓鍏ュ亸绉?0 澶勬棤鏁堝湴鍧€鐨勫€硷紝鍏朵綑鐨?'.' 琛ㄧず鍚庣画瀛楄妭鏈瑙︾銆傛敞鎰忥紝鍙湁鍦ㄥ唴鏍镐互 `no_hash_pointers` 寮曞鏃舵墠浼氭樉绀虹湡瀹炲€硷紱鍚﹀垯涓洪伩鍏嶄俊鎭硠闇诧紝浼氫娇鐢?'!' 鏉ヨ〃绀鸿鏃犳晥鍐欏叆鐨勫瓧鑺傘€?
鏈€鍚庯紝KFENCE 杩樺彲鑳芥姤鍛婂浠讳綍鍙椾繚鎶ら〉鐨勬棤鏁堣闂紝鑰屾鏃舵棤娉曠‘瀹氬叧鑱旂殑瀵硅薄锛屼緥濡傚綋鐩搁偦
```
    ==================================================================
    BUG: KFENCE: invalid read in test_invalid_access+0x26/0xe0

    Invalid read at 0xffffffffb670b00a:
     test_invalid_access+0x26/0xe0
     kunit_try_run_case+0x51/0x85
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x137/0x160
     ret_from_fork+0x22/0x30

    CPU: 4 PID: 124 Comm: kunit_try_catch Tainted: G        W         5.8.0-rc6+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.13.0-1 04/01/2014
    ==================================================================

```

#### DebugFS 鎺ュ彛


涓€浜涜皟璇曚俊鎭€氳繃 debugfs 鏆撮湶鍑烘潵锛?
- 鏂囦欢 `/sys/kernel/debug/kfence/stats` 鎻愪緵杩愯鏃剁粺璁′俊鎭€?
- 鏂囦欢 `/sys/kernel/debug/kfence/objects` 鎻愪緵閫氳繃 KFENCE 鍒嗛厤鐨勫璞″垪琛紝鍖呮嫭閭ｄ簺宸查噴鏀句絾浠嶅彈淇濇姢鐨勫璞°€?
### 瀹炵幇缁嗚妭


鍙椾繚鎶ょ殑鍒嗛厤鍩轰簬閲囨牱闂撮殧寤虹珛銆傞噰鏍烽棿闅斿埌鏈熷悗锛屼笅涓€娆￠€氳繃涓诲垎閰嶅櫒锛圫LAB 鎴?SLUB锛夎繘琛岀殑鍒嗛厤浼氳繑鍥炰竴涓潵鑷?KFENCE 瀵硅薄姹犵殑鍙椾繚鎶ゅ垎閰嶏紙鏀寔鏈€澶у埌 PAGE_SIZE 鐨勫垎閰嶅ぇ灏忥級銆傛鏃跺畾鏃跺櫒琚噸缃紝骞跺湪璇ラ棿闅斿埌鏈熷悗鍐嶅缓绔嬩笅涓€娆″垎閰嶃€?
褰撲娇鐢?`CONFIG_KFENCE_STATIC_KEYS=y` 鏃讹紝KFENCE 鍒嗛厤閫氳繃涓诲垎閰嶅櫒蹇€熻矾寰勭殑闈欐€佸垎鏀紙static branch锛夛紝渚濊禆闈欐€侀敭锛坰tatic keys锛夊熀纭€璁炬柦杩涜鈥滈棬鎺р€濄€傝闈欐€佸垎鏀細琚垏鎹紝浠ュ皢鍒嗛厤閲嶅畾鍚戝埌 KFENCE銆傛牴鎹噰鏍烽棿闅斻€佺洰鏍囧伐浣滆礋杞戒互鍙婄郴缁熸灦鏋勭殑涓嶅悓锛岃繖鍙兘姣旂畝鍗曠殑鍔ㄦ€佸垎鏀€ц兘鏇村ソ銆傚缓璁繘琛屼粩缁嗙殑鍩哄噯娴嬭瘯銆?
姣忎釜 KFENCE 瀵硅薄閮介┗鐣欏湪涓€涓笓鐢ㄩ〉涓婏紝浣嶄簬闅忔満閫夋嫨鐨勫乏杈圭晫鎴栧彸杈圭晫椤靛銆傚璞￠〉宸﹀彸涓や晶鐨勯〉鏄€滀繚鎶ら〉鈥濓紝鍏跺睘鎬ц鏀逛负鍙椾繚鎶ょ姸鎬侊紝骞跺浠讳綍灏濊瘯鐨勮闂骇鐢熼〉閿欒銆傛绫婚〉閿欒闅忓悗琚?KFENCE 鎷︽埅锛孠FENCE 閫氳繃鎶ュ憡涓€娆¤秺鐣岃闂潵浼橀泤鍦板鐞嗚閿欒锛屽苟灏嗚椤垫爣璁颁负鍙闂紝浠ヤ究寮曞彂閿欒鐨勪唬鐮佽兘澶燂紙閿欒鍦帮級缁х画鎵ц锛堣缃?`panic_on_warn` 鍒欐敼涓鸿Е鍙?panic锛夈€?
涓轰簡妫€娴嬪璞￠〉鏈韩鍐呴儴鐨勫唴瀛樿秺鐣屽啓鍏ワ紝KFENCE 杩樹娇鐢ㄤ簡鍩轰簬妯″紡鐨勭孩鍖恒€傚浜庢瘡涓璞￠〉锛屼細涓烘墍鏈夐潪瀵硅薄鍐呭瓨璁剧疆涓€涓孩鍖恒€傚浜庡吀鍨嬬殑瀵归綈鏂瑰紡锛岀孩鍖哄彧闇€瑕佸湪瀵硅薄鐨勬湭鍙椾繚鎶や竴渚ц缃€傜敱浜?KFENCE 蹇呴』閬靛畧缂撳瓨鎵€璇锋眰鐨勫榻愭柟寮忥紝鐗规畩鐨勫榻愬彲鑳藉鑷村璞′换鎰忎竴渚у嚭鐜版湭鍙椾繚鎶ょ殑闂撮殭锛屾墍鏈夎繖浜涢棿闅欓兘浼氳璁句负绾㈠尯銆?
```
    ---+-----------+-----------+-----------+-----------+-----------+---
       | xxxxxxxxx | O :       | xxxxxxxxx |       : O | xxxxxxxxx |
       | xxxxxxxxx | B :       | xxxxxxxxx |       : B | xxxxxxxxx |
       | x GUARD x | J : RED-  | x GUARD x | RED-  : J | x GUARD x |
       | xxxxxxxxx | E :  ZONE | xxxxxxxxx |  ZONE : E | xxxxxxxxx |
       | xxxxxxxxx | C :       | xxxxxxxxx |       : C | xxxxxxxxx |
       | xxxxxxxxx | T :       | xxxxxxxxx |       : T | xxxxxxxxx |
    ---+-----------+-----------+-----------+-----------+-----------+---

```

KFENCE 瀵硅薄琚噴鏀炬椂锛岃瀵硅薄鐨勯〉浼氬啀娆¤淇濇姢锛屽苟涓斿璞¤鏍囪涓哄凡閲婃斁銆傚璇ュ璞＄殑浠讳綍杩涗竴姝ヨ闂兘浼氬紩鍙戦敊璇紝KFENCE 浼氭姤鍛婁竴娆￠噴鏀惧悗浣跨敤璁块棶銆傚凡閲婃斁鐨勫璞¤鎻掑叆鍒?KFENCE 绌洪棽閾捐〃鐨勫熬閮紝浠ヤ究鏈€杩戞渶灏戦噴鏀剧殑瀵硅薄琚紭鍏堝鐢紝浠庤€屽鍔犳娴嬪埌鏈€杩戦噴鏀惧璞＄殑閲婃斁鍚庝娇鐢ㄩ棶棰樼殑姒傜巼銆?
濡傛灉鍐呭瓨姹犲埄鐢ㄧ巼杈惧埌 75%锛堥粯璁わ級鎴栦互涓婏紝涓洪檷浣庡唴瀛樻睜鏈€缁堣宸插垎閰嶅璞″畬鍏ㄥ崰婊＄殑椋庨櫓锛屽悓鏃朵繚璇佸垎閰嶇殑澶氭牱鍖栬鐩栵紝KFENCE 浼氶檺鍒跺綋鍓嶅凡瑕嗙洊鐨勩€佹潵鑷悓涓€鏉ユ簮鐨勫垎閰嶈繘涓€姝ュ～婊″唴瀛樻睜銆備竴娆″垎閰嶇殑鈥滄潵婧愨€濆熀浜庡叾閮ㄥ垎鍒嗛厤鏍堝洖婧€備竴涓壇浣滅敤鏄紝杩欎篃闄愬埗浜嗘潵鑷悓涓€鏉ユ簮鐨勯绻侀暱鐢熷懡鍛ㄦ湡鍒嗛厤锛堜緥濡傞〉缂撳瓨锛夋案涔呭～婊″唴瀛樻睜锛岃€岃繖鏄鑷村唴瀛樻睜鍙樻弧銆侀噰鏍峰垎閰嶇巼闄嶄负闆剁殑鏈€甯歌椋庨櫓銆傚紑濮嬮檺鍒跺綋鍓嶅凡瑕嗙洊鍒嗛厤鐨勯槇鍊煎彲浠ラ€氳繃寮曞鍙傛暟 `kfence.skip_covered_thresh`锛堝唴瀛樻睜浣跨敤鐜?%锛夎繘琛岄厤缃€?
### 鎺ュ彛


浠ヤ笅鎻忚堪鍒嗛厤鍣ㄤ互鍙婇〉澶勭悊浠ｇ爜鐢ㄤ簬寤虹珛鍜屽鐞?KFENCE 鍒嗛厤鐨勫嚱鏁般€?
   :functions: is_kfence_address
               kfence_shutdown_cache
               kfence_alloc kfence_free __kfence_free
               kfence_ksize kfence_object_start
               kfence_handle_page_fault

### 鐩稿叧宸ュ叿


鍦ㄧ敤鎴风┖闂翠腑锛宍GWP-ASan <http://llvm.org/docs/GwpAsan.html>`_ 閲囩敤浜嗙被浼肩殑鏂规硶銆侴WP-ASan 鍚屾牱渚濊禆淇濇姢椤靛拰閲囨牱绛栫暐鏉ュぇ瑙勬ā妫€娴嬪唴瀛樹笉瀹夊叏缂洪櫡銆侹FENCE 鐨勮璁＄洿鎺ュ彈鍒?GWP-ASan 鐨勫奖鍝嶏紝鍙涓哄叾鍐呮牳鐗堟湰鍏勫紵銆傚彟涓€涓被浼间絾闈為噰鏍枫€佸苟涓斾篃鍚彂浜?鈥淜FENCE鈥?杩欎竴鍚嶇О鐨勬柟娉曪紝鍙互鍦ㄧ敤鎴风┖闂寸殑 `Electric Fence Malloc Debugger <https://linux.die.net/man/3/efence>`_ 涓壘鍒般€?
鍦ㄥ唴鏍镐腑锛屽瓨鍦ㄨ嫢骞茬敤浜庤皟璇曞唴瀛樿闂敊璇殑宸ュ叿锛岀壒鍒槸 KASAN 鑳藉妫€娴嬪嚭 KFENCE 鎵€鑳芥娴嬬殑鎵€鏈夌己闄风被鍒€傝櫧鐒?KASAN 鍊熷姪缂栬瘧鍣ㄦ彃妗╂洿涓虹簿纭紝浣嗚繖浼氬甫鏉ユ€ц兘浠ｄ环銆?
鍊煎緱寮鸿皟鐨勬槸锛孠ASAN 涓?KFENCE 鏄簰琛ョ殑锛岄潰鍚戜笉鍚岀殑鐩爣鐜銆備緥濡傦紝鍦ㄥ瓨鍦ㄦ祴璇曠敤渚嬫垨澶嶇幇鍣ㄧ殑鎯呭喌涓嬶紝KASAN 鏄洿濂界殑璋冭瘯杈呭姪鎵嬫锛氱敱浜?KFENCE 妫€娴嬪埌閿欒鐨勬鐜囪緝浣庯紝浣跨敤 KFENCE 鏉ヨ皟璇曢渶瑕佹洿澶氱殑绮惧姏銆傜劧鑰岋紝閭ｄ簺鏃犳硶鎵挎媴鍚敤 KASAN 鎴愭湰鐨勫ぇ瑙勬ā閮ㄧ讲锛屽皢鍙楃泭浜庝娇鐢?KFENCE 鏉ュ彂鐜伴偅浜涙湭琚祴璇曠敤渚嬫垨妯＄硦娴嬭瘯鎵ц鍒扮殑浠ｇ爜璺緞涓殑缂洪櫡銆?