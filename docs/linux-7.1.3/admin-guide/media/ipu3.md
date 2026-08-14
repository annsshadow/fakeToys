

## Intel 鍥惧儚澶勭悊鍗曞厓 3锛圛PU3锛夋垚鍍忓崟鍏冿紙ImgU锛夐┍鍔?


Copyright |copy| 2018 Intel Corporation

## 绠€浠?


鏈枃妗ｈ鏄庝簡浣嶄簬 drivers/media/pci/intel/ipu3锛圕IO2锛変互鍙?drivers/staging/media/ipu3锛圛mgU锛変笅鐨?
Intel IPU3锛堢涓変唬鍥惧儚澶勭悊鍗曞厓锛夋垚鍍忓崟鍏冮┍鍔ㄣ€?

Intel IPU3 鍑虹幇鍦ㄦ煇浜?Kaby Lake锛堜互鍙婃煇浜?Sky Lake锛夊钩鍙帮紙U/Y 澶勭悊鍣ㄧ郴鍒楋級涓紝鐢变袱涓儴鍒嗙粍鎴愶紝
鍗虫垚鍍忓崟鍏冿紙ImgU锛夊拰 CIO2 璁惧锛圡IPI CSI2 鎺ユ敹鍣級銆?

CIO2 璁惧浠庝紶鎰熷櫒鎺ユ敹鍘熷 Bayer 鏁版嵁锛屽苟浠?IPU3 鐗规湁鐨勬牸寮忥紙渚?IPU3 ImgU 娑堣垂锛夎緭鍑哄抚銆?
CIO2 椹卞姩浣嶄簬 drivers/media/pci/intel/ipu3/ipu3-cio2*锛屽苟閫氳繃 CONFIG_VIDEO_IPU3_CIO2 閰嶇疆閫夐」鍚敤銆?

鎴愬儚鍗曞厓锛圛mgU锛夎礋璐ｅ鐞嗙敱 IPU3 CIO2 璁惧鎹曡幏鐨勫浘鍍忋€侷mgU 椹卞姩婧愮爜浣嶄簬 drivers/staging/media/ipu3
鐩綍銆傝椹卞姩閫氳繃 CONFIG_VIDEO_IPU3_IMGU 閰嶇疆閫夐」鍚敤銆?

涓や釜椹卞姩妯″潡鍒嗗埆鍚嶄负 ipu3_csi2 鍜?ipu3_imgu銆?

杩欎簺椹卞姩宸插湪 Kaby Lake 骞冲彴锛圲/Y 澶勭悊鍣ㄧ郴鍒楋級涓婅繘琛屼簡娴嬭瘯銆?

涓や釜椹卞姩鍧囧疄鐜颁簡 V4L2銆丮edia Controller 浠ュ強 V4L2 瀛愯澶囨帴鍙ｃ€侷PU3 CIO2 椹卞姩鏀寔閫氳繃 V4L2 瀛愯澶?
浼犳劅鍣ㄩ┍鍔ㄨ繛鎺ュ埌 CIO2 MIPI CSI-2 鎺ュ彛鐨勬憚鍍忓ご浼犳劅鍣ㄣ€?

## CIO2


CIO2 琛ㄧ幇涓轰竴涓?V4L2 瀛愯澶囷紝鍚戠敤鎴风┖闂存彁渚?V4L2 瀛愯澶囨帴鍙ｃ€傛瘡涓?CSI-2 鎺ユ敹鍣ㄩ兘鏈変竴涓棰戣妭鐐癸紝
鏁翠釜璁惧鏈変竴涓?Media Controller 鎺ュ彛銆?

CIO2 鍖呭惈鍥涗釜鐙珛鐨勬崟鑾烽€氶亾锛屾瘡涓€氶亾閮芥湁鑷繁鐙珛鐨?MIPI CSI-2 鎺ユ敹鍣ㄥ拰 DMA 寮曟搸銆傛瘡涓€氶亾琚缓妯′负
涓€涓?V4L2 瀛愯澶囷紝鍚戠敤鎴风┖闂存毚闇蹭负涓€涓?V4L2 瀛愯澶囪妭鐐癸紝骞舵湁涓や釜 pad锛?


    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - MIPI CSI-2 杈撳叆锛岃繛鎺ュ埌浼犳劅鍣ㄥ瓙璁惧

    - - 1
      - source
      - 鍘熷瑙嗛鎹曡幏锛岃繛鎺ュ埌 V4L2 瑙嗛鎺ュ彛

V4L2 瑙嗛鎺ュ彛瀵?DMA 寮曟搸杩涜寤烘ā銆傚畠浠互 V4L2 瑙嗛璁惧鑺傜偣褰㈠紡鍚戠敤鎴风┖闂存毚闇层€?

### 浠ュ師濮?Bayer 鏍煎紡鎹曡幏甯?


CIO2 MIPI CSI2 鎺ユ敹鍣ㄧ敤浜庝粠杩炴帴鍒?CSI2 绔彛鐨勫師濮嬩紶鎰熷櫒鎹曡幏甯э紙閲囩敤 packed 鍘熷 Bayer 鏍煎紡锛夈€?
鎹曡幏鐨勫抚浣滀负 ImgU 椹卞姩鐨勮緭鍏ャ€?

浣跨敤 IPU3 ImgU 杩涜鍥惧儚澶勭悊闇€瑕佽濡?raw2pnm [#f1]_ 鍜?yavta [#f2]_ 涔嬬被鐨勫伐鍏凤紝鍘熷洜鏄敱浜?IPU3
鐗规湁鐨勪互涓嬮渶姹傚強/鎴栫壒鎬с€?

-- IPU3 CSI2 鎺ユ敹鍣ㄤ互 IPU3 鐗规湁鐨?packed 鍘熷 Bayer 鏍煎紡浠庝紶鎰熷櫒杈撳嚭鎹曡幏鐨勫抚銆?

-- 蹇呴』鍚屾椂鎿嶄綔澶氫釜瑙嗛鑺傜偣銆?

璁╂垜浠互杩炴帴鍒?CSI2 绔彛 0 鐨?ov5670 浼犳劅鍣ㄤ负渚嬶紝杩涜 2592x1944 鐨勫浘鍍忔崟鑾枫€?

浣跨敤 Media Controller API锛屽皢 ov5670 浼犳劅鍣ㄩ厤缃负浠?packed 鍘熷 Bayer 鏍煎紡鍚?IPU3 CSI2 鎺ユ敹鍣ㄥ彂閫佸抚銆?


    # 鏈ず渚嬪亣璁?/dev/media0 涓?CIO2 濯掍綋璁惧
    export MDEV=/dev/media0

    # 骞跺亣璁?ov5670 浼犳劅鍣ㄨ繛鎺ュ埌 i2c 鎬荤嚎 10锛屽湴鍧€涓?0x36
    export SDEV=$(media-ctl -d $MDEV -e "ov5670 10-0036")

    # 浣跨敤 media-ctl 寤虹珛濯掍綋璁惧鐨勮繛鎺?
    media-ctl -d $MDEV -l "ov5670:0 -> ipu3-csi2 0:0[^1^]"

    # 璁剧疆濯掍綋璁惧鐨勬牸寮?
    media-ctl -d $MDEV -V "ov5670:0 [fmt:SGRBG10/2592x1944]"
    media-ctl -d $MDEV -V "ipu3-csi2 0:0 [fmt:SGRBG10/2592x1944]"
    media-ctl -d $MDEV -V "ipu3-csi2 0:1 [fmt:SGRBG10/2592x1944]"

濯掍綋绠￠亾閰嶇疆瀹屾垚鍚庯紝鍙互浣跨敤 yavta 宸ュ叿璁剧疆鎵€闇€鐨勪紶鎰熷櫒鐗瑰畾璁剧疆锛堜緥濡傛洕鍏夊拰澧炵泭璁剧疆锛夈€?

渚嬪


    yavta -w 0x009e0903 444 $SDEV
    yavta -w 0x009e0913 1024 $SDEV
    yavta -w 0x009e0911 2046 $SDEV

璁剧疆濂芥墍闇€鐨勪紶鎰熷櫒璁剧疆鍚庯紝鍗冲彲鎸夊涓嬫柟寮忔崟鑾峰抚銆?

渚嬪


    yavta --data-prefix -u -c10 -n5 -I -s2592x1944 --file=/tmp/frame-#.bin \
          -f IPU3_SGRBG10 $(media-ctl -d $MDEV -e "ipu3-cio2 0")

閫氳繃涓婅堪鍛戒护锛屼互 2592x1944 鍒嗚鲸鐜囥€乻GRBG10 鏍煎紡鎹曡幏 10 甯э紝骞朵互 IPU3_SGRBG10 鏍煎紡杈撳嚭銆?

鎹曡幏鐨勫抚浠?/tmp/frame-#.bin 鏂囦欢褰㈠紡鎻愪緵銆?

## ImgU


ImgU 琛ㄧ幇涓轰袱涓?V4L2 瀛愯澶囷紝姣忎釜閮藉悜鐢ㄦ埛绌洪棿鎻愪緵涓€涓?V4L2 瀛愯澶囨帴鍙ｃ€?

姣忎釜 V4L2 瀛愯澶囦唬琛ㄤ竴鏉＄閬擄紙pipe锛夛紝鏈€澶氬彲鏀寔 2 璺祦銆傝繖鏈夊姪浜庢敮鎸侀珮绾ф憚鍍忓ご鐗规€э紝渚嬪
杩炵画鍙栨櫙鍣紙CVF锛夊拰瑙嗛涓姄鎷嶏紙SDV锛夈€?

ImgU 鍖呭惈涓ゆ潯鐙珛鐨勭閬擄紝姣忔潯閮借寤烘ā涓轰竴涓?V4L2 瀛愯澶囷紝浠?V4L2 瀛愯澶囪妭鐐瑰舰寮忓悜鐢ㄦ埛绌洪棿鏆撮湶銆?

姣忔潯绠￠亾鏈変袱涓?sink pad 鍜屼笁涓?source pad锛岀敤閫斿涓嬶細


    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 杈撳叆鍘熷瑙嗛娴?

    - - 1
      - sink
      - 澶勭悊鍙傛暟

    - - 2
      - source
      - 杈撳嚭澶勭悊鍚庣殑瑙嗛娴?

    - - 3
      - source
      - 杈撳嚭鍙栨櫙鍣ㄨ棰戞祦

    - - 4
      - source
      - 3A 缁熻淇℃伅

姣忎釜 pad 閮借繛鎺ュ埌涓€涓浉搴旂殑 V4L2 瑙嗛鎺ュ彛锛屼互 V4L2 瑙嗛璁惧鑺傜偣褰㈠紡鍚戠敤鎴风┖闂存毚闇层€?

### 璁惧鎿嶄綔


瀵逛簬 ImgU锛屼竴鏃﹁緭鍏ヨ棰戣妭鐐癸紙"ipu3-imgu 0/1":0锛岄噰鐢?<entity>:<pad-number> 鏍煎紡锛夎濉叆缂撳啿鍖?
锛坧acked 鍘熷 Bayer 鏍煎紡锛夛紝ImgU 灏卞紑濮嬪鐞嗚缂撳啿鍖猴紝骞跺湪鍚勮嚜鐨勮緭鍑鸿妭鐐逛笂浠?YUV 鏍煎紡浜х敓瑙嗛杈撳嚭
浠ュ強缁熻淇℃伅杈撳嚭銆傚綋杈撳叆瑙嗛鑺傜偣琚～鍏ョ紦鍐插尯鏃讹紝椹卞姩搴斿綋宸蹭负鍙傛暟銆佽緭鍑哄拰缁熻淇℃伅鎵€鏈夎妭鐐瑰噯澶囧ソ缂撳啿鍖恒€?

鑷冲皯锛岃緭鍏ャ€佷富杈撳嚭銆?A 缁熻淇℃伅鍜屽彇鏅櫒瑙嗛鑺傜偣閮藉簲鍚敤锛孖PU3 鎵嶈兘寮€濮嬪浘鍍忓鐞嗐€?

姣忎釜 ImgU V4L2 瀛愯澶囧叿鏈変互涓嬩竴缁勮棰戣妭鐐广€?

### 杈撳叆銆佽緭鍑哄拰鍙栨櫙鍣ㄨ棰戣妭鐐?


杈撳叆瑙嗛鑺傜偣鏀跺埌鐨勫抚锛堥噰鐢?IPU3 鐗规湁鐨?packed 鍘熷 Bayer 鏍煎紡锛夌敱 IPU3 鎴愬儚鍗曞厓澶勭悊锛屽苟杈撳嚭鍒?
2 涓棰戣妭鐐癸紝姣忎釜闈㈠悜涓嶅悓鐢ㄩ€旓紙涓昏緭鍑哄拰鍙栨櫙鍣ㄨ緭鍑猴級銆?

鏈夊叧 IPU3 鐗规湁鐨?Bayer 鏍煎紡璇︽儏锛岃鍙傝 v4l2-pix-fmt-ipu3-sbggr10銆?

璇ラ┍鍔ㄦ敮鎸佸湪 devices 涓畾涔夌殑 V4L2 瑙嗛鎹曡幏鎺ュ彛銆?

浠呮敮鎸佸骞抽潰锛坢ulti-planar锛堿PI銆傛洿澶氳鎯呰鍙傝 planar-apis銆?

### 鍙傛暟瑙嗛鑺傜偣


鍙傛暟瑙嗛鑺傜偣鎺ユ敹鐢ㄤ簬閰嶇疆 ImgU 绠楁硶濡備綍澶勭悊鍥惧儚鐨?ImgU 绠楁硶鍙傛暟銆?

鏈夊叧 IPU3 鐗规湁鐨勫鐞嗗弬鏁拌鎯咃紝璇峰弬瑙?v4l2-meta-fmt-params銆?

### 3A 缁熻淇℃伅瑙嗛鑺傜偣


3A 缁熻淇℃伅瑙嗛鑺傜偣琚?ImgU 椹卞姩鐢ㄦ潵鍚戠敤鎴风┖闂村簲鐢ㄧ▼搴忚緭鍑烘鍦ㄨ ImgU 澶勭悊鐨勫抚鐨?3A
锛堣嚜鍔ㄥ鐒︺€佽嚜鍔ㄦ洕鍏夊拰鑷姩鐧藉钩琛★級缁熻淇℃伅銆傜敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ュ埄鐢ㄨ繖浜涚粺璁℃暟鎹绠?ImgU 鎵€闇€鐨?
绠楁硶鍙傛暟銆?

## 閰嶇疆 Intel IPU3


IPU3 ImgU 绠￠亾鍙互浣跨敤 Media Controller 閰嶇疆锛屽畾涔夎 media_controller銆?

### 杩愯妯″紡涓庡浐浠朵簩杩涘埗鏂囦欢閫夋嫨


ImgU 鍩轰簬鍥轰欢宸ヤ綔锛岀洰鍓?ImgU 鍥轰欢鏀寔浠ユ椂鍒嗘柟寮忚繍琛?2 鏉＄閬擄紝浣跨敤鍗曚釜杈撳叆甯ф暟鎹€傛瘡鏉＄閬撳彲浠ヨ繍琛屽湪
鐗瑰畾妯″紡 鈥斺€?"VIDEO" 鎴?"STILL"锛?VIDEO" 妯″紡閫氬父鐢ㄤ簬瑙嗛甯ф崟鑾凤紝"STILL" 鐢ㄤ簬闈欐€佸抚鎹曡幏銆備笉杩囷紝濡傛灉
甯屾湜浠ユ洿灏忕殑绯荤粺璐熻浇鍜屽姛鑰楁崟鑾峰浘鍍忥紝涔熷彲浠ラ€夋嫨 "VIDEO" 鏉ユ崟鑾烽潤鎬佸抚銆傚浜?"STILL" 妯″紡锛孖mgU 浼氬皾璇?
浣跨敤鏇村皬鐨?BDS 鍥犲瓙锛屽苟杈撳嚭姣?"VIDEO" 妯″紡鏇村ぇ鐨?bayer 甯х敤浜庡悗缁?YUV 澶勭悊锛屼互鑾峰緱楂樿川閲忓浘鍍忋€傛澶栵紝
"STILL" 妯″紡闇€瑕?XNR3 杩涜闄嶅櫔锛屽洜姝?"STILL" 妯″紡姣?"VIDEO" 妯″紡闇€瑕佹洿澶氱殑鍔熻€楀拰鍐呭瓨甯﹀銆俆NR 鍦?
"VIDEO" 妯″紡涓嬪惎鐢紝鍦?"STILL" 妯″紡涓嬭鏃佽矾銆侷mgU 榛樿浠?"VIDEO" 妯″紡杩愯锛岀敤鎴峰彲浠ヤ娇鐢?v4l2 鎺у埗
V4L2_CID_INTEL_IPU3_MODE锛堝綋鍓嶅畾涔変簬 drivers/staging/media/ipu3/include/uapi/intel-ipu3.h锛夋潵鏌ヨ鍜?
璁剧疆杩愯妯″紡銆傚浜庣敤鎴疯€岃█锛?VIDEO" 鍜?"STILL" 妯″紡鍦ㄧ紦鍐插尯鎺掗槦涓婃病鏈夊尯鍒紝蹇呴』鍚敤杈撳叆鍜屼富杈撳嚭鑺傜偣
骞舵帓闃熺紦鍐插尯锛岀粺璁′俊鎭拰鍙栨櫙鍣ㄩ槦鍒楁槸鍙€夌殑銆?

鍥轰欢浜岃繘鍒舵枃浠跺皢鏍规嵁褰撳墠杩愯妯″紡閫夋嫨锛屽鏋滀綘鍚敤 ImgU 鍔ㄦ€佽皟璇曪紝鍙互瑙傚療鍒拌濡?
"using binary if_to_osys_striped " 鎴?"using binary if_to_osys_primary_striped" 鐨勬棩蹇楋紝
浜岃繘鍒舵枃浠?if_to_osys_striped 鐢ㄤ簬 "VIDEO"锛岃€屼簩杩涘埗鏂囦欢 "if_to_osys_primary_striped" 鐢ㄤ簬 "STILL"銆?


### 浠ュ師濮?Bayer 鏍煎紡澶勭悊鍥惧儚


#### 閰嶇疆 ImgU V4L2 瀛愯澶囪繘琛屽浘鍍忓鐞?


ImgU V4L2 瀛愯澶囧繀椤讳娇鐢?Media Controller API 杩涜閰嶇疆锛屼互姝ｇ‘寤虹珛鎵€鏈夎棰戣妭鐐广€?

璁╂垜浠互 "ipu3-imgu 0" 瀛愯澶囦负渚嬨€?


    media-ctl -d $MDEV -r
    media-ctl -d $MDEV -l "ipu3-imgu 0 input":0 -> "ipu3-imgu 0":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":2 -> "ipu3-imgu 0 output":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":3 -> "ipu3-imgu 0 viewfinder":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":4 -> "ipu3-imgu 0 3a stat":0[^1^]

姝ゅ锛岀浉搴?V4L2 瀛愯澶囩殑绠￠亾妯″紡搴旀寜闇€璁剧疆锛堜緥濡?0 琛ㄧず瑙嗛妯″紡锛? 琛ㄧず闈欐€佹ā寮忥級锛岄€氳繃鎺у埗 id 0x009819a1锛?
濡備笅鎵€绀恒€?


    yavta -w "0x009819A1 1" /dev/v4l-subdev7

ImgU 绠￠亾涓殑鏌愪簺纭欢妯″潡鍙互閫氳繃瑁佸壀鎴栫缉鏀炬敼鍙樺抚鍒嗚鲸鐜囷紝杩欎簺纭欢妯″潡鍖呮嫭杈撳叆棣堥€佸櫒锛圛F锛夈€丅ayer
缂╁皬鍣紙BDS锛夊拰鍑犱綍鐣稿彉鏍℃锛圙DC锛夈€傝繕鏈変竴涓彲浠ユ敼鍙樺抚鍒嗚鲸鐜囩殑妯″潡 鈥斺€?YUV 缂╂斁鍣紝瀹冧粎閫傜敤浜庢绾ц緭鍑恒€?

鍘熷 Bayer 甯х粡杩囪繖浜?ImgU 绠￠亾纭欢妯″潡锛屾渶缁堝鐞嗗悗鐨勫浘鍍忚緭鍑哄埌 DDR 鍐呭瓨銆?

   :alt: ipu3 resolution blocks image

   IPU3 鍒嗚鲸鐜囨敼鍙樼‖浠舵ā鍧?

**Input Feeder锛堣緭鍏ラ閫佸櫒锛?*

杈撳叆棣堥€佸櫒浠庝紶鎰熷櫒鑾峰彇 Bayer 甯ф暟鎹紝瀹冨彲浠ュ甯т腑鐨勮鍜屽垪杩涜瑁佸壀锛岀劧鍚庡皢鍍忕礌瀛樺叆璁惧鐨勫唴閮ㄥ儚绱犵紦鍐插尯锛?
渚涘悗缁ā鍧楄鍑恒€?

**Bayer Down Scaler锛圔ayer 缂╁皬鍣級**

Bayer 缂╁皬鍣ㄨ兘澶熷湪 Bayer 鍩熸墽琛屽浘鍍忕缉鏀撅紝缂╁皬鍥犲瓙鍙湪姣忎釜杞翠笂浠?1X 閰嶇疆鍒?1/4X锛岄厤缃闀夸负
0.03125锛?/32锛夈€?

**Geometric Distortion Correction锛堝嚑浣曠暩鍙樻牎姝ｏ級**

鍑犱綍鐣稿彉鏍℃鐢ㄤ簬鎵ц鐣稿彉鏍℃鍜屽浘鍍忔护娉€傚畠闇€瑕佷竴浜涢澶栫殑婊ゆ尝鍣ㄥ拰鍖呯粶濉厖鍍忕礌鎵嶈兘宸ヤ綔锛屽洜姝?GDC 鐨勮緭鍏?
鍒嗚鲸鐜囧簲澶т簬杈撳嚭鍒嗚鲸鐜囥€?

**YUV Scaler锛圷UV 缂╂斁鍣級**

YUV 缂╂斁鍣ㄤ笌 BDS 绫讳技锛屼絾涓昏鍦?YUV 鍩熻繘琛屽浘鍍忕缉灏忥紝瀹冩渶澶氭敮鎸?1/12X 缂╁皬锛屼絾涓嶈兘搴旂敤浜庝富杈撳嚭銆?

瀵逛簬缁欏畾鐨勮緭鍏ュ垎杈ㄧ巼锛孖mgU V4L2 瀛愯澶囧繀椤诲湪涓婅堪鎵€鏈夌‖浠舵ā鍧椾腑閰嶇疆鍙楁敮鎸佺殑鍒嗚鲸鐜囥€傚浜庣粰瀹氱殑杈撳叆甯?
鍙楁敮鎸佺殑鍒嗚鲸鐜囷紝杈撳叆棣堥€佸櫒銆丅ayer 缂╁皬鍣ㄥ拰 GDC 妯″潡閮藉簲閰嶇疆涓哄彈鏀寔鐨勫垎杈ㄧ巼锛屽洜涓烘瘡涓‖浠舵ā鍧楅兘鏈?
鑷繁鐨勫榻愯姹傘€?

浣犲繀椤诲阀濡欏湴閰嶇疆纭欢妯″潡鐨勮緭鍑哄垎杈ㄧ巼锛屾棦婊¤冻纭欢瑕佹眰锛屽張淇濇寔鏈€澶х殑瑙嗗満銆備腑闂村垎杈ㄧ巼鍙互鐢辩壒瀹氬伐鍏风敓鎴?鈥斺€?

https://github.com/intel/intel-ipu3-pipecfg

璇ュ伐鍏峰彲鐢ㄤ簬鐢熸垚涓棿鍒嗚鲸鐜囥€傛洿澶氫俊鎭彲閫氳繃鏌ョ湅浠ヤ笅 IPU3 ImgU 閰嶇疆琛ㄨ幏寰椼€?

https://chromium.googlesource.com/chromiumos/overlays/board-overlays/+/master

鍦?baseboard-poppy/media-libs/cros-camera-hal-configs-poppy/files/gcss 鐩綍涓嬶紝
graph_settings_ov5670.xml 鍙綔涓虹ず渚嬨€?

浠ヤ笅姝ラ涓?ImgU 绠￠亾鍑嗗鍥惧儚澶勭悊銆?

1. 搴斾娇鐢?GDC 鑾峰緱鐨勫搴﹀拰楂樺害锛屽湪 pad 0 涓婇€氳繃 VIDIOC_SUBDEV_S_FMT 璁剧疆 ImgU V4L2 瀛愯澶囨暟鎹牸寮忋€?

2. 搴斿湪 pad 0 涓婇€氳繃 VIDIOC_SUBDEV_S_SELECTION 璁剧疆 ImgU V4L2 瀛愯澶囩殑瑁佸壀锛岀洰鏍囦负 V4L2_SEL_TGT_CROP锛?
浣跨敤杈撳叆棣堥€佸櫒鐨勯珮搴﹀拰瀹藉害銆?

3. 搴斿湪 pad 0 涓婇€氳繃 VIDIOC_SUBDEV_S_SELECTION 璁剧疆 ImgU V4L2 瀛愯澶囩殑鍚堟垚锛岀洰鏍囦负 V4L2_SEL_TGT_COMPOSE锛?
浣跨敤 BDS 鐨勯珮搴﹀拰瀹藉害銆?

浠?ov5670 涓轰緥锛屽浜庡垎杈ㄧ巼涓?2592x1944锛堣緭鍏ュ埌 ImgU 瀛愯澶?pad 0锛夌殑杈撳叆甯э紝杈撳叆棣堥€佸櫒銆丅DS 鍜?GDC 鐨?
鐩稿簲鍒嗚鲸鐜囧垎鍒负 2592x1944銆?592x1944 鍜?2560x1920銆?

瀹屾垚涓婅堪姝ラ鍚庯紝鍙互浣跨敤濡備笅鏂瑰紡灏嗘帴鏀跺埌鐨勫師濮?Bayer 甯ц緭鍏ュ埌 ImgU V4L2 瀛愯澶囷紝浣跨敤寮€婧愬簲鐢ㄧ▼搴?v4l2n [#f1]_銆?

瀵逛簬浠?2592x1944 [#f4]_ 鍒嗚鲸鐜囨崟鑾枫€佹湡鏈涜緭鍑哄垎杈ㄧ巼涓?2560x1920銆佸彇鏅櫒鍒嗚鲸鐜囦负 2560x1920 鐨勫浘鍍忥紝
鍙互浣跨敤浠ヤ笅 v4l2n 鍛戒护銆傝繖鏈夊姪浜庡鐞嗗師濮?Bayer 甯э紝骞朵互 NV12 鏍煎紡浜х敓涓昏緭鍑哄浘鍍忓拰鍙栨櫙鍣ㄨ緭鍑虹殑鏈熸湜缁撴灉銆?


    v4l2n --pipe=4 --load=/tmp/frame-#.bin --open=/dev/video4
          --fmt=type:VIDEO_OUTPUT_MPLANE,width=2592,height=1944,pixelformat=0X47337069 \
          --reqbufs=type:VIDEO_OUTPUT_MPLANE,count:1 --pipe=1 \
          --output=/tmp/frames.out --open=/dev/video5 \
          --fmt=type:VIDEO_CAPTURE_MPLANE,width=2560,height=1920,pixelformat=NV12 \
          --reqbufs=type:VIDEO_CAPTURE_MPLANE,count:1 --pipe=2 \
          --output=/tmp/frames.vf --open=/dev/video6 \
          --fmt=type:VIDEO_CAPTURE_MPLANE,width=2560,height=1920,pixelformat=NV12 \
          --reqbufs=type:VIDEO_CAPTURE_MPLANE,count:1 --pipe=3 --open=/dev/video7 \
          --output=/tmp/frames.3A --fmt=type:META_CAPTURE,? \
          --reqbufs=count:1,type:META_CAPTURE --pipe=1,2,3,4 --stream=5

浣犱篃鍙互浣跨敤 yavta [#f2]_ 鍛戒护瀹屾垚涓庝笂杩扮浉鍚岀殑鎿嶄綔锛?


    yavta --data-prefix -Bcapture-mplane -c10 -n5 -I -s2592x1944 \
          --file=frame-#.out-f NV12 /dev/video5 & \
    yavta --data-prefix -Bcapture-mplane -c10 -n5 -I -s2592x1944 \
          --file=frame-#.vf -f NV12 /dev/video6 & \
    yavta --data-prefix -Bmeta-capture -c10 -n5 -I \
          --file=frame-#.3a /dev/video7 & \
    yavta --data-prefix -Boutput-mplane -c10 -n5 -I -s2592x1944 \
          --file=/tmp/frame-in.cio2 -f IPU3_SGRBG10 /dev/video4

鍏朵腑 /dev/video4銆?dev/video5銆?dev/video6 鍜?/dev/video7 璁惧鍒嗗埆鎸囧悜杈撳叆銆佽緭鍑恒€佸彇鏅櫒鍜?
3A 缁熻淇℃伅瑙嗛鑺傜偣銆?

### 灏嗗師濮?Bayer 鍥惧儚杞崲鍒?YUV 鍩?


涓婅堪姝ラ澶勭悊鍚庣殑鍥惧儚鍙互濡備笅鏂瑰紡杞崲鍒?YUV 鍩熴€?

#### 涓昏緭鍑哄抚



    raw2pnm -x2560 -y1920 -fNV12 /tmp/frames.out /tmp/frames.out.ppm

鍏朵腑 2560x1920 涓鸿緭鍑哄垎杈ㄧ巼锛孨V12 涓鸿棰戞牸寮忥紝鍏跺悗涓鸿緭鍏ュ抚鍜岃緭鍑?PNM 鏂囦欢銆?

#### 鍙栨櫙鍣ㄨ緭鍑哄抚



    raw2pnm -x2560 -y1920 -fNV12 /tmp/frames.vf /tmp/frames.vf.ppm

鍏朵腑 2560x1920 涓鸿緭鍑哄垎杈ㄧ巼锛孨V12 涓鸿棰戞牸寮忥紝鍏跺悗涓鸿緭鍏ュ抚鍜岃緭鍑?PNM 鏂囦欢銆?

## IPU3 鐨勭敤鎴风┖闂寸ず渚嬩唬鐮?


閰嶇疆骞朵娇鐢?IPU3 鐨勭敤鎴风┖闂翠唬鐮佸彲鍦ㄦ澶勮幏鍙栥€?

https://chromium.googlesource.com/chromiumos/platform/arc-camera/+/master/

婧愮爜浣嶄簬 hal/intel 鐩綍涓嬨€?

## IPU3 绠￠亾姒傝堪


IPU3 绠￠亾鏈夊涓浘鍍忓鐞嗛樁娈碉紝姣忎釜闃舵鎺ユ敹涓€缁勫弬鏁颁綔涓鸿緭鍏ャ€傜閬撶殑涓昏闃舵濡備笅鎵€绀猴細

   :alt: IPU3 ImgU Pipeline
   :caption: IPU3 ImgU Pipeline Diagram

   digraph "IPU3 ImgU" {
       node [shape=box]
       splines="ortho"
       rankdir="LR"

       a [label="Raw pixels"]
       b [label="Bayer Downscaling"]
       c [label="Optical Black Correction"]
       d [label="Linearization"]
       e [label="Lens Shading Correction"]
       f [label="White Balance / Exposure / Focus Apply"]
       g [label="Bayer Noise Reduction"]
       h [label="ANR"]
       i [label="Demosaicing"]
       j [label="Color Correction Matrix"]
       k [label="Gamma correction"]
       l [label="Color Space Conversion"]
       m [label="Chroma Down Scaling"]
       n [label="Chromatic Noise Reduction"]
       o [label="Total Color Correction"]
       p [label="XNR3"]
       q [label="TNR"]
       r [label="DDR", style=filled, fillcolor=yellow, shape=cylinder]
       s [label="YUV Downscaling"]
       t [label="DDR", style=filled, fillcolor=yellow, shape=cylinder]

       { rank=same; a -> b -> c -> d -> e -> f -> g -> h -> i }
       { rank=same; j -> k -> l -> m -> n -> o -> p -> q -> s -> t}

       a -> j [style=invis, weight=10]
       i -> j
       q -> r
   }

涓嬭〃缁欏嚭浜嗕笂杩扮畻娉曠殑鎻忚堪銆?

======================== =======================================================
Name			 Description
======================== =======================================================
Optical Black Correction Optical Black Correction 妯″潡浠庣浉搴旂殑鍍忕礌鍊间腑鍑忓幓涓€涓瀹氫箟
			 鐨勫€硷紝浠ヨ幏寰楁洿濂界殑鍥惧儚璐ㄩ噺銆?
			 瀹氫箟浜?struct ipu3_uapi_obgrid_param銆?
Linearization		 Linearization 绠楁硶鍧椾娇鐢ㄧ嚎鎬у寲鍙傛暟鏉ヨВ鍐充紶鎰熷櫒闈炵嚎鎬ф晥搴斻€?
			 鏌ユ壘琛ㄥ畾涔変簬
			 struct ipu3_uapi_isp_lin_vmem_params銆?
SHD			 Lens shading correction 鐢ㄤ簬鏍℃鐢变簬鍏夊闀滃ご闃村奖瀵艰嚧鐨?
			 鍍忕礌鍝嶅簲鐨勭┖闂翠笉鍧囧寑鎬с€傝繖鏄€氳繃瀵规瘡涓儚绱犲簲鐢ㄤ笉鍚岀殑澧炵泭
			 鏉ュ疄鐜扮殑銆傚鐩娿€侀粦鐢靛钩绛夊湪
			 struct ipu3_uapi_shd_config_static 涓厤缃€?
BNR			 Bayer 闄嶅櫔妯″潡閫氳繃搴旂敤鍙岃竟婊ゆ尝鍣ㄦ潵鍘婚櫎鍥惧儚鍣０銆?
			 璇﹁ struct ipu3_uapi_bnr_static_config銆?
ANR			 Advanced Noise Reduction 鏄竴绉嶅熀浜庡潡鐨勭畻娉曪紝鍦?Bayer 鍩?
			 鎵ц闄嶅櫔銆傚嵎绉煩闃电瓑鍙湪
			 struct ipu3_uapi_anr_config 涓壘鍒般€?
DM			 Demosaicing 灏?Bayer 鏍煎紡鐨勫師濮嬩紶鎰熷櫒鏁版嵁杞崲涓?
			 RGB锛堢孩銆佺豢銆佽摑锛夎〃绀恒€傜劧鍚庝负鍚庣画鐢卞浐浠惰繘琛岀殑娴佸鐞?
			 娣诲姞 Y 閫氶亾鐨勪及璁¤緭鍑恒€傝缁撴瀯浣撳畾涔変负
			 struct ipu3_uapi_dm_config銆?
Color Correction	 Color Correction 绠楁硶灏嗕紶鎰熷櫒鐗瑰畾鐨勮壊褰╃┖闂磋浆鎹负鏍囧噯鐨?
			 "sRGB" 鑹插僵绌洪棿銆傝繖鏄€氳繃搴旂敤瀹氫箟浜?
			 struct ipu3_uapi_ccm_mat_config 鐨?3x3 鐭╅樀瀹炵幇鐨勩€?
Gamma correction	 Gamma correction 缁撴瀯浣?struct ipu3_uapi_gamma_config 鏄竴绉?
			 鍩烘湰鐨勯潪绾挎€ц壊璋冩槧灏勬牎姝ｏ紝瀵规瘡涓儚绱犵殑姣忎釜鍒嗛噺閫愬儚绱犲簲鐢ㄣ€?
CSC			 Color space conversion 灏嗘瘡涓儚绱犱粠 RGB 鍘熻壊琛ㄧず杞崲涓?
			 YUV锛圷锛氫寒搴︼紝UV锛氳壊搴︼級琛ㄧず銆傝繖鏄€氳繃搴旂敤瀹氫箟浜?
			 struct ipu3_uapi_csc_mat_config 鐨?3x3 鐭╅樀瀹炵幇鐨勩€?
CDS			 Chroma down sampling锛堣壊搴︿笅閲囨牱锛?
			 CSC 鎵ц鍚庯紝搴旂敤 Chroma Down Sampling 瀵?UV 骞抽潰杩涜
			 涓嬮噰鏍凤紝瀵逛簬 YUV 4:2:0锛屾瘡涓柟鍚戞寜鍥犲瓙 2 浣跨敤 4x2 鐨?
			 鍙厤缃护娉㈠櫒 struct ipu3_uapi_cds_params銆?
CHNR			 Chroma noise reduction锛堣壊搴﹂檷鍣級
			 璇ユā鍧椾粎澶勭悊鑹插害鍍忕礌锛屽苟閫氳繃娓呴櫎楂橀鍣０鏉ユ墽琛岄檷鍣€?
			 鍙傝 struct struct ipu3_uapi_yuvp1_chnr_config銆?
TCC			 Total color correction锛屽畾涔変簬缁撴瀯浣?
			 struct ipu3_uapi_yuvp2_tcc_static_config銆?
XNR3			 eXtreme Noise Reduction V3 鏄涓変唬闄嶅櫔绠楁硶锛岀敤浜庢敼鍠?
			 鍥惧儚璐ㄩ噺銆傚畠鍘婚櫎鎵€鎹曡幏鍥惧儚涓殑浣庨鍣０銆傚畾涔変簡涓や釜鐩稿叧
			 缁撴瀯浣擄細鐢ㄤ簬 ISP 鏁版嵁鍐呭瓨鐨?struct ipu3_uapi_isp_xnr3_params
			 鍜岀敤浜庡悜閲忓唴瀛樼殑 struct ipu3_uapi_isp_xnr3_vmem_params銆?
TNR			 Temporal Noise Reduction 妯″潡姣旇緝鏃堕棿涓婅繛缁殑甯э紝浠ュ幓闄?
			 鍍忕礌鍊间腑鐨勫紓甯?鍣０銆備负 ISP 鍚戦噺鍜屾暟鎹唴瀛樺垎鍒畾涔変簡
			 struct ipu3_uapi_isp_tnr3_vmem_params 鍜?
			 struct ipu3_uapi_isp_tnr3_params銆?
======================== =======================================================

涓婅〃鏈垪鍑虹殑鍏朵粬甯歌缂╁啓锛?

	ACC
		Accelerator cluster锛堝姞閫熼泦缇わ級
	AWB_FR
		Auto white balance filter response statistics锛堣嚜鍔ㄧ櫧骞宠　婊ゆ尝鍝嶅簲缁熻锛?
	BDS
		Bayer downscaler parameters锛圔ayer 缂╁皬鍣ㄥ弬鏁帮級
	CCM
		Color correction matrix coefficients锛堣壊褰╂牎姝ｇ煩闃电郴鏁帮級
	IEFd
		Image enhancement filter directed锛堝浘鍍忓寮烘护娉㈠畾鍚戯級
	Obgrid
		Optical black level compensation锛堝厜瀛﹂粦鐢靛钩琛ュ伩锛?
	OSYS
		Output system configuration锛堣緭鍑虹郴缁熼厤缃級
	ROI
		Region of interest锛堟劅鍏磋叮鍖哄煙锛?
	YDS
		Y down sampling锛圷 涓嬮噰鏍凤級
	YTM
		Y-tone mapping锛圷 鑹茶皟鏄犲皠锛?

绠￠亾鐨勪竴浜涢樁娈靛皢鐢辫繍琛屽湪 ISP 澶勭悊鍣ㄤ笂鐨勫浐浠舵墽琛岋紝鑰岃澶氬叾浠栭樁娈靛皢浣跨敤涓€缁勫浐瀹氱殑纭欢妯″潡锛堜篃绉颁负
鍔犻€熼泦缇わ紝ACC锛夋潵澶勭悊鍍忕礌鏁版嵁骞剁敓鎴愮粺璁′俊鎭€?

鍚勪釜绠楁硶鐨?ACC 鍙傛暟锛堢敱 struct ipu3_uapi_acc_param 瀹氫箟锛夊彲琚€夋嫨鐢辩敤鎴风┖闂撮€氳繃宓屽叆鍦?
struct ipu3_uapi_params 缁撴瀯浣撲腑鐨?struct struct ipu3_uapi_flags 鏉ュ簲鐢ㄣ€傚浜庤鐢ㄦ埛绌洪棿閰嶇疆涓?
鏈惎鐢ㄧ殑鍙傛暟锛岀浉搴旂殑缁撴瀯浣撳皢琚┍鍔ㄥ拷鐣ワ紝鍦ㄨ繖绉嶆儏鍐典笅锛岃绠楁硶鐨勭幇鏈夐厤缃皢琚繚鐣欍€?

## 鍙傝€冭祫鏂?




