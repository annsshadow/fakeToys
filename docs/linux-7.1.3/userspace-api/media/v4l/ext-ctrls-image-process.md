


######## 图像处理控制参

图像处理控制类用于对图像处理功能进行底层控制。与 `V4L2_CID_IMAGE_SOURCE_CLASS`
不同，该类中的控制影响的是图像处理过程，而不是对图像的捕获


## 图像处理控制 ID


`V4L2_CID_IMAGE_PROC_CLASS (class)`
    IMAGE_PROC 类的描述符

`V4L2_CID_LINK_FREQ (integer menu)`
    数据总线的频率（例如并行总线CSI-2）

`V4L2_CID_PIXEL_RATE (64-bit integer)`
    设备像素阵列中的像素采样率。该控制为只读，其单位是 像素/秒
    某些设备使用水平和垂直消隐来配置帧率。帧率可由像素率、模拟裁剪矩形以    水平和垂直消隐计算而得。像素率控制可能位于与消隐控制和模拟裁剪矩形配置
    不同的子设备中
    帧率的配置通过选择期望的水平与垂直消隐来完成。该控制的单位是 Hz
`V4L2_CID_TEST_PATTERN (menu)`
    部分捕获/显示/传感器设备具备生成测试图案的能力。这些硬件特定的测试图案
    可用于测试设备是否工作正常
`V4L2_CID_DEINTERLACING_MODE (menu)`
    视频去隔行模式（例如 Bob、Weave 等）。菜单项由驱动特定，并在 uapi-v4l-drivers
    中有文档说明
`V4L2_CID_DIGITAL_GAIN (integer)`
    数字增益是所有颜色分量所乘的数值。通常所施加的数字增益为控制值除以例    0x100，也就是说，要得到无数字增益，控制值需要为 0x100。无增益配置通常也是
    默认值