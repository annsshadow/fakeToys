## EP93xx LCD 鎺у埗鍣ㄩ┍鍔。
EP93xx LCD 控制器可以同时驱动标准的桌面显示器和嵌入LCD 显示屏。如果你拥有标准桌面显示器，那么
```

	static struct ep93xxfb_mach_info some_board_fb_info = {
		.num_modes	= EP93XXFB_USE_MODEDB,
		.bpp		= 16,
	};

```
如果你拥有嵌入式 LCD 显示屏，则需要定义一段视```

	static struct fb_videomode some_board_video_modes[] = {
		{
			.name		= "some_lcd_name",
			/* Pixel clock, porches, etc */
		},
	};

```
注意像素时钟值以皮秒（pico-seconds）为单位。你可以使用 KHZ2PICOS 宏来转换像素时钟
值。大多数其它值以像素时钟为单位。更多细节参Documentation/fb/framebuffer.rst
你板卡的 ep93xxfb_mach_info 结构应类似于
```

	static struct ep93xxfb_mach_info some_board_fb_info = {
		.num_modes	= ARRAY_SIZE(some_board_video_modes),
		.modes		= some_board_video_modes,
		.default_mode	= &some_board_video_modes[0],
		.bpp		= 16,
	};

```
可以通过在下面添加以下内容来注册帧缓冲设```

	ep93xx_register_fb(&some_board_fb_info);

```
## 视频属性标
ep93xxfb_mach_info 结构有一flags 字段，可用于配置控制器。视频属性标志在 EP93xx
用户指南的第 7 节中有完整说明。可用的标志如下
=============================== ==========================================
EP93XXFB_PCLK_FALLING		Clock data on the falling edge of the
				pixel clock. The default is to clock
				data on the rising edge.

EP93XXFB_SYNC_BLANK_HIGH	Blank signal is active high. By
				default the blank signal is active low.

EP93XXFB_SYNC_HORIZ_HIGH	Horizontal sync is active high. By
				default the horizontal sync is active low.

EP93XXFB_SYNC_VERT_HIGH		Vertical sync is active high. By
				default the vertical sync is active high.
=============================== ==========================================

帧缓冲区的物理地址可以使用以下标志来控制：

=============================== ======================================
EP93XXFB_USE_SDCSN0		Use SDCSn[^0^] for the framebuffer. This
				is the default setting.

EP93XXFB_USE_SDCSN1		Use SDCSn[^1^] for the framebuffer.

EP93XXFB_USE_SDCSN2		Use SDCSn[^2^] for the framebuffer.

EP93XXFB_USE_SDCSN3		Use SDCSn[^3^] for the framebuffer.
=============================== ======================================

## 平台回调

EP93xx 帧缓冲驱动支持三个可选的平台回调：setup、teardown blank。setup teardown
函数分别在帧缓冲驱动被安装和移除时调用。blank 函数在显示器被消隐（blank）或取消消隐
（unblank）时调用
setup teardown 设备platform_device 结构作为参数传入。fb_info ep93xxfb_mach_info 结构可以```

	static int some_board_fb_setup(struct platform_device *pdev)
	{
		struct ep93xxfb_mach_info *mach_info = pdev->dev.platform_data;
		struct fb_info *fb_info = platform_get_drvdata(pdev);

		/* Board specific framebuffer setup */
	}

```
## 设置视频模式

```

	video=XRESxYRES[-BPP][@REFRESH]

```
如果 EP93xx 视频驱动是内建的，则视频模式```

	video=ep93xx-fb:800x600-16@60

```
中设置。如EP93xx 视频驱动是作为模块构建的，则视频模式```

	modprobe ep93xx-fb video=320x240

```
中设置## Screenpage 缺陷（bug
至少EP9315 上存在一个硅缺陷，会导致 VIDSCRNPAGE（帧缓冲物理偏移）的27 位被
固定为低电平。存```

	https://marc.info/?l=linux-arm-kernel&m=110061245502000&w=2

```
默认情况下，EP93xx 帧缓冲驱动会检查已分配的物理地址的第 27 位是否被设置。如果设置了则释放该内存并返回错误。可以通过将以下内容添加来禁用该检```

      ep93xx-fb.check_screenpage_bug=0

```
在某些情况下，可以重新配置你SDRAM 布局来规避此缺陷。详EP93xx 用户指南13 节