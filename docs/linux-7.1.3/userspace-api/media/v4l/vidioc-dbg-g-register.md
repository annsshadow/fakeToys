######## ioctl VIDIOC_DBG_G_REGISTER, VIDIOC_DBG_S_REGISTER


## 名称


VIDIOC_DBG_G_REGISTER - VIDIOC_DBG_S_REGISTER - 读或写硬件寄存器

## 概要



`int ioctl(int fd, VIDIOC_DBG_G_REGISTER, struct v4l2_dbg_register *argp)`


`int ioctl(int fd, VIDIOC_DBG_S_REGISTER, const struct v4l2_dbg_register *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向 struct `v4l2_dbg_register` 的指针

## 描述



    这是一个实验性接口，未来可能会发生变化

出于驱动调试目的，这ioctl 允许测试应用程序直接访问硬件寄存器。普通应用程序不得使用它们

由于写入甚至读取寄存器都可能危及系统安全、稳定性并损坏硬件，两ioctl 都需要超级用户权限。此外，Linux 内核必须编译时启`CONFIG_VIDEO_ADV_DEBUG` 选项以启用这ioctl

要写入一个寄存器，应用程序必须初始化 struct `v4l2_dbg_register` 的所有字段（`size` 除外），并使用指向该结构的指针调`VIDIOC_DBG_S_REGISTER`。`match.type` `match.addr` `match.name` 字段选择 TV 卡上的一个芯片，`reg` 字段指定寄存器编号，`val` 字段为要写入寄存器的值

要读取一个寄存器，应用程序必须初始化 `match.type`、`match.addr` `match.name` 以及 `reg` 字段，并使用指向该结构的指针调用 `VIDIOC_DBG_G_REGISTER`。成功时，驱动将寄存器值存储在 `val` 字段中，并将该值的大小（以字节为单位）存储`size` 中

`match.type` `V4L2_CHIP_MATCH_BRIDGE` 时，`match.addr` 选择 TV 卡上的第 n 个非子设备芯片。数字零始终选择主芯片，例如连接PCI USB 总线的芯片。你可以通过 VIDIOC_DBG_G_CHIP_INFO ioctl 了解存在哪些芯片

`match.type` `V4L2_CHIP_MATCH_SUBDEV` 时，`match.addr` 选择n 个子设备

这些 ioctl 是可选的，并非所有驱动都可能支持它们。然而，当驱动支持这ioctl 时，它也必须支持 VIDIOC_DBG_G_CHIP_INFO。反之，它可能支`VIDIOC_DBG_G_CHIP_INFO` 但不支持这些 ioctl

`VIDIOC_DBG_G_REGISTER` `VIDIOC_DBG_S_REGISTER` 是在 Linux 2.6.21 中引入的，但API 在内2.6.29 中被更改为此处所描述的版本

我们建议使用 v4l2-dbg 工具，而不是直接调用这ioctl。它可从 LinuxTV v4l-dvb 仓库获取；访问说明见 `https://linuxtv.org/repo/ <https://linuxtv.org/repo/>`__




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 有关可能的类型列表，请参chip-match-types
    - - union {
      - (anonymous)
    - - __u32
      - `addr`
      - 按此编号匹配芯片，依`type` 字段解释
    - - char
      - `name[^32^]`
      - 按此名称匹配芯片，依`type` 字段解释。当前未使用
    - - }
      -

    :header-rows:  0
    :stub-columns: 0

    - - struct v4l2_dbg_match
      - `match`
      - 如何匹配芯片，请参见 `v4l2_dbg_match`
    - - __u32
      - `size`
      - 寄存器大小（以字节为单位）
    - - __u64
      - `reg`
      - 寄存器编号
    - - __u64
      - `val`
      - 从寄存器读取或要写入寄存器的值



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CHIP_MATCH_BRIDGE`
      - 0
      - 匹配卡上的第 n 个芯片，桥接芯片为零。不匹配子设备
    - - `V4L2_CHIP_MATCH_SUBDEV`
      - 4
      - 匹配n 个子设备

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

EPERM
    权限不足。执行这ioctl 需root 权限
