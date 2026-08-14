
## 内核驱动 asus_wmi_sensors


支持的板卡：
 - PRIME X399-A,
 - PRIME X470-PRO,
 - ROG CROSSHAIR VI EXTREME,
 - ROG CROSSHAIR VI HERO,
 - ROG CROSSHAIR VI HERO (WI-FI AC),
 - ROG CROSSHAIR VII HERO,
 - ROG CROSSHAIR VII HERO (WI-FI),
 - ROG STRIX B450-E GAMING,
 - ROG STRIX B450-F GAMING,
 - ROG STRIX B450-I GAMING,
 - ROG STRIX X399-E GAMING,
 - ROG STRIX X470-F GAMING,
 - ROG STRIX X470-I GAMING,
 - ROG ZENITH EXTREME,
 - ROG ZENITH EXTREME ALPHA.

Authors:
    - Ed Brindley <kernel@maidavale.org>

### 描述：

华硕（ASUS）主板通过 WMI 接口发布硬件监视信息。

ASUS WMI 接口提供一种获取传感器列表及其值的方法，本驱动利用它将这些传感器读数发布到 HWMON 系统。

该驱动能够识别并读取以下传感器：
 - CPU 核心电压（CPU Core Voltage）,
 - CPU SOC 电压（CPU SOC Voltage）,
 - DRAM 电压（DRAM Voltage）,
 - VDDP 电压（VDDP Voltage）,
 - 1.8V PLL 电压（1.8V PLL Voltage）,
 - +12V 电压（+12V Voltage）,
 - +5V 电压（+5V Voltage）,
 - 3VSB 电压（3VSB Voltage）,
 - VBAT 电压（VBAT Voltage）,
 - AVCC3 电压（AVCC3 Voltage）,
 - SB 1.05V 电压（SB 1.05V Voltage）,
 - CPU 核心电压（CPU Core Voltage）,
 - CPU SOC 电压（CPU SOC Voltage）,
 - DRAM 电压（DRAM Voltage）,
 - CPU 风扇转速（CPU Fan RPM）,
 - 机箱风扇 1 转速（Chassis Fan 1 RPM）,
 - 机箱风扇 2 转速（Chassis Fan 2 RPM）,
 - 机箱风扇 3 转速（Chassis Fan 3 RPM）,
 - HAMP 风扇转速（HAMP Fan RPM）,
 - 水泵转速（Water Pump RPM）,
 - CPU OPT 转速（CPU OPT RPM）,
 - 水流量转速（Water Flow RPM）,
 - AIO 水泵转速（AIO Pump RPM）,
 - CPU 温度（CPU Temperature）,
 - CPU 插槽温度（CPU Socket Temperature）,
 - 主板温度（Motherboard Temperature）,
 - 芯片组温度（Chipset Temperature）,
 - Tsensor 1 温度（Tsensor 1 Temperature）,
 - CPU VRM 温度（CPU VRM Temperature）,
 - 进水温度（Water In）,
 - 出水温度（Water Out）,
 - CPU VRM 输出电流（CPU VRM Output Current）.

已知问题：
 - 华硕部分 BIOS 中的 WMI 实现存在 bug。这可能导致风扇停止、风扇卡在最高转速，或温度读数卡住。这不是驱动的问题，而是 BIOS 的问题。Prime X470 Pro 在这方面似乎尤其糟糕。WMI 接口被轮询得越频繁，发生这种情况的可能性就越大。在你对计算机进行长时间压力测试并频繁轮询传感器之前，不要让你的计算机无人看管。升级到方法版本大于等于 2 的新 BIOS 版本应当能纠正该问题。
 - 少数主板报告的 12v 电压约为 10v。
