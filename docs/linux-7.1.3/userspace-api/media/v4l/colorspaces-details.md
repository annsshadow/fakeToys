

######## 色彩空间详细描述（Detailed Colorspace Descriptions）



## 色彩空间 SMPTE 170M (V4L2_COLORSPACE_SMPTE170M)

smpte170m 标准定义了 NTSC 和 PAL 以及通常 SDTV 所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_709`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：

    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.630
      - 0.340
    - - Green
      - 0.310
      - 0.595
    - - Blue
      - 0.155
      - 0.070
    - - White Reference (D65)
      - 0.3127
      - 0.3290


红色、绿色和蓝色的色度坐标也常被称为 SMPTE C 集合，因此该色彩空间有时也被称为 SMPTE C。

为 SMPTE 170M 定义的传递函数与 Rec. 709 中定义的相同。


    L' = -1.099(-L)^{0.45} + 0.099 \text{, for } L \le-0.018

    L' = 4.5L \text{, for } -0.018 < L < 0.018

    L' = 1.099L^{0.45} - 0.099 \text{, for } L \ge 0.018

反传递函数：


    L = -\left( \frac{L' - 0.099}{-1.099} \right) ^{\frac{1}{0.45}} \text{, for } L' \le -0.081

    L = \frac{L'}{4.5} \text{, for } -0.081 < L' < 0.081

    L = \left(\frac{L' + 0.099}{1.099}\right)^{\frac{1}{0.45} } \text{, for } L' \ge 0.081

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_601` 编码获得：


    Y' = 0.2990R' + 0.5870G' + 0.1140B'

    Cb = -0.1687R' - 0.3313G' + 0.5B'

    Cr = 0.5R' - 0.4187G' - 0.0813B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。该向 Y'CbCr 的转换与 itu601 标准中定义的相同，因此该色彩空间有时也被称为 BT.601，尽管 BT.601 并未提及任何原色。

默认量化是有限范围，但全范围也可能，尽管很少见。



## 色彩空间 Rec. 709 (V4L2_COLORSPACE_REC709)

itu709 标准定义了通常 HDTV 所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_709`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_709`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：

    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.640
      - 0.330
    - - Green
      - 0.300
      - 0.600
    - - Blue
      - 0.150
      - 0.060
    - - White Reference (D65)
      - 0.3127
      - 0.3290


该标准的完整名称为 Rec. ITU-R BT.709-5。

传递函数。通常 L 在 [0…1] 范围内，但对于扩展色域 xvYCC 编码，允许该范围之外的值。


    L' = -1.099(-L)^{0.45} + 0.099 \text{, for } L \le -0.018

    L' = 4.5L \text{, for } -0.018 < L < 0.018

    L' = 1.099L^{0.45} - 0.099 \text{, for } L \ge 0.018

反传递函数：


    L = -\left( \frac{L' - 0.099}{-1.099} \right)^\frac{1}{0.45} \text{, for } L' \le -0.081

    L = \frac{L'}{4.5}\text{, for } -0.081 < L' < 0.081

    L = \left(\frac{L' + 0.099}{1.099}\right)^{\frac{1}{0.45} } \text{, for } L' \ge 0.081

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_709` 编码获得：


    Y' = 0.2126R' + 0.7152G' + 0.0722B'

    Cb = -0.1146R' - 0.3854G' + 0.5B'

    Cr = 0.5R' - 0.4542G' - 0.0458B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。

默认量化是有限范围，但全范围也可能，尽管很少见。

上文描述的 `V4L2_YCBCR_ENC_709` 编码是该色彩空间的默认编码，但可以用 `V4L2_YCBCR_ENC_601` 覆盖，此时使用 BT.601 的 Y'CbCr 编码。

该色彩空间还支持两种额外的扩展色域 Y'CbCr 编码：

xvYCC 709 编码（`V4L2_YCBCR_ENC_XV709`，xvycc）与 Rec. 709 编码类似，但它允许 R'、G' 和 B' 取 [0…1] 范围之外的值。所得的 Y'、Cb 和 Cr 值根据有限范围公式进行缩放与偏移：


    Y' = \frac{219}{256} * (0.2126R' + 0.7152G' + 0.0722B') + \frac{16}{256}

    Cb = \frac{224}{256} * (-0.1146R' - 0.3854G' + 0.5B')

    Cr = \frac{224}{256} * (0.5R' - 0.4542G' - 0.0458B')

xvYCC 601 编码（`V4L2_YCBCR_ENC_XV601`，xvycc）与 BT.601 编码类似，但它允许 R'、G' 和 B' 取 [0…1] 范围之外的值。所得的 Y'、Cb 和 Cr 值根据有限范围公式进行缩放与偏移：


    Y' = \frac{219}{256} * (0.2990R' + 0.5870G' + 0.1140B') + \frac{16}{256}

    Cb = \frac{224}{256} * (-0.1687R' - 0.3313G' + 0.5B')

    Cr = \frac{224}{256} * (0.5R' - 0.4187G' - 0.0813B')

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内，并在不进一步缩放或偏移的情况下进行量化。
非标准的 xvYCC 709 或 xvYCC 601 编码可通过选择 `V4L2_YCBCR_ENC_XV709` 或 `V4L2_YCBCR_ENC_XV601` 来使用。
如 xvYCC 公式所示，这些编码始终使用有限范围量化，没有全范围变体。这些扩展色域编码的全部意义在于，有限范围之外的值仍然有效，尽管它们映射到 [0…1] 范围之外的 R'、G' 和 B' 值，因此落在 Rec. 709 色彩空间的色域之外。



## 色彩空间 sRGB (V4L2_COLORSPACE_SRGB)

srgb 标准定义了大多数网络摄像头和计算机图形所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_SRGB`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`。默认的 Y'CbCr 量化是有限范围。

请注意，sycc 标准规定使用全范围量化，然而内核当前支持的所有采集硬件都将 R'G'B' 转换为有限范围的 Y'CbCr。因此若将全范围作为默认，会破坏应用程序对量化范围的解释。

原色与白色参考的色度坐标如下：

    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.640
      - 0.330
    - - Green
      - 0.300
      - 0.600
    - - Blue
      - 0.150
      - 0.060
    - - White Reference (D65)
      - 0.3127
      - 0.3290


这些色度坐标与 Rec. 709 色彩空间完全相同。

传递函数。请注意，L 的负值仅用于 Y'CbCr 转换。


    L' = -1.055(-L)^{\frac{1}{2.4} } + 0.055\text{, for }L < -0.0031308

    L' = 12.92L\text{, for }-0.0031308 \le L \le 0.0031308

    L' = 1.055L ^{\frac{1}{2.4} } - 0.055\text{, for }0.0031308 < L \le 1

反传递函数：


    L = -((-L' + 0.055) / 1.055) ^{2.4}\text{, for }L' < -0.04045

    L = L' / 12.92\text{, for }-0.04045 \le L' \le 0.04045

    L = ((L' + 0.055) / 1.055) ^{2.4}\text{, for }L' > 0.04045

亮度（Y'）与色差（Cb 和 Cr）通过 sycc 定义的以下 `V4L2_YCBCR_ENC_601` 编码获得：


    Y' = 0.2990R' + 0.5870G' + 0.1140B'

    Cb = -0.1687R' - 0.3313G' + 0.5B'

    Cr = 0.5R' - 0.4187G' - 0.0813B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。该变换与 SMPTE 170M/BT.601 中定义的相同。Y'CbCr 量化是有限范围。



## 色彩空间 opRGB (V4L2_COLORSPACE_OPRGB)

oprgb 标准定义了使用 opRGB 色彩空间的计算机图形所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_OPRGB`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`。默认的 Y'CbCr 量化是有限范围。

请注意，oprgb 标准规定使用全范围量化，然而内核当前支持的所有采集硬件都将 R'G'B' 转换为有限范围的 Y'CbCr。因此若将全范围作为默认，会破坏应用程序对量化范围的解释。

原色与白色参考的色度坐标如下：

    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.6400
      - 0.3300
    - - Green
      - 0.2100
      - 0.7100
    - - Blue
      - 0.1500
      - 0.0600
    - - White Reference (D65)
      - 0.3127
      - 0.3290



传递函数：


    L' = L ^{\frac{1}{2.19921875}}

反传递函数：


    L = L'^{(2.19921875)}

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_601` 编码获得：


    Y' = 0.2990R' + 0.5870G' + 0.1140B'

    Cb = -0.1687R' - 0.3313G' + 0.5B'

    Cr = 0.5R' - 0.4187G' - 0.0813B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。该变换与 SMPTE 170M/BT.601 中定义的相同。Y'CbCr 量化是有限范围。



## 色彩空间 BT.2020 (V4L2_COLORSPACE_BT2020)

itu2020 标准定义了超高清晰度电视（UHDTV）所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_709`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_BT2020`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：

    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.708
      - 0.292
    - - Green
      - 0.170
      - 0.797
    - - Blue
      - 0.131
      - 0.046
    - - White Reference (D65)
      - 0.3127
      - 0.3290



传递函数（与 Rec. 709 相同）：


    L' = 4.5L\text{, for }0 \le L < 0.018

    L' = 1.099L ^{0.45} - 0.099\text{, for } 0.018 \le L \le 1

反传递函数：


    L = L' / 4.5\text{, for } L' < 0.081

    L = \left( \frac{L' + 0.099}{1.099}\right) ^{\frac{1}{0.45} }\text{, for } L' \ge 0.081

请注意，虽然 itu2020 标准将 Rec. 709 定义为默认传递函数，但在实践中该色彩空间常与 xf-smpte-2084 配合使用。特别是 Ultra HD Blu-ray 光盘使用这一组合。

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_BT2020` 编码获得：


    Y' = 0.2627R' + 0.6780G' + 0.0593B'

    Cb = -0.1396R' - 0.3604G' + 0.5B'

    Cr = 0.5R' - 0.4598G' - 0.0402B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。Y'CbCr 量化是有限范围。

还有一种可选的恒定亮度 R'G'B' 到 Yc'CbcCrc（`V4L2_YCBCR_ENC_BT2020_CONST_LUM`）编码：

亮度：

    :nowrap:

    \begin{align*}
    Yc' = (0.2627R + 0.6780G + 0.0593B)'& \\
    B' - Yc' \le 0:& \\
        &Cbc = (B' - Yc') / 1.9404 \\
    B' - Yc' > 0: & \\
        &Cbc = (B' - Yc') / 1.5816 \\
    R' - Yc' \le 0:& \\
        &Crc = (R' - Y') / 1.7184 \\
    R' - Yc' > 0:& \\
        &Crc = (R' - Y') / 0.9936
    \end{align*}

Yc' 被限制在 [0…1] 范围内，Cbc 和 Crc 被限制在 [-0.5…0.5] 范围内。Yc'CbcCrc 量化是有限范围。



## 色彩空间 DCI-P3 (V4L2_COLORSPACE_DCI_P3)

smpte431 标准定义了使用 DCI-P3 色彩空间的影院投影机所使用的色彩空间。默认传递函数为 `V4L2_XFER_FUNC_DCI_P3`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_709`。默认的 Y'CbCr 量化是有限范围。


   注意，该色彩空间标准并未规定 Y'CbCr 编码，因为它本意并非编码为 Y'CbCr。因此选取该默认 Y'CbCr 编码是因为它是 HDTV 编码。

原色与白色参考的色度坐标如下：


    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.6800
      - 0.3200
    - - Green
      - 0.2650
      - 0.6900
    - - Blue
      - 0.1500
      - 0.0600
    - - White Reference
      - 0.3140
      - 0.3510



传递函数：


    L' = L^{\frac{1}{2.6}}

反传递函数：


    L = L'^{(2.6)}

Y'CbCr 编码未作规定。V4L2 默认为 Rec. 709。



## 色彩空间 SMPTE 240M (V4L2_COLORSPACE_SMPTE240M)

smpte240m 标准是 HDTV 早期（1988-1998）使用的一项过渡标准。它已被 Rec. 709 取代。默认传递函数为 `V4L2_XFER_FUNC_SMPTE240M`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_SMPTE240M`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：


    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.630
      - 0.340
    - - Green
      - 0.310
      - 0.595
    - - Blue
      - 0.155
      - 0.070
    - - White Reference (D65)
      - 0.3127
      - 0.3290


这些色度坐标与 SMPTE 170M 色彩空间完全相同。

传递函数：


    L' = 4L\text{, for } 0 \le L < 0.0228

    L' = 1.1115L ^{0.45} - 0.1115\text{, for } 0.0228 \le L \le 1

反传递函数：


    L = \frac{L'}{4}\text{, for } 0 \le L' < 0.0913

    L = \left( \frac{L' + 0.1115}{1.1115}\right) ^{\frac{1}{0.45} }\text{, for } L' \ge 0.0913

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_SMPTE240M` 编码获得：


    Y' = 0.2122R' + 0.7013G' + 0.0865B'

    Cb = -0.1161R' - 0.3839G' + 0.5B'

    Cr = 0.5R' - 0.4451G' - 0.0549B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。Y'CbCr 量化是有限范围。



## 色彩空间 NTSC 1953 (V4L2_COLORSPACE_470_SYSTEM_M)

该标准定义了 1953 年 NTSC 所使用的色彩空间。在实践中该色彩空间已过时，应改用 SMPTE 170M。默认传递函数为 `V4L2_XFER_FUNC_709`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：


    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.67
      - 0.33
    - - Green
      - 0.21
      - 0.71
    - - Blue
      - 0.14
      - 0.08
    - - White Reference (C)
      - 0.310
      - 0.316



   该色彩空间使用 C 光源而非 D65 作为白色参考。要将该色彩空间中的图像正确转换为另一个使用 D65 的图像，你需要应用一种色度适配算法，例如 Bradford 方法。

NTSC 1953 的传递函数从未被正确定义。文献中推荐使用 Rec. 709 的传递函数：


    L' = 4.5L\text{, for } 0 \le L < 0.018

    L' = 1.099L ^{0.45} - 0.099\text{, for } 0.018 \le L \le 1

反传递函数：


    L = \frac{L'}{4.5} \text{, for } L' < 0.081

    L = \left( \frac{L' + 0.099}{1.099}\right) ^{\frac{1}{0.45} }\text{, for } L' \ge 0.081

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_601` 编码获得：


    Y' = 0.2990R' + 0.5870G' + 0.1140B'

    Cb = -0.1687R' - 0.3313G' + 0.5B'

    Cr = 0.5R' - 0.4187G' - 0.0813B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。Y'CbCr 量化是有限范围。该变换与 SMPTE 170M/BT.601 中定义的相同。



## 色彩空间 EBU Tech. 3213 (V4L2_COLORSPACE_470_SYSTEM_BG)

tech3213 标准定义了 1975 年 PAL/SECAM 所使用的色彩空间。请注意，HDMI 接口不支持该色彩空间。反之，tech3321 建议 HDMI 改用 Rec. 709。默认传递函数为 `V4L2_XFER_FUNC_709`。默认的 Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`。默认的 Y'CbCr 量化是有限范围。原色与白色参考的色度坐标如下：


    :header-rows:  1
    :stub-columns: 0
    :widths:       1 1 2

    - - Color
      - x
      - y
    - - Red
      - 0.64
      - 0.33
    - - Green
      - 0.29
      - 0.60
    - - Blue
      - 0.15
      - 0.06
    - - White Reference (D65)
      - 0.3127
      - 0.3290



该色彩空间的传递函数从未被正确定义。文献中推荐使用 Rec. 709 的传递函数：


    L' = 4.5L\text{, for } 0 \le L < 0.018

    L' = 1.099L ^{0.45} - 0.099\text{, for } 0.018 \le L \le 1

反传递函数：


    L = \frac{L'}{4.5} \text{, for } L' < 0.081

    L = \left(\frac{L' + 0.099}{1.099} \right) ^{\frac{1}{0.45} }\text{, for } L' \ge 0.081

亮度（Y'）与色差（Cb 和 Cr）通过以下 `V4L2_YCBCR_ENC_601` 编码获得：


    Y' = 0.2990R' + 0.5870G' + 0.1140B'

    Cb = -0.1687R' - 0.3313G' + 0.5B'

    Cr = 0.5R' - 0.4187G' - 0.0813B'

Y' 被限制在 [0…1] 范围内，Cb 和 Cr 被限制在 [-0.5…0.5] 范围内。Y'CbCr 量化是有限范围。该变换与 SMPTE 170M/BT.601 中定义的相同。



## 色彩空间 JPEG (V4L2_COLORSPACE_JPEG)

该色彩空间定义了大多数（动态）JPEG 格式所使用的色彩空间。原色与白色参考的色度坐标与 sRGB 相同。使用的传递函数为 `V4L2_XFER_FUNC_SRGB`。Y'CbCr 编码为 `V4L2_YCBCR_ENC_601`，使用全范围量化，其中 Y' 缩放至 [0…255]，Cb/Cr 缩放至 [-128…128]，然后裁剪到 [-128…127]。


    JPEG 标准实际上并不存储色彩空间信息。因此，若使用了 sRGB 之外的其它色彩空间，驱动程序必须显式设置该信息。实际上，`V4L2_COLORSPACE_JPEG` 可视为 `V4L2_COLORSPACE_SRGB`、`V4L2_XFER_FUNC_SRGB`、`V4L2_YCBCR_ENC_601` 和 `V4L2_QUANTIZATION_FULL_RANGE` 的缩写。

######## 传递函数详细描述（Detailed Transfer Function Descriptions）



## 传递函数 SMPTE 2084 (V4L2_XFER_FUNC_SMPTE2084)

smpte2084 标准定义了高动态范围（HDR）内容所使用的传递函数。

常量：
    m1 = (2610 / 4096) / 4

    m2 = (2523 / 4096) * 128

    c1 = 3424 / 4096

    c2 = (2413 / 4096) * 32

    c3 = (2392 / 4096) * 32

传递函数：
    L' = ((c1 + c2 ** L\ `m1`) / (1 + c3 ** L\ `m1`))\ `m2`

反传递函数：
    L = (max(L'`1/m2` - c1, 0) / (c2 - c3 *
    L'\ `1/m2`))\ `1/m1`

在将该传递函数与非 HDR 传递函数相互转换时务必小心：HDR 内容的线性 RGB 值 [0…1] 映射到 0 到 10000 cd/m\ `2` 的亮度范围，而非 HDR（即标准动态范围 SDR）的线性 RGB 值映射到 0 到 100 cd/m\ `2` 的亮度范围。

要从 SDR 转到 HDR，你需要先将 L 除以 100。要反向转换，则需要将 L 乘以 100。当然，这会把所有超过 100 cd/m\ `2` 的亮度值都裁剪到 100 cd/m\ `2`。

有更好的方法，例如参见 colimg 获取关于此主题的更深入信息。
