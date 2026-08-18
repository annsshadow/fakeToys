## Packet MMAP锛圥acket 鍐呭瓨鏄犲皠锛?

## 鎽樿


鏈枃浠惰褰曚簡 PACKET 濂楁帴瀛楁帴鍙ｆ墍鎻愪緵鐨?mmap() 鏈哄埗銆傝繖绫诲鎺ュ瓧鐢ㄤ簬锛?
i) 浣跨敤 tcpdump 涔嬬被鐨勫伐鍏锋崟鑾风綉缁滄祦閲忥紝
ii) 鍙戦€佺綉缁滄祦閲忥紝鎴栦换浣曞叾浠栭渶瑕佺洿鎺ワ紙raw锛?    璁块棶缃戠粶鎺ュ彛鐨勫満鏅€?
浣跨敤鏂规硶鐨勮缁嗕粙缁嶅彲鍦ㄤ互涓嬪湴鍧€鎵惧埌锛?
    https://web.archive.org/web/20220404160947/https://sites.google.com/site/packetmmap/

璇峰皢鎮ㄧ殑鎰忚鍙戦€佺粰鎴戜滑锛?    - Ulisses Alonso Camar贸 <uaca@i.hate.spam.alumni.uv.es>
    - Johann Baudy

## 涓轰粈涔堣浣跨敤 PACKET_MMAP


涓嶄娇鐢?PACKET_MMAP 鐨勬崟鑾疯繃绋嬶紙绾?AF_PACKET锛夋晥鐜囬潪甯镐綆銆傚畠浣跨敤闈炲父鏈夐檺
鐨勭紦鍐插尯锛屽苟涓旀瘡鎹曡幏涓€涓暟鎹寘灏遍渶瑕佷竴娆＄郴缁熻皟鐢紱鑻ヨ繕鎯宠幏鍙栨暟鎹寘鐨勬椂闂存埑
锛坙ibpcap 鎬绘槸濡傛锛夛紝鍒欓渶瑕佷袱娆＄郴缁熻皟鐢ㄣ€?
鐩稿弽锛孭ACKET_MMAP 鐨勬晥鐜囧緢楂樸€侾ACKET_MMAP 鎻愪緵浜嗕竴涓ぇ灏忓彲閰嶇疆銆佹槧灏勫埌鐢ㄦ埛绌洪棿
鐨勭幆褰㈢紦鍐插尯锛坈ircular buffer锛夛紝鍙敤浜庢帴鏀舵垨鍙戦€佹暟鎹寘銆傝繖鏍疯鍙栨暟鎹寘鍙渶绛夊緟
瀹冧滑鍒版潵锛屽ぇ澶氭暟鎯呭喌涓嬩笉闇€瑕佸彂鍑轰换浣曠郴缁熻皟鐢ㄣ€傚湪鍙戦€佹柟闈紝鍙互閫氳繃涓€娆＄郴缁熻皟鐢?鍙戦€佸涓暟鎹寘浠ヨ幏寰楁渶楂樺甫瀹姐€傜敱浜庡湪鍐呮牳涓庣敤鎴蜂箣闂翠娇鐢ㄤ簡鍏变韩缂撳啿鍖猴紝杩樺甫鏉ヤ簡
鍑忓皯鏁版嵁鍖呮嫹璐濇鏁扮殑濂藉銆?
浣跨敤 PACKET_MMAP 鏉ユ彁鍗囨崟鑾峰拰鍙戦€佽繃绋嬬殑鎬ц兘鏄悎閫傜殑锛屼絾瀹冨苟闈炲叏閮ㄣ€傝嚦灏戯紝濡傛灉浣犲湪
楂橀€燂紙鐩稿浜?CPU 閫熷害鑰岃█锛夋崟鑾凤紝搴斿綋妫€鏌ヤ綘鐨勭綉鍗★紙network interface card锛夎澶囬┍鍔ㄦ槸鍚?鏀寔鏌愮涓柇璐熻浇缂撹В鏈哄埗锛屾垨鑰咃紙鏇村ソ锛夋槸鍚︽敮鎸?NAPI锛屽苟涓旂‘淇濆畠宸插惎鐢ㄣ€傚浜庡彂閫侊紝
璇锋鏌ヤ綘鐨勭綉缁滆澶囨墍浣跨敤鍜屾敮鎸佺殑 MTU锛圡aximum Transmission Unit锛屾渶澶т紶杈撳崟鍏冿級銆傚皢浣犵殑
缃戝崱涓柇锛圛RQ锛夌粦瀹氬埌鐗瑰畾 CPU 涔熶細甯︽潵濂藉銆?
## 濡備綍浣跨敤 mmap() 鎻愬崌鎹曡幏杩囩▼


浠庣敤鎴风殑瑙掑害鐪嬶紝浣犲簲璇ヤ娇鐢ㄦ洿楂樺眰鐨?libpcap 搴擄紝瀹冩槸浜嬪疄涓婄殑鏍囧噯锛屽嚑涔庡彲绉绘鍒?鍖呮嫭 Win32 鍦ㄥ唴鐨勬墍鏈夋搷浣滅郴缁熴€?
Packet MMAP 鐨勬敮鎸佸ぇ绾︽槸鍦?1.3.0 鐗堟湰鏃堕泦鎴愯繘 libpcap 鐨勶紱TPACKET_V3 鐨勬敮鎸佸湪
1.5.0 鐗堟湰涓姞鍏ャ€?
## 濡備綍鐩存帴浣跨敤 mmap() 鎻愬崌鎹曡幏杩囩▼


浠庣郴缁熻皟鐢ㄧ殑瑙掑害鐪嬶紝PACKET_MMAP 鐨勪娇鐢ㄦ秹鍙?```

    [setup]     socket() -------> creation of the capture socket
		setsockopt() ---> allocation of the circular buffer (ring)
				  option: PACKET_RX_RING
		mmap() ---------> mapping of the allocated buffer to the
				  user process

    [capture]   poll() ---------> to wait for incoming packets

    [shutdown]  close() --------> destruction of the capture socket and
				  deallocation of all associated
				  resources.


```
濂楁帴瀛楃殑鍒涘缓涓庨攢姣侀兘寰堢洿鎺ワ紝閫氳繃濡備笅鏂瑰紡瀹屾垚
```

 int fd = socket(PF_PACKET, mode, htons(ETH_P_ALL));

```
鍏朵腑 mode 涓?SOCK_RAW 琛ㄧず鍘熷锛坮aw锛夋帴鍙ｏ紝鍙崟鑾烽摼璺眰淇℃伅锛涙垨涓?SOCK_DGRAM
琛ㄧず鈥滅啛鈥濓紙cooked锛夋帴鍙ｏ紝鍏朵腑涓嶆敮鎸佹崟鑾烽摼璺眰淇℃伅锛岃€屾槸鐢卞唴鏍告彁渚涗竴涓摼璺眰
浼ご閮紙pseudo-header锛夈€?
濂楁帴瀛楀強鎵€鏈夌浉鍏宠祫婧愮殑閿€姣侀€氳繃绠€鍗曞湴璋冪敤 close(fd) 鏉ュ畬鎴愩€?
涓庝娇鐢?PACKET_MMAP 鏃犲叧锛屼篃鍙互鐢ㄤ竴涓鎺ュ瓧鍚屾椂杩涜鎹曡幏鍜屽彂閫併€傝繖鍙渶鐢ㄤ竴娆?mmap() 璋冪敤鍚屾椂鏄犲皠宸插垎閰嶇殑 RX 鍜?TX 缂撳啿鍖虹幆锛坮ing锛夊嵆鍙€傚弬瑙佲€滅幆褰㈢紦鍐插尯锛坮ing锛?鐨勬槧灏勪笌浣跨敤鈥濄€?
鎺ヤ笅鏉ユ垜灏嗘弿杩?PACKET_MMAP 鐨勮缃強鍏剁害鏉燂紝浠ュ強鐜舰缂撳啿鍖哄湪鐢ㄦ埛杩涚▼涓殑鏄犲皠
鍜岃缂撳啿鍖虹殑浣跨敤銆?
## 濡備綍鐩存帴浣跨敤 mmap() 鎻愬崌鍙戦€佽繃绋?```

    [setup]         socket() -------> creation of the transmission socket
		    setsockopt() ---> allocation of the circular buffer (ring)
				      option: PACKET_TX_RING
		    bind() ---------> bind transmission socket with a network interface
		    mmap() ---------> mapping of the allocated buffer to the
				      user process

    [transmission]  poll() ---------> wait for free packets (optional)
		    send() ---------> send all packets that are set as ready in
				      the ring
				      The flag MSG_DONTWAIT can be used to return
				      before end of transfer.

    [shutdown]      close() --------> destruction of the transmission socket and
				      deallocation of all associated resources.

```
濂楁帴瀛楃殑鍒涘缓涓庨攢姣佸悓鏍峰緢鐩存帴锛岄€氳繃濡備笅鏂瑰紡瀹屾垚
```

 int fd = socket(PF_PACKET, mode, 0);

```
濡傛灉鎴戜滑鍙€氳繃璇ュ鎺ュ瓧鍙戦€侊紝鍗忚锛坧rotocol锛夊彲浠ュ彲閫夊湴璁句负 0锛屼粠鑰岄伩鍏嶄竴娆℃槀璐电殑
packet_rcv() 璋冪敤銆傝繖绉嶆儏鍐典笅锛屼綘杩橀渶瑕佸皢 TX_RING 涓?sll_protocol = 0 杩涜
bind(2) 缁戝畾銆傚惁鍒欙紝渚嬪浣跨敤 htons(ETH_P_ALL) 鎴栦换浣曞叾浠栧崗璁€?
灏嗗鎺ュ瓧缁戝畾鍒颁綘鐨勭綉缁滄帴鍙ｆ槸蹇呴』鐨勶紙閲囩敤闆舵嫹璐濇椂锛夛紝浠ヤ究鑾风煡鐜舰缂撳啿鍖轰腑浣跨敤鐨?甯уご閮ㄥぇ灏忋€?```

    --------------------
    | struct tpacket_hdr | Header. It contains the status of
    |                    | of this frame
    |--------------------|
    | data buffer        |
    .                    .  Data that will be sent over the network interface.
    .                    .
    --------------------

 bind() associates the socket to your network interface thanks to
 sll_ifindex parameter of struct sockaddr_ll.

 Initialization example::

    struct sockaddr_ll my_addr;
    struct ifreq s_ifr;
    ...

    strscpy_pad (s_ifr.ifr_name, "eth0", sizeof(s_ifr.ifr_name));

    /* get interface index of eth0 */
    ioctl(this->socket, SIOCGIFINDEX, &s_ifr);

    /* fill sockaddr_ll struct to prepare binding */
    my_addr.sll_family = AF_PACKET;
    my_addr.sll_protocol = htons(ETH_P_ALL);
    my_addr.sll_ifindex =  s_ifr.ifr_ifindex;

    /* bind socket to eth0 */
    bind(this->socket, (struct sockaddr *)&my_addr, sizeof(struct sockaddr_ll));

 A complete tutorial is available at:
 https://web.archive.org/web/20220404160947/https://sites.google.com/site/packetmmap/

```
```

 frame base + TPACKET_HDRLEN - sizeof(struct sockaddr_ll)

```
鍥犳锛屾棤璁轰綘涓哄鎺ュ瓧妯″紡閫夋嫨浠€涔堬紙SOCK_DGRAM 鎴?SOCK_RAW锛夛紝
```

 frame base + TPACKET_ALIGN(sizeof(struct tpacket_hdr))

```
濡傛灉浣犳兂灏嗙敤鎴锋暟鎹斁鍦ㄨ窛绂诲抚璧峰澶勭殑鑷畾涔夊亸绉讳綅缃紙渚嬪涓轰簡涓?SOCK_RAW 妯″紡涓嬬殑
璐熻浇瀵归綈锛夛紝鍙互璁剧疆 tp_net锛堥厤鍚?SOCK_DGRAM锛夋垨 tp_mac锛堥厤鍚?SOCK_RAW锛夈€備负浜嗕娇鍏?鐢熸晥锛屽繀椤讳簨鍏堥€氳繃 setsockopt() 鍙?PACKET_TX_HAS_OFF 閫夐」鍚敤瀹冦€?
## PACKET_MMAP 璁剧疆


瑕佸湪鐢ㄦ埛绾т唬鐮佷腑璁剧疆 PACKET_MMAP锛屾槸閫氳繃绫讳技濡備笅鐨勮皟鐢ㄦ潵瀹屾垚鐨?
```

     setsockopt(fd, SOL_PACKET, PACKET_RX_RING, (void *) &req, sizeof(req))

 - Transmission process::

     setsockopt(fd, SOL_PACKET, PACKET_TX_RING, (void *) &req, sizeof(req))

```
涓婅堪璋冪敤涓渶閲嶈鐨勫弬鏁版槸 req 鍙傛暟锛?```

    struct tpacket_req
    {
	unsigned int    tp_block_size;  /* Minimal size of contiguous block */
	unsigned int    tp_block_nr;    /* Number of blocks */
	unsigned int    tp_frame_size;  /* Size of frame */
	unsigned int    tp_frame_nr;    /* Total number of frames */
    };

```
璇ョ粨鏋勫畾涔夊湪 /usr/include/linux/if_packet.h 涓紝瀹冨缓绔嬩簡涓€涓笉鍙崲鍑猴紙unswappable锛?鍐呭瓨鐨勭幆褰㈢紦鍐插尯锛坮ing锛夈€?
瀹冭鏄犲皠鍒版崟鑾疯繘绋嬪悗锛屽厑璁稿湪涓嶅彂鍑虹郴缁熻皟鐢ㄧ殑鎯呭喌涓嬭鍙栧凡鎹曡幏鐨勫抚浠ュ強鏃堕棿鎴崇瓑
鐩稿叧鍏冧俊鎭€?
甯ц鍒嗙粍鍒板潡锛坆lock锛変腑銆傛瘡涓潡鏄竴娈电墿鐞嗚繛缁殑鍐呭瓨鍖哄煙锛屽寘鍚?tp_block_size/tp_frame_size 涓抚銆傛€绘暟閲忔弧瓒?```

    frames_per_block = tp_block_size/tp_frame_size

```
```

    frames_per_block * tp_block_nr == tp_frame_nr

```
```

     tp_block_size= 4096
     tp_frame_size= 2048
     tp_block_nr  = 4
     tp_frame_nr  = 8

```
```

	    block #1                 block #2
    +---------+---------+    +---------+---------+
    | frame 1 | frame 2 |    | frame 3 | frame 4 |
    +---------+---------+    +---------+---------+

	    block #3                 block #4
    +---------+---------+    +---------+---------+
    | frame 5 | frame 6 |    | frame 7 | frame 8 |
    +---------+---------+    +---------+---------+

```
涓€涓抚鍙互鏄换鎰忓ぇ灏忥紝鍞竴鏉′欢鏄畠鑳芥斁鍏ヤ竴涓潡涓€備竴涓潡鍙兘瀹圭撼鏁存暟涓抚锛屾崲鍙ヨ瘽璇达紝
涓€涓抚涓嶈兘璺ㄨ秺涓や釜鍧楋紝鍥犳鍦ㄩ€夋嫨 frame_size 鏃舵湁浜涚粏鑺傞渶瑕佹敞鎰忋€傚弬瑙佲€滅幆褰㈢紦鍐插尯锛坮ing锛?鐨勬槧灏勪笌浣跨敤鈥濄€?
## PACKET_MMAP 璁剧疆绾︽潫


鍦ㄥ唴鏍哥増鏈?2.4.26 涔嬪墠锛堥拡瀵?2.4 鍒嗘敮锛夊拰 2.6.5 涔嬪墠锛?.6 鍒嗘敮锛夛紝PACKET_MMAP 缂撳啿鍖哄湪
32 浣嶆灦鏋勪笂鏈€澶氬彧鑳藉绾?32768 涓抚锛屽湪 64 浣嶆灦鏋勪笂鏈€澶氬彧鑳藉绾?16384 涓抚銆?
### 鍧楀ぇ灏忛檺鍒?

濡傚墠鎵€杩帮紝姣忎釜鍧楅兘鏄竴娈佃繛缁殑鐗╃悊鍐呭瓨鍖哄煙銆傝繖浜涘唴瀛樺尯鍩熸槸閫氳繃璋冪敤
__get_free_pages() 鍑芥暟鍒嗛厤鐨勩€傞【鍚嶆€濅箟锛岃鍑芥暟鍒嗛厤鍐呭瓨椤碉紝绗簩涓弬鏁版槸鈥渙rder鈥濓紝鍗?2 鐨勫箓娆￠〉鏁伴噺锛屼篃灏辨槸锛堝綋 PAGE_SIZE == 4096 鏃讹級order=0 ==> 4096 瀛楄妭锛宱rder=1 ==>
8192 瀛楄妭锛宱rder=2 ==> 16384 瀛楄妭锛屼緷姝ょ被鎺ㄣ€傜敱 __get_free_pages 鍒嗛厤鐨勫尯鍩熺殑鏈€澶уぇ灏?鐢?MAX_PAGE_ORDER 瀹忓喅瀹氥€?```

   PAGE_SIZE << MAX_PAGE_ORDER

   In a i386 architecture PAGE_SIZE is 4096 bytes
   In a 2.4/i386 kernel MAX_PAGE_ORDER is 10
   In a 2.6/i386 kernel MAX_PAGE_ORDER is 11

```
鍥犳鍦?2.4/2.6 鍐呮牳涓紝閰嶅悎 i386 鏋舵瀯锛実et_free_pages 鏈€澶氬彲鍒嗛厤 4MB 鎴?8MB銆?
鐢ㄦ埛绌洪棿绋嬪簭鍙互鍖呭惈 /usr/include/sys/user.h 鍜?/usr/include/linux/mmzone.h 鏉ヨ幏鍙?PAGE_SIZE銆丮AX_PAGE_ORDER 鐨勫０鏄庛€?
椤靛ぇ灏忎篃鍙互閫氳繃 getpagesize (2) 绯荤粺璋冪敤鍔ㄦ€佺‘瀹氥€?
### 鍧楁暟閲忛檺鍒?

涓轰簡鐞嗚В PACKET_MMAP 鐨勭害鏉燂紝鎴戜滑闇€瑕佹煡鐪嬬敤浜庝繚瀛樻瘡涓潡鎸囬拡鐨勭粨鏋勩€?
鐩墠锛岃缁撴瀯鏄竴涓敤 kmalloc 鍔ㄦ€佸垎閰嶇殑鍚戦噺
```

    +---+---+---+---+
    | x | x | x | x |
    +---+---+---+---+
      |   |   |   |
      |   |   |   v
      |   |   v  block #4
      |   v  block #3
      v  block #2
     block #1

```
kmalloc 浠庝竴缁勯鍏堢‘瀹氱殑澶у皬鐨勫唴瀛樻睜涓垎閰嶄换鎰忓瓧鑺傛暟鐨勭墿鐞嗚繛缁唴瀛樸€傝鍐呭瓨姹犵敱
slab 鍒嗛厤鍣ㄧ淮鎶わ紝鏈€缁堢敱瀹冭礋璐ｅ畬鎴愬垎閰嶏紝鍥犳涔熺敱瀹冮檺鍒朵簡 kmalloc 鑳藉垎閰嶇殑鏈€澶у唴瀛樸€?
鍦?2.4/2.6 鍐呮牳鍜?i386 鏋舵瀯涓婏紝闄愬埗涓?131072 瀛楄妭銆俴malloc 浣跨敤鐨勯瀹氬ぇ灏忓彲浠ュ湪
/proc/slabinfo 鐨勨€渟ize-<bytes>鈥濇潯鐩腑鏌ョ湅銆?
鍦?32 浣嶆灦鏋勪笂锛屾寚閽堥暱搴︿负 4 瀛楄妭锛屽洜姝ゆ€诲潡鏁颁负
```

     131072/4 = 32768 blocks

```
## PACKET_MMAP 缂撳啿鍖哄ぇ灏忚绠楀櫒


瀹氫箟锛?
==============  ================================================================
<size-max>      is the maximum size of allocable with kmalloc
		(see /proc/slabinfo)
<pointer size>  depends on the architecture -- `sizeof(void *)`
<page size>     depends on the architecture -- PAGE_SIZE or getpagesize (2)
<max-order>     is the value defined with MAX_PAGE_ORDER
<frame size>    it's an upper bound of frame's capture size (more on this later)
==============  ================================================================

```

	<block number> = <size-max>/<pointer size>
	<block size> = <pagesize> << <max-order>

```
```

	<block number> * <block size>

```
```

	<block number> * <block size> / <frame size>

```
鍋囪鏈変互涓嬪弬鏁帮紝閫傜敤浜?2.6 鍐呮牳鍜?```

	<size-max> = 131072 bytes
	<pointer size> = 4 bytes
	<pagesize> = 4096 bytes
	<max-order> = 11

```
```

	<block number> = 131072/4 = 32768 blocks
	<block size> = 4096 << 11 = 8 MiB.

```
鍥犳缂撳啿鍖哄皢鏈?262144 MiB 澶у皬銆傚畠鍙互瀹圭撼 262144 MiB / 2048 瀛楄妭 = 134217728 涓抚銆?
瀹為檯涓婏紝杩欎釜缂撳啿鍖哄ぇ灏忓湪 i386 鏋舵瀯涓婃槸涓嶅彲鑳界殑銆傝浣忥紝鍐呭瓨鏄湪鍐呮牳绌洪棿鍒嗛厤鐨勶紝瀵逛簬
i386 鍐呮牳锛屽唴瀛樺ぇ灏忛檺鍒朵负 1GiB銆?
鎵€鏈夊唴瀛樺垎閰嶉兘涓嶄細琚噴鏀撅紝鐩村埌濂楁帴瀛楀叧闂€傚唴瀛樺垎閰嶄互 GFP_KERNEL 浼樺厛绾ц繘琛岋紝杩欏熀鏈笂
鎰忓懗鐫€鍒嗛厤鍙互绛夊緟骞舵崲鍑哄叾浠栬繘绋嬬殑鍐呭瓨浠ュ垎閰嶆墍闇€鍐呭瓨锛屽洜姝ら€氬父鍙互杈惧埌涓婇檺銆?
### 鍏朵粬绾︽潫


濡傛灉浣犳煡鐪嬫簮浠ｇ爜锛屼綘浼氱湅鍒版垜杩欓噷鐢绘垚鈥滃抚鈥濈殑骞朵笉鍙槸閾捐矾灞傚抚銆傚湪姣忎釜甯х殑寮€澶存湁涓€涓?绉颁负 struct tpacket_hdr 鐨勫ご閮紝鐢ㄤ簬 PACKET_MMAP 涓繚瀛橀摼璺眰甯х殑鍏冧俊鎭紝濡傛椂闂存埑銆?鎵€浠ユ垜浠繖閲岀敾鐨勨€滃抚鈥濆疄闄呬笂鏄?```

 /*
   Frame structure:

   - Start. Frame must be aligned to TPACKET_ALIGNMENT=16
   - struct tpacket_hdr
   - pad to TPACKET_ALIGNMENT=16
   - struct sockaddr_ll
   - Gap, chosen so that packet data (Start+tp_net) aligns to
     TPACKET_ALIGNMENT=16
   - Start+tp_mac: [ Optional MAC header ]
   - Start+tp_net: Packet data, aligned to TPACKET_ALIGNMENT=16.
   - Pad to align to TPACKET_ALIGNMENT=16
 */

```
浠ヤ笅鏄?packet_set_ring 涓細妫€鏌ョ殑鏉′欢

   - tp_block_size must be a multiple of PAGE_SIZE (1)
   - tp_frame_size must be greater than TPACKET_HDRLEN (obvious)
   - tp_frame_size must be a multiple of TPACKET_ALIGNMENT
   - tp_frame_nr   must be exactly frames_per_block*tp_block_nr

娉ㄦ剰 tp_block_size 搴旈€夋嫨涓?2 鐨勫箓锛屽惁鍒欎細娴垂鍐呭瓨銆?
### 鐜舰缂撳啿鍖猴紙ring锛夌殑鏄犲皠涓庝娇鐢?

缂撳啿鍖哄湪鐢ㄦ埛杩涚▼涓殑鏄犲皠鏄€氳繃甯歌鐨?mmap 鍑芥暟瀹屾垚鐨勩€傚嵆浣跨幆褰㈢紦鍐插尯鐢辫嫢骞茬墿鐞嗕笂
涓嶈繛缁殑鍐呭瓨鍧楃粍鎴愶紝瀹冧滑鍦ㄧ敤鎴风┖闂寸湅鏉ユ槸杩炵画鐨勶紝鍥犳
```

    mmap(0, size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);

```
濡傛灉 tp_frame_size 鏄?tp_block_size 鐨勭害鏁帮紝甯у皢鎸?tp_frame_size 瀛楄妭闂撮殧杩炵画鎺掑垪銆?濡傛灉涓嶆槸锛屽垯姣?tp_block_size/tp_frame_size 涓抚涔嬮棿浼氭湁涓€涓棿闅欙紙gap锛夈€傝繖鏄洜涓轰竴涓?甯т笉鑳借法瓒婁袱涓潡銆?
瑕佸湪涓€涓鎺ュ瓧涓婂悓鏃惰繘琛屾崟鑾峰拰鍙戦€侊紝瀵逛袱鑰呯殑鏄犲皠涓?```

    ...
    setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &foo, sizeof(foo));
    setsockopt(fd, SOL_PACKET, PACKET_TX_RING, &bar, sizeof(bar));
    ...
    rx_ring = mmap(0, size * 2, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    tx_ring = rx_ring + size;

```
RX 蹇呴』鍦ㄥ墠锛屽洜涓哄唴鏍哥揣鎺ョ潃 RX 涔嬪悗鏄犲皠 TX 鐜唴瀛樸€?
鍦ㄦ瘡涓抚鐨勫紑澶存湁涓€涓姸鎬佸瓧娈碉紙鍙傝 struct tpacket_hdr锛夈€傚鏋滆瀛楁涓?0锛岃〃绀鸿甯?鍙緵鍐呮牳浣跨敤锛涘惁鍒欙紝瀛樺湪涓€涓敤鎴峰彲璇荤殑甯э紝閫傜敤浠ヤ笅鏍囧織锛?
##### 鎹曡幏杩囩▼
```

     #define TP_STATUS_COPY          (1 << 1)
     #define TP_STATUS_LOSING        (1 << 2)
     #define TP_STATUS_CSUMNOTREADY  (1 << 3)
     #define TP_STATUS_CSUM_VALID    (1 << 7)

```
======================  =======================================================
TP_STATUS_COPY		This flag indicates that the frame (and associated
			meta information) has been truncated because it's
			larger than tp_frame_size. This packet can be
			read entirely with recvfrom().

			In order to make this work it must to be
			enabled previously with setsockopt() and
			the PACKET_COPY_THRESH option.

			The number of frames that can be buffered to
			be read with recvfrom is limited like a normal socket.
			See the SO_RCVBUF option in the socket (7) man page.

TP_STATUS_LOSING	indicates there were packet drops from last time
			statistics where checked with getsockopt() and
			the PACKET_STATISTICS option.

TP_STATUS_CSUMNOTREADY	currently it's used for outgoing IP packets which
			its checksum will be done in hardware. So while
			reading the packet we should not try to check the
			checksum.

TP_STATUS_CSUM_VALID	This flag indicates that at least the transport
			header checksum of the packet has been already
			validated on the kernel side. If the flag is not set
			then we are free to check the checksum by ourselves
			provided that TP_STATUS_CSUMNOTREADY is also not set.
======================  =======================================================

```

     #define TP_STATUS_KERNEL        0
     #define TP_STATUS_USER          1

```
鍐呮牳灏嗘墍鏈夊抚鍒濆鍖栦负 TP_STATUS_KERNEL锛屽綋鍐呮牳鎺ユ敹鍒颁竴涓暟鎹寘鏃讹紝瀹冨皢鍏舵斁鍏ョ紦鍐插尯锛?骞舵洿鏂扮姸鎬佷负鑷冲皯鍖呭惈 TP_STATUS_USER 鏍囧織銆傜劧鍚庣敤鎴峰彲浠ヨ鍙栬鏁版嵁鍖咃紝璇诲彇瀹屽悗鐢ㄦ埛蹇呴』
灏嗙姸鎬佸瓧娈垫竻闆讹紝浠ヤ究鍐呮牳鍙互鍐嶆浣跨敤璇ュ抚缂撳啿鍖恒€?
鐢ㄦ埛鍙互浣跨敤 poll锛堝叾浠栧彉浣撲篃搴旈€傜敤锛夋潵妫€鏌ユ槸鍚︽湁鏂?```

    struct pollfd pfd;

    pfd.fd = fd;
    pfd.revents = 0;
    pfd.events = POLLIN|POLLRDNORM|POLLERR;

    if (status == TP_STATUS_KERNEL)
	retval = poll(&pfd, 1, timeout);

```
鍏堟鏌ョ姸鎬佸€煎啀 poll 绛夊緟甯э紝骞朵笉浼氫骇鐢熺珵浜夋潯浠躲€?
##### 鍙戦€佽繃绋?```

     #define TP_STATUS_AVAILABLE        0 // Frame is available
     #define TP_STATUS_SEND_REQUEST     1 // Frame will be sent on next send()
     #define TP_STATUS_SENDING          2 // Frame is currently in transmission
     #define TP_STATUS_WRONG_FORMAT     4 // Frame format is not correct

```
棣栧厛锛屽唴鏍稿皢鎵€鏈夊抚鍒濆鍖栦负 TP_STATUS_AVAILABLE銆傝鍙戦€佷竴涓暟鎹寘锛岀敤鎴峰～鍏呬竴涓?鍙敤甯х殑鏁版嵁缂撳啿鍖猴紝灏?tp_len 璁句负褰撳墠鏁版嵁缂撳啿鍖哄ぇ灏忥紝骞跺皢鍏剁姸鎬佸瓧娈佃涓?TP_STATUS_SEND_REQUEST銆傝繖鍙互鍦ㄥ涓抚涓婂畬鎴愩€備竴鏃︾敤鎴峰噯澶囧ソ鍙戦€侊紝灏辫皟鐢?send()銆?鐒跺悗鎵€鏈夌姸鎬佺瓑浜?TP_STATUS_SEND_REQUEST 鐨勭紦鍐插尯琚浆鍙戝埌缃戠粶璁惧銆傚唴鏍稿皢姣忎釜宸插彂閫?甯х殑鐘舵€佹洿鏂颁负 TP_STATUS_SENDING锛岀洿鍒颁紶杈撶粨鏉熴€?
姣忔浼犺緭缁撴潫鏃讹紝缂撳啿鍖虹姸鎬佹仮澶嶄负 TP_STATUS_AVAILABLE銆?```

    header->tp_len = in_i_size;
    header->tp_status = TP_STATUS_SEND_REQUEST;
    retval = send(this->socket, NULL, 0, 0);

```
鐢ㄦ埛涔熷彲浠ヤ娇鐢?poll() 鏉ユ鏌ョ紦鍐插尯鏄惁鍙敤锛?
(status == TP_STATUS_SENDING)
```

    struct pollfd pfd;
    pfd.fd = fd;
    pfd.revents = 0;
    pfd.events = POLLOUT;
    retval = poll(&pfd, 1, timeout);

```
## 鏈夊摢浜?TPACKET 鐗堟湰鍙敤锛屼綍鏃朵娇鐢ㄥ畠浠紵
```

 int val = tpacket_version;
 setsockopt(fd, SOL_PACKET, PACKET_VERSION, &val, sizeof(val));
 getsockopt(fd, SOL_PACKET, PACKET_VERSION, &val, sizeof(val));

```
鍏朵腑 'tpacket_version' 鍙互鏄?TPACKET_V1锛堥粯璁わ級銆乀PACKET_V2銆乀PACKET_V3銆?
TPACKET_V1锛? - 鑻ユ湭閫氳繃 setsockopt(2) 鍙﹁鎸囧畾锛屽垯涓洪粯璁ょ増鏈? - 鎻愪緵 RX_RING銆乀X_RING

TPACKET_V1 --> TPACKET_V2锛? - 鐢变簬 TPACKET_V1 缁撴瀯涓娇鐢ㄤ簡 unsigned long锛屾敼涓?64 浣嶅共鍑€锛?4 bit clean锛夛紝
	  鍥犳涔熻兘鍦?64 浣嶅唴鏍?+ 32 浣嶇敤鎴风┖闂寸瓑缁勫悎涓嬪伐浣? - 鏃堕棿鎴冲垎杈ㄧ巼鐢卞井绉掓敼涓虹撼绉? - 鎻愪緵 RX_RING銆乀X_RING
 - 鏁版嵁鍖呯殑 VLAN 鍏冧俊鎭彲鐢?	  锛圱P_STATUS_VLAN_VALID銆乀P_STATUS_VLAN_TPID_VALID锛夛紝
	  鍦?tpacket2_hdr 缁撴瀯涓細

  - tp_status 瀛楁涓缃簡 TP_STATUS_VLAN_VALID 浣嶏紝琛ㄧず
		  tp_vlan_tci 瀛楁鍚湁鏈夋晥鐨?VLAN TCI 鍊?  - tp_status 瀛楁涓缃簡 TP_STATUS_VLAN_TPID_VALID 浣嶏紝琛ㄧず
		  tp_vlan_tpid 瀛楁鍚湁鏈夋晥鐨?VLAN TPID 鍊?
 - 濡備綍鍒囨崲鍒?TPACKET_V2锛?
  1. 鐢?struct tpacket2_hdr 鏇挎崲 struct tpacket_hdr
  2. 鏌ヨ骞朵繚瀛樺ご閮ㄩ暱搴?  3. 灏嗗崗璁増鏈涓?2锛岀収甯稿缓绔?ring
  4. 鑾峰彇 sockaddr_ll 鏃讹紝
		   浣跨敤 `(void *)hdr + TPACKET_ALIGN(hdrlen)` 鑰岄潪
		   `(void *)hdr + TPACKET_ALIGN(sizeof(struct tpacket_hdr))`

TPACKET_V2 --> TPACKET_V3锛? - RX_RING 鐨勭伒娲荤紦鍐插尯瀹炵幇锛?  1. 鍧楀彲閰嶇疆涓洪潪闈欐€佸抚澶у皬
  2. 璇?poll 鍦ㄥ潡绾у埆锛堣€岄潪鍖呯骇鍒級杩涜
  3. 澧炲姞浜?poll 瓒呮椂锛岄伩鍏嶇敤鎴风┖闂村湪绌洪棽閾捐矾涓婃棤闄愮瓑寰?  4. 澧炲姞浜嗙敤鎴峰彲閰嶇疆鐨勫弬鏁帮細

			4.1 block::timeout
			4.2 tpkt_hdr::sk_rxhash

 - 鐢ㄦ埛绌洪棿鍙幏鍙?RX Hash 鏁版嵁
 - TX_RING 璇箟鍦ㄦ蹇典笂绫讳技浜?TPACKET_V2锛?	  浣跨敤 tpacket3_hdr 鑰岄潪 tpacket2_hdr锛屼互鍙?TPACKET3_HDRLEN
	  鑰岄潪 TPACKET2_HDRLEN銆傚湪褰撳墠瀹炵幇涓紝tpacket3_hdr 涓殑 tp_next_offset
	  瀛楁蹇呴』璁句负闆讹紝琛ㄧず ring 涓嶄繚瀛樺彲鍙樺ぇ灏忕殑甯с€倀p_next_offset 闈為浂鐨?	  鏁版嵁鍖呭皢琚涪寮冦€?
## AF_PACKET fanout 妯″紡


鍦?AF_PACKET fanout 妯″紡涓嬶紝鏁版嵁鍖呮帴鏀跺彲浠ュ湪澶氫釜杩涚▼闂磋繘琛岃礋杞藉潎琛°€傝繖涔熷彲浠ヤ笌
packet 濂楁帴瀛椾笂鐨?mmap(2) 缁撳悎浣跨敤銆?
褰撳墠宸插疄鐜扮殑 fanout 绛栫暐鏈夛細

  - PACKET_FANOUT_HASH锛氭寜 skb 鐨勬暟鎹寘鍝堝笇璋冨害鍒板鎺ュ瓧
  - PACKET_FANOUT_LB锛氭寜杞锛坮ound-robin锛夎皟搴﹀埌濂楁帴瀛?  - PACKET_FANOUT_CPU锛氭寜鏁版嵁鍖呭埌杈剧殑 CPU 璋冨害鍒板鎺ュ瓧
  - PACKET_FANOUT_RND锛氭寜闅忔満閫夋嫨璋冨害鍒板鎺ュ瓧
  - PACKET_FANOUT_ROLLOVER锛氳嫢鏌愪釜濂楁帴瀛楀凡婊★紝鍒欐粴鍔ㄥ埌鍙︿竴涓?  - PACKET_FANOUT_QM锛氭寜 skb 璁板綍鐨?queue_mapping 璋冨害鍒板鎺ュ瓧

鐢?David S. Miller 鎻愪緵鐨勬渶灏忕ず渚嬩唬鐮侊紙鍙互灏濊瘯 "./test eth0 hash" 涔嬬被锛夛細
```

    #include <stddef.h>
    #include <stdlib.h>
    #include <stdio.h>
    #include <string.h>

    #include <sys/types.h>
    #include <sys/wait.h>
    #include <sys/socket.h>
    #include <sys/ioctl.h>

    #include <unistd.h>

    #include <linux/if_ether.h>
    #include <linux/if_packet.h>

    #include <net/if.h>

    static const char *device_name;
    static int fanout_type;
    static int fanout_id;

    #ifndef PACKET_FANOUT
    # define PACKET_FANOUT			18
    # define PACKET_FANOUT_HASH		0
    # define PACKET_FANOUT_LB		1
    #endif

    static int setup_socket(void)
    {
	    int err, fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_IP));
	    struct sockaddr_ll ll;
	    struct ifreq ifr;
	    int fanout_arg;

	    if (fd < 0) {
		    perror("socket");
		    return EXIT_FAILURE;
	    }

	    memset(&ifr, 0, sizeof(ifr));
	    strcpy(ifr.ifr_name, device_name);
	    err = ioctl(fd, SIOCGIFINDEX, &ifr);
	    if (err < 0) {
		    perror("SIOCGIFINDEX");
		    return EXIT_FAILURE;
	    }

	    memset(&ll, 0, sizeof(ll));
	    ll.sll_family = AF_PACKET;
	    ll.sll_ifindex = ifr.ifr_ifindex;
	    err = bind(fd, (struct sockaddr *) &ll, sizeof(ll));
	    if (err < 0) {
		    perror("bind");
		    return EXIT_FAILURE;
	    }

	    fanout_arg = (fanout_id | (fanout_type << 16));
	    err = setsockopt(fd, SOL_PACKET, PACKET_FANOUT,
			    &fanout_arg, sizeof(fanout_arg));
	    if (err) {
		    perror("setsockopt");
		    return EXIT_FAILURE;
	    }

	    return fd;
    }

    static void fanout_thread(void)
    {
	    int fd = setup_socket();
	    int limit = 10000;

	    if (fd < 0)
		    exit(fd);

	    while (limit-- > 0) {
		    char buf[1600];
		    int err;

		    err = read(fd, buf, sizeof(buf));
		    if (err < 0) {
			    perror("read");
			    exit(EXIT_FAILURE);
		    }
		    if ((limit % 10) == 0)
			    fprintf(stdout, "(%d) \n", getpid());
	    }

	    fprintf(stdout, "%d: Received 10000 packets\n", getpid());

	    close(fd);
	    exit(0);
    }

    int main(int argc, char **argp)
    {
	    int fd, err;
	    int i;

	    if (argc != 3) {
		    fprintf(stderr, "Usage: %s INTERFACE {hash|lb}\n", argp[0]);
		    return EXIT_FAILURE;
	    }

	    if (!strcmp(argp[2], "hash"))
		    fanout_type = PACKET_FANOUT_HASH;
	    else if (!strcmp(argp[2], "lb"))
		    fanout_type = PACKET_FANOUT_LB;
	    else {
		    fprintf(stderr, "Unknown fanout type [%s]\n", argp[2]);
		    exit(EXIT_FAILURE);
	    }

	    device_name = argp[1];
	    fanout_id = getpid() & 0xffff;

	    for (i = 0; i < 4; i++) {
		    pid_t pid = fork();

		    switch (pid) {
		    case 0:
			    fanout_thread();

		    case -1:
			    perror("fork");
			    exit(EXIT_FAILURE);
		    }
	    }

	    for (i = 0; i < 4; i++) {
		    int status;

		    wait(&status);
	    }

	    return 0;
    }

```
## AF_PACKET TPACKET_V3 绀轰緥


AF_PACKET 鐨?TPACKET_V3 鐜舰缂撳啿鍖哄彲閰嶇疆涓轰娇鐢ㄩ潪闈欐€佸抚澶у皬锛岄€氳繃瀹冭嚜韬殑鍐呭瓨绠＄悊
瀹炵幇銆傚畠鍩轰簬鍧楋紙block锛夊伐浣滐紝杞锛坧olling锛夋寜姣忓潡杩涜锛岃€岄潪鍍?TPACKET_V2 鍙婂叾鍓嶈韩
閭ｆ牱鎸夋瘡涓?ring 杩涜銆?
鎹 TPACKET_V3 甯︽潵浠ヤ笅濂藉锛?
 - CPU 浣跨敤鐜囬檷浣庣害 15% - 20%
 - 鏁版嵁鍖呮崟鑾风巼鎻愬崌绾?20%
 - 鏁版嵁鍖呭瘑搴︽彁鍗囩害 2 鍊? - 绔彛鑱氬悎鍒嗘瀽
 - 闈為潤鎬佸抚澶у皬浠ユ崟鑾峰畬鏁寸殑鏁版嵁鍖呰礋杞?
鍥犳瀹冧技涔庢槸閰嶅悎 packet fanout 浣跨敤鐨勮壇濂藉€欓€夈€?
鐢?Daniel Borkmann 鍩轰簬 Chetan Loke 鐨?lolpcap 鎻愪緵鐨勬渶灏忕ず渚嬩唬鐮侊紙缂栬瘧
```

    /* Written from scratch, but kernel-to-user space API usage
    * dissected from lolpcap:
    *  Copyright 2011, Chetan Loke <loke.chetan@gmail.com>
    *  License: GPL, version 2.0
    */

    #include <stdio.h>
    #include <stdlib.h>
    #include <stdint.h>
    #include <string.h>
    #include <assert.h>
    #include <net/if.h>
    #include <arpa/inet.h>
    #include <netdb.h>
    #include <poll.h>
    #include <unistd.h>
    #include <signal.h>
    #include <inttypes.h>
    #include <sys/socket.h>
    #include <sys/mman.h>
    #include <linux/if_packet.h>
    #include <linux/if_ether.h>
    #include <linux/ip.h>

    #ifndef likely
    # define likely(x)		__builtin_expect(!!(x), 1)
    #endif
    #ifndef unlikely
    # define unlikely(x)		__builtin_expect(!!(x), 0)
    #endif

    struct block_desc {
	    uint32_t version;
	    uint32_t offset_to_priv;
	    struct tpacket_hdr_v1 h1;
    };

    struct ring {
	    struct iovec *rd;
	    uint8_t *map;
	    struct tpacket_req3 req;
    };

    static unsigned long packets_total = 0, bytes_total = 0;
    static sig_atomic_t sigint = 0;

    static void sighandler(int num)
    {
	    sigint = 1;
    }

    static int setup_socket(struct ring *ring, char *netdev)
    {
	    int err, i, fd, v = TPACKET_V3;
	    struct sockaddr_ll ll;
	    unsigned int blocksiz = 1 << 22, framesiz = 1 << 11;
	    unsigned int blocknum = 64;

	    fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
	    if (fd < 0) {
		    perror("socket");
		    exit(1);
	    }

	    err = setsockopt(fd, SOL_PACKET, PACKET_VERSION, &v, sizeof(v));
	    if (err < 0) {
		    perror("setsockopt");
		    exit(1);
	    }

	    memset(&ring->req, 0, sizeof(ring->req));
	    ring->req.tp_block_size = blocksiz;
	    ring->req.tp_frame_size = framesiz;
	    ring->req.tp_block_nr = blocknum;
	    ring->req.tp_frame_nr = (blocksiz * blocknum) / framesiz;
	    ring->req.tp_retire_blk_tov = 60;
	    ring->req.tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;

	    err = setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &ring->req,
			    sizeof(ring->req));
	    if (err < 0) {
		    perror("setsockopt");
		    exit(1);
	    }

	    ring->map = mmap(NULL, ring->req.tp_block_size * ring->req.tp_block_nr,
			    PROT_READ | PROT_WRITE, MAP_SHARED | MAP_LOCKED, fd, 0);
	    if (ring->map == MAP_FAILED) {
		    perror("mmap");
		    exit(1);
	    }

	    ring->rd = malloc(ring->req.tp_block_nr * sizeof(*ring->rd));
	    assert(ring->rd);
	    for (i = 0; i < ring->req.tp_block_nr; ++i) {
		    ring->rd[i].iov_base = ring->map + (i * ring->req.tp_block_size);
		    ring->rd[i].iov_len = ring->req.tp_block_size;
	    }

	    memset(&ll, 0, sizeof(ll));
	    ll.sll_family = PF_PACKET;
	    ll.sll_protocol = htons(ETH_P_ALL);
	    ll.sll_ifindex = if_nametoindex(netdev);
	    ll.sll_hatype = 0;
	    ll.sll_pkttype = 0;
	    ll.sll_halen = 0;

	    err = bind(fd, (struct sockaddr *) &ll, sizeof(ll));
	    if (err < 0) {
		    perror("bind");
		    exit(1);
	    }

	    return fd;
    }

    static void display(struct tpacket3_hdr *ppd)
    {
	    struct ethhdr *eth = (struct ethhdr *) ((uint8_t *) ppd + ppd->tp_mac);
	    struct iphdr *ip = (struct iphdr *) ((uint8_t *) eth + ETH_HLEN);

	    if (eth->h_proto == htons(ETH_P_IP)) {
		    struct sockaddr_in ss, sd;
		    char sbuff[NI_MAXHOST], dbuff[NI_MAXHOST];

		    memset(&ss, 0, sizeof(ss));
		    ss.sin_family = PF_INET;
		    ss.sin_addr.s_addr = ip->saddr;
		    getnameinfo((struct sockaddr *) &ss, sizeof(ss),
				sbuff, sizeof(sbuff), NULL, 0, NI_NUMERICHOST);

		    memset(&sd, 0, sizeof(sd));
		    sd.sin_family = PF_INET;
		    sd.sin_addr.s_addr = ip->daddr;
		    getnameinfo((struct sockaddr *) &sd, sizeof(sd),
				dbuff, sizeof(dbuff), NULL, 0, NI_NUMERICHOST);

		    printf("%s -> %s, ", sbuff, dbuff);
	    }

	    printf("rxhash: 0x%x\n", ppd->hv1.tp_rxhash);
    }

    static void walk_block(struct block_desc *pbd, const int block_num)
    {
	    int num_pkts = pbd->h1.num_pkts, i;
	    unsigned long bytes = 0;
	    struct tpacket3_hdr *ppd;

	    ppd = (struct tpacket3_hdr *) ((uint8_t *) pbd +
					pbd->h1.offset_to_first_pkt);
	    for (i = 0; i < num_pkts; ++i) {
		    bytes += ppd->tp_snaplen;
		    display(ppd);

		    ppd = (struct tpacket3_hdr *) ((uint8_t *) ppd +
						ppd->tp_next_offset);
	    }

	    packets_total += num_pkts;
	    bytes_total += bytes;
    }

    static void flush_block(struct block_desc *pbd)
    {
	    pbd->h1.block_status = TP_STATUS_KERNEL;
    }

    static void teardown_socket(struct ring *ring, int fd)
    {
	    munmap(ring->map, ring->req.tp_block_size * ring->req.tp_block_nr);
	    free(ring->rd);
	    close(fd);
    }

    int main(int argc, char **argp)
    {
	    int fd, err;
	    socklen_t len;
	    struct ring ring;
	    struct pollfd pfd;
	    unsigned int block_num = 0, blocks = 64;
	    struct block_desc *pbd;
	    struct tpacket_stats_v3 stats;

	    if (argc != 2) {
		    fprintf(stderr, "Usage: %s INTERFACE\n", argp[0]);
		    return EXIT_FAILURE;
	    }

	    signal(SIGINT, sighandler);

	    memset(&ring, 0, sizeof(ring));
	    fd = setup_socket(&ring, argp[argc - 1]);
	    assert(fd > 0);

	    memset(&pfd, 0, sizeof(pfd));
	    pfd.fd = fd;
	    pfd.events = POLLIN | POLLERR;
	    pfd.revents = 0;

	    while (likely(!sigint)) {
		    pbd = (struct block_desc *) ring.rd[block_num].iov_base;

		    if ((pbd->h1.block_status & TP_STATUS_USER) == 0) {
			    poll(&pfd, 1, -1);
			    continue;
		    }

		    walk_block(pbd, block_num);
		    flush_block(pbd);
		    block_num = (block_num + 1) % blocks;
	    }

	    len = sizeof(stats);
	    err = getsockopt(fd, SOL_PACKET, PACKET_STATISTICS, &stats, &len);
	    if (err < 0) {
		    perror("getsockopt");
		    exit(1);
	    }

	    fflush(stdout);
	    printf("\nReceived %u packets, %lu bytes, %u dropped, freeze_q_cnt: %u\n",
		stats.tp_packets, bytes_total, stats.tp_drops,
		stats.tp_freeze_q_cnt);

	    teardown_socket(&ring, fd);
	    return 0;
    }

```
## PACKET_QDISC_BYPASS


濡傛灉鏈夐渶姹傚儚 pktgen 閭ｆ牱鐢ㄥぇ閲忔暟鎹寘鐏屾弧缃戠粶锛屼綘鍙互鍦ㄥ垱寤哄鎺ュ瓧鍚庤缃涓?閫夐」
```

    int one = 1;
    setsockopt(fd, SOL_PACKET, PACKET_QDISC_BYPASS, &one, sizeof(one));

```
杩欐湁涓€涓壇浣滅敤锛氶€氳繃 PF_PACKET 鍙戦€佺殑鏁版嵁鍖呬細缁曡繃鍐呮牳鐨?qdisc 灞傦紝琚己鍒剁洿鎺ユ帹缁?椹卞姩銆備篃灏辨槸璇达紝鏁版嵁鍖呬笉浼氳缂撳啿锛宼c 瑙勫垯锛坉isciplines锛夎蹇界暐锛屽彲鑳戒細澧炲姞涓㈠寘锛屽苟涓?杩欑被鏁版嵁鍖呭鍏朵粬 PF_PACKET 濂楁帴瀛椾篃涓嶅啀鍙銆傚洜姝わ紝杩欓噷宸叉彁閱掍綘锛涗竴鑸潵璇达紝杩欏
鍘嬪姏娴嬭瘯绯荤粺鐨勫悇涓粍浠朵細寰堟湁鐢ㄣ€?
榛樿鎯呭喌涓嬶紝PACKET_QDISC_BYPASS 鏄鐢ㄧ殑锛岄渶瑕佸湪 PF_PACKET 濂楁帴瀛椾笂鏄惧紡鍚敤銆?
## PACKET_TIMESTAMP


PACKET_TIMESTAMP 璁剧疆鍐冲畾浜?mmap(2) 鏄犲皠鐨?RX_RING 鍜?TX_RING 涓暟鎹寘鍏冧俊鎭殑
鏃堕棿鎴虫潵婧愩€傚鏋滀綘鐨?NIC 鑳藉鍦ㄧ‖浠朵腑瀵规暟鎹寘鎵撴椂闂存埑锛屼綘鍙互璇锋眰浣跨敤杩欎簺纭欢
鏃堕棿鎴炽€傛敞鎰忥細浣犲彲鑳介渶瑕侀€氳繃 SIOCSHWTSTAMP 鍚敤纭欢鏃堕棿鎴崇殑鐢熸垚锛堝弬瑙?Documentation/networking/timestamping.rst 涓殑鐩稿叧淇℃伅锛夈€?```

    int req = SOF_TIMESTAMPING_RAW_HARDWARE;
    setsockopt(fd, SOL_PACKET, PACKET_TIMESTAMP, (void *) &req, sizeof(req))

```
瀵逛簬 mmap(2) 鏄犲皠鐨勭幆褰㈢紦鍐插尯锛岃繖绫绘椂闂存埑瀛樺偍鍦?`tpacket{,2,3}_hdr` 缁撴瀯鐨?tp_sec
鍜?`tp_{n,u}sec` 鎴愬憳涓€傝纭畾鎶ュ憡浜嗗摢绉嶆椂闂存埑锛宼p_status 瀛楁涓庝互涓嬪彲鑳界殑浣嶈繘琛?浜岃繘鍒舵垨杩愮畻鈥︹€?
```

    TP_STATUS_TS_RAW_HARDWARE
    TP_STATUS_TS_SOFTWARE

```
鈥︹€﹀畠浠瓑浠蜂簬鍏?`SOF_TIMESTAMPING_*` 瀵瑰簲鐨勪綅銆傚浜?RX_RING锛屽鏋滀袱鑰呴兘娌℃湁璁剧疆
锛堝嵆鏈缃?PACKET_TIMESTAMP锛夛紝鍒欏湪 PF_PACKET 鐨勫鐞嗕唬鐮佸唴閮ㄨ皟鐢ㄤ簡杞欢鍥為€€锛堢簿搴﹁緝浣庯級銆?
鑾峰彇 TX_RING 鐨勬椂闂存埑杩囩▼濡備笅锛歩) 濉厖 ring 甯э紝ii) 璋冪敤 sendto()锛屼緥濡傚湪闃诲妯″紡涓嬶紝
iii) 绛夊緟鐩稿叧甯х殑鐘舵€佽鏇存柊锛屽嵆璇ュ抚琚氦鍥炵粰搴旂敤绋嬪簭锛宨v) 閬嶅巻鍚勫抚浠ュ彇鍑哄悇鑷殑纭欢/
杞欢鏃堕棿鎴炽€?
鍙湁锛?锛夊湪鍚敤浜嗗彂閫佹椂闂存埑鏃讹紝杩欎簺浣嶆墠浼氫笌 TP_STATUS_AVAILABLE 杩涜浜岃繘鍒?| 杩愮畻锛?鍥犳浣犲繀椤诲湪搴旂敤绋嬪簭涓鏌ュ畠锛堜緥濡傚厛閫氳繃 !(tp_status & (TP_STATUS_SEND_REQUEST |
TP_STATUS_SENDING)) 鍒ゆ柇璇ュ抚鏄惁灞炰簬搴旂敤绋嬪簭锛岀劧鍚庡湪绗簩姝ヤ粠 tp_status 涓彁鍙栨椂闂存埑绫诲瀷锛夛紒

濡傛灉浣犱笉鍦ㄤ箮瀹冧滑锛屽嵆淇濇寔绂佺敤锛岄偅涔堟鏌?TP_STATUS_AVAILABLE 鎴?TP_STATUS_WRONG_FORMAT
灏辫冻澶熶簡銆傚鏋滃湪 TX_RING 閮ㄥ垎鍙缃簡 TP_STATUS_AVAILABLE锛岄偅涔?tp_sec 鍜?tp_{n,u}sec
鎴愬憳涓嶅寘鍚湁鏁堝€笺€傚浜?TX_RING锛岄粯璁や笉鐢熸垚鏃堕棿鎴筹紒

鏈夊叧纭欢鏃堕棿鎴崇殑鏇村淇℃伅锛岃鍙傝 include/linux/net_tstamp.h 鍜?Documentation/networking/timestamping.rst銆?
## 鏉傞」


- Packet 濂楁帴瀛椾笌 Linux socket 杩囨护鍣ㄩ厤鍚堜娇鐢ㄥ緱寰堝ソ锛屽洜姝や綘鍙兘涔熸兂鐪嬬湅
  Documentation/networking/filter.rst

## 鑷磋阿


   Jesse Brandeburg锛屾劅璋粬淇浜嗘垜鐨勮娉?鎷煎啓閿欒
