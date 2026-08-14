


## DVB 闊抽璁惧


             See: legacy_dvb_decoder_notes

DVB 闊抽璁惧鎺у埗鐫€ DVB 纭欢鐨?MPEG2 闊抽瑙ｇ爜鍣ㄣ€傚彲浠ラ€氳繃
`/dev/dvb/adapter?/audio?` 璁块棶瀹冦€傛暟鎹被鍨嬩笌 ioctl 瀹氫箟鍙互閫氳繃鍦?搴旂敤绋嬪簭涓寘鍚?`linux/dvb/audio.h` 鏉ヤ娇鐢ㄣ€?
璇锋敞鎰忥紝澶у鏁?DVB 鍗℃病鏈夎嚜宸辩殑 MPEG 瑙ｇ爜鍣紝鍥犳浼氱渷鐣ラ煶棰戝拰瑙嗛
璁惧銆?
杩欎簺 ioctl 涔熸浘琚?V4L2 鐢ㄦ潵鎺у埗 V4L2 涓疄鐜扮殑 MPEG 瑙ｇ爜鍣ㄣ€傚皢杩欑被
ioctl 鐢ㄤ簬璇ョ洰鐨勭殑鍋氭硶宸茶搴熷純锛屽苟宸插垱寤虹浉搴旂殑 V4L2 ioctl 鎴栨帶浠舵潵
鍙栦唬璇ュ姛鑳姐€傛柊鐨勯┍鍔ㄧ▼搴忚浣跨敤 V4L2 ioctls<audio>锛?

## 闊抽鏁版嵁绫诲瀷


鏈妭鎻忚堪涓庨煶棰戣澶囦氦浜掓椂鎵€浣跨敤鐨勭粨鏋勪綋銆佹暟鎹被鍨嬩笌瀹忓畾涔夈€?

-----



### audio_stream_source_t


#### 姒傝堪


    typedef enum {
    AUDIO_SOURCE_DEMUX,
    AUDIO_SOURCE_MEMORY
    } audio_stream_source_t;

#### 甯搁噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_SOURCE_DEMUX`

       - `1` 閫夋嫨瑙ｅ鐢ㄥ櫒锛堢敱鍓嶇鎴?DVR 璁惧鎻愪緵鏁版嵁锛変綔涓鸿棰戞祦鐨勬潵婧愩€?
    - ..

       - `AUDIO_SOURCE_MEMORY`

       - 閫夋嫨閫氳繃 `write()`_ 绯荤粺璋冪敤鏉ヨ嚜搴旂敤绋嬪簭鐨勬祦銆?
#### 鎻忚堪


闊抽娴佹潵婧愰€氳繃 `AUDIO_SELECT_SOURCE`_ 璋冪敤璁剧疆锛屽彲鍙栧€煎涓嬶紝鍙栧喅浜?鎴戜滑鏄洖鏀惧唴閮紙demux锛夎繕鏄閮紙鐢ㄦ埛鍐欏叆锛夋潵婧愩€?
閫佸叆瑙ｇ爜鍣ㄧ殑鏁版嵁杩樺彈 PID 杩囨护鍣ㄦ帶鍒躲€傝緭鍑洪€夋嫨锛歚dmx_output`
`DMX_OUT_DECODER`銆?

-----



### audio_play_state_t


#### 姒傝堪


    typedef enum {
	AUDIO_STOPPED,
	AUDIO_PLAYING,
	AUDIO_PAUSED
    } audio_play_state_t;

#### 甯搁噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_STOPPED`

       - 闊抽宸插仠姝€?
    - ..

       - `AUDIO_PLAYING`

       - 闊抽姝ｅ湪鎾斁銆?
    - ..

       - `AUDIO_PAUSE`

       - 闊抽宸插喕缁撱€?
#### 鎻忚堪


姝ゅ€煎彲鐢?`AUDIO_GET_STATUS`_ 璋冪敤杩斿洖锛岃〃绀洪煶棰戞挱鏀剧殑鐘舵€併€?

-----



### audio_channel_select_t


#### 姒傝堪


    typedef enum {
	AUDIO_STEREO,
	AUDIO_MONO_LEFT,
	AUDIO_MONO_RIGHT,
	AUDIO_MONO,
	AUDIO_STEREO_SWAPPED
    } audio_channel_select_t;

#### 甯搁噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_STEREO`

       - 绔嬩綋澹般€?
    - ..

       - `AUDIO_MONO_LEFT`

       - 鍗曞０閬擄紝閫夋嫨宸﹀０閬撲綔涓烘潵婧愩€?
    - ..

       - `AUDIO_MONO_RIGHT`

       - 鍗曞０閬擄紝閫夋嫨鍙冲０閬撲綔涓烘潵婧愩€?
    - ..

       - `AUDIO_MONO`

       - 浠呭崟澹伴亾鏉ユ簮銆?
    - ..

       - `AUDIO_STEREO_SWAPPED`

       - 绔嬩綋澹帮紝浜ゆ崲宸︼紙L锛変笌鍙筹紙R锛夈€?
#### 鎻忚堪


閫氳繃 `AUDIO_CHANNEL_SELECT`_ 閫夋嫨鐨勯煶棰戝０閬撶敱姝ゅ€煎喅瀹氥€?

-----



### audio_mixer_t


#### 姒傝堪


    typedef struct audio_mixer {
	unsigned int volume_left;
	unsigned int volume_right;
    } audio_mixer_t;

#### 鍙橀噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `unsigned int volume_left`

       - 宸﹀０閬撻煶閲忋€?          鏈夋晥鑼冨洿锛? ... 255

    - ..

       - `unsigned int volume_right`

       - 鍙冲０閬撻煶閲忋€?          鏈夋晥鑼冨洿锛? ... 255

#### 鎻忚堪


姝ょ粨鏋勪綋鐢?`AUDIO_SET_MIXER`_ 璋冪敤鐢ㄦ潵璁剧疆闊抽闊抽噺銆?

-----



### audio_status


#### 姒傝堪


    typedef struct audio_status {
	int AV_sync_state;
	int mute_state;
	audio_play_state_t play_state;
	audio_stream_source_t stream_source;
	audio_channel_select_t channel_select;
	int bypass_mode;
	audio_mixer_t mixer_state;
    } audio_status_t;

#### 鍙橀噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `2` `int AV_sync_state`

       - `1` 鏄剧ず A/V 鍚屾鏄紑鍚繕鏄叧闂€?
    - ..

       - TRUE  ( != 0 )

       - A/V 鍚屾寮€鍚€?
    - ..

       - FALSE ( == 0 )

       - A/V 鍚屾鍏抽棴銆?
    - ..

       - `2` `int mute_state`

       - `1` 鎸囩ず闊抽鏄惁闈欓煶銆?
    - ..

       - TRUE  ( != 0 )

       - 闈欓煶闊抽

    - ..

       - FALSE ( == 0 )

       - 鍙栨秷闈欓煶闊抽

    - ..

       - `audio_play_state_t`_ `play_state`

       - 褰撳墠鎾斁鐘舵€併€?
    - ..

       - `audio_stream_source_t`_ `stream_source`

       - 褰撳墠鐨勬暟鎹潵婧愩€?
    - ..

       - `2` `int bypass_mode`

       - `1` 褰撳墠闊抽娴佸湪 DVB 瀛愮郴缁熶腑鐨勮В鐮佹槸鍚﹁鍚敤鎴栫鐢ㄣ€?
    - ..

       - TRUE  ( != 0 )

       - 鏃佽矾绂佺敤銆?
    - ..

       - FALSE ( == 0 )

       - 鏃佽矾鍚敤銆?
    - ..

       - `audio_mixer_t`_ `mixer_state`

       - 褰撳墠闊抽噺璁剧疆銆?
#### 鎻忚堪


`AUDIO_GET_STATUS`_ 璋冪敤杩斿洖姝ょ粨鏋勪綋锛屼綔涓烘挱鏀炬搷浣滃悇绉嶇姸鎬佺殑淇℃伅銆?

-----



### audio encodings


#### 姒傝堪


     #define AUDIO_CAP_DTS    1
     #define AUDIO_CAP_LPCM   2
     #define AUDIO_CAP_MP1    4
     #define AUDIO_CAP_MP2    8
     #define AUDIO_CAP_MP3   16
     #define AUDIO_CAP_AAC   32
     #define AUDIO_CAP_OGG   64
     #define AUDIO_CAP_SDDS 128
     #define AUDIO_CAP_AC3  256

#### 甯搁噺


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_CAP_DTS`

       - `1` 纭欢鎺ュ彈 DTS 闊宠建銆?
    - ..

       - `AUDIO_CAP_LPCM`

       - 纭欢鎺ュ彈閲囩敤绾挎€ц剦鍐茬紪鐮佽皟鍒讹紙LPCM锛夌殑闈炲帇缂╅煶棰戙€?
    - ..

       - `AUDIO_CAP_MP1`

       - 纭欢鎺ュ彈 MPEG-1 Audio Layer 1銆?
    - ..

       - `AUDIO_CAP_MP2`

       - 纭欢鎺ュ彈 MPEG-1 Audio Layer 2銆?          涔熺О涓?MUSICAM銆?
    - ..

       - `AUDIO_CAP_MP3`

       - 纭欢鎺ュ彈 MPEG-1 Audio Layer III銆?          閫氬父绉颁负 .mp3銆?
    - ..

       - `AUDIO_CAP_AAC`

       - 纭欢鎺ュ彈 AAC锛堥珮绾ч煶棰戠紪鐮侊級銆?
    - ..

       - `AUDIO_CAP_OGG`

       - 纭欢鎺ュ彈 Vorbis 闊宠建銆?
    - ..

       - `AUDIO_CAP_SDDS`

       - 纭欢鎺ュ彈 Sony Dynamic Digital Sound锛圫DDS锛夈€?
    - ..

       - `AUDIO_CAP_AC3`

       - 纭欢鎺ュ彈 Dolby Digital ATSC A/52 闊抽銆?          涔熺О涓?AC-3銆?
#### 鎻忚堪


瀵?`AUDIO_GET_CAPABILITIES`_ 鐨勮皟鐢ㄨ繑鍥炰竴涓棤绗﹀彿鏁存暟锛屽叾涓牴鎹‖浠?鑳藉姏璁剧疆浜嗕互涓嬫瘮鐗逛綅銆?

-----



## 闊抽鍑芥暟璋冪敤



### AUDIO_STOP


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_STOP)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵瑰簲璇ュ懡浠わ紝绛変簬 `AUDIO_STOP`銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧鍋滄鎾斁褰撳墠鐨勬祦銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_PLAY


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_PLAY)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵瑰簲璇ュ懡浠わ紝绛変簬 `AUDIO_PLAY`銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧寮€濮嬩粠鎵€閫夋潵婧愭挱鏀鹃煶棰戞祦銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_PAUSE


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_PAUSE)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_PAUSE`銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤鏆傚仠姝ｅ湪鎾斁鐨勯煶棰戞祦銆傝В鐮佷笌鎾斁閮借鏆傚仠銆備箣鍚庡彲浠ヤ娇鐢?`AUDIO_CONTINUE`_ 鍛戒护閲嶆柊寮€濮嬮煶棰戞祦鐨勮В鐮佷笌鎾斁杩囩▼銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_CONTINUE


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_CONTINUE)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_CONTINUE`銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 閲嶆柊鍚姩鍏堝墠琚?`AUDIO_PAUSE`_ 鍛戒护鏆傚仠鐨勮В鐮佷笌鎾斁杩囩▼銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SELECT_SOURCE


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_SELECT_SOURCE,
	 audio_stream_source_t source)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_SELECT_SOURCE`銆?
    - ..

       - `audio_stream_source_t`_ `source`

       - 鎸囩ず搴旂敤浜庨煶棰戞祦鐨勬潵婧愩€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤鍛婄煡闊抽璁惧杈撳叆鏁版嵁搴斾娇鐢ㄥ摢涓潵婧愩€傚彲鑳界殑鏉ユ簮鏄?demux
鎴?memory銆傝嫢閫夋嫨 `AUDIO_SOURCE_MEMORY`锛屽垯鏁版嵁閫氳繃 write 鍛戒护閫佸叆闊抽
璁惧銆傝嫢閫夋嫨 `AUDIO_SOURCE_DEMUX`锛屾暟鎹垯鐩存帴浠庢澘杞借В澶嶇敤璁惧浼犺緭鍒?瑙ｇ爜鍣ㄣ€傛敞鎰忥細鍒扮洰鍓嶄负姝㈣繖浠呮敮鎸佸叿鏈変竴涓В澶嶇敤鍣ㄥ拰涓€涓В鐮佸櫒鐨?DVB 璁惧銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_MUTE


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_SET_MUTE, int state)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵瑰簲璇ュ懡浠わ紝绛変簬 `AUDIO_SET_MUTE`銆?
    - ..

       - `2` `int state`

       - `1` 鎸囩ず闊抽璁惧鏄惁搴旈潤闊炽€?
    - ..

       - TRUE  ( != 0 )

       - 闈欓煶闊抽

    - ..

       - FALSE ( == 0 )

       - 鍙栨秷闈欓煶闊抽

#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 浠呴€傜敤浜?DVB 璁惧銆傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2
VIDIOC_DECODER_CMD锛屽苟甯︿笂 `V4L2_DEC_CMD_START_MUTE_AUDIO` 鏍囧織銆?
姝?ioctl 璋冪敤璇锋眰闊抽璁惧瀵瑰綋鍓嶆鍦ㄦ挱鏀剧殑娴佽繘琛岄潤闊炽€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_AV_SYNC


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_SET_AV_SYNC, int state)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵瑰簲璇ュ懡浠わ紝绛変簬 `AUDIO_AV_SYNC`銆?
    - ..

       - `2` `int state`

       - `1` 鍛婄煡 DVB 瀛愮郴缁?A/V 鍚屾搴斿紑鍚繕鏄叧闂€?
    - ..

       - TRUE  ( != 0 )

       - A/V 鍚屾寮€鍚€?
    - ..

       - FALSE ( == 0 )

       - A/V 鍚屾鍏抽棴銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧寮€鍚垨鍏抽棴 A/V 鍚屾銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_BYPASS_MODE


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_SET_BYPASS_MODE, int mode)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵瑰簲璇ュ懡浠わ紝绛変簬 `AUDIO_SET_BYPASS_MODE`銆?
    - ..

       - `2` `int mode`

       - `1` 鍚敤鎴栫鐢ㄥ綋鍓嶉煶棰戞祦鍦?DVB 瀛愮郴缁熶腑鐨勮В鐮併€?
    - ..

       - TRUE  ( != 0 )

       - 绂佺敤鏃佽矾

    - ..

       - FALSE ( == 0 )

       - 鍚敤鏃佽矾

#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧鏃佽矾闊抽瑙ｇ爜鍣紝骞剁洿鎺ヨ浆鍙戞祦鑰屼笉杩涜瑙ｇ爜銆?褰撴棤娉曡 DVB 绯荤粺澶勭悊鐨勬祦闇€瑕佽В鐮佹椂锛屽簲浣跨敤姝ゆā寮忋€傚鏋滅‖浠舵敮鎸侊紝
Dolby DigitalTM 娴佷細琚?DVB 瀛愮郴缁熻嚜鍔ㄨ浆鍙戙€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_CHANNEL_SELECT


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_CHANNEL_SELECT,
	 audio_channel_select_t)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_CHANNEL_SELECT`銆?
    - ..

       - `audio_channel_select_t`_ `ch`

       - 閫夋嫨闊抽鐨勮緭鍑烘牸寮忥紙宸?鍙冲崟澹伴亾銆佺珛浣撳０锛夈€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 浠呴€傜敤浜?DVB 璁惧銆傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2
`V4L2_CID_MPEG_AUDIO_DEC_PLAYBACK` 鎺т欢銆?
姝?ioctl 璋冪敤鍦ㄥ彲鑳界殑鎯呭喌涓嬭姹傞煶棰戣澶囬€夋嫨鎵€璇锋眰鐨勫０閬撱€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_GET_STATUS


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_GET_STATUS,
	 struct audio_status *status)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 AUDIO_GET_STATUS銆?
    - ..

       - `struct` `audio_status`_ `*status`

       - 杩斿洖闊抽璁惧鐨勫綋鍓嶇姸鎬併€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧杩斿洖闊抽璁惧鐨勫綋鍓嶇姸鎬併€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_GET_CAPABILITIES


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_GET_CAPABILITIES,
	 unsigned int *cap)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_GET_CAPABILITIES`銆?
    - ..

       - `unsigned int *cap`

       - 杩斿洖鍙楁敮鎸佺殑澹伴煶鏍煎紡鐨勪綅鏁扮粍銆?          姣旂壒浣嶅湪 `audio encodings`_ 涓畾涔夈€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧鍛婄煡鎴戜滑闊抽纭欢鐨勮В鐮佽兘鍔涖€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_CLEAR_BUFFER


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_CLEAR_BUFFER)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_CLEAR_BUFFER`銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇锋眰闊抽璁惧娓呯┖闊抽瑙ｇ爜鍣ㄨ澶囩殑鎵€鏈夎蒋浠朵笌纭欢缂撳啿鍖恒€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_ID


#### 姒傝堪


	 int  ioctl(int fd, int request = AUDIO_SET_ID, int id)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_SET_ID`銆?
    - ..

       - `int id`

       - 闊抽瀛愭祦 id銆?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

濡傛灉绋嬪簭娴佹垨绯荤粺娴佽鍙戦€佸埌瑙嗛璁惧锛屾 ioctl 閫夋嫨瑕佽瑙ｇ爜鐨勫瓙娴併€?
濡傛灉鏈缃煶棰戞祦绫诲瀷锛屽垯瀵逛簬 MPEG 澹伴煶锛宨d 蹇呴』鍦?[0xC0,0xDF] 鑼冨洿鍐咃紱
瀵逛簬 AC3锛屽湪 [0x80,0x87] 鑼冨洿鍐咃紱瀵逛簬 LPCM锛屽湪 [0xA0,0xA7] 鑼冨洿鍐呫€?鏇村璇存槑璇峰弬瑙?ITU-T H.222.0 | ISO/IEC 13818-1銆?
濡傛灉娴佺被鍨嬪凡閫氳繃 `AUDIO_SET_STREAMTYPE`_ 璁剧疆锛屽垯 id 鍙〃绀洪煶棰戞祦鐨?瀛愭祦 id锛屼笖鍙瘑鍒墠 5 涓瘮鐗癸紙& 0x1F锛夈€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_MIXER


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_SET_MIXER, audio_mixer_t *mix)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_SET_MIXER`銆?
    - ..

       - `audio_mixer_t *mix`

       - 娣烽煶鍣ㄨ缃€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 鍏佽浣犺皟鏁撮煶棰戣В鐮佸櫒鐨勬贩闊冲櫒璁剧疆銆?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### AUDIO_SET_STREAMTYPE


#### 姒傝堪


	 int  ioctl(fd, int request = AUDIO_SET_STREAMTYPE, int type)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_SET_STREAMTYPE`銆?
    - ..

       - `int type`

       - 娴佺被鍨嬨€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

姝?ioctl 鍛婅瘔椹卞姩绋嬪簭棰勬湡鎺ユ敹鍝闊抽娴併€傚綋娴佹彁渚涘绉嶉煶棰戝瓙娴侊紙濡?LPCM 鍜?AC3锛夋椂锛岃繖寰堟湁鐢ㄣ€?
浣跨敤 ITU-T H.222.0 | ISO/IEC 13818-1 涓畾涔夌殑娴佺被鍨嬨€?

#### 杩斿洖鍊?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EINVAL`

       - 绫诲瀷涓嶆槸鏈夋晥鎴栧彈鏀寔鐨勬祦绫诲瀷銆?

-----



### AUDIO_BILINGUAL_CHANNEL_SELECT


#### 姒傝堪


	 int ioctl(int fd, int request = AUDIO_BILINGUAL_CHANNEL_SELECT,
	 audio_channel_select_t)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 绛変簬 `AUDIO_BILINGUAL_CHANNEL_SELECT`銆?
    - ..

       - `audio_channel_select_t ch`

       - 閫夋嫨闊抽鐨勮緭鍑烘牸寮忥紙宸?鍙冲崟澹伴亾銆佺珛浣撳０锛夈€?
#### 鎻忚堪


             See: legacy_dvb_decoder_notes

瀵逛簬閫氳繃 V4L2 鎺у埗鐨?MPEG 瑙ｇ爜鍣紝姝?ioctl 宸茶 V4L2
`V4L2_CID_MPEG_AUDIO_DEC_MULTILINGUAL_PLAYBACK` 鎺т欢鍙栦唬銆?
姝?ioctl 璋冪敤鍦ㄥ彲鑳界殑鎯呭喌涓嬭姹傞煶棰戣澶囦负鍙岃娴侀€夋嫨鎵€璇锋眰鐨勫０閬撱€?
#### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑璇存槑銆?

-----



### open()


#### 姒傝堪


    #include <fcntl.h>


#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 鐗瑰畾闊抽璁惧鐨勫悕绉般€?
    - ..

       - `3` `int flags`

       - `1` 涓嬪垪鏍囧織鐨勬寜浣嶆垨锛?
    - ..

       - `O_RDONLY`

       - 鍙璁块棶

    - ..

       - `O_RDWR`

       - 璇诲啓璁块棶

    - ..

       - `O_NONBLOCK`
       - | 浠ラ潪闃诲妯″紡鎵撳紑
          | 锛堥粯璁ゆ槸闃诲妯″紡锛?
#### 鎻忚堪


姝ょ郴缁熻皟鐢ㄦ墦寮€涓€涓懡鍚嶇殑闊抽璁惧锛堜緥濡?`/dev/dvb/adapter0/audio0`锛?浠ヤ緵鍚庣画浣跨敤銆傚綋 open() 璋冪敤鎴愬姛鍚庯紝璁惧鍗冲彲浣跨敤銆傞樆濉炴垨闈為樆濉炴ā寮?鐨勬剰涔夊湪瀛樺湪宸紓鐨勫嚱鏁版枃妗ｄ腑璇存槑銆傚畠涓嶅奖鍝?open() 璋冪敤鏈韩鐨勮涔夈€?浠ラ樆濉炴ā寮忔墦寮€鐨勮澶囦箣鍚庡彲浠ヤ娇鐢?fcntl 绯荤粺璋冪敤鐨?F_SETFL 鍛戒护鍒囨崲
鍒伴潪闃诲妯″紡锛堝弽涔嬩害鐒讹級銆傝繖鏄竴涓爣鍑嗙殑绯荤粺璋冪敤锛屽湪 Linux 鐨?fcntl
鎵嬪唽椤典腑鏈夎鏄庛€傚彧鏈変竴涓敤鎴疯兘浠?O_RDWR 妯″紡鎵撳紑闊抽璁惧銆傛墍鏈夊叾浠?浠ヨ妯″紡鎵撳紑璁惧鐨勫皾璇曢兘浼氬け璐ワ紝骞惰繑鍥為敊璇爜銆傚鏋滀互 O_RDONLY 妯″紡
鎵撳紑闊抽璁惧锛屽垯鍞竴鍙互浣跨敤鐨?ioctl 璋冪敤鏄?`AUDIO_GET_STATUS`_銆?鎵€鏈夊叾浠栬皟鐢ㄩ兘浼氳繑鍥為敊璇爜銆?
#### 杩斿洖鍊?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - 璁惧椹卞姩鏈姞杞?涓嶅彲鐢ㄣ€?
    - ..

       - `EBUSY`

       - 璁惧鎴栬祫婧愬繖銆?
    - ..

       - `EINVAL`

       - 鏃犳晥鍙傛暟銆?

-----



### close()


#### 姒傝堪



#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
#### 鎻忚堪


姝ょ郴缁熻皟鐢ㄥ叧闂厛鍓嶆墦寮€鐨勯煶棰戣澶囥€?
#### 杩斿洖鍊?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - 鏂囦欢鎻忚堪绗︿笉鏄湁鏁堢殑宸叉墦寮€鏂囦欢鎻忚堪绗︺€?

-----



### write()


#### 姒傝堪


	 size_t write(int fd, const void *buf, size_t count)

#### 鍙傛暟


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢卞厛鍓嶅 `open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︺€?
    - ..

       - `void *buf`

       - 鎸囧悜鍖呭惈 PES 鏁版嵁鐨勭紦鍐插尯鐨勬寚閽堛€?
    - ..

       - `size_t count`

       - buf 鐨勫ぇ灏忋€?
#### 鎻忚堪


姝ょ郴缁熻皟鐢ㄥ彧鑳藉湪 ioctl 璋冪敤 `AUDIO_SELECT_SOURCE`_ 涓€夋嫨浜?`AUDIO_SOURCE_MEMORY` 鏃朵娇鐢ㄣ€傛墍鎻愪緵鐨勬暟鎹簲涓?PES 鏍煎紡銆傚鏋滄湭鎸囧畾
`O_NONBLOCK`锛岃鍑芥暟灏嗛樆濉烇紝鐩村埌缂撳啿鍖虹┖闂村彲鐢ㄣ€傝浼犺緭鐨勬暟鎹噺鐢?count 闅愬惈缁欏嚭銆?
#### 杩斿洖鍊?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - `1` 鏈€夋嫨 `AUDIO_SOURCE_MEMORY` 妯″紡銆?
    - ..

       - `ENOMEM`

       - 灏濊瘯鍐欏叆鐨勬暟鎹秴杩囦簡鍐呴儴缂撳啿鍖烘墍鑳藉绾崇殑閲忋€?
    - ..

       - `EBADF`

       - 鏂囦欢鎻忚堪绗︿笉鏄湁鏁堢殑宸叉墦寮€鏂囦欢鎻忚堪绗︺€?