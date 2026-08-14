
## 鎿嶄綔鐘舵€?

## 1. 绠€浠?

Linux 鍖哄垎鎺ュ彛鐨勭鐞嗙姸鎬侊紙administrative state锛変笌鎿嶄綔鐘舵€侊紙operational state锛夈€?绠＄悊鐘舵€佹槸鈥渋p link set dev <dev> up 鎴?down鈥濈殑缁撴灉锛屽弽鏄犵鐞嗗憳鏄惁甯屾湜浣跨敤璇?璁惧浼犺緭娴侀噺銆?
鐒惰€岋紝鎺ュ彛骞堕潪浠呬粎鍥犱负绠＄悊鍛樺惎鐢ㄤ簡瀹冨氨鍙敤鈥斺€斾互澶綉闇€瑕佹彃鍒颁氦鎹㈡満涓婏紝骞朵笖鏍规嵁
绔欑偣鐨勭綉缁滅瓥鐣ヤ笌閰嶇疆锛屽湪鐢ㄦ埛鏁版嵁浼犺緭涔嬪墠杩橀渶瑕佽繘琛?802.1X 璁よ瘉銆傛搷浣滅姸鎬佸弽鏄?浜嗕竴涓帴鍙ｄ紶杈撹繖浜涚敤鎴锋暟鎹殑鑳藉姏銆?
寰楃泭浜?802.1X锛屽繀椤诲厑璁哥敤鎴风┖闂村奖鍝嶆搷浣滅姸鎬併€備负姝わ紝鎿嶄綔鐘舵€佽鎷嗗垎涓轰袱閮ㄥ垎锛氫袱涓?鍙兘鐢遍┍鍔ㄨ缃殑鏍囧織锛屼互鍙婁竴涓敱杩欎簺鏍囧織銆佷竴椤圭瓥鐣ヤ互鍙婂湪鏌愪簺瑙勫垯涓嬪彲鐢辩敤鎴风┖闂?鏇存敼鐨勩€佷笌 RFC2863 鍏煎鐨勭姸鎬併€?

## 2. 浠庣敤鎴风┖闂存煡璇?

绠＄悊鐘舵€佷笌鎿嶄綔鐘舵€侀兘鍙互閫氳繃 netlink 鎿嶄綔 RTM_GETLINK 鏌ヨ銆備篃鍙互璁㈤槄
RTNLGRP_LINK 浠ュ湪鎺ュ彛澶勪簬绠＄悊 up 鏃舵敹鍒版洿鏂伴€氱煡銆傝繖瀵逛簬浠庣敤鎴风┖闂磋繘琛岃缃緢閲嶈銆?
杩欎簺鍊煎寘鍚帴鍙ｇ姸鎬侊細

**ifinfomsg**
: if_flags & IFF_UP锛? 鎺ュ彛澶勪簬绠＄悊 up銆?
**ifinfomsg**
: if_flags & IFF_RUNNING锛? 鎺ュ彛澶勪簬 RFC2863 鎿嶄綔鐘舵€?UP 鎴?UNKNOWN銆傝繖鏄负浜嗗悜鍚庡吋瀹癸紝璺敱瀹堟姢杩涚▼銆? dhcp 瀹㈡埛绔彲鐢ㄦ鏍囧織鏉ュ垽鏂槸鍚﹀簲璇ヤ娇鐢ㄨ鎺ュ彛銆?
**ifinfomsg**
: if_flags & IFF_LOWER_UP锛? 椹卞姩宸插彂鍑?netif_carrier_on() 淇″彿銆?
**ifinfomsg**
: if_flags & IFF_DORMANT锛? 椹卞姩宸插彂鍑?netif_dormant_on() 淇″彿銆?
### TLV IFLA_OPERSTATE


鍖呭惈鎺ュ彛鐨?RFC2863 鐘舵€侊紝浠ユ暟鍊艰〃绀猴細

IF_OPER_UNKNOWN (0)锛? 鎺ュ彛澶勪簬鏈煡鐘舵€侊紝椹卞姩鍜岀敤鎴风┖闂撮兘鏈缃搷浣滅姸鎬併€傜敱浜庡苟闈炴瘡涓┍鍔ㄩ兘瀹炵幇浜? 鎿嶄綔鐘舵€佽缃紝鎺ュ彛鍦ㄨ€冭檻鐢ㄦ埛鏁版嵁鏃跺繀椤昏瑙嗕负鏈煡銆?
IF_OPER_NOTPRESENT (1)锛? 褰撳墠鍐呮牳涓湭浣跨敤锛坣otpresent 鎺ュ彛閫氬父浼氭秷澶憋級锛屼粎浣滄暟鍊煎崰浣嶃€?
IF_OPER_DOWN (2)锛? 鎺ュ彛鏃犳硶鍦?L1 涓婁紶杈撴暟鎹紝渚嬪浠ュお缃戞湭鎻掔嚎锛屾垨鎺ュ彛澶勪簬 ADMIN down銆?
IF_OPER_LOWERLAYERDOWN (3)锛? 鍫嗗彔鍦?IF_OPER_DOWN 鎺ュ彛涔嬩笂鐨勬帴鍙ｆ樉绀烘鐘舵€侊紙渚嬪 VLAN锛夈€?
IF_OPER_TESTING (4)锛? 鎺ュ彛澶勪簬娴嬭瘯妯″紡锛屼緥濡傛鍦ㄦ墽琛岄┍鍔ㄨ嚜妫€鎴栦粙璐紙绾跨紗锛夋祴璇曘€傚湪娴嬭瘯瀹屾垚涔嬪墠涓嶈兘
 鐢ㄤ簬姝ｅ父娴侀噺銆?
IF_OPER_DORMANT (5)锛? 鎺ュ彛 L1 宸?up锛屼絾鍦ㄧ瓑寰呬竴涓閮ㄤ簨浠讹紝渚嬪绛夊緟鏌愪釜鍗忚寤虹珛锛?02.1X锛夈€?
IF_OPER_UP (6)锛? 鎺ュ彛鎿嶄綔 up锛屽彲浠ヤ娇鐢ㄣ€?
姝?TLV 涔熷彲閫氳繃 sysfs 鏌ヨ銆?
### TLV IFLA_LINKMODE


鍖呭惈閾捐矾绛栫暐銆備笅闈㈡弿杩扮殑鐢ㄦ埛绌洪棿浜や簰闇€瑕佸畠銆?
姝?TLV 涔熷彲閫氳繃 sysfs 鏌ヨ銆?

## 3. 鍐呮牳椹卞姩 API


鍐呮牳椹卞姩鍙互璁块棶鏄犲皠鍒?IFF_LOWER_UP 鍜?IFF_DORMANT 鐨勪袱涓爣蹇椼€傝繖浜涙爣蹇楀彲浠ュ湪
浠讳綍鍦版柟璁剧疆锛岀敋鑷冲彲浠ュ湪涓柇涓缃€傝櫧鐒舵病鏈夊叾瀹冮儴鍒嗘嫢鏈夊啓鏉冮檺锛屼絾濡傛灉椹卞姩鐨勪笉鍚?灞傛搷浣滃悓涓€涓爣蹇楋紝椹卞姩蹇呴』鎻愪緵蹇呰鐨勫悓姝ャ€?
__LINK_STATE_NOCARRIER锛屾槧灏勫埌 !IFF_LOWER_UP锛?
椹卞姩浣跨敤 netif_carrier_on() 娓呴櫎璇ユ爣蹇楋紝浣跨敤 netif_carrier_off() 璁剧疆瀹冦€傚湪
netif_carrier_off() 鏃讹紝璋冨害鍣ㄥ仠姝㈠彂閫佸寘銆傚悕绉扳€渃arrier鈥濆強鍏跺彇鍙嶆槸鍘嗗彶鍘熷洜锛屽彲
灏嗗叾鐞嗚В涓轰笅灞傦紙lower layer锛夈€?
娉ㄦ剰锛屽浜庢煇浜涗笉绠＄悊浠讳綍鐪熷疄纭欢鐨勮蒋璁惧锛屽彲浠ヤ粠鐢ㄦ埛绌洪棿璁剧疆姝や綅銆傚簲浣跨敤 TLV
IFLA_CARRIER 鏉ヨ繖涔堝仛銆?
netif_carrier_ok() 鍙敤浜庢煡璇㈣浣嶃€?
__LINK_STATE_DORMANT锛屾槧灏勫埌 IFF_DORMANT锛?
鐢遍┍鍔ㄨ缃紝琛ㄧず璁惧鐢变簬鏌愪簺椹卞姩鎺у埗鐨勫崗璁缓绔嬪皻鏈畬鎴愯€屾殏鏃舵棤娉曚娇鐢ㄣ€傚搴旂殑鍑芥暟
鏄?netif_dormant_on() 璁剧疆璇ユ爣蹇楋紝netif_dormant_off() 娓呴櫎瀹冿紝netif_dormant()
鐢ㄤ簬鏌ヨ銆?
鍦ㄨ澶囧垎閰嶆椂锛宊_LINK_STATE_NOCARRIER 涓?__LINK_STATE_DORMANT 涓や釜鏍囧織閮借娓呴櫎锛?鍥犳鏈夋晥鐘舵€佺瓑鍚屼簬 netif_carrier_ok() 涓?!netif_dormant()銆?

姣忓綋椹卞姩鏇存敼杩欎袱涓爣蹇椾箣涓€鏃讹紝浼氳皟搴︿竴涓伐浣滈槦鍒椾簨浠讹紝灏嗘爣蹇楃粍鍚堣浆鎹负
IFLA_OPERSTATE锛屽涓嬫墍绀猴細

!netif_carrier_ok()锛? 鑻ユ帴鍙ｆ槸鍫嗗彔鐨勫垯涓?IF_OPER_LOWERLAYERDOWN锛屽惁鍒欎负 IF_OPER_DOWN銆傚唴鏍稿彲浠ヨ瘑鍒? 鍫嗗彔鎺ュ彛锛屽洜涓哄畠浠殑 ifindex != iflink銆?
netif_carrier_ok() && netif_dormant()锛? IF_OPER_DORMANT

netif_carrier_ok() && !netif_dormant()锛? 鑻ョ敤鎴风┖闂翠氦浜掕绂佺敤鍒欎负 IF_OPER_UP銆傚惁鍒欎负 IF_OPER_DORMANT锛屼箣鍚庣敤鎴风┖闂村彲浠? 鍙戣捣鍚?IF_OPER_UP 鐨勮浆鎹€?

## 4. 浠庣敤鎴风┖闂磋缃?

搴旂敤绋嬪簭蹇呴』浣跨敤 netlink 鎺ュ彛鏉ュ奖鍝嶆帴鍙ｇ殑 RFC2863 鎿嶄綔鐘舵€併€傞€氳繃 RTM_SETLINK 灏?IFLA_LINKMODE 璁句负 1 浼氭寚绀哄唴鏍革細褰撻┍鍔ㄨ缃?netif_carrier_ok() && !netif_dormant() 缁勫悎鏃讹紝鎺ュ彛搴旇繘鍏?IF_OPER_DORMANT 鑰岄潪
IF_OPER_UP銆備箣鍚庯紝鍙椹卞姩娌℃湁璁剧疆 netif_carrier_off() 鎴?netif_dormant_on()锛?鐢ㄦ埛绌洪棿搴旂敤绋嬪簭灏卞彲浠ュ皢 IFLA_OPERSTATE 璁句负 IF_OPER_DORMANT 鎴?IF_OPER_UP銆傜敤鎴?绌洪棿鎵€鍋氱殑鏇存敼浼氬湪 netlink 缁?RTNLGRP_LINK 涓婂箍鎾€?
鍥犳锛屼竴涓?802.1X 璇锋眰鏂癸紙supplicant锛変笌鍐呮牳鐨勪氦浜掑ぇ鑷村涓嬶細

- 璁㈤槄 RTNLGRP_LINK
- 閫氳繃 RTM_SETLINK 灏?IFLA_LINKMODE 璁句负 1
- 鏌ヨ涓€娆?RTM_GETLINK 浠ヨ幏鍙栧垵濮嬬姸鎬?- 濡傛灉鍒濆鏍囧織涓嶆槸 (IFF_LOWER_UP && !IFF_DORMANT)锛屽垯绛夊緟鐩村埌 netlink 澶氭挱鍙戝嚭
  姝ょ姸鎬佷俊鍙?- 鎵ц 802.1X锛屽鏋滄爣蹇楀啀娆″彉 down 鍒欎腑姝?- 濡傛灉璁よ瘉鎴愬姛锛屽彂閫?RTM_SETLINK 灏?operstate 璁句负 IF_OPER_UP锛屽惁鍒欒涓?  IF_OPER_DORMANT
- 瑙傚療 operstate 鍜?IFF_RUNNING 濡備綍閫氳繃 netlink 澶氭挱鍥炴樉
- 濡傛灉 802.1X 閲嶆柊璁よ瘉澶辫触锛屽皢鎺ュ彛璁惧洖 IF_OPER_DORMANT
- 濡傛灉鍐呮牳鏇存敼浜?IFF_LOWER_UP 鎴?IFF_DORMANT 鏍囧織锛屽垯閲嶆柊寮€濮?
濡傛灉璇锋眰鏂归€€鍑猴紝灏?IFLA_LINKMODE 鎭㈠涓?0锛屽苟灏?IFLA_OPERSTATE 鎭㈠涓轰竴涓悎鐞嗙殑鍊笺€?
璺敱瀹堟姢杩涚▼鎴?dhcp 瀹㈡埛绔彧闇€鍏虫敞 IFF_RUNNING锛屾垨鍦ㄨ€冭檻浣跨敤璇ユ帴鍙?/ 鏌ヨ DHCP
鍦板潃涔嬪墠锛岀瓑寰?operstate 鍙樹负 IF_OPER_UP/IF_OPER_UNKNOWN銆?

鎶€鏈棶棰樺強/鎴栨剰瑙佽鍙戦偖浠剁粰 Stefan Rompf锛坰tefan at loplof.de锛夈€?