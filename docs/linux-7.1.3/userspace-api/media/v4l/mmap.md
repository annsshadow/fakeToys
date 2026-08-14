
######## 流 I/O（内存映射，Memory Mapping）


当由 VIDIOC_QUERYCAP ioctl 返回的 struct `v4l2_capability` 的 `capabilities`
字段中的 `V4L2_CAP_STREAMING` 标志被置位时，输入和输出设备支持这种 I/O 方法。
有两类流方法，要确定是否支持内存映射这种方式，应用程序必须将内存类型设为
`V4L2_MEMORY_MMAP` 来调用 VIDIOC_REQBUFS ioctl。

流（Streaming）是一种 I/O 方法，其中应用程序与驱动之间只交换指向缓冲区的指针，数据
本身不被复制。内存映射主要旨在把设备内存中的缓冲区映射到应用程序的地址空间。设备
内存可以是例如带有视频采集附加卡的显卡上的视频内存。不过，作为长期以来最高效的 I/O
方法，许多其他驱动也支持流，在可 DMA 的主内存中分配缓冲区。

一个驱动可以支持多组缓冲区。每组由一个唯一的缓冲区类型值标识。这些组是相互独立的，
每组可以持有不同类型的数据。要同时访问不同的组，必须使用不同的文件描述符。[#f1]_

要分配设备缓冲区，应用程序调用 VIDIOC_REQBUFS ioctl，并传入期望的缓冲区数量和缓冲区
类型，例如 `V4L2_BUF_TYPE_VIDEO_CAPTURE`。只要没有任何缓冲区仍处于映射状态，这个 ioctl
也可以用来改变缓冲区数量或释放已分配的内存。

在应用程序能够访问这些缓冲区之前，它们必须用 `mmap()` 函数映射到自己的地址空间。缓冲
区在设备内存中的位置可以通过 VIDIOC_QUERYBUF ioctl 确定。在单平面（single-planar）
API 的情况下，struct `v4l2_buffer` 中返回的 `m.offset` 和 `length` 作为第六个和第二个
参数传给 `mmap()` 函数。当使用多平面（multi-planar）API 时，struct `v4l2_buffer` 包含
一个 struct `v4l2_plane` 结构体数组，每个结构体都包含自己的 `m.offset` 和 `length`。当
使用多平面 API 时，每个缓冲区的每个平面都必须分别映射，因此对 `mmap()` 的调用次数应当
等于缓冲区数量乘以每个缓冲区中的平面数量。offset 和 length 值不得被修改。请记住，缓冲区
分配在物理内存中，而非可以被换出到磁盘的虚拟内存中。应用程序应当尽快用 `munmap()`
函数释放这些缓冲区。

## 示例：在单平面 API 中映射缓冲区


    struct v4l2_requestbuffers reqbuf;
    struct {
	void *start;
	size_t length;
    } *buffers;
    unsigned int i;

    memset(&reqbuf, 0, sizeof(reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_MMAP;
    reqbuf.count = 20;

    if (-1 == ioctl (fd, VIDIOC_REQBUFS, &reqbuf)) {
	if (errno == EINVAL)
	    printf("Video capturing or mmap-streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

    /** We want at least five buffers. **/

    if (reqbuf.count < 5) {
	/** You may need to free the buffers here. **/
	printf("Not enough buffer memory\\n");
	exit(EXIT_FAILURE);
    }

    buffers = calloc(reqbuf.count, sizeof(*buffers));
    assert(buffers != NULL);

    for (i = 0; i < reqbuf.count; i++) {
	struct v4l2_buffer buffer;

	memset(&buffer, 0, sizeof(buffer));
	buffer.type = reqbuf.type;
	buffer.memory = V4L2_MEMORY_MMAP;
	buffer.index = i;

	if (-1 == ioctl (fd, VIDIOC_QUERYBUF, &buffer)) {
	    perror("VIDIOC_QUERYBUF");
	    exit(EXIT_FAILURE);
	}

	buffers[i].length = buffer.length; /** remember for munmap() **/

	buffers[i].start = mmap(NULL, buffer.length,
		    PROT_READ | PROT_WRITE, /** recommended **/
		    MAP_SHARED,             /** recommended **/
		    fd, buffer.m.offset);

	if (MAP_FAILED == buffers[i].start) {
	    /* If you do not exit here you should unmap() and free()
	       the buffers mapped so far. */
	    perror("mmap");
	    exit(EXIT_FAILURE);
	}
    }

    /** Cleanup. **/

    for (i = 0; i < reqbuf.count; i++)
	munmap(buffers[i].start, buffers[i].length);

## 示例：在多平面 API 中映射缓冲区


    struct v4l2_requestbuffers reqbuf;
    /** Our current format uses 3 planes per buffer **/
    #define FMT_NUM_PLANES = 3

    struct {
	void *start[FMT_NUM_PLANES];
	size_t length[FMT_NUM_PLANES];
    } *buffers;
    unsigned int i, j;

    memset(&reqbuf, 0, sizeof(reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE;
    reqbuf.memory = V4L2_MEMORY_MMAP;
    reqbuf.count = 20;

    if (ioctl(fd, VIDIOC_REQBUFS, &reqbuf) < 0) {
	if (errno == EINVAL)
	    printf("Video capturing or mmap-streaming is not supported\\n");
	else
	    perror("VIDIOC_REQBUFS");

	exit(EXIT_FAILURE);
    }

    /** We want at least five buffers. **/

    if (reqbuf.count < 5) {
	/** You may need to free the buffers here. **/
	printf("Not enough buffer memory\\n");
	exit(EXIT_FAILURE);
    }

    buffers = calloc(reqbuf.count, sizeof(*buffers));
    assert(buffers != NULL);

    for (i = 0; i < reqbuf.count; i++) {
	struct v4l2_buffer buffer;
	struct v4l2_plane planes[FMT_NUM_PLANES];

	memset(&buffer, 0, sizeof(buffer));
	buffer.type = reqbuf.type;
	buffer.memory = V4L2_MEMORY_MMAP;
	buffer.index = i;
	/* length in struct v4l2_buffer in multi-planar API stores the size
  - of planes array. */
	buffer.length = FMT_NUM_PLANES;
	buffer.m.planes = planes;

	if (ioctl(fd, VIDIOC_QUERYBUF, &buffer) < 0) {
	    perror("VIDIOC_QUERYBUF");
	    exit(EXIT_FAILURE);
	}

	/** Every plane has to be mapped separately **/
	for (j = 0; j < FMT_NUM_PLANES; j++) {
	    buffers[i].length[j] = buffer.m.planes[j].length; /** remember for munmap() **/

	    buffers[i].start[j] = mmap(NULL, buffer.m.planes[j].length,
		     PROT_READ | PROT_WRITE, /** recommended **/
		     MAP_SHARED,             /** recommended **/
		     fd, buffer.m.planes[j].m.mem_offset);

	    if (MAP_FAILED == buffers[i].start[j]) {
		/* If you do not exit here you should unmap() and free()
		   the buffers and planes mapped so far. */
		perror("mmap");
		exit(EXIT_FAILURE);
	    }
	}
    }

    /** Cleanup. **/

    for (i = 0; i < reqbuf.count; i++)
	for (j = 0; j < FMT_NUM_PLANES; j++)
	    munmap(buffers[i].start[j], buffers[i].length[j]);

从概念上讲，流驱动维护两个缓冲区队列：一个传入队列和一个传出队列。它们把锁定到视频
时钟的同步采集或输出操作，与可能受到随机磁盘或网络延迟以及其他进程抢占影响的应用程序
分离开来，从而降低了数据丢失的概率。队列以 FIFO 方式组织，缓冲区将按照它们在传入 FIFO
中入队的顺序输出，并且是在从传出 FIFO 出队时的顺序被采集的。

驱动可能要求在任何时刻都至少有最少数量的缓冲区入队才能工作，除此之外，对应用程序可以
提前入队、或出队并处理的缓冲区数量没有限制。它们也可以按与缓冲区出队不同的顺序入队，
而驱动可以以**任意**顺序**填充**已入队的**空**缓冲区。[#f2]_ 缓冲区的索引号（struct
`v4l2_buffer` 的 `index`）在这里不起作用，它只是用于标识缓冲区。

最初，所有已映射的缓冲区都处于出队状态，驱动无法访问。对于采集类应用程序，习惯上先
把所有的已映射缓冲区入队，然后开始采集并进入读取循环。在这里应用程序等待，直到一个
已填充的缓冲区可以被出队，并在数据不再需要时重新入队该缓冲区。输出类应用程序填充并
入队缓冲区，当堆积了足够的缓冲区后，用 VIDIOC_STREAMON <VIDIOC_STREAMON> 开始输出。在
写入循环中，当应用程序用光空闲缓冲区时，它必须等待，直到一个空缓冲区可以被出队并复用。

要入队和出队一个缓冲区，应用程序使用 VIDIOC_QBUF <VIDIOC_QBUF> 和 VIDIOC_DQBUF
<VIDIOC_QBUF> ioctl。一个缓冲区处于已映射、已入队、已满或已空的状态，在任何时候都可以通过
VIDIOC_QUERYBUF ioctl 确定。存在两种方法来挂起应用程序的执行，直到一个或多个缓冲区可以
被出队。默认情况下，当没有缓冲区在传出队列中时，VIDIOC_DQBUF <VIDIOC_QBUF> 会阻塞。当
向 `open()` 函数传入了 `O_NONBLOCK` 标志时，在没有缓冲区可用时，VIDIOC_DQBUF
<VIDIOC_QBUF> 会立即返回 `EAGAIN` 错误码。`select()` 或 `poll()` 函数始终可用。

要开始和停止采集或输出，应用程序调用 VIDIOC_STREAMON <VIDIOC_STREAMON> 和
:ref:`VIDIOC_STREAMOFF <VIDIOC_STREAMON>` ioctl。

   作为副作用，它会把两个队列中的所有缓冲区都移除。由于在一个多任务系统上不存在“现在”
   就做某事的概念，如果一个应用程序需要与其他事件同步，它应当检查所采集或输出缓冲区的
   struct :`v4l2_buffer` `timestamp`。

实现内存映射 I/O 的驱动必须支持 VIDIOC_REQBUFS <VIDIOC_REQBUFS>、:ref:`VIDIOC_QUERYBUF
<VIDIOC_QUERYBUF>`、VIDIOC_QBUF <VIDIOC_QBUF>、:ref:`VIDIOC_DQBUF
<VIDIOC_QBUF>`、VIDIOC_STREAMON <VIDIOC_STREAMON> 和 VIDIOC_STREAMOFF
<VIDIOC_STREAMON> ioctl，以及 :ref:`mmap() <func-mmap>`、`munmap()`、:ref:`select()
<func-select>` 和 `poll()` 函数。[#f3]_

[采集示例]

   可以使用一个文件描述符，并在调用 VIDIOC_QBUF 等时相应地设置缓冲区类型字段，但这会让
   `select()` 函数变得含糊。我们更喜欢每个逻辑流一个文件描述符这种干净的做法。例如视频
   叠加（overlay）也是一个逻辑流，尽管连续运行并不需要 CPU。

   随机入队顺序允许乱序处理图像（例如视频编解码器）的应用程序更早地归还缓冲区，从而降低
   数据丢失的概率。随机填充顺序允许驱动基于 LIFO 复用缓冲区，利用缓存中保存的分散-聚集
   列表等。

   在驱动层面，`select()` 和 `poll()` 是相同的，而 `select()` 太重要了，不能成为可选项。
   其余的应当是不言自明的。
