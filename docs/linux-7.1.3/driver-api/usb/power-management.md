
#### USB 的电源管

:Author: Alan Stern <stern@rowland.harvard.edu>
:Date: Last-updated: February 2014

..
# 	目录
 - 什么是电源管理 - 什么是远程唤醒（Remote Wakeup）？
 - USB 设备何时处于空闲状态？
 - 动PM 的形 - 动PM 的用户接 - 更改默认的空闲延迟时 - 警告
 - 面向电源管理的驱动接 - 面向 autosuspend autoresume 的驱动接 - 驱动接口的其他部 - 互斥
 - 动PM 与系PM 之间的交 - xHCI 硬件链路 PM
 - USB 端口电源控制
 - 端口电源控制的用户接 - 建议的用户空间端口电源策

### 什么是电源管理

电源管理（PM）是一种通过在不使用计算机系统某些部分时将其挂起（suspend）来节约能源做法。当一个组件被`suspended`（挂起）时，它处于一种非功能性的低功耗状态；它甚至可能被
完全关闭。当内核需要使用一个被挂起的组件时，它可以`resumed`（被恢复，回到功能性的功耗状态）。（也有一些其他形式的 PM，其中组件被置于功能较弱但仍可用的状态，而不是被
挂起；例如降CPU 的时钟频率。本文档不讨论这些其他形式。）

当被挂起的部件包CPU 以及系统的其余大部分时，我们称之为“系统挂起”（system suspend）当整个系统仍在运行而某个特定设备被关闭时，我们称之为“动态挂起”（dynamic suspend，也
称为“运行时挂起”runtime suspend 或“选择性挂起”selective suspend）。本文档主要关注 USB
子系统中动PM 是如何实现的，尽管系PM 也有一定程度的覆盖（关于系PM 的更多信请参`Documentation/power/*.rst`）
只有当内核在构建时启用了 `CONFIG_SUSPEND` `CONFIG_HIBERNATION` 时，系统 PM 支持存在。USB 的动PM 支持只要内核在构建时启用`CONFIG_PM` 就存在
[历史上，USB 的动PM 支持只有内核在构建时启用`CONFIG_USB_SUSPEND`（它依赖`CONFIG_PM_RUNTIME`）才存在。从 3.10 内核版本开始，只要内核在构建时启用`CONFIG_PM_RUNTIME`，USB 的动PM 支持就存在。`CONFIG_USB_SUSPEND` 选项已被移除。]

### 什么是远程唤醒（Remote Wakeup）？


当一个设备被挂起后，通常它不会恢复，直到计算机告诉它恢复。同样，如果整个计算机被挂起通常它也不会恢复，直到用户告诉它恢复，例如按下电源按钮或打开上盖
然而，某些设备有能力自行恢复，或者请求内核恢复它们，甚至告诉整个计算机恢复。这种能有几种名称，例如“Wake On LAN”（局域网唤醒）；我们将其统称为“远程唤醒”（remote
wakeup）。当一个设备启用了远程唤醒并且被挂起时，它可能会响应某个外部事件而自行恢复（发送恢复请求）。例子包括：被挂起的键盘在按键时恢复，或者被挂起USB 集线器在插入设备恢复
### USB 设备何时处于空闲状态？


当内核认为一个设备没有在忙于做任何重要的事情、从而成为被挂起的候选对象时，该设备就是
空闲的。确切的定义取决于设备的驱动；即使没有实际的通信在进行，驱动也被允许声明一设备不是空闲的。（例如，除非插入该集线器的所有设备都已被挂起，否则该集线器不被视空闲。）此外，只要某个程序保持着它的 usbfs 文件打开，无论是否正在进I/O，该设备都不
被视为空闲
如果一USB 设备没有驱动、它usbfs 文件没有打开、并且它不是通过 sysfs 被访问，那么
它肯定是空闲的
### 动PM 的形

当内核决定挂起一个空闲的设备时，就会发生动态挂起。简言之，这称`autosuspend`（自挂起）。一般来说，除非一个设备已经空闲了某段最短时间（即所谓的空闲延迟时间 idle-delay
time），否则它不会被自动挂起
当然，内核主动做的任何事情都不应该妨碍计算机或其设备的正常工作。如果一个设备已被自挂起，而某个程序试图使用它，内核会自动恢复该设备（autoresume，自动恢复）。出于同样的
原因，如果设备支持远程唤醒，一个被自动挂起的设备通常会启用远程唤醒
值得一提的是，许多 USB 驱动不支autosuspend。事实上，在撰写本文时（Linux 2.6.23），
唯一支持它的驱动是集线器驱动、kaweth、asix、usblp、usblcd 以及 usb-skeleton（后者不
算数）。如果一个不支持的驱动绑定到了一个设备上，该设备就不会被自动挂起。实际上，内假装该设备永远不会空闲
我们可以将电源管理事件分为两大类：外部（external）和内部（internal）。外部事件是USB
栈之外的某个代理触发的：系统挂起/恢复（由用户空间触发）、手动动态恢复（也由用户空间触发以及远程唤醒（由设备触发）。内部事件是USB 栈内部触发的：autosuspend autoresume注意，所有动态挂起事件都是内部的；外部代理不允许发出动态挂起
### 动PM 的用户接

用于控制动PM 的用户接口位于每USB 设备sysfs 目录下的 `power/` 子目录中，也就是
`/sys/bus/usb/devices/.../power/`，其中..”是设备ID。相关的属性文件有：wakeupcontrol `autosuspend_delay_ms`。（也可能有一个名`level` 的文件；该文件自 2.6.35
内核起已被弃用，并被 `control` 文件取代。在 2.6.38 `autosuspend` 文件将被弃用并由
`autosuspend_delay_ms` 文件取代。唯一的区别是，新文件以毫秒表示延迟，而旧文件使用秒令人困惑的是，这两个文件2.6.37 中都存在，但只有 `autosuspend` 起作用。）

	`power/wakeup`

		如果设备不支持远程唤醒，该文件为空。否则，该文件包含单		`enabled` 或单`disabled`，你也可以把这两个单词写入该文件。该设置
		决定了设备下一次被挂起时是否会启用远程唤醒。（如果在设备被挂起		更改了设置，则该更改要到下一次挂起时才会生效。）

	`power/control`

		该文件包`on` `auto` 两个单词之一。你可以把这两个单词写入该文		来更改设备的设置
  - `on` 表示设备应当被恢复，并且不允autosuspend。（当然，系统挂		  仍然被允许。）

  - `auto` 是正常状态，内核可以在该状态下对设备进autosuspend 		  autoresume
		（在 2.6.32 及更早的内核中，你还可以指定 `suspend`，表示设备应当保		挂起且不允许 autoresume。该设置不再被支持。）

	`power/autosuspend_delay_ms`

		该文件包含一个整数值，即设备在由内核自动挂起之前应当保持空闲的毫秒		（空闲延迟时间）。默认值为 2000 表示设备一旦空闲就立即自动挂起		负值表示永远不自动挂起。你可以把一个数字写入该文件来更autosuspend
		的空闲延迟时间
`power/autosuspend_delay_ms` 写入 `-1` 和向 `power/control` 写入 `on` 做的几乎同一件事——它们都阻止设备被自动挂起。是的，这是 API 中的一个冗余
（在 2.6.21 中，`power/autosuspend` 写入 `0` 会阻止设备被自动挂起；该行为2.6.22
中被更改。`power/autosuspend` 属性在 2.6.21 之前不存在，`power/level` 属性在 2.6.22
之前也不存在。`power/control` 是在 2.6.34 加入的，`power/autosuspend_delay_ms` 是在
2.6.37 加入的，但直2.6.38 才变得可用。）

### 更改默认的空闲延迟时

默认autosuspend 空闲延迟时间（以秒为单位）由 usbcore 中的一个模块参数控制。你可以加载 usbcore 时指定该值。例如，要将其设5 秒而不2 秒，你可```

	modprobe usbcore autosuspend=5

```
等价地，你可以把它添加到 /etc/modprobe.d 中的一个配置文件里
```

	options usbcore autosuspend=5

```
某些发行版在启动过程的很早阶段就通过 initramfs 映像中运行的程序或脚本来加载 usbcore
模块。要更改参数值，你将不得不重建该映像
如果 usbcore 是编译进内核而不是作为可加载模块构建```

	usbcore.autosuspend=5

```
加到内核的启动命令行中
最后，该参数值可以在系统运行时更```

	echo 5 >/sys/module/usbcore/parameters/autosuspend

```
那么每个新插入的 USB 设备都会将其 autosuspend 空闲延迟初始化为 5。（已经存在的设备的
空闲延迟值不会受影响。）

将初始默认空闲延迟设-1 将阻止任USB 设备的自动挂起。这样做的好处是，之后你可以选定的设备启autosuspend
### 警告


USB 规范规定，所USB 设备都必须支持电源管理。然而，可悲的事实是许多设备支持得并不好你可以把它们挂起，但当你试图恢复它们时，它们会从 USB 总线上断开连接，或者完全停止工作这似乎在打印机和扫描仪中尤为普遍，但大量其他类型的设备也有同样的缺陷
因此，默认情况下，内核对除集线器以外的所有设备禁autosuspend（`power/control` 属被初始化`on`）。集线器至少在这方面表现得相当规矩
（在 2.6.21 2.6.22 中情况并非如此。当时几乎所USB 设备默认都启用了 autosuspend不少人因此遇到了问题。）

这意味着非集线器设备不会被自动挂起，除非用户或某个程序显式启用它。在撰写本文时，还没任何广泛使用的程序会这样做；我们希望在不久的将来，像 HAL 这样的设备管理器会承担起这份
额外的责任。与此同时，你总是可以手动执行必要的操作，或者把它们添加udev 脚本中。你
也可以更改空闲延迟时间；2 秒并不是每个设备的最佳选择
如果一个驱动知道它的设备具有正确的挂起/恢复支持，它可以自己启用 autosuspend。例如，笔记
本电脑摄像头的视频驱动可能会这样做（在近期的市核中它们确实这样做了），因为这些设备很被使用，因此通常应该被自动挂起
有时会发现，即使一个设备与 autosuspend 配合工作正常，仍然会有问题。例如，管理键盘和鼠usbhid 驱动就支autosuspend。对一些键盘的测试表明，在被挂起的键盘上打字，虽然正确地导致键盘进行远程唤醒，但常常会丢失按键。对鼠标的测试表明，有些鼠标会响应按键按而发出远程唤醒请求，但对移动不会，有些则对两者都不响应
内核不会阻止你在无法处理 autosuspend 的设备上启用它。理论上，在错误的时间挂起设备甚至有
可能损坏设备。（极不可能，但有可能。）务必小心
### 面向电源管理的驱动接

一USB 驱动要支持外部电源管理，需要在`usb_driver` 结构中提```

	.suspend
	.resume
	.reset_resume

```
方法，其`reset_resume` 方法是可选的。这些方法的工作相当简单：

      - `suspend` 方法被调用来警告驱动设备即将被挂起。如果驱动返回一个负的错误码	挂起将被中止。通常驱动会返0，这种情况下它必须取消所有未完成URB
	（`usb_kill_urb`）并且不再提交任URB
      - `resume` 方法被调用来告诉驱动设备已经被恢复，驱动可以恢复正常操作。可以再	提交 URB
      - `reset_resume` 方法被调用来告诉驱动设备已经被恢复，并且它也被重置过。驱动应	重做任何必要的设备初始化，因为设备很可能已经丢失了大部分或全部的（尽管接	会处于与挂起前相同的 altsetting）
如果设备在挂起期间被断开连接或断电，`disconnect` 方法将被调用，而不`resume` `reset_resume` 方法。当从休眠（hibernation）中唤醒时，这也极有可能发生，因为许多系统在
休眠期间不向 USB 主机控制器维持挂起电流。（可以通过使用 USB Persist 机制来绕休眠强制断开连接的问题。）

`reset_resume` 方法USB Persist 机制（参usb-persist）使用，在某`CONFIG_USB_PERSIST`
未启用的情形下也可以使用。目前，如果一个设备在恢复期间被重置，而驱动没`reset_resume`
方法，驱动将不会收到任何关于恢复的通知。更晚的内核会调用驱动的 `disconnect` 方法2.6.23 没有这样做
USB 驱动是绑定到接口上的，因此当接口被挂起或恢复时，它们`suspend` `resume` 方法
会被调用。原则上，人们可能希望挂起设备上的某些接口（即强制这些接口的驱动停止所有活动）
而不挂起其他接口。USB 核心不允许这样做；当设备本身被挂起时，所有接口都被挂起，当设备被
恢复时，所有接口都被恢复。不可能只挂起或恢复设备的一部分接口而不涉及其余接口。你能做最接近的做法是解绑（unbind）这些接口的驱动
### 面向 autosuspend autoresume 的驱动接

要支autosuspend autoresume，一个驱动应当实现上面列出的全部三个方法。此外，驱动通过
在其 usb_driver 结构中设`.supports_autosuspend` 标志来表明它支持 autosuspend。然它负责在其某个接口变忙或变空闲时通知 USB 核心```

	int  usb_autopm_get_interface(struct usb_interface *intf);
	void usb_autopm_put_interface(struct usb_interface *intf);
	int  usb_autopm_get_interface_async(struct usb_interface *intf);
	void usb_autopm_put_interface_async(struct usb_interface *intf);
	void usb_autopm_get_interface_no_resume(struct usb_interface *intf);
	void usb_autopm_put_interface_no_suspend(struct usb_interface *intf);

```
这些函数通过维护 usb_interface 内嵌device 结构中的一个使用计数（usage counter）来
工作。当计数 > 0 时，接口被视为忙，内核将不会自动挂起该接口的设备。当使用计数= 0
时，接口被视为空闲，内核可能会自动挂起该设备
驱动必须小心地平衡它们对使用计数的总体更改。不平衡的“get”在驱动从其接口解绑时仍然会
生效，从而在该接口再次绑定到驱动时阻止设备进入运行时挂起。另一方面，驱动允许通过调用
`usb_autopm_*` 函数来实现这种平衡，即使在其 `disconnect` 例程返回之后——例如从工作队列
例程中——只要它们保持对该接口的一个活动引用（通过 `usb_get_intf` `usb_put_intf`）
使用异步例程的驱动要负责自己的同步与互斥
	`usb_autopm_get_interface` 增加使用计数，并在设备被挂起时执行一autoresume	如果 autoresume 失败，计数会被减回去
	`usb_autopm_put_interface` 减少使用计数，并在新值为 = 0 时尝试一autosuspend
	`usb_autopm_get_interface_async` `usb_autopm_put_interface_async` 做的与非
	异步版本几乎相同。最大的区别在于它们使用工作队列来完成恢复或挂起部分的工作	因此，它们可以在原子上下文（atomic context）中调用，例如在 URB 的完成处理程序中	但当它们返回时，设备通常尚未处于期望的状态
	`usb_autopm_get_interface_no_resume` 鍜?`usb_autopm_put_interface_no_suspend`
	仅仅增加或减少使用计数；它们不尝试执autoresume autosuspend。因此它们可	在原子上下文中调用
最简单的使用模式是：驱动在其 open 例程中调`usb_autopm_get_interface`，并在其 close release 例程中调`usb_autopm_put_interface`。但也可能有其他模式
上面提到autosuspend 尝试常常会因为这样或那样的原因失败。例如，`power/control` 属可能被设`on`，或者同一设备中的另一个接口可能不是空闲的。这完全正常。如果失败的原因
是设备空闲时间还不够长，则会安排一个定时器，在 autosuspend 空闲延迟到期时自动执行该操作
autoresume 尝试也可能失败，尽管失败意味着设备不再存在或工作不正常。与 autosuspend 不同autoresume 没有空闲延迟
### 驱动接口的其他部

```

	usb_enable_autosuspend(struct usb_device *udev);

```
在其 `probe` 例程中，如果它们知道设备能够正确挂起和恢复。这完全等同于向设备`power/control` 属性写`auto`。类似地```

	usb_disable_autosuspend(struct usb_device *udev);

```
这完全等同于`power/control` 属性写`on`
有时驱动需要确保在 autosuspend 期间启用了远程唤醒。例如，如果用户不能通过打字来让键盘
进行远程唤醒，那么自动挂起一个键盘就没有多大意义。如果驱动将 `intf->needs_remote_wakeup`
设为 1，那么当远程唤醒不可用时，内核将不会自动挂起该设备。（不过，如果设备已经被自动
挂起了，设置此标志不会导致内核对其进autoresume。通常驱动会在`probe` 方法中设置此
标志，此时设备保证不会被自动挂起。）

如果驱动在中断上下文中异步进I/O，它应该在开始输出前调用
`usb_autopm_get_interface_async`，并在输出队列排空时调用 `usb_autopm_put_interface_async````

	usb_mark_last_busy(struct usb_device *udev);

```
在事件处理程序中。这告诉 PM 核心设备刚刚忙过，因此下一autosuspend 空闲延迟的到期应被推迟。许usb_autopm_* 例程也会进行这个调用，所以驱动只需要在有中断驱动的输入到达操心这一点
异步操作总是会遇到竞争。例如，驱动可能在核心刚刚判定设备已经空闲了足够长的时间、但还没
来得及调用驱动的 `suspend` 方法时，调用 `usb_autopm_get_interface_async` 例程。`suspend`
方法必须负责I/O 请求例程URB 完成处理程序同步；如果驱动需要使用该设备，它应该autosuspend -EBUSY 失败
外部挂起调用绝不应该以这种方式被允许失败，只autosuspend 调用可以。驱动可以通过对传`suspend` 方法message 参数应用 `PMSG_IS_AUTO` 宏来区分它们；对于内PM 事件（autosuspend它会返回 True，对于外PM 事件会返False
### 互斥


对于外部事件——但不一定对autosuspend autoresume——在调用 `suspend` `resume`
方法时，会持有设备信号量（udev->dev.sem）。这意味着外部挂起/恢复事件与对 `probe``disconnect`、`pre_reset` `post_reset` 的调用是互斥的；USB 核心保证对于 autosuspend/
autoresume 事件也是如此
如果驱动想在某个临界区期间阻止所有挂恢复调用，最好的办法是锁定设备并调用
`usb_autopm_get_interface`（并在临界区结束时做相反的操作）。持有设备信号量会阻止所有外PM 调用，`usb_autopm_get_interface` 会阻止任何内PM 调用，即使它失败了。（练习：为什么？
### 动PM 与系PM 之间的交

动态电源管理和系统电源管理可以通过几种方式交互
首先，当系统挂起发生时，一个设备可能已经被自动挂起了。由于系统挂起应该尽可能透明，设在系统恢复后应该保持挂起。但这个理论在实践中可能并不顺利；随着时间的推移，内核在这方面行为已经发生了变化。从 2.6.37 起，策略是在系统恢复期间恢复所有设备，并让它们在之后处自己的运行时挂起
其次，动态电源管理事件可能在系统挂起进行期间发生。这个窗口很短，因为系统挂起不会花很
长时间（通常只有几秒），但它确实可能发生。例如，一个被挂起的设备可能在系统正在挂起发送一个远程唤醒信号。远程唤醒可能成功，从而导致系统挂起被中止。如果远程唤醒不成功，它
可能仍然保持活动，从而在系统挂起一完成就导致系统恢复。或者远程唤醒可能失败并被丢失发生哪种结果取决于时序以及硬件和固件的设计
### xHCI 硬件链路 PM


xHCI 主机控制器为支持链路 PM usb2.0（xHCI 1.0 特性）usb3.0 设备提供硬件链路电源
管理。通过启用硬件 LPM，主机可以自动将设备置于更低功耗的状态（usb2.0 设备L1，usb3.0
设备U1/U2），该状态下设备可以非常快地进入和恢复
用于控制硬件 LPM 的用户接口位于每USB 设备sysfs 目录下的 `power/` 子目录中，也就是
`/sys/bus/usb/devices/.../power/`，其中..”是设备ID。相关的属性文件是
`usb2_hardware_lpm` 鍜?`usb3_hardware_lpm`銆?
	`power/usb2_hardware_lpm`

		当一个支LPM USB2 设备被插到支持软LPM xHCI 主机根集线器上时		主机会为它运行一个软LPM 测试；如果设备成功进L1 状态并恢复，且主机
		支持 USB2 硬件 LPM，该文件就会出现，驱动会为该设备启用硬件 LPM。你可以
		向该文件写入 y/Y/1 n/N/0 来手动启禁用 USB2 硬件 LPM。这主要出于
		测试目的
	`power/usb3_hardware_lpm_u1`
	`power/usb3_hardware_lpm_u2`

		当一个支lpm USB 3.0 设备被插到支持链PM xHCI 主机上时，它会检		U1 U2 退出延迟是否已BOS 描述符中设置；如果检查通过且主机支USB3
		硬件 LPM，则会为该设备启USB3 硬件 LPM 并创建这些文件。这些文件保存一		字符串值（enable disable），指示是否为该设备启用USB3 硬件 LPM 		U1 U2
### USB 端口电源控制


除了挂起端点设备和启用硬件控制的链路电源管理之外，USB 子系统还能够在一些条件下关闭端口
的电源。电源通过向集线器发`Set/ClearPortFeature(PORT_POWER)` 请求来控制。对于根集线或平台内部集线器，主机控制器驱动`PORT_POWER` 请求转换为平台固件（ACPI）方法调用来设置
端口电源状态。更多背景请参见 2012 Linux Plumbers Conference 的幻灯片 [#f1]_ 和视[#f2]_
当收到一`ClearPortFeature(PORT_POWER)` 请求时，一USB 端口在逻辑上被关闭，并可能触发
该端VBUS 的实际丢[#f3]_。在集线器将多个端口组成一个共享电源域（power well）的情况下，
VBUS 可能会保持，导致在该电源域中的所有端口都关闭之前电源一直保持。VBUS 也可能由配置充电应用的集线器端口保持。无论如何，一个逻辑上关闭的端口会与其设备失去连接，不响应热插拔
事件，也不响应远程唤醒事件

   关闭一个端口可能导致无法热添加设备。详情请参见“端口电源控制的用户接口”
就其对设备本身的影响而言，它类似于设备在系统挂起期间所经历的过程，即电源会话（power
session）丢失。任何在系统挂起时表现异常的 USB 设备或驱动，也会同样受到端口电源周期事件影响。因此，该实现共享了与系统恢复路径相同的设备恢复路径（并且遵循相同的怪异设备处理
quirks），用于该集线器

  http://dl.dropbox.com/u/96820575/sarah-sharp-lpt-port-power-off2-mini.pdf


  http://linuxplumbers.ubicast.tv/videos/usb-port-power-off-kerneluserspace-api/


  USB 3.1 Section 10.12

  wakeup note: 如果一个设备被配置为发送唤醒事件，端口电源控制实现会阻止对该端口进行断  尝试
### 端口电源控制的用户接

端口电源控制机制使用 PM 运行时系统。通过清除端口设备`power/pm_qos_no_power_off` 标志
（默认为 1）来请求断电。如果该端口已断开连接，它会立即收到一`ClearPortFeature(PORT_POWER)`
请求。否则，它会遵循 pm 运行时规则，并要求所连接的子设备及其所有后代都被挂起。该机制
依赖于集线器在其集线器描述符中通告了端口电源切换（wHubCharacteristics 逻辑电源切换模式
字段）
注意，某些接口设驱动不支autosuspend。在 `usb_device` 挂起之前，用户空间可能需解绑这些接口驱动。一个未绑定的接口设备默认是被挂起的。解绑时，要小心解绑接口驱动，而不usb 设备的驱动。同时，保留集线器接口驱动为绑定状态。如usb 设备（非接口）的驱动解绑，内核将无法再恢复该设备。如果集线器接口驱动被解绑，对其子端口的控制将丢失，且所连接的子设备都会断开。一个好的经验法则是：如果一个设备的 'driver/module' 链接指向
`/sys/module/usbcore`，那么解绑它会干扰端口电源控制
端口电源控制相关文件的示例。注意，在此示例```

     prefix=/sys/devices/pci0000:00/0000:00:14.0/usb3/3-1

                      attached child device +
                  hub port device +         |
     hub interface device +       |         |
                          v       v         v
                  $prefix/3-1:1.0/3-1-port1/device

     $prefix/3-1:1.0/3-1-port1/power/pm_qos_no_power_off
     $prefix/3-1:1.0/3-1-port1/device/power/control
     $prefix/3-1:1.0/3-1-port1/device/3-1.1:<intf0>/driver/unbind
     $prefix/3-1:1.0/3-1-port1/device/3-1.1:<intf1>/driver/unbind
     ...
     $prefix/3-1:1.0/3-1-port1/device/3-1.1:<intfN>/driver/unbind

```
除了这些文件，某些端口可能有一个指向另一个集线器上某个端口的 'peer' 链接。预期是，所超速（superspeed）端口都有一```

  $prefix/3-1:1.0/3-1-port1/peer -> ../../../../usb2/2-1/2-1:1.0/2-1-port1
  ../../../../usb2/2-1/2-1:1.0/2-1-port1/peer -> ../../../../usb3/3-1/3-1:1.0/3-1-port1

```
与“companion ports”（伴随端口）或“ehci/xhci 共享切换端口”不同，peer 端口只是被组合到
单个 usb3 连接器中的高速（hi-speed）和超速（superspeed）接口引脚。peer 端口共享相同祖先 XHCI 设备
当一个超速端口断电时，设备可能降级其连接并尝试连接到高速引脚。实现采取了措施来防止这情况
1. 端口挂起被排序，以保证高速端口在其超peer 被允许断电之前先断电。这意味着，在高   peer 进入其运行时挂起状态之前，在超速端口上`pm_qos_no_power_off` 设为零可能不会导   该端口断电。如果用户空间想保证一个超速端口会断电，必须注意排序这些挂起操作
2. 端口恢复被排序，以强制超速端口在其高peer 之前上电
3. 端口恢复总是触发所连接的子设备恢复。在电源会话丢失之后，设备可能已被移除，或需要重置   当父端口重新获得电力时恢复子设备，解决了这些状态，并将最大端口电源周期频率限制在子设   能够挂起（autosuspend-delay）和恢复（reset-resume 延迟）的速率上
与端口电源控制相关的 sysfs 文件
	`<hubdev-portX>/power/pm_qos_no_power_off`		这个可写标志控制一个空闲端口的状态。一旦所有子设备和后代都被挂起，只要
		pm_qos_no_power_off '0'，端口就可以挂起/断电。如pm_qos_no_power_off
		'1'，无论后代的状态如何，端口都会保持活动/通电。默认为 1
	`<hubdev-portX>/power/runtime_status`		该文件反映端口是 'active'（通电）还'suspended'（逻辑上关闭）。对		用户空间来说，没有任何指示表VBUS 是否仍然被提供
	`<hubdev-portX>/connect_type`		一个给用户空间的建议性只读标志，指示端口的位置和连接类型。它返回四个		之一hotplug'hardwired'not used' 'unknown'。除 unknown 外，
		所有值都由平台固件设置
		`hotplug` 表示平台上外部可连接/可见的端口。通常用户空间会选择保持此类
		端口通电，以处理新设备连接事件
		`hardwired` 指的是不可见但可连接的端口。例子包括可以通过外部开关断开		内部 USB 蓝牙端口，或带有硬连USB 摄像头的端口。只pm_qos_no_power_off
		与任何控制连接的开关协调好，允许这些端口挂起应该是安全的。用户空间必		安排在端口断电之前设备已被连接，或者在通过开关启用连接之前激活该端口
		`not used` 指的是预期永远不会连接设备的内部端口。这些可能是空的内部端口		或者是平台上物理上没有暴露出来的端口。被认为可以随时安全断电
		`unknown` 表示平台固件没有提供该端口的信息。最常见是指外部集线器端口，		策略决策中应将它们视'hotplug'
```

			- since we are relying on the BIOS to get this ACPI
			  information correct, the USB port descriptions may
			  be missing or wrong.

			- Take care in clearing ``pm_qos_no_power_off``. Once
			  power is off this port will
			  not respond to new connect events.

	一旦有子设备连接，在允许端口断电之前会施加额外的约束
	``<child>/power/control``		必须``auto``，并且端口在 ``<child>/power/runtime_status``
		反映 'suspended' 状态之前不会断电。默认值由子设备驱动控制
	``<child>/power/persist``		对于大多数设备默认为 ``1``，表示内核是否可以在电源会话丢失（挂/ 端口
		断电事件）期间保持设备的配置。当该值为 ``0``（怪异设备）时，端口断电被
		禁用
	``<child>/driver/unbind``		具有唤醒能力的设备会阻止端口断电。目前，清除一个接口设备的 usb 内部唤醒
		能力的唯一机制是解绑它的驱动
```
```

	echo 0 > power/pm_qos_no_power_off
	echo 0 > peer/power/pm_qos_no_power_off # if it exists
	echo auto > power/control # this is the default value
	echo auto > <child>/power/control
	echo 1 > <child>/power/persist # this is the default value

```
### 建议的用户空间端口电源策

如上所述，用户空间需要谨慎而有意地决定启用哪些端口断电
默认配置是所有端口都`power/pm_qos_no_power_off` 设为 `1` 启动，导致端口始终保持活动
如果对平台固件的端口描述有信心（端口ACPI _PLD 记录填好'connect_type'），用户空间
可以清除所'not used' 端口pm_qos_no_power_off。对'hardwired' 端口也可以这样做只要断电与该端口的任何连接开关协调好
一种更激进的用户空间策略是：当某些外部因素表明用户已停止与系统交互时，为所有端口启USB
端口断电（将 `<hubdev-portX>/power/pm_qos_no_power_off` 设为 `0`）。例如，某个发行版可希望在屏幕熄灭时启用所USB 端口断电，并在屏幕重新激活时重新通电。智能手机和平板电脑可能
希望在用户按下电源按钮时关闭 USB 端口