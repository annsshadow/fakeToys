## 鍐呮牳椹卞姩 menf21bmc_hwmon


鏀寔鐨勮姱鐗囷細

 - MEN 14F021P00

	 鍓嶇紑锛?menf21bmc_hwmon'

	 鎵弿鐨勫湴鍧€锛?

浣滆€咃細Andreas Werner <andreas.werner@men.de>

### 鎻忚堪


menf21bmc 鏄竴涓澘绠＄悊鎺у埗鍣紙BMC锛夛紝瀹冩彁渚?I2C 鎺ュ彛渚涗富鏈鸿闂?BMC 涓?瀹炵幇鐨勫姛鑳姐€?
璇ラ┍鍔ㄦ彁渚涘鏉夸富鐢靛帇鐩戞帶鍔熻兘鐨勮闂€?鐢靛帇浼犳劅鍣ㄨ繛鎺ュ埌 BMC 鐨?ADC 杈撳叆锛孊MC 鏄竴涓?PIC16F917 寰帶鍒跺櫒銆?
### 浣跨敤璇存槑


璇ラ┍鍔ㄦ槸鍚嶄负 "menf21bmc" 鐨?MFD 椹卞姩鐨勪竴閮ㄥ垎锛屼笉浼氳嚜鍔ㄦ帰娴嬭澶囥€?浣犲繀椤绘樉寮忓湴瀹炰緥鍖?MFD 椹卞姩銆?璇﹁ Documentation/i2c/instantiating-devices.rst銆?
### Sysfs 鏉＄洰


鏀寔浠ヤ笅灞炴€с€傛墍鏈夊睘鎬у潎涓哄彧璇汇€?闄愬€肩敱椹卞姩涓€娆℃€ц鍙栥€?
=============== ==========================
in0_input	+3.3V 杈撳叆鐢靛帇
in1_input	+5.0V 杈撳叆鐢靛帇
in2_input	+12.0V 杈撳叆鐢靛帇
in3_input	+5V 寰呮満杈撳叆鐢靛帇
in4_input	VBAT锛堟澘杞界數姹狅級

in[0-4]_min	鏈€灏忕數鍘嬮檺鍊?in[0-4]_max	鏈€澶х數鍘嬮檺鍊?
in0_label	"MON_3_3V"
in1_label	"MON_5V"
in2_label	"MON_12V"
in3_label	"5V_STANDBY"
in4_label	"VBAT"
=============== ==========================
