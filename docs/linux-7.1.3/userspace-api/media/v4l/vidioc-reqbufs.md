


######## ioctl VIDIOC_REQBUFS


## Name


VIDIOC_REQBUFS - 鍙戣捣鍐呭瓨鏄犲皠銆佺敤鎴锋寚閽?I/O 鎴?DMA 缂撳啿鍖?I/O

## Synopsis



`int ioctl(int fd, VIDIOC_REQBUFS, struct v4l2_requestbuffers *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_requestbuffers` 鐨勬寚閽堛€?
## Description


姝?ioctl 鐢ㄤ簬鍙戣捣鍩轰簬鍐呭瓨鏄犲皠 <mmap>銆佺敤鎴锋寚閽?<userp> 鎴?DMABUF <dmabuf> 鐨?I/O銆?
鍐呭瓨鏄犲皠缂撳啿鍖轰綅浜庤澶囧唴瀛樹腑锛屽繀椤诲厛閫氳繃姝?ioctl 鍒嗛厤锛岀劧鍚庢墠鑳芥槧灏勫埌搴旂敤绋嬪簭鐨勫湴鍧€绌洪棿銆傜敤鎴风紦鍐插尯鐢卞簲鐢ㄧ▼搴忚嚜韬垎閰嶏紝姝?ioctl 浠呯敤浜庡皢椹卞姩鍒囨崲鍒扮敤鎴锋寚閽?I/O 妯″紡骞惰缃竴浜涘唴閮ㄧ粨鏋勩€傜被浼煎湴锛孌MABUF 缂撳啿鍖虹敱搴旂敤绋嬪簭閫氳繃璁惧椹卞姩鍒嗛厤锛屾 ioctl 浠呭皢椹卞姩閰嶇疆涓?DMABUF I/O 妯″紡锛岃€屼笉鎵ц浠讳綍鐩存帴鐨勫垎閰嶃€?
瑕佸垎閰嶈澶囩紦鍐插尯锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 struct `v4l2_requestbuffers` 缁撴瀯鐨勬墍鏈夊瓧娈点€傚畠浠皢 `type` 瀛楁璁句负鐩稿簲鐨勬祦鎴栫紦鍐插尯绫诲瀷锛屽皢 `count` 瀛楁璁句负鎵€闇€鐨勭紦鍐插尯鏁伴噺锛宍memory` 蹇呴』璁句负璇锋眰鐨?I/O 鏂规硶锛屽苟涓?`reserved` 鏁扮粍蹇呴』娓呴浂銆傚綋浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢ㄦ ioctl 鏃讹紝椹卞姩浼氬皾璇曞垎閰嶆墍璇锋眰鏁伴噺鐨勭紦鍐插尯锛屽苟灏嗗疄闄呭垎閰嶇殑缂撳啿鍖烘暟閲忓瓨鍏?`count` 瀛楁銆傚綋椹卞姩鑰楀敖绌洪棽鍐呭瓨鏃讹紝璇ュ€煎彲鑳藉皬浜庤姹傜殑鏁伴噺锛岀敋鑷充负 0銆傚綋椹卞姩闇€瑕佹洿澶氱紦鍐插尯鎵嶈兘姝ｅ父宸ヤ綔鏃讹紝涔熷彲鑳借繑鍥炴洿澶х殑鏁伴噺銆備緥濡傝棰戣緭鍑鸿嚦灏戦渶瑕佷袱涓紦鍐插尯锛屼竴涓敤浜庢樉绀猴紝涓€涓敱搴旂敤绋嬪簭濉厖銆?
褰?I/O 鏂规硶涓嶅彈鏀寔鏃讹紝姝?ioctl 杩斿洖 `EINVAL` 閿欒鐮併€?
搴旂敤绋嬪簭鍙互鍐嶆璋冪敤 VIDIOC_REQBUFS 鏉ユ敼鍙樼紦鍐插尯鏁伴噺銆傛敞鎰忥紝濡傛灉浠嶆湁浠讳綍缂撳啿鍖鸿鏄犲皠鎴栭€氳繃 DMABUF 瀵煎嚭锛岄偅涔堝彧鏈夊湪璁剧疆浜?`V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS` 鑳藉姏鏃?VIDIOC_REQBUFS 鎵嶈兘鎴愬姛銆傚惁鍒?VIDIOC_REQBUFS 灏嗚繑鍥?`EBUSY` 閿欒鐮併€傚鏋滆缃簡 `V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS`锛屽垯杩欎簺缂撳啿鍖轰細琚€滃鍎垮寲锛坥rphaned锛夆€濓紝骞跺湪瀹冧滑琚彇娑堟槧灏勬垨瀵煎嚭鐨?DMABUF fds 琚叧闂椂琚噴鏀俱€俙count` 鍊间负 0 浼氬湪涓鎴栧畬鎴愪换浣曡繘琛屼腑鐨?DMA 涔嬪悗閲婃斁鎴栧鍎垮寲鎵€鏈夌紦鍐插尯锛岃繖鏄竴涓殣寮忕殑 VIDIOC_STREAMOFF <VIDIOC_STREAMON>銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `count`
      - 璇锋眰鎴栨巿浜堢殑缂撳啿鍖烘暟閲忋€?    - - __u32
      - `type`
      - 娴佹垨缂撳啿鍖虹殑绫诲瀷锛屼笌 struct `v4l2_format` 鐨?`type` 瀛楁鐩稿悓銆傛湁鏁堝€煎弬瑙?`v4l2_buf_type`銆?    - - __u32
      - `memory`
      - 搴旂敤绋嬪簭灏嗘瀛楁璁句负 `V4L2_MEMORY_MMAP`銆乣V4L2_MEMORY_DMABUF` 鎴?`V4L2_MEMORY_USERPTR`銆傚弬瑙?`v4l2_memory`銆?    - - __u32
      - `capabilities`
      - 鐢遍┍鍔ㄨ缃€傚鏋滀负 0锛岃鏄庨┍鍔ㄤ笉鏀寔鑳藉姏鏌ヨ銆傚湪杩欑鎯呭喌涓嬶紝浣犳墍鐭ラ亾鐨勫彧鏄┍鍔ㄤ繚璇佹敮鎸?`V4L2_MEMORY_MMAP`锛屽苟涓?*鍙兘**鏀寔鍏朵粬 `v4l2_memory` 绫诲瀷銆傚畠涓嶄細鏀寔浠讳綍鍏朵粬鑳藉姏銆?
	濡傛灉浣犳兂浠ユ渶灏忕殑鍓綔鐢ㄦ煡璇㈣兘鍔涳紝鍙互浣跨敤 `count` 璁句负 0銆乣memory` 璁句负 `V4L2_MEMORY_MMAP`銆乣type` 璁句负缂撳啿鍖虹被鍨嬫潵璋冪敤銆傝繖浼氶噴鏀句换浣曚箣鍓嶅垎閰嶇殑缂撳啿鍖猴紝鍥犳閫氬父鏄湪搴旂敤绋嬪簭鍚姩鏃惰繘琛岀殑鎿嶄綔銆?    - - __u8
      - `flags`
      - 鎸囧畾棰濆鐨勭紦鍐插尯绠＄悊灞炴€с€傚弬瑙?memory-flags銆?    - - __u8
      - `reserved`\ [^3^]
      - 淇濈暀渚涘皢鏉ユ墿灞曚娇鐢ㄣ€?

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_BUF_CAP_SUPPORTS_MMAP`
      - 0x00000001
      - 姝ょ紦鍐插尯绫诲瀷鏀寔 `V4L2_MEMORY_MMAP` 娴佹ā寮忋€?    - - `V4L2_BUF_CAP_SUPPORTS_USERPTR`
      - 0x00000002
      - 姝ょ紦鍐插尯绫诲瀷鏀寔 `V4L2_MEMORY_USERPTR` 娴佹ā寮忋€?    - - `V4L2_BUF_CAP_SUPPORTS_DMABUF`
      - 0x00000004
      - 姝ょ紦鍐插尯绫诲瀷鏀寔 `V4L2_MEMORY_DMABUF` 娴佹ā寮忋€?    - - `V4L2_BUF_CAP_SUPPORTS_REQUESTS`
      - 0x00000008
      - 姝ょ紦鍐插尯绫诲瀷鏀寔璇锋眰 <media-request-api>銆?    - - `V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS`
      - 0x00000010
      - 鍐呮牳鍏佽鍦ㄧ紦鍐插尯浠嶈鏄犲皠鎴栭€氳繃 DMABUF 瀵煎嚭鏃惰皟鐢?VIDIOC_REQBUFS銆傝繖浜涒€滃鍎垮寲鈥濈殑缂撳啿鍖轰細鍦ㄥ畠浠鍙栨秷鏄犲皠鎴栧鍑虹殑 DMABUF fds 琚叧闂椂琚噴鏀俱€?    - - `V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF`
      - 0x00000020
      - 浠呭鏃犵姸鎬佽В鐮佸櫒鏈夋晥銆傚鏋滆缃紝鍒欑敤鎴风┖闂村彲浠ヨ缃?`V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF` 鏍囧織锛屼互寤惰繜杩斿洖鎹曡幏缂撳啿鍖猴紝鐩村埌 OUTPUT 鏃堕棿鎴冲彂鐢熷彉鍖栥€?    - - `V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS`
      - 0x00000040
      - 姝よ兘鍔涚敱椹卞姩璁剧疆锛岃〃绀洪槦鍒楁敮鎸佺紦瀛樺拰鍐呭瓨绠＄悊鎻愮ず銆傜劧鑰岋紝瀹冧粎鍦ㄩ槦鍒楃敤浜庡唴瀛樻槧灏?<mmap> 娴?I/O 鏃舵墠鏈夋晥銆傚弬瑙?V4L2_BUF_FLAG_NO_CACHE_INVALIDATE <V4L2-BUF-FLAG-NO-CACHE-INVALIDATE>銆乂4L2_BUF_FLAG_NO_CACHE_CLEAN <V4L2-BUF-FLAG-NO-CACHE-CLEAN> 鍜?V4L2_MEMORY_FLAG_NON_COHERENT <V4L2-MEMORY-FLAG-NON-COHERENT>銆?    - - `V4L2_BUF_CAP_SUPPORTS_MAX_NUM_BUFFERS`
      - 0x00000080
      - 濡傛灉璁剧疆锛屽垯 `struct v4l2_create_buffers` 涓殑 `max_num_buffers` 瀛楁鏈夋晥銆傚鏋滄湭璁剧疆锛屽垯鏈€澶у€间负 `VIDEO_MAX_FRAME` 涓紦鍐插尯銆?    - - `V4L2_BUF_CAP_SUPPORTS_REMOVE_BUFS`
      - 0x00000100
      - 濡傛灉璁剧疆锛屽垯鏀寔 `VIDIOC_REMOVE_BUFS`銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_MEMORY_FLAG_NON_COHERENT`
      - 0x00000001
      - 缂撳啿鍖鸿鍒嗛厤鍦ㄤ竴鑷达紙coherent锛屽畠灏嗗湪 CPU 鍜屾€荤嚎涔嬮棿鑷姩淇濇寔涓€鑷达級鎴栭潪涓€鑷达紙non-coherent锛夊唴瀛樹腑銆傚悗鑰呭彲浠ユ彁渚涙€ц兘鎻愬崌锛屼緥濡傦紝濡傛灉缂撳啿鍖轰粎鐢辩浉搴旇澶囪闂笖 CPU 涓嶅璇ョ紦鍐插尯杩涜璇诲啓锛屽垯鍙互閬垮厤 CPU 缂撳瓨鍚屾/鍒锋柊鎿嶄綔銆傜劧鑰岋紝杩欓渶瑕侀┍鍔ㄦ牸澶栧皬蹇冣€斺€斿畠蹇呴』鍦ㄩ渶瑕佷竴鑷存€ф椂閫氳繃鍙戝嚭缂撳瓨鍒锋柊/鍚屾鏉ヤ繚璇佸唴瀛樹竴鑷存€с€傚鏋滆缃簡姝ゆ爣蹇楋紝V4L2 灏嗗皾璇曞湪闈炰竴鑷村唴瀛樹腑鍒嗛厤缂撳啿鍖恒€傝鏍囧織浠呭湪缂撳啿鍖虹敤浜庡唴瀛樻槧灏?<mmap> I/O 涓旈槦鍒楁姤鍛婁簡 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS <V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 鑳藉姏鏃舵墠鐢熸晥銆?

   \normalsize

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞堕€傚綋鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    缂撳啿鍖虹被鍨嬶紙`type` 瀛楁锛夋垨璇锋眰鐨?I/O 鏂规硶锛坄memory`锛変笉鍙楁敮鎸併€?