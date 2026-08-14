


######## V4L2_PIX_FMT_SRGGB14P ('pREE'), V4L2_PIX_FMT_SGRBG14P ('pgEE'), V4L2_PIX_FMT_SGBRG14P ('pGEE'), V4L2_PIX_FMT_SBGGR14P ('pBEE'),

本页描述 V4L2 中四种 14 位打包的 sRGB/Bayer 原始像素格式（SRGGB14P、SGRBG14P、SGBRG14P、SBGGR14P），说明其字节打包排列方式与内存布局，供摄像头与图像处理应用正确解读原始帧数据。


**man V4L2_PIX_FMT_SRGGB14P(2)**

V4L2_PIX_FMT_SGRBG14P
V4L2_PIX_FMT_SGBRG14P
V4L2_PIX_FMT_SBGGR14P
14 位打包的 Bayer 格式

## 描述


这四种像素格式是每色 14 位的打包原始 sRGB / Bayer 格式。每四个连续的采样被
打包进七个字节。前四个字节各包含像素的八个高位，随后的三个字节以相同顺序包含
每个像素的六个低位。

每个 n 像素行包含 n/2 个绿色采样和 n/2 个蓝色或红色采样，绿色-红色与绿色-蓝色
行交替排列。它们通常描述为 GRGR... BGBG...、RGRG... GBGB... 等。下面是其中一种
格式的示例：

**字节顺序。** 每个单元格为一个字节。


    \begingroup
    \footnotesize
    \setlength{\tabcolsep}{2pt}


    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 3 3 3


    - .. row 1

       - start + 0

       - B\ `00high`

       - G\ `01high`

       - B\ `02high`

       - G\ `03high`

       - G\ `01low bits 1--0`\ (bits 7--6)

	  B\ `00low bits 5--0`\ (bits 5--0)

       - B\ `02low bits 3--0`\ (bits 7--4)

	  G\ `01low bits 5--2`\ (bits 3--0)

       - G\ `03low bits 5--0`\ (bits 7--2)

	  B\ `02low bits 5--4`\ (bits 1--0)

    - .. row 2

       - start + 7

       - G\ `10high`

       - R\ `11high`

       - G\ `12high`

       - R\ `13high`

       - R\ `11low bits 1--0`\ (bits 7--6)

	  G\ `10low bits 5--0`\ (bits 5--0)

       - G\ `12low bits 3--0`\ (bits 7--4)

	  R\ `11low bits 5--2`\ (bits 3--0)

       - R\ `13low bits 5--0`\ (bits 7--2)

	  G\ `12low bits 5--4`\ (bits 1--0)

    - .. row 3

       - start + 14

       - B\ `20high`

       - G\ `21high`

       - B\ `22high`

       - G\ `23high`

       - G\ `21low bits 1--0`\ (bits 7--6)

	  B\ `20low bits 5--0`\ (bits 5--0)

       - B\ `22low bits 3--0`\ (bits 7--4)

	  G\ `21low bits 5--2`\ (bits 3--0)

       - G\ `23low bits 5--0`\ (bits 7--2)

	  B\ `22low bits 5--4`\ (bits 1--0)

    - .. row 4

       - start + 21

       - G\ `30high`

       - R\ `31high`

       - G\ `32high`

       - R\ `33high`

       - R\ `31low bits 1--0`\ (bits 7--6)
	  G\ `30low bits 5--0`\ (bits 5--0)

       - G\ `32low bits 3--0`\ (bits 7--4)
	  R\ `31low bits 5--2`\ (bits 3--0)

       - R\ `33low bits 5--0`\ (bits 7--2)
	  G\ `32low bits 5--4`\ (bits 1--0)


    \endgroup
