## ethtool 鐨?Netlink 鎺ュ彛


## 鍩烘湰淇℃伅


ethtool 鐨?netlink 鎺ュ彛浣跨敤鍚嶄负 `ethtool` 鐨?generic netlink 绯诲垪
锛堢敤鎴锋€佸簲鐢ㄧ▼搴忓簲浣跨敤鍦?`<linux/ethtool_netlink.h>` uapi 澶存枃浠朵腑瀹氫箟鐨勫畯
`ETHTOOL_GENL_NAME` 涓?`ETHTOOL_GENL_VERSION`锛夈€傝绯诲垪涓嶄娇鐢ㄧ壒瀹氱殑澶撮儴锛岃姹備笌
鍥炲涓殑鎵€鏈変俊鎭潎閫氳繃 netlink 灞炴€т紶閫掋€?

ethtool netlink 鎺ュ彛浣跨敤鎵╁睍 ACK锛坋xtended ACK锛夋潵涓婃姤閿欒涓庤鍛婏紝寤鸿鐢ㄦ埛鎬佸簲鐢?
绋嬪簭寮€鍙戣€呬互鍚堥€傜殑鏂瑰紡灏嗚繖浜涙秷鎭憟鐜扮粰鐢ㄦ埛銆?

璇锋眰鍙垎涓轰笁绫伙細鈥済et鈥濓紙鑾峰彇淇℃伅锛夈€佲€渟et鈥濓紙璁剧疆鍙傛暟锛変笌鈥渁ction鈥濓紙鎵ц鏌愪釜鍔ㄤ綔锛夈€?

鎵€鏈夆€渟et鈥濅笌鈥渁ction鈥濈被鍨嬬殑璇锋眰閮介渶瑕佺鐞嗗憳鏉冮檺锛堝懡鍚嶇┖闂村唴鐨?
`CAP_NET_ADMIN`锛夈€傚ぇ澶氭暟鈥済et鈥濈被鍨嬬殑璇锋眰鍏佽浠讳綍浜鸿皟鐢紝浣嗕篃鏈変緥澶栵紙褰撳洖澶嶄腑鍖呭惈
鏁忔劅淇℃伅鏃讹級銆傚湪鏌愪簺鎯呭喌涓嬶紝璇锋眰鏈韩瀵逛换浣曚汉閮芥槸鍏佽鐨勶紝浣嗛潪鐗规潈鐢ㄦ埛浼氳鐪佺暐鎺?
鍖呭惈鏁忔劅淇℃伅鐨勫睘鎬э紙渚嬪鍞ら啋灞€鍩熺綉瀵嗙爜锛夈€?


## 绾﹀畾


琛ㄧず甯冨皵鍊肩殑灞炴€ч€氬父娌跨敤 NLA_U8 绫诲瀷锛屼互渚垮尯鍒嗕笁绉嶇姸鎬侊細鈥渙n鈥濓紙寮€锛夈€佲€渙ff鈥濓紙鍏筹級
涓庘€渘ot present鈥濓紙涓嶅瓨鍦紝鍗斥€済et鈥濊姹備腑淇℃伅涓嶅彲鐢紝鎴栤€渟et鈥濊姹備腑鏃犻渶鏀瑰彉璇ュ€硷級銆?
瀵逛簬杩欎簺灞炴€э紝鈥渢rue鈥濓紙鐪燂級鍊煎簲浠ユ暟瀛?1 浼犻€掞紝浣嗘帴鏀舵柟搴斿皢浠讳綍闈為浂鍊奸兘鐞嗚В涓衡€渢rue鈥濄€?
鍦ㄤ笅鏂圭殑琛ㄤ腑锛屸€渂ool鈥濊〃绀轰互姝ゆ柟寮忚В閲婄殑 NLA_U8 灞炴€с€?

鍦ㄤ笅闈㈢殑娑堟伅缁撴瀯鎻忚堪涓紝鑻ユ煇灞炴€у悕甯︽湁鈥?鈥濆悗缂€锛屽垯琛ㄧず鍏剁埗宓屽涓彲浠ュ寘鍚涓浉鍚?
绫诲瀷鐨勫睘鎬с€傝繖瀹炵幇浜嗕竴涓潯鐩暟缁勩€?

闇€瑕佺敱璁惧椹卞姩濉厖銆佸苟渚濇嵁鍏舵槸鍚︽湁鏁堟潵dump鍒扮敤鎴风┖闂寸殑灞炴€э紝涓嶅簲浣跨敤闆朵綔涓烘湁鏁堝€笺€?
杩欐牱鍙互閬垮厤鍦ㄨ澶囬┍鍔?API 涓樉寮忔爣璇嗚灞炴€х殑鏈夋晥鎬с€?


## 璇锋眰澶?


姣忎釜璇锋眰鎴栧洖澶嶆秷鎭兘鍖呭惈涓€涓甫鏈夊叕鍏卞ご閮ㄧ殑宓屽灞炴€с€傝澶撮儴鐨勭粨鏋勫涓嬶細

  ==============================  ======  =============================
  `ETHTOOL_A_HEADER_DEV_INDEX`  u32     device ifindex
  `ETHTOOL_A_HEADER_DEV_NAME`   string  device name
  `ETHTOOL_A_HEADER_FLAGS`      u32     flags common for all requests
  `ETHTOOL_A_HEADER_PHY_INDEX`  u32     phy device index
  ==============================  ======  =============================

`ETHTOOL_A_HEADER_DEV_INDEX` 涓?`ETHTOOL_A_HEADER_DEV_NAME` 鐢ㄤ簬鏍囪瘑娑堟伅鎵€鍏宠仈鐨?
璁惧銆傚湪璇锋眰涓彁渚涘叾涓竴涓嵆鍙紱鑻ヤ袱鑰呴兘鎻愪緵锛屽垯瀹冧滑蹇呴』鎸囧悜鍚屼竴璁惧銆傛煇浜涜姹?
锛堜緥濡傚叏灞€瀛楃涓查泦锛変笉闇€瑕佽澶囨爣璇嗐€傚ぇ澶氭暟 `GET` 璇锋眰涔熷厑璁镐笉甯﹁澶囨爣璇嗙殑 dump
璇锋眰锛屼互鏌ヨ鎻愪緵璇ヤ俊鎭殑鍏ㄩ儴璁惧锛堟瘡涓澶囧崟鐙竴鏉℃秷鎭級銆?

`ETHTOOL_A_HEADER_FLAGS` 鏄竴涓鎵€鏈夎姹傜被鍨嬮€氱敤鐨勮姹傛爣蹇椾綅鍥俱€傝繖浜涙爣蹇楃殑瑙ｉ噴
瀵规墍鏈夎姹傜被鍨嬮兘鐩稿悓锛屼絾鏌愪簺鏍囧織鍙兘涓嶉€傜敤浜庣壒瀹氳姹傘€傚凡璇嗗埆鐨勬爣蹇楀涓嬶細

  =================================  ===================================
  `ETHTOOL_FLAG_COMPACT_BITSETS`   鍥炲涓娇鐢ㄧ揣鍑戞牸寮忎綅闆?
  `ETHTOOL_FLAG_OMIT_REPLY`        鐪佺暐鍙€夊洖澶嶏紙_SET 涓?_ACT锛?
  `ETHTOOL_FLAG_STATS`             鍖呭惈鍙€夌殑璁惧缁熻淇℃伅
  =================================  ===================================

鏂扮殑璇锋眰鏍囧織搴旈伒寰竴涓€讳綋鍘熷垯锛氳嫢鏈缃鏍囧織锛屽垯琛屼负淇濇寔鍚戝悗鍏煎锛屽嵆鏉ヨ嚜涓嶄簡瑙?
璇ユ爣蹇楃殑鏃у鎴风鍙戝嚭鐨勮姹傦紝搴旀寜瀹㈡埛绔湡鏈涚殑鏂瑰紡瑙ｉ噴銆傚鎴风涓嶅緱璁剧疆瀹冩墍涓嶄簡瑙ｇ殑
鏍囧織銆?

`ETHTOOL_A_HEADER_PHY_INDEX` 鏍囪瘑娑堟伅鎵€鍏宠仈鐨勪互澶綉 PHY銆傜敱浜庢湁澶ч噺鍛戒护涓?PHY 閰嶇疆
鐩稿叧锛屼笖閾捐矾涓婂彲鑳藉瓨鍦ㄥ涓?PHY锛屽浜庨渶瑕佸畠鐨勫懡浠わ紝鍙互鍦ㄨ姹備腑浼犲叆 PHY 绱㈠紩銆備絾
杩欏苟闈炲己鍒惰姹傦紱濡傛灉閽堝 PHY 鐨勫懡浠ゆ湭浼犲叆璇ュ€硷紝鍒欎細浣跨敤 net_device.phydev 鎸囬拡銆?

## 浣嶉泦鍚?


瀵逛簬闀垮害锛堢浉瀵癸級鍥哄畾鐨勭煭浣嶅浘锛屼娇鐢ㄦ爣鍑嗙殑 `NLA_BITFIELD32` 绫诲瀷銆傚浜庝换鎰忛暱搴︾殑浣嶅浘锛?
ethtool netlink 浣跨敤涓€绉嶅祵濂楀睘鎬э紝鍏跺唴瀹归噰鐢ㄤ袱绉嶅舰寮忎箣涓€锛氱揣鍑戝舰寮忥紙涓や釜浜岃繘鍒朵綅鍥撅紝
鍒嗗埆琛ㄧず浣嶅€间笌鍙楀奖鍝嶄綅鐨勬帺鐮侊級涓庨€愪綅褰㈠紡锛堢敱绱㈠紩鎴栧悕绉版爣璇嗙殑浣嶅垪琛級銆?

璇︾粏锛堥€愪綅锛変綅闆嗗厑璁稿皢浣嶇殑绗﹀彿鍚嶄笌鍏跺€间竴鍚屽彂閫侊紝浠庤€岀渷鍘讳竴娆″線杩旓紙褰撲綅闆嗗湪璇锋眰涓紶閫掓椂锛?
鎴栬嚦灏戠渷鍘讳竴娆￠澶栬姹傦紙褰撲綅闆嗗湪鍥炲涓椂锛夈€傝繖瀵逛簬浼犵粺 ethtool 鍛戒护杩欑被涓€娆℃€у簲鐢ㄥ緢鏈?
鐢ㄥ銆傚彟涓€鏂归潰锛屽儚 ethtool monitor锛堟樉绀洪€氱煡锛夋垨缃戠粶绠＄悊瀹堟姢杩涚▼杩欑被闀挎湡杩愯鐨勫簲鐢紝
鍙兘鏇村€惧悜浜庝粎鑾峰彇涓€娆″悕绉帮紝骞朵娇鐢ㄧ揣鍑戝舰寮忎互鍑忓皬娑堟伅浣撶Н銆俥thtool netlink 鎺ュ彛鍙戝嚭鐨?
閫氱煡濮嬬粓瀵逛綅闆嗕娇鐢ㄧ揣鍑戝舰寮忋€?

涓€涓綅闆嗘棦鍙互琛ㄧず涓€涓€?鎺╃爜瀵癸紙`ETHTOOL_A_BITSET_NOMASK` 鏈缃級锛屼篃鍙互琛ㄧず鍗曚釜
浣嶅浘锛坄ETHTOOL_A_BITSET_NOMASK` 宸茶缃級銆傚湪淇敼浣嶅浘鐨勮姹備腑锛屽墠鑰呭皢鎺╃爜涓綅瀵瑰簲鐨勪綅
淇敼涓哄€间腑璁剧疆鐨勪綅锛屽叾浣欎繚鎸佷笉鍙橈紱鍚庤€呭垯灏嗕綅鍥句腑璁剧疆鐨勪綅璁句负 1锛屽叾浣欐竻 0銆?

绱у噾褰㈠紡锛氬祵濂楋紙浣嶉泦锛夊睘鎬х殑鍐呭锛?

  ============================  ======  ============================
  `ETHTOOL_A_BITSET_NOMASK`   flag    no mask, only a list
  `ETHTOOL_A_BITSET_SIZE`     u32     number of significant bits
  `ETHTOOL_A_BITSET_VALUE`    binary  bitmap of bit values
  `ETHTOOL_A_BITSET_MASK`     binary  bitmap of valid bits
  ============================  ======  ============================

鍊煎拰鎺╃爜鐨勯暱搴﹀繀椤昏嚦灏戜负 `ETHTOOL_A_BITSET_SIZE` 涓綅锛屽苟鍚戜笂鍙栨暣鍒?32 浣嶇殑鏁存暟鍊嶃€?
瀹冧滑鐢变互涓绘満瀛楄妭搴忓瓨鏀剧殑 32 浣嶅瓧缁勬垚锛屽瓧浠庢渶浣庢湁鏁堝埌鏈€楂樻湁鏁堟帓搴忥紙鍗充笌 ioctl 鎺ュ彛
浼犻€掍綅鍥剧殑鏂瑰紡鐩稿悓锛夈€?

瀵逛簬绱у噾褰㈠紡锛宍ETHTOOL_A_BITSET_SIZE` 涓?`ETHTOOL_A_BITSET_VALUE` 鏄繀濉殑銆?
褰?`ETHTOOL_A_BITSET_NOMASK` 鏈缃椂锛堜綅闆嗚〃绀轰竴涓€?鎺╃爜瀵癸級锛宍ETHTOOL_A_BITSET_MASK`
灞炴€т负蹇呭～锛涜嫢 `ETHTOOL_A_BITSET_NOMASK` 鏈缃紝`ETHTOOL_A_BITSET_MASK` 鍒欎笉鍏佽鍑虹幇
锛堜綅闆嗚〃绀轰竴涓崟鐙綅鍥撅級銆?

濡傛灉杈冩棫鐨勫簲鐢ㄧ▼搴忚繍琛屽湪杈冩柊鐨勫唴鏍镐笂锛屾垨鍙嶄箣锛屽唴鏍哥殑浣嶉泦闀垮害鍙兘涓庣敤鎴风┖闂寸殑闀垮害涓嶅悓銆?
鑻ョ敤鎴风┖闂寸殑浣嶅浘鏇撮暱锛屼粎褰撹姹傚疄闄呰瘯鍥捐缃煇浜涘唴鏍告棤娉曡瘑鍒殑浣嶇殑鍊兼椂锛屾墠浼氳繑鍥為敊璇€?

閫愪綅褰㈠紡锛氬祵濂楋紙浣嶉泦锛夊睘鎬х殑鍐呭锛?

 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_NOMASK`        | flag   | no mask, only a list        |
 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_SIZE`          | u32    | number of significant bits  |
 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_BITS`          | nested | array of bits               |
 +-+----------------------------------+--------+-----------------------------+
 | | `ETHTOOL_A_BITSET_BITS_BIT+`   | nested | one bit                     |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_INDEX` | u32    | bit index (0 for LSB)       |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_NAME`  | string | bit name                    |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_VALUE` | flag   | present if bit is set       |
 +-+-+--------------------------------+--------+-----------------------------+

瀵逛簬閫愪綅褰㈠紡锛宍ETHTOOL_A_BITSET_SIZE` 鏄彲閫夌殑锛宍ETHTOOL_A_BITSET_BITS` 涓哄繀濉€?
`ETHTOOL_A_BITSET_BITS` 宓屽涓彧鑳藉寘鍚?`ETHTOOL_A_BITSET_BITS_BIT` 灞炴€э紝浣嗗叾鏁伴噺
鍙互浠绘剰銆備竴涓綅鍙互閫氳繃鍏剁储寮曟垨鍚嶇О鏉ユ爣璇嗐€傚湪璇锋眰涓娇鐢ㄦ椂锛屾墍鍒楀嚭鐨勪綅浼氭牴鎹?
`ETHTOOL_A_BITSET_BIT_VALUE` 琚涓?0 鎴?1锛屽叾浣欎繚鎸佷笉鍙樸€?

濡傛灉绱㈠紩瓒呭嚭浜嗗唴鏍哥殑浣嶉暱搴︼紝鎴栬€呭悕绉版棤娉曡瘑鍒紝璇锋眰灏嗗け璐ャ€傝嫢鍚嶇О鍜岀储寮曞悓鏃惰缃紝涓?
瀹冧滑鎸囧悜涓嶅悓鐨勪綅锛岃姹備篃浼氬け璐ャ€?

褰?`ETHTOOL_A_BITSET_NOMASK` 鏍囧織瀛樺湪鏃讹紝浣嶉泦琚В閲婁负涓€涓畝鍗曚綅鍥俱€傝繖绉嶆儏鍐典笅涓嶄娇鐢?
`ETHTOOL_A_BITSET_BIT_VALUE` 灞炴€с€傛绫讳綅闆嗚〃绀轰竴涓綅鍥撅紝鍏朵腑鎵€鍒楀嚭鐨勪綅琚疆浣嶏紝鍏朵綑
涓?0銆?

鍦ㄨ姹備腑锛屽簲鐢ㄧ▼搴忓彲浠ヤ娇鐢ㄤ换鎰忎竴绉嶅舰寮忋€傚唴鏍稿湪鍥炲涓娇鐢ㄧ殑褰㈠紡鐢辫姹傚ご flags 瀛楁涓殑
`ETHTOOL_FLAG_COMPACT_BITSETS` 鏍囧織鍐冲畾銆傚€间笌鎺╃爜鐨勮涔夊彇鍐充簬鍏蜂綋灞炴€с€?


## 娑堟伅绫诲瀷鍒楄〃


鎵€鏈夋爣璇嗘秷鎭被鍨嬬殑甯搁噺閮戒娇鐢?`ETHTOOL_CMD_` 鍓嶇紑锛屽苟鏍规嵁娑堟伅鐢ㄩ€斾娇鐢ㄧ浉搴旂殑鍚庣紑锛?

  ==============    ======================================
  `_GET`          鐢ㄦ埛绌洪棿鐢ㄤ簬鑾峰彇鏁版嵁鐨勮姹?
  `_SET`          鐢ㄦ埛绌洪棿鐢ㄤ簬璁剧疆鏁版嵁鐨勮姹?
  `_ACT`          鐢ㄦ埛绌洪棿鐢ㄤ簬鎵ц鏌愪釜鍔ㄤ綔鐨勮姹?
  `_GET_REPLY`    鍐呮牳瀵?`GET` 璇锋眰鐨勫洖澶?
  `_SET_REPLY`    鍐呮牳瀵?`SET` 璇锋眰鐨勫洖澶?
  `_ACT_REPLY`    鍐呮牳瀵?`ACT` 璇锋眰鐨勫洖澶?
  `_NTF`          鍐呮牳閫氱煡
  ==============    ======================================

鐢ㄦ埛绌洪棿鍒板唴鏍革細

  ===================================== =================================
  `ETHTOOL_MSG_STRSET_GET`            鑾峰彇瀛楃涓查泦
  `ETHTOOL_MSG_LINKINFO_GET`          鑾峰彇閾捐矾璁剧疆
  `ETHTOOL_MSG_LINKINFO_SET`          璁剧疆閾捐矾璁剧疆
  `ETHTOOL_MSG_LINKMODES_GET`         鑾峰彇閾捐矾妯″紡淇℃伅
  `ETHTOOL_MSG_LINKMODES_SET`         璁剧疆閾捐矾妯″紡淇℃伅
  `ETHTOOL_MSG_LINKSTATE_GET`         鑾峰彇閾捐矾鐘舵€?
  `ETHTOOL_MSG_DEBUG_GET`             鑾峰彇璋冭瘯璁剧疆
  `ETHTOOL_MSG_DEBUG_SET`             璁剧疆璋冭瘯璁剧疆
  `ETHTOOL_MSG_WOL_GET`               鑾峰彇鍞ら啋灞€鍩熺綉璁剧疆
  `ETHTOOL_MSG_WOL_SET`               璁剧疆鍞ら啋灞€鍩熺綉璁剧疆
  `ETHTOOL_MSG_FEATURES_GET`          鑾峰彇璁惧鐗规€?
  `ETHTOOL_MSG_FEATURES_SET`          璁剧疆璁惧鐗规€?
  `ETHTOOL_MSG_PRIVFLAGS_GET`         鑾峰彇绉佹湁鏍囧織
  `ETHTOOL_MSG_PRIVFLAGS_SET`         璁剧疆绉佹湁鏍囧織
  `ETHTOOL_MSG_RINGS_GET`             鑾峰彇鐜舰闃熷垪澶у皬
  `ETHTOOL_MSG_RINGS_SET`             璁剧疆鐜舰闃熷垪澶у皬
  `ETHTOOL_MSG_CHANNELS_GET`          鑾峰彇閫氶亾鏁伴噺
  `ETHTOOL_MSG_CHANNELS_SET`          璁剧疆閫氶亾鏁伴噺
  `ETHTOOL_MSG_COALESCE_GET`          鑾峰彇涓柇鑱氬悎鍙傛暟
  `ETHTOOL_MSG_COALESCE_SET`          璁剧疆涓柇鑱氬悎鍙傛暟
  `ETHTOOL_MSG_PAUSE_GET`             鑾峰彇鏆傚仠鍙傛暟
  `ETHTOOL_MSG_PAUSE_SET`             璁剧疆鏆傚仠鍙傛暟
  `ETHTOOL_MSG_EEE_GET`               鑾峰彇 EEE 璁剧疆
  `ETHTOOL_MSG_EEE_SET`               璁剧疆 EEE 璁剧疆
  `ETHTOOL_MSG_TSINFO_GET`		鑾峰彇鏃堕棿鎴充俊鎭?
  `ETHTOOL_MSG_CABLE_TEST_ACT`        鍔ㄤ綔锛氬惎鍔ㄧ嚎缂嗘祴璇?
  `ETHTOOL_MSG_CABLE_TEST_TDR_ACT`    鍔ㄤ綔锛氬惎鍔ㄥ師濮?TDR 绾跨紗娴嬭瘯
  `ETHTOOL_MSG_TUNNEL_INFO_GET`       鑾峰彇闅ч亾鍗歌浇淇℃伅
  `ETHTOOL_MSG_FEC_GET`               鑾峰彇 FEC 璁剧疆
  `ETHTOOL_MSG_FEC_SET`               璁剧疆 FEC 璁剧疆
  `ETHTOOL_MSG_MODULE_EEPROM_GET`     璇诲彇 SFP 妯″潡 EEPROM
  `ETHTOOL_MSG_STATS_GET`             鑾峰彇鏍囧噯缁熻淇℃伅
  `ETHTOOL_MSG_PHC_VCLOCKS_GET`       鑾峰彇 PHC 铏氭嫙鏃堕挓淇℃伅
  `ETHTOOL_MSG_MODULE_SET`            璁剧疆鏀跺彂鍣ㄦā鍧楀弬鏁?
  `ETHTOOL_MSG_MODULE_GET`            鑾峰彇鏀跺彂鍣ㄦā鍧楀弬鏁?
  `ETHTOOL_MSG_PSE_SET`               璁剧疆 PSE 鍙傛暟
  `ETHTOOL_MSG_PSE_GET`               鑾峰彇 PSE 鍙傛暟
  `ETHTOOL_MSG_RSS_GET`               鑾峰彇 RSS 璁剧疆
  `ETHTOOL_MSG_PLCA_GET_CFG`          鑾峰彇 PLCA RS 鍙傛暟
  `ETHTOOL_MSG_PLCA_SET_CFG`          璁剧疆 PLCA RS 鍙傛暟
  `ETHTOOL_MSG_PLCA_GET_STATUS`       鑾峰彇 PLCA RS 鐘舵€?
  `ETHTOOL_MSG_MM_GET`                鑾峰彇 MAC 鍚堝苟灞傜姸鎬?
  `ETHTOOL_MSG_MM_SET`                璁剧疆 MAC 鍚堝苟灞傚弬鏁?
  `ETHTOOL_MSG_MODULE_FW_FLASH_ACT`   鐑у綍鏀跺彂鍣ㄦā鍧楀浐浠?
  `ETHTOOL_MSG_PHY_GET`               鑾峰彇浠ュお缃?PHY 淇℃伅
  `ETHTOOL_MSG_TSCONFIG_GET`          鑾峰彇纭欢鏃堕棿鎴抽厤缃?
  `ETHTOOL_MSG_TSCONFIG_SET`          璁剧疆纭欢鏃堕棿鎴抽厤缃?
  `ETHTOOL_MSG_RSS_SET`               璁剧疆 RSS 璁剧疆
  `ETHTOOL_MSG_RSS_CREATE_ACT`        鍒涘缓棰濆鐨?RSS 涓婁笅鏂?
  `ETHTOOL_MSG_RSS_DELETE_ACT`        鍒犻櫎棰濆鐨?RSS 涓婁笅鏂?
  `ETHTOOL_MSG_MSE_GET`               鑾峰彇 MSE 璇婃柇鏁版嵁
  ===================================== =================================

鍐呮牳鍒扮敤鎴风┖闂达細

  ======================================== =================================
  `ETHTOOL_MSG_STRSET_GET_REPLY`         瀛楃涓查泦鍐呭
  `ETHTOOL_MSG_LINKINFO_GET_REPLY`       閾捐矾璁剧疆
  `ETHTOOL_MSG_LINKINFO_NTF`             閾捐矾璁剧疆閫氱煡
  `ETHTOOL_MSG_LINKMODES_GET_REPLY`      閾捐矾妯″紡淇℃伅
  `ETHTOOL_MSG_LINKMODES_NTF`            閾捐矾妯″紡閫氱煡
  `ETHTOOL_MSG_LINKSTATE_GET_REPLY`      閾捐矾鐘舵€佷俊鎭?
  `ETHTOOL_MSG_DEBUG_GET_REPLY`          璋冭瘯璁剧疆
  `ETHTOOL_MSG_DEBUG_NTF`                璋冭瘯璁剧疆閫氱煡
  `ETHTOOL_MSG_WOL_GET_REPLY`            鍞ら啋灞€鍩熺綉璁剧疆
  `ETHTOOL_MSG_WOL_NTF`                  鍞ら啋灞€鍩熺綉璁剧疆閫氱煡
  `ETHTOOL_MSG_FEATURES_GET_REPLY`       璁惧鐗规€?
  `ETHTOOL_MSG_FEATURES_SET_REPLY`       閽堝 FEATURES_SET 鐨勫彲閫夊洖澶?
  `ETHTOOL_MSG_FEATURES_NTF`             缃戠粶璁惧鐗规€ч€氱煡
  `ETHTOOL_MSG_PRIVFLAGS_GET_REPLY`      绉佹湁鏍囧織
  `ETHTOOL_MSG_PRIVFLAGS_NTF`            绉佹湁鏍囧織
  `ETHTOOL_MSG_RINGS_GET_REPLY`          鐜舰闃熷垪澶у皬
  `ETHTOOL_MSG_RINGS_NTF`                鐜舰闃熷垪澶у皬
  `ETHTOOL_MSG_CHANNELS_GET_REPLY`       閫氶亾鏁伴噺
  `ETHTOOL_MSG_CHANNELS_NTF`             閫氶亾鏁伴噺
  `ETHTOOL_MSG_COALESCE_GET_REPLY`       涓柇鑱氬悎鍙傛暟
  `ETHTOOL_MSG_COALESCE_NTF`             涓柇鑱氬悎鍙傛暟
  `ETHTOOL_MSG_PAUSE_GET_REPLY`          鏆傚仠鍙傛暟
  `ETHTOOL_MSG_PAUSE_NTF`                鏆傚仠鍙傛暟
  `ETHTOOL_MSG_EEE_GET_REPLY`            EEE 璁剧疆
  `ETHTOOL_MSG_EEE_NTF`                  EEE 璁剧疆
  `ETHTOOL_MSG_TSINFO_GET_REPLY`         鏃堕棿鎴充俊鎭?
  `ETHTOOL_MSG_CABLE_TEST_NTF`           绾跨紗娴嬭瘯缁撴灉
  `ETHTOOL_MSG_CABLE_TEST_TDR_NTF`       绾跨紗娴嬭瘯 TDR 缁撴灉
  `ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY`    闅ч亾鍗歌浇淇℃伅
  `ETHTOOL_MSG_FEC_GET_REPLY`            FEC 璁剧疆
  `ETHTOOL_MSG_FEC_NTF`                  FEC 璁剧疆
  `ETHTOOL_MSG_MODULE_EEPROM_GET_REPLY`  璇诲彇 SFP 妯″潡 EEPROM
  `ETHTOOL_MSG_STATS_GET_REPLY`          鏍囧噯缁熻淇℃伅
  `ETHTOOL_MSG_PHC_VCLOCKS_GET_REPLY`     PHC 铏氭嫙鏃堕挓淇℃伅
  `ETHTOOL_MSG_MODULE_GET_REPLY`         鏀跺彂鍣ㄦā鍧楀弬鏁?
  `ETHTOOL_MSG_PSE_GET_REPLY`            PSE 鍙傛暟
  `ETHTOOL_MSG_RSS_GET_REPLY`            RSS 璁剧疆
  `ETHTOOL_MSG_RSS_NTF`                  RSS 璁剧疆
  `ETHTOOL_MSG_PLCA_GET_CFG_REPLY`       PLCA RS 鍙傛暟
  `ETHTOOL_MSG_PLCA_GET_STATUS_REPLY`    PLCA RS 鐘舵€?
  `ETHTOOL_MSG_PLCA_NTF`                 PLCA RS 鍙傛暟
  `ETHTOOL_MSG_MM_GET_REPLY`             MAC 鍚堝苟灞傜姸鎬?
  `ETHTOOL_MSG_MODULE_FW_FLASH_NTF`      鏀跺彂鍣ㄦā鍧楀浐浠舵洿鏂?
  `ETHTOOL_MSG_PHY_GET_REPLY`            浠ュお缃?PHY 淇℃伅
  `ETHTOOL_MSG_PHY_NTF`                  浠ュお缃?PHY 淇℃伅鍙樻洿
  `ETHTOOL_MSG_TSCONFIG_GET_REPLY`       纭欢鏃堕棿鎴抽厤缃?
  `ETHTOOL_MSG_TSCONFIG_SET_REPLY`       鏂扮殑纭欢鏃堕棿鎴抽厤缃?
  `ETHTOOL_MSG_PSE_NTF`                  PSE 浜嬩欢閫氱煡
  `ETHTOOL_MSG_RSS_NTF`                  RSS 璁剧疆閫氱煡
  `ETHTOOL_MSG_RSS_CREATE_ACT_REPLY`     鍒涘缓棰濆鐨?RSS 涓婁笅鏂?
  `ETHTOOL_MSG_RSS_CREATE_NTF`           宸插垱寤洪澶栫殑 RSS 涓婁笅鏂?
  `ETHTOOL_MSG_RSS_DELETE_NTF`           宸插垹闄ら澶栫殑 RSS 涓婁笅鏂?
  `ETHTOOL_MSG_MSE_GET_REPLY`            MSE 璇婃柇鏁版嵁
  ======================================== =================================

`GET` 璇锋眰鐢辩敤鎴风┖闂村簲鐢ㄧ▼搴忓彂鍑猴紝鐢ㄤ簬鑾峰彇璁惧淇℃伅銆傚畠浠€氬父涓嶅寘鍚换浣曟秷鎭壒瀹氱殑
灞炴€с€傚唴鏍搁€氳繃鐩稿簲鐨勨€淕ET_REPLY鈥濇秷鎭洖澶嶃€傚浜庡ぇ澶氭暟绫诲瀷锛屼笉甯﹁澶囨爣璇嗐€佸苟璁剧疆
`NLM_F_DUMP` 鐨?`GET` 璇锋眰鍙敤浜庢煡璇㈡墍鏈夋敮鎸佽璇锋眰鐨勮澶囩殑瀵瑰簲淇℃伅銆?

濡傛灉鏁版嵁涔熷彲浠ヨ淇敼锛屽垯浣跨敤鍏锋湁鐩稿悓甯冨眬锛堜笌鐩稿簲 `GET_REPLY` 涓€鑷达級鐨?`SET` 娑堟伅鏉?
璇锋眰鏇存敼銆傛绫昏姹備腑浠呭寘鍚姹備簡鏇存敼鐨勫睘鎬э紙褰撶劧锛屼篃骞堕潪鎵€鏈夊睘鎬ч兘鍙鏇存敼锛夈€傚澶у鏁?
`SET` 璇锋眰鐨勫洖澶嶄粎鍖呭惈閿欒鐮佷笌 extack锛涜嫢鍐呮牳鎻愪緵棰濆鏁版嵁锛屽垯浼氫互鐩稿簲 `SET_REPLY`
娑堟伅鐨勫舰寮忓彂閫侊紝鍙€氳繃鍦ㄨ姹傚ご涓缃?`ETHTOOL_FLAG_OMIT_REPLY` 鏍囧織鏉ユ姂鍒惰鍥炲銆?

鏁版嵁淇敼杩樹細瑙﹀彂鍙戦€佷竴鏉″寘鍚€氱煡鐨?`NTF` 娑堟伅銆傝繖浜涙秷鎭€氬父鍙惡甯﹀彈璇ユ洿鏀瑰奖鍝嶇殑灞炴€?
瀛愰泦銆傚鏋滀娇鐢ㄥ叾浠栨柟寮忥紙涓昏鏄?ioctl ethtool 鎺ュ彛锛変慨鏀逛簡鏁版嵁锛屼篃浼氬彂鍑虹浉鍚岀殑閫氱煡銆?
涓庝粎鍦ㄦ暟鎹疄闄呭彂鐢熷彉鍖栨椂鎵嶅彂閫佺殑 ethtool netlink 浠ｇ爜閫氱煡涓嶅悓锛岀敱 ioctl 鎺ュ彛瑙﹀彂鐨?
閫氱煡鍗充娇璇锋眰瀹為檯涓婃病鏈夋敼鍙樹换浣曟暟鎹篃鍙兘琚彂閫併€?

`ACT` 娑堟伅璇锋眰鍐呮牳锛堥┍鍔級鎵ц鏌愪釜鐗瑰畾鍔ㄤ綔銆傚鏋滃唴鏍镐笂鎶ヤ簡鏌愪簺淇℃伅锛堝彲閫氳繃鍦ㄨ姹傚ご涓?
璁剧疆 `ETHTOOL_FLAG_OMIT_REPLY` 鏍囧織鏉ユ姂鍒讹級锛屽垯璇ュ洖澶嶄互 `ACT_REPLY` 娑堟伅鐨勫舰寮忓憟鐜般€?
鎵ц鍔ㄤ綔杩樹細瑙﹀彂涓€鏉￠€氱煡锛坄NTF` 娑堟伅锛夈€?

鍚庣画绔犺妭灏嗘弿杩拌繖浜涙秷鎭殑鏍煎紡涓庤涔夈€?


## STRSET_GET


璇锋眰 ioctl 鍛戒护 `ETHTOOL_GSSET_INFO` 涓?`ETHTOOL_GSTRINGS` 鎵€鎻愪緵鐨勫瓧绗︿覆闆嗗唴瀹广€?
瀛楃涓查泦涓嶅彲鐢辩敤鎴峰啓鍏ワ紝鍥犳鐩稿簲鐨?`STRSET_SET` 娑堟伅浠呭湪鍐呮牳鍥炲涓娇鐢ㄣ€傚瓧绗︿覆闆嗗垎涓?
涓ょ被锛氬叏灞€鐨勶紙涓庤澶囨棤鍏筹紝渚嬪璁惧鐗规€у悕绉帮級涓庤澶囩壒瀹氱殑锛堜緥濡傝澶囩鏈夋爣蹇楋級銆?

璇锋眰鍐呭锛?

 +---------------------------------------+--------+------------------------+
 | `ETHTOOL_A_STRSET_HEADER`           | nested | request header         |
 +---------------------------------------+--------+------------------------+
 | `ETHTOOL_A_STRSET_STRINGSETS`       | nested | string set to request  |
 +-+-------------------------------------+--------+------------------------+
 | | `ETHTOOL_A_STRINGSETS_STRINGSET+` | nested | one string set         |
 +-+-+-----------------------------------+--------+------------------------+
 | | | `ETHTOOL_A_STRINGSET_ID`        | u32    | set id                 |
 +-+-+-----------------------------------+--------+------------------------+

鍐呮牳鍝嶅簲鍐呭锛?

 +---------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_HEADER`           | nested | reply header          |
 +---------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_STRINGSETS`       | nested | array of string sets  |
 +-+-------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_STRINGSETS_STRINGSET+` | nested | one string set        |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_ID`        | u32    | set id                |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_COUNT`     | u32    | number of strings     |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_STRINGS`   | nested | array of strings      |
 +-+-+-+---------------------------------+--------+-----------------------+
 | | | | `ETHTOOL_A_STRINGS_STRING+`   | nested | one string            |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | | | | | `ETHTOOL_A_STRING_INDEX`    | u32    | string index          |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | | | | | `ETHTOOL_A_STRING_VALUE`    | string | string value          |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_COUNTS_ONLY`      | flag   | return only counts    |
 +---------------------------------------+--------+-----------------------+

璇锋眰澶翠腑鐨勮澶囨爣璇嗘槸鍙€夌殑銆傛牴鎹叾鏄惁瀛樺湪浠ュ強 `NLM_F_DUMP` 鏍囧織锛屽瓨鍦ㄤ笁绉嶇被鍨嬬殑
`STRSET_GET` 璇锋眰锛?

 - 鏃?`NLM_F_DUMP,` 鏃犺澶囷細鑾峰彇鈥滃叏灞€鈥濆瓧绗︿覆闆?
 - 鏃?`NLM_F_DUMP`锛屽甫璁惧锛氳幏鍙栦笌璇ヨ澶囩浉鍏崇殑瀛楃涓查泦
 - `NLM_F_DUMP`锛屾棤璁惧锛氳幏鍙栨墍鏈夎澶囩殑璁惧鐩稿叧瀛楃涓查泦

濡傛灉娌℃湁 `ETHTOOL_A_STRSET_STRINGSETS` 鏁扮粍锛屽垯杩斿洖鎵€鏈夎姹傜被鍨嬬殑瀛楃涓查泦锛屽惁鍒欎粎
杩斿洖璇锋眰涓寚瀹氱殑閭ｄ簺銆俙ETHTOOL_A_STRSET_COUNTS_ONLY` 鏍囧織鍛婄煡鍐呮牳鍙繑鍥炲瓧绗︿覆闆嗙殑
璁℃暟锛岃€岄潪瀹為檯鐨勫瓧绗︿覆銆?


## LINKINFO_GET


璇锋眰閾捐矾璁剧疆锛屼笌 `ETHTOOL_GLINKSETTINGS` 鎻愪緵鐨勫唴瀹圭浉鍚岋紝浣嗕笉鍖呮嫭閾捐矾妯″紡涓庤嚜鍗忓晢
鐩稿叧鐨勪俊鎭€傝璇锋眰涓嶄娇鐢ㄤ换浣曞睘鎬с€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  reply header
  `ETHTOOL_A_LINKINFO_PORT`           u8      physical port
  `ETHTOOL_A_LINKINFO_PHYADDR`        u8      phy MDIO address
  `ETHTOOL_A_LINKINFO_TP_MDIX`        u8      MDI(-X) status
  `ETHTOOL_A_LINKINFO_TP_MDIX_CTRL`   u8      MDI(-X) control
  `ETHTOOL_A_LINKINFO_TRANSCEIVER`    u8      transceiver
  ====================================  ======  ==========================

鍚勫睘鎬у強鍏跺彇鍊间笌鐩稿簲 ioctl 缁撴瀯浣撲腑瀵瑰簲鐨勬垚鍛樺惈涔夌浉鍚屻€?

`LINKINFO_GET` 鍏佽 dump 璇锋眰锛堝唴鏍镐负鎵€鏈夋敮鎸佽璇锋眰鐨勮澶囪繑鍥炲洖澶嶆秷鎭級銆?


## LINKINFO_SET


`LINKINFO_SET` 璇锋眰鍏佽璁剧疆 `LINKINFO_GET` 鎵€涓婃姤鐨勯儴鍒嗗睘鎬с€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  request header
  `ETHTOOL_A_LINKINFO_PORT`           u8      physical port
  `ETHTOOL_A_LINKINFO_PHYADDR`        u8      phy MDIO address
  `ETHTOOL_A_LINKINFO_TP_MDIX_CTRL`   u8      MDI(-X) control
  ====================================  ======  ==========================

MDI(-X) 鐘舵€佷笌鏀跺彂鍣ㄤ笉鍙缃紝鎼哄甫鐩稿簲灞炴€х殑璇锋眰灏嗚鎷掔粷銆?


## LINKMODES_GET


璇锋眰閾捐矾妯″紡锛堟敮鎸佺殑銆侀€氬憡鐨勪互鍙婂绔€氬憡鐨勶級浠ュ強鐩稿叧淇℃伅锛堣嚜鍗忓晢鐘舵€併€侀摼璺€熺巼涓庡弻宸ワ級锛?
涓?`ETHTOOL_GLINKSETTINGS` 鎻愪緵鐨勫唴瀹圭浉鍚屻€傝璇锋眰涓嶄娇鐢ㄤ换浣曞睘鎬с€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`        nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ==========================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`              nested  reply header
  `ETHTOOL_A_LINKMODES_AUTONEG`             u8      autonegotiation status
  `ETHTOOL_A_LINKMODES_OURS`                bitset  advertised link modes
  `ETHTOOL_A_LINKMODES_PEER`                bitset  partner link modes
  `ETHTOOL_A_LINKMODES_SPEED`               u32     link speed (Mb/s)
  `ETHTOOL_A_LINKMODES_DUPLEX`              u8      duplex mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG`    u8      Master/slave port mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_STATE`  u8      Master/slave port state
  `ETHTOOL_A_LINKMODES_RATE_MATCHING`       u8      PHY rate matching
  ==========================================  ======  ==========================

瀵逛簬 `ETHTOOL_A_LINKMODES_OURS`锛屽€艰〃绀洪€氬憡鐨勬ā寮忥紝鎺╃爜琛ㄧず鏀寔鐨勬ā寮忋€傚洖澶嶄腑鐨?
`ETHTOOL_A_LINKMODES_PEER` 鏄竴涓綅鍒楄〃銆?

`LINKMODES_GET` 鍏佽 dump 璇锋眰锛堝唴鏍镐负鎵€鏈夋敮鎸佽璇锋眰鐨勮澶囪繑鍥炲洖澶嶆秷鎭級銆?


## LINKMODES_SET


璇锋眰鍐呭锛?

  ==========================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`              nested  request header
  `ETHTOOL_A_LINKMODES_AUTONEG`             u8      autonegotiation status
  `ETHTOOL_A_LINKMODES_OURS`                bitset  advertised link modes
  `ETHTOOL_A_LINKMODES_PEER`                bitset  partner link modes
  `ETHTOOL_A_LINKMODES_SPEED`               u32     link speed (Mb/s)
  `ETHTOOL_A_LINKMODES_DUPLEX`              u8      duplex mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG`    u8      Master/slave port mode
  `ETHTOOL_A_LINKMODES_RATE_MATCHING`       u8      PHY rate matching
  `ETHTOOL_A_LINKMODES_LANES`               u32     lanes
  ==========================================  ======  ==========================

`ETHTOOL_A_LINKMODES_OURS` 浣嶉泦鍏佽璁剧疆閫氬憡鐨勯摼璺ā寮忋€傚鏋滆嚜鍗忓晢澶勪簬寮€鍚姸鎬侊紙鏃犺鏄?
鏈璁剧疆杩樻槸娌跨敤涔嬪墠鐨勮缃級锛屼笖閫氬憡妯″紡鏈敼鍙橈紙鏃?`ETHTOOL_A_LINKMODES_OURS` 灞炴€э級锛?
骞朵笖鑷冲皯鎸囧畾浜嗛€熺巼銆佸弻宸ヤ笌閫氶亾锛坙anes锛変腑鐨勪竴椤癸紝鍐呮牳浼氬皢閫氬憡妯″紡璋冩暣涓烘墍鏈夊尮閰嶆墍鎸囧畾
鐨勯€熺巼銆佸弻宸ャ€侀€氶亾锛堟垨鍏ㄩ儴锛岃鎸囧畾鎯呭喌鑰屽畾锛夌殑鏀寔妯″紡銆傝繖绉嶈嚜鍔ㄩ€夋嫨鏄湪浣跨敤 ioctl 鎺ュ彛
鏃剁敱 ethtool 涓€渚у畬鎴愮殑锛沶etlink 鎺ュ彛鍒欐棬鍦ㄥ厑璁稿湪涓嶇煡閬撳唴鏍稿叿浣撴敮鎸佷粈涔堢殑鎯呭喌涓嬭姹傛洿鏀广€?


## LINKSTATE_GET


璇锋眰閾捐矾鐘舵€佷俊鎭€傛彁渚涗簡閾捐矾 up/down 鏍囧織锛堢敱 `ETHTOOL_GLINK` ioctl 鍛戒护鎻愪緵锛夈€?
鍙€夊湴锛屼篃鍙兘鎻愪緵鎵╁睍鐘舵€併€傛€讳綋涓婏紝鎵╁睍鐘舵€佹弿杩颁簡绔彛涓轰綍澶勪簬 down 鐘舵€侊紝鎴栦负浣曚互
鏌愮闈炴樉鑰屾槗瑙佺殑鏂瑰紡杩愯銆傝璇锋眰娌℃湁浠讳綍灞炴€с€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKSTATE_HEADER`        nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ============================
  `ETHTOOL_A_LINKSTATE_HEADER`        nested  reply header
  `ETHTOOL_A_LINKSTATE_LINK`          bool    link state (up/down)
  `ETHTOOL_A_LINKSTATE_SQI`           u32     Current Signal Quality Index
  `ETHTOOL_A_LINKSTATE_SQI_MAX`       u32     Max support SQI value
  `ETHTOOL_A_LINKSTATE_EXT_STATE`     u8      link extended state
  `ETHTOOL_A_LINKSTATE_EXT_SUBSTATE`  u8      link extended substate
  `ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT`  u32     count of link down events
  ====================================  ======  ============================

瀵逛簬澶у鏁?NIC 椹卞姩锛宍ETHTOOL_A_LINKSTATE_LINK` 鐨勫€艰繑鍥炵敱 `netif_carrier_ok()`
鎻愪緵鐨勮浇娉㈡爣蹇楋紝浣嗕篃瀛樺湪鑷瀹氫箟澶勭悊鍑芥暟鐨勯┍鍔ㄣ€?

`ETHTOOL_A_LINKSTATE_EXT_STATE` 涓?`ETHTOOL_A_LINKSTATE_EXT_SUBSTATE` 涓哄彲閫夊€笺€?
ethtool 鏍稿績鍙互鏃㈡彁渚?`ETHTOOL_A_LINKSTATE_EXT_STATE` 鍙堟彁渚?
`ETHTOOL_A_LINKSTATE_EXT_SUBSTATE`锛屾垨鍙彁渚?`ETHTOOL_A_LINKSTATE_EXT_STATE`锛屾垨
涓よ€呴兘涓嶆彁渚涖€?

`LINKSTATE_GET` 鍏佽 dump 璇锋眰锛堝唴鏍镐负鎵€鏈夋敮鎸佽璇锋眰鐨勮澶囪繑鍥炲洖澶嶆秷鎭級銆?


閾捐矾鎵╁睍鐘舵€侊細

  ================================================      ============================================
  `ETHTOOL_LINK_EXT_STATE_AUTONEG`                    涓庤嚜鍗忓晢鎴栧叾涓瓨鍦ㄧ殑闂鐩稿叧鐨勭姸鎬?

  `ETHTOOL_LINK_EXT_STATE_LINK_TRAINING_FAILURE`      閾捐矾璁粌鏈熼棿澶辫触

  `ETHTOOL_LINK_EXT_STATE_LINK_LOGICAL_MISMATCH`      鐗╃悊缂栫爜瀛愬眰鎴栧墠鍚戠籂閿欏瓙灞備腑鐨勯€昏緫涓嶅尮閰?

  `ETHTOOL_LINK_EXT_STATE_BAD_SIGNAL_INTEGRITY`       淇″彿瀹屾暣鎬ч棶棰?

  `ETHTOOL_LINK_EXT_STATE_NO_CABLE`                   鏈繛鎺ョ嚎缂?

  `ETHTOOL_LINK_EXT_STATE_CABLE_ISSUE`                鏁呴殰涓庣嚎缂嗙浉鍏筹紝渚嬪涓嶆敮鎸佺殑绾跨紗

  `ETHTOOL_LINK_EXT_STATE_EEPROM_ISSUE`               鏁呴殰涓?EEPROM 鐩稿叧锛屼緥濡傚湪璇诲彇鎴栬В鏋愭暟鎹椂澶辫触

  `ETHTOOL_LINK_EXT_STATE_CALIBRATION_FAILURE`        鏍″噯绠楁硶鏈熼棿澶辫触

  `ETHTOOL_LINK_EXT_STATE_POWER_BUDGET_EXCEEDED`      纭欢鏃犳硶鎻愪緵绾跨紗鎴栨ā鍧楁墍闇€鐨勫姛鐜?

  `ETHTOOL_LINK_EXT_STATE_OVERHEAT`                   妯″潡杩囩儹

  `ETHTOOL_LINK_EXT_STATE_MODULE`                     鏀跺彂鍣ㄦā鍧楅棶棰?
  ================================================      ============================================

閾捐矾鎵╁睍瀛愮姸鎬侊細

  鑷崗鍟嗗瓙鐘舵€侊細

  ===============================================================   ================================
  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_PARTNER_DETECTED`              瀵圭澶勪簬 down 鐘舵€?

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_ACK_NOT_RECEIVED`                 鏈敹鍒板绔殑 Ack

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NEXT_PAGE_EXCHANGE_FAILED`        涓嬩竴椤典氦鎹㈠け璐?

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_PARTNER_DETECTED_FORCE_MODE`   鍦ㄥ己鍒舵ā寮忔湡闂村绔浜?down 鐘舵€侊紝鎴栭€熺巼鏈揪鎴愪竴鑷?

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_FEC_MISMATCH_DURING_OVERRIDE`     鍙屾柟鐨勫墠鍚戠籂閿欐ā寮忎笉鍖归厤

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_HCD`                           鏃犳渶楂樺叕鍒嗘瘝锛圚ighest Common Denominator锛?
  ===============================================================   ================================

  閾捐矾璁粌瀛愮姸鎬侊細

  ===========================================================================   ====================
  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_FRAME_LOCK_NOT_ACQUIRED`                    甯ф湭琚瘑鍒紝閿佸畾澶辫触

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_LINK_INHIBIT_TIMEOUT`                       鍦ㄨ秴鏃跺墠鏈畬鎴愰攣瀹?

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_LINK_PARTNER_DID_NOT_SET_RECEIVER_READY`    璁粌杩囩▼鍚庡绔湭鍙戝嚭灏辩华淇″彿

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_REMOTE_FAULT`                                  杩滅灏氭湭灏辩华
  ===========================================================================   ====================

  閾捐矾閫昏緫涓嶅尮閰嶅瓙鐘舵€侊細

  ================================================================   ===============================
  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_ACQUIRE_BLOCK_LOCK`  鐗╃悊缂栫爜瀛愬眰鍦ㄧ涓€闃舵鏈攣瀹氣€斺€斿潡閿?

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_ACQUIRE_AM_LOCK`     鐗╃悊缂栫爜瀛愬眰鍦ㄧ浜岄樁娈垫湭閿佸畾鈥斺€斿榻愭爣璁伴攣

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_GET_ALIGN_STATUS`    鐗╃悊缂栫爜瀛愬眰鏈幏寰楀榻愮姸鎬?

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_FC_FEC_IS_NOT_LOCKED`            FC 鍓嶅悜绾犻敊鏈攣瀹?

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_RS_FEC_IS_NOT_LOCKED`            RS 鍓嶅悜绾犻敊鏈攣瀹?
  ================================================================   ===============================

  淇″彿瀹屾暣鎬у樊瀛愮姸鎬侊細

  =================================================================    =============================
  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_LARGE_NUMBER_OF_PHYSICAL_ERRORS`    澶ч噺鐗╃悊閿欒

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_UNSUPPORTED_RATE`                   绯荤粺灏濊瘯浠ヤ笉琚寮忔敮鎸佺殑閫熺巼杩愯绾跨紗锛屽鑷翠俊鍙峰畬鏁存€ч棶棰?

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_SERDES_REFERENCE_CLOCK_LOST`        SerDes 鐨勫閮ㄦ椂閽熶俊鍙疯繃寮辨垨涓嶅彲鐢?

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_SERDES_ALOS`                        SerDes 鐨勬帴鏀朵俊鍙峰洜妯℃嫙淇″彿涓㈠け鑰岃繃寮?
  =================================================================    =============================

  绾跨紗闂瀛愮姸鎬侊細

  ===================================================   ============================================
  `ETHTOOL_LINK_EXT_SUBSTATE_CI_UNSUPPORTED_CABLE`    涓嶆敮鎸佺殑绾跨紗

  `ETHTOOL_LINK_EXT_SUBSTATE_CI_CABLE_TEST_FAILURE`   绾跨紗娴嬭瘯澶辫触
  ===================================================   ============================================

  鏀跺彂鍣ㄦā鍧楅棶棰樺瓙鐘舵€侊細

  ===================================================   ============================================
  `ETHTOOL_LINK_EXT_SUBSTATE_MODULE_CMIS_NOT_READY`   CMIS 妯″潡鐘舵€佹満鏈埌杈?ModuleReady 鐘舵€侊紝渚嬪妯″潡鍋滅暀鍦?ModuleFault 鐘舵€?
  ===================================================   ============================================

## DEBUG_GET


璇锋眰璁惧鐨勮皟璇曡缃€傜洰鍓嶄粎鎻愪緵娑堟伅鎺╃爜銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  reply header
  `ETHTOOL_A_DEBUG_MSGMASK`           bitset  message mask
  ====================================  ======  ==========================

娑堟伅鎺╃爜锛坄ETHTOOL_A_DEBUG_MSGMASK`锛夌瓑鍚屼簬 ioctl 鎺ュ彛涓敱 `ETHTOOL_GMSGLVL` 鎻愪緵銆?
骞剁敱 `ETHTOOL_SMSGLVL` 璁剧疆鐨勬秷鎭骇鍒€傝櫧鐒跺嚭浜庡巻鍙插師鍥犲湪閭ｉ噷琚О涓烘秷鎭骇鍒紝浣嗗ぇ澶氭暟
椹卞姩浠ュ強鍑犱箮鎵€鏈夎緝鏂扮殑椹卞姩閮藉皢鍏剁敤浣滃惎鐢ㄦ秷鎭被鍒殑鎺╃爜锛堢敱 `NETIF_MSG_*` 甯搁噺琛ㄧず锛夛紱
鍥犳 netlink 鎺ュ彛閬靛惊鍏跺疄闄呯敤娉曘€?

`DEBUG_GET` 鍏佽 dump 璇锋眰锛堝唴鏍镐负鎵€鏈夋敮鎸佽璇锋眰鐨勮澶囪繑鍥炲洖澶嶆秷鎭級銆?


## DEBUG_SET


璁剧疆鎴栨洿鏂拌澶囩殑璋冭瘯璁剧疆銆傜洰鍓嶄粎鏀寔娑堟伅鎺╃爜銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  request header
  `ETHTOOL_A_DEBUG_MSGMASK`           bitset  message mask
  ====================================  ======  ==========================

`ETHTOOL_A_DEBUG_MSGMASK` 浣嶉泦鍏佽璁剧疆鎴栦慨鏀硅澶囧凡鍚敤鐨勮皟璇曟秷鎭被鍨嬬殑鎺╃爜銆?


## WOL_GET


鏌ヨ璁惧鐨勫敜閱掑眬鍩熺綉锛坵ake-on-lan锛夎缃€備笌澶у鏁扳€淕ET鈥濈被鍨嬬殑璇锋眰涓嶅悓锛?
`ETHTOOL_MSG_WOL_GET` 闇€瑕侊紙netns 鐨勶級`CAP_NET_ADMIN` 鏉冮檺锛屽洜涓哄畠锛堝彲鑳斤級浼氭彁渚?
淇濆瘑鐨?SecureOn(tm) 瀵嗙爜銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  reply header
  `ETHTOOL_A_WOL_MODES`               bitset  mask of enabled WoL modes
  `ETHTOOL_A_WOL_SOPASS`              binary  SecureOn(tm) password
  ====================================  ======  ==========================

鍦ㄥ洖澶嶄腑锛宍ETHTOOL_A_WOL_MODES` 鎺╃爜鐢辫澶囨敮鎸佺殑妯″紡锛屼互鍙婂叾涓凡鍚敤鐨勬ā寮忓€肩粍鎴愩€?
浠呭綋鏀寔 `WAKE_MAGICSECURE` 妯″紡鏃讹紝`ETHTOOL_A_WOL_SOPASS` 鎵嶄細鍖呭惈鍦ㄥ洖澶嶄腑銆?


## WOL_SET


璁剧疆鎴栨洿鏂板敜閱掑眬鍩熺綉锛坵ake-on-lan锛夎缃€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  request header
  `ETHTOOL_A_WOL_MODES`               bitset  enabled WoL modes
  `ETHTOOL_A_WOL_SOPASS`              binary  SecureOn(tm) password
  ====================================  ======  ==========================

`ETHTOOL_A_WOL_SOPASS` 浠呭厑璁哥敤浜庢敮鎸?`WAKE_MAGICSECURE` 妯″紡鐨勮澶囥€?


## FEATURES_GET


鑾峰彇缃戠粶璁惧鐗规€э紝绫讳技浜?`ETHTOOL_GFEATURES` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  reply header
  `ETHTOOL_A_FEATURES_HW`             bitset  dev->hw_features
  `ETHTOOL_A_FEATURES_WANTED`         bitset  dev->wanted_features
  `ETHTOOL_A_FEATURES_ACTIVE`         bitset  dev->features
  `ETHTOOL_A_FEATURES_NOCHANGE`       bitset  NETIF_F_NEVER_CHANGE
  ====================================  ======  ==========================


鍐呮牳鍝嶅簲涓殑浣嶅浘涓?ioctl 鎺ュ彛涓娇鐢ㄧ殑浣嶅浘鍚箟鐩稿悓锛屼絾灞炴€у悕绉颁笉鍚岋紙瀹冧滑鍩轰簬
struct net_device 鐨勫搴旀垚鍛橈級銆傛棫寮忕殑鈥渇lags鈥濅笉浼氳鎻愪緵锛涘鏋滅敤鎴风┖闂撮渶瑕佸畠浠?
锛堝緢鍙兘鍙湁 ethtool 涓轰簡鍚戝悗鍏煎锛夛紝瀹冨彲浠ユ牴鎹浉鍏崇殑鐗规€т綅鑷璁＄畻鍏跺€笺€?
ETHA_FEATURES_HW 浣跨敤鐨勬帺鐮佺敱鍐呮牳璇嗗埆鐨勬墍鏈夌壒鎬х粍鎴愶紙浠ヤ究鍦ㄤ娇鐢ㄨ缁嗕綅鍥炬牸寮忔椂鎻愪緵
鍏ㄩ儴鍚嶇О锛夛紝鍏朵綑涓変釜鍒欎笉浣跨敤鎺╃爜锛堜粎浣滀负绠€鍗曠殑浣嶅垪琛級銆?


## FEATURES_SET


璇锋眰璁剧疆缃戠粶璁惧鐗规€э紝绫讳技浜?`ETHTOOL_SFEATURES` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  request header
  `ETHTOOL_A_FEATURES_WANTED`         bitset  requested features
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  reply header
  `ETHTOOL_A_FEATURES_WANTED`         bitset  diff wanted vs. result
  `ETHTOOL_A_FEATURES_ACTIVE`         bitset  diff old vs. new active
  ====================================  ======  ==========================

璇锋眰涓彧鍖呭惈涓€涓綅闆嗭紝瀹冨彲浠ユ槸鍊?鎺╃爜瀵癸紙璇锋眰鏇存敼鐗瑰畾鐨勭壒鎬т綅鑰屼繚鐣欏叾浣欙級鎴栦粎涓€涓€?
锛堣姹傚皢鎵€鏈夌壒鎬ц缃负鎸囧畾鐨勯泦鍚堬級銆?

鐢变簬璇锋眰瑕佹帴鍙?netdev_change_features() 鐨勫悎鐞嗘€ф鏌ワ紝鍙€夌殑鍐呮牳鍥炲锛堝彲閫氳繃璇锋眰澶翠腑鐨?
`ETHTOOL_FLAG_OMIT_REPLY` 鏍囧織鎶戝埗锛変細鍛婄煡瀹㈡埛绔疄闄呯粨鏋溿€俙ETHTOOL_A_FEATURES_WANTED`
鎶ュ憡瀹㈡埛绔姹備笌瀹為檯缁撴灉涔嬮棿鐨勫樊寮傦細鎺╃爜鐢辫姹傜殑鐗规€т笌缁撴灉锛堟搷浣滃悗 dev->features锛変箣闂?
涓嶅悓鐨勪綅缁勬垚锛屽€肩敱杩欎簺浣嶅湪璇锋眰涓殑鍙栧€肩粍鎴愶紙鍗虫潵鑷粨鏋滅壒鎬х殑鍙栧弽鍊硷級銆?
`ETHTOOL_A_FEATURES_ACTIVE` 鎶ュ憡鏂版棫 dev->features 涔嬮棿鐨勫樊寮傦細鎺╃爜鐢卞彂鐢熷彉鍖栫殑浣嶇粍鎴愶紝
鍊间负杩欎簺浣嶅湪鏂扮殑 dev->features锛堟搷浣滃悗锛変腑鐨勫彇鍊笺€?

`ETHTOOL_MSG_FEATURES_NTF` 閫氱煡涓嶄粎鍦ㄩ€氳繃 `ETHTOOL_MSG_FEATURES_SET` 璇锋眰鎴栦慨鏀?
ethtool ioctl 璇锋眰鏉ヤ慨鏀硅澶囩壒鎬ф椂鍙戦€侊紝涔熶細鍦ㄦ瘡娆￠€氳繃 netdev_update_features() 鎴?
netdev_change_features() 淇敼鐗规€ф椂鍙戦€併€?


## PRIVFLAGS_GET


鑾峰彇绉佹湁鏍囧織锛岀被浼间簬 `ETHTOOL_GPFLAGS` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  reply header
  `ETHTOOL_A_PRIVFLAGS_FLAGS`         bitset  private flags
  ====================================  ======  ==========================

`ETHTOOL_A_PRIVFLAGS_FLAGS` 鏄竴涓甫鏈夎澶囩鏈夋爣蹇楀€肩殑浣嶉泦銆傝繖浜涙爣蹇楃敱椹卞姩瀹氫箟锛屽叾鏁伴噺
涓庡悕绉帮紙浠ュ強鍚箟锛夊彇鍐充簬鍏蜂綋璁惧銆傚浜庣揣鍑戜綅闆嗘牸寮忥紝鍚嶇О鍙€氳繃 `ETH_SS_PRIV_FLAGS`
瀛楃涓查泦鑾峰彇銆傚鏋滆姹備簡璇︾粏浣嶉泦鏍煎紡锛屽搷搴斾細浣跨敤璁惧鏀寔鐨勫叏閮ㄧ鏈夋爣蹇椾綔涓烘帺鐮侊紝浠庤€?
瀹㈡埛绔棤闇€鍐嶅幓鑾峰彇甯︽湁鍚嶇О鐨勫瓧绗︿覆闆嗗嵆鍙幏寰楀畬鏁翠俊鎭€?


## PRIVFLAGS_SET


璁剧疆鎴栦慨鏀硅澶囩鏈夋爣蹇楃殑鍊硷紝绫讳技浜?`ETHTOOL_SPFLAGS` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  request header
  `ETHTOOL_A_PRIVFLAGS_FLAGS`         bitset  private flags
  ====================================  ======  ==========================

`ETHTOOL_A_PRIVFLAGS_FLAGS` 鏃㈠彲浠ヨ缃暣涓鏈夋爣蹇楅泦鍚堬紝涔熷彲浠ュ彧淇敼鍏朵腑閮ㄥ垎鏍囧織鐨勫€笺€?


## RINGS_GET


鑾峰彇鐜舰闃熷垪澶у皬锛岀被浼间簬 `ETHTOOL_GRINGPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_RINGS_HEADER`            nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  =======================================   ======  ===========================
  `ETHTOOL_A_RINGS_HEADER`                nested  reply header
  `ETHTOOL_A_RINGS_RX_MAX`                u32     鎺ユ敹锛圧X锛夌幆褰㈤槦鍒楁渶澶уぇ灏?
  `ETHTOOL_A_RINGS_RX_MINI_MAX`           u32     RX mini 鐜舰闃熷垪鏈€澶уぇ灏?
  `ETHTOOL_A_RINGS_RX_JUMBO_MAX`          u32     RX jumbo 鐜舰闃熷垪鏈€澶уぇ灏?
  `ETHTOOL_A_RINGS_TX_MAX`                u32     鍙戦€侊紙TX锛夌幆褰㈤槦鍒楁渶澶уぇ灏?
  `ETHTOOL_A_RINGS_RX`                    u32     RX 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_MINI`               u32     RX mini 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_JUMBO`              u32     RX jumbo 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_TX`                    u32     TX 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_BUF_LEN`            u32     鐜舰闃熷垪涓婄紦鍐插尯鐨勫ぇ灏?
  `ETHTOOL_A_RINGS_TCP_DATA_SPLIT`        u8      TCP 澶?/ 鏁版嵁鍒嗙
  `ETHTOOL_A_RINGS_CQE_SIZE`              u32     TX/RX CQE 鐨勫ぇ灏?
  `ETHTOOL_A_RINGS_TX_PUSH`               u8      TX Push 妯″紡鏍囧織
  `ETHTOOL_A_RINGS_RX_PUSH`               u8      RX Push 妯″紡鏍囧織
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN`       u32     TX push 缂撳啿鍖哄ぇ灏?
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN_MAX`   u32     TX push 缂撳啿鍖烘渶澶уぇ灏?
  `ETHTOOL_A_RINGS_HDS_THRESH`            u32     澶?/ 鏁版嵁鍒嗙闃堝€?
  `ETHTOOL_A_RINGS_HDS_THRESH_MAX`        u32     澶?/ 鏁版嵁鍒嗙鏈€澶ч槇鍊?
  =======================================   ======  ===========================

`ETHTOOL_A_RINGS_TCP_DATA_SPLIT` 鎸囩ず璇ヨ澶囨槸鍚﹀彲涓庨〉缈昏浆锛坧age-flipping锛夌殑 TCP 闆舵嫹璐?
鎺ユ敹锛坄getsockopt(TCP_ZEROCOPY_RECEIVE)`锛夐厤鍚堜娇鐢ㄣ€傝嫢鍚敤锛岃澶囪閰嶇疆涓哄皢甯уご涓庢暟鎹?
鏀惧叆鐙珛鐨勭紦鍐插尯銆傝澶囬厤缃繀椤昏兘澶熸帴鏀跺畬鏁寸殑鍐呭瓨椤垫暟鎹紝渚嬪鍥犱负 MTU 瓒冲澶ф垨閫氳繃
HW-GRO銆?

`ETHTOOL_A_RINGS_[RX|TX]_PUSH` 鏍囧織鐢ㄤ簬鍚敤鎻忚堪绗﹀揩閫熻矾寰勬潵鍙戦€佹垨鎺ユ敹鏁版嵁鍖呫€傚湪鏅€氳矾寰?
涓紝椹卞姩鍦?DRAM 涓～鍏呮弿杩扮骞堕€氱煡 NIC 纭欢銆傚湪蹇€熻矾寰勪腑锛岄┍鍔ㄩ€氳繃 MMIO 鍐欐搷浣滃皢鎻忚堪绗?
鎺ㄩ€佸埌璁惧锛屼粠鑰岄檷浣庡欢杩熴€傜劧鑰岋紝鍚敤璇ョ壒鎬у彲鑳藉鍔?CPU 寮€閿€銆傞┍鍔ㄥ彲鑳戒細鏂藉姞棰濆鐨勯€愬寘
璧勬牸妫€鏌ワ紙渚嬪渚濇嵁鍖呭ぇ灏忥級銆?

`ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN` 鎸囧畾椹卞姩鍙互鐩存帴鎺ㄩ€佸埌搴曞眰璁惧锛堚€榩ush鈥欐ā寮忥級鐨勫彂閫佸寘
鐨勬渶澶у瓧鑺傛暟銆傚皢閮ㄥ垎杞借嵎瀛楄妭鎺ㄩ€佸埌璁惧鍏锋湁鍑忓皯灏忓寘寤惰繜锛堥伩鍏?DMA 鏄犲皠锛屼笌
`ETHTOOL_A_RINGS_TX_PUSH` 鍙傛暟鐩稿悓锛変互鍙婂厑璁稿簳灞傝澶囧湪澶勭悊鍏惰浇鑽蜂箣鍓嶅厛澶勭悊鍖呭ご鐨勪紭鍔裤€?
杩欏彲浠ュ府鍔╄澶囧熀浜庡寘澶村揩閫熼噰鍙栬鍔ㄣ€傝繖涓庘€渢x-copybreak鈥濆弬鏁扮被浼硷紝鍚庤€呭皢鍖呭鍒跺埌棰勫垎閰嶇殑
DMA 鍐呭瓨鍖哄煙鑰岄潪鏄犲皠鏂板唴瀛樸€傜劧鑰岋紝tx-push-buff 鍙傛暟灏嗗寘鐩存帴澶嶅埗鍒拌澶囷紝浠ヨ璁惧鑳藉鍖?
閲囧彇鏇村揩鐨勫姩浣溿€?

## RINGS_SET


璁剧疆鐜舰闃熷垪澶у皬锛岀被浼间簬 `ETHTOOL_SRINGPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ===========================
  `ETHTOOL_A_RINGS_HEADER`            nested  reply header
  `ETHTOOL_A_RINGS_RX`                u32     RX 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_MINI`           u32     RX mini 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_JUMBO`          u32     RX jumbo 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_TX`                u32     TX 鐜舰闃熷垪澶у皬
  `ETHTOOL_A_RINGS_RX_BUF_LEN`        u32     鐜舰闃熷垪涓婄紦鍐插尯鐨勫ぇ灏?
  `ETHTOOL_A_RINGS_TCP_DATA_SPLIT`    u8      TCP 澶?/ 鏁版嵁鍒嗙
  `ETHTOOL_A_RINGS_CQE_SIZE`          u32     TX/RX CQE 鐨勫ぇ灏?
  `ETHTOOL_A_RINGS_TX_PUSH`           u8      TX Push 妯″紡鏍囧織
  `ETHTOOL_A_RINGS_RX_PUSH`           u8      RX Push 妯″紡鏍囧織
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN`   u32     TX push 缂撳啿鍖哄ぇ灏?
  `ETHTOOL_A_RINGS_HDS_THRESH`        u32     澶?/ 鏁版嵁鍒嗙闃堝€?
  ====================================  ======  ===========================

鍐呮牳浼氭鏌ヨ姹傜殑鐜舰闃熷垪澶у皬涓嶈秴杩囬┍鍔ㄤ笂鎶ョ殑闄愬埗銆傞┍鍔ㄥ彲鑳芥柦鍔犻澶栫殑绾︽潫锛屼篃鍙兘涓嶆敮鎸?
鎵€鏈夊睘鎬с€?


`ETHTOOL_A_RINGS_CQE_SIZE` 鎸囧畾瀹屾垚闃熷垪浜嬩欢锛圕ompletion Queue Event锛夌殑澶у皬銆傚畬鎴愰槦鍒?
浜嬩欢锛圕QE锛夋槸 NIC 鍙戝嚭鐨勩€佺敤浜庢寚绀哄寘鍙戦€侊紙濡傚彂閫佹垚鍔熸垨鍑洪敊锛夋垨鎺ユ敹锛堝鍖呯墖娈垫寚閽堬級瀹屾垚
鐘舵€佺殑浜嬩欢銆侰QE 澶у皬鍙傛暟鍙湪 NIC 鏀寔鏃朵慨鏀归粯璁ょ殑 CQE 澶у皬銆傛洿澶х殑 CQE 鍙互鎼哄甫鏇村鐨勬帴鏀?
缂撳啿鍖烘寚閽堬紝杩涜€?NIC 鍙粠绾胯矾涓婁紶杈撴洿澶х殑甯с€傚熀浜?NIC 纭欢锛岃嫢淇敼浜?CQE 澶у皬锛屾暣浣撳畬鎴?
闃熷垪澶у皬鍙湪椹卞姩涓皟鏁淬€?

`ETHTOOL_A_RINGS_HDS_THRESH` 鎸囧畾澶?/ 鏁版嵁鍒嗙鐗规€х殑闃堝€笺€傝嫢鎺ユ敹鍒扮殑鍖呭ぇ灏忓ぇ浜庤闃堝€硷紝
鍒欏ご涓庢暟鎹皢琚垎绂汇€?

## CHANNELS_GET


鑾峰彇閫氶亾鏁伴噺锛岀被浼间簬 `ETHTOOL_GCHANNELS` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`         nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`          nested  reply header
  `ETHTOOL_A_CHANNELS_RX_MAX`          u32     max receive channels
  `ETHTOOL_A_CHANNELS_TX_MAX`          u32     max transmit channels
  `ETHTOOL_A_CHANNELS_OTHER_MAX`       u32     max other channels
  `ETHTOOL_A_CHANNELS_COMBINED_MAX`    u32     max combined channels
  `ETHTOOL_A_CHANNELS_RX_COUNT`        u32     receive channel count
  `ETHTOOL_A_CHANNELS_TX_COUNT`        u32     transmit channel count
  `ETHTOOL_A_CHANNELS_OTHER_COUNT`     u32     other channel count
  `ETHTOOL_A_CHANNELS_COMBINED_COUNT`  u32     combined channel count
  =====================================  ======  ==========================


## CHANNELS_SET


璁剧疆閫氶亾鏁伴噺锛岀被浼间簬 `ETHTOOL_SCHANNELS` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`          nested  request header
  `ETHTOOL_A_CHANNELS_RX_COUNT`        u32     receive channel count
  `ETHTOOL_A_CHANNELS_TX_COUNT`        u32     transmit channel count
  `ETHTOOL_A_CHANNELS_OTHER_COUNT`     u32     other channel count
  `ETHTOOL_A_CHANNELS_COMBINED_COUNT`  u32     combined channel count
  =====================================  ======  ==========================

鍐呮牳浼氭鏌ヨ姹傜殑閫氶亾鏁伴噺涓嶈秴杩囬┍鍔ㄤ笂鎶ョ殑闄愬埗銆傞┍鍔ㄥ彲鑳芥柦鍔犻澶栫殑绾︽潫锛屼篃鍙兘涓嶆敮鎸佹墍鏈?
灞炴€с€?


## COALESCE_GET


鑾峰彇涓柇鑱氬悎鍙傛暟锛岀被浼间簬 `ETHTOOL_GCOALESCE` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_COALESCE_HEADER`         nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ===========================================  ======  =======================
  `ETHTOOL_A_COALESCE_HEADER`                nested  reply header
  `ETHTOOL_A_COALESCE_RX_USECS`              u32     寤惰繜锛堝井绉掞級锛屾櫘閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES`         u32     鏈€澶у寘鏁帮紝鏅€?Rx
  `ETHTOOL_A_COALESCE_RX_USECS_IRQ`          u32     寤惰繜锛堝井绉掞級锛孖RQ 涓殑 Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ`     u32     鏈€澶у寘鏁帮紝IRQ 涓殑 Rx
  `ETHTOOL_A_COALESCE_TX_USECS`              u32     寤惰繜锛堝井绉掞級锛屾櫘閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES`         u32     鏈€澶у寘鏁帮紝鏅€?Tx
  `ETHTOOL_A_COALESCE_TX_USECS_IRQ`          u32     寤惰繜锛堝井绉掞級锛孖RQ 涓殑 Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ`     u32     IRQ 涓殑鍖呮暟锛孴x
  `ETHTOOL_A_COALESCE_STATS_BLOCK_USECS`     u32     缁熻鏇存柊寤惰繜
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX`       bool    鑷€傚簲 Rx 鑱氬悎
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX`       bool    鑷€傚簲 Tx 鑱氬悎
  `ETHTOOL_A_COALESCE_PKT_RATE_LOW`          u32     浣庨€熺巼闃堝€?
  `ETHTOOL_A_COALESCE_RX_USECS_LOW`          u32     寤惰繜锛堝井绉掞級锛屼綆閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW`     u32     鏈€澶у寘鏁帮紝浣庨€?Rx
  `ETHTOOL_A_COALESCE_TX_USECS_LOW`          u32     寤惰繜锛堝井绉掞級锛屼綆閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW`     u32     鏈€澶у寘鏁帮紝浣庨€?Tx
  `ETHTOOL_A_COALESCE_PKT_RATE_HIGH`         u32     楂橀€熺巼闃堝€?
  `ETHTOOL_A_COALESCE_RX_USECS_HIGH`         u32     寤惰繜锛堝井绉掞級锛岄珮閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH`    u32     鏈€澶у寘鏁帮紝楂橀€?Rx
  `ETHTOOL_A_COALESCE_TX_USECS_HIGH`         u32     寤惰繜锛堝井绉掞級锛岄珮閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH`    u32     鏈€澶у寘鏁帮紝楂橀€?Tx
  `ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL`  u32     閫熺巼閲囨牱闂撮殧
  `ETHTOOL_A_COALESCE_USE_CQE_TX`            bool    瀹氭椂鍣ㄩ噸缃ā寮忥紝Tx
  `ETHTOOL_A_COALESCE_USE_CQE_RX`            bool    瀹氭椂鍣ㄩ噸缃ā寮忥紝Rx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES`     u32     鏈€澶ц仛鍚堝ぇ灏忥紝Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES`    u32     鏈€澶ц仛鍚堝寘鏁帮紝Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS`    u32     鏃堕棿锛堝井绉掞級锛岃仛鍚堬紝Tx
  `ETHTOOL_A_COALESCE_RX_PROFILE`            nested  DIM 閰嶇疆鏂囦欢锛孯x
  `ETHTOOL_A_COALESCE_TX_PROFILE`            nested  DIM 閰嶇疆鏂囦欢锛孴x
  `ETHTOOL_A_COALESCE_RX_CQE_FRAMES`         u32     鏈€澶у寘鏁帮紝Rx CQE
  `ETHTOOL_A_COALESCE_RX_CQE_NSECS`          u32     寤惰繜锛堢撼绉掞級锛孯x CQE
  ===========================================  ======  =======================

浠呭綋灞炴€х殑鍊间笉涓洪浂锛屾垨 **瀵瑰簲浣嶅湪 ``ethtool_ops`` 鐨?``supported_coalesce_params`` 涓?*
琚缃紙鍗宠椹卞姩澹版槑涓烘敮鎸侊級鏃讹紝璇ュ睘鎬ф墠浼氬寘鍚湪鍥炲涓€?

瀹氭椂鍣ㄩ噸缃ā寮忥紙`ETHTOOL_A_COALESCE_USE_CQE_TX` 涓?`ETHTOOL_A_COALESCE_USE_CQE_RX`锛?
鎺у埗鍖呭埌杈句笌鍚勪釜鍩轰簬鏃堕棿鐨勫欢杩熷弬鏁颁箣闂寸殑浜や簰銆傞粯璁ゆ儏鍐典笅锛屽畾鏃跺櫒搴旈檺鍒朵换鎰忓寘鍒拌揪/绂诲紑
涓庣浉搴斾腑鏂箣闂寸殑鏈€澶у欢杩熴€傚湪姝ゆā寮忎笅锛屽畾鏃跺櫒搴旂敱鍖呭埌杈撅紙鏈夋椂鏄笂涓€娆′腑鏂殑鎶曢€掞級鍚姩锛?
骞跺湪涓柇鎶曢€掓椂閲嶇疆銆傚皢鐩稿簲灞炴€ц缃负 1 灏嗗惎鐢?`CQE` 妯″紡锛屽叾涓瘡涓寘浜嬩欢閮戒細閲嶇疆瀹氭椂鍣ㄣ€?
鍦ㄦ妯″紡涓嬶紝瀹氭椂鍣ㄧ敤浜庨槻姝㈤槦鍒楃┖闂叉椂寮哄埗浜х敓涓柇锛岃€岀箒蹇欑殑闃熷垪鍒欎緷璧栧寘涓婇檺鏉ヨЕ鍙戜腑鏂€?

Tx 鑱氬悎鏄寚灏嗗抚澶嶅埗鍒拌繛缁殑缂撳啿鍖轰腑锛屼互渚夸綔涓轰竴涓崟鐙殑 IO 鎿嶄綔鎻愪氦銆?
`ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES` 鎻忚堪鎻愪氦缂撳啿鍖虹殑鏈€澶у瓧鑺傛暟銆?
`ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES` 鎻忚堪鍙仛鍚堝埌鍗曚釜缂撳啿鍖轰腑鐨勬渶澶у抚鏁般€?
`ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS` 鎻忚堪鑷仛鍚堝潡涓涓€涓寘鍒拌揪璧风畻鐨勬椂闂达紙寰锛夛紝
瓒呰繃璇ユ椂闂村悗搴斿彂閫佽鍧椼€傛鐗规€т富瑕佸鏌愪簺涓嶈兘寰堝ソ澶勭悊棰戠箒灏忓昂瀵?URB 浼犺緭鐨勭壒瀹?USB 璁惧
鏈夋剰涔夈€?

`ETHTOOL_A_COALESCE_RX_PROFILE` 涓?`ETHTOOL_A_COALESCE_TX_PROFILE` 寮曠敤 DIM 鍙傛暟锛?
鍙傝 `Generic Network Dynamic Interrupt Moderation (Net DIM)
<https://www.kernel.org/doc/Documentation/networking/net_dim.rst>`_銆?

Rx CQE 鑱氬悎鍏佽澶氫釜鎺ユ敹鍒扮殑鍖呰鑱氬悎鍒板崟涓畬鎴愰槦鍒楁潯鐩紙CQE锛夋垨鎻忚堪绗﹀洖鍐欎腑銆?
`ETHTOOL_A_COALESCE_RX_CQE_FRAMES` 鎻忚堪鍙仛鍚堝埌 CQE 鎴栧洖鍐欎腑鐨勬渶澶у抚鏁般€?
`ETHTOOL_A_COALESCE_RX_CQE_NSECS` 鎻忚堪鑱氬悎鐨?CQE 鎴栧洖鍐欒嚜绗竴涓寘鍒拌揪璧枫€佽鍙戦€佸墠鐨?
鏈€澶ф椂闂达紙绾崇锛夈€?

## COALESCE_SET


璁剧疆涓柇鑱氬悎鍙傛暟锛岀被浼间簬 `ETHTOOL_SCOALESCE` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ===========================================  ======  =======================
  `ETHTOOL_A_COALESCE_HEADER`                nested  request header
  `ETHTOOL_A_COALESCE_RX_USECS`              u32     寤惰繜锛堝井绉掞級锛屾櫘閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES`         u32     鏈€澶у寘鏁帮紝鏅€?Rx
  `ETHTOOL_A_COALESCE_RX_USECS_IRQ`          u32     寤惰繜锛堝井绉掞級锛孖RQ 涓殑 Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ`     u32     鏈€澶у寘鏁帮紝IRQ 涓殑 Rx
  `ETHTOOL_A_COALESCE_TX_USECS`              u32     寤惰繜锛堝井绉掞級锛屾櫘閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES`         u32     鏈€澶у寘鏁帮紝鏅€?Tx
  `ETHTOOL_A_COALESCE_TX_USECS_IRQ`          u32     寤惰繜锛堝井绉掞級锛孖RQ 涓殑 Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ`     u32     IRQ 涓殑鍖呮暟锛孴x
  `ETHTOOL_A_COALESCE_STATS_BLOCK_USECS`     u32     缁熻鏇存柊寤惰繜
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX`       bool    鑷€傚簲 Rx 鑱氬悎
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX`       bool    鑷€傚簲 Tx 鑱氬悎
  `ETHTOOL_A_COALESCE_PKT_RATE_LOW`          u32     浣庨€熺巼闃堝€?
  `ETHTOOL_A_COALESCE_RX_USECS_LOW`          u32     寤惰繜锛堝井绉掞級锛屼綆閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW`     u32     鏈€澶у寘鏁帮紝浣庨€?Rx
  `ETHTOOL_A_COALESCE_TX_USECS_LOW`          u32     寤惰繜锛堝井绉掞級锛屼綆閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW`     u32     鏈€澶у寘鏁帮紝浣庨€?Tx
  `ETHTOOL_A_COALESCE_PKT_RATE_HIGH`         u32     楂橀€熺巼闃堝€?
  `ETHTOOL_A_COALESCE_RX_USECS_HIGH`         u32     寤惰繜锛堝井绉掞級锛岄珮閫?Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH`    u32     鏈€澶у寘鏁帮紝楂橀€?Rx
  `ETHTOOL_A_COALESCE_TX_USECS_HIGH`         u32     寤惰繜锛堝井绉掞級锛岄珮閫?Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH`    u32     鏈€澶у寘鏁帮紝楂橀€?Tx
  `ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL`  u32     閫熺巼閲囨牱闂撮殧
  `ETHTOOL_A_COALESCE_USE_CQE_TX`            bool    瀹氭椂鍣ㄩ噸缃ā寮忥紝Tx
  `ETHTOOL_A_COALESCE_USE_CQE_RX`            bool    瀹氭椂鍣ㄩ噸缃ā寮忥紝Rx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES`     u32     鏈€澶ц仛鍚堝ぇ灏忥紝Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES`    u32     鏈€澶ц仛鍚堝寘鏁帮紝Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS`    u32     鏃堕棿锛堝井绉掞級锛岃仛鍚堬紝Tx
  `ETHTOOL_A_COALESCE_RX_PROFILE`            nested  DIM 閰嶇疆鏂囦欢锛孯x
  `ETHTOOL_A_COALESCE_TX_PROFILE`            nested  DIM 閰嶇疆鏂囦欢锛孴x
  `ETHTOOL_A_COALESCE_RX_CQE_FRAMES`         u32     鏈€澶у寘鏁帮紝Rx CQE
  `ETHTOOL_A_COALESCE_RX_CQE_NSECS`          u32     寤惰繜锛堢撼绉掞級锛孯x CQE
  ===========================================  ======  =======================

濡傛灉璇锋眰鍖呭惈琚┍鍔ㄥ０鏄庝负涓嶆敮鎸佺殑灞炴€э紙鍗?**鐩稿簲浣嶅湪 ``ethtool_ops`` 鐨?
``supported_coalesce_params`` 涓湭璁剧疆**锛夛紝鍒欐棤璁哄叾鍊煎浣曡姹傞兘浼氳鎷掔粷銆傞┍鍔ㄥ彲鑳藉
鑱氬悎鍙傛暟鍙婂叾鍙栧€兼柦鍔犻澶栫殑绾︽潫銆?

涓庨€氳繃 `ioctl()` 鍙戝嚭鐨勮姹傜浉姣旓紝璇ヨ姹傜殑 netlink 鐗堟湰浼氭洿鍔姏鍦扮‘淇濈敤鎴锋寚瀹氱殑鍊煎凡琚?
搴旂敤锛屽苟鍙兘璋冪敤椹卞姩涓ゆ銆?


## PAUSE_GET


鑾峰彇鏆傚仠甯ц缃紝绫讳技浜?`ETHTOOL_GPAUSEPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_STATS_SRC`          u32     source of statistics
  =====================================  ======  ==========================

`ETHTOOL_A_PAUSE_STATS_SRC` 鏄彲閫夌殑銆傚畠鍙栧€艰嚜锛?

    :identifiers: ethtool_mac_stats_src

鑻ヨ姹備腑缂虹渷锛屽垯鍝嶅簲涓細甯︽湁涓€涓瓑浜?`ETHTOOL_MAC_STATS_SRC_AGGREGATE` 鐨?
`ETHTOOL_A_PAUSE_STATS_SRC` 灞炴€ф潵鎻愪緵缁熻淇℃伅銆?

鍐呮牳鍝嶅簲鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_AUTONEG`            bool    pause autonegotiation
  `ETHTOOL_A_PAUSE_RX`                 bool    receive pause frames
  `ETHTOOL_A_PAUSE_TX`                 bool    transmit pause frames
  `ETHTOOL_A_PAUSE_STATS`              nested  pause statistics
  =====================================  ======  ==========================

鑻?`ETHTOOL_A_HEADER_FLAGS` 涓缃簡 `ETHTOOL_FLAG_STATS`锛屽垯浼氭姤鍛?
`ETHTOOL_A_PAUSE_STATS`銆傚鏋滈┍鍔ㄦ湭鎶ュ憡浠讳綍缁熻淇℃伅锛屽畠灏嗘槸绌虹殑銆傞┍鍔ㄥ湪浠ヤ笅缁撴瀯涓～鍐?
缁熻淇℃伅锛?

    :identifiers: ethtool_pause_stats

姣忎釜鎴愬憳閮芥湁瀵瑰簲鐨勫凡瀹氫箟灞炴€с€?


## PAUSE_SET


璁剧疆鏆傚仠鍙傛暟锛岀被浼间簬 `ETHTOOL_GPAUSEPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_AUTONEG`            bool    pause autonegotiation
  `ETHTOOL_A_PAUSE_RX`                 bool    receive pause frames
  `ETHTOOL_A_PAUSE_TX`                 bool    transmit pause frames
  =====================================  ======  ==========================


## EEE_GET


鑾峰彇楂樻晥浠ュお缃戯紙Energy Efficient Ethernet锛夎缃紝绫讳技浜?`ETHTOOL_GEEE` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  `ETHTOOL_A_EEE_MODES_OURS`           bool    supported/advertised modes
  `ETHTOOL_A_EEE_MODES_PEER`           bool    peer advertised link modes
  `ETHTOOL_A_EEE_ACTIVE`               bool    EEE is actively used
  `ETHTOOL_A_EEE_ENABLED`              bool    EEE is enabled
  `ETHTOOL_A_EEE_TX_LPI_ENABLED`       bool    Tx lpi enabled
  `ETHTOOL_A_EEE_TX_LPI_TIMER`         u32     Tx lpi timeout (in us)
  =====================================  ======  ==========================

鍦?`ETHTOOL_A_EEE_MODES_OURS` 涓紝鎺╃爜鐢卞惎鐢?EEE 鐨勯摼璺ā寮忕粍鎴愶紝鍊间负閫氬憡浜?EEE 鐨勯摼璺?
妯″紡銆傚绔€氬憡浜?EEE 鐨勯摼璺ā寮忓垪鍦?`ETHTOOL_A_EEE_MODES_PEER` 涓紙鏃犳帺鐮侊級銆俷etlink
鎺ュ彛鍏佽鎶ュ憡鎵€鏈夐摼璺ā寮忕殑 EEE 鐘舵€侊紝浣嗙洰鍓嶅彧鏈夊墠 32 涓敱 `ethtool_ops` 鍥炶皟鎻愪緵銆?


## EEE_SET


璁剧疆楂樻晥浠ュお缃戝弬鏁帮紝绫讳技浜?`ETHTOOL_SEEE` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  `ETHTOOL_A_EEE_MODES_OURS`           bool    advertised modes
  `ETHTOOL_A_EEE_ENABLED`              bool    EEE is enabled
  `ETHTOOL_A_EEE_TX_LPI_ENABLED`       bool    Tx lpi enabled
  `ETHTOOL_A_EEE_TX_LPI_TIMER`         u32     Tx lpi timeout (in us)
  =====================================  ======  ==========================

`ETHTOOL_A_EEE_MODES_OURS` 鐢ㄤ簬鍒楀嚭瑕侀€氬憡 EEE 鐨勯摼璺ā寮忥紙鑻ユ棤鎺╃爜锛夛紝鎴栨寚瀹氬璇ュ垪琛ㄧ殑
鏇存敼锛堣嫢鏈夋帺鐮侊級銆俷etlink 鎺ュ彛鍏佽鎶ュ憡鎵€鏈夐摼璺ā寮忕殑 EEE 鐘舵€侊紝浣嗙洰鍓嶅彧鑳借缃墠 32 涓紝
鍥犱负 `ethtool_ops` 鍥炶皟浠呮敮鎸佽繖涔堝銆?


## TSINFO_GET


鑾峰彇鏃堕棿鎴充俊鎭紝绫讳技浜?`ETHTOOL_GET_TS_INFO` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ========================================  ======  ============================
  `ETHTOOL_A_TSINFO_HEADER`               nested  request header
  `ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER`    nested  PTP hw clock provider
  ========================================  ======  ============================

鍐呮牳鍝嶅簲鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_TSINFO_HEADER`            nested  request header
  `ETHTOOL_A_TSINFO_TIMESTAMPING`      bitset  SO_TIMESTAMPING flags
  `ETHTOOL_A_TSINFO_TX_TYPES`          bitset  supported Tx types
  `ETHTOOL_A_TSINFO_RX_FILTERS`        bitset  supported Rx filters
  `ETHTOOL_A_TSINFO_PHC_INDEX`         u32     PTP hw clock index
  `ETHTOOL_A_TSINFO_STATS`             nested  HW timestamping statistics
  =====================================  ======  ==========================

鑻ユ棤鍏宠仈鐨?PHC锛屽垯 `ETHTOOL_A_TSINFO_PHC_INDEX` 缂虹渷锛堟鎯呭喌鏃犵壒娈婂彇鍊硷級銆傝嫢浣嶉泦灞炴€у皢
涓虹┖锛堟棤浠讳綍浣嶈璁剧疆锛夛紝鍒欎細琚渷鐣ャ€?

棰濆鐨勭‖浠舵椂闂存埑缁熻鍝嶅簲鍐呭锛?

  ==================================================  ======  =====================
  `ETHTOOL_A_TS_STAT_TX_PKTS`                       uint    甯?Tx 纭欢鏃堕棿鎴崇殑鍖?
  `ETHTOOL_A_TS_STAT_TX_LOST`                       uint    鏈埌杈剧殑 Tx 纭欢鏃堕棿鎴宠鏁?
  `ETHTOOL_A_TS_STAT_TX_ERR`                        uint    纭欢閿欒璇锋眰鐨?Tx 鏃堕棿鎴宠鏁?
  `ETHTOOL_A_TS_STAT_TX_ONESTEP_PKTS_UNCONFIRMED`   uint    甯︿竴姝ワ紙one-step锛夌‖浠?Tx 鏃堕棿鎴炽€佹姇閫掓湭纭鐨勫寘
  ==================================================  ======  =====================

## CABLE_TEST


鍚姩绾跨紗娴嬭瘯銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_CABLE_TEST_HEADER`       nested  request header
  ====================================  ======  ==========================

閫氱煡鍐呭锛?

涓€鏉′互澶綉绾跨紗閫氬父鍖呭惈 1銆? 鎴?4 瀵圭嚎銆傚彧鏈夊湪鏌愬绾垮瓨鍦ㄦ晠闅滀粠鑰屽彂鐢熷弽灏勬椂锛屾墠鑳芥祴閲忚瀵?
绾跨殑闀垮害銆傚叿浣撶‖浠跺彲鑳戒笉鎻愪緵鏁呴殰淇℃伅銆傚洜姝ら€氱煡娑堟伅鐨勫唴瀹瑰ぇ澶氭槸鍙€夌殑銆傝繖浜涘睘鎬у彲浠ヤ互
浠绘剰娆℃暟銆佷换鎰忛『搴忥紝閽堝浠绘剰鏁伴噺鐨勭嚎瀵归噸澶嶅嚭鐜般€?

绀轰緥灞曠ず浜嗗 T2 绾跨紗锛堝嵆涓ゅ绾匡級瀹屾垚娴嬭瘯鏃跺彂閫佺殑閫氱煡銆傚叾涓竴瀵规甯革紝鍥犳娌℃湁闀垮害淇℃伅銆?
绗簩瀵瑰瓨鍦ㄦ晠闅滐紝鍥犳甯︽湁闀垮害淇℃伅銆?

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_HEADER`             | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_STATUS`             | u8     | completed           |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_NTF_NEST`           | nested | all the results     |
 +-+-------------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_RESULT`           | nested | cable test result   |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_CODE`        | u8     | result code         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_RESULT`           | nested | cable test results  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_CODE`        | u8     | result code         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULT_SRC`          | u32    | information source  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_FAULT_LENGTH`     | nested | cable length        |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_PAIR`   | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_CM`     | u32    | length in cm        |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_SRC`    | u32    | information source  |
 +-+-+-----------------------------------------+--------+---------------------+

## CABLE_TEST TDR


鍚姩绾跨紗娴嬭瘯骞朵笂鎶ュ師濮?TDR 鏁版嵁

璇锋眰鍐呭锛?

 +--------------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_HEADER`        | nested | reply header          |
 +--------------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_CFG`           | nested | test configuration    |
 +-+------------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE`  | u32    | first data distance   |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_LAST_DISTANCE`   | u32    | last data distance    |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_STEP_DISTANCE`   | u32    | distance of each step |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR`    | u8     | pair to test          |
 +-+-+----------------------------------------+--------+-----------------------+

ETHTOOL_A_CABLE_TEST_TDR_CFG 鍙婂叾宓屽涓殑鍏ㄩ儴鎴愬憳鍧囦负鍙€夈€傛墍鏈夎窛绂婚兘浠ュ帢绫宠〃绀恒€侾HY 灏?
杩欎簺璺濈浣滀负鍙傝€冿紝骞跺彇鏁村埌瀹冨疄闄呮敮鎸佺殑鏈€杩戣窛绂汇€傚鏋滀紶鍏ユ煇瀵圭嚎锛屽垯鍙祴璇曡瀵圭嚎锛涘惁鍒欐祴璇?
鎵€鏈夊绾裤€?

閫氱煡鍐呭锛?

鍘熷 TDR 鏁版嵁閫氳繃鍚戠嚎缂嗗彂閫佽剦鍐插苟璁板綍缁欏畾璺濈鐨勫弽灏勮剦鍐插箙搴︽潵閲囬泦銆?

濡傛灉浠?1 绫抽棿闅旀帰娴嬪畬鏁寸殑 100 绫筹紝閲囬泦 TDR 鏁版嵁鍙兘闇€瑕佽嫢骞茬銆傛祴璇曞惎鍔ㄦ椂浼氬彂閫佷竴鏉?
浠呭寘鍚?ETHTOOL_A_CABLE_TEST_TDR_STATUS銆佷笖鍊间负
ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED 鐨勯€氱煡銆?

娴嬭瘯瀹屾垚鏃朵細鍙戦€佺浜屾潯閫氱煡锛屽寘鍚?ETHTOOL_A_CABLE_TEST_TDR_STATUS锛堝€间负
ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED锛変互鍙?TDR 鏁版嵁銆?

娑堟伅鍙兘鍙€夊湴鍖呭惈娌跨嚎缂嗗彂閫佺殑鑴夊啿骞呭害銆傚畠浠?mV 璁￠噺銆傚弽灏勪笉搴斿ぇ浜庡彂閫佺殑鑴夊啿銆?

鍦ㄥ師濮?TDR 鏁版嵁涔嬪墠搴旀湁涓€涓?ETHTOOL_A_CABLE_TDR_NEST_STEP 宓屽锛屽叾涓寘鍚叧浜庨娆¤鍙栥€?
鏈璇诲彇浠ュ強姣忔璇诲彇涔嬮棿姝ヨ繘璺濈鐨勪俊鎭€傝窛绂讳互鍘樼背璁￠噺銆傝繖浜涘簲涓?PHY 浣跨敤鐨勭簿纭€笺€傚鏋?
鍘熺敓娴嬮噺鍒嗚鲸鐜囧ぇ浜?1 cm锛岃繖浜涘€煎彲鑳戒笌鐢ㄦ埛璇锋眰鐨勪笉鍚屻€?

瀵逛簬绾跨紗涓婄殑姣忎竴姝ワ紝浣跨敤涓€涓?ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE 鏉ユ姤鍛婄粰瀹氬绾夸笂鐨勫弽灏?
骞呭害銆?

 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_HEADER`         | nested | reply header         |
 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_STATUS`         | u8     | completed            |
 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST`       | nested | all the results      |
 +-+-------------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_PULSE`        | nested | TX Pulse amplitude   |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_PULSE_mV`            | s16    | Pulse amplitude      |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_NEST_STEP`             | nested | TDR step info        |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE` | u32    | First data distance  |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_LAST_DISTANCE`  | u32    | Last data distance   |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_STEP_DISTANCE`  | u32    | distance of each step|
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+

## TUNNEL_INFO


鑾峰彇 NIC 鎵€鎰熺煡鐨勯毀閬撶姸鎬佷俊鎭€?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_TUNNEL_INFO_HEADER`       nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_TUNNEL_INFO_HEADER`            | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_TUNNEL_INFO_UDP_PORTS`         | nested | all UDP port tables |
 +-+-------------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_TUNNEL_UDP_TABLE`            | nested | one UDP port table  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE`     | u32    | max size of the     |
 | | |                                         |        | table               |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES`    | bitset | tunnel types which  |
 | | |                                         |        | table can hold      |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_ENTRY`    | nested | offloaded UDP port  |
 +-+-+-+---------------------------------------+--------+---------------------+
 | | | | `ETHTOOL_A_TUNNEL_UDP_ENTRY_PORT`   | be16   | UDP port            |
 +-+-+-+---------------------------------------+--------+---------------------+
 | | | | `ETHTOOL_A_TUNNEL_UDP_ENTRY_TYPE`   | u32    | tunnel type         |
 +-+-+-+---------------------------------------+--------+---------------------+

瀵逛簬 UDP 闅ч亾琛紝绌虹殑 `ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES` 琛ㄧず璇ヨ〃鍖呭惈鐢?NIC 纭紪鐮佺殑
闈欐€佹潯鐩€?

## FEC_GET


鑾峰彇 FEC 閰嶇疆涓庣姸鎬侊紝绫讳技浜?`ETHTOOL_GFECPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  `ETHTOOL_A_FEC_MODES`                bitset  configured modes
  `ETHTOOL_A_FEC_AUTO`                 bool    FEC mode auto selection
  `ETHTOOL_A_FEC_ACTIVE`               u32     index of active FEC mode
  `ETHTOOL_A_FEC_STATS`                nested  FEC statistics
  =====================================  ======  ==========================

`ETHTOOL_A_FEC_ACTIVE` 鏄綋鍓嶅湪鎺ュ彛涓婂浜庢椿鍔ㄧ姸鎬佺殑 FEC 閾捐矾妯″紡鐨勪綅绱㈠紩銆傝嫢璁惧涓嶆敮鎸?
FEC锛岃灞炴€у彲鑳戒笉瀛樺湪銆?

`ETHTOOL_A_FEC_MODES` 涓?`ETHTOOL_A_FEC_AUTO` 浠呭湪绂佺敤鑷崗鍟嗘椂鎵嶆湁鎰忎箟銆傝嫢
`ETHTOOL_A_FEC_AUTO` 闈為浂锛岄┍鍔ㄥ皢鏍规嵁 SFP 妯″潡鐨勫弬鏁拌嚜鍔ㄩ€夋嫨 FEC 妯″紡銆傝繖绛変环浜?ioctl
鎺ュ彛鐨?`ETHTOOL_FEC_AUTO` 浣嶃€俙ETHTOOL_A_FEC_MODES` 浣跨敤閾捐矾妯″紡浣嶏紙鑰岄潪鏃х殑
`ETHTOOL_FEC_*` 浣嶏級鎼哄甫褰撳墠鐨?FEC 閰嶇疆銆?

鑻?`ETHTOOL_A_HEADER_FLAGS` 涓缃簡 `ETHTOOL_FLAG_STATS`锛屽垯浼氭姤鍛?
`ETHTOOL_A_FEC_STATS`銆傛瘡涓睘鎬ф惡甯︿竴涓敱 64 浣嶇粺璁＄粍鎴愮殑鏁扮粍銆傛暟缁勭殑绗竴涓潯鐩寘鍚鍙?
涓婄殑浜嬩欢鎬绘暟锛屽悗缁潯鐩垯瀵瑰簲閫氶亾/PCS 瀹炰緥鐨勮鏁板櫒銆傛暟缁勪腑鐨勬潯鐩暟灏嗕负锛?

+--------------+---------------------------------------------+
| `0`          | 璁惧涓嶆敮鎸?FEC 缁熻                          |
+--------------+---------------------------------------------+
| `1`          | 璁惧涓嶆敮鎸佹寜閫氶亾缁嗗垎                          |
+--------------+---------------------------------------------+
| `1 + #lanes` | 璁惧瀹屽叏鏀寔 FEC 缁熻                          |
+--------------+---------------------------------------------+

椹卞姩鍦ㄤ互涓嬬粨鏋勪腑濉啓缁熻淇℃伅锛?

    :identifiers: ethtool_fec_stats

缁熻鍙兘甯︽湁 FEC 鍒嗙鐩存柟鍥惧睘鎬?`ETHTOOL_A_FEC_STAT_HIST`锛屽叾瀹氫箟瑙?IEEE 802.3ck-2022
涓?802.3df-2024銆傚祵濂楀睘鎬у皢鍖呭惈璇ュ垎绠卞唴 FEC 閿欒鐨勮寖鍥达紙鍚竟鐣岋級浠ュ強璇ュ垎绠卞唴鐨勯敊璇簨浠?
鏁伴噺銆?

## FEC_SET


璁剧疆 FEC 鍙傛暟锛岀被浼间簬 `ETHTOOL_SFECPARAM` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  `ETHTOOL_A_FEC_MODES`                bitset  configured modes
  `ETHTOOL_A_FEC_AUTO`                 bool    FEC mode auto selection
  =====================================  ======  ==========================

`FEC_SET` 浠呭湪绂佺敤鑷崗鍟嗘椂鏈夋剰涔夈€傚惁鍒?FEC 妯″紡灏嗕綔涓鸿嚜鍗忓晢鐨勪竴閮ㄥ垎琚€夋嫨銆?


`ETHTOOL_A_FEC_MODES` 閫夋嫨搴斾娇鐢ㄥ摢绉?FEC 妯″紡銆傚缓璁彧璁剧疆涓€浣嶏紱鑻ヨ缃簡澶氫綅锛岄┍鍔ㄥ彲鑳?
浠ュ叿浣撳疄鐜扮浉鍏崇殑鏂瑰紡鍦ㄥ叾涓€夋嫨銆?

`ETHTOOL_A_FEC_AUTO` 璇锋眰椹卞姩鏍规嵁 SFP 妯″潡鍙傛暟閫夋嫨 FEC 妯″紡銆傝繖骞朵笉浠ｈ〃鑷崗鍟嗐€?

## MODULE_EEPROM_GET


鑾峰彇妯″潡 EEPROM 鏁版嵁杞偍銆傛鎺ュ彛璁捐涓烘瘡娆℃渶澶氬厑璁歌浆鍌?1/2 椤点€傝繖鎰忓懗鐫€鍙厑璁歌浆鍌?
128锛堟垨鏇村皯锛夊瓧鑺傦紝涓斾笉寰楄法瓒婁綅浜庡亸绉?128 澶勭殑鍗婇〉杈圭晫銆傚浜?0 涔嬪鐨勫叾瀹冮〉锛屽彧鏈夐珮
128 瀛楄妭鍙闂€?

璇锋眰鍐呭锛?

  =======================================  ======  ==========================
  `ETHTOOL_A_MODULE_EEPROM_HEADER`       nested  request header
  `ETHTOOL_A_MODULE_EEPROM_OFFSET`       u32     offset within a page
  `ETHTOOL_A_MODULE_EEPROM_LENGTH`       u32     amount of bytes to read
  `ETHTOOL_A_MODULE_EEPROM_PAGE`         u8      page number
  `ETHTOOL_A_MODULE_EEPROM_BANK`         u8      bank number
  `ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS`  u8      page I2C address
  =======================================  ======  ==========================

鑻ユ湭鎸囧畾 `ETHTOOL_A_MODULE_EEPROM_BANK`锛屽垯鍋囧畾涓?bank 0銆?

鍐呮牳鍝嶅簲鍐呭锛?

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_MODULE_EEPROM_HEADER`          | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_MODULE_EEPROM_DATA`            | binary | array of bytes from |
 |                                             |        | module EEPROM       |
 +---------------------------------------------+--------+---------------------+

`ETHTOOL_A_MODULE_EEPROM_DATA` 鐨勫睘鎬ч暱搴︾瓑浜庨┍鍔ㄥ疄闄呰鍙栫殑瀛楄妭鏁般€?

## STATS_GET


鑾峰彇鎺ュ彛鐨勬爣鍑嗙粺璁′俊鎭€傛敞鎰忥紝杩欎笉鏄鏆撮湶椹卞姩瀹氫箟缁熻鐨?`ETHTOOL_GSTATS` 鐨勯噸鏂板疄鐜般€?

璇锋眰鍐呭锛?

  =======================================  ======  ==========================
  `ETHTOOL_A_STATS_HEADER`               nested  request header
  `ETHTOOL_A_STATS_SRC`                  u32     source of statistics
  `ETHTOOL_A_STATS_GROUPS`               bitset  requested groups of stats
  =======================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_HEADER`        | nested | reply header                   |
 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_SRC`           | u32    | source of statistics           |
 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_GRP`           | nested | one or more group of stats     |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_ID`      | u32    | group ID - `ETHTOOL_STATS_*` |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_SS_ID`   | u32    | string set ID for names        |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_STAT`    | nested | nest containing a statistic    |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_HIST_RX` | nested | histogram statistic (Rx)       |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_HIST_TX` | nested | histogram statistic (Tx)       |
 +-+---------------------------------+--------+--------------------------------+

鐢ㄦ埛閫氳繃 `ETHTOOL_A_STATS_GROUPS` 浣嶉泦鎸囧畾浠栦滑璇锋眰鐨勭粺璁″垎缁勩€傚綋鍓嶅凡瀹氫箟鐨勫€煎涓嬶細

 ====================== ======== ===============================================
 ETHTOOL_STATS_ETH_MAC  eth-mac  Basic IEEE 802.3 MAC statistics (30.3.1.1.*)
 ETHTOOL_STATS_ETH_PHY  eth-phy  Basic IEEE 802.3 PHY statistics (30.3.2.1.*)
 ETHTOOL_STATS_ETH_CTRL eth-ctrl Basic IEEE 802.3 MAC Ctrl statistics (30.3.3.*)
 ETHTOOL_STATS_RMON     rmon     RMON (RFC 2819) statistics
 ETHTOOL_STATS_PHY      phy      Additional PHY statistics, not defined by IEEE
 ====================== ======== ===============================================

姣忎釜鍒嗙粍搴斿湪鍥炲涓湁涓€涓搴旂殑 `ETHTOOL_A_STATS_GRP`銆俙ETHTOOL_A_STATS_GRP_ID` 鏍囪瘑璇?
鍒嗙粍鐨勭粺璁″祵濂楀寘鍚簡浠€涔堛€俙ETHTOOL_A_STATS_GRP_SS_ID` 鏍囪瘑鍒嗙粍鍐呯粺璁″悕绉扮殑瀛楃涓查泦 ID
锛堣嫢鍙敤锛夈€?

缁熻琚坊鍔犲埌 `ETHTOOL_A_STATS_GRP` 宓屽涓嬬殑 `ETHTOOL_A_STATS_GRP_STAT`銆?
`ETHTOOL_A_STATS_GRP_STAT` 鍐呴儴搴斿寘鍚竴涓?8 瀛楄妭锛坲64锛夊睘鎬р€斺€旇灞炴€х殑绫诲瀷鍗充负缁熻 ID锛?
鍊间负璇ョ粺璁＄殑鍊笺€傛瘡涓垎缁勫缁熻 ID 鏈夎嚜宸辩殑瑙ｉ噴銆傚睘鎬?ID 瀵瑰簲浜庣敱 `ETHTOOL_A_STATS_GRP_SS_ID`
鏍囪瘑鐨勫瓧绗︿覆闆嗕腑鐨勫瓧绗︿覆銆傚鏉傜粺璁★紙渚嬪 RMON 鐩存柟鍥炬潯鐩級涔熷垪鍦?`ETHTOOL_A_STATS_GRP`
鍐咃紝涓旀湭鍦ㄥ瓧绗︿覆闆嗕腑瀹氫箟瀛楃涓层€?

RMON 鈥滅洿鏂瑰浘鈥濊鏁板櫒缁熻缁欏畾澶у皬鑼冨洿鍐呯殑鍖呮暟閲忋€傜敱浜?RFC 鏈瀹氳秴鍑烘爣鍑?1518 MTU 鐨勮寖鍥达紝
鍚勮澶囧妗剁殑瀹氫箟鏈夋墍涓嶅悓銆傚洜姝ゅ寘鑼冨洿鐨勫畾涔変氦鐢卞悇椹卞姩鍐冲畾銆?

`ETHTOOL_A_STATS_GRP_HIST_RX` 涓?`ETHTOOL_A_STATS_GRP_HIST_TX` 宓屽鍖呭惈浠ヤ笅灞炴€э細

 ================================= ====== ===================================
 ETHTOOL_A_STATS_RMON_HIST_BKT_LOW u32    low bound of the packet size bucket
 ETHTOOL_A_STATS_RMON_HIST_BKT_HI  u32    high bound of the bucket
 ETHTOOL_A_STATS_RMON_HIST_VAL     u64    packet counter
 ================================= ====== ===================================

涓嬬晫涓庝笂鐣屽潎涓哄惈杈圭晫锛屼緥濡傦細

 ============================= ==== ====
 RFC statistic                 low  high
 ============================= ==== ====
 etherStatsPkts64Octets          0    64
 etherStatsPkts512to1023Octets 512  1023
 ============================= ==== ====

`ETHTOOL_A_STATS_SRC` 鏄彲閫夌殑銆備笌 `PAUSE_GET` 绫讳技锛屽畠鍙栧€艰嚜 `enum ethtool_mac_stats_src`銆?
鑻ヨ姹備腑缂虹渷锛屽垯鍝嶅簲涓細甯︽湁涓€涓瓑浜?`ETHTOOL_MAC_STATS_SRC_AGGREGATE` 鐨?
`ETHTOOL_A_STATS_SRC` 灞炴€ф潵鎻愪緵缁熻淇℃伅銆?

## PHC_VCLOCKS_GET


鏌ヨ璁惧 PHC 铏氭嫙鏃堕挓淇℃伅銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PHC_VCLOCKS_HEADER`      nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PHC_VCLOCKS_HEADER`      nested  reply header
  `ETHTOOL_A_PHC_VCLOCKS_NUM`         u32     PHC virtual clocks number
  `ETHTOOL_A_PHC_VCLOCKS_INDEX`       s32     PHC index array
  ====================================  ======  ==========================

## MODULE_GET


鑾峰彇鏀跺彂鍣ㄦā鍧楀弬鏁般€?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`            nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ======================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`             nested  reply header
  `ETHTOOL_A_MODULE_POWER_MODE_POLICY`  u8      power mode policy
  `ETHTOOL_A_MODULE_POWER_MODE`         u8      operational power mode
  ======================================  ======  ==========================

鍙€夌殑 `ETHTOOL_A_MODULE_POWER_MODE_POLICY` 灞炴€х紪鐮佷簡鐢变富鏈哄己鍒舵墽琛岀殑鏀跺彂鍣ㄦā鍧楃數婧愭ā寮?
绛栫暐銆傞粯璁ょ瓥鐣ュ彇鍐充簬椹卞姩锛屼絾鈥渁uto鈥濇槸鎺ㄨ崘鐨勯粯璁ゅ€硷紝鏂伴┍鍔ㄤ互鍙婁笉瑕佹眰鍏煎鏃ц涓虹殑椹卞姩閮藉簲
瀹炵幇瀹冦€?

鍙€夌殑 `ETHTOOL_A_MODULE_POWER_MODE` 灞炴€х紪鐮佷簡鏀跺彂鍣ㄦā鍧楃殑鎿嶄綔鐢垫簮妯″紡绛栫暐銆傚畠浠呭湪鎻掑叆
妯″潡鏃舵墠琚笂鎶ャ€傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_module_power_mode

## MODULE_SET


璁剧疆鏀跺彂鍣ㄦā鍧楀弬鏁般€?

璇锋眰鍐呭锛?

  ======================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`             nested  request header
  `ETHTOOL_A_MODULE_POWER_MODE_POLICY`  u8      power mode policy
  ======================================  ======  ==========================

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_MODULE_POWER_MODE_POLICY` 灞炴€х敤浜庤缃敱涓绘満寮哄埗鎵ц鐨勬敹鍙戝櫒
妯″潡鐢垫簮绛栫暐銆傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_module_power_mode_policy

瀵逛簬 SFF-8636 妯″潡锛屼綆鍔熻€楁ā寮忕敱涓绘満鏍规嵁瑙勮寖 2.10a 淇鐗堢殑琛?6-10 寮哄埗璁剧疆銆?

瀵逛簬 CMIS 妯″潡锛屼綆鍔熻€楁ā寮忕敱涓绘満鏍规嵁瑙勮寖 5.0 淇鐗堢殑琛?6-12 寮哄埗璁剧疆銆?

## PSE_GET


鑾峰彇 PSE 灞炴€с€?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PSE_HEADER`               nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ==========================================  ======  =============================
  `ETHTOOL_A_PSE_HEADER`                    nested  reply header
  `ETHTOOL_A_PODL_PSE_ADMIN_STATE`             u32  PoDL PSE 鍔熻兘鐨勬搷浣滅姸鎬?
  `ETHTOOL_A_PODL_PSE_PW_D_STATUS`             u32  PoDL PSE 鐨勪緵鐢垫娴嬬姸鎬?
  `ETHTOOL_A_C33_PSE_ADMIN_STATE`              u32  PoE PSE 鍔熻兘鐨勬搷浣滅姸鎬?
  `ETHTOOL_A_C33_PSE_PW_D_STATUS`              u32  PoE PSE 鐨勪緵鐢垫娴嬬姸鎬?
  `ETHTOOL_A_C33_PSE_PW_CLASS`                 u32  PoE PSE 鐨勫姛鐜囩瓑绾?
  `ETHTOOL_A_C33_PSE_ACTUAL_PW`                u32  PoE PSE 涓婂疄闄呮秷鑰楃殑鍔熺巼
  `ETHTOOL_A_C33_PSE_EXT_STATE`                u32  PoE PSE 鐨勬墿灞曢敊璇姸鎬?
  `ETHTOOL_A_C33_PSE_EXT_SUBSTATE`             u32  PoE PSE 鐨勬墿灞曢敊璇瓙鐘舵€?
  `ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT`           u32  PoE PSE 褰撳墠閰嶇疆鐨勫姛鐜囬檺鍒?
  `ETHTOOL_A_C33_PSE_PW_LIMIT_RANGES`       nested  鏀寔鐨勫姛鐜囬檺鍒堕厤缃寖鍥?
  `ETHTOOL_A_PSE_PW_D_ID`                      u32  PSE 渚涚數鍩熺殑绱㈠紩
  `ETHTOOL_A_PSE_PRIO_MAX`                     u32  PoE PSE 涓婂彲閰嶇疆鐨勬渶澶т紭鍏堢骇
  `ETHTOOL_A_PSE_PRIO`                         u32  PoE PSE 褰撳墠閰嶇疆鐨勪紭鍏堢骇
  ==========================================  ======  =============================

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PODL_PSE_ADMIN_STATE` 灞炴€ф爣璇?PoDL PSE 鍔熻兘鐨勬搷浣滅姸鎬併€侾SE
鍔熻兘鐨勬搷浣滅姸鎬佸彲浣跨敤 `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL` 鍔ㄤ綔鏇存敼銆傝灞炴€у搴斾簬
`IEEE 802.3-2018` 30.15.1.1.2 aPoDLPSEAdminState銆傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_podl_pse_admin_state

`ETHTOOL_A_C33_PSE_ADMIN_STATE` 鍚岀悊锛屽疄鐜颁簡 `IEEE 802.3-2022` 30.9.1.1.2
aPSEAdminState銆?

    :identifiers: ethtool_c33_pse_admin_state

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PODL_PSE_PW_D_STATUS` 灞炴€ф爣璇?PoDL PSE 鐨勪緵鐢垫娴嬬姸鎬併€傝鐘舵€?
鍙栧喅浜庡唴閮?PSE 鐘舵€佹満涓庤嚜鍔?PD 鍒嗙被鏀寔鎯呭喌銆傝灞炴€у搴斾簬 `IEEE 802.3-2018`
30.15.1.1.3 aPoDLPSEPowerDetectionStatus銆傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_podl_pse_pw_d_status

`ETHTOOL_A_C33_PSE_ADMIN_PW_D_STATUS` 鍚岀悊锛屽疄鐜颁簡 `IEEE 802.3-2022` 30.9.1.1.5
aPSEPowerDetectionStatus銆?

    :identifiers: ethtool_c33_pse_pw_d_status

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_PW_CLASS` 灞炴€ф爣璇?C33 PSE 鐨勫姛鐜囩瓑绾с€傚畠鍙栧喅浜?PSE 涓?
PD 涔嬮棿鍗忓晢寰楀埌鐨勭瓑绾с€傝灞炴€у搴斾簬 `IEEE 802.3-2022` 30.9.1.1.8 aPSEPowerClassification銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_ACTUAL_PW` 灞炴€ф爣璇?C33 PSE 瀹為檯娑堣€楃殑鍔熺巼銆傝灞炴€у搴斾簬
`IEEE 802.3-2022` 30.9.1.1.23 aPSEActualPower銆傚疄闄呭姛鐜囦互 mW 鎶ュ憡銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_EXT_STATE` 灞炴€ф爣璇?C33 PSE 鐨勬墿灞曢敊璇姸鎬併€傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_c33_pse_ext_state

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_EXT_SUBSTATE` 灞炴€ф爣璇?C33 PSE 鐨勬墿灞曢敊璇瓙鐘舵€併€傚彲鑳界殑
鍙栧€间负锛?

    :identifiers: ethtool_c33_pse_ext_substate_class_num_events
		  ethtool_c33_pse_ext_substate_error_condition
		  ethtool_c33_pse_ext_substate_mr_pse_enable
		  ethtool_c33_pse_ext_substate_option_detect_ted
		  ethtool_c33_pse_ext_substate_option_vport_lim
		  ethtool_c33_pse_ext_substate_ovld_detected
		  ethtool_c33_pse_ext_substate_pd_dll_power_type
		  ethtool_c33_pse_ext_substate_power_not_available
		  ethtool_c33_pse_ext_substate_short_detected

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT` 灞炴€ф爣璇嗕互 mW 涓哄崟浣嶇殑 C33 PSE 鍔熺巼闄愬埗銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_PW_LIMIT_RANGES` 宓屽灞炴€ч€氳繃
`ETHTOOL_A_C33_PSE_PWR_VAL_LIMIT_RANGE_MIN` 涓?`ETHTOOL_A_C33_PSE_PWR_VAL_LIMIT_RANGE_MAX`
鏍囪瘑 C33 PSE 鍔熺巼闄愬埗鑼冨洿銆傝嫢鎺у埗鍣ㄤ互鍥哄畾绛夌骇宸ヤ綔锛屾渶灏忓€间笌鏈€澶у€煎皢鐩哥瓑銆?

`ETHTOOL_A_PSE_PW_D_ID` 灞炴€ф爣璇?PSE 渚涚數鍩熺殑绱㈠紩銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PSE_PRIO_MAX` 灞炴€ф爣璇?PSE 鏈€澶т紭鍏堢骇鍊笺€傝缃椂锛屽彲閫夌殑
`ETHTOOL_A_PSE_PRIO` 灞炴€х敤浜庢爣璇嗗綋鍓嶉厤缃殑 PSE 浼樺厛绾с€傛湁鍏?PSE 浼樺厛绾у睘鎬х殑璇存槑锛屽弬瑙?
`PSE_SET`銆?

## PSE_SET


璁剧疆 PSE 鍙傛暟銆?

璇锋眰鍐呭锛?

  ======================================  ======  =============================
  `ETHTOOL_A_PSE_HEADER`                nested  request header
  `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL`       u32  Control PoDL PSE Admin state
  `ETHTOOL_A_C33_PSE_ADMIN_CONTROL`        u32  Control PSE Admin state
  `ETHTOOL_A_C33_PSE_AVAIL_PWR_LIMIT`      u32  Control PoE PSE available
                                                  power limit
  `ETHTOOL_A_PSE_PRIO`                     u32  Control priority of the
                                                  PoE PSE
  ======================================  ======  =============================

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL` 灞炴€х敤浜庢帶鍒?PoDL PSE 绠＄悊鍔熻兘銆傝閫夐」
瀹炵幇浜?`IEEE 802.3-2018` 30.15.1.2.1 acPoDLPSEAdminControl銆傛敮鎸佺殑鍙栧€煎弬瑙?
`ETHTOOL_A_PODL_PSE_ADMIN_STATE`銆?

`ETHTOOL_A_C33_PSE_ADMIN_CONTROL` 鍚岀悊锛屽疄鐜颁簡 `IEEE 802.3-2022` 30.9.1.2.1
acPSEAdminControl銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_C33_PSE_AVAIL_PWR_LIMIT` 灞炴€х敤浜庢帶鍒?C33 PSE 浠ユ鐡︿负鍗曚綅鐨勫彲鐢?
鍔熺巼鍊奸檺鍒躲€傝灞炴€у搴斾簬 `IEEE 802.3-2022` 33.2.4.4 鍙橀噺涓?145.2.5.4 鍙橀噺涓弿杩扮殑
`pse_available_power` 鍙橀噺涓?`pse_avail_pwr` 鍙橀噺锛屼簩鑰呬互鍔熺巼绛夌骇鎻忚堪銆?

鍐冲畾鍦ㄦ湰鎺ュ彛涓娇鐢ㄦ鐡︼紝鏄负浜嗕笌鍏跺畠鍚屾牱浣跨敤姣摝鐨勫姛鐜囩洃鎺ф帴鍙ｇ粺涓€锛屽苟涓庡悇绫讳互鐡︼紙鑰岄潪
绛夌骇锛夎褰曞姛鑰楃殑鐜版湁浜у搧淇濇寔涓€鑷淬€傚鏋滅‘瀹為渶瑕佸熀浜庣瓑绾х殑鍔熺巼闄愬埗閰嶇疆锛屽彲浠ュ湪鐢ㄦ埛绌洪棿杩涜
杞崲锛屼緥濡傞€氳繃 ethtool銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PSE_PRIO` 灞炴€х敤浜庢帶鍒?PSE 浼樺厛绾с€傚厑璁哥殑浼樺厛绾у彇鍊间粙浜?0 涓?
`ETHTOOL_A_PSE_PRIO_MAX` 灞炴€у€间箣闂淬€?

杈冨皬鐨勫€艰〃绀烘洿楂樼殑浼樺厛绾э紝鍗充紭鍏堢骇鍊间负 0 瀵瑰簲鏈€楂樼鍙ｄ紭鍏堢骇銆傜鍙ｄ紭鍏堢骇鏈変袱涓綔鐢細

 - 涓婄數椤哄簭锛氬浣嶅悗锛岀鍙ｆ寜鍏朵紭鍏堢骇浠庨珮鍒颁綆渚濇涓婄數銆備紭鍏堢骇鏇撮珮锛堝€兼洿灏忥級鐨勭鍙ｅ厛涓婄數銆?
 - 鍏抽棴椤哄簭锛氬綋鍔熺巼棰勭畻瓒呴檺鏃讹紝浼樺厛绾ф洿浣庯紙鍊兼洿澶э級鐨勭鍙ｅ厛琚叧闂€?

## PSE_NTF


閫氱煡 PSE 浜嬩欢銆?

閫氱煡鍐呭锛?

  ===============================  ======  ========================
  `ETHTOOL_A_PSE_HEADER`         nested  request header
  `ETHTOOL_A_PSE_EVENTS`         bitset  PSE events
  ===============================  ======  ========================

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PSE_EVENTS` 灞炴€ф爣璇?PSE 浜嬩欢銆?

    :identifiers: ethtool_pse_event

## RSS_GET


鑾峰彇涓庢帴鍙ｆ煇涓?RSS 涓婁笅鏂囩浉鍏崇殑闂存帴琛ㄣ€佸搱甯屽瘑閽ヤ笌鍝堝笇鍑芥暟淇℃伅锛岀被浼间簬 `ETHTOOL_GRSSH`
ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

=====================================  ======  ============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_START_CONTEXT`      u32     start context number (dumps)
=====================================  ======  ============================

`ETHTOOL_A_RSS_CONTEXT` 鎸囧畾瑕佹煡璇㈢殑 RSS 涓婁笅鏂囩紪鍙凤紱鑻ユ湭璁剧疆锛屽垯鏌ヨ涓婁笅鏂?0锛堜富涓婁笅鏂囷級銆?
dump 鍙互鎸夎澶囪繃婊わ紙鍙垪鍑虹粰瀹?netdev 鐨勪笂涓嬫枃锛夈€備笉鏀寔杩囨护鍗曚釜涓婁笅鏂囩紪鍙凤紝浣嗗彲浠ヤ娇鐢?
`ETHTOOL_A_RSS_START_CONTEXT` 浠庤缂栧彿寮€濮?dump 涓婁笅鏂囷紙涓昏鐢ㄤ簬蹇界暐涓婁笅鏂?0銆佸彧 dump
棰濆鐨勪笂涓嬫枃锛夈€?

鍐呮牳鍝嶅簲鍐呭锛?

=====================================  ======  ===============================
  `ETHTOOL_A_RSS_HEADER`             nested  reply header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
  `ETHTOOL_A_RSS_FLOW_HASH`          nested  Header fields included in hash
=====================================  ======  ===============================

ETHTOOL_A_RSS_HFUNC 灞炴€ф槸鎸囩ず鎵€鐢ㄥ搱甯屽嚱鏁扮殑浣嶅浘銆傚綋鍓嶆敮鎸佺殑閫夐」鏈?toeplitz銆亁or 鎴?crc32銆?
ETHTOOL_A_RSS_INDIR 灞炴€ц繑鍥?RSS 闂存帴琛紝鍏朵腑姣忎釜瀛楄妭琛ㄧず涓€涓槦鍒楃紪鍙枫€?
ETHTOOL_A_RSS_INPUT_XFRM 灞炴€ф槸涓€涓綅鍥撅紝鎸囩ず鍦ㄩ€佺粰 RSS hfunc 涔嬪墠瀵硅緭鍏ュ崗璁瓧娈垫墍搴旂敤鐨?
杞崲绫诲瀷銆傚綋鍓嶆敮鎸佺殑閫夐」鏈?symmetric-xor 涓?symmetric-or-xor銆?
ETHTOOL_A_RSS_FLOW_HASH 鎼哄甫姣忎釜娴佺被鍨嬬殑浣嶆帺鐮侊紝鎸囩ず鍝簺澶村瓧娈佃鍖呭惈鍦ㄥ搱甯岃绠椾腑銆?

## RSS_SET


璇锋眰鍐呭锛?

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
  `ETHTOOL_A_RSS_FLOW_HASH`          nested  Header fields included in hash
=====================================  ======  ==============================

`ETHTOOL_A_RSS_INDIR` 鏄敤鎴锋湡鏈涚殑鏈€灏?RSS 琛ㄣ€傝嫢鍏跺皬浜庤澶囨敮鎸佺殑鏈€灏忚〃澶у皬锛屽唴鏍镐笌璁惧
椹卞姩鍙兘浼氬鍒惰琛ㄣ€備緥濡傦紝鑻ョ敤鎴疯姹?`[0, 1]`锛屼絾璁惧鑷冲皯闇€瑕?8 涓潯鐩紝鍒欏疄闄呬娇鐢ㄧ殑琛ㄥ皢
鍙樹负 `[0, 1, 0, 1, 0, 1, 0, 1]`銆傚ぇ澶氭暟璁惧瑕佹眰琛ㄥぇ灏忎负 2 鐨勫箓锛屽洜姝ゅぇ灏忎笉鏄?2 鐨勫箓鐨勮〃
寰堝彲鑳借鎷掔粷銆備娇鐢ㄥぇ灏忎负 0 鐨勮〃浼氬皢闂存帴琛ㄩ噸缃负榛樿鍊笺€?

## RSS_CREATE_ACT


璇锋眰鍐呭锛?

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
=====================================  ======  ==============================

鍐呮牳鍝嶅簲鍐呭锛?

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
=====================================  ======  ==============================

鍒涘缓涓€涓澶栫殑 RSS 涓婁笅鏂囷紱鑻ユ湭鎸囧畾 `ETHTOOL_A_RSS_CONTEXT`锛屽唴鏍稿皢鑷姩鍒嗛厤涓€涓€?

## RSS_DELETE_ACT


璇锋眰鍐呭锛?

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
=====================================  ======  ==============================

鍒犻櫎涓€涓澶栫殑 RSS 涓婁笅鏂囥€?

## PLCA_GET_CFG


鑾峰彇 IEEE 802.3cg-2019 绗?148 鏉＄墿鐞嗗眰鍐茬獊閬垮厤锛圥LCA锛夊崗璋冨瓙灞傦紙RS锛夊睘鎬с€?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PLCA_HEADER`              nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  reply header
  `ETHTOOL_A_PLCA_VERSION`              u16     Supported PLCA management
                                                  interface standard/version
  `ETHTOOL_A_PLCA_ENABLED`              u8      PLCA Admin State
  `ETHTOOL_A_PLCA_NODE_ID`              u32     PLCA unique local node ID
  `ETHTOOL_A_PLCA_NODE_CNT`             u32     Number of PLCA nodes on the
                                                  network, including the
                                                  coordinator
  `ETHTOOL_A_PLCA_TO_TMR`               u32     Transmit Opportunity Timer
                                                  value in bit-times (BT)
  `ETHTOOL_A_PLCA_BURST_CNT`            u32     Number of additional packets
                                                  the node is allowed to send
                                                  within a single TO
  `ETHTOOL_A_PLCA_BURST_TMR`            u32     Time to wait for the MAC to
                                                  transmit a new frame before
                                                  terminating the burst
  ======================================  ======  =============================

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_VERSION` 灞炴€ф寚绀?PLCA 绠＄悊鎺ュ彛鎵€绗﹀悎鐨勬爣鍑嗕笌鐗堟湰銆傝嫢鏈缃紝
璇ユ帴鍙ｄ负渚涘簲鍟嗙壒瀹氱殑锛屽苟锛堝彲鑳斤級鐢遍┍鍔ㄦ彁渚涖€侽PEN Alliance SIG 涓哄唴宓?PLCA 鍗忚皟瀛愬眰鐨?
10BASE-T1S PHY 瑙勫畾浜嗘爣鍑嗗瘎瀛樺櫒鏄犲皠銆傚弬瑙?https://www.opensig.org/about/specifications/ 涓婄殑
鈥?0BASE-T1S PLCA Management Registers鈥濄€?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_ENABLED` 灞炴€ф寚绀?PLCA RS 鐨勭鐞嗙姸鎬併€傝嫢鏈缃紝鑺傜偣杩愯鍦?
鈥減lain鈥?CSMA/CD 妯″紡涓嬨€傝閫夐」瀵瑰簲浜?`IEEE 802.3cg-2019` 30.16.1.1.1
aPLCAAdminState / 30.16.1.2.1 acPLCAAdminControl銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_NODE_ID` 灞炴€ф寚绀?PHY 閰嶇疆濂界殑鏈湴鑺傜偣 ID銆傝 ID 鍐冲畾浜嗕负
鑺傜偣棰勭暀鐢ㄤ簬鍙戦€佺殑鍙戦€佹満浼氾紙TO锛夈€傝閫夐」瀵瑰簲浜?`IEEE 802.3cg-2019` 30.16.1.1.4
aPLCALocalNodeID銆傝灞炴€х殑鏈夋晥鑼冨洿涓?[0 .. 255]锛屽叾涓?255 琛ㄧず鈥滄湭閰嶇疆鈥濄€?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_NODE_CNT` 灞炴€ф寚绀烘贩鍚堟涓婇厤缃殑 PLCA 鑺傜偣鏈€澶ф暟閲忋€傝鏁板瓧
鍐冲畾浜嗗湪涓€涓?PLCA 鍛ㄦ湡鍐呯敓鎴愮殑鍙戦€佹満浼氭€绘暟銆傝灞炴€т粎涓?PLCA 鍗忚皟鍣紙鍗?aPLCALocalNodeID
璁句负 0 鐨勮妭鐐癸級鐩稿叧锛岃窡闅忚妭鐐瑰拷鐣ユ璁剧疆銆傝閫夐」瀵瑰簲浜?`IEEE 802.3cg-2019` 30.16.1.1.3
aPLCANodeCount銆傝灞炴€х殑鏈夋晥鑼冨洿涓?[1 .. 255]銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_TO_TMR` 灞炴€ф寚绀轰互浣嶆椂闂达紙bit-times锛変负鍗曚綅鐨勫彂閫佹満浼氬畾鏃跺櫒
閰嶇疆鍊笺€備负浜嗚 PLCA 姝ｅ父宸ヤ綔锛屽叡浜粙璐ㄧ殑鎵€鏈夎妭鐐规鍊煎繀椤昏涓虹浉绛夈€傝閫夐」瀵瑰簲浜?
`IEEE 802.3cg-2019` 30.16.1.1.5 aPLCATransmitOpportunityTimer銆傝灞炴€х殑鏈夋晥鑼冨洿涓?[0 .. 255]銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_BURST_CNT` 灞炴€ф寚绀鸿妭鐐瑰湪鍗曚釜鍙戦€佹満浼氬唴鍏佽鍙戦€佺殑棰濆鍖呮暟閲忋€?
榛樿鎯呭喌涓嬭灞炴€т负 0锛岃〃绀鸿妭鐐规瘡涓?TO 鍙兘鍙戦€佸崟涓抚銆傚綋澶т簬 0 鏃讹紝PLCA RS 浼氬湪浠绘剰鍙戦€佸悗
淇濇寔璇?TO锛岀瓑寰?MAC 鍦ㄦ渶澶?aPLCABurstTimer 涓綅鏃堕棿鍐呭彂閫佹柊甯с€傚湪涓€涓?PLCA 鍛ㄦ湡鍐呰繖绉嶆儏鍐?
鏈€澶氬彂鐢熸湰鍙傛暟鎵€鎸囧畾娆℃暟锛屼箣鍚庣獊鍙戠粨鏉燂紝姝ｅ父鐨?TO 璁℃暟鎭㈠銆傝閫夐」瀵瑰簲浜?`IEEE 802.3cg-2019`
30.16.1.1.6 aPLCAMaxBurstCount銆傝灞炴€х殑鏈夋晥鑼冨洿涓?[0 .. 255]銆?

璁剧疆鏃讹紝鍙€夌殑 `ETHTOOL_A_PLCA_BURST_TMR` 灞炴€ф寚绀哄綋 aPLCAMaxBurstCount 澶т簬 0 鏃讹紝PLCA RS
绛夊緟 MAC 鍙戣捣鏂板彂閫佺殑浣嶆椂闂存暟銆傝嫢 MAC 鍦ㄦ鏃堕棿鍐呮湭鑳藉彂閫佹柊甯э紝绐佸彂缁撴潫锛孴O 璁℃暟鎭㈠銆傚惁鍒欙紝
鏂板抚浣滀负褰撳墠绐佸彂鐨勪竴閮ㄥ垎琚彂閫併€傝閫夐」瀵瑰簲浜?`IEEE 802.3cg-2019` 30.16.1.1.7
aPLCABurstTimer銆傝灞炴€х殑鏈夋晥鑼冨洿涓?[0 .. 255]銆備笉杩囷紝涓轰簡璁?PLCA 绐佸彂妯″紡鎸夐鏈熷伐浣滐紝璇ュ€?
搴旇缃负澶т簬 MAC 鐨勫抚闂撮棿闅旓紙IFG锛夋椂闂达紙骞剁暀鏈変竴瀹氫綑閲忥級銆?

## PLCA_SET_CFG


璁剧疆 PLCA RS 鍙傛暟銆?

璇锋眰鍐呭锛?

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  request header
  `ETHTOOL_A_PLCA_ENABLED`              u8      PLCA Admin State
  `ETHTOOL_A_PLCA_NODE_ID`              u8      PLCA unique local node ID
  `ETHTOOL_A_PLCA_NODE_CNT`             u8      Number of PLCA nodes on the
                                                  network, including the
                                                  coordinator
  `ETHTOOL_A_PLCA_TO_TMR`               u8      Transmit Opportunity Timer
                                                  value in bit-times (BT)
  `ETHTOOL_A_PLCA_BURST_CNT`            u8      Number of additional packets
                                                  the node is allowed to send
                                                  within a single TO
  `ETHTOOL_A_PLCA_BURST_TMR`            u8      Time to wait for the MAC to
                                                  transmit a new frame before
                                                  terminating the burst
  ======================================  ======  =============================

鍚勫睘鎬х殑璇存槑鍙傝 `PLCA_GET_CFG`銆?

## PLCA_GET_STATUS


鑾峰彇 PLCA RS 鐘舵€佷俊鎭€?

璇锋眰鍐呭锛?

  =====================================  ======  ==========================
  `ETHTOOL_A_PLCA_HEADER`              nested  request header
  =====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  reply header
  `ETHTOOL_A_PLCA_STATUS`               u8      PLCA RS operational status
  ======================================  ======  =============================

璁剧疆鏃讹紝`ETHTOOL_A_PLCA_STATUS` 灞炴€ф寚绀鸿妭鐐规槸鍚︽娴嬪埌缃戠粶涓?BEACON 鐨勫瓨鍦ㄣ€傝鏍囧織瀵瑰簲浜?
`IEEE 802.3cg-2019` 30.16.1.1.2 aPLCAStatus銆?

## MM_GET


鑾峰彇 802.3 MAC 鍚堝苟锛圡AC Merge锛夊弬鏁般€?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_MM_HEADER`               nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  =================================  ======  ===================================
  `ETHTOOL_A_MM_HEADER`            nested  request header
  `ETHTOOL_A_MM_PMAC_ENABLED`      bool    鑻ュ惎鐢ㄥ彲鎶㈠崰甯т笌 SMD-V 甯х殑鎺ユ敹鍒欑疆浣?
  `ETHTOOL_A_MM_TX_ENABLED`        bool    鑻ョ鐞嗕笂鍚敤浜嗗彲鎶㈠崰甯х殑鍙戦€佸垯缃綅
                                            锛堣嫢楠岃瘉澶辫触鍙兘鏈縺娲伙級
  `ETHTOOL_A_MM_TX_ACTIVE`         bool    鑻ユ搷浣滀笂鍚敤浜嗗彲鎶㈠崰甯х殑鍙戦€佸垯缃綅
  `ETHTOOL_A_MM_TX_MIN_FRAG_SIZE`  u32     鍙戦€佺殑闈炴湯灏剧墖娈电殑鏈€灏忓ぇ灏忥紝浠ュ叓浣嶇粍璁?
  `ETHTOOL_A_MM_RX_MIN_FRAG_SIZE`  u32     鎺ユ敹鐨勯潪鏈熬鐗囨鐨勬渶灏忓ぇ灏忥紝浠ュ叓浣嶇粍璁?
  `ETHTOOL_A_MM_VERIFY_ENABLED`    bool    鑻ョ鐞嗕笂鍚敤浜?SMD-V 甯х殑鍙戦€佸垯缃綅
  `ETHTOOL_A_MM_VERIFY_STATUS`     u8      verification 鍔熻兘鐨勭姸鎬?
  `ETHTOOL_A_MM_VERIFY_TIME`       u32     涓ゆ楠岃瘉灏濊瘯涔嬮棿鐨勫欢杩?
  `ETHTOOL_A_MM_MAX_VERIFY_TIME``  u32     maximum verification interval
                                             supported by device
  `ETHTOOL_A_MM_STATS`             nested  IEEE 802.3-2018 瀛愭潯娆?30.14.1
                                             oMACMergeEntity 缁熻璁℃暟鍣?
  =================================  ======  ===================================

杩欎簺灞炴€х敱璁惧椹卞姩閫氳繃浠ヤ笅缁撴瀯濉厖锛?

    :identifiers: ethtool_mm_state

`ETHTOOL_A_MM_VERIFY_STATUS` 灏嗘姤鍛婃潵鑷互涓嬪彇鍊间箣涓€锛?

    :identifiers: ethtool_mm_verify_status

鑻ュ湪 `MM_SET` 鍛戒护涓?`ETHTOOL_A_MM_VERIFY_ENABLED` 浠?false 浼犲叆锛屽垯
`ETHTOOL_A_MM_VERIFY_STATUS` 灏嗘姤鍛?`ETHTOOL_MM_VERIFY_STATUS_INITIAL` 鎴?
`ETHTOOL_MM_VERIFY_STATUS_DISABLED`锛屽惁鍒欏簲鎶ュ憡鍏跺畠鏌愪釜鐘舵€併€?

寤鸿椹卞姩浠?pMAC 绂佺敤鐘舵€佸惎鍔紝骞跺湪鐢ㄦ埛绌洪棿璇锋眰鏃跺惎鐢ㄥ畠銆傚悓鏃跺缓璁敤鎴风┖闂翠笉瑕佷緷璧?
`ETHTOOL_MSG_MM_GET` 璇锋眰鐨勯粯璁ゅ€笺€?

鑻?`ETHTOOL_A_HEADER_FLAGS` 涓缃簡 `ETHTOOL_FLAG_STATS`锛屽垯浼氭姤鍛?`ETHTOOL_A_MM_STATS`銆?
濡傛灉椹卞姩鏈姤鍛婁换浣曠粺璁′俊鎭紝璇ュ睘鎬у皢涓虹┖銆傞┍鍔ㄥ湪浠ヤ笅缁撴瀯涓～鍐欑粺璁′俊鎭細

    :identifiers: ethtool_mm_stats

## MM_SET


淇敼 802.3 MAC 鍚堝苟灞傜殑閰嶇疆銆?

璇锋眰鍐呭锛?

  =================================  ======  ==========================
  `ETHTOOL_A_MM_VERIFY_TIME`       u32     see MM_GET description
  `ETHTOOL_A_MM_VERIFY_ENABLED`    bool    see MM_GET description
  `ETHTOOL_A_MM_TX_ENABLED`        bool    see MM_GET description
  `ETHTOOL_A_MM_PMAC_ENABLED`      bool    see MM_GET description
  `ETHTOOL_A_MM_TX_MIN_FRAG_SIZE`  u32     see MM_GET description
  =================================  ======  ==========================

杩欎簺灞炴€ч€氳繃浠ヤ笅缁撴瀯浼犳挱缁欓┍鍔細

    :identifiers: ethtool_mm_cfg

## MODULE_FW_FLASH_ACT


鐑у綍鏀跺彂鍣ㄦā鍧楀浐浠躲€?

璇锋眰鍐呭锛?

  =======================================  ======  ===========================
  `ETHTOOL_A_MODULE_FW_FLASH_HEADER`     nested  request header
  `ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME`  string  firmware image file name
  `ETHTOOL_A_MODULE_FW_FLASH_PASSWORD`   u32     transceiver module password
  =======================================  ===========================

鍥轰欢鏇存柊杩囩▼鐢变笁涓€昏緫姝ラ缁勬垚锛?

1. 灏嗗浐浠舵槧鍍忎笅杞藉埌鏀跺彂鍣ㄦā鍧楀苟鏍￠獙瀹冦€?
2. 杩愯鍥轰欢鏄犲儚銆?
3. 鎻愪氦鍥轰欢鏄犲儚锛屼娇鍏跺湪澶嶄綅鍚庤繍琛屻€?

缁欏畾鐑у綍鍛戒护鍚庯紝杩欎笁涓楠ゆ寜椤哄簭鎵ц銆?

璇ユ秷鎭粎璋冨害鏇存柊杩囩▼骞剁珛鍗宠繑鍥烇紝涓嶄細闃诲銆傞殢鍚庤杩囩▼寮傛杩愯銆傜敱浜庡畬鎴愬彲鑳介渶瑕佹暟鍒嗛挓锛?
鍦ㄦ洿鏂拌繃绋嬩腑鍐呮牳浼氬悜鐢ㄦ埛绌洪棿鍙戝嚭閫氱煡锛屾洿鏂板叾鐘舵€佷笌杩涘害銆?

`ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME` 灞炴€х紪鐮佸浐浠舵槧鍍忔枃浠跺悕銆傚浐浠舵槧鍍忚涓嬭浇鍒版敹鍙戝櫒妯″潡銆?
鏍￠獙銆佽繍琛屽苟鎻愪氦銆?

鍙€夌殑 `ETHTOOL_A_MODULE_FW_FLASH_PASSWORD` 灞炴€х紪鐮佷竴涓瘑鐮侊紝璇ュ瘑鐮佸彲鑳戒綔涓烘敹鍙戝櫒妯″潡
鍥轰欢鏇存柊杩囩▼鐨勪竴閮ㄥ垎琚渶瑕併€?

鍥轰欢鏇存柊杩囩▼鍙兘闇€瑕佹暟鍒嗛挓鎵嶈兘瀹屾垚銆傚洜姝わ紝鍦ㄦ洿鏂拌繃绋嬩腑鍐呮牳浼氬悜鐢ㄦ埛绌洪棿鍙戝嚭閫氱煡锛屾洿鏂板叾
鐘舵€佷笌杩涘害銆?


閫氱煡鍐呭锛?

 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_HEADER`              | nested | reply header   |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_STATUS`              | u32    | status         |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_STATUS_MSG`          | string | status message |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_DONE`                | uint   | progress       |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_TOTAL`               | uint   | total          |
 +---------------------------------------------------+--------+----------------+

`ETHTOOL_A_MODULE_FW_FLASH_STATUS` 灞炴€х紪鐮佸浐浠舵洿鏂拌繃绋嬬殑褰撳墠鐘舵€併€傚彲鑳界殑鍙栧€间负锛?

    :identifiers: ethtool_module_fw_flash_status

`ETHTOOL_A_MODULE_FW_FLASH_STATUS_MSG` 灞炴€х紪鐮佺姸鎬佹秷鎭瓧绗︿覆銆?

`ETHTOOL_A_MODULE_FW_FLASH_DONE` 涓?`ETHTOOL_A_MODULE_FW_FLASH_TOTAL` 灞炴€у垎鍒紪鐮佸凡瀹屾垚涓?
鎬荤殑宸ヤ綔閲忋€?

## PHY_GET


鑾峰彇閾捐矾涓婄粰瀹氫互澶綉 PHY 鐨勪俊鎭€侱O 鎿嶄綔杩斿洖鍏充簬 dev->phydev 鐨勬墍鏈夊彲鐢ㄤ俊鎭€傜敤鎴蜂篃鍙互
鎸囧畾 PHY_INDEX锛屾鏃?DO 璇锋眰杩斿洖鍏充簬璇ョ壒瀹?PHY 鐨勪俊鎭€?

鐢变簬鍙兘瀛樺湪澶氫簬涓€涓?PHY锛屽彲浠ヤ娇鐢?DUMP 鎿嶄綔锛岄€氳繃鍦?dump 璇锋眰涓紶鍏ユ帴鍙ｇ储寮曟垨鍚嶇О锛屾潵
鍒楀嚭缁欏畾鎺ュ彛涓婂瓨鍦ㄧ殑 PHY銆?

鏇村淇℃伅鍙傝 phy_link_topology銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_PHY_HEADER`              nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ===================================== ======  ===============================
  `ETHTOOL_A_PHY_HEADER`              nested  request header
  `ETHTOOL_A_PHY_INDEX`               u32     phy 鐨勫敮涓€绱㈠紩锛屽彲鐢ㄤ簬閽堝璇?phy 鐨勮姹?
  `ETHTOOL_A_PHY_DRVNAME`             string  璇?phy 鐨勯┍鍔ㄥ悕
  `ETHTOOL_A_PHY_NAME`                string  璇?phy 鐨勮澶囧悕
  `ETHTOOL_A_PHY_UPSTREAM_TYPE`       u32     璇?phy 鎵€杩炴帴璁惧鐨勭被鍨?
  `ETHTOOL_A_PHY_UPSTREAM_INDEX`      u32     涓婃父 PHY 鐨?PHY 绱㈠紩
  `ETHTOOL_A_PHY_UPSTREAM_SFP_NAME`   string  鑻ヨ PHY 閫氳繃 SFP 鎬荤嚎杩炴帴鍒板叾鐖?PHY锛岃 sfp 鎬荤嚎鐨勫悕绉?
  `ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME` string  鑻ヨ phy 鎺у埗涓€涓?sfp 鎬荤嚎锛岃 sfp 鎬荤嚎鐨勫悕绉?
  ===================================== ======  ===============================

褰?`ETHTOOL_A_PHY_UPSTREAM_TYPE` 涓?PHY_UPSTREAM_PHY 鏃讹紝璇?PHY 鐨勭埗绾ф槸鍙︿竴涓?PHY銆?

## TSCONFIG_GET


鑾峰彇褰撳墠纭欢鏃堕棿鎴虫簮涓庨厤缃殑鐩稿叧淇℃伅銆?

瀹冪被浼间簬宸插簾寮冪殑 `SIOCGHWTSTAMP` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?

  ====================================  ======  ==========================
  `ETHTOOL_A_TSCONFIG_HEADER`         nested  request header
  ====================================  ======  ==========================

鍐呮牳鍝嶅簲鍐呭锛?

  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

璁剧疆鏃讹紝`ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` 灞炴€ф爣璇嗙‖浠舵椂闂存埑鎻愪緵鑰呯殑鏉ユ簮銆傚畠鐢辨弿杩?PTP
璁惧绱㈠紩鐨?`ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX` 灞炴€э紝浠ュ強鎻忚堪鏃堕棿鎴抽檺瀹氱鐨?
`ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER` 灞炴€х粍鎴愩€?

璁剧疆鏃讹紝`ETHTOOL_A_TSCONFIG_TX_TYPES`銆乣ETHTOOL_A_TSCONFIG_RX_FILTERS` 涓?
`ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS` 灞炴€ф爣璇嗗綋鍓嶇‖浠舵椂闂存埑鎻愪緵鑰呮墍閰嶇疆鐨?Tx 绫诲瀷銆丷x 杩囨护鍣?
涓庢爣蹇椼€傝繖浜涘睘鎬ч€氳繃浠ヤ笅缁撴瀯浼犳挱缁欓┍鍔細

    :identifiers: kernel_hwtstamp_config

## TSCONFIG_SET


璁剧疆褰撳墠纭欢鏃堕棿鎴虫簮涓庨厤缃殑鐩稿叧淇℃伅銆?

瀹冪被浼间簬宸插簾寮冪殑 `SIOCSHWTSTAMP` ioctl 璇锋眰銆?

璇锋眰鍐呭锛?


  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

鍐呮牳鍝嶅簲鍐呭锛?

  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

鍚勫睘鎬х殑璇存槑鍙傝 `TSCONFIG_GET`銆?

## MSE_GET


浠?PHY 鑾峰彇璇︾粏鐨勫钩鍧囧钩鏂硅宸紙Mean Square Error锛孧SE锛夎瘖鏂俊鎭€?

璇锋眰鍐呭锛?

  ====================================  ======  ============================
  `ETHTOOL_A_MSE_HEADER`              nested  request header
  ====================================  ======  ============================

鍐呮牳鍝嶅簲鍐呭锛?

  ====================================  ======  ================================
  `ETHTOOL_A_MSE_HEADER`              nested  reply header
  `ETHTOOL_A_MSE_CAPABILITIES`        nested  MSE 娴嬮噺鐨勮兘鍔?姣斾緥淇℃伅
  `ETHTOOL_A_MSE_CHANNEL_A`           nested  Channel A 鐨勫揩鐓?
  `ETHTOOL_A_MSE_CHANNEL_B`           nested  Channel B 鐨勫揩鐓?
  `ETHTOOL_A_MSE_CHANNEL_C`           nested  Channel C 鐨勫揩鐓?
  `ETHTOOL_A_MSE_CHANNEL_D`           nested  Channel D 鐨勫揩鐓?
  `ETHTOOL_A_MSE_WORST_CHANNEL`       nested  鏈€宸€氶亾鐨勫揩鐓?
  `ETHTOOL_A_MSE_LINK`                nested  閾捐矾绾ц仛鍚堢殑蹇収
  ====================================  ======  ================================

### MSE 鑳藉姏


杩欎釜宓屽灞炴€ф姤鍛婄敤浜庤В閲婂揩鐓у€肩殑鑳藉姏 / 缂╂斁灞炴€с€?

  ============================================== ======  =========================
  `ETHTOOL_A_MSE_CAPABILITIES_MAX_AVERAGE_MSE` uint    鏈€澶?avg_mse 姣斾緥
  `ETHTOOL_A_MSE_CAPABILITIES_MAX_PEAK_MSE`    uint    鏈€澶?peak_mse 姣斾緥
  `ETHTOOL_A_MSE_CAPABILITIES_REFRESH_RATE_PS` uint    閲囨牱鐜囷紙鐨锛?
  `ETHTOOL_A_MSE_CAPABILITIES_NUM_SYMBOLS`     uint    姣忎釜纭欢閲囨牱鐨勭鍙锋暟
  ============================================== ======  =========================

max-average/peak 瀛楁浠呭湪 PHY 鏀寔鐩稿簲鎸囨爣鏃舵墠鍖呭惈銆傚畠浠殑缂哄け琛ㄧず璇ユ寚鏍囦笉鍙敤銆?

鍙傝 `include/linux/phy.h` 涓?`struct phy_mse_capability` 鐨勫唴鏍告枃妗ｃ€?

### MSE 蹇収


姣忎釜姣忛€氶亾宓屽鍖呭惈璇ラ€夋嫨鍣紙閫氶亾 A/B/C/D銆佹渶宸€氶亾鎴栭摼璺級鐨?MSE 鍊肩殑鍘熷瓙蹇収銆?

  ==========================================  ======  ===================
  `ETHTOOL_A_MSE_SNAPSHOT_AVERAGE_MSE`      uint    骞冲潎 MSE 鍊?
  `ETHTOOL_A_MSE_SNAPSHOT_PEAK_MSE`         uint    褰撳墠宄板€?MSE
  `ETHTOOL_A_MSE_SNAPSHOT_WORST_PEAK_MSE`   uint    鏈€鍧忔儏鍐靛嘲鍊?MSE
  ==========================================  ======  ===================

鍦ㄦ瘡涓€氶亾宓屽涓紝浠呬細鍑虹幇 PHY 鎵€鏀寔鐨勬寚鏍囥€?

鍙傝 `include/linux/phy.h` 涓?`struct phy_mse_snapshot` 鐨勫唴鏍告枃妗ｃ€?

## 璇锋眰缈昏瘧


涓嬭〃灏?ioctl 鍛戒护鏄犲皠鍒版彁渚涘叾鍔熻兘鐨?netlink 鍛戒护銆傚彸鍒椾负鈥渘/a鈥濈殑鏉＄洰鏄皻鏃?netlink 鏇夸唬
鐨勫懡浠ゃ€傚乏鍒椾负鈥渘/a鈥濈殑鏉＄洰鍒欎粎瀛樺湪浜?netlink銆?

  =================================== =====================================
  ioctl command                       netlink command
  =================================== =====================================
  `ETHTOOL_GSET`                    `ETHTOOL_MSG_LINKINFO_GET`
                                      `ETHTOOL_MSG_LINKMODES_GET`
  `ETHTOOL_SSET`                    `ETHTOOL_MSG_LINKINFO_SET`
                                      `ETHTOOL_MSG_LINKMODES_SET`
  `ETHTOOL_GDRVINFO`                n/a
  `ETHTOOL_GREGS`                   n/a
  `ETHTOOL_GWOL`                    `ETHTOOL_MSG_WOL_GET`
  `ETHTOOL_SWOL`                    `ETHTOOL_MSG_WOL_SET`
  `ETHTOOL_GMSGLVL`                 `ETHTOOL_MSG_DEBUG_GET`
  `ETHTOOL_SMSGLVL`                 `ETHTOOL_MSG_DEBUG_SET`
  `ETHTOOL_NWAY_RST`                n/a
  `ETHTOOL_GLINK`                   `ETHTOOL_MSG_LINKSTATE_GET`
  `ETHTOOL_GEEPROM`                 n/a
  `ETHTOOL_SEEPROM`                 n/a
  `ETHTOOL_GCOALESCE`               `ETHTOOL_MSG_COALESCE_GET`
  `ETHTOOL_SCOALESCE`               `ETHTOOL_MSG_COALESCE_SET`
  `ETHTOOL_GRINGPARAM`              `ETHTOOL_MSG_RINGS_GET`
  `ETHTOOL_SRINGPARAM`              `ETHTOOL_MSG_RINGS_SET`
  `ETHTOOL_GPAUSEPARAM`             `ETHTOOL_MSG_PAUSE_GET`
  `ETHTOOL_SPAUSEPARAM`             `ETHTOOL_MSG_PAUSE_SET`
  `ETHTOOL_GRXCSUM`                 `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SRXCSUM`                 `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GTXCSUM`                 `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_STXCSUM`                 `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GSG`                     `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SSG`                     `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_TEST`                    n/a
  `ETHTOOL_GSTRINGS`                `ETHTOOL_MSG_STRSET_GET`
  `ETHTOOL_PHYS_ID`                 n/a
  `ETHTOOL_GSTATS`                  n/a
  `ETHTOOL_GTSO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_STSO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GPERMADDR`               rtnetlink `RTM_GETLINK`
  `ETHTOOL_GUFO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SUFO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GGSO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SGSO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GFLAGS`                  `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SFLAGS`                  `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GPFLAGS`                 `ETHTOOL_MSG_PRIVFLAGS_GET`
  `ETHTOOL_SPFLAGS`                 `ETHTOOL_MSG_PRIVFLAGS_SET`
  `ETHTOOL_GRXFH`                   `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRXFH`                   `ETHTOOL_MSG_RSS_SET`
  `ETHTOOL_GGRO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SGRO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GRXRINGS`                n/a
  `ETHTOOL_GRXCLSRLCNT`             n/a
  `ETHTOOL_GRXCLSRULE`              n/a
  `ETHTOOL_GRXCLSRLALL`             n/a
  `ETHTOOL_SRXCLSRLDEL`             n/a
  `ETHTOOL_SRXCLSRLINS`             n/a
  `ETHTOOL_FLASHDEV`                n/a
  `ETHTOOL_RESET`                   n/a
  `ETHTOOL_SRXNTUPLE`               n/a
  `ETHTOOL_GRXNTUPLE`               n/a
  `ETHTOOL_GSSET_INFO`              `ETHTOOL_MSG_STRSET_GET`
  `ETHTOOL_GRXFHINDIR`              `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRXFHINDIR`              `ETHTOOL_MSG_RSS_SET`
  `ETHTOOL_GFEATURES`               `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SFEATURES`               `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GCHANNELS`               `ETHTOOL_MSG_CHANNELS_GET`
  `ETHTOOL_SCHANNELS`               `ETHTOOL_MSG_CHANNELS_SET`
  `ETHTOOL_SET_DUMP`                n/a
  `ETHTOOL_GET_DUMP_FLAG`           n/a
  `ETHTOOL_GET_DUMP_DATA`           n/a
  `ETHTOOL_GET_TS_INFO`             `ETHTOOL_MSG_TSINFO_GET`
  `ETHTOOL_GMODULEINFO`             `ETHTOOL_MSG_MODULE_EEPROM_GET`
  `ETHTOOL_GMODULEEEPROM`           `ETHTOOL_MSG_MODULE_EEPROM_GET`
  `ETHTOOL_GEEE`                    `ETHTOOL_MSG_EEE_GET`
  `ETHTOOL_SEEE`                    `ETHTOOL_MSG_EEE_SET`
  `ETHTOOL_GRSSH`                   `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRSSH`                   n/a
  `ETHTOOL_GTUNABLE`                n/a
  `ETHTOOL_STUNABLE`                n/a
  `ETHTOOL_GPHYSTATS`               n/a
  `ETHTOOL_PERQUEUE`                n/a
  `ETHTOOL_GLINKSETTINGS`           `ETHTOOL_MSG_LINKINFO_GET`
                                      `ETHTOOL_MSG_LINKMODES_GET`
  `ETHTOOL_SLINKSETTINGS`           `ETHTOOL_MSG_LINKINFO_SET`
                                      `ETHTOOL_MSG_LINKMODES_SET`
  `ETHTOOL_PHY_GTUNABLE`            n/a
  `ETHTOOL_PHY_STUNABLE`            n/a
  `ETHTOOL_GFECPARAM`               `ETHTOOL_MSG_FEC_GET`
  `ETHTOOL_SFECPARAM`               `ETHTOOL_MSG_FEC_SET`
  n/a                                 `ETHTOOL_MSG_CABLE_TEST_ACT`
  n/a                                 `ETHTOOL_MSG_CABLE_TEST_TDR_ACT`
  n/a                                 `ETHTOOL_MSG_TUNNEL_INFO_GET`
  n/a                                 `ETHTOOL_MSG_PHC_VCLOCKS_GET`
  n/a                                 `ETHTOOL_MSG_MODULE_GET`
  n/a                                 `ETHTOOL_MSG_MODULE_SET`
  n/a                                 `ETHTOOL_MSG_PLCA_GET_CFG`
  n/a                                 `ETHTOOL_MSG_PLCA_SET_CFG`
  n/a                                 `ETHTOOL_MSG_PLCA_GET_STATUS`
  n/a                                 `ETHTOOL_MSG_MM_GET`
  n/a                                 `ETHTOOL_MSG_MM_SET`
  n/a                                 `ETHTOOL_MSG_MODULE_FW_FLASH_ACT`
  n/a                                 `ETHTOOL_MSG_PHY_GET`
  `SIOCGHWTSTAMP`                   `ETHTOOL_MSG_TSCONFIG_GET`
  `SIOCSHWTSTAMP`                   `ETHTOOL_MSG_TSCONFIG_SET`
  =================================== =====================================
