## PINCTRL（引脚控制）子系

本文档概述了 Linux 中的引脚控制子系
该子系统处理
- 枚举并命名可控的引脚

- 引脚、焊盘、管脚（finger，等）的多路复用（multiplexing，详见下文）

- 引脚、焊盘、管脚（等）的配置，例如软件控制的偏置（biasing）和驱动
  特定引脚的驱动模式，例如上拉、下拉、开漏、负载电容等
## 顶层接口


定义
- 一个引脚控制器（PIN CONTROLLER）是一块硬件，通常是一组寄存器，可  控制引脚（PIN）。它可以为单个引脚或引脚组进行多路复用、偏置、设  负载电容、设置驱动强度等
- 引脚（PINS）等同于焊盘、管脚、球（ball）或任何你想控制的封装输入或
  输出线，它们由范0..maxpin 内的无符号整数表示。这个编号空间对于每  引脚控制器是局部的，因此系统中可能存在多个这样的编号空间。这个引脚空  可能是稀疏的——即空间中可能存在没有引脚的编号间隙
当实例化一个引脚控制器时，它会向引脚控制框架注册一个描述符，该描述包含一个引脚描述符数组，描述该特定引脚控制器所处理的引脚
```

        A   B   C   D   E   F   G   H

   8    o   o   o   o   o   o   o   o

   7    o   o   o   o   o   o   o   o

   6    o   o   o   o   o   o   o   o

   5    o   o   o   o   o   o   o   o

   4    o   o   o   o   o   o   o   o

   3    o   o   o   o   o   o   o   o

   2    o   o   o   o   o   o   o   o

   1    o   o   o   o   o   o   o   o

```
要注册一个引脚控制器并命名此封装上的所有引脚，我们可以在我们的驱动中这样做

	#include <linux/pinctrl/pinctrl.h>

	const struct pinctrl_pin_desc foo_pins[] = {
		PINCTRL_PIN(0, "A8"),
		PINCTRL_PIN(1, "B8"),
		PINCTRL_PIN(2, "C8"),
		...
		PINCTRL_PIN(61, "F1"),
		PINCTRL_PIN(62, "G1"),
		PINCTRL_PIN(63, "H1"),
	};

	static struct pinctrl_desc foo_desc = {
		.name = "foo",
		.pins = foo_pins,
		.npins = ARRAY_SIZE(foo_pins),
		.owner = THIS_MODULE,
	};

	int __init foo_init(void)
	{
		int error;

		struct pinctrl_dev *pctl;

		error = pinctrl_register_and_init(&foo_desc, <PARENT>, NULL, &pctl);
		if (error)
			return error;

		return pinctrl_enable(pctl);
	}

要启pinctrl 子系统以PINMUX PINCONF 的子组和选定的驱动，你需从你机器Kconfig 条目中选择它们，因为它们与所使用机器的集成非常紧密参见 `arch/arm/mach-ux500/Kconfig` 作为示例
引脚通常比这有更花哨的名字。你可以在芯片的数据手册中找到这些。注意核pinctrl.h 文件提供了一个名`PINCTRL_PIN()` 的便捷宏来创建结构体条目。如你所见，
引脚从左上角0 枚举到右下角63。这个枚举是随意选择的，实际上你需要想你的编号系统，使其与驱动中寄存器等事物的布局相匹配，否则代码可能会变得复杂你还必须考虑与引脚控制器可能处理GPIO 范围的偏移匹配
对于一个有 467 个焊盘（pad）而非实际引脚的封装，枚举将像这样，绕着芯片边缘走，这似乎是行业


     0 ..... 104
   466        105
     .        .
     .        .
   358        224
    357 .... 225


## 引脚组（Pin groups

许多控制器需要处理引脚组，因此引脚控制器子系统有一种机制来枚举引脚组并
检索属于某个特定组的实际枚举引脚
例如，假设我们有一个处SPI 接口的引脚组，位{ 0, 8, 16, 24 }，以及一处理 I2C 接口的引脚组，位{ 24, 25 }
这两个组通过实现一些通用`pinctrl_ops` 呈现给引脚控制子系统，像这样

	#include <linux/pinctrl/pinctrl.h>

	static const unsigned int spi0_pins[] = { 0, 8, 16, 24 };
	static const unsigned int i2c0_pins[] = { 24, 25 };

	static const struct pingroup foo_groups[] = {
		PINCTRL_PINGROUP("spi0_grp", spi0_pins, ARRAY_SIZE(spi0_pins)),
		PINCTRL_PINGROUP("i2c0_grp", i2c0_pins, ARRAY_SIZE(i2c0_pins)),
	};

	static int foo_get_groups_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_groups);
	}

	static const char **foo_get_group_name(struct pinctrl_dev **pctldev,
					      unsigned int selector)
	{
		return foo_groups[selector].name;
	}

	static int foo_get_group_pins(struct pinctrl_dev *pctldev,
				      unsigned int selector,
				      const unsigned int **pins,
				      unsigned int *npins)
	{
		*pins = foo_groups[selector].pins;
		*npins = foo_groups[selector].npins;
		return 0;
	}

	static struct pinctrl_ops foo_pctrl_ops = {
		.get_groups_count = foo_get_groups_count,
		.get_group_name = foo_get_group_name,
		.get_group_pins = foo_get_group_pins,
	};

	static struct pinctrl_desc foo_desc = {
		...
		.pctlops = &foo_pctrl_ops,
	};

引脚控制子系统将调用 `.get_groups_count()` 函数来确定合法选择器（selector）的数，然后它将调用其他函数来获取组的名字和引脚。维护组的数据结构是驱动的责任，只是一个简单的例子——实际上你可能需要在你的组结构中加入更多条目，例如与每个关联的特定寄存器范围等等

## 引脚配置（Pin configuration

引脚有时可以以各种方式进行软件配置，主要与其用作输入或输出时的电子特性相关例如，你可以使一个输出引脚为高阻（Hi-Z），或“三态（tristate）”，意味着它实际上断开连接。你可以使用某个特定的电阻值将输入引脚连接VDD GND——上拉和下拉—这样当没有东西驱动它所连接的线路，或者它未连接时，引脚有一个稳定值
引脚配置可以通过将配置条目添加到映射表中来编程；参见下文 `Board/machine
configuration`_ 一节
上面提到的配置参PLATFORM_X_PULL_UP 的格式和含义完全由引脚控制器驱动定义
引脚配置驱动实现用于更改引脚控制ops 中引脚配置的回调，像这样

	#include <linux/pinctrl/pinconf.h>
	#include <linux/pinctrl/pinctrl.h>

	#include "platform_x_pindefs.h"

	static int foo_pin_config_get(struct pinctrl_dev *pctldev,
				      unsigned int offset,
				      unsigned long *config)
	{
		struct my_conftype conf;

		/** ... Find setting for pin @ offset ... **/

		*config = (unsigned long) conf;
	}

	static int foo_pin_config_set(struct pinctrl_dev *pctldev,
				      unsigned int offset,
				      unsigned long config)
	{
		struct my_conftype **conf = (struct my_conftype **) config;

		switch (conf) {
			case PLATFORM_X_PULL_UP:
			...
			break;
		}
	}

	static int foo_pin_config_group_get(struct pinctrl_dev *pctldev,
					    unsigned selector,
					    unsigned long *config)
	{
		...
	}

	static int foo_pin_config_group_set(struct pinctrl_dev *pctldev,
					    unsigned selector,
					    unsigned long config)
	{
		...
	}

	static struct pinconf_ops foo_pconf_ops = {
		.pin_config_get = foo_pin_config_get,
		.pin_config_set = foo_pin_config_set,
		.pin_config_group_get = foo_pin_config_group_get,
		.pin_config_group_set = foo_pin_config_group_set,
	};

	/** Pin config operations are handled by some pin controller **/
	static struct pinctrl_desc foo_desc = {
		...
		.confops = &foo_pconf_ops,
	};

## GPIO 子系统的交互


GPIO 驱动可能想要在同样注册为引脚控制器引脚的相同物理引脚上执行各种类型的操作
首先且最重要的是，这两个子系统可以完全正交地使用，参见名`Pin control requests
from drivers`_ `Drivers needing both pin control and GPIOs`_ 的小节以了解详情但在某些情况下，引脚GPIO 之间的跨子系统映射是需要的
由于引脚控制器子系统的引脚空间（pinspace）对于引脚控制器是局部的，我们需要一映射，以便引脚控制子系统能够弄清楚哪个引脚控制器处理某个 GPIO 引脚的控制。由单个引脚控制器可能正在多路复用多GPIO 范围（通常是这样一SoC：它有一组引脚，
但内部有多个 GPIO 硅模块，每个都被建模为一struct gpio_chip），因此可以将任数量GPIO 范围添加到引脚控制器实例中，像这样：


	#include <linux/gpio/driver.h>

	#include <linux/pinctrl/pinctrl.h>

	struct gpio_chip chip_a;
	struct gpio_chip chip_b;

	static struct pinctrl_gpio_range gpio_range_a = {
		.name = "chip a",
		.id = 0,
		.base = 32,
		.pin_base = 32,
		.npins = 16,
		.gc = &chip_a,
	};

	static struct pinctrl_gpio_range gpio_range_b = {
		.name = "chip b",
		.id = 0,
		.base = 48,
		.pin_base = 64,
		.npins = 8,
		.gc = &chip_b;
	};

	int __init foo_init(void)
	{
		struct pinctrl_dev *pctl;
		...
		pinctrl_add_gpio_range(pctl, &gpio_range_a);
		pinctrl_add_gpio_range(pctl, &gpio_range_b);
		...
	}

因此这个复杂的系统有一个引脚控制器处理两个不同GPIO 芯片。“chip a”有 16 引脚，“chip b”有 8 个引脚。“chip a”和“chip b”有不同`pin_base`，这意味着 GPIO
范围的起始引脚号
“chip a”的 GPIO 范围GPIO 基址 32 开始，实际的引脚范围也32 开始。然“chip b”的 GPIO 范围和引脚范围有不同的起始偏移。“chip b”的 GPIO 范围GPIO 编号
48 开始，而“chip b”的引脚范围64 开始
我们可以使用这个 `pin_base` 将一gpio 编号转换为实际的引脚编号。它们在全局 GPIO
引脚空间中映射为
chip a:
 - GPIO range : [32 .. 47]
 - pin range  : [32 .. 47]
chip b:
 - GPIO range : [48 .. 55]
 - pin range  : [64 .. 71]

上面的例子假GPIO 与引脚之间的映射是线性的。如果映射是稀疏的或随意的，一任意引脚编号数组可以像这样编码到范围中：


	static const unsigned int range_pins[] = { 14, 1, 22, 17, 10, 8, 6, 2 };

	static struct pinctrl_gpio_range gpio_range = {
		.name = "chip",
		.id = 0,
		.base = 32,
		.pins = &range_pins,
		.npins = ARRAY_SIZE(range_pins),
		.gc = &chip,
	};

在这种情况下，`pin_base` 属性将被忽略。如果已知一个引脚组的名字，上述结构pins npins 元素可以使用函数 `pinctrl_get_group_pins()` 来初始化，例如对于引脚组 “foo”：


	pinctrl_get_group_pins(pctl, "foo", &gpio_range.pins, &gpio_range.npins);

当引脚控制子系统中与 GPIO 相关的函数被调用时，这些范围将被用来通过检查并将引脚与所控制器上的引脚范围进行匹配来查找合适的引脚控制器。当找到处理匹配范围的引脚控制器时，
将在该特定引脚控制器上调用与 GPIO 相关的函数
对于所有涉及引脚偏置、引脚多路复用等的功能，引脚控制器子系统将通过传入gpio 编号
查找相应的引脚编号，并使用该范围的内部来检索一个引脚编号。之后，子系统将其传递给引脚
控制驱动，以便驱动将得到一个在其处理编号范围内的引脚编号。此外还会传递范ID 值，以便
引脚控制器知道它应该处理哪个范围
pinctrl 驱动调用 `pinctrl_add_gpio_range()` *已弃用（DEPRECATED*的。请参阅
`Documentation/devicetree/bindings/gpio/gpio.txt` 的第 2.1 节，了解如何绑定 pinctrl gpio 驱动

## PINMUX 接口


这些调用使用 pinmux_* 命名前缀。其他调用都不应使用该前缀

## 什么是引脚多路复用（pinmuxing）？


PINMUX，也称为 padmux、ballmux、复用功能（alternate functions）或任务模式（mission
modes），是生产某种电气封装的芯片厂商使用某个特定物理引脚（球、焊盘、管脚等）来用于
多个互斥功能的一种方式，具体取决于应用。在这个上下文中，我们所说的“应用”通常是指封装焊接或布线到电子系统中的一种方式，尽管该框架也使得在运行时改变功能成为可能
```

        A   B   C   D   E   F   G   H
      +---+
   8  | o | o   o   o   o   o   o   o
      |   |
   7  | o | o   o   o   o   o   o   o
      |   |
   6  | o | o   o   o   o   o   o   o
      +---+---+
   5  | o | o | o   o   o   o   o   o
      +---+---+               +---+
   4    o   o   o   o   o   o | o | o
                              |   |
   3    o   o   o   o   o   o | o | o
                              |   |
   2    o   o   o   o   o   o | o | o
      +-------+-------+-------+---+---+
   1  | o   o | o   o | o   o | o | o |
      +-------+-------+-------+---+---+

```
这不是俄罗斯方块。要联想的游戏是象棋。并非所有的 PGA/BGA 封装都像棋盘一样，大的封装
会根据不同的设计模式有一些“空洞”，但我们这里用作一个简单的例子。在你能看到的引脚中有些会被诸如几个 VCC GND 用来给芯片供电，还有相当多会被大的端口（如外部存储器接口占用。剩下的引脚通常会受到引脚多路复用的影响
上面这个 8x8 PGA 封装将为其物理引脚分配引脚编0 63。它将使pinctrl_register_pins() 和如前面所示的一组合适数据，将引脚命名为 { A1, A2, A3 ...
H6, H7, H8 }銆。
在这8x8 BGA 封装中，引脚 { A8, A7, A6, A5 } 可以用作一SPI 端口（这是四个引脚：
CLK、RXD、TXD、FRM）。在这种情况下，引脚 B5 可以用作某个通用GPIO 引脚。然而，在另一设置中，引脚 { A5, B5 } 可以用作一I2C 端口（这只是两个引脚：SCL、SDA）。不用说，我不能同时使用SPI 端口I2C 端口。然而在封装内部，执SPI 逻辑的硅可以改道输出到引{ G4, G3, G2, G1 }
在最下面一{ A1, B1, C1, D1, E1, F1, G1, H1 } 我们有一些特别的东西——它是一个外部的
MMC 总线，可以是 2 8 位宽，并会分别消2 8 个引脚，因此要么占用 { A1, B1 }要么占用 { A1, B1, C1, D1 }，要么全部占用。如果我们使用全8 位，我们当然就不能使引脚 { G4, G3, G2, G1 } 上的 SPI 端口
通过这种方式，芯片内部存在的硅模块可以被多路复用（“muxed”）输出到不同的引脚范围。当SoC（片上系统）通常包含多个 I2C、SPI、SDIO/MMC 等硅模块，可以通过 pinmux 设置路由到不同的
引脚
由于通用输入/输出引脚（GPIO）通常总是短缺，通常如果某个引脚当前没有被其I/O 端口使用就可以将它用GPIO 引脚

## Pinmux 约定


引脚控制器子系统pinmux 功能的目的是，为你选择在机器配置中实例化的设备抽象并提pinmux 设置。它受到 clk、GPIO regulator 子系统的启发，因此设备将请求它们mux 设置但也可以为例GPIO 请求单个引脚
约定如下
- 函数（FUNCTION）可以由驻留在内`drivers/pinctrl` 目录中的引脚控制子系统内的驱  切换进出。引脚控制驱动知道可能的功能。在上面的例子中，你可以识别出三pinmux 功能  一个用spi，一个用i2c，一个用mmc
- 函数（FUNCTION）被假定为从一个一维数组中从零开始可枚举。在这种情况下，数组可能是像
  { spi0, i2c0, mmc0 } 这样的东西，对应三个可用的功能
- 函数（FUNCTION）具有在通用层面定义的引脚组（PIN GROUP）——因此某个特定函*总是**
  与某个特定的引脚组集合相关联，可能只有一个，但也可能有很多。在上面的例子中，函i2c
  与引{ A5, B5 } 相关联，在控制器引脚空间中枚举为 { 24, 25 }
  函数 spi 与引脚组 { A8, A7, A6, A5 } { G4, G3, G2, G1 } 相关联，分别枚举  { 0, 8, 16, 24 } { 38, 46, 54, 62 }
  组名对于每个引脚控制器必须是唯一的，同一个控制器上不能有两个同名的组
- 函数（FUNCTION）和引脚组（PIN GROUP）的组合决定了某组引脚的某个特定功能。函数和引脚  及其机器特定细节的知识保存在 pinmux 驱动内部，从外部只知道枚举器，驱动核心可以请求：

  - 具有某个选择器（>= 0）的函数  - 与某个特定函数关联的组列  - 该列表中某个特定组被激活以用于某个特定函数

  如上所述，引脚组本身又是自描述的，因此核心将从驱动中检索某个组中实际的引脚范围
- 某个引脚控制器上的函数（FUNCTION）和组（GROUP）通过板文件、设备树或类似的机器设置配置
  机制被映射（MAP）到某个特定设备，类似于 regulator 如何连接到设备，通常按名字。定义引  控制器、函数和组从而唯一标识某个设备要使用的引脚集合。（如果该函数只有一个可能的引脚  可用，则无需提供组名——核心将简单地选择第一个也是唯一可用的组。）

  在例子中，我们可以定义这台特定的机器应使用设spi0，配pinmux 函数 fspi0、组 gspi0  以及 i2c0，配合函fi2c0、组 gi2c0，在主引脚控制器上，我们得到如下映射
  .. code-block:: c

	{
		{"map-spi0", spi0, pinctrl0, fspi0, gspi0},
		{"map-i2c0", i2c0, pinctrl0, fi2c0, gi2c0},
	}

  每个映射都必须被分配一个状态名、引脚控制器、设备和函数。组不是强制的——如果省略，驱动
  所呈现的适用于该函数第一个组将被选中，这对简单情况很有用
  可以将多个组映射到相同的设备、引脚控制器和函数的组合。这是针对某个引脚控制器上的某个
  特定功能在不同配置中可以使用不同引脚集合的情况
- 某个引脚控制器上、使用某个引脚组的某个函数（FUNCTION）的引脚（PINS）按先到先得  first-come first-serve）的原则提供，因此如果某个其他设mux 设置GPIO 引脚请求已经
  占用了你的物理引脚，你将无法使用它。要获取（激活）一个新设置，必须先将旧的释放（停用）
有时文档和硬件寄存器会围绕焊盘（pad，或“管finger”）而不是引脚来组织——这些是封装硅上的焊接面，可能与外壳下面的实际引球数量匹配或不匹配。选择对你有意义的某种枚举方式如果讲得通，只为你能够控制的引脚定义枚举器
假设
我们假设可能的功能到引脚组的映射数量受硬件限制。即我们假设不存在某个功能可以被映射到任引脚的系统，就像电话交换机那样。因此某个特定功能的可用引脚组将限于少数几种选择（比如最八个左右），而不是数百种或任意数量的选择。这是我们通过检查可用的 pinmux 硬件所发现的特性，
并且是一个必要的假设，因为我们期pinmux 驱动向子系统呈现**所*可能的功能与引脚组的
映射

## Pinmux 驱动


pinmux 核心负责防止引脚上的冲突，并调用引脚控制器驱动来执行不同的设置
pinmux 驱动有责任施加进一步的限制（例如推断由于负载等带来的电子限制），以确定所请求功能是否确实被允许，并且在可以执行所请求mux 设置的情况下，去拨动（poke）硬件以使其发生
Pinmux 驱动需要提供一些回调函数，有些是可选的。通常实现 `.set_mux()` 函数，将值写入某特定寄存器以激活某个引脚的特定 mux 设置
对上述例子的一个简单驱动将通过设置0 5 到某个名MUX 的寄存器来选择某个
具有特定引脚组的功能，大致像这样

	#include <linux/pinctrl/pinctrl.h>
	#include <linux/pinctrl/pinmux.h>

	static const unsigned int spi0_0_pins[] = { 0, 8, 16, 24 };
	static const unsigned int spi0_1_pins[] = { 38, 46, 54, 62 };
	static const unsigned int i2c0_pins[] = { 24, 25 };
	static const unsigned int mmc0_1_pins[] = { 56, 57 };
	static const unsigned int mmc0_2_pins[] = { 58, 59 };
	static const unsigned int mmc0_3_pins[] = { 60, 61, 62, 63 };

	static const struct pingroup foo_groups[] = {
		PINCTRL_PINGROUP("spi0_0_grp", spi0_0_pins, ARRAY_SIZE(spi0_0_pins)),
		PINCTRL_PINGROUP("spi0_1_grp", spi0_1_pins, ARRAY_SIZE(spi0_1_pins)),
		PINCTRL_PINGROUP("i2c0_grp", i2c0_pins, ARRAY_SIZE(i2c0_pins)),
		PINCTRL_PINGROUP("mmc0_1_grp", mmc0_1_pins, ARRAY_SIZE(mmc0_1_pins)),
		PINCTRL_PINGROUP("mmc0_2_grp", mmc0_2_pins, ARRAY_SIZE(mmc0_2_pins)),
		PINCTRL_PINGROUP("mmc0_3_grp", mmc0_3_pins, ARRAY_SIZE(mmc0_3_pins)),
	};

	static int foo_get_groups_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_groups);
	}

	static const char **foo_get_group_name(struct pinctrl_dev **pctldev,
					      unsigned int selector)
	{
		return foo_groups[selector].name;
	}

	static int foo_get_group_pins(struct pinctrl_dev *pctldev, unsigned int selector,
				      const unsigned int **pins,
				      unsigned int *npins)
	{
		*pins = foo_groups[selector].pins;
		*npins = foo_groups[selector].npins;
		return 0;
	}

	static struct pinctrl_ops foo_pctrl_ops = {
		.get_groups_count = foo_get_groups_count,
		.get_group_name = foo_get_group_name,
		.get_group_pins = foo_get_group_pins,
	};

	static const char * const spi0_groups[] = { "spi0_0_grp", "spi0_1_grp" };
	static const char * const i2c0_groups[] = { "i2c0_grp" };
	static const char * const mmc0_groups[] = { "mmc0_1_grp", "mmc0_2_grp", "mmc0_3_grp" };

	static const struct pinfunction foo_functions[] = {
		PINCTRL_PINFUNCTION("spi0", spi0_groups, ARRAY_SIZE(spi0_groups)),
		PINCTRL_PINFUNCTION("i2c0", i2c0_groups, ARRAY_SIZE(i2c0_groups)),
		PINCTRL_PINFUNCTION("mmc0", mmc0_groups, ARRAY_SIZE(mmc0_groups)),
	};

	static int foo_get_functions_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_functions);
	}

	static const char **foo_get_fname(struct pinctrl_dev **pctldev, unsigned int selector)
	{
		return foo_functions[selector].name;
	}

	static int foo_get_groups(struct pinctrl_dev *pctldev, unsigned int selector,
				  const char * const **groups,
				  unsigned int * const ngroups)
	{
		*groups = foo_functions[selector].groups;
		*ngroups = foo_functions[selector].ngroups;
		return 0;
	}

	static int foo_set_mux(struct pinctrl_dev *pctldev, unsigned int selector,
			       unsigned int group)
	{
		u8 regbit = BIT(group);

		writeb((readb(MUX) | regbit), MUX);
		return 0;
	}

	static struct pinmux_ops foo_pmxops = {
		.get_functions_count = foo_get_functions_count,
		.get_function_name = foo_get_fname,
		.get_function_groups = foo_get_groups,
		.set_mux = foo_set_mux,
		.strict = true,
	};

	/** Pinmux operations are handled by some pin controller **/
	static struct pinctrl_desc foo_desc = {
		...
		.pctlops = &foo_pctrl_ops,
		.pmxops = &foo_pmxops,
	};

在例子中，同时激muxing 0 2 设置0 2，共用了引脚 24，因此它们会冲突。对muxes 1 5 也一样，它们共用了引62
pinmux 子系统的美妙之处在于，由于它跟踪所有引脚以及谁在使用它们，它早已拒绝了这样一不可能的请求，因此驱动无需担心这类事情——当它传入一个选择器时，pinmux 子系统确保没有其设备GPIO 分配已经在使用所选的引脚。因此控制寄存器中的0 2，或 1 5，永远不会被
同时设置
以上所有函数对pinmux 驱动来说都是必须实现的

## 引脚控制GPIO 子系统的交互


注意，以下内容暗示了使用场景是在 Linux 内核中使`<linux/gpio/consumer.h>` 中的 API配合 gpiod_get() 及类似函数。有些情况下你可能正在使用你的数据手册称为“GPIO 模式”的东西但实际上只是某个设备的电气配置。参见下文的 `GPIO mode pitfalls`_ 小节，了解更多关于此
场景的细节
公共 pinmux API 包含两个名为 `pinctrl_gpio_request()` `pinctrl_gpio_free()` 的函数这两个函*只能**从基gpiolib 的驱动中调用，作为它们的 `.request()` `.free()` 语义
的一部分。同样地，`pinctrl_gpio_direction_input()` / `pinctrl_gpio_direction_output()`
只能分别在各 gpiolib `.direction_input()` / `.direction_output()` 实现内部调用
注意，平台和各个驱动**不应**请求 GPIO 引脚受控，例如被 mux 进来。相反，实现一个合适的
gpiolib 驱动，并让该驱动为它的引脚请求合适的 muxing 和其他控制
函数列表可能会变得很长，特别是如果你能将每个单独的引脚转换为一GPIO 引脚而不依赖于任何其引脚，然后尝试用定义每个引脚为一个函数的方法
在这种情况下，函数数组将变成每个 GPIO 设置64 个条目，然后是设备功能
因此有两个函数可供引脚控制驱动实现，以仅在单个引脚上启用 GPIO：`.gpio_request_enable()`
鍜?`.gpio_disable_free()`銆。
这个函数将传入由引脚控制器核心标识的受影响的 GPIO 范围，因此你知道哪些 GPIO 引脚受到请求操作的影响
如果你的驱动需要来自框架的关于 GPIO 引脚应用于输入还是输出的指示，你可以实现
`.gpio_set_direction()` 函数。如前所述，这将gpiolib 驱动中被调用，受影响GPIO 范围引脚偏移和期望方向将被传递给该函数
作为使用这些特殊函数的替代方案，完全允许为每GPIO 引脚使用命名函数，`pinctrl_gpio_request()`
将尝试获取名“gpioN的函数，其中 “N是全局 GPIO 引脚编号，前提是没有注册特殊GPIO 处理器

## GPIO 模式陷阱（GPIO mode pitfalls

由于硬件工程师使用的命名约定，其“GPIO的含义与内核所做的不同，开发者可能会被数据手谈论某个引脚可以被设置为 “GPIO 模式所困惑。看起来硬件工程师所说的 “GPIO 模式并不一是内核接`<linux/gpio/consumer.h>` 所暗示的使用场景：一个你从内核代码中抓取、然后要监听输入、要么驱动高/低以断言/取消断言某个外部线路的引脚
相反，硬件工程师认为 “GPIO 模式意味着你可以软件控制引脚的一些电气特性，而如果引脚处其他模式（例如被 mux 进某个设备）时你将无法控制这些特性
一个引脚的 GPIO 部分及其与某个引脚控制器配置muxing 逻辑的关系可以用几种方式构建。这有两个例子
```

                       pin config
                       logic regs
                       |               +- SPI
     Physical pins --- pad --- pinmux -+- I2C
                               |       +- mmc
                               |       +- GPIO
                               pin
                               multiplex
                               logic regs

```
这里引脚的一些电气特性无论引脚是否用GPIO 都可以被配置。如果你将一GPIO 多路复用到一引脚上，你也可以“GPIO寄存器驱动它低。或者，该引脚可以被某个特定外设控制，同时仍应用所需的引脚配置属性。因GPIO 功能与使用该引脚的任何其他设备是正交的
在这种安排中，引脚控制器GPIO 部分寄存器，GPIO 硬件模块的寄存器，可能位于一个仅GPIO
驱动的单独内存范围中，而处理引脚配置和引脚多路复用的寄存器范围被放在一个不同的内存范围和数手册的不同章节中
struct pinmux_ops 中有一个标“strict”，可用于检查和拒绝来自 GPIO 和引脚多路复消费者在同一类型硬件上同时访问同一个引脚。pinctrl 驱动应相应地设置此标志
```

                       pin config
                       logic regs
                       |               +- SPI
     Physical pins --- pad --- pinmux -+- I2C
                       |       |       +- mmc
                       |       |
                       GPIO    pin
                               multiplex
                               logic regs

```
在这种安排中，GPIO 功能总是可以被启用，例如一GPIO 输入可以用来“窥探（spy）”正在脉冲输SPI/I2C/MMC 信号。由于它从未真正断开连接，通过 GPIO 块上做错事来干扰引脚上的流量是可的。GPIO、引脚配置和引脚多路复用寄存器可能被放在同一个内存范围和数据手册的同一个章节中，尽并不一定非得如此
在一些引脚控制器中，虽然物理引脚的设计与（B）相同，GPIO 功能仍然不能与外设功能同时启用因此同样应该设置 “strict标志，拒GPIO 和其mux 进来的设备同时激活
然而，从内核的角度来看，这些是硬件的不同方面，应该被放到不同的子系统中
- 控制引脚电气特性（如偏置和驱动强度）的寄存器（或寄存器中的字段）应该通过 pinctrl 子系  作为“引脚配置（pin configuration）”设置暴露
- 控制来自各种其他硬件模块（例I2C、MMC GPIO）的信号到引脚的多路复用的寄存器（或寄存  中的字段）应该通过 pinctrl 子系统作mux 功能暴露
- 控制 GPIO 功能（例如设GPIO 的输出值、读GPIO 的输入值，或设GPIO 引脚方向）的寄存  （或寄存器中的字段）应该通过 GPIO 子系统暴露，如果它们还支持中断能力，则通过 irqchip 抽象
  暴露
根据确切的硬件寄存器设计，GPIO 子系统暴露的一些功能可能会调用 pinctrl 子系统，以协调跨硬件
模块的寄存器设置。特别是，对于具有独GPIO 和引脚控制器硬件模块、其中例GPIO 方向由引控制器硬件模块中的寄存器而不GPIO 硬件模块决定的硬件，这可能就是必需的
引脚的电气特性，例如偏置和驱动强度，在所有情况下可能被放在某个引脚特定的寄存器中，或者在（B情况中尤其是作为 GPIO 寄存器的一部分。这并不意味着此类特性必然属Linux 内核所称的 “GPIO”
例子：一个引脚通常mux 进来用作 UART TX 线。但在系统休眠期间，我们需要将这个引脚置于
“GPIO 模式”并将其接地
如果你为这个引脚做一对一映射GPIO 子系统，你可能会开始认为你需要想出某种真正复杂的东西，即
该引脚要同时用于 UART TX GPIO，你将抓取一个引脚控制句柄并将其设置为某个状态以启用 UART TX
mux 进来，然后将其切换到 GPIO 模式并使gpiod_direction_output() 在休眠期间将其驱动为低，
然后在唤醒时再将mux UART TX，甚至可能在这个循环中用gpiod_get() / gpiod_put()。这一变得非常复杂
解决方案是不要认为数据手册所称的 “GPIO 模式必须`<linux/gpio/consumer.h>` 接口来处理。相将其视为某个特定的引脚配置设置。例如看`<linux/pinctrl/pinconf-generic.h>`，你会在文档中找这个
  PIN_CONFIG_LEVEL:
     这将把引脚配置为输出，使用参1 表示高电平，参数 0 表示低电平
因此完全可以将一个引脚推“GPIO 模式并作为通常引脚控制映射的一部分将线路驱动为低。所以例你的 UART 驱动可能看起来像这样

	#include <linux/pinctrl/consumer.h>

	struct pinctrl          *pinctrl;
	struct pinctrl_state    *pins_default;
	struct pinctrl_state    *pins_sleep;

	pins_default = pinctrl_lookup_state(uap->pinctrl, PINCTRL_STATE_DEFAULT);
	pins_sleep = pinctrl_lookup_state(uap->pinctrl, PINCTRL_STATE_SLEEP);

	/** Normal mode **/
	retval = pinctrl_select_state(pinctrl, pins_default);

	/** Sleep mode **/
	retval = pinctrl_select_state(pinctrl, pins_sleep);

而你的机器配置可能看起来像这样：


	static unsigned long uart_default_mode[] = {
		PIN_CONF_PACKED(PIN_CONFIG_DRIVE_PUSH_PULL, 0),
	};

	static unsigned long uart_sleep_mode[] = {
		PIN_CONF_PACKED(PIN_CONFIG_LEVEL, 0),
	};

	static struct pinctrl_map pinmap[] __initdata = {
		PIN_MAP_MUX_GROUP("uart", PINCTRL_STATE_DEFAULT, "pinctrl-foo",
				  "u0_group", "u0"),
		PIN_MAP_CONFIGS_PIN("uart", PINCTRL_STATE_DEFAULT, "pinctrl-foo",
				    "UART_TX_PIN", uart_default_mode),
		PIN_MAP_MUX_GROUP("uart", PINCTRL_STATE_SLEEP, "pinctrl-foo",
				  "u0_group", "gpio-mode"),
		PIN_MAP_CONFIGS_PIN("uart", PINCTRL_STATE_SLEEP, "pinctrl-foo",
				    "UART_TX_PIN", uart_sleep_mode),
	};

	foo_init(void)
	{
		pinctrl_register_mappings(pinmap, ARRAY_SIZE(pinmap));
	}

这里我们要控制的引脚“u0_group中，并且有一个名“u0的功能可以在这个引脚组上启用，然一切就像平常的 UART 业务。但还有一个名“gpio-mode的功能可以被映射到相同的引脚，将它们移入
GPIO 模式
这将产生期望的效果，而无需任何GPIO 子系统的虚假交互。它只是该设备在进入休眠时使用的电气
配置，它可能意味着该引脚被设置为数据手册所称的 “GPIO 模式”，但这不是重点：它仍然被那UART
设备用来控制属于UART 驱动器的引脚，将它们置于 UART 所需的状态。Linux 内核意义上的 GPIO 只是
某种 1 位的线，是一个不同的使用场景
寄存器如何被拨动（poke）以达到推或拉、输出低配置以及“u0“gpio-modemux 到这些引上，是驱动要解决的问题
一些数据手册会更有帮助，将 “GPIO 模式称为 “低功耗模式”，而不是任何与 GPIO 有关的东西。后者在
电气上通常意味着相同的东西，但在后一种情况下，软件工程师通常会迅速识别出这是某个特定muxing
或配置，而不是与 GPIO API 相关的任何东西

## 机器配置（Board/machine configuration

板和机器定义了某个完整的运行系统是如何组合在一起的，包GPIO 和设备是如何 mux 的、regulator
是如何受约束的，以及时钟树是什么样子。当pinmux 设置也是其中的一部分
一个机器的引脚控制器配置看起来非常像一个简单的 regulator 配置，因此对于上面的示例数组，我想要在第二个功能映射上启i2c spi

	#include <linux/pinctrl/machine.h>

	static const struct pinctrl_map mapping[] __initconst = {
		{
			.dev_name = "foo-spi.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "spi0",
		},
		{
			.dev_name = "foo-i2c.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "i2c0",
		},
		{
			.dev_name = "foo-mmc.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "mmc0",
		},
	};

这里dev_name 匹配可用于查找设struct 的唯一设备名（就像 clockdev regulator 那样）函数名必须匹配处理此引脚范围pinmux 驱动提供的函数
如你所见，我们系统上可能有多个引脚控制器，因此我们需要指定其中包含我们要映射的功能的那一个
你只需通过以下方式将这pinmux 映射注册pinmux 子系统：


       ret = pinctrl_register_mappings(mapping, ARRAY_SIZE(mapping));

由于上述构造相当常见，有一个辅助宏可以让它更紧凑，该宏假设你想要使pinctrl-foo 和位0
进行映射，例如：


	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_MUX_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				  "pinctrl-foo", NULL, "i2c0"),
	};

映射表也可能包含引脚配置条目。每个引组通常有一组影响其的配置条目，因此用于配置的表条目
引用一个配置参数和值的数组。一个使用便捷宏的例子如下所示：


	static unsigned long i2c_grp_configs[] = {
		FOO_PIN_DRIVEN,
		FOO_PIN_PULLUP,
	};

	static unsigned long i2c_pin_configs[] = {
		FOO_OPEN_COLLECTOR,
		FOO_SLEW_RATE_SLOW,
	};

	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_MUX_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				  "pinctrl-foo", "i2c0", "i2c0"),
		PIN_MAP_CONFIGS_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				      "pinctrl-foo", "i2c0", i2c_grp_configs),
		PIN_MAP_CONFIGS_PIN("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				    "pinctrl-foo", "i2c0scl", i2c_pin_configs),
		PIN_MAP_CONFIGS_PIN("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				    "pinctrl-foo", "i2c0sda", i2c_pin_configs),
	};

最后，一些设备期望映射表包含某些特定的命名状态。当运行在不需要任何引脚控制器配置的硬件上时，
映射表仍然必须包含那些命名状态，以明确表明这些状态被提供并意图为空。表条目`PIN_MAP_DUMMY_STATE()` 用于定义一个命名状态而不导致任何引脚控制器被编程

	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_DUMMY_STATE("foo-i2c.0", PINCTRL_STATE_DEFAULT),
	};


## 复杂映射（Complex mappings

由于可以将一个功能映射到不同的引脚组，可以像这样指定一个可选的 .group

	...
	{
		.dev_name = "foo-spi.0",
		.name = "spi0-pos-A",
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "spi0",
		.group = "spi0_0_grp",
	},
	{
		.dev_name = "foo-spi.0",
		.name = "spi0-pos-B",
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "spi0",
		.group = "spi0_1_grp",
	},
	...

这个示例映射用于在运行时spi0 的两个位置之间切换，`Runtime pinmuxing`_ 标题下进一描述
此外，一个命名状态可能影响多个引脚组muxing，例如上mmc0 的例子中，你可以mmc0 总线2 位加法式扩展4 位再8 位。如果我们想对总共 2 + 2 + 4 = 8 个引脚（8 MMC 总线的情况）
使用全部三个组，我们定义一个如下的映射

	...
	{
		.dev_name = "foo-mmc.0",
		.name = "2bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "4bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "4bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_2_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_2_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_3_grp",
	},
	...

用像这样（见下一段）从设备中抓取此映射的结果为：


	p = devm_pinctrl_get(dev);
	s = pinctrl_lookup_state(p, "8bit");
	ret = pinctrl_select_state(p, s);

或者更简单：


	p = devm_pinctrl_get_select(dev, "8bit");

结果将是你一次激活映射中全部三个底部记录。由于它们共享相同的名字、引脚控制器设备、功能和设备并且由于我们允许将多个组匹配到单个设备，它们全部被选中，并且全部由 pinmux 核心同时启用禁用

## 来自驱动的引脚控制请求（Pin control requests from drivers

当设备驱动即将探测（probe）时，如果设备树中定义了标准状态，设备核心会通过调用
`pinctrl_bind_pins()` 将这些状态附加到这些设备上。可能的标准状态名有：“default”、“init”“sleep“idle”
- 如果 `default` 在设备树中定义，它会在设备探测之前被选中
- 如果 `init` `default` 都在设备树中定义，则 “init状态在驱动探测之前被选中，“default  状态在驱动探测之后被选中
- `sleep` `idle` 状态用于电源管理，只能使用下面PM API 选中
## PM 接口


PM 运行时挂恢复（runtime suspend/resume）可能需要执行与探测期间相同的初始化序列。由于预定义
状态已经附加到设备，驱动可以使用以下辅助函数显式激活这些状态：

- `pinctrl_pm_select_default_state()`
- `pinctrl_pm_select_init_state()`
- `pinctrl_pm_select_sleep_state()`
- `pinctrl_pm_select_idle_state()`

例如，如果恢复设备依赖于某些 pinmux 状

	foo_suspend()
	{
		/** suspend device **/
		...

		pinctrl_pm_select_sleep_state(dev);
	}

	foo_resume()
	{
		pinctrl_pm_select_init_state(dev);

		/** resuming device **/
		...

		pinctrl_pm_select_default_state(dev);
	}

这样驱动编写者无需添加下面这类样板代码。然而，当进行细粒度的状态选择而不使用 “default状态时你可能需要做一些设备驱动对 pinctrl 句柄和状态的处理
所以如果你只是想将某个设备的引脚置于默认状态并就此了事，除了提供正确的映射表之外，你无需做其任何事情。设备核心会处理其余部分
通常不建议让各个驱动去获取和启用引脚控制。因此如果可能，在平台代码或你能访问所有受影响 struct
device * 指针的其他地方处理引脚控制。在某些情况下，当驱动需要在运行时切换不同的 mux 映射时，是不可能的
一个典型情况是驱动需要在正常操作和进入休眠之间切换引脚的偏置，从 `PINCTRL_STATE_DEFAULT` 移动`PINCTRL_STATE_SLEEP`，在运行时重新偏置甚至重mux 引脚以在休眠模式下节省电流
另一种情况是 pinctrl 需要在探测期间切换到某个模式，然后在探测结束时恢复到默认状态。例如，一PINMUX 可能需要在探测期间被配置为 GPIO。在这种情况下，使用 `PINCTRL_STATE_INIT` 在探测前切换状态，
然后在探测结束时移动`PINCTRL_STATE_DEFAULT` 以进行正常操作
驱动可以像这样请求激活某个控制状态，通常只是默认状态：


	#include <linux/pinctrl/consumer.h>

	struct foo_state {
	struct pinctrl *p;
	struct pinctrl_state *s;
	...
	};

	foo_probe()
	{
		/** Allocate a state holder named "foo" etc **/
		struct foo_state *foo = ...;
		int ret;

		foo->p = devm_pinctrl_get(&device);
		if (IS_ERR(foo->p)) {
			ret = PTR_ERR(foo->p);
			foo->p = NULL;
			return ret;
		}

		foo->s = pinctrl_lookup_state(foo->p, PINCTRL_STATE_DEFAULT);
		if (IS_ERR(foo->s)) {
			devm_pinctrl_put(foo->p);
			return PTR_ERR(foo->s);
		}

		ret = pinctrl_select_state(foo->p, foo->s);
		if (ret < 0) {
			devm_pinctrl_put(foo->p);
			return ret;
		}
	}

这个获取/查找/选择/释放序列同样可以由总线驱动处理，如果你不想让每个驱动都处理它，并且你清楚你总线上的安排
pinctrl API 的语义是
- `pinctrl_get()` 在进程上下文中被调用，以获取给定客户端设备的所pinctrl 信息的句柄。它将从
  内核内存分配一struct 来保pinmux 状态。所有映射表解析或类似的慢速操作都在此 API 内发生
- `devm_pinctrl_get()` pinctrl_get() 的一个变体，它在关联的设备被移除时自动调  `pinctrl_put()` 于所获取的指针上。建议使用此函数而非普通的 `pinctrl_get()`
- `pinctrl_lookup_state()` 在进程上下文中被调用，以获取客户端设备某个特定状态的句柄。这个操  也可能很慢
- `pinctrl_select_state()` 根据映射表给出的状态定义对引脚控制器硬件进行编程。理论上，这是一  快速路径操作，因为它只涉及将一些寄存器设置写入硬件。然而，请注意某些引脚控制器的寄存器可能位于
  慢基于 IRQ 的总线上，因此客户端设备不应假设它们可以在非阻塞上下文中调  `pinctrl_select_state()`
- `pinctrl_put()` 释放pinctrl 句柄关联的所有信息
- `devm_pinctrl_put()` `pinctrl_put()` 的一个变体，可用于显式销毁由 `devm_pinctrl_get()`
  返回pinctrl 对象。然而，由于即使不调用它也会发生自动清理，使用此函数的情况会很少
  `pinctrl_get()` 必须与普通的 `pinctrl_put()` 配对  `pinctrl_get()` 不得`devm_pinctrl_put()` 配对  `devm_pinctrl_get()` 可以选择`devm_pinctrl_put()` 配对  `devm_pinctrl_get()` 不得与普通的 `pinctrl_put()` 配对
通常引脚控制核心处理获取/释放对，并调用设备驱动的记账操作，如检查可用功能和关联的引脚，`pinctrl_select_state()` 传递给引脚控制器驱动，由后者负责通过快速拨动一些寄存器来激活和/停用 mux 设置
当你发出 `devm_pinctrl_get()` 调用时，会为你的设备分配引脚，之后你应该能在所有引脚的 debugfs
列表中看到这一点
注意：如果找不到请求pinctrl 句柄，例pinctrl 驱动尚未注册，pinctrl 系统将返`-EPROBE_DEFER`。因此请确保你的驱动中的错误路径能够优雅地清理，并准备好在启动过程的后期重试
探测

## 同时需要引脚控制和 GPIO 的驱动（Drivers needing both pin control and GPIOs

再次说明，不建议让驱动自己查找和选择引脚控制状态，但同样有时这是不可避免的
所以假设你的驱动像这样获取它的资源

	#include <linux/pinctrl/consumer.h>
	#include <linux/gpio/consumer.h>

	struct pinctrl *pinctrl;
	struct gpio_desc *gpio;

	pinctrl = devm_pinctrl_get_select_default(&dev);
	gpio = devm_gpiod_get(&dev, "foo");

这里我们首先请求某个引脚状态，然后请求使用 GPIO “foo”。如果你像这样正交地使用子系统，你通常应该
始终在请GPIO **之前**获取你的 pinctrl 句柄并选择所需pinctrl 状态。这是一个语义约定，用于
避免可能在电气上令人不快的情况，你肯定会想在 GPIO 子系统开始处理它们之前，以某种方mux 进来偏置引脚
以上可以隐藏：使用设备核心，pinctrl 核心可能会在设备探测之前就设置好引脚的配置和 muxing，然而这GPIO 子系统正交
但也存在这样的情况，GPIO 子系统直接与 pinctrl 子系统通信是有意义的，将后者用作后端。这就是 GPIO
驱动可能调用上文 `Pin control interaction with the GPIO subsystem`_ 一节中描述的函数的时候。这涉及每引脚的多路复用，并将完全隐藏在 gpiod_*() 函数命名空间之后。在这种情况下，驱动根本不需要与
引脚控制子系统交互
如果一个引脚控制驱动和一GPIO 驱动处理相同的引脚，并且使用场景涉及多路复用，你必须将引脚控制器
实现GPIO 驱动的后端，如下所示，除非你的硬件设计使得 GPIO 控制器可以通过硬件覆盖引脚控制器的
多路复用状态，而无需与引脚控制系统交互
如果引脚控制驱动GPIO 驱动处理相同的引脚，并且使用场景涉及多路复用，你**必须**将引脚控制器实现
GPIO 驱动的后端，除非你的硬件设计使得 GPIO 控制器可以通过硬件覆盖引脚控制器的多路复用状态，无需与引脚控制系统交互

## 系统引脚控制占用（System pin control hogging

当引脚控制器被注册时，引脚控制映射条目可以被核心占用（hogged）。这意味着核心将在引脚控制设备
注册之后立即尝试对其调用 `pinctrl_get()`、`pinctrl_lookup_state()` `pinctrl_select_state()`
这发生在映射表条目中客户端设备名等于引脚控制器设备名，且状态名`PINCTRL_STATE_DEFAULT` 的情况：


	{
		.dev_name = "pinctrl-foo",
		.name = PINCTRL_STATE_DEFAULT,
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "power_func",
	},

由于请求核心在主引脚控制器上占用一些始终适用mux 设置可能是常见的，有一个用于此的便捷宏

	PIN_MAP_MUX_GROUP_HOG_DEFAULT("pinctrl-foo", NULL /** group **/,
				      "power_func")

这会得到与上面构造完全相同的结果

## 运行时引脚多路复用（Runtime pinmuxing

可以在运行时将某个功mux 进来和出去，例如将一SPI 端口从一组引脚移动到另一组引脚。例如对上面spi0，我们为同一个功能暴露两个不同的引脚组，但在映射中使用不同的命名，如上文 “Advanced
mapping所述。因此对于一SPI 设备，我们有两个名为 “pos-A“pos-B的状态
这个片段首先为两个组（在 foo_probe() 中）初始化一个状态对象，然后在组 A 定义的引脚上 mux 进该
功能，最后在B 定义的引脚上 mux 进它

	#include <linux/pinctrl/consumer.h>

	struct pinctrl *p;
	struct pinctrl_state **s1, **s2;

	foo_probe()
	{
		/** Setup **/
		p = devm_pinctrl_get(&device);
		if (IS_ERR(p))
			...

		s1 = pinctrl_lookup_state(p, "pos-A");
		if (IS_ERR(s1))
			...

		s2 = pinctrl_lookup_state(p, "pos-B");
		if (IS_ERR(s2))
			...
	}

	foo_switch()
	{
		/** Enable on position A **/
		ret = pinctrl_select_state(p, s1);
		if (ret < 0)
			...

		...

		/** Enable on position B **/
		ret = pinctrl_select_state(p, s2);
		if (ret < 0)
			...

		...
	}

上述必须在进程上下文中完成。引脚的保留将在状态被激活时进行，因此实际上，在某个运行中的系统上，
一个特定引脚可以在不同时间被不同功能使用

## Debugfs 文件


这些文件`/sys/kernel/debug/pinctrl` 中创建：

- `pinctrl-devices`：打印每个引脚控制器设备以及指示是否支持 pinmux pinconf 的列

- `pinctrl-handles`：打印每个已配置的引脚控制器句柄及相应的 pinmux 映射

- `pinctrl-maps`：打印所pinctrl 映射

`/sys/kernel/debug/pinctrl` 内部为每个引脚控制器设备创建一个子目录，包含这些文件：

- `pins`：为引脚控制器上注册的每个引脚打印一行。pinctrl 驱动可以添加额外信息，例如寄存器内容
- `gpio-ranges`：打印将 gpio 线映射到控制器上引脚的范
- `pingroups`：打印引脚控制器上注册的所有引脚组

- `pinconf-pins`：为每个引脚打印引脚配置设置

- `pinconf-groups`：按引脚组打印引脚配置设
- `pinmux-functions`：打印每个引脚功能以及映射到该引脚功能的引脚
- `pinmux-pins`：遍历所有引脚并打印 mux 拥有者、gpio 拥有者以及该引脚是否hog

- `pinmux-select`：写入此文件以激活某个组的引脚功能：

  .. code-block:: sh

        echo "<group-name function-name>" > pinmux-select
