######## 与旧裁剪 API 的比

选择 API 的引入是为了应对较旧CROP API <crop> 的不足，后者设计用于控简单的捕获设备。后来裁API 被视频输出驱动所采用。ioctl 用于选择显示视频信号被插入的部分。这应被视为一API 滥用，因为所描述的操作实际上
是合成。选择 API 通过设置适当的目标，在合成与裁剪操作之间做了清晰的区分
CROP API 缺乏对内存缓冲区中图像的合成与裁剪的任何支持。应用程序可以通过
滥用 V4L2 API，将捕获设备配置为仅填充图像的一部分。从较大图像中裁剪较图像是通过struct `v4l2_pix_format` 中设`bytesperline` 字段实现的引入图像偏移可以通过在调VIDIOC_QBUF <VIDIOC_QBUF> 之前修改 struct
`v4l2_buffer` 中的 `m_userptr` 字段来完成。应避免这些操作，因为它们不移植（字节序问题），并且对宏块、Bayer 格式以及 mmap 缓冲区无效
选择 API 以清晰、直观且可移植的方式处理缓冲区裁合成的配置。此外，选择
API 引入了填充目标与约束标志的概念。最后，struct `v4l2_crop` struct
`v4l2_cropcap` 没有保留字段。因此无法扩展其功能。新struct
`v4l2_selection` 为未来的扩展提供了充足的空间
鼓励驱动开发者仅实现选择 API。前者（裁剪 API）将使用新的 API 模拟