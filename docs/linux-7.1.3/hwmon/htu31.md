## 内核驱动 HTU31


支持芯片：

  - Measurement Specialties HTU31

    Prefix: 'htu31'

    Addresses scanned: -

    Datasheet: 公开可从 https://www.te.com/en/product-CAT-HSC0007.html 获取

Author:

  - Andrei Lalaev <andrey.lalaev@gmail.com>

### 说明


HTU31 是一款湿度与温度传感器。

支持的温度范围为 -40 到 125 摄氏度。

与设备的通信通过 I2C 协议进行。传感器默认地址为 0x40。

### sysfs 接口


=================== =================
temp1_input:        温度输入
humidity1_input:    湿度输入
heater_enable:      加热器控制
=================== =================
