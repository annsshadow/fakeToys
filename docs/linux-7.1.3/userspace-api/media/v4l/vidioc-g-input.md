
######## ioctl VIDIOC_G_INPUT, VIDIOC_S_INPUT


## 名称


VIDIOC_G_INPUT - VIDIOC_S_INPUT - 查询或选择当前的视频输入

## 概要


`int ioctl(int fd, VIDIOC_G_INPUT, int *argp)`


`int ioctl(int fd, VIDIOC_S_INPUT, int *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向包含输入索引的整数。

## 描述


要查询当前的视频输入，应用程序需调用 VIDIOC_G_INPUT <VIDIOC_G_INPUT> ioctl，并传入一个指向整数的指针，驱动会将该输入的编号存入其中，如同结构体 `v4l2_input` 的 `index` 字段一样。该 ioctl 仅在没有视频输入时才会失败，并返回 `EINVAL`。

要选择某个视频输入，应用程序需将要选择的输入编号存入一个整数，并调用 VIDIOC_S_INPUT <VIDIOC_G_INPUT> ioctl，传入指向该整数的指针。这可能会产生副作用。例如，输入可能支持不同的视频标准，因此驱动可能会隐式地切换当前标准。由于这些可能的副作用，应用程序必须先选择一个输入，然后再查询或协商任何其他参数。

有关视频输入的信息可通过 VIDIOC_ENUMINPUT ioctl 获取。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    视频输入的编号越界。
