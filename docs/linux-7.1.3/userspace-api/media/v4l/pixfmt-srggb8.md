


######## V4L2_PIX_FMT_SRGGB8 ('RGGB'), V4L2_PIX_FMT_SGRBG8 ('GRBG'), V4L2_PIX_FMT_SGBRG8 ('GBRG'), V4L2_PIX_FMT_SBGGR8 ('BA81'),



## 8 Bayer 格式


## 描述


这四种像素格式是原始 sRGB / Bayer 格式，每个样8 位。每个样本存储在一字节中。每n 像素行包n/2 个绿色样本和 n/2 个蓝色或红色样本，红色和蓝色
行交替。它们通常描述GRGR... BGBG...、RGRG... GBGB... 等。下面是一个小V4L2_PIX_FMT_SBGGR8 图像示例
**字节顺序*
每个单元为一个字节



    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - B\ `00`
      - G\ `01`
      - B\ `02`
      - G\ `03`
    - - start + 4:
      - G\ `10`
      - R\ `11`
      - G\ `12`
      - R\ `13`
    - - start + 8:
      - B\ `20`
      - G\ `21`
      - B\ `22`
      - G\ `23`
    - - start + 12:
      - G\ `30`
      - R\ `31`
      - G\ `32`
      - R\ `33`
