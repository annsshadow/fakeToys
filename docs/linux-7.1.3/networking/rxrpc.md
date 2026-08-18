
## RxRPC 缃戠粶鍗忚


RxRPC 鍗忚椹卞姩鍦?UDP 涔嬩笂鎻愪緵浜嗕竴涓彲闈犵殑涓ら樁娈典紶杈擄紝鍙敤浜庢墽琛?RxRPC 杩滅▼鎿嶄綔銆傝繖鏄€氳繃
AF_RXRPC 鏃忕殑濂楁帴瀛楋紝浣跨敤 sendmsg() 鍜?recvmsg() 閰嶅悎鎺у埗鏁版嵁鏉ュ彂閫佸拰鎺ユ敹鏁版嵁銆佷腑姝㈠拰閿欒銆?
鏈枃妗ｅ唴瀹癸細

 (#) 姒傝堪銆?
 (#) RxRPC 鍗忚鎽樿銆?
 (#) AF_RXRPC 椹卞姩妯″瀷銆?
 (#) 鎺у埗娑堟伅銆?
 (#) 濂楁帴瀛楅€夐」銆?
 (#) 瀹夊叏鎬с€?
 (#) 绀轰緥瀹㈡埛绔敤娉曘€?
 (#) 绀轰緥鏈嶅姟绔敤娉曘€?
 (#) AF_RXRPC 鍐呮牳鎺ュ彛銆?
 (#) 鍙厤缃弬鏁般€?

## 姒傝堪


RxRPC 鏄竴涓袱灞傚崗璁€傛湁涓€涓細璇濆眰锛屽畠浣跨敤 UDP over IPv4锛堟垨 IPv6锛変綔涓轰紶杈撳眰鏉ユ彁渚涘彲闈犵殑
铏氭嫙杩炴帴锛屼絾瀹炵幇鐨勬槸涓€涓湡姝ｇ殑缃戠粶鍗忚锛涜繕鏈変竴涓〃绀哄眰锛屽畠浣跨敤 XDR 鎶婄粨鏋勫寲鏁版嵁娓叉煋鎴?浜岃繘鍒跺潡锛屽啀杞崲鍥炴潵锛?
```
		+-------------+
		| Application |
		+-------------+
		|     XDR     |		Presentation
		+-------------+
		|    RxRPC    |		Session
		+-------------+
		|     UDP     |		Transport
		+-------------+
```

AF_RXRPC 鎻愪緵锛?
 (1) 涓€涓?RxRPC 璁炬柦鐨勪竴閮ㄥ垎锛屼緵鍐呮牳鍜屽簲鐢ㄧ▼搴忓悓鏃朵娇鐢紝鏂规硶鏄妸鍏朵腑鐨勪細璇濋儴鍒嗗仛鎴愪竴涓?     Linux 缃戠粶鍗忚锛圓F_RXRPC锛夈€?
 (2) 涓€涓袱闃舵鍗忚銆傚鎴风鍙戦€佷竴涓簩杩涘埗鍧楋紙璇锋眰锛夛紝鐒跺悗鎺ユ敹涓€涓簩杩涘埗鍧楋紙搴旂瓟锛夛紱鏈嶅姟绔?     鎺ユ敹璇锋眰锛岀劧鍚庡彂閫佸簲绛斻€?
 (3) 淇濈暀涓轰竴涓皟鐢ㄦ墍寤虹珛鐨勪紶杈撶郴缁熶腑鍙鐢ㄧ殑閮ㄥ垎锛屼互鍔犻€熷悗缁皟鐢ㄣ€?
 (4) 涓€涓畨鍏ㄥ崗璁紝浣跨敤 Linux 鍐呮牳鐨勫瘑閽ヤ繚鐣欒鏂芥潵鍦ㄥ鎴风绠＄悊瀹夊叏銆傛湇鍔＄绔湪鍗忓晢瀹夊叏鏃?     蹇呴』鏇翠负娲昏穬銆?
AF_RXRPC 涓嶆彁渚?XDR 缂栫粍/琛ㄧず璁炬柦銆傞偅鐣欑粰搴旂敤绋嬪簭銆侫F_RXRPC 鍙鐞嗕簩杩涘埗鍧椼€傚嵆渚挎槸鎿嶄綔 ID
涔熷彧鏄姹備簩杩涘埗鍧楃殑鍓嶅洓涓瓧鑺傦紝鍥犳瓒呭嚭浜嗗唴鏍哥殑鍏虫敞鑼冨洿銆?

AF_RXRPC 鏃忕殑濂楁帴瀛楋細

 (1) 浠ョ被鍨?SOCK_DGRAM 鍒涘缓锛?
 (2) 鎻愪緵瀹冧滑灏嗚浣跨敤鐨勫簳灞備紶杈撶被鍨嬬殑鍗忚鈥斺€旂洰鍓嶅彧鏀寔 PF_INET銆?

Andrew 鏂囦欢绯荤粺锛圓FS锛夋槸浣跨敤瀹冨苟涓斿悓鏃跺叿鏈夊唴鏍革紙鏂囦欢绯荤粺锛夊拰鐢ㄦ埛绌洪棿锛堝伐鍏凤級缁勪欢鐨勫簲鐢ㄧ▼搴忕殑
渚嬪瓙銆?

## RxRPC 鍗忚鎽樿


RxRPC 鍗忚姒傝堪锛?
 (#) RxRPC 浣嶄簬鍙︿竴涓綉缁滃崗璁箣涓婏紙鐩墠鍞竴閫夐」鏄?UDP锛夛紝骞剁敤瀹冩潵鎻愪緵缃戠粶浼犺緭銆備緥濡傦紝UDP
     绔彛鎻愪緵浼犺緭绔偣銆?
 (#) RxRPC 鏀寔鏉ヨ嚜浠讳綍缁欏畾浼犺緭绔偣鐨勫涓櫄鎷?杩炴帴"锛屼粠鑰屽厑璁哥鐐硅鍏变韩锛岀敋鑷冲叡浜埌鍚屼竴涓?     杩滅▼绔偣銆?
 (#) 姣忎釜杩炴帴閮介€氬悜涓€涓壒瀹氱殑"鏈嶅姟"銆備竴涓繛鎺ヤ笉鑳介€氬悜澶氫釜鏈嶅姟銆備竴涓湇鍔″彲浠ヨ璁や负鏄?RxRPC
     瀵圭鍙ｅ彿鐨勭瓑浠风墿銆侫F_RXRPC 鍏佽澶氫釜鏈嶅姟鍏变韩涓€涓鐐广€?
 (#) 瀹㈡埛绔彂璧风殑鏁版嵁鍖呰鏍囪锛屽洜姝や竴涓紶杈撶鐐瑰彲浠ュ湪瀹㈡埛绔繛鎺ュ拰鏈嶅姟绔繛鎺ヤ箣闂村叡浜紙杩炴帴
     鏈夋柟鍚戯級銆?
 (#) 鍦ㄤ竴涓湰鍦颁紶杈撶鐐逛笌鏌愪釜杩滅▼绔偣涓婄殑涓€涓湇鍔′箣闂达紝鍙互骞跺彂鏀寔澶氳揪鍗佷嚎涓繛鎺ャ€備竴涓?RxRPC

```
	Local address	}
	Local port	} Transport (UDP) address
	Remote address	}
	Remote port	}
	Direction
	Connection ID
	Service ID
```

 (#) 姣忎釜 RxRPC 鎿嶄綔閮芥槸涓€涓?璋冪敤"锛坈all锛夈€備竴涓繛鎺ユ渶澶氬彲浠ヨ繘琛屽洓鍗佷嚎娆¤皟鐢紝浣嗗湪浠绘剰鏃跺埢
     涓€涓繛鎺ヤ笂鏈€澶氬彧鑳芥湁鍥涙璋冪敤鍦ㄨ繘琛屻€?
 (#) 璋冪敤鏄袱闃舵涓旈潪瀵圭О鐨勶細瀹㈡埛绔彂閫佸叾璇锋眰鏁版嵁锛岀敱鏈嶅姟鎺ユ敹锛涚劧鍚庢湇鍔″彂閫佸簲绛旀暟鎹紝鐢卞鎴风
     鎺ユ敹銆?
 (#) 鏁版嵁鍧楃殑澶у皬涓嶅畾锛屼竴涓樁娈电殑缁撴潫鐢辨暟鎹寘涓殑涓€涓爣蹇楁爣璁般€傜粍鎴愪竴涓潡鐨勬暟鎹寘鏁伴噺涓嶅緱瓒呰繃
     鍥涘崄浜匡紝鍚﹀垯浼氬鑷村簭鍒楀彿鍥炵粫銆?
 (#) 璇锋眰鏁版嵁鐨勫墠鍥涗釜瀛楄妭鏄湇鍔℃搷浣?ID銆?
 (#) 瀹夊叏鏄€愯繛鎺ュ崗鍟嗙殑銆傝繛鎺ョ敱鎶佃揪鍏朵笂鐨勭涓€涓暟鎹寘鍙戣捣銆傚鏋滆姹備簡瀹夊叏锛屾湇鍔＄闅忓悗鍙戝嚭涓€涓?     "challenge"锛堣川璇級锛岀劧鍚庡鎴风鐢ㄤ竴涓?"response"锛堝搷搴旓級鍥炲銆傚鏋滃搷搴旀垚鍔燂紝璇ュ畨鍏ㄤ负姝よ繛鎺?     鐨勭敓瀛樻湡璁剧疆锛屽苟涓斿湪璇ヨ繛鎺ヤ笂杩涜鐨勫悗缁墍鏈夎皟鐢ㄩ兘浣跨敤鍚屼竴瀹夊叏銆傚鏋滄湇鍔＄鍦ㄥ鎴风涔嬪墠璁╀竴涓?     杩炴帴澶辨晥锛屽垯褰撳鎴风鍐嶆浣跨敤璇ヨ繛鎺ユ椂锛屽畨鍏ㄥ皢琚噸鏂板崗鍟嗐€?
 (#) 璋冪敤浣跨敤 ACK 鏁版嵁鍖呮潵澶勭悊鍙潬鎬с€傛暟鎹暟鎹寘鍦ㄦ瘡涓皟鐢ㄥ唴杩樿鏄惧紡鍦版帓搴忋€?
 (#) 鏈変袱绉嶇Н鏋佺‘璁わ細hard-ACK锛堢‖纭锛夊拰 soft-ACK锛堣蒋纭锛夈€俬ard-ACK 鍚戝绔〃鏄庯紝鍒版煇涓€鐐逛负姝?     鏀跺埌鐨勬墍鏈夋暟鎹兘宸茶鎺ユ敹骞跺鐞嗭紱soft-ACK 琛ㄦ槑鏁版嵁宸茶鎺ユ敹锛屼絾鍙兘浠嶄細琚涪寮冨苟閲嶆柊璇锋眰銆傚彂閫佹柟
     鍦ㄦ暟鎹寘琚?hard-ACK 涔嬪墠涓嶅緱涓㈠純浠讳綍鍙彂閫佺殑鏁版嵁鍖呫€?
 (#) 鎺ユ敹涓€涓簲绛旀暟鎹暟鎹寘浼氶殣寮忓湴瀵圭粍鎴愯姹傜殑鎵€鏈夋暟鎹暟鎹寘杩涜 hard-ACK銆?
 (#) 褰撲竴涓皟鐢ㄥ凡鍙戦€佽姹傘€佸凡鎺ユ敹搴旂瓟锛屽苟涓斿簲绛旀渶鍚庝竴涓暟鎹寘涓婄殑鏈€缁?hard-ACK 宸插埌杈炬湇鍔＄鏃讹紝
     璇ヨ皟鐢ㄥ畬鎴愩€?
 (#) 涓€涓皟鐢ㄥ湪鍏跺畬鎴愪箣鍓嶇殑浠讳綍鏃跺€欓兘鍙互琚换涓€绔腑姝€?

## AF_RXRPC 椹卞姩妯″瀷


鍏充簬 AF_RXRPC 椹卞姩锛?
 (#) AF_RXRPC 鍗忚閫忔槑鍦颁娇鐢ㄤ紶杈撳崗璁殑鍐呴儴濂楁帴瀛楁潵琛ㄧず浼犺緭绔偣銆?
 (#) AF_RXRPC 濂楁帴瀛楁槧灏勫埌 RxRPC 杩炴帴鏉熴€傚疄闄呯殑 RxRPC 杩炴帴琚€忔槑鍦板鐞嗐€備竴涓鎴风濂楁帴瀛楀彲鐢ㄤ簬
     瀵瑰悓涓€鏈嶅姟杩涜澶氫釜骞跺彂璋冪敤銆備竴涓湇鍔＄濂楁帴瀛楀彲澶勭悊鏉ヨ嚜璁稿瀹㈡埛绔殑璋冪敤銆?
 (#) 灏嗗彂璧烽澶栫殑骞惰瀹㈡埛绔繛鎺ワ紝浠ユ敮鎸侀澶栫殑骞跺彂璋冪敤锛屼笂闄愬彲璋冦€?
 (#) 姣忎釜杩炴帴鍦ㄦ渶鍚庝竴涓鍦ㄤ娇鐢ㄥ畠鐨勮皟鐢ㄥ畬鎴愪箣鍚庯紝浼氳淇濈暀涓€娈垫椂闂?[鍙皟]锛屼互闃插彲浠ュ鐢ㄥ畠鐨勬柊
     璋冪敤鍑虹幇銆?
 (#) 姣忎釜鍐呴儴 UDP 濂楁帴瀛楀湪鏈€鍚庝竴涓娇鐢ㄥ畠鐨勮繛鎺ヨ涓㈠純涔嬪悗锛屼細琚繚鐣?[鍙皟] 涓€娈垫椂闂?[鍙皟]锛屼互闃?     鍙互澶嶇敤瀹冪殑鏂拌繛鎺ュ嚭鐜般€?
 (#) 涓€涓鎴风杩炴帴鍙湁鍦ㄨ皟鐢ㄥ叿鏈夋弿杩板叾瀹夊叏鐨勭浉鍚?key 缁撴瀯浣撴椂锛屾墠浼氬湪璋冪敤涔嬮棿鍏变韩锛堝苟涓斿亣璁捐繖浜?     璋冪敤鏈潵涔熶細鍏变韩璇ヨ繛鎺ワ級銆傛湭鍔犱繚鎶ょ殑璋冪敤涔熻兘澶熷郊姝ゅ叡浜繛鎺ャ€?
 (#) 涓€涓湇鍔＄杩炴帴鐢卞鎴风璇村彲浠ュ叡浜椂鎵嶅叡浜€?
 (#) ACK锛堢‘璁わ級鐢卞崗璁┍鍔ㄨ嚜鍔ㄥ鐞嗭紝鍖呮嫭 ping 鍥炲銆?
 (#) SO_KEEPALIVE 鑷姩 ping 鍙︿竴绔互淇濇寔杩炴帴瀛樻椿 [TODO]銆?
 (#) 濡傛灉鏀跺埌涓€涓?ICMP 閿欒锛屾墍鏈夊彈璇ラ敊璇奖鍝嶇殑璋冪敤灏嗚涓锛屽苟閫氳繃 recvmsg() 浼犻€掍竴涓€傚綋鐨勭綉缁?     閿欒銆?

涓?RxRPC 濂楁帴瀛楃敤鎴风殑浜や簰锛?
 (#) 涓€涓鎺ュ瓧閫氳繃缁戝畾涓€涓叿鏈夐潪闆舵湇鍔?ID 鐨勫湴鍧€鑰屾垚涓烘湇鍔＄濂楁帴瀛椼€?
 (#) 鍦ㄥ鎴风锛屽彂閫佷竴涓姹傛槸閫氳繃涓€涓垨澶氫釜 sendmsg 瀹屾垚鐨勶紝闅忓悗閫氳繃涓€涓垨澶氫釜 recvmsg 鎺ユ敹搴旂瓟銆?
 (#) 浠庡鎴风鍙戝嚭鐨勮姹傜殑绗竴涓?sendmsg 鍖呭惈涓€涓爣璁帮紙tag锛夛紝鐢ㄤ簬涓庤璋冪敤鍏宠仈鐨勬墍鏈夊叾浠?sendmsg
     鎴?recvmsg銆傝鏍囪鎼哄甫鍦ㄦ帶鍒舵暟鎹腑銆?
 (#) connect() 鐢ㄤ簬涓哄鎴风鐨勫鎺ュ瓧鎻愪緵涓€涓粯璁ょ洰鏍囧湴鍧€銆傝繖鍙互閫氳繃缁欒皟鐢ㄧ殑绗竴涓?sendmsg() 鎻愪緵
     涓€涓鐢ㄥ湴鍧€锛坰truct msghdr::msg_name锛夋潵瑕嗙洊銆?
 (#) 濡傛灉鍦ㄦ湭缁戝畾鐨勫鎴风涓婅皟鐢?connect()锛屽湪鎿嶄綔鍙戠敓鍓嶄細缁戝畾涓€涓殢鏈虹殑鏈湴绔彛銆?
 (#) 涓€涓湇鍔＄濂楁帴瀛椾篃鍙敤浜庤繘琛屽鎴风璋冪敤銆備负姝わ紝璇ヨ皟鐢ㄧ殑绗竴涓?sendmsg() 蹇呴』鎸囧畾鐩爣鍦板潃銆?     鏈嶅姟绔殑浼犺緭绔偣鐢ㄤ簬鍙戦€佹暟鎹寘銆?
 (#) 涓€鏃﹀簲鐢ㄧ▼搴忔帴鏀朵簡涓庢煇涓皟鐢ㄥ叧鑱旂殑鏈€鍚庝竴鏉℃秷鎭紝灏变繚璇佷笉浼氬啀鐪嬪埌璇ユ爣璁帮紝鍥犳鍙互鐢ㄥ畠鏉?     鍥哄畾瀹㈡埛绔祫婧愩€傜劧鍚庡彲浠ョ敤鐩稿悓鐨勬爣璁板彂璧蜂竴涓柊璋冪敤锛岃€屼笉蹇呮媴蹇冪浉浜掑共鎵般€?
 (#) 鍦ㄦ湇鍔＄锛屼竴涓姹傞€氳繃涓€涓垨澶氫釜 recvmsg 鎺ユ敹锛岀劧鍚庡簲绛旈€氳繃涓€涓垨澶氫釜 sendmsg 鍙戦€侊紝鐒跺悗
     鏈€缁堢殑 ACK 閫氳繃涓€涓渶鍚庣殑 recvmsg 鎺ユ敹銆?
 (#) 褰撲负鏌愪釜璋冪敤鍙戦€佹暟鎹椂锛屽鏋滃湪璇ヨ皟鐢ㄤ笂杩樻湁鏇村鏁版嵁瑕佹潵锛宻endmsg 浼氳璧嬩簣 MSG_MORE銆?
 (#) 褰撲负鏌愪釜璋冪敤鎺ユ敹鏁版嵁鏃讹紝濡傛灉鍦ㄨ璋冪敤涓婅繕鏈夋洿澶氭暟鎹鏉ワ紝recvmsg 浼氭爣璁?MSG_MORE銆?
 (#) 褰撲负鏌愪釜璋冪敤鎺ユ敹鏁版嵁鎴栨秷鎭椂锛宺ecvmsg 浼氭爣璁?MSG_EOR 浠ユ寚绀鸿璋冪敤鐨勬渶缁堟秷鎭€?
 (#) 涓€涓皟鐢ㄥ彲浠ラ€氳繃鍦ㄦ帶鍒舵暟鎹腑娣诲姞涓€涓腑姝㈡帶鍒舵秷鎭潵涓銆傚彂鍑轰腑姝細缁堟鍐呮牳瀵硅璋冪敤鏍囪鐨勪娇鐢ㄣ€?     浠讳綍鍦ㄨ璋冪敤鐨勬帴鏀堕槦鍒椾腑绛夊緟鐨勬秷鎭兘灏嗚涓㈠純銆?
 (#) 涓銆佸繖閫氱煡锛坆usy notification锛夊拰璐ㄨ鏁版嵁鍖呴€氳繃 recvmsg 浼犻€掞紝骞朵笖鎺у埗鏁版嵁娑堟伅灏嗚璁剧疆浠?     鎸囩ず涓婁笅鏂囥€傛帴鏀朵竴涓腑姝㈡垨蹇欐秷鎭細缁堟鍐呮牳瀵硅璋冪敤鏍囪鐨勪娇鐢ㄣ€?
 (#) msghdr 缁撴瀯浣撶殑鎺у埗鏁版嵁閮ㄥ垎鐢ㄤ簬鑻ュ共鐢ㄩ€旓細

     (#) 鐩爣鎴栧彈褰卞搷鐨勮皟鐢ㄧ殑鏍囪銆?
     (#) 鍙戦€佹垨鎺ユ敹閿欒銆佷腑姝㈠拰蹇欓€氱煡銆?
     (#) 浼犲叆璋冪敤鐨勯€氱煡銆?
     (#) 鍙戦€佽皟璇曡姹傚拰鎺ユ敹璋冭瘯鍥炲 [TODO]銆?
 (#) 褰撳唴鏍告帴鏀跺苟寤虹珛涓€涓紶鍏ヨ皟鐢ㄦ椂锛屽畠浼氬悜鏈嶅姟绔簲鐢ㄧ▼搴忓彂閫佷竴鏉℃秷鎭紝璁╁畠鐭ラ亾鏈変竴涓柊璋冪敤鍦ㄧ瓑寰?     瀹冪殑鎺ュ彈 [recvmsg 鎶ュ憡涓€涓壒娈婄殑鎺у埗娑堟伅]銆傜劧鍚庢湇鍔＄搴旂敤绋嬪簭浣跨敤 sendmsg 涓烘柊璋冪敤鍒嗛厤涓€涓爣璁般€?     涓€鏃﹀畬鎴愶紝璇锋眰鏁版嵁鐨勭涓€閮ㄥ垎灏嗙敱 recvmsg 浼犻€掋€?
 (#) 鏈嶅姟绔簲鐢ㄧ▼搴忓繀椤诲悜鏈嶅姟绔鎺ュ瓧鎻愪緵涓€涓瘑閽ョ幆锛坘eyring锛夛紝鍏朵腑鍖呭惈涓庡叾鍏佽鐨勫畨鍏ㄧ被鍨嬪搴旂殑
     瀵嗛挜銆傚綋寤虹珛涓€涓畨鍏ㄨ繛鎺ユ椂锛屽唴鏍稿湪瀵嗛挜鐜腑鏌ユ壘閫傚綋鐨勫瘑閽ワ紝鐒跺悗鍚戝鎴风鍙戦€佷竴涓川璇㈡暟鎹寘骞?     鎺ユ敹涓€涓搷搴旀暟鎹寘銆傚唴鏍搁殢鍚庢鏌ヨ鏁版嵁鍖呯殑鎺堟潈锛岃涔堜腑姝㈣繛鎺ワ紝瑕佷箞寤虹珛瀹夊叏銆?
 (#) 瀹㈡埛绔皢鐢ㄤ簬淇濇姢鍏堕€氫俊鐨勫瘑閽ョ殑鍚嶇О鐢变竴涓鎺ュ瓧閫夐」鎸囧畾銆?

鍏充簬 sendmsg 鐨勬敞鎰忎簨椤癸細

 (#) 鍙互璁剧疆 MSG_WAITALL锛屽憡璇?sendmsg 蹇界暐淇″彿锛屽彧瑕佸绔湪鍚堢悊鏃堕棿鍐呭彇寰楄繘灞曘€佷娇鎴戜滑寰椾互鎶婅
     鍙戦€佺殑鎵€鏈夋暟鎹帓鍏ラ槦鍒楀嵆鍙€傝繖瑕佹眰瀹㈡埛绔湪姣?2*RTT 鐨勬椂闂存鍐呰嚦灏戞帴鏀朵竴涓暟鎹寘銆?
     濡傛灉娌℃湁璁剧疆杩欎釜锛宻endmsg() 浼氱珛鍗宠繑鍥烇紝濡傛灉浠€涔堥兘娌℃秷璐瑰垯杩斿洖 EINTR/ERESTARTSYS锛屽惁鍒欒繑鍥?     宸叉秷璐圭殑鏁版嵁閲忋€?

鍏充簬 recvmsg 鐨勬敞鎰忎簨椤癸細

 (#) 濡傛灉鎺ユ敹闃熷垪涓湁涓€绯诲垪灞炰簬鏌愪釜鐗瑰畾璋冪敤鐨勬暟鎹秷鎭紝閭ｄ箞 recvmsg 灏嗘寔缁鐞嗗畠浠紝鐩村埌锛?
     (a) 瀹冮亣鍒拌璋冪敤宸叉帴鏀舵暟鎹殑鏈熬锛?
     (b) 瀹冮亣鍒颁竴涓潪鏁版嵁娑堟伅锛?
     (c) 瀹冮亣鍒板睘浜庡彟涓€涓皟鐢ㄧ殑娑堟伅锛屾垨

     (d) 瀹冨～婊′簡鐢ㄦ埛缂撳啿鍖恒€?
     濡傛灉 recvmsg 鍦ㄩ樆濉炴ā寮忎笅琚皟鐢紝瀹冨皢鎸佺画鐫＄湢锛岀瓑寰呰繘涓€姝ユ暟鎹殑鎺ユ敹锛岀洿鍒颁笂杩板洓涓潯浠朵箣涓€
     琚弧瓒炽€?
 (2) MSG_PEEK 鎿嶄綔绫讳技锛屼絾濡傛灉瀹冨凡鍦ㄧ紦鍐插尯涓斁鍏ヤ簡浠讳綍鏁版嵁锛屽畠浼氱珛鍗宠繑鍥烇紝鑰屼笉鏄竴鐩寸潯鐪犵洿鍒?     鑳藉～婊＄紦鍐插尯銆?
 (3) 濡傛灉涓€涓暟鎹秷鎭湪濉弧鐢ㄦ埛缂撳啿鍖烘椂鍙閮ㄥ垎娑堣垂锛岄偅涔堣娑堟伅鐨勫墿浣欓儴鍒嗗皢鐣欏湪闃熷垪鍓嶇渚涗笅涓€涓?     鎺ユ敹鑰呬娇鐢ㄣ€傛案杩滀笉浼氭爣璁?MSG_TRUNC銆?
 (4) 濡傛灉涓€涓皟鐢ㄨ繕鏈夋暟鎹彲鍙栵紙瀹冨皻鏈鍒惰闃舵鏈€鍚庝竴涓暟鎹秷鎭殑鏈€鍚庝竴涓瓧鑺傦級锛岄偅涔堝皢鏍囪
     MSG_MORE銆?

## 鎺у埗娑堟伅


AF_RXRPC 鍒╃敤 sendmsg() 鍜?recvmsg() 涓殑鎺у埗娑堟伅鏉ュ璺鐢ㄨ皟鐢ㄣ€佽皟鐢ㄦ煇浜涙搷浣滃苟鎶ュ憡鏌愪簺鐘跺喌銆?瀹冧滑鏄細

	=======================	=== ===========	===============================
	MESSAGE ID		SRT DATA	MEANING
	=======================	=== ===========	===============================
	RXRPC_USER_CALL_ID	sr- User ID	App's call specifier
	RXRPC_ABORT		srt Abort code	Abort code to issue/received
	RXRPC_ACK		-rt n/a		Final ACK received
	RXRPC_NET_ERROR		-rt error num	Network error on call
	RXRPC_BUSY		-rt n/a		Call rejected (server busy)
	RXRPC_LOCAL_ERROR	-rt error num	Local error encountered
	RXRPC_NEW_CALL		-r- n/a		New call received
	RXRPC_ACCEPT		s-- n/a		Accept new call
	RXRPC_EXCLUSIVE_CALL	s-- n/a		Make an exclusive client call
	RXRPC_UPGRADE_SERVICE	s-- n/a		Client call can be upgraded
	RXRPC_TX_LENGTH		s-- data len	Total length of Tx data
	=======================	=== ===========	===============================

	(SRT = usable in Sendmsg / delivered by Recvmsg / Terminal message)

 (#) RXRPC_USER_CALL_ID

     杩欑敤浜庢寚绀哄簲鐢ㄧ▼搴忕殑璋冪敤 ID銆傚畠鏄竴涓棤绗﹀彿闀挎暣鍨嬶紝鐢卞簲鐢ㄧ▼搴忓湪瀹㈡埛绔€氳繃鎶婂畠闄勫姞鍒扮涓€涓?     鏁版嵁娑堟伅銆佹垨鍦ㄦ湇鍔＄閫氳繃鍦?RXRPC_ACCEPT 娑堟伅鐨勫叧鑱斾腑浼犻€掑畠鏉ユ寚瀹氥€俽ecvmsg() 鍦ㄩ櫎
     RXRPC_NEW_CALL 娑堟伅涔嬪鐨勬墍鏈夋秷鎭腑浼犻€掑畠銆?
 (#) RXRPC_ABORT

     杩欏彲琚簲鐢ㄧ▼搴忕敤鏉ラ€氳繃鎶婂畠浼犵粰 sendmsg 鏉ヤ腑姝竴涓皟鐢紝鎴栬€呭彲鐢?recvmsg 浼犻€掍互鎸囩ず鏀跺埌浜嗕竴涓?     杩滅▼涓銆傛棤璁哄摢绉嶆柟寮忥紝瀹冮兘蹇呴』涓?RXRPC_USER_CALL_ID 鍏宠仈浠ユ寚瀹氬彈褰卞搷鐨勮皟鐢ㄣ€傚鏋滆鍙戦€佷竴涓?     涓锛屼絾涓嶅瓨鍦ㄥ叿鏈夎鐢ㄦ埛 ID 鐨勮皟鐢紝鍒欏皢杩斿洖閿欒 EBADSLT銆?
 (#) RXRPC_ACK

     杩欒浼犻€掔粰鏈嶅姟绔簲鐢ㄧ▼搴忥紝浠ユ寚绀轰粠瀹㈡埛绔敹鍒颁簡涓€涓皟鐢ㄧ殑鏈€缁?ACK銆傚畠灏嗕笌 RXRPC_USER_CALL_ID
     鍏宠仈锛屼互鎸囩ず鐜板湪宸茬粡瀹屾垚鐨勮皟鐢ㄣ€?
 (#) RXRPC_NET_ERROR

     杩欒浼犻€掔粰搴旂敤绋嬪簭锛屼互鎸囩ず鍦ㄥ皾璇曚笌瀵圭閫氫俊鐨勮繃绋嬩腑閬囧埌浜?ICMP 閿欒娑堟伅銆傛帶鍒舵秷鎭暟鎹腑浼氬寘鍚?     涓€涓?errno 绫荤殑鏁存暟鍊兼潵鎸囩ず闂锛岃€?RXRPC_USER_CALL_ID 灏嗘寚绀哄彈褰卞搷鐨勮皟鐢ㄣ€?
 (#) RXRPC_BUSY

     杩欒浼犻€掔粰瀹㈡埛绔簲鐢ㄧ▼搴忥紝浠ユ寚绀轰竴涓皟鐢ㄥ洜鏈嶅姟绔蹇欒€岃鎷掔粷銆傚畠灏嗕笌 RXRPC_USER_CALL_ID 鍏宠仈锛?     浠ユ寚绀鸿鎷掔粷鐨勮皟鐢ㄣ€?
 (#) RXRPC_LOCAL_ERROR

     杩欒浼犻€掔粰搴旂敤绋嬪簭锛屼互鎸囩ず閬囧埌浜嗘湰鍦伴敊璇紝骞跺洜姝や腑姝簡涓€涓皟鐢ㄣ€傛帶鍒舵秷鎭暟鎹腑浼氬寘鍚竴涓?errno
     绫荤殑鏁存暟鍊兼潵鎸囩ず闂锛岃€?RXRPC_USER_CALL_ID 灏嗘寚绀哄彈褰卞搷鐨勮皟鐢ㄣ€?
 (#) RXRPC_NEW_CALL

     杩欒浼犻€掍互鍚戞湇鍔＄搴旂敤绋嬪簭鎸囩ず涓€涓柊鐨勮皟鐢ㄥ凡缁忓埌杈惧苟姝ｅ湪绛夊緟鎺ュ彈銆傛病鏈変笌涔嬪叧鑱旂殑鐢ㄦ埛 ID锛屽洜涓?     涔嬪悗蹇呴』閫氳繃鎵ц RXRPC_ACCEPT 鏉ュ垎閰嶄竴涓敤鎴?ID銆?
 (#) RXRPC_ACCEPT

     杩欒鏈嶅姟绔簲鐢ㄧ▼搴忕敤鏉ュ皾璇曟帴鍙椾竴涓皟鐢ㄥ苟涓哄叾鍒嗛厤涓€涓敤鎴?ID銆傚畠搴斿綋涓?RXRPC_USER_CALL_ID 鍏宠仈锛?     浠ユ寚绀鸿鍒嗛厤鐨勭敤鎴?ID銆傚鏋滄病鏈夎鎺ュ彈鐨勮皟鐢紙瀹冨彲鑳藉凡瓒呮椂銆佽涓绛夛級锛屽垯 sendmsg 灏嗚繑鍥為敊璇?     ENODATA銆傚鏋滆鐢ㄦ埛 ID 宸茶鍙︿竴涓皟鐢ㄤ娇鐢紝鍒欏皢杩斿洖閿欒 EBADSLT銆?
 (#) RXRPC_EXCLUSIVE_CALL

     杩欑敤浜庢寚绀轰竴涓鎴风璋冪敤搴斿綋鍦ㄤ竴涓竴娆℃€х殑杩炴帴涓婅繘琛屻€傝杩炴帴浼氬湪璋冪敤缁堟鍚庤涓㈠純銆?
 (#) RXRPC_UPGRADE_SERVICE

     杩欑敤浜庤繘琛屼竴涓鎴风璋冪敤锛屼互鎺㈡祴鎸囧畾鐨勬湇鍔?ID 鏄惁鍙鏈嶅姟绔崌绾с€傝皟鐢ㄨ€呭繀椤绘鏌?recvmsg() 杩斿洖
     鐨?msg_name 涓疄闄呬娇鐢ㄧ殑鏈嶅姟 ID銆傝鎺㈡祴鐨勬搷浣滃繀椤绘槸鍦ㄤ袱涓湇鍔′腑閮介噰鐢ㄧ浉鍚屽弬鏁扮殑閭ｄ釜銆?
     涓€鏃︾敤杩欏缓绔嬩簡鏈嶅姟绔崌绾ц兘鍔涳紙鎴栫己涔忚鑳藉姏锛夛紝杩斿洖鐨勬湇鍔?ID 搴斿綋鐢ㄤ簬鏈潵鍒拌鏈嶅姟绔殑鎵€鏈夐€氫俊锛?     骞朵笖涓嶅簲鍐嶈缃?RXRPC_UPGRADE_SERVICE銆?
 (#) RXRPC_TX_LENGTH

     杩欑敤浜庢妸涓€娆¤皟鐢紙鏃犺鏄鎴风璇锋眰杩樻槸鏈嶅姟搴旂瓟锛夊皢瑕佷紶杈撶殑鏁版嵁鎬婚噺鍛婄煡鍐呮牳銆傚鏋滅粰鍑猴紝瀹冨厑璁?     鍐呮牳鐩存帴浠庣敤鎴风┖闂寸紦鍐插尯鍔犲瘑鍒版暟鎹寘缂撳啿鍖猴紝鑰屼笉鏄厛澶嶅埗鍒扮紦鍐插尯鍐嶅氨鍦板姞瀵嗐€傝繖鍙兘闅忎负涓€娆¤皟鐢?     鎻愪緵鏁版嵁鐨勭涓€涓?sendmsg() 涓€璧风粰鍑恒€傚鏋滃疄闄呯粰鍑虹殑鏁版嵁閲忎笉鍚岋紝灏嗕骇鐢?EMSGSIZE銆?
     瀹冩帴鍙椾竴涓?__s64 绫诲瀷鐨勫弬鏁帮紝鎸囩ず灏嗚浼犺緭澶氬皯銆傝鍊间笉寰楀皬浜庨浂銆?
绗﹀彿 RXRPC__SUPPORTED 琚畾涔変负姣旀墍鏀寔鐨勬渶楂樻帶鍒舵秷鎭被鍨嬪ぇ涓€銆傚湪杩愯鏃讹紝杩欏彲浠ラ€氳繃
RXRPC_SUPPORTED_CMSG 濂楁帴瀛楅€夐」锛堣涓嬫枃锛夋潵鏌ヨ銆?

## 濂楁帴瀛楅€夐」


AF_RXRPC 濂楁帴瀛楀湪 SOL_RXRPC 灞傜骇鏀寔灏戞暟鍑犱釜濂楁帴瀛楅€夐」锛?
 (#) RXRPC_SECURITY_KEY

     杩欑敤浜庢寚瀹氳浣跨敤鐨勫瘑閽ョ殑鎻忚堪銆傝瀵嗛挜閫氳繃 request_key() 浠庤皟鐢ㄨ繘绋嬬殑瀵嗛挜鐜腑鎻愬彇锛屽苟涓斿簲褰撴槸
     "rxrpc" 绫诲瀷銆?
     optval 鎸囬拡鎸囧悜鎻忚堪瀛楃涓诧紝optlen 鎸囩ず瀛楃涓茬殑闀垮害锛堜笉鍚?NUL 缁堟绗︼級銆?
 (#) RXRPC_SECURITY_KEYRING

     涓庝笂闈㈢被浼硷紝浣嗘寚瀹氳浣跨敤鐨勬湇鍔＄瀵嗛挜鐨勫瘑閽ョ幆锛堝瘑閽ョ被鍨?"keyring"锛夈€傚弬瑙?瀹夊叏鎬?涓€鑺傘€?
 (#) RXRPC_EXCLUSIVE_CONNECTION

     杩欑敤浜庤姹傚湪鏈鎺ュ瓧涓婂悗缁繘琛岀殑姣忔璋冪敤閮戒娇鐢ㄦ柊杩炴帴銆俹ptval 搴斾负 NULL 涓?optlen 涓?0銆?
 (#) RXRPC_MIN_SECURITY_LEVEL

     杩欑敤浜庢寚瀹氭湰濂楁帴瀛椾笂璋冪敤鎵€闇€鐨勬渶浣庡畨鍏ㄧ骇鍒€俹ptval 蹇呴』鎸囧悜涓€涓寘鍚笅鍒楀€间箣涓€鐨?int锛?
     (a) RXRPC_SECURITY_PLAIN

	 浠呭姞瀵嗘牎楠屽拰銆?
     (b) RXRPC_SECURITY_AUTH

	 鍔犲瘑鏍￠獙鍜岋紝澶栧姞鏁版嵁鍖呰濉厖涓斿墠鍏釜瀛楄妭琚姞瀵嗏€斺€斿叾涓寘鍚疄闄呯殑鏁版嵁鍖呴暱搴︺€?
     (c) RXRPC_SECURITY_ENCRYPT

	 鍔犲瘑鏍￠獙鍜岋紝澶栧姞鏁翠釜鏁版嵁鍖呰濉厖骞跺姞瀵嗭紝鍖呮嫭瀹為檯鐨勬暟鎹寘闀垮害銆?
 (#) RXRPC_UPGRADEABLE_SERVICE

     杩欑敤浜庢寚绀轰竴涓叿鏈変袱涓粦瀹氱殑鏈嶅姟濂楁帴瀛楀彲浠ュ湪瀹㈡埛绔姹傛椂鎶婁竴涓粦瀹氱殑鏈嶅姟鍗囩骇鍒板彟涓€涓€俹ptval
     蹇呴』鎸囧悜涓€涓寘鍚袱涓棤绗﹀彿鐭暣鍨嬬殑鏁扮粍銆傜涓€涓槸瑕佷粠涓崌绾х殑鏈嶅姟 ID锛岀浜屼釜鏄鍗囩骇鍒扮殑鏈嶅姟 ID銆?
 (#) RXRPC_SUPPORTED_CMSG

     杩欐槸涓€涓彧璇婚€夐」锛屽畠鎶婁竴涓?int 鍐欏叆缂撳啿鍖猴紝鎸囩ず鎵€鏀寔鐨勬渶楂樻帶鍒舵秷鎭被鍨嬨€?

## 瀹夊叏鎬?

鐩墠锛屽彧瀹炵幇浜?kerberos 4 鐨勭瓑浠峰崗璁紙瀹夊叏绱㈠紩 2 - rxkad锛夈€傝繖闇€瑕佸姞杞?rxkad 妯″潡锛屽苟涓斿湪瀹㈡埛绔?闇€瑕佷粠 AFS kaserver 鎴?kerberos 鏈嶅姟鍣ㄨ幏鍙栭€傚綋绫诲瀷鐨勭エ鎹紙ticket锛夊苟瀹夎涓?"rxrpc" 绫诲瀷鐨勫瘑閽ャ€?杩欓€氬父浣跨敤 klog 绋嬪簭瀹屾垚銆備竴涓畝鍗曠殑 klog 绀轰緥绋嬪簭鍙互鍦ㄤ笅闈㈡壘鍒帮細

	http://people.redhat.com/~dhowells/rxrpc/klog.c

鎻愪緵缁欏鎴风 add_key() 鐨勬湁鏁堣浇鑽峰簲褰撻噰鐢ㄤ互涓嬬粨鏋勶細

```
	struct rxrpc_key_sec2_v1 {
		uint16_t	security_index;	/* 2 */
		uint16_t	ticket_length;	/* length of ticket[] */
		uint32_t	expiry;		/* time at which expires */
		uint8_t		kvno;		/* key version number */
		uint8_t		__pad[3];
		uint8_t		session_key[8];	/* DES session key */
		uint8_t		ticket[0];	/* the encrypted ticket */
	};
```

鍏朵腑绁ㄦ嵁浜岃繘鍒跺潡鍙槸闄勫姞鍦ㄤ笂杩扮粨鏋勪箣鍚庛€?

瀵逛簬鏈嶅姟绔紝蹇呴』璁?"rxrpc_s" 绫诲瀷鐨勫瘑閽ュ鏈嶅姟绔彲鐢ㄣ€傚畠浠殑鎻忚堪涓?"<serviceID>:<securityIndex>"
锛堜緥濡傦細"52:2" 琛ㄧず AFS VL 鏈嶅姟鐨?rxkad 瀵嗛挜锛夈€傚綋鍒涘缓杩欐牱涓€涓瘑閽ユ椂锛屽簲褰撴妸鏈嶅姟绔殑瀵嗛挜浣滀负
瀹炰緥鍖栨暟鎹彁渚涚粰瀹冿紙瑙佷笅闈㈢殑渚嬪瓙锛夈€?
	add_key("rxrpc_s", "52:2", secret_key, 8, keyring);

涓€涓瘑閽ョ幆閫氳繃鍦ㄤ竴涓?sockopt 涓懡鍚嶅畠鑰岃浼犵粰鏈嶅姟绔鎺ュ瓧銆傜劧鍚庡綋寤虹珛瀹夊叏鐨勪紶鍏ヨ繛鎺ユ椂锛屾湇鍔＄
濂楁帴瀛楀湪杩欎釜瀵嗛挜鐜腑鏌ユ壘鏈嶅姟绔瘑閽ャ€傝繖鍙互鍦ㄤ竴涓ず渚嬬▼搴忎腑鐪嬪埌锛岃绋嬪簭浣嶄簬锛?
	http://people.redhat.com/~dhowells/rxrpc/listen.c


## 绀轰緥瀹㈡埛绔敤娉?

瀹㈡埛绔細鎸変笅杩版柟寮忓彂璧蜂竴涓搷浣滐細

```
	client = socket(AF_RXRPC, SOCK_DGRAM, PF_INET);

     Where the third parameter indicates the protocol family of the transport
     socket used - usually IPv4 but it can also be IPv6 [TODO].

 (2) A local address can optionally be bound::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= 0,  /* we're a client */
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7000), /* AFS callback */
		.transport.sin_address	= 0,  /* all local interfaces */
	};
	bind(client, &srx, sizeof(srx));

     This specifies the local UDP port to be used. If not given, a random
     non-privileged port will be used. A UDP port may be shared between
     several unrelated RxRPC sockets. Security is handled on a basis of
     per-RxRPC virtual connection.

 (3) The security is set::

	const char *key = "AFS:cambridge.redhat.com";
	setsockopt(client, SOL_RXRPC, RXRPC_SECURITY_KEY, key, strlen(key));

     This issues a request_key() to get the key representing the security
     context. The minimum security level can be set::

	unsigned int sec = RXRPC_SECURITY_ENCRYPT;
	setsockopt(client, SOL_RXRPC, RXRPC_MIN_SECURITY_LEVEL,
		   &sec, sizeof(sec));

 (4) The server to be contacted can then be specified (alternatively this can
     be done through sendmsg)::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= VL_SERVICE_ID,
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7005), /* AFS volume manager */
		.transport.sin_address	= ...,
	};
	connect(client, &srx, sizeof(srx));

 (5) The request data should then be posted to the server socket using a series
     of sendmsg() calls, each with the following control message attached:

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

     MSG_MORE should be set in msghdr::msg_flags on all but the last part of
     the request. Multiple requests may be made simultaneously.

     An RXRPC_TX_LENGTH control message can also be specified on the first
     sendmsg() call.

     If a call is intended to go to a destination other than the default
     specified through connect(), then msghdr::msg_name should be set on the
     first request message of that call.

 (6) The reply data will then be posted to the server socket for recvmsg() to
     pick up. MSG_MORE will be flagged by recvmsg() if there's more reply data
     for a particular call to be read. MSG_EOR will be set on the terminal
     read for a call.

     All data will be delivered with the following control message attached:

	RXRPC_USER_CALL_ID	- specifies the user ID for this call

     If an abort or error occurred, this will be returned in the control data
     buffer instead, and MSG_EOR will be flagged to indicate the end of that
     call.
```

瀹㈡埛绔彲浠ヨ姹備竴涓畠宸茬煡鐨勬湇鍔?ID锛屽苟閫氳繃鍦ㄨ皟鐢ㄧ殑绗竴涓?sendmsg() 涓婃彁渚?RXRPC_UPGRADE_SERVICE
鏉ヨ姹傚湪鏈夋洿濂界殑鏈嶅姟鍙敤鏃舵妸瀹冨崌绾у埌鏇村ソ鐨勬湇鍔°€傚鎴风闅忓悗搴斿綋鍦ㄦ敹闆嗙粨鏋滄椂妫€鏌ョ敱 recvmsg() 濉厖鐨?msg_name 涓殑 srx_service銆傚鏋滆鍗囩骇璇锋眰琚湇鍔″拷鐣ヤ簡锛宻rx_service 灏嗘寔鏈変笌浼犵粰 sendmsg() 鐩稿悓鐨勫€硷紱
鍚﹀垯瀹冧細琚敼涓烘寚绀烘湇鍔＄鍗囩骇鍒扮殑鏈嶅姟 ID銆傛敞鎰忥紝鍗囩骇鍚庣殑鏈嶅姟 ID 鐢辨湇鍔＄閫夋嫨銆傝皟鐢ㄨ€呭繀椤荤瓑鍒板畠鍦ㄥ簲绛?涓湅鍒拌鏈嶅姟 ID 涔嬪悗锛屾墠鑳藉彂閫佷换浣曟洿澶氱殑璋冪敤锛堝湪鐩稿悓鐩殑鍦颁笂鐨勮繘涓€姝ヨ皟鐢ㄤ細琚樆濉烇紝鐩村埌鎺㈡祴缁撴潫锛夈€?

## 绀轰緥鏈嶅姟绔敤娉?

鏈嶅姟绔細鎸変笅杩版柟寮忓缓绔嬩互鎺ュ彈鎿嶄綔锛?
```
	server = socket(AF_RXRPC, SOCK_DGRAM, PF_INET);

     Where the third parameter indicates the address type of the transport
     socket used - usually IPv4.

 (2) Security is set up if desired by giving the socket a keyring with server
     secret keys in it::

	keyring = add_key("keyring", "AFSkeys", NULL, 0,
			  KEY_SPEC_PROCESS_KEYRING);

	const char secret_key[8] = {
		0xa7, 0x83, 0x8a, 0xcb, 0xc7, 0x83, 0xec, 0x94 };
	add_key("rxrpc_s", "52:2", secret_key, 8, keyring);

	setsockopt(server, SOL_RXRPC, RXRPC_SECURITY_KEYRING, "AFSkeys", 7);

     The keyring can be manipulated after it has been given to the socket. This
     permits the server to add more keys, replace keys, etc. while it is live.

 (3) A local address must then be bound::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= VL_SERVICE_ID, /* RxRPC service ID */
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7000), /* AFS callback */
		.transport.sin_address	= 0,  /* all local interfaces */
	};
	bind(server, &srx, sizeof(srx));

     More than one service ID may be bound to a socket, provided the transport
     parameters are the same. The limit is currently two. To do this, bind()
     should be called twice.

 (4) If service upgrading is required, first two service IDs must have been
     bound and then the following option must be set::

	unsigned short service_ids[2] = { from_ID, to_ID };
	setsockopt(server, SOL_RXRPC, RXRPC_UPGRADEABLE_SERVICE,
		   service_ids, sizeof(service_ids));

     This will automatically upgrade connections on service from_ID to service
     to_ID if they request it. This will be reflected in msg_name obtained
     through recvmsg() when the request data is delivered to userspace.

 (5) The server is then set to listen out for incoming calls::

	listen(server, 100);

 (6) The kernel notifies the server of pending incoming connections by sending
     it a message for each. This is received with recvmsg() on the server
     socket. It has no data, and has a single dataless control message
     attached::

	RXRPC_NEW_CALL

     The address that can be passed back by recvmsg() at this point should be
     ignored since the call for which the message was posted may have gone by
     the time it is accepted - in which case the first call still on the queue
     will be accepted.

 (7) The server then accepts the new call by issuing a sendmsg() with two
     pieces of control data and no actual data:

	==================	==============================
	RXRPC_ACCEPT		indicate connection acceptance
	RXRPC_USER_CALL_ID	specify user ID for this call
	==================	==============================

 (8) The first request data packet will then be posted to the server socket for
     recvmsg() to pick up. At that point, the RxRPC address for the call can
     be read from the address fields in the msghdr struct.

     Subsequent request data will be posted to the server socket for recvmsg()
     to collect as it arrives. All but the last piece of the request data will
     be delivered with MSG_MORE flagged.

     All data will be delivered with the following control message attached:


	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

 (9) The reply data should then be posted to the server socket using a series
     of sendmsg() calls, each with the following control messages attached:

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

     MSG_MORE should be set in msghdr::msg_flags on all but the last message
     for a particular call.

```

(10) 瀹㈡埛绔殑鏈€缁?ACK 鍦ㄨ鏀跺埌鏃跺皢鍙戝竷渚?recvmsg() 鑾峰彇銆傚畠灏嗛噰鍙栦竴涓笉甯︽暟鎹殑娑堟伅鐨勫舰寮忥紝骞堕檮甯?     涓や釜鎺у埗娑堟伅锛?
	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	RXRPC_ACK		indicates final ACK (no data)
	==================	===================================

     MSG_EOR 浼氳鏍囪浠ユ寚绀鸿繖鏄璋冪敤鐨勬渶缁堟秷鎭€?
(11) 鐩村埌搴旂瓟鏁版嵁鐨勬渶鍚庝竴涓暟鎹寘琚彂閫佷箣鍓嶏紝璇ヨ皟鐢ㄩ兘鍙互閫氳繃璋冪敤甯︽湁涓€涓笉甯︽暟鎹殑娑堟伅鐨?sendmsg()
     鏉ヤ腑姝紝璇ユ秷鎭檮甯︿互涓嬫帶鍒舵秷鎭細

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	RXRPC_ABORT		indicates abort code (4 byte data)
	==================	===================================

     濡傛灉鍙戝嚭杩欎釜锛屼换浣曞湪濂楁帴瀛楁帴鏀堕槦鍒椾腑绛夊緟鐨勬暟鎹寘閮藉皢琚涪寮冦€?
娉ㄦ剰锛屾煇涓壒瀹氭湇鍔＄殑鎵€鏈夐€氫俊閮介€氳繃閭ｄ竴涓湇鍔＄濂楁帴瀛楄繘琛岋紝浣跨敤 sendmsg() 鍜?recvmsg() 涓婄殑鎺у埗
娑堟伅鏉ョ‘瀹氬彈褰卞搷鐨勮皟鐢ㄣ€?

## AF_RXRPC 鍐呮牳鎺ュ彛


AF_RXRPC 妯″潡杩樹负鍐呮牳鍐呭疄鐢ㄧ▼搴忥紙渚嬪 AFS 鏂囦欢绯荤粺锛夋彁渚涗簡涓€涓帴鍙ｃ€傝繖鍏佽杩欐牱鐨勫疄鐢ㄧ▼搴忥細

 (1) 鍦ㄤ竴涓鎺ュ瓧涓婄洿鎺ュ鍚勪釜瀹㈡埛绔皟鐢ㄤ娇鐢ㄤ笉鍚岀殑瀵嗛挜锛岃€屼笉蹇呮墦寮€涓€澶у爢濂楁帴瀛椼€佹瘡涓彲鑳芥兂鐢ㄧ殑
     瀵嗛挜涓€涓€?
 (2) 閬垮厤璁?RxRPC 鍦ㄥ彂璧疯皟鐢ㄦ垨鎵撳紑濂楁帴瀛楃殑鏃跺埢璋冪敤 request_key()銆傝€屾槸鐢卞疄鐢ㄧ▼搴忚礋璐ｅ湪閫傚綋鐨?     鏃跺埢璇锋眰瀵嗛挜銆備緥濡傦紝AFS 浼氬湪 open() 鎴?unlink() 绛?VFS 鎿嶄綔鏈熼棿杩欐牱鍋氥€傜劧鍚庡湪璋冪敤鍙戣捣鏃舵妸
     瀵嗛挜浼犺繘鍘汇€?
 (3) 璇锋眰浣跨敤 GFP_KERNEL 涔嬪鐨勪笢瑗挎潵鍒嗛厤鍐呭瓨銆?
 (4) 閬垮厤 recvmsg() 璋冪敤鐨勫紑閿€銆俁xRPC 娑堟伅鍙互鍦ㄨ鏀惧叆濂楁帴瀛?Rx 闃熷垪涔嬪墠琚嫤鎴紝骞剁洿鎺ユ搷浣滃鎺ュ瓧
     缂撳啿鍖恒€?
瑕佷娇鐢?RxRPC 璁炬柦锛屼竴涓唴鏍稿疄鐢ㄧ▼搴忎粛鐒跺繀椤绘墦寮€涓€涓?AF_RXRPC 濂楁帴瀛楋紝閫傚綋鍦扮粦瀹氫竴涓湴鍧€锛屽苟涓斿鏋?瀹冩槸鏈嶅姟绔鎺ュ瓧灏辩洃鍚紝鐒跺悗鎶婂畠浼犵粰鍐呮牳鎺ュ彛鍑芥暟銆?
鍐呮牳鎺ュ彛鍑芥暟濡備笅锛?
```
	struct rxrpc_call *
	rxrpc_kernel_begin_call(struct socket *sock,
				struct sockaddr_rxrpc *srx,
				struct key *key,
				unsigned long user_call_ID,
				s64 tx_total_len,
				gfp_t gfp,
				rxrpc_notify_rx_t notify_rx,
				bool upgrade,
				bool intr,
				unsigned int debug_id);

     This allocates the infrastructure to make a new RxRPC call and assigns
     call and connection numbers. The call will be made on the UDP port that
     the socket is bound to. The call will go to the destination address of a
     connected client socket unless an alternative is supplied (srx is
     non-NULL).

     If a key is supplied then this will be used to secure the call instead of
     the key bound to the socket with the RXRPC_SECURITY_KEY sockopt. Calls
     secured in this way will still share connections if at all possible.

     The user_call_ID is equivalent to that supplied to sendmsg() in the
     control data buffer. It is entirely feasible to use this to point to a
     kernel data structure.

     tx_total_len is the amount of data the caller is intending to transmit
     with this call (or -1 if unknown at this point). Setting the data size
     allows the kernel to encrypt directly to the packet buffers, thereby
     saving a copy. The value may not be less than -1.

     notify_rx is a pointer to a function to be called when events such as
     incoming data packets or remote aborts happen.

     upgrade should be set to true if a client operation should request that
     the server upgrade the service to a better one. The resultant service ID
     is returned by rxrpc_kernel_recv_data().

     intr should be set to true if the call should be interruptible. If this
     is not set, this function may not return until a channel has been
     allocated; if it is set, the function may return -ERESTARTSYS.

     debug_id is the call debugging ID to be used for tracing. This can be
     obtained by atomically incrementing rxrpc_debug_id.

     If this function is successful, an opaque reference to the RxRPC call is
     returned. The caller now holds a reference on this and it must be
     properly ended.

 (#) Shut down a client call::

	void rxrpc_kernel_shutdown_call(struct socket *sock,
					struct rxrpc_call *call);

     This is used to shut down a previously begun call. The user_call_ID is
     expunged from AF_RXRPC's knowledge and will not be seen again in
     association with the specified call.

 (#) Release the ref on a client call::

	void rxrpc_kernel_put_call(struct socket *sock,
				   struct rxrpc_call *call);

     This is used to release the caller's ref on an rxrpc call.

 (#) Send data through a call::

	typedef void (*rxrpc_notify_end_tx_t)(struct sock *sk,
					      unsigned long user_call_ID,
					      struct sk_buff *skb);

	int rxrpc_kernel_send_data(struct socket *sock,
				   struct rxrpc_call *call,
				   struct msghdr *msg,
				   size_t len,
				   rxrpc_notify_end_tx_t notify_end_rx);

     This is used to supply either the request part of a client call or the
     reply part of a server call. msg.msg_iovlen and msg.msg_iov specify the
     data buffers to be used. msg_iov may not be NULL and must point
     exclusively to in-kernel virtual addresses. msg.msg_flags may be given
     MSG_MORE if there will be subsequent data sends for this call.

     The msg must not specify a destination address, control data or any flags
     other than MSG_MORE. len is the total amount of data to transmit.

     notify_end_rx can be NULL or it can be used to specify a function to be
     called when the call changes state to end the Tx phase. This function is
     called with a spinlock held to prevent the last DATA packet from being
     transmitted until the function returns.

 (#) Receive data from a call::

	int rxrpc_kernel_recv_data(struct socket *sock,
				   struct rxrpc_call *call,
				   void *buf,
				   size_t size,
				   size_t *_offset,
				   bool want_more,
				   u32 *_abort,
				   u16 *_service)

      This is used to receive data from either the reply part of a client call
      or the request part of a service call. buf and size specify how much
      data is desired and where to store it. *_offset is added on to buf and
      subtracted from size internally; the amount copied into the buffer is
      added to *_offset before returning.

      want_more should be true if further data will be required after this is
      satisfied and false if this is the last item of the receive phase.

      There are three normal returns: 0 if the buffer was filled and want_more
      was true; 1 if the buffer was filled, the last DATA packet has been
      emptied and want_more was false; and -EAGAIN if the function needs to be
      called again.

      If the last DATA packet is processed but the buffer contains less than
      the amount requested, EBADMSG is returned. If want_more wasn't set, but
      more data was available, EMSGSIZE is returned.

      If a remote ABORT is detected, the abort code received will be stored in
      ``*_abort`` and ECONNABORTED will be returned.

      The service ID that the call ended up with is returned into *_service.
      This can be used to see if a call got a service upgrade.

 (#) Abort a call??

     ::

	void rxrpc_kernel_abort_call(struct socket *sock,
				     struct rxrpc_call *call,
				     u32 abort_code);

     This is used to abort a call if it's still in an abortable state. The
     abort code specified will be placed in the ABORT message sent.

 (#) Intercept received RxRPC messages::

	typedef void (*rxrpc_interceptor_t)(struct sock *sk,
					    unsigned long user_call_ID,
					    struct sk_buff *skb);

	void
	rxrpc_kernel_intercept_rx_messages(struct socket *sock,
					   rxrpc_interceptor_t interceptor);

     This installs an interceptor function on the specified AF_RXRPC socket.
     All messages that would otherwise wind up in the socket's Rx queue are
     then diverted to this function. Note that care must be taken to process
     the messages in the right order to maintain DATA message sequentiality.

     The interceptor function itself is provided with the address of the socket
     and handling the incoming message, the ID assigned by the kernel utility
     to the call and the socket buffer containing the message.

     The skb->mark field indicates the type of message:

	===============================	=======================================
	Mark				Meaning
	===============================	=======================================
	RXRPC_SKB_MARK_DATA		Data message
	RXRPC_SKB_MARK_FINAL_ACK	Final ACK received for an incoming call
	RXRPC_SKB_MARK_BUSY		Client call rejected as server busy
	RXRPC_SKB_MARK_REMOTE_ABORT	Call aborted by peer
	RXRPC_SKB_MARK_NET_ERROR	Network error detected
	RXRPC_SKB_MARK_LOCAL_ERROR	Local error encountered
	RXRPC_SKB_MARK_NEW_CALL		New incoming call awaiting acceptance
	===============================	=======================================

     The remote abort message can be probed with rxrpc_kernel_get_abort_code().
     The two error messages can be probed with rxrpc_kernel_get_error_number().
     A new call can be accepted with rxrpc_kernel_accept_call().

     Data messages can have their contents extracted with the usual bunch of
     socket buffer manipulation functions. A data message can be determined to
     be the last one in a sequence with rxrpc_kernel_is_data_last(). When a
     data message has been used up, rxrpc_kernel_data_consumed() should be
     called on it.

     Messages should be handled to rxrpc_kernel_free_skb() to dispose of. It
     is possible to get extra refs on all types of message for later freeing,
     but this may pin the state of a call until the message is finally freed.

 (#) Accept an incoming call::

	struct rxrpc_call *
	rxrpc_kernel_accept_call(struct socket *sock,
				 unsigned long user_call_ID);

     This is used to accept an incoming call and to assign it a call ID. This
     function is similar to rxrpc_kernel_begin_call() and calls accepted must
     be ended in the same way.

     If this function is successful, an opaque reference to the RxRPC call is
     returned. The caller now holds a reference on this and it must be
     properly ended.

 (#) Reject an incoming call::

	int rxrpc_kernel_reject_call(struct socket *sock);

     This is used to reject the first incoming call on the socket's queue with
     a BUSY message. -ENODATA is returned if there were no incoming calls.
     Other errors may be returned if the call had been aborted (-ECONNABORTED)
     or had timed out (-ETIME).

 (#) Allocate a null key for doing anonymous security::

	struct key *rxrpc_get_null_key(const char *keyname);

     This is used to allocate a null RxRPC key that can be used to indicate
     anonymous security for a particular domain.

 (#) Get the peer address of a call::

	void rxrpc_kernel_get_peer(struct socket *sock, struct rxrpc_call *call,
				   struct sockaddr_rxrpc *_srx);

     This is used to find the remote peer address of a call.

 (#) Set the total transmit data size on a call::

	void rxrpc_kernel_set_tx_length(struct socket *sock,
					struct rxrpc_call *call,
					s64 tx_total_len);

     This sets the amount of data that the caller is intending to transmit on a
     call. It's intended to be used for setting the reply size as the request
     size should be set when the call is begun. tx_total_len may not be less
     than zero.

 (#) Get call RTT::

	u64 rxrpc_kernel_get_rtt(struct socket *sock, struct rxrpc_call *call);

     Get the RTT time to the peer in use by a call. The value returned is in
     nanoseconds.

 (#) Check call still alive::

	bool rxrpc_kernel_check_life(struct socket *sock,
				     struct rxrpc_call *call,
				     u32 *_life);
	void rxrpc_kernel_probe_life(struct socket *sock,
				     struct rxrpc_call *call);

     The first function passes back in ``*_life`` a number that is updated when
     ACKs are received from the peer (notably including PING RESPONSE ACKs
     which we can elicit by sending PING ACKs to see if the call still exists
     on the server). The caller should compare the numbers of two calls to see
     if the call is still alive after waiting for a suitable interval. It also
     returns true as long as the call hasn't yet reached the completed state.

     This allows the caller to work out if the server is still contactable and
     if the call is still alive on the server while waiting for the server to
     process a client operation.

     The second function causes a ping ACK to be transmitted to try to provoke
     the peer into responding, which would then cause the value returned by the
     first function to change. Note that this must be called in TASK_RUNNING
     state.

 (#) Apply the RXRPC_MIN_SECURITY_LEVEL sockopt to a socket from within in the
     kernel::

       int rxrpc_sock_set_min_security_level(struct sock *sk,
					     unsigned int val);

     This specifies the minimum security level required for calls on this
     socket.
```

## 鍙厤缃弬鏁?

RxRPC 鍗忚椹卞姩鏈変竴缁勫彲閰嶇疆鍙傛暟锛屽彲浠ラ€氳繃 /proc/net/rxrpc/ 涓殑 sysctl 杩涜璋冩暣锛?
 (#) req_ack_delay

     鍦ㄦ敹鍒颁竴涓缃簡 request-ack 鏍囧織鐨勬暟鎹寘涔嬪悗锛屽埌鎴戜滑鍏戠幇璇ユ爣蹇楀苟瀹為檯鍙戦€佹墍璇锋眰鐨?ack 涔嬪墠鐨?     鏃堕棿閲忥紙姣锛夈€?
     閫氬父瀵圭鍦ㄦ垜浠叕甯冪殑鎺ユ敹绐楀彛濉弧锛堟渶澶?255 涓暟鎹寘锛変箣鍓嶄笉浼氬仠姝㈠彂閫佹暟鎹寘锛屽洜姝ゅ欢杩?ACK 鍏佽
     涓€娆℃€у澶氫釜鏁版嵁鍖呰繘琛?ACK銆?
 (#) soft_ack_delay

     鍦ㄦ敹鍒颁竴涓柊鏁版嵁鍖呬箣鍚庯紝鍒版垜浠敓鎴愪竴涓?soft-ACK 鏉ュ憡璇夊彂閫佹柟瀹冩棤闇€閲嶅彂涔嬪墠鐨勬椂闂撮噺锛堟绉掞級銆?
 (#) idle_ack_delay

     鍦ㄥ綋鍓嶆帴鏀堕槦鍒椾腑鐨勬墍鏈夋暟鎹寘閮藉凡琚秷璐逛箣鍚庯紝鍒版垜浠敓鎴愪竴涓?hard-ACK 鏉ュ憡璇夊彂閫佹柟瀹冨彲浠ラ噴鏀惧叾
     缂撳啿鍖轰箣鍓嶇殑鏃堕棿閲忥紙姣锛夛紝鍓嶆彁鏄病鏈変换浣曞叾浠栦細璁╂垜浠彂閫?ACK 鐨勭悊鐢卞嚭鐜般€?
 (#) resend_timeout

     鍦ㄤ紶杈撲竴涓暟鎹寘涔嬪悗锛屽埌鎴戜滑鍦ㄥ亣璁炬病鏈夋敹鍒版潵鑷帴鏀舵柟鐨勩€佸憡鐭ュ叾宸叉敹鍒扮殑 ACK 涔嬪墠閲嶆柊浼犺緭瀹冧箣鍓嶇殑
     鏃堕棿閲忥紙姣锛夈€?
 (#) max_call_lifetime

     涓€涓皟鐢ㄥ湪鎴戜滑鍙互涓诲姩鏉€姝诲畠涔嬪墠鍙互澶勪簬杩涜鐘舵€佺殑鏈€澶ф椂闂撮噺锛堢锛夈€?
 (#) dead_call_expiry

     鍦ㄦ垜浠粠璋冪敤鍒楄〃涓Щ闄や竴涓璋冪敤涔嬪墠鐨勬椂闂撮噺锛堢锛夈€傛璋冪敤浼氳淇濈暀涓€灏忔鏃堕棿锛屼互渚块噸澶嶅彂閫?ACK
     鍜?ABORT 鏁版嵁鍖呫€?
 (#) connection_expiry

     鍦ㄤ竴涓繛鎺ユ渶鍚庝竴娆¤浣跨敤涔嬪悗锛屽埌鎴戜滑鎶婂畠浠庤繛鎺ュ垪琛ㄤ腑绉婚櫎涔嬪墠鐨勬椂闂撮噺锛堢锛夈€傚湪涓€涓繛鎺ュ瓨鍦ㄦ湡闂达紝
     瀹冨厖褰撳凡鍗忓晢瀹夊叏鐨勫崰浣嶇锛涘綋瀹冭鍒犻櫎鏃讹紝瀹夊叏蹇呴』閲嶆柊鍗忓晢銆?
 (#) transport_expiry

     鍦ㄤ竴涓紶杈撴渶鍚庝竴娆¤浣跨敤涔嬪悗锛屽埌鎴戜滑鎶婂畠浠庝紶杈撳垪琛ㄤ腑绉婚櫎涔嬪墠鐨勬椂闂撮噺锛堢锛夈€傚湪涓€涓紶杈撳瓨鍦ㄦ湡闂达紝
     瀹冪敤浜庨敋瀹氬绔暟鎹苟淇濇寔杩炴帴 ID 璁℃暟鍣ㄣ€?
 (#) rxrpc_rx_window_size

     浠ユ暟鎹寘涓哄崟浣嶇殑鎺ユ敹绐楀彛澶у皬銆傝繖鏄垜浠効鎰忎负浠讳綍鐗瑰畾璋冪敤鍦ㄥ唴瀛樹腑淇濈暀鐨勬湭娑堣垂鎺ユ敹鏁版嵁鍖呯殑鏈€澶ф暟閲忋€?
 (#) rxrpc_rx_mtu

     鎴戜滑鎰挎剰鎺ユ敹鐨勬渶澶ф暟鎹寘 MTU 澶у皬锛堝瓧鑺傦級銆傝繖鍚戝绔寚绀烘垜浠槸鍚︽効鎰忔帴鍙楀法鍨嬶紙jumbo锛夋暟鎹寘銆?
 (#) rxrpc_rx_jumbo_max

     鎴戜滑鎰挎剰鍦ㄤ竴涓法鍨嬫暟鎹寘涓帴鍙楃殑鍖呯殑鏈€澶ф暟閲忋€傚法鍨嬫暟鎹寘涓殑闈炵粓绔暟鎹寘蹇呴』鍖呭惈涓€涓洓瀛楄妭鐨?     澶撮儴鍔犱笂姝ｅソ 1412 瀛楄妭鐨勬暟鎹€傜粓绔暟鎹寘蹇呴』鍖呭惈涓€涓洓瀛楄妭鐨勫ご閮ㄥ姞涓婁换鎰忔暟閲忕殑鏁版嵁銆傛棤璁哄浣曪紝
     涓€涓法鍨嬫暟鎹寘鐨勫ぇ灏忎笉寰楄秴杩?rxrpc_rx_mtu銆?

## API 鍑芥暟鍙傝€?