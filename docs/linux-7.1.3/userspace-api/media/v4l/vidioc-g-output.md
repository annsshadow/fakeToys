
######## ioctl VIDIOC_G_OUTPUT, VIDIOC_S_OUTPUT


## Name


VIDIOC_G_OUTPUT - VIDIOC_S_OUTPUT - 查询或选择当前的视频输

## Synopsis


`int ioctl(int fd, VIDIOC_G_OUTPUT, int *argp)`


`int ioctl(int fd, VIDIOC_S_OUTPUT, int *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符

`argp`
    指向包含输出索引的整数的指针

## Description


要查询当前的视频输出，应用程序需调用
VIDIOC_G_OUTPUT <VIDIOC_G_OUTPUT> ioctl，并传入一个指向整数的指针，驱动会
将输出编号存入其中，如同结构
`v4l2_output` `index` 字段一样。该 ioctl 仅在没有视频输出时才
失败，并返回 `EINVAL` 错误码

要选择某个视频输出，应用程序需将要选择的输出编号存入一个整数，并调VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> ioctl，传
指向该整数的指针。这可能会产生副作用。例如，不同的输出可能支持不同的视频标准，因此驱动可能会隐式
切换当前标准。由于这些可能的副作用，应用程序必须先选择一个输出，然后再查询或
协商任何其他参数

有关视频输出的信息可通过
VIDIOC_ENUMOUTPUT ioctl 获取

## Return Value


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

EINVAL
    视频输出的编号越界，或者根本没有任何视频输出
