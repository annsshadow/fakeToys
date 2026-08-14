

######## ioctl VIDIOC_G_SLICED_VBI_CAP


## 名称


VIDIOC_G_SLICED_VBI_CAP - 查询切片（sliced）VBI 能力

## 摘要


`int ioctl(int fd, VIDIOC_G_SLICED_VBI_CAP, struct v4l2_sliced_vbi_cap *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_sliced_vbi_cap` 的指针。

## 描述


为了查明切片 VBI 捕获或输出设备支持哪些数据服务，应用程序初始化 struct
`v4l2_sliced_vbi_cap` 的 `type` 字段，清除 `reserved` 数组，并调用
VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl。驱动填充其余字段，
如果切片 VBI API 不受支持或 `type` 无效，则返回 `EINVAL` 错误码。


    `type` 字段是在 Linux 2.6.19 中添加的，并且该 ioctl 从只读变更为了
    读写。


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 3 2 2 2

    - - __u16
      - `service_set`
      - `2` 驱动支持的所有数据服务的集合。

	等于 `service_lines` 数组所有元素的并集。
    - - __u16
      - `service_lines`\ [^2^][^24^]
      - `2` 此数组的每个元素包含一个数据服务集合，硬件可以在特定扫描
	行上查找或插入这些服务。数据服务在 vbi-services 中定义。
	数组索引映射到 ITU-R 行号\ [#f1]_，如下所示：
#     * -

      - 元素
      - 525 行系统
      - 625 行系统
#     * -

      - `service_lines`\ [^0^][^1^]
      - 1
      - 1
#     * -

      - `service_lines`\ [^0^][^23^]
      - 23
      - 23
#     * -

      - `service_lines`\ [^1^][^1^]
      - 264
      - 314
#     * -

      - `service_lines`\ [^1^][^23^]
      - 286
      - 336
    - -
#     * -

      - `2` 硬件每帧可以捕获或输出的 VBI 行数，或者它能在给定行上识别的
	服务数量可能是受限的。例如，在 PAL 第 16 行上，硬件可能能够查找
	VPS 或 Teletext 信号，但不能同时查找两者。应用程序可以使用
	VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl（如 sliced 中所述）来了解这些限制。
    - -
#     * -

      - `2` 驱动必须将 `service_lines` [^0^][^0^] 和
	`service_lines`\ [^1^][^0^] 设为零。
    - - __u32
      - `type`
      - 数据流的类型，见 `v4l2_buf_type`。应为
	`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` 或
	`V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`。
    - - __u32
      - `reserved`\ [^3^]
      - `2` 此数组为将来扩展保留。

	应用程序和驱动必须将其设为零。


   另见 vbi-525 和 vbi-625。


    \scriptsize



    :header-rows:  1
    :stub-columns: 0
    :widths:       2 1 1 2 2

    - - 符号
      - 值
      - 参考
      - 行，通常
      - 载荷
    - - `V4L2_SLICED_TELETEXT_B`（Teletext System B）
      - 0x0001
      - ets300706、

	itu653
      - PAL/SECAM 第 7-22 行、320-335 行（第二场 7-22 行）
      - 45 字节 Teletext 包的最后 42 字节，即不带时钟
	引导和成帧码，最低位（lsb）先传输。
    - - `V4L2_SLICED_VPS`
      - 0x0400
      - ets300231
      - PAL 第 16 行
      - 根据 ETS 300 231 图 9 的第 3 到 15 字节，最低位先传输。
    - - `V4L2_SLICED_CAPTION_525`
      - 0x1000
      - cea608
      - NTSC 第 21、284 行（第二场 21 行）
      - 传输顺序的两个字节，包括奇偶校验位，最低位先传输。
    - - `V4L2_SLICED_WSS_625`
      - 0x4000
      - en300294、

	itu1119
      - PAL/SECAM 第 23 行
      - 见下方的 v4l2-sliced-vbi-cap-wss-625-payload。
    - - `V4L2_SLICED_VBI_525`
      - 0x1000
      - `2` 适用于 525 行系统的服务集合。
    - - `V4L2_SLICED_VBI_625`
      - 0x4401
      - `2` 适用于 625 行系统的服务集合。



    \normalsize


#### V4L2_SLICED_VBI_CAP WSS_625 载荷


`V4L2_SLICED_WSS_625` 的载荷为：

	    +-----+------------------+-----------------------+
	    |字节 |        0         |           1           |
	    +-----+--------+---------+-----------+-----------+
	    |     | msb    | lsb     | msb       | lsb       |
	    |     +-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+
	    | 位 |7|6|5|4 | 3|2|1|0 | x|x|13|12 | 11|10|9|8 |
	    +-----+-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+


## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用的错误码
在通用错误码 <gen-errors> 章节中描述。

EINVAL
    `type` 字段中的值错误。
