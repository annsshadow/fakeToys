

######## ioctl VIDIOC_G_DV_TIMINGS, VIDIOC_S_DV_TIMINGS


## 名称


VIDIOC_G_DV_TIMINGS - VIDIOC_S_DV_TIMINGS - VIDIOC_SUBDEV_G_DV_TIMINGS - VIDIOC_SUBDEV_S_DV_TIMINGS - 获取或设置输输出DV 时序


## 概要



`int ioctl(int fd, VIDIOC_G_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_S_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_DV_TIMINGS, struct v4l2_dv_timings *argp)`


## 参数



`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_dv_timings` 的指针
## 描述


要为输入或输出设DV 时序，应用程序使VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl而要获取当前时序，应用程序使VIDIOC_G_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl。详细的
时序信息使用 struct `v4l2_dv_timings` 结构体填充。这ioctl 以指struct
`v4l2_dv_timings` 结构体的指针作为参数。如ioctl 不被支持或时序值不正确，驱动将
返回 `EINVAL` 错误码
在以只读模式注册的子设备（subdev）设备节点上调用 `VIDIOC_SUBDEV_S_DV_TIMINGS` 是不允许的此时会返回错误，errno 变量被设`-EPERM`
`linux/v4l2-dv-timings.h` 头文件可用于获取 cea861 vesadmt 标准中各个格式的时序。如当前的输入或输出不支DV 时序（例VIDIOC_ENUMINPUT 没有设置
`V4L2_IN_CAP_DV_TIMINGS` 标志），则返`ENODATA` 错误码
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述
EINVAL
    ioctl 不被支持，或VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> 参数不合适
ENODATA
    该输入或输出不支持数字视频时序
EBUSY
    设备正忙，因此无法更改时序
EPERM
    `VIDIOC_SUBDEV_S_DV_TIMINGS` 在被调用的只读子设备上被调用



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 有效视频的宽度，以像素为单位    - - __u32
      - `height`
      - 有效视频帧的高度，以行为单位。因此对于隔行（interlaced）格式，
	每个场（field）的有效视频高度`height`/2    - - __u32
      - `interlaced`
      - 逐行（progressive，`V4L2_DV_PROGRESSIVE`）或隔行
	（interlaced，`V4L2_DV_INTERLACED`）    - - __u32
      - `polarities`
      - 一个位掩码，定义同步信号的极性。位 0（`V4L2_DV_VSYNC_POS_POL`	对应垂直同步极性，1（`V4L2_DV_HSYNC_POS_POL`）对应水平同步极性	若某位被置位）则为正极性，被清零（0）则为负极性    - - __u64
      - `pixelclock`
      - 像素时钟，单位为 Hz。例74.25MHz->74250000
    - - __u32
      - `hfrontporch`
      - 水平前肩（horizontal front porch），以像素为单位
    - - __u32
      - `hsync`
      - 水平同步长度，以像素为单    - - __u32
      - `hbackporch`
      - 水平后肩（horizontal back porch），以像素为单位
    - - __u32
      - `vfrontporch`
      - 垂直前肩（vertical front porch），以行为单位。对于隔行格式，这指的是
	奇数场（aka field 1，即1）    - - __u32
      - `vsync`
      - 垂直同步长度，以行为单位。对于隔行格式，这指的是奇数场（aka field 1）    - - __u32
      - `vbackporch`
      - 垂直后肩（vertical back porch），以行为单位。对于隔行格式，这指的是
	奇数场（aka field 1）    - - __u32
      - `il_vfrontporch`
      - 隔行场格式中偶数场（aka field 2，即2）的垂直前肩，以行为单位	对于逐行格式必须0    - - __u32
      - `il_vsync`
      - 隔行场格式中偶数场（aka field 2）的垂直同步长度，以行为为单位	对于逐行格式必须0    - - __u32
      - `il_vbackporch`
      - 隔行场格式中偶数场（aka field 2）的垂直后肩，以行为单位	对于逐行格式必须0    - - __u32
      - `standards`
      - 该格式所属的视频标准（可以多个）。这由驱动填充。应用程序必须将
	其设0。标准列表见 dv-bt-standards    - - __u32
      - `flags`
      - 提供关于该格式更多信息的若干标志。各标志的说明见 dv-bt-flags    - - struct `v4l2_fract`
      - `picture_aspect`
      - 当像素不是正方形时的画面宽高比。仅`V4L2_DV_FL_HAS_PICTURE_ASPECT`
	标志被设置时有效    - - __u8
      - `cea861_vic`
      - 依据 CEA-861 标准的视频识别码（Video Identification Code）	仅当 `V4L2_DV_FL_HAS_CEA861_VIC` 标志被设置时有效    - - __u8
      - `hdmi_vic`
      - 依据 HDMI 标准的视频识别码。仅`V4L2_DV_FL_HAS_HDMI_VIC` 标志
	被设置时有效    - - __u8
      - `reserved[^46^]`
      - 保留供将来扩展使用。驱动和应用程序必须将该数组置零


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - DV 时序的类型，dv-timing-types 中的列表    - - union {
      - (anonymous)
    - - struct `v4l2_bt_timings`
      - `bt`
      - BT.656/1120 规范定义的时    - - __u32
      - `reserved`\ [^32^]
      -
    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - Timing type
      - value
      - Description
#     * -

      -
    - - `V4L2_DV_BT_656_1120`
      - 0
      - BT.656/1120 时序



    :header-rows:  0
    :stub-columns: 0

    - - Timing standard
      - Description
    - - `V4L2_DV_BT_STD_CEA861`
      - 时序遵循 CEA-861 数字电视（Digital TV）Profile 标准
    - - `V4L2_DV_BT_STD_DMT`
      - 时序遵循 VESA 离散监视器时序（Discrete Monitor Timings）标    - - `V4L2_DV_BT_STD_CVT`
      - 时序遵循 VESA 协调视频时序（Coordinated Video Timings）标    - - `V4L2_DV_BT_STD_GTF`
      - 时序遵循 VESA 通用时序公式（Generalized Timings Formula）标    - - `V4L2_DV_BT_STD_SDI`
      - 时序遵循 SDI 时序标准	该格式完全没有水平同肩（syncs/porches）	总的消隐（blanking）时序必须只设置hsync vsync 字段中


    :header-rows:  0
    :stub-columns: 0

    - - Flag
      - Description
    - - `V4L2_DV_FL_REDUCED_BLANKING`
      - CVT/GTF 专用：时序使用缩减消隐（reduced blanking，CVT）或
	“次GTF”（'Secondary GTF'）曲线（GTF）。两种情况下水平
	或垂直消隐间隔都被缩减，从而允许在相同的带宽下获得更高的分辨率	这是一个只读标志，应用程序不得设置它    - - `V4L2_DV_FL_CAN_REDUCE_FPS`
      - CEA-861 专用：针对帧率为 6 的倍数CEA-861 格式设置。这些格式可	选择性地1 / 1.001 的速度播放，以兼容使用 29.97 秒帧率的
	基于 60 Hz 的标准，NTSC PAL-M。如果发送端无法产生这样的频率，
	该标志也会被清零。这是一个只读标志，应用程序不得设置它    - - `V4L2_DV_FL_REDUCED_FPS`
      - CEA-861 专用：仅对设置了 `V4L2_DV_FL_CAN_DETECT_REDUCED_FPS` 	视频发送端或视频接收端有效。否则该标志会被清零。它也仅对设置了
	`V4L2_DV_FL_CAN_REDUCE_FPS` 标志的格式有效，对于其他格式该标	会被驱动清零
	如果应用程序为发送端设置该标志，那么用于设置发送端的像素时钟会除以
	1.001，以兼容 NTSC 帧率。如果发送端无法产生这样的频率，该标志会被清零
	如果视频接收端检测到该格式使用了缩减的帧率，则会设置该标志以向应用程	发出信号    - - `V4L2_DV_FL_HALF_LINE`
      - 隔行格式专用：若设置，则1（aka 奇数场）的垂直前肩实际上多半个行长，
	而场 2（aka 偶数场）的垂直后肩实际上少半个行长，因此每个场恰好具	相同数量的半行。能否检测或使用半行取决于硬件    - - `V4L2_DV_FL_IS_CE_VIDEO`
      - 若设置，则这是一个消费电子（Consumer Electronics，CE）视频格式。这类格	与其他格式（通常称为 IT 格式）的不同之处在于：如果使R'G'B' 编码	默认情况R'G'B' 值使用受限范围（16-235），而非全范围（0-255）	CEA-861 中定义的所有格式（640x480p59.94 格式除外）都CE 格式    - - `V4L2_DV_FL_FIRST_FIELD_EXTRA_LINE`
      - 某些格式（如 SMPTE-125M）具有奇数总高度的隔行信号。对于这些格式，如果
	设置了该标志，则多余的行属于第一个场；否则属于第二个场    - - `V4L2_DV_FL_HAS_PICTURE_ASPECT`
      - 若设置，picture_aspect 字段有效。否则假定像素为正方形，因此画面宽高	与宽高比相同    - - `V4L2_DV_FL_HAS_CEA861_VIC`
      - 若设置，cea861_vic 字段有效，并包含按照 CEA-861 标准的视频识别码    - - `V4L2_DV_FL_HAS_HDMI_VIC`
      - 若设置，hdmi_vic 字段有效，并包含按照 HDMI 标准的视频识别码
	（HDMI Vendor Specific InfoFrame）    - - `V4L2_DV_FL_CAN_DETECT_REDUCED_FPS`
      - CEA-861 专用：仅对视频接收端有效，该标志由发送端清零。若设置，则硬件
	能够检测常规帧率与1000/1001 缩减的帧率之间的差异。例如：60 59.94 Hz	30 29.97 Hz，或 24 23.976 Hz