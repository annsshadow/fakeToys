## 帧缓冲设备内部机制


这是关于帧缓冲设备内部机制的部分文档的初步起点。

Authors:

- Geert Uytterhoeven <geert@linux-m68k.org>, 1998 年 7 月 21 日
- James Simmons <jsimmons@user.sf.net>, 2002 年 11 月 26 日

--------------------------------------------------------------------------------

## 帧缓冲设备 API 使用的结构


以下结构在帧缓冲设备的运作中发挥作用。它们定义于 <linux/fb.h>。

1. 内核之外（用户空间）

  - struct fb_fix_screeninfo

    关于帧缓冲设备与特定视频模式的、设备无关且不可改变的信息。可以通过
    FBIOGET_FSCREENINFO ioctl 获取。

  - struct fb_var_screeninfo

    关于帧缓冲设备与特定视频模式的、设备无关且可改变的信息。可以通过
    FBIOGET_VSCREENINFO ioctl 获取，并通过 FBIOPUT_VSCREENINFO ioctl 更新。如果只想
    平移（pan）屏幕，可以使用 FBIOPAN_DISPLAY ioctl。

  - struct fb_cmap

    设备无关的调色板（colormap）信息。可以使用 FBIOGETCMAP 与 FBIOPUTCMAP ioctl
    获取和设置调色板。


2. 内核之内

  - struct fb_info

    关于某个特定帧缓冲设备实例（插槽号、板卡地址等）的通用信息、API 与底层信息。

  - struct `par`

    设备相关的信息，唯一定义了这块特定硬件的视频模式。


## 帧缓冲设备 API 使用的视觉类型（Visuals）


### 单色（FB_VISUAL_MONO01 与 FB_VISUAL_MONO10）


每个像素非黑即白。


### 伪彩色（FB_VISUAL_PSEUDOCOLOR 与 FB_VISUAL_STATIC_PSEUDOCOLOR）


整个像素值被送入一个可编程查找表，该表为每个可能的像素值提供一个颜色（包括红、绿、
蓝强度），并显示该颜色。


### 真彩色（FB_VISUAL_TRUECOLOR）


像素值被拆分为红、绿、蓝字段。


### 直接彩色（FB_VISUAL_DIRECTCOLOR）


像素值被拆分为红、绿、蓝字段，每个字段分别在独立的红、绿、蓝查找表中查找。


### 灰度显示


灰度与静态灰度是伪彩色与静态伪彩色的特殊变体，其中红、绿、蓝分量始终彼此相等。
