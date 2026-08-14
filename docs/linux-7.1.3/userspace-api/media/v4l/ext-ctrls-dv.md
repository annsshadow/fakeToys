

######## 鏁板瓧瑙嗛鎺у埗鍙傝€?


鏁板瓧瑙嗛锛圖igital Video锛夋帶鍒剁被鏃ㄥ湪鎺у埗 `VGA <http://en.wikipedia.org/wiki/Vga>`__銆?
`DVI <http://en.wikipedia.org/wiki/Digital_Visual_Interface>`__
锛圖igital Visual Interface锛屾暟瀛楀彲瑙嗘帴鍙ｏ級銆丠DMI (hdmi) 涓?DisplayPort
(dp) 鐨勬帴鏀跺櫒涓庡彂閫佸櫒銆傝繖浜涙帶鍒堕€氬父棰勬湡涓哄疄鐜颁簡瀹冧滑鐨勬帴鏀跺櫒鎴栧彂閫佸櫒瀛愯澶囩殑绉佹湁鎺у埗锛?
鍥犳鍙毚闇插湪 `/dev/v4l-subdev*` 璁惧鑺傜偣涓娿€?


   娉ㄦ剰锛岃繖浜涜澶囧彲鑳芥湁澶氫釜杈撳叆鎴栬緭鍑?pad锛屽畠浠繛鎺ュ埌渚嬪 HDMI 杩炴帴鍣ㄣ€傚嵆浣垮瓙璁惧
   鍙細浠?鍚戝叾涓竴涓?pad 鎺ユ敹鎴栧彂閫佽棰戯紝鍏朵粬 pad 鍦?EDID锛圗xtended Display
   Identification Data锛屾墿灞曟樉绀鸿瘑鍒暟鎹紝vesaedid锛変笌 HDCP锛圚igh-bandwidth Digital
   Content Protection System锛岄珮甯﹀鏁板瓧鍐呭淇濇姢绯荤粺锛宧dcp锛夊鐞嗘柟闈粛鐒跺彲浠ユ槸娲昏穬鐨勶紝
   浠庤€屼娇璁惧鑳藉鎻愬墠瀹屾垚鐩稿杈冩參鐨?EDID/HDCP 澶勭悊銆傝繖鏍峰氨鍙互鍦ㄨ繛鎺ュ櫒涔嬮棿蹇€熷垏鎹€?

杩欎簺 pad 鍦ㄦ湰鑺傜殑澶氫釜鎺у埗涓互浣嶆帺鐮佺殑褰㈠紡鍑虹幇锛屾瘡涓€浣嶅搴斾竴涓?pad銆備綅 0 瀵瑰簲 pad 0锛?
浣?1 瀵瑰簲 pad 1锛屼緷姝ょ被鎺ㄣ€傛帶鍒剁殑鏈€澶у€煎嵆涓烘湁鏁?pad 鐨勯泦鍚堛€?


## 鏁板瓧瑙嗛鎺у埗 ID


`V4L2_CID_DV_CLASS (class)`
    鏁板瓧瑙嗛绫绘弿杩扮銆?

`V4L2_CID_DV_TX_HOTPLUG (bitmask)`
    璁稿杩炴帴鍣ㄥ甫鏈変竴涓儹鎻掓嫈寮曡剼锛屽綋鏉ヨ嚜婧愮殑 EDID 淇℃伅鍙敤鏃惰寮曡剼涓洪珮鐢靛钩銆傝鎺у埗鏄剧ず鍙戦€佸櫒鎵€瑙佸埌鐨勭儹鎻掓嫈寮曡剼鐘舵€併€傛瘡涓€浣嶅搴斿彂閫佸櫒涓婄殑涓€涓緭鍑?pad銆傚鏋滄煇涓緭鍑?pad 娌℃湁鍏宠仈鐨勭儹鎻掓嫈寮曡剼锛屽垯璇?pad 瀵瑰簲鐨勪綅涓?0銆傝鍙鎺у埗閫傜敤浜?DVI-D銆丠DMI 涓?DisplayPort 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_TX_RXSENSE (bitmask)`
    Rx Sense 鏄 TMDS 鏃堕挓绾夸笂鎷夌數闃荤殑妫€娴嬨€傝繖閫氬父鎰忓懗鐫€鎺ユ敹鍣ㄥ凡杩涘叆/閫€鍑哄緟鏈猴紙鍗冲彂閫佸櫒鍙互鎰熺煡鍒版帴鏀跺櫒宸插噯澶囧ソ鎺ユ敹瑙嗛锛夈€傛瘡涓€浣嶅搴斿彂閫佸櫒涓婄殑涓€涓緭鍑?pad銆傚鏋滄煇涓緭鍑?pad 娌℃湁鍏宠仈鐨?Rx Sense锛屽垯璇?pad 瀵瑰簲鐨勪綅涓?0銆傝鍙鎺у埗閫傜敤浜?DVI-D 涓?HDMI 璁惧銆?

`V4L2_CID_DV_TX_EDID_PRESENT (bitmask)`
    褰撳彂閫佸櫒浠庢帴鏀跺櫒鐪嬪埌鐑彃鎷斾俊鍙锋椂锛屽畠浼氬皾璇曡鍙?EDID銆傝嫢宸茶缃紝鍒欏彂閫佸櫒鑷冲皯宸茶鍙栫涓€鍧楋紙= 128 瀛楄妭锛夈€傛瘡涓€浣嶅搴斿彂閫佸櫒涓婄殑涓€涓緭鍑?pad銆傚鏋滄煇涓緭鍑?pad 涓嶆敮鎸?EDID锛屽垯璇?pad 瀵瑰簲鐨勪綅涓?0銆傝鍙鎺у埗閫傜敤浜?VGA銆丏VI-A/D銆丠DMI 涓?DisplayPort 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_TX_MODE`
    (enum)

enum v4l2_dv_tx_mode -
    HDMI 鍙戦€佸櫒鍙互浠?DVI-D 妯″紡锛堜粎瑙嗛锛夋垨 HDMI 妯″紡锛堣棰?+ 闊抽 + 杈呭姪鏁版嵁锛夊彂閫併€傝鎺у埗閫夋嫨浣跨敤鍝妯″紡锛歏4L2_DV_TX_MODE_DVI_D 鎴?V4L2_DV_TX_MODE_HDMI銆傝鎺у埗閫傜敤浜?HDMI 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_TX_RGB_RANGE`
    (enum)

enum v4l2_dv_rgb_range -
    涓?RGB 杈撳嚭閫夋嫨閲忓寲鑼冨洿銆俈4L2_DV_RANGE_AUTO 閬靛惊瑙嗛鎺ュ彛鏍囧噯涓瀹氱殑 RGB 閲忓寲鑼冨洿锛堝嵆 HDMI 鐨?cea861锛夈€俈4L2_DV_RANGE_LIMITED 涓?V4L2_DV_RANGE_FULL 浼氳鐩栨爣鍑嗭紝浠ュ吋瀹归偅浜涙湭姝ｇ‘瀹炵幇鏍囧噯鐨勬帴鏀剁锛堝浜?HDMI 涓?DVI-D 鑰岃█杩欑鎯呭喌鐩稿綋甯歌锛夈€傚叏鑼冨洿鍏佽浣跨敤鎵€鏈夊彲鑳界殑鍊硷紝鑰岄檺鍒惰寖鍥村皢鑼冨洿璁句负 (16 << (N-8)) - (235 << (N-8))锛屽叾涓?N 鏄瘡涓垎閲忕殑浣嶆暟銆傝鎺у埗閫傜敤浜?VGA銆丏VI-A/D銆丠DMI 涓?DisplayPort 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_TX_IT_CONTENT_TYPE`
    (enum)

enum v4l2_dv_it_content_type -
    閰嶇疆鎵€鍙戦€佽棰戠殑 IT 鍐呭绫诲瀷銆傝淇℃伅浣滀负 AVI InfoFrame 鐨勪竴閮ㄥ垎閫氳繃 HDMI 涓?DisplayPort 杩炴帴鍣ㄥ彂閫併€傛湳璇€淚T Content鈥濈敤浜庢簮鑷绠楁満鐨勫唴瀹癸紝浠ュ尯鍒簬鐢佃骞挎挱鎴栨ā鎷熸簮鐨勫唴瀹广€俥num v4l2_dv_it_content_type 瀹氫箟浜嗗彲鑳界殑鍐呭绫诲瀷锛?


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_DV_IT_CONTENT_TYPE_GRAPHICS`
      - 鍥惧舰鍐呭銆傚儚绱犳暟鎹簲涓嶇粡婊ゆ尝銆佷篃涓嶈繘琛屾ā鎷熼噸寤哄湴浼犻€掋€?
    - - `V4L2_DV_IT_CONTENT_TYPE_PHOTO`
      - 鐓х墖鍐呭銆傚唴瀹规簮鑷暟瀛楅潤鎬佸浘鐗囥€傚唴瀹瑰簲缁忚繃鏈€灏忕缉鏀句笌鐢昏川澧炲己鍦颁紶閫掋€?
    - - `V4L2_DV_IT_CONTENT_TYPE_CINEMA`
      - 褰遍櫌鍐呭銆?
    - - `V4L2_DV_IT_CONTENT_TYPE_GAME`
      - 娓告垙鍐呭銆傚簲浣块煶棰戜笌瑙嗛寤惰繜鏈€灏忓寲銆?
    - - `V4L2_DV_IT_CONTENT_TYPE_NO_ITC`
      - 娌℃湁鍙敤鐨?IT Content 淇℃伅锛屽苟涓?AVI InfoFrame 涓殑 ITC 浣嶈璁句负 0銆?



`V4L2_CID_DV_RX_POWER_PRESENT (bitmask)`
    妫€娴嬫帴鏀跺櫒鏄惁浠庢簮鎺ユ敹鍒扮數婧愶紙渚嬪 HDMI 鍦ㄦ煇鏍瑰紩鑴氫笂鎼哄甫 5V锛夈€傝繖閫氬父鐢ㄤ簬涓哄寘鍚?EDID 淇℃伅鐨?eeprom 渚涚數锛屼娇寰楁簮鍗充娇鍦ㄦ帴鏀跺櫒澶勪簬寰呮満/鏂數鐘舵€佹椂涔熻兘璇诲彇 EDID銆傛瘡涓€浣嶅搴旀帴鏀跺櫒涓婄殑涓€涓緭鍏?pad銆傚鏋滄煇涓緭鍏?pad 鏃犳硶妫€娴嬬數婧愭槸鍚﹀瓨鍦紝鍒欒 pad 瀵瑰簲鐨勪綅涓?0銆傝鍙鎺у埗閫傜敤浜?DVI-D銆丠DMI 涓?DisplayPort 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_RX_RGB_RANGE`
    (enum)

enum v4l2_dv_rgb_range -
    涓?RGB 杈撳叆閫夋嫨閲忓寲鑼冨洿銆俈4L2_DV_RANGE_AUTO 閬靛惊瑙嗛鎺ュ彛鏍囧噯涓瀹氱殑 RGB 閲忓寲鑼冨洿锛堝嵆 HDMI 鐨?cea861锛夈€俈4L2_DV_RANGE_LIMITED 涓?V4L2_DV_RANGE_FULL 浼氳鐩栨爣鍑嗭紝浠ュ吋瀹归偅浜涙湭姝ｇ‘瀹炵幇鏍囧噯鐨勬簮锛堝浜?HDMI 涓?DVI-D 鑰岃█杩欑鎯呭喌鐩稿綋甯歌锛夈€傚叏鑼冨洿鍏佽浣跨敤鎵€鏈夊彲鑳界殑鍊硷紝鑰岄檺鍒惰寖鍥村皢鑼冨洿璁句负 (16 << (N-8)) - (235 << (N-8))锛屽叾涓?N 鏄瘡涓垎閲忕殑浣嶆暟銆傝鎺у埗閫傜敤浜?VGA銆丏VI-A/D銆丠DMI 涓?DisplayPort 杩炴帴鍣ㄣ€?

`V4L2_CID_DV_RX_IT_CONTENT_TYPE`
    (enum)

enum v4l2_dv_it_content_type -
    璇诲彇鎵€鎺ユ敹瑙嗛鐨?IT 鍐呭绫诲瀷銆傝淇℃伅浣滀负 AVI InfoFrame 鐨勪竴閮ㄥ垎閫氳繃 HDMI 涓?DisplayPort 杩炴帴鍣ㄥ彂閫併€傛湳璇€淚T Content鈥濈敤浜庢簮鑷绠楁満鐨勫唴瀹癸紝浠ュ尯鍒簬鐢佃骞挎挱鎴栨ā鎷熸簮鐨勫唴瀹广€傚彲鐢ㄥ唴瀹圭被鍨嬪弬瑙?`V4L2_CID_DV_TX_IT_CONTENT_TYPE`銆?
