## 鍐呮牳涓殑 CPU 鐑彃鎷?

:Date: September, 2021
:Author: Sebastian Andrzej Siewior <bigeasy@linutronix.de>,
         Rusty Russell <rusty@rustcorp.com.au>,
         Srivatsa Vaddagiri <vatsa@in.ibm.com>,
         Ashok Raj <ashok.raj@intel.com>,
         Joel Schopp <jschopp@austin.ibm.com>,
	 Thomas Gleixner <tglx@kernel.org>

## 绠€浠?

鐜颁唬绯荤粺鏋舵瀯鐨勮繘姝ュ湪澶勭悊鍣ㄤ腑寮曞叆浜嗗厛杩涚殑閿欒鎶ュ憡涓庣籂姝ｈ兘鍔涖€?鏈変竴浜?OEM 鏀寔鍚屾牱鍙儹鎻掓嫈鐨?NUMA 纭欢锛屽叾涓墿鐞嗚妭鐐圭殑鎻掑叆涓庣Щ闄ら渶瑕?CPU 鐑彃鎷旂殑鏀寔銆?
姝ょ被杩涙瑕佹眰鍐呮牳鍙敤鐨?CPU 鍑轰簬璧勬簮璋冮厤鍘熷洜锛屾垨鍑轰簬 RAS 鐩殑锛堜娇鏈夐棶棰樼殑 CPU 杩滅绯荤粺鎵ц璺緞锛夎€岃绉婚櫎銆?鍥犳闇€瑕佸湪 Linux 鍐呮牳涓敮鎸?CPU 鐑彃鎷斻€?
CPU 鐑彃鎷旀敮鎸佷竴涓洿鏂伴鐨勭敤閫旀槸浠婂ぉ鍦?SMP 鐨勬寕璧?鎭㈠鏀寔涓殑浣跨敤銆?鍙屾牳涓庤秴绾跨▼锛圚T锛夋敮鎸佷娇寰楀嵆渚挎槸绗旇鏈數鑴戜篃鑳借繍琛屽師鏈笉鏀寔杩欎簺鏂规硶鐨?SMP 鍐呮牳銆?

## 鍛戒护琛屽紑鍏?

`maxcpus=n`
  灏嗗惎鍔ㄦ椂鐨?CPU 闄愬埗涓?**n**銆備緥濡傦紝濡傛灉浣犳湁鍥涢 CPU锛屼娇鐢?  `maxcpus=2` 灏嗗彧鍚姩涓ら銆備綘鍙互閫夋嫨绋嶅悗灏?  鍏朵粬 CPU 涓婄嚎銆?
`nr_cpus=n`
  闄愬埗鍐呮牳灏嗘敮鎸佺殑 CPU 鎬绘暟銆傚鏋滄澶勬彁渚涚殑鏁板瓧浣庝簬鐗╃悊鍙敤 CPU 鐨勬暟閲忥紝閭ｄ箞
  杩欎簺 CPU 绋嶅悗涔熸棤娉曡涓婄嚎銆?
`possible_cpus=n`
  姝ら€夐」鍦?`cpu_possible_mask` 涓缃?`possible_cpus` 浣嶃€?
  姝ら€夐」浠呴檺浜?X86 涓?S390 鏋舵瀯銆?
`cpu0_hotplug`
  鍏佽鍏抽棴 CPU0銆?
  姝ら€夐」浠呴檺浜?X86 鏋舵瀯銆?
## CPU 鏄犲皠


`cpu_possible_mask`
  绯荤粺涓浘缁忓彲鐢ㄧ殑鍙兘 CPU 鐨勪綅鍥俱€傝繖鐢ㄤ簬鍦ㄥ惎鍔ㄦ椂涓洪偅浜涘苟闈炶璁℃垚闅?CPU 鐨?  鍙敤鎴栫Щ闄よ€屽闀?鏀剁缉鐨?per_cpu 鍙橀噺鍒嗛厤涓€浜涘惎鍔ㄦ湡鍐呭瓨銆?  涓€鏃﹀湪鍚姩鏈熺殑鍙戠幇闃舵璁剧疆锛岃鏄犲皠灏辨槸闈欐€佺殑锛屽嵆浠讳綍鏃跺€欓兘涓嶄細娣诲姞鎴栫Щ闄や綅銆?  涓轰綘鐨勭郴缁熼渶姹傛彁鍓嶇簿纭湴瑁佸壀瀹冨彲浠ヨ妭鐪佷竴浜涘惎鍔ㄦ湡鍐呭瓨銆?
`cpu_online_mask`
  褰撳墠鎵€鏈夊湪绾?CPU 鐨勪綅鍥俱€傚畠鍦ㄤ竴涓?CPU 鍙敤浜庡唴鏍歌皟搴﹀苟鍑嗗濂芥帴鏀舵潵鑷澶囩殑
  涓柇涔嬪悗锛屼簬 `__cpu_up()` 涓缃€傚綋涓€涓?CPU 閫氳繃
  `__cpu_disable()` 琚叧闂椂锛屽畠琚竻闄わ紝鍦ㄦ涔嬪墠鐨勫寘鎷腑鏂湪鍐呯殑鎵€鏈?OS 鏈嶅姟
  閮借杩佺Щ鍒板彟涓€涓洰鏍?CPU銆?
`cpu_present_mask`
  褰撳墠绯荤粺涓瓨鍦ㄧ殑 CPU 鐨勪綅鍥俱€傚苟闈炴墍鏈?  鐨勫畠浠兘鍦ㄧ嚎銆傚綋鐩稿叧鐨勫瓙绯荤粺锛堜緥濡?ACPI锛夊鐞嗙墿鐞嗙儹鎻掓嫈鏃讹紝浼氭牴鎹簨浠舵槸鐑坊鍔?鐑Щ闄?  鑰屾敼鍙橈紝浠庢槧灏勪腑鏂板鎴栫Щ闄や竴涓綅銆傜洰鍓嶆病鏈変换浣曢攣瀹氳鍒欍€?  鍏稿瀷鐢ㄦ硶鏄湪鍚姩鏃跺垵濮嬪寲鎷撴墤锛屾鏃剁儹鎻掓嫈琚鐢ㄣ€?
浣犵湡鐨勪笉闇€瑕佸幓鎿嶇旱浠讳綍绯荤粺 CPU 鏄犲皠銆傚浜庡ぇ澶氭暟鐢ㄩ€旓紝瀹冧滑搴斿綋鏄彧璇荤殑銆?鍦ㄥ缓绔?per-cpu 璧勬簮鏃讹紝鍑犱箮鎬绘槸浣跨敤 `cpu_possible_mask` 鎴?`for_each_possible_cpu()`
鏉ヨ凯浠ｃ€傚畯 `for_each_cpu()` 鍙敤浜庤凯浠ｄ竴涓嚜瀹氫箟鐨?CPU 鎺╃爜銆?
闄や簡 `cpumask_t` 涔嬪锛岀粷涓嶈浣跨敤浠讳綍鍏朵粬涓滆タ鏉ヨ〃绀?CPU 鐨勪綅鍥俱€?

## 浣跨敤 CPU 鐑彃鎷?

闇€瑕佸惎鐢ㄥ唴鏍搁€夐」 **CONFIG_HOTPLUG_CPU**銆傚畠鐩墠鍦ㄥ寘鎷?ARM銆丮IPS銆丳owerPC 涓?X86 鍦ㄥ唴鐨勫绉嶆灦鏋勪笂鍙敤銆?```
 $ ls -lh /sys/devices/system/cpu
 total 0
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu0
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu1
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu2
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu3
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu4
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu5
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu6
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu7
 drwxr-xr-x  2 root root    0 Dec 21 16:33 hotplug
 -r--r--r--  1 root root 4.0K Dec 21 16:33 offline
 -r--r--r--  1 root root 4.0K Dec 21 16:33 online
 -r--r--r--  1 root root 4.0K Dec 21 16:33 possible
 -r--r--r--  1 root root 4.0K Dec 21 16:33 present
```
鏂囦欢 **offline**銆?*online**銆?*possible**銆?*present** 浠ｈ〃 CPU 鎺╃爜銆?姣忎釜 CPU 鏂囦欢澶归兘鍖呭惈涓€涓?**online** 鏂囦欢锛屽畠鎺у埗閫昏緫涓婄殑寮€锛?锛変笌
```
 $ echo 0 > /sys/devices/system/cpu/cpu4/online
  smpboot: CPU 4 is now offline
```
涓€鏃?CPU 琚叧闂紝瀹冨皢浠?**/proc/interrupts**銆?**/proc/cpuinfo** 涓Щ闄わ紝骞朵笖涔熶笉搴斿啀琚?**top** 鍛戒护鍙銆傝
```
 $ echo 1 > /sys/devices/system/cpu/cpu4/online
 smpboot: Booting Node 0 Processor 4 APIC 0x1
```
CPU 鍐嶆鍙敤銆傝繖搴斿綋閫傜敤浜庢墍鏈?CPU锛屼絾 CPU0 閫氬父姣旇緝鐗规畩锛岃鎺掗櫎鍦?CPU 鐑彃鎷斾箣澶栥€?
## CPU 鐑彃鎷斿崗璋?

### 绂荤嚎鎯呭舰


涓€鏃︿竴涓?CPU 琚€昏緫涓婂叧闂紝宸叉敞鍐岀殑
鐑彃鎷旂姸鎬佺殑鎷嗛櫎鍥炶皟灏变細琚皟鐢紝浠?`CPUHP_ONLINE` 寮€濮嬶紝鍒扮姸鎬?`CPUHP_OFFLINE` 缁撴潫銆傝繖鍖呮嫭锛?
- 濡傛灉鐢变簬鎸傝捣鎿嶄綔瀵艰嚧浠诲姟琚喕缁擄紝鍒?**cpuhp_tasks_frozen**
  浼氳璁句负 true銆?- 鎵€鏈夎繘绋嬮兘浠庤绂荤嚎鐨?CPU 杩佺Щ鍒版柊鐨?CPU銆?  鏂扮殑 CPU 浠庢瘡涓繘绋嬪綋鍓嶇殑 cpuset 涓€夋嫨锛屽畠鍙兘鏄湪绾?CPU 鐨勪竴涓瓙闆嗐€?- 鎵€鏈変互璇?CPU 涓虹洰鏍囩殑涓柇閮借杩佺Щ鍒颁竴涓柊鐨?CPU
- 瀹氭椂鍣ㄤ篃琚縼绉诲埌涓€涓柊鐨?CPU
- 涓€鏃︽墍鏈夋湇鍔￠兘琚縼绉伙紝鍐呮牳璋冪敤涓€涓?arch 鐗瑰畾鐨勪緥绋?  `__cpu_disable()` 鏉ユ墽琛?arch 鐗瑰畾鐨勬竻鐞嗐€?

## CPU 鐑彃鎷?API


### CPU 鐑彃鎷旂姸鎬佹満


CPU 鐑彃鎷斾娇鐢ㄤ竴涓粠 CPUHP_OFFLINE 鍒?CPUHP_ONLINE 鐨勭嚎鎬х姸鎬佺┖闂寸殑绠€鍗曠姸鎬佹満銆?姣忎釜鐘舵€侀兘鏈変竴涓惎鍔紙startup锛夊拰涓€涓媶闄わ紙teardown锛夊洖璋冦€?
褰撲竴涓?CPU 琚笂绾挎椂锛屽惎鍔ㄥ洖璋冭椤哄簭璋冪敤锛岀洿鍒拌揪鍒扮姸鎬?CPUHP_ONLINE銆?褰撴煇涓姸鎬佺殑鍥炶皟琚缓绔嬶紝鎴栬€呬竴涓疄渚嬭娣诲姞鍒颁竴涓瀹炰緥鐘舵€佹椂锛屽畠浠篃鍙互琚皟鐢ㄣ€?
褰撲竴涓?CPU 琚绾挎椂锛屾媶闄ゅ洖璋冧互鐩稿弽鐨勯『搴忚椤哄簭璋冪敤锛岀洿鍒拌揪鍒扮姸鎬?CPUHP_OFFLINE銆?褰撴煇涓姸鎬佺殑鍥炶皟琚Щ闄わ紝鎴栬€呬竴涓疄渚嬩粠澶氬疄渚嬬姸鎬佽绉婚櫎鏃讹紝瀹冧滑涔熷彲浠ヨ璋冪敤銆?
濡傛灉涓€涓娇鐢ㄧ偣鍙渶瑕佸湪鐑彃鎷旀搷浣滅殑涓€涓柟鍚戯紙CPU 涓婄嚎鎴?CPU 绂荤嚎锛変笂鏈変竴涓洖璋冿紝
閭ｄ箞鍙︿竴涓笉闇€瑕佺殑鍥炶皟鍙互鍦ㄥ缓绔嬭鐘舵€佹椂璁句负 NULL銆?
鐘舵€佺┖闂磋鍒掑垎涓轰笁涓儴鍒嗭細

- PREPARE 閮ㄥ垎

  PREPARE 閮ㄥ垎瑕嗙洊浠?CPUHP_OFFLINE 鍒?  CPUHP_BRINGUP_CPU 鐨勭姸鎬佺┖闂淬€?
  璇ラ儴鍒嗕腑鐨勫惎鍔ㄥ洖璋冨湪 CPU 涓婄嚎鎿嶄綔鏈熼棿銆丆PU 鍚姩涔嬪墠琚皟鐢ㄣ€?  鎷嗛櫎鍥炶皟鍦?CPU 绂荤嚎鎿嶄綔鏈熼棿銆丆PU 鍙樺緱涓嶅彲鐢ㄤ箣鍚庤璋冪敤銆?
  鍥炶皟鍦ㄤ竴涓帶鍒?CPU 涓婅璋冪敤锛屽洜涓哄畠浠樉鐒舵棤娉曡繍琛屽湪瑕佷箞灏氭湭鍚姩銆佽涔堝凡缁忓彉寰椾笉鍙敤鐨勭儹鎻掓嫈 CPU 涓娿€?
  鍚姩鍥炶皟鐢ㄤ簬寤虹珛鎴愬姛璁╀竴涓?CPU 涓婄嚎鎵€闇€鐨勮祫婧愩€傛媶闄ゅ洖璋冪敤浜庡湪鐑彃鎷?CPU 鍙樺緱涓嶅彲鐢ㄤ箣鍚庨噴鏀捐祫婧愶紝鎴栬€呭皢寰呭鐞嗙殑宸ヤ綔绉诲姩鍒颁竴涓湪绾跨殑 CPU銆?
  鍚姩鍥炶皟鍏佽澶辫触銆傚鏋滄煇涓洖璋冨け璐ワ紝CPU 涓婄嚎鎿嶄綔琚腑姝紝骞朵笖璇?CPU 鍐嶆琚檷鍒颁箣鍓嶇殑鐘舵€侊紙閫氬父鏄?CPUHP_OFFLINE锛夈€?
  璇ラ儴鍒嗕腑鐨勬媶闄ゅ洖璋冧笉鍏佽澶辫触銆?
- STARTING 閮ㄥ垎

  STARTING 閮ㄥ垎瑕嗙洊鍦?CPUHP_BRINGUP_CPU + 1
  涓?CPUHP_AP_ONLINE 涔嬮棿鐨勭姸鎬佺┖闂淬€?
  璇ラ儴鍒嗕腑鐨勫惎鍔ㄥ洖璋冨湪 CPU 涓婄嚎鎿嶄綔鏈熼棿鐨勬棭鏈?CPU 璁剧疆浠ｇ爜涓紝鍦ㄤ腑鏂鐢ㄧ殑鎯呭喌涓嬶紝浜庣儹鎻掓嫈 CPU 涓婅璋冪敤銆?  鎷嗛櫎鍥炶皟鍦?CPU 绂荤嚎鎿嶄綔鏈熼棿銆丆PU 瀹屽叏鍏抽棴鍓嶄笉涔咃紝鍦ㄤ腑鏂鐢ㄧ殑鎯呭喌涓嬶紝浜庣儹鎻掓嫈 CPU 涓婅璋冪敤銆?
  璇ラ儴鍒嗕腑鐨勫洖璋冧笉鍏佽澶辫触銆?
  鍥炶皟鐢ㄤ簬搴曞眰纭欢鐨勫垵濮嬪寲/鍏抽棴浠ュ強鏍稿績瀛愮郴缁熴€?
- ONLINE 閮ㄥ垎

  ONLINE 閮ㄥ垎瑕嗙洊鍦?CPUHP_AP_ONLINE + 1 涓?  CPUHP_ONLINE 涔嬮棿鐨勭姸鎬佺┖闂淬€?
  璇ラ儴鍒嗕腑鐨勫惎鍔ㄥ洖璋冨湪 CPU 涓婄嚎鎿嶄綔鏈熼棿锛屼簬鐑彃鎷?CPU 涓婅璋冪敤銆?  鎷嗛櫎鍥炶皟鍦?CPU 绂荤嚎鎿嶄綔鏈熼棿锛屼簬鐑彃鎷?CPU 涓婅璋冪敤銆?
  鍥炶皟鍦?per CPU 鐑彃鎷旂嚎绋嬬殑涓婁笅鏂囦腑琚皟鐢紝璇ョ嚎绋嬭鍥哄畾鍦ㄧ儹鎻掓嫈 CPU 涓娿€?  鍥炶皟鍦ㄤ腑鏂笌鎶㈠崰鍧囧惎鐢ㄧ殑鎯呭喌涓嬭璋冪敤銆?
  鍥炶皟鍏佽澶辫触銆傚綋鏌愪釜鍥炶皟澶辫触鏃讹紝鐑彃鎷旀搷浣滆涓锛屽苟涓?CPU 琚甫鍥炰箣鍓嶇殑鐘舵€併€?
### CPU 涓婄嚎/绂荤嚎鎿嶄綔


```
  [CPUHP_OFFLINE]
  [CPUHP_OFFLINE + 1]->startup()       -> success
  [CPUHP_OFFLINE + 2]->startup()       -> success
  [CPUHP_OFFLINE + 3]                  -> skipped because startup == NULL
  ...
  [CPUHP_BRINGUP_CPU]->startup()       -> success
  === End of PREPARE section
  [CPUHP_BRINGUP_CPU + 1]->startup()   -> success
  ...
  [CPUHP_AP_ONLINE]->startup()         -> success
  === End of STARTUP section
  [CPUHP_AP_ONLINE + 1]->startup()     -> success
  ...
  [CPUHP_ONLINE - 1]->startup()        -> success
  [CPUHP_ONLINE]
```
```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_AP_ONLINE + 1]->teardown()    -> success
  === Start of STARTUP section
  [CPUHP_AP_ONLINE]->teardown()        -> success
  ...
  [CPUHP_BRINGUP_ONLINE - 1]->teardown()
  ...
  === Start of PREPARE section
  [CPUHP_BRINGUP_CPU]->teardown()
  [CPUHP_OFFLINE + 3]->teardown()
  [CPUHP_OFFLINE + 2]                  -> skipped because teardown == NULL
  [CPUHP_OFFLINE + 1]->teardown()
  [CPUHP_OFFLINE]
```
```
  [CPUHP_OFFLINE]
  [CPUHP_OFFLINE + 1]->startup()       -> success
  [CPUHP_OFFLINE + 2]->startup()       -> success
  [CPUHP_OFFLINE + 3]                  -> skipped because startup == NULL
  ...
  [CPUHP_BRINGUP_CPU]->startup()       -> success
  === End of PREPARE section
  [CPUHP_BRINGUP_CPU + 1]->startup()   -> success
  ...
  [CPUHP_AP_ONLINE]->startup()         -> success
  === End of STARTUP section
  [CPUHP_AP_ONLINE + 1]->startup()     -> success
  ---
  [CPUHP_AP_ONLINE + N]->startup()     -> fail
  [CPUHP_AP_ONLINE + (N - 1)]->teardown()
  ...
  [CPUHP_AP_ONLINE + 1]->teardown()
  === Start of STARTUP section
  [CPUHP_AP_ONLINE]->teardown()
  ...
  [CPUHP_BRINGUP_ONLINE - 1]->teardown()
  ...
  === Start of PREPARE section
  [CPUHP_BRINGUP_CPU]->teardown()
  [CPUHP_OFFLINE + 3]->teardown()
  [CPUHP_OFFLINE + 2]                  -> skipped because teardown == NULL
  [CPUHP_OFFLINE + 1]->teardown()
  [CPUHP_OFFLINE]
```
```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()
  ...
  [CPUHP_ONLINE - 1]->startup()
  [CPUHP_ONLINE]
```
閫掑綊澶辫触鏃犳硶琚槑鏅哄湴澶勭悊銆傝鐪嬩互涓?```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()  -> success
  [CPUHP_ONLINE - (N - 2)]->startup()  -> fail
```
CPU 鐑彃鎷旂姸鎬佹満浼氬氨鍋滃湪杩欓噷锛屼笉鍐嶅皾璇曞洖閫€
```
  [CPUHP_ONLINE - (N - 1)]->teardown() -> success
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()  -> success
  [CPUHP_ONLINE - (N - 2)]->startup()  -> fail
  [CPUHP_ONLINE - (N - 1)]->teardown() -> success
  [CPUHP_ONLINE - N]->teardown()       -> fail
```
```
  [CPUHP_ONLINE - (N - 1)]
```
杩欒嚦灏戣绯荤粺鑳藉鍙栧緱杩涘睍锛屽苟缁欑敤鎴蜂竴涓皟璇曠敋鑷宠В鍐宠鎯呭喌鐨勬満浼氥€?
### 鍒嗛厤涓€涓姸鎬?

鍒嗛厤涓€涓?CPU 鐑彃鎷旂姸鎬佹湁涓ょ鏂瑰紡锛?
- 闈欐€佸垎閰?
  褰撳瓙绯荤粺鎴栭┍鍔ㄧ浉瀵逛簬鍏朵粬 CPU 鐑彃鎷旂姸鎬佹湁鎺掑簭瑕佹眰鏃讹紝蹇呴』浣跨敤闈欐€佸垎閰嶃€?  渚嬪锛孭ERF 鏍稿績鐨勫惎鍔ㄥ洖璋冨繀椤诲湪 CPU 涓婄嚎鎿嶄綔鏈熼棿 PERF 椹卞姩鐨勫惎鍔ㄥ洖璋冧箣鍓嶈璋冪敤銆?  鍦?CPU 绂荤嚎鎿嶄綔鏈熼棿锛岄┍鍔ㄧ殑鎷嗛櫎鍥炶皟蹇呴』鍦ㄦ牳蹇冩媶闄ゅ洖璋冧箣鍓嶈璋冪敤銆?  闈欐€佸垎閰嶇殑鐘舵€佺敱 cpuhp_state 鏋氫妇涓殑甯搁噺鎻忚堪锛岃鏋氫妇鍙湪 include/linux/cpuhotplug.h 涓壘鍒般€?
  灏嗙姸鎬佹彃鍏ュ埌鏋氫妇涓殑鎭板綋浣嶇疆锛屼互婊¤冻鎺掑簭瑕佹眰銆傝鐘舵€佸父閲忓繀椤荤敤浜庣姸鎬佺殑寤虹珛涓庣Щ闄ゃ€?
  褰撶姸鎬佸洖璋冧笉鏄湪杩愯鏃跺缓绔嬶紝鑰屾槸 kernel/cpu.c 涓?CPU 鐑彃鎷旂姸鎬佹暟缁勭殑鍒濆鍖栧櫒鐨勪竴閮ㄥ垎鏃讹紝涔熼渶瑕侀潤鎬佸垎閰嶃€?
- 鍔ㄦ€佸垎閰?
  褰撶姸鎬佸洖璋冩病鏈夋帓搴忚姹傛椂锛屽姩鎬佸垎閰嶆槸棣栭€夋柟娉曘€傜姸鎬佸彿鐢卞缓绔嬪嚱鏁板垎閰嶏紝骞跺湪鎴愬姛鏃惰繑鍥炵粰璋冪敤鑰呫€?
  鍙湁 PREPARE 涓?ONLINE 閮ㄥ垎鎻愪緵鍔ㄦ€佸垎閰嶈寖鍥淬€係TARTING 閮ㄥ垎涓嶆彁渚涳紝鍥犱负璇ラ儴鍒嗕腑鐨勫ぇ澶氭暟鍥炶皟閮芥湁鏄惧紡鐨勬帓搴忚姹傘€?
### 寤虹珛涓€涓?CPU 鐑彃鎷旂姸鎬?

鏍稿績浠ｇ爜鎻愪緵浠ヤ笅鍑芥暟鏉ュ缓绔嬩竴涓姸鎬侊細

- cpuhp_setup_state(state, name, startup, teardown)
- cpuhp_setup_state_nocalls(state, name, startup, teardown)
- cpuhp_setup_state_cpuslocked(state, name, startup, teardown)
- cpuhp_setup_state_nocalls_cpuslocked(state, name, startup, teardown)

瀵逛簬椹卞姩鎴栧瓙绯荤粺鏈夊涓疄渚嬨€佸苟涓旂浉鍚岀殑 CPU 鐑彃鎷旂姸鎬佸洖璋冮渶瑕佸姣忎釜瀹炰緥閮借皟鐢ㄧ殑鎯呭喌锛?CPU 鐑彃鎷旀牳蹇冩彁渚涘瀹炰緥鏀寔銆傜浉瀵逛簬椹卞姩鐗瑰畾鐨勫疄渚嬪垪琛紝鍏朵紭鍔垮湪浜庡疄渚嬬浉鍏冲嚱鏁板畬鍏ㄩ拡瀵?CPU 鐑彃鎷旀搷浣滆涓茶鍖栵紝骞朵笖鎻愪緵鍦ㄦ坊鍔犱笌绉婚櫎鏃剁姸鎬佸洖璋冪殑鑷姩璋冪敤銆?瑕佸缓绔嬭繖鏍蜂竴涓瀹炰緥鐘舵€侊紝鍙娇鐢ㄤ互涓嬪嚱鏁帮細

- cpuhp_setup_state_multi(state, name, startup, teardown)

@state 鍙傛暟瑕佷箞鏄潤鎬佸垎閰嶇殑鐘舵€侊紝瑕佷箞鏄姩鎬佸垎閰嶇姸鎬佺殑甯搁噺涔嬩竴鈥斺€擟PUHP_BP_PREPARE_DYN銆?CPUHP_AP_ONLINE_DYN鈥斺€斿彇鍐充簬搴斾负鍏跺垎閰嶅姩鎬佺姸鎬佺殑閭ｄ釜鐘舵€侀儴鍒嗭紙PREPARE銆丱NLINE锛夈€?
@name 鍙傛暟鐢ㄤ簬 sysfs 杈撳嚭涓庢彃妗┿€傚懡鍚嶇害瀹氭槸 "subsys:mode" 鎴?"subsys/driver:mode"锛?渚嬪 "perf:mode" 鎴?"perf/x86:mode"銆傚父瑙佺殑 mode 鍚嶇О鏈夛細

======== =======================================================
prepare  For states in the PREPARE section

dead     For states in the PREPARE section which do not provide
         a startup callback

starting For states in the STARTING section

dying    For states in the STARTING section which do not provide
         a startup callback

online   For states in the ONLINE section

offline  For states in the ONLINE section which do not provide
         a startup callback
======== =======================================================

鐢变簬 @name 鍙傛暟浠呯敤浜?sysfs 涓庢彃妗╋紝濡傛灉鍏朵粬 mode 鎻忚堪绗︽瘮甯歌鐨勯偅浜涙洿鑳芥弿杩扮姸鎬佺殑鎬ц川锛屼篃鍙互浣跨敤瀹冧滑銆?
@name 鍙傛暟鐨勭ず渚嬶細"perf/online"銆?perf/x86:prepare"銆?"RCU/tree:dying"銆?sched/waitempty"

@startup 鍙傛暟鏄竴涓嚱鏁版寚閽堬紝鎸囧悜閭ｄ釜搴斿湪 CPU 涓婄嚎鎿嶄綔鏈熼棿琚皟鐢ㄧ殑鍥炶皟銆?濡傛灉浣跨敤鐐逛笉闇€瑕佸惎鍔ㄥ洖璋冿紝灏嗘寚閽堣涓?NULL銆?
@teardown 鍙傛暟鏄竴涓嚱鏁版寚閽堬紝鎸囧悜閭ｄ釜搴斿湪 CPU 绂荤嚎鎿嶄綔鏈熼棿琚皟鐢ㄧ殑鍥炶皟銆?濡傛灉浣跨敤鐐逛笉闇€瑕佹媶闄ゅ洖璋冿紝灏嗘寚閽堣涓?NULL銆?
杩欎簺鍑芥暟鍦ㄦ墍瀹夎鐨勫洖璋冭瀵瑰緟鐨勬柟寮忎笂鏈夋墍涓嶅悓锛?
  - cpuhp_setup_state_nocalls()銆乧puhp_setup_state_nocalls_cpuslocked()
    浠ュ強 cpuhp_setup_state_multi() 鍙畨瑁呭洖璋?
  - cpuhp_setup_state() 涓?cpuhp_setup_state_cpuslocked() 瀹夎鍥炶皟锛屽苟閽堝褰撳墠鐘舵€佸彿澶т簬鏂板畨瑁呯姸鎬佺殑
    鎵€鏈夊湪绾?CPU 璋冪敤 @startup 鍥炶皟锛堝鏋滈潪 NULL锛夈€傛牴鎹姸鎬侀儴鍒嗭紝璇ュ洖璋冭涔堝湪褰撳墠 CPU锛圥REPARE 閮ㄥ垎锛?    涓婅璋冪敤锛岃涔堝湪姣忎釜鍦ㄧ嚎 CPU锛圤NLINE 閮ㄥ垎锛変笂鐨?CPU 鐑彃鎷旂嚎绋嬩笂涓嬫枃涓璋冪敤銆?
    濡傛灉鏌愪釜鍥炶皟瀵?CPU N 澶辫触锛屽垯璋冪敤 CPU 0 .. N-1 鐨勬媶闄ゅ洖璋冩潵鍥炴粴璇ユ搷浣溿€?    鐘舵€佸缓绔嬪け璐ワ紝璇ョ姸鎬佺殑鍥炶皟涓嶈瀹夎锛屽苟涓斿浜庡姩鎬佸垎閰嶏紝鎵€鍒嗛厤鐨勭姸鎬佽閲婃斁銆?
鐘舵€佸缓绔嬩笌鍥炶皟璋冪敤鐩稿浜?CPU 鐑彃鎷旀搷浣滆涓茶鍖栥€?濡傛灉寤虹珛鍑芥暟蹇呴』浠庝竴涓?CPU 鐑彃鎷旇閿佸畾鍖哄煙璋冪敤锛屽垯蹇呴』浣跨敤 _cpuslocked() 鍙樹綋銆?杩欎簺鍑芥暟涓嶈兘浠?CPU 鐑彃鎷斿洖璋冨唴閮ㄤ娇鐢ㄣ€?
鍑芥暟杩斿洖鍊硷細
  ======== ===================================================================
  0        Statically allocated state was successfully set up

  >0       Dynamically allocated state was successfully set up.

           The returned number is the state number which was allocated. If
           the state callbacks have to be removed later, e.g. module
           removal, then this number has to be saved by the caller and used
           as @state argument for the state remove function. For
           multi-instance states the dynamically allocated state number is
           also required as @state argument for the instance add/remove
           operations.

  <0	   Operation failed
  ======== ===================================================================

### 绉婚櫎涓€涓?CPU 鐑彃鎷旂姸鎬?

瑕佺Щ闄や竴涓箣鍓嶅凡寤虹珛鐨勭姸鎬侊紝鎻愪緵浠ヤ笅鍑芥暟锛?
- cpuhp_remove_state(state)
- cpuhp_remove_state_nocalls(state)
- cpuhp_remove_state_nocalls_cpuslocked(state)
- cpuhp_remove_multi_state(state)

@state 鍙傛暟瑕佷箞鏄潤鎬佸垎閰嶇殑鐘舵€侊紝瑕佷箞鏄敱 cpuhp_setup_state*() 鍦ㄥ姩鎬佽寖鍥村唴鍒嗛厤鐨勭姸鎬佸彿銆?濡傛灉璇ョ姸鎬佸湪鍔ㄦ€佽寖鍥村唴锛屽垯璇ョ姸鎬佸彿琚噴鏀撅紝骞跺彲鍐嶆鐢ㄤ簬鍔ㄦ€佸垎閰嶃€?
杩欎簺鍑芥暟鍦ㄦ墍瀹夎鐨勫洖璋冭瀵瑰緟鐨勬柟寮忎笂鏈夋墍涓嶅悓锛?
  - cpuhp_remove_state_nocalls()銆乧puhp_remove_state_nocalls_cpuslocked()
    浠ュ強 cpuhp_remove_multi_state() 鍙Щ闄ゅ洖璋冦€?
  - cpuhp_remove_state() 绉婚櫎鍥炶皟锛屽苟閽堝褰撳墠鐘舵€佸彿澶т簬琚Щ闄ょ姸鎬佺殑
    鎵€鏈夊湪绾?CPU 璋冪敤鎷嗛櫎鍥炶皟锛堝鏋滈潪 NULL锛夈€傛牴鎹姸鎬侀儴鍒嗭紝璇ュ洖璋冭涔堝湪褰撳墠 CPU锛圥REPARE 閮ㄥ垎锛?    涓婅璋冪敤锛岃涔堝湪姣忎釜鍦ㄧ嚎 CPU锛圤NLINE 閮ㄥ垎锛変笂鐨?CPU 鐑彃鎷旂嚎绋嬩笂涓嬫枃涓璋冪敤銆?
    涓轰簡瀹屾垚绉婚櫎锛屾媶闄ゅ洖璋冧笉搴斿け璐ャ€?
鐘舵€佺Щ闄や笌鍥炶皟璋冪敤鐩稿浜?CPU 鐑彃鎷旀搷浣滆涓茶鍖栥€?濡傛灉绉婚櫎鍑芥暟蹇呴』浠庝竴涓?CPU 鐑彃鎷旇閿佸畾鍖哄煙璋冪敤锛屽垯蹇呴』浣跨敤 _cpuslocked() 鍙樹綋銆?杩欎簺鍑芥暟涓嶈兘浠?CPU 鐑彃鎷斿洖璋冨唴閮ㄤ娇鐢ㄣ€?
濡傛灉绉婚櫎涓€涓瀹炰緥鐘舵€侊紝鍒欒皟鐢ㄨ€呭繀椤诲厛绉婚櫎鎵€鏈夊疄渚嬨€?
### 澶氬疄渚嬬姸鎬佸疄渚嬬鐞?

涓€鏃﹀瀹炰緥鐘舵€佽寤虹珛锛屽氨鍙互鍚戣鐘舵€佹坊鍔犲疄渚嬶細

  - cpuhp_state_add_instance(state, node)
  - cpuhp_state_add_instance_nocalls(state, node)

@state 鍙傛暟瑕佷箞鏄潤鎬佸垎閰嶇殑鐘舵€侊紝瑕佷箞鏄敱 cpuhp_setup_state_multi() 鍦ㄥ姩鎬佽寖鍥村唴鍒嗛厤鐨勭姸鎬佸彿銆?
@node 鍙傛暟鏄竴涓寚鍚?hlist_node 鐨勬寚閽堬紝璇?hlist_node 琚祵鍏ュ湪瀹炰緥鐨勬暟鎹粨鏋勪腑銆?璇ユ寚閽堣浜ょ粰澶氬疄渚嬬姸鎬佺殑鍥炶皟锛屽苟涓斿彲琚洖璋冮€氳繃 container_of() 鐢ㄦ潵鍙栧洖璇ュ疄渚嬨€?
杩欎簺鍑芥暟鍦ㄦ墍瀹夎鐨勫洖璋冭瀵瑰緟鐨勬柟寮忎笂鏈夋墍涓嶅悓锛?
  - cpuhp_state_add_instance_nocalls() 鍙皢瀹炰緥娣诲姞鍒?    澶氬疄渚嬬姸鎬佺殑鑺傜偣鍒楄〃銆?
  - cpuhp_state_add_instance() 娣诲姞瀹炰緥锛屽苟閽堝褰撳墠鐘舵€佸彿澶т簬 @state 鐨?    鎵€鏈夊湪绾?CPU 璋冪敤涓?@state 鍏宠仈鐨勫惎鍔ㄥ洖璋冿紙濡傛灉闈?NULL锛夈€傝鍥炶皟鍙拡瀵硅娣诲姞鐨勫疄渚嬭璋冪敤銆?    鏍规嵁鐘舵€侀儴鍒嗭紝璇ュ洖璋冭涔堝湪褰撳墠 CPU锛圥REPARE 閮ㄥ垎锛変笂琚皟鐢紝瑕佷箞鍦ㄦ瘡涓湪绾?CPU锛圤NLINE 閮ㄥ垎锛?    涓婄殑 CPU 鐑彃鎷旂嚎绋嬩笂涓嬫枃涓璋冪敤銆?
    濡傛灉鏌愪釜鍥炶皟瀵?CPU N 澶辫触锛屽垯璋冪敤 CPU 0 .. N-1 鐨勬媶闄ゅ洖璋冩潵鍥炴粴璇ユ搷浣滐紝璇ュ嚱鏁板け璐ワ紝骞朵笖
    璇ュ疄渚嬩笉琚坊鍔犲埌澶氬疄渚嬬姸鎬佺殑鑺傜偣鍒楄〃銆?
瑕佷粠鐘舵€佺殑鑺傜偣鍒楄〃绉婚櫎涓€涓疄渚嬶紝鍙娇鐢ㄨ繖浜涘嚱鏁帮細

  - cpuhp_state_remove_instance(state, node)
  - cpuhp_state_remove_instance_nocalls(state, node)

鍙傛暟涓庝笂闈㈢殑 cpuhp_state_add_instance*() 鍙樹綋鐩稿悓銆?
杩欎簺鍑芥暟鍦ㄦ墍瀹夎鐨勫洖璋冭瀵瑰緟鐨勬柟寮忎笂鏈夋墍涓嶅悓锛?
  - cpuhp_state_remove_instance_nocalls() 鍙粠
    鐘舵€佺殑鑺傜偣鍒楄〃绉婚櫎璇ュ疄渚嬨€?
  - cpuhp_state_remove_instance() 绉婚櫎瀹炰緥锛屽苟閽堝褰撳墠鐘舵€佸彿澶т簬 @state 鐨?    鎵€鏈夊湪绾?CPU 璋冪敤涓?@state 鍏宠仈鐨勬媶闄ゅ洖璋冿紙濡傛灉闈?NULL锛夈€傝鍥炶皟鍙拡瀵硅绉婚櫎鐨勫疄渚嬭璋冪敤銆?    鏍规嵁鐘舵€侀儴鍒嗭紝璇ュ洖璋冭涔堝湪褰撳墠 CPU锛圥REPARE 閮ㄥ垎锛変笂琚皟鐢紝瑕佷箞鍦ㄦ瘡涓湪绾?CPU锛圤NLINE 閮ㄥ垎锛?    涓婄殑 CPU 鐑彃鎷旂嚎绋嬩笂涓嬫枃涓璋冪敤銆?
    涓轰簡瀹屾垚绉婚櫎锛屾媶闄ゅ洖璋冧笉搴斿け璐ャ€?
鑺傜偣鍒楄〃鐨勬坊鍔?绉婚櫎鎿嶄綔涓庡洖璋冭皟鐢ㄧ浉瀵逛簬 CPU 鐑彃鎷旀搷浣滆涓茶鍖栥€?杩欎簺鍑芥暟涓嶈兘浠?CPU 鐑彃鎷斿洖璋冨唴閮ㄤ互鍙?CPU 鐑彃鎷旇閿佸畾鍖哄煙涓娇鐢ㄣ€?
### 绀轰緥


鍦?STARTING 閮ㄥ垎涓缓绔嬪苟鎷嗛櫎涓€涓潤鎬佸垎閰嶇殑鐘舵€?```
   ret = cpuhp_setup_state(CPUHP_SUBSYS_STARTING, "subsys:starting", subsys_cpu_starting, subsys_cpu_dying);
   if (ret < 0)
        return ret;
   ....
   cpuhp_remove_state(CPUHP_SUBSYS_STARTING);
```
鍦?ONLINE 閮ㄥ垎涓缓绔嬪苟鎷嗛櫎涓€涓姩鎬佸垎閰嶇殑鐘舵€?```
   state = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "subsys:offline", NULL, subsys_cpu_offline);
   if (state < 0)
       return state;
   ....
   cpuhp_remove_state(state);
```
鍦?ONLINE 閮ㄥ垎涓缓绔嬪苟鎷嗛櫎涓€涓姩鎬佸垎閰嶇殑鐘舵€?```
   state = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "subsys:online", subsys_cpu_online, NULL);
   if (state < 0)
       return state;
   ....
   cpuhp_remove_state_nocalls(state);
```
寤虹珛銆佷娇鐢ㄥ苟鎷嗛櫎涓€涓姩鎬佸垎閰嶇殑澶氬疄渚嬬姸鎬?```
   state = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN, "subsys:online", subsys_cpu_online, subsys_cpu_offline);
   if (state < 0)
       return state;
   ....
   ret = cpuhp_state_add_instance(state, &inst1->node);
   if (ret)
        return ret;
   ....
   ret = cpuhp_state_add_instance(state, &inst2->node);
   if (ret)
        return ret;
   ....
   cpuhp_remove_instance(state, &inst1->node);
   ....
   cpuhp_remove_instance(state, &inst2->node);
   ....
   cpuhp_remove_multi_state(state);
```

## 鐑彃鎷旂姸鎬佺殑娴嬭瘯


楠岃瘉涓€涓嚜瀹氫箟鐘舵€佹槸鍚﹀棰勬湡宸ヤ綔鐨勪竴绉嶆柟寮忔槸鍏抽棴涓€涓?CPU锛岀劧鍚庡啀灏嗗叾涓婄嚎銆?涔熷彲浠ュ皢璇?CPU 缃簬鏌愪釜鐗瑰畾鐘舵€侊紙渚嬪 **CPUHP_AP_ONLINE**锛夛紝鐒跺悗鍥炲埌
**CPUHP_ONLINE**銆傝繖浼氭ā鎷熷湪 **CPUHP_AP_ONLINE** 涔嬪悗鐨勪竴涓姸鎬佸嚭閿欙紝
浠庤€屽鑷村洖婊氬埌鍦ㄧ嚎鐘舵€併€?
```
 $ tail /sys/devices/system/cpu/hotplug/states
 138: mm/vmscan:online
 139: mm/vmstat:online
 140: lib/percpu_cnt:online
 141: acpi/cpu-drv:online
 142: base/cacheinfo:online
 143: virtio/net:online
 144: x86/mce:online
 145: printk:online
 168: sched:active
 169: online
```
```
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  169
  $ echo 140 > /sys/devices/system/cpu/cpu4/hotplug/target
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  140
```
闇€瑕佹敞鎰忕殑鏄紝鐘舵€?140 鐨勬媶闄ゅ洖璋冨凡琚?```
  $ echo 169 > /sys/devices/system/cpu/cpu4/hotplug/target
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  169
```
```
  #  TASK-PID   CPU#    TIMESTAMP  FUNCTION
  #     | |       |        |         |
      bash-394  [001]  22.976: cpuhp_enter: cpu: 0004 target: 140 step: 169 (cpuhp_kick_ap_work)
   cpuhp/4-31   [004]  22.977: cpuhp_enter: cpu: 0004 target: 140 step: 168 (sched_cpu_deactivate)
   cpuhp/4-31   [004]  22.990: cpuhp_exit:  cpu: 0004  state: 168 step: 168 ret: 0
   cpuhp/4-31   [004]  22.991: cpuhp_enter: cpu: 0004 target: 140 step: 144 (mce_cpu_pre_down)
   cpuhp/4-31   [004]  22.992: cpuhp_exit:  cpu: 0004  state: 144 step: 144 ret: 0
   cpuhp/4-31   [004]  22.993: cpuhp_multi_enter: cpu: 0004 target: 140 step: 143 (virtnet_cpu_down_prep)
   cpuhp/4-31   [004]  22.994: cpuhp_exit:  cpu: 0004  state: 143 step: 143 ret: 0
   cpuhp/4-31   [004]  22.995: cpuhp_enter: cpu: 0004 target: 140 step: 142 (cacheinfo_cpu_pre_down)
   cpuhp/4-31   [004]  22.996: cpuhp_exit:  cpu: 0004  state: 142 step: 142 ret: 0
      bash-394  [001]  22.997: cpuhp_exit:  cpu: 0004  state: 140 step: 169 ret: 0
      bash-394  [005]  95.540: cpuhp_enter: cpu: 0004 target: 169 step: 140 (cpuhp_kick_ap_work)
   cpuhp/4-31   [004]  95.541: cpuhp_enter: cpu: 0004 target: 169 step: 141 (acpi_soft_cpu_online)
   cpuhp/4-31   [004]  95.542: cpuhp_exit:  cpu: 0004  state: 141 step: 141 ret: 0
   cpuhp/4-31   [004]  95.543: cpuhp_enter: cpu: 0004 target: 169 step: 142 (cacheinfo_cpu_online)
   cpuhp/4-31   [004]  95.544: cpuhp_exit:  cpu: 0004  state: 142 step: 142 ret: 0
   cpuhp/4-31   [004]  95.545: cpuhp_multi_enter: cpu: 0004 target: 169 step: 143 (virtnet_cpu_online)
   cpuhp/4-31   [004]  95.546: cpuhp_exit:  cpu: 0004  state: 143 step: 143 ret: 0
   cpuhp/4-31   [004]  95.547: cpuhp_enter: cpu: 0004 target: 169 step: 144 (mce_cpu_online)
   cpuhp/4-31   [004]  95.548: cpuhp_exit:  cpu: 0004  state: 144 step: 144 ret: 0
   cpuhp/4-31   [004]  95.549: cpuhp_enter: cpu: 0004 target: 169 step: 145 (console_cpu_notify)
   cpuhp/4-31   [004]  95.550: cpuhp_exit:  cpu: 0004  state: 145 step: 145 ret: 0
   cpuhp/4-31   [004]  95.551: cpuhp_enter: cpu: 0004 target: 169 step: 168 (sched_cpu_activate)
   cpuhp/4-31   [004]  95.552: cpuhp_exit:  cpu: 0004  state: 168 step: 168 ret: 0
      bash-394  [005]  95.553: cpuhp_exit:  cpu: 0004  state: 169 step: 140 ret: 0
```
濡傛墍瑙侊紝CPU4 涓€璺笅闄嶅埌鏃堕棿鎴?22.996锛岀劧鍚庡張涓€璺笂鍗囧埌 95.552銆?鎵€鏈夎璋冪敤鐨勫洖璋冨強鍏惰繑鍥炵爜鍦?trace 涓兘鍙銆?
## 鏋舵瀯瑕佹眰


闇€瑕佷互涓嬪嚱鏁颁笌閰嶇疆锛?
`CONFIG_HOTPLUG_CPU`
  姝ゆ潯鐩渶瑕佸湪 Kconfig 涓惎鐢?
`__cpu_up()`
  Arch 鎺ュ彛锛岀敤浜庡惎鍔ㄤ竴涓?CPU

`__cpu_disable()`
  Arch 鎺ュ彛锛岀敤浜庡叧闂竴涓?CPU锛屽湪璇ヤ緥绋嬭繑鍥炰箣鍚庡唴鏍镐笉鑳藉啀澶勭悊浠讳綍涓柇銆傝繖鍖呮嫭瀹氭椂鍣ㄧ殑鍏抽棴銆?
`__cpu_die()`
  杩欏疄闄呬笂鏄敤鏉ョ‘淇?CPU 鐨勬浜°€傚疄闄呬笂璇峰弬鑰冨叾浠栧疄鐜颁簡 CPU 鐑彃鎷旂殑 arch 涓殑绀轰緥浠ｇ爜銆?  澶勭悊鍣ㄤ粠璇ョ壒瀹氭灦鏋勭殑 `idle()` 寰幆涓鍙栦笅銆俙__cpu_die()`
  閫氬父绛夊緟鏌愪釜 per_cpu 鐘舵€佽璁剧疆锛屼互纭繚澶勭悊鍣ㄦ浜′緥绋嬭璋冪敤锛屼粠鑰岀‘淇″叾宸叉浜°€?
## 鐢ㄦ埛绌洪棿閫氱煡


```
  SUBSYSTEM=="cpu", DRIVERS=="processor", DEVPATH=="/devices/system/cpu/*", RUN+="the_hotplug_receiver.sh"
```
```
  #!/bin/sh

  if [ "${ACTION}" = "offline" ]
  then
      echo "CPU ${DEVPATH##*/} offline"

  elif [ "${ACTION}" = "online" ]
  then
      echo "CPU ${DEVPATH##*/} online"

  fi
```
鍙互杩涗竴姝ュ鐞嗚浜嬩欢銆?
褰撶郴缁熶腑鍙戠敓瀵?CPU 鐨勬洿鏀规椂锛屽鏋滃唴鏍歌嚜琛屾洿鏂?kdump 鎹曡幏鍐呮牳鐨?CPU 鍒楄〃锛堥€氳繃 elfcorehdr 浠ュ強
鍏朵粬鐩稿叧鐨?kexec 娈碉級锛屽垯 sysfs 鏂囦欢
/sys/devices/system/cpu/crash_hotplug 鍖呭惈 '1'锛屽鏋滅敤鎴风┖闂村繀椤绘洿鏂?kdump 鎹曡幏鍐呮牳鐨?CPU 鍒楄〃锛屽垯鍖呭惈 '0'銆?
鍏跺彲鐢ㄦ€у彇鍐充簬 CONFIG_HOTPLUG_CPU 鍐呮牳閰嶇疆閫夐」銆?
涓轰簡璺宠繃鐢ㄦ埛绌洪棿瀵圭敤浜?kdump 鐨?CPU 鐑彃鎷?鎷斾笅浜嬩欢鐨勫鐞嗭紙鍗冲厛鍗歌浇鍐嶉噸杞戒互鑾峰緱褰撳墠 CPU 鍒楄〃锛夛紝
姝?sysfs 鏂囦欢鍙湪 udev 瑙勫垯涓涓嬩娇鐢細

 SUBSYSTEM=="cpu", ATTRS{crash_hotplug}=="1", GOTO="kdump_reload_end"

瀵逛簬 CPU 鐑彃鎷?鎷斾笅浜嬩欢锛屽鏋滄灦鏋勬敮鎸佸 elfcorehdr锛堝寘鍚?CPU 鍒楄〃锛変互鍙婂叾浠栫浉鍏?kexec 娈电殑
鍐呮牳鏇存柊锛岄偅涔堣瑙勫垯浼氳烦杩?kdump 鎹曡幏鍐呮牳鐨勫嵏杞?閲嶈浇銆?
## 鍐呮牳鍐呰仈鏂囨。鍙傝€?