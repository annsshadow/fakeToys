
## 内核驱动 sbrmi


Supported hardware:

  - 通过 APML 连接到 BMC 的、兼容 Sideband Remote Management Interface
    （SB-RMI）的 AMD SoC 设备。

    Prefix: 'sbrmi'

    Addresses scanned: 该驱动不支持地址扫描。

    要在支持 SB-RMI 的 AMD CPU 上实例化该驱动，i2c 总线编号应为从板级管理
    控制器（BMC）连接到 CPU 的总线。
    SMBus 地址实际为 7 位。部分厂商及 SMBus 规范将地址表示为 8 位、左对齐，
    并将 R/W 位作为写（0）使 bit 0 为 0。部分厂商仅使用 7 位来描述地址。
    如 AMD 的 APML 规范所述，SB-RMI 地址通常为 socket 0 的 78h(0111 100W) 或
    3Ch(011 1100)，以及 socket 1 的 70h(0111 000W) 或 38h(011 1000)，但会
    因硬件地址选择引脚而有所变化。

    Datasheet: SB-RMI 接口与协议，连同 Advanced Platform Management Link
               （APML）规范，作为开源 SoC 寄存器参考的一部分提供，位于：

               https://www.amd.com/en/support/tech-docs?keyword=55898

Author: Akshay Gupta <akshay.gupta@amd.com>

### 描述


APML 提供了一种从外部 SMBus 主设备与 SB Remote Management interface（SB-RMI）
模块通信的方式，可用于通过邮箱命令报告 AMD 平台上的插槽功耗，并类似于典型的
8 引脚远端电源传感器的 I2C 接口连接到 BMC。

该驱动实现了当前功耗以及功耗上限与最大功耗上限。

### sysfs 接口


电源传感器可通过标准 `hwmon` 接口在 `sysfs` 上查询与设置，位于目录
`/sys/class/hwmon/hwmonX`（X 为某值，查找使 `/sys/class/hwmon/hwmonX/name`
内容为 `sbrmi` 的那个 `X`）。

================ ===== ========================================================
Name             Perm   描述
================ ===== ========================================================
power1_input     RO    当前消耗功率
power1_cap       RW    可在 0 与 power1_cap_max 之间设置功耗限制
power1_cap_max   RO    SMU FW 计算并报告的最大功耗限制
================ ===== ========================================================

以下示例展示了来自 i2c 地址的 'Power' 属性
```

  # sensors
  sbrmi-i2c-1-38
  Adapter: bcm2835 I2C adapter
  power1:       61.00 W (cap = 225.00 W)

  sbrmi-i2c-1-3c
  Adapter: bcm2835 I2C adapter
  power1:       28.39 W (cap = 224.77 W)
  #

```
```
  # cat /sys/class/hwmon/hwmon1/power1_cap_max
  225000000

  # echo 180000000 > /sys/class/hwmon/hwmon1/power1_cap
  # cat /sys/class/hwmon/hwmon1/power1_cap
  180000000

```
