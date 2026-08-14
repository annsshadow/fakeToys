## Linux 即插即用（Plug and Play）文档


:Author: Adam Belay <ambx1@neo.rr.com>
:Last updated: Oct. 16, 2002


### 概述


即插即用（Plug and Play）提供了一种检测并为传统设备或其他不可配置设备
设置资源的手段。Linux 即插即用层向兼容的驱动提供这些服务。


### 用户界面


Linux 即插即用的用户界面为那些不支持 Linux 即插即用的传统驱动与用户态驱动
提供了一种激活 PnP 设备的手段。该用户界面集成在 sysfs 中。

除了标准的 sysfs 文件外，还会在每个设备的目录下创建以下文件：
- id —— 显示所支持的 EISA ID 列表
- options —— 显示可能的资源配置
- resources —— 显示当前已分配的资源，并允许更改资源

##### 激活设备

```

	# echo "auto" > resources

```
这将调用自动资源配置系统来激活该设备

##### 手动激活设备

```

	# echo "manual <depnum> <mode>" > resources

	<depnum> - the configuration number
	<mode> - static or dynamic
		 static = for next boot
		 dynamic = now

```
##### 禁用设备

```

	# echo "disable" > resources


```
示例：

假设你需要激活软盘控制器。

1. 切换到正确的目录，在我这里该目录为

```

	# cd /driver/bus/pnp/devices/00:0f
	# cat name
	PC standard floppy disk controller

```
```

	# cat resources
	DISABLED

  - Notice the string "DISABLED".  This means the device is not active.

```
```

	# cat options
	Dependent: 01 - Priority acceptable
	    port 0x3f0-0x3f0, align 0x7, size 0x6, 16-bit address decoding
	    port 0x3f7-0x3f7, align 0x0, size 0x1, 16-bit address decoding
	    irq 6
	    dma 2 8-bit compatible
	Dependent: 02 - Priority acceptable
	    port 0x370-0x370, align 0x7, size 0x6, 16-bit address decoding
	    port 0x377-0x377, align 0x0, size 0x1, 16-bit address decoding
	    irq 6
	    dma 2 8-bit compatible

```
```

	# echo "auto" > resources

```
```

	# cat resources
	io 0x3f0-0x3f5
	io 0x3f7-0x3f7
	irq 6
	dma 2

```
```

	pnp_reserve_irq=irq1[,irq2] ....
	pnp_reserve_dma=dma1[,dma2] ....
	pnp_reserve_io=io1,size1[,io2,size2] ....
	pnp_reserve_mem=mem1,size1[,mem2,size2] ....



```
### 统一的即插即用层


所有即插即用驱动、协议与服务都在一个称为“即插即用层”的中心位置汇合。该层
负责在 PnP 驱动与 PnP 协议之间交换信息，因此会自动将命令转发给相应的协议。
这使得编写 PnP 驱动变得容易得多。

即插即用层提供以下函数：

pnp_get_protocol
  将使用计数加一

pnp_put_protocol
  将使用计数减一

pnp_register_protocol
  用于注册一个新的 PnP 协议

pnp_register_driver
  将一个 PnP 驱动添加到即插即用层

  其中包含驱动模型的集成
  成功时返回 0，失败时返回负的错误号；若你想了解有多少个设备绑定到该驱动，可统计对 .add() 方法的调用次数

pnp_unregister_driver
  从即插即用层中移除一个 PnP 驱动



### 即插即用协议


本节面向 PnP 协议开发者提供相关信息。

当前计算世界中可用的协议如下：

- PNPBIOS:
    用于串口、并口等系统设备。
- ISAPNP:
    为 ISA 总线提供 PnP 支持
- ACPI:
    在其众多用途中，ACPI 提供关于系统级设备的信息。

它旨在取代 PNPBIOS。Linux 即插即用目前尚未支持它，但计划在不久的将来实现。


Linux PnP 协议的要求：
1. 协议必须使用 EISA ID
2. 协议必须向 PnP 层报告设备当前的配置

- 设置资源的能力是可选的，但推荐使用。

以下是与 PnP 协议相关的函数：

pnp_add_device
  使用此函数将一个 PnP 设备添加到 PnP 层

  仅当 pnp_dev 结构中的所有期望字段都已设置时才调用此函数

pnp_init_device
  调用它来初始化 PnP 结构

pnp_remove_device
  调用它从即插即用层移除设备。
  若设备仍在使用则会失败。
  会自动释放设备及相关结构所占用的内存

pnp_add_id
  将一个 EISA ID 添加到指定设备所支持的 ID 列表中

更多信息请参考某个协议的源码，例如
/drivers/pnp/pnpbios/core.c。



### Linux 即插即用驱动


本节面向 Linux PnP 驱动开发者提供相关信息。

##### 新方式


1. 首先列出所支持的 EISA ID

```

	static const struct pnp_id pnp_dev_table[] = {
		/* Standard LPT Printer Port */
		{.id = "PNP0400", .driver_data = 0},
		/* ECP Printer Port */
		{.id = "PNP0401", .driver_data = 0},
		{.id = ""}
	};

   Please note that the character 'X' can be used as a wild card in the function
   portion (last four characters).

   ex::

	/* Unknown PnP modems */
	{	"PNPCXXX",		UNKNOWN_DEV	},

   Supported PnP card IDs can optionally be defined.
   ex::

	static const struct pnp_id pnp_card_table[] = {
		{	"ANYDEVS",		0	},
		{	"",			0	}
	};

```
2. 可选地定义 probe 与 remove 函数。如果驱动已经拥有可靠的资源检测方法（例如 parport_pc 驱动），
   不定义这些函数是合理的。

```

	static int
	serial_pnp_probe(struct pnp_dev * dev, const struct pnp_id *card_id, const
			struct pnp_id *dev_id)
	{
	. . .

   ex::

	static void serial_pnp_remove(struct pnp_dev * dev)
	{
	. . .

   consult /drivers/serial/8250_pnp.c for more information.

```
3. 创建驱动结构

```

	static struct pnp_driver serial_pnp_driver = {
		.name		= "serial",
		.card_id_table	= pnp_card_table,
		.id_table	= pnp_dev_table,
		.probe		= serial_pnp_probe,
		.remove		= serial_pnp_remove,
	};

   * name and id_table cannot be NULL.

```
4. 注册驱动

```

	static int __init serial8250_pnp_init(void)
	{
		return pnp_register_driver(&serial_pnp_driver);
	}

```
##### 旧方式


创建了一系列兼容函数，以便于转换 ISAPNP 驱动。它们只应作为临时方案使用。

```

	struct pnp_dev *pnp_find_dev(struct pnp_card *card,
				     unsigned short vendor,
				     unsigned short function,
				     struct pnp_dev *from)


```