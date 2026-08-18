## 鍔熻€楀皝椤讹紙Power Capping锛夋鏋?

鍔熻€楀皝椤舵鏋跺湪鍐呮牳涓庣敤鎴风┖闂翠箣闂存彁渚涗簡涓€鑷寸殑鎺ュ彛锛屼娇寰楀姛鑰楀皝椤堕┍鍔ㄨ兘澶熶互缁熶竴鐨勬柟寮忓皢鐩稿叧璁剧疆鏆撮湶缁欑敤鎴风┖闂淬€?
## 鏈


璇ユ鏋堕€氳繃 sysfs 浠ュ璞℃爲鐨勫舰寮忓皢鍔熻€楀皝椤惰澶囨毚闇茬粰鐢ㄦ埛绌洪棿銆傛爲鏍瑰眰绾х殑瀵硅薄浠ｈ〃鈥滄帶鍒剁被鍨嬶紙control types锛夆€濓紝瀵瑰簲浜庝笉鍚岀殑鍔熻€楀皝椤舵柟娉曘€備緥濡傦紝intel-rapl 鎺у埗绫诲瀷浠ｈ〃 Intel 鐨勨€滆繍琛屽钩鍧囧姛鐜囬檺鍒垛€濓紙Running Average Power Limit锛孯APL锛夋妧鏈紝鑰?idle-injection 鎺у埗绫诲瀷鍒欏搴斾簬浣跨敤绌洪棽娉ㄥ叆锛坕dle injection锛夋潵鎺у埗鍔熻€椼€?
鍔熻€楀尯锛坧ower zone锛変唬琛ㄧ郴缁熶腑鍙互杩涜鎺у埗鍜岀洃鎺х殑涓嶅悓閮ㄥ垎锛屼娇鐢ㄥ叾鎵€褰掑睘鐨勬帶鍒剁被鍨嬫墍纭畾鐨勫姛鑰楀皝椤舵柟娉曘€傚畠浠悇鑷寘鍚敤浜庣洃鎺у姛鑰楃殑灞炴€э紝浠ュ強浠ュ姛鑰楃害鏉燂紙power constraint锛夊舰寮忚〃绀虹殑鎺т欢銆傚鏋滅敱涓嶅悓鍔熻€楀尯鎵€浠ｈ〃鐨勭郴缁熷悇閮ㄥ垎涔嬮棿瀛樺湪灞傛鍏崇郴锛堝嵆涓€涓緝澶х殑閮ㄥ垎鐢卞涓緝灏忕殑銆佸悇鑷嫢鏈夌嫭绔嬪姛鑰楁帶鍒剁殑閮ㄥ垎缁勬垚锛夛紝閭ｄ箞杩欎簺鍔熻€楀尯涔熷彲浠ョ粍缁囨垚灞傛缁撴瀯锛氫竴涓埗鍔熻€楀尯鍖呭惈澶氫釜瀛愬尯锛屼互姝ょ被鎺紝浠ュ弽鏄犵郴缁熺殑鍔熻€楁帶鍒舵嫇鎵戙€傚湪杩欑鎯呭喌涓嬶紝鍙互閫氳繃鐖跺姛鑰楀尯灏嗗姛鑰楀皝椤跺悓鏃跺簲鐢ㄤ簬涓€缁勮澶囷紱濡傛灉闇€瑕佹洿缁嗙矑搴︾殑鎺у埗锛屽垯鍙互閫氳繃瀛愬尯鏉ユ柦鍔犮€?
```
  /sys/devices/virtual/powercap
  鈹斺攢鈹€intel-rapl
      鈹溾攢鈹€intel-rapl:0
      鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl
      鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹溾攢鈹€intel-rapl:0:0
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl:0
      鈹偮犅?鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../../class/power_cap
      鈹偮犅?鈹偮犅?鈹斺攢鈹€uevent
      鈹偮犅?鈹溾攢鈹€intel-rapl:0:1
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl:0
      鈹偮犅?鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../../class/power_cap
      鈹偮犅?鈹偮犅?鈹斺攢鈹€uevent
      鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹溾攢鈹€max_power_range_uw
      鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../class/power_cap
      鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹溾攢鈹€uevent
      鈹溾攢鈹€intel-rapl:1
      鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl
      鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹溾攢鈹€intel-rapl:1:0
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl:1
      鈹偮犅?鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../../class/power_cap
      鈹偮犅?鈹偮犅?鈹斺攢鈹€uevent
      鈹偮犅?鈹溾攢鈹€intel-rapl:1:1
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_0_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_power_limit_uw
      鈹偮犅?鈹偮犅?鈹溾攢鈹€constraint_1_time_window_us
      鈹偮犅?鈹偮犅?鈹溾攢鈹€device -> ../../intel-rapl:1
      鈹偮犅?鈹偮犅?鈹溾攢鈹€energy_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../../class/power_cap
      鈹偮犅?鈹偮犅?鈹斺攢鈹€uevent
      鈹偮犅?鈹溾攢鈹€max_energy_range_uj
      鈹偮犅?鈹溾攢鈹€max_power_range_uw
      鈹偮犅?鈹溾攢鈹€name
      鈹偮犅?鈹溾攢鈹€enabled
      鈹偮犅?鈹溾攢鈹€power
      鈹偮犅?鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?鈹偮犅?[]
      鈹偮犅?鈹溾攢鈹€subsystem -> ../../../../../class/power_cap
      鈹偮犅?鈹溾攢鈹€uevent
      鈹溾攢鈹€power
      鈹偮犅?鈹溾攢鈹€async
      鈹偮犅?[]
      鈹溾攢鈹€subsystem -> ../../../../class/power_cap
      鈹溾攢鈹€enabled
      鈹斺攢鈹€uevent
```

涓婅堪绀轰緥灞曠ず浜嗕娇鐢?Intel庐 IA-64 涓?IA-32 澶勭悊鍣ㄦ灦鏋勪腑鍙敤鐨?Intel RAPL 鎶€鏈殑鎯呭喌銆傚叾涓湁涓€涓悕涓?intel-rapl 鐨勬帶鍒剁被鍨嬶紝瀹冨寘鍚袱涓姛鑰楀尯 intel-rapl:0 涓?intel-rapl:1锛屼唬琛?CPU 灏佽锛坧ackage锛夈€傛瘡涓姛鑰楀尯鍙堝寘鍚袱涓瓙鍖?intel-rapl:j:0 涓?intel-rapl:j:1锛坖 = 0, 1锛夛紝鍒嗗埆浠ｈ〃璇?CPU 灏佽鐨勨€滄牳蹇冿紙core锛夆€濅笌鈥滈潪鏍稿績锛坲ncore锛夆€濋儴鍒嗐€傛墍鏈夌殑鍖轰笌瀛愬尯閮藉寘鍚兘鑰楃洃鎺у睘鎬э紙energy_uj銆乵ax_energy_range_uj锛変互鍙婄害鏉熷睘鎬э紙constraint_*锛夛紝鐢ㄤ互鏂藉姞鎺у埗锛堚€滃皝瑁咃紙package锛夆€濆姛鑰楀尯涓殑绾︽潫浣滅敤浜庢暣涓?CPU 灏佽锛岃€屽瓙鍖虹害鏉熷彧鍒嗗埆浣滅敤浜庤灏佽鍚勮嚜鐨勯儴鍒嗭級銆傜敱浜?Intel RAPL 涓嶆彁渚涚灛鏃跺姛鐜囧€硷紝鍥犳娌℃湁 power_uw 灞炴€с€?
姝ゅ锛屾瘡涓姛鑰楀尯杩樺寘鍚竴涓?name 灞炴€э紝鐢ㄤ簬鏍囪瘑璇ュ尯鎵€浠ｈ〃鐨勭郴缁熼儴鍒嗐€?
```
	cat /sys/class/power_cap/intel-rapl/intel-rapl:0/name
```

### package-0


鏍规嵁鍔熻€楀尯鐨勪笉鍚岋紝Intel RAPL 鎶€鏈厑璁稿鍚勪釜鍔熻€楀尯鏂藉姞涓€涓垨澶氫釜绾︽潫锛屼緥濡傜煭鏈熴€侀暱鏈熶互鍙婂嘲鍊煎姛鐜囷紝骞跺甫鏈変笉鍚岀殑鏃堕棿绐楀彛銆?鎵€鏈夌殑鍖洪兘鍖呭惈浠ｈ〃绾︽潫鍚嶇О銆佸姛鐜囬檺鍒朵互鍙婃椂闂寸獥鍙ｅぇ灏忕殑灞炴€с€傛敞鎰忥紝鏃堕棿绐楀彛涓嶉€傜敤浜庡嘲鍊煎姛鐜囥€傝繖閲岀殑 constraint_j_* 灞炴€у搴斾簬绗?j 涓害鏉燂紙j = 0,1,2锛夈€?
```
	constraint_0_name
	constraint_0_power_limit_uw
	constraint_0_time_window_us
	constraint_1_name
	constraint_1_power_limit_uw
	constraint_1_time_window_us
	constraint_2_name
	constraint_2_power_limit_uw
	constraint_2_time_window_us
```

## 鍔熻€楀尯灞炴€?

### 鐩戞帶灞炴€?

energy_uj (rw)
	褰撳墠鑳借€楄鏁板櫒锛屽崟浣嶄负寰劍锛坢icro joules锛夈€傚啓鍏?鈥?鈥?浠ラ噸缃€?	濡傛灉璁℃暟鍣ㄦ棤娉曢噸缃紝鍒欒灞炴€т负鍙銆?
max_energy_range_uj (ro)
	涓婅堪鑳借€楄鏁板櫒鐨勮寖鍥达紝鍗曚綅涓哄井鐒︺€?
power_uw (ro)
	褰撳墠鍔熺巼锛屽崟浣嶄负寰摝銆?
max_power_range_uw (ro)
	涓婅堪鍔熺巼鍊肩殑鑼冨洿锛屽崟浣嶄负寰摝銆?
name (ro)
	鏈姛鑰楀尯鐨勫悕绉般€?
鏌愪簺鍩熷彲鑳藉悓鏃跺叿鏈夊姛鐜囪寖鍥翠笌鑳借€楄鏁板櫒鑼冨洿锛涗笉杩囷紝浜岃€呬腑鍙湁涓€涓槸蹇呴』鐨勩€?
### 绾︽潫


constraint_X_power_limit_uw (rw)
	鍔熺巼闄愬埗锛屽崟浣嶄负寰摝锛屽簲閫傜敤浜庣敱 鈥渃onstraint_X_time_window_us鈥?鎸囧畾鐨?	鏃堕棿绐楀彛銆?
constraint_X_time_window_us (rw)
	鏃堕棿绐楀彛锛屽崟浣嶄负寰銆?
constraint_X_name (ro)
	绾︽潫鐨勫彲閫夊悕绉般€?
constraint_X_max_power_uw (ro)
	鍏佽鐨勬渶澶у姛鐜囷紝鍗曚綅涓哄井鐡︺€?
constraint_X_min_power_uw (ro)
	鍏佽鐨勬渶灏忓姛鐜囷紝鍗曚綅涓哄井鐡︺€?
constraint_X_max_time_window_us (ro)
	鍏佽鐨勬渶澶ф椂闂寸獥鍙ｏ紝鍗曚綅涓哄井绉掋€?
constraint_X_min_time_window_us (ro)
	鍏佽鐨勬渶灏忔椂闂寸獥鍙ｏ紝鍗曚綅涓哄井绉掋€?
闄?power_limit_uw 涓?time_window_us 澶栵紝鍏朵綑瀛楁鍧囦负鍙€夈€?
### 閫氱敤鍖轰笌鎺у埗绫诲瀷灞炴€?

enabled (rw)锛氬湪鍖虹骇鍒垨浣跨敤鏌愪釜鎺у埗绫诲瀷瀵规墍鏈夊尯鍚敤/绂佺敤鎺у埗銆?
## 鍔熻€楀皝椤跺鎴风椹卞姩鎺ュ彛


API 姒傝锛?
璋冪敤 powercap_register_control_type() 娉ㄥ唽鎺у埗绫诲瀷瀵硅薄銆?璋冪敤 powercap_register_zone() 娉ㄥ唽涓€涓姛鑰楀尯锛堝湪鏌愪釜缁欏畾鐨勬帶鍒剁被鍨嬩笅锛夛紝
鏃㈠彲浠ヤ綔涓洪《灞傚姛鑰楀尯锛屼篃鍙互浣滀负鍏堝墠娉ㄥ唽鐨勫彟涓€涓姛鑰楀尯鐨勫瓙鍖恒€?鍦ㄨ皟鐢?powercap_register_zone() 娉ㄥ唽鏌愪釜鍔熻€楀尯涔嬪墠锛屽繀椤诲厛瀹氫箟璇ュ尯涓?绾︽潫鐨勬暟閲忎互鍙婄浉搴旂殑鍥炶皟鍑芥暟銆?
瑕侀噴鏀句竴涓姛鑰楀尯锛岃皟鐢?powercap_unregister_zone()銆?瑕侀噴鏀句竴涓帶鍒剁被鍨嬪璞★紝璋冪敤 powercap_unregister_control_type()銆?璇︾粏鐨?API 鍙互閫氳繃瀵?include/linux/powercap.h 浣跨敤 kernel-doc 鐢熸垚銆?