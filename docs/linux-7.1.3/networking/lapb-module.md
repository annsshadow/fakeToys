
## Linux LAPB 妯″潡鎺ュ彛

鐗堟湰 1.3

Jonathan Naylor 29.12.96

鍙樻洿锛圚enner Eisen锛?000-10-29锛夛細data_indication() 鐨勮繑鍥炲€兼敼涓?int

LAPB 妯″潡灏嗘槸涓€涓崟鐙紪璇戠殑妯″潡锛屼緵 Linux 鎿嶄綔绯荤粺涓换浣曢渶瑕?LAPB 鏈嶅姟鐨勯儴鍒嗕娇鐢ㄣ€傛湰鏂囨。
瀹氫箟浜嗚妯″潡鐨勬帴鍙ｄ互鍙婂畠鎵€鎻愪緵鐨勬湇鍔°€傝繖閲岀殑鈥滄ā鍧椻€濅竴璇嶅苟涓嶆殫绀?LAPB 妯″潡鏄竴涓彲鍗曠嫭鍔犺浇鐨?妯″潡锛屽敖绠″畠鍙互鏄€傝鏈鏄湪鏇存爣鍑嗙殑鎰忎箟涓婁娇鐢ㄧ殑銆?
鍒?LAPB 妯″潡鐨勬帴鍙ｇ敱浠ヤ笅閮ㄥ垎缁勬垚锛氳皟鐢ㄦā鍧楃殑鍑芥暟銆佹ā鍧楀洖璋冧互鎸囩ず閲嶈鐨勭姸鎬佸彉鍖栵紝浠ュ強鐢ㄤ簬
鑾峰彇鍜岃缃ā鍧楃浉鍏充俊鎭殑鏁版嵁缁撴瀯銆?
### 缁撴瀯

鍙兘鏈€閲嶈鐨勭粨鏋勬槸鎸佹湁鎺ユ敹鍜屽彂閫佹暟鎹殑 skbuff 缁撴瀯锛屼絾杩欒秴鍑轰簡鏈枃妗ｇ殑鑼冨洿銆?
涓や釜 LAPB 鐗规湁鐨勭粨鏋勬槸 LAPB 鍒濆鍖栫粨鏋勫拰 LAPB 鍙傛暟缁撴瀯銆傚畠浠皢鍦ㄦ爣鍑嗗ご鏂囦欢 <linux/lapb.h>
涓畾涔夈€傚ご鏂囦欢 <net/lapb.h> 鏄?LAPB 妯″潡鍐呴儴浣跨敤鐨勶紝涓嶅彲浣跨敤銆?
### LAPB 鍒濆鍖栫粨鏋?
璇ョ粨鏋勫彧鍦ㄨ皟鐢?lapb_register锛堣涓嬶級鏃朵娇鐢ㄤ竴娆°€傚畠鍖呭惈鍏充簬闇€瑕佽鏈嶅姟鐨勮澶囬┍鍔ㄧ殑淇℃伅锛?
```
	struct lapb_register_struct {
		void (*connect_confirmation)(int token, int reason);
		void (*connect_indication)(int token, int reason);
		void (*disconnect_confirmation)(int token, int reason);
		void (*disconnect_indication)(int token, int reason);
		int  (*data_indication)(int token, struct sk_buff *skb);
		void (*data_transmit)(int token, struct sk_buff *skb);
	};

```
璇ョ粨鏋勭殑姣忎釜鎴愬憳閮藉搴旇澶囬┍鍔ㄤ腑鐨勪竴涓嚱鏁帮紝褰?LAPB 妯″潡涓彂鐢熺壒瀹氫簨浠舵椂浼氳璋冪敤銆傝繖浜涘皢鍦?涓嬮潰璇﹁堪銆傚鏋滀笉闇€瑕佹煇涓洖璋冿紙锛侊紒锛夛紝鍒欏彲浠ヤ紶鍏?NULL銆?
### LAPB 鍙傛暟缁撴瀯

璇ョ粨鏋勪笌 lapb_getparms 鍜?lapb_setparms 鍑芥暟锛堣涓嬶級涓€璧蜂娇鐢ㄣ€傚畠浠敤浜庡厑璁歌澶囬┍鍔ㄨ幏鍙栧拰璁剧疆锛?
```
	struct lapb_parms_struct {
		unsigned int t1;
		unsigned int t1timer;
		unsigned int t2;
		unsigned int t2timer;
		unsigned int n2;
		unsigned int n2count;
		unsigned int window;
		unsigned int state;
		unsigned int mode;
	};

```
T1 鍜?T2 鏄崗璁椂搴忓弬鏁帮紝鍗曚綅涓?100ms銆侼2 鏄湪閾捐矾琚鍛婂け璐ヤ箣鍓嶇殑鏈€澶ч噸璇曟鏁般€傜獥鍙ｅぇ灏忔槸
鍏佽杩滅鏈‘璁ょ殑鏈€澶у湪閫旀暟鎹寘鏁伴噺锛涘浜庢爣鍑?LAPB 閾捐矾锛岀獥鍙ｅ€煎湪 1 鍒?7 涔嬮棿锛屽浜庢墿灞?LAPB
閾捐矾锛屽湪 1 鍒?127 涔嬮棿銆?
mode 鍙橀噺鏄竴涓綅鍩燂紝鐢ㄤ簬璁剧疆锛堢洰鍓嶏級涓変釜鍊笺€傝繖浜涗綅鍩熺殑鍚箟濡備笅锛?
======  =================================================
Bit	鍚箟
======  =================================================
0	LAPB 鎿嶄綔锛?=LAPB_STANDARD 1=LAPB_EXTENDED锛夈€?1	[SM]LP 鎿嶄綔锛?=LAPB_SLP 1=LAPB=MLP锛夈€?2	DTE/DCE 鎿嶄綔锛?=LAPB_DTE 1=LAPB_DCE锛?3-31	淇濈暀锛屽繀椤讳负 0銆?======  =================================================

鎵╁睍 LAPB 鎿嶄綔琛ㄧず浣跨敤鎵╁睍搴忓垪鍙凤紝浠庤€屽厑璁告洿澶х殑绐楀彛澶у皬锛岄粯璁ゆ槸鏍囧噯 LAPB 鎿嶄綔銆侻LP 鎿嶄綔涓?SLP 鎿嶄綔鐩稿悓锛屽彧鏄?LAPB 浣跨敤鐨勫湴鍧€涓嶅悓浠ユ寚绀烘搷浣滄ā寮忥紝榛樿鏄崟閾捐矾杩囩▼锛圫ingle Link
Procedure锛夈€侱CE 涓?DTE 鎿嶄綔鐨勫尯鍒湪浜庯細(i) 鐢ㄤ簬鍛戒护鍜屽搷搴旂殑鍦板潃锛?ii) 褰?DCE 鏈繛鎺ユ椂锛屽畠
姣忛殧 T1 鍙戦€佷竴娆′笉甯﹁疆璇綅锛坧oll锛夌殑 DM銆傝繖浜涘ぇ鍐欏父閲忓悕灏嗗湪鍏叡 LAPB 澶存枃浠朵腑瀹氫箟銆?
### 鍑芥暟

LAPB 妯″潡鎻愪緵浜嗗涓嚱鏁板叆鍙ｇ偣銆?
```
    int lapb_register(void *token, struct lapb_register_struct);

```
杩欏繀椤诲湪 LAPB 妯″潡琚娇鐢ㄤ箣鍓嶈皟鐢ㄣ€傚鏋滆皟鐢ㄦ垚鍔燂紝鍒欒繑鍥?LAPB_OK銆倀oken 蹇呴』鏄澶囬┍鍔ㄧ敓鎴愮殑
鍞竴鏍囪瘑绗︼紝浠ヤ究鍞竴鏍囪瘑 LAPB 閾捐矾鐨勫疄渚嬨€傚畠鐢?LAPB 妯″潡鍦ㄦ墍鏈夊洖璋冧腑杩斿洖锛屽苟琚澶囬┍鍔ㄥ湪
鎵€鏈夊 LAPB 妯″潡鐨勮皟鐢ㄤ腑浣跨敤銆傚浜庡崟涓澶囬┍鍔ㄤ腑鐨勫涓?LAPB 閾捐矾锛屽繀椤昏繘琛屽娆?lapb_register
璋冪敤銆俵apb_register_struct 鐨勬牸寮忓涓婃墍杩般€傝繑鍥炲€间负锛?
=============		=============================
LAPB_OK			LAPB 娉ㄥ唽鎴愬姛銆?LAPB_BADTOKEN		token 宸茶娉ㄥ唽銆?LAPB_NOMEM		鍐呭瓨涓嶈冻
=============		=============================

```
    int lapb_unregister(void *token);

```
杩欎細閲婃斁涓?LAPB 閾捐矾鍏宠仈鐨勬墍鏈夎祫婧愩€備换浣曞綋鍓嶇殑 LAPB 閾捐矾閮藉皢琚斁寮冿紝涓嶅啀浼犻€掕繘涓€姝ョ殑娑堟伅銆?鍦ㄦ璋冪敤涔嬪悗锛宼oken 鐨勫€煎浜庝换浣曞 LAPB 鍑芥暟鐨勮皟鐢ㄩ兘涓嶅啀鏈夋晥銆傛湁鏁堢殑杩斿洖鍊间负锛?
=============		===============================
LAPB_OK			LAPB 娉ㄩ攢鎴愬姛銆?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?=============		===============================

```
    int lapb_getparms(void *token, struct lapb_parms_struct *parms);

```
杩欏厑璁歌澶囬┍鍔ㄨ幏鍙栧綋鍓?LAPB 鍙橀噺鐨勫€硷紝lapb_parms_struct 濡備笂鎵€杩般€傛湁鏁堢殑杩斿洖鍊间负锛?
=============		=============================
LAPB_OK			LAPB getparms 鎴愬姛銆?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?=============		=============================

```
    int lapb_setparms(void *token, struct lapb_parms_struct *parms);

```
杩欏厑璁歌澶囬┍鍔ㄨ缃綋鍓?LAPB 鍙橀噺鐨勫€硷紝lapb_parms_struct 濡備笂鎵€杩般€倀1timer銆乼2timer 鍜?n2count
鐨勫€间細琚拷鐣ワ紝鍚屾牱锛屽湪宸茶繛鎺ユ椂鏇存敼 mode 浣嶄篃浼氳蹇界暐銆傚嚭閿欐剰鍛崇潃娌℃湁浠讳綍鍊艰鏀瑰彉銆傛湁鏁堢殑
杩斿洖鍊间负锛?
=============		=================================================
LAPB_OK			LAPB getparms 鎴愬姛銆?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?LAPB_INVALUE		鏌愪釜鍊艰秴鍑轰簡鍏跺厑璁哥殑鑼冨洿銆?=============		=================================================

```
    int lapb_connect_request(void *token);

```
浣跨敤褰撳墠鍙傛暟璁剧疆鍙戣捣杩炴帴銆傛湁鏁堢殑杩斿洖鍊间负锛?
==============		=================================
LAPB_OK			LAPB 姝ｅ湪寮€濮嬭繛鎺ャ€?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?LAPB_CONNECTED		LAPB 妯″潡宸茶繛鎺ャ€?==============		=================================

```
    int lapb_disconnect_request(void *token);

```
鍙戣捣鏂紑杩炴帴銆傛湁鏁堢殑杩斿洖鍊间负锛?
=================	===============================
LAPB_OK			LAPB 姝ｅ湪寮€濮嬫柇寮€杩炴帴銆?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?LAPB_NOTCONNECTED	LAPB 妯″潡鏈繛鎺ャ€?=================	===============================

```
    int lapb_data_request(void *token, struct sk_buff *skb);

```
灏嗘暟鎹帓闃熷埌 LAPB 妯″潡锛屼互渚块€氳繃閾捐矾鍙戦€併€傚鏋滆皟鐢ㄦ垚鍔燂紝鍒?skbuff 褰?LAPB 妯″潡鎵€鏈夛紝璁惧椹卞姩
涓嶅緱鍐嶆浣跨敤瀹冦€傛湁鏁堢殑杩斿洖鍊间负锛?
=================	=============================
LAPB_OK			LAPB 宸叉帴鍙楁暟鎹€?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?LAPB_NOTCONNECTED	LAPB 妯″潡鏈繛鎺ャ€?=================	=============================

```
    int lapb_data_received(void *token, struct sk_buff *skb);

```
灏嗕粠璁惧鎺ユ敹鍒扮殑鏁版嵁鎺掗槦鍒?LAPB 妯″潡銆傛湡鏈涗紶閫掔粰 LAPB 妯″潡鐨勬暟鎹殑 skb->data 鎸囧悜 LAPB 鏁版嵁鐨?寮€澶淬€傚鏋滆皟鐢ㄦ垚鍔燂紝鍒?skbuff 褰?LAPB 妯″潡鎵€鏈夛紝璁惧椹卞姩涓嶅緱鍐嶆浣跨敤瀹冦€傛湁鏁堢殑杩斿洖鍊间负锛?
=============		===========================
LAPB_OK			LAPB 宸叉帴鍙楁暟鎹€?LAPB_BADTOKEN		鏃犳晥/鏈煡鐨?LAPB token銆?=============		===========================

### 鍥炶皟

杩欎簺鍥炶皟鏄澶囬┍鍔ㄦ彁渚涚粰 LAPB 妯″潡銆佸湪鍙戠敓浜嬩欢鏃惰皟鐢ㄧ殑鍑芥暟銆傚畠浠€氳繃 lapb_register锛堣涓婏級鍦?缁撴瀯 lapb_register_struct锛堣涓婏級涓悜 LAPB 妯″潡娉ㄥ唽銆?
```
    void (*connect_confirmation)(void *token, int reason);

```
褰撳湪璋冪敤 lapb_connect_request锛堣涓婏級璇锋眰涔嬪悗杩炴帴寤虹珛鏃讹紝鐢?LAPB 妯″潡璋冪敤銆俽eason 鎬绘槸
LAPB_OK銆?
```
    void (*connect_indication)(void *token, int reason);

```
褰撻摼璺敱杩滅▼绯荤粺寤虹珛鏃讹紝鐢?LAPB 妯″潡璋冪敤銆俽eason 鐨勫€兼€绘槸 LAPB_OK銆?
```
    void (*disconnect_confirmation)(void *token, int reason);

```
褰撹澶囬┍鍔ㄨ皟鐢?lapb_disconnect_request锛堣涓婏級涔嬪悗鍙戠敓浜嬩欢鏃讹紝鐢?LAPB 妯″潡璋冪敤銆俽eason 鎸囩ず
鍙戠敓浜嗕粈涔堛€傚湪鎵€鏈夋儏鍐典笅锛孡APB 閾捐矾閮藉彲瑙嗕负宸茬粓姝€俽eason 鐨勫彇鍊间负锛?
=================	====================================================
LAPB_OK			LAPB 閾捐矾姝ｅ父缁堟銆?LAPB_NOTCONNECTED	杩滅▼绯荤粺鏈繛鎺ャ€?LAPB_TIMEDOUT		鍦?N2 娆″皾璇曚腑閮芥湭鏀跺埌杩滅▼绯荤粺鐨勫搷搴斻€?=================	====================================================

```
    void (*disconnect_indication)(void *token, int reason);

```
褰撻摼璺杩滅▼绯荤粺缁堟鎴栧彂鐢熷叾浠栦簨浠跺鑷撮摼璺粓姝㈡椂锛岀敱 LAPB 妯″潡璋冪敤銆傚鏋滆繙绋嬬郴缁熸嫆缁濅簡璇锋眰锛?杩欎篃鍙兘浣滀负瀵?lapb_connect_request锛堣涓婏級鐨勫搷搴旇€岃繑鍥炪€俽eason 鐨勫彇鍊间负锛?
=================	====================================================
LAPB_OK			LAPB 閾捐矾琚繙绋嬬郴缁熸甯哥粓姝€?LAPB_REFUSED		杩滅▼绯荤粺鎷掔粷浜嗚繛鎺ヨ姹傘€?LAPB_NOTCONNECTED	杩滅▼绯荤粺鏈繛鎺ャ€?LAPB_TIMEDOUT		鍦?N2 娆″皾璇曚腑閮芥湭鏀跺埌杩滅▼绯荤粺鐨勫搷搴斻€?=================	====================================================

```
    int (*data_indication)(void *token, struct sk_buff *skb);

```
褰撲粠杩滅▼绯荤粺鎺ユ敹鍒板簲浼犻€掔粰鍗忚鏍堜笅涓€灞傜殑鏁版嵁鏃讹紝鐢?LAPB 妯″潡璋冪敤銆俿kbuff 鎴愪负璁惧椹卞姩鐨勮储浜э紝
LAPB 妯″潡涓嶄細鍐嶅瀹冩墽琛屼换浣曟搷浣溿€俿kb->data 鎸囬拡灏嗘寚鍚?LAPB 澶撮儴涔嬪悗鐨勭涓€涓暟鎹瓧鑺傘€?
褰撲笖浠呭綋璇ュ抚鍦ㄤ氦浠樼粰涓婂眰涔嬪墠琚涪寮冩椂锛岃鏂规硶搴旇繑鍥?NET_RX_DROP锛堝畾涔変簬澶存枃浠?include/linux/netdevice.h锛夈€?
```
    void (*data_transmit)(void *token, struct sk_buff *skb);

```
褰撴暟鎹鐢辫澶囬┍鍔ㄥ彂閫佸埌杩滅▼绯荤粺鏃讹紝鐢?LAPB 妯″潡璋冪敤銆俿kbuff 鎴愪负璁惧椹卞姩鐨勮储浜э紝LAPB 妯″潡
涓嶄細鍐嶅瀹冩墽琛屼换浣曟搷浣溿€俿kb->data 鎸囬拡灏嗘寚鍚?LAPB 澶撮儴鐨勭涓€涓瓧鑺傘€?