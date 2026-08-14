## i.MX7 视频采集驱动

本文档说明 i.MX7 处理器的视频采集驱动架构与媒体管线，涵盖 MIPI CSI-2 接收器、视频多路复用器与 CMOS 传感器接口（CSI）等硬件单元，以及它们在 V4L2 框架下暴露的实体与数据路径。

### 简介（Introduction）


与 i.MX5/6 系列不同，i.MX7 不包含图像处理单元（IPU）；因此，执行操作或处理采集帧的能力在功能上较不丰富。

i.MX7 的采集包含三个单元：
- CMOS 传感器接口（CSI）
- 视频多路复用器（Video Multiplexer）
- MIPI CSI-2 接收器（MIPI CSI-2 Receiver）


   MIPI Camera Input ---> MIPI CSI-2 --- > |\
                                           | \
                                           |  \
                                           | M |
                                           | U | ------>  CSI ---> Capture
                                           | X |
                                           |  /
   Parallel Camera Input ----------------> | /
                                           |/

更多信息，请参考最新版本的 i.MX7 参考手册 [#f1]_。

### 实体（Entities）


### imx-mipi-csi2


这是 MIPI CSI-2 接收器实体。它有一个 sink pad 用于接收来自 MIPI CSI-2 摄像头传感器的像素数据。它有一个 source pad，对应于虚拟通道 0。该模块兼容早期版本的 Samsung D-phy，并支持两条 D-PHY Rx 数据通道。

### csi-mux


这是视频多路复用器。它有两个 sink pad，用于从带有并行接口的摄像头传感器或 MIPI CSI-2 虚拟通道 0 中选择。它有一个单一的 source pad 路由到 CSI。

### csi


CSI 使芯片能够直接连接到外部 CMOS 图像传感器。CSI 可以直接与并行和 MIPI CSI-2 总线接口。它拥有 256 x 64 的 FIFO 用于存储接收到的图像像素数据，以及嵌入式 DMA 控制器用于通过 AHB 总线从 FIFO 传输数据。

该实体有一个 sink pad 从 csi-mux 实体接收数据，以及一个单一的 source pad 将视频帧直接路由到内存缓冲区。该 pad 路由到一个采集设备节点。

### 使用说明（Usage Notes）


为了辅助配置，并为了与那些仅从视频设备节点访问控制项的 V4L2 应用程序向后兼容，采集设备接口会从当前流水线中的活动实体继承控制项，因此既可以直接从子设备（subdev）访问控制项，也可以从活动采集设备接口访问。例如，传感器控制项既可以从传感器子设备获取，也可以从活动采集设备获取。

### 搭配 OV2680 的 Warp7


在此平台上，一个 OV2680 MIPI CSI-2 模块连接到内部 MIPI CSI-2 接收器。以下示例配置了一条视频采集流水线，输出为 800x600，BGGR 10 位 bayer 格式：


   # Setup links
   media-ctl -l "'ov2680 1-0036':0 -> 'imx7-mipi-csis.0':0[^1^]"
   media-ctl -l "'imx7-mipi-csis.0':1 -> 'csi-mux':1[^1^]"
   media-ctl -l "'csi-mux':2 -> 'csi':0[^1^]"
   media-ctl -l "'csi':1 -> 'csi capture':0[^1^]"

   # Configure pads for pipeline
   media-ctl -V "'ov2680 1-0036':0 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi-mux':1 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi-mux':2 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'imx7-mipi-csis.0':0 [fmt:SBGGR10_1X10/800x600 field:none]"
   media-ctl -V "'csi':0 [fmt:SBGGR10_1X10/800x600 field:none]"

此后即可开始流式传输。v4l2-ctl 工具可用于选择传感器支持的任何分辨率。


	# media-ctl -p
	Media controller API version 5.2.0

# 	Media device information

	driver          imx7-csi
	model           imx-media
	serial
	bus info
	hw revision     0x0
	driver version  5.2.0

	Device topology
 - entity 1: csi (2 pads, 2 links)
	            type V4L2 subdev subtype Unknown flags 0
	            device node name /dev/v4l-subdev0
	        pad0: Sink
	                [fmt:SBGGR10_1X10/800x600 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                <- "csi-mux":2 [ENABLED]
	        pad1: Source
	                [fmt:SBGGR10_1X10/800x600 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi capture":0 [ENABLED]

 - entity 4: csi capture (1 pad, 1 link)
	            type Node subtype V4L flags 0
	            device node name /dev/video0
	        pad0: Sink
	                <- "csi":1 [ENABLED]

 - entity 10: csi-mux (3 pads, 2 links)
	             type V4L2 subdev subtype Unknown flags 0
	             device node name /dev/v4l-subdev1
	        pad0: Sink
	                [fmt:Y8_1X8/1x1 field:none]
	        pad1: Sink
	               [fmt:SBGGR10_1X10/800x600 field:none]
	                <- "imx7-mipi-csis.0":1 [ENABLED]
	        pad2: Source
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                -> "csi":0 [ENABLED]

 - entity 14: imx7-mipi-csis.0 (2 pads, 2 links)
	             type V4L2 subdev subtype Unknown flags 0
	             device node name /dev/v4l-subdev2
	        pad0: Sink
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                <- "ov2680 1-0036":0 [ENABLED]
	        pad1: Source
	                [fmt:SBGGR10_1X10/800x600 field:none]
	                -> "csi-mux":1 [ENABLED]

 - entity 17: ov2680 1-0036 (1 pad, 1 link)
	             type V4L2 subdev subtype Sensor flags 0
	             device node name /dev/v4l-subdev3
	        pad0: Source
	                [fmt:SBGGR10_1X10/800x600@1/30 field:none colorspace:srgb]
	                -> "imx7-mipi-csis.0":0 [ENABLED]

### 搭配 OV5640 的 i.MX6ULL-EVK


在此平台上，一个并行的 OV5640 传感器连接到 CSI 端口。
以下示例配置了一条视频采集流水线，输出为 640x480，格式为 UYVY8_2X8：


   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'csi':0[^1^]"
   media-ctl -l "'csi':1 -> 'csi capture':0[^1^]"

   # Configure pads for pipeline
   media-ctl -v -V "'ov5640 1-003c':0 [fmt:UYVY8_2X8/640x480 field:none]"

此后即可开始流式传输：


   gst-launch-1.0 -v v4l2src device=/dev/video1 ! video/x-raw,format=UYVY,width=640,height=480 ! v4l2convert ! fbdevsink


	# media-ctl -p
	Media controller API version 5.14.0

# 	Media device information

	driver          imx7-csi
	model           imx-media
	serial
	bus info
	hw revision     0x0
	driver version  5.14.0

	Device topology
 - entity 1: csi (2 pads, 2 links)
	            type V4L2 subdev subtype Unknown flags 0
	            device node name /dev/v4l-subdev0
	        pad0: Sink
	                [fmt:UYVY8_2X8/640x480 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                <- "ov5640 1-003c":0 [ENABLED,IMMUTABLE]
	        pad1: Source
	                [fmt:UYVY8_2X8/640x480 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi capture":0 [ENABLED,IMMUTABLE]

 - entity 4: csi capture (1 pad, 1 link)
	            type Node subtype V4L flags 0
	            device node name /dev/video1
	        pad0: Sink
	                <- "csi":1 [ENABLED,IMMUTABLE]

 - entity 10: ov5640 1-003c (1 pad, 1 link)
	             type V4L2 subdev subtype Sensor flags 0
	             device node name /dev/v4l-subdev1
	        pad0: Source
	                [fmt:UYVY8_2X8/640x480@1/30 field:none colorspace:srgb xfer:srgb ycbcr:601 quantization:full-range]
	                -> "csi":0 [ENABLED,IMMUTABLE]

### 参考（References）
