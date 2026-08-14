

######## ioctl VIDIOC_G_SELECTION, VIDIOC_S_SELECTION


## Name


VIDIOC_G_SELECTION - VIDIOC_S_SELECTION - 鑾峰彇鎴栬缃叾涓竴涓€夋嫨鐭╁舰

## Synopsis


`int ioctl(int fd, VIDIOC_G_SELECTION, struct v4l2_selection *argp)`


`int ioctl(int fd, VIDIOC_S_SELECTION, struct v4l2_selection *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜 struct `v4l2_selection` 鐨勬寚閽堛€?

## Description


杩欎簺 ioctl 鐢ㄤ簬鏌ヨ鍜岄厤缃€夋嫨鐭╁舰銆?

瑕佹煡璇㈣鍓紙缁勫悎锛夌煩褰紝璇峰皢 struct `v4l2_selection` 鐨?`type` 瀛楁璁剧疆涓虹浉搴旂殑缂撳啿鍖虹被鍨嬨€備笅涓€姝ユ槸灏?struct `v4l2_selection` 鐨?`target` 瀛楁璁剧疆涓?`V4L2_SEL_TGT_CROP`锛坄V4L2_SEL_TGT_COMPOSE`锛夈€傛洿澶氱洰鏍囪鍙傝€冭〃 v4l2-selections-common 鎴?selection-api銆俿truct `v4l2_selection` 鐨?`flags` 鍜?`reserved` 瀛楁浼氳蹇界暐锛屽繀椤诲～闆躲€傞┍鍔ㄤ細濉厖缁撴瀯鐨勫叾浣欓儴鍒嗭紝濡傛灉浣跨敤浜嗕笉姝ｇ‘鐨勭紦鍐插尯绫诲瀷鎴栫洰鏍囧垯杩斿洖 EINVAL 閿欒鐮併€傚鏋滀笉鏀寔瑁佸壀锛堢粍鍚堬級锛屽垯娲诲姩鐭╁舰涓嶅彲鍙橈紝涓斿缁堢瓑浜庤竟鐣岀煩褰€傛渶鍚庯紝struct `v4l2_rect` 鐨?`r` 鐭╁舰浼氳濉叆褰撳墠鐨勮鍓紙缁勫悎锛夊潗鏍囥€傚潗鏍囦互椹卞姩鐩稿叧鐨勫崟浣嶈〃绀恒€傚敮涓€鐨勪緥澶栨槸鍘熷鏍煎紡鍥惧儚鐨勭煩褰紝鍏跺潗鏍囧缁堜互鍍忕礌琛ㄧず銆?

瑕佷慨鏀硅鍓紙缁勫悎锛夌煩褰紝璇峰皢 struct `v4l2_selection` 鐨?`type` 瀛楁璁剧疆涓虹浉搴旂殑缂撳啿鍖虹被鍨嬨€備笅涓€姝ユ槸灏?struct `v4l2_selection` 鐨?`target` 璁剧疆涓?`V4L2_SEL_TGT_CROP`锛坄V4L2_SEL_TGT_COMPOSE`锛夈€傛洿澶氱洰鏍囪鍙傝€冭〃 v4l2-selections-common 鎴?selection-api銆俿truct `v4l2_rect` 鐨?`r` 鐭╁舰闇€瑕佽璁剧疆涓烘湡鏈涚殑娲诲姩鍖哄煙銆俿truct `v4l2_selection` 鐨?`reserved` 瀛楁浼氳蹇界暐锛屽繀椤诲～闆躲€傞┍鍔ㄥ彲鑳戒細璋冩暣鎵€璇锋眰鐭╁舰鐨勫潗鏍囥€傚簲鐢ㄧ▼搴忓彲浠ュ紩鍏ョ害鏉熸潵鎺у埗鑸嶅叆琛屼负銆俿truct `v4l2_selection` 鐨?`flags` 瀛楁蹇呴』璁剧疆涓轰笅鍒椾箣涓€锛?

- `0` - 椹卞姩鍙互鑷敱璋冩暣鐭╁舰澶у皬锛屽苟搴旈€夋嫨灏藉彲鑳芥帴杩戞墍璇锋眰鐭╁舰鐨勮鍓?缁勫悎鐭╁舰銆?

- `V4L2_SEL_FLAG_GE` - 涓嶅厑璁搁┍鍔ㄧ缉灏忕煩褰€傚師濮嬬煩褰㈠繀椤讳綅浜庤皟鏁村悗鐨勭煩褰㈠唴閮ㄣ€?

- `V4L2_SEL_FLAG_LE` - 涓嶅厑璁搁┍鍔ㄦ斁澶х煩褰€傝皟鏁村悗鐨勭煩褰㈠繀椤讳綅浜庡師濮嬬煩褰㈠唴閮ㄣ€?

- `V4L2_SEL_FLAG_GE | V4L2_SEL_FLAG_LE` - 椹卞姩蹇呴』閫夋嫨澶у皬涓庢墍璇锋眰鐭╁舰瀹屽叏鐩稿悓鐨勭煩褰€?

璇峰弬鑰?sel-const-adjust銆?

椹卞姩鍙兘蹇呴』鏍规嵁纭欢闄愬埗浠ュ強娴佹按绾跨殑鍏朵粬閮ㄥ垎锛堝嵆鎹曡幏/杈撳嚭绐楀彛鎴栫數瑙嗘樉绀烘墍缁欏嚭鐨勮竟鐣岋級鏉ヨ皟鏁存墍璇锋眰鐨勫昂瀵搞€傛寜鐓т互涓嬩紭鍏堢骇閫夋嫨灏藉彲鑳芥帴杩戠殑姘村钩涓庡瀭鐩村亸绉诲強澶у皬锛?

1. 婊¤冻鏉ヨ嚜 struct `v4l2_selection` `flags` 鐨勭害鏉熴€?

2. 鏍规嵁纭欢闄愬埗鍜屽榻愯姹傝皟鏁村搴︺€侀珮搴︺€佸乏杈瑰拰涓婅竟銆?

3. 浣胯皟鏁村悗鐭╁舰鐨勪腑蹇冨敖鍙兘鎺ヨ繎鍘熷鐭╁舰銆?

4. 浣垮搴﹀拰楂樺害灏藉彲鑳芥帴杩戝師濮嬪€笺€?

5. 浣挎按骞冲拰鍨傜洿鍋忕Щ灏藉彲鑳芥帴杩戝師濮嬪€笺€?

鎴愬姛鏃讹紝struct `v4l2_rect` 鐨?`r` 瀛楁鍖呭惈璋冩暣鍚庣殑鐭╁舰銆傚綋鍙傛暟涓嶅悎閫傛椂锛屽簲鐢ㄧ▼搴忓彲浠ヤ慨鏀硅鍓紙缁勫悎锛夋垨鍥惧儚鍙傛暟骞堕噸澶嶈寰幆锛岀洿鍒板崗鍟嗗嚭婊℃剰鐨勫弬鏁般€傚鏋滃繀椤昏繚鍙嶇害鏉熸爣蹇楋紝鍒欒繑鍥?`ERANGE`銆傝閿欒琛ㄦ槑**涓嶅瓨鍦?*婊¤冻绾︽潫鐨勭煩褰€?

閫夋嫨鐩爣涓庢爣蹇楀湪 v4l2-selections-common 涓湁鏂囨。璇存槑銆?


    :alt:    constraints.svg
    :align:  center

    甯︾害鏉熸爣蹇楃殑灏哄璋冩暣銆?

    涓嶅悓绾︽潫鏍囧織涓嬬煩褰㈣皟鏁寸殑琛屼负銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 缂撳啿鍖虹殑绫诲瀷锛堟潵鑷?enum `v4l2_buf_type`锛夈€?
    - - __u32
      - `target`
      - 鐢ㄤ簬鍦ㄨ鍓煩褰㈠拰缁勫悎鐭╁舰涔嬮棿杩涜閫夋嫨 <v4l2-selections-common>銆?
    - - __u32
      - `flags`
      - 鎺у埗閫夋嫨鐭╁舰璋冩暣鐨勬爣蹇楋紝璇峰弬鑰?selection flags <v4l2-selection-flags>銆?
    - - struct `v4l2_rect`
      - `r`
      - 閫夋嫨鐭╁舰銆?
    - - __u32
      - `reserved[^9^]`
      - 渚涘皢鏉ヤ娇鐢ㄧ殑淇濈暀瀛楁銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗘湰鏁扮粍娓呴浂銆?

   閬楁喚鐨勬槸锛屽浜庡骞抽潰缂撳啿鍖虹被鍨嬶紙`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` 鍜?`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`锛夛紝鍏充簬搴斿浣曞～鍐?`v4l2_selection` 鐨?`type` 瀛楁锛岃 API 鏄贩涔辩殑銆傛煇浜涢┍鍔ㄥ彧鎺ュ彈 `_MPLANE` 缂撳啿鍖虹被鍨嬶紝鑰屽彟涓€浜涢┍鍔ㄥ彧鎺ュ彈闈炲骞抽潰缂撳啿鍖虹被鍨嬶紙鍗虫湯灏句笉甯?`_MPLANE`锛夈€?

   浠庡唴鏍?4.13 寮€濮嬶紝涓ょ鍐欐硶閮借鍏佽銆?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    缁欏畾鐨勭紦鍐插尯绫诲瀷 `type` 鎴栭€夋嫨鐩爣 `target` 涓嶅彈鏀寔锛屾垨鑰?`flags` 鍙傛暟鏃犳晥銆?

ERANGE
    鏃犳硶璋冩暣 struct `v4l2_rect` 鐨?`r` 鐭╁舰浠ユ弧瓒?`flags` 鍙傛暟涓粰鍑虹殑鎵€鏈夌害鏉熴€?

ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸侀€夋嫨銆?

EBUSY
    褰撳墠鏃犳硶搴旂敤閫夋嫨鐭╁舰鐨勪慨鏀广€傞€氬父鏄洜涓烘鍦ㄨ繘琛屾祦浼犺緭銆?
