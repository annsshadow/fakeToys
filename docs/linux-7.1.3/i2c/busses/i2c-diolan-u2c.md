## 内核驱动 i2c-diolan-u2c


支持的适配器：
  - Diolan U2C-12 I2C-USB adapter

    Documentation:
	http://www.diolan.com/i2c/u2c12.html

Author: Guenter Roeck <linux@roeck-us.net>

### Description


这是 Diolan U2C-12 USB-I2C 适配器的驱动

Diolan U2C-12 I2C-USB 适配器提供了一种低成本方案，通过 USB 接口将计算机连接I2C 从设备。它也支持连SPI 设备

该驱动仅支持 U2C-12 I2C 接口。该驱动不使用中断


### 模块参数


- frequency: I2C 总线频率
