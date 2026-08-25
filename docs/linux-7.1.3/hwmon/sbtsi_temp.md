
## 内核驱动 sbtsi_temp


支持的硬件：

  - 符合 Sideband interface（SBI）Temperature Sensor Interface（SB-TSI）的 AMD SoC 温度传感器设备

    Prefix: 'sbtsi_temp'

    Addresses scanned: 该驱动不支持地址扫描

    要在支持 SB-TSI AMD CPU 上实例化该驱动，i2c 总线号应为从板级管理控制器（BMC）连接到 CPU 的总线。i2c 地址SoC 寄存器参考的6.3.1 节中指定：SB-TSI 地址通常对于 socket 0 98h，对socket 1 90h，但可能根据硬件地址选择引脚而不同

    Datasheet: SB-TSI 接口和协议作为开SoC 寄存器参考的一部分提供，位于：

	       https://www.amd.com/system/files/TechDocs/56255_OSRR.pdf

               Advanced Platform Management Link（APML）规范位于：

	       http://developer.amd.com/wordpress/media/2012/10/41918.pdf

Author: Kun Yi <kunyi@google.com>

### 描述


SBI 温度传感器接口（SB-TSI）是AMD SoC 上典8 引脚远程温度传感器（RTS）的软硬件接口的模拟。它实现一个温度传感器，其读数和限制寄存器0.125 为增量对 0 255.875 的温度进行编码。可通过可写阈值设置限制，一旦达到将触发相应的报警信号
