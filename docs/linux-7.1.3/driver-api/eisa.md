## EISA 总线支持


:Author: Marc Zyngier <maz@wild-wind.fr.eu.org>

本文档汇集了关于EISA 驱动移植到新EISA/sysfs API 的一些零散笔记
2.5.59 版本开始，EISA 总线几乎获得了与 PCI USB 等其它更主流总线相同地位。这通过 sysfs 得以实现，sysfs 定义了一套足够完善的抽象来管理总线、设备和
驱动
尽管API 用起来相当简单，但将现有驱动转换到新基础设施并非易事（主要是因为
探测代码通常也用于探ISA 卡）。此外，大多EISA 驱动都是最老的一Linux
驱动，所以可想而知，这些年里这里积了不少灰尘
EISA 基础设施由三部分组成
    - 总线代码实现了大部分通用代码。它在运EISA 代码的所有架构之间共享。它
      实现总线探测（检测总线上可用的 EISA 卡）、分I/O 资源、通过 sysfs 实现
      花哨的命名，并为驱动提供注册接口
    - 总线根驱动实现了总线硬件与通用总线代码之间的粘合。它负责发现实现该总线      设备，并将其设置好以便稍后由总线代码探测。这可以是像x86 上保留一I/O
      区域这样简单的事情，也可以是像 hppa EISA 代码那样相当复杂的事情。这      为了EISA 在“新”平台上运行而需要实现的部分
    - 驱动向总线提供它所管理设备的一个列表，并实现必要的回调，以便在被告知时
      探测和释放设备
下面每个函数/结构体都位于 <linux/eisa.h> 中，该文件严重依<linux/device.h>
## 鎬荤嚎鏍归┍鍔。

```

	int eisa_root_register (struct eisa_root_device *root);

```
eisa_root_register 函数用于将一个设备声明为 EISA 总线的根。eisa_root_device
结构体持有一个引```

	struct eisa_root_device {
		struct device   *dev;	 /* Pointer to bridge device */
		struct resource *res;
		unsigned long    bus_base_addr;
		int		 slots;  /* Max slot number */
		int		 force_probe; /* Probe even when no slot 0 */
		u64		 dma_mask; /* from bridge device */
		int              bus_nr; /* Set by eisa_root_register */
		struct resource  eisa_root_res;	/* ditto */
	};

```
============= ======================================================
node          用于 eisa_root_register 的内部用dev           指向根设备的指针
res           根设I/O 资源
bus_base_addr 此总线slot 0 的地址
slots	     最大探slot force_probe   即使 slot 0 为空（无 EISA 主板）也进行探测
dma_mask      默认 DMA 掩码。通常为桥设备dma_maskbus_nr	     唯一总线 id，由 eisa_root_register 设置
============= ======================================================

## 驱动


```

	int eisa_driver_register (struct eisa_driver *edrv);
	void eisa_driver_unregister (struct eisa_driver *edrv);

```
够清楚了吗？

```

	struct eisa_device_id {
		char sig[EISA_SIG_LEN];
		unsigned long driver_data;
	};

	struct eisa_driver {
		const struct eisa_device_id *id_table;
		struct device_driver         driver;
	};

```
=============== ====================================================
id_table	一个以 NULL 结尾EISA id 字符串数组，
		后跟一个空字符串。每个字符串可选择性地
		与一个驱动相关的值（driver_data）配对
driver		一个通用驱动，如
		Documentation/driver-api/driver-model/driver.rst
		所述。只.nameprobe .remove 成员是必填的=============== ====================================================

```

	static struct eisa_device_id vortex_eisa_ids[] = {
		{ "TCM5920", EISA_3C592_OFFSET },
		{ "TCM5970", EISA_3C597_OFFSET },
		{ "" }
	};

	static struct eisa_driver vortex_eisa_driver = {
		.id_table = vortex_eisa_ids,
		.driver   = {
			.name    = "3c59x",
			.probe   = vortex_eisa_probe,
			.remove  = vortex_eisa_remove
		}
	};

```
## 设备


sysfs 框架在设备被发现和移除时调用 .probe .remove 函数（注意，.remove 函数
仅在驱动作为模块构建时才会被调用）
这两个函数都传入一个指'struct device' 的指针，该结构体```

	struct eisa_device {
		struct eisa_device_id id;
		int                   slot;
		int                   state;
		unsigned long         base_addr;
		struct resource       res[EISA_MAX_RESOURCES];
		u64                   dma_mask;
		struct device         dev; /* generic device */
	};

```
======== ============================================================
id	 EISA id，从设备读取。id.driver_data 从匹配的驱动 EISA id 设置slot	 检测到该设备的 slot state    一组指示设备状态的标志。当前的标志EISA_CONFIG_ENABLED 	 EISA_CONFIG_FORCEDres	 分配给该设备的四256 字节 I/O 区域
dma_mask 从父设备设置DMA 掩码
dev	 通用设备（参Documentation/driver-api/driver-model/device.rst======== ============================================================

你可以使'to_eisa_device' 宏从 'struct device' 获取 'struct eisa_device'
## 杂项


```

	void eisa_set_drvdata (struct eisa_device *edev, void *data);

```
将数据存储到设备driver_data 区域
```

	void *eisa_get_drvdata (struct eisa_device *edev):

```
获取先前存储到设driver_data 区域的指针
```

	int eisa_get_region_index (void *addr);

```
返回给定地址的区域号 <= x < EISA_MAX_RESOURCES）
## 内核参数


eisa_bus.enable_dev
	一个以逗号分隔的、要被启用的 slot 列表，即使固件将该卡设为禁用。驱动必	能够在这样的条件下正确地初始化设备
eisa_bus.disable_dev
	一个以逗号分隔的、要被禁用的 slot 列表，即使固件将该卡设为启用。驱动将
	不会被调用来处理此设备
virtual_root.force_probe
	强制探测代码去探EISA slot，即使它找不到符EISA 的主板（slot 0 	什么也没有出现）。默认为 0（不强制），当设置了 CONFIG_EISA_VLB_PRIMING
	时设1（强制探测）
## 零散笔记


EISA 驱动转换到新 API 主要涉及**删除**代码（因为探测现在位于核EISA 代码
中）。遗憾的是，大多数驱动在 ISA EISA 之间共享它们的探测例程。在剥离 EISA
代码时必须特别小心，以免其它总线遭受这些“外科手术式打击”的影响…
*绝不*期望eisa_driver_register 返回时能检测到任何 EISA 设备，因为总线
很可能尚未被探测。事实上，大多数时候正是如此（总线根驱动通常在启动过程中相当
晚的时候才介入）。遗憾的是，大多数驱动都自行进行探测，并期望在退出其探测例程已经探索了整台机器
例如，将你喜欢的 EISA SCSI 卡切换到“热插拔”模型是“正确之举tm)
## 致谢


我要感谢以下人士的帮助：

- Xavier Benigni，借给我一台绝妙的 Alpha Jensen- James Bottomley、Jeff Garzik，将这部分代码合入内核，
- Andries Brouwer，贡献了大量 EISA id- Catrin Jones，在家里应付了太多的机器