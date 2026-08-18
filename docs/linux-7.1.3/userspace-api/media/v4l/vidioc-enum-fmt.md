

######## ioctl VIDIOC_ENUM_FMT


## Name


VIDIOC_ENUM_FMT - 鏋氫妇鍥惧儚鏍煎紡

## Synopsis



`int ioctl(int fd, VIDIOC_ENUM_FMT, struct v4l2_fmtdesc *argp)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜 struct `v4l2_fmtdesc` 鐨勬寚閽堛€?

## Description


涓轰簡鏋氫妇鍥惧儚鏍煎紡锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 struct `v4l2_fmtdesc` 鐨?`type`銆乣mbus_code` 鍜?`index`
瀛楁锛屽苟浣跨敤鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_ENUM_FMT ioctl銆傞┍鍔ㄥ～鍏呯粨鏋勭殑鍏朵綑閮ㄥ垎锛屾垨杩斿洖
`EINVAL` 閿欒鐮併€傛墍鏈夋牸寮忛兘鍙€氳繃浠?index 0 寮€濮嬫瘡娆″姞 1 鐩村埌杩斿洖 `EINVAL` 鏉ユ灇涓俱€傚鏋?
閫傜敤锛岄┍鍔ㄥ簲鎸変紭鍏堥『搴忚繑鍥炴牸寮忥紝鍏朵腑浼樺厛鏍煎紡鍦紙鍗充娇鐢ㄦ洿灏忕殑 `index` 鍊硷級闈炰紭鍏堟牸寮忎箣鍓?
杩斿洖銆?

鏍规嵁 `V4L2_CAP_IO_MC` 鑳藉姏 <device-capabilities>锛宍mbus_code` 瀛楁鐨勫鐞嗘柟寮忎笉鍚岋細

1) 鏈缃?`V4L2_CAP_IO_MC`锛堜篃绉颁负鈥渧ideo-node-centric鈥濋┍鍔級

   搴旂敤绋嬪簭搴斿皢 `mbus_code` 瀛楁鍒濆鍖栦负闆讹紝椹卞姩搴斿拷鐣ヨ瀛楁鐨勫€笺€?

   椹卞姩搴旀灇涓炬墍鏈夊浘鍍忔牸寮忋€?

```

      After switching the input or output the list of enumerated image
      formats may be different.

```
2) 璁剧疆浜?`V4L2_CAP_IO_MC`锛堜篃绉颁负鈥淢C-centric鈥濋┍鍔級

   濡傛灉 `mbus_code` 瀛楁涓洪浂锛屽垯搴旀灇涓炬墍鏈夊浘鍍忔牸寮忋€?

   濡傛灉 `mbus_code` 瀛楁琚垵濮嬪寲涓轰竴涓湁鏁堢殑锛堥潪闆讹級濯掍綋鎬荤嚎鏍煎紡鐮?
   <v4l2-mbus-pixelcode>锛屽垯椹卞姩搴斿皢鏋氫妇闄愬埗涓哄彧鑳界敓鎴愶紙瀵逛簬瑙嗛杈撳嚭璁惧锛夋垨鍙兘鐢憋紙瀵逛簬
   瑙嗛鎹曡幏璁惧锛夎濯掍綋鎬荤嚎鐮佷骇鐢?琚骇鐢熺殑鍥惧儚鏍煎紡銆傚鏋滈┍鍔ㄤ笉鏀寔璇?`mbus_code`锛屽垯搴?
   杩斿洖 `EINVAL`銆?

   鏃犺 `mbus_code` 瀛楁鐨勫€煎浣曪紝鏋氫妇鍑虹殑鍥惧儚鏍煎紡涓嶅簲渚濊禆浜庤棰戣澶囨垨璁惧娴佹按绾跨殑娲诲姩
   閰嶇疆銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏋氫妇涓牸寮忕殑缂栧彿锛岀敱搴旂敤绋嬪簭璁剧疆銆傝繖涓?`pixelformat` 瀛楁姣棤鍏崇郴銆傚綋 index 涓?
        `V4L2_FMTDESC_FLAG_ENUM_ALL` 杩涜 OR 杩愮畻鏃讹紝椹卞姩娓呴櫎璇ユ爣蹇楀苟鏋氫妇鎵€鏈夊彲鑳界殑鏍煎紡锛?
        蹇界暐褰撳墠閰嶇疆甯︽潵鐨勪换浣曢檺鍒躲€備笉鏀寔璇ユ爣蹇楃殑椹卞姩鎬绘槸杩斿洖 `EINVAL` 閿欒鐮侊紝涓斾笉娓呴櫎
        璇ユ爣蹇椼€備娇鐢?`V4L2_FMTDESC_FLAG_ENUM_ALL` 鏍囧織鏋氫妇鐨勬牸寮忎笉搴斿湪璋冪敤
        `VIDIOC_ENUM_FRAMESIZES` 鎴?`VIDIOC_ENUM_FRAMEINTERVALS` 鏃朵娇鐢ㄣ€俙V4L2_FMTDESC_FLAG_ENUM_ALL`
        鍙簲鐢辫兘鏍规嵁璇ユ爣蹇楄繑鍥炰笉鍚屾牸寮忓垪琛ㄧ殑椹卞姩浣跨敤銆?
    - - __u32
      - `type`
      - 鏁版嵁娴佺殑绫诲瀷锛岀敱搴旂敤绋嬪簭璁剧疆銆傛澶勫彧鏈変互涓嬬被鍨嬫槸鏈夋晥鐨勶細`V4L2_BUF_TYPE_VIDEO_CAPTURE`銆?
        `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`銆乣V4L2_BUF_TYPE_VIDEO_OUTPUT`銆?
        `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`銆乣V4L2_BUF_TYPE_VIDEO_OVERLAY`銆?
        `V4L2_BUF_TYPE_SDR_CAPTURE`銆乣V4L2_BUF_TYPE_SDR_OUTPUT`銆乣V4L2_BUF_TYPE_META_CAPTURE`
        鍜?`V4L2_BUF_TYPE_META_OUTPUT`銆傚弬瑙?`v4l2_buf_type`銆?
    - - __u32
      - `flags`
      - 鍙傝 fmtdesc-flags
    - - __u8
      - `description`\ [^32^]
      - 鏍煎紡鐨勬弿杩帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€傛淇℃伅渚涚敤鎴蜂娇鐢紝渚嬪锛氣€淵UV 4:2:2鈥濄€?
    - - __u32
      - `pixelformat`
      - 鍥惧儚鏍煎紡鏍囪瘑绗︺€傝繖鏄竴涓敱 v4l2_fourcc() 瀹忚绠楀嚭鐨勫洓瀛楃鐮侊細
    - - `2`

	.. _v4l2-fourcc:

	`#define v4l2_fourcc(a,b,c,d)`

	`(((__u32)(a)<<0)|((__u32)(b)<<8)|((__u32)(c)<<16)|((__u32)(d)<<24))`

	鏈鑼冨凡鍦?pixfmt 涓畾涔変簡鑻ュ共鍥惧儚鏍煎紡銆?

```

	   These codes are not the same as those used
	   in the Windows world.
    * - __u32
      - ``mbus_code``
      - Media bus code restricting the enumerated formats, set by the
        application. Only applicable to drivers that advertise the
        ``V4L2_CAP_IO_MC`` :ref:`capability <device-capabilities>`, shall be 0
        otherwise.
    * - __u32
      - ``reserved``\ [3]
      - Reserved for future extensions. Drivers must set the array to
	zero.


```



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FMT_FLAG_COMPRESSED`
      - 0x0001
      - 杩欐槸涓€涓帇缂╂牸寮忋€?
    - - `V4L2_FMT_FLAG_EMULATED`
      - 0x0002
      - 璇ユ牸寮忓苟闈炶澶囩殑鍘熺敓鏍煎紡锛岃€屾槸閫氳繃杞欢锛堥€氬父鏄?libv4l2锛夋ā鎷熺殑锛屽湪鍙兘鐨勬儏鍐典笅搴?
        灏介噺浣跨敤鍘熺敓鏍煎紡浠ヨ幏寰楁洿濂芥€ц兘銆?
    - - `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`
      - 0x0004
      - 璇ュ帇缂╁瓧鑺傛祦鏍煎紡锛堝張绉?coded 鏍煎紡锛夌殑纭欢瑙ｇ爜鍣ㄨ兘澶熻В鏋愯繛缁殑瀛楄妭娴併€傚簲鐢ㄧ▼搴忔棤闇€
        鑷瑙ｆ瀽瀛楄妭娴佹潵鏌ユ壘甯?鍦轰箣闂寸殑杈圭晫銆?

	璇ユ爣蹇楀彧鑳戒笌 `V4L2_FMT_FLAG_COMPRESSED` 鏍囧織缁勫悎浣跨敤锛屽洜涓哄畠浠呴€傜敤浜庡帇缂╂牸寮忋€傝
	鏍囧織浠呭 stateful 瑙ｇ爜鍣ㄦ湁鏁堛€?
    - - `V4L2_FMT_FLAG_DYN_RESOLUTION`
      - 0x0008
      - 璁惧鏀寔璇ュ帇缂╁瓧鑺傛祦鏍煎紡锛堝張绉?coded 鏍煎紡锛夌殑鍔ㄦ€佸垎杈ㄧ巼鍒囨崲銆傚綋妫€娴嬪埌瑙嗛鍙傛暟鍙樺寲鏃讹紝
        瀹冧細閫氳繃浜嬩欢 `V4L2_EVENT_SOURCE_CHANGE` 閫氱煡鐢ㄦ埛銆?

	璇ユ爣蹇楀彧鑳戒笌 `V4L2_FMT_FLAG_COMPRESSED` 鏍囧織缁勫悎浣跨敤锛屽洜涓哄畠浠呴€傜敤浜庡帇缂╂牸寮忋€傝
	鏍囧織浠呭 stateful 缂栬В鐮佸櫒鏈夋晥銆?
    - - `V4L2_FMT_FLAG_ENC_CAP_FRAME_INTERVAL`
      - 0x0010
      - 纭欢缂栫爜鍣ㄦ敮鎸佸皢 `CAPTURE` coded 甯ч棿闅斾笌 `OUTPUT` 鍘熷甯ч棿闅斿垎寮€璁剧疆銆備娇鐢?
        VIDIOC_S_PARM <VIDIOC_G_PARM> 璁剧疆 `OUTPUT` 鍘熷甯ч棿闅斾篃浼氬皢 `CAPTURE` coded 甯ч棿闅?
        璁句负鐩稿悓鐨勫€笺€傚鏋滆缃簡璇ユ爣蹇楋紝鍒欎箣鍚庡彲浠ュ皢 `CAPTURE` coded 甯ч棿闅旇涓轰笉鍚岀殑鍊笺€傝繖
        閫氬父鐢ㄤ簬绂荤嚎缂栫爜锛屽叾涓?`OUTPUT` 鍘熷甯ч棿闅旂敤浣滀繚鐣欑‖浠剁紪鐮佸櫒璧勬簮鐨勬彁绀猴紝鑰?
        `CAPTURE` coded 甯ч棿闅旀槸宓屽叆鍦ㄧ紪鐮佽棰戞祦涓殑瀹為檯甯х巼銆?

	璇ユ爣蹇楀彧鑳戒笌 `V4L2_FMT_FLAG_COMPRESSED` 鏍囧織缁勫悎浣跨敤锛屽洜涓哄畠浠呴€傜敤浜庡帇缂╂牸寮忋€傝
        鏍囧織浠呭 stateful 缂栫爜鍣ㄦ湁鏁堛€?
    - - `V4L2_FMT_FLAG_CSC_COLORSPACE`
      - 0x0020
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鑹插僵绌洪棿銆傝鏍囧織浠呬笌鎹曡幏璁惧鐩稿叧銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 骞惰缃?V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 鏃讹紝璇锋眰閰嶇疆鎹曡幏璁惧鐨勮壊褰╃┖闂淬€?
    - - `V4L2_FMT_FLAG_CSC_XFER_FUNC`
      - 0x0040
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿浼犻€掑嚱鏁帮紙transfer function锛夈€傝鏍囧織浠呬笌鎹曡幏璁惧鐩稿叧銆?
        搴旂敤绋嬪簭鍙互鍦ㄨ皟鐢?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 骞惰缃?
        V4L2_PIX_FMT_FLAG_SET_CSC <v4l2-pix-fmt-flag-set-csc> 鏃讹紝璇锋眰閰嶇疆鎹曡幏璁惧鐨勪紶閫掑嚱鏁般€?
    - - `V4L2_FMT_FLAG_CSC_YCBCR_ENC`
      - 0x0080
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨?Y'CbCr 缂栫爜銆傝鏍囧織浠呬笌鎹曡幏璁惧鐩稿叧銆傚簲鐢ㄧ▼搴忓彲浠ュ湪
        璋冪敤 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 骞惰缃?V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 鏃讹紝璇锋眰閰嶇疆鎹曡幏璁惧鐨?Y'CbCr 缂栫爜銆?
    - - `V4L2_FMT_FLAG_CSC_HSV_ENC`
      - 0x0080
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨?HSV 缂栫爜銆傝鏍囧織浠呬笌鎹曡幏璁惧鐩稿叧銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 骞惰缃?V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 鏃讹紝璇锋眰閰嶇疆鎹曡幏璁惧鐨?HSV 缂栫爜銆?
    - - `V4L2_FMT_FLAG_CSC_QUANTIZATION`
      - 0x0100
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨勯噺鍖栥€傝鏍囧織浠呬笌鎹曡幏璁惧鐩稿叧銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 骞惰缃?V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 鏃讹紝璇锋眰閰嶇疆鎹曡幏璁惧鐨勯噺鍖栥€?
    - - `V4L2_FMT_FLAG_META_LINE_BASED`
      - 0x0200
      - 鍏冩暟鎹牸寮忔槸鍩轰簬琛岀殑銆傚湪杩欑鎯呭喌涓?`v4l2_meta_format` 鐨?`width`銆乣height` 鍜?
        `bytesperline` 瀛楁鏄湁鏁堢殑銆傜紦鍐插尯鐢?`height` 琛岀粍鎴愶紝姣忚鏈?`width` 涓暟鎹崟鍏冿紝涓?
        姣忎袱涓繛缁涔嬮棿鐨勫亸绉婚噺锛堝瓧鑺傦級涓?`bytesperline`銆?
    - - `V4L2_FMTDESC_FLAG_ENUM_ALL`
      - 0x80000000
      - 褰撳簲鐢ㄧ▼搴忓皢 `index` 涓?`V4L2_FMTDESC_FLAG_ENUM_ALL` 鏍囧織杩涜 OR 杩愮畻鏃讹紝椹卞姩鏋氫妇
        鎵€鏈夊彲鑳界殑鍍忕礌鏍煎紡锛岃€屼笉鑰冭檻浠讳綍宸茶缃殑閰嶇疆銆備笉鏀寔璇ユ爣蹇楃殑椹卞姩鎬绘槸杩斿洖 `EINVAL`锛?
        涓斾笉娓呴櫎璇ユ爣蹇椼€?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞惰缃?`errno`銆傞€氱敤閿欒鐮佸湪 Generic Error Codes
<gen-errors> 涓€绔犱腑鎻忚堪銆?

EINVAL
    struct `v4l2_fmtdesc` 鐨?`type` 涓嶈鏀寔锛屾垨 `index` 瓒婄晫銆?

    濡傛灉璁剧疆浜?`V4L2_CAP_IO_MC` 涓旀寚瀹氱殑 `mbus_code` 涓嶈鏀寔锛屽垯涔熻繑鍥炴閿欒鐮併€?
