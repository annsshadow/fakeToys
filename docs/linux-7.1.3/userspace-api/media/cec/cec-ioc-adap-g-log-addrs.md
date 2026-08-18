


######## ioctl CEC_ADAP_G_LOG_ADDRS 涓?CEC_ADAP_S_LOG_ADDRS


## 鍚嶇О


CEC_ADAP_G_LOG_ADDRS銆丆EC_ADAP_S_LOG_ADDRS - 鑾峰彇鎴栬缃€昏緫鍦板潃

## 姒傝


`int ioctl(int fd, CEC_ADAP_G_LOG_ADDRS, struct cec_log_addrs *argp)`


`int ioctl(int fd, CEC_ADAP_S_LOG_ADDRS, struct cec_log_addrs *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `cec_log_addrs` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠綋鍓嶇殑 CEC 閫昏緫鍦板潃锛屽簲鐢ㄧ▼搴忎互鎸囧悜 struct `cec_log_addrs` 鐨勬寚閽堣皟鐢?ioctl CEC_ADAP_G_LOG_ADDRS <CEC_ADAP_G_LOG_ADDRS>锛岄┍鍔ㄥ湪鍏朵腑瀛樺偍閫昏緫鍦板潃銆?
瑕佽缃柊鐨勯€昏緫鍦板潃锛屽簲鐢ㄧ▼搴忓～鍐?struct `cec_log_addrs` 骞朵互鎸囧悜姝ょ粨鏋勭殑鎸囬拡璋冪敤 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>銆俰octl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>
浠呭湪璁剧疆浜?`CEC_CAP_LOG_ADDRS` 鏃跺彲鐢紙鍚﹀垯杩斿洖 `ENOTTY` 閿欒鐮侊級銆俰octl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>
鍙兘鐢卞浜庡彂璧疯€呮ā寮忕殑鏂囦欢鎻忚堪绗﹁皟鐢紙瑙?CEC_S_MODE锛夛紝鍚﹀垯灏嗚繑鍥?`EBUSY` 閿欒鐮併€?
瑕佹竻闄ょ幇鏈夌殑閫昏緫鍦板潃锛屽皢 `num_log_addrs` 璁句负 0銆傛鏃舵墍鏈夊叾浠栧瓧娈甸兘灏嗚蹇界暐銆傞€傞厤鍣ㄥ皢杩涘叆鏈厤缃姸鎬侊紝涓?`cec_version`銆乣vendor_id` 鍜?`osd_name` 瀛楁閮借閲嶇疆涓哄叾榛樿鍊硷紙CEC 鐗堟湰 2.0銆佹棤鍘傚晢 ID 鍜岀┖鐨?OSD 鍚嶇О锛夈€?
濡傛灉鐗╃悊鍦板潃鏈夋晥锛堣 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR>锛夛紝鍒欐 ioctl 浼氶樆濉烇紝鐩村埌鎵€鏈夎姹傜殑閫昏緫鍦板潃閮借璁ら銆傚鏋滄枃浠舵弿杩扮澶勪簬闈為樆濉炴ā寮忥紝鍒欏畠涓嶄細绛夊緟閫昏緫鍦板潃琚棰嗭紝鑰屾槸鐩存帴杩斿洖 0銆?
褰撻€昏緫鍦板潃琚棰嗘垨娓呴櫎鏃讹紝浼氬彂閫佷竴涓?CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 浜嬩欢銆?
鍦ㄩ€昏緫鍦板潃绫诲瀷宸插畾涔夌殑鎯呭喌涓嬪皾璇曡皟鐢?ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 灏嗚繑鍥為敊璇?`EBUSY`銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u8
      - `log_addr[CEC_MAX_LOG_ADDRS]`
      - 琚棰嗙殑瀹為檯閫昏緫鍦板潃銆傜敱椹卞姩璁剧疆銆傚鏋滄棤娉曡棰嗕换浣曢€昏緫鍦板潃锛屽垯灏嗗叾璁句负
	`CEC_LOG_ADDR_INVALID`銆傚鏋滄閫傞厤鍣ㄦ槸鏈敞鍐岀殑锛屽垯
	`log_addr[^0^]` 璁句负 0xf锛屾墍鏈夊叾浠栧湴鍧€璁句负
	`CEC_LOG_ADDR_INVALID`銆?    - - __u16
      - `log_addr_mask`
      - 姝ら€傞厤鍣ㄥ凡璁ら鐨勬墍鏈夐€昏緫鍦板潃鐨勪綅鎺╃爜銆傚鏋滄閫傞厤鍣ㄦ槸鏈敞鍐岀殑锛屽垯 `log_addr_mask` 缃綅绗?15 浣?	骞舵竻闄ゆ墍鏈夊叾浠栦綅銆傚鏋滄閫傞厤鍣ㄦ牴鏈湭閰嶇疆锛屽垯 `log_addr_mask` 璁句负 0銆傜敱椹卞姩璁剧疆銆?    - - __u8
      - `cec_version`
      - 姝ら€傞厤鍣ㄥ簲褰撲娇鐢ㄧ殑 CEC 鐗堟湰銆傝
	cec-versions銆傜敤浜庡疄鐜?	`CEC_MSG_CEC_VERSION` 鍜?`CEC_MSG_REPORT_FEATURES` 娑堟伅銆?	娉ㄦ剰 CEC_OP_CEC_VERSION_1_3A <CEC-OP-CEC-VERSION-1-3A> 涓嶈 CEC 妗嗘灦鍏佽銆?    - - __u8
      - `num_log_addrs`
      - 瑕佽缃殑 logical 鍦板潃鏁伴噺銆傚繀椤?鈮?	CEC_ADAP_G_CAPS 杩斿洖鐨?	`available_log_addrs`銆傛缁撴瀯涓殑鎵€鏈夋暟缁勫彧濉厖鍒扮储寮?	`available_log_addrs`-1銆傚叾浣欐暟缁勫厓绱犲皢琚拷鐣ャ€傛敞鎰?CEC 2.0 鏍囧噯鍏佽鏈€澶?2 涓€昏緫鍦板潃锛屽敖绠℃煇浜涚‖浠舵敮鎸佹洿澶氥€?	`CEC_MAX_LOG_ADDRS` 涓?4銆傞┍鍔ㄥ皢杩斿洖瀹冨疄闄呰兘澶熻棰嗙殑閫昏緫鍦板潃鏁伴噺锛屽彲鑳藉皯浜庢墍璇锋眰鐨勩€傚鏋滄瀛楁璁句负 0锛屽垯 CEC
	閫傞厤鍣ㄥ簲娓呴櫎鎵€鏈夊凡璁ら鐨勯€昏緫鍦板潃锛屽苟涓旀墍鏈夊叾浠栧瓧娈甸兘灏嗚蹇界暐銆?    - - __u32
      - `vendor_id`
      - 鍘傚晢 ID 鏄竴涓?24 浣嶇殑鏁板瓧锛岀敤浜庢爣璇嗙壒瀹氱殑鍘傚晢鎴栧疄浣撱€傚熀浜庢 ID 鍙互瀹氫箟鍘傚晢鐗瑰畾鐨勫懡浠ゃ€傚鏋滀綘涓嶆兂瑕佸巶鍟?ID锛屽垯灏嗗叾璁句负
	`CEC_VENDOR_ID_NONE`銆?    - - __u32
      - `flags`
      - 鏍囧織銆傚彲鐢ㄦ爣蹇楀垪琛ㄨ cec-log-addrs-flags銆?    - - char
      - `osd_name[^15^]`
      - 鐢?`CEC_MSG_SET_OSD_NAME` 娑堟伅杩斿洖鐨勫睆涓婃樉绀哄悕绉般€?    - - __u8
      - `primary_device_type[CEC_MAX_LOG_ADDRS]`
      - 姣忎釜閫昏緫鍦板潃鐨勪富璁惧绫诲瀷銆傚彲鑳界被鍨嬭
	cec-prim-dev-types銆?    - - __u8
      - `log_addr_type[CEC_MAX_LOG_ADDRS]`
      - 閫昏緫鍦板潃绫诲瀷銆傚彲鑳界被鍨嬭 cec-log-addr-types銆?	椹卞姩浼氱敤瀹冨疄闄呰棰嗙殑閫昏緫鍦板潃绫诲瀷鏇存柊姝ゅ瓧娈碉紙渚嬪瀹冨彲鑳介渶瑕佸洖閫€鍒?CEC_LOG_ADDR_TYPE_UNREGISTERED <CEC-LOG-ADDR-TYPE-UNREGISTERED>锛夈€?    - - __u8
      - `all_device_types[CEC_MAX_LOG_ADDRS]`
      - CEC 2.0 鐗规湁锛氭墍鏈夎澶囩被鍨嬬殑浣嶆帺鐮併€傝
	cec-all-dev-types-flags銆傚畠鐢ㄤ簬 CEC 2.0 鐨?	`CEC_MSG_REPORT_FEATURES` 娑堟伅銆傚浜?CEC 1.4锛屼綘鍙互灏嗘瀛楁淇濈暀涓?0锛屾垨鑰呮寜鐓?CEC 2.0 鐨勬寚鍗楀～鍐欙紝浠ュ悜 CEC 妗嗘灦鎻愪緵鍏充簬璁惧绫诲瀷鐨勬洿澶氫俊鎭紝鍗充娇妗嗘灦涓嶄細鍦?CEC 娑堟伅涓洿鎺ヤ娇鐢ㄥ畠銆?    - - __u8
      - `features[CEC_MAX_LOG_ADDRS][^12^]`
      - 姣忎釜閫昏緫鍦板潃鐨勭壒鎬с€傚畠鐢ㄤ簬 CEC 2.0 鐨?	`CEC_MSG_REPORT_FEATURES` 娑堟伅銆傝繖 12 涓瓧鑺傚悓鏃跺寘鍚?	RC Profile 鍜岃澶囩壒鎬с€傚浜?CEC 1.4锛屼綘鍙互灏嗘瀛楁淇濈暀涓哄叏 0锛屾垨鑰呮寜鐓?CEC 2.0 鐨勬寚鍗楀～鍐欙紝浠ュ悜 CEC 妗嗘灦鎻愪緵鍏充簬璁惧绫诲瀷鐨勬洿澶氫俊鎭紝鍗充娇妗嗘灦涓嶄細鍦?CEC 娑堟伅涓洿鎺ヤ娇鐢ㄥ畠銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-LOG-ADDRS-FL-ALLOW-UNREG-FALLBACK`:

      - `CEC_LOG_ADDRS_FL_ALLOW_UNREG_FALLBACK`
      - 1
      - 榛樿鎯呭喌涓嬶紝濡傛灉鏃犳硶璁ら鎵€璇锋眰绫诲瀷鐨勯€昏緫鍦板潃锛屽垯瀹冨皢鍥炲埌鏈厤缃姸鎬併€傚鏋滆缃簡姝ゆ爣蹇楋紝鍒欏畠浼氬洖閫€鍒版湭娉ㄥ唽鐨勯€昏緫鍦板潃銆傛敞鎰忥紝濡傛灉鏄惧紡璇锋眰浜嗘湭娉ㄥ唽鐨勯€昏緫鍦板潃锛屽垯姝ゆ爣蹇椾笉璧蜂綔鐢ㄣ€?    - .. _`CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU`:

      - `CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU`
      - 2
      - 榛樿鎯呭喌涓嬶紝`CEC_MSG_USER_CONTROL_PRESSED` 鍜?`CEC_MSG_USER_CONTROL_RELEASED`
        娑堟伅鍙紶閫掔粰 follower锛堝鏋滄湁锛夈€傚鏋滆缃簡姝ゆ爣蹇楋紝鍒欒繖浜涙秷鎭篃浼氫紶閫掔粰杩滅▼鎺у埗杈撳叆瀛愮郴缁燂紝骞朵綔涓烘寜閿嚭鐜般€傛鐗规€ч渶瑕佹樉寮忓惎鐢ㄣ€傚鏋?CEC 鐢ㄤ簬杈撳叆瀵嗙爜绛夛紝浣犲彲鑳戒笉鎯冲惎鐢ㄦ鐗规€э紝浠ラ伩鍏嶅鎸夐敭鐨勭畝鍗曞梾鎺€?    - .. _`CEC-LOG-ADDRS-FL-CDC-ONLY`:

      - `CEC_LOG_ADDRS_FL_CDC_ONLY`
      - 4
      - 濡傛灉璁剧疆浜嗘鏍囧織锛屽垯璇ヨ澶囨槸 CDC-Only锛堜粎 CDC锛夈€侰DC-Only 鐨?CEC 璁惧鏄彧鑳藉鐞?CDC 娑堟伅鐨?CEC 璁惧銆?
	鎵€鏈夊叾浠栨秷鎭兘琚拷鐣ャ€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-CEC-VERSION-1-3A`:

      - `CEC_OP_CEC_VERSION_1_3A`
      - 4
      - 鏍规嵁 HDMI 1.3a 鏍囧噯鐨?CEC 鐗堟湰銆?    - .. _`CEC-OP-CEC-VERSION-1-4B`:

      - `CEC_OP_CEC_VERSION_1_4B`
      - 5
      - 鏍规嵁 HDMI 1.4b 鏍囧噯鐨?CEC 鐗堟湰銆?    - .. _`CEC-OP-CEC-VERSION-2-0`:

      - `CEC_OP_CEC_VERSION_2_0`
      - 6
      - 鏍规嵁 HDMI 2.0 鏍囧噯鐨?CEC 鐗堟湰銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-PRIM-DEVTYPE-TV`:

      - `CEC_OP_PRIM_DEVTYPE_TV`
      - 0
      - 鐢ㄤ簬鐢佃銆?    - .. _`CEC-OP-PRIM-DEVTYPE-RECORD`:

      - `CEC_OP_PRIM_DEVTYPE_RECORD`
      - 1
      - 鐢ㄤ簬褰曞儚璁惧銆?    - .. _`CEC-OP-PRIM-DEVTYPE-TUNER`:

      - `CEC_OP_PRIM_DEVTYPE_TUNER`
      - 3
      - 鐢ㄤ簬甯﹁皟璋愬櫒鐨勮澶囥€?    - .. _`CEC-OP-PRIM-DEVTYPE-PLAYBACK`:

      - `CEC_OP_PRIM_DEVTYPE_PLAYBACK`
      - 4
      - 鐢ㄤ簬鎾斁璁惧銆?    - .. _`CEC-OP-PRIM-DEVTYPE-AUDIOSYSTEM`:

      - `CEC_OP_PRIM_DEVTYPE_AUDIOSYSTEM`
      - 5
      - 鐢ㄤ簬闊抽绯荤粺锛堜緥濡傞煶棰?瑙嗛鎺ユ敹鍣級銆?    - .. _`CEC-OP-PRIM-DEVTYPE-SWITCH`:

      - `CEC_OP_PRIM_DEVTYPE_SWITCH`
      - 6
      - 鐢ㄤ簬 CEC 寮€鍏炽€?    - .. _`CEC-OP-PRIM-DEVTYPE-VIDEOPROC`:

      - `CEC_OP_PRIM_DEVTYPE_VIDEOPROC`
      - 7
      - 鐢ㄤ簬瑙嗛澶勭悊鍣ㄨ澶囥€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-LOG-ADDR-TYPE-TV`:

      - `CEC_LOG_ADDR_TYPE_TV`
      - 0
      - 鐢ㄤ簬鐢佃銆?    - .. _`CEC-LOG-ADDR-TYPE-RECORD`:

      - `CEC_LOG_ADDR_TYPE_RECORD`
      - 1
      - 鐢ㄤ簬褰曞儚璁惧銆?    - .. _`CEC-LOG-ADDR-TYPE-TUNER`:

      - `CEC_LOG_ADDR_TYPE_TUNER`
      - 2
      - 鐢ㄤ簬璋冭皭鍣ㄨ澶囥€?    - .. _`CEC-LOG-ADDR-TYPE-PLAYBACK`:

      - `CEC_LOG_ADDR_TYPE_PLAYBACK`
      - 3
      - 鐢ㄤ簬鎾斁璁惧銆?    - .. _`CEC-LOG-ADDR-TYPE-AUDIOSYSTEM`:

      - `CEC_LOG_ADDR_TYPE_AUDIOSYSTEM`
      - 4
      - 鐢ㄤ簬闊抽绯荤粺璁惧銆?    - .. _`CEC-LOG-ADDR-TYPE-SPECIFIC`:

      - `CEC_LOG_ADDR_TYPE_SPECIFIC`
      - 5
      - 鐢ㄤ簬绗簩鍙扮數瑙嗘垨瑙嗛澶勭悊鍣ㄨ澶囥€?    - .. _`CEC-LOG-ADDR-TYPE-UNREGISTERED`:

      - `CEC_LOG_ADDR_TYPE_UNREGISTERED`
      - 6
      - 濡傛灉浣犲彧鎯充繚鎸佹湭娉ㄥ唽锛屽垯浣跨敤姝ょ被鍨嬨€傜敤浜庣函 CEC 寮€鍏虫垨浠?CDC 璁惧锛圕DC锛氳兘鍔涘彂鐜颁笌鎺у埗锛夈€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-ALL-DEVTYPE-TV`:

      - `CEC_OP_ALL_DEVTYPE_TV`
      - 0x80
      - 鏀寔 TV 绫诲瀷銆?    - .. _`CEC-OP-ALL-DEVTYPE-RECORD`:

      - `CEC_OP_ALL_DEVTYPE_RECORD`
      - 0x40
      - 鏀寔褰曞埗绫诲瀷銆?    - .. _`CEC-OP-ALL-DEVTYPE-TUNER`:

      - `CEC_OP_ALL_DEVTYPE_TUNER`
      - 0x20
      - 鏀寔璋冭皭鍣ㄧ被鍨嬨€?    - .. _`CEC-OP-ALL-DEVTYPE-PLAYBACK`:

      - `CEC_OP_ALL_DEVTYPE_PLAYBACK`
      - 0x10
      - 鏀寔鎾斁绫诲瀷銆?    - .. _`CEC-OP-ALL-DEVTYPE-AUDIOSYSTEM`:

      - `CEC_OP_ALL_DEVTYPE_AUDIOSYSTEM`
      - 0x08
      - 鏀寔闊抽绯荤粺绫诲瀷銆?    - .. _`CEC-OP-ALL-DEVTYPE-SWITCH`:

      - `CEC_OP_ALL_DEVTYPE_SWITCH`
      - 0x04
      - 鏀寔 CEC 寮€鍏虫垨瑙嗛澶勭悊绫诲瀷銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 鍙互杩斿洖浠ヤ笅閿欒鐮侊細

ENOTTY
    鏈缃?`CEC_CAP_LOG_ADDRS` 鑳藉姏锛屽洜姝や笉鏀寔姝?ioctl銆?
EBUSY
    CEC 閫傞厤鍣ㄥ綋鍓嶆鍦ㄨ嚜琛岄厤缃紝鎴栬€呭畠宸茬粡閰嶇疆涓?    `num_log_addrs` 闈為浂锛屾垨鑰呭彟涓€涓枃浠跺彞鏌勫浜庣嫭鍗?follower 鎴栧彂璧疯€呮ā寮忥紝鎴栬€呮枃浠跺彞鏌勫浜?`CEC_MODE_NO_INITIATOR` 妯″紡銆?
EINVAL
    struct `cec_log_addrs` 鐨勫唴瀹规棤鏁堛€?