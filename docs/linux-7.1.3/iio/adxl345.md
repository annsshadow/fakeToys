
## ADXL345 驱动

本驱动支Analog Device ADXL345/375，工作于 SPI/I2C 总线

## 1. 支持的设

- `ADXL345 <https://www.analog.com/ADXL345>`_
- `ADXL375 <https://www.analog.com/ADXL375>`_

ADXL345 是一款通用、低功耗的 3 轴加速度计，支持可选的测量量程。ADXL345 支持以下量程

- 卤2g  (绾，卤19.61 m/s^2)
- 卤4g  (绾，卤39.23 m/s^2)
- 卤8g  (绾，卤78.45 m/s^2)
- 卤16g (绾，卤156.91 m/s^2)

## 2. 设备属

每个 IIO 设备`/sys/bus/iio/devices/iio:deviceX` 下都有一个设备文件夹，其X 是该设备IIO 索引。这些文件夹下包含一组设备文件，具体取决于相关硬件设备的特性与功能。这些文件是统一泛化的，并在 IIO ABI 文档中有说明

下表展示了位于特定设备文件夹路径 `/sys/bus/iio/devices/iio:deviceX` 下的 ADXL345 相关设备文件

+-------------------------------------------+----------------------------------------------------------+
| 3 轴加速度计相关设备文                  | 说明                                                     |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_sampling_frequency               | 当前选定的采样率                                      |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_sampling_frequency_available     | 可用的采样频率配置                                    |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_scale                            | 加速度计各通道的量范围                             |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_scale_available                  | 加速度计通道可用的量程范围                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_x_calibbias                      | X 轴加速度计通道的校准偏置                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_x_raw                            | X 轴加速度计通道的原始值                              |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_y_calibbias                      | Y 轴加速度偏移校正                                    |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_y_raw                            | Y 轴加速度计通道的原始值                              |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_z_calibbias                      | Z 轴加速度计通道的校准偏置                            |
+-------------------------------------------+----------------------------------------------------------+
| in_accel_z_raw                            | Z 轴加速度计通道的原始值                              |
+-------------------------------------------+----------------------------------------------------------+

### 通道处理后的

通道的值可从其 _raw 属性读取。返回的值是设备所报告的原值。要得到该通道的处理后值，请应用以下公式：


        processed value = (_raw + _offset) * _scale

其中 _offset _scale 是设备属性。如果不存在 _offset 属性，则直接假定其值为 0

+-------------------------------------+---------------------------+
| 通道类型                            | 测量单位                  |
+-------------------------------------+---------------------------+
| X、Y、Z 三轴上的加速度              | 米每二次方秒              |
+-------------------------------------+---------------------------+

### 传感器事

特定IIO 事件由其对应的中断触发。传感器驱动支持「无」或「单个」有效中断（INT）线，可INT1 INT2 两个可用选项中选择。有效的 INT 线应在设备树中指定。如果未配置 INT 线，传感器默认进FIFO 旁路模式，此时事件检测被禁用，仅能获取单独的 X、Y、Z 轴测量值

下表列出了位于设备特定路`/sys/bus/iio/devices/iio:deviceX/events` 下的 ADXL345 相关设备文件。注意，活动（activity）与静止（inactivity）检测默认是直流（DC）耦合的；因此，此处仅显式列出交流（AC）耦合的活动与静止事件

+---------------------------------------------+---------------------------------------------+
| 事件句柄                                    | 说明                                        |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_en               | 在所有轴上启用双击检                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_reset_timeout    | 双击窗口，单[us]                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_scale            | 双击手势阈值比例                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_tap2_min_delay   | 双击延迟，单[us]                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_doubletap_value            | 双击阈                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_scale            | 单击手势阈值比例                         |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_timeout          | 单击持续时间，单[us]                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_gesture_singletap_value            | 单击阈                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_period        | 交流耦合静止时间，单位秒                    |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_scale         | 交流耦合静止阈值比例                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_falling_value         | 交流耦合静止阈                           |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_en             | X 轴上启用交流耦合活动检              |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_scale          | 交流耦合活动阈值比例                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_adaptive_rising_value          | 交流耦合活动阈                           |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_period                 | 静止时间，单位秒                            |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_scale                  | 直流耦合静止阈值比例                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_falling_value                  | 静止阈                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_en                      | X 轴上启用活动检                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_scale                   | 直流耦合活动阈值比例                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_mag_rising_value                   | 活动阈                                   |
+---------------------------------------------+---------------------------------------------+
| in_accel_x&y&z_mag_adaptive_falling_en      | 在所有轴上启用交流耦合静止检             |
+---------------------------------------------+---------------------------------------------+
| in_accel_x&y&z_mag_falling_en               | 在所有轴上启用静止检                     |
+---------------------------------------------+---------------------------------------------+
| in_accel_x_gesture_singletap_en             | X 轴上启用单击检                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_y_gesture_singletap_en             | Y 轴上启用单击检                      |
+---------------------------------------------+---------------------------------------------+
| in_accel_z_gesture_singletap_en             | Z 轴上启用单击检                      |
+---------------------------------------------+---------------------------------------------+

有关该功能的具体说明，请参阅传感器的数据手册（datasheet）

手动设置 **ODR** 会使驱动为静止检测时序估算默认值，其中较高ODR 值对应较长的默认等待时间，较低的 ODR 值对应较短的等待时间。如果这些默认值不能满足你的应用需求，你可以显式配置静止等待时间。将该值设0 会恢复默认行为

更改 **g 量程** 配置时，驱动会依据旧量程与新量程之比对默认值进行缩放，从而估算出合适的活动与静止阈值。所得阈值永远不会为零，且始终落1 255 之间，对应数据手册中规定的上62.5 mg/LSB.612915 m/s^2/LSB）。不过，你也可以通过设置显式值来覆盖这些估算阈值

**activity** **inactivity** 事件被启用时，驱动会通过设置 **link** **auto-sleep** 位自动管理迟滞行为。link 位将活动与静止功能相连，使二者相互跟随。auto-sleep 功能在检测到静止时使传感器进入睡眠模式，将功耗降12.5 Hz 以下的速率

静止时间可在 1 255 秒之间配置。除静止检测外，传感器还支持自由落体（free-fall）检测；IIO 的角度看，自由落体被视为所有轴上幅值的下降。就传感器而言，自由落体由一0.000 1.000 秒的静止周期来定义

驱动的行为如下：

- 若配置的静止周期1 秒或以上，驱动使用传感器的静止寄存器。这使得该事件能够与活动检测关联、使auto-sleep，并可采用交流（AC）或直流（DC）耦合

- 若静止周期小1 秒，则该事件被视为普通静止或自由落体检测。此时不应用 auto-sleep 与耦合（AC/DC）

- 若配0 秒的静止时间，驱动会选择一个启发式确定的默认周期（大于 1 秒）以优化功耗。这同样使用静止寄存器

注意：根据数据手册，用于检测活动、静止（或在使用自由落体寄存器时）的最ODR 应落12.5 Hz 400 Hz 之间。推荐的自由落体阈值为 300 mg 600 mg（寄存器0x05 0x09）

在直流（DC）耦合模式下，当前加速度幅值直接与 THRESH_ACT THRESH_INACT 寄存器中的值比较，以判定活动或静止。相比之下，交流（AC）耦合的活动检测以检测开始时的加速度值作为参考点，后续采样与该参考进行比较。直流耦合是默认模式——将实时值与固定阈值比较；而交流耦合则依赖相对于所配置阈值的内部滤波器

交流与直流（DC）耦合模式分别针对活动与静止检测进行配置，但每种检测同一时刻只能有一种模式生效。例如，若先启用交流耦合的活动检测，再设置为直流耦合模式，则只有直流耦合的活动检测会生效。换言之，仅应用最近一次的配置

**Single tap**（单击）检测可按照数据手册，通过设置阈值与持续时间参数来配置。当仅启用单击检测时，只要加速度超过阈值（标志着持续时间的开始）随后又低于阈值（且未超过持续时间上限），就会触发单击中断。若同时启用了单击与双击检测，则单击中断仅在双击事件被确认或取消后才会触发

要配**double tap**（双击）检测，还必须设置窗口与延迟参数，单位为微秒（µs）。延迟期从单击信号低于阈值时开始，作为一段等待时间，在此期间双击检测会忽略任何尖峰。延迟期结束后，检测窗口开始。若加速度在该窗口内先升过阈值、再降回阈值以下，则在降至阈值以下时触发双击事件

双击事件检测在数据手册中有详尽说明。在检测到单击事件后，若信号满足特定条件，可能会跟随一个双击事件。不过，双击检测可能因以下三个原因而失效：

- 若设置了 **suppress bit**，则在点击延迟期内任何超过点击阈值的加速度尖峰都会立即使双击检测失效。换言之，suppress 位激活时，延迟期内不允许出现任何尖峰

- 若双击窗口开始时加速度高于阈值，则双击事件无效

- 若加速度持续时间超过 duration 寄存器设定的上限，双击检测同样会失效

对于双击检测，适用的持续时间与单击相同：加速度必须先升过阈值、再在指定持续时间内降回阈值以下。注意，当双击检测处于活动状态时，通常会启suppress 位

### 使用示例

显示设备名：


        root:/sys/bus/iio/devices/iio:device0> cat name
        adxl345

显示加速度计通道值：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_raw
        -1
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_y_raw
        2
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_z_raw
        -253

设置加速度计通道的校准偏置：


        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        0

        root:/sys/bus/iio/devices/iio:device0> echo 50 > in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> cat in_accel_x_calibbias
        50

给定 13 位全分辨率，可用量程由以下公式计算：


        (g ** 2 ** 9.80665) / (2^(resolution) - 1) * 100; for g := 2|4|8|16

量程配置


        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale
        0.004789
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale_available
        0.004789 0.009578 0.019156 0.038312

        root:/sys/bus/iio/devices/iio:device0> echo 0.019156 > ./in_accel_scale
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_scale
        0.019156

设置输出数据速率（ODR）：


        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency
        200.000000

        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency_available
        0.097000 0.195000 0.390000 0.781000 1.562000 3.125000 6.250000 12.500000 25.000000 50.000000 100.000000 200.000000 400.000000 800.000000 1600.000000 3200.000000

        root:/sys/bus/iio/devices/iio:device0> echo 1.562000 > ./in_accel_sampling_frequency
        root:/sys/bus/iio/devices/iio:device0> cat ./in_accel_sampling_frequency
        1.562000

配置一个或多个事件


        root:> cd /sys/bus/iio/devices/iio:device0

        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./buffer0/in_accel_z_en

        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./scan_elements/in_accel_z_en

        root:/sys/bus/iio/devices/iio:device0> echo 14   > ./in_accel_x_calibbias
        root:/sys/bus/iio/devices/iio:device0> echo 2    > ./in_accel_y_calibbias
        root:/sys/bus/iio/devices/iio:device0> echo -250 > ./in_accel_z_calibbias

        root:/sys/bus/iio/devices/iio:device0> echo 24 > ./buffer0/length

        ## Check the event scale factor (0.0625 * 9.80665)
        root:/sys/bus/iio/devices/iio:device0> cat ./events/in_accel_gesture_doubletap_scale
        0.612915

        ## AC coupled activity, threshold [0.612915 m/s^2/LSB]
        root:/sys/bus/iio/devices/iio:device0> echo 6 > ./events/in_accel_mag_adaptive_rising_value

        ## AC coupled inactivity, threshold, [0.612915 m/s^2/LSB]
        root:/sys/bus/iio/devices/iio:device0> echo 4 > ./events/in_accel_mag_adaptive_falling_value

        ## AC coupled inactivity, time [s]
        root:/sys/bus/iio/devices/iio:device0> echo 3 > ./events/in_accel_mag_adaptive_falling_period

        ## singletap, threshold
        root:/sys/bus/iio/devices/iio:device0> echo 35 > ./events/in_accel_gesture_singletap_value

        ## singletap, duration [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.001875  > ./events/in_accel_gesture_singletap_timeout

        ## doubletap, window [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.025 > ./events/in_accel_gesture_doubletap_reset_timeout

        ## doubletap, latency [us]
        root:/sys/bus/iio/devices/iio:device0> echo 0.025 > ./events/in_accel_gesture_doubletap_tap2_min_delay

        ## AC coupled activity, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_mag_adaptive_rising_en

        ## AC coupled inactivity, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x\&y\&z_mag_adaptive_falling_en

        ## singletap, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_x_gesture_singletap_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_y_gesture_singletap_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_z_gesture_singletap_en

        ## doubletap, enable
        root:/sys/bus/iio/devices/iio:device0> echo 1 > ./events/in_accel_gesture_doubletap_en

验证接收到的事件


        root:# iio_event_monitor adxl345
        Found IIO device with name adxl345 with device number 0
        Event: time: 1739063415957073383, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063415963770218, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063416002563061, type: accel(z), channel: 0, evtype: gesture, direction: singletap
        Event: time: 1739063426271128739, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1739063436539080713, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        Event: time: 1739063438357970381, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446726161586, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446727892670, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446743019768, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446744650696, type: accel(z), channel: 0, evtype: mag, direction: rising
        Event: time: 1739063446763559386, type: accel(z), channel: 0, evtype: gesture, direction: singletap
        Event: time: 1739063448818126480, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...

活动与静止相互关联，并按下述方式指示状态变化：


        root:# iio_event_monitor adxl345
        Found IIO device with name adxl345 with device number 0
        Event: time: 1744648001133946293, type: accel(x), channel: 0, evtype: mag, direction: rising
          <after inactivity time elapsed>
        Event: time: 1744648057724775499, type: accel(x&y&z), channel: 0, evtype: mag, direction: falling
        ...

## 3. 设备缓冲

本驱动支IIO 缓冲区。所有设备都支持通过缓冲区获取原始加速度与温度测量值

### 使用示例

为缓冲读取选择通道


        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_x_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_y_en
        root:/sys/bus/iio/devices/iio:device0> echo 1 > scan_elements/in_accel_z_en

设置缓冲区中存储的样本数量：


        root:/sys/bus/iio/devices/iio:device0> echo 10 > buffer/length

启用缓冲读取


        root:/sys/bus/iio/devices/iio:device0> echo 1 > buffer/enable

获取缓冲数据


        root:> iio_readdev -b 16 -s 1024 adxl345 | hexdump -d
        WARNING: High-speed mode not enabled
        0000000   00003   00012   00013   00005   00010   00011   00005   00011
        0000010   00013   00004   00012   00011   00003   00012   00014   00007
        0000020   00011   00013   00004   00013   00014   00003   00012   00013
        0000030   00004   00012   00013   00005   00011   00011   00005   00012
        0000040   00014   00005   00012   00014   00004   00010   00012   00004
        0000050   00013   00011   00003   00011   00012   00005   00011   00013
        0000060   00003   00012   00012   00003   00012   00012   00004   00012
        0000070   00012   00003   00013   00013   00003   00013   00012   00005
        0000080   00012   00013   00003   00011   00012   00005   00012   00013
        0000090   00003   00013   00011   00005   00013   00014   00003   00012
        00000a0   00012   00003   00012   00013   00004   00012   00015   00004
        00000b0   00014   00011   00003   00014   00013   00004   00012   00011
        00000c0   00004   00012   00013   00004   00014   00011   00004   00013
        00000d0   00012   00002   00014   00012   00005   00012   00013   00005
        00000e0   00013   00013   00003   00013   00013   00005   00012   00013
        00000f0   00004   00014   00015   00005   00012   00011   00005   00012
        ...

有关缓冲数据的结构，请参Documentation/iio/iio_devbuf.rst

## 4. IIO 接口工具

有关可用 IIO 接口工具的说明，请参Documentation/iio/iio_tools.rst
