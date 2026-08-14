

## 用于表示媒体图元素的类型与标志




    :header-rows:  0
    :stub-columns: 0

    - -  `MEDIA_ENT_F_UNKNOWN` and
	  `MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN`
       - 未知实体。通常表示驱动没有正确初始化该实体，这是内核的一个 bug

    - -  `MEDIA_ENT_F_IO_V4L`
       - 数据流输入和/或输出实体。

    - -  `MEDIA_ENT_F_IO_VBI`
       - V4L VBI 流输入或输出实体

    - -  `MEDIA_ENT_F_IO_SWRADIO`
       - V4L 软件数字无线电（SDR）流输入或输出实体

    - -  `MEDIA_ENT_F_IO_DTV`
       - DVB 数字电视流输入或输出实体

    - -  `MEDIA_ENT_F_DTV_DEMOD`
       - 数字电视解调器实体。

    - -  `MEDIA_ENT_F_TS_DEMUX`
       - MPEG 传输流解复用实体。可由硬件实现，或由 Linux DVB 子系统在内核态实现。

    - -  `MEDIA_ENT_F_DTV_CA`
       - 数字电视条件接收模块（CAM）实体

    - -  `MEDIA_ENT_F_DTV_NET_DECAP`
       - 数字电视网络 ULE/MLE 去封装实体。可由硬件实现，或在内核态实现

    - -  `MEDIA_ENT_F_CONN_RF`
       - 射频（RF）信号连接器。

    - -  `MEDIA_ENT_F_CONN_SVIDEO`
       - S-Video 信号连接器。

    - -  `MEDIA_ENT_F_CONN_COMPOSITE`
       - RGB 复合信号连接器。

    - -  `MEDIA_ENT_F_CAM_SENSOR`
       - 摄像头视频传感器实体。

    - -  `MEDIA_ENT_F_FLASH`
       - 闪光灯控制器实体。

    - -  `MEDIA_ENT_F_LENS`
       - 镜头控制器实体。

    - -  `MEDIA_ENT_F_ATV_DECODER`
       - 模拟视频解码器，其基本功能是接收来自广播、DVD 播放器、摄像头和录像机等各种来源的模拟视频，格式可为 NTSC、PAL、SECAM 或 HD，将流分离为其组成部分（亮度和色度），并以某种数字视频标准配合适当的定时信号输出。

    - -  `MEDIA_ENT_F_TUNER`
       - 数字电视、模拟电视、收音机和/或软件无线电调谐器，包含一个 PLL 调谐级，将射频（RF）信号转换为中频（IF）。现代调谐器内部带有用于音频和视频的 IF-PLL 解码器，但较旧的型号将这些级实现在独立的实体中。

    - -  `MEDIA_ENT_F_IF_VID_DECODER`
       - IF-PLL 视频解码器。它接收来自 PLL 的 IF 并解码模拟电视视频信号。这在一些非常老的模拟调谐器上很常见，例如飞利浦 MK3 设计。它们都包含 tda9887（或某些软件兼容的类似芯片，如 tda9885）。这些设备使用的 I2C 地址与调谐器 PLL 不同。

    - -  `MEDIA_ENT_F_IF_AUD_DECODER`
       - IF-PLL 声音解码器。它接收来自 PLL 的 IF 并解码模拟电视音频信号。这在一些非常老的模拟硬件上很常见，例如 Micronas msp3400、飞利浦 tda9840、tda985x 等。这些设备使用的 I2C 地址与调谐器 PLL 不同，应和 IF-PLL 视频解码器一起控制。

    - -  `MEDIA_ENT_F_AUDIO_CAPTURE`
       - 音频采集功能实体。

    - -  `MEDIA_ENT_F_AUDIO_PLAYBACK`
       - 音频回放功能实体。

    - -  `MEDIA_ENT_F_AUDIO_MIXER`
       - 音频混音功能实体。

    - -  `MEDIA_ENT_F_PROC_VIDEO_COMPOSER`
       - 视频合成器（blender）。能够合成视频的实体必须至少有两个 sink 衬垫和一个 source 衬垫，并将输入视频帧合成到输出视频帧上。合成可通过 alpha 混合、色键、光栅操作（ROP）、拼接或任何其他方式完成。

    - -  `MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER`
       - 视频像素格式化器。能够格式化像素的实体必须至少有一个 sink 衬垫和一个 source 衬垫。读像素格式化器从内存读取像素，并执行解包、裁剪、色键、alpha 乘法和像素编码转换等部分操作。写像素格式化器执行抖动、像素编码转换和打包等部分操作，并将像素写入内存。

    - -  `MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV`
       - 视频像素编码转换器。能够转换像素编码的实体必须至少有一个 sink 衬垫和一个 source 衬垫，并将在其 sink 衬垫上接收到的像素编码转换为在其 source 衬垫上输出的不同编码。像素编码转换包括但不限于 RGB 与 HSV 互转、RGB 与 YUV 互转，以及 CFA（Bayer）到 RGB 的转换。

    - -  `MEDIA_ENT_F_PROC_VIDEO_LUT`
       - 视频查找表。能够进行视频查找表处理的实体必须有一个 sink 衬垫和一个 source 衬垫。它使用在其 sink 衬垫上接收到的像素值，在内部表中查找条目并在其 source 衬垫上输出。查找处理可以分别对所有分量进行，也可以将它们组合起来进行多维表查找。

    - -  `MEDIA_ENT_F_PROC_VIDEO_SCALER`
       - 视频缩放器。能够进行视频缩放的实体必须至少有一个 sink 衬垫和一个 source 衬垫，并将在其 sink 衬垫上接收到的视频帧缩放到其 source 衬垫上不同分辨率的输出。支持的缩放比例范围是实体特定的，水平与垂直方向可能不同（特别地，可能仅支持单向缩放）。合并（binning）和下采样（有时也称为 skipping）也被视为缩放。

    - -  `MEDIA_ENT_F_PROC_VIDEO_STATISTICS`
       - 视频统计计算（直方图、3A 等）。能够进行统计计算的实体必须有一个 sink 衬垫和一个 source 衬垫。它对在其 sink 衬垫上接收到的帧计算统计量，并在其 source 衬垫上输出统计数据。

    - -  `MEDIA_ENT_F_PROC_VIDEO_ENCODER`
       - 视频（MPEG、HEVC、VPx 等）编码器。能够压缩视频帧的实体。必须有一个 sink 衬垫和至少一个 source 衬垫。

    - -  `MEDIA_ENT_F_PROC_VIDEO_DECODER`
       - 视频（MPEG、HEVC、VPx 等）解码器。能够将压缩视频流解压缩为未压缩视频帧的实体。必须有一个 sink 衬垫和至少一个 source 衬垫。

    - -  `MEDIA_ENT_F_PROC_VIDEO_ISP`
       - 图像信号处理器（ISP）设备。ISP 通常是独一无二的设备，具有特定的控制接口，组合使用自定义的 V4L2 控制与 IOCTL，以及通过元数据缓冲区提供的参数。

    - -  `MEDIA_ENT_F_VID_MUX`
       - 视频多路复用器。能够进行多路复用的实体必须至少有两个 sink 衬垫和一个 source 衬垫，并且必须将从活动 sink 衬垫接收到的视频帧传送到 source 衬垫。

    - -  `MEDIA_ENT_F_VID_IF_BRIDGE`
       - 视频接口桥。视频接口桥实体必须至少有一个 sink 衬垫和至少一个 source 衬垫。它在其 sink 衬垫上从一种类型的输入视频总线（HDMI、eDP、MIPI CSI-2 等）接收视频帧，并在其 source 衬垫上将其输出到另一种类型的输出视频总线（eDP、MIPI CSI-2、并行等）。

    - -  `MEDIA_ENT_F_DV_DECODER`
       - 数字视频解码器。视频解码器的基本功能是接收来自各种来源的数字视频，并以某种数字视频标准配合适当的定时信号输出。

    - -  `MEDIA_ENT_F_DV_ENCODER`
       - 数字视频编码器。视频编码器的基本功能是接收带有适当定时信号（通常是一条带同步信号的并行视频总线）的某种数字视频标准的数字视频，并将其输出到 HDMI 或 DisplayPort 等数字视频输出连接器。



    :header-rows:  0
    :stub-columns: 0

    - -  `MEDIA_ENT_FL_DEFAULT`
       - 该类型默认实体。用于发现默认的音频、VBI 和视频设备，以及默认的摄像头传感器等。

    - -  `MEDIA_ENT_FL_CONNECTOR`
       - 该实体表示一个连接器。



    :header-rows:  0
    :stub-columns: 0

    - -  `MEDIA_INTF_T_DVB_FE`
       - 数字电视前端的设备节点接口
       - 通常为 /dev/dvb/adapter?/frontend?

    - -  `MEDIA_INTF_T_DVB_DEMUX`
       - 数字电视解复用器的设备节点接口
       - 通常为 /dev/dvb/adapter?/demux?

    - -  `MEDIA_INTF_T_DVB_DVR`
       - 数字电视 DVR 的设备节点接口
       - 通常为 /dev/dvb/adapter?/dvr?

    - -  `MEDIA_INTF_T_DVB_CA`
       - 数字电视条件接收的设备节点接口
       - 通常为 /dev/dvb/adapter?/ca?

    - -  `MEDIA_INTF_T_DVB_NET`
       - 数字电视网络控制的设备节点接口
       - 通常为 /dev/dvb/adapter?/net?

    - -  `MEDIA_INTF_T_V4L_VIDEO`
       - 视频（V4L）的设备节点接口
       - 通常为 /dev/video?

    - -  `MEDIA_INTF_T_V4L_VBI`
       - VBI（V4L）的设备节点接口
       - 通常为 /dev/vbi?

    - -  `MEDIA_INTF_T_V4L_RADIO`
       - 收音机（V4L）的设备节点接口
       - 通常为 /dev/radio?

    - -  `MEDIA_INTF_T_V4L_SUBDEV`
       - V4L 子设备的设备节点接口
       - 通常为 /dev/v4l-subdev?

    - -  `MEDIA_INTF_T_V4L_SWRADIO`
       - 软件定义无线电（V4L）的设备节点接口
       - 通常为 /dev/swradio?

    - -  `MEDIA_INTF_T_V4L_TOUCH`
       - 触摸设备（V4L）的设备节点接口
       - 通常为 /dev/v4l-touch?

    - -  `MEDIA_INTF_T_ALSA_PCM_CAPTURE`
       - ALSA PCM 采集的设备节点接口
       - 通常为 /dev/snd/pcmC?D?c

    - -  `MEDIA_INTF_T_ALSA_PCM_PLAYBACK`
       - ALSA PCM 回放的设备节点接口
       - 通常为 /dev/snd/pcmC?D?p

    - -  `MEDIA_INTF_T_ALSA_CONTROL`
       - ALSA 控制的设备节点接口
       - 通常为 /dev/snd/controlC?

    - -  `MEDIA_INTF_T_ALSA_COMPRESS`
       - ALSA 压缩的设备节点接口
       - 通常为 /dev/snd/compr?

    - -  `MEDIA_INTF_T_ALSA_RAWMIDI`
       - ALSA 原始 MIDI 的设备节点接口
       - 通常为 /dev/snd/midi?

    - -  `MEDIA_INTF_T_ALSA_HWDEP`
       - ALSA 硬件相关的设备节点接口
       - 通常为 /dev/snd/hwC?D?

    - -  `MEDIA_INTF_T_ALSA_SEQUENCER`
       - ALSA 音序器的设备节点接口
       - 通常为 /dev/snd/seq

    - -  `MEDIA_INTF_T_ALSA_TIMER`
       - ALSA 定时器的设备节点接口
       - 通常为 /dev/snd/timer



    :header-rows:  0
    :stub-columns: 0

    - -  `MEDIA_PAD_FL_SINK`
       - 输入衬垫，相对于实体而言。输入衬垫接收数据，是连接的终点。

    - -  `MEDIA_PAD_FL_SOURCE`
       - 输出衬垫，相对于实体而言。输出衬垫提供数据，是连接的起点。

    - -  `MEDIA_PAD_FL_MUST_CONNECT`
       - 如果设置了此标志，则该衬垫要能够流化，必须至少被一条已启用的连接连接。即使未设置此标志，衬垫也可能需要已启用的连接（例如取决于设备配置），存在临时原因；缺少该标志并不意味着没有此类需要。


每个衬垫都必须且只能设置 `MEDIA_PAD_FL_SINK` 和 `MEDIA_PAD_FL_SOURCE` 其中之一。



    :header-rows:  0
    :stub-columns: 0

    - -  `MEDIA_LNK_FL_ENABLED`
       - 该连接已启用，可用于传输媒体数据。当两个或多个连接指向同一个 sink 衬垫时，一次只能启用其中一个。

    - -  `MEDIA_LNK_FL_IMMUTABLE`
       - 连接的启用状态不能在运行时修改。不可变连接始终处于启用状态。

    - -  `MEDIA_LNK_FL_DYNAMIC`
       - 连接的启用状态可以在流化过程中修改。此标志由驱动设置，对应用程序是只读的。

    - -  `MEDIA_LNK_FL_LINK_TYPE`
       - 这是一个位掩码，定义连接的类型。目前支持以下连接类型：

	  .. _MEDIA-LNK-FL-DATA-LINK:

	  `MEDIA_LNK_FL_DATA_LINK` 表示两个衬垫之间数据连接的连接。

	  .. _MEDIA-LNK-FL-INTERFACE-LINK:

	  `MEDIA_LNK_FL_INTERFACE_LINK` 表示将实体与其接口关联起来的连接。

	  .. _MEDIA-LNK-FL-ANCILLARY-LINK:

	  `MEDIA_LNK_FL_ANCILLARY_LINK` 表示两个实体之间物理关系的连接。该连接可能是也可能不是不可变的，因此应用程序不得假定其中任意一种情况。
