
######## V4L2_META_FMT_RK_ISP1_PARAMS ('rk1p')銆乂4L2_META_FMT_RK_ISP1_STAT_3A ('rk1s')銆乂4L2_META_FMT_RK_ISP1_EXT_PARAMS ('rk1e')


## 閰嶇疆鍙傛暟


RkISP1 ISP 鐨勯厤缃敱鐢ㄦ埛绌洪棿閫氳繃 `v4l2_meta_format` 鎺ュ彛鍚戦┍鍔ㄦ彁渚?ISP 鍙傛暟
鏉ュ畬鎴愩€?
鏈変袱绉嶆柟娉曞彲浠ラ厤缃?ISP锛屽垎鍒槸 `fixed parameters`锛堝浐瀹氬弬鏁帮級閰嶇疆鏍煎紡涓?`extensible parameters`锛堝彲鎵╁睍鍙傛暟锛夐厤缃牸寮忋€?

## 鍥哄畾鍙傛暟閰嶇疆鏍煎紡


浣跨敤鍥哄畾閰嶇疆鏍煎紡鏃讹紝鍙傛暟閫氳繃 `V4L2_META_FMT_RK_ISP1_PARAMS` 鍏冩牸寮忎紶閫掔粰
rkisp1_params <rkisp1_params> 鍏冩暟鎹緭鍑鸿棰戣妭鐐广€?
缂撳啿鍖哄寘鍚?`rkisp1-config.h` 涓畾涔夌殑 C 缁撴瀯浣?`rkisp1_params_cfg` 鐨勫崟涓?瀹炰緥銆傚洜姝ゅ彲浠ヤ粠缂撳啿鍖轰腑杩欐牱鑾峰彇璇ョ粨鏋勪綋锛?

	struct rkisp1_params_cfg **params = (struct rkisp1_params_cfg**) buffer;

璇ユ柟娉曚粎鏀寔 ISP 鐗规€х殑涓€閮ㄥ垎锛屾柊鐨勫簲鐢ㄧ▼搴忓簲褰撲娇鐢ㄥ彲鎵╁睍鍙傛暟鏂规硶銆?

## 鍙墿灞曞弬鏁伴厤缃牸寮?

浣跨敤鍙墿灞曢厤缃牸寮忔椂锛屽弬鏁伴€氳繃 `V4L2_META_FMT_RK_ISP1_EXT_PARAMS` 鍏冩牸寮?浼犻€掔粰 rkisp1_params <rkisp1_params> 鍏冩暟鎹緭鍑鸿棰戣妭鐐广€?
缂撳啿鍖哄寘鍚?`rkisp1-config.h` 涓畾涔夌殑 C 缁撴瀯浣?`rkisp1_ext_params_cfg` 鐨?鍗曚釜瀹炰緥銆俙rkisp1_ext_params_cfg` 缁撴瀯浣撹璁捐涓哄厑璁哥敤鎴风┖闂翠粎鐢ㄥ叾鎵撶畻閰嶇疆鐨?ISP 妯″潡鐨勯厤缃暟鎹潵濉厖鏁版嵁缂撳啿鍖恒€傚彲鎵╁睍鍙傛暟鏍煎紡鐨勮璁″厑璁稿紑鍙戣€呭畾涔夋柊鐨?妯″潡绫诲瀷浠ユ敮鎸佹柊鐨勯厤缃弬鏁帮紝骞跺畾涔変簡涓€濂楃増鏈満鍒讹紝浠庤€屽彲浠ュ湪涓嶇牬鍧忎笌鐜版湁
搴旂敤绋嬪簭鍏煎鎬х殑鎯呭喌涓嬭繘琛屾墿灞曚笌鐗堟湰绠＄悊銆?
鍩轰簬杩欎簺鍘熷洜锛岃閰嶇疆鏂规硶浼樺厛浜?`fixed parameters`锛堝浐瀹氬弬鏁帮級鏍煎紡鏂规銆?

## 3A 涓庣洿鏂瑰浘缁熻


ISP1 璁惧浼氶拡瀵硅緭鍏ョ殑 Bayer 甯ф敹闆嗕笉鍚岀殑缁熻鏁版嵁銆傝繖浜涚粺璁℃暟鎹€氳繃
`v4l2_meta_format` 鎺ュ彛浠?rkisp1_stats <rkisp1_stats> 鍏冩暟鎹崟鑾疯棰戣妭鐐?鑾峰彇锛岀紦鍐插尯鍖呭惈 `rkisp1-config.h` 涓畾涔夌殑 C 缁撴瀯浣?`rkisp1_stat_buffer` 鐨?鍗曚釜瀹炰緥銆傚洜姝ゅ彲浠ヤ粠缂撳啿鍖轰腑杩欐牱鑾峰彇璇ョ粨鏋勪綋锛?

	struct rkisp1_stat_buffer **stats = (struct rkisp1_stat_buffer**) buffer;

鏀堕泦鐨勭粺璁′俊鎭寘鎷洕鍏夈€丄WB锛堣嚜鍔ㄧ櫧骞宠　锛夈€佺洿鏂瑰浘涓?AF锛堣嚜鍔ㄥ鐒︼級銆傜粺璁′俊鎭?鐨勮鎯呰鍙傝 `rkisp1_stat_buffer`銆?
姝ゅ鎻忚堪鐨?3A 缁熻淇℃伅涓庨厤缃弬鏁伴€氬父鐢变笓鐢ㄧ殑鐢ㄦ埛绌洪棿搴撴秷璐瑰拰浜х敓锛岃繖浜涘簱
鍖呭惈浜嗕娇鐢ㄨ蒋浠舵帶鍒剁幆鐨勯噸瑕佽皟浼樺伐鍏枫€?

## rkisp1 uAPI 鏁版嵁绫诲瀷


