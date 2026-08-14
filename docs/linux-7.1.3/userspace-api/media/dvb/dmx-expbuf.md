


######## ioctl DMX_EXPBUF


## 名称


DMX_EXPBUF - 将一个缓冲区导出为 DMABUF 文件描述符。


## 概要


`int ioctl(int fd, DMX_EXPBUF, struct dmx_exportbuffer *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `dmx_exportbuffer` 的指针。

## 描述


该 ioctl 是内存映射 I/O 方法的扩展。
它可用于在通过 DMX_REQBUFS ioctl 分配缓冲区之后的任意时刻，将一个缓冲区导出为 DMABUF 文件。

要导出一个缓冲区，应用程序需要填充 struct `dmx_exportbuffer`。
应用程序必须设置 `index` 字段。有效的索引编号范围从零到使用 DMX_REQBUFS 分配的缓冲区数量（struct `dmx_requestbuffers` 的 `count`）减一。
可以在 `flags` 字段中设置额外的标志。有关详细信息，请参考 open() 的手册页。目前仅支持 O_CLOEXEC、O_RDONLY、O_WRONLY 和 O_RDWR。
所有其他字段必须设置为零。在多平面（multi-planar）API 的情况下，每个平面都通过多次 DMX_EXPBUF 调用分别导出。

调用 DMX_EXPBUF 后，若成功，`fd` 字段将被驱动设置。这是一个 DMABUF 文件描述符。应用程序可以将其传递给其他支持 DMABUF 的设备。建议在不使用该 DMABUF 文件时将其关闭，以便回收相关的内存。

## 示例


    int buffer_export(int v4lfd, enum dmx_buf_type bt, int index, int *dmafd)
    {
	struct dmx_exportbuffer expbuf;

	memset(&expbuf, 0, sizeof(expbuf));
	expbuf.type = bt;
	expbuf.index = index;
	if (ioctl(v4lfd, DMX_EXPBUF, &expbuf) == -1) {
	    perror("DMX_EXPBUF");
	    return -1;
	}

	*dmafd = expbuf.fd;

	return 0;
    }

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用的错误码在 Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    队列不处于 MMAP 模式，或不支持 DMABUF 导出，或 `flags`、`index` 字段无效。
