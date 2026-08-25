#### USB 核心回调


## usbcore 会进行哪些回调？


Usbcore 会通过驱动结构中定义的回调，以及驱动提交的 URB 完成处理程序（completion handler）来调用驱动本文档仅涉及前者。这两类回调彼此完全独立。有完成回调的信息可以在 usb-urb 中找到
在驱动结构中定义的回调有
1. 热插拔（Hotplugging）回调：

 - @probe	调用以查看驱动是否愿意管理设备上的某	特定接口
 - @disconnect	当接口不再可访问时调用，通常是因	其设备已被（或正在被）断开连接，或	驱动模块正在被卸载
2. 经由 usbfs 的特殊后门：

 - @ioctl	用于那些希望通过 "usbfs" 文件系统与用户空	通信的驱动。这让设备能够提供向用户空间
	暴露信息的方式，而不论它们在其余情况下是否（或是否不	出现在文件系统中
3. 电源管理（PM）回调：

 - @suspend	当设备即将被挂起时调用
 - @resume	当设备正在被恢复时调用
 - @reset_resume	当挂起的设备被重置而非被恢复时调用
4. 设备级操作：

 - @pre_reset	当设备即将被重置时调用
 - @post_reset	在设备被重置之后调用
ioctl 接口）只有在你有非常充分的理由时才应使用如今更推荐使sysfs。PM 回调usb-power-management 中单独介绍
## 调用约定


所有回调相互排斥。无需针对其他 USB 回调进行加锁所有回调都从任务上下文中调用。你可以睡眠。然而，重要的是
所有睡眠都应在时间上有较小的固定上限。特别地，你
不得调用用户空间并等待结果
## 热插拔回

这些回调旨在将驱动与接口关联和解除关联驱动与接口之间的绑定是独占的
### probe() 回调


```

  int (*probe) (struct usb_interface *intf,
		const struct usb_device_id *id);

```

接受或拒绝一个接口。如果你接受该设备，返回 0否则返回 -ENODEV -ENXIO。只有在初始化期间发生了
真正的错误、导致驱动无法接受本应被接受的设备时才应使用其他错误码强烈建议你使usbcore 的设usb_set_intfdata()，将数据结构与接口关联起来，以便
你知道与特定接口关联的内部状态和身份。该设备不会被挂起，
你可以对调用你的接口以及设备的端0 进行 IO在这里进行耗时不太长的设备初始化是个好主意
### disconnect() 回调


```

  void (*disconnect) (struct usb_interface *intf);

```

该回调是一个断开与接口之间任何连接的信号从该回调返回后，你不得再对设备进行任IO你也不得进行任何其他可能干扰绑定到该接口另一个驱动的操作，例如电源管理操作。该设备未完成的操作必须在此回调返回之前完成或中止
如果你是因物理断开连接而被调用，你所有的 URB 都会usbcore 终止。注意在这种情况下，disconnect 在物理断开之后一段时间才被调用。因此你的驱动必须准备好
即使在回调之前也能处理失败的 IO
## 设备级回

### pre_reset


```

  int (*pre_reset)(struct usb_interface *intf);

```

驱动或用户空间正在触发包含作为参数传入接口的
设备的重置。停IO，等待所有未完成URB 完成，并保存
你需要恢复的任何设备状态。在 post_reset 方法被调用之前，
不得再提交任URB
如果你需要在这里分配内存，若你处于原子上下文请使GFP_NOIO GFP_ATOMIC
### post_reset


```

  int (*post_reset)(struct usb_interface *intf);

```

重置已完成。恢复任何已保存的设备状态并重新开使用该设备
如果你需要在这里分配内存，若你处于原子上下文请使GFP_NOIO GFP_ATOMIC
## 调用顺序


除了 probe 之外，不会为未绑定到你的驱动接口调用任何回调
不会为已绑定到驱动的接口调用 probe因此在一次成功的 probe 之后，disconnect 会在
对同一接口的另一probe 之前被调用
一旦你的驱动绑定到某个接口，disconnect 可以在任何时刻被调用唯独不能pre_reset post_reset 之间pre_reset 之后总是跟着 post_reset，即使重置失或设备已被拔下
suspend 之后总是跟着以下之一：resume、reset_resume disconnect