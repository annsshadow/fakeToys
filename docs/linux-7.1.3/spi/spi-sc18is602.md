## 内核驱动 spi-sc18is602


支持的芯片：

  - NXP SI18IS602/602B/603

    数据手册：https://www.nxp.com/documents/data_sheet/SC18IS602_602B_603.pdf

作者：
        Guenter Roeck <linux@roeck-us.net>


### 描述


该驱动将 NXP SC18IS602/603 I2C 总线SPI 桥接到内核的 SPI 核心子系统
由于 SI18IS602/603 不支持芯ID 寄存器，该驱动不探测受支持的设备。你必须
显式地实例化设备。详Documentation/i2c/instantiating-devices.rst

### 使用说明


该驱动要I2C 适配器驱动支持原I2C 消息。仅能处SMBus 协议I2C 适配驱动不受支持
SC18IS602/603 支持的最SPI 消息大小200 字节。尝试发起更长的传输将以
-EINVAL 失败。EEPROM 读取操作及类似的大访问必须拆分为每个 SPI 消息不超200 字节的多个块（建议每条消128 字节数据）。这意味着"cp" "od" 这样
自动使用大块大小访问设备的程序，不能直接用于EEPROM 读取数据。应改用dd 这样可以指定块大小的程序