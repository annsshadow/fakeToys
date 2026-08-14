## 帧缓冲设备 API

最后修订：2011 年 6 月 21 日


### 0. 引言

本文档描述了应用用来与帧缓冲设备交互的帧缓冲 API。设备驱动与帧缓冲核心之间的内核内 API 不在描述范围内。

由于原始帧缓冲 API 缺乏文档，驱动的行为在细微（以及不那么细微）的方面存在差异。本文档描述了推荐的 API 实现，但应用应当准备好应对不同的行为。


### 1. 能力（Capabilities）

设备和驱动的能力在固定的屏幕信息中报告

```
  struct fb_fix_screeninfo {
	...
	__u16 capabilities;		/* see FB_CAP_*			*/
	...
  };

```
应用应当使用这些能力来查明它们可以从设备和驱动期待哪些特性。

- FB_CAP_FOURCC

驱动支持基于四字符码（FOURCC）的格式设置 API。当支持时，格式使用 FOURCC 配置，而不是手动指定颜色分量的布局。


### 2. 类型与视觉（Types and visuals）

像素以依赖于硬件的格式存储在内存中。应用需要了解像素存储格式，以便以硬件期望的格式将图像数据写入帧缓冲内存。

格式由帧缓冲类型和视觉（visual）描述。某些视觉需要额外的信息，这些信息存储在可变的屏幕信息 bits_per_pixel、grayscale、red、green、blue 和 transp 字段中。

视觉描述颜色信息如何被编码并组装以创建宏像素（macropixel）。类型描述宏像素如何存储在内存中。支持以下类型和视觉。

- FB_TYPE_PACKED_PIXELS

宏像素在单一平面中连续存储。如果每个宏像素的位数不是 8 的倍数，那么宏像素是填充到下一个 8 位的倍数还是打包进字节，取决于视觉。

行末可能存在填充，并通过固定屏幕信息的 line_length 字段报告。

- FB_TYPE_PLANES

宏像素被拆分到多个平面中。平面数等于每个宏像素的位数，第 i 个平面存储所有宏像素的第 i 位。

平面在内存中连续存放。

- FB_TYPE_INTERLEAVED_PLANES

宏像素被拆分到多个平面中。平面数等于每个宏像素的位数，第 i 个平面存储所有宏像素的第 i 位。

平面在内存中交错存放。交错因子（定义为属于不同平面的两个连续交错块起点之间的字节距离）存储在固定屏幕信息的 type_aux 字段中。

- FB_TYPE_FOURCC

宏像素按照存储在可变屏幕信息 grayscale 字段中的格式 FOURCC 标识符所描述的那样存储在内存中。

- FB_VISUAL_MONO01

像素为黑或白，并存储在由可变屏幕信息 bpp 字段指定的若干位（通常是一位）上。

黑像素由所有位设为 1 表示，白像素由所有位设为 0 表示。当每像素位数小于 8 时，多个像素被打包进一个字节。

FB_VISUAL_MONO01 目前仅与 FB_TYPE_PACKED_PIXELS 一起使用。

- FB_VISUAL_MONO10

像素为黑或白，并存储在由可变屏幕信息 bpp 字段指定的若干位（通常是一位）上。

黑像素由所有位设为 0 表示，白像素由所有位设为 1 表示。当每像素位数小于 8 时，多个像素被打包进一个字节。

FB_VISUAL_MONO10 目前仅与 FB_TYPE_PACKED_PIXELS 一起使用。

- FB_VISUAL_TRUECOLOR

像素被分解为红、绿、蓝分量，每个分量索引一个只读查找表以获得对应的值。查找表依赖于设备，并提供线性或非线性斜坡。

每个分量根据可变屏幕信息的 red、green、blue 和 transp 字段存储在一个宏像素中。

- FB_VISUAL_PSEUDOCOLOR 与 FB_VISUAL_STATIC_PSEUDOCOLOR

像素值被编码为索引，存入存储红、绿、蓝分量的颜色映射表（colormap）。对于 FB_VISUAL_STATIC_PSEUDOCOLOR 颜色映射表是只读的，对于 FB_VISUAL_PSEUDOCOLOR 是可读写的。

每个像素值存储在由可变屏幕信息 bits_per_pixel 字段报告的位数中。

- FB_VISUAL_DIRECTCOLOR

像素被分解为红、绿、蓝分量，每个分量索引一个可编程的查找表以获得对应的值。

每个分量根据可变屏幕信息的 red、green、blue 和 transp 字段存储在一个宏像素中。

- FB_VISUAL_FOURCC

像素按照存储在可变屏幕信息 grayscale 字段中的格式 FOURCC 标识符所描述的那样进行编码和解释。


### 3. 屏幕信息

屏幕信息由应用使用 FBIOGET_FSCREENINFO 和 FBIOGET_VSCREENINFO ioctl 查询。这些 ioctl 分别接受一个指向 fb_fix_screeninfo 和 fb_var_screeninfo 结构的指针。

struct fb_fix_screeninfo 存储关于帧缓冲设备及其当前格式的、与设备无关且不可更改的信息。这些信息不能被应用直接修改，但可以在驱动执行

```
  struct fb_fix_screeninfo {
	char id[16];			/* identification string eg "TT Builtin" */
	unsigned long smem_start;	/* Start of frame buffer mem */
					/* (physical address) */
	__u32 smem_len;			/* Length of frame buffer mem */
	__u32 type;			/* see FB_TYPE_*		*/
	__u32 type_aux;			/* Interleave for interleaved Planes */
	__u32 visual;			/* see FB_VISUAL_*		*/
	__u16 xpanstep;			/* zero if no hardware panning  */
	__u16 ypanstep;			/* zero if no hardware panning  */
	__u16 ywrapstep;		/* zero if no hardware ywrap    */
	__u32 line_length;		/* length of a line in bytes    */
	unsigned long mmio_start;	/* Start of Memory Mapped I/O   */
					/* (physical address) */
	__u32 mmio_len;			/* Length of Memory Mapped I/O  */
	__u32 accel;			/* Indicate to driver which	*/
					/*  specific chip/card we have	*/
	__u16 capabilities;		/* see FB_CAP_*			*/
	__u16 reserved[2];		/* Reserved for future compatibility */
  };

```
struct fb_var_screeninfo 存储关于帧缓冲设备、其当前格式和视频模式以及与设备无关且可更改的信息，以及

```
  struct fb_var_screeninfo {
	__u32 xres;			/* visible resolution		*/
	__u32 yres;
	__u32 xres_virtual;		/* virtual resolution		*/
	__u32 yres_virtual;
	__u32 xoffset;			/* offset from virtual to visible */
	__u32 yoffset;			/* resolution			*/

	__u32 bits_per_pixel;		/* guess what			*/
	__u32 grayscale;		/* 0 = color, 1 = grayscale,	*/
					/* >1 = FOURCC			*/
	struct fb_bitfield red;		/* bitfield in fb mem if true color, */
	struct fb_bitfield green;	/* else only length is significant */
	struct fb_bitfield blue;
	struct fb_bitfield transp;	/* transparency			*/

	__u32 nonstd;			/* != 0 Non standard pixel format */

	__u32 activate;			/* see FB_ACTIVATE_*		*/

	__u32 height;			/* height of picture in mm    */
	__u32 width;			/* width of picture in mm     */

	__u32 accel_flags;		/* (OBSOLETE) see fb_info.flags */

	/* Timing: All values in pixclocks, except pixclock (of course) */
	__u32 pixclock;			/* pixel clock in ps (pico seconds) */
	__u32 left_margin;		/* time from sync to picture	*/
	__u32 right_margin;		/* time from picture to sync	*/
	__u32 upper_margin;		/* time from sync to picture	*/
	__u32 lower_margin;
	__u32 hsync_len;		/* length of horizontal sync	*/
	__u32 vsync_len;		/* length of vertical sync	*/
	__u32 sync;			/* see FB_SYNC_*		*/
	__u32 vmode;			/* see FB_VMODE_*		*/
	__u32 rotate;			/* angle we rotate counter clockwise */
	__u32 colorspace;		/* colorspace for FOURCC-based modes */
	__u32 reserved[4];		/* Reserved for future compatibility */
  };

```
要修改可变信息，应用调用 FBIOPUT_VSCREENINFO ioctl，并传入一个指向 fb_var_screeninfo 结构的指针。如果调用成功，驱动将相应地更新固定屏幕信息。

应用不应手动填充整个 fb_var_screeninfo 结构，而应调用 FBIOGET_VSCREENINFO ioctl 并仅修改它们关心的字段。


### 4. 格式配置

帧缓冲设备提供两种方式来配置帧缓冲格式：传统 API 和基于 FOURCC 的 API。


传统 API 长期以来一直是唯一的帧缓冲格式配置 API，因此被应用广泛使用。对于 RGB 和灰度格式以及传统的非标准格式，它是推荐给应用使用的 API。

要选择一种格式，应用将 fb_var_screeninfo 的 bits_per_pixel 字段设为所需的帧缓冲深度。最大为 8 的值通常会映射到单色、灰度或伪彩色视觉，但这并不强制要求。

- 对于灰度格式，应用将 grayscale 字段设为 1。red、blue、green 和 transp 字段必须由应用设为 0，并被驱动忽略。驱动必须将 red、blue 和 green 的偏移填为 0，长度填为 bits_per_pixel 的值。

- 对于伪彩色格式，应用将 grayscale 字段设为 0。red、blue、green 和 transp 字段必须由应用设为 0，并被驱动忽略。驱动必须将 red、blue 和 green 的偏移填为 0，长度填为 bits_per_pixel 的值。

- 对于真彩色（truecolor）和直接彩色（directcolor）格式，应用将 grayscale 字段设为 0，并将 red、blue、green 和 transp 字段设为描述

```
    struct fb_bitfield {
	__u32 offset;			/* beginning of bitfield	*/
	__u32 length;			/* length of bitfield		*/
	__u32 msb_right;		/* != 0 : Most significant bit is */
					/* right */
    };

  像素值为 bits_per_pixel 宽，并被拆分为不重叠的红、绿、蓝和 alpha（透明度）分量。每个分量在像素值中的位置和大小由 fb_bitfield 的 offset 和 length 字段描述。偏移从右侧计算。

  像素总是存储在整数个字节中。如果每像素位数不是 8 的倍数，像素值被填充到下一个 8 位的倍数。

```
格式配置成功后，驱动根据所选格式更新 fb_fix_screeninfo 的 type、visual 和 line_length 字段。


基于 FOURCC 的 API 用四字符码（FOURCC）替代格式描述。FOURCC 是抽象标识符，在不显式描述格式的情况下唯一地定义一个格式。这是唯一支持 YUV 格式的 API。也鼓励驱动为 RGB 和灰度格式实现基于 FOURCC 的 API。

支持基于 FOURCC 的 API 的驱动通过在 fb_fix_screeninfo 的 capabilities 字段中设置 FB_CAP_FOURCC 位来报告此能力。

FOURCC 定义位于 linux/videodev2.h 头文件中。然而，尽管以 V4L2_PIX_FMT_ 前缀开头，它们并不局限于 V4L2，也不要求使用 V4L2 子系统。FOURCC 文档可在 Documentation/userspace-api/media/v4l/pixfmt.rst 中获取。

要选择一种格式，应用将 grayscale 字段设为所需的 FOURCC。对于 YUV 格式，它们还应通过将 colorspace 字段设为 linux/videodev2.h 中列出并在 Documentation/userspace-api/media/v4l/colorspaces.rst 中记录的某个色彩空间来选择适当的 colorspace。

基于 FOURCC 的 API 不使用 red、green、blue 和 transp 字段。出于向前兼容的原因，应用必须将那些字段清零，驱动必须忽略它们。除 0 以外的值可能在未来的扩展中获得含义。

格式配置成功后，驱动根据所选格式更新 fb_fix_screeninfo 的 type、visual 和 line_length 字段。type 和 visual 字段分别设为 FB_TYPE_FOURCC 和 FB_VISUAL_FOURCC。
