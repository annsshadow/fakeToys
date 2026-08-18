## CPU 鏁ｇ儹 API 浣跨敤璇存槑


浣滆€咃細Amit Daniel Kachhap <amit.kachhap@linaro.org>

鏇存柊锛?015 骞?1 鏈?6 鏃?
Copyright (c)  2012 Samsung Electronics Co., Ltd(http://www.samsung.com)

## 0. 绠€浠?

閫氱敤鐨?CPU 鏁ｇ儹锛堥鐜囬檺鍒讹紝freq clipping锛夊悜璋冪敤鑰呮彁渚涙敞鍐?娉ㄩ攢 API銆傚皢鏁ｇ儹璁惧缁戝畾鍒?瑙﹀彂鐐癸紙trip point锛夌暀缁欑敤鎴峰畬鎴愩€傛敞鍐?API 杩斿洖鏁ｇ儹璁惧鎸囬拡銆?
## 1. CPU 鏁ｇ儹 API


### 1.1 cpufreq 娉ㄥ唽/娉ㄩ攢 API

```

	struct thermal_cooling_device
	*cpufreq_cooling_register(struct cpumask *clip_cpus)

    璇ユ帴鍙ｅ嚱鏁颁互鍚嶇О "thermal-cpufreq-%x" 娉ㄥ唽 cpufreq 鏁ｇ儹璁惧銆傝 API 鍙敮鎸佸涓?    cpufreq 鏁ｇ儹璁惧瀹炰緥銆?
   clip_cpus:
	灏嗘柦鍔犻鐜囩害鏉熺殑 CPU 鐨?cpumask銆?
    ::

	struct thermal_cooling_device
	*of_cpufreq_cooling_register(struct cpufreq_policy *policy)

    璇ユ帴鍙ｅ嚱鏁颁互鍚嶇О "thermal-cpufreq-%x" 娉ㄥ唽 cpufreq 鏁ｇ儹璁惧锛屽苟灏嗗叾涓庝竴涓澶囨爲
    鑺傜偣鍏宠仈锛屼互渚块€氳繃 thermal DT 浠ｇ爜杩涜缁戝畾銆傝 API 鍙敮鎸佸涓?cpufreq 鏁ｇ儹璁惧瀹炰緥銆?
    policy:
	CPUFreq policy銆?

    ::

	void cpufreq_cooling_unregister(struct thermal_cooling_device *cdev)

    璇ユ帴鍙ｅ嚱鏁版敞閿€ "thermal-cpufreq-%x" 鏁ｇ儹璁惧銆?
    cdev: 闇€瑕佹敞閿€鐨勬暎鐑澶囨寚閽堛€?
```
## 2. 鍔熻€楁ā鍨?

鍔熻€?API 娉ㄥ唽鍑芥暟涓?CPU 鎻愪緵浜嗕竴涓畝鍗曠殑鍔熻€楁ā鍨嬨€傚綋鍓嶅姛鑰楁寜鍔ㄦ€佸姛鑰楄绠楋紙闈欐€佸姛鑰?褰撳墠涓嶆敮鎸侊級銆傝鍔熻€楁ā鍨嬭姹?CPU 鐨勫伐浣滅偣锛坥perating-points锛夊凡浣跨敤鍐呮牳鐨?OPP 搴撴敞鍐岋紝
涓?`cpufreq_frequency_table` 宸茶祴缁?CPU 鐨?`struct device`銆傚鏋滀綘浣跨敤
CONFIG_CPUFREQ_DT锛岄偅涔?`cpufreq_frequency_table` 搴旇宸茬粡璧嬬粰浜?CPU 璁惧銆?
澶勭悊鍣ㄧ殑鍔ㄦ€佸姛鑰楁秷鑰楀彇鍐充簬璁稿鍥犵礌銆傚浜庣粰瀹氱殑澶勭悊鍣ㄥ疄鐜帮紝涓昏鍥犵礌鏈夛細

- 澶勭悊鍣ㄨ姳璐瑰湪杩愯銆佹秷鑰楀姩鎬佸姛鑰楃殑鏃堕棿锛屼笌澶勪簬绌洪棽鐘舵€併€佸姩鎬佹秷鑰楀彲蹇界暐鐨勬椂闂翠箣姣斻€?  杩欓噷鎴戜滑绉板叾涓衡€滃埄鐢ㄧ巼锛坲tilisation锛夆€濄€?- 鐢?DVFS 浜х敓鐨勭數鍘嬩笌棰戠巼姘村钩銆侱VFS 姘村钩鏄敮閰嶅姛鑰楃殑涓诲鍥犵礌銆?- 鍦ㄨ繍琛屾椂闂村唴锛屸€滄墽琛屸€濊涓猴紙鎸囦护绫诲瀷銆佸唴瀛樿闂ā寮忕瓑锛夊湪澶氭暟鎯呭喌涓嬮€犳垚浜岄樁鍙樺寲銆?  鍦ㄦ瀬绔儏鍐典笅杩欑鍙樺寲鍙兘寰堟樉钁楋紝浣嗛€氬父鍏跺奖鍝嶈繙灏忎簬涓婅堪鍥犵礌銆?
```

	Pdyn = f(run) * Voltage^2 * Frequency * Utilisation

```
杩欓噷鐨?f(run) 琛ㄧず涓婅堪鎵ц琛屼负锛屽叾缁撴灉鍗曚綅涓?Watts/Hz/Volt^2锛堝父琛ㄧず涓?mW/MHz/uVolt^2锛夈€?
f(run) 鐨勮缁嗚涓哄彲浠ュ缓妯′负鍦ㄧ嚎锛坥n-line锛夋ā鍨嬨€傜劧鑰屽疄闄呬笂锛岃繖鏍风殑鍦ㄧ嚎妯″瀷渚濊禆浜庤嫢骞?瀹炵幇鐗瑰畾鐨勫鐞嗗櫒鏀寔涓庣壒鎬у埢鐢诲洜绱犮€傚洜姝わ紝鍦ㄥ垵濮嬪疄鐜颁腑璇ラ」璐＄尞鐢ㄤ竴涓父鏁扮郴鏁拌〃绀恒€?杩欐槸涓€涓笌鏁翠綋鍔熻€楀彉鍖栫浉瀵硅础鐚浉涓€鑷寸殑绠€鍖栥€?
```

	Pdyn = Capacitance * Voltage^2 * Frequency * Utilisation

```
鍏朵腑 `capacitance` 鏄竴涓父鏁帮紝浠ュ熀纭€鍗曚綅 mW/MHz/uVolt^2 琛ㄧず鎸囩ず鎬х殑杩愯鏃跺姩鎬佸姛鑰?绯绘暟銆傜Щ鍔?CPU 鐨勫吀鍨嬪€煎彲鑳藉湪 100 鍒?500 涔嬮棿銆備綔涓哄弬鑰冿紝ARM Juno 寮€鍙戝钩鍙颁腑 SoC 鐨?杩戜技鍊煎浜?Cortex-A57 绨囦负 530锛屽浜?Cortex-A53 绨囦负 140銆?