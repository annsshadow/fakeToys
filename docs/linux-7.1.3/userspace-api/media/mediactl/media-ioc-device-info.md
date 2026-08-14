


######## ioctl MEDIA_IOC_DEVICE_INFO


## 鍚嶇О


MEDIA_IOC_DEVICE_INFO - 鏌ヨ璁惧淇℃伅

## 姒傝



`int ioctl(int fd, MEDIA_IOC_DEVICE_INFO, struct media_device_info *argp)`

## 鍙傛暟



`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`media_device_info` 鐨勬寚閽堛€?
## 鎻忚堪


鎵€鏈夊獟浣撹澶囬兘蹇呴』鏀寔 `MEDIA_IOC_DEVICE_INFO` ioctl銆傝鏌ヨ璁惧淇℃伅锛屽簲鐢ㄧ▼搴忎互鎸囧悜缁撴瀯浣?`media_device_info` 鐨勬寚閽堣皟鐢ㄨ ioctl銆傞┍鍔ㄥ～鍏呰缁撴瀯骞跺皢淇℃伅杩斿洖缁欏簲鐢ㄧ▼搴忋€傝 ioctl 姘歌繙涓嶄細澶辫触銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  char
       - `driver`\ [^16^]
       - 瀹炵幇濯掍綋 API 鐨勯┍鍔ㄥ悕绉帮紝涓?NUL 缁撳熬鐨?ASCII 瀛楃涓层€傞┍鍔ㄧ増鏈瓨鍌ㄥ湪
	  `driver_version` 瀛楁涓€?
	  鐗瑰畾浜庨┍鍔ㄧ殑搴旂敤绋嬪簭鍙互浣跨敤姝や俊鎭潵楠岃瘉椹卞姩韬唤銆傚畠涔熸湁鍔╀簬瑙勯伩宸茬煡缂洪櫡锛?	  鎴栧湪閿欒鎶ュ憡涓瘑鍒┍鍔ㄣ€?
    - -  char
       - `model`\ [^32^]
       - 璁惧鍨嬪彿鍚嶇О锛屼负 NUL 缁撳熬鐨?UTF-8 瀛楃涓层€傝澶囩増鏈瓨鍌ㄥ湪 `device_version`
	  瀛楁涓紝涓斾笉闄勫姞鍒板瀷鍙峰悕绉颁箣鍚庛€?
    - -  char
       - `serial`\ [^40^]
       - 搴忓垪鍙凤紝涓?NUL 缁撳熬鐨?ASCII 瀛楃涓层€?
    - -  char
       - `bus_info`\ [^32^]
       - 璁惧鍦ㄧ郴缁熶腑鐨勪綅缃紝涓?NUL 缁撳熬鐨?ASCII 瀛楃涓层€傝繖鍖呮嫭鎬荤嚎绫诲瀷鍚嶇О
	  锛圥CI銆乁SB 绛夛級浠ュ強鎬荤嚎鐗瑰畾鐨勬爣璇嗙銆?
    - -  __u32
       - `media_version`
       - 濯掍綋 API 鐗堟湰锛屼娇鐢?`KERNEL_VERSION()` 瀹忔牸寮忓寲銆?
    - -  __u32
       - `hw_revision`
       - 纭欢璁惧淇鍙凤紝閲囩敤椹卞姩鐗瑰畾鐨勬牸寮忋€?
    - -  __u32
       - `driver_version`
       - 濯掍綋璁惧椹卞姩鐗堟湰锛屼娇鐢?`KERNEL_VERSION()` 瀹忔牸寮忓寲銆備笌 `driver` 瀛楁涓€璧?	  鐢ㄤ簬鏍囪瘑鐗瑰畾鐨勯┍鍔ㄣ€?
    - -  __u32
       - `reserved`\ [^31^]
       - 淇濈暀浠ュ灏嗘潵鎵╁睍銆傞┍鍔ㄥ拰搴旂敤绋嬪簭閮藉繀椤诲皢璇ユ暟缁勭疆闆躲€?
`serial` 涓?`bus_info` 瀛楁鍙敤浜庡尯鍒嗗涓叾浠栨柟闈㈢浉鍚岀殑纭欢瀹炰緥銆傚湪鎻愪緵搴忓垪鍙锋椂锛屽簭鍒楀彿浼樺厛锛屼笖鍙亣瀹氫负鍞竴銆傚鏋滃簭鍒楀彿涓虹┖瀛楃涓诧紝鍒欏彲鏀圭敤 `bus_info` 瀛楁銆俙bus_info` 瀛楁淇濊瘉鍞竴锛屼絾鍙兘鍦ㄩ噸鍚垨璁惧鎷旀彃涔嬮棿鍙樺寲銆?
## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?