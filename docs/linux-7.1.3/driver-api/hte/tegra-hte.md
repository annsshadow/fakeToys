
## HTE 内核提供者驱动


### 描述

Nvidia tegra HTE 提供者（也称为 GTE，Generic Timestamping Engine，通用时间戳引擎）驱动实现了两个 GTE 实例：1) GPIO GTE 和 2) LIC（Legacy Interrupt Controller，传统中断控制器）IRQ GTE。两个 GTE 实例都从系统计数器 TSC 获取时间戳，其时钟频率为 31.25MHz，驱动在将其存储为时间戳值之前会将时钟节拍率转换为纳秒。

### GPIO GTE


该 GTE 实例对 GPIO 进行实时时间戳标记。为此，GPIO 需要被配置为输入。只有常开（AON）GPIO 控制器实例支持对 GPIO 进行实时时间戳标记，因为它与 GPIO GTE 紧密耦合。为此，GPIOLIB 新增了两个可选 API，如下所述。GPIO GTE 代码同时支持内核态和用户态消费者。内核态消费者可以直接与 HTE 子系统通信，而用户态消费者的时间戳请求则经由 GPIOLIB CDEV 框架到达 HTE 子系统。位于 `Documentation/devicetree/bindings/timestamp` 的 hte 设备树绑定提供了一个消费者如何请求一条 GPIO 线的示例。

参见 gpiod_enable_hw_timestamp_ns() 和 gpiod_disable_hw_timestamp_ns()。

对于用户态消费者，必须在 IOCTL 调用期间指定 GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE 标志。参考 `tools/gpio/gpio-event-mon.c`，它会以纳秒为单位返回时间戳。

### LIC（Legacy Interrupt Controller，传统中断控制器）IRQ GTE


该 GTE 实例对 LIC IRQ 线进行实时时间戳标记。位于 `Documentation/devicetree/bindings/timestamp` 的 hte 设备树绑定提供了一个消费者如何请求一条 IRQ 线的示例。由于它与 IRQ GTE 提供者是——对应的映射关系，消费者只需直接指定其感兴趣的 IRQ 号即可。HTE 框架目前不支持该 GTE 实例的用户态消费者。

两个 IRQ 和 GPIO GTE 实例的提供者源代码位于 `drivers/hte/hte-tegra194.c`。测试驱动 `drivers/hte/hte-tegra194-test.c` 演示了 IRQ 和 GPIO GTE 的 HTE API 用法。
