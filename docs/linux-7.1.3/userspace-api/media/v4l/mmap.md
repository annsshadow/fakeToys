
######## 娴?I/O锛堝唴瀛樻槧灏勶紝Memory Mapping锛?

褰撶敱 VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct `v4l2_capability` 鐨?`capabilities`
瀛楁涓殑 `V4L2_CAP_STREAMING` 鏍囧織琚疆浣嶆椂锛岃緭鍏ュ拰杈撳嚭璁惧鏀寔杩欑 I/O 鏂规硶銆?鏈変袱绫绘祦鏂规硶锛岃纭畾鏄惁鏀寔鍐呭瓨鏄犲皠杩欑鏂瑰紡锛屽簲鐢ㄧ▼搴忓繀椤诲皢鍐呭瓨绫诲瀷璁句负
`V4L2_MEMORY_MMAP` 鏉ヨ皟鐢?VIDIOC_REQBUFS ioctl銆?
娴侊紙Streaming锛夋槸涓€绉?I/O 鏂规硶锛屽叾涓簲鐢ㄧ▼搴忎笌椹卞姩涔嬮棿鍙氦鎹㈡寚鍚戠紦鍐插尯鐨勬寚閽堬紝鏁版嵁
鏈韩涓嶈澶嶅埗銆傚唴瀛樻槧灏勪富瑕佹棬鍦ㄦ妸璁惧鍐呭瓨涓殑缂撳啿鍖烘槧灏勫埌搴旂敤绋嬪簭鐨勫湴鍧€绌洪棿銆傝澶?鍐呭瓨鍙互鏄緥濡傚甫鏈夎棰戦噰闆嗛檮鍔犲崱鐨勬樉鍗′笂鐨勮棰戝唴瀛樸€備笉杩囷紝浣滀负闀挎湡浠ユ潵鏈€楂樻晥鐨?I/O
鏂规硶锛岃澶氬叾浠栭┍鍔ㄤ篃鏀寔娴侊紝鍦ㄥ彲 DMA 鐨勪富鍐呭瓨涓垎閰嶇紦鍐插尯銆?
涓€涓┍鍔ㄥ彲浠ユ敮鎸佸缁勭紦鍐插尯銆傛瘡缁勭敱涓€涓敮涓€鐨勭紦鍐插尯绫诲瀷鍊兼爣璇嗐€傝繖浜涚粍鏄浉浜掔嫭绔嬬殑锛?姣忕粍鍙互鎸佹湁涓嶅悓绫诲瀷鐨勬暟鎹€傝鍚屾椂璁块棶涓嶅悓鐨勭粍锛屽繀椤讳娇鐢ㄤ笉鍚岀殑鏂囦欢鎻忚堪绗︺€俒#f1]_

瑕佸垎閰嶈澶囩紦鍐插尯锛屽簲鐢ㄧ▼搴忚皟鐢?VIDIOC_REQBUFS ioctl锛屽苟浼犲叆鏈熸湜鐨勭紦鍐插尯鏁伴噺鍜岀紦鍐插尯
绫诲瀷锛屼緥濡?`V4L2_BUF_TYPE_VIDEO_CAPTURE`銆傚彧瑕佹病鏈変换浣曠紦鍐插尯浠嶅浜庢槧灏勭姸鎬侊紝杩欎釜 ioctl
涔熷彲浠ョ敤鏉ユ敼鍙樼紦鍐插尯鏁伴噺鎴栭噴鏀惧凡鍒嗛厤鐨勫唴瀛樸€?
鍦ㄥ簲鐢ㄧ▼搴忚兘澶熻闂繖浜涚紦鍐插尯涔嬪墠锛屽畠浠繀椤荤敤 `mmap()` 鍑芥暟鏄犲皠鍒拌嚜宸辩殑鍦板潃绌洪棿銆傜紦鍐?鍖哄湪璁惧鍐呭瓨涓殑浣嶇疆鍙互閫氳繃 VIDIOC_QUERYBUF ioctl 纭畾銆傚湪鍗曞钩闈紙single-planar锛?API 鐨勬儏鍐典笅锛宻truct `v4l2_buffer` 涓繑鍥炵殑 `m.offset` 鍜?`length` 浣滀负绗叚涓拰绗簩涓?鍙傛暟浼犵粰 `mmap()` 鍑芥暟銆傚綋浣跨敤澶氬钩闈紙multi-planar锛堿PI 鏃讹紝struct `v4l2_buffer` 鍖呭惈
涓€涓?struct `v4l2_plane` 缁撴瀯浣撴暟缁勶紝姣忎釜缁撴瀯浣撻兘鍖呭惈鑷繁鐨?`m.offset` 鍜?`length`銆傚綋
浣跨敤澶氬钩闈?API 鏃讹紝姣忎釜缂撳啿鍖虹殑姣忎釜骞抽潰閮藉繀椤诲垎鍒槧灏勶紝鍥犳瀵?`mmap()` 鐨勮皟鐢ㄦ鏁板簲褰?绛変簬缂撳啿鍖烘暟閲忎箻浠ユ瘡涓紦鍐插尯涓殑骞抽潰鏁伴噺銆俹ffset 鍜?length 鍊间笉寰楄淇敼銆傝璁颁綇锛岀紦鍐插尯
鍒嗛厤鍦ㄧ墿鐞嗗唴瀛樹腑锛岃€岄潪鍙互琚崲鍑哄埌纾佺洏鐨勮櫄鎷熷唴瀛樹腑銆傚簲鐢ㄧ▼搴忓簲褰撳敖蹇敤 `munmap()`
鍑芥暟閲婃斁杩欎簺缂撳啿鍖恒€?
## 绀轰緥锛氬湪鍗曞钩闈?API 涓槧灏勭紦鍐插尯


    struct v4l2_requestbuffers reqbuf;
    struct {
	void *start;
	size_t length;
    } *buffers;
    unsigned int i;

    memset(&reqbuf, 0, sizeof(reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_MMAP;
    reqbuf.count = 20;

    if (-1 == ioctl (fd, VIDIOC_REQBUFS, &reqbuf)) {
	if (errno == EINVAL)
	    printf("Video capturing or mmap-streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

    /** We want at least five buffers. **/

    if (reqbuf.count < 5) {
	/** You may need to free the buffers here. **/
	printf("Not enough buffer memory\\n");
	exit(EXIT_FAILURE);
    }

    buffers = calloc(reqbuf.count, sizeof(*buffers));
    assert(buffers != NULL);

    for (i = 0; i < reqbuf.count; i++) {
	struct v4l2_buffer buffer;

	memset(&buffer, 0, sizeof(buffer));
	buffer.type = reqbuf.type;
	buffer.memory = V4L2_MEMORY_MMAP;
	buffer.index = i;

	if (-1 == ioctl (fd, VIDIOC_QUERYBUF, &buffer)) {
	    perror("VIDIOC_QUERYBUF");
	    exit(EXIT_FAILURE);
	}

	buffers[i].length = buffer.length; /** remember for munmap() **/

	buffers[i].start = mmap(NULL, buffer.length,
		    PROT_READ | PROT_WRITE, /** recommended **/
		    MAP_SHARED,             /** recommended **/
		    fd, buffer.m.offset);

	if (MAP_FAILED == buffers[i].start) {
	    /* If you do not exit here you should unmap() and free()
	       the buffers mapped so far. */
	    perror("mmap");
	    exit(EXIT_FAILURE);
	}
    }

    /** Cleanup. **/

    for (i = 0; i < reqbuf.count; i++)
	munmap(buffers[i].start, buffers[i].length);

## 绀轰緥锛氬湪澶氬钩闈?API 涓槧灏勭紦鍐插尯


    struct v4l2_requestbuffers reqbuf;
    /** Our current format uses 3 planes per buffer **/
    #define FMT_NUM_PLANES = 3

    struct {
	void *start[FMT_NUM_PLANES];
	size_t length[FMT_NUM_PLANES];
    } *buffers;
    unsigned int i, j;

    memset(&reqbuf, 0, sizeof(reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
    reqbuf.memory = V4L2_MEMORY_MMAP;
    reqbuf.count = 20;

    if (ioctl(fd, VIDIOC_REQBUFS, &reqbuf) < 0) {
	if (errno == EINVAL)
	    printf("Video capturing or mmap-streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

    /** We want at least five buffers. **/

    if (reqbuf.count < 5) {
	/** You may need to free the buffers here. **/
	printf("Not enough buffer memory\\n");
	exit(EXIT_FAILURE);
    }

    buffers = calloc(reqbuf.count, sizeof(*buffers));
    assert(buffers != NULL);

    for (i = 0; i < reqbuf.count; i++) {
	struct v4l2_buffer buffer;
	struct v4l2_plane planes[FMT_NUM_PLANES];

	memset(&buffer, 0, sizeof(buffer));
	buffer.type = reqbuf.type;
	buffer.memory = V4L2_MEMORY_MMAP;
	buffer.index = i;
	/* length in struct v4l2_buffer in multi-planar API stores the size
  - of planes array. */
	buffer.length = FMT_NUM_PLANES;
	buffer.m.planes = planes;

	if (ioctl(fd, VIDIOC_QUERYBUF, &buffer) < 0) {
	    perror("VIDIOC_QUERYBUF");
	    exit(EXIT_FAILURE);
	}

	/** Every plane has to be mapped separately **/
	for (j = 0; j < FMT_NUM_PLANES; j++) {
	    buffers[i].length[j] = buffer.m.planes[j].length; /** remember for munmap() **/

	    buffers[i].start[j] = mmap(NULL, buffer.m.planes[j].length,
		     PROT_READ | PROT_WRITE, /** recommended **/
		     MAP_SHARED,             /** recommended **/
		     fd, buffer.m.planes[j].m.mem_offset);

	    if (MAP_FAILED == buffers[i].start[j]) {
		/* If you do not exit here you should unmap() and free()
		   the buffers and planes mapped so far. */
		perror("mmap");
		exit(EXIT_FAILURE);
	    }
	}
    }

    /** Cleanup. **/

    for (i = 0; i < reqbuf.count; i++)
	for (j = 0; j < FMT_NUM_PLANES; j++)
	    munmap(buffers[i].start[j], buffers[i].length[j]);

浠庢蹇典笂璁诧紝娴侀┍鍔ㄧ淮鎶や袱涓紦鍐插尯闃熷垪锛氫竴涓紶鍏ラ槦鍒楀拰涓€涓紶鍑洪槦鍒椼€傚畠浠妸閿佸畾鍒拌棰?鏃堕挓鐨勫悓姝ラ噰闆嗘垨杈撳嚭鎿嶄綔锛屼笌鍙兘鍙楀埌闅忔満纾佺洏鎴栫綉缁滃欢杩熶互鍙婂叾浠栬繘绋嬫姠鍗犲奖鍝嶇殑搴旂敤绋嬪簭
鍒嗙寮€鏉ワ紝浠庤€岄檷浣庝簡鏁版嵁涓㈠け鐨勬鐜囥€傞槦鍒椾互 FIFO 鏂瑰紡缁勭粐锛岀紦鍐插尯灏嗘寜鐓у畠浠湪浼犲叆 FIFO
涓叆闃熺殑椤哄簭杈撳嚭锛屽苟涓旀槸鍦ㄤ粠浼犲嚭 FIFO 鍑洪槦鏃剁殑椤哄簭琚噰闆嗙殑銆?
椹卞姩鍙兘瑕佹眰鍦ㄤ换浣曟椂鍒婚兘鑷冲皯鏈夋渶灏戞暟閲忕殑缂撳啿鍖哄叆闃熸墠鑳藉伐浣滐紝闄ゆ涔嬪锛屽搴旂敤绋嬪簭鍙互
鎻愬墠鍏ラ槦銆佹垨鍑洪槦骞跺鐞嗙殑缂撳啿鍖烘暟閲忔病鏈夐檺鍒躲€傚畠浠篃鍙互鎸変笌缂撳啿鍖哄嚭闃熶笉鍚岀殑椤哄簭鍏ラ槦锛?鑰岄┍鍔ㄥ彲浠ヤ互**浠绘剰**椤哄簭**濉厖**宸插叆闃熺殑**绌?*缂撳啿鍖恒€俒#f2]_ 缂撳啿鍖虹殑绱㈠紩鍙凤紙struct
`v4l2_buffer` 鐨?`index`锛夊湪杩欓噷涓嶈捣浣滅敤锛屽畠鍙槸鐢ㄤ簬鏍囪瘑缂撳啿鍖恒€?
鏈€鍒濓紝鎵€鏈夊凡鏄犲皠鐨勭紦鍐插尯閮藉浜庡嚭闃熺姸鎬侊紝椹卞姩鏃犳硶璁块棶銆傚浜庨噰闆嗙被搴旂敤绋嬪簭锛屼範鎯笂鍏?鎶婃墍鏈夌殑宸叉槧灏勭紦鍐插尯鍏ラ槦锛岀劧鍚庡紑濮嬮噰闆嗗苟杩涘叆璇诲彇寰幆銆傚湪杩欓噷搴旂敤绋嬪簭绛夊緟锛岀洿鍒颁竴涓?宸插～鍏呯殑缂撳啿鍖哄彲浠ヨ鍑洪槦锛屽苟鍦ㄦ暟鎹笉鍐嶉渶瑕佹椂閲嶆柊鍏ラ槦璇ョ紦鍐插尯銆傝緭鍑虹被搴旂敤绋嬪簭濉厖骞?鍏ラ槦缂撳啿鍖猴紝褰撳爢绉簡瓒冲鐨勭紦鍐插尯鍚庯紝鐢?VIDIOC_STREAMON <VIDIOC_STREAMON> 寮€濮嬭緭鍑恒€傚湪
鍐欏叆寰幆涓紝褰撳簲鐢ㄧ▼搴忕敤鍏夌┖闂茬紦鍐插尯鏃讹紝瀹冨繀椤荤瓑寰咃紝鐩村埌涓€涓┖缂撳啿鍖哄彲浠ヨ鍑洪槦骞跺鐢ㄣ€?
瑕佸叆闃熷拰鍑洪槦涓€涓紦鍐插尯锛屽簲鐢ㄧ▼搴忎娇鐢?VIDIOC_QBUF <VIDIOC_QBUF> 鍜?VIDIOC_DQBUF
<VIDIOC_QBUF> ioctl銆備竴涓紦鍐插尯澶勪簬宸叉槧灏勩€佸凡鍏ラ槦銆佸凡婊℃垨宸茬┖鐨勭姸鎬侊紝鍦ㄤ换浣曟椂鍊欓兘鍙互閫氳繃
VIDIOC_QUERYBUF ioctl 纭畾銆傚瓨鍦ㄤ袱绉嶆柟娉曟潵鎸傝捣搴旂敤绋嬪簭鐨勬墽琛岋紝鐩村埌涓€涓垨澶氫釜缂撳啿鍖哄彲浠?琚嚭闃熴€傞粯璁ゆ儏鍐典笅锛屽綋娌℃湁缂撳啿鍖哄湪浼犲嚭闃熷垪涓椂锛孷IDIOC_DQBUF <VIDIOC_QBUF> 浼氶樆濉炪€傚綋
鍚?`open()` 鍑芥暟浼犲叆浜?`O_NONBLOCK` 鏍囧織鏃讹紝鍦ㄦ病鏈夌紦鍐插尯鍙敤鏃讹紝VIDIOC_DQBUF
<VIDIOC_QBUF> 浼氱珛鍗宠繑鍥?`EAGAIN` 閿欒鐮併€俙select()` 鎴?`poll()` 鍑芥暟濮嬬粓鍙敤銆?
瑕佸紑濮嬪拰鍋滄閲囬泦鎴栬緭鍑猴紝搴旂敤绋嬪簭璋冪敤 VIDIOC_STREAMON <VIDIOC_STREAMON> 鍜?:ref:`VIDIOC_STREAMOFF <VIDIOC_STREAMON>` ioctl銆?
   浣滀负鍓綔鐢紝瀹冧細鎶婁袱涓槦鍒椾腑鐨勬墍鏈夌紦鍐插尯閮界Щ闄ゃ€傜敱浜庡湪涓€涓浠诲姟绯荤粺涓婁笉瀛樺湪鈥滅幇鍦ㄢ€?   灏卞仛鏌愪簨鐨勬蹇碉紝濡傛灉涓€涓簲鐢ㄧ▼搴忛渶瑕佷笌鍏朵粬浜嬩欢鍚屾锛屽畠搴斿綋妫€鏌ユ墍閲囬泦鎴栬緭鍑虹紦鍐插尯鐨?   struct :`v4l2_buffer` `timestamp`銆?
瀹炵幇鍐呭瓨鏄犲皠 I/O 鐨勯┍鍔ㄥ繀椤绘敮鎸?VIDIOC_REQBUFS <VIDIOC_REQBUFS>銆?ref:`VIDIOC_QUERYBUF
<VIDIOC_QUERYBUF>`銆乂IDIOC_QBUF <VIDIOC_QBUF>銆?ref:`VIDIOC_DQBUF
<VIDIOC_QBUF>`銆乂IDIOC_STREAMON <VIDIOC_STREAMON> 鍜?VIDIOC_STREAMOFF
<VIDIOC_STREAMON> ioctl锛屼互鍙?:ref:`mmap() <func-mmap>`銆乣munmap()`銆?ref:`select()
<func-select>` 鍜?`poll()` 鍑芥暟銆俒#f3]_

[閲囬泦绀轰緥]

   鍙互浣跨敤涓€涓枃浠舵弿杩扮锛屽苟鍦ㄨ皟鐢?VIDIOC_QBUF 绛夋椂鐩稿簲鍦拌缃紦鍐插尯绫诲瀷瀛楁锛屼絾杩欎細璁?   `select()` 鍑芥暟鍙樺緱鍚硦銆傛垜浠洿鍠滄姣忎釜閫昏緫娴佷竴涓枃浠舵弿杩扮杩欑骞插噣鐨勫仛娉曘€備緥濡傝棰?   鍙犲姞锛坥verlay锛変篃鏄竴涓€昏緫娴侊紝灏界杩炵画杩愯骞朵笉闇€瑕?CPU銆?
   闅忔満鍏ラ槦椤哄簭鍏佽涔卞簭澶勭悊鍥惧儚锛堜緥濡傝棰戠紪瑙ｇ爜鍣級鐨勫簲鐢ㄧ▼搴忔洿鏃╁湴褰掕繕缂撳啿鍖猴紝浠庤€岄檷浣?   鏁版嵁涓㈠け鐨勬鐜囥€傞殢鏈哄～鍏呴『搴忓厑璁搁┍鍔ㄥ熀浜?LIFO 澶嶇敤缂撳啿鍖猴紝鍒╃敤缂撳瓨涓繚瀛樼殑鍒嗘暎-鑱氶泦
   鍒楄〃绛夈€?
   鍦ㄩ┍鍔ㄥ眰闈紝`select()` 鍜?`poll()` 鏄浉鍚岀殑锛岃€?`select()` 澶噸瑕佷簡锛屼笉鑳芥垚涓哄彲閫夐」銆?   鍏朵綑鐨勫簲褰撴槸涓嶈█鑷槑鐨勩€?