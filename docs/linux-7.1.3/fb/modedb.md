## modedb 默认视频模式支持


目前所有帧缓冲设备驱动都拥有各自的视频模式数据库，这既混乱又浪费资源。modedb
的主要想法是拥有

  - 一个用于探测视频模式的例程，可被所有帧缓冲设备使用
  - 一个带有相当数量标准视频模式（取自 XFree86）的通用视频模式数据  - 为需要非标准模式的图形硬件提供自定义模式数据库的可能性，例如 amifb     Mac 帧缓冲驱动（它们使用 macmodes.c
当帧缓冲设备收到一个它不认识的 video= 选项时，它应将其视为一个视频模式选项如果video= 选项中没有指定帧缓冲设备，fbmem 将其视为一个全局视频模式选项
```

    <xres>x<yres>[M][R][-<bpp>][@<refresh>][i][m][eDd]
    <name>[-<bpp>][@<refresh>]

```
其中 <xres>yres>bpp> <refresh> 是十进制数字name> 是一个字符串方括号中的内容是可选项
```

  - NSTC: 480i 输出，采CCIR System-M 电视模式NTSC 彩色编码
  - NTSC-J: 480i 输出，采CCIR System-M 电视模式、NTSC 彩色编码，以    等于消隐电平的黑电平  - PAL: 576i 输出，采CCIR System-B 电视模式PAL 彩色编码
  - PAL-M: 480i 输出，采CCIR System-M 电视模式PAL 彩色编码

```
如果mode_option 参数中指定了 'M'（位<yres> 之后bpp> <refresh>
之前，如果指定的话），时序将使用 VESA(TM) 协调视频时序（Coordinated Video
Timings）计算，而不是从表中查找模式。如果指定了 'R'，则对数字显示器执行
“缩减消隐”（reduced blanking）计算。如果指定了 'i'，则针对隔行模式计算。如指定'm'，则在计算中加上边距（xres 1.8% 向下取整8 像素，以yres 1.8%）
       示例用法024x768M@60m - 带边距的 CVT 时序

DRM 驱动还添加了用于启用或禁用输出的选项
'e' 将强制启用显示，即如果检测到有显示器连接，它会覆盖检测结果D' 将强制启显示并使用数字输出。这对于同时具有模拟和数字信号的输出（例HDMI DVI-I很有用。对于其它输出，它的行为类似'e'。如果指定了 'd'，则输出被禁用
你还可以额外指定这些选项匹配到哪个输出```

    video=VGA-1:1280x1024@60me

```
```

    video=LVDS-1:d video=HDMI-1:D

```
选项也可以在模式之后传递，使用逗号作为分隔符
       示例用法20x480,rotate=180 - 720x480 模式，旋180 
```

  - margin_top、margin_bottom、margin_left、margin_right（整数）    边距中的像素数，通常用于处理电视上的过扫  - reflect_x（布尔值）：在 X 轴上执行轴对  - reflect_y（布尔值）：在 Y 轴上执行轴对  - rotate（整数）：将初始帧缓冲旋x 度。有效值为 0080 270  - tv_mode：模拟电视模式。为 "NTSC"NTSC-443"NTSC-J"PAL"    "PAL-M"PAL-N" "SECAM" 之一  - panel_orientation，为 "normal"upside_down"left_side_up"     "right_side_up" 之一。仅适用KMS 驱动，它kms 连接器上设置
    “panel orientation”属性，作为kms 用户的提示
```

-----------------------------------------------------------------------------

## 什么是 VESA(TM) 协调视频时序（CVT）？


来自 VESA(TM) 网站
     "CVT 的目的是为计算机显示产品（既包括采用 CRT 的，也包括采用其它显      技术的）提供一套一致且协调的标准格式、显示刷新率和时序规范生成方法      CVT 的意图是为信号源和显示制造商提供一套通用工具，使新时序能够以一      的方式开发，从而确保更大的兼容性

这是 VESA(TM) 批准的第三个关于视频时序的标准。第一个是离散视频时序（DVT），
它是 VESA(TM) 批准的一组预定义模式。第二个是通用时序公式（GTF），它是一算法，用于在给定像素时钟、水平同步频率或垂直刷新率的情况下计算时序
GTF 的局限在于它主要CRT 显示器设计。由于其高消隐要求，它人为地提高了像时钟。这对于具有高数据速率、要求尽可能保留像素时钟的数字显示接口是不合适的此外，GTF 没有考虑显示器的宽高比
CVT 解决了这些限制。如果用CRT，所使用的公式是GTF 做了少许修改的派生如果用于数字显示器，则可以使用“缩减消隐”计算
从帧缓冲子系统的角度来看，每当显示制造商发布新模式时，无需将新格式添加到全局
模式数据库。为 CVT 指定参数对大多数（如果不是全部）相对较新CRT 显示器，以及
可能对大多数平板显示器都有效，前提是指定了“缩减消隐”计算。（显示CVT 兼容可由EDID 确定。EDID 1.3 版本有额外的 128 字节块，其中放置了额外的时序
信息。截至目前，该层尚不支持解析这些额外块。）

```

    <pix>M<a>[-R]

    where: pix = total amount of pixels in MB (xres x yres)
	   M   = always present
	   a   = aspect ratio (3 - 4:3; 4 - 5:4; 9 - 15:9, 16:9; A - 16:10)
	  -R   = reduced blanking

	  example:  .48M3-R - 800x600 with reduced blanking

```
注意：VESA(TM) 对什么是标准 CVT 时序有若干限制：

      - 宽高比只能是上述值之一
      - 可接受的刷新率仅5000 85 Hz
      - 如果使用缩减消隐，刷新率必须60Hz

如果不满足上述任一条件，内核会打印一条警告，但时序仍会被计算
-----------------------------------------------------------------------------

```

  int __init fb_find_mode(struct fb_var_screeninfo *var,
			  struct fb_info *info, const char *mode_option,
			  const struct fb_videomode *db, unsigned int dbsize,
			  const struct fb_videomode *default_mode,
			  unsigned int default_bpp)

```
其中 db/dbsize 是你的非标准视频模式数据库，或传NULL 以使用标准视频模数据库
fb_find_mode() 首先尝试指定的视频模式（或任何匹配的模式，例如可以存在多640x480 模式，每个都会被尝试）。如果失败，则尝试默认模式。如果也失败，它会遍所有模式
```

    video=<driver>:<xres>x<yres>[-<bpp>][@refresh]

```
其中 <driver> 是下表中某个名称。有效的默认模式可以drivers/video/fbdev/core/modedb.c 中找到。请查阅你的驱动文档```

    Drivers that support modedb boot options
    Boot Name	  Cards Supported

    amifb	- Amiga chipset frame buffer
    aty128fb	- ATI Rage128 / Pro frame buffer
    atyfb	- ATI Mach64 frame buffer
    pm2fb	- Permedia 2/2V frame buffer
    pm3fb	- Permedia 3 frame buffer
    sstfb	- Voodoo 1/2 (SST1) chipset frame buffer
    tdfxfb	- 3D Fx frame buffer
    tridentfb	- Trident (Cyber)blade chipset frame buffer
    vt8623fb	- VIA 8623 frame buffer

```
顺便说一句，目前只有少数 fb 驱动使用这个。其它驱动会陆续跟进（欢迎发送补丁）DRM 驱动也支持这个