


######## ioctl DMX_QBUF, DMX_DQBUF


## 名称


DMX_QBUF - DMX_DQBUF - 与驱动交换缓冲区


## 概要



`int ioctl(int fd, DMX_QBUF, struct dmx_buffer *argp)`


`int ioctl(int fd, DMX_DQBUF, struct dmx_buffer *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `dmx_buffer` 的指针
## 描述


应用程序调用 `DMX_QBUF` ioctl 将空的（捕获用）或已填充的（输出用）缓冲区入队到驱动的传入队列。其语义取决于所选择I/O 方法
要入队缓冲区，应用程序设`index` 字段。有效的索引编号范围从零到用 DMX_REQBUFS（struct `dmx_requestbuffers` `count`）分配的缓冲区数量减一。由 DMX_QUERYBUF ioctl 返回struct `dmx_buffer` 内容也同样适用
当使用一个指向该结构的指针调`DMX_QBUF` 时，它会将缓冲区的物理内存页锁定，使其不能被换出到磁盘。缓冲区会一直保持锁定，直到出队或设备被关闭
应用程序调用 `DMX_DQBUF` ioctl 从驱动的传出队列中取出一个已填充的（捕获用）缓冲区。它们只需用要入队的缓冲区 ID 设置 `index` 字段。当使用指向 struct `dmx_buffer` 的指针调`DMX_DQBUF` 时，驱动会填充其余字段或返回错误码
默认情况下，当传出队列中没有缓冲区时 `DMX_DQBUF` 会阻塞。当`open()` 函数传入 `O_NONBLOCK` 标志时，若没有可用缓冲区，`DMX_DQBUF` 会立即返`EAGAIN` 错误码
struct `dmx_buffer` 结构buffer 中定义
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EAGAIN
    已使`O_NONBLOCK` 选择了非阻塞 I/O，且传出队列中没有缓冲区
EINVAL
    `index` 超出范围，或尚未分配任何缓冲区
EIO
    `DMX_DQBUF` 由于内部错误而失败。也可能指示临时性问题，如信号丢失或 CRC 错误