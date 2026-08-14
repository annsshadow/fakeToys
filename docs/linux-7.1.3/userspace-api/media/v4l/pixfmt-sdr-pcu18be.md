
######## V4L2_SDR_FMT_PCU18BE ('PC18')


平面复无符号 18 位大端 IQ 采样

## 描述


该格式包含一串复数采样。每个复数由两部分组成，称为同相（In-phase）与正交
（Quadrature，IQ）。I 与 Q 均表示为 18 位无符号大端数，存储于 32 位空间中。
32 位空间中剩余的未用位将以 0 填充。I 值先开始，Q 值从偏移等于缓冲区大小一半
（即 offset = buffersize/2）处开始。在 18 位中，bit 17:2（16 位）为数据，
bit 1:0（2 位）可为任意值。

**字节序。**
每个单元为一个字节。

    :header-rows:  1
    :stub-columns: 0

    - -  Offset:
      - Byte B0
      - Byte B1
      - Byte B2
      - Byte B3
    - -  start + 0:
      - I'\ `0[17:10]`
      - I'\ `0[9:2]`
      - I'\ `0[1:0]; B2[5:0]=pad`
      - pad
    - -  start + 4:
      - I'\ `1[17:10]`
      - I'\ `1[9:2]`
      - I'\ `1[1:0]; B2[5:0]=pad`
      - pad
    - -  ...
    - - start + offset:
      - Q'\ `0[17:10]`
      - Q'\ `0[9:2]`
      - Q'\ `0[1:0]; B2[5:0]=pad`
      - pad
    - - start + offset + 4:
      - Q'\ `1[17:10]`
      - Q'\ `1[9:2]`
      - Q'\ `1[1:0]; B2[5:0]=pad`
      - pad
