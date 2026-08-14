## ISA 驱动


以下文本改编自由 Rene Herman 撰写的 ISA 总线驱动初始提交的提交说明。

在近期关于"使用 platform 设备的 ISA 驱动"的讨论中，有人指出（ALSA 的）ISA
驱动遇到了一个问题：由于在驱动模型中 probe() 错误没有被向上传递，因此当
探测不到硬件时，它们无法选择让驱动加载（更准确地说是设备注册）失败。在此
过程中，我建议单独设立一条 ISA 总线可能是最好的方案；Russell King 表示同意，
并建议该总线可以使用 .match() 方法来进行实际的设备发现。

附带的实现正是如此。对于这种旧的、不可（通用地）发现的 ISA 硬件，只有驱动
自身才能进行发现，因此与 platform_bus 不同，isa_bus 也将 match() 向上分发到
驱动。

另一个不同点是：这些设备之所以存在于驱动模型中，只是因为驱动为了驱动它们而
创建了它们，这意味着所有的设备创建也都已被内部化。

这种方式提供的使用模型很好，并且已经得到 ALSA 方面 Takashi Iwai 和
Jaroslav Kysela 的认可。ALSA 驱动的 module_init 因此与其它总线模型非常相似。
这从 ALSA 的 ISA 驱动中移除了大量重复的初始化代码。
```

	static int __init alsa_card_foo_init(void)
	{
		return isa_register_driver(&snd_foo_isa_driver, SNDRV_CARDS);
	}

	static void __exit alsa_card_foo_exit(void)
	{
		isa_unregister_driver(&snd_foo_isa_driver);
	}

```
传入的 isa_driver 结构体就是常规的驱动结构体，内嵌了一个 struct device_driver、
常规的 probe/remove/shutdown/suspend/resume 回调，以及如前所述的 .match 回调。

你看到的传入的 "SNDRV_CARDS" 是一个 "unsigned int ndev" 参数，表示要创建
多少个设备并以之调用我们的方法。

platform_driver 的回调以一个 platform_device 参数被调用；isa_driver 的回调
则直接以 ⟦C0⟧struct device *dev, unsigned int id⟦C0⟧ 对被调用——由于设备创建
完全在总线内部，完全不泄漏 isa_dev 是最干净的做法。id 毕竟是我们除了
struct device 之外唯一想要的东西，这也让回调中的代码更美观。

借助这个额外的 .match() 回调，ISA 驱动拥有了全部选项。如果 ALSA 想保留旧的
"不加载"行为，它可以把全部旧的 .probe 放进 .match 中，这样只有在一切都存在且
齐备时才保持注册。如果它想要始终加载的行为（在向 platform 设备切换后曾短暂地
无意中如此），它可以干脆不提供 .match()，并像以前一样在 .probe() 中做所有事情。

如果它（正如 Takashi Iwai 早先建议的、作为一种更贴近健康总线模型的方式）想在
稍后的绑定可能成功时加载，它可以在 .match() 中处理前置条件（例如检查用户是否
希望启用该卡，以及 port/irq/dma 值是否已经传入），而把其余一切放在 .probe() 中。
这是最理想的模型。

进入代码……

它只导出两个函数：isa_{,un}register_driver()。

isa_register_driver() 注册 struct device_driver，然后遍历传入的 ndev，创建
设备并注册它们。

它做的第一件事是检查该设备是否确实是该驱动的设备之一，方式是查看设备的
platform_data 指针是否被设为本驱动。platform 设备比较字符串，但既然一切都已
内部化，我们就无需那样做，因此 isa_register_driver() 把 dev->platform_data
当作 isa_driver 指针来用，以便在此处检查。
```

	int isa_bus_match(struct device *dev, struct device_driver *driver)
	{
		struct isa_driver *isa_driver = to_isa_driver(driver);

		if (dev->platform_data == isa_driver) {
			if (!isa_driver->match ||
				isa_driver->match(dev, to_isa_dev(dev)->id))
				return 1;
			dev->platform_data = NULL;
		}
		return 0;
	}

```
我相信 platform_data 可用于此目的，但如果并不愿意，把 isa_driver 指针移到私有的
struct isa_dev 中当然也完全可以。

然后，如果驱动没有提供 .match，则匹配。如果提供了，就调用驱动的 match() 方法
来判定是否匹配。

如果**没有**匹配，dev->platform_data 会被重置以向 isa_register_driver 表明这一点，
后者随后可以再次注销该设备。

如果在这一切过程中发生任何错误，或者根本没有设备匹配，则一切都会被回退，
并返回该错误或 -ENODEV。

isa_unregister_driver() 只是注销已匹配的设备以及驱动自身。

module_isa_driver 是一个用于 ISA 驱动的辅助宏，适用于那些在模块 init/exit 中
不做任何特殊事情的驱动。它消除了大量样板代码。每个模块只能使用该宏一次，调用
它会替换 module_init 和 module_exit。

max_num_isa_dev 是一个宏，用于在给定 ISA 设备的地址范围时，确定在 I/O 端口
地址空间中可能注册的最大 ISA 设备数量。
