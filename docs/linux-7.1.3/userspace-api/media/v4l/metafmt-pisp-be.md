


######## V4L2_META_FMT_RPI_BE_CFG


## Raspberry Pi PiSP 后端（Back End）配置格

Raspberry Pi PiSP 后端内存到内图像信号处理器由用户空间通过 `v4l2_meta_format` 接口，向 `pispbe-config` 输出视频设备节点提供一组配置参数缓冲区来进行配置
PiSP 后端以分块（tiles）方式处理图像，其配置需要填`pisp_be_config.h` 头文件中定义`pisp_be_tiles_config` 的成员，以指定两组不同的参数
`Raspberry Pi PiSP technical specification
<https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_
提供了对 ISP 后端配置和编程模型的详细描述
### 全局配置数据


全局配置数据描述了特定图像中的像素应当如何处理，因此在图像的所有分块之间共享。例如，LSC（镜头阴影校正，Lens Shading Correction）或降噪（Denoise）参数在同一帧的所有分块中是通用的
全局配置数据通过填充 `pisp_be_config` 的成员传递给 ISP
### 分块（Tile）参

由于 ISP 以分块方式处理图像，每一组分块参数描述了图像中单个分块将如何处理。一组分块参数由 160 字节的数据组成，要处理一批分块需要多组分块参数
分块参数通过填充 `pisp_tile` 的成员以`pisp_be_tiles_config` `num_tiles` 字段传递给 ISP
## Raspberry Pi PiSP 后端 uAPI 数据类型


本节描述Raspberry Pi PiSP 后端暴露给用户空间的数据类型。本节仅供参考，关于每个字段的详细描述，请参`Raspberry Pi PiSP technical specification
<https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_銆。