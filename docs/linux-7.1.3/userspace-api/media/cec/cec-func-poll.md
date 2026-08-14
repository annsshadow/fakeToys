


######## cec poll()


## 名称


cec-poll - 等待文件描述符上的某个事件

## 概要



    #include <sys/poll.h>


## 参数


`ufds`
   要监视的 FD 事件列表

`nfds`
   \*ufds 数组中的 FD 事件数量

`timeout`
   等待事件的超时时间

## 描述


通过 `poll()` 函数，应用程序可以等待 CEC
事件。

成功时，`poll()` 返回已选中的文件描述符
数量（即相应结构体 `pollfd` 的
`revents` 字段非零的文件描述符）。当接收队列中
有消息时，CEC 设备会在
`revents` 字段中设置 `POLLIN` 和 `POLLRDNORM` 标志。如果
发送队列有空间容纳新消息，则设置 `POLLOUT` 和
`POLLWRNORM` 标志。如果事件队列中有事件，
则设置 `POLLPRI` 标志。当函数超时时返回
零值；失败时返回 -1，并相应地设置 `errno` 变量。

更多细节请参阅 `poll()` 手册页。

## 返回值


成功时，`poll()` 返回具有非零
`revents` 字段的结构体数量，若调用超时则返回零。出错时返回 -1，
并相应地设置 `errno` 变量：

`EBADF`
    一个或多个 `ufds` 成员指定了无效的文件
    描述符。

`EFAULT`
    `ufds` 引用了不可访问的内存区域。

`EINTR`
    调用被信号中断。

`EINVAL`
    `nfds` 值超过了 `RLIMIT_NOFILE` 值。使用
    `getrlimit()` 获取该值。
