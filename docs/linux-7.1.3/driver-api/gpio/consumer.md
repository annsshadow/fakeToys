## GPIO 描述符消费者接口


本文档描述 GPIO 框架的消费者（consumer）接口。


## GPIO 消费者指南


无法在标准 GPIO 调用缺失的情况下工作的驱动，应当具有依赖（depend on）GPIOLIB 或
选择（select）GPIOLIB 的 Kconfig 条目。允许驱动使用 GPIO 的函数都声明在
<linux/gpio/consumer.h> 头文件中：

```
	#include <linux/gpio/consumer.h>
```

在 GPIOLIB 被禁用的情况下，头文件中为所有函数提供了静态内联桩（stub）函数。调用
这些桩函数时会发出警告。这些桩函数用于两种用例：

- 简单的编译覆盖（compile coverage），例如使用 COMPILE_TEST——当前平台是否启用或
  选择 GPIOLIB 并不重要，因为我们本来就不打算运行该系统。

- 真正可选的 GPIOLIB 支持——驱动在某些编译时配置下的某些系统中并不真正使用
  GPIO，但在其它编译时配置下会使用。在这种情况下，消费者必须确保不调用这些函数，
  否则用户会遇到可能令人不安的控制台警告。将真正可选的 GPIOLIB 使用与对
  `[devm_]gpiod_get_optional()` 的调用结合起来是一个**糟糕的主意**，并会导致奇怪
  的错误消息。请对可选的 GPIOLIB 使用普通的 getter 函数：这样做时应当预期需要一些
  手动编写的错误处理的代码。

所有使用基于描述符的 GPIO 接口的函数都以 `gpiod_` 为前缀。`gpio_` 前缀用于遗留
（legacy）接口。内核中其它函数不应使用这些前缀。强烈不建议使用遗留函数，新代码应当
仅使用 <linux/gpio/consumer.h> 和描述符。


## 获取与释放 GPIO


在基于描述符的接口中，GPIO 通过不透明（opaque）、不可伪造（non-forgeable）的句柄
（handler）来标识，该句柄必须通过对某个 gpiod_get() 函数的调用来获取。与许多其它
内核子系统一样，gpiod_get() 接收将使用该 GPIO 的设备，以及所请求 GPIO 应当
对应的功能：

```
	struct gpio_desc *gpiod_get(struct device *dev, const char *con_id,
				    enum gpiod_flags flags)
```

如果某个功能是通过将多个 GPIO 一起使用来实现的（例如一个简单的 LED），则使用：

```
	struct gpio_desc *gpiod_get_index(struct device *dev,
					  const char *con_id, unsigned int idx,
					  enum gpiod_flags flags)
```

关于在 DeviceTree 情况下 con_id 参数的更详细描述，请参见
Documentation/driver-api/gpio/board.rst

flags 参数用于可选地指定 GPIO 的方向和初始值。其值可以为：

- GPIOD_ASIS 或 0：完全不初始化该 GPIO。方向必须稍后通过某个专用函数设置。
- GPIOD_IN：将该 GPIO 初始化为输入。
- GPIOD_OUT_LOW：将该 GPIO 初始化为输出，值为 0。
- GPIOD_OUT_HIGH：将该 GPIO 初始化为输出，值为 1。
- GPIOD_OUT_LOW_OPEN_DRAIN：与 GPIOD_OUT_LOW 相同，但还强制该线路以开漏
  （open drain）方式电气使用。
- GPIOD_OUT_HIGH_OPEN_DRAIN：与 GPIOD_OUT_HIGH 相同，但还强制该线路以开漏方式
  电气使用。

注意，初始值是**逻辑**（logical）值，物理线路电平取决于该线路被配置为低有效
（active low）还是高有效（active high）（见 active_low_semantics）。

最后两个标志用于开漏是必需的使用场景，例如 I2C：如果该线路在映射（见 board.rst）
中尚未被配置为开漏，那么无论如何都会强制使用开漏，并打印一条警告，提示需要更新板级
配置以匹配该使用场景。

两个函数都返回一个有效的 GPIO 描述符，或者一个可用 IS_ERR() 检查的错误码（它们
永远不会返回 NULL 指针）。当且仅当没有 GPIO 被分配给该设备/功能/索引三元组时，
会返回 -ENOENT；其它错误码用于已经分配了 GPIO 但获取它时发生错误的情况。这对于区分
普通错误和可选 GPIO 参数缺失 GPIO 很有用。对于 GPIO 可选的常见模式，可以使用
gpiod_get_optional() 和 gpiod_get_index_optional() 函数。这些函数在没有 GPIO 时
返回 NULL：

```
	struct gpio_desc *gpiod_get_optional(struct device *dev,
					     const char *con_id,
					     enum gpiod_flags flags)

	struct gpio_desc *gpiod_get_index_optional(struct device *dev,
						   const char *con_id,
						   unsigned int index,
						   enum gpiod_flags flags)
```

注意，gpio_get*_optional() 函数（及其托管变体）与 gpiolib API 的其余部分不同，在
gpiolib 支持被禁用时也会返回 NULL。这对驱动作者很有帮助，因为他们不需要特判
-ENOSYS 返回码。不过系统集成员应当小心，在需要 gpiolib 的系统上启用它。

```
	struct gpio_descs *gpiod_get_array(struct device *dev,
					   const char *con_id,
					   enum gpiod_flags flags)
```

该函数返回一个包含描述符数组的 struct gpio_descs。它还包含一个指向 gpiolib
私有结构的指针，该结构：

```
	struct gpio_descs {
		struct gpio_array *info;
		unsigned int ndescs;
		struct gpio_desc *desc[];
	}
```

如果没有分配任何 GPIO，以下函数返回 NULL 而非 -ENOENT：

```
	struct gpio_descs *gpiod_get_array_optional(struct device *dev,
						    const char *con_id,
						    enum gpiod_flags flags)
```

```
	struct gpio_desc *devm_gpiod_get(struct device *dev, const char *con_id,
					 enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_index(struct device *dev,
					       const char *con_id,
					       unsigned int idx,
					       enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_optional(struct device *dev,
						  const char *con_id,
						  enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_index_optional(struct device *dev,
							const char *con_id,
							unsigned int index,
							enum gpiod_flags flags)

	struct gpio_descs *devm_gpiod_get_array(struct device *dev,
						const char *con_id,
						enum gpiod_flags flags)

	struct gpio_descs *devm_gpiod_get_array_optional(struct device *dev,
							 const char *con_id,
							 enum gpiod_flags flags)
```

```
	void gpiod_put(struct gpio_desc *desc)
```

```
	void gpiod_put_array(struct gpio_descs *descs)
```

在调用这些函数之后使用描述符是严格禁止的。也不允许从通过 gpiod_get_array() 获取的
数组中单独释放描述符（使用 gpiod_put()）。

```
	void devm_gpiod_put(struct device *dev, struct gpio_desc *desc)

	void devm_gpiod_put_array(struct device *dev, struct gpio_descs *descs)
```

## 使用 GPIO


### 设置方向


驱动对 GPIO 要做的第一件事是设置其方向。如果没有给定任何方向设置标志给
gpiod_get*()，则由以下函数完成：

```
	int gpiod_direction_input(struct gpio_desc *desc)
	int gpiod_direction_output(struct gpio_desc *desc, int value)
```

返回值为零表示成功，否则为负的 errno。应当检查该返回值，因为 get/set 调用不会返回
错误，且可能发生错误配置。你通常应当在任务（task）上下文中发出这些调用。然而，对于
自旋锁安全的（spinlock-safe）GPIO，在任务启用之前、作为早期板级初始化的一部分使用
它们是没问题的。

对于输出 GPIO，所提供的值成为初始输出值。这有助于避免系统启动期间的信号毛刺
（glitch）。

```
	int gpiod_get_direction(const struct gpio_desc *desc)
```

该函数返回 0 表示输出，1 表示输入；出错时返回错误码。

请注意，GPIO 没有默认方向。因此，**在不先设置其方向的情况下使用 GPIO 是非法行为，
并将导致未定义的行为！**


### 自旋锁安全的 GPIO 访问


大多数 GPIO 控制器可以通过内存读/写指令访问。这些不需要睡眠，并且可以安全地从硬
（非线程化）IRQ 处理程序及类似上下文中进行。

```
	int gpiod_get_value(const struct gpio_desc *desc);
	void gpiod_set_value(struct gpio_desc *desc, int value);
```

这些值是布尔值，零表示非激活（inactive），非零表示激活（active）。读取输出引脚的
值时，返回的值应当是引脚上所看到的值。由于包括开漏信号（open-drain signaling）和
输出延迟在内的各种问题，这并不总是与指定的输出值相匹配。

get/set 调用不会返回错误，因为 “无效 GPIO” 应当早已由 gpiod_direction_*() 报告。
然而请注意，并非所有平台都能读取输出引脚的值；那些不能读取的平台应当始终返回零。
此外，对这些在没有睡眠的情况下无法安全访问的 GPIO（见下文）使用这些调用是一种错误。


### 可能睡眠的 GPIO 访问


有些 GPIO 控制器必须使用基于消息的总线（如 I2C 或 SPI）来访问。读取或写入这些 GPIO
值的命令需要等待排到队列头部以发送命令并获取其响应。这需要睡眠，而这是无法在 IRQ
处理程序内部完成的。

支持此类 GPIO 的平台通过以下方式将它们与其它 GPIO 区分开来：

```
	int gpiod_cansleep(const struct gpio_desc *desc)
```

```
	int gpiod_get_value_cansleep(const struct gpio_desc *desc)
	void gpiod_set_value_cansleep(struct gpio_desc *desc, int value)
```

访问此类 GPIO 需要一个可能睡眠的上下文，例如线程化 IRQ 处理程序，并且必须使用这些
访问器，而不是不带 cansleep() 后缀的自旋锁安全访问器。

除了这些访问器可能睡眠、且能工作于无法从硬 IRQ 处理程序访问的 GPIO 之外，这些调用
的行为与自旋锁安全的调用相同。


### 低有效与开漏语义


由于消费者不应关心物理线路电平，所有的 gpiod_set_value_xxx() 或
gpiod_set_array_value_xxx() 函数都以**逻辑**（logical）值进行操作。由此它们会考虑
低有效（active low）属性。这意味着它们会检查该 GPIO 是否被配置为低有效，如果是，则
在驱动物理线路电平之前对传入的值进行相应处理。

这同样适用于开漏或开源（open source）输出线路：它们不会主动驱动输出为高（开漏）或
为低（开源），而只是将其输出切换为高阻（high impedance）值。消费者不应需要关心这点。
（细节请阅读 driver.rst 中关于开漏的内容。）

由此，所有 gpiod_set_(array)_value_xxx() 函数将参数 “value” 解释为 “激活”（“1”）
或 “非激活”（“0”）。物理线路电平将被相应地驱动。

例如，如果为某个专用 GPIO 设置了低有效属性，而 gpiod_set_(array)_value_xxx() 传入
“激活”（“1”），则物理线路电平将被驱动为低。

```
  Function (example)                 line property          physical line
  gpiod_set_raw_value(desc, 0);      don't care             low
  gpiod_set_raw_value(desc, 1);      don't care             high
  gpiod_set_value(desc, 0);          default (active high)  low
  gpiod_set_value(desc, 1);          default (active high)  high
  gpiod_set_value(desc, 0);          active low             high
  gpiod_set_value(desc, 1);          active low             low
  gpiod_set_value(desc, 0);          open drain             low
  gpiod_set_value(desc, 1);          open drain             high impedance
  gpiod_set_value(desc, 0);          open source            high impedance
  gpiod_set_value(desc, 1);          open source            high
```

可以使用 set_raw/get_raw 函数来覆盖这些语义，但应当尽可能避免，尤其是与系统无关的
驱动，它们不应需要关心实际的物理线路电平，而应关注逻辑值。


### 访问原始 GPIO 值


存在这样的消费者：它们需要管理 GPIO 线路的逻辑状态，即其设备实际将接收到的值，无论
其与该 GPIO 线路之间隔着什么。

以下这组调用会忽略 GPIO 的低有效或开漏属性：

```
	int gpiod_get_raw_value(const struct gpio_desc *desc)
	void gpiod_set_raw_value(struct gpio_desc *desc, int value)
	int gpiod_get_raw_value_cansleep(const struct gpio_desc *desc)
	void gpiod_set_raw_value_cansleep(struct gpio_desc *desc, int value)
	int gpiod_direction_output_raw(struct gpio_desc *desc, int value)
```

GPIO 的低有效状态也可以使用以下函数查询和切换：

```
	int gpiod_is_active_low(const struct gpio_desc *desc)
	void gpiod_toggle_active_low(struct gpio_desc *desc)
```

请注意，这些函数应当仅在非常节制的情况下使用；驱动不应需要关心物理线路电平或开漏
语义。


### 使用单次调用访问多个 GPIO


```
	int gpiod_get_array_value(unsigned int array_size,
				  struct gpio_desc **desc_array,
				  struct gpio_array *array_info,
				  unsigned long *value_bitmap);
	int gpiod_get_raw_array_value(unsigned int array_size,
				      struct gpio_desc **desc_array,
				      struct gpio_array *array_info,
				      unsigned long *value_bitmap);
	int gpiod_get_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap);
	int gpiod_get_raw_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap);

	int gpiod_set_array_value(unsigned int array_size,
				  struct gpio_desc **desc_array,
				  struct gpio_array *array_info,
				  unsigned long *value_bitmap)
	int gpiod_set_raw_array_value(unsigned int array_size,
				      struct gpio_desc **desc_array,
				      struct gpio_array *array_info,
				      unsigned long *value_bitmap)
	int gpiod_set_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap)
	int gpiod_set_raw_array_value_cansleep(unsigned int array_size,
					       struct gpio_desc **desc_array,
					       struct gpio_array *array_info,
					       unsigned long *value_bitmap)
```

该数组可以是任意一组 GPIO。如果相应的芯片驱动支持，这些函数会尝试同时访问属于同一
bank 或芯片的 GPIO。在这种情况下，性能会有显著提升。如果无法同时访问，则 GPIO 将被
顺序访问。

这些函数接受四个参数：

 - array_size	- 数组元素的数量
 - desc_array	- 一个 GPIO 描述符数组
 - array_info	- 从 gpiod_get_array() 获取的可选信息
 - value_bitmap	- 用于存储 GPIO 值的位图（get），或
          要分配给 GPIO 的值的位图（set）

描述符数组可以使用 gpiod_get_array() 函数或其某个变体获取。如果该函数返回的描述符组
与所需的 GPIO 组相匹配，那么只需使用以下方式即可访问这些 GPIO：

```
	struct gpio_descs *my_gpio_descs = gpiod_get_array(...);
	gpiod_set_array_value(my_gpio_descs->ndescs, my_gpio_descs->desc,
			      my_gpio_descs->info, my_gpio_value_bitmap);
```

也可以访问完全任意的描述符数组。描述符可以使用 gpiod_get() 和 gpiod_get_array()
的任何组合获取。之后，在将该描述符数组传递给上述函数之一之前，必须手动设置它。在这种
情况下，array_info 应当设为 NULL。

请注意，为了获得最佳性能，属于同一芯片的 GPIO 应当在描述符数组中连续排列。

如果描述符的数组索引与单个芯片的硬件引脚号相匹配，则可以实现更好的性能。如果传递给
get/set 数组函数的数组与从 gpiod_get_array() 获取的数组相匹配，并且也传递了与该数组
关联的 array_info，那么函数可能会采取快速的位图处理路径，将 value_bitmap 参数直接
传递给该芯片相应的 .get/set_multiple() 回调。这样可以将 GPIO bank 用作数据 I/O 端口
而不会损失太多性能。

gpiod_get_array_value() 及其变体的返回值为 0 表示成功，为负表示出错。请注意这与
gpiod_get_value() 的区别，后者成功时返回 0 或 1 以传达 GPIO 值。对于数组函数，GPIO
值存储在 value_array 中，而不是作为返回值传回。


### 映射到 IRQ 的 GPIO


GPIO 线路经常可以用作 IRQ。你可以获取 IRQ 号：

```
	int gpiod_to_irq(const struct gpio_desc *desc)
```

它会返回一个 IRQ 号，或者当映射无法完成时返回负的 errno 码（最可能是因为该特定 GPIO
无法用作 IRQ）。使用未通过 gpiod_direction_input() 设置为输入的 GPIO，或使用并非
原本来自 gpiod_to_irq() 的 IRQ 号，都是未检查的错误。gpiod_to_irq() 不允许睡眠。

gpiod_to_irq() 返回的非错误值可以传递给 request_irq() 或 free_irq()。它们通常会被
板级特定的初始化代码存入平台设备的 IRQ 资源中。请注意，IRQ 触发选项（如
IRQF_TRIGGER_FALLING）以及系统唤醒（wakeup）能力都属于 IRQ 接口的一部分。


## GPIO 与 ACPI


在 ACPI 系统上，GPIO 由设备 _CRS 配置对象所列出的 GpioIo()/GpioInt() 资源描述。这些
资源不为 GPIO 提供连接 ID（名称），因此需要为此使用一个额外的机制。

符合 ACPI 5.1 或更新版本的系统可以提供 _DSD 配置对象，除其它用途外，它可用于为 _CRS
中由 GpioIo()/GpioInt() 资源描述的特定 GPIO 提供连接 ID。如果是这种情况，它将由 GPIO
子系统自动处理。然而，如果 _DSD 不存在，则 GpioIo()/GpioInt() 资源与 GPIO 连接 ID
之间的映射需要由设备驱动提供。

细节请参阅 Documentation/firmware-guide/acpi/gpio-properties.rst


## 与遗留 GPIO 子系统交互


许多内核子系统和驱动仍使用遗留的基于整数的接口处理 GPIO。强烈建议将这些更新为新的
gpiod 接口。对于需要同时使用两种接口的情况，以下两个函数允许将 GPIO 描述符转换为
GPIO 整数命名空间：

```
	int desc_to_gpio(const struct gpio_desc *desc)
	struct gpio_desc *gpio_to_desc(unsigned gpio)
```

只要 GPIO 描述符 `desc` 未被释放，desc_to_gpio() 返回的 GPIO 号就可以安全地用作
gpio\_*() 函数的参数。同样，传递给 gpio_to_desc() 的 GPIO 号必须首先通过例如
gpio_request_one() 正确获取，并且返回的 GPIO 描述符仅在该 GPIO 号通过 gpio_free()
释放之前被视为有效。

用一个 API 释放由另一个 API 获取的 GPIO 是被禁止的，并且是未检查的错误。
