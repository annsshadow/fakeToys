## 鍐呮牳椹卞姩 inspur-ipsps1


鏀寔鐨勮姱鐗囷細

  - Inspur Power System 鐢垫簮渚涘簲鍗曞厓

Author: John Wang <wangzqbj@inspur.com>

### 鎻忚堪


璇ラ┍鍔ㄦ敮鎸?Inspur Power System 鐢垫簮銆傝椹卞姩鏄牳蹇?PMBus 椹卞姩鐨勪竴涓鎴风銆?
### 浣跨敤璇存槑


璇ラ┍鍔ㄤ笉浼氳嚜鍔ㄦ娴嬭澶囥€備綘蹇呴』鏄惧紡瀹炰緥鍖栬澶囥€傝缁嗕俊鎭鍙傞槄 Documentation/i2c/instantiating-devices.rst銆?
### Sysfs 鎺ュ彛


鏀寔浠ヤ笅灞炴€э細

======================= ======================================================
curr1_input		娴嬪緱鐨勮緭鍏ョ數娴?curr1_label		"iin"
curr1_max		鏈€澶х數娴?curr1_max_alarm		鐢垫祦杩囬珮鎶ヨ
curr2_input		娴嬪緱鐨勮緭鍑虹數娴侊紙鍗曚綅 mA锛夈€?curr2_label		"iout1"
curr2_crit		涓寸晫鏈€澶х數娴?curr2_crit_alarm	鐢垫祦涓寸晫杩囬珮鎶ヨ
curr2_max		鏈€澶х數娴?curr2_max_alarm		鐢垫祦杩囬珮鎶ヨ

fan1_alarm		椋庢墖 1 璀﹀憡銆?fan1_fault		椋庢墖 1 鏁呴殰銆?fan1_input		椋庢墖 1 杞€燂紙鍗曚綅 RPM锛夈€?
in1_alarm		杈撳叆鐢靛帇娆犲帇鎶ヨ銆?in1_input		娴嬪緱鐨勮緭鍏ョ數鍘嬶紙鍗曚綅 mV锛夈€?in1_label		"vin"
in2_input		娴嬪緱鐨勮緭鍑虹數鍘嬶紙鍗曚綅 mV锛夈€?in2_label		"vout1"
in2_lcrit		涓寸晫鏈€灏忚緭鍑虹數鍘?in2_lcrit_alarm		杈撳嚭鐢靛帇涓寸晫杩囦綆鎶ヨ
in2_max			鏈€澶ц緭鍑虹數鍘?in2_max_alarm		杈撳嚭鐢靛帇杩囬珮鎶ヨ
in2_min			鏈€灏忚緭鍑虹數鍘?in2_min_alarm		杈撳嚭鐢靛帇杩囦綆鎶ヨ

power1_alarm		杈撳叆鏁呴殰鎴栨姤璀︺€?power1_input		娴嬪緱鐨勮緭鍏ュ姛鐜囷紙鍗曚綅 uW锛夈€?power1_label		"pin"
power1_max		杈撳叆鍔熺巼闄愬埗
power2_max_alarm	杈撳嚭鍔熺巼杩囬珮鎶ヨ
power2_max		杈撳嚭鍔熺巼闄愬埗
power2_input		娴嬪緱鐨勮緭鍑哄姛鐜囷紙鍗曚綅 uW锛夈€?power2_label		"pout"

temp[1-3]_input		娴嬪緱鐨勬俯搴?temp[1-2]_max		鏈€澶ф俯搴?temp[1-3]_max_alarm	娓╁害杩囬珮鎶ヨ

vendor			鍒堕€犲晢鍚嶇О
model			浜у搧鍨嬪彿
part_number		浜у搧閮ㄤ欢鍙?serial_number		浜у搧搴忓垪鍙?fw_version		鍥轰欢鐗堟湰
hw_version		纭欢鐗堟湰
mode			宸ヤ綔妯″紡銆傚彲璁剧疆涓?active 鎴?			standby锛屽綋璁剧疆涓?standby 鏃讹紝PSU 灏嗗湪
			standby 涓?redundancy 妯″紡涔嬮棿鑷姩鍒囨崲銆?======================= ======================================================
