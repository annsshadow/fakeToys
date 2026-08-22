


######## V4L2 select()


## 名称


v4l2-select - 同步 I/O 多路复用

## 概要


    #include <sys/time.h>
    #include <sys/types.h>
    #include <unistd.h>


## 参数


`nfds`
  三个集合中编号最高的文件描述符，1

`readfds`
  read() 调用不会阻塞时需监视的文件描述符

`writefds`
  write() 不会阻塞时需监视的文件描述符

`exceptfds`
  需监视 V4L2 事件的文件描述符

`timeout`
  最长等待时间

## 描述


通过 `select()` 函数，应用程序可以挂起执行，直到驱动已捕获数据或准备好接收用于输出的数据

当已协商I/O 时，该函数等待直到一个缓冲区被填充或显示，并可以使用 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 出队。当缓冲区已在驱动的传出队列中时，该函数立即返回

成功`select()` 返回`fd_set` 中设置的位的总数。当函数超时时返回值为零。失败时返回 -1，并相应地设`errno` 变量。当应用程序尚未调用 VIDIOC_QBUF VIDIOC_STREAMON 时，`select()` 函数会成功，设置 `readfds` `writefds` 中文件描述符的位，但后续VIDIOC_DQBUF <VIDIOC_QBUF> 调用将失败。[#f1]_

当已协商使用 `read()` 函数而驱动尚未捕获时，`select()` 函数开始捕获。当失败时，`select()` 返回成功，而随后同样尝试开始捕获的 `read()` 调用将返回适当的错误码。当驱动连续捕获（相对于例如静止图像）且数据已可用时，`select()` 函数立即返回

当已协商使用 `write()` 函数时，`select()` 函数只是等待，直到驱动准备好进行非阻塞的 `write()` 调用

所有实现了 `read()` `write()` 函数或流 I/O 的驱动也必须支持 `select()` 函数

更多细节请参`select()` 手册页

## 杩斿洖鍊。


成功时，`select()` 返回三个返回的描述符集合中包含的描述符数量，若超时则0。出错时返回 -1，并相应地设`errno` 变量；集合与 `timeout` 变为未定义。可能的错误码为

EBADF
    一个或多个文件描述符集合指定了未打开的文件描述符

EBUSY
    驱动不支持多个读或写流，且设备已在使用中

EFAULT
    `readfds`、`writefds`、`exceptfds` `timeout` 指针引用了不可访问的内存区域

EINTR
    调用被信号中断

EINVAL
    `nfds` 参数小于零或大于 `FD_SETSIZE`

   Linux 内核实现 `select()` 类似`poll()` 函数，但 `select()` 不能返回 `POLLERR`
