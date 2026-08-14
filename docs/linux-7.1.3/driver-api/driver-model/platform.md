## 平台设备与驱动


有关平台总线（platform bus）的驱动模型接口，请参见 <linux/platform_device.h>：
platform_device 与 platform_driver。这条伪总线（pseudo-bus）用于连接那些
基础设施最简的总线上的设备，例如许多系统级芯片（system-on-chip）处理器上
集成外设所使用的总线，或某些“传统”PC 互连；与之相对的是像 PCI 或 USB 这样
正式规范定义的大型总线。


#### 平台设备


平台设备通常是作为系统中独立的实体出现的设备。这包括基于传统端口的设备、
到外围总线的主桥，以及大多数集成到系统级芯片平台中的控制器。它们的共同点
通常是能够由 CPU 总线直接寻址。极少数情况下，一个 platform_device 会通过
某段其他类型的总线连接；但其寄存器仍然可以直接寻址。

平台设备会被赋予一个名称（用于驱动绑定）以及一个
```

  struct platform_device {
	const char	*name;
	u32		id;
	struct device	dev;
	u32		num_resources;
	struct resource	*resource;
  };


```
#### 平台驱动


平台驱动遵循标准的驱动模型约定，其中发现/枚举由驱动之外的部分处理，驱动
提供 probe() 与 remove() 方法。它们支持电源管理
```

  struct platform_driver {
	int (*probe)(struct platform_device *);
	void (*remove)(struct platform_device *);
	void (*shutdown)(struct platform_device *);
	int (*suspend)(struct platform_device *, pm_message_t state);
	int (*resume)(struct platform_device *);
	struct device_driver driver;
	const struct platform_device_id *id_table;
	bool prevent_deferred_probe;
	bool driver_managed_dma;
  };

```
注意，probe() 通常应当验证所指定的设备硬件确实是否存在；有时平台初始化代码
无法确认这一点。探测过程可以使用设备资源，包括时钟，以及设备的 platform_data。

```

	int platform_driver_register(struct platform_driver *drv);

```
或者，在设备已知不可热插拔的常见情况下，probe() 例程可以放在 init 段中以
减小驱动的
```

	int platform_driver_probe(struct platform_driver *drv,
			  int (*probe)(struct platform_device *))

```
内核模块可以由多个平台驱动组成。平台核心
```

	int __platform_register_drivers(struct platform_driver * const *drivers,
				      unsigned int count, struct module *owner);
	void platform_unregister_drivers(struct platform_driver * const *drivers,
					 unsigned int count);

```
如果其中一个驱动注册失败，到该点为止已注册的所有驱动将以相反的顺序注销。
注意，有一个便捷的
```

	#define platform_register_drivers(drivers, count)


```
#### 设备枚举


作为一般规则，平台相关（且常常是板级相关）的初始化代码会
```

	int platform_device_register(struct platform_device *pdev);

	int platform_add_devices(struct platform_device **pdevs, int ndev);

```
一般的规则是只注册那些实际存在的设备，但在某些情况下可能会注册额外的设备。
例如，一个内核可能被配置为配合某个外部网络适配器工作，而该适配器可能并未
在所有电路板上都焊接上；或者类似地，配合某个集成的控制器工作，而某些电路板
可能没有将它连接到任何外围设备。

在某些情况下，引导固件会导出描述某个给定电路板上所装配设备的表。如果没有
这样的表，系统初始化代码设置正确设备的唯一方式通常就是为特定的目标电路板
构建内核。这种板级特定的内核在嵌入式与定制系统开发中很常见。

在许多情况下，与平台设备关联的内存和 IRQ 资源不足以让设备驱动正常工作。
板级初始化代码通常会使用设备的 platform_data 字段来提供额外信息。

嵌入式系统常常需要一个或多个时钟来驱动平台设备，这些时钟通常在被实际需要
之前（为省电）保持关闭。系统初始化也会将这些时钟与设备关联起来，以便对
clk_get(&pdev->dev, clock_name) 的调用能在需要时返回它们。


#### 传统驱动：设备探测


有些驱动没有被完全转换为驱动模型，因为它们承担了非驱动的角色：由驱动自己
注册其平台设备，而不是交给系统基础设施来做。这样的驱动无法热插拔或冷插拔，
因为这些机制要求设备的创建位于与驱动不同的系统组件中。

这样做唯一“合理”的理由，是处理较旧的系统设计——它们像最初的 IBM PC 一样，
依赖于容易出错的“探测硬件”模型来进行硬件配置。较新的系统在很大程度上已经
放弃了该模型，转而采用总线级的动态配置支持（PCI、USB），或者由引导固件提供的
设备表（例如 x86 上的 PNPACPI）。关于“什么东西可能在什么地方”存在太多相互冲突
的选项，即便是操作系统基于经验的猜测也常常出错，从而带来麻烦。

这种风格的驱动是不被鼓励的。如果你正在更新这样一个驱动，请尽量将设备枚举
移到驱动之外的更合适的位置。这通常会是一种清理，因为这类驱动往往已经有了
“正常”的模式，例如使用由 PNP 或平台设备初始化所创建的设备节点。

尽管如此，仍有一些 API 用于支持这类传统驱动。请避免
```

	struct platform_device *platform_device_alloc(
			const char *name, int id);

```
你可以使用 platform_device_alloc() 动态分配一个设备，然后用资源与
platform_device_register() 来初始化它。
```

	struct platform_device *platform_device_register_simple(
			const char *name, int id,
			struct resource *res, unsigned int nres);

```
你可以使用 platform_device_register_simple() 作为一步式调用来分配并注册
一个设备。


#### 设备命名与驱动绑定


platform_device.dev.bus_id 是设备的规范名称。它由两部分构成：

    - platform_device.name ... 该部分也用于驱动匹配。

    - platform_device.id ... 设备实例编号，否则为 “-1” 以表示只有一个。

这两部分被拼接起来，因此 name/id 为 “serial”/0 表示 bus_id 为 “serial.0”，
而 “serial/3” 表示 bus_id 为 “serial.3”；两者都会使用名为 “serial” 的
platform_driver。而 “my_rtc”/-1 的 bus_id 为 “my_rtc”（无实例 id），并
使用名为 “my_rtc” 的 platform_driver。

驱动绑定由驱动核心自动执行，在找到设备与驱动之间的匹配后调用驱动的 probe()。
如果 probe() 成功，驱动与设备就会像通常那样绑定。有三种不同的方式可以找到
这样的匹配：

    - 每当一个设备被注册时，都会检查该总线上的驱动是否匹配。平台设备应当在
      系统启动的很早阶段就注册。

    - 当使用 platform_driver_register() 注册一个驱动时，会检查该总线上所有
      未绑定的设备是否匹配。驱动通常在启动的较晚阶段，或通过模块加载来注册。

    - 使用 platform_driver_probe() 注册驱动与使用 platform_driver_register()
      工作方式相同，唯一的区别是：如果之后有另一个设备注册，该驱动不会被再次
      探测。（这没问题，因为该接口仅用于不可热插拔的设备。）


#### 早期平台设备与驱动


早期平台接口在系统启动的早期就向平台设备驱动提供平台数据。该代码构建在
early_param() 命令行解析之上，可以在很早的阶段执行。

示例：用 6 个步骤实现 “earlyprintk” 类的早期串行控制台


#### 1. 注册早期平台设备数据


体系结构代码使用 early_platform_add_devices() 函数注册平台设备数据。对于
早期串行控制台而言，这里应是串行端口的硬件配置。此时注册的设备稍后会被用来
与早期平台驱动匹配。


#### 2. 解析内核命令行


体系结构代码调用 parse_early_param() 来解析内核命令行。这将执行所有匹配的
early_param() 回调。用户指定的早期平台设备会在此刻注册。对于早期串行控制台
的情况，用户可以在内核命令行上以 “earlyprintk=serial.0” 的形式指定端口，其中
“earlyprintk” 是类（class）字符串，“serial” 是平台驱动的名称，而 0 是平台
设备 id。如果 id 为 -1，则可以省略点号与 id。


#### 3. 安装属于某个类的早期平台驱动


体系结构代码可以选择使用 early_platform_driver_register_all() 函数强制注册
属于某个类的所有早期平台驱动。来自第 2 步的用户指定设备优先级高于这些。串行
驱动示例省略了这一步，因为早期串行驱动代码应当被禁用，除非用户在内核命令行
上指定了端口。


#### 4. 早期平台驱动注册


使用 early_platform_init() 的内置平台驱动会在第 2 步或第 3 步期间自动注册。
串行驱动示例应使用 early_platform_init("earlyprintk", &platform_driver)。


#### 5. 探测属于某个类的早期平台驱动


体系结构代码调用 early_platform_driver_probe() 来将属于某个类的已注册早期
平台设备与已注册的早期平台驱动匹配。匹配到的设备会被调用其 probe()。这一步
可以在早期启动期间的任意时刻执行。对于串行端口的情况，越早越好。


#### 6. 在早期平台驱动 probe() 内部


驱动代码在早期启动期间需要特别小心，尤其是在内存分配与中断注册方面。probe()
函数中的代码可以使用 is_early_platform_device() 来检查它是在早期平台设备
时刻还是常规平台设备时刻被调用的。早期串行驱动会在此刻执行 register_console()。

更多信息请参见 <linux/platform_device.h>。
