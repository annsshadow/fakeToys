## CoreSight 绯荤粺閰嶇疆绠＄悊鍣?

    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     October 2020

## 绠€浠?

CoreSight 绯荤粺閰嶇疆绠＄悊鍣紙System Configuration manager锛夋槸涓€涓?API锛屽畠鍏佽浣跨敤棰勫畾涔夌殑閰嶇疆瀵?CoreSight 绯荤粺杩涜缂栫▼锛岃繖浜涢厤缃殢鍚庡彲浠ヨ交鏉惧湴浠?sysfs 鎴?perf 鍚敤銆?
璁稿 CoreSight 缁勪欢鍙互浠ュ鏉傜殑鏂瑰紡缂栫▼ 鈥斺€?灏ゅ叾鏄?ETM銆傛澶栵紝缁勪欢涔嬮棿鍙互璺ㄨ秺 CoreSight 绯荤粺浜や簰锛岄€氬父缁忕敱浜ゅ弶瑙﹀彂缁勪欢锛堝 CTI 鍜?CTM锛夈€傝繖浜涚郴缁熻缃彲浠ヨ瀹氫箟涓哄懡鍚嶇殑閰嶇疆骞跺惎鐢ㄣ€?

## 鍩烘湰姒傚康


鏈妭浠嬬粛 CoreSight 绯荤粺閰嶇疆鐨勫熀鏈蹇点€?

### 鐗规€э紙Features锛?

涓€涓壒鎬э紙feature锛夋槸涓€缁勯拡瀵规煇涓?CoreSight 璁惧鐨勫懡鍚嶇紪绋嬮泦鍚堛€傜紪绋嬫槸璁惧鐩稿叧鐨勶紝鍙互鐢ㄧ粷瀵瑰瘎瀛樺櫒鍊笺€佽祫婧愪娇鐢ㄥ拰鍙傛暟鍊兼潵瀹氫箟銆?
鐗规€т娇鐢ㄤ竴涓弿杩扮锛坉escriptor锛夋潵瀹氫箟銆傝繖涓弿杩扮鐢ㄤ簬鍔犺浇鍒板尮閰嶇殑璁惧涓婏紝鏃犺鏄湪鐗规€ц鍔犺浇鍒扮郴缁熶腑鏃讹紝杩樻槸鍦?CoreSight 璁惧鍚戦厤缃鐞嗗櫒娉ㄥ唽鏃躲€?
鍔犺浇杩囩▼娑夊強灏嗘弿杩扮瑙ｉ噴涓洪┍鍔ㄤ腑鐨勪竴缁勫瘎瀛樺櫒璁块棶 鈥斺€?璧勬簮浣跨敤鍜屽弬鏁版弿杩拌杞崲涓虹浉搴旂殑瀵勫瓨鍣ㄨ闂€傝繖绉嶈В閲婁负鍦ㄩ渶瑕佹椂楂樻晥鍦板皢鐗规€х紪绋嬪埌璁惧涓婃彁渚涗簡渚垮埄銆?
鍦ㄧ壒鎬ц鍚敤涓旇澶囨湰韬篃琚惎鐢ㄤ箣鍓嶏紝璇ョ壒鎬т笉浼氬湪璁惧涓婂浜庢椿鍔ㄧ姸鎬併€傚綋璁惧琚惎鐢ㄦ椂锛屽凡鍚敤鐨勭壒鎬у皢琚紪绋嬪埌璁惧纭欢涓€?
涓€涓壒鎬ф槸浣滀负鍦ㄧ郴缁熶笂鍚敤鐨勬煇涓厤缃殑涓€閮ㄥ垎鑰岃鍚敤鐨勩€?

#### 鍙傛暟鍊硷紙Parameter Value锛?

鍙傛暟鍊兼槸涓€涓懡鍚嶇殑鍊硷紝鐢ㄦ埛鍙互鍦ㄧ壒鎬у惎鐢ㄤ箣鍓嶈缃畠锛屼互璋冩暣鐢辫鐗规€ф墍缂栫▼鐨勬搷浣滅殑琛屼负銆?
渚嬪锛岃繖鍙互鏄竴涓湪缁欏畾閫熺巼涓嬮噸澶嶇殑缂栫▼鎿嶄綔涓殑璁℃暟鍊笺€傚綋鐗规€ц鍚敤鏃讹紝鍙傛暟鐨勫綋鍓嶅€间細琚敤浜庤澶囩紪绋嬨€?
鐗规€ф弿杩扮涓哄弬鏁板畾涔変簡涓€涓粯璁ゅ€硷紝濡傛灉鐢ㄦ埛娌℃湁鎻愪緵鏂板€硷紝鍒欎娇鐢ㄩ粯璁ゅ€笺€?
鐢ㄦ埛鍙互浣跨敤 CoreSight 绯荤粺鐨?configfs API 鏇存柊鍙傛暟鍊?鈥斺€?涓嬫枃浼氬仛浠嬬粛銆?
褰撶壒鎬у湪璇ヨ澶囦笂鍚敤鏃讹紝鍙傛暟鐨勫綋鍓嶅€间細琚姞杞藉埌璁惧涓€?

### 閰嶇疆锛圕onfigurations锛?

涓€涓厤缃畾涔変簡涓€缁勭壒鎬э紝杩欎簺鐗规€у皢鍦ㄩ€夋嫨浜嗚閰嶇疆鐨勮窡韪細璇濅腑浣跨敤銆傚浜庝换浣曡窡韪細璇濓紝鍙兘閫夋嫨涓€涓厤缃€?
鎵€瀹氫箟鐨勭壒鎬у彲浠ヤ綅浜庝换浣曞凡娉ㄥ唽浠ユ敮鎸佺郴缁熼厤缃殑璁惧绫诲瀷涓娿€備竴涓厤缃彲浠ラ€夋嫨瑕佸湪鏌愮被璁惧涓婂惎鐢ㄧ殑鐗规€?鈥斺€?鍗充换浣?ETMv4锛屾垨鐗瑰畾璁惧锛屼緥濡傜郴缁熶笂鐨勬煇涓壒瀹?CTI銆?
涓庣壒鎬т竴鏍凤紝閰嶇疆涔熶娇鐢ㄤ竴涓弿杩扮鏉ュ畾涔夈€傝繖浼氬畾涔夊繀椤讳綔涓洪厤缃竴閮ㄥ垎鍚敤鐨勭壒鎬э紝浠ュ強浠讳綍鍙敤浜庤鐩栭粯璁ゅ弬鏁板€肩殑棰勭疆鍊笺€?

#### 棰勭疆鍊硷紙Preset Values锛?

棰勭疆鍊兼槸閰嶇疆鎵€鐢ㄧ壒鎬х殑鍙傛暟鍊间腑鏄撲簬閫夋嫨鐨勪竴缁勯泦鍚堛€傚崟涓缃泦鍚堜腑鐨勫€肩殑鏁伴噺锛岀瓑浜庨厤缃墍鐢ㄧ壒鎬х殑鍙傛暟鍊间箣鍜屻€?
渚嬪锛屼竴涓厤缃敱 3 涓壒鎬х粍鎴愶紝鍏朵腑涓€涓湁 2 涓弬鏁帮紝涓€涓湁 1 涓弬鏁帮紝鍙︿竴涓病鏈夊弬鏁般€傚洜姝わ紝鍗曚釜棰勭疆闆嗗悎灏嗘湁 3 涓€笺€?
棰勭疆鐢遍厤缃彲閫夊湴瀹氫箟锛屾渶澶氬彲浠ュ畾涔?15 涓€傚鏋滄湭閫夋嫨浠讳綍棰勭疆锛屽垯鐓у父浣跨敤鐗规€т腑瀹氫箟鐨勫弬鏁板€笺€?

#### 鎿嶄綔锛圤peration锛?

閰嶇疆鐨勬搷浣滀細缁忓巻浠ヤ笅姝ラ銆?
1) 鍦ㄦ绀轰緥涓紝閰嶇疆涓?'autofdo'锛屽畠鏈変竴涓叧鑱旂殑鐗规€?'strobing'锛屼綔鐢ㄤ簬 ETMv4 CoreSight 璁惧銆?
2) 閰嶇疆琚惎鐢ㄣ€備緥濡?'perf' 鍙互鍍忎笅闈㈣繖鏍烽€夋嫨
```
    perf record -e cs_etm/autofdo/ myapp
```

   杩欏皢鍚敤 'autofdo' 閰嶇疆銆?
3) perf 鍦ㄧ郴缁熶笂寮€濮嬭窡韪€傞殢鐫€ perf 鐢ㄤ簬璺熻釜鐨勬瘡涓?ETMv4 琚惎鐢紝閰嶇疆绠＄悊鍣ㄤ細妫€鏌ヨ ETMv4 鏄惁鍏锋湁涓庡綋鍓嶆椿鍔ㄩ厤缃浉鍏崇殑鐗规€с€傚湪杩欑鎯呭喌涓嬶紝'strobing' 琚惎鐢ㄥ苟缂栫▼鍒?ETMv4 涓€?
4) 褰?ETMv4 琚鐢ㄦ椂锛屼换浣曟爣璁颁负闇€瑕佷繚瀛樼殑瀵勫瓨鍣ㄥ皢琚鍥炪€?
5) 鍦?perf 浼氳瘽缁撴潫鏃讹紝閰嶇疆灏嗚绂佺敤銆?

## 鏌ョ湅閰嶇疆涓庣壒鎬?

褰撳墠鍔犺浇鍒扮郴缁熶腑鐨勯厤缃拰鐗规€ч泦鍚堝彲浠ヤ娇鐢?configfs API 鏌ョ湅銆?
```
    $ ls /config
    cs-syscfg  stp-policy
```

```
    $ cd cs-syscfg/
    $ ls
    configurations  features
```

绯荤粺鍐呯疆浜?'autofdo' 閰嶇疆銆傚彲浠ュ儚涓嬮潰杩欐牱妫€鏌ュ畠
```
    $ cd configurations/
    $ ls
    autofdo
    $ cd autofdo/
    $ ls
    description  feature_refs  preset1  preset3  preset5  preset7  preset9
    enable       preset        preset2  preset4  preset6  preset8
    $ cat description
    Setup ETMs with strobing for autofdo
    $ cat feature_refs
    strobing
```

姣忎釜澹版槑鐨勯缃兘鏈変竴涓?'preset<n>' 瀛愮洰褰曘€傚叾鍊煎
```
    $ cat preset1/values
    strobing.window = 0x1388 strobing.period = 0x2
    $ cat preset2/values
    strobing.window = 0x1388 strobing.period = 0x4
```

'enable' 鍜?'preset' 鏂囦欢鍏佽鍦ㄤ娇鐢?CoreSight 涓?sysfs 鏃舵帶鍒朵竴涓厤缃€?
閰嶇疆鎵€寮曠敤鐨勭壒鎬у彲浠ュ湪 features 涓鏌?```
    $ cd ../../features/strobing/
    $ ls
    description  matches  nr_params  params
    $ cat description
    Generate periodic trace capture windows.
    parameter 'window': a number of CPU cycles (W)
    parameter 'period': trace enabled for W cycles every period x W cycles
    $ cat matches
    SRC_ETMV4
    $ cat nr_params
    2
```

```
    cd params
    $ ls
    period  window
    $ cd period
    $ ls
    value
    $ cat value
    0x2710
    # echo 15000 > value
    # cat value
    0x3a98
```

浠ヨ繖绉嶆柟寮忚皟鏁寸殑鍙傛暟浼氬弽鏄犲埌鎵€鏈夊凡鍔犺浇璇ョ壒鎬х殑璁惧瀹炰緥涓€?

## 鍦?perf 涓娇鐢ㄩ厤缃?

鍔犺浇鍒?CoreSight 閰嶇疆绠＄悊涓殑閰嶇疆涔熶細鍦?perf 鐨?'cs_etm' 浜嬩欢鍩虹璁炬柦涓０鏄庯紝浠ヤ究瀹冧滑鍙互
```
    $ ls /sys/devices/cs_etm
    cpu0  cpu2  events  nr_addr_filters		power  subsystem  uevent
    cpu1  cpu3  format  perf_event_mux_interval_ms	sinks  type
```

杩欓噷鐨勫叧閿洰褰曟槸 'events' 鈥斺€?涓€涓€氱敤鐨?perf 鐩綍锛屽厑璁稿湪 perf 鍛戒护琛屼笂杩涜閫夋嫨銆備笌 sinks 鏉＄洰涓€鏍凤紝杩欐彁渚涗簡閰嶇疆鍚嶇О鐨勫搱甯屻€?
'events' 鐩綍涓殑鏉＄洰浣跨敤 perf 鍐呯疆鐨勮娉曠敓鎴愬櫒
```
    $ ls events/
    autofdo
    $ cat events/autofdo
    configid=0xa7c3dddd
```

```
    $ perf record -e cs_etm/autofdo/u --per-thread <application>
```

```
    $ perf record -e cs_etm/autofdo,preset=1/u --per-thread <application>
```

褰撲互杩欑鏂瑰紡閫夋嫨閰嶇疆鏃讹紝鎵€浣跨敤鐨勮窡韪帴鏀剁锛坰ink锛変細琚嚜鍔ㄩ€夋嫨銆?
## 鍦?sysfs 涓娇鐢ㄩ厤缃?

CoreSight 鍙互閫氳繃 sysfs 鎺у埗銆傚綋浣跨敤 sysfs 鏃讹紝鍙互浣夸竴涓厤缃 sysfs 浼氳瘽涓墍鐢ㄧ殑璁惧澶勪簬娲诲姩鐘舵€併€?
鍦ㄤ竴涓厤缃腑鏈?'enable' 鍜?'preset' 鏂囦欢銆?
```
    $ cd configurations/autofdo
    $ echo 1 > enable
```

杩欏皢浣跨敤鐗规€т腑鐨勪换浣曢粯璁ゅ弬鏁板€?鈥斺€?杩欎簺鍙傛暟鍊煎彲浠ュ涓婃墍杩拌繘琛岃皟鏁淬€?
```
    $ echo 3 > preset
```

杩欏皢涓洪厤缃€夋嫨 preset3銆俻reset 鐨勬湁鏁堝€间负 0 鈥斺€?鐢ㄤ簬鍙栨秷閫夋嫨棰勭疆锛屼互鍙婁换浣曞瓨鍦?preset<n> 瀛愮洰褰曠殑 <n> 鍊笺€?
璇锋敞鎰忥紝娲诲姩鐨?sysfs 閰嶇疆鏄竴涓叏灞€鍙傛暟锛屽洜姝?sysfs 鍦ㄤ换浣曟椂鍒诲彧鑳芥湁涓€涓椿鍔ㄩ厤缃€傚皾璇曞惎鐢ㄧ浜屼釜閰嶇疆灏嗗鑷撮敊璇€傛澶栵紝灏濊瘯鍦ㄨ浣跨敤鏃剁鐢ㄨ閰嶇疆涔熶細瀵艰嚧閿欒銆?
sysfs 瀵规椿鍔ㄩ厤缃殑浣跨敤涓?perf 涓娇鐢ㄧ殑閰嶇疆鏄嫭绔嬬殑銆?

## 鍒涘缓骞跺姞杞借嚜瀹氫箟閰嶇疆


鑷畾涔夐厤缃拰锛堟垨锛夌壒鎬у彲浠ラ€氳繃浣跨敤鍙姞杞芥ā鍧楀姩鎬佸湴鍔犺浇鍒扮郴缁熶腑銆?
鑷畾涔夐厤缃殑涓€涓ず渚嬩綅浜?./samples/coresight銆?
杩欎細鍒涘缓涓€涓柊鐨勯厤缃紝璇ラ厤缃娇鐢ㄧ幇鏈夌殑鍐呯疆 strobing 鐗规€э紝浣嗘彁渚涗簡涓€缁勪笉鍚岀殑棰勭疆銆?
褰撴ā鍧楄鍔犺浇鏃讹紝璇ラ厤缃細鍑虹幇鍦?configfs 鏂囦欢绯荤粺涓紝骞朵笖鍙互鍍忎笂鏂囨墍杩扮殑鍐呯疆閰嶇疆涓€鏍疯閫夋嫨銆?
閰嶇疆鍙互浣跨敤涔嬪墠宸插姞杞界殑鐗规€с€傜郴缁熶細纭繚鍦ㄥ綋鍓嶆鍦ㄤ娇鐢ㄧ殑鐗规€ф棤娉曡鍗歌浇锛屾柟娉曟槸寮哄埗鍗歌浇椤哄簭涓ユ牸涓哄姞杞介『搴忕殑閫嗗簭銆?