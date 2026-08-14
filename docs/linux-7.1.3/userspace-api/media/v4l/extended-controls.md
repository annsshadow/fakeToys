
######## 鎵╁睍鎺у埗 API


## 绠€浠?

鏈€鍒濊璁＄殑鎺у埗鏈哄埗鏃ㄥ湪鐢ㄤ簬鐢ㄦ埛璁剧疆锛堜寒搴︺€侀ケ鍜屽害绛夛級銆傜劧鑰岋紝瀹炶返璇佹槑瀹?瀵逛簬瀹炵幇鏇村鏉傜殑椹卞姩 API 涔熸槸涓€涓潪甯告湁鐢ㄧ殑妯″瀷锛屽湪杩欑 API 涓瘡涓┍鍔?鍙疄鐜版洿澶х殑 API 鐨勪竴涓瓙闆嗐€?
MPEG 缂栫爜 API 鏄璁″拰瀹炵幇杩欎竴鎵╁睍鎺у埗鏈哄埗鐨勯┍鍔ㄥ姏锛歁PEG 鏍囧噯鐩稿綋搴炲ぇ锛?鑰屽綋鍓嶅彈鏀寔鐨勭‖浠?MPEG 缂栫爜鍣ㄥ悇鑷彧瀹炵幇浜嗚鏍囧噯鐨勪竴涓瓙闆嗐€傛澶栵紝璁稿
鍏充簬濡備綍灏嗚棰戠紪鐮佷负 MPEG 娴佺殑鐩稿叧鍙傛暟鐗瑰畾浜?MPEG 缂栫爜鑺墖锛屽洜涓?MPEG
鏍囧噯鍙畾涔変簡鏈€缁?MPEG 娴佺殑鏍煎紡锛岃€岄潪瑙嗛瀹為檯琚紪鐮佷负璇ユ牸寮忕殑鏂瑰紡銆?
閬楁喚鐨勬槸锛屽師濮嬬殑鎺у埗 API 缂哄皯杩欎簺鏂扮敤閫旀墍闇€鐨勪竴浜涚壒鎬э紝鍥犳瀹冭鎵╁睍涓?锛堝懡鍚嶅苟涓嶅崄鍒嗘湁鍒涙剰鐨勶級鎵╁睍鎺у埗 API銆?
灏界 MPEG 缂栫爜 API 鏄娇鐢ㄦ墿灞曟帶鍒?API 鐨勯娆″皾璇曪紝濡備粖涔熷嚭鐜颁簡鍏朵粬绫诲埆
鐨勬墿灞曟帶鍒讹紝渚嬪 Camera Controls锛堟憚鍍忓ご鎺у埗锛夊拰 FM Transmitter Controls
锛團M 鍙戝皠鍣ㄦ帶鍒讹級銆傛墿灞曟帶鍒?API 浠ュ強鎵€鏈夋墿灞曟帶鍒剁被鍒湪涓嬫枃涓弿杩般€?

## 鎵╁睍鎺у埗 API


鏈変笁涓柊鐨?ioctl 鍙敤锛歏IDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>銆?VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鍜?VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>銆傝繖浜?ioctl 浣滅敤浜庢帶鍒舵暟缁?锛堢浉瀵逛簬浣滅敤浜庡崟涓帶鍒剁殑 VIDIOC_G_CTRL <VIDIOC_G_CTRL> 鍜?VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl锛夈€傝繖鏄繀闇€鐨勶紝鍥犱负甯稿父闇€瑕佸師瀛愬湴
涓€娆℃€ф洿鏀瑰涓帶鍒躲€?
姣忎釜鏂扮殑 ioctl 閮芥湡鏈涗竴涓寚鍚?struct `v4l2_ext_controls` 鐨勬寚閽堛€傝缁撴瀯
鍖呭惈涓€涓寚鍚戞帶鍒舵暟缁勭殑鎸囬拡銆佹暟缁勪腑鎺у埗鏁伴噺鐨勪竴涓鏁帮紝浠ュ強涓€涓帶鍒剁被鍒€?鎺у埗绫诲埆鐢ㄤ簬灏嗙浉浼肩殑鎺у埗褰掍负鍗曚竴绫诲埆銆備緥濡傦紝鎺у埗绫诲埆 `V4L2_CTRL_CLASS_USER`
鍖呭惈鎵€鏈夌敤鎴锋帶鍒讹紙鍗充篃鑳戒娇鐢ㄦ棫鐨?VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl
璁剧疆鐨勬墍鏈夋帶鍒讹級銆傛帶鍒剁被鍒?`V4L2_CTRL_CLASS_CODEC` 鍖呭惈涓庣紪瑙ｇ爜鍣ㄧ浉鍏崇殑
鎺у埗銆?
鎺у埗鏁扮粍涓殑鎵€鏈夋帶鍒堕兘蹇呴』灞炰簬鎸囧畾鐨勬帶鍒剁被鍒€傚惁鍒欎細杩斿洖閿欒銆?
涔熷彲浠ヤ娇鐢ㄤ竴涓┖鐨勬帶鍒舵暟缁勶紙`count` == 0锛夋潵妫€鏌ユ寚瀹氱殑鎺у埗绫诲埆鏄惁鍙楁敮鎸併€?
鎺у埗鏁扮粍鏄竴涓?struct `v4l2_ext_control` 鏁扮粍銆俿truct `v4l2_ext_control`
涓?struct `v4l2_control` 闈炲父鐩镐技锛屽彧鏄畠杩樺厑璁镐紶鍏?64 浣嶅€煎拰鎸囬拡銆?
鐢变簬 struct `v4l2_ext_control` 鏀寔鎸囬拡锛岀幇鍦ㄤ篃鍙互鎷ユ湁澶嶅悎绫诲瀷锛堝
N 缁存暟缁勫拰/鎴栫粨鏋勪綋锛夌殑鎺у埗銆傚湪鏋氫妇鎺у埗鏃讹紝浣犻渶瑕佹寚瀹?`V4L2_CTRL_FLAG_NEXT_COMPOUND` 鎵嶈兘瀹為檯鐪嬪埌杩欑被澶嶅悎鎺у埗銆傛崲瑷€涔嬶紝杩欎簺
澶嶅悎绫诲瀷鐨勬帶鍒跺彧搴斾互缂栫▼鏂瑰紡浣跨敤銆?
鐢变簬杩欑被澶嶅悎鎺у埗闇€瑕佹毚闇叉瘮 VIDIOC_QUERYCTRL <VIDIOC_QUERYCTRL> 鎵€鑳芥彁渚?鐨勬洿澶氫俊鎭紝鍥犳澧炲姞浜?VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL> ioctl銆?鐗瑰埆鍦帮紝褰撹鎺у埗鐢卞涓厓绱犵粍鎴愭椂锛屾 ioctl 浼氱粰鍑?N 缁存暟缁勭殑缁村害銆?

   #. 閲嶈鐨勬槸瑕佽璇嗗埌锛岀敱浜庢帶鍒剁殑鐏垫椿鎬э紝鏈夊繀瑕佹鏌ヤ綘鎯宠璁剧疆鐨勬帶鍒舵槸鍚?      纭疄鍙楅┍鍔ㄦ敮鎸侊紝浠ュ強鍏舵湁鏁堝€艰寖鍥存槸浠€涔堛€傛墍浠ヨ浣跨敤 VIDIOC_QUERYCTRL
      鏉ユ鏌ャ€?
   #. 绫诲瀷涓?`V4L2_CTRL_TYPE_MENU` 鐨勬帶鍒朵腑锛屾煇浜涜彍鍗曠储寮曞彲鑳戒笉鍙楁敮鎸?      锛坄VIDIOC_QUERYMENU` 浼氳繑鍥為敊璇級銆備竴涓緢濂界殑渚嬪瓙鏄彈鏀寔鐨?MPEG
      闊抽姣旂壒鐜囧垪琛ㄣ€傛湁浜涢┍鍔ㄥ彧鏀寔涓€涓ょ姣旂壒鐜囷紝鍙︿竴浜涘垯鏀寔鏇村鐨勮寖鍥淬€?
鎵€鏈夋帶鍒堕兘浣跨敤鏈哄櫒瀛楄妭搴忋€?

## 鏋氫妇鎵╁睍鎺у埗


鎺ㄨ崘鐨勬灇涓炬墿灞曟帶鍒剁殑鏂瑰紡鏄娇鐢?VIDIOC_QUERYCTRL 閰嶅悎
`V4L2_CTRL_FLAG_NEXT_CTRL` 鏍囧織锛?


    struct v4l2_queryctrl qctrl;

    qctrl.id = V4L2_CTRL_FLAG_NEXT_CTRL;
    while (0 == ioctl (fd, VIDIOC_QUERYCTRL, &qctrl)) {
	/** ... **/
	qctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL;
    }

鍒濆鐨勬帶鍒?ID 琚涓?0 涓?`V4L2_CTRL_FLAG_NEXT_CTRL` 鏍囧織鐩告垨鐨勭粨鏋溿€?`VIDIOC_QUERYCTRL` ioctl 灏嗚繑鍥?ID 姣旀寚瀹氬€兼洿楂樼殑绗竴涓帶鍒躲€傚綋鎵句笉鍒拌繖鏍?鐨勬帶鍒舵椂锛屼細杩斿洖閿欒銆?
濡傛灉浣犳兂鑾峰彇鐗瑰畾鎺у埗绫诲埆鍐呯殑鎵€鏈夋帶鍒讹紝鍙互灏嗗垵濮嬬殑 `qctrl.id` 鍊艰涓鸿
鎺у埗绫诲埆锛屽苟澧炲姞涓€涓澶栫殑妫€鏌ワ紝浠ヤ究鍦ㄥ彂鐜板睘浜庡彟涓€鎺у埗绫诲埆鐨勬帶鍒舵椂璺冲嚭
寰幆锛?


    qctrl.id = V4L2_CTRL_CLASS_CODEC | V4L2_CTRL_FLAG_NEXT_CTRL;
    while (0 == ioctl(fd, VIDIOC_QUERYCTRL, &qctrl)) {
	if (V4L2_CTRL_ID2CLASS(qctrl.id) != V4L2_CTRL_CLASS_CODEC)
	    break;
	/** ... **/
	qctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL;
    }

32 浣嶇殑 `qctrl.id` 鍊艰鍒掑垎涓轰笁涓綅娈碉細鏈€楂樼殑 4 浣嶄繚鐣欑粰鏍囧織锛堜緥濡?`V4L2_CTRL_FLAG_NEXT_CTRL`锛夛紝骞朵笉灞炰簬 ID 鏈韩銆傚墿涓嬬殑 28 浣嶆瀯鎴愭帶鍒?ID锛?鍏朵腑鏈€楂?12 浣嶅畾涔夋帶鍒剁被鍒紝鏈€浣?16 浣嶆爣璇嗚鎺у埗绫诲埆鍐呯殑鎺у埗銆傚彲浠ヤ繚璇?杩欎簺鏈€鍚庣殑 16 浣嶅浜庢墍鏈夋帶鍒堕兘闈為浂銆?x1000 鍙婁互涓婄殑鑼冨洿淇濈暀缁欓┍鍔ㄧ鏈?鎺у埗銆傚畯 `V4L2_CTRL_ID2CLASS(id)` 鏍规嵁鎺у埗 ID 杩斿洖鎺у埗绫诲埆 ID銆?
濡傛灉椹卞姩涓嶆敮鎸佹墿灞曟帶鍒讹紝閭ｄ箞 `VIDIOC_QUERYCTRL` 涓?`V4L2_CTRL_FLAG_NEXT_CTRL`
閰嶅悎浣跨敤鏃跺皢澶辫触銆傝繖绉嶆儏鍐典笅搴斾娇鐢ㄦ棫鐨勬灇涓炬帶鍒舵柟娉曪紙瑙?enum_all_controls锛夈€?浣嗗鏋滃彈鏀寔锛屽垯淇濊瘉浼氭灇涓炬墍鏈夋帶鍒讹紝鍖呮嫭椹卞姩绉佹湁鎺у埗銆?

## 鍒涘缓鎺у埗闈㈡澘


鍙互涓哄浘褰㈢敤鎴风晫闈㈠垱寤烘帶鍒堕潰鏉匡紝璁╃敤鎴峰彲浠ラ€夋嫨鍚勭鎺у埗銆傚熀鏈笂浣犲皢闇€瑕?浣跨敤涓婅堪鏂规硶閬嶅巻鎵€鏈夋帶鍒躲€傛瘡涓帶鍒剁被鍒兘浠ヤ竴涓被鍨嬩负
`V4L2_CTRL_TYPE_CTRL_CLASS` 鐨勬帶鍒跺紑濮嬨€俙VIDIOC_QUERYCTRL` 灏嗚繑鍥炴鎺у埗
绫诲埆鐨勫悕绉帮紝鍙敤浣滄帶鍒堕潰鏉夸腑鏍囩椤电殑鏍囬銆?
struct v4l2_queryctrl <v4l2-queryctrl> 鐨?flags 瀛楁涔熷寘鍚叧浜庢帶鍒惰涓虹殑
鎻愮ず銆傝瑙?VIDIOC_QUERYCTRL 鏂囨。銆?