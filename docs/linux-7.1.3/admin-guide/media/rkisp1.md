
## Rockchip 鍥惧儚淇″彿澶勭悊鍣?(rkisp1)


## 绠€浠?

鏈枃浠惰褰曚簡 Rockchip ISP1 椹卞姩锛岃椹卞姩鏄?RK3288 鍜?RK3399 SoC 鐨勪竴閮ㄥ垎銆?椹卞姩浣嶄簬 drivers/media/platform/rockchip/rkisp1锛屼娇鐢?Media-Controller API銆?
## 鐗堟湰


璇?ISP 瀛樺湪澶氫釜鍦ㄥ悗缁?SoC 涓紩鍏ョ殑杈冨皬鐗堟湰銆傚悇鐗堟湰鍙湪 UAPI 涓殑鏋氫妇
`rkisp1_cif_isp_version` 涓壘鍒帮紝鑰岃繍琛屼腑鐨?SoC 鍐呴儴璇?ISP 鐨勭増鏈彲閫氳繃
ioctl MEDIA_IOC_DEVICE_INFO 杩斿洖鐨?struct media_device_info 鐨?hw_revision
瀛楁璇诲彇銆?
鍦ㄧ敤鐨勭増鏈湁锛?
- RKISP1_V10锛氳嚦灏戠敤浜?rk3288 鍜?rk3399
- RKISP1_V11锛氬湪鍘熷鍘傚晢浠ｇ爜涓０鏄庯紝浣嗘湭琚娇鐢?- RKISP1_V12锛氳嚦灏戠敤浜?rk3326 鍜?px30
- RKISP1_V13锛氳嚦灏戠敤浜?rk1808

## 鎷撴墤


    :alt:   Diagram of the default media pipeline topology
    :align: center


璇ラ┍鍔ㄥ寘鍚?4 涓棰戣澶囷細

- rkisp1_mainpath锛氱敤浜庤幏鍙栧浘鍍忥紙閫氬父鍒嗚鲸鐜囪緝楂橈級鐨勯噰闆嗚澶囥€?- rkisp1_selfpath锛氱敤浜庤幏鍙栧浘鍍忕殑閲囬泦璁惧銆?- rkisp1_stats锛氬彂閫佺粺璁′俊鎭殑鍏冩暟鎹紙metadata锛夐噰闆嗚澶囥€?- rkisp1_params锛氫粠鐢ㄦ埛绌洪棿鎺ユ敹鍙傛暟閰嶇疆鐨勫厓鏁版嵁杈撳嚭璁惧銆?
璇ラ┍鍔ㄥ寘鍚?3 涓瓙璁惧锛?
- rkisp1_resizer_mainpath锛氱敤浜庝负 mainpath 閲囬泦璁惧缂╂斁鍜岄檷閲囨牱甯с€?- rkisp1_resizer_selfpath锛氱敤浜庝负 selfpath 閲囬泦璁惧缂╂斁鍜岄檷閲囨牱甯с€?- rkisp1_isp锛氳繛鎺ュ埌浼犳劅鍣紝璐熻矗鎵€鏈?isp 鎿嶄綔銆?

### rkisp1_mainpath銆乺kisp1_selfpath 鈥?甯ч噰闆嗚棰戣妭鐐?
杩欎簺鏄?`mainpath` 鍜?`selfpath` 閲囬泦璁惧锛岀敤浜庨噰闆嗗抚銆傝繖浜涘疄浣撴槸灏嗗抚鍐欏叆
鍐呭瓨鐨?DMA 寮曟搸銆俿elfpath 瑙嗛璁惧鍙噰闆?YUV/RGB 鏍煎紡銆傚叾杈撳叆涓?YUV 缂栫爜
鐮佹祦锛屽苟鑳藉皢鍏惰浆鎹负 RGB銆俿elfpath 鏃犳硶閲囬泦 bayer 鏍煎紡銆?mainpath 鍙噰闆?bayer 鍜?YUV 鏍煎紡锛屼絾鏃犳硶閲囬泦 RGB 鏍煎紡銆?涓や釜閲囬泦瑙嗛璁惧鍧囨敮鎸?`V4L2_CAP_IO_MC` 鑳藉姏 <device-capabilities>銆?

### rkisp1_resizer_mainpath銆乺kisp1_resizer_selfpath 鈥?缂╂斁鍣ㄥ瓙璁惧鑺傜偣

杩欎簺鏄?mainpath 鍜?selfpath 鐨勭缉鏀惧櫒瀹炰綋銆傝繖浜涘疄浣撳彲浠ュ皢甯ф斁澶у拰缂╁皬锛屽苟
鏇存敼 YUV 閲囨牱锛堜緥濡?YUV4:2:2 -> YUV4:2:0锛夈€傚畠浠湪 sink pad 涓婅繕鍏锋湁瑁佸壀
鑳藉姏銆傜缉鏀惧櫒瀹炰綋鍙兘浠?YUV:4:2:2 鏍煎紡
锛圡EDIA_BUS_FMT_YUYV8_2X8锛夊伐浣溿€?mainpath 閲囬泦璁惧鏀寔閲囬泦 bayer 鏍煎紡鐨勮棰戙€傝繖绉嶆儏鍐典笅锛宮ainpath 鐨勭缉鏀惧櫒
琚涓?'bypass'锛堟梺璺級妯″紡鈥斺€斿嵆鐩存帴杞彂甯ц€屼笉瀵瑰叾鍋氬鐞嗐€?
### rkisp1_isp 鈥?鍥惧儚淇″彿澶勭悊瀛愯澶囪妭鐐?
杩欐槸 isp 瀹炰綋銆傚畠閫氳繃 sink pad 0 杩炴帴鍒颁紶鎰熷櫒锛屽苟浣跨敤 CSI-2 鍗忚鎺ユ敹甯с€?瀹冭礋璐ｉ厤缃?CSI-2 鍗忚銆傚畠鍦ㄨ繛鎺ュ埌浼犳劅鍣ㄧ殑 sink pad 0 涓婏紝浠ュ強杩炴帴鍒扮缉鏀惧櫒
瀹炰綋鐨?source pad 2 涓婂叿鏈夎鍓兘鍔涖€?sink pad 0 涓婄殑瑁佸壀瀹氫箟浜嗘潵鑷紶鎰熷櫒鐨勫浘鍍忓尯鍩熴€?source pad 2 涓婄殑瑁佸壀瀹氫箟浜嗗浘鍍忕ǔ瀹氬櫒锛圛S锛夌殑鍖哄煙銆?

### rkisp1_stats 鈥?缁熻瑙嗛鑺傜偣

缁熻瑙嗛鑺傜偣杈撳嚭 3A锛堣嚜鍔ㄥ鐒︺€佽嚜鍔ㄦ洕鍏夊拰鑷姩鐧藉钩琛★級缁熻淇℃伅锛屼互鍙婃鐢?rkisp1 澶勭悊銆侀潰鍚戠敤鎴风┖闂村簲鐢ㄧ▼搴忕殑甯х殑鐩存柟鍥剧粺璁°€?鍒╃敤杩欎簺鏁版嵁锛屽簲鐢ㄧ▼搴忓彲浠ュ疄鐜扮畻娉曪紝骞堕€氳繃 rkisp_params 鑺傜偣閲嶆柊閰嶇疆椹卞姩锛?浠ュ湪瑙嗛娴佽繃绋嬩腑鏀瑰杽鍥惧儚璐ㄩ噺銆?缂撳啿鍖烘牸寮忕敱 struct `rkisp1_stat_buffer` 瀹氫箟锛岀敤鎴风┖闂村簲灏?V4L2_META_FMT_RK_ISP1_STAT_3A <v4l2-meta-fmt-rk-isp1-stat-3a> 璁句负
鏁版嵁鏍煎紡锛坉ataformat锛夈€?

### rkisp1_params 鈥?鍙傛暟瑙嗛鑺傜偣

rkisp1_params 瑙嗛鑺傜偣浠庣敤鎴风┖闂存帴鏀朵竴缁勫弬鏁帮紝鍦ㄨ棰戞祦杩囩▼涓簲鐢ㄥ埌纭欢锛?鍏佽鐢ㄦ埛绌洪棿鍔ㄦ€佷慨鏀归粦鐢靛钩銆佷覆鎵版牎姝ｇ瓑鏁板€笺€?
璇?ISP 椹卞姩鏀寔涓ょ涓嶅悓鐨勫弬鏁伴厤缃柟娉曪細`fixed parameters format`锛堝浐瀹氬弬鏁版牸寮忥級
鎴?`extensible parameters format`锛堝彲鎵╁睍鍙傛暟鏍煎紡锛夈€?
浣跨敤 `fixed parameters`锛堝浐瀹氬弬鏁帮級鏂规硶鏃讹紝缂撳啿鍖烘牸寮忕敱 struct
`rkisp1_params_cfg` 瀹氫箟锛岀敤鎴风┖闂村簲灏?V4L2_META_FMT_RK_ISP1_PARAMS <v4l2-meta-fmt-rk-isp1-params> 璁句负
鏁版嵁鏍煎紡銆?
浣跨敤 `extensible parameters`锛堝彲鎵╁睍鍙傛暟锛夋柟娉曟椂锛岀紦鍐插尯鏍煎紡鐢?struct
`rkisp1_ext_params_cfg` 瀹氫箟锛岀敤鎴风┖闂村簲灏?V4L2_META_FMT_RK_ISP1_EXT_PARAMS <v4l2-meta-fmt-rk-isp1-ext-params> 璁句负
鏁版嵁鏍煎紡銆?
## 閲囬泦瑙嗛甯хず渚?

鍦ㄤ笅闈㈢殑绀轰緥涓紝杩炴帴鍒?'rkisp1_isp' 鐨?pad 0 鐨勪紶鎰熷櫒鏄?imx219銆?
浠ヤ笅鍛戒护鍙敤浜庝粠 selfpath 瑙嗛鑺傜偣閲囬泦灏哄涓?900x800銆佸钩闈㈡牸寮?YUV 4:2:2
鐨勮棰戙€傚畠浣跨敤浜嗘墍鏈夊彲鐢ㄧ殑瑁佸壀鑳藉姏锛堣鏄庤涓嬫枃锛夈€?

	# set the links
	"media-ctl" "-d" "platform:rkisp1" "-r"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'imx219 4-0010':0 -> 'rkisp1_isp':0 [^1^]"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'rkisp1_isp':2 -> 'rkisp1_resizer_selfpath':0 [^1^]"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'rkisp1_isp':2 -> 'rkisp1_resizer_mainpath':0 [^0^]"

	# set format for imx219 4-0010:0
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"imx219 4-0010":0 [fmt:SRGGB10_1X10/1640x1232]'

	# set format for rkisp1_isp pads:
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_isp":0 [fmt:SRGGB10_1X10/1640x1232 crop: (0,0)/1600x1200]'
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_isp":2 [fmt:YUYV8_2X8/1600x1200 crop: (0,0)/1500x1100]'

	# set format for rkisp1_resizer_selfpath pads:
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_resizer_selfpath":0 [fmt:YUYV8_2X8/1500x1100 crop: (300,400)/1400x1000]'
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_resizer_selfpath":1 [fmt:YUYV8_2X8/900x800]'

	# set format for rkisp1_selfpath:
	"v4l2-ctl" "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "-v" "width=900,height=800,"
	"v4l2-ctl" "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "-v" "pixelformat=422P"

	# start streaming:
	v4l2-ctl "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "--stream-mmap" "--stream-count" "10"


鍦ㄤ笂杩扮ず渚嬩腑锛屼紶鎰熷櫒琚厤缃负 bayer 鏍煎紡锛?`SRGGB10_1X10/1640x1232`銆俽kisp1_isp:0 pad 搴旈厤缃负涓庝紶鎰熷櫒鐩稿悓鐨?mbus 鏍煎紡
鍜屽昂瀵革紝鍚﹀垯娴佸紡浼犺緭灏嗕互 'EPIPE' 閿欒澶辫触銆傚洜姝ゅ畠涔熻閰嶇疆涓?`SRGGB10_1X10/1640x1232`銆?姝ゅ锛宺kisp1_isp:0 pad 琚厤缃负瑁佸壀 `(0,0)/1600x1200`銆?
瑁佸壀灏哄浼氳嚜鍔ㄤ紶鎾垚涓?isp 婧?pad `rkisp1_isp:2` 鐨勬牸寮忋€傚彟涓€涓鍓搷浣?閰嶇疆鍦?isp 婧?pad 涓婏細`(0,0)/1500x1100`銆?
缂╂斁鍣ㄧ殑 sink pad `rkisp1_resizer_selfpath` 搴旈厤缃负鏍煎紡
`YUYV8_2X8/1500x1100`锛屼互鍖归厤閾捐矾鍙︿竴渚х殑鏍煎紡銆傛澶栬繕鍦ㄥ叾涓婇厤缃簡瑁佸壀
`(300,400)/1400x1000`銆?
缂╂斁鍣ㄧ殑婧?pad `rkisp1_resizer_selfpath:1` 琚厤缃负鏍煎紡 `YUYV8_2X8/900x800`銆?杩欐剰鍛崇潃缂╂斁鍣ㄥ厛浠庢帴鏀跺埌鐨勫抚涓鍓嚭 `(300,400)/1400x100` 鐨勭獥鍙ｏ紝鐒跺悗灏?璇ョ獥鍙ｇ缉鏀惧埌 `900x800` 灏哄銆?
娉ㄦ剰锛屼笂杩扮ず渚嬫湭浣跨敤 stats-params 鎺у埗鐜€傚洜姝ら噰闆嗗埌鐨勫抚涓嶄細缁忚繃 3A 绠楁硶锛?璐ㄩ噺鍙兘涓嶄匠锛岀敋鑷冲彲鑳芥樉寰楀亸鏆椼€佸亸缁裤€?
## 閰嶇疆閲忓寲


璇ラ┍鍔ㄦ敮鎸?YUV 鏍煎紡鐨?limited锛堝彈闄愶級鍜?full range锛堝叏鑼冨洿锛夐噺鍖栵紝鍏朵腑
limited 涓洪粯璁ゃ€?瑕佸湪浜岃€呬箣闂村垏鎹紝鐢ㄦ埛绌洪棿搴斾娇鐢?isp锛坄rkisp1_isp:2`锛夌殑 source pad 2 涓?瀛愯澶囩殑鑹插僵绌洪棿杞崲 API锛圕SC锛夈€傚湪姝?pad 涓婇厤缃殑閲忓寲灏辨槸 mainpath 鍜?selfpath 瑙嗛鑺傜偣涓婃墍閲囬泦瑙嗛甯х殑閲忓寲銆?娉ㄦ剰锛屽嵆浣块噺鍖栧湪 `rkisp1_isp:2` 涓婅閰嶇疆涓哄叏鑼冨洿锛岀缉鏀惧櫒鍜岄噰闆嗗疄浣撲篃濮嬬粓
浼氭姤鍛?`V4L2_QUANTIZATION_DEFAULT`銆傚洜姝わ紝瑕佽幏鍙栨墍閰嶇疆鐨勯噺鍖栧€硷紝搴旂敤绋嬪簭
搴斿彇鑷?pad `rkisp1_isp:2`銆?