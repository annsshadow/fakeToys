
## UDP-Lite 鍗忚锛圧FC 3828锛?

  UDP-Lite 鏄竴涓?IETF 鏍囧噯杞ㄩ亾浼犺緭鍗忚锛屽叾鐗瑰緛鏄彲鍙橀暱搴︾殑鏍￠獙鍜屻€傝繖瀵逛簬閫氳繃鏃犵嚎缃戠粶浼犺緭澶氬獟浣擄紙瑙嗛銆乂oIP锛夊叿鏈変紭鍔匡紝鍥犱负閮ㄥ垎鎹熷潖鐨勬暟鎹寘浠嶇劧鍙互閫佸叆缂栬В鐮佸櫒锛岃€屼笉鏄洜涓烘牎楠屽拰娴嬭瘯澶辫触鑰岃涓㈠純銆?
  鏈枃浠剁畝瑕佹弿杩扮幇鏈夌殑鍐呮牳鏀寔浠ュ強濂楁帴瀛?API銆傛繁鍏ヤ簡瑙ｄ俊鎭紝浣犲彲浠ユ煡闃咃細

   - UDP-Lite 涓婚〉锛?     http://web.archive.org/web/%2E/http://www.erg.abdn.ac.uk/users/gerrit/udp-lite/

     浠庤繖閲屼綘杩樺彲浠ヤ笅杞戒竴浜涚ず渚嬪簲鐢ㄧ▼搴忔簮浠ｇ爜銆?
   - UDP-Lite HOWTO锛?     http://web.archive.org/web/%2E/http://www.erg.abdn.ac.uk/users/gerrit/udp-lite/files/UDP-Lite-HOWTO.txt

   - Wireshark UDP-Lite WiKi锛堝惈鎶撳寘鏂囦欢锛夛細
     https://wiki.wireshark.org/Lightweight_User_Datagram_Protocol

   - 鍗忚瑙勮寖 RFC 3828锛歨ttp://www.ietf.org/rfc/rfc3828.txt


## 1. 搴旂敤绋嬪簭


  澶氫釜搴旂敤绋嬪簭宸叉垚鍔熺Щ妞嶅埌 UDP-Lite銆侲thereal锛堢幇绉?wireshark锛夐粯璁ゆ敮鎸?UDP-Litev4/v6銆?
  灏嗗簲鐢ㄧ▼搴忕Щ妞嶅埌 UDP-Lite 寰堢畝鍗曪細鍙渶鏇存敼濂楁帴瀛楃骇鍒拰 IPPROTO锛涘彂閫佹柟杩橀渶璁剧疆鏍￠獙鍜岃鐩栭暱搴︼紙榛樿 = 澶撮儴闀垮害 = 8锛夈€傜粏鑺傚湪涓嬩竴鑺傘€?
## 2. 缂栫▼ API


  UDP-Lite 鎻愪緵鏃犺繛鎺ャ€佷笉鍙潬鐨勬暟鎹姤鏈嶅姟锛屽洜姝や娇鐢ㄤ笌 UDP 鐩稿悓鐨勫鎺ュ瓧绫诲瀷銆備簨瀹炰笂锛屼粠 UDP 绉绘鍒?UDP-Lite 闈炲父瀹规槗锛氬彧闇€灏?`IPPROTO_UDPLITE` 浣滀负鏈€鍚庝竴涓弬鏁板姞鍏?```

      s = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDPLITE);

  鎴栬€咃紝鍒嗗埆瀵瑰簲鍦帮細

  ::

      s = socket(PF_INET6, SOCK_DGRAM, IPPROTO_UDPLITE);

```
  浠呭仛涓婅堪鏇存敼锛屼綘灏辫兘澶熻繍琛?UDP-Lite 鏈嶅姟鎴栬繛鎺ュ埌 UDP-Lite 鏈嶅姟鍣ㄣ€傚唴鏍镐細鍋囧畾浣犲浣跨敤閮ㄥ垎鏍￠獙鍜岃鐩栦笉鎰熷叴瓒ｏ紝浠庤€屾ā鎷?UDP 妯″紡锛堝畬鍏ㄨ鐩栵級銆?
  瑕佷娇鐢ㄩ儴鍒嗘牎楠屽拰瑕嗙洊鍔熻兘锛岄渶瑕佽缃竴涓崟鐙殑濂楁帴瀛楅€夐」锛屽畠鎺ュ彈涓€涓寚瀹氳鐩栭暱搴︾殑鏁存暟锛?
    * 鍙戦€佹柟鏍￠獙鍜岃鐩栵細UDPLITE_SEND_CSCOV

      渚嬪::

	int val = 20;
	setsockopt(s, SOL_UDPLITE, UDPLITE_SEND_CSCOV, &val, sizeof(int));

      灏嗘牎楠屽拰瑕嗙洊闀垮害璁句负 20 瀛楄妭锛?2 瀛楄妭鏁版嵁 + 8 瀛楄妭澶撮儴锛夈€傛瘡涓暟鎹寘涓彧鏈夊墠 20 瀛楄妭锛堝姞涓婁吉澶撮儴锛変細琚牎楠屽拰銆傝繖瀵逛簬鍏锋湁 12 瀛楄妭鍩哄ご鐨?RTP 搴旂敤绋嬪簭寰堟湁鐢ㄣ€?

    * 鎺ユ敹鏂规牎楠屽拰瑕嗙洊锛歎DPLITE_RECV_CSCOV

      姝ら€夐」鏄帴鏀舵柟瀵瑰簲鐨勯儴鍒嗐€傚畠鏄湡姝ｅ彲閫夌殑锛屽嵆骞堕潪鍚敤閮ㄥ垎鏍￠獙鍜岃鐩栨祦閲忔墍蹇呴渶銆傚畠鐨勫姛鑳芥槸浣滀负娴侀噺杩囨护鍣細鍚敤鏃讹紝瀹冩寚绀哄唴鏍镐涪寮冩墍鏈夎鐩栭暱搴灏忎簬_姝ゅ€肩殑鏁版嵁鍖呫€備緥濡傦紝濡傛灉瑕佷繚鎶?RTP 鍜?UDP 澶撮儴锛屾帴鏀舵柟鍙互寮哄埗鍙帴鏀舵渶灏忚鐩栦负 20 鐨勬暟鎹寘::

	int min = 20;
	setsockopt(s, SOL_UDPLITE, UDPLITE_RECV_CSCOV, &min, sizeof(int));

  getsockopt(2) 鐨勮皟鐢ㄤ笌涔嬬被浼笺€備綔涓轰竴涓墿灞曡€岄潪鐙珛鍗忚锛屾墍鏈変粠 UDP 宸茬煡鐨勫鎺ュ瓧閫夐」閮藉彲浠ヤ互涓庝互鍓嶅畬鍏ㄧ浉鍚岀殑鏂瑰紡浣跨敤锛屼緥濡?UDP_CORK 鎴?UDP_ENCAP銆?
  鍏充簬 UDP-Lite 鏍￠獙鍜岃鐩栭€夐」鐨勮缁嗚璁哄湪绗?IV 鑺傘€?
```
## 3. 澶存枃浠?

  濂楁帴瀛?API 闇€瑕侀€氳繃 /usr/include 涓嬬殑澶存枃浠惰幏寰楁敮鎸侊細

    - /usr/include/netinet/in.h
      鐢ㄤ簬瀹氫箟 IPPROTO_UDPLITE

    - /usr/include/netinet/udplite.h
      鐢ㄤ簬 UDP-Lite 澶撮儴瀛楁鍜屽崗璁父閲?
```

    #define IPPROTO_UDPLITE       136
    #define SOL_UDPLITE           136
    #define UDPLITE_SEND_CSCOV     10
    #define UDPLITE_RECV_CSCOV     11

```
  鍚勭鍙戣鐗堢幇鎴愮殑澶存枃浠跺湪 UDP-Lite tarball 涓€?
```
## 4. 鍐呮牳鍏充簬鍚勭濂楁帴瀛楅€夐」鐨勮涓?

  瑕佸惎鐢ㄨ皟璇曟秷鎭紝闇€瑕佸皢鏃ュ織绾у埆璁句负 8锛屽洜涓哄ぇ澶氭暟娑堟伅浣跨敤 KERN_DEBUG 绾у埆锛?锛夈€?
  1) 鍙戦€佹柟濂楁帴瀛楅€夐」

  濡傛灉鍙戦€佹柟鎸囧畾瑕嗙洊闀垮害涓哄€?0锛屾ā鍧楀亣瀹氫负瀹屽叏瑕嗙洊锛屼紶杈撲竴涓鐩栭暱搴︿负 0 鐨勬暟鎹寘鍙婄浉搴旂殑鏍￠獙鍜屻€傚鏋滃彂閫佹柟鎸囧畾鐨勮鐩?< 8 涓斾笉涓?0锛屽唴鏍稿亣瀹?8 涓洪粯璁ゅ€笺€傛渶鍚庯紝濡傛灉鎸囧畾鐨勮鐩栭暱搴﹁秴杩囨暟鎹寘闀垮害锛屽垯鏀圭敤鏁版嵁鍖呴暱搴︿綔涓鸿鐩栭暱搴︺€?
  2) 鎺ユ敹鏂瑰鎺ュ瓧閫夐」

  鎺ユ敹鏂规寚瀹氬畠鎰挎剰鎺ュ彈鐨勬渶灏忚鐩栭暱搴﹀€笺€傛澶勫€间负 0 琛ㄧず鎺ユ敹鏂规€绘槸甯屾湜鏁翠釜鏁版嵁鍖呰瑕嗙洊銆傚湪杩欑鎯呭喌涓嬶紝鎵€鏈夐儴鍒嗚鐩栫殑鏁版嵁鍖呴兘浼氳涓㈠純锛屽苟璁板綍涓€涓敊璇€?
  涓嶅彲鑳芥寚瀹氶潪娉曞€硷紙<0 鍜?<8锛夛紱鍦ㄨ繖浜涙儏鍐典笅鍋囧畾榛樿涓?8銆?
  鎵€鏈夎鐩栧€煎皬浜庢寚瀹氶槇鍊煎埌杈剧殑鏁版嵁鍖呴兘浼氳涓㈠純锛岃繖浜涗簨浠朵篃浼氳璁板綍銆?
  3) 绂佺敤鏍￠獙鍜岃绠?
  鍦ㄥ彂閫佹柟鍜屾帴鏀舵柟锛屾牎楠屽拰鎬绘槸浼氳鎵ц
```

	setsockopt(sockfd, SOL_SOCKET, SO_NO_CHECK,  ... );

```
  灏嗘€绘槸琚拷鐣ワ紝鑰?:

	getsockopt(sockfd, SOL_SOCKET, SO_NO_CHECK, &value, ...);

```
  鐨勫€兼病鏈夋剰涔夛紙濡傚悓鍦?TCP 涓級銆傛牎楠屽拰瀛楁涓洪浂鐨勬暟鎹寘鏄潪娉曠殑锛堝弬瑙?RFC 3828 绗?3.1 鑺傦級锛屼細琚潤榛樹涪寮冦€?
  4) 鍒嗙墖

  鏍￠獙鍜岃绠楀悓鏃惰€冭檻缂撳啿鍖哄ぇ灏忓拰 MTU銆俇DP-Lite 鏁版嵁鍖呯殑澶у皬鐢卞彂閫佺紦鍐插尯鐨勫ぇ灏忓喅瀹氥€傚彂閫佺紦鍐插尯鐨勬渶灏忓ぇ灏忎负 2048锛堝湪 include/net/sock.h 涓畾涔変负 SOCK_MIN_SNDBUF锛夛紝榛樿鍊煎彲閰嶇疆涓?net.core.wmem_default锛屾垨閫氳繃璁剧疆 SO_SNDBUF socket(7) 閫夐」銆傚彂閫佺紦鍐插尯鐨勬渶澶т笂闄愮敱 net.core.wmem_max 鍐冲畾銆?
  缁欏畾澶т簬鍙戦€佺紦鍐插尯澶у皬鐨勮礋杞藉ぇ灏忥紝UDP-Lite 浼氬皢璐熻浇鎷嗗垎涓鸿嫢骞蹭釜鐙珛鐨勬暟鎹寘锛屾瘡绉嶆儏鍐典笅濉弧鍙戦€佺紦鍐插尯澶у皬銆?
  纭垏鐨勫€艰繕鍙栧喅浜庢帴鍙?MTU銆傛帴鍙?MTU 鍙嶈繃鏉ュ彲鑳借Е鍙?IP 鍒嗙墖銆傚湪杩欑鎯呭喌涓嬶紝鐢熸垚鐨?UDP-Lite 鏁版嵁鍖呰鎷嗗垎涓哄涓?IP 鏁版嵁鍖咃紝鍏朵腑鍙湁绗竴涓寘鍚?L4 澶撮儴銆?
  鍙戦€佺紦鍐插尯澶у皬瀵规牎楠屽拰瑕嗙洊闀垮害鏈夊奖鍝嶃€傝€冭檻浠ヤ笅绀轰緥::

    Payload: 1536 bytes          Send Buffer:     1024 bytes
    MTU:     1500 bytes          Coverage Length:  856 bytes

```
  UDP-Lite 灏嗘妸杩?1536 瀛楄妭鍒嗚鍦ㄤ袱涓嫭绔嬬殑鏁版嵁鍖呬腑::

    Packet 1: 1024 payload + 8 byte header + 20 byte IP header = 1052 bytes
    Packet 2:  512 payload + 8 byte header + 20 byte IP header =  540 bytes

```
  瑕嗙洊鏁版嵁鍖呰鐩栫涓€涓暟鎹寘涓殑 UDP-Lite 澶撮儴鍜?848 瀛楄妭璐熻浇锛岀浜屼釜鏁版嵁鍖呰瀹屽叏瑕嗙洊銆傛敞鎰忓浜庣浜屼釜鏁版嵁鍖咃紝瑕嗙洊闀垮害瓒呰繃浜嗘暟鎹寘闀垮害銆傚唴鏍稿湪杩欑鎯呭喌涓嬫€绘槸灏嗚鐩栭暱搴﹂噸鏂拌皟鏁翠负鏁版嵁鍖呴暱搴︺€?
  浣滀负涓€涓?UDP-Lite 鏁版嵁鍖呰鎷嗗垎涓哄涓井灏忓垎鐗囩殑渚嬪瓙锛岃€冭檻浠ヤ笅绀轰緥::

    Payload: 1024 bytes            Send buffer size: 1024 bytes
    MTU:      300 bytes            Coverage length:   575 bytes

    +-+-----------+--------------+--------------+--------------+
    |8|    272    |      280     |     280      |     280      |
    +-+-----------+--------------+--------------+--------------+
		280            560            840           1032
					^
    *****checksum coverage*************

```
  UDP-Lite 妯″潡鐢熸垚涓€涓?1032 瀛楄妭鐨勬暟鎹寘锛?024 + 8 瀛楄妭澶撮儴锛夈€傛牴鎹帴鍙?MTU锛岃繖浜涜鎷嗗垎涓?4 涓?IP 鏁版嵁鍖咃紙280 瀛楄妭 IP 璐熻浇 + 20 瀛楄妭 IP 澶撮儴锛夈€傚唴鏍告ā鍧楀湪瀵瑰垎鐗囬噴鏀剧粰 IP 妯″潡涔嬪墠锛屽鍓嶄袱涓畬鏁存暟鎹寘鐨勫唴瀹癸紝鍔犱笂鏈€鍚庝竴涓暟鎹寘鐨?15 瀛楄妭姹傚拰銆?
  瑕佹煡鐪?IPv6 鍒嗙墖鐨勭被浼兼儏鍐碉紝鑰冭檻閾捐矾 MTU 涓?1280 瀛楄妭銆佸啓缂撳啿鍖轰负 3356 瀛楄妭銆傚鏋滄牎楠屽拰瑕嗙洊灏忎簬 1232 瀛楄妭锛圡TU 鍑忓幓 IPv6/鍒嗙墖澶撮儴闀垮害锛夛紝鍙渶鑰冭檻绗竴涓垎鐗囥€傚綋浣跨敤鏇村ぇ鐨勬牎楠屽拰瑕嗙洊闀垮害鏃讹紝姣忎釜绗﹀悎鏉′欢鐨勫垎鐗囬兘闇€瑕佽鏍￠獙鍜屻€傚亣璁炬垜浠湁涓€涓?3062 鐨勬牎楠屽拰瑕嗙洊銆?356 瀛楄妭鐨勭紦鍐插尯灏嗚鎷嗗垎涓轰互涓嬪垎鐗?:

    Fragment 1: 1280 bytes carrying  1232 bytes of UDP-Lite data
    Fragment 2: 1280 bytes carrying  1232 bytes of UDP-Lite data
    Fragment 3:  948 bytes carrying   900 bytes of UDP-Lite data

```
  鍓嶄袱涓垎鐗囧繀椤昏瀹屾暣鏍￠獙鍜岋紝鏈€鍚庝竴涓垎鐗囦腑鍙湁 598锛? 3062 - 2*1232锛夊瓧鑺傝鏍￠獙鍜屻€?
  铏界劧姝ｇ‘澶勭悊姝ょ被鎯呭喌寰堥噸瑕侊紝浣嗗畠浠紙浠や汉璁ㄥ帉鍦帮級缃曡锛歎DP-Lite 璁捐鐢ㄤ簬浼樺寲閫氳繃鏃犵嚎锛堟垨涓€鑸櫔澹帮級閾捐矾鐨勫僵澶氬獟浣撴€ц兘锛屽洜姝ゆ洿鍙兘棰勬湡杈冨皬鐨勮鐩栭暱搴︺€?
```
## 5. UDP-Lite 杩愯鏃剁粺璁″強鍏跺惈涔?

  寮傚父鍜岄敊璇潯浠朵互 KERN_DEBUG 绾у埆璁板綍鍒?syslog銆傚叧浜?UDP-Lite 鐨勫疄鏃剁粺璁″彲鍦?/proc/net/snmp 鑾峰彇
```

			    netstat -svu

```
  杩欎細鏄剧ず UDP-Lite 缁熻鍙橀噺锛屽叾鍚箟濡備笅銆?
   ============     =====================================================
   InDatagrams      浜や粯缁欑敤鎴风殑鏁版嵁鎶ユ€绘暟銆?
   NoPorts          鎺ユ敹鍒颁竴涓湭鐭ョ鍙ｇ殑鏁版嵁鍖呮暟閲忋€?		    杩欎簺鎯呭喌琚崟鐙鏁帮紙涓嶈鍏?InErrors锛夈€?
   InErrors         閿欒鐨?UDP-Lite 鏁版嵁鍖呮暟閲忋€傞敊璇寘鎷細

		      * 鍐呴儴濂楁帴瀛楅槦鍒楁帴鏀堕敊璇?		      * 鏁版嵁鍖呭お鐭紙灏忎簬 8 瀛楄妭锛屾垨澹版槑鐨?			瑕嗙洊闀垮害瓒呰繃鎺ユ敹闀垮害锛?		      * xfrm4_policy_check() 杩斿洖閿欒
		      * 搴旂敤绋嬪簭鎸囧畾鐨勬渶灏忚鐩栭暱搴﹀ぇ浜?			鍏ョ珯鏁版嵁鍖呯殑瑕嗙洊闀垮害
		      * 鏍￠獙鍜岃鐩栬杩濆弽
		      * 閿欒鐨勬牎楠屽拰

   OutDatagrams     宸插彂閫佹暟鎹姤鐨勬€绘暟銆?   ============     =====================================================

   杩欎簺缁熻鏉ヨ嚜 UDP MIB锛圧FC 2013锛夈€?
```
## 6. IPtables


  瀵?UDP-Lite 鏈夋暟鎹寘鍖归厤鏀寔锛屼互鍙?LOG 鐩爣鐨勬敮鎸併€?```

    udplite 136     UDP-Lite        # UDP-Lite [RFC 3828]

```
  鐒跺悗::

	      iptables -A INPUT -p udplite -j LOG

```
  灏嗕骇鐢熻緭鍑哄埌 syslog 鐨勬棩蹇楄褰曘€備涪寮冨拰鎷掔粷鏁版嵁鍖呬篃鍙伐浣溿€?
```
## 7. 缁存姢鑰呭湴鍧€


  UDP-Lite 琛ヤ竵寮€鍙戜簬

		    University of Aberdeen
		    Electronics Research Group
		    Department of Engineering
		    Fraser Noble Building
		    Aberdeen AB24 3UE; UK

```
  褰撳墠鐨勭淮鎶よ€呮槸 Gerrit Renker锛?gerrit@erg.abdn.ac.uk>銆傚垵濮嬩唬鐮佺敱 William Stanislaus锛?william@erg.abdn.ac.uk> 寮€鍙戙€?