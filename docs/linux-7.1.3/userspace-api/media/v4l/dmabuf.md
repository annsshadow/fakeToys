

######## 娴佸紡 I/O锛圖MA 缂撳啿鍖哄鍏ワ級


DMABUF 妗嗘灦鎻愪緵浜嗕竴绉嶅湪澶氫釜璁惧涔嬮棿鍏变韩缂撳啿鍖虹殑閫氱敤鏂规硶銆傛敮鎸?DMABUF 鐨勮澶囬┍鍔?鍙互灏嗕竴涓?DMA 缂撳啿鍖轰綔涓烘枃浠舵弿杩扮瀵煎嚭鍒扮敤鎴锋€侊紙绉颁负 exporter锛屽鍑鸿€呰鑹诧級锛?浣跨敤鍏堝墠涓轰笉鍚屾垨鍚屼竴璁惧瀵煎嚭鐨勬枃浠舵弿杩扮浠庣敤鎴锋€佸鍏ヤ竴涓?DMA 缂撳啿鍖猴紙绉颁负
importer锛屽鍏ヨ€呰鑹诧級锛屾垨鍚屾椂鏀寔涓よ€呫€傛湰鑺傛弿杩?V4L2 涓殑 DMABUF 瀵煎叆鑰呰鑹?API銆?
鍏充簬灏?V4L2 缂撳啿鍖哄鍑轰负 DMABUF 鏂囦欢鎻忚堪绗︾殑缁嗚妭锛岃鍙傝 DMABUF 瀵煎嚭
<VIDIOC_EXPBUF>銆?
褰撶敱 VIDIOC_QUERYCAP <VIDIOC_QUERYCAP> ioctl 杩斿洖鐨?struct `v4l2_capability`
鐨?`capabilities` 瀛楁涓殑 `V4L2_CAP_STREAMING` 鏍囧織琚疆浣嶆椂锛岃緭鍏ュ拰杈撳嚭璁惧
鏀寔娴佸紡 I/O 鏂规硶銆傛槸鍚︽敮鎸侀€氳繃 DMABUF 鏂囦欢鎻忚堪绗﹀鍏?DMA 缂撳啿鍖猴紝鍒欑敱浠ュ唴瀛樼被鍨?璁剧疆涓?`V4L2_MEMORY_DMABUF` 璋冪敤 VIDIOC_REQBUFS <VIDIOC_REQBUFS> ioctl 鏉ョ‘瀹氥€?
鏈?I/O 鏂规硶涓撶敤浜庡湪涓嶅悓璁惧涔嬮棿鍏变韩 DMA 缂撳啿鍖猴紝杩欎簺璁惧鍙互鏄?V4L 璁惧鎴栧叾浠?瑙嗛鐩稿叧璁惧锛堝 DRM锛夈€傜紦鍐插尯锛堝钩闈級鐢遍┍鍔ㄤ唬琛ㄥ簲鐢ㄧ▼搴忓垎閰嶃€傛帴鐫€锛岃繖浜涚紦鍐插尯
閫氳繃鍒嗛厤鍣ㄩ┍鍔ㄧ壒瀹氱殑 API 浣滀负鏂囦欢鎻忚堪绗﹀鍑虹粰搴旂敤绋嬪簭銆傚彧鏈夎繖鏍风殑鏂囦欢鎻忚堪绗﹁
浜ゆ崲銆傛弿杩扮鍜屽厓淇℃伅鍦?struct `v4l2_buffer`锛堟垨瀵逛簬澶氬钩闈?API 鎯呭舰鍦?struct
`v4l2_plane`锛変腑浼犻€掋€傚繀椤婚€氳繃浠ユ湡鏈涚殑缂撳啿鍖虹被鍨嬭皟鐢?VIDIOC_REQBUFS
<VIDIOC_REQBUFS> 灏嗛┍鍔ㄥ垏鎹㈠埌 DMABUF I/O 妯″紡銆?
## 绀轰緥锛氫娇鐢?DMABUF 鏂囦欢鎻忚堪绗﹀彂璧锋祦寮?I/O



    struct v4l2_requestbuffers reqbuf;

    memset(&reqbuf, 0, sizeof (reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_DMABUF;
    reqbuf.count = 1;

    if (ioctl(fd, VIDIOC_REQBUFS, &reqbuf) == -1) {
	if (errno == EINVAL)
	    printf("Video capturing or DMABUF streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

缂撳啿鍖猴紙骞抽潰锛夋枃浠舵弿杩扮闅?VIDIOC_QBUF <VIDIOC_QBUF> ioctl 鍗虫椂浼犲叆銆傚浜庡骞抽潰
缂撳啿鍖猴紝姣忎釜骞抽潰閮藉彲浠ュ叧鑱斾竴涓笉鍚岀殑 DMABUF 鎻忚堪绗︺€傚敖绠＄紦鍐插尯閫氬父琚惊鐜娇鐢紝
浣嗗簲鐢ㄧ▼搴忎篃鍙互鍦ㄦ瘡娆?VIDIOC_QBUF <VIDIOC_QBUF> 璋冪敤鏃朵紶鍏ヤ笉鍚岀殑 DMABUF 鎻忚堪绗︺€?
## 绀轰緥锛氫娇鐢ㄥ崟骞抽潰 API 灏?DMABUF 鍏ラ槦



    int buffer_queue(int v4lfd, int index, int dmafd)
    {
	struct v4l2_buffer buf;

	memset(&buf, 0, sizeof buf);
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
	buf.memory = V4L2_MEMORY_DMABUF;
	buf.index = index;
	buf.m.fd = dmafd;

	if (ioctl(v4lfd, VIDIOC_QBUF, &buf) == -1) {
	    perror("VIDIOC_QBUF");
	    return -1;
	}

	return 0;
    }

## 绀轰緥 3.6. 浣跨敤澶氬钩闈?API 灏?DMABUF 鍏ラ槦



    int buffer_queue_mp(int v4lfd, int index, int dmafd[], int n_planes)
    {
	struct v4l2_buffer buf;
	struct v4l2_plane planes[VIDEO_MAX_PLANES];
	int i;

	memset(&buf, 0, sizeof buf);
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
	buf.memory = V4L2_MEMORY_DMABUF;
	buf.index = index;
	buf.m.planes = planes;
	buf.length = n_planes;

	memset(&planes, 0, sizeof planes);

	for (i = 0; i < n_planes; ++i)
	    buf.m.planes[i].m.fd = dmafd[i];

	if (ioctl(v4lfd, VIDIOC_QBUF, &buf) == -1) {
	    perror("VIDIOC_QBUF");
	    return -1;
	}

	return 0;
    }

鎹曡幏鎴栨樉绀虹殑缂撳啿鍖洪€氳繃 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鍑洪槦銆傞┍鍔ㄥ彲浠ュ湪 DMA
瀹屾垚涓庢 ioctl 涔嬮棿鐨勪换鎰忔椂鍒昏В閿佽缂撳啿鍖恒€傚綋璋冪敤 VIDIOC_STREAMOFF
<VIDIOC_STREAMON>銆乂IDIOC_REQBUFS <VIDIOC_REQBUFS>锛屾垨璁惧琚叧闂椂锛屽唴瀛樹篃浼氳
瑙ｉ攣銆?
瀵逛簬鎹曡幏绫诲簲鐢ㄧ▼搴忥紝閫氬父鐨勫仛娉曟槸棰勫厛鍏ラ槦鑻ュ共绌虹紦鍐插尯锛岀劧鍚庡惎鍔ㄦ崟鑾峰苟杩涘叆璇诲彇
寰幆銆傚湪姝ゅ惊鐜腑锛屽簲鐢ㄧ▼搴忕瓑寰呯洿鍒版湁宸插～鍏呯殑缂撳啿鍖哄彲浠ュ嚭闃燂紝骞跺湪鏁版嵁涓嶅啀闇€瑕佹椂
閲嶆柊鍏ラ槦璇ョ紦鍐插尯銆傝緭鍑虹被搴旂敤绋嬪簭鍒欏～鍏呭苟鍏ラ槦缂撳啿鍖猴紝褰撶疮绉簡瓒冲澶氱殑缂撳啿鍖哄悗
寮€濮嬭緭鍑恒€傚湪鍐欏叆寰幆涓紝褰撳簲鐢ㄧ▼搴忚€楀敖绌洪棽缂撳啿鍖烘椂锛屽畠蹇呴』绛夊緟鐩村埌鏈夌┖缂撳啿鍖?鍙互鍑洪槦骞跺鐢ㄣ€傚瓨鍦ㄤ袱绉嶆柟娉曞彲鎸傝捣搴旂敤绋嬪簭鐨勬墽琛岋紝鐩村埌鏈変竴涓垨澶氫釜缂撳啿鍖哄彲琚?鍑洪槦銆傞粯璁ゆ儏鍐典笅锛屽綋鍑洪槦闃熷垪涓病鏈夌紦鍐插尯鏃?:ref:`VIDIOC_DQBUF <VIDIOC_QBUF>`
浼氶樆濉炪€傚綋鍚?`open()` 鍑芥暟浼犲叆浜?`O_NONBLOCK` 鏍囧織鏃讹紝VIDIOC_DQBUF
<VIDIOC_QBUF> 鍦ㄦ病鏈夊彲鐢ㄧ紦鍐插尯鏃朵細绔嬪嵆杩斿洖 `EAGAIN` 閿欒鐮併€傝€?`select()` 鍜?`poll()` 鍑芥暟濮嬬粓鍙敤銆?
瑕佸惎鍔ㄥ拰鍋滄鎹曡幏鎴栨樉绀虹被搴旂敤绋嬪簭锛岃璋冪敤 VIDIOC_STREAMON <VIDIOC_STREAMON> 鍜?VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctls銆?

   VIDIOC_STREAMOFF <VIDIOC_STREAMON> 浣滀负鍓綔鐢ㄤ細浠庝袱涓槦鍒椾腑绉婚櫎鎵€鏈夌紦鍐插尯
   骞惰В閿佹墍鏈夌紦鍐插尯銆傜敱浜庡湪澶氫换鍔＄郴缁熶笂娌℃湁"绔嬪嵆"鎵ц鏌愪簨鐨勬蹇碉紝濡傛灉搴旂敤绋嬪簭
   闇€瑕佷笌鍏朵粬浜嬩欢鍚屾锛屽畠搴斿綋妫€鏌ュ凡鎹曡幏鎴栧凡杈撳嚭缂撳啿鍖虹殑 struct `v4l2_buffer`
   鐨?`timestamp`锛堟椂闂存埑锛夈€?
瀹炵幇 DMABUF 瀵煎叆 I/O 鐨勯┍鍔ㄥ繀椤绘敮鎸?VIDIOC_REQBUFS <VIDIOC_REQBUFS>銆?VIDIOC_QBUF <VIDIOC_QBUF>銆乂IDIOC_DQBUF <VIDIOC_DQBUF>銆?ref:`VIDIOC_STREAMON
<VIDIOC_STREAMON>` 鍜?VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctls锛屼互鍙?`select()` 鍜?`poll()` 鍑芥暟銆?