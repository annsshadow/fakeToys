## 杩愯鎬ц兘鐐癸紙OPP锛夊簱


(C) 2009-2010 Nishanth Menon <nm@ti.com>, Texas Instruments Incorporated


  1. 绠€浠?
  2. OPP 鍒楄〃鐨勫垵濮嬫敞鍐?
  3. OPP 鎼滅储鍑芥暟
  4. OPP 鍙敤鎬ф帶鍒跺嚱鏁?
  5. OPP 鏁版嵁妫€绱㈠嚱鏁?
  6. 鏁版嵁缁撴瀯

## 1. 绠€浠?


### 1.1 浠€涔堟槸杩愯鎬ц兘鐐癸紙OPP锛夛紵


濡備粖鐨勫鏉?SoC 鐢卞涓崗鍚屽伐浣滅殑瀛愭ā鍧楃粍鎴愩€傚湪涓€涓繍琛屽绉嶇敤渚嬬殑鎿嶄綔绯荤粺涓紝骞堕潪 SoC 涓殑鎵€鏈夋ā鍧楅兘闇€瑕佸缁堜互鏈€楂樻€ц兘棰戠巼宸ヤ綔銆備负渚夸簬瀹炵幇杩欎竴鐐癸紝SoC 涓殑瀛愭ā鍧楄鍒掑垎涓哄涓煙锛屽厑璁告煇浜涘煙浠ヨ緝浣庣數鍘嬪拰棰戠巼杩愯锛岃€屽叾浠栧煙浠ヨ緝楂樼殑鐢靛帇/棰戠巼瀵硅繍琛屻€?

璁惧鍦ㄦ瘡涓煙鎵€鏀寔鐨勩€佺敱棰戠巼鍜岀數鍘嬪缁勬垚鐨勭鏁ｅ厓缁勯泦鍚堬紝绉颁负杩愯鎬ц兘鐐癸紙Operating Performance Points锛岀畝绉?OPP锛夈€?

渚嬪锛?

Let us consider an MPU device which supports the following:
{300MHz at minimum voltage of 1V}, {800MHz at minimum voltage of 1.2V},
{1GHz at minimum voltage of 1.3V}

鎴戜滑鍙互灏嗚繖浜涜〃绀轰负濡備笅涓変釜 OPP锛屽嵆 {Hz, uV} 鍏冪粍锛?

- {300000000, 1000000}
- {800000000, 1200000}
- {1000000000, 1300000}

### 1.2 杩愯鎬ц兘鐐瑰簱


OPP 搴撴彁渚涗簡涓€缁勮緟鍔╁嚱鏁帮紝鐢ㄤ簬缁勭粐涓庢煡璇?OPP 淇℃伅銆傝搴撲綅浜?drivers/opp/ 鐩綍锛屽ご鏂囦欢浣嶄簬 include/linux/pm_opp.h銆傚彲浠ラ€氳繃鐢垫簮绠＄悊 menuconfig 鑿滃崟涓殑 CONFIG_PM_OPP 鏉ュ惎鐢?OPP 搴撱€傛煇浜?SoC锛堝寰峰窞浠櫒锛圱exas Instruments锛夌殑 OMAP 妗嗘灦锛夊厑璁稿湪涓嶉渶瑕?cpufreq 鐨勬儏鍐典笅浠ユ煇涓壒瀹?OPP 鍚姩銆?

```

 (users)	-> registers a set of default OPPs		-> (library)
 SoC framework	-> modifies on required cases certain OPPs	-> OPP layer
		-> queries to search/retrieve information	->

```
OPP 灞傛湡鏈涙瘡涓煙鐢变竴涓敮涓€鐨勮澶囨寚閽堣〃绀恒€係oC 妗嗘灦鍚?OPP 灞傛敞鍐屾瘡涓澶囩殑涓€缁勫垵濮?OPP銆傝鍒楄〃棰勬湡涓烘渶浼樼殑灏忔暟鐩紝閫氬父姣忎釜璁惧绾?5 涓€傝繖涓垵濮嬪垪琛ㄥ寘鍚鏋舵湡鏈涘湪绯荤粺涓粯璁ゅ畨鍏ㄥ惎鐢ㄧ殑涓€缁?OPP銆?

##### 鍏充簬 OPP 鍙敤鎬х殑璇存槑


闅忕潃绯荤粺寮€濮嬭繍琛岋紝SoC 妗嗘灦鍙互鍩轰簬鍚勭澶栭儴鍥犵礌閫夋嫨浣挎煇浜?OPP 鍦ㄦ瘡涓澶囦笂鍙敤鎴栦笉鍙敤銆傜ず渚嬬敤娉曪細鐑鐞嗘垨鍏朵粬寮傚父鎯呭喌锛屾鏃?SoC 妗嗘灦鍙兘閫夋嫨绂佺敤涓€涓緝楂橀鐜囩殑 OPP锛屼互瀹夊叏鍦扮户缁繍琛岋紝鐩村埌璇?OPP 鍦ㄥ彲鑳芥椂閲嶆柊鍚敤銆?

OPP 搴撳湪鍏跺疄鐜颁腑鏀寔杩欎竴姒傚康銆備互涓嬫搷浣滃嚱鏁颁粎瀵瑰彲鐢ㄧ殑 OPP 璧蜂綔鐢細dev_pm_opp_find_freq_{ceil, floor}銆乨ev_pm_opp_get_voltage銆乨ev_pm_opp_get_freq銆乨ev_pm_opp_get_opp_count銆?

dev_pm_opp_find_freq_exact 鐢ㄤ簬鏌ユ壘 opp 鎸囬拡锛岃鎸囬拡闅忓悗鍙敤浜?dev_pm_opp_enable/disable 鍑芥暟锛屼互鎸夐渶浣挎煇涓?opp 鍙敤銆?

璀﹀憡锛歄PP 搴撶殑鐢ㄦ埛濡傛灉涓烘煇涓澶囪皟鐢ㄤ簡 dev_pm_opp_enable/disable 鍑芥暟锛屽簲褰撲娇鐢?get_opp_count 鍒锋柊鍏跺彲鐢ㄨ鏁帮紱瑙﹀彂杩欎簺鎿嶄綔鐨勭簿纭満鍒讹紝鎴栧悜 cpufreq 绛夊叾浠栦緷璧栧瓙绯荤粺鍙戝嚭閫氱煡鐨勬満鍒讹紝鐢变娇鐢?OPP 搴撶殑 SoC 涓撶敤妗嗘灦鑷鍐冲畾銆傚湪鎵ц杩欎簺鎿嶄綔鏃讹紝鍚屾牱闇€瑕佹敞鎰忓埛鏂?cpufreq 琛ㄣ€?

## 2. OPP 鍒楄〃鐨勫垵濮嬫敞鍐?

SoC 瀹炵幇浼氳凯浠ｈ皟鐢?dev_pm_opp_add 鍑芥暟锛屼负姣忎釜璁惧娣诲姞 OPP銆傞鏈?SoC 妗嗘灦浼氭渶浼樺湴娉ㄥ唽 OPP 鏉＄洰鈥斺€斿吀鍨嬫暟閲忓皯浜?5 涓€傛敞鍐?OPP 鎵€鐢熸垚鐨勫垪琛ㄧ敱 OPP 搴撳湪璁惧杩愯鐨勬暣涓繃绋嬩腑缁存姢銆係oC 妗嗘灦闅忓悗鍙互浣跨敤 dev_pm_opp_enable/disable 鍑芥暟鍔ㄦ€佹帶鍒?OPP 鐨勫彲鐢ㄦ€с€?

dev_pm_opp_add
	涓鸿澶囨寚閽堟墍琛ㄧず鐨勭壒瀹氬煙娣诲姞涓€涓柊鐨?OPP銆?
	璇?OPP 鐢遍鐜囧拰鐢靛帇瀹氫箟銆備竴鏃︽坊鍔狅紝璇?OPP 鍗宠瑙嗕负鍙敤锛屽苟鍙娇鐢?dev_pm_opp_enable/disable 鍑芥暟鎺у埗鍏跺彲鐢ㄦ€с€侽PP 搴撳湪 dev_pm_opp struct 鍐呴儴瀛樺偍骞剁鐞嗘淇℃伅銆?
	SoC 妗嗘灦鍙互浣跨敤姝ゅ嚱鏁帮紝鏍规嵁 SoC 浣跨敤鐜鐨勯渶姹傚畾涔変竴涓渶浼樺垪琛ㄣ€?

	璀﹀憡锛?
		涓嶈鍦ㄤ腑鏂笂涓嬫枃涓娇鐢ㄦ鍑芥暟銆?

```

	 soc_pm_init()
	 {
		/* Do things */
		r = dev_pm_opp_add(mpu_dev, 1000000, 900000);
		if (!r) {
			pr_err("%s: unable to register mpu opp(%d)\n", r);
			goto no_cpufreq;
		}
		/* Do cpufreq things */
	 no_cpufreq:
		/* Do remaining things */
	 }

```
## 3. OPP 鎼滅储鍑芥暟

鍍?cpufreq 杩欐牱鐨勯珮灞傛鏋朵互棰戠巼涓哄崟浣嶅伐浣溿€備负浜嗗皢棰戠巼鏄犲皠鍥炲搴旂殑 OPP锛孫PP 搴撴彁渚涗簡渚挎嵎鍑芥暟鏉ユ悳绱?OPP 搴撳唴閮ㄧ鐞嗙殑 OPP 鍒楄〃銆傝繖浜涙悳绱㈠嚱鏁板湪鎵惧埌鍖归厤鏃惰繑鍥炰唬琛ㄨ opp 鐨勫尮閰嶆寚閽堬紝鍚﹀垯杩斿洖閿欒銆傝繖浜涢敊璇簲閫氳繃 IS_ERR() 绛夋爣鍑嗛敊璇鏌ユ潵澶勭悊锛屽苟鐢辫皟鐢ㄨ€呴噰鍙栭€傚綋鎺柦銆?

杩欎簺鍑芥暟鐨勮皟鐢ㄨ€呭湪浣跨敤瀹?OPP 鍚庯紝搴斿綋璋冪敤 dev_pm_opp_put()銆傚惁鍒?OPP 鐨勫唴瀛樺皢姘歌繙涓嶄細琚噴鏀撅紝浠庤€屽鑷村唴瀛樻硠婕忥紙memleak锛夈€?

dev_pm_opp_find_freq_exact
	鍩轰簬**绮剧‘**棰戠巼鍜屽彲鐢ㄦ€ф悳绱?OPP銆傛鍑芥暟鍦ㄥ惎鐢ㄤ竴涓粯璁や笉鍙敤鐨?OPP 鏃跺挨鍏舵湁鐢ㄣ€?
	绀轰緥锛氬綋 SoC 妗嗘灦妫€娴嬪埌鍙互浣挎煇涓洿楂橀鐜囧彲鐢ㄧ殑鎯呭喌鏃讹紝瀹冨彲浠ュ厛浣跨敤姝ゅ嚱鏁版壘鍒拌 OPP锛岀劧鍚庡啀璋冪敤 dev_pm_opp_enable 瀹為檯浣垮叾
```

	 opp = dev_pm_opp_find_freq_exact(dev, 1000000000, false);
	 dev_pm_opp_put(opp);
	 /* dont operate on the pointer.. just do a sanity check.. */
	 if (IS_ERR(opp)) {
		pr_err("frequency not disabled!\n");
		/* trigger appropriate actions.. */
	 } else {
		dev_pm_opp_enable(dev,1000000000);
	 }

	NOTE:
	  This is the only search function that operates on OPPs which are
	  not available.

```
dev_pm_opp_find_freq_floor
	鎼滅储涓€涓彲鐢ㄤ笖棰戠巼**鑷冲**绛変簬鎵€鎻愪緵棰戠巼鐨?OPP銆傛鍑芥暟鍦ㄦ悳绱㈣緝灏忓尮閰嶏紝鎴栨寜棰戠巼閫掑噺椤哄簭澶勭悊 OPP 淇℃伅鏃跺緢鏈夌敤銆?
```

	 freq = ULONG_MAX;
	 opp = dev_pm_opp_find_freq_floor(dev, &freq);
	 dev_pm_opp_put(opp);

```
dev_pm_opp_find_freq_ceil
	鎼滅储涓€涓彲鐢ㄤ笖棰戠巼**鑷冲皯**绛変簬鎵€鎻愪緵棰戠巼鐨?OPP銆傛鍑芥暟鍦ㄦ悳绱㈣緝澶у尮閰嶏紝鎴栨寜棰戠巼閫掑椤哄簭澶勭悊 OPP 淇℃伅鏃跺緢鏈夌敤銆?
```

	 freq = 0;
	 opp = dev_pm_opp_find_freq_ceil(dev, &freq);
	 dev_pm_opp_put(opp);

	Example 2: A simplified implementation of a SoC cpufreq_driver->target::

	 soc_cpufreq_target(..)
	 {
		/* Do stuff like policy checks etc. */
		/* Find the best frequency match for the req */
		opp = dev_pm_opp_find_freq_ceil(dev, &freq);
		dev_pm_opp_put(opp);
		if (!IS_ERR(opp))
			soc_switch_to_freq_voltage(freq);
		else
			/* do something when we can't satisfy the req */
		/* do other stuff */
	 }

```
## 4. OPP 鍙敤鎬ф帶鍒跺嚱鏁?

鍚?OPP 搴撴敞鍐岀殑榛樿 OPP 鍒楄〃鍙兘鏃犳硶婊¤冻鎵€鏈夊彲鑳界殑鎯呭舰銆侽PP 搴撴彁渚涗簡涓€缁勫嚱鏁版潵淇敼 OPP 鍒楄〃涓煇涓?OPP 鐨勫彲鐢ㄦ€с€傝繖浣?SoC 妗嗘灦鑳藉绮剧粏鍦板姩鎬佹帶鍒跺摢浜?OPP 闆嗗悎鍦ㄨ繍琛屼笂鍙敤銆傝繖浜涘嚱鏁扮敤浜庡湪鏌愪簺鏉′欢锛堜緥濡傜儹鑰冭檻锛堝锛氬湪娓╁害涓嬮檷鍓嶄笉瑕佷娇鐢?OPPx锛夛級涓?*涓存椂**绉婚櫎涓€涓?OPP銆?

璀﹀憡锛?
	涓嶈鍦ㄤ腑鏂笂涓嬫枃涓娇鐢ㄨ繖浜涘嚱鏁般€?

dev_pm_opp_enable
	浣夸竴涓?OPP 鍙敤浜庤繍琛屻€?
	绀轰緥锛氬亣璁?1GHz OPP 浠呭湪 SoC 娓╁害浣庝簬鏌愪釜闃堝€兼椂鎵嶅彲鐢ㄣ€係oC 妗嗘灦
```

	 if (cur_temp < temp_low_thresh) {
		/* Enable 1GHz if it was disabled */
		opp = dev_pm_opp_find_freq_exact(dev, 1000000000, false);
		dev_pm_opp_put(opp);
		/* just error check */
		if (!IS_ERR(opp))
			ret = dev_pm_opp_enable(dev, 1000000000);
		else
			goto try_something_else;
	 }

```
dev_pm_opp_disable
	浣夸竴涓?OPP 涓嶅彲鐢ㄤ簬杩愯
	绀轰緥锛氬亣璁惧綋娓╁害瓒呰繃闃堝€兼椂锛?GHz OPP 灏嗚绂佺敤銆係oC 妗嗘灦鐨勫疄鐜板彲鑳?
```

	 if (cur_temp > temp_high_thresh) {
		/* Disable 1GHz if it was enabled */
		opp = dev_pm_opp_find_freq_exact(dev, 1000000000, true);
		dev_pm_opp_put(opp);
		/* just error check */
		if (!IS_ERR(opp))
			ret = dev_pm_opp_disable(dev, 1000000000);
		else
			goto try_something_else;
	 }

```
## 5. OPP 鏁版嵁妫€绱㈠嚱鏁?

鐢变簬 OPP 搴撳 OPP 淇℃伅杩涜浜嗘娊璞★紝鍥犳闇€瑕佷粠 dev_pm_opp 缁撴瀯涓彁鍙栦俊鎭殑涓€缁勫嚱鏁般€備竴鏃︿娇鐢ㄦ悳绱㈠嚱鏁拌幏鍙栦簡 OPP 鎸囬拡锛孲oC 妗嗘灦灏卞彲浠ヤ娇鐢ㄤ互涓嬪嚱鏁版绱?OPP 灞傚唴閮ㄦ墍琛ㄧず鐨勪俊鎭€?

dev_pm_opp_get_voltage
	妫€绱?opp 鎸囬拡鎵€琛ㄧず鐨勭數鍘嬨€?
	绀轰緥锛氬湪 cpufreq 鍒囨崲鍒颁笉鍚岄鐜囨椂锛孲oC 妗嗘灦闇€瑕佷娇鐢?regulator 妗嗘灦灏?OPP 鎵€琛ㄧず鐨勭數鍘嬭缃埌鎻愪緵璇ョ數鍘嬬殑鐢垫簮绠＄悊鑺墖涓?
```

	 soc_switch_to_freq_voltage(freq)
	 {
		/* do things */
		opp = dev_pm_opp_find_freq_ceil(dev, &freq);
		v = dev_pm_opp_get_voltage(opp);
		dev_pm_opp_put(opp);
		if (v)
			regulator_set_voltage(.., v);
		/* do other things */
	 }

```
dev_pm_opp_get_freq
	妫€绱?opp 鎸囬拡鎵€琛ㄧず鐨勯鐜囥€?
	绀轰緥锛氬亣璁?SoC 妗嗘灦浣跨敤浜嗗嚑涓緟鍔╁嚱鏁帮紝鎴戜滑鍙互浼犻€?opp 鎸囬拡锛岃€屾棤闇€棰濆浼犲弬
```

	 soc_cpufreq_target(..)
	 {
		/* do things.. */
		 max_freq = ULONG_MAX;
		 max_opp = dev_pm_opp_find_freq_floor(dev,&max_freq);
		 requested_opp = dev_pm_opp_find_freq_ceil(dev,&freq);
		 if (!IS_ERR(max_opp) && !IS_ERR(requested_opp))
			r = soc_test_validity(max_opp, requested_opp);
		 dev_pm_opp_put(max_opp);
		 dev_pm_opp_put(requested_opp);
		/* do other things */
	 }
	 soc_test_validity(..)
	 {
		 if(dev_pm_opp_get_voltage(max_opp) < dev_pm_opp_get_voltage(requested_opp))
			 return -EINVAL;
		 if(dev_pm_opp_get_freq(max_opp) < dev_pm_opp_get_freq(requested_opp))
			 return -EINVAL;
		/* do things.. */
	 }

```
dev_pm_opp_get_opp_count
	妫€绱㈡煇涓澶囧彲鐢ㄧ殑 opp 鏁伴噺
	绀轰緥锛氬亣璁?SoC 涓殑鍗忓鐞嗗櫒闇€瑕佷簡瑙ｅ彲鐢ㄧ殑
```

	 soc_notify_coproc_available_frequencies()
	 {
		/* Do things */
		num_available = dev_pm_opp_get_opp_count(dev);
		speeds = kcalloc(num_available, sizeof(u32), GFP_KERNEL);
		/* populate the table in increasing order */
		freq = 0;
		while (!IS_ERR(opp = dev_pm_opp_find_freq_ceil(dev, &freq))) {
			speeds[i] = freq;
			freq++;
			i++;
			dev_pm_opp_put(opp);
		}

		soc_notify_coproc(AVAILABLE_FREQs, speeds, num_available);
		/* Do other things */
	 }

```
## 6. 鏁版嵁缁撴瀯

閫氬父锛屼竴涓?SoC 鍖呭惈澶氫釜鍙彉鐨勭數鍘嬪煙銆傛瘡涓煙鐢变竴涓澶囨寚閽堣〃绀恒€傚叾涓?OPP 鐨勫叧绯诲彲浠?
```

  SoC
   |- device 1
   |	|- opp 1 (availability, freq, voltage)
   |	|- opp 2 ..
   ...	...
   |	`- opp n ..
   |- device 2
   ...
   `- device m

```
OPP 搴撶淮鎶や竴涓敱 SoC 妗嗘灦濉厖銆佸苟鐢变笂杩板悇绫诲嚱鏁拌闂殑鍐呴儴鍒楄〃銆傜劧鑰岋紝琛ㄧず瀹為檯 OPP 鍜屽煙鐨勭粨鏋勫 OPP 搴撹嚜韬槸鍐呴儴鐨勶紝浠ュ疄鐜拌法绯荤粺鍙鐢ㄧ殑鎭板綋鎶借薄銆?

struct dev_pm_opp
	OPP 搴撶敤浜庤〃绀哄崟涓?OPP 鐨勫唴閮ㄦ暟鎹粨鏋勩€傞櫎棰戠巼銆佺數鍘嬨€佸彲鐢ㄦ€т俊鎭锛屽畠杩樺寘鍚?OPP 搴撹繍琛屾墍闇€鐨勫唴閮ㄨ璐︿俊鎭€傝缁撴瀯鐨勬寚閽堜細琚繑鍥炵粰 SoC 妗嗘灦绛夌敤鎴凤紝鐢ㄤ綔涓?OPP 灞備氦浜掓椂鏍囪瘑鏌愪釜 OPP 鐨勬爣璇嗙銆?

	璀﹀憡锛?
	  鐢ㄦ埛涓嶅簲瑙ｆ瀽鎴栦慨鏀?struct dev_pm_opp 鎸囬拡銆傛煇涓疄渚嬬殑榛樿鍊肩敱 dev_pm_opp_add 濉厖锛屼絾璇?OPP 鐨勫彲鐢ㄦ€у彲鐢?dev_pm_opp_enable/disable 鍑芥暟淇敼銆?

struct device
	杩欑敤浜庡悜 OPP 灞傛爣璇嗕竴涓煙銆傝澶囩殑鎬ц川鍙婂叾瀹炵幇鐣欑粰 OPP 搴撶殑鐢ㄦ埛锛堝 SoC 妗嗘灦锛夊喅瀹氥€?

鎬讳綋鑰岃█锛屼粠绠€鍖栫殑瑙掑害鐪嬶紝鏁版嵁缁撴瀯鎿嶄綔琛ㄧず濡備笅
```

  Initialization / modification:
              +-----+        /- dev_pm_opp_enable
  dev_pm_opp_add --> | opp | <-------
    |         +-----+        \- dev_pm_opp_disable
    \-------> domain_info(device)

  Search functions:
               /-- dev_pm_opp_find_freq_ceil  ---\   +-----+
  domain_info<---- dev_pm_opp_find_freq_exact -----> | opp |
               \-- dev_pm_opp_find_freq_floor ---/   +-----+

  Retrieval functions:
  +-----+     /- dev_pm_opp_get_voltage
  | opp | <---
  +-----+     \- dev_pm_opp_get_freq

  domain_info <- dev_pm_opp_get_opp_count

```