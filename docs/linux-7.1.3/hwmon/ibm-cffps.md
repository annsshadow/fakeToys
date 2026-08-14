## ibm-cffps 内核驱动


支持的芯片：

  - IBM Common Form Factor power supply

Author: Eddie James <eajames@us.ibm.com>

### 描述


该驱动支持 IBM 通用外形规格（CFF）电源。该驱动是核心 PMBus 驱动的客户端。

### 使用说明


该驱动不会自动检测设备。你需要显式实例化设备。详细信息请参阅
Documentation/i2c/instantiating-devices.rst。

### Sysfs 条目


支持以下属性：

======================= ======================================================
curr1_alarm		输出电流过流告警。
curr1_input		测量的输出电流（单位 mA）。
curr1_label		"iout1"

fan1_alarm		风扇 1 警告。
fan1_fault		风扇 1 故障。
fan1_input		风扇 1 转速（RPM）。
fan2_alarm		风扇 2 警告。
fan2_fault		风扇 2 故障。
fan2_input		风扇 2 转速（RPM）。

in1_alarm		输入电压欠压告警。
in1_input		测量的输入电压（单位 mV）。
in1_label		"vin"
in2_alarm		输出电压过压告警。
in2_input		测量的输出电压（单位 mV）。
in2_label		"vout1"

power1_alarm		输入故障或告警。
power1_input		测量的输入功率（单位 uW）。
power1_label		"pin"

temp1_alarm		PSU 进风口环境温度过温告警。
temp1_input		测量的 PSU 进风口环境温度（单位毫摄氏度）。
temp2_alarm		次级整流器温度过温告警。
temp2_input		测量的次级整流器温度（单位毫摄氏度）。
temp3_alarm		ORing FET 温度过温告警。
temp3_input		测量的 ORing FET 温度（单位毫摄氏度）。
======================= ======================================================
