
## 内核驱动 aquacomputer-d5next


支持的设备：

- Aquacomputer Aquaero 5/6 风扇控制器
- Aquacomputer D5 Next 水冷泵
- Aquacomputer Farbwerk RGB 控制器
- Aquacomputer Farbwerk 360 RGB 控制器
- Aquacomputer Octo 风扇控制器
- Aquacomputer Quadro 风扇控制器
- Aquacomputer High Flow Next 传感器
- Aquacomputer Leakshield 防漏系统
- Aquacomputer Aquastream XT 水冷泵
- Aquacomputer Aquastream Ultimate 水冷泵
- Aquacomputer Poweradjust 3 风扇控制器
- Aquacomputer High Flow USB 流量计
- Aquacomputer MPS Flow 设备

Author: Aleksa Savic

### 描述


该驱动暴露所列 Aquacomputer 设备的硬件传感器，这些设备通过专有的 USB HID 协议通信。

Aquaero 设备暴露 8 个物理、8 个虚拟和 4 个计算得到的虚拟温度传感器，以及 2 个流量传感器。
风扇暴露其转速（RPM）、功率、电压和电流。温度偏移和风扇转速可以被控制。

对于 D5 Next 泵，可用的传感器有泵和风扇转速、功率、电压和电流，以及冷却液温度和 8 个虚拟
温度传感器的读数。此外，通过 debugfs 还可获得序列号、固件版本和上电次数。把风扇连接到它
上是可选的，并允许通过泵直接使用温度曲线控制它。如果未连接，与风扇相关的传感器将报告零。

该泵可以通过软件或通过其物理接口进行配置。通过本驱动配置泵尚未实现，因为这似乎需要向它
发送一份完整的配置。其中包括可寻址的 RGB LED，目前没有标准的 sysfs 接口。因此，该任务更
适合用户空间工具。

Octo 暴露 4 个物理和 16 个虚拟温度传感器、一个流量传感器，以及 8 个 PWM 可控风扇，连同它们的
转速（RPM）、功率、电压和电流。流量传感器脉冲也可用。

Quadro 暴露 4 个物理和 16 个虚拟温度传感器、一个流量传感器和 4 个 PWM 可控风扇，连同它们的
转速（RPM）、功率、电压和电流。流量传感器脉冲也可用。

Farbwerk 和 Farbwerk 360 暴露 4 个温度传感器。此外，Farbwerk 360 的 16 个虚拟温度传感器也被
暴露。

High Flow Next 暴露 +5V 电压、水质、电导率和流量读数。可以连接一个温度传感器到它，这种情况下
它提供其读数以及液冷回路中耗散/吸收功率的估计值。

Leakshield 暴露 2 个温度传感器和冷却液压力（当前、最小、最大和目标读数）。它还暴露估计的
储液罐体积以及其中充入冷却液的量。可以设置泵 RPM 和流量以增强设备上的计算，但这在此尚未
实现。

Aquastream XT 泵暴露冷却液、外部传感器和风扇 IC 的温度读数。它还暴露泵和风扇转速（RPM）、电压
以及泵电流。

Aquastream Ultimate 泵暴露冷却液温度和外部温度传感器，以及泵和可选连接的风扇的转速、功率、电压
和电流。它还暴露压力和流速读数。

Poweradjust 3 控制器暴露一个外部温度传感器。

High Flow USB 暴露一个内部和外部温度传感器，以及一个流量计。

MPS Flow 设备暴露与 High Flow USB 相同的条目，因为它们具有相同的 USB 产品 ID 并以相同方式
报告传感器。

根据设备的不同，并非所有 sysfs 和 debugfs 条目都可用。写入虚拟温度传感器目前不受支持。

### 使用说明


这些设备通过 HID 报告通信。驱动由内核自动加载，并支持热插拔。

### Sysfs 条目


================ ==============================================================
temp[1-20]_input 物理/虚拟温度传感器（单位毫摄氏度）
temp[1-8]_offset 温度传感器校正偏移（单位毫摄氏度）
fan[1-9]_input   泵/风扇转速（RPM）/ 流速（单位 dL/h）
fan1_min         最小风扇转速（RPM）
fan1_max         最大风扇转速（RPM）
fan1_target      目标风扇转速（RPM）
fan5_pulses      Quadro 流量传感器脉冲
fan9_pulses      Octo 流量传感器脉冲
power[1-8]_input 泵/风扇功率（单位微瓦）
in[0-7]_input    泵/风扇电压（单位毫伏）
curr[1-8]_input  泵/风扇电流（单位毫安）
pwm[1-8]         风扇 PWM（0 - 255）
================ ==============================================================

### Debugfs 条目


================ =================================================
serial_number    设备的序列号
firmware_version 已安装固件的版本
power_cycles     设备被上电的次数
================ =================================================
