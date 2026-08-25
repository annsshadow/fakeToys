## dps920ab 内核驱动


支持的芯片：

  - Delta DPS920AB

    Prefix: 'dps920ab'

    Addresses scanned: -

Authors:
    Robert Marko <robert.marko@sartura.hr>


### 描述


该驱动实现对 Delta DPS920AB 920W 54V 直流单输出、支PMBus 的电源的支持
该驱动是核心 PMBus 驱动的客户端驱动。有PMBus 客户端驱动的详细信息，请
参阅 Documentation/hwmon/pmbus.rst

### 使用说明


该驱动不会自动检测设备。你需要显式实例化设备。详细信息请参阅
Documentation/i2c/instantiating-devices.rst銆。

### Sysfs 条目


======================= ======================================================
curr1_label		"iin"
curr1_input		测量的输入电curr1_alarm		输入电流高告
curr2_label		"iout1"
curr2_input		测量的输出电curr2_max		最大输出电curr2_rated_max		额定最大输出电
in1_label		"vin"
in1_input		测量的输入电in1_alarm		输入电压告警

in2_label		"vout1"
in2_input		测量的输出电in2_rated_min		额定最小输出电in2_rated_max		额定最大输出电in2_alarm		输出电压告警

power1_label		"pin"
power1_input		测量的输入功power1_alarm		输入功率高告
power2_label		"pout1"
power2_input		测量的输出功power2_rated_max		额定最大输出功
temp[1-3]_input		测量的温temp[1-3]_alarm		温度告警

fan1_alarm		风扇 1 警告fan1_fault		风扇 1 故障fan1_input		风扇 1 转速（RPM）======================= ======================================================
