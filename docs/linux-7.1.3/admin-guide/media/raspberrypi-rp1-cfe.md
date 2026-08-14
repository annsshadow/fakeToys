
## Raspberry Pi PiSP 相机前端（rp1-cfe）


## PiSP 相机前端（Camera Front End）


PiSP 相机前端（CFE）是一个模块，它将一个 CSI-2 接收器与一个称为前端（FE）的简单 ISP 组合在一起。

CFE 有四个 DMA 引擎，可以将从 CSI-2 接收到的四个独立流中的帧写入内存。其中某一个流也可以直接路由到 FE，FE 可以进行最少的图像处理，将接收到的帧的两个版本（例如未缩放版本与缩小版本）写入内存，并提供所接收帧的统计信息。

FE 寄存器记录在 `Raspberry Pi Image Signal Processor (ISP) Specification document <https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_ 中，FE 的示例代码可以在 `libpisp <https://github.com/raspberrypi/libpisp>`_ 中找到。

## rp1-cfe 驱动


Raspberry Pi PiSP 相机前端（rp1-cfe）驱动位于 drivers/media/platform/raspberrypi/rp1-cfe。它使用 `V4L2 API` 注册若干视频捕获与输出设备，使用 `V4L2 subdev API` 为 CSI-2 接收端与连接视频设备的 FE 注册子设备，这些设备由一个使用 `Media Controller (MC) API` 实现的单一媒体图连接。

由 `rp1-cfe` 驱动注册的媒体拓扑，在这个连接到 imx219 传感器的特定示例中，如下所述：


    :alt:   一个示例媒体流水线拓扑图
    :align: center

该媒体图包含以下视频设备节点：

- rp1-cfe-csi2-ch0：第一个 CSI-2 流的捕获设备
- rp1-cfe-csi2-ch1：第二个 CSI-2 流的捕获设备
- rp1-cfe-csi2-ch2：第三个 CSI-2 流的捕获设备
- rp1-cfe-csi2-ch3：第四个 CSI-2 流的捕获设备
- rp1-cfe-fe-image0：第一个 FE 输出的捕获设备
- rp1-cfe-fe-image1：第二个 FE 输出的捕获设备
- rp1-cfe-fe-stats：FE 统计信息的捕获设备
- rp1-cfe-fe-config：FE 配置的输出设备

### rp1-cfe-csi2-chX


rp1-cfe-csi2-chX 捕获设备是普通的 V4L2 捕获设备，可用于捕获从 CSI-2 接收到的视频帧或元数据。

### rp1-cfe-fe-image0, rp1-cfe-fe-image1


rp1-cfe-fe-image0 与 rp1-cfe-fe-image1 捕获设备用于将处理后的帧写入内存。

### rp1-cfe-fe-stats


FE 统计信息缓冲区的格式由 `pisp_statistics` C 结构体定义，每个参数的含义在 `PiSP specification` 文档中描述。

### rp1-cfe-fe-config


FE 配置缓冲区的格式由 `pisp_fe_config` C 结构体定义，每个参数的含义在 `PiSP specification` 文档中描述。
