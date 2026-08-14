
## 传统 DVB MPEG 解码器 API


## 一般说明


该 API 最初仅针对 DVB 设计，因此仅限于此类数字电视广播系统中使用的 legacy_dvb_decoder_formats。

为了规避此限制，设计了更加通用的 V4L2 <v4l2spec> API，它替代了 DVB API 的这一部分。

尽管如此，仍有一些项目基于该 API 构建。为了确保兼容性，该 API 保持原样。


    音频和视频请使用 V4L2 <v4l2spec> 和 ALSA API。

    流水线应使用媒体控制器 API<media_controller> 建立。

实际上，解码器似乎被区别对待。应用程序通常知道正在使用哪个解码器，或者它是专门为某一种解码器类型编写的。由于能力已经已知，很少会去查询能力。


## 数据格式


该 API 是为 DVB 及兼容的广播系统设计的。因此，唯一支持的数据格式是兼容 ISO/IEC 13818-1 的 MPEG 流。支持的负载可能因所用解码器而异。

除非另有说明，时间戳始终是 ITU T-REC-H.222.0 / ISO/IEC 13818-1 中定义的 MPEG PTS。

存储录制内容时通常使用 TS 流，使用 PES 的情况较少。这两种变体在播放时通常被接受，但这可能取决于驱动。


## 目录


- [legacy_dvb_video](legacy_dvb_video)
- [legacy_dvb_audio](legacy_dvb_audio)
- [legacy_dvb_osd](legacy_dvb_osd)
