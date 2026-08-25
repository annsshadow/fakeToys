
## GPIO 相关_DSD 设备属

随着 ACPI 5.1 的发布，_DSD 配置对象终于允许_CRS 返回GPIO（以及其他事物）指定名称以前我们只能使用整数索引来查找对应的 GPIO，这非常容易出错（例如，它依赖于 _CRS 输出的顺序）
借助 _DSD，我们现在可以使用名称而不是整数来查询 GPIO
```

  // Bluetooth device with reset and shutdown GPIOs
  Device (BTH)
  {
      Name (_HID, ...)

      Name (_CRS, ResourceTemplate ()
      {
          GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 15 }
          GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 27, 31 }
      })

      Name (_DSD, Package ()
      {
          ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
          Package ()
          {
              Package () { "reset-gpios", Package () { ^BTH, 1, 1, 0 } },
              Package () { "shutdown-gpios", Package () { ^BTH, 0, 0, 0 } },
          }
      })
  }

```

```

  Package () { "name", Package () { ref, index, pin, active_low }}

```

ref
  _CRS 中包GpioIo()/GpioInt() 资源的设备，通常就是设备自身（本例中BTH）index
  _CRS GpioIo()/GpioInt() 资源的索引，从零开始pin
  GpioIo()/GpioInt() 资源中的引脚。通常为零active_low
  如果1，则GPIO 被标记为低电平有效（active-low）
由于 ACPI GpioIo() 资源没有字段说明它是低电平有效还是高电平有效，“active_low”参数可在此处使用将其设置1 可将 GPIO 标记为低电平有效
注意，_DSD 中的 active_low GpioInt() 资源没有意义，必须为 0。GpioInt() 资源有其自身的定义方式
在我们的蓝牙示例中，“reset-gpios”指的是第二GpioIo() 资源、该资源中的第二个引脚，GPIO 编号31
遗憾的是，GpioIo() 资源没有显式提供驱动在其初始化期间应当使用的输出引脚的初始状态
Linux 在这里尝试使用常识，并从偏置（bias）和极性设置中推导状态。下表显示了预期
+-------------+-------------+-----------------------------------------------+
| Pull Bias   | Polarity    | Requested...                                  |
+=============+=============+===============================================+
| Implicit                                                                  |
+-------------+-------------+-----------------------------------------------+
| **Default** | x           | AS IS（假设固件已为我们配置好             |
+-------------+-------------+-----------------------------------------------+
| Explicit                                                                  |
+-------------+-------------+-----------------------------------------------+
| **None**    | x           | AS IS（假设固件已为我们配置好             |
|             |             | 且无 Pull Bias                                |
+-------------+-------------+-----------------------------------------------+
| **Up**      | x (no _DSD) |                                               |
|             +-------------+ 假设非激活，视为高电                         |
|             | Low         |                                               |
|             +-------------+-----------------------------------------------+
|             | High        | 假设激活，视为高电                         |
+-------------+-------------+-----------------------------------------------+
| **Down**    | x (no _DSD) |                                               |
|             +-------------+ 假设非激活，视为低电                         |
|             | High        |                                               |
|             +-------------+-----------------------------------------------+
|             | Low         | 假设激活，视为低电                         |
+-------------+-------------+-----------------------------------------------+

也就是说，对于我们上面的示例，由于偏置设置是显式的且存在 _DSD，两GPIO 都将被视为高电平有效并且 Linux 会将引脚配置为此状态，直到驱动以不同方式重新编程它们
可以GPIO 数组中留下空洞。这在像 SPI 主机控制器这样的情况下很有用，其中一些片选可能实现为
GPIO，而另一些实现为原生信号。例如，一SPI 主机控制器可以将片0 2 实现GPIO，而将 1
实现
```

  Package () {
      "cs-gpios",
      Package () {
          ^GPIO, 19, 0, 0, // chip select 0: GPIO
          0,               // chip select 1: native signal
          ^GPIO, 20, 0, 0, // chip select 2: GPIO
      }
  }

```

注意，历史上 ACPI 没有表示 GPIO 极性的手段，因SPISerialBus() 资源按每芯片定义极性。为了避一连串的否定，GPIO 极性被视为高电平有效（Active High）。即使在涉及 _DSD() 的情况下（见上面示例），GPIO CS 极性也必须定义为高电平有效以避免歧义
## 其他受支持的属

以下与设备树兼容的设备属性也GPIO 控制器的 _DSD 设备属性支持：

- gpio-hog
- output-high
- output-low
- input
- line-name

```

  Name (_DSD, Package () {
      // _DSD Hierarchical Properties Extension UUID
      ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
      Package () {
          Package () { "hog-gpio8", "G8PU" }
      }
  })

  Name (G8PU, Package () {
      ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
      Package () {
          Package () { "gpio-hog", 1 },
          Package () { "gpios", Package () { 8, 0 } },
          Package () { "output-high", 1 },
          Package () { "line-name", "gpio8-pullup" },
      }
  })

```

- gpio-line-names

`gpio-line-names` 声明是一个字符串列表（“names”），描GPIO 控制扩展器的每个线路/引脚此列表包含在一个包中，必须插入ACPI 表（通常位于 DSDT 内）GPIO 控制器声明内部。`gpio-line-names`
列表必须遵守以下规则（另见示例）
  - 列表中的第一个名称对GPIO 控制扩展器的第一个线引脚
  - 列表内的名称必须连续（不允许有“空洞”）
  - 列表可以不完整，并可以在最后一GPIO 线路之前结束：换句话说，不强制填充所GPIO 线路
  - 允许空名称（两个引号 `""` 对应一个空名称  - 同一GPIO 控制扩展器内的名称必须唯一

一个具16 条线路的 GPIO 控制器示例，带有一个不完整的列表，其中包含两个

```

  Package () {
      "gpio-line-names",
      Package () {
          "pin_0",
          "pin_1",
          "",
          "",
          "pin_3",
          "pin_4_push_button",
      }
  }

```

在运行时，上述声明产生如下结果（使用

```

  root@debian:~# gpioinfo gpiochip4
  gpiochip4 - 16 lines:
          line   0:      "pin_0"       unused   input  active-high
          line   1:      "pin_1"       unused   input  active-high
          line   2:      unnamed       unused   input  active-high
          line   3:      unnamed       unused   input  active-high
          line   4:      "pin_3"       unused   input  active-high
          line   5: "pin_4_push_button" unused input active-high
          line   6:      unnamed       unused   input  active-high
          line   7       unnamed       unused   input  active-high
          line   8:      unnamed       unused   input  active-high
          line   9:      unnamed       unused   input  active-high
          line  10:      unnamed       unused   input  active-high
          line  11:      unnamed       unused   input  active-high
          line  12:      unnamed       unused   input  active-high
          line  13:      unnamed       unused   input  active-high
          line  14:      unnamed       unused   input  active-high
          line  15:      unnamed       unused   input  active-high
  root@debian:~# gpiofind pin_4_push_button
  gpiochip4 5
  root@debian:~#

```

```

  Package () {
      "gpio-line-names",
      Package () {
          "SPI0_CS_N", "EXP2_INT", "MUX6_IO", "UART0_RXD",
          "MUX7_IO", "LVL_C_A1", "MUX0_IO", "SPI1_MISO",
      }
  }

```

有关这些属性的更多信息，请参阅 Documentation/devicetree/bindings/gpio/gpio.txt
## 驱动提供ACPI GPIO 映射


有些系统ACPI 表不包含 _DSD，但提供了带GpioIo()/GpioInt() 资源_CRS，而设备驱动仍需与之配合工作
在这些情况下，驱动可用的 ACPI 设备标识对象（_HID、_CID、_CLS、_SUB、_HRV）可用于标识设备，这应当
足以确定 _CRS 返回GpioIo()/GpioInt() 资源所列出的所GPIO 线路的含义和用途。换句话说，一驱动标识了设备，它就应该知道要使GpioIo()/GpioInt() 资源中的哪些内容。完成此工作后，它可以简单地
为它将要使用GPIO 线路分配名称，并GPIO 子系统提供这些名称与对应 ACPI GPIO 资源之间的映射
为此，驱动需要定义一个映射表，作struct acpi_gpio_mapping 对象的以 NULL 结尾的数组，每个对象包含
一个名称、一个指向线路数据（struct acpi_gpio_params）对象数组的指针，以及该数组的大小。每struct acpi_gpio_params 对象由三个字段组成：crs_entry_index、line_index、active_low，分别表_CRS 中目GpioIo()/GpioInt() 资源的索引（从零开始）、该资源中目标线路的索引（从零开始）以及该线的低电平有效标志，与上文中指定的 _DSD GPIO 属性格式相对应
对于前面讨论的示例蓝牙设备，数据结构位于

```

  static const struct acpi_gpio_params reset_gpio = { 1, 1, false };
  static const struct acpi_gpio_params shutdown_gpio = { 0, 0, false };

  static const struct acpi_gpio_mapping bluetooth_acpi_gpios[] = {
      { "reset-gpios", &reset_gpio, 1 },
      { "shutdown-gpios", &shutdown_gpio, 1 },
      { }
  };

```

接下来，需要将映射表作为第二个参数传递给 acpi_dev_add_driver_gpios() 或其托管版本，后者将把它
注册到其第一个参数所指向ACPI 设备对象。这应当在驱动的 .probe() 例程中完成。在移除时，驱动应通过
在先前注册该表的 ACPI 设备对象上调acpi_dev_remove_driver_gpios() 来注销GPIO 映射表
## 使用 _CRS 回退


如果设备没有 _DSD，或者驱动没有创ACPI GPIO 映射，Linux GPIO 框架会拒绝返回任GPIO。这是因驱动不知道它实际得到的是什么。例如，如果

```

  Device (BTH)
  {
      Name (_HID, ...)

      Name (_CRS, ResourceTemplate () {
          GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionNone,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 15 }
          GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionNone,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 27 }
      })
  }

```

```

  desc = gpiod_get(dev, "reset", GPIOD_OUT_LOW);
  if (IS_ERR(desc))
	...error handling...

```

但由于无法知道“reset”与 _CRS 中的 GpioIo() 之间的映射，desc 将持ERR_PTR(-ENOENT)
驱动作者可以通过显式传递映射来解决这个问题（这是推荐的方式，已在上一章中说明）
ACPI GPIO 映射表不应污染那些不知道自己正在服务哪个具体设备的驱动。这意味着 ACPI GPIO 映射表与
ACPI ID 以及上文所列的该设备特定对象紧密相连
## 获取 GPIO 描述

```

  desc = gpiod_get(dev, connection_id, flags);
  desc = gpiod_get_index(dev, connection_id, index, flags);

```

我们在这里可以考虑两种不同的情形，即是否提供了连接 ID（connection ID）
```

  desc = gpiod_get(dev, "non-null-connection-id", flags);
  desc = gpiod_get_index(dev, "non-null-connection-id", index, flags);

```

情形 1 假定相应ACPI 设备描述必须已定义了设备属性，否则将阻止获取任GPIO 资源
```

  desc = gpiod_get(dev, NULL, flags);
  desc = gpiod_get_index(dev, NULL, index, flags);

```

情形 2 显式告知 GPIO 核心_CRS 中查找资源
请注意，在情1 和情2 中，假设提供了两个版本的 ACPI 设备描述且驱动中没有映射，gpiod_get_index()
将返回不同的资源。这就是为什么某个特定驱动必须如上一章所述小心处理它们