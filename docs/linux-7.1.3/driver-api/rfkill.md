## rfkill - RF 开关（kill switch）支持


   :depth: 2

## 简介


rfkill 子系统提供了一个通用接口，用于禁用系统中任何无线
发射机。当发射机被屏蔽（blocked）时，它不应辐射任何功率。

该子系统还提供对按键按下的响应能力，并禁用某一类型（或所有）
的发射机。这适用于需要关闭发射机的情形，例如在飞机上。

rfkill 子系统有“硬”（hard）和“软”（soft）屏蔽的概念，它们的
含义差别很小（屏蔽 == 发射机关闭），差别在于是否可以被改变：

 - 硬屏蔽（hard block）
	只读的无线电屏蔽，无法被软件覆盖

 - 软屏蔽（soft block）
	可写的无线电屏蔽（不必可读），由系统软件设置。

rfkill 子系统有两个参数，rfkill.default_state 和
rfkill.master_switch_mode，它们在
admin-guide/kernel-parameters.rst 中有文档说明。

## 实现细节


rfkill 子系统由三个主要组件组成：

 - rfkill 核心（core），
 - 已弃用的 rfkill-input 模块（一个输入层处理程序，正被
   用户空间策略代码取代），以及
 - rfkill 驱动。

rfkill 核心为内核驱动提供了 API，用于将它们的无线电
发射机注册到内核、打开和关闭它的方法，并让系统知道
可能在设备上实现的硬件禁用状态。

rfkill 核心代码还会通知用户空间状态变化，并提供
用户空间查询当前状态的方式。请参见下面的“用户空间支持”一节。

当设备被硬屏蔽时（无论是通过调用 rfkill_set_hw_state()
还是来自 query_hw_block），set_block() 将被调用以进行额外的软件
屏蔽，但驱动可以忽略该方法调用，因为它们可以使用函数
rfkill_set_hw_state() 的返回值来同步软件状态，而无需跟踪对
set_block() 的调用。实际上，除非硬件确实分别跟踪软屏蔽和硬屏蔽，
否则驱动应该使用 rfkill_set_hw_state() 的返回值。

## 内核 API


无线发射机的驱动通常实现一个 rfkill 驱动。

如果 rfkill 按钮仅仅是一个按钮，平台驱动可能实现输入设备。
如果该按钮影响硬件，那么你需要改为实现一个 rfkill 驱动。这同样
适用于平台提供打开/关闭发射机的方式的情况。

对于某些平台，硬件状态可能在挂起/休眠期间发生变化，在这种情况下，
需要在恢复时以当前状态更新 rfkill 核心。

```

	depends on RFKILL || !RFKILL

```
以确保当 rfkill 为模块时，该驱动不能被内建（built-in）。!RFKILL
的情况允许在 rfkill 未配置时构建该驱动，此时所有 rfkill API 仍可
使用，但将由编译后几乎不产生任何代码的静态内联（static inline）函数提供。

当状态发生变化时，控制可被硬屏蔽的设备的 rfkill 驱动必须调用
rfkill_set_hw_state()，除非它们也分配了 poll_hw_block() 回调
（那样 rfkill 核心将轮询设备）。除非你无法以任何其他方式获取事件，
否则不要这样做。

rfkill 提供每个开关的 LED 触发器，可用于根据开关状态驱动 LED
（屏蔽时为 LED_FULL，否则为 LED_OFF）。

## 用户空间支持


推荐使用的用户空间接口是 /dev/rfkill，这是一个杂项（misc）
字符设备，允许用户空间获取并设置 rfkill 设备与设备集合的状态。
它还会通知用户空间设备的添加与移除。该 API 是一个简单的读/写 API，
定义在 linux/rfkill.h 中，带有一个 ioctl，用于在过渡期内关闭内核中
已弃用的输入处理程序。

除那一个 ioctl 之外，与内核的通信通过读写 'struct rfkill_event'
的实例来完成。在该结构中，软屏蔽和硬屏蔽被正确地分开（与 sysfs 不同，
见下文），并且用户空间能够获取系统中所有 rfkill 设备的一致快照。
此外，还可以将所有 rfkill 驱动（或某一指定类型的所有驱动）切换到一个
同时更新热插拔设备默认状态的状态。

应用程序打开 /dev/rfkill 后，可以读取所有设备的当前状态。可以通过
轮询描述符以获取热插拔或状态变化事件，或者监听 rfkill 核心框架
发出的 uevent 来获取变化。

此外，每个 rfkill 设备都在 sysfs 中注册并发出 uevent。

rfkill 设备发出 uevent（动作为 "change"），带有以下内容
```

	RFKILL_NAME
	RFKILL_STATE
	RFKILL_TYPE

```
这些变量的内容对应于上面解释的 "name"、"state" 和 "type" sysfs 文件。

更多细节请查阅 Documentation/ABI/stable/sysfs-class-rfkill。
