
## Intel 鐑妭娴侊紙thermal throttle锛変簨浠舵姤鍛?

:Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 绠€浠嬶紙Introduction锛?

Intel 澶勭悊鍣ㄥ唴缃嚜鍔ㄤ笖鑷€傚簲鐨勭儹鐩戞帶鏈哄埗锛屽己鍒跺鐞嗗櫒闄嶄綆鍏跺姛鑰楋紝浠ヤ究鍦ㄩ瀹氱殑娓╁害闄愬埗鍐呰繍琛屻€?
鏇村缁嗚妭璇峰弬闃?Intel庐 64 and IA-32 Architectures Software Developer's Manual Volume 3 (3A, 3B, 3C, & 3D): System Programming Guide"涓殑"THERMAL MONITORING AND PROTECTION"涓€鑺傘€?
涓€鑸€岃█锛屾湁涓ょ鏈哄埗鐢ㄤ簬鎺у埗澶勭悊鍣ㄧ殑鏍稿績娓╁害銆傚畠浠О涓?Thermal Monitor 1锛圱M1锛岀儹鐩戞帶鍣?1锛変笌 Thermal Monitor 2锛圱M2锛岀儹鐩戞帶鍣?2锛?銆?
瑙﹀彂鐑洃鎺э紙TM1/TM2锛夌殑娓╁害浼犳劅鍣ㄧ殑鐘舵€侊紝閫氳繃鏍稿績绾х殑 MSR_IA32_THERM_STATUS 涓庡皝瑁咃紙package锛夌骇鐨?MSR_IA32_PACKAGE_THERM_STATUS 涓殑"鐑姸鎬佹爣蹇?锛坱hermal status flag锛変笌"鐑姸鎬佹棩蹇楁爣蹇?锛坱hermal status log flag锛夋潵鎸囩ず銆?
鐑姸鎬佹爣蹇楋紙Thermal Status flag锛夛紝绗?0 浣?鈥?缃綅鏃讹紝琛ㄧず澶勭悊鍣ㄦ牳蹇冩俯搴﹀綋鍓嶅浜庣儹鐩戞帶鍣ㄧ殑瑙﹀彂锛坱rip锛夋俯搴︼紝涓斿鐞嗗櫒鍔熻€楁閫氳繃 TM1 鎴?TM2锛堝彇鍐充簬鍝釜琚惎鐢級琚檷浣庛€傛竻闆舵椂锛岃鏍囧織琛ㄧず鏍稿績娓╁害浣庝簬鐑洃鎺у櫒瑙﹀彂娓╁害銆傝鏍囧織涓哄彧璇汇€?
鐑姸鎬佹棩蹇楁爣蹇楋紙Thermal Status Log flag锛夛紝绗?1 浣?鈥?缃綅鏃讹紝琛ㄧず鑷笂娆′笂鐢垫垨澶嶄綅浠ユ潵锛屾垨鑰呰嚜杞欢涓婃娓呴櫎璇ユ爣蹇椾互鏉ワ紝鐑紶鎰熷櫒宸茶Е鍙戣繃銆傝鏍囧織鏄?榛忔粸浣?锛坰ticky bit锛夛紱涓€鏃︾疆浣嶏紝瀹冧細淇濇寔缃綅锛岀洿鍒拌杞欢娓呴櫎鎴栫洿鍒板鐞嗗櫒涓婄數鎴栧浣嶃€傞粯璁ょ姸鎬佷负娓呴浂銆?
鏈夊彲鑳藉綋鐢ㄦ埛璇诲彇 MSR_IA32_THERM_STATUS 鎴?MSR_IA32_PACKAGE_THERM_STATUS 鏃讹紝TM1/TM2 骞舵湭澶勪簬娲诲姩鐘舵€併€傛鏃讹紝"鐑姸鎬佹爣蹇?灏嗚涓?0"锛岃€?鐑姸鎬佹棩蹇楁爣蹇?浼氳缃綅浠ユ樉绀轰换浣曞厛鍓嶇殑"TM1/TM2"婵€娲汇€備絾鐢变簬瀹冮渶瑕佽杞欢娓呴櫎锛屽洜姝ゆ棤娉曟樉绀?TM1/TM2"婵€娲荤殑鍙戠敓娆℃暟銆?
鍥犳锛孡inux 鎻愪緵浜?鐑姸鎬佹爣蹇?琚疆浣嶇殑娆℃暟璁℃暟锛屽悓鏃跺憟鐜?鐑姸鎬佹爣蹇?澶勪簬娲诲姩鐘舵€佺殑姣鏃堕暱銆傚埄鐢ㄨ繖浜涜鏁板櫒锛岀敤鎴峰彲浠ユ鏌ユ€ц兘鏄惁鍥犵儹浜嬩欢鑰屽彈鍒伴檺鍒躲€傚缓璁粠 sysfs 璇诲彇锛岃€岄潪鐩存帴璇诲彇 MSR锛屽洜涓?鐑姸鎬佹棩蹇楁爣蹇?浼氳椹卞姩閲嶇疆浠ュ疄鐜伴€熺巼鎺у埗锛坮ate control锛夈€?
### Sysfs 鎺ュ彛锛圫ysfs Interface锛?

鐑妭娴佷簨浠跺湪姣忎釜 CPU 涓嬮€氳繃 "/sys/devices/system/cpu/cpuX/thermal_throttle/" 鍛堢幇锛屽叾涓?"X" 涓?CPU 缂栧彿銆?
鎵€鏈夎繖浜涜鏁板櫒閮芥槸鍙鐨勩€傚畠浠笉鑳借閲嶇疆涓?0銆傚洜姝わ紝瀹冧滑鍦ㄨ揪鍒?64 浣嶆棤绗﹀彿鏁存暟鐨勬渶澶у€煎悗鍙兘浼氭孩鍑恒€?
`core_throttle_count`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忥紙thermal vector锛夊垵濮嬪寲浠ユ潵锛岃 CPU 鐨?鐑姸鎬佹爣蹇?浠?0 鍙樹负 1 鐨勬鏁般€傝繖鏄竴涓?64 浣嶈鏁板櫒銆?
`package_throttle_count`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忓垵濮嬪寲浠ユ潵锛屽寘鍚 CPU 鐨勫皝瑁咃紙package锛夌殑"鐑姸鎬佹爣蹇?浠?0 鍙樹负 1 鐨勬鏁般€傚皝瑁呯姸鎬佷細琚箍鎾埌鎵€鏈?CPU锛涘皝瑁呭唴鎵€鏈?CPU 閮介€掑璇ヨ鏁般€傝繖鏄竴涓?64 浣嶈鏁板櫒銆?
`core_throttle_max_time_ms`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忓垵濮嬪寲浠ユ潵锛岃 CPU 鍦ㄦ牳蹇冪骇"鐑姸鎬佹爣蹇?琚疆涓?1 鐨勬渶澶ф€绘椂闀裤€?
`package_throttle_max_time_ms`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忓垵濮嬪寲浠ユ潵锛屽寘鍚 CPU 鐨勫皝瑁呯殑"鐑姸鎬佹爣蹇?琚疆涓?1 鐨勬渶澶ф€绘椂闀裤€?
`core_throttle_total_time_ms`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忓垵濮嬪寲浠ユ潵锛岃 CPU 鍦ㄦ牳蹇冪骇"鐑姸鎬佹爣蹇?琚疆涓?1 鐨勭疮璁℃椂闀裤€?
`package_throttle_total_time_ms`
	鏄剧ず鑷搷浣滅郴缁熷惎鍔ㄤ笖鐑悜閲忓垵濮嬪寲浠ユ潵锛屽寘鍚 CPU 鐨勫皝瑁呯殑"鐑姸鎬佹爣蹇?琚疆涓?1 鐨勭疮璁℃椂闀裤€?