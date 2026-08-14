## 如何实例化 I2C 设备


与 PCI 或 USB 设备不同，I2C 设备不在硬件层面被枚举。相反，软件必须知道每个 I2C
总线段上连接了哪些设备，以及这些设备使用的地址。因此，内核代码必须显式地实例化
I2C 设备。根据上下文和需求，有几种方法可以实现这一点。


### 方法 1：静态声明 I2C 设备


当 I2C 总线是一条系统总线时（许多嵌入式系统就是这种情况），这种方法很合适。在
这类系统上，每条 I2C 总线的编号是预先知道的。因此，可以预先声明驻留在此总线上的
I2C 设备。

这些信息在不同架构上以不同方式提供给内核：设备树（devicetree）、ACPI 或板文件
（board files）。

当所讨论的 I2C 总线被注册时，I2C 设备将由 i2c-core 自动实例化。当它们所在 I2C
总线消失时（如果曾经发生），这些设备将自动解绑并销毁。


##### 通过 devicetree 声明 I2C 设备


在使用 devicetree 的平台上，I2C 设备的声明是在主控制器的子节点中完成的。

示例：


	i2c1: i2c@400a0000 {
		/** ... master properties skipped ... **/
		clock-frequency = <100000>;

		flash@50 {
			compatible = "atmel,24c256";
			reg = <0x50>;
		};

		pca9532: gpio@60 {
			compatible = "nxp,pca9532";
			gpio-controller;
			#gpio-cells = <2>;
			reg = <0x60>;
		};
	};

这里，两个设备以 100kHz 的速度连接到总线。关于设置设备可能需要的额外属性，请参考
其在 Documentation/devicetree/bindings/ 中的 devicetree 文档。


##### 通过 ACPI 声明 I2C 设备


ACPI 也可以描述 I2C 设备。对此有专门的文档，目前位于
Documentation/firmware-guide/acpi/enumeration.rst。


##### 在板文件中声明 I2C 设备


在许多嵌入式架构中，devicetree 已经取代了基于板文件的旧硬件描述，但后者仍在旧
代码中使用。通过板文件实例化 I2C 设备，是使用一个 struct i2c_board_info 数组，
并通过调用 i2c_register_board_info() 来注册的。

示例（来自 omap2 h4）：


  static struct i2c_board_info h4_i2c_board_info[] __initdata = {
	{
		I2C_BOARD_INFO("isp1301_omap", 0x2d),
		.irq		= OMAP_GPIO_IRQ(125),
	},
	{	/** 主板上的 EEPROM **/
		I2C_BOARD_INFO("24c01", 0x52),
		.platform_data	= &m24c01,
	},
	{	/** CPU 卡上的 EEPROM **/
		I2C_BOARD_INFO("24c01", 0x57),
		.platform_data	= &m24c01,
	},
  };

  static void __init omap_h4_init(void)
  {
	(...)
	i2c_register_board_info(1, h4_i2c_board_info,
			ARRAY_SIZE(h4_i2c_board_info));
	(...)
  }

上面的代码在 I2C 总线 1 上声明了 3 个设备，包括它们各自的地址以及其驱动程序需要
的自定义数据。


### 方法 2：显式实例化设备


当更大的设备使用 I2C 总线进行内部通信时，这种方法很合适。典型情况是电视适配器。
它们可以有一个调谐器、一个视频解码器、一个音频解码器等，通常通过 I2C 总线连接到
主芯片。你事先不会知道 I2C 总线的编号，因此上面描述的方法 1 无法使用。相反，你
可以显式地实例化你的 I2C 设备。这通过填充一个 struct i2c_board_info 并调用
i2c_new_client_device() 来完成。

示例（来自 sfe4001 网络驱动程序）：


  static struct i2c_board_info sfe4001_hwmon_info = {
	I2C_BOARD_INFO("max6647", 0x4e),
  };

  int sfe4001_init(struct efx_nic *efx)
  {
	(...)
	efx->board_info.hwmon_client =
		i2c_new_client_device(&efx->i2c_adap, &sfe4001_hwmon_info);

	(...)
  }

上面的代码在所讨论的网络适配器上的 I2C 总线上实例化 1 个 I2C 设备。

一种变体情况是，你不确定某个 I2C 设备是否存在（例如某个可选特性在廉价板型上不
存在，但你无法区分它们），或者它可能从一块板到另一块板具有不同的地址（制造商在
未通知的情况下更改了其设计）。在这种情况下，你可以调用 i2c_new_scanned_device()
而不是 i2c_new_client_device()。

示例（来自 nxp OHCI 驱动程序）：


  static const unsigned short normal_i2c[] = { 0x2c, 0x2d, I2C_CLIENT_END };

  static int usb_hcd_nxp_probe(struct platform_device *pdev)
  {
	(...)
	struct i2c_adapter *i2c_adap;
	struct i2c_board_info i2c_info;

	(...)
	i2c_adap = i2c_get_adapter(2);
	memset(&i2c_info, 0, sizeof(struct i2c_board_info));
	strscpy(i2c_info.type, "isp1301_nxp", sizeof(i2c_info.type));
	isp1301_i2c_client = i2c_new_scanned_device(i2c_adap, &i2c_info,
						    normal_i2c, NULL);
	i2c_put_adapter(i2c_adap);
	(...)
  }

上面的代码在所讨论的 OHCI 适配器上的 I2C 总线上最多实例化 1 个 I2C 设备。它先尝试
地址 0x2c，如果那里没有找到，则尝试地址 0x2d，如果仍然没有找到，就直接放弃。

实例化该 I2C 设备的驱动程序负责在清理时销毁它。这是通过对先前由
i2c_new_client_device() 或 i2c_new_scanned_device() 返回的指针调用
i2c_unregister_device() 来完成的。


### 方法 3：探测 I2C 总线以寻找特定设备


有时你对某个 I2C 设备的信息不足，甚至无法调用 i2c_new_scanned_device()。典型的
情况是 PC 主板上的硬件监控芯片。有几十种型号，可以驻留在 25 个不同的地址上。鉴于
市面上有大量的主板，几乎不可能建立一个详尽的硬件监控芯片清单。幸运的是，这些芯片
大多数都有制造商和设备 ID 寄存器，因此可以通过探测来识别它们。

在这种情况下，I2C 设备既不被声明，也不被显式实例化。相反，i2c-core 将在它们的
驱动程序被加载后立即探测此类设备，如果找到任何一个，就会自动实例化一个 I2C 设备。
为了防止此机制出现任何异常行为，适用以下限制：

- I2C 设备驱动程序必须实现 detect() 方法，该方法通过从任意寄存器读取来识别受
  支持的设备。
- 只有那些可能有受支持设备并同意被探测的总线才会被探测。例如，这就避免了对
  电视适配器上的硬件监控芯片进行探测。

示例：
参见 drivers/hwmon/lm90.c 中的 lm90_driver 和 lm90_detect()。

作为这种成功探测的结果而实例化的 I2C 设备，将在探测到它们的驱动程序被移除时，或
者当底层的 I2C 总线本身被销毁时（以先发生者为准）被自动销毁。

熟悉 2.4 内核和早期 2.6 内核 I2C 子系统的人会发现，这方法 3 本质上与当时所做的
类似。两个显著的区别是：

- 探测现在只是实例化 I2C 设备的一种方式，而当时是唯一的方式。在可能的情况下，
  应优先使用方法 1 和 2。方法 3 只应在没有其他途径时使用，因为它可能产生不希望
  的副作用。
- I2C 总线现在必须显式说明哪些 I2C 驱动程序类可以探测它们（通过 class 位域），
  而当时默认会探测所有 I2C 总线。默认是一个空的 class，这意味着不进行任何探测。
  class 位域的目的是限制上述不希望的副作用。

再次强调，应尽可能避免方法 3。显式设备实例化（方法 1 和 2）更可取，因为它更安全、
  更快。


### 方法 4：从用户空间实例化


一般来说，内核应该知道连接了哪些 I2C 设备以及它们位于什么地址。但是，在某些情况下
它并不知道，因此添加了一个 sysfs 接口，让用户提供这些信息。这个接口由两个属性文件
组成，它们在每个 I2C 总线目录中创建：`new_device` 和 `delete_device`。这两个文件
都只可写，你必须向它们写入正确的参数，才能正确地实例化或删除一个 I2C 设备。

文件 `new_device` 接受 2 个参数：I2C 设备的名称（一个字符串）和 I2C 设备的地址
（一个数字，通常以 0x 开头的十六进制表示，但也可以用十进制表示）。

文件 `delete_device` 接受单个参数：I2C 设备的地址。由于在给定的 I2C 段上不可能
有两个设备位于同一地址，该地址足以唯一标识要删除的设备。

```

  # echo eeprom 0x50 > /sys/bus/i2c/devices/i2c-3/new_device

```

虽然这个接口只应在无法在核内声明设备时使用，但有许多情况下它可能很有帮助：

- I2C 驱动程序通常检测设备（上面的方法 3），但你的设备所在的总线段没有设置正确的
  class 位，因此检测没有触发。
- I2C 驱动程序通常检测设备，但你的设备位于一个意外的地址。
- I2C 驱动程序通常检测设备，但你的设备未被检测到，要么是因为检测例程太严格，要么
  是因为你的设备尚未被正式支持，但你知道它是兼容的。
- 你正在一块测试板上开发驱动程序，在那里你自己焊接了 I2C 设备。

这个接口取代了一些 I2C 驱动程序实现的 force_* 模块参数。由于它是在 i2c-core 中
实现的，而不是在每个设备驱动程序中单独实现，因此它效率更高，并且还有一个优点，即
你不需要为了更改设置而重新加载驱动程序。你也可以在驱动程序加载甚至可用之前实例化
设备，而且你不需要知道该设备需要什么驱动程序。
