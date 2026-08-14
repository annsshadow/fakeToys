## VIA 集成图形芯片控制台帧缓冲驱动


### 平台


    控制台帧缓冲驱动适用于 VIA UniChrome 家族的图形芯片
    （CLE266、PM800 / CN400 / CN300、
    P4M800CE / P4M800Pro / CN700 / VN800、
    CX700 / VX700、K8M890、P4M890、
    CN896 / P4M900、VX800、VX855）

### 驱动特性


    设备：CRT、LCD、DVI

```
	CRT:
	    640x480(60, 75, 85, 100, 120 Hz), 720x480(60 Hz),
	    720x576(60 Hz), 800x600(60, 75, 85, 100, 120 Hz),
	    848x480(60 Hz), 856x480(60 Hz), 1024x512(60 Hz),
	    1024x768(60, 75, 85, 100 Hz), 1152x864(75 Hz),
	    1280x768(60 Hz), 1280x960(60 Hz), 1280x1024(60, 75, 85 Hz),
	    1440x1050(60 Hz), 1600x1200(60, 75 Hz), 1280x720(60 Hz),
	    1920x1080(60 Hz), 1400x1050(60 Hz), 800x480(60 Hz)

    color depth: 8 bpp, 16 bpp, 32 bpp supports.

    Support 2D hardware accelerator.
```

### 使用 viafb 模块


```
	#modprobe viafb

    Start viafb with user options::

	#modprobe viafb viafb_mode=800x600 viafb_bpp=16 viafb_refresh=60
		  viafb_active_dev=CRT+DVI viafb_dvi_port=DVP1
		  viafb_mode1=1024x768 viafb_bpp=16 viafb_refresh1=60
		  viafb_SAMM_ON=1

    viafb_mode:
	- 640x480 (默认)
	- 720x480
	- 800x600
	- 1024x768

    viafb_bpp:
	- 8, 16, 32 (默认:32)

    viafb_refresh:
	- 60, 75, 85, 100, 120 (默认:60)

    viafb_lcd_dsp_method:
	- 0 : 扩展（默认）
	- 1 : 居中

    viafb_lcd_mode:
	0 : LSB 数据格式输入的 LCD 面板（默认）
	1 : MSB 数据格式输入的 LCD 面板

    viafb_lcd_panel_id:
	- 0 : 分辨率: 640x480, 通道: 单, 抖动: 启用
	- 1 : 分辨率: 800x600, 通道: 单, 抖动: 启用
	- 2 : 分辨率: 1024x768, 通道: 单, 抖动: 启用（默认）
	- 3 : 分辨率: 1280x768, 通道: 单, 抖动: 启用
	- 4 : 分辨率: 1280x1024, 通道: 双, 抖动: 启用
	- 5 : 分辨率: 1400x1050, 通道: 双, 抖动: 启用
	- 6 : 分辨率: 1600x1200, 通道: 双, 抖动: 启用

	- 8 : 分辨率: 800x480, 通道: 单, 抖动: 启用
	- 9 : 分辨率: 1024x768, 通道: 双, 抖动: 启用
	- 10: 分辨率: 1024x768, 通道: 单, 抖动: 禁用
	- 11: 分辨率: 1024x768, 通道: 双, 抖动: 禁用
	- 12: 分辨率: 1280x768, 通道: 单, 抖动: 禁用
	- 13: 分辨率: 1280x1024, 通道: 双, 抖动: 禁用
	- 14: 分辨率: 1400x1050, 通道: 双, 抖动: 禁用
	- 15: 分辨率: 1600x1200, 通道: 双, 抖动: 禁用
	- 16: 分辨率: 1366x768, 通道: 单, 抖动: 禁用
	- 17: 分辨率: 1024x600, 通道: 单, 抖动: 启用
	- 18: 分辨率: 1280x768, 通道: 双, 抖动: 启用
	- 19: 分辨率: 1280x800, 通道: 单, 抖动: 启用

    viafb_accel:
	- 0 : 无 2D 硬件加速
	- 1 : 2D 硬件加速（默认）

    viafb_SAMM_ON:
	- 0 : viafb_SAMM_ON 禁用（默认）
	- 1 : viafb_SAMM_ON 启用

    viafb_mode1:（副显示设备）
	- 640x480（默认）
	- 720x480
	- 800x600
	- 1024x768

    viafb_bpp1:（副显示设备）
	- 8, 16, 32（默认:32）

    viafb_refresh1:（副显示设备）
	- 60, 75, 85, 100, 120（默认:60）

    viafb_active_dev:
	此选项用于指定活动设备。（CRT、DVI、CRT+LCD……）
	DVI 代表 DVI 或 HDMI，例如，若想启用 HDMI，
	设置 viafb_active_dev=DVI。在 SAMM 情况下，viafb_active_dev
	之前的是主设备，之后的是副设备。

	例如：

	要启用一个设备，例如仅 DVI，我们可以使用::

	    modprobe viafb viafb_active_dev=DVI

	要启用两个设备，例如 CRT+DVI::

	    modprobe viafb viafb_active_dev=CRT+DVI;

	对于 DuoView 情况，我们可以使用::

	    modprobe viafb viafb_active_dev=CRT+DVI

	或::

	    modprobe viafb viafb_active_dev=DVI+CRT...

	对于 SAMM 情况：

	若 CRT 为主、DVI 为副，我们应该使用::

	    modprobe viafb viafb_active_dev=CRT+DVI viafb_SAMM_ON=1...

	若 DVI 为主、CRT 为副，我们应该使用::

	    modprobe viafb viafb_active_dev=DVI+CRT viafb_SAMM_ON=1...

    viafb_display_hardware_layout:
	此选项用于指定 CX700 芯片的显示硬件布局。

	- 1 : 仅 LCD
	- 2 : 仅 DVI
	- 3 : LCD+DVI（默认）
	- 4 : LCD1+LCD2（内部 + 内部）
	- 16: LCD1+ExternalLCD2（内部 + 外部）

    viafb_second_size:
	此选项用于设置 SAMM 情况下第二设备的内存大小（MB）。
	最小大小为 16。

    viafb_platform_epia_dvi:
	此选项用于启用 EPIA - M 上的 DVI

	- 0 : EPIA - M 上无 DVI（默认）
	- 1 : EPIA - M 上有 DVI

    viafb_bus_width:
	当使用 24 位总线宽度的数字接口时，
	应设置此选项。

	- 12: 12 位 LVDS 或 12 位 TMDS（默认）
	- 24: 24 位 LVDS 或 24 位 TMDS

    viafb_device_lcd_dualedge:
	当使用双边缘面板时，应设置此选项。

	- 0 : 无双边缘面板（默认）
	- 1 : 双边缘面板

    viafb_lcd_port:
	此选项用于指定 LCD 输出端口，
	可用值为 "DVP0" "DVP1" "DFP_HIGHLOW" "DFP_HIGH" "DFP_LOW"。

	对于 CX700 上的外部 LCD + 外部 DVI（外部 LCD 在 DVP0 上），
	我们应该使用::

	    modprobe viafb viafb_lcd_port=DVP0...
```

说明：
    1. 对于 DuoView CRT 与 DVI 显示，在启用了 DVI 过扫描的 “640x480” PAL 模式下，CRT 可能显示不正常。
    2. SAMM 代表单适配器多显示器（single adapter multi monitors）。它与多显示头（multi-head）不同，因为 SAMM 在驱动层支持多显示器，因此 fbcon 层甚至不知道它的存在；SAMM 的第二个屏幕没有设备节点文件，因此用户态应用程序无法直接访问它。当 SAMM 启用时，viafb_mode 与 viafb_mode1、viafb_bpp 与 viafb_bpp1、viafb_refresh 与 viafb_refresh1 可以不同。
    3. 当控制台依赖于 viafbinfo1 时，动态更改分辨率和 bpp，需要调用 VIAFB 指定的 ioctl 接口 VIAFB_SET_DEVICE，而不是调用通用的 ioctl 函数 FBIOPUT_VSCREENINFO，因为 viafb 对多显示头的支持不太好，否则会导致屏幕崩溃。

### 用 “fbset” 工具配置 viafb


    “fbset” 是 Linux 的一个内置实用工具。

```
	   # fbset -i

    2. 设置各种分辨率和 viafb_refresh 速率::

	   # fbset <resolution-vertical_sync>

       example::

	   # fbset "1024x768-75"

       or::

	   # fbset -g 1024 768 1024 768 32

       Check the file "/etc/fb.modes" to find display modes available.

    3. Set the color depth::

	   # fbset -depth <value>

       example::

	   # fbset -depth 16
```

### 通过 /proc 配置 viafb


    以下文件存在于 /proc/viafb 中

    supported_output_devices
	这个只读文件包含一个完整的、以 “,” 分隔的列表，包含你的平台上可能可用的所有输出设备。很可能并非所有这些设备在你的硬件上都有连接器，但它应能提供良好的起点，以弄清这些名称中哪些对应真实的连接器。

```
		# cat /proc/viafb/supported_output_devices

    iga1/output_devices, iga2/output_devices
	这两个文件可读可写。iga1 和 iga2 是产生屏幕图像的两个独立单元。这些图像可以被转发到一个或多个输出设备。读取这些文件是查询某个 iga 当前正在使用哪些输出设备的一种方式。

	示例::

		# cat /proc/viafb/iga1/output_devices

	如果未打印任何输出设备，则该 iga 的输出丢失。例如，如果只使用了一个（另一个）iga，就可能发生这种情况。写入这些文件允许在运行时调整输出设备。可以添加新设备、移除已有设备，或在 iga 之间切换。本质上，你可以写入一个以 “,” 分隔的设备名列表（或单个设备名），格式与这些文件的输出相同。你可以添加 “+” 或 “-” 作为前缀，以便简单地添加和移除设备。因此前缀 “+” 将你列表中的设备添加到已有设备之上，“-” 从已有设备中移除列出的设备，如果没有前缀，则用列出的设备替换所有已有设备。如果你移除设备，它们应当被关闭。如果你添加的设备已经是另一个 iga 的一部分，则会从那里移除并添加到新的 iga 中。

	示例：

	将 CRT 添加为 iga1 的输出设备::

		# echo +CRT > /proc/viafb/iga1/output_devices

	移除（关闭）DVP1 和 LVDS1 作为 iga2 的输出设备::

		# echo -DVP1,LVDS1 > /proc/viafb/iga2/output_devices

	用 CRT 替换 iga1 的所有输出设备::

		# echo CRT > /proc/viafb/iga1/output_devices
```

### 用 viafb 启动


```
    append = "video=viafb:viafb_mode=1024x768,viafb_bpp=32,viafb_refresh=85"
```

## VIA 帧缓冲模式


   :literal:
