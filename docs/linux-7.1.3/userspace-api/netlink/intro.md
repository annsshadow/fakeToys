
## Netlink 绠€浠?

Netlink 甯歌鎻忚堪涓?ioctl() 鐨勬浛浠ｅ搧銆傚畠鏃ㄥ湪鐢ㄤ竴绉嶄究浜庢柟渚垮湴娣诲姞鎴栨墿灞?鍙傛暟鐨勬牸寮忥紝鏉ユ浛浠ｆ彁渚涚粰 ioctl() 鐨勫浐瀹氭牸寮?C 缁撴瀯浣撱€?
涓烘锛孨etlink 浣跨敤涓€涓渶灏忕殑鍥哄畾鏍煎紡鍏冩暟鎹ご锛屽叾鍚庤窡闅忓涓噰鐢?TLV
锛堢被鍨嬨€侀暱搴︺€佸€硷級鏍煎紡鐨勫睘鎬с€?
閬楁喚鐨勬槸锛岃鍗忚澶氬勾鏉ヤ互鏈夋満涓旀湭鏂囨。鍖栫殑鏂瑰紡婕斿彉锛屼娇寰楀緢闅捐繛璐湴瑙ｉ噴銆?涓轰簡鏈€鍒囧悎瀹為檯锛屾湰鏂囨。棣栧厛鎻忚堪浠婂ぉ鎵€浣跨敤鐨?netlink锛屽苟鍦ㄥ悗闈㈢殑绔犺妭娣卞叆
鎺㈣鏇村叿鈥滃巻鍙测€濈敤閫旂殑鐢ㄦ硶銆?
## 鎵撳紑濂楁帴瀛?

Netlink 閫氫俊閫氳繃濂楁帴瀛楄繘琛岋紝棣栧厛闇€瑕佹墦寮€涓€涓鎺ュ瓧锛?

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

濂楁帴瀛楃殑浣跨敤鎻愪緵浜嗕竴绉嶈嚜鐒剁殑鏂瑰紡鍦ㄥ弻鍚戯紙鍙戝線鍐呮牳涓庢潵鑷唴鏍革級浜ゆ崲淇℃伅銆?褰撳簲鐢ㄧ▼搴?send() 璇锋眰鏃讹紝鎿嶄綔浠嶇劧鏄悓姝ユ墽琛岀殑锛屼絾闇€瑕佸崟鐙殑 recv()
绯荤粺璋冪敤鏉ヨ鍙栧洖澶嶃€?
Netlink 鈥滆皟鐢ㄢ€濈殑涓€涓潪甯哥畝鍖栫殑娴佺▼澶ц嚧濡備笅锛?

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

  /** format the request **/
  send(fd, &request, sizeof(request));
  n = recv(fd, &response, RSP_BUFFER_SIZE);
  /** interpret the response **/

Netlink 杩樺ぉ鐒舵敮鎸佲€渄umping鈥濓紙杞偍锛夛紝鍗冲悜鐢ㄦ埛绌洪棿浼犻€掓煇涓€绫诲瀷鐨勬墍鏈夊璞?锛堜緥濡傝浆鍌ㄦ墍鏈夌殑缃戠粶鎺ュ彛锛夈€?

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

  /** format the dump request **/
  send(fd, &request, sizeof(request));
  while (1) {
    n = recv(fd, &buffer, RSP_BUFFER_SIZE);
    /** one recv() call can read multiple messages, hence the loop below **/
    for (nl_msg in buffer) {
      if (nl_msg.nlmsg_type == NLMSG_DONE)
        goto dump_finished;
      /** process the object **/
    }
  }
  dump_finished:

socket() 璋冪敤鐨勫墠涓や釜鍙傛暟鏃犻渶澶瑙ｉ噴鈥斺€斿畠鎵撳紑涓€涓?Netlink 濂楁帴瀛楋紝鎵€鏈?澶撮儴鐢辩敤鎴锋彁渚涳紙鍥犳鏄?NETLINK銆丷AW锛夈€傛渶鍚庝竴涓弬鏁版槸 Netlink 鍐呴儴鐨勫崗璁€?璇ュ瓧娈电敤浜庢爣璇嗗鎺ュ瓧灏嗕笌涔嬮€氫俊鐨勫瓙绯荤粺銆?
### 缁忓吀 Netlink 涓庨€氱敤 Netlink


Netlink 鐨勬渶鍒濆疄鐜颁緷璧栦簬鍚戝瓙绯荤粺闈欐€佸垎閰?ID锛屽苟鎻愪緵寰堝皯鐨勬敮鎸佸熀纭€璁炬柦銆?鎴戜滑灏嗚繖浜涘崗璁粺绉颁负 **Classic Netlink锛堢粡鍏?Netlink锛?*銆傚畠浠殑鍒楄〃瀹氫箟浜?`include/uapi/linux/netlink.h` 鏂囦欢涔嬩笂锛屽叾涓寘鎷€氱敤缃戠粶
锛圢ETLINK_ROUTE锛夈€乮SCSI锛圢ETLINK_ISCSI锛夊拰瀹¤锛圢ETLINK_AUDIT锛夌瓑銆?
**Generic Netlink锛堥€氱敤 Netlink锛?*锛堜簬 2005 骞村紩鍏ワ級鍏佽鍔ㄦ€佹敞鍐屽瓙绯荤粺
锛堜互鍙婂瓙绯荤粺 ID 鍒嗛厤锛夈€佽嚜鐪侊紝骞剁畝鍖栦簡鎺ュ彛鍐呮牳渚х殑瀹炵幇銆?
涓嬩竴鑺傛弿杩板浣曚娇鐢?Generic Netlink锛屽洜涓轰娇鐢?Generic Netlink 鐨勫瓙绯荤粺鏁伴噺
姣旀棫鍗忚澶氬嚭涓€涓暟閲忕骇銆傚唴鏍镐篃娌℃湁璁″垝娣诲姞鏇村 Classic Netlink 鍗忚銆傚叧浜?濡備綍涓?Linux 鍐呮牳鐨勬牳蹇冪綉缁滈儴鍒嗭紙鎴栦娇鐢?Classic Netlink 鐨勫彟澶?20 涓瓙绯荤粺
涔嬩竴锛夐€氫俊銆佷互鍙婂畠涓?Generic Netlink 鐨勫尯鍒紝鏈枃妗ｅ悗闈細鎻愪緵鍩烘湰淇℃伅銆?
## 閫氱敤 Netlink


闄や簡 Netlink 鍥哄畾鍏冩暟鎹ご涔嬪锛屾瘡涓?Netlink 鍗忚閮藉畾涔変簡鑷繁鐨勫浐瀹氬厓鏁版嵁
澶淬€傦紙绫讳技浜庣綉缁滃ご閮ㄧ殑鍫嗗彔鈥斺€擡thernet > IP > TCP锛屾垜浠湁
Netlink > Generic N. > Family銆傦級

涓€鏉?Netlink 娑堟伅鎬绘槸浠?struct nlmsghdr 寮€濮嬶紝鍏跺悗璺熼殢涓€涓崗璁壒瀹氱殑澶撮儴銆?鍦?Generic Netlink 鐨勬儏鍐典笅锛岃鍗忚澶撮儴鏄?struct genlmsghdr銆?
鍦?Generic Netlink 鐨勬儏鍐典笅锛屽悇瀛楁鐨勫疄闄呭惈涔夊涓嬶細


  struct nlmsghdr {
	__u32	nlmsg_len;	/** Length of message including headers **/
	__u16	nlmsg_type;	/** Generic Netlink Family (subsystem) ID **/
	__u16	nlmsg_flags;	/** Flags - request or dump **/
	__u32	nlmsg_seq;	/** Sequence number **/
	__u32	nlmsg_pid;	/** Port ID, set to 0 **/
  };
  struct genlmsghdr {
	__u8	cmd;		/** Command, as defined by the Family **/
	__u8	version;	/** Irrelevant, set to 1 **/
	__u16	reserved;	/** Reserved, set to 0 **/
  };
  /** TLV attributes follow... **/

鍦?Classic Netlink 涓紝:c`nlmsghdr.nlmsg_type` 鐢ㄤ簬鏍囪瘑娑堟伅鎵€鎸囩殑鏄瓙绯荤粺
鍐呯殑鍝釜鎿嶄綔锛堜緥濡傝幏鍙栧叧浜庢煇涓?netdev 鐨勪俊鎭級銆侴eneric Netlink 闇€瑕佸湪涓€涓?鍗忚閲屽璺鐢ㄥ涓瓙绯荤粺锛屽洜姝ゅ畠鐢ㄨ瀛楁鏉ユ爣璇嗗瓙绯荤粺锛岃€岀敱
:c`genlmsghdr.cmd` 鏉ユ爣璇嗘搷浣溿€傦紙鍏充簬濡備綍鎵惧埌鎰熷叴瓒ｅ瓙绯荤粺鐨?Family ID锛?璇峰弬闃?res_fam銆傦級璇锋敞鎰忥紝鍦?Classic Netlink 鍜?Generic Netlink 涓紝璇ュ瓧娈?鐨勫墠 16 涓€硷紙0 - 15锛夐兘淇濈暀鐢ㄤ簬鎺у埗娑堟伅銆傛洿澶氱粏鑺傝鍙傞槄 nl_msg_type銆?
Netlink 濂楁帴瀛椾笂閫氬父鏈?3 绉嶆秷鎭氦鎹㈢被鍨嬶細

 - 鎵ц鍗曚釜鍔ㄤ綔锛坄do`锛夛紱
 - 杞偍淇℃伅锛坄dump`锛夛紱
 - 鑾峰彇寮傛閫氱煡锛坄multicast`锛夈€?
Classic Netlink 闈炲父鐏垫椿锛屽ぇ姒備篃鍏佽鍏朵粬绫诲瀷鐨勪氦鎹㈠彂鐢燂紝浣嗗湪瀹炶返涓敤鍒扮殑
灏辨槸杩欎笁绫汇€?
寮傛閫氱煡鐢卞唴鏍稿彂閫侊紝鐢辫闃呬簡瀹冧滑鐨勭敤鎴峰鎺ュ瓧鎺ユ敹銆俙do` 鍜?`dump` 璇锋眰鐢?鐢ㄦ埛鍙戣捣銆?c`nlmsghdr.nlmsg_flags` 搴旀寜濡備笅鏂瑰紡璁剧疆锛?
 - 瀵逛簬 `do`锛歚NLM_F_REQUEST | NLM_F_ACK`
 - 瀵逛簬 `dump`锛歚NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP`

:c`nlmsghdr.nlmsg_seq` 搴旇缃负涓€涓崟璋冮€掑鐨勫€笺€傝鍊间細鍦ㄥ搷搴斾腑琚洖鏄撅紝
瀹炶返涓苟涓嶉噸瑕侊紝浣嗕负鍙戦€佺殑姣忔潯娑堟伅璁剧疆涓€涓€掑鐨勫€艰瑙嗕负鑹ソ鐨勪範鎯€傝
瀛楁鐨勭洰鐨勬槸灏嗗搷搴斾笌璇锋眰鍖归厤銆傚紓姝ラ€氱煡鐨?:c`nlmsghdr.nlmsg_seq` 灏嗕负
`0`銆?
:c`nlmsghdr.nlmsg_pid` 鏄?Netlink 涓浉褰撲簬鍦板潃鐨勫瓧娈点€備笌鍐呮牳閫氫俊鏃惰瀛楁
鍙涓?`0`銆傚叧浜庤瀛楁锛堜笉甯歌锛夌殑鐢ㄩ€旓紝璇峰弬闃?nlmsg_pid銆?
:c`genlmsghdr.version` 鐨勯鏈熺敤閫旀槸鍏佽瀵瑰瓙绯荤粺鎻愪緵鐨?API 杩涜鐗堟湰绠＄悊銆?杩勪粖涓烘娌℃湁浠讳綍瀛愮郴缁熷ぇ閲忎娇鐢ㄨ瀛楁锛屽洜姝ゅ皢鍏惰涓?`1` 浼间箮鏄ǔ濡ョ殑閫夋嫨銆?

### Netlink 娑堟伅绫诲瀷


濡傚墠鎵€杩帮紝:c`nlmsghdr.nlmsg_type` 鎼哄甫鍗忚鐗瑰畾鐨勫€硷紝浣嗗墠 16 涓爣璇嗙鏄?淇濈暀鐨勶紙绗竴涓瓙绯荤粺鐗瑰畾鐨勬秷鎭被鍨嬪簲绛変簬 `NLMSG_MIN_TYPE`锛屽嵆 `0x10`锛夈€?
鍙畾涔変簡 4 绉?Netlink 鎺у埗娑堟伅锛?
 - `NLMSG_NOOP` - 蹇界暐璇ユ秷鎭紝瀹炶返涓湭浣跨敤锛? - `NLMSG_ERROR` - 鎼哄甫鎿嶄綔鐨勮繑鍥炵爜锛? - `NLMSG_DONE` - 鏍囪涓€娆?dump 鐨勭粨鏉燂紱
 - `NLMSG_OVERRUN` - 濂楁帴瀛楃紦鍐插尯宸叉孩鍑猴紝鑷充粖鏈娇鐢ㄣ€?
`NLMSG_ERROR` 鍜?`NLMSG_DONE` 鍏锋湁瀹為檯閲嶈鎬с€傚畠浠惡甯︽搷浣滅殑杩斿洖鐮併€傝
娉ㄦ剰锛岄櫎闈炲湪璇锋眰涓婅缃簡 `NLM_F_ACK` 鏍囧織锛屽惁鍒欏鏋滄病鏈夐敊璇紝Netlink 涓嶄細
浠?`NLMSG_ERROR` 鍥炲簲銆備负浜嗛伩鍏嶅繀椤讳负杩欑鎬紓琛屼负鐗规畩澶勭悊锛屽缓璁缁堣缃?`NLM_F_ACK`銆?
```

  ----------------------------------------------
  | struct nlmsghdr - response header          |
  ----------------------------------------------
  |    int error                               |
  ----------------------------------------------
  | struct nlmsghdr - original request header |
  ----------------------------------------------
  | ** optionally (1) payload of the request   |
  ----------------------------------------------
  | ** optionally (2) extended ACK             |
  ----------------------------------------------

```
杩欓噷鏈変袱涓?struct nlmsghdr 瀹炰緥锛岀涓€涓睘浜庡搷搴旓紝绗簩涓睘浜庤姹傘€?`NLMSG_ERROR` 鎼哄甫瀵艰嚧閿欒鐨勮姹傜殑淇℃伅銆傝繖鍦ㄥ皾璇曞皢璇锋眰涓庡搷搴斿尮閰嶏紝鎴栭噸鏂?瑙ｆ瀽璇锋眰浠ヨ浆鍌ㄥ埌鏃ュ織涓椂鍙兘寰堟湁鐢ㄣ€?
璇锋眰鐨勬湁鏁堣浇鑽蜂笉浼氬湪鎶ュ憡鎴愬姛鐨勬秷鎭紙`error == 0`锛変腑鍥炴樉锛屽鏋滆缃簡
`NETLINK_CAP_ACK` setsockopt() 涔熶笉浼氬洖鏄俱€傚悗鑰呭緢甯歌锛屾垨璁镐篃鍊煎緱鎺ㄨ崘锛?鍥犱负涓嶅緱涓嶄粠鍐呮牳璇诲洖姣忎釜璇锋眰鐨勫壇鏈槸鐩稿綋娴垂鐨勩€傝姹傛湁鏁堣浇鑽风殑缂哄け鐢?:c`nlmsghdr.nlmsg_flags` 涓殑 `NLM_F_CAPPED` 鎸囩ず銆?
`NLMSG_ERROR` 鐨勭浜屼釜鍙€夊厓绱犳槸鎵╁睍 ACK 灞炴€с€傛洿澶氱粏鑺傝鍙傞槄 ext_ack銆?鎵╁睍 ACK 鐨勫瓨鍦ㄧ敱 :c`nlmsghdr.nlmsg_flags` 涓殑 `NLM_F_ACK_TLVS` 鎸囩ず銆?
`NLMSG_DONE` 鏇寸畝鍗曪紝璇锋眰姘歌繙涓嶄細琚洖鏄撅紝浣嗘墿灞?```

  ----------------------------------------------
  | struct nlmsghdr - response header          |
  ----------------------------------------------
  |    int error                               |
  ----------------------------------------------
  | ** optionally extended ACK                 |
  ----------------------------------------------

```
璇锋敞鎰忥紝鏌愪簺瀹炵幇鍙兘浼氬彂鍑鸿嚜瀹氫箟鐨?`NLMSG_DONE` 娑堟伅鏉ュ洖搴?`do` 鍔ㄤ綔璇锋眰銆?鍦ㄨ繖绉嶆儏鍐典笅锛屾湁鏁堣浇鑽锋槸瀹炵幇鐗瑰畾鐨勶紝涔熷彲鑳戒笉瀛樺湪銆?

### 瑙ｆ瀽 Family ID


鏈妭璇存槑濡備綍鎵惧埌瀛愮郴缁熺殑 Family ID銆傚畠鍚屾椂涔熶綔涓?Generic Netlink 閫氫俊鐨?涓€涓ず渚嬨€?
Generic Netlink 鏈韩灏辨槸涓€涓€氳繃 Generic Netlink API 鏆撮湶鐨勫瓙绯荤粺銆備负閬垮厤
寰幆渚濊禆锛孏eneric Netlink 鏈変竴涓潤鎬佸垎閰嶇殑 Family ID锛坄GENL_ID_CTRL`锛岀瓑浜?`NLMSG_MIN_TYPE`锛夈€侴eneric Netlink family 瀹炵幇浜嗕竴涓敤浜庢煡璇㈠叾浠?family
淇℃伅鐨勫懡浠わ紙`CTRL_CMD_GETFAMILY`锛夈€?
瑕佽幏鍙栦緥濡傚悕涓?`"test1"` 鐨?Generic Netlink family 鐨勪俊鎭紝鎴戜滑闇€瑕佸湪涔嬪墠
鎵撳紑鐨?Generic Netlink 濂楁帴瀛椾笂鍙戦€佷竴鏉℃秷鎭€傝娑堟伅搴旀寚鍚?Generic Netlink
Family锛?锛夛紝鏄 `CTRL_CMD_GETFAMILY`锛?锛夌殑涓€涓?`do`锛?锛夎皟鐢ㄣ€傛璋冪敤鐨?`dump` 鐗堟湰浼氳鍐呮牳浠ュ叾鎵€鐭ョ殑 **鎵€鏈?* family 鐨勪俊鎭潵鍥炲簲銆傛渶鍚庝絾鍚屾牱閲嶈
鐨勬槸锛岀浉鍏?family 鐨勫悕绉板寘鍚?```

  struct nlmsghdr:
    __u32 nlmsg_len:	32
    __u16 nlmsg_type:	GENL_ID_CTRL               // (1)
    __u16 nlmsg_flags:	NLM_F_REQUEST | NLM_F_ACK  // (2)
    __u32 nlmsg_seq:	1
    __u32 nlmsg_pid:	0

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_GETFAMILY         // (3)
    __u8 version:	2 /* or 1, doesn't matter */
    __u16 reserved:	0

  struct nlattr:                                   // (4)
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    char data:		\0\0

```
Netlink 涓殑闀垮害瀛楁锛?c`nlmsghdr.nlmsg_len` 鍜?:c`nlattr.nla_len`锛夋€绘槸
**鍖呭惈** 澶撮儴銆侼etlink 涓殑灞炴€уご閮ㄥ繀椤讳粠娑堟伅璧峰浣嶇疆瀵归綈鍒?4 瀛楄妭锛屽洜姝ゅ湪
`CTRL_ATTR_FAMILY_NAME` 涔嬪悗鏈夐澶栫殑 `\0\0`銆傚睘鎬ч暱搴?**涓嶅寘鍚?* 濉厖銆?
濡傛灉鎵惧埌浜嗚 family锛屽唴鏍镐細鐢ㄤ袱鏉℃秷鎭洖搴旓紝鍗冲搷搴?```

  /* Message #1 - reply */
  struct nlmsghdr:
    __u32 nlmsg_len:	136
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	0
    __u32 nlmsg_seq:	1    /* echoed from our request */
    __u32 nlmsg_pid:	5831 /* The PID of our user space process */

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_GETFAMILY
    __u8 version:	2
    __u16 reserved:	0

  struct nlattr:
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    data:		\0\0

  struct nlattr:
    __u16 nla_len:	6
    __u16 nla_type:	CTRL_ATTR_FAMILY_ID
    __u16: 		123  /* The Family ID we are after */

  (padding:)
    char data:		\0\0

  struct nlattr:
    __u16 nla_len:	9
    __u16 nla_type:	CTRL_ATTR_FAMILY_VERSION
    __u16: 		1

  /* ... etc, more attributes will follow. */

```
```

  /* Message #2 - the ACK */
  struct nlmsghdr:
    __u32 nlmsg_len:	36
    __u16 nlmsg_type:	NLMSG_ERROR
    __u16 nlmsg_flags:	NLM_F_CAPPED /* There won't be a payload */
    __u32 nlmsg_seq:	1    /* echoed from our request */
    __u32 nlmsg_pid:	5831 /* The PID of our user space process */

  int error:		0

  struct nlmsghdr: /* Copy of the request header as we sent it */
    __u32 nlmsg_len:	32
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	NLM_F_REQUEST | NLM_F_ACK
    __u32 nlmsg_seq:	1
    __u32 nlmsg_pid:	0

```
灞炴€х殑椤哄簭锛坰truct nlattr锛変笉淇濊瘉锛屽洜姝ょ敤鎴峰繀椤婚亶鍘嗗睘鎬у苟瑙ｆ瀽瀹冧滑銆?
璇锋敞鎰忥紝Generic Netlink 濂楁帴瀛楀苟涓嶅叧鑱旀垨缁戝畾鍒板崟涓€ family銆備竴涓鎺ュ瓧鍙敤浜?涓庤澶氫笉鍚岀殑 family 浜ゆ崲娑堟伅锛岄€氳繃 :c`nlmsghdr.nlmsg_type` 瀛楁閫愭潯娑堟伅鍦?閫夋嫨鎺ユ敹鏂?family銆?

### 鎵╁睍 ACK


鎵╁睍 ACK 鎺у埗 `NLMSG_ERROR` 鍜?`NLMSG_DONE` 娑堟伅涓澶栭敊璇?璀﹀憡 TLV 鐨?鎶ュ憡銆備负浜嗕繚鎸佸悜鍚庡吋瀹癸紝姝ゅ姛鑳藉繀椤婚€氳繃鎶?`NETLINK_EXT_ACK` setsockopt()
璁句负 `1` 鏉ユ樉寮忓惎鐢ㄣ€?
鎵╁睍 ack 灞炴€х殑绫诲瀷瀹氫箟浜?enum nlmsgerr_attrs銆傛渶甯哥敤鐨勫睘鎬ф槸
`NLMSGERR_ATTR_MSG`銆乣NLMSGERR_ATTR_OFFS` 鍜?`NLMSGERR_ATTR_MISS_*`銆?
`NLMSGERR_ATTR_MSG` 鎼哄甫涓€鏉℃弿杩版墍閬囬棶棰樼殑鑻辨枃娑堟伅銆傝繖浜涙秷鎭瘮閫氳繃鏍囧噯
UNIX 閿欒鐮佹墍鑳借〃杈剧殑璇︾粏寰楀銆?
`NLMSGERR_ATTR_OFFS` 鎸囧悜瀵艰嚧闂鐨勫睘鎬с€?
`NLMSGERR_ATTR_MISS_TYPE` 鍜?`NLMSGERR_ATTR_MISS_NEST` 鎶ュ憡缂哄け鐨勫睘鎬с€?
鎵╁睍 ACK 鏃㈠彲鍦ㄥ嚭閿欐椂鎶ュ憡锛屼篃鍙湪鎴愬姛鏃舵姤鍛娿€傚悗鑰呭簲琚涓鸿鍛娿€?
鎵╁睍 ACK 鏋佸ぇ鍦版彁鍗囦簡 Netlink 鐨勫彲鐢ㄦ€э紝搴斿綋濮嬬粓鍚敤銆佹伆褰撳湴瑙ｆ瀽骞舵姤鍛婄粰
鐢ㄦ埛銆?
## 楂樼骇涓婚


### Dump 涓€鑷存€?

鍐呮牳鐢ㄤ簬瀛樺偍瀵硅薄鐨勯儴鍒嗘暟鎹粨鏋勶紝浣垮緱闅句互鎻愪緵涓€娆?dump 涓墍鏈夊璞＄殑鍘熷瓙
蹇収锛堝悓鏃朵笉褰卞搷鏇存柊瀹冧滑鐨勫揩閫熻矾寰勶級銆?
濡傛灉 dump 琚腑鏂苟鍙兘涓嶄竴鑷达紙渚嬪缂哄皯瀵硅薄锛夛紝鍐呮牳鍙兘鍦?dump 涓殑浠讳綍
娑堟伅涓婏紙鍖呮嫭 `NLMSG_DONE` 娑堟伅锛夎缃?`NLM_F_DUMP_INTR` 鏍囧織銆傜敤鎴风┖闂村湪
鐪嬪埌璇ユ爣蹇楁椂搴旈噸璇?dump銆?
### 鑷渷


鍩烘湰鑷渷鑳藉姏閫氳繃璁块棶 res_fam 涓姤鍛婄殑 Family 瀵硅薄鏉ュ惎鐢ㄣ€傜敤鎴峰彲浠ユ煡璇㈠叧浜?Generic Netlink family 鐨勪俊鎭紝鍖呮嫭鍐呮牳鏀寔鍝簺鎿嶄綔銆佸唴鏍哥悊瑙ｅ摢浜涘睘鎬с€?Family 淇℃伅鍖呭惈鍐呮牳鍙В鏋愮殑灞炴€х殑鏈€楂?ID锛屼竴涓崟鐙殑鍛戒护
锛坄CTRL_CMD_GETPOLICY`锛夋彁渚涘叧浜庡彈鏀寔灞炴€х殑璇︾粏淇℃伅锛屽寘鎷唴鏍告帴鍙楃殑鍊?鑼冨洿銆?
褰撶敤鎴风┖闂撮渶瑕佸湪鍙戝嚭璇锋眰涔嬪墠纭鍐呮牳鏄惁鏀寔鏌愪釜鍔熻兘鏃讹紝鏌ヨ family 淇℃伅
寰堟湁鐢ㄣ€?

### nlmsg_pid


:c`nlmsghdr.nlmsg_pid` 鏄?Netlink 涓浉褰撲簬鍦板潃鐨勫瓧娈点€傚畠琚О涓?Port ID锛?鏈夋椂涔熷彨 Process ID锛屽洜涓哄嚭浜庡巻鍙插師鍥狅紝濡傛灉搴旂敤绋嬪簭鏈€夋嫨锛坆ind() 鍒帮級涓€涓?鏄惧紡鐨?Port ID锛屽唴鏍镐細鑷姩灏嗗叾鍒嗛厤涓虹瓑浜庡叾 Process ID 鐨?ID锛堢敱 getpid()
绯荤粺璋冪敤鎶ュ憡锛夈€?
涓?TCP/IP 缃戠粶鍗忚鐨?bind() 璇箟绫讳技锛岄浂鍊艰〃绀衡€滆嚜鍔ㄥ垎閰嶁€濓紝鍥犳搴旂敤绋嬪簭
閫氬父浼氬皢 :c`nlmsghdr.nlmsg_pid` 瀛楁鍒濆鍖栦负 `0`銆?
璇ュ瓧娈典粖澶╁湪缃曡鎯呭喌涓嬩粛鍦ㄤ娇鐢紝鍗冲唴鏍搁渶瑕佸彂閫佸崟鎾€氱煡鏃躲€傜敤鎴风┖闂村簲鐢?绋嬪簭鍙互浣跨敤 bind() 灏嗗叾濂楁帴瀛椾笌鐗瑰畾鐨?PID 鍏宠仈锛岀劧鍚庡皢瀹冪殑 PID 鍛婄煡鍐呮牳銆?杩欐牱鍐呮牳灏辫兘鑱旂郴鍒扮壒瀹氱殑鐢ㄦ埛绌洪棿杩涚▼銆?
杩欑被閫氫俊鐢ㄤ簬绫讳技 UMH锛圲ser Mode Helper锛夌殑鍦烘櫙锛屽嵆鍐呮牳闇€瑕佽Е鍙戠敤鎴风┖闂?澶勭悊鎴栧悜鐢ㄦ埛绌洪棿璇㈤棶绛栫暐鍐崇瓥鏃躲€?
### 缁勬挱閫氱煡


Netlink 鐨勪紭鍔夸箣涓€鏄兘澶熷悜鐢ㄦ埛绌洪棿鍙戦€佷簨浠堕€氱煡銆傝繖鏄竴绉嶅崟鍚戦€氫俊褰㈠紡
锛堝唴鏍?-> 鐢ㄦ埛锛夛紝涓嶆秹鍙婁换浣曞儚 `NLMSG_ERROR` 鎴?`NLMSG_DONE` 杩欐牱鐨勬帶鍒?娑堟伅銆?
渚嬪锛孏eneric Netlink family 鑷韩灏卞畾涔変簡涓€缁勫叧浜庡凡娉ㄥ唽 family 鐨勭粍鎾?閫氱煡銆傚綋娣诲姞涓€涓柊鐨?family 鏃讹紝
```

  struct nlmsghdr:
    __u32 nlmsg_len:	136
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	0
    __u32 nlmsg_seq:	0
    __u32 nlmsg_pid:	0

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_NEWFAMILY
    __u8 version:	2
    __u16 reserved:	0

  struct nlattr:
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    data:		\0\0

  struct nlattr:
    __u16 nla_len:	6
    __u16 nla_type:	CTRL_ATTR_FAMILY_ID
    __u16: 		123  /* The Family ID we are after */

  (padding:)
    char data:		\0\0

  struct nlattr:
    __u16 nla_len:	9
    __u16 nla_type:	CTRL_ATTR_FAMILY_VERSION
    __u16: 		1

  /* ... etc, more attributes will follow. */

```
璇ラ€氱煡鍖呭惈涓庡 `CTRL_CMD_GETFAMILY` 璇锋眰鐨勫搷搴旂浉鍚岀殑淇℃伅銆?
閫氱煡鐨?Netlink 澶撮儴澶у涓?0 涓旀棤鍏崇揣瑕併€?c`nlmsghdr.nlmsg_seq` 鍙互鏄浂锛?涔熷彲浠ユ槸璇?family 缁存姢鐨勫崟璋冮€掑鐨勯€氱煡搴忓垪鍙枫€?
瑕佹帴鏀堕€氱煡锛岀敤鎴峰鎺ュ瓧蹇呴』璁㈤槄鐩稿叧鐨勯€氱煡缁勩€備笌 Family ID 闈炲父鐩镐技锛岀粰瀹?缁勬挱缁勭殑 Group ID 鏄姩鎬佺殑锛屽彲浠ュ湪 Family 淇℃伅涓壘鍒般€俙CTRL_ATTR_MCAST_GROUPS`
灞炴€у寘鍚祵濂楋紝鍏朵腑鏈夊悇缁勭殑鍚嶇О锛坄CTRL_ATTR_MCAST_GRP_NAME`锛夊拰 ID
锛坄CTRL_ATTR_MCAST_GRP_ID`锛夈€?
涓€鏃︾煡閬撲簡 Group ID锛屼竴涓?setsockopt() 璋冪敤灏变細灏嗚濂楁帴瀛楀姞鍏ヨ缁勶細


  unsigned int group_id;

  /** .. find the group ID... **/

  setsockopt(fd, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP,
             &group_id, sizeof(group_id));

璇ュ鎺ュ瓧鐜板湪灏嗘帴鏀堕€氱煡銆?
寤鸿涓烘帴鏀堕€氱煡鍜屽悜鍐呮牳鍙戦€佽姹備娇鐢ㄥ崟鐙殑濂楁帴瀛椼€傞€氱煡鐨勫紓姝ョ壒鎬ф剰鍛崇潃瀹冧滑
鍙兘浼氫笌鍝嶅簲娣峰湪涓€璧凤紝浣垮緱娑堟伅澶勭悊鍥伴毦寰楀銆?
### 缂撳啿鍖哄ぇ灏?

Netlink 濂楁帴瀛楁槸鏁版嵁鎶ュ鎺ュ瓧鑰岄潪娴佸鎺ュ瓧锛岃繖鎰忓懗鐫€姣忔潯娑堟伅閮藉繀椤荤敱鍗曟
recv()/recvmsg() 绯荤粺璋冪敤瀹屾暣鍦版帴鏀躲€傚鏋滅敤鎴锋彁渚涚殑缂撳啿鍖哄お鐭紝娑堟伅灏嗚
鎴柇锛屽苟鍦?struct msghdr 涓缃?`MSG_TRUNC` 鏍囧織锛坰truct msghdr 鏄?recvmsg()
绯荤粺璋冪敤鐨勭浜屼釜鍙傛暟锛?*涓嶆槸** Netlink 澶撮儴锛夈€?
鎴柇鍚庯紝娑堟伅鐨勫墿浣欓儴鍒嗗皢琚涪寮冦€?
Netlink 鏈熸湜鐢ㄦ埛缂撳啿鍖鸿嚦灏戜负 8kB锛屾垨 CPU 鏋舵瀯鐨勯〉澶у皬锛屽彇涓よ€呬腑杈冨ぇ鑰呫€?鐒惰€岋紝鐗瑰畾鐨?Netlink family 鍙兘瑕佹眰鏇村ぇ鐨勭紦鍐插尯銆備负鏈€楂樻晥鍦板鐞?dump锛?鎺ㄨ崘浣跨敤 32kB 缂撳啿鍖猴紙鏇村ぇ鐨勭紦鍐插尯鍙绾虫洿澶氳 dump 鐨勫璞★紝鍥犳闇€瑕佺殑
recvmsg() 璋冪敤鏇村皯锛夈€?

## 缁忓吀 Netlink


Classic 涓?Generic Netlink 鐨勪富瑕佸尯鍒湪浜庡瓙绯荤粺鏍囪瘑绗︾殑鍔ㄦ€佸垎閰嶄互鍙婅嚜鐪佺殑
鍙敤鎬с€傜悊璁轰笂璇ュ崗璁病鏈夋樉钁楀樊寮傦紝鐒惰€屽湪瀹炶返涓紝Classic Netlink 璇曢獙浜嗕竴
浜涘湪 Generic Netlink 涓搴熷純鐨勬蹇碉紙瀹為檯涓婏紝瀹冧滑閫氬父鍙湪鍗曚釜瀛愮郴缁熺殑涓€涓?灏忚钀介噷浣跨敤杩囷級銆傛湰鑺傛棬鍦ㄨВ閲婂叾涓嚑涓蹇碉紝鏄庣‘鐩爣鏄 Generic Netlink
鐢ㄦ埛鍦ㄩ槄璇?uAPI 澶撮儴鏃惰兘鏈変俊蹇冨拷鐣ュ畠浠€?
杩欓噷鐨勫ぇ澶氭暟姒傚康鍜岀ず渚嬮兘娑夊強 `NETLINK_ROUTE` family锛屽畠娑电洊浜?Linux 缃戠粶
鏍堢殑澶ч儴鍒嗛厤缃€傚璇?family 鐨勭湡姝ｆ枃妗ｅ€煎緱鍗曠嫭鍐欎竴绔狅紙鎴栦竴鏈功锛夈€?
### Families


Netlink 灏嗗瓙绯荤粺绉颁负 families銆傝繖鏄娇鐢ㄥ鎺ュ瓧鍜屽崗璁棌姒傚康鐨勯仐鐣欎骇鐗╋紝鑰?鍗忚鏃忔槸 `NETLINK_ROUTE` 涓秷鎭В澶嶇敤鐨勭粍鎴愰儴鍒嗐€?
閬楁喚鐨勬槸锛屾瘡涓€灞傚皝瑁呴兘鍠滄鎶婂畠鎵€鎵胯浇鐨勪笢瑗跨О涓衡€渇amilies鈥濓紝浣垮緱杩欎釜鏈
闈炲父浠や汉鍥版儜锛?
 1. AF_NETLINK 鏄竴涓悕鍓叾瀹炵殑濂楁帴瀛楀崗璁棌
 2. AF_NETLINK 鐨勬枃妗ｅ皢娑堟伅涓畠鑷韩澶撮儴锛坰truct nlmsghdr锛変箣鍚庣殑鍐呭绉颁负
    鈥淔amily Header鈥? 3. Generic Netlink 鏄?AF_NETLINK 鐨勪竴涓?family锛坰truct genlmsghdr 璺熼殢
    struct nlmsghdr锛夛紝浣嗗畠涔熺О鍏剁敤鎴蜂负鈥淔amilies鈥濄€?
璇锋敞鎰忥紝Generic Netlink 鐨?Family ID 澶勪簬涓嶅悓鐨勨€淚D 绌洪棿鈥濅腑锛屽苟涓斾笌 Classic
Netlink 鍗忚鍙烽噸鍙狅紙渚嬪 `NETLINK_CRYPTO` 鐨?Classic Netlink 鍗忚 ID 涓?21锛?鑰?Generic Netlink 涔熶細寰堜箰鎰忓皢鍏跺垎閰嶇粰瀹冪殑鏌愪釜 family锛夈€?
### 涓ユ牸妫€鏌?

`NETLINK_GET_STRICT_CHK` 濂楁帴瀛楅€夐」鍦?`NETLINK_ROUTE` 涓惎鐢ㄤ弗鏍肩殑杈撳叆
妫€鏌ャ€備箣鎵€浠ラ渶瑕佸畠锛屾槸鍥犱负鍘嗗彶涓婂唴鏍镐笉浼氶獙璇佸畠鏈鐞嗙殑缁撴瀯浣撶殑瀛楁銆傝繖浣垮緱
鍚庢潵涓嶅彲鑳藉紑濮嬩娇鐢ㄩ偅浜涘瓧娈碉紝鑰屼笉鍐掗偅浜涢敊璇湴鎴栨湭鍒濆鍖栧畠浠殑搴旂敤绋嬪簭鍑虹幇
鍥炲綊鐨勯闄┿€?
`NETLINK_GET_STRICT_CHK` 澹版槑搴旂敤绋嬪簭姝ｅ湪姝ｇ‘鍒濆鍖栨墍鏈夊瓧娈点€傚畠杩橀€夋嫨楠岃瘉
娑堟伅涓嶅寘鍚熬闅忔暟鎹紝骞惰姹傚唴鏍告嫆缁濈被鍨嬮珮浜庡唴鏍稿凡鐭ユ渶澶у睘鎬х被鍨嬬殑灞炴€с€?
`NETLINK_GET_STRICT_CHK` 涓嶅湪 `NETLINK_ROUTE` 涔嬪浣跨敤銆?
### 鏈煡灞炴€?

鍘嗗彶涓?Netlink 蹇界暐浜嗘墍鏈夋湭鐭ュ睘鎬с€傚叾鎯虫硶鏄搴旂敤绋嬪簭涓嶅繀鍘绘帰鏌ュ唴鏍告敮鎸?浠€涔堛€傚簲鐢ㄧ▼搴忓彲浠ュ彂鍑烘敼鍙樼姸鎬佺殑璇锋眰锛屽苟妫€鏌ヨ姹傜殑鍝簺閮ㄥ垎鈥滅敓鏁堚€濅簡銆?
瀵逛簬鏂扮殑 Generic Netlink family 浠ュ強閫夋嫨涓ユ牸妫€鏌ョ殑閭ｄ簺锛屾儏鍐靛凡涓嶅啀濡傛銆?鎵€鎵ц鐨勯獙璇佺被鍨嬭鍙傞槄 enum netlink_validation銆?
### 鍥哄畾鍏冩暟鎹笌缁撴瀯浣?

Classic Netlink 鍦ㄦ秷鎭腑澶ч噺浣跨敤鍥哄畾鏍煎紡鐨勭粨鏋勪綋銆傛秷鎭€氬父鍦?struct
nlmsghdr 涔嬪悗甯︽湁涓€涓叿鏈夊ぇ閲忓瓧娈电殑缁撴瀯浣撱€傛妸鍏锋湁澶氫釜鎴愬憳鐨勭粨鏋勪綋鏀惧叆
灞炴€т腑銆佽€屼笉鎶婃瘡涓垚鍛樻媶鎴愬悇鑷殑灞炴€э紝涔熸槸寰堝父瑙佺殑鍋氭硶銆?
杩欑粰楠岃瘉鍜屽彲鎵╁睍鎬у甫鏉ヤ簡闂锛屽洜姝ゅ浜庢柊灞炴€э紝涓嶉紦鍔变娇鐢ㄤ簩杩涘埗缁撴瀯浣撱€?
### 璇锋眰绫诲瀷


`NETLINK_ROUTE` 灏嗚姹傚垎涓?4 绉嶇被鍨嬶細`NEW`銆乣DEL`銆乣GET` 鍜?`SET`銆傛瘡涓?瀵硅薄鍙互澶勭悊鎵€鏈夎繖浜涙垨鍏朵腑閮ㄥ垎璇锋眰锛堝璞″嵆 netdev銆佽矾鐢便€佸湴鍧€銆乹disc 绛夛級銆?璇锋眰绫诲瀷鐢辨秷鎭被鍨嬬殑鏈€浣?2 浣嶅畾涔夛紝鍥犳鏂板璞＄殑鍛戒护鎬绘槸浠?4 涓烘闀垮垎閰嶃€?
姣忎釜瀵硅薄杩樹細鎷ユ湁鑷繁鐨勩€佺敱鎵€鏈夎姹傜被鍨嬪叡浜殑鍥哄畾鍏冩暟鎹紙渚嬪 netdev 璇锋眰
浣跨敤 struct ifinfomsg锛屽湴鍧€璇锋眰浣跨敤 struct ifaddrmsg锛宷disc 璇锋眰浣跨敤
struct tcmsg锛夈€?
灏界鍏朵粬鍗忚鍜?Generic Netlink 鍛戒护缁忓父鍦ㄥ畠浠殑娑堟伅鍚嶄腑浣跨敤鐩稿悓鐨勫姩璇?锛坄GET`銆乣SET`锛夛紝浣嗚姹傜被鍨嬬殑姒傚康骞舵湭寰楀埌鏇村箍娉涚殑閲囩敤銆?
### 閫氱煡鍥炴樉


`NLM_F_ECHO` 璇锋眰灏嗙敱璇ヨ姹備骇鐢熺殑閫氱煡鎺掗槦鍒板彂璧疯姹傜殑濂楁帴瀛椾笂銆傝繖鏈夊姪浜?鍙戠幇璇ヨ姹傜殑褰卞搷銆?
璇锋敞鎰忥紝姝ゅ姛鑳藉苟鏈鏅亶瀹炵幇銆?
### 鍏朵粬璇锋眰绫诲瀷鐗瑰畾鐨勬爣蹇?

Classic Netlink 鍦?struct nlmsghdr 鐨?nlmsg_flags 鐨勯珮瀛楄妭涓负瀹冪殑 `GET`銆?`NEW` 鍜?`DEL` 璇锋眰瀹氫箟浜嗗悇绉嶆爣蹇椼€傜敱浜庤姹傜被鍨嬪皻鏈€氱敤鍖栵紝杩欎簺璇锋眰绫诲瀷
鐗瑰畾鐨勬爣蹇楀緢灏戜娇鐢紙骞朵笖瀵逛簬鏂扮殑 family 琚涓哄凡寮冪敤锛夈€?
瀵逛簬 `GET` - `NLM_F_ROOT` 鍜?`NLM_F_MATCH` 琚悎骞朵负 `NLM_F_DUMP`锛屼笉鍗曠嫭
浣跨敤銆俙NLM_F_ATOMIC` 浠庢湭浣跨敤銆?
瀵逛簬 `DEL` - `NLM_F_NONREC` 浠呰 nftables 浣跨敤锛宍NLM_F_BULK` 浠呰 FDB 鐨?閮ㄥ垎鎿嶄綔浣跨敤銆?
鐢ㄤ簬 `NEW` 鐨勬爣蹇楀湪 classic Netlink 涓渶甯哥敤銆傞仐鎲剧殑鏄紝鍏跺惈涔夊苟涓嶅崄鍒嗘竻鏅般€?浠ヤ笅鎻忚堪鍩轰簬浣滆€呭鎰忓浘鐨勬渶浣崇寽娴嬶紝鑰屽湪瀹炶返涓墍鏈?family 閮戒細浠ユ煇绉嶆柟寮忓亸绂?瀹冦€俙NLM_F_REPLACE` 瑕佹眰鏇挎崲涓€涓凡瀛樺湪鐨勫璞★紝濡傛灉涓嶅瓨鍦ㄥ尮閰嶇殑瀵硅薄锛屾搷浣?搴斿綋澶辫触銆俙NLM_F_EXCL` 鍏锋湁鐩稿弽鐨勮涔夛紝浠呭綋瀵硅薄宸茬粡瀛樺湪鏃舵墠鎴愬姛銆?`NLM_F_CREATE` 瑕佹眰濡傛灉瀵硅薄涓嶅瓨鍦ㄥ氨鍒涘缓瀹冿紝瀹冨彲涓?`NLM_F_REPLACE` 鍜?`NLM_F_EXCL` 缁勫悎銆?
```

   4.4BSD ADD		NLM_F_CREATE|NLM_F_EXCL
   4.4BSD CHANGE	NLM_F_REPLACE

   True CHANGE		NLM_F_CREATE|NLM_F_REPLACE
   Append		NLM_F_CREATE
   Check		NLM_F_EXCL

```
杩欎技涔庤〃鏄庤繖浜涙爣蹇楁棭浜庤姹傜被鍨嬨€俙NLM_F_REPLACE` 鍦ㄦ病鏈?`NLM_F_CREATE` 鏃?鏈€鍒濊鐢ㄦ潵浠ｆ浛 `SET` 鍛戒护銆俙NLM_F_EXCL` 鍦ㄦ病鏈?`NLM_F_CREATE` 鏃剁敤浜庢鏌?瀵硅薄鏄惁瀛樺湪鑰屼笉鍒涘缓瀹冿紝澶ф鏃╀簬 `GET` 鍛戒护銆?
`NLM_F_APPEND` 琛ㄧず濡傛灉涓€涓敭鍙互鍏宠仈澶氫釜瀵硅薄锛堜緥濡備竴鏉¤矾鐢辩殑澶氫釜涓嬩竴璺?瀵硅薄锛夛紝鏂板璞″簲褰撹娣诲姞鍒板垪琛ㄤ腑锛岃€屼笉鏄浛鎹㈡暣涓垪琛ㄣ€?
## uAPI 鍙傝€?