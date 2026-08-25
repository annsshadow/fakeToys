## ACPI 视频扩展


本驱动实现了 ACPI 2.0 规范附录 B 中规定的“显示适配器的 ACPI 扩展”，用于主板上的集成显卡设备，允许执行一些基本控制，例如定义视频 POST 设备、获EDID 信息或设置视频输出等。注意，这仅仅是一个参考实现。它可能对你的集成显卡设备有效，也可能无效

ACPI 视频驱动在背光控制方面做三件事

## 导出 sysfs 接口供用户空间控制背光亮


如果 ACPI 表中含有视频设备，且内核命令行未指定 acpi_backlight=vendor，驱动将注册一个背光设备，并为其设sysfs 接口控制所需的背光操作结构体。对于每个注册的类设备，都会/sys/class/backlight 下生成一个名acpi_videoX 的目录

背光 sysfs 接口的标准定义位于：
Documentation/ABI/stable/sysfs-class-backlight銆。

ACPI 视频驱动所做的工作如下

actual_brightness锛。
  读取时，控制方法 _BQC 会被求值，以获取固件认为当前所处的亮度级别
bl_power锛。
  未实现，会改为设置当前亮度；
brightness锛。
  写入时，控制方法 _BCM 会运行以设置请求的亮度级别；
max_brightness锛。
  _BCL 包（见下文）推导得出
type锛。
  firmware

注意，ACPI 视频背光驱动始终使用索引来表brightness、actual_brightness max_brightness。因此，如果我们
```

	Method (_BCL, 0, NotSerialized)
	{
		Return (Package (0x0C)
		{
			0x64,
			0x32,
			0x0A,
			0x14,
			0x1E,
			0x28,
			0x32,
			0x3C,
			0x46,
			0x50,
			0x5A,
			0x64
		})
	}

```
前两个级别用于笔记本电脑使用交流电或电池时，目前 Linux 未使用。其10 个级别为受支持的级别，可供我们选择。适用的索引值范围从 0（对0x0A 亮度值）9（对0x64 亮度值），含端点。这些索引值中的每一个都被视为一个“亮度级别”指示符。因此从用户空间的角度看，可用亮度级别的范围是从 0 9（max_brightness），含端点

## 向用户空间通知热键事件


热键事件上报一般有两种情况

i) 对于某些笔记本电脑，当用户按下热键时，会生成一个扫描码（scancode），并通过键盘驱动创建的输入设备以按键类型输入事件的形式发送给用户空间，经过适当的重映射后，
```

	EV_KEY, KEY_BRIGHTNESSUP
	EV_KEY, KEY_BRIGHTNESSDOWN
	etc.

```
对于这种情况，ACPI 视频驱动无需做任何事情（实际上它甚至不知道发生了该事件）

ii) 对于某些笔记本电脑，按下热键不会生成扫描码，而是固件会就这一事件通知视频设备ACPI 节点。事件值在 ACPI 规范中定义。ACPI 视频驱动会根据收到的通知值生成一个按键类型的输入事件，并通过其创建的输入设备将该事件发送给用户空间

	=====		==================
	event		keycode
	=====		==================
	0x86		KEY_BRIGHTNESSUP
	0x87		KEY_BRIGHTNESSDOWN
	etc.
	=====		==================

因此现在这会产生与情i) 相同的效果

一旦用户空间工具收到该事件，它就可以通过 sysfs 接口修改背光亮度

## 在内核中改变背光级别


这适用于第 2 节中情况 ii) 所覆盖的机器。一旦驱动收到通知，它就会相应地设置背光级别。这并不影响向用户空间发送事件，无论视频模块是否直接控制背光级别，事件都始终被发送到用户空间。此行为可通过 brightness_switch_enabled 模块参数进行控制，详admin-guide/kernel-parameters.rst。建议在 GUI 环境启动并希望对背光级别拥有完全控制时禁用此行为
