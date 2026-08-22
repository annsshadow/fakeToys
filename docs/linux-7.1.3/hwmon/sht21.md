## 内核驱动 sht21


支持的设备：

  - Sensirion SHT20

    Prefix: 'sht20'

    Addresses scanned: none

    Datasheet: 公开提供，位Sensirion 网站

    https://www.sensirion.com/file/datasheet_sht20

  - Sensirion SHT21

    Prefix: 'sht21'

    Addresses scanned: none

    Datasheet: 公开提供，位Sensirion 网站

    https://www.sensirion.com/file/datasheet_sht21

  - Sensirion SHT25

    Prefix: 'sht25'

    Addresses scanned: none

    Datasheet: 公开提供，位Sensirion 网站

    https://www.sensirion.com/file/datasheet_sht25

Author:

  Urs Fleisch <urs.fleisch@sensirion.com>

### 描述


SHT21 SHT25 是采DFN 封装的湿度和温度传感器，其尺寸仅3 x 3 mm，高度为 1.1 mm。这两个器件的区别在SHT25 的精度更高（相对湿度 1.8%，温0.2 摄氏度），SHT21 为（相对湿度 2.0%，温0.3 摄氏度）
这些器件通过 I2C 协议通信。所有传感器都设置为相同I2C 地址 0x40，因此可以在板级设置代码中使I2C_BOARD_INFO("sht21", 0x40) 这样一个条目
### sysfs 接口


=================== ============================================================
temp1_input         温度输入
humidity1_input     湿度输入
eic                 电子识别码（Electronic Identification Code=================== ============================================================

### 注意事项


该驱动使用默认的分辨率设置：湿度 12 位，温度 14 位，这导致典型的测量时间为湿22 ms、温66 ms。为使自发热低于 0.1 摄氏度，器件处于活动状态的时间不应超过 10%，例如在给定分辨率下每秒最多两次测量
不同的分辨率、片上加热器以及使用 CRC 校验和目前尚不支持