## 串行外设接口（SPI, Serial Peripheral Interface

SPI 是“串行外设接口”，由于它是一个简单而高效的接口——基本上就是一个多路复用的
移位寄存器，因而被广泛用于嵌入式系统。它的三条信号线分别承载一个时钟（SCK，通常
1-20 MHz 范围内）、一条“主出从入”（MOSI）数据线，以及一条“主入从出”（MISO数据线。SPI 是一个全双工协议；每MOSI 线移出一位（每时钟一位），就MISO 线移另一位。这些位在往返系统内存的途中被组装成各种大小的字。通常还有一条额外的片线（nCS，低有效）；每个外设一般使用四条信号，有时外加一个中断
此处列出SPI 总线设施提供了一个通用接口，用于声SPI 总线与设备、按照标Linux
驱动模型管理它们，并执行输入/输出操作。目前仅支持“主（master）”端接口，即 Linux
SPI 外设通信，而自身不实现这样的外设。（支持实现 SPI 从设备的接口必然看起不同。）

编程接口围绕两类驱动与两类设备构建。一个“Controller Driver（控制器驱动）”抽了控制器硬件，它可能简单到一GPIO 引脚，也可能复杂到位SPI 移位寄存器另一侧的连接DMA 引擎的一FIFO（以最大化吞吐量）。这类驱动在它们所位于的任意总线
（通常是平台总线）与 SPI 之间架桥，并将它们设备的 SPI 侧作为一:c:type:`struct
spi_controller <spi_controller>` 暴露出来。SPI 设备是该 master 的子设备，表示为
`struct spi_device <spi_device>`，并由通常是板级初始化代码提供:c:type:`struct
spi_board_info <spi_board_info>` 描述符制造而来。一:c:type:`struct spi_driver
<spi_driver>` 被称为“Protocol Driver（协议驱动）”，并使用普通的驱动模型调用绑定一spi_device
I/O 模型是一组排队的消息。协议驱动提交一个或多个 `struct spi_message
<spi_message>` 对象，这些对象被异步处理并完成。（不过也有同步封装。）消息由一
个或多个 `struct spi_transfer <spi_transfer>` 对象构建，每个对象封装一次全双工
SPI 传输。需要各种协议微调选项，因为不同的芯片对于如何使用通过 SPI 传输的位采用非常不同的策略
   :internal:

   :functions: spi_register_board_info

   :export:
