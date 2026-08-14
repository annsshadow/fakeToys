


######## ioctl VIDIOC_SUBSCRIBE_EVENT, VIDIOC_UNSUBSCRIBE_EVENT


## 鍚嶇О


VIDIOC_SUBSCRIBE_EVENT - VIDIOC_UNSUBSCRIBE_EVENT - 璁㈤槄鎴栧彇娑堣闃呬簨浠?
## 姒傝



`int ioctl(int fd, VIDIOC_SUBSCRIBE_EVENT, struct v4l2_event_subscription *argp)`


`int ioctl(int fd, VIDIOC_UNSUBSCRIBE_EVENT, struct v4l2_event_subscription *argp)`

## 鍙傛暟



`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_event_subscription` 鐨勬寚閽堛€?
## 鎻忚堪


璁㈤槄鎴栧彇娑堣闃?V4L2 浜嬩欢銆傚凡璁㈤槄鐨勪簨浠堕€氳繃 VIDIOC_DQEVENT ioctl 鍑洪槦銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 浜嬩欢鐨勭被鍨嬶紝鍙傝 event-type銆?```
	   ``V4L2_EVENT_ALL`` 鍙笌
	   :ref:`VIDIOC_UNSUBSCRIBE_EVENT <VIDIOC_SUBSCRIBE_EVENT>` 涓€璧蜂娇鐢紝
	   鐢ㄤ簬涓€娆℃€у彇娑堣闃呮墍鏈変簨浠躲€?    * - __u32
      - ``id``
      - 浜嬩欢婧愮殑 ID銆傚鏋滀簨浠舵簮娌℃湁鍏宠仈鐨?ID锛屽垯灏嗗叾璁句负 0銆備簨浠舵槸鍚﹂渶瑕?ID
	鍙栧喅浜庝簨浠剁被鍨嬨€?    * - __u32
      - ``flags``
      - 浜嬩欢鏍囧織锛屽弬瑙?:ref:`event-flags`銆?    * - __u32
      - ``reserved``\ [5]
      - 淇濈暀浠ュ灏嗘潵鎵╁睍銆傞┍鍔ㄥ拰搴旂敤绋嬪簭閮藉繀椤诲皢璇ユ暟缁勭疆闆躲€?

```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_SUB_FL_SEND_INITIAL`
      - 0x0001
      - 褰撹闃呮浜嬩欢鏃讹紝浼氬彂閫佷竴涓寘鍚綋鍓嶇姸鎬佺殑鍒濆浜嬩欢銆傝繖浠呭鐢辩姸鎬佸彉鍖栬Е鍙戠殑浜嬩欢
	锛堝 `V4L2_EVENT_CTRL`锛夋湁鎰忎箟銆傚叾浠栦簨浠朵細蹇界暐姝ゆ爣蹇椼€?    - - `V4L2_EVENT_SUB_FL_ALLOW_FEEDBACK`
      - 0x0002
      - 鑻ヨ缃紝鍒欑洿鎺ョ敱 ioctl 寮曡捣鐨勪簨浠朵篃浼氬彂閫佺粰璋冪敤璇?ioctl 鐨勬枃浠跺彞鏌勩€備緥濡傦紝
	浣跨敤 VIDIOC_S_CTRL <VIDIOC_G_CTRL> 鏀瑰彉涓€涓帶浠朵細瀵艰嚧涓€涓?V4L2_EVENT_CTRL 琚?	鍙戦€佸洖鍚屼竴涓枃浠跺彞鏌勩€?	閫氬父姝ょ被浜嬩欢浼氳鎶戝埗锛屼互闃叉鍙嶉鐜矾锛氫竴涓簲鐢ㄧ▼搴忓皢鏌愪釜鎺т欢鏀逛负涓€涓€硷紝
	鐒跺悗鍙堟敼涓哄彟涓€涓€硷紝鎺ョ潃鏀跺埌涓€涓簨浠跺憡璇夊畠璇ユ帶浠跺凡鍙樺洖绗竴涓€笺€?
	鐢变簬瀹冩棤娉曞垽鏂浜嬩欢鏄敱鍙︿竴涓簲鐢ㄧ▼搴忓紩璧风殑锛岃繕鏄敱 VIDIOC_S_CTRL <VIDIOC_G_CTRL>
	璋冪敤寮曡捣鐨勶紝鍥犳寰堥毦鍐冲畾鏄皢鎺т欢璁句负浜嬩欢涓殑鍊硷紝杩樻槸蹇界暐瀹冦€?
	璁剧疆姝ゆ爣蹇楁椂璇蜂粩缁嗚€冭檻锛屼互鍏嶉櫡鍏ユ绫绘儏褰€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?