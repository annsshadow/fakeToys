## Intel INT3496 ACPI 设备 extcon 驱动文档


Intel INT3496 ACPI 设备 extcon 驱动是一个用acpi-id INT3496 ACPI 设备的驱动，例如可在 Intel Baytrail Cherrytrail 平板电脑上找到

ACPI 设备描述了操作系统如何读取设USB-otg 端口id 引脚，以及它如何选择性地otg 端口上启Vbus 输出，如何选择性地控制数据引脚USB 主机USB 外设控制器之间的多路复用

ACPI 设备通过从其 ACPI _CRS（Current Resource Settings，当前资源设置）调用返回最3 gpio 描述符数组来暴露此功能：

=======  =====================================================================
Index 0  id 引脚的输gpio，始终存在且有效
Index 1  用于从设备向 otg 端口启用 Vbus 输出的输gpio，写1 以启Vbus 输出（该 gpio 描述符可能不存在或无效）
Index 2  用于USB 主机USB 外设控制器之间多路复用数据引脚的输出 gpio，写1 以多路复用到外设控制
=======  =====================================================================

索引GPIO 连接 ID 之间的映射如

	======= =======
	id	index 0
	vbus	index 1
	mux	index 2
	======= =======
