## Sync File API 指南


:Author: Gustavo Padovan <gustavo at padovan dot org>

本文档作为设备驱动编写者的指南，说sync_file API 是什么，以及驱动如何支持它。Sync file 是同步（struct dma_fence）的载体，这些同步是在驱动之间或跨进程边界进行同步所必需的

sync_file API 旨在用于向用户空间发送和从用户空间接fence 信息。它使用户空间能够进行显式围栏（fencing），即生产者驱动（GPU V4L 驱动）不是将 fence 附加到缓冲区，而是通过 sync_file 将与该缓冲区相关fence 发送给用户空间

然后sync_file 可以被发送给消费者（例如 DRM 驱动），fence 发出信号之前，该消费者不会对缓冲区做任何事情——即发出 fence 的驱动不再使处理该缓冲区，因此它发出信号表示该缓冲区已可使用。对于消费-> 生产者的循环部分反之亦然

Sync file 使用户空间能够感知驱动之间缓冲区共享的同步

Sync file 最初添加于 Android 内核，但当前 Linux 桌面也能从中获益良多

### in-fence 涓?out-fence


Sync file 既可以发往用户空间，也可以来自用户空间。当 sync_file 从驱动发送到用户空间时，我们称其包含fence 为“out-fence”。它们与一个驱动正在处理或即将处理的缓冲区相关，因此驱动创建一out-fence，以便能够在它通过 dma_fence_signal() 完成使用该缓冲区（或处理完）时通知。Out-fence 是驱动创建的 fence

另一方面，如果驱动通过 sync_file 从用户空间接收到 fence，我们称这些 fence 为“in-fence”。接收到 in-fence 意味着我们需要在使用in-fence 相关的任何缓冲区之前，等待该 fence 发出信号

### 创建 Sync File


当驱动需要向用户空间发out-fence 时，它创建一sync_file

```

	struct sync_file *sync_file_create(struct dma_fence *fence);

```

调用者传out-fence，取sync_file。这只是第一步，接下来它需要在 sync_file->file 上安装一fd。因此它获取一
```

	fd = get_unused_fd_flags(O_CLOEXEC);

```

```
	fd_install(fd, sync_file->file);

```

sync_file fd 现在可以被发送给用户空间

如果创建过程失败，或者由于任何其他原因需要释sync_file，应使用 fput(sync_file->file)

### 从用户空间接Sync File


当用户空间需要向驱动发in-fence 时，它将 Sync File 的文件描述符传递给内核。内核随后可以从中检fence

```

	struct dma_fence *sync_file_get_fence(int fd);


```

返回的引用由调用者拥有，之后必须使用 dma_fence_put() 释放。在出错的情况下，返回的NULL 而非引用

参考：

1. include/linux/sync_file.h 中的 struct sync_file
2. 上述所有接口均定义include/linux/sync_file.h 
