

## DVB 视频设备（DVB Video Device

             See: legacy_dvb_decoder_notes

DVB 视频设备控制 DVB 硬件MPEG2 视频解码器。可以通过 `/dev/dvb/adapter0/video0` 访问它。数据类型和 ioctl 定义可以通过在应用程序中包含 `linux/dvb/video.h` 来访问
注意，DVB 视频设备只控MPEG 视频流的解码，而不是其在电视或计算机屏幕上的呈现。在 PC 上，这通常由相关的 video4linux 设备（例`/dev/video`）处理，它允许缩放和定义输出窗口
大多DVB 卡没有自己的 MPEG 解码器，这导致音频和视频设备以及 video4linux 设备被省略
这些 ioctl 也曾V4L2 用来控制 V4L2 中实现的 MPEG 解码器。将这些 ioctl 用于此目的的做法已被废弃，并且已经创建了适当V4L2 ioctl 或控制来取代该功能。请为新驱动使用 V4L2 ioctls<video>
## 视频数据类型（Video Data Types

### video_format_t


#### 概要（Synopsis

    typedef enum {
	VIDEO_FORMAT_4_3,
	VIDEO_FORMAT_16_9,
	VIDEO_FORMAT_221_1
    } video_format_t;

#### 常量（Constants

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_FORMAT_4_3`

       - 选择 4:3 格式
    - ..

       - `VIDEO_FORMAT_16_9`

       - 选择 16:9 格式
    - ..

       - `VIDEO_FORMAT_221_1`

       - 选择 2.21:1 格式
#### 描述（Description

`video_format_t` 数据类型
`VIDEO_SET_FORMAT`_ 函数中用于告诉驱动输出硬件（例如电视）具有哪宽高比。它也用于由 `VIDEO_GET_STATUS`_ 返回数据结构 `video_status`_ 以及`VIDEO_GET_EVENT`_ 返回`video_event`_ 中，这些结构报告当前视频流的显示格式

-----


### video_displayformat_t


#### 概要（Synopsis

    typedef enum {
	VIDEO_PAN_SCAN,
	VIDEO_LETTER_BOX,
	VIDEO_CENTER_CUT_OUT
    } video_displayformat_t;

#### 常量（Constants

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_PAN_SCAN`

       - 使用平移和扫描（pan and scan）格式
    - ..

       - `VIDEO_LETTER_BOX`

       - 使用信箱（letterbox）格式
    - ..

       - `VIDEO_CENTER_CUT_OUT`

       - 使用中心裁剪（center cut out）格式
#### 描述（Description

如果视频流的显示格式与显示硬件的显示格式不同，应用程序必须指定如何处画面的裁剪。这可以通过接受此枚举作为参数的
`VIDEO_SET_DISPLAY_FORMAT`_ 调用来完成

-----


### video_size_t


#### 概要（Synopsis

    typedef struct {
	int w;
	int h;
	video_format_t aspect_ratio;
    } video_size_t;

#### 变量（Variables

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int w`

       - 视频宽度（像素）
    - ..

       - `int h`

       - 视频高度（像素）
    - ..

       - `video_format_t`_ `aspect_ratio`

       - 宽高比
#### 描述（Description

用于结构`video_event`_ 中。它存储视频的分辨率和宽高比

-----


### video_stream_source_t


#### 概要（Synopsis

    typedef enum {
	VIDEO_SOURCE_DEMUX,
	VIDEO_SOURCE_MEMORY
    } video_stream_source_t;

#### 常量（Constants

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_SOURCE_DEMUX`

       - `1` 选择解复用器（demux）作为主源
    - ..

       - `VIDEO_SOURCE_MEMORY`

       - 如果选择了此源，则流
          来自用户通过 write
          系统调用
#### 描述（Description

视频流源通过 `VIDEO_SELECT_SOURCE`_ 调用设置，并且根据我们是从内部（解复用器还是外部（用户写入）源回放，可以取以下值VIDEO_SOURCE_DEMUX 选择解复用器（由前端DVR 设备提供）作为视频流的源。如选择 VIDEO_SOURCE_MEMORY，则流来自应用程序，通过 `write()`_ 系统调用

-----


### video_play_state_t


#### 概要（Synopsis

    typedef enum {
	VIDEO_STOPPED,
	VIDEO_PLAYING,
	VIDEO_FREEZED
    } video_play_state_t;

#### 常量（Constants

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_STOPPED`

       - 视频已停止
    - ..

       - `VIDEO_PLAYING`

       - 视频正在播放
    - ..

       - `VIDEO_FREEZED`

       - 视频已冻结
#### 描述（Description

这些值可以由 `VIDEO_GET_STATUS`_ 调用返回，表示视频播放的状态

-----


### struct video_command


#### 概要（Synopsis

    struct video_command {
	__u32 cmd;
	__u32 flags;
	union {
	    struct {
		__u64 pts;
	    } stop;

	    struct {
		__s32 speed;
		__u32 format;
	    } play;

	    struct {
		__u32 data[^16^];
	    } raw;
	};
    };


#### 变量（Variables

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `__u32 cmd`

       - `Decoder command`_

    - ..

       - `__u32 flags`

       - `Decoder command`_ 的标志
    - ..

       - `struct stop`

       - `__u64 pts`

       - MPEG PTS

    - ..

       - `5` `stuct play`

       - `4` `__s32 speed`

       - 0 1000 表示正常速度
    - ..

       - 1：表示正向单步，

    - ..

       - -1：表示反向单步，

    - ..

       - >1：以正常速度speed / 1000 倍播
    - ..

       - <-1：以正常速度( -speed / 1000 ) 倍反向播放
    - ..

       - `__u32 format`

       - `Play input formats`_

    - ..

       - `__u32 data[^16^]`

       - 保留

#### 描述（Description

该结构体在使用前必须由应用程序清零。这确保了它将来可以安全地扩展

-----


### 预定义的译码器命令与标志（Predefined decoder commands and flags

#### 概要（Synopsis

    #define VIDEO_CMD_PLAY                      (0)
    #define VIDEO_CMD_STOP                      (1)
    #define VIDEO_CMD_FREEZE                    (2)
    #define VIDEO_CMD_CONTINUE                  (3)

    #define VIDEO_CMD_FREEZE_TO_BLACK      (1 << 0)

    #define VIDEO_CMD_STOP_TO_BLACK        (1 << 0)
    #define VIDEO_CMD_STOP_IMMEDIATELY     (1 << 1)

    #define VIDEO_PLAY_FMT_NONE                 (0)
    #define VIDEO_PLAY_FMT_GOP                  (1)

    #define VIDEO_VSYNC_FIELD_UNKNOWN           (0)
    #define VIDEO_VSYNC_FIELD_ODD               (1)
    #define VIDEO_VSYNC_FIELD_EVEN              (2)
    #define VIDEO_VSYNC_FIELD_PROGRESSIVE       (3)

#### 常量（Constants

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `3` _`Decoder command`

       - `VIDEO_CMD_PLAY`

       - 开始播放
    - ..

       - `VIDEO_CMD_STOP`

       - 停止播放
    - ..

       - `VIDEO_CMD_FREEZE`

       - 冻结播放
    - ..

       - `VIDEO_CMD_CONTINUE`

       - 冻结后继续播放
    - ..

       - `VIDEO_CMD_FREEZE` 的标
       - `VIDEO_CMD_FREEZE_TO_BLACK`

       - 冻结时显示黑屏
    - ..

       - `1` `VIDEO_CMD_STOP` 的标
       - `VIDEO_CMD_STOP_TO_BLACK`

       - 停止时显示黑屏
    - ..

       - `VIDEO_CMD_STOP_IMMEDIATELY`

       - 立即停止，不排空缓冲区
    - ..

       - `1` _`Play input formats`

       - `VIDEO_PLAY_FMT_NONE`

       - 解码器没有特殊的格式要求

    - ..

       - `VIDEO_PLAY_FMT_GOP`

       - 解码器需要完整的 GOP

    - ..

       - `3` 场顺序（Field order
       - `VIDEO_VSYNC_FIELD_UNKNOWN`

       - 如果硬件不知Vsync 是对应奇数场          偶数场还是逐行（即非隔行）场，可以使用 FIELD_UNKNOWN
    - ..

       - `VIDEO_VSYNC_FIELD_ODD`

       - Vsync 对应奇数场
    - ..

       - `VIDEO_VSYNC_FIELD_EVEN`

       - Vsync 对应偶数场
    - ..

       - `VIDEO_VSYNC_FIELD_PROGRESSIVE`

       - 逐行（即非隔行）


-----


### video_event


#### 概要（Synopsis

    struct video_event {
	__s32 type;
    #define VIDEO_EVENT_SIZE_CHANGED        1
    #define VIDEO_EVENT_FRAME_RATE_CHANGED  2
    #define VIDEO_EVENT_DECODER_STOPPED     3
    #define VIDEO_EVENT_VSYNC               4
	long timestamp;
	union {
	    video_size_t size;
	    unsigned int frame_rate;
	    unsigned char vsync_field;
	} u;
    };

#### 变量（Variables

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `4` `__s32 type`

       - `1` 事件类型
    - ..

       - `VIDEO_EVENT_SIZE_CHANGED`

       - 尺寸已改变
    - ..

       - `VIDEO_EVENT_FRAME_RATE_CHANGED`

       - 帧率已改变
    - ..

       - `VIDEO_EVENT_DECODER_STOPPED`

       - 解码器已停止
    - ..

       - `VIDEO_EVENT_VSYNC`

       - 发生Vsync
    - ..

       - `long timestamp`

       - `1` 发生时的 MPEG PTS
    - ..

       - `2` `union u`

       - `video_size_t`_ size

       - 视频的分辨率和宽高比
    - ..

       - `unsigned int frame_rate`

       - 单位为每 1000 秒的帧数

    - ..

       - `unsigned char vsync_field`

       - | unknown / odd / even / progressive
          | 参见：`Predefined decoder commands and flags`_

#### 描述（Description

这是 `VIDEO_GET_EVENT`_ 调用返回视频事件的结构。更多细节请参见那里

-----


### video_status


#### 概要（Synopsis

`VIDEO_GET_STATUS`_ 调用返回以下结构体，告知
播放操作的各种状态

    struct video_status {
	int                    video_blank;
	video_play_state_t     play_state;
	video_stream_source_t  stream_source;
	video_format_t         video_format;
	video_displayformat_t  display_format;
    };

#### 变量（Variables

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `2` `int video_blank`

       - `1` 冻结时是否显示空白视频？

    - ..

       - TRUE  ( != 0 )

       - 冻结时黑屏
    - ..

       - FALSE ( == 0 )

       - 显示最后解码的帧
    - ..

       - `video_play_state_t`_ `play_state`

       - 当前的播放状态
    - ..

       - `video_stream_source_t`_ `stream_source`

       - 当前源（demux/memory）
    - ..

       - `video_format_t`_ `video_format`

       - 流的当前宽高比
    - ..

       - `video_displayformat_t`_ `display_format`

       - 应用的裁剪模式
#### 描述（Description

如果 `video_blank` 被设置为 `TRUE`，则在切换频道或停止播放时视频将清空。否则，将显示最后一幅画面。`play_state` 指示视频当前是冻结、停止还正在播放。`stream_source` 对应于为视频流选择的源。它可以来自
解复用器或来自内存。`video_format` 指示当前播放的视频流的宽高比
:3 16:9 之一）。最后，`display_format` 在源视频格式与输设备的格式不同时，对应于所应用的裁剪模式

-----


### video_still_picture


#### 概要（Synopsis

    struct video_still_picture {
    char *iFrame;
    int32_t size;
    };

#### 变量（Variables

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `char *iFrame`

       - 指向内存中单I 帧的指针
    - ..

       - `int32_t size`

       - I 帧的大小

#### 描述（Description

通过 `VIDEO_STILLPICTURE`_ 调用显示I 帧在此结构体中被传入

-----


### video capabilities


#### 概要（Synopsis

    #define VIDEO_CAP_MPEG1   1
    #define VIDEO_CAP_MPEG2   2
    #define VIDEO_CAP_SYS     4
    #define VIDEO_CAP_PROG    8

#### 常量（Constants

能力位的定义
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `VIDEO_CAP_MPEG1`

       - `1` 硬件可以解码 MPEG1
    - ..

       - `VIDEO_CAP_MPEG2`

       - 硬件可以解码 MPEG2
    - ..

       - `VIDEO_CAP_SYS`

       - 视频设备接受系统流（system stream）
          你仍然必须打开视频和音频设备，
          但只将流发送到视频设备
    - ..

       - `VIDEO_CAP_PROG`

       - 视频设备接受节目流（program stream）
          你仍然必须打开视频和音频设备，
          但只将流发送到视频设备
#### 描述（Description

`VIDEO_GET_CAPABILITIES`_ 的调用返回一个无符号整数，其根据
硬件的能力设置了以下位

-----


## 视频函数调用（Video Function Calls

### VIDEO_STOP


#### 概要（Synopsis


	int ioctl(fd, VIDEO_STOP, int mode)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - `1` 对此命令等于 `VIDEO_STOP`
    - ..

       - `2` `int mode`

       - `1` 指示应如何处理屏幕
    - ..

       - TRUE  ( != 0 )

       - 停止时黑屏
    - ..

       - FALSE ( == 0 )

       - 显示最后解码的帧
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用于数字电视（Digital TV）设备。要控制 V4L2 解码器，请改V4L2 VIDIOC_DECODER_CMD
ioctl 调用要求视频设备停止播放当前流。根据输入参数，屏幕可以被清空或
显示最后解码的帧
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_PLAY


#### 概要（Synopsis


	int ioctl(fd, VIDEO_PLAY)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_PLAY`
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用于数字电视设备。要控制 V4L2 解码器，请改V4L2 VIDIOC_DECODER_CMD
ioctl 调用要求视频设备开始从所选源播放视频流
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_FREEZE


#### 概要（Synopsis


	int ioctl(fd, VIDEO_FREEZE)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_FREEZE`
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用于数字电视设备。要控制 V4L2 解码器，请改V4L2 VIDIOC_DECODER_CMD
如果选择VIDEO_SOURCE_DEMUX，此 ioctl 调用会挂起正在播放的实时视频流解码和播放被冻结。之后可以使`VIDEO_CONTINUE`_ 命令重启视频流的
解码和播放过程如果ioctl 调用 `VIDEO_SELECT_SOURCE`_ 中选择VIDEO_SOURCE_MEMORY则在执行 `VIDEO_CONTINUE`_ `VIDEO_PLAY`_ ioctl 调用之前，数字电视子系统
将不会解码任何更多数据
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_CONTINUE


#### 概要（Synopsis


	int ioctl(fd, VIDEO_CONTINUE)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_CONTINUE`
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用于数字电视设备。要控制 V4L2 解码器，请改V4L2 VIDIOC_DECODER_CMD
ioctl 调用重启在调`VIDEO_FREEZE`_ 之前播放的视频流的解码和播放过程
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_SELECT_SOURCE


#### 概要（Synopsis


	int ioctl(fd, VIDEO_SELECT_SOURCE, video_stream_source_t source)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_SELECT_SOURCE`
    - ..

       - `video_stream_source_t`_ `source`

       - 指示视频流应使用哪个源
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用于数字电视设备。此 ioctl 也曾V4L2 ivtv 驱动支持，但已被
ivtv 特有`IVTV_IOC_PASSTHROUGH_MODE` ioctl 取代
ioctl 调用告知视频设备输入数据应使用哪个源。可能的源是 demux memory如果选择 memory，则数据通过 write 命令使用结构`video_stream_source_t`_
馈送给视频设备。如果选择 demux，则数据直接从板载解复用设备传输到解码器
馈送给解码器的数据也由 PID 过滤器控制。输出选择：`dmx_output`
`DMX_OUT_DECODER`銆。

#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_SET_BLANK


#### 概要（Synopsis


	int ioctl(fd, VIDEO_SET_BLANK, int mode)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - `1` 对此命令等于 `VIDEO_SET_BLANK`
    - ..

       - `2` `int mode`

       - `1` 指示屏幕是否应被清空
    - ..

       - TRUE  ( != 0 )

       - 停止时黑屏
    - ..

       - FALSE ( == 0 )

       - 显示最后解码的帧
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备清空画面
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_GET_STATUS


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_GET_STATUS,
	struct video_status *status)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_STATUS`
    - ..

       - `struct` `video_status`_ `*status`

       - 返回视频设备的当前状态
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备返回设备的当前状态
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_GET_EVENT


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_GET_EVENT,
	struct video_event *ev)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_EVENT`
    - ..

       - `struct` `video_event`_ `*ev`

       - 指向若存在则事件要存储的位置
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 仅用DVB 设备。要V4L2 解码器获取事件，请改V4L2 VIDIOC_DQEVENT ioctl
ioctl 调用在可用时返回 `video_event`_ 类型的事件。一定数量的
最新事件将被排队并按发生顺序返回。如果不及时获取，较旧的事件可能会被丢弃。如没有可用事件，行为取决于设备处于阻塞还是非阻塞模式。在后者情况下，调用会立即
失败，errno 被设置为 `EWOULDBLOCK`。在前者情况下，调用会阻塞直到有事件可用标准Linux poll() select() 系统调用可以与设备文件描述符一起使以监视新事件。对select()，文件描述符应包含在 exceptfds 参数中，对于 poll()应指POLLPRI 作为唤醒条件。此 ioctl 调用只需读权限即可
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EWOULDBLOCK`

       - `1` 没有待处理事件，且设备处          非阻塞模式
    - ..

       - `EOVERFLOW`

       - 事件队列溢出——丢失了一个或多个事件

-----


### VIDEO_SET_DISPLAY_FORMAT


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_SET_DISPLAY_FORMAT,
	video_display_format_t format)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_SET_DISPLAY_FORMAT`
    - ..

       - `video_displayformat_t`_ `format`

       - 选择要使用的视频格式
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备选择要由 MPEG 芯片应用于视频的
视频格式
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_STILLPICTURE


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_STILLPICTURE,
	struct video_still_picture *sp)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_STILLPICTURE`
    - ..

       - `struct` `video_still_picture`_ `*sp`

       - 指向存储带有 I 帧和大小的结构体的位置的指针
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备显示静止画面（I 帧）。输入数据应是包I 帧的
基本视频流的一部分。通常此部分是TS PES 录制中提取的。设备必须支分辨率和编解码器（参`video capabilities`_）。如果指针为 NULL，则当前显示静止画面将被清空
例如，AV7110 支持具有常用 PAL-SD 分辨率的 MPEG1 MPEG2
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_FAST_FORWARD


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_FAST_FORWARD, int nFrames)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_FAST_FORWARD`
    - ..

       - `int nFrames`

       - 要跳过的帧数
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备跳过N I 帧的解码。此调用只能在选择`VIDEO_SOURCE_MEMORY` 时使用
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - 未选择 `VIDEO_SOURCE_MEMORY` 模式

-----


### VIDEO_SLOWMOTION


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_SLOWMOTION, int nFrames)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_SLOWMOTION`
    - ..

       - `int nFrames`

       - 每帧重复的次数
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用要求视频设备将每帧解码重N 次。此调用只能在选择`VIDEO_SOURCE_MEMORY` 时使用
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - 未选择 `VIDEO_SOURCE_MEMORY` 模式

-----


### VIDEO_GET_CAPABILITIES


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_GET_CAPABILITIES, unsigned int *cap)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_CAPABILITIES`
    - ..

       - `unsigned int *cap`

       - 指向存储能力信息的位置的指针
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用询问视频设备的解码能力。成功时它返回一个整数，其根`video capabilities`_ 中的定义设置了相应的位
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_CLEAR_BUFFER


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_CLEAR_BUFFER)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_CLEAR_BUFFER`
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 调用清除驱动和解码器硬件中的所有视频缓冲区
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_SET_STREAMTYPE


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_SET_STREAMTYPE, int type)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_SET_STREAMTYPE`
    - ..

       - `int type`

       - 流类型
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 告知驱动期望写入其中的流类型是什么智能解码器也可能不支持或忽略（如 AV7110）此调用，而自行确定流类型
当前使用的流类型
    :header-rows:  1
    :stub-columns: 0

    - ..

       - Codec

       - Stream type

    - ..

       - MPEG2

       - 0

    - ..

       - MPEG4 h.264

       - 1

    - ..

       - VC1

       - 3

    - ..

       - MPEG4 Part2

       - 4

    - ..

       - VC1 SM

       - 5

    - ..

       - MPEG1

       - 6

    - ..

       - HEVC h.265

       - | 7
          | DREAMBOX: 22

    - ..

       - AVS

       - 16

    - ..

       - AVS2

       - 40

并非每个解码器都支持所有流类型
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_SET_FORMAT


#### 概要（Synopsis


	int ioctl(fd, int request = VIDEO_SET_FORMAT, video_format_t format)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_SET_FORMAT`
    - ..

       - `video_format_t`_ `format`

       - TV 的视频格式，`video_format_t`_ 节所定义
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 设置所连接输出设备（TV）的屏幕格式（宽高比），以便相应调整
解码器的输出
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_GET_SIZE


#### 概要（Synopsis


	int ioctl(int fd, int request = VIDEO_GET_SIZE, video_size_t *size)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的文件描述符，

    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_SIZE`
    - ..

       - `video_size_t`_ `*size`

       - 返回尺寸和宽高比
#### 描述（Description

             See: legacy_dvb_decoder_notes

ioctl 返回尺寸和宽高比
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_GET_PTS


#### 概要（Synopsis


	int ioctl(int fd, int request = VIDEO_GET_PTS, __u64 *pts)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_PTS`
    - ..

       - `__u64 *pts`

       - 返回 ITU T-REC-H.222.0 /
          ISO/IEC 13818-1 定义33 位时间戳
          如果可能，PTS 应属于当前播放的帧，但也可能是一个接近它的值，
          例如最后解码帧PTS PES 解析器提取的最后一PTS
#### 描述（Description

             See: legacy_dvb_decoder_notes

对于 V4L2 解码器，ioctl 已被 `V4L2_CID_MPEG_VIDEO_DEC_PTS` 控制取代
ioctl 调用要求视频设备返回当前PTS 时间戳
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_GET_FRAME_COUNT


#### 概要（Synopsis


	int ioctl(int fd, VIDEO_GET_FRAME_COUNT, __u64 *pts)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_GET_FRAME_COUNT`
    - ..

       - `__u64 *pts`

       - 返回自解码器启动以来显示的帧数
#### 描述（Description

             See: legacy_dvb_decoder_notes

对于 V4L2 解码器，ioctl 已被 `V4L2_CID_MPEG_VIDEO_DEC_FRAME` 控制取代
ioctl 调用要求视频设备返回自解码器启动以来显示的帧数
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_COMMAND


#### 概要（Synopsis


	int ioctl(int fd, int request = VIDEO_COMMAND,
	struct video_command *cmd)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_COMMAND`
    - ..

       - `struct video_command`_ `*cmd`

       - 命令解码器
#### 描述（Description

             See: legacy_dvb_decoder_notes

对于 V4L2 解码器，ioctl 已被 VIDIOC_DECODER_CMD ioctl 取代
ioctl 命令解码器。`struct video_command`_ `v4l2_decoder_cmd`
结构体的一个子集，因此请参VIDIOC_DECODER_CMD 文档以获更多信息
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### VIDEO_TRY_COMMAND


#### 概要（Synopsis


	int ioctl(int fd, int request = VIDEO_TRY_COMMAND,
	struct video_command *cmd)

#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `int request`

       - 对此命令等于 `VIDEO_TRY_COMMAND`
    - ..

       - `struct video_command`_ `*cmd`

       - 尝试一个解码器命令
#### 描述（Description

             See: legacy_dvb_decoder_notes

对于 V4L2 解码器，ioctl 已被 VIDIOC_TRY_DECODER_CMD <VIDIOC_DECODER_CMD> ioctl 取代
ioctl 尝试一个解码器命令。`struct video_command`_ `v4l2_decoder_cmd`
结构体的一个子集，因此请参VIDIOC_TRY_DECODER_CMD <VIDIOC_DECODER_CMD> 文档
以获取更多信息
#### 返回值（Return Value

成功时返0，出错时返回 -1 并且 `errno` 变量会被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

-----


### open()


#### 概要（Synopsis


    #include <fcntl.h>


#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 特定视频设备的名称
    - ..

       - `3` `int flags`

       - `1` 以下标志的按位或
    - ..

       - `O_RDONLY`

       - 只读访问

    - ..

       - `O_RDWR`

       - 读写访问

    - ..

       - `O_NONBLOCK`
       - | 以非阻塞模式打开
          | （默认是阻塞模式
#### 描述（Description

此系统调用打开一个具名的视频设备（例/dev/dvb/adapter/video）以供后续使用
open() 调用成功后，设备将准备就绪可供使用。阻塞或非阻塞模式的意义存在差异的函数文档中描述。它不影open() 调用本身的语义。以
阻塞模式打开的设备之后可以使fcntl 系统调用F_SETFL 命令切换到非阻塞模式
（反之亦然）。这是一个标准的系统调用，在 Linux fcntl 手册页中有文档只有一个用户可以以 O_RDWR 模式打开视频设备。所有其他以该模式打开设备的尝都将失败，并返回错误码。如果以 O_RDONLY 模式打开视频设备，则唯一可以使用ioctl 调用`VIDEO_GET_STATUS`_。所有其他调用都将返回错误码
#### 返回值（Return Value

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - `1` 设备驱动未加不可用
    - ..

       - `EINTERNAL`

       - 内部错误
    - ..

       - `EBUSY`

       - 设备或资源忙
    - ..

       - `EINVAL`

       - 无效参数

-----


### close()


#### 概要（Synopsis


#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
#### 描述（Description

此系统调用关闭先前打开的视频设备
#### 返回值（Return Value

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - fd 不是有效的打开文件描述符

-----


### write()


#### 概要（Synopsis


#### 参数（Arguments

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 先前`open()`_ 的调用返回的
          文件描述符
    - ..

       - `void *buf`

       - 指向包含 PES 数据的缓冲区的指针
    - ..

       - `size_t count`

       - buf 的大小
#### 描述（Description

此系统调用只能在 ioctl 调用 `VIDEO_SELECT_SOURCE`_ 中选择VIDEO_SOURCE_MEMORY 使用。所提供的数据应PES 格式，除非能力允许其他格式。TS 是存DVB 数据
最常见的格式，通常也受支持。如果未指定 O_NONBLOCK，该函数将阻塞直到有缓冲区空可用。要传输的数据量count 隐式确定

#### 返回值（Return Value

    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - `1` 未选择 `VIDEO_SOURCE_MEMORY` 模式
    - ..

       - `ENOMEM`

       - 试图写入的数据超过内部缓冲区可容纳的量
    - ..

       - `EBADF`

       - fd 不是有效的打开文件描述符