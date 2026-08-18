## 鍩轰簬 connector 鐨勭敤鎴风┖闂撮€氫俊鍗忚


## 娑堟伅绫诲瀷


w1 鏍稿績涓庣敤鎴风┖闂翠箣闂存湁涓夌绫诲瀷鐨勬秷鎭細

1. 浜嬩欢銆傛瘡褰撻€氳繃鑷姩鎴栬姹傚紡鎼滅储鍙戠幇涓€涓柊鐨勪富璁惧鎴栦粠璁惧鏃剁敓鎴愩€?2. 鐢ㄦ埛绌洪棿鍛戒护銆?3. 瀵圭敤鎴风┖闂村懡浠ょ殑鍥炲銆?

## 鍗忚


```

  [struct cn_msg] - connector 澶撮儴銆?	鍏?length 瀛楁绛変簬闄勫甫鏁版嵁鐨勫ぇ灏?  [struct w1_netlink_msg] - w1 netlink 澶撮儴銆?	__u8 type 	- 娑堟伅绫诲瀷銆?			W1_LIST_MASTERS
				鍒楀嚭褰撳墠鐨勬€荤嚎涓昏澶?			W1_SLAVE_ADD/W1_SLAVE_REMOVE
				浠庤澶囨坊鍔?绉婚櫎浜嬩欢
			W1_MASTER_ADD/W1_MASTER_REMOVE
				涓昏澶囨坊鍔?绉婚櫎浜嬩欢
			W1_MASTER_CMD
				闈㈠悜鎬荤嚎涓昏澶囩殑鐢ㄦ埛绌洪棿鍛戒护
				锛堟悳绱?鎶ヨ鎼滅储锛?			W1_SLAVE_CMD
				闈㈠悜浠庤澶囩殑鐢ㄦ埛绌洪棿鍛戒护
				锛堣/鍐?瑙︽懜锛?	__u8 status	- 鏉ヨ嚜鍐呮牳鐨勯敊璇寚绀?	__u16 len	- 闄勫姞鍒版澶撮儴鏁版嵁鐨勬暟鎹ぇ灏?	union {
		__u8 id[8];			 - 浠庤澶囧敮涓€ id
		struct w1_mst {
			__u32		id;	 - 涓昏澶?id
			__u32		res;	 - 淇濈暀
		} mst;
	} id;

  [struct w1_netlink_cmd] - 缁欏畾涓昏澶囨垨浠庤澶囩殑鍛戒护銆?	__u8 cmd	- 鍛戒护鎿嶄綔鐮併€?			W1_CMD_READ 	- 璇诲懡浠?			W1_CMD_WRITE	- 鍐欏懡浠?			W1_CMD_SEARCH	- 鎼滅储鍛戒护
			W1_CMD_ALARM_SEARCH - 鎶ヨ鎼滅储鍛戒护
			W1_CMD_TOUCH	- 瑙︽懜鍛戒护
				锛堝啓鏁版嵁骞跺皢閲囨牱缁撴灉杩斿洖鐢ㄦ埛绌洪棿锛?			W1_CMD_RESET	- 鍙戦€佹€荤嚎澶嶄綅
			W1_CMD_SLAVE_ADD	- 灏?slave 娣诲姞鍒板唴鏍稿垪琛?			W1_CMD_SLAVE_REMOVE	- 浠庡唴鏍稿垪琛ㄧЩ闄?slave
			W1_CMD_LIST_SLAVES	- 浠庡唴鏍歌幏鍙?slave 鍒楄〃
	__u8 res	- 淇濈暀
	__u16 len	- 姝ゅ懡浠ょ殑鏁版嵁闀垮害
		For read command data must be allocated like for write command
	__u8 data[0]	- 姝ゅ懡浠ょ殑鏁版嵁


```
姣忎釜 connector 娑堟伅鍙互鍖呭惈涓€涓垨澶氫釜 w1_netlink_msg锛屼互鍙婇浂涓垨澶氫釜闄勫甫鐨?w1_netlink_cmd 娑堟伅銆?
瀵逛簬浜嬩欢娑堟伅锛屾病鏈夊祵鍏ョ殑 w1_netlink_cmd 缁撴瀯锛屽彧鏈?connector 澶撮儴鍜?w1_netlink_msg 缁撴瀯锛屽叾涓?"len" 瀛楁涓洪浂锛屽苟濉厖浜嗙被鍨嬶紙浜嬩欢绫诲瀷涔嬩竴锛夊拰 id锛氳涔堟槸涓绘満瀛楄妭搴忕殑 8 瀛楄妭浠庤澶囧敮涓€ id锛岃涔堟槸涓昏澶囩殑 id锛堝湪灏嗗叾娣诲姞鍒?w1 鏍稿績鏃跺垎閰嶇粰鎬荤嚎涓昏澶囷級銆?
鐩墠浠呭璇诲懡浠よ姹傜敓鎴愬鐢ㄦ埛绌洪棿鍛戒护鐨勫洖澶嶃€傛瘡涓?w1_netlink_cmd 璇昏姹傛伆濂界敓鎴愪竴涓洖澶嶃€傚彂閫佹椂鍥炲涓嶄細鍚堝苟鈥斺€斿嵆鍏稿瀷鐨勫洖澶?```

  [cn_msg][w1_netlink_msg][w1_netlink_cmd]
  cn_msg.len = sizeof(struct w1_netlink_msg) +
	     sizeof(struct w1_netlink_cmd) +
	     cmd->len;
  w1_netlink_msg.len = sizeof(struct w1_netlink_cmd) + cmd->len;
  w1_netlink_cmd.len = cmd->len;

```
瀵?W1_LIST_MASTERS 鐨勫洖澶嶅簲鍚戠敤鎴风┖闂村彂鍥炰竴鏉℃秷鎭紝鍏朵腑鍖呭惈浠ヤ笅褰㈠紡鐨勬墍鏈夊凡娉ㄥ唽涓昏澶?id 鍒楄〃
```

	cn_msg (CN_W1_IDX.CN_W1_VAL 浣滀负 id锛宭en 绛変簬 sizeof(struct
	w1_netlink_msg) 鍔犱笂涓昏澶囨暟閲忎箻浠?4)
	w1_netlink_msg (type: W1_LIST_MASTERS, len 绛変簬
		涓昏澶囨暟閲忎箻浠?4 (u32 澶у皬))
	id0 ... idN

```
姣忔潯娑堟伅鏈€澶т负 4k锛屽洜姝ゅ鏋滀富璁惧鏁伴噺瓒呰繃姝ゅ€硷紝瀹冨皢琚媶鍒嗕负澶氭潯娑堟伅銆?
W1 鎼滅储鍜屾姤璀︽悳绱㈠懡浠ゃ€?```

  [cn_msg]
    [w1_netlink_msg type = W1_MASTER_CMD
	id is equal to the bus master id to use for searching]
    [w1_netlink_cmd cmd = W1_CMD_SEARCH or W1_CMD_ALARM_SEARCH]

```
```

  [cn_msg, ack = 1 and increasing, 0 means the last message,
	seq is equal to the request seq]
  [w1_netlink_msg type = W1_MASTER_CMD]
  [w1_netlink_cmd cmd = W1_CMD_SEARCH or W1_CMD_ALARM_SEARCH
	len is equal to number of IDs multiplied by 8]
  [64bit-id0 ... 64bit-idN]

```
姣忎釜澶撮儴涓殑闀垮害瀵瑰簲浜庡叾鍚庨潰鏁版嵁鐨勫ぇ灏忥紝鍥犳
w1_netlink_cmd->len = N * 8锛涘叾涓?N 鏄湰娑堟伅涓?ID 鐨勬暟閲忋€傚彲浠ヤ负闆躲€?```

  w1_netlink_msg->len = sizeof(struct w1_netlink_cmd) + N * 8;
  cn_msg->len = sizeof(struct w1_netlink_msg) +
	      sizeof(struct w1_netlink_cmd) +
	      N*8;

```
```

  [cn_msg]
    [w1_netlink_msg type = W1_MASTER_CMD
	id is equal to the bus master id to use for searching]
    [w1_netlink_cmd cmd = W1_CMD_RESET]


```
## 鍛戒护鐘舵€佸洖澶?

姣忎釜鍛戒护锛堟棤璁烘槸 root銆乵aster 杩樻槸 slave锛屾棤璁烘槸鍚﹀甫鏈?w1_netlink_cmd 缁撴瀯锛夐兘浼氳 w1 鏍稿績鈥滅‘璁も€濓紙acked锛夈€傚洖澶嶇殑鏍煎紡涓庤姹傛秷鎭浉鍚岋紝鍙槸闀垮害鍙傛暟涓嶈鍏ョ敤鎴疯姹傜殑鏁版嵁锛屽嵆璇?鍐?瑙︽懜 IO 璇锋眰灏嗕笉鍖呭惈鏁版嵁锛屽洜姝?w1_netlink_cmd.len 灏嗕负 0锛寃1_netlink_msg.len 灏嗕负 w1_netlink_cmd 缁撴瀯鐨勫ぇ灏忥紝鑰?cn_msg.len 灏嗙瓑浜?sizeof(struct w1_netlink_msg) 涓?sizeof(struct w1_netlink_cmd) 涔嬪拰銆傚鏋滃洖澶嶆槸涓?master 鎴?root 鍛戒护锛堜笉甯?w1_netlink_cmd锛夌敓鎴愮殑锛屽垯鍥炲浠呭寘鍚?cn_msg 鍜?w1_netlink_msg 缁撴瀯銆?
w1_netlink_msg.status 瀛楁灏嗘惡甯︽鐨勯敊璇€硷紙渚嬪 EINVAL锛夋垨鎴愬姛鏃剁殑 0銆?
姣忎釜缁撴瀯涓殑鎵€鏈夊叾浠栧瓧娈靛皢闀滃儚璇锋眰娑堟伅涓殑鐩稿悓鍙傛暟锛堥櫎涓婅堪闀垮害澶栵級銆?
浼氫负 w1_netlink_msg 涓祵鍏ョ殑姣忎釜 w1_netlink_cmd 鐢熸垚鐘舵€佸洖澶嶏紱濡傛灉娌℃湁 w1_netlink_cmd 缁撴瀯锛屽垯灏嗕负 w1_netlink_msg 鐢熸垚鍥炲銆?
鍦ㄦ瘡涓?w1_netlink_msg 涓紝鎵€鏈?w1_netlink_cmd 鍛戒护缁撴瀯閮戒細琚鐞嗭紝鍗充娇瀛樺湪閿欒锛屽彧鏈夐暱搴︿笉鍖归厤鎵嶄細涓柇娑堟伅澶勭悊銆?

## 褰撴帴鏀跺埌鏂板懡浠ゆ椂 w1 鏍稿績涓殑鎿嶄綔姝ラ


褰撴帴鏀跺埌鏂版秷鎭紙w1_netlink_msg锛夋椂锛寃1 鏍稿績鏍规嵁 w1_netlink_msg.type 瀛楁妫€娴嬪畠鏄?master 杩樻槸 slave 璇锋眰銆傜劧鍚庢悳绱?master 鎴?slave 璁惧銆傛壘鍒板悗锛宮aster 璁惧锛堣璇锋眰鐨勶紝鎴栨槸鎵惧埌 slave 璁惧鐨勯偅涓級琚攣瀹氥€傚鏋滆姹傜殑鏄?slave 鍛戒护锛屽垯鍚姩澶嶄綅/閫夋嫨锛坮eset/select锛夎繃绋嬩互閫夋嫨缁欏畾璁惧銆?
鐒跺悗 w1_netlink_msg 涓姹傜殑鎵€鏈夋搷浣滆閫愪竴鎵ц銆傚鏋滃懡浠ら渶瑕佸洖澶嶏紙濡傝鍛戒护锛夛紝鍒欏湪鍛戒护瀹屾垚鏃跺彂閫併€?
褰撴墍鏈夊懡浠わ紙w1_netlink_cmd锛夊鐞嗗畬姣曞悗锛宮aster 璁惧琚В閿侊紝骞跺紑濮嬪鐞嗕笅涓€涓?w1_netlink_msg 澶撮儴銆?

## Connector [1] 鐗瑰畾鏂囨。


姣忎釜 connector 娑堟伅鍖呭惈涓や釜 u32 瀛楁浣滀负鈥滃湴鍧€鈥濄€倃1 浣跨敤鍦?include/linux/connector.h 澶存枃浠朵腑瀹氫箟鐨?CN_W1_IDX 鍜?CN_W1_VAL銆傛瘡涓秷鎭繕鍖呭惈搴忓垪鍙峰拰纭鍙枫€?
浜嬩欢娑堟伅鐨勫簭鍒楀彿鏄浉搴旂殑鎬荤嚎涓昏澶囧簭鍒楀彿锛屾瘡閫氳繃璇ヤ富璁惧鍙戦€佷竴鏉′簨浠舵秷鎭氨閫掑銆傜敤鎴风┖闂磋姹傜殑搴忓垪鍙风敱鐢ㄦ埛绌洪棿搴旂敤绋嬪簭璁剧疆銆傚洖澶嶇殑搴忓垪鍙蜂笌璇锋眰涓殑鐩稿悓锛岀‘璁ゅ彿璁句负 seq+1銆?

## 闄勫姞鏂囨。銆佹簮浠ｇ爜绀轰緥


1. Documentation/driver-api/connector.rst
2. https://github.com/bioothod/w1

   姝ゅ綊妗ｅ寘鍚敤鎴风┖闂村簲鐢ㄧ▼搴?w1d.c锛屽畠浣跨敤璇?鍐?鎼滅储鍛戒护鎿嶄綔鎬荤嚎涓婃壘鍒扮殑鎵€鏈?master/slave 璁惧銆?