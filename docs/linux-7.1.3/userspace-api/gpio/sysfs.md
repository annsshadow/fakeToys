## 面向用户空间GPIO Sysfs 接口


   API 已被 chardev.rst 废弃，其 ABI 文档已移Documentation/ABI/obsolete/sysfs-gpio
   新的开发应使用 chardev.rst，已有的开发也鼓励尽快迁移，因为该 API 未来将被移除
   在迁移期间该接口将继续得到维护，但新特性只会添加到API 中
### 废弃sysfs ABI


使用“gpiolib”实现者框架的平台可以选择配置一个面GPIO sysfs 用户接口。这debugfs 接口不同，因为它提供GPIO 方向和值的控制，而不只是显示 GPIO 状态摘要此外，它可以在没有调试支持的生产系统上存在
在拥有适当的系统硬件文档的情况下，用户空间可以例如知道 GPIO #23 控制用于保护闪存
中引导加载程序段的写保护线。系统升级过程可能需要临时移除该保护：先导入一GPIO再改变其输出状态，然后在重新启用写保护之前更新代码。在正常使用中，GPIO #23 永远
不应被触碰，内核也无需知道它
同样取决于适当的硬件文档，在某些系统上，用户空GPIO 可用于确定标准内核不会知的系统配置数据。对于某些任务，简单的用户空间 GPIO 驱动可能就是系统真正需要的全部
   不要sysfs 去控制已proper 内核驱动的硬件   请阅Documentation/driver-api/gpio/drivers-on-gpio.rst，以避免在用户空间重   发明内核已有的轮子
   I MEAN IT. REALLY.

### Sysfs 中的路径


/sys/class/gpio 中有三种条目
   - 用于获取用户GPIO 控制权的控制接口
   - GPIO 本身；以
   - GPIO 控制器（“gpio_chip实例）
此外还有标准文件，包“device符号链接
控制接口是只写的
    /sys/class/gpio/

	"export" ...
		用户空间可以通过向该文件写入 GPIO 编号，请求内核将某个 GPIO 的控制权
		导出给用户空间
		示例：“echo 19 > export将为 GPIO #19 创建一“gpio19节点		前提是该 GPIO 未被内核代码请求
	"unexport" ...
		撤销导出到用户空间的效果
		示例：“echo 19 > unexport将移除使“export文件导出“gpio19		节点
GPIO 信号的路径形/sys/class/gpio/gpio42/（对GPIO #42），并具有如下可可写
属性：

    /sys/class/gpio/gpioN/

	"direction" ...
		读取“in“out”。该值通常可被写入。写“out默认将初始值初始化
		为低电平。为确保无毛刺（glitch free）操作，可以写入 “low“high		将该 GPIO 配置为具有该初始值的输出
		注意，如果内核不支持改变 GPIO 方向，或者它是由未显式允许用户空间重新配		GPIO 方向的内核代码导出的，则此属*不会存在**
	"value" ...
		读取0（无非激活）1（激活）。如GPIO 被配置为输出，则可以写入
		此值；任何非零值都被视为激活
		如果该引脚可配置为产生中断，并且已配置为产生中断（参“edge的描述）		你可以对该文件执poll(2)，且每当中断被触发时 poll(2) 就会返回。如果你
		使用 poll(2)，请设置事件 POLLPRI POLLERR。如果你使用 select(2)，请		文件描述符置exceptfds 中。在 poll(2) 返回后，使用 pread(2) 在偏移零		读取该值。或者，sysfs 文件执行 lseek(2) 到开头并读取新值，或者关闭文		再重新打开以读取该值
	"edge" ...
		读取“none”、“rising”、“falling“both”。写入这些字符串以选择将使
		“value文件上的 poll(2) 返回的（一个或多个）信号边沿
		仅当该引脚可配置为中断产生的输入引脚时，此文件才存在
	"active_low" ...
		读取0（false）或 1（true）。写入任何非零值以反转 value 属性的读取		写入语义。已有的以及后续的、通过 edge 属性为 “rising“falling边沿
		配置poll(2) 支持都将遵循此设置
GPIO 控制器的路径形如 /sys/class/gpio/gpiochip42/（对应实现从 #42 开始的 GPIO 控制器），并具有如下只读属性：

    /sys/class/gpio/gpiochipN/

	"base" ...
		N 相同，即该芯片管理的第一GPIO

	"label" ...
		用于诊断（不保证唯一
	"ngpio" ...
		该芯片管理多少个 GPIO（N N + ngpio - 1
板级文档在大多数情况下应涵盖 GPIO 用于什么目的。然而，这些编号并非总是稳定；子（daughtercard）上GPIO 可能因所使用的基板或其它堆叠的卡而不同。在这种情况下，可能需要使gpiochip 节点（可能结合原理图）来确定用于给定信号的正GPIO 编号

### 从内核代码导

内核代码可以显式管理已通过
```

	/* export the GPIO to userspace */
	int gpiod_export(struct gpio_desc *desc, bool direction_may_change);

	/* reverse gpiod_export() */
	void gpiod_unexport(struct gpio_desc *desc);

	/* create a sysfs link to an exported GPIO node */
	int gpiod_export_link(struct device *dev, const char *name,
		      struct gpio_desc *desc);

```
在内核驱动请求一GPIO 之后，它只能通过 gpiod_export() sysfs 接口中变得可用驱动可以控制信号方向是否可以改变。这有助于驱动防止用户空间代码意外破坏重要的系统
状态
这种显式导出有助于调试（使某些实验更容易），或者可以提供一个始终存在的接口，适合
作为板级支持包（BSP）的一部分来记录
GPIO 被导出之后，gpiod_export_link() 允许sysfs 的其它位置创建指向该 GPIO sysfs
节点的符号链接。驱动可以用它来sysfs 中自己的设备下提供该接口，并配以描述性名称