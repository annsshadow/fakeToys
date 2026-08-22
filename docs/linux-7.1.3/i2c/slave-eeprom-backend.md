## Linux I2C 从机 EEPROM 后端


作者：Wolfram Sang <wsa@sang-engineering.com> in 2014-20

本后端在连接I2C 总线上模拟一EEPROM。其内存内容
```

	/sys/bus/i2c/devices/<device-directory>/slave-eeprom

```
可用的类型有4c024c324c64 24c512。也支持只读变体
实例化所需的名称形式为 'slave-<type>[ro]'。示例如下：

24c02，读/写，地址 0x64
  # echo slave-24c02 0x1064 > /sys/bus/i2c/devices/i2c-1/new_device

24c512，只读，地址 0x42
  # echo slave-24c512ro 0x1042 > /sys/bus/i2c/devices/i2c-1/new_device

如果在启动时预加载数据，且名'firmware-name' 的设备属
包含一个有效的文件名（仅限 DT ACPI）

截至 2015 年，Linux 不支持对二进sysfs 文件进行 poll，因此当另一
主设备改变内容时不会有通知

