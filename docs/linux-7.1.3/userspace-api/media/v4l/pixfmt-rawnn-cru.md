


######## V4L2_PIX_FMT_RAW_CRU10 ('CR10'), V4L2_PIX_FMT_RAW_CRU12 ('CR12'), V4L2_PIX_FMT_RAW_CRU14 ('CR14'), V4L2_PIX_FMT_RAW_CRU20 ('CR20')

本文档描述 V4L2 中 Renesas RZ/V2H SoC 摄像头接收单元（CRU）的 RAW 像素格式（CR10/CR12/CR14/CR20）。这些格式将像素连续打包进 64 位单元并以高位作填充；文中给出其位级布局与字节序，供视频采集驱动开发与像素格式适配时参考。



## Renesas RZ/V2H 摄像头接收单元 64 位打包像素格式


| V4L2_PIX_FMT_RAW_CRU10 (CR10)
| V4L2_PIX_FMT_RAW_CRU12 (CR12)
| V4L2_PIX_FMT_RAW_CRU14 (CR14)
| V4L2_PIX_FMT_RAW_CRU20 (CR20)

## 描述


这些像素格式是 Renesas RZ/V2H SoC 中 Camera Receiver Unit（摄像头接收单元）的部分 RAW 输出格式。它们是原始格式，将像素连续打包进 64 位单元，并以 4 或 8 个最高有效位作为填充。

**字节序**

    :header-rows:  2
    :stub-columns: 0
    :widths: 36 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
    :fill-cells:

    - - `1` Pixel Format Code
      - `63` Data organization
    - - 63
      - 62
      - 61
      - 60
      - 59
      - 58
      - 57
      - 56
      - 55
      - 54
      - 53
      - 52
      - 51
      - 50
      - 49
      - 48
      - 47
      - 46
      - 45
      - 44
      - 43
      - 42
      - 41
      - 40
      - 39
      - 38
      - 37
      - 36
      - 35
      - 34
      - 33
      - 32
      - 31
      - 30
      - 29
      - 28
      - 27
      - 26
      - 25
      - 24
      - 23
      - 22
      - 21
      - 20
      - 19
      - 18
      - 17
      - 16
      - 15
      - 14
      - 13
      - 12
      - 11
      - 10
      - 9
      - 8
      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0
    - - V4L2_PIX_FMT_RAW_CRU10
      - 0
      - 0
      - 0
      - 0
      - `9` P5
      - `9` P4
      - `9` P3
      - `9` P2
      - `9` P1
      - `9` P0
    - - V4L2_PIX_FMT_RAW_CRU12
      - 0
      - 0
      - 0
      - 0
      - `11` P4
      - `11` P3
      - `11` P2
      - `11` P1
      - `11` P0
    - - V4L2_PIX_FMT_RAW_CRU14
      - 0
      - 0
      - 0
      - 0
      - 0
      - 0
      - 0
      - 0
      - `13` P3
      - `13` P2
      - `13` P1
      - `13` P0
    - - V4L2_PIX_FMT_RAW_CRU20
      - 0
      - 0
      - 0
      - 0
      - `19` P2
      - `19` P1
      - `19` P0
