

######## 缂栬В鐮佸櫒鎺у埗鍙傝€?


涓嬮潰鎻忚堪缂栬В鐮佸櫒鎺у埗绫讳腑鐨勬墍鏈夋帶浠躲€傞鍏堟槸閫氱敤鎺т欢锛岀劧鍚庢槸鐗瑰畾浜庢煇浜涚‖浠剁殑鎺т欢銆?


   杩欎簺鎺т欢骞堕潪浠呴€傜敤浜?MPEG锛岃€屾槸閫傜敤浜庢墍鏈夌紪瑙ｇ爜鍣ㄣ€傝繖浜涘畾涔変互
   V4L2_CID_MPEG/V4L2_MPEG 涓哄墠缂€锛屽洜涓烘帶浠舵渶鍒濇槸涓?MPEG 缂栬В鐮佸櫒鍒涘缓鐨勶紝
   鍚庢潵琚墿灞曚互娑电洊鎵€鏈夌紪鐮佹牸寮忋€?


## 閫氱敤缂栬В鐮佸櫒鎺т欢



### 缂栬В鐮佸櫒鎺т欢 ID



`V4L2_CID_CODEC_CLASS (class)`
    缂栬В鐮佸櫒锛圕odec锛夌被鐨勬弿杩扮銆傚璇ユ帶浠惰皟鐢?VIDIOC_QUERYCTRL 灏嗚繑鍥?
    璇ユ帶浠剁被鐨勬弿杩般€備緥濡傦紝姝ゆ弿杩板彲鐢ㄤ綔 GUI 涓煇涓€夐」鍗★紙Tab锛夐〉闈㈢殑
    鏍囬銆?


`V4L2_CID_MPEG_STREAM_TYPE`
    (enum)

enum v4l2_mpeg_stream_type -
    MPEG-1銆?2 鎴?-4 杈撳嚭娴佺被鍨嬨€傝繖閲屼笉鑳藉仛浠讳綍鍋囪銆傛瘡绉嶇‖浠?MPEG 缂栫爜鍣ㄥ線寰€鏀寔鍙敤 MPEG 娴佺被鍨嬬殑涓嶅悓瀛愰泦銆傝鎺т欢涓撶敤浜庡璺鐢ㄧ殑 MPEG 娴併€傚綋鍓嶅凡瀹氫箟鐨勬祦绫诲瀷濡備笅锛?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_PS`
      - MPEG-2 鑺傜洰娴?
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_TS`
      - MPEG-2 浼犺緭娴?
    - - `V4L2_MPEG_STREAM_TYPE_MPEG1_SS`
      - MPEG-1 绯荤粺娴?
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_DVD`
      - MPEG-2 DVD 鍏煎娴?
    - - `V4L2_MPEG_STREAM_TYPE_MPEG1_VCD`
      - MPEG-1 VCD 鍏煎娴?
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_SVCD`
      - MPEG-2 SVCD 鍏煎娴?



`V4L2_CID_MPEG_STREAM_PID_PMT (integer)`
    MPEG 浼犺緭娴佺殑绋嬪簭鏄犲皠琛紙PMT锛夊寘 ID锛堥粯璁?16锛?

`V4L2_CID_MPEG_STREAM_PID_AUDIO (integer)`
    MPEG 浼犺緭娴佺殑闊抽鍖?ID锛堥粯璁?256锛?

`V4L2_CID_MPEG_STREAM_PID_VIDEO (integer)`
    MPEG 浼犺緭娴佺殑瑙嗛鍖?ID锛堥粯璁?260锛?

`V4L2_CID_MPEG_STREAM_PID_PCR (integer)`
    鎵胯浇 PCR 瀛楁鐨?MPEG 浼犺緭娴佸寘 ID锛堥粯璁?259锛?

`V4L2_CID_MPEG_STREAM_PES_ID_AUDIO (integer)`
    MPEG PES 鐨勯煶棰?ID

`V4L2_CID_MPEG_STREAM_PES_ID_VIDEO (integer)`
    MPEG PES 鐨勮棰?ID


`V4L2_CID_MPEG_STREAM_VBI_FMT`
    (enum)

enum v4l2_mpeg_stream_vbi_fmt -
    鏌愪簺鍗″彲浠ュ皢 VBI 鏁版嵁锛堜緥濡傚瓧骞曪紙Closed Caption锛夈€佸浘鏂囩數瑙嗭紙Teletext锛夛級宓屽叆鍒?MPEG 娴佷腑銆傝鎺т欢閫夋嫨鏄惁搴斿祵鍏?VBI 鏁版嵁锛屼互鍙婂鏋滃祵鍏ワ紝搴旈噰鐢ㄤ綍绉嶅祵鍏ユ柟寮忋€傚彲鑳界殑 VBI 鏍煎紡鍒楄〃鍙栧喅浜庨┍鍔ㄣ€傚綋鍓嶅凡瀹氫箟鐨?VBI 鏍煎紡绫诲瀷濡備笅锛?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_STREAM_VBI_FMT_NONE`
      - MPEG 娴佷腑鏃?VBI
    - - `V4L2_MPEG_STREAM_VBI_FMT_IVTV`
      - 绉佹湁鍖呬腑鐨?VBI锛孖VTV 鏍煎紡锛堝湪鍐呮牳婧愮爜鏂囦欢
	`Documentation/userspace-api/media/drivers/cx2341x-uapi.rst` 涓湁鏂囨。璇存槑锛?



`V4L2_CID_MPEG_AUDIO_SAMPLING_FREQ`
    (enum)

enum v4l2_mpeg_audio_sampling_freq -
    MPEG 闊抽閲囨牱棰戠巼銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_44100`
      - 44.1 kHz
    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_48000`
      - 48 kHz
    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_32000`
      - 32 kHz



`V4L2_CID_MPEG_AUDIO_ENCODING`
    (enum)

enum v4l2_mpeg_audio_encoding -
    MPEG 闊抽缂栫爜銆傝鎺т欢涓撶敤浜庡璺鐢ㄧ殑 MPEG 娴併€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_1`
      - MPEG-1/2 绗竴灞傦紙Layer I锛夌紪鐮?
    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_2`
      - MPEG-1/2 绗簩灞傦紙Layer II锛夌紪鐮?
    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_3`
      - MPEG-1/2 绗笁灞傦紙Layer III锛夌紪鐮?
    - - `V4L2_MPEG_AUDIO_ENCODING_AAC`
      - MPEG-2/4 AAC锛堥珮绾ч煶棰戠紪鐮侊紝Advanced Audio Coding锛?
    - - `V4L2_MPEG_AUDIO_ENCODING_AC3`
      - AC-3锛屽嵆 ATSC A/52 缂栫爜



`V4L2_CID_MPEG_AUDIO_L1_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l1_bitrate -
    MPEG-1/2 绗竴灞傛瘮鐗圭巼銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L1_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_288K`
      - 288 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_352K`
      - 352 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_384K`
      - 384 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_416K`
      - 416 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_448K`
      - 448 kbit/s



`V4L2_CID_MPEG_AUDIO_L2_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l2_bitrate -
    MPEG-1/2 绗簩灞傛瘮鐗圭巼銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L2_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_384K`
      - 384 kbit/s



`V4L2_CID_MPEG_AUDIO_L3_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l3_bitrate -
    MPEG-1/2 绗笁灞傛瘮鐗圭巼銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L3_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_40K`
      - 40 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_320K`
      - 320 kbit/s



`V4L2_CID_MPEG_AUDIO_AAC_BITRATE (integer)`
    AAC 姣旂壒鐜囷紝鍗曚綅涓烘瘮鐗规瘡绉掋€?


`V4L2_CID_MPEG_AUDIO_AC3_BITRATE`
    (enum)

enum v4l2_mpeg_audio_ac3_bitrate -
    AC-3 姣旂壒鐜囥€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_40K`
      - 40 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_384K`
      - 384 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_448K`
      - 448 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_512K`
      - 512 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_576K`
      - 576 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_640K`
      - 640 kbit/s



`V4L2_CID_MPEG_AUDIO_MODE`
    (enum)

enum v4l2_mpeg_audio_mode -
    MPEG 闊抽妯″紡銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_MODE_STEREO`
      - 绔嬩綋澹?
    - - `V4L2_MPEG_AUDIO_MODE_JOINT_STEREO`
      - 鑱斿悎绔嬩綋澹?
    - - `V4L2_MPEG_AUDIO_MODE_DUAL`
      - 鍙岃
    - - `V4L2_MPEG_AUDIO_MODE_MONO`
      - 鍗曞０閬?

`V4L2_CID_MPEG_AUDIO_MODE_EXTENSION`
    (enum)

enum v4l2_mpeg_audio_mode_extension -
    鑱斿悎绔嬩綋澹伴煶棰戞ā寮忔墿灞曘€傚湪绗竴灞傚拰绗簩灞備腑锛屽畠浠寚绀哄摢浜涘瓙甯﹂噰鐢ㄥ己搴︾珛浣撳０锛坕ntensity stereo锛夈€傚叾浣欏瓙甯︿互绔嬩綋澹扮紪鐮併€傜涓夊眰灏氫笉鏀寔锛堟垨鏈敮鎸侊級銆傚彲鑳界殑鍊煎涓嬶細


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_4`
      - 瀛愬甫 4-31 閲囩敤寮哄害绔嬩綋澹?
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_8`
      - 瀛愬甫 8-31 閲囩敤寮哄害绔嬩綋澹?
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_12`
      - 瀛愬甫 12-31 閲囩敤寮哄害绔嬩綋澹?
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_16`
      - 瀛愬甫 16-31 閲囩敤寮哄害绔嬩綋澹?



`V4L2_CID_MPEG_AUDIO_EMPHASIS`
    (enum)

enum v4l2_mpeg_audio_emphasis -
    闊抽棰勫姞閲嶏紙Emphasis锛夈€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_EMPHASIS_NONE`
      - 鏃?
    - - `V4L2_MPEG_AUDIO_EMPHASIS_50_DIV_15_uS`
      - 50/15 寰棰勫姞閲?
    - - `V4L2_MPEG_AUDIO_EMPHASIS_CCITT_J17`
      - CCITT J.17



`V4L2_CID_MPEG_AUDIO_CRC`
    (enum)

enum v4l2_mpeg_audio_crc -
    CRC 鏂规硶銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_CRC_NONE`
      - 鏃?
    - - `V4L2_MPEG_AUDIO_CRC_CRC16`
      - 16 浣嶅鍋舵牎楠?



`V4L2_CID_MPEG_AUDIO_MUTE (boolean)`
    閲囬泦鏃堕潤闊抽煶棰戙€傝繖涓嶆槸閫氳繃闈欓煶闊抽纭欢鏉ュ疄鐜扮殑锛堢‖浠堕潤闊充粛鍙兘浜х敓杞诲井鍢跺樁澹帮級锛岃€屾槸鍦ㄧ紪鐮佸櫒鍐呴儴瀹屾垚锛屼粠鑰屼繚璇佸浐瀹氱殑銆佸彲澶嶇幇鐨勯煶棰戠爜娴併€? = 闈為潤闊筹紝1 = 闈欓煶銆?


`V4L2_CID_MPEG_AUDIO_DEC_PLAYBACK`
    (enum)

enum v4l2_mpeg_audio_dec_playback -
    鍐冲畾鍗曡锛坢onolingual锛夐煶棰戝簲濡備綍鎾斁銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_AUTO`
      - 鑷姩纭畾鏈€浣虫挱鏀炬ā寮忋€?
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_STEREO`
      - 绔嬩綋澹版挱鏀俱€?
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_LEFT`
      - 宸﹀０閬撴挱鏀俱€?
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_RIGHT`
      - 鍙冲０閬撴挱鏀俱€?
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_MONO`
      - 鍗曞０閬撴挱鏀俱€?
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_SWAPPED_STEREO`
      - 宸﹀彸澹伴亾浜掓崲鐨勭珛浣撳０鎾斁銆?



`V4L2_CID_MPEG_AUDIO_DEC_MULTILINGUAL_PLAYBACK`
    (enum)

enum v4l2_mpeg_audio_dec_playback -
    鍐冲畾澶氳瑷€闊抽搴斿浣曟挱鏀俱€?


`V4L2_CID_MPEG_VIDEO_ENCODING`
    (enum)

enum v4l2_mpeg_video_encoding -
    MPEG 瑙嗛缂栫爜鏂规硶銆傝鎺т欢涓撶敤浜庡璺鐢ㄧ殑 MPEG 娴併€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_1`
      - MPEG-1 瑙嗛缂栫爜
    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_2`
      - MPEG-2 瑙嗛缂栫爜
    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC`
      - MPEG-4 AVC锛圚.264锛夎棰戠紪鐮?



`V4L2_CID_MPEG_VIDEO_ASPECT`
    (enum)

enum v4l2_mpeg_video_aspect -
    瑙嗛瀹介珮姣斻€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_ASPECT_1x1`
    - - `V4L2_MPEG_VIDEO_ASPECT_4x3`
    - - `V4L2_MPEG_VIDEO_ASPECT_16x9`
    - - `V4L2_MPEG_VIDEO_ASPECT_221x100`



`V4L2_CID_MPEG_VIDEO_B_FRAMES (integer)`
    B 甯ф暟閲忥紙榛樿 2锛?

`V4L2_CID_MPEG_VIDEO_GOP_SIZE (integer)`
    GOP 澶у皬锛堥粯璁?12锛?

`V4L2_CID_MPEG_VIDEO_GOP_CLOSURE (boolean)`
    GOP 闂悎锛堥粯璁?1锛?

`V4L2_CID_MPEG_VIDEO_PULLDOWN (boolean)`
    鍚敤 3:2 涓嬫媺锛坧ulldown锛夛紙榛樿 0锛?


`V4L2_CID_MPEG_VIDEO_BITRATE_MODE`
    (enum)

enum v4l2_mpeg_video_bitrate_mode -
    瑙嗛姣旂壒鐜囨ā寮忋€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_VBR`
      - 鍙彉姣旂壒鐜?
    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_CBR`
      - 鎭掑畾姣旂壒鐜?
    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_CQ`
      - 鎭掑畾璐ㄩ噺



`V4L2_CID_MPEG_VIDEO_BITRATE (integer)`
    骞冲潎瑙嗛姣旂壒鐜囷紝鍗曚綅涓烘瘮鐗规瘡绉掋€?

`V4L2_CID_MPEG_VIDEO_BITRATE_PEAK (integer)`
    宄板€艰棰戞瘮鐗圭巼锛屽崟浣嶄负姣旂壒姣忕銆傚繀椤诲ぇ浜庢垨绛変簬骞冲潎瑙嗛姣旂壒鐜囥€傚鏋滆棰戞瘮鐗圭巼妯″紡璁剧疆涓烘亽瀹氭瘮鐗圭巼锛屽垯璇ユ帶浠惰蹇界暐銆?

`V4L2_CID_MPEG_VIDEO_CONSTANT_QUALITY (integer)`
    鎭掑畾璐ㄩ噺绛夌骇鎺у埗銆傚綋 `V4L2_CID_MPEG_VIDEO_BITRATE_MODE` 鐨勫€间负 `V4L2_MPEG_VIDEO_BITRATE_MODE_CQ` 鏃堕€傜敤姝ゆ帶浠躲€傛湁鏁堣寖鍥翠负 1 鍒?100锛屽叾涓?1 琛ㄧず鏈€浣庤川閲忥紝100 琛ㄧず鏈€楂樿川閲忋€傜紪鐮佸櫒灏嗗喅瀹氶€傚綋鐨勯噺鍖栧弬鏁板拰姣旂壒鐜囷紝浠ヤ骇鐢熸墍璇锋眰鐨勫抚璐ㄩ噺銆?


`V4L2_CID_MPEG_VIDEO_FRAME_SKIP_MODE (enum)`

enum v4l2_mpeg_video_frame_skip_mode -
    鎸囩ず缂栫爜鍣ㄥ湪浣曠鏉′欢涓嬪簲璺宠繃甯с€傚鏋滅紪鐮佹煇涓€甯т細瀵艰嚧缂栫爜鍚庣殑娴佸ぇ浜庢墍閫夌殑鏁版嵁闄愬埗锛屽垯璇ュ抚灏嗚璺宠繃銆傚彲鑳界殑鍊煎涓嬶細



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_DISABLED`
      - 甯ц烦杩囨ā寮忓凡绂佺敤銆?
    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT`
      - 甯ц烦杩囨ā寮忓凡鍚敤锛岀紦鍐插尯闄愬埗鐢辨墍閫夌瓑绾ц瀹氾紝骞剁敱鏍囧噯瀹氫箟銆?
    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT`
      - 甯ц烦杩囨ā寮忓凡鍚敤锛岀紦鍐插尯闄愬埗鐢?VBV锛圡PEG1/2/4锛?<v4l2-mpeg-video-vbv-size> 鎴?CPB锛圚264锛夌紦鍐插尯澶у皬 <v4l2-mpeg-video-h264-cpb-size> 鎺т欢璁惧畾銆?


    \normalsize

`V4L2_CID_MPEG_VIDEO_TEMPORAL_DECIMATION (integer)`
    瀵逛簬姣忎竴甯ч噰闆嗗埌鐨勫抚锛岃烦杩囧叾鍚庤繖涔堝甯э紙榛樿 0锛夈€?

`V4L2_CID_MPEG_VIDEO_MUTE (boolean)`
    閲囬泦鏃跺皢瑙嗛鈥滈潤闊斥€濅负鍥哄畾棰滆壊銆傝繖瀵逛簬娴嬭瘯浠ヤ骇鐢熷浐瀹氱殑瑙嗛鐮佹祦寰堟湁鐢ㄣ€? = 闈為潤闊筹紝1 = 闈欓煶銆?

`V4L2_CID_MPEG_VIDEO_MUTE_YUV (integer)`
    璁剧疆瑙嗛鐨勨€滈潤闊斥€濋鑹层€傛墍鎻愪緵鐨?32 浣嶆暣鏁版寜濡備笅鏂瑰紡瑙ｉ噴锛坆it 0 = 鏈€浣庢湁鏁堜綅锛夛細



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:7
      - V 鑹插害淇℃伅
    - - Bit 8:15
      - U 鑹插害淇℃伅
    - - Bit 16:23
      - Y 浜害淇℃伅
    - - Bit 24:31
      - 蹇呴』涓洪浂銆?



`V4L2_CID_MPEG_VIDEO_DEC_PTS (integer64)`
    杩欎釜鍙鎺т欢杩斿洖褰撳墠鏄剧ず甯х殑 33 浣嶈棰戞樉绀烘椂闂存埑锛圥resentation Time Stamp锛夛紝鍏跺畾涔夎 ITU T-REC-H.222.0 鍜?ISO/IEC 13818-1銆傚畠涓?VIDIOC_DECODER_CMD 涓墍鐢ㄧ殑 PTS 鐩稿悓銆?


`V4L2_CID_MPEG_VIDEO_DEC_FRAME (integer64)`
    杩欎釜鍙鎺т欢杩斿洖褰撳墠鏄剧ず锛堝凡瑙ｇ爜锛夊抚鐨勫抚璁℃暟鍣ㄣ€傛瘡褰撹В鐮佸櫒鍚姩鏃讹紝璇ュ€间細琚噸缃负 0銆?

`V4L2_CID_MPEG_VIDEO_DEC_CONCEAL_COLOR (integer64)`
    姝ゆ帶浠惰缃?YUV 鑹插僵绌洪棿涓殑闅愯棌锛坈onceal锛夐鑹层€傚畠鎻忚堪鍦ㄥ弬鑰冨抚缂哄け瀵艰嚧鍑洪敊鏃讹紝瀹㈡埛绔閿欒闅愯棌棰滆壊鐨勫亸濂姐€傝В鐮佸櫒搴斾娇鐢ㄥ亸濂介鑹插～鍏呭弬鑰冪紦鍐插尯锛屽苟灏嗗叾鐢ㄤ簬鍚庣画瑙ｇ爜銆傝鎺т欢姣忎釜閫氶亾浣跨敤 16 浣嶃€傞€傜敤浜庤В鐮佸櫒銆?

    :header-rows:  0
    :stub-columns: 0

    - -
      - 8 浣嶆牸寮?
      - 10 浣嶆牸寮?
      - 12 浣嶆牸寮?
    - - Y 浜害
      - Bit 0:7
      - Bit 0:9
      - Bit 0:11
    - - Cb 鑹插害
      - Bit 16:23
      - Bit 16:25
      - Bit 16:27
    - - Cr 鑹插害
      - Bit 32:39
      - Bit 32:41
      - Bit 32:43
    - - 蹇呴』涓洪浂
      - Bit 48:63
      - Bit 48:63
      - Bit 48:63

`V4L2_CID_MPEG_VIDEO_DECODER_SLICE_INTERFACE (boolean)`
    濡傛灉鍚敤锛岃В鐮佸櫒鏈熸湜姣忎釜缂撳啿鍖烘帴鏀跺崟涓垏鐗囷紙slice锛夛紱鍚﹀垯瑙ｇ爜鍣ㄦ湡鏈涙瘡涓紦鍐插尯鎺ユ敹鍗曞抚銆傞€傜敤浜庤В鐮佸櫒锛屾墍鏈夌紪瑙ｇ爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE (boolean)`
    濡傛灉鍚敤浜嗘樉绀哄欢杩燂紝鍒欒В鐮佸櫒鍦ㄥ鐞嗕竴瀹氭暟閲忕殑 OUTPUT 缂撳啿鍖哄悗锛岃杩繑鍥炰竴涓?CAPTURE 缂撳啿鍖猴紙宸茶В鐮佸抚锛夈€傝寤惰繜鍙€氳繃 `V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY` 璁剧疆銆備緥濡傦紝姝ょ壒鎬у彲鐢ㄤ簬鐢熸垚瑙嗛缂╃暐鍥俱€傞€傜敤浜庤В鐮佸櫒銆?

`V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY (integer)`
    瑙ｇ爜鍣ㄧ殑鏄剧ず寤惰繜鍊笺€傝В鐮佸櫒鍦ㄨ瀹氱殑鈥滄樉绀哄欢杩熲€濆抚鏁颁箣鍚庤杩繑鍥炰竴甯у凡瑙ｇ爜甯с€傚鏋滆鏁板€艰緝灏忥紝鍙兘瀵艰嚧杩斿洖鐨勫抚涔卞簭鏄剧ず锛涙澶栫‖浠跺彲鑳戒粛灏嗚杩斿洖缂撳啿鍖虹敤浣滃悗缁抚鐨勫弬鑰冨浘鍍忋€?

`V4L2_CID_MPEG_VIDEO_AU_DELIMITER (boolean)`
    濡傛灉鍚敤锛屽皢鐢熸垚 AUD锛堣闂崟鍏冨畾鐣岀锛孉ccess Unit Delimiter锛塏ALU銆傝繖鍦ㄦ棤闇€瀹屽叏瑙ｆ瀽姣忎釜 NALU 鍗冲彲鎵惧埌甯ц捣濮嬫椂寰堟湁鐢ㄣ€傞€傜敤浜?H264 鍜?HEVC 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_VUI_SAR_ENABLE (boolean)`
    鍚敤鍦ㄨ棰戝彲鐢ㄦ€т俊鎭紙Video Usability Information锛変腑鍐欏叆閲囨牱瀹介珮姣斻€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_VUI_SAR_IDC`
    (enum)

enum v4l2_mpeg_video_h264_vui_sar_idc -
    鐢ㄤ簬 H.264 缂栫爜鐨?VUI 閲囨牱瀹介珮姣旀寚绀虹銆傝鍊煎湪鏍囧噯琛?E-1 涓畾涔夈€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_UNSPECIFIED`
      - 鏈寚瀹?
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_1x1`
      - 1x1
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_12x11`
      - 12x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_10x11`
      - 10x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_16x11`
      - 16x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_40x33`
      - 40x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_24x11`
      - 24x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_20x11`
      - 20x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_32x11`
      - 32x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_80x33`
      - 80x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_18x11`
      - 18x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_15x11`
      - 15x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_64x33`
      - 64x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_160x99`
      - 160x99
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_4x3`
      - 4x3
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_3x2`
      - 3x2
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_2x1`
      - 2x1
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_EXTENDED`
      - 鎵╁睍 SAR


`V4L2_CID_MPEG_VIDEO_H264_VUI_EXT_SAR_WIDTH (integer)`
    鐢ㄤ簬 H.264 VUI 缂栫爜鐨勬墿灞曢噰鏍峰楂樻瘮瀹藉害銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_VUI_EXT_SAR_HEIGHT (integer)`
    鐢ㄤ簬 H.264 VUI 缂栫爜鐨勬墿灞曢噰鏍峰楂樻瘮楂樺害銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_LEVEL`
    (enum)

enum v4l2_mpeg_video_h264_level -
    H264 瑙嗛鍩烘湰娴佺殑绛夌骇锛坙evel锛変俊鎭€傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_0`
      - Level 1.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1B`
      - Level 1B
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_1`
      - Level 1.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_2`
      - Level 1.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_3`
      - Level 1.3
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_0`
      - Level 2.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_1`
      - Level 2.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_2`
      - Level 2.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_0`
      - Level 3.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_1`
      - Level 3.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_2`
      - Level 3.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_0`
      - Level 4.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_1`
      - Level 4.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_2`
      - Level 4.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_0`
      - Level 5.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_1`
      - Level 5.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_2`
      - Level 5.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_0`
      - Level 6.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_1`
      - Level 6.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_2`
      - Level 6.2



`V4L2_CID_MPEG_VIDEO_MPEG2_LEVEL`
    (enum)

enum v4l2_mpeg_video_mpeg2_level -
    MPEG2 鍩烘湰娴佺殑绛夌骇淇℃伅銆傞€傜敤浜?MPEG2 缂栬В鐮佸櫒銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_LOW`
      - 浣庣瓑绾э紙LL锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_MAIN`
      - 涓荤瓑绾э紙ML锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH_1440`
      - 楂?1440 绛夌骇锛圚-14锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH`
      - 楂樼瓑绾э紙HL锛?

`V4L2_CID_MPEG_VIDEO_MPEG4_LEVEL`
    (enum)

enum v4l2_mpeg_video_mpeg4_level -
    MPEG4 鍩烘湰娴佺殑绛夌骇淇℃伅銆傞€傜敤浜?MPEG4 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_0`
      - Level 0
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_0B`
      - Level 0b
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_1`
      - Level 1
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_2`
      - Level 2
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_3`
      - Level 3
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_3B`
      - Level 3b
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_4`
      - Level 4
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_5`
      - Level 5



`V4L2_CID_MPEG_VIDEO_H264_PROFILE`
    (enum)

enum v4l2_mpeg_video_h264_profile -
    H264 鐨勬。娆★紙profile锛変俊鎭€傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_PROFILE_BASELINE`
      - 鍩虹嚎妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_BASELINE`
      - 鍙楅檺鍩虹嚎妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_MAIN`
      - 涓绘。娆?
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_EXTENDED`
      - 鎵╁睍妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH`
      - 楂樻。娆?
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10`
      - 楂?10 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422`
      - 楂?422 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_PREDICTIVE`
      - 楂?444 棰勬祴妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10_INTRA`
      - 楂?10 Intra 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422_INTRA`
      - 楂?422 Intra 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_INTRA`
      - 楂?444 Intra 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CAVLC_444_INTRA`
      - CAVLC 444 Intra 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_BASELINE`
      - 鍙几缂╁熀绾挎。娆?
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH`
      - 鍙几缂╅珮妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH_INTRA`
      - 鍙几缂╅珮 Intra 妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_STEREO_HIGH`
      - 绔嬩綋澹伴珮妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_MULTIVIEW_HIGH`
      - 澶氳鐐归珮妗ｆ
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_HIGH`
      - 鍙楅檺楂樻。娆?


    \normalsize


`V4L2_CID_MPEG_VIDEO_MPEG2_PROFILE`
    (enum)

enum v4l2_mpeg_video_mpeg2_profile -
    MPEG2 鐨勬。娆′俊鎭€傞€傜敤浜?MPEG2 缂栬В鐮佸櫒銆傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SIMPLE`
      - 绠€鍗曟。娆★紙SP锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_MAIN`
      - 涓绘。娆★紙MP锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SNR_SCALABLE`
      - SNR 鍙几缂╂。娆★紙SNR锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SPATIALLY_SCALABLE`
      - 绌洪棿鍙几缂╂。娆★紙Spt锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_HIGH`
      - 楂樻。娆★紙HP锛?
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_MULTIVIEW`
      - 澶氳鐐规。娆★紙MVP锛?



    \normalsize


`V4L2_CID_MPEG_VIDEO_MPEG4_PROFILE`
    (enum)

enum v4l2_mpeg_video_mpeg4_profile -
    MPEG4 鐨勬。娆′俊鎭€傞€傜敤浜?MPEG4 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE`
      - 绠€鍗曟。娆?
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_SIMPLE`
      - 楂樼骇绠€鍗曟。娆?
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_CORE`
      - 鏍稿績妗ｆ
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE_SCALABLE`
      - 绠€鍗曞彲浼哥缉妗ｆ
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_CODING_EFFICIENCY`
      - 楂樼骇缂栫爜鏁堢巼妗ｆ


    \normalsize

`V4L2_CID_MPEG_VIDEO_MAX_REF_PIC (integer)`
    鐢ㄤ簬缂栫爜鐨勫弬鑰冨浘鍍忕殑鏈€澶ф暟閲忋€傞€傜敤浜庣紪鐮佸櫒銆?


`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE`
    (enum)

enum v4l2_mpeg_video_multi_slice_mode -
    鍐冲畾缂栫爜鍣ㄥ簲濡備綍灏嗗抚鍒掑垎涓哄垏鐗囷紙slice锛夈€傞€傜敤浜庣紪鐮佸櫒銆傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_SINGLE`
      - 姣忓抚鍗曚釜鍒囩墖銆?
    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_MB`
      - 澶氫釜鍒囩墖锛屾瘡涓垏鐗囪瀹氭渶澶у畯鍧楁暟銆?
    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_BYTES`
      - 澶氫釜鍒囩墖锛屾瘡涓垏鐗囪瀹氭渶澶у瓧鑺傛暟銆?



`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MAX_MB (integer)`
    涓€涓垏鐗囦腑鐨勬渶澶у畯鍧楁暟銆傚綋 `V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE` 璁剧疆涓?`V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_MB` 鏃朵娇鐢ㄣ€傞€傜敤浜庣紪鐮佸櫒銆?

`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MAX_BYTES (integer)`
    涓€涓垏鐗囩殑鏈€澶у瓧鑺傛暟銆傚綋 `V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE` 璁剧疆涓?`V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_BYTES` 鏃朵娇鐢ㄣ€傞€傜敤浜庣紪鐮佸櫒銆?


`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_MODE`
    (enum)

enum v4l2_mpeg_video_h264_loop_filter_mode -
    H264 缂栫爜鍣ㄧ殑鐜唴婊ゆ尝鍣ㄦā寮忋€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_ENABLED`
      - 鐜唴婊ゆ尝鍣ㄥ凡鍚敤銆?
    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED`
      - 鐜唴婊ゆ尝鍣ㄥ凡绂佺敤銆?
    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY`
      - 鍦ㄥ垏鐗囪竟鐣屽绂佺敤鐜唴婊ゆ尝鍣ㄣ€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_ALPHA (integer)`
    鐜唴婊ゆ尝鍣?alpha 绯绘暟锛屽畾涔変簬 H264 鏍囧噯銆傝鍊煎搴斾簬 slice header 瀛楁 slice_alpha_c0_offset_div2锛屽彇鍊艰寖鍥村簲涓?-6 鍒?+6锛堝惈锛夈€傚疄闄呯殑 alpha 鍋忕Щ FilterOffsetA 鏄鍊肩殑涓ゅ€嶃€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_BETA (integer)`
    鐜唴婊ゆ尝鍣?beta 绯绘暟锛屽畾涔変簬 H264 鏍囧噯銆傝鍊煎搴斾簬 slice header 瀛楁 slice_beta_offset_div2锛屽彇鍊艰寖鍥村簲涓?-6 鍒?+6锛堝惈锛夈€傚疄闄呯殑 beta 鍋忕Щ FilterOffsetB 鏄鍊肩殑涓ゅ€嶃€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_ENTROPY_MODE`
    (enum)

enum v4l2_mpeg_video_h264_entropy_mode -
    H264 鐨勭喌缂栫爜妯″紡 - CABAC/CAVLC銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CAVLC`
      - 浣跨敤 CAVLC 鐔电紪鐮併€?
    - - `V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CABAC`
      - 浣跨敤 CABAC 鐔电紪鐮併€?


`V4L2_CID_MPEG_VIDEO_H264_8X8_TRANSFORM (boolean)`
    涓?H264 鍚敤 8X8 鍙樻崲銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_CONSTRAINED_INTRA_PREDICTION (boolean)`
    涓?H264 鍚敤鍙楅檺甯у唴棰勬祴銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_CHROMA_QP_INDEX_OFFSET (integer)`
    鎸囧畾搴斿姞鍒颁寒搴﹂噺鍖栧弬鏁颁笂浠ョ‘瀹氳壊搴﹂噺鍖栧弬鏁扮殑鍋忕Щ閲忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_CYCLIC_INTRA_REFRESH_MB (integer)`
    寰幆甯у唴瀹忓潡鍒锋柊銆傝繖鏄瘡甯у埛鏂扮殑杩炵画瀹忓潡鏁伴噺銆傛瘡涓€甯т緷娆″埛鏂颁竴缁勫畯鍧楋紝鐩村埌鏁翠釜寰幆瀹屾垚骞朵粠甯ч《閮ㄩ噸鏂板紑濮嬨€傚皢姝ゆ帶浠惰涓洪浂琛ㄧず涓嶅埛鏂板畯鍧椼€傛敞鎰忥紝褰?`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD` 鎺т欢琚涓洪潪闆跺€兼椂锛屾鎺т欢灏嗕笉璧蜂綔鐢ㄣ€傞€傜敤浜?H264銆丠263 鍜?MPEG4 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE (enum)`

enum v4l2_mpeg_video_intra_refresh_period_type -
    璁剧疆甯у唴鍒锋柊鐨勭被鍨嬨€傚埛鏂版暣涓抚鐨勫懆鏈熺敱 V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD 鎸囧畾銆傛敞鎰忥紝濡傛灉涓嶅瓨鍦ㄦ鎺т欢锛屽垯鎵€浣跨敤鐨勫埛鏂扮被鍨嬫湭瀹氫箟锛岀敱椹卞姩鍐冲畾銆傞€傜敤浜?H264 鍜?HEVC 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_RANDOM`
      - 鍦ㄦ寚瀹氬懆鏈熷悗锛屾暣涓抚琚殢鏈哄湴瀹屽叏鍒锋柊銆?
    - - `V4L2_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_CYCLIC`
      - 鍦ㄦ寚瀹氬懆鏈熷悗锛屾暣涓抚鐨勫畯鍧楁寜寰幆椤哄簭琚畬鍏ㄥ埛鏂般€?

`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD (integer)`
    甯у唴瀹忓潡鍒锋柊鍛ㄦ湡銆傚畠璁剧疆鍒锋柊鏁翠釜甯х殑鍛ㄦ湡銆傛崲鍙ヨ瘽璇达紝瀹冨畾涔変簡鏁翠釜甯у皢琚抚鍐呭埛鏂扮殑甯ф暟銆備緥濡傦細灏嗗懆鏈熻涓?1 琛ㄧず鏁翠釜甯у皢琚埛鏂帮紱璁句负 2 琛ㄧず涓€鍗婂畯鍧楀湪 frameX 涓婅繘琛屽抚鍐呭埛鏂帮紝鍙︿竴鍗婂畯鍧楀湪 frameX + 1 涓婂埛鏂帮紝渚濇绫绘帹銆傚皢鍛ㄦ湡璁句负闆惰〃绀烘湭鎸囧畾鍛ㄦ湡銆傛敞鎰忥紝濡傛灉瀹㈡埛绔皢姝ゆ帶浠惰涓洪潪闆跺€硷紝鍒?`V4L2_CID_MPEG_VIDEO_CYCLIC_INTRA_REFRESH_MB` 鎺т欢搴旇蹇界暐銆傞€傜敤浜?H264 鍜?HEVC 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_FRAME_RC_ENABLE (boolean)`
    甯х骇鐮佺巼鎺у埗浣胯兘銆傚鏋滅鐢ㄦ鎺т欢锛屽垯姣忕甯х被鍨嬬殑閲忓寲鍙傛暟涓哄父閲忥紝骞堕€氳繃鐩稿簲鎺т欢璁剧疆锛堜緥濡?`V4L2_CID_MPEG_VIDEO_H263_I_FRAME_QP`锛夈€傚鏋滃惎鐢ㄥ抚鐮佺巼鎺у埗锛屽垯閲忓寲鍙傛暟浼氳璋冩暣浠ユ弧瓒虫墍閫夋瘮鐗圭巼銆傞噺鍖栧弬鏁扮殑鏈€灏忓€煎拰鏈€澶у€煎彲閫氳繃鐩稿簲鎺т欢璁剧疆锛堜緥濡?`V4L2_CID_MPEG_VIDEO_H263_MIN_QP`锛夈€傞€傜敤浜庣紪鐮佸櫒銆?

`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE (boolean)`
    瀹忓潡绾х爜鐜囨帶鍒朵娇鑳姐€傞€傜敤浜?MPEG4 鍜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_MPEG4_QPEL (boolean)`
    MPEG4 鐨?1/4 鍍忕礌杩愬姩浼拌銆傞€傜敤浜?MPEG4 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H263_I_FRAME_QP (integer)`
    H263 鐨?I 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_H263_MIN_QP (integer)`
    H263 鐨勬渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_H263_MAX_QP (integer)`
    H263 鐨勬渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_H263_P_FRAME_QP (integer)`
    H263 鐨?P 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_H263_B_FRAME_QP (integer)`
    H263 鐨?B 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_QP (integer)`
    H264 鐨?I 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆?

`V4L2_CID_MPEG_VIDEO_H264_MIN_QP (integer)`
    H264 鐨勬渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆?

`V4L2_CID_MPEG_VIDEO_H264_MAX_QP (integer)`
    H264 鐨勬渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆?

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_QP (integer)`
    H264 鐨?P 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆?

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_QP (integer)`
    H264 鐨?B 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆?

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 I 甯ц川閲忚寖鍥寸殑 H264 I 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 I 甯ц川閲忚寖鍥寸殑 H264 I 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 P 甯ц川閲忚寖鍥寸殑 H264 P 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 P 甯ц川閲忚寖鍥寸殑 H264 P 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 B 甯ц川閲忚寖鍥寸殑 H264 B 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 H264 B 甯ц川閲忚寖鍥寸殑 H264 B 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細0 鍒?51銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_H264_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_MPEG4_I_FRAME_QP (integer)`
    MPEG4 鐨?I 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_MPEG4_MIN_QP (integer)`
    MPEG4 鐨勬渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_MPEG4_MAX_QP (integer)`
    MPEG4 鐨勬渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_MPEG4_P_FRAME_QP (integer)`
    MPEG4 鐨?P 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_MPEG4_B_FRAME_QP (integer)`
    MPEG4 鐨?B 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?


`V4L2_CID_MPEG_VIDEO_VBV_SIZE (integer)`
    瑙嗛缂撳啿鏍￠獙鍣紙Video Buffer Verifier锛夊ぇ灏忥紝鍗曚綅涓哄崈瀛楄妭锛岀敤浣滃抚璺宠繃鐨勯檺鍒躲€俈BV 鍦ㄦ爣鍑嗕腑琚畾涔変负涓€绉嶉獙璇佹墍浜х敓鐮佹祦鑳藉惁琚垚鍔熻В鐮佺殑鎵嬫銆傛爣鍑嗗皢鍏舵弿杩颁负鈥滀竴涓亣璁捐В鐮佸櫒鐨勪竴閮ㄥ垎锛屽湪姒傚康涓婅繛鎺ュ埌缂栫爜鍣ㄧ殑杈撳嚭銆傚叾鐩殑鏄缂栫爜鍣ㄦ垨缂栬緫杩囩▼鍙兘浜х敓鐨勬暟鎹€熺巼鐨勫彉鍖栨€ф柦鍔犵害鏉熴€傗€濄€傞€傜敤浜?MPEG1銆丮PEG2銆丮PEG4 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_VBV_DELAY (integer)`
    涓?VBV 缂撳啿鍖烘帶鍒惰缃垵濮嬪欢杩燂紝鍗曚綅涓烘绉掋€?


`V4L2_CID_MPEG_VIDEO_MV_H_SEARCH_RANGE (integer)`
    姘村钩鎼滅储鑼冨洿瀹氫箟浜嗗湪褰撳墠瀹忓潡锛圡B锛変簬鍙傝€冨浘鍍忎腑鎼滅储鍜屽尮閰嶆椂鐨勬渶澶ф按骞虫悳绱㈠尯鍩燂紙浠ュ儚绱犺锛夈€傛 V4L2 鎺т欢瀹忕敤浜庤缃棰戠紪鐮佸櫒涓繍鍔ㄤ及璁℃ā鍧楃殑姘村钩鎼滅储鑼冨洿銆?


`V4L2_CID_MPEG_VIDEO_MV_V_SEARCH_RANGE (integer)`
    鍨傜洿鎼滅储鑼冨洿瀹氫箟浜嗗湪褰撳墠瀹忓潡锛圡B锛変簬鍙傝€冨浘鍍忎腑鎼滅储鍜屽尮閰嶆椂鐨勬渶澶у瀭鐩存悳绱㈠尯鍩燂紙浠ュ儚绱犺锛夈€傛 V4L2 鎺т欢瀹忕敤浜庤缃棰戠紪鐮佸櫒涓繍鍔ㄤ及璁℃ā鍧楃殑鍨傜洿鎼滅储鑼冨洿銆?


`V4L2_CID_MPEG_VIDEO_FORCE_KEY_FRAME (button)`
    涓轰笅涓€涓帓闃熺殑缂撳啿鍖哄己鍒剁敓鎴愬叧閿抚銆傞€傜敤浜庣紪鐮佸櫒銆傝繖鏄竴涓€氱敤鐨勩€佷笌缂栬В鐮佸櫒鏃犲叧鐨勫己鍒跺叧閿抚鎺т欢銆?


`V4L2_CID_MPEG_VIDEO_H264_CPB_SIZE (integer)`
    缂栫爜鍥惧儚缂撳啿鍖猴紙Coded Picture Buffer锛夊ぇ灏忥紝鍗曚綅涓哄崈瀛楄妭锛岀敤浣滃抚璺宠繃鐨勯檺鍒躲€侰PB 鍦?H264 鏍囧噯涓瀹氫箟涓轰竴绉嶉獙璇佹墍浜х敓鐮佹祦鑳藉惁琚垚鍔熻В鐮佺殑鎵嬫銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_I_PERIOD (integer)`
    H264 鍦ㄥ紑鏀?GOP 涓?I 甯т箣闂寸殑鍛ㄦ湡銆傚浜庡紑鏀?GOP锛岃繖鏄袱涓?I 甯т箣闂寸殑鍛ㄦ湡銆侷DR锛圛nstantaneous Decoding Refresh锛屽嵆鏃惰В鐮佸埛鏂帮級甯т箣闂寸殑鍛ㄦ湡鍙栬嚜 GOP_SIZE 鎺т欢銆侷DR 甯т唬琛ㄥ嵆鏃惰В鐮佸埛鏂帮紝鏄竴绉嶅湪鍏朵箣鍚庝笉鍐嶅紩鐢ㄤ换浣曞厛鍓嶅抚鐨?I 甯с€傝繖鎰忓懗鐫€鍙互浠?IDR 甯ч噸鏂板紑濮嬬爜娴侊紝鑰屾棤闇€瀛樺偍鎴栬В鐮佷换浣曞厛鍓嶅抚銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_HEADER_MODE`
    (enum)

enum v4l2_mpeg_video_header_mode -
    鍐冲畾澶撮儴鏄綔涓虹涓€涓紦鍐插尯杩斿洖锛岃繕鏄笌绗竴甯т竴璧疯繑鍥炪€傞€傜敤浜庣紪鐮佸櫒銆傚彲鑳界殑鍊煎涓嬶細


    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEADER_MODE_SEPARATE`
      - 娴佸ご閮ㄥ湪绗竴涓紦鍐插尯涓崟鐙繑鍥炪€?
    - - `V4L2_MPEG_VIDEO_HEADER_MODE_JOINED_WITH_1ST_FRAME`
      - 娴佸ご閮ㄤ笌绗竴甯х紪鐮佸抚涓€璧疯繑鍥炪€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_REPEAT_SEQ_HEADER (boolean)`
    閲嶅瑙嗛搴忓垪澶淬€傞噸澶嶈繖浜涘ご閮ㄤ娇瀵硅棰戞祦鐨勯殢鏈鸿闂洿瀹规槗銆傞€傜敤浜?MPEG1銆? 鍜?4 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_DECODER_MPEG4_DEBLOCK_FILTER (boolean)`
    涓?MPEG4 瑙ｇ爜鍣ㄥ惎鐢ㄥ幓鍧楀悗澶勭悊婊ゆ尝鍣ㄣ€傞€傜敤浜?MPEG4 瑙ｇ爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_MPEG4_VOP_TIME_RES (integer)`
    MPEG4 鐨?vop_time_increment_resolution 鍊笺€傞€傜敤浜?MPEG4 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_MPEG4_VOP_TIME_INC (integer)`
    MPEG4 鐨?vop_time_increment 鍊笺€傞€傜敤浜?MPEG4 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_SEI_FRAME_PACKING (boolean)`
    鍦ㄧ紪鐮佺爜娴佷腑鍚敤鐢熸垚甯у皝瑁呰ˉ鍏呭寮轰俊鎭紙frame packing SEI锛夈€傚抚灏佽 SEI 娑堟伅鍖呭惈鐢ㄤ簬 3D 瑙傜湅鐨?L 鍜?R 骞抽潰鐨勬帓鍒楁柟寮忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_SEI_FP_CURRENT_FRAME_0 (boolean)`
    鍦ㄥ抚灏佽 SEI 涓皢褰撳墠甯ц涓?frame0銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE`
    (enum)

enum v4l2_mpeg_video_h264_sei_fp_arrangement_type -
    H264 SEI 鐨勫抚灏佽鎺掑垪鏂瑰紡绫诲瀷銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_CHEKERBOARD`
      - 鍍忕礌浜ゆ浛鏉ヨ嚜 L 鍜?R銆?
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_COLUMN`
      - L 鍜?R 鎸夊垪闅旇鎺掑垪銆?
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_ROW`
      - L 鍜?R 鎸夎闅旇鎺掑垪銆?
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_SIDE_BY_SIDE`
      - L 鍦ㄥ乏锛孯 鍦ㄥ彸銆?
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TOP_BOTTOM`
      - L 鍦ㄤ笂锛孯 鍦ㄤ笅銆?
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TEMPORAL`
      - 姣忓抚涓€涓鐐广€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_FMO (boolean)`
    鍦ㄧ紪鐮佺爜娴佷腑鍚敤鐏垫椿瀹忓潡鎺掑簭锛團MO锛夈€傝繖鏄竴绉嶇敤浜庨噸缁勫浘鍍忎腑瀹忓潡椤哄簭鐨勬妧鏈€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_FMO_MAP_TYPE`
   (enum)

enum v4l2_mpeg_video_h264_fmo_map_type -
    浣跨敤 FMO 鏃讹紝鏄犲皠绫诲瀷灏嗗浘鍍忓垝鍒嗕负涓嶅悓鐨勫畯鍧楁壂鎻忔ā寮忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_INTERLEAVED_SLICES`
      - 鍒囩墖鎸夋父绋嬮暱搴﹂『搴忓郊姝や氦缁囨帓鍒楀畯鍧椼€?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_SCATTERED_SLICES`
      - 鍩轰簬缂栫爜鍣ㄥ拰瑙ｇ爜鍣ㄥ弻鏂瑰潎宸茬煡鐨勬暟瀛﹀嚱鏁板垎鏁ｅ畯鍧椼€?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_FOREGROUND_WITH_LEFT_OVER`
      - 瀹忓潡鎺掑垪鍦ㄧ煩褰㈠尯鍩熸垨鎰熷叴瓒ｅ尯鍩熷唴銆?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_BOX_OUT`
      - 鍒囩墖缁勪粠涓績鍚戝浠ュ惊鐜柟寮忓闀裤€?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_RASTER_SCAN`
      - 鍒囩墖缁勬寜鍏夋爡鎵弿妯″紡浠庡乏鍒板彸澧為暱銆?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_WIPE_SCAN`
      - 鍒囩墖缁勬寜鎿﹂櫎鎵弿妯″紡浠庝笂鍒颁笅澧為暱銆?
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_EXPLICIT`
      - 鐢ㄦ埛鑷畾涔夋槧灏勭被鍨嬨€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_FMO_SLICE_GROUP (integer)`
    FMO 涓垏鐗囩粍鐨勬暟閲忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_FMO_CHANGE_DIRECTION`
    (enum)

enum v4l2_mpeg_video_h264_fmo_change_dir -
    鎸囧畾鍏夋爡鍜屾摝闄ゆ槧灏勭殑鍒囩墖缁勫彉鍖栨柟鍚戙€傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_RIGHT`
      - 鍏夋爡鎵弿鎴栧悜鍙虫摝闄ゃ€?
    - - `V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_LEFT`
      - 鍙嶅悜鍏夋爡鎵弿鎴栧悜宸︽摝闄ゃ€?



`V4L2_CID_MPEG_VIDEO_H264_FMO_CHANGE_RATE (integer)`
    鎸囧畾鍏夋爡鍜屾摝闄ゆ槧灏勪腑绗竴涓垏鐗囩粍鐨勫ぇ灏忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_FMO_RUN_LENGTH (integer)`
    鎸囧畾浜ょ粐鏄犲皠涓繛缁畯鍧楃殑鏁伴噺銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_ASO (boolean)`
    鍦ㄧ紪鐮佺爜娴佷腑鍚敤浠绘剰鍒囩墖鎺掑簭锛圓SO锛夈€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_ASO_SLICE_ORDER (integer)`
    鎸囧畾 ASO 涓殑鍒囩墖椤哄簭銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€傛墍鎻愪緵鐨?32 浣嶆暣鏁版寜濡備笅鏂瑰紡瑙ｉ噴锛坆it 0 = 鏈€浣庢湁鏁堜綅锛夛細



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:15
      - 鍒囩墖 ID
    - - Bit 16:32
      - 鍒囩墖浣嶇疆鎴栭『搴?



`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING (boolean)`
    鍚敤 H264 鍒嗗眰缂栫爜銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_TYPE`
    (enum)

enum v4l2_mpeg_video_h264_hierarchical_coding_type -
    鎸囧畾鍒嗗眰缂栫爜绫诲瀷銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_B`
      - 鍒嗗眰 B 缂栫爜銆?
    - - `V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_P`
      - 鍒嗗眰 P 缂栫爜銆?



`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_LAYER (integer)`
    鎸囧畾鍒嗗眰缂栫爜灞傜殑鏁伴噺銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_LAYER_QP (integer)`
    涓烘瘡涓€灞傛寚瀹氱敤鎴疯嚜瀹氫箟鐨?QP銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€傛墍鎻愪緵鐨?32 浣嶆暣鏁版寜濡備笅鏂瑰紡瑙ｉ噴锛坆it 0 = 鏈€浣庢湁鏁堜綅锛夛細



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:15
      - QP 鍊?
    - - Bit 16:32
      - 灞傜紪鍙?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L0_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 0 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L1_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 1 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L2_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 2 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L3_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 3 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L4_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 4 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L5_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 5 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L6_BR (integer)`
    鎸囩ず H264 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 6 鐨勬瘮鐗圭巼锛坆ps锛夈€?

`V4L2_CID_FWHT_I_FRAME_QP (integer)`
    FWHT 鐨?I 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_FWHT_P_FRAME_QP (integer)`
    FWHT 鐨?P 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細1 鍒?31銆?

`V4L2_CID_MPEG_VIDEO_AVERAGE_QP (integer)`
    杩欎釜鍙鎺т欢杩斿洖褰撳墠宸茬紪鐮佸抚鐨勫钩鍧?QP 鍊笺€傝鍊奸€傜敤浜庢渶鍚庝竴涓嚭闃熺殑鎹曡幏缂撳啿鍖猴紙VIDIOC_DQBUF锛夈€傚叾鏈夋晥鑼冨洿鍙栧喅浜庣紪鐮佹牸寮忓拰鍙傛暟銆傚浜?H264锛屾湁鏁堣寖鍥翠负 0 鍒?51銆傚浜?HEVC锛? 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚浜?H263 鍜?MPEG4锛屾湁鏁堣寖鍥翠负 1 鍒?31銆傚浜?VP8锛屾湁鏁堣寖鍥翠负 0 鍒?127銆傚浜?VP9锛屾湁鏁堣寖鍥翠负 0 鍒?255銆傚鏋滅紪瑙ｇ爜鍣ㄧ殑 MIN_QP 鍜?MAX_QP 宸茶缃紝鍒?QP 灏嗘弧瓒充袱鑰呯殑瑕佹眰銆傜紪瑙ｇ爜鍣ㄩ渶瑕佸缁堜娇鐢ㄦ寚瀹氱殑鑼冨洿锛岃€屼笉鏄‖浠惰嚜瀹氫箟鑼冨洿銆傞€傜敤浜庣紪鐮佸櫒


    \normalsize

## MFC 5.1 MPEG 鎺т欢


浠ヤ笅 MPEG 绫绘帶浠舵秹鍙婄壒瀹氫簬涓夋槦 S5P 绯诲垪 SoC 涓?Multi Format Codec 5.1 璁惧鐨?MPEG 瑙ｇ爜涓庣紪鐮佽缃€?



### MFC 5.1 鎺т欢 ID


`V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY_ENABLE (boolean)`
    濡傛灉鍚敤浜嗘樉绀哄欢杩燂紝鍒欒В鐮佸櫒鍦ㄥ鐞嗕竴瀹氭暟閲忕殑 OUTPUT 缂撳啿鍖哄悗锛岃杩繑鍥炰竴涓?CAPTURE 缂撳啿鍖猴紙宸茶В鐮佸抚锛夈€傝寤惰繜鍙€氳繃 `V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY` 璁剧疆銆備緥濡傦紝姝ょ壒鎬у彲鐢ㄤ簬鐢熸垚瑙嗛缂╃暐鍥俱€傞€傜敤浜?H264 瑙ｇ爜鍣ㄣ€?

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE`` control instead.

```
`V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY (integer)`
    鏄剧ず寤惰繜鍊硷紝鐢ㄤ簬 H264 瑙ｇ爜鍣ㄣ€傝В鐮佸櫒鍦ㄨ瀹氱殑鈥滄樉绀哄欢杩熲€濆抚鏁颁箣鍚庤杩繑鍥炰竴甯у凡瑙ｇ爜甯с€傚鏋滆鏁板€艰緝灏忥紝鍙兘瀵艰嚧杩斿洖鐨勫抚涔卞簭鏄剧ず锛涙澶栫‖浠跺彲鑳戒粛灏嗚杩斿洖缂撳啿鍖虹敤浣滃悗缁抚鐨勫弬鑰冨浘鍍忋€?

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY`` control instead.

```
`V4L2_CID_MPEG_MFC51_VIDEO_H264_NUM_REF_PIC_FOR_P (integer)`
    鐢ㄤ簬缂栫爜 P 甯х殑鍙傝€冨浘鍍忔暟閲忋€傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_MFC51_VIDEO_PADDING (boolean)`
    鍦ㄧ紪鐮佸櫒涓惎鐢ㄥ～鍏呪€斺€斾娇鐢ㄩ鑹茶€屼笉鏄噸澶嶈竟鐣屽儚绱犮€傞€傜敤浜庣紪鐮佸櫒銆?

`V4L2_CID_MPEG_MFC51_VIDEO_PADDING_YUV (integer)`
    缂栫爜鍣ㄤ腑鐨勫～鍏呴鑹层€傞€傜敤浜庣紪鐮佸櫒銆傛墍鎻愪緵鐨?32 浣嶆暣鏁版寜濡備笅鏂瑰紡瑙ｉ噴锛坆it 0 = 鏈€浣庢湁鏁堜綅锛夛細



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:7
      - V 鑹插害淇℃伅
    - - Bit 8:15
      - U 鑹插害淇℃伅
    - - Bit 16:23
      - Y 浜害淇℃伅
    - - Bit 24:31
      - 蹇呴』涓洪浂銆?



`V4L2_CID_MPEG_MFC51_VIDEO_RC_REACTION_COEFF (integer)`
    MFC 鐮佺巼鎺у埗鐨勫弽搴旂郴鏁般€傞€傜敤浜庣紪鐮佸櫒銆?

```

       #. Valid only when the frame level RC is enabled.

       #. For tight CBR, this field must be small (ex. 2 ~ 10). For
	  VBR, this field must be large (ex. 100 ~ 1000).

       #. It is not recommended to use the greater number than
	  FRAME_RATE * (10^9 / BIT_RATE).

```
`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_DARK (boolean)`
    閽堝鏆楀尯鍩熺殑鑷€傚簲鐮佺巼鎺у埗銆備粎褰撳惎鐢?H.264 鍜屽畯鍧楃骇鐮佺巼鎺у埗锛坄V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`锛夋椂鏈夋晥銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_SMOOTH (boolean)`
    閽堝骞虫粦鍖哄煙鐨勮嚜閫傚簲鐮佺巼鎺у埗銆備粎褰撳惎鐢?H.264 鍜屽畯鍧楃骇鐮佺巼鎺у埗锛坄V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`锛夋椂鏈夋晥銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_STATIC (boolean)`
    閽堝闈欐€佸尯鍩熺殑鑷€傚簲鐮佺巼鎺у埗銆備粎褰撳惎鐢?H.264 鍜屽畯鍧楃骇鐮佺巼鎺у埗锛坄V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`锛夋椂鏈夋晥銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_ACTIVITY (boolean)`
    閽堝娲诲姩鍖哄煙鐨勮嚜閫傚簲鐮佺巼鎺у埗銆備粎褰撳惎鐢?H.264 鍜屽畯鍧楃骇鐮佺巼鎺у埗锛坄V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`锛夋椂鏈夋晥銆傞€傜敤浜?H264 缂栫爜鍣ㄣ€?


`V4L2_CID_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE`
    (enum)

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_FRAME_SKIP_MODE`` control instead.

```
enum v4l2_mpeg_mfc51_video_frame_skip_mode -
    鎸囩ず缂栫爜鍣ㄥ湪浣曠鏉′欢涓嬪簲璺宠繃甯с€傚鏋滅紪鐮佹煇涓€甯т細瀵艰嚧缂栫爜鍚庣殑娴佸ぇ浜庢墍閫夌殑鏁版嵁闄愬埗锛屽垯璇ュ抚灏嗚璺宠繃銆傚彲鑳界殑鍊煎涓嬶細



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_DISABLED`
      - 甯ц烦杩囨ā寮忓凡绂佺敤銆?
    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT`
      - 甯ц烦杩囨ā寮忓凡鍚敤锛岀紦鍐插尯闄愬埗鐢辨墍閫夌瓑绾ц瀹氾紝骞剁敱鏍囧噯瀹氫箟銆?
    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT`
      - 甯ц烦杩囨ā寮忓凡鍚敤锛岀紦鍐插尯闄愬埗鐢?VBV锛圡PEG1/2/4锛夋垨 CPB锛圚264锛夌紦鍐插尯澶у皬鎺т欢璁惧畾銆?


    \normalsize

`V4L2_CID_MPEG_MFC51_VIDEO_RC_FIXED_TARGET_BIT (integer)`
    鍚敤鍏锋湁鍥哄畾鐩爣姣旂壒鐨勭爜鐜囨帶鍒躲€傚鏋滃惎鐢ㄦ璁剧疆锛岀紪鐮佸櫒鐨勭爜鐜囨帶鍒堕€昏緫灏嗕负 GOP 璁＄畻骞冲潎姣旂壒鐜囷紝骞朵娇鍏朵綆浜庢垨绛変簬璁惧畾鐨勬瘮鐗圭巼鐩爣銆傚惁鍒欙紝鐮佺巼鎺у埗閫昏緫璁＄畻鏁翠釜鐮佹祦鐨勬€讳綋骞冲潎姣旂壒鐜囷紝骞朵娇鍏朵綆浜庢垨绛変簬璁惧畾姣旂壒鐜囥€傚湪绗竴绉嶆儏鍐典笅锛屾暣涓爜娴佺殑骞冲潎姣旂壒鐜囧皢灏忎簬璁惧畾姣旂壒鐜囥€傝繖鏄洜涓哄钩鍧囧€兼槸鍩轰簬杈冨皯鐨勫抚鏁拌绠楃殑锛涘彟涓€鏂归潰锛屽惎鐢ㄦ璁剧疆鍙‘淇濈爜娴佹弧瓒充弗鏍肩殑甯﹀绾︽潫銆傞€傜敤浜庣紪鐮佸櫒銆?


`V4L2_CID_MPEG_MFC51_VIDEO_FORCE_FRAME_TYPE`
    (enum)

enum v4l2_mpeg_mfc51_video_force_frame_type -
    涓轰笅涓€涓帓闃熺殑缂撳啿鍖哄己鍒跺抚绫诲瀷銆傞€傜敤浜庣紪鐮佸櫒銆傚彲鑳界殑鍊煎涓嬶細


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_DISABLED`
      - 绂佺敤寮哄埗鐗瑰畾甯х被鍨嬨€?
    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_I_FRAME`
      - 寮哄埗 I 甯с€?
    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_NOT_CODED`
      - 寮哄埗闈炵紪鐮佸抚銆?


## CX2341x MPEG 鎺т欢


浠ヤ笅 MPEG 绫绘帶浠舵秹鍙婄壒瀹氫簬 Conexant CX23415 鍜?CX23416 MPEG 缂栫爜鑺墖鐨?MPEG 缂栫爜璁剧疆銆?



### CX2341x 鎺т欢 ID



`V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE`
    (enum)

enum v4l2_mpeg_cx2341x_video_spatial_filter_mode -
    璁剧疆绌洪棿婊ゆ尝鍣ㄦā寮忥紙榛樿 `MANUAL`锛夈€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_MANUAL`
      - 鎵嬪姩閫夋嫨婊ゆ尝鍣?
    - - `V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_AUTO`
      - 鑷姩閫夋嫨婊ゆ尝鍣?



`V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER (integer (0-15))`
    绌洪棿婊ゆ尝鍣ㄧ殑璁剧疆銆? = 鍏抽棴锛?5 = 鏈€澶с€傦紙榛樿 0銆傦級


`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_luma_spatial_filter_type -
    閫夋嫨鐢ㄤ簬浜害绌洪棿婊ゆ尝鍣ㄧ殑绠楁硶锛堥粯璁?`1D_HOR`锛夈€傚彲鑳界殑鍊硷細



    \footnotesize

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_OFF`
      - 鏃犳护娉㈠櫒
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_HOR`
      - 涓€缁存按骞?
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_VERT`
      - 涓€缁村瀭鐩?
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_HV_SEPARABLE`
      - 浜岀淮鍙垎绂?
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_SYM_NON_SEPARABLE`
      - 浜岀淮瀵圭О涓嶅彲鍒嗙


    \normalsize


`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_chroma_spatial_filter_type -
    閫夋嫨鐢ㄤ簬鑹插害绌洪棿婊ゆ尝鍣ㄧ殑绠楁硶锛堥粯璁?`1D_HOR`锛夈€傚彲鑳界殑鍊煎涓嬶細


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_OFF`
      - 鏃犳护娉㈠櫒
    - - `V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_1D_HOR`
      - 涓€缁存按骞?


    \normalsize


`V4L2_CID_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE`
    (enum)

enum v4l2_mpeg_cx2341x_video_temporal_filter_mode -
    璁剧疆鏃堕棿婊ゆ尝鍣ㄦā寮忥紙榛樿 `MANUAL`锛夈€傚彲鑳界殑鍊煎涓嬶細


    \footnotesize

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_MANUAL`
      - 鎵嬪姩閫夋嫨婊ゆ尝鍣?
    - - `V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_AUTO`
      - 鑷姩閫夋嫨婊ゆ尝鍣?


    \normalsize

`V4L2_CID_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER (integer (0-31))`
    鏃堕棿婊ゆ尝鍣ㄧ殑璁剧疆銆? = 鍏抽棴锛?1 = 鏈€澶с€傦紙鍏ㄥ垎杈ㄧ巼閲囬泦鏃堕粯璁?8锛岀缉鏀鹃噰闆嗘椂榛樿 0銆傦級

`V4L2_CID_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_median_filter_type -
    涓€兼护娉㈠櫒绫诲瀷锛堥粯璁?`OFF`锛夈€傚彲鑳界殑鍊煎涓嬶細



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_OFF`
      - 鏃犳护娉㈠櫒
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR`
      - 姘村钩婊ゆ尝鍣?
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_VERT`
      - 鍨傜洿婊ゆ尝鍣?
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR_VERT`
      - 姘村钩鍜屽瀭鐩存护娉㈠櫒
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_DIAG`
      - 瀵硅婊ゆ尝鍣?


    \normalsize

`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_MEDIAN_FILTER_BOTTOM (integer (0-255))`
    鍚敤浜害涓€兼护娉㈠櫒鐨勯槇鍊间笂闄愶紙榛樿 0锛?

`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_MEDIAN_FILTER_TOP (integer (0-255))`
    鍚敤浜害涓€兼护娉㈠櫒鐨勯槇鍊间笅闄愶紙榛樿 255锛?

`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_MEDIAN_FILTER_BOTTOM (integer (0-255))`
    鍚敤鑹插害涓€兼护娉㈠櫒鐨勯槇鍊间笂闄愶紙榛樿 0锛?

`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_MEDIAN_FILTER_TOP (integer (0-255))`
    鍚敤鑹插害涓€兼护娉㈠櫒鐨勯槇鍊间笅闄愶紙榛樿 255锛?

`V4L2_CID_MPEG_CX2341X_STREAM_INSERT_NAV_PACKETS (boolean)`
    CX2341X MPEG 缂栫爜鍣ㄥ彲浠ュ湪姣忓洓涓棰戝抚涔嬮棿鍚戠爜娴佷腑鎻掑叆涓€涓┖鐨?MPEG-2 PES 鍖呫€傚寘澶у皬涓?2048 瀛楄妭锛屽寘鍚?packet_start_code_prefix 鍜?stream_id 瀛楁銆俿tream_id 涓?0xBF锛堢鏈夋祦 2锛夈€傝浇鑽风敱 0x00 瀛楄妭缁勬垚锛岀敱搴旂敤绋嬪簭濉厖銆? = 涓嶆彃鍏ワ紝1 = 鎻掑叆鍖呫€?

## VPX 鎺т欢鍙傝€?


VPX 鎺т欢鍖呭惈鐢ㄤ簬 VPx 瑙嗛缂栬В鐮佸櫒缂栫爜鍙傛暟鐨勬帶浠躲€?



### VPX 鎺т欢 ID



`V4L2_CID_MPEG_VIDEO_VPX_NUM_PARTITIONS`
    (enum)

enum v4l2_vp8_num_partitions -
    VP8 缂栫爜鍣ㄤ腑浣跨敤鐨勬爣璁帮紙token锛夊垎鍖烘暟閲忋€傚彲鑳界殑鍊煎涓嬶細



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_1_PARTITION`
      - 1 涓郴鏁板垎鍖?
    - - `V4L2_CID_MPEG_VIDEO_VPX_2_PARTITIONS`
      - 2 涓郴鏁板垎鍖?
    - - `V4L2_CID_MPEG_VIDEO_VPX_4_PARTITIONS`
      - 4 涓郴鏁板垎鍖?
    - - `V4L2_CID_MPEG_VIDEO_VPX_8_PARTITIONS`
      - 8 涓郴鏁板垎鍖?



`V4L2_CID_MPEG_VIDEO_VPX_IMD_DISABLE_4X4 (boolean)`
    璁剧疆姝ら」鍙槻姝㈠湪甯у唴妯″紡鍐崇瓥涓娇鐢ㄥ抚鍐?4x4 妯″紡銆?


`V4L2_CID_MPEG_VIDEO_VPX_NUM_REF_FRAMES`
    (enum)

enum v4l2_vp8_num_ref_frames -
    鐢ㄤ簬缂栫爜 P 甯х殑鍙傝€冨浘鍍忔暟閲忋€傚彲鑳界殑鍊煎涓嬶細



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_1_REF_FRAME`
      - 灏嗘悳绱㈡渶鍚庝竴甯у凡缂栫爜甯?
    - - `V4L2_CID_MPEG_VIDEO_VPX_2_REF_FRAME`
      - 灏嗗湪鏈€鍚庝竴甯у凡缂栫爜甯с€侀粍閲戝抚锛坓olden frame锛夊拰澶囩敤鍙傝€冿紙altref锛夊抚涓悳绱袱甯с€傜紪鐮佸櫒瀹炵幇灏嗗喅瀹氶€夋嫨鍝袱甯с€?
    - - `V4L2_CID_MPEG_VIDEO_VPX_3_REF_FRAME`
      - 灏嗘悳绱㈡渶鍚庝竴甯у凡缂栫爜甯с€侀粍閲戝抚鍜?altref 甯с€?


    \normalsize



`V4L2_CID_MPEG_VIDEO_VPX_FILTER_LEVEL (integer)`
    鎸囩ず鐜矾婊ゆ尝鍣ㄧ瓑绾с€傜幆璺护娉㈠櫒绛夌骇鐨勮皟鏁存槸閫氳繃鐩稿浜庡熀鍑嗙幆璺护娉㈠櫒鍊肩殑澧為噺鍊兼潵瀹屾垚鐨勩€?

`V4L2_CID_MPEG_VIDEO_VPX_FILTER_SHARPNESS (integer)`
    姝ゅ弬鏁板奖鍝嶇幆璺护娉㈠櫒銆備换浣曞ぇ浜庨浂鐨勫€奸兘浼氬噺寮辩幆璺护娉㈠櫒鐨勫幓鍧楁晥搴斻€?

`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_REF_PERIOD (integer)`
    璁剧疆榛勯噾甯х殑鍒锋柊鍛ㄦ湡銆傝鍛ㄦ湡浠ュ抚鏁板畾涔夈€傚浜庡€?'n'锛屼粠绗竴涓叧閿抚寮€濮嬶紝姣忕 n 甯у皢琚涓洪粍閲戝抚銆備緥濡傦紝瀵逛簬缂栫爜搴忓垪 0銆?銆?銆?銆?銆?銆?銆?锛岃嫢榛勯噾甯у埛鏂板懆鏈熻涓?4锛屽垯甯?0銆?銆? 绛夊皢琚涓洪粍閲戝抚锛屽洜涓哄抚 0 濮嬬粓鏄叧閿抚銆?


`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_SEL`
    (enum)

enum v4l2_vp8_golden_frame_sel -
    閫夋嫨鐢ㄤ簬缂栫爜鐨勯粍閲戝抚銆傚彲鑳界殑鍊煎涓嬶細


    \scriptsize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_PREV`
      - 浣跨敤绗?(n-2) 甯т綔涓洪粍閲戝抚锛屽綋鍓嶅抚绱㈠紩涓?'n'銆?
    - - `V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_REF_PERIOD`
      - 浣跨敤鐢?`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_REF_PERIOD` 鎸囩ず鐨勫墠涓€涓壒瀹氬抚浣滀负榛勯噾甯с€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_VPX_MIN_QP (integer)`
    VP8 鐨勬渶灏忛噺鍖栧弬鏁般€?

`V4L2_CID_MPEG_VIDEO_VPX_MAX_QP (integer)`
    VP8 鐨勬渶澶ч噺鍖栧弬鏁般€?

`V4L2_CID_MPEG_VIDEO_VPX_I_FRAME_QP (integer)`
    VP8 鐨?I 甯ч噺鍖栧弬鏁般€?

`V4L2_CID_MPEG_VIDEO_VPX_P_FRAME_QP (integer)`
    VP8 鐨?P 甯ч噺鍖栧弬鏁般€?


`V4L2_CID_MPEG_VIDEO_VP8_PROFILE`
    (enum)

enum v4l2_mpeg_video_vp8_profile -
    姝ゆ帶浠剁敤浜庨€夋嫨 VP8 缂栫爜鍣ㄧ殑妗ｆ銆傚畠涔熺敤浜庢灇涓?VP8 缂栫爜鍣ㄦ垨瑙ｇ爜鍣ㄦ敮鎸佺殑妗ｆ銆傚彲鑳界殑鍊煎涓嬶細

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_0`
      - Profile 0
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_1`
      - Profile 1
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_2`
      - Profile 2
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_3`
      - Profile 3


`V4L2_CID_MPEG_VIDEO_VP9_PROFILE`
    (enum)

enum v4l2_mpeg_video_vp9_profile -
    姝ゆ帶浠剁敤浜庨€夋嫨 VP9 缂栫爜鍣ㄧ殑妗ｆ銆傚畠涔熺敤浜庢灇涓?VP9 缂栫爜鍣ㄦ垨瑙ｇ爜鍣ㄦ敮鎸佺殑妗ｆ銆傚彲鑳界殑鍊煎涓嬶細

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_0`
      - Profile 0
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_1`
      - Profile 1
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_2`
      - Profile 2
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_3`
      - Profile 3


`V4L2_CID_MPEG_VIDEO_VP9_LEVEL (enum)`

enum v4l2_mpeg_video_vp9_level -
    姝ゆ帶浠剁敤浜庨€夋嫨 VP9 缂栫爜鍣ㄧ殑绛夌骇銆傚畠涔熺敤浜庢灇涓?VP9 缂栫爜鍣ㄦ垨瑙ｇ爜鍣ㄦ敮鎸佺殑绛夌骇銆傛洿澶氫俊鎭彲鍙傞槄 `webmproject <https://www.webmproject.org/vp9/levels/>`__銆傚彲鑳界殑鍊煎涓嬶細

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_1_0`
      - Level 1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_1_1`
      - Level 1.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_2_0`
      - Level 2
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_2_1`
      - Level 2.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_3_0`
      - Level 3
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_3_1`
      - Level 3.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_4_0`
      - Level 4
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_4_1`
      - Level 4.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_0`
      - Level 5
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_1`
      - Level 5.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_2`
      - Level 5.2
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_0`
      - Level 6
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_1`
      - Level 6.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_2`
      - Level 6.2


## 楂樻晥瑙嗛缂栫爜锛圚EVC/H.265锛夋帶浠跺弬鑰?


HEVC/H.265 鎺т欢鍖呭惈鐢ㄤ簬 HEVC/H.265 瑙嗛缂栬В鐮佸櫒缂栫爜鍙傛暟鐨勬帶浠躲€?



### HEVC/H.265 鎺т欢 ID



`V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP (integer)`
    HEVC 鐨勬渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆?

`V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP (integer)`
    HEVC 鐨勬渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆?

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_QP (integer)`
    HEVC 鐨?I 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_QP (integer)`
    HEVC 鐨?P 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_QP (integer)`
    HEVC 鐨?B 甯ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC I 甯ц川閲忚寖鍥寸殑 HEVC I 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC I 甯ц川閲忚寖鍥寸殑 HEVC I 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC P 甯ц川閲忚寖鍥寸殑 HEVC P 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC P 甯ц川閲忚寖鍥寸殑 HEVC P 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_MIN_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC B 甯ц川閲忚寖鍥寸殑 HEVC B 甯ф渶灏忛噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_MAX_QP (integer)`
    鐢ㄤ簬闄愬埗 HEVC B 甯ц川閲忚寖鍥寸殑 HEVC B 甯ф渶澶ч噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細8 浣嶆椂涓?0 鍒?51锛?0 浣嶆椂涓?0 鍒?63銆傚鏋滃悓鏃惰缃簡 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP锛屽垯閲忓寲鍙傛暟搴旀弧瓒充袱鑰呯殑瑕佹眰銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_QP (boolean)`
    HIERARCHICAL_QP 鍏佽涓绘満閫氳繃 HIERARCHICAL_QP_LAYER 涓烘瘡涓€鏃堕棿灞傛寚瀹氶噺鍖栧弬鏁板€笺€備粎褰?HIERARCHICAL_CODING_LAYER 澶т簬 1 鏃舵湁鏁堛€傚皢姝ゆ帶浠跺€艰涓?1 鍙惎鐢ㄥ悇灞傜殑 QP 鍊艰缃€?


`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_TYPE`
    (enum)

enum v4l2_mpeg_video_hevc_hier_coding_type -
    閫夋嫨鐢ㄤ簬缂栫爜鐨勫垎灞傜紪鐮佺被鍨嬨€傚彲鑳界殑鍊煎涓嬶細


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_B`
      - 浣跨敤 B 甯ц繘琛屽垎灞傜紪鐮併€?
    - - `V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_P`
      - 浣跨敤 P 甯ц繘琛屽垎灞傜紪鐮併€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_LAYER (integer)`
    閫夋嫨鍒嗗眰缂栫爜灞傘€傚湪鏅€氱紪鐮侊紙闈炲垎灞傜紪鐮侊級涓紝搴旇涓洪浂銆傚彲鑳界殑鍊间负 [0, 6]銆? 琛ㄧず鍒嗗眰缂栫爜灞?0锛? 琛ㄧず鍒嗗眰缂栫爜灞?1锛屼緷姝ょ被鎺ㄣ€?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L0_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?0 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L1_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?1 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L2_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?2 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L3_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?3 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L4_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?4 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L5_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?5 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L6_QP (integer)`
    鎸囩ず鍒嗗眰缂栫爜灞?6 鐨勯噺鍖栧弬鏁般€傛湁鏁堣寖鍥达細[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]銆?


`V4L2_CID_MPEG_VIDEO_HEVC_PROFILE`
    (enum)

enum v4l2_mpeg_video_hevc_profile -
    涓?HEVC 缂栫爜鍣ㄩ€夋嫨鎵€闇€鐨勬。娆°€?


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN`
      - 涓绘。娆°€?
    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_STILL_PICTURE`
      - 涓婚潤鎬佸浘鍍忔。娆°€?
    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_10`
      - 涓?10 妗ｆ銆?


    \normalsize



`V4L2_CID_MPEG_VIDEO_HEVC_LEVEL`
    (enum)

enum v4l2_mpeg_video_hevc_level -
    涓?HEVC 缂栫爜鍣ㄩ€夋嫨鎵€闇€鐨勭瓑绾с€?

==================================	=========
`V4L2_MPEG_VIDEO_HEVC_LEVEL_1`	Level 1.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_2`	Level 2.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_2_1`	Level 2.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_3`	Level 3.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_3_1`	Level 3.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_4`	Level 4.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_4_1`	Level 4.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5`	Level 5.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5_1`	Level 5.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5_2`	Level 5.2
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6`	Level 6.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6_1`	Level 6.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6_2`	Level 6.2
==================================	=========

`V4L2_CID_MPEG_VIDEO_HEVC_FRAME_RATE_RESOLUTION (integer)`
    鎸囩ず涓€绉掑唴鐨勫潎鍖€闂撮殧瀛愬尯闂达紙绉颁负 ticks锛夋暟閲忋€傝繖鏄竴涓?16 浣嶆棤绗﹀彿鏁存暟锛屾渶澶у€间负 0xffff锛屾渶灏忓€间负 1銆?


`V4L2_CID_MPEG_VIDEO_HEVC_TIER`
    (enum)

enum v4l2_mpeg_video_hevc_tier -
    TIER_FLAG 鎸囧畾 HEVC 缂栫爜鍥惧儚鐨勫眰绾э紙tier锛変俊鎭€傚眰绾ф槸涓轰簡澶勭悊鏈€澶ф瘮鐗圭巼涓嶅悓鐨勫簲鐢ㄨ€岃绔嬬殑銆傚皢璇ユ爣蹇楄涓?0 閫夋嫨 HEVC 鐨?Main 灞傜骇锛岃涓?1 琛ㄧず High 灞傜骇銆侶igh 灞傜骇鐢ㄤ簬闇€瑕侀珮姣旂壒鐜囩殑搴旂敤銆?

==================================	==========
`V4L2_MPEG_VIDEO_HEVC_TIER_MAIN`	涓诲眰绾с€?
`V4L2_MPEG_VIDEO_HEVC_TIER_HIGH`	楂樺眰绾с€?
==================================	==========


`V4L2_CID_MPEG_VIDEO_HEVC_MAX_PARTITION_DEPTH (integer)`
    閫夋嫨 HEVC 鏈€澶х紪鐮佸崟鍏冩繁搴︺€?


`V4L2_CID_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE`
    (enum)

enum v4l2_mpeg_video_hevc_loop_filter_mode -
    HEVC 缂栫爜鍣ㄧ殑鐜矾婊ゆ尝鍣ㄦā寮忋€傚彲鑳界殑鍊煎涓嬶細


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED`
      - 鐜矾婊ゆ尝鍣ㄥ凡绂佺敤銆?
    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_ENABLED`
      - 鐜矾婊ゆ尝鍣ㄥ凡鍚敤銆?
    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY`
      - 鍦ㄥ垏鐗囪竟鐣屽绂佺敤鐜矾婊ゆ尝鍣ㄣ€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_LF_BETA_OFFSET_DIV2 (integer)`
    閫夋嫨 HEVC 鐜矾婊ゆ尝鍣?beta 鍋忕Щ銆傛湁鏁堣寖鍥翠负 [-6, +6]銆?

`V4L2_CID_MPEG_VIDEO_HEVC_LF_TC_OFFSET_DIV2 (integer)`
    閫夋嫨 HEVC 鐜矾婊ゆ尝鍣?tc 鍋忕Щ銆傛湁鏁堣寖鍥翠负 [-6, +6]銆?


`V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_TYPE`
    (enum)

enum v4l2_mpeg_video_hevc_hier_refresh_type -
    閫夋嫨 HEVC 缂栫爜鍣ㄧ殑鍒锋柊绫诲瀷銆備富鏈哄繀椤诲皢鍛ㄦ湡鎸囧畾鍒?V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_PERIOD銆?


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_NONE`
      - 浣跨敤 B 甯ц繘琛屽垎灞傜紪鐮併€?
    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_CRA`
      - 浣跨敤 CRA锛圕lean Random Access Unit锛夊浘鍍忕紪鐮併€?
    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_IDR`
      - 浣跨敤 IDR锛圛nstantaneous Decoding Refresh锛夊浘鍍忕紪鐮併€?


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_PERIOD (integer)`
    閫夋嫨 HEVC 缂栫爜鍣ㄧ殑鍒锋柊鍛ㄦ湡銆傚畠鎸囧畾涓や釜 CRA/IDR 鍥惧儚涔嬮棿鐨?I 鍥惧儚鏁伴噺銆備粎褰?REFRESH_TYPE 涓嶄负 0 鏃舵湁鏁堛€?

`V4L2_CID_MPEG_VIDEO_HEVC_LOSSLESS_CU (boolean)`
    鎸囩ず HEVC 鏃犳崯缂栫爜銆傝涓?0 绂佺敤鏃犳崯缂栫爜锛岃涓?1 鍚敤鏃犳崯缂栫爜銆?

`V4L2_CID_MPEG_VIDEO_HEVC_CONST_INTRA_PRED (boolean)`
    鎸囩ず HEVC 缂栫爜鍣ㄧ殑鎭掑畾甯у唴棰勬祴銆傛寚瀹氬彈闄愬抚鍐呴娴嬶紝鍏朵腑甯у唴鏈€澶х紪鐮佸崟鍏冿紙LCU锛夌殑棰勬祴浠呬娇鐢ㄧ浉閭诲抚鍐?LCU 鐨勬畫宸暟鎹拰宸茶В鐮佹牱鏈潵杩涜銆傚皢璇ュ€艰涓?1 鍚敤鎭掑畾甯у唴棰勬祴锛岃涓?0 绂佺敤鎭掑畾甯у唴棰勬祴銆?

`V4L2_CID_MPEG_VIDEO_HEVC_WAVEFRONT (boolean)`
    鎸囩ず HEVC 缂栫爜鍣ㄧ殑娉㈠墠骞惰澶勭悊锛坵avefront parallel processing锛夈€傝涓?0 绂佺敤璇ョ壒鎬э紝璁句负 1 鍚敤娉㈠墠骞惰澶勭悊銆?

`V4L2_CID_MPEG_VIDEO_HEVC_GENERAL_PB (boolean)`
    灏嗚鍊艰涓?1 鍙负 HEVC 缂栫爜鍣ㄥ惎鐢?P 甯у拰 B 甯х殑缁勫悎銆?

`V4L2_CID_MPEG_VIDEO_HEVC_TEMPORAL_ID (boolean)`
    鎸囩ず HEVC 缂栫爜鍣ㄧ殑鏃堕棿鏍囪瘑绗︼紝閫氳繃灏嗗€艰涓?1 鏉ュ惎鐢ㄣ€?

`V4L2_CID_MPEG_VIDEO_HEVC_STRONG_SMOOTHING (boolean)`
    鎸囩ず褰撹涓?1 鏃讹紝鍦?CVS 鐨勫抚鍐呴娴嬫护娉㈣繃绋嬩腑鏈夋潯浠跺湴浣跨敤鍙岀嚎鎬ф彃鍊笺€傛寚绀哄綋璁句负 0 鏃讹紝鍦?CVS 涓笉浣跨敤鍙岀嚎鎬ф彃鍊笺€?

`V4L2_CID_MPEG_VIDEO_HEVC_MAX_NUM_MERGE_MV_MINUS1 (integer)`
    鎸囩ず鍚堝苟鍊欓€夎繍鍔ㄧ煝閲忕殑鏈€澶ф暟閲忋€傚彇鍊艰寖鍥翠负 0 鍒?4銆?

`V4L2_CID_MPEG_VIDEO_HEVC_TMV_PREDICTION (boolean)`
    鎸囩ず HEVC 缂栫爜鍣ㄧ殑鏃堕棿杩愬姩鐭㈤噺棰勬祴銆傝涓?1 鍚敤棰勬祴锛岃涓?0 绂佺敤棰勬祴銆?

`V4L2_CID_MPEG_VIDEO_HEVC_WITHOUT_STARTCODE (boolean)`
    鎸囧畾 HEVC 鏄惁鐢熸垚浠ラ暱搴﹀瓧娈靛ぇ灏忎唬鏇胯捣濮嬬爜妯″紡鐨勭爜娴併€傞暱搴﹀瓧娈电殑澶у皬鍙€氳繃 V4L2_CID_MPEG_VIDEO_HEVC_SIZE_OF_LENGTH_FIELD 鎺т欢閰嶇疆銆傚皢璇ュ€艰涓?0 绂佺敤鏃犺捣濮嬬爜妯″紡鐨勭紪鐮併€傚皢璇ュ€艰涓?1 灏嗗惎鐢ㄦ棤璧峰鐮佹ā寮忕殑缂栫爜銆?


`V4L2_CID_MPEG_VIDEO_HEVC_SIZE_OF_LENGTH_FIELD`
(enum)

enum v4l2_mpeg_video_hevc_size_of_length_field -
    鎸囩ず闀垮害瀛楁鐨勫ぇ灏忋€傚綋鍚敤 WITHOUT_STARTCODE_ENABLE 缂栫爜鏃舵湁鏁堛€?


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_0`
      - 鐢熸垚璧峰鐮佹ā寮忥紙鏅€氾級銆?
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_1`
      - 鐢熸垚闀垮害瀛楁鐨勫ぇ灏忎唬鏇胯捣濮嬬爜妯″紡锛岄暱搴︿负 1銆?
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_2`
      - 鐢熸垚闀垮害瀛楁鐨勫ぇ灏忎唬鏇胯捣濮嬬爜妯″紡锛岄暱搴︿负 2銆?
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_4`
      - 鐢熸垚闀垮害瀛楁鐨勫ぇ灏忎唬鏇胯捣濮嬬爜妯″紡锛岄暱搴︿负 4銆?


    \normalsize

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L0_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 0 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L1_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 1 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L2_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 2 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L3_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 3 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L4_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 4 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L5_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 5 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L6_BR (integer)`
    鎸囩ず HEVC 缂栫爜鍣ㄥ垎灞傜紪鐮佸眰 6 鐨勬瘮鐗圭巼銆?

`V4L2_CID_MPEG_VIDEO_REF_NUMBER_FOR_PFRAMES (integer)`
    閫夋嫨 HEVC 缂栫爜鍣ㄦ墍闇€鐨?P 鍙傝€冨浘鍍忔暟閲忋€侾 甯у彲浣跨敤 1 鎴?2 甯т綔涓哄弬鑰冦€?

`V4L2_CID_MPEG_VIDEO_PREPEND_SPSPPS_TO_IDR (integer)`
    鎸囩ず鏄惁鍦ㄦ瘡涓?IDR 澶勭敓鎴?SPS 鍜?PPS銆傝涓?0 绂佺敤鍦ㄦ瘡涓?IDR 澶勭敓鎴?SPS 鍜?PPS銆傝涓?1 鍚敤鍦ㄦ瘡涓?IDR 澶勭敓鎴?SPS 鍜?PPS銆?

