


######## ioctl VIDIOC_G_STD, VIDIOC_S_STD, VIDIOC_SUBDEV_G_STD, VIDIOC_SUBDEV_S_STD


## 名称


VIDIOC_G_STD - VIDIOC_S_STD - VIDIOC_SUBDEV_G_STD - VIDIOC_SUBDEV_S_STD - 查询或选择当前输入的视频标
## 概要



`int ioctl(int fd, VIDIOC_G_STD, v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_S_STD, const v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_STD, v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_STD, const v4l2_std_id *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 `v4l2_std_id` 的指针
## 描述


要查询和选择当前的视频标准，应用程序使用 VIDIOC_G_STD <VIDIOC_G_STD> VIDIOC_S_STD <VIDIOC_G_STD> ioctl，它们将指向 v4l2_std_id <v4l2-std-id> 类型的指针作为参数。VIDIOC_G_STD <VIDIOC_G_STD> 可以返回单个标志或一组标志，如同 struct `v4l2_standard` `id` 字段那样。这些标志必须明确无误，即它们只出现在唯一的某个被枚举struct `v4l2_standard` 结构中
VIDIOC_S_STD <VIDIOC_G_STD> 接受一个或多个标志，作为一个只ioctl，它不会VIDIOC_G_STD <VIDIOC_G_STD> 那样返回实际的新标准。当没有给出任何标志，或者当前输入不支持所请求的标准时，驱动返`EINVAL` 错误码。当标准集合存在歧义时，驱动可能返回 `EINVAL` 或选择任意一个所请求的标准。如果当前输入或输出不支持标准视频时序（例如，若 VIDIOC_ENUMINPUT 没有设置 `V4L2_IN_CAP_STD` 标志），则返`ENODATA` 错误码
在对以只读模式注册的子设备（subdev）节点上调用 `VIDIOC_SUBDEV_S_STD` 是不允许的。将返回错误并把 errno 变量设置`-EPERM`
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    VIDIOC_S_STD <VIDIOC_G_STD> 参数不合适
ENODATA
    该输入或输出不支持标准视频时序
EPERM
    `VIDIOC_SUBDEV_S_STD` 在被调用在一个只读子设备上