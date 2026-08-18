## 鍐呮牳椹卞姩 nouveau


Supported chips:

- NV43+

Authors: Martin Peres (mupuf) <martin.peres@free.fr>

### 鎻忚堪


鏈┍鍔ㄥ厑璁歌鍙?GPU 鏍稿績娓╁害銆侀┍鍔?GPU 椋庢墖骞惰缃俯搴︽姤璀︺€?
鐩墠锛岀敱浜庡唴鏍镐腑缂哄皯璁块棶 HWMON 椹卞姩鐨?API锛孨ouveau 鏃犳硶璁块棶瀹冨彲鑳藉彂鐜扮殑浠讳綍 i2c 澶栭儴鐩戞帶鑺墖銆傚鏋滀綘鎷ユ湁姝ょ被鑺墖锛岄偅涔堥€氳繃 Nouveau 鐨?HWMON 鎺ュ彛杩涜娓╁害鍜?鎴栭鎵囩鐞嗗緢鍙兘鏃犳硶宸ヤ綔銆傛湰鏂囨。鍙兘鍥犳鏃犳硶瀹屽叏瑕嗙洊浣犵殑鎯呭喌銆?
### 娓╁害绠＄悊


娓╁害浠ヤ竴涓彧璇荤殑 HWMON 灞炴€?temp1_input 鏆撮湶銆?
涓轰繚鎶?GPU 涓嶈繃鐑紝Nouveau 鏀寔 4 涓彲閰嶇疆鐨勬俯搴﹂槇鍊硷細

 - Fan_boost锛堥鎵囧姞閫燂級锛?	杈惧埌璇ユ俯搴︽椂椋庢墖杞€熻涓?100%锛? - Downclock锛堥檷棰戯級锛?	GPU 灏嗚闄嶉浠ュ噺灏戝姛鑰楋紱
 - Critical锛堜复鐣岋級锛?	GPU 琚殏鍋滀互杩涗竴姝ラ檷浣庡姛鑰楋紱
 - Shutdown锛堝叧鏈猴級锛?	鍏抽棴璁＄畻鏈轰互淇濇姢浣犵殑 GPU銆?
WARNING锛堣鍛婏級锛?	鏍规嵁鑺墖缁勪笉鍚岋紝Nouveau 鍙兘涓嶄細浣跨敤鍏朵腑鏌愪簺闃堝€笺€?
杩欎簺闃堝€肩殑榛樿鍊兼潵鑷?GPU 鐨?vbios銆傝繖浜涢槇鍊煎彲閫氳繃浠ヤ笅 HWMON 灞炴€ч厤缃細

 - Fan_boost锛歵emp1_auto_point1_temp 涓?temp1_auto_point1_temp_hyst锛? - Downclock锛歵emp1_max 涓?temp1_max_hyst锛? - Critical锛歵emp1_crit 涓?temp1_crit_hyst锛? - Shutdown锛歵emp1_emergency 涓?temp1_emergency_hyst銆?
NOTE锛堟敞鎰忥級锛氳璁颁綇锛岃繖浜涘€间互姣憚姘忓害锛坢illi degrees Celsius锛夊瓨鍌ㄣ€傚埆蹇樹簡鎹㈢畻锛?
### 椋庢墖绠＄悊


骞堕潪鎵€鏈夋樉鍗￠兘鏈夊彲椹卞姩鐨勯鎵囥€傚鏋滄湁锛屽垯浠ヤ笅 HWMON 灞炴€у簲褰撳彲鐢細

 - pwm1_enable锛?	褰撳墠椋庢墖绠＄悊妯″紡锛圢ONE銆丮ANUAL 鎴?AUTO锛夛紱
 - pwm1锛?	褰撳墠 PWM 鍊硷紙鍔熺巼鐧惧垎姣旓級锛? - pwm1_min锛?	鍏佽鐨勬渶灏?PWM 杞€燂紱
 - pwm1_max锛?	鍏佽鐨勬渶澶?PWM 杞€燂紙鍛戒腑 Fan_boost 鏃朵細琚粫杩囷級锛?
浣犲彲鑳借繕鎷ユ湁浠ヤ笅灞炴€э細

 - fan1_input锛?	椋庢墖杞€燂紙RPM锛夈€?
浣犵殑椋庢墖鍙互鍦ㄤ笉鍚屾ā寮忎笅椹卞姩锛?
 - 0锛氶鎵囦繚鎸佷笉鍔紱
 - 1锛氶鎵囧彲鎵嬪姩椹卞姩锛堜娇鐢?pwm1 鏀瑰彉杞€燂級锛? - 2锛氶鎵囨牴鎹俯搴﹁嚜鍔ㄩ┍鍔ㄣ€?
NOTE锛堟敞鎰忥級锛?  鑻ユ兂鎵嬪姩椹卞姩椋庢墖杞€燂紝璇峰姟蹇呬娇鐢ㄦ墜鍔ㄦā寮忋€?
NOTE2锛堟敞鎰?锛夛細
  褰撳湪 vbios 瀹氫箟鐨?[PWM_min, PWM_max] 鑼冨洿涔嬪浠ユ墜鍔ㄦā寮忚繍琛屾椂锛屾牴鎹‖浠朵笉鍚岋紝鎶ュ憡鐨勯鎵囪浆閫燂紙RPM锛夊彲鑳戒笉鍑嗙‘銆?
### 缂洪櫡鎶ュ憡


Nouveau 涓婄殑鐑鐞嗗睘浜庢柊鍔熻兘锛屽彲鑳藉苟闈炲湪鎵€鏈夋樉鍗′笂閮借兘宸ヤ綔銆傚鏈夌枒闂紝璇峰湪 IRC锛?nouveau锛孫FTC锛変笂鑱旂郴 mupuf銆?
缂洪櫡鎶ュ憡搴旀彁浜ゅ埌 Freedesktop 鐨?bug 璺熻釜鍣ㄣ€傝璁块棶
https://nouveau.freedesktop.org/wiki/Bugs
