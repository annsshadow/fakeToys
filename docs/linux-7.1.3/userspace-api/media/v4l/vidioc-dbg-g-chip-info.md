


######## ioctl VIDIOC_DBG_G_CHIP_INFO


## 名称


VIDIOC_DBG_G_CHIP_INFO - 识别电视卡上的芯
## 语法


`int ioctl(int fd, VIDIOC_DBG_G_CHIP_INFO, struct v4l2_dbg_chip_info *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_dbg_chip_info` 的指针
## 描述



    这是一个实验性接口，将来可能会发生变化
出于驱动调试目的，该 ioctl 允许测试程序向驱动查询电视卡上存在的芯片信息。普应用程序不得使用该接口。如果你发现了芯片相关的 bug，请联系 linux-media 邮件列表
（`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__），
以便修复
此外，Linux 内核必须开`CONFIG_VIDEO_ADV_DEBUG` 选项编译，才能启用该 ioctl
要查询驱动，应用程序必须初始struct `v4l2_dbg_chip_info` `match.type` `match.addr` `match.name` 字段，并以指向该结构的指针调VIDIOC_DBG_G_CHIP_INFO。成功时，驱动将所选芯片的信息存入 `name` `flags` 字段
`match.type` `V4L2_CHIP_MATCH_BRIDGE` 时，`match.addr` 选择电视卡上的第 n
个桥接“芯片”。你可以0 开始，每次`match.addr` 1，直VIDIOC_DBG_G_CHIP_INFO
`EINVAL` 错误码失败，从而枚举所有芯片。编0 总是选择桥接芯片本身，例如连接到
PCI USB 总线的芯片。非零编号标识桥接芯片的特定部分，例如一AC97 寄存器块
`match.type` `V4L2_CHIP_MATCH_SUBDEV` 时，`match.addr` 选择n 个子设备这允许你枚举所有子设备
成功时，`name` 字段将包含一个芯片名称，`flags` 字段在驱动支持从设备读取寄存器时
包含 `V4L2_CHIP_FL_READABLE`，或在驱动支持向设备写入寄存器时包含
`V4L2_CHIP_FL_WRITABLE`銆。
相比直接调用ioctl，我们推荐使v4l2-dbg 工具。它可从 LinuxTV v4l-dvb 仓库
获取，参`https://linuxtv.org/repo/ <https://linuxtv.org/repo/>`__ 以获访问说明


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 可能的类型列表，参见 name-chip-match-types    - - union {
      - (anonymous)
    - - __u32
      - `addr`
      - 按此编号匹配芯片，具体解释取决于 `type` 字段    - - char
      - `name[^32^]`
      - 按此名称匹配芯片，具体解释取决于 `type` 字段。当前未使用    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct v4l2_dbg_match
      - `match`
      - 如何匹配芯片，参name-v4l2-dbg-match    - - char
      - `name[^32^]`
      - 芯片的名称    - - __u32
      - `flags`
      - 由驱动设置。若设置`V4L2_CHIP_FL_READABLE`，则驱动支持从设备读取寄存器	若设置了 `V4L2_CHIP_FL_WRITABLE`，则支持写入寄存器    - - __u32
      - `reserved[^8^]`
      - 保留字段，应用程序与驱动都必须将其置0


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CHIP_MATCH_BRIDGE`
      - 0
      - 匹配卡上的第 n 个芯片，0 表示桥接芯片。不匹配子设备    - - `V4L2_CHIP_MATCH_SUBDEV`
      - 4
      - 匹配n 个子设备
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
EINVAL
    `match_type` 无效，或无法匹配到任何设备