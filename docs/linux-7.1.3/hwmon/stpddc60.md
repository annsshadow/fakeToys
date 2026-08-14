
## 内核驱动 stpddc60


Supported chips:

  - ST STPDDC60

    Prefix: 'stpddc60', 'bmr481'

    Addresses scanned: -

    Datasheet: https://flexpowermodules.com/documents/fpm-techspec-bmr481

Author: Erik Rosen <erik.rosen@metormote.com>


### 描述


本驱动支持对 ST STPDDC60 控制器芯片及兼容模块的硬件监控。

该驱动是核心 PMBus 驱动的客户端驱动。有关 PMBus 客户端驱动的详细信息，请参阅 Documentation/hwmon/pmbus.rst 与 Documentation.hwmon/pmbus-core。


### 使用注意事项


本驱动不会自动检测设备。你需要显式地实例化设备。详情请参阅 Documentation/i2c/instantiating-devices.rst。

vout 的下限与上限过压限值，是相对于指令输出电压、以 50mV 到 400mV 区间、按 50mV 步长的正或负偏移来设置的。这意味着当指令输出电压改变时，这些限值的绝对值也会改变。此外，在写入这些限值时应当小心，因为在最坏情况下，指令输出电压可能会与限值写入同时改变，从而导致不可预测的结果。


### 平台数据支持


该驱动支持标准的 PMBus 驱动平台数据。


### Sysfs 条目


支持以下属性。Vin、iout、pout 与 temp 限值为读写；所有其他属性均为只读。

======================= ========================================================
in1_label		"vin"
in1_input		测得的输入电压。
in1_lcrit		临界最小输入电压。
in1_crit		临界最大输入电压。
in1_lcrit_alarm		输入电压临界低报警。
in1_crit_alarm		输入电压临界高报警。

in2_label		"vout1"
in2_input		测得的输出电压。
in2_lcrit		临界最小输出电压。
in2_crit		临界最大输出电压。
in2_lcrit_alarm		输出电压临界低报警。
in2_crit_alarm		输出电压临界高报警。

curr1_label		"iout1"
curr1_input		测得的输出电流。
curr1_max		最大输出电流。
curr1_max_alarm		输出电流高报警。
curr1_crit		临界最大输出电流。
curr1_crit_alarm	输出电流临界高报警。

power1_label		"pout1"
power1_input		测得的输出功率。
power1_crit		临界最大输出功率。
power1_crit_alarm	输出功率临界高报警。

temp1_input		测得的所有相的最大温度。
temp1_max		最高温度限值。
temp1_max_alarm		高温报警。
temp1_crit		临界最高温度限值。
temp1_crit_alarm	临界最高温度报警。
======================= ========================================================
