######## 简介


libv4l 是一组库，它在 video4linux2 设备之上叠加了一层轻量的抽象层。这一（轻量）层的目的是让应用程序编写者能够轻松支持种类繁多的设备，而无需为同一类别中不同设备编写单独的代码。

使用 libv4l 的示例由 v4l2grab <v4l2grab-example> 提供。

libv4l 由 3 个不同的库组成：

## libv4lconvert


libv4lconvert 是一个库，它把 V4L2 驱动中存在的若干种像素格式转换为少数几种常见的 RGB 和 YUY 格式。

它目前接受以下 V4L2 驱动格式：
V4L2_PIX_FMT_BGR24 <V4L2-PIX-FMT-BGR24>、
V4L2_PIX_FMT_NV12_16L16 <V4L2-PIX-FMT-NV12-16L16>、
V4L2_PIX_FMT_JPEG <V4L2-PIX-FMT-JPEG>、
V4L2_PIX_FMT_MJPEG <V4L2-PIX-FMT-MJPEG>、
V4L2_PIX_FMT_MR97310A <V4L2-PIX-FMT-MR97310A>、
V4L2_PIX_FMT_OV511 <V4L2-PIX-FMT-OV511>、
V4L2_PIX_FMT_OV518 <V4L2-PIX-FMT-OV518>、
V4L2_PIX_FMT_PAC207 <V4L2-PIX-FMT-PAC207>、
V4L2_PIX_FMT_PJPG <V4L2-PIX-FMT-PJPG>、
V4L2_PIX_FMT_RGB24 <V4L2-PIX-FMT-RGB24>、
V4L2_PIX_FMT_SBGGR8 <V4L2-PIX-FMT-SBGGR8>、
V4L2_PIX_FMT_SGBRG8 <V4L2-PIX-FMT-SGBRG8>、
V4L2_PIX_FMT_SGRBG8 <V4L2-PIX-FMT-SGRBG8>、
V4L2_PIX_FMT_SN9C10X <V4L2-PIX-FMT-SN9C10X>、
V4L2_PIX_FMT_SN9C20X_I420 <V4L2-PIX-FMT-SN9C20X-I420>、
V4L2_PIX_FMT_SPCA501 <V4L2-PIX-FMT-SPCA501>、
V4L2_PIX_FMT_SPCA505 <V4L2-PIX-FMT-SPCA505>、
V4L2_PIX_FMT_SPCA508 <V4L2-PIX-FMT-SPCA508>、
V4L2_PIX_FMT_SPCA561 <V4L2-PIX-FMT-SPCA561>、
V4L2_PIX_FMT_SQ905C <V4L2-PIX-FMT-SQ905C>、
V4L2_PIX_FMT_SRGGB8 <V4L2-PIX-FMT-SRGGB8>、
V4L2_PIX_FMT_UYVY <V4L2-PIX-FMT-UYVY>、
V4L2_PIX_FMT_YUV420 <V4L2-PIX-FMT-YUV420>、
V4L2_PIX_FMT_YUYV <V4L2-PIX-FMT-YUYV>、
V4L2_PIX_FMT_YVU420 <V4L2-PIX-FMT-YVU420>，以及
V4L2_PIX_FMT_YVYU <V4L2-PIX-FMT-YVYU>。

后来 libv4lconvert 又被扩展为能够执行多种视频处理功能以提升摄像头视频质量。视频处理被拆分为 2 个部分：libv4lconvert/control 和 libv4lconvert/processing。

控制部分用于提供视频控制项，这些控制项可用于控制由 libv4lconvert/processing 提供的视频处理功能。这些控制项借助一个持久的共享内存对象在应用程序范围内（直至重启）保存。

libv4lconvert/processing 提供实际的视频处理功能。

## libv4l1


本库提供的函数可用于让 v4l1 应用程序快速适用于 v4l2 设备。这些函数的行为与普通的 open/close 等完全一致，区别在于 libv4l1 在 v4l2 驱动之上对 v4l1 api 做了完整模拟；而对于 v4l1 驱动，它只是将调用直接透传。

由于这些函数是对旧 V4L1 API 的模拟，不应在新应用程序中使用。

## libv4l2


本库应当用于所有现代 V4L2 应用程序。

它提供句柄以调用 V4L2 的 open/ioctl/close/poll 方法。它不只是提供设备的原始输出，而是增强这些调用：它会使用 libv4lconvert 来提供更多视频格式并提升图像质量。

在大多数情况下，libv4l2 只是将调用直接透传给 v4l2 驱动，并拦截对 VIDIOC_TRY_FMT <VIDIOC_G_FMT>、VIDIOC_G_FMT <VIDIOC_G_FMT>、VIDIOC_S_FMT <VIDIOC_G_FMT>、VIDIOC_ENUM_FRAMESIZES <VIDIOC_ENUM_FRAMESIZES> 和 VIDIOC_ENUM_FRAMEINTERVALS <VIDIOC_ENUM_FRAMEINTERVALS> 的调用，以便在驱动不支持时模拟 V4L2_PIX_FMT_BGR24 <V4L2-PIX-FMT-BGR24>、V4L2_PIX_FMT_RGB24 <V4L2-PIX-FMT-RGB24>、V4L2_PIX_FMT_YUV420 <V4L2-PIX-FMT-YUV420> 和 V4L2_PIX_FMT_YVU420 <V4L2-PIX-FMT-YVU420> 等格式。VIDIOC_ENUM_FMT <VIDIOC_ENUM_FMT> 会持续枚举硬件支持的格式，并在最后附上 libv4l 提供的模拟格式。


### Libv4l 设备控制函数


通用的文件操作方法由 libv4l 提供。

这些函数的行为类似于 gcc 的 `dup()` 函数以及 V4L2 函数 `open()`、`close()`、`ioctl()`、`read()`、`mmap()` 和 `munmap()`：


   行为类似于 `open()` 函数。


   行为类似于 `close()` 函数。


   行为类似于 libc 的 `dup()` 函数，复制一个文件句柄。


   行为类似于 `ioctl()` 函数。


   行为类似于 `read()` 函数。


   行为类似于 `mmap()` 函数。


   行为类似于 `munmap()` 函数。

这些函数提供额外的控制：


   打开一个已经打开的 fd，以便通过 v4l2lib 进一步使用，并可能通过 `v4l2_flags` 参数修改 libv4l2 的默认行为。目前 `v4l2_flags` 可以为 `V4L2_DISABLE_CONVERSION`，用于禁用格式转换。


   本函数接受一个 0–65535 的值，然后将该范围缩放到给定 v4l control id 的实际范围，如果该 cid 存在且未被锁定，则将其设置为缩放后的值。


   本函数返回一个 0–65535 的值，该值由给定 v4l control id 的实际范围缩放而来。当该 cid 不存在、由于某种原因无法访问或发生某种错误时，返回 0。

## v4l1compat.so 包装库


本库拦截对 `open()`、`close()`、`ioctl()`、`mmap()` 和 `munmap()` 操作的调用，并通过 `LD_PRELOAD=/usr/lib/v4l1compat.so` 将它们重定向到 libv4l 的对应实现。它还通过 V4L2 API 模拟 V4L1 调用。

它允许仍在使用、但未使用 libv4l 的二进制遗留应用程序继续运行。
