


######## ioctl VIDIOC_STREAMON, VIDIOC_STREAMOFF


## 名称


VIDIOC_STREAMON - VIDIOC_STREAMOFF - 开始或停止I/O

## 概要


`int ioctl(int fd, VIDIOC_STREAMON, const int *argp)`


`int ioctl(int fd, VIDIOC_STREAMOFF, const int *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向一个整数的指针

## 描述


`VIDIOC_STREAMON` `VIDIOC_STREAMOFF` ioctl 在流（streaming）（内存映射 <mmap>、用户指<userp> DMABUF <dmabuf>）I/O 期间启动与停止捕获或输出过程

在调`VIDIOC_STREAMON` 之前，捕获硬件被禁用，不会填充任何输入缓冲区（如果传入队列中有任何空缓冲区）。在调用 `VIDIOC_STREAMON` 之前，输出硬件被禁用，不会产生任何视频信号

内存到内存设备直到为捕获与输出两种流类型都调用了 `VIDIOC_STREAMON` 后才会启动

如果 `VIDIOC_STREAMON` 失败，则任何已排队的缓冲区将保持排队状态

`VIDIOC_STREAMOFF` ioctl 除了中止或完成任何进行中DMA 外，还会解锁任何锁定在物理内存中的用户指针缓冲区，并将所有缓冲区从传入与传出队列中移除。这意味着所有已捕获但尚未出队的帧都将丢失，同样所有已入队用于输出但尚未传输的帧也会丢失。I/O 返回到与调用 VIDIOC_REQBUFS 之后相同的状态，并可相应地重新启动

如果缓冲区已通过 VIDIOC_QBUF 排队，且在从未调用过 `VIDIOC_STREAMON` 的情况下调用`VIDIOC_STREAMOFF`，那么这些已排队的缓冲区也将从传入队列中移除，并全部返回到与调用 VIDIOC_REQBUFS 之后相同的状态，可相应地重新启动

两个 ioctl 都接受一个指向整数的指针，即期望的缓冲区或流类型。这与结构体 `v4l2_requestbuffers` `type` 相同

如果在流已在进行中时调用 `VIDIOC_STREAMON`，或在流已停止时调用 `VIDIOC_STREAMOFF`，则返回 0。在 `VIDIOC_STREAMON` 的情况下什么也不会发生，但 `VIDIOC_STREAMOFF` 会如上所述将已排队的缓冲区返回到它们的起始状态


   应用程序可能`VIDIOC_STREAMON` `VIDIOC_STREAMOFF` 调用之前或之后的未知时间段内被抢占，没有“现在”开始或停止的概念。可以使用缓冲区时间戳来与其他事件同步

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 章节中描述

EINVAL
    不支持缓冲区 `type`，或尚未分配（内存映射）或入队（输出）任何缓冲区

EPIPE
    驱动实现pad 级格式配<pad-level-formats>，且流水线配置无效

ENOLINK
    驱动实现Media Controller 接口，且流水线链路配置无效
