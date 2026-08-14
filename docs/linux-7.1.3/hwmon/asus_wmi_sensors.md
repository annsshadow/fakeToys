
## 鍐呮牳椹卞姩 asus_wmi_sensors


鏀寔鐨勬澘鍗★細
 - PRIME X399-A,
 - PRIME X470-PRO,
 - ROG CROSSHAIR VI EXTREME,
 - ROG CROSSHAIR VI HERO,
 - ROG CROSSHAIR VI HERO (WI-FI AC),
 - ROG CROSSHAIR VII HERO,
 - ROG CROSSHAIR VII HERO (WI-FI),
 - ROG STRIX B450-E GAMING,
 - ROG STRIX B450-F GAMING,
 - ROG STRIX B450-I GAMING,
 - ROG STRIX X399-E GAMING,
 - ROG STRIX X470-F GAMING,
 - ROG STRIX X470-I GAMING,
 - ROG ZENITH EXTREME,
 - ROG ZENITH EXTREME ALPHA.

Authors:
    - Ed Brindley <kernel@maidavale.org>

### 鎻忚堪锛?
鍗庣锛圓SUS锛変富鏉块€氳繃 WMI 鎺ュ彛鍙戝竷纭欢鐩戣淇℃伅銆?
ASUS WMI 鎺ュ彛鎻愪緵涓€绉嶈幏鍙栦紶鎰熷櫒鍒楄〃鍙婂叾鍊肩殑鏂规硶锛屾湰椹卞姩鍒╃敤瀹冨皢杩欎簺浼犳劅鍣ㄨ鏁板彂甯冨埌 HWMON 绯荤粺銆?
璇ラ┍鍔ㄨ兘澶熻瘑鍒苟璇诲彇浠ヤ笅浼犳劅鍣細
 - CPU 鏍稿績鐢靛帇锛圕PU Core Voltage锛?
 - CPU SOC 鐢靛帇锛圕PU SOC Voltage锛?
 - DRAM 鐢靛帇锛圖RAM Voltage锛?
 - VDDP 鐢靛帇锛圴DDP Voltage锛?
 - 1.8V PLL 鐢靛帇锛?.8V PLL Voltage锛?
 - +12V 鐢靛帇锛?12V Voltage锛?
 - +5V 鐢靛帇锛?5V Voltage锛?
 - 3VSB 鐢靛帇锛?VSB Voltage锛?
 - VBAT 鐢靛帇锛圴BAT Voltage锛?
 - AVCC3 鐢靛帇锛圓VCC3 Voltage锛?
 - SB 1.05V 鐢靛帇锛圫B 1.05V Voltage锛?
 - CPU 鏍稿績鐢靛帇锛圕PU Core Voltage锛?
 - CPU SOC 鐢靛帇锛圕PU SOC Voltage锛?
 - DRAM 鐢靛帇锛圖RAM Voltage锛?
 - CPU 椋庢墖杞€燂紙CPU Fan RPM锛?
 - 鏈虹椋庢墖 1 杞€燂紙Chassis Fan 1 RPM锛?
 - 鏈虹椋庢墖 2 杞€燂紙Chassis Fan 2 RPM锛?
 - 鏈虹椋庢墖 3 杞€燂紙Chassis Fan 3 RPM锛?
 - HAMP 椋庢墖杞€燂紙HAMP Fan RPM锛?
 - 姘存车杞€燂紙Water Pump RPM锛?
 - CPU OPT 杞€燂紙CPU OPT RPM锛?
 - 姘存祦閲忚浆閫燂紙Water Flow RPM锛?
 - AIO 姘存车杞€燂紙AIO Pump RPM锛?
 - CPU 娓╁害锛圕PU Temperature锛?
 - CPU 鎻掓Ы娓╁害锛圕PU Socket Temperature锛?
 - 涓绘澘娓╁害锛圡otherboard Temperature锛?
 - 鑺墖缁勬俯搴︼紙Chipset Temperature锛?
 - Tsensor 1 娓╁害锛圱sensor 1 Temperature锛?
 - CPU VRM 娓╁害锛圕PU VRM Temperature锛?
 - 杩涙按娓╁害锛圵ater In锛?
 - 鍑烘按娓╁害锛圵ater Out锛?
 - CPU VRM 杈撳嚭鐢垫祦锛圕PU VRM Output Current锛?

宸茬煡闂锛? - 鍗庣閮ㄥ垎 BIOS 涓殑 WMI 瀹炵幇瀛樺湪 bug銆傝繖鍙兘瀵艰嚧椋庢墖鍋滄銆侀鎵囧崱鍦ㄦ渶楂樿浆閫燂紝鎴栨俯搴﹁鏁板崱浣忋€傝繖涓嶆槸椹卞姩鐨勯棶棰橈紝鑰屾槸 BIOS 鐨勯棶棰樸€侾rime X470 Pro 鍦ㄨ繖鏂归潰浼间箮灏ゅ叾绯熺硶銆俉MI 鎺ュ彛琚疆璇㈠緱瓒婇绻侊紝鍙戠敓杩欑鎯呭喌鐨勫彲鑳芥€у氨瓒婂ぇ銆傚湪浣犲璁＄畻鏈鸿繘琛岄暱鏃堕棿鍘嬪姏娴嬭瘯骞堕绻佽疆璇紶鎰熷櫒涔嬪墠锛屼笉瑕佽浣犵殑璁＄畻鏈烘棤浜虹湅绠°€傚崌绾у埌鏂规硶鐗堟湰澶т簬绛変簬 2 鐨勬柊 BIOS 鐗堟湰搴斿綋鑳界籂姝ｈ闂銆? - 灏戞暟涓绘澘鎶ュ憡鐨?12v 鐢靛帇绾︿负 10v銆?