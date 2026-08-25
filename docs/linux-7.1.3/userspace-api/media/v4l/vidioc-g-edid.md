


######## ioctl VIDIOC_G_EDID, VIDIOC_S_EDID, VIDIOC_SUBDEV_G_EDID, VIDIOC_SUBDEV_S_EDID


## 名称


VIDIOC_G_EDID - VIDIOC_S_EDID - VIDIOC_SUBDEV_G_EDID - VIDIOC_SUBDEV_S_EDID - 获取或设置视频接收器/发送器EDID

## 概要


`int ioctl(int fd, VIDIOC_G_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_S_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_EDID, struct v4l2_edid *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
   指向 struct `v4l2_edid` 的指针
## 描述


这些 ioctl 可用于获取或设置与接收器的输入或发送器设备的输出相关联EDID。它们可以与子设备节dev/v4l-subdevX）或视频节点dev/videoX）一起使用
与视频节点一起使用时，`pad` 字段表示输入（对于视频采集设备）或输出（对于视频输出设备）索引，分别VIDIOC_ENUMINPUT VIDIOC_ENUMOUTPUT 返回。与子设备节点一起使用时，`pad` 字段表示子设备的输入输出 pad。如果对于给定的 `pad` 值没EDID 支持，则将返`EINVAL` 错误码
要获EDID 数据，应用程序必须填`pad`、`start_block`、`blocks` `edid` 字段，将 `reserved`
数组清零，并调用 VIDIOC_G_EDID <VIDIOC_G_EDID>。从 `start_block` 块开始、大小为 `blocks` 的当EDID 将被放入 `edid` 指向的内存中。`edid` 指针必须指向至少 `blocks` * 128 字节大小的内存（一个块大小128 字节）
如果块数少于指定的数量，则驱动会`blocks` 设置为实际的块数。如果根本没有任EDID 块可用，则设错误`ENODATA`
如果块必须从 sink 获取，则此调用将阻塞，直到它们被读取
如果在调VIDIOC_G_EDID <VIDIOC_G_EDID> `start_block` `blocks` 都设置为 0，则驱动会将
`blocks` 设置为可用的 EDID 块总数，并返回 0 而不复制任何数据。这是发现有多少EDID 块的简单方法

   如果没有任何 EDID 块可用，则驱动会`blocks` 设置0 并返0
要设置接收器EDID 块，应用程序必须填写 `pad`、`blocks` `edid` 字段，将 `start_block` 设置0并将 `reserved` 数组清零。不可能只设EDID 的一部分，它总是全有或全无。设EDID 数据仅对接收器有效，
因为对发送器来说没有意义
驱动假定传入的是完整EDID。如EDID 块多于硬件能处理的数量，则不会写EDID，而是设置错误`E2BIG`，并`blocks` 被设置为硬件支持的最大值。如`start_block` 0 以外的任何值，则设置错误码
`EINVAL`銆。
要禁EDID，你`blocks` 设置0。根据硬件的不同，这会将热插拔引脚拉低和/或以某种方式阻止源读EDID
数据。无论如何，最终结果是相同的：EDID 不再可用

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - 要获设置 EDID 块的 pad。与视频设备节点一起使用时，pad 表示输入或输出索引，分别	VIDIOC_ENUMINPUT VIDIOC_ENUMOUTPUT 返回    - - __u32
      - `start_block`
      - 从此块开始读EDID。设EDID 时必须为 0    - - __u32
      - `blocks`
      - 要获取或设置的块数。必须小于或等于 256（标准定义的最大块数）。当你设EDID `blocks` 0
	时，EDID 被禁用或擦除    - - __u32
      - `reserved`\ [^5^]
      - 为未来扩展保留。应用程序和驱动必须将数组设置为零    - - __u8 *
      - `edid`
      - 指向包含 EDID 的内存。最小大小为 `blocks` * 128
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 章节中描述
`ENODATA`
    EDID 数据不可用
`E2BIG`
    你提供的 EDID 数据超过了硬件能处理的数量