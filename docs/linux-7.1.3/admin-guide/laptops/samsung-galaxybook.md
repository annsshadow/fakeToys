

## Samsung Galaxy Book 驱动


Joshua Grisham <josh@joshuagrisham.com>

这是一个用于三星 Galaxy Book 系列笔记本设备的 Linux x86 平台驱动，它利用三星的 `SCAI` ACPI 设备来控制额外功能并接收各种通知。

## 支持的设备


任何带有受支持 ACPI 设备 ID 之一的设备都应受支持。这涵盖了截至本文撰写时大多数在售的“Samsung Galaxy Book”系列笔记本，也可能包括其他三星笔记本设备。

## 状态


目前支持以下功能：

- 键盘背光灯 <keyboard-backlight> 控制
- 性能模式 <performance-mode> 控制，使用平台 profile 接口实现
- :ref:`电池充电控制终止阈值 <battery-charge-control-end-threshold>`（在给定百分比处停止为电池充电），作为电池钩子实现
- 固件属性 <firmware-attributes>，用于控制各种设备设置
- 处理各种操作的 Fn 热键 <keyboard-hotkey-actions>
- :ref:`处理 ACPI 通知与热键 <acpi-notifications-and-hotkey-actions>`

由于这些设备的不同型号在功能上可能存在差异，驱动内部构建了相应的逻辑，会在启用某项功能的支持（注册额外的设备或扩展、添加 sysfs 属性等）之前，尝试测试每个已实现的功能是否能返回有效响应。因此，需要注意：你的特定设备可能并不支持全部功能。

以下功能有可能实现，但需要进一步调查，因此目前尚不支持：

- 扬声器的“Dolby Atmos”模式
- 在带有 `SAM0427` 的型号上提高屏幕亮度的“Outdoor Mode”
- 在带有 `SAM0427` 的型号上的“Silent Mode”


## 键盘背光灯


**会创建一个名为 ``samsung-galaxybook : kbd_backlight`` 的新 LED 类**，随后会通过位于 **``/sys/class/leds/samsung-galaxybook : kbd_backlight`` 的标准基于 sysfs 的 LED 接口来暴露该设备。可以通过向 `brightness` sysfs 属性写入所需值，或使用任何其他期望的用户空间工具来控制亮度。

  这些设备大多带有环境光传感器，在光照充足的环境下也会关闭键盘背光灯。这种行为目前似乎无法控制，但最好有所了解。


## 性能模式


该驱动实现了
Documentation/userspace-api/sysfs-platform_profile.rst 接口，用于配合三星 ACPI 设备的“性能模式”功能工作。

由于并非所有型号都支持全部相同的性能模式，每个三星“性能模式”到其对应平台 profile 的映射由驱动动态执行。你的设备可能具有下列映射中的一个或多个：

- “Silent” 映射到 `low-power`
- “Quiet” 映射到 `quiet`
- “Optimized” 映射到 `balanced`
- “High performance” 映射到 `performance`

映射的结果会在模块加载时打印到内核日志中。受支持的 profile 也可以从
`/sys/firmware/acpi/platform_profile_choices` 获取，而
`/sys/firmware/acpi/platform_profile` 可用于读取或写入当前所选的 profile。

如果之前没有设置过 profile，则会在模块加载时将 `balanced` 平台 profile 设为当前值。


## 电池充电控制终止阈值


该平台驱动会添加设置电池充电控制终止阈值的能力，但无法设置起始阈值。

这一功能在 Windows 下通常被三星各类应用程序称为“Battery Saver”，但在 Linux 中，我们在电池设备上实现了标准化的“charge control threshold”（充电控制阈值）sysfs 接口，以便从用户空间控制该功能。

sysfs 属性
`/sys/class/power_supply/BAT1/charge_control_end_threshold` 可用于读取或设置期望的充电终止阈值。

如果你希望与 Windows 下的 Samsung Settings 应用保持互操作性，则应将值设为 100 表示“关闭”，或仅使用下列值之一启用该功能：50、60、70、80 或 90。否则，驱动会接受 1 到 100 之间的任意值作为你希望电池停止充电的百分比。

  有观测表明，当输入值小于 30 时，某些设备会自动“关闭”充电控制终止阈值。


## 固件属性


以下枚举类型的固件属性由该驱动设置，如果你的设备支持，应可在
`/sys/class/firmware-attributes/samsung-galaxybook/attributes/` 下访问：

- `power_on_lid_open`（打开盖子时设备应通电）
- `usb_charging`（即使在设备关机或处于低功耗睡眠状态时，USB 端口也能为所连设备供电）
- `block_recording`（阻止对摄像头与麦克风的访问）

所有这些属性都是简单的类布尔枚举值，用 0 表示“关闭”、1 表示“开启”。使用 `current_value` 属性来获取或更改设备上的设置。

注意，当 `block_recording` 更新时，输入设备“Samsung Galaxy Book Lens Cover”会收到一个 `SW_CAMERA_LENS_COVER` 开关事件，反映当前状态。


## 键盘热键操作（i8042 过滤器）


i8042 过滤器会吞掉 Fn+F9 热键（多级键盘背光灯切换）和 Fn+F10 热键（阻止录制切换）的键盘事件，转而在驱动自身内部执行对应的操作。

Fn+F9 会循环切换键盘背光灯的亮度级别。会使用 `led_classdev_notify_brightness_hw_changed` 发送通知，以便用户空间知晓这一变化。这模拟了其他现有设备的行为：亮度级别由嵌入式控制器在内部循环，然后通过通知上报。

Fn+F10 会切换“block recording”设置的值，从而阻止或允许使用内置摄像头与麦克风（并生成上述相同的 Lens Cover 开关事件）。


## ACPI 通知与热键操作


ACPI 通知会在设备类 `samsung-galaxybook` 以及与你设备上找到的三星 ACPI 设备 ID 相匹配的总线 ID 下生成 ACPI netlink 事件。可以使用 `acpi_listen`、`acpid` 等用户空间工具接收这些事件。

Fn+F11 性能模式热键将由驱动处理；每次按键会循环切换到下一个可用的平台 profile。
