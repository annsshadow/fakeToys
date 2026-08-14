
## Intel(R) Dynamic Platform and Thermal Framework Sysfs 接口


:版权: © 2022 Intel Corporation

:作者: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 简介


Intel(R) Dynamic Platform and Thermal Framework（DPTF，动态平台与热管理框架）是一个用于电源与热管理的平台级硬件/软件解决方案。

作为一个容纳多种电源/热管理技术的容器，DPTF 为不同策略影响系统硬件状态提供了一种协调的方式。

由于它是一个平台级框架，因此包含多个组件。该技术的部分内容实现在固件中，并使用 ACPI 与 PCI 设备来暴露各种监控与控制功能。Linux 拥有一组内核对用户空间暴露硬件接口的驱动。这使得诸如 “Linux Thermal Daemon” 之类的用户空间热管理方案能够读取平台特定的热与电源表，从而在将系统保持在热限制范围内的同时提供充足的性能。

### DPTF ACPI 驱动接口


`/sys/bus/platform/devices/<N>/uuids`，其中 <N>
=INT3400|INTC1040|INTC1041|INTC10A0

`available_uuids` (RO)
	一组 UUID 字符串，表示可用的策略。当
	用户空间能够支持这些策略时，应将其通知给固件。

	UUID 字符串：

	"42A441D6-AE6A-462b-A84B-4A8CE79027D3" : 被动 1（Passive 1）

	"3A95C389-E4B8-4629-A526-C52C88626BAE" : 主动（Active）

	"97C68AE7-15FA-499c-B8C9-5DA81D606E0A" : 严重（Critical）

	"63BE270F-1C11-48FD-A6F7-3AF253FF3E2D" : 自适应性能（Adaptive performance）

	"5349962F-71E6-431D-9AE8-0A635B710AEE" : 紧急呼叫（Emergency call）

	"9E04115A-AE87-4D1C-9500-0F3E340BFE75" : 被动 2（Passive 2）

	"F5A35014-C209-46A4-993A-EB56DE7530A1" : Power Boss

	"6ED722A7-9240-48A5-B479-31EEF723D7CF" : 虚拟传感器（Virtual Sensor）

	"16CAF1B7-DD38-40ED-B1C1-1B8A1913D531" : 散热模式（Cooling mode）

	"BE84BABF-C4D4-403D-B495-3128FD44dAC1" : HDC

`current_uuid` (RW)
	用户空间可以一次一个地写入来自可用 UUID 的字符串。

`/sys/bus/platform/devices/<N>/`，其中 <N>
=INT3400|INTC1040|INTC1041|INTC10A0

`imok` (WO)
	用户空间守护进程写入 1 以响应固件的发送保活（keep alive）通知事件。当固件调用 imok ACPI 方法要求用户空间响应时，用户空间会收到
	THERMAL_EVENT_KEEP_ALIVE kobject uevent 通知。

`odvp*` (RO)
	固件热状态变量值。热表会根据这些变量值的不同进行不同的处理。

`data_vault` (RO)
	二进制热表。解码热表请参阅
	https:/github.com/intel/thermal_daemon。

`production_mode` (RO)
	当不为零时，制造商锁定了热配置，禁止进一步更改。

### ACPI 热关系表接口


`/dev/acpi_thermal_rel`

	该设备提供 IOCTL 接口，通过 ACPI 方法 _TRT 与 _ART 读取标准 ACPI 热关系表。这些 IOCTL 定义在
	drivers/thermal/intel/int340x_thermal/acpi_thermal_rel.h 中

	IOCTL：

	ACPI_THERMAL_GET_TRT_LEN: 获取 TRT 表的长度

	ACPI_THERMAL_GET_ART_LEN: 获取 ART 表的长度

	ACPI_THERMAL_GET_TRT_COUNT: TRT 表中的记录数

	ACPI_THERMAL_GET_ART_COUNT: ART 表中的记录数

	ACPI_THERMAL_GET_TRT: 读取二进制 TRT 表，读取长度通过 ioctl() 的参数提供。

	ACPI_THERMAL_GET_ART: 读取二进制 ART 表，读取长度通过 ioctl() 的参数提供。

### DPTF ACPI 传感器驱动


DPTF 传感器驱动以标准的热管理 sysfs thermal_zone 形式呈现。


### DPTF ACPI 散热驱动


DPTF 散热驱动以标准的热管理 sysfs cooling_device 形式呈现。


### DPTF 处理器热管理 PCI 驱动接口


`/sys/bus/pci/devices/0000\:00\:04.0/power_limits/`

有关 powercap ABI 请参阅 Documentation/power/powercap/powercap.rst。

`power_limit_0_max_uw` (RO)
	Intel RAPL 的 powercap sysfs constraint_0_power_limit_uw 的最大值

`power_limit_0_step_uw` (RO)
	Intel RAPL 约束 0 功率限制的功率增量/减量

`power_limit_0_min_uw` (RO)
	Intel RAPL 的 powercap sysfs constraint_0_power_limit_uw 的最小值

`power_limit_0_tmin_us` (RO)
	Intel RAPL 的 powercap sysfs constraint_0_time_window_us 的最小值

`power_limit_0_tmax_us` (RO)
	Intel RAPL 的 powercap sysfs constraint_0_time_window_us 的最大值

`power_limit_1_max_uw` (RO)
	Intel RAPL 的 powercap sysfs constraint_1_power_limit_uw 的最大值

`power_limit_1_step_uw` (RO)
	Intel RAPL 约束 1 功率限制的功率增量/减量

`power_limit_1_min_uw` (RO)
	Intel RAPL 的 powercap sysfs constraint_1_power_limit_uw 的最小值

`power_limit_1_tmin_us` (RO)
	Intel RAPL 的 powercap sysfs constraint_1_time_window_us 的最小值

`power_limit_1_tmax_us` (RO)
	Intel RAPL 的 powercap sysfs constraint_1_time_window_us 的最大值

`power_floor_status` (RO)
	当置为 1 时，表示当前配置下已达到系统的功率下限。需要重新配置才能进一步降低功率。

`power_floor_enable` (RW)
	当置为 1 时，启用功率下限状态的读取与通知。当 power_floor_status 属性值发生变化时会触发通知。

`/sys/bus/pci/devices/0000\:00\:04.0/`

`tcc_offset_degree_celsius` (RW)
	硬件将限制 CPU 的临界温度之上的 TCC 偏移量。

`/sys/bus/pci/devices/0000\:00\:04.0/workload_request`

`workload_available_types` (RO)
	可用的工作负载类型。用户空间可以通过 workload_type 指定它当前正在执行的某一工作负载类型。例如：idle（空闲）、bursty（突发）、sustained（持续）等。

`workload_type` (RW)
	用户空间可以通过此接口指定任意一个可用的工作负载类型。

`/sys/bus/pci/devices/0000\:00\:04.0/ptc_0_control`
`/sys/bus/pci/devices/0000\:00\:04.0/ptc_1_control`
`/sys/bus/pci/devices/0000\:00\:04.0/ptc_2_control`

所有这些控制都需要管理员权限才能更新。

`enable` (RW)
	1 表示启用，0 表示禁用。显示平台温度控制功能当前的启用状态。用户空间可以启用/禁用硬件控制。

`temperature_target` (RW)
	更新硬件用于温度控制的新温度目标，单位为毫摄氏度。

`thermal_tolerance` (RW)
	该属性取值范围为 0 到 7，其中 0 表示最激进的控制以避免任何温度超调，7 表示更平缓的方式，即便以温度超调为代价也偏向性能。
	注意：该级别可能并非线性缩放。例如，值 3 并不一定意味着相比值 0 有 50% 的性能提升。

鉴于这是平台温度控制，期望由单一的用户级管理器拥有并管理这些控制。如果多个用户级软件应用尝试写入不同的目标，可能导致非预期的行为。


### DPTF 处理器热管理 RFIM 接口


RFIM 接口允许调整 FIVR（全集成电压调节器）、DDR（双倍数据速率）与 DLVR（数字线性电压调节器）的频率，以避免对 WiFi 与 5G 的射频干扰。

开关电压调节器（VR）会在基频及其谐波处产生辐射 EMI 或 RFI。某些谐波可能会干扰集成到笔记本等主机系统中非常敏感的无线接收器，如 Wi-Fi 与蜂窝网络。缓解方法之一是请求将 SoC 集成的 VR（IVR）开关频率调整一个小的百分比，并将开关噪声的谐波干扰从无线信道移开。OEM 或 ODM 可以使用该驱动在不会影响 IVR 性能的范围内控制 SoC IVR 的运行。

某些产品使用 DLVR 而非 FIVR 作为开关电压调节器。在这种情况下，必须调整 DLVR 的属性而非 FIVR。

在移动频率时可能会引入额外的时钟噪声，这可以通过调整扩频百分比来补偿。这有助于降低时钟噪声以满足法规合规要求。该扩频百分比增加了信号传输的带宽，从而减少干扰、噪声与信号衰落的影响。

DDR IO 接口的 DRAM 设备及其电源平面可能在数据速率处产生 EMI。与 IVR 控制机制类似，Intel 提供了一种机制，在满足若干条件时改变 DDR 数据速率：由于 DDR 存在强烈的 RFI 干扰；CPU 电源管理在改变 DDR 数据速率方面没有其他限制；PC ODM 在 BIOS 中为此启用了该特性（实时 DDR RFI 缓解，称为 DDR-RFIM）以用于 Wi-Fi。


FIVR 属性

`/sys/bus/pci/devices/0000\:00\:04.0/fivr/`

`vco_ref_code_lo` (RW)
	VCO 参考码是一个 11 位字段，控制 FIVR 的开关频率。这是 3 位的低字节（LSB）字段。

`vco_ref_code_hi` (RW)
	VCO 参考码是一个 11 位字段，控制 FIVR 的开关频率。这是 8 位的高字节（MSB）字段。

`spread_spectrum_pct` (RW)
	设置 FIVR 扩频时钟百分比

`spread_spectrum_clk_enable` (RW)
	FIVR 扩频时钟特性的启用/禁用

`rfi_vco_ref_code` (RW)
	该字段是一个只读状态寄存器，反映当前 FIVR 开关频率

`fivr_fffc_rev` (RW)
	该字段指示 FIVR 硬件的修订版本。


DVFS 属性

`/sys/bus/pci/devices/0000\:00\:04.0/dvfs/`

`rfi_restriction_run_busy` (RW)
	请求限制特定的 DDR 数据速率，并将此值置为 1。操作完成后自动复位为 0。

`rfi_restriction_err_code` (RW)
	0：请求被接受，1：特性被禁用，
	2：请求限制的点数超过允许值

`rfi_restriction_data_rate_Delta` (RW)
	用于 RFI 保护的受限 DDR 数据速率：下限

`rfi_restriction_data_rate_Base` (RW)
	用于 RFI 保护的受限 DDR 数据速率：上限

`ddr_data_rate_point_0` (RO)
	DDR 数据速率选择第 1 个点

`ddr_data_rate_point_1` (RO)
	DDR 数据速率选择第 2 个点

`ddr_data_rate_point_2` (RO)
	DDR 数据速率选择第 3 个点

`ddr_data_rate_point_3` (RO)
	DDR 数据速率选择第 4 个点

`rfi_disable (RW)`
	禁用 DDR 速率改变特性

DLVR 属性

`/sys/bus/pci/devices/0000\:00\:04.0/dlvr/`

`dlvr_hardware_rev` (RO)
	DLVR 硬件修订版本。

`dlvr_freq_mhz` (RO)
	当前 DLVR PLL 频率，单位为 MHz。

`dlvr_freq_select` (RW)
	设置 DLVR PLL 时钟频率。一旦设置并通过 dlvr_rfim_enable 启用，dlvr_freq_mhz 将显示当前 DLVR PLL 频率。

`dlvr_pll_busy` (RO)
	置位时 PLL 无法接受频率变更。

`dlvr_rfim_enable` (RW)
	0：禁用射频跳频，1：启用射频跳频。

`dlvr_spread_spectrum_pct` (RW)
	设置 DLVR 扩频百分比值。

`dlvr_control_mode` (RW)
        指定使用扩频时频率如何展开。
        0：向下展开（Down spread），
        1：中心展开（Spread in the Center）。

`dlvr_control_lock` (RW)
    1：后续写入被忽略。

### DPTF 电源与电池接口


请参阅 Documentation/ABI/testing/sysfs-platform-dptf

### DPTF 风扇控制


请参阅 Documentation/admin-guide/acpi/fan_performance_states.rst

### 工作负载类型提示


Meteor Lake 处理器代的固件能够识别工作负载类型，并将有关它的提示传递给 OS。提供了一个特殊的 sysfs 接口，允许用户空间从固件获取工作负载类型提示，并控制其提供的速率。

用户空间可以轮询属性 “workload_type_index” 获取当前提示，也可以在该属性值更新时收到通知。

file:`/sys/bus/pci/devices/0000:00:04.0/workload_hint/`
段 0、总线 0、设备 4、功能 0 在所有 Intel 客户端处理器上都保留给处理器热设备。因此，上述路径不会随处理器代的更迭而改变。

`workload_hint_enable` (RW)
	启用固件向用户空间发送工作负载类型提示。

`workload_slow_hint_enable` (RW)
	启用固件向用户空间发送慢速工作负载类型提示。

`notification_delay_ms` (RW)
	固件通知 OS 之前的最小延迟，单位为毫秒。这用于控制通知的速率。该延迟介于固件改变工作负载类型预测与将改变通知 OS 之间。默认延迟为 1024 ms。延迟为 0 是无效的。延迟会被向上取整到最接近的 2 的幂，以简化固件对延迟值的编程。读取 notification_delay_ms 属性会显示所使用的有效值。

`workload_type_index` (RO)
	预测的工作负载类型索引。用户空间可以通过现有的 sysfs 属性变更通知机制获得变更通知。

	Meteor Lake 处理器代所支持的索引值及其含义如下：

	0 -  空闲（Idle）：系统不执行任何任务，功耗与空闲驻留时间长时间持续偏低。

	1 – 电池续航（Battery Life）：功耗相对较低，但处理器可能仍在主动执行任务，例如长时间的视频播放。

	2 – 持续（Sustained）：在较长一段时间内功耗相对较高，几乎没有空闲时段，最终会耗尽 RAPL Power Limit 1 与 2。

	3 – 突发（Bursty）：消耗相对恒定的平均功率，但相对空闲的时段会被突发活动打断。突发相对较短，其间相对空闲的时段通常能防止 RAPL Power Limit 1 被耗尽。

	4 – 未知（Unknown）：无法分类。

	从 Panther Lake 开始的处理器提供了额外的提示。硬件在较长一段时间内分析工作负载驻留情况，以确定该工作负载分类倾向于空闲/电池续航状态还是持续/性能状态。基于此长期分析，它分类如下：

	功耗分类（Power Classification）：如果工作负载表现出更多的空闲或电池续航驻留，则归类为 “power”（功耗）。

	性能分类（Performance Classification）：如果工作负载表现出更多的持续或性能驻留，则归类为 “performance”（性能）。

	这种方式使应用可以忽略短期的工作负载波动，转而响应长期的功耗与性能趋势。

	该分类的驻留阈值是 CPU 代特定的。分类通过 workload_type_index 的第 4 位报告：

	第 4 位 = 1：功耗分类（Power classification）

	第 4 位 = 0：性能分类（Performance classification）
