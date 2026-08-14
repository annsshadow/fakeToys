


######## 打包（Packed）YUV 格式


与打包 RGB 格式类似，打包 YUV 格式将 Y、Cb 和 Cr 分量在内存中连续存储。它们可能对色度分量应用子采样，因此在交错这三个分量的方式上有所不同。

   - 在以下所有表格中，位 7 是一个字节中最高有效位。
   - “Y”、“Cb”和“Cr”分别表示亮度、蓝色色度（也称为“U”）和红色色度（也称为“V”）分量的位。“A”表示 alpha 分量的位（如果格式支持），“X”表示填充位。

## 4:4:4 子采样


这些格式不对色度分量进行子采样，并将每个像素存储为一个由 Y、Cb 和 Cr 值组成的完整三元组。

下一个表格列出了每个分量少于 8 位的打包 YUV 4:4:4 格式。它们根据在 16 位字中看到的 Y、Cb 和 Cr 分量顺序（随后以 little endian 字节序存入内存）以及每个分量的位数来命名。例如，YUV565 格式将一个像素存储在一个 16 位字 [15:0] 中，布局为 [Y'\ `4-0` Cb\ `5-0` Cr\ `4-0`]，并以两个字节存入内存，[Cb\ `2-0` Cr\ `4-0`] 在前，接着是 [Y'\ `4-0` Cb\ `5-3`]。

    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}


    :header-rows:  2
    :stub-columns: 0

    - - Identifier
      - Code

      - `7` Byte 0 in memory

      - `7` Byte 1

#     * -

      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0

      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0

    - .. _V4L2-PIX-FMT-YUV444:

      - `V4L2_PIX_FMT_YUV444`
      - 'Y444'

      - Cb\ `3`
      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - a\ `3`
      - a\ `2`
      - a\ `1`
      - a\ `0`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`

    - .. _V4L2-PIX-FMT-YUV555:

      - `V4L2_PIX_FMT_YUV555`
      - 'YUVO'

      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `4`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - a
      - Y'\ `4`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`
      - Cb\ `4`
      - Cb\ `3`

    - .. _V4L2-PIX-FMT-YUV565:

      - `V4L2_PIX_FMT_YUV565`
      - 'YUVP'

      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `4`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - Y'\ `4`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`
      - Cb\ `5`
      - Cb\ `4`
      - Cb\ `3`


    \endgroup


    对于 YUV444 和 YUV555 格式，alpha 位的值在从驱动读取时未定义，在写入驱动时被忽略，除非针对 :ref:`视频叠加 <overlay>` 或视频输出叠加 <osd> 协商了 alpha 混合。

下一个表格列出了每个分量 8 位的打包 YUV 4:4:4 格式。它们根据在内存中存储的 Y、Cb 和 Cr 分量顺序以及每像素的总位数来命名。例如，VUYX32 格式将像素的 Cr\ `7-0` 存储在第一个字节、Cb\ `7-0` 存储在第二个字节、Y'\ `7-0` 存储在第三个字节。

    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3

    - .. _V4L2-PIX-FMT-YUV32:

      - `V4L2_PIX_FMT_YUV32`
      - 'YUV4'

      - A\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-AYUV32:

      - `V4L2_PIX_FMT_AYUV32`
      - 'AYUV'

      - A\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-XYUV32:

      - `V4L2_PIX_FMT_XYUV32`
      - 'XYUV'

      - X\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-VUYA32:

      - `V4L2_PIX_FMT_VUYA32`
      - 'VUYA'

      - Cr\ `7-0`
      - Cb\ `7-0`
      - Y'\ `7-0`
      - A\ `7-0`

    - .. _V4L2-PIX-FMT-VUYX32:

      - `V4L2_PIX_FMT_VUYX32`
      - 'VUYX'

      - Cr\ `7-0`
      - Cb\ `7-0`
      - Y'\ `7-0`
      - X\ `7-0`

    - .. _V4L2-PIX-FMT-YUVA32:

      - `V4L2_PIX_FMT_YUVA32`
      - 'YUVA'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - A\ `7-0`

    - .. _V4L2-PIX-FMT-YUVX32:

      - `V4L2_PIX_FMT_YUVX32`
      - 'YUVX'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - X\ `7-0`

    - .. _V4L2-PIX-FMT-YUV24:

      - `V4L2_PIX_FMT_YUV24`
      - 'YUV3'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - -\

    - alpha 分量应包含一个对驱动和应用程序有意义的值。
    - 填充位包含未定义的值，必须被所有应用程序和驱动忽略。

下一个表格列出了每个分量 12 位的打包 YUV 4:4:4 格式。将每个分量扩展到 16 位，数据放在高字节，低字节补零，按 little endian 顺序排列，用 6 个字节存储 1 个像素。

    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 1-0
      - Byte 3-2
      - Byte 5-4
      - Byte 7-6
      - Byte 9-8
      - Byte 11-10

    - .. _V4L2-PIX-FMT-YUV48-12:

      - `V4L2_PIX_FMT_YUV48_12`
      - 'Y312'

      - Y'\ `0`
      - Cb\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `1`
      - Cr\ `1`

## 4:2:2 子采样


这些通常称为 YUYV 或 YUY2 的格式，将色度分量水平子采样 2 倍，在容器中存储 2 个像素。8 位格式的容器为 32 位，10 位及以上格式的容器为 64 位。

每个分量多于 8 位的打包 YUYV 格式存储为四个 16 位的 little endian 字。每个字的最高有效位包含一个分量，最低有效位为零填充。


    \footnotesize


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3
      - Byte 4
      - Byte 5
      - Byte 6
      - Byte 7
    - .. _V4L2-PIX-FMT-UYVY:

      - `V4L2_PIX_FMT_UYVY`
      - 'UYVY'

      - Cb\ `0`
      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `2`
      - Y'\ `2`
      - Cr\ `2`
      - Y'\ `3`
    - .. _V4L2-PIX-FMT-VYUY:

      - `V4L2_PIX_FMT_VYUY`
      - 'VYUY'

      - Cr\ `0`
      - Y'\ `0`
      - Cb\ `0`
      - Y'\ `1`
      - Cr\ `2`
      - Y'\ `2`
      - Cb\ `2`
      - Y'\ `3`
    - .. _V4L2-PIX-FMT-YUYV:

      - `V4L2_PIX_FMT_YUYV`
      - 'YUYV'

      - Y'\ `0`
      - Cb\ `0`
      - Y'\ `1`
      - Cr\ `0`
      - Y'\ `2`
      - Cb\ `2`
      - Y'\ `3`
      - Cr\ `2`
    - .. _V4L2-PIX-FMT-YVYU:

      - `V4L2_PIX_FMT_YVYU`
      - 'YVYU'

      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `0`
      - Y'\ `2`
      - Cr\ `2`
      - Y'\ `3`
      - Cb\ `2`


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Word 0
      - Word 1
      - Word 2
      - Word 3
    - .. _V4L2-PIX-FMT-Y210:

      - `V4L2_PIX_FMT_Y210`
      - 'Y210'

      - Y'\ `0` (bits 15-6)
      - Cb\ `0` (bits 15-6)
      - Y'\ `1` (bits 15-6)
      - Cr\ `0` (bits 15-6)
    - .. _V4L2-PIX-FMT-Y212:

      - `V4L2_PIX_FMT_Y212`
      - 'Y212'

      - Y'\ `0` (bits 15-4)
      - Cb\ `0` (bits 15-4)
      - Y'\ `1` (bits 15-4)
      - Cr\ `0` (bits 15-4)
    - .. _V4L2-PIX-FMT-Y216:

      - `V4L2_PIX_FMT_Y216`
      - 'Y216'

      - Y'\ `0` (bits 15-0)
      - Cb\ `0` (bits 15-0)
      - Y'\ `1` (bits 15-0)
      - Cr\ `0` (bits 15-0)


    \normalsize

**颜色采样位置：**
色度样本在水平方向上居中插于其间<yuv-chroma-centered>。

## 4:1:1 子采样


此格式将色度分量水平子采样 4 倍，用 12 个字节存储 8 个像素。


    \scriptsize


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3
      - Byte 4
      - Byte 5
      - Byte 6
      - Byte 7
      - Byte 8
      - Byte 9
      - Byte 10
      - Byte 11
    - .. _V4L2-PIX-FMT-Y41P:

      - `V4L2_PIX_FMT_Y41P`
      - 'Y41P'

      - Cb\ `0`
      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `4`
      - Y'\ `2`
      - Cr\ `4`
      - Y'\ `3`
      - Y'\ `4`
      - Y'\ `5`
      - Y'\ `6`
      - Y'\ `7`


    \normalsize


    不要将 `V4L2_PIX_FMT_Y41P` 与
    V4L2_PIX_FMT_YUV411P <V4L2-PIX-FMT-YUV411P> 混淆。Y41P 派生自
    “YUV 4:1:1 **packed**”（打包），而 YUV411P 代表 “YUV 4:1:1 **planar**”（平面）。

**颜色采样位置：**
色度样本在水平方向上居中插于其间<yuv-chroma-centered>。
