
### V4L2 设备实例


每个设备实例由一struct v4l2_device 表示。非常简单的设备可以直接分配这个结构体，
但在大多数情况下你会把它嵌入到一个更大的结构体中
你必须通过调用以下函数来注册设备实例：

	`v4l2_device_register <v4l2_device_register>`
	(dev, `v4l2_dev <v4l2_device>`)銆。
注册将初始化 `v4l2_device` 结构体。如dev->driver_data 字段`NULL`它将被链接到 `v4l2_dev <v4l2_device>` 参数
希望与媒体设备框架集成的驱动，需要手动设dev->driver_data，使其指向嵌入了
struct v4l2_device 实例的驱动特定设备结构。这是通过在注V4L2 设备实例之前调用一`dev_set_drvdata()` 来实现的。它们还必须struct v4l2_device mdev 字段设置指向一个已正确初始化并注册`media_device` 实例
如果 `v4l2_dev <v4l2_device>`\ ->name 为空，则它将被设为从 dev 派生的（确切地说，是驱动名后跟 bus_id）。如果你在调`v4l2_device_register` 之前设置好它它将保持不变。如dev `NULL`，那么你**必须**在调`v4l2_device_register`
之前设置`v4l2_dev <v4l2_device>`\ ->name
你可以使`v4l2_device_set_name` 来基于驱动名和一个驱动全局atomic_t 实例设置名称这将生成诸如 `ivtv0`、`ivtv1` 这样的名称。如果名称以数字结尾，则会插入一个短横线`cx18-0`、`cx18-1` 等。该函数返回实例编号
第一`dev` 参数通常`pci_dev`、`usb_interface` `platform_device` `struct device` 指针。dev `NULL` 的情况很少见，但ISA 设备或当一个设备创建多PCI 设备时会发生，从而使得无法将 `v4l2_dev <v4l2_device>` 关联到某个特定的父设备
你还可以提供一`notify()` 回调，子设备可以调用它来通知你事件。是否需要设置它取决子设备。子设备支持的任何通知必须`include/media/subdevice.h` 中的一个头文件里定义
V4L2 设备通过调用以下函数注销
	`v4l2_device_unregister`
	(`v4l2_dev <v4l2_device>`)銆。
如果 dev->driver_data 字段指向 `v4l2_dev <v4l2_device>`，它将被重置`NULL`注销也会自动注销该设备上的所有子设备（subdev）
如果你有一个可热插拔设备（例如 USB 设备），那么在断开连接发生时父设备会变得无效由于 `v4l2_device` 有一个指向该父设备的指针，它也必须被清除，以标记父设备已经消失为此请调用：

	`v4l2_device_disconnect`
	(`v4l2_dev <v4l2_device>`)銆。
这并**不会**注销子设备，因此你仍然需要为此调`v4l2_device_unregister` 函数如果你的驱动不可热插拔，则无需调用 `v4l2_device_disconnect`
有时你需要遍历某个特定驱动注册的所有设备。当多个设备驱动使用相同的硬件时通常是这情况。例如，ivtvfb 驱动是一个使ivtv 硬件的帧缓冲驱动。ALSA 驱动同理
你可以如下遍历所有已注册的设备：


	static int callback(struct device **dev, void **p)
	{
		struct v4l2_device *v4l2_dev = dev_get_drvdata(dev);

		/** 测试该设备是否已初始**/
		if (v4l2_dev == NULL)
			return 0;
		...
		return 0;
	}

	int iterate(void *p)
	{
		struct device_driver *drv;
		int err;

		/* PCI 总线上查找驱'ivtv'		pci_bus_type 是一个全局变量。对USB 总线使用 usb_bus_type*/
		drv = driver_find("ivtv", &pci_bus_type);
		/** 遍历所ivtv 设备实例 **/
		err = driver_for_each_device(drv, NULL, p, callback);
		put_driver(drv);
		return err;
	}

有时你需要维护一个设备实例的运行计数器。这通常用于将设备实例映射到模块选项数组的索引
推荐的做法如下：


	static atomic_t drv_instance = ATOMIC_INIT(0);

	static int drv_probe(struct pci_dev **pdev, const struct pci_device_id **pci_id)
	{
		...
		state->instance = atomic_inc_return(&drv_instance) - 1;
	}

如果你有多个设备节点，那么对于可热插拔设备，可能很难知道何时注销 `v4l2_device` 才是
安全的。为`v4l2_device` 提供了引用计数（refcounting）支持。每当调`video_register_device` 时引用计数加一，每当该设备节点被释放时减一。当引用计数
归零时，将调`v4l2_device` release() 回调。你可以在那里做最终的清理
如果创建了其它设备节点（例如 ALSA），你也可以通过调用以下函数手动增减引用计数
	`v4l2_device_get`
	(`v4l2_dev <v4l2_device>`)銆。
或：

	`v4l2_device_put`
	(`v4l2_dev <v4l2_device>`)銆。
由于初始引用计数1，你还需要在 `disconnect()` 回调（对USB 设备）或 `remove()`
回调（例如对PCI 设备）中调用 `v4l2_device_put`，否则引用计数永远不会归零
##### v4l2_device 函数与数据结