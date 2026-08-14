
## AD525x 数字电位器


ad525x_dpot 驱动导出一个简单的 sysfs 接口。这让你既能操作即时电阻设置，也能更新保存的启动设置。还提供对工厂编程容差的访问，但需要根据所使用的特定器件由最终应用对该设置进行解释。

## 文件


每个 dpot 设备都有一组 eeprom、rdac 和 tolerance 文件。具体有多少取决于你拥有的实际器件，允许的取指范围也是如此。

eeprom 文件用于编程设备的启动值。

rdac 文件用于编程设备的即时值。

tolerance 文件是只读的工厂编程容差设置，并且可能在不同器件之间差异很大。有关该字段的确切解释，请参阅你器件的 datasheet。它以 hex 文件形式呈现以便于解析。

## 示例


在你的 sysfs 树中定位该设备。这最简单的方法是进入
```
	# ls /sys/bus/i2c/devices/
	0-0022  0-0027  0-002f
```
因此假设相关设备在第一个 i2c 总线上，并且从设备地址为
```
	# ls /sys/bus/i2c/devices/0-002f/
	eeprom0 rdac0 tolerance0
```
```
	# cd /sys/bus/i2c/devices/0-002f/

	# cat eeprom0
	0
	# echo 10 > eeprom0
	# cat eeprom0
	10

	# cat rdac0
	5
	# echo 3 > rdac0
	# cat rdac0
	3
```
