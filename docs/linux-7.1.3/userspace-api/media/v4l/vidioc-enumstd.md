

######## ioctl VIDIOC_ENUMSTD, VIDIOC_SUBDEV_ENUMSTD


## Name


VIDIOC_ENUMSTD - VIDIOC_SUBDEV_ENUMSTD - 枚举支持的视频标准

## Synopsis



`int ioctl(int fd, VIDIOC_ENUMSTD, struct v4l2_standard *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_ENUMSTD, struct v4l2_standard *argp)`

## Arguments


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_standard` 的指针。

## Description


为了查询某个视频标准的属性，尤其是自定义的（由驱动定义的）标准，应用程序初始化
struct `v4l2_standard` 的 `index` 字段，并使用指向该结构的指针调用 VIDIOC_ENUMSTD
ioctl。驱动填充结构的其余部分，或者当 index 越界时返回 `EINVAL` 错误码。为了枚举
所有标准，应用程序应从 index 0 开始，每次加 1，直到驱动返回 `EINVAL`。驱动在切换
视频输入或输出后，可能会枚举出一组不同的标准。[#f1]_



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 视频标准的编号，由应用程序设置。
    - - v4l2_std_id <v4l2-std-id>
      - `id`
      - 该字段中的位将标准标识为 v4l2-std-id 中列出的常见标准之一，或者，如果
	第 32 至 63 位被置位，则标识为自定义标准。如果硬件无法区分这些标准，可以
	设置多个位；不过独立的 index 并不表示相反的情况。`id` 必须是唯一的。对于
	该输入或输出，任何其他被枚举的 struct `v4l2_standard` 结构都不能包含相同的
	位集合。
    - - __u8
      - `name`\ [^24^]
      - 标准的名称，一个以 NUL 结尾的 ASCII 字符串，例如：“PAL-B/G”、“NTSC Japan”。
	此信息供用户使用。
    - - struct `v4l2_fract`
      - `frameperiod`
      - 帧周期（而非场周期）为 numerator / denominator。例如 M/NTSC 的帧周期为
	1001 / 30000 秒。
    - - __u32
      - `framelines`
      - 每帧的总行数，含消隐，例如 B/PAL 为 625。
    - - __u32
      - `reserved`\ [^4^]
      - 为未来扩展保留。驱动必须将该数组置为零。




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `numerator`
      -
    - - __u32
      - `denominator`
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `v4l2_std_id`
      - 该类型是一个集合，每一位代表下面以及 video-standards 中所列出的另一个视频
	标准。最高的 32 位保留给自定义（驱动定义的）视频标准。



    #define V4L2_STD_PAL_B          ((v4l2_std_id)0x00000001)
    #define V4L2_STD_PAL_B1         ((v4l2_std_id)0x00000002)
    #define V4L2_STD_PAL_G          ((v4l2_std_id)0x00000004)
    #define V4L2_STD_PAL_H          ((v4l2_std_id)0x00000008)
    #define V4L2_STD_PAL_I          ((v4l2_std_id)0x00000010)
    #define V4L2_STD_PAL_D          ((v4l2_std_id)0x00000020)
    #define V4L2_STD_PAL_D1         ((v4l2_std_id)0x00000040)
    #define V4L2_STD_PAL_K          ((v4l2_std_id)0x00000080)

    #define V4L2_STD_PAL_M          ((v4l2_std_id)0x00000100)
    #define V4L2_STD_PAL_N          ((v4l2_std_id)0x00000200)
    #define V4L2_STD_PAL_Nc         ((v4l2_std_id)0x00000400)
    #define V4L2_STD_PAL_60         ((v4l2_std_id)0x00000800)

`V4L2_STD_PAL_60` 是一种混合标准，具有 525 行、60 Hz 刷新率，以及使用 4.43 MHz
色度副载波的 PAL 彩色调制。某些 PAL 录像机可在该模式下回放 NTSC 磁带，以便在
50/60 Hz 无关的 PAL 电视上显示。


    #define V4L2_STD_NTSC_M         ((v4l2_std_id)0x00001000)
    #define V4L2_STD_NTSC_M_JP      ((v4l2_std_id)0x00002000)
    #define V4L2_STD_NTSC_443       ((v4l2_std_id)0x00004000)

`V4L2_STD_NTSC_443` 是一种混合标准，具有 525 行、60 Hz 刷新率，以及使用 4.43 MHz
色度副载波的 NTSC 彩色调制。


    #define V4L2_STD_SECAM_B        ((v4l2_std_id)0x00010000)
    #define V4L2_STD_SECAM_D        ((v4l2_std_id)0x00020000)
    #define V4L2_STD_SECAM_G        ((v4l2_std_id)0x00040000)
    #define V4L2_STD_SECAM_H        ((v4l2_std_id)0x00080000)
    #define V4L2_STD_SECAM_K        ((v4l2_std_id)0x00100000)
    #define V4L2_STD_SECAM_K1       ((v4l2_std_id)0x00200000)
    #define V4L2_STD_SECAM_L        ((v4l2_std_id)0x00400000)
    #define V4L2_STD_SECAM_LC       ((v4l2_std_id)0x00800000)

    #define V4L2_STD_ATSC_8_VSB     ((v4l2_std_id)0x01000000)
    #define V4L2_STD_ATSC_16_VSB    ((v4l2_std_id)0x02000000)

    /** ATSC/HDTV **/
    #define V4L2_STD_ATSC_8_VSB     ((v4l2_std_id)0x01000000)
    #define V4L2_STD_ATSC_16_VSB    ((v4l2_std_id)0x02000000)

`V4L2_STD_ATSC_8_VSB` 和 `V4L2_STD_ATSC_16_VSB` 是美国地面数字电视标准。目前
V4L2 API 不支持数字电视。另请参阅 `https://linuxtv.org <https://linuxtv.org>`__ 上的
Linux DVB API。


    #define V4L2_STD_PAL_BG         (V4L2_STD_PAL_B         |
		     V4L2_STD_PAL_B1        |
		     V4L2_STD_PAL_G)
    #define V4L2_STD_B              (V4L2_STD_PAL_B         |
		     V4L2_STD_PAL_B1        |
		     V4L2_STD_SECAM_B)
    #define V4L2_STD_GH             (V4L2_STD_PAL_G         |
		     V4L2_STD_PAL_H         |
		     V4L2_STD_SECAM_G       |
		     V4L2_STD_SECAM_H)
    #define V4L2_STD_PAL_DK         (V4L2_STD_PAL_D         |
		     V4L2_STD_PAL_D1        |
		     V4L2_STD_PAL_K)
    #define V4L2_STD_PAL            (V4L2_STD_PAL_BG        |
		     V4L2_STD_PAL_DK        |
		     V4L2_STD_PAL_H         |
		     V4L2_STD_PAL_I)
    #define V4L2_STD_NTSC           (V4L2_STD_NTSC_M        |
		     V4L2_STD_NTSC_M_JP     |
		     V4L2_STD_NTSC_M_KR)
    #define V4L2_STD_MN             (V4L2_STD_PAL_M         |
		     V4L2_STD_PAL_N         |
		     V4L2_STD_PAL_Nc        |
		     V4L2_STD_NTSC)
    #define V4L2_STD_SECAM_DK       (V4L2_STD_SECAM_D       |
		     V4L2_STD_SECAM_K       |
		     V4L2_STD_SECAM_K1)
    #define V4L2_STD_DK             (V4L2_STD_PAL_DK        |
		     V4L2_STD_SECAM_DK)

    #define V4L2_STD_SECAM          (V4L2_STD_SECAM_B       |
		     V4L2_STD_SECAM_G       |
		     V4L2_STD_SECAM_H       |
		     V4L2_STD_SECAM_DK      |
		     V4L2_STD_SECAM_L       |
		     V4L2_STD_SECAM_LC)

    #define V4L2_STD_525_60         (V4L2_STD_PAL_M         |
		     V4L2_STD_PAL_60        |
		     V4L2_STD_NTSC          |
		     V4L2_STD_NTSC_443)
    #define V4L2_STD_625_50         (V4L2_STD_PAL           |
		     V4L2_STD_PAL_N         |
		     V4L2_STD_PAL_Nc        |
		     V4L2_STD_SECAM)

    #define V4L2_STD_UNKNOWN        0
    #define V4L2_STD_ALL            (V4L2_STD_525_60        |
		     V4L2_STD_625_50)


    \begingroup
    \tiny
    \setlength{\tabcolsep}{2pt}



    :header-rows:  1
    :stub-columns: 0

    - - Characteristics
      - M/NTSC [#f2]_
      - M/PAL
      - N/PAL [#f3]_
      - B, B1, G/PAL
      - D, D1, K/PAL
      - H/PAL
      - I/PAL
      - B, G/SECAM
      - D, K/SECAM
      - K1/SECAM
      - L/SECAM
    - - Frame lines
      - `1` 525
      - `8` 625
    - - Frame period (s)
      - `1` 1001/30000
      - `8` 1/25
    - - Chrominance sub-carrier frequency (Hz)
      - 3579545 ± 10
      - 3579611.49 ± 10
      - 4433618.75 ± 5

	(3582056.25 ± 5)
      - `3` 4433618.75 ± 5
      - 4433618.75 ± 1
      - `2` f\ `OR` = 4406250 ± 2000,

	f\ `OB` = 4250000 ± 2000
    - - Nominal radio-frequency channel bandwidth (MHz)
      - 6
      - 6
      - 6
      - B: 7; B1, G: 8
      - 8
      - 8
      - 8
      - 8
      - 8
      - 8
      - 8
    - - Sound carrier relative to vision carrier (MHz)
      - 4.5
      - 4.5
      - 4.5
      - 5.5 ± 0.001  [#f4]_  [#f5]_  [#f6]_  [#f7]_
      - 6.5 ± 0.001
      - 5.5
      - 5.9996 ± 0.0005
      - 5.5 ± 0.001
      - 6.5 ± 0.001
      - 6.5
      - 6.5 [#f8]_


    \endgroup


## Return Value


成功时返回 0，出错时返回 -1 并设置 `errno`。通用错误码在 Generic Error Codes
<gen-errors> 一章中描述。

EINVAL
    struct `v4l2_standard` 的 `index` 越界。

ENODATA
    该输入或输出不支持标准视频时序。

   支持的标准可能相互重叠，我们需要一个明确的集合来查找由 VIDIOC_G_STD
   <VIDIOC_G_STD> 返回的当前标准。

   日本使用的标准类似于 M/NTSC（V4L2_STD_NTSC_M_JP）。

   括号中的值适用于被称为 N\ `C` 的 N/PAL 组合，用于阿根廷（V4L2_STD_PAL_Nc）。

   在德国、奥地利、意大利、荷兰、斯洛伐克和瑞士，使用双伴音载波系统，第二载波
   频率比第一伴音载波高 242.1875 kHz。澳大利亚使用类似的系统进行立体声广播。

   新西兰使用的伴音载波偏离图像载波 5.4996 ± 0.0005 MHz。

   在丹麦、芬兰、新西兰、瑞典和西班牙使用双伴音载波系统。冰岛、挪威和波兰正在
   引入相同系统。第二载波比图像载波高 5.85 MHz，采用 DQPSK 调制，带 728 kbit/s
   的伴音与数据复用。（NICAM 系统）

   在英国使用双伴音载波系统。第二伴音载波比图像载波高 6.552 MHz，采用 DQPSK 调制，
   带有能承载两个伴音声道的 728 kbit/s 伴音与数据复用。（NICAM 系统）

   在法国，除主伴音载波外，还可能使用一个偏离图像载波 5.85 MHz 的数字载波。它采用
   差分编码的 QPSK 调制，带有一个能承载两个伴音声道的 728 kbit/s 伴音与数据复用器。
   （NICAM 系统）
