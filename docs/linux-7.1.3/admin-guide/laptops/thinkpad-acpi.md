## ThinkPad ACPI Extras 驱动

版本 0.25

2013 年 10 月 16 日

- Borislav Deianov <borislav@users.sf.net>
- Henrique de Moraes Holschuh <hmh@hmh.eng.br>

http://ibm-acpi.sf.net/

这是一个面向 IBM 和联想（Lenovo）ThinkPad 笔记本电脑的 Linux 驱动。它
支持这些笔记本电脑中可通过 ACPI 与 ACPI EC 框架访问、但通用 Linux ACPI
驱动未提供完整支持的各种功能特性。

该驱动在 kernel 2.6.21 及发布版本 0.13-20070314 之前名为 ibm-acpi。它曾
位于 drivers/acpi 目录下，在 kernel 2.6.22 及发布版本 0.14 中，被移动到
drivers/misc 目录下并更名为 thinkpad-acpi。在 kernel 2.6.29 及发布版本
0.22 中，它被移动到 drivers/platform/x86。

该驱动名为 "thinkpad-acpi"。在某些地方（如模块名与日志消息）由于用户空间
的兼容性问题，使用的是 "thinkpad_acpi"。

"tpacpi" 是 "thinkpad-acpi" 的简写，用于某些 Linux 内核版本因长度限制而
无法容纳完整名称之处。

### 状态

当前支持的功能如下（详细说明见下文）：

 - Fn 键组合
 - 蓝牙（Bluetooth）启用与禁用
 - 视频输出切换、扩展控制
 - ThinkLight 开与关
 - CMOS/UCMS 控制
 - LED 控制
 - ACPI 声音
 - 温度传感器
 - 实验性：嵌入式控制器寄存器转储
 - LCD 亮度控制
 - 音量控制
 - 风扇控制与监控：风扇转速、风扇启用/禁用
 - WAN 启用与禁用
 - UWB 启用与禁用
 - LCD 阴影（PrivacyGuard）启用与禁用
 - 膝上模式（Lap mode）传感器
 - 设置键盘语言
 - WWAN 天线类型
 - Auxmac
 - 硬件损坏检测能力

按型号与功能划分的兼容性表格维护于网站
http://ibm-acpi.sf.net/。欢迎提交任何成功或失败的测试报告，尤其是能够补充或
修正兼容性表格的报告。请在报告中包含以下信息：

 - ThinkPad 型号名称
 - 使用 "acpidump" 工具获取的 ACPI 表副本
 - dmidecode 的输出副本，其中序列号与 UUID 需做掩码处理
 - 哪些驱动功能可用、哪些不可用
 - 不可用功能所观察到的行为

其他任何意见或补丁同样非常欢迎提交。

### 安装

如果你正在编译作为 Linux 内核源码一部分的该驱动，请查找
CONFIG_THINKPAD_ACPI 这个 Kconfig 选项。它位于菜单路径："Device Drivers"
-> "X86 Platform Specific Device Drivers" -> "ThinkPad ACPI Laptop Extras"。

### 功能

该驱动向用户空间导出两个不同的接口，可用于访问它提供的功能。其一是遗留的
基于 procfs 的接口，该接口将在未来的某个时刻被移除。其二是新的基于 sysfs
的接口，目前尚不完整。

procfs 接口会创建 /proc/acpi/ibm 目录。该目录下为每个所支持的功能提供了一
个文件。procfs 接口基本处于冻结状态，即使有变化也极少：它不会被扩展以添加
驱动中的任何新功能，所有新功能都将实现在 sysfs 接口上。

sysfs 接口尽可能与通用的 Linux sysfs 子系统与类相融合。由于其中某些子系统
尚未就绪或尚未稳定，预计该接口会发生变动，所有用户空间程序都必须对此作出
处理。

##### 关于 sysfs 接口的注意事项

与 procfs 接口的做法不同，与 sysfs 接口交互时的正确性将被强制要求，thinkpad-acpi
实现 sysfs 接口的正确性同样如此。

此外，thinkpad-acpi 的 sysfs 驱动代码或其对 sysfs 接口的实现中的任何 bug 都
将被修复以获得最大正确性，即便这意味着以不兼容的方式更改某个接口。随着这些接口
在内核与 thinkpad-acpi 中逐渐成熟，此类改动应当会变得相当罕见。

与 thinkpad-acpi 的 sysfs 接口交互的应用程序必须遵循所有 sysfs 规范，并
正确处所有错误（sysfs 接口大量使用错误返回）。对 sysfs inode 的文件描述符
以及 open/close 操作也必须被正确实现。

thinkpad-acpi 的 sysfs 接口的版本由驱动作为一个驱动属性导出（见下文）。

sysfs 驱动属性位于驱动的 sysfs 属性空间中，对于 2.6.23+，该空间为
/sys/bus/platform/drivers/thinkpad_acpi/ 以及
/sys/bus/platform/drivers/thinkpad_hwmon/。

sysfs 设备属性位于 thinkpad_acpi 设备的 sysfs 属性空间中，对于 2.6.23+，
该空间为 /sys/devices/platform/thinkpad_acpi/。

传感器与风扇的 sysfs 设备属性位于 thinkpad_hwmon 设备的 sysfs 属性空间中，
但你应该通过查找 name 属性为 "thinkpad" 的 hwmon 设备来定位它，或者更好的
做法是借助 libsensors。对于 4.14+，sysfs 属性已被移动到 hwmon 设备
（/sys/bus/platform/devices/thinkpad_hwmon/hwmon/hwmon? 或
/sys/class/hwmon/hwmon?）。

### 驱动版本

procfs: /proc/acpi/ibm/driver

sysfs 驱动属性：version

驱动名称与版本。不能向该文件写入任何命令。

### sysfs 接口版本

sysfs 驱动属性：interface_version

thinkpad-acpi 的 sysfs 接口版本，以无符号长整型形式给出（以十六进制格式输出：
0xAAAABBCC），其中：

	AAAA
   - 主版本号
	BB
   - 次版本号
	CC
   - bugfix 修订号

该驱动的 sysfs 接口版本变更日志可在本文档末尾找到。由内核各子系统所做出的
sysfs 接口变更不在此处记录，也不由该属性跟踪。

thinkpad-acpi 的 sysfs 接口的变更，只有在被提交到 Linux 主线（mainline）时
才被视为非实验性的，此时该接口的变更会被记录下来，且 interface_version 可能
会被更新。如果你正在使用任何尚未发送到主线合并的 thinkpad-acpi 功能，需自行
承担风险：这些功能可能会消失，或在被合并进 Linux 主线时以不同的、不兼容的方式
实现。

本质上向后兼容的变更（例如新增不会改变其他属性工作方式的属性）并不总是需要
更新 interface_version。因此，必须预期某个属性可能并不存在，并对此作出正确
处理（某个属性不存在**本身**就是表明某项功能在 sysfs 中不可用的有效方式）。

### 热键（Hot keys）

procfs: /proc/acpi/ibm/hotkey

sysfs 设备属性：hotkey_*

在 ThinkPad 中，ACPI HKEY 处理器负责将一些重要事件以及键盘热键按键传达给
操作系统。启用 thinkpad-acpi 的热键功能即向固件表明存在这样一个驱动，并会
改变 ThinkPad 固件在许多场景下的行为。

驱动在加载时会自动启用 HKEY（"hot key"）事件上报，并在移除时禁用它。

```

	ibm/hotkey HKEY 00000080 0000xxxx

```
其中部分事件涉及热键按键，但并非全部。

驱动将通过输入层（input layer）为热键与无线开关（radio switch）生成事件，
并通过 ACPI netlink 层为其他事件生成事件。输入层支持标准的 IOCTL 来重映射
分配给每个热键的键码（keycode）。

热键位掩码（bit mask）可控制哪些热键生成事件。如果一个键被"掩码"（masked，
即该位在掩码中设为 0），则由固件处理它。如果它被"解除掩码"（unmasked），则
向固件表明 thinkpad-acpi 更希望由自己来处理它——如果固件愿意允许的话（而它
常常不允许！）。

并非掩码中的所有位都可以被修改。并非所有可修改的位都会起作用。并非所有热键
都能被掩码单独控制。某些型号根本不支持掩码。因此，掩码的行为高度依赖于
ThinkPad 型号。

驱动会过滤掉任何已解除掩码的热键，因此即使固件不允许禁用某个特定热键，驱动
也不会为已解除掩码的热键上报事件。

请注意，解除某些键的掩码会阻止它们的默认行为。例如，如果解除 Fn+F5 的掩码，
该键将不再在固件层面自行启用/禁用蓝牙。

还需注意，并非所有 Fn 键组合都通过 ACPI 受支持，这取决于 ThinkPad 型号与
固件版本。在这些 ThinkPad 上，仍有可能通过以每秒至少 10 次的频率轮询
"CMOS NVRAM" 来支持一些额外的热键。驱动会在需要时尝试自动启用该功能。

##### procfs 注意事项

```

	echo 0xffffffff > /proc/acpi/ibm/hotkey -- 启用所有热键
	echo 0 > /proc/acpi/ibm/hotkey -- 禁用所有可能热键
	... 任何其他 8 位十六进制掩码 ...
	echo reset > /proc/acpi/ibm/hotkey -- 恢复推荐掩码

```
以下命令已被废弃，并会导致内核
```

	echo enable > /proc/acpi/ibm/hotkey -- 无作用
	echo disable > /proc/acpi/ibm/hotkey -- 返回错误

```
procfs 接口不支持 NVRAM 轮询控制。为了保持最大的 bug 级（bug-to-bug）兼容性，
它不报告任何掩码，也不允许在固件根本不支持掩码时操纵热键掩码，即使正在使用
NVRAM 轮询也是如此。

##### sysfs 注意事项

	hotkey_bios_enabled:
		已废弃，即将被移除。

		返回 0。

	hotkey_bios_mask:
		已废弃，请勿使用，未来将被移除。

		返回 thinkpad-acpi 加载时的热键掩码。
		在模块卸载时，热键掩码会被恢复为该值。它始终为 0x80c，
		因为那些是没有掩码支持的古旧固件所支持的热键。

	hotkey_enable:
		已废弃，即将被移除。

		0：返回 -EPERM
		1：无作用

	hotkey_mask:
		位掩码，用于启用对每个热键的事件上报（以及根据固件情况，
		ACPI 事件生成）（见上文）。返回热键掩码的当前状态，并允许
		对其作出修改。

	hotkey_all_mask:
		位掩码，当被回显（echo）到上面的 hotkey_mask 时应能启用
		所有受支持热键的事件上报。除非你知道哪些事件需要被被动
		处理（因为固件**会**自行处理它们），否则**不要**使用
		hotkey_all_mask，而应使用 hotkey_recommended_mask。
		已事先警告。

	hotkey_recommended_mask:
		位掩码，应能启用所有受支持热键的事件上报，但那些始终由
		固件处理的除外。将其回显到上面的 hotkey_mask 以使用。
		这是驱动使用的默认掩码。

	hotkey_source_mask:
		位掩码，选择驱动将对哪些热键轮询 NVRAM。它由驱动根据
		ACPI 固件报告的能力自动检测，但也可在运行时被覆盖。

		在 hotkey_source_mask 中置位的那些热键会被在 NVRAM 中轮询，
		并在 hotkey_mask 中启用时作为热键事件上报。只有少数热键
		可通过 CMOS NVRAM 轮询获得。

		警告：在 NVRAM 模式下，音量增/减/静音键是根据混音器
		（mixer）的变化合成出来的，它利用单次音量增或音量减热键
		按键来取消静音，这与 ThinkPad 音量混音器用户界面一致。
		在 ACPI 事件模式下，音量增/减/静音事件由固件报告，行为
		可能有所不同（且该行为会随固件版本——不仅随固件型号——以及
		OSI(Linux) 状态而变化）。

	hotkey_poll_freq:
		热键轮询频率，单位为 Hz。其值必须介于 0 到 25 Hz 之间。
		仅在确实必要时才进行轮询。

		将 hotkey_poll_freq 设为零会禁用轮询，并将导致需要 NVRAM
		轮询的热键按键永不被上报。

		将 hotkey_poll_freq 设置得过低可能导致同一热键的重复按下
		被误报为单次按键，甚至根本检测不到。推荐的轮询频率为 10Hz。

	hotkey_radio_sw:
		如果 ThinkPad 配有硬件无线开关，则当开关处于"无线已禁用"
		位置时该属性读为 0，处于"无线已启用"位置时读为 1。

		该属性支持 poll()/select()。

	hotkey_tablet_mode:
		如果 ThinkPad 具有平板（tablet）能力，则当 ThinkPad 处于
		正常模式时该属性读为 0，处于平板模式时读为 1。

		该属性支持 poll()/select()。

	wakeup_reason:
		如果系统因用户请求弹出扩展坞（bay）而唤醒，则设为 1。
		如果系统因用户请求系统取消扩展坞（undock）而唤醒，则设为 2。
		对于正常的唤醒或由于未知原因的唤醒，设为零。

		该属性支持 poll()/select()。

	wakeup_hotunplug_complete:
		如果系统因某个取消扩展坞或弹出扩展坞的请求而唤醒，且该
		请求已成功完成，则设为 1。此时，根据用户的选择，将系统
		重新置为休眠可能是有用的。参见下文的 HKEY 事件 0x4003
		与 0x3003。

		该属性支持 poll()/select()。

##### 输入层注意事项

一个热键被映射为单个输入层 EV_KEY 事件，其后可能跟随一个 EV_MSC MSC_SCAN
事件，其中包含该键的扫描码（scan code）。始终会生成一个 EV_SYN 事件以标记
事件块的结束。

不要使用 EV_MSC MSC_SCAN 事件来处理按键。它们仅作为重映射按键的辅助手段。
在重映射 KEY_UNKNOWN 键时它们特别有用。

这些事件可通过一个输入设备获得，其 id 如下：

	==============  ==============================
	Bus		BUS_HOST
	vendor		0x1014 (PCI_VENDOR_ID_IBM)  或
			0x17aa (PCI_VENDOR_ID_LENOVO)
	product		0x5054 ("TP")
	version		0x4101
	==============  ==============================

如果该输入设备的键映射以向后兼容的方式发生变化，其 LSB 会递增。其 MSB 对于
该输入设备应始终为 0x41。如果 MSB 不是 0x41，请勿按本节所述使用该设备，因为
它要么是别的东西（例如另一个由 thinkpad 驱动导出的输入设备，如 HDAPS），要么
其功能已以不向后兼容的方式发生了改变。

为此输入设备添加其他类型的事件应被视为向后兼容的变更。

Thinkpad-acpi 热键事件映射（版本 0x4101）：

=======	=======	==============	==============================================
ACPI	Scan
event	code	Key		说明
=======	=======	==============	==============================================
0x1001	0x00	FN+F1		-

0x1002	0x01	FN+F2		IBM：电池（罕见）
				Lenovo：屏幕锁定

0x1003	0x02	FN+F3		许多 IBM 型号总是上报此热键，
				即使在热键被禁用或 Fn+F3 被掩码
				关闭时也是如此
				IBM：屏幕锁定，常作为副作用关闭
				ThinkLight
				Lenovo：电池

0x1004	0x03	FN+F4		睡眠按钮（ACPI 睡眠按钮语义，
				即睡眠至内存（sleep-to-RAM））。
				它总是会生成某种事件，要么是热键
				事件，要么是 ACPI 睡眠按钮事件。
				固件可能拒绝生成进一步的 FN+F4
				按键事件，直到执行了一次 S3 或 S4 的
				ACPI 睡眠周期或经过一段时间。

0x1005	0x04	FN+F5		无线（Radio）。在固件控制下，
				启用/禁用内部蓝牙硬件与 W-WAN 卡。
				不影响 WLAN 卡。
				真正应当用于开启/关闭所有无线
				（Bluetooth+W-WAN+WLAN）。

0x1006	0x05	FN+F6		-

0x1007	0x06	FN+F7		视频输出循环。
				今天你觉得自己走运吗？

0x1008	0x07	FN+F8		IBM：切换屏幕扩展
				Lenovo：配置 UltraNav，或切换屏幕扩展。
				在 2024 平台上被 0x131f（见下文）取代，
				在更新的（2025+）平台上键码被 0x1401
				（见下文）取代。

0x1009	0x08	FN+F9		-

...	...	...		...

0x100B	0x0A	FN+F11		-

0x100C	0x0B	FN+F12		睡眠到磁盘（Sleep to disk）。你应始终
				自行处理它，无论是通过 ACPI 事件还是
				通过热键事件。
				固件可能拒绝生成进一步的 FN+F12 按键
				事件，直到执行了一次 S3 或 S4 的 ACPI
				睡眠周期，或经过一段时间。

0x100D	0x0C	FN+BACKSPACE	-
0x100E	0x0D	FN+INSERT	-
0x100F	0x0E	FN+DELETE	-

0x1010	0x0F	FN+HOME		亮度增。此键在 IBM ThinkPad 上总是由
				固件处理，即使解除掩码也是如此。
				就让它保持原样。对于具有新 BIOS 的
				Lenovo ThinkPad，它必须由 ACPI OSI
				或用户空间处理。驱动会做正确的事，
				绝不要动它。
0x1011	0x10	FN+END		亮度减。详见亮度增。

0x1012	0x11	FN+PGUP		ThinkLight 切换。此键总是由固件处理，
				即使解除掩码也是如此。

0x1013	0x12	FN+PGDOWN	-

0x1014	0x13	FN+SPACE	缩放（Zoom）键

0x1015	0x14	VOLUME UP	内部混音器音量增。此键总是由固件处理，
				即使解除掩码也是如此。
				注意：Lenovo 似乎正在改变这一行为。
0x1016	0x15	VOLUME DOWN	内部混音器音量增。此键总是由固件处理，
				即使解除掩码也是如此。
				注意：Lenovo 似乎正在改变这一行为。
0x1017	0x16	MUTE		内部混音器静音。此键总是由固件处理，
				即使解除掩码也是如此。

0x1018	0x17	THINKPAD	ThinkPad/Access IBM/Lenovo 键

0x1019	0x18	unknown

0x131f	...	FN+F8		平台模式变更（2024 系统）。
				由驱动实现。

0x1401	...	FN+F8		平台模式变更（2025+ 系统）。
				由驱动实现。
...	...	...

0x1020	0x1F	unknown
=======	=======	==============	==============================================

ThinkPad 固件不允许区分大多数热键是按下还是释放（要么如此，要么是我们还
不知道如何区分）。对于这些键，驱动会为按键按下生成一组事件，并立即为按键
释放发出同一组事件。驱动并不知道 ThinkPad 固件是在热键按下还是释放时触发
这些事件的，但固件会对其中一种情况触发，而非两种。

如果一个键被映射为 KEY_RESERVED，则它根本不生成任何输入事件。如果一个键被
映射为 KEY_UNKNOWN，则它会生成一个包含扫描码的输入事件。如果一个键被映射为
其他任何值，则它会生成输入设备的 EV_KEY 事件。

除 EV_KEY 事件外，thinkpad-acpi 还可能针对开关发出 EV_SW 事件：

==============	==============================================
SW_RFKILL_ALL	T60 及更晚硬件的 rfkill 摇杆开关
SW_TABLET_MODE	平板型 ThinkPad 的 HKEY 事件 0x5009 与 0x500A
==============	==============================================

### 非热键的 ACPI HKEY 事件映射

驱动从不传播的以下事件：

======		==================================================
0x2304		系统正从挂起（suspend）中唤醒以取消扩展坞
0x2305		系统正从挂起中唤醒以弹出扩展坞
0x2404		系统正从休眠（hibernation）中唤醒以取消扩展坞
0x2405		系统正从休眠中唤醒以弹出扩展坞
0x5001		盖子关闭
0x5002		盖子打开
0x5009		平板旋转：切换到平板模式
0x500A		平板旋转：切换到正常模式
0x5010		亮度级别已变更/控制事件
0x6000		键盘：Numlock 键被按下
0x6005		键盘：Fn 键被按下（待验证）
0x7000		无线开关可能已改变状态
======		==================================================

由驱动传播到用户空间的以下事件：

======		=====================================================
0x2313		警报：系统因电池即将耗尽而从挂起中唤醒
0x2413		警报：系统因电池即将耗尽而从休眠中唤醒
0x3003		扩展坞弹出完成（见 0x2x05），可再次休眠
0x3006		扩展坞热插拔请求（当光驱托盘被弹出时，提示给
		SATA 链路加电）
0x4003		已取消扩展坞（见 0x2x04），可再次休眠
0x4010		已接入热插拔端口复制器（非 ACPI 扩展坞）
0x4011		已从热插拔端口复制器取消扩展坞（非 ACPI 扩展坞）
0x500B		平板笔已插入其存放仓
0x500C		平板笔已从其存放仓取出
0x6011		警报：电池过热
0x6012		警报：电池极热
0x6021		警报：某传感器过热
0x6022		警报：某传感器极热
0x6030		系统散热表已变更
0x6032		热控制命令集完成（DYTC，Windows）
0x6040		Nvidia Optimus/交流适配器相关（待验证）
0x60C0		X1 Yoga 2016，平板模式状态已变更
0x60F0		热变换已变更（GMTS，Windows）
======		=====================================================

电池即将耗尽的警报是让操作系统干净地休眠或关机的最后手段（0x2313），或在
断电前干净地关机（0x2413）。必须对它们作出响应，因为由固件引起的唤醒将已
使大多数安全网失效……

当任何"过热"警报发生时，根据 Lenovo 的建议，用户应当挂起或休眠该笔记本
（对于电池警报，还应拔下交流适配器）以使其冷却。这些警报确实表明出现了
异常，它们绝不应在正常操作条件下发生。

"极热"警报属于紧急情况。根据 Lenovo 的建议，操作系统应强制立即挂起或
休眠，或系统关机。显然，如果发生这种情况，说明出现了非常严重的问题。

##### 亮度热键注意事项

不要去动 Thinkpad 上的亮度热键。如果你想要屏幕显示（OSD）通知，请使用
sysfs 背光类（backlight class）的事件支持。

对于需要用户空间采取动作来实现亮度变更的情况，驱动会自动发出
KEY_BRIGHTNESS_UP 与 KEY_BRIGHTNESS_DOWN 事件。当你覆盖这些事件时，你
要么无法正确处理那些需要显式动作来改变背光亮度的 ThinkPad，要么会破坏那些
不需要任何动作就能正常工作的 ThinkPad。

### 蓝牙（Bluetooth）

procfs: /proc/acpi/ibm/bluetooth

sysfs 设备属性：bluetooth_enable（已废弃）

sysfs rfkill 类：开关 "tpacpi_bluetooth_sw"

该功能显示 ThinkPad 内部 ThinkPad CDC 插槽中蓝牙设备的存在与当前状态。

如果 ThinkPad 支持，蓝牙状态会被存储在 NVRAM 中，因此在重启与关机后都会
保留。

##### Procfs 注意事项

```

	echo enable > /proc/acpi/ibm/bluetooth
	echo disable > /proc/acpi/ibm/bluetooth

```
##### Sysfs 注意事项

	如果安装了蓝牙 CDC 卡，可通过 "bluetooth_enable" 这个 thinkpad-acpi
	设备属性启用/禁用它，并也可查询其当前状态。

	enable:

  - 0：禁用蓝牙 / 蓝牙已禁用
  - 1：启用蓝牙 / 蓝牙已启用。

	注意：该接口已被通用的 rfkill 类取代。它已被废弃，并将于 2010 年
	被移除。

	rfkill 控制器开关 "tpacpi_bluetooth_sw"：详见
	Documentation/driver-api/rfkill.rst。

### 视频输出控制 -- /proc/acpi/ibm/video

该功能用于控制用于视频输出的设备——
```

	echo lcd_enable > /proc/acpi/ibm/video
	echo lcd_disable > /proc/acpi/ibm/video
	echo crt_enable > /proc/acpi/ibm/video
	echo crt_disable > /proc/acpi/ibm/video
	echo dvi_enable > /proc/acpi/ibm/video
	echo dvi_disable > /proc/acpi/ibm/video
	echo auto_enable > /proc/acpi/ibm/video
	echo auto_disable > /proc/acpi/ibm/video
	echo expand_toggle > /proc/acpi/ibm/video
	echo video_switch > /proc/acpi/ibm/video

```
注意：
  出于安全原因，对该功能的访问仅限于拥有 CAP_SYS_ADMIN 能力的进程，
  因为它可能与某些版本的 X.org 配合得很差，导致其崩溃。

每个视频输出设备都可被单独启用或禁用。读取 /proc/acpi/ibm/video 会显示
每个设备的状态。

自动视频切换可被启用或禁用。当启用自动视频切换时，某些事件（例如打开盖子、
接入或移除扩展坞）会导致视频输出设备自动变更。虽然这可能有用，但也会造成
闪烁，并且在 X40 上还会造成视频损坏。通过禁用自动切换，可以避免闪烁或
视频损坏。

video_switch 命令会循环遍历可用的视频输出（它模拟了 Fn-F7 的行为）。

可通过该功能切换视频扩展。这控制着当使用低于全分辨率模式时，显示是否被
扩展以填满整个 LCD 屏幕。请注意，无法通过该功能确定当前的视频扩展状态。

请注意，在许多型号上（尤其是使用 Radeon 显卡芯片的型号），X 驱动会以某种
方式配置显卡，从而阻止 Fn-F7 工作。这也会禁用该驱动的视频输出切换功能，
因为它使用与 Fn-F7 相同的 ACPI 方法。在控制台上进行视频切换仍应可用。

更新：参见 https://bugs.freedesktop.org/show_bug.cgi?id=2000

### ThinkLight 控制

procfs: /proc/acpi/ibm/light

**sysfs 属性：遵循 LED 类，针对 "tpacpi**
: thinklight" 这一 LED

##### procfs 注意事项

ThinkLight 的状态可通过 procfs 接口读取与设置。少数无法提供状态的型号会
显示 ThinkLight
```

	echo on  > /proc/acpi/ibm/light
	echo off > /proc/acpi/ibm/light

```
##### sysfs 注意事项

ThinkLight 的 sysfs 接口由 LED 类的文档记录，位于 Documentation/leds/leds-class.rst。
ThinkLight LED 的名称**为 "tpacpi**
: thinklight"。

由于 sysfs LED 类的限制，如果 ThinkLight 的状态无法读取或未知，thinkpad-acpi
会将其报告为 "off"。无法得知通过 sysfs 返回的状态是否有效。

### CMOS/UCMS 控制

procfs: /proc/acpi/ibm/cmos

sysfs 设备属性：cmos_command

该功能主要由 ACPI 固件在内部使用，以使遗留的 CMOS NVRAM 位与当前机器状态
保持同步，并记录该状态，以便 ThinkPad 在重启后保留这些设置。

其中某些命令会在部分 ThinkPad 型号上真正执行动作，但预计在较新型号中会
越来越少。例如，在 T43 与 X40 上，命令 12 与 13 仍然真正控制 ThinkLight
状态，但命令 0 到 2 已不再控制混音器（它们已被淘汰），只是更新 NVRAM。

有效 cmos 命令号的范围是 0 到 21，但并非所有命令都有效，且行为因型号而异。
以下是 X40 上的行为（tpb 是 ThinkPad Buttons 工具）：

 - 0 - 与"音量减"键按下相关
 - 1 - 与"音量增"键按下相关
 - 2 - 与"静音开"键按下相关
 - 3 - 与"Access IBM"键按下相关
 - 4 - 与"LCD 亮度增"键按下相关
 - 5 - 与"LCD 亮度减"键按下相关
 - 11 - 与"切换屏幕扩展"键按下/功能相关
 - 12 - 与"ThinkLight 开"相关
 - 13 - 与"ThinkLight 关"相关
 - 14 - 与"ThinkLight"键按下相关（切换 ThinkLight）

cmos 命令接口容易出现固件"分裂大脑"（split-brain）问题，因为在较新的
ThinkPad 上它只是一个兼容层。不要使用它，它仅作为调试工具导出。

### LED 控制

procfs: /proc/acpi/ibm/led
sysfs 属性：遵循 LED 类，名称见下文

部分 LED 指示灯可通过该功能控制。在某些较旧的 ThinkPad 型号上，还可以查询
LED 指示灯的状态。较新的 ThinkPad 无法查询 LED 指示灯的真实状态。

由于 LED 的误用可能诱导不知情的用户执行危险操作（例如在各总线仍处于活动
状态时取消扩展坞或弹出扩展坞设备），或掩盖重要警报（例如电池即将耗尽或
电池损坏），因此对大多数 LED 的访问是受限的。

对所有 LED 的无限制访问需要 thinkpad-acpi 在编译时启用
CONFIG_THINKPAD_ACPI_UNSAFE_LEDS 选项。发行版绝不可启用该选项。了解后果的
个人用户可自行启用。

音频静音与麦克风静音 LED 受支持，但目前对用户空间不可见。它们由 snd-hda-intel
音频驱动使用。

##### procfs 注意事项

```

	echo '<LED number> on' >/proc/acpi/ibm/led
	echo '<LED number> off' >/proc/acpi/ibm/led
	echo '<LED number> blink' >/proc/acpi/ibm/led

```
<LED number> 的范围是 0 到 15。可控制的 LED 集合因型号而异。以下是常见的
ThinkPad 映射：

 - 0 - 电源
 - 1 - 电池（橙色）
 - 2 - 电池（绿色）
 - 3 - UltraBase/扩展坞
 - 4 - UltraBay
 - 5 - UltraBase 电池槽
 - 6 - （未知）
 - 7 - 待机
 - 8 - 扩展坞状态 1
 - 9 - 扩展坞状态 2
 - 10, 11 - （未知）
 - 12 - thinkvantage
 - 13, 14, 15 - （未知）

以上所有 LED 都可被开启、关闭以及闪烁。

##### sysfs 注意事项

ThinkPad LED 的 sysfs 接口由 LED 类文档详细描述，位于 Documentation/leds/leds-class.rst。

LED 的命名（按 LED ID 顺序，从 0 到 12）为：
**"tpacpi**
: power"、"tpacpi:orange:batt"、"tpacpi:green:batt"、
**"tpacpi**
: dock_active"、"tpacpi::bay_active"、"tpacpi::dock_batt"、
**"tpacpi**
: unknown_led"、"tpacpi::standby"、"tpacpi::dock_status1"、
**"tpacpi**
: dock_status2"、"tpacpi::unknown_led2"、"tpacpi::unknown_led3"、
**"tpacpi**
: thinkvantage"。

由于 sysfs LED 类的限制，如果因错误而无法读取 LED 指示灯的状态，thinkpad-acpi
会将其报告为亮度零（与 LED 关闭相同）。

如果 thinkpad 固件不支持读取当前状态，尝试读取当前 LED 亮度只会返回上一次
写入该属性的亮度值。

这些 LED 可使用硬件加速进行闪烁。要请求某个 ThinkPad 指示灯 LED 以硬件加速
模式闪烁，请使用 "timer" 触发器，并将 delay_on 与 delay_off 参数设为零（以
请求硬件加速自动检测）。

已知在某个 ThinkPad 型号中不存在的 LED 不会通过 sysfs 接口提供。如果你有
扩展坞，并注意到为你的 ThinkPad 列出了并不存在的 LED（且不在扩展坞中），或
注意到有缺失的 LED，欢迎向 ibm-acpi-devel@lists.sourceforge.net 提交报告。

### ACPI 声音 -- /proc/acpi/ibm/beep

BEEP 方法由 ACPI 固件在内部使用，以在各种情况下提供声音警报。该功能允许手动
触发相同的声音。

```

	echo <number> >/proc/acpi/ibm/beep

```
有效的 <number> 范围是 0 到 17。并非所有数字都会触发声音，且声音因型号而异。
以下是 X40 上的行为：

 - 0 - 停止正在进行的声音（但停止 16 需用 17）
 - 2 - 两声蜂鸣，暂停，第三声蜂鸣（"电池电量低"）
 - 3 - 单声蜂鸣
 - 4 - 高音后接低音蜂鸣（"无法"）
 - 5 - 单声蜂鸣
 - 6 - 极高音后接高音蜂鸣（"交流/直流"）
 - 7 - 高音蜂鸣
 - 9 - 三声短蜂鸣
 - 10 - 长蜂鸣
 - 12 - 低音蜂鸣
 - 15 - 三声高音蜂鸣持续重复，以 0 停止
 - 16 - 一声中音蜂鸣持续重复，以 17 停止
 - 17 - 停止 16

### 温度传感器

procfs: /proc/acpi/ibm/thermal

sysfs 设备属性：（hwmon "thinkpad"）temp*_input

大多数 ThinkPad 包含六个或更多独立的温度传感器，但只通过标准 ACPI 方法暴露
CPU 温度。该功能显示较旧 ThinkPad 上多达八个、较新 ThinkPad 上多达十六个
不同传感器的读数。

例如在 X40 上，典型的输出可能为：

temperatures:
	42 42 45 41 36 -128 33 -128

在 T43/p 上，典型的输出可能为：

temperatures:
	48 48 36 52 38 -128 31 -128 48 52 48 -128 -128 -128 -128 -128

热传感器到物理位置的映射因系统板（system-board）型号而异（因此也因 ThinkPad
型号而异）。

https://thinkwiki.org/wiki/Thermal_Sensors 是一个公开的 wiki 页面，试图追踪
各型号的这些位置。

大多数（较新？）型号似乎遵循以下模式：

- 1：  CPU
- 2：  （取决于型号）
- 3：  （取决于型号）
- 4：  GPU
- 5：  主电池：主传感器
- 6：  扩展坞电池：主传感器
- 7：  主电池：辅助传感器
- 8：  扩展坞电池：辅助传感器
- 9-15：（取决于型号）

对于 R51（来源：Thomas Gruber）：

- 2：  Mini-PCI
- 3：  内部 HDD

对于 T43、T43/p（来源：Shmidoax/Thinkwiki.org）
https://thinkwiki.org/wiki/Thermal_Sensors#ThinkPad_T43.2C_T43p

- 2：  系统板，左侧（靠近 PCMCIA 插槽），作为 HDAPS 温度报告
- 3：  PCMCIA 插槽
- 9：  MCH（北桥）到 DRAM 总线
- 10: 时钟发生器、mini-pci 卡与 ICH（南桥），位于 Mini-PCI 卡下方、
      触摸板下方
- 11: 电源稳压器，系统板底面，F2 键下方

A31 的热传感器布局非常不典型（来源：Milos Popovic，https://thinkwiki.org/wiki/Thermal_Sensors#ThinkPad_A31）

- 1：  CPU
- 2：  主电池：主传感器
- 3：  电源转换器
- 4：  扩展坞电池：主传感器
- 5：  MCH（北桥）
- 6：  PCMCIA/环境
- 7：  主电池：辅助传感器
- 8：  扩展坞电池：辅助传感器

##### Procfs 注意事项

	不可用的传感器读数返回 -128。
	不能向该文件写入任何命令。

##### Sysfs 注意事项

	不可用的传感器返回 ENXIO 错误。该状态可能在运行时改变，因为存在
	热插拔温度传感器，例如电池与扩展坞内部的传感器。

	thinkpad-acpi 温度传感器通过 hwmon 子系统上报，并遵循 Documentation/hwmon
	下的所有 hwmon 规范。

### 实验性：嵌入式控制器寄存器转储

该功能已不再包含在 thinkpad 驱动中。
取而代之的是，可通过 /sys/kernel/debug/ec 借助一个用户空间工具访问 EC，该
工具可在此处找到：
ftp://ftp.suse.com/pub/people/trenn/sources/ec

用它来确定某些型号上保存风扇转速的寄存器。为此，请执行以下操作：

 - 确保电池已充满电
 - 确保风扇正在运行
 - 使用上述工具读取 EC

风扇与温度读数在多次读取之间常常有所不同。由于温度变化不快，你可以进行
几次快速转储以消除它们的影响。

你可以使用类似的方法弄清其他嵌入式控制器寄存器的含义——例如，确保除了
充电或放电的电池外没有任何其他变化，以确定哪些寄存器包含当前电池容量等。
如果你对此进行实验，请务必将你的结果（包含一些完整转储及其产生时条件的
描述）发送给我。

### LCD 亮度控制

procfs: /proc/acpi/ibm/brightness

sysfs 背光设备 "thinkpad_screen"

该功能允许对没有硬件亮度滑块的 ThinkPad 型号进行 LCD 亮度的软件控制。

它有一些限制：该接口不能真正打开或关闭 LCD 背光灯，它只控制背光亮度级别。

在 IBM（以及部分较早的 Lenovo）ThinkPad 上，背光控制有八个亮度级别，范围从
0 到 7。其中某些级别可能并不分明。实现 ACPI 显示背光亮度控制方法的较新
Lenovo 型号有 16 个级别，范围从 0 到 15。

对于 IBM ThinkPad，固件提供了两个用于直接亮度控制的接口：EC 与 UCMS（或
CMOS）。要选择使用哪一个，可使用 brightness_mode 模块参数：brightness_mode=1
选择 EC 模式，brightness_mode=2 选择 UCMS 模式，brightness_mode=3 选择带有
NVRAM 后备的 EC 模式（使得亮度变更在关机/重启后保留）。

驱动会尝试从每个 ThinkPad 型号的默认表中选出要使用的接口。如果它选错了，请
将其作为 bug 报告，以便我们修复。

Lenovo ThinkPad 仅支持 brightness_mode=2（UCMS）。

当标准的 ACPI 接口可提供显示背光亮度控制时，最好使用它而非这个 ThinkPad
专用的接口。如果驱动检测到 ThinkPad 上存在标准 ACPI 接口，它会禁用其原生的
背光亮度控制接口。

如果你出于某种原因想要使用 thinkpad-acpi 的背光亮度控制，而非通用 ACPI 视频
背光亮度控制，应使用 acpi_backlight=vendor 内核参数。

brightness_enable 模块参数可用于控制在可用时是否启用 LCD 亮度控制功能。
brightness_enable=0 强制将其禁用。brightness_enable=1 在可用时强制启用，即使
标准 ACPI 接口也可用。

##### Procfs 注意事项

```

	echo up   >/proc/acpi/ibm/brightness
	echo down >/proc/acpi/ibm/brightness
	echo 'level <level>' >/proc/acpi/ibm/brightness

```
##### Sysfs 注意事项

该接口通过 backlight 的 sysfs 类实现，目前文档记录很少。

在 /sys/class/backlight 下定位 thinkpad_screen 设备，其内部的属性如下：

	max_brightness:
		读取硬件可被设置为的最大亮度。
		最小值始终为零。

	actual_brightness:
		读取屏幕此瞬间被设置的亮度。

	brightness:
		写入请求驱动将亮度改变为给定值。读取将告诉你当
		"power" 被设为零、且显示未被内核电源管理事件调暗时，
		驱动正试图将显示设置到的亮度。

	power:
		电源管理模式，其中 0 为"显示开启"，1 到 3 会将显示
		背光灯调暗至亮度级别 0，因为 thinkpad-acpi 无法真正
		关闭背光灯。内核电源管理事件可临时提高当前的电源管理
		级别，即它们可以调暗显示。

警告：

    无论你做什么，都绝不要同时调用 thinkpad-acpi 的背光级别变更接口与
    基于 ACPI 的背光级别变更接口（在较新 BIOS 上可用，由 Linux ACPI 视频
    驱动驱动）。两者会以糟糕的方式相互作用，做出奇怪的事情，并可能因在
    每次变更时毫无必要地上下踢动级别而缩短背光灯的寿命。

### 音量控制（控制台音频控制）

procfs: /proc/acpi/ibm/volume

ALSA："ThinkPad Console Audio Control"，默认 ID："ThinkPadEC"

注意：默认情况下，音量控制接口以只读模式运行，因为它本应用于屏幕显示
（on-screen-display）目的。读/写模式可通过使用 "volume_control=1" 模块参数
来启用。

注意：敦促发行版不要默认启用 volume_control，这只能由本地管理员完成。
ThinkPad 的用户界面设计为由音量键单独完成控制台音频控制，桌面环境只需提供
屏幕显示反馈。软件音量控制应仅在主 AC97/HDA 混音器中完成。

##### 关于 ThinkPad 控制台音频控制

ThinkPad 有一个内置的放大器与静音电路，驱动控制台耳机与扬声器。该电路位于
音频路径中主 AC97 或 HDA 混音器之后，并完全由固件控制。

ThinkPad 有三个特殊热键用于与控制台音频控制交互：音量增、音量减与静音。

值得指出的是，静音功能正常工作的方式（在没有"静音 LED"的 ThinkPad 上）是：

1. 按静音以静音。它**总是**会静音，你可以按任意多次，声音都会保持静音。

2. 按音量键之一以取消 ThinkPad 的静音（它_不会_改变音量，只是取消静音）。

与在普通消费级笔记本上发现的廉价纯软件静音切换方案相比，这是一个优越得多的
设计：无论之前状态如何，你都可以绝对确信 ThinkPad 在按下静音按钮时不会发出
声音。

IBM ThinkPad 以及较早的 Lenovo ThinkPad 拥有驱动扬声器与耳机输出的可变增益
放大器，固件也在这些 ThinkPad 上处理耳机与扬声器的音量控制，无需操作系统的
任何帮助（该音量控制级位于音频路径中主 AC97 或 HDA 混音器之后）。

较新的 Lenovo 型号只有固件静音控制，并依赖主 HDA 混音器来完成音量控制（由
操作系统完成）。在这种情况下，音量键被过滤掉用于取消静音键按下（此区域存在
一些固件 bug），并作为普通按键按下传递给操作系统（thinkpad-acpi 不参与）。

##### ThinkPad-ACPI 音量控制

与控制台音频控制交互的首选方式是 ALSA 接口。

遗留的 procfs 接口允许读取当前状态，
```

	echo up   >/proc/acpi/ibm/volume
	echo down >/proc/acpi/ibm/volume
	echo mute >/proc/acpi/ibm/volume
	echo unmute >/proc/acpi/ibm/volume
	echo 'level <level>' >/proc/acpi/ibm/volume

```
<level> 数字范围是 0 到 14，尽管并非所有级别都分明。要在静音命令后取消静音，
使用 up 或 down 命令（level 命令不会取消静音），或使用 unmute 命令。

你可以使用 volume_capabilities 参数告诉驱动你的 thinkpad 具有音量控制还是
仅静音控制：volume_capabilities=1 对应带有静音与音量控制的混音器，
volume_capabilities=2 对应仅有静音控制的混音器。

如果驱动错误检测了你的 ThinkPad 型号的能力，请向
ibm-acpi-devel@lists.sourceforge.net 报告，以便我们更新驱动。

音量控制有两种策略。要选择使用哪一种，可使用 volume_mode 模块参数：
volume_mode=1 选择 EC 模式，volume_mode=3 选择带有 NVRAM 后备的 EC 模式（使得
音量/静音变更在关机/重启后保留）。

驱动默认以 volume_mode=3 运行。如果它在你的 ThinkPad 型号上工作不佳，请向
ibm-acpi-devel@lists.sourceforge.net 报告。

驱动支持标准的 ALSA 模块参数。如果 ALSA 混音器被禁用，驱动将禁用所有音量
功能。

### 风扇控制与监控：风扇转速、风扇启用/禁用

procfs: /proc/acpi/ibm/fan

sysfs 设备属性：（hwmon "thinkpad"）fan1_input、pwm1、pwm1_enable、fan2_input

sysfs hwmon 驱动属性：fan_watchdog

注意 注意 注意：
   出于安全原因，风扇控制操作默认是禁用的。要启用它们，必须向 thinkpad-acpi
   提供模块参数 "fan_control=1"。

该功能试图显示当前风扇转速、控制模式以及其他可能可用的风扇数据。转速直接从
嵌入式控制器的硬件寄存器读取。已知在较新的 R、T、X 与 Z 系列 ThinkPad 上
可用，但在其他型号上可能显示虚假值。

部分 Lenovo ThinkPad 支持辅助风扇。该风扇不能被单独控制，它与主风扇控制共享。

##### 风扇级别

大多数 ThinkPad 风扇在固件接口上以"级别"工作。级别 0 停止风扇。级别越高，
风扇转速越高，尽管相邻的级别常常映射到相同的风扇转速。7 是最高级别，此时
风扇达到最大推荐转速。

"auto"级别意味着 EC 根据某种内部算法改变风扇级别，通常基于温度传感器的读数。

还有一个"full-speed"（全速）级别，也称为"disengaged"（脱离）级别。在该级别，
EC 禁用速度锁定的闭环风扇控制，并以尽可能快的速度驱动风扇，这可能超出硬件
限制，因此请谨慎使用该级别。

风扇通常从一种转速缓慢地升或降到另一种转速，而 EC 需要几秒钟来响应风扇命令
是正常的。全速级别可能需要长达两分钟才能升到最大速度，并且在部分 ThinkPad
上，EC 过渡到全速级别时转速计读数会失效。

警告 警告 警告：除非你正在监控所有温度传感器读数，并准备好在必要时启用风扇
以避免过热，否则不要将风扇保持禁用状态。

处于"auto"级别的已启用风扇，如果 EC 判定 ThinkPad 足够凉爽且不需要额外的
气流，可能会停止转动。这是正常的，当各种温度读数升得过高时，EC 会再次加速
风扇。

在 X40 上，这似乎取决于 CPU 与 HDD 温度。具体而言，当 CPU 温度升至 56 度或
HDD 温度升至 46 度时风扇开启。当 CPU 温度降至 49 度且 HDD 温度降至 41 度时
风扇关闭。这些阈值当前无法控制。

ThinkPad 的 ACPI DSDT 代码会在某些条件满足时自行重新编程风扇。它会覆盖任何
通过 thinkpad-acpi 完成风扇编程。

thinkpad-acpi 内核驱动可被编程，以在用户空间未发出以下某个 procfs 风扇命令：
"enable"、"disable"、"level" 或 "watchdog"，或在可配置的、最长 120 秒的时间
内没有对 pwm1_enable（或 pwm1——**当且仅当** pwm1_enable 被设为 1，即手动模式）
的写入时，将风扇级别恢复为安全设置。该功能称为风扇安全看门狗（fan safety
watchdog）。

请注意看门狗定时器在启用风扇后停止。当收到上述某个风扇命令时，它会使用相同的
间隔自动重新武装。因此，风扇看门狗不适合用于防范通过"enable"、"disable"与
"level" procfs 风扇命令或 hwmon 风扇控制 sysfs 接口以外的方式所做的风扇模式
变更。

##### Procfs 注意事项

```

	echo enable  >/proc/acpi/ibm/fan
	echo disable >/proc/acpi/ibm/fan

```
将风扇置于级别 0 等同于禁用它。启用风扇会试图在其过慢或被禁用时将其置于安全
级别。

```

	echo 'level <level>' > /proc/acpi/ibm/fan

```
其中 <level> 是 0 到 7 之间的整数，或是 "auto" 或 "full-speed"（不带引号）之一。
并非所有 ThinkPad 都支持 "auto" 与 "full-speed" 级别。驱动接受 "disengaged" 作为
"full-speed" 的别名，并出于向后兼容性将其报告为 "disengaged"。

在 X31 与 X40（且仅在这些型号）上，风扇转速可在一定程度上被控制。一旦风扇
运行，它可被
```

	echo 'speed <speed>' > /proc/acpi/ibm/fan

```
X40 上可持续的风扇转速范围似乎约为 3700 到约 7350。超出此范围的值要么没有
任何效果，要么风扇转速最终会稳定在该范围内的某处。该命令不能停止或启动风扇。
该功能不完整，且不可通过 sysfs 接口获得。

```

	echo 'watchdog <interval in seconds>' > /proc/acpi/ibm/fan

```
如果你想禁用看门狗，请将间隔设为 0。

##### Sysfs 注意事项

sysfs 接口在大多数情况下遵循 hwmon 子系统规范，例外之处是风扇安全看门狗。

对任何 sysfs 属性的写入，如果给定 ThinkPad 不支持该操作或参数越界，可能返回
EINVAL 错误，如果操作被禁止则返回 EPERM。它们还可能返回 EINTR（被中断的系统
调用）与 EIO（尝试与固件通信时的 I/O 错误）。

驱动尚未实现的功能返回 ENOSYS。

hwmon 设备属性 pwm1_enable：
 - 0：PWM 离线（风扇被设为全速模式）
 - 1：手动 PWM 控制（使用 pwm1 设置风扇级别）
 - 2：硬件 PWM 控制（EC "auto" 模式）
 - 3：保留（软件 PWM 控制，尚未实现）

	模式 0 与 2 并非所有 ThinkPad 都支持，且驱动并非总能检测到这一点。
	如果它确实知道某个模式不受支持，会返回 -EINVAL。

hwmon 设备属性 pwm1：
	风扇级别，从固件的 0-7 值缩放到 hwmon 的 0-255 范围。0 表示风扇
	停止，255 表示最高正常转速（级别 7）。

	该属性仅在 pwm1_enable 被设为 1（手动 PWM 控制）时才能命令风扇。

hwmon 设备属性 fan1_input：
	风扇转速计读数，单位为 RPM。在某些 ThinkPad 上，当 EC 将 PWM 过渡
	到离线模式时（可能长达两分钟）可能失效。在较旧的 ThinkPad 上可能
	返回垃圾值。

hwmon 设备属性 fan2_input：
	风扇转速计读数，单位为 RPM，对应辅助风扇。仅在某些 ThinkPad 上
	可用。如果未安装辅助风扇，将始终读为 0。

hwmon 驱动属性 fan_watchdog：
	风扇安全看门狗定时器间隔，单位为秒。最小为 1 秒，最大为 120 秒。
	0 禁用看门狗。

要停止风扇：将 pwm1 设为零，并将 pwm1_enable 设为 1。

要安全地启动风扇：将 pwm1_enable 设为 2。如果以 EINVAL 失败，试着将
pwm1_enable 设为 1，并将 pwm1 设为至少 128（不过 255 会是最安全的选择）。

### WAN

procfs: /proc/acpi/ibm/wan

sysfs 设备属性：wwan_enable（已废弃）

sysfs rfkill 类：开关 "tpacpi_wwan_sw"

该功能显示内置无线 WAN 设备的存在与当前状态。

如果 ThinkPad 支持，WWAN 状态会被存储在 NVRAM 中，因此在重启与关机后都会
保留。

它已在 Lenovo ThinkPad X60 上测试。它应该也能在其它装有该模块的其他 ThinkPad
型号上工作。

##### Procfs 注意事项

```

	echo enable > /proc/acpi/ibm/wan
	echo disable > /proc/acpi/ibm/wan

```
##### Sysfs 注意事项

	如果安装了 W-WAN 卡，可通过 "wwan_enable" 这个 thinkpad-acpi 设备
	属性启用/禁用它，并也可查询其当前状态。

	enable:
  - 0：禁用 WWAN 卡 / WWAN 卡已禁用
  - 1：启用 WWAN 卡 / WWAN 卡已启用。

	注意：该接口已被通用的 rfkill 类取代。它已被废弃，并将于 2010 年
	被移除。

	rfkill 控制器开关 "tpacpi_wwan_sw"：详见
	Documentation/driver-api/rfkill.rst。

### LCD 阴影控制

procfs: /proc/acpi/ibm/lcdshadow

部分较新的 T480s 与 T490s ThinkPad 提供一项称为 PrivacyGuard 的功能。开启该功能后，
LCD 可用的垂直与水平可视角度会被限制（就如同在显示屏前手动贴上了某种隐私
保护膜）。

##### procfs 注意事项

```

	echo '0' >/proc/acpi/ibm/lcdshadow
	echo '1' >/proc/acpi/ibm/lcdshadow

```
第一条命令确保最佳可视角度，第二条命令开启该功能，限制可视角度。

### DYTC Lapmode 传感器

sysfs：dytc_lapmode

较新的 thinkpad 与移动工作站能够确定设备处于桌面模式（deskmode）还是膝上模式
（lapmode）。该功能被用户空间用于决定是否可以将 WWAN 发射功率提升到最大，并且
也有助于理解不同的散热模式，因为这些模式在桌面与膝上模式下有所不同。

该属性是只读的。如果平台不支持，则不会创建该 sysfs 类。

### 实验性：UWB

该功能被视为实验性，因为它尚未在各种 ThinkPad 型号中得到广泛的测试与验证。
该功能可能不按预期工作。谨慎使用！要使用此功能，你需要在加载模块时提供
experimental=1 参数。

sysfs rfkill 类：开关 "tpacpi_uwb_sw"

如果存在一个 UWB 设备且在 BIOS 中已启用，该功能会为其导出一个 rfkill 控制器。

##### Sysfs 注意事项

	rfkill 控制器开关 "tpacpi_uwb_sw"：详见
	Documentation/driver-api/rfkill.rst。

### 设置键盘语言

sysfs：keyboard_lang

该功能用于通过 ASL 接口将键盘语言设置到 ECFW。较少的 thinkpad 型号（如 T580、
T590、T15 Gen 1 等）具有 "="、"("、")" 数字键，当键盘语言不是"english"时，
这些键显示不正确。这是因为 ECFW 中的默认键盘语言被设为 "english"。因此使用
此 sysfs，用户可以将正确的键盘语言设置到 ECFW，之后这些键即可正常工作。

```

        echo jp > /sys/devices/platform/thinkpad_acpi/keyboard_lang

```
对应于要设置的键盘布局的文本为：be（比利时）、cz（捷克）、da（丹麦）、de（德语）、
en（英语）、es（西班牙）、et（爱沙尼亚）、fr（法语）、fr-ch（法语（瑞士））、
hu（匈牙利）、it（意大利）、jp（日本）、nl（荷兰）、nn（挪威）、pl（波兰）、
pt（葡萄牙语）、sl（斯洛文尼亚）、sv（瑞典）、tr（土耳其）

### WWAN 天线类型

sysfs：wwan_antenna_type

在某些较新的 Thinkpad 上，我们需要根据天线类型设置 SAR 值。用户空间将使用该
接口获取天线类型并设置相应的 SAR 值，这是 FCC 认证所要求的。

```

        cat /sys/devices/platform/thinkpad_acpi/wwan_antenna_type

```
当前支持以下 2 种天线类型：
- type a
- type b

该属性是只读的。如果平台不支持，则不会创建该 sysfs 类。

### doubletap_enable

sysfs：doubletap_enable

控制是否过滤掉 TrackPoint 双击（doubletap）事件。双击是一种快速双击 TrackPoint
两次以触发特殊功能键事件的功能。

```

                cat /sys/devices/platform/thinkpad_acpi/doubletap_enable
                echo 1 | sudo tee /sys/devices/platform/thinkpad_acpi/doubletap_enable
                echo 0 | sudo tee /sys/devices/platform/thinkpad_acpi/doubletap_enable

```
取值：

 - 1 - 处理双击事件（默认）
 - 0 - 过滤掉（忽略）双击事件

	该设置也可通过 Fn+doubletap 热键切换。

### Auxmac

sysfs：auxmac

某些较新的 Thinkpads 具有一项称为 MAC 地址透传（MAC Address Pass-through）的功能。
该功能由系统固件实现，用于提供一个系统唯一的 MAC，当连接到网络时可覆盖扩展坞
或 USB 以太网适配器的 MAC。该属性使用户空间能够在该功能启用时轻松确定 MAC 地址。

这些辅助 MAC 的值为：

        cat /sys/devices/platform/thinkpad_acpi/auxmac

如果功能被禁用，该值将为 'disabled'。

该属性是只读的。

### 自适应键盘（Adaptive keyboard）

sysfs 设备属性：adaptive_kbd_mode

该 sysfs 属性控制将在 Lenovo X1 Carbon 2nd gen（2014）的自适应键盘上显示的键盘
"面"。该值可被读取与设置。

- 0 = 主页模式（Home mode）
- 1 = 网页浏览器模式（Web-browser mode）
- 2 = 网络会议模式（Web-conference mode）
- 3 = 功能模式（Function mode）
- 4 = 平放模式（Layflat mode）

有关每种模式下将出现哪些按钮的更多细节，请查阅笔记本的用户指南：
https://download.lenovo.com/ibmdl/pub/pc/pccbbs/mobiles_pdf/x1carbon_2_ug_en.pdf

### 电池充电控制

sysfs 属性：
/sys/class/power_supply/BAT*/charge_control_{start,end}_threshold

这两个属性是为那些受驱动支持的电池创建的。它们使用户能够控制给定电池的充电
阈值。两个值都可被读取与设置。`charge_control_start_threshold` 接受 0 到 99
（含）之间的整数；该值代表一个电池百分比级别，低于该级别时充电将开始。
`charge_control_end_threshold` 接受 1 到 100（含）之间的整数；该值代表一个电池
百分比级别，高于该级别时充电将停止。

这些属性的确切语义可在 Documentation/ABI/testing/sysfs-class-power 中找到。

### 硬件损坏检测能力

sysfs 属性：hwdd_status、hwdd_detail

Thinkpad 正在增加检测与报告硬件损坏的能力。新增 sysfs 接口以识别受损设备的
状态。初步支持适用于 USB-C 可更换连接器。

```

        cat /sys/devices/platform/thinkpad_acpi/hwdd_status

```
该值显示设备受损的状态。

- 0 = 未损坏
- 1 = 已损坏

```

        cat /sys/devices/platform/thinkpad_acpi/hwdd_detail

```
该值显示受损设备的位置，每个受损"项"占一行。例如：

如果未检测到损坏：

- No damage detected（未检测到损坏）

如果检测到损坏：

- TYPE-C: Base, Right side, Center port（TYPE-C：底座、右侧、中间端口）

该属性是只读的。如果不支持该功能，则不会创建 sysfs 属性。

### 多条命令、模块参数

可将多条命令一次性写入 proc 文件，方法是
```

	echo enable,0xffff > /proc/acpi/ibm/hotkey
	echo lcd_disable,crt_enable > /proc/acpi/ibm/video

```
命令也可在加载 thinkpad-acpi 模块时指定，
```

	modprobe thinkpad_acpi hotkey=enable,0xffff video=auto_disable


```
### 启用调试输出

模块接受一个 debug 参数，可用于有选择地
```

	 modprobe thinkpad_acpi debug=0xffff

```
这将启用所有调试输出类别。它接受一个位掩码，因此要启用多个输出类别，只需将
它们的值相加。

	=============		======================================
	Debug 位掩码		描述
	=============		======================================
	0x8000			公开访问驱动某些功能的用户空间程序 PID
	0x0001			初始化与探测
	0x0002			移除
	0x0004			射频发射器控制（RFKILL）
				（蓝牙、WWAN、UWB……）
	0x0008			HKEY 事件接口、热键
	0x0010			风扇控制
	0x0020			背光亮度
	0x0040			音频混音器/音量控制
	=============		======================================

还有一个内核编译选项可启用更多调试信息，这在调试驱动问题时可能是必需的。

驱动输出的调试信息级别可在运行时通过 sysfs 改变，使用驱动属性 debug_level。
该属性接受与上面 debug 模块参数相同的位掩码。

### 强制加载模块

如果 thinkpad-acpi 拒绝检测你的 ThinkPad，你可以尝试指定模块参数 force_load=1。
无论是否有效，都请向 ibm-acpi-devel@lists.sourceforge.net 提交报告。

##### sysfs 接口变更日志

=========	===============================================================
0x000100:	初始的 sysfs 支持，作为单个平台驱动与设备。
0x000200:	32 个热键的热键支持，以及无线滑块开关支持。
0x010000:	热键现在默认通过输入层处理，无线开关生成输入事件
		EV_RADIO，且驱动默认在固件中启用热键处理。
0x020000:	ABI 修复：新增了独立的 hwmon 平台设备与驱动，必须按名称
		（thinkpad）以及 hwmon 类定位，以兼容 libsensors4
		（lm-sensors 3）。将所有 hwmon 属性移动到此新平台设备。
0x020100:	带有热键 NVRAM 轮询支持的 thinkpad-acpi 的标记。如果必须，
		用它来得知你不应启动用户空间的 NVRAM 轮询器（可用于检测
		NVRAM 何时被用户因为本就不需要/不想要而编译掉）。
0x020101:	带有热键 NVRAM 轮询以及正确 hotkey_mask 语义的 thinkpad-acpi
		的标记（NVRAM 轮询补丁的第 8 版）。0.18 的某些开发快照有
		一个做了奇怪事情的早期版本，影响了 hotkey_mask。
0x020200:	为以下属性添加 poll()/select() 支持：
		hotkey_radio_sw、wakeup_hotunplug_complete、wakeup_reason
0x020300:	热键启用/禁用支持被移除，属性 hotkey_bios_enabled 与
		hotkey_enable 被废弃并标记为待移除。
0x020400:	支持 16 个 LED 的标记。此外，已知在给定型号中不存在的 LED
		不再向 LED sysfs 类注册。
0x020500:	更新后的热键驱动，hotkey_mask 始终可用且始终能够禁用热键。
		非常老的 thinkpad 得到恰当支持。hotkey_bios_mask 被废弃并
		标记为待移除。
0x020600:	支持背光变更事件的标记。
0x020700:	支持仅静音的混音器。音量控制默认处于只读模式。支持 ALSA
		混音器的标记。
0x030000:	散热与风扇的 sysfs 属性被移动到 hwmon 设备，而非附加在
		背后的平台设备上。
=========	===============================================================
