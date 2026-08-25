## GPIO 驱动接口


本文档作为编GPIO 芯片驱动的开发者的指南
每个 GPIO 控制器驱动都需要包含以下头文件，它定义```

  #include <linux/gpio/driver.h>


```
## GPIO 的内部表

一GPIO 芯片处理一条或多条 GPIO 线。要被视GPIO 芯片，这些线必须符合定义：通用输入/输出（General Purpose Input/Output）。如果该线并非通用用途，那么它就不是 GPIO，也不应GPIO 芯片来处理。用例具有指示性：系统中某些线可能被称GPIO，却服务于非常特定的用途，因此不满足通用 I/O 的判据。另一方面，LED 驱动的一条线可能被当GPIO 使用，因此仍应由 GPIO 芯片驱动来处理
GPIO 驱动内部，每GPIO 线由其硬件编号标识，有时也称`offset`，这是一个介0 n-1 之间的唯一编号，其n 是芯片管理的 GPIO 数量
硬件 GPIO 编号应当对于硬件而言是直观的，例如，如果某个系统使用一组内存映射的 I/O 寄存器，其中 32 GPIO 线在 32 位寄存器中由每位对应一条线来处理，那么对这些线使用硬件偏移 0..31 是合理的，它们分别对应寄存器中的0..31
这个编号纯粹是内部的：特GPIO 线的硬件编号永远不会在驱动之外可见
在这个内部编号之上，每条 GPIO 线还需要在整数 GPIO 命名空间中拥有一个全局编号，以便能够与传统GPIO 接口一起使用。因此每个芯片必须有一个“base”编号（可以自动分配），而对于每GPIO 线，其全局编号将是（base + 硬件编号）。尽管整数表示法被视为已废弃，但它仍有许多使用者，因此需要继续维护
例如，某个平台可以对 GPIO 使用全局编号 32-159，其中一个控制器在“base”为 32 的位置定义了 128 GPIO；而另一个平台使用全局编号 0..63 配合一GPIO 控制器4-79 配合另一种类型的 GPIO 控制器，而在某块特定板卡80-95 配合一FPGA。传统编号无需连续；这两个平台中的任何一个也都可以使用编2000-2063 来标I2C GPIO 扩展器组中的 GPIO 线

## 控制器驱动：gpio_chip


gpiolib 框架中，每个 GPIO 控制器被封装为一个“struct gpio_chip”（完整定义<linux/gpio/driver.h>），其中包含该类型每个控制器共有的成员，这些应由驱动代码赋值：

 - 用于确定 GPIO 线方向的方法
 - 用于访问 GPIO 线值的方法
 - 用于设置给定 GPIO 线电气配置的方法
 - 用于返回与给GPIO 线关联的 IRQ 编号的方 - 指示对其方法的调用是否可能休眠的标志
 - 可选的、用于标识各线的线名称数 - 可选的 debugfs dump 方法（显示额外的状态信息）
 - 可选的 base 编号（若省略则自动分配）
 - 可选的、用于诊断和借助平台数据进行 GPIO 芯片映射label

实现 gpio_chip 的代码应当支持控制器的多个实例，最好使用驱动模型。该代码会配置每gpio_chip 并发gpiochip_add_data() devm_gpiochip_add_data()。移GPIO 控制器应当很少见；只有在不可避免时才使用 gpiochip_remove()
gpio_chip 常常是属于某个实例特定结构的一部分，该结构含有 GPIO 接口未暴露的状态，例如寻址、电源管理等。像音频编解码器这样的芯片就会拥有复杂的GPIO 状态
任何 debugfs dump 方法通常应当忽略尚未被请求的线。它们可以使gpiochip_is_requested()，该函数要么返回 NULL，要么返回请求该 GPIO 线时关联label
实时（Realtime）考量：如果预期要在实时（realtime）内核上从原子上下文（硬 IRQ 处理函数及类似上下文中）调用 GPIO API，那GPIO 驱动不应在其 gpio_chip 实现get/.set 以及方向控制回调）中使用 spinlock_t 或任何可休眠API（如 PM runtime）。通常这并非必需

### GPIO 电平语义


gpiolib .get/set[_multiple]() 线值被限制在布尔空[0, 1]，即低电平或高电平
低电平与高电平定义为连接到连接器（如物理焊盘、引脚或电源轨）的线上的物理低电高电平
GPIO 库具有内部逻辑来处理低电平有效（active low）的线，例如原理图中以删除线#name 标注的线，驱动不应试图去猜测一条线的逻辑值
消费者处GPIO 值的方式是，库向消费者呈*逻辑（logical*值。一条线在其**逻辑**值为 1 时被视为**有效（asserted*，在其逻辑值为 0 时被视为**无效（de-asserted*。如果需要反转，这由 gpiolib 处理，并借助硬件描述（如设备树或 ACPI）进行配置，这些描述能够明确指出一条线是高电平有效还是低电平有效
由于电子设备通常会在 GPIO 线前面插入反相器作为驱动级或保护缓冲器，因此这种语义必须是硬件描述的一部分，这样一来消费者（如内核驱动）就无需为此担忧，例如即使某RESET 线在物理上是低电平有效，也可以将其设为逻辑 1 来使其有效

### GPIO 电气配置


GPIO 线可以通过 .set_config() 回调配置为多种电气工作模式。目前该 API 支持设置
- 去抖（Debouncing- 单端模式（open drain/open source，开开源）
- 上拉与下拉电阻使
以下对这些设置进行说明
.set_config() 回调使用与通用引脚控制（pin control）驱动相同的枚举量与配置语义。这并非巧合：可以将 .set_config() 指定为函gpiochip_generic_config()，这会导致调pinctrl_gpio_set_config()，并最终落GPIO 控制器“背后”的引脚控制后端，通常更靠近实际引脚。这样，引脚控制器就可以管理下面列出GPIO 配置
如果使用了引脚控制器后端，GPIO 控制器或硬件描述需要提供“GPIO ranges”，GPIO 线偏移映射到引脚控制器上的引脚编号，以便它们能够正确地相互交叉引用

### 支持去抖GPIO 

去抖（Debouncing）是一种为引脚设置的配置，表明它连接到了可能会产生抖动的机械开关或按钮等。抖动是指由于机械原因，线在极短间隔内被快速拉拉低。这会导致数值不稳定IRQ 反复触发，除非该线被去抖
实践中，去抖的做法是：当线上发生某事件时设置一个定时器，稍等片刻后再次采样该线，看它是否仍具有相同的值（低或高）。这也可以由一个巧妙的状态机重复进行，等待该线变得稳定。无论哪种情况，它都会为去抖设置一个确定的毫秒数，或者如果该时间不可配置，则简单地设为“开/关”

### 支持开开源的 GPIO 

开漏（open drain，CMOS）或开集（open collector，TTL）意味着该线不会被主动驱动为高电平：相反，你把漏集电极作为输出，因此当晶体管
```



   CMOS CONFIGURATION      TTL CONFIGURATION

            ||--- out              +--- out
     in ----||                   |/
            ||--+         in ----|
                |                |\
               GND                 GND

```
这种配置通常用来实现以下两件事之一
- 电平转换（Level-shifting）：达到高于输出所在硅片的逻辑电平- I/O 线（例如 GPIO 线）上的反向线与（wire-OR），使得线上任何驱动级都可以把它拉低，即使同一根线的任何其他输出同时把它驱动为高。一个特例是驱动 I2C 总线SCL SDA 线，按其定义，它就是一个线与（wire-OR）总线
这两种用例都要求该线配备上拉电阻。该电阻会使线倾向于高电平，除非轨上的某个晶体管主动将其拉低。线上的电平会升到上拉电阻的 VDD 那么高，而该 VDD 可能高于晶体管所支持的电平，从而实现向更高 VDD 的电平转换
集成电子器件通常具有 CMOS“图腾柱（totem-pole）”形式的输出驱动级，包含一N-MOS 和一P-MOS 晶体管，其中一个将线驱动为高，另一个将线驱动为低。这被称为推挽（push-pull```

                     VDD
                      |
            OD    ||--+
         +--/ ---o||     P-MOS-FET
         |        ||--+
    IN --+            +----- out
         |        ||--+
         +--/ ----||     N-MOS-FET
            OS    ||--+
                      |
                     GND

```
所需的输出信号（例如直接来自某个 GPIO 输出寄存器）到达 IN。名为“OD”和“OS”的开关通常是闭合的，从而构成推挽电路
考虑名为“OD”和“OS”的小“开关”，它们在输入分叉之后启禁用 P-MOS N-MOS 晶体管。如你所见，如果此开关断开，任一个晶体管都会完全失效。图腾柱于是被减半，并给出高阻态，而非分别主动将线驱动为高或低。这通常是软件控制的开开源的工作方式
一GPIO 硬件以开开源配置出现。有些是硬连线（hard-wired）的线，无论如何都只支持开漏或开源：那里只有一个晶体管。有些是可软件配置的：通过翻转寄存器中的某一位，输出可以被配置为开漏或开源，实际上就是通过拨开上图中标注为“OD”和“OS”的开关来实现。通过禁用 P-MOS 晶体管，输出可以GND 与高阻态之间被驱动（开漏）；通过禁用 N-MOS 晶体管，输出可以VDD 与高阻态之间被驱动（开源）。第一种情况需要在输出轨上配备上拉电阻以完成电路，第二种情况则需要在轨上配备下拉电阻。支持开漏、开源或两者皆支持的硬件，可以gpio_chip 中实现一个特殊的回调set_config()，它接受一个通用pinconf 打包值，指明是将线配置为开漏、开源还是推挽。这会在响应 machine 文件中设置的 GPIO_OPEN_DRAIN GPIO_OPEN_SOURCE 标志时发生，也可来自其他硬件描述
如果这种状态无法在硬件中配置，即如GPIO 硬件不支持硬件层面的开开源，GPIO 库会改用一种技巧：当一条线被设为输出时，如果该线被标记为开漏，IN 输出值为低，它会像往常一样被驱动为低。但如果 IN 输出值被设为高，*不会**被驱动为高，而是会被切换到输入模式，因为输入模式等价于高阻态，从而实现某种“开漏仿真（open drain emulation）”：在电气行为上二者相同，唯一的例外是在切换线的模式时可能出现硬件毛刺
对于开源配置，使用相同的原理，只是它并非主动将线驱动为低，而是将其设为输入

### 支持上拉/下拉电阻GPIO 

GPIO 线可以通过 .set_config() 回调支持上拉/下拉。这意味着 GPIO 线输出端配备有上拉或下拉电阻，且该电阻由软件控制
在分立（discrete）设计中，上拉或下拉电阻直接焊接在电路板上。这不是我们在软件中处理或建模的东西。你对这些线最多只会想到它们很可能被配置为开漏或开源（见上一节）
.set_config() 回调只能开启或关闭上拉/下拉，而不会对所使用电阻的阻值有任何语义层面的了解。它只会切换寄存器中的某一位，以启用或禁用上拉/下拉
如果 GPIO 线支持以不同的阻值对上拉或下拉电阻进行分流（shunting），那么 GPIO 芯片回调 .set_config() 就不够用了。对于这些复杂用例，需要实GPIO 芯片与引脚控制器的组合，因为引脚控制器的引脚配置接口支持对电气属性进行更灵活的控制，并能处理不同的上拉或下拉阻值

## 提供 IRQ GPIO 驱动


GPIO 驱动（GPIO 芯片）同时提供中断是一种惯例，最常见的是级联（cascaded）自一个父中断控制器，而在某些特殊情况下，GPIO 逻辑会与 SoC 的主中断控制器融合在一起。GPIO 块的 IRQ 部分使用 irq_chip 实现，用到头文件 <linux/irq.h>。因此这种组合驱动同时利用了两个子系统：gpio irq
任何 IRQ 消费者都合法地从任何 irqchip 请求 IRQ，即使它是一个组合的 GPIO+IRQ 驱动。基本前提是 gpio_chip irq_chip 是正交的，彼此独立地提供服务。gpiod_to_irq() 只是一个为了方便而找出某GPIO 线对应的 IRQ 的函数，不应依赖它在 IRQ 被使用之前已被调用。始终在来自 GPIO irq_chip API 的各自回调中准备好硬件并使其就绪。不要依gpiod_to_irq() 已被首先调用
我们可以GPIO irqchip 大致分为两类
- 级联中断芯片（CASCADED INTERRUPT CHIPS）：这意味着 GPIO 芯片有一条公共的中断输出线，它由该芯片上任何已使能的 GPIO 线触发。这条中断输出线随后会被路由到上一级的父中断控制器，在最简单的情况下就是系统的主中断控制器。这由一irqchip 建模，它会检GPIO 控制器内部的位，以判断是哪条线触发了它。驱动中irqchip 部分需要检查寄存器来判断这一点，并且很可能还需要通过清除某个位（有时是隐式地，仅通过读取状态寄存器）来确认它正在处理该中断，并且通常还需要设置诸如边沿敏感度（例如上升沿或下降沿，或低电平中断）之类的配置
- 层级中断芯片（HIERARCHICAL INTERRUPT CHIPS）：这意味着每条 GPIO 线都有一条专用于上一级父中断控制器的 irq 线。无需查询 GPIO 硬件来判断是哪条线触发了中断，但仍可能需要确认中断并设置诸如边沿敏感度之类的配置
实时（Realtime）考量：一个实时兼容的 GPIO 驱动不应在其 irqchip 实现中使spinlock_t 或任何可休眠API（如 PM runtime）- spinlock_t 应当替换raw_spinlock_t。[^1^]
- 如果必须使用可休眠的 API，可以从 .irq_bus_lock() .irq_bus_unlock() 回调中完成，因为这是 irqchip 上唯一的慢路径回调。必要时创建这些回调。[^2^]


### 级联 GPIO irqchip


级联 GPIO irqchip 通常属于以下三类之一
- 链式级联 GPIO IRQCHIP（CHAINED CASCADED GPIO IRQCHIPS）：这类通常是内嵌于 SoC 上的类型。这意味着 GPIO 有一个快速的 IRQ 流处理函数，它从IRQ 处理函数以链的方式被调用，最常见的就是系统中断控制器。这意味着 GPIO irqchip 处理函数会在保持 IRQ 禁用的情况下立即从父 irqchip 被调用。GPIO irqchip 随后最终会调用类似这样的代```

    static irqreturn_t foo_gpio_irq(int irq, void *data)
        chained_irq_enter(...);
        generic_handle_irq(...);
        chained_irq_exit(...);

```
  链式 GPIO irqchip 通常不能设置 struct gpio_chip 上的 .can_sleep 标志，因为一切都直接发生在回调中：不能使用像 I2C 这样的慢速总线通信
  实时（Realtime）考量：注意链IRQ 处理函数不会被强制线程化-RT 上。因此，spinlock_t 或任何可休眠API（如 PM runtime）都不能在链IRQ 处理函数中使用
  如果需要（并且如果它无法转换为嵌套线程GPIO irqchip，见下文），可以将链IRQ 处理函数转换为通用 IRQ 处理函数，这样在 -RT 上它将成为线程化 IRQ 处理函数，在RT 上成为硬 IRQ 处理函数（例如，[3]）
  generic_handle_irq() 预期IRQ 禁用的情况下被调用，因此如果它从一个被强制线程化的 IRQ 处理函数中调用，IRQ 核心会报错。那个“fake原始锁可用于绕过此问题：

```
    raw_spinlock_t wa_lock;
    static irqreturn_t omap_gpio_irq_handler(int irq, void *gpiobank)
        unsigned long wa_lock_flags;
        raw_spin_lock_irqsave(&bank->wa_lock, wa_lock_flags);
        generic_handle_irq(irq_find_mapping(bank->chip.irq.domain, bit));
        raw_spin_unlock_irqrestore(&bank->wa_lock, wa_lock_flags);

```
- 通用链式 GPIO IRQCHIP（GENERIC CHAINED GPIO IRQCHIPS）：这类与“CHAINED GPIO irqchip”相同，但不使用链式 IRQ 处理函数。取而代之，GPIO IRQ 的分派由通过 request_irq() 配置的通用 IRQ 处理函数执行。GPIO irqchip 随后最终会```

    static irqreturn_t gpio_rcar_irq_handler(int irq, void *dev_id)
        for each detected GPIO IRQ
            generic_handle_irq(...);

```
  的序列中调用类似这样的代码。实时（Realtime）考量：这类处理函数会被强制线程化-RT 上，因此 IRQ 核心会报错说 generic_handle_irq() 是在 IRQ 启用的情况下被调用的，可以应用与“CHAINED GPIO irqchips”相同的变通办法
```
- 嵌套线程GPIO IRQCHIP（NESTED THREADED GPIO IRQCHIPS）：这类是片外（off-chip）GPIO 扩展器，以及驻留I2C SPI 等睡眠总线另一端的任何其他 GPIO irqchip
  当然，这类需要慢速总线通信来读IRQ 状态、且此类通信又可能引发其IRQ 的驱动，无法IRQ 禁用的情况下于快IRQ 处理函数中处理。取而代之，它们需要生成一个线程，然后屏蔽IRQ 线，直到该中断被驱动处理完。这类驱动的标志是调用类似这样的代码
```

    static irqreturn_t foo_gpio_irq(int irq, void *data)
        ...
        handle_nested_irq(irq);

```
  线程GPIO irqchip 的标志是它们struct gpio_chip 上的 .can_sleep 标志设为 true，表明该芯片在访GPIO 时可能会休眠
  这类 irqchip 天生对实时（realtime）具有容忍度，因为它们已经被设置为处理睡眠上下文

```
### 面向 GPIO irqchip 的基础设施工具


为了帮助处理 GPIO irqchip 及其关联irqdomain 和资源分配回调的设置与管理。这些通过选择 Kconfig 符号 GPIOLIB_IRQCHIP 来激活。如果同时还选择IRQ_DOMAIN_HIERARCHY 符号，则也会提供层级（hierarchical）工具。在假设你的中断GPIO 线索引是一一映射的前提下，gpiolib 会管理其中很大一部分开销代码
    :header: GPIO 线偏 硬件 IRQ

    0,0
    1,1
    2,2
    ...,...
    ngpio-1, ngpio-1


如果某些 GPIO 线没有对应的 IRQ，可以使gpio_irq_chip 中的位掩valid_mask 与标need_valid_mask，将一些线屏蔽为不可用于关IRQ
设置这些工具的首选方式是，在添加 gpio_chip 之前，先struct gpio_chip 内部填充 struct gpio_irq_chip。如果这样做，额外的 irq_chip 会由 gpiolib 在与设置其余 GPIO 功能的同时被建立起来。以下是一个使gpio_irq_chip 的链式级联中断处理函数的典型示例。注mask/unmask（或 disable/enable）函数是如何调用核心 gpiolib 代码的：


  /** Typical state container **/
  struct my_gpio {
      struct gpio_chip gc;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 执行任何必要的操作以屏蔽中断       - 然后调用核心代码以同步状态       */
      gpiochip_disable_irq(gc, hwirq);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 执行任何必要的操作以解除屏蔽中断       - 在调用核心代码以同步状态之后       */
  }

  /*
   - 静态填irqchip。注意它被设const
     （并IRQCHIP_IMMUTABLE 标志进一步标明），并     GPIOCHIP_IRQ_RESOURCE_HELPER 宏向该结构添加了
     一些额外的回调   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  int irq; /** from platform etc **/
  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  /** 获取指向 gpio_irq_chip 的指**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  girq->parent_handler = ftgpio_gpio_irq_handler;
  girq->num_parents = 1;
  girq->parents = devm_kcalloc(dev, 1, sizeof(*girq->parents),
                               GFP_KERNEL);
  if (!girq->parents)
      return -ENOMEM;
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;
  girq->parents[^0^] = irq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

这些工具也支持使用线程化中断。这时你只需单独请求该中断并照此处理

  /** Typical state container **/
  struct my_gpio {
      struct gpio_chip gc;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 执行任何必要的操作以屏蔽中断       - 然后调用核心代码以同步状态       */
      gpiochip_disable_irq(gc, hwirq);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 执行任何必要的操作以解除屏蔽中断       - 在调用核心代码以同步状态之后       */
  }

  /*
   - 静态填irqchip。注意它被设const
     （并IRQCHIP_IMMUTABLE 标志进一步标明），并     GPIOCHIP_IRQ_RESOURCE_HELPER 宏向该结构添加了
     一些额外的回调   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  int irq; /** from platform etc **/
  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  ret = devm_request_threaded_irq(dev, irq, NULL, irq_thread_fn,
                                  IRQF_ONESHOT, "my-chip", g);
  if (ret < 0)
      return ret;

  /** 获取指向 gpio_irq_chip 的指**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  /** 这将让我们在驱动中处理父 IRQ **/
  girq->parent_handler = NULL;
  girq->num_parents = 0;
  girq->parents = NULL;
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

这些工具也支持使用层级中断控制器。在这种情况下，典型的设置如下所示：


  /** 带有动irqchip 的典型状态容**/
  struct my_gpio {
      struct gpio_chip gc;
      struct fwnode_handle *fwnode;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 执行任何必要的操作以屏蔽中断       - 然后调用核心代码以同步状态       */
      gpiochip_disable_irq(gc, hwirq);
      irq_mask_mask_parent(d);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 执行任何必要的操作以解除屏蔽中断       - 在调用核心代码以同步状态之后       */
      irq_mask_unmask_parent(d);
  }

  /*
   - 静态填irqchip。注意它被设const
     （并IRQCHIP_IMMUTABLE 标志进一步标明），并     GPIOCHIP_IRQ_RESOURCE_HELPER 宏向该结构添加了
     一些额外的回调   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  /** 获取指向 gpio_irq_chip 的指**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;
  girq->fwnode = g->fwnode;
  girq->parent_domain = parent;
  girq->child_to_parent_hwirq = my_gpio_child_to_parent_hwirq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

如你所见，非常相似，但你不再为 IRQ 提供父处理函数，而是提供一个父 irqdomain、一个用于硬件的 fwnode，以及一.child_to_parent_hwirq() 函数，其用途是从子（即gpio 芯片）硬irq 查找父硬irq。一如既往，查看内核树中的示例以获取关于如何找到所需部件的参考是很好的做法
如果需要将这些工具所处理IRQ 域中的某GPIO 线排除在外，我们可以在调devm_gpiochip_add_data() gpiochip_add_data() 之前设置 gpiochip .irq.need_valid_mask。这会分配一.irq.valid_mask，其中置位的位数与芯片中GPIO 线数量相同，每一位代表线 0..n-1。驱动可以通过清除此掩码中的位来排GPIO 线。该掩码可以在属struct gpio_irq_chip 一部分init_valid_mask() 回调中填充
使用这些工具时，请记住以下几点：

- 确保赋struct gpio_chip 的所有相关成员，以便 irqchip 能够初始化。例如，.dev .can_sleep 应当被正确设置
- 名义上将 gpio_irq_chip.handler 设为 handle_bad_irq。然后，如果你的 irqchip 是级联的，则根据控制器所支持的以及消费者所请求的，irqchip .set_type() 回调中将处理函数设为 handle_level_irq() handle_edge_irq()

### 锁定 IRQ 的使

由于 GPIO irq_chip 是正交的，我们可能会在不同的用例之间产生冲突。例如，用于 IRQ GPIO 线应当是一条输入线，在输出GPIO 上触发中断是没有意义的。如果子系统内部存在关于哪一侧正在使用资源（例如某条特定GPIO 线和寄存器）的竞争，它就需要拒绝某些操作，并在 gpiolib 子系统内部跟踪使用情况
输入GPIO 可以用作 IRQ 信号。当发生这种情况时，会请求一个驱动，调用
```

  int gpiochip_lock_as_irq(struct gpio_chip *chip, unsigned int offset)

```
这将阻止使用irq 无关GPIO API，直GPIO IRQ 锁被
```

  void gpiochip_unlock_as_irq(struct gpio_chip *chip, unsigned int offset)

```
解除。当GPIO 驱动内部实现 irqchip 时，这两个函数通常应当irqchip .startup() .shutdown() 回调中被调用。当使用 gpiolib irqchip 工具时，这些回调会被自动分配

### 禁用与启IRQ


在一些（边缘）用例中，驱动可能将一GPIO 线用IRQ 的输入，但偶尔会将该线切换为驱动输出，然后再切换回带中断的输入。这发生在诸CEC（消费电子控制，Consumer Electronics Control）这样的器件上
GPIO 被用IRQ 信号时，gpiolib 也需要知道该 IRQ 是启用还是禁用。为了将此告gpiolib```

  void gpiochip_disable_irq(struct gpio_chip *chip, unsigned int offset)

```
这允许驱动在 IRQ 处于
```

  void gpiochip_enable_irq(struct gpio_chip *chip, unsigned int offset)

```
禁用状态时GPIO 驱动为输出。当GPIO 驱动内部实现 irqchip 时，这两个函数通常应当irqchip .irq_disable() .irq_enable() 回调中被调用。当 irqchip 没有声明 IRQCHIP_IMMUTABLE 时，这些回调会被自动分配。这种行为已被废弃，并正在从内核中移除

### GPIO IRQ 芯片的实时（Real-Time）合规

任何 irqchip 的提供者都需要经过精心调整以支持实时（Real-Time）抢占。期GPIO 子系统中的全irqchip 都能牢记这一点，并进行适当的测试，以确保它们启用了实时能力。因此，请注意文档中上述关于实时的考量
以下是在为实时合规准备驱动时要遵循的检查清单：

- 确保 spinlock_t 不被用作 irq_chip 实现的一部分
- 确保可休眠的 API 不被用作 irq_chip 实现的一部分
  如果必须使用可休眠的 API，可以从 .irq_bus_lock() .irq_bus_unlock() 回调中完- 链式 GPIO irqchip：确spinlock_t 或任何可休眠API 不被用于链式 IRQ 处理函数
- 通用链式 GPIO irqchip：注generic_handle_irq() 调用并应用相应的变通办- 链式 GPIO irqchip：尽可能去掉链式 IRQ 处理函数，改用通用 irq 处理函数
- regmap_mmio：可以通过设置 .disable_locking 来禁regmap 内部的锁，并GPIO 驱动中自行处理锁
- 使用内核内适当的实时测试用例，针对电平 IRQ 和边IRQ 分别测试你的驱动

- [^1^] https://lore.kernel.org/r/1437496011-11486-1-git-send-email-bigeasy@linutronix.de/
- [^2^] https://lore.kernel.org/r/1443209283-20781-2-git-send-email-grygorii.strashko@ti.com
- [^3^] https://lore.kernel.org/r/1443209283-20781-3-git-send-email-grygorii.strashko@ti.com


## 请求自身拥有GPIO 引脚


有时允许 GPIO 芯片驱动通过 gpiolib API 请求其自身的 GPIO 描述符是有用的。GPIO 驱动可以使用以下
```

  struct gpio_desc *gpiochip_request_own_desc(struct gpio_desc *desc,
                                              u16 hwnum,
                                              const char *label,
                                              enum gpiod_flags flags)

  void gpiochip_free_own_desc(struct gpio_desc *desc)

```
通过 gpiochip_request_own_desc() 请求的描述符必须gpiochip_free_own_desc() 释放
这些函数必须谨慎使用，因为它们不影响模块使用计数。不要用这些函数来请求不属于调用驱动所拥有gpio 描述符