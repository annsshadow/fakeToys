


######## ioctl VIDIOC_SUBDEV_G_CLIENT_CAP, VIDIOC_SUBDEV_S_CLIENT_CAP


## 名称


VIDIOC_SUBDEV_G_CLIENT_CAP - VIDIOC_SUBDEV_S_CLIENT_CAP - 获取或设置客户端
能力
## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_G_CLIENT_CAP, struct v4l2_subdev_client_capability *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_CLIENT_CAP, struct v4l2_subdev_client_capability *argp)`

## 参数


`fd`
    open() <func-open> 返回的文件描述符
`argp`
    指向 struct `v4l2_subdev_client_capability` 的指针
## 描述


这些 ioctl 用于获取和设置客户端（使subdevice ioctl 的应用程序）能力。客户端
能力存储在打开subdev 设备节点的文件句柄中，客户端必须分别为每个打开subdev 设置能力
默认情况下，打开 subdev 设备节点时不设置任何客户端能力
客户端能力的用途是告知内核该客户端的行为，主要与保持不同内核与用户空间版本
之间的兼容性有关
`VIDIOC_SUBDEV_G_CLIENT_CAP` ioctl 返回与文件句`fd` 关联的当前客户端能力
`VIDIOC_SUBDEV_S_CLIENT_CAP` ioctl 设置文件句柄 `fd` 的客户端能力。新的能力会
完全替换当前能力，因此该 ioctl 也可用于移除先前已设置的能力
`VIDIOC_SUBDEV_S_CLIENT_CAP` 会修struct `v4l2_subdev_client_capability` 反映已被接受的能力。内核不接受某能力的一种常见情况是，内核比用户空间使用头文件更旧，因此该能力对内核而言是未知的

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u64
      - `capabilities`
      - 所打开设备的子设备客户端能力
    :header-rows:  1

    - - 能力
      - 描述
    - - `V4L2_SUBDEV_CLIENT_CAP_STREAMS`
      - 客户端了解流（stream）。设置此标志可启用各ioctl 'stream' 字段
        （指流编号）的使用。若未设置（默认如此），'stream' 字段将被内核强制
        涓?0銆?    - - `V4L2_SUBDEV_CLIENT_CAP_INTERVAL_USES_WHICH`
      - 客户端了`v4l2_subdev_frame_interval` `which` 字段。若未设        （默认如此），`which` 字段将被内核强制`V4L2_SUBDEV_FORMAT_ACTIVE`
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
通用错误<gen-errors> 章节中描述
ENOIOCTLCMD
   内核不支持此 ioctl