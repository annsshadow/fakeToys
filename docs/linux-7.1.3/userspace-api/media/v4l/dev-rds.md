

######## RDS 鎺ュ彛


鏃犵嚎鐢垫暟鎹郴缁燂紙Radio Data System锛変互浜岃繘鍒舵牸寮忎紶杈撹ˉ鍏呬俊鎭紝渚嬪鐢靛彴鍚嶇О
鎴栦氦閫氫俊鎭紝浣嶄簬骞挎挱鑺傜洰鍚笉瑙佺殑闊抽鍓浇娉笂銆傛鎺ュ彛闈㈠悜鑳藉鎺ユ敹鍜?鎴?鍙戦€?RDS 淇℃伅鐨勮澶囥€?
鏇村淇℃伅璇峰弬瑙佹牳蹇?RDS 鏍囧噯 iec62106 鍜?RBDS 鏍囧噯 nrsc4銆?

   娉ㄦ剰 RBDS 鏍囧噯锛堝湪缇庡浗浣跨敤锛変笌 RDS 鏍囧噯鍑犱箮瀹屽叏鐩稿悓銆備换浣?RDS 瑙ｇ爜鍣?
   缂栫爜鍣ㄤ篃鍙互澶勭悊 RBDS銆傚彧鏈夋煇浜涘瓧娈电殑鍚箟鐣ユ湁涓嶅悓銆傛洿澶氫俊鎭鍙傝
   RBDS 鏍囧噯銆?
RBDS 鏍囧噯杩樿瀹氫簡瀵?MMBS锛圡odified Mobile Search锛夌殑鏀寔銆傝繖鏄竴绉嶄技涔庡凡琚?寮冪敤鐨勪笓鏈夋牸寮忋€俁DS 鎺ュ彛涓嶆敮鎸佹鏍煎紡銆傚鏋滈渶瑕佹敮鎸?MMBS锛堟垨閫氬父鎵€璋撶殑
鈥淓 blocks鈥濓級锛岃鑱旂郴 linux-media 閭欢鍒楄〃锛?`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__銆?
## 鏌ヨ鑳藉姏


鏀寔 RDS 鎹曡幏 API 鐨勮澶囦細鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct
`v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_RDS_CAPTURE` 鏍囧織銆?浠讳綍鏀寔 RDS 鐨勮皟璋愬櫒锛坱uner锛夐兘浼氬湪 struct `v4l2_tuner` 鐨?`capability`
瀛楁涓缃?`V4L2_TUNER_CAP_RDS` 鏍囧織銆傚鏋滈┍鍔ㄥ彧鏄紶閫?RDS 鍧楄€屼笉瑙ｉ噴鏁版嵁锛?鍒欏繀椤昏缃?`V4L2_TUNER_CAP_RDS_BLOCK_IO` 鏍囧織锛岃璇诲彇 RDS 鏁版嵁
<reading-rds-data>銆備负灏嗘潵浣跨敤锛屼篃瀹氫箟浜?`V4L2_TUNER_CAP_RDS_CONTROLS`
鏍囧織銆傜劧鑰岋紝鍏锋湁姝よ兘鍔涚殑鏃犵嚎鐢佃皟璋愬櫒椹卞姩灏氫笉瀛樺湪锛屽洜姝ゅ鏋滀綘鎵撶畻缂栧啓杩欐牱
涓€涓┍鍔紝浣犲簲璇ュ湪 linux-media 閭欢鍒楄〃涓婅璁猴細
`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__銆?
鏄惁瀛樺湪鐨?RDS 淇″彿鍙互閫氳繃鏌ョ湅 struct `v4l2_tuner` 鐨?`rxsubchans` 瀛楁
鏉ユ娴嬶細濡傛灉妫€娴嬪埌浜?RDS 鏁版嵁锛屽皢璁剧疆 `V4L2_TUNER_SUB_RDS`銆?
鏀寔 RDS 杈撳嚭 API 鐨勮澶囦細鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct
`v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_RDS_OUTPUT` 鏍囧織銆?浠讳綍鏀寔 RDS 鐨勮皟鍒跺櫒锛坢odulator锛夐兘浼氬湪 struct `v4l2_modulator` 鐨?`capability` 瀛楁涓缃?`V4L2_TUNER_CAP_RDS` 鏍囧織銆備负浜嗗惎鐢?RDS 浼犺緭锛屽繀椤?鍦?struct `v4l2_modulator` 鐨?`txsubchans` 瀛楁涓缃?`V4L2_TUNER_SUB_RDS`
浣嶃€傚鏋滈┍鍔ㄥ彧鏄紶閫?RDS 鍧楄€屼笉瑙ｉ噴鏁版嵁锛屽垯蹇呴』璁剧疆
`V4L2_TUNER_CAP_RDS_BLOCK_IO` 鏍囧織銆傚鏋滆皟璋愬櫒鑳藉澶勭悊 RDS 瀹炰綋锛堝鑺傜洰
璇嗗埆鐮佸拰骞挎挱鏂囨湰锛夛紝鍒欏簲璁剧疆 `V4L2_TUNER_CAP_RDS_CONTROLS` 鏍囧織锛岃鍐欏叆
RDS 鏁版嵁 <writing-rds-data> 鍜?FM 鍙戝皠鍣ㄦ帶鍒跺弬鑰?<fm-tx-controls>銆?

## 璇诲彇 RDS 鏁版嵁


鍙互浣跨敤 `read()` 鍑芥暟浠庢棤绾跨數璁惧璇诲彇 RDS 鏁版嵁銆傛暟鎹互涓変釜瀛楄妭涓轰竴缁?鎵撳寘銆?

## 鍐欏叆 RDS 鏁版嵁


鍙互浣跨敤 `write()` 鍑芥暟鍚戞棤绾跨數璁惧鍐欏叆 RDS 鏁版嵁銆傛暟鎹互涓変釜瀛楄妭涓轰竴缁?鎵撳寘锛屽涓嬫墍绀猴細

## RDS 鏁版嵁缁撴瀯



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 5

    - - __u8
      - `lsb`
      - RDS 鍧楃殑鏈€浣庢湁鏁堝瓧鑺傦紙Least Significant Byte锛夈€?    - - __u8
      - `msb`
      - RDS 鍧楃殑鏈€楂樻湁鏁堝瓧鑺傦紙Most Significant Byte锛夈€?    - - __u8
      - `block`
      - 鍧楁弿杩般€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 5

    - - Bits 0-2
      - 鎺ユ敹鏁版嵁鐨勫潡锛堝嵆鍋忕Щ閲忥紝offset锛夈€?    - - Bits 3-5
      - 宸插純鐢ㄣ€傚綋鍓嶄笌 bits 0-2 鐩稿悓銆備笉瑕佷娇鐢ㄨ繖浜涗綅銆?    - - Bit 6
      - 宸茬籂姝ｄ綅锛圕orrected bit锛夈€傛寚绀烘鏁版嵁鍧椾腑鏈変竴涓敊璇绾犳銆?    - - Bit 7
      - 閿欒浣嶏紙Error bit锛夈€傛寚绀哄湪鎺ユ敹姝ゅ潡鏈熼棿鍙戠敓浜嗕笉鍙籂姝ｇ殑閿欒銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 1 5

    - - V4L2_RDS_BLOCK_MSK
      -
      - 7
      - 鐢ㄤ簬鑾峰彇鍧?ID 鐨?bits 0-2 鎺╃爜銆?    - - V4L2_RDS_BLOCK_A
      -
      - 0
      - 鍧?A銆?    - - V4L2_RDS_BLOCK_B
      -
      - 1
      - 鍧?B銆?    - - V4L2_RDS_BLOCK_C
      -
      - 2
      - 鍧?C銆?    - - V4L2_RDS_BLOCK_D
      -
      - 3
      - 鍧?D銆?    - - V4L2_RDS_BLOCK_C_ALT
      -
      - 4
      - 鍧?C'銆?    - - V4L2_RDS_BLOCK_INVALID
      - read-only
      - 7
      - 涓€涓棤鏁堢殑鍧椼€?    - - V4L2_RDS_BLOCK_CORRECTED
      - read-only
      - 0x40
      - 妫€娴嬪埌涓€涓綅閿欒浣嗗凡琚籂姝ｃ€?    - - V4L2_RDS_BLOCK_ERROR
      - read-only
      - 0x80
      - 鍙戠敓浜嗕笉鍙籂姝ｇ殑閿欒銆?