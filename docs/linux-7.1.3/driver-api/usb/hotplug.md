#### USB 热插


## Linux 热插



USB（以Cardbus PCI）这类可热插拔的总线上，终端用户在系统通电状态下将设备插入总线。在大多数情况下，用户希望设备能够立即可用。这意味着系统必须完成许多工作，包括：

    - 找到一个能够处理该设备的驱动。这可能涉及加载一个内核模块；较新的驱动可以使module-init-tools
      将其设备（及类别）支持信息发布给实用工具

    - 将一个驱动绑定到该设备。总线框架通过其设备驱动的 probe() 例程来完成这一工作

    - 通知其他子系统配置新设备。打印队列可能需要启用，网络需要启动，磁盘分区需要挂载，等等
      在某些情况下，这些将是驱动特定的操作

这涉及内核态与用户态操作的混合。使设备立即可用意味着任何用户态操作都不能等待管理员去执行
内核必须触发它们，无论是被动地（触发某个监控守护进程调用辅助程序）还是主动地（直接调用这样的
用户态辅助程序）

那些被触发的操作必须支持系统的管理策略；此类程序在此被称为“策略代理”（policy agents）
它们通常涉及 shell 脚本，这些脚本分派给更为熟悉的管理工具

由于其中一些操作依赖于关于驱动（元数据）的信息，而这些信息目前仅在驱动被动态链接时才可用，
因此当你配置一个高度模块化的系统时，可以获得最佳的热插拔效果

## 内核热插拔辅助程(``/sbin/hotplug``)


存在一个内核参数：`/proc/sys/kernel/hotplug`，它通常保存路径`/sbin/hotplug`。该参数指定了一
程序，内核可在不同时机调用它

/sbin/hotplug 程序可由任何子系统作为其响应配置变更的一部分，从该系统中的一个线程调用。只需要一个参数：
被通知某内核事件的子系统名称。该名称被用作进一步事件分派的第一把钥匙；任何其他参数和环境参数由
发起该调用的子系统指定

热插拔软件及其他资源可在以下位置获取

	http://linux-hotplug.sourceforge.net

邮件列表信息也可在该站点获取


## USB 策略代理


USB 子系统当前在 USB 设备被添加或从系统中移除时调`/sbin/hotplug`。该调用由内hub 工作队列
[hub_wq] 完成，或作为hub 初始化的一部分（由 init、modprobe、kapmd 等完成）。它唯一的命令行参数
字符"usb"，并传递以下环境变量：

========== ============================================
ACTION     `add`, `remove`
PRODUCT    USB 厂商、产品和版本代码（十六进制）
TYPE       设备类别代码（十进制
INTERFACE  接口 0 的类别代码（十进制）
========== ============================================

如果配置"usbdevfs"，则还会传DEVICE DEVFS。DEVICE 是该设备的路径名，对于具有多个和/
备用接口、从而令驱动选择复杂化的设备很有用。按设计，USB 热插拔独立于 `usbdevfs`：你可以在不使用
文件系统、也不运行用户态守护进程来检测系统配置变化的情况下，完成 USB 设备设置的大部分关键部分

当前可用的策略代理实现可以为模块加载驱动，并可以调用驱动特定的设置脚本。最新的实现利用USB
module-init-tools 支持。后续的代理可能会卸载驱动


## USB Modutils 支持


当前版本module-init-tools 会创建一`modules.usbmap` 文件，其中包含每个驱`MODULE_DEVICE_TABLE`
中的条目。此类文件可被各种用户态策略代理用来确保加载所有正确的驱动模块，无论是在启动时刻还是之后

有关此类表条目的完整信息，请参见 `linux/usb.h`；或查看现有驱动。每个表条目描述了一个或多个在将驱动
与某设备或设备类别进行匹配时所使用的判据。具体的判据"match_flags" 中置位的位与字段值配对来标识
你可以直接构造这些判据，或使
```

    USB_DEVICE (vendorId, productId)
	... matching devices with specified vendor and product ids
    USB_DEVICE_VER (vendorId, productId, lo, hi)
	... like USB_DEVICE with lo <= productversion <= hi
    USB_INTERFACE_INFO (class, subclass, protocol)
	... matching specified interface class info
    USB_DEVICE_INFO (class, subclass, protocol)
	... matching specified device class info

```
下面是一个简短示例，适用于一个支持若干特USB 设备的驱
```

    static const struct usb_device_id mydriver_id_table[] = {
	{ USB_DEVICE (0x9999, 0xaaaa), driver_info: QUIRK_X },
	{ USB_DEVICE (0xbbbb, 0x8888), driver_info: QUIRK_Y|QUIRK_Z },
	...
	{ } /* end with an all-zeroes entry */
    };
    MODULE_DEVICE_TABLE(usb, mydriver_id_table);

```
大多USB 设备驱动应当将这些表同时传递给 USB 子系统和模块管理子系统。不过并非所有驱动都如此
某些驱动框架通过构建USB 之上的接口进行连接，因此它们不需要这样的 struct usb_driver

直接连接USB 子系统的驱动应当按如下方式声
```

    static struct usb_driver mydriver = {
	.name		= "mydriver",
	.id_table	= mydriver_id_table,
	.probe		= my_probe,
	.disconnect	= my_disconnect,

	/*
	if using the usb chardev framework:
	    .minor		= MY_USB_MINOR_START,
	    .fops		= my_file_ops,
	if exposing any operations through usbdevfs:
	    .ioctl		= my_ioctl,
	*/
    };

```
USB 子系统获知某个驱动的 device ID 表后，它会在选择probe() 的驱动时使用该表。负责新设备
处理的线程会将各驱动device ID 条目与设备的接口device 描述符进行比对。只有在匹配时它才会调用
`probe()`，而传`probe()` 的第三个参数将是所匹配的条目

如果你没有为驱动提供 `id_table`，那么你的驱动可能会针对每个新设备都probe 一次；传给 `probe()`
的第三个参数将是 `NULL`
