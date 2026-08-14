## PHY 子系统


:作者: Kishon Vijay Abraham I <kishon@ti.com>

本文档说明了通用 PHY 框架及其提供的 API，以及如何使用。

## 简介


**PHY** 是 physical layer（物理层）的缩写。它用于将设备连接到物理介质，例如
USB 控制器有一个 PHY，用来提供串行化、反串行化、编码、解码等功能，并负责
获取所需的数据传输速率。注意，某些 USB 控制器将 PHY 功能内嵌其中，而其他则使用外部
PHY。其他使用 PHY 的外设包括 Wireless LAN、Ethernet、SATA 等。

创建此框架的意图是将散布在整个 Linux 内核中的 PHY 驱动集中到 drivers/phy，以增加代码复用并
改善代码的可维护性。

此框架仅对使用外部 PHY（PHY 功能未内嵌于控制器中）的设备有用。

## 注册/注销 PHY provider


PHY provider 指实现了一个或多个 PHY 实例的实体。
对于 PHY provider 仅实现单个 PHY 实例的简单情况，框架在
of_phy_simple_xlate 中提供了它自己的 of_xlate 实现。如果 PHY provider 实现了多个实例，
它应提供自己的 of_xlate 实现。of_xlate 仅用于
dt（device tree）引导的情况。

```
	#define of_phy_provider_register(dev, xlate)    \
		__of_phy_provider_register((dev), NULL, THIS_MODULE, (xlate))

	#define devm_of_phy_provider_register(dev, xlate)       \
		__devm_of_phy_provider_register((dev), NULL, THIS_MODULE,
						(xlate))
```
of_phy_provider_register 与 devm_of_phy_provider_register 宏可用于
注册 phy_provider，它以 device 和 of_xlate 作为参数。对于 dt 引导情况，所有 PHY provider 都应使用上述
两个宏之一来注册该 PHY provider。

通常，与 PHY provider 关联的设备树节点会包含一组子节点，每个子节点代表一个 PHY。某些绑定可能为了
上下文和可扩展性而将子节点嵌套在额外的层级中，此时可使用低层的
of_phy_provider_register_full() 与 devm_of_phy_provider_register_full()
宏来覆盖包含子节点的节点。

```
	#define of_phy_provider_register_full(dev, children, xlate) \
		__of_phy_provider_register(dev, children, THIS_MODULE, xlate)

	#define devm_of_phy_provider_register_full(dev, children, xlate) \
		__devm_of_phy_provider_register_full(dev, children,
						     THIS_MODULE, xlate)

	void devm_of_phy_provider_unregister(struct device *dev,
		struct phy_provider *phy_provider);
	void of_phy_provider_unregister(struct phy_provider *phy_provider);
```
devm_of_phy_provider_unregister 与 of_phy_provider_unregister 可用于
注销该 PHY。

## 创建 PHY


PHY 驱动应创建 PHY，以便其他外设控制器能够使用它。PHY 框架提供了 2 个 API 来创建 PHY。

```
	struct phy *phy_create(struct device *dev, struct device_node *node,
			       const struct phy_ops *ops);
	struct phy *devm_phy_create(struct device *dev,
				    struct device_node *node,
				    const struct phy_ops *ops);
```
PHY 驱动可以使用上述 2 个 API 之一，通过传入 device 指针和 phy ops 来创建 PHY。
phy_ops 是一组用于执行 PHY 操作（如 init、exit、power_on 和 power_off）的函数指针。

为了在 phy_ops 中解引用私有数据（private data），PHY provider 驱动可以在创建 PHY 后使用
phy_set_drvdata()，并在 phy_ops 中使用 phy_get_drvdata() 取回私有数据。

## 获取对 PHY 的引用


在控制器能够使用该 PHY 之前，它必须先获得对它的引用。此框架提供了以下 API 来获取对 PHY 的引用。

```
	struct phy *phy_get(struct device *dev, const char *string);
	struct phy *devm_phy_get(struct device *dev, const char *string);
	struct phy *devm_phy_optional_get(struct device *dev,
					  const char *string);
	struct phy *devm_of_phy_get(struct device *dev, struct device_node *np,
				    const char *con_id);
	struct phy *devm_of_phy_optional_get(struct device *dev,
					     struct device_node *np,
					     const char *con_id);
	struct phy *devm_of_phy_get_by_index(struct device *dev,
					     struct device_node *np,
					     int index);
```
phy_get、devm_phy_get 与 devm_phy_optional_get 可用于获取 PHY。
在 dt 引导情况下，string 参数应包含 dt 数据中给出的 phy 名称；在
非 dt 引导情况下，它应包含 PHY 的 label（标签）。两个
devm_phy_get 在成功获取 PHY 后，使用 devres 将设备与 PHY 关联。
在驱动分离（detach）时，会在 devres 数据上调用释放函数并释放 devres 数据。
_optional_get 变体应在 phy 为可选时使用。这些函数永远不会返回 -ENODEV，而是在
找不到 phy 时返回 NULL。

某些通用驱动（如 ehci）可能使用多个 phys。在这种情况下，
devm_of_phy_get 或 devm_of_phy_get_by_index 可用于基于名称或索引获取 phy 引用。

应注意，NULL 是一个合法的 phy 引用。所有对 NULL phy 的 phy 消费者调用都会变成 NOP（空操作）。
即释放调用、phy_init() 与 phy_exit() 调用，以及 phy_power_on() 与
phy_power_off() 调用，在应用于 NULL phy 时都是 NOP。NULL phy 在处理可选 phy 设备的场景中很有用。

## API 调用顺序


```
    [devm_][of_]phy_get()
    phy_init()
    phy_power_on()
    [phy_set_mode[_ext]()]
    ...
    phy_power_off()
    phy_exit()
    [[of_]phy_put()]
```
某些 PHY 驱动可能未实现 `phy_init` 或 `phy_power_on`，
但控制器应始终调用这些函数以兼容其他 PHY。某些 PHY 可能需要 `phy_set_mode <phy_set_mode_ext>`，
而其他则可能使用默认模式（通常通过 devicetree 或其他固件配置）。为了兼容性，如果你知道
将使用的模式，应始终调用此函数。通常，此函数应在 `phy_power_on` 之后调用，
尽管某些 PHY 驱动可能允许在任何时候调用它。

## 释放对 PHY 的引用


当控制器不再需要该 PHY 时，它必须释放使用上述章节提到的 API 所获得的 PHY 引用。PHY 框架提供了 2 个 API 来释放对 PHY 的引用。

```
	void phy_put(struct phy *phy);
	void devm_phy_put(struct device *dev, struct phy *phy);
```
这两个 API 都用于释放对 PHY 的引用，devm_phy_put 会销毁与此 PHY 关联的 devres。

## 销毁 PHY


当创建该 PHY 的驱动被卸载时，它应销毁它创建的 PHY：

```
	void phy_destroy(struct phy *phy);
	void devm_phy_destroy(struct device *dev, struct phy *phy);
```
这两个 API 都会销毁 PHY，devm_phy_destroy 会销毁与此 PHY 关联的 devres。

## PM Runtime


此子系统启用了 pm runtime（电源管理运行时）。因此在创建 PHY 时，
会调用此子系统创建的 phy device 的 pm_runtime_enable，而在销毁 PHY 时，
会调用 pm_runtime_disable。注意，此子系统创建的 phy device 将是调用
phy_create（PHY provider 设备）的设备的子设备。

因此，此子系统创建的 phy_device 的 pm_runtime_get_sync 会由于父子关系而调用
PHY provider 设备的 pm_runtime_get_sync。还应注意，phy_power_on 与 phy_power_off 分别执行
phy_pm_runtime_get_sync 与 phy_pm_runtime_put。
还有一些导出的 API，如 phy_pm_runtime_get、phy_pm_runtime_get_sync、
phy_pm_runtime_put 与 phy_pm_runtime_put_sync，用于执行 PM 操作。

## PHY 映射


为了在没有 DeviceTree 帮助的情况下获取对 PHY 的引用，框架提供了查找（lookup）机制，类似于 clkdev，
后者允许将 clk 结构绑定到设备。当已经存在指向 struct phy 的句柄时，可以在运行时进行查找。

框架提供了以下 API 用于注册和注销查找：

```
	int phy_create_lookup(struct phy *phy, const char *con_id,
			      const char *dev_id);
	void phy_remove_lookup(struct phy *phy, const char *con_id,
			       const char *dev_id);
```

## DeviceTree 绑定


PHY dt 绑定的文档可在以下位置找到：
Documentation/devicetree/bindings/phy/phy-bindings.txt
