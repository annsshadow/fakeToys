
######## V4L2_SDR_FMT_CS8 ('CS08')


复数有符号 8 位 IQ 采样


## 描述


该格式包含复数采样序列。每个复数由两部分组成，称为同相（In-phase）和正交
（Quadrature，IQ）。I 和 Q 均表示为 8 位有符号数值。I 值在前，Q 值在后。

**字节序。**
每个单元为一个字节。

    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - I'\ `0`
    - - start + 1:
      - Q'\ `0`
