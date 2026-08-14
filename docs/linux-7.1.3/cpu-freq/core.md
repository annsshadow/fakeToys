
## CPUFreq 鏍稿績涓?CPUFreq 閫氱煡鍣ㄧ殑涓€鑸€ф弿杩?

浣滆€咃細
 - Dominik Brodowski  <linux@brodo.de>
 - David Kimdon <dwhedon@debian.org>
 - Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 - Viresh Kumar <viresh.kumar@linaro.org>


   1. CPUFreq 鏍稿績涓庢帴鍙?   2. CPUFreq 閫氱煡鍣?   3. 浣跨敤宸ヤ綔鎬ц兘鐐癸紙OPP锛夌敓鎴?CPUFreq 琛?
## 1. 涓€鑸俊鎭?

CPUFreq 鏍稿績浠ｇ爜浣嶄簬 drivers/cpufreq/cpufreq.c銆傝 cpufreq 浠ｇ爜涓?CPUFreq
鏋舵瀯椹卞姩锛堝嵆鐪熸鎵ц棰戠巼鍒囨崲鐨勯偅閮ㄥ垎浠ｇ爜锛変互鍙娾€滈€氱煡鍣紙notifier锛夆€濇彁渚?鏍囧噯鍖栫殑鎺ュ彛銆傝繖浜涙槸璁惧椹卞姩鎴栧唴鏍哥殑鍏朵粬閮ㄥ垎锛屽畠浠渶瑕佸湪绛栫暐鏀瑰彉鏃讹紙渚嬪
鍍?ACPI 杩欐牱鐨勭儹妯″潡锛夋垨鎵€鏈夐鐜囨敼鍙樻椂锛堜緥濡傝鏃朵唬鐮侊級寰楀埌閫氱煡锛岀敋鑷抽渶瑕?寮哄埗鏌愪簺閫熷害闄愬埗锛堜緥濡?ARM 鏋舵瀯涓婄殑 LCD 椹卞姩锛夈€傛澶栵紝鍐呮牳鈥滃父閲忊€?loops_per_jiffy 浼氬湪棰戠巼鏀瑰彉鏃跺湪姝ゅ鏇存柊銆?
cpufreq 绛栫暐鐨勫紩鐢ㄨ鏁扮敱 cpufreq_cpu_get 鍜?cpufreq_cpu_put 瀹屾垚锛屽畠浠?纭繚 cpufreq 椹卞姩宸叉纭悜鏍稿績娉ㄥ唽锛屽苟涓斿湪璋冪敤 cpufreq_put_cpu 涔嬪墠涓嶄細琚?鍗歌浇銆傝繖涔熺‘淇濅簡鐩稿簲鐨?cpufreq 绛栫暐鍦ㄨ浣跨敤鏃朵笉浼氳閲婃斁銆?
## 2. CPUFreq 閫氱煡鍣?

CPUFreq 閫氱煡鍣ㄩ伒寰爣鍑嗙殑鍐呮牳閫氱煡鍣ㄦ帴鍙ｃ€傚叧浜庨€氱煡鍣ㄧ殑缁嗚妭鍙傝
linux/include/linux/notifier.h銆?
鏈変袱绉嶄笉鍚岀殑 CPUFreq 閫氱煡鍣ㄢ€斺€旂瓥鐣ラ€氱煡鍣ㄥ拰鍒囨崲閫氱煡鍣ㄣ€?

### 2.1 CPUFreq 绛栫暐閫氱煡鍣?

褰撳垱寤烘垨绉婚櫎涓€涓柊绛栫暐鏃讹紝浼氶€氱煡杩欎簺閫氱煡鍣ㄣ€?
闃舵锛坧hase锛夌敱浼犵粰閫氱煡鍣ㄧ殑绗簩涓弬鏁版寚瀹氥€傚綋绛栫暐棣栨鍒涘缓鏃堕樁娈典负
CPUFREQ_CREATE_POLICY锛岀Щ闄ょ瓥鐣ユ椂涓?CPUFREQ_REMOVE_POLICY銆?
绗笁涓弬鏁版槸涓€涓?`void *pointer`锛屾寚鍚戜竴涓?struct cpufreq_policy锛?鍏朵腑鍖呭惈鑻ュ共鍊硷紝鍖呮嫭 min銆乵ax锛堟柊绛栫暐鐨勪笂涓嬮檺棰戠巼锛屽崟浣?kHz锛夈€?

### 2.2 CPUFreq 鍒囨崲閫氱煡鍣?

瀵逛簬绛栫暐涓殑姣忎釜鍦ㄧ嚎 CPU锛屽綋 CPUfreq 椹卞姩鍒囨崲 CPU 鏍稿績棰戠巼涓旇鏀瑰彉娌℃湁
浠讳綍澶栭儴褰卞搷鏃讹紝浼氶€氱煡杩欎簺閫氱煡鍣ㄤ袱娆°€?
绗簩涓弬鏁版寚瀹氶樁娈碘€斺€擟PUFREQ_PRECHANGE 鎴?CPUFREQ_POSTCHANGE銆?
绗笁涓弬鏁版槸涓€涓?struct cpufreq_freqs锛屽寘鍚互涓嬪€硷細

======	======================================
policy	鎸囧悜 struct cpufreq_policy 鐨勬寚閽?old	鏃ч鐜?new	鏂伴鐜?flags	cpufreq 椹卞姩鐨勬爣蹇?======	======================================

## 3. 浣跨敤宸ヤ綔鎬ц兘鐐癸紙OPP锛夌敓鎴?CPUFreq 琛?
鍏充簬 OPP 鐨勭粏鑺傦紝鍙傝 Documentation/power/opp.rst

dev_pm_opp_init_cpufreq_table -
	璇ュ嚱鏁版彁渚涗竴涓嵆鍙栧嵆鐢ㄧ殑杞崲渚嬬▼锛屾妸 OPP 灞傚唴閮ㄥ叧浜庡彲鐢ㄩ鐜囩殑淇℃伅
	缈昏瘧鎴愪竴绉嶅彲浠ユ柟渚垮湴鎻愪緵缁?cpufreq 鐨勬牸寮忋€?
```

	   Do not use this function in interrupt context.

	Example::

	 soc_pm_init()
	 {
		/* Do things */
		r = dev_pm_opp_init_cpufreq_table(dev, &freq_table);
		if (!r)
			policy->freq_table = freq_table;
		/* Do other things */
	 }

	.. note::

	   This function is available only if CONFIG_CPU_FREQ is enabled in
	   addition to CONFIG_PM_OPP.

```
dev_pm_opp_free_cpufreq_table
	閲婃斁鐢?dev_pm_opp_init_cpufreq_table 鍒嗛厤鐨勮〃

