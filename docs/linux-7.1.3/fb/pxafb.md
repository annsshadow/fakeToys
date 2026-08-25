## PXA25x LCD 鎺у埗鍣ㄩ┍鍔。

该驱动支持以下选项，模块方式下通过 `options=<OPTIONS>`，内建方式下通过 `video=pxafb:<OPTIONS>`
```

	modprobe pxafb options=vmem:2M,mode:640x480-8,passive

```
```

	video=pxafb:vmem:2M,mode:640x480-8,passive

```
vmem: VIDEO_MEM_SIZE

	要分配的显存大小（可加后缀 K M 表示千字节或兆字节）

mode:XRESxYRES[-BPP]

	XRES == LCCR1_PPL + 1

	YRES == LLCR2_LPP + 1

		以像素为单位的显示分辨率

	BPP == 位深。合法值为 1 16
pixclock:PIXCLOCK

	像素时钟，单位为皮秒

left:LEFT == LCCR1_BLW + 1

right:RIGHT == LCCR1_ELW + 1

hsynclen:HSYNC == LCCR1_HSW + 1

upper:UPPER == LCCR2_BFW

lower:LOWER == LCCR2_EFR

vsynclen:VSYNC == LCCR2_VSW + 1

	显示边距与同步时
color | mono => LCCR0_CMS

	嗯…
active | passive => LCCR0_PAS

	主动（TFT）或被动（STN）显
single | dual => LCCR0_SDS

	单面板或双面板被动显
4pix | 8pix => LCCR0_DPD

	4 8 像素单色单面板数
hsync:HSYNC, vsync:VSYNC

	水平与垂直同步 => 低电平有效，1 => 高电平有效
dpc:DPC

	双倍像素时钟=>真，0=>
outputen:POLARITY

	输出使能极性 => 低电平有效，1 => 高电平有
pixclockpol:POLARITY

	像素时钟极	0 => 下降沿，1 => 上升

## PXA27x 鍙婃洿楂樼増鏈?LCD 鎺у埗鍣ㄧ殑鍙犲姞灞傛敮鎸。

  PXA27x 及更高版本的处理器在基础帧缓冲之上支overlay1 overlay2（当然也可以位于基础层之下）。它们支持带调色板与无调色板RGB 格式，以YUV 格式（仅overlay2 上可用）。这些叠加层拥有专用DMA 通道，行为方式与帧缓冲类似
  然而，这些叠加层帧缓冲与普通帧缓冲之间存在一些差异，如下所示：

  1. 叠加层可以起始于基础帧缓冲中 32 位字对齐的位置，这意味着它们具有一个起始坐(x, y)。该信息被编码进 `var->nonstd`（注意，`var->xoffset` `var->yoffset` 并非用于此目的）
  2. 叠加层帧缓冲根据指定的内容动态分
```

	var->xres_virtual * var->yres_virtual * bpp

     bpp = 16 -- for RGB565 or RGBT555

     bpp = 24 -- for YUV444 packed

     bpp = 24 -- for YUV444 planar

     bpp = 16 -- for YUV422 planar (1 pixel = 1 Y + 1/2 Cb + 1/2 Cr)

     bpp = 12 -- for YUV420 planar (1 pixel = 1 Y + 1/4 Cb + 1/4 Cr)

     NOTE:

     a. 叠加层不支持 x 方向平移，因	var->xres_virtual 将始终等var->xres

     b. 叠加层的行长度必须位32 位字边界上，
	对于 YUV planar 模式，这是针对每像素位数最少的
	分量而言的要求，例如对于 YUV420，一个像素的 Cr 分量
	实际2 位，这意味着行长度应16 像素的整数
     c. 起始水平位置（XPOS）应位于 32 位字边界上，
	否则 fb_check_var() 将直接失败
     d. 叠加层的矩形区域应位于基础平面之内	否则失败

     Applications should follow the sequence below to operate an overlay
     framebuffer:

	 a. open("/dev/fb[1-2]", ...)
	 b. ioctl(fd, FBIOGET_VSCREENINFO, ...)
	 c. modify 'var' with desired parameters:

	    1) var->xres and var->yres
	    2) 如果需要更多内存（通常用于双缓冲）	       增大 var->yres_virtual
	    3) var->nonstd 用于起始 (x, y) 与颜色格	    4) 若使RGB 模式，则设置 var->{red, green, blue, transp}

	 d. ioctl(fd, FBIOPUT_VSCREENINFO, ...)
	 e. ioctl(fd, FBIOGET_FSCREENINFO, ...)
	 f. mmap
	 g. ...

  3. 对于 YUV planar 格式，帧缓冲框架实际上并不支持，应用程序必须自行处理各分量在帧缓冲中的偏移与长度
  4. `var->nonstd` 用于传递起(x, y) 位置与颜色格式，详细的位域如下所:

      31                23  20         10          0
       +-----------------+---+----------+----------+
       |  ... unused ... |FOR|   XPOS   |   YPOS   |
       +-----------------+---+----------+----------+

     FOR  - 颜色格式，由 pxafb.h 中的 OVERLAY_FORMAT_* 定义

	  - 0 - RGB
	  - 1 - YUV444 PACKED
	  - 2 - YUV444 PLANAR
	  - 3 - YUV422 PLANAR
	  - 4 - YUR420 PLANAR

     XPOS - 起始水平位置

     YPOS - 起始垂直位置

```
