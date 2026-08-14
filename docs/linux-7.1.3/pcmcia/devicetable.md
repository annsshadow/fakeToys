## 璁惧琛?

PCMCIA 璁惧涓庨┍鍔ㄧ殑鍖归厤浣跨敤浠ヤ笅涓€涓垨澶氫釜鍑嗗垯瀹屾垚锛?
- 鍘傚晢 ID
- 鍗?ID
- 浜у搧 ID 瀛楃涓?_鍙奯 杩欎簺瀛楃涓茬殑鍝堝笇
- 鍔熻兘 ID
- 璁惧鍔熻兘锛堝疄闄呬笌浼級

浣犲簲浣跨敤 include/pcmcia/device_id.h 涓殑杈呭姪瀹忔潵鐢熸垚灏嗚澶囧尮閰嶅埌椹卞姩鐨?struct pcmcia_device_id[] 鏉＄洰銆?
鑻ユ兂鍖归厤浜у搧 ID 瀛楃涓诧紝浣犺繕闇€瑕佸皢瀛楃涓茬殑 crc32 鍝堝笇浼犵粰瀹忥紝渚嬪鑻ユ兂
鍖归厤浜у搧 ID 瀛楃涓?1锛屼綘闇€瑕佷娇鐢?
PCMCIA_DEVICE_PROD_ID1("some_string", 0x(hash_of_some_string)),

濡傛灉鍝堝笇涓嶆纭紝鍐呮牳浼氬湪妯″潡鍒濆鍖栨椂浜?"dmesg" 涓€氱煡浣狅紝骞跺憡鐭ヤ綘
姝ｇ‘鐨勫搱甯屻€?
浣犲彲浠ラ€氳繃 cat 璇?PCMCIA 璁惧 sysfs 鐩綍涓嬬殑 "modalias" 鏂囦欢鏉ョ‘瀹氫骇鍝?ID 瀛楃涓茬殑鍝堝笇銆傚畠浼氱敓鎴愬涓嬪舰寮忕殑瀛楃涓诧細
pcmcia:m0149cC1ABf06pfn00fn00pa725B842DpbF1EFEE84pc0877B627pd00000000

"pa" 涔嬪悗鐨勫崄鍏繘鍒跺€兼槸浜у搧 ID 瀛楃涓?1 鐨勫搱甯岋紝"pb" 涔嬪悗鐨勬槸瀛楃涓?2 鐨?鍝堝笇锛屼緷姝ょ被鎺ㄣ€?
鎴栬€咃紝浣犲彲浠ヤ娇鐢?crc32hash锛堣 tools/pcmcia/crc32hash.c锛夋潵纭畾 crc32
鍝堝笇銆傚彧闇€灏嗕綘鎯宠绠楃殑瀛楃涓蹭綔涓哄弬鏁颁紶缁欒绋嬪簭锛屼緥濡傦細
$ tools/pcmcia/crc32hash "Dual Speed"
