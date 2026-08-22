## 内核驱动 sht3x


Supported chips:

  - Sensirion SHT3x-DIS

    Prefix: 'sht3x'

    Addresses scanned: none

    Datasheets:
        - https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf
        - https://sensirion.com/media/documents/051DF50B/639C8101/Sensirion_Humidity_and_Temperature_Sensors_Datasheet_SHT33.pdf

  - Sensirion STS3x-DIS

    Prefix: 'sts3x'

    Addresses scanned: none

    Datasheets:
        - https://sensirion.com/media/documents/1DA31AFD/61641F76/Sensirion_Temperature_Sensors_STS3x_Datasheet.pdf
        - https://sensirion.com/media/documents/292A335C/65537BAF/Sensirion_Datasheet_STS32_STS33.pdf

  - Sensirion SHT85

    Prefix: 'sht85'

    Addresses scanned: none

    Datasheet: https://sensirion.com/media/documents/4B40CEF3/640B2346/Sensirion_Humidity_Sensors_SHT85_Datasheet.pdf

Author:

  - David Frey <david.frey@sensirion.com>
  - Pascal Sachs <pascal.sachs@sensirion.com>

### 描述


该驱动实现了Sensirion SHT3x-DIS、STS3x-DIS 以及 SHT85 系列温湿度传感器支持。温度以摄氏度为单位测量，相对湿度以百分比表示。在 sysfs 接口中，所值都放大 1000 倍，31.5 摄氏度对应的值为 31500
该器件通过 I2C 协议通信。SHT3x 传感器根据接线不同，I2C 地址可以0x44 0x45（sts3x 0x4a 0x4b）。SHT85 的地址固定0x44。实例化该器件的方法
请参Documentation/i2c/instantiating-devices.rst
尽管 sht3x 传感器在单次模式下支持时钟延展（阻塞模式）和非延展（非阻塞模式）本驱动仅支持后者
sht3x 传感器支持单次测量模式以5 种周期测量模式，可通过 update_interval
sysfs 接口控制。所允许update_interval（单位毫秒）如下
    ===== ======= ====================
       0          单次测量模式
    2000   0.5 Hz  周期测量
    1000   1   Hz  周期测量
     500   2   Hz  周期测量
     250   4   Hz  周期测量
     100  10   Hz  周期测量
    ===== ======= ====================

在周期测量模式下，传感器以芯片上配置的更新间隔自动触发测量。当温度或湿度读超出配置的限值时，alert 属性被置为 1，且传感器上alert 引脚被置为高电平当温度和湿度读数回到迟滞值之间时，alert 位被置为 0，传感器上的 alert 引脚置为低电平
暴露debugfs 的序列号可用于对传感器进行唯一标识。对sts32、sts33 sht33制造商通过 API 提供校准证书
### sysfs 接口


=================== ============================================================
temp1_input:        温度输入humidity1_input:    湿度输入temp1_max:          温度最大temp1_max_hyst:     温度上限的迟滞humidity1_max:      湿度最大humidity1_max_hyst: 湿度上限的迟滞temp1_min:          温度最小temp1_min_hyst:     温度下限的迟滞humidity1_min:      湿度最小humidity1_min_hyst: 湿度下限的迟滞temp1_alarm:        若温度超出配置的限值，告警标志被置1。告警仅在周期测		    模式下有humidity1_alarm:    若湿度超出配置的限值，告警标志被置1。告警仅在周期测		    模式下有heater_enable:      加热器使能，加热元件用于去除传感器上多余的湿气：

   - 0: 关闭
   - 1: 开update_interval:    更新间隔 表示单次模式，周期测量时单位为毫秒。若传感		    不支持该间隔，则选择下一个更快的间隔
repeatability:      写入或读取重复精度，重复精度越高意味着测量耗时更长、噪                    更低、能耗更大：

                        - 0: 低重复精                        - 1: 中重复精                        - 2: 高重复精=================== ============================================================

### debugfs 接口


=================== ============================================================
serial_number:      传感器的唯一序列号（十进制）
=================== ============================================================
