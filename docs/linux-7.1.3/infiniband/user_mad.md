## 鐢ㄦ埛鎬?MAD 璁块棶


## 璁惧鏂囦欢


  姣忎釜 InfiniBand 璁惧鐨勬瘡涓鍙ｉ兘闄勬湁涓€涓?"umad" 璁惧鍜屼竴涓?"issm" 璁惧銆備緥濡傦紝
  涓€涓弻绔彛鐨?HCA 灏嗘湁涓や釜 umad 璁惧鍜屼袱涓?issm 璁惧锛岃€屼竴涓氦鎹㈡満锛坰witch锛夊皢鏈?  姣忕绫诲瀷鍚勪竴涓澶囷紙瀵瑰簲浜ゆ崲鏈虹鍙?0锛夈€?
## 鍒涘缓 MAD 浠ｇ悊锛坅gents锛?

  鍙互閫氳繃濉厖涓€涓?struct ib_user_mad_reg_req锛岀劧鍚庡湪瀵圭浉搴旇澶囨枃浠剁殑鏂囦欢鎻忚堪绗?  涓婅皟鐢?IB_USER_MAD_REGISTER_AGENT ioctl 鏉ュ垱寤轰竴涓?MAD 浠ｇ悊銆傚鏋滄敞鍐岃姹傛垚鍔燂紝
  涓€涓?32 浣嶇殑 id 灏嗚杩斿洖鍒拌缁撴瀯涓€?```

	struct ib_user_mad_reg_req req = { /* ... */ };
	ret = ioctl(fd, IB_USER_MAD_REGISTER_AGENT, (char *) &req);
        if (!ret)
		my_agent = req.id;
	else
		perror("agent register");

  浠ｇ悊鍙互閫氳繃 IB_USER_MAD_UNREGISTER_AGENT ioctl 娉ㄩ攢銆傛澶栵紝閫氳繃涓€涓枃浠舵弿杩扮
  娉ㄥ唽鐨勬墍鏈変唬鐞嗗皢鍦ㄨ繖璇ユ弿杩扮琚叧闂椂娉ㄩ攢銆?
  2014
       鐜板湪鎻愪緵浜嗕竴涓柊鐨勬敞鍐?ioctl锛屽厑璁稿湪娉ㄥ唽鏈熼棿鎻愪緵棰濆鐨勫瓧娈点€傛娉ㄥ唽璋冪敤鐨?       浣跨敤鑰呴殣寮忓湴璁剧疆浜?pkey_index 鐨勪娇鐢紙瑙佷笅鏂囷級銆?
```
## 鎺ユ敹 MAD


  MAD 閫氳繃 read() 鎺ユ敹銆傛帴鏀剁鐜板湪鏀寔 RMPP銆備紶缁?read() 鐨勭紦鍐插尯蹇呴』鑷冲皯涓?  涓€涓?struct ib_user_mad + 256 瀛楄妭銆備緥濡傦細

  濡傛灉浼犲叆鐨勭紦鍐插尯涓嶅澶т互瀹圭撼鎺ユ敹鍒扮殑 MAD锛圧MPP锛夛紝errno 浼氳璁句负 ENOSPC锛屽苟涓?  鎵€闇€缂撳啿鍖虹殑闀垮害琚缃埌 mad.length 涓€?
```

	struct ib_user_mad *mad;
	mad = malloc(sizeof *mad + 256);
	ret = read(fd, mad, sizeof *mad + 256);
	if (ret != sizeof mad + 256) {
		perror("read");
		free(mad);
	}

  RMPP 璇诲彇鐨勭ず渚?:

	struct ib_user_mad *mad;
	mad = malloc(sizeof *mad + 256);
	ret = read(fd, mad, sizeof *mad + 256);
	if (ret == -ENOSPC)) {
		length = mad.length;
		free(mad);
		mad = malloc(sizeof *mad + length);
		ret = read(fd, mad, sizeof *mad + length);
	}
	if (ret < 0) {
		perror("read");
		free(mad);
	}

  闄や簡瀹為檯鐨?MAD 鍐呭澶栵紝struct ib_user_mad 鐨勫叾瀹冨瓧娈典篃浼氳濉厖涓婂叧浜庢帴鏀跺埌鐨?  MAD 鐨勪俊鎭€備緥濡傦紝杩滅 LID 灏嗗湪 mad.lid 涓€?
  濡傛灉鍙戦€佽秴鏃讹紝灏嗙敓鎴愪竴涓帴鏀讹紝鍏?mad.status 琚涓?ETIMEDOUT銆傚惁鍒欙紝褰撲竴涓?MAD
  琚垚鍔熸帴鏀舵椂锛宮ad.status 灏嗕负 0銆?
  poll()/select() 鍙敤浜庣瓑寰呯洿鍒颁竴涓?MAD 鍙互琚鍙栥€?
```
## 鍙戦€?MAD


  MAD 閫氳繃 write() 鍙戦€併€傜敤浜庡彂閫佺殑浠ｇ悊 ID 搴旇濉叆 MAD 鐨?id 瀛楁锛岀洰鐨?LID 搴旇
  濉叆 lid 瀛楁锛屼緷姝ょ被鎺ㄣ€傚彂閫佺纭疄鏀寔
```

	struct ib_user_mad *mad;

	mad = malloc(sizeof *mad + mad_length);

	/* 濉厖 mad->data */

	mad->hdr.id  = my_agent;	/* 鏉ヨ嚜浠ｇ悊娉ㄥ唽鐨?req.id */
	mad->hdr.lid = my_dest;		/* 缃戠粶瀛楄妭搴?.. */
	/* 绛夌瓑 */

	ret = write(fd, &mad, sizeof *mad + mad_length);
	if (ret != sizeof *mad + mad_length)
		perror("write");

```
## 浜嬪姟 ID锛圱ransaction IDs锛?

  鐢ㄦ埛鎬?umad 璁惧鐨勪娇鐢ㄨ€呭彲浠ヤ娇鐢ㄤ簨鍔?ID 瀛楁鐨勪綆 32 浣嶏紙鍗冲湪缃戠粶瀛楄妭搴忎腑璇ュ瓧娈?  鐨勮緝浣庢湁鏁堜竴鍗婏級鏉ュ尮閰嶆鍦ㄥ彂閫佺殑 MAD 涓殑璇锋眰/鍝嶅簲瀵广€傞珮 32 浣嶄繚鐣欑粰鍐呮牳浣跨敤锛?  骞跺皢鍦?MAD 琚彂閫佷箣鍓嶈瑕嗙洊銆?
## P_Key 绱㈠紩澶勭悊


  鏃х殑 ib_umad 鎺ュ彛涓嶅厑璁镐负鍙戦€佺殑 MAD 璁剧疆 P_Key 绱㈠紩锛屼篃涓嶆彁渚涜幏鍙栨帴鏀跺埌鐨?MAD 鐨?  P_Key 绱㈠紩鐨勬柟娉曘€傚凡缁忓畾涔変簡涓€涓甫鏈?pkey_index 鎴愬憳鐨?struct ib_user_mad_hdr 鐨?  鏂板竷灞€锛涚劧鑰岋紝涓轰簡涓庢棫搴旂敤绋嬪簭淇濇寔浜岃繘鍒跺吋瀹规€э紝闄ら潪鍦ㄦ枃浠舵弿杩扮琚敤浜庡叾瀹冧换浣?  鎿嶄綔涔嬪墠璋冪敤浜?IB_USER_MAD_ENABLE_PKEY 鎴?IB_USER_MAD_REGISTER_AGENT2 ioctl 涔嬩竴锛?  鍚﹀垯涓嶄細浣跨敤杩欎釜鏂板竷灞€銆?
  鍦?2008 骞?9 鏈堬紝IB_USER_MAD_ABI_VERSION 灏嗛€掑鍒?6锛宻truct ib_user_mad_hdr 鐨?  鏂板竷灞€灏嗛粯璁や娇鐢紝骞朵笖 IB_USER_MAD_ENABLE_PKEY ioctl 灏嗚绉婚櫎銆?
## 璁剧疆 IsSM 鑳藉姏浣?

  瑕佷负鏌愪釜绔彛璁剧疆 IsSM 鑳藉姏浣嶏紝鍙渶鎵撳紑鐩稿簲鐨?issm 璁惧鏂囦欢銆傚鏋?IsSM 浣嶅凡缁忚缃紝
  鍒?open 璋冪敤灏嗛樆濉炵洿鍒拌浣嶈娓呴櫎锛堟垨鑰咃紝濡傛灉鍚?open() 浼犲叆浜?O_NONBLOCK 鏍囧織锛屽垯
  绔嬪嵆杩斿洖骞跺皢 errno 璁句负 EAGAIN锛夈€傚綋 issm 鏂囦欢琚叧闂椂锛孖sSM 浣嶅皢琚竻闄ゃ€備笉鑳藉
  issm 鏂囦欢鎵ц read銆亀rite 鎴栧叾瀹冩搷浣溿€?
## /dev 鏂囦欢


  瑕佷娇鐢ㄤ互涓嬭鍒欒嚜鍔ㄥ垱寤虹浉搴旂殑瀛楃璁惧鏂囦欢
```

    KERNEL=="umad*", NAME="infiniband/%k"
    KERNEL=="issm*", NAME="infiniband/%k"

  杩欏彲浠ョ敤鏉ャ€傝繖灏嗗垱寤鸿澶囪妭鐐癸紝鍛藉悕涓?:

    /dev/infiniband/umad0
    /dev/infiniband/issm0

  瀵瑰簲绗竴涓鍙ｏ紝渚濇绫绘帹銆備笌杩欎簺璁惧鍏宠仈鐨?InfiniBand 璁惧鍜岀鍙ｅ彲浠ヤ粠浠ヤ笅鏂囦欢
  纭畾::

    /sys/class/infiniband_mad/umad0/ibdev
    /sys/class/infiniband_mad/umad0/port

  浠ュ強::

    /sys/class/infiniband_mad/issm0/ibdev
    /sys/class/infiniband_mad/issm0/port

```
