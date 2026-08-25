
## 内核驱动 crps


Supported chips:

  - Intel CRPS185

    Prefix: 'crps185'

    Addresses scanned: -

    Datasheet: Only available under NDA.

Authors:
    Ninad Palsule <ninad@linux.ibm.com>


### 描述


本驱动实现对带有 PMBus 支持Intel 通用冗余电源（Common Redundant Power supply）的支持
该驱动是核心 PMBus 驱动的客户端驱动。有PMBus 客户端驱动的详细信息，请参阅 Documentation/hwmon/pmbus.rst

### 使用注意事项


本驱动不会自动检测设备。你需要显式地实例化设备。详情请参阅 Documentation/i2c/instantiating-devices.rst

### Sysfs 条目


======================= ======================================================
curr1_label		"iin"
curr1_input		测得的输入电curr1_max		最大输入电curr1_max_alarm		输入最大电流高报警
curr1_crit		临界高输入电curr1_crit_alarm	输入临界电流高报curr1_rated_max		最大额定输入电
curr2_label		"iout1"
curr2_input		测得的输出电curr2_max		最大输出电curr2_max_alarm		输出最大电流高报警
curr2_crit		临界高输出电curr2_crit_alarm	输出临界电流高报curr2_rated_max		最大额定输出电
in1_label		"vin"
in1_input		测得的输入电in1_crit		临界输入过压
in1_crit_alarm		临界输入过压报警
in1_max			最大输入过in1_max_alarm		最大输入过压报in1_rated_min		最小额定输入电in1_rated_max		最大额定输入电
in2_label		"vout1"
in2_input		测得的输入电in2_crit		临界输入过压
in2_crit_alarm		临界输入过压报警
in2_lcrit		临界输入欠压故障
in2_lcrit_alarm		临界输入欠压故障报警
in2_max			最大输入过in2_max_alarm		最大输入过压报in2_min			最小输入欠压警in2_min_alarm		最小输入欠压警告报in2_rated_min		最小额定输入电in2_rated_max		最大额定输入电
power1_label		"pin"
power1_input		测得的输入功power1_alarm		输入功率高报power1_max  		最大输入功power1_rated_max		最大额定输入功
temp[1-2]_input		测得的温temp[1-2]_crit 		临界温度
temp[1-2]_crit_alarm	临界温度报警
temp[1-2]_max		最高温temp[1-2]_max_alarm	最高温度报temp[1-2]_rated_max	最大额定温
fan1_alarm		风扇 1 警告fan1_fault		风扇 1 故障fan1_input		风扇 1 转速（RPM）fan1_target		风扇 1 目标======================= ======================================================
