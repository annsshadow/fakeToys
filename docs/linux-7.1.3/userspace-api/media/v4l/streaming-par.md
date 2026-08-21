######## 流参数（Streaming Parameters

流参数（Streaming parameters）旨在优化视频采集过程以I/O。目前，应用程序可以通过 VIDIOC_S_PARM <VIDIOC_G_PARM> ioctl 请求高质量采集模式
当前的视频标准决定了标称的每秒帧数。如果要采集或输出的帧数少于该数值，应用程序可以请求驱动程序端跳过或复制帧。这在使`read()` `write()` 时尤其有用，因为它们没有时间戳或序列计数器的增强，也可用于避免不必要的数据拷贝
最后，这些 ioctl 还可用于确定驱动程序在读写模式下内部使用的缓冲区数量。相关影响请参阅讨论 `read()` 函数的章节
要获取和设置流参数，应用程序分别调用 VIDIOC_G_PARM <VIDIOC_G_PARM> VIDIOC_S_PARM <VIDIOC_G_PARM> ioctl。它们接收一个指struct `v4l2_streamparm` 的指针，其中包含一个联合体（union），保存着针对输入和输出设备的独立参数
这些 ioctl 是可选的，驱动程序无需实现它们。如果未实现，它们会返回 `EINVAL` 错误码