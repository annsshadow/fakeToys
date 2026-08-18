
## ARM Mali-C55 Image Signal Processor driver锛圓RM Mali-C55 鍥惧儚淇″彿澶勭悊鍣ㄩ┍鍔級


## Introduction锛堢畝浠嬶級


鏈枃浠惰褰曚簡 ARM Mali-C55 鍥惧儚淇″彿澶勭悊鍣紙ISP锛夌殑椹卞姩銆傝椹卞姩浣嶄簬
drivers/media/platform/arm/mali-c55銆?
Mali-C55 ISP 鎺ユ敹鏉ヨ嚜浼犳劅鍣ㄧ殑鏁版嵁锛屾暟鎹彲浠ユ槸鍘熷 Bayer 鏍煎紡鎴?RGB/YUV 鏍煎紡锛?閫氳繃骞惰鎺ュ彛鎴栧唴瀛樻€荤嚎杩涘叆锛岀粡澶勭悊鍚庣敱鍐呴儴 DMA 寮曟搸杈撳嚭銆傚瓨鍦ㄤ袱鏉″彲鑳界殑
杈撳嚭娴佹按绾匡紙涓嶈繃鍏蜂綋瀹炵幇鍙兘鍙閰嶄簡涓€鏉★級锛屽垎鍒О涓衡€滃叏鍒嗚鲸鐜囷紙Full resolution锛夆€?鍜屸€滅缉鏀撅紙Downscale锛夆€濓紝浣嗘鍛藉悕鏄巻鍙叉部鐢紝涓ゆ潯娴佹按绾块兘鍏峰瑁佸壀/缂╂斁鑳藉姏銆?鍏ㄥ垎杈ㄧ巼娴佹按绾胯繕鑳借緭鍑?RAW 鏁版嵁锛岀粫杩?ISP 鐨勫ぇ閮ㄥ垎澶勭悊锛涚缉鏀炬祦姘寸嚎鍒欎笉鑳借緭鍑?RAW 鏁版嵁銆傞泦鎴愮殑娴嬭瘯鍥炬鍙戠敓鍣ㄥ彲鍦ㄦ病鏈夎繛鎺ョ浉鏈轰紶鎰熷櫒鏃堕┍鍔?ISP 骞朵骇鐢熷浘鍍忔暟鎹€?椹卞姩妯″潡鍚嶄负 mali_c55锛岄€氳繃 CONFIG_VIDEO_MALI_C55 閰嶇疆閫夐」鍚敤銆?
璇ラ┍鍔ㄥ疄鐜颁簡 V4L2銆丮edia Controller 涓?V4L2 Subdevice 鎺ュ彛锛屽苟鏈熸湜杩炴帴鍒?ISP
鐨勭浉鏈轰紶鎰熷櫒鍏峰 V4L2 瀛愯澶囨帴鍙ｃ€?
## Mali-C55 ISP hardware锛圡ali-C55 ISP 纭欢锛?

涓嬮潰缁欏嚭 Mali-C55 ISP 鐨勯珮灞傚姛鑳借鍥俱€侷SP 鐨勮緭鍏ュ彲浠ユ潵鑷疄鏃舵簮锛屾垨閫氳繃 DMA
寮曟搸鏉ヨ嚜鍐呭瓨杈撳叆锛?```

  +---------+    +----------+                                     +--------+
  | Sensor  |--->| CSI-2 Rx |                "Full Resolution"    |  DMA   |
  +---------+    +----------+   |\                 Output    +--->| Writer |
                       |        | \                          |    +--------+
                       |        |  \    +----------+  +------+---> Streaming I/O
  +------------+       +------->|   |   |          |  |
  |            |                |   |-->| Mali-C55 |--+
  | DMA Reader |--------------->|   |   |    ISP   |  |
  |            |                |  /    |          |  |      +---> Streaming I/O
  +------------+                | /     +----------+  |      |
                                |/                    +------+
                                                             |    +--------+
                                                             +--->|  DMA   |
                                               "Downscaled"       | Writer |
                                                  Output          +--------+

```
## Media Controller Topology锛堝獟浣撴帶鍒跺櫒鎷撴墤锛?

涓嬮潰缁欏嚭涓€涓?ISP 鎷撴墤鐨勭ず渚嬶紙瀹炵幇浜庡甫 IMX415 鐩告満浼犳劅鍣ㄤ笌閫氱敤 CSI-2 鎺ユ敹鍣ㄧ殑
绯荤粺锛夛細

    :alt:   mali-c55-graph.dot
    :align: center

璇ラ┍鍔ㄦ嫢鏈?4 涓?V4L2 瀛愯澶囷細

- `mali_c55 isp`锛氳礋璐ｉ厤缃緭鍏ヨ鍓笌鑹插僵绌洪棿杞崲
- `mali_c55 tpg`锛氭祴璇曞浘妗堝彂鐢熷櫒锛屾ā鎷熺浉鏈轰紶鎰熷櫒
- `mali_c55 resizer fr`锛氬叏鍒嗚鲸鐜囨祦姘寸嚎鐨?resizer
- `mali_c55 resizer ds`锛氱缉鏀炬祦姘寸嚎鐨?resizer

璇ラ┍鍔ㄦ嫢鏈?3 涓?V4L2 瑙嗛璁惧锛?
- `mali-c55 fr`锛氬叏鍒嗚鲸鐜囨祦姘寸嚎鐨勯噰闆嗚澶?- `mali-c55 ds`锛氱缉鏀炬祦姘寸嚎鐨勯噰闆嗚澶?- `mali-c55 3a stats`锛?A 缁熻淇℃伅閲囬泦璁惧

甯у簭鍒楀湪涓や釜閲囬泦璁惧涔嬮棿鏄悓姝ョ殑锛屼篃灏辨槸璇达紝濡傛灉鏌愭潯娴佹按绾挎瘮鍙︿竴鏉″惎鍔ㄥ緱鏅氾紝
鍏剁紦鍐插尯涓繑鍥炵殑搴忓彿灏嗕笌鍙︿竴鏉℃祦姘寸嚎鐩稿尮閰嶏紝鑰屼笉鏄粠闆跺紑濮嬨€?
### Idiosyncrasies锛堢壒鎬у樊寮傦級


**mali-c55 isp**
`mali-c55 isp` 瀛愯澶囨湁涓€涓崟涓€鐨?sink 琛灚锛坧ad锛夛紝鎵€鏈夋暟鎹簮閮藉簲杩炴帴鍒板畠銆?閫氳繃鍚敤鐩稿簲鐨勫獟浣撻摼璺苟绂佺敤鍏朵粬鎵€鏈夐摼璺潵閫夋嫨娲昏穬鐨勬暟鎹簮銆侷SP 鏈変袱涓?source
琛灚锛屽弽鏄犱簡鍏跺唴閮ㄨ矾鐢辨暟鎹殑涓嶅悓璺緞銆侷SP 鍐呴儴鐨勬娊澶寸偣锛坱ap point锛夊厑璁哥敤鎴峰垎娴?鏁版嵁锛屼互閬垮紑閮ㄥ垎鎴栧叏閮ㄧ‖浠跺鐞嗘楠ゃ€備笅鍥句粎鐢ㄤ簬璇存槑鏃佽矾鏈哄埗濡備綍宸ヤ綔锛屽苟闈炲
閭ｄ簺澶勭悊姝ラ鐨勭湡瀹炲弽鏄狅紱鏈夊叧楂樺眰鍔熻兘妗嗗浘锛岃鍙傞槄 ARM 鐨?Mali-C55 寮€鍙戣€呴〉闈細
```

  +--------------------------------------------------------------+
  |                Possible Internal ISP Data Routes             |
  |          +------------+  +----------+  +------------+        |
  +---+      |            |  |          |  |  Colour    |    +---+
  | 0 |--+-->| Processing |->| Demosaic |->|   Space    |--->| 1 |
  +---+  |   |            |  |          |  | Conversion |    +---+
  |      |   +------------+  +----------+  +------------+        |
  |      |                                                   +---+
  |      +---------------------------------------------------| 2 |
  |                                                          +---+
  |                                                              |
  +--------------------------------------------------------------+


```
    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 鏁版嵁杈撳叆锛岃繛鎺ュ埌 TPG 涓庣浉鏈轰紶鎰熷櫒

    - - 1
      - source
      - RGB/YUV 鏁版嵁锛岃繛鎺ュ埌 FR 涓?DS V4L2 瀛愯澶?
    - - 2
      - source
      - RAW bayer 鏁版嵁锛岃繛鎺ュ埌 FR V4L2 瀛愯澶?
ISP 鐨勮緭鍏ヤ笌杈撳嚭鍒嗚鲸鐜囧潎闄愬埗鍦?640x480 鍒?8192x8192 涔嬮棿锛岃繖涓€鐐逛綋鐜板湪 ISP 涓?resizer 瀛愯澶囩殑 .set_fmt() 鎿嶄綔涓€?
**mali-c55 resizer fr**
`mali-c55 resizer fr` 瀛愯澶囨湁涓や釜 *sink* 琛灚锛屼互鍙嶆槧纭欢涓笉鍚岀殑鎻掑叆鐐癸紙RAW 鎴?鍘婚┈璧涘厠鍚庣殑鏁版嵁锛夛細

    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 鏁版嵁杈撳叆锛岃繛鎺ュ埌 ISP 鐨勫幓椹禌鍏嬫暟鎹祦

    - - 1
      - source
      - 鏁版嵁杈撳嚭锛岃繛鎺ュ埌閲囬泦瑙嗛璁惧

    - - 2
      - sink
      - 鏁版嵁杈撳叆锛岃繛鎺ュ埌 ISP 鐨?raw 鏁版嵁娴?
瀹為檯浣跨敤鐨勭殑鏁版嵁婧愰€氳繃璺敱 API 閫夋嫨锛涘彲鐢ㄤ袱鏉″悇鍚崟娴佺殑璺敱锛?
    :header-rows: 1

    - - Sink Pad
      - Source Pad
      - Purpose

    - - 0
      - 1
      - 鍘婚┈璧涘厠鏁版嵁璺敱

    - - 2
      - 1
      - 鍘熷鏁版嵁璺敱


濡傛灉鍘婚┈璧涘厠璺敱澶勪簬娲昏穬鐘舵€侊紝鍒?FR 娴佹按绾垮彧鑳戒互 RGB/YUV 鏍煎紡杈撳嚭銆傚鏋?raw 璺敱
澶勪簬娲昏穬鐘舵€侊紝鍒欒緭鍑哄弽鏄犺緭鍏ワ紙鍙互鏄?Bayer 鎴?RGB/YUV 鏁版嵁锛夈€?
## Using the driver to capture video锛堜娇鐢ㄩ┍鍔ㄩ噰闆嗚棰戯級


鍒╃敤濯掍綋鎺у埗鍣?API锛屾垜浠彲浠ュ皢杈撳叆婧愪笌 ISP 閰嶇疆涓轰互澶氱鏍煎紡閲囬泦鍥惧儚銆傚湪浠ヤ笅
绀轰緥涓紝濯掍綋鍥剧殑閰嶇疆閫氳繃 v4l-utils [^1^]_ 杞欢鍖呯殑 media-ctl 宸ュ叿瀹屾垚锛屽浘鍍忕殑
閲囬泦鍒欓€氳繃 yavta [^2^]_ 瀹屾垚銆?
### Configuring the input source锛堥厤缃緭鍏ユ簮锛?

绗竴姝ユ槸閫氳繃鍚敤姝ｇ‘鐨勫獟浣撻摼璺潵璁惧畾鎴戜滑鏈熸湜鐨勮緭鍏ユ簮銆備娇鐢ㄤ笂闈㈢殑绀轰緥鎷撴墤锛?鎴戜滑鍙互濡備笅閫夋嫨 TPG锛?
    media-ctl -l "'lte-csi2-rx':1->'mali-c55 isp':0[^0^]"
    media-ctl -l "'mali-c55 tpg':0->'mali-c55 isp':0[^1^]"

### Configuring which video devices will stream data锛堥厤缃皢娴佸紡浼犺緭鏁版嵁鐨勮棰戣澶囷級


椹卞姩浼氱瓑寰呮墍鏈夎棰戣澶囬兘璋冪敤浜?VIDIOC_STREAMON ioctl 涔嬪悗锛屾墠鍛婄煡浼犳劅鍣ㄥ紑濮?娴佸紡浼犺緭銆備负姝わ紝鎴戜滑闇€瑕佸惎鐢ㄥ埌鎯宠浣跨敤鐨勮棰戣澶囩殑閾捐矾銆傚湪涓嬮潰鐨勭ず渚嬩腑锛屾垜浠?鍚敤浜嗗埌涓や釜鍥惧儚閲囬泦瑙嗛璁惧鐨勯摼璺細

    media-ctl -l "'mali-c55 resizer fr':1->'mali-c55 fr':0[^1^]"
    media-ctl -l "'mali-c55 resizer ds':1->'mali-c55 ds':0[^1^]"

### Capturing bayer data from the source and processing to RGB/YUV锛堥噰闆嗘簮绔殑 bayer 鏁版嵁骞跺鐞嗕负 RGB/YUV锛?

瑕佷粠婧愮閲囬泦 1920x1080 鐨?bayer 鏁版嵁锛屽苟灏嗗叾鎺ㄨ繃 ISP 鐨勫畬鏁村鐞嗘祦姘寸嚎锛屾垜浠渶瑕佸湪
婧愩€両SP 涓?resizer 瀛愯澶囦笂閫傚綋鍦伴厤缃暟鎹牸寮忥紝骞跺皢 FR resizer 鐨勮矾鐢辫涓洪€夋嫨
宸插鐞嗙殑鏁版嵁銆俽esizer source 琛灚涓婄殑濯掍綋鎬荤嚎鏍煎紡灏嗘槸 RGB121212_1X36 鎴?YUV10_1X30锛屽彇鍐充簬浣犳兂瑕侀噰闆?RGB 杩樻槸 YUV銆侷SP 鐨勫幓椹禌鍏嬪潡鍘熺敓杈撳嚭 RGB 鏁版嵁锛?灏?source 琛灚鏍煎紡璁句负 YUV10_1X30 浼氬惎鐢ㄨ壊褰╃┖闂磋浆鎹㈠潡銆?
鍦ㄦ湰绀轰緥涓紝鎴戜滑浠?RGB565 杈撳嚭涓虹洰鏍囷紝鍥犳閫夋嫨 RGB121212_1X36 浣滀负 resizer source
琛灚鐨勬牸寮忥細

    # Set formats on the TPG and ISP
    media-ctl -V "'mali-c55 tpg':0[fmt:SRGGB20_1X20/1920x1080]"
    media-ctl -V "'mali-c55 isp':0[fmt:SRGGB20_1X20/1920x1080]"
    media-ctl -V "'mali-c55 isp':1[fmt:SRGGB20_1X20/1920x1080]"

    # Set routing on the FR resizer
    media-ctl -R "'mali-c55 resizer fr'[0/0->1/0[^1^],2/0->1/0[^0^]]"

    # Set format on the resizer, must be done AFTER the routing.
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/1920x1080]"

缂╂斁杈撳嚭涔熷彲鍚屾椂鐢ㄤ簬娴佸紡浼犺緭鏁版嵁銆傚湪鏈緥涓紝鐢变簬缂╂斁杈撳嚭鍙兘閲囬泦宸插鐞嗙殑鏁版嵁锛?鍥犳鏃犻渶璁剧疆璺敱锛?
    # Set format on the resizer
    media-ctl -V "'mali-c55 resizer ds':1[fmt:RGB121212_1X36/1920x1080]"

闅忓悗鍗冲彲浠?FR 涓?DS 涓や釜杈撳嚭鐨勮棰戣澶囬噰闆嗗浘鍍忥紙鑻ラ渶瑕佷篃鍙悓鏃惰繘琛岋級锛?
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video0
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video1

#### Cropping the image锛堣鍓浘鍍忥級


鍏ㄥ垎杈ㄧ巼涓庣缉鏀句袱鏉℃祦姘寸嚎閮借兘瑁佸壀鍒版渶灏忓垎杈ㄧ巼 640x480銆傝瑁佸壀鍥惧儚锛屽彧闇€閰嶇疆
resizer sink 琛灚鐨?crop 涓?compose 鐭╁舰锛屽苟鍦ㄨ棰戣澶囦笂璁剧疆鏍煎紡锛?
    media-ctl -V "'mali-c55 resizer fr':0[fmt:RGB121212_1X36/1920x1080 crop:(480,270)/640x480 compose:(0,0)/640x480]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/640x480]"
    yavta -f RGB565 -s 640x480 -c10 /dev/video0

#### Downscaling the image锛堢缉灏忓浘鍍忥級


鍏ㄥ垎杈ㄧ巼涓庣缉鏀句袱鏉℃祦姘寸嚎閮借兘灏嗗浘鍍忕缉灏忚嚦澶?8 鍊嶏紝鍓嶆彁鏄伒瀹堟渶灏?640x480 鐨?杈撳嚭鍒嗚鲸鐜囥€備负鑾峰緱鏈€浣冲浘鍍忔晥鏋滐紝鍚勬柟鍚戠殑缂╂斁姣斿簲鐩稿悓銆傝閰嶇疆缂╂斁锛屾垜浠娇鐢?resizer sink 琛灚涓婄殑 compose 鐭╁舰锛?
    media-ctl -V "'mali-c55 resizer fr':0[fmt:RGB121212_1X36/1920x1080 crop:(0,0)/1920x1080 compose:(0,0)/640x480]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/640x480]"
    yavta -f RGB565 -s 640x480 -c10 /dev/video0

#### Capturing images in YUV formats锛堜互 YUV 鏍煎紡閲囬泦鍥惧儚锛?

濡傛灉鎴戜滑闇€瑕佽緭鍑?YUV 鏁版嵁鑰岄潪 RGB锛屽垯闇€瑕佸惎鐢ㄨ壊褰╃┖闂磋浆鎹㈠潡锛屾柟娉曟槸鍦?resizer
鐨?source 琛灚涓婅缃?MEDIA_BUS_FMT_YUV10_1X30銆傞殢鍚庢垜浠彲浠ラ厤缃竴涓噰闆嗘牸寮忥紝
渚嬪 NV12锛堟澶勪负鍏跺骞抽潰鍙樹綋锛夛細

    media-ctl -V "'mali-c55 resizer fr':1[fmt:YUV10_1X30/1920x1080]"
    yavta -f NV12M -s 1920x1080 -c10 /dev/video0

### Capturing RGB data from the source and processing it with the resizers锛堥噰闆嗘簮绔殑 RGB 鏁版嵁骞剁敤 resizer 澶勭悊锛?

Mali-C55 ISP 鍙笌鑳藉杈撳嚭 RGB 鏁版嵁鐨勪紶鎰熷櫒鍗忓悓宸ヤ綔銆傚湪杩欑鎯呭喌涓嬶紝铏界劧涓嶄細浣跨敤
浠讳綍鍥惧儚璐ㄩ噺鍧楋紝浣嗕粛鍙互鎸夊父瑙勬柟寮忚鍓?缂╂斁鏁版嵁銆傚洜姝わ紝杈撳叆 ISP 鐨?RGB 鏁版嵁
浠嶇劧缁忚繃 ISP 瀛愯澶囩殑琛灚 1 杩涘叆 resizer銆?
涓哄疄鐜拌繖涓€鐐癸紝ISP sink 琛灚鐨勬牸寮忚璁句负 MEDIA_BUS_FMT_RGB202020_1X60鈥斺€旇繖鍙嶆槧浜?鏁版嵁瑕佷笌 ISP 鍗忓悓宸ヤ綔鎵€蹇呴』鍏峰鐨勬牸寮忋€傚皢鐩告満浼犳劅鍣ㄧ殑杈撳嚭杞崲涓鸿鏍煎紡鏄閮ㄧ‖浠?鐨勮亴璐ｃ€?
鍦ㄦ湰绀轰緥涓紝鎴戜滑璁╂祴璇曞浘妗堝彂鐢熷櫒涓烘垜浠彁渚?RGB 鏁版嵁鑰岄潪 bayer 鏁版嵁锛?
    media-ctl -V "'mali-c55 tpg':0[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 isp':0[fmt:RGB202020_1X60/1920x1080]"

瑁佸壀鎴栫缉鏀炬暟鎹殑鏂瑰紡涓庡墠闈㈡墍杩板畬鍏ㄧ浉鍚屻€?
### Capturing raw data from the source and outputting it unmodified锛堥噰闆嗘簮绔殑 raw 鏁版嵁骞跺師鏍疯緭鍑猴級


ISP 杩樿兘浠ュ畬鍏ㄦ湭淇敼鐨勬柟寮忥紝浠呬粠婧愮閲囬泦 raw 鏁版嵁骞跺湪鍏ㄥ垎杈ㄧ巼娴佹按绾夸笂杈撳嚭銆傚湪杩欑
鎯呭喌涓嬶紝缂╂斁娴佹按绾夸粛鍙甯稿鐞嗘暟鎹紝骞朵笖鍙互鍚屾椂琚娇鐢ㄣ€?
瑕侀厤缃?raw 鏃佽矾锛岄渶瑕佸厛閰嶇疆 FR resizer 瀛愯澶囩殑璺敱琛紝鐒跺悗鍦ㄩ€傚綋浣嶇疆璁剧疆鏍煎紡锛?
    media-ctl -R "'mali-c55 resizer fr'[0/0->1/0[^0^],2/0->1/0[^1^]]"
    media-ctl -V "'mali-c55 isp':0[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 resizer fr':2[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB202020_1X60/1920x1080]"

    # Set format on the video device and stream
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video0


## Capturing ISP Statistics锛堥噰闆?ISP 缁熻淇℃伅锛?

ISP 鑳藉浜х敓缁熻淇℃伅锛屼緵杩愯鍦ㄧ敤鎴风┖闂寸殑鍥惧儚澶勭悊绠楁硶浣跨敤銆傝繖浜涚粺璁′俊鎭彲浠ュ湪
ISP 娴佸紡浼犺緭鏈熼棿锛岄€氳繃鍚?`mali-c55 3a stats` V4L2 璁惧鎺掗槦缂撳啿鍖烘潵閲囬泦銆備粎鏀寔
V4L2_META_FMT_MALI_C55_STATS <v4l2-meta-fmt-mali-c55-stats> 鏍煎紡锛屽洜姝ゆ棤闇€璁剧疆
鏍煎紡锛?
    # We assume the media graph has been configured to support RGB565 capture
    # from the mali-c55 fr V4L2 Device, which is at /dev/video0. The statistics
    # V4L2 device is at /dev/video3

    yavta -f RGB565 -s 1920x1080 -c32 /dev/video0 && \
    yavta -c10 -F /dev/video3

缂撳啿鍖虹殑甯冨眬鐢?`mali_c55_stats_buffer` 鎻忚堪锛屼絾澶т綋涓婄粺璁′俊鎭槸涓烘敮鎸佷笁绉嶅浘鍍?澶勭悊绠楁硶鑰岀敓鎴愮殑锛欰EXP锛堣嚜鍔ㄦ洕鍏夛級銆丄WB锛堣嚜鍔ㄧ櫧骞宠　锛変笌 AF锛堣嚜鍔ㄥ鐒︼級銆傝繖浜?缁熻淇℃伅鍙互鍙栬嚜 Mali-C55 ISP 娴佹按绾夸腑鐨勪笉鍚屼綅缃紝鍗虫墍璋撶殑鈥滄娊澶寸偣锛坱ap points锛夆€濄€?涓嬮潰杩欎釜楂樺眰妗嗗浘鏃ㄥ湪璇存槑杩欎簺缁熻淇℃伅鍦ㄤ綍澶勭敓鎴愶細
```

                  +--> AEXP-2            +----> AEXP-1          +--> AF-0
                  |                      +----> AF-1            |
                  |                      |                      |
      +---------+ |   +--------------+   |   +--------------+   |
      |  Input  +-+-->+ Digital Gain +---+-->+ Black Level  +---+---+
      +---------+     +--------------+       +--------------+       |
  +-----------------------------------------------------------------+
  |
  |   +--------------+ +---------+       +----------------+
  +-->| Sinter Noise +-+  White  +--+--->|  Lens Shading  +--+---------------+
      |   Reduction  | | Balance |  |    |                |  |               |
      +--------------+ +---------+  |    +----------------+  |               |
                                    +---> AEXP-0 (A)         +--> AEXP-0 (B) |
  +--------------------------------------------------------------------------+
  |
  |   +----------------+      +--------------+  +----------------+
  +-->|  Tone mapping  +-+--->| Demosaicing  +->+ Purple Fringe  +-+-----------+
      |                | |    +--------------+  |   Correction   | |           |
      +----------------+ +-> AEXP-IRIDIX        +----------------+ +---> AWB-0 |
  +----------------------------------------------------------------------------+
  |                    +-------------+        +-------------+
  +------------------->|   Colour    +---+--->|    Output   |
                       | Correction  |   |    |  Pipelines  |
                       +-------------+   |    +-------------+
                                         +-->  AWB-1

```
榛樿鎯呭喌涓嬶紝鎵€鏈夌粺璁′俊鎭兘鍙栬嚜姣忕绠楁硶鐨勭 0 涓娊澶寸偣锛涘嵆 AEXP 缁熻淇℃伅鏉ヨ嚜
AEXP-0 (A)锛孉WB 缁熻淇℃伅鏉ヨ嚜 AWB-0锛孉F 缁熻淇℃伅鏉ヨ嚜 AF-0銆傞€氳繃缂栫▼ ISP 鐨勫弬鏁帮紝
鍙 AEXP 涓?AWB 缁熻淇℃伅鐨勬娊澶寸偣杩涜閰嶇疆銆?

## Programming ISP Parameters锛堢紪绋?ISP 鍙傛暟锛?

ISP 鍙互浠庣敤鎴风┖闂翠互鍚勭鍙傛暟杩涜缂栫▼锛屼互渚垮湪瑙嗛娴佸紑濮嬪墠鍙婅繘琛屼腑搴旂敤鍒扮‖浠躲€?杩欎娇鐢ㄦ埛绌洪棿鑳藉鍔ㄦ€佹敼鍙樿濡傞粦鐢靛钩銆佺櫧骞宠　涓庨暅澶撮槾褰卞鐩婄瓑鏁板€笺€?
缂撳啿鍖烘牸寮忓強鍏跺～鍏呮柟寮忕敱 V4L2_META_FMT_MALI_C55_PARAMS <v4l2-meta-fmt-mali-c55-params>
鏍煎紡鎻忚堪锛屽簲灏嗗叾璁句负 `mali-c55 3a params` 瑙嗛鑺傜偣鐨勬暟鎹牸寮忋€?
## References锛堝弬鑰冭祫鏂欙級


