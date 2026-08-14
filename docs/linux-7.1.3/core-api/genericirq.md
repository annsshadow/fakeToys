
## Linux 通用中断处理

:Copyright: |copy| 2005-2010: Thomas Gleixner
:Copyright: |copy| 2005-2006:  Ingo Molnar

## 简介

通用中断处理层旨在为设备驱动提供一套完整的中断处理抽象。它能够处理各种不同类型的中断控制器硬件。设备驱动使用通用 API 函数来请求、使能、禁用和释放中断。驱动无需了解任何中断硬件细节，因此可以不加修改地用于不同平台。

本文档面向希望基于通用中断处理层、为自己的体系架构实现中断子系统的开发者。

## 设计缘由

Linux 最初的中断处理实现使用了 `__do_IRQ()` 超级处理程序（super-handler），它能够处理任意类型的中断逻辑。

最初，Russell King 为了 Linux 2.5/2.6 的 ARM 中断处理实现，归纳出若干种处理程序类型，构成了一套相当通用的集合。他区分了：

- 电平触发类型（Level type）

- 边沿触发类型（Edge type）

- 简单类型（Simple type）

在实现过程中我们又识别出另一种类型：

- 快速 EOI 类型（Fast EOI type）

在 `__do_IRQ()` 超级处理程序的 SMP 世界里，又识别出一种类型：

- 每 CPU 类型（Per CPU type）

这种对高层 IRQ 处理程序进行拆分实现的做法，使我们能够针对每种特定的中断类型优化中断处理的流程。它降低了特定代码路径的复杂度，并允许对给定类型进行优化处理。

最初的通用 IRQ 实现使用 `hw_interrupt_type` 结构及其 `->ack`、`->end` [等] 回调来区分超级处理程序中的流程控制。这导致流程逻辑与底层硬件逻辑混杂在一起，并造成不必要的代码重复：例如在 i386 上，存在 `ioapic_level_irq` 和 `ioapic_edge_irq` 两种 IRQ 类型，它们共享许多底层细节，却拥有不同的流程处理。

一种更为自然的抽象，是将“irq 流程（irq flow）”与“芯片细节（chip details）”彻底分离。

分析若干体系架构的 IRQ 子系统实现后可以发现，其中大多数都能使用一套通用的“irq 流程”方法，而只需添加芯片级别的特定代码。这种分离对于那些需要在 IRQ 流程本身（而非芯片细节）中使用特定变通处理的（子）架构同样有价值——从而提供一个更透明的 IRQ 子系统设计。

每个中断描述符都被分配了自己的高层流程处理程序，通常取自通用实现之一。（这种高层流程处理程序的实现，也使得提供解复用处理程序变得简单，这类处理程序可见于各种架构的嵌入式平台。）

这种分离使通用中断处理层更加灵活且可扩展。例如，某个（子）架构可以对“电平触发类型”中断使用通用的 IRQ 流程实现，同时为“边沿触发类型”添加（子）架构特定的实现。

为了让向新模型的过渡更加容易，并避免破坏已有的实现，`__do_IRQ()` 超级处理程序目前仍然可用。这在一段时间内导致了一种双轨并存的状态。随着时间推移，新模型应当被越来越多的架构采用，因为它能让 IRQ 子系统更小巧、更清晰。它已被弃用三年之久，即将被移除。

## 已知缺陷与假设

无（老天保佑）。

## 抽象层

中断代码中主要有三个抽象层级：

1. 高层驱动 API

2. 高层 IRQ 流程处理程序

3. 芯片级硬件封装

### 中断控制流

每个中断都由一个中断描述符结构 `irq_desc` 来描述。该中断通过一个“unsigned int”数值来引用，该数值用于在描述符结构数组中选中对应的中断描述结构。描述符结构包含状态信息，以及指向分配给该中断的中断流程方法和中断芯片结构的指针。

每当一个中断被触发，底层体系架构代码就会通过调用 `desc->handle_irq()` 进入通用中断代码。这个高层 IRQ 处理函数只使用由所分配的芯片描述符结构引用的 `desc->irq_data.chip` 原语。

### 高层驱动 API

高层驱动 API 由以下函数组成：

- request_irq()

- request_threaded_irq()

- free_irq()

- disable_irq()

- enable_irq()

- disable_irq_nosync()（仅 SMP）

- synchronize_irq()（仅 SMP）

- irq_set_irq_type()

- irq_set_irq_wake()

- irq_set_handler_data()

- irq_set_chip()

- irq_set_chip_data()

详见自动生成的函数文档。

### 高层 IRQ 流程处理程序

通用层提供了一组预定义的 irq 流程方法：

- handle_level_irq()

- handle_edge_irq()

- handle_fasteoi_irq()

- handle_simple_irq()

- handle_percpu_irq()

- handle_edge_eoi_irq()

- handle_bad_irq()

中断流程处理程序（无论是预定义的还是体系架构特定的）由体系架构在启动阶段或设备初始化阶段分配给特定的中断。

#### 默认流程实现

##### 辅助函数

辅助函数调用芯片原语，并被默认流程实现所使用。下列辅助函数为
```

    default_enable(struct irq_data *data)
    {
        desc->irq_data.chip->irq_unmask(data);
    }

    default_disable(struct irq_data *data)
    {
        if (!delay_disable(data))
            desc->irq_data.chip->irq_mask(data);
    }

    default_ack(struct irq_data *data)
    {
        chip->irq_ack(data);
    }

    default_mask_ack(struct irq_data *data)
    {
        if (chip->irq_mask_ack) {
            chip->irq_mask_ack(data);
        } else {
            chip->irq_mask(data);
            chip->irq_ack(data);
        }
    }

    noop(struct irq_data *data)
    {
    }



```
#### 默认流程处理程序实现

##### 默认电平触发 IRQ 流程处理程序

`handle_level_irq` 为电平触发中断提供通用实现。

```

    desc->irq_data.chip->irq_mask_ack();
    handle_irq_event(desc->action);
    desc->irq_data.chip->irq_unmask();


```
##### 默认快速 EOI IRQ 流程处理程序

`handle_fasteoi_irq` 为那些仅需在处理器末尾发出一个 EOI 的中断提供通用实现。

```

    handle_irq_event(desc->action);
    desc->irq_data.chip->irq_eoi();


```
##### 默认边沿触发 IRQ 流程处理程序

`handle_edge_irq` 为边沿触发中断提供通用实现。

```

    if (desc->status & running) {
        desc->irq_data.chip->irq_mask_ack();
        desc->status |= pending | masked;
        return;
    }
    desc->irq_data.chip->irq_ack();
    desc->status |= running;
    do {
        if (desc->status & masked)
            desc->irq_data.chip->irq_unmask();
        desc->status &= ~pending;
        handle_irq_event(desc->action);
    } while (desc->status & pending);
    desc->status &= ~running;


```
##### 默认简单 IRQ 流程处理程序

`handle_simple_irq` 为简单中断提供通用实现。


   简单流程处理程序不会调用任何处理程序/芯片原语。

```

    handle_irq_event(desc->action);


```
##### 默认每 CPU 流程处理程序

`handle_percpu_irq` 为每 CPU 中断提供通用实现。

每 CPU 中断仅在 SMP 上可用，该处理程序提供了一个无需加锁的简化版本。

```

    if (desc->irq_data.chip->irq_ack)
        desc->irq_data.chip->irq_ack();
    handle_irq_event(desc->action);
    if (desc->irq_data.chip->irq_eoi)
        desc->irq_data.chip->irq_eoi();


```
##### EOI 边沿 IRQ 流程处理程序

`handle_edge_eoi_irq` 是边沿处理程序的一个“怪胎”版本，仅用于驯服 powerpc/cell 上一个被严重毁坏的 irq 控制器。

##### 错误 IRQ 流程处理程序

`handle_bad_irq` 用于没有分配真实处理程序的伪中断（spurious interrupts）。

#### 变通处理与优化

通用函数面向那些“干净”的架构和芯片，即不存在特定平台 IRQ 处理变通需求的架构。如果一个架构需要在“流程”层面实现变通处理，那么它可以通过重写高层 irq 流程处理程序来做到。

#### 延迟中断禁用

这一可按中断选择的功能由 Russell King 在 ARM 中断实现中引入。当调用 `disable_irq()` 时，它并不会在硬件层面屏蔽中断。中断保持使能状态，并在中断事件发生时于流程处理程序中被屏蔽。这样可以避免在某些硬件上丢失边沿中断——这类硬件在中断于硬件层面被禁用期间不会保存边沿中断事件。当一个中断在 `IRQ_DISABLED` 标志被置位时到达，则该中断会在硬件层面被屏蔽，并置位 `IRQ_PENDING` 位。当中断被 `enable_irq()` 重新使能时，会检查 pending 位，若其被置位，则该中断会通过硬件或软件重发机制重新发送。（若要使用延迟中断禁用功能而你的硬件又无法重新触发中断，则需要使能 `CONFIG_HARDIRQS_SW_RESEND`。）延迟中断禁用是不可配置的。

### 芯片级硬件封装

芯片级硬件描述符结构 `irq_chip` 包含了所有与芯片直接相关的函数，可供 irq 流程实现使用。

- `irq_ack`

- `irq_mask_ack` - 可选，出于性能考虑推荐实现

- `irq_mask`

- `irq_unmask`

- `irq_eoi` - 可选，EOI 流程处理程序所需

- `irq_retrigger` - 可选

- `irq_set_type` - 可选

- `irq_set_wake` - 可选

这些原语严格如其字面含义：ack 即 ACK，masking 即屏蔽一条 IRQ 线，以此类推。如何使用这些底层功能的基本单元，取决于流程处理程序。

## __do_IRQ 入口点

最初的实现 `__do_IRQ()` 是所有类型中断的一个备用入口点。它已不复存在。

这一处理程序最终被证明并不适用于所有中断硬件，因此被重新实现为针对边沿/电平/简单/每 CPU 中断的拆分功能。这不仅仅是一项功能上的优化，它还缩短了中断的代码路径。

## SMP 上的加锁

芯片寄存器的加锁由定义芯片原语的架构负责。每个 irq 结构由通用层通过 `desc->lock` 进行保护。

## 通用中断芯片

为了避免重复实现完全相同的 IRQ 芯片功能，内核核心提供了一个可配置的通用中断芯片实现。开发者在自行以略微不同的方式实现相同功能之前，应当仔细确认通用芯片是否符合他们的需求。

   :export:

## 结构

本章包含通用 IRQ 层中所用结构的自动生成文档。

   :internal:

   :internal:

## 提供的公共函数

本章包含已导出的内核 API 函数的自动生成文档。


   :export:

## 提供的内部函数

本章包含内部函数的自动生成文档。



   :internal:

## 致谢

以下人员为本文档做出了贡献：

1. Thomas Gleixner tglx@kernel.org

2. Ingo Molnar mingo@elte.hu
