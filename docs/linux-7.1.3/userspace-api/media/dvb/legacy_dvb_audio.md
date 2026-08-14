


## DVB 音频设备


             See: legacy_dvb_decoder_notes

DVB 音频设备控制着 DVB 硬件的 MPEG2 音频解码器。可以通过
`/dev/dvb/adapter?/audio?` 访问它。数据类型与 ioctl 定义可以通过在
应用程序中包含 `linux/dvb/audio.h` 来使用。

请注意，大多数 DVB 卡没有自己的 MPEG 解码器，因此会省略音频和视频
设备。

这些 ioctl 也曾被 V4L2 用来控制 V4L2 中实现的 MPEG 解码器。将这类
ioctl 用于该目的的做法已被废弃，并已创建相应的 V4L2 ioctl 或控件来
取代该功能。新的驱动程序请使用 V4L2 ioctls<audio>！


## 音频数据类型


本节描述与音频设备交互时所使用的结构体、数据类型与宏定义。


-----



### audio_stream_source_t


#### 概述


    typedef enum {
    AUDIO_SOURCE_DEMUX,
    AUDIO_SOURCE_MEMORY
    } audio_stream_source_t;

#### 常量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_SOURCE_DEMUX`

       - `1` 选择解复用器（由前端或 DVR 设备提供数据）作为视频流的来源。

    - ..

       - `AUDIO_SOURCE_MEMORY`

       - 选择通过 `write()`_ 系统调用来自应用程序的流。

#### 描述


音频流来源通过 `AUDIO_SELECT_SOURCE`_ 调用设置，可取值如下，取决于
我们是回放内部（demux）还是外部（用户写入）来源。

送入解码器的数据还受 PID 过滤器控制。输出选择：`dmx_output`
`DMX_OUT_DECODER`。


-----



### audio_play_state_t


#### 概述


    typedef enum {
	AUDIO_STOPPED,
	AUDIO_PLAYING,
	AUDIO_PAUSED
    } audio_play_state_t;

#### 常量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_STOPPED`

       - 音频已停止。

    - ..

       - `AUDIO_PLAYING`

       - 音频正在播放。

    - ..

       - `AUDIO_PAUSE`

       - 音频已冻结。

#### 描述


此值可由 `AUDIO_GET_STATUS`_ 调用返回，表示音频播放的状态。


-----



### audio_channel_select_t


#### 概述


    typedef enum {
	AUDIO_STEREO,
	AUDIO_MONO_LEFT,
	AUDIO_MONO_RIGHT,
	AUDIO_MONO,
	AUDIO_STEREO_SWAPPED
    } audio_channel_select_t;

#### 常量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_STEREO`

       - 立体声。

    - ..

       - `AUDIO_MONO_LEFT`

       - 单声道，选择左声道作为来源。

    - ..

       - `AUDIO_MONO_RIGHT`

       - 单声道，选择右声道作为来源。

    - ..

       - `AUDIO_MONO`

       - 仅单声道来源。

    - ..

       - `AUDIO_STEREO_SWAPPED`

       - 立体声，交换左（L）与右（R）。

#### 描述


通过 `AUDIO_CHANNEL_SELECT`_ 选择的音频声道由此值决定。


-----



### audio_mixer_t


#### 概述


    typedef struct audio_mixer {
	unsigned int volume_left;
	unsigned int volume_right;
    } audio_mixer_t;

#### 变量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `unsigned int volume_left`

       - 左声道音量。
          有效范围：0 ... 255

    - ..

       - `unsigned int volume_right`

       - 右声道音量。
          有效范围：0 ... 255

#### 描述


此结构体由 `AUDIO_SET_MIXER`_ 调用用来设置音频音量。


-----



### audio_status


#### 概述


    typedef struct audio_status {
	int AV_sync_state;
	int mute_state;
	audio_play_state_t play_state;
	audio_stream_source_t stream_source;
	audio_channel_select_t channel_select;
	int bypass_mode;
	audio_mixer_t mixer_state;
    } audio_status_t;

#### 变量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `2` `int AV_sync_state`

       - `1` 显示 A/V 同步是开启还是关闭。

    - ..

       - TRUE  ( != 0 )

       - A/V 同步开启。

    - ..

       - FALSE ( == 0 )

       - A/V 同步关闭。

    - ..

       - `2` `int mute_state`

       - `1` 指示音频是否静音。

    - ..

       - TRUE  ( != 0 )

       - 静音音频

    - ..

       - FALSE ( == 0 )

       - 取消静音音频

    - ..

       - `audio_play_state_t`_ `play_state`

       - 当前播放状态。

    - ..

       - `audio_stream_source_t`_ `stream_source`

       - 当前的数据来源。

    - ..

       - `2` `int bypass_mode`

       - `1` 当前音频流在 DVB 子系统中的解码是否被启用或禁用。

    - ..

       - TRUE  ( != 0 )

       - 旁路禁用。

    - ..

       - FALSE ( == 0 )

       - 旁路启用。

    - ..

       - `audio_mixer_t`_ `mixer_state`

       - 当前音量设置。

#### 描述


`AUDIO_GET_STATUS`_ 调用返回此结构体，作为播放操作各种状态的信息。


-----



### audio encodings


#### 概述


     #define AUDIO_CAP_DTS    1
     #define AUDIO_CAP_LPCM   2
     #define AUDIO_CAP_MP1    4
     #define AUDIO_CAP_MP2    8
     #define AUDIO_CAP_MP3   16
     #define AUDIO_CAP_AAC   32
     #define AUDIO_CAP_OGG   64
     #define AUDIO_CAP_SDDS 128
     #define AUDIO_CAP_AC3  256

#### 常量


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `AUDIO_CAP_DTS`

       - `1` 硬件接受 DTS 音轨。

    - ..

       - `AUDIO_CAP_LPCM`

       - 硬件接受采用线性脉冲编码调制（LPCM）的非压缩音频。

    - ..

       - `AUDIO_CAP_MP1`

       - 硬件接受 MPEG-1 Audio Layer 1。

    - ..

       - `AUDIO_CAP_MP2`

       - 硬件接受 MPEG-1 Audio Layer 2。
          也称为 MUSICAM。

    - ..

       - `AUDIO_CAP_MP3`

       - 硬件接受 MPEG-1 Audio Layer III。
          通常称为 .mp3。

    - ..

       - `AUDIO_CAP_AAC`

       - 硬件接受 AAC（高级音频编码）。

    - ..

       - `AUDIO_CAP_OGG`

       - 硬件接受 Vorbis 音轨。

    - ..

       - `AUDIO_CAP_SDDS`

       - 硬件接受 Sony Dynamic Digital Sound（SDDS）。

    - ..

       - `AUDIO_CAP_AC3`

       - 硬件接受 Dolby Digital ATSC A/52 音频。
          也称为 AC-3。

#### 描述


对 `AUDIO_GET_CAPABILITIES`_ 的调用返回一个无符号整数，其中根据硬件
能力设置了以下比特位。


-----



## 音频函数调用



### AUDIO_STOP


#### 概述


	 int ioctl(int fd, int request = AUDIO_STOP)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - `1` 对应该命令，等于 `AUDIO_STOP`。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备停止播放当前的流。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_PLAY


#### 概述


	 int  ioctl(int fd, int request = AUDIO_PLAY)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - `1` 对应该命令，等于 `AUDIO_PLAY`。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备开始从所选来源播放音频流。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_PAUSE


#### 概述


	 int  ioctl(int fd, int request = AUDIO_PAUSE)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_PAUSE`。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用暂停正在播放的音频流。解码与播放都被暂停。之后可以使用
`AUDIO_CONTINUE`_ 命令重新开始音频流的解码与播放过程。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_CONTINUE


#### 概述


	 int  ioctl(int fd, int request = AUDIO_CONTINUE)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_CONTINUE`。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 重新启动先前被 `AUDIO_PAUSE`_ 命令暂停的解码与播放过程。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SELECT_SOURCE


#### 概述


	 int ioctl(int fd, int request = AUDIO_SELECT_SOURCE,
	 audio_stream_source_t source)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_SELECT_SOURCE`。

    - ..

       - `audio_stream_source_t`_ `source`

       - 指示应用于音频流的来源。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用告知音频设备输入数据应使用哪个来源。可能的来源是 demux
或 memory。若选择 `AUDIO_SOURCE_MEMORY`，则数据通过 write 命令送入音频
设备。若选择 `AUDIO_SOURCE_DEMUX`，数据则直接从板载解复用设备传输到
解码器。注意：到目前为止这仅支持具有一个解复用器和一个解码器的
DVB 设备。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_MUTE


#### 概述


	 int  ioctl(int fd, int request = AUDIO_SET_MUTE, int state)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - `1` 对应该命令，等于 `AUDIO_SET_MUTE`。

    - ..

       - `2` `int state`

       - `1` 指示音频设备是否应静音。

    - ..

       - TRUE  ( != 0 )

       - 静音音频

    - ..

       - FALSE ( == 0 )

       - 取消静音音频

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 仅适用于 DVB 设备。要控制 V4L2 解码器，请改用 V4L2
VIDIOC_DECODER_CMD，并带上 `V4L2_DEC_CMD_START_MUTE_AUDIO` 标志。

此 ioctl 调用请求音频设备对当前正在播放的流进行静音。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_AV_SYNC


#### 概述


	 int  ioctl(int fd, int request = AUDIO_SET_AV_SYNC, int state)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - `1` 对应该命令，等于 `AUDIO_AV_SYNC`。

    - ..

       - `2` `int state`

       - `1` 告知 DVB 子系统 A/V 同步应开启还是关闭。

    - ..

       - TRUE  ( != 0 )

       - A/V 同步开启。

    - ..

       - FALSE ( == 0 )

       - A/V 同步关闭。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备开启或关闭 A/V 同步。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_BYPASS_MODE


#### 概述


	 int ioctl(int fd, int request = AUDIO_SET_BYPASS_MODE, int mode)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - `1` 对应该命令，等于 `AUDIO_SET_BYPASS_MODE`。

    - ..

       - `2` `int mode`

       - `1` 启用或禁用当前音频流在 DVB 子系统中的解码。

    - ..

       - TRUE  ( != 0 )

       - 禁用旁路

    - ..

       - FALSE ( == 0 )

       - 启用旁路

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备旁路音频解码器，并直接转发流而不进行解码。
当无法被 DVB 系统处理的流需要解码时，应使用此模式。如果硬件支持，
Dolby DigitalTM 流会被 DVB 子系统自动转发。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_CHANNEL_SELECT


#### 概述


	 int ioctl(int fd, int request = AUDIO_CHANNEL_SELECT,
	 audio_channel_select_t)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_CHANNEL_SELECT`。

    - ..

       - `audio_channel_select_t`_ `ch`

       - 选择音频的输出格式（左/右单声道、立体声）。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 仅适用于 DVB 设备。要控制 V4L2 解码器，请改用 V4L2
`V4L2_CID_MPEG_AUDIO_DEC_PLAYBACK` 控件。

此 ioctl 调用在可能的情况下请求音频设备选择所请求的声道。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_GET_STATUS


#### 概述


	 int ioctl(int fd, int request = AUDIO_GET_STATUS,
	 struct audio_status *status)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 AUDIO_GET_STATUS。

    - ..

       - `struct` `audio_status`_ `*status`

       - 返回音频设备的当前状态。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备返回音频设备的当前状态。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_GET_CAPABILITIES


#### 概述


	 int ioctl(int fd, int request = AUDIO_GET_CAPABILITIES,
	 unsigned int *cap)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_GET_CAPABILITIES`。

    - ..

       - `unsigned int *cap`

       - 返回受支持的声音格式的位数组。
          比特位在 `audio encodings`_ 中定义。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备告知我们音频硬件的解码能力。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_CLEAR_BUFFER


#### 概述


	 int  ioctl(int fd, int request = AUDIO_CLEAR_BUFFER)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_CLEAR_BUFFER`。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 调用请求音频设备清空音频解码器设备的所有软件与硬件缓冲区。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_ID


#### 概述


	 int  ioctl(int fd, int request = AUDIO_SET_ID, int id)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_SET_ID`。

    - ..

       - `int id`

       - 音频子流 id。

#### 描述


             See: legacy_dvb_decoder_notes

如果程序流或系统流被发送到视频设备，此 ioctl 选择要被解码的子流。

如果未设置音频流类型，则对于 MPEG 声音，id 必须在 [0xC0,0xDF] 范围内；
对于 AC3，在 [0x80,0x87] 范围内；对于 LPCM，在 [0xA0,0xA7] 范围内。
更多说明请参见 ITU-T H.222.0 | ISO/IEC 13818-1。

如果流类型已通过 `AUDIO_SET_STREAMTYPE`_ 设置，则 id 只表示音频流的
子流 id，且只识别前 5 个比特（& 0x1F）。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_MIXER


#### 概述


	 int ioctl(int fd, int request = AUDIO_SET_MIXER, audio_mixer_t *mix)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_SET_MIXER`。

    - ..

       - `audio_mixer_t *mix`

       - 混音器设置。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 允许你调整音频解码器的混音器设置。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### AUDIO_SET_STREAMTYPE


#### 概述


	 int  ioctl(fd, int request = AUDIO_SET_STREAMTYPE, int type)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_SET_STREAMTYPE`。

    - ..

       - `int type`

       - 流类型。

#### 描述


             See: legacy_dvb_decoder_notes

此 ioctl 告诉驱动程序预期接收哪种音频流。当流提供多种音频子流（如
LPCM 和 AC3）时，这很有用。

使用 ITU-T H.222.0 | ISO/IEC 13818-1 中定义的流类型。


#### 返回值


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EINVAL`

       - 类型不是有效或受支持的流类型。


-----



### AUDIO_BILINGUAL_CHANNEL_SELECT


#### 概述


	 int ioctl(int fd, int request = AUDIO_BILINGUAL_CHANNEL_SELECT,
	 audio_channel_select_t)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `int request`

       - 等于 `AUDIO_BILINGUAL_CHANNEL_SELECT`。

    - ..

       - `audio_channel_select_t ch`

       - 选择音频的输出格式（左/右单声道、立体声）。

#### 描述


             See: legacy_dvb_decoder_notes

对于通过 V4L2 控制的 MPEG 解码器，此 ioctl 已被 V4L2
`V4L2_CID_MPEG_AUDIO_DEC_MULTILINGUAL_PLAYBACK` 控件取代。

此 ioctl 调用在可能的情况下请求音频设备为双语流选择所请求的声道。

#### 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中说明。


-----



### open()


#### 概述


    #include <fcntl.h>


#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `const char *deviceName`

       - 特定音频设备的名称。

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

#### 描述


此系统调用打开一个命名的音频设备（例如 `/dev/dvb/adapter0/audio0`）
以供后续使用。当 open() 调用成功后，设备即可使用。阻塞或非阻塞模式
的意义在存在差异的函数文档中说明。它不影响 open() 调用本身的语义。
以阻塞模式打开的设备之后可以使用 fcntl 系统调用的 F_SETFL 命令切换
到非阻塞模式（反之亦然）。这是一个标准的系统调用，在 Linux 的 fcntl
手册页中有说明。只有一个用户能以 O_RDWR 模式打开音频设备。所有其他
以该模式打开设备的尝试都会失败，并返回错误码。如果以 O_RDONLY 模式
打开音频设备，则唯一可以使用的 ioctl 调用是 `AUDIO_GET_STATUS`_。
所有其他调用都会返回错误码。

#### 返回值


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `ENODEV`

       - 设备驱动未加载/不可用。

    - ..

       - `EBUSY`

       - 设备或资源忙。

    - ..

       - `EINVAL`

       - 无效参数。


-----



### close()


#### 概述



#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

#### 描述


此系统调用关闭先前打开的音频设备。

#### 返回值


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EBADF`

       - 文件描述符不是有效的已打开文件描述符。


-----



### write()


#### 概述


	 size_t write(int fd, const void *buf, size_t count)

#### 参数


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `int fd`

       - `1` 由先前对 `open()`_ 的调用返回的文件描述符。

    - ..

       - `void *buf`

       - 指向包含 PES 数据的缓冲区的指针。

    - ..

       - `size_t count`

       - buf 的大小。

#### 描述


此系统调用只能在 ioctl 调用 `AUDIO_SELECT_SOURCE`_ 中选择了
`AUDIO_SOURCE_MEMORY` 时使用。所提供的数据应为 PES 格式。如果未指定
`O_NONBLOCK`，该函数将阻塞，直到缓冲区空间可用。要传输的数据量由
count 隐含给出。

#### 返回值


    :header-rows:  0
    :stub-columns: 0

    - ..

       - `EPERM`

       - `1` 未选择 `AUDIO_SOURCE_MEMORY` 模式。

    - ..

       - `ENOMEM`

       - 尝试写入的数据超过了内部缓冲区所能容纳的量。

    - ..

       - `EBADF`

       - 文件描述符不是有效的已打开文件描述符。
