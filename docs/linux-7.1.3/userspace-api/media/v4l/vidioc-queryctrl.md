


######## ioctls VIDIOC_QUERYCTRL, VIDIOC_QUERY_EXT_CTRL and VIDIOC_QUERYMENU


## Name


VIDIOC_QUERYCTRL - VIDIOC_QUERY_EXT_CTRL - VIDIOC_QUERYMENU - 鏋氫妇鎺т欢鍜岃彍鍗曟帶浠堕」

## Synopsis


`int ioctl(int fd, int VIDIOC_QUERYCTRL, struct v4l2_queryctrl *argp)`


`int ioctl(int fd, VIDIOC_QUERY_EXT_CTRL, struct v4l2_query_ext_ctrl *argp)`


`int ioctl(int fd, VIDIOC_QUERYMENU, struct v4l2_querymenu *argp)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_queryctrl`銆乣v4l2_query_ext_ctrl`
    鎴?`v4l2_querymenu` 鐨勬寚閽堬紙鍙栧喅浜庡叿浣撶殑 ioctl锛夈€?
## Description


涓轰簡鏌ヨ涓€涓帶浠剁殑灞炴€э紝搴旂敤绋嬪簭璁剧疆 struct v4l2_queryctrl <v4l2-queryctrl> 鐨?`id` 瀛楁锛屽苟璋冪敤 `VIDIOC_QUERYCTRL` ioctl锛屼紶鍏ユ寚鍚戣缁撴瀯鐨勬寚閽堛€傞┍鍔ㄥ～鍏呯粨鏋勭殑
鍏朵綑閮ㄥ垎锛屾垨鑰呭綋 `id` 鏃犳晥鏃惰繑鍥?`EINVAL` 閿欒鐮併€?
鍙互閫氳繃浠?`V4L2_CID_BASE` 寮€濮嬨€佸埌锛堜笉鍚級`V4L2_CID_LASTP1` 涓烘锛屼互杩炵画鐨?`id` 鍊艰皟鐢?`VIDIOC_QUERYCTRL` 鏉ユ灇涓炬帶浠躲€傚鏋滄鑼冨洿鍐呯殑鏌愪釜鎺т欢涓嶅彈鏀寔锛岄┍鍔?鍙兘杩斿洖 `EINVAL`銆傝繘涓€姝ワ紝搴旂敤绋嬪簭鍙互閫氳繃浠?`V4L2_CID_PRIVATE_BASE` 寮€濮嬪苟閫掑
`id`锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`锛屾潵鏋氫妇鏈鑼冩湭瀹氫箟鐨勭鏈夋帶浠躲€?
鍦ㄨ繖涓ょ鎯呭喌涓嬶紝褰撻┍鍔ㄥ湪 `flags` 瀛楁涓缃簡 `V4L2_CTRL_FLAG_DISABLED` 鏍囧織鏃讹紝
璇ユ帶浠惰姘镐箙绂佺敤锛屽簲鐢ㄧ▼搴忓簲蹇界暐瀹冦€?[#f1]_

褰撳簲鐢ㄧ▼搴忓皢 `id` 涓?`V4L2_CTRL_FLAG_NEXT_CTRL` 鍋?OR 杩愮畻鏃讹紝椹卞姩杩斿洖涓嬩竴涓彈鏀寔鐨?闈炲鍚堟帶浠讹紝濡傛灉娌℃湁鍒欒繑鍥?`EINVAL`銆傛澶栵紝鍙互鎸囧畾 `V4L2_CTRL_FLAG_NEXT_COMPOUND`
鏍囧織鏉ユ灇涓炬墍鏈夌殑澶嶅悎鎺т欢锛堝嵆绫诲瀷 鈮?`V4L2_CTRL_COMPOUND_TYPES` 鍜?鎴栨暟缁勬帶浠讹紝鎹㈣█涔?鍖呭惈澶氫釜鍊肩殑鎺т欢锛夈€傚悓鏃舵寚瀹?`V4L2_CTRL_FLAG_NEXT_CTRL` 鍜?`V4L2_CTRL_FLAG_NEXT_COMPOUND` 浠ユ灇涓炬墍鏈夋帶浠讹紙鏃犺鏄惁澶嶅悎锛夈€傚皻涓嶆敮鎸佽繖浜涙爣蹇楃殑
椹卞姩鎬绘槸杩斿洖 `EINVAL`銆?
寮曞叆 `VIDIOC_QUERY_EXT_CTRL` ioctl 鏄负浜嗘洿濂藉湴鏀寔鍙互浣跨敤澶嶅悎绫诲瀷鐨勬帶浠讹紝骞舵毚闇?鏃犳硶鍦?struct v4l2_queryctrl <v4l2-queryctrl> 涓繑鍥烇紙鍥犱负璇ョ粨鏋勫凡婊★級鐨勯澶栨帶浠?淇℃伅銆?
`VIDIOC_QUERY_EXT_CTRL` 鐨勪娇鐢ㄦ柟寮忎笌 `VIDIOC_QUERYCTRL` 鐩稿悓锛屽彧鏄?`reserved`
鏁扮粍涔熷繀椤昏缃浂銆?
鑿滃崟鎺т欢闇€瑕侀澶栫殑淇℃伅锛氳彍鍗曢」鐨勫悕绉般€備负浜嗘煡璇㈠畠浠紝搴旂敤绋嬪簭璁剧疆 struct
v4l2_querymenu <v4l2-querymenu> 鐨?`id` 鍜?`index` 瀛楁锛屽苟璋冪敤 `VIDIOC_QUERYMENU`
ioctl锛屼紶鍏ユ寚鍚戣缁撴瀯鐨勬寚閽堛€傞┍鍔ㄥ～鍏呯粨鏋勭殑鍏朵綑閮ㄥ垎锛屾垨鑰呭綋 `id` 鎴?`index` 鏃犳晥鏃?杩斿洖 `EINVAL` 閿欒鐮併€傝彍鍗曢」閫氳繃浠ヤ粠 struct v4l2_queryctrl <v4l2-queryctrl> 鐨?`minimum` 鍒?`maximum`锛堝惈锛夌殑杩炵画 `index` 鍊艰皟鐢?`VIDIOC_QUERYMENU` 鏉ユ灇涓俱€?

   `VIDIOC_QUERYMENU` 鏈夊彲鑳藉 `minimum` 鍜?`maximum` 涔嬮棿鐨勬煇浜涚储寮曡繑鍥?   `EINVAL` 閿欒鐮併€傚湪杩欑鎯呭喌涓嬶紝璇ョ壒瀹氱殑鑿滃崟椤逛笉鍙楁椹卞姩鏀寔銆傚彟璇锋敞鎰忥紝
   `minimum` 鍊间笉涓€瀹氫负 0銆?
鍙﹁鍙傝 control 涓殑绀轰緥銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 鏍囪瘑鎺т欢锛岀敱搴旂敤绋嬪簭璁剧疆銆傞瀹氫箟 ID 鍙傝 control-id銆傚綋 ID 涓?	V4L2_CTRL_FLAG_NEXT_CTRL 鍋?OR 杩愮畻鏃讹紝椹卞姩娓呴櫎璇ユ爣蹇楀苟杩斿洖鍏锋湁鏇撮珮 ID 鐨?	绗竴涓帶浠躲€傚皻涓嶆敮鎸佹鏍囧織鐨勯┍鍔ㄦ€绘槸杩斿洖 `EINVAL` 閿欒鐮併€?    - - __u32
      - `type`
      - 鎺т欢绫诲瀷锛屽弬瑙?`v4l2_ctrl_type`銆?    - - __u8
      - `name`\ [^32^]
      - 鎺т欢鍚嶇О锛屼竴涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€傛淇℃伅渚涚敤鎴蜂娇鐢ㄣ€?    - - __s32
      - `minimum`
      - 鏈€灏忓€硷紝鍚€傝瀛楁缁欏嚭鎺т欢鐨勪竴涓笅鐣屻€傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢?	鏈€灏忓€硷紝鍙傝鏋氫妇 `v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓湁绗﹀彿鐨?32 浣嶅€笺€?    - - __s32
      - `maximum`
      - 鏈€澶у€硷紝鍚€傝瀛楁缁欏嚭鎺т欢鐨勪竴涓笂鐣屻€傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢?	鏈€澶у€硷紝鍙傝鏋氫妇 `v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓湁绗﹀彿鐨?32 浣嶅€笺€?    - - __s32
      - `step`
      - 璇ュ瓧娈电粰鍑烘帶浠剁殑姝ラ暱銆傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢ㄦ闀垮€硷紝鍙傝鏋氫妇
	`v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓棤绗﹀彿鐨?32 浣嶅€笺€?
	閫氬父椹卞姩涓嶅簲缂╂斁纭欢鎺у埗鍊笺€備緥濡傚綋 `name` 鎴?`id` 鏆楃ず浜嗘煇涓壒瀹氬崟浣嶏紝鑰?	纭欢瀹為檯涓婂彧鎺ュ彈璇ュ崟浣嶇殑鏁存暟鍊嶆椂锛屽彲鑳藉氨鏈夋蹇呰銆傚鏋滄槸杩欐牱锛岄┍鍔ㄥ繀椤绘敞鎰?	鍦ㄧ缉鏀炬椂姝ｇ‘鍦板鍊艰繘琛屽洓鑸嶄簲鍏ワ紝浠ヤ娇閿欒涓嶄細鍦ㄥ弽澶嶇殑璇?鍐欏惊鐜腑绱Н銆?
	璇ュ瓧娈电粰鍑哄疄闄呭奖鍝嶇‖浠剁殑鏁存暟鎺т欢鐨勬渶灏忓彉鍖栭噺銆傚綋鐢ㄦ埛鍙互閫氳繃閿洏鎴?GUI 鎸夐挳
	锛堣€岄潪婊戝潡锛夋敼鍙樻帶浠舵椂锛屽父甯搁渶瑕佹淇℃伅銆備緥濡傦紝褰撶‖浠跺瘎瀛樺櫒鎺ュ彈鍊?0-511锛岃€?	椹卞姩鎶ュ憡 0-65535 鏃讹紝step 搴斾负 128銆?
	娉ㄦ剰锛屽敖绠℃槸鏈夌鍙风殑锛屼絾 step 鍊煎簲褰撳缁堜负姝ｃ€?    - - __s32
      - `default_value`
      - `V4L2_CTRL_TYPE_INTEGER`銆乣_BOOLEAN`銆乣_BITMASK`銆乣_MENU` 鎴?	`_INTEGER_MENU` 鎺т欢鐨勯粯璁ゅ€笺€傚鍏朵粬绫诲瀷鐨勬帶浠舵棤鏁堛€?
```

	   Drivers reset controls to their default value only when
	   the driver is first loaded, never afterwards.
    * - __u32
      - ``flags``
      - Control flags, see :ref:`control-flags`.
    * - __u32
      - ``reserved``\ [2]
      - Reserved for future extensions. Drivers must set the array to
	zero.


```




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 鏍囪瘑鎺т欢锛岀敱搴旂敤绋嬪簭璁剧疆銆傞瀹氫箟 ID 鍙傝 control-id銆傚綋 ID 涓?	V4L2_CTRL_FLAG_NEXT_CTRL 鍋?OR 杩愮畻鏃讹紝椹卞姩娓呴櫎璇ユ爣蹇楀苟杩斿洖鍏锋湁鏇撮珮 ID 鐨?	绗竴涓潪澶嶅悎鎺т欢銆傚綋 ID 涓?`V4L2_CTRL_FLAG_NEXT_COMPOUND` 鍋?OR 杩愮畻鏃讹紝椹卞姩
	娓呴櫎璇ユ爣蹇楀苟杩斿洖鍏锋湁鏇撮珮 ID 鐨勭涓€涓鍚堟帶浠躲€傚悓鏃惰缃袱鑰呬互鑾峰彇鍏锋湁鏇撮珮 ID
	鐨勭涓€涓帶浠讹紙鏃犺鏄惁澶嶅悎锛夈€?    - - __u32
      - `type`
      - 鎺т欢绫诲瀷锛屽弬瑙?`v4l2_ctrl_type`銆?    - - char
      - `name`\ [^32^]
      - 鎺т欢鍚嶇О锛屼竴涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€傛淇℃伅渚涚敤鎴蜂娇鐢ㄣ€?    - - __s64
      - `minimum`
      - 鏈€灏忓€硷紝鍚€傝瀛楁缁欏嚭鎺т欢鐨勪竴涓笅鐣屻€傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢?	鏈€灏忓€硷紝鍙傝鏋氫妇 `v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓湁绗﹀彿鐨?64 浣嶅€笺€?    - - __s64
      - `maximum`
      - 鏈€澶у€硷紝鍚€傝瀛楁缁欏嚭鎺т欢鐨勪竴涓笂鐣屻€傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢?	鏈€澶у€硷紝鍙傝鏋氫妇 `v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓湁绗﹀彿鐨?64 浣嶅€笺€?    - - __u64
      - `step`
      - 璇ュ瓧娈电粰鍑烘帶浠剁殑姝ラ暱銆傚叧浜庢瘡绉嶅彲鑳界殑鎺т欢绫诲瀷搴斿浣曚娇鐢ㄦ闀垮€硷紝鍙傝鏋氫妇
	`v4l2_ctrl_type`銆傛敞鎰忚繖鏄竴涓棤绗﹀彿鐨?64 浣嶅€笺€?
	閫氬父椹卞姩涓嶅簲缂╂斁纭欢鎺у埗鍊笺€備緥濡傚綋 `name` 鎴?`id` 鏆楃ず浜嗘煇涓壒瀹氬崟浣嶏紝鑰?	纭欢瀹為檯涓婂彧鎺ュ彈璇ュ崟浣嶇殑鏁存暟鍊嶆椂锛屽彲鑳藉氨鏈夋蹇呰銆傚鏋滄槸杩欐牱锛岄┍鍔ㄥ繀椤绘敞鎰?	鍦ㄧ缉鏀炬椂姝ｇ‘鍦板鍊艰繘琛屽洓鑸嶄簲鍏ワ紝浠ヤ娇閿欒涓嶄細鍦ㄥ弽澶嶇殑璇?鍐欏惊鐜腑绱Н銆?
	璇ュ瓧娈电粰鍑哄疄闄呭奖鍝嶇‖浠剁殑鏁存暟鎺т欢鐨勬渶灏忓彉鍖栭噺銆傚綋鐢ㄦ埛鍙互閫氳繃閿洏鎴?GUI 鎸夐挳
	锛堣€岄潪婊戝潡锛夋敼鍙樻帶浠舵椂锛屽父甯搁渶瑕佹淇℃伅銆備緥濡傦紝褰撶‖浠跺瘎瀛樺櫒鎺ュ彈鍊?0-511锛岃€?	椹卞姩鎶ュ憡 0-65535 鏃讹紝step 搴斾负 128銆?    - - __s64
      - `default_value`
      - `V4L2_CTRL_TYPE_INTEGER`銆乣_INTEGER64`銆乣_BOOLEAN`銆乣_BITMASK`銆乣_MENU`銆?	`_INTEGER_MENU`銆乣_U8` 鎴?`_U16` 鎺т欢鐨勯粯璁ゅ€笺€傚鍏朵粬绫诲瀷鐨勬帶浠舵棤鏁堛€?
```

	   Drivers reset controls to their default value only when
	   the driver is first loaded, never afterwards.
    * - __u32
      - ``flags``
      - Control flags, see :ref:`control-flags`.
    * - __u32
      - ``elem_size``
      - The size in bytes of a single element of the array. Given a char
	pointer ``p`` to a 3-dimensional array you can find the position
	of cell ``(z, y, x)`` as follows:
	``p + ((z * dims[1] + y) * dims[0] + x) * elem_size``.
	``elem_size`` is always valid, also when the control isn't an
	array. For string controls ``elem_size`` is equal to
	``maximum + 1``.
    * - __u32
      - ``elems``
      - The number of elements in the N-dimensional array. If this control
	is not an array, then ``elems`` is 1. The ``elems`` field can
	never be 0.
    * - __u32
      - ``nr_of_dims``
      - The number of dimension in the N-dimensional array. If this
	control is not an array, then this field is 0.
    * - __u32
      - ``dims[V4L2_CTRL_MAX_DIMS]``
      - The size of each dimension. The first ``nr_of_dims`` elements of
	this array must be non-zero, all remaining elements must be zero.
    * - __u32
      - ``reserved``\ [32]
      - Reserved for future extensions. Applications and drivers must set
	the array to zero.


```




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 鏍囪瘑鎺т欢锛岀敱搴旂敤绋嬪簭鏍规嵁鐩稿簲鐨?struct v4l2_queryctrl <v4l2-queryctrl>
	`id` 璁剧疆銆?    - - __u32
      - `index`
      - 鑿滃崟椤圭殑绱㈠紩锛屼粠闆跺紑濮嬶紝鐢卞簲鐢ㄧ▼搴忚缃€?    - - union {
      - (anonymous)
    - - __u8
      - `name`\ [^32^]
      - 鑿滃崟椤瑰悕绉帮紝涓€涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€傛淇℃伅渚涚敤鎴蜂娇鐢ㄣ€傝瀛楁瀵?	`V4L2_CTRL_TYPE_MENU` 绫诲瀷鐨勬帶浠舵湁鏁堛€?    - - __s64
      - `value`
      - 鏁存暟鑿滃崟椤圭殑鍊笺€傝瀛楁瀵?`V4L2_CTRL_TYPE_INTEGER_MENU` 绫诲瀷鐨勬帶浠舵湁鏁堛€?    - - }
      -
    - - __u32
      - `reserved`
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ繀椤诲皢鏁扮粍缃浂銆?


   \footnotesize




    :header-rows:  1
    :stub-columns: 0
    :widths:       30 5 5 5 55

    - - Type
      - `minimum`
      - `step`
      - `maximum`
      - Description
    - - `V4L2_CTRL_TYPE_INTEGER`
      - any
      - any
      - any
      - 涓€涓彇鍊艰寖鍥翠粠 minimum 鍒?maximum锛堝惈锛夌殑鏁存暟鍊兼帶浠躲€俿tep 鍊艰〃绀哄彇鍊间箣闂寸殑
	澧為噺銆?    - - `V4L2_CTRL_TYPE_BOOLEAN`
      - 0
      - 1
      - 1
      - 涓€涓竷灏斿€兼帶浠躲€傞浂瀵瑰簲鈥渄isabled锛堢鐢級鈥濓紝涓€瀵瑰簲鈥渆nabled锛堝惎鐢級鈥濄€?    - - `V4L2_CTRL_TYPE_MENU`
      - 鈮?0
      - 1
      - N-1
      - 璇ユ帶浠舵湁涓€涓寘鍚?N 涓€夐」鐨勮彍鍗曘€傝彍鍗曢」鐨勫悕绉板彲浠ラ€氳繃 `VIDIOC_QUERYMENU`
	ioctl 鏋氫妇銆?    - - `V4L2_CTRL_TYPE_INTEGER_MENU`
      - 鈮?0
      - 1
      - N-1
      - 璇ユ帶浠舵湁涓€涓寘鍚?N 涓€夐」鐨勮彍鍗曘€傝彍鍗曢」鐨勫€煎彲浠ラ€氳繃 `VIDIOC_QUERYMENU`
	ioctl 鏋氫妇銆傝繖涓?`V4L2_CTRL_TYPE_MENU` 绫讳技锛屽彧鏄彍鍗曢」鏄甫绗﹀彿鐨?64 浣?	鏁存暟锛岃€岄潪瀛楃涓层€?    - - `V4L2_CTRL_TYPE_BITMASK`
      - 0
      - n/a
      - any
      - 涓€涓綅鎺╃爜瀛楁銆傛渶澶у€兼槸鍙互浣跨敤鐨勪竴缁勪綅锛屾墍鏈夊叾浠栦綅搴斾负 0銆傛渶澶у€艰瑙ｉ噴涓?	涓€涓?__u32锛屽厑璁镐娇鐢ㄤ綅鎺╃爜涓殑绗?31 浣嶃€?    - - `V4L2_CTRL_TYPE_BUTTON`
      - 0
      - 0
      - 0
      - 涓€涓湪璁剧疆鏃舵墽琛屾煇涓姩浣滅殑鎺т欢銆傞┍鍔ㄥ繀椤诲拷鐣ラ殢 `VIDIOC_S_CTRL` 浼犲叆鐨勫€硷紝
	骞跺湪 `VIDIOC_G_CTRL` 灏濊瘯鏃惰繑鍥?`EACCES` 閿欒鐮併€?    - - `V4L2_CTRL_TYPE_INTEGER64`
      - any
      - any
      - any
      - 涓€涓?64 浣嶆暣鏁板€兼帶浠躲€傛渶灏忓€笺€佹渶澶у€煎拰姝ラ暱鏃犳硶浣跨敤 `VIDIOC_QUERYCTRL`
	鏌ヨ銆傚彧鏈?`VIDIOC_QUERY_EXT_CTRL` 鍙互妫€绱?64 浣嶇殑鏈€灏忓€?鏈€澶у€?姝ラ暱鍊硷紝
	鍦ㄤ娇鐢?`VIDIOC_QUERYCTRL` 鏃跺簲灏嗗畠浠В閲婁负 n/a銆?    - - `V4L2_CTRL_TYPE_STRING`
      - 鈮?0
      - 鈮?1
      - 鈮?0
      - 瀛楃涓茬殑鏈€灏忓拰鏈€澶ч暱搴︺€傛闀挎剰鍛崇潃瀛楃涓茬殑闀垮害蹇呴』涓猴紙minimum + N * step锛?	涓瓧绗︼紝鍏朵腑 N 鈮?0銆傝繖浜涢暱搴︿笉鍖呭惈缁堟闆讹紝鍥犳涓轰簡灏嗛暱搴︿负 8 鐨勫瓧绗︿覆浼犵粰
	VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>锛屼綘闇€瑕佸皢 struct
	`v4l2_ext_control` 鐨?`size` 瀛楁璁剧疆涓?9銆傚浜?VIDIOC_G_EXT_CTRLS
	<VIDIOC_G_EXT_CTRLS>锛屼綘鍙互灏?`size` 瀛楁璁剧疆涓?`maximum` + 1銆備娇鐢ㄤ綍绉?	瀛楃缂栫爜鍙栧喅浜庡瓧绗︿覆鎺т欢鏈韩锛屽苟搴斾綔涓烘帶浠舵枃妗ｇ殑涓€閮ㄥ垎銆?    - - `V4L2_CTRL_TYPE_CTRL_CLASS`
      - n/a
      - n/a
      - n/a
      - 杩欎笉鏄竴涓帶浠躲€傚綋浠ョ瓑浜庢帶浠剁被浠ｇ爜锛堝弬瑙?ctrl-class锛夌殑鎺т欢 ID 鍑?1 璋冪敤
	`VIDIOC_QUERYCTRL` 鏃讹紝ioctl 杩斿洖璇ユ帶浠剁被鐨勫悕绉颁互鍙婃鎺т欢绫诲瀷銆備笉鏀寔姝?	鐗规€х殑杈冩棫椹卞姩杩斿洖 `EINVAL` 閿欒鐮併€?    - - `V4L2_CTRL_TYPE_U8`
      - any
      - any
      - any
      - 涓€涓彇鍊艰寖鍥翠粠 minimum 鍒?maximum锛堝惈锛夌殑鏃犵鍙?8 浣嶅€兼帶浠躲€俿tep 鍊艰〃绀?	鍙栧€间箣闂寸殑澧為噺銆?    - - `V4L2_CTRL_TYPE_U16`
      - any
      - any
      - any
      - 涓€涓彇鍊艰寖鍥翠粠 minimum 鍒?maximum锛堝惈锛夌殑鏃犵鍙?16 浣嶅€兼帶浠躲€俿tep 鍊艰〃绀?	鍙栧€间箣闂寸殑澧為噺銆?    - - `V4L2_CTRL_TYPE_U32`
      - any
      - any
      - any
      - 涓€涓彇鍊艰寖鍥翠粠 minimum 鍒?maximum锛堝惈锛夌殑鏃犵鍙?32 浣嶅€兼帶浠躲€俿tep 鍊艰〃绀?	鍙栧€间箣闂寸殑澧為噺銆?    - - `V4L2_CTRL_TYPE_MPEG2_QUANTISATION`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_mpeg2_quantisation`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	MPEG-2 閲忓寲鐭╅樀銆?    - - `V4L2_CTRL_TYPE_MPEG2_SEQUENCE`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_mpeg2_sequence`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?MPEG-2
	搴忓垪鍙傛暟銆?    - - `V4L2_CTRL_TYPE_MPEG2_PICTURE`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_mpeg2_picture`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?MPEG-2
	鍥惧儚鍙傛暟銆?    - - `V4L2_CTRL_TYPE_AREA`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_area`锛屽寘鍚煩褰㈠尯鍩熺殑瀹藉害鍜岄珮搴︺€傚崟浣嶅彇鍐充簬鍏蜂綋鐢ㄤ緥銆?    - - `V4L2_CTRL_TYPE_RECT`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_rect`锛屽寘鍚敱宸︿笂瑙掍綅缃€佸搴﹀拰楂樺害鎻忚堪鐨勭煩褰€傚崟浣?	鍙栧喅浜庡叿浣撶敤渚嬨€傚 `V4L2_CTRL_WHICH_MIN_VAL` 鍜?`V4L2_CTRL_WHICH_MAX_VAL`
	鐨勬敮鎸佹槸鍙€夌殑锛屽彇鍐充簬 `V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX` 鏍囧織銆傚叧浜庡浣?	瑙ｉ噴鏈€灏忓€煎拰鏈€澶у€硷紝璇峰弬瑙佸叿浣撴帶浠剁殑鏂囨。銆?    - - `V4L2_CTRL_TYPE_H264_SPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_h264_sps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?H264 搴忓垪
	鍙傛暟銆?    - - `V4L2_CTRL_TYPE_H264_PPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_h264_pps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?H264 鍥惧儚
	鍙傛暟銆?    - - `V4L2_CTRL_TYPE_H264_SCALING_MATRIX`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_h264_scaling_matrix`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	H264 缂╂斁鐭╅樀銆?    - - `V4L2_CTRL_TYPE_H264_SLICE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_h264_slice_params`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	H264 鍒囩墖鍙傛暟銆?    - - `V4L2_CTRL_TYPE_H264_DECODE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_h264_decode_params`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	H264 瑙ｇ爜鍙傛暟銆?    - - `V4L2_CTRL_TYPE_FWHT_PARAMS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_fwht_params`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?FWHT 鍙傛暟銆?    - - `V4L2_CTRL_TYPE_HEVC_SPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_sps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?HEVC 搴忓垪
	鍙傛暟闆嗐€?    - - `V4L2_CTRL_TYPE_HEVC_PPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_pps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?HEVC 鍥惧儚
	鍙傛暟闆嗐€?    - - `V4L2_CTRL_TYPE_HEVC_SLICE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_slice_params`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?HEVC
	鍒囩墖鍙傛暟銆?    - - `V4L2_CTRL_TYPE_HEVC_SCALING_MATRIX`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_scaling_matrix`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	HEVC 缂╂斁鐭╅樀銆?    - - `V4L2_CTRL_TYPE_VP8_FRAME`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_vp8_frame`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?VP8 甯у弬鏁般€?    - - `V4L2_CTRL_TYPE_HEVC_DECODE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_decode_params`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?HEVC
	瑙ｇ爜鍙傛暟銆?    - - `V4L2_CTRL_TYPE_HEVC_EXT_SPS_LT_RPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_ext_sps_lt_rps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	HEVC 鎵╁睍闀挎湡 RPS銆?    - - `V4L2_CTRL_TYPE_HEVC_EXT_SPS_ST_RPS`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_hevc_ext_sps_st_rps`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	HEVC 鎵╁睍鐭湡 RPS銆?    - - `V4L2_CTRL_TYPE_VP9_COMPRESSED_HDR`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_vp9_compressed_hdr`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?VP9
	姒傜巼鏇存柊銆?    - - `V4L2_CTRL_TYPE_VP9_FRAME`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_vp9_frame`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?VP9 甯цВ鐮?	鍙傛暟銆?    - - `V4L2_CTRL_TYPE_AV1_SEQUENCE`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_av1_sequence`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?AV1 Sequence
	OBU 瑙ｇ爜鍙傛暟銆?    - - `V4L2_CTRL_TYPE_AV1_TILE_GROUP_ENTRY`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_av1_tile_group_entry`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?	AV1 Tile Group OBU 瑙ｇ爜鍙傛暟銆?    - - `V4L2_CTRL_TYPE_AV1_FRAME`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_av1_frame`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?AV1 Frame/Frame
	Header OBU 瑙ｇ爜鍙傛暟銆?    - - `V4L2_CTRL_TYPE_AV1_FILM_GRAIN`
      - n/a
      - n/a
      - n/a
      - 涓€涓?struct `v4l2_ctrl_av1_film_grain`锛屽寘鍚敤浜庢棤鐘舵€佽棰戣В鐮佸櫒鐨?AV1 鑳剁墖
	棰楃矑鍙傛暟銆?

   \normalsize




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CTRL_FLAG_DISABLED`
      - 0x0001
      - 璇ユ帶浠惰姘镐箙绂佺敤锛屽簲鐢ㄧ▼搴忓簲蹇界暐瀹冦€備换浣曞皾璇曟敼鍙樿鎺т欢鐨勬搷浣滈兘灏嗗鑷?	`EINVAL` 閿欒鐮併€?    - - `V4L2_CTRL_FLAG_GRABBED`
      - 0x0002
      - 璇ユ帶浠舵殏鏃朵笉鍙洿鏀癸紝渚嬪鍥犱负鍙︿竴涓簲鐢ㄧ▼搴忔帴绠′簡瀵圭浉搴旇祫婧愮殑鎺у埗銆傛绫绘帶浠?	鍦ㄧ敤鎴风晫闈腑鍙兘浼氫互鐗规畩鏂瑰紡鏄剧ず銆傚皾璇曟敼鍙樿鎺т欢鍙兘瀵艰嚧 `EBUSY` 閿欒鐮併€?    - - `V4L2_CTRL_FLAG_READ_ONLY`
      - 0x0004
      - 璇ユ帶浠舵槸姘镐箙鍙鐨勩€備换浣曞皾璇曟敼鍙樿鎺т欢鐨勬搷浣滈兘灏嗗鑷?`EINVAL` 閿欒鐮併€?    - - `V4L2_CTRL_FLAG_UPDATE`
      - 0x0008
      - 涓€涓彁绀猴紝琛ㄦ槑鏀瑰彉姝ゆ帶浠跺彲鑳戒細褰卞搷鍚屼竴鎺т欢绫讳腑鍏朵粬鎺т欢鐨勫€笺€傚簲鐢ㄧ▼搴忓簲鐩稿簲鍦?	鏇存柊鍏剁敤鎴风晫闈€?    - - `V4L2_CTRL_FLAG_INACTIVE`
      - 0x0010
      - 璇ユ帶浠朵笉閫傜敤浜庡綋鍓嶉厤缃紝鍦ㄧ敤鎴风晫闈腑搴旂浉搴斿湴鏄剧ず銆備緥濡傦紝褰撲娇鐢ㄥ彟涓€涓帶浠堕€夋嫨浜?	MPEG 闊抽缂栫爜绾у埆 1 鏃讹紝鍙兘浼氬湪 MPEG 闊抽绾у埆 2 鐮佺巼鎺т欢涓婅缃鏍囧織銆?    - - `V4L2_CTRL_FLAG_SLIDER`
      - 0x0020
      - 涓€涓彁绀猴紝琛ㄦ槑璇ユ帶浠跺湪鐢ㄦ埛鐣岄潰涓渶閫傚悎琛ㄧず涓烘粦鍧楀紡鐨勫厓绱犮€?    - - `V4L2_CTRL_FLAG_WRITE_ONLY`
      - 0x0040
      - 璇ユ帶浠舵槸姘镐箙鍙啓鐨勩€備换浣曞皾璇曡鍙栬鎺т欢鐨勬搷浣滈兘灏嗗鑷?`EACCES` 閿欒鐮併€傛鏍囧織
	閫氬父鍑虹幇鍦ㄧ浉瀵规帶浠舵垨鍔ㄤ綔鎺т欢涓婏紝鍏朵腑鍐欏叆涓€涓€煎皢瀵艰嚧璁惧鎵ц缁欏畾鍔ㄤ綔锛堜緥濡?	鐢垫満鎺у埗锛夛紝浣嗘棤娉曡繑鍥炴湁鎰忎箟鐨勫€笺€?    - - `V4L2_CTRL_FLAG_VOLATILE`
      - 0x0080
      - 璇ユ帶浠舵槸鏄撳彉鐨勶紙volatile锛夛紝杩欐剰鍛崇潃鎺т欢鐨勫€间細鎸佺画鍙樺寲銆備竴涓吀鍨嬬殑渚嬪瓙鏄綋璁惧
	澶勪簬鑷姩澧炵泭妯″紡鏃剁殑褰撳墠澧炵泭鍊笺€傚湪杩欑鎯呭喌涓嬶紝纭欢鏍规嵁鍙兘闅忔椂闂村彉鍖栫殑鐓ф槑
	鏉′欢璁＄畻澧炵泭鍊笺€?
```

	   Setting a new value for a volatile control will be ignored
	   unless
	   :ref:`V4L2_CTRL_FLAG_EXECUTE_ON_WRITE <FLAG_EXECUTE_ON_WRITE>`
	   is also set.
	   Setting a new value for a volatile control will *never* trigger a
	   :ref:`V4L2_EVENT_CTRL_CH_VALUE <ctrl-changes-flags>` event.
    * - ``V4L2_CTRL_FLAG_HAS_PAYLOAD``
      - 0x0100
      - This control has a pointer type, so its value has to be accessed
	using one of the pointer fields of struct
	:c:type:`v4l2_ext_control`. This flag is set
	for controls that are an array, string, or have a compound type.
	In all cases you have to set a pointer to memory containing the
	payload of the control.
    * .. _FLAG_EXECUTE_ON_WRITE:

      - ``V4L2_CTRL_FLAG_EXECUTE_ON_WRITE``
      - 0x0200
      - The value provided to the control will be propagated to the driver
	even if it remains constant. This is required when the control
	represents an action on the hardware. For example: clearing an
	error flag or triggering the flash. All the controls of the type
	``V4L2_CTRL_TYPE_BUTTON`` have this flag set.
    * .. _FLAG_MODIFY_LAYOUT:

      - ``V4L2_CTRL_FLAG_MODIFY_LAYOUT``
      - 0x0400
      - Changing this control value may modify the layout of the
        buffer (for video devices) or the media bus format (for sub-devices).

	A typical example would be the ``V4L2_CID_ROTATE`` control.

	Note that typically controls with this flag will also set the
	``V4L2_CTRL_FLAG_GRABBED`` flag when buffers are allocated or
	streaming is in progress since most drivers do not support changing
	the format in that case.
    * - ``V4L2_CTRL_FLAG_DYNAMIC_ARRAY``
      - 0x0800
      - This control is a dynamically sized 1-dimensional array. It
        behaves the same as a regular array, except that the number
	of elements as reported by the ``elems`` field is between 1 and
	``dims[0]``. So setting the control with a differently sized
	array will change the ``elems`` field when the control is
	queried afterwards.
    * - ``V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX``
      - 0x1000
      - This control supports getting minimum and maximum values using
        vidioc_g_ext_ctrls with V4L2_CTRL_WHICH_MIN/MAX_VAL.


```
## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    struct v4l2_queryctrl <v4l2-queryctrl> 鐨?`id` 鏃犳晥銆俿truct
    v4l2_querymenu <v4l2-querymenu> 鐨?`id` 鏃犳晥锛屾垨 `index` 瓒呭嚭鑼冨洿锛堝皬浜?    `minimum` 鎴栧ぇ浜?`maximum`锛夛紝鎴栬€呰鐗瑰畾鐨勮彍鍗曢」涓嶅彈椹卞姩鏀寔銆?
EACCES
    灏濊瘯璇诲彇涓€涓彧鍐欐帶浠躲€?
   `V4L2_CTRL_FLAG_DISABLED` 鏈変袱涓敤閫旓細椹卞姩鍙互璺宠繃纭欢涓嶆敮鎸佺殑棰勫畾涔夋帶浠?   锛堝敖绠¤繑鍥?`EINVAL` 涔熷悓鏍峰彲浠ワ級锛屾垨鑰呭湪纭欢妫€娴嬪悗绂佺敤棰勫畾涔夊拰绉佹湁鎺т欢锛岃€屾棤闇€
   閲嶆柊鎺掑簭鎺т欢鏁扮粍鍜岀储寮曠殑楹荤儲锛坄EINVAL` 涓嶈兘鐢ㄤ簬璺宠繃绉佹湁鎺т欢锛屽洜涓洪偅浼氳繃鏃╁湴
   缁撴潫鏋氫妇锛夈€?