
## 内核驱动 ChipCap2


支持的芯片：

  - Amphenol CC2D23, CC2D23S, CC2D25, CC2D25S, CC2D33, CC2D33S, CC2D35, CC2D35S

    Prefix: 'chipcap2'

    Addresses scanned: -

    Datasheet: https://www.amphenol-sensors.com/en/telaire/humidity/527-humidity-sensors/3095-chipcap-2

Author:

  - Javier Carrasco <javier.carrasco.cruz@gmail.com>

### 描述


本驱动实现对 Amphenol ChipCap 2 的支持，这是一个湿度与温度芯片系列。温度以毫摄氏度（milli degrees celsius）测量，相对湿度以千分之一百分比（per cent mille）表示。测量范围如下：

  - 相对湿度 100000 pcm4 位分辨率  - 温度40000 +125000 m°C4 位分辨率
该器件使I2C 协议通信，默认使I2C 地址 0x28
根据硬件配置，最多提供两个湿度报警，用于控制最小值和最大值。它们的阈值与迟滞（hysteresis）可通过 sysfs 配置
阈值与迟滞必须以千分之一百分比提供。这些值可能被截断以匹14 位器件分辨率.1 pcm/LSB）
### 已知问题


该驱动不支持修改 I2C 地址与命令窗口长度
### sysfs 接口


以下列表包含驱动始终提供sysfs 属性、其权限以及简短描述：

=============================== ======= ========================================
Name                            Perm    Description
=============================== ======= ========================================
temp1_input:                    RO      温度输入
humidity1_input:                RO      湿度输入
=============================== ======= ========================================

以下列表包含驱动根据硬件配置可能提供sysfs 属性：

=============================== ======= ========================================
Name                            Perm    Description
=============================== ======= ========================================
humidity1_min:                  RW      湿度下限。低于此限的测量值会触发湿度低报humidity1_max:                  RW      湿度上限。高于此限的测量值会触发湿度高报humidity1_min_hyst:             RW      湿度低迟humidity1_max_hyst:             RW      湿度高迟humidity1_min_alarm:            RO      湿度低报警指humidity1_max_alarm:            RO      湿度高报警指=============================== ======= ========================================
