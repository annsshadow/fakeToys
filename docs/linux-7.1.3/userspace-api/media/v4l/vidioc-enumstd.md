

######## ioctl VIDIOC_ENUMSTD, VIDIOC_SUBDEV_ENUMSTD


## Name


VIDIOC_ENUMSTD - VIDIOC_SUBDEV_ENUMSTD - 鏋氫妇鏀寔鐨勮棰戞爣鍑?

## Synopsis



`int ioctl(int fd, VIDIOC_ENUMSTD, struct v4l2_standard *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_ENUMSTD, struct v4l2_standard *argp)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜 struct `v4l2_standard` 鐨勬寚閽堛€?

## Description


涓轰簡鏌ヨ鏌愪釜瑙嗛鏍囧噯鐨勫睘鎬э紝灏ゅ叾鏄嚜瀹氫箟鐨勶紙鐢遍┍鍔ㄥ畾涔夌殑锛夋爣鍑嗭紝搴旂敤绋嬪簭鍒濆鍖?
struct `v4l2_standard` 鐨?`index` 瀛楁锛屽苟浣跨敤鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_ENUMSTD
ioctl銆傞┍鍔ㄥ～鍏呯粨鏋勭殑鍏朵綑閮ㄥ垎锛屾垨鑰呭綋 index 瓒婄晫鏃惰繑鍥?`EINVAL` 閿欒鐮併€備负浜嗘灇涓?
鎵€鏈夋爣鍑嗭紝搴旂敤绋嬪簭搴斾粠 index 0 寮€濮嬶紝姣忔鍔?1锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`銆傞┍鍔ㄥ湪鍒囨崲
瑙嗛杈撳叆鎴栬緭鍑哄悗锛屽彲鑳戒細鏋氫妇鍑轰竴缁勪笉鍚岀殑鏍囧噯銆俒#f1]_



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 瑙嗛鏍囧噯鐨勭紪鍙凤紝鐢卞簲鐢ㄧ▼搴忚缃€?
    - - v4l2_std_id <v4l2-std-id>
      - `id`
      - 璇ュ瓧娈典腑鐨勪綅灏嗘爣鍑嗘爣璇嗕负 v4l2-std-id 涓垪鍑虹殑甯歌鏍囧噯涔嬩竴锛屾垨鑰咃紝濡傛灉
	绗?32 鑷?63 浣嶈缃綅锛屽垯鏍囪瘑涓鸿嚜瀹氫箟鏍囧噯銆傚鏋滅‖浠舵棤娉曞尯鍒嗚繖浜涙爣鍑嗭紝鍙互
	璁剧疆澶氫釜浣嶏紱涓嶈繃鐙珛鐨?index 骞朵笉琛ㄧず鐩稿弽鐨勬儏鍐点€俙id` 蹇呴』鏄敮涓€鐨勩€傚浜?
	璇ヨ緭鍏ユ垨杈撳嚭锛屼换浣曞叾浠栬鏋氫妇鐨?struct `v4l2_standard` 缁撴瀯閮戒笉鑳藉寘鍚浉鍚岀殑
	浣嶉泦鍚堛€?
    - - __u8
      - `name`\ [^24^]
      - 鏍囧噯鐨勫悕绉帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓诧紝渚嬪锛氣€淧AL-B/G鈥濄€佲€淣TSC Japan鈥濄€?
	姝や俊鎭緵鐢ㄦ埛浣跨敤銆?
    - - struct `v4l2_fract`
      - `frameperiod`
      - 甯у懆鏈燂紙鑰岄潪鍦哄懆鏈燂級涓?numerator / denominator銆備緥濡?M/NTSC 鐨勫抚鍛ㄦ湡涓?
	1001 / 30000 绉掋€?
    - - __u32
      - `framelines`
      - 姣忓抚鐨勬€昏鏁帮紝鍚秷闅愶紝渚嬪 B/PAL 涓?625銆?
    - - __u32
      - `reserved`\ [^4^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ繀椤诲皢璇ユ暟缁勭疆涓洪浂銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `numerator`
      -
    - - __u32
      - `denominator`
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `v4l2_std_id`
      - 璇ョ被鍨嬫槸涓€涓泦鍚堬紝姣忎竴浣嶄唬琛ㄤ笅闈互鍙?video-standards 涓墍鍒楀嚭鐨勫彟涓€涓棰?
	鏍囧噯銆傛渶楂樼殑 32 浣嶄繚鐣欑粰鑷畾涔夛紙椹卞姩瀹氫箟鐨勶級瑙嗛鏍囧噯銆?



    #define V4L2_STD_PAL_B          ((v4l2_std_id)0x00000001)
    #define V4L2_STD_PAL_B1         ((v4l2_std_id)0x00000002)
    #define V4L2_STD_PAL_G          ((v4l2_std_id)0x00000004)
    #define V4L2_STD_PAL_H          ((v4l2_std_id)0x00000008)
    #define V4L2_STD_PAL_I          ((v4l2_std_id)0x00000010)
    #define V4L2_STD_PAL_D          ((v4l2_std_id)0x00000020)
    #define V4L2_STD_PAL_D1         ((v4l2_std_id)0x00000040)
    #define V4L2_STD_PAL_K          ((v4l2_std_id)0x00000080)

    #define V4L2_STD_PAL_M          ((v4l2_std_id)0x00000100)
    #define V4L2_STD_PAL_N          ((v4l2_std_id)0x00000200)
    #define V4L2_STD_PAL_Nc         ((v4l2_std_id)0x00000400)
    #define V4L2_STD_PAL_60         ((v4l2_std_id)0x00000800)

`V4L2_STD_PAL_60` 鏄竴绉嶆贩鍚堟爣鍑嗭紝鍏锋湁 525 琛屻€?0 Hz 鍒锋柊鐜囷紝浠ュ強浣跨敤 4.43 MHz
鑹插害鍓浇娉㈢殑 PAL 褰╄壊璋冨埗銆傛煇浜?PAL 褰曞儚鏈哄彲鍦ㄨ妯″紡涓嬪洖鏀?NTSC 纾佸甫锛屼互渚垮湪
50/60 Hz 鏃犲叧鐨?PAL 鐢佃涓婃樉绀恒€?


    #define V4L2_STD_NTSC_M         ((v4l2_std_id)0x00001000)
    #define V4L2_STD_NTSC_M_JP      ((v4l2_std_id)0x00002000)
    #define V4L2_STD_NTSC_443       ((v4l2_std_id)0x00004000)

`V4L2_STD_NTSC_443` 鏄竴绉嶆贩鍚堟爣鍑嗭紝鍏锋湁 525 琛屻€?0 Hz 鍒锋柊鐜囷紝浠ュ強浣跨敤 4.43 MHz
鑹插害鍓浇娉㈢殑 NTSC 褰╄壊璋冨埗銆?


    #define V4L2_STD_SECAM_B        ((v4l2_std_id)0x00010000)
    #define V4L2_STD_SECAM_D        ((v4l2_std_id)0x00020000)
    #define V4L2_STD_SECAM_G        ((v4l2_std_id)0x00040000)
    #define V4L2_STD_SECAM_H        ((v4l2_std_id)0x00080000)
    #define V4L2_STD_SECAM_K        ((v4l2_std_id)0x00100000)
    #define V4L2_STD_SECAM_K1       ((v4l2_std_id)0x00200000)
    #define V4L2_STD_SECAM_L        ((v4l2_std_id)0x00400000)
    #define V4L2_STD_SECAM_LC       ((v4l2_std_id)0x00800000)

    #define V4L2_STD_ATSC_8_VSB     ((v4l2_std_id)0x01000000)
    #define V4L2_STD_ATSC_16_VSB    ((v4l2_std_id)0x02000000)

    /** ATSC/HDTV **/
    #define V4L2_STD_ATSC_8_VSB     ((v4l2_std_id)0x01000000)
    #define V4L2_STD_ATSC_16_VSB    ((v4l2_std_id)0x02000000)

`V4L2_STD_ATSC_8_VSB` 鍜?`V4L2_STD_ATSC_16_VSB` 鏄編鍥藉湴闈㈡暟瀛楃數瑙嗘爣鍑嗐€傜洰鍓?
V4L2 API 涓嶆敮鎸佹暟瀛楃數瑙嗐€傚彟璇峰弬闃?`https://linuxtv.org <https://linuxtv.org>`__ 涓婄殑
Linux DVB API銆?


    #define V4L2_STD_PAL_BG         (V4L2_STD_PAL_B         |
		     V4L2_STD_PAL_B1        |
		     V4L2_STD_PAL_G)
    #define V4L2_STD_B              (V4L2_STD_PAL_B         |
		     V4L2_STD_PAL_B1        |
		     V4L2_STD_SECAM_B)
    #define V4L2_STD_GH             (V4L2_STD_PAL_G         |
		     V4L2_STD_PAL_H         |
		     V4L2_STD_SECAM_G       |
		     V4L2_STD_SECAM_H)
    #define V4L2_STD_PAL_DK         (V4L2_STD_PAL_D         |
		     V4L2_STD_PAL_D1        |
		     V4L2_STD_PAL_K)
    #define V4L2_STD_PAL            (V4L2_STD_PAL_BG        |
		     V4L2_STD_PAL_DK        |
		     V4L2_STD_PAL_H         |
		     V4L2_STD_PAL_I)
    #define V4L2_STD_NTSC           (V4L2_STD_NTSC_M        |
		     V4L2_STD_NTSC_M_JP     |
		     V4L2_STD_NTSC_M_KR)
    #define V4L2_STD_MN             (V4L2_STD_PAL_M         |
		     V4L2_STD_PAL_N         |
		     V4L2_STD_PAL_Nc        |
		     V4L2_STD_NTSC)
    #define V4L2_STD_SECAM_DK       (V4L2_STD_SECAM_D       |
		     V4L2_STD_SECAM_K       |
		     V4L2_STD_SECAM_K1)
    #define V4L2_STD_DK             (V4L2_STD_PAL_DK        |
		     V4L2_STD_SECAM_DK)

    #define V4L2_STD_SECAM          (V4L2_STD_SECAM_B       |
		     V4L2_STD_SECAM_G       |
		     V4L2_STD_SECAM_H       |
		     V4L2_STD_SECAM_DK      |
		     V4L2_STD_SECAM_L       |
		     V4L2_STD_SECAM_LC)

    #define V4L2_STD_525_60         (V4L2_STD_PAL_M         |
		     V4L2_STD_PAL_60        |
		     V4L2_STD_NTSC          |
		     V4L2_STD_NTSC_443)
    #define V4L2_STD_625_50         (V4L2_STD_PAL           |
		     V4L2_STD_PAL_N         |
		     V4L2_STD_PAL_Nc        |
		     V4L2_STD_SECAM)

    #define V4L2_STD_UNKNOWN        0
    #define V4L2_STD_ALL            (V4L2_STD_525_60        |
		     V4L2_STD_625_50)


    \begingroup
    \tiny
    \setlength{\tabcolsep}{2pt}



    :header-rows:  1
    :stub-columns: 0

    - - Characteristics
      - M/NTSC [#f2]_
      - M/PAL
      - N/PAL [#f3]_
      - B, B1, G/PAL
      - D, D1, K/PAL
      - H/PAL
      - I/PAL
      - B, G/SECAM
      - D, K/SECAM
      - K1/SECAM
      - L/SECAM
    - - Frame lines
      - `1` 525
      - `8` 625
    - - Frame period (s)
      - `1` 1001/30000
      - `8` 1/25
    - - Chrominance sub-carrier frequency (Hz)
      - 3579545 卤 10
      - 3579611.49 卤 10
      - 4433618.75 卤 5

	(3582056.25 卤 5)
      - `3` 4433618.75 卤 5
      - 4433618.75 卤 1
      - `2` f\ `OR` = 4406250 卤 2000,

	f\ `OB` = 4250000 卤 2000
    - - Nominal radio-frequency channel bandwidth (MHz)
      - 6
      - 6
      - 6
      - B: 7; B1, G: 8
      - 8
      - 8
      - 8
      - 8
      - 8
      - 8
      - 8
    - - Sound carrier relative to vision carrier (MHz)
      - 4.5
      - 4.5
      - 4.5
      - 5.5 卤 0.001  [#f4]_  [#f5]_  [#f6]_  [#f7]_
      - 6.5 卤 0.001
      - 5.5
      - 5.9996 卤 0.0005
      - 5.5 卤 0.001
      - 6.5 卤 0.001
      - 6.5
      - 6.5 [#f8]_


    \endgroup


## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞惰缃?`errno`銆傞€氱敤閿欒鐮佸湪 Generic Error Codes
<gen-errors> 涓€绔犱腑鎻忚堪銆?

EINVAL
    struct `v4l2_standard` 鐨?`index` 瓒婄晫銆?

ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佹爣鍑嗚棰戞椂搴忋€?

   鏀寔鐨勬爣鍑嗗彲鑳界浉浜掗噸鍙狅紝鎴戜滑闇€瑕佷竴涓槑纭殑闆嗗悎鏉ユ煡鎵剧敱 VIDIOC_G_STD
   <VIDIOC_G_STD> 杩斿洖鐨勫綋鍓嶆爣鍑嗐€?

   鏃ユ湰浣跨敤鐨勬爣鍑嗙被浼间簬 M/NTSC锛圴4L2_STD_NTSC_M_JP锛夈€?

   鎷彿涓殑鍊奸€傜敤浜庤绉颁负 N\ `C` 鐨?N/PAL 缁勫悎锛岀敤浜庨樋鏍瑰环锛圴4L2_STD_PAL_Nc锛夈€?

   鍦ㄥ痉鍥姐€佸ゥ鍦板埄銆佹剰澶у埄銆佽嵎鍏般€佹柉娲涗紣鍏嬪拰鐟炲＋锛屼娇鐢ㄥ弻浼撮煶杞芥尝绯荤粺锛岀浜岃浇娉?
   棰戠巼姣旂涓€浼撮煶杞芥尝楂?242.1875 kHz銆傛境澶у埄浜氫娇鐢ㄧ被浼肩殑绯荤粺杩涜绔嬩綋澹板箍鎾€?

   鏂拌タ鍏颁娇鐢ㄧ殑浼撮煶杞芥尝鍋忕鍥惧儚杞芥尝 5.4996 卤 0.0005 MHz銆?

   鍦ㄤ腹楹︺€佽姮鍏般€佹柊瑗垮叞銆佺憺鍏稿拰瑗跨彮鐗欎娇鐢ㄥ弻浼撮煶杞芥尝绯荤粺銆傚啺宀涖€佹尓濞佸拰娉㈠叞姝ｅ湪
   寮曞叆鐩稿悓绯荤粺銆傜浜岃浇娉㈡瘮鍥惧儚杞芥尝楂?5.85 MHz锛岄噰鐢?DQPSK 璋冨埗锛屽甫 728 kbit/s
   鐨勪即闊充笌鏁版嵁澶嶇敤銆傦紙NICAM 绯荤粺锛?

   鍦ㄨ嫳鍥戒娇鐢ㄥ弻浼撮煶杞芥尝绯荤粺銆傜浜屼即闊宠浇娉㈡瘮鍥惧儚杞芥尝楂?6.552 MHz锛岄噰鐢?DQPSK 璋冨埗锛?
   甯︽湁鑳芥壙杞戒袱涓即闊冲０閬撶殑 728 kbit/s 浼撮煶涓庢暟鎹鐢ㄣ€傦紙NICAM 绯荤粺锛?

   鍦ㄦ硶鍥斤紝闄や富浼撮煶杞芥尝澶栵紝杩樺彲鑳戒娇鐢ㄤ竴涓亸绂诲浘鍍忚浇娉?5.85 MHz 鐨勬暟瀛楄浇娉€傚畠閲囩敤
   宸垎缂栫爜鐨?QPSK 璋冨埗锛屽甫鏈変竴涓兘鎵胯浇涓や釜浼撮煶澹伴亾鐨?728 kbit/s 浼撮煶涓庢暟鎹鐢ㄥ櫒銆?
   锛圢ICAM 绯荤粺锛?
