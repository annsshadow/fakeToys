## MEN Chameleon 总线


   =================
   1 简介
       1.1 本文档的范围
       1.2 当前实现的限制
   2 架构
       2.1 MEN Chameleon 总线
       2.2 载板设备
       2.3 解析器
   3 资源处理
       3.1 内存资源
       3.2 中断
   4 编写 MCB 驱动
       4.1 驱动结构
       4.2 探测与挂载
       4.3 初始化驱动
       4.4 使用 DMA

## 简介


本文档描述 MEN Chameleon 总线（本文档中简称 MCB）的架构与实现。

### 本文档的范围


本文档旨在简要概述当前实现，绝不描述基于 MCB 的设备的完整可能性。

### 当前实现的限制


当前实现仅限于基于 PCI 与 PCIe 的载板设备，这些设备只使用单个内存资源并共享 PCI 传统 IRQ。未实现的有：

- 多资源 MCB 设备，如 VME 控制器或 M-Module 载板。
- 需要另一个 MCB 设备的 MCB 设备，例如 DMA 控制器的缓冲描述符 SRAM，或视频控制器的视频内存。
- 为每个 MCB 设备提供一个（或多个）IRQ 的载板设备（如支持 MSI 或 MSI-X 的基于 PCIe 的载板）的每载板 IRQ 域。

## 架构


MCB 分为 3 个功能块：

- MEN Chameleon 总线本身、
- MCB 载板设备的驱动，以及
- Chameleon 表的解析器。

### MEN Chameleon 总线


MEN Chameleon 总线是一个人工总线系统，它挂载到由 MEN Mikro Elektronik GmbH 生产的某些硬件上发现的所谓 Chameleon FPGA 设备。这些设备是在单个 FPGA 中实现的多功能设备，通常通过某种 PCI 或 PCIe 链路挂载。每个 FPGA 包含一个描述 FPGA 内容的头部段。该头部列出设备 id、PCI BAR、距 PCI BAR 起始的偏移、在 FPGA 中的大小、中断号，以及一些当前 MCB 实现尚未处理的其他属性。

### 载板设备


载板设备只是 Chameleon FPGA 所挂载的真实物理总线的一个抽象。某些 IP 核驱动可能需要与载板设备的属性交互（例如查询 PCI 设备的中断号）。为了提供与真实硬件总线的抽象，MCB 载板设备提供回调方法，将驱动的 MCB 函数调用转换为与硬件相关的函数调用。例如，载板设备可以实现 get_irq() 方法，该方法可以被转换为对硬件总线的查询，以获取设备应使用的 IRQ 号。

### 解析器


解析器读取 Chameleon 设备的前 512 字节并解析 Chameleon 表。目前解析器只支持 Chameleon 表的 v2 变体，但可以容易地改造以支持更旧或未来可能的变体。在解析表的条目时，会分配新的 MCB 设备，并根据 Chameleon 表中的资源分配来分配它们的资源。资源分配完成后，MCB 设备被注册到 MCB，进而注册到 Linux 内核的驱动核心。

## 资源处理


当前实现为每个 MCB 设备分配恰好一个内存与一个 IRQ 资源。但这在未来很可能会改变。

### 内存资源


每个 MCB 设备恰好有一个内存资源，可以从 MCB 总线请求。该内存资源是 MCB 设备在载板内部的物理地址，旨在传递给 ioremap() 及其同类函数。它已经通过调用 request_mem_region() 从内核请求过了。

### 中断


每个 MCB 设备恰好有一个 IRQ 资源，可以从 MCB 总线请求。如果载板设备驱动实现了 ->get_irq() 回调方法，则返回由载板设备分配的 IRQ 号，否则返回 Chameleon 表中的 IRQ 号。该数字适合传递给 request_irq()。

## 编写 MCB 驱动


### 驱动结构


每个 MCB 驱动都有一个结构来标识设备驱动，以及标识 FPGA 内部 IP 核的设备 id。该驱动结构还包含回调方法，在驱动探测（probe）时执行
```

	static const struct mcb_device_id foo_ids[] = {
		{ .device = 0x123 },
		{ }
	};
	MODULE_DEVICE_TABLE(mcb, foo_ids);

	static struct mcb_driver foo_driver = {
	driver = {
		.name = "foo-bar",
		.owner = THIS_MODULE,
	},
		.probe = foo_probe,
		.remove = foo_remove,
		.id_table = foo_ids,
	};

```
### 探测与挂载


当加载一个驱动并找到它所服务的 MCB 设备时，MCB 核心将调用驱动的 probe 回调方法。当移除驱动时
```

	static init foo_probe(struct mcb_device *mdev, const struct mcb_device_id *id);
	static void foo_remove(struct mcb_device *mdev);

```
### 初始化驱动


当内核启动或插入你的 foo 驱动模块时，你必须执行驱动初始化。通常只需注册你的驱动就足够了
```

	static int __init foo_init(void)
	{
		return mcb_register_driver(&foo_driver);
	}
	module_init(foo_init);

	static void __exit foo_exit(void)
	{
		mcb_unregister_driver(&foo_driver);
	}
	module_exit(foo_exit);

```
```

	module_mcb_driver(foo_driver);

```
### 使用 DMA


为了使用内核的 DMA-API 函数，你需要使用载板设备的 'struct device'。幸运的是 'struct mcb_device' 嵌入了一个
```

        ret = dma_set_mask_and_coherent(&mdev->dma_dev, DMA_BIT_MASK(dma_bits));
        if (rc)
                /* Handle errors */

```
