


######## V4L2_PIX_FMT_SRGGB12P ('pRCC')、V4L2_PIX_FMT_SGRBG12P ('pgCC')、V4L2_PIX_FMT_SGBRG12P ('pGCC')、V4L2_PIX_FMT_SBGGR12P ('pBCC')


### 12 位打包 Bayer 格式



## 描述


这四种像素格式是每颜色 12 位的打包原始 sRGB / Bayer 格式。每两个连续的采样被
打包进三个字节。前两个字节各包含像素的 8 个高位，第三个字节包含每个像素的 4 个
最低有效位，顺序相同。

每个 n 像素行包含 n/2 个绿色采样和 n/2 个蓝色或红色采样，绿色-红色与绿色-蓝色行
交替排列。它们通常描述为 GRGR... BGBG...、RGRG... GBGB... 等。下面是一个小的
V4L2_PIX_FMT_SBGGR12P 图像示例：

**字节顺序。**
每个单元格为一个字节。



    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 1 1


    - -  start + 0:
       - B\ `00high`
       - G\ `01high`
       - G\ `01low`\ (bits 7--4)

          B\ `00low`\ (bits 3--0)
       - B\ `02high`
       - G\ `03high`
       - G\ `03low`\ (bits 7--4)

          B\ `02low`\ (bits 3--0)

    - -  start + 6:
       - G\ `10high`
       - R\ `11high`
       - R\ `11low`\ (bits 7--4)

          G\ `10low`\ (bits 3--0)
       - G\ `12high`
       - R\ `13high`
       - R\ `13low`\ (bits 7--4)

          G\ `12low`\ (bits 3--0)
    - -  start + 12:
       - B\ `20high`
       - G\ `21high`
       - G\ `21low`\ (bits 7--4)

          B\ `20low`\ (bits 3--0)
       - B\ `22high`
       - G\ `23high`
       - G\ `23low`\ (bits 7--4)

          B\ `22low`\ (bits 3--0)
    - -  start + 18:
       - G\ `30high`
       - R\ `31high`
       - R\ `31low`\ (bits 7--4)

          G\ `30low`\ (bits 3--0)
       - G\ `32high`
       - R\ `33high`
       - R\ `33low`\ (bits 7--4)

          G\ `32low`\ (bits 3--0)
