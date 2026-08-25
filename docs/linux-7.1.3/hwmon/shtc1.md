## 内核驱动 shtc1


支持的芯片：

  - Sensirion SHTC1

    Prefix: 'shtc1'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtc1



  - Sensirion SHTW1

    Prefix: 'shtw1'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtw1



  - Sensirion SHTC3

    Prefix: 'shtc3'

    Addresses scanned: none

    Datasheet: https://www.sensirion.com/file/datasheet_shtc3



Author:

  Johannes Winkelmann <johannes.winkelmann@sensirion.com>

### 描述


该驱动实现了Sensirion SHTC1、SHTW1 SHTC3 芯片的支持，这些芯片是温湿度传感器。温度以摄氏度为单位测量，相对湿度以百分比表示

该设备通过 I2C 协议通信。所有传感器I2C 地址均设0x70。实例化该设备的方法请参Documentation/i2c/instantiating-devices.rst

可通过 shtc1_platform_data 配置两个选项

1. 阻塞模式（在执行测量时拉I2C 时钟线）
   非阻塞模式。阻塞模式能保证最快的结果，但
   I2C 总线在此期间将处于繁忙状态。默认使用非阻塞模式
   如果要使用阻塞模式，请确保设备上的时钟延展（clock-stretching）工作正常
2. 高精度或低精度。默认使用高精度，强烈建议使用高精度

### sysfs 接口


temp1_input
 - 温度输入
humidity1_input
 - 湿度输入
