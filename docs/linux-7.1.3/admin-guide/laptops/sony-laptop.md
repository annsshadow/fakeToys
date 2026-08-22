## Sony Notebook Control Driver (SNC) Readme


 - Copyright (C) 2004- 2005 Stelian Pop <stelian@popies.net>
 - Copyright (C) 2007 Mattia Dongili <malattia@linux.it>

这个迷你驱动驱动 Sony Vaio 笔记本电ACPI BIOS 中存在的 SNC SPIC 设备。该驱动在（尽量一致的）同一接口下混合了这两种设备的功能。这也意味着 sonypi 驱动现在已被 sony-laptop 取代
### Fn keys (hotkeys):


一些型号通过 SNC SPIC 设备报告热键，此类事件既通过 ACPI 子系统作acpi 事件报告，也通过 INPUT 子系统报告。查/proc/bus/input/devices 的日志可以找出这些事件是什么，以及驱动创建了哪些输入设备。此外，使用 debug 选项加载驱动会在内核日志中报告所有事件
传递给输入系统（可以用 udev 重新映射）的“扫描码”是 sony-laptop.c 模块中表 "sony_laptop_input_keycode_map" 的索引。例“FN/E组合键（在某些型号上EJECTCD）生成扫描码 20x14）
### Backlight control:


如果你的笔记本型号支持，你会/sys/class/backlight/sony/
目录下找sysfs 文件。你将能够查询和设置当前屏幕亮度
	======================	=========================================
	brightness		get/set screen brightness (an integer
				between 0 and 7)
	actual_brightness	reading from this file will query the HW
				to get real brightness value
	max_brightness		the maximum brightness value
	======================	=========================================


### Platform specific:


鍔犺浇 sony-laptop 妯″潡浼氬垱寤?/sys/devices/platform/sony-laptop/
目录，其中填充了一些文件
你可以通过标准 UNIX 工具对这些文件进行整数值的写
这些文件是：

	======================	==========================================
	brightness_default	screen brightness which will be set
				when the laptop will be rebooted
	cdpower			power on/off the internal CD drive
	audiopower		power on/off the internal sound card
	lanpower		power on/off the internal ethernet card
				(only in debug mode)
	bluetoothpower		power on/off the internal bluetooth device
	fanspeed		get/set the fan speed
	======================	==========================================

注意，如果某个文件不被你的特定笔记本型号支持，它可能不存在
```

	# echo "1" > /sys/devices/platform/sony-laptop/brightness_default

```
为下次及以后的重启设置最低屏幕亮
```

	# echo "8" > /sys/devices/platform/sony-laptop/brightness_default

```
为下次及以后的重启设置最高屏幕亮
```

	# cat /sys/devices/platform/sony-laptop/brightness_default

```
获取该
```

	# echo "0" > /sys/devices/platform/sony-laptop/audiopower

```
关闭声卡

```

	# echo "1" > /sys/devices/platform/sony-laptop/audiopower

```
打开声卡

### RFkill control:


较新Vaio 型号暴露了一组一致的 ACPI 方法来控制射频发射设备。如果你有幸拥有这样的笔记本，你会在
```

	# grep . /sys/class/rfkill/*/{state,name}


```
下找到所需rfkill 设备

### Development:


如果你想帮助开发这个驱动（并且你不怕对你的 ACPI BIOS 做奇怪的事情可能给你的笔记本带来的任何副作用），加载驱动并传入选项 'debug=1'
REPEAT:
	**如果你不喜欢冒险行为，就不要这样做*

在内核日志中你会找到你的笔记本上 SNC 设备拥有的所ACPI 方法列表
- 对于新型号，你会看到一个长长的无意义方法名列表，阅DSDT 表源码应该能揭示
(1) SNC 设备使用内部能力查找(2) SN00 用于在查找表中查找(3) SN06 SN07 用于根据你可以通过 SN00 迭代表获得的偏移量调用真实方(4) SN02 用于启用事件
能力查找表中的一些值或多或少是已知的，参见所sony_call_snc_handle 调用的代码，其他则更晦涩
- 对于旧型号，你可以看到用于打开/关闭 CD 驱动GCDP/GCDP 方法，但还有其他方法，并且它们通常因型号而异
**我完全不知道那些方法是做什么的*

sony-laptop 驱动为其中一些方法（在多Vaio 型号上找到的最新方法）/sys/devices/platform/sony-laptop 下创建了一个条目，就像 'cdpower' 那样。你可以通过进一步编辑源码（参见 'sony_nc_values' 表，并使SNC_HANDLE_NAMES 宏把你的 get/set 方法名作为新条目加入该表）来创建对应于你自己笔记本方法的其他条目
你的任务（如果你接受的话）是尝试通过从这些文件读/写随机值来找出这些条目是做什么用的，以及它们对你的笔记本有什么影响
如果你发现了任何有趣的东西，请回报给我，我不会否认对你行为的全部了解 :)

另见 http://www.linux.it/~malattia/wiki/index.php/Sony_drivers 获取其他有用信息
### Bugs/Limitations:


- 该驱动并非基Sony 的官方文档（因为根本没有），因此不保证该驱动能工作，或做正确的事。尽管这没有发生在我身上，但该驱动可能对你的笔记本做很糟糕的事，包括永久性损坏
- sony-laptop sonypi 驱动之间完全不交互。将来，sonypi 将被移除并由 sony-laptop 取代
- spicctrl 是用于与 sonypi 驱动（通过 /dev/sonypi）通信的用户空间工具，也已被弃用，因为它的所有特性现在都可以通过 sony-laptop sysfs 树下使用