## i.MX7 瑙嗛閲囬泦椹卞姩

鏈枃妗ｈ鏄?i.MX7 澶勭悊鍣ㄧ殑瑙嗛閲囬泦椹卞姩鏋舵瀯涓庡獟浣撶绾匡紝娑电洊 MIPI CSI-2 鎺ユ敹鍣ㄣ€佽棰戝璺鐢ㄥ櫒涓?CMOS 浼犳劅鍣ㄦ帴鍙ｏ紙CSI锛夌瓑纭欢鍗曞厓锛屼互鍙婂畠浠湪 V4L2 妗嗘灦涓嬫毚闇茬殑瀹炰綋涓庢暟鎹矾寰勩€?

### 绠€浠嬶紙Introduction锛?


涓?i.MX5/6 绯诲垪涓嶅悓锛宨.MX7 涓嶅寘鍚浘鍍忓鐞嗗崟鍏冿紙IPU锛夛紱鍥犳锛屾墽琛屾搷浣滄垨澶勭悊閲囬泦甯х殑鑳藉姏鍦ㄥ姛鑳戒笂杈冧笉涓板瘜銆?

i.MX7 鐨勯噰闆嗗寘鍚笁涓崟鍏冿細
- CMOS 浼犳劅鍣ㄦ帴鍙ｏ紙CSI锛?
- 瑙嗛澶氳矾澶嶇敤鍣紙Video Multiplexer锛?
- MIPI CSI-2 鎺ユ敹鍣紙MIPI CSI-2 Receiver锛?


   MIPI Camera Input ---> MIPI CSI-2 --- > |\
                                           | \
                                           |  \
                                           | M |
                                           | U | ------>  CSI ---> Capture
                                           | X |
                                           |  /
   Parallel Camera Input ----------------> | /
                                           |/

鏇村淇℃伅锛岃鍙傝€冩渶鏂扮増鏈殑 i.MX7 鍙傝€冩墜鍐?[#f1]_銆?

### 瀹炰綋锛圗ntities锛?


### imx-mipi-csi2


杩欐槸 MIPI CSI-2 鎺ユ敹鍣ㄥ疄浣撱€傚畠鏈変竴涓?sink pad 鐢ㄤ簬鎺ユ敹鏉ヨ嚜 MIPI CSI-2 鎽勫儚澶翠紶鎰熷櫒鐨勫儚绱犳暟鎹€傚畠鏈変竴涓?source pad锛屽搴斾簬铏氭嫙閫氶亾 0銆傝妯″潡鍏煎鏃╂湡鐗堟湰鐨?Samsung D-phy锛屽苟鏀寔涓ゆ潯 D-PHY Rx 鏁版嵁閫氶亾銆?

### csi-mux


杩欐槸瑙嗛澶氳矾澶嶇敤鍣ㄣ€傚畠鏈変袱涓?sink pad锛岀敤浜庝粠甯︽湁骞惰鎺ュ彛鐨勬憚鍍忓ご浼犳劅鍣ㄦ垨 MIPI CSI-2 铏氭嫙閫氶亾 0 涓€夋嫨銆傚畠鏈変竴涓崟涓€鐨?source pad 璺敱鍒?CSI銆?

### csi


CSI 浣胯姱鐗囪兘澶熺洿鎺ヨ繛鎺ュ埌澶栭儴 CMOS 鍥惧儚浼犳劅鍣ㄣ€侰SI 鍙互鐩存帴涓庡苟琛屽拰 MIPI CSI-2 鎬荤嚎鎺ュ彛銆傚畠鎷ユ湁 256 x 64 鐨?FIFO 鐢ㄤ簬瀛樺偍鎺ユ敹鍒扮殑鍥惧儚鍍忕礌鏁版嵁锛屼互鍙婂祵鍏ュ紡 DMA 鎺у埗鍣ㄧ敤浜庨€氳繃 AHB 鎬荤嚎浠?FIFO 浼犺緭鏁版嵁銆?

璇ュ疄浣撴湁涓€涓?sink pad 浠?csi-mux 瀹炰綋鎺ユ敹鏁版嵁锛屼互鍙婁竴涓崟涓€鐨?source pad 灏嗚棰戝抚鐩存帴璺敱鍒板唴瀛樼紦鍐插尯銆傝 pad 璺敱鍒颁竴涓噰闆嗚澶囪妭鐐广€?

### 浣跨敤璇存槑锛圲sage Notes锛?


涓轰簡杈呭姪閰嶇疆锛屽苟涓轰簡涓庨偅浜涗粎浠庤棰戣澶囪妭鐐硅闂帶鍒堕」鐨?V4L2 搴旂敤绋嬪簭鍚戝悗鍏煎锛岄噰闆嗚澶囨帴鍙ｄ細浠庡綋鍓嶆祦姘寸嚎涓殑娲诲姩瀹炰綋缁ф壙鎺у埗椤癸紝鍥犳鏃㈠彲浠ョ洿鎺ヤ粠瀛愯澶囷紙subdev锛夎闂帶鍒堕」锛屼篃鍙互浠庢椿鍔ㄩ噰闆嗚澶囨帴鍙ｈ闂€備緥濡傦紝浼犳劅鍣ㄦ帶鍒堕」鏃㈠彲浠ヤ粠浼犳劅鍣ㄥ瓙璁惧鑾峰彇锛屼篃鍙互浠庢椿鍔ㄩ噰闆嗚澶囪幏鍙栥€?

### 鎼厤 OV2680 鐨?Warp7


鍦ㄦ骞冲彴涓婏紝涓€涓?OV2680 MIPI CSI-2 妯″潡杩炴帴鍒板唴閮?MIPI CSI-2 鎺ユ敹鍣ㄣ€備互涓嬬ず渚嬮厤缃簡涓€鏉¤棰戦噰闆嗘祦姘寸嚎锛岃緭鍑轰负 800x600锛孊GGR 10 浣?bayer 鏍煎紡锛?


   # Setup links
   media-ctl -l "'ov2680 1-0036':0 -> 'imx7-mipi-csis.0':0[^1^]"
   media-ctl -l "'imx7-mipi-csis.0':1 -> 'csi-mux':1[^1^]"
   media-ctl -l "'csi-mux':2 -> 'csi':0[^1^]"
   media-ctl -l "'csi':1 -> 'csi capture':0[^1^]"

   # Configure pads for pipeline
   media-ctl -V "'ov2680 1-0036':0 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi-mux':1 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi-mux':2 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'imx7-mipi-csis.0':0 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi':0 [fmt:SBGGR10_1X10/800x600 field:none]"

姝ゅ悗鍗冲彲寮€濮嬫祦寮忎紶杈撱€倂4l2-ctl 宸ュ叿鍙敤浜庨€夋嫨浼犳劅鍣ㄦ敮鎸佺殑浠讳綍鍒嗚鲸鐜囥€?


	# media-ctl -p
	Media controller API version 5.2.0

# 	Media device information

	driver          imx7-csi
	model           imx-media
	serial
	bus info
	hw revision     0x0
	driver version  5.2.0

	Device topology
 - entity 1: csi (2 pads, 2 links)
	            type V4L2 subdev subtype Unknown flags 0
	            device node name /dev/v4l-subdev0
	        pad0: Sink
	                [fmt:SBGGR10_1X10/800x600 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                <- "csi-mux":2 [ENABLED]
	        pad1: Source
	                [fmt:SBGGR10_1X10/800x600 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi capture":0 [ENABLED]

 - entity 4: csi capture (1 pad, 1 link)
	            type Node subtype V4L flags 0
	            device node name /dev/video0
	        pad0: Sink
	                <- "csi":1 [ENABLED]

 - entity 10: csi-mux (3 pads, 2 links)
	             type V4L2 subdev subtype Unknown flags 0
	             device node name /dev/v4l-subdev1
	        pad0: Sink
	                [fmt:Y8_1X8/1x1 field:none]
	        pad1: Sink
	               [fmt:SBGGR10_1X10/800x600 field:none]
	                <- "imx7-mipi-csis.0":1 [ENABLED]
	        pad2: Source
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                -> "csi":0 [ENABLED]

 - entity 14: imx7-mipi-csis.0 (2 pads, 2 links)
	             type V4L2 subdev subtype Unknown flags 0
	             device node name /dev/v4l-subdev2
	        pad0: Sink
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                <- "ov2680 1-0036":0 [ENABLED]
	        pad1: Source
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                -> "csi-mux":1 [ENABLED]

 - entity 17: ov2680 1-0036 (1 pad, 1 link)
	             type V4L2 subdev subtype Sensor flags 0
	             device node name /dev/v4l-subdev3
	        pad0: Source
	                [fmt:SBGGR10_1X10/800x600@1/30 field:none colorspace:srgb]
	                -> "imx7-mipi-csis.0":0 [ENABLED]

### 鎼厤 OV5640 鐨?i.MX6ULL-EVK


鍦ㄦ骞冲彴涓婏紝涓€涓苟琛岀殑 OV5640 浼犳劅鍣ㄨ繛鎺ュ埌 CSI 绔彛銆?
浠ヤ笅绀轰緥閰嶇疆浜嗕竴鏉¤棰戦噰闆嗘祦姘寸嚎锛岃緭鍑轰负 640x480锛屾牸寮忎负 UYVY8_2X8锛?


   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'csi':0[^1^]"
   media-ctl -l "'csi':1 -> 'csi capture':0[^1^]"

   # Configure pads for pipeline
   media-ctl -v -V "'ov5640 1-003c':0 [fmt:UYVY8_2X8/640x480 field:none]"

姝ゅ悗鍗冲彲寮€濮嬫祦寮忎紶杈擄細


   gst-launch-1.0 -v v4l2src device=/dev/video1 ! video/x-raw,format=UYVY,width=640,height=480 ! v4l2convert ! fbdevsink


	# media-ctl -p
	Media controller API version 5.14.0

# 	Media device information

	driver          imx7-csi
	model           imx-media
	serial
	bus info
	hw revision     0x0
	driver version  5.14.0

	Device topology
 - entity 1: csi (2 pads, 2 links)
	            type V4L2 subdev subtype Unknown flags 0
	            device node name /dev/v4l-subdev0
	        pad0: Sink
	                [fmt:UYVY8_2X8/640x480 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                <- "ov5640 1-003c":0 [ENABLED,IMMUTABLE]
	        pad1: Source
	                [fmt:UYVY8_2X8/640x480 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi capture":0 [ENABLED,IMMUTABLE]

 - entity 4: csi capture (1 pad, 1 link)
	            type Node subtype V4L flags 0
	            device node name /dev/video1
	        pad0: Sink
	                <- "csi":1 [ENABLED,IMMUTABLE]

 - entity 10: ov5640 1-003c (1 pad, 1 link)
	             type V4L2 subdev subtype Sensor flags 0
	             device node name /dev/v4l-subdev1
	        pad0: Source
	                [fmt:UYVY8_2X8/640x480@1/30 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi":0 [ENABLED,IMMUTABLE]

### 鍙傝€冿紙References锛?
