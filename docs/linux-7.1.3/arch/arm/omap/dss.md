# OMAP2/3 显示子系


这是 一almost 总计 rewrite the OMAP FB 驱动 驱动/视频/omap
(let's call DSS1). The 主要 differences 之间 DSS1 DSS2 DSI,
TV-out 多个 显示支持, 存在 lots small improvements
涔。

The DSS2 驱动 (omapdss 模块) arch/arm/plat-omap/dss/, the FB,
面板 控制驱动 驱动/视频/omap2/. DSS1 DSS2 实时
currently side side, 您可choose one 使用.

### 特


Working tested 特包含:

- MIPI DPI (并行) 输出
- MIPI DSI 输出 命令 模式
- MIPI DBI (RFBI) 输出
- SDI 输出
- TV 输出
- 全部 pieces compiled 作为 一模块 inside 内核
- 使用 DISPC 更新 任何 the outputs
- 使用 CPU 更新 RFBI DSI 输出
- OMAP DISPC planes
- RGB16, RGB24 packed, RGB24 unpacked
- YUV2, UYVY
- Scaling
- Adjusting DSS FCK find 一good pixel clock
- 使用 DSI DPLL 创建 DSS FCK

Tested boards 包含:
- OMAP3 SDP board
- Beagle board
- N810

### omapdss 驱动


The DSS 驱动 执行 itself 具有 任何 支持 用于 Linux framebuffer, V4L 
此类 类似 the 电流 ones, 具有 一内部 内核 API upper level
驱动 使用.

The DSS 驱动 models OMAP's overlays, overlay managers displays 一
flexible way 启用 non-common multi-display 配置. 此外 
modelling the 硬件 overlays, omapdss supports 虚拟 overlays overlay
managers. 这些 使用 updating 一显示CPU 系统 DMA.

### omapdss 驱动 支持 用于 音频

那里 exist 若干 显示technologies standards 支持 音频 作为
well. Hence, 它是 relevant 更新 the DSS 设备 驱动 提供 一音频
接口 使用 一音频 驱动 任何 其他 驱动 interested 
the functionality.

The 音频_启用 函数 intended prepare the relevant
IP 用于 playback (e.g., enabling 一音频 FIFO, taking 超出 reset
一IP, enabling companion chips, . 它是 intended called 之前
音频_启动. The 音频_禁用 函数 performs the reverse 操作 
intended called 之后 音频_停止.

同时 一given DSS 设备 驱动 支持 音频, 它是 可能 用于
某些 configurations 音频 受支(e.g., 一HDMI 显示使用 一
VESA 视频 timing). The 音频_受支函数 intended query 是否
the 电流 配置 the 显示supports 音频.

The 音频_配置 函数 intended configure 全部 the relevant 音频
参数 the 显示 为了 make the 函数 independent 任何
特定 DSS 设备 驱动, 一结构omap_dss_音频 定义. purpose
包含 全部 the 必需 参数 用于 音频 配置. the
moment, 此类 结构包含 指针 IEC-60958 channel 状word
CEA-861 音频 infoframe 结构 应当 enough 支持
HDMI DisplayPort, 作为 两基于 CEA-861 IEC-60958.

The 音频_启用/禁用, 音频_配置 音频_受支函数 可以 
implemented 作为 函数 sleep. Hence, 它们 应当 called
同时 holding 一自旋一readlock.

The 音频_启动/音频_停止 函数 intended effectively 启动/停止 音频
playback 之后 the 配置 具有 taken place. 这些 函数 designed
使用 一原子 上下 Hence, 音频_启动 应当 return quickly 
called 之后 全部 the needed resources 用于 音频 playback (音频 FIFOs,
DMA channels, companion chips,  具有 已经 已启begin 数据 transfers.
音频_停止 designed 停止 the 音频 transfers. The resources 使用
用于 playback released 使用 音频_禁用.

The enum omap_dss_音频_状使用 help the implementations 
the 接口 keep track the 音频 状 The initial 状_已禁
然后, the 状transitions _CONFIGURED, 然后, 它是 ready 
play 音频, _已启 The 状_PLAYING 使用 the 音频 正在
rendered.


### 闈㈡澘 鍜，鎺у埗鍣，椹卞姩


The 驱动 implement 面板 控制特定 functionality 
通常 visible users except through omapfb 驱动.  它们 注册
themselves the DSS 驱动.

### omapfb 驱动


The omapfb 驱动 implements arbitrary 数字 标准 linux framebuffers.
这些 framebuffers routed flexibly 任何 overlays, 从allowing very
动显示architecture.

The 驱动 exports 一omapfb 特定 ioctls, compatible the
ioctls the 驱动.

The rest the non 标准 特exported 通过 sysfs. 是否 the final
implementation 使用 sysfs, ioctls, 仍然 打开.

### V4L2 驱动


V4L2 正在 implemented TI.

来自 omapdss point view the V4L2 驱动 应当 similar framebuffer
驱动.

### Architecture


一clarification 什the 不同 components 执行:

    - Framebuffer 一内存 area inside OMAP's SRAM/SDRAM 包含 the
      pixel 数据 用于 the image. Framebuffer 具有 width height color
      depth.
    - Overlay defines 何处 the pixels 读取 来自 何处 它们 go the
      screen. The overlay 小于 framebuffer, 从displaying 
      part 鐨?the framebuffer. The position 鐨?the overlay 鍙，涓?changed 鑻。
      the overlay 小于 the 显示
    - Overlay manager combines the overlays 鍦，鍒?one image 鍜?feeds them 鍒。
      显示
    - 显示the actual 物理 显示设备.

一framebuffer connected 多个 overlays 显示 the 相同 pixel 数据
全部 the overlays. 注意 case the overlay 输入 sizes 必须 
the 相同,  如果发生 视频 overlays, the 输出 大小 不同. 任何
framebuffer connected 任何 overlay.

一overlay connected one overlay manager. DISPC overlays 
connected DISPC overlay managers, 虚拟 overlays 
connected 虚拟 overlays.

一overlay manager connected one 显示 存在 某些
restrictions kinds displays 一overlay manager connected:

    - DISPC TV overlay manager connected TV 显示
    - 虚拟 overlay managers connected DBI DSI displays.
    - DISPC LCD overlay manager connected 全部 displays, except TV
      显示

### Sysfs

The sysfs 接口 mainly 使用 用于 testing. I don't think sysfs
接口 the best 用于 the final 版本, I don't quite know
什将会 the best interfaces 用于 这些 things.

The sysfs 接口 divided two parts: DSS FB.

/sys/绫?graphics/fb? directory:
mirror		0=off, 1=鍦。
rotate		Rotation 0-3 用于 0, 90, 180, 270 degrees
rotate_类型	0 = DMA rotation, 1 = VRFB rotation
overlays	列出 overlay numbers framebuffer pixels go
phys_addr	物理 地址 the framebuffer
virt_addr	虚拟 地址 the framebuffer
大小		大小 the framebuffer

/sys/设备/platform/omapdss/overlay? directory:
已启	0=off, 1=
输入_大小	width,height (ie. the framebuffer 大小)
manager		Destination overlay manager name
name
输出_大小	width,height
position	x,y
screen_width	width
全局_alpha   	全局 alpha 0-255 0=transparent 255=opaque

/sys/设备/platform/omapdss/manager? directory:
显示			Destination 显示
name
alpha_blending_已启	0=off, 1=
trans_key_已启	0=off, 1=
trans_key_类型			gfx-destination, video-source
trans_key_鍊?		transparency color key (RGB24)
默认_color			默认 background color (RGB24)

/sys/设备/platform/omapdss/显示 directory:

=============== =============================================================
ctrl_name	鎺у埗鍣?name
mirror		0=off, 1=鍦。
更新_模式	0=off, 1=auto, 2=manual
已启	0=off, 1=
name
rotate		Rotation 0-3 用于 0, 90, 180, 270 degrees
timings		显示timings (pixclock,xres/hfp/hbp/hsw,yres/vfp/vbp/vsw)
		writing, two 特殊 timings accepted 用于 tv-out:
		"pal" 鍜?"ntsc"
面板_name
tear_elim	Tearing elimination 0=off, 1=鍦。
输出_类型	输出 类型 (视频 encoder : "composite" "svideo"
=============== =============================================================

存在 一debugfs 文件 <debugfs>/omapdss/ 显示 information
鍏充簬 clocks 鍜，瀵勫瓨鍣。

### 示例


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
### 默认 setup OMAP3 SDP


此处's the 默认 setup OMAP3 SDP board. 全部 planes go LCD. DVI
TV-out 使用. The columns 来自 left right 
framebuffers, overlays, overlay managers, displays. Framebuffers 鏄。
```

	FB0 --- GFX  -\            DVI
	FB1 --- VID1 --+- LCD ---- LCD
	FB2 --- VID2 -/   TV ----- TV

```
### 示例: Switch 来自 LCD DVI


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
### 示例: Clone GFX overlay LCD TV


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


OMAP FB allocates the framebuffer 内存 使用 the 标准 dma allocator. 
启用 Contiguous 内存 Allocator (配置_CMA) improve the dma
allocator, CMA 已启 使用 "cma=" 内核 参数 increase
the 全局 内存 area 用于 CMA.

使用 DSI DPLL generate pixel clock 它是 可能 produce the pixel clock
86.5MHz (max 可能), get 1280x1024@57 输出 来自 DVI.

Rotation 鍜?mirroring currently 浠?supports RGB565 鍜?RGB8888 modes. VRFB
执行 支持 mirroring.

VRFB rotation 需much 更多 内存 non-rotated framebuffer, 因此 
probably 需increase 您的 vram 设置 之前 使用 VRFB rotation. 
许多 applications work VRFB 它们 执行 pay attention 全部
framebuffer 参数.

### 内核 boot arguments


omapfb.模式=<显示:<模式>[,...]
 - 默认 视频 模式 用于 specified displays. 例如,
	  "dvi:800x400MR-24@60".  参见 驱动/视频/modedb.c.
	  存在 two 特殊 modes: "pal" "ntsc" 
	  使用 tv out.

omapfb.vram=<fbnum>:<size>[@<physaddr>][,...]
 - VRAM allocated 用于 一framebuffer. Normally omapfb allocates vram
	  depending the 显示大小. 您可manually allocate
	  更多 定义 the 物理 地址 每个 framebuffer. 例如,
	  "1:4M" allocate 4M 用于 fb1.

omapfb.debug=<y|n>
 - 启用 debug printing. 具有 具有 OMAPFB debug 支持 已启
	  内核 配置.

omapfb.test=<y|n>
 - Draw test pattern framebuffer whenever framebuffer 设置 change.
	  需具有 OMAPFB debug 支持 已启内核 配置.

omapfb.vrfb=<y|n>
 - 使用 VRFB rotation 用于 全部 framebuffers.

omapfb.rotate=<angle>
 - 默认 rotation applied 全部 framebuffers.
	  0 - 0 degree rotation
	  1 - 90 degree rotation
	  2 - 180 degree rotation
	  3 - 270 degree rotation

omapfb.mirror=<y|n>
 - 默认 mirror 用于 全部 framebuffers. works DMA rotation.

omapdss.def_disp=<显示
 - Name 默认 显示 全部 overlays connected.
	  通用 示例 "LCD" "tv".

omapdss.debug=<y|n>
 - 启用 debug printing. 具有 具有 DSS debug 支持 已启
	  内核 配置.

### TODO


DSS locking

错误 checking

- Lots checks missing implemented just 作为 BUG()

系统 DMA 更新 用于 DSI

- 使用 用于 RGB16 RGB24P modes. Probably 用于 RGB24U (如何
  鍒?skip the empty byte锛。

OMAP1 支持

- 涓?sure 鑻?needed
