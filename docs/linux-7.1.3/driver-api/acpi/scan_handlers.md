## ACPI 扫描处理程序


:Copyright: |copy| 2012, Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>

在系统初始化以及基于 ACPI 的设备热添加过程中，会扫ACPI 命名空间以寻找通常表示
各种硬件的设备对象。这将导致为 ACPI 命名空间中的每个设备对象创建一struct
acpi_device 对象并向驱动核心注册，这struct acpi_device 对象的层级结构反映了
命名空间的布局（即命名空间中的父设备对象由struct acpi_device 对象表示，其子对同理）。下文将这些 struct acpi_device 对象称为“设备节点”，但不应将它们与设备树解析
代码所使用struct device_node 对象混淆（尽管它们的作用与那些对象类似）
在基ACPI 的设备热移除过程中，表示被移除硬件的设备节点会被注销并删除
drivers/acpi/scan.c 中的 ACPI 命名空间核心扫描代码会对设备节点执行基本初始化，例如
从它们所表示的设备对象中检索通用配置信息，并用适当的数据填充它们，但其中一些在注册还需要额外处理。例如，如果给定的设备节点表示一PCI 主桥，其注册应当导致该桥下的 PCI
总线被枚举，并且该总线上的 PCI 设备向驱动核心注册。类似地，如果设备节点表示一PCI
中断链路，则需要配置该链路以便内核可以使用它
这些额外的配置任务通常取决于给定设备节点所表示硬件组件的类型，该类型可依据设备节点硬件 ID（HID）确定。它们由以下对象执行

```

	struct acpi_scan_handler {
		const struct acpi_device_id *ids;
		struct list_head list_node;
		int (*attach)(struct acpi_device *dev, const struct acpi_device_id *id);
		void (*detach)(struct acpi_device *dev);
	};

```

其中，ids 是该处理程序应当负责的那些设备节点的 ID 列表；list_node 是挂ACPI 核心
维护ACPI 扫描处理程序全局链表的钩子；.attach() .detach() 回调分别在注册新
设备节点之后、以及在该处理程序此前附加的设备节点注销之前执行
命名空间扫描函数 acpi_bus_scan() 首先将该给定命名空间范围内的所有设备节点向驱动核心
注册。然后，它尝试用可用扫描处理程序ids 数组，对每个设备节点匹配一个扫描处理程序如果找到匹配的扫描处理程序，就对该设备节点执行其 .attach() 回调。若该回调返1，意味着
该处理程序已认领此设备节点，并负责执行与之相关的任何额外配置任务。在这种情况下，它也负责为该设备节点的注销做准备。随后，设备节点handler 字段会被填入认领它的扫描处理程序
的地址
如果 .attach() 回调返回 0，表示该设备节点对给定的扫描处理程序而言不相关，可继续与链表中的
下一个扫描处理程序匹配。如果它返回一个（负数）错误码，则表示由于严重错误，命名空间扫描应终止。返回的错误码应反映错误的类型
命名空间裁剪函数 acpi_bus_trim() 首先执行该给定命名空间范围内所有设备节点（若它们有扫描
处理程序）的扫描处理程序.detach() 回调。然后，它注销该范围内的所有设备节点
可以借助 acpi_scan_add_handler() 函数ACPI 扫描处理程序添加ACPI 核心维护的链表中该函数以指向新扫描处理程序的指针作为参数。扫描处理程序被加入链表的顺序，就是命名空间扫描
期间它们与设备节点匹配的顺序
所有扫描处理程序都必须acpi_bus_scan() 首次运行之前加入链表，且之后不能被移除