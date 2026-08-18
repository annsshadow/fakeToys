

######## V4L 涓?V4L2 鐨勫尯鍒?

Video For Linux锛圴4L锛堿PI 鏈€鏃╁湪 Linux 2.1 涓紩鍏ワ紝鐢ㄤ簬缁熶竴骞跺彇浠ｆ棭浜涘勾鐢遍┍鍔ㄥ紑鍙戣€呭悇鑷紑鍙戠殑澶氱鐢佃鍜屾敹闊虫満璁惧鐩稿叧鎺ュ彛銆備粠 Linux 2.5 寮€濮嬶紝鏀硅繘鐨?V4L2 API 鍙栦唬浜?V4L API銆傚唴鏍镐腑宸茬Щ闄ゅ鏃?V4L 璋冪敤鐨勬敮鎸侊紝浣?libv4l 搴撴敮鎸佸皢 V4L API 绯荤粺璋冪敤杞崲涓?V4L2 璋冪敤銆?
## 璁惧鐨勬墦寮€涓庡叧闂?

鍑轰簬鍏煎鎬ц€冭檻锛孷4L2 寤鸿鐢ㄤ簬瑙嗛鎹曡幏銆佸彔鍔犮€佹敹闊虫満鍜屽師濮?VBI 鎹曡幏璁惧鐨勫瓧绗﹁澶囨枃浠跺悕锛屼笌 V4L 浣跨敤鐨勬枃浠跺悕淇濇寔涓€鑷淬€傚畠浠垪鍦?devices 浠ュ強涓嬫枃 v4l-dev 涓€?
鍥炬枃鐢佃锛坱eletext锛夎澶囷紙娆¤澶囧彿鑼冨洿 192-223锛夊湪 V4L2 涓凡琚Щ闄わ紝涓嶅啀瀛樺湪銆傜洰鍓嶅凡鏃犲鐞嗙函鍥炬枃鐢佃鐨勭‖浠讹紝鍙栬€屼唬涔嬩娇鐢ㄥ師濮嬫垨鍒囩墖 VBI銆?
V4L 鐨?`videodev` 妯″潡浼氭牴鎹敞鍐岀殑璁惧绫诲瀷锛屾寜鍔犺浇椤哄簭鑷姩涓洪┍鍔ㄥ垎閰嶆璁惧鍙枫€傛垜浠缓璁?V4L2 椹卞姩榛樿浣跨敤鐩稿悓鐨勭紪鍙锋敞鍐岃澶囷紝浣嗙郴缁熺鐞嗗憳鍙互閫氳繃椹卞姩妯″潡閫夐」鍒嗛厤浠绘剰娆¤澶囧彿銆備富璁惧鍙蜂粛涓?81銆?

    :header-rows:  1
    :stub-columns: 0

    - - 璁惧绫诲瀷
      - 鏂囦欢鍚?      - 娆¤澶囧彿
    - - 瑙嗛鎹曡幏涓庡彔鍔?      - `/dev/video` and `/dev/bttv0`\  [#f1]_, `/dev/video0` to
	`/dev/video63`
      - 0-63
    - - 鏀堕煶鏈烘帴鏀跺櫒
      - `/dev/radio`\  [#f2]_, `/dev/radio0` to `/dev/radio63`
      - 64-127
    - - 鍘熷 VBI 鎹曡幏
      - `/dev/vbi`, `/dev/vbi0` to `/dev/vbi31`
      - 224-255

V4L 绂佹锛堟垨鏇剧粡绂佹锛夊娆℃墦寮€鍚屼竴璁惧鏂囦欢銆俈4L2 椹卞姩**鍙兘**鏀寔澶氭鎵撳紑锛岃瑙?open 浜嗚В缁嗚妭涓庡悗鏋溿€?
V4L 椹卞姩浼氫互 `EINVAL` 閿欒鐮佸搷搴?V4L2 鐨?ioctl銆?
## 鏌ヨ鑳藉姏


V4L 鐨?`VIDIOCGCAP` ioctl 绛変环浜?V4L2 鐨?VIDIOC_QUERYCAP銆?
struct `video_capability` 涓殑 `name` 瀛楁鍦?struct `v4l2_capability` 涓彉涓?`card`锛宍type` 琚?`capabilities` 鍙栦唬銆傛敞鎰?V4L2 骞朵笉浼氬姝ゅ尯鍒嗚澶囩被鍨嬶紝鏇村噯纭湴璇达紝搴斿皢鍏惰涓烘敮鎸佷竴缁勭浉鍏冲姛鑳斤紙濡傝棰戞崟鑾枫€佽棰戝彔鍔犲拰 VBI 鎹曡幏锛夌殑鍩烘湰瑙嗛杈撳叆銆佽棰戣緭鍑哄拰鏀堕煶鏈鸿澶囥€傚弬瑙?open 浜嗚В浠嬬粛銆?

   \small



    :header-rows:  1
    :stub-columns: 0

    - - struct `video_capability` `type`
      - struct `v4l2_capability`
	`capabilities` flags
      - 鐢ㄩ€?    - - `VID_TYPE_CAPTURE`
      - `V4L2_CAP_VIDEO_CAPTURE`
      - 鏀寔瑙嗛鎹曡幏锛坴ideo capture锛夋帴鍙ｃ€?    - - `VID_TYPE_TUNER`
      - `V4L2_CAP_TUNER`
      - 璁惧甯︽湁璋冭皭鍣ㄦ垨璋冨埗鍣紙tuner/modulator锛夈€?    - - `VID_TYPE_TELETEXT`
      - `V4L2_CAP_VBI_CAPTURE`
      - 鏀寔鍘熷 VBI 鎹曡幏锛坮aw VBI锛夋帴鍙ｃ€?    - - `VID_TYPE_OVERLAY`
      - `V4L2_CAP_VIDEO_OVERLAY`
      - 鏀寔瑙嗛鍙犲姞锛坴ideo overlay锛夋帴鍙ｃ€?    - - `VID_TYPE_CHROMAKEY`
      - `V4L2_FBUF_CAP_CHROMAKEY` in field `capability` of struct
	`v4l2_framebuffer`
      - 鏄惁鏀寔鑹查敭锛坈hromakey锛夊彔鍔犮€傚叧浜庡彔鍔犵殑鏇村淇℃伅璇峰弬瑙?overlay銆?    - - `VID_TYPE_CLIPPING`
      - `V4L2_FBUF_CAP_LIST_CLIPPING` and
	`V4L2_FBUF_CAP_BITMAP_CLIPPING` in field `capability` of
	struct `v4l2_framebuffer`
      - 鏄惁鏀寔瀵瑰彔鍔犲浘鍍忚繘琛岃鍓紙clipping锛夛紝鍙傝 overlay銆?    - - `VID_TYPE_FRAMERAM`
      - `V4L2_FBUF_CAP_EXTERNOVERLAY` **not set** in field `capability`
	of struct `v4l2_framebuffer`
      - 鍙犲姞鏄惁瑕嗙洊甯х紦鍐插唴瀛橈紝鍙傝 overlay銆?    - - `VID_TYPE_SCALES`
      - `-`
      - 璇ユ爣蹇楄〃绀虹‖浠舵槸鍚﹁兘澶熺缉鏀惧浘鍍忋€俈4L2 API 閫氳繃鍒嗗埆浣跨敤 VIDIOC_S_CROP <VIDIOC_G_CROP> 鍜?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 璁剧疆瑁佸壀灏哄鍜屽浘鍍忓ぇ灏忔潵闅愬惈缂╂斁绯绘暟銆傞┍鍔ㄤ細杩斿洖灏藉彲鑳芥帴杩戠殑灏哄銆傚叧浜庤鍓拰缂╂斁鐨勬洿澶氫俊鎭鍙傝 crop銆?    - - `VID_TYPE_MONOCHROME`
      - `-`
      - 搴旂敤绋嬪簭鍙互閫氳繃 VIDIOC_ENUM_FMT ioctl 鏋氫妇鏀寔鐨勭殑鍥惧儚鏍煎紡锛屼互纭畾璁惧鏄惁浠呮敮鎸佺伆搴︽崟鑾枫€傚叧浜庡浘鍍忔牸寮忕殑鏇村淇℃伅璇峰弬瑙?pixfmt銆?    - - `VID_TYPE_SUBCAPTURE`
      - `-`
      - 搴旂敤绋嬪簭鍙互璋冪敤 VIDIOC_G_CROP <VIDIOC_G_CROP> ioctl 鏉ョ‘瀹氳澶囨槸鍚︽敮鎸佹崟鑾峰畬鏁寸敾闈㈢殑涓€閮ㄥ垎锛堝嵆 V4L2 涓殑鈥渃ropping鈥濓級銆傚鏋滀笉鏀寔锛岃 ioctl 浼氳繑鍥?`EINVAL` 閿欒鐮併€傚叧浜庤鍓拰缂╂斁鐨勬洿澶氫俊鎭鍙傝 crop銆?    - - `VID_TYPE_MPEG_DECODER`
      - `-`
      - 搴旂敤绋嬪簭鍙互閫氳繃 VIDIOC_ENUM_FMT ioctl 鏋氫妇鏀寔鐨勭殑鍥惧儚鏍煎紡锛屼互纭畾璁惧鏄惁鏀寔 MPEG 娴併€?    - - `VID_TYPE_MPEG_ENCODER`
      - `-`
      - 鍙傝涓婃枃銆?    - - `VID_TYPE_MJPEG_DECODER`
      - `-`
      - 鍙傝涓婃枃銆?    - - `VID_TYPE_MJPEG_ENCODER`
      - `-`
      - 鍙傝涓婃枃銆?

   \normalsize

`audios` 瀛楁琚?`capabilities` 鏍囧織 `V4L2_CAP_AUDIO` 鍙栦唬锛岀敤浜庢寚绀鸿澶?*鏄惁**鍏锋湁浠讳綍闊抽杈撳叆鎴栬緭鍑恒€傝纭畾鍏舵暟閲忥紝搴旂敤绋嬪簭鍙互浣跨敤 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl 鏋氫妇闊抽杈撳叆銆傞煶棰戠浉鍏崇殑 ioctl 鍦?audio 涓湁璇存槑銆?
`maxwidth`銆乣maxheight`銆乣minwidth` 鍜?`minheight` 瀛楁宸茶绉婚櫎銆備娇鐢ㄦ湡鏈涚殑灏哄璋冪敤 VIDIOC_S_FMT <VIDIOC_G_FMT> 鎴?VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 鏃讹紝浼氱患鍚堣€冭檻褰撳墠瑙嗛鏍囧噯銆佽鍓拰缂╂斁闄愬埗锛岃繑鍥炲敖鍙兘鎺ヨ繎鐨勫昂瀵搞€?
## 瑙嗛婧?

V4L 浣跨敤 struct `video_channel` 鎻愪緵 `VIDIOCGCHAN` 鍜?`VIDIOCSCHAN` ioctl锛岀敤浜庢灇涓?V4L 璁惧鐨勮棰戣緭鍏ャ€傜瓑浠风殑 V4L2 ioctl 鏄?VIDIOC_ENUMINPUT銆乂IDIOC_G_INPUT <VIDIOC_G_INPUT> 鍜?VIDIOC_S_INPUT <VIDIOC_G_INPUT>锛屽畠浠娇鐢?struct `v4l2_input`锛屾濡?video 涓墍杩般€?
鐢ㄤ簬璁℃暟杈撳叆鐨?`channel` 瀛楁琚噸鍛藉悕涓?`index`锛岃棰戣緭鍏ョ被鍨嬬殑閲嶅懡鍚嶅涓嬶細


    :header-rows:  1
    :stub-columns: 0

    - - struct `video_channel` `type`
      - struct `v4l2_input` `type`
    - - `VIDEO_TYPE_TV`
      - `V4L2_INPUT_TYPE_TUNER`
    - - `VIDEO_TYPE_CAMERA`
      - `V4L2_INPUT_TYPE_CAMERA`

涓庤〃绀烘杈撳叆璋冭皭鍣ㄦ暟閲忕殑 `tuners` 瀛楁涓嶅悓锛孷4L2 鍋囪姣忎釜瑙嗛杈撳叆鏈€澶氳繛鎺ヤ竴涓皟璋愬櫒銆備絾涓€涓皟璋愬櫒鍙互鏈夊涓緭鍏ワ紙鍗?RF 杩炴帴鍣級锛屼笖涓€涓澶囧彲浠ユ湁澶氫釜璋冭皭鍣ㄣ€備笌璇ヨ緭鍏ュ叧鑱旂殑璋冭皭鍣紙濡傛湁锛夌殑绱㈠紩鍙峰瓨鍌ㄥ湪 struct `v4l2_input` 鐨?`tuner` 瀛楁涓€傝皟璋愬櫒鐨勬灇涓惧湪 tuner 涓璁恒€?
鍐椾綑鐨?`VIDEO_VC_TUNER` 鏍囧織琚Щ闄ゃ€備笌璋冭皭鍣ㄥ叧鑱旂殑瑙嗛杈撳叆绫诲瀷涓?`V4L2_INPUT_TYPE_TUNER`銆俙VIDEO_VC_AUDIO` 鏍囧織琚?`audioset` 瀛楁鍙栦唬銆俈4L2 鏀寔鏈€澶?32 涓煶棰戣緭鍏ョ殑璁惧銆俙audioset` 瀛楁涓瘡涓缃綅鐨勪綅浠ｈ〃璇ヨ棰戣緭鍏ユ墍缁勫悎鐨勪竴涓煶棰戣緭鍏ャ€傚叧浜庨煶棰戣緭鍏ュ強鍏跺垏鎹㈡柟寮忕殑淇℃伅璇峰弬瑙?audio銆?
鎻忚堪鎵€鏀寔瑙嗛鏍囧噯鐨?`norm` 瀛楁琚?`std` 鍙栦唬銆俈4L 瑙勮寖鎻愬埌杩?`VIDEO_VC_NORM` 鏍囧織锛岀敤浜庤〃绀烘爣鍑嗘槸鍚﹀彲鏇存敼銆傝鏍囧織涓?`norm` 瀛楁鏄悗鏉ヤ竴璧峰姞鍏ョ殑锛岀幇宸茶绉婚櫎銆俈4L2 瀵硅棰戞爣鍑嗛噰鐢ㄤ簡绫讳技浣嗘洿鍏ㄩ潰鐨勬柟妗堬紝璇﹁ standard銆?
## 璋冭皭


V4L 鐨?`VIDIOCGTUNER` 鍜?`VIDIOCSTUNER` ioctl 浠ュ強 struct `video_tuner` 鍙敤浜庢灇涓?V4L 鐢佃鎴栨敹闊虫満璁惧鐨勮皟璋愬櫒銆傜瓑浠风殑 V4L2 ioctl 鏄?VIDIOC_G_TUNER <VIDIOC_G_TUNER> 鍜?VIDIOC_S_TUNER <VIDIOC_G_TUNER>锛屼娇鐢?struct `v4l2_tuner`銆傝皟璋愬櫒鐩稿叧璇存槑瑙?tuner銆?
鐢ㄤ簬璁℃暟璋冭皭鍣ㄧ殑 `tuner` 瀛楁琚噸鍛藉悕涓?`index`銆俙name`銆乣rangelow` 鍜?`rangehigh` 瀛楁淇濇寔涓嶅彉銆?
琛ㄧず鎵€鏀寔瑙嗛鏍囧噯鐨?`VIDEO_TUNER_PAL`銆乣VIDEO_TUNER_NTSC` 鍜?`VIDEO_TUNER_SECAM` 鏍囧織宸茶绉婚櫎銆傝淇℃伅鐜板湪鍖呭惈鍦ㄥ叧鑱旂殑 struct `v4l2_input` 涓€傚浜庤〃绀鸿棰戞爣鍑嗘槸鍚﹀彲鍒囨崲鐨?`VIDEO_TUNER_NORM` 鏍囧織锛岀洰鍓嶆病鏈夋浛浠ｉ」銆傜敤浜庨€夋嫨涓嶅悓瑙嗛鏍囧噯鐨?`mode` 瀛楁琚竴鏁村鏂扮殑 ioctl 鍜岀粨鏋勫彇浠ｏ紝璇﹁ standard銆傚€煎緱涓€鎻愮殑鏄紝鐢变簬 BTTV 椹卞姩搴旂敤骞挎硾锛岄櫎甯歌鐨?`VIDEO_MODE_PAL` (0)銆乣VIDEO_MODE_NTSC`銆乣VIDEO_MODE_SECAM` 鍜?`VIDEO_MODE_AUTO` (3) 澶栵紝瀹冭繕鏀寔 N/PAL Argentina銆丮/PAL銆丯/PAL 鍜?NTSC Japan锛堢紪鍙蜂负 3-6锛屽師鏂囧姝わ級銆?
琛ㄧず绔嬩綋澹版帴鏀剁殑 `VIDEO_TUNER_STEREO_ON` 鏍囧織鍦?`rxsubchans` 瀛楁涓彉涓?`V4L2_TUNER_SUB_STEREO`銆傝瀛楁杩樺厑璁告娴嬪崟澹伴亾鍜屽弻璇煶棰戯紝璇﹁ struct `v4l2_tuner` 鐨勫畾涔夈€傜洰鍓嶅浜?`VIDEO_TUNER_RDS_ON` 鍜?`VIDEO_TUNER_MBS_ON` 鏍囧織灏氭棤鏇夸唬椤广€?
`VIDEO_TUNER_LOW` 鏍囧織鍦?struct `v4l2_tuner` 鐨?`capability` 瀛楁涓閲嶅懡鍚嶄负 `V4L2_TUNER_CAP_LOW`銆?
鐢ㄤ簬鏇存敼璋冭皭鍣ㄩ鐜囩殑 `VIDIOCGFREQ` 鍜?`VIDIOCSFREQ` ioctl 琚噸鍛藉悕涓?VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 鍜?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>銆傚畠浠帴鍙楁寚鍚?struct `v4l2_frequency` 鐨勬寚閽堬紝鑰岄潪 unsigned long 鏁存暟銆?

## 鍥惧儚灞炴€?

V4L2 娌℃湁涓?`VIDIOCGPICT` 鍜?`VIDIOCSPICT` ioctl 浠ュ強 struct `video_picture` 绛変环鐨勫唴瀹广€備互涓嬪瓧娈佃鍙€氳繃 VIDIOC_QUERYCTRL銆乂IDIOC_G_CTRL <VIDIOC_G_CTRL> 鍜?VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 璁块棶鐨?V4L2 鎺т欢鍙栦唬锛?

    :header-rows:  1
    :stub-columns: 0

    - - struct `video_picture`
      - V4L2 Control ID
    - - `brightness`
      - `V4L2_CID_BRIGHTNESS`
    - - `hue`
      - `V4L2_CID_HUE`
    - - `colour`
      - `V4L2_CID_SATURATION`
    - - `contrast`
      - `V4L2_CID_CONTRAST`
    - - `whiteness`
      - `V4L2_CID_WHITENESS`

V4L 鐨勫浘鍍忔帶浠跺亣瀹氬彇鍊艰寖鍥翠负 0 鍒?65535锛屾病鏈夌壒瀹氱殑澶嶄綅鍊笺€俈4L2 API 鍏佽浠绘剰鐨勯檺鍒跺拰榛樿鍊硷紝鍙€氳繃 VIDIOC_QUERYCTRL ioctl 鏌ヨ銆傚叧浜庢帶浠剁殑涓€鑸俊鎭鍙傝 control銆?
瑙嗛鍥惧儚鐨?`depth`锛堟瘡鍍忕礌骞冲潎浣嶆暟锛夌敱鎵€閫夊浘鍍忔牸寮忛殣鍚€俈4L2 涓嶆樉寮忔彁渚涙绫讳俊鎭紝瀹冨亣璁捐兘璇嗗埆璇ユ牸寮忕殑搴旂敤绋嬪簭浜嗚В鍥惧儚娣卞害锛岃€屽叾浠栧簲鐢ㄧ▼搴忓垯鏃犻渶鐭ラ亾銆俙palette` 瀛楁绉诲叆浜?struct `v4l2_pix_format`锛?

    :header-rows:  1
    :stub-columns: 0

    - - struct `video_picture` `palette`
      - struct `v4l2_pix_format` `pixfmt`
    - - `VIDEO_PALETTE_GREY`
      - V4L2_PIX_FMT_GREY <V4L2-PIX-FMT-GREY>
    - - `VIDEO_PALETTE_HI240`
      - V4L2_PIX_FMT_HI240 <pixfmt-reserved> [#f3]_
    - - `VIDEO_PALETTE_RGB565`
      - V4L2_PIX_FMT_RGB565 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB555`
      - V4L2_PIX_FMT_RGB555 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB24`
      - V4L2_PIX_FMT_BGR24 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB32`
      - V4L2_PIX_FMT_BGR32 <pixfmt-rgb> [#f4]_
    - - `VIDEO_PALETTE_YUV422`
      - V4L2_PIX_FMT_YUYV <V4L2-PIX-FMT-YUYV>
    - - `VIDEO_PALETTE_YUYV`\  [#f5]_
      - V4L2_PIX_FMT_YUYV <V4L2-PIX-FMT-YUYV>
    - - `VIDEO_PALETTE_UYVY`
      - V4L2_PIX_FMT_UYVY <V4L2-PIX-FMT-UYVY>
    - - `VIDEO_PALETTE_YUV420`
      - None
    - - `VIDEO_PALETTE_YUV411`
      - V4L2_PIX_FMT_Y41P <V4L2-PIX-FMT-Y41P> [#f6]_
    - - `VIDEO_PALETTE_RAW`
      - None [#f7]_
    - - `VIDEO_PALETTE_YUV422P`
      - V4L2_PIX_FMT_YUV422P <V4L2-PIX-FMT-YUV422P>
    - - `VIDEO_PALETTE_YUV411P`
      - V4L2_PIX_FMT_YUV411P <V4L2-PIX-FMT-YUV411P> [#f8]_
    - - `VIDEO_PALETTE_YUV420P`
      - V4L2_PIX_FMT_YVU420 <V4L2-PIX-FMT-YVU420>
    - - `VIDEO_PALETTE_YUV410P`
      - V4L2_PIX_FMT_YVU410 <V4L2-PIX-FMT-YVU410>

V4L2 鍥惧儚鏍煎紡瀹氫箟浜?pixfmt銆傚浘鍍忔牸寮忓彲閫氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 閫夋嫨銆?
## 闊抽


`VIDIOCGAUDIO` 鍜?`VIDIOCSAUDIO` ioctl 浠ュ強 struct `video_audio` 鐢ㄤ簬鏋氫妇 V4L 璁惧鐨勯煶棰戣緭鍏ャ€傜瓑浠风殑 V4L2 ioctl 鏄?VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 鍜?VIDIOC_S_AUDIO <VIDIOC_G_AUDIO>锛屼娇鐢?struct `v4l2_audio`锛屽 audio 涓墍杩般€?
鐢ㄤ簬璁℃暟闊抽杈撳叆鐨?`audio` 鈥渃hannel number鈥?瀛楁琚噸鍛藉悕涓?`index`銆?
鍦?`VIDIOCSAUDIO` 涓紝`mode` 瀛楁閫夋嫨 `VIDEO_SOUND_MONO`銆乣VIDEO_SOUND_STEREO`銆乣VIDEO_SOUND_LANG1` 鎴?`VIDEO_SOUND_LANG2` 闊抽瑙ｈ皟妯″紡涓殑**涓€绉?*銆傚綋褰撳墠闊抽鏍囧噯涓?BTSC 鏃讹紝`VIDEO_SOUND_LANG2` 鎸?SAP锛岃€?`VIDEO_SOUND_LANG1` 娌℃湁鎰忎箟銆俈4L 瑙勮寖涓篃鏈褰曪紝娌℃湁鍔炴硶鏌ヨ鎵€閫夋ā寮忋€傚湪 `VIDIOCGAUDIO` 涓紝椹卞姩鍦ㄨ瀛楁杩斿洖**瀹為檯鎺ユ敹**鍒扮殑闊抽鑺傜洰銆傚湪 V4L2 API 涓紝璇ヤ俊鎭垎鍒瓨鍌ㄥ湪 struct `v4l2_tuner` 鐨?`rxsubchans` 鍜?`audmode` 瀛楁涓€傛湁鍏宠皟璋愬櫒鐨勬洿澶氫俊鎭鍙傝 tuner銆備笌闊抽妯″紡鐩稿叧锛宻truct `v4l2_audio` 杩樹細鎶ュ憡杩欐槸鍗曞０閬撹繕鏄珛浣撳０杈撳叆锛屾棤璁哄叾鏉ユ簮鏄惁涓鸿皟璋愬櫒銆?
浠ヤ笅瀛楁琚彲閫氳繃 VIDIOC_QUERYCTRL銆乂IDIOC_G_CTRL <VIDIOC_G_CTRL> 鍜?VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 璁块棶鐨?V4L2 鎺т欢鍙栦唬锛?

    :header-rows:  1
    :stub-columns: 0

    - - struct `video_audio`
      - V4L2 Control ID
    - - `volume`
      - `V4L2_CID_AUDIO_VOLUME`
    - - `bass`
      - `V4L2_CID_AUDIO_BASS`
    - - `treble`
      - `V4L2_CID_AUDIO_TREBLE`
    - - `balance`
      - `V4L2_CID_AUDIO_BALANCE`

涓轰簡纭畾椹卞姩鏀寔鍏朵腑鍝簺鎺т欢锛孷4L 鎻愪緵浜?`flags` `VIDEO_AUDIO_VOLUME`銆乣VIDEO_AUDIO_BASS`銆乣VIDEO_AUDIO_TREBLE` 鍜?`VIDEO_AUDIO_BALANCE`銆傚湪 V4L2 API 涓紝VIDIOC_QUERYCTRL ioctl 浼氭姤鍛婄浉搴旀帶浠舵槸鍚﹁鏀寔銆傜浉搴斿湴锛宍VIDEO_AUDIO_MUTABLE` 鍜?`VIDEO_AUDIO_MUTE` 鏍囧織琚竷灏斿瀷 `V4L2_CID_AUDIO_MUTE` 鎺т欢鍙栦唬銆?
鎵€鏈?V4L2 鎺т欢閮芥湁涓€涓?`step` 灞炴€э紝鍙栦唬浜?struct `video_audio` 鐨?`step` 瀛楁銆俈4L 闊抽鎺т欢鍋囧畾鍙栧€艰寖鍥翠负 0 鍒?65535锛屾病鏈夌壒瀹氱殑澶嶄綅鍊笺€俈4L2 API 鍏佽浠绘剰鐨勯檺鍒跺拰榛樿鍊硷紝鍙€氳繃 VIDIOC_QUERYCTRL ioctl 鏌ヨ銆傚叧浜庢帶浠剁殑涓€鑸俊鎭鍙傝 control銆?
## 甯х紦鍐插彔鍔?

涓?`VIDIOCGFBUF` 鍜?`VIDIOCSFBUF` 绛変环鐨?V4L2 ioctl 鏄?VIDIOC_G_FBUF <VIDIOC_G_FBUF> 鍜?VIDIOC_S_FBUF <VIDIOC_G_FBUF>銆俿truct `video_buffer` 鐨?`base` 瀛楁淇濇寔涓嶅彉锛屼絾 V4L2 瀹氫箟浜嗕竴涓爣蹇楁潵琛ㄧず闈炵牬鍧忔€х殑鍙犲姞锛岃€岄潪浣跨敤 `NULL` 鎸囬拡銆傛墍鏈夊叾浠栧瓧娈甸兘绉诲叆浜?struct `v4l2_framebuffer` 鐨?struct `v4l2_pix_format` `fmt` 瀛愮粨鏋勩€俙depth` 瀛楁琚?`pixelformat` 鍙栦唬銆傚叧浜?RGB 鏍煎紡鍙婂叾鍚勮嚜棰滆壊娣卞害鐨勫垪琛紝璇峰弬瑙?pixfmt-rgb銆?
V4L2 浣跨敤閫氱敤鐨勬暟鎹牸寮忓崗鍟?ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜?VIDIOC_S_FMT <VIDIOC_G_FMT>锛岃€岄潪鐗规畩鐨?`VIDIOCGWIN` 鍜?`VIDIOCSWIN` ioctl銆傚畠浠帴鍙椾竴涓寚鍚?struct `v4l2_format` 鐨勬寚閽堜綔涓哄弬鏁般€傝繖閲屼娇鐢?`fmt` 鑱斿悎鐨?`win` 鎴愬憳锛屽嵆 struct `v4l2_window`銆?
struct `video_window` 鐨?`x`銆乣y`銆乣width` 鍜?`height` 瀛楁绉诲叆浜?struct `v4l2_window` 鐨?struct `v4l2_rect` 瀛愮粨鏋?`w`銆俙chromakey`銆乣clips` 鍜?`clipcount` 瀛楁淇濇寔涓嶅彉銆俿truct `video_clip` 琚噸鍛藉悕涓?struct `v4l2_clip`锛屽悓鏍峰寘鍚竴涓?struct `v4l2_rect`锛屼絾璇箟浠嶇劧鐩稿悓銆?
`VIDEO_WINDOW_INTERLACE` 鏍囧織琚Щ闄ゃ€傜浉鍙嶏紝搴旂敤绋嬪簭蹇呴』灏?`field` 瀛楁璁剧疆涓?`V4L2_FIELD_ANY` 鎴?`V4L2_FIELD_INTERLACED`銆俙VIDEO_WINDOW_CHROMAKEY` 鏍囧織绉诲叆浜?struct `v4l2_framebuffer`锛屾柊鍚嶇О涓?`V4L2_FBUF_FLAG_CHROMAKEY`銆?
鍦?V4L 涓紝灏嗕綅鍥炬寚閽堝瓨鍏?`clips` 骞跺皢 `clipcount` 璁句负 `VIDEO_CLIP_BITMAP` (-1) 鍗宠姹備綅鍥捐鍓紝浣跨敤鍥哄畾澶у皬涓?1024 脳 625 浣嶇殑浣嶅浘銆俿truct `v4l2_window` 涓烘鎻愪緵浜嗕竴涓嫭绔嬬殑 `bitmap` 鎸囬拡瀛楁锛屼綅鍥惧ぇ灏忕敱 `w.width` 鍜?`w.height` 鍐冲畾銆?
鐢ㄤ簬鍚敤鎴栫鐢ㄥ彔鍔犵殑 `VIDIOCCAPTURE` ioctl 琚噸鍛藉悕涓?VIDIOC_OVERLAY銆?
## 瑁佸壀


涓轰簡浠呮崟鑾峰畬鏁寸敾闈㈢殑涓€閮ㄥ垎锛孷4L 瀹氫箟浜嗕娇鐢?struct `video_capture` 鐨?`VIDIOCGCAPTURE` 鍜?`VIDIOCSCAPTURE` ioctls銆傜瓑浠风殑 V4L2 ioctl 鏄娇鐢?struct `v4l2_crop` 鐨?VIDIOC_G_CROP <VIDIOC_G_CROP> 鍜?VIDIOC_S_CROP <VIDIOC_G_CROP>锛屼互鍙婄浉鍏崇殑 VIDIOC_CROPCAP ioctl銆傝繖鏄浉褰撳鏉傜殑闂锛岃瑙?crop銆?
`x`銆乣y`銆乣width` 鍜?`height` 瀛楁绉诲叆浜?struct `v4l2_crop` 鐨?struct `v4l2_rect` 瀛愮粨鏋?`c`銆俙decimation` 瀛楁琚Щ闄ゃ€傚湪 V4L2 API 涓紝缂╂斁绯绘暟鐢辫鍓煩褰㈢殑澶у皬浠ュ強鎵€鎹曡幏鎴栧彔鍔犲浘鍍忕殑澶у皬闅愬惈銆?
鐢ㄤ簬浠呮崟鑾峰鏁板満鎴栧伓鏁板満鐨?`VIDEO_CAPTURE_ODD` 鍜?`VIDEO_CAPTURE_EVEN` 鏍囧織锛屽湪 struct `v4l2_pix_format` 鍜?struct `v4l2_window` 涓悕涓?`field` 鐨勫瓧娈甸噷琚?`V4L2_FIELD_TOP` 鍜?`V4L2_FIELD_BOTTOM` 鍙栦唬銆傝繖浜涚粨鏋勭敤浜庨€氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 閫夋嫨鎹曡幏鎴栧彔鍔犳牸寮忋€?
## 璇诲彇鍥惧儚銆佸唴瀛樻槧灏?

### 浣跨敤 read 鏂规硶鎹曡幏


浣跨敤 `read()` 鍑芥暟浠?V4L 鎴?V4L2 璁惧璇诲彇鍥惧儚鍦ㄦ湰璐ㄤ笂娌℃湁鍖哄埆锛屼絾 V4L2 椹卞姩骞朵笉瑕佹眰鏀寔杩欑 I/O 鏂规硶銆傚簲鐢ㄧ▼搴忓彲浠ラ€氳繃 VIDIOC_QUERYCAP ioctl 纭畾璇ュ嚱鏁版槸鍚﹀彲鐢ㄣ€傛墍鏈変笌搴旂敤绋嬪簭浜ゆ崲鏁版嵁鐨?V4L2 璁惧閮藉繀椤绘敮鎸?`select()` 鍜?`poll()` 鍑芥暟銆?
瑕侀€夋嫨鍥惧儚鏍煎紡鍜屽ぇ灏忥紝V4L 鎻愪緵 `VIDIOCSPICT` 鍜?`VIDIOCSWIN` ioctls銆俈4L2 浣跨敤閫氱敤鐨勬暟鎹牸寮忓崗鍟?ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜?VIDIOC_S_FMT <VIDIOC_G_FMT>銆傚畠浠帴鍙椾竴涓寚鍚?struct `v4l2_format` 鐨勬寚閽堜綔涓哄弬鏁帮紝杩欓噷浣跨敤鍏?`fmt` 鑱斿悎涓悕涓?`pix` 鐨?struct `v4l2_pix_format`銆?
鍏充簬 V4L2 read 鎺ュ彛鐨勬洿澶氫俊鎭鍙傝 rw銆?
### 浣跨敤鍐呭瓨鏄犲皠鎹曡幏


搴旂敤绋嬪簭鍙互閫氳繃灏嗚澶囧唴瀛樹腑鐨勭紦鍐插尯锛堟垨鏇村父瑙佸湴锛屼粎鍦ㄥ彲 DMA 鐨勭郴缁熷唴瀛樹腑鍒嗛厤鐨勭紦鍐插尯锛夋槧灏勫埌鍏跺湴鍧€绌洪棿锛屾潵浠?V4L 璁惧璇诲彇鏁版嵁銆傝繖閬垮厤浜?read 鏂规硶鐨勬暟鎹嫹璐濆紑閿€銆俈4L2 鍚屾牱鏀寔鍐呭瓨鏄犲皠锛屼絾鏈変竴浜涘尯鍒€?

    :header-rows:  1
    :stub-columns: 0

    - - V4L
      - V4L2
    - -
      - 鍦ㄥ垎閰嶇紦鍐插尯涔嬪墠蹇呴』閫夋嫨鍥惧儚鏍煎紡锛屼娇鐢?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl銆傝嫢鏈€夋嫨鏍煎紡锛岄┍鍔ㄥ彲鑳戒細浣跨敤涓婁竴娆★紙鍙兘鐢卞彟涓€搴旂敤绋嬪簭璇锋眰鐨勶級鏍煎紡銆?    - - 搴旂敤绋嬪簭鏃犳硶鏇存敼缂撳啿鍖虹殑鏁伴噺銆傛暟閲忓唴缃簬椹卞姩涓紝闄ら潪椹卞姩妯″潡鍦ㄥ姞杞芥椂鎻愪緵浜嗙敤浜庢洿鏀规暟閲忕殑妯″潡閫夐」銆?      - VIDIOC_REQBUFS ioctl 鍒嗛厤鎵€闇€鏁伴噺鐨勭紦鍐插尯锛岃繖鏄垵濮嬪寲搴忓垪涓繀闇€鐨勬楠ゃ€?    - - 椹卞姩灏嗘墍鏈夌紦鍐插尯浣滀负涓€涓繛缁殑鍐呭瓨鑼冨洿杩涜鏄犲皠銆傚彲浣跨敤 `VIDIOCGMBUF` ioctl 鏌ヨ缂撳啿鍖烘暟閲忋€佹瘡涓紦鍐插尯鐩稿浜庤櫄鎷熸枃浠惰捣濮嬩綅缃殑鍋忕Щ閲忥紝浠ュ強鎵€鐢ㄧ殑鎬诲唴瀛橀噺锛岃繖浜涘彲浣滀负 `mmap()` 鍑芥暟鐨勫弬鏁般€?      - 缂撳啿鍖鸿鍗曠嫭鏄犲皠銆傛瘡涓紦鍐插尯鐨勫亸绉婚噺鍜屽ぇ灏忓彲閫氳繃 VIDIOC_QUERYBUF ioctl 纭畾銆?    - - `VIDIOCMCAPTURE` ioctl 鍑嗗涓€涓紦鍐插尯鐢ㄤ簬鎹曡幏锛屽悓鏃剁‘瀹氳缂撳啿鍖虹殑鍥惧儚鏍煎紡銆傝 ioctl 绔嬪嵆杩斿洖锛岃嫢鏈娴嬪埌瑙嗛淇″彿锛屾渶缁堝彲鑳借繑鍥?`EAGAIN` 閿欒鐮併€傚綋椹卞姩鏀寔澶氫釜缂撳啿鍖烘椂锛屽簲鐢ㄧ▼搴忓彲浠ュ娆¤皟鐢ㄨ ioctl锛屼粠鑰屾嫢鏈夊涓湭瀹屾垚鐨勬崟鑾疯姹傘€?
	`VIDIOCSYNC` ioctl 浼氭寕璧锋墽琛岋紝鐩村埌鐗瑰畾缂撳啿鍖鸿濉厖瀹屾瘯銆?      - 椹卞姩缁存姢涓€涓紶鍏ラ槦鍒楀拰涓€涓紶鍑洪槦鍒椼€俈IDIOC_QBUF 灏嗕换鎰忕┖缂撳啿鍖哄姞鍏ヤ紶鍏ラ槦鍒椼€傚凡濉厖鐨勭紦鍐插尯閫氳繃 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 浠庝紶鍑洪槦鍒楀彇鍑恒€傝绛夊緟宸插～鍏呯紦鍐插尯鍙樹负鍙敤锛屽彲浠ヤ娇鐢ㄨ鍑芥暟銆乣select()` 鎴?`poll()`銆傚湪鍏ラ槦涓€涓垨澶氫釜缂撳啿鍖哄悗锛屽繀椤昏皟鐢ㄤ竴娆?VIDIOC_STREAMON ioctl 浠ュ紑濮嬫崟鑾枫€傚叾瀵瑰簲椤?VIDIOC_STREAMOFF <VIDIOC_STREAMON> 浼氬仠姝㈡崟鑾凤紝骞朵粠涓や釜闃熷垪涓彇鍑烘墍鏈夌紦鍐插尯銆傝嫢宸茬煡淇″彿鐘舵€侊紝搴旂敤绋嬪簭鍙互閫氳繃 VIDIOC_ENUMINPUT ioctl 鏌ヨ銆?
鍏充簬鍐呭瓨鏄犲皠鍙婄ず渚嬬殑鏇存繁鍏ヨ璁猴紝璇峰弬瑙?mmap銆?
## 璇诲彇鍘熷 VBI 鏁版嵁


鏈€鍒?V4L API 骞舵湭瑙勫畾鍘熷 VBI 鎹曡幏鎺ュ彛锛屼粎涓鸿鐢ㄩ€斾繚鐣欎簡璁惧鏂囦欢 `/dev/vbi`銆傚敮涓€鏀寔璇ユ帴鍙ｇ殑椹卞姩鏄?BTTV 椹卞姩锛屽畠瀹為檯涓婂畾涔変簡 V4L VBI 鎺ュ彛銆備粠璇ヨ澶囪鍙栦細寰楀埌涓€涓叿鏈変互涓嬪弬鏁扮殑鍘熷 VBI 鍥惧儚锛?

    :header-rows:  1
    :stub-columns: 0

    - - struct `v4l2_vbi_format`
      - V4L, BTTV driver
    - - sampling_rate
      - 28636363 Hz NTSC (or any other 525-line standard); 35468950 Hz PAL
	and SECAM (625-line standards)
    - - offset
      - ?
    - - samples_per_line
      - 2048
    - - sample_format
      - V4L2_PIX_FMT_GREY銆傛渶鍚庡洓涓瓧鑺傦紙涓€涓満鍣ㄥ瓧鑺傚簭鏁存暟锛夊寘鍚竴涓抚璁℃暟鍣ㄣ€?    - - start[]
      - 10, 273 NTSC; 22, 335 PAL and SECAM
    - - count[]
      - 16, 16 [#f9]_
    - - flags
      - 0

V4L 瑙勮寖涓湭璁板綍锛屽湪 Linux 2.3 涓姞鍏ヤ簡浣跨敤 struct `vbi_format` 鐨?`VIDIOCGVBIFMT` 鍜?`VIDIOCSVBIFMT` ioctls锛岀敤浜庣‘瀹?VBI 鍥惧儚鍙傛暟銆傝繖浜?ioctl 浠呬笌 raw-vbi 涓瀹氱殑 V4L2 VBI 鎺ュ彛閮ㄥ垎鍏煎銆?
涓嶅瓨鍦?`offset` 瀛楁锛宍sample_format` 搴斾负 `VIDEO_PALETTE_RAW`锛岀瓑浠蜂簬 `V4L2_PIX_FMT_GREY`銆傚叾浣欏瓧娈靛彲鑳界瓑浠蜂簬 struct `v4l2_vbi_format`銆?
鏄剧劧鍙湁 Zoran锛圸R 36120锛夐┍鍔ㄥ疄鐜颁簡杩欎簺 ioctl銆傚叾璇箟涓?V4L2 鐨勮瀹氭湁涓ゅ涓嶅悓锛氬弬鏁板湪 `open()` 鏃堕噸缃紝涓斿綋鍙傛暟鏃犳晥鏃?`VIDIOCSVBIFMT` 鎬绘槸杩斿洖 `EINVAL` 閿欒鐮併€?
## 鏉傞」


V4L2 娌℃湁涓?`VIDIOCGUNIT` ioctl 绛変环鐨勫唴瀹广€傚簲鐢ㄧ▼搴忓彲浠ラ€氳繃閲嶆柊鎵撳紑璁惧骞惰姹?VBI 鏁版嵁锛屾潵鎵惧埌涓庤棰戞崟鑾疯澶囧叧鑱旂殑 VBI 璁惧锛堝弽涔嬩害鐒讹級銆傝鎯呰鍙傝 open銆?
瀵逛簬 `VIDIOCKEY` 浠ュ強 V4L 鐨勫井鐮佺紪绋嬪嚱鏁帮紝鐩墠娌℃湁鏇夸唬椤广€傚叧浜?MPEG 鍘嬬缉鍜屾挱鏀捐澶囩殑鏂版帴鍙ｈ褰曞湪 extended-controls 涓€?
   鏍规嵁 Documentation/admin-guide/devices.rst锛岃繖浜涘簲璇ユ槸鎸囧悜 `/dev/video0` 鐨勭鍙烽摼鎺ャ€傛敞鎰忓師濮嬬殑 bttv 鎺ュ彛涓?V4L 鎴?V4L2 鍧囦笉鍏煎銆?
   鏍规嵁 `Documentation/admin-guide/devices.rst`锛岃繖鏄竴涓寚鍚?`/dev/radio0` 鐨勭鍙烽摼鎺ャ€?
   杩欐槸 BTTV 椹卞姩浣跨敤鐨勮嚜瀹氫箟鏍煎紡锛屽苟闈?V4L2 鏍囧噯鏍煎紡涔嬩竴銆?
   鎺ㄦ祴鎵€鏈?V4L RGB 鏍煎紡閮芥槸灏忕搴忥紝灏界鏌愪簺椹卞姩鍙兘鎸夋満鍣ㄥ瓧鑺傚簭鏉ヨВ閲婂畠浠€俈4L2 瀹氫箟浜嗗皬绔簭銆佸ぇ绔簭浠ュ強绾?钃濅氦鎹㈢殑鍙樹綋銆傝鎯呰鍙傝 pixfmt-rgb銆?
   `VIDEO_PALETTE_YUV422` 鍜?`VIDEO_PALETTE_YUYV` 鏄悓涓€鏍煎紡銆傛煇浜?V4L 椹卞姩鍝嶅簲鍏朵腑涓€绉嶏紝鏌愪簺鍝嶅簲鍙︿竴绉嶃€?
   涓嶈涓?`V4L2_PIX_FMT_YUV411P` 娣锋穯锛屽悗鑰呮槸骞抽潰锛坧lanar锛夋牸寮忋€?
   V4L 灏嗗叾瑙ｉ噴涓猴細"RAW capture (BT848)"

   涓嶈涓?`V4L2_PIX_FMT_Y41P` 娣锋穯锛屽悗鑰呮槸鎵撳寘锛坧acked锛夋牸寮忋€?
   鏃х殑椹卞姩鐗堟湰浣跨敤浜嗕笉鍚岀殑鍊硷紝鏈€缁堝姞鍏ヤ簡鑷畾涔夌殑 `BTTV_VBISIZE` ioctl 鏉ユ煡璇㈡纭殑鍊笺€?