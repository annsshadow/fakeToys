
## Xilinx SD-FEC Driver


## Overview


璇ラ┍鍔ㄦ敮鎸?Zynq |Ultrascale+ (TM)| RFSoC 鐨?SD-FEC 闆嗘垚妯″潡銆?

   .. with trademark sign

鏈夊叧 SD-FEC 鏍稿績鍔熻兘鐨勫畬鏁存弿杩帮紝璇峰弬闃?`SD-FEC Product Guide (PG256) <https://www.xilinx.com/cgi-bin/docs/ipdoc?c=sd_fec;v=latest;d=pg256-sdfec-integrated-block.pdf>`_

璇ラ┍鍔ㄦ敮鎸佷互涓嬬壒鎬э細

  - 鑾峰彇闆嗘垚妯″潡鐨勯厤缃拰鐘舵€佷俊鎭?
  - 閰嶇疆 LDPC 鐮?
  - 閰嶇疆 Turbo 瑙ｇ爜
  - 鐩戣閿欒

SD-FEC 椹卞姩缂哄け鐨勭壒鎬с€佸凡鐭ラ棶棰樺強闄愬埗濡備笅锛?

  - 浠讳綍鏃跺埢瀵归┍鍔ㄧ殑浠讳綍瀹炰緥鍙厑璁稿崟涓墦寮€鐨勬枃浠跺彞鏌?
  - SD-FEC 闆嗘垚妯″潡鐨勫浣嶄笉鐢辫椹卞姩鎺у埗
  - 涓嶆敮鎸佸叡浜?LDPC 鐮佽〃鍥炵粫锛坵raparound锛?

璁惧鏍戞潯鐩弿杩颁簬锛?
`linux-xlnx/Documentation/devicetree/bindings/misc/xlnx,sd-fec.yaml <https://github.com/Xilinx/linux-xlnx/blob/master/Documentation/devicetree/bindings/misc/xlnx%2Csd-fec.yaml>`_


### Modes of Operation


璇ラ┍鍔ㄥ湪涓ょ鎿嶄綔妯″紡涓嬩笌 SD-FEC 鏍稿績鍗忓悓宸ヤ綔锛?

  - 杩愯鏃堕厤缃?
  - 鍙紪绋嬮€昏緫锛圥L锛夊垵濮嬪寲


#### Run-time Configuration


瀵逛簬杩愯鏃堕厤缃紝椹卞姩鐨勪綔鐢ㄦ槸鍏佽杞欢搴旂敤鎵ц浠ヤ笅鎿嶄綔锛?

 - 鍔犺浇 Turbo 瑙ｇ爜鎴?LDPC 缂栫爜鎴栬В鐮佺殑閰嶇疆鍙傛暟
 - 婵€娲?SD-FEC 鏍稿績
 - 鐩戣 SD-FEC 鏍稿績鏄惁鍑洪敊
 - 鑾峰彇 SD-FEC 鏍稿績鐨勭姸鎬佸拰閰嶇疆

#### Programmable Logic (PL) Initialization


瀵逛簬 PL 鍒濆鍖栵紝鏀寔閫昏緫浼氬姞杞?Turbo 瑙ｇ爜鎴?LDPC 缂栫爜鎴栬В鐮佺殑閰嶇疆鍙傛暟銆傞┍鍔ㄧ殑浣滅敤鏄?
鍏佽杞欢搴旂敤鎵ц浠ヤ笅鎿嶄綔锛?

 - 婵€娲?SD-FEC 鏍稿績
 - 鐩戣 SD-FEC 鏍稿績鏄惁鍑洪敊
 - 鑾峰彇 SD-FEC 鏍稿績鐨勭姸鎬佸拰閰嶇疆


## Driver Structure


璇ラ┍鍔ㄦ彁渚涗竴涓钩鍙拌澶囷紝鍏朵腑鎻愪緵浜?`probe` 鍜?`remove` 鎿嶄綔銆?

  - probe锛氱敤璁惧鏍戞潯鐩洿鏂伴厤缃瘎瀛樺櫒锛屽苟纭畾鏍稿績褰撳墠鐨勬縺娲荤姸鎬侊紝渚嬪鏍稿績鏄惁琚梺璺垨
    鏍稿績鏄惁宸插惎鍔ㄣ€?


璇ラ┍鍔ㄥ畾涔変簡浠ヤ笅椹卞姩鏂囦欢鎿嶄綔锛屼互鎻愪緵鐢ㄦ埛搴旂敤鎺ュ彛锛?

  - open锛氬疄鐜伴檺鍒讹紝鍗虫瘡涓?SD-FEC 瀹炰緥鍦ㄤ换浣曟椂鍒诲彧鑳芥墦寮€涓€涓枃浠舵弿杩扮
  - release锛氬厑璁告墦寮€鍙︿竴涓枃浠舵弿杩扮锛屽嵆褰撳墠鏂囦欢鎻忚堪绗﹀叧闂箣鍚?
  - poll锛氭彁渚涗竴绉嶇洃瑙?SD-FEC 閿欒浜嬩欢鐨勬柟娉?
  - unlocked_ioctl锛氭彁渚涗互涓?ioctl 鍛戒护锛屽厑璁稿簲鐢ㄩ厤缃?SD-FEC 鏍稿績锛?

  - `XSDFEC_START_DEV`
  - `XSDFEC_STOP_DEV`
  - `XSDFEC_GET_STATUS`
  - `XSDFEC_SET_IRQ`
  - `XSDFEC_SET_TURBO`
  - `XSDFEC_ADD_LDPC_CODE_PARAMS`
  - `XSDFEC_GET_CONFIG`
  - `XSDFEC_SET_ORDER`
  - `XSDFEC_SET_BYPASS`
  - `XSDFEC_IS_ACTIVE`
  - `XSDFEC_CLEAR_STATS`
  - `XSDFEC_SET_DEFAULT_CONFIG`


## Driver Usage



### Overview


鎵撳紑椹卞姩鍚庯紝鐢ㄦ埛搴旂‘瀹氶渶瑕佹墽琛屽摢浜涙搷浣滄潵閰嶇疆鍜屾縺娲?SD-FEC 鏍稿績锛屽苟纭畾椹卞姩鐨勯厤缃€?
浠ヤ笅鏄敤鎴峰簲褰撻伒寰殑娴佺▼锛?

  - 纭畾閰嶇疆
  - 璁剧疆椤哄簭锛坥rder锛夛紝濡傛灉灏氭湭鎸夋湡鏈涢厤缃?
  - 璁剧疆 Turbo 瑙ｇ爜銆丩DPC 缂栫爜鎴栬В鐮佸弬鏁帮紝鍏蜂綋鍙栧喅浜?SD-FEC 鏍稿績鐨勯厤缃柟寮忥紝浠ュ強 SD-FEC
    鏄惁灏氭湭閰嶇疆涓?PL 鍒濆鍖?
  - 鍚敤涓柇锛堝鏋滃皻鏈惎鐢級
  - 鏃佽矾 SD-FEC 鏍稿績锛堝鏋滈渶瑕侊級
  - 鍚姩 SD-FEC 鏍稿績锛堝鏋滃皻鏈惎鍔級
  - 鑾峰彇 SD-FEC 鏍稿績鐘舵€?
  - 鐩戣涓柇
  - 鍋滄 SD-FEC 鏍稿績


娉ㄦ剰锛氬湪鐩戣涓柇鏃讹紝濡傛灉妫€娴嬪埌闇€瑕佸浣嶇殑鍏抽敭閿欒锛屽垯闇€瑕侀┍鍔ㄥ姞杞介粯璁ら厤缃€?


### Determine Configuration


閫氳繃浣跨敤 ioctl `XSDFEC_GET_CONFIG` 纭畾 SD-FEC 鏍稿績鐨勯厤缃€?

### Set the Order


璁剧疆椤哄簭锛坥rder锛夊喅瀹氫簡浠庤緭鍏ュ埌杈撳嚭鏃?Block 鐨勯『搴忓浣曞彉鍖栥€?

璁剧疆椤哄簭鏄€氳繃浣跨敤 ioctl `XSDFEC_SET_ORDER` 瀹屾垚鐨?

鍙湁鍦ㄦ弧瓒充互涓嬮檺鍒舵椂鎵嶈兘璁剧疆椤哄簭锛?

 - 鐢?ioctl `XSDFEC_GET_STATUS` 濉厖鐨?struct `xsdfec_status <xsdfec_status>` 鐨?
   `state` 鎴愬憳鎸囩ず SD-FEC 鏍稿績灏氭湭 STARTED


### Add LDPC Codes


浠ヤ笅姝ラ璇存槑濡備綍鍚?SD-FEC 鏍稿績娣诲姞 LDPC 鐮侊細

 - 浣跨敤鑷姩鐢熸垚鐨勫弬鏁板～鍏呮墍闇€ LDPC 鐮佺殑 `struct xsdfec_ldpc_params <xsdfec_ldpc_params>`銆?
 - 涓?LPDC 鍙傛暟浠ュ強缁撴瀯 `struct xsdfec_ldpc_params <xsdfec_ldpc_params>` 涓殑鍙傛暟璁剧疆 SC銆?
   QA 鍜?LA 琛ㄥ亸绉?
 - 鍦ㄧ粨鏋?`struct xsdfec_ldpc_params <xsdfec_ldpc_params>` 涓缃湡鏈涚殑 Code Id 鍊?
 - 浣跨敤 ioctl `XSDFEC_ADD_LDPC_CODE_PARAMS` 娣诲姞 LPDC 鐮佸弬鏁?
 - 瀵逛簬鎵€搴旂敤鐨?LPDC 鐮佸弬鏁帮紝浣跨敤鍑芥暟 `xsdfec_calculate_shared_ldpc_table_entry_size`
   璁＄畻鍏变韩 LPDC 鐮佽〃鐨勫ぇ灏忋€傝繖璁╃敤鎴疯兘澶熺‘瀹氬叡浜〃鐨勪娇鐢ㄦ儏鍐碉紝浠庤€屽湪閫夋嫨涓嬩竴涓?LDPC 鐮?
   鍙傛暟鐨勮〃鍋忕Щ鏃跺彲浠ラ€夋嫨鏈娇鐢ㄧ殑琛ㄥ尯鍩熴€?
 - 瀵规瘡涓?LDPC 鐮佸弬鏁伴噸澶嶄笂杩版楠ゃ€?

鍙湁鍦ㄦ弧瓒充互涓嬮檺鍒舵椂鎵嶈兘娣诲姞 LDPC 鐮侊細

 - 鐢?ioctl `XSDFEC_GET_CONFIG` 濉厖鐨?`struct xsdfec_config <xsdfec_config>` 鐨?`code`
   鎴愬憳鎸囩ず SD-FEC 鏍稿績宸查厤缃负 LDPC
 - 鐢?ioctl `XSDFEC_GET_CONFIG` 濉厖鐨?`struct xsdfec_config <xsdfec_config>` 鐨?
   `code_wr_protect` 鎸囩ず鏈惎鐢ㄥ啓淇濇姢
 - 鐢?ioctl `XSDFEC_GET_STATUS` 濉厖鐨?struct `xsdfec_status <xsdfec_status>` 鐨?`state`
   鎴愬憳鎸囩ず SD-FEC 鏍稿績灏氭湭鍚姩

### Set Turbo Decode


閰嶇疆 Turbo 瑙ｇ爜鍙傛暟鏄€氳繃浣跨敤 ioctl `XSDFEC_SET_TURBO` 瀹屾垚鐨勶紝浣跨敤鑷姩鐢熸垚鐨勫弬鏁板～鍏?
鎵€闇€ Turbo 鐮佺殑 `struct xsdfec_turbo <xsdfec_turbo>`銆?

鍙湁鍦ㄦ弧瓒充互涓嬮檺鍒舵椂鎵嶈兘娣诲姞 Turbo 瑙ｇ爜锛?

 - 鐢?ioctl `XSDFEC_GET_CONFIG` 濉厖鐨?`struct xsdfec_config <xsdfec_config>` 鐨?`code`
   鎴愬憳鎸囩ず SD-FEC 鏍稿績宸查厤缃负 TURBO
 - 鐢?ioctl `XSDFEC_GET_STATUS` 濉厖鐨?struct `xsdfec_status <xsdfec_status>` 鐨?`state`
   鎴愬憳鎸囩ず SD-FEC 鏍稿績灏氭湭 STARTED

### Enable Interrupts


鍚敤鎴栫鐢ㄤ腑鏂槸閫氳繃浣跨敤 ioctl `XSDFEC_SET_IRQ` 瀹屾垚鐨勩€備紶閫掔粰 ioctl 鐨勫弬鏁?
`struct xsdfec_irq <xsdfec_irq>` 鐨勬垚鍛樼敤浜庤缃拰娓呴櫎涓嶅悓绫诲埆鐨勪腑鏂€備腑鏂被鍒殑鎺у埗
濡備笅锛?

  - `enable_isr` 鎺у埗 `tlast` 涓柇
  - `enable_ecc_isr` 鎺у埗 ECC 涓柇

濡傛灉鐢?ioctl `XSDFEC_GET_CONFIG` 濉厖鐨?`struct xsdfec_config <xsdfec_config>` 鐨?`code`
鎴愬憳鎸囩ず SD-FEC 鏍稿績宸查厤缃负 TURBO锛屽垯涓嶉渶瑕佸惎鐢?ECC 閿欒銆?

### Bypass the SD-FEC


鏃佽矾 SD-FEC 鏄€氳繃浣跨敤 ioctl `XSDFEC_SET_BYPASS` 瀹屾垚鐨?

鍙湁鍦ㄦ弧瓒充互涓嬮檺鍒舵椂鎵嶈兘鏃佽矾 SD-FEC锛?

 - 鐢?ioctl `XSDFEC_GET_STATUS` 濉厖鐨?struct `xsdfec_status <xsdfec_status>` 鐨?`state`
   鎴愬憳鎸囩ず SD-FEC 鏍稿績灏氭湭 STARTED

### Start the SD-FEC core


閫氳繃浣跨敤 ioctl `XSDFEC_START_DEV` 鍚姩 SD-FEC 鏍稿績

### Get SD-FEC Status


閫氳繃浣跨敤 ioctl `XSDFEC_GET_STATUS` 鑾峰彇璁惧鐨?SD-FEC 鐘舵€侊紝瀹冨皢濉厖
`struct xsdfec_status <xsdfec_status>`

### Monitor for Interrupts


 - 浣跨敤 poll 绯荤粺璋冪敤鐩戣涓柇銆俻oll 绯荤粺璋冪敤绛夊緟涓柇灏嗗叾鍞ら啋锛岃嫢鏃犱腑鏂彂鐢熷垯瓒呮椂銆?
 - 杩斿洖鏃?poll 鐨?`revents` 灏嗘寚绀?stats 鍜?鎴?state 鏄惁宸叉洿鏂?
  - `POLLPRI` 琛ㄧず鍏抽敭閿欒锛岀敤鎴峰簲浣跨敤 `XSDFEC_GET_STATUS` 鍜?`XSDFEC_GET_STATS` 鏉ョ‘璁?
  - `POLLRDNORM` 琛ㄧず鍙戠敓浜嗛潪鍏抽敭閿欒锛岀敤鎴峰簲浣跨敤 `XSDFEC_GET_STATS` 鏉ョ‘璁?
 - 浣跨敤 ioctl `XSDFEC_GET_STATS` 鑾峰彇缁熻淇℃伅
  - 瀵逛簬鍏抽敭閿欒锛宍struct xsdfec_stats <xsdfec_stats>` 鐨?`isr_err_count` 鎴?
    `uecc_count` 鎴愬憳闈為浂
  - 瀵逛簬闈炲叧閿敊璇紝`struct xsdfec_stats <xsdfec_stats>` 鐨?`cecc_count` 鎴愬憳闈為浂
 - 浣跨敤 ioctl `XSDFEC_GET_STATUS` 鑾峰彇鐘舵€?
  - 瀵逛簬鍏抽敭閿欒锛宍xsdfec_status <xsdfec_status>` 鐨?`state` 灏嗘寚绀洪渶瑕佸浣?
 - 浣跨敤 ioctl `XSDFEC_CLEAR_STATS` 娓呴櫎缁熻淇℃伅

濡傛灉妫€娴嬪埌闇€瑕佸浣嶇殑鍏抽敭閿欒锛屽簲鐢ㄧ▼搴忛渶瑕佸湪澶嶄綅鍚庤皟鐢?ioctl `XSDFEC_SET_DEFAULT_CONFIG`锛?
鑰屼笉闇€瑕佽皟鐢?ioctl `XSDFEC_STOP_DEV`

娉ㄦ剰锛氫娇鐢?poll 绯荤粺璋冪敤鍙伩鍏嶉€氳繃 `XSDFEC_GET_STATS` 鍜?`XSDFEC_GET_STATUS` 杩涜蹇欏惊鐜?

### Stop the SD-FEC Core


閫氳繃浣跨敤 ioctl `XSDFEC_STOP_DEV` 鍋滄璁惧

### Set the Default Configuration


閫氳繃浣跨敤 ioctl `XSDFEC_SET_DEFAULT_CONFIG` 鍔犺浇榛樿閰嶇疆浠ユ仮澶嶉┍鍔ㄣ€?

### Limitations


鐢ㄦ埛涓嶅簲澶嶅埗 SD-FEC 璁惧鏂囦欢鍙ユ焺锛屼緥濡?fork() 鎴?dup() 涓€涓凡鍒涘缓 SD-FEC 鏂囦欢鍙ユ焺鐨勮繘绋嬨€?

## Driver IOCTLs


   :doc: XSDFEC_START_DEV

   :doc: XSDFEC_STOP_DEV

   :doc: XSDFEC_GET_STATUS

   :doc: XSDFEC_SET_IRQ

   :doc: XSDFEC_SET_TURBO

   :doc: XSDFEC_ADD_LDPC_CODE_PARAMS

   :doc: XSDFEC_GET_CONFIG

   :doc: XSDFEC_SET_ORDER

   :doc: XSDFEC_SET_BYPASS

   :doc: XSDFEC_IS_ACTIVE

   :doc: XSDFEC_CLEAR_STATS

   :doc: XSDFEC_GET_STATS

   :doc: XSDFEC_SET_DEFAULT_CONFIG

## Driver Type Definitions


   :internal:
