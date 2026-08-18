
## 鍐呮牳椹卞姩 intel-m10-bmc-hwmon


鏀寔鐨勮澶囷細

 - Intel MAX 10 BMC锛堢敤浜?Intel PAC N3000锛?
   Prefix: 'n3000bmc-hwmon'

Author: Xu Yilun <yilun.xu@intel.com>


### 鎻忚堪


璇ラ┍鍔ㄤ负 Intel MAX 10 鏉跨鐞嗘帶鍒跺櫒锛圔MC锛夎姱鐗囨坊鍔犱簡娓╁害銆佺數鍘嬨€佺數娴佸拰鍔熻€楃殑璇诲彇鏀寔銆傝 BMC 鑺墖闆嗘垚鍦ㄤ竴浜?Intel 鍙紪绋嬪姞閫熷崱锛圥AC锛変腑銆傚畠杩炴帴鍒颁竴缁勪紶鎰熷櫒鑺墖锛屼互鐩戞帶鏉夸笂涓嶅悓缁勪欢鐨勪紶鎰熷櫒鏁版嵁銆侭MC 鍥轰欢璐熻矗鍦ㄥ叡浜瘎瀛樺櫒涓噰鏍峰拰璁板綍浼犳劅鍣ㄦ暟鎹€備富鏈洪┍鍔ㄤ粠杩欎簺鍏变韩瀵勫瓨鍣ㄤ腑璇诲彇浼犳劅鍣ㄦ暟鎹紝骞朵互 hwmon 鎺ュ彛鐨勫舰寮忔毚闇茬粰鐢ㄦ埛銆?
璇?BMC 鑺墖浣跨敤 Intel MAX 10 CPLD 瀹炵幇銆傚畠鍙互琚噸鏂扮紪绋嬩负鏌愪簺鍙樹綋锛屼互鏀寔涓嶅悓鐨?Intel PAC銆傝椹卞姩璁捐涓婅兘澶熷尯鍒嗚繖浜涘彉浣擄紝浣嗙洰鍓嶄粎鏀寔鐢ㄤ簬 Intel PAC N3000 鐨?BMC銆?

### Sysfs 灞炴€?

鏀寔浠ヤ笅灞炴€э細

- Intel MAX 10 BMC锛堢敤浜?Intel PAC N3000锛夛細

======================= =======================================================
tempX_input             缁勪欢娓╁害锛堢敱 tempX_label 鎸囧畾锛?tempX_max               缁勪欢娓╁害鏈€澶ц瀹氱偣
tempX_crit              缁勪欢娓╁害涓寸晫璁惧畾鐐?tempX_max_hyst          缁勪欢娓╁害鏈€澶у€肩殑杩熸粸
tempX_crit_hyst         缁勪欢娓╁害涓寸晫鍊肩殑杩熸粸
temp1_label             "鏉胯浇娓╁害"
temp2_label             "FPGA 鑺墖娓╁害"
temp3_label             "QSFP0 娓╁害"
temp4_label             "QSFP1 娓╁害"
temp5_label             "Retimer A 娓╁害"
temp6_label             "Retimer A SerDes 娓╁害"
temp7_label             "Retimer B 娓╁害"
temp8_label             "Retimer B SerDes 娓╁害"

inX_input               缁勪欢鐨勬祴閲忕數鍘嬶紙鐢?inX_label 鎸囧畾锛?in0_label               "QSFP0 渚涚數鐢靛帇"
in1_label               "QSFP1 渚涚數鐢靛帇"
in2_label               "FPGA 鏍稿績鐢靛帇"
in3_label               "12V 鑳屾澘鐢靛帇"
in4_label               "1.2V 鐢靛帇"
in5_label               "12V AUX 鐢靛帇"
in6_label               "1.8V 鐢靛帇"
in7_label               "3.3V 鐢靛帇"

currX_input             缁勪欢鐨勬祴閲忕數娴侊紙鐢?currX_label 鎸囧畾锛?curr1_label             "FPGA 鏍稿績鐢垫祦"
curr2_label             "12V 鑳屾澘鐢垫祦"
curr3_label             "12V AUX 鐢垫祦"

powerX_input            缁勪欢鐨勬祴閲忓姛鑰楋紙鐢?powerX_label 鎸囧畾锛?power1_label            "鏉胯浇鍔熻€?

======================= =======================================================

鎵€鏈夊睘鎬у潎涓哄彧璇汇€?