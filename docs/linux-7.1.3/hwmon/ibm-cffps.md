## ibm-cffps 鍐呮牳椹卞姩


鏀寔鐨勮姱鐗囷細

  - IBM Common Form Factor power supply

Author: Eddie James <eajames@us.ibm.com>

### 鎻忚堪


璇ラ┍鍔ㄦ敮鎸?IBM 閫氱敤澶栧舰瑙勬牸锛圕FF锛夌數婧愩€傝椹卞姩鏄牳蹇?PMBus 椹卞姩鐨勫鎴风銆?
### 浣跨敤璇存槑


璇ラ┍鍔ㄤ笉浼氳嚜鍔ㄦ娴嬭澶囥€備綘闇€瑕佹樉寮忓疄渚嬪寲璁惧銆傝缁嗕俊鎭鍙傞槄
Documentation/i2c/instantiating-devices.rst銆?
### Sysfs 鏉＄洰


鏀寔浠ヤ笅灞炴€э細

======================= ======================================================
curr1_alarm		杈撳嚭鐢垫祦杩囨祦鍛婅銆?curr1_input		娴嬮噺鐨勮緭鍑虹數娴侊紙鍗曚綅 mA锛夈€?curr1_label		"iout1"

fan1_alarm		椋庢墖 1 璀﹀憡銆?fan1_fault		椋庢墖 1 鏁呴殰銆?fan1_input		椋庢墖 1 杞€燂紙RPM锛夈€?fan2_alarm		椋庢墖 2 璀﹀憡銆?fan2_fault		椋庢墖 2 鏁呴殰銆?fan2_input		椋庢墖 2 杞€燂紙RPM锛夈€?
in1_alarm		杈撳叆鐢靛帇娆犲帇鍛婅銆?in1_input		娴嬮噺鐨勮緭鍏ョ數鍘嬶紙鍗曚綅 mV锛夈€?in1_label		"vin"
in2_alarm		杈撳嚭鐢靛帇杩囧帇鍛婅銆?in2_input		娴嬮噺鐨勮緭鍑虹數鍘嬶紙鍗曚綅 mV锛夈€?in2_label		"vout1"

power1_alarm		杈撳叆鏁呴殰鎴栧憡璀︺€?power1_input		娴嬮噺鐨勮緭鍏ュ姛鐜囷紙鍗曚綅 uW锛夈€?power1_label		"pin"

temp1_alarm		PSU 杩涢鍙ｇ幆澧冩俯搴﹁繃娓╁憡璀︺€?temp1_input		娴嬮噺鐨?PSU 杩涢鍙ｇ幆澧冩俯搴︼紙鍗曚綅姣憚姘忓害锛夈€?temp2_alarm		娆＄骇鏁存祦鍣ㄦ俯搴﹁繃娓╁憡璀︺€?temp2_input		娴嬮噺鐨勬绾ф暣娴佸櫒娓╁害锛堝崟浣嶆鎽勬皬搴︼級銆?temp3_alarm		ORing FET 娓╁害杩囨俯鍛婅銆?temp3_input		娴嬮噺鐨?ORing FET 娓╁害锛堝崟浣嶆鎽勬皬搴︼級銆?======================= ======================================================
