
## Kernel Connector


鍐呮牳杩炴帴鍣紙Kernel connector锛夆€斺€斾竴绉嶅熀浜?netlink 鐨勩€佺敤鎴风┖闂?<-> 鍐呮牳绌洪棿涔嬮棿鏄撲簬浣跨敤鐨勯€氫俊妯″潡銆?
杩炴帴鍣ㄩ┍鍔ㄤ娇寰椾娇鐢ㄥ熀浜?netlink 鐨勭綉缁滆繛鎺ュ悇绉嶄唬鐞嗗彉寰楀鏄撱€備娇鐢ㄨ€呭繀椤绘敞鍐屼竴涓洖璋冨拰涓€涓爣璇嗙銆傚綋椹卞姩鏀跺埌甯︽湁鐩稿簲鏍囪瘑绗︾殑鐗规畩 netlink 娑堟伅鏃讹紝灏变細璋冪敤鐩稿簲鐨勫洖璋冦€?
浠庣敤鎴风┖闂寸殑瑙掑害鏉ョ湅锛岃繖鐩稿綋鐩存帴锛?
 - socket();
 - bind();
 - send();
 - recv();

浣嗗鏋滃唴鏍哥┖闂存兂瑕佸厖鍒嗗埄鐢ㄨ繖绉嶈繛鎺ョ殑濞佸姏锛岄┍鍔ㄧ紪鍐欒€呭繀椤诲垱寤虹壒娈婄殑濂楁帴瀛楋紝蹇呴』浜嗚В struct sk_buff 鐨勫鐞嗙瓑绛夆€︹€﹁繛鎺ュ櫒椹卞姩鍏佽浠讳綍鍐呮牳绌洪棿浠ｇ悊浠ユ樉钁楃畝鍖栫殑鏂瑰紡浣跨敤鍩轰簬 netlink 鐨勭綉缁滆繘琛岃繘绋嬮棿閫氫俊锛?
```

  int cn_add_callback(const struct cb_id *id, char *name, void (*callback) (struct cn_msg *, struct netlink_skb_parms *));
  void cn_netlink_send_mult(struct cn_msg *msg, u16 len, u32 portid, u32 __group, int gfp_mask);
  void cn_netlink_send(struct cn_msg *msg, u32 portid, u32 __group, int gfp_mask);

  struct cb_id
  {
	__u32			idx;
	__u32			val;
  };

```
idx 鍜?val 鏄敮涓€鏍囪瘑绗︼紝蹇呴』鍦?connector.h 澶存枃浠朵腑娉ㄥ唽浠ヤ緵鍐呮牳鍐呴儴浣跨敤銆俙void (**callback) (void **)` 鏄竴涓洖璋冨嚱鏁帮紝褰撹繛鎺ュ櫒鏍稿績鏀跺埌甯︽湁涓婅堪 idx.val 鐨勬秷鎭椂浼氳璋冪敤銆傝鍑芥暟鐨勫弬鏁板繀椤绘槸锛?
```

  struct cn_msg
  {
	struct cb_id		id;

	__u32			seq;
	__u32			ack;

	__u16			len;	/* 鍚庣画鏁版嵁鐨勯暱搴?*/
	__u16			flags;
	__u8			data[0];
  };

```
## Connector interfaces


 .. kernel-doc:: include/linux/connector.h

 娉ㄦ剰锛?   鍦ㄦ敞鍐屾柊鐨勫洖璋冪敤鎴锋椂锛岃繛鎺ュ櫒鏍稿績浼氬垎閰嶇粰璇ョ敤鎴蜂竴涓?netlink 缁勶紝鍏跺€肩瓑浜庡畠鐨?id.idx銆?
## Protocol description


褰撳墠妗嗘灦鎻愪緵浜嗕竴涓甫鏈夊浐瀹氬ご鐨勪紶杈撳眰銆備娇鐢ㄨ澶撮儴鐨勬帹鑽愬崗璁涓嬶細

msg->seq 鍜?msg->ack 鐢ㄤ簬纭畾娑堟伅鐨勮氨绯汇€傚綋鏌愭柟鍙戦€佷竴鏉℃秷鎭椂锛屽畠浼氫娇鐢ㄤ竴涓湰鍦板敮涓€鐨勫簭鍒楀彿鍜岄殢鏈虹殑纭鍙枫€傝搴忓垪鍙蜂篃鍙互澶嶅埗鍒?nlmsghdr->nlmsg_seq 涓€?
搴忓垪鍙烽殢姣忔潯鍙戦€佺殑娑堟伅閫掑銆?
濡傛灉浣犳湡鏈涙敹鍒板璇ユ秷鎭殑鍥炲锛岄偅涔堟帴鏀跺埌鐨勬秷鎭腑鐨勫簭鍒楀彿蹇呴』涓庡師娑堟伅鐩稿悓锛屼笖纭鍙峰繀椤绘槸鍘熷簭鍒楀彿 + 1銆?
濡傛灉鎴戜滑鏀跺埌涓€鏉℃秷鎭紝鍏跺簭鍒楀彿涓庢垜浠湡鏈涚殑涓嶇浉绛夛紝閭ｄ箞瀹冨氨鏄竴鏉℃柊娑堟伅銆傚鏋滄垜浠敹鍒颁竴鏉℃秷鎭紝鍏跺簭鍒楀彿涓庢垜浠湡鏈涚殑鐩稿悓锛屼絾纭鍙蜂笉绛変簬鍘熸秷鎭腑鐨勫簭鍒楀彿 + 1锛岄偅涔堝畠涔熸槸涓€鏉℃柊娑堟伅銆?
鏄剧劧锛屽崗璁ご閮ㄥ寘鍚簡涓婅堪鐨?id銆?
杩炴帴鍣ㄥ厑璁镐互濡備笅褰㈠紡杩涜浜嬩欢閫氱煡锛氬唴鏍搁┍鍔ㄦ垨鐢ㄦ埛绌洪棿杩涚▼鍙互璇锋眰杩炴帴鍣ㄥ湪閫夊畾鐨?id 琚墦寮€鎴栧叧闂紙娉ㄥ唽鎴栨敞閿€鍏跺洖璋冿級鏃堕€氱煡瀹冦€傝繖鏄€氳繃鍚戣繛鎺ュ櫒椹卞姩鍙戦€佷竴鏉＄壒娈婂懡浠ゆ潵瀹屾垚鐨勶紙瀹冭嚜韬篃浠?id={-1, -1} 娉ㄥ唽锛夈€?
鍏充簬杩欑鐢ㄦ硶鐨勭ず渚嬪彲浠ュ湪 cn_test.c 妯″潡涓壘鍒帮紝璇ユā鍧椾娇鐢ㄨ繛鎺ュ櫒鏉ヨ姹傞€氱煡骞跺彂閫佹秷鎭€?
## Reliability


Netlink 鏈韩骞朵笉鏄竴涓彲闈犵殑鍗忚銆傝繖鎰忓懗鐫€娑堟伅鍙兘浼氱敱浜庡唴瀛樺帇鍔涙垨杩涚▼鐨勬帴鏀堕槦鍒楁孩鍑鸿€屼涪澶憋紝鍥犳璋冪敤鑰呰璀﹀憡蹇呴』鏈夋墍鍑嗗銆傝繖灏辨槸涓轰粈涔?struct cn_msg锛堣繛鎺ュ櫒鐨勪富瑕佹秷鎭ご锛夊寘鍚?u32 seq 鍜?u32 ack 瀛楁銆?
## Userspace usage


2.6.14 寮曞叆浜嗕竴绉嶆柊鐨?netlink 濂楁帴瀛楀疄鐜帮紝榛樿鎯呭喌涓嬩笉鍏佽鍚戦櫎 1 浠ュ鐨?netlink 缁勫彂閫佹暟鎹€?鍥犳锛屽鏋滀綘甯屾湜浣跨敤鍏锋湁涓嶅悓缁勫彿鐨?netlink 濂楁帴瀛楋紙渚嬪浣跨敤杩炴帴鍣級锛岀敤鎴风┖闂村簲鐢ㄧ▼搴忓繀椤昏闃咃細

```

  s = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);

  l_local.nl_family = AF_NETLINK;
  l_local.nl_groups = 12345;
  l_local.nl_pid = 0;

  if (bind(s, (struct sockaddr *)&l_local, sizeof(struct sockaddr_nl)) == -1) {
	perror("bind");
	close(s);
	return -1;
  }

  {
	int on = l_local.nl_groups;
	setsockopt(s, 270, 1, &on, sizeof(on));
  }

```
鍏朵腑涓婇潰鐨?270 鏄?SOL_NETLINK锛? 鏄?NETLINK_ADD_MEMBERSHIP 濂楁帴瀛楅€夐」銆傝鍙栨秷澶氭挱璁㈤槄锛屽簲浣跨敤瀹氫箟涓?0 鐨?NETLINK_DROP_MEMBERSHIP 鍙傛暟璋冪敤涓婅堪濂楁帴瀛楅€夐」銆?
2.6.14 鐨?netlink 浠ｇ爜鍙厑璁搁€夋嫨灏忎簬鎴栫瓑浜庢渶澶х粍鍙风殑缁勶紝璇ユ渶澶х粍鍙峰湪 netlink_kernel_create() 鏃朵娇鐢ㄣ€傚浜庤繛鎺ュ櫒鑰岃█锛屽畠鏄?CN_NETLINK_USERS + 0xf锛屽洜姝ゅ鏋滀綘鎯充娇鐢ㄧ粍鍙?12345锛屽繀椤诲皢 CN_NETLINK_USERS 澧炲姞鍒拌鏁板€笺€傞澶栫殑 0xf 涓紪鍙峰垎閰嶇粰鍐呮牳澶栭儴鐨勭敤鎴蜂娇鐢ㄣ€?
鐢变簬杩欎竴闄愬埗锛岀粍 0xffffffff 鐩墠鏃犳硶宸ヤ綔锛屽洜姝や笉鑳戒娇鐢ㄦ坊鍔?鍒犻櫎杩炴帴鍣ㄧ殑缁勯€氱煡锛屼絾鎹垜鎵€鐭ワ紝鍙湁 cn_test.c 娴嬭瘯妯″潡浣跨敤杩囧畠銆?
netlink 棰嗗煙鐨勪竴浜涘伐浣滀粛鍦ㄨ繘琛屼腑锛屽洜姝ゅ湪 2.6.15 鏈熼棿鍙兘浼氭湁鍙樺姩锛屽鏋滃彂鐢燂紝灏嗕細鏇存柊瀵瑰簲鍐呮牳鐨勬枃妗ｃ€?
## Code samples


杩炴帴鍣ㄦ祴璇曟ā鍧楀拰鐢ㄦ埛绌洪棿鐨勭ず渚嬩唬鐮佸彲浠ュ湪 samples/connector/ 涓壘鍒般€傝鏋勫缓杩欎簺浠ｇ爜锛岃鍚敤 CONFIG_CONNECTOR 鍜?CONFIG_SAMPLES銆?