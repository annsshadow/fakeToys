## 鍐呮牳椹卞姩 sht21


鏀寔鐨勮澶囷細

  - Sensirion SHT20

    Prefix: 'sht20'

    Addresses scanned: none

    Datasheet: 鍏紑鎻愪緵锛屼綅浜?Sensirion 缃戠珯

    https://www.sensirion.com/file/datasheet_sht20

  - Sensirion SHT21

    Prefix: 'sht21'

    Addresses scanned: none

    Datasheet: 鍏紑鎻愪緵锛屼綅浜?Sensirion 缃戠珯

    https://www.sensirion.com/file/datasheet_sht21

  - Sensirion SHT25

    Prefix: 'sht25'

    Addresses scanned: none

    Datasheet: 鍏紑鎻愪緵锛屼綅浜?Sensirion 缃戠珯

    https://www.sensirion.com/file/datasheet_sht25

Author:

  Urs Fleisch <urs.fleisch@sensirion.com>

### 鎻忚堪


SHT21 鍜?SHT25 鏄噰鐢?DFN 灏佽鐨勬箍搴﹀拰娓╁害浼犳劅鍣紝鍏跺昂瀵镐粎涓?3 x 3 mm锛岄珮搴︿负 1.1 mm銆傝繖涓や釜鍣ㄤ欢鐨勫尯鍒湪浜?SHT25 鐨勭簿搴︽洿楂橈紙鐩稿婀垮害 1.8%锛屾俯搴?0.2 鎽勬皬搴︼級锛岃€?SHT21 涓猴紙鐩稿婀垮害 2.0%锛屾俯搴?0.3 鎽勬皬搴︼級銆?
杩欎簺鍣ㄤ欢閫氳繃 I2C 鍗忚閫氫俊銆傛墍鏈変紶鎰熷櫒閮借缃负鐩稿悓鐨?I2C 鍦板潃 0x40锛屽洜姝ゅ彲浠ュ湪鏉跨骇璁剧疆浠ｇ爜涓娇鐢?I2C_BOARD_INFO("sht21", 0x40) 杩欐牱涓€涓潯鐩€?
### sysfs 鎺ュ彛


=================== ============================================================
temp1_input         娓╁害杈撳叆
humidity1_input     婀垮害杈撳叆
eic                 鐢靛瓙璇嗗埆鐮侊紙Electronic Identification Code锛?=================== ============================================================

### 娉ㄦ剰浜嬮」


璇ラ┍鍔ㄤ娇鐢ㄩ粯璁ょ殑鍒嗚鲸鐜囪缃細婀垮害 12 浣嶏紝娓╁害 14 浣嶏紝杩欏鑷村吀鍨嬬殑娴嬮噺鏃堕棿涓烘箍搴?22 ms銆佹俯搴?66 ms銆備负浣胯嚜鍙戠儹浣庝簬 0.1 鎽勬皬搴︼紝鍣ㄤ欢澶勪簬娲诲姩鐘舵€佺殑鏃堕棿涓嶅簲瓒呰繃 10%锛屼緥濡傚湪缁欏畾鍒嗚鲸鐜囦笅姣忕鏈€澶氫袱娆℃祴閲忋€?
涓嶅悓鐨勫垎杈ㄧ巼銆佺墖涓婂姞鐑櫒浠ュ強浣跨敤 CRC 鏍￠獙鍜岀洰鍓嶅皻涓嶆敮鎸併€?