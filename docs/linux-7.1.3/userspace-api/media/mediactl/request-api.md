

## 请求 API（Request API

请求 API 的设计目的是V4L2 能够处理现代设备（无状态编解码器、复杂的摄像头流水线……）
API（Android Codec v2）的需求。其中一个需求是，属于同一流水线的设备能够重新配置并在逐帧的基础上紧密协作。另一个需求是对无状态编解码器的支持，它们需要将控制应用特定的帧（即"逐帧控制"）才能被高效使用
虽然最初的用例V4L2，但只要其他子系统也使用媒体控制器，它也可以扩展到其他子系统
如果不使用请API，要支持这些特性并不总是可行；而即便可行，也极其低效：用户空间必须
冲刷媒体流水线上的所有活动，为下一帧重新配置它，将与那个配置一起排队的缓冲区送入
处理，并等到它们全部可用于出队后才考虑下一帧。这违背了拥有缓冲区队列的初衷，因为实际
上一次只会有一个缓冲区被排队
请求 API 允许将流水线的特定配置（媒体控制器拓+ 每个媒体实体的配置）与特定的缓冲关联起来。这允许用户空间提前调度多个具有不同配置请求"（任务），并知道该配置将需要时应用以获得预期的结果。请求完成时的配置值也可供读取
### 一般用

请求 API 扩展了媒体控制器 API，并与特定于子系统的 API 协作以支持请求的使用。在媒体
控制器层面，请求是从支持请求的媒体控制器设备节点分配的。它们的生命周期随后通过请求
文件描述符以一种不透明的方式管理。存储在请求中的配置数据、缓冲区句柄和处理结果，通过
为请求支持而扩展的特定于子系统API（例如接受显`request_fd` 参数V4L2 API）来
访问
### 请求分配


用户空间使用 MEDIA_IOC_REQUEST_ALLOC 为媒体设备节点分配请求。这会返回一个代表该请求文件描述符。通常，会分配多个这样的请求
### 请求准备


标准V4L2 ioctl 随后可以接收一个请求文件描述符，以表达ioctl 属于上述请求、而不要立即应用这一事实。关于支持此方式ioctl 列表，请参阅 MEDIA_IOC_REQUEST_ALLOC。以
`request_fd` 参数设置的配置会被存储，而不是立即应用，排队到请求的缓冲区在请求本身排队之前不会进入常规缓冲区队列
### 请求提交


一旦指定了请求的配置和缓冲区，就可以通过在请求文件描述符上调MEDIA_REQUEST_IOC_QUEUE
来将其排队。一个请求必须至少包含一个缓冲区，否则返`ENOENT`。一个已排队的请求不能再
被修改
   对于内存到内存设<mem2mem>，你只能对输出（output）缓冲区使用请求，而不能对捕获
   （capture）缓冲区使用。试图向请求添加捕获缓冲区将导致 `EBADR` 错误
如果一个请求包含多个实体的配置，各个驱动可能会进行同步，以便在缓冲区被处理之前应用所
请求的流水线拓扑。媒体控制器驱动会尽力实现，因为由于硬件限制，完美的原子性可能无做到
   不允许将排队请求与直接排队缓冲区混用：无论先使用哪种方式，都会将其锁定，直到调用
   VIDIOC_STREAMOFF <VIDIOC_STREAMON> 或设备被关闭 <func-close>。如果之前通过请求
   排队的缓冲区，又试图直接排队一个缓冲区，或反之，都将导`EBUSY` 错误
仍然可以不带请求地设置控制，并且会立即应用，无论是否在使用请求
   通过请求和直接方式设置同一个控制可能导致未定义的行为！

用户空间可以对请求文件描述符调用 `poll()` 以等待请求完成。一旦所有关联的缓冲区都可用出队、并且所有关联的控制都已用完成时的值更新，该请求即视为完成。注意，用户空间无需
等待请求完成即可出队其缓冲区：在请求进行到一半时就可用的缓冲区可以独立于请求的状态被
出队
一个已完成的请求包含请求执行后设备的状态。用户空间可以通过使用请求文件描述符调ioctl VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 来查询该状态。对一个已排队但尚未完成的
请求调用 ioctl VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 将返`EBUSY`，因为控制值可能在
请求运行期间被驱动随时改变
### 回收与销

最后，一个已完成的请求可以被丢弃或重新使用。在请求文件描述符上调用 `close()` 会使文件描述符不可用，并且一旦内核不再使用时该请求就会被释放。也就是说，如果请求被排然后文件描述符被关闭，那么它要等到驱动完成该请求后才会被释放
MEDIA_REQUEST_IOC_REINIT 会清除请求的状态并使其再次可用。此操作不保留任何状态：该请就像刚刚被分配时一样
### 编解码器设备示例


对于编解码器 <mem2mem> 这样的用例，请求 API 可用于将特定的控制关联到由驱动为 OUTPUT
缓冲区应用，允许用户空间提前排队许多这样的缓冲区。它也可以利用请求在请求完成时捕控制状态的能力，来回读可能发生变化的信息
落实到代码中，在获得一个请求后，用户空间可以将控制和一OUTPUT 缓冲区分配给它：


	struct v4l2_buffer buf;
	struct v4l2_ext_controls ctrls;
	int req_fd;
	...
	if (ioctl(media_fd, MEDIA_IOC_REQUEST_ALLOC, &req_fd))
		return errno;
	...
	ctrls.which = V4L2_CTRL_WHICH_REQUEST_VAL;
	ctrls.request_fd = req_fd;
	if (ioctl(codec_fd, VIDIOC_S_EXT_CTRLS, &ctrls))
		return errno;
	...
	buf.type = V4L2_BUF_TYPE_VIDEO_OUTPUT;
	buf.flags |= V4L2_BUF_FLAG_REQUEST_FD;
	buf.request_fd = req_fd;
	if (ioctl(codec_fd, VIDIOC_QBUF, &buf))
		return errno;

注意，不允许CAPTURE 缓冲区使用请API，因为那里没有需要报告的逐帧设置
一旦请求完全准备好，它就可以被排队到驱动：


	if (ioctl(req_fd, MEDIA_REQUEST_IOC_QUEUE))
		return errno;

用户空间随后可以通过在其文件描述符上调用 poll() 来等待请求完成，或者开始出CAPTURE
缓冲区。很可能它希望尽快获CAPTURE 缓冲区，这可以通过常规VIDIOC_DQBUF
<VIDIOC_QBUF> 来完成：


	struct v4l2_buffer buf;

	memset(&buf, 0, sizeof(buf));
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
	if (ioctl(codec_fd, VIDIOC_DQBUF, &buf))
		return errno;

注意，为简单起见，此示例假设每OUTPUT 缓冲区对应一CAPTURE 缓冲区，但实际情况未如此
然后，在通过轮询请求文件描述符确保其完成后，我们可以在它完成时通过调用
VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 查询控制值。这对于易变（volatile）控制特别有用，
我们希望在捕获缓冲区一产生时就查询其值

	struct pollfd pfd = { .events = POLLPRI, .fd = req_fd };
	poll(&pfd, 1, -1);
	...
	ctrls.which = V4L2_CTRL_WHICH_REQUEST_VAL;
	ctrls.request_fd = req_fd;
	if (ioctl(codec_fd, VIDIOC_G_EXT_CTRLS, &ctrls))
		return errno;

一旦我们不再需要该请求，可以用 MEDIA_REQUEST_IOC_REINIT 回收它以供重用…

	if (ioctl(req_fd, MEDIA_REQUEST_IOC_REINIT))
		return errno;

…或者关闭它的文件描述符以彻底释放它

	close(req_fd);

### 简单捕获设备示

对于简单的捕获设备，请求可用于为给定的 CAPTURE 缓冲区指定要应用的控制

	struct v4l2_buffer buf;
	struct v4l2_ext_controls ctrls;
	int req_fd;
	...
	if (ioctl(media_fd, MEDIA_IOC_REQUEST_ALLOC, &req_fd))
		return errno;
	...
	ctrls.which = V4L2_CTRL_WHICH_REQUEST_VAL;
	ctrls.request_fd = req_fd;
	if (ioctl(camera_fd, VIDIOC_S_EXT_CTRLS, &ctrls))
		return errno;
	...
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
	buf.flags |= V4L2_BUF_FLAG_REQUEST_FD;
	buf.request_fd = req_fd;
	if (ioctl(camera_fd, VIDIOC_QBUF, &buf))
		return errno;

一旦请求完全准备好，它就可以被排队到驱动：


	if (ioctl(req_fd, MEDIA_REQUEST_IOC_QUEUE))
		return errno;

用户空间随后可以出队缓冲区、等待请求完成、查询控制并回收请求，如上面M2M 示例所示