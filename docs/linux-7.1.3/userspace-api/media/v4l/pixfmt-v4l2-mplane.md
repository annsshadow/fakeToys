######## 多平面格式结构体


struct `v4l2_plane_pix_format` 结构体定义了多平面格式中每个平面大小和布局。struct `v4l2_pix_format_mplane` 结构体包所有平面共有的信息（如图像宽度和高度），以及一struct
`v4l2_plane_pix_format` 结构体数组，描述该格式的所有平面


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `sizeimage`
      - 该平面中图像数据所需的最大字节数，由驱动设置。当图像由变	压缩数据组成时，这是编解码器支持最坏情况压缩场景所需的字节数
	驱动将为此类未压缩图像设置该值
	客户端允许为VIDIOC_ENUM_FMT 处被
	`V4L2_FMT_FLAG_COMPRESSED` 标记的变长压缩数据设	sizeimage 字段，但驱动可以忽略它并自行设置该值，或者根	对齐要求或最最大尺寸要求修改所提供的值。如果客户端希望
	将此交由驱动处理，则应把 sizeimage 设为 0    - - __u32
      - `bytesperline`
      - 两条相邻行中最左侧像素之间的字节距离。见 struct
	`v4l2_pix_format`銆?    - - __u16
      - `reserved[^6^]`
      - 为未来扩展保留。驱动和应用程序应将其置零


    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 图像宽度（像素）。见 struct
	`v4l2_pix_format`銆?    - - __u32
      - `height`
      - 图像高度（像素）。见 struct
	`v4l2_pix_format`銆?    - - __u32
      - `pixelformat`
      - 像素格式。可以使用单平面和多平面四字符码    - - __u32
      - `field`
      - 场顺序，来自枚举 `v4l2_field`        struct `v4l2_pix_format`    - - __u32
      - `colorspace`
      - 色彩空间编码，来自枚`v4l2_colorspace`        struct `v4l2_pix_format`    - - struct `v4l2_plane_pix_format`
      - `plane_fmt[VIDEO_MAX_PLANES]`
      - 描述此像素格式所包含的每个平面的结构体数组。该数组中有	条目的数量必须放`num_planes` 字段    - - __u8
      - `num_planes`
      - 该格式的平面数（即独立的内存缓冲区），以`plane_fmt`
	数组中有效条目的数量    - - __u8
      - `flags`
      - 由应用程序或驱动设置的标志，format-flags    - - union {
      - (匿名)
    - - __u8
      - `ycbcr_enc`
      - Y'CbCr 编码，来自枚`v4l2_ycbcr_encoding`	struct `v4l2_pix_format`    - - __u8
      - `hsv_enc`
      - HSV 编码，来自枚`v4l2_hsv_encoding`	struct `v4l2_pix_format`    - - }
      -
    - - __u8
      - `quantization`
      - 量化范围，来自枚`v4l2_quantization`	struct `v4l2_pix_format`    - - __u8
      - `xfer_func`
      - 传输函数，来自枚`v4l2_xfer_func`	struct `v4l2_pix_format`    - - __u8
      - `reserved[^7^]`
      - 为未来扩展保留。驱动和应用程序应将其置零

    \normalsize
