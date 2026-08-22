
######## V4L2_SDR_FMT_PCU16BE ('PC16')


平面复无符号 16 位大IQ 采样

## 描述


该格式包含一串复数采样。每个复数由两部分组成，称为同相（In-phase）与正交
（Quadrature，IQ）。I Q 均表示为 16 位无符号大端数，存储32 位空间中32 位空间中剩余的未用位将以 0 填充。I 值先开始，Q 值从偏移等于缓冲区大小一（即 offset = buffersize/2）处开始。在 16 位中，bit 15:24 位）为数据，
bit 1:0 位）可为任意值
**字节序*
每个单元为一个字节
    :header-rows:  1
    :stub-columns: 0

    - -  Offset:
      - Byte B0
      - Byte B1
      - Byte B2
      - Byte B3
    - -  start + 0:
      - I'\ `0[13:6]`
      - I'\ `0[5:0]; B1[1:0]=pad`
      - pad
      - pad
    - -  start + 4:
      - I'\ `1[13:6]`
      - I'\ `1[5:0]; B1[1:0]=pad`
      - pad
      - pad
    - -  ...
    - - start + offset:
      - Q'\ `0[13:6]`
      - Q'\ `0[5:0]; B1[1:0]=pad`
      - pad
      - pad
    - - start + offset + 4:
      - Q'\ `1[13:6]`
      - Q'\ `1[5:0]; B1[1:0]=pad`
      - pad
      - pad
