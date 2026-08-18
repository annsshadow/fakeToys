

######## 鐩告満鎺у埗鍙傝€?


Camera 绫诲寘鍚敤浜庤澶囨満姊帮紙鎴栫瓑鏁堢殑鏁板瓧锛夌壒鎬х殑鎺у埗锛屼緥濡傚彲鎺ч暅澶存垨浼犳劅鍣ㄣ€?


## 鐩告満鎺у埗 ID


`V4L2_CID_CAMERA_CLASS (class)`
    鐩告満绫绘弿杩扮銆傚璇ユ帶鍒惰皟鐢?
    VIDIOC_QUERYCTRL 灏嗚繑鍥炶鎺у埗绫荤殑鎻忚堪銆?


`V4L2_CID_EXPOSURE_AUTO`
    (enum)

enum v4l2_exposure_auto_type -
    鍚敤瀵规洕鍏夋椂闂村拰/鎴栧厜鍦堝瓟寰勭殑鑷姩璋冩暣銆傚湪鍚敤杩欎簺鐗规€ф椂鎵嬪姩淇敼鏇濆厜鏃堕棿鎴栧厜鍦堝瓟寰勭殑鏁堟灉鏄湭瀹氫箟鐨勶紝椹卞姩搴斿拷鐣ユ绫昏姹傘€傚彲鑳界殑鍊兼湁锛?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_EXPOSURE_AUTO`
      - 鑷姩鏇濆厜鏃堕棿锛岃嚜鍔ㄥ厜鍦堝瓟寰勩€?
    - - `V4L2_EXPOSURE_MANUAL`
      - 鎵嬪姩鏇濆厜鏃堕棿锛屾墜鍔ㄥ厜鍦堛€?
    - - `V4L2_EXPOSURE_SHUTTER_PRIORITY`
      - 鎵嬪姩鏇濆厜鏃堕棿锛岃嚜鍔ㄥ厜鍦堛€?
    - - `V4L2_EXPOSURE_APERTURE_PRIORITY`
      - 鑷姩鏇濆厜鏃堕棿锛屾墜鍔ㄥ厜鍦堛€?



`V4L2_CID_EXPOSURE_ABSOLUTE (integer)`
    鍐冲畾鐩告満浼犳劅鍣ㄧ殑鏇濆厜鏃堕棿銆傛洕鍏夋椂闂村彈甯ч棿闅旈檺鍒躲€傞┍鍔ㄥ簲灏嗘暟鍊艰В閲婁负 100 碌s 鍗曚綅锛屽叾涓€?1 琛ㄧず 1/10000 绉掞紝10000 琛ㄧず 1 绉掞紝100000 琛ㄧず 10 绉掋€?

`V4L2_CID_EXPOSURE_AUTO_PRIORITY (boolean)`
    褰?`V4L2_CID_EXPOSURE_AUTO` 璁句负 `AUTO` 鎴?
    `APERTURE_PRIORITY` 鏃讹紝璇ユ帶鍒跺喅瀹氳澶囨槸鍚﹀彲浠ュ姩鎬佽皟鏁村抚鐜囥€傞粯璁ゆ儏鍐典笅璇ュ姛鑳借绂佺敤
    (0)锛屽抚鐜囧繀椤讳繚鎸佹亽瀹氥€?

`V4L2_CID_AUTO_EXPOSURE_BIAS (integer menu)`
    鍐冲畾鑷姩鏇濆厜琛ュ伩锛屼粎褰?`V4L2_CID_EXPOSURE_AUTO` 鎺у埗璁句负 `AUTO`銆?
    `SHUTTER_PRIORITY` 鎴?`APERTURE_PRIORITY` 鏃舵墠鐢熸晥銆傚畠浠?
    EV 琛ㄧず锛岄┍鍔ㄥ簲灏嗘暟鍊艰В閲婁负 0.001 EV 鍗曚綅锛屽叾涓€?1000 琛ㄧず +1 EV銆?

    澧炲ぇ鏇濆厜琛ュ伩鍊肩浉褰撲簬闄嶄綆鏇濆厜鍊硷紙EV锛夛紝骞朵細澧炲姞鍥惧儚浼犳劅鍣ㄥ鐨勫厜閲忋€傜浉鏈洪€氳繃璋冩暣缁濆鏇濆厜鏃堕棿鍜?鎴栧厜鍦堝瓟寰勬潵鎵ц鏇濆厜琛ュ伩銆?


`V4L2_CID_EXPOSURE_METERING`
    (enum)

enum v4l2_exposure_metering -
    鍐冲畾鐩告満濡備綍娴嬮噺鍙敤浜庡抚鏇濆厜鐨勫厜閲忋€傚彲鑳界殑鍊兼湁锛?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_EXPOSURE_METERING_AVERAGE`
      - 浣跨敤鏉ヨ嚜鏁翠釜甯х殑鍏変俊鎭苟鍙栧钩鍧囷紝瀵规祴鍏夊尯鍩熺殑浠讳綍鐗瑰畾閮ㄥ垎閮戒笉鍔犳潈銆?
    - - `V4L2_EXPOSURE_METERING_CENTER_WEIGHTED`
      - 瀵规潵鑷暣涓抚鐨勫厜淇℃伅鍙栧钩鍧囷紝浣嗕紭鍏堝寰呮祴鍏夊尯鍩熺殑涓績銆?
    - - `V4L2_EXPOSURE_METERING_SPOT`
      - 浠呮祴閲忓抚涓績闈炲父灏忕殑鍖哄煙銆?
    - - `V4L2_EXPOSURE_METERING_MATRIX`
      - 澶氬尯鍩熸祴鍏夈€傚湪甯х殑鑻ュ共涓偣娴嬮噺鍏夊己骞跺悎骞剁粨鏋溿€傚尯鍩熼€夋嫨鍙婂叾鍦ㄨ绠楁渶缁堝€间腑鐨勯噸瑕佹€х殑绠楁硶鍙栧喅浜庡叿浣撹澶囥€?



`V4L2_CID_PAN_RELATIVE (integer)`
    璇ユ帶鍒跺皢鐩告満姘村钩杞姩鎸囧畾閲忋€傚崟浣嶆湭瀹氫箟銆傛鍊间娇鐩告満鍚戝彸绉诲姩锛堜粠涓婃柟鐪嬩负椤烘椂閽堬級锛岃礋鍊煎悜宸︺€傞浂鍊间笉寮曡捣杩愬姩銆傝繖鏄竴涓彧鍐欐帶鍒躲€?

`V4L2_CID_TILT_RELATIVE (integer)`
    璇ユ帶鍒跺皢鐩告満鍨傜洿杞姩鎸囧畾閲忋€傚崟浣嶆湭瀹氫箟銆傛鍊间娇鐩告満鍚戜笂绉诲姩锛岃礋鍊煎悜涓嬨€傞浂鍊间笉寮曡捣杩愬姩銆傝繖鏄竴涓彧鍐欐帶鍒躲€?

`V4L2_CID_PAN_RESET (button)`
    璁剧疆璇ユ帶鍒舵椂锛岀浉鏈烘按骞崇Щ鍔ㄥ埌榛樿浣嶇疆銆?

`V4L2_CID_TILT_RESET (button)`
    璁剧疆璇ユ帶鍒舵椂锛岀浉鏈哄瀭鐩寸Щ鍔ㄥ埌榛樿浣嶇疆銆?

`V4L2_CID_PAN_ABSOLUTE (integer)`
    璇ユ帶鍒跺皢鐩告満姘村钩杞姩鍒版寚瀹氫綅缃€傛鍊间娇鐩告満鍚戝彸绉诲姩锛堜粠涓婃柟鐪嬩负椤烘椂閽堬級锛岃礋鍊煎悜宸︺€傞┍鍔ㄥ簲灏嗘暟鍊艰В閲婁负瑙掔锛屾湁鏁堝€煎湪 -180
    * 3600 鍒?+180 * 3600锛堝惈杈圭晫锛変箣闂淬€?

`V4L2_CID_TILT_ABSOLUTE (integer)`
    璇ユ帶鍒跺皢鐩告満鍨傜洿杞姩鍒版寚瀹氫綅缃€傛鍊间娇鐩告満鍚戜笂绉诲姩锛岃礋鍊煎悜涓嬨€傞┍鍔ㄥ簲灏嗘暟鍊艰В閲?
    涓鸿绉掞紝鏈夋晥鍊煎湪 -180 ** 3600 鍒?+180 ** 3600锛堝惈杈圭晫锛変箣闂淬€?

`V4L2_CID_FOCUS_ABSOLUTE (integer)`
    璇ユ帶鍒跺皢鐩告満鐨勭劍鐐硅缃埌鎸囧畾浣嶇疆銆傚崟浣嶆湭瀹氫箟銆傛鍊煎皢鐒︾偣绉昏繎鐩告満锛岃礋鍊肩Щ鍚戞棤绌疯繙銆?

`V4L2_CID_FOCUS_RELATIVE (integer)`
    璇ユ帶鍒跺皢鐩告満鐨勭劍鐐圭Щ鍔ㄦ寚瀹氶噺銆傚崟浣嶆湭瀹氫箟銆傛鍊煎皢鐒︾偣绉昏繎鐩告満锛岃礋鍊肩Щ鍚戞棤绌疯繙銆傝繖鏄竴涓彧鍐欐帶鍒躲€?

`V4L2_CID_FOCUS_AUTO (boolean)`
    鍚敤杩炵画鑷姩瀵圭劍璋冩暣銆傚湪鍚敤璇ョ壒鎬ф椂鎵嬪姩瀵圭劍璋冩暣鐨勬晥鏋滄槸鏈畾涔夌殑锛岄┍鍔ㄥ簲蹇界暐姝ょ被璇锋眰銆?

`V4L2_CID_AUTO_FOCUS_START (button)`
    鍚姩鍗曟鑷姩瀵圭劍杩囩▼銆傚綋 `V4L2_CID_FOCUS_AUTO` 璁句负 `TRUE` (1) 鏃惰缃鎺у埗鐨勬晥鏋滄槸鏈畾涔夌殑锛岄┍鍔ㄥ簲蹇界暐姝ょ被璇锋眰銆?

`V4L2_CID_AUTO_FOCUS_STOP (button)`
    涓鐢?`V4L2_CID_AUTO_FOCUS_START` 鎺у埗鍚姩鐨勮嚜鍔ㄥ鐒︺€備粎褰撹繛缁嚜鍔ㄥ鐒﹁绂佺敤锛堝嵆 `V4L2_CID_FOCUS_AUTO` 鎺у埗璁句负 `FALSE` (0)锛夋椂鎵嶇敓鏁堛€?


`V4L2_CID_AUTO_FOCUS_STATUS (bitmask)`
    鑷姩瀵圭劍鐘舵€併€傝繖鏄竴涓彧璇绘帶鍒躲€?

    璁剧疆 `V4L2_CID_3A_LOCK` 鎺у埗鐨?`V4L2_LOCK_FOCUS` 閿佷綅鍙兘浼氬仠姝㈠
    `V4L2_CID_AUTO_FOCUS_STATUS` 鎺у埗鍊肩殑鏇存柊銆?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_AUTO_FOCUS_STATUS_IDLE`
      - 鑷姩瀵圭劍鏈縺娲汇€?
    - - `V4L2_AUTO_FOCUS_STATUS_BUSY`
      - 鑷姩瀵圭劍杩涜涓€?
    - - `V4L2_AUTO_FOCUS_STATUS_REACHED`
      - 宸茶揪鍒扮劍鐐广€?
    - - `V4L2_AUTO_FOCUS_STATUS_FAILED`
      - 鑷姩瀵圭劍澶辫触锛屽湪搴旂敤绋嬪簭鎵ц鍙︿竴涓姩浣滀箣鍓嶏紝椹卞姩涓嶄細浠庤鐘舵€佽浆鎹€?



`V4L2_CID_AUTO_FOCUS_RANGE`
    (enum)

enum v4l2_auto_focus_range -
    鍐冲畾闀滃ご鍙皟鐨勮嚜鍔ㄥ鐒﹁窛绂昏寖鍥淬€?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_AUTO_FOCUS_RANGE_AUTO`
      - 鐩告満鑷姩閫夋嫨瀵圭劍鑼冨洿銆?
    - - `V4L2_AUTO_FOCUS_RANGE_NORMAL`
      - 姝ｅ父璺濈鑼冨洿锛屼负鑾峰緱鏈€浣宠嚜鍔ㄥ鐒︽€ц兘鑰屽彈闄愩€?
    - - `V4L2_AUTO_FOCUS_RANGE_MACRO`
      - 寰窛锛堢壒鍐欙級鑷姩瀵圭劍銆傜浉鏈哄皢浣跨敤鍏舵渶灏忓彲鑳借窛绂昏繘琛岃嚜鍔ㄥ鐒︺€?
    - - `V4L2_AUTO_FOCUS_RANGE_INFINITY`
      - 闀滃ご璁句负瀵圭劍鏃犵┓杩滃鐨勭墿浣撱€?



`V4L2_CID_ZOOM_ABSOLUTE (integer)`
    浠ョ粷瀵瑰€兼寚瀹氱墿闀滅劍璺濄€傚彉鐒﹀崟浣嶇敱椹卞姩鐗瑰畾锛屽叾鍊煎簲涓烘鏁存暟銆?

`V4L2_CID_ZOOM_RELATIVE (integer)`
    鐩稿浜庡綋鍓嶅€兼寚瀹氱墿闀滅劍璺濄€傛鍊间娇鍙樼劍闀滃ご缁勭Щ鍚戦暱鐒︽柟鍚戯紝璐熷€肩Щ鍚戝箍瑙掓柟鍚戙€傚彉鐒﹀崟浣嶇敱椹卞姩鐗瑰畾銆傝繖鏄竴涓彧鍐欐帶鍒躲€?

`V4L2_CID_ZOOM_CONTINUOUS (integer)`
    浠ユ寚瀹氶€熷害绉诲姩鍙樼劍闀滃ご缁勶紝鐩村埌杈惧埌鐗╃悊璁惧鏋侀檺鎴栨敹鍒版槑纭殑鍋滄绉诲姩璇锋眰銆傛鍊间娇鍙樼劍闀滃ご缁勭Щ鍚戦暱鐒︽柟鍚戙€傞浂鍊煎仠姝㈠彉鐒﹂暅澶寸粍鐨勮繍鍔ㄣ€傝礋鍊间娇鍙樼劍闀滃ご缁勭Щ鍚戝箍瑙掓柟鍚戙€傚彉鐒﹂€熷害鍗曚綅鐢遍┍鍔ㄧ壒瀹氥€?

`V4L2_CID_IRIS_ABSOLUTE (integer)`
    璇ユ帶鍒跺皢鐩告満鐨勫厜鍦堣缃埌鎸囧畾鍊笺€傚崟浣嶆湭瀹氫箟銆傝緝澶х殑鍊间娇鍏夊湀寮€寰楁洿澶э紝杈冨皬鐨勫€间娇鍏跺叧闂€?

`V4L2_CID_IRIS_RELATIVE (integer)`
    璇ユ帶鍒舵寜鎸囧畾閲忎慨鏀圭浉鏈虹殑鍏夊湀銆傚崟浣嶆湭瀹氫箟銆傛鍊间娇鍏夊湀鍐嶅紑澶т竴姝ワ紝璐熷€煎啀鍏抽棴涓€姝ャ€傝繖鏄竴涓彧鍐欐帶鍒躲€?

`V4L2_CID_PRIVACY (boolean)`
    闃绘鐩告満鑾峰彇瑙嗛銆傚綋璇ユ帶鍒惰涓?`TRUE` (1) 鏃讹紝鐩告満鏃犳硶鎹曡幏浠讳綍鍥惧儚銆傚己鍒堕殣绉佺殑甯歌鎵嬫鏄紶鎰熷櫒鐨勬満姊伴伄鍏変互鍙婂浐浠跺浘鍍忓鐞嗭紝浣嗚澶囦笉闄愪簬杩欎簺鏂规硶銆傚疄鐜?privacy 鎺у埗鐨勮澶囧繀椤绘敮鎸佽璁块棶锛屽苟鍙互鏀寔鍐欒闂€?

`V4L2_CID_BAND_STOP_FILTER (integer)`
    寮€鍚垨鍏抽棴鐩告満浼犳劅鍣ㄧ殑甯﹂樆婊ゆ尝鍣紝鎴栨寚瀹氬叾寮哄害銆傛绫诲甫闃绘护娉㈠櫒鍙敤浜庝緥濡傛护闄よ崸鍏夌伅鎴愬垎銆?


`V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`
    (enum)

enum v4l2_auto_n_preset_white_balance -
    灏嗙櫧骞宠　璁句负鑷姩銆佹墜鍔ㄦ垨棰勮銆傞璁惧喅瀹氬厜鐨勮壊娓╋紝浣滀负鐩告満杩涜鐧藉钩琛¤皟鏁寸殑鎻愮ず锛屼粠鑰岃幏寰楁渶鍑嗙‘鐨勮壊褰╄〃鐜般€備互涓嬬櫧骞宠　棰勮鎸夎壊娓╅€掑椤哄簭鎺掑垪銆?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_WHITE_BALANCE_MANUAL`
      - 鎵嬪姩鐧藉钩琛°€?
    - - `V4L2_WHITE_BALANCE_AUTO`
      - 鑷姩鐧藉钩琛¤皟鏁淬€?
    - - `V4L2_WHITE_BALANCE_INCANDESCENT`
      - 鐧界偨鐏紙閽ㄤ笣鐏級鐓ф槑鐨勭櫧骞宠　璁剧疆銆傚畠閫氬父浼氫娇棰滆壊鍋忓喎锛屽ぇ绾﹀搴斾簬
        2500...3500 K 鑹叉俯鑼冨洿銆?
    - - `V4L2_WHITE_BALANCE_FLUORESCENT`
      - 鑽у厜鐏収鏄庣殑鐧藉钩琛￠璁俱€傚ぇ绾﹀搴斾簬 4000...5000 K 鑹叉俯銆?
    - - `V4L2_WHITE_BALANCE_FLUORESCENT_H`
      - 浣跨敤璇ヨ缃椂锛岀浉鏈哄皢琛ュ伩鑽у厜鐏?H 鐓ф槑銆?
    - - `V4L2_WHITE_BALANCE_HORIZON`
      - 鍦板钩绾挎棩鍏夌殑鐧藉钩琛¤缃€傚ぇ绾﹀搴斾簬 5000 K 鑹叉俯銆?
    - - `V4L2_WHITE_BALANCE_DAYLIGHT`
      - 鏃ュ厜锛堟櫞鏈楀ぉ绌猴級鐨勭櫧骞宠　棰勮銆傚ぇ绾﹀搴斾簬 5000...6500 K 鑹叉俯銆?
    - - `V4L2_WHITE_BALANCE_FLASH`
      - 浣跨敤璇ヨ缃椂锛岀浉鏈哄皢琛ュ伩闂厜鐏収鏄庛€傚畠浣块鑹茬暐寰亸鏆栵紝澶х害瀵瑰簲浜?
        5000...5500 K 鑹叉俯銆?
    - - `V4L2_WHITE_BALANCE_CLOUDY`
      - 涓害闃村ぉ鐨勭櫧骞宠　棰勮銆傝閫夐」澶х害瀵瑰簲浜?6500...8000 K 鑹叉俯鑼冨洿銆?
    - - `V4L2_WHITE_BALANCE_SHADE`
      - 闃村奖鎴栨祿瀵嗛槾澶╃殑鐧藉钩琛￠璁俱€傚ぇ绾﹀搴斾簬 9000...10000 K 鑹叉俯銆?



`V4L2_CID_WIDE_DYNAMIC_RANGE (boolean)`
    鍚敤鎴栫鐢ㄧ浉鏈虹殑瀹藉姩鎬佽寖鍥寸壒鎬с€傝鐗规€у厑璁稿湪鍦烘櫙鍐呭厜鐓у己搴﹀彉鍖栨樉钁楋紙鍗冲悓鏃跺瓨鍦ㄩ潪甯告殫鍜岄潪甯镐寒鐨勫尯鍩燂級鐨勬儏鍐典笅鑾峰緱娓呮櫚鐨勫浘鍍忋€傚畠鏈€甯歌鐨勬槸閫氳繃鍚堝苟涓ゅ抚鏇濆厜鏃堕棿涓嶅悓鐨勫悗缁抚鏉ュ疄鐜般€?[#f1]_


`V4L2_CID_IMAGE_STABILIZATION (boolean)`
    鍚敤鎴栫鐢ㄥ浘鍍忕ǔ瀹氥€?

`V4L2_CID_ISO_SENSITIVITY (integer menu)`
    鍐冲畾鍥惧儚浼犳劅鍣ㄧ殑 ISO 绛夋晥鍊硷紝琛ㄧず浼犳劅鍣ㄥ鍏夌殑鐏垫晱搴︺€傝繖浜涙暟瀛楁寜绠楁湳鏍囧害琛ㄧず锛岄伒寰?iso12232 鏍囧噯锛屽叾涓紶鎰熷櫒鐏垫晱搴﹀姞鍊嶇敱鏁板€?ISO 鍊煎姞鍊嶈〃绀恒€傚簲鐢ㄧ▼搴忓簲灏嗘暟鍊艰В閲婁负鏍囧噯 ISO 鍊间箻浠?1000锛屼緥濡傛帶鍒跺€?800 琛ㄧず ISO 0.8銆傞┍鍔ㄩ€氬父鍙敮鎸佹爣鍑?ISO 鍊肩殑涓€涓瓙闆嗐€傚湪
    `V4L2_CID_ISO_SENSITIVITY_AUTO` 鎺у埗璁句负 `V4L2_CID_ISO_SENSITIVITY_MANUAL`
    浠ュ鐨勫€兼椂璁剧疆璇ユ帶鍒剁殑鏁堟灉鏄湭瀹氫箟鐨勶紝椹卞姩搴斿拷鐣ユ绫昏姹傘€?


`V4L2_CID_ISO_SENSITIVITY_AUTO`
    (enum)

enum v4l2_iso_sensitivity_type -
    鍚敤鎴栫鐢ㄨ嚜鍔?ISO 鐏垫晱搴﹁皟鏁淬€?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_ISO_SENSITIVITY_MANUAL`
      - 鎵嬪姩 ISO 鐏垫晱搴︺€?
    - - `V4L2_CID_ISO_SENSITIVITY_AUTO`
      - 鑷姩 ISO 鐏垫晱搴﹁皟鏁淬€?



`V4L2_CID_SCENE_MODE`
    (enum)

enum v4l2_scene_mode -
    璇ユ帶鍒跺厑璁搁€夋嫨鍦烘櫙绋嬪簭锛屽嵆鐩告満閽堝甯歌鎷嶆憚鍦烘櫙浼樺寲鐨勮嚜鍔ㄦā寮忋€傚湪杩欎簺妯″紡涓嬶紝鐩告満鍐冲畾鏈€浣虫洕鍏夈€佸厜鍦堛€佸鐒︺€佹祴鍏夈€佺櫧骞宠　鍜岀瓑鏁堢伒鏁忓害銆傝繖浜涘弬鏁扮殑鎺у埗鍙楀満鏅ā寮忔帶鍒跺奖鍝嶃€傛瘡绉嶆ā寮忎笅鐨勭‘鍒囪涓哄彇鍐充簬鐩告満瑙勬牸銆?

    褰撲笉浣跨敤鍦烘櫙妯″紡鐗规€ф椂锛屽簲灏嗘鎺у埗璁句负 `V4L2_SCENE_MODE_NONE`锛屼互纭繚鍏朵粬鍙兘鐩稿叧鐨勬帶鍒跺彲璁块棶銆傚畾涔変簡浠ヤ笅鍦烘櫙绋嬪簭锛?


    \small



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_SCENE_MODE_NONE`
      - 鍦烘櫙妯″紡鐗规€ц绂佺敤銆?
    - - `V4L2_SCENE_MODE_BACKLIGHT`
      - 鑳屽厜銆傚綋鍏夋潵鑷富浣撹儗鍚庢椂琛ュ伩鏆楅儴闃村奖锛屼篃浼氳嚜鍔ㄥ紑鍚棯鍏夌伅銆?
    - - `V4L2_SCENE_MODE_BEACH_SNOW`
      - 娴锋哗鍜岄洩鍦般€傝妯″紡琛ュ伩鍏ㄧ櫧鎴栨槑浜殑鍦烘櫙锛屽綋鐩告満鑷姩鏇濆厜鍩轰簬骞冲潎鍦烘櫙浜害鏃讹紝杩欑被鍦烘櫙寰€寰€鏄惧緱鐏版殫涓斿姣斿害浣庛€備负琛ュ伩锛岃妯″紡鑷姩鐣ュ井杩囨洕甯с€傜櫧骞宠　涔熷彲鑳借璋冩暣锛屼互琛ュ伩鍙嶅皠鐨勯洩鐪嬭捣鏉ュ亸钃濊€岄潪鐧借壊杩欎竴浜嬪疄銆?
    - - `V4L2_SCENE_MODE_CANDLELIGHT`
      - 鐑涘厜銆傜浉鏈洪€氬父浼氭彁楂?ISO 鐏垫晱搴﹀苟闄嶄綆蹇棬閫熷害銆傝妯″紡琛ュ伩鍦烘櫙涓浉瀵归潬杩戠殑涓讳綋銆備负淇濈暀鍏夌嚎姘涘洿锛岄棯鍏夌伅琚鐢ㄣ€?
    - - `V4L2_SCENE_MODE_DAWN_DUSK`
      - 榛庢槑鍜岄粍鏄忋€備繚鐣欓粍鏄忓墠鍜岄粠鏄庡悗浣庤嚜鐒跺厜涓嬬湅鍒扮殑棰滆壊銆傜浉鏈哄彲鑳戒細鍏抽棴闂厜鐏紝骞惰嚜鍔ㄥ鐒﹀埌鏃犵┓杩溿€傚畠閫氬父浼氭彁楂橀ケ鍜屽害骞堕檷浣庡揩闂ㄩ€熷害銆?
    - - `V4L2_SCENE_MODE_FALL_COLORS`
      - 绉嬭壊銆傛彁楂橀ケ鍜屽害骞惰皟鏁寸櫧骞宠　浠ュ寮鸿壊褰┿€傜鍙剁収鐗囦細寰楀埌楗卞拰鐨勭孩鑹插拰榛勮壊銆?
    - - `V4L2_SCENE_MODE_FIREWORKS`
      - 鐑熻姳銆備娇鐢ㄩ暱鏇濆厜鏃堕棿鏉ユ崟鎹夌儫鑺卞悜澶栨墿鏁ｇ殑鍏夌垎鍙戙€傜浉鏈哄彲鑳戒細璋冪敤鍥惧儚绋冲畾銆?
    - - `V4L2_SCENE_MODE_LANDSCAPE`
      - 椋庢櫙銆傜浉鏈轰細閫夋嫨灏忓厜鍦堜互鎻愪緵娣辨櫙娣憋紝骞朵娇鐢ㄩ暱鏇濆厜鏃堕暱浠ュ府鍔╁湪鏄忔殫鍏夌嚎涓嬫崟鎹夌粏鑺傘€傚鐒﹀浐瀹氬湪鏃犵┓杩溿€傞€傚悎杩滄櫙鍜屽箍闃旈鏅€?
    - - `V4L2_SCENE_MODE_NIGHT`
      - 澶滈棿锛屼篃绉板闂撮鏅€備负浣庡厜鏉′欢璁捐锛屽畠鍦ㄤ繚鐣欐殫閮ㄧ粏鑺傜殑鍚屾椂涓嶄細浣挎槑浜墿浣撹繃鏇濄€傜浉鏈洪€氬父灏嗚嚜韬涓轰腑鍒伴珮 ISO 鐏垫晱搴︼紝閰嶅悎鐩稿杈冮暱鐨勬洕鍏夋椂闂达紝骞跺叧闂棯鍏夌伅銆傚洜姝わ紝鍥惧儚鍣偣浼氬鍔狅紝骞跺彲鑳藉嚭鐜板浘鍍忔ā绯娿€?
    - - `V4L2_SCENE_MODE_PARTY_INDOOR`
      - 鑱氫細鍜屽鍐呫€備负鎹曟崏鐢卞鍐呰儗鏅収鏄庝互鍙婇棯鍏夌伅鍏卞悓鐓ф槑鐨勫鍐呭満鏅€岃璁°€傜浉鏈洪€氬父浼氭彁楂?ISO 鐏垫晱搴︼紝骞朵负浣庡厜鏉′欢璋冩暣鏇濆厜銆?
    - - `V4L2_SCENE_MODE_PORTRAIT`
      - 浜哄儚銆傜浉鏈鸿皟鏁村厜鍦堜互鍑忓皬鏅繁锛屾湁鍔╀簬灏嗕富浣撲粠骞虫粦鐨勮儗鏅腑鍒嗙鍑烘潵銆傚ぇ澶氭暟鐩告満浼氳瘑鍒満鏅腑浜鸿劯骞跺鍏跺鐒︺€傝壊璋冭璋冩暣浠ュ寮鸿偆鑹层€傞棯鍏夌伅寮哄害閫氬父闄嶄綆銆?
    - - `V4L2_SCENE_MODE_SPORTS`
      - 杩愬姩銆傛樉钁楁彁楂?ISO 骞朵娇鐢ㄥ揩閫熷揩闂ㄩ€熷害浠ュ喕缁撳揩閫熺Щ鍔ㄤ富浣撶殑鍔ㄤ綔銆傝妯″紡涓嬪彲鑳戒細鐪嬪埌澧炲姞鐨勫浘鍍忓櫔鐐广€?
    - - `V4L2_SCENE_MODE_SUNSET`
      - 鏃ヨ惤銆備繚鐣欏湪鏃ヨ惤鍜屾棩鍑轰腑鐪嬪埌鐨勬繁娌夎壊璋冦€傚畠鎻愰珮楗卞拰搴︺€?
    - - `V4L2_SCENE_MODE_TEXT`
      - 鏂囨湰銆傚畠搴旂敤棰濆鐨勫姣斿害鍜岄攼搴︼紝閫氬父鏄竴绉嶄负鍙鎬т紭鍖栫殑榛戠櫧妯″紡銆傝嚜鍔ㄥ鐒﹀彲鑳藉垏鎹㈠埌鐗瑰啓妯″紡锛岃璁剧疆涔熷彲鑳芥秹鍙婁竴浜涢暅澶寸暩鍙樻牎姝ｃ€?

    \normalsize


`V4L2_CID_3A_LOCK (bitmask)`
    璇ユ帶鍒堕攣瀹氭垨瑙ｉ攣鑷姩瀵圭劍銆佹洕鍏夊拰鐧藉钩琛°€傞€氳繃灏嗙浉搴旂殑閿佷綅璁句负 1锛屽彲浠ョ嫭绔嬪湴鏆傚仠鑷姩璋冩暣銆傜劧鍚庣浉鏈轰繚鐣欒繖浜涜缃紝鐩村埌閿佷綅琚竻闄ゃ€傚畾涔変簡浠ヤ笅閿佷綅锛?

    褰撴煇涓粰瀹氱畻娉曟湭鍚敤鏃讹紝椹卞姩搴斿拷鐣ラ攣瀹氬畠鐨勮姹傦紝骞朵笖涓嶅簲杩斿洖閿欒銆備緥濡傦紝褰?
    `V4L2_CID_AUTO_WHITE_BALANCE` 鎺у埗璁句负 `FALSE` 鏃讹紝搴旂敤绋嬪簭璁剧疆
    `V4L2_LOCK_WHITE_BALANCE` 浣嶃€傝鎺у埗鐨勫€煎彲鑳借鏇濆厜銆佺櫧骞宠　鎴栧鐒︽帶鍒舵敼鍙樸€?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_LOCK_EXPOSURE`
      - 鑷姩鏇濆厜璋冩暣閿併€?
    - - `V4L2_LOCK_WHITE_BALANCE`
      - 鑷姩鐧藉钩琛¤皟鏁撮攣銆?
    - - `V4L2_LOCK_FOCUS`
      - 鑷姩瀵圭劍閿併€?



`V4L2_CID_PAN_SPEED (integer)`
    璇ユ帶鍒朵互鐗瑰畾閫熷害灏嗙浉鏈烘按骞宠浆鍔ㄣ€傚崟浣嶆湭瀹氫箟銆傛鍊间娇鐩告満鍚戝彸绉诲姩锛堜粠涓婃柟鐪嬩负椤烘椂閽堬級锛岃礋鍊煎悜宸︺€傞浂鍊煎仠姝㈡鍦ㄨ繘琛岀殑杩愬姩锛堝鏋滄湁鐨勮瘽锛夛紝鍚﹀垯鏃犳晥鏋溿€?

`V4L2_CID_TILT_SPEED (integer)`
    璇ユ帶鍒朵互鎸囧畾閫熷害灏嗙浉鏈哄瀭鐩磋浆鍔ㄣ€傚崟浣嶆湭瀹氫箟銆傛鍊间娇鐩告満鍚戜笂绉诲姩锛岃礋鍊煎悜涓嬨€傞浂鍊煎仠姝㈡鍦ㄨ繘琛岀殑杩愬姩锛堝鏋滄湁鐨勮瘽锛夛紝鍚﹀垯鏃犳晥鏋溿€?


`V4L2_CID_CAMERA_ORIENTATION (menu)`
    璇ュ彧璇绘帶鍒堕€氳繃鎶ュ憡鐩告満鎵€瀹夎璁惧鐨勫畨瑁呬綅缃潵鎻忚堪鐩告満鏈濆悜銆傛帶鍒跺€兼槸鎭掑畾鐨勶紝涓嶈兘琚蒋浠朵慨鏀广€傝鎺у埗瀵逛簬鍏锋湁鏄庣‘瀹氫箟鏈濆悜鐨勮澶囷紙渚嬪鎵嬫満銆佺瑪璁版湰鐢佃剳鍜屼究鎼鸿澶囷級鐗瑰埆鏈夋剰涔夛紝鍥犱负璇ユ帶鍒惰〃绀轰负鐩稿浜庤澶囬鏈熶娇鐢ㄦ湞鍚戠殑浣嶇疆銆備緥濡傦紝瀹夎鍦ㄦ墜鏈恒€佸钩鏉挎垨绗旇鏈數鑴戠敤鎴蜂晶闈㈢殑鐩告満琚О涓哄叿鏈?`V4L2_CAMERA_ORIENTATION_FRONT` 鏈濆悜锛岃€屽畨瑁呭湪姝ｉ潰鐩稿弽涓€渚х殑鐩告満琚О涓哄叿鏈?`V4L2_CAMERA_ORIENTATION_BACK` 鏈濆悜銆傛湭鐩存帴闄勬帴鍒拌澶囥€佹垨浠ュ厑璁稿叾鑷敱绉诲姩鐨勬柟寮忛檮鎺ョ殑鐩告満浼犳劅鍣紙渚嬪缃戠粶鎽勫儚澶村拰鏁扮爜鐩告満锛夎绉颁负鍏锋湁 `V4L2_CAMERA_ORIENTATION_EXTERNAL` 鏈濆悜銆?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CAMERA_ORIENTATION_FRONT`
      - 鐩告満鏈濆悜璁惧鐨勭敤鎴蜂晶闈€?
    - - `V4L2_CAMERA_ORIENTATION_BACK`
      - 鐩告満鏈濆悜璁惧鐨勮儗闈€?
    - - `V4L2_CAMERA_ORIENTATION_EXTERNAL`
      - 鐩告満鏈洿鎺ラ檮鎺ュ埌璁惧锛屼笖鍙嚜鐢辩Щ鍔ㄣ€?



`V4L2_CID_CAMERA_SENSOR_ROTATION (integer)`
    璇ュ彧璇绘帶鍒舵弿杩板湪鍥惧儚鎹曡幏鍒板唴瀛樺悗锛屼负琛ュ伩鐩告満浼犳劅鍣ㄥ畨瑁呮棆杞€岄渶浠ラ€嗘椂閽堟柟鍚戞柦鍔犵殑鏃嬭浆鏍℃瑙掑害銆?

    鏈夊叧浼犳劅鍣ㄥ畨瑁呮棆杞殑绮剧‘瀹氫箟锛岃鍙傞槄璁惧鏍戠粦瀹氭枃浠?'video-interfaces.txt' 涓 'rotation' 灞炴€х殑璇﹀敖鎻忚堪銆?

    涓嬮潰鎶ュ憡浜嗗嚑涓ず渚嬶紝浣跨敤涓€鏉′粠宸﹀悜鍙虫父鍔ㄧ殑椴ㄩ奔
```

                 0               X-axis
               0 +------------------------------------->
                 !
                 !
                 !
                 !           |\____)\___
                 !           ) _____  __`<
                 !           |/     )/
                 !
                 !
                 !
                 V
               Y-axis

    Example one - Webcam

    Assuming you can bring your laptop with you while swimming with sharks,
    the camera module of the laptop is installed on the user facing part of a
    laptop screen casing, and is typically used for video calls. The captured
    images are meant to be displayed in landscape mode (width > height) on the
    laptop screen.

    The camera is typically mounted upside-down to compensate the lens optical
    inversion effect. In this case the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION control is 0, no rotation is required to
    display images correctly to the user.

    If the camera sensor is not mounted upside-down it is required to compensate
    the lens optical inversion effect and the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION control is 180 degrees, as images will
    result rotated when captured to memory. ::

                 +--------------------------------------+
                 !                                      !
                 !                                      !
                 !                                      !
                 !              __/(_____/|             !
                 !            >.___  ____ (             !
                 !                 \(    \|             !
                 !                                      !
                 !                                      !
                 !                                      !
                 +--------------------------------------+

    A software rotation correction of 180 degrees has to be applied to correctly
    display the image on the user screen. ::

                 +--------------------------------------+
                 !                                      !
                 !                                      !
                 !                                      !
                 !             |\____)\___              !
                 !             ) _____  __`<            !
                 !             |/     )/                !
                 !                                      !
                 !                                      !
                 !                                      !
                 +--------------------------------------+

    Example two - Phone camera

    It is more handy to go and swim with sharks with only your mobile phone
    with you and take pictures with the camera that is installed on the back
    side of the device, facing away from the user. The captured images are meant
    to be displayed in portrait mode (height > width) to match the device screen
    orientation and the device usage orientation used when taking the picture.

    The camera sensor is typically mounted with its pixel array longer side
    aligned to the device longer side, upside-down mounted to compensate for
    the lens optical inversion effect.

    The images once captured to memory will be rotated and the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION will report a 90 degree rotation. ::


                 +-------------------------------------+
                 |                 _ _                 |
                 |                \   /                |
                 |                 | |                 |
                 |                 | |                 |
                 |                 |  >                |
                 |                <  |                 |
                 |                 | |                 |
                 |                   .                 |
                 |                  V                  |
                 +-------------------------------------+

    A correction of 90 degrees in counter-clockwise direction has to be
    applied to correctly display the image in portrait mode on the device
    screen. ::

                          +--------------------+
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |   |\____)\___      |
                          |   ) _____  __`<    |
                          |   |/     )/        |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          +--------------------+


```
   璇ユ帶鍒舵湭鏉ュ彲鑳戒細鏀逛负鑿滃崟鎺у埗锛屽鏋滈渶瑕佹洿澶氶€夐」鐨勮瘽銆?

`V4L2_CID_HDR_SENSOR_MODE (menu)`
    鏇存敼浼犳劅鍣?HDR 妯″紡銆侶DR 鍥惧儚鏄€氳繃浣跨敤涓や釜涓嶅悓鐨勬洕鍏夊懆鏈熷悎骞跺悓涓€鍦烘櫙鐨勪袱娆℃崟鑾疯幏寰楃殑銆侶DR 妯″紡鎻忚堪浜嗚繖涓や釜鎹曡幏鍦ㄤ紶鎰熷櫒涓悎骞剁殑鏂瑰紡銆?

    鐢变簬姣忕浼犳劅鍣ㄧ殑妯″紡涓嶅悓锛岃彍鍗曢」涓嶇敱璇ユ帶鍒舵爣鍑嗗寲锛岃€岀暀缁欑紪绋嬭€呭喅瀹氥€?
