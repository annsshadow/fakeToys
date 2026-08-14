
## Linux Phonet 鍗忚鏃?

### 绠€浠?

Phonet 鏄鍩轰簹铚傜獫璋冨埗瑙ｈ皟鍣ㄧ敤浜?IPC 鍜?RPC 鐨勬暟鎹寘鍗忚銆傞€氳繃 Linux Phonet
濂楁帴瀛楁棌锛孡inux 涓绘満杩涚▼鍙互浠庤皟鍒惰В璋冨櫒鎴栬繛鎺ュ埌璋冨埗瑙ｈ皟鍣ㄧ殑浠讳綍鍏跺畠澶栭儴璁惧
鎺ユ敹鍜屽彂閫佹秷鎭€傝皟鍒惰В璋冨櫒璐熻矗璺敱銆?
Phonet 鏁版嵁鍖呭彲浠ラ€氳繃鍚勭纭欢杩炴帴浜ゆ崲锛屽叿浣撳彇鍐充簬璁惧锛屼緥濡傦細

  - 甯︽湁 CDC Phonet 鎺ュ彛鐨?USB锛?  - 绾㈠锛?  - 钃濈墮锛?  - RS232 涓茶绔彛锛堝甫鏈変笓鐢ㄧ殑 "FBUS" 绾胯矾瑙勭▼锛夛紝
  - 甯︽湁鏌愪簺 TI OMAP 澶勭悊鍣ㄧ殑 SSI 鎬荤嚎銆?

### 鏁版嵁鍖呮牸寮?

```

  struct phonethdr {
    uint8_t  pn_media;  /* Media type (link-layer identifier) */
    uint8_t  pn_rdev;   /* Receiver device ID */
    uint8_t  pn_sdev;   /* Sender device ID */
    uint8_t  pn_res;    /* Resource ID or function */
    uint16_t pn_length; /* Big-endian message byte length (minus 6) */
    uint8_t  pn_robj;   /* Receiver object ID */
    uint8_t  pn_sobj;   /* Sender object ID */
  };

```
鍦?Linux 涓婏紝閾捐矾灞傚ご閮ㄥ寘鍚?pn_media 瀛楄妭锛堣涓嬫枃锛夈€傛帴涓嬫潵鐨?7 涓瓧鑺傛槸
缃戠粶灞傚ご閮ㄧ殑涓€閮ㄥ垎銆?
璁惧 ID 琚媶鍒嗭細楂?6 浣嶆瀯鎴愯澶囧湴鍧€锛岃€屼綆 2 浣嶇敤浜庡璺鐢紝8 浣嶅璞℃爣璇嗙
涔熸槸濡傛銆傚洜姝わ紝Phonet 鍙互琚涓轰竴涓叿鏈?6 浣嶅湴鍧€绌洪棿鍜?10 浣嶄紶杈撳崗璁紙寰堝儚
IP 涓栫晫涓殑绔彛鍙凤級鐨勭綉缁滃眰銆?
璋冨埗瑙ｈ皟鍣ㄥ缁堝叿鏈夊湴鍧€缂栧彿闆躲€傛墍鏈夊叾瀹冭澶囬兘鏈夎嚜宸辩殑 6 浣嶅湴鍧€銆?

### 閾捐矾灞?

Phonet 閾捐矾濮嬬粓鏄偣瀵圭偣閾捐矾銆傞摼璺眰澶撮儴鐢卞崟涓?Phonet 浠嬭川绫诲瀷瀛楄妭缁勬垚銆備粠
璋冨埗瑙ｈ皟鍣ㄧ殑瑙掑害鏉ョ湅锛屽畠鍞竴鏍囪瘑鏁版嵁鍖呮墍缁忕敱鐨勯摼璺€傛瘡涓?Phonet 缃戠粶璁惧搴?閫傚綋鍦板墠缃苟璁剧疆浠嬭川绫诲瀷瀛楄妭銆備负鏂逛究璧疯锛屾彁渚涗簡涓€涓€氱敤鐨?phonet_header_ops
閾捐矾灞傚ご閮ㄦ搷浣滅粨鏋勩€傚畠鏍规嵁缃戠粶璁惧纭欢鍦板潃璁剧疆浠嬭川绫诲瀷銆?
Linux Phonet 缃戠粶鎺ュ彛鏀寔涓撶敤鐨勯摼璺眰鏁版嵁鍖呯被鍨嬶紙ETH_P_PHONET锛夛紝瀹冭秴鍑?浠ュお缃戠被鍨嬭寖鍥淬€傚畠浠彧鑳藉彂閫佸拰鎺ユ敹 Phonet 鏁版嵁鍖呫€?
铏氭嫙 TUN 闅ч亾璁惧椹卞姩绋嬪簭涔熷彲鐢ㄤ簬 Phonet銆傝繖闇€瑕?IFF_TUN 妯″紡锛宊涓峗甯?IFF_NO_PI
鏍囧織銆傚湪杩欑鎯呭喌涓嬶紝娌℃湁閾捐矾灞傚ご閮紝鍥犳娌℃湁 Phonet 浠嬭川绫诲瀷瀛楄妭銆?
娉ㄦ剰锛孭honet 鎺ュ彛涓嶅厑璁稿鏁版嵁鍖呴噸鏂版帓搴忥紝鍥犳鍙兘涓庯紙榛樿鐨勶級Linux FIFO qdisc
涓€璧蜂娇鐢ㄣ€?

### 缃戠粶灞?

```

  struct sockaddr_pn {
    sa_family_t spn_family;    /* AF_PHONET */
    uint8_t     spn_obj;       /* Object ID */
    uint8_t     spn_dev;       /* Device ID */
    uint8_t     spn_resource;  /* Resource or function */
    uint8_t     spn_zero[...]; /* Padding */
  };

```
resource 瀛楁浠呭湪鍙戦€佸拰鎺ユ敹鏃朵娇鐢紱瀹冨湪 bind() 鍜?getsockname() 涓蹇界暐銆?

### 搴曞眰鏁版嵁鎶ュ崗璁?

搴旂敤绋嬪簭鍙互浣跨敤鏉ヨ嚜 PF_PHONET 鏃忕殑 Phonet 鏁版嵁鎶ュ鎺ュ瓧鍗忚鍙戦€?Phonet 娑堟伅銆?姣忎釜濂楁帴瀛楃粦瀹氬埌鍙敤鐨?2^10 涓璞?ID 涔嬩竴锛屽苟鍙互涓庝换浣曞叾瀹冨绛夋柟鍙戦€佸拰鎺ユ敹
鏁版嵁鍖呫€?
```

  struct sockaddr_pn addr = { .spn_family = AF_PHONET, };
  ssize_t len;
  socklen_t addrlen = sizeof(addr);
  int fd;

  fd = socket(PF_PHONET, SOCK_DGRAM, 0);
  bind(fd, (struct sockaddr *)&addr, sizeof(addr));
  /* ... */

  sendto(fd, msg, msglen, 0, (struct sockaddr *)&addr, sizeof(addr));
  len = recvfrom(fd, buf, sizeof(buf), 0,
		 (struct sockaddr *)&addr, &addrlen);

```
姝ゅ崗璁伒寰?SOCK_DGRAM 鏃犺繛鎺ヨ涔夈€備絾鏄紝涓嶆敮鎸?connect() 鍜?getpeername()锛?鍥犱负瀹冧滑鍦?Phonet 鐢ㄦ硶涓技涔庢病鏈夌敤澶勶紙鍙互杞绘澗娣诲姞锛夈€?

### 璧勬簮璁㈤槄


Phonet 鏁版嵁鎶ュ鎺ュ瓧鍙互璁㈤槄浠绘剰鏁伴噺鐨?8 浣?```

  uint32_t res = 0xXX;
  ioctl(fd, SIOCPNADDRESOURCE, &res);

```
璁㈤槄鍚屾牱浣跨敤 SIOCPNDELRESOURCE I/O 鎺у埗璇锋眰鍙栨秷锛屾垨鍦ㄥ鎺ュ瓧鍏抽棴鏃跺彇娑堛€?
娉ㄦ剰锛屼换浣曠粰瀹氳祫婧愪竴娆℃渶澶氬彧鑳芥湁涓€涓鎺ュ瓧璁㈤槄銆傚惁鍒欙紝ioctl() 灏嗚繑鍥?EBUSY銆?

### Phonet 绠￠亾鍗忚


Phonet 绠￠亾鍗忚鏄竴绉嶅甫鏈夌鍒扮鎷ュ鎺у埗鐨勭畝鍗曟湁搴忔暟鎹寘鍗忚銆傚畠浣跨敤琚姩鐩戝惉
濂楁帴瀛楄寖寮忋€傜洃鍚鎺ュ瓧缁戝畾鍒颁竴涓敮涓€鐨勭┖闂插璞?ID銆傛瘡涓洃鍚鎺ュ瓧鏈€澶氬彲澶勭悊
255 涓苟鍙戣繛鎺ワ紝姣忎釜 accept() 鍒扮殑濂楁帴瀛椾竴涓€?
```

  int lfd, cfd;

  lfd = socket(PF_PHONET, SOCK_SEQPACKET, PN_PROTO_PIPE);
  listen (lfd, INT_MAX);

  /* ... */
  cfd = accept(lfd, NULL, NULL);
  for (;;)
  {
    char buf[...];
    ssize_t len = read(cfd, buf, sizeof(buf));

    /* ... */

    write(cfd, msg, msglen);
  }

```
杩炴帴浼犵粺涓婄敱"绗笁鏂?搴旂敤鍦ㄤ袱绔箣闂村缓绔嬨€傝繖鎰忓懗鐫€涓ょ閮芥槸琚姩鐨勩€?

鑷?Linux 鍐呮牳鐗堟湰 2.6.39 璧凤紝涔熷彲浠ヤ娇鐢ㄤ富鍔ㄧ鐨?connect() 鐩存帴杩炴帴涓や釜绔偣銆?杩欐棬鍦ㄦ敮鎸佽緝鏂扮殑璇哄熀浜氭棤绾胯皟鍒惰В璋冨櫒 API锛屽鍦?```

  struct sockaddr_spn spn;
  int fd;

  fd = socket(PF_PHONET, SOCK_SEQPACKET, PN_PROTO_PIPE);
  memset(&spn, 0, sizeof(spn));
  spn.spn_family = AF_PHONET;
  spn.spn_obj = ...;
  spn.spn_dev = ...;
  spn.spn_resource = 0xD9;
  connect(fd, (struct sockaddr *)&spn, sizeof(spn));
  /* normal I/O here ... */
  close(fd);


```
涓壘鍒扮殑銆?
   褰撹疆璇㈠凡杩炴帴鐨勭閬撳鎺ュ瓧浠ユ鏌ュ彲鍐欐€ф椂锛屽瓨鍦ㄤ竴涓唴鍦ㄧ殑绔炴€佹潯浠讹紝鍗冲彲鍐欐€у彲鑳?   鍦ㄨ疆璇㈠拰绯荤粺璋冪敤鍐欏叆涔嬮棿涓㈠け銆傚湪杩欑鎯呭喌涓嬶紝濂楁帴瀛楀皢闃诲锛岀洿鍒板啓鍏ュ啀娆″彉涓?   鍙兘锛岄櫎闈炲惎鐢ㄤ簡闈為樆濉炴ā寮忋€?

绠￠亾鍗忚鍦?SOL_PNPIPE 绾у埆鎻愪緵涓や釜濂楁帴瀛楅€夐」锛?
  PNPIPE_ENCAP 鎺ュ彈涓€涓暣鏁板€硷紙int锛夛細

    PNPIPE_ENCAP_NONE锛?      濂楁帴瀛楁甯歌繍琛岋紙榛樿锛夈€?
    PNPIPE_ENCAP_IP锛?      濂楁帴瀛楃敤浣滆櫄鎷?IP 鎺ュ彛鐨勫悗绔€傝繖闇€瑕?CAP_NET_ADMIN 鑳藉姏銆傝鍩轰簹璋冨埗瑙ｈ皟鍣?      涓婄殑 GPRS 鏁版嵁鏀寔鍙互浣跨敤姝ら€夐」銆傛敞鎰忥紝鍦ㄦ妯″紡涓嬩笉鑳藉彲闈犲湴瀵硅濂楁帴瀛楄繘琛?      poll() 鎴?read()銆?
  PNPIPE_IFINDEX
      鏄竴涓彧璇绘暣鏁板€笺€傚畠鍖呭惈鐢?PNPIPE_ENCAP 鍒涘缓鐨勭綉缁滄帴鍙ｇ殑鎺ュ彛绱㈠紩锛?      濡傛灉灏佽鍏抽棴鍒欎负 0銆?
  PNPIPE_HANDLE
      鏄竴涓彧璇绘暣鏁板€笺€傚畠鍖呭惈绠￠亾鐨勫簳灞傛爣璇嗙锛?pipe handle"锛夈€傝繖浠呬负
      宸茶繛鎺ユ垨姝ｅ湪杩炴帴鐨勫鎺ュ瓧鎻忚堪绗﹀畾涔夈€?

### 浣滆€?

Linux Phonet 鏈€鍒濈敱 Sakari Ailus 缂栧啓銆?
鍏跺畠璐＄尞鑰呭寘鎷?Mik盲 Liljeberg銆丄ndras Domokos銆丆arlos Chinea 鍜?R茅mi Denis-Courmont銆?
Copyright |copy| 2008 Nokia Corporation.
