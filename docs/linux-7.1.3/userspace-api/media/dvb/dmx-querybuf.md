
######## ioctl DMX_QUERYBUF


## 名称


DMX_QUERYBUF - 查询缓冲区的状


## 概要


`int ioctl(int fd, DMX_QUERYBUF, struct dvb_buffer *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向结构`dvb_buffer` 的指针

## 描述


ioctl mmap 流式 I/O 方法的一部分。它可以在使DMX_REQBUFS ioctl 分配缓冲区之后的任何时间用于查询缓冲区的状态

应用程序设置 `index` 字段。有效的索引编号范围从零到使DMX_REQBUFS 分配的缓冲区数量（结构体 `dvb_requestbuffers` `count`）减一

在使用指向该结构的指针调DMX_QUERYBUF 后，驱动返回错误码或填充结构的其余部分

成功时，`offset` 将包含缓冲区距设备内存起始位置的偏移，`length` 字段为其大小，`bytesused` 为缓冲区中数据（有效载荷）占用的字节数

## 杩斿洖鍊。


成功时返0，`offset` 将包含缓冲区距设备内存起始位置的偏移，`length` 字段为其大小，`bytesused` 为缓冲区中数据（有效载荷）占用的字节数

出错时返-1，并相应地设`errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述

EINVAL
    `index` 越界
