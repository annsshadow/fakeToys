## 鍐呮牳椹卞姩 peci-cputemp


鏀寔鐨勮姱鐗囷細
	涓嬫柟鎵€鍒楃殑鏌愭杩炴帴鍒?PECI 鎬荤嚎鐨?Intel 鏈嶅姟鍣?CPU銆?  - Intel Xeon E5/E7 v3 鏈嶅姟鍣ㄥ鐞嗗櫒
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
  - Intel Xeon 鍙墿灞曟湇鍔″櫒澶勭悊鍣?			Intel Xeon D 绯诲垪
			Intel Xeon Bronze 绯诲垪
			Intel Xeon Silver 绯诲垪
			Intel Xeon Gold 绯诲垪
			Intel Xeon Platinum 绯诲垪

	Datasheet: Available from http://www.intel.com/design/literature.htm

Author: Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>

### 鎻忚堪


璇ラ┍鍔ㄥ疄鐜颁簡閫氱敤鐨?PECI hwmon 鍔熻兘锛屾彁渚涘彲閫氳繃澶勭悊鍣?PECI 鎺ュ彛璁块棶鐨?CPU
灏佽涓?CPU 鏍稿績鐨勬暟瀛楃儹浼犳劅鍣紙DTS锛夋俯搴﹁鏁般€?
鎵€鏈夋俯搴﹀€煎潎浠ユ鎽勬皬搴︾粰鍑猴紝骞朵笖浠呭綋鐩爣 CPU 涓婄數鏃舵墠鍙祴閲忋€?
### Sysfs 鎺ュ彛


======================= =======================================================
temp1_label		"Die"
temp1_input		鎻愪緵 CPU 灏佽鐨勫綋鍓嶈姱鐗囨俯搴︺€?temp1_max		鎻愪緵 CPU 灏佽鐨勭儹鎺у埗娓╁害锛屼篃绉颁负 Tcontrol銆?temp1_crit		鎻愪緵 CPU 灏佽鐨勫叧鏈烘俯搴︼紝涔熺О涓哄鐞嗗櫒鏈€澶х粨娓?			Tjmax 鎴?Tprochot銆?temp1_crit_hyst		鎻愪緵 CPU 灏佽鐨勮繜婊炴俯搴︺€傝繑鍥?Tcontrol锛屽嵆涓寸晫
			鏉′欢瑙ｉ櫎鏃剁殑娓╁害銆?temp2_label		"DTS"
temp2_input		鎻愪緵 CPU 灏佽鐨勫綋鍓嶆俯搴︼紝宸茬缉鏀句互鍖归厤 DTS 鐑洸绾裤€?temp2_max		鎻愪緵 CPU 灏佽鐨勭儹鎺у埗娓╁害锛屼篃绉颁负 Tcontrol銆?temp2_crit		鎻愪緵 CPU 灏佽鐨勫叧鏈烘俯搴︼紝涔熺О涓哄鐞嗗櫒鏈€澶х粨娓?			Tjmax 鎴?Tprochot銆?temp2_crit_hyst		鎻愪緵 CPU 灏佽鐨勮繜婊炴俯搴︺€傝繑鍥?Tcontrol锛屽嵆涓寸晫
			鏉′欢瑙ｉ櫎鏃剁殑娓╁害銆?temp3_label		"Tcontrol"
temp3_input		鎻愪緵 CPU 灏佽鐨勫綋鍓?Tcontrol 娓╁害锛屼篃绉颁负椋庢墖娓╁害鐩爣銆?			琛ㄧず鐑洃瑙嗗櫒瑙﹀彂娓╁害鐨勭浉瀵瑰€硷紝杈惧埌璇ユ俯搴︽椂搴斿惎鍔ㄩ鎵囥€?temp3_crit		鎻愪緵 CPU 灏佽鐨?Tcontrol 涓寸晫鍊硷紝涓?Tjmax 鐩稿悓銆?temp4_label		"Tthrottle"
temp4_input		鎻愪緵 CPU 灏佽鐨勫綋鍓?Tthrottle 娓╁害銆傜敤浜庤妭娴佹俯搴︺€傝嫢璇ュ€?			琚厑璁镐笖浣庝簬 Tjmax锛屽垯浼氬彂鐢熻妭娴侊紝骞跺湪浣庝簬 Tjmax 鏃舵姤鍛娿€?temp5_label		"Tjmax"
temp5_input		鎻愪緵 CPU 灏佽鐨勬渶澶х粨娓?Tjmax銆?temp[6-N]_label		鎻愪緵瀛楃涓?鈥淐ore X鈥濓紝鍏朵腑 X 涓鸿В鏋愬嚭鐨勬牳蹇冪紪鍙枫€?temp[6-N]_input		鎻愪緵姣忎釜鏍稿績鐨勫綋鍓嶆俯搴︺€?======================= =======================================================
