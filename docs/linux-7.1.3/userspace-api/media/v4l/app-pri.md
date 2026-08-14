
######## 应用优先级


当多个应用程序共享一个设备时，可能希望为它们分配不同的优先级。与传统的
“rm -rf /”思想相反，例如视频录制应用程序可以阻止其他应用程序更改视频控制
或切换当前的电视频道。另一个目标是允许在后台运行的低优先级应用程序，它们
可以被用户控制的应用程序抢占，并在稍后自动重新获得对设备的控制。

由于这些特性无法完全在用户空间实现，V4L2 定义了 VIDIOC_G_PRIORITY
<VIDIOC_G_PRIORITY> 与 VIDIOC_S_PRIORITY <VIDIOC_G_PRIORITY> ioctl 来请求
并查询与文件描述符关联的访问优先级。打开设备会分配一个中等优先级，与不支持
这些 ioctl 的早期版本 V4L2 及驱动兼容。需要不同优先级的应用程序通常会在用
VIDIOC_QUERYCAP ioctl 验证设备后调用 :ref:`VIDIOC_S_PRIORITY
<VIDIOC_G_PRIORITY>`。

更改驱动属性的 ioctl（例如 VIDIOC_S_INPUT <VIDIOC_G_INPUT>）在另一个
应用程序获得更高优先级后，会返回 `EBUSY` 错误码。
