## Charger Manager（充电管理器）


	(C) 2011 MyungJoo Ham <myungjoo.ham@samsung.com>, GPL

Charger Manager 提供内核内的电池充电管理，它需要在挂起至 RAM（suspend-to-RAM）状态下进行温度监控，并且每块电池可能挂接多个充电器，而用户空间希望查看这多个充电器的聚合信息。

Charger Manager 是一个带有 power-supply-class 条目的 platform_driver。Charger Manager 的一个实例（由 Charger-Manager 创建的 platform-device）代表一块带有充电器的独立电池。如果一个系统中有多块电池各自带有独立工作的充电器，该系统可能需要多个 Charger Manager 实例。

## 1. 简介


Charger Manager 支持以下功能：

- 支持多个充电器（例如，带有 USB、AC 和太阳能板的设备）
	一个系统可能有多个充电器（或电源），其中部分可能同时激活。每个充电器可以拥有自己的 power-supply-class，而每个 power-supply-class 可以提供关于电池状态的不同信息。该框架从多个来源聚合与充电器相关的信息，并以单一 power-supply-class 的形式展示合并后的信息。

- 支持挂起至 RAM 期间的轮询（借助 suspend_again 回调）
	在电池充电且系统处于 suspend-to-RAM 时，我们可能需要通过查看环境或电池温度来监控电池健康。我们可以通过周期性唤醒系统来实现。然而，这种方法会为监控电池健康和任务而唤醒不必要的设备，以及本应保持挂起的用户进程。这反过来会导致不必要的功耗，并拖慢充电过程。甚至，这种峰值功耗可能在充电中途停止充电器（外部输入功率 < 设备功耗），这不仅影响充电时间，也影响电池寿命。

	Charger Manager 提供一个函数 “cm_suspend_again”，可用作 platform_suspend_ops 的 suspend_again 回调。如果平台需要除 cm_suspend_again 之外的其他任务，它可以实现自己的 suspend_again 回调，在中间调用 cm_suspend_again。通常，平台需要恢复并挂起 Charger Manager 使用的一些设备。

- 支持提前的满电事件处理
	如果在满电事件之后经过 “fullbatt_vchkdrop_ms”，电池电压下降了 “fullbatt_vchkdrop_uV”，框架将重新启动充电。该检查也会在挂起期间通过设置相应的唤醒时间并借助 suspend_again 来执行。

- 支持 uevent 通知
	在充电器相关事件发生时，设备向用户发送 UEVENT 通知。

## 2. 与 suspend_again 相关的全局 Charger-Manager 数据

为了为 Charger Manager 配置 suspend_again 特性（挂起中监控），用户应提供 charger_global_desc，并通过 setup_charger_manager(`struct charger_global_desc *`) 进行设置。顾名思义，这个用于挂起中监控的 charger_global_desc 数据是全局的。因此，即使有多块电池，用户也只需提供一次。如果有多个电池，多个 Charger Manager 实例共享同一个 charger_global_desc，它将为所有 Charger Manager 实例管理挂起中监控。

用户需要正确地为 `struct charger_global_desc` 提供全部三个条目，才能激活挂起中监控：

`char *rtc_name;`
	用于从挂起中唤醒系统的 rtc 名称（例如 “rtc0”）。rtc 的闹钟中断（AIE）应当能够唤醒系统。Charger Manager 会保存并恢复闹钟值，并在闹钟将比 Charger Manager 设定的更早触发时使用先前定义的闹钟，从而不干扰先前定义的闹钟。

`bool (*rtc_only_wakeup)(void);`
	该回调应让 CM 知道从挂起中唤醒是否仅由同一结构体中的 “rtc” 闹钟引起。如果有任何其他唤醒源触发了唤醒，它应返回 false。如果 “rtc” 是唯一的唤醒原因，它应返回 true。

`bool assume_timer_stops_in_suspend;`
	如果为 true，Charger Manager 假定定时器（CM 使用 jiffies 作为定时器）在挂起期间停止。那么，CM 假定挂起时长与闹钟长度相同。

## 3. 如何配置 suspend_again

Charger Manager 提供函数 “extern bool cm_suspend_again(void)”。当调用 cm_suspend_again 时，它会监控每一块电池。系统 platform_suspend_ops 的 suspend_ops 回调可以调用 cm_suspend_again 函数，以了解 Charger Manager 是否希望再次挂起。如果没有其他设备或任务想使用 suspend_again 特性，platform_suspend_ops 可以直接将其 suspend_again 回调指向 cm_suspend_again。

如果系统由 Charger Manager 唤醒且轮询（挂起中监控）结果为 “normal”，cm_suspend_again() 返回 true（意为“我希望再次挂起”）。

## 4. Charger-Manager 数据（struct charger_desc）

对于每块独立充电的电池（如果一系列电池由单个充电器充电，则它们算作一块独立电池），会挂接一个 Charger Manager 实例。下列

struct charger_desc 元素：

`char *psy_name;`
	电池的 power-supply-class 名称。若 psy_name 为 NULL，默认为 “battery”。用户可在 “/sys/class/power_supply/[psy_name]/” 访问 psy 条目。

`enum polling_modes polling_mode;`
	  CM_POLL_DISABLE:
		不轮询该电池。
	  CM_POLL_ALWAYS:
		始终轮询该电池。
	  CM_POLL_EXTERNAL_POWER_ONLY:
		当且仅当挂接了外部电源时才轮询该电池。
	  CM_POLL_CHARGING_ONLY:
		当且仅当电池正在充电时才轮询该电池。

`unsigned int fullbatt_vchkdrop_ms; / unsigned int fullbatt_vchkdrop_uV;`
	若两者都具有非零值，Charger Manager 会在电池充满后经过 fullbatt_vchkdrop_ms 检查电池电压下降。如果电压下降超过 fullbatt_vchkdrop_uV，Charger Manager 将尝试通过禁用并重新启用充电器来对电池重新充电。仅根据电压下降条件（不带延迟条件）重新充电，需要借助来自电量计或充电器设备/芯片的硬件中断来实现。

`unsigned int fullbatt_uV;`
	如果指定了非零值，Charger Manager 假定当电池未被充电且电池电压等于或大于 fullbatt_uV 时，电池已充满（容量 = 100）。

`unsigned int polling_interval_ms;`
	所需的轮询间隔（毫秒）。Charger Manager 会每 polling_interval_ms 或更频繁地轮询该电池。

`enum data_source battery_present;`
	CM_BATTERY_PRESENT:
		假定电池存在。
	CM_NO_BATTERY:
		假定电池不存在。
	CM_FUEL_GAUGE:
		从电量计获取电池存在信息。
	CM_CHARGER_STAT:
		从充电器获取电池存在信息。

`char **psy_charger_stat;`
	以 NULL 结尾的数组，包含充电器的 power-supply-class 名称。每个 power-supply-class 应提供 “PRESENT”（若 battery_present 为 “CM_CHARGER_STAT”）、“ONLINE”（显示是否挂接了外部电源）和 “STATUS”（显示电池是否 {“FULL” 或 未满} 或 {“FULL”、“Charging”、“Discharging”、“NotCharging”}）。

`int num_charger_regulators; / struct regulator_bulk_data *charger_regulators;`
	以 regulator 框架批量函数形式表示充电器的调节器。

`char *psy_fuel_gauge;`
	电量计的 power-supply-class 名称。

`int (**temperature_out_of_range)(int **mC); / bool measure_battery_temp;`
	如果温度对充电是安全的，该回调返回 0；如果过热无法充电，返回正数；如果过冷无法充电，返回负数。借助变量 mC，该回调以摄氏度的千分之一返回温度。根据 measure_battery_temp 的值，温度来源可以是电池温度或环境温度。

## 5. 其他注意事项


在充电器/电池相关事件（如电池拔出、充电器拔出、充电器插入、DCIN 过压/欠压、充电器停止）以及其他对充电器至关重要的情况下，系统应被配置为唤醒。至少以下事件应将系统从挂起中唤醒：a) 充电器开/关 b) 外部电源接入/断开 c) 电池装入/取出（充电期间）

这通常通过将 PMIC 配置为唤醒源来实现。
