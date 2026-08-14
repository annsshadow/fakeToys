
######## Defining Colorspaces in V4L2


在 V4L2 中，色彩空间由四个值定义。第一个是色彩空间标识符（enum `v4l2_pix_format_mplane`），
它定义了色度、默认传递函数、默认 Y'CbCr 编码和默认量化方法。
第二个是传递函数标识符（enum `v4l2_pix_format_mplane`），用于指定非标准传递函数。
第三个是 Y'CbCr 编码标识符（enum `v4l2_pix_format_mplane`），用于指定非标准 Y'CbCr 编码；
第四个是量化标识符（enum `v4l2_pix_format_mplane`），用于指定非标准量化方法。
大多数情况下只需要填写 struct `v4l2_pix_format_mplane` 或 struct `v4l2_pix_format_mplane` 的 colorspace 字段。


在 HSV 格式 <hsv-formats> 上，**Hue（色相）** 被定义为圆柱颜色表示上的角度。
通常该角度以度为单位度量，即 0-360。当将该角度值映射到 8 位时，有两种基本方式：
将角度值除以 2（0-179），或使用整个范围 0-255，将角度值除以 1.41。
enum `v4l2_hsv_encoding` 指定使用哪种编码。

   色彩空间。HSV 格式始终为全范围。



    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_COLORSPACE_RAW`
      - 默认色彩空间。应用程序可使用它让驱动填充色彩空间。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-smpte-170m。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-rec709。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-srgb。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-oprgb。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-bt2020。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-dcip3。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-smpte-240m。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-sysm。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-sysbg。
    - - `V4L2_COLORSPACE_RAW`
      - 参见 col-jpeg。
    - - `V4L2_COLORSPACE_RAW`
      - 原始色彩空间。用于原始图像采集，此时图像经过最少处理并使用设备内部的色彩空间。
      使用此“色彩空间”处理图像的软件必须了解采集设备的内部细节。





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用色彩空间定义的默认传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 Rec. 709 传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 sRGB 传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 opRGB 传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 SMPTE 240M 传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 不使用传递函数（即使用线性 RGB 值）。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 DCI-P3 传递函数。
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 使用 SMPTE 2084 传递函数。参见 xf-smpte-2084。





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用色彩空间定义的默认 Y'CbCr 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用 BT.601 Y'CbCr 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用 Rec. 709 Y'CbCr 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用扩展色域 xvYCC BT.601 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用扩展色域 xvYCC Rec. 709 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用默认非常量亮度 BT.2020 Y'CbCr 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用常量亮度 BT.2020 Yc'CbcCrc 编码。
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 使用 SMPTE 240M Y'CbCr 编码。





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_HSV_ENC_256`
      - 对于色相，每个 LSB 为两度。
    - - `V4L2_HSV_ENC_256`
      - 对于色相，360 度映射到 8 位，即每个 LSB 约为 1.41 度。





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 使用色彩空间定义的默认量化编码。对于 R'G'B' 和 HSV 这始终是全范围。
      对于 Y'CbCr 通常为受限范围。
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 使用全范围量化编码。即范围 [0…1] 映射到 [0…255]（可能裁切到 [1…254] 以避免
      0x00 和 0xff 值）。Cb 和 Cr 从 [-0.5…0.5] 映射到 [0…255]（可能裁切到 [1…254] 以避免
      0x00 和 0xff 值）。
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 使用受限范围量化编码。即范围 [0…1] 映射到 [16…235]。Cb 和 Cr 从 [-0.5…0.5] 映射到
      [16…240]。受限范围不能与 HSV 一起使用。

