


## DVB OSD 璁惧

             See: legacy_dvb_decoder_notes

DVB OSD 璁惧鎺у埗鍩轰簬 AV7110銆佸甫鏈夌‖浠?MPEG2 瑙ｇ爜鍣ㄧ殑 DVB 鍗＄殑灞忎笂鏄剧ず锛圤nScreen-Display锛夈€傚畠鍙互閫氳繃 `/dev/dvb/adapter?/osd0` 璁块棶銆傛暟鎹被鍨嬪拰 ioctl 瀹氫箟鍙互閫氳繃鍦ㄥ簲鐢ㄧ▼搴忎腑鍖呭惈 `linux/dvb/osd.h` 鏉ヤ娇鐢ㄣ€?
OSD 骞朵笉鍍忚澶氬叾浠栧崱閭ｆ牱鏄竴涓抚缂撳啿锛坒rame-buffer锛夈€傚畠鏇村儚鏄竴鍧楀彲浠ョ粯鍒剁殑鐢诲竷锛坈anvas锛夈€傝壊娣卞彈鎵€瀹夎鍐呭瓨澶у皬鐨勯檺鍒躲€傚繀椤诲缓绔嬩竴濂楀悎閫傜殑璋冭壊鏉裤€傛墍瀹夎鐨勫唴瀛樺ぇ灏忓彲浠ョ敤 `OSD_GET_CAPABILITY`_ ioctl 鏉ヨ瘑鍒€?
## OSD 鏁版嵁绫诲瀷

### OSD_Command

#### Synopsis锛堟瑕侊級


    typedef enum {
	/** All functions return -2 on "not open" **/
	OSD_Close = 1,
	OSD_Open,
	OSD_Show,
	OSD_Hide,
	OSD_Clear,
	OSD_Fill,
	OSD_SetColor,
	OSD_SetPalette,
	OSD_SetTrans,
	OSD_SetPixel,
	OSD_GetPixel,
	OSD_SetRow,
	OSD_SetBlock,
	OSD_FillRow,
	OSD_FillBlock,
	OSD_Line,
	OSD_Query,
	OSD_Test,
	OSD_Text,
	OSD_SetWindow,
	OSD_MoveWindow,
	OSD_OpenRaw,
    } OSD_Command;

#### Commands锛堝懡浠わ級


    :header-rows:  1
    :stub-columns: 0

    - ..

       - Command

       - | 鎵€浣跨敤鐨?`struct` `osd_cmd_t`_ 鍙橀噺銆?          | 濡備负鍙€夌敤娉曪紝鍒欎负 Usage{variable}銆?
       - `2` Description

    - ..

       - `OSD_Close`

       - -

       - | 绂佺敤 OSD 骞堕噴鏀剧紦鍐插尯銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_Open`

       - | x0,y0,x1,y1,
          | BitPerPixel[2/4/8]{color&0x0F},
          | mix[0..15]{color&0xF0}

       - | 浠ヨ灏哄鍜屼綅娣辨墦寮€ OSD銆?          | 鎴愬姛鏃惰繑鍥?0锛?          | DRAM 鍒嗛厤閿欒鏃惰繑鍥?-1锛?          | 鈥滃凡缁忔墦寮€鈥?鏃惰繑鍥?-2銆?
    - ..

       - `OSD_Show`

       - -

       - | 鍚敤 OSD 妯″紡銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_Hide`

       - -

       - | 绂佺敤 OSD 妯″紡銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_Clear`

       - -

       - | 灏嗘墍鏈夊儚绱犺涓洪鑹?0銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_Fill`

       - color

       - | 灏嗘墍鏈夊儚绱犺涓洪鑹?<color>銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_SetColor`

       - | color,
          | R{x0},G{y0},B{x1},
          | opacity{y1}

       - | 灏嗚皟鑹叉澘鏉＄洰 <num> 璁句负 <r,g,b>锛?mix> 鍜?<trans> 鐢熸晥銆?          | R,G,B: 0..255
          | R=绾紙Red锛夛紝G=缁匡紙Green锛夛紝B=钃濓紙Blue锛?          | opacity=0:      鍍忕礌涓嶉€忔槑搴?0%锛堝彧鏄剧ず瑙嗛鍍忕礌锛?          | opacity=1..254: 鍍忕礌涓嶉€忔槑搴﹀澶撮儴鎵€鎸囧畾
          | opacity=255:    鍍忕礌涓嶉€忔槑搴?100%锛堝彧鏄剧ず OSD 鍍忕礌锛?          | 鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1銆?
    - ..

       - `OSD_SetPalette`

       - | firstcolor{color},
          | lastcolor{x0},data

       - | 璁剧疆璋冭壊鏉夸腑鐨勮嫢骞叉潯鐩€?          | 浠庢暟缁?"data" 涓缃?"firstcolor" 鍒?"lastcolor" 鐨勬潯鐩€?          | 姣忎釜棰滆壊鍗?4 瀛楄妭锛?          | R銆丟銆丅 涓庝竴涓笉閫忔槑搴﹀€硷細0->閫忔槑锛?..254->娣峰悎锛?55->鍍忕礌

    - ..

       - `OSD_SetTrans`

       - transparency{color}

       - | 璁剧疆娣峰悎鍍忕礌鐨勪笉閫忔槑搴︼紙0..15锛夈€?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_SetPixel`

       - x0,y0,color

       - | 灏嗗儚绱?<x>,<y> 璁句负棰滆壊缂栧彿 <color>銆?          | 鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1銆?
    - ..

       - `OSD_GetPixel`

       - x0,y0

       - | 杩斿洖鍍忕礌 <x>,<y> 鐨勯鑹茬紪鍙凤紝鎴?-1銆?          | 璇ュ懡浠ょ洰鍓?AV7110 灏氫笉鏀寔锛?
    - ..

       - `OSD_SetRow`

       - x0,y0,x1,data

       - | 鐢?data[] 鐨勫唴瀹瑰～鍏呭儚绱?x0,y 鍒?x1,y銆?          | 鎴愬姛鏃惰繑鍥?0锛屾墍鏈夊儚绱犺瑁佸壀鏃讹紙鏈粯鍒朵换浣曞儚绱狅級杩斿洖 -1銆?
    - ..

       - `OSD_SetBlock`

       - | x0,y0,x1,y1,
          | increment{color},
          | data

       - | 鐢?data[] 鐨勫唴瀹瑰～鍏呭儚绱?x0,y0 鍒?x1,y1銆?          | Inc 鍖呭惈鏁版嵁鍧椾腑涓€琛岀殑瀹藉害锛?          | inc<=0 鏃朵娇鐢ㄥ潡瀹藉害浣滀负琛屽銆?          | 鎴愬姛鏃惰繑鍥?0锛屾墍鏈夊儚绱犺瑁佸壀鏃惰繑鍥?-1銆?
    - ..

       - `OSD_FillRow`

       - x0,y0,x1,color

       - | 鐢ㄩ鑹?<color> 濉厖鍍忕礌 x0,y 鍒?x1,y銆?          | 鎴愬姛鏃惰繑鍥?0锛屾墍鏈夊儚绱犺瑁佸壀鏃惰繑鍥?-1銆?
    - ..

       - `OSD_FillBlock`

       - x0,y0,x1,y1,color

       - | 鐢ㄩ鑹?<color> 濉厖鍍忕礌 x0,y0 鍒?x1,y1銆?          | 鎴愬姛鏃惰繑鍥?0锛屾墍鏈夊儚绱犺瑁佸壀鏃惰繑鍥?-1銆?
    - ..

       - `OSD_Line`

       - x0,y0,x1,y1,color

       - | 鐢ㄩ鑹?<color> 浠?x0,y0 鍒?x1,y1 鐢讳竴鏉＄嚎銆?          | 鎴愬姛鏃惰繑鍥?0銆?
    - ..

       - `OSD_Query`

       - | x0,y0,x1,y1,
          | xasp{color}; yasp=11

       - | 鐢ㄥ浘鍍忓昂瀵镐笌鍍忕礌闀垮姣斿～鍏呭弬鏁般€?          | 鎴愬姛鏃惰繑鍥?0銆?          | 璇ュ懡浠ょ洰鍓?AV7110 灏氫笉鏀寔锛?
    - ..

       - `OSD_Test`

       - -

       - | 缁樺埗涓€寮犳祴璇曞浘銆?          | 浠呯敤浜庤皟璇曠洰鐨勩€?          | 鎴愬姛鏃惰繑鍥?0銆?    - ..

       - `OSD_Text`

       - x0,y0,size,color,text

       - 鍦ㄤ綅缃?x0,y0 鐢ㄩ鑹?<color> 缁樺埗涓€娈垫枃鏈€?
    - ..

       - `OSD_SetWindow`

       - x0

       - 灏嗙紪鍙蜂负 0<x0<8 鐨勭獥鍙ｈ涓哄綋鍓嶇獥鍙ｃ€?
    - ..

       - `OSD_MoveWindow`

       - x0,y0

       - 灏嗗綋鍓嶇獥鍙ｇЩ鍔ㄥ埌 (x0, y0)銆?
    - ..

       - `OSD_OpenRaw`

       - | x0,y0,x1,y1,
          | `osd_raw_window_t`_ {color}

       - 鎵撳紑鍏朵粬绫诲瀷鐨?OSD 绐楀彛銆?
#### Description锛堣鏄庯級


`OSD_Command` 鏁版嵁绫诲瀷涓?`OSD_SEND_CMD`_ ioctl 閰嶅悎浣跨敤锛岀敤浜庡憡鐭ラ┍鍔ㄨ鎵ц鍝釜 OSD_Command銆?

-----

### osd_cmd_t

#### Synopsis锛堟瑕侊級


    typedef struct osd_cmd_s {
	OSD_Command cmd;
	int x0;
	int y0;
	int x1;
	int y1;
	int color;
	void __user *data;
    } osd_cmd_t;

#### Variables锛堝彉閲忥級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_Command cmd`

       - 寰呮墽琛岀殑 `OSD_Command`_銆?
    - ..

       - `int x0`

       - 绗竴涓按骞充綅缃€?
    - ..

       - `int y0`

       - 绗竴涓瀭鐩翠綅缃€?
    - ..

       - `int x1`

       - 绗簩涓按骞充綅缃€?
    - ..

       - `int y1`

       - 绗簩涓瀭鐩翠綅缃€?
    - ..

       - `int color`

       - 璋冭壊鏉夸腑棰滆壊鐨勭紪鍙枫€?
    - ..

       - `void __user *data`

       - 鍛戒护鐩稿叧鐨勬暟鎹€?
#### Description锛堣鏄庯級


`osd_cmd_t` 鏁版嵁绫诲瀷涓?`OSD_SEND_CMD`_ ioctl 閰嶅悎浣跨敤銆傚畠鍖呭惈 OSD_Command 鐨勬暟鎹互鍙?`OSD_Command`_ 鏈韩銆傝缁撴瀯蹇呴』浼犵粰椹卞姩锛屽叾鍚勭粍鎴愰儴鍒嗗彲鑳戒細琚┍鍔ㄤ慨鏀广€?

-----

### osd_raw_window_t

#### Synopsis锛堟瑕侊級


    typedef enum {
	OSD_BITMAP1,
	OSD_BITMAP2,
	OSD_BITMAP4,
	OSD_BITMAP8,
	OSD_BITMAP1HR,
	OSD_BITMAP2HR,
	OSD_BITMAP4HR,
	OSD_BITMAP8HR,
	OSD_YCRCB422,
	OSD_YCRCB444,
	OSD_YCRCB444HR,
	OSD_VIDEOTSIZE,
	OSD_VIDEOHSIZE,
	OSD_VIDEOQSIZE,
	OSD_VIDEODSIZE,
	OSD_VIDEOTHSIZE,
	OSD_VIDEOTQSIZE,
	OSD_VIDEOTDSIZE,
	OSD_VIDEONSIZE,
	OSD_CURSOR
    } osd_raw_window_t;

#### Constants锛堝父閲忥級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_BITMAP1`

       - `1` 1 浣嶄綅鍥?
    - ..

       - `OSD_BITMAP2`

       - 2 浣嶄綅鍥?
    - ..

       - `OSD_BITMAP4`

       - 4 浣嶄綅鍥?
    - ..

       - `OSD_BITMAP8`

       - 8 浣嶄綅鍥?
    - ..

       - `OSD_BITMAP1HR`

       - 1 浣嶄綅鍥撅紝鍗婂垎杈ㄧ巼

    - ..

       - `OSD_BITMAP2HR`

       - 2 浣嶄綅鍥撅紝鍗婂垎杈ㄧ巼

    - ..

       - `OSD_BITMAP4HR`

       - 4 浣嶄綅鍥撅紝鍗婂垎杈ㄧ巼

    - ..

       - `OSD_BITMAP8HR`

       - 8 浣嶄綅鍥撅紝鍗婂垎杈ㄧ巼

    - ..

       - `OSD_YCRCB422`

       - 4:2:2 YCRCB 鍥惧舰鏄剧ず

    - ..

       - `OSD_YCRCB444`

       - 4:4:4 YCRCB 鍥惧舰鏄剧ず

    - ..

       - `OSD_YCRCB444HR`

       - 4:4:4 YCRCB 鍥惧舰锛屽崐鍒嗚鲸鐜?
    - ..

       - `OSD_VIDEOTSIZE`

       - 鐪熷疄灏哄 甯歌 MPEG 瑙嗛鏄剧ず

    - ..

       - `OSD_VIDEOHSIZE`

       - MPEG 瑙嗛鏄剧ず 鍗婂垎杈ㄧ巼

    - ..

       - `OSD_VIDEOQSIZE`

       - MPEG 瑙嗛鏄剧ず 鍥涘垎涔嬩竴鍒嗚鲸鐜?
    - ..

       - `OSD_VIDEODSIZE`

       - MPEG 瑙嗛鏄剧ず 鍙屽€嶅垎杈ㄧ巼

    - ..

       - `OSD_VIDEOTHSIZE`

       - 鐪熷疄灏哄 MPEG 瑙嗛鏄剧ず 鍗婂垎杈ㄧ巼

    - ..

       - `OSD_VIDEOTQSIZE`

       - 鐪熷疄灏哄 MPEG 瑙嗛鏄剧ず 鍥涘垎涔嬩竴鍒嗚鲸鐜?
    - ..

       - `OSD_VIDEOTDSIZE`

       - 鐪熷疄灏哄 MPEG 瑙嗛鏄剧ず 鍙屽€嶅垎杈ㄧ巼

    - ..

       - `OSD_VIDEONSIZE`

       - 鍏ㄥ昂瀵?MPEG 瑙嗛鏄剧ず

    - ..

       - `OSD_CURSOR`

       - 鍏夋爣

#### Description锛堣鏄庯級


`osd_raw_window_t` 鏁版嵁绫诲瀷涓?`OSD_Command`_ 鐨?OSD_OpenRaw 閰嶅悎浣跨敤锛岀敤浜庡憡鐭ラ┍鍔ㄨ鎵撳紑鍝绫诲瀷鐨?OSD銆?

-----

### osd_cap_t

#### Synopsis锛堟瑕侊級


    typedef struct osd_cap_s {
	int  cmd;
    #define OSD_CAP_MEMSIZE         1
	long val;
    } osd_cap_t;

#### Variables锛堝彉閲忥級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int  cmd`

       - 瑕佹煡璇㈢殑鑳藉姏銆?
    - ..

       - `long val`

       - 鐢ㄤ簬瀛樺偍鏁版嵁銆?
#### Supported capabilities锛堝彈鏀寔鐨勮兘鍔涳級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_CAP_MEMSIZE`

       - 鍗′笂瀹夎鐨勫唴瀛樺ぇ灏忋€?
#### Description锛堣鏄庯級


璇ユ暟鎹粨鏋勪笌 `OSD_GET_CAPABILITY`_ 璋冪敤閰嶅悎浣跨敤銆?

-----

## OSD Function Calls锛圤SD 鍑芥暟璋冪敤锛?
### OSD_SEND_CMD

#### Synopsis锛堟瑕侊級



    int ioctl(int fd, int request = OSD_SEND_CMD, enum osd_cmd_t *cmd)

#### Arguments锛堝弬鏁帮級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢变箣鍓嶅 `open()`_ 鐨勮皟鐢ㄦ墍杩斿洖鐨勬枃浠舵弿杩扮銆?
    - ..

       - `int request`

       - 鎸囧悜璇ュ懡浠ゆ墍鐢ㄧ殑 `osd_cmd_t`_ 缁撴瀯鎵€鍦ㄤ綅缃殑鎸囬拡銆?
#### Description锛堣鏄庯級


             See: legacy_dvb_decoder_notes

璇?ioctl 灏?`OSD_Command`_ 鍙戦€佺粰鍗°€?
#### Return Value锛堣繑鍥炲€硷級


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EINVAL`

       - 鍛戒护瓒呭嚭鑼冨洿銆?

-----

### OSD_GET_CAPABILITY

#### Synopsis锛堟瑕侊級



    int ioctl(int fd, int request = OSD_GET_CAPABILITY,
    struct osd_cap_t *cap)

#### Arguments锛堝弬鏁帮級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢变箣鍓嶅 `open()`_ 鐨勮皟鐢ㄦ墍杩斿洖鐨勬枃浠舵弿杩扮銆?
    - ..

       - `int request`

       - 璇ュ懡浠ょ瓑浜?`OSD_GET_CAPABILITY`銆?
    - ..

       - `unsigned int *cap`

       - 鎸囧悜璇ュ懡浠ゆ墍鐢ㄧ殑 `osd_cap_t`_ 缁撴瀯鎵€鍦ㄤ綅缃殑鎸囬拡銆?
#### Description锛堣鏄庯級


             See: legacy_dvb_decoder_notes

璇?ioctl 鐢ㄤ簬鑾峰彇姝ｅ湪浣跨敤鐨勩€佸熀浜?AV7110 鐨?DVB 瑙ｇ爜鍣ㄥ崱鐨?OSD 鐨勮兘鍔涖€?
    缁撴瀯 osd_cap_t 蹇呴』鐢辩敤鎴疯缃苟浼犵粰椹卞姩銆?
#### Return Value锛堣繑鍥炲€硷級


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
    :header-rows:  0
    :stub-columns: 0


    - ..

       - `EINVAL`

       - 涓嶆敮鎸佺殑鑳藉姏銆?

-----

### open()

#### Synopsis锛堟瑕侊級



    #include <fcntl.h>

#### Arguments锛堝弬鏁帮級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 鐗瑰畾 OSD 璁惧鐨勫悕绉般€?
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
#### Description锛堣鏄庯級


璇?system call 鎵撳紑涓€涓叿鍚嶇殑 OSD 璁惧锛堜緥濡?`/dev/dvb/adapter?/osd0`锛変互渚涘悗缁娇鐢ㄣ€?
#### Return Value锛堣繑鍥炲€硷級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - 璁惧椹卞姩鏈姞杞?涓嶅彲鐢ㄣ€?
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

#### Synopsis锛堟瑕侊級



#### Arguments锛堝弬鏁帮級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 鐢变箣鍓嶅 `open()`_ 鐨勮皟鐢ㄦ墍杩斿洖鐨勬枃浠舵弿杩扮銆?
#### Description锛堣鏄庯級


璇?system call 鍏抽棴涓€涓厛鍓嶆墦寮€鐨?OSD 璁惧銆?
#### Return Value锛堣繑鍥炲€硷級


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - fd 涓嶆槸涓€涓湁鏁堢殑宸叉墦寮€鏂囦欢鎻忚堪绗︺€?