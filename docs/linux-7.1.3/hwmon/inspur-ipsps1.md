## 内核驱动 inspur-ipsps1


支持的芯片：

  - Inspur Power System 电源供应单元

Author: John Wang <wangzqbj@inspur.com>

### 描述


该驱动支持 Inspur Power System 电源。该驱动是核心 PMBus 驱动的一个客户端。

### 使用说明


该驱动不会自动检测设备。你必须显式实例化设备。详细信息请参阅 Documentation/i2c/instantiating-devices.rst。

### Sysfs 接口


支持以下属性：

======================= ======================================================
curr1_input		测得的输入电流
curr1_label		"iin"
curr1_max		最大电流
curr1_max_alarm		电流过高报警
curr2_input		测得的输出电流（单位 mA）。
curr2_label		"iout1"
curr2_crit		临界最大电流
curr2_crit_alarm	电流临界过高报警
curr2_max		最大电流
curr2_max_alarm		电流过高报警

fan1_alarm		风扇 1 警告。
fan1_fault		风扇 1 故障。
fan1_input		风扇 1 转速（单位 RPM）。

in1_alarm		输入电压欠压报警。
in1_input		测得的输入电压（单位 mV）。
in1_label		"vin"
in2_input		测得的输出电压（单位 mV）。
in2_label		"vout1"
in2_lcrit		临界最小输出电压
in2_lcrit_alarm		输出电压临界过低报警
in2_max			最大输出电压
in2_max_alarm		输出电压过高报警
in2_min			最小输出电压
in2_min_alarm		输出电压过低报警

power1_alarm		输入故障或报警。
power1_input		测得的输入功率（单位 uW）。
power1_label		"pin"
power1_max		输入功率限制
power2_max_alarm	输出功率过高报警
power2_max		输出功率限制
power2_input		测得的输出功率（单位 uW）。
power2_label		"pout"

temp[1-3]_input		测得的温度
temp[1-2]_max		最大温度
temp[1-3]_max_alarm	温度过高报警

vendor			制造商名称
model			产品型号
part_number		产品部件号
serial_number		产品序列号
fw_version		固件版本
hw_version		硬件版本
mode			工作模式。可设置为 active 或
			standby，当设置为 standby 时，PSU 将在
			standby 与 redundancy 模式之间自动切换。
======================= ======================================================
