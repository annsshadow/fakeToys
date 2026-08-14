
### V4L2 设备实例


每个设备实例由一个 struct v4l2_device 表示。非常简单的设备可以直接分配这个结构体，
但在大多数情况下你会把它嵌入到一个更大的结构体中。

你必须通过调用以下函数来注册设备实例：

	`v4l2_device_register <v4l2_device_register>`
	(dev, `v4l2_dev <v4l2_device>`)。

注册将初始化 `v4l2_device` 结构体。如果 dev->driver_data 字段为 `NULL`，
它将被链接到 `v4l2_dev <v4l2_device>` 参数。

希望与媒体设备框架集成的驱动，需要手动设置 dev->driver_data，使其指向嵌入了
struct v4l2_device 实例的驱动特定设备结构。这是通过在注册 V4L2 设备实例之前调用一次
`dev_set_drvdata()` 来实现的。它们还必须将 struct v4l2_device 的 mdev 字段设置为
指向一个已正确初始化并注册的 `media_device` 实例。

如果 `v4l2_dev <v4l2_device>`\ ->name 为空，则它将被设为从 dev 派生的值
（确切地说，是驱动名后跟 bus_id）。如果你在调用 `v4l2_device_register` 之前设置好它，
它将保持不变。如果 dev 为 `NULL`，那么你**必须**在调用 `v4l2_device_register`
之前设置好 `v4l2_dev <v4l2_device>`\ ->name。

你可以使用 `v4l2_device_set_name` 来基于驱动名和一个驱动全局的 atomic_t 实例设置名称。
这将生成诸如 `ivtv0`、`ivtv1` 这样的名称。如果名称以数字结尾，则会插入一个短横线：
`cx18-0`、`cx18-1` 等。该函数返回实例编号。

第一个 `dev` 参数通常是 `pci_dev`、`usb_interface` 或 `platform_device` 的
`struct device` 指针。dev 为 `NULL` 的情况很少见，但在 ISA 设备或当一个设备创建多个
PCI 设备时会发生，从而使得无法将 `v4l2_dev <v4l2_device>` 关联到某个特定的父设备。

你还可以提供一个 `notify()` 回调，子设备可以调用它来通知你事件。是否需要设置它取决于
子设备。子设备支持的任何通知必须在 `include/media/subdevice.h` 中的一个头文件里定义。

V4L2 设备通过调用以下函数注销：

	`v4l2_device_unregister`
	(`v4l2_dev <v4l2_device>`)。

如果 dev->driver_data 字段指向 `v4l2_dev <v4l2_device>`，它将被重置为 `NULL`。
注销也会自动注销该设备上的所有子设备（subdev）。

如果你有一个可热插拔设备（例如 USB 设备），那么在断开连接发生时父设备会变得无效。
由于 `v4l2_device` 有一个指向该父设备的指针，它也必须被清除，以标记父设备已经消失。
为此请调用：

	`v4l2_device_disconnect`
	(`v4l2_dev <v4l2_device>`)。

这并**不会**注销子设备，因此你仍然需要为此调用 `v4l2_device_unregister` 函数。
如果你的驱动不可热插拔，则无需调用 `v4l2_device_disconnect`。

有时你需要遍历某个特定驱动注册的所有设备。当多个设备驱动使用相同的硬件时通常是这种
情况。例如，ivtvfb 驱动是一个使用 ivtv 硬件的帧缓冲驱动。ALSA 驱动同理。

你可以如下遍历所有已注册的设备：


	static int callback(struct device **dev, void **p)
	{
		struct v4l2_device *v4l2_dev = dev_get_drvdata(dev);

		/** 测试该设备是否已初始化 **/
		if (v4l2_dev == NULL)
			return 0;
		...
		return 0;
	}

	int iterate(void *p)
	{
		struct device_driver *drv;
		int err;

		/* 在 PCI 总线上查找驱动 'ivtv'。
		pci_bus_type 是一个全局变量。对于 USB 总线使用 usb_bus_type。 */
		drv = driver_find("ivtv", &pci_bus_type);
		/** 遍历所有 ivtv 设备实例 **/
		err = driver_for_each_device(drv, NULL, p, callback);
		put_driver(drv);
		return err;
	}

有时你需要维护一个设备实例的运行计数器。这通常用于将设备实例映射到模块选项数组的索引。

推荐的做法如下：


	static atomic_t drv_instance = ATOMIC_INIT(0);

	static int drv_probe(struct pci_dev **pdev, const struct pci_device_id **pci_id)
	{
		...
		state->instance = atomic_inc_return(&drv_instance) - 1;
	}

如果你有多个设备节点，那么对于可热插拔设备，可能很难知道何时注销 `v4l2_device` 才是
安全的。为此 `v4l2_device` 提供了引用计数（refcounting）支持。每当调用
`video_register_device` 时引用计数加一，每当该设备节点被释放时减一。当引用计数
归零时，将调用 `v4l2_device` 的 release() 回调。你可以在那里做最终的清理。

如果创建了其它设备节点（例如 ALSA），你也可以通过调用以下函数手动增减引用计数：

	`v4l2_device_get`
	(`v4l2_dev <v4l2_device>`)。

或：

	`v4l2_device_put`
	(`v4l2_dev <v4l2_device>`)。

由于初始引用计数为 1，你还需要在 `disconnect()` 回调（对于 USB 设备）或 `remove()`
回调（例如对于 PCI 设备）中调用 `v4l2_device_put`，否则引用计数永远不会归零。

##### v4l2_device 函数与数据结构
