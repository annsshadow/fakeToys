## OMAP PM 鎺ュ彛


鏈枃妗ｆ弿杩颁簡涓存椂鐨?OMAP PM 鎺ュ彛銆傞┍鍔ㄥ紑鍙戣€呬娇鐢ㄨ繖浜涘嚱鏁帮紝鍚戝唴鏍哥數婧愮鐞嗕唬鐮佷紶杈炬渶浣庡欢杩熸垨鍚炲悙閲忕害鏉熴€傞暱杩滅洰鏍囨槸鎶?OMAP PM 鎺ュ彛鐨勭壒鎬у悎骞跺埌 Linux PM QoS 浠ｇ爜涔嬩腑銆?
椹卞姩闇€瑕佽〃杈炬弧瓒充互涓嬫潯浠剁殑鐢垫簮绠＄悊鍙傛暟锛?
- 鏀寔 TI SRF 涓瓨鍦ㄧ殑鐢垫簮绠＄悊鍙傛暟鑼冨洿锛?
- 灏嗛┍鍔ㄤ笌搴曞眰鐨?PM 鍙傛暟瀹炵幇鐩稿垎绂伙紝鏃犺鍏舵槸 TI SRF銆丩inux PM QoS銆丩inux 寤惰繜妗嗘灦锛岃繕鏄叾浠栧疄鐜帮紱

- 浠ュ熀鏈崟浣嶏紙渚嬪寤惰繜鍜屽悶鍚愰噺锛夛紝鑰岄潪 OMAP 涓撴湁鎴栫壒瀹?OMAP 鍙樹綋涓撴湁鐨勫崟浣嶆潵鎸囧畾 PM 鍙傛暟锛?
- 鍏佽涓庡叾浠栨灦鏋勶紙渚嬪 DaVinci锛夊叡浜殑椹卞姩锛屼互涓嶅奖鍝嶉潪 OMAP 绯荤粺鐨勬柟寮忔坊鍔犺繖浜涚害鏉燂紱

- 鑳藉绔嬪嵆瀹炵幇锛屼笖瀵瑰叾浠栨灦鏋勭殑骞叉壈鏈€灏忋€?

鏈枃妗ｆ彁鍑?OMAP PM 鎺ュ彛锛屽寘鍚緵椹卞姩浠ｇ爜浣跨敤鐨勪互涓嬩簲涓數婧愮鐞嗗嚱鏁帮細

```
   (*pdata->set_max_mpu_wakeup_lat)(struct device *dev, unsigned long t)
```
```
   (*pdata->set_max_dev_wakeup_lat)(struct device *dev, unsigned long t)
```
```
   (*pdata->set_max_sdma_lat)(struct device *dev, long t)
```
```
   (*pdata->set_min_bus_tput)(struct device *dev, u8 agent_id, unsigned long r)
```
```
   (*pdata->get_dev_context_loss_count)(struct device *dev)
```

鎵€鏈?OMAP PM 鎺ュ彛鍑芥暟鐨勮繘涓€姝ユ枃妗ｅ彲鍦?arch/arm/plat-omap/include/mach/omap-pm.h 涓壘鍒般€?

### OMAP PM 灞傝璁′负涓存椂鏂规


鐩爣鏄渶缁堢敱 Linux PM QoS 灞傛敮鎸?OMAP3 涓瓨鍦ㄧ殑鐢垫簮绠＄悊鐗规€ц寖鍥淬€傞殢鐫€杩欎竴鐩爣瀹炵幇锛屼娇鐢?OMAP PM 鎺ュ彛鐨勬棦鏈夐┍鍔ㄥ彲浠ヤ慨鏀逛负浣跨敤 Linux PM QoS 浠ｇ爜锛涘眾鏃?OMAP PM 鎺ュ彛渚垮彲琚Щ闄ゃ€?

### 椹卞姩瀵?OMAP PM 鍑芥暟鐨勪娇鐢?

姝ｅ涓婅堪绀轰緥涓殑 'pdata' 鎵€绀猴紝杩欎簺鍑芥暟閫氳繃椹卞姩 `.platform_data` 缁撴瀯涓殑鍑芥暟鎸囬拡鏆撮湶缁欓┍鍔ㄣ€傝繖浜涘嚱鏁版寚閽堢敱 `board-*.c` 鏂囦欢鍒濆鍖栵紝鎸囧悜鐩稿簲鐨?OMAP PM 鍑芥暟锛?
- set_max_dev_wakeup_lat 灏嗘寚鍚?omap_pm_set_max_dev_wakeup_lat() 绛夈€備笉鏀寔杩欎簺鍑芥暟鐨勫叾浠栨灦鏋勫簲灏嗚繖绫诲嚱鏁版寚閽堜繚鎸佷负 NULL銆?
```
        if (pdata->set_max_dev_wakeup_lat)
            (*pdata->set_max_dev_wakeup_lat)(dev, t);
```

杩欎簺鍑芥暟鏈€甯歌鐨勭敤娉曞ぇ姒傛槸锛氭寚瀹氫粠涓柇鍙戠敓鍒拌澶囧彉涓哄彲璁块棶涔嬮棿鐨勬渶澶ф椂闂淬€備负姝わ紝椹卞姩缂栧啓鑰呭簲浣?set_max_mpu_wakeup_lat() 鍑芥暟绾︽潫 MPU 鍞ら啋寤惰繜锛屽苟浣跨敤 set_max_dev_wakeup_lat() 鍑芥暟绾︽潫璁惧鍞ら啋寤惰繜锛堜粠 clk_enable() 鍒板彲璁块棶锛夈€備緥濡傦細

```
        /* Limit MPU wakeup latency */
        if (pdata->set_max_mpu_wakeup_lat)
            (*pdata->set_max_mpu_wakeup_lat)(dev, tc);

        /* Limit device powerdomain wakeup latency */
        if (pdata->set_max_dev_wakeup_lat)
            (*pdata->set_max_dev_wakeup_lat)(dev, td);

        /* total wakeup latency in this example: (tc + td) */
```

鍙互閫氳繃鍐嶆璋冪敤璇ュ嚱鏁板苟浼犲叆鏂板€兼潵瑕嗙洊 PM 鍙傛暟銆傚彲浠ラ€氳繃灏?t 鍙傛暟璁句负 -1 鏉ョЩ闄よ缃紙set_max_bus_tput() 闄ゅ锛屽畠搴斾互 r 鍙傛暟璁句负 0 鏉ヨ皟鐢級銆?
涓婅堪绗簲涓嚱鏁?omap_pm_get_dev_context_loss_count()锛屾棬鍦ㄤ綔涓轰竴绉嶄紭鍖栵紝浣块┍鍔ㄨ兘澶熷垽鏂澶囨槸鍚﹀凡涓㈠け鍏跺唴閮ㄤ笂涓嬫枃銆傚鏋滀笂涓嬫枃宸蹭涪澶憋紝椹卞姩蹇呴』鍦ㄧ户缁箣鍓嶆仮澶嶅叾鍐呴儴涓婁笅鏂囥€?

### 鍏朵粬涓撶敤鎺ュ彛鍑芥暟


涓婇潰鍒楀嚭鐨勪簲涓嚱鏁版棬鍦ㄤ緵浠讳綍璁惧椹卞姩浣跨敤銆侱SPBridge 鍜?CPUFreq 鏈変竴浜涚壒娈婇渶姹傘€侱SPBridge 浠?OPP ID 鐨勫舰寮忚〃杈剧洰鏍?DSP 鎬ц兘绾у埆銆侰PUFreq 浠?MPU 棰戠巼鐨勫舰寮忚〃杈剧洰鏍?MPU 鎬ц兘绾у埆銆侽MAP PM 鎺ュ彛涓鸿繖浜涗笓鐢ㄥ満鏅彁渚涗簡鍑芥暟锛岀敤浜庡皢璇ヨ緭鍏ヤ俊鎭紙OPP/MPU 棰戠巼锛夎浆鎹负搴曞眰鐢垫簮绠＄悊瀹炵幇鎵€闇€鐨勫舰寮忥細

6. `(*pdata->dsp_get_opp_table)(void)`

7. `(*pdata->dsp_set_min_opp)(u8 opp_id)`

8. `(*pdata->dsp_get_opp)(void)`

9. `(*pdata->cpu_get_freq_table)(void)`

10. `(*pdata->cpu_set_freq)(unsigned long f)`

11. `(*pdata->cpu_get_freq)(void)`

## 涓哄钩鍙板畾鍒?OPP

瀹氫箟 CONFIG_PM 搴斿綋浼氫负纭呯墖鍚敤 OPP 灞傦紝骞朵笖 OPP 琛ㄧ殑娉ㄥ唽搴斿綋鑷姩杩涜銆傜劧鑰屽湪鐗规畩鎯呭喌涓嬶紝榛樿鐨?OPP 琛ㄥ彲鑳介渶瑕佽皟鏁达紝渚嬪锛?
 - 鍚敤榛樿琚鐢ㄣ€佷絾鍦ㄦ煇骞冲彴涓婂彲浠ュ惎鐢ㄧ殑榛樿 OPP
 - 鍦ㄨ骞冲彴涓婄鐢ㄤ竴涓笉鍙楁敮鎸佺殑 OPP
 - 瀹氫箟骞舵坊鍔犺嚜瀹氫箟鐨?OPP 琛ㄩ」

鍦ㄨ繖浜涙儏鍐典笅锛屾澘绾ф枃浠堕渶瑕佹墽琛屽涓嬮澶栨楠わ細

```
	#include "pm.h"
	....
	static void __init omap_xyz_init_irq(void)
	{
		....
		/* Initialize the default table */
		omapx_opp_init();
		/* Do customization to the defaults */
		....
	}
```

娉ㄦ剰锛?  omapx_opp_init 灏嗕緷鎹?omap 绯诲垪鎴愪负 omap3_opp_init 鎴栫浉搴斿悕绉般€?