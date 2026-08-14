
######## request poll()


## 名称


request-poll - 等待文件描述符上的某个事件

## 概要


    #include <sys/poll.h>


## 参数


`ufds`
   待监视的文件描述符事件列表

`nfds`
   \*ufds 数组中的文件描述符事件数量

`timeout`
   等待事件的超时时间

## 描述


应用程序可以使用 `poll()` 函数等待请求完成。

成功时 `poll()` 返回已被选中的文件描述符数量（即其相应的结构体 `pollfd` 的 `revents` 字段非零的文件描述符）。当请求完成时，请求文件描述符会在 `revents` 中设置 `POLLPRI` 标志。当函数超时时返回零，失败时返回 -1 并相应地设置 `errno` 变量。

试图 poll 一个尚未入队的请求会在 `revents` 中设置 `POLLERR` 标志。

## 返回值


成功时，`poll()` 返回具有非零 `revents` 字段的结构体数量，若调用超时则返回零。出错时返回 -1，并相应地设置 `errno` 变量：

`EBADF`
    一个或多个 `ufds` 成员指定了无效的文件描述符。

`EFAULT`
    `ufds` 引用了不可访问的内存区域。

`EINTR`
    调用被信号中断。

`EINVAL`
    `nfds` 值超过了 `RLIMIT_NOFILE` 值。使用 `getrlimit()` 获取该值。
