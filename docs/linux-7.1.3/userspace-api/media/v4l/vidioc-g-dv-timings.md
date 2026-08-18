

######## ioctl VIDIOC_G_DV_TIMINGS, VIDIOC_S_DV_TIMINGS


## 鍚嶇О


VIDIOC_G_DV_TIMINGS - VIDIOC_S_DV_TIMINGS - VIDIOC_SUBDEV_G_DV_TIMINGS - VIDIOC_SUBDEV_S_DV_TIMINGS - 鑾峰彇鎴栬缃緭鍏?杈撳嚭鐨?DV 鏃跺簭


## 姒傝



`int ioctl(int fd, VIDIOC_G_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_S_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_DV_TIMINGS, struct v4l2_dv_timings *argp)`


## 鍙傛暟



`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_dv_timings` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佷负杈撳叆鎴栬緭鍑鸿缃?DV 鏃跺簭锛屽簲鐢ㄧ▼搴忎娇鐢?VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl锛?鑰岃鑾峰彇褰撳墠鏃跺簭锛屽簲鐢ㄧ▼搴忎娇鐢?VIDIOC_G_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl銆傝缁嗙殑
鏃跺簭淇℃伅浣跨敤 struct `v4l2_dv_timings` 缁撴瀯浣撳～鍏呫€傝繖浜?ioctl 浠ユ寚鍚?struct
`v4l2_dv_timings` 缁撴瀯浣撶殑鎸囬拡浣滀负鍙傛暟銆傚鏋?ioctl 涓嶈鏀寔鎴栨椂搴忓€间笉姝ｇ‘锛岄┍鍔ㄥ皢
杩斿洖 `EINVAL` 閿欒鐮併€?
鍦ㄤ互鍙妯″紡娉ㄥ唽鐨勫瓙璁惧锛坰ubdev锛夎澶囪妭鐐逛笂璋冪敤 `VIDIOC_SUBDEV_S_DV_TIMINGS` 鏄笉鍏佽鐨勩€?姝ゆ椂浼氳繑鍥為敊璇紝errno 鍙橀噺琚涓?`-EPERM`銆?
`linux/v4l2-dv-timings.h` 澶存枃浠跺彲鐢ㄤ簬鑾峰彇 cea861 鍜?vesadmt 鏍囧噯涓悇涓牸寮忕殑鏃跺簭銆傚鏋?褰撳墠鐨勮緭鍏ユ垨杈撳嚭涓嶆敮鎸?DV 鏃跺簭锛堜緥濡?VIDIOC_ENUMINPUT 娌℃湁璁剧疆
`V4L2_IN_CAP_DV_TIMINGS` 鏍囧織锛夛紝鍒欒繑鍥?`ENODATA` 閿欒鐮併€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    璇?ioctl 涓嶈鏀寔锛屾垨鑰?VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> 鍙傛暟涓嶅悎閫傘€?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佹暟瀛楄棰戞椂搴忋€?
EBUSY
    璁惧姝ｅ繖锛屽洜姝ゆ棤娉曟洿鏀规椂搴忋€?
EPERM
    `VIDIOC_SUBDEV_S_DV_TIMINGS` 鍦ㄨ璋冪敤鐨勫彧璇诲瓙璁惧涓婅璋冪敤銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 鏈夋晥瑙嗛鐨勫搴︼紝浠ュ儚绱犱负鍗曚綅銆?    - - __u32
      - `height`
      - 鏈夋晥瑙嗛甯х殑楂樺害锛屼互琛屼负鍗曚綅銆傚洜姝ゅ浜庨殧琛岋紙interlaced锛夋牸寮忥紝
	姣忎釜鍦猴紙field锛夌殑鏈夋晥瑙嗛楂樺害涓?`height`/2銆?    - - __u32
      - `interlaced`
      - 閫愯锛坧rogressive锛宍V4L2_DV_PROGRESSIVE`锛夋垨闅旇
	锛坕nterlaced锛宍V4L2_DV_INTERLACED`锛夈€?    - - __u32
      - `polarities`
      - 涓€涓綅鎺╃爜锛屽畾涔夊悓姝ヤ俊鍙风殑鏋佹€с€備綅 0锛坄V4L2_DV_VSYNC_POS_POL`锛?	瀵瑰簲鍨傜洿鍚屾鏋佹€э紝浣?1锛坄V4L2_DV_HSYNC_POS_POL`锛夊搴旀按骞冲悓姝ユ瀬鎬с€?	鑻ユ煇浣嶈缃綅锛?锛夊垯涓烘鏋佹€э紝琚竻闆讹紙0锛夊垯涓鸿礋鏋佹€с€?    - - __u64
      - `pixelclock`
      - 鍍忕礌鏃堕挓锛屽崟浣嶄负 Hz銆備緥濡?74.25MHz->74250000
    - - __u32
      - `hfrontporch`
      - 姘村钩鍓嶈偐锛坔orizontal front porch锛夛紝浠ュ儚绱犱负鍗曚綅
    - - __u32
      - `hsync`
      - 姘村钩鍚屾闀垮害锛屼互鍍忕礌涓哄崟浣?    - - __u32
      - `hbackporch`
      - 姘村钩鍚庤偐锛坔orizontal back porch锛夛紝浠ュ儚绱犱负鍗曚綅
    - - __u32
      - `vfrontporch`
      - 鍨傜洿鍓嶈偐锛坴ertical front porch锛夛紝浠ヨ涓哄崟浣嶃€傚浜庨殧琛屾牸寮忥紝杩欐寚鐨勬槸
	濂囨暟鍦猴紙aka field 1锛屽嵆鍦?1锛夈€?    - - __u32
      - `vsync`
      - 鍨傜洿鍚屾闀垮害锛屼互琛屼负鍗曚綅銆傚浜庨殧琛屾牸寮忥紝杩欐寚鐨勬槸濂囨暟鍦猴紙aka field 1锛夈€?    - - __u32
      - `vbackporch`
      - 鍨傜洿鍚庤偐锛坴ertical back porch锛夛紝浠ヨ涓哄崟浣嶃€傚浜庨殧琛屾牸寮忥紝杩欐寚鐨勬槸
	濂囨暟鍦猴紙aka field 1锛夈€?    - - __u32
      - `il_vfrontporch`
      - 闅旇鍦烘牸寮忎腑鍋舵暟鍦猴紙aka field 2锛屽嵆鍦?2锛夌殑鍨傜洿鍓嶈偐锛屼互琛屼负鍗曚綅銆?	瀵逛簬閫愯鏍煎紡蹇呴』涓?0銆?    - - __u32
      - `il_vsync`
      - 闅旇鍦烘牸寮忎腑鍋舵暟鍦猴紙aka field 2锛夌殑鍨傜洿鍚屾闀垮害锛屼互琛屼负涓哄崟浣嶃€?	瀵逛簬閫愯鏍煎紡蹇呴』涓?0銆?    - - __u32
      - `il_vbackporch`
      - 闅旇鍦烘牸寮忎腑鍋舵暟鍦猴紙aka field 2锛夌殑鍨傜洿鍚庤偐锛屼互琛屼负鍗曚綅銆?	瀵逛簬閫愯鏍煎紡蹇呴』涓?0銆?    - - __u32
      - `standards`
      - 璇ユ牸寮忔墍灞炵殑瑙嗛鏍囧噯锛堝彲浠ュ涓級銆傝繖鐢遍┍鍔ㄥ～鍏呫€傚簲鐢ㄧ▼搴忓繀椤诲皢
	鍏惰涓?0銆傛爣鍑嗗垪琛ㄨ dv-bt-standards銆?    - - __u32
      - `flags`
      - 鎻愪緵鍏充簬璇ユ牸寮忔洿澶氫俊鎭殑鑻ュ共鏍囧織銆傚悇鏍囧織鐨勮鏄庤 dv-bt-flags銆?    - - struct `v4l2_fract`
      - `picture_aspect`
      - 褰撳儚绱犱笉鏄鏂瑰舰鏃剁殑鐢婚潰瀹介珮姣斻€備粎褰?`V4L2_DV_FL_HAS_PICTURE_ASPECT`
	鏍囧織琚缃椂鏈夋晥銆?    - - __u8
      - `cea861_vic`
      - 渚濇嵁 CEA-861 鏍囧噯鐨勮棰戣瘑鍒爜锛圴ideo Identification Code锛夈€?	浠呭綋 `V4L2_DV_FL_HAS_CEA861_VIC` 鏍囧織琚缃椂鏈夋晥銆?    - - __u8
      - `hdmi_vic`
      - 渚濇嵁 HDMI 鏍囧噯鐨勮棰戣瘑鍒爜銆備粎褰?`V4L2_DV_FL_HAS_HDMI_VIC` 鏍囧織
	琚缃椂鏈夋晥銆?    - - __u8
      - `reserved[^46^]`
      - 淇濈暀渚涘皢鏉ユ墿灞曚娇鐢ㄣ€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - DV 鏃跺簭鐨勭被鍨嬶紝瑙?dv-timing-types 涓殑鍒楄〃銆?    - - union {
      - (anonymous)
    - - struct `v4l2_bt_timings`
      - `bt`
      - 鐢?BT.656/1120 瑙勮寖瀹氫箟鐨勬椂搴?    - - __u32
      - `reserved`\ [^32^]
      -
    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - Timing type
      - value
      - Description
#     * -

      -
    - - `V4L2_DV_BT_656_1120`
      - 0
      - BT.656/1120 鏃跺簭



    :header-rows:  0
    :stub-columns: 0

    - - Timing standard
      - Description
    - - `V4L2_DV_BT_STD_CEA861`
      - 鏃跺簭閬靛惊 CEA-861 鏁板瓧鐢佃锛圖igital TV锛塒rofile 鏍囧噯
    - - `V4L2_DV_BT_STD_DMT`
      - 鏃跺簭閬靛惊 VESA 绂绘暎鐩戣鍣ㄦ椂搴忥紙Discrete Monitor Timings锛夋爣鍑?    - - `V4L2_DV_BT_STD_CVT`
      - 鏃跺簭閬靛惊 VESA 鍗忚皟瑙嗛鏃跺簭锛圕oordinated Video Timings锛夋爣鍑?    - - `V4L2_DV_BT_STD_GTF`
      - 鏃跺簭閬靛惊 VESA 閫氱敤鏃跺簭鍏紡锛圙eneralized Timings Formula锛夋爣鍑?    - - `V4L2_DV_BT_STD_SDI`
      - 鏃跺簭閬靛惊 SDI 鏃跺簭鏍囧噯銆?	璇ユ牸寮忓畬鍏ㄦ病鏈夋按骞冲悓姝?鑲╋紙syncs/porches锛夈€?	鎬荤殑娑堥殣锛坆lanking锛夋椂搴忓繀椤诲彧璁剧疆鍦?hsync 鎴?vsync 瀛楁涓€?


    :header-rows:  0
    :stub-columns: 0

    - - Flag
      - Description
    - - `V4L2_DV_FL_REDUCED_BLANKING`
      - CVT/GTF 涓撶敤锛氭椂搴忎娇鐢ㄧ缉鍑忔秷闅愶紙reduced blanking锛孋VT锛夋垨
	鈥滄绾?GTF鈥濓紙'Secondary GTF'锛夋洸绾匡紙GTF锛夈€備袱绉嶆儏鍐典笅姘村钩
	鍜?鎴栧瀭鐩存秷闅愰棿闅旈兘琚缉鍑忥紝浠庤€屽厑璁稿湪鐩稿悓鐨勫甫瀹戒笅鑾峰緱鏇撮珮鐨勫垎杈ㄧ巼銆?	杩欐槸涓€涓彧璇绘爣蹇楋紝搴旂敤绋嬪簭涓嶅緱璁剧疆瀹冦€?    - - `V4L2_DV_FL_CAN_REDUCE_FPS`
      - CEA-861 涓撶敤锛氶拡瀵瑰抚鐜囦负 6 鐨勫€嶆暟鐨?CEA-861 鏍煎紡璁剧疆銆傝繖浜涙牸寮忓彲浠?	閫夋嫨鎬у湴浠?1 / 1.001 鐨勯€熷害鎾斁锛屼互鍏煎浣跨敤 29.97 甯?绉掑抚鐜囩殑
	鍩轰簬 60 Hz 鐨勬爣鍑嗭紝濡?NTSC 鍜?PAL-M銆傚鏋滃彂閫佺鏃犳硶浜х敓杩欐牱鐨勯鐜囷紝
	璇ユ爣蹇椾篃浼氳娓呴浂銆傝繖鏄竴涓彧璇绘爣蹇楋紝搴旂敤绋嬪簭涓嶅緱璁剧疆瀹冦€?    - - `V4L2_DV_FL_REDUCED_FPS`
      - CEA-861 涓撶敤锛氫粎瀵硅缃簡 `V4L2_DV_FL_CAN_DETECT_REDUCED_FPS` 鐨?	瑙嗛鍙戦€佺鎴栬棰戞帴鏀剁鏈夋晥銆傚惁鍒欒鏍囧織浼氳娓呴浂銆傚畠涔熶粎瀵硅缃簡
	`V4L2_DV_FL_CAN_REDUCE_FPS` 鏍囧織鐨勬牸寮忔湁鏁堬紝瀵逛簬鍏朵粬鏍煎紡璇ユ爣蹇?	浼氳椹卞姩娓呴浂銆?
	濡傛灉搴旂敤绋嬪簭涓哄彂閫佺璁剧疆璇ユ爣蹇楋紝閭ｄ箞鐢ㄤ簬璁剧疆鍙戦€佺鐨勫儚绱犳椂閽熶細闄や互
	1.001锛屼互鍏煎 NTSC 甯х巼銆傚鏋滃彂閫佺鏃犳硶浜х敓杩欐牱鐨勯鐜囷紝璇ユ爣蹇椾細琚竻闆躲€?
	濡傛灉瑙嗛鎺ユ敹绔娴嬪埌璇ユ牸寮忎娇鐢ㄤ簡缂╁噺鐨勫抚鐜囷紝鍒欎細璁剧疆璇ユ爣蹇椾互鍚戝簲鐢ㄧ▼搴?	鍙戝嚭淇″彿銆?    - - `V4L2_DV_FL_HALF_LINE`
      - 闅旇鏍煎紡涓撶敤锛氳嫢璁剧疆锛屽垯鍦?1锛坅ka 濂囨暟鍦猴級鐨勫瀭鐩村墠鑲╁疄闄呬笂澶氬崐涓闀匡紝
	鑰屽満 2锛坅ka 鍋舵暟鍦猴級鐨勫瀭鐩村悗鑲╁疄闄呬笂灏戝崐涓闀匡紝鍥犳姣忎釜鍦烘伆濂藉叿鏈?	鐩稿悓鏁伴噺鐨勫崐琛屻€傝兘鍚︽娴嬫垨浣跨敤鍗婅鍙栧喅浜庣‖浠躲€?    - - `V4L2_DV_FL_IS_CE_VIDEO`
      - 鑻ヨ缃紝鍒欒繖鏄竴涓秷璐圭數瀛愶紙Consumer Electronics锛孋E锛夎棰戞牸寮忋€傝繖绫绘牸寮?	涓庡叾浠栨牸寮忥紙閫氬父绉颁负 IT 鏍煎紡锛夌殑涓嶅悓涔嬪鍦ㄤ簬锛氬鏋滀娇鐢?R'G'B' 缂栫爜锛?	榛樿鎯呭喌涓?R'G'B' 鍊间娇鐢ㄥ彈闄愯寖鍥达紙鍗?16-235锛夛紝鑰岄潪鍏ㄨ寖鍥达紙鍗?0-255锛夈€?	CEA-861 涓畾涔夌殑鎵€鏈夋牸寮忥紙640x480p59.94 鏍煎紡闄ゅ锛夐兘鏄?CE 鏍煎紡銆?    - - `V4L2_DV_FL_FIRST_FIELD_EXTRA_LINE`
      - 鏌愪簺鏍煎紡锛堝 SMPTE-125M锛夊叿鏈夊鏁版€婚珮搴︾殑闅旇淇″彿銆傚浜庤繖浜涙牸寮忥紝濡傛灉
	璁剧疆浜嗚鏍囧織锛屽垯澶氫綑鐨勮灞炰簬绗竴涓満锛涘惁鍒欏睘浜庣浜屼釜鍦恒€?    - - `V4L2_DV_FL_HAS_PICTURE_ASPECT`
      - 鑻ヨ缃紝鍒?picture_aspect 瀛楁鏈夋晥銆傚惁鍒欏亣瀹氬儚绱犱负姝ｆ柟褰紝鍥犳鐢婚潰瀹介珮姣?	涓庡楂樻瘮鐩稿悓銆?    - - `V4L2_DV_FL_HAS_CEA861_VIC`
      - 鑻ヨ缃紝鍒?cea861_vic 瀛楁鏈夋晥锛屽苟鍖呭惈鎸夌収 CEA-861 鏍囧噯鐨勮棰戣瘑鍒爜銆?    - - `V4L2_DV_FL_HAS_HDMI_VIC`
      - 鑻ヨ缃紝鍒?hdmi_vic 瀛楁鏈夋晥锛屽苟鍖呭惈鎸夌収 HDMI 鏍囧噯鐨勮棰戣瘑鍒爜
	锛圚DMI Vendor Specific InfoFrame锛夈€?    - - `V4L2_DV_FL_CAN_DETECT_REDUCED_FPS`
      - CEA-861 涓撶敤锛氫粎瀵硅棰戞帴鏀剁鏈夋晥锛岃鏍囧織鐢卞彂閫佺娓呴浂銆傝嫢璁剧疆锛屽垯纭欢
	鑳藉妫€娴嬪父瑙勫抚鐜囦笌鎸?1000/1001 缂╁噺鐨勫抚鐜囦箣闂寸殑宸紓銆備緥濡傦細60 涓?59.94 Hz锛?	30 涓?29.97 Hz锛屾垨 24 涓?23.976 Hz銆?