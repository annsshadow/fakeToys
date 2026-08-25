

######## 流式 I/O（DMA 缓冲区导入）


DMABUF 框架提供了一种在多个设备之间共享缓冲区的通用方法。支DMABUF 的设备驱可以将一DMA 缓冲区作为文件描述符导出到用户态（称为 exporter，导出者角色）使用先前为不同或同一设备导出的文件描述符从用户态导入一DMA 缓冲区（称为
importer，导入者角色），或同时支持两者。本节描V4L2 中的 DMABUF 导入者角API
关于V4L2 缓冲区导出为 DMABUF 文件描述符的细节，请参见 DMABUF 导出
<VIDIOC_EXPBUF>銆。
当由 VIDIOC_QUERYCAP <VIDIOC_QUERYCAP> ioctl 返回struct `v4l2_capability`
`capabilities` 字段中的 `V4L2_CAP_STREAMING` 标志被置位时，输入和输出设备
支持流式 I/O 方法。是否支持通过 DMABUF 文件描述符导DMA 缓冲区，则由以内存类设置`V4L2_MEMORY_DMABUF` 调用 VIDIOC_REQBUFS <VIDIOC_REQBUFS> ioctl 来确定
I/O 方法专用于在不同设备之间共享 DMA 缓冲区，这些设备可以V4L 设备或其视频相关设备（如 DRM）。缓冲区（平面）由驱动代表应用程序分配。接着，这些缓冲区
通过分配器驱动特定的 API 作为文件描述符导出给应用程序。只有这样的文件描述符被
交换。描述符和元信息struct `v4l2_buffer`（或对于多平API 情形struct
`v4l2_plane`）中传递。必须通过以期望的缓冲区类型调VIDIOC_REQBUFS
<VIDIOC_REQBUFS> 将驱动切换到 DMABUF I/O 模式
## 示例：使DMABUF 文件描述符发起流I/O



    struct v4l2_requestbuffers reqbuf;

    memset(&reqbuf, 0, sizeof (reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_DMABUF;
    reqbuf.count = 1;

    if (ioctl(fd, VIDIOC_REQBUFS, &reqbuf) == -1) {
	if (errno == EINVAL)
	    printf("Video capturing or DMABUF streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

缓冲区（平面）文件描述符VIDIOC_QBUF <VIDIOC_QBUF> ioctl 即时传入。对于多平面
缓冲区，每个平面都可以关联一个不同的 DMABUF 描述符。尽管缓冲区通常被循环使用，
但应用程序也可以在每VIDIOC_QBUF <VIDIOC_QBUF> 调用时传入不同的 DMABUF 描述符
## 示例：使用单平面 API DMABUF 入队



    int buffer_queue(int v4lfd, int index, int dmafd)
    {
	struct v4l2_buffer buf;

	memset(&buf, 0, sizeof buf);
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
	buf.memory = V4L2_MEMORY_DMABUF;
	buf.index = index;
	buf.m.fd = dmafd;

	if (ioctl(v4lfd, VIDIOC_QBUF, &buf) == -1) {
	    perror("VIDIOC_QBUF");
	    return -1;
	}

	return 0;
    }

## 示例 3.6. 使用多平API DMABUF 入队



    int buffer_queue_mp(int v4lfd, int index, int dmafd[], int n_planes)
    {
	struct v4l2_buffer buf;
	struct v4l2_plane planes[VIDEO_MAX_PLANES];
	int i;

	memset(&buf, 0, sizeof buf);
	buf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
	buf.memory = V4L2_MEMORY_DMABUF;
	buf.index = index;
	buf.m.planes = planes;
	buf.length = n_planes;

	memset(&planes, 0, sizeof planes);

	for (i = 0; i < n_planes; ++i)
	    buf.m.planes[i].m.fd = dmafd[i];

	if (ioctl(v4lfd, VIDIOC_QBUF, &buf) == -1) {
	    perror("VIDIOC_QBUF");
	    return -1;
	}

	return 0;
    }

捕获或显示的缓冲区通过 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 出队。驱动可以在 DMA
完成与此 ioctl 之间的任意时刻解锁该缓冲区。当调用 VIDIOC_STREAMOFF
<VIDIOC_STREAMON>、VIDIOC_REQBUFS <VIDIOC_REQBUFS>，或设备被关闭时，内存也会被
解锁
对于捕获类应用程序，通常的做法是预先入队若干空缓冲区，然后启动捕获并进入读取
循环。在此循环中，应用程序等待直到有已填充的缓冲区可以出队，并在数据不再需要时
重新入队该缓冲区。输出类应用程序则填充并入队缓冲区，当累积了足够多的缓冲区后
开始输出。在写入循环中，当应用程序耗尽空闲缓冲区时，它必须等待直到有空缓冲可以出队并复用。存在两种方法可挂起应用程序的执行，直到有一个或多个缓冲区可出队。默认情况下，当出队队列中没有缓冲区:ref:`VIDIOC_DQBUF <VIDIOC_QBUF>`
会阻塞。当`open()` 函数传入`O_NONBLOCK` 标志时，VIDIOC_DQBUF
<VIDIOC_QBUF> 在没有可用缓冲区时会立即返回 `EAGAIN` 错误码。`select()` `poll()` 函数始终可用
要启动和停止捕获或显示类应用程序，请调用 VIDIOC_STREAMON <VIDIOC_STREAMON> VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctls

   VIDIOC_STREAMOFF <VIDIOC_STREAMON> 作为副作用会从两个队列中移除所有缓冲区
   并解锁所有缓冲区。由于在多任务系统上没有"立即"执行某事的概念，如果应用程序
   需要与其他事件同步，它应当检查已捕获或已输出缓冲区的 struct `v4l2_buffer`
   `timestamp`（时间戳）
实现 DMABUF 导入 I/O 的驱动必须支VIDIOC_REQBUFS <VIDIOC_REQBUFS>VIDIOC_QBUF <VIDIOC_QBUF>、VIDIOC_DQBUF <VIDIOC_DQBUF>ref:`VIDIOC_STREAMON
<VIDIOC_STREAMON>` VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctls，以`select()` `poll()` 函数