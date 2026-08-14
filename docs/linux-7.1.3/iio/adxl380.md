
## ADXL380 驱动

本文件描述 Linux IIO 子系统中 ADXL380/ADXL382 三轴加速度计的驱动支持，涵盖其基于 SPI/I2C 的总线连接、测量量程以及对应的 sysfs 设备属性，主要面向驱动使用者与硬件适配者。



本驱动支持 Analog Device 的 ADXL380/382，运行在 SPI/I2C 总线上。

## 1. 受支持的设备


- `ADXL380 <https://www.analog.com/ADXL380>`_
- `ADXL382 <https://www.analog.com/ADXL382>`_

ADXL380/ADXL382 是一款低噪声密度、低功耗、具有可选测量量程的 3 轴加速度计。
ADXL380 支持 ±4 g、±8 g 和 ±16 g 量程，ADXL382 支持 ±15 g、±30 g 和 ±60 g
量程。

## 2. 设备属性


加速度计测量值始终提供。

温度数据也会提供。这些数据可用于监控系统内部温度，或通过校准来改善设备的
温度稳定性。

每个 IIO 设备，在 `/sys/bus/iio/devices/iio:deviceX` 下都有一个设备文件夹，
其中 X 是该设备的 IIO 索引号。根据这些相关硬件设备的特性和功能，在这些文件夹
下会存在一组设备文件。这些文件是统一的，并在 IIO ABI 文档中有说明。

下表显示了位于特定设备文件夹路径 `/sys/bus/iio/devices/iio:deviceX` 下的
adxl380 相关设备文件。

+---------------------------------------------------+----------------------------------------------------------+
| 3 轴加速度计相关设备文件                           | 描述                                                     |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_scale                                    | 加速度计各通道的比例（scale）。                          |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_filter_high_pass_3db_frequency           | 低通滤波器带宽。                                         |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_filter_high_pass_3db_frequency_available | 可用的低通滤波器带宽配置。                               |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_filter_low_pass_3db_frequency            | 高通滤波器带宽。                                         |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_filter_low_pass_3db_frequency_available  | 可用的高通滤波器带宽配置。                               |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_calibbias                              | X 轴加速度计通道的校准偏移（calibration offset）。       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_x_raw                                    | X 轴加速度计通道的原始（raw）值。                        |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_calibbias                              | Y 轴加速度偏移修正                                       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_y_raw                                    | Y 轴加速度计通道的原始（raw）值。                        |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_calibbias                              | Z 轴加速度计通道的校准偏移（calibration offset）。       |
+---------------------------------------------------+----------------------------------------------------------+
| in_accel_z_raw                                    | Z 轴加速度计通道的原始（raw）值。                        |
+---------------------------------------------------+----------------------------------------------------------+

+----------------------------------+--------------------------------------------+
| 温度传感器相关文件               | 描述                                       |
+----------------------------------+--------------------------------------------+
| in_temp_raw                      | 温度通道的原始（raw）值。                  |
+----------------------------------+--------------------------------------------+
| in_temp_offset                   | 温度传感器通道的偏移（offset）。           |
+----------------------------------+--------------------------------------------+
| in_temp_scale                    | 温度传感器通道的比例（scale）。            |
+----------------------------------+--------------------------------------------+

+------------------------------+----------------------------------------------+
| 杂项设备文件                 | 描述                                         |
+------------------------------+----------------------------------------------+
| name                         | IIO 设备的名称。                             |
+------------------------------+----------------------------------------------+
| sampling_frequency           | 当前选择的采样率。                           |
+------------------------------+----------------------------------------------+
| sampling_frequency_available | 可用的采样频率配置。                         |
+------------------------------+----------------------------------------------+

### 通道处理值


可以从通道的 _raw 属性读取通道值。返回的值是设备报告的原始值。要获得通道的
处理值（processed value），应用以下公式：


        processed value = (_raw + _offset) * _scale

其中 _offset 和 _scale 是设备属性。如果不存在 _offset 属性，则直接假定其值为 0。

ADXL380 驱动提供 2 种类型通道的数据，下表显示了处理值的测量单位，它们由 IIO
框架定义：

+-------------------------------------+---------------------------+
| 通道类型                            | 测量单位                  |
+-------------------------------------+---------------------------+
| X、Y、Z 轴上的加速度                | 米每二次方秒              |
+-------------------------------------+---------------------------+
| 温度                                | 毫摄氏度                  |
+-------------------------------------+---------------------------+

### 使用示例


显示设备名称：


	root:/sys/bus/iio/devices/iio:device0> cat name
        adxl382

显示加速度计通道值：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_raw
        -1771
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_y_raw
        282
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_z_raw
        -1523
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_scale
        0.004903325

- X 轴加速度 = in_accel_x_raw * in_accel_scale = −8.683788575 m/s^2
- Y 轴加速度 = in_accel_y_raw * in_accel_scale = 1.38273765 m/s^2
- Z 轴加速度 = in_accel_z_raw * in_accel_scale = -7.467763975 m/s^2

设置加速度计通道的校准偏移：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        0

        root:/sys/bus/iio/devices/iio:device0> echo 50 > in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        50

设置采样频率：


	root:/sys/bus/iio/devices/iio:device0> cat sampling_frequency
        16000
        root:/sys/bus/iio/devices/iio:device0> cat sampling_frequency_available
        16000 32000 64000

        root:/sys/bus/iio/devices/iio:device0> echo 32000 > sampling_frequency
        root:/sys/bus/iio/devices/iio:device0> cat sampling_frequency
        32000

设置加速度计通道的低通滤波器带宽：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_filter_low_pass_3db_frequency
        32000
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_filter_low_pass_3db_frequency_available
        32000 8000 4000 2000

        root:/sys/bus/iio/devices/iio:device0> echo 2000 > in_accel_filter_low_pass_3db_frequency
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_filter_low_pass_3db_frequency
        2000

## 3. 设备缓冲区


本驱动支持 IIO 缓冲区。

所有设备都支持通过缓冲区获取原始加速度和温度测量值。

### 使用示例


为缓冲区读取选择通道：


        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_z_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_temp_en

设置缓冲区中要存储的采样数量：


        root:/sys/bus/iio/devices/iio:device0> echo 10 > buffer/length

启用缓冲区读取：


        root:/sys/bus/iio/devices/iio:device0> echo 1 > buffer/enable

获取缓冲数据：


        root:/sys/bus/iio/devices/iio:device0> hexdump -C /dev/iio\:device0
        ...
        002bc300  f7 e7 00 a8 fb c5 24 80  f7 e7 01 04 fb d6 24 80  |......$.......$.|
        002bc310  f7 f9 00 ab fb dc 24 80  f7 c3 00 b8 fb e2 24 80  |......$.......$.|
        002bc320  f7 fb 00 bb fb d1 24 80  f7 b1 00 5f fb d1 24 80  |......$...._..$.|
        002bc330  f7 c4 00 c6 fb a6 24 80  f7 a6 00 68 fb f1 24 80  |......$....h..$.|
        002bc340  f7 b8 00 a3 fb e7 24 80  f7 9a 00 b1 fb af 24 80  |......$.......$.|
        002bc350  f7 b1 00 67 fb ee 24 80  f7 96 00 be fb 92 24 80  |...g..$.......$.|
        002bc360  f7 ab 00 7a fc 1b 24 80  f7 b6 00 ae fb 76 24 80  |...z..$......v$.|
        002bc370  f7 ce 00 a3 fc 02 24 80  f7 c0 00 be fb 8b 24 80  |......$.......$.|
        002bc380  f7 c3 00 93 fb d0 24 80  f7 ce 00 d8 fb c8 24 80  |......$.......$.|
        002bc390  f7 bd 00 c0 fb 82 24 80  f8 00 00 e8 fb db 24 80  |......$.......$.|
        002bc3a0  f7 d8 00 d3 fb b4 24 80  f8 0b 00 e5 fb c3 24 80  |......$.......$.|
        002bc3b0  f7 eb 00 c8 fb 92 24 80  f7 e7 00 ea fb cb 24 80  |......$.......$.|
        002bc3c0  f7 fd 00 cb fb 94 24 80  f7 e3 00 f2 fb b8 24 80  |......$.......$.|
        ...

关于缓冲数据的结构，更多信息请参阅 Documentation/iio/iio_devbuf.rst。

## 4. IIO 接口工具


关于可用 IIO 接口工具的描述，请参阅 Documentation/iio/iio_tools.rst。
