######## V4L2 API 的变

本页记录 V4L2（Video4Linux2）用户空API 的演进与变更历史，按时间顺序整理了自 1998 V4L2 取代 V4L 以来各版本在接口、ioctl 与数据格式上的关键改动，供驱动开发者与应用程序作者追API 的演变


Soon after the V4L API was added to the kernel it was criticised as too
inflexible. In August 1998 Bill Dirks proposed a number of improvements
and began to work on documentation, example drivers and applications.
With the help of other volunteers this eventually became the V4L2 API,
not just an extension but a replacement for the V4L API. However it took
another four years and two stable kernel releases until the new API was
finally accepted for inclusion into the kernel in its present form.

V4L API 加入内核后不久，人们便批评它过于
缺乏灵活性998 8 月，Bill Dirks 提出了一系列改进建议
并开始着手编写文档、示例驱动程序以及应用程序
在其他志愿者的帮助下，这些工作最终演变成V4L2 API
它不仅仅V4L API 的扩展，而是其替代品。然而又过了
四年以及两次稳定的内核发布，这个API 才最终以
目前的形式被内核接受并合入

## Early Versions

## 早期版本


1998-08-20: First version.

1998-08-20：首个版本

1998-08-27: The `select()` function was introduced.

1998-08-27：引入了 `select()` 函数

1998-09-10: New video standard interface.

1998-09-10：新的视频标准接口

1998-09-18: The `VIDIOC_NONCAP` ioctl was replaced by the otherwise
meaningless `O_TRUNC` `open()` flag, and the
aliases `O_NONCAP` and `O_NOIO` were defined. Applications can set
this flag if they intend to access controls only, as opposed to capture
applications which need exclusive access. The `VIDEO_STD_XXX`
identifiers are now ordinals instead of flags, and the
`video_std_construct()` helper function takes id and
transmission arguments.

1998-09-18：`VIDIOC_NONCAP` ioctl 被一个本
无意义的 `O_TRUNC` `open()` 标志所取代，同时定义了
别名 `O_NONCAP` `O_NOIO`。如果应用程序仅打算访问
控制项（与需要独占访问的采集应用程序相对），则可以设
该标志。`VIDEO_STD_XXX`
标识符现在是序数而非标志位，
`video_std_construct()` 辅助函数接受 id 
传输（transmission）参数

1998-09-28: Revamped video standard. Made video controls individually
enumerable.

1998-09-28：重塑了视频标准。使视频控制项可被单
枚举

1998-10-02: The `id` field was removed from
struct `video_standard` and the color subcarrier fields were
renamed. The VIDIOC_QUERYSTD ioctl was
renamed to VIDIOC_ENUMSTD,
VIDIOC_G_INPUT <VIDIOC_G_INPUT> to
VIDIOC_ENUMINPUT. A first draft of the
Codec API was released.

1998-10-02：从 struct `video_standard` 中移除了 `id` 字段
颜色副载波（color subcarrier）字段被重命名。VIDIOC_QUERYSTD ioctl 
重命名为 VIDIOC_ENUMSTD
VIDIOC_G_INPUT <VIDIOC_G_INPUT> 被重命名
VIDIOC_ENUMINPUT。Codec API 的首个草案发布

1998-11-08: Many minor changes. Most symbols have been renamed. Some
material changes to struct v4l2_capability.

1998-11-08：大量细微改动。大多数符号被重命名。struct v4l2_capability
有一些实质性改动

1998-11-12: The read/write direction of some ioctls was misdefined.

1998-11-12：某ioctl 的读/写方向定义有误

1998-11-14: `V4L2_PIX_FMT_RGB24` changed to `V4L2_PIX_FMT_BGR24`,
and `V4L2_PIX_FMT_RGB32` changed to `V4L2_PIX_FMT_BGR32`. Audio
controls are now accessible with the
VIDIOC_G_CTRL <VIDIOC_G_CTRL> and
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls under names starting
with `V4L2_CID_AUDIO`. The `V4L2_MAJOR` define was removed from
`videodev.h` since it was only used once in the `videodev` kernel
module. The `YUV422` and `YUV411` planar image formats were added.

1998-11-14：`V4L2_PIX_FMT_RGB24` 改为 `V4L2_PIX_FMT_BGR24`
`V4L2_PIX_FMT_RGB32` 改为 `V4L2_PIX_FMT_BGR32`。音
控制项现在可通过
VIDIOC_G_CTRL <VIDIOC_G_CTRL> 鍜。
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 浠。
`V4L2_CID_AUDIO` 开头的名称访问。`V4L2_MAJOR` 宏定
`videodev.h` 中移除，因为它仅`videodev` 内核
模块中被使用过一次。`YUV422` `YUV411` 平面图像格式被加入

1998-11-28: A few ioctl symbols changed. Interfaces for codecs and video
output devices were added.

1998-11-28：少ioctl 符号发生变化。新增了用于编解码器（codec）和视频
输出设备的接口

1999-01-14: A raw VBI capture interface was added.

1999-01-14：新增了原始 VBI 采集接口

1999-01-19: The `VIDIOC_NEXTBUF` ioctl was removed.

1999-01-19：移除了 `VIDIOC_NEXTBUF` ioctl

## V4L2 Version 0.16 1999-01-31

## V4L2 0.16 鐗?1999-01-31


1999-01-27: There is now one QBUF ioctl, VIDIOC_QWBUF and VIDIOC_QRBUF
are gone. VIDIOC_QBUF takes a v4l2_buffer as a parameter. Added
digital zoom (cropping) controls.

1999-01-27：现在只有一QBUF ioctl，VIDIOC_QWBUF VIDIOC_QRBUF
已废弃。VIDIOC_QBUF v4l2_buffer 作为参数。新增了
数字缩放（裁剪）控制项

## V4L2 Version 0.18 1999-03-16

## V4L2 0.18 鐗?1999-03-16


Added a v4l to V4L2 ioctl compatibility layer to videodev.c. Driver
writers, this changes how you implement your ioctl handler. See the
Driver Writer's Guide. Added some more control id codes.

videodev.c 中新增了 v4l V4L2 ioctl 兼容层。驱
开发者请注意，这改变ioctl 处理函数的实现方式。请参阅
《驱动开发者指南》。新增了更多控制 id 代码

## V4L2 Version 0.19 1999-06-05

## V4L2 0.19 鐗?1999-06-05


1999-03-18: Fill in the category and catname fields of v4l2_queryctrl
objects before passing them to the driver. Required a minor change to
the VIDIOC_QUERYCTRL handlers in the sample drivers.

1999-03-18：在v4l2_queryctrl 对象传递给驱动之前，需要先
填好category catname 字段。这需要对
示例驱动中的 VIDIOC_QUERYCTRL 处理函数做小幅改动

1999-03-31: Better compatibility for v4l memory capture ioctls. Requires
changes to drivers to fully support new compatibility features, see
Driver Writer's Guide and v4l2cap.c. Added new control IDs:
V4L2_CID_HFLIP, _VFLIP. Changed V4L2_PIX_FMT_YUV422P to _YUV422P,
and _YUV411P to _YUV411P.

1999-03-31：改善了v4l 内存采集 ioctl 的兼容性。需
修改驱动以完整支持新的兼容特性，请参阅《驱动开发者指南》和
v4l2cap.c。新增了控制 ID：V4L2_CID_HFLIP、_VFLIP。将
V4L2_PIX_FMT_YUV422P 改为 _YUV422P，将 _YUV411P 改为 _YUV411P

1999-04-04: Added a few more control IDs.

1999-04-04：新增了更多控制 ID

1999-04-07: Added the button control type.

1999-04-07：新增了按钮（button）控制类型

1999-05-02: Fixed a typo in videodev.h, and added the
V4L2_CTRL_FLAG_GRAYED (later V4L2_CTRL_FLAG_GRABBED) flag.

1999-05-02：修正了 videodev.h 中的一个拼写错误，并新增了
V4L2_CTRL_FLAG_GRAYED（后改名V4L2_CTRL_FLAG_GRABBED）标志

1999-05-20: Definition of VIDIOC_G_CTRL was wrong causing a
malfunction of this ioctl.

1999-05-20：VIDIOC_G_CTRL 的定义有误，导致ioctl 工作异常

1999-06-05: Changed the value of V4L2_CID_WHITENESS.

1999-06-05：更改了 V4L2_CID_WHITENESS 的值

## V4L2 Version 0.20 (1999-09-10)

## V4L2 0.20 版（1999-09-10


Version 0.20 introduced a number of changes which were *not backward
compatible* with 0.19 and earlier versions. Purpose of these changes was
to simplify the API, while making it more extensible and following
common Linux driver API conventions.

0.20 版引入了许多0.19 及更早版不向后兼的改动。这些改动的
目的是简API，同时使其更具可扩展性，并遵
通用Linux 驱动 API 惯例

1. Some typos in `V4L2_FMT_FLAG` symbols were fixed. struct v4l2_clip
   was changed for compatibility with v4l. (1999-08-30)

1. 修正`V4L2_FMT_FLAG` 符号中的一些拼写错误。为兼容 v4l
   修改struct v4l2_clip。（1999-08-30

2. `V4L2_TUNER_SUB_LANG1` was added. (1999-09-05)

2. 新增`V4L2_TUNER_SUB_LANG1`。（1999-09-05

3. All ioctl() commands that used an integer argument now take a pointer
   to an integer. Where it makes sense, ioctls will return the actual
   new value in the integer pointed to by the argument, a common
   convention in the V4L2 API. The affected ioctls are: VIDIOC_PREVIEW,
   VIDIOC_STREAMON, VIDIOC_STREAMOFF, VIDIOC_S_FREQ,
   VIDIOC_S_INPUT, VIDIOC_S_OUTPUT, VIDIOC_S_EFFECT. For example

3. 所有使用整数参数的 ioctl() 命令现在都改为接受一个指向整数的
   指针。在合理的情况下，ioctl 会通过参数所指向的整数返
   实际的新值，这是 V4L2 API 中的通用惯例。受影响ioctl 包括
   VIDIOC_PREVIEW、VIDIOC_STREAMON、VIDIOC_STREAMOFF、VIDIOC_S_FREQ
   VIDIOC_S_INPUT、VIDIOC_S_OUTPUT、VIDIOC_S_EFFECT。例

   .. code-block:: c

       err = ioctl (fd, VIDIOC_XXX, V4L2_XXX);

   becomes

   变为

   .. code-block:: c

       int a = V4L2_XXX; err = ioctl(fd, VIDIOC_XXX, &a);

4. All the different get- and set-format commands were swept into one
   VIDIOC_G_FMT <VIDIOC_G_FMT> and
   VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl taking a union and a
   type field selecting the union member as parameter. Purpose is to
   simplify the API by eliminating several ioctls and to allow new and
   driver private data streams without adding new ioctls.

4. 所有不同的获取/设置格式命令被合并为单一
   VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜。
   VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl，其参数为一个联合（union
   以及一个用于选择联合成员type 字段。其目的是通过消除若干
   ioctl 来简API，并在不新增 ioctl 的前提下支持新的以及
   驱动私有的数据流

   This change obsoletes the following ioctls: `VIDIOC_S_INFMT`,
   `VIDIOC_G_INFMT`, `VIDIOC_S_OUTFMT`, `VIDIOC_G_OUTFMT`,
   `VIDIOC_S_VBIFMT` and `VIDIOC_G_VBIFMT`. The image format
   struct v4l2_format was renamed to struct v4l2_pix_format, while
   struct v4l2_format is now the enveloping structure
   for all format negotiations.

   这一改动使以ioctl 过时：`VIDIOC_S_INFMT`
   `VIDIOC_G_INFMT`、`VIDIOC_S_OUTFMT`、`VIDIOC_G_OUTFMT`
   `VIDIOC_S_VBIFMT` `VIDIOC_G_VBIFMT`。图像格
   struct v4l2_format 被重命名struct v4l2_pix_format，
   struct v4l2_format 现在成为所有格式协商的
   外层封装结构

5. Similar to the changes above, the `VIDIOC_G_PARM` and
   `VIDIOC_S_PARM` ioctls were merged with `VIDIOC_G_OUTPARM` and
   `VIDIOC_S_OUTPARM`. A `type` field in the new struct v4l2_streamparm
   selects the respective union member.

5. 与上面的改动类似，`VIDIOC_G_PARM` 
   `VIDIOC_S_PARM` ioctl 涓?`VIDIOC_G_OUTPARM` 鍜。
   `VIDIOC_S_OUTPARM` 合并。新 struct v4l2_streamparm 中的
   `type` 字段用于选择相应的联合成员

   This change obsoletes the `VIDIOC_G_OUTPARM` and
   `VIDIOC_S_OUTPARM` ioctls.

   这一改动`VIDIOC_G_OUTPARM` `VIDIOC_S_OUTPARM` ioctl 过时

6. Control enumeration was simplified, and two new control flags were
   introduced and one dropped. The `catname` field was replaced by a
   `group` field.

6. 简化了控制枚举，引入了两个新的控制标志并去掉了一个。`catname` 字段
   `group` 字段取代

   Drivers can now flag unsupported and temporarily unavailable controls
   with `V4L2_CTRL_FLAG_DISABLED` and `V4L2_CTRL_FLAG_GRABBED`
   respectively. The `group` name indicates a possibly narrower
   classification than the `category`. In other words, there may be
   multiple groups within a category. Controls within a group would
   typically be drawn within a group box. Controls in different
   categories might have a greater separation, or may even appear in
   separate windows.

   驱动现在可以使用 `V4L2_CTRL_FLAG_DISABLED` 
   `V4L2_CTRL_FLAG_GRABBED` 分别标记不受支持和临时不可用
   控制项。`group` 名称表示可能`category` 更细
   分类。换句话说，一category 中可能有多个 group。同一 group 内的
   控制项通常会被绘制在一个分组框（group box）中。不category 中的
   控制项可能间隔更大，甚至可能出现在独立的窗口中

7. The struct v4l2_buffer `timestamp` was
   changed to a 64 bit integer, containing the sampling or output time
   of the frame in nanoseconds. Additionally timestamps will be in
   absolute system time, not starting from zero at the beginning of a
   stream. The data type name for timestamps is stamp_t, defined as a
   signed 64-bit integer. Output devices should not send a buffer out
   until the time in the timestamp field has arrived. I would like to
   follow SGI's lead, and adopt a multimedia timestamping system like
   their UST (Unadjusted System Time). See
   http://web.archive.org/web/\*/http://reality.sgi.com
   /cpirazzi_engr/lg/time/intro.html. UST uses timestamps that are
   64-bit signed integers (not struct timeval's) and given in nanosecond
   units. The UST clock starts at zero when the system is booted and
   runs continuously and uniformly. It takes a little over 292 years for
   UST to overflow. There is no way to set the UST clock. The regular
   Linux time-of-day clock can be changed periodically, which would
   cause errors if it were being used for timestamping a multimedia
   stream. A real UST style clock will require some support in the
   kernel that is not there yet. But in anticipation, I will change the
   timestamp field to a 64-bit integer, and I will change the
   v4l2_masterclock_gettime() function (used only by drivers) to
   return a 64-bit integer.

7. struct v4l2_buffer 鐨?`timestamp` 琚。
   改为 64 位整数，以纳秒为单位保存帧的采样或输出时间。此外，
   时间戳将采用绝对系统时间，而不是从流开始时的零算起。时间戳
   数据类型名为 stamp_t，定义为有符64 位整数。输出设备在
   timestamp 字段所表示的时间到来之前不应发出缓冲区。我希望
   效仿 SGI 的做法，采用类似UST（Unadjusted System Time
   未校正系统时间）的多媒体时间戳系统。参
   http://web.archive.org/web/\*/http://reality.sgi.com
   /cpirazzi_engr/lg/time/intro.html。UST 使用 64 位有符号整数
   （而非 struct timeval）作为时间戳，单位为纳秒。UST 时钟
   系统启动时从零开始，连续且均匀地运行。UST 溢出需要略多于 292 年
   UST 时钟无法被设置。普通的 Linux 日时钟（time-of-day clock）会
   被周期性地更改，若将其用于多媒体流的时间戳则会导致错误。真正的
   UST 风格时钟需要内核中尚不存在的某些支持。但作为预期，我会将
   timestamp 字段改为 64 位整数，并将
   v4l2_masterclock_gettime() 函数（仅驱动使用）改
   返回一64 位整数

8. A `sequence` field was added to struct v4l2_buffer. The `sequence`
   field counts captured frames, it is ignored by output devices. When a
   capture driver drops a frame, the sequence number of that frame is skipped.

8. struct v4l2_buffer 中新增了 `sequence` 字段。`sequence`
   字段对采集到的帧进行计数，输出设备会忽略它。当采集驱动
   丢弃某一帧时，该帧的序号会被跳过

## V4L2 Version 0.20 incremental changes

## V4L2 0.20 版的增量改动


1999-12-23: In struct v4l2_vbi_format the
`reserved1` field became `offset`. Previously drivers were required
to clear the `reserved1` field.

1999-12-23：在 struct v4l2_vbi_format 中，`reserved1` 字段变为
`offset`。此前驱动需要清`reserved1` 字段

2000-01-13: The `V4L2_FMT_FLAG_NOT_INTERLACED` flag was added.

2000-01-13：新增了 `V4L2_FMT_FLAG_NOT_INTERLACED` 标志

2000-07-31: The `linux/poll.h` header is now included by
`videodev.h` for compatibility with the original `videodev.h` file.

2000-07-31：为了与原始`videodev.h` 文件兼容，`videodev.h`
现在包含`linux/poll.h` 头文件

2000-11-20: `V4L2_TYPE_VBI_OUTPUT` and `V4L2_PIX_FMT_Y41P` were
added.

2000-11-20：新增了 `V4L2_TYPE_VBI_OUTPUT` `V4L2_PIX_FMT_Y41P`

2000-11-25: `V4L2_TYPE_VBI_INPUT` was added.

2000-11-25：新增了 `V4L2_TYPE_VBI_INPUT`

2000-12-04: A couple typos in symbol names were fixed.

2000-12-04：修正了符号名中的若干拼写错误

2001-01-18: To avoid namespace conflicts the `fourcc` macro defined in
the `videodev.h` header file was renamed to `v4l2_fourcc`.

2001-01-18：为避免命名空间冲突，`videodev.h` 头文件中定义
`fourcc` 宏被重命名为 `v4l2_fourcc`

2001-01-25: A possible driver-level compatibility problem between the
`videodev.h` file in Linux 2.4.0 and the `videodev.h` file included
in the `videodevX` patch was fixed. Users of an earlier version of
`videodevX` on Linux 2.4.0 should recompile their V4L and V4L2
drivers.

2001-01-25：修复了 Linux 2.4.0 中的 `videodev.h` 文件
`videodevX` 补丁中所包含`videodev.h` 文件之间可能存在
驱动级兼容性问题。在 Linux 2.4.0 上使用较早版
`videodevX` 的用户应重新编译V4L V4L2
驱动

2001-01-26: A possible kernel-level incompatibility between the
`videodev.h` file in the `videodevX` patch and the `videodev.h`
file in Linux 2.2.x with devfs patches applied was fixed.

2001-01-26：修复了 `videodevX` 补丁中的 `videodev.h` 文件
与打devfs 补丁Linux 2.2.x 中的 `videodev.h` 文件之间
可能存在的內核级不兼容问题

2001-03-02: Certain V4L ioctls which pass data in both direction
although they are defined with read-only parameter, did not work
correctly through the backward compatibility layer. [Solution?]

2001-03-02：某V4L ioctl 以只读参数定义，却会双向传递数据，
它们通过向后兼容层时无法正确工作。[解决方案？]

2001-04-13: Big endian 16-bit RGB formats were added.

2001-04-13：新增了大端（big endian6 RGB 格式

2001-09-17: New YUV formats and the
VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> and
VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctls were added.
(The old `VIDIOC_G_FREQ` and `VIDIOC_S_FREQ` ioctls did not take
multiple tuners into account.)

2001-09-17：新增了新的 YUV 格式以及
VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 鍜。
VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctls銆。
（旧`VIDIOC_G_FREQ` `VIDIOC_S_FREQ` ioctl 没有考虑
多个调谐器的情况。）

2000-09-18: `V4L2_BUF_TYPE_VBI` was added. This may *break
compatibility* as the VIDIOC_G_FMT <VIDIOC_G_FMT> and
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctls may fail now if the
struct `v4l2_fmt` `type` field does not contain
`V4L2_BUF_TYPE_VBI`. In the documentation of the struct v4l2_vbi_format`,
the `offset` field the ambiguous phrase "rising edge" was changed to
"leading edge".

2000-09-18：新增了 `V4L2_BUF_TYPE_VBI`。这可能破坏
兼容，因为如struct `v4l2_fmt` `type` 字段不包
`V4L2_BUF_TYPE_VBI`，VIDIOC_G_FMT <VIDIOC_G_FMT> 
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctls 现在可能会失败。在
struct v4l2_vbi_format 的文档中，`offset` 字段处含糊的
短语 "rising edge" 被改"leading edge"

## V4L2 Version 0.20 2000-11-23

## V4L2 0.20 鐗?2000-11-23


A number of changes were made to the raw VBI interface.

对原VBI 接口做了若干改动

1. Figures clarifying the line numbering scheme were added to the V4L2
   API specification. The `start`\ [^0^] and `start`\ [^1^] fields no
   longer count line numbers beginning at zero. Rationale: a) The
   previous definition was unclear. b) The `start`\ [] values are
   ordinal numbers. c) There is no point in inventing a new line
   numbering scheme. We now use line number as defined by ITU-R, period.
   Compatibility: Add one to the start values. Applications depending on
   the previous semantics may not function correctly.

1. V4L2 API 规范中新增了用于阐明行编号方案的图示。`start`\ [^0^]
   `start`\ [^1^] 字段不再从零开始计数行号。理由：a) 之前的定
   不清。b) `start`\ [] 的值是序数。c) 没有必要发明新的
   编号方案。现在我们采ITU-R 定义的行号，仅此而已
   兼容性：需start 值加一。依赖先前语义的应用程序可能无法
   正常工作

2. The restriction "count[^0^] > 0 and count[^1^] > 0" has been relaxed to
   "(count[^0^] + count[^1^]) > 0". Rationale: Drivers may allocate
   resources at scan line granularity and some data services are
   transmitted only on the first field. The comment that both `count`
   values will usually be equal is misleading and pointless and has been
   removed. This change **breaks compatibility** with earlier versions:
   Drivers may return `EINVAL`, applications may not function correctly.

2. 限制 "count[^0^] > 0 count[^1^] > 0" 已放宽至
   "(count[^0^] + count[^1^]) > 0"。理由：驱动可能以扫描行为粒
   分配资源，而某些数据服务仅在第一个场（field）上传输。关于两
   `count` 值通常相等的注释具有误导性且无意义，已被移除。这一改动
   **破坏了与早期版本的兼容*：驱动可能返`EINVAL`
   应用程序可能无法正常工作

3. Drivers are again permitted to return negative (unknown) start values
   as proposed earlier. Why this feature was dropped is unclear. This
   change may **break compatibility** with applications depending on the
   start values being positive. The use of `EBUSY` and `EINVAL`
   error codes with the VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl was
   clarified. The `EBUSY` error code was finally documented, and the
   `reserved2` field which was previously mentioned only in the
   `videodev.h` header file.

3. 驱动再次被允许返回负的（未知的）start 值，正如早先所建议的
   不清楚当初为何去掉了这一特性。这一改动可能**破坏与依
   正的 start 值的应用程序的兼容*。澄清了 `EBUSY` `EINVAL`
   错误码与 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 的使用方式。`EBUSY`
   错误码终于得到了文档说明，而此前仅`videodev.h` 头文件中
   提及`reserved2` 字段也得到了说明

4. New buffer types `V4L2_TYPE_VBI_INPUT` and `V4L2_TYPE_VBI_OUTPUT`
   were added. The former is an alias for the old `V4L2_TYPE_VBI`, the
   latter was missing in the `videodev.h` file.

4. 新增了新的缓冲区类型 `V4L2_TYPE_VBI_INPUT` 
   `V4L2_TYPE_VBI_OUTPUT`。前者是旧的 `V4L2_TYPE_VBI` 的别名，
   后者则`videodev.h` 文件中缺失的

## V4L2 Version 0.20 2002-07-25

## V4L2 0.20 鐗?2002-07-25


Added sliced VBI interface proposal.

新增了分片（sliced）VBI 接口提案

## V4L2 in Linux 2.5.46, 2002-10

## Linux 2.5.46 中的 V4L2002-10


Around October-November 2002, prior to an announced feature freeze of
Linux 2.5, the API was revised, drawing from experience with V4L2 0.20.
This unnamed version was finally merged into Linux 2.5.46.

2002 10 月至 11 月前后，Linux 2.5 宣布特性冻结之前，
API 借鉴 V4L2 0.20 的经验进行了修订。这个未命名的版本最
被合并进Linux 2.5.46

1. As specified in related, drivers must make related device
    functions available under all minor device numbers.

1. 如相关章节所规定，驱动必须在所有次设备号下提供相关的设
   功能

2. The `open()` function requires access mode
    `O_RDWR` regardless of the device type. All V4L2 drivers
    exchanging data with applications must support the `O_NONBLOCK`
    flag. The `O_NOIO` flag, a V4L2 symbol which aliased the
    meaningless `O_TRUNC` to indicate accesses without data exchange
    (panel applications) was dropped. Drivers must stay in "panel mode"
    until the application attempts to initiate a data exchange, see
    open.

2. 无论设备类型如何，`open()` 函数都要求使
    `O_RDWR` 访问模式。所有与应用程序交换数据V4L2 驱动必须
    支持 `O_NONBLOCK` 标志。曾经作V4L2 符号、将无意义的
    `O_TRUNC` 别名为表示无数据交换访问（面板应用程序）
    `O_NOIO` 标志被去掉了。驱动必须保持在"面板模式"
    直到应用程序尝试发起数据交换，详open

3. The struct v4l2_capability changed
    dramatically. Note that also the size of the structure changed,
    which is encoded in the ioctl request code, thus older V4L2 devices
    will respond with an `EINVAL` error code to the new
    VIDIOC_QUERYCAP ioctl.

3. struct v4l2_capability 发生
    巨大变化。注意该结构的大小也改变了，而这被编码进 ioctl 请求
    码中，因此较旧的 V4L2 设备会以 `EINVAL` 错误码来响应新的
    VIDIOC_QUERYCAP ioctl銆。

    There are new fields to identify the driver, a new RDS device
    function `V4L2_CAP_RDS_CAPTURE`, the `V4L2_CAP_AUDIO` flag
    indicates if the device has any audio connectors, another I/O
    capability V4L2_CAP_ASYNCIO can be flagged. In response to these
    changes the `type` field became a bit set and was merged into the
    `flags` field. `V4L2_FLAG_TUNER` was renamed to
    `V4L2_CAP_TUNER`, `V4L2_CAP_VIDEO_OVERLAY` replaced
    `V4L2_FLAG_PREVIEW` and `V4L2_CAP_VBI_CAPTURE` and
    `V4L2_CAP_VBI_OUTPUT` replaced `V4L2_FLAG_DATA_SERVICE`.
    `V4L2_FLAG_READ` and `V4L2_FLAG_WRITE` were merged into
    `V4L2_CAP_READWRITE`.

    新增了用于标识驱动程序的字段、新RDS 设备功能
    `V4L2_CAP_RDS_CAPTURE`；`V4L2_CAP_AUDIO` 标志
    指示设备是否具有音频连接器；另一I/O 能力 V4L2_CAP_ASYNCIO 
    可以被标记。作为对这些改动的响应，`type` 字段变成了一个位集合
    并被合并`flags` 字段。`V4L2_FLAG_TUNER` 被重命名
    `V4L2_CAP_TUNER`，`V4L2_CAP_VIDEO_OVERLAY` 取代
    `V4L2_FLAG_PREVIEW`，`V4L2_CAP_VBI_CAPTURE` 
    `V4L2_CAP_VBI_OUTPUT` 取代`V4L2_FLAG_DATA_SERVICE`
    `V4L2_FLAG_READ` `V4L2_FLAG_WRITE` 被合并为
    `V4L2_CAP_READWRITE`銆。

    The redundant fields `inputs`, `outputs` and `audios` were
    removed. These properties can be determined as described in
    video and audio.

    冗余`inputs`、`outputs` `audios` 字段
    移除。这些属性可video audio 章节所述的方式确定

    The somewhat volatile and therefore barely useful fields
    `maxwidth`, `maxheight`, `minwidth`, `minheight`,
    `maxframerate` were removed. This information is available as
    described in format and standard.

    那些不太稳定、因而几乎无用的字段
    `maxwidth`、`maxheight`、`minwidth`、`minheight`
    `maxframerate` 被移除。这些信息可format standard 章节
    所述的方式获取

    `V4L2_FLAG_SELECT` was removed. We believe the select() function
    is important enough to require support of it in all V4L2 drivers
    exchanging data with applications. The redundant
    `V4L2_FLAG_MONOCHROME` flag was removed, this information is
    available as described in format.

    `V4L2_FLAG_SELECT` 被移除。我们认select() 函数
    非常重要，要求所有与应用程序交换数据V4L2 驱动都支持它
    冗余`V4L2_FLAG_MONOCHROME` 标志被移除，该信息可
    format 章节所述的方式获取

4. In struct v4l2_input the `assoc_audio`
    field and the `capability` field and its only flag
    `V4L2_INPUT_CAP_AUDIO` was replaced by the new `audioset` field.
    Instead of linking one video input to one audio input this field
    reports all audio inputs this video input combines with.

4. struct v4l2_input 中，`assoc_audio`
    字段以及 `capability` 字段及其唯一的标
    `V4L2_INPUT_CAP_AUDIO` 被新`audioset` 字段取代
    该字段不再将一个视频输入关联到单个音频输入，而是报告
    该视频输入所组合的所有音频输入

    New fields are `tuner` (reversing the former link from tuners to
    video inputs), `std` and `status`.

    新增`tuner`（反转了原先从调谐器到视频输入的关联）
    `std` `status` 字段

    Accordingly struct v4l2_output lost its
    `capability` and `assoc_audio` fields. `audioset`,
    `modulator` and `std` where added instead.

    相应地，struct v4l2_output 失去了其
    `capability` `assoc_audio` 字段。取而代之新增了
    `audioset`、`modulator` `std`

5. The struct v4l2_audio field `audio` was
    renamed to `index`, for consistency with other structures. A new
    capability flag `V4L2_AUDCAP_STEREO` was added to indicated if the
    audio input in question supports stereo sound.
    `V4L2_AUDCAP_EFFECTS` and the corresponding `V4L2_AUDMODE` flags
    where removed. This can be easily implemented using controls.
    (However the same applies to AVL which is still there.)

5. 为与其他结构保持一致，struct v4l2_audio `audio` 字段
    重命名为 `index`。新增了一个能力标`V4L2_AUDCAP_STEREO`
    以指示相关音频输入是否支持立体声。`V4L2_AUDCAP_EFFECTS` 
    相应`V4L2_AUDMODE` 标志被移除。这可轻易地使用控制
    实现。（不过 AVL 的情况类似，但它仍然存在。）

    Again for consistency the struct v4l2_audioout field `audio` was renamed
    to `index`.

    同样为了保持一致，struct v4l2_audioout `audio` 字段被重命名
    `index`銆。

6. The struct v4l2_tuner `input` field was
    replaced by an `index` field, permitting devices with multiple
    tuners. The link between video inputs and tuners is now reversed,
    inputs point to their tuner. The `std` substructure became a
    simple set (more about this below) and moved into struct v4l2_input.
    A `type` field was added.

6. struct v4l2_tuner `input` 字段
    一`index` 字段取代，从而支持具有多个调谐器的设备。视频输
    与调谐器之间的关联现在被反转，输入指向其调谐器。`std` 子结
    变为一个简单的集合（详见下文）并移struct v4l2_input
    新增了一`type` 字段

    Accordingly in struct v4l2_modulator the
    `output` was replaced by an `index` field.

    相应地，struct v4l2_modulator 中，`output` 被一
    `index` 字段取代

    In struct v4l2_frequency the `port`
    field was replaced by a `tuner` field containing the respective
    tuner or modulator index number. A tuner `type` field was added
    and the `reserved` field became larger for future extensions
    (satellite tuners in particular).

    struct v4l2_frequency 中，`port`
    字段被一个包含相应调谐器或调制器索引号的 `tuner` 字段取代
    新增了调谐器 `type` 字段，并`reserved` 字段被扩大以
    便将来扩展（尤其是卫星调谐器）

7. The idea of completely transparent video standards was dropped.
    Experience showed that applications must be able to work with video
    standards beyond presenting the user a menu. Instead of enumerating
    supported standards with an ioctl applications can now refer to
    standards by v4l2_std_id <v4l2-std-id> and symbols
    defined in the `videodev2.h` header file. For details see
    standard. The VIDIOC_G_STD <VIDIOC_G_STD> and
    VIDIOC_S_STD <VIDIOC_G_STD> now take a pointer to this
    type as argument. VIDIOC_QUERYSTD was
    added to autodetect the received standard, if the hardware has this
    capability. In struct v4l2_standard an
    `index` field was added for
    VIDIOC_ENUMSTD. A
    v4l2_std_id <v4l2-std-id> field named `id` was added as
    machine readable identifier, also replacing the `transmission`
    field. The misleading `framerate` field was renamed to
    `frameperiod`. The now obsolete `colorstandard` information,
    originally needed to distguish between variations of standards, were
    removed.

7. 完全透明的视频标准这一设想被放弃了。经验表明，应用程序必须
    能够超越"向用户展示菜来与视频标准打交道。现在，应用程序不再
    ioctl 枚举所支持的标准，而是可以通过 `videodev2.h` 头文件中
    定义v4l2_std_id <v4l2-std-id> 和符号来引用标准。详
    standard。VIDIOC_G_STD <VIDIOC_G_STD> 
    VIDIOC_S_STD <VIDIOC_G_STD> 现在以指向该类型的指针作为参数
    新增VIDIOC_QUERYSTD，用于在硬件支持的情况下
    自动检测所接收的标准。在 struct v4l2_standard 中新增了
    `index` 字段用于
    VIDIOC_ENUMSTD。新增了一个名`id` 
    v4l2_std_id <v4l2-std-id> 字段作为机器可读标识符，同时
    取代`transmission` 字段。具有误导性的 `framerate` 字段
    被重命名`frameperiod`。现已过时的 `colorstandard` 信息
    （最初用于区分标准的不同变体）被移除

    Struct `v4l2_enumstd` ceased to be.
    VIDIOC_ENUMSTD now takes a pointer to a
    struct v4l2_standard directly. The
    information which standards are supported by a particular video
    input or output moved into struct v4l2_input
    and struct v4l2_output fields named `std`,
    respectively.

    struct `v4l2_enumstd` 不复存在。VIDIOC_ENUMSTD 现在直接
    以指struct v4l2_standard 的指针作为参数。某特定视频
    输入或输出所支持的标准这一信息分别移入了名`std` 
    struct v4l2_input struct v4l2_output 字段

8. The struct v4l2_queryctrl <v4l2-queryctrl> fields
    `category` and `group` did not catch on and/or were not
    implemented as expected and therefore removed.

8. struct v4l2_queryctrl <v4l2-queryctrl> 鐨。
    `category` `group` 字段未被广泛采用或未如预
    实现，因此被移除

9. The VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl was added to
    negotiate data formats as with
    VIDIOC_S_FMT <VIDIOC_G_FMT>, but without the overhead of
    programming the hardware and regardless of I/O in progress.

9. 新增VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl，用
    VIDIOC_S_FMT <VIDIOC_G_FMT> 那样协商数据格式，但
    产生编程硬件的开销，且不受进行中的 I/O 影响

    In struct v4l2_format the `fmt` union was
    extended to contain struct v4l2_window. All
    image format negotiations are now possible with `VIDIOC_G_FMT`,
    `VIDIOC_S_FMT` and `VIDIOC_TRY_FMT`; ioctl. The `VIDIOC_G_WIN`
    and `VIDIOC_S_WIN` ioctls to prepare for a video overlay were
    removed. The `type` field changed to type enum v4l2_buf_type and
    the buffer type names changed as follows.

    struct v4l2_format 中，`fmt` 联合
    扩展为包struct v4l2_window。现在所有图像格式协商都可以通过
    `VIDIOC_G_FMT`、`VIDIOC_S_FMT` `VIDIOC_TRY_FMT` ioctl
    完成。`VIDIOC_G_WIN` `VIDIOC_S_WIN` 这两个用于准备视
    叠加（overlay）的 ioctl 被移除。`type` 字段改为 enum v4l2_buf_type
    类型，缓冲区类型名称改动如下


```
	:header-rows:  1
	:stub-columns: 0

	* - Old defines
	  - enum v4l2_buf_type
	* - ``V4L2_BUF_TYPE_CAPTURE``
	  - ``V4L2_BUF_TYPE_VIDEO_CAPTURE``
	* - ``V4L2_BUF_TYPE_CODECIN``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_CODECOUT``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSIN``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSIN2``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_EFFECTSOUT``
	  - Omitted for now
	* - ``V4L2_BUF_TYPE_VIDEOOUT``
	  - ``V4L2_BUF_TYPE_VIDEO_OUTPUT``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VIDEO_OVERLAY``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VBI_CAPTURE``
	* - ``-``
	  - ``V4L2_BUF_TYPE_VBI_OUTPUT``
	* - ``-``
	  - ``V4L2_BUF_TYPE_SLICED_VBI_CAPTURE``
	* - ``-``
	  - ``V4L2_BUF_TYPE_SLICED_VBI_OUTPUT``
	* - ``V4L2_BUF_TYPE_PRIVATE_BASE``
	  - ``V4L2_BUF_TYPE_PRIVATE`` (but this is deprecated)

```

10. In struct v4l2_fmtdesc a enum v4l2_buf_type field named `type` was
    added as in struct v4l2_format. The `VIDIOC_ENUM_FBUFFMT` ioctl is no
    longer needed and was removed. These calls can be replaced by
    VIDIOC_ENUM_FMT with type `V4L2_BUF_TYPE_VIDEO_OVERLAY`.

10. struct v4l2_fmtdesc 中，新增了一个名`type` 
    enum v4l2_buf_type 字段，与 struct v4l2_format 中相同
    `VIDIOC_ENUM_FBUFFMT` ioctl 不再需要，已被移除。这些调用可
    使用 `V4L2_BUF_TYPE_VIDEO_OVERLAY` 类型VIDIOC_ENUM_FMT 取代

11. In struct v4l2_pix_format the `depth`
    field was removed, assuming applications which recognize the format
    by its four-character-code already know the color depth, and others
    do not care about it. The same rationale lead to the removal of the
    `V4L2_FMT_FLAG_COMPRESSED` flag. The
    `V4L2_FMT_FLAG_SWCONVECOMPRESSED` flag was removed because drivers
    are not supposed to convert images in kernel space. A user library
    of conversion functions should be provided instead. The
    `V4L2_FMT_FLAG_BYTESPERLINE` flag was redundant. Applications can
    set the `bytesperline` field to zero to get a reasonable default.
    Since the remaining flags were replaced as well, the `flags` field
    itself was removed.

11. struct v4l2_pix_format 中，`depth`
    字段被移除，因为假定那些通过四字符码（four-character-code）识
    格式的应用程序已经知道颜色深度，而其他应用程序并不关心它。同样的
    理由导致`V4L2_FMT_FLAG_COMPRESSED` 标志的移除
    `V4L2_FMT_FLAG_SWCONVECOMPRESSED` 标志被移除，因为驱动不应
    内核空间转换图像。应改为提供一个用户态的转换函数库
    `V4L2_FMT_FLAG_BYTESPERLINE` 标志是冗余的。应用程序可以将
    `bytesperline` 字段设为零以获得合理的默认值。由于其余标志也
    被替换了，因`flags` 字段本身也被移除

    The interlace flags were replaced by a enum v4l2_field value in a
    newly added `field` field.

    隔行（interlace）标志被新增加的 `field` 字段中的
    enum v4l2_field 值所取代


```
	:header-rows:  1
	:stub-columns: 0

	* - Old flag
	  - enum v4l2_field
	* - ``V4L2_FMT_FLAG_NOT_INTERLACED``
	  - ?
	* - ``V4L2_FMT_FLAG_INTERLACED`` = ``V4L2_FMT_FLAG_COMBINED``
	  - ``V4L2_FIELD_INTERLACED``
	* - ``V4L2_FMT_FLAG_TOPFIELD`` = ``V4L2_FMT_FLAG_ODDFIELD``
	  - ``V4L2_FIELD_TOP``
	* - ``V4L2_FMT_FLAG_BOTFIELD`` = ``V4L2_FMT_FLAG_EVENFIELD``
	  - ``V4L2_FIELD_BOTTOM``
	* - ``-``
	  - ``V4L2_FIELD_SEQ_TB``
	* - ``-``
	  - ``V4L2_FIELD_SEQ_BT``
	* - ``-``
	  - ``V4L2_FIELD_ALTERNATE``

    The color space flags were replaced by a enum v4l2_colorspace value in
    a newly added ``colorspace`` field, where one of
    ``V4L2_COLORSPACE_SMPTE170M``, ``V4L2_COLORSPACE_BT878``,
    ``V4L2_COLORSPACE_470_SYSTEM_M`` or
    ``V4L2_COLORSPACE_470_SYSTEM_BG`` replaces ``V4L2_FMT_CS_601YUV``.

    颜色空间（color space）标志被新增加的 ``colorspace`` 字段中的
    enum v4l2_colorspace 值所取代，其
    ``V4L2_COLORSPACE_SMPTE170M``、``V4L2_COLORSPACE_BT878``
    ``V4L2_COLORSPACE_470_SYSTEM_M`` 鎴?
    ``V4L2_COLORSPACE_470_SYSTEM_BG`` 涔嬩竴鍙栦唬浜?
    ``V4L2_FMT_CS_601YUV``銆?

```

12. In struct v4l2_requestbuffers the
    `type` field was properly defined as enum v4l2_buf_type. Buffer types
    changed as mentioned above. A new `memory` field of type
    enum v4l2_memory was added to distinguish between
    I/O methods using buffers allocated by the driver or the
    application. See io for details.

12. struct v4l2_requestbuffers 中，`type` 字段被正确定义为
    enum v4l2_buf_type。缓冲区类型如前所述发生了变化。新增了一
    enum v4l2_memory 类型`memory` 字段，用于区
    使用驱动分配还是应用程序分配的缓冲区I/O 方法。详io

13. In struct v4l2_buffer the `type` field was
    properly defined as enum v4l2_buf_type.
    Buffer types changed as mentioned above. A `field` field of type
    enum v4l2_field was added to indicate if a
    buffer contains a top or bottom field. The old field flags were
    removed. Since no unadjusted system time clock was added to the
    kernel as planned, the `timestamp` field changed back from type
    stamp_t, an unsigned 64 bit integer expressing the sample time in
    nanoseconds, to struct timeval. With the addition
    of a second memory mapping method the `offset` field moved into
    union `m`, and a new `memory` field of type enum v4l2_memory
    was added to distinguish between
    I/O methods. See io for details.

13. struct v4l2_buffer 中，`type` 字段被正确定义为
    enum v4l2_buf_type。缓冲区类型如前所述发生了变化。新增了一
    enum v4l2_field 类型`field` 字段，用于指示缓冲区包含的是
    顶场（top）还是底场（bottom）。旧的场标志被移除。由于内核中
    并未按原计划加入未校正系统时间时钟，`timestamp` 字段
    表示采样时间（纳秒）的无符号 64 位整stamp_t 类型改回
    struct timeval。随着第二种内存映射方法的加入，`offset`
    字段移入了联`m`，并新增了一enum v4l2_memory 类型
    `memory` 字段用于区分不同I/O 方法。详io

    The `V4L2_BUF_REQ_CONTIG` flag was used by the V4L compatibility
    layer, after changes to this code it was no longer needed. The
    `V4L2_BUF_ATTR_DEVICEMEM` flag would indicate if the buffer was
    indeed allocated in device memory rather than DMA-able system
    memory. It was barely useful and so was removed.

    `V4L2_BUF_REQ_CONTIG` 标志曾由 V4L 兼容层使用，在对该代
    进行改动后不再需要。`V4L2_BUF_ATTR_DEVICEMEM` 标志用于指示
    缓冲区是否确实分配在设备内存而非DMA 的系统内存中。它几乎
    没有用处，因此被移除

14. In struct v4l2_framebuffer the
    `base[^3^]` array anticipating double- and triple-buffering in
    off-screen video memory, however without defining a synchronization
    mechanism, was replaced by a single pointer. The
    `V4L2_FBUF_CAP_SCALEUP` and `V4L2_FBUF_CAP_SCALEDOWN` flags were
    removed. Applications can determine this capability more accurately
    using the new cropping and scaling interface. The
    `V4L2_FBUF_CAP_CLIPPING` flag was replaced by
    `V4L2_FBUF_CAP_LIST_CLIPPING` and
    `V4L2_FBUF_CAP_BITMAP_CLIPPING`.

14. struct v4l2_framebuffer 中，那个预期用于离屏视频内存
    双缓冲和三缓冲、却未定义同步机制的 `base[^3^]` 数组被单
    指针取代。`V4L2_FBUF_CAP_SCALEUP` `V4L2_FBUF_CAP_SCALEDOWN`
    标志被移除。应用程序可通过新的裁剪和缩放接口更准确地确定这一
    能力。`V4L2_FBUF_CAP_CLIPPING` 标志
    `V4L2_FBUF_CAP_LIST_CLIPPING` 鍜。
    `V4L2_FBUF_CAP_BITMAP_CLIPPING` 取代

15. In struct v4l2_clip the `x`, `y`,
    `width` and `height` field moved into a `c` substructure of
    type struct v4l2_rect. The `x` and `y`
    fields were renamed to `left` and `top`, i. e. offsets to a
    context dependent origin.

15. struct v4l2_clip 中，`x`、`y`
    `width` `height` 字段移入struct v4l2_rect 类型
    `c` 子结构。其`x` `y`
    字段被重命名`left` `top`，即相对
    上下文相关原点的偏移量

16. In struct v4l2_window the `x`, `y`,
    `width` and `height` field moved into a `w` substructure as
    above. A `field` field of type enum v4l2_field was added to
    distinguish between field and frame (interlaced) overlay.

16. struct v4l2_window 中，`x`、`y`
    `width` `height` 字段如上所述移入了 `w` 子结构。新增了
    一enum v4l2_field 类型`field` 字段，用于区
    场（field）和帧（frame，即隔行）叠加

17. The digital zoom interface, including struct `v4l2_zoomcap`,
    struct `v4l2_zoom`, `V4L2_ZOOM_NONCAP` and
    `V4L2_ZOOM_WHILESTREAMING` was replaced by a new cropping and
    scaling interface. The previously unused
    struct v4l2_cropcap and struct v4l2_crop
    where redefined for this purpose. See crop for details.

17. 数字缩放接口（包struct `v4l2_zoomcap`、struct `v4l2_zoom`
    `V4L2_ZOOM_NONCAP` `V4L2_ZOOM_WHILESTREAMING`）被新的
    裁剪和缩放接口取代。此前未使用struct v4l2_cropcap 
    struct v4l2_crop 为此被重新定义。详crop

18. In struct v4l2_vbi_format the
    `SAMPLE_FORMAT` field now contains a four-character-code as used
    to identify video image formats and `V4L2_PIX_FMT_GREY` replaces
    the `V4L2_VBI_SF_UBYTE` define. The `reserved` field was
    extended.

18. struct v4l2_vbi_format 中，`SAMPLE_FORMAT` 字段现在包含
    一个用于标识视频图像格式的四字符码，且 `V4L2_PIX_FMT_GREY`
    取代`V4L2_VBI_SF_UBYTE` 定义。`reserved` 字段被扩展

19. In struct v4l2_captureparm the type of
    the `timeperframe` field changed from unsigned long to
    struct v4l2_fract. This allows the accurate
    expression of multiples of the NTSC-M frame rate 30000 / 1001. A new
    field `readbuffers` was added to control the driver behaviour in
    read I/O mode.

19. struct v4l2_captureparm 中，`timeperframe` 字段的类型从
    unsigned long 改为 struct v4l2_fract。这样可以精确地表达
    NTSC-M 帧率 30000 / 1001 的倍数。新增了 `readbuffers` 字段
    用于控制驱动read I/O 模式下的行为

    Similar changes were made to struct v4l2_outputparm.

    struct v4l2_outputparm 也做了类似的改动

20. The struct `v4l2_performance` and
    `VIDIOC_G_PERF` ioctl were dropped. Except when using the
    read/write I/O method <rw>, which is limited anyway, this
    information is already available to applications.

20. struct `v4l2_performance` 鍜。
    `VIDIOC_G_PERF` ioctl 被废弃。除了受限的 read/write I/O 方法
    <rw> 外，这些信息应用程序已经可以获取

21. The example transformation from RGB to YCbCr color space in the old
    V4L2 documentation was inaccurate, this has been corrected in
    pixfmt.

21. 旧版 V4L2 文档中从 RGB YCbCr 颜色空间的示例变换是不准确的
    已在 pixfmt 中予以纠正

## V4L2 2003-06-19

## V4L2 2003-06-19


1. A new capability flag `V4L2_CAP_RADIO` was added for radio devices.
   Prior to this change radio devices would identify solely by having
   exactly one tuner whose type field reads `V4L2_TUNER_RADIO`.

1. 为无线电（radio）设备新增了能力标志 `V4L2_CAP_RADIO`。在此改
   之前，无线电设备仅凭其拥有恰好一type 字段`V4L2_TUNER_RADIO`
   的调谐器来标识

2. An optional driver access priority mechanism was added, see
   app-pri for details.

2. 新增了一个可选的驱动访问优先级机制，详见 app-pri

3. The audio input and output interface was found to be incomplete.

3. 人们发现音频输入和输出接口并不完整

   Previously the VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl would
   enumerate the available audio inputs. An ioctl to determine the
   current audio input, if more than one combines with the current video
   input, did not exist. So `VIDIOC_G_AUDIO` was renamed to
   `VIDIOC_G_AUDIO_OLD`, this ioctl was removed on Kernel 2.6.39. The
   VIDIOC_ENUMAUDIO ioctl was added to
   enumerate audio inputs, while
   VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> now reports the current
   audio input.

   此前 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl 会枚举可用的音频
   输入。而用于确定当前音频输入的 ioctl（当不止一个音频输入与当前
   视频输入组合时）并不存在。因`VIDIOC_G_AUDIO` 被重命名
   `VIDIOC_G_AUDIO_OLD`，该 ioctl 在内2.6.39 中被移除。新增了
   VIDIOC_ENUMAUDIO ioctl 用于枚举音频输入
   VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 现在报告当前
   音频输入

   The same changes were made to
   VIDIOC_G_AUDOUT <VIDIOC_G_AUDOUT> and
   VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT>.

   瀵?VIDIOC_G_AUDOUT <VIDIOC_G_AUDOUT> 鍜。
   VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT> 也做了同样的改动

   Until further the "videodev" module will automatically translate
   between the old and new ioctls, but drivers and applications must be
   updated to successfully compile again.

   在此过渡期间videodev" 模块会自动在ioctl 和新 ioctl 之间
   进行转换，但驱动和应用程序必须更新才能成功重新编译

4. The VIDIOC_OVERLAY ioctl was incorrectly
   defined with write-read parameter. It was changed to write-only,
   while the write-read version was renamed to `VIDIOC_OVERLAY_OLD`.
   The old ioctl was removed on Kernel 2.6.39. Until further the
   "videodev" kernel module will automatically translate to the new
   version, so drivers must be recompiled, but not applications.

4. VIDIOC_OVERLAY ioctl 曾被错误地定义为写参数。它被改
   只写（write-only），而读-写版本被重命名为 `VIDIOC_OVERLAY_OLD`
   旧的 ioctl 在内2.6.39 中被移除。在此过渡期间，"videodev"
   内核模块会自动转换为新版，因此驱动必须重新编译，但应用程
   无需重新编译

5. overlay incorrectly stated that clipping rectangles define
   regions where the video can be seen. Correct is that clipping
   rectangles define regions where **no** video shall be displayed and so
   the graphics surface can be seen.

5. overlay 文档错误地声称裁剪矩形定义了视频可见的区域。正确的说法是，
   裁剪矩形定义*不应**显示视频的区域，从而可以看到图形表面

6. The VIDIOC_S_PARM <VIDIOC_G_PARM> and
   VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls were defined with
   write-only parameter, inconsistent with other ioctls modifying their
   argument. They were changed to write-read, while a `_OLD` suffix
   was added to the write-only versions. The old ioctls were removed on
   Kernel 2.6.39. Drivers and applications assuming a constant parameter
   need an update.

6. VIDIOC_S_PARM <VIDIOC_G_PARM> 鍜。
   VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctls 被定义为只写参数，这与其
   修改其参数的 ioctl 不一致。它们被改为写，而只写版本加上了
   `_OLD` 后缀。旧 ioctl 在内2.6.39 中被移除。假定参数为
   常量（constant）的驱动和应用程序需要更新

## V4L2 2003-11-05

## V4L2 2003-11-05


1. In pixfmt-rgb the following pixel formats were incorrectly
   transferred from Bill Dirks' V4L2 specification. Descriptions below
   refer to bytes in memory, in ascending address order.


```
       :header-rows:  1
       :stub-columns: 0

       * - Symbol
	 - In this document prior to revision 0.5
	 - Corrected
       * - ``V4L2_PIX_FMT_RGB24``
	 - B, G, R
	 - R, G, B
       * - ``V4L2_PIX_FMT_BGR24``
	 - R, G, B
	 - B, G, R
       * - ``V4L2_PIX_FMT_RGB32``
	 - B, G, R, X
	 - R, G, B, X
       * - ``V4L2_PIX_FMT_BGR32``
	 - R, G, B, X
	 - B, G, R, X

   The ``V4L2_PIX_FMT_BGR24`` example was always correct.

   In :ref:`v4l-image-properties` the mapping of the V4L
   ``VIDEO_PALETTE_RGB24`` and ``VIDEO_PALETTE_RGB32`` formats to V4L2
   pixel formats was accordingly corrected.

```

1. pixfmt-rgb 中，以下像素格式错误地照搬自 Bill Dirks V4L2 规范
   下面的描述指的是内存中的字节，按地址升序排列


   ``V4L2_PIX_FMT_BGR24`` 的示例始终是正确的

   :ref:`v4l-image-properties` 中，V4L 
   ``VIDEO_PALETTE_RGB24`` ``VIDEO_PALETTE_RGB32`` 格式V4L2
   像素格式的映射也做了相应的纠正

2. Unrelated to the fixes above, drivers may still interpret some V4L2
   RGB pixel formats differently. These issues have yet to be addressed,
   for details see pixfmt-rgb.

2. 与上述修正无关，驱动可能仍然以不同方式解释某V4L2 RGB 像素格式
   这些问题尚待解决，详pixfmt-rgb

## V4L2 in Linux 2.6.6, 2004-05-09

## Linux 2.6.6 中的 V4L2004-05-09


1. The VIDIOC_CROPCAP ioctl was incorrectly
   defined with read-only parameter. It is now defined as write-read
   ioctl, while the read-only version was renamed to
   `VIDIOC_CROPCAP_OLD`. The old ioctl was removed on Kernel 2.6.39.

1. VIDIOC_CROPCAP ioctl 曾被错误地定义为只读参数。现在它被定义为
   ioctl，而只读版本被重命名为 `VIDIOC_CROPCAP_OLD`。旧
   ioctl 在内2.6.39 中被移除

## V4L2 in Linux 2.6.8

## Linux 2.6.8 中的 V4L2


1. A new field `input` (former `reserved[^0^]`) was added to the
   struct v4l2_buffer. Purpose of this
   field is to alternate between video inputs (e. g. cameras) in step
   with the video capturing process. This function must be enabled with
   the new `V4L2_BUF_FLAG_INPUT` flag. The `flags` field is no
   longer read-only.

1. struct v4l2_buffer 中新增了一个字`input`（原
   `reserved[^0^]`）。该字段的目的是在视频采集过程中
   与采集流程同步地切换不同的视频输入（例如摄像头）。这一功能必须
   通过新的 `V4L2_BUF_FLAG_INPUT` 标志启用。`flags` 字段不再
   只读的

## V4L2 spec erratum 2004-08-01

## V4L2 规范勘误 2004-08-01


1. The return value of the func-open function was incorrectly
   documented.

1. func-open 函数的返回值文档有误

2. Audio output ioctls end in -AUDOUT, not -AUDIOOUT.

2. 音频输出 ioctl -AUDOUT 结尾，而非 -AUDIOOUT

3. In the Current Audio Input example the `VIDIOC_G_AUDIO` ioctl took
   the wrong argument.

3. 当前音频输入"示例中，`VIDIOC_G_AUDIO` ioctl 使用了错误的
   参数

4. The documentation of the VIDIOC_QBUF and
   VIDIOC_DQBUF <VIDIOC_QBUF> ioctls did not mention the
   struct v4l2_buffer `memory` field. It was
   also missing from examples. Also on the `VIDIOC_DQBUF` page the `EIO`
   error code was not documented.

4. VIDIOC_QBUF VIDIOC_DQBUF <VIDIOC_QBUF> ioctls 的文
   没有提及 struct v4l2_buffer `memory` 字段。示例中
   也缺失了它。此外，`VIDIOC_DQBUF` 页面`EIO` 错误码也
   未被文档说明

## V4L2 in Linux 2.6.14

## Linux 2.6.14 中的 V4L2


1. A new sliced VBI interface was added. It is documented in
   sliced and replaces the interface first proposed in V4L2
   specification 0.8.

1. 新增了一个新的分片（sliced）VBI 接口。它sliced 中有文档
   取代V4L2 规范 0.8 中最初提出的接口

## V4L2 in Linux 2.6.15

## Linux 2.6.15 中的 V4L2


1. The VIDIOC_LOG_STATUS ioctl was added.

1. 新增VIDIOC_LOG_STATUS ioctl

2. New video standards `V4L2_STD_NTSC_443`, `V4L2_STD_SECAM_LC`,
   `V4L2_STD_SECAM_DK` (a set of SECAM D, K and K1), and
   `V4L2_STD_ATSC` (a set of `V4L2_STD_ATSC_8_VSB` and
   `V4L2_STD_ATSC_16_VSB`) were defined. Note the `V4L2_STD_525_60`
   set now includes `V4L2_STD_NTSC_443`. See also
   v4l2-std-id.

2. 定义了新的视频标`V4L2_STD_NTSC_443`、`V4L2_STD_SECAM_LC`
   `V4L2_STD_SECAM_DK`（一SECAM D、K K1）以
   `V4L2_STD_ATSC`（一`V4L2_STD_ATSC_8_VSB` 
   `V4L2_STD_ATSC_16_VSB`）。注`V4L2_STD_525_60` 集合现在
   包含`V4L2_STD_NTSC_443`。另v4l2-std-id

3. The `VIDIOC_G_COMP` and `VIDIOC_S_COMP` ioctl were renamed to
   `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` respectively. Their
   argument was replaced by a struct
   `v4l2_mpeg_compression` pointer. (The
   `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls where removed
   in Linux 2.6.25.)

3. `VIDIOC_G_COMP` `VIDIOC_S_COMP` ioctl 分别被重命名
   `VIDIOC_G_MPEGCOMP` `VIDIOC_S_MPEGCOMP`。它们的参数被替换为
   struct `v4l2_mpeg_compression` 指针。（`VIDIOC_G_MPEGCOMP` 
   `VIDIOC_S_MPEGCOMP` ioctls Linux 2.6.25 中被移除。）

## V4L2 spec erratum 2005-11-27

## V4L2 规范勘误 2005-11-27


The capture example in capture-example called the
VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl without checking if
cropping is supported. In the video standard selection example in
standard the VIDIOC_S_STD <VIDIOC_G_STD> call used
the wrong argument type.

capture-example 中的采集示例调用VIDIOC_S_CROP <VIDIOC_G_CROP>
ioctl，却没有检查是否支持裁剪。而在 standard 中的视频标准选择
示例里，VIDIOC_S_STD <VIDIOC_G_STD> 调用使用了错误的参数类型

## V4L2 spec erratum 2006-01-10

## V4L2 规范勘误 2006-01-10


1. The `V4L2_IN_ST_COLOR_KILL` flag in struct v4l2_input not only
   indicates if the color killer is enabled, but also if it is active.
   (The color killer disables color decoding when it detects no color
   in the video signal to improve the image quality.)

1. struct v4l2_input 中的 `V4L2_IN_ST_COLOR_KILL` 标志不仅指示
   消色器（color killer）是否启用，还指示其是否处于活动状态
   （当消色器检测到视频信号中没有颜色时，它会禁用颜色解码以改善
   图像质量。）

2. VIDIOC_S_PARM <VIDIOC_G_PARM> is a write-read ioctl, not
   write-only as stated on its reference page. The ioctl changed in 2003
   as noted above.

2. VIDIOC_S_PARM <VIDIOC_G_PARM> 是一个读-ioctl，而不是其
   参考页上所说的只写。该 ioctl 2003 年已如前所述发生了改变

## V4L2 spec erratum 2006-02-03

## V4L2 规范勘误 2006-02-03


1. In struct v4l2_captureparm and struct v4l2_outputparm the `timeperframe`
   field gives the time in seconds, not microseconds.

1. struct v4l2_captureparm struct v4l2_outputparm 中，
   `timeperframe` 字段给出的时间单位是秒，而非微秒

## V4L2 spec erratum 2006-02-04

## V4L2 规范勘误 2006-02-04


1. The `clips` field in struct v4l2_window
   must point to an array of struct v4l2_clip, not
   a linked list, because drivers ignore the
   struct v4l2_clip. `next` pointer.

1. struct v4l2_window 中的 `clips` 字段必须指向
   struct v4l2_clip 数组，而不是链表，因为驱动会忽
   struct v4l2_clip `next` 指针

## V4L2 in Linux 2.6.17

## Linux 2.6.17 中的 V4L2


1. New video standard macros were added: `V4L2_STD_NTSC_M_KR` (NTSC M
   South Korea), and the sets `V4L2_STD_MN`, `V4L2_STD_B`,
   `V4L2_STD_GH` and `V4L2_STD_DK`. The `V4L2_STD_NTSC` and
   `V4L2_STD_SECAM` sets now include `V4L2_STD_NTSC_M_KR` and
   `V4L2_STD_SECAM_LC` respectively.

1. 新增了视频标准宏：`V4L2_STD_NTSC_M_KR`（NTSC M 韩国版）
   以及集合 `V4L2_STD_MN`、`V4L2_STD_B`、`V4L2_STD_GH` 
   `V4L2_STD_DK`。`V4L2_STD_NTSC` `V4L2_STD_SECAM` 集合现在
   分别包含`V4L2_STD_NTSC_M_KR` `V4L2_STD_SECAM_LC`

2. A new `V4L2_TUNER_MODE_LANG1_LANG2` was defined to record both
   languages of a bilingual program. The use of
   `V4L2_TUNER_MODE_STEREO` for this purpose is deprecated now. See
   the VIDIOC_G_TUNER <VIDIOC_G_TUNER> section for details.

2. 定义了一个新`V4L2_TUNER_MODE_LANG1_LANG2`，用于记录双语节
   的两种语言。现在不推荐为此目的使用 `V4L2_TUNER_MODE_STEREO`
   详见 VIDIOC_G_TUNER <VIDIOC_G_TUNER> 章节

## V4L2 spec erratum 2006-09-23 (Draft 0.15)

## V4L2 规范勘误 2006-09-23（草0.15


1. In various places `V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` and
   `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT` of the sliced VBI interface were
   not mentioned along with other buffer types.

1. 在许多地方，分片 VBI 接口`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` 
   `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT` 没有和其他缓冲区类型一起被
   提及

2. In VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> it was clarified that the
   struct v4l2_audio `mode` field is a flags field.

2. VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 中澄清了 struct v4l2_audio 
   `mode` 字段是一个标志位字段

3. VIDIOC_QUERYCAP did not mention the sliced VBI and radio
   capability flags.

3. VIDIOC_QUERYCAP 没有提及分片 VBI 和无线电的能力标志

4. In VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> it was clarified that
   applications must initialize the tuner `type` field of
   struct v4l2_frequency before calling
   VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>.

4. VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 中澄清了应用程序
   必须在调VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> 之前初始
   struct v4l2_frequency 的调谐器 `type` 字段

5. The `reserved` array in struct v4l2_requestbuffers has 2 elements,
   not 32.

5. struct v4l2_requestbuffers 中的 `reserved` 数组2 个元素，
   而非 32 个

6. In output and raw-vbi the device file names
   `/dev/vout` which never caught on were replaced by `/dev/video`.

6. output raw-vbi 中，从未流行的设备文件名
   `/dev/vout` `/dev/video` 取代

7. With Linux 2.6.15 the possible range for VBI device minor numbers was
   extended from 224-239 to 224-255. Accordingly device file names
   `/dev/vbi0` to `/dev/vbi31` are possible now.

7. Linux 2.6.15 起，VBI 设备次设备号的可能范围从 224-239 扩展
   224-255。相应地，现在可以使`/dev/vbi0` `/dev/vbi31` 这样
   设备文件名

## V4L2 in Linux 2.6.18

## Linux 2.6.18 中的 V4L2


1. New ioctls VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>,
   VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> and
   VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> were added, a
   flag to skip unsupported controls with
   VIDIOC_QUERYCTRL, new control types
   `V4L2_CTRL_TYPE_INTEGER64` and `V4L2_CTRL_TYPE_CTRL_CLASS`
   (enum v4l2_ctrl_type), and new control flags
   `V4L2_CTRL_FLAG_READ_ONLY`, `V4L2_CTRL_FLAG_UPDATE`,
   `V4L2_CTRL_FLAG_INACTIVE` and `V4L2_CTRL_FLAG_SLIDER`
   (control-flags). See extended-controls for details.

1. 新增VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>
   VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鍜。
   VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctls，一个配
   VIDIOC_QUERYCTRL 跳过不受支持控制项的标志，新的控制类
   `V4L2_CTRL_TYPE_INTEGER64` 鍜?`V4L2_CTRL_TYPE_CTRL_CLASS`
   （enum v4l2_ctrl_type），以及新的控制标志
   `V4L2_CTRL_FLAG_READ_ONLY`、`V4L2_CTRL_FLAG_UPDATE`
   `V4L2_CTRL_FLAG_INACTIVE` 鍜?`V4L2_CTRL_FLAG_SLIDER`
   （control-flags）。详extended-controls

## V4L2 in Linux 2.6.19

## Linux 2.6.19 中的 V4L2


1. In struct v4l2_sliced_vbi_cap a
   buffer type field was added replacing a reserved field. Note on
   architectures where the size of enum types differs from int types the
   size of the structure changed. The
   VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl
   was redefined from being read-only to write-read. Applications must
   initialize the type field and clear the reserved fields now. These
   changes may **break the compatibility** with older drivers and
   applications.

1. struct v4l2_sliced_vbi_cap 中，新增了一个缓冲区类型字段
   取代一个保留字段。注意在枚举类型大小int 类型不同的架构上
   该结构的大小会发生改变。VIDIOC_G_SLICED_VBI_CAP
   <VIDIOC_G_SLICED_VBI_CAP> ioctl 从只读被重新定义为读-写
   应用程序现在必须初始type 字段并清除保留字段。这些变化可
   **破坏与较旧驱动和应用程序的兼容*

2. The ioctls VIDIOC_ENUM_FRAMESIZES
   and
   VIDIOC_ENUM_FRAMEINTERVALS
   were added.

2. 新增VIDIOC_ENUM_FRAMESIZES 
   VIDIOC_ENUM_FRAMEINTERVALS ioctls銆。

3. A new pixel format `V4L2_PIX_FMT_RGB444` (pixfmt-rgb) was
   added.

3. 新增了新的像素格`V4L2_PIX_FMT_RGB444`（pixfmt-rgb）

## V4L2 spec erratum 2006-10-12 (Draft 0.17)

## V4L2 规范勘误 2006-10-12（草0.17


1. `V4L2_PIX_FMT_HM12` (reserved-formats) is a YUV 4:2:0, not
   4:2:2 format.

1. `V4L2_PIX_FMT_HM12`（reserved-formats）是 YUV 4:2:0，而非
   4:2:2 格式

## V4L2 in Linux 2.6.21

## Linux 2.6.21 中的 V4L2


1. The `videodev2.h` header file is now dual licensed under GNU
   General Public License version two or later, and under a 3-clause
   BSD-style license.

1. `videodev2.h` 头文件现在采用双重许可：GNU 通用公共许可证（版本
   或更高）以及 3 条款BSD 风格许可证

## V4L2 in Linux 2.6.22

## Linux 2.6.22 中的 V4L2


1. Two new field orders `V4L2_FIELD_INTERLACED_TB` and
   `V4L2_FIELD_INTERLACED_BT` were added. See enum v4l2_field for
   details.

1. 新增了两个新的场`V4L2_FIELD_INTERLACED_TB` 
   `V4L2_FIELD_INTERLACED_BT`。详enum v4l2_field

2. Three new clipping/blending methods with a global or straight or
   inverted local alpha value were added to the video overlay interface.
   See the description of the VIDIOC_G_FBUF <VIDIOC_G_FBUF>
   and VIDIOC_S_FBUF <VIDIOC_G_FBUF> ioctls for details.

2. 视频叠加（overlay）接口新增了三种裁剪/混合（clipping/blending
   方法，分别带有全局、或正向、或取反的局alpha 值。详
   VIDIOC_G_FBUF <VIDIOC_G_FBUF> 鍜?VIDIOC_S_FBUF
   <VIDIOC_G_FBUF> ioctls 的描述

   A new `global_alpha` field was added to struct v4l2_window,
   extending the structure. This may **break compatibility** with
   applications using a struct v4l2_window directly. However the
   VIDIOC_G/S/TRY_FMT <VIDIOC_G_FMT> ioctls, which take a
   pointer to a struct v4l2_format parent structure
   with padding bytes at the end, are not affected.

   struct v4l2_window 中新增了 `global_alpha` 字段，扩展了
   结构。这可能**破坏与直接使struct v4l2_window 的应用程序的
   兼容*。不过，VIDIOC_G/S/TRY_FMT <VIDIOC_G_FMT> ioctls 接受
   一个指向末尾带填充字节struct v4l2_format 父结构的指针
   不会受到影响

3. The format of the `chromakey` field in struct v4l2_window changed from
   "host order RGB32" to a pixel value in the same format as the framebuffer.
   This may **break compatibility** with existing applications. Drivers
   supporting the "host order RGB32" format are not known.

3. struct v4l2_window `chromakey` 字段的格式从
   "host order RGB32" 改为与帧缓冲（framebuffer）相同的像素值格式
   这可*破坏与现有应用程序的兼容*。目前不知道有驱动支
   "host order RGB32" 格式

## V4L2 in Linux 2.6.24

## Linux 2.6.24 中的 V4L2


1. The pixel formats `V4L2_PIX_FMT_PAL8`, `V4L2_PIX_FMT_YUV444`,
   `V4L2_PIX_FMT_YUV555`, `V4L2_PIX_FMT_YUV565` and
   `V4L2_PIX_FMT_YUV32` were added.

1. 新增了像素格`V4L2_PIX_FMT_PAL8`、`V4L2_PIX_FMT_YUV444`
   `V4L2_PIX_FMT_YUV555`、`V4L2_PIX_FMT_YUV565` 
   `V4L2_PIX_FMT_YUV32`銆。

## V4L2 in Linux 2.6.25

## Linux 2.6.25 中的 V4L2


1. The pixel formats V4L2_PIX_FMT_Y16 <V4L2-PIX-FMT-Y16> and
   V4L2_PIX_FMT_SBGGR16 <V4L2-PIX-FMT-SBGGR16> were added.

1. 新增了像素格V4L2_PIX_FMT_Y16 <V4L2-PIX-FMT-Y16> 
   V4L2_PIX_FMT_SBGGR16 <V4L2-PIX-FMT-SBGGR16>銆。

2. New controls <control> `V4L2_CID_POWER_LINE_FREQUENCY`,
   `V4L2_CID_HUE_AUTO`, `V4L2_CID_WHITE_BALANCE_TEMPERATURE`,
   `V4L2_CID_SHARPNESS` and `V4L2_CID_BACKLIGHT_COMPENSATION` were
   added. The controls `V4L2_CID_BLACK_LEVEL`, `V4L2_CID_WHITENESS`,
   `V4L2_CID_HCENTER` and `V4L2_CID_VCENTER` were deprecated.

2. 新增了控制项 <control> `V4L2_CID_POWER_LINE_FREQUENCY`
   `V4L2_CID_HUE_AUTO`、`V4L2_CID_WHITE_BALANCE_TEMPERATURE`
   `V4L2_CID_SHARPNESS` 鍜?`V4L2_CID_BACKLIGHT_COMPENSATION`銆。
   控制`V4L2_CID_BLACK_LEVEL`、`V4L2_CID_WHITENESS`
   `V4L2_CID_HCENTER` `V4L2_CID_VCENTER` 被弃用

3. A Camera controls class <camera-controls> was added, with
   the new controls `V4L2_CID_EXPOSURE_AUTO`,
   `V4L2_CID_EXPOSURE_ABSOLUTE`, `V4L2_CID_EXPOSURE_AUTO_PRIORITY`,
   `V4L2_CID_PAN_RELATIVE`, `V4L2_CID_TILT_RELATIVE`,
   `V4L2_CID_PAN_RESET`, `V4L2_CID_TILT_RESET`,
   `V4L2_CID_PAN_ABSOLUTE`, `V4L2_CID_TILT_ABSOLUTE`,
   `V4L2_CID_FOCUS_ABSOLUTE`, `V4L2_CID_FOCUS_RELATIVE` and
   `V4L2_CID_FOCUS_AUTO`.

3. 新增了相机（Camera）控制类 <camera-controls>，包含新的控制项
   `V4L2_CID_EXPOSURE_AUTO`、`V4L2_CID_EXPOSURE_ABSOLUTE`
   `V4L2_CID_EXPOSURE_AUTO_PRIORITY`、`V4L2_CID_PAN_RELATIVE`
   `V4L2_CID_TILT_RELATIVE`、`V4L2_CID_PAN_RESET`
   `V4L2_CID_TILT_RESET`、`V4L2_CID_PAN_ABSOLUTE`
   `V4L2_CID_TILT_ABSOLUTE`、`V4L2_CID_FOCUS_ABSOLUTE`
   `V4L2_CID_FOCUS_RELATIVE` 鍜?`V4L2_CID_FOCUS_AUTO`銆。

4. The `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls, which
   were superseded by the extended controls <extended-controls>
   interface in Linux 2.6.18, where finally removed from the
   `videodev2.h` header file.

4. Linux 2.6.18 中已被扩展控<extended-controls> 接口取代
   `VIDIOC_G_MPEGCOMP` `VIDIOC_S_MPEGCOMP` ioctls，最终从
   `videodev2.h` 头文件中移除

## V4L2 in Linux 2.6.26

## Linux 2.6.26 中的 V4L2


1. The pixel formats `V4L2_PIX_FMT_Y16` and `V4L2_PIX_FMT_SBGGR16`
   were added.

1. 新增了像素格`V4L2_PIX_FMT_Y16` `V4L2_PIX_FMT_SBGGR16`

2. Added user controls `V4L2_CID_CHROMA_AGC` and
   `V4L2_CID_COLOR_KILLER`.

2. 新增了用户控制项 `V4L2_CID_CHROMA_AGC` `V4L2_CID_COLOR_KILLER`

## V4L2 in Linux 2.6.27

## Linux 2.6.27 中的 V4L2


1. The VIDIOC_S_HW_FREQ_SEEK ioctl
   and the `V4L2_CAP_HW_FREQ_SEEK` capability were added.

1. 新增VIDIOC_S_HW_FREQ_SEEK ioctl 
   `V4L2_CAP_HW_FREQ_SEEK` 能力

2. The pixel formats `V4L2_PIX_FMT_YVYU`, `V4L2_PIX_FMT_PCA501`,
   `V4L2_PIX_FMT_PCA505`, `V4L2_PIX_FMT_PCA508`,
   `V4L2_PIX_FMT_PCA561`, `V4L2_PIX_FMT_SGBRG8`,
   `V4L2_PIX_FMT_PAC207` and `V4L2_PIX_FMT_PJPG` were added.

2. 新增了像素格`V4L2_PIX_FMT_YVYU`、`V4L2_PIX_FMT_PCA501`
   `V4L2_PIX_FMT_PCA505`、`V4L2_PIX_FMT_PCA508`
   `V4L2_PIX_FMT_PCA561`、`V4L2_PIX_FMT_SGBRG8`
   `V4L2_PIX_FMT_PAC207` 鍜?`V4L2_PIX_FMT_PJPG`銆。

## V4L2 in Linux 2.6.28

## Linux 2.6.28 中的 V4L2


1. Added `V4L2_MPEG_AUDIO_ENCODING_AAC` and
   `V4L2_MPEG_AUDIO_ENCODING_AC3` MPEG audio encodings.

1. 新增`V4L2_MPEG_AUDIO_ENCODING_AAC` 
   `V4L2_MPEG_AUDIO_ENCODING_AC3` MPEG 音频编码

2. Added `V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC` MPEG video encoding.

2. 新增`V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC` MPEG 视频编码

3. The pixel formats `V4L2_PIX_FMT_SGRBG10` and
   `V4L2_PIX_FMT_SGRBG10DPCM8` were added.

3. 新增了像素格`V4L2_PIX_FMT_SGRBG10` `V4L2_PIX_FMT_SGRBG10DPCM8`

## V4L2 in Linux 2.6.29

## Linux 2.6.29 中的 V4L2


1. The `VIDIOC_G_CHIP_IDENT` ioctl was renamed to
   `VIDIOC_G_CHIP_IDENT_OLD` and `VIDIOC_DBG_G_CHIP_IDENT` was
   introduced in its place. The old struct `v4l2_chip_ident` was renamed to
   struct `v4l2_chip_ident_old`.

1. `VIDIOC_G_CHIP_IDENT` ioctl 被重命名
   `VIDIOC_G_CHIP_IDENT_OLD`，并引入 `VIDIOC_DBG_G_CHIP_IDENT`
   取而代之。旧struct `v4l2_chip_ident` 被重命名
   struct `v4l2_chip_ident_old`銆。

2. The pixel formats `V4L2_PIX_FMT_VYUY`, `V4L2_PIX_FMT_NV16` and
   `V4L2_PIX_FMT_NV61` were added.

2. 新增了像素格`V4L2_PIX_FMT_VYUY`、`V4L2_PIX_FMT_NV16` 
   `V4L2_PIX_FMT_NV61`銆。

3. Added camera controls `V4L2_CID_ZOOM_ABSOLUTE`,
   `V4L2_CID_ZOOM_RELATIVE`, `V4L2_CID_ZOOM_CONTINUOUS` and
   `V4L2_CID_PRIVACY`.

3. 新增了相机控制项 `V4L2_CID_ZOOM_ABSOLUTE`
   `V4L2_CID_ZOOM_RELATIVE`、`V4L2_CID_ZOOM_CONTINUOUS` 
   `V4L2_CID_PRIVACY`銆。

## V4L2 in Linux 2.6.30

## Linux 2.6.30 中的 V4L2


1. New control flag `V4L2_CTRL_FLAG_WRITE_ONLY` was added.

1. 新增了控制标`V4L2_CTRL_FLAG_WRITE_ONLY`

2. New control `V4L2_CID_COLORFX` was added.

2. 新增了控制项 `V4L2_CID_COLORFX`

## V4L2 in Linux 2.6.32

## Linux 2.6.32 中的 V4L2


1. In order to be easier to compare a V4L2 API and a kernel version, now
   V4L2 API is numbered using the Linux Kernel version numeration.

1. 为了便于V4L2 API 与内核版本进行比较，现在 V4L2 API 采用
   Linux 内核版本号进行编号

2. Finalized the RDS capture API. See rds for more information.

2. 完善RDS 采集 API。更多信息见 rds

3. Added new capabilities for modulators and RDS encoders.

3. 为调制器（modulator）和 RDS 编码器新增了能力

4. Add description for libv4l API.

4. 增加libv4l API 的说明

5. Added support for string controls via new type
   `V4L2_CTRL_TYPE_STRING`.

5. 通过新类`V4L2_CTRL_TYPE_STRING` 增加了对字符串控制项的支持

6. Added `V4L2_CID_BAND_STOP_FILTER` documentation.

6. 增加`V4L2_CID_BAND_STOP_FILTER` 文档

7. Added FM Modulator (FM TX) Extended Control Class:
   `V4L2_CTRL_CLASS_FM_TX` and their Control IDs.

7. 新增FM 调制器（FM TX）扩展控制类：`V4L2_CTRL_CLASS_FM_TX` 
   其控ID

8. Added FM Receiver (FM RX) Extended Control Class:
   `V4L2_CTRL_CLASS_FM_RX` and their Control IDs.

8. 新增FM 接收器（FM RX）扩展控制类：`V4L2_CTRL_CLASS_FM_RX` 
   其控ID

9. Added Remote Controller chapter, describing the default Remote
   Controller mapping for media devices.

9. 新增遥控（Remote Controller）章节，描述媒体设备的默
   遥控器映射

## V4L2 in Linux 2.6.33

## Linux 2.6.33 中的 V4L2


1. Added support for Digital Video timings in order to support HDTV
   receivers and transmitters.

1. 新增了对数字视频（Digital Video）时序的支持，以便支HDTV
   接收器和发送器

## V4L2 in Linux 2.6.34

## Linux 2.6.34 中的 V4L2


1. Added `V4L2_CID_IRIS_ABSOLUTE` and `V4L2_CID_IRIS_RELATIVE`
   controls to the Camera controls class <camera-controls>.

1. 向相机控制类 <camera-controls> 中新增了
   `V4L2_CID_IRIS_ABSOLUTE` `V4L2_CID_IRIS_RELATIVE` 控制项

## V4L2 in Linux 2.6.37

## Linux 2.6.37 中的 V4L2


1. Remove the vtx (videotext/teletext) API. This API was no longer used
   and no hardware exists to verify the API. Nor were any userspace
   applications found that used it. It was originally scheduled for
   removal in 2.6.35.

1. 移除vtx（videotext/teletext）API。该 API 已不再被使用，也没有
   可用于验证该 API 的硬件。也未发现任何使用它的用户态应用程序
   它原本计划在 2.6.35 中移除

## V4L2 in Linux 2.6.39

## Linux 2.6.39 中的 V4L2


1. The old VIDIOC_*_OLD symbols and V4L1 support were removed.

1. 移除了旧VIDIOC_*_OLD 符号以及 V4L1 支持

2. Multi-planar API added. Does not affect the compatibility of current
   drivers and applications. See multi-planar API <planar-apis>
   for details.

2. 新增了多平面（multi-planar）API。不影响当前驱动和应用程序的
   兼容性。详见多平面 API <planar-apis>

## V4L2 in Linux 3.1

## Linux 3.1 中的 V4L2


1. VIDIOC_QUERYCAP now returns a per-subsystem version instead of a
   per-driver one.

1. VIDIOC_QUERYCAP 现在返回每个子系统（per-subsystem）的版本，而非
   每个驱动（per-driver）的版本

   Standardize an error code for invalid ioctl.

   为无效的 ioctl 统一了错误码

   Added V4L2_CTRL_TYPE_BITMASK.

   新增V4L2_CTRL_TYPE_BITMASK

## V4L2 in Linux 3.2

## Linux 3.2 中的 V4L2


1. V4L2_CTRL_FLAG_VOLATILE was added to signal volatile controls to
   userspace.

1. 新增V4L2_CTRL_FLAG_VOLATILE，用于向用户态标示易变（volatile
   的控制项

2. Add selection API for extended control over cropping and composing.
   Does not affect the compatibility of current drivers and
   applications. See selection API <selection-api> for details.

2. 新增了选择（selection）API，用于扩展对裁剪（cropping）和合成
   （composing）的控制。不影响当前驱动和应用程序的兼容性。详
   选择 API <selection-api>

## V4L2 in Linux 3.3

## Linux 3.3 中的 V4L2


1. Added `V4L2_CID_ALPHA_COMPONENT` control to the
   User controls class <control>.

1. 向用户控制类 <control> 中新增了 `V4L2_CID_ALPHA_COMPONENT` 控制项

2. Added the device_caps field to struct v4l2_capabilities and added
   the new V4L2_CAP_DEVICE_CAPS capability.

2. struct v4l2_capabilities 中新增了 device_caps 字段，并新增
   新的 V4L2_CAP_DEVICE_CAPS 能力

## V4L2 in Linux 3.4

## Linux 3.4 中的 V4L2


1. Added JPEG compression control class <jpeg-controls>.

1. 新增JPEG 压缩控制<jpeg-controls>

2. Extended the DV Timings API:
   VIDIOC_ENUM_DV_TIMINGS,
   VIDIOC_QUERY_DV_TIMINGS and
   VIDIOC_DV_TIMINGS_CAP.

2. 扩展DV Timings API：新增了 VIDIOC_ENUM_DV_TIMINGS
   VIDIOC_QUERY_DV_TIMINGS 鍜?VIDIOC_DV_TIMINGS_CAP銆。

## V4L2 in Linux 3.5

## Linux 3.5 中的 V4L2


1. Added integer menus, the new type will be
   V4L2_CTRL_TYPE_INTEGER_MENU.

1. 新增了整数菜单，新类型为 V4L2_CTRL_TYPE_INTEGER_MENU

2. Added selection API for V4L2 subdev interface:
   VIDIOC_SUBDEV_G_SELECTION and
   VIDIOC_SUBDEV_S_SELECTION <VIDIOC_SUBDEV_G_SELECTION>.

2. V4L2 子设备（subdev）接口新增了选择 API
   VIDIOC_SUBDEV_G_SELECTION 鍜。
   VIDIOC_SUBDEV_S_SELECTION <VIDIOC_SUBDEV_G_SELECTION>銆。

3. Added `V4L2_COLORFX_ANTIQUE`, `V4L2_COLORFX_ART_FREEZE`,
   `V4L2_COLORFX_AQUA`, `V4L2_COLORFX_SILHOUETTE`,
   `V4L2_COLORFX_SOLARIZATION`, `V4L2_COLORFX_VIVID` and
   `V4L2_COLORFX_ARBITRARY_CBCR` menu items to the
   `V4L2_CID_COLORFX` control.

3. `V4L2_CID_COLORFX` 控制项新增了菜单
   `V4L2_COLORFX_ANTIQUE`、`V4L2_COLORFX_ART_FREEZE`
   `V4L2_COLORFX_AQUA`、`V4L2_COLORFX_SILHOUETTE`
   `V4L2_COLORFX_SOLARIZATION`、`V4L2_COLORFX_VIVID` 
   `V4L2_COLORFX_ARBITRARY_CBCR`銆。

4. Added `V4L2_CID_COLORFX_CBCR` control.

4. 新增`V4L2_CID_COLORFX_CBCR` 控制项

5. Added camera controls `V4L2_CID_AUTO_EXPOSURE_BIAS`,
   `V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`,
   `V4L2_CID_IMAGE_STABILIZATION`, `V4L2_CID_ISO_SENSITIVITY`,
   `V4L2_CID_ISO_SENSITIVITY_AUTO`, `V4L2_CID_EXPOSURE_METERING`,
   `V4L2_CID_SCENE_MODE`, `V4L2_CID_3A_LOCK`,
   `V4L2_CID_AUTO_FOCUS_START`, `V4L2_CID_AUTO_FOCUS_STOP`,
   `V4L2_CID_AUTO_FOCUS_STATUS` and `V4L2_CID_AUTO_FOCUS_RANGE`.

5. 新增了相机控制项 `V4L2_CID_AUTO_EXPOSURE_BIAS`
   `V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`、`V4L2_CID_IMAGE_STABILIZATION`
   `V4L2_CID_ISO_SENSITIVITY`、`V4L2_CID_ISO_SENSITIVITY_AUTO`
   `V4L2_CID_EXPOSURE_METERING`、`V4L2_CID_SCENE_MODE`
   `V4L2_CID_3A_LOCK`、`V4L2_CID_AUTO_FOCUS_START`
   `V4L2_CID_AUTO_FOCUS_STOP`、`V4L2_CID_AUTO_FOCUS_STATUS` 
   `V4L2_CID_AUTO_FOCUS_RANGE`銆。

## V4L2 in Linux 3.6

## Linux 3.6 中的 V4L2


1. Replaced `input` in struct v4l2_buffer by
   `reserved2` and removed `V4L2_BUF_FLAG_INPUT`.

1. struct v4l2_buffer 中的 `input` 替换`reserved2`，并
   移除`V4L2_BUF_FLAG_INPUT`

2. Added V4L2_CAP_VIDEO_M2M and V4L2_CAP_VIDEO_M2M_MPLANE
   capabilities.

2. 新增V4L2_CAP_VIDEO_M2M V4L2_CAP_VIDEO_M2M_MPLANE 能力

3. Added support for frequency band enumerations:
   VIDIOC_ENUM_FREQ_BANDS.

3. 新增了对频带（frequency band）枚举的支持：VIDIOC_ENUM_FREQ_BANDS

## V4L2 in Linux 3.9

## Linux 3.9 中的 V4L2


1. Added timestamp types to `flags` field in
   struct v4l2_buffer. See buffer-flags.

1. struct v4l2_buffer `flags` 字段中新增了时间戳类型
   详见 buffer-flags

2. Added `V4L2_EVENT_CTRL_CH_RANGE` control event changes flag. See
   ctrl-changes-flags.

2. 新增`V4L2_EVENT_CTRL_CH_RANGE` 控制事件变更标志。详
   ctrl-changes-flags銆。

## V4L2 in Linux 3.10

## Linux 3.10 中的 V4L2


1. Removed obsolete and unused DV_PRESET ioctls VIDIOC_G_DV_PRESET,
   VIDIOC_S_DV_PRESET, VIDIOC_QUERY_DV_PRESET and
   VIDIOC_ENUM_DV_PRESET. Remove the related v4l2_input/output
   capability flags V4L2_IN_CAP_PRESETS and V4L2_OUT_CAP_PRESETS.

1. 移除了过时且未使用的 DV_PRESET ioctls：VIDIOC_G_DV_PRESET
   VIDIOC_S_DV_PRESET、VIDIOC_QUERY_DV_PRESET 
   VIDIOC_ENUM_DV_PRESET。移除了相关v4l2_input/output 能力标志
   V4L2_IN_CAP_PRESETS 鍜?V4L2_OUT_CAP_PRESETS銆。

2. Added new debugging ioctl
   VIDIOC_DBG_G_CHIP_INFO.

2. 新增了调ioctl VIDIOC_DBG_G_CHIP_INFO

## V4L2 in Linux 3.11

## Linux 3.11 中的 V4L2


1. Remove obsolete `VIDIOC_DBG_G_CHIP_IDENT` ioctl.

1. 移除已过时的 `VIDIOC_DBG_G_CHIP_IDENT` ioctl

## V4L2 in Linux 3.14

## Linux 3.14 中的 V4L2


1. In struct v4l2_rect, the type of `width` and
   `height` fields changed from _s32 to _u32.

1. struct v4l2_rect 中，`width` `height` 字段的类型从
   _s32 改为 _u32

## V4L2 in Linux 3.15

## Linux 3.15 中的 V4L2


1. Added Software Defined Radio (SDR) Interface.

1. 新增了软件定义无线电（SDR）接口

## V4L2 in Linux 3.16

## Linux 3.16 中的 V4L2


1. Added event V4L2_EVENT_SOURCE_CHANGE.

1. 新增了事V4L2_EVENT_SOURCE_CHANGE

## V4L2 in Linux 3.17

## Linux 3.17 中的 V4L2


1. Extended struct v4l2_pix_format. Added
   format flags.

1. 扩展struct v4l2_pix_format。新增了格式标志

2. Added compound control types and
   VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>.

2. 新增了复合（compound）控制类型以
   VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>銆。

## V4L2 in Linux 3.18

## Linux 3.18 中的 V4L2


1. Added `V4L2_CID_PAN_SPEED` and `V4L2_CID_TILT_SPEED` camera
   controls.

1. 新增了相机控制项 `V4L2_CID_PAN_SPEED` `V4L2_CID_TILT_SPEED`

## V4L2 in Linux 3.19

## Linux 3.19 中的 V4L2


1. Rewrote Colorspace chapter, added new enum v4l2_ycbcr_encoding
   and enum v4l2_quantization fields to struct v4l2_pix_format,
   struct v4l2_pix_format_mplane and struct v4l2_mbus_framefmt.

1. 重写Colorspace 章节，向 struct v4l2_pix_format
   struct v4l2_pix_format_mplane 鍜?struct v4l2_mbus_framefmt 涓。
   新增enum v4l2_ycbcr_encoding enum v4l2_quantization 字段

## V4L2 in Linux 4.4

## Linux 4.4 中的 V4L2


1. Renamed `V4L2_TUNER_ADC` to `V4L2_TUNER_SDR`. The use of
   `V4L2_TUNER_ADC` is deprecated now.

2. Added `V4L2_CID_RF_TUNER_RF_GAIN` RF Tuner control.

2. 新增`V4L2_CID_RF_TUNER_RF_GAIN` RF 调谐器控制项

3. Added transmitter support for Software Defined Radio (SDR) Interface.

3. 新增了对软件定义无线电（SDR）接口的发射器（transmitter）支持


## Relation of V4L2 to other Linux multimedia APIs

## V4L2 与其Linux 多媒API 的关


### X Video Extension

### X Video 扩展


The X Video Extension (abbreviated XVideo or just Xv) is an extension of
the X Window system, implemented for example by the XFree86 project. Its
scope is similar to V4L2, an API to video capture and output devices for
X clients. Xv allows applications to display live video in a window,
send window contents to a TV output, and capture or output still images
in XPixmaps [#f1]_. With their implementation XFree86 makes the extension
available across many operating systems and architectures.

X Video 扩展（简XVideo Xv）是 X Window 系统的一个扩展，
XFree86 项目等实现。它的作用范围与 V4L2 类似，都是面X 客户端的
视频采集和输出设API。Xv 允许应用程序在窗口中显示实时视频、将
窗口内容发送到电视输出，并XPixmaps [#f1]_ 中采集或输出静态图像
通过 XFree86 的实现，该扩展在许多操作系统和架构上均可使用

Because the driver is embedded into the X server Xv has a number of
advantages over the V4L2 video overlay interface <overlay>. The
driver can easily determine the overlay target, i. e. visible graphics
memory or off-screen buffers for a destructive overlay. It can program
the RAMDAC for a non-destructive overlay, scaling or color-keying, or
the clipping functions of the video capture hardware, always in sync
with drawing operations or windows moving or changing their stacking
order.

由于驱动嵌入X 服务器中，Xv 相对V4L2 视频叠加（overlay）接
<overlay> 具有若干优势。驱动可以轻松地确定叠加目标，即可见
图形显存或用于破坏性叠加（destructive overlay）的离屏缓冲区。它
可以编程 RAMDAC 以实现非破坏性叠加、缩放或色键（color-keying），
或者利用视频采集硬件的裁剪功能，并且始终与绘图操作或窗口移动
改变堆叠顺序保持同步

To combine the advantages of Xv and V4L a special Xv driver exists in
XFree86 and XOrg, just programming any overlay capable Video4Linux
device it finds. To enable it `/etc/X11/XF86Config` must contain these
lines:

为了结合 Xv V4L 的优势，XFree86 XOrg 中存在一个特殊的 Xv 驱动
它会对它发现的任何支持叠加的 Video4Linux 设备进行编程。要启用它，
`/etc/X11/XF86Config` 必须包含以下行：

```
    Section "Module"
	Load "v4l"
    EndSection
```

As of XFree86 4.2 this driver still supports only V4L ioctls, however it
should work just fine with all V4L2 devices through the V4L2
backward-compatibility layer. Since V4L2 permits multiple opens it is
possible (if supported by the V4L2 driver) to capture video while an X
client requested video overlay. Restrictions of simultaneous capturing
and overlay are discussed in overlay apply.

截至 XFree86 4.2，该驱动仍然只支V4L ioctls，不过通过 V4L2 
向后兼容层，它应该能与所V4L2 设备正常工作。由V4L2 允许多次
打开，因此（V4L2 驱动支持的情况下）可以在 X 客户端请求视频叠加的
同时采集视频。同时采集和叠加的限制在 overlay apply 中讨论

Only marginally related to V4L2, XFree86 extended Xv to support hardware
YUV to RGB conversion and scaling for faster video playback, and added
an interface to MPEG-2 decoding hardware. This API is useful to display
images captured with V4L2 devices.

V4L2 仅有少许关联，XFree86 扩展Xv 以支持硬YUV RGB 转换
缩放，从而实现更快的视频播放，并增加了对 MPEG-2 解码硬件的接口
API 可用于显示用 V4L2 设备采集的图像

### Digital Video

### 数字视频


V4L2 does not support digital terrestrial, cable or satellite broadcast.
A separate project aiming at digital receivers exists. You can find its
homepage at `https://linuxtv.org <https://linuxtv.org>`__. The Linux
DVB API has no connection to the V4L2 API except that drivers for hybrid
hardware may support both.

V4L2 不支持数字地面、有线或卫星广播。存在一个面向数字接收器的独
项目。你可以`https://linuxtv.org <https://linuxtv.org>`__ 找到它的
主页。Linux DVB API V4L2 API 没有关系，只是混合（hybrid）硬件的
驱动可能同时支持两者

### Audio Interfaces

### 音频接口


[to do - OSS/ALSA]

[待办 - OSS/ALSA]


## Experimental API Elements

## 瀹為獙鎬?API 鍏冪礌


The following V4L2 API elements are currently experimental and may
change in the future.

以下 V4L2 API 元素目前是实验性的，将来可能会发生变化

- VIDIOC_DBG_G_REGISTER and
   VIDIOC_DBG_S_REGISTER <VIDIOC_DBG_G_REGISTER> ioctls.

- VIDIOC_DBG_G_REGISTER 鍜。
   VIDIOC_DBG_S_REGISTER <VIDIOC_DBG_G_REGISTER> ioctls銆。

- VIDIOC_DBG_G_CHIP_INFO ioctl.

- VIDIOC_DBG_G_CHIP_INFO ioctl銆。


## Obsolete API Elements

## 已废弃的 API 元素


The following V4L2 API elements were superseded by new interfaces and
should not be implemented in new drivers.

以下 V4L2 API 元素已被新接口取代，不应在新驱动中实现

- `VIDIOC_G_MPEGCOMP` and `VIDIOC_S_MPEGCOMP` ioctls. Use Extended
   Controls, extended-controls.

- `VIDIOC_G_MPEGCOMP` `VIDIOC_S_MPEGCOMP` ioctls。请使用扩展
   控制（Extended Controls），extended-controls

- VIDIOC_G_DV_PRESET, VIDIOC_S_DV_PRESET,
   VIDIOC_ENUM_DV_PRESETS and VIDIOC_QUERY_DV_PRESET ioctls. Use
   the DV Timings API (dv-timings).

- VIDIOC_G_DV_PRESET、VIDIOC_S_DV_PRESET
   VIDIOC_ENUM_DV_PRESETS VIDIOC_QUERY_DV_PRESET ioctls。请使用
   DV Timings API（dv-timings）

- `VIDIOC_SUBDEV_G_CROP` and `VIDIOC_SUBDEV_S_CROP` ioctls. Use
   `VIDIOC_SUBDEV_G_SELECTION` and `VIDIOC_SUBDEV_S_SELECTION`,
   VIDIOC_SUBDEV_G_SELECTION.

- `VIDIOC_SUBDEV_G_CROP` `VIDIOC_SUBDEV_S_CROP` ioctls。请使用
   `VIDIOC_SUBDEV_G_SELECTION` 鍜?`VIDIOC_SUBDEV_S_SELECTION`锛。
   VIDIOC_SUBDEV_G_SELECTION銆。

   This is not implemented in XFree86.

   这在 XFree86 中未实现

