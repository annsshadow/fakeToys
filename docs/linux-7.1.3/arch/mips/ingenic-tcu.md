
## Ingenic JZ47xx SoC 的定时器/计数器单元硬件


Ingenic JZ47xx SoC 中的定时器/计数器单元（TCU）是一个多功能硬件模块。它最多
具有八个通道，可用作计数器、定时器或 PWM。

- JZ4725B、JZ4750、JZ4755 仅有六个 TCU 通道，其余 SoC 均有八个通道。

- JZ4725B 引入了一个独立的通道，称为操作系统定时器（OST）。它是一个 32 位
  可编程定时器。在 JZ4760B 及以上版本中，它是 64 位的。

- 每个 TCU 通道都有各自的时钟，可通过其 TCSR 寄存器重新指定父时钟为三种
  不同时钟（pclk、ext、rtc）、进行门控以及重定时。

    - watchdog 与 OST 硬件块在其寄存器空间中也带有格式相同的 TCSR 寄存器。
    - 用于门控/取消门控的 TCU 寄存器也可以门控/取消门控 watchdog 与 OST
      的时钟。

- 每个 TCU 通道工作于两种模式之一：

    - TCU1 模式：通道不能在睡眠模式下工作，但更易于操作。
    - TCU2 模式：通道可以在睡眠模式下工作，但操作比 TCU1 通道稍复杂。

- 每个 TCU 通道的模式取决于所使用的 SoC：

    - 在最老的 SoC（到 JZ4740 为止）上，全部八个通道都以 TCU1 模式运行。
    - 在 JZ4725B 上，通道 5 以 TCU2 运行，其余以 TCU1 运行。
    - 在最新的 SoC（JZ4750 及以上）上，通道 1-2 以 TCU2 运行，其余以 TCU1 运行。

- 每个通道都能产生中断。部分通道共享中断线，部分不共享，且这在不同 SoC 版本
  间有所变化：

    - 在较老的 SoC（JZ4740 及以下）上，通道 0 与通道 1 各有自己的中断线；
      通道 2-7 共享最后一条中断线。
    - 在 JZ4725B 上，通道 0 有自己独立的中断；通道 1-5 共享一条中断线；OST
      使用最后一条中断线。
    - 在较新的 SoC（JZ4750 及以上）上，通道 5 有自己独立的中断；通道 0-4 及
      （若有八个通道）6-7 共享一条中断线；OST 使用最后一条中断线。

## 实现


TCU 硬件的功能分散在多个驱动中：

===========  =====
clocks       drivers/clk/ingenic/tcu.c
interrupts   drivers/irqchip/irq-ingenic-tcu.c
timers       drivers/clocksource/ingenic-timer.c
OST          drivers/clocksource/ingenic-ost.c
PWM          drivers/pwm/pwm-jz4740.c
watchdog     drivers/watchdog/jz4740_wdt.c
===========  =====

由于属于不同驱动与框架的 TCU 各项功能可由相同的寄存器控制，所有这些驱动都
通过同一个 regmap 访问其寄存器。

有关 TCU 驱动 device tree 绑定的更多信息，请参见
Documentation/devicetree/bindings/timer/ingenic,tcu.yaml。
