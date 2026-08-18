


######## ioctl VIDIOC_QUERYCAP


## Name


VIDIOC_QUERYCAP - 鏌ヨ璁惧鑳藉姏


## Synopsis


`int ioctl(int fd, VIDIOC_QUERYCAP, struct v4l2_capability *argp)`


## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_capability` 鐨勬寚閽堛€?
## Description


鎵€鏈?V4L2 璁惧閮芥敮鎸?`VIDIOC_QUERYCAP` ioctl銆傚畠鐢ㄤ簬璇嗗埆涓庢湰瑙勮寖鍏煎鐨勫唴鏍?璁惧锛屽苟鑾峰彇鏈夊叧椹卞姩绋嬪簭鍜岀‖浠惰兘鍔涚殑淇℃伅銆傝 ioctl 鎺ュ彈涓€涓寚鍚?struct
`v4l2_capability` 鐨勬寚閽堬紝璇ョ粨鏋勭敱椹卞姩绋嬪簭濉厖銆傚綋椹卞姩绋嬪簭涓庢湰瑙勮寖涓嶅吋瀹规椂锛?璇?ioctl 杩斿洖 `EINVAL` 閿欒鐮併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u8
      - `driver`\ [^16^]
      - 椹卞姩绋嬪簭鐨勫悕绉帮紝涓€涓敮涓€鐨勩€佷互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€備緥濡傦細
	"bttv"銆傜壒瀹氫簬椹卞姩绋嬪簭鐨勫簲鐢ㄧ▼搴忓彲浠ヤ娇鐢ㄦ淇℃伅鏉ラ獙璇侀┍鍔ㄧ▼搴忕殑
	韬唤銆傚畠涔熸湁鍔╀簬瑙勯伩宸茬煡鐨勭己闄凤紝鎴栧湪閿欒鎶ュ憡涓瘑鍒┍鍔ㄧ▼搴忋€?
	鍦ㄥ浐瀹氬ぇ灏忕殑鏁扮粍涓瓨鍌ㄥ瓧绗︿覆鏄竴绉嶄笉濂界殑鍋氭硶锛屼絾鍦ㄨ繖閲屼笉鍙伩鍏嶃€?	椹卞姩绋嬪簭鍜屽簲鐢ㄧ▼搴忓簲褰撻噰鍙栨帾鏂斤紝缁濅笉鍘昏鍙栨垨鍐欏叆鏁扮粍鏈熬涔嬪锛屽苟
	纭繚瀛楃涓茶姝ｇ‘ NUL 缁撳熬銆?    - - __u8
      - `card`\ [^32^]
      - 璁惧鐨勫悕绉帮紝涓€涓互 NUL 缁撳熬鐨?UTF-8 瀛楃涓层€備緥濡傦細"Yoyodyne TV/FM"銆?	涓€涓┍鍔ㄧ▼搴忓彲鑳芥敮鎸佷笉鍚屽搧鐗屾垨鍨嬪彿鐨勭‖浠躲€傛淇℃伅闈㈠悜鐢ㄦ埛锛屼緥濡?	鏄剧ず鍦ㄥ彲鐢ㄨ澶囩殑鑿滃崟涓€傜敱浜庡彲鑳藉畨瑁呬簡澶氫釜鍚屼竴鍝佺墝鐨勭數瑙嗗崱锛屼笖
	瀹冧滑鐢卞悓涓€涓┍鍔ㄧ▼搴忔敮鎸侊紝姝ゅ悕绉板簲涓庡瓧绗﹁澶囨枃浠跺悕锛堜緥濡?	`/dev/video2`锛夋垨 `bus_info` 瀛楃涓茬粨鍚堜娇鐢紝浠ラ伩鍏嶆涔夈€?    - - __u8
      - `bus_info`\ [^32^]
      - 璁惧鍦ㄧ郴缁熶腑鐨勪綅缃紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€備緥濡傦細
	"PCI:0000:05:06.0"銆傛淇℃伅闈㈠悜鐢ㄦ埛锛岀敤浜庡尯鍒嗗涓浉鍚岀殑璁惧銆傚鏋?	娌℃湁姝ょ被淇℃伅鍙敤锛岃瀛楁蹇呴』绠€鍗曞湴瀵圭敱椹卞姩绋嬪簭鎺у埗鐨勮澶囪繘琛岃鏁?	锛?platform:vivid-000"锛夈€傚浜?PCI 鏉垮崱锛宐us_info 蹇呴』浠?"PCI:" 寮€澶达紱
	瀵逛簬 PCI Express 鏉垮崱浠?"PCIe:" 寮€澶达紱瀵逛簬 USB 璁惧浠?"usb-" 寮€澶达紱
	瀵逛簬 i2c 璁惧浠?"I2C:" 寮€澶达紱瀵逛簬 ISA 璁惧浠?"ISA:" 寮€澶达紱瀵逛簬骞跺彛
	璁惧浠?"parport" 寮€澶达紱瀵逛簬骞冲彴璁惧浠?"platform:" 寮€澶淬€?    - - __u32
      - `version`
      - 椹卞姩绋嬪簭鐨勭増鏈彿銆?
	浠庡唴鏍?3.1 寮€濮嬶紝鎶ュ憡鐨勭増鏈彿鐢?V4L2 瀛愮郴缁熸寜鐓у唴鏍哥紪鍙锋柟妗堟彁渚涖€?	浣嗘槸锛屽鏋滀緥濡備竴涓ǔ瀹氱増鎴栧彂琛岀増淇敼杩囩殑鍐呮牳浣跨敤浜嗘潵鑷洿鏂板唴鏍哥殑
	V4L2 鏍堬紝瀹冨彲鑳藉苟涓嶆€绘槸杩斿洖涓庡唴鏍哥浉鍚岀殑鐗堟湰銆?
	鐗堟湰鍙蜂娇鐢?`KERNEL_VERSION()` 瀹忔潵鏍煎紡鍖栥€備緥濡傦紝濡傛灉濯掍綋鏍堝搴斾簬
	闅忓唴鏍?4.14 涓€璧峰彂甯冪殑 V4L2 鐗堟湰锛屽垯瀹冪瓑浠蜂簬锛?    - - `2`

	`#define KERNEL_VERSION(a,b,c) (((a) << 16) + ((b) << 8) + (c))`

	`__u32 version = KERNEL_VERSION(4, 14, 0);`

	`printf ("Version: %u.%u.%u\\n",`

	`(version >> 16) & 0xFF, (version >> 8) & 0xFF, version & 0xFF);`
    - - __u32
      - `capabilities`
      - 鏁翠釜鐗╃悊璁惧鍙敤鐨勮兘鍔涳紝瑙?device-capabilities銆傚悓涓€涓墿鐞嗚澶囧彲浠ュ湪
	/dev 涓鍑哄涓澶囷紙渚嬪 /dev/videoX銆?dev/vbiY 鍜?/dev/radioZ锛夈€?	`capabilities` 瀛楁搴斿寘鍚鍑哄埌鐢ㄦ埛绌洪棿鐨勬墍鏈?V4L2 璁惧鍛ㄥ洿鍙敤鑳藉姏鐨?	骞堕泦銆傚浜庢墍鏈夎繖浜涜澶囷紝`capabilities` 瀛楁杩斿洖鐩稿悓鐨勪竴缁勮兘鍔涖€傝繖
	鍏佽搴旂敤绋嬪簭鍙墦寮€鍏朵腑涓€涓澶囷紙閫氬父鏄棰戣澶囷級锛屽苟鍙戠幇鏄惁涔熸敮鎸?	瑙嗛銆乿bi 鍜?鎴栧箍鎾€?    - - __u32
      - `device_caps`
      - 鎵€鎵撳紑璁惧鐨勮兘鍔涳紝瑙?device-capabilities銆傚簲褰撳寘鍚鐗瑰畾璁惧鑺傜偣鐨?	鍙敤鑳藉姏銆傚洜姝わ紝渚嬪锛屼竴涓箍鎾澶囩殑 `device_caps` 灏嗗彧鍖呭惈涓庡箍鎾?	鐩稿叧鐨勮兘鍔涳紝鑰屼笉鍖呭惈浠讳綍瑙嗛鎴?vbi 鑳藉姏銆備粎褰?`capabilities` 瀛楁鍖呭惈
	`V4L2_CAP_DEVICE_CAPS` 鑳藉姏鏃讹紝鎵嶄細璁剧疆姝ゅ瓧娈点€傚彧鏈?`capabilities`
	瀛楁鍙互鏈?`V4L2_CAP_DEVICE_CAPS` 鑳藉姏锛宍device_caps` 姘歌繙涓嶄細璁剧疆
	`V4L2_CAP_DEVICE_CAPS`銆?    - - __u32
      - `reserved`\ [^3^]
      - 涓哄皢鏉ョ殑鎵╁睍淇濈暀銆傞┍鍔ㄧ▼搴忓繀椤诲皢姝ゆ暟缁勭疆闆躲€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CAP_VIDEO_CAPTURE`
      - 0x00000001
      - 璇ヨ澶囬€氳繃 Video Capture <capture> 鎺ュ彛鏀寔鍗曞钩闈?API銆?    - - `V4L2_CAP_VIDEO_CAPTURE_MPLANE`
      - 0x00001000
      - 璇ヨ澶囬€氳繃 Video Capture <capture> 鎺ュ彛鏀寔澶氬钩闈?API <planar-apis>銆?    - - `V4L2_CAP_VIDEO_OUTPUT`
      - 0x00000002
      - 璇ヨ澶囬€氳繃 Video Output <output> 鎺ュ彛鏀寔鍗曞钩闈?API銆?    - - `V4L2_CAP_VIDEO_OUTPUT_MPLANE`
      - 0x00002000
      - 璇ヨ澶囬€氳繃 Video Output <output> 鎺ュ彛鏀寔澶氬钩闈?API <planar-apis>銆?    - - `V4L2_CAP_VIDEO_M2M`
      - 0x00008000
      - 璇ヨ澶囬€氳繃 Video Memory-To-Memory 鎺ュ彛鏀寔鍗曞钩闈?API銆?    - - `V4L2_CAP_VIDEO_M2M_MPLANE`
      - 0x00004000
      - 璇ヨ澶囬€氳繃 Video Memory-To-Memory 鎺ュ彛鏀寔澶氬钩闈?API <planar-apis>銆?    - - `V4L2_CAP_VIDEO_OVERLAY`
      - 0x00000004
      - 璇ヨ澶囨敮鎸?Video Overlay <overlay> 鎺ュ彛銆傝棰戝彔鍔犺澶囬€氬父灏嗘崟鑾风殑
	鍥惧儚鐩存帴瀛樺叆鏄惧崱鐨勮棰戝唴瀛樹腑锛屽苟甯︽湁纭欢瑁佸壀鍜岀缉鏀俱€?    - - `V4L2_CAP_VBI_CAPTURE`
      - 0x00000010
      - 璇ヨ澶囨敮鎸?Raw VBI Capture <raw-vbi> 鎺ュ彛锛屾彁渚涘浘鏂囩數瑙嗗拰闅愯棌瀛楀箷
	鏁版嵁銆?    - - `V4L2_CAP_VBI_OUTPUT`
      - 0x00000020
      - 璇ヨ澶囨敮鎸?Raw VBI Output <raw-vbi> 鎺ュ彛銆?    - - `V4L2_CAP_SLICED_VBI_CAPTURE`
      - 0x00000040
      - 璇ヨ澶囨敮鎸?Sliced VBI Capture <sliced> 鎺ュ彛銆?    - - `V4L2_CAP_SLICED_VBI_OUTPUT`
      - 0x00000080
      - 璇ヨ澶囨敮鎸?Sliced VBI Output <sliced> 鎺ュ彛銆?    - - `V4L2_CAP_RDS_CAPTURE`
      - 0x00000100
      - 璇ヨ澶囨敮鎸?RDS <rds> 鎹曡幏鎺ュ彛銆?    - - `V4L2_CAP_VIDEO_OUTPUT_OVERLAY`
      - 0x00000200
      - 璇ヨ澶囨敮鎸?Video Output Overlay <osd>锛圤SD锛夋帴鍙ｃ€備笌 **Video Overlay**
	鎺ュ彛涓嶅悓锛岃繖鏄棰戣緭鍑鸿澶囩殑娆¤鍔熻兘锛屽皢涓€骞呭浘鍍忓彔鍔犲埌浼犲嚭鐨勮棰?	淇″彿涓娿€傚綋椹卞姩绋嬪簭璁剧疆姝ゆ爣蹇楁椂锛屽畠蹇呴』娓呴櫎 `V4L2_CAP_VIDEO_OVERLAY`
	鏍囧織锛屽弽涔嬩害鐒躲€俒#f1]_
    - - `V4L2_CAP_HW_FREQ_SEEK`
      - 0x00000400
      - 璇ヨ澶囨敮鎸佺敤浜庣‖浠堕鐜囨悳绱㈢殑 VIDIOC_S_HW_FREQ_SEEK ioctl銆?    - - `V4L2_CAP_RDS_OUTPUT`
      - 0x00000800
      - 璇ヨ澶囨敮鎸?RDS <rds> 杈撳嚭鎺ュ彛銆?    - - `V4L2_CAP_TUNER`
      - 0x00010000
      - 璇ヨ澶囧甫鏈夋煇绉嶇敤浜庢帴鏀跺皠棰戣皟鍒惰棰戜俊鍙风殑璋冭皭鍣ㄣ€傛湁鍏宠皟璋愬櫒缂栫▼鐨?	鏇村淇℃伅锛岃 tuner銆?    - - `V4L2_CAP_AUDIO`
      - 0x00020000
      - 璇ヨ澶囧叿鏈夐煶棰戣緭鍏ユ垨杈撳嚭銆傚畠鍙兘鏀寔涔熷彲鑳戒笉鏀寔浠?PCM 鎴栧帇缂╂牸寮?	杩涜闊抽褰曞埗鎴栨挱鏀俱€侾CM 闊抽鏀寔蹇呴』瀹炵幇涓?ALSA 鎴?OSS 鎺ュ彛銆傛湁鍏?	闊抽杈撳叆鍜岃緭鍑虹殑鏇村淇℃伅锛岃 audio銆?    - - `V4L2_CAP_RADIO`
      - 0x00040000
      - 杩欐槸涓€涓箍鎾帴鏀跺櫒銆?    - - `V4L2_CAP_MODULATOR`
      - 0x00080000
      - 璇ヨ澶囧甫鏈夋煇绉嶇敤浜庡彂灏勫皠棰戣皟鍒惰棰?闊抽淇″彿鐨勮皟鍒跺櫒銆傛湁鍏宠皟鍒跺櫒
	缂栫▼鐨勬洿澶氫俊鎭紝瑙?tuner銆?    - - `V4L2_CAP_SDR_CAPTURE`
      - 0x00100000
      - 璇ヨ澶囨敮鎸?SDR Capture <sdr> 鎺ュ彛銆?    - - `V4L2_CAP_EXT_PIX_FORMAT`
      - 0x00200000
      - 璇ヨ澶囨敮鎸?struct `v4l2_pix_format` 鐨勬墿灞曞瓧娈点€?    - - `V4L2_CAP_SDR_OUTPUT`
      - 0x00400000
      - 璇ヨ澶囨敮鎸?SDR Output <sdr> 鎺ュ彛銆?    - - `V4L2_CAP_META_CAPTURE`
      - 0x00800000
      - 璇ヨ澶囨敮鎸佸厓鏁版嵁鎹曡幏鎺ュ彛銆?    - - `V4L2_CAP_READWRITE`
      - 0x01000000
      - 璇ヨ澶囨敮鎸?`read()` 鍜?鎴?`write()` I/O 鏂规硶銆?    - - `V4L2_CAP_EDID`
      - 0x02000000
      - 璇ヨ澶囦负瑙嗛杈撳叆瀛樺偍 EDID锛屾垨涓鸿棰戣緭鍑烘绱?EDID銆傚畠鏄竴涓嫭绔嬬殑
	EDID 璁惧锛屽洜姝や笉浼氬彂鐢熻棰戞祦浼犺緭绛夋搷浣溿€?
        瀵逛簬瑙嗛杈撳叆锛岃繖閫氬父鏄竴涓敮鎸?VESA 澧炲己鍨嬫樉绀烘暟鎹€氶亾鏍囧噯
	<vesaeddc> 鐨?eeprom銆傚畠涔熷彲浠ユ槸鍒殑涓滆タ锛屼緥濡備竴涓井鎺у埗鍣ㄣ€?
        瀵逛簬瑙嗛杈撳嚭锛岃繖閫氬父浠庡閮ㄨ澶囪鍙栵紝渚嬪閫氳繃涓插彛璁块棶鐨?HDMI 鍒嗛厤鍣ㄣ€?    - - `V4L2_CAP_STREAMING`
      - 0x04000000
      - 璇ヨ澶囨敮鎸?streaming <mmap> I/O 鏂规硶銆?    - - `V4L2_CAP_META_OUTPUT`
      - 0x08000000
      - 璇ヨ澶囨敮鎸佸厓鏁版嵁杈撳嚭鎺ュ彛銆?    - - `V4L2_CAP_TOUCH`
      - 0x10000000
      - 杩欐槸涓€涓Е鎽歌澶囥€?    - - `V4L2_CAP_IO_MC`
      - 0x20000000
      - 浠庣敤鎴风┖闂寸湅鍒扮殑鍙湁涓€涓緭鍏ュ拰/鎴栬緭鍑恒€傛暣涓棰戞嫇鎵戦厤缃紝鍖呮嫭鍝釜
	I/O 瀹炰綋琚矾鐢卞埌杈撳叆/杈撳嚭锛岀敱鐢ㄦ埛绌洪棿閫氳繃濯掍綋鎺у埗鍣ㄩ厤缃€傝
	media_controller銆?    - - `V4L2_CAP_DEVICE_CAPS`
      - 0x80000000
      - 椹卞姩绋嬪簭濉厖 `device_caps` 瀛楁銆傛鑳藉姏鍙兘鍑虹幇鍦?`capabilities`
	瀛楁涓紝鑰岀粷涓嶄細鍑虹幇鍦?`device_caps` 瀛楁涓€?
## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞惰缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error
Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
   struct `v4l2_framebuffer` 缂哄皯涓€涓?enum `v4l2_buf_type` 瀛楁锛屽洜姝ゅ彔鍔犵殑
   绫诲瀷鐢遍┍鍔ㄧ▼搴忚兘鍔涢殣鍚€?