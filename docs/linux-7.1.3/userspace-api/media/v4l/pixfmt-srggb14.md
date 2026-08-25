



######## V4L2_PIX_FMT_SRGGB14 ('RG14'), V4L2_PIX_FMT_SGRBG14 ('GR14'), V4L2_PIX_FMT_SGBRG14 ('GB14'), V4L2_PIX_FMT_SBGGR14 ('BG14'),


## 扩展16 位的 14 Bayer 格式


## 描述


这四种像素格式是每颜色分14 位的原始 sRGB / Bayer 格式
每个采样存储在一16 位字中，2 位未使用
并填充为零。每n 像素行包n/2 个绿色采
n/2 个蓝色或红色采样，红色和蓝色行交替排列。字
以小端序存储在内存中。它们通常
描述GRGR... BGBG...、RGRG... GBGB... 等。下面是一
小型 V4L2_PIX_FMT_SBGGR14 图像的示例：

**字节顺序*
每个单元为一个字节，高字节中 2 个最高有效位
涓?0銆。






    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 1 1 1 1

    - - start + 0:
      - B\ `00low`
      - B\ `00high`
      - G\ `01low`
      - G\ `01high`
      - B\ `02low`
      - B\ `02high`
      - G\ `03low`
      - G\ `03high`
    - - start + 8:
      - G\ `10low`
      - G\ `10high`
      - R\ `11low`
      - R\ `11high`
      - G\ `12low`
      - G\ `12high`
      - R\ `13low`
      - R\ `13high`
    - - start + 16:
      - B\ `20low`
      - B\ `20high`
      - G\ `21low`
      - G\ `21high`
      - B\ `22low`
      - B\ `22high`
      - G\ `23low`
      - G\ `23high`
    - - start + 24:
      - G\ `30low`
      - G\ `30high`
      - R\ `31low`
      - R\ `31high`
      - G\ `32low`
      - G\ `32high`
      - R\ `33low`
      - R\ `33high`
