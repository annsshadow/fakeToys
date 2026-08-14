

######## V4L2_PIX_FMT_SRGGB10P ('pRAA'), V4L2_PIX_FMT_SGRBG10P ('pgAA'), V4L2_PIX_FMT_SGBRG10P ('pGAA'), V4L2_PIX_FMT_SBGGR10P ('pBAA'),



V4L2_PIX_FMT_SGRBG10P
V4L2_PIX_FMT_SGBRG10P
V4L2_PIX_FMT_SBGGR10P
10 位打包 Bayer 格式


## 描述


这四种像素格式是每样本 10 位的打包原始 sRGB / Bayer 格式。每四个连续的样本被打包进 5 个字节。前 4 个字节中的每一个包含像素的 8 个高位，第 5 个字节以相同顺序包含每个像素的 2 个最低有效位。

每个 n 像素行包含 n/2 个绿色样本和 n/2 个蓝色或红色样本，绿-红行与绿-蓝行交替。它们通常被描述为 GRGR... BGBG...、RGRG... GBGB... 等。下面是一个小的 V4L2_PIX_FMT_SBGGR10P 图像示例：

**字节顺序。**
每个单元格为一个字节。


    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 68

    - - start + 0:
      - B\ `00high`
      - G\ `01high`
      - B\ `02high`
      - G\ `03high`
      - G\ `03low`\ (bits 7--6) B\ `02low`\ (bits 5--4)

	G\ `01low`\ (bits 3--2) B\ `00low`\ (bits 1--0)
    - - start + 5:
      - G\ `10high`
      - R\ `11high`
      - G\ `12high`
      - R\ `13high`
      - R\ `13low`\ (bits 7--6) G\ `12low`\ (bits 5--4)

	R\ `11low`\ (bits 3--2) G\ `10low`\ (bits 1--0)
    - - start + 10:
      - B\ `20high`
      - G\ `21high`
      - B\ `22high`
      - G\ `23high`
      - G\ `23low`\ (bits 7--6) B\ `22low`\ (bits 5--4)

	G\ `21low`\ (bits 3--2) B\ `20low`\ (bits 1--0)
    - - start + 15:
      - G\ `30high`
      - R\ `31high`
      - G\ `32high`
      - R\ `33high`
      - R\ `33low`\ (bits 7--6) G\ `32low`\ (bits 5--4)

	R\ `31low`\ (bits 3--2) G\ `30low`\ (bits 1--0)
