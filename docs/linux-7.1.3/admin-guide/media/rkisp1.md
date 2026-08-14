
## Rockchip 图像信号处理器 (rkisp1)


## 简介


本文件记录了 Rockchip ISP1 驱动，该驱动是 RK3288 和 RK3399 SoC 的一部分。
驱动位于 drivers/media/platform/rockchip/rkisp1，使用 Media-Controller API。

## 版本


该 ISP 存在多个在后续 SoC 中引入的较小版本。各版本可在 UAPI 中的枚举
`rkisp1_cif_isp_version` 中找到，而运行中的 SoC 内部该 ISP 的版本可通过
ioctl MEDIA_IOC_DEVICE_INFO 返回的 struct media_device_info 的 hw_revision
字段读取。

在用的版本有：

- RKISP1_V10：至少用于 rk3288 和 rk3399
- RKISP1_V11：在原始厂商代码中声明，但未被使用
- RKISP1_V12：至少用于 rk3326 和 px30
- RKISP1_V13：至少用于 rk1808

## 拓扑


    :alt:   Diagram of the default media pipeline topology
    :align: center


该驱动包含 4 个视频设备：

- rkisp1_mainpath：用于获取图像（通常分辨率较高）的采集设备。
- rkisp1_selfpath：用于获取图像的采集设备。
- rkisp1_stats：发送统计信息的元数据（metadata）采集设备。
- rkisp1_params：从用户空间接收参数配置的元数据输出设备。

该驱动包含 3 个子设备：

- rkisp1_resizer_mainpath：用于为 mainpath 采集设备缩放和降采样帧。
- rkisp1_resizer_selfpath：用于为 selfpath 采集设备缩放和降采样帧。
- rkisp1_isp：连接到传感器，负责所有 isp 操作。


### rkisp1_mainpath、rkisp1_selfpath — 帧采集视频节点

这些是 `mainpath` 和 `selfpath` 采集设备，用于采集帧。这些实体是将帧写入
内存的 DMA 引擎。selfpath 视频设备可采集 YUV/RGB 格式。其输入为 YUV 编码
码流，并能将其转换为 RGB。selfpath 无法采集 bayer 格式。
mainpath 可采集 bayer 和 YUV 格式，但无法采集 RGB 格式。
两个采集视频设备均支持
`V4L2_CAP_IO_MC` 能力 <device-capabilities>。


### rkisp1_resizer_mainpath、rkisp1_resizer_selfpath — 缩放器子设备节点

这些是 mainpath 和 selfpath 的缩放器实体。这些实体可以将帧放大和缩小，并
更改 YUV 采样（例如 YUV4:2:2 -> YUV4:2:0）。它们在 sink pad 上还具有裁剪
能力。缩放器实体只能以 YUV:4:2:2 格式
（MEDIA_BUS_FMT_YUYV8_2X8）工作。
mainpath 采集设备支持采集 bayer 格式的视频。这种情况下，mainpath 的缩放器
被设为 'bypass'（旁路）模式——即直接转发帧而不对其做处理。

### rkisp1_isp — 图像信号处理子设备节点

这是 isp 实体。它通过 sink pad 0 连接到传感器，并使用 CSI-2 协议接收帧。
它负责配置 CSI-2 协议。它在连接到传感器的 sink pad 0 上，以及连接到缩放器
实体的 source pad 2 上具有裁剪能力。
sink pad 0 上的裁剪定义了来自传感器的图像区域。
source pad 2 上的裁剪定义了图像稳定器（IS）的区域。


### rkisp1_stats — 统计视频节点

统计视频节点输出 3A（自动对焦、自动曝光和自动白平衡）统计信息，以及正由
rkisp1 处理、面向用户空间应用程序的帧的直方图统计。
利用这些数据，应用程序可以实现算法，并通过 rkisp_params 节点重新配置驱动，
以在视频流过程中改善图像质量。
缓冲区格式由 struct `rkisp1_stat_buffer` 定义，用户空间应将
V4L2_META_FMT_RK_ISP1_STAT_3A <v4l2-meta-fmt-rk-isp1-stat-3a> 设为
数据格式（dataformat）。


### rkisp1_params — 参数视频节点

rkisp1_params 视频节点从用户空间接收一组参数，在视频流过程中应用到硬件，
允许用户空间动态修改黑电平、串扰校正等数值。

该 ISP 驱动支持两种不同的参数配置方法：`fixed parameters format`（固定参数格式）
或 `extensible parameters format`（可扩展参数格式）。

使用 `fixed parameters`（固定参数）方法时，缓冲区格式由 struct
`rkisp1_params_cfg` 定义，用户空间应将
V4L2_META_FMT_RK_ISP1_PARAMS <v4l2-meta-fmt-rk-isp1-params> 设为
数据格式。

使用 `extensible parameters`（可扩展参数）方法时，缓冲区格式由 struct
`rkisp1_ext_params_cfg` 定义，用户空间应将
V4L2_META_FMT_RK_ISP1_EXT_PARAMS <v4l2-meta-fmt-rk-isp1-ext-params> 设为
数据格式。

## 采集视频帧示例


在下面的示例中，连接到 'rkisp1_isp' 的 pad 0 的传感器是 imx219。

以下命令可用于从 selfpath 视频节点采集尺寸为 900x800、平面格式 YUV 4:2:2
的视频。它使用了所有可用的裁剪能力（说明见下文）。


	# set the links
	"media-ctl" "-d" "platform:rkisp1" "-r"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'imx219 4-0010':0 -> 'rkisp1_isp':0 [^1^]"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'rkisp1_isp':2 -> 'rkisp1_resizer_selfpath':0 [^1^]"
	"media-ctl" "-d" "platform:rkisp1" "-l" "'rkisp1_isp':2 -> 'rkisp1_resizer_mainpath':0 [^0^]"

	# set format for imx219 4-0010:0
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"imx219 4-0010":0 [fmt:SRGGB10_1X10/1640x1232]'

	# set format for rkisp1_isp pads:
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_isp":0 [fmt:SRGGB10_1X10/1640x1232 crop: (0,0)/1600x1200]'
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_isp":2 [fmt:YUYV8_2X8/1600x1200 crop: (0,0)/1500x1100]'

	# set format for rkisp1_resizer_selfpath pads:
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_resizer_selfpath":0 [fmt:YUYV8_2X8/1500x1100 crop: (300,400)/1400x1000]'
	"media-ctl" "-d" "platform:rkisp1" "--set-v4l2" '"rkisp1_resizer_selfpath":1 [fmt:YUYV8_2X8/900x800]'

	# set format for rkisp1_selfpath:
	"v4l2-ctl" "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "-v" "width=900,height=800,"
	"v4l2-ctl" "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "-v" "pixelformat=422P"

	# start streaming:
	v4l2-ctl "-z" "platform:rkisp1" "-d" "rkisp1_selfpath" "--stream-mmap" "--stream-count" "10"


在上述示例中，传感器被配置为 bayer 格式：
`SRGGB10_1X10/1640x1232`。rkisp1_isp:0 pad 应配置为与传感器相同的 mbus 格式
和尺寸，否则流式传输将以 'EPIPE' 错误失败。因此它也被配置为
`SRGGB10_1X10/1640x1232`。
此外，rkisp1_isp:0 pad 被配置为裁剪 `(0,0)/1600x1200`。

裁剪尺寸会自动传播成为 isp 源 pad `rkisp1_isp:2` 的格式。另一个裁剪操作
配置在 isp 源 pad 上：`(0,0)/1500x1100`。

缩放器的 sink pad `rkisp1_resizer_selfpath` 应配置为格式
`YUYV8_2X8/1500x1100`，以匹配链路另一侧的格式。此外还在其上配置了裁剪
`(300,400)/1400x1000`。

缩放器的源 pad `rkisp1_resizer_selfpath:1` 被配置为格式 `YUYV8_2X8/900x800`。
这意味着缩放器先从接收到的帧中裁剪出 `(300,400)/1400x100` 的窗口，然后将
该窗口缩放到 `900x800` 尺寸。

注意，上述示例未使用 stats-params 控制环。因此采集到的帧不会经过 3A 算法，
质量可能不佳，甚至可能显得偏暗、偏绿。

## 配置量化


该驱动支持 YUV 格式的 limited（受限）和 full range（全范围）量化，其中
limited 为默认。
要在二者之间切换，用户空间应使用 isp（`rkisp1_isp:2`）的 source pad 2 上
子设备的色彩空间转换 API（CSC）。在此 pad 上配置的量化就是 mainpath 和
selfpath 视频节点上所采集视频帧的量化。
注意，即使量化在 `rkisp1_isp:2` 上被配置为全范围，缩放器和采集实体也始终
会报告 `V4L2_QUANTIZATION_DEFAULT`。因此，要获取所配置的量化值，应用程序
应取自 pad `rkisp1_isp:2`。
