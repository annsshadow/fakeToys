
######## 搴旂敤浼樺厛绾。

当多个应用程序共享一个设备时，可能希望为它们分配不同的优先级。与传统“rm -rf /”思想相反，例如视频录制应用程序可以阻止其他应用程序更改视频控或切换当前的电视频道。另一个目标是允许在后台运行的低优先级应用程序，它可以被用户控制的应用程序抢占，并在稍后自动重新获得对设备的控制
由于这些特性无法完全在用户空间实现，V4L2 定义VIDIOC_G_PRIORITY
<VIDIOC_G_PRIORITY> VIDIOC_S_PRIORITY <VIDIOC_G_PRIORITY> ioctl 来请并查询与文件描述符关联的访问优先级。打开设备会分配一个中等优先级，与不支这些 ioctl 的早期版V4L2 及驱动兼容。需要不同优先级的应用程序通常会在VIDIOC_QUERYCAP ioctl 验证设备后调:ref:`VIDIOC_S_PRIORITY
<VIDIOC_G_PRIORITY>`銆。
更改驱动属性的 ioctl（例VIDIOC_S_INPUT <VIDIOC_G_INPUT>）在另一应用程序获得更高优先级后，会返回 `EBUSY` 错误码