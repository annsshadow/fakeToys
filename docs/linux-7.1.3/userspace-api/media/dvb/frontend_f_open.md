


######## 数字电视前端 open()


## 名称（Name

fe-open - 打开一个前端设
## 概要（Synopsis


    #include <fcntl.h>


## 参数（Arguments

`device_name`
    要打开的设备
`flags`
    打开标志。访问方式可以是 `O_RDWR` `O_RDONLY`
    允许多次`O_RDONLY` 打开。在这种模式下，只允许查询与读取 ioctl
    `O_RDWR` 模式下只允许打开一次。在这种模式下，允许所ioctl
    当给`O_NONBLOCK` 标志时，在没有数据可用或设备驱动暂时繁忙的情况下，系    调用可能会返`EAGAIN` 错误码
    其它标志没有效果
## 描述（Description

该系统调用打开一个命名的前端设备（`/dev/dvb/adapter/frontend`）以供后续使用通常在成功打开后要做的第一件事，是FE_GET_INFO 查明前端类型
设备可以以只读模式打开（只允许监视设备状态与统计信息），也可以以读写模式打开
（允许任何类型的使用，例如执行调谐操作）
在一个具有多个前端的系统中，通常无法同时以读写模式打开多个设备。只要一个前设备以读写模式打开，其它以读写模式open() 调用要么失败，要么阻塞，取决于指的是非阻塞还是阻塞模式。一个以阻塞模式打开的前端设备，之后可以使用 fcntl 系统
调用F_SETFL 命令切换到非阻塞模式（反之亦然）。这是一个标准的系统调用，在
Linux fcntl 手册页中有说明。当 open() 调用成功后，设备将准备好以指定模式使用这意味着相应的硬件被上电，并且其它前端可能已被断电以使之成为可能
## 返回值（Return Value

成功`open()` 返回新的文件描述符出错时返-1，并适当地设`errno` 变量
可能的错误码有：

成功时返0，并`ca_slot_info` 被填充
出错时返-1，并`errno` 变量被适当设置

    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - - `EPERM`
       - 调用者没有访问该设备的权限
    - - `EBUSY`
       - 设备驱动已在使用中
    - - `EMFILE`
       - 进程已经打开了最大数量的文件
    - - `ENFILE`
       - 系统上打开文件总数的限制已经达到
通用错误码在 Generic Error Codes <gen-errors> 章节中描述