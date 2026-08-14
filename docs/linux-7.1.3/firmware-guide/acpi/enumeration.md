
## 基于 ACPI 的设备枚举


ACPI 5 引入了一组新资源（UartTSerialBus、I2cSerialBus、SpiSerialBus、GpioIo 与 GpioInt），可用于枚举串行总线控制器背后的从设备。

此外，我们开始看到集成在 SoC/芯片组中的外设仅出现在 ACPI 命名空间中。这些通常是通过内存映射寄存器访问的设备。

为了支持这一点并尽可能复用现有驱动，我们决定采取以下做法：

  - 没有总线连接器资源的设备表示为 platform 设备。

  - 位于真实总线之后、且存在连接器资源的设备表示为 struct spi_device 或 struct i2c_client。注意，标准 UART 并不是总线，因此不存在 struct uart_device，不过其中一些可以由 struct serdev_device 表示。

由于 ACPI 与 Device Tree 都表示一棵设备（及其资源）树，本实现尽可能遵循 Device Tree 的方式。ACPI 实现枚举总线（platform、SPI、I2C，以及某些情况下的 UART）背后的设备，创建物理设备，并将它们绑定到其在 ACPI 命名空间中的 ACPI handle。

这意味着当 ACPI_HANDLE(dev) 返回非 NULL 时，该设备是从 ACPI 命名空间枚举而来的。此 handle 可用于提取其他设备特定的配置。下面有一个示例。

## 平台总线支持


由于我们使用 platform 设备来表示未连接到任何物理总线的设备，我们只需为该设备实现一个 platform 驱动并添加受支持的 ACPI ID。如果在其他某个非 ACPI 平台上使用了相同的 IP 模块，该驱动也许可以开箱即用，或只需少量修改。

为现有驱动添加 ACPI 支持应当相当

```
	static const struct acpi_device_id mydrv_acpi_match[] = {
		/* ACPI IDs here */
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, mydrv_acpi_match);

	static struct platform_driver my_driver = {
		...
		.driver = {
			.acpi_match_table = mydrv_acpi_match,
		},
	};
```

如果驱动需要执行更复杂的初始化（例如获取并配置 GPIO），它可以获取其 ACPI handle 并从此 ACPI 表中提取该信息。

## ACPI 设备对象


一般来说，在使用 ACPI 作为平台固件与 OS 之间接口的系统中有两类设备：一类是无需平台固件协助、通过为其所在特定总线定义的协议（例如 PCI 中的配置空间）即可被原生发现并枚举的设备；另一类是需要由平台固件描述才能被发现的设备。不过，对于平台固件已知的任何设备，无论它属于哪一类，在 ACPI 命名空间中都可能存在一个对应的 ACPI 设备对象，此时 Linux 内核会基于它为该设备创建一个 struct acpi_device 对象。

那些 struct acpi_device 对象从不用于为原生可发现的设备绑定驱动，因为它们由其他类型的设备对象（例如 PCI 设备的 struct pci_dev）表示，并由设备驱动绑定（相应的 struct acpi_device 对象则用作关于该设备配置的额外信息来源）。此外，ACPI 设备枚举核心代码为绝大多数借助平台固件发现并枚举的设备创建 struct platform_device 对象，而这些 platform 设备对象可以由 platform 驱动绑定，与原生可枚举设备的情况直接类比。因此，将驱动绑定到 struct acpi_device 对象在逻辑上是不一致的，因而通常是无效的，包括为借助平台固件发现的设备编写的驱动也是如此。

历史上，曾为一些借助平台固件枚举的设备实现过直接绑定到 struct acpi_device 对象的 ACPI 驱动，但不建议任何新驱动这样做。如上所述，这些设备原则上都会创建 platform 设备对象（此处无关的少数例外除外），因此即使相应的 ACPI 设备对象是这种情况下唯一的设备配置信息来源，也应使用 platform 驱动来处理它们。

对于每个拥有对应 struct acpi_device 对象的设备，指向它的指针由 ACPI_COMPANION() 宏返回，因此总是可以通过这种方式获取到存储在 ACPI 设备对象中的设备配置信息。相应地，struct acpi_device 可视为内核与 ACPI 命名空间之间接口的一部分，而其他类型的设备对象（例如 struct pci_dev 或 struct platform_device）则用于与系统其余部分交互。

## DMA 支持


通过 ACPI 枚举的 DMA 控制器应在系统中注册，以提供对其资源的通用访问。例如，希望从属设备能通过通用 API 调用 dma_request_chan() 访问的驱动，必须在 probe 函数末尾像下面这样注册自己：

```
	err = devm_acpi_dma_controller_register(dev, xlate_func, dw);
	/* Handle the error if it's not a case of !CONFIG_ACPI */
```

并在需要时实现自定义的 xlate 函数（通常 acpi_dma_simple_xlate() 已足够），该函数将 struct acpi_dma_spec 提供的 FixedDMA 资源转换为相应的 DMA 通道。相关代码片断如下：

```
	#ifdef CONFIG_ACPI
	struct filter_args {
		/* Provide necessary information for the filter_func */
		...
	};

	static bool filter_func(struct dma_chan *chan, void *param)
	{
		/* Choose the proper channel */
		...
	}

	static struct dma_chan *xlate_func(struct acpi_dma_spec *dma_spec,
			struct acpi_dma *adma)
	{
		dma_cap_mask_t cap;
		struct filter_args args;

		/* Prepare arguments for filter_func */
		...
		return dma_request_channel(cap, filter_func, &args);
	}
	#else
	static struct dma_chan *xlate_func(struct acpi_dma_spec *dma_spec,
			struct acpi_dma *adma)
	{
		return NULL;
	}
	#endif
```

dma_request_chan() 会为每个已注册的 DMA 控制器调用 xlate_func()。在 xlate 函数中，必须根据 struct acpi_dma_spec 中的信息以及 struct acpi_dma 提供的控制器属性来选择合适的通道。

客户端必须使用对应于特定 FixedDMA 资源的字符串参数调用 dma_request_chan()。默认情况下 "tx" 表示 FixedDMA 资源数组的第一项，"rx" 表示第二项。下表演示了一个

```
	Device (I2C0)
	{
		...
		Method (_CRS, 0, NotSerialized)
		{
			Name (DBUF, ResourceTemplate ()
			{
				FixedDMA (0x0018, 0x0004, Width32bit, _Y48)
				FixedDMA (0x0019, 0x0005, Width32bit, )
			})
		...
		}
	}
```

因此，在本例中请求线为 0x0018 的 FixedDMA 是 "tx"，下一个是 "rx"。

在健壮的实现中，客户端不巧需要直接调用 acpi_dma_request_slave_chan_by_index()，从而按索引选择特定的 FixedDMA 资源。

## 命名中断


通过 ACPI 枚举的驱动可以在 ACPI 表中为中断命名，这些名称可用于在驱动中获取 IRQ 号。中断名称可以在 _DSD 中以 'interrupt-names' 列出。这些名称应列为一个字符串数组，它们将映射到 ACPI 表中与其索引对应的 Interrupt() 资源。

```
    Device (DEV0) {
        ...
        Name (_CRS, ResourceTemplate() {
            ...
            Interrupt (ResourceConsumer, Level, ActiveHigh, Exclusive) {
                0x20,
                0x24
            }
        })

        Name (_DSD, Package () {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "interrupt-names", Package () { "default", "alert" } },
            }
        ...
        })
    }
```

中断名称 'default' 将对应 Interrupt() 资源中的 0x20，'alert' 对应 0x24。注意，仅映射 Interrupt() 资源，而不映射 GpioInt() 或类似资源。

驱动可以调用函数 fwnode_irq_get_byname()，以 fwnode 与中断名称作为参数，来获取相应的 IRQ 号。

## SPI 串行总线支持


位于 SPI 总线之后的从设备附有 SpiSerialBus 资源。SPI 核心会自动提取它，并且一旦总线驱动调用 spi_register_master()，从设备就会被枚举。

```
	Device (EEP0)
	{
		Name (_ADR, 1)
		Name (_CID, Package () {
			"ATML0025",
			"AT25",
		})
		...
		Method (_CRS, 0, NotSerialized)
		{
			SPISerialBus(1, PolarityLow, FourWireMode, 8,
				ControllerInitiated, 1000000, ClockPolarityLow,
				ClockPhaseFirst, "\\_SB.PCI0.SPI1",)
		}
		...
```

SPI 设备驱动只需以类似于 platform 设备驱动的方式添加 ACPI ID。下面是一个我们添加 ACPI 支持的示例

```
	static const struct acpi_device_id at25_acpi_match[] = {
		{ "AT25", 0 },
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, at25_acpi_match);

	static struct spi_driver at25_driver = {
		.driver = {
			...
			.acpi_match_table = at25_acpi_match,
		},
	};
```

注意，该驱动实际上需要更多信息，例如页大小等

```
	Device (EEP0)
	{
		...
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () { "size", 1024 },
				Package () { "pagesize", 32 },
				Package () { "address-width", 16 },
			}
		})
	}
```

然后 at25 SPI 驱动可以通过调用设备属性接口获取此配置

```
	err = device_property_read_u32(dev, "size", &size);
	if (err)
		...error handling...

	err = device_property_read_u32(dev, "pagesize", &page_size);
	if (err)
		...error handling...

	err = device_property_read_u32(dev, "address-width", &addr_width);
	if (err)
		...error handling...
```

## I2C 串行总线支持


位于 I2C 总线控制器之后的从设备只需像 platform 与 SPI 驱动那样添加 ACPI ID。一旦适配器注册，I2C 核心会自动枚举控制器设备背后的任何从设备。

下面是将 ACPI 支持添加到现有 mpu3050 的示例

```
	static const struct acpi_device_id mpu3050_acpi_match[] = {
		{ "MPU3050", 0 },
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, mpu3050_acpi_match);

	static struct i2c_driver mpu3050_i2c_driver = {
		.driver	= {
			.name	= "mpu3050",
			.pm	= &mpu3050_pm,
			.of_match_table = mpu3050_of_match,
			.acpi_match_table = mpu3050_acpi_match,
		},
		.probe		= mpu3050_probe,
		.remove		= mpu3050_remove,
		.id_table	= mpu3050_ids,
	};
	module_i2c_driver(mpu3050_i2c_driver);
```

## 对 PWM 设备的引用


有时一个设备可以是某个 PWM 通道的消费者。显然 OS 希望知道是哪一个。为了提供这种映射，特殊属性已被

```
    Device (DEV)
    {
        Name (_DSD, Package ()
        {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "compatible", Package () { "pwm-leds" } },
                Package () { "label", "alarm-led" },
                Package () { "pwms",
                    Package () {
                        "\\_SB.PCI0.PWM",  // <PWM device reference>
                        0,                 // <PWM index>
                        600000000,         // <PWM period>
                        0,                 // <PWM flags>
                    }
                }
            }
        })
        ...
    }
```

在上述示例中，基于 PWM 的 LED 驱动引用了 \_SB.PCI0.PWM 设备的 PWM 通道 0，初始周期设置为 600 ms（注意该值以纳秒给出）。

## GPIO 支持


ACPI 5 引入了两个新资源来描述 GPIO 连接：GpioIo 与 GpioInt。这些资源可用于将设备使用的 GPIO 编号传递给驱动。ACPI 5.1 通过 _DSD（Device Specific Data，设备特定数据）对此进行了扩展，除其他功能外，还使得可以为 GPIO 命名。

```
	Device (DEV)
	{
		Method (_CRS, 0, NotSerialized)
		{
			Name (SBUF, ResourceTemplate()
			{
				// Used to power on/off the device
				GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionOutputOnly,
					"\\_SB.PCI0.GPI0", 0, ResourceConsumer) { 85 }

				// Interrupt for the device
				GpioInt (Edge, ActiveHigh, ExclusiveAndWake, PullNone, 0,
					 "\\_SB.PCI0.GPI0", 0, ResourceConsumer) { 88 }
			}

			Return (SBUF)
		}

		// ACPI 5.1 _DSD used for naming the GPIOs
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () { "power-gpios", Package () { ^DEV, 0, 0, 0 } },
				Package () { "irq-gpios", Package () { ^DEV, 1, 0, 0 } },
			}
		})
		...
	}
```

这些 GPIO 编号是相对于控制器的，路径 "\\_SB.PCI0.GPI0" 指定了控制器所在的路径。为了在 Linux 中使用这些 GPIO，我们需要将它们转换为相应的 Linux GPIO 描述符。

对此有一个标准的 GPIO API，其文档位于 Documentation/admin-guide/gpio/。

在上述示例中，我们可以通过以下方式获取相应的两个 GPIO 描述符：

```
	#include <linux/gpio/consumer.h>
	...

	struct gpio_desc *irq_desc, *power_desc;

	irq_desc = gpiod_get(dev, "irq");
	if (IS_ERR(irq_desc))
		/* handle error */

	power_desc = gpiod_get(dev, "power");
	if (IS_ERR(power_desc))
		/* handle error */

	/* Now we can use the GPIO descriptors */
```

这些函数还有 devm_* 版本，会在设备释放时一并释放描述符。

有关与 GPIO 相关的 _DSD 绑定，详见 Documentation/firmware-guide/acpi/gpio-properties.rst。

## RS-485 支持


ACPI _DSD（Device Specific Data）可用于描述 UART 的 RS-485 能力。

```
	Device (DEV)
	{
		...

		// ACPI 5.1 _DSD used for RS-485 capabilities
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () {"rs485-rts-active-low", Zero},
				Package () {"rs485-rx-active-high", Zero},
				Package () {"rs485-rx-during-tx", Zero},
			}
		})
		...
```

## MFD 设备


MFD 设备将其子设备注册为 platform 设备。对于子设备，需要一个 ACPI handle，供其引用与自身相关的 ACPI 命名空间部分。在 Linux MFD 子系统中我们提供两种方式：

  - 子设备共享父设备的 ACPI handle。
  - MFD cell 可以指定该设备的 ACPI id。

对于第一种情况，MFD 驱动无需做任何事。生成的子 platform 设备其 ACPI_COMPANION() 将被设置为指向父设备。

如果 ACPI 命名空间中有一个我们可以通过 ACPI id 或 ACPI

```
	static struct mfd_cell_acpi_match my_subdevice_cell_acpi_match = {
		.pnpid = "XYZ0001",
		.adr = 0,
	};

	static struct mfd_cell my_subdevice_cell = {
		.name = "my_subdevice",
		/* set the resources relative to the parent */
		.acpi_match = &my_subdevice_cell_acpi_match,
	};
```

然后，ACPI id "XYZ0001" 被用于直接在 MFD 设备下查找一个 ACPI 设备，若找到，则该 ACPI companion 设备被绑定到生成的子 platform 设备。

## Device Tree 命名空间链接设备 ID


Device Tree 协议使用基于 "compatible" 属性的设备标识，该属性的值是一个字符串或一组字符串，被驱动与驱动核心识别为设备标识符。所有这些字符串的集合可被视为一个设备标识命名空间，类似于 ACPI/PNP 设备 ID 命名空间。因此，原则上不应有必要为在 Device Tree（DT）命名空间中已有标识字符串的设备分配一个新的（且可说是冗余的）ACPI/PNP 设备 ID，尤其是当该 ID 仅用于表明某个给定设备与另一个设备兼容（后者大概在内核中已有匹配的驱动）时。

在 ACPI 中，名为 _CID（Compatible ID，兼容 ID）的设备标识对象用于列出给定设备所兼容设备的 ID，但这些 ID 必须属于 ACPI 规范规定的某个命名空间（详见 ACPI 6.0 第 6.1.2 节），而 DT 命名空间并非其中之一。此外，规范强制要求所有表示设备的 ACPI 对象都必须存在 _HID 或 _ADR 标识对象（ACPI 6.0 第 6.1 节）。对于不可枚举的总线类型，该对象必须是 _HID，且其值也必须是规范规定的某个命名空间中的设备 ID。

特殊的 DT 命名空间链接设备 ID，PRP0001，提供了一种在 ACPI 中使用现有 DT 兼容设备标识、同时又能满足上述源自 ACPI 规范之要求的方法。具体来说，如果 _HID 返回 PRP0001，ACPI 子系统将在设备对象的 _DSD 中查找 "compatible" 属性，并使用该属性的值按照原始 DT 设备标识算法来识别相应设备。如果 "compatible" 属性不存在或其值无效，该设备将不会被 ACPI 子系统枚举。否则，它将自动作为 platform 设备被枚举（除非该设备与其父设备之间存在 I2C 或 SPI 链接，此时 ACPI 核心会将设备枚举留给父设备的驱动），并且 "compatible" 属性值中的标识字符串将与 _CID 列出的设备 ID（如果存在）一起用于为该设备查找驱动。

类似地，如果 PRP0001 出现在 _CID 返回的设备 ID 列表中，则 "compatible" 属性值（如果存在且有效）列出的标识字符串将被用于查找匹配该设备的驱动，但在这种情况下，它们相对于 _HID 与 _CID 列出的其他设备 ID 的优先级，取决于 PRP0001 在 _CID 返回包中的位置。具体来说，_HID 返回的设备 ID 以及在 _CID 返回包中位于 PRP0001 之前的设备 ID 将首先被检查。同样在这种情况下，设备将被枚举到的总线类型取决于 _HID 返回的设备 ID。

例如，下面的 ACPI 示例可用于枚举一个 lm75 类型的 I2C 温度传感器，并使用 Device Tree

```
	Device (TMP0)
	{
		Name (_HID, "PRP0001")
		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "compatible", "ti,tmp75" },
			}
		})
		Method (_CRS, 0, Serialized)
		{
			Name (SBUF, ResourceTemplate ()
			{
				I2cSerialBusV2 (0x48, ControllerInitiated,
					400000, AddressingMode7Bit,
					"\\_SB.PCI0.I2C1", 0x00,
					ResourceConsumer, , Exclusive,)
			})
			Return (SBUF)
		}
	}
```

定义 _HID 返回 PRP0001、且 _DSD 中没有 "compatible" 属性或 _CID 的设备对象是合法的，只要它们的某个祖先提供了一个带有有效 "compatible" 属性的 _DSD。这样的设备对象随后被简单地视为额外的「块」，向复合祖先设备驱动提供分层配置信息。

不过，PRP0001 只能从设备对象的 _HID 或 _CID 返回，前提是与它关联的 _DSD（无论是设备对象自身的 _DSD，还是上述「复合设备」情况下其祖先的 _DSD）返回的所有属性都可以在 ACPI 环境中使用。否则，_DSD 本身被视为无效，因而其返回的 "compatible" 属性也就毫无意义。

更多信息请参阅 Documentation/firmware-guide/acpi/DSD-properties-rules.rst。

## PCI 层级表示


有时，在已知 PCI 设备位于 PCI 总线上的位置时枚举它会很有用。例如，某些系统将 PCI 设备（以太网、Wi-Fi、串口等）直接焊接在主板上固定位置。在这种情况下，可以根据这些 PCI 设备在 PCI 总线拓扑中的位置来引用它们。

要识别一个 PCI 设备，需要完整的层级描述，从芯片组根端口一直到最终设备，经过板上所有的中间桥/交换机。

例如，假设我们有一个系统，其主板上焊接了一个 PCIe 串口——Exar XR17V3521。该 UART 芯片还包含 16 个 GPIO，我们希望为这些引脚添加属性 `gpio-line-names` [^1^]_。

```
	07:00.0 Serial controller: Exar Corp. XR17V3521 Dual PCIe UART (rev 03)


	00:00.0 Host bridge: Intel Corp... Host Bridge (rev 0d)
	...
	00:13.0 PCI bridge: Intel Corp... PCI Express Port A #1 (rev fd)
	00:13.1 PCI bridge: Intel Corp... PCI Express Port A #2 (rev fd)
	00:13.2 PCI bridge: Intel Corp... PCI Express Port A #3 (rev fd)
	00:14.0 PCI bridge: Intel Corp... PCI Express Port B #1 (rev fd)
	00:14.1 PCI bridge: Intel Corp... PCI Express Port B #2 (rev fd)
	...
	05:00.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:01.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:02.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:03.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	07:00.0 Serial controller: Exar Corp. XR17V3521 Dual PCIe UART (rev 03) <-- Exar
	...


	-[0000:00]-+-00.0
	           ...
	           +-13.0-[01]----00.0
	           +-13.1-[02]----00.0
	           +-13.2-[03]--
	           +-14.0-[04]----00.0
	           +-14.1-[05-09]----00.0-[06-09]--+-01.0-[07]----00.0 <-- Exar
	           |                               +-02.0-[08]----00.0
	           |                               \-03.0-[09]--
	           ...
	           \-1f.1
```


要描述这个 Exar 设备在 PCI 总线上的位置，我们必须从 ACPI 名称开始

```
	Bus: 0 - Device: 14 - Function: 1
```


要找到这些信息，有必要反汇编 BIOS ACPI 表，

```
	mkdir ~/tables/
	cd ~/tables/
	acpidump > acpidump
	acpixtract -a acpidump
	iasl -e ssdt?.* -d dsdt.dat
```


现在，在 dsdt.dsl 中，我们必须搜索地址与 0x14（设备）和 0x01（功能）相关的设备。在这种情况下我们可以找到以下内容

```
	Scope (_SB.PCI0)
	{
	... other definitions follow ...
		Device (RP02)
		{
			Method (_ADR, 0, NotSerialized)  // _ADR: Address
			{
				If ((RPA2 != Zero))
				{
					Return (RPA2) /* \RPA2 */
				}
				Else
				{
					Return (0x00140001)
				}
			}
	... other definitions follow ...
```


而 _ADR 方法 [^3^]_ 恰好返回我们正在寻找的设备/功能组合。借助这些信息并分析上面的 `lspci` 输出（设备列表与设备树两者），我们可以为 Exar PCIe UART 编写如下 ACPI 描述，同时加入其 GPIO 线列表

```
	Scope (_SB.PCI0.RP02)
	{
		Device (BRG1) //Bridge
		{
			Name (_ADR, 0x0000)

			Device (BRG2) //Bridge
		{
				Name (_ADR, 0x00010000)

				Device (EXAR)
			{
					Name (_ADR, 0x0000)

					Name (_DSD, Package ()
				{
						ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
						Package ()
						{
							Package ()
							{
								"gpio-line-names",
								Package ()
								{
									"mode_232",
									"mode_422",
									"mode_485",
									"misc_1",
									"misc_2",
									"misc_3",
									"",
									"",
									"aux_1",
									"aux_2",
									"aux_3",
								}
							}
						}
					}
				})
				}
			}
		}
	}
```


位置 "_SB.PCI0.RP02" 是通过上述对 dsdt.dsl 表的调查得到的，而设备名 "BRG1"、"BRG2" 与 "EXAR" 是通过分析 Exar UART 在 PCI 总线拓扑中的位置创建的。

## 参考资料



```
    https://uefi.org/sites/default/files/resources/ACPI_6_3_May16.pdf，引用日期 2020-11-18
```
