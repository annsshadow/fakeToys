
######## ioctl VIDIOC_DQEVENT


## 鍚嶇О


VIDIOC_DQEVENT - 鍑洪槦锛圖equeue锛変簨浠?
## 姒傝


`int ioctl(int fd, VIDIOC_DQEVENT, struct v4l2_event *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_event` 鐨勬寚閽堛€?
## 鎻忚堪


浠庝竴涓棰戣澶囧嚭闃熶竴涓簨浠躲€傝繖涓?ioctl 涓嶉渶瑕佽緭鍏ャ€俿truct `v4l2_event` 鐨勬墍鏈夊瓧娈?閮界敱椹卞姩濉厖銆傛枃浠跺彞鏌勮繕浼氭敹鍒板紓甯革紝搴旂敤绋嬪簭鍙互閫氳繃渚嬪浣跨敤 select 绯荤粺璋冪敤鏉ヨ幏鍙?杩欎簺寮傚父銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 浜嬩欢鐨勭被鍨嬶紝鍙傝 event-type銆?    - - union {
      - `u`
    - - struct `v4l2_event_vsync`
      - `vsync`
      - 浜嬩欢 `V4L2_EVENT_VSYNC` 鐨勪簨浠舵暟鎹€?    - - struct `v4l2_event_ctrl`
      - `ctrl`
      - 浜嬩欢 `V4L2_EVENT_CTRL` 鐨勪簨浠舵暟鎹€?    - - struct `v4l2_event_frame_sync`
      - `frame_sync`
      - 浜嬩欢 `V4L2_EVENT_FRAME_SYNC` 鐨勪簨浠舵暟鎹€?    - - struct `v4l2_event_motion_det`
      - `motion_det`
      - 浜嬩欢 V4L2_EVENT_MOTION_DET 鐨勪簨浠舵暟鎹€?    - - struct `v4l2_event_src_change`
      - `src_change`
      - 浜嬩欢 V4L2_EVENT_SOURCE_CHANGE 鐨勪簨浠舵暟鎹€?    - - __u8
      - `data`\ [^64^]
      - 浜嬩欢鏁版嵁銆傜敱浜嬩欢绫诲瀷瀹氫箟銆傚簲褰撲娇鐢ㄨ鑱斿悎浣撲负浜嬩欢瀹氫箟鏄撲簬璁块棶鐨勭被鍨嬨€?    - - }
      -
    - - __u32
      - `pending`
      - 闄ゆ湰浜嬩欢澶栧緟澶勭悊浜嬩欢鐨勬暟閲忋€?    - - __u32
      - `sequence`
      - 浜嬩欢搴忓垪鍙枫€傛瘡鍙戠敓涓€涓凡璁㈤槄鐨勪簨浠讹紝搴忓垪鍙峰氨閫掑銆傚鏋滃簭鍒楀彿涓嶈繛缁紝鎰忓懗鐫€
	浜嬩欢宸茬粡涓㈠け銆?    - - struct timespec
      - `timestamp`
      - 浜嬩欢鏃堕棿鎴炽€傛椂闂存埑鍙栬嚜 `CLOCK_MONOTONIC` 鏃堕挓銆傝鍦?V4L2 涔嬪璁块棶鍚屼竴涓椂閽燂紝
	璇蜂娇鐢?`clock_gettime`銆?    - - u32
      - `id`
      - 涓庝簨浠舵簮鍏宠仈鐨?ID銆傚鏋滀簨浠舵病鏈夊叧鑱旂殑 ID锛堣繖鍙栧喅浜庝簨浠剁被鍨嬶級锛岄偅涔堣繖閲屾槸 0銆?    - - __u32
      - `reserved`\ [^8^]
      - 涓烘湭鏉ョ殑鎵╁睍淇濈暀銆傞┍鍔ㄥ繀椤绘妸璇ユ暟缁勭疆涓洪浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_ALL`
      - 0
      - 鎵€鏈変簨浠躲€俈4L2_EVENT_ALL 浠呭 VIDIOC_UNSUBSCRIBE_EVENT 鏈夋晥锛岀敤浜庝竴娆℃€ч€€璁?	鎵€鏈変簨浠躲€?    - - `V4L2_EVENT_VSYNC`
      - 1
      - 璇ヤ簨浠跺湪鍨傜洿鍚屾锛坴ertical sync锛夋椂瑙﹀彂銆傝浜嬩欢鍏宠仈浜嗕竴涓?struct
	`v4l2_event_vsync`銆?    - - `V4L2_EVENT_EOS`
      - 2
      - 褰撳埌杈炬祦鐨勬湯灏炬椂瑙﹀彂璇ヤ簨浠躲€傝繖閫氬父閰嶅悎 MPEG 瑙ｇ爜鍣ㄤ娇鐢紝鐢ㄦ潵鍚戝簲鐢ㄧ▼搴忔姤鍛?	MPEG 娴佺殑鏈€鍚庝竴閮ㄥ垎宸茬粡琚В鐮併€?    - - `V4L2_EVENT_CTRL`
      - 3
      - 璇ヤ簨浠惰姹?`id` 涓庝綘鎯宠鎺ユ敹浜嬩欢鐨勬帶浠剁殑 ID 鍖归厤銆傚綋鎺т欢鐨勫€兼敼鍙樸€佹寜閽帶浠?	琚寜涓嬶紝鎴栬€呮帶浠剁殑鏍囧織鏀瑰彉鏃讹紝瑙﹀彂璇ヤ簨浠躲€傝浜嬩欢鍏宠仈浜嗕竴涓?struct
	`v4l2_event_ctrl`銆傝缁撴瀯浣撳寘鍚笌 struct
	v4l2_queryctrl <v4l2-queryctrl> 鍜?struct
	`v4l2_control` 鍩烘湰鐩稿悓鐨勪俊鎭€?
	濡傛灉璇ヤ簨浠舵槸鐢变簬璋冪敤 VIDIOC_S_CTRL <VIDIOC_G_CTRL> 鎴?	VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鑰屼骇鐢熺殑锛岄偅涔堣浜嬩欢灏?*涓嶄細**鍙戦€佺粰
	璋冪敤璇?ioctl 鍑芥暟鐨勬枃浠跺彞鏌勩€傝繖閬垮厤浜嗘伡浜虹殑鍙嶉寰幆銆傚鏋滀綘**纭疄**鎯宠鏀跺埌
	璇ヤ簨浠讹紝鍒欒缃?`V4L2_EVENT_SUB_FL_ALLOW_FEEDBACK` 鏍囧織銆?
	杩欑浜嬩欢绫诲瀷鍙互纭繚鍦ㄥ唴閮ㄧ┖闂翠笉瓒炽€佷骇鐢熺殑浜嬩欢澶氫簬鍙绾虫暟閲忔椂涓嶄細涓㈠け淇℃伅銆?	鍦ㄩ偅绉嶆儏鍐典笅锛岀浜屾棫浜嬩欢鐨?struct `v4l2_event_ctrl` 浼氳淇濈暀锛屼絾鍏?`changes`
	瀛楁浼氫笌鏈€鏃т簨浠剁殑 `changes` 瀛楁鍋氭寜浣嶆垨杩愮畻銆?    - - `V4L2_EVENT_FRAME_SYNC`
      - 4
      - 鍦ㄥ抚鐨勬帴鏀朵竴寮€濮嬫椂绔嬪嵆瑙﹀彂銆傝浜嬩欢鍏宠仈浜嗕竴涓?struct
	`v4l2_event_frame_sync`銆?
	濡傛灉纭欢鍦ㄧ紦鍐插尯娆犺浇锛坲nderrun锛夌殑鎯呭喌涓嬮渶瑕佽鍋滄锛屽畠鍙兘灏辨棤娉曠敓鎴愯浜嬩欢銆傚湪
	杩欑鎯呭喌涓嬶紝struct `v4l2_event_frame_sync` 涓殑 `frame_sequence` 瀛楁涓嶄細琚€掑銆?	杩欎細瀵艰嚧涓や釜杩炵画鐨勫抚搴忓垪鍙蜂箣闂存湁 n 鍊嶇殑甯ч棿闅斻€?    - - `V4L2_EVENT_SOURCE_CHANGE`
      - 5
      - 褰撹棰戣澶囧湪杩愯鏃舵娴嬪埌婧愬弬鏁板彉鍖栨椂瑙﹀彂璇ヤ簨浠躲€傚畠鍙互鏄棰戣В鐮佸櫒瑙﹀彂鐨?	杩愯鏃跺垎杈ㄧ巼鍙樺寲锛屾垨鑰呮槸鍙戠敓鍦ㄦ煇涓緭鍏ヨ繛鎺ュ櫒涓婄殑鏍煎紡鍙樺寲銆傝浜嬩欢瑕佹眰 `id` 涓?	浣犳兂瑕佹帴鏀朵簨浠剁殑杈撳叆绱㈠紩锛堢敤浜庤棰戣澶囪妭鐐规椂锛夋垨 pad 绱㈠紩锛堢敤浜庡瓙璁惧鑺傜偣鏃讹級
	鍖归厤銆?
	璇ヤ簨浠跺叧鑱斾簡涓€涓?struct
	`v4l2_event_src_change`銆俙changes` 浣嶅煙琛ㄧず鎵€璁㈤槄鐨?pad 涓婂彂鐢熶簡浠€涔堝彉鍖栥€傚鏋?	鍦ㄥ簲鐢ㄧ▼搴忚兘澶熷嚭闃熶箣鍓嶅彂鐢熶簡澶氫釜浜嬩欢锛岄偅涔?changes 灏嗗叿鏈夋墍鏈夊凡鐢熸垚浜嬩欢鐨勬寜浣?	鎴栧€笺€?    - - `V4L2_EVENT_MOTION_DET`
      - 6
      - 褰撲竴涓垨澶氫釜鍖哄煙鐨勮繍鍔ㄦ娴嬬姸鎬佸彂鐢熷彉鍖栨椂瑙﹀彂銆傝浜嬩欢鍏宠仈浜嗕竴涓?struct
	`v4l2_event_motion_det`銆?    - - `V4L2_EVENT_PRIVATE_START`
      - 0x08000000
      - 椹卞姩绉佹湁浜嬩欢鐨勫熀鍑嗕簨浠跺彿銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `field`
      - 鍗冲皢鍒版潵鐨勫満銆傚弬瑙?enum `v4l2_field`銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `changes`
      - 涓€涓綅鎺╃爜锛岃〃绀哄彂鐢熶簡浠€涔堝彉鍖栥€傚弬瑙?ctrl-changes-flags銆?    - - __u32
      - `type`
      - 鎺т欢鐨勭被鍨嬨€傚弬瑙?enum `v4l2_ctrl_type`銆?    - - union {
      - (anonymous)
    - - __s32
      - `value`
      - 32 浣嶆帶浠剁被鍨嬬殑鎺т欢鐨?32 浣嶅€笺€傚瀛楃涓叉帶浠惰繖鏄?0锛屽洜涓哄瓧绗︿覆鐨勫€兼棤娉曢€氳繃
	VIDIOC_DQEVENT 浼犻€掋€?    - - __s64
      - `value64`
      - 64 浣嶆帶浠剁被鍨嬬殑鎺т欢鐨?64 浣嶅€笺€?    - - }
      -
    - - __u32
      - `flags`
      - 鎺т欢鏍囧織銆傚弬瑙?control-flags銆?    - - __s32
      - `minimum`
      - 鎺т欢鐨勬渶灏忓€笺€傚弬瑙?struct v4l2_queryctrl <v4l2-queryctrl>銆?    - - __s32
      - `maximum`
      - 鎺т欢鐨勬渶澶у€笺€傚弬瑙?struct v4l2_queryctrl <v4l2-queryctrl>銆?    - - __s32
      - `step`
      - 鎺т欢鐨勬杩涘€笺€傚弬瑙?struct v4l2_queryctrl <v4l2-queryctrl>銆?    - - __s32
      - `default_value`
      - 鎺т欢鐨勯粯璁ゅ€笺€傚弬瑙?struct v4l2_queryctrl <v4l2-queryctrl>銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `frame_sequence`
      - 姝ｅ湪鎺ユ敹鐨勫抚鐨勫簭鍒楀彿銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `changes`
      - 涓€涓綅鎺╃爜锛岃〃绀哄彂鐢熶簡浠€涔堝彉鍖栥€傚弬瑙?src-changes-flags銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `flags`
      - 鐩墠鍙湁涓€涓爣蹇楀彲鐢細濡傛灉璁剧疆浜?`V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ`锛岄偅涔?	`frame_sequence` 瀛楁鏈夋晥锛屽惁鍒欏簲褰撳拷鐣ヨ瀛楁銆?    - - __u32
      - `frame_sequence`
      - 姝ｅ湪鎺ユ敹鐨勫抚鐨勫簭鍒楀彿銆備粎褰?`V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ` 鏍囧織琚缃椂鏈夋晥銆?    - - __u32
      - `region_mask`
      - 鎶ュ憡浜嗚繍鍔ㄧ殑鍖哄煙鐨勪綅鎺╃爜銆傝嚦灏戞湁涓€涓尯鍩熴€傚鏋滆瀛楁涓?0锛屽垯鏍规湰鏈娴嬪埌
	杩愬姩銆傚鏋滄病鏈?`V4L2_CID_DETECT_MD_REGION_GRID` 鎺т欢锛堣 detect-controls锛夋潵涓?	杩愬姩妫€娴嬬綉鏍间腑鐨勬瘡涓崟鍏冨垎閰嶄笉鍚岀殑鍖哄煙锛岄偅涔堟墍鏈夊崟鍏冮兘浼氳嚜鍔ㄨ鍒嗛厤鍒伴粯璁?	鍖哄煙 0銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_CTRL_CH_VALUE`
      - 0x0001
      - 璇ユ帶浠朵簨浠舵槸鍥犳帶浠剁殑鍊兼敼鍙樿€岃Е鍙戠殑銆傜壒娈婃儏鍐碉細鏄撳彉锛圴olatile锛夋帶浠朵笉浼氫骇鐢?	璇ヤ簨浠讹紱濡傛灉涓€涓帶浠惰缃簡 `V4L2_CTRL_FLAG_EXECUTE_ON_WRITE` 鏍囧織锛岄偅涔堟棤璁哄叾鍊?	濡備綍锛屼篃浼氬彂閫佽浜嬩欢銆?    - - `V4L2_EVENT_CTRL_CH_FLAGS`
      - 0x0002
      - 璇ユ帶浠朵簨浠舵槸鍥犳帶浠舵爣蹇楁敼鍙樿€岃Е鍙戠殑銆?    - - `V4L2_EVENT_CTRL_CH_RANGE`
      - 0x0004
      - 璇ユ帶浠朵簨浠舵槸鍥犳帶浠剁殑鏈€灏忓€笺€佹渶澶у€笺€佹杩涙垨榛樿鍊兼敼鍙樿€岃Е鍙戠殑銆?    - - `V4L2_EVENT_CTRL_CH_DIMENSIONS`
      - 0x0008
      - 璇ユ帶浠朵簨浠舵槸鍥犳帶浠剁殑缁村害鏀瑰彉鑰岃Е鍙戠殑銆傛敞鎰忕淮搴︾殑鏁伴噺淇濇寔涓嶅彉銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_SRC_CH_RESOLUTION`
      - 0x0001
      - 褰撳湪杈撳叆涓婃娴嬪埌鍒嗚鲸鐜囧彉鍖栨椂瑙﹀彂璇ヤ簨浠躲€傝繖鍙互鏉ヨ嚜杈撳叆杩炴帴鍣紝涔熷彲浠ユ潵鑷?	瑙嗛瑙ｇ爜鍣ㄣ€傚簲鐢ㄧ▼搴忓皢涓嶅緱涓嶆煡璇㈡柊鐨勫垎杈ㄧ巼锛堝鏋滄湁鐨勮瘽锛涗俊鍙蜂篃鍙兘宸茬粡涓㈠け锛夈€?
	瀵逛簬鏈夌姸鎬侊紙stateful锛夎В鐮佸櫒锛岃閬靛惊 decoder 涓殑鎸囧崡銆傝棰戦噰闆嗚澶囧繀椤讳娇鐢?	VIDIOC_QUERY_DV_TIMINGS 鎴?	VIDIOC_QUERYSTD <VIDIOC_QUERYSTD> 鏌ヨ鏂扮殑鏃跺簭銆?
	**閲嶈**锛氬嵆浣挎柊鐨勮棰戞椂搴忕湅璧锋潵涓庢棫鐨勭浉鍚岋紝鏀跺埌璇ヤ簨浠朵篃琛ㄦ槑瑙嗛淇″彿鍑虹幇杩囬棶棰橈紝
	浣犲繀椤诲仠姝㈠苟閲嶆柊鍚姩娴侊紙鍏?VIDIOC_STREAMOFF <VIDIOC_STREAMON>锛屽啀
	VIDIOC_STREAMON <VIDIOC_STREAMON>锛夈€傚師鍥犳槸璁稿瑙嗛閲囬泦璁惧鏃犳硶浠庝俊鍙风殑涓存椂
	涓㈠け涓仮澶嶏紝鍥犳涓轰簡纭欢鑳戒笌瑙嗛淇″彿閲嶆柊鍚屾锛岄渶瑕侀噸鍚祦 I/O銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鏈夋弿杩般€?