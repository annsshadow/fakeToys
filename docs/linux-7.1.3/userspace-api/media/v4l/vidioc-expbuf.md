


######## ioctl VIDIOC_EXPBUF


## 名称


VIDIOC_EXPBUF - 将一个缓冲区导出为 DMABUF 文件描述符。

## 概要


`int ioctl(int fd, VIDIOC_EXPBUF, struct v4l2_exportbuffer *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_exportbuffer` 的指针。

## 描述


该 ioctl 是内存映射 <mmap> I/O 方法的扩展，因此仅对 `V4L2_MEMORY_MMAP`
缓冲区可用。它可以在使用 VIDIOC_REQBUFS ioctl 分配完缓冲区之后的任何时刻，
将缓冲区导出为一个 DMABUF 文件。

要导出缓冲区，应用程序需要填写 struct `v4l2_exportbuffer`。`type` 字段应设为
与之前使用 struct `v4l2_requestbuffers` 的 `type` 相同的缓冲区类型。应用程序
还必须设置 `index` 字段。有效的索引号范围从零到用 VIDIOC_REQBUFS（struct
`v4l2_requestbuffers` 的 `count`）分配的缓冲区数量减一。对于多平面（multi-planar）
API，应用程序将 `plane` 字段设为要导出的平面索引。有效平面范围从零到当前活动
格式支持的最大有效平面数。对于单平面（single-planar）API，应用程序必须将
`plane` 设为零。可以在 `flags` 字段中设置额外的标志，具体细节参见 open() 的
手册。目前仅支持 O_CLOEXEC、O_RDONLY、O_WRONLY 和 O_RDWR。所有其它字段必须
设为零。对于多平面 API，每个平面都使用多次 VIDIOC_EXPBUF 调用来分别导出。

调用 VIDIOC_EXPBUF 后，`fd` 字段会被驱动设置。这是一个 DMABUF 文件描述符。
应用程序可将其传递给其它支持 DMABUF 的设备。关于将 DMABUF 文件导入 V4L2 节点的
细节，请参考 DMABUF importing <dmabuf>。建议在不再使用某个 DMABUF 文件时关闭它，
以便回收相关的内存。

## 示例



    int buffer_export(int v4lfd, enum v4l2_buf_type bt, int index, int *dmafd)
    {
	struct v4l2_exportbuffer expbuf;

	memset(&expbuf, 0, sizeof(expbuf));
	expbuf.type = bt;
	expbuf.index = index;
	if (ioctl(v4lfd, VIDIOC_EXPBUF, &expbuf) == -1) {
	    perror("VIDIOC_EXPBUF");
	    return -1;
	}

	*dmafd = expbuf.fd;

	return 0;
    }


    int buffer_export_mp(int v4lfd, enum v4l2_buf_type bt, int index,
	int dmafd[], int n_planes)
    {
	int i;

	for (i = 0; i < n_planes; ++i) {
	    struct v4l2_exportbuffer expbuf;

	    memset(&expbuf, 0, sizeof(expbuf));
	    expbuf.type = bt;
	    expbuf.index = index;
	    expbuf.plane = i;
	    if (ioctl(v4lfd, VIDIOC_EXPBUF, &expbuf) == -1) {
		perror("VIDIOC_EXPBUF");
		while (i)
		    close(dmafd[--i]);
		return -1;
	    }
	    dmafd[i] = expbuf.fd;
	}

	return 0;
    }



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 缓冲区的类型，与 struct `v4l2_format` 的 `type` 或 struct
	`v4l2_requestbuffers` 的 `type` 相同，由应用程序设置。参见 `v4l2_buf_type`
    - - __u32
      - `index`
      - 缓冲区的编号，由应用程序设置。该字段仅用于内存映射 <mmap> I/O，
	范围可从零到由 VIDIOC_REQBUFS 和/或 VIDIOC_CREATE_BUFS ioctl 分配的
	缓冲区数量。
    - - __u32
      - `plane`
      - 使用多平面 API 时要导出的平面索引。否则该值必须设为零。
    - - __u32
      - `flags`
      - 新创建文件的标志，目前仅支持 `O_CLOEXEC`、`O_RDONLY`、`O_WRONLY`
	和 `O_RDWR`，更多细节请参考 open() 的手册。
    - - __s32
      - `fd`
      - 与缓冲区关联的 DMABUF 文件描述符。由驱动设置。
    - - __u32
      - `reserved[^11^]`
      - 保留字段，供将来使用。驱动和应用程序必须将该数组设为零。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    队列不处于 MMAP 模式，或是不支持 DMABUF 导出，或者 `flags`、`type`、
    `index` 或 `plane` 字段无效。
