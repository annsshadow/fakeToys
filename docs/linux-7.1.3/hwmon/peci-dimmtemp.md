
## 鍐呮牳椹卞姩 peci-dimmtemp


鏀寔鐨勮澶囷細
	涓嬭堪杩炴帴鍒?PECI 鎬荤嚎鐨?Intel 鏈嶅姟鍣?CPU 涔嬩竴銆?  - Intel Xeon E5/E7 v3 鏈嶅姟鍣ㄥ鐞嗗櫒
			Intel Xeon E5-14xx v3 绯诲垪
			Intel Xeon E5-24xx v3 绯诲垪
			Intel Xeon E5-16xx v3 绯诲垪
			Intel Xeon E5-26xx v3 绯诲垪
			Intel Xeon E5-46xx v3 绯诲垪
			Intel Xeon E7-48xx v3 绯诲垪
			Intel Xeon E7-88xx v3 绯诲垪
  - Intel Xeon E5/E7 v4 鏈嶅姟鍣ㄥ鐞嗗櫒
			Intel Xeon E5-16xx v4 绯诲垪
			Intel Xeon E5-26xx v4 绯诲垪
			Intel Xeon E5-46xx v4 绯诲垪
			Intel Xeon E7-48xx v4 绯诲垪
			Intel Xeon E7-88xx v4 绯诲垪
  - Intel Xeon Scalable 鏈嶅姟鍣ㄥ鐞嗗櫒
			Intel Xeon D 绯诲垪
			Intel Xeon Bronze 绯诲垪
			Intel Xeon Silver 绯诲垪
			Intel Xeon Gold 绯诲垪
			Intel Xeon Platinum 绯诲垪

	Datasheet: Available from http://www.intel.com/design/literature.htm

Author: Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>

### 鎻忚堪


鏈┍鍔ㄥ疄鐜颁簡涓€涓€氱敤鐨?PECI hwmon 鐗规€э紝鎻愪緵鍙€氳繃澶勭悊鍣?PECI 鎺ュ彛璁块棶鐨?DIMM 娓╁害浼犳劅璇绘暟銆?
鎵€鏈夋俯搴﹀€煎潎浠ユ鎽勬皬搴︾粰鍑猴紝涓斾粎鍦ㄧ洰鏍?CPU 涓婄數鏃跺彲娴嬮噺銆?
### Sysfs 鎺ュ彛


======================= =======================================================

temp[N]_label		鎻愪緵瀛楃涓?"DIMM CI"锛屽叾涓?C 涓?DIMM 閫氶亾锛孖 涓哄凡瀹夎 DIMM 鐨勭储寮曘€?temp[N]_input		鎻愪緵宸插畨瑁?DIMM 鐨勫綋鍓嶆俯搴︺€?temp[N]_max		鎻愪緵 DIMM 鐨勭儹鎺у埗娓╁害銆?temp[N]_crit		鎻愪緵 DIMM 鐨勫叧鏂俯搴︺€?
======================= =======================================================

璇存槑锛?	DIMM 娓╁害灞炴€т細鍦ㄥ鎴风 CPU 鐨?BIOS 瀹屾垚鍐呭瓨璁粌涓庢祴璇曞悗鍑虹幇銆?