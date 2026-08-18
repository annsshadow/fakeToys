
## Alienware WMI 椹卞姩


Kurt Borja <kuurtb@gmail.com>

杩欐槸涓€涓敤浜?"WMAX" WMI 璁惧鐨勯┍鍔紝璇ヨ澶囧瓨鍦ㄤ簬澶у鏁?Dell 娓告垙鏈腑锛屾帶鍒跺悇绉嶇壒娈婂姛鑳姐€?
鍦?M 绯诲垪绗旇鏈帹鍑轰箣鍓嶏紙绾?2018 骞达級锛?WMAX" 璁惧鎺у埗鍩烘湰鐨?RGB 鐏厜銆佹繁搴︾潯鐪犳ā寮忋€?HDMI 妯″紡涓庢斁澶у櫒鐘舵€併€?
鍚庢潵锛岃璁惧琚畬鍏ㄩ噸鏂板畾浣嶇敤閫斻€傜幇鍦ㄥ畠涓昏澶勭悊鏁ｇ儹閰嶇疆鏂囦欢銆佷紶鎰熷櫒鐩戣涓庤秴棰戙€傝鎺ュ彛
鍚嶄负 "AWCC"锛屽凡鐭ヨ AWCC OEM 搴旂敤绋嬪簭鐢ㄦ潵鎺у埗杩欎簺鐗规€с€?
alienware-wmi 椹卞姩鎺у埗杩欎袱涓帴鍙ｃ€?
## AWCC 鎺ュ彛


WMI 璁惧鏂囨。锛欴ocumentation/wmi/devices/alienware-wmi.rst

### 鏀寔鐨勮澶?

- Alienware M 绯诲垪绗旇鏈?- Alienware X 绯诲垪绗旇鏈?- Alienware Aurora 鍙板紡鏈?- Dell G 绯诲垪绗旇鏈?
濡傛灉浣犺涓轰綘鐨勮澶囨敮鎸?AWCC 鎺ュ彛锛屼絾娌℃湁鏈枃妗ｄ腑鎻忚堪鐨勪换浣曠壒鎬э紝璇峰皾璇曚互涓?alienware-wmi
妯″潡鍙傛暟锛?
- `force_platform_profile=1`锛氬己鍒舵帰娴嬪钩鍙伴厤缃枃浠舵敮鎸?- `force_hwmon=1`锛氬己鍒舵帰娴?HWMON 鏀寔

濡傛灉浣跨敤杩欎簺鍙傛暟妯″潡鍔犺浇鎴愬姛锛岃鑰冭檻鎻愪氦涓€涓ˉ涓侊紝灏嗕綘鐨勫瀷鍙峰姞鍏ヤ綅浜?`drivers/platform/x86/dell/alienware-wmi-wmax.c` 鐨?`awcc_dmi_table`锛屾垨鑱旂郴缁存姢鑰?鑾峰彇杩涗竴姝ユ寚瀵笺€?
### 鐘舵€?

褰撳墠鏀寔浠ヤ笅鐗规€э細

- 骞冲彴閰嶇疆鏂囦欢 <platform-profile>锛?
  - 鏁ｇ儹閰嶇疆鏂囦欢鎺у埗

  - G-Mode 鍒囨崲

- HWMON <hwmon>锛?
  - 浼犳劅鍣ㄧ洃瑙?
  - 鎵嬪姩椋庢墖鎺у埗


### 骞冲彴閰嶇疆鏂囦欢


AWCC 鎺ュ彛鏆撮湶鍚勭鍥轰欢瀹氫箟鐨勬暎鐑厤缃枃浠躲€傝繖浜涢€氳繃骞冲彴閰嶇疆鏂囦欢绫绘帴鍙ｆ毚闇茬粰鐢ㄦ埛绌洪棿銆傛洿澶?淇℃伅璇峰弬鑰?sysfs-class-platform-profile
<abi_file_testing_sysfs_class_platform_profile>銆?
璇ラ┍鍔ㄥ鍑虹殑骞冲彴閰嶇疆鏂囦欢绫昏澶囧悕绉颁负 "alienware-wmi"锛屽叾璺緞鍙€氳繃浠ヤ笅鏂瑰紡鎵惧埌锛?
```

 grep -l "alienware-wmi" /sys/class/platform-profile/platform-profile-*/name | sed 's|/[^/]*$||'

```
濡傛灉璁惧鏀寔 G-Mode锛岄€夋嫨 `performance` 閰嶇疆鏂囦欢鏃朵篃浼氬垏鎹㈠畠銆?
   浣犲彲浠ヨ缃?`force_gmode` 妯″潡鍙傛暟鏉ュ缁堝皾璇曞垏鎹㈡鐗规€э紝鑰屼笉妫€鏌ヤ綘鐨勫瀷鍙锋槸鍚︽敮鎸佸畠銆?

### HWMON


AWCC 鎺ュ彛杩樻敮鎸佷紶鎰熷櫒鐩戣涓庢墜鍔ㄩ鎵囨帶鍒躲€傝繖涓や釜鐗规€ч兘閫氳繃 HWMON 鎺ュ彛鏆撮湶缁欑敤鎴风┖闂淬€?
璇ラ┍鍔ㄥ鍑虹殑 hwmon 绫昏澶囧悕绉颁负 "alienware_wmi"锛屽叾璺緞鍙€氳繃浠ヤ笅鏂瑰紡鎵惧埌锛?
```

 grep -l "alienware_wmi" /sys/class/hwmon/hwmon*/name | sed 's|/[^/]*$||'

```
浼犳劅鍣ㄧ洃瑙嗛€氳繃鏍囧噯 HWMON 鎺ュ彛瀹屾垚銆傛洿澶氫俊鎭鍙傝€?sysfs-class-hwmon
<abi_file_testing_sysfs_class_hwmon>銆?
鍙︿竴鏂归潰锛屾墜鍔ㄩ鎵囨帶鍒跺苟闈炵敱 AWCC 鎺ュ彛鐩存帴鏆撮湶銆傜浉鍙嶏紝瀹冨厑璁告垜浠帶鍒堕鎵?`boost` 鍊笺€?璇?`boost` 鍊煎椋庢墖 pwm 鐨勮繎浼艰涓哄涓嬶細

```

 pwm = pwm_base + (fan_boost / 255) * (pwm_max - pwm_base)

```
鐢变簬涓婅堪琛屼负锛岄鎵?`boost` 鎺у埗閫氳繃浠ヤ笅鑷畾涔夌殑 hwmon sysfs 灞炴€ф毚闇茬粰鐢ㄦ埛绌洪棿锛?
=============================== ======= =======================================
鍚嶇О			鏉冮檺	鎻忚堪
=============================== ======= =======================================
fan[1-4]_boost			RW	椋庢墖 boost 鍊笺€?
					浠嬩簬 0 涓?255 涔嬮棿鐨勬暣鏁板€?=============================== ======= =======================================

   鍦ㄦ煇浜涜澶囦笂锛屾墜鍔ㄩ鎵囨帶鍒跺彧鏈夊湪閫夋嫨浜?`custom` 骞冲彴閰嶇疆鏂囦欢鏃舵墠鍙潬宸ヤ綔銆?