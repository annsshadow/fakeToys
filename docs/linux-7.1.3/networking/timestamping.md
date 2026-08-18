
## 鏃堕棿鎴?

## 1. 鎺у埗鎺ュ彛


鐢ㄤ簬鎺ユ敹缃戠粶鍖呮椂闂存埑鐨勬帴鍙ｅ涓嬶細

SO_TIMESTAMP
  涓烘瘡涓叆绔欐暟鎹寘鐢熸垚鏃堕棿鎴筹紝鏃堕棿鍩轰簬锛堜笉涓€瀹氬崟璋冪殑锛夌郴缁熸椂闂淬€傞€氳繃 recvmsg() 鍦ㄦ帶鍒舵秷鎭腑浠ュ井绉掞紙usec锛夊垎杈ㄧ巼鎶ュ憡鏃堕棿鎴炽€係O_TIMESTAMP 鏍规嵁 libc 鐨勬灦鏋勭被鍨嬪拰 time_t 琛ㄧず锛岃瀹氫箟涓?SO_TIMESTAMP_NEW 鎴?SO_TIMESTAMP_OLD銆傚浜?SO_TIMESTAMP_OLD锛屾帶鍒舵秷鎭牸寮忎负 struct __kernel_old_timeval锛涘浜?SO_TIMESTAMP_NEW锛屽垯涓?struct __kernel_sock_timeval銆?
SO_TIMESTAMPNS
  涓?SO_TIMESTAMP 鐩稿悓鐨勬墦鏃堕棿鎴虫満鍒讹紝浣嗕互 struct timespec 褰㈠紡銆佷互绾崇锛坣sec锛夊垎杈ㄧ巼鎶ュ憡鏃堕棿鎴炽€係O_TIMESTAMPNS 鏍规嵁 libc 鐨勬灦鏋勭被鍨嬪拰 time_t 琛ㄧず锛岃瀹氫箟涓?SO_TIMESTAMPNS_NEW 鎴?SO_TIMESTAMPNS_OLD銆傚浜?SO_TIMESTAMPNS_OLD锛屾帶鍒舵秷鎭牸寮忎负 struct timespec锛涘浜?SO_TIMESTAMPNS_NEW锛屽垯涓?struct __kernel_timespec銆?
IP_MULTICAST_LOOP + SO_TIMESTAMP[NS]
  浠呯敤浜庡鎾細閫氳繃璇诲彇鍥炵幆鏁版嵁鍖呯殑鎺ユ敹鏃堕棿鎴虫潵鑾峰緱杩戜技鐨勫彂閫佹椂闂存埑銆?
SO_TIMESTAMPING
  鍦ㄦ帴鏀躲€佸彂閫佹垨涓よ€呬笂鐢熸垚鏃堕棿鎴炽€傛敮鎸佸绉嶆椂闂存埑鏉ユ簮锛屽寘鎷‖浠躲€傛敮鎸佷负娴佸鎺ュ瓧鐢熸垚鏃堕棿鎴炽€?

### 1.1 SO_TIMESTAMP锛堜互鍙?SO_TIMESTAMP_OLD 鍜?SO_TIMESTAMP_NEW锛?

璇ュ鎺ュ瓧閫夐」鍦ㄦ帴鏀惰矾寰勪笂鍚敤瀵规暟鎹姤鐨勬椂闂存埑銆傚洜涓虹洰鏍囧鎺ュ瓧锛堝鏋滃瓨鍦級鍦ㄧ綉缁滄爤涓緢鏅氭墠琚煡鏅擄紝鎵€浠ヨ鍔熻兘蹇呴』瀵规墍鏈夋暟鎹寘鍚敤銆傛墍鏈夋棭鏈熺殑鎺ユ敹鏃堕棿鎴抽€夐」涔熸槸濡傛銆?
鏈夊叧鎺ュ彛缁嗚妭锛岃鍙傝 `man 7 socket`銆?
濮嬬粓浣跨敤 SO_TIMESTAMP_NEW 鏃堕棿鎴筹紝浠ュ缁堣幏寰?struct __kernel_sock_timeval 鏍煎紡鐨勬椂闂存埑銆?
鍦?32 浣嶆満鍣ㄤ笂锛孲O_TIMESTAMP_OLD 鍦?2038 骞翠箣鍚庝細杩斿洖閿欒鐨勬椂闂存埑銆?
### 1.2 SO_TIMESTAMPNS锛堜互鍙?SO_TIMESTAMPNS_OLD 鍜?SO_TIMESTAMPNS_NEW锛?

璇ラ€夐」涓?SO_TIMESTAMP 瀹屽叏鐩稿悓锛屽彧鏄繑鍥炵殑鏁版嵁绫诲瀷涓嶅悓銆傚叾 struct timespec 鍏佽姣?SO_TIMESTAMP 鐨?timeval锛堟绉掞級鍏锋湁鏇撮珮鐨勫垎杈ㄧ巼锛堢撼绉掞級銆?
濮嬬粓浣跨敤 SO_TIMESTAMPNS_NEW 鏃堕棿鎴筹紝浠ュ缁堣幏寰?struct __kernel_timespec 鏍煎紡鐨勬椂闂存埑銆?
鍦?32 浣嶆満鍣ㄤ笂锛孲O_TIMESTAMPNS_OLD 鍦?2038 骞翠箣鍚庝細杩斿洖閿欒鐨勬椂闂存埑銆?
### 1.3 SO_TIMESTAMPING锛堜互鍙?SO_TIMESTAMPING_OLD 鍜?SO_TIMESTAMPING_NEW锛?

鏀寔澶氱绫诲瀷鐨勬椂闂存埑璇锋眰銆傚洜姝わ紝杩欓渶瑕?
```

  err = setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val, sizeof(val));

```

val 鏄竴涓暣鍨嬶紝鍙缃互涓嬩换鎰忔瘮鐗逛綅銆傝缃叾浠栨瘮鐗逛綅浼氳繑鍥?EINVAL锛屼笖涓嶄細鏀瑰彉褰撳墠鐘舵€併€?
璇ュ鎺ュ瓧閫夐」涓哄悇涓?sk_buff锛?.3.1锛夈€佸悜濂楁帴瀛楅敊璇槦鍒楁姤鍛婃椂闂存埑锛?.3.2锛変互鍙婇€夐」锛?.3.3锛夐厤缃椂闂存埑鐢熸垚銆備篃鍙互鍒╃敤 cmsg锛?.3.4锛変负鍗曚釜 sendmsg 璋冪敤鍚敤鏃堕棿鎴崇敓鎴愩€?

##### 1.3.1 鏃堕棿鎴崇敓鎴?

鏌愪簺姣旂壒浣嶆槸璇锋眰鍗忚鏍堝皾璇曠敓鎴愭椂闂存埑銆傚畠浠殑浠绘剰缁勫悎閮芥槸鏈夋晥鐨勩€傚杩欎簺姣旂壒浣嶇殑鏇存敼閫傜敤浜庢柊鍒涘缓鐨勬暟鎹寘锛岃€屼笉閫傜敤浜庡凡鍦ㄦ爤涓殑鏁版嵁鍖呫€傚洜姝わ紝鍙互閫氳繃灏?send() 璋冪敤宓屽叆鍒颁袱涓?setsockopt 璋冪敤涔嬮棿锛堜竴涓惎鐢ㄦ椂闂存埑鐢熸垚锛屼竴涓鐢級鏉ユ湁閫夋嫨鍦颁负涓€閮ㄥ垎鏁版嵁鍖呰姹傛椂闂存埑锛堜緥濡傜敤浜庨噰鏍凤級銆傛椂闂存埑涔熷彲鑳藉洜涓虹壒瀹氬鎺ュ瓧涔嬪鐨勫叾浠栧師鍥犺€岀敓鎴愶紝渚嬪鍓嶉潰鎵€杩板湪绯荤粺鑼冨洿鍐呭惎鐢ㄤ簡鎺ユ敹鏃堕棿鎴虫椂銆?
SOF_TIMESTAMPING_RX_HARDWARE:
  璇锋眰鐢辩綉缁滈€傞厤鍣ㄧ敓鎴愮殑 rx 鏃堕棿鎴炽€?
SOF_TIMESTAMPING_RX_SOFTWARE:
  褰撴暟鎹繘鍏ュ唴鏍告椂璇锋眰 rx 鏃堕棿鎴炽€傝繖浜涙椂闂存埑鍦ㄨ澶囬┍鍔ㄥ皢鏁版嵁鍖呬氦缁欏唴鏍告帴鏀舵爤涔嬪悗绔嬪嵆鐢熸垚銆?
SOF_TIMESTAMPING_TX_HARDWARE:
  璇锋眰鐢辩綉缁滈€傞厤鍣ㄧ敓鎴愮殑 tx 鏃堕棿鎴炽€傝鏍囧織鍙€氳繃濂楁帴瀛楅€夐」鍜屾帶鍒舵秷鎭袱绉嶆柟寮忔潵鍚敤銆?
SOF_TIMESTAMPING_TX_SOFTWARE:
  褰撴暟鎹寮€鍐呮牳鏃惰姹?tx 鏃堕棿鎴炽€傝繖浜涙椂闂存埑鍦ㄨ澶囬┍鍔ㄤ腑灏藉彲鑳芥帴杩戯紙浣嗘€绘槸鍦ㄤ箣鍓嶏級灏嗘暟鎹寘浼犻€掔粰缃戠粶鎺ュ彛鏃剁敓鎴愩€傚洜姝わ紝瀹冧滑闇€瑕侀┍鍔ㄦ敮鎸侊紝骞堕潪鎵€鏈夎澶囬兘鍙敤銆傝鏍囧織鍙€氳繃濂楁帴瀛楅€夐」鍜屾帶鍒舵秷鎭袱绉嶆柟寮忔潵鍚敤銆?
SOF_TIMESTAMPING_TX_SCHED:
  鍦ㄨ繘鍏ユ暟鎹寘璋冨害鍣ㄤ箣鍓嶈姹?tx 鏃堕棿鎴炽€傚鏋滆緝闀匡紝鍐呮牳鍙戦€佸欢杩熼€氬父鐢辨帓闃熷欢杩熶富瀵笺€傝鏃堕棿鎴充笌鍦?SOF_TIMESTAMPING_TX_SOFTWARE 澶勮幏鍙栫殑鏃堕棿鎴充箣闂寸殑宸€硷紝浼氭毚闇茶寤惰繜锛堜笌鍗忚澶勭悊鏃犲叧锛夈€傚崗璁鐞嗘墍浜х敓鐨勫欢杩燂紙濡傛灉鏈夌殑璇濓級鍙互閫氳繃灏嗘湰鏃堕棿鎴冲噺鍘?send() 涔嬪墠绔嬪嵆鑾峰彇鐨勭敤鎴风┖闂存椂闂存埑鏉ヨ绠椼€傚湪甯︽湁铏氭嫙璁惧鐨勬満鍣ㄤ笂锛屽彂閫佺殑鍖呬細缁忚繃澶氫釜璁惧銆佷粠鑰岀粡杩囧涓暟鎹寘璋冨害鍣紝姣忎竴灞傞兘浼氱敓鎴愪竴涓椂闂存埑銆傝繖鍏佽瀵规帓闃熷欢杩熻繘琛岀粏绮掑害娴嬮噺銆傝鏍囧織鍙€氳繃濂楁帴瀛楅€夐」鍜屾帶鍒舵秷鎭袱绉嶆柟寮忔潵鍚敤銆?
SOF_TIMESTAMPING_TX_ACK:
  褰撳彂閫佺紦鍐插尯涓殑鎵€鏈夋暟鎹兘宸茶纭鏃惰姹?tx 鏃堕棿鎴炽€傝繖浠呭鍙潬鍗忚鏈夋剰涔夈€傜洰鍓嶄粎閽堝 TCP 瀹炵幇銆傚浜庤鍗忚锛屽畠鍙兘浼氶珮浼版祴閲忕粨鏋滐紝鍥犱负鏃堕棿鎴虫槸鍦?send() 澶勭紦鍐插尯涓洿鑷筹紙鍚級璇ヤ綅缃殑鎵€鏈夋暟鎹兘琚‘璁ゆ椂鐢熸垚鐨勶細鍗崇疮绉‘璁わ紙cumulative acknowledgment锛夈€傝鏈哄埗蹇界暐 SACK 鍜?FACK銆傝鏍囧織鍙€氳繃濂楁帴瀛楅€夐」鍜屾帶鍒舵秷鎭袱绉嶆柟寮忔潵鍚敤銆?
SOF_TIMESTAMPING_TX_COMPLETION:
  鍦ㄦ暟鎹寘鍙戦€佸畬鎴愭椂璇锋眰 tx 鏃堕棿鎴炽€傚畬鎴愭椂闂存埑鐢卞唴鏍稿湪鏀跺埌纭欢鐨勫彂閫佸畬鎴愭姤鍛婃椂鐢熸垚銆傜‖浠跺彲鑳戒竴娆℃姤鍛婂涓暟鎹寘锛岃€屽畬鎴愭椂闂存埑鍙嶆槧鐨勬槸鎶ュ憡鐨勬椂闂达紝鑰岄潪瀹為檯鐨勫彂閫佹椂闂淬€傝鏍囧織鍙€氳繃濂楁帴瀛楅€夐」鍜屾帶鍒舵秷鎭袱绉嶆柟寮忔潵鍚敤銆?

##### 1.3.2 鏃堕棿鎴虫姤鍛?

鍙﹀涓変釜姣旂壒浣嶆帶鍒跺摢浜涙椂闂存埑浼氬湪鐢熸垚鐨勬帶鍒舵秷鎭腑琚姤鍛娿€傚杩欎簺姣旂壒浣嶇殑鏇存敼浼氬湪鏍堜腑鐨勬椂闂存埑鎶ュ憡浣嶇疆绔嬪嵆鐢熸晥銆傛椂闂存埑浠呭鍚屾椂璁剧疆浜嗙浉鍏虫椂闂存埑鐢熸垚璇锋眰鐨勬暟鎹寘杩涜鎶ュ憡銆?
SOF_TIMESTAMPING_SOFTWARE:
  鍦ㄥ彲鐢ㄦ椂鎶ュ憡浠讳綍杞欢鏃堕棿鎴炽€?
SOF_TIMESTAMPING_SYS_HARDWARE:
  璇ラ€夐」宸插純鐢ㄥ苟琚拷鐣ャ€?
SOF_TIMESTAMPING_RAW_HARDWARE:
  鍦ㄥ彲鐢ㄦ椂鎶ュ憡鐢?SOF_TIMESTAMPING_TX_HARDWARE 鎴?SOF_TIMESTAMPING_RX_HARDWARE 鐢熸垚鐨勭‖浠舵椂闂存埑銆?

##### 1.3.3 鏃堕棿鎴抽€夐」


璇ユ帴鍙ｆ敮鎸佷互涓嬮€夐」锛?
SOF_TIMESTAMPING_OPT_ID:
  涓烘瘡涓暟鎹寘鐢熸垚涓€涓敮涓€鏍囪瘑绗︺€備竴涓繘绋嬪彲浠ユ湁澶氫釜骞跺彂鏈畬鎴愮殑鏃堕棿鎴宠姹傘€傛暟鎹寘鍦ㄥ彂閫佽矾寰勪笂鍙兘琚噸鏂版帓搴忥紝渚嬪鍦ㄦ暟鎹寘璋冨害鍣ㄤ腑銆傚湪杩欑鎯呭喌涓嬶紝鏃堕棿鎴充細鎸夌収涓庡師濮?send() 璋冪敤涓嶅悓鐨勯『搴忔帓闃熷埌閿欒闃熷垪涓€傛鏃讹紝浠呴潬鏃堕棿鎴抽『搴忔垨鏈夋晥杞借嵎妫€鏌ワ紝骞朵笉鎬绘槸鑳藉鍞竴鍦板皢鏃堕棿鎴充笌鍘熷鐨?send() 璋冪敤瀵瑰簲璧锋潵銆?
  璇ラ€夐」灏?send() 鏃剁殑姣忎釜鏁版嵁鍖呬笌涓€涓敮涓€鏍囪瘑绗﹀叧鑱旓紝骞惰繛鍚屾椂闂存埑涓€璧疯繑鍥炶鏍囪瘑绗︺€傛爣璇嗙娲剧敓鑷竴涓瘡濂楁帴瀛?u32 璁℃暟鍣紙浼氬洖缁曪級銆傚浜庢暟鎹姤濂楁帴瀛楋紝璁℃暟鍣ㄦ瘡鍙戦€佷竴涓暟鎹寘閫掑锛涘浜庢祦濂楁帴瀛楋紝姣忓彂閫佷竴涓瓧鑺傞€掑銆傚浜庢祦濂楁帴瀛楋紝杩樺簲璁剧疆 SOF_TIMESTAMPING_OPT_ID_TCP锛岃涓嬫枃銆?
  璁℃暟鍣ㄤ粠闆跺紑濮嬨€傚畠鍦ㄥ鎺ュ瓧閫夐」棣栨鍚敤鏃跺垵濮嬪寲銆傛瘡娆″湪绂佺敤鍚庨噸鏂板惎鐢ㄨ閫夐」鏃讹紝璁℃暟鍣ㄩ兘浼氶噸缃€傞噸缃鏁板櫒涓嶄細鏀瑰彉绯荤粺涓凡鏈夋暟鎹寘鐨勬爣璇嗙銆?
  璇ラ€夐」浠呴拡瀵瑰彂閫佹椂闂存埑瀹炵幇銆傚湪閭ｉ噷锛屾椂闂存埑鎬绘槸涓?struct sock_extended_err 涓€璧峰洖鐜€傝閫夐」淇敼 ee_data 瀛楁锛屼互浼犻€掍竴涓湪璇ュ鎺ュ瓧鎵€鏈夊彲鑳藉苟鍙戞湭瀹屾垚鐨勬椂闂存埑璇锋眰涓敮涓€鐨?id銆?
  杩涚▼鍙互閫夋嫨鎬у湴瑕嗙洊榛樿鐢熸垚鐨?ID锛屾柟娉曟槸浣跨敤鎺у埗娑堟伅 SCM_TS_OPT_ID 浼犻€掍竴涓壒瀹氱殑 ID锛堜笉

```

    struct msghdr *msg;
    ...
    cmsg			 = CMSG_FIRSTHDR(msg);
    cmsg->cmsg_level		 = SOL_SOCKET;
    cmsg->cmsg_type		 = SCM_TS_OPT_ID;
    cmsg->cmsg_len		 = CMSG_LEN(sizeof(__u32));
    *((__u32 *) CMSG_DATA(cmsg)) = opt_id;
    err = sendmsg(fd, msg, 0);

```

SOF_TIMESTAMPING_OPT_ID_TCP:
  瀵逛簬鏂扮殑 TCP 鏃堕棿鎴冲簲鐢紝灏嗘淇グ绗︿笌 SOF_TIMESTAMPING_OPT_ID 涓€璧蜂紶閫掋€係OF_TIMESTAMPING_OPT_ID 瀹氫箟浜嗘祦濂楁帴瀛楃殑璁℃暟鍣ㄥ浣曢€掑锛屼絾鍏惰捣濮嬬偣骞堕潪瀹屽叏绠€鍗曘€傝閫夐」淇浜嗚繖涓€鐐广€?
  瀵逛簬娴佸鎺ュ瓧锛屽鏋滆缃簡 SOF_TIMESTAMPING_OPT_ID锛屼篃搴斿缁堣缃閫夐」銆傚浜庢暟鎹姤濂楁帴瀛楋紝璇ラ€夐」鏃犳晥銆?
  涓€涓悎鐞嗙殑棰勬湡鏄紝璁℃暟鍣ㄩ殢绯荤粺璋冪敤閲嶇疆涓洪浂锛屼娇寰楅殢鍚庡啓鍏?N 瀛楄妭鐨?write() 鐢熸垚璁℃暟鍣ㄤ负 N-1 鐨勬椂闂存埑銆係OF_TIMESTAMPING_OPT_ID_TCP 鍦ㄦ墍鏈夋儏鍐典笅閮藉疄鐜颁簡杩欎竴琛屼负銆?
  涓嶅甫淇グ绗︾殑 SOF_TIMESTAMPING_OPT_ID 閫氬父鎶ュ憡鐩稿悓鐨勭粨鏋滐紝灏ゅ叾鏄綋濂楁帴瀛楅€夐」鍦ㄦ病鏈夋暟鎹紶杈撴椂璁剧疆銆傚鏋滄鍦ㄤ紶杈撴暟鎹紝瀹冨彲鑳戒細鍋忓樊杈撳嚭闃熷垪鐨勯暱搴︼紙SIOCOUTQ锛夈€?
  杩欎竴宸紓婧愪簬鍩轰簬 snd_una 杩樻槸 write_seq銆俿nd_una 鏄凡琚绔‘璁ょ殑娴佷腑鐨勫亸绉婚噺锛屽畠鍙栧喅浜庤繘绋嬫帶鍒朵箣澶栫殑鍥犵礌锛堜緥濡傜綉缁?RTT锛夈€倃rite_seq 鏄繘绋嬪啓鍏ョ殑鏈€鍚庝竴涓瓧鑺傦紝璇ュ亸绉婚噺涓嶅彈澶栭儴杈撳叆褰卞搷銆?
  褰撳湪鍒濆鍒涘缓濂楁帴瀛楁椂閰嶇疆锛堟鏃舵病鏈夋暟鎹帓闃熸垨鍙戦€侊級锛岃繖涓€宸紓寰堝井濡欙紝涓嶅お鍙兘琚敞鎰忓埌銆備絾鏃犺浣曟椂璁剧疆濂楁帴瀛楅€夐」锛孲OF_TIMESTAMPING_OPT_ID_TCP 鐨勮涓洪兘鏇寸ǔ鍋ャ€?
SOF_TIMESTAMPING_OPT_CMSG:
  涓烘墍鏈夊甫鏃堕棿鎴崇殑鏁版嵁鍖呮敮鎸?recv() cmsg銆傚浜庡甫鎺ユ敹鏃堕棿鎴崇殑鎵€鏈夋暟鎹寘浠ュ強甯﹀彂閫佹椂闂存埑鐨?IPv6 鏁版嵁鍖咃紝鎺у埗娑堟伅宸叉棤鏉′欢鏀寔銆傝閫夐」灏嗗叾鎵╁睍鍒板甫鍙戦€佹椂闂存埑鐨?IPv4 鏁版嵁鍖呫€備竴涓敤渚嬫槸閫氳繃鍚屾椂鍚敤濂楁帴瀛楅€夐」 IP_PKTINFO锛屽皢鏁版嵁鍖呬笌鍏跺嚭鍙ｈ澶囧叧鑱旇捣鏉ャ€?

SOF_TIMESTAMPING_OPT_TSONLY:
  浠呴€傜敤浜庡彂閫佹椂闂存埑銆備娇鍐呮牳灏嗘椂闂存埑浣滀负 cmsg 涓庣┖鏁版嵁鍖咃紙鑰屼笉鏄師濮嬫暟鎹寘锛変竴璧疯繑鍥炪€傝繖鍑忓皯浜嗚鍏ュ鎺ュ瓧鎺ユ敹棰勭畻锛圫O_RCVBUF锛夌殑鍐呭瓨閲忥紝骞朵笖鍗充娇 sysctl net.core.tstamp_allow_data 涓?0 涔熻兘浜や粯鏃堕棿鎴炽€傝閫夐」浼氱鐢?SOF_TIMESTAMPING_OPT_CMSG銆?
SOF_TIMESTAMPING_OPT_STATS:
  涓庡彂閫佹椂闂存埑涓€璧疯幏鍙栫殑鍙€夌粺璁′俊鎭€傚畠蹇呴』涓?SOF_TIMESTAMPING_OPT_TSONLY 涓€璧蜂娇鐢ㄣ€傚綋鍙戦€佹椂闂存埑鍙敤鏃讹紝缁熻淇℃伅浠ョ被鍨嬩负 SCM_TIMESTAMPING_OPT_STATS 鐨勭嫭绔嬫帶鍒舵秷鎭舰寮忔彁渚涳紝浣滀负涓€缁?TLV锛坰truct nlattr锛夌被鍨嬨€傝繖浜涚粺璁′俊鎭厑璁稿簲鐢ㄧ▼搴忓皢鍚勭浼犺緭灞傜粺璁′笌鍙戦€佹椂闂存埑鍏宠仈璧锋潵锛屼緥濡傛煇鍧楁暟鎹瀵圭鎺ユ敹绐楀彛闄愬埗鐨勬椂闂撮暱搴︺€?
SOF_TIMESTAMPING_OPT_PKTINFO:
  涓哄甫鏈夌‖浠舵椂闂存埑鐨勫叆绔欐暟鎹寘鍚敤 SCM_TIMESTAMPING_PKTINFO 鎺у埗娑堟伅銆傝娑堟伅鍖呭惈 struct scm_ts_pktinfo锛屾彁渚涙帴鏀惰鏁版嵁鍖呯殑鐪熷疄鎺ュ彛鐨勭储寮曞強鍏跺湪浜屽眰鐨勯暱搴︺€備粎褰撳惎鐢ㄤ簡 CONFIG_NET_RX_BUSY_POLL 涓旈┍鍔ㄤ娇鐢?NAPI 鏃讹紝鎵嶄細杩斿洖鏈夋晥鐨勶紙闈為浂锛夋帴鍙ｇ储寮曘€傝缁撴瀯浣撹繕鍖呭惈涓や釜鍏朵粬瀛楁锛屼絾瀹冧滑鏄繚鐣欎笖鏈畾涔夌殑銆?
SOF_TIMESTAMPING_OPT_TX_SWHW:
  褰撳悓鏃跺惎鐢?SOF_TIMESTAMPING_TX_HARDWARE 鍜?SOF_TIMESTAMPING_TX_SOFTWARE 鏃讹紝涓哄嚭绔欐暟鎹寘鍚屾椂璇锋眰纭欢鍜岃蒋浠舵椂闂存埑銆傚鏋滀袱绉嶆椂闂存埑閮界敓鎴愶紝灏嗘湁涓や釜鐙珛鐨勬秷鎭洖鐜埌濂楁帴瀛楃殑閿欒闃熷垪锛屾瘡涓秷鎭彧鍖呭惈涓€涓椂闂存埑銆?
SOF_TIMESTAMPING_OPT_RX_FILTER:
  杩囨护鎺夎櫄鍋囩殑鎺ユ敹鏃堕棿鎴筹細浠呭綋鍚敤浜嗗尮閰嶇殑鏃堕棿鎴崇敓鎴愭爣蹇楁椂鎵嶆姤鍛婃帴鏀舵椂闂存埑銆?
  鎺ユ敹鏃堕棿鎴冲湪鍏ュ彛璺緞鐨勬棭鏈熴€佸湪鏁版嵁鍖呯殑鐩爣濂楁帴瀛楀凡鐭ヤ箣鍓嶇敓鎴愩€傚鏋滀换浣曞鎺ュ瓧鍚敤浜嗘帴鏀舵椂闂存埑锛屾墍鏈夊鎺ュ瓧鐨勬暟鎹寘閮戒細鏀跺埌甯︽椂闂存埑鐨勬暟鎹寘銆傚寘鎷偅浜涜姹傞€氳繃 SOF_TIMESTAMPING_SOFTWARE 鍜?鎴?SOF_TIMESTAMPING_RAW_HARDWARE 鎶ュ憡鏃堕棿鎴炽€佷絾涓嶈姹傜敓鎴愭帴鏀舵椂闂存埑鐨勫鎺ュ瓧銆傝繖鍙兘鍦ㄤ粎璇锋眰鍙戦€佹椂闂存埑鏃跺彂鐢熴€?
  鎺ユ敹铏氬亣鏃堕棿鎴抽€氬父鏃犲銆傝繘绋嬪彲浠ュ拷鐣ユ剰澶栫殑闈為浂鍊笺€備絾杩欎細浣胯涓哄井濡欏湴渚濊禆浜庡叾浠栧鎺ュ瓧銆傝鏍囧織闅旂浜嗗鎺ュ瓧锛屼互鑾峰緱鏇村叿纭畾鎬х殑琛屼负銆?
榧撳姳鏂板簲鐢ㄧ▼搴忎紶閫?SOF_TIMESTAMPING_OPT_ID 浠ユ秷闄ゆ椂闂存埑鐨勬涔夛紝骞朵紶閫?SOF_TIMESTAMPING_OPT_TSONLY 浠ュ湪鏃犺 sysctl net.core.tstamp_allow_data 濡備綍璁剧疆鐨勬儏鍐典笅閮借兘宸ヤ綔銆?
涓€绉嶄緥澶栨儏鍐垫槸褰撹繘绋嬮渶瑕侀澶栫殑 cmsg 鏁版嵁鏃讹紝渚嬪浣跨敤 SOL_IP/IP_PKTINFO 鏉ユ娴嬪嚭鍙ｇ綉缁滄帴鍙ｃ€傛鏃跺簲浼犻€掗€夐」 SOF_TIMESTAMPING_OPT_CMSG銆傝閫夐」渚濊禆浜庤兘澶熻闂師濮嬫暟鎹寘鐨勫唴瀹癸紝鍥犳鏃犳硶涓?SOF_TIMESTAMPING_OPT_TSONLY 缁勫悎浣跨敤銆?

##### 1.3.4. 閫氳繃鎺у埗娑堟伅鍚敤鏃堕棿鎴?

闄や簡濂楁帴瀛楅€夐」澶栵紝杩樺彲浠ラ拡瀵规瘡娆″啓鍏ラ€氳繃 cmsg 璇锋眰鏃堕棿鎴崇敓鎴愶紝浣嗕粎閫傜敤浜?SOF_TIMESTAMPING_TX_*锛堣绗?1.3.1 鑺傦級銆備娇鐢ㄨ鐗规€э紝搴旂敤绋嬪簭鍙互閽堝姣忎釜 sendmsg() 閲囨牱鏃堕棿鎴筹紝鑰屾棤闇€鎵挎媴閫氳繃

```

  struct msghdr *msg;
  ...
  cmsg			       = CMSG_FIRSTHDR(msg);
  cmsg->cmsg_level	       = SOL_SOCKET;
  cmsg->cmsg_type	       = SO_TIMESTAMPING;
  cmsg->cmsg_len	       = CMSG_LEN(sizeof(__u32));
  *((__u32 *) CMSG_DATA(cmsg)) = SOF_TIMESTAMPING_TX_SCHED |
				 SOF_TIMESTAMPING_TX_SOFTWARE |
				 SOF_TIMESTAMPING_TX_ACK;
  err = sendmsg(fd, msg, 0);

```

鍚敤鍜岀鐢ㄦ椂闂存埑鑰屽甫鏉ョ殑寮€閿€銆?
閫氳繃 cmsg 璁剧疆鐨?SOF_TIMESTAMPING_TX_* 鏍囧織灏嗚鐩栭€氳繃 setsockopt 璁剧疆鐨?SOF_TIMESTAMPING_TX_* 鏍囧織銆?
姝ゅ锛屽簲鐢ㄧ▼搴忎粛蹇呴』閫氳繃

```

  __u32 val = SOF_TIMESTAMPING_SOFTWARE |
	      SOF_TIMESTAMPING_OPT_ID /* or any other flag */;
  err = setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val, sizeof(val));

```

鍚敤鏃堕棿鎴虫姤鍛娿€?
### 1.4 瀛楄妭娴佹椂闂存埑


SO_TIMESTAMPING 鎺ュ彛鏀寔瀵瑰瓧鑺傛祦涓殑瀛楄妭鎵撴椂闂存埑銆傛瘡涓姹傝瑙ｉ噴涓猴細褰撶紦鍐插尯鐨勫叏閮ㄥ唴瀹归€氳繃鏌愪釜鏃堕棿鎴崇偣鏃惰褰曚竴娆°€備篃灏辨槸璇达紝瀵逛簬娴侊紝閫夐」 SOF_TIMESTAMPING_TX_SOFTWARE 浼氳褰曟墍鏈夊瓧鑺備綍鏃跺埌杈捐澶囬┍鍔紝鑰屼笉绠℃暟鎹杞崲鎴愪簡澶氬皯涓暟鎹寘銆?
涓€鑸€岃█锛屽瓧鑺傛祦娌℃湁鑷劧鐨勫垎鐣岀锛屽洜姝ゅ皢鏃堕棿鎴充笌鏁版嵁鍏宠仈璧锋潵骞堕潪鏄撲簨銆備竴涓瓧鑺傝寖鍥村彲鑳借鍒嗗壊鍒板涓涓紝浠讳綍娈甸兘鍙兘琚悎骞讹紙鍙兘灏嗕箣鍓嶄笌鐙珛 send() 璋冪敤鍏宠仈鐨勫凡鍒嗘缂撳啿鍖哄尯娈靛悎骞讹級銆傛鍙兘琚噸鏂版帓搴忥紝瀵逛簬瀹炵幇浜嗛噸浼犵殑鍗忚锛屽悓涓€瀛楄妭鑼冨洿鍙兘鍚屾椂瀛樺湪浜庡涓涓€?
鎵€鏈夋椂闂存埑閮藉繀椤诲疄鐜扮浉鍚岀殑璇箟锛屾棤璁鸿繖浜涘彲鑳界殑杞崲濡備綍锛屽惁鍒欏畠浠棤娉曠浉浜掓瘮杈冦€備互涓嶅悓浜庣畝鍗曟儏鍐碉紙缂撳啿鍖哄埌 skb 鐨?1:1 鏄犲皠锛夌殑鏂瑰紡澶勭悊"缃曡"鐨勮竟鐣屾儏鍐垫槸涓嶅鐨勶紝鍥犱负鎬ц兘璋冭瘯寰€寰€闇€瑕佸叧娉ㄦ绫荤缇ゅ€笺€?
鍦ㄥ疄璺典腑锛屽鏋滄椂闂存埑鐨勮涔夊拰娴嬮噺鐨勬椂鏈洪兘閫夋嫨姝ｇ‘锛屾椂闂存埑灏卞彲浠ヤ笌瀛楄妭娴佺殑娈典竴鑷村湴鍏宠仈璧锋潵銆傝繖涓€鎸戞垬涓庝负 IP 鍒嗙墖鍐冲畾绛栫暐骞舵棤涓嶅悓銆傚湪 IP 鍒嗙墖涓紝瀹氫箟鏄彧鏈夌涓€涓垎鐗囪鎵撴椂闂存埑銆傚浜庡瓧鑺傛祦锛屾垜浠€夋嫨鍙湪鎵€鏈夊瓧鑺傞兘閫氳繃鏌愪竴鐐规椂鎵嶇敓鎴愭椂闂存埑銆傛墍瀹氫箟鐨?SOF_TIMESTAMPING_TX_ACK 鏄撲簬瀹炵幇鍜屾帹鐞嗐€備竴涓渶瑕佽€冭檻 SACK 鐨勫疄鐜颁細鏇村鏉傦紝鍥犱负鍙兘瀛樺湪浼犺緭绌烘礊鍜屼贡搴忓埌杈俱€?
鍦ㄤ富鏈轰笂锛岀敱浜?Nagle銆乧ork銆乤utocork銆佸垎娈靛拰 GSO锛孴CP 涔熷彲鑳界牬鍧忎粠缂撳啿鍖哄埌 skbuff 鐨勭畝鍗?1:1 鏄犲皠銆傚疄鐜伴€氳繃璺熻釜浼犻€掔粰 send() 鐨勫悇涓渶鍚庝竴涓瓧鑺傛潵纭繚鍚勭鎯呭喌涓嬬殑姝ｇ‘鎬э紝鍗充娇瀹冨湪 skbuff 鎵╁睍鎴栧悎骞舵搷浣滀箣鍚庝笉鍐嶆槸鏈€鍚庝竴涓瓧鑺傘€傚畠灏嗙浉鍏崇殑搴忓垪鍙峰瓨鍌ㄥ湪 skb_shinfo(skb)->tskey 涓€傜敱浜庝竴涓?skbuff 鍙湁涓€涓繖鏍风殑瀛楁锛屽洜姝ゅ彧鑳界敓鎴愪竴涓椂闂存埑銆?
鍦ㄦ瀬灏戞暟鎯呭喌涓嬶紝濡傛灉涓や釜璇锋眰琚悎骞跺埌鍚屼竴涓?skb 涓婏紝鍙兘浼氭紡鎺変竴涓椂闂存埑璇锋眰銆傝繘绋嬪彲浠ラ€氳繃鍚敤 SOF_TIMESTAMPING_OPT_ID锛屽苟灏嗗彂閫佹椂鐨勫瓧鑺傚亸绉婚噺涓庝负姣忎釜鏃堕棿鎴宠繑鍥炵殑鍊艰繘琛屾瘮杈冿紝鏉ユ娴嬭繖绉嶆儏鍐点€傚畠鍙互閫氳繃鍦ㄨ姹備箣闂村缁堝埛鏂?TCP 鏍堬紙渚嬪鍚敤 TCP_NODELAY 骞剁鐢?TCP_CORK 鍜?autocork锛夋潵闃叉杩欑鎯呭喌銆傚湪 linux-4.7 涔嬪悗锛岄槻姝㈠悎骞剁殑鏇村ソ鏂规硶鏄湪 sendmsg() 鏃朵娇鐢?MSG_EOR 鏍囧織銆?
杩欎簺棰勯槻鎺柦纭繚浠呭湪鎵€鏈夊瓧鑺傞兘閫氳繃鏃堕棿鎴崇偣鏃舵墠鐢熸垚鏃堕棿鎴斥€斺€斿墠鎻愭槸缃戠粶鏍堟湰韬笉浼氬娈甸噸鏂版帓搴忋€傛爤纭疄浼氬敖閲忛伩鍏嶉噸鏂版帓搴忋€傚敮涓€鐨勪緥澶栧湪绠＄悊鍛樻帶鍒朵箣涓嬶細鍙互鏋勯€犱竴涓暟鎹寘璋冨害鍣ㄩ厤缃紝浠ヤ笉鍚屾柟寮忓欢杩熸潵鑷悓涓€娴佺殑娈点€傝繖鏍风殑閰嶇疆杈冧负缃曡銆?

## 2 鏁版嵁鎺ュ彛


鏃堕棿鎴抽€氳繃 recvmsg() 鐨勮緟鍔╂暟鎹紙ancillary data锛夌壒鎬ц鍙栥€傛湁鍏宠鎺ュ彛鐨勮鎯呰鍙傝 `man 3 cmsg`銆傚鎺ュ瓧鎵嬪唽椤碉紙`man 7 socket`锛夋弿杩颁簡濡備綍鑾峰彇鐢?SO_TIMESTAMP 鍜?SO_TIMESTAMPNS 鐢熸垚鐨勬椂闂存埑璁板綍銆?

### 2.1 SCM_TIMESTAMPING 璁板綍


杩欎簺鏃堕棿鎴冲湪涓€鏉℃帶鍒舵秷鎭腑杩斿洖锛屽叾 cmsg_level 涓?SOL_SOCKET锛宑msg_type 涓?SCM_TIMESTAMPING锛岃礋杞界被鍨嬩负

```

	struct scm_timestamping {
		struct timespec ts[3];
	};

```

```

	struct scm_timestamping64 {
		struct __kernel_timespec ts[3];

```

濮嬬粓浣跨敤 SO_TIMESTAMPING_NEW 鏃堕棿鎴筹紝浠ュ缁堣幏寰?struct scm_timestamping64 鏍煎紡鐨勬椂闂存埑銆?
鍦?32 浣嶆満鍣ㄤ笂锛孲O_TIMESTAMPING_OLD 鍦?2038 骞翠箣鍚庝細杩斿洖閿欒鐨勬椂闂存埑銆?
璇ョ粨鏋勪綋鏈€澶氬彲杩斿洖涓変釜鏃堕棿鎴炽€傝繖鏄竴涓仐鐣欑壒鎬с€備换浣曟椂鍒昏嚦灏戞湁涓€涓瓧娈甸潪闆躲€傚ぇ澶氭暟鏃堕棿鎴抽€氳繃 ts[^0^] 浼犻€掋€傜‖浠舵椂闂存埑閫氳繃 ts[^2^] 浼犻€掋€?
ts[^1^] 杩囧幓鐢ㄤ簬淇濆瓨杞崲涓虹郴缁熸椂闂寸殑纭欢鏃堕棿鎴炽€傜幇鍦ㄦ敼涓哄皢 NIC 涓婄殑纭欢鏃堕挓璁惧鐩存帴浣滀负 HW PTP 鏃堕挓婧愭毚闇插嚭鏉ワ紝浠ヤ究鍦ㄧ敤鎴风┖闂磋繘琛屾椂闂磋浆鎹紝骞跺彲閫夊湴閫氳繃 linuxptp 绛夌敤鎴风┖闂?PTP 鏍堟潵鍚屾绯荤粺鏃堕棿銆傛湁鍏?PTP 鏃堕挓 API锛岃鍙傝 Documentation/driver-api/ptp.rst銆?
璇锋敞鎰忥紝濡傛灉 SO_TIMESTAMP 鎴?SO_TIMESTAMPNS 閫夐」涓?SO_TIMESTAMPING锛堜娇鐢?SOF_TIMESTAMPING_SOFTWARE锛変竴璧峰惎鐢紝閭ｄ箞鍦ㄧ己灏戠湡瀹炶蒋浠舵椂闂存埑鏃讹紝recvmsg() 璋冪敤浼氱敓鎴愪竴涓櫄鍋囩殑杞欢鏃堕棿鎴筹紝骞堕€氳繃 ts[^0^] 浼犻€掋€傝繖绉嶆儏鍐靛湪纭欢鍙戦€佹椂闂存埑涓婁篃浼氬彂鐢熴€?
##### 2.1.1 浣跨敤 MSG_ERRQUEUE 鐨勫彂閫佹椂闂存埑


瀵逛簬鍙戦€佹椂闂存埑锛屽嚭绔欐暟鎹寘浼氳繛鍚屽彂閫佹椂闂存埑涓€璧峰洖鐜埌濂楁帴瀛楃殑閿欒闃熷垪銆傝繘绋嬮€氳繃璋冪敤璁剧疆浜?MSG_ERRQUEUE 鏍囧織鐨?recvmsg()锛屽苟鎻愪緵瓒冲澶т互鎺ユ敹鐩稿叧鍏冩暟鎹粨鏋勭殑 msg_control 缂撳啿鍖猴紝鏉ユ帴鏀惰繖浜涙椂闂存埑銆俽ecvmsg 璋冪敤杩斿洖鍘熷鐨勫嚭绔欐暟鎹寘锛屽苟闄勫甫涓ゆ潯杈呭姪娑堟伅銆?
涓€鏉?cm_level 涓?SOL_IP(V6)銆乧m_type 涓?IP(V6)_RECVERR 鐨勬秷鎭唴宓屼簡涓€涓?struct sock_extended_err銆傚畠瀹氫箟浜嗛敊璇被鍨嬨€傚浜庢椂闂存埑锛宔e_errno 瀛楁涓?ENOMSG銆傚彟涓€鏉¤緟鍔╂秷鎭殑 cm_level 涓?SOL_SOCKET锛宑m_type 涓?SCM_TIMESTAMPING銆傚畠鍐呭祵浜?struct scm_timestamping銆?

#### 2.1.1.2 鏃堕棿鎴崇被鍨?

杩欎笁涓?struct timespec 鐨勮涔夌敱鎵╁睍閿欒缁撴瀯浣撲腑鐨?ee_info 瀛楁瀹氫箟銆傚畠鍖呭惈涓€涓?SCM_TSTAMP_* 绫诲瀷鐨勫€硷紝鐢ㄤ簬瀹氫箟浼犲叆 scm_timestamping 鐨勫疄闄呮椂闂存埑銆?
SCM_TSTAMP_** 绫诲瀷涓庡墠闈㈣璁虹殑 SOF_TIMESTAMPING_** 鎺у埗瀛楁涓€涓€瀵瑰簲锛屽彧鏈変竴涓緥澶栥€傚嚭浜庨仐鐣欏師鍥狅紝SCM_TSTAMP_SND 绛変簬闆讹紝骞朵笖鍙互鍚屾椂涓?SOF_TIMESTAMPING_TX_HARDWARE 鍜?SOF_TIMESTAMPING_TX_SOFTWARE 璁剧疆銆傚鏋?ts[^2^] 闈為浂锛屽垯涓哄墠鑰咃紱鍚﹀垯涓哄悗鑰咃紝姝ゆ椂鏃堕棿鎴冲瓨鍌ㄥ湪 ts[^0^] 涓€?

#### 2.1.1.3 鍒嗙墖


鍑虹珯鏁版嵁鎶ョ殑鍒嗙墖寰堝皯瑙侊紝浣嗘湁鍙兘鍙戠敓锛屼緥濡傞€氳繃鏄惧紡绂佺敤 PMTU 鍙戠幇銆傚鏋滃嚭绔欐暟鎹寘琚垎鐗囷紝閭ｄ箞鍙湁绗竴涓垎鐗囦細琚墦鏃堕棿鎴冲苟杩斿洖缁欏彂閫佸鎺ュ瓧銆?

#### 2.1.1.4 鏁版嵁鍖呰礋杞?

璋冪敤搴旂敤绋嬪簭閫氬父瀵规帴鏀跺畠鏈€鍒濅紶缁欐爤鐨勫畬鏁存暟鎹寘璐熻浇涓嶆劅鍏磋叮锛氬鎺ュ瓧閿欒闃熷垪鏈哄埗鍙槸涓€绉嶆崕甯︼紙piggyback锛夋椂闂存埑鐨勬柟娉曘€傚湪杩欑鎯呭喌涓嬶紝搴旂敤绋嬪簭鍙互閫夋嫨鐢ㄦ洿灏忕殑缂撳啿鍖猴紙鐢氳嚦鍙兘闀垮害涓?0锛夎鍙栨暟鎹姤銆傝礋杞戒細鐩稿簲琚埅鏂€傜劧鑰岋紝鍦ㄨ繘绋嬪閿欒闃熷垪璋冪敤 recvmsg() 涔嬪墠锛屽畬鏁寸殑鏁版嵁鍖呬細琚帓闃燂紝鍗犵敤 SO_RCVBUF 鐨勯绠椼€?

#### 2.1.1.5 闃诲璇诲彇


浠庨敊璇槦鍒楄鍙栧缁堟槸涓€涓潪闃诲鎿嶄綔銆傝闃诲绛夊緟鏃堕棿鎴筹紝璇蜂娇鐢?poll 鎴?select銆傚鏋滈敊璇槦鍒椾笂鏈変换浣曟暟鎹氨缁紝poll() 浼氬湪 pollfd.revents 涓繑鍥?POLLERR銆傛棤闇€鍦?pollfd.events 涓紶閫掕鏍囧織銆傝鏍囧織鍦ㄨ姹傛椂琚拷鐣ャ€傚彟璇峰弬瑙?`man 2 poll`銆?

##### 2.1.2 鎺ユ敹鏃堕棿鎴?

鍦ㄦ帴鏀舵椂锛屾病鏈夌悊鐢变粠濂楁帴瀛楅敊璇槦鍒楄鍙栥€係CM_TIMESTAMPING 杈呭姪鏁版嵁鍦ㄦ甯哥殑 recvmsg() 涓殢鏁版嵁鍖呬竴璧峰彂閫併€傜敱浜庤繖涓嶆槸濂楁帴瀛楅敊璇紝瀹冧笉闄勫甫 SOL_IP(V6)/IP(V6)_RECVERROR 娑堟伅銆傚湪杩欑鎯呭喌涓嬶紝struct scm_timestamping 涓笁涓瓧娈电殑鍚箟鏄殣寮忓畾涔夌殑銆倀s[^0^] 鍦ㄨ缃椂淇濆瓨杞欢鏃堕棿鎴筹紝ts[^1^] 鍚屾牱宸插純鐢紝ts[^2^] 鍦ㄨ缃椂淇濆瓨纭欢鏃堕棿鎴炽€?

## 3. 纭欢鏃堕棿鎴抽厤缃細ETHTOOL_MSG_TSCONFIG_SET/GET


纭欢鏃堕棿鎵撴埑杩樺繀椤讳负姣忎釜棰勬湡鎵ц纭欢鏃堕棿鎵撴埑鐨勮澶囬┍鍔ㄨ繘琛屽垵濮嬪寲銆傚弬鏁板畾涔変簬

```

	struct hwtstamp_config {
		int flags;	/* no flags defined right now, must be zero */
		int tx_type;	/* HWTSTAMP_TX_* */
		int rx_filter;	/* HWTSTAMP_FILTER_* */
	};

```

鏈熸湜鐨勮涓洪€氳繃璋冪敤 tsconfig netlink 濂楁帴瀛?`ETHTOOL_MSG_TSCONFIG_SET` 浼犲叆鍐呮牳鍜岀壒瀹氳澶囥€傞殢鍚?`ETHTOOL_A_TSCONFIG_TX_TYPES`銆乣ETHTOOL_A_TSCONFIG_RX_FILTERS` 鍜?`ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS` 杩欎簺 netlink 灞炴€ц鐢ㄦ潵鐩稿簲鍦拌缃?struct hwtstamp_config銆?
`ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` netlink 宓屽灞炴€х敤浜庨€夋嫨纭欢鏃堕棿鎵撴埑鐨勬潵婧愩€傚畠鐢辫澶囨潵婧愮殑绱㈠紩鍜屾椂闂存墦鎴崇被鍨嬬殑闄愬畾绗︾粍鎴愩€?
椹卞姩鍙互鑷敱鍦颁娇鐢ㄦ瘮鎵€璇锋眰閰嶇疆鏇村鏉剧殑閰嶇疆銆傛湡鏈涢┍鍔ㄥ彧鐩存帴瀹炵幇鎵€鑳芥敮鎸佺殑鏈€閫氱敤妯″紡銆備緥濡傦紝濡傛灉纭欢鏀寔 HWTSTAMP_FILTER_PTP_V2_EVENT锛岄偅涔堝畠閫氬父搴斿缁堝皢 HWTSTAMP_FILTER_PTP_V2_L2_SYNC 绛夊悜涓婃墿灞曚负瀹冿紝鍥犱负 HWTSTAMP_FILTER_PTP_V2_EVENT 鏇撮€氱敤锛堝搴旂敤绋嬪簭涔熸洿鏈夌敤锛夈€?
鏀寔纭欢鏃堕棿鎵撴埑鐨勯┍鍔ㄥ簲浣跨敤瀹為檯鐨勩€佸彲鑳芥洿瀹芥澗鐨勯厤缃洿鏂拌缁撴瀯浣撱€傚鏋滄墍璇锋眰鐨勬暟鎹寘鏃犳硶琚墦鏃堕棿鎴筹紝鍒欎笉搴旀洿鏀逛换浣曞唴瀹癸紝骞跺簲杩斿洖 ERANGE锛堜笌 EINVAL 鐩稿锛孍INVAL 琛ㄧず鏍规湰涓嶆敮鎸?SIOCSHWTSTAMP锛夈€?
鍙湁鍏锋湁绠＄悊鍛樻潈闄愮殑杩涚▼鎵嶈兘鏇存敼閰嶇疆銆傜敤鎴风┖闂磋礋璐ｇ‘淇濆涓繘绋嬩簰涓嶅共鎵帮紝骞朵笖璁剧疆浼氳閲嶇疆銆?
浠讳綍杩涚▼閮藉彲浠ラ€氳繃璇锋眰 tsconfig netlink 濂楁帴瀛?`ETHTOOL_MSG_TSCONFIG_GET` 鏉ヨ鍙栧疄闄呴厤缃€?
鏃х殑閰嶇疆鏂瑰紡鏄娇鐢?ioctl(SIOCSHWTSTAMP)锛屽苟浼犲叆涓€涓寚鍚?struct ifreq 鐨勬寚閽堬紝鍏朵腑 ifr_data 鎸囧悜 struct hwtstamp_config銆倀x_type 鍜?rx_filter 鏄椹卞姩棰勬湡琛屼负鐨勬彁绀恒€傚鏋滀笉鏀寔鎵€璇锋眰鐨勫叆绔欐暟鎹寘缁嗙矑搴﹁繃婊わ紝椹卞姩鍙兘浼氫负瓒呭嚭鎵€璇锋眰绫诲瀷鐨勬暟鎹寘鎵撴椂闂存埑銆俰octl(SIOCGHWTSTAMP) 鐨勪娇鐢ㄦ柟寮忎笌 ioctl(SIOCSHWTSTAMP) 鐩稿悓銆備絾鏄紝骞堕潪鎵€鏈夐┍鍔ㄩ兘瀹炵幇浜嗗畠銆?
```

    /* possible values for hwtstamp_config->tx_type */
    enum {
	    /*
	    * no outgoing packet will need hardware time stamping;
	    * should a packet arrive which asks for it, no hardware
	    * time stamping will be done
	    */
	    HWTSTAMP_TX_OFF,

	    /*
	    * enables hardware time stamping for outgoing packets;
	    * the sender of the packet decides which are to be
	    * time stamped by setting SOF_TIMESTAMPING_TX_SOFTWARE
	    * before sending the packet
	    */
	    HWTSTAMP_TX_ON,
    };

    /* possible values for hwtstamp_config->rx_filter */
    enum {
	    /* time stamp no incoming packet at all */
	    HWTSTAMP_FILTER_NONE,

	    /* time stamp any incoming packet */
	    HWTSTAMP_FILTER_ALL,

	    /* return value: time stamp all packets requested plus some others */
	    HWTSTAMP_FILTER_SOME,

	    /* PTP v1, UDP, any kind of event packet */
	    HWTSTAMP_FILTER_PTP_V1_L4_EVENT,

	    /* for the complete list of values, please check
	    * the include file include/uapi/linux/net_tstamp.h
	    */
    };

```

### 3.1 纭欢鏃堕棿鎴冲疄鐜帮細璁惧椹卞姩


鏀寔纭欢鏃堕棿鎵撴埑鐨勯┍鍔ㄥ繀椤绘敮鎸?ndo_hwtstamp_set NDO锛屽苟鎸夌収 SIOCSHWTSTAMP 涓€鑺傛墍杩帮紝鐢ㄥ疄闄呭€兼洿鏂版墍鎻愪緵鐨?struct hwtstamp_config銆傚畠杩樺簲鏀寔 ndo_hwtstamp_get NDO 鏉ユ绱㈤厤缃€?
鎺ユ敹鏁版嵁鍖呯殑鏃堕棿鎴冲繀椤诲瓨鍌ㄥ湪 skb 涓€傝鑾峰彇 skb 鐨勫叡浜椂闂存埑缁撴瀯鐨勬寚閽堬紝璋冪敤 skb_hwtstamps()銆傜劧鍚?
```

	struct skb_shared_hwtstamps {
	    /* hardware time stamp transformed into duration
	    * since arbitrary point in time
	    */
	    ktime_t	hwtstamp;
	};

```

鍑虹珯鏁版嵁鍖呯殑鏃堕棿鎴冲簲鎸夊涓嬫柟寮忕敓鎴愶細

- 鍦?hard_start_xmit() 涓紝妫€鏌?(skb_shinfo(skb)->tx_flags & SKBTX_HW_TSTAMP) 鏄惁琚缃负闈為浂銆傚鏋滄槸锛屽垯椹卞姩搴斿綋鎵ц纭欢鏃堕棿鎵撴埑銆?- 濡傛灉瀵逛簬璇?skb 鍙涓旇璇锋眰锛屽垯閫氳繃璁剧疆鏍囧織

```

      skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS;

  You might want to keep a pointer to the associated skb for the next step
  and not free the skb. A driver not supporting hardware time stamping doesn't
  do that. A driver must never touch sk_buff::tstamp! It is used to store
  software generated time stamps by the network subsystem.
```

  澹版槑椹卞姩姝ｅ湪鎵ц鏃堕棿鎵撴埑銆?- 椹卞姩搴斿敖鍙兘鍦ㄥ皢 sk_buff 浼犻€掔粰纭欢涔嬪墠璋冪敤 skb_tx_timestamp()銆傚鏋滆姹備簡杞欢鏃堕棿鎴充笖鏃犳硶杩涜纭欢鏃堕棿鎴筹紙鏈缃?SKBTX_IN_PROGRESS锛夛紝skb_tx_timestamp() 浼氭彁渚涗竴涓蒋浠舵椂闂存埑銆?- 涓€鏃﹂┍鍔ㄥ彂閫佷簡鏁版嵁鍖呭拰/鎴栦负鍏惰幏鍙栦簡纭欢鏃堕棿鎴筹紝瀹冨氨閫氳繃璋冪敤 skb_tstamp_tx()锛屼紶鍏ュ師濮?skb 鍜屽師濮嬬‖浠舵椂闂存埑锛屽皢鏃堕棿鎴充紶鍥炪€俿kb_tstamp_tx() 浼氬厠闅嗗師濮?skb 骞舵坊鍔犳椂闂存埑锛屽洜姝ょ幇鍦ㄥ繀椤婚噴鏀惧師濮?skb銆傚鏋滆幏鍙栫‖浠舵椂闂存埑鐢变簬鏌愮鍘熷洜澶辫触锛屽垯椹卞姩涓嶅簲鍥為€€鍒拌蒋浠舵椂闂存墦鎴炽€傚叾鐞嗙敱鏄細杩欎細鍦ㄥ鐞嗘祦姘寸嚎鐨勬洿鏅氶樁娈靛彂鐢燂紝涓嶅悓浜庡叾浠栬蒋浠舵椂闂存墦鎴筹紝鍥犳鍙兘瀵艰嚧鏃堕棿鎴充箣闂村嚭鐜版剰澶栫殑宸€笺€?
### 3.2 鍫嗗彔 PTP 纭欢鏃堕挓鐨勭壒娈婃敞鎰忎簨椤?

鍦ㄦ煇浜涙儏鍐典笅锛屼竴涓暟鎹寘鐨勬暟鎹矾寰勪腑鍙兘瀛樺湪澶氫釜 PHC锛圥TP 纭欢鏃堕挓锛夈€傚唴鏍告病鏈夋樉寮忔満鍒跺厑璁哥敤鎴烽€夋嫨鍝釜 PHC 鐢ㄤ簬瀵逛互澶綉甯ф墦鏃堕棿鎴炽€傜浉鍙嶏紝鍏跺亣璁炬槸鏈€澶栧眰鐨?PHC 鎬绘槸鏈€鍙彇鐨勶紝骞朵笖鍐呮牳椹卞姩浼氬崗浣滀互瀹炵幇杩欎竴鐩爣銆傜洰鍓嶆湁 3 绉嶅爢鍙?PHC 鐨勬儏鍐碉紝璇﹁堪濡備笅锛?
##### 3.2.1 DSA锛堝垎甯冨紡浜ゆ崲鏋舵瀯锛変氦鎹㈡満


杩欎簺鏄互澶綉浜ゆ崲鏈猴紝鍏朵竴涓鍙ｈ繛鎺ュ埌锛堝畬鍏ㄤ笉鐭ユ儏鐨勶級涓绘満浠ュお缃戞帴鍙ｏ紝骞跺厖褰撳甫鏈夊彲閫夎浆鍙戝姞閫熷姛鑳界殑绔彛鍊嶅鍣ㄣ€傛瘡涓?DSA 浜ゆ崲鏈虹鍙ｅ鐢ㄦ埛鑰岃█琛ㄧ幇涓轰竴涓嫭绔嬬殑锛堣櫄鎷燂級缃戠粶鎺ュ彛锛岃€屽叾缃戠粶 I/O 鍦ㄥ簳灞傛槸闂存帴鍦伴€氳繃涓绘満鎺ュ彛鎵ц鐨勶紙鍦?TX 鏃堕噸瀹氬悜鍒颁富鏈虹鍙ｏ紝鍦?RX 鏃舵嫤鎴抚锛夈€?
褰?DSA 浜ゆ崲鏈鸿繛鎺ュ埌涓绘満绔彛鏃讹紝PTP 鍚屾蹇呯劧鍙楀埌褰卞搷锛屽洜涓轰氦鎹㈡満鐨勫彲鍙樻帓闃熷欢杩熶細鍦ㄤ富鏈虹鍙ｄ笌鍏?PTP 浼欎即涔嬮棿寮曞叆璺緞寤惰繜鎶栧姩銆傚洜姝わ紝涓€浜?DSA 浜ゆ崲鏈鸿嚜甯︽椂闂存埑鏃堕挓锛屽苟鑳藉鍦ㄨ嚜宸辩殑 MAC 涓婃墽琛岀綉缁滄椂闂存埑锛屼粠鑰屼娇璺緞寤惰繜浠呮祴閲忕嚎缂嗗拰 PHY 鐨勪紶鎾欢杩熴€侺inux 鏀寔甯︽椂闂存埑鐨?DSA 浜ゆ崲鏈猴紝骞舵毚闇蹭笌浠讳綍鍏朵粬缃戠粶鎺ュ彛鐩稿悓鐨?ABI锛堥櫎浜?DSA 鎺ュ彛鍦ㄧ綉缁?I/O 鏂归潰瀹為檯涓婃槸铏氭嫙鐨勮繖涓€鐐癸紝瀹冧滑纭疄鎷ユ湁鑷繁鐨?PHC锛夈€侱SA 浜ゆ崲鏈虹殑鎵€鏈夋帴鍙ｅ叡浜悓涓€涓?PHC 鏄吀鍨嬫儏鍐碉紝浣嗛潪寮哄埗銆?
鎸夎璁★紝浣跨敤 DSA 浜ゆ崲鏈鸿繘琛?PTP 鏃堕棿鎴充笉闇€瑕佸鍏舵墍杩炴帴鐨勪富鏈虹鍙ｉ┍鍔ㄥ仛浠讳綍鐗规畩澶勭悊銆傜劧鑰岋紝褰撲富鏈虹鍙ｄ篃鏀寔 PTP 鏃堕棿鎴虫椂锛孌SA 浼氳礋璐ｆ嫤鎴拡瀵逛富鏈虹鍙ｇ殑 `.ndo_eth_ioctl` 璋冪敤锛屽苟闃绘鍦ㄥ叾涓婂惎鐢ㄧ‖浠舵椂闂存埑鐨勫皾璇曘€傝繖鏄洜涓?SO_TIMESTAMPING API 涓嶅厑璁镐负鍚屼竴鏁版嵁鍖呬氦浠樺涓‖浠舵椂闂存埑锛屽洜姝ゅ繀椤婚樆姝?DSA 浜ゆ崲鏈虹鍙ｄ箣澶栫殑浠讳綍鍏朵粬鏂硅繖鏍峰仛銆?
鍦ㄩ€氱敤灞傦紝DSA 涓?PTP 鏃堕棿鎴虫彁渚涗簡浠ヤ笅鍩虹璁炬柦锛?
- `.port_txtstamp()`锛氬湪鍙戦€佸甫鏈夋潵鑷敤鎴风┖闂寸殑纭欢 TX 鏃堕棿鎴宠姹傜殑鏁版嵁鍖呬箣鍓嶈皟鐢ㄧ殑閽╁瓙銆傝繖鏄袱姝ワ紙two-step锛夋椂闂存埑鎵€蹇呴渶鐨勶紝鍥犱负纭欢鏃堕棿鎴冲湪瀹為檯 MAC 鍙戦€佷箣鍚庢墠鍙敤锛屾墍浠ラ┍鍔ㄥ繀椤诲噯澶囧ソ灏嗚鏃堕棿鎴充笌鍘熷鏁版嵁鍖呭叧鑱旇捣鏉ワ紝浠ヤ究灏嗗叾閲嶆柊鍏ラ槦鍒板鎺ュ瓧鐨勯敊璇槦鍒椼€備负浜嗗湪鏃堕棿鎴冲彲鐢ㄦ椂淇濆瓨鏁版嵁鍖咃紝椹卞姩鍙互璋冪敤 `skb_clone_sk`锛屽皢鍏嬮殕鎸囬拡淇濆瓨鍦?skb->cb 涓紝骞跺皢 tx skb 鍏ラ槦銆傞€氬父锛屼氦鎹㈡満浼氭湁涓€涓?PTP TX 鏃堕棿鎴冲瘎瀛樺櫒锛堟湁鏃舵槸 FIFO锛夛紝鏃堕棿鎴冲湪閭ｉ噷鍙敤銆傚湪 FIFO 鐨勬儏鍐典笅锛岀‖浠跺彲鑳藉瓨鍌?PTP 搴忓垪 ID/娑堟伅绫诲瀷/鍩熷彿涓庡疄闄呮椂闂存埑鐨勯敭鍊煎銆備负浜嗗湪绛夊緟鏃堕棿鎴崇殑闃熷垪涓殑鏁版嵁鍖呬笌瀹為檯鏃堕棿鎴充箣闂存纭叧鑱旓紝椹卞姩鍙互浣跨敤 BPF 鍒嗙被鍣紙`ptp_classify_raw`锛夋潵璇嗗埆 PTP 浼犺緭绫诲瀷锛屼娇鐢?`ptp_parse_header` 鏉ヨВ閲?PTP 澶撮儴瀛楁銆傚彲鑳藉湪璇ユ椂闂存埑鍙敤鏃惰Е鍙戜竴涓?IRQ锛屾垨鑰呴┍鍔ㄥ彲鑳戒笉寰椾笉鍦ㄨ皟鐢?`dev_queue_xmit()` 鍙戝線涓绘満鎺ュ彛鍚庤繘琛岃疆璇€備竴姝ワ紙one-step锛塗X 鏃堕棿鎴充笉闇€瑕佸厠闅嗘暟鎹寘锛屽洜涓?PTP 鍗忚涓嶉渶瑕佸悗缁秷鎭紙鍥犱负 TX 鏃堕棿鎴崇敱 MAC 宓屽叆鍒版暟鎹寘涓級锛屽洜姝ょ敤鎴风┖闂翠笉鏈熸湜甯?TX 鏃堕棿鎴崇殑鏁版嵁鍖呰閲嶆柊鍏ラ槦鍒板叾濂楁帴瀛楃殑閿欒闃熷垪銆?
- `.port_rxtstamp()`锛氬湪 RX 鏃讹紝DSA 杩愯 BPF 鍒嗙被鍣ㄦ潵璇嗗埆 PTP 浜嬩欢娑堟伅锛堜换浣曞叾浠栨暟鎹寘锛屽寘鎷?PTP 閫氱敤娑堟伅锛岄兘涓嶈鎵撴椂闂存埑锛夈€傚師濮嬶紙涔熸槸鍞竴鐨勶級鍙墦鏃堕棿鎴崇殑 skb 琚彁渚涚粰椹卞姩锛屼互渚垮畠鍦ㄦ椂闂存埑绔嬪嵆鍙敤鏃朵负鍏跺姞涓婃椂闂存埑娉ㄨВ锛屾垨鎺ㄨ繜鍒颁互鍚庛€傚湪鎺ユ敹鏃讹紝鏃堕棿鎴冲彲鑳戒互甯﹀唴鏂瑰紡鍙敤锛堥€氳繃 DSA 澶撮儴涓殑鍏冩暟鎹紝鎴栦互鍏朵粬鏂瑰紡闄勫姞鍒版暟鎹寘涓婏級锛屾垨浠ュ甫澶栨柟寮忓彲鐢紙閫氳繃鍙︿竴涓?RX 鏃堕棿鎴?FIFO锛夈€傚湪 RX 涓婃帹杩熼€氬父鏄繀瑕佺殑锛屽綋鑾峰彇鏃堕棿鎴抽渶瑕佷竴涓彲浼戠湢鐨勪笂涓嬫枃鏃躲€傚湪杩欑鎯呭喌涓嬶紝鐢?DSA 椹卞姩璐熻矗鍦ㄥ垰鎵撲笂鏃堕棿鎴崇殑 skb 涓婅皟鐢?`netif_rx()`銆?
##### 3.2.2 浠ュお缃?PHY


杩欎簺鏄€氬父鍦ㄧ綉缁滄爤涓壆婕旂 1 灞傝鑹茬殑璁惧锛屽洜姝ゅ畠浠笉鍍?DSA 浜ゆ崲鏈洪偅鏍锋嫢鏈夌綉缁滄帴鍙ｈ〃绀恒€傜劧鑰岋紝鍑轰簬鎬ц兘鍘熷洜锛孭HY 鍙兘鑳藉妫€娴嬪苟瀵?PTP 鏁版嵁鍖呮墦鏃堕棿鎴筹細灏藉彲鑳介潬杩戠嚎缂嗚幏鍙栫殑鏃堕棿鎴虫湁鍙兘浜х敓鏇寸ǔ瀹氥€佹洿绮剧‘鐨勫悓姝ャ€?
鏀寔 PTP 鏃堕棿鎴崇殑 PHY 椹卞姩蹇呴』鍒涘缓涓€涓?``struct mii_timestamper` 骞跺湪 `phydev->mii_ts`` 涓坊鍔犳寚鍚戝畠鐨勬寚閽堛€傜綉缁滄爤浼氭鏌ヨ鎸囬拡鏄惁瀛樺湪銆?
鐢变簬 PHY 娌℃湁缃戠粶鎺ュ彛琛ㄧず锛屽畠浠殑鎵撴椂闂存埑鍜?ethtool ioctl 鎿嶄綔闇€瑕佺敱鍏跺悇鑷殑 MAC 椹卞姩鏉ヤ腑浠嬨€傚洜姝わ紝涓?DSA 浜ゆ崲鏈轰笉鍚岋紝闇€瑕佷负姣忎釜鍗曠嫭鐨?MAC 椹卞姩杩涜淇敼浠ユ敮鎸?PHY 鏃堕棿鎴炽€傝繖鍖呮嫭锛?
- 鍦?`.ndo_eth_ioctl` 涓紝妫€鏌?`phy_has_hwtstamp(netdev->phydev)` 鏄惁涓虹湡銆傚鏋滄槸锛屽垯 MAC 椹卞姩涓嶅簲澶勭悊璇ヨ姹傦紝鑰屾槸浣跨敤 `phy_mii_ioctl()` 灏嗗叾浼犻€掔粰 PHY銆?
- 鍦?RX 鏃讹紝鍙兘闇€瑕佷篃鍙兘涓嶉渶瑕佺壒娈婂共棰勶紝鍙栧喅浜庣敤浜庡皢 skb 鍚戜笂浼犻€掔綉缁滄爤鐨勫嚱鏁般€傚浜庢櫘閫氱殑 `netif_rx()` 鍙婄被浼煎嚱鏁帮紝MAC 椹卞姩蹇呴』妫€鏌?`skb_defer_rx_timestamp(skb)` 鏄惁蹇呰鈥斺€斿鏋滃繀瑕侊紝鍒欏畬鍏ㄤ笉瑕佽皟鐢?`netif_rx()`銆傚鏋滃惎鐢ㄤ簡 `CONFIG_NETWORK_PHY_TIMESTAMPING`锛屼笖 `skb->dev->phydev->mii_ts` 瀛樺湪锛屽垯鍏?`.rxtstamp()` 閽╁瓙鐜板湪浼氳璋冪敤锛屼互浣跨敤涓?DSA 闈炲父鐩镐技鐨勯€昏緫鏉ョ‘瀹氭槸鍚﹂渶瑕佷负 RX 鏃堕棿鎴虫帹杩熴€傚悓鏍峰儚 DSA 涓€鏍凤紝褰撴椂闂存埑鍙敤鏃讹紝鐢?PHY 椹卞姩璐熻矗灏嗘暟鎹寘鍚戜笂鍙戦€佸埌鏍堛€?
  瀵逛簬鍏朵粬 skb 鎺ユ敹鍑芥暟锛屽 `napi_gro_receive` 鍜?`netif_receive_skb`锛屾爤浼氳嚜鍔ㄦ鏌?`skb_defer_rx_timestamp()` 鏄惁蹇呰锛屽洜姝ら┍鍔ㄥ唴閮ㄤ笉闇€瑕佹妫€鏌ャ€?
- 鍦?TX 涓婏紝鍚屾牱锛屽彲鑳介渶瑕佷篃鍙兘涓嶉渶瑕佺壒娈婂共棰勩€傝皟鐢?`mii_ts->txtstamp()` 閽╁瓙鐨勫嚱鏁板悕涓?`skb_clone_tx_timestamp()`銆傝繖涓嚱鏁版棦鍙互鐩存帴璋冪敤锛堣繖绉嶆儏鍐典笅纭疄闇€瑕佹湁鏄惧紡鐨?MAC 椹卞姩鏀寔锛夛紝浣嗚鍑芥暟涔熶細浠?`skb_tx_timestamp()` 璋冪敤涓崕甯︽墽琛岋紝鑰岃澶?MAC 椹卞姩宸茬粡涓轰簡杞欢鏃堕棿鎴崇洰鐨勬墽琛屼簡 `skb_tx_timestamp()`銆傚洜姝わ紝濡傛灉 MAC 鏀寔杞欢鏃堕棿鎴筹紝鍦ㄦ闃舵鏃犻渶鍐嶅仛浠讳綍浜嬫儏銆?
##### 3.2.3 MII 鎬荤嚎鍡呮帰璁惧


瀹冧滑鎵紨涓庡甫鏃堕棿鎴崇殑浠ュお缃?PHY 鐩稿悓鐨勮鑹诧紝鍙槸瀹冧滑鏄垎绔嬭澶囷紝鍥犳鍙互杩炲悓浠讳綍 PHY 涓€璧蜂娇鐢紝鍗充娇璇?PHY 涓嶆敮鎸佹椂闂存埑銆傚湪 Linux 涓紝瀹冧滑鍙€氳繃璁惧鏍戯紙Device Tree锛夎鍙戠幇骞堕檮鍔犲埌 `struct phy_device`锛屽叾浣欓儴鍒嗕娇鐢ㄤ笌閭ｄ簺鐩稿悓鐨?mii_ts 鍩虹璁炬柦銆傛洿澶氱粏鑺傝鍙傝 Documentation/devicetree/bindings/ptp/timestamper.txt銆?
##### 3.2.4 MAC 椹卞姩鐨勫叾浠栨敞鎰忎簨椤?

鍫嗗彔 PHC 鐨勪娇鐢ㄥ彲鑳戒細鏆撮湶鍑哄湪娌℃湁瀹冧滑鏃朵笉鍙兘瑙﹀彂鐨?MAC 椹卞姩 bug銆備竴涓緥瀛愪笌杩欒浠ｇ爜鏈夊叧锛?
```

      skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS;

```

浠讳綍 TX 鏃堕棿鎴抽€昏緫鈥斺€旀棤璁烘槸鏅€氱殑 MAC 椹卞姩銆丏SA 浜ゆ崲鏈洪┍鍔ㄣ€丳HY 椹卞姩杩樻槸 MII 鎬荤嚎鍡呮帰璁惧椹卞姩鈥斺€旈兘搴旇缃鏍囧織銆備絾鏄紝涓€涓笉鐭ラ亾 PHC 鍫嗗彔鐨?MAC 椹卞姩鍙兘浼氬洜涓洪櫎鑷韩涔嬪鐨勫叾浠栨柟璁剧疆浜嗘鏍囧織鑰岄櫡鍏ユ贩涔憋紝骞朵氦浠橀噸澶嶇殑鏃堕棿鎴炽€?
渚嬪锛屼竴涓吀鍨嬬殑 TX 鏃堕棿鎴抽┍鍔ㄨ璁″彲鑳藉皢鍙戦€侀儴鍒嗘媶鍒嗕负 2 閮ㄥ垎锛?
1. "TX"锛氭鏌?PTP 鏃堕棿鎴虫槸鍚︿箣鍓嶅凡閫氳繃 `.ndo_eth_ioctl`锛坄priv->hwtstamp_tx_enabled == true`锛夊惎鐢紝涓斿綋鍓?skb 闇€瑕佷竴涓?TX 鏃堕棿鎴筹紙`skb_shinfo(skb)->tx_flags & SKBTX_HW_TSTAMP`锛夈€傚鏋滀负鐪燂紝瀹冭缃?"`skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS`" 鏍囧織銆傛敞鎰忥細濡備笂鎵€杩帮紝鍦ㄥ爢鍙?PHC 绯荤粺鐨勬儏鍐典笅锛岃繖涓潯浠朵笉搴旇Е鍙戯紝鍥犱负璇?MAC 鑲畾涓嶆槸鏈€澶栧眰鐨?PHC銆備絾杩欎笉鏄吀鍨嬮棶棰樻墍鍦ㄣ€傚彂閫侀殢璇ユ暟鎹寘缁х画銆?
2. "TX confirmation"锛氬彂閫佸凡瀹屾垚銆傞┍鍔ㄦ鏌ユ槸鍚︽湁蹇呰涓哄叾鏀堕泦浠讳綍 TX 鏃堕棿鎴炽€傚吀鍨嬮棶棰樺氨鍑哄湪杩欓噷锛歁AC 椹卞姩璧版嵎寰勶紝鍙鏌ユ槸鍚﹁缃簡 "`skb_shinfo(skb)->tx_flags & SKBTX_IN_PROGRESS`"銆傚湪鍫嗗彔 PHC 绯荤粺涓紝杩欐槸涓嶆纭殑锛屽洜涓鸿 MAC 椹卞姩骞堕潪 TX 鏁版嵁璺緞涓敮涓€鍙兘棣栧厛鍚敤 SKBTX_IN_PROGRESS 鐨勫疄浣撱€?
姝ら棶棰樼殑姝ｇ‘瑙ｅ喅鏂规鏄紝MAC 椹卞姩鍦ㄥ叾 "TX confirmation" 閮ㄥ垎杩涜澶嶅悎妫€鏌ワ紝涓嶄粎瑕佹鏌?"`skb_shinfo(skb)->tx_flags & SKBTX_IN_PROGRESS`"锛岃繕瑕佹鏌?"`priv->hwtstamp_tx_enabled == true`"銆傜敱浜庣郴缁熺殑鍏朵綑閮ㄥ垎纭繚 PTP 鏃堕棿鎴充笉浼氬鏈€澶栧眰 PHC 涔嬪鐨勪换浣曞璞″惎鐢紝杩欎竴澧炲己妫€鏌ュ皢閬垮厤鍚戠敤鎴风┖闂翠氦浠橀噸澶嶇殑 TX 鏃堕棿鎴炽€?