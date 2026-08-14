## OMAP2/3 显示器 子系统


这是 一个 almost 总计 rewrite 的 the OMAP FB 驱动 在 驱动/视频/omap
(let's call 它 DSS1). The 主要 differences 之间 DSS1 和 DSS2 是 DSI,
TV-out 和 多个 显示器 支持, 但 存在 lots 的 small improvements
也.

The DSS2 驱动 (omapdss 模块) 是 在 arch/arm/plat-omap/dss/, 和 the FB,
面板 和 控制器 驱动 是 在 驱动/视频/omap2/. DSS1 和 DSS2 实时
currently side 由 side, 您可以 choose 其 one 到 使用.

### 特性


Working 和 tested 特性 包含:

- MIPI DPI (并行) 输出
- MIPI DSI 输出 在 命令 模式
- MIPI DBI (RFBI) 输出
- SDI 输出
- TV 输出
- 全部 pieces 可 为 compiled 作为 一个 模块 或 inside 内核
- 使用 DISPC 到 更新 任何 的 the outputs
- 使用 CPU 到 更新 RFBI 或 DSI 输出
- OMAP DISPC planes
- RGB16, RGB24 packed, RGB24 unpacked
- YUV2, UYVY
- Scaling
- Adjusting DSS FCK 到 find 一个 good pixel clock
- 使用 DSI DPLL 到 创建 DSS FCK

Tested boards 包含:
- OMAP3 SDP board
- Beagle board
- N810

### omapdss 驱动


The DSS 驱动 执行 不 itself 具有 任何 支持 用于 Linux framebuffer, V4L 或
此类 类似 the 电流 ones, 但 它 具有 一个 内部 内核 API 该 upper level
驱动 可 使用.

The DSS 驱动 models OMAP's overlays, overlay managers 和 displays 在 一个
flexible way 到 启用 non-common multi-display 配置. 此外 到
modelling the 硬件 overlays, omapdss supports 虚拟 overlays 和 overlay
managers. 这些 可 为 使用 当 updating 一个 显示器 与 CPU 或 系统 DMA.

### omapdss 驱动 支持 用于 音频

那里 exist 若干 显示器 technologies 和 standards 该 支持 音频 作为
well. Hence, 它是 relevant 到 更新 the DSS 设备 驱动 到 提供 一个 音频
接口 该 可 为 使用 由 一个 音频 驱动 或 任何 其他 驱动 interested 在
the functionality.

The 音频_启用 函数 是 intended 到 prepare the relevant
IP 用于 playback (e.g., enabling 一个 音频 FIFO, taking 在/超出 reset
一些 IP, enabling companion chips, 等). 它是 intended 到 为 called 之前
音频_启动. The 音频_禁用 函数 performs the reverse 操作 和 是
intended 到 为 called 之后 音频_停止.

同时 一个 given DSS 设备 驱动 可 支持 音频, 它是 可能 该 用于
某些 configurations 音频 是 不 受支持 (e.g., 一个 HDMI 显示器 使用 一个
VESA 视频 timing). The 音频_受支持 函数 是 intended 到 query 是否
the 电流 配置 的 the 显示器 supports 音频.

The 音频_配置 函数 是 intended 到 configure 全部 the relevant 音频
参数 的 the 显示器. 为了 make the 函数 independent 的 任何
特定 DSS 设备 驱动, 一个 结构体 omap_dss_音频 是 定义. 其 purpose
是 到 包含 全部 the 必需 参数 用于 音频 配置. 在 the
moment, 此类 结构体 包含 指针 到 IEC-60958 channel 状态 word
和 CEA-861 音频 infoframe 结构体. 此 应当 为 enough 到 支持
HDMI 和 DisplayPort, 作为 两者 是 基于 CEA-861 和 IEC-60958.

The 音频_启用/禁用, 音频_配置 和 音频_受支持 函数 可以 为
implemented 作为 函数 该 可 sleep. Hence, 它们 应当 不 为 called
同时 holding 一个 自旋锁 或 一个 readlock.

The 音频_启动/音频_停止 函数 是 intended 到 effectively 启动/停止 音频
playback 之后 the 配置 具有 taken place. 这些 函数 是 designed
到 为 使用 在 一个 原子 上下文. Hence, 音频_启动 应当 return quickly 和 为
called 仅 之后 全部 the needed resources 用于 音频 playback (音频 FIFOs,
DMA channels, companion chips, 等) 具有 已经 已启用 到 begin 数据 transfers.
音频_停止 是 designed 到 仅 停止 the 音频 transfers. The resources 使用
用于 playback 是 released 使用 音频_禁用.

The enum omap_dss_音频_状态 可 为 使用 到 help the implementations 的
the 接口 到 keep track 的 the 音频 状态. The initial 状态 是 _已禁用;
然后, the 状态 transitions 到 _CONFIGURED, 和 然后, 当 它是 ready 到
play 音频, 到 _已启用. The 状态 _PLAYING 是 使用 当 the 音频 是 正在
rendered.


### 面板 和 控制器 驱动


The 驱动 implement 面板 或 控制器 特定 functionality 和 是 不
通常 visible 到 users except through omapfb 驱动.  它们 注册
themselves 到 the DSS 驱动.

### omapfb 驱动


The omapfb 驱动 implements arbitrary 数字 的 标准 linux framebuffers.
这些 framebuffers 可 为 routed flexibly 到 任何 overlays, 从而 allowing very
动态 显示器 architecture.

The 驱动 exports 一些 omapfb 特定 ioctls, 其 是 compatible 与 the
ioctls 在 the 旧 驱动.

The rest 的 the non 标准 特性 是 exported 通过 sysfs. 是否 the final
implementation 将 使用 sysfs, 或 ioctls, 是 仍然 打开.

### V4L2 驱动


V4L2 是 正在 implemented 在 TI.

来自 omapdss point 的 view the V4L2 驱动 应当 为 similar 到 framebuffer
驱动.

### Architecture


一些 clarification 什么 the 不同 components 执行:

    - Framebuffer 是 一个 内存 area inside OMAP's SRAM/SDRAM 该 包含 the
      pixel 数据 用于 the image. Framebuffer 具有 width 和 height 和 color
      depth.
    - Overlay defines 何处 the pixels 是 读取 来自 和 何处 它们 go 在 the
      screen. The overlay 可 为 小于 framebuffer, 从而 displaying 仅
      part 的 the framebuffer. The position 的 the overlay 可 为 changed 若
      the overlay 是 小于 the 显示器.
    - Overlay manager combines the overlays 在 到 one image 和 feeds them 到
      显示器.
    - 显示器 是 the actual 物理 显示器 设备.

一个 framebuffer 可 为 connected 到 多个 overlays 到 显示 the 相同 pixel 数据
在 全部 的 the overlays. 注意 该 在 此 case the overlay 输入 sizes 必须 为
the 相同, 但, 如果发生 视频 overlays, the 输出 大小 可 为 不同. 任何
framebuffer 可 为 connected 到 任何 overlay.

一个 overlay 可 为 connected 到 one overlay manager. 也 DISPC overlays 可 为
connected 仅 到 DISPC overlay managers, 和 虚拟 overlays 可 为 仅
connected 到 虚拟 overlays.

一个 overlay manager 可 为 connected 到 one 显示器. 存在 某些
restrictions 其 kinds 的 displays 一个 overlay manager 可 为 connected:

    - DISPC TV overlay manager 可 为 仅 connected 到 TV 显示器.
    - 虚拟 overlay managers 可 仅 为 connected 到 DBI 或 DSI displays.
    - DISPC LCD overlay manager 可 为 connected 到 全部 displays, except TV
      显示器.

### Sysfs

The sysfs 接口 是 mainly 使用 用于 testing. I don't think sysfs
接口 是 the best 用于 此 在 the final 版本, 但 I don't quite know
什么 将会 为 the best interfaces 用于 这些 things.

The sysfs 接口 是 divided 到 two parts: DSS 和 FB.

/sys/类/graphics/fb? directory:
mirror		0=off, 1=在
rotate		Rotation 0-3 用于 0, 90, 180, 270 degrees
rotate_类型	0 = DMA rotation, 1 = VRFB rotation
overlays	列出 的 overlay numbers 到 其 framebuffer pixels go
phys_addr	物理 地址 的 the framebuffer
virt_addr	虚拟 地址 的 the framebuffer
大小		大小 的 the framebuffer

/sys/设备/platform/omapdss/overlay? directory:
已启用		0=off, 1=在
输入_大小	width,height (ie. the framebuffer 大小)
manager		Destination overlay manager name
name
输出_大小	width,height
position	x,y
screen_width	width
全局_alpha   	全局 alpha 0-255 0=transparent 255=opaque

/sys/设备/platform/omapdss/manager? directory:
显示器				Destination 显示器
name
alpha_blending_已启用		0=off, 1=在
trans_key_已启用		0=off, 1=在
trans_key_类型			gfx-destination, video-source
trans_key_值			transparency color key (RGB24)
默认_color			默认 background color (RGB24)

/sys/设备/platform/omapdss/显示器? directory:

=============== =============================================================
ctrl_name	控制器 name
mirror		0=off, 1=在
更新_模式	0=off, 1=auto, 2=manual
已启用		0=off, 1=在
name
rotate		Rotation 0-3 用于 0, 90, 180, 270 degrees
timings		显示器 timings (pixclock,xres/hfp/hbp/hsw,yres/vfp/vbp/vsw)
		当 writing, two 特殊 timings 是 accepted 用于 tv-out:
		"pal" 和 "ntsc"
面板_name
tear_elim	Tearing elimination 0=off, 1=在
输出_类型	输出 类型 (视频 encoder 仅): "composite" 或 "svideo"
=============== =============================================================

存在 也 一些 debugfs 文件 在 <debugfs>/omapdss/ 其 显示 information
关于 clocks 和 寄存器.

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
### 默认 setup 在 OMAP3 SDP


此处's the 默认 setup 在 OMAP3 SDP board. 全部 planes go 到 LCD. DVI
和 TV-out 是 不 在 使用. The columns 来自 left 到 right 是:
framebuffers, overlays, overlay managers, displays. Framebuffers 是
```

	FB0 --- GFX  -\            DVI
	FB1 --- VID1 --+- LCD ---- LCD
	FB2 --- VID2 -/   TV ----- TV

```
### 示例: Switch 来自 LCD 到 DVI


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
### 示例: Clone GFX overlay 到 LCD 和 TV


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


OMAP FB allocates the framebuffer 内存 使用 the 标准 dma allocator. 您
可 启用 Contiguous 内存 Allocator (配置_CMA) 到 improve the dma
allocator, 和 若 CMA 是 已启用, 您 使用 "cma=" 内核 参数 到 increase
the 全局 内存 area 用于 CMA.

使用 DSI DPLL 到 generate pixel clock 它是 可能 produce the pixel clock
的 86.5MHz (max 可能), 和 与 该 您 get 1280x1024@57 输出 来自 DVI.

Rotation 和 mirroring currently 仅 supports RGB565 和 RGB8888 modes. VRFB
执行 不 支持 mirroring.

VRFB rotation 需要 much 更多 内存 比 non-rotated framebuffer, 因此 您
probably 需要 到 increase 您的 vram 设置 之前 使用 VRFB rotation. 也,
许多 applications 可 不 work 与 VRFB 若 它们 执行 不 pay attention 到 全部
framebuffer 参数.

### 内核 boot arguments


omapfb.模式=<显示器>:<模式>[,...]
 - 默认 视频 模式 用于 specified displays. 例如,
	  "dvi:800x400MR-24@60".  参见 驱动/视频/modedb.c.
	  存在 也 two 特殊 modes: "pal" 和 "ntsc" 该
	  可 为 使用 到 tv out.

omapfb.vram=<fbnum>:<size>[@<physaddr>][,...]
 - VRAM allocated 用于 一个 framebuffer. Normally omapfb allocates vram
	  depending 在 the 显示器 大小. 与 此 您可以 manually allocate
	  更多 或 定义 the 物理 地址 的 每个 framebuffer. 例如,
	  "1:4M" 到 allocate 4M 用于 fb1.

omapfb.debug=<y|n>
 - 启用 debug printing. 您 具有 到 具有 OMAPFB debug 支持 已启用
	  在 内核 配置.

omapfb.test=<y|n>
 - Draw test pattern 到 framebuffer whenever framebuffer 设置 change.
	  您 需要 到 具有 OMAPFB debug 支持 已启用 在 内核 配置.

omapfb.vrfb=<y|n>
 - 使用 VRFB rotation 用于 全部 framebuffers.

omapfb.rotate=<angle>
 - 默认 rotation applied 到 全部 framebuffers.
	  0 - 0 degree rotation
	  1 - 90 degree rotation
	  2 - 180 degree rotation
	  3 - 270 degree rotation

omapfb.mirror=<y|n>
 - 默认 mirror 用于 全部 framebuffers. 仅 works 与 DMA rotation.

omapdss.def_disp=<显示器>
 - Name 的 默认 显示器, 到 其 全部 overlays 将 为 connected.
	  通用 示例 是 "LCD" 或 "tv".

omapdss.debug=<y|n>
 - 启用 debug printing. 您 具有 到 具有 DSS debug 支持 已启用 在
	  内核 配置.

### TODO


DSS locking

错误 checking

- Lots 的 checks 是 missing 或 implemented just 作为 BUG()

系统 DMA 更新 用于 DSI

- 可 为 使用 用于 RGB16 和 RGB24P modes. Probably 不 用于 RGB24U (如何
  到 skip the empty byte?)

OMAP1 支持

- 不 sure 若 needed
