
######## V4L2 poll()


## 名称


v4l2-poll - 等待文件描述符上发生某个事件

## 摘要


    #include <sys/poll.h>

## 参数


## 描述


通过 `poll()` 函数，应用程序可以挂起执行，直到驱动已捕获到数据或已准备好
接受用于输出的数据。

当已协商使用流式 I/O 时，此函数会等待直到捕获设备已填充一个缓冲区，并可由
VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 取出。对于输出设备，此函数会等待直到
设备准备好接受一个新缓冲区，由 VIDIOC_QBUF <VIDIOC_QBUF> ioctl 入队
以进行显示。当缓冲区已在驱动的出队队列中（捕获）或入队队列未满（显示）时，
该函数会立即返回。

成功时 `poll()` 返回已被选中的文件描述符数量（即相应 `struct pollfd`
结构的 `revents` 字段非零的文件描述符）。捕获设备在 `revents` 字段中
设置 `POLLIN` 与 `POLLRDNORM` 标志，输出设备设置 `POLLOUT` 与
`POLLWRNORM` 标志。当函数超时时返回 0，失败时返回 -1 并设置 `errno` 变量。
当应用程序尚未调用 VIDIOC_STREAMON <VIDIOC_STREAMON> 时，`poll()`
函数会成功，但在 `revents` 字段中设置 `POLLERR` 标志。当应用程序已为
捕获设备调用了 VIDIOC_STREAMON <VIDIOC_STREAMON>，但还没有调用
VIDIOC_QBUF <VIDIOC_QBUF> 时，`poll()` 函数会成功并在 `revents`
字段中设置 `POLLERR` 标志。对于输出设备，相同情形也会导致 `poll()` 成功，
但会在 `revents` 字段中设置 `POLLOUT` 与 `POLLWRNORM` 标志。

如果发生了事件（参见 VIDIOC_DQEVENT），则 `revents` 字段中会设置
`POLLPRI`，并且 `poll()` 会返回。

当已协商使用 `read()` 函数而驱动尚未开始捕获时，`poll()` 函数会
启动捕获。若失败，则如上返回 `POLLERR`。否则它会等待直到数据已被捕获并可读。
当驱动持续捕获时（与拍摄静态图像不同），该函数可能会立即返回。

当已协商使用 `write()` 函数而驱动尚未开始流式传输时，`poll()` 函数会
启动流式传输。若失败，则如上返回 `POLLERR`。否则它会等待直到驱动准备好进行
非阻塞的 `write()` 调用。

如果调用者只对事件感兴趣（即在 `events` 字段中仅设置了 `POLLPRI`），
那么当驱动尚未开始流式传输时，`poll()` 将**不会**启动流式传输。这使得仅轮询事件
而不轮询缓冲区成为可能。

所有实现了 `read()` 或 `write()` 函数或流式 I/O 的驱动也必须支持 `poll()`
函数。

更多细节请参见 `poll()` 手册页。

## 返回值


成功时，`poll()` 返回 `revents` 字段非零的结构数量，若调用超时则返回 0。
出错时返回 -1，并设置 `errno` 变量：

EBADF
    一个或多个 `ufds` 成员指定了无效的文件描述符。

EBUSY
    驱动不支持多个读或写流，且设备已在使用中。

EFAULT
    `ufds` 引用了不可访问的内存区域。

EINTR
    调用被信号中断。

EINVAL
    `nfds` 值超过了 `RLIMIT_NOFILE` 值。使用 `getrlimit()`
    获取该值。
