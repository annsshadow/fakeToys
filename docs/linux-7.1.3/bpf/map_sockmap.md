
## BPF_MAP_TYPE_SOCKMAP 涓?BPF_MAP_TYPE_SOCKHASH


   - `BPF_MAP_TYPE_SOCKMAP` 浜庡唴鏍哥増鏈?4.14 寮曞叆
   - `BPF_MAP_TYPE_SOCKHASH` 浜庡唴鏍哥増鏈?4.18 寮曞叆

`BPF_MAP_TYPE_SOCKMAP` 涓?`BPF_MAP_TYPE_SOCKHASH` 绫诲瀷鐨勬槧灏勫彲鐢ㄤ簬鍦ㄥ鎺ュ瓧涔嬮棿閲嶅畾鍚?skb锛堟暟鎹?鍖咃級锛屾垨鍊熷姪 BPF 杈呭姪鍑芥暟 `bpf_sk_redirect_map()`銆乣bpf_sk_redirect_hash()`銆?`bpf_msg_redirect_map()` 涓?`bpf_msg_redirect_hash()`锛屽熀浜?BPF锛坴erdict锛岃鍐筹級绋嬪簭鐨勮繍琛岀粨鏋滃湪濂楁帴瀛?灞傜骇鏂藉姞绛栫暐銆?
`BPF_MAP_TYPE_SOCKMAP` 搴曞眰鏄竴涓暟缁勶紝浣跨敤鏁存暟閿綔涓虹储寮曟潵鏌ユ壘瀵?`struct sock` 鐨勫紩鐢ㄣ€傝鏄犲皠鐨?鍊煎嵆涓哄鎺ュ瓧鎻忚堪绗︺€傜被浼煎湴锛宍BPF_MAP_TYPE_SOCKHASH` 鏄竴绉嶄互鍝堝笇涓哄簳灞傛敮鎸佺殑 BPF 鏄犲皠锛屽畠閫氳繃
濂楁帴瀛楁弿杩扮鎸佹湁瀵瑰鎺ュ瓧鐨勫紩鐢ㄣ€?
    鍊肩殑绫诲瀷涓?__u32 鎴?__u64锛涘悗鑰咃紙__u64锛夌敤浜庢敮鎸佸悜鐢ㄦ埛绌洪棿杩斿洖濂楁帴瀛?cookie銆傚皢鏄犲皠鎸佹湁鐨?    `struct sock *` 杩斿洖缁欑敤鎴风┖闂存棦涓嶅畨鍏ㄤ篃鏃犵敤澶勩€?
杩欎簺鏄犲皠鍙互闄勫姞 BPF 绋嬪簭锛屽叿浣撹€岃█鏄竴涓В鏋愶紙parser锛夌▼搴忓拰涓€涓鍐筹紙verdict锛夌▼搴忋€傝В鏋愮▼搴?鍐冲畾宸茶В鏋愮殑鏁版嵁閲忥紝浠庤€屽喅瀹氶渶瑕佹帓闃熷灏戞暟鎹墠鑳藉緱鍑鸿鍐炽€傝鍐崇▼搴忔湰璐ㄤ笂灏辨槸閲嶅畾鍚戠▼搴忥紝鍙互
杩斿洖 `__SK_DROP`銆乣__SK_PASS` 鎴?`__SK_REDIRECT` 杩欐牱鐨勮鍐崇粨鏋溿€?
褰撲竴涓鎺ュ瓧琚彃鍏ュ埌杩欎簺鏄犲皠涔嬩竴鏃讹紝瀹冪殑濂楁帴瀛楀洖璋冧細琚浛鎹紝骞朵笖浼氫负鍏堕檮鍔犱竴涓?`struct sk_psock`銆?姝ゅ锛岃繖涓?`sk_psock` 浼氱户鎵块檮鍔犲埌璇ユ槧灏勪笂鐨勭▼搴忋€?
涓€涓?sock 瀵硅薄鍙互瀛樺湪浜庡涓槧灏勪腑锛屼絾鍙兘缁ф壙鍗曚竴鐨勮В鏋愮▼搴忔垨瑁佸喅绋嬪簭銆傚鏋滃皢涓€涓?sock 瀵硅薄
鍔犲叆鏌愪釜鏄犲皠浼氬鑷村嚭鐜板涓В鏋愮▼搴忥紝鍒欒鏇存柊浼氳繑鍥?EBUSY 閿欒銆?
鍙互鍚戣繖浜涙槧灏勯檮鍔犵殑鍙楁敮鎸佺▼搴忓涓嬶細


	struct sk_psock_progs {
		struct bpf_prog *msg_parser;
		struct bpf_prog *stream_parser;
		struct bpf_prog *stream_verdict;
		struct bpf_prog	*skb_verdict;
	};

    涓嶅厑璁稿皢 `stream_verdict` 涓?`skb_verdict` 绋嬪簭闄勫姞鍒板悓涓€涓槧灏勩€?
杩欎簺鏄犲皠鐨勭▼搴忛檮鍔犵被鍨嬪涓嬶細

- `msg_parser` 绋嬪簭 - `BPF_SK_MSG_VERDICT`銆?- `stream_parser` 绋嬪簭 - `BPF_SK_SKB_STREAM_PARSER`銆?- `stream_verdict` 绋嬪簭 - `BPF_SK_SKB_STREAM_VERDICT`銆?- `skb_verdict` 绋嬪簭 - `BPF_SK_SKB_VERDICT`銆?
瑙ｆ瀽绋嬪簭涓庤鍐崇▼搴忚繕鍙娇鐢ㄩ澶栫殑杈呭姪鍑芥暟锛歚bpf_msg_apply_bytes()` 涓?`bpf_msg_cork_bytes()`銆傚€熷姪 `bpf_msg_apply_bytes()`锛孊PF 绋嬪簭鍙互鍛婄煡鍩虹璁炬柦缁欏畾鐨勮鍐冲簲
浣滅敤澶氬皯瀛楄妭銆傝緟鍔╁嚱鏁?`bpf_msg_cork_bytes()` 澶勭悊鍙︿竴绉嶆儏鍐碉細BPF 绋嬪簭鍦ㄦ敹鍒版洿澶氬瓧鑺備箣鍓嶆棤娉曞
鏌愭潯 msg 鍋氬嚭瑁佸喅锛屼笖鍦ㄨ msg 琚‘璁ゆ棤璇箣鍓嶄笉甯屾湜杞彂璇ユ暟鎹寘銆?
鏈€鍚庯紝杈呭姪鍑芥暟 `bpf_msg_pull_data()` 涓?`bpf_msg_push_data()` 鍙緵
`BPF_PROG_TYPE_SK_MSG` 绫诲瀷鐨?BPF 绋嬪簭浣跨敤锛岀敤浜庢媺鍏ユ暟鎹苟灏嗚捣濮嬩笌缁撴潫鎸囬拡璁剧疆涓虹粰瀹氬€硷紝鎴栧悜
``struct sk_msg_buff *msg`` 杩藉姞鍏冩暟鎹€?
浠ヤ笂鎵€鏈夎緟鍔╁嚱鏁板皢鍦ㄦ鍚庨€愪竴璇︾粏璇存槑銆?
## 鐢ㄦ硶

### 鍐呮牳 BPF

##### bpf_msg_redirect_map()


	long bpf_msg_redirect_map(struct sk_msg_buff **msg, struct bpf_map **map, u32 key, u64 flags)

璇ヨ緟鍔╁嚱鏁扮敤浜庡疄鐜板鎺ュ瓧灞傜骇鐨勭瓥鐣ャ€傚鏋滄秷鎭?`msg` 琚厑璁搁€氳繃锛堝嵆瑁佸喅 BPF 绋嬪簭杩斿洖
`SK_PASS`锛夛紝鍒欏皢鍏堕噸瀹氬悜鍒?`map`锛堢被鍨嬩负 `BPF_MAP_TYPE_SOCKMAP`锛変腑绱㈠紩 `key` 鎵€寮曠敤鐨勯偅涓?濂楁帴瀛椼€傚叆鍙ｏ紙ingress锛変笌鍑哄彛锛坋gress锛夋帴鍙ｅ潎鍙敤浜庨噸瀹氬悜銆俙flags` 涓殑 `BPF_F_INGRESS` 鍊?鐢ㄤ簬閫夋嫨鍏ュ彛璺緞锛屽惁鍒欓€夋嫨鍑哄彛璺緞銆傝繖鏄洰鍓嶅敮涓€鍙楁敮鎸佺殑鏍囧織銆?
鎴愬姛鏃惰繑鍥?`SK_PASS`锛屽嚭閿欐椂杩斿洖 `SK_DROP`銆?
##### bpf_sk_redirect_map()


    long bpf_sk_redirect_map(struct sk_buff **skb, struct bpf_map **map, u32 key u64 flags)

灏嗘暟鎹寘閲嶅畾鍚戝埌 `map`锛堢被鍨嬩负 `BPF_MAP_TYPE_SOCKMAP`锛変腑绱㈠紩 `key` 鎵€寮曠敤鐨勯偅涓鎺ュ瓧銆傚叆鍙ｄ笌
鍑哄彛鎺ュ彛鍧囧彲鐢ㄤ簬閲嶅畾鍚戙€俙flags` 涓殑 `BPF_F_INGRESS` 鍊肩敤浜庨€夋嫨鍏ュ彛璺緞锛屽惁鍒欓€夋嫨鍑哄彛璺緞銆傝繖鏄?鐩墠鍞竴鍙楁敮鎸佺殑鏍囧織銆?
鎴愬姛鏃惰繑鍥?`SK_PASS`锛屽嚭閿欐椂杩斿洖 `SK_DROP`銆?
##### bpf_map_lookup_elem()


    void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

绫诲瀷涓?`struct sock *` 鐨勫鎺ュ瓧鏉＄洰鍙€氳繃 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鍙栧嚭銆?
##### bpf_sock_map_update()


    long bpf_sock_map_update(struct bpf_sock_ops **skops, struct bpf_map **map, void *key, u64 flags)

鍚戜竴涓紩鐢ㄥ鎺ュ瓧鐨?`map` 娣诲姞鏉＄洰锛屾垨鏇存柊鍏朵腑鐨勬潯鐩€俙skops` 琚敤浣滀笌 `key` 鐩稿叧鑱旀潯鐩殑鏂板€笺€?`flags` 鍙傛暟鍙互鏄互涓嬩箣涓€锛?
- `BPF_ANY`锛氬垱寤轰竴涓柊鍏冪礌鎴栨洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?- `BPF_NOEXIST`锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓涓€涓柊鍏冪礌銆?- `BPF_EXIST`锛氭洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?
濡傛灉璇?`map` 甯︽湁 BPF 绋嬪簭锛堣В鏋愮▼搴忎笌瑁佸喅绋嬪簭锛夛紝杩欎簺绋嬪簭浼氳姝ｅ湪娣诲姞鐨勫鎺ュ瓧鎵€缁ф壙銆傚鏋滆
濂楁帴瀛楀凡缁忛檮鍔犱簡 BPF 绋嬪簭锛屽垯浼氬鑷撮敊璇€?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
##### bpf_sock_hash_update()


    long bpf_sock_hash_update(struct bpf_sock_ops **skops, struct bpf_map **map, void *key, u64 flags)

鍚戜竴涓紩鐢ㄥ鎺ュ瓧鐨?sockhash `map` 娣诲姞鏉＄洰锛屾垨鏇存柊鍏朵腑鐨勬潯鐩€俙skops` 琚敤浣滀笌 `key` 鐩稿叧鑱?鏉＄洰鐨勬柊鍊笺€?
`flags` 鍙傛暟鍙互鏄互涓嬩箣涓€锛?
- `BPF_ANY`锛氬垱寤轰竴涓柊鍏冪礌鎴栨洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?- `BPF_NOEXIST`锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓涓€涓柊鍏冪礌銆?- `BPF_EXIST`锛氭洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?
濡傛灉璇?`map` 甯︽湁 BPF 绋嬪簭锛堣В鏋愮▼搴忎笌瑁佸喅绋嬪簭锛夛紝杩欎簺绋嬪簭浼氳姝ｅ湪娣诲姞鐨勫鎺ュ瓧鎵€缁ф壙銆傚鏋滆
濂楁帴瀛楀凡缁忛檮鍔犱簡 BPF 绋嬪簭锛屽垯浼氬鑷撮敊璇€?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
##### bpf_msg_redirect_hash()


    long bpf_msg_redirect_hash(struct sk_msg_buff **msg, struct bpf_map **map, void *key, u64 flags)

璇ヨ緟鍔╁嚱鏁扮敤浜庡疄鐜板鎺ュ瓧灞傜骇鐨勭瓥鐣ャ€傚鏋滄秷鎭?`msg` 琚厑璁搁€氳繃锛堝嵆瑁佸喅 BPF 绋嬪簭杩斿洖
`SK_PASS`锛夛紝鍒欎娇鐢ㄥ搱甯?`key` 灏嗗叾閲嶅畾鍚戝埌 `map`锛堢被鍨嬩负 `BPF_MAP_TYPE_SOCKHASH`锛夋墍寮曠敤鐨勯偅涓?濂楁帴瀛椼€傚叆鍙ｄ笌鍑哄彛鎺ュ彛鍧囧彲鐢ㄤ簬閲嶅畾鍚戙€俙flags` 涓殑 `BPF_F_INGRESS` 鍊肩敤浜庨€夋嫨鍏ュ彛璺緞锛屽惁鍒欓€夋嫨
鍑哄彛璺緞銆傝繖鏄洰鍓嶅敮涓€鍙楁敮鎸佺殑鏍囧織銆?
鎴愬姛鏃惰繑鍥?`SK_PASS`锛屽嚭閿欐椂杩斿洖 `SK_DROP`銆?
##### bpf_sk_redirect_hash()


    long bpf_sk_redirect_hash(struct sk_buff **skb, struct bpf_map **map, void *key, u64 flags)

璇ヨ緟鍔╁嚱鏁扮敤浜庡疄鐜?skb 濂楁帴瀛楀眰绾х殑绛栫暐銆傚鏋?sk_buff `skb` 琚厑璁搁€氳繃锛堝嵆瑁佸喅 BPF 绋嬪簭杩斿洖
`SK_PASS`锛夛紝鍒欎娇鐢ㄥ搱甯?`key` 灏嗗叾閲嶅畾鍚戝埌 `map`锛堢被鍨嬩负 `BPF_MAP_TYPE_SOCKHASH`锛夋墍寮曠敤鐨勯偅涓?濂楁帴瀛椼€傚叆鍙ｄ笌鍑哄彛鎺ュ彛鍧囧彲鐢ㄤ簬閲嶅畾鍚戙€俙flags` 涓殑 `BPF_F_INGRESS` 鍊肩敤浜庨€夋嫨鍏ュ彛璺緞锛屽惁鍒欓€夋嫨
鍑哄彛璺緞銆傝繖鏄洰鍓嶅敮涓€鍙楁敮鎸佺殑鏍囧織銆?
鎴愬姛鏃惰繑鍥?`SK_PASS`锛屽嚭閿欐椂杩斿洖 `SK_DROP`銆?
##### bpf_msg_apply_bytes()


    long bpf_msg_apply_bytes(struct sk_msg_buff *msg, u32 bytes)

瀵逛簬濂楁帴瀛楃瓥鐣ワ紝灏?BPF 绋嬪簭鐨勮鍐冲簲鐢ㄥ埌娑堟伅 `msg` 鎺ヤ笅鏉ョ殑 `bytes`锛堝瓧鑺傛暟锛変笂銆備緥濡傦紝璇ヨ緟鍔╁嚱鏁?鍙敤浜庝互涓嬫儏褰細

- 鍗曟 `sendmsg()` 鎴?`sendfile()` 绯荤粺璋冪敤鍖呭惈澶氭潯閫昏緫娑堟伅锛孊PF 绋嬪簭搴斿綋璇诲彇杩欎簺娑堟伅骞朵负鍏?  鍋氬嚭瑁佸喅銆?- BPF 绋嬪簭鍙叧蹇冭鍙?`msg` 鐨勫墠 `bytes` 涓瓧鑺傘€傚鏋滄秷鎭殑璐熻浇寰堝ぇ锛岄偅涔堝嵆浣胯鍐冲凡缁忕‘瀹氾紝浠嶇劧
  涓哄叏閮ㄥ瓧鑺傚弽澶?setup 骞惰皟鐢?BPF 绋嬪簭锛屼細閫犳垚涓嶅繀瑕佺殑寮€閿€銆?
杩斿洖 0銆?
##### bpf_msg_cork_bytes()


    long bpf_msg_cork_bytes(struct sk_msg_buff *msg, u32 bytes)

瀵逛簬濂楁帴瀛楃瓥鐣ワ紝鍦ㄧ疮绉埌 `bytes` 涓瓧鑺備箣鍓嶏紝闃绘瑁佸喅 BPF 绋嬪簭瀵规秷鎭?`msg` 鐨勬墽琛屻€?
褰撻渶瑕佸湪鍋氬嚭瑁佸喅涔嬪墠鑾峰緱鐗瑰畾鏁伴噺鐨勫瓧鑺傛椂鍙互浣跨敤璇ヨ緟鍔╁嚱鏁帮紝鍗充究鏁版嵁璺ㄨ秺浜嗗娆?`sendmsg()` 鎴?`sendfile()` 璋冪敤銆?
杩斿洖 0銆?
##### bpf_msg_pull_data()


    long bpf_msg_pull_data(struct sk_msg_buff *msg, u32 start, u32 end, u64 flags)

瀵逛簬濂楁帴瀛楃瓥鐣ワ紝浠庣敤鎴风┖闂存媺鍏?`msg` 鐨勯潪绾挎€ф暟鎹紝骞跺皢鎸囬拡 `msg->data` 涓?`msg->data_end`
鍒嗗埆璁剧疆涓?`msg` 涓?`start` 涓?`end` 瀛楄妭鐨勫亸绉婚噺銆?
濡傛灉绫诲瀷涓?`BPF_PROG_TYPE_SK_MSG` 鐨勭▼搴忓湪 `msg` 涓婅繍琛岋紝瀹冨彧鑳借В鏋愶紙`data`锛宍data_end`锛夋寚閽?宸茬粡娑堣垂杩囩殑鏁版嵁銆傚浜?`sendmsg()` 閽╁瓙鑰岃█锛岃繖閫氬父灏辨槸绗竴涓?scatterlist 鍏冪礌銆備絾瀵逛簬渚濊禆
MSG_SPLICE_PAGES 鐨勮皟鐢紙渚嬪 `sendfile()`锛夎€岃█锛屽叾鑼冨洿灏嗘槸锛?*0**锛?*0**锛夛紝鍥犱负鏁版嵁涓庣敤鎴风┖闂?鍏变韩锛岃€岄粯璁ょ洰鏍囨槸鍦?BPF 瑁佸喅鍋氬嚭鏈熼棿锛堟垨涔嬪悗锛夐伩鍏嶅厑璁哥敤鎴风┖闂翠慨鏀规暟鎹€傝杈呭姪鍑芥暟鍙敤浜庢媺鍏?鏁版嵁骞跺皢璧峰涓庣粨鏉熸寚閽堣缃负缁欏畾鍊笺€傚繀瑕佹椂浼氬鍒舵暟鎹紙鍗冲綋鏁版嵁涓嶆槸绾挎€х殑銆佷笖璧峰涓庣粨鏉熸寚閽堜笉鎸囧悜
鍚屼竴鏁版嵁鍧楁椂锛夈€?
璋冪敤璇ヨ緟鍔╁嚱鏁板彲鑳戒細鏀瑰彉搴曞眰鐨勬暟鎹寘缂撳啿鍖恒€傚洜姝わ紝鍦ㄥ姞杞芥椂锛屾牎楠屽櫒锛坴erifier锛夋鍓嶅鎵€鏈夋寚閽?鎵€鍋氱殑妫€鏌ラ兘浼氬け鏁堬紝濡傛灉鍦ㄨ杈呭姪鍑芥暟涓庣洿鎺ユ暟鎹寘璁块棶閰嶅悎浣跨敤鏃讹紝蹇呴』閲嶆柊鎵ц杩欎簺妫€鏌ャ€?
`flags` 鐨勬墍鏈夊彇鍊奸兘淇濈暀渚涘皢鏉ヤ娇鐢紝蹇呴』淇濇寔涓洪浂銆?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
##### bpf_map_lookup_elem()




	void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

鍦?sockmap 鎴?sockhash 鏄犲皠涓煡鎵句竴涓鎺ュ瓧鏉＄洰銆?
杩斿洖涓?`key` 鐩稿叧鑱旂殑濂楁帴瀛楁潯鐩紝濡傛灉娌℃湁鎵惧埌鏉＄洰鍒欒繑鍥?NULL銆?
##### bpf_map_update_elem()


	long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

鍦?sockmap 鎴?sockhash 涓坊鍔犳垨鏇存柊涓€涓鎺ュ瓧鏉＄洰銆?
flags 鍙傛暟鍙互鏄互涓嬩箣涓€锛?
- BPF_ANY锛氬垱寤轰竴涓柊鍏冪礌鎴栨洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?- BPF_NOEXIST锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓涓€涓柊鍏冪礌銆?- BPF_EXIST锛氭洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
##### bpf_map_delete_elem()


    long bpf_map_delete_elem(struct bpf_map **map, const void **key)

浠?sockmap 鎴?sockhash 涓垹闄や竴涓鎺ュ瓧鏉＄洰銆?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
### 鐢ㄦ埛绌洪棿

##### bpf_map_update_elem()


	int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags)

鍙互浣跨敤 `bpf_map_update_elem()` 鍑芥暟娣诲姞鎴栨洿鏂?sockmap 鏉＄洰銆俙key` 鍙傛暟鏄?sockmap 鏁扮粍鐨?绱㈠紩鍊硷紝`value` 鍙傛暟鏄濂楁帴瀛楃殑 FD 鍊笺€?
鍦ㄥ簳灞傦紝sockmap 鐨勬洿鏂板嚱鏁颁細浣跨敤濂楁帴瀛?FD 鍊煎幓鍙栧嚭鐩稿叧鑱旂殑濂楁帴瀛楀強鍏堕檮鍔犵殑 psock銆?
flags 鍙傛暟鍙互鏄互涓嬩箣涓€锛?
- BPF_ANY锛氬垱寤轰竴涓柊鍏冪礌鎴栨洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?- BPF_NOEXIST锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓涓€涓柊鍏冪礌銆?- BPF_EXIST锛氭洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?
##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value)

鍙互浣跨敤 `bpf_map_lookup_elem()` 鍑芥暟鍙栧嚭 sockmap 鏉＄洰銆?
	杩斿洖鐨勬潯鐩槸涓€涓鎺ュ瓧 cookie锛岃€岄潪濂楁帴瀛楁湰韬€?
##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key)

鍙互浣跨敤 `bpf_map_delete_elem()` 鍑芥暟鍒犻櫎 sockmap 鏉＄洰銆?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖涓€涓礋鐨勯敊璇爜銆?
## 绀轰緥


### 鍐呮牳 BPF

鍏充簬 sockmap API 鐢ㄦ硶鐨勮嫢骞茬ず渚嬪彲浠ュ湪浠ヤ笅浣嶇疆鎵惧埌锛?
- `tools/testing/selftests/bpf/progs/test_sockmap_kern.h`_
- `tools/testing/selftests/bpf/progs/sockmap_parse_prog.c`_
- `tools/testing/selftests/bpf/progs/sockmap_verdict_prog.c`_
- `tools/testing/selftests/bpf/progs/test_sockmap_listen.c`_
- `tools/testing/selftests/bpf/progs/test_sockmap_update.c`_

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓?sockmap銆?

	struct {
		__uint(type, BPF_MAP_TYPE_SOCKMAP);
		__uint(max_entries, 1);
		__type(key, __u32);
		__type(value, __u64);
	} sock_map_rx SEC(".maps");

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓ず渚嬭В鏋愮▼搴忋€?

	SEC("sk_skb/stream_parser")
	int bpf_prog_parser(struct __sk_buff *skb)
	{
		return skb->len;
	}

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓畝鍗曠殑瑁佸喅绋嬪簭锛屽畠涓庝竴涓?sockmap 浜や簰锛屾牴鎹湰鍦扮鍙ｅ皢娴侀噺閲嶅畾鍚戝埌鍙︿竴涓?濂楁帴瀛椼€?

	SEC("sk_skb/stream_verdict")
	int bpf_prog_verdict(struct __sk_buff *skb)
	{
		__u32 lport = skb->local_port;
		__u32 idx = 0;

		if (lport == 10000)
			return bpf_sk_redirect_map(skb, &sock_map_rx, idx, 0);

		return SK_PASS;
	}

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓?sockhash 鏄犲皠銆?

	struct socket_key {
		__u32 src_ip;
		__u32 dst_ip;
		__u32 src_port;
		__u32 dst_port;
	};

	struct {
		__uint(type, BPF_MAP_TYPE_SOCKHASH);
		__uint(max_entries, 1);
		__type(key, struct socket_key);
		__type(value, __u64);
	} sock_hash_rx SEC(".maps");

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓畝鍗曠殑瑁佸喅绋嬪簭锛屽畠涓庝竴涓?sockhash 浜や簰锛屾牴鎹?skb 鏌愪簺鍙傛暟鐨勫搱甯屽€煎皢娴侀噺
閲嶅畾鍚戝埌鍙︿竴涓鎺ュ瓧銆?

	static inline
	void extract_socket_key(struct __sk_buff **skb, struct socket_key **key)
	{
		key->src_ip = skb->remote_ip4;
		key->dst_ip = skb->local_ip4;
		key->src_port = skb->remote_port >> 16;
		key->dst_port = (bpf_htonl(skb->local_port)) >> 16;
	}

	SEC("sk_skb/stream_verdict")
	int bpf_prog_verdict(struct __sk_buff *skb)
	{
		struct socket_key key;

		extract_socket_key(skb, &key);

		return bpf_sk_redirect_hash(skb, &sock_hash_rx, &key, 0);
	}

### 鐢ㄦ埛绌洪棿

鍏充簬 sockmap API 鐢ㄦ硶鐨勮嫢骞茬ず渚嬪彲浠ュ湪浠ヤ笅浣嶇疆鎵惧埌锛?
- `tools/testing/selftests/bpf/prog_tests/sockmap_basic.c`_
- `tools/testing/selftests/bpf/test_sockmap.c`_
- `tools/testing/selftests/bpf/test_maps.c`_

浠ヤ笅浠ｇ爜绀轰緥灞曠ず浜嗗浣曞垱寤轰竴涓?sockmap銆侀檮鍔犱竴涓В鏋愮▼搴忎笌瑁佸喅绋嬪簭锛屽苟娣诲姞涓€涓鎺ュ瓧鏉＄洰銆?

	int create_sample_sockmap(int sock, int parse_prog_fd, int verdict_prog_fd)
	{
		int index = 0;
		int map, err;

		map = bpf_map_create(BPF_MAP_TYPE_SOCKMAP, NULL, sizeof(int), sizeof(int), 1, NULL);
		if (map < 0) {
			fprintf(stderr, "Failed to create sockmap: %s\n", strerror(errno));
			return -1;
		}

		err = bpf_prog_attach(parse_prog_fd, map, BPF_SK_SKB_STREAM_PARSER, 0);
		if (err){
			fprintf(stderr, "Failed to attach_parser_prog_to_map: %s\n", strerror(errno));
			goto out;
		}

		err = bpf_prog_attach(verdict_prog_fd, map, BPF_SK_SKB_STREAM_VERDICT, 0);
		if (err){
			fprintf(stderr, "Failed to attach_verdict_prog_to_map: %s\n", strerror(errno));
			goto out;
		}

		err = bpf_map_update_elem(map, &index, &sock, BPF_NOEXIST);
		if (err) {
			fprintf(stderr, "Failed to update sockmap: %s\n", strerror(errno));
			goto out;
		}

	out:
		close(map);
		return err;
	}

## 鍙傝€冭祫鏂?

- https://github.com/jrfastab/linux-kernel-xdp/commit/c89fd73cb9d2d7f3c716c3e00836f07b1aeb261f
- https://lwn.net/Articles/731133/
- http://vger.kernel.org/lpc_net2018_talks/ktls_bpf_paper.pdf
- https://lwn.net/Articles/748628/
- https://lore.kernel.org/bpf/20200218171023.844439-7-jakub@cloudflare.com/
