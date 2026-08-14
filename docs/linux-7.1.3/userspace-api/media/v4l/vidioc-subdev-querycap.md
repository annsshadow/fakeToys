


######## ioctl VIDIOC_SUBDEV_QUERYCAP


## 名称


VIDIOC_SUBDEV_QUERYCAP - 查询子设备能力

## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_QUERYCAP, struct v4l2_subdev_capability *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_subdev_capability` 的指针。

## 描述


所有 V4L2 子设备都支持 `VIDIOC_SUBDEV_QUERYCAP` ioctl。它用于识别与本
规范兼容的内核设备，并获取有关驱动与硬件能力的信息。该 ioctl 接受一个
指向 struct `v4l2_subdev_capability` 的指针，由驱动填充。当驱动与本
规范不兼容时，该 ioctl 返回 `ENOTTY` 错误码。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u32
      - `version`
      - 驱动的版本号。

	报告的版本由 V4L2 子系统按照内核编号方案提供。不过，它可能并非
	总是返回与内核相同的版本，例如，当某个稳定版或修改过的发行版
	内核使用了来自更新内核的 V4L2 栈时。

	版本号使用 `KERNEL_VERSION()` 宏格式化：
    - - `2`

	`#define KERNEL_VERSION(a,b,c) (((a) << 16) + ((b) << 8) + (c))`

	`__u32 version = KERNEL_VERSION(0, 8, 1);`

	`printf ("Version: %u.%u.%u\\n",`

	`(version >> 16) & 0xFF, (version >> 8) & 0xFF, version & 0xFF);`
    - - __u32
      - `capabilities`
      - 所打开设备的子设备能力，请参阅
	subdevice-capabilities。
    - - __u32
      - `reserved`\ [^14^]
      - 为未来扩展保留。由 V4L2 核心设置为 0。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_CAP_RO_SUBDEV
      - 0x00000001
      - 子设备设备节点以只读模式注册。
	对修改设备状态的子设备 ioctl 的访问受到限制。关于哪些限制适用于
	只读子设备，请参阅各自的子设备 ioctl 文档。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

ENOTTY
    该设备节点不是 V4L2 子设备。
