## 中断


2.5.2-rmk5:
  这是第一个包含了某些主要架构相关子系统重大调整的内核。

首先，它对 MMU TLB 的处理方式进行了一些相当重大的改动。每种 MMU TLB
变体现在都完全独立地处理——我们有 TLB v3、TLB v4（无写缓冲）、TLB v4
（带写缓冲），以及最后的 TLB v4（带写缓冲，并带 I TLB 失效项）。这些
函数内部包含了更多的汇编代码，主要是为了将来能够更灵活地处理 TLB。

其次，是 IRQ 子系统。

2.5 内核将对 IRQs 的处理方式进行重大改动。不幸的是，这意味着所有会
触及 irq_desc[] 数组（基本涵盖所有机器类型）的机器类型都会出问题，
也就是说我们目前拥有的每一种机器类型都会受影响。

```

                  GPIO25                 IRR:2
        SA1100 ------------> Neponset -----------> SA1111
                                         IIR:1
                                      -----------> USAR
                                         IIR:0
                                      -----------> SMC9196

```
目前的工作方式是，所有 SA1111 中断彼此互斥——如果你正在处理来自
SA1111 的一个中断，此时又来了一个新的中断，你必须等当前这个中断
处理完毕后才能响应新的中断。例如，SA1111 上基于 IDE PIO 的中断会
排除所有其他 SA1111 与 SMC9196 中断，直到它完成多扇区数据传输为止，
而这可能会持续较长时间。另请注意，由于我们在 SA1111 的 IRQ 处理函数中
循环，SA1111 的 IRQs 可能会无限期地阻塞 SMC9196 的 IRQs。


新的方案引入了几项新思路……

我们引入了“父”（parent）和“子”（child）的概念。例如，对 Neponset
处理程序而言，“父”是 GPIO25，而“子”则是 SA1111、SMC9196 和 USAR。

我们还引入了 IRQ “芯片”（chip）的概念（主要是为减小 irqdesc 数组的
大小）。它不必是一个真实的“IC”；实际上 SA11x0 的 IRQs 由两个不同的
“chip” 结构来处理，一个负责 GPIO0-10，另一个负责其余所有。它只是用于
容纳各种操作的一个容器（也许以后会改名）。
```

  struct irqchip {
          /*
           * Acknowledge the IRQ.
           * If this is a level-based IRQ, then it is expected to mask the IRQ
           * as well.
           */
          void (*ack)(unsigned int irq);
          /*
           * Mask the IRQ in hardware.
           */
          void (*mask)(unsigned int irq);
          /*
           * Unmask the IRQ in hardware.
           */
          void (*unmask)(unsigned int irq);
          /*
           * Re-run the IRQ
           */
          void (*rerun)(unsigned int irq);
          /*
           * Set the type of the IRQ.
           */
          int (*type)(unsigned int irq, unsigned int, type);
  };

```
ack
       - 必需。对于由 do_level_IRQ 处理的 IRQs，可以与 mask 是同一个函数。
mask
       - 必需。
unmask
       - 必需。
rerun
       - 可选。如果你对所有使用此 “irqchip” 的 IRQs 都使用 do_level_IRQ，
         则不需要它。通常期望在可能的情况下重新触发硬件 IRQ。如果做不到，
         则可以直接调用处理函数。
type
       - 可选。如果你不支持改变 IRQ 的类型，则应置为 null，以便人们能
         检测出他们是否无法设置 IRQ 类型。

对于每个 IRQ，我们保留以下信息：

        - “disable” 深度（未配对的 enable_irq() 的 disable_irq() 次数）
        - 标志位，指示我们能对此 IRQ 做什么（valid、probe、
          noautounmask），与之前一致
        - IRQ 的状态（probing、enable 等）
        - chip
        - 每-IRQ 的处理函数
        - irqaction 结构链表

处理函数可以是 3 种标准处理函数之一——“level”、“edge” 和
“simple”，或者如果你需要做特殊操作，也可以是自己的专有处理函数。

“level” 处理函数即我们目前所用的——它相当简单。“edge” 了解这类 IRQ
实现的缺陷——你需要让硬件 IRQ 在处理期间保持使能，并在处理过程中
如果 IRQ 再次发生则排队后续的 IRQ 事件。“simple” 处理函数非常基础，
不进行任何硬件操作，也不做状态跟踪。这对上面提到的 SMC9196 和 USAR
很有用。

## 那么，有什么变化？


1. 机器实现不得再写入 irqdesc 数组。

2. 新增了用于操作 irqdesc 数组的函数。前 4 个预期仅对机器相关代码有用。
   最后一个建议仅由机器相关代码使用，但在绝对必要的情况下也可在驱动中使用。

        set_irq_chip(irq,chip)
                设置处理此 IRQ 的 mask/unmask 方法

        set_irq_handler(irq,handler)
                设置此 IRQ 的处理函数（level、edge、simple）

        set_irq_chained_handler(irq,handler)
                为此 IRQ 设置一个“链式”处理函数——会自动使能此 IRQ
                （例如 Neponset 和 SA1111 的处理函数）。

        set_irq_flags(irq,flags)
                设置 valid/probe/noautoenable 标志。

        set_irq_type(irq,type)
                激活 IRQ 的 边沿/电平。这取代了 SA1111 INTPOL 的操作，
                以及 set_GPIO_IRQ_edge() 函数。type 应为 <linux/irq.h>
                中定义的 IRQ_TYPE_xxx 之一。

3. set_GPIO_IRQ_edge() 已废弃，应被 set_irq_type 取代。

4. 直接访问 SA1111 INTPOL 已被弃用。请改用 set_irq_type。

5. 处理函数应通过正确的 chip 专有函数来确认（acknowledge）父 IRQ。
   例如，如果 SA1111 直接连接到 SA1110 的 GPIO，那么每次重新读取
   SA1111 的 IRQ 状态时，都应确认 SA1110 的 IRQ。

6. 对于任何没有自身 IRQ 使能/禁用控制（例如 SMC9196）的子设备，处理
   函数在调用子处理函数期间必须屏蔽或确认父 IRQ，且子处理函数应使用
   “simple” 处理函数（而非 “edge” 或 “level”）。处理函数完成后，应
   解除父 IRQ 的屏蔽，并重新检查所有子设备的状态以发现待处理事件。
   （详见 Neponset 的 IRQ 处理函数）。

7. fixup_irq() 已移除，同样移除的还有 `arch/arm/mach-*/include/mach/irq.h`

请注意，这并不能解决所有问题——其中一些是硬件层面的。在同一父信号
上混合电平触发和边沿触发的 IRQs（例如 neponset）就属于此类，软件方案
无法提供低 IRQ 延迟的完整答案。
