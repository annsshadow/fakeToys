## I\ :sup:`2`\ C SMBus 子系

I\ `2`\ C（或者不用花哨排版的话，"I2C"）是 "Inter-IC" 总线的缩写，是一种简单的协议总线，在只需低速通信的场合被广泛使用。由于它也是一项注册商标，一些厂商为同一总线使用了其他名称（"Two-Wire Interface"，TWI）。I2C 只需要两个信号（SCL 用于时钟，SDA 用于数据），从而节省板级空间并最小化信号质量问题。大多数 I2C 设备使用七位地址，总线速度最高可400 kHz；还有一个高速扩展（3.4 MHz）尚未得到广泛使用。I2C 是一种多主（multi-master）总线；使用开漏（open drain）信号在主机之间仲裁，并与较慢的客户端握手及同步时钟
Linux I2C 编程接口支持总线交互的主机侧与从机侧。该编程接口围绕两类驱动与两类设备构建。I2C “适配器驱动（Adapter Driver）”抽象了控制器硬件；它绑定到一个物理设备（可能是一PCI 设备platform_device），并暴露一个代表其所管理的每I2C 总线段的 `struct i2c_adapter <i2c_adapter>`。在每个 I2C 总线段上都会有由 `struct i2c_client <i2c_client>` 表示I2C 设备。这些设备会绑定到一:c:type:`struct i2c_driver <i2c_driver>`，它应当遵循标准Linux 驱动模型。有一些函数用于执行各I2C 协议操作；在撰写本文时，所有这些函数都只能在任务上下文（task context）中使用
系统管理总线（SMBus）是一个兄弟协议。大多数 SMBus 系统也兼I2C。SMBus 的电气约束更严格，并且它标准化了特定的协议报文与惯用法。支I2C 的控制器也能支持大多SMBus 操作，但 SMBus 控制器并不支I2C 控制器所支持的所有协议选项。有一些函数用于执行各SMBus 协议操作，既可以使用 I2C 原语，也可以向不支持这些 I2C 操作i2c_adapter 设备发出 SMBus 命令
   :internal:

   :functions: i2c_register_board_info

   :export:

   :export:
