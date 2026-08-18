## SocketCAN - 鎺у埗鍣ㄥ眬鍩熺綉锛圕ontroller Area Network锛?


## 姒傝堪 / 浠€涔堟槸 SocketCAN


socketcan 鍖呮槸 Linux 涓?CAN 鍗忚锛圕ontroller Area Network锛夌殑涓€绉嶅疄鐜般€侰AN 鏄竴椤瑰湪缃戠粶鍖栨妧鏈紝鍦ㄨ嚜鍔ㄥ寲銆佸祵鍏ュ紡璁惧鍜屾苯杞﹂鍩熸湁骞挎硾搴旂敤銆傝櫧鐒舵鍓嶅凡鏈夊熀浜庡瓧绗﹁澶囩殑鍏朵粬 CAN 瀹炵幇锛屼絾 SocketCAN 浣跨敤浜?Berkeley 濂楁帴瀛?API銆丩inux 缃戠粶鏍堬紝骞跺皢 CAN 璁惧椹卞姩瀹炵幇涓虹綉缁滄帴鍙ｃ€侰AN 濂楁帴瀛?API 鐨勮璁″敖鍙兘绫讳技浜?TCP/IP 鍗忚锛屼互渚跨啛鎮夌綉缁滅紪绋嬬殑绋嬪簭鍛樿兘澶熻交鏉惧涔犲浣曚娇鐢?CAN 濂楁帴瀛椼€?


## 鍔ㄦ満 / 涓轰粈涔堜娇鐢ㄥ鎺ュ瓧 API


鍦?SocketCAN 涔嬪墠锛孡inux 涓婂凡缁忔湁杩?CAN 瀹炵幇锛屽洜姝ら棶棰樻潵浜嗭細鎴戜滑涓轰粈涔堣鍚姩鍙︿竴涓」鐩€傚ぇ澶氭暟鐜版湁鐨勫疄鐜版槸浣滀负鏌愪簺 CAN 纭欢鐨勮澶囬┍鍔ㄥ嚭鐜扮殑锛屽畠浠熀浜庡瓧绗﹁澶囷紝骞朵笖鎻愪緵鐩稿杈冨皯鐨勫姛鑳姐€傞€氬父锛屽彧鏈変竴涓壒瀹氫簬纭欢鐨勮澶囬┍鍔紝鎻愪緵瀛楃璁惧鎺ュ彛鏉ョ洿鎺ュ悜/浠庢帶鍒跺櫒纭欢鍙戦€佸拰鎺ユ敹鍘熷 CAN 甯с€傚抚鐨勬帓闃熶互鍙?ISO-TP 绛夐珮灞備紶杈撳崗璁繀椤诲湪鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涓疄鐜般€傛澶栵紝澶у鏁板瓧绗﹁澶囧疄鐜板彧鏀寔涓€涓繘绋嬪湪鏌愪竴鏃跺埢鎵撳紑璁惧锛岀被浼间簬涓茶鎺ュ彛銆傛洿鎹?CAN 鎺у埗鍣ㄩ渶瑕佹崲鐢ㄥ彟涓€涓澶囬┍鍔紝骞朵笖甯稿父闇€瑕佽搴旂敤绋嬪簭鐨勫緢澶ч儴鍒嗗幓閫傞厤鏂伴┍鍔ㄧ殑 API銆?

SocketCAN 鐨勮璁″氨鏄负浜嗗厠鏈嶆墍鏈夎繖浜涢檺鍒躲€傛垜浠疄鐜颁簡涓€涓柊鐨勫崗璁棌锛屽畠涓虹敤鎴风┖闂村簲鐢ㄧ▼搴忔彁渚涘鎺ュ瓧鎺ュ彛锛屽苟鏋勫缓鍦?Linux 缃戠粶灞備箣涓婏紝浠庤€岃兘澶熶娇鐢ㄦ墍鎻愪緵鐨勫叏閮ㄦ帓闃熷姛鑳姐€侰AN 鎺у埗鍣ㄧ‖浠剁殑璁惧椹卞姩浣滀负缃戠粶璁惧鍚?Linux 缃戠粶灞傛敞鍐岋紝杩欐牱鏉ヨ嚜鎺у埗鍣ㄧ殑 CAN 甯у氨鍙互琚笂浼犲埌缃戠粶灞傦紝鍐嶄紶閫佸埌 CAN 鍗忚鏃忔ā鍧楋紝鍙嶄箣浜︾劧銆傛澶栵紝鍗忚鏃忔ā鍧椾负浼犺緭鍗忚妯″潡鎻愪緵娉ㄥ唽 API锛屼粠鑰屽彲浠ュ姩鎬佸湴鍔犺浇鎴栧嵏杞戒换鎰忔暟閲忕殑浼犺緭鍗忚銆傚疄闄呬笂锛屽崟鐙殑 can 鏍稿績妯″潡涓嶆彁渚涗换浣曞崗璁紝骞朵笖濡傛灉涓嶅姞杞借嚦灏戜竴涓澶栫殑鍗忚妯″潡灏辨棤娉曚娇鐢ㄣ€傚彲浠ュ悓鏃舵墦寮€澶氫釜濂楁帴瀛楋紝鍦ㄤ笉鍚岀殑鎴栫浉鍚岀殑鍗忚妯″潡涓婏紝瀹冧滑鍙互鍦ㄤ笉鍚岀殑鎴栫浉鍚岀殑 CAN ID 涓婄洃鍚?鍙戦€佸抚銆傚涓鎺ュ瓧鍦ㄥ悓涓€鎺ュ彛涓婄洃鍚叿鏈夌浉鍚?CAN ID 鐨勫抚鏃讹紝閮戒細琚紶鍏ョ浉鍚岀殑鍖归厤 CAN 甯с€傚笇鏈涗娇鐢ㄧ壒瀹氫紶杈撳崗璁紙渚嬪 ISO-TP锛夎繘琛岄€氫俊鐨勫簲鐢ㄧ▼搴忥紝鍙渶鍦ㄦ墦寮€濂楁帴瀛楁椂閫夋嫨璇ュ崗璁紝鐒跺悗灏卞彲浠ヨ鍐欏簲鐢ㄧ▼搴忔暟鎹祦锛岃€屼笉蹇呭鐞?CAN-ID銆佸抚绛夈€?

绫讳技鐨勩€佷粠鐢ㄦ埛绌洪棿鍙鐨勫姛鑳戒篃鍙互鐢卞瓧绗﹁澶囨彁渚涳紝浣嗚繖浼氬洜鍑犱釜鍘熷洜瀵艰嚧鎶€鏈笂涓嶄紭闆呯殑瑙ｅ喅鏂规锛?

- **浣跨敤澶嶆潅锛?* 搴旂敤绋嬪簭涓嶅繀鍚?socket(2) 浼犻€掑崗璁弬鏁板苟浣跨敤 bind(2) 閫夋嫨 CAN 鎺ュ彛鍜?CAN ID锛岃€屾槸蹇呴』浣跨敤 ioctl(2) 鏉ュ畬鎴愭墍鏈夎繖浜涙搷浣溿€?

- **浠ｇ爜閲嶅锛?* 瀛楃璁惧鏃犳硶鍒╃敤 Linux 鐨勭綉缁滄帓闃熶唬鐮侊紝鍥犳鎵€鏈夐偅浜涗唬鐮侀兘蹇呴』涓?CAN 缃戠粶閲嶅瀹炵幇銆?

- **鎶借薄锛?* 鍦ㄥぇ澶氭暟鐜版湁鐨勫瓧绗﹁澶囧疄鐜颁腑锛孋AN 鎺у埗鍣ㄧ殑鐗瑰畾浜庣‖浠剁殑璁惧椹卞姩鐩存帴涓哄簲鐢ㄧ▼搴忔彁渚涘瓧绗﹁澶囥€傝嚦灏戝湪 Unix 绯荤粺涓紝瀵逛簬瀛楃璁惧鍜屽潡璁惧鏉ヨ锛岃繖閮芥槸闈炲父涓嶅甯哥殑銆備緥濡傦紝浣犱笉浼氫负涓茶鎺ュ彛鐨勬煇涓壒瀹?UART銆佽绠楁満涓殑鏌愪釜鐗瑰畾澹板崱銆佹垨鎻愪緵瀵逛綘纭洏鎴栫甯︽祦璁惧璁块棶鐨?SCSI 鎴?IDE 鎺у埗鍣ㄦ彁渚涘瓧绗﹁澶囥€傜浉鍙嶏紝浣犳湁鎶借薄灞傦紝涓€鏂归潰涓哄簲鐢ㄧ▼搴忔彁渚涚粺涓€鐨勫瓧绗︽垨鍧楄澶囨帴鍙ｏ紝鍙︿竴鏂归潰涓虹壒瀹氫簬纭欢鐨勮澶囬┍鍔ㄦ彁渚涙帴鍙ｃ€傝繖浜涙娊璞＄敱璇稿 tty 灞傘€侀煶棰戝瓙绯荤粺鎴栦笂杩拌澶囩殑 SCSI 鍜?IDE 瀛愮郴缁熺瓑瀛愮郴缁熸彁渚涖€?

  瀹炵幇 CAN 璁惧椹卞姩鏈€绠€鍗曠殑鏂瑰紡鏄綔涓轰笉甯﹁繖绉嶏紙瀹屾暣锛夋娊璞″眰鐨勫瓧绗﹁澶囷紝灏卞儚澶у鏁扮幇鏈夐┍鍔ㄦ墍鍋氱殑閭ｆ牱銆傜劧鑰岋紝姝ｇ‘鐨勫仛娉曟槸娣诲姞杩欐牱涓€涓眰锛屾彁渚涜濡備负鐗瑰畾 CAN ID 娉ㄥ唽銆佹敮鎸佸涓墦寮€鐨勬枃浠舵弿杩扮浠ュ強瀹冧滑涔嬮棿 CAN 甯х殑锛堣В锛夊鐢ㄣ€侊紙澶嶆潅鐨勶級CAN 甯ф帓闃燂紝浠ュ強涓鸿澶囬┍鍔ㄦ彁渚涙敞鍐?API 绛夊叏閮ㄥ姛鑳姐€備絾鏄紝杩欐牱涓€鏉ワ紝浣跨敤 Linux 鍐呮牳鎻愪緵鐨勭綉缁滄鏋跺氨涓嶅啀鏇村洶闅撅紝鐢氳嚦鍙兘鏇村鏄擄紝鑰岃繖姝ｆ槸 SocketCAN 鎵€鍋氱殑銆?

浣跨敤 Linux 鍐呮牳鐨勭綉缁滄鏋跺彧鏄负 Linux 瀹炵幇 CAN 鏈€鑷劧銆佹渶鍚堥€傜殑鏂瑰紡銆?


## SocketCAN 姒傚康


濡?socketcan-motivation 鎵€杩帮紝SocketCAN 鐨勪富瑕佺洰鏍囨槸鎻愪緵涓€涓瀯寤哄湪 Linux 缃戠粶灞備箣涓婄殑銆侀潰鍚戠敤鎴风┖闂村簲鐢ㄧ▼搴忕殑濂楁帴瀛楁帴鍙ｃ€備笌浼楁墍鍛ㄧ煡鐨?TCP/IP 鍜屼互澶綉缃戠粶鐩告瘮锛孋AN 鎬荤嚎鏄竴涓粎骞挎挱锛?锛夌殑浠嬭川锛屽畠娌℃湁鍍忎互澶綉閭ｆ牱鐨?MAC 灞傚鍧€銆侰AN 鏍囪瘑绗︼紙can_id锛夌敤浜庡湪 CAN 鎬荤嚎涓婅繘琛屼徊瑁併€傚洜姝?CAN-ID 蹇呴』鍦ㄦ€荤嚎涓婂敮涓€閫夋嫨銆傚湪璁捐 CAN-ECU 缃戠粶鏃讹紝CAN-ID 琚槧灏勪负鐢辩壒瀹氱殑 ECU 鍙戦€併€傚洜姝わ紝CAN-ID 鏈€濂借瑙嗕负涓€绉嶆簮鍦板潃銆?


### 鎺ユ敹鍒楄〃


澶氫釜搴旂敤绋嬪簭鐨勭綉缁滈€忔槑璁块棶瀵艰嚧杩欐牱涓€涓棶棰橈細涓嶅悓鐨勫簲鐢ㄧ▼搴忓彲鑳藉鏉ヨ嚜鍚屼竴 CAN 缃戠粶鎺ュ彛鐨勭浉鍚?CAN-ID 鎰熷叴瓒ｃ€係ocketCAN 鏍稿績妯″潡鈥斺€斿畠瀹炵幇鍗忚鏃?CAN鈥斺€斾负姝ゆ彁渚涗簡鍑犱釜楂樻晥鐨勬帴鏀跺垪琛ㄣ€備緥濡傦紝濡傛灉鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鎵撳紑涓€涓?CAN RAW 濂楁帴瀛楋紝raw 鍗忚妯″潡鏈韩浼氬悜 SocketCAN 鏍稿績璇锋眰鐢ㄦ埛鎵€璇锋眰鐨?CAN-ID锛堣寖鍥达級銆侰AN-ID 鐨勮闃呭拰閫€璁㈠彲浠ラ拡瀵圭壒瀹氱殑 CAN 鎺ュ彛鎴栭拡瀵规墍鏈夛紙!锛夊凡鐭ョ殑 CAN 鎺ュ彛锛屼娇鐢?SocketCAN 鏍稿績鎻愪緵缁?CAN 鍗忚妯″潡鐨?can_rx_(un)register() 鍑芥暟锛堣 socketcan-core-module锛夈€備负浜嗗湪杩愯鏃朵紭鍖?CPU 浣跨敤鐜囷紝鎺ユ敹鍒楄〃琚媶鍒嗕负姣忎釜璁惧鐨勮嫢骞蹭釜鐗瑰畾鍒楄〃锛屼互鍖归厤缁欏畾鐢ㄤ緥鎵€璇锋眰鐨勮繃婊ゅ鏉傚害銆?


### 宸插彂閫佸抚鐨勬湰鍦板洖鐜?


姝ｅ鍏朵粬缃戠粶姒傚康鎵€鐭ワ紝浜ゆ崲鏁版嵁鐨勫簲鐢ㄧ▼搴忓彲浠ヨ繍琛屽湪鐩稿悓鎴栦笉鍚岀殑鑺傜偣涓婅€屾棤闇€浠讳綍鏀瑰彉锛堥櫎浜嗙浉搴旂殑瀵诲潃淇℃伅锛夛細


	 ___   ___   ___                   _______   ___
	| _ | | _ | | _ |                 | _   _ | | _ |
	||A|| ||B|| ||C||                 ||A| |B|| ||C||
	|___| |___| |___|                 |_______| |___|
	  |     |     |                       |       |
	-----------------(1)- CAN bus -(2)---------------

涓轰簡纭繚鍦ㄧず渚?(2) 涓簲鐢ㄧ▼搴?A 鎺ユ敹鍒扮殑淇℃伅涓庡湪绀轰緥 (1) 涓帴鏀跺埌鐨勪俊鎭浉鍚岋紝闇€瑕佸湪鐩稿簲鑺傜偣涓婂宸茬粡鍙戦€佺殑 CAN 甯ц繘琛屾煇绉嶆湰鍦板洖鐜€?

Linux 缃戠粶璁惧锛堥粯璁ゆ儏鍐典笅锛夊彧鑳藉鐞嗕緷璧栦簬浠嬭川鐨勫抚鐨勬敹鍙戙€傜敱浜?CAN 鎬荤嚎涓婄殑浠茶锛屼綆浼樺厛绾х殑 CAN-ID 鐨勫彂閫佸彲鑳戒細琚珮浼樺厛绾?CAN 甯х殑鎺ユ敹鎵€寤惰繜銆備负浜嗗弽鏄犺妭鐐逛笂姝ｇ‘鐨?[#f1]_ 娴侀噺锛屽凡鍙戦€佹暟鎹殑鍥炵幆蹇呴』鍦ㄦ垚鍔熷彂閫佷箣鍚庣珛鍗虫墽琛屻€傚鏋?CAN 缃戠粶鎺ュ彛鐢变簬鏌愮鍘熷洜鏃犳硶鎵ц鍥炵幆锛孲ocketCAN 鏍稿績鍙互浣滀负鍥為€€鏂规鎵ц姝や换鍔°€傝瑙?socketcan-local-loopback2锛堟帹鑽愶級銆?

鍥炵幆鍔熻兘榛樿鍚敤锛屼互鍙嶆槧 CAN 搴旂敤绋嬪簭鐨勬爣鍑嗙綉缁滆涓恒€傜敱浜?RT-SocketCAN 灏忕粍鐨勪竴浜涜姹傦紝鍥炵幆鍙€夊湴鍙互閽堝姣忎釜鍗曠嫭鐨勫鎺ュ瓧绂佺敤銆傚弬瑙?socketcan-raw-sockets 涓?CAN RAW 濂楁帴瀛楃殑 sockopts銆?


       浠ュ強锛堝悓涓€锛夎妭鐐逛笂鍍?'candump' 鎴?'cansniffer' 杩欐牱鐨勫伐鍏枫€?


### 缃戠粶闂閫氱煡


CAN 鎬荤嚎鐨勪娇鐢ㄥ彲鑳戒細鍦ㄧ墿鐞嗗眰涓庝粙璐ㄨ闂帶鍒跺眰涓婂鑷磋嫢骞查棶棰樸€傛娴嬪拰璁板綍杩欎簺搴曞眰闂瀵逛簬 CAN 鐢ㄦ埛璇嗗埆鐗╃悊鏀跺彂鍣ㄥ眰涓婄殑纭欢闂浠ュ強鐢变笉鍚?ECU 寮曡捣鐨勪徊瑁侀棶棰樺拰閿欒甯ф槸鑷冲叧閲嶈鐨勮姹傘€傛墍妫€娴嬪埌閿欒鐨勫嚭鐜板浜庤瘖鏂緢閲嶈锛屽繀椤讳笌绮剧‘鐨勬椂闂存埑涓€璧疯褰曘€備负姝わ紝CAN 鎺ュ彛椹卞姩鍙互鐢熸垚鎵€璋撶殑閿欒娑堟伅甯э紙Error Message Frames锛夛紝瀹冨彲浠ヤ笌鍒殑 CAN 甯т竴鏍峰彲閫夊湴浼犻€掔粰鐢ㄦ埛搴旂敤绋嬪簭銆傛瘡褰撳湪鐗╃悊灞傛垨 MAC 灞傛娴嬪埌閿欒锛堜緥濡傜敱 CAN 鎺у埗鍣ㄦ娴嬪埌锛夋椂锛岄┍鍔ㄥ氨浼氬垱寤虹浉搴旂殑閿欒娑堟伅甯с€傞敊璇秷鎭抚鍙互閫氳繃甯哥敤鐨?CAN 杩囨护鏈哄埗鐢辩敤鎴峰簲鐢ㄧ▼搴忚姹傘€傚湪杩欎釜杩囨护瀹氫箟涓紝鍙互閫夋嫨锛堟劅鍏磋叮鐨勶級閿欒绫诲瀷銆傞敊璇秷鎭殑鎺ユ敹榛樿鏄鐢ㄧ殑銆侰AN 閿欒娑堟伅甯х殑鏍煎紡鍦?Linux 澶存枃浠?"include/uapi/linux/can/error.h" 涓湁绠€瑕佹弿杩般€?


## 濡備綍浣跨敤 SocketCAN


涓?TCP/IP 涓€鏍凤紝浣犻鍏堥渶瑕佹墦寮€涓€涓鎺ュ瓧浠ラ€氳繃 CAN 缃戠粶杩涜閫氫俊銆傜敱浜?SocketCAN 瀹炵幇浜嗘柊鐨勫崗璁棌锛屼綘闇€瑕佸皢 PF_CAN 浣滀负绗竴涓弬鏁颁紶閫掔粰 socket(2) 绯荤粺璋冪敤銆傜洰鍓嶆湁涓ょ CAN 鍗忚鍙緵閫夋嫨锛歳aw 濂楁帴瀛楀崗璁拰骞挎挱绠＄悊鍣紙BCM锛夈€傚洜姝わ紝瑕佹墦寮€涓€涓鎺ュ瓧锛?
```

    s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

```
```

    s = socket(PF_CAN, SOCK_DGRAM, CAN_BCM);

```
鍒嗗埆鍦般€傚湪鎴愬姛鍒涘缓濂楁帴瀛椾箣鍚庯紝浣犻€氬父浼氫娇鐢?bind(2) 绯荤粺璋冪敤灏嗗鎺ュ瓧缁戝畾鍒?CAN 鎺ュ彛锛堢敱浜庡鍧€鏂瑰紡涓嶅悓锛岃繖涓?TCP/IP 涓嶅悓鈥斺€旇 socketcan-concept锛夈€傚湪缁戝畾锛圕AN_RAW锛夋垨杩炴帴锛圕AN_BCM锛夊鎺ュ瓧涔嬪悗锛屼綘鍙互鍍忓線甯镐竴鏍峰濂楁帴瀛楄繘琛?read(2) 鍜?write(2)锛屾垨浣跨敤 send(2)銆乻endto(2)銆乻endmsg(2) 鍙婂叾 recv* 瀵瑰簲鎿嶄綔銆備笅闈㈣繕鎻忚堪浜?CAN 鐗瑰畾鐨勫鎺ュ瓧閫夐」銆?

缁忓吀 CAN 甯х粨鏋勶紙鍗?CAN 2.0B锛夈€丆AN FD 甯х粨鏋勫拰 sockaddr 缁撴瀯瀹氫箟鍦?include/linux/can.h 涓細


    struct can_frame {
            canid_t can_id;  /** 32 bit CAN_ID + EFF/RTR/ERR flags **/
            union {
                    /* CAN frame payload length in byte (0 .. CAN_MAX_DLEN)
                     - was previously named can_dlc so we need to carry that
                     - name for legacy support
                     */
                    __u8 len;
                    __u8 can_dlc; /** deprecated **/
            };
            __u8    __pad;   /** padding **/
            __u8    __res0;  /** reserved / padding **/
            __u8    len8_dlc; /** optional DLC for 8 byte payload length (9 .. 15) **/
            __u8    data[^8^] __attribute__((aligned(8)));
    };

澶囨敞锛歭en 鍏冪礌鍖呭惈杞借嵎闀垮害锛堝瓧鑺傦級锛屽簲褰撲娇鐢ㄥ畠鑰岄潪 can_dlc銆傚凡搴熷純鐨?can_dlc 鍛藉悕鍏锋湁璇鎬э紝鍥犱负瀹冩€绘槸鍖呭惈鏅€氱殑杞借嵎闀垮害锛堝瓧鑺傦級锛岃€屼笉鏄墍璋撶殑鈥滄暟鎹暱搴︿唬鐮佲€濓紙DLC锛夈€?

涓轰簡浠?鍚戠粡鍏?CAN 缃戠粶璁惧浼犻€掑師濮?DLC锛屽綋 len 鍏冪礌涓?8锛堟墍鏈夊ぇ浜庣瓑浜?8 鐨?DLC 鍊兼墍瀵瑰簲鐨勭湡瀹炶浇鑽烽暱搴︼級鏃讹紝len8_dlc 鍏冪礌鍙互鍖呭惈 9 鍒?15 鐨勫€笺€?

锛堢嚎鎬э級杞借嵎 data[] 鍚?64 浣嶈竟鐣岀殑瀵归綈鍏佽鐢ㄦ埛瀹氫箟鑷繁鐨勭粨鏋勪綋鍜岃仈鍚堜綋鏉ユ柟渚垮湴璁块棶 CAN 杞借嵎銆侰AN 鎬荤嚎涓婇粯璁ゆ病鏈変换浣曠粰瀹氱殑瀛楄妭搴忋€傚 CAN_RAW 濂楁帴瀛楃殑 read(2) 绯荤粺璋冪敤浼氬皢涓€涓?struct can_frame 浼犻€佸埌鐢ㄦ埛绌洪棿銆?

sockaddr_can 缁撴瀯鏈変竴涓儚 PF_PACKET 濂楁帴瀛楅偅鏍风殑鎺ュ彛绱㈠紩锛屽畠涔熺粦瀹氬埌鐗瑰畾鎺ュ彛锛?


    struct sockaddr_can {
            sa_family_t can_family;
            int         can_ifindex;
            union {
                    /** transport protocol class address info (e.g. ISOTP) **/
                    struct { canid_t rx_id, tx_id; } tp;

                    /** J1939 address information **/
                    struct {
                            /** 8 byte name when using dynamic addressing **/
                            __u64 name;

                            /* pgn:
                             - 8 bit: PS in PDU2 case, else 0
                             - 8 bit: PF
                             - 1 bit: DP
                             - 1 bit: reserved
                             */
                            __u32 pgn;

                            /** 1 byte address **/
                            __u8 addr;
                    } j1939;

                    /** reserved for future CAN protocols address information **/
            } can_addr;
    };

涓轰簡纭畾鎺ュ彛绱㈠紩锛屽繀椤讳娇鐢ㄤ竴涓€傚綋鐨?ioctl()锛堜互 CAN_RAW 濂楁帴瀛椾负渚嬶紝鏈仛閿欒妫€鏌ワ級锛?


    int s;
    struct sockaddr_can addr;
    struct ifreq ifr;

    s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

    strcpy(ifr.ifr_name, "can0" );
    ioctl(s, SIOCGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    bind(s, (struct sockaddr *)&addr, sizeof(addr));

    (..)

瑕佸皢濂楁帴瀛楃粦瀹氬埌鎵€鏈夛紙!锛塁AN 鎺ュ彛锛屾帴鍙ｇ储寮曞繀椤讳负 0锛堥浂锛夈€傚湪杩欑鎯呭喌涓嬶紝濂楁帴瀛椾粠姣忎釜宸插惎鐢ㄧ殑 CAN 鎺ュ彛鎺ユ敹 CAN 甯с€傝纭畾婧?CAN 鎺ュ彛锛屽彲浠ヤ娇鐢ㄧ郴缁熻皟鐢?recvfrom(2) 鑰岄潪 read(2)銆傝鍦ㄧ粦瀹氬埌 'any' 鎺ュ彛鐨勫鎺ュ瓧涓婂彂閫侊紝闇€瑕佷娇鐢?sendto(2) 鏉ユ寚瀹氬嚭鍙ｆ帴鍙ｃ€?

浠庣粦瀹氱殑 CAN_RAW 濂楁帴瀛楋紙瑙佷笂锛夎鍙?CAN 甯у寘鎷鍙栦竴涓?struct can_frame锛?


    struct can_frame frame;

    nbytes = read(s, &frame, sizeof(struct can_frame));

    if (nbytes < 0) {
            perror("can raw socket read");
            return 1;
    }

    /** paranoid check ... **/
    if (nbytes < sizeof(struct can_frame)) {
            fprintf(stderr, "read: incomplete CAN frame\n");
            return 1;
    }

    /** do something with the received CAN frame **/

```

    nbytes = write(s, &frame, sizeof(struct can_frame));

```
褰?CAN 鎺ュ彛缁戝畾鍒颁换浣曪紙'any'锛夊凡瀛樺湪鐨?CAN 鎺ュ彛锛坅ddr.can_ifindex = 0锛夋椂锛屽鏋滈渶瑕佸湪鎰忔簮 CAN 鎺ュ彛鐨勪俊鎭紝寤鸿浣跨敤 recvfrom(2)锛?


    struct sockaddr_can addr;
    struct ifreq ifr;
    socklen_t len = sizeof(addr);
    struct can_frame frame;

    nbytes = recvfrom(s, &frame, sizeof(struct can_frame),
                      0, (struct sockaddr*)&addr, &len);

    /** get interface name of the received CAN frame **/
    ifr.ifr_ifindex = addr.can_ifindex;
    ioctl(s, SIOCGIFNAME, &ifr);
    printf("Received a CAN frame from interface %s", ifr.ifr_name);

瑕佸湪缁戝畾鍒?'any' CAN 鎺ュ彛鐨勫鎺ュ瓧涓婂啓鍏?CAN 甯э紝蹇呴』鏄庣‘鎸囧畾鍑哄彛鎺ュ彛锛?


    strcpy(ifr.ifr_name, "can0");
    ioctl(s, SIOCGIFINDEX, &ifr);
    addr.can_ifindex = ifr.ifr_ifindex;
    addr.can_family  = AF_CAN;

    nbytes = sendto(s, &frame, sizeof(struct can_frame),
                    0, (struct sockaddr*)&addr, sizeof(addr));

鍦ㄤ粠濂楁帴瀛楄鍙栨秷鎭箣鍚庯紝鍙互閫氳繃 ioctl(2) 璋冪敤鑾峰彇绮剧‘鐨勬椂闂存埑锛?


    struct timeval tv;
    ioctl(s, SIOCGSTAMP, &tv);

璇ユ椂闂存埑鐨勫垎杈ㄧ巼涓轰竴寰锛屽苟鍦ㄦ帴鏀跺埌 CAN 甯ф椂鑷姩璁剧疆銆?

鍏充簬 CAN FD锛堢伒娲绘暟鎹€熺巼锛夋敮鎸佺殑澶囨敞锛?

閫氬父锛孋AN FD 鐨勫鐞嗕笌鍓嶉潰鎻忚堪鐨勭ず渚嬮潪甯哥浉浼笺€傛柊鐨勬敮鎸?CAN FD 鐨?CAN 鎺у埗鍣ㄤ负 CAN FD 甯х殑浠茶闃舵鍜岃浇鑽烽樁娈垫敮鎸佷袱绉嶄笉鍚岀殑姣旂壒鐜囷紝浠ュ強鏈€澶?64 瀛楄妭鐨勮浇鑽枫€傝繖绉嶆墿灞曠殑杞借嵎闀垮害鐮村潖浜嗘墍鏈変弗閲嶄緷璧栧浐瀹氬叓瀛楄妭杞借嵎鐨?CAN 甯э紙struct can_frame锛屽 CAN_RAW 濂楁帴瀛楋級鐨勫唴鏍告帴鍙ｏ紙ABI锛夈€傚洜姝わ紝渚嬪 CAN_RAW 濂楁帴瀛楁敮鎸佷竴涓柊鐨勫鎺ュ瓧閫夐」 CAN_RAW_FD_FRAMES锛屽畠灏嗗鎺ュ瓧鍒囨崲鍒颁竴涓厑璁稿悓鏃跺鐞?CAN FD 甯у拰缁忓吀 CAN 甯х殑妯″紡锛堣 socketcan-rawfd锛夈€?

struct canfd_frame 瀹氫箟鍦?include/linux/can.h 涓細


    struct canfd_frame {
            canid_t can_id;  /** 32 bit CAN_ID + EFF/RTR/ERR flags **/
            __u8    len;     /** frame payload length in byte (0 .. 64) **/
            __u8    flags;   /** additional flags for CAN FD **/
            __u8    __res0;  /** reserved / padding **/
            __u8    __res1;  /** reserved / padding **/
            __u8    data[^64^] __attribute__((aligned(8)));
    };

struct canfd_frame 鍜屽凡鏈夌殑 struct can_frame 鍦ㄧ粨鏋勪綋鍐呯殑鐩稿悓鍋忕Щ澶勬嫢鏈?can_id銆佽浇鑽烽暱搴﹀拰杞借嵎鏁版嵁銆傝繖浣垮緱鍙互浠ラ潪甯哥浉浼肩殑鏂瑰紡澶勭悊涓嶅悓鐨勭粨鏋勩€傚綋 struct can_frame 鐨勫唴瀹硅澶嶅埗鍒?struct canfd_frame 涓椂锛屾墍鏈夌粨鏋勫厓绱犻兘鍙互鍘熸牱浣跨敤鈥斺€斿彧鏄?data[] 鍙橀暱浜嗐€?

鍦ㄥ紩鍏?struct canfd_frame 鏃跺彂鐜帮紝struct can_frame 鐨勬暟鎹暱搴︿唬鐮侊紙DLC锛夎鐢ㄤ綔闀垮害淇℃伅锛屽洜涓哄湪 0 鍒?8 鐨勮寖鍥村唴闀垮害涓?DLC 鏄?1:1 鏄犲皠鐨勩€備负浜嗕繚鎸侀暱搴︿俊鎭鐞嗙殑绠€渚挎€э紝canfd_frame.len 鍏冪礌鍖呭惈涓€涓粠 0 鍒?64 鐨勬櫘閫氶暱搴﹀€笺€傚洜姝わ紝canfd_frame.len 鍜?can_frame.len 閮界浉绛夛紝骞跺寘鍚暱搴︿俊鎭€岄潪 DLC銆傚叧浜?CAN 涓庢敮鎸?CAN FD 鐨勮澶囩殑鍖哄埆浠ュ強鍒颁笌鎬荤嚎鐩稿叧鐨勬暟鎹暱搴︿唬鐮侊紙DLC锛夌殑鏄犲皠锛岃瑙?socketcan-can-fd-driver銆?

杩欎袱绉?CAN(FD) 甯х粨鏋勭殑闀垮害瀹氫箟浜?CAN(FD) 缃戠粶鎺ュ彛鍜?skbuff 鏁版嵁闀垮害鐨勬渶澶т紶杈撳崟鍏冿紙MTU锛夈€傚湪 include/linux/can.h 涓负 CAN 鐗瑰畾鐨?MTU 瑙勫畾浜嗕袱涓畾涔夛細


  #define CAN_MTU   (sizeof(struct can_frame))   == 16  => 缁忓吀 CAN 甯?
  #define CANFD_MTU (sizeof(struct canfd_frame)) == 72  => CAN FD 甯?


### 杩斿洖鐨勬姤鏂囨爣蹇?


鍦?RAW 鎴?BCM 濂楁帴瀛椾笂浣跨敤绯荤粺璋冪敤 recvmsg(2) 鏃讹紝msg->msg_flags 瀛楁鍙兘鍖呭惈浠ヤ笅鏍囧織锛?

MSG_DONTROUTE锛?
	褰撴帴鏀跺埌鐨勫抚鏄湪鏈湴涓绘満涓婂垱寤烘椂璁剧疆銆?

MSG_CONFIRM锛?
	褰撳抚閫氳繃鎺ユ敹鍒板畠鐨勯偅涓鎺ュ瓧鍙戦€佹椂璁剧疆銆傚綋 CAN 椹卞姩鏀寔椹卞姩灞傞潰鐨勫抚鍥炴樉鏃讹紝姝ゆ爣蹇楀彲琚В閲婁负鈥滃彂閫佺‘璁も€濓紝瑙?socketcan-local-loopback1 鍜?socketcan-local-loopback2銆傦紙娉ㄦ剰锛氫负浜嗗湪 RAW 濂楁帴瀛椾笂鎺ユ敹姝ょ被娑堟伅锛屽繀椤昏缃?CAN_RAW_RECV_OWN_MSGS銆傦級


### 甯?can_filters 鐨?RAW 鍗忚濂楁帴瀛楋紙SOCK_RAW锛?


浣跨敤 CAN_RAW 濂楁帴瀛楀湪寰堝ぇ绋嬪害涓婂彲涓庝紬鎵€鍛ㄧ煡鐨勩€佸 CAN 瀛楃璁惧鐨勮闂浉濯茬編銆備负浜嗘弧瓒冲鐢ㄦ埛 SocketCAN 鏂规鎻愪緵鐨勬柊鍙兘锛屼竴浜涘悎鐞嗙殑榛樿鍊煎湪 RAW 濂楁帴瀛楃粦瀹氭椂琚缃細

- 杩囨护鍣ㄨ璁剧疆涓烘伆濂戒竴涓帴鏀舵墍鏈夊唴瀹圭殑杩囨护鍣?
- 濂楁帴瀛楀彧鎺ユ敹鏈夋晥鐨勬暟鎹抚锛?> 鏃犻敊璇秷鎭抚锛?
- 宸插彂閫?CAN 甯х殑鍥炵幆琚惎鐢紙瑙?socketcan-local-loopback2锛?
- 濂楁帴瀛椾笉鎺ユ敹鑷韩宸插彂閫佺殑甯э紙鍦ㄥ洖鐜ā寮忎笅锛?

杩欎簺榛樿璁剧疆鍙互鍦ㄧ粦瀹氬鎺ュ瓧涔嬪墠鎴栦箣鍚庢洿鏀广€傝浣跨敤 CAN_RAW 濂楁帴瀛楃浉鍏崇殑濂楁帴瀛楅€夐」鐨勫畾涔夛紝璇峰寘鍚?<linux/can/raw.h>銆?


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_FILTER


浣跨敤 CAN_RAW 濂楁帴瀛楁帴鏀?CAN 甯у彲浠ラ€氳繃 CAN_RAW_FILTER 濂楁帴瀛楅€夐」瀹氫箟 0 鍒?n 涓繃婊ゅ櫒鏉ユ帶鍒躲€?

CAN 杩囨护缁撴瀯瀹氫箟鍦?include/linux/can.h 涓細


    struct can_filter {
            canid_t can_id;
            canid_t can_mask;
    };

涓€涓繃婊ゅ櫒鍦ㄤ互涓嬫儏鍐典笅鍖归厤锛?


    <received_can_id> & mask == can_id & mask

杩欑被浼间簬宸茬煡鐨?CAN 鎺у埗鍣ㄧ‖浠惰繃婊よ涔夈€傚綋 can_filter 缁撴瀯鐨?can_id 鍏冪礌涓缃簡 CAN_INV_FILTER 浣嶆椂锛岃杩囨护鍣ㄥ彲浠ュ湪姝よ涔変笅琚弽杞€備笌 CAN 鎺у埗鍣ㄧ‖浠惰繃婊ゅ櫒鐩告瘮锛岀敤鎴峰彲浠ヤ负姣忎釜鎵撳紑鐨勫鎺ュ瓧鍒嗗埆璁剧疆 0 鍒?n 涓帴鏀惰繃婊ゅ櫒锛?


    struct can_filter rfilter[^2^];

    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = CAN_SFF_MASK;
    rfilter[^1^].can_id   = 0x200;
    rfilter[^1^].can_mask = 0x700;

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));

瑕佸湪鎵€閫夌殑 CAN_RAW 濂楁帴瀛椾笂绂佺敤 CAN 甯х殑鎺ユ敹锛?


    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, NULL, 0);

灏嗚繃婊ゅ櫒璁剧疆涓?0 涓繃婊ゅ櫒宸茬粡鐩稿綋杩囨椂浜嗭紝鍥犱负涓嶈鍙栨暟鎹細瀵艰嚧 raw 濂楁帴瀛椾涪寮冩帴鏀跺埌鐨?CAN 甯с€備絾鏈変簡杩欎釜鈥滃彧鍙戦€佲€濈殑鐢ㄤ緥锛屾垜浠彲浠ュ湪鍐呮牳涓Щ闄ゆ帴鏀跺垪琛紝浠ヨ妭鐪佷竴鐐圭偣锛堢湡鐨勯潪甯稿皯锛侊級CPU 浣跨敤鐜囥€?

CAN 杩囨护鍣ㄤ娇鐢ㄤ紭鍖?
.............................

CAN 杩囨护鍣ㄥ湪 CAN 甯ф帴鏀舵椂浜庢瘡璁惧鐨勮繃婊ゅ櫒鍒楄〃涓鐞嗐€備负浜嗗噺灏戦亶鍘嗚繃婊ゅ櫒鍒楄〃鏃堕渶瑕佹墽琛岀殑妫€鏌ユ鏁帮紝褰撹繃婊よ闃呴泦涓簬鍗曚釜 CAN ID 鏃讹紝CAN 鏍稿績鎻愪緵浼樺寲鐨勮繃婊ゅ鐞嗐€?

瀵逛簬鍙兘鐨?2048 涓?SFF CAN 鏍囪瘑绗︼紝鏍囪瘑绗﹁鐢ㄤ綔绱㈠紩鏉ヨ闂浉搴旂殑璁㈤槄鍒楄〃锛岃€屾棤闇€浠讳綍杩涗竴姝ユ鏌ャ€傚浜?2^29 涓彲鑳界殑 EFF CAN 鏍囪瘑绗︼紝浣跨敤 10 浣?XOR 鎶樺彔浣滀负鍝堝笇鍑芥暟鏉ユ绱?EFF 琛ㄧ储寮曘€?

涓轰簡浠庨拡瀵瑰崟涓?CAN 鏍囪瘑绗︾殑浼樺寲杩囨护鍣ㄤ腑鑾风泭锛屽繀椤诲皢 CAN_SFF_MASK 鎴?CAN_EFF_MASK 涓庢墍璁剧疆鐨?CAN_EFF_FLAG 鍜?CAN_RTR_FLAG 浣嶄竴璧疯缃埌 can_filter.mask 涓€俢an_filter.mask 涓缃簡鐨?CAN_EFF_FLAG 浣嶆竻妤氬湴琛ㄦ槑锛岃闃呯殑鏄?SFF 杩樻槸 EFF CAN ID 鏄湁鍖哄埆鐨勩€備緥濡傦紝鍦ㄤ笂闈㈢殑绀轰緥涓細


    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = CAN_SFF_MASK;

SFF 甯э紙CAN ID 0x123锛夊拰 EFF 甯э紙0xXXXXX123锛夐兘鍙互閫氳繃銆?

瑕佷粎杩囨护 0x123锛圫FF锛夊拰 0x12345678锛圗FF锛塁AN 鏍囪瘑绗︼紝蹇呴』浠ヤ笅鍒楁柟寮忓畾涔夎繃婊ゅ櫒鎵嶈兘浠庝紭鍖栫殑杩囨护鍣ㄤ腑鍙楃泭锛?


    struct can_filter rfilter[^2^];

    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = (CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_SFF_MASK);
    rfilter[^1^].can_id   = 0x12345678 | CAN_EFF_FLAG;
    rfilter[^1^].can_mask = (CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_EFF_MASK);

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_ERR_FILTER


濡?socketcan-network-problem-notifications 鎵€杩帮紝CAN 鎺ュ彛椹卞姩鍙互鐢熸垚鎵€璋撶殑閿欒娑堟伅甯э紝瀹冨彲浠ヤ笌鍏朵粬 CAN 甯т竴鏍峰彲閫夊湴浼犻€掔粰鐢ㄦ埛搴旂敤绋嬪簭銆傚彲鑳界殑閿欒琚垝鍒嗕负涓嶅悓鐨勯敊璇被锛屽彲浠ヤ娇鐢ㄩ€傚綋鐨勯敊璇帺鐮佽繘琛岃繃婊ゃ€傝娉ㄥ唽姣忎竴绉嶅彲鑳界殑閿欒鏉′欢锛屽彲浠ヤ娇鐢?CAN_ERR_MASK 浣滀负閿欒鎺╃爜鐨勫€笺€傞敊璇帺鐮佺殑鍊煎畾涔夊湪 linux/can/error.h 涓細


    can_err_mask_t err_mask = ( CAN_ERR_TX_TIMEOUT | CAN_ERR_BUSOFF );

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_ERR_FILTER,
               &err_mask, sizeof(err_mask));


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_LOOPBACK


涓轰簡婊¤冻澶氱敤鎴烽渶姹傦紝鏈湴鍥炵幆榛樿鏄惎鐢ㄧ殑锛堣瑙?socketcan-local-loopback1锛夈€備絾鍦ㄦ煇浜涘祵鍏ュ紡鐢ㄤ緥涓紙渚嬪褰撳彧鏈変竴涓簲鐢ㄧ▼搴忎娇鐢?CAN 鎬荤嚎鏃讹級锛岃繖涓洖鐜姛鑳藉彲浠ヨ绂佺敤锛堥拡瀵规瘡涓鎺ュ瓧鍒嗗埆璁剧疆锛夛細


    int loopback = 0; /** 0 = 绂佺敤, 1 = 鍚敤 (榛樿) **/

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_LOOPBACK, &loopback, sizeof(loopback));


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_RECV_OWN_MSGS


褰撴湰鍦板洖鐜惎鐢ㄦ椂锛屾墍鏈夊凡鍙戦€佺殑 CAN 甯ч兘浼氳鍥炵幆鍒伴偅浜涘湪璇ョ粰瀹氭帴鍙ｄ笂涓鸿繖浜?CAN 甯х殑 CAN-ID 娉ㄥ唽浜嗙殑宸叉墦寮€ CAN 濂楁帴瀛楋紝浠ユ弧瓒冲鐢ㄦ埛闇€姹傘€傚湪鍚屼竴涓彂閫佷簡 CAN 甯х殑濂楁帴瀛椾笂鎺ユ敹璇?CAN 甯ц璁や负鏄笉闇€瑕佺殑锛屽洜姝ら粯璁ょ鐢ㄣ€傝繖涓粯璁よ涓哄彲浠ユ寜闇€姹傛洿鏀癸細


    int recv_own_msgs = 1; /** 0 = 绂佺敤 (榛樿), 1 = 鍚敤 **/

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_RECV_OWN_MSGS,
               &recv_own_msgs, sizeof(recv_own_msgs));

璇锋敞鎰忥紝濂楁帴瀛楄嚜韬?CAN 甯х殑鎺ユ敹涓庡叾浠?CAN 甯т竴鏍峰彈鍒扮浉鍚岀殑杩囨护锛堣 socketcan-rawfilter锛夈€?


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_FD_FRAMES


CAN_RAW 濂楁帴瀛椾腑鐨?CAN FD 鏀寔鍙互閫氳繃涓€涓柊鐨勫鎺ュ瓧閫夐」 CAN_RAW_FD_FRAMES 鍚敤锛岃閫夐」榛樿鍏抽棴銆傚綋 CAN_RAW 濂楁帴瀛椾笉鏀寔璇ユ柊濂楁帴瀛楅€夐」鏃讹紙渚嬪鍦ㄨ緝鏃х殑鍐呮牳涓婏級锛屽垏鎹?CAN_RAW_FD_FRAMES 閫夐」浼氳繑鍥為敊璇?-ENOPROTOOPT銆?

涓€鏃﹀惎鐢ㄤ簡 CAN_RAW_FD_FRAMES锛屽簲鐢ㄧ▼搴忓氨鍙互鍙戦€?CAN 甯у拰 CAN FD 甯с€傚彟涓€鏂归潰锛屽簲鐢ㄧ▼搴忓湪浠庡鎺ュ瓧璇诲彇鏃跺繀椤诲鐞?CAN 鍜?CAN FD 甯э細


    CAN_RAW_FD_FRAMES enabled:  CAN_MTU 鍜?CANFD_MTU 閮藉厑璁?
    CAN_RAW_FD_FRAMES disabled: 鍙厑璁?CAN_MTU (榛樿)

绀轰緥锛?


    [ 璁板緱: CANFD_MTU == sizeof(struct canfd_frame) ]

    struct canfd_frame cfd;

    nbytes = read(s, &cfd, CANFD_MTU);

    if (nbytes == CANFD_MTU) {
            printf("got CAN FD frame with length %d\n", cfd.len);
            /** cfd.flags 鍖呭惈鏈夋晥鏁版嵁 **/
    } else if (nbytes == CAN_MTU) {
            printf("got Classical CAN frame with length %d\n", cfd.len);
            /** cfd.flags 鏈畾涔?**/
    } else {
            fprintf(stderr, "read: invalid CAN(FD) frame\n");
            return 1;
    }

    /** 鍐呭鍙互鐙珛浜庢帴鏀跺埌鐨?MTU 澶у皬鏉ュ鐞?**/

    printf("can_id: %X data length: %d data: ", cfd.can_id, cfd.len);
    for (i = 0; i < cfd.len; i++)
            printf("%02X ", cfd.data[i]);

褰撲互 CANFD_MTU 澶у皬璇诲彇鍙繑鍥炰粠濂楁帴瀛楁帴鏀跺埌鐨?CAN_MTU 瀛楄妭鏃讹紝涓€涓粡鍏?CAN 甯у凡琚鍏ユ墍鎻愪緵鐨?CAN FD 缁撴瀯涓€傝娉ㄦ剰锛宑anfd_frame.flags 鏁版嵁瀛楁鍦?struct can_frame 涓苟鏈瀹氾紝鍥犳瀹冨彧鍦?CANFD_MTU 澶у皬鐨?CAN FD 甯т腑鏈夋晥銆?

鏂?CAN 搴旂敤绋嬪簭鐨勫疄鐜版彁绀猴細

瑕佹瀯寤烘劅鐭?CAN FD 鐨勫簲鐢ㄧ▼搴忥紝璇蜂娇鐢?struct canfd_frame 浣滀负鍩轰簬 CAN_RAW 鐨勫簲鐢ㄧ▼搴忕殑鍩烘湰 CAN 鏁版嵁缁撴瀯銆傚綋搴旂敤绋嬪簭鍦ㄨ緝鏃х殑 Linux 鍐呮牳涓婃墽琛岋紝骞朵笖鍒囨崲 CAN_RAW_FD_FRAMES 濂楁帴瀛楅€夐」杩斿洖閿欒鏃讹細娌℃湁闂銆備綘浼氬緱鍒扮粡鍏?CAN 甯ф垨 CAN FD 甯э紝骞朵笖鍙互鐢ㄧ浉鍚岀殑鏂瑰紡澶勭悊瀹冧滑銆?

鍦ㄥ悜 CAN 璁惧鍙戦€佹椂锛岃纭繚璇ヨ澶囪兘澶熼€氳繃妫€鏌ヨ澶囨渶澶т紶杈撳崟鍏冩槸鍚︿负 CANFD_MTU 鏉ュ鐞?CAN FD 甯с€侰AN 璁惧 MTU 鍙互閫氳繃渚嬪 SIOCGIFMTU ioctl() 绯荤粺璋冪敤鑾峰彇銆?


#### RAW 濂楁帴瀛楅€夐」 CAN_RAW_JOIN_FILTERS


CAN_RAW 濂楁帴瀛楀彲浠ヨ缃涓壒瀹氫簬 CAN 鏍囪瘑绗︾殑杩囨护鍣紝杩欎簺杩囨护鍣ㄥ湪 af_can.c 鐨勮繃婊ゅ鐞嗕腑瀵艰嚧澶氫釜杩囨护鍣ㄣ€傝繖浜涜繃婊ゅ櫒褰兼鐙珛锛屽湪搴旂敤鏃跺鑷撮€昏緫鈥滄垨鈥濓紙OR锛夌殑杩囨护鍣紙瑙?socketcan-rawfilter锛夈€?

杩欎釜濂楁帴瀛楅€夐」浠ヨ繖鏍风殑鏂瑰紡杩炴帴缁欏畾鐨?CAN 杩囨护鍣細鍙湁鍖归厤**鎵€鏈?*缁欏畾 CAN 杩囨护鍣ㄧ殑 CAN 甯ф墠浼氳浼犻€掑埌鐢ㄦ埛绌洪棿銆傚洜姝わ紝鎵€搴旂敤杩囨护鍣ㄧ殑璇箟琚敼鍙樹负閫昏緫鈥滀笌鈥濓紙AND锛夈€?

杩欏湪杩囨护鍣ㄩ泦鍚堟槸杩囨护鍣ㄧ粍鍚堛€佷笖鍏朵腑璁剧疆浜?CAN_INV_FILTER 鏍囧織浠ヤ究浠庝紶鍏ユ祦閲忎腑婊ら櫎鍗曚釜 CAN ID 鎴?CAN ID 鑼冨洿鏃剁壒鍒湁鐢ㄣ€?


### 骞挎挱绠＄悊鍣ㄥ崗璁鎺ュ瓧锛圫OCK_DGRAM锛?


骞挎挱绠＄悊鍣ㄥ崗璁彁渚涗簡涓€涓熀浜庡懡浠ょ殑閰嶇疆鎺ュ彛锛岀敤浜庡湪鍐呮牳绌洪棿涓繃婊ゅ拰鍙戦€侊紙渚嬪鍛ㄦ湡鎬э級CAN 娑堟伅銆?

鎺ユ敹杩囨护鍣ㄥ彲鐢ㄤ簬瀵归绻佺殑娑堟伅杩涜闄嶉噰鏍凤紱妫€娴嬭濡傛秷鎭唴瀹规敼鍙樸€佸寘闀垮害鏀瑰彉绛変簨浠讹紝骞跺鎺ユ敹鍒扮殑娑堟伅杩涜瓒呮椂鐩戞帶銆?

鍙互鍒涘缓 CAN 甯ф垨 CAN 甯у簭鍒楃殑鍛ㄦ湡鎬у彂閫佷换鍔★紝骞跺湪杩愯鏃朵慨鏀癸紱娑堟伅鍐呭鍜屼袱绉嶅彲鑳界殑鍙戦€侀棿闅旈兘鍙互鏇存敼銆?

BCM 濂楁帴瀛椾笉鎵撶畻鐢ㄤ簬浣跨敤宸茬煡鏉ヨ嚜 CAN_RAW 濂楁帴瀛楃殑 struct can_frame 鍙戦€佸崟涓?CAN 甯с€傜浉鍙嶏紝瀹氫箟浜嗕竴涓壒娈婄殑 BCM 閰嶇疆娑堟伅銆傜敤浜庝笌骞挎挱绠＄悊鍣ㄩ€氫俊鐨勫熀鏈?BCM 閰嶇疆娑堟伅浠ュ強鍙敤鐨勬搷浣滃畾涔夊湪 linux/can/bcm.h 澶存枃浠朵腑銆侭CM 娑堟伅鐢变竴涓甫鍛戒护锛?opcode'锛夌殑娑堟伅澶翠互鍙婇浂涓垨澶氫釜 CAN 甯х粍鎴愩€傚箍鎾鐞嗗櫒浠ョ浉鍚岀殑褰㈠紡鍚戠敤鎴风┖闂村彂閫佸搷搴旓細


    struct bcm_msg_head {
            __u32 opcode;                   /** command **/
            __u32 flags;                    /** special flags **/
            __u32 count;                    /** run 'count' times with ival1 **/
            struct timeval ival1, ival2;    /** count and subsequent interval **/
            canid_t can_id;                 /** unique can_id for task **/
            __u32 nframes;                  /** number of can_frames following **/
            struct can_frame frames[];
    };

瀵归綈鐨勮浇鑽?'frames' 浣跨敤鍦?socketcan-rawfd 寮€澶村拰 include/linux/can.h 澶存枃浠朵腑瀹氫箟鐨勭浉鍚屽熀鏈?CAN 甯х粨鏋勩€傛墍鏈変粠鐢ㄦ埛绌洪棿鍙戝線骞挎挱绠＄悊鍣ㄧ殑娑堟伅閮藉叿鏈夋缁撴瀯銆?

璇锋敞鎰忥紝CAN_BCM 濂楁帴瀛楀湪鍒涘缓鍚庡繀椤?connect 鑰岄潪 bind锛堢ず渚嬫湭鍋氶敊璇鏌ワ級锛?


    int s;
    struct sockaddr_can addr;
    struct ifreq ifr;

    s = socket(PF_CAN, SOCK_DGRAM, CAN_BCM);

    strcpy(ifr.ifr_name, "can0");
    ioctl(s, SIOCGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    connect(s, (struct sockaddr *)&addr, sizeof(addr));

    (..)

骞挎挱绠＄悊鍣ㄥ鎺ュ瓧鑳藉骞跺彂澶勭悊浠绘剰鏁伴噺鐨勫湪閫斿彂閫佹垨鎺ユ敹杩囨护鍣ㄣ€備笉鍚岀殑 RX/TX 浣滀笟閫氳繃姣忎釜 BCM 娑堟伅涓敮涓€鐨?can_id 鏉ュ尯鍒嗐€備絾寤鸿浣跨敤棰濆鐨?CAN_BCM 濂楁帴瀛楀湪澶氫釜 CAN 鎺ュ彛涓婇€氫俊銆傚綋骞挎挱绠＄悊鍣ㄥ鎺ュ瓧缁戝畾鍒?'any' CAN 鎺ュ彛锛?> 鎺ュ彛绱㈠紩璁剧疆涓洪浂锛夋椂锛屾墍閰嶇疆鐨勬帴鏀惰繃婊ゅ櫒閫傜敤浜庝换浣?CAN 鎺ュ彛锛岄櫎闈炰娇鐢?sendto() 绯荤粺璋冪敤鏉ヨ鐩?'any' CAN 鎺ュ彛绱㈠紩銆傚綋浣跨敤 recvfrom() 鑰岄潪 read() 鏉ユ绱?BCM 濂楁帴瀛楁秷鎭椂锛屾簮 CAN 鎺ュ彛鍦?can_ifindex 涓彁渚涖€?


#### 骞挎挱绠＄悊鍣ㄦ搷浣?


opcode 瀹氫箟浜嗗箍鎾鐞嗗櫒瑕佹墽琛岀殑鎿嶄綔锛屾垨鑰呰杩颁簡骞挎挱绠＄悊鍣ㄥ鑻ュ共浜嬩欢鐨勫搷搴旓紝鍖呮嫭鐢ㄦ埛璇锋眰銆?

鍙戦€佹搷浣滐紙鐢ㄦ埛绌洪棿鍒板箍鎾鐞嗗櫒锛夛細

TX_SETUP锛?
	鍒涘缓锛堝懆鏈熸€э級鍙戦€佷换鍔°€?

TX_DELETE锛?
	绉婚櫎锛堝懆鏈熸€э級鍙戦€佷换鍔★紝鍙渶瑕?can_id銆?

TX_READ锛?
	璇诲彇锛堝懆鏈熸€э級鍙戦€佷换鍔＄殑灞炴€э紝閽堝 can_id銆?

TX_SEND锛?
	鍙戦€佷竴涓?CAN 甯с€?

鍙戦€佸搷搴旓紙骞挎挱绠＄悊鍣ㄥ埌鐢ㄦ埛绌洪棿锛夛細

TX_STATUS锛?
	瀵?TX_READ 璇锋眰鐨勫洖澶嶏紙鍙戦€佷换鍔￠厤缃級銆?

TX_EXPIRED锛?
	褰撹鏁板櫒浠ュ垵濮嬮棿闅?'ival1' 瀹屾垚鍙戦€佹椂鐨勯€氱煡銆?
	闇€瑕佸湪 TX_SETUP 鏃惰缃?TX_COUNTEVT 鏍囧織銆?

鎺ユ敹鎿嶄綔锛堢敤鎴风┖闂村埌骞挎挱绠＄悊鍣級锛?

RX_SETUP锛?
	鍒涘缓 RX 鍐呭杩囨护鍣ㄨ闃呫€?

RX_DELETE锛?
	绉婚櫎 RX 鍐呭杩囨护鍣ㄨ闃咃紝鍙渶瑕?can_id銆?

RX_READ锛?
	璇诲彇閽堝 can_id 鐨?RX 鍐呭杩囨护鍣ㄨ闃呯殑灞炴€с€?

鎺ユ敹鍝嶅簲锛堝箍鎾鐞嗗櫒鍒扮敤鎴风┖闂达級锛?

RX_STATUS锛?
	瀵?RX_READ 璇锋眰鐨勫洖澶嶏紙杩囨护鍣ㄤ换鍔￠厤缃級銆?

RX_TIMEOUT锛?
	妫€娴嬪埌鍛ㄦ湡娑堟伅缂哄け锛堝畾鏃跺櫒 ival1 杩囨湡锛夈€?

RX_CHANGED锛?
	甯︽湁鏇存柊 CAN 甯х殑 BCM 娑堟伅锛堟娴嬪埌鍐呭鏀瑰彉锛夈€?
	鍦ㄦ敹鍒扮涓€鏉℃秷鎭垨鏀跺埌淇鐨?CAN 娑堟伅鏃跺彂閫併€?


#### 骞挎挱绠＄悊鍣ㄦ秷鎭爣蹇?


褰撳悜骞挎挱绠＄悊鍣ㄥ彂閫佹秷鎭椂锛?flags' 鍏冪礌鍙互鍖呭惈浠ヤ笅褰卞搷琛屼负鐨勬爣蹇楀畾涔夛細

SETTIMER锛?
	璁剧疆 ival1銆乮val2 鍜?count 鐨勫€笺€?

STARTTIMER锛?
	浠?ival1銆乮val2 鍜?count 鐨勫疄闄呭€煎惎鍔ㄥ畾鏃跺櫒銆?
	鍚姩瀹氭椂鍣ㄥ悓鏃朵細瀵艰嚧鍙戝嚭涓€涓?CAN 甯с€?

TX_COUNTEVT锛?
	褰?count 杩囨湡鏃跺垱寤?TX_EXPIRED 娑堟伅銆?

TX_ANNOUNCE锛?
	杩涚▼瀵规暟鎹殑鏀瑰彉浼氱珛鍗冲彂鍑恒€?

TX_CP_CAN_ID锛?
	灏?can_id 浠庢秷鎭ご澶嶅埗鍒?frames 涓殑姣忎釜鍚庣画甯с€傝繖鏃ㄥ湪绠€鍖栦娇鐢ㄣ€傚浜?TX 浠诲姟锛屾秷鎭ご涓敮涓€鐨?can_id 鍙兘涓庝负鍚庣画 struct can_frame(s) 涓紶杈撹€屽瓨鍌ㄧ殑 can_id(s) 涓嶅悓銆?

RX_FILTER_ID锛?
	浠呮寜 can_id 杩囨护锛屼笉闇€瑕佸抚锛坣frames=0锛夈€?

RX_CHECK_DLC锛?
	DLC 鐨勬敼鍙樹細瀵艰嚧 RX_CHANGED銆?

RX_NO_AUTOTIMER锛?
	闃绘鑷姩鍚姩瓒呮椂鐩戞帶銆?

RX_ANNOUNCE_RESUME锛?
	濡傛灉鍦?RX_SETUP 鏃朵紶鍏ワ紝涓斿彂鐢熶簡鎺ユ敹瓒呮椂锛屽垯锛堝懆鏈熸€э級鎺ユ敹閲嶅惎鏃朵細鐢熸垚涓€鏉?RX_CHANGED 娑堟伅銆?

TX_RESET_MULTI_IDX锛?
	閲嶇疆澶氬抚浼犺緭鐨勭储寮曘€?

RX_RTR_FRAME锛?
	鍙戦€佸 RTR 璇锋眰鐨勫洖澶嶏紙鏀惧湪 op->frames[^0^] 涓級銆?

CAN_FD_FRAME锛?
	bcm_msg_head 鍚庨潰鐨?CAN 甯ф槸 struct canfd_frame銆?

#### 骞挎挱绠＄悊鍣ㄥ彂閫佸畾鏃跺櫒


鍛ㄦ湡鎬у彂閫侀厤缃渶澶氬彲浠ヤ娇鐢ㄤ袱涓棿闅斿畾鏃跺櫒銆傚湪杩欑鎯呭喌涓嬶紝BCM 浠ヤ竴涓棿闅?'ival1' 鍙戦€佽嫢骞叉秷鎭紙'count'锛夛紝鐒跺悗浠ュ彟涓€涓粰瀹氶棿闅?'ival2' 缁х画鍙戦€併€傚綋鍙渶瑕佷竴涓畾鏃跺櫒鏃讹紝'count' 璁剧疆涓洪浂锛屽苟涓斿彧浣跨敤 'ival2'銆傚綋璁剧疆浜?SET_TIMER 鍜?START_TIMER 鏍囧織鏃讹紝瀹氭椂鍣ㄨ婵€娲汇€傚綋鍙缃簡 SET_TIMER 鏃讹紝瀹氭椂鍣ㄥ€煎彲浠ュ湪杩愯鏃舵洿鏀广€?


#### 骞挎挱绠＄悊鍣ㄦ秷鎭簭鍒楀彂閫?


鍦ㄥ懆鏈熸€?TX 浠诲姟閰嶇疆鐨勬儏鍐典笅锛屾渶澶?256 涓?CAN 甯у彲浠ユ寜搴忓垪鍙戦€併€侰AN 甯х殑鏁伴噺鍦?BCM 娑堟伅澶寸殑 'nframes' 鍏冪礌涓彁渚涖€傛墍瀹氫箟鐨?CAN 甯ф暟閲忎綔涓烘暟缁勬坊鍔犲埌 TX_SETUP BCM 閰嶇疆娑堟伅涓細


    /** 鍒涘缓涓€涓敤浜庤缃洓涓?CAN 甯у簭鍒楃殑缁撴瀯 **/
    struct {
            struct bcm_msg_head msg_head;
            struct can_frame frame[^4^];
    } mytxmsg;

    (..)
    mytxmsg.msg_head.nframes = 4;
    (..)

    write(s, &mytxmsg, sizeof(mytxmsg));

姣忔鍙戦€佹椂锛孋AN 甯ф暟缁勪腑鐨勭储寮曚細閫掑锛屽苟鍦ㄧ储寮曟孩鍑烘椂閲嶇疆涓洪浂銆?


#### 骞挎挱绠＄悊鍣ㄦ帴鏀惰繃婊ゅ櫒瀹氭椂鍣?


瀹氭椂鍣ㄥ€?ival1 鎴?ival2 鍙互鍦?RX_SETUP 鏃惰缃负闈為浂鍊笺€傚綋璁剧疆浜?SET_TIMER 鏍囧織鏃讹紝瀹氭椂鍣ㄨ鍚敤锛?

ival1锛?
	褰撴帴鏀跺埌鐨勬秷鎭湪缁欏畾鏃堕棿鍐呮湭鍐嶆鏀跺埌鏃讹紝鍙戦€?RX_TIMEOUT銆傚鏋滃湪 RX_SETUP 鏃惰缃簡 START_TIMER锛屽垯瓒呮椂妫€娴嬩細鐩存帴婵€娲烩€斺€斿嵆浣挎病鏈夊厛鍓嶇殑 CAN 甯ф帴鏀躲€?

ival2锛?
	灏嗘帴鏀跺埌鐨勬秷鎭€熺巼闄愬埗鍒?ival2 鐨勫€笺€傚綋 CAN 甯у唴鐨勪俊鍙锋槸鏃犵姸鎬佺殑銆佷笖 ival2 鍛ㄦ湡鍐呯殑鐘舵€佹敼鍙樺彲鑳戒細涓㈠け鏃讹紝杩欏浜庡噺灏戝簲鐢ㄧ▼搴忕殑娑堟伅閲忓緢鏈夌敤銆?

#### 骞挎挱绠＄悊鍣ㄥ璺鐢ㄦ秷鎭帴鏀惰繃婊ゅ櫒


涓轰簡杩囨护澶氳矾澶嶇敤娑堟伅搴忓垪涓殑鍐呭鏀瑰彉锛屽彲浠ュ湪 RX_SETUP 閰嶇疆娑堟伅涓紶鍏ュ浜庝竴涓?CAN 甯х殑鏁扮粍銆傜涓€涓?CAN 甯х殑鏁版嵁瀛楄妭鍖呭惈鐩稿叧浣嶇殑鎺╃爜锛岃繖浜涗綅蹇呴』鍦ㄥ悗缁?CAN 甯т腑涓庢帴鏀跺埌鐨?CAN 甯у尮閰嶃€傚鏋滄煇涓悗缁?CAN 甯у尮閰嶏紝鍒欒甯ф暟鎹腑鐨勪綅鏍囪瑕佷笌鍏堝墠鎺ユ敹鐨勫唴瀹硅繘琛屾瘮杈冪殑鐩稿叧鍐呭銆傛渶澶?257 涓?CAN 甯э紙澶氳矾澶嶇敤杩囨护鍣ㄤ綅鎺╃爜 CAN 甯у姞 256 涓?CAN 杩囨护鍣級鍙互浣滀负鏁扮粍娣诲姞鍒?TX_SETUP BCM 閰嶇疆娑堟伅涓細


    /** 閫氬父鐢ㄤ簬娓呯┖ CAN 甯?data[] - 娉ㄦ剰澶у皬绔棶棰? **/
    #define U64_DATA(p) (**(unsigned long long**)(p)->data)

    struct {
            struct bcm_msg_head msg_head;
            struct can_frame frame[^5^];
    } msg;

    msg.msg_head.opcode  = RX_SETUP;
    msg.msg_head.can_id  = 0x42;
    msg.msg_head.flags   = 0;
    msg.msg_head.nframes = 5;
    U64_DATA(&msg.frame[^0^]) = 0xFF00000000000000ULL; /** MUX mask **/
    U64_DATA(&msg.frame[^1^]) = 0x01000000000000FFULL; /** data mask (MUX 0x01) **/
    U64_DATA(&msg.frame[^2^]) = 0x0200FFFF000000FFULL; /** data mask (MUX 0x02) **/
    U64_DATA(&msg.frame[^3^]) = 0x330000FFFFFF0003ULL; /** data mask (MUX 0x33) **/
    U64_DATA(&msg.frame[^4^]) = 0x4F07FC0FF0000000ULL; /** data mask (MUX 0x4F) **/

    write(s, &msg, sizeof(msg));


#### 骞挎挱绠＄悊鍣?CAN FD 鏀寔


CAN_BCM 鐨勭紪绋?API 渚濊禆浜?struct can_frame锛屽畠浣滀负鏁扮粍鐩存帴鏀惧湪 bcm_msg_head 缁撴瀯涔嬪悗銆備负浜嗗 CAN FD 甯ч伒寰妯″紡锛宐cm_msg_head 鏍囧織涓殑涓€涓柊鏍囧織 'CAN_FD_FRAME' 鎸囩ず bcm_msg_head 鍚庨潰杩炴帴鐨?CAN 甯х粨鏋勮瀹氫箟涓?struct canfd_frame锛?


    struct {
            struct bcm_msg_head msg_head;
            struct canfd_frame frame[^5^];
    } msg;

    msg.msg_head.opcode  = RX_SETUP;
    msg.msg_head.can_id  = 0x42;
    msg.msg_head.flags   = CAN_FD_FRAME;
    msg.msg_head.nframes = 5;
    (..)

褰撲娇鐢?CAN FD 甯ц繘琛屽璺鐢ㄨ繃婊ゆ椂锛孧UX 鎺╃爜浠嶇劧鏈熸湜鍦?struct canfd_frame 鏁版嵁娈电殑鍓?64 浣嶄腑銆?


### 闈㈠悜杩炴帴鐨勪紶杈撳崗璁紙SOCK_SEQPACKET锛?


锛堝緟鍐欙級


### 鏃犺繛鎺ョ殑浼犺緭鍗忚锛圫OCK_DGRAM锛?


锛堝緟鍐欙級


## SocketCAN 鏍稿績妯″潡


SocketCAN 鏍稿績妯″潡瀹炵幇浜嗗崗璁棌 PF_CAN銆侰AN 鍗忚妯″潡鍦ㄨ繍琛屾椂鐢辨牳蹇冩ā鍧楀姞杞姐€傛牳蹇冩ā鍧椾负 CAN 鍗忚妯″潡鎻愪緵浜嗕竴涓帴鍙ｆ潵璁㈤槄鎵€闇€鐨?CAN ID锛堣 socketcan-receive-lists锛夈€?


### can.ko 妯″潡鍙傛暟


- **stats_timer**锛?
  涓轰簡璁＄畻 SocketCAN 鏍稿績缁熻淇℃伅锛堜緥濡傚綋鍓?鏈€澶ф瘡绉掑抚鏁帮級锛岃繖涓?1 绉掑畾鏃跺櫒榛樿鍦?can.ko 妯″潡鍚姩鏃跺惎鍔ㄣ€傚彲浠ラ€氳繃鍦ㄦā鍧楀懡浠よ涓婁娇鐢?stattimer=0 鏉ョ鐢ㄦ瀹氭椂鍣ㄣ€?

- **debug**锛?
  锛堣嚜 SocketCAN SVN r546 璧峰凡绉婚櫎锛?


### procfs 鍐呭


濡?socketcan-receive-lists 鎵€杩帮紝SocketCAN 鏍稿績浣跨敤澶氫釜杩囨护鍣ㄥ垪琛ㄥ皢鎺ユ敹鍒扮殑 CAN 甯ф姇閫掔粰 CAN 鍗忚妯″潡銆傝繖浜涙帴鏀跺垪琛ㄣ€佸畠浠殑杩囨护鍣ㄤ互鍙婅繃婊ゅ櫒鍖归厤娆℃暟鍙互鍦ㄧ浉搴旂殑鎺ユ敹鍒楄〃涓煡鐪嬨€傛墍鏈夋潯鐩兘鍖呭惈
```

    foo@bar:~$ cat /proc/net/can/rcvlist_all

    receive list 'rx_all':
      (vcan3: no entry)
      (vcan2: no entry)
      (vcan1: no entry)
      device   can_id   can_mask  function  userdata   matches  ident
       vcan0     000    00000000  f88e6370  f6c6f400         0  raw
      (any: no entry)

```
```

    rcvlist_all - 鏈繃婊ゆ潯鐩殑鍒楄〃 (鏃犺繃婊ゆ搷浣?
    rcvlist_eff - 鍗曚釜鎵╁睍甯?(EFF) 鏉＄洰鐨勫垪琛?
    rcvlist_err - 閿欒娑堟伅甯ф帺鐮佺殑鍒楄〃
    rcvlist_fil - 鎺╃爜/鍊艰繃婊ゅ櫒鐨勫垪琛?
    rcvlist_inv - 鎺╃爜/鍊艰繃婊ゅ櫒鐨勫垪琛?(閫嗚涔?
    rcvlist_sff - 鍗曚釜鏍囧噯甯?(SFF) 鏉＄洰鐨勫垪琛?

```
```

    stats       - SocketCAN 鏍稿績缁熻淇℃伅 (rx/tx 甯? 鍖归厤姣旂巼, ...)
    reset_stats - 鎵嬪姩缁熻閲嶇疆
    version     - 鎵撳嵃 SocketCAN 鏍稿績鍜?ABI 鐗堟湰 (鍦?Linux 5.10 涓Щ闄?

```
### 缂栧啓鑷繁鐨?CAN 鍗忚妯″潡


瑕佸湪鍗忚鏃?PF_CAN 涓疄鐜版柊鍗忚锛屽繀椤诲湪 include/linux/can.h 涓畾涔夋柊鍗忚銆傜敤浜庝娇鐢?SocketCAN 鏍稿績鐨勫師鍨嬩笌瀹氫箟鍙互閫氳繃鍖呭惈 include/linux/can/core.h 鏉ヨ闂€傞櫎浜嗘敞鍐?CAN 鍗忚鍜?CAN 璁惧閫氱煡閾剧殑鍑芥暟澶栵紝杩樻湁璁㈤槄 CAN 鐨勫嚱鏁?
```

    can_rx_register   - 璁㈤槄鏉ヨ嚜鐗瑰畾鎺ュ彛鐨?CAN 甯?
    can_rx_unregister - 閫€璁㈡潵鑷壒瀹氭帴鍙ｇ殑 CAN 甯?
    can_send          - 鍙戦€佷竴涓?CAN 甯?(鍙€夋嫨甯︽湰鍦板洖鐜?

```
鏈夊叧璇︽儏锛岃鍙傝 net/can/af_can.c 涓殑 kerneldoc 鏂囨。锛屾垨 net/can/raw.c 涓?net/can/bcm.c 鐨勬簮浠ｇ爜銆?


## CAN 缃戠粶椹卞姩


缂栧啓 CAN 缃戠粶璁惧椹卞姩姣旂紪鍐?CAN 瀛楃璁惧椹卞姩瑕佸鏄撳緱澶氥€備笌鍏朵粬宸茬煡鐨勭綉缁滆澶囬┍鍔ㄧ被浼硷紝浣犱富瑕侀渶瑕佸鐞嗭細

- TX锛氬皢 CAN 甯т粠濂楁帴瀛楃紦鍐插尯閫佸叆 CAN 鎺у埗鍣ㄣ€?
- RX锛氬皢 CAN 甯т粠 CAN 鎺у埗鍣ㄩ€佸叆濂楁帴瀛楃紦鍐插尯銆?

鍙傝渚嬪 Documentation/networking/netdevices.rst 銆傜紪鍐?CAN 缃戠粶璁惧椹卞姩鐨勪笉鍚屼箣澶勬弿杩板涓嬶細


### 閫氱敤璁剧疆


CAN 缃戠粶璁惧椹卞姩鍙互浣跨敤 alloc_candev_mqs() 鍙婂叾鐩稿叧鍑芥暟锛岃€岄潪 alloc_netdev_mqs()锛屼互鑷姩澶勭悊 CAN 鐗规湁鐨勫垵濮嬪寲宸ヤ綔锛?


    dev = alloc_candev_mqs(...);

struct can_frame 鎴?struct canfd_frame 鏄?PF_CAN 鍗忚鏃忎腑姣忎釜濂楁帴瀛楃紦鍐插尯锛坰kbuff锛夌殑杞借嵎銆?


### 宸插彂閫佸抚鐨勬湰鍦板洖鐜?


濡?socketcan-local-loopback1 鎵€杩帮紝CAN 缃戠粶璁惧椹卞姩搴旀敮鎸佷竴绉嶇被浼间簬鏈湴鍥炴樉锛堜緥濡?tty 璁惧閭ｆ牱锛夌殑鏈湴鍥炵幆鍔熻兘銆傚湪杩欑鎯呭喌涓嬶紝蹇呴』璁剧疆椹卞姩鏍囧織 IFF_ECHO锛屼互闃叉 PF_CAN 鏍稿績瀵瑰凡鍙戦€佺殑甯ц繘琛屾湰鍦板洖鏄俱€?
```

    dev->flags = (IFF_NOARP | IFF_ECHO);


```
### CAN 鎺у埗鍣ㄧ‖浠惰繃婊ゅ櫒


涓轰簡鍑忓皯娣卞害宓屽叆寮忕郴缁熶笂鐨勪腑鏂礋杞斤紝涓€浜?CAN 鎺у埗鍣ㄦ敮鎸佸 CAN ID 鎴?CAN ID 鑼冨洿杩涜杩囨护銆傝繖浜涚‖浠惰繃婊よ兘鍔涘洜鎺у埗鍣ㄨ€屽紓锛屽苟涓斿繀椤昏瀹氫负鍦ㄥ鐢ㄦ埛缃戠粶鏂规涓槸涓嶅彲琛岀殑銆備娇鐢ㄩ珮搴︽帶鍒跺櫒鐩稿叧鐨勭‖浠惰繃婊ゅ櫒鍙兘鍙湪闈炲父涓撶敤鐨勭敤渚嬩腑鏈夋剰涔夛紝鍥犱负椹卞姩灞傞潰鐨勮繃婊ゅ櫒浼氬奖鍝嶅鐢ㄦ埛绯荤粺涓殑鎵€鏈夌敤鎴枫€侾F_CAN 鏍稿績鍐呴儴鐨勯珮鏁堣繃婊ゅ櫒闆嗗悎鍏佽涓烘瘡涓鎺ュ瓧鍒嗗埆璁剧疆澶氫釜涓嶅悓鐨勮繃婊ゅ櫒銆傚洜姝わ紝纭欢杩囨护鍣ㄧ殑浣跨敤褰掑叆鈥滄繁搴﹀祵鍏ュ紡绯荤粺涓婄殑鎵嬪伐璋冧紭鈥濊繖涓€绫诲埆銆備綔鑰呭湪涓€涓?@133MHz 鐨?MPC603e 涓婏紝浣跨敤鍥涗釜 SJA1000 CAN 鎺у埗鍣紝鑷?2002 骞磋捣鍦ㄩ噸鎬荤嚎璐熻浇涓嬭繍琛岃€屾病鏈変换浣曢棶棰樷€︹€?


### 鍙垏鎹㈢殑缁堢鐢甸樆


CAN 鎬荤嚎闇€瑕佸湪宸垎瀵逛袱绔彁渚涚壒瀹氱殑闃绘姉锛岄€氬父鐢辨€荤嚎鏈€杩滅鐨勮妭鐐逛笂鐨勪袱涓?120Ohm 鐢甸樆鎻愪緵銆備竴浜?CAN 鎺у埗鍣ㄦ敮鎸佹縺娲?鍋滅敤缁堢鐢甸樆浠ユ彁渚涙纭殑闃绘姉銆?

```

    $ ip -details link show can0
    ...
    termination 120 [ 0, 120 ]


```
```

    $ ip link set dev can0 type can termination 120


```
```

    $ ip link set dev can0 type can termination 0


```
瑕佷负 can 鎺у埗鍣ㄥ惎鐢ㄧ粓绔數闃绘敮鎸侊紝瑕佷箞
```

    termination_const
    termination_const_cnt
    do_set_termination


```
瑕佷箞閫氳繃浠ヤ笅璁惧鏍戞潯鐩坊鍔?gpio 鎺у埗锛?
Documentation/devicetree/bindings/net/can/can-controller.yaml


### 铏氭嫙 CAN 椹卞姩锛坴can锛?


涓庣綉缁滃洖鐜澶囩被浼硷紝vcan 鎻愪緵浜嗕竴涓櫄鎷熺殑鏈湴 CAN 鎺ュ彛銆侰AN 涓婄殑涓€涓畬鏁撮檺瀹氬湴鍧€鐢变互涓嬮儴鍒嗙粍鎴愶細

- 涓€涓敮涓€鐨?CAN 鏍囪瘑绗︼紙CAN ID锛?
- 璇?CAN ID 鎵€浼犺緭鍒扮殑 CAN 鎬荤嚎锛堜緥濡?can0锛?

鍥犳锛屽湪甯歌鐢ㄤ緥涓渶瑕佷笉姝竴涓櫄鎷?CAN 鎺ュ彛銆?

铏氭嫙 CAN 鎺ュ彛鍏佽鍦ㄦ病鏈夌湡瀹?CAN 鎺у埗鍣ㄧ‖浠剁殑鎯呭喌涓嬫敹鍙?CAN 甯с€傝櫄鎷?CAN 缃戠粶璁惧閫氬父鍛藉悕涓?'vcanX'锛屼緥濡?vcan0銆乿can1銆乿can2鈥︹€﹀綋缂栬瘧涓烘ā鍧楁椂锛岃櫄鎷?CAN 椹卞姩妯″潡鍚嶄负 vcan.ko銆?

鑷?Linux 鍐呮牳鐗堟湰 2.6.24 璧凤紝vcan 椹卞姩鏀寔鍐呮牳 netlink 鎺ュ彛鏉ュ垱寤?vcan 缃戠粶璁惧銆傚垱寤轰互鍙?
```

  - 鍒涘缓涓€涓櫄鎷?CAN 缃戠粶鎺ュ彛锛?
       $ ip link add type vcan

  - 鍒涘缓涓€涓寚瀹氬悕绉?'vcan42' 鐨勮櫄鎷?CAN 缃戠粶鎺ュ彛锛?
       $ ip link add dev vcan42 type vcan

  - 绉婚櫎涓€涓紙铏氭嫙 CAN锛夌綉缁滄帴鍙?'vcan42'锛?
       $ ip link del vcan42


```
### CAN 缃戠粶璁惧椹卞姩鎺ュ彛


CAN 缃戠粶璁惧椹卞姩鎺ュ彛鎻愪緵浜嗕竴涓敤浜庤缃€侀厤缃拰鐩戞帶 CAN 缃戠粶璁惧鐨勯€氱敤鎺ュ彛銆傜劧鍚庣敤鎴峰彲浠ラ厤缃?CAN 璁惧锛屼緥濡傞€氳繃 netlink 鎺ュ彛浣跨敤 "IPROUTE2" 宸ュ叿濂椾欢涓殑 "ip" 绋嬪簭鏉ヨ缃綅鏃跺簭鍙傛暟銆備笅闈竴绔犵畝瑕佹弿杩颁簡濡備綍浣跨敤瀹冦€傛澶栵紝璇ユ帴鍙ｄ娇鐢ㄤ竴涓€氱敤鐨勬暟鎹粨鏋勫苟瀵煎嚭涓€缁勫叕鍏卞嚱鏁帮紝鎵€鏈夌湡瀹炵殑 CAN 缃戠粶璁惧椹卞姩閮藉簲褰撲娇鐢ㄥ畠浠€傝鍙傝€?SJA1000 鎴?MSCAN 椹卞姩浠ヤ簡瑙ｅ浣曚娇鐢ㄥ畠浠€傝妯″潡鍚嶄负 can-dev.ko銆?


#### 鐢ㄤ簬璁剧疆/鑾峰彇璁惧灞炴€х殑 Netlink 鎺ュ彛


CAN 璁惧蹇呴』閫氳繃 netlink 鎺ュ彛杩涜閰嶇疆銆傛敮鎸佺殑 netlink 娑堟伅绫诲瀷鍦?"include/linux/can/netlink.h" 涓畾涔夊苟浣滀簡绠€瑕佽鏄庛€侷PROUTE2 宸ュ叿濂椾欢涓?"ip" 绋嬪簭鐨?CAN 閾捐矾鏀寔宸茬粡鍙敤锛屽叾鐢ㄦ硶濡備笅鎵€绀猴細

```

    $ ip link set can0 type can help
    Usage: ip link set DEVICE type can
        [ bitrate BITRATE [ sample-point SAMPLE-POINT] ] |
        [ tq TQ prop-seg PROP_SEG phase-seg1 PHASE-SEG1
          phase-seg2 PHASE-SEG2 [ sjw SJW ] ]

        [ dbitrate BITRATE [ dsample-point SAMPLE-POINT] ] |
        [ dtq TQ dprop-seg PROP_SEG dphase-seg1 PHASE-SEG1
          dphase-seg2 PHASE-SEG2 [ dsjw SJW ] ]

        [ loopback { on | off } ]
        [ listen-only { on | off } ]
        [ triple-sampling { on | off } ]
        [ one-shot { on | off } ]
        [ berr-reporting { on | off } ]
        [ fd { on | off } ]
        [ fd-non-iso { on | off } ]
        [ presume-ack { on | off } ]
        [ cc-len8-dlc { on | off } ]

        [ restart-ms TIME-MS ]
        [ restart ]

        Where: BITRATE       := { 1..1000000 }
               SAMPLE-POINT  := { 0.000..0.999 }
               TQ            := { NUMBER }
               PROP-SEG      := { 1..8 }
               PHASE-SEG1    := { 1..8 }
               PHASE-SEG2    := { 1..8 }
               SJW           := { 1..4 }
               RESTART-MS    := { 0 | NUMBER }


```
```

    $ ip -details -statistics link show can0
    2: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 16 qdisc pfifo_fast state UP qlen 10
      link/can
      can <TRIPLE-SAMPLING> state ERROR-ACTIVE restart-ms 100
      bitrate 125000 sample_point 0.875
      tq 125 prop-seg 6 phase-seg1 7 phase-seg2 2 sjw 1
      sja1000: tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1
      clock 8000000
      re-started bus-errors arbit-lost error-warn error-pass bus-off
      41         17457      0          41         42         41
      RX: bytes  packets  errors  dropped overrun mcast
      140859     17608    17457   0       0       0
      TX: bytes  packets  errors  dropped carrier collsns
      861        112      0       41      0       0


```
涓婅堪杈撳嚭鐨勬洿澶氫俊鎭細

"<TRIPLE-SAMPLING>"
	鏄剧ず鎵€閫?CAN 鎺у埗鍣ㄦā寮忕殑鍒楄〃锛歀OOPBACK銆丩ISTEN-ONLY 鎴?TRIPLE-SAMPLING銆?

"state ERROR-ACTIVE"
	CAN 鎺у埗鍣ㄧ殑褰撳墠鐘舵€侊細"ERROR-ACTIVE"銆?ERROR-WARNING"銆?ERROR-PASSIVE"銆?BUS-OFF" 鎴?"STOPPED"銆?

"restart-ms 100"
	鑷姩閲嶅惎寤惰繜鏃堕棿銆傚鏋滆缃负闈為浂鍊硷紝鍒欏湪鍙戠敓鎬荤嚎鍏抽棴锛坆us-off锛夋儏鍐垫椂锛屼細鍦ㄦ寚瀹氱殑寤惰繜鏃堕棿锛堟绉掞級涔嬪悗鑷姩瑙﹀彂 CAN 鎺у埗鍣ㄧ殑閲嶅惎銆傞粯璁ゆ槸鍏抽棴鐨勩€?

"bitrate 125000 sample-point 0.875"
	鏄剧ず鐪熷疄鐨勬瘮鐗圭巼锛堝崟浣?bit/sec锛夊拰閲囨牱鐐癸紙鑼冨洿 0.000..0.999锛夈€傚鏋滃唴鏍稿惎鐢ㄤ簡浣嶆椂搴忓弬鏁扮殑璁＄畻锛圕ONFIG_CAN_CALC_BITTIMING=y锛夛紝鍒欏彲浠ラ€氳繃璁剧疆 "bitrate" 鍙傛暟鏉ュ畾涔変綅鏃跺簭銆傚彲閫夊湴涔熷彲浠ユ寚瀹?"sample-point"銆傞粯璁ゆ槸 0.000锛屽亣瀹氶噰鐢?CIA 鎺ㄨ崘鐨勯噰鏍风偣銆?

"tq 125 prop-seg 6 phase-seg1 7 phase-seg2 2 sjw 1"
	鏄剧ず鏃堕棿閲忓瓙锛坱q锛屽崟浣?ns锛夈€佷紶鎾銆佺浉浣嶇紦鍐叉 1 鍜?2锛屼互鍙婂悓姝ヨ烦杞搴︼紙鍗曚綅涔熸槸 tq锛夈€傚畠浠厑璁镐互涓庣‖浠舵棤鍏崇殑鏍煎紡瀹氫箟 CAN 浣嶆椂搴忥紝姝ｅ Bosch CAN 2.0 瑙勮寖鎵€寤鸿鐨勯偅鏍凤紙瑙?http://www.semiconductors.bosch.de/pdf/can2spec.pdf 鐨勭 8 绔狅級銆?

"sja1000: tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1 clock 8000000"
	鏄剧ず CAN 鎺у埗鍣紙姝ゅ涓?"sja1000"锛夌殑浣嶆椂搴忓父閲忋€傚寘鎷椂闂存鏃堕棿 1 鍜?2 鐨勬渶灏忓€煎拰鏈€澶у€笺€佸悓姝ヨ烦杞搴︼紙鍗曚綅 tq锛夈€佷綅鐜囬鍒嗛鍣紝浠ュ強 CAN 绯荤粺鏃堕挓棰戠巼锛堝崟浣?Hz锛夈€傝繖浜涘父閲忓彲鐢ㄤ簬鐢ㄦ埛绌洪棿涓敤鎴疯嚜瀹氫箟鐨勶紙闈炴爣鍑嗭級浣嶆椂搴忚绠楃畻娉曘€?

"re-started bus-errors arbit-lost error-warn error-pass bus-off"
	鏄剧ず閲嶅惎娆℃暟銆佹€荤嚎閿欒涓庝徊瑁佷涪澶遍敊璇鏁帮紝浠ュ強鍒伴敊璇鍛娿€侀敊璇鍔ㄥ拰鎬荤嚎鍏抽棴鐘舵€佺殑杞崲娆℃暟銆俁X 婧㈠嚭閿欒鍒楀湪鏍囧噯缃戠粶缁熻鐨?"overrun" 瀛楁涓€?

#### 璁剧疆 CAN 浣嶆椂搴?


CAN 浣嶆椂搴忓弬鏁板缁堝彲浠ヤ互涓庣‖浠舵棤鍏崇殑鏍煎紡瀹氫箟锛屾濡?Bosch CAN 2.0 瑙勮寖鎵€寤鸿鐨勶紝閫氳繃鎸囧畾 "tq"銆?prop_seg"銆?phase_seg1"銆?phase_seg2" 鍙傛暟锛?
```

    $ ip link set canX type can tq 125 prop-seg 6 \
				phase-seg1 7 phase-seg2 2 sjw 1

```
濡傛灉鍚敤浜嗗唴鏍搁€夐」 CONFIG_CAN_CALC_BITTIMING锛屽垯鍦ㄨ缃瘮鐗圭巼鏃朵細璁＄畻 CIA 鎺ㄨ崘鐨?CAN 浣嶆椂搴忓弬鏁帮細
```

    $ ip link set canX type can bitrate 125000

```
璇锋敞鎰忥紝杩欏澶у鏁板甫鏈夋爣鍑嗘瘮鐗圭巼鐨勫父瑙?CAN 鎺у埗鍣ㄩ兘鑳芥甯稿伐浣滐紝浣嗗浜庣壒娈婄殑姣旂壒鐜囨垨 CAN 绯荤粺鏃堕挓棰戠巼鍙兘浼?*澶辫触**銆傜鐢?CONFIG_CAN_CALC_BITTIMING 鍙互鑺傜渷涓€浜涚┖闂达紝骞跺厑璁哥敤鎴风┖闂村伐鍏风嫭绔嬪湴纭畾鍜岃缃綅鏃跺簭鍙傛暟銆侰AN 鎺у埗鍣ㄧ壒鏈夌殑浣嶆椂搴忓父閲忓彲鐢ㄤ簬姝ょ洰鐨勩€傚畠浠敱浠ヤ笅鍛戒护鍒楀嚭锛?
```

    $ ip -details link show can0
    ...
      sja1000: clock 8000000 tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1


```
#### 鍚姩鍜屽仠姝?CAN 缃戠粶璁惧


CAN 缃戠粶璁惧鐨勫惎鍔ㄦ垨鍋滄涓庨€氬父涓€鏍凤紝浣跨敤鍛戒护 "ifconfig canX up/down" 鎴?"ip link set canX up/down"銆傝娉ㄦ剰锛屽浜庣湡瀹炵殑 CAN 璁惧锛屼綘**蹇呴』**瀹氫箟姝ｇ‘鐨勪綅鏃跺簭鍙傛暟锛?
```

    $ ip link set canX up type can bitrate 125000

```
濡傛灉 CAN 鎬荤嚎涓婂彂鐢熶簡杩囧閿欒锛岃澶囧彲鑳戒細杩涘叆 "bus-off" 鐘舵€併€傛鏃朵笉鍐嶆敹鍙戜换浣曟秷鎭€傚彲浠ラ€氳繃璁剧疆 "restart-ms" 鏉ュ惎鐢ㄨ嚜鍔ㄧ殑鎬荤嚎鍏抽棴鎭㈠锛?
```

    $ ip link set canX type can restart-ms 100

```
鍙﹀锛屽簲鐢ㄧ▼搴忎篃鍙互閫氳繃鐩戞帶 CAN 閿欒娑堟伅甯ф潵鎰忚瘑鍒?"bus-off" 鐘舵€侊紝骞跺湪鍙戠敓璇ョ姸鎬佹椂杩涜閲嶅惎锛?
```

    $ ip link set canX type can restart

```
璇锋敞鎰忥紝涓€娆￠噸鍚篃浼氬垱寤轰竴涓?CAN 閿欒娑堟伅甯э紙鍙﹁ socketcan-network-problem-notifications锛夈€?



### CAN FD锛堢伒娲绘暟鎹€熺巼锛夐┍鍔ㄦ敮鎸?


鏀寔 CAN FD 鐨?CAN 鎺у埗鍣ㄤ负 CAN FD 甯х殑浠茶闃舵鍜岃浇鑽烽樁娈垫敮鎸佷袱绉嶄笉鍚岀殑姣旂壒鐜囥€傚洜姝わ紝蹇呴』鎸囧畾绗簩涓綅鏃跺簭鎵嶈兘鍚敤 CAN FD 姣旂壒鐜囥€?

姝ゅ锛屾敮鎸?CAN FD 鐨?CAN 鎺у埗鍣ㄦ敮鎸佹渶澶?64 瀛楄妭鐨勮浇鑽枫€傝繖绉嶉暱搴﹀湪 can_frame.len 鍜?canfd_frame.len 涓紝瀵逛簬鐢ㄦ埛绌洪棿搴旂敤绋嬪簭浠ュ強 Linux 缃戠粶灞傚唴閮紝鏄竴涓粠 0 鍒?64 鐨勬櫘閫氭暟鍊硷紝鑰屼笉鏄寖鍥翠粠 0 鍒?8 鐨勭粡鍏?CAN 闀垮害銆傝浇鑽烽暱搴﹀埌涓庢€荤嚎鐩稿叧鐨?DLC 鐨勬槧灏勫彧鍦?CAN 椹卞姩鍐呴儴鎵ц锛屾渶濂戒娇鐢ㄨ緟鍔╁嚱鏁?can_fd_dlc2len() 鍜?can_fd_len2dlc()銆?

CAN 缃戠粶璁惧椹卞姩鐨勮兘鍔涘彲浠ラ€氳繃缃戠粶鏉ュ尯鍒嗭細
```

  MTU = 16 (CAN_MTU)   => sizeof(struct can_frame)   => 缁忓吀 CAN 璁惧
  MTU = 72 (CANFD_MTU) => sizeof(struct canfd_frame) => CAN FD 鑳藉姏璁惧

```
CAN 璁惧 MTU 鍙互閫氳繃渚嬪 SIOCGIFMTU ioctl() 绯荤粺璋冪敤鑾峰彇銆傛敞鎰忥細鏀寔 CAN FD 鐨勮澶囦篃鍙互澶勭悊鍜屽彂閫佺粡鍏?CAN 甯с€?

鍦ㄩ厤缃敮鎸?CAN FD 鐨?CAN 鎺у埗鍣ㄦ椂锛屽繀椤昏缃竴涓澶栫殑 'data'锛堟暟鎹級姣旂壒鐜囥€侰AN FD 甯ф暟鎹樁娈电殑杩欎釜姣旂壒鐜囧繀椤昏嚦灏戠瓑浜庝负浠茶闃舵閰嶇疆鐨勬瘮鐗圭巼銆傝繖绗簩涓瘮鐗圭巼鐨勬寚瀹氭柟寮忎笌绗竴涓被浼硷紝浣?'data' 姣旂壒鐜囩殑璁剧疆鍏抽敭瀛椾互 'd' 寮€澶达紝渚嬪 dbitrate銆乨sample-point銆乨sjw 鎴?dtq 浠ュ強绫讳技鐨勮缃€傚湪閰嶇疆杩囩▼涓缃簡鏁版嵁姣旂壒鐜囨椂锛屽彲浠ユ寚瀹氭帶鍒跺櫒閫夐」 "fd on" 鏉ュ湪 CAN 鎺у埗鍣ㄤ腑鍚敤 CAN FD 妯″紡銆傝鎺у埗鍣ㄩ€夐」鍚屾椂浼氬皢璁惧 MTU 鍒囨崲涓?72锛圕ANFD_MTU锛夈€?

褰撳墠鐨?CAN FD 瑙勮寖鍦?2012 骞村浗闄?CAN 澶т細涓婁互鐧界毊涔﹀舰寮忛娆℃彁鍑猴紝鍑轰簬鏁版嵁瀹屾暣鎬у師鍥犻渶瑕佹敼杩涖€傚洜姝わ紝濡備粖蹇呴』鍖哄垎涓ょ CAN FD 瀹炵幇锛?

- ISO 鍏煎锛?    ISO 11898-1:2015 鐨?CAN FD 瀹炵幇锛堥粯璁わ級
- 闈?ISO 鍏煎锛?閬靛惊 2012 骞寸櫧鐨功鐨?CAN FD 瀹炵幇

鏈€缁堟湁涓夌被 CAN FD 鎺у埗鍣細

1. ISO 鍏煎锛堝浐瀹氾級
2. 闈?ISO 鍏煎锛堝浐瀹氾紝渚嬪 m_can.c 涓殑 M_CAN IP 鏍?v3.0.1锛?
3. ISO/闈?ISO CAN FD 鎺у埗鍣紙鍙垏鎹紝渚嬪 PEAK PCAN-USB FD锛?

褰撳墠鐨?ISO/闈?ISO 妯″紡鐢?CAN 鎺у埗鍣ㄩ┍鍔ㄩ€氳繃 netlink 鍏竷锛屽苟鐢?'ip' 宸ュ叿鏄剧ず锛堟帶鍒跺櫒閫夐」 FD-NON-ISO锛夈€侷SO/闈?ISO 妯″紡鍙兘閫氳繃涓哄彲鍒囨崲鐨?CAN FD 鎺у埗鍣ㄨ缃?'fd-non-iso {on|off}' 鏉ユ敼鍙樸€?

```

    $ ip link set can0 up type can bitrate 500000 sample-point 0.75 \
                                   dbitrate 4000000 dsample-point 0.8 fd on
    $ ip -details link show can0
    5: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 72 qdisc pfifo_fast state UNKNOWN \
             mode DEFAULT group default qlen 10
    link/can  promiscuity 0
    can <FD> state ERROR-ACTIVE (berr-counter tx 0 rx 0) restart-ms 0
          bitrate 500000 sample-point 0.750
          tq 50 prop-seg 14 phase-seg1 15 phase-seg2 10 sjw 1
          pcan_usb_pro_fd: tseg1 1..64 tseg2 1..16 sjw 1..16 brp 1..1024 \
          brp-inc 1
          dbitrate 4000000 dsample-point 0.800
          dtq 12 dprop-seg 7 dphase-seg1 8 dphase-seg2 4 dsjw 1
          pcan_usb_pro_fd: dtseg1 1..16 dtseg2 1..8 dsjw 1..4 dbrp 1..1024 \
          dbrp-inc 1
          clock 80000000


```
```

   can <FD,FD-NON-ISO> state ERROR-ACTIVE (berr-counter tx 0 rx 0) restart-ms 0


```
#### 鍙戦€佸櫒寤惰繜琛ュ伩


鍦ㄩ珮浣嶉€熺巼涓嬶紝浠庢敹鍙戝櫒 TX 寮曡剼鍒?RX 寮曡剼鐨勪紶鎾欢杩熷彲鑳藉彉寰楀ぇ浜庡疄闄呬綅鏃堕棿锛屼粠鑰屽鑷存祴閲忛敊璇細RX 寮曡剼浠嶄細鍦ㄦ祴閲忎笂涓€涓綅銆?

鍙戦€佸櫒寤惰繜琛ュ伩锛堟鍚庣О TDC锛夐€氳繃寮曞叆涓€涓绾ч噰鏍风偣锛圫SP锛夋潵瑙ｅ喅姝ら棶棰橈紝璇ラ噰鏍风偣绛変簬浠?TX 寮曡剼涓婁綅鏃堕棿寮€濮嬪埌 RX 寮曡剼涓婂疄闄呮祴閲忎箣闂寸殑銆佷互鏈€灏忔椂闂撮噺瀛愪负鍗曚綅鐨勮窛绂汇€係SP 璁＄畻涓轰袱涓彲閰嶇疆鍊间箣鍜岋細TDC 鍊硷紙TDCV锛夊拰 TDC 鍋忕Щ锛圱DCO锛夈€?

濡傛灉璁惧鏀寔锛孴DC 鍙互涓?CAN-FD 涓€璧蜂娇鐢?ip 宸ュ叿鐨?"tdc-mode" 鍙傛暟杩涜閰嶇疆锛屽涓嬫墍绀猴細

**omitted**
	褰撲笉鎻愪緵 "tdc-mode" 閫夐」鏃讹紝鍐呮牳灏嗚嚜鍔ㄥ喅瀹氭槸鍚﹀簲鎵撳紑 TDC锛屽湪杩欑鎯呭喌涓嬪畠灏嗚绠椾竴涓粯璁ょ殑 TDCO 骞朵娇鐢ㄨ澶囨祴寰楃殑 TDCV銆傝繖鏄娇鐢?TDC 鐨勬帹鑽愭柟娉曘€?

**"tdc-mode off"**
	TDC 琚樉寮忕鐢ㄣ€?

**"tdc-mode auto"**
	鐢ㄦ埛蹇呴』鎻愪緵 "tdco" 鍙傛暟銆俆DCV 灏嗙敱璁惧鑷姩璁＄畻銆傛閫夐」浠呭湪璁惧鏀寔 TDC-AUTO CAN 鎺у埗鍣ㄦā寮忔椂鎵嶅彲鐢ㄣ€?

**"tdc-mode manual"**
	鐢ㄦ埛蹇呴』鍚屾椂鎻愪緵 "tdco" 鍜?"tdcv" 鍙傛暟銆傛閫夐」浠呭湪璁惧鏀寔 TDC-MANUAL CAN 鎺у埗鍣ㄦā寮忔椂鎵嶅彲鐢ㄣ€?

璇锋敞鎰忥紝鏌愪簺璁惧鍙兘鎻愪緵棰濆鐨勫弬鏁帮細"tdcf"锛圱DC 婊ゆ尝绐楀彛锛夈€傚鏋滄偍鐨勮澶囨敮鎸侊紝鍙互灏嗗叾浣滀负鍙€夊弬鏁版坊鍔犲埌 "tdc-mode auto" 鎴?"tdc-mode manual" 涓€?

閰嶇疆 500 kbit/s 浠茶姣旂壒鐜囥€? Mbit/s 鏁版嵁姣旂壒鐜囥€乀DCO 涓?15 涓渶灏忔椂闂撮噺瀛愪互鍙婅嚜鍔ㄦ祴閲忕殑 TDCV 鐨勭ず渚嬶細
```

    $ ip link set can0 up type can bitrate 500000 \
                                   fd on dbitrate 4000000 \
				   tdc-mode auto tdco 15
    $ ip -details link show can0
    5: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 72 qdisc pfifo_fast state UP \
             mode DEFAULT group default qlen 10
        link/can  promiscuity 0 allmulti 0 minmtu 72 maxmtu 72
        can <FD,TDC-AUTO> state ERROR-ACTIVE restart-ms 0
          bitrate 500000 sample-point 0.875
          tq 12 prop-seg 69 phase-seg1 70 phase-seg2 20 sjw 10 brp 1
          ES582.1/ES584.1: tseg1 2..256 tseg2 2..128 sjw 1..128 brp 1..512 \
          brp_inc 1
          dbitrate 4000000 dsample-point 0.750
          dtq 12 dprop-seg 7 dphase-seg1 7 dphase-seg2 5 dsjw 2 dbrp 1
          tdco 15 tdcf 0
          ES582.1/ES584.1: dtseg1 2..32 dtseg2 1..16 dsjw 1..8 dbrp 1..32 \
          dbrp_inc 1
          tdco 0..127 tdcf 0..127
          clock 80000000


```
### 鏀寔鐨?CAN 纭欢


璇锋鏌?"drivers/net/can" 涓殑 "Kconfig" 鏂囦欢浠ヨ幏鍙栧綋鍓嶆敮鎸佺殑 CAN 纭欢鍒楄〃銆傚湪 SocketCAN 椤圭洰缃戠珯锛堣 socketcan-resources锛変笂鍙兘鏈夋洿澶氶┍鍔ㄥ彲鐢紝鍖呮嫭杈冩棫鐨勫唴鏍哥増鏈€?


## SocketCAN 璧勬簮


Linux CAN / SocketCAN 椤圭洰璧勬簮锛堥」鐩珯鐐?/ 閭欢鍒楄〃锛夊湪 Linux 婧愮爜鏍戠殑 MAINTAINERS 鏂囦欢涓湁寮曠敤銆傛悳绱?CAN NETWORK [LAYERS|DRIVERS]銆?

## 鑷磋阿


- Oliver Hartkopp锛圥F_CAN 鏍稿績銆佽繃婊ゅ櫒銆侀┍鍔ㄣ€乥cm銆丼JA1000 椹卞姩锛?
- Urs Thuermann锛圥F_CAN 鏍稿績銆佸唴鏍搁泦鎴愩€佸鎺ュ瓧鎺ュ彛銆乺aw銆乿can锛?
- Jan Kizka锛圧T-SocketCAN 鏍稿績銆丼ocket-API 鍗忚皟锛?
- Wolfgang Grandegger锛圧T-SocketCAN 鏍稿績涓庨┍鍔ㄣ€丷aw Socket-API 璇勫銆丆AN 璁惧椹卞姩鎺ュ彛銆丮SCAN 椹卞姩锛?
- Robert Schwebel锛堣璁¤瘎瀹°€丳TXdist 闆嗘垚锛?
- Marc Kleine-Budde锛堣璁¤瘎瀹°€並ernel 2.6 娓呯悊銆侀┍鍔級
- Benedikt Spranger锛堣瘎瀹★級
- Thomas Gleixner锛圠KML 璇勫銆佷唬鐮侀鏍笺€佸彂甯冩彁绀猴級
- Andrey Volkov锛堝唴鏍稿瓙鏍戠粨鏋勩€乮octls銆丮SCAN 椹卞姩锛?
- Matthias Brukner锛堥涓?SJA1000 CAN 缃戠粶璁惧瀹炵幇锛?003 骞寸浜屽搴︼級
- Klaus Hitschler锛圥EAK 椹卞姩闆嗘垚锛?
- Uwe Koppe锛堥噰鐢?PF_PACKET 鏂瑰紡鐨?CAN 缃戠粶璁惧锛?
- Michael Schulze锛堥┍鍔ㄥ眰鍥炵幆闇€姹傘€丷T CAN 椹卞姩璇勫锛?
- Pavel Pisa锛堜綅鏃跺簭璁＄畻锛?
- Sascha Hauer锛圫JA1000 骞冲彴椹卞姩锛?
- Sebastian Haas锛圫JA1000 EMS PCI 椹卞姩锛?
- Markus Plessing锛圫JA1000 EMS PCI 椹卞姩锛?
- Per Dalen锛圫JA1000 Kvaser PCI 椹卞姩锛?
- Sam Ravnborg锛堣瘎瀹°€佷唬鐮侀鏍笺€乲build 甯姪锛?
