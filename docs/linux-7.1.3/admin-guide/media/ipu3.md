

## Intel 图像处理单元 3（IPU3）成像单元（ImgU）驱


Copyright |copy| 2018 Intel Corporation

## 简


本文档说明了位于 drivers/media/pci/intel/ipu3（CIO2）以drivers/staging/media/ipu3（ImgU）下
Intel IPU3（第三代图像处理单元）成像单元驱动

Intel IPU3 出现在某Kaby Lake（以及某Sky Lake）平台（U/Y 处理器系列）中，由两个部分组成，
即成像单元（ImgU）和 CIO2 设备（MIPI CSI2 接收器）

CIO2 设备从传感器接收原始 Bayer 数据，并IPU3 特有的格式（IPU3 ImgU 消费）输出帧
CIO2 驱动位于 drivers/media/pci/intel/ipu3/ipu3-cio2*，并通过 CONFIG_VIDEO_IPU3_CIO2 配置选项启用

成像单元（ImgU）负责处理由 IPU3 CIO2 设备捕获的图像。ImgU 驱动源码位于 drivers/staging/media/ipu3
目录。该驱动通过 CONFIG_VIDEO_IPU3_IMGU 配置选项启用

两个驱动模块分别名为 ipu3_csi2 ipu3_imgu

这些驱动已在 Kaby Lake 平台（U/Y 处理器系列）上进行了测试

两个驱动均实现了 V4L2、Media Controller 以及 V4L2 子设备接口。IPU3 CIO2 驱动支持通过 V4L2 子设
传感器驱动连接到 CIO2 MIPI CSI-2 接口的摄像头传感器

## CIO2


CIO2 表现为一V4L2 子设备，向用户空间提V4L2 子设备接口。每CSI-2 接收器都有一个视频节点，
整个设备有一Media Controller 接口

CIO2 包含四个独立的捕获通道，每个通道都有自己独立MIPI CSI-2 接收器和 DMA 引擎。每个通道被建模为
一V4L2 子设备，向用户空间暴露为一V4L2 子设备节点，并有两个 pad


    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - MIPI CSI-2 输入，连接到传感器子设备

    - - 1
      - source
      - 原始视频捕获，连接到 V4L2 视频接口

V4L2 视频接口DMA 引擎进行建模。它们以 V4L2 视频设备节点形式向用户空间暴露

### 以原Bayer 格式捕获


CIO2 MIPI CSI2 接收器用于从连接CSI2 端口的原始传感器捕获帧（采用 packed 原始 Bayer 格式）
捕获的帧作为 ImgU 驱动的输入

使用 IPU3 ImgU 进行图像处理需要诸raw2pnm [#f1]_ yavta [#f2]_ 之类的工具，原因是由IPU3
特有的以下需求及/或特性

-- IPU3 CSI2 接收器以 IPU3 特有packed 原始 Bayer 格式从传感器输出捕获的帧

-- 必须同时操作多个视频节点

让我们以连接CSI2 端口 0 ov5670 传感器为例，进行 2592x1944 的图像捕获

使用 Media Controller API，将 ov5670 传感器配置为packed 原始 Bayer 格式IPU3 CSI2 接收器发送帧


    # 本示例假/dev/media0 CIO2 媒体设备
    export MDEV=/dev/media0

    # 并假ov5670 传感器连接到 i2c 总线 10，地址0x36
    export SDEV=$(media-ctl -d $MDEV -e "ov5670 10-0036")

    # 使用 media-ctl 建立媒体设备的连
    media-ctl -d $MDEV -l "ov5670:0 -> ipu3-csi2 0:0[^1^]"

    # 设置媒体设备的格
    media-ctl -d $MDEV -V "ov5670:0 [fmt:SGRBG10/2592x1944]"
    media-ctl -d $MDEV -V "ipu3-csi2 0:0 [fmt:SGRBG10/2592x1944]"
    media-ctl -d $MDEV -V "ipu3-csi2 0:1 [fmt:SGRBG10/2592x1944]"

媒体管道配置完成后，可以使用 yavta 工具设置所需的传感器特定设置（例如曝光和增益设置）

例如


    yavta -w 0x009e0903 444 $SDEV
    yavta -w 0x009e0913 1024 $SDEV
    yavta -w 0x009e0911 2046 $SDEV

设置好所需的传感器设置后，即可按如下方式捕获帧

例如


    yavta --data-prefix -u -c10 -n5 -I -s2592x1944 --file=/tmp/frame-#.bin \
          -f IPU3_SGRBG10 $(media-ctl -d $MDEV -e "ipu3-cio2 0")

通过上述命令，以 2592x1944 分辨率、sGRBG10 格式捕获 10 帧，并以 IPU3_SGRBG10 格式输出

捕获的帧/tmp/frame-#.bin 文件形式提供

## ImgU


ImgU 表现为两V4L2 子设备，每个都向用户空间提供一V4L2 子设备接口

每个 V4L2 子设备代表一条管道（pipe），最多可支持 2 路流。这有助于支持高级摄像头特性，例如
连续取景器（CVF）和视频中抓拍（SDV）

ImgU 包含两条独立的管道，每条都被建模为一V4L2 子设备，V4L2 子设备节点形式向用户空间暴露

每条管道有两sink pad 和三source pad，用途如下：


    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 输入原始视频

    - - 1
      - sink
      - 处理参数

    - - 2
      - source
      - 输出处理后的视频

    - - 3
      - source
      - 输出取景器视频流

    - - 4
      - source
      - 3A 统计信息

每个 pad 都连接到一个相应的 V4L2 视频接口，以 V4L2 视频设备节点形式向用户空间暴露

### 设备操作


对于 ImgU，一旦输入视频节点（"ipu3-imgu 0/1":0，采<entity>:<pad-number> 格式）被填入缓冲
（packed 原始 Bayer 格式），ImgU 就开始处理该缓冲区，并在各自的输出节点上YUV 格式产生视频输出
以及统计信息输出。当输入视频节点被填入缓冲区时，驱动应当已为参数、输出和统计信息所有节点准备好缓冲区

至少，输入、主输出A 统计信息和取景器视频节点都应启用，IPU3 才能开始图像处理

每个 ImgU V4L2 子设备具有以下一组视频节点

### 输入、输出和取景器视频节


输入视频节点收到的帧（采IPU3 特有packed 原始 Bayer 格式）由 IPU3 成像单元处理，并输出
2 个视频节点，每个面向不同用途（主输出和取景器输出）

有关 IPU3 特有Bayer 格式详情，请参见 v4l2-pix-fmt-ipu3-sbggr10

该驱动支持在 devices 中定义的 V4L2 视频捕获接口

仅支持多平面（multi-planar）API。更多详情请参见 planar-apis

### 参数视频节点


参数视频节点接收用于配置 ImgU 算法如何处理图像ImgU 算法参数

有关 IPU3 特有的处理参数详情，请参v4l2-meta-fmt-params

### 3A 统计信息视频节点


3A 统计信息视频节点ImgU 驱动用来向用户空间应用程序输出正在被 ImgU 处理的帧3A
（自动对焦、自动曝光和自动白平衡）统计信息。用户空间应用程序可以利用这些统计数据计ImgU 所需
算法参数

## 配置 Intel IPU3


IPU3 ImgU 管道可以使用 Media Controller 配置，定义见 media_controller

### 运行模式与固件二进制文件选择


ImgU 基于固件工作，目ImgU 固件支持以时分方式运2 条管道，使用单个输入帧数据。每条管道可以运行在
特定模式 —"VIDEO" "STILL"VIDEO" 模式通常用于视频帧捕获，"STILL" 用于静态帧捕获。不过，如果
希望以更小的系统负载和功耗捕获图像，也可以选择 "VIDEO" 来捕获静态帧。对"STILL" 模式，ImgU 会尝
使用更小BDS 因子，并输出"VIDEO" 模式更大bayer 帧用于后YUV 处理，以获得高质量图像。此外，
"STILL" 模式需XNR3 进行降噪，因"STILL" 模式"VIDEO" 模式需要更多的功耗和内存带宽。TNR 
"VIDEO" 模式下启用，"STILL" 模式下被旁路。ImgU 默认"VIDEO" 模式运行，用户可以使v4l2 控制
V4L2_CID_INTEL_IPU3_MODE（当前定义于 drivers/staging/media/ipu3/include/uapi/intel-ipu3.h）来查询
设置运行模式。对于用户而言VIDEO" "STILL" 模式在缓冲区排队上没有区别，必须启用输入和主输出节点
并排队缓冲区，统计信息和取景器队列是可选的

固件二进制文件将根据当前运行模式选择，如果你启用 ImgU 动态调试，可以观察到诸
"using binary if_to_osys_striped " "using binary if_to_osys_primary_striped" 的日志，
二进制文if_to_osys_striped 用于 "VIDEO"，而二进制文件 "if_to_osys_primary_striped" 用于 "STILL"


### 以原Bayer 格式处理图像


#### 配置 ImgU V4L2 子设备进行图像处


ImgU V4L2 子设备必须使Media Controller API 进行配置，以正确建立所有视频节点

让我们以 "ipu3-imgu 0" 子设备为例


    media-ctl -d $MDEV -r
    media-ctl -d $MDEV -l "ipu3-imgu 0 input":0 -> "ipu3-imgu 0":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":2 -> "ipu3-imgu 0 output":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":3 -> "ipu3-imgu 0 viewfinder":0[^1^]
    media-ctl -d $MDEV -l "ipu3-imgu 0":4 -> "ipu3-imgu 0 3a stat":0[^1^]

此外，相V4L2 子设备的管道模式应按需设置（例0 表示视频模式 表示静态模式），通过控制 id 0x009819a1
如下所示


    yavta -w "0x009819A1 1" /dev/v4l-subdev7

ImgU 管道中的某些硬件模块可以通过裁剪或缩放改变帧分辨率，这些硬件模块包括输入馈送器（IF）、Bayer
缩小器（BDS）和几何畸变校正（GDC）。还有一个可以改变帧分辨率的模块 —YUV 缩放器，它仅适用于次级输出

原始 Bayer 帧经过这ImgU 管道硬件模块，最终处理后的图像输出到 DDR 内存

   :alt: ipu3 resolution blocks image

   IPU3 分辨率改变硬件模

**Input Feeder（输入馈送器*

输入馈送器从传感器获取 Bayer 帧数据，它可以对帧中的行和列进行裁剪，然后将像素存入设备的内部像素缓冲区
供后续模块读出

**Bayer Down Scaler（Bayer 缩小器）**

Bayer 缩小器能够在 Bayer 域执行图像缩放，缩小因子可在每个轴上1X 配置1/4X，配置步长为
0.03125/32）

**Geometric Distortion Correction（几何畸变校正）**

几何畸变校正用于执行畸变校正和图像滤波。它需要一些额外的滤波器和包络填充像素才能工作，因GDC 的输
分辨率应大于输出分辨率

**YUV Scaler（YUV 缩放器）**

YUV 缩放器与 BDS 类似，但主要YUV 域进行图像缩小，它最多支1/12X 缩小，但不能应用于主输出

对于给定的输入分辨率，ImgU V4L2 子设备必须在上述所有硬件模块中配置受支持的分辨率。对于给定的输入
受支持的分辨率，输入馈送器、Bayer 缩小器和 GDC 模块都应配置为受支持的分辨率，因为每个硬件模块都
自己的对齐要求

你必须巧妙地配置硬件模块的输出分辨率，既满足硬件要求，又保持最大的视场。中间分辨率可以由特定工具生—

https://github.com/intel/intel-ipu3-pipecfg

该工具可用于生成中间分辨率。更多信息可通过查看以下 IPU3 ImgU 配置表获得

https://chromium.googlesource.com/chromiumos/overlays/board-overlays/+/master

baseboard-poppy/media-libs/cros-camera-hal-configs-poppy/files/gcss 目录下，
graph_settings_ov5670.xml 可作为示例

以下步骤ImgU 管道准备图像处理

1. 应使GDC 获得的宽度和高度，在 pad 0 上通过 VIDIOC_SUBDEV_S_FMT 设置 ImgU V4L2 子设备数据格式

2. 应在 pad 0 上通过 VIDIOC_SUBDEV_S_SELECTION 设置 ImgU V4L2 子设备的裁剪，目标为 V4L2_SEL_TGT_CROP
使用输入馈送器的高度和宽度

3. 应在 pad 0 上通过 VIDIOC_SUBDEV_S_SELECTION 设置 ImgU V4L2 子设备的合成，目标为 V4L2_SEL_TGT_COMPOSE
使用 BDS 的高度和宽度

ov5670 为例，对于分辨率2592x1944（输入到 ImgU 子设pad 0）的输入帧，输入馈送器、BDS GDC 
相应分辨率分别为 2592x1944592x1944 2560x1920

完成上述步骤后，可以使用如下方式将接收到的原Bayer 帧输入到 ImgU V4L2 子设备，使用开源应用程v4l2n [#f1]_

对于2592x1944 [#f4]_ 分辨率捕获、期望输出分辨率2560x1920、取景器分辨率为 2560x1920 的图像，
可以使用以下 v4l2n 命令。这有助于处理原Bayer 帧，并以 NV12 格式产生主输出图像和取景器输出的期望结果


    v4l2n --pipe=4 --load=/tmp/frame-#.bin --open=/dev/video4
          --fmt=type:VIDEO_OUTPUT_MPLANE,width=2592,height=1944,pixelformat=0X47337069 \
          --reqbufs=type:VIDEO_OUTPUT_MPLANE,count:1 --pipe=1 \
          --output=/tmp/frames.out --open=/dev/video5 \
          --fmt=type:VIDEO_CAPTURE_MPLANE,width=2560,height=1920,pixelformat=NV12 \
          --reqbufs=type:VIDEO_CAPTURE_MPLANE,count:1 --pipe=2 \
          --output=/tmp/frames.vf --open=/dev/video6 \
          --fmt=type:VIDEO_CAPTURE_MPLANE,width=2560,height=1920,pixelformat=NV12 \
          --reqbufs=type:VIDEO_CAPTURE_MPLANE,count:1 --pipe=3 --open=/dev/video7 \
          --output=/tmp/frames.3A --fmt=type:META_CAPTURE,? \
          --reqbufs=count:1,type:META_CAPTURE --pipe=1,2,3,4 --stream=5

你也可以使用 yavta [#f2]_ 命令完成与上述相同的操作


    yavta --data-prefix -Bcapture-mplane -c10 -n5 -I -s2592x1944 \
          --file=frame-#.out-f NV12 /dev/video5 & \
    yavta --data-prefix -Bcapture-mplane -c10 -n5 -I -s2592x1944 \
          --file=frame-#.vf -f NV12 /dev/video6 & \
    yavta --data-prefix -Bmeta-capture -c10 -n5 -I \
          --file=frame-#.3a /dev/video7 & \
    yavta --data-prefix -Boutput-mplane -c10 -n5 -I -s2592x1944 \
          --file=/tmp/frame-in.cio2 -f IPU3_SGRBG10 /dev/video4

其中 /dev/video4dev/video5dev/video6 /dev/video7 设备分别指向输入、输出、取景器
3A 统计信息视频节点

### 将原Bayer 图像转换YUV 


上述步骤处理后的图像可以如下方式转换YUV 域

#### 主输出帧



    raw2pnm -x2560 -y1920 -fNV12 /tmp/frames.out /tmp/frames.out.ppm

其中 2560x1920 为输出分辨率，NV12 为视频格式，其后为输入帧和输PNM 文件

#### 取景器输出帧



    raw2pnm -x2560 -y1920 -fNV12 /tmp/frames.vf /tmp/frames.vf.ppm

其中 2560x1920 为输出分辨率，NV12 为视频格式，其后为输入帧和输PNM 文件

## IPU3 的用户空间示例代


配置并使IPU3 的用户空间代码可在此处获取

https://chromium.googlesource.com/chromiumos/platform/arc-camera/+/master/

源码位于 hal/intel 目录下

## IPU3 管道概述


IPU3 管道有多个图像处理阶段，每个阶段接收一组参数作为输入。管道的主要阶段如下所示：

   :alt: IPU3 ImgU Pipeline
   :caption: IPU3 ImgU Pipeline Diagram

   digraph "IPU3 ImgU" {
       node [shape=box]
       splines="ortho"
       rankdir="LR"

       a [label="Raw pixels"]
       b [label="Bayer Downscaling"]
       c [label="Optical Black Correction"]
       d [label="Linearization"]
       e [label="Lens Shading Correction"]
       f [label="White Balance / Exposure / Focus Apply"]
       g [label="Bayer Noise Reduction"]
       h [label="ANR"]
       i [label="Demosaicing"]
       j [label="Color Correction Matrix"]
       k [label="Gamma correction"]
       l [label="Color Space Conversion"]
       m [label="Chroma Down Scaling"]
       n [label="Chromatic Noise Reduction"]
       o [label="Total Color Correction"]
       p [label="XNR3"]
       q [label="TNR"]
       r [label="DDR", style=filled, fillcolor=yellow, shape=cylinder]
       s [label="YUV Downscaling"]
       t [label="DDR", style=filled, fillcolor=yellow, shape=cylinder]

       { rank=same; a -> b -> c -> d -> e -> f -> g -> h -> i }
       { rank=same; j -> k -> l -> m -> n -> o -> p -> q -> s -> t}

       a -> j [style=invis, weight=10]
       i -> j
       q -> r
   }

下表给出了上述算法的描述

======================== =======================================================
Name			 Description
======================== =======================================================
Optical Black Correction Optical Black Correction 模块从相应的像素值中减去一个预定义
			 的值，以获得更好的图像质量
			 定义struct ipu3_uapi_obgrid_param
Linearization		 Linearization 算法块使用线性化参数来解决传感器非线性效应
			 查找表定义于
			 struct ipu3_uapi_isp_lin_vmem_params銆。
SHD			 Lens shading correction 用于校正由于光学镜头阴影导致
			 像素响应的空间不均匀性。这是通过对每个像素应用不同的增益
			 来实现的。增益、黑电平等在
			 struct ipu3_uapi_shd_config_static 中配置
BNR			 Bayer 降噪模块通过应用双边滤波器来去除图像噪声
			 详见 struct ipu3_uapi_bnr_static_config
ANR			 Advanced Noise Reduction 是一种基于块的算法，Bayer 
			 执行降噪。卷积矩阵等可在
			 struct ipu3_uapi_anr_config 中找到
DM			 Demosaicing Bayer 格式的原始传感器数据转换
			 RGB（红、绿、蓝）表示。然后为后续由固件进行的流处
			 添加 Y 通道的估计输出。该结构体定义为
			 struct ipu3_uapi_dm_config銆。
Color Correction	 Color Correction 算法将传感器特定的色彩空间转换为标准
			 "sRGB" 色彩空间。这是通过应用定义
			 struct ipu3_uapi_ccm_mat_config 3x3 矩阵实现的
Gamma correction	 Gamma correction 结构struct ipu3_uapi_gamma_config 是一
			 基本的非线性色调映射校正，对每个像素的每个分量逐像素应用
CSC			 Color space conversion 将每个像素从 RGB 原色表示转换
			 YUV（Y：亮度，UV：色度）表示。这是通过应用定义
			 struct ipu3_uapi_csc_mat_config 3x3 矩阵实现的
CDS			 Chroma down sampling（色度下采样
			 CSC 执行后，应用 Chroma Down Sampling UV 平面进行
			 下采样，对于 YUV 4:2:0，每个方向按因子 2 使用 4x2 
			 可配置滤波器 struct ipu3_uapi_cds_params
CHNR			 Chroma noise reduction（色度降噪）
			 该模块仅处理色度像素，并通过清除高频噪声来执行降噪
			 参见 struct struct ipu3_uapi_yuvp1_chnr_config
TCC			 Total color correction，定义于结构
			 struct ipu3_uapi_yuvp2_tcc_static_config銆。
XNR3			 eXtreme Noise Reduction V3 是第三代降噪算法，用于改
			 图像质量。它去除所捕获图像中的低频噪声。定义了两个相关
			 结构体：用于 ISP 数据内存struct ipu3_uapi_isp_xnr3_params
			 和用于向量内存的 struct ipu3_uapi_isp_xnr3_vmem_params
TNR			 Temporal Noise Reduction 模块比较时间上连续的帧，以去
			 像素值中的异噪声。为 ISP 向量和数据内存分别定义了
			 struct ipu3_uapi_isp_tnr3_vmem_params 鍜。
			 struct ipu3_uapi_isp_tnr3_params銆。
======================== =======================================================

上表未列出的其他常见缩写

	ACC
		Accelerator cluster（加速集群）
	AWB_FR
		Auto white balance filter response statistics（自动白平衡滤波响应统计
	BDS
		Bayer downscaler parameters（Bayer 缩小器参数）
	CCM
		Color correction matrix coefficients（色彩校正矩阵系数）
	IEFd
		Image enhancement filter directed（图像增强滤波定向）
	Obgrid
		Optical black level compensation（光学黑电平补偿
	OSYS
		Output system configuration（输出系统配置）
	ROI
		Region of interest（感兴趣区域
	YDS
		Y down sampling（Y 下采样）
	YTM
		Y-tone mapping（Y 色调映射

管道的一些阶段将由运行在 ISP 处理器上的固件执行，而许多其他阶段将使用一组固定的硬件模块（也称为
加速集群，ACC）来处理像素数据并生成统计信息

各个算法ACC 参数（由 struct ipu3_uapi_acc_param 定义）可被选择由用户空间通过嵌入
struct ipu3_uapi_params 结构体中struct struct ipu3_uapi_flags 来应用。对于被用户空间配置
未启用的参数，相应的结构体将被驱动忽略，在这种情况下，该算法的现有配置将被保留

## 参考资




