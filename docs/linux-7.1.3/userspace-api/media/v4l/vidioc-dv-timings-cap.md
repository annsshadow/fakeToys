
######## ioctl VIDIOC_DV_TIMINGS_CAP, VIDIOC_SUBDEV_DV_TIMINGS_CAP


## 鍚嶇О


VIDIOC_DV_TIMINGS_CAP - VIDIOC_SUBDEV_DV_TIMINGS_CAP - 鏁板瓧瑙嗛鎺ユ敹/鍙戦€佸櫒鐨勮兘鍔?
## 璇硶


`int ioctl(int fd, VIDIOC_DV_TIMINGS_CAP, struct v4l2_dv_timings_cap *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_DV_TIMINGS_CAP, struct v4l2_dv_timings_cap *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_dv_timings_cap` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇?DV 鎺ユ敹/鍙戦€佸櫒鐨勮兘鍔涳紝搴旂敤绋嬪簭灏?struct `v4l2_dv_timings_cap` 鐨?`pad`
瀛楁鍒濆鍖栦负 0锛屽皢 reserved 鏁扮粍娓呴浂锛屽苟鍦ㄨ棰戣妭鐐逛笂璋冪敤 `VIDIOC_DV_TIMINGS_CAP`
ioctl锛岄┍鍔ㄩ殢鍚庝細濉厖璇ョ粨鏋勩€?

   椹卞姩鍦ㄥ垏鎹㈣棰戣緭鍏ユ垨杈撳嚭鍚庯紝鍙兘杩斿洖涓嶅悓鐨勫€笺€?
褰撶敱椹卞姩瀹炵幇鏃讹紝瀛愯澶囩殑 DV 鑳藉姏鍙€氳繃鍦ㄥ瓙璁惧鑺傜偣涓婄洿鎺ヨ皟鐢?`VIDIOC_SUBDEV_DV_TIMINGS_CAP` ioctl 鏉ユ煡璇€傝繖浜涜兘鍔涚壒瀹氫簬杈撳叆锛堝浜?DV 鎺ユ敹鍣級
鎴栬緭鍑猴紙瀵逛簬 DV 鍙戦€佸櫒锛夛紝搴旂敤绋嬪簭蹇呴』鍦?struct `v4l2_dv_timings_cap` 鐨?`pad`
瀛楁涓寚瀹氭墍闇€鐨?pad 缂栧彿锛屽苟灏?`reserved` 鏁扮粍娓呴浂銆傚皾璇曟煡璇笉鏀寔璇ヨ兘鍔涚殑 pad
灏嗚繑鍥?`EINVAL` 閿欒鐮併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `min_width`
      - 娲诲姩瑙嗛鐨勬渶灏忓搴︼紙鍍忕礌锛夈€?    - - __u32
      - `max_width`
      - 娲诲姩瑙嗛鐨勬渶澶у搴︼紙鍍忕礌锛夈€?    - - __u32
      - `min_height`
      - 娲诲姩瑙嗛鐨勬渶灏忛珮搴︼紙琛屾暟锛夈€?    - - __u32
      - `max_height`
      - 娲诲姩瑙嗛鐨勬渶澶ч珮搴︼紙琛屾暟锛夈€?    - - __u64
      - `min_pixelclock`
      - 鏈€灏忓儚绱犳椂閽熼鐜囷紙Hz锛夈€?    - - __u64
      - `max_pixelclock`
      - 鏈€澶у儚绱犳椂閽熼鐜囷紙Hz锛夈€?    - - __u32
      - `standards`
      - 纭欢鏀寔鐨勮棰戞爣鍑嗐€傛爣鍑嗗垪琛ㄥ弬瑙?dv-bt-standards銆?    - - __u32
      - `capabilities`
      - 鎻愪緵鍏充簬杩欎簺鑳藉姏鐨勬洿澶氫俊鎭殑涓€浜涙爣蹇椼€傛爣蹇楄鏄庡弬瑙?dv-bt-cap-capabilities銆?    - - __u32
      - `reserved`\ [^16^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ繀椤诲皢鏁扮粍缃浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - DV 鏃跺簭绫诲瀷锛屽垪鍑轰簬 dv-timing-types銆?    - - __u32
      - `pad`
      - 鐢卞獟浣撴帶鍒跺櫒 API 鎶ュ憡鐨?pad 缂栧彿銆傝瀛楁浠呭湪瀵瑰瓙璁惧鑺傜偣鎿嶄綔鏃朵娇鐢ㄣ€?	鍦ㄥ瑙嗛鑺傜偣鎿嶄綔鏃讹紝搴旂敤绋嬪簭蹇呴』灏嗚瀛楁缃负闆躲€?    - - __u32
      - `reserved`\ [^2^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€?
	椹卞姩涓庡簲鐢ㄧ▼搴忛兘蹇呴』灏嗘暟缁勭疆闆躲€?    - - union {
      - (anonymous)
    - - struct `v4l2_bt_timings_cap`
      - `bt`
      - 纭欢鐨?BT.656/1120 鏃跺簭鑳藉姏銆?    - - __u32
      - `raw_data`\ [^32^]
    - - }
      -



    :header-rows:  0
    :stub-columns: 0

    - - 鏍囧織
      - 鎻忚堪
#     * -

    - - `V4L2_DV_BT_CAP_INTERLACED`
      - 鏀寔闅旇锛坕nterlaced锛夋牸寮忋€?    - - `V4L2_DV_BT_CAP_PROGRESSIVE`
      - 鏀寔閫愯锛坧rogressive锛夋牸寮忋€?    - - `V4L2_DV_BT_CAP_REDUCED_BLANKING`
      - CVT/GTF 涓撶敤锛氭椂搴忓彲鍒╃敤缂╁噺娑堥殣锛圕VT锛夋垨鈥淪econdary GTF鈥濇洸绾匡紙GTF锛夈€?    - - `V4L2_DV_BT_CAP_CUSTOM`
      - 鏀寔闈炴爣鍑嗘椂搴忥紝鍗充笉灞炰簬 `standards` 瀛楁鎵€璁炬爣鍑嗙殑鏃跺簭銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?