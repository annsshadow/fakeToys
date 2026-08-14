


## DVB OSD 设备

             See: legacy_dvb_decoder_notes

DVB OSD 设备控制基于 AV7110、带有硬件 MPEG2 解码器的 DVB 卡的屏上显示（OnScreen-Display）。它可以通过 `/dev/dvb/adapter?/osd0` 访问。数据类型和 ioctl 定义可以通过在应用程序中包含 `linux/dvb/osd.h` 来使用。

OSD 并不像许多其他卡那样是一个帧缓冲（frame-buffer）。它更像是一块可以绘制的画布（canvas）。色深受所安装内存大小的限制。必须建立一套合适的调色板。所安装的内存大小可以用 `OSD_GET_CAPABILITY`_ ioctl 来识别。

## OSD 数据类型

### OSD_Command

#### Synopsis（概要）


    typedef enum {
	/** All functions return -2 on "not open" **/
	OSD_Close = 1,
	OSD_Open,
	OSD_Show,
	OSD_Hide,
	OSD_Clear,
	OSD_Fill,
	OSD_SetColor,
	OSD_SetPalette,
	OSD_SetTrans,
	OSD_SetPixel,
	OSD_GetPixel,
	OSD_SetRow,
	OSD_SetBlock,
	OSD_FillRow,
	OSD_FillBlock,
	OSD_Line,
	OSD_Query,
	OSD_Test,
	OSD_Text,
	OSD_SetWindow,
	OSD_MoveWindow,
	OSD_OpenRaw,
    } OSD_Command;

#### Commands（命令）


    :header-rows:  1
    :stub-columns: 0

    - ..

       - Command

       - | 所使用的 `struct` `osd_cmd_t`_ 变量。
          | 如为可选用法，则为 Usage{variable}。

       - `2` Description

    - ..

       - `OSD_Close`

       - -

       - | 禁用 OSD 并释放缓冲区。
          | 成功时返回 0。

    - ..

       - `OSD_Open`

       - | x0,y0,x1,y1,
          | BitPerPixel[2/4/8]{color&0x0F},
          | mix[0..15]{color&0xF0}

       - | 以该尺寸和位深打开 OSD。
          | 成功时返回 0，
          | DRAM 分配错误时返回 -1，
          | “已经打开” 时返回 -2。

    - ..

       - `OSD_Show`

       - -

       - | 启用 OSD 模式。
          | 成功时返回 0。

    - ..

       - `OSD_Hide`

       - -

       - | 禁用 OSD 模式。
          | 成功时返回 0。

    - ..

       - `OSD_Clear`

       - -

       - | 将所有像素设为颜色 0。
          | 成功时返回 0。

    - ..

       - `OSD_Fill`

       - color

       - | 将所有像素设为颜色 <color>。
          | 成功时返回 0。

    - ..

       - `OSD_SetColor`

       - | color,
          | R{x0},G{y0},B{x1},
          | opacity{y1}

       - | 将调色板条目 <num> 设为 <r,g,b>，<mix> 和 <trans> 生效。
          | R,G,B: 0..255
          | R=红（Red），G=绿（Green），B=蓝（Blue）
          | opacity=0:      像素不透明度 0%（只显示视频像素）
          | opacity=1..254: 像素不透明度如头部所指定
          | opacity=255:    像素不透明度 100%（只显示 OSD 像素）
          | 成功时返回 0，出错时返回 -1。

    - ..

       - `OSD_SetPalette`

       - | firstcolor{color},
          | lastcolor{x0},data

       - | 设置调色板中的若干条目。
          | 从数组 "data" 中设置 "firstcolor" 到 "lastcolor" 的条目。
          | 每个颜色占 4 字节：
          | R、G、B 与一个不透明度值：0->透明，1..254->混合，255->像素

    - ..

       - `OSD_SetTrans`

       - transparency{color}

       - | 设置混合像素的不透明度（0..15）。
          | 成功时返回 0。

    - ..

       - `OSD_SetPixel`

       - x0,y0,color

       - | 将像素 <x>,<y> 设为颜色编号 <color>。
          | 成功时返回 0，出错时返回 -1。

    - ..

       - `OSD_GetPixel`

       - x0,y0

       - | 返回像素 <x>,<y> 的颜色编号，或 -1。
          | 该命令目前 AV7110 尚不支持！

    - ..

       - `OSD_SetRow`

       - x0,y0,x1,data

       - | 用 data[] 的内容填充像素 x0,y 到 x1,y。
          | 成功时返回 0，所有像素被裁剪时（未绘制任何像素）返回 -1。

    - ..

       - `OSD_SetBlock`

       - | x0,y0,x1,y1,
          | increment{color},
          | data

       - | 用 data[] 的内容填充像素 x0,y0 到 x1,y1。
          | Inc 包含数据块中一行的宽度，
          | inc<=0 时使用块宽度作为行宽。
          | 成功时返回 0，所有像素被裁剪时返回 -1。

    - ..

       - `OSD_FillRow`

       - x0,y0,x1,color

       - | 用颜色 <color> 填充像素 x0,y 到 x1,y。
          | 成功时返回 0，所有像素被裁剪时返回 -1。

    - ..

       - `OSD_FillBlock`

       - x0,y0,x1,y1,color

       - | 用颜色 <color> 填充像素 x0,y0 到 x1,y1。
          | 成功时返回 0，所有像素被裁剪时返回 -1。

    - ..

       - `OSD_Line`

       - x0,y0,x1,y1,color

       - | 用颜色 <color> 从 x0,y0 到 x1,y1 画一条线。
          | 成功时返回 0。

    - ..

       - `OSD_Query`

       - | x0,y0,x1,y1,
          | xasp{color}; yasp=11

       - | 用图像尺寸与像素长宽比填充参数。
          | 成功时返回 0。
          | 该命令目前 AV7110 尚不支持！

    - ..

       - `OSD_Test`

       - -

       - | 绘制一张测试图。
          | 仅用于调试目的。
          | 成功时返回 0。
    - ..

       - `OSD_Text`

       - x0,y0,size,color,text

       - 在位置 x0,y0 用颜色 <color> 绘制一段文本。

    - ..

       - `OSD_SetWindow`

       - x0

       - 将编号为 0<x0<8 的窗口设为当前窗口。

    - ..

       - `OSD_MoveWindow`

       - x0,y0

       - 将当前窗口移动到 (x0, y0)。

    - ..

       - `OSD_OpenRaw`

       - | x0,y0,x1,y1,
          | `osd_raw_window_t`_ {color}

       - 打开其他类型的 OSD 窗口。

#### Description（说明）


`OSD_Command` 数据类型与 `OSD_SEND_CMD`_ ioctl 配合使用，用于告知驱动要执行哪个 OSD_Command。


-----

### osd_cmd_t

#### Synopsis（概要）


    typedef struct osd_cmd_s {
	OSD_Command cmd;
	int x0;
	int y0;
	int x1;
	int y1;
	int color;
	void __user *data;
    } osd_cmd_t;

#### Variables（变量）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_Command cmd`

       - 待执行的 `OSD_Command`_。

    - ..

       - `int x0`

       - 第一个水平位置。

    - ..

       - `int y0`

       - 第一个垂直位置。

    - ..

       - `int x1`

       - 第二个水平位置。

    - ..

       - `int y1`

       - 第二个垂直位置。

    - ..

       - `int color`

       - 调色板中颜色的编号。

    - ..

       - `void __user *data`

       - 命令相关的数据。

#### Description（说明）


`osd_cmd_t` 数据类型与 `OSD_SEND_CMD`_ ioctl 配合使用。它包含 OSD_Command 的数据以及 `OSD_Command`_ 本身。该结构必须传给驱动，其各组成部分可能会被驱动修改。


-----

### osd_raw_window_t

#### Synopsis（概要）


    typedef enum {
	OSD_BITMAP1,
	OSD_BITMAP2,
	OSD_BITMAP4,
	OSD_BITMAP8,
	OSD_BITMAP1HR,
	OSD_BITMAP2HR,
	OSD_BITMAP4HR,
	OSD_BITMAP8HR,
	OSD_YCRCB422,
	OSD_YCRCB444,
	OSD_YCRCB444HR,
	OSD_VIDEOTSIZE,
	OSD_VIDEOHSIZE,
	OSD_VIDEOQSIZE,
	OSD_VIDEODSIZE,
	OSD_VIDEOTHSIZE,
	OSD_VIDEOTQSIZE,
	OSD_VIDEOTDSIZE,
	OSD_VIDEONSIZE,
	OSD_CURSOR
    } osd_raw_window_t;

#### Constants（常量）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_BITMAP1`

       - `1` 1 位位图

    - ..

       - `OSD_BITMAP2`

       - 2 位位图

    - ..

       - `OSD_BITMAP4`

       - 4 位位图

    - ..

       - `OSD_BITMAP8`

       - 8 位位图

    - ..

       - `OSD_BITMAP1HR`

       - 1 位位图，半分辨率

    - ..

       - `OSD_BITMAP2HR`

       - 2 位位图，半分辨率

    - ..

       - `OSD_BITMAP4HR`

       - 4 位位图，半分辨率

    - ..

       - `OSD_BITMAP8HR`

       - 8 位位图，半分辨率

    - ..

       - `OSD_YCRCB422`

       - 4:2:2 YCRCB 图形显示

    - ..

       - `OSD_YCRCB444`

       - 4:4:4 YCRCB 图形显示

    - ..

       - `OSD_YCRCB444HR`

       - 4:4:4 YCRCB 图形，半分辨率

    - ..

       - `OSD_VIDEOTSIZE`

       - 真实尺寸 常规 MPEG 视频显示

    - ..

       - `OSD_VIDEOHSIZE`

       - MPEG 视频显示 半分辨率

    - ..

       - `OSD_VIDEOQSIZE`

       - MPEG 视频显示 四分之一分辨率

    - ..

       - `OSD_VIDEODSIZE`

       - MPEG 视频显示 双倍分辨率

    - ..

       - `OSD_VIDEOTHSIZE`

       - 真实尺寸 MPEG 视频显示 半分辨率

    - ..

       - `OSD_VIDEOTQSIZE`

       - 真实尺寸 MPEG 视频显示 四分之一分辨率

    - ..

       - `OSD_VIDEOTDSIZE`

       - 真实尺寸 MPEG 视频显示 双倍分辨率

    - ..

       - `OSD_VIDEONSIZE`

       - 全尺寸 MPEG 视频显示

    - ..

       - `OSD_CURSOR`

       - 光标

#### Description（说明）


`osd_raw_window_t` 数据类型与 `OSD_Command`_ 的 OSD_OpenRaw 配合使用，用于告知驱动要打开哪种类型的 OSD。


-----

### osd_cap_t

#### Synopsis（概要）


    typedef struct osd_cap_s {
	int  cmd;
    #define OSD_CAP_MEMSIZE         1
	long val;
    } osd_cap_t;

#### Variables（变量）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int  cmd`

       - 要查询的能力。

    - ..

       - `long val`

       - 用于存储数据。

#### Supported capabilities（受支持的能力）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `OSD_CAP_MEMSIZE`

       - 卡上安装的内存大小。

#### Description（说明）


该数据结构与 `OSD_GET_CAPABILITY`_ 调用配合使用。


-----

## OSD Function Calls（OSD 函数调用）

### OSD_SEND_CMD

#### Synopsis（概要）



    int ioctl(int fd, int request = OSD_SEND_CMD, enum osd_cmd_t *cmd)

#### Arguments（参数）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由之前对 `open()`_ 的调用所返回的文件描述符。

    - ..

       - `int request`

       - 指向该命令所用的 `osd_cmd_t`_ 结构所在位置的指针。

#### Description（说明）


             See: legacy_dvb_decoder_notes

该 ioctl 将 `OSD_Command`_ 发送给卡。

#### Return Value（返回值）


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 一章中描述。

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EINVAL`

       - 命令超出范围。


-----

### OSD_GET_CAPABILITY

#### Synopsis（概要）



    int ioctl(int fd, int request = OSD_GET_CAPABILITY,
    struct osd_cap_t *cap)

#### Arguments（参数）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由之前对 `open()`_ 的调用所返回的文件描述符。

    - ..

       - `int request`

       - 该命令等于 `OSD_GET_CAPABILITY`。

    - ..

       - `unsigned int *cap`

       - 指向该命令所用的 `osd_cap_t`_ 结构所在位置的指针。

#### Description（说明）


             See: legacy_dvb_decoder_notes

该 ioctl 用于获取正在使用的、基于 AV7110 的 DVB 解码器卡的 OSD 的能力。

    结构 osd_cap_t 必须由用户设置并传给驱动。

#### Return Value（返回值）


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 一章中描述。

    :header-rows:  0
    :stub-columns: 0


    - ..

       - `EINVAL`

       - 不支持的能力。


-----

### open()

#### Synopsis（概要）



    #include <fcntl.h>

#### Arguments（参数）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 特定 OSD 设备的名称。

    - ..

       - `3` `int flags`

       - `1` 下列标志的按位或：

    - ..

       - `O_RDONLY`

       - 只读访问

    - ..

       - `O_RDWR`

       - 读写访问

    - ..

       - `O_NONBLOCK`
       - | 以非阻塞模式打开
          | （默认是阻塞模式）

#### Description（说明）


该 system call 打开一个具名的 OSD 设备（例如 `/dev/dvb/adapter?/osd0`）以供后续使用。

#### Return Value（返回值）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - 设备驱动未加载/不可用。

    - ..

       - `EINTERNAL`

       - 内部错误。

    - ..

       - `EBUSY`

       - 设备或资源忙。

    - ..

       - `EINVAL`

       - 无效参数。


-----

### close()

#### Synopsis（概要）



#### Arguments（参数）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由之前对 `open()`_ 的调用所返回的文件描述符。

#### Description（说明）


该 system call 关闭一个先前打开的 OSD 设备。

#### Return Value（返回值）


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - fd 不是一个有效的已打开文件描述符。
