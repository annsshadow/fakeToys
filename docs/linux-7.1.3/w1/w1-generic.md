## 1-wire（w1）子系统简介

1-wire 总线是一种简单的主从总线，它通过一根信号线（外加地线，所以是两根线）进行通信。

设备通过在总线上通过开漏输出把信号拉低到地，并对信号线的逻辑电平进行采样来进行通信。

w1 子系统提供了管理 w1 主设备（master）以及与其从设备（slave）通信的框架。

所有 w1 从设备都必须连接到一个 w1 总线主设备。

w1 主设备示例：

    - DS9490 usb device
    - W1-over-GPIO
    - DS2482 (i2c to w1 bridge)
    - Emulated devices, such as a RS232 converter, parallel port adapter, etc

### w1 子系统做了什么？

当一个 w1 主设备驱动向 w1 子系统注册时，会发生以下事情：

 - 为该 w1 主设备创建 sysfs 条目
 - 周期性地在 w1 总线上搜索新的从设备

当在总线上发现一个设备时，w1 核心会尝试为它的族（family）加载驱动并检查它是否已加载。如果已加载，则该族驱动被绑定到该从设备。如果该族没有驱动，则会分配一个默认驱动，它几乎可以执行任何种类的操作。每个逻辑操作本质上都是一个事务，其中可以包含若干个（两个或一个）底层操作。我们来看一下如何读取 EEPROM 内容：
1. 必须写入控制缓冲区，即包含命令字节和两个字节地址的缓冲区。在这一步中，总线被复位并使用 W1_SKIP_ROM 或 W1_MATCH_ROM 命令选中相应的设备。然后提供的控制缓冲区被写入总线。
2. 读取。这将发出读取 eeprom 的响应。

在 1. 和 2. 之间，w1 主设备线程可能会复位总线以进行搜索，从设备甚至会被移除，但在这种情况下会读到 0xff，因为没有选中任何设备。

### w1 设备族

从设备由为某个 w1 设备族编写的驱动来处理。

一个族驱动填充一个 struct w1_family_ops（见 w1_family.h）并向 w1 子系统注册。

当前的族驱动：

w1_therm
  - （ds18?20 温度传感器族驱动）
    提供温度读取函数，它被绑定到上述 w1_family_ops 结构的 ->rbin() 方法上。

w1_smem
  - 用于简单 64 位存储单元的驱动，提供 ID 读取方法。

你可以通过读取相应的 sysfs 文件来调用上述方法。

### w1 主设备驱动需要实现什么？

w1 总线主设备的驱动至少必须提供两个函数。

模拟设备必须提供设置输出信号电平（write_bit）和采样信号电平（read_bit）的能力。

原生支持 1-wire 的设备必须提供写入和采样一个比特（touch_bit）以及复位总线（reset_bus）的能力。

大多数硬件提供更高层的函数，将 w1 处理工作卸载掉。详见 w1.h 中的 struct w1_bus_master 定义。

### w1 主设备 sysfs 接口

========================= =====================================================
<xx-xxxxxxxxxxxx>         A directory for a found device. The format is
                          family-serial
bus                       (standard) symlink to the w1 bus
driver                    (standard) symlink to the w1 driver
w1_master_add             (rw) manually register a slave device
w1_master_attempts        (ro) the number of times a search was attempted
w1_master_max_slave_count (rw) maximum number of slaves to search for at a time
w1_master_name            (ro) the name of the device (w1_bus_masterX)
w1_master_pullup          (rw) 5V strong pullup 0 enabled, 1 disabled
w1_master_remove          (rw) manually remove a slave device
w1_master_search          (rw) the number of searches left to do,
                          -1=continual (default)
w1_master_slave_count     (ro) the number of slaves found
w1_master_slaves          (ro) the names of the slaves, one per line
w1_master_timeout         (ro) the delay in seconds between searches
w1_master_timeout_us      (ro) the delay in microseconds between searches
========================= =====================================================

如果你有一个从不变化的 w1 总线（你不添加或移除设备），可以把模块参数 search_count 设为一个较小的正整数，从而在初始阶段只进行少量的总线搜索。或者也可以把它设为 0，然后通过 w1_master_add 设备文件手动添加从设备序列号。w1_master_add 和 w1_master_remove 文件一般只在搜索被禁用时才有意义，因为一次搜索会重新检测到手动移除但仍在总线上存在的设备，并让超时被加上到手动添加但实际不在总线上的设备。

总线搜索以一定间隔发生，该间隔指定为 timeout 和 timeout_us 模块参数之和（两者任一可以为 0），只要 w1_master_search 仍大于 0 或为 -1。每次搜索尝试会把 w1_master_search 减 1（减到 0），并把 w1_master_attempts 加 1。

### w1 从设备 sysfs 接口

=================== ============================================================
bus                 (standard) symlink to the w1 bus
driver              (standard) symlink to the w1 driver
name                the device name, usually the same as the directory name
w1_slave            (optional) a binary file whose meaning depends on the
                    family driver
rw		    (optional) created for slave devices which do not have
		    appropriate family driver. Allows to read/write binary data.
=================== ============================================================
