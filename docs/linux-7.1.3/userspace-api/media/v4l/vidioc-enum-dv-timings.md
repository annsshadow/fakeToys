


######## ioctl VIDIOC_ENUM_DV_TIMINGS, VIDIOC_SUBDEV_ENUM_DV_TIMINGS


## 鍚嶇О


VIDIOC_ENUM_DV_TIMINGS - VIDIOC_SUBDEV_ENUM_DV_TIMINGS - 鏋氫妇鍙楁敮鎸佺殑鏁板瓧瑙嗛锛圖igital Video锛夋椂搴?
## 姒傝



`int ioctl(int fd, VIDIOC_ENUM_DV_TIMINGS, struct v4l2_enum_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_DV_TIMINGS, struct v4l2_enum_dv_timings *argp)`

## 鍙傛暟



`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_enum_dv_timings` 鐨勬寚閽堛€?
## 鎻忚堪


铏界劧鏌愪簺 DV 鎺ユ敹绔垨鍙戦€佺鏀寔寰堝鑼冨洿鐨勬椂搴忥紝浣嗗彟涓€浜涘彧鏀寔鏁伴噺鏈夐檺鐨勬椂搴忋€傚簲鐢ㄧ▼搴忓彲閫氳繃鏈?ioctl 鏋氫妇涓€浠藉凡鐭ュ彈鏀寔鏃跺簭鐨勫垪琛ㄣ€傝嫢杩橀渶纭瀹冩槸鍚︽敮鎸佹湰鍒楄〃涔嬪鐨勫叾浠栨爣鍑嗙敋鑷宠嚜瀹氫箟鏃跺簭锛屽彲璋冪敤 VIDIOC_DV_TIMINGS_CAP 杩涜妫€鏌ャ€?
瑕佹煡璇㈠彲鐢ㄦ椂搴忥紝搴旂敤绋嬪簭鍏堝垵濮嬪寲 `index` 瀛楁锛屽皢 `pad` 瀛楁璁句负 0锛屾妸缁撴瀯浣?`v4l2_enum_dv_timings` 鐨?reserved 鏁扮粍娓呴浂锛岀劧鍚庡湪璇ヨ棰戣妭鐐逛笂浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_ENUM_DV_TIMINGS` ioctl銆傞┍鍔ㄤ細濉厖缁撴瀯鐨勫叾浣欓儴鍒嗭紱褰撶储寮曡秺鐣屾椂杩斿洖 `EINVAL` 閿欒鐮併€傝鏋氫妇鎵€鏈夊彈鏀寔鐨?DV 鏃跺簭锛屽簲鐢ㄧ▼搴忓簲浠庣储寮?0 寮€濮嬶紝姣忔鍔?1锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`銆?

   椹卞姩鍦ㄥ垏鎹㈣棰戣緭鍏ユ垨杈撳嚭鍚庯紝鍙兘浼氭灇涓惧嚭涓€缁勪笉鍚岀殑 DV 鏃跺簭銆?
褰撶敱椹卞姩瀹炵幇鏃讹紝瀛愯澶囩殑 DV 鏃跺簭鍙€氳繃鍦ㄥ瓙璁惧鑺傜偣涓婄洿鎺ヨ皟鐢?`VIDIOC_SUBDEV_ENUM_DV_TIMINGS` ioctl 鏉ユ煡璇€侱V 鏃跺簭閽堝杈撳叆锛圖V 鎺ユ敹绔級鎴栬緭鍑猴紙DV 鍙戦€佺锛夎€岀壒瀹氾紝搴旂敤绋嬪簭蹇呴』鍦ㄧ粨鏋勪綋 `v4l2_enum_dv_timings` 鐨?`pad` 瀛楁涓寚瀹氭墍闇€鐨?pad 缂栧彿銆傚皾璇曞湪涓嶆敮鎸佺殑 pad 涓婃灇涓炬椂搴忓皢杩斿洖 `EINVAL` 閿欒鐮併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - DV 鏃跺簭鐨勭紪鍙凤紝鐢卞簲鐢ㄧ▼搴忚缃€?    - - __u32
      - `pad`
      - 鐢卞獟浣撴帶鍒跺櫒 API 鎶ュ憡鐨?pad 缂栧彿銆傝瀛楁浠呭湪瀵瑰瓙璁惧鑺傜偣鎿嶄綔鏃朵娇鐢ㄣ€傚湪瀵硅棰戣妭鐐规搷浣滄椂锛屽簲鐢ㄧ▼搴忓繀椤诲皢璇ュ瓧娈佃涓?0銆?    - - __u32
      - `reserved`\ [^2^]
      - 淇濈暀浠ュ灏嗘潵鎵╁睍銆傞┍鍔ㄥ拰搴旂敤绋嬪簭閮藉繀椤诲皢璇ユ暟缁勭疆闆躲€?    - - struct `v4l2_dv_timings`
      - `timings`
      - 鏃跺簭銆?
## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    缁撴瀯浣?`v4l2_enum_dv_timings` 鐨?`index` 瓒婄晫锛屾垨 `pad` 缂栧彿鏃犳晥銆?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佹暟瀛楄棰戦璁俱€?