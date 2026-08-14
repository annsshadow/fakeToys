## PXA2xx/PXA3xx 处理器的 MFP 配置


			Eric Miao <eric.miao@marvell.com>

MFP 是 Multi-Function Pin（多功能引脚）的缩写，是 PXA3xx 及后续 PXA
系列处理器上的引脚复用（pin-mux）逻辑。本文档描述了现有的 MFP API，
以及板级/平台驱动作者应当如何使用它。

## 基本概念


与 PXA25x 和 PXA27x 上的 GPIO 复用功能设置不同，从 PXA3xx 起引入了一种
全新的 MFP 机制，把引脚复用功能彻底移出了 GPIO 控制器。除了引脚复用配置
之外，MFP 还控制着每个引脚的低功耗状态、驱动强度、上拉/下拉以及事件检测。
下面是各内部模块之间连接关系的示意图：

```
 +--------+
 |        |--(GPIO19)--+
 |  GPIO  |            |
 |        |--(GPIO...) |
 +--------+            |
                       |       +---------+
 +--------+            +------>|         |
 |  PWM2  |--(PWM_OUT)-------->|   MFP   |
 +--------+            +------>|         |-------> to external PAD
                       | +---->|         |
 +--------+            | | +-->|         |
 |  SSP2  |---(TXD)----+ | |   +---------+
 +--------+              | |
                         | |
 +--------+              | |
 | Keypad |--(MKOUT4)----+ |
 +--------+                |
                           |
 +--------+                |
 |  UART2 |---(TXD)--------+
 +--------+
```

注意：外部焊盘（pad）被命名为 MFP_PIN_GPIO19，这并不必然意味着它是专用于
GPIO19 的，而只是提示该引脚在内部可以由 GPIO 控制器的 GPIO19 路由而来。

为了更好地理解从 PXA25x/PXA27x 的 GPIO 复用功能到这种新 MFP 机制的变化，
下面是几个关键点：

  1. PXA3xx 上的 GPIO 控制器现在是一个专用控制器，与其他内部控制器（如
     PWM、SSP 和 UART）一样，拥有 128 个内部信号，这些信号可以通过一个或多个
     MFP 路由到外部（例如 GPIO<0> 既可以通过 MFP_PIN_GPIO0，也可以通过
     MFP_PIN_GPIO0_2 路由，参见 arch/arm/mach-pxa/mfp-pxa300.h）

  2. 复用功能配置已从该 GPIO 控制器中移除，剩下的功能都是纯 GPIO 相关的，即

       - GPIO 信号电平控制
       - GPIO 方向控制
       - GPIO 电平变化检测

  3. 每个引脚的低功耗状态现在由 MFP 控制，这意味着 PXA2xx 上的 PGSRx 寄存器
     在 PXA3xx 上已经没有用处了

  4. 唤醒检测现在由 MFP 控制，PWER 不再控制来自 GPIO 的唤醒；根据睡眠状态的不同，
     由 ADxER（定义于 pxa3xx-regs.h）控制来自 MFP 的唤醒

注意：由于 MFP 与 GPIO 之间有如此清晰的分工，通常我们用 GPIO<xx> 表示一个
GPIO 信号，而用 MFP<xxx> 或引脚 xxx 表示一个物理焊盘（或球栅）。

## MFP API 用法


对于板级代码编写者，下面是一些指导原则：

1. 在你的 <board>.c 中包含下列头文件之一：

   - #include "mfp-pxa25x.h"
   - #include "mfp-pxa27x.h"
   - #include "mfp-pxa300.h"
   - #include "mfp-pxa320.h"
   - #include "mfp-pxa930.h"

   注意：你的 <board>.c 中只包含其中一个文件，具体取决于所使用的处理器，因为
   这些文件中的引脚配置定义可能会冲突（即同名在不同处理器上含义和设置不同）。
   例如对于同时支持 PXA300/PXA310 和 PXA320 的 zylonite 平台，引入了两个独立
   的文件：zylonite_pxa300.c 和 zylonite_pxa320.c（除了处理 MFP 配置的差异，
   它们还处理这两种组合之间的其他差异）。

   注意：PXA300 和 PXA310 在引脚配置上几乎完全相同（PXA310 额外支持其中一些），
   因此这一差异实际上被涵盖在单个 mfp-pxa300.h 中。

```
     static unsigned long mainstone_pin_config[] __initdata = {
	/* Chip Select */
	GPIO15_nCS_1,

	/* LCD - 16bpp Active TFT */
	GPIOxx_TFT_LCD_16BPP,
	GPIO16_PWM0_OUT,	/* Backlight */

	/* MMC */
	GPIO32_MMC_CLK,
	GPIO112_MMC_CMD,
	GPIO92_MMC_DAT_0,
	GPIO109_MMC_DAT_1,
	GPIO110_MMC_DAT_2,
	GPIO111_MMC_DAT_3,

	...

	/* GPIO */
	GPIO1_GPIO | WAKEUP_ON_EDGE_BOTH,
     };

   a) 一旦引脚配置被传递给 pxa{2xx,3xx}_mfp_config() 并写入实际寄存器后，
   它们就没有用了，可能会被丢弃，加上 '__initdata' 可以在这里节省一些额外的字节。

   b) 当一个部件只有一种可行的引脚配置时，可以使用一些简化的定义，例如
   PXA25x 和 PXA27x 处理器上的 GPIOxx_TFT_LCD_16BPP

   c) 如果按板级设计，某个引脚可以被配置为从低功耗状态唤醒系统，它可以用
   下列任意一项做“或”运算：

      WAKEUP_ON_EDGE_BOTH
      WAKEUP_ON_EDGE_RISE
      WAKEUP_ON_EDGE_FALL
      WAKEUP_ON_LEVEL_HIGH - 专门用于启用键盘 GPIO

   以表明该引脚具备唤醒系统的能力，以及在哪些边沿上。然而，这并不必然意味着
   该引脚“会”唤醒系统，只有当使用相应的 GPIO IRQ（GPIO_IRQ(xx) 或
   gpio_to_irq()）调用 set_irq_wake()，并最终为实际的寄存器设置调用
   gpio_set_wake() 时，它才会唤醒系统。

   d) 尽管 PXA3xx 的 MFP 支持每个引脚的边沿检测，内部逻辑只在 ADxER 寄存器中的
   那些特定位被置位时才会唤醒系统，而这些位可以很好地映射到相应的外设，因此
   可以针对外设 IRQ 调用 set_irq_wake() 来启用唤醒。


```
## PXA3xx 上的 MFP


PXA3xx 上的每个外部 I/O 焊盘（特殊用途的除外）都关联着一个 MFP 逻辑，并
由一个 MFP 寄存器（MFPR）控制。

```
 31                        16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  +-------------------------+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
  |         RESERVED        |PS|PU|PD|  DRIVE |SS|SD|SO|EC|EF|ER|--| AF_SEL |
  +-------------------------+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

  Bit 3:   RESERVED
  Bit 4:   EDGE_RISE_EN - enable detection of rising edge on this pin
  Bit 5:   EDGE_FALL_EN - enable detection of falling edge on this pin
  Bit 6:   EDGE_CLEAR   - disable edge detection on this pin
  Bit 7:   SLEEP_OE_N   - enable outputs during low power modes
  Bit 8:   SLEEP_DATA   - output data on the pin during low power modes
  Bit 9:   SLEEP_SEL    - selection control for low power modes signals
  Bit 13:  PULLDOWN_EN  - enable the internal pull-down resistor on this pin
  Bit 14:  PULLUP_EN    - enable the internal pull-up resistor on this pin
  Bit 15:  PULL_SEL     - pull state controlled by selected alternate function
                          (0) or by PULL{UP,DOWN}_EN bits (1)

  Bit 0 - 2: AF_SEL - alternate function selection, 8 possibilities, from 0-7
  Bit 10-12: DRIVE  - drive strength and slew rate
			0b000 - fast 1mA
			0b001 - fast 2mA
			0b002 - fast 3mA
			0b003 - fast 4mA
			0b004 - slow 6mA
			0b005 - fast 6mA
			0b006 - slow 10mA
			0b007 - fast 10mA

```
## PXA2xx/PXA3xx 的 MFP 设计


由于 PXA2xx 与 PXA3xx 在引脚复用处理上的差异，引入了一套统一的 MFP API
来同时涵盖这两个系列的处理器。

该设计的基本思想是引入针对所有可能引脚配置的定义，这些定义与处理器和平台
无关，再调用实际的 API 把这些定义转换为寄存器设置并使之生效。

### 涉及的文件


  - arch/arm/mach-pxa/include/mach/mfp.h

  用于
    1. 统一的引脚定义 —— 所有可配置引脚的枚举常量
    2. 与处理器无关的、针对一种可能的 MFP 配置的位定义

  - arch/arm/mach-pxa/mfp-pxa3xx.h

  用于 PXA3xx 特有的 MFPR 寄存器位定义以及 PXA3xx 通用引脚配置

  - arch/arm/mach-pxa/mfp-pxa2xx.h

  用于 PXA2xx 特有的定义以及 PXA25x/PXA27x 通用引脚配置

  - arch/arm/mach-pxa/mfp-pxa25x.h
    arch/arm/mach-pxa/mfp-pxa27x.h
    arch/arm/mach-pxa/mfp-pxa300.h
    arch/arm/mach-pxa/mfp-pxa320.h
    arch/arm/mach-pxa/mfp-pxa930.h

  用于处理器特有的定义

  - arch/arm/mach-pxa/mfp-pxa3xx.c
  - arch/arm/mach-pxa/mfp-pxa2xx.c

  用于使引脚配置在实际上对具体处理器生效的实现。

### 引脚配置


  以下注释摘自 mfp.h（参见实际源代码
```
    /*
     * a possible MFP configuration is represented by a 32-bit integer
     *
     * bit  0.. 9 - MFP Pin Number (1024 Pins Maximum)
     * bit 10..12 - Alternate Function Selection
     * bit 13..15 - Drive Strength
     * bit 16..18 - Low Power Mode State
     * bit 19..20 - Low Power Mode Edge Detection
     * bit 21..22 - Run Mode Pull State
     *
     * to facilitate the definition, the following macros are provided
     *
     * MFP_CFG_DEFAULT - default MFP configuration value, with
     * 		  alternate function = 0,
     * 		  drive strength = fast 3mA (MFP_DS03X)
     * 		  low power mode = default
     * 		  edge detection = none
     *
     * MFP_CFG	- default MFPR value with alternate function
     * MFP_CFG_DRV	- default MFPR value with alternate function and
     * 		  pin drive strength
     * MFP_CFG_LPM	- default MFPR value with alternate function and
     * 		  low power mode
     * MFP_CFG_X	- default MFPR value with alternate function,
     * 		  pin drive strength and low power mode
     */

   Examples of pin configurations are::

     #define GPIO94_SSP3_RXD		MFP_CFG_X(GPIO94, AF1, DS08X, FLOAT)

   其含义是 GPIO94 可以被配置为 SSP3_RXD，复用功能选择为 1，驱动强度为
   0b101，并且在低功耗模式下处于浮空（float）状态。

   注意：这是将该引脚配置为 SSP3_RXD 的默认设置，在板级代码中可以稍作修改，
   不过并不推荐这样做，原因仅仅是这种默认设置通常经过精心编码，并且在大多数
   情况下都能正常工作。

```
### 寄存器设置


   在 PXA3xx 上，针对某个引脚配置的寄存器设置实际上非常直接，大多数位可以
   以一种更简单的方式直接转换为 MFPR 值。计算出了两组 MFPR 值：运行时的
   那组和低功耗模式的那组，以便允许不同的设置。

   从通用的引脚配置到 PXA2xx 上实际寄存器设置的转换则稍显复杂：涉及许多
   寄存器，包括 GAFRx、GPDRx、PGSRx、PWER、PKWR、PFER 和 PRER。关于这种
   转换是如何进行的，请参阅 mfp-pxa2xx.c。
