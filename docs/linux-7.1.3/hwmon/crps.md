
## 鍐呮牳椹卞姩 crps


Supported chips:

  - Intel CRPS185

    Prefix: 'crps185'

    Addresses scanned: -

    Datasheet: Only available under NDA.

Authors:
    Ninad Palsule <ninad@linux.ibm.com>


### 鎻忚堪


鏈┍鍔ㄥ疄鐜板甯︽湁 PMBus 鏀寔鐨?Intel 閫氱敤鍐椾綑鐢垫簮锛圕ommon Redundant Power supply锛夌殑鏀寔銆?
璇ラ┍鍔ㄦ槸鏍稿績 PMBus 椹卞姩鐨勫鎴风椹卞姩銆傛湁鍏?PMBus 瀹㈡埛绔┍鍔ㄧ殑璇︾粏淇℃伅锛岃鍙傞槄 Documentation/hwmon/pmbus.rst銆?

### 浣跨敤娉ㄦ剰浜嬮」


鏈┍鍔ㄤ笉浼氳嚜鍔ㄦ娴嬭澶囥€備綘闇€瑕佹樉寮忓湴瀹炰緥鍖栬澶囥€傝鎯呰鍙傞槄 Documentation/i2c/instantiating-devices.rst銆?

### Sysfs 鏉＄洰


======================= ======================================================
curr1_label		"iin"
curr1_input		娴嬪緱鐨勮緭鍏ョ數娴?curr1_max		鏈€澶ц緭鍏ョ數娴?curr1_max_alarm		杈撳叆鏈€澶х數娴侀珮鎶ヨ
curr1_crit		涓寸晫楂樿緭鍏ョ數娴?curr1_crit_alarm	杈撳叆涓寸晫鐢垫祦楂樻姤璀?curr1_rated_max		鏈€澶ч瀹氳緭鍏ョ數娴?
curr2_label		"iout1"
curr2_input		娴嬪緱鐨勮緭鍑虹數娴?curr2_max		鏈€澶ц緭鍑虹數娴?curr2_max_alarm		杈撳嚭鏈€澶х數娴侀珮鎶ヨ
curr2_crit		涓寸晫楂樿緭鍑虹數娴?curr2_crit_alarm	杈撳嚭涓寸晫鐢垫祦楂樻姤璀?curr2_rated_max		鏈€澶ч瀹氳緭鍑虹數娴?
in1_label		"vin"
in1_input		娴嬪緱鐨勮緭鍏ョ數鍘?in1_crit		涓寸晫杈撳叆杩囧帇
in1_crit_alarm		涓寸晫杈撳叆杩囧帇鎶ヨ
in1_max			鏈€澶ц緭鍏ヨ繃鍘?in1_max_alarm		鏈€澶ц緭鍏ヨ繃鍘嬫姤璀?in1_rated_min		鏈€灏忛瀹氳緭鍏ョ數鍘?in1_rated_max		鏈€澶ч瀹氳緭鍏ョ數鍘?
in2_label		"vout1"
in2_input		娴嬪緱鐨勮緭鍏ョ數鍘?in2_crit		涓寸晫杈撳叆杩囧帇
in2_crit_alarm		涓寸晫杈撳叆杩囧帇鎶ヨ
in2_lcrit		涓寸晫杈撳叆娆犲帇鏁呴殰
in2_lcrit_alarm		涓寸晫杈撳叆娆犲帇鏁呴殰鎶ヨ
in2_max			鏈€澶ц緭鍏ヨ繃鍘?in2_max_alarm		鏈€澶ц緭鍏ヨ繃鍘嬫姤璀?in2_min			鏈€灏忚緭鍏ユ瑺鍘嬭鍛?in2_min_alarm		鏈€灏忚緭鍏ユ瑺鍘嬭鍛婃姤璀?in2_rated_min		鏈€灏忛瀹氳緭鍏ョ數鍘?in2_rated_max		鏈€澶ч瀹氳緭鍏ョ數鍘?
power1_label		"pin"
power1_input		娴嬪緱鐨勮緭鍏ュ姛鐜?power1_alarm		杈撳叆鍔熺巼楂樻姤璀?power1_max  		鏈€澶ц緭鍏ュ姛鐜?power1_rated_max		鏈€澶ч瀹氳緭鍏ュ姛鐜?
temp[1-2]_input		娴嬪緱鐨勬俯搴?temp[1-2]_crit 		涓寸晫娓╁害
temp[1-2]_crit_alarm	涓寸晫娓╁害鎶ヨ
temp[1-2]_max		鏈€楂樻俯搴?temp[1-2]_max_alarm	鏈€楂樻俯搴︽姤璀?temp[1-2]_rated_max	鏈€澶ч瀹氭俯搴?
fan1_alarm		椋庢墖 1 璀﹀憡銆?fan1_fault		椋庢墖 1 鏁呴殰銆?fan1_input		椋庢墖 1 杞€燂紙RPM锛夈€?fan1_target		椋庢墖 1 鐩爣銆?======================= ======================================================
