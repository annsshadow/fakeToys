


######## ioctls CEC_RECEIVE 鍜?CEC_TRANSMIT


## 鍚嶇О


CEC_RECEIVE銆丆EC_TRANSMIT - 鎺ユ敹鎴栧彂閫佷竴鏉?CEC 娑堟伅

## 姒傝



`int ioctl(int fd, CEC_RECEIVE, struct cec_msg *argp)`


`int ioctl(int fd, CEC_TRANSMIT, struct cec_msg *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct cec_msg 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹帴鏀朵竴鏉?CEC 娑堟伅锛屽簲鐢ㄧ▼搴忓繀椤诲～濂?struct `cec_msg` 鐨?`timeout` 瀛楁锛屽苟灏嗗叾浼犵粰 ioctl CEC_RECEIVE <CEC_RECEIVE>銆傚鏋滄枃浠舵弿杩扮澶勪簬闈為樆濉炴ā寮忥紝涓旀病鏈夊緟鎺ユ敹鐨勬秷鎭紝閭ｄ箞瀹冧細杩斿洖 -1 骞跺皢 errno 璁句负 `EAGAIN` 閿欒鐮併€傚鏋滄枃浠舵弿杩扮澶勪簬闃诲妯″紡锛屼笖 `timeout` 闈為浂锛岃€屽湪 `timeout` 姣鍐呮病鏈夋秷鎭埌杈撅紝閭ｄ箞瀹冧細杩斿洖 -1 骞跺皢 errno 璁句负 `ETIMEDOUT` 閿欒鐮併€?
涓€鏉℃帴鏀跺埌鐨勬秷鎭彲浠ユ槸锛?
1. 浠庡彟涓€涓?CEC 璁惧鎺ユ敹鍒扮殑娑堟伅锛坄sequence` 瀛楁涓?0锛宍tx_status` 涓?0锛宍rx_status` 闈為浂锛夈€?2. 涔嬪墠涓€娆￠潪闃诲鍙戦€佺殑鍙戦€佺粨鏋滐紙`sequence` 瀛楁闈為浂锛宍tx_status` 闈為浂锛宍rx_status` 涓?0锛夈€?3. 涔嬪墠涓€娆￠潪闃诲鍙戦€佺殑搴旂瓟锛坮eply锛夛紙`sequence` 瀛楁闈為浂锛宍tx_status` 涓?0锛宍rx_status` 闈為浂锛夈€?
瑕佸彂閫佷竴鏉?CEC 娑堟伅锛屽簲鐢ㄧ▼搴忓繀椤诲～濂?struct `cec_msg` 骞跺皢鍏朵紶缁?ioctl CEC_TRANSMIT <CEC_TRANSMIT>銆俰octl CEC_TRANSMIT <CEC_TRANSMIT> 浠呭湪璁剧疆浜?`CEC_CAP_TRANSMIT` 鏃舵墠鍙敤銆傚鏋滃彂閫侀槦鍒椾腑娌℃湁鏇村绌洪棿锛岄偅涔堝畠浼氳繑鍥?-1 骞跺皢 errno 璁句负 `EBUSY` 閿欒鐮併€傚彂閫侀槦鍒楁湁瓒冲鐨勭┖闂村绾?18 鏉℃秷鎭紙澶х害鐩稿綋浜?1 绉掔殑 2 瀛楄妭娑堟伅锛夈€傛敞鎰忥紝CEC 鍐呮牳妗嗘灦涔熶細璁★紙reply锛夋牳蹇冩秷鎭紙鍙傝 cec-core-processing锛夛紝鍥犳灏嗗彂閫侀槦鍒楀畬鍏ㄥ～婊″苟涓嶆槸涓ソ涓绘剰銆?
濡傛灉鏂囦欢鎻忚堪绗﹀浜庨潪闃诲妯″紡锛岄偅涔堝彂閫佷細杩斿洖 0锛屽苟涓斿湪鍙戦€佸畬鎴愬悗锛屽彂閫佺殑缁撴灉鍙€氳繃 ioctl CEC_RECEIVE <CEC_RECEIVE> 鑾峰緱銆傚鏋滀竴娆￠潪闃诲鍙戦€佽繕鎸囧畾浜嗙瓑寰呭簲绛旓紙reply锛夛紝閭ｄ箞搴旂瓟浼氬湪涓€涓悗缁秷鎭腑鍒拌揪銆俙sequence` 瀛楁鍙敤浜庡皢鍙戦€佺粨鏋滃拰搴旂瓟涓庡師濮嬪彂閫佺浉鍏宠仈銆?
閫氬父锛屽綋鐗╃悊鍦板潃鏃犳晥鏃讹紙渚嬪鐢变簬鏂紑杩炴帴锛夎皟鐢?ioctl CEC_TRANSMIT <CEC_TRANSMIT> 浼氳繑鍥?`ENONET`銆?
鐒惰€岋紝CEC 瑙勮寖鍏佽鍦ㄧ墿鐞嗗湴鍧€鏃犳晥鏃讹紝浠?'Unregistered' 鍚?'TV' 鍙戦€佹秷鎭紝鍥犱负鏌愪簺鐢佃鍦ㄨ繘鍏ュ緟鏈虹姸鎬佹垨鍒囨崲鍒板彟涓€涓緭鍏ユ椂锛屼細灏?HDMI 杩炴帴鍣ㄧ殑鐑彃鎷旀娴嬶紙hotplug detect锛夊紩鑴氭媺浣庛€?
褰撶儹鎻掓嫈妫€娴嬪紩鑴氬彉浣庢椂锛孍DID 娑堝け锛屼粠鑰岀墿鐞嗗湴鍧€涔熸秷澶憋紝浣嗙嚎缂嗕粛鐒惰繛鎺ワ紝CEC 浠嶇劧宸ヤ綔銆備负浜嗘娴?鍞ら啋璁惧锛屽厑璁镐粠鍙戣捣鑰?0xf锛?Unregistered'锛夊悜鐩爣 0锛?TV'锛夊彂閫佽疆璇紙poll锛夊拰 'Image/Text View On' 娑堟伅銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u64
      - `tx_ts`
      - 娑堟伅鏈€鍚庝竴涓瓧鑺傝鍙戦€佹椂鐨勬椂闂存埑锛屽崟浣嶄负 ns銆?	璇ユ椂闂存埑鍙栬嚜 `CLOCK_MONOTONIC` 鏃堕挓銆傝浠庣敤鎴风┖闂磋闂悓涓€涓椂閽燂紝
	璇蜂娇鐢?`clock_gettime`銆?    - - __u64
      - `rx_ts`
      - 娑堟伅鏈€鍚庝竴涓瓧鑺傝鎺ユ敹鏃剁殑鏃堕棿鎴筹紝鍗曚綅涓?ns銆?	璇ユ椂闂存埑鍙栬嚜 `CLOCK_MONOTONIC` 鏃堕挓銆傝浠庣敤鎴风┖闂磋闂悓涓€涓椂閽燂紝
	璇蜂娇鐢?`clock_gettime`銆?    - - __u32
      - `len`
      - 娑堟伅鐨勯暱搴︺€傚浜?ioctl CEC_TRANSMIT <CEC_TRANSMIT>锛?	杩欑敱搴旂敤绋嬪簭濉叆銆傞┍鍔ㄤ細涓?ioctl CEC_RECEIVE <CEC_RECEIVE> 濉叆姝ゅ瓧娈点€?	瀵逛簬 ioctl CEC_TRANSMIT <CEC_TRANSMIT>锛屽鏋滆缃簡 `reply`锛?	椹卞姩浼氬皢鍏跺～鍏ヤ负搴旂瓟娑堟伅鐨勯暱搴︺€?    - - __u32
      - `timeout`
      - 瓒呮椂鏃堕棿锛屽崟浣嶄负姣銆傝繖鏄澶囧湪瓒呮椂涔嬪墠绛夊緟鎺ユ敹涓€鏉℃秷鎭殑鏃堕棿銆?	濡傛灉璁句负 0锛岄偅涔堝綋瀹冪敱 ioctl CEC_RECEIVE <CEC_RECEIVE> 璋冪敤鏃讹紝
	灏嗘棤闄愭湡绛夊緟銆傚鏋滃畠涓?0 涓旂敱 ioctl CEC_TRANSMIT <CEC_TRANSMIT> 璋冪敤锛?	閭ｄ箞褰?`reply` 闈為浂鏃跺畠浼氳鏇挎崲涓?1000锛屾垨鑰呭綋 `reply` 涓?0 鏃惰蹇界暐銆?    - - __u32
      - `sequence`
      - 涓€涓潪闆剁殑搴忓垪鍙凤紝鐢?CEC 妗嗘灦涓烘墍鏈夊凡鍙戦€佺殑娑堟伅鑷姩鍒嗛厤銆?	褰?CEC 妗嗘灦涓洪潪闃诲鍙戦€佹帓闃熷彂閫佺粨鏋滄椂锛屼細鐢ㄥ埌瀹冦€?	杩欎娇寰楀簲鐢ㄧ▼搴忚兘澶熷皢鎺ユ敹鍒扮殑娑堟伅涓庡師濮嬪彂閫佺浉鍏宠仈銆?
	姝ゅ锛屽鏋滀竴娆￠潪闃诲鍙戦€佷細绛夊緟搴旂瓟锛堝嵆 `timeout` 涓嶄负 0锛夛紝
	閭ｄ箞搴旂瓟鐨?`sequence` 瀛楁浼氳璁句负鍘熷鍙戦€佺殑搴忓垪鍊笺€?	杩欎娇寰楀簲鐢ㄧ▼搴忚兘澶熷皢鎺ユ敹鍒扮殑娑堟伅涓庡師濮嬪彂閫佺浉鍏宠仈銆?    - - __u32
      - `flags`
      - 鏍囧織浣嶃€傚彲鐢ㄦ爣蹇楀垪琛ㄥ弬瑙?cec-msg-flags銆?    - - __u8
      - `msg[^16^]`
      - 娑堟伅鏈夋晥杞借嵎銆傚浜?ioctl CEC_TRANSMIT <CEC_TRANSMIT>锛?	杩欑敱搴旂敤绋嬪簭濉叆銆傞┍鍔ㄤ細涓?ioctl CEC_RECEIVE <CEC_RECEIVE> 濉叆姝ゅ瓧娈点€?	瀵逛簬 ioctl CEC_TRANSMIT <CEC_TRANSMIT>锛屽鏋滆缃簡 `timeout`锛?	椹卞姩浼氬皢鍏跺～鍏ヤ负搴旂瓟娑堟伅鐨勬湁鏁堣浇鑽枫€?    - - __u8
      - `reply`
      - 绛夊緟姝ゆ秷鎭搴旂瓟銆傚鏋?`reply` 涓?0 涓?`timeout` 涓?0锛?	鍒欎笉绛夊緟搴旂瓟锛岃€屾槸鍦ㄥ彂閫佹秷鎭悗杩斿洖銆俰octl CEC_RECEIVE <CEC_RECEIVE> 浼氬拷鐣ュ畠銆?	`reply` 涓?0锛堣繖鏄?Feature Abort 娑堟伅鐨勬搷浣滅爜锛変笖 `timeout` 闈為浂鐨勬儏鍐?	琚壒鎰忓厑璁革紝浠ヤ究鑳藉鍙戦€佷竴鏉℃秷鎭苟绛夊緟鏈€澶?`timeout` 姣浠ユ敹鍒颁竴涓?	Feature Abort 搴旂瓟銆傚湪杩欑鎯呭喌涓嬶紝`rx_status` 浼氳璁句负
	CEC_RX_STATUS_TIMEOUT <CEC-RX-STATUS-TIMEOUT> 鎴?	CEC_RX_STATUS_FEATURE_ABORT <CEC-RX-STATUS-FEATURE-ABORT>銆?
	濡傛灉鍙戦€佹柟娑堟伅鏄?`CEC_MSG_INITIATE_ARC`锛岄偅涔?`reply` 鍊?	`CEC_MSG_REPORT_ARC_INITIATED` 鍜?`CEC_MSG_REPORT_ARC_TERMINATED`
	浼氳鍖哄埆澶勭悊锛氫换涓€鍊奸兘鑳藉尮閰嶄袱绉嶅彲鑳界殑搴旂瓟銆?	鍘熷洜鏄?`CEC_MSG_INITIATE_ARC` 娑堟伅鏄敮涓€涓€鏉￠櫎 Feature Abort 澶?	杩樻湁涓ょ鍙兘搴旂瓟鐨?CEC 娑堟伅銆俙reply` 瀛楁浼氳鏇存柊涓哄疄闄呯殑搴旂瓟锛?	浠ヤ究涓庢墍鎺ユ敹娑堟伅鐨勫唴瀹逛繚鎸佸悓姝ャ€?    - - __u8
      - `rx_status`
      - 鎵€鎺ユ敹娑堟伅鐨勭姸鎬佷綅銆傚彲鑳界殑鐘舵€佸€煎弬瑙?cec-rx-status銆?    - - __u8
      - `tx_status`
      - 鎵€鍙戦€佹秷鎭殑鐘舵€佷綅銆傚彲鑳界殑鐘舵€佸€煎弬瑙?cec-tx-status銆?	褰撲互闈為樆濉炴ā寮忚皟鐢?ioctl CEC_TRANSMIT <CEC_TRANSMIT> 鏃讹紝
	濡傛灉鍙戦€佸凡寮€濮嬶紝姝ゅ瓧娈典负 0锛涘鏋滃彂閫佺粨鏋滅珛鍗冲彲鐭ワ紝鍒欎负闈?0銆?	鍚庝竴绉嶆儏鍐靛彂鐢熷湪灏濊瘯鍚戣嚜宸卞彂閫?Poll 娑堟伅鏃躲€傝繖浼氬鑷翠竴涓?	CEC_TX_STATUS_NACK <CEC-TX-STATUS-NACK>锛岃€屽疄闄呬笂浠庢湭鍙戦€佽 Poll 娑堟伅銆?    - - __u8
      - `tx_arb_lost_cnt`
      - 瀵艰嚧浠茶涓㈠け锛圓rbitration Lost锛夐敊璇殑鍙戦€佸皾璇曡鏁般€?	浠呭湪纭欢鏀寔姝ゅ姛鑳芥椂璁剧疆锛屽惁鍒欏缁堜负 0銆?	姝よ鏁板櫒浠呭湪璁剧疆浜?CEC_TX_STATUS_ARB_LOST <CEC-TX-STATUS-ARB-LOST>
	鐘舵€佷綅鏃舵湁鏁堛€?    - - __u8
      - `tx_nack_cnt`
      - 瀵艰嚧鏈‘璁わ紙Not Acknowledged锛夐敊璇殑鍙戦€佸皾璇曡鏁般€?	浠呭湪纭欢鏀寔姝ゅ姛鑳芥椂璁剧疆锛屽惁鍒欏缁堜负 0銆?	姝よ鏁板櫒浠呭湪璁剧疆浜?CEC_TX_STATUS_NACK <CEC-TX-STATUS-NACK>
	鐘舵€佷綅鏃舵湁鏁堛€?    - - __u8
      - `tx_low_drive_cnt`
      - 瀵艰嚧浠茶涓㈠け锛圓rbitration Lost锛夐敊璇殑鍙戦€佸皾璇曡鏁般€?	浠呭湪纭欢鏀寔姝ゅ姛鑳芥椂璁剧疆锛屽惁鍒欏缁堜负 0銆?	姝よ鏁板櫒浠呭湪璁剧疆浜?CEC_TX_STATUS_LOW_DRIVE <CEC-TX-STATUS-LOW-DRIVE>
	鐘舵€佷綅鏃舵湁鏁堛€?    - - __u8
      - `tx_error_cnt`
      - 闄や徊瑁佷涪澶辨垨鏈‘璁や箣澶栫殑鍙戦€侀敊璇鏁般€?	浠呭湪纭欢鏀寔姝ゅ姛鑳芥椂璁剧疆锛屽惁鍒欏缁堜负 0銆?	姝よ鏁板櫒浠呭湪璁剧疆浜?CEC_TX_STATUS_ERROR <CEC-TX-STATUS-ERROR>
	鐘舵€佷綅鏃舵湁鏁堛€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-MSG-FL-REPLY-TO-FOLLOWERS`:

      - `CEC_MSG_FL_REPLY_TO_FOLLOWERS`
      - 1
      - 濡傛灉涓€娆?CEC 鍙戦€佹湡鏈涗竴涓簲绛旓紝閭ｄ箞榛樿鎯呭喌涓嬭搴旂瓟鍙彂閫佺粰
	璋冪敤浜?ioctl CEC_TRANSMIT <CEC_TRANSMIT> 鐨勬枃浠跺彞鏌勶紙filehandle锛夈€?	濡傛灉璁剧疆浜嗘鏍囧織锛岄偅涔堝簲绛斾篃浼氬彂閫佺粰鎵€鏈?follower锛堝鏋滄湁鐨勮瘽锛夈€?	濡傛灉璋冪敤浜?ioctl CEC_TRANSMIT <CEC_TRANSMIT> 鐨勬枃浠跺彞鏌勬湰韬篃鏄竴涓?	follower锛岄偅涔堣鏂囦欢鍙ユ焺浼氭敹鍒颁袱娆″簲绛旓細涓€娆′綔涓?ioctl CEC_TRANSMIT <CEC_TRANSMIT>
	鐨勭粨鏋滐紝涓€娆￠€氳繃 ioctl CEC_RECEIVE <CEC_RECEIVE>銆?
    - .. _`CEC-MSG-FL-RAW`:

      - `CEC_MSG_FL_RAW`
      - 2
      - 閫氬父 CEC 娑堟伅鍦ㄥ彂閫佸墠浼氱粡杩囨牎楠屻€傚鏋滆皟鐢?ioctl CEC_TRANSMIT <CEC_TRANSMIT>
	鏃惰缃簡姝ゆ爣蹇楋紝鍒欎笉杩涜浠讳綍鏍￠獙锛屾秷鎭寜鍘熸牱鍙戦€併€傝繖鍦ㄨ皟璇?CEC 闂鏃跺緢鏈夌敤銆?	姝ゆ爣蹇椾粎鍦ㄨ繘绋嬪叿鏈?`CAP_SYS_RAWIO` 鑳藉姏锛坈apability锛夋椂鎵嶅厑璁镐娇鐢ㄣ€?	濡傛灉鏈缃紝鍒欒繑鍥?`EPERM` 閿欒鐮併€?
    - .. _`CEC-MSG-FL-REPLY-VENDOR-ID`:

      - `CEC_MSG_FL_REPLY_VENDOR_ID`
      - 4
      - 姝ゆ爣蹇椾粎鍦ㄨ缃簡 `CEC_CAP_REPLY_VENDOR_ID` 鑳藉姏鏃跺彲鐢ㄣ€?	濡傛灉璁剧疆浜嗘鏍囧織锛屽垯鏈熸湜搴旂瓟鐢?`CEC_MSG_VENDOR_COMMAND_WITH_ID` 鎿嶄綔鐮?	鍚庤窡鍘傚晢 ID锛堟秷鎭殑绗?1-4 瀛楄妭锛夛紝鍐嶅悗璺?struct cec_msg 鐨?`reply` 瀛楁缁勬垚銆?
	娉ㄦ剰锛岃繖鍋囪鍘傚晢 ID 涔嬪悗鐨勫瓧鑺傛槸涓€涓巶鍟嗙壒瀹氱殑鎿嶄綔鐮併€?
	姝ゆ爣蹇椾娇寰楃瓑寰呭巶鍟嗗懡浠ょ殑搴旂瓟鍙樺緱鏇村姞瀹规槗銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-TX-STATUS-OK`:

      - `CEC_TX_STATUS_OK`
      - 0x01
      - 娑堟伅鍙戦€佹垚鍔熴€傝繖涓?CEC_TX_STATUS_MAX_RETRIES <CEC-TX-STATUS-MAX-RETRIES>
	浜掓枼銆傚鏋滄棭鏈熷皾璇曞湪鏈€缁堝彂閫佹垚鍔熶箣鍓嶉伃閬囧け璐ワ紝鍏朵粬浣嶄粛鍙璁剧疆銆?    - .. _`CEC-TX-STATUS-ARB-LOST`:

      - `CEC_TX_STATUS_ARB_LOST`
      - 0x02
      - CEC 绾胯矾浠茶涓㈠け锛屽嵆鍙︿竴涓彂閫佸湪鍚屾椂浠ユ洿楂樹紭鍏堢骇寮€濮嬨€?	鍙€夌姸鎬侊紝骞堕潪鎵€鏈夌‖浠堕兘鑳芥娴嬪埌姝ら敊璇潯浠躲€?    - .. _`CEC-TX-STATUS-NACK`:

      - `CEC_TX_STATUS_NACK`
      - 0x04
      - 娑堟伅鏈纭銆傛敞鎰忔煇浜涚‖浠舵棤娉曞尯鍒?鏈‘璁?鐘舵€佷笌鍏朵粬閿欒鏉′欢锛?	鍗冲彂閫佺粨鏋滃彧鏈?OK 鎴?FAIL銆傚湪杩欑鎯呭喌涓嬶紝鍙戦€佸け璐ユ椂浼氳繑鍥炴鐘舵€併€?    - .. _`CEC-TX-STATUS-LOW-DRIVE`:

      - `CEC_TX_STATUS_LOW_DRIVE`
      - 0x08
      - 鍦?CEC 鎬荤嚎涓婃娴嬪埌浣庨┍鍔紙low drive锛夈€傝繖琛ㄧず鏌愪釜 follower
	妫€娴嬪埌鎬荤嚎涓婄殑閿欒骞惰姹傞噸浼犮€傚彲閫夌姸鎬侊紝骞堕潪鎵€鏈夌‖浠堕兘鑳芥娴嬪埌姝ら敊璇潯浠躲€?    - .. _`CEC-TX-STATUS-ERROR`:

      - `CEC_TX_STATUS_ERROR`
      - 0x10
      - 鍙戠敓浜嗘煇浜涢敊璇€傝繖鐢ㄤ簬浠讳綍涓嶉€傚悎 `CEC_TX_STATUS_ARB_LOST`
	鎴?`CEC_TX_STATUS_LOW_DRIVE` 鐨勯敊璇紝鍙兘鏄洜涓虹‖浠舵棤娉曞垽鏂彂鐢熶簡鍝釜閿欒锛?	鎴栬€呯‖浠舵祴璇曚簡闄よ繖涓よ€呬箣澶栫殑鍏朵粬鏉′欢銆傚彲閫夌姸鎬併€?    - .. _`CEC-TX-STATUS-MAX-RETRIES`:

      - `CEC_TX_STATUS_MAX_RETRIES`
      - 0x20
      - 鍦ㄩ噸璇曚竴娆℃垨澶氭鍚庡彂閫佷粛鐒跺け璐ャ€傛鐘舵€佷綅涓?CEC_TX_STATUS_OK <CEC-TX-STATUS-OK>
	浜掓枼銆傚叾浠栦綅浠嶅彲琚缃紝浠ヨ鏄庣湅鍒颁簡鍝簺澶辫触銆?    - .. _`CEC-TX-STATUS-ABORTED`:

      - `CEC_TX_STATUS_ABORTED`
      - 0x40
      - 鍙戦€佸洜 HDMI 鏂紑杩炴帴銆佹垨閫傞厤鍣ㄨ鍙栨秷閰嶇疆锛坲nconfigured锛夈€?	鎴栦竴娆″彂閫佽涓柇銆佹垨椹卞姩鍦ㄥ皾璇曞紑濮嬩竴娆″彂閫佹椂杩斿洖閿欒鑰岃涓銆?    - .. _`CEC-TX-STATUS-TIMEOUT`:

      - `CEC_TX_STATUS_TIMEOUT`
      - 0x80
      - 鍙戦€佽秴鏃躲€傝繖閫氬父涓嶅簲鍙戠敓锛岃〃鏄庡瓨鍦ㄩ┍鍔ㄩ棶棰樸€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-RX-STATUS-OK`:

      - `CEC_RX_STATUS_OK`
      - 0x01
      - 娑堟伅鎺ユ敹鎴愬姛銆?    - .. _`CEC-RX-STATUS-TIMEOUT`:

      - `CEC_RX_STATUS_TIMEOUT`
      - 0x02
      - 瀵逛竴鏉¤緝鏃╁彂閫佹秷鎭殑搴旂瓟瓒呮椂銆?    - .. _`CEC-RX-STATUS-FEATURE-ABORT`:

      - `CEC_RX_STATUS_FEATURE_ABORT`
      - 0x04
      - 娑堟伅鎺ユ敹鎴愬姛锛屼絾搴旂瓟鏄?`CEC_MSG_FEATURE_ABORT`銆?	姝ょ姸鎬佷粎鍦ㄦ秷鎭槸瀵逛竴鏉¤緝鏃╁彂閫佹秷鎭殑搴旂瓟鏃舵墠琚缃€?    - .. _`CEC-RX-STATUS-ABORTED`:

      - `CEC_RX_STATUS_ABORTED`
      - 0x08
      - 绛夊緟涓€鏉¤緝鏃╁彂閫佹秷鎭殑搴旂瓟琚腑姝紝鍘熷洜鏄?HDMI 绾跨紗琚柇寮€銆?	閫傞厤鍣ㄨ鍙栨秷閰嶇疆锛屾垨绛夊緟搴旂瓟鐨?CEC_TRANSMIT <CEC_RECEIVE> 琚腑鏂€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
ioctl CEC_RECEIVE <CEC_RECEIVE> 鍙互杩斿洖浠ヤ笅閿欒鐮侊細

EAGAIN
    鎺ユ敹闃熷垪涓病鏈夋秷鎭紝涓旀枃浠跺彞鏌勫浜庨潪闃诲妯″紡銆?
ETIMEDOUT
    绛夊緟娑堟伅鏃惰揪鍒颁簡 `timeout`銆?
ERESTARTSYS
    绛夊緟娑堟伅琚腑鏂紙渚嬪琚?Ctrl-C 涓柇锛夈€?
ioctl CEC_TRANSMIT <CEC_TRANSMIT> 鍙互杩斿洖浠ヤ笅閿欒鐮侊細

ENOTTY
    鏈缃?`CEC_CAP_TRANSMIT` 鑳藉姏锛屽洜姝や笉鏀寔姝?ioctl銆?
EPERM
    CEC 閫傞厤鍣ㄦ湭閰嶇疆锛屽嵆浠庢湭璋冪敤杩?ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>锛?    鎴栬€?`CEC_MSG_FL_RAW` 琚竴涓笉鍏锋湁 `CAP_SYS_RAWIO` 鑳藉姏鐨勮繘绋嬩娇鐢ㄣ€?
ENONET
    CEC 閫傞厤鍣ㄦ湭閰嶇疆锛屽嵆璋冪敤杩?ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>锛?    浣嗙墿鐞嗗湴鍧€鏃犳晥锛屽洜姝ゆ病鏈夊０鏄庯紙claim锛夐€昏緫鍦板潃銆?    姝ゆ儏鍐典笅鏈変竴涓緥澶栵紝鍏佽浠庡彂璧疯€?0xf锛?Unregistered'锛夊悜鐩爣 0锛?TV'锛夊彂閫併€?    閭ｇ鎯呭喌涓嬪彂閫佷細鐓у父杩涜銆?
EBUSY
    鍙︿竴涓枃浠跺彞鏌勫浜庣嫭鍗?follower 鎴?initiator 妯″紡锛屾垨鑰呮枃浠跺彞鏌勫浜?    `CEC_MODE_NO_INITIATOR` 妯″紡銆傚綋鍙戦€侀槦鍒楀凡婊℃椂涔熶細杩斿洖姝ら敊璇€?
EINVAL
    struct `cec_msg` 鐨勫唴瀹规棤鏁堛€?
ERESTARTSYS
    绛夊緟涓€娆℃垚鍔熷彂閫佽涓柇锛堜緥濡傝 Ctrl-C 涓柇锛夈€?