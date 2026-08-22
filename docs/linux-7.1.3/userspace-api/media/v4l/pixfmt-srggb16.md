



######## V4L2_PIX_FMT_SRGGB16 ('RG16')、V4L2_PIX_FMT_SGRBG16 ('GR16')、V4L2_PIX_FMT_SGBRG16 ('GB16')、V4L2_PIX_FMT_SBGGR16 ('BYR2')


## 16 Bayer 格式


## 描述


这四种像素格式是原始 sRGB / Bayer 格式，每个样16 位。每个样本存储在一16 位字中。每n 像素行包n/2 个绿色样本和 n/2 个蓝色或红色样本，红色和蓝色行交替。字节以小端顺序存储在内存中。它们通常被描述为 GRGR... BGBG...、RGRG... GBGB... 等。下面是一个小示例 V4L2_PIX_FMT_SBGGR16 图像

**字节序*
每个单元为一个字节

    :header-rows:  0
    :stub-columns: 0

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
