
######## ioctl VIDIOC_ENUMINPUT


## 名称


VIDIOC_ENUMINPUT - 枚举视频输入

## 概要


`int ioctl(int fd, VIDIOC_ENUMINPUT, struct v4l2_input *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_input` 的指针
## 描述


为了查询某个视频输入的属性，应用程序先初始化结构`v4l2_input` `index` 字段，然以指向该结构的指针调VIDIOC_ENUMINPUT。驱动填充结构的其余部分；当 index 越界时返`EINVAL` 错误码。为了枚举所有输入，应用程序应从索引 0 开始，每次1，直到驱动返`EINVAL`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 标识该输入，由应用程序设置    - - __u8
      - `name`\ [^32^]
      - 视频输入的名称，一个以 NUL 结尾ASCII 字符串，例如Vin (Composite 2)"	此信息面向用户，最好是设备本身的连接器标签    - - __u32
      - `type`
      - 输入的类型，input-type    - - __u32
      - `audioset`
      - 驱动最多可以枚32 个视频与音频输入。该字段显示当此输入为当前选中视频输入时，
	哪些音频输入可作为音源被选择。它是一个位掩码。最低位（LSB）对应音频输0，最高位
	（MSB）对应输31。可以设置任意数量的位，也可以都不设置
	当驱动不枚举音频输入时，不得设置任何位。应用程序不应将此解释为缺少音频支持	某些驱动会自动选择音源，由于本就没有选择余地，因此不枚举它们
	有关音频输入以及如何选择当前输入的细节，audio    - - __u32
      - `tuner`
      - 采集设备可以拥有零个或多个调谐器（RF 解调器）。当 `type` 设为
	`V4L2_INPUT_TYPE_TUNER` 时，这是一RF 连接器，该字段标识此调谐器。它对应	结构`v4l2_tuner` `index` 字段。有关调谐器的细节，tuner    - - v4l2_std_id <v4l2-std-id>
      - `std`
      - 每个视频输入都支持一种或多种不同的视频标准。该字段是所有受支持标准组成的集合	有关视频标准以及如何切换的细节，standard    - - __u32
      - `status`
      - 该字段提供关于此输入的状态信息。标志见 input-status。除传感器方向位外，
	`status` 仅在这是当前输入时才有效    - - __u32
      - `capabilities`
      - 该字段提供此输入的能力。标志见 input-capabilities    - - __u32
      - `reserved`\ [^3^]
      - 为将来扩展保留。驱动必须将该数组置零


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_INPUT_TYPE_TUNER`
      - 1
      - 该输入使用一个调谐器（RF 解调器）    - - `V4L2_INPUT_TYPE_CAMERA`
      - 2
      - 任何非调谐器的视频输入，例如复合视频、S-Video、HDMI、摄像头传感器。命名为
	`_TYPE_CAMERA` 是历史原因，如今我们大概会把它叫`_TYPE_VIDEO`    - - `V4L2_INPUT_TYPE_TOUCH`
      - 3
      - 该输入是一个用于采集原始触摸数据的触摸设备


    :header-rows:  0
    :stub-columns: 0

    - - `2` General
    - - `V4L2_IN_ST_NO_POWER`
      - 0x00000001
      - 附属设备已关闭    - - `V4L2_IN_ST_NO_SIGNAL`
      - 0x00000002
      -
    - - `V4L2_IN_ST_NO_COLOR`
      - 0x00000004
      - 硬件支持彩色解码，但未在信号中检测到彩色调制    - - `2` Sensor Orientation
    - - `V4L2_IN_ST_HFLIP`
      - 0x00000010
      - 该输入连接到一个会产生水平翻转信号、且在将信号传给用户空间之前不予以纠正的设备    - - `V4L2_IN_ST_VFLIP`
      - 0x00000020
      - 该输入连接到一个会产生垂直翻转信号、且在将信号传给用户空间之前不予以纠正的设备    - - `2` Analog Video
    - - `V4L2_IN_ST_NO_H_LOCK`
      - 0x00000100
      - 无水平同步锁定    - - `V4L2_IN_ST_COLOR_KILL`
      - 0x00000200
      - 当检测到无彩色调制时，消色电路会自动禁用彩色解码。当设置此标志时，消色电路已启用
	并关闭了彩色解码    - - `V4L2_IN_ST_NO_V_LOCK`
      - 0x00000400
      - 无垂直同步锁定    - - `V4L2_IN_ST_NO_STD_LOCK`
      - 0x00000800
      - 在器件自动检测格式的情况下，无标准格式锁定    - - `2` Digital Video
    - - `V4L2_IN_ST_NO_SYNC`
      - 0x00010000
      - 无同步锁定    - - `V4L2_IN_ST_NO_EQU`
      - 0x00020000
      - 无均衡器锁定    - - `V4L2_IN_ST_NO_CARRIER`
      - 0x00040000
      - 载波恢复失败    - - `2` VCR and Set-Top Box
    - - `V4L2_IN_ST_MACROVISION`
      - 0x01000000
      - Macrovision 是一种模拟复制保护系统，它通过扰乱视频信号来迷惑录像机。当设置此标志时	表示已检测到 Macrovision    - - `V4L2_IN_ST_NO_ACCESS`
      - 0x02000000
      - 条件访问被拒绝    - - `V4L2_IN_ST_VTR`
      - 0x04000000
      - VTR 时间常数。[?]



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_IN_CAP_DV_TIMINGS`
      - 0x00000002
      - 该输入支持通过使用 `VIDIOC_S_DV_TIMINGS` 设置视频时序    - - `V4L2_IN_CAP_STD`
      - 0x00000004
      - 该输入支持通过使用 `VIDIOC_S_STD` 设置电视标准    - - `V4L2_IN_CAP_NATIVE_SIZE`
      - 0x00000008
      - 该输入支持使`V4L2_SEL_TGT_NATIVE_SIZE` 选择目标设置原生尺寸，见
	v4l2-selections-common銆。
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述
EINVAL
    结构`v4l2_input` `index` 越界