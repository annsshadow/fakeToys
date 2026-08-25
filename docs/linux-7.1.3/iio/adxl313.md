
## ADXL313 驱动

本文件介Linux IIO 子系统中 ADXL313 三轴加速度计的驱动实现，说明其 SPI/I2C 连接、测量范围以及对应的 sysfs 设备文件与活非活动事件配置，供驱动使用者参考


本驱动支Analog Device ADXL313，通过 SPI/I2C 总线连接

## 1. 支持的设


- `ADXL313 <https://www.analog.com/ADXL313>`_

ADXL313 是一款低噪声密度、低功耗的 3 轴加速度计，具有可选的测量范围。ADXL313 支持 ±0.5 g、 g、 g ±4 g 范围

## 2. 设备属


加速度计测量值始终提供

每个 IIO 设备，在 `/sys/bus/iio/devices/iio:deviceX` 下都有一个设备文件夹，其X 是该设备IIO 索引。根据所讨论硬件设备的特性与功能，这些文件夹下存放着一组设备文件。这些文件被一致地泛化，并记录IIO ABI 文档中

下表显示了与 adxl313 相关的设备文件，它们位于特定设备文件夹路`/sys/bus/iio/devices/iio:deviceX` 下

+---------------------------------------------------+----------------------------------------------------------+
| 3 轴加速度计相关设备文                           | 描述                                                     |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_scale                                    | 加速度计通道的比例因子（scale）                       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_calibbias                              | X 轴加速度计通道的校准偏移                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_raw                                    | X 轴加速度计通道的原始值                              |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_calibbias                              | Y 轴加速度偏移校正                                       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_raw                                    | Y 轴加速度计通道的原始值                              |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_calibbias                              | Z 轴加速度计通道的校准偏移                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_raw                                    | Z 轴加速度计通道的原始值                              |
+---------------------------------------------------+----------------------------------------------------------+

+---------------------------------------+----------------------------------------------+
| 杂项设备文件                          | 描述                                         |
+---------------------------------------+----------------------------------------------+
| name                                  | IIO 设备的名称                            |
+---------------------------------------+----------------------------------------------+
| in_accel_sampling_frequency           | 当前选择的采样率                          |
+---------------------------------------+----------------------------------------------+
| in_accel_sampling_frequency_available | 可用的采样频率配置                        |
+---------------------------------------+----------------------------------------------+

iio 事件相关的设置，位于 `/sys/bus/iio/devices/iio:deviceX/events` 下

+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_falling_period              | AC 耦合的非活动时间                                  |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_falling_value               | AC 耦合的非活动阈值                                  |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_adaptive_rising_value                | AC 耦合的活动阈值                                    |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_falling_period                       | 非活动时间                                          |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_falling_value                        | 非活动阈值                                          |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_mag_rising_value                         | 活动阈值                                            |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\&y\&z_mag_adaptive_falling_en          | 启用或禁AC 耦合的非活动事件                      |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\|y\|z_mag_adaptive_rising_en           | 启用或禁AC 耦合的活动事件                        |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\&y\&z_mag_falling_en                   | 启用或禁用非活动事件                                |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x\|y\|z_mag_rising_en                    | 启用或禁用活动事件                                  |
+---------------------------------------------------+----------------------------------------------------------+

默认耦合DC 耦合事件。在这种情况下阈值将保持原样，而对AC 耦合的情况，传感器会应用一个自适应阈值（datasheet 中描述）。通常活动，即 `ACTIVITY` `ACTIVITY_AC`，以及非活动，即 `INACTIVITY` `INACTIVITY_AC`，在两者都启用时将与自动休眠关联。这意味着特别`ACTIVITY` 也可以与 `INACTIVITY_AC` 关联，反之亦然，没有问题

注意，这`ACTIVITY` `ACTIVITY_AC` 是互斥的。这意味着，最近的一次配置将被设置。例如，如果 `ACTIVITY` 已启用，`ACTIVITY_AC` 将被启用，那么传感器驱动将禁`ACTIVITY`，但启用 `ACTIVITY_AC`。对于非活动同样成立。在关闭一个事件时，它必须与实际启用的相匹配，即启`ACTIVITY_AC` 然后禁用 `ACTIVITY` 会被简单地忽略，因为它已经处于禁用状态。或者，就像对待任何其他未启用的事件一样

### 通道处理后的


可以从通道_raw 属性读取一个通道值。返回的值是设备所报告的原始值。要获得通道的处理后值，应用以下公式


        processed value = (_raw + _offset) * _scale

其中 _offset _scale 是设备属性。如果不存在 _offset 属性，则简单地假定其值为 0

ADXL313 驱动为单一类型的通道提供数据，下表显示了处理后值的测量单位，它们由 IIO 框架定义

+-------------------------------------+---------------------------+
| 通道类型                            | 测量单位                  |
+-------------------------------------+---------------------------+
| X、Y Z 轴上的加速度              | 米每二次方秒             |
+-------------------------------------+---------------------------+

### 使用示例


显示设备名称


        root:/sys/bus/iio/devices/iio:device0> cat name
        adxl313

显示加速度计通道值：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_y_raw
        -57
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_z_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_scale
        0.009576806

加速度计的值将是：

- X 轴加速度 = in_accel_x_raw * in_accel_scale = 0.0191536 m/s^2
- Y 轴加速度 = in_accel_y_raw * in_accel_scale = -0.5458779 m/s^2
- Z 轴加速度 = in_accel_z_raw * in_accel_scale = 0.0191536 m/s^2

设置加速度计通道的校准偏移。注意，校准将根LSB 单位的刻度进行四舍五入：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        0

        root:/sys/bus/iio/devices/iio:device0> echo 50 > in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        48

设置采样频率


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency
        100.000000
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency_available
        6.250000 12.500000 25.000000 50.000000 100.000000 200.000000 400.000000 800.000000 1600.000000 3200.000000

        root:/sys/bus/iio/devices/iio:device0> echo 400 > in_accel_sampling_frequency
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_sampling_frequency
        400.000000

## 3. 设备缓冲区与触发


本驱动支IIO 缓冲区

所有设备都支持使用缓冲区检索原始加速度测量值

### 使用示例


为缓冲区读取选择通道


        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_z_en

设置在缓冲区中存储的样本数量


        root:/sys/bus/iio/devices/iio:device0> echo 10 > buffer/length

启用缓冲区读取：


        root:/sys/bus/iio/devices/iio:device0> echo 1 > buffer/enable

获取缓冲数据


        root:/sys/bus/iio/devices/iio:device0> hexdump -C /dev/iio\:device0
        ...
        000000d0  01 fc 31 00 c7 ff 03 fc  31 00 c7 ff 04 fc 33 00  |..1.....1.....3.|
        000000e0  c8 ff 03 fc 32 00 c5 ff  ff fc 32 00 c7 ff 0a fc  |....2.....2.....|
        000000f0  30 00 c8 ff 06 fc 33 00  c7 ff 01 fc 2f 00 c8 ff  |0.....3...../...|
        00000100  02 fc 32 00 c6 ff 04 fc  33 00 c8 ff 05 fc 33 00  |..2.....3.....3.|
        00000110  ca ff 02 fc 31 00 c7 ff  02 fc 30 00 c9 ff 09 fc  |....1.....0.....|
        00000120  35 00 c9 ff 08 fc 35 00  c8 ff 02 fc 31 00 c5 ff  |5.....5.....1...|
        00000130  03 fc 32 00 c7 ff 04 fc  32 00 c7 ff 02 fc 31 00  |..2.....2.....1.|
        00000140  c7 ff 08 fc 30 00 c7 ff  02 fc 32 00 c5 ff ff fc  |....0.....2.....|
        00000150  31 00 c5 ff 04 fc 31 00  c8 ff 03 fc 32 00 c8 ff  |1.....1.....2...|
        00000160  01 fc 31 00 c7 ff 05 fc  31 00 c3 ff 04 fc 31 00  |..1.....1.....1.|
        00000170  c5 ff 04 fc 30 00 c7 ff  03 fc 31 00 c9 ff 03 fc  |....0.....1.....|
        ...

启用活动检测：


        root:/sys/bus/iio/devices/iio:device0> echo 1.28125 > ./events/in_accel_mag_rising_value
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\|y\|z_mag_rising_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        <only while moving the sensor>
        Event: time: 1748795762298351281, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        Event: time: 1748795762302653704, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        Event: time: 1748795762304340726, type: accel(x|y|z), channel: 0, evtype: mag, direction: rising
        ...

禁用活动检测：


        root:/sys/bus/iio/devices/iio:device0> echo 0 > ./events/in_accel_x\|y\|z_mag_rising_en
        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        <nothing>

启用非活动检测：


        root:/sys/bus/iio/devices/iio:device0> echo 1.234375 > ./events/in_accel_mag_falling_value
        root:/sys/bus/iio/devices/iio:device0> echo 5 > ./events/in_accel_mag_falling_period
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\&y\&z_mag_falling_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        Event: time: 1748796324115962975, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1748796329329981772, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1748796334543399706, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...
        <every 5s now indicates inactivity>

现在，启用活动，例如 AC 耦合的对应项 `ACTIVITY_AC`


        root:/sys/bus/iio/devices/iio:device0> echo 1.28125 > ./events/in_accel_mag_rising_value
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\|y\|z_mag_rising_en

        root:/sys/bus/iio/devices/iio:device0> iio_event_monitor adxl313
        Found IIO device with name adxl313 with device number 0
        <some activity with the sensor>
        Event: time: 1748796880354686777, type: accel(x|y|z), channel: 0, evtype: mag_adaptive, direction: rising
        <5s of inactivity, then>
        Event: time: 1748796885543252017, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        <some other activity detected by accelerating the sensor>
        Event: time: 1748796887756634678, type: accel(x|y|z), channel: 0, evtype: mag_adaptive, direction: rising
        <again, 5s of inactivity>
        Event: time: 1748796892964368352, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        <stays like this until next activity in auto-sleep>

注意，当启用 AC 耦合时，事件类型将为 `mag_adaptive`。AC 耦合DC 耦合（默认）事件的使用方式类似

## 4. IIO 接口工具


有关可用 IIO 接口工具的描述，请参Documentation/iio/iio_tools.rst
