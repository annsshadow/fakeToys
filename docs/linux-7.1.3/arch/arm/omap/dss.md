锘?# OMAP2/3 鏄剧ず鍣?瀛愮郴缁?


杩欐槸 涓€涓?almost 鎬昏 rewrite 鐨?the OMAP FB 椹卞姩 鍦?椹卞姩/瑙嗛/omap
(let's call 瀹?DSS1). The 涓昏 differences 涔嬮棿 DSS1 鍜?DSS2 鏄?DSI,
TV-out 鍜?澶氫釜 鏄剧ず鍣?鏀寔, 浣?瀛樺湪 lots 鐨?small improvements
涔?

The DSS2 椹卞姩 (omapdss 妯″潡) 鏄?鍦?arch/arm/plat-omap/dss/, 鍜?the FB,
闈㈡澘 鍜?鎺у埗鍣?椹卞姩 鏄?鍦?椹卞姩/瑙嗛/omap2/. DSS1 鍜?DSS2 瀹炴椂
currently side 鐢?side, 鎮ㄥ彲浠?choose 鍏?one 鍒?浣跨敤.

### 鐗规€?


Working 鍜?tested 鐗规€?鍖呭惈:

- MIPI DPI (骞惰) 杈撳嚭
- MIPI DSI 杈撳嚭 鍦?鍛戒护 妯″紡
- MIPI DBI (RFBI) 杈撳嚭
- SDI 杈撳嚭
- TV 杈撳嚭
- 鍏ㄩ儴 pieces 鍙?涓?compiled 浣滀负 涓€涓?妯″潡 鎴?inside 鍐呮牳
- 浣跨敤 DISPC 鍒?鏇存柊 浠讳綍 鐨?the outputs
- 浣跨敤 CPU 鍒?鏇存柊 RFBI 鎴?DSI 杈撳嚭
- OMAP DISPC planes
- RGB16, RGB24 packed, RGB24 unpacked
- YUV2, UYVY
- Scaling
- Adjusting DSS FCK 鍒?find 涓€涓?good pixel clock
- 浣跨敤 DSI DPLL 鍒?鍒涘缓 DSS FCK

Tested boards 鍖呭惈:
- OMAP3 SDP board
- Beagle board
- N810

### omapdss 椹卞姩


The DSS 椹卞姩 鎵ц 涓?itself 鍏锋湁 浠讳綍 鏀寔 鐢ㄤ簬 Linux framebuffer, V4L 鎴?
姝ょ被 绫讳技 the 鐢垫祦 ones, 浣?瀹?鍏锋湁 涓€涓?鍐呴儴 鍐呮牳 API 璇?upper level
椹卞姩 鍙?浣跨敤.

The DSS 椹卞姩 models OMAP's overlays, overlay managers 鍜?displays 鍦?涓€涓?
flexible way 鍒?鍚敤 non-common multi-display 閰嶇疆. 姝ゅ 鍒?
modelling the 纭欢 overlays, omapdss supports 铏氭嫙 overlays 鍜?overlay
managers. 杩欎簺 鍙?涓?浣跨敤 褰?updating 涓€涓?鏄剧ず鍣?涓?CPU 鎴?绯荤粺 DMA.

### omapdss 椹卞姩 鏀寔 鐢ㄤ簬 闊抽

閭ｉ噷 exist 鑻ュ共 鏄剧ず鍣?technologies 鍜?standards 璇?鏀寔 闊抽 浣滀负
well. Hence, 瀹冩槸 relevant 鍒?鏇存柊 the DSS 璁惧 椹卞姩 鍒?鎻愪緵 涓€涓?闊抽
鎺ュ彛 璇?鍙?涓?浣跨敤 鐢?涓€涓?闊抽 椹卞姩 鎴?浠讳綍 鍏朵粬 椹卞姩 interested 鍦?
the functionality.

The 闊抽_鍚敤 鍑芥暟 鏄?intended 鍒?prepare the relevant
IP 鐢ㄤ簬 playback (e.g., enabling 涓€涓?闊抽 FIFO, taking 鍦?瓒呭嚭 reset
涓€浜?IP, enabling companion chips, 绛?. 瀹冩槸 intended 鍒?涓?called 涔嬪墠
闊抽_鍚姩. The 闊抽_绂佺敤 鍑芥暟 performs the reverse 鎿嶄綔 鍜?鏄?
intended 鍒?涓?called 涔嬪悗 闊抽_鍋滄.

鍚屾椂 涓€涓?given DSS 璁惧 椹卞姩 鍙?鏀寔 闊抽, 瀹冩槸 鍙兘 璇?鐢ㄤ簬
鏌愪簺 configurations 闊抽 鏄?涓?鍙楁敮鎸?(e.g., 涓€涓?HDMI 鏄剧ず鍣?浣跨敤 涓€涓?
VESA 瑙嗛 timing). The 闊抽_鍙楁敮鎸?鍑芥暟 鏄?intended 鍒?query 鏄惁
the 鐢垫祦 閰嶇疆 鐨?the 鏄剧ず鍣?supports 闊抽.

The 闊抽_閰嶇疆 鍑芥暟 鏄?intended 鍒?configure 鍏ㄩ儴 the relevant 闊抽
鍙傛暟 鐨?the 鏄剧ず鍣? 涓轰簡 make the 鍑芥暟 independent 鐨?浠讳綍
鐗瑰畾 DSS 璁惧 椹卞姩, 涓€涓?缁撴瀯浣?omap_dss_闊抽 鏄?瀹氫箟. 鍏?purpose
鏄?鍒?鍖呭惈 鍏ㄩ儴 the 蹇呴渶 鍙傛暟 鐢ㄤ簬 闊抽 閰嶇疆. 鍦?the
moment, 姝ょ被 缁撴瀯浣?鍖呭惈 鎸囬拡 鍒?IEC-60958 channel 鐘舵€?word
鍜?CEA-861 闊抽 infoframe 缁撴瀯浣? 姝?搴斿綋 涓?enough 鍒?鏀寔
HDMI 鍜?DisplayPort, 浣滀负 涓よ€?鏄?鍩轰簬 CEA-861 鍜?IEC-60958.

The 闊抽_鍚敤/绂佺敤, 闊抽_閰嶇疆 鍜?闊抽_鍙楁敮鎸?鍑芥暟 鍙互 涓?
implemented 浣滀负 鍑芥暟 璇?鍙?sleep. Hence, 瀹冧滑 搴斿綋 涓?涓?called
鍚屾椂 holding 涓€涓?鑷棆閿?鎴?涓€涓?readlock.

The 闊抽_鍚姩/闊抽_鍋滄 鍑芥暟 鏄?intended 鍒?effectively 鍚姩/鍋滄 闊抽
playback 涔嬪悗 the 閰嶇疆 鍏锋湁 taken place. 杩欎簺 鍑芥暟 鏄?designed
鍒?涓?浣跨敤 鍦?涓€涓?鍘熷瓙 涓婁笅鏂? Hence, 闊抽_鍚姩 搴斿綋 return quickly 鍜?涓?
called 浠?涔嬪悗 鍏ㄩ儴 the needed resources 鐢ㄤ簬 闊抽 playback (闊抽 FIFOs,
DMA channels, companion chips, 绛? 鍏锋湁 宸茬粡 宸插惎鐢?鍒?begin 鏁版嵁 transfers.
闊抽_鍋滄 鏄?designed 鍒?浠?鍋滄 the 闊抽 transfers. The resources 浣跨敤
鐢ㄤ簬 playback 鏄?released 浣跨敤 闊抽_绂佺敤.

The enum omap_dss_闊抽_鐘舵€?鍙?涓?浣跨敤 鍒?help the implementations 鐨?
the 鎺ュ彛 鍒?keep track 鐨?the 闊抽 鐘舵€? The initial 鐘舵€?鏄?_宸茬鐢?
鐒跺悗, the 鐘舵€?transitions 鍒?_CONFIGURED, 鍜?鐒跺悗, 褰?瀹冩槸 ready 鍒?
play 闊抽, 鍒?_宸插惎鐢? The 鐘舵€?_PLAYING 鏄?浣跨敤 褰?the 闊抽 鏄?姝ｅ湪
rendered.


### 闈㈡澘 鍜?鎺у埗鍣?椹卞姩


The 椹卞姩 implement 闈㈡澘 鎴?鎺у埗鍣?鐗瑰畾 functionality 鍜?鏄?涓?
閫氬父 visible 鍒?users except through omapfb 椹卞姩.  瀹冧滑 娉ㄥ唽
themselves 鍒?the DSS 椹卞姩.

### omapfb 椹卞姩


The omapfb 椹卞姩 implements arbitrary 鏁板瓧 鐨?鏍囧噯 linux framebuffers.
杩欎簺 framebuffers 鍙?涓?routed flexibly 鍒?浠讳綍 overlays, 浠庤€?allowing very
鍔ㄦ€?鏄剧ず鍣?architecture.

The 椹卞姩 exports 涓€浜?omapfb 鐗瑰畾 ioctls, 鍏?鏄?compatible 涓?the
ioctls 鍦?the 鏃?椹卞姩.

The rest 鐨?the non 鏍囧噯 鐗规€?鏄?exported 閫氳繃 sysfs. 鏄惁 the final
implementation 灏?浣跨敤 sysfs, 鎴?ioctls, 鏄?浠嶇劧 鎵撳紑.

### V4L2 椹卞姩


V4L2 鏄?姝ｅ湪 implemented 鍦?TI.

鏉ヨ嚜 omapdss point 鐨?view the V4L2 椹卞姩 搴斿綋 涓?similar 鍒?framebuffer
椹卞姩.

### Architecture


涓€浜?clarification 浠€涔?the 涓嶅悓 components 鎵ц:

    - Framebuffer 鏄?涓€涓?鍐呭瓨 area inside OMAP's SRAM/SDRAM 璇?鍖呭惈 the
      pixel 鏁版嵁 鐢ㄤ簬 the image. Framebuffer 鍏锋湁 width 鍜?height 鍜?color
      depth.
    - Overlay defines 浣曞 the pixels 鏄?璇诲彇 鏉ヨ嚜 鍜?浣曞 瀹冧滑 go 鍦?the
      screen. The overlay 鍙?涓?灏忎簬 framebuffer, 浠庤€?displaying 浠?
      part 鐨?the framebuffer. The position 鐨?the overlay 鍙?涓?changed 鑻?
      the overlay 鏄?灏忎簬 the 鏄剧ず鍣?
    - Overlay manager combines the overlays 鍦?鍒?one image 鍜?feeds them 鍒?
      鏄剧ず鍣?
    - 鏄剧ず鍣?鏄?the actual 鐗╃悊 鏄剧ず鍣?璁惧.

涓€涓?framebuffer 鍙?涓?connected 鍒?澶氫釜 overlays 鍒?鏄剧ず the 鐩稿悓 pixel 鏁版嵁
鍦?鍏ㄩ儴 鐨?the overlays. 娉ㄦ剰 璇?鍦?姝?case the overlay 杈撳叆 sizes 蹇呴』 涓?
the 鐩稿悓, 浣? 濡傛灉鍙戠敓 瑙嗛 overlays, the 杈撳嚭 澶у皬 鍙?涓?涓嶅悓. 浠讳綍
framebuffer 鍙?涓?connected 鍒?浠讳綍 overlay.

涓€涓?overlay 鍙?涓?connected 鍒?one overlay manager. 涔?DISPC overlays 鍙?涓?
connected 浠?鍒?DISPC overlay managers, 鍜?铏氭嫙 overlays 鍙?涓?浠?
connected 鍒?铏氭嫙 overlays.

涓€涓?overlay manager 鍙?涓?connected 鍒?one 鏄剧ず鍣? 瀛樺湪 鏌愪簺
restrictions 鍏?kinds 鐨?displays 涓€涓?overlay manager 鍙?涓?connected:

    - DISPC TV overlay manager 鍙?涓?浠?connected 鍒?TV 鏄剧ず鍣?
    - 铏氭嫙 overlay managers 鍙?浠?涓?connected 鍒?DBI 鎴?DSI displays.
    - DISPC LCD overlay manager 鍙?涓?connected 鍒?鍏ㄩ儴 displays, except TV
      鏄剧ず鍣?

### Sysfs

The sysfs 鎺ュ彛 鏄?mainly 浣跨敤 鐢ㄤ簬 testing. I don't think sysfs
鎺ュ彛 鏄?the best 鐢ㄤ簬 姝?鍦?the final 鐗堟湰, 浣?I don't quite know
浠€涔?灏嗕細 涓?the best interfaces 鐢ㄤ簬 杩欎簺 things.

The sysfs 鎺ュ彛 鏄?divided 鍒?two parts: DSS 鍜?FB.

/sys/绫?graphics/fb? directory:
mirror		0=off, 1=鍦?
rotate		Rotation 0-3 鐢ㄤ簬 0, 90, 180, 270 degrees
rotate_绫诲瀷	0 = DMA rotation, 1 = VRFB rotation
overlays	鍒楀嚭 鐨?overlay numbers 鍒?鍏?framebuffer pixels go
phys_addr	鐗╃悊 鍦板潃 鐨?the framebuffer
virt_addr	铏氭嫙 鍦板潃 鐨?the framebuffer
澶у皬		澶у皬 鐨?the framebuffer

/sys/璁惧/platform/omapdss/overlay? directory:
宸插惎鐢?	0=off, 1=鍦?
杈撳叆_澶у皬	width,height (ie. the framebuffer 澶у皬)
manager		Destination overlay manager name
name
杈撳嚭_澶у皬	width,height
position	x,y
screen_width	width
鍏ㄥ眬_alpha   	鍏ㄥ眬 alpha 0-255 0=transparent 255=opaque

/sys/璁惧/platform/omapdss/manager? directory:
鏄剧ず鍣?			Destination 鏄剧ず鍣?
name
alpha_blending_宸插惎鐢?	0=off, 1=鍦?
trans_key_宸插惎鐢?	0=off, 1=鍦?
trans_key_绫诲瀷			gfx-destination, video-source
trans_key_鍊?		transparency color key (RGB24)
榛樿_color			榛樿 background color (RGB24)

/sys/璁惧/platform/omapdss/鏄剧ず鍣? directory:

=============== =============================================================
ctrl_name	鎺у埗鍣?name
mirror		0=off, 1=鍦?
鏇存柊_妯″紡	0=off, 1=auto, 2=manual
宸插惎鐢?	0=off, 1=鍦?
name
rotate		Rotation 0-3 鐢ㄤ簬 0, 90, 180, 270 degrees
timings		鏄剧ず鍣?timings (pixclock,xres/hfp/hbp/hsw,yres/vfp/vbp/vsw)
		褰?writing, two 鐗规畩 timings 鏄?accepted 鐢ㄤ簬 tv-out:
		"pal" 鍜?"ntsc"
闈㈡澘_name
tear_elim	Tearing elimination 0=off, 1=鍦?
杈撳嚭_绫诲瀷	杈撳嚭 绫诲瀷 (瑙嗛 encoder 浠?: "composite" 鎴?"svideo"
=============== =============================================================

瀛樺湪 涔?涓€浜?debugfs 鏂囦欢 鍦?<debugfs>/omapdss/ 鍏?鏄剧ず information
鍏充簬 clocks 鍜?瀵勫瓨鍣?

### 绀轰緥


```

	ovl0=/sys/devices/platform/omapdss/overlay0
	ovl1=/sys/devices/platform/omapdss/overlay1
	ovl2=/sys/devices/platform/omapdss/overlay2

	mgr0=/sys/devices/platform/omapdss/manager0
	mgr1=/sys/devices/platform/omapdss/manager1

	lcd=/sys/devices/platform/omapdss/display0
	dvi=/sys/devices/platform/omapdss/display1
	tv=/sys/devices/platform/omapdss/display2

	fb0=/sys/class/graphics/fb0
	fb1=/sys/class/graphics/fb1
	fb2=/sys/class/graphics/fb2

```
### 榛樿 setup 鍦?OMAP3 SDP


姝ゅ's the 榛樿 setup 鍦?OMAP3 SDP board. 鍏ㄩ儴 planes go 鍒?LCD. DVI
鍜?TV-out 鏄?涓?鍦?浣跨敤. The columns 鏉ヨ嚜 left 鍒?right 鏄?
framebuffers, overlays, overlay managers, displays. Framebuffers 鏄?
```

	FB0 --- GFX  -\            DVI
	FB1 --- VID1 --+- LCD ---- LCD
	FB2 --- VID2 -/   TV ----- TV

```
### 绀轰緥: Switch 鏉ヨ嚜 LCD 鍒?DVI


```

	w=`cat $dvi/timings | cut -d "," -f 2 | cut -d "/" -f 1`
	h=`cat $dvi/timings | cut -d "," -f 3 | cut -d "/" -f 1`

	echo "0" > $lcd/enabled
	echo "" > $mgr0/display
	fbset -fb /dev/fb0 -xres $w -yres $h -vxres $w -vyres $h
	# at this point you have to switch the dvi/lcd dip-switch from the omap board
	echo "dvi" > $mgr0/display
	echo "1" > $dvi/enabled

```
```

	FB0 --- GFX  -\         -- DVI
	FB1 --- VID1 --+- LCD -/   LCD
	FB2 --- VID2 -/   TV ----- TV

```
### 绀轰緥: Clone GFX overlay 鍒?LCD 鍜?TV


```

	w=`cat $tv/timings | cut -d "," -f 2 | cut -d "/" -f 1`
	h=`cat $tv/timings | cut -d "," -f 3 | cut -d "/" -f 1`

	echo "0" > $ovl0/enabled
	echo "0" > $ovl1/enabled

	echo "" > $fb1/overlays
	echo "0,1" > $fb0/overlays

	echo "$w,$h" > $ovl1/output_size
	echo "tv" > $ovl1/manager

	echo "1" > $ovl0/enabled
	echo "1" > $ovl1/enabled

	echo "1" > $tv/enabled

```
```

	FB0 +-- GFX  ---- LCD ---- LCD
	\- VID1 ---- TV  ---- TV

```
### Misc notes


OMAP FB allocates the framebuffer 鍐呭瓨 浣跨敤 the 鏍囧噯 dma allocator. 鎮?
鍙?鍚敤 Contiguous 鍐呭瓨 Allocator (閰嶇疆_CMA) 鍒?improve the dma
allocator, 鍜?鑻?CMA 鏄?宸插惎鐢? 鎮?浣跨敤 "cma=" 鍐呮牳 鍙傛暟 鍒?increase
the 鍏ㄥ眬 鍐呭瓨 area 鐢ㄤ簬 CMA.

浣跨敤 DSI DPLL 鍒?generate pixel clock 瀹冩槸 鍙兘 produce the pixel clock
鐨?86.5MHz (max 鍙兘), 鍜?涓?璇?鎮?get 1280x1024@57 杈撳嚭 鏉ヨ嚜 DVI.

Rotation 鍜?mirroring currently 浠?supports RGB565 鍜?RGB8888 modes. VRFB
鎵ц 涓?鏀寔 mirroring.

VRFB rotation 闇€瑕?much 鏇村 鍐呭瓨 姣?non-rotated framebuffer, 鍥犳 鎮?
probably 闇€瑕?鍒?increase 鎮ㄧ殑 vram 璁剧疆 涔嬪墠 浣跨敤 VRFB rotation. 涔?
璁稿 applications 鍙?涓?work 涓?VRFB 鑻?瀹冧滑 鎵ц 涓?pay attention 鍒?鍏ㄩ儴
framebuffer 鍙傛暟.

### 鍐呮牳 boot arguments


omapfb.妯″紡=<鏄剧ず鍣?:<妯″紡>[,...]
 - 榛樿 瑙嗛 妯″紡 鐢ㄤ簬 specified displays. 渚嬪,
	  "dvi:800x400MR-24@60".  鍙傝 椹卞姩/瑙嗛/modedb.c.
	  瀛樺湪 涔?two 鐗规畩 modes: "pal" 鍜?"ntsc" 璇?
	  鍙?涓?浣跨敤 鍒?tv out.

omapfb.vram=<fbnum>:<size>[@<physaddr>][,...]
 - VRAM allocated 鐢ㄤ簬 涓€涓?framebuffer. Normally omapfb allocates vram
	  depending 鍦?the 鏄剧ず鍣?澶у皬. 涓?姝?鎮ㄥ彲浠?manually allocate
	  鏇村 鎴?瀹氫箟 the 鐗╃悊 鍦板潃 鐨?姣忎釜 framebuffer. 渚嬪,
	  "1:4M" 鍒?allocate 4M 鐢ㄤ簬 fb1.

omapfb.debug=<y|n>
 - 鍚敤 debug printing. 鎮?鍏锋湁 鍒?鍏锋湁 OMAPFB debug 鏀寔 宸插惎鐢?
	  鍦?鍐呮牳 閰嶇疆.

omapfb.test=<y|n>
 - Draw test pattern 鍒?framebuffer whenever framebuffer 璁剧疆 change.
	  鎮?闇€瑕?鍒?鍏锋湁 OMAPFB debug 鏀寔 宸插惎鐢?鍦?鍐呮牳 閰嶇疆.

omapfb.vrfb=<y|n>
 - 浣跨敤 VRFB rotation 鐢ㄤ簬 鍏ㄩ儴 framebuffers.

omapfb.rotate=<angle>
 - 榛樿 rotation applied 鍒?鍏ㄩ儴 framebuffers.
	  0 - 0 degree rotation
	  1 - 90 degree rotation
	  2 - 180 degree rotation
	  3 - 270 degree rotation

omapfb.mirror=<y|n>
 - 榛樿 mirror 鐢ㄤ簬 鍏ㄩ儴 framebuffers. 浠?works 涓?DMA rotation.

omapdss.def_disp=<鏄剧ず鍣?
 - Name 鐨?榛樿 鏄剧ず鍣? 鍒?鍏?鍏ㄩ儴 overlays 灏?涓?connected.
	  閫氱敤 绀轰緥 鏄?"LCD" 鎴?"tv".

omapdss.debug=<y|n>
 - 鍚敤 debug printing. 鎮?鍏锋湁 鍒?鍏锋湁 DSS debug 鏀寔 宸插惎鐢?鍦?
	  鍐呮牳 閰嶇疆.

### TODO


DSS locking

閿欒 checking

- Lots 鐨?checks 鏄?missing 鎴?implemented just 浣滀负 BUG()

绯荤粺 DMA 鏇存柊 鐢ㄤ簬 DSI

- 鍙?涓?浣跨敤 鐢ㄤ簬 RGB16 鍜?RGB24P modes. Probably 涓?鐢ㄤ簬 RGB24U (濡備綍
  鍒?skip the empty byte锛?

OMAP1 鏀寔

- 涓?sure 鑻?needed
