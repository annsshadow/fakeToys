######## V4L2_SDR_FMT_CS14LE ('CS14')


复式有符14 位小IQ 采样


## 说明


该格式包含一系列复数（complex number）样本。每个复数由两部分组成，称为同相（In-phase）和正交（Quadrature，IQ）。I Q 都表示为 14 位有符号小端数。I 值在前，Q 值在后4 位值存储在 16 位空间中，未使用的高位以 0 填充

**字节序*
每个单元为一个字节


    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - I'\ `0[7:0]`
      - I'\ `0[13:8]`
    - - start + 2:
      - Q'\ `0[7:0]`
      - Q'\ `0[13:8]`
