
######## RF 璋冭皭鍣ㄦ帶鍒跺弬鑰?

RF 璋冭皭鍣紙RF_TUNER锛夌被鍖呭惈鍏锋湁 RF 璋冭皭鍣ㄨ澶囩殑閫氱敤鐗规€х浉鍏崇殑鎺т欢銆?
鍦ㄦ璇涓嬶紝RF 璋冭皭鍣ㄦ槸浠嬩簬澶╃嚎涓庤В璋冨櫒涔嬮棿鐨勬棤绾跨數鎺ユ敹鐢佃矾銆傚畠浠庡ぉ绾挎帴鏀跺皠棰戯紙RF锛夛紝
骞跺皢鎺ユ敹鍒扮殑淇″彿杞崲涓鸿緝浣庣殑涓锛圛F锛夋垨鍩哄甫棰戠巼锛圔B锛夈€傝兘澶熻緭鍑哄熀甯︾殑璋冭皭鍣ㄩ€氬父
琚О涓洪浂涓锛圸ero-IF锛夎皟璋愬櫒銆傝緝鑰佺殑璋冭皭鍣ㄩ€氬父鏄噾灞炵洅鍐呯殑绠€鍗?PLL 璋冭皭鍣紝鑰岃緝鏂扮殑
鍒欐槸楂樺害闆嗘垚鐨勮姱鐗囷紙鏃犻噾灞炵洅鐨勨€滅璋冭皭鍣ㄢ€濓級銆傝繖浜涙帶浠跺ぇ澶氶€傜敤浜庡姛鑳戒赴瀵岀殑鏂板紡纭呰皟璋愬櫒锛?鍥犱负杈冭€佺殑璋冭皭鍣ㄥ嚑涔庢病鏈夊灏戝彲璋冪壒鎬с€?
鏈夊叧 RF 璋冭皭鍣ㄧ殑鏇村淇℃伅锛岃鍙傝缁村熀鐧剧涓婄殑
`Tuner (radio) <http://en.wikipedia.org/wiki/Tuner_%28radio%29>`__
涓?`RF front end <http://en.wikipedia.org/wiki/RF_front_end>`__銆?
## RF_TUNER 鎺т欢 ID


`V4L2_CID_RF_TUNER_CLASS (class)`
    RF_TUNER 绫绘弿杩扮銆傚姝ゆ帶浠惰皟鐢?VIDIOC_QUERYCTRL 灏?    杩斿洖璇ユ帶浠剁被鐨勬弿杩般€?
`V4L2_CID_RF_TUNER_BANDWIDTH_AUTO (boolean)`
    鍚敤/绂佺敤璋冭皭鍣ㄦ棤绾跨數棰戦亾甯﹀閰嶇疆銆傚湪鑷姩妯″紡涓嬶紝甯﹀閰嶇疆鐢遍┍鍔ㄦ墽琛屻€?
`V4L2_CID_RF_TUNER_BANDWIDTH (integer)`
    璋冭皭鍣ㄤ俊鍙疯矾寰勪笂鐨勬护娉㈠櫒鐢ㄤ簬鏍规嵁鎺ユ敹鏂圭殑闇€姹傝繃婊や俊鍙枫€傞┍鍔ㄩ厤缃护娉㈠櫒浠ユ弧瓒?    鏈熸湜鐨勫甫瀹借姹傘€傚湪 V4L2_CID_RF_TUNER_BANDWIDTH_AUTO 鏈缃椂浣跨敤銆傚崟浣嶄负 Hz銆?    鑼冨洿涓庢杩涚敱椹卞姩鍐冲畾銆?
`V4L2_CID_RF_TUNER_LNA_GAIN_AUTO (boolean)`
    鍚敤/绂佺敤 LNA 鑷姩澧炵泭鎺у埗锛圓GC锛?
`V4L2_CID_RF_TUNER_MIXER_GAIN_AUTO (boolean)`
    鍚敤/绂佺敤娣烽鍣ㄨ嚜鍔ㄥ鐩婃帶鍒讹紙AGC锛?
`V4L2_CID_RF_TUNER_IF_GAIN_AUTO (boolean)`
    鍚敤/绂佺敤 IF 鑷姩澧炵泭鎺у埗锛圓GC锛?
`V4L2_CID_RF_TUNER_RF_GAIN (integer)`
    RF 鏀惧ぇ鍣ㄦ槸鎺ユ敹淇″彿璺緞涓婄揣鎺ュぉ绾胯緭鍏ヤ箣鍚庣殑绗竴涓斁澶у櫒銆傛湰鏂囨。涓?LNA 澧炵泭涓?    RF 澧炵泭鐨勫尯鍒湪浜庯細LNA 澧炵泭闆嗘垚鍦ㄨ皟璋愬櫒鑺墖鍐咃紝鑰?RF 澧炵泭鏄嫭绔嬬殑鑺墖銆?    鍚屼竴璁惧涓彲鑳藉悓鏃跺瓨鍦?RF 涓?LNA 澧炵泭鎺т欢銆傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_RF_TUNER_LNA_GAIN (integer)`
    LNA锛堜綆鍣０鏀惧ぇ鍣級澧炵泭鏄?RF 璋冭皭鍣ㄤ俊鍙疯矾寰勪笂鐨勭涓€绾у鐩娿€傚畠浣嶄簬闈炲父闈犺繎璋冭皭鍣?    澶╃嚎杈撳叆鐨勪綅缃€傚湪 `V4L2_CID_RF_TUNER_LNA_GAIN_AUTO` 鏈缃椂浣跨敤銆傚弬瑙?    `V4L2_CID_RF_TUNER_RF_GAIN` 浠ヤ簡瑙?RF 澧炵泭涓?LNA 澧炵泭鐨勫尯鍒€傝寖鍥翠笌姝ヨ繘鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_RF_TUNER_MIXER_GAIN (integer)`
    娣烽鍣ㄥ鐩婃槸 RF 璋冭皭鍣ㄤ俊鍙疯矾寰勪笂鐨勭浜岀骇澧炵泭銆傚畠浣嶄簬娣烽鍣ㄥ潡鍐呴儴锛孯F 淇″彿鍦ㄦ琚?    娣烽鍣ㄤ笅鍙橀銆傚湪 `V4L2_CID_RF_TUNER_MIXER_GAIN_AUTO` 鏈缃椂浣跨敤銆傝寖鍥翠笌姝ヨ繘
    鐢遍┍鍔ㄥ喅瀹氥€?
`V4L2_CID_RF_TUNER_IF_GAIN (integer)`
    IF 澧炵泭鏄?RF 璋冭皭鍣ㄤ俊鍙疯矾寰勪笂鐨勬渶鍚庝竴绾у鐩娿€傚畠浣嶄簬 RF 璋冭皭鍣ㄧ殑杈撳嚭绔€傚畠鎺у埗
    涓杈撳嚭鎴栧熀甯﹁緭鍑虹殑淇″彿鐢靛钩銆傚湪 `V4L2_CID_RF_TUNER_IF_GAIN_AUTO` 鏈缃椂浣跨敤銆?    鑼冨洿涓庢杩涚敱椹卞姩鍐冲畾銆?
`V4L2_CID_RF_TUNER_PLL_LOCK (boolean)`
    鍚堟垚鍣?PLL 鏄惁宸查攣瀹氾紵褰撹鎺т欢缃綅鏃讹紝RF 璋冭皭鍣ㄦ鍦ㄦ帴鏀剁粰瀹氱殑棰戠巼銆傝繖鏄竴涓彧璇绘帶浠躲€?