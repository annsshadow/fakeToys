


######## YUV 格式


YUV 是电视广播与复合视频信号原生的格式。它将亮度信息（Y）与色彩信息（U 和 V，或 Cb 和 Cr）分离。
色彩信息由红、蓝**色差**信号组成，这样可以通过从亮度分量中减去来重建绿色分量。转换示例参见 colorspaces。
之所以选择 YUV，是因为早期电视只传输亮度信息。为了以一种与现有接收机兼容的方式加入色彩，新增了一个
信号载波来传输色差信号。


## 子采样（Subsampling）


YUV 格式通常以低于亮度（luma）分量的分辨率对色度（chroma）分量进行编码。这种压缩技术利用了人眼对亮度
比对色彩差异更敏感的特点，称为色度子采样。

虽然在水平和垂直方向上许多子采样因子的组合都是可能的，但常见的因子是 1（无子采样）、2 和 4，且水平
子采样始终大于或等于垂直子采样。常见的组合命名如下。

- `4:4:4`：无子采样
- `4:2:2`：水平子采样 2 倍，无垂直子采样
- `4:2:0`：水平子采样 2 倍，垂直子采样 2 倍
- `4:1:1`：水平子采样 4 倍，无垂直子采样
- `4:1:0`：水平子采样 4 倍，垂直子采样 4 倍

对色度分量进行子采样会有效地产生可位于不同空间位置的色度值：

- .. _yuv-chroma-centered:

  子采样后的色度值可以通过简单地对两个连续像素的色度值取平均来计算。它有效地建模了位于两个原始像素
  之间的像素的色度。这被称为居中（centered）或间隙（interstitial）放置的色度。

- .. _yuv-chroma-cosited:

  另一种选择是以将色度值放置在与像素相同的空间位置的方式来子采样。这可以通过跳过每隔一个色度样本
  （会产生混叠伪影），或使用具有奇数个抽头的滤波器来完成。这被称为同址（co-sited）色度。

以下示例展示了 4x4 图像中色度放置的不同组合。

    :header-rows: 1
    :stub-columns: 1

    - -
      - 0
      -
      - 1
      -
      - 2
      -
      - 3
    - - 0
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y
    - - 1
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y
    - - 2
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y
    - - 3
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y

    :header-rows: 1
    :stub-columns: 1

    - -
      - 0
      -
      - 1
      -
      - 2
      -
      - 3
    - - 0
      - Y/C
      -
      - Y
      -
      - Y/C
      -
      - Y
    - - 1
      - Y/C
      -
      - Y
      -
      - Y/C
      -
      - Y
    - - 2
      - Y/C
      -
      - Y
      -
      - Y/C
      -
      - Y
    - - 3
      - Y/C
      -
      - Y
      -
      - Y/C
      -
      - Y

    :header-rows: 1
    :stub-columns: 1

    - -
      - 0
      -
      - 1
      -
      - 2
      -
      - 3
    - - 0
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y
    - - 1
      - Y
      -
      - Y
      -
      - Y
      -
      - Y
    - - 2
      - Y
      - C
      - Y
      -
      - Y
      - C
      - Y
    - - 3
      - Y
      -
      - Y
      -
      - Y
      -
      - Y

    :header-rows: 1
    :stub-columns: 1

    - -
      - 0
      -
      - 1
      -
      - 2
      -
      - 3
    - - 0
      - Y
      -
      - Y
      -
      - Y
      -
      - Y
#     * -

#       -

#       -

#       -

    - - 1
      - Y
      -
      - Y
      -
      - Y
      -
      - Y
#     * -

#       -

#       -

      - C
#       -

      -
    - - 2
      - Y
      -
      - Y
      -
      - Y
      -
      - Y
#     * -

#       -

#       -

#       -

    - - 3
      - Y
      -
      - Y
      -
      - Y
      -
      - Y


- [pixfmt-packed-yuv](pixfmt-packed-yuv)
- [pixfmt-yuv-planar](pixfmt-yuv-planar)
- [pixfmt-yuv-luma](pixfmt-yuv-luma)
- [pixfmt-y8i](pixfmt-y8i)
- [pixfmt-y12i](pixfmt-y12i)
- [pixfmt-y16i](pixfmt-y16i)
- [pixfmt-uv8](pixfmt-uv8)
- [pixfmt-m420](pixfmt-m420)
