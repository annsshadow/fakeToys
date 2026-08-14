
## Intel Image Processing Unit 6（IPU6）输入系统驱动


Copyright |copy| 2023--2024 Intel Corporation

## 简介


本文件记录了位于 drivers/media/pci/intel/ipu6 下的 Intel IPU6（第 6 代图像处理单元）
输入系统（MIPI CSI2 接收器）驱动。

Intel IPU6 可以在某些 Intel SoC 中找到，但并非所有 SKU 都有：

- Tiger Lake
- Jasper Lake
- Alder Lake
- Raptor Lake
- Meteor Lake

Intel IPU6 由两个部分组成——输入系统（ISYS）和处理系统（PSYS）。

输入系统主要作为 MIPI CSI-2 接收器工作，接收并处理来自传感器的图像数据，并将帧
输出到内存。

共有 2 个驱动模块——intel-ipu6 和 intel-ipu6-isys。intel-ipu6 是一个 IPU6 通用
驱动，负责 PCI 配置、固件加载与解析、固件认证、DMA 映射以及 IPU-MMU（内部内存
映射单元）配置。intel_ipu6_isys 实现了 V4L2、Media Controller 和 V4L2 子设备
接口。IPU6 ISYS 驱动支持通过 V4L2 子设备传感器驱动连接到 IPU6 ISYS 的摄像头传感器。

	 有关 IPU6 硬件的信息。

## 输入系统驱动


输入系统驱动主要配置 CSI-2 D-PHY，构建固件流配置，向固件发送命令，从硬件和固件
获取响应，然后将缓冲区返回给用户。ISYS 被表示为多个 V4L2 子设备以及视频节点。

   :alt: 支持多流的 ipu6 isys 媒体图

   IPU6 ISYS 媒体图，支持多流

该图是使用以下命令生成的：


   fdp -Gsplines=true -Tsvg < dot > dot.svg

### 使用 IPU6 ISYS 捕获帧


IPU6 ISYS 用于从连接到 CSI2 端口的摄像头传感器捕获帧。ISYS 支持的输入格式列于下
表：


    :header-rows: 1

    - - IPU6 ISYS 支持的输入格式

    - - RGB565, RGB888

    - - UYVY8, YUYV8

    - - RAW8, RAW10, RAW12


#### 示例


以下是 IPU6 ISYS 在 Dell XPS 9315 笔记本上进行 raw 捕获的示例。在该机器上，ov01a10
传感器连接到 IPU ISYS CSI-2 端口 2，可以以 1280x800 分辨率生成 sBGGR10 图像。

使用媒体控制器 API，我们可以通过 media-ctl [#f1]_ 和 yavta [#f2]_ 配置 ov01a10
传感器，将帧传输到 IPU6 ISYS。


    # Example 1 capture frame from ov01a10 camera sensor
    # This example assumes /dev/media0 as the IPU ISYS media device
    export MDEV=/dev/media0

    # Establish the link for the media devices using media-ctl
    media-ctl -d $MDEV -l "\"ov01a10 3-0036\":0 -> \"Intel IPU6 CSI2 2\":0[^1^]"

    # Set the format for the media devices
    media-ctl -d $MDEV -V "ov01a10:0 [fmt:SBGGR10/1280x800]"
    media-ctl -d $MDEV -V "Intel IPU6 CSI2 2:0 [fmt:SBGGR10/1280x800]"
    media-ctl -d $MDEV -V "Intel IPU6 CSI2 2:1 [fmt:SBGGR10/1280x800]"

配置好媒体流水线之后，可以使用 yavta 工具设置所需的传感器特定设置（例如曝光和增益
设置）。

例如


    # and that ov01a10 sensor is connected to i2c bus 3 with address 0x36
    export SDEV=$(media-ctl -d $MDEV -e "ov01a10 3-0036")

    yavta -w 0x009e0903 400 $SDEV
    yavta -w 0x009e0913 1000 $SDEV
    yavta -w 0x009e0911 2000 $SDEV

设置好所需的传感器设置后，就可以如下进行帧捕获。

例如


    yavta --data-prefix -u -c10 -n5 -I -s 1280x800 --file=/tmp/frame-#.bin \
            -f SBGGR10 $(media-ctl -d $MDEV -e "Intel IPU6 ISYS Capture 0")

通过上述命令，以 1280x800 分辨率和 sBGGR10 格式捕获 10 帧。捕获的帧以
/tmp/frame-#.bin 文件的形式提供。

以下是另一个示例，在 Lenovo X1 Yoga 笔记本上从摄像头传感器 ov2740 进行 IPU6 ISYS
RAW 和元数据捕获。


    media-ctl -l "\"ov2740 14-0036\":0 -> \"Intel IPU6 CSI2 1\":0[^1^]"
    media-ctl -l "\"Intel IPU6 CSI2 1\":1 -> \"Intel IPU6 ISYS Capture 0\":0[^1^]"
    media-ctl -l "\"Intel IPU6 CSI2 1\":2 -> \"Intel IPU6 ISYS Capture 1\":0[^1^]"

    # set routing
    media-ctl -R "\"Intel IPU6 CSI2 1\" [0/0->1/0[^1^],0/1->2/1[^1^]]"

    media-ctl -V "\"Intel IPU6 CSI2 1\":0/0 [fmt:SGRBG10/1932x1092]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":0/1 [fmt:GENERIC_8/97x1]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":1/0 [fmt:SGRBG10/1932x1092]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":2/1 [fmt:GENERIC_8/97x1]"

    CAPTURE_DEV=$(media-ctl -e "Intel IPU6 ISYS Capture 0")
    ./yavta --data-prefix -c100 -n5 -I -s1932x1092 --file=/tmp/frame-#.bin \
        -f SGRBG10 ${CAPTURE_DEV}

    CAPTURE_META=$(media-ctl -e "Intel IPU6 ISYS Capture 1")
    ./yavta --data-prefix -c100 -n5 -I -s97x1 -B meta-capture \
        --file=/tmp/meta-#.bin -f GENERIC_8 ${CAPTURE_META}

## 参考资料


