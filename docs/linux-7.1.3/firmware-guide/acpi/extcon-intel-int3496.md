## Intel INT3496 ACPI 设备 extcon 驱动文档


Intel INT3496 ACPI 设备 extcon 驱动是一个用于 acpi-id 为 INT3496 的 ACPI 设备的驱动，例如可在 Intel Baytrail 和 Cherrytrail 平板电脑上找到。

该 ACPI 设备描述了操作系统如何读取设备 USB-otg 端口的 id 引脚，以及它如何选择性地在 otg 端口上启用 Vbus 输出，如何选择性地控制数据引脚在 USB 主机和 USB 外设控制器之间的多路复用。

ACPI 设备通过从其 ACPI _CRS（Current Resource Settings，当前资源设置）调用返回最多 3 个 gpio 描述符数组来暴露此功能：

=======  =====================================================================
Index 0  id 引脚的输入 gpio，始终存在且有效
Index 1  用于从设备向 otg 端口启用 Vbus 输出的输出 gpio，写入 1 以启用 Vbus 输出（该 gpio 描述符可能不存在或无效）
Index 2  用于在 USB 主机和 USB 外设控制器之间多路复用数据引脚的输出 gpio，写入 1 以多路复用到外设控制器
=======  =====================================================================

索引与 GPIO 连接 ID 之间的映射如下

	======= =======
	id	index 0
	vbus	index 1
	mux	index 2
	======= =======
