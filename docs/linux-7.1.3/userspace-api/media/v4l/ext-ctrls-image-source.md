
######## 图像源控制参考


图像源控制类用于图像源设备（如图像传感器）的底层控制。这些设备带有模数
转换器以及用于将图像数据传输出设备的总线发送器。


## 图像源控制 ID


`V4L2_CID_IMAGE_SOURCE_CLASS (class)`
    图像源类的描述符。

`V4L2_CID_VBLANK (integer)`
    垂直消隐。每一帧之后不产生图像数据的空闲时段。垂直消隐的单位为行。
    每一行的长度为图像宽度加上由同一子设备中的 `V4L2_CID_PIXEL_RATE`
    控制所定义的像素速率下的水平消隐。

`V4L2_CID_HBLANK (integer)`
    水平消隐。每一行图像数据之后不产生图像数据的空闲时段。水平消隐的
    单位为像素。

`V4L2_CID_ANALOGUE_GAIN (integer)`
    模拟增益，即影响像素矩阵中所有颜色分量的增益。增益操作在 A/D 转换
    之前的模拟域中完成。

`V4L2_CID_TEST_PATTERN_RED (integer)`
    测试图案的红色分量。

`V4L2_CID_TEST_PATTERN_GREENR (integer)`
    测试图案的绿色（紧邻红色）分量。

`V4L2_CID_TEST_PATTERN_BLUE (integer)`
    测试图案的蓝色分量。

`V4L2_CID_TEST_PATTERN_GREENB (integer)`
    测试图案的绿色（紧邻蓝色）分量。

`V4L2_CID_UNIT_CELL_SIZE (struct)`
    该控制返回单元像素单元的尺寸（单位：纳米）。结构体 `v4l2_area`
    通过独立的字段分别提供宽度和高度，以考虑非对称像素的情况。
    该控制不考虑任何可能存在的硬件合并（binning）。
    单元像素由像素的全部区域构成，包含感光与非感光部分。
    该控制用于传感器/摄像头的自动校准。


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 区域的宽度。
    - - __u32
      - `height`
      - 区域的高度。

`V4L2_CID_NOTIFY_GAINS (integer array)`
    传感器会被告知后续处理（例如由 ISP）将应用到不同颜色通道的增益值。
    传感器仅仅是获知这些值，以便其在需要这些值的处理中使用，但它并不会
    将这些增益实际应用到输出像素上。

    目前该控制仅针对 Bayer 传感器定义，是一个包含 4 个增益值的数组控制，
    分别对应每个 Bayer 通道的增益。无论传感器自身的 Bayer 顺序如何，增益
    顺序始终为 B、Gb、Gr 和 R。

    使用数组使得该控制可以扩展到例如采用非 Bayer CFA（色彩滤波阵列）的
    传感器。

    增益值的单位为线性，默认值为恰好 1.0 的增益。例如，若默认报告为
    （假设）128，则 192 表示恰好 1.5 的增益。

