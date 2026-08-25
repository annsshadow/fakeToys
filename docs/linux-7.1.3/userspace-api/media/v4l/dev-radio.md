


######## 无线电接


该接口面AM FM（模拟）无线电接收机
发射机

传统上，V4L2 无线电设备通过名为 `/dev/radio` `/dev/radio0` 
`/dev/radio63` 的字符设备特殊文件访问，
其主设备号为 81，次设备号为 64 127


## 查询能力


支持无线电接口的设备会在
`v4l2_capability` 结构体（VIDIOC_QUERYCAP ioctl 返回）的
`capabilities` 字段中设`V4L2_CAP_RADIO`
以及 `V4L2_CAP_TUNER` `V4L2_CAP_MODULATOR` 标志
其他能力标志
组合保留供将来扩展


## 附加功能


无线电设备可支持 controls <control>，且必须支持
tuner 鎴?modulator <tuner> ioctls銆。

它们不支持视频输入或输出、音频输入或输出
视频制式、裁剪与缩放、压缩与
参数，或 overlay ioctls。所有其ioctls I/O 方法
保留供将来扩展


## 编程


无线电设备可能具有若干音频控制（control 中所述）
例如音量控制，也可能有自定义控制

此外，所有无线电设备都有一tuner modulator（在 tuner 中讨论）
其索引号0，用于选择无线
频率，并确定接收/发射的是单声道还FM 立体声节目
驱动会根据所选频率在 AM FM 之间自动切换
VIDIOC_G_TUNER <VIDIOC_G_TUNER> 鎴。
VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> ioctl 报告
支持的频率范围
