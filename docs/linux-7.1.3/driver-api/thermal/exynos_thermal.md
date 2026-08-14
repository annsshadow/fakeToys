## 鍐呮牳椹卞姩 exynos_tmu


Supported chips:

- ARM Samsung Exynos4, Exynos5 series of SoC

  Datasheet: Not publicly available

Authors: Donggeun Kim <dg77.kim@samsung.com>
Authors: Amit Daniel <amit.daniel@samsung.com>

### TMU 鎺у埗鍣ㄦ弿杩帮細


鏈┍鍔ㄥ厑璁歌鍙?Samsung Exynos4/5 绯诲垪 SoC 鍐呴儴鐨勬俯搴︺€?
璇ヨ姱鐗囦粎閫氳繃涓€涓瘎瀛樺櫒鏆撮湶娴嬮噺寰楀埌鐨?8 浣嶆俯搴︿唬鐮佸€笺€?娓╁害鍙敱娓╁害浠ｇ爜鎹㈢畻寰楀嚭銆?鍏辨湁涓変釜浠庢俯搴︽崲绠椾负娓╁害浠ｇ爜鐨勫叕寮忋€?
杩欎笁涓叕寮忓涓嬶細
```

	Tc = (T - 25) * (TI2 - TI1) / (85 - 25) + TI1

  2. 鍗曠偣淇暣锛圤ne point trimming锛?:

	Tc = T + TI1 - 25

  3. 鏃犱慨鏁达紙No trimming锛?:

	Tc = T + 50

  Tc:
       娓╁害浠ｇ爜锛孴锛氭俯搴︼紝
  TI1:
       25 鎽勬皬搴﹀搴旂殑淇暣淇℃伅锛堝瓨鍌ㄥ湪 TRIMINFO 瀵勫瓨鍣級
       鍦?25 鎽勬皬搴︿笅娴嬪緱鐨勩€佷繚鎸佷笉鍙樼殑娓╁害浠ｇ爜
  TI2:
       85 鎽勬皬搴﹀搴旂殑淇暣淇℃伅锛堝瓨鍌ㄥ湪 TRIMINFO 瀵勫瓨鍣級
       鍦?85 鎽勬皬搴︿笅娴嬪緱鐨勩€佷繚鎸佷笉鍙樼殑娓╁害浠ｇ爜

```
Exynos4/5 涓殑 TMU锛堢儹绠＄悊鍗曞厓锛孴hermal Management Unit锛夊湪娓╁害瓒呰繃棰勫畾涔夌骇鍒椂浜х敓涓柇銆?鍙厤缃殑闃堝€兼渶澶ф暟閲忎负浜斾釜銆?```

  Level_0: current temperature > trigger_level_0 + threshold
  Level_1: current temperature > trigger_level_1 + threshold
  Level_2: current temperature > trigger_level_2 + threshold
  Level_3: current temperature > trigger_level_3 + threshold

```
闃堝€间笌鍚勪釜 trigger_level 閫氳繃鐩稿簲鐨勫瘎瀛樺櫒璁剧疆銆?
褰撲腑鏂彂鐢熸椂锛屾湰椹卞姩閫氳繃 exynos_report_trigger 鍑芥暟閫氱煡鍐呮牳鐑鏋躲€?铏界劧鍙互涓?level_0 璁剧疆涓柇鏉′欢锛屼絾瀹冨彲鐢ㄤ簬鍚屾闄嶆俯鍔ㄤ綔銆?
### TMU 椹卞姩鎻忚堪锛?

```

					Kernel Core thermal framework
				(thermal_core.c, step_wise.c, cpufreq_cooling.c)
								^
								|
								|
  TMU configuration data -----> TMU Driver  <----> Exynos Core thermal wrapper
  (exynos_tmu_data.c)	      (exynos_tmu.c)	   (exynos_thermal_common.c)
  (exynos_tmu_data.h)	      (exynos_tmu.h)	   (exynos_thermal_common.h)

```
a) TMU 閰嶇疆鏁版嵁锛?		瀹冪敱閫氳繃缁撴瀯浣?exynos_tmu_registers 鎻忚堪鐨?TMU 瀵勫瓨鍣ㄥ亸绉?浣嶅煙缁勬垚銆傛澶栬繕浣跨敤鑻ュ共鍏朵粬骞冲彴鏁版嵁锛坰truct exynos_tmu_platform_data锛夋垚鍛樻潵閰嶇疆 TMU銆?b) TMU 椹卞姩锛?		璇ョ粍浠跺垵濮嬪寲 TMU 鎺у埗鍣ㄥ苟璁剧疆涓嶅悓鐨勯槇鍊笺€傚畠閫氳繃璋冪敤 exynos_report_trigger 鏉ヨЕ鍙戞牳蹇冪儹瀹炵幇銆?c) Exynos 鏍稿績鐑皝瑁呭眰锛圗xynos Core thermal wrapper锛夛細
		瀹冩彁渚?3 涓皝瑁呭嚱鏁颁互浣跨敤鍐呮牳鏍稿績鐑鏋讹紝鍒嗗埆鏄?exynos_unregister_thermal銆乪xynos_register_thermal 鍜?exynos_report_trigger銆?