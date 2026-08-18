

## DVB 瑙嗛璁惧锛圖VB Video Device锛?

             See: legacy_dvb_decoder_notes

DVB 瑙嗛璁惧鎺у埗 DVB 纭欢鐨?MPEG2 瑙嗛瑙ｇ爜鍣ㄣ€傚彲浠ラ€氳繃 `/dev/dvb/adapter0/video0` 璁块棶瀹冦€傛暟鎹被鍨嬪拰 ioctl 瀹氫箟鍙互閫氳繃鍦ㄥ簲鐢ㄧ▼搴忎腑鍖呭惈 `linux/dvb/video.h` 鏉ヨ闂€?
娉ㄦ剰锛孌VB 瑙嗛璁惧鍙帶鍒?MPEG 瑙嗛娴佺殑瑙ｇ爜锛岃€屼笉鏄叾鍦ㄧ數瑙嗘垨璁＄畻鏈哄睆骞曚笂鐨勫憟鐜般€傚湪 PC 涓婏紝杩欓€氬父鐢辩浉鍏崇殑 video4linux 璁惧锛堜緥濡?`/dev/video`锛夊鐞嗭紝瀹冨厑璁哥缉鏀惧拰瀹氫箟杈撳嚭绐楀彛銆?
澶у鏁?DVB 鍗℃病鏈夎嚜宸辩殑 MPEG 瑙ｇ爜鍣紝杩欏鑷撮煶棰戝拰瑙嗛璁惧浠ュ強 video4linux 璁惧琚渷鐣ャ€?
杩欎簺 ioctl 涔熸浘琚?V4L2 鐢ㄦ潵鎺у埗 V4L2 涓疄鐜扮殑 MPEG 瑙ｇ爜鍣ㄣ€傚皢杩欎簺 ioctl 鐢ㄤ簬姝ょ洰鐨勭殑鍋氭硶宸茶搴熷純锛屽苟涓斿凡缁忓垱寤轰簡閫傚綋鐨?V4L2 ioctl 鎴栨帶鍒舵潵鍙栦唬璇ュ姛鑳姐€傝涓烘柊椹卞姩浣跨敤 V4L2 ioctls<video>锛?
## 瑙嗛鏁版嵁绫诲瀷锛圴ideo Data Types锛?

### video_format_t


#### 姒傝锛圫ynopsis锛?

    typedef enum {
	VIDEO_FORMAT_4_3,
	VIDEO_FORMAT_16_9,
	VIDEO_FORMAT_221_1
    } video_format_t;

#### 甯搁噺锛圕onstants锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_FORMAT_4_3`

       - 閫夋嫨 4:3 鏍煎紡銆?
    - ..

       - `VIDEO_FORMAT_16_9`

       - 閫夋嫨 16:9 鏍煎紡銆?
    - ..

       - `VIDEO_FORMAT_221_1`

       - 閫夋嫨 2.21:1 鏍煎紡銆?
#### 鎻忚堪锛圖escription锛?

`video_format_t` 鏁版嵁绫诲瀷
鍦?`VIDEO_SET_FORMAT`_ 鍑芥暟涓敤浜庡憡璇夐┍鍔ㄨ緭鍑虹‖浠讹紙渚嬪鐢佃锛夊叿鏈夊摢绉?瀹介珮姣斻€傚畠涔熺敤浜庣敱 `VIDEO_GET_STATUS`_ 杩斿洖鐨?鏁版嵁缁撴瀯 `video_status`_ 浠ュ強鐢?`VIDEO_GET_EVENT`_ 杩斿洖鐨?`video_event`_ 涓紝杩欎簺缁撴瀯鎶ュ憡褰撳墠瑙嗛娴佺殑鏄剧ず鏍煎紡銆?

-----


### video_displayformat_t


#### 姒傝锛圫ynopsis锛?

    typedef enum {
	VIDEO_PAN_SCAN,
	VIDEO_LETTER_BOX,
	VIDEO_CENTER_CUT_OUT
    } video_displayformat_t;

#### 甯搁噺锛圕onstants锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_PAN_SCAN`

       - 浣跨敤骞崇Щ鍜屾壂鎻忥紙pan and scan锛夋牸寮忋€?
    - ..

       - `VIDEO_LETTER_BOX`

       - 浣跨敤淇＄锛坙etterbox锛夋牸寮忋€?
    - ..

       - `VIDEO_CENTER_CUT_OUT`

       - 浣跨敤涓績瑁佸壀锛坈enter cut out锛夋牸寮忋€?
#### 鎻忚堪锛圖escription锛?

濡傛灉瑙嗛娴佺殑鏄剧ず鏍煎紡涓庢樉绀虹‖浠剁殑鏄剧ず鏍煎紡涓嶅悓锛屽簲鐢ㄧ▼搴忓繀椤绘寚瀹氬浣曞鐞?鐢婚潰鐨勮鍓€傝繖鍙互閫氳繃鎺ュ彈姝ゆ灇涓句綔涓哄弬鏁扮殑
`VIDEO_SET_DISPLAY_FORMAT`_ 璋冪敤鏉ュ畬鎴愩€?

-----


### video_size_t


#### 姒傝锛圫ynopsis锛?

    typedef struct {
	int w;
	int h;
	video_format_t aspect_ratio;
    } video_size_t;

#### 鍙橀噺锛圴ariables锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int w`

       - 瑙嗛瀹藉害锛堝儚绱狅級銆?
    - ..

       - `int h`

       - 瑙嗛楂樺害锛堝儚绱狅級銆?
    - ..

       - `video_format_t`_ `aspect_ratio`

       - 瀹介珮姣斻€?
#### 鎻忚堪锛圖escription锛?

鐢ㄤ簬缁撴瀯浣?`video_event`_ 涓€傚畠瀛樺偍瑙嗛鐨勫垎杈ㄧ巼鍜屽楂樻瘮銆?

-----


### video_stream_source_t


#### 姒傝锛圫ynopsis锛?

    typedef enum {
	VIDEO_SOURCE_DEMUX,
	VIDEO_SOURCE_MEMORY
    } video_stream_source_t;

#### 甯搁噺锛圕onstants锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_SOURCE_DEMUX`

       - `1` 閫夋嫨瑙ｅ鐢ㄥ櫒锛坉emux锛変綔涓轰富婧愩€?
    - ..

       - `VIDEO_SOURCE_MEMORY`

       - 濡傛灉閫夋嫨浜嗘婧愶紝鍒欐祦
          鏉ヨ嚜鐢ㄦ埛閫氳繃 write
          绯荤粺璋冪敤銆?
#### 鎻忚堪锛圖escription锛?

瑙嗛娴佹簮閫氳繃 `VIDEO_SELECT_SOURCE`_ 璋冪敤璁剧疆锛屽苟涓旀牴鎹垜浠槸浠庡唴閮紙瑙ｅ鐢ㄥ櫒锛?杩樻槸澶栭儴锛堢敤鎴峰啓鍏ワ級婧愬洖鏀撅紝鍙互鍙栦互涓嬪€笺€?VIDEO_SOURCE_DEMUX 閫夋嫨瑙ｅ鐢ㄥ櫒锛堢敱鍓嶇鎴?DVR 璁惧鎻愪緵锛変綔涓鸿棰戞祦鐨勬簮銆傚鏋?閫夋嫨 VIDEO_SOURCE_MEMORY锛屽垯娴佹潵鑷簲鐢ㄧ▼搴忥紝閫氳繃 `write()`_ 绯荤粺璋冪敤銆?

-----


### video_play_state_t


#### 姒傝锛圫ynopsis锛?

    typedef enum {
	VIDEO_STOPPED,
	VIDEO_PLAYING,
	VIDEO_FREEZED
    } video_play_state_t;

#### 甯搁噺锛圕onstants锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_STOPPED`

       - 瑙嗛宸插仠姝€?
    - ..

       - `VIDEO_PLAYING`

       - 瑙嗛姝ｅ湪鎾斁銆?
    - ..

       - `VIDEO_FREEZED`

       - 瑙嗛宸插喕缁撱€?
#### 鎻忚堪锛圖escription锛?

杩欎簺鍊煎彲浠ョ敱 `VIDEO_GET_STATUS`_ 璋冪敤杩斿洖锛岃〃绀鸿棰戞挱鏀剧殑鐘舵€併€?

-----


### struct video_command


#### 姒傝锛圫ynopsis锛?

    struct video_command {
	__u32 cmd;
	__u32 flags;
	union {
	    struct {
		__u64 pts;
	    } stop;

	    struct {
		__s32 speed;
		__u32 format;
	    } play;

	    struct {
		__u32 data[^16^];
	    } raw;
	};
    };


#### 鍙橀噺锛圴ariables锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `__u32 cmd`

       - `Decoder command`_

    - ..

       - `__u32 flags`

       - `Decoder command`_ 鐨勬爣蹇椼€?
    - ..

       - `struct stop`

       - `__u64 pts`

       - MPEG PTS

    - ..

       - `5` `stuct play`

       - `4` `__s32 speed`

       - 0 鎴?1000 琛ㄧず姝ｅ父閫熷害锛?
    - ..

       - 1锛氳〃绀烘鍚戝崟姝ワ紝

    - ..

       - -1锛氳〃绀哄弽鍚戝崟姝ワ紝

    - ..

       - >1锛氫互姝ｅ父閫熷害鐨?speed / 1000 鍊嶆挱鏀?
    - ..

       - <-1锛氫互姝ｅ父閫熷害鐨?( -speed / 1000 ) 鍊嶅弽鍚戞挱鏀俱€?
    - ..

       - `__u32 format`

       - `Play input formats`_

    - ..

       - `__u32 data[^16^]`

       - 淇濈暀

#### 鎻忚堪锛圖escription锛?

璇ョ粨鏋勪綋鍦ㄤ娇鐢ㄥ墠蹇呴』鐢卞簲鐢ㄧ▼搴忔竻闆躲€傝繖纭繚浜嗗畠灏嗘潵鍙互瀹夊叏鍦版墿灞曘€?

-----


### 棰勫畾涔夌殑璇戠爜鍣ㄥ懡浠や笌鏍囧織锛圥redefined decoder commands and flags锛?

#### 姒傝锛圫ynopsis锛?

    #define VIDEO_CMD_PLAY                      (0)
    #define VIDEO_CMD_STOP                      (1)
    #define VIDEO_CMD_FREEZE                    (2)
    #define VIDEO_CMD_CONTINUE                  (3)

    #define VIDEO_CMD_FREEZE_TO_BLACK      (1 << 0)

    #define VIDEO_CMD_STOP_TO_BLACK        (1 << 0)
    #define VIDEO_CMD_STOP_IMMEDIATELY     (1 << 1)

    #define VIDEO_PLAY_FMT_NONE                 (0)
    #define VIDEO_PLAY_FMT_GOP                  (1)

    #define VIDEO_VSYNC_FIELD_UNKNOWN           (0)
    #define VIDEO_VSYNC_FIELD_ODD               (1)
    #define VIDEO_VSYNC_FIELD_EVEN              (2)
    #define VIDEO_VSYNC_FIELD_PROGRESSIVE       (3)

#### 甯搁噺锛圕onstants锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `3` _`Decoder command`

       - `VIDEO_CMD_PLAY`

       - 寮€濮嬫挱鏀俱€?
    - ..

       - `VIDEO_CMD_STOP`

       - 鍋滄鎾斁銆?
    - ..

       - `VIDEO_CMD_FREEZE`

       - 鍐荤粨鎾斁銆?
    - ..

       - `VIDEO_CMD_CONTINUE`

       - 鍐荤粨鍚庣户缁挱鏀俱€?
    - ..

       - `VIDEO_CMD_FREEZE` 鐨勬爣蹇?
       - `VIDEO_CMD_FREEZE_TO_BLACK`

       - 鍐荤粨鏃舵樉绀洪粦灞忋€?
    - ..

       - `1` `VIDEO_CMD_STOP` 鐨勬爣蹇?
       - `VIDEO_CMD_STOP_TO_BLACK`

       - 鍋滄鏃舵樉绀洪粦灞忋€?
    - ..

       - `VIDEO_CMD_STOP_IMMEDIATELY`

       - 绔嬪嵆鍋滄锛屼笉鎺掔┖缂撳啿鍖恒€?
    - ..

       - `1` _`Play input formats`

       - `VIDEO_PLAY_FMT_NONE`

       - 瑙ｇ爜鍣ㄦ病鏈夌壒娈婄殑鏍煎紡瑕佹眰

    - ..

       - `VIDEO_PLAY_FMT_GOP`

       - 瑙ｇ爜鍣ㄩ渶瑕佸畬鏁寸殑 GOP

    - ..

       - `3` 鍦洪『搴忥紙Field order锛?
       - `VIDEO_VSYNC_FIELD_UNKNOWN`

       - 濡傛灉纭欢涓嶇煡閬?Vsync 鏄搴斿鏁板満銆?          鍋舵暟鍦鸿繕鏄€愯锛堝嵆闈為殧琛岋級鍦猴紝鍙互浣跨敤 FIELD_UNKNOWN銆?
    - ..

       - `VIDEO_VSYNC_FIELD_ODD`

       - Vsync 瀵瑰簲濂囨暟鍦恒€?
    - ..

       - `VIDEO_VSYNC_FIELD_EVEN`

       - Vsync 瀵瑰簲鍋舵暟鍦恒€?
    - ..

       - `VIDEO_VSYNC_FIELD_PROGRESSIVE`

       - 閫愯锛堝嵆闈為殧琛岋級


-----


### video_event


#### 姒傝锛圫ynopsis锛?

    struct video_event {
	__s32 type;
    #define VIDEO_EVENT_SIZE_CHANGED        1
    #define VIDEO_EVENT_FRAME_RATE_CHANGED  2
    #define VIDEO_EVENT_DECODER_STOPPED     3
    #define VIDEO_EVENT_VSYNC               4
	long timestamp;
	union {
	    video_size_t size;
	    unsigned int frame_rate;
	    unsigned char vsync_field;
	} u;
    };

#### 鍙橀噺锛圴ariables锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `4` `__s32 type`

       - `1` 浜嬩欢绫诲瀷銆?
    - ..

       - `VIDEO_EVENT_SIZE_CHANGED`

       - 灏哄宸叉敼鍙樸€?
    - ..

       - `VIDEO_EVENT_FRAME_RATE_CHANGED`

       - 甯х巼宸叉敼鍙樸€?
    - ..

       - `VIDEO_EVENT_DECODER_STOPPED`

       - 瑙ｇ爜鍣ㄥ凡鍋滄銆?
    - ..

       - `VIDEO_EVENT_VSYNC`

       - 鍙戠敓浜?Vsync銆?
    - ..

       - `long timestamp`

       - `1` 鍙戠敓鏃剁殑 MPEG PTS銆?
    - ..

       - `2` `union u`

       - `video_size_t`_ size

       - 瑙嗛鐨勫垎杈ㄧ巼鍜屽楂樻瘮銆?
    - ..

       - `unsigned int frame_rate`

       - 鍗曚綅涓烘瘡 1000 绉掔殑甯ф暟

    - ..

       - `unsigned char vsync_field`

       - | unknown / odd / even / progressive
          | 鍙傝锛歚Predefined decoder commands and flags`_

#### 鎻忚堪锛圖escription锛?

杩欐槸 `VIDEO_GET_EVENT`_ 璋冪敤杩斿洖鐨?瑙嗛浜嬩欢鐨勭粨鏋勩€傛洿澶氱粏鑺傝鍙傝閭ｉ噷銆?

-----


### video_status


#### 姒傝锛圫ynopsis锛?

`VIDEO_GET_STATUS`_ 璋冪敤杩斿洖浠ヤ笅缁撴瀯浣擄紝鍛婄煡
鎾斁鎿嶄綔鐨勫悇绉嶇姸鎬併€?

    struct video_status {
	int                    video_blank;
	video_play_state_t     play_state;
	video_stream_source_t  stream_source;
	video_format_t         video_format;
	video_displayformat_t  display_format;
    };

#### 鍙橀噺锛圴ariables锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `2` `int video_blank`

       - `1` 鍐荤粨鏃舵槸鍚︽樉绀虹┖鐧借棰戯紵

    - ..

       - TRUE  ( != 0 )

       - 鍐荤粨鏃堕粦灞忋€?
    - ..

       - FALSE ( == 0 )

       - 鏄剧ず鏈€鍚庤В鐮佺殑甯с€?
    - ..

       - `video_play_state_t`_ `play_state`

       - 褰撳墠鐨勬挱鏀剧姸鎬併€?
    - ..

       - `video_stream_source_t`_ `stream_source`

       - 褰撳墠婧愶紙demux/memory锛夈€?
    - ..

       - `video_format_t`_ `video_format`

       - 娴佺殑褰撳墠瀹介珮姣斻€?
    - ..

       - `video_displayformat_t`_ `display_format`

       - 搴旂敤鐨勮鍓ā寮忋€?
#### 鎻忚堪锛圖escription锛?

濡傛灉 `video_blank` 琚缃负 `TRUE`锛屽垯鍦ㄥ垏鎹㈤閬撴垨鍋滄鎾斁鏃惰棰戝皢琚?娓呯┖銆傚惁鍒欙紝灏嗘樉绀烘渶鍚庝竴骞呯敾闈€俙play_state` 鎸囩ず瑙嗛褰撳墠鏄喕缁撱€佸仠姝㈣繕鏄?姝ｅ湪鎾斁銆俙stream_source` 瀵瑰簲浜庝负瑙嗛娴侀€夋嫨鐨勬簮銆傚畠鍙互鏉ヨ嚜
瑙ｅ鐢ㄥ櫒鎴栨潵鑷唴瀛樸€俙video_format` 鎸囩ず褰撳墠鎾斁鐨勮棰戞祦鐨勫楂樻瘮
锛?:3 鎴?16:9 涔嬩竴锛夈€傛渶鍚庯紝`display_format` 鍦ㄦ簮瑙嗛鏍煎紡涓庤緭鍑?璁惧鐨勬牸寮忎笉鍚屾椂锛屽搴斾簬鎵€搴旂敤鐨勮鍓ā寮忋€?

-----


### video_still_picture


#### 姒傝锛圫ynopsis锛?

    struct video_still_picture {
    char *iFrame;
    int32_t size;
    };

#### 鍙橀噺锛圴ariables锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `char *iFrame`

       - 鎸囧悜鍐呭瓨涓崟涓?I 甯х殑鎸囬拡銆?
    - ..

       - `int32_t size`

       - I 甯х殑澶у皬銆?

#### 鎻忚堪锛圖escription锛?

閫氳繃 `VIDEO_STILLPICTURE`_ 璋冪敤鏄剧ず鐨?I 甯у湪姝ょ粨鏋勪綋涓浼犲叆銆?

-----


### video capabilities


#### 姒傝锛圫ynopsis锛?

    #define VIDEO_CAP_MPEG1   1
    #define VIDEO_CAP_MPEG2   2
    #define VIDEO_CAP_SYS     4
    #define VIDEO_CAP_PROG    8

#### 甯搁噺锛圕onstants锛?

鑳藉姏浣嶇殑瀹氫箟锛?
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_CAP_MPEG1`

       - `1` 纭欢鍙互瑙ｇ爜 MPEG1銆?
    - ..

       - `VIDEO_CAP_MPEG2`

       - 纭欢鍙互瑙ｇ爜 MPEG2銆?
    - ..

       - `VIDEO_CAP_SYS`

       - 瑙嗛璁惧鎺ュ彈绯荤粺娴侊紙system stream锛夈€?
          浣犱粛鐒跺繀椤绘墦寮€瑙嗛鍜岄煶棰戣澶囷紝
          浣嗗彧灏嗘祦鍙戦€佸埌瑙嗛璁惧銆?
    - ..

       - `VIDEO_CAP_PROG`

       - 瑙嗛璁惧鎺ュ彈鑺傜洰娴侊紙program stream锛夈€?
          浣犱粛鐒跺繀椤绘墦寮€瑙嗛鍜岄煶棰戣澶囷紝
          浣嗗彧灏嗘祦鍙戦€佸埌瑙嗛璁惧銆?
#### 鎻忚堪锛圖escription锛?

瀵?`VIDEO_GET_CAPABILITIES`_ 鐨勮皟鐢ㄨ繑鍥炰竴涓棤绗﹀彿鏁存暟锛屽叾鏍规嵁
纭欢鐨勮兘鍔涜缃簡浠ヤ笅浣嶃€?

-----


## 瑙嗛鍑芥暟璋冪敤锛圴ideo Function Calls锛?

### VIDEO_STOP


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_STOP, int mode)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵规鍛戒护绛変簬 `VIDEO_STOP`銆?
    - ..

       - `2` `int mode`

       - `1` 鎸囩ず搴斿浣曞鐞嗗睆骞曘€?
    - ..

       - TRUE  ( != 0 )

       - 鍋滄鏃堕粦灞忋€?
    - ..

       - FALSE ( == 0 )

       - 鏄剧ず鏈€鍚庤В鐮佺殑甯с€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜庢暟瀛楃數瑙嗭紙Digital TV锛夎澶囥€傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2 VIDIOC_DECODER_CMD銆?
姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧鍋滄鎾斁褰撳墠娴併€傛牴鎹緭鍏ュ弬鏁帮紝灞忓箷鍙互琚竻绌烘垨
鏄剧ず鏈€鍚庤В鐮佺殑甯с€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_PLAY


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_PLAY)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_PLAY`銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜庢暟瀛楃數瑙嗚澶囥€傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2 VIDIOC_DECODER_CMD銆?
姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧寮€濮嬩粠鎵€閫夋簮鎾斁瑙嗛娴併€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_FREEZE


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_FREEZE)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_FREEZE`銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜庢暟瀛楃數瑙嗚澶囥€傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2 VIDIOC_DECODER_CMD銆?
濡傛灉閫夋嫨浜?VIDEO_SOURCE_DEMUX锛屾 ioctl 璋冪敤浼氭寕璧锋鍦ㄦ挱鏀剧殑瀹炴椂瑙嗛娴併€?瑙ｇ爜鍜屾挱鏀捐鍐荤粨銆備箣鍚庡彲浠ヤ娇鐢?`VIDEO_CONTINUE`_ 鍛戒护閲嶅惎瑙嗛娴佺殑
瑙ｇ爜鍜屾挱鏀捐繃绋嬨€?濡傛灉鍦?ioctl 璋冪敤 `VIDEO_SELECT_SOURCE`_ 涓€夋嫨浜?VIDEO_SOURCE_MEMORY锛?鍒欏湪鎵ц `VIDEO_CONTINUE`_ 鎴?`VIDEO_PLAY`_ ioctl 璋冪敤涔嬪墠锛屾暟瀛楃數瑙嗗瓙绯荤粺
灏嗕笉浼氳В鐮佷换浣曟洿澶氭暟鎹€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_CONTINUE


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_CONTINUE)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_CONTINUE`銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜庢暟瀛楃數瑙嗚澶囥€傝鎺у埗 V4L2 瑙ｇ爜鍣紝璇锋敼鐢?V4L2 VIDIOC_DECODER_CMD銆?
姝?ioctl 璋冪敤閲嶅惎鍦ㄨ皟鐢?`VIDEO_FREEZE`_ 涔嬪墠鎾斁鐨勮棰戞祦鐨勮В鐮佸拰鎾斁杩囩▼銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_SELECT_SOURCE


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_SELECT_SOURCE, video_stream_source_t source)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_SELECT_SOURCE`銆?
    - ..

       - `video_stream_source_t`_ `source`

       - 鎸囩ず瑙嗛娴佸簲浣跨敤鍝釜婧愩€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜庢暟瀛楃數瑙嗚澶囥€傛 ioctl 涔熸浘琚?V4L2 ivtv 椹卞姩鏀寔锛屼絾宸茶
ivtv 鐗规湁鐨?`IVTV_IOC_PASSTHROUGH_MODE` ioctl 鍙栦唬銆?
姝?ioctl 璋冪敤鍛婄煡瑙嗛璁惧杈撳叆鏁版嵁搴斾娇鐢ㄥ摢涓簮銆傚彲鑳界殑婧愭槸 demux 鎴?memory銆?濡傛灉閫夋嫨 memory锛屽垯鏁版嵁閫氳繃 write 鍛戒护浣跨敤缁撴瀯浣?`video_stream_source_t`_
棣堥€佺粰瑙嗛璁惧銆傚鏋滈€夋嫨 demux锛屽垯鏁版嵁鐩存帴浠庢澘杞借В澶嶇敤璁惧浼犺緭鍒拌В鐮佸櫒銆?
棣堥€佺粰瑙ｇ爜鍣ㄧ殑鏁版嵁涔熺敱 PID 杩囨护鍣ㄦ帶鍒躲€傝緭鍑洪€夋嫨锛歚dmx_output`
`DMX_OUT_DECODER`銆?

#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_SET_BLANK


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, VIDEO_SET_BLANK, int mode)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - `1` 瀵规鍛戒护绛変簬 `VIDEO_SET_BLANK`銆?
    - ..

       - `2` `int mode`

       - `1` 鎸囩ず灞忓箷鏄惁搴旇娓呯┖銆?
    - ..

       - TRUE  ( != 0 )

       - 鍋滄鏃堕粦灞忋€?
    - ..

       - FALSE ( == 0 )

       - 鏄剧ず鏈€鍚庤В鐮佺殑甯с€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧娓呯┖鐢婚潰銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_GET_STATUS


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_GET_STATUS,
	struct video_status *status)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_STATUS`銆?
    - ..

       - `struct` `video_status`_ `*status`

       - 杩斿洖瑙嗛璁惧鐨勫綋鍓嶇姸鎬併€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧杩斿洖璁惧鐨勫綋鍓嶇姸鎬併€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_GET_EVENT


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_GET_EVENT,
	struct video_event *ev)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_EVENT`銆?
    - ..

       - `struct` `video_event`_ `*ev`

       - 鎸囧悜鑻ュ瓨鍦ㄥ垯浜嬩欢瑕佸瓨鍌ㄧ殑浣嶇疆銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 浠呯敤浜?DVB 璁惧銆傝浠?V4L2 瑙ｇ爜鍣ㄨ幏鍙栦簨浠讹紝璇锋敼鐢?V4L2 VIDIOC_DQEVENT ioctl銆?
姝?ioctl 璋冪敤鍦ㄥ彲鐢ㄦ椂杩斿洖 `video_event`_ 绫诲瀷鐨勪簨浠躲€備竴瀹氭暟閲忕殑
鏈€鏂颁簨浠跺皢琚帓闃熷苟鎸夊彂鐢熼『搴忚繑鍥炪€傚鏋滀笉鍙婃椂鑾峰彇锛岃緝鏃х殑浜嬩欢鍙兘浼氳涓㈠純銆傚鏋?娌℃湁鍙敤浜嬩欢锛岃涓哄彇鍐充簬璁惧澶勪簬闃诲杩樻槸闈為樆濉炴ā寮忋€傚湪鍚庤€呮儏鍐典笅锛岃皟鐢ㄤ細绔嬪嵆
澶辫触锛宔rrno 琚缃负 `EWOULDBLOCK`銆傚湪鍓嶈€呮儏鍐典笅锛岃皟鐢ㄤ細闃诲鐩村埌鏈変簨浠跺彲鐢ㄣ€?鏍囧噯鐨?Linux poll() 鍜?鎴?select() 绯荤粺璋冪敤鍙互涓庤澶囨枃浠舵弿杩扮涓€璧蜂娇鐢?浠ョ洃瑙嗘柊浜嬩欢銆傚浜?select()锛屾枃浠舵弿杩扮搴斿寘鍚湪 exceptfds 鍙傛暟涓紝瀵逛簬 poll()锛?搴旀寚瀹?POLLPRI 浣滀负鍞ら啋鏉′欢銆傛 ioctl 璋冪敤鍙渶璇绘潈闄愬嵆鍙€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EWOULDBLOCK`

       - `1` 娌℃湁寰呭鐞嗕簨浠讹紝涓旇澶囧浜?          闈為樆濉炴ā寮忋€?
    - ..

       - `EOVERFLOW`

       - 浜嬩欢闃熷垪婧㈠嚭鈥斺€斾涪澶变簡涓€涓垨澶氫釜浜嬩欢銆?

-----


### VIDEO_SET_DISPLAY_FORMAT


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_SET_DISPLAY_FORMAT,
	video_display_format_t format)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_SET_DISPLAY_FORMAT`銆?
    - ..

       - `video_displayformat_t`_ `format`

       - 閫夋嫨瑕佷娇鐢ㄧ殑瑙嗛鏍煎紡銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧閫夋嫨瑕佺敱 MPEG 鑺墖搴旂敤浜庤棰戠殑
瑙嗛鏍煎紡銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_STILLPICTURE


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_STILLPICTURE,
	struct video_still_picture *sp)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_STILLPICTURE`銆?
    - ..

       - `struct` `video_still_picture`_ `*sp`

       - 鎸囧悜瀛樺偍甯︽湁 I 甯у拰澶у皬鐨勭粨鏋勪綋鐨勪綅缃殑鎸囬拡銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧鏄剧ず闈欐鐢婚潰锛圛 甯э級銆傝緭鍏ユ暟鎹簲鏄寘鍚?I 甯х殑
鍩烘湰瑙嗛娴佺殑涓€閮ㄥ垎銆傞€氬父姝ら儴鍒嗘槸浠?TS 鎴?PES 褰曞埗涓彁鍙栫殑銆傝澶囧繀椤绘敮鎸?鍒嗚鲸鐜囧拰缂栬В鐮佸櫒锛堝弬瑙?`video capabilities`_锛夈€傚鏋滄寚閽堜负 NULL锛屽垯褰撳墠鐨?鏄剧ず闈欐鐢婚潰灏嗚娓呯┖銆?
渚嬪锛孉V7110 鏀寔鍏锋湁甯哥敤 PAL-SD 鍒嗚鲸鐜囩殑 MPEG1 鍜?MPEG2銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_FAST_FORWARD


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_FAST_FORWARD, int nFrames)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_FAST_FORWARD`銆?
    - ..

       - `int nFrames`

       - 瑕佽烦杩囩殑甯ф暟銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧璺宠繃瀵?N 涓?I 甯х殑瑙ｇ爜銆傛璋冪敤鍙兘鍦ㄩ€夋嫨浜?`VIDEO_SOURCE_MEMORY` 鏃朵娇鐢ㄣ€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - 鏈€夋嫨 `VIDEO_SOURCE_MEMORY` 妯″紡銆?

-----


### VIDEO_SLOWMOTION


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_SLOWMOTION, int nFrames)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_SLOWMOTION`銆?
    - ..

       - `int nFrames`

       - 姣忓抚閲嶅鐨勬鏁般€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧灏嗘瘡甯цВ鐮侀噸澶?N 娆°€傛璋冪敤鍙兘鍦ㄩ€夋嫨浜?`VIDEO_SOURCE_MEMORY` 鏃朵娇鐢ㄣ€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - 鏈€夋嫨 `VIDEO_SOURCE_MEMORY` 妯″紡銆?

-----


### VIDEO_GET_CAPABILITIES


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_GET_CAPABILITIES, unsigned int *cap)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_CAPABILITIES`銆?
    - ..

       - `unsigned int *cap`

       - 鎸囧悜瀛樺偍鑳藉姏淇℃伅鐨勪綅缃殑鎸囬拡銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤璇㈤棶瑙嗛璁惧鐨勮В鐮佽兘鍔涖€傛垚鍔熸椂瀹冭繑鍥炰竴涓暣鏁帮紝鍏舵牴鎹?`video capabilities`_ 涓殑瀹氫箟璁剧疆浜嗙浉搴旂殑浣嶃€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_CLEAR_BUFFER


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_CLEAR_BUFFER)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_CLEAR_BUFFER`銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璋冪敤娓呴櫎椹卞姩鍜岃В鐮佸櫒纭欢涓殑鎵€鏈夎棰戠紦鍐插尯銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_SET_STREAMTYPE


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_SET_STREAMTYPE, int type)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_SET_STREAMTYPE`銆?
    - ..

       - `int type`

       - 娴佺被鍨嬨€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 鍛婄煡椹卞姩鏈熸湜鍐欏叆鍏朵腑鐨勬祦绫诲瀷鏄粈涔堛€?鏅鸿兘瑙ｇ爜鍣ㄤ篃鍙兘涓嶆敮鎸佹垨蹇界暐锛堝 AV7110锛夋璋冪敤锛岃€岃嚜琛岀‘瀹氭祦绫诲瀷銆?
褰撳墠浣跨敤鐨勬祦绫诲瀷锛?
    :header-rows:  1
    :stub-columns: 0

    - ..

       - Codec

       - Stream type

    - ..

       - MPEG2

       - 0

    - ..

       - MPEG4 h.264

       - 1

    - ..

       - VC1

       - 3

    - ..

       - MPEG4 Part2

       - 4

    - ..

       - VC1 SM

       - 5

    - ..

       - MPEG1

       - 6

    - ..

       - HEVC h.265

       - | 7
          | DREAMBOX: 22

    - ..

       - AVS

       - 16

    - ..

       - AVS2

       - 40

骞堕潪姣忎釜瑙ｇ爜鍣ㄩ兘鏀寔鎵€鏈夋祦绫诲瀷銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_SET_FORMAT


#### 姒傝锛圫ynopsis锛?


	int ioctl(fd, int request = VIDEO_SET_FORMAT, video_format_t format)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_SET_FORMAT`銆?
    - ..

       - `video_format_t`_ `format`

       - TV 鐨勮棰戞牸寮忥紝濡?`video_format_t`_ 鑺傛墍瀹氫箟銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 璁剧疆鎵€杩炴帴杈撳嚭璁惧锛圱V锛夌殑灞忓箷鏍煎紡锛堝楂樻瘮锛夛紝浠ヤ究鐩稿簲璋冩暣
瑙ｇ爜鍣ㄧ殑杈撳嚭銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_GET_SIZE


#### 姒傝锛圫ynopsis锛?


	int ioctl(int fd, int request = VIDEO_GET_SIZE, video_size_t *size)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑鏂囦欢鎻忚堪绗︼紝

    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_SIZE`銆?
    - ..

       - `video_size_t`_ `*size`

       - 杩斿洖灏哄鍜屽楂樻瘮銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

姝?ioctl 杩斿洖灏哄鍜屽楂樻瘮銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_GET_PTS


#### 姒傝锛圫ynopsis锛?


	int ioctl(int fd, int request = VIDEO_GET_PTS, __u64 *pts)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_PTS`銆?
    - ..

       - `__u64 *pts`

       - 杩斿洖 ITU T-REC-H.222.0 /
          ISO/IEC 13818-1 瀹氫箟鐨?33 浣嶆椂闂存埑銆?
          濡傛灉鍙兘锛孭TS 搴斿睘浜庡綋鍓嶆挱鏀剧殑甯э紝浣嗕篃鍙兘鏄竴涓帴杩戝畠鐨勫€硷紝
          渚嬪鏈€鍚庤В鐮佸抚鐨?PTS 鎴?PES 瑙ｆ瀽鍣ㄦ彁鍙栫殑鏈€鍚庝竴涓?PTS銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

瀵逛簬 V4L2 瑙ｇ爜鍣紝姝?ioctl 宸茶 `V4L2_CID_MPEG_VIDEO_DEC_PTS` 鎺у埗鍙栦唬銆?
姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧杩斿洖褰撳墠鐨?PTS 鏃堕棿鎴炽€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_GET_FRAME_COUNT


#### 姒傝锛圫ynopsis锛?


	int ioctl(int fd, VIDEO_GET_FRAME_COUNT, __u64 *pts)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_GET_FRAME_COUNT`銆?
    - ..

       - `__u64 *pts`

       - 杩斿洖鑷В鐮佸櫒鍚姩浠ユ潵鏄剧ず鐨勫抚鏁般€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

瀵逛簬 V4L2 瑙ｇ爜鍣紝姝?ioctl 宸茶 `V4L2_CID_MPEG_VIDEO_DEC_FRAME` 鎺у埗鍙栦唬銆?
姝?ioctl 璋冪敤瑕佹眰瑙嗛璁惧杩斿洖鑷В鐮佸櫒鍚姩浠ユ潵鏄剧ず鐨勫抚鏁般€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_COMMAND


#### 姒傝锛圫ynopsis锛?


	int ioctl(int fd, int request = VIDEO_COMMAND,
	struct video_command *cmd)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_COMMAND`銆?
    - ..

       - `struct video_command`_ `*cmd`

       - 鍛戒护瑙ｇ爜鍣ㄣ€?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

瀵逛簬 V4L2 瑙ｇ爜鍣紝姝?ioctl 宸茶 VIDIOC_DECODER_CMD ioctl 鍙栦唬銆?
姝?ioctl 鍛戒护瑙ｇ爜鍣ㄣ€俙struct video_command`_ 鏄?`v4l2_decoder_cmd`
缁撴瀯浣撶殑涓€涓瓙闆嗭紝鍥犳璇峰弬闃?VIDIOC_DECODER_CMD 鏂囨。浠ヨ幏鍙?鏇村淇℃伅銆?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### VIDEO_TRY_COMMAND


#### 姒傝锛圫ynopsis锛?


	int ioctl(int fd, int request = VIDEO_TRY_COMMAND,
	struct video_command *cmd)

#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `int request`

       - 瀵规鍛戒护绛変簬 `VIDEO_TRY_COMMAND`銆?
    - ..

       - `struct video_command`_ `*cmd`

       - 灏濊瘯涓€涓В鐮佸櫒鍛戒护銆?
#### 鎻忚堪锛圖escription锛?

             See: legacy_dvb_decoder_notes

瀵逛簬 V4L2 瑙ｇ爜鍣紝姝?ioctl 宸茶 VIDIOC_TRY_DECODER_CMD <VIDIOC_DECODER_CMD> ioctl 鍙栦唬銆?
姝?ioctl 灏濊瘯涓€涓В鐮佸櫒鍛戒护銆俙struct video_command`_ 鏄?`v4l2_decoder_cmd`
缁撴瀯浣撶殑涓€涓瓙闆嗭紝鍥犳璇峰弬闃?VIDIOC_TRY_DECODER_CMD <VIDIOC_DECODER_CMD> 鏂囨。
浠ヨ幏鍙栨洿澶氫俊鎭€?
#### 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺浼氳閫傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

-----


### open()


#### 姒傝锛圫ynopsis锛?


    #include <fcntl.h>


#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 鐗瑰畾瑙嗛璁惧鐨勫悕绉般€?
    - ..

       - `3` `int flags`

       - `1` 浠ヤ笅鏍囧織鐨勬寜浣嶆垨锛?
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
#### 鎻忚堪锛圖escription锛?

姝ょ郴缁熻皟鐢ㄦ墦寮€涓€涓叿鍚嶇殑瑙嗛璁惧锛堜緥濡?/dev/dvb/adapter?/video?锛変互渚涘悗缁娇鐢ㄣ€?
褰?open() 璋冪敤鎴愬姛鍚庯紝璁惧灏嗗噯澶囧氨缁彲渚涗娇鐢ㄣ€傞樆濉炴垨闈為樆濉炴ā寮忕殑鎰忎箟鍦?瀛樺湪宸紓鐨勫嚱鏁版枃妗ｄ腑鎻忚堪銆傚畠涓嶅奖鍝?open() 璋冪敤鏈韩鐨勮涔夈€備互
闃诲妯″紡鎵撳紑鐨勮澶囦箣鍚庡彲浠ヤ娇鐢?fcntl 绯荤粺璋冪敤鐨?F_SETFL 鍛戒护鍒囨崲鍒伴潪闃诲妯″紡
锛堝弽涔嬩害鐒讹級銆傝繖鏄竴涓爣鍑嗙殑绯荤粺璋冪敤锛屽湪 Linux 鐨?fcntl 鎵嬪唽椤典腑鏈夋枃妗ｃ€?鍙湁涓€涓敤鎴峰彲浠ヤ互 O_RDWR 妯″紡鎵撳紑瑙嗛璁惧銆傛墍鏈夊叾浠栦互璇ユā寮忔墦寮€璁惧鐨勫皾璇?閮藉皢澶辫触锛屽苟杩斿洖閿欒鐮併€傚鏋滀互 O_RDONLY 妯″紡鎵撳紑瑙嗛璁惧锛屽垯鍞竴鍙互浣跨敤鐨?ioctl 璋冪敤鏄?`VIDEO_GET_STATUS`_銆傛墍鏈夊叾浠栬皟鐢ㄩ兘灏嗚繑鍥為敊璇爜銆?
#### 杩斿洖鍊硷紙Return Value锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - `1` 璁惧椹卞姩鏈姞杞?涓嶅彲鐢ㄣ€?
    - ..

       - `EINTERNAL`

       - 鍐呴儴閿欒銆?
    - ..

       - `EBUSY`

       - 璁惧鎴栬祫婧愬繖銆?
    - ..

       - `EINVAL`

       - 鏃犳晥鍙傛暟銆?

-----


### close()


#### 姒傝锛圫ynopsis锛?


#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
#### 鎻忚堪锛圖escription锛?

姝ょ郴缁熻皟鐢ㄥ叧闂厛鍓嶆墦寮€鐨勮棰戣澶囥€?
#### 杩斿洖鍊硷紙Return Value锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - fd 涓嶆槸鏈夋晥鐨勬墦寮€鏂囦欢鎻忚堪绗︺€?

-----


### write()


#### 姒傝锛圫ynopsis锛?


#### 鍙傛暟锛圓rguments锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鍏堝墠瀵?`open()`_ 鐨勮皟鐢ㄨ繑鍥炵殑
          鏂囦欢鎻忚堪绗︺€?
    - ..

       - `void *buf`

       - 鎸囧悜鍖呭惈 PES 鏁版嵁鐨勭紦鍐插尯鐨勬寚閽堛€?
    - ..

       - `size_t count`

       - buf 鐨勫ぇ灏忋€?
#### 鎻忚堪锛圖escription锛?

姝ょ郴缁熻皟鐢ㄥ彧鑳藉湪 ioctl 璋冪敤 `VIDEO_SELECT_SOURCE`_ 涓€夋嫨浜?VIDEO_SOURCE_MEMORY 鏃?浣跨敤銆傛墍鎻愪緵鐨勬暟鎹簲涓?PES 鏍煎紡锛岄櫎闈炶兘鍔涘厑璁稿叾浠栨牸寮忋€俆S 鏄瓨鍌?DVB 鏁版嵁
鏈€甯歌鐨勬牸寮忥紝閫氬父涔熷彈鏀寔銆傚鏋滄湭鎸囧畾 O_NONBLOCK锛岃鍑芥暟灏嗛樆濉炵洿鍒版湁缂撳啿鍖虹┖闂?鍙敤銆傝浼犺緭鐨勬暟鎹噺鐢?count 闅愬紡纭畾銆?

#### 杩斿洖鍊硷紙Return Value锛?

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - `1` 鏈€夋嫨 `VIDEO_SOURCE_MEMORY` 妯″紡銆?
    - ..

       - `ENOMEM`

       - 璇曞浘鍐欏叆鐨勬暟鎹秴杩囧唴閮ㄧ紦鍐插尯鍙绾崇殑閲忋€?
    - ..

       - `EBADF`

       - fd 涓嶆槸鏈夋晥鐨勬墦寮€鏂囦欢鎻忚堪绗︺€?