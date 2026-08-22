
######## 单平面（single-planar）格式结

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 以像素为单位的图像宽度    - - __u32
      - `height`
      - 以像素为单位的图像高度。如`field` `V4L2_FIELD_TOP`	`V4L2_FIELD_BOTTOM` `V4L2_FIELD_ALTERNATE` 之一，那height
	指的是该场（field）中的行数，否则它指的是帧中的行数（对于隔行
	格式而言，这是场高度的两倍）    - - `2` 应用程序设置这些字段以请求一个图像尺寸，驱动返回尽可能接近的
      值。对于平面（planar）格式，`width` `height` 适用于最大的
      平面。为避免歧义，驱动返回的值必须向上取整为任何较小平面比例因子      整数倍。例如，当图像格式为 YUV 4:2:0 时，`width` `height`
      必须2 的整数倍
	对于在码流内部编码了分辨率信息的压缩格式，当喂给一个有状态（stateful	mem2mem 解码器时，这些字段可以为零，以依赖解码器检测正确的取值。更	细节请参阅各解码器与格式描述
	对于有状mem2mem 编码CAPTURE 侧的压缩格式，这些字段必须为零，
	因为编码后大小预期由编码器自身基OUTPUT 侧在内部计算。更多细节请参阅
	各编码器与格式描述    - - __u32
      - `pixelformat`
      - 由应用程序设置的像素格式或压缩类型。这是一个小端序的四字符	<v4l2-fourcc>。V4L2 pixfmt-rgb 中定义标RGB 格式，在
	yuv-formats 中定YUV 格式，在 reserved-formats 中定义保留的
	编码    - - __u32
      - `field`
      - 场顺序，来自枚举 `v4l2_field`。视频图像通常是隔行的。应用程序可	请求只捕获或输出顶场或底场，或者把两个场隔行或顺序地存放在一个缓冲区
	中，或者交替存放在不同的缓冲区中。驱动返回实际选定的场顺序。关于场	更多细节请参field-order    - - __u32
      - `bytesperline`
      - 两相邻行中最左侧像素之间的字节距离    - - `2`

	应用程序和驱动都可以设置这个字段，以请求在每行末尾添加填充（padding	字节。然而驱动可能会忽略应用程序请求的值，返回 `width` 乘以每像素字节数	或硬件所需的更大值。这意味着应用程序只需把这个字段设0 即可获得一	合理的默认值
	视频硬件可能会访问填充字节，因此它们必须位于可访问的内存中。要考虑图像
	最后一行之后的填充字节跨越系统页边界的情况。输入设备可能会写入填充字节	其值未定义。输出设备忽略填充字节的内容
	当图像格式为平面格式时，`bytesperline` 值适用于第一个平面，并为其他平面
	除以`width` 字段相同的因子。例YUV 4:2:0 图像Cb Cr 平面	每行之后的填充字节数量是 Y 平面的一半。为避免歧义，驱动必须返回一个向	取整到比例因子整数倍的 `bytesperline` 值
	对于压缩格式，`bytesperline` 值没有意义。在这种情况下，应用程序和驱	都必须把它设0    - - __u32
      - `sizeimage`
      - 保存一整幅图像所需的缓冲区大小（字节数），由驱动设置。通常这是
	`bytesperline` 乘以 `height`。当图像由变长压缩数据组成时，这是编解码	支持最坏情况压缩场景所需的字节数
	驱动会为未压缩图像设置这个值
	客户端允许为VIDIOC_ENUM_FMT 处标记为 `V4L2_FMT_FLAG_COMPRESSED`
	的变长压缩数据设sizeimage 字段，但驱动可能忽略它并自行设置该值，	者基于对齐要求或最最大尺寸要求修改所提供的值。如果客户端想把这件	留给驱动，则应当sizeimage 设为 0    - - __u32
      - `colorspace`
      - 图像色彩空间，来自枚`v4l2_colorspace`。该信息是对 `pixelformat`
	的补充，对于捕获流必须由驱动设置，对于输出流必须由应用程序设置，参见
	colorspaces。如果应用程序设置了标志 `V4L2_PIX_FMT_FLAG_SET_CSC`，那	应用程序可以为捕获流设置这个字段，以请求对捕获图像数据使用特定的色彩
	空间。如果驱动无法处理所请求的转换，它将返回另一个受支持的色彩空间	驱动通过在枚举时于对应的 struct `v4l2_fmtdesc` 中设置标	V4L2_FMT_FLAG_CSC_COLORSPACE 来表明支持色彩空间转换。参fmtdesc-flags    - - __u32
      - `priv`
      - 这个字段表示 struct `v4l2_pix_format` 的其余字段（也称为扩展字段）
	是否有效。当设置`V4L2_PIX_FMT_PRIV_MAGIC` 时，表示扩展字段已被正确
	初始化。当设置为任何其他值时，表示扩展字段包含未定义的值
	想要使用像素格式扩展字段的应用程序，必须先通过向设备查	V4L2_CAP_EXT_PIX_FORMAT <querycap> 能力来确认该功能受支持。如果该能力
	未被设置，则像素格式扩展字段不受支持，使用扩展字段将导致未定义的结果
	要使用扩展字段，应用程序必须`priv` 字段设为
	`V4L2_PIX_FMT_PRIV_MAGIC`，初始化所有扩展字段，并把 struct `v4l2_format`
	`raw_data` 字段中未使用的字节清零
	`priv` 字段未被设为 `V4L2_PIX_FMT_PRIV_MAGIC` 时，驱动必须表现得像
	所有扩展字段都被设0 一样。返回时，驱动必须把 `priv` 字段设为
	`V4L2_PIX_FMT_PRIV_MAGIC`，并把所有扩展字段设为适用的取值    - - __u32
      - `flags`
      - 由应用程序或驱动设置的标志，参见 format-flags    - - union {
      - (anonymous)
    - - __u32
      - `ycbcr_enc`
      - Y'CbCr 编码，来自枚`v4l2_ycbcr_encoding`。该信息是对
	`colorspace` 的补充，对于捕获流必须由驱动设置，对于输出流必须由应用程	设置，参colorspaces。如果应用程序设置了标志
	`V4L2_PIX_FMT_FLAG_SET_CSC`，那么应用程序可以为捕获流设置这个字段，	请求对捕获图像数据使用特定的 Y'CbCr 编码。如果驱动无法处理所请求	转换，它将返回另一个受支持的编码。这个字段对HSV 像素格式会被忽略	驱动通过在枚举时于对应的 struct `v4l2_fmtdesc` 中设置标	V4L2_FMT_FLAG_CSC_YCBCR_ENC 来表明支ycbcr_enc 转换。参fmtdesc-flags    - - __u32
      - `hsv_enc`
      - HSV 编码，来自枚`v4l2_hsv_encoding`。该信息是对 `colorspace` 	补充，对于捕获流必须由驱动设置，对于输出流必须由应用程序设置，参	colorspaces。如果应用程序设置了标志 `V4L2_PIX_FMT_FLAG_SET_CSC`	那么应用程序可以为捕获流设置这个字段，以请求对捕获图像数据使用特定的
	HSV 编码。如果驱动无法处理所请求的转换，它将返回另一个受支持的编码	这个字段对于HSV 像素格式会被忽略。驱动通过在枚举时于对应的 struct
	`v4l2_fmtdesc` 中设置标V4L2_FMT_FLAG_CSC_HSV_ENC 来表明支hsv_enc
	转换。参fmtdesc-flags    - - }
      -
    - - __u32
      - `quantization`
      - 量化范围，来自枚`v4l2_quantization`。该信息是对 `colorspace` 	补充，对于捕获流必须由驱动设置，对于输出流必须由应用程序设置，参	colorspaces。如果应用程序设置了标志 `V4L2_PIX_FMT_FLAG_SET_CSC`，那	应用程序可以为捕获流设置这个字段，以请求对捕获图像数据使用特定的量化
	范围。如果驱动无法处理所请求的转换，它将返回另一个受支持的量化。驱	通过在枚举时于对应的 struct `v4l2_fmtdesc` 中设置标	V4L2_FMT_FLAG_CSC_QUANTIZATION 来表明支持量化转换。参fmtdesc-flags    - - __u32
      - `xfer_func`
      - 传输函数，来自枚`v4l2_xfer_func`。该信息是对 `colorspace` 的补充，
	对于捕获流必须由驱动设置，对于输出流必须由应用程序设置，参见 colorspaces	如果应用程序设置了标`V4L2_PIX_FMT_FLAG_SET_CSC`，那么应用程序可以为
	捕获流设置这个字段，以请求对捕获图像数据使用特定的传输函数。如果驱	无法处理所请求的转换，它将返回另一个受支持的传输函数。驱动通过在枚举时
	于对应的 struct `v4l2_fmtdesc` 中设置标V4L2_FMT_FLAG_CSC_XFER_FUNC
	来表明支xfer_func 转换。参fmtdesc-flags


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_PIX_FMT_FLAG_PREMUL_ALPHA`
      - 0x00000001
      - 颜色值已经预先乘以了 alpha 通道的值。例如，如果一50% 透明的浅蓝色
        像素RGBA (128, 192, 255, 128) 描述，那么用预乘颜色描述的同一	像素则为 RGBA (64, 96, 128, 128)    - .. _`v4l2-pix-fmt-flag-set-csc`:

      - `V4L2_PIX_FMT_FLAG_SET_CSC`
      - 0x00000002
      - 由应用程序设置。它仅用于捕获流，对输出流会被忽略。如果设置，则请        设备把接收到的色彩空间转换为所请求的色彩空间取值。如果色度学字段
	（`colorspace`、`xfer_func`、`ycbcr_enc`、`hsv_enc` `quantization`	被设`*_DEFAULT`，那么该色度学设置将保持与接收到时不变。因此，为了
	改变量化，只需`quantization` 字段设为非默认	（`V4L2_QUANTIZATION_FULL_RANGE` `V4L2_QUANTIZATION_LIM_RANGE`），	所有其他色度学字段都应设为 `*_DEFAULT`
	要查看当前像素格式下硬件支持哪些转换，请参阅 fmtdesc-flags