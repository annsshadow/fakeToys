


######## 鍥惧儚瑁佸壀銆佹彃鍏ヤ笌缂╂斁 鈥?CROP API


   CROP API 澶у宸茶鏇存柊鐨?:ref:`SELECTION API
   <selection-api>` 鍙栦唬銆傚湪澶у鏁版儏鍐典笅搴斾紭鍏堜娇鐢ㄦ柊 API锛?   鍞竴鐨勪緥澶栨槸鍍忕礌瀹介珮姣旓紙pixel aspect ratio锛夋娴嬶紝瀹冪敱
   VIDIOC_CROPCAP <VIDIOC_CROPCAP> 瀹炵幇锛屽湪 SELECTION API 涓病鏈?   瀵瑰簲鐨勫姛鑳姐€傚弬瑙?selection-vs-crop 浠ヤ簡瑙ｈ繖涓や釜 API 鐨勫姣斻€?
鏈変簺瑙嗛閲囬泦璁惧鍙互閲囨牱鍥惧儚鐨勪竴涓瓙鍖哄煙锛屽苟灏嗗叾缂╁皬鎴栨斁澶у埌浠绘剰
灏哄鐨勫浘鍍忋€傛垜浠О杩欎簺鑳藉姏涓鸿鍓紙cropping锛変笌缂╂斁锛坰caling锛夈€傛湁浜?瑙嗛杈撳嚭璁惧鍙互灏嗗浘鍍忔斁澶ф垨缂╁皬锛屽苟鎻掑叆鍒拌棰戜俊鍙蜂腑浠绘剰鐨勬壂鎻忚涓?姘村钩鍋忕Щ澶勩€?
搴旂敤绋嬪簭鍙互浣跨敤浠ヤ笅 API 鏉ラ€夋嫨瑙嗛淇″彿涓殑涓€涓尯鍩燂紝骞舵煡璇㈤粯璁ゅ尯鍩?浠ュ強纭欢闄愬埗銆?

   CROP API 鐨勫悕绉拌櫧鐒跺姝わ紝浣?VIDIOC_CROPCAP <VIDIOC_CROPCAP>銆?   VIDIOC_G_CROP <VIDIOC_G_CROP> 涓?:ref:`VIDIOC_S_CROP
   <VIDIOC_G_CROP>` ioctl 鏃㈤€傜敤浜庤緭鍏ヨ澶囷紝涔熼€傜敤浜庤緭鍑鸿澶囥€?
缂╂斁闇€瑕佹簮涓庣洰鏍囥€傚湪瑙嗛閲囬泦鎴栧彔鍔狅紙overlay锛夎澶囦笂锛屾簮鏄棰戜俊鍙凤紝
瑁佸壀 ioctl 鍐冲畾瀹為檯琚噰鏍风殑鍖哄煙銆傜洰鏍囧垯鏄簲鐢ㄧ▼搴忚鍙栫殑鍥惧儚锛屾垨鏄?鍙犲姞鍒板浘褰㈠睆骞曚笂鐨勫浘鍍忋€傚叾灏哄锛堝浜庡彔鍔犺繕鍖呮嫭浣嶇疆锛夌敱
VIDIOC_G_FMT <VIDIOC_G_FMT> 涓?VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl 鍗忓晢纭畾銆?
鍦ㄨ棰戣緭鍑鸿澶囦笂锛屾簮鏄簲鐢ㄧ▼搴忎紶鍏ョ殑鍥惧儚锛屽叾灏哄鍚屾牱鐢?VIDIOC_G_FMT <VIDIOC_G_FMT> 涓?VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl 鍗忓晢锛屾垨鑰呭彲鑳藉凡缁忕紪鐮佸湪鍘嬬缉瑙嗛娴佷腑銆傜洰鏍囨槸瑙嗛淇″彿锛岃鍓?ioctl 鍐冲畾鍥惧儚琚彃鍏ョ殑鍖哄煙銆?
鍗充娇璁惧涓嶆敮鎸佺缉鏀炬垨 VIDIOC_G_CROP <VIDIOC_G_CROP> 涓?VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl锛屾簮鐭╁舰涓庣洰鏍囩煩褰篃鏄湁瀹氫箟鐨勩€?鍦ㄨ繖绉嶆儏鍐典笅锛屽叾灏哄锛堜互鍙婇€傜敤鐨勪綅缃級灏嗘槸鍥哄畾鐨勩€?

   鎵€鏈夋敮鎸?CROP 鎴?SELECTION API 鐨勯噰闆嗕笌杈撳嚭璁惧锛屼篃閮芥敮鎸?   VIDIOC_CROPCAP <VIDIOC_CROPCAP> ioctl銆?
## 瑁佸壀缁撴瀯浣?

   :alt:    crop.svg
   :align:  center

   鍥惧儚瑁佸壀銆佹彃鍏ヤ笌缂╂斁

   瑁佸壀銆佹彃鍏ヤ笌缂╂斁鐨勮繃绋?


瀵逛簬閲囬泦璁惧锛屽彲琚噰鏍风殑鍖哄煙鐨勫乏涓婅鍧愭爣銆佸搴︿笌楂樺害鐢?VIDIOC_CROPCAP <VIDIOC_CROPCAP> ioctl 杩斿洖鐨?struct
`v4l2_cropcap` 鐨?`bounds` 瀛愮粨鏋勭粰鍑恒€備负浜嗘敮鎸佸箍娉涚殑
纭欢锛屾湰瑙勮寖骞舵湭瀹氫箟鍘熺偣鎴栧崟浣嶃€備絾鎸夌収鎯緥锛岄┍鍔ㄥ簲鐩稿浜?0H锛堟按骞?鍚屾鑴夊啿鐨勫墠娌匡紝鍙傝 vbi-hsync锛夋按骞冲湴缁熻鏈缉鏀剧殑閲囨牱鐐广€傚湪鍨傜洿
鏂瑰悜涓婏紝浣跨敤绗竴涓満锛坒ield锛夌殑 ITU-R 琛屽彿锛堝弬瑙?525 琛岀殑
ITU R-525 琛岀紪鍙?<vbi-525> 涓?625 琛岀殑 <vbi-625>锛夛紝濡傛灉椹卞姩
鑳藉閲囬泦涓や釜鍦猴紝鍒欎箻浠?2銆?
婧愮煩褰紙鍗冲疄闄呰閲囨牱鐨勫尯鍩燂級鐨勫乏涓婅銆佸搴︿笌楂樺害鐢?struct
`v4l2_crop` 缁欏嚭锛屼娇鐢ㄤ笌 struct `v4l2_cropcap` 鐩稿悓鐨?鍧愭爣绯汇€傚簲鐢ㄧ▼搴忓彲浠ヤ娇鐢?VIDIOC_G_CROP <VIDIOC_G_CROP> 涓?VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl 鏉ヨ幏鍙栧拰璁剧疆杩欎釜鐭╁舰銆傚畠蹇呴』
瀹屽叏钀藉湪閲囬泦杈圭晫涔嬪唴锛屽苟涓旈┍鍔ㄥ彲鑳戒細鏍规嵁纭欢闄愬埗杩涗竴姝ヨ皟鏁存墍璇锋眰鐨?灏哄鍜?鎴栦綅缃€?
姣忎釜閲囬泦璁惧閮芥湁涓€涓粯璁ょ殑婧愮煩褰紝鐢?struct `v4l2_cropcap` 鐨?`defrect` 瀛愮粨鏋勭粰鍑恒€傝鐭╁舰鐨勪腑蹇冨簲涓庤棰戜俊鍙锋湁鏁堝浘鍍忓尯鍩熺殑涓績
瀵归綈锛屽苟瑕嗙洊椹卞姩缂栧啓鑰呮墍璁や负鐨勫畬鏁村浘鍍忋€傞┍鍔ㄥ簲鍦ㄩ娆″姞杞芥椂灏嗘簮鐭╁舰
閲嶇疆涓洪粯璁ゅ€硷紝浣嗕箣鍚庝笉搴斿啀閲嶇疆銆?
瀵逛簬杈撳嚭璁惧锛岃繖浜涚粨鏋勪綋涓?ioctl 浠ョ被浼肩殑鏂瑰紡浣跨敤锛屽畾涔夊浘鍍忓皢琚彃鍏?瑙嗛淇″彿涓殑**鐩爣**鐭╁舰銆?

## 缂╂斁璋冩暣


瑙嗛纭欢鍙兘鍏锋湁鍚勭鍚勬牱鐨勮鍓€佹彃鍏ヤ笌缂╂斁闄愬埗銆傚畠鍙兘鍙兘鏀惧ぇ鎴栧彧鑳?缂╁皬锛屽彧鏀寔绂绘暎鐨勭缉鏀剧郴鏁帮紝鎴栬€呭湪姘村钩涓庡瀭鐩存柟鍚戜笂鍏锋湁涓嶅悓鐨勭缉鏀?鑳藉姏銆備篃鍙兘鏍规湰涓嶆敮鎸佺缉鏀俱€備笌姝ゅ悓鏃讹紝struct `v4l2_crop` 鐭╁舰鍙兘
蹇呴』瀵归綈锛岃€屼笖婧愮煩褰笌鐩爣鐭╁舰閮藉彲鑳芥湁浠绘剰鐨勪笂闄愪笌涓嬮檺灏哄闄愬埗銆傜壒鍒?鏄紝struct `v4l2_crop` 涓渶澶х殑 `width` 涓?`height` 鍙兘灏忎簬
struct `v4l2_cropcap` 鐨?`bounds` 鍖哄煙銆傚洜姝わ紝鍍忓線甯镐竴鏍凤紝椹卞姩
搴旇皟鏁存墍璇锋眰鐨勫弬鏁板苟杩斿洖瀹為檯閫夊畾鐨勫€笺€?
搴旂敤绋嬪簭鍙互鍏堟敼鍙樻簮鐭╁舰鎴栫洰鏍囩煩褰紝鍙栧喅浜庡畠鏇村€惧悜浜庣壒瀹氱殑鍥惧儚灏哄
杩樻槸瑙嗛淇″彿涓殑鏌愪釜鍖哄煙銆傚鏋滈┍鍔ㄥ繀椤诲悓鏃惰皟鏁翠袱鑰呬互婊¤冻纭欢闄愬埗锛屽垯
鏈€鍚庤姹傜殑鐭╁舰搴斾紭鍏堬紝骞朵笖椹卞姩鏈€濂藉幓璋冩暣鍙︿竴涓浉鍙嶇殑鐭╁舰銆備笉杩?VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 涓嶅簲鏀瑰彉椹卞姩鐘舵€侊紝鍥犳鍙皟鏁?鎵€璇锋眰鐨勭煩褰€?
鍋囪瑙嗛閲囬泦璁惧涓婄殑缂╂斁琚檺鍒朵负浠讳竴鏂瑰悜涓?1:1 鎴?2:1 鐨勭郴鏁帮紝涓旂洰鏍?鍥惧儚灏哄蹇呴』鏄?16 脳 16 鍍忕礌鐨勫€嶆暟銆傛簮瑁佸壀鐭╁舰琚缃负榛樿鍊硷紙鍦ㄦ湰渚?涓篃鏄笂闄愶級锛屽嵆鍦ㄥ亸绉?0, 0 澶勭殑 640 脳 400 鍍忕礌銆備竴涓簲鐢ㄧ▼搴忚姹?300 脳 225 鍍忕礌鐨勫浘鍍忓昂瀵革紝鍋囧畾瑙嗛浼氭嵁姝や粠鈥滃畬鏁村浘鍍忊€濈缉灏忋€傞┍鍔ㄥ皢
鍥惧儚灏哄璁剧疆涓烘渶鎺ヨ繎鐨勫彲鐢ㄥ€?304 脳 224锛岀劧鍚庨€夋嫨鏈€鎺ヨ繎璇锋眰灏哄鐨?瑁佸壀鐭╁舰锛屽嵆 608 脳 224锛?24 脳 2:1 浼氳秴杩?400 鐨勪笂闄愶級銆傚亸绉?0, 0 浠嶇劧
鏈夋晥锛屽洜姝や繚鎸佷笉鍙樸€傜粰瀹?VIDIOC_CROPCAP <VIDIOC_CROPCAP> 鎶ュ憡鐨勯粯璁?瑁佸壀鐭╁舰锛屽簲鐢ㄧ▼搴忓彲浠ュ緢瀹规槗鍦版彁鍑哄彟涓€涓亸绉绘潵浣胯鍓煩褰㈠眳涓€?
鐜板湪搴旂敤绋嬪簭鍙兘鍧氭寔瑕佽鐩栦竴涓娇鐢ㄦ洿鎺ヨ繎鍘熷璇锋眰鐨勫浘鍍忓楂樻瘮鐨勫尯鍩燂紝
鍥犳瀹冭姹備竴涓?608 脳 456 鍍忕礌鐨勮鍓煩褰€傚綋鍓嶇殑缂╂斁绯绘暟灏嗚鍓檺鍒朵负
640 脳 384锛屽洜姝ら┍鍔ㄨ繑鍥?608 脳 384 鐨勮鍓昂瀵革紝骞跺皢鍥惧儚灏哄璋冩暣涓烘渶鎺ヨ繎鐨?鍙敤鍊?304 脳 192銆?

## 绀轰緥


婧愮煩褰笌鐩爣鐭╁舰鍦ㄥ叧闂苟閲嶆柊鎵撳紑璁惧鍚庡簲淇濇寔涓嶅彉锛岃繖鏍峰悜璁惧杈撳叆鎴?浠庤澶囪緭鍑烘暟鎹棤闇€鐗规畩鍑嗗鍗冲彲宸ヤ綔銆傛洿鍏堣繘鐨勫簲鐢ㄧ▼搴忓簲鍦ㄥ紑濮?I/O 涔嬪墠
纭繚鍙傛暟鏄悎閫傜殑銆?

   鍦ㄦ帴涓嬫潵鐨勪袱涓ず渚嬩腑锛屽亣瀹氭槸涓€涓棰戦噰闆嗚澶囷紱瀵逛簬鍏朵粬绫诲瀷鐨勮澶囷紝
   璇峰皢 `V4L2_BUF_TYPE_VIDEO_CAPTURE` 鏀逛负鐩稿簲绫诲瀷銆?
## 绀轰緥锛氶噸缃鍓弬鏁?

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));
    crop.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    crop.c = cropcap.defrect;

    /** Ignore if cropping is not supported (EINVAL). **/

    if (-1 == ioctl (fd, VIDIOC_S_CROP, &crop)
	&& errno != EINVAL) {
	perror ("VIDIOC_S_CROP");
	exit (EXIT_FAILURE);
    }


## 绀轰緥锛氱畝鍗曚笅缂╂斁


    struct v4l2_cropcap cropcap;
    struct v4l2_format format;

    reset_cropping_parameters ();

    /** Scale down to 1/4 size of full picture. **/

    memset (&format, 0, sizeof (format)); /** defaults **/

    format.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    format.fmt.pix.width = cropcap.defrect.width >> 1;
    format.fmt.pix.height = cropcap.defrect.height >> 1;
    format.fmt.pix.pixelformat = V4L2_PIX_FMT_YUYV;

    if (-1 == ioctl (fd, VIDIOC_S_FMT, &format)) {
	perror ("VIDIOC_S_FORMAT");
	exit (EXIT_FAILURE);
    }

    /* We could check the actual image size now, the actual scaling factor
       or if the driver can scale at all. */

## 绀轰緥锛氶€夋嫨涓€涓緭鍑哄尯鍩?

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_OUTPUT;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP;, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));

    crop.type = V4L2_BUF_TYPE_VIDEO_OUTPUT;
    crop.c = cropcap.defrect;

    /* Scale the width and height to 50 % of their original size
       and center the output. */

    crop.c.width /= 2;
    crop.c.height /= 2;
    crop.c.left += crop.c.width / 2;
    crop.c.top += crop.c.height / 2;

    /** Ignore if cropping is not supported (EINVAL). **/

    if (-1 == ioctl (fd, VIDIOC_S_CROP, &crop)
	&& errno != EINVAL) {
	perror ("VIDIOC_S_CROP");
	exit (EXIT_FAILURE);
    }

## 绀轰緥锛氬綋鍓嶇缉鏀剧郴鏁颁笌鍍忕礌瀹介珮姣?

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;
    struct v4l2_format format;
    double hscale, vscale;
    double aspect;
    int dwidth, dheight;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));
    crop.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_G_CROP, &crop)) {
	if (errno != EINVAL) {
	    perror ("VIDIOC_G_CROP");
	    exit (EXIT_FAILURE);
	}

	/** Cropping not supported. **/

	crop.c = cropcap.defrect;
    }

    memset (&format, 0, sizeof (format));
    format.fmt.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_G_FMT, &format)) {
	perror ("VIDIOC_G_FMT");
	exit (EXIT_FAILURE);
    }

    /** The scaling applied by the driver. **/

    hscale = format.fmt.pix.width / (double) crop.c.width;
    vscale = format.fmt.pix.height / (double) crop.c.height;

    aspect = cropcap.pixelaspect.numerator /
	 (double) cropcap.pixelaspect.denominator;
    aspect = aspect * hscale / vscale;

    /* Devices following ITU-R BT.601 do not capture
       square pixels. For playback on a computer monitor
       we should scale the images to this size. */

    dwidth = format.fmt.pix.width / aspect;
    dheight = format.fmt.pix.height;
