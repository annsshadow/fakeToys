
######## JPEG 控制参考


JPEG 类包含 JPEG 编码器与解码器通用特性的控件。目前它包含实现了使用 Huffman 熵编码的
渐进式基线 DCT 压缩过程的编解码器特性。

## JPEG 控件 ID


`V4L2_CID_JPEG_CLASS (class)`
    JPEG 类描述符。对此控件调用 VIDIOC_QUERYCTRL 将返回该控件类的描述。

`V4L2_CID_JPEG_CHROMA_SUBSAMPLING (menu)`
    色度子采样因子描述输入图像的每个分量如何被采样，相对于每个空间维度中的最大采样率。
    更多细节请参见 itu-t81，第 A.1.1 节。`V4L2_CID_JPEG_CHROMA_SUBSAMPLING` 控件决定
    在将输入图像从 RGB 转换到 Y'CbCr 色彩空间后，Cb 与 Cr 分量如何被下采样。

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_444`
      - 无色度子采样，每个像素都有 Y、Cr 与 Cb 值。
    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_422`
      - 对 Cr、Cb 分量按因子 2 水平子采样。
    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_420`
      - 对 Cr、Cb 分量水平与垂直各子采样 2 倍。
    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_411`
      - 对 Cr、Cb 分量按因子 4 水平子采样。
    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_410`
      - 对 Cr、Cb 分量水平子采样 4 倍、垂直子采样 2 倍。
    - - `V4L2_JPEG_CHROMA_SUBSAMPLING_GRAY`
      - 仅使用亮度分量。

`V4L2_CID_JPEG_RESTART_INTERVAL (integer)`
    重启间隔决定了插入 RSTm 标记（m = 0..7）的间隔。这些标记的目的是额外地重新初始化
    编码器过程，以便独立地处理图像的块。对于无损压缩过程，重启间隔的单位是 MCU（最小编码单元），
    其值包含在 DRI（Define Restart Interval）标记中。如果 `V4L2_CID_JPEG_RESTART_INTERVAL`
    控件设为 0，则不会插入 DRI 与 RSTm 标记。

`V4L2_CID_JPEG_COMPRESSION_QUALITY (integer)`
    决定图像质量与大小之间的权衡。它为应用程序提供了一种更简单的控制图像质量的方法，
    而无需直接重新配置亮度与色度量化表。在驱动使用由应用程序通过其他定义的接口直接配置的
    量化表的情况下，`V4L2_CID_JPEG_COMPRESSION_QUALITY` 控件应由驱动设为 0。

    此控件的取值范围由驱动决定。只有正的、非零值才有意义。推荐范围为 1 - 100，其中较大的
    值对应更好的图像质量。

`V4L2_CID_JPEG_ACTIVE_MARKER (bitmask)`
    指定压缩流中包含哪些 JPEG 标记。此控件仅对编码器有效。

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_JPEG_ACTIVE_MARKER_APP0`
      - 应用数据段 APP\ `0`。
    - - `V4L2_JPEG_ACTIVE_MARKER_APP1`
      - 应用数据段 APP\ `1`。
    - - `V4L2_JPEG_ACTIVE_MARKER_COM`
      - 注释段。
    - - `V4L2_JPEG_ACTIVE_MARKER_DQT`
      - 量化表段。
    - - `V4L2_JPEG_ACTIVE_MARKER_DHT`
      - Huffman 表段。

有关 JPEG 规范的更多细节，请参考 itu-t81、jfif、w3c-jpeg-jfif。
