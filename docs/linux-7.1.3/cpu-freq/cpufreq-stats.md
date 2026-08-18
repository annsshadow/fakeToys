
## sysfs CPUFreq 缁熻閫氱敤鎻忚堪


闈㈠悜鐢ㄦ埛鐨勪俊鎭?

Author: Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>


   1. 绠€浠?   2. 鎻愪緵鐨勭粺璁★紙闄勭ず渚嬶級
   3. 閰嶇疆 cpufreq-stats


## 1. 绠€浠?

cpufreq-stats 鏄竴涓负姣忎釜 CPU 鎻愪緵 CPU 棰戠巼缁熻鐨勯┍鍔ㄣ€傝繖浜涚粺璁′互涓€缁勫彧璇绘帴鍙ｇ殑褰㈠紡
鍦?/sysfs 涓彁渚涖€傝鎺ュ彛锛堥厤缃悗锛変細涓烘瘡涓?CPU 鍑虹幇鍦?/sysfs 涓?cpufreq 涓嬬殑涓€涓嫭绔?鐩綍锛?sysfs root>/devices/system/cpu/cpuX/cpufreq/stats/锛夈€傚悇绉嶇粺璁℃暟鎹皢鏋勬垚璇ョ洰褰曚笅鐨?鍙鏂囦欢銆?
璇ラ┍鍔ㄨ璁捐涓虹嫭绔嬩簬浠讳綍鍙兘杩愯鍦ㄤ綘 CPU 涓婄殑鐗瑰畾 cpufreq_driver銆傚洜姝わ紝瀹冨彲浠ヤ笌浠讳綍
cpufreq_driver 涓€璧峰伐浣溿€?

## 2. 鎻愪緵鐨勭粺璁★紙闄勭ず渚嬶級


cpufreq stats 鎻愪緵浠ヤ笅缁熻锛堜笅鏂囪缁嗚В閲婏級銆?
- time_in_state
- total_trans
- trans_table

鎵€鏈夌粺璁℃暟鎹兘浠?stats 椹卞姩琚彃鍏ワ紙鎴?stats 琚噸缃級鐨勬椂鍒昏捣锛屽埌浣犺鍙栨煇涓壒瀹氱粺璁＄殑鏃跺埢
涓烘銆傛樉鐒讹紝stats 椹卞姩涓嶄細鎷ユ湁浠讳綍鍏充簬 stats 椹卞姩鎻掑叆涔嬪墠鐨勯鐜囧垏鎹㈢殑淇℃伅銆?
```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # ls -l
    total 0
    drwxr-xr-x  2 root root    0 May 14 16:06 .
    drwxr-xr-x  3 root root    0 May 14 15:58 ..
    --w-------  1 root root 4096 May 14 16:06 reset
    -r--r--r--  1 root root 4096 May 14 16:06 time_in_state
    -r--r--r--  1 root root 4096 May 14 16:06 total_trans
    -r--r--r--  1 root root 4096 May 14 16:06 trans_table

```
- **reset**

鍙啓灞炴€э紝鍙敤浜庨噸缃粺璁¤鏁板櫒銆傝繖瀵逛簬鍦ㄤ笉鍚岃皟閫熷櫒锛坓overnor锛変笅璇勪及绯荤粺琛屼负鏃跺緢鏈夌敤锛?鑰屾棤闇€閲嶅惎銆?
- **time_in_state**

杩欑粰鍑鸿 CPU 鍦ㄦ瘡涓墍鏀寔棰戠巼涓婅姳璐圭殑鏃堕棿閲忋€俢at 杈撳嚭姣忎竴琛屽皢鏈変竴涓?"<frequency> <time>"
瀵癸紝琛ㄧず璇?CPU 鍦?<frequency> 涓婅姳璐逛簡 <time> 涓敤鎴锋椂闂村崟浣嶃€傝緭鍑哄姣忎釜鎵€鏀寔鐨勯鐜囦細鏈変竴琛屻€?杩欓噷鐨勭敤鎴锋椂闂村崟浣嶆槸 10mS锛堢被浼间簬 /proc 涓鍑虹殑鍏跺畠鏃堕棿锛夈€?
```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat time_in_state
    3600000 2089
    3400000 136
    3200000 34
    3000000 67
    2800000 172488


```
- **total_trans**

杩欑粰鍑鸿 CPU 涓婇鐜囧垏鎹㈢殑鎬绘鏁般€俢at 杈撳嚭灏嗘湁涓€涓崟鐙殑璁℃暟锛屽嵆棰戠巼鍒囨崲鐨勬€绘鏁般€?
```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat total_trans
    20

```
- **trans_table**

杩欏皢缁欏嚭鍏充簬鎵€鏈?CPU 棰戠巼鍒囨崲鐨勭粏绮掑害淇℃伅銆傝繖閲岀殑 cat 杈撳嚭鏄竴涓簩缁寸煩闃碉紝鍏朵腑鏉＄洰
<i,j>锛堢 i 琛岋紝绗?j 鍒楋級琛ㄧず浠?Freq_i 鍒?Freq_j 鐨勫垏鎹㈡鏁般€侳req_i 琛屽拰 Freq_j 鍒?閬靛惊椹卞姩鏈€鍒濆悜 cpufreq 鏍稿績鎻愪緵棰戠巼琛ㄦ椂鐨勬帓搴忛『搴忥紝鍥犳鍙互鏄凡鎺掑簭锛堝崌搴忔垨闄嶅簭锛夋垨鏈帓搴忋€?杩欓噷鐨勮緭鍑轰篃鍖呭惈姣忚姣忓垪鐨勫疄闄呴鐜囧€间互鎻愰珮鍙鎬с€?
濡傛灉鍒囨崲琛ㄥぇ浜?PAGE_SIZE锛岃鍙栧畠灏嗚繑鍥?-EFBIG 閿欒銆?
```

    <mysystem>:/sys/devices/system/cpu/cpu0/cpufreq/stats # cat trans_table
    From  :    To
	    :   3600000   3400000   3200000   3000000   2800000
    3600000:         0         5         0         0         0
    3400000:         4         0         2         0         0
    3200000:         0         1         0         2         0
    3000000:         0         0         1         0         3
    2800000:         0         0         0         2         0

```
## 3. 閰嶇疆 cpufreq-stats


```

	Config Main Menu
		Power management options (ACPI, APM)  --->
			CPU Frequency scaling  --->
				[*] CPU Frequency scaling
				[*]   CPU frequency translation statistics


```
瑕侀厤缃?cpufreq-stats锛屽簲鍚敤 "CPU Frequency scaling"锛圕ONFIG_CPU_FREQ锛夈€?
"CPU frequency translation statistics"锛圕ONFIG_CPU_FREQ_STAT锛夋彁渚涘寘鍚?time_in_state銆?total_trans 鍜?trans_table 鐨勭粺璁°€?
涓€鏃﹀惎鐢ㄦ閫夐」涓斾綘鐨?CPU 鏀寔 cpufrequency锛屼綘灏辫兘鍦?/sysfs 涓湅鍒?CPU 棰戠巼缁熻銆?