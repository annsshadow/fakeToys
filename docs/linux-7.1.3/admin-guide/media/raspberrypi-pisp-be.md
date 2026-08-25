## Raspberry Pi PiSP 后端内存到内ISP（pisp-be


## PiSP 后端


PiSP 后端是一个内存到内存的图像信号处理器（ISP），它从 DRAM 内存读取图像数据，并根据应用程序通过配置缓冲区中的参数所指定的方式执行图像处理，然后通过两个独立的输出通道将像素数据写回内存

ISP 寄存器与编程模型记录`Raspberry Pi Image Signal Processor (PiSP) Specification document`_ 中

PiSP 后端 ISP 以图块（tile）方式处理图像。图像分块（tessellation）的处理以及底层配置参数的计算，由一个名`libpisp <https://github.com/raspberrypi/libpisp>`_ 的自由软件库实现

完整的图像处理流水线（包括通过兼容 MIPI CSI-2 的采集接口从图像传感器采RAW Bayer 数据、将其存DRAM 内存，并PiSP 后端中进行处理以得到应用程序可用的图像）`libcamera <https://libcamera.org>`_ 中作Raspberry Pi 平台支持的一部分实现

## pisp-be 驱动


Raspberry Pi PiSP 后端（pisp-be）驱动位drivers/media/platform/raspberrypi/pisp-be。它使用 `V4L2 API` 注册若干视频采集与输出设备，使用 `V4L2 subdev API` 注册一个连接这些视频设备的 ISP 子设备，从而形成由 `Media Controller (MC) API` 实现的单一媒体图（media graph）

`pisp-be` 驱动注册的媒体拓扑如下图所示：

    :alt:   默认媒体流水线拓扑图
    :align: center


媒体图注册了以下视频设备节点

- pispbe-input：提交给 ISP 进行处理的图像的输出设备
- pispbe-tdn_input：用于时域去噪（temporal denoise）的输出设备
- pispbe-stitch_input：用于图像拼接（HDR）的输出设备
- pispbe-output0：处理后图像的第一个采集设备
- pispbe-output1：处理后图像的第二个采集设备
- pispbe-tdn_output：用于时域去噪的采集设备
- pispbe-stitch_output：用于图像拼接（HDR）的采集设备
- pispbe-config：用ISP 配置参数的输出设备

### pispbe-input


ISP 处理的图像被排入 `pispbe-input` 输出设备节点。有ISP 输入所支持的图像格式列表，请参`Raspberry Pi Image Signal Processor (PiSP) Specification document`_

### pispbe-tdn_input, pispbe-tdn_output


`pispbe-tdn_input` 输出视频设备接收待时域去噪块处理的图像，这些图像`pispbe-tdn_output` 采集视频设备获取。用户空间负责维护这两个设备上的队列，并确保输出设备上完成的缓冲区被排入输入设备

### pispbe-stitch_input, pispbe-stitch_output


为实HDR（高动态范围）图像处理，使用图像拼接与色调映射（tonemapping）块。`pispbe-stitch_output` 将图像写入内存，`pispbe-stitch_input` 接收先前写入的帧，将其与当前输入图像一起处理。用户空间负责维护这两个设备上的队列，并确保输出设备上完成的缓冲区被排入输入设备

### pispbe-output0, pispbe-output1


这两个采集设备将ISP 处理后的像素数据写入内存

### pispbe-config


`pispbe-config` 输出视频设备接收一个配置参数字段，该字段定义了 ISP 待执行的图像处理

ISP 配置参数的格式由 `pisp_be_tiles_config` C 结构体定义，各参数的含义`Raspberry Pi Image Signal Processor (PiSP) Specification document`_ 中描述

## ISP 配置


ISP 配置仅由参数缓冲区的内容描述。用户空间需要使V4L2 API 配置的唯一参数，是输出与采集视频设备上的图像格式，用于校验参数缓冲区内容的合法性
