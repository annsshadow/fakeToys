


######## ioctl VIDIOC_G_PRIORITY, VIDIOC_S_PRIORITY


## 名称


VIDIOC_G_PRIORITY - VIDIOC_S_PRIORITY - 查询或请求与文件描述符关联的访问优先
## 概要



`int ioctl(int fd, VIDIOC_G_PRIORITY, enum v4l2_priority *argp)`


`int ioctl(int fd, VIDIOC_S_PRIORITY, const enum v4l2_priority *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 enum `v4l2_priority` 类型的指针
## 描述


要查询当前的访问优先级，应用程序调用 VIDIOC_G_PRIORITY <VIDIOC_G_PRIORITY> ioctl，并传入一个指enum v4l2_priority 变量的指针，驱动将把当前优先级存入其中
要请求某个访问优先级，应用程序将期望的优先级存入一enum v4l2_priority 变量，并调用 VIDIOC_S_PRIORITY <VIDIOC_G_PRIORITY> ioctl，传入指向该变量的指针


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_PRIORITY_UNSET`
      - 0
      -
    - - `V4L2_PRIORITY_BACKGROUND`
      - 1
      - 最低优先级，通常是后台运行的应用程序，例如监VBI 传输。如果多个应用程序想	此优先级从设备读取，则需要一个运行在用户空间的代理应用程序    - - `V4L2_PRIORITY_INTERACTIVE`
      - 2
      -
    - - `V4L2_PRIORITY_DEFAULT`
      - 2
      - 中等优先级，通常是用户启动并交互控制的应用程序。例如电视查看器、图文电视（Teletext	浏览器，或仅用于改变频道或视频控制的“面板”应用程序。除非某应用程序请求了其他优先级	否则这是默认优先级    - - `V4L2_PRIORITY_RECORD`
      - 3
      - 最高优先级。只有一个文件描述符可以具有此优先级，它会阻止任何其fd 改变设备属性	通常是像视频录制这样不能被中断的应用程序
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    请求的优先级值无效
EBUSY
    另一个应用程序已经请求了更高的优先级