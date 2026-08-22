

######## RDS 接口


无线电数据系统（Radio Data System）以二进制格式传输补充信息，例如电台名称
或交通信息，位于广播节目听不见的音频副载波上。此接口面向能够接收发RDS 信息的设备
更多信息请参见核RDS 标准 iec62106 RBDS 标准 nrsc4

   注意 RBDS 标准（在美国使用）与 RDS 标准几乎完全相同。任RDS 解码
   编码器也可以处理 RBDS。只有某些字段的含义略有不同。更多信息请参见
   RBDS 标准
RBDS 标准还规定了MMBS（Modified Mobile Search）的支持。这是一种似乎已弃用的专有格式。RDS 接口不支持此格式。如果需要支MMBS（或通常所谓的
“E blocks”），请联系 linux-media 邮件列表`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__
## 查询能力


支持 RDS 捕获 API 的设备会VIDIOC_QUERYCAP ioctl 返回struct
`v4l2_capability` `capabilities` 字段中设`V4L2_CAP_RDS_CAPTURE` 标志任何支持 RDS 的调谐器（tuner）都会在 struct `v4l2_tuner` `capability`
字段中设`V4L2_TUNER_CAP_RDS` 标志。如果驱动只是传RDS 块而不解释数据则必须设`V4L2_TUNER_CAP_RDS_BLOCK_IO` 标志，见读取 RDS 数据
<reading-rds-data>。为将来使用，也定义`V4L2_TUNER_CAP_RDS_CONTROLS`
标志。然而，具有此能力的无线电调谐器驱动尚不存在，因此如果你打算编写这样
一个驱动，你应该在 linux-media 邮件列表上讨论：
`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__銆。
是否存在RDS 信号可以通过查看 struct `v4l2_tuner` `rxsubchans` 字段
来检测：如果检测到RDS 数据，将设置 `V4L2_TUNER_SUB_RDS`
支持 RDS 输出 API 的设备会VIDIOC_QUERYCAP ioctl 返回struct
`v4l2_capability` `capabilities` 字段中设`V4L2_CAP_RDS_OUTPUT` 标志任何支持 RDS 的调制器（modulator）都会在 struct `v4l2_modulator` `capability` 字段中设`V4L2_TUNER_CAP_RDS` 标志。为了启RDS 传输，必struct `v4l2_modulator` `txsubchans` 字段中设`V4L2_TUNER_SUB_RDS`
位。如果驱动只是传RDS 块而不解释数据，则必须设置
`V4L2_TUNER_CAP_RDS_BLOCK_IO` 标志。如果调谐器能够处理 RDS 实体（如节目
识别码和广播文本），则应设置 `V4L2_TUNER_CAP_RDS_CONTROLS` 标志，见写入
RDS 数据 <writing-rds-data> FM 发射器控制参<fm-tx-controls>

## 读取 RDS 数据


可以使用 `read()` 函数从无线电设备读取 RDS 数据。数据以三个字节为一打包

## 写入 RDS 数据


可以使用 `write()` 函数向无线电设备写入 RDS 数据。数据以三个字节为一打包，如下所示：

## RDS 数据结构



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 5

    - - __u8
      - `lsb`
      - RDS 块的最低有效字节（Least Significant Byte）    - - __u8
      - `msb`
      - RDS 块的最高有效字节（Most Significant Byte）    - - __u8
      - `block`
      - 块描述



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 5

    - - Bits 0-2
      - 接收数据的块（即偏移量，offset）    - - Bits 3-5
      - 已弃用。当前与 bits 0-2 相同。不要使用这些位    - - Bit 6
      - 已纠正位（Corrected bit）。指示此数据块中有一个错误被纠正    - - Bit 7
      - 错误位（Error bit）。指示在接收此块期间发生了不可纠正的错误



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 1 5

    - - V4L2_RDS_BLOCK_MSK
      -
      - 7
      - 用于获取ID bits 0-2 掩码    - - V4L2_RDS_BLOCK_A
      -
      - 0
      - 鍧?A銆?    - - V4L2_RDS_BLOCK_B
      -
      - 1
      - 鍧?B銆?    - - V4L2_RDS_BLOCK_C
      -
      - 2
      - 鍧?C銆?    - - V4L2_RDS_BLOCK_D
      -
      - 3
      - 鍧?D銆?    - - V4L2_RDS_BLOCK_C_ALT
      -
      - 4
      - 鍧?C'銆?    - - V4L2_RDS_BLOCK_INVALID
      - read-only
      - 7
      - 一个无效的块    - - V4L2_RDS_BLOCK_CORRECTED
      - read-only
      - 0x40
      - 检测到一个位错误但已被纠正    - - V4L2_RDS_BLOCK_ERROR
      - read-only
      - 0x80
      - 发生了不可纠正的错误