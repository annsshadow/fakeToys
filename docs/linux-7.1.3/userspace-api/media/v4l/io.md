


# 输入/输出


V4L2 API 定义了多种从设备读取或写入设备的方法。
所有与应用交换数据的驱动必须至少支持其中
一种。

使用 `read()` 和
`write()` 函数的经典 I/O 方法会在打开
V4L2 设备后自动选中。当驱动不支持该方法时，任何
读取或写入尝试都会随时失败。

其他方法必须经过协商。要选择使用内存映射或用户缓冲区的
流式 I/O 方法，应用程序需调用
VIDIOC_REQBUFS ioctl。

视频叠加可视为另一种 I/O 方法，尽管
应用并不直接接收图像数据。它通过
使用 VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl 启动视频叠加来选择。更多信息见 overlay。

通常每个文件描述符恰好关联一种 I/O 方法（包括叠加）。
唯一的例外是不与驱动交换数据的应用
（"面板应用"，见 open），以及为兼容 V4L 和早期
V4L2 版本而允许使用同一文件描述符同时进行视频采集与叠加的驱动。

VIDIOC_S_FMT <VIDIOC_G_FMT> 和 VIDIOC_REQBUFS 在某种程度上允许这样做，
但为简单起见，驱动无需支持切换 I/O
方法（在首次从 read/write 切换离开后），只能通过
关闭并重新打开设备来实现。

以下各节更详细地描述了各种 I/O 方法。

- [rw]](rw)
- [mmap]](mmap)
- [userp]](userp)
- [dmabuf]](dmabuf)
- [buffer]](buffer)
- [field-order]](field-order)
