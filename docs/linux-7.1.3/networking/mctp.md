
## 绠＄悊缁勪欢浼犺緭鍗忚锛圡anagement Component Transport Protocol锛孧CTP锛?


net/mctp/ 鍖呭惈浜?MCTP 鐨勫崗璁敮鎸侊紝鍏跺畾涔夎 DMTF 鏍囧噯 DSP0236銆傜墿鐞嗘帴鍙ｉ┍鍔紙瑙勮寖涓О涓衡€渂indings鈥濓級鐢?drivers/net/mctp/ 鎻愪緵銆?

鏍稿績浠ｇ爜閫氳繃 AF_MCTP銆丼OCK_DGRAM 濂楁帴瀛楁彁渚涗簡涓€涓熀浜庡鎺ュ瓧鐨勬帴鍙ｏ紝鐢ㄤ簬鍙戦€佸拰鎺ユ敹 MCTP 娑堟伅銆?

## 缁撴瀯锛氭帴鍙ｄ笌缃戠粶


鍐呮牳閫氳繃涓や釜瑕佺礌涓烘湰鍦?MCTP 鎷撴墤寤烘ā锛氭帴鍙ｏ紙interface锛夊拰缃戠粶锛坣etwork锛夈€?

鎺ュ彛锛堟垨绉扳€渓ink鈥濓級鏄?MCTP 鐗╃悊浼犺緭缁戝畾锛堢敱 DSP0236 绗?3.2.47 鑺傚畾涔夛級鐨勪竴涓疄渚嬶紝鍙兘杩炴帴鍒版煇涓壒瀹氱殑纭欢璁惧銆傚畠琛ㄧず涓轰竴涓?`struct netdevice`銆?

缃戠粶閫氳繃绔偣 ID锛坋ndpoint-ID锛岀敱 DSP0236 绗?3.2.31 鑺傛弿杩帮級涓?MCTP 绔偣瀹氫箟浜嗕竴涓敮涓€鐨勫湴鍧€绌洪棿銆傜綉缁滄湁涓€涓鐢ㄦ埛鍙鐨勬爣璇嗙锛屼互渚夸粠鐢ㄦ埛绌洪棿寮曠敤銆傝矾鐢卞畾涔夌壒瀹氫簬鏌愪竴涓綉缁溿€?

鎺ュ彛涓庢煇涓€涓綉缁滃叧鑱斻€備竴涓綉缁滃彲浠ヤ笌涓€涓垨澶氫釜鎺ュ彛鍏宠仈銆?

濡傛灉瀛樺湪澶氫釜缃戠粶锛屾瘡涓綉缁滈兘鍙兘鍖呭惈涔熷嚭鐜板湪鍏朵粬缃戠粶涓婄殑绔偣 ID锛圗ID锛夈€?

## 濂楁帴瀛?API


### 鍗忚瀹氫箟


MCTP 浣跨敤 `AF_MCTP` / `PF_MCTP` 浣滀负鍦板潃鏃忓拰鍗忚鏃忋€傜敱浜?MCTP 鏄熀浜庢秷鎭殑锛屽洜姝や粎鏀寔 `SOCK_DGRAM` 濂楁帴瀛椼€?

    int sd = socket(AF_MCTP, SOCK_DGRAM, 0);

`protocol` 鍙傛暟褰撳墠鍞竴锛堜笖鏈夋晥锛夌殑鍊兼槸 0銆?

涓庢墍鏈夊鎺ュ瓧鍦板潃鏃忎竴鏍凤紝婧愬湴鍧€鍜岀洰鐨勫湴鍧€浣跨敤 `sockaddr` 绫诲瀷鎸囧畾锛屽叾涓寘鍚竴涓崟瀛楄妭鐨勭鐐瑰湴鍧€锛?

    typedef __u8		mctp_eid_t;

    struct mctp_addr {
            mctp_eid_t		s_addr;
    };

    struct sockaddr_mctp {
            __kernel_sa_family_t smctp_family;
            unsigned int         smctp_network;
            struct mctp_addr     smctp_addr;
            __u8                 smctp_type;
            __u8                 smctp_tag;
    };

    #define MCTP_NET_ANY	0x0
    #define MCTP_ADDR_ANY	0xff

### 绯荤粺璋冪敤琛屼负


浠ヤ笅鍚勮妭鎻忚堪浜嗘爣鍑嗗鎺ュ瓧绯荤粺璋冪敤涓笌 MCTP 鐩稿叧鐨勮涓恒€傝繖浜涜涓鸿閫夋嫨涓轰笌鐜版湁鐨勫鎺ュ瓧 API 绱у瘑瀵瑰簲銆?

##### ``bind()`` 锛氳缃湰鍦板鎺ュ瓧鍦板潃


鎺ユ敹浼犲叆璇锋眰鏁版嵁鍖呯殑濂楁帴瀛楀皢浣跨敤 `bind()` 绯荤粺璋冪敤鏉ョ粦瀹氬埌涓€涓湰鍦板湴鍧€銆?

    struct sockaddr_mctp addr;

    addr.smctp_family = AF_MCTP;
    addr.smctp_network = MCTP_NET_ANY;
    addr.smctp_addr.s_addr = MCTP_ADDR_ANY;
    addr.smctp_type = MCTP_TYPE_PLDM;
    addr.smctp_tag = MCTP_TAG_OWNER;

    int rc = bind(sd, (struct sockaddr *)&addr, sizeof(addr));

杩欏皢寤虹珛濂楁帴瀛楃殑鏈湴鍦板潃銆備笌姝ょ綉缁滅殑缃戠粶鍙枫€佸湴鍧€鍜屾秷鎭被鍨嬬浉鍖归厤鐨勪紶鍏?MCTP 娑堟伅灏嗚璇ュ鎺ュ瓧鎺ユ敹銆傝繖閲屽鈥渋ncoming鈥濈殑寮曠敤寰堥噸瑕侊紱缁戝畾鍚庣殑濂楁帴瀛楀彧浼氭帴鏀惰缃簡 TO 浣嶇殑娑堟伅锛岃〃绀鸿繖鏄竴鏉′紶鍏ョ殑璇锋眰娑堟伅锛岃€屼笉鏄搷搴斻€?

`smctp_tag` 鐨勫€煎皢閰嶇疆浠庤濂楁帴瀛楄繙绔帴鍙楃殑鏍囩銆傛牴鎹互涓婅鏄庯紝鍞竴鏈夋晥鐨勫€兼槸 `MCTP_TAG_OWNER`锛岃繖灏嗕娇杩滅鈥滄嫢鏈夆€濈殑鏍囩琚矾鐢卞埌璇ュ鎺ュ瓧銆傜敱浜庤缃簡 `MCTP_TAG_OWNER`锛宍smctp_tag` 鐨勬渶浣?3 浣嶄笉浼氳浣跨敤锛涜皟鐢ㄨ€呭繀椤诲皢瀹冧滑璁句负闆躲€?

`smctp_network` 鐨勫€间负 `MCTP_NET_ANY` 鏃讹紝浼氬皢濂楁帴瀛楅厤缃负鎺ユ敹鏉ヨ嚜浠讳綍鏈湴杩炴帴缃戠粶鐨勪紶鍏ユ暟鎹寘銆傛寚瀹氭煇涓綉缁滃€煎垯浼氫娇濂楁帴瀛楀彧鎺ユ敹鏉ヨ嚜璇ョ綉缁滅殑浼犲叆娑堟伅銆?

`smctp_addr` 瀛楁鎸囧畾瑕佺粦瀹氱殑鏈湴鍦板潃銆傚€间负 `MCTP_ADDR_ANY` 鏃讹紝灏嗗鎺ュ瓧閰嶇疆涓烘帴鏀跺鍧€鍒颁换浣曟湰鍦扮洰鐨?EID 鐨勬秷鎭€?

`smctp_type` 瀛楁鎸囧畾瑕佹帴鏀剁殑娑堟伅绫诲瀷銆備紶鍏ユ秷鎭彧鍖归厤绫诲瀷鐨勪綆 7 浣嶏紙鍗虫渶楂樹綅鐨?IC 浣嶄笉鍙備笌鍖归厤锛夈€傝繖瀵艰嚧濂楁帴瀛楁棦鑳芥帴鏀跺甫鏈夋秷鎭畬鏁存€ф鏌ュ熬閮ㄧ殑鏁版嵁鍖咃紝涔熻兘鎺ユ敹涓嶅甫璇ュ熬閮ㄧ殑鏁版嵁鍖呫€?

##### ``sendto()``銆乣`sendmsg()``銆乣`send()`` 锛氬彂閫佷竴鏉?MCTP 娑堟伅


涓€鏉?MCTP 娑堟伅鍙互浣跨敤 `sendto()`銆乣sendmsg()` 鎴?`send()` 绯荤粺璋冪敤涔嬩竴鍙戦€併€備互 `sendto()` 浣滀负涓昏绀轰緥锛?

    struct sockaddr_mctp addr;
    char buf[^14^];
    ssize_t len;

    /** set message destination **/
    addr.smctp_family = AF_MCTP;
    addr.smctp_network = 0;
    addr.smctp_addr.s_addr = 8;
    addr.smctp_tag = MCTP_TAG_OWNER;
    addr.smctp_type = MCTP_TYPE_ECHO;

    /** arbitrary message to send, with message-type header **/
    buf[^0^] = MCTP_TYPE_ECHO;
    memcpy(buf + 1, "hello, world!", sizeof(buf) - 1);

    len = sendto(sd, buf, sizeof(buf), 0,
                    (struct sockaddr_mctp *)&addr, sizeof(addr));

`addr` 鐨勭綉缁滃拰鍦板潃瀛楁瀹氫箟浜嗚鍙戦€佸埌鐨勮繙绔湴鍧€銆傚鏋?`smctp_tag` 甯︽湁 `MCTP_TAG_OWNER`锛屽唴鏍稿皢蹇界暐 `MCTP_TAG_VALUE` 涓缃殑浠讳綍浣嶏紝骞朵负鐩殑 EID 鐢熸垚涓€涓悎閫傜殑鏍囩鍊笺€傚鏋滄湭璁剧疆 `MCTP_TAG_OWNER`锛屾秷鎭皢鎸夋寚瀹氱殑鏍囩鍊煎彂閫併€傚鏋滄棤娉曞垎閰嶆爣绛撅紝绯荤粺璋冪敤灏嗚繑鍥?`EAGAIN` 閿欒鐮併€?

搴旂敤绋嬪簭蹇呴』灏嗘秷鎭被鍨嬪瓧鑺備綔涓轰紶鍏?`sendto()` 鐨勬秷鎭紦鍐插尯鐨勭涓€涓瓧鑺傛彁渚涖€傚鏋滆鍦ㄥ彂閫佺殑娑堟伅涓寘鍚秷鎭畬鏁存€ф鏌ワ紝涔熷繀椤诲皢鍏舵斁鍦ㄦ秷鎭紦鍐插尯涓紝骞朵笖娑堟伅绫诲瀷瀛楄妭鐨勬渶楂樹綅蹇呴』涓?1銆?

`sendmsg()` 绯荤粺璋冪敤鍏佽鏇寸揣鍑戠殑鍙傛暟鎺ュ彛锛屽苟鍏佽灏嗘秷鎭紦鍐插尯鎸囧畾涓哄垎鏁?鑱氶泦锛坰catter-gather锛夊垪琛ㄣ€傜洰鍓嶆病鏈夊畾涔変换浣曡緟鍔╂秷鎭被鍨嬶紙鐢ㄤ簬浼犵粰 `sendmsg()` 鐨?`msg_control` 鏁版嵁锛夈€?

鍦ㄦ湭杩炴帴鐨勫鎺ュ瓧涓婂彂閫佹秷鎭椂鑻ユ寚瀹氫簡 `MCTP_TAG_OWNER`锛屽垯濡傛灉灏氭湭涓鸿鐩殑鍦板潃鍒嗛厤鏈夋晥鏍囩锛屽皢瀵艰嚧鍒嗛厤涓€涓爣绛俱€傦紙鐩殑 EID锛屾爣绛撅級鍏冪粍鍏呭綋闅愬紡鐨勬湰鍦板鎺ュ瓧鍦板潃锛屼娇濂楁帴瀛楄兘澶熸帴鏀舵鍑虹珯娑堟伅鐨勫搷搴斻€傚鏋滀箣鍓嶅凡缁忔墽琛岃繃鍒嗛厤锛堥拡瀵逛笉鍚岀殑杩滅 EID锛夛紝鍒欒鍒嗛厤灏嗕涪澶便€?

濂楁帴瀛楀彧浼氭帴鏀跺畠浠墍鍙戦€佽姹傦紙TO=1锛夌殑鍝嶅簲锛屽苟涓斿彧鑳藉瀹冧滑鏀跺埌鐨勮姹傦紙TO=0锛変綔鍑哄搷搴斻€?

##### ``recvfrom()``銆乣`recvmsg()``銆乣`recv()`` 锛氭帴鏀朵竴鏉?MCTP 娑堟伅


搴旂敤绋嬪簭鍙互浣跨敤 `recvfrom()`銆乣recvmsg()` 鎴?`recv()` 绯荤粺璋冪敤涔嬩竴鎺ユ敹 MCTP 娑堟伅銆備互 `recvfrom()` 浣滀负涓昏绀轰緥锛?

    struct sockaddr_mctp addr;
    socklen_t addrlen;
    char buf[^14^];
    ssize_t len;

    addrlen = sizeof(addr);

    len = recvfrom(sd, buf, sizeof(buf), 0,
                    (struct sockaddr_mctp *)&addr, &addrlen);

    /** We can expect addr to describe an MCTP address **/
    assert(addrlen >= sizeof(buf));
    assert(addr.smctp_family == AF_MCTP);

    printf("received %zd bytes from remote EID %d
", rc, addr.smctp_addr);

浼犵粰 `recvfrom` 鍜?`recvmsg` 鐨勫湴鍧€鍙傛暟浼氳濉叆浼犲叆娑堟伅鐨勮繙绔湴鍧€锛屽寘鎷爣绛惧€硷紙鍥炲璇ユ秷鎭椂浼氶渶瑕佸畠锛夈€?

娑堟伅缂撳啿鍖虹殑绗竴涓瓧鑺傚皢鍖呭惈娑堟伅绫诲瀷瀛楄妭銆傚鏋滄秷鎭悗闈㈣窡闅忔湁瀹屾暣鎬ф鏌ワ紝瀹冧篃浼氳鍖呭惈鍦ㄦ帴鏀跺埌鐨勭紦鍐插尯涓€?

`recv()` 绯荤粺璋冪敤鐨勮涓虹被浼硷紝浣嗕笉浼氬悜搴旂敤绋嬪簭鎻愪緵杩滅鍦板潃銆傚洜姝わ紝鍙湁鍦ㄨ繙绔湴鍧€宸茬粡宸茬煡锛屾垨鑰呮秷鎭笉闇€瑕佸洖澶嶆椂锛屽畠鎵嶆湁鐢ㄣ€?

涓庡彂閫佽皟鐢ㄤ竴鏍凤紝濂楁帴瀛楀彧浼氭帴鏀跺畠浠墍鍙戦€佽姹傦紙TO=1锛夌殑鍝嶅簲锛屽苟涓斿彧鑳藉瀹冧滑鏀跺埌鐨勮姹傦紙TO=0锛変綔鍑哄搷搴斻€?

##### ``ioctl(SIOCMCTPALLOCTAG)`` 涓?``ioctl(SIOCMCTPDROPTAG)``


杩欎簺鏍囩璁╁簲鐢ㄧ▼搴忚兘澶熷 MCTP 娑堟伅鏍囩鏈夋洿澶氱殑鎺у埗锛屾柟娉曟槸鏄惧紡鍦板垎閰嶏紙鍜岄噴鏀撅級鏍囩鍊硷紝鑰屼笉鏄敱鍐呮牳鍦?`sendmsg()` 鏃惰嚜鍔ㄥ垎閰嶆瘡娑堟伅鏍囩銆?

涓€鑸潵璇达紝鍙湁褰撴偍鐨?MCTP 鍗忚涓嶇鍚堥€氬父鐨勮姹?鍝嶅簲妯″瀷鏃讹紝鎵嶉渶瑕佷娇鐢ㄨ繖浜?ioctl銆備緥濡傦紝濡傛灉鎮ㄩ渶瑕佸湪澶氫釜璇锋眰涔嬮棿淇濇寔鏍囩锛屾垨鑰呬竴涓姹傚彲鑳戒骇鐢熷涓搷搴斻€傚湪杩欎簺鎯呭喌涓嬶紝ioctl 鍏佽鎮ㄥ皢鏍囩鐨勫垎閰嶏紙鍜岄噴鏀撅級涓庡崟涓秷鎭殑鍙戦€佸拰鎺ユ敹鎿嶄綔瑙ｈ€︺€?

涓や釜 ioctl 閮戒紶鍏ヤ竴涓寚鍚?`struct mctp_ioc_tag_ctl` 鐨勬寚閽堬細

    struct mctp_ioc_tag_ctl {
        mctp_eid_t      peer_addr;
        __u8		tag;
        __u16   	flags;
    };

`SIOCMCTPALLOCTAG` 涓轰竴涓壒瀹氱殑瀵圭瓑鏂瑰垎閰嶄竴涓爣绛撅紝搴旂敤绋嬪簭鍙互鍦ㄥ皢鏉ョ殑 `sendmsg()` 璋冪敤涓娇鐢ㄥ畠銆傚簲鐢ㄧ▼搴忕敤杩滅 EID 濉厖 `peer_addr` 鎴愬憳銆傚叾浠栧瓧娈靛繀椤讳负闆躲€?

杩斿洖鏃讹紝`tag` 鎴愬憳灏嗚濉叆宸插垎閰嶇殑鏍囩鍊笺€傚凡鍒嗛厤鐨勬爣绛惧皢璁剧疆浠ヤ笅鏍囩浣嶏細

 - `MCTP_TAG_OWNER`锛氬彧鏈夊綋鎮ㄦ槸鏍囩鎷ユ湁鑰呮椂锛屽垎閰嶆爣绛炬墠鏈夋剰涔?

 - `MCTP_TAG_PREALLOC`锛氱敤浜庡悜 `sendmsg()` 琛ㄦ槑杩欐槸涓€涓鍒嗛厤鐨勬爣绛?

 - 鈥︹€︿互鍙婂疄闄呯殑鏍囩鍊硷紝浣嶄簬鏈€浣庝笁浣嶏紙`MCTP_TAG_MASK`锛変腑銆傛敞鎰忥紝闆舵槸涓€涓湁鏁堢殑鏍囩鍊笺€?

璇ユ爣绛惧€煎簲鎸夊師鏍风敤浜?``struct sockaddr_mctp`` 鐨?`smctp_tag` 鎴愬憳銆?

`SIOCMCTPDROPTAG` 閲婃斁涓€涓箣鍓嶇敱 `SIOCMCTPALLOCTAG` ioctl 鍒嗛厤鐨勬爣绛俱€俙peer_addr` 蹇呴』涓庡垎閰嶆椂浣跨敤鐨勭浉鍚岋紝骞朵笖 `tag` 鍊煎繀椤荤簿纭尮閰嶅垎閰嶆椂杩斿洖鐨勬爣绛撅紙鍖呮嫭 `MCTP_TAG_OWNER` 鍜?`MCTP_TAG_PREALLOC` 浣嶏級銆俙flags` 瀛楁蹇呴』涓洪浂銆?

```

	sendmsg()
	 -> mctp_local_output()
	    : route lookup
	    -> rt->output() (== mctp_route_output)
	       -> dev_queue_xmit()

```
```

	sendmsg()
	-> mctp_local_output()
	    -> mctp_do_fragment_route()
	       : creates packet-sized skbs. For each new skb:
	       -> rt->output() (== mctp_route_output)
	          -> dev_queue_xmit()

```
```

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   -> sock_queue_rcv_skb()

```
```

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   : stores skb in struct sk_key->reasm_head

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   : finds existing reassembly in sk_key->reasm_head
	   : appends new fragment
	   -> sock_queue_rcv_skb()

```
### 鍏抽敭寮曠敤璁℃暟


 - 閿殑寮曠敤鏉ヨ嚜锛?

   - 涓€涓?skb锛氬湪璺敱杈撳嚭鏈熼棿锛屽瓨鍌ㄥ湪 `skb->cb` 涓€?

   - netns 鍜?sock 鍒楄〃銆?

 - 閿彲浠ヤ笌涓€涓澶囧叧鑱旓紝姝ゆ椂瀹冧滑鎸佹湁瀵硅璁惧锛坉ev锛夌殑寮曠敤锛堥€氳繃 `key->dev` 璁剧疆锛岄€氳繃 `dev->key_count` 璁℃暟锛夈€傚涓敭鍙互寮曠敤鍚屼竴涓澶囥€?

