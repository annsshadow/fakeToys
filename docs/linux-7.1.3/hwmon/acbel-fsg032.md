## 内核驱动 acbel-fsg032


支持的设备：

  - ACBEL FSG032-00xG 电源。

Author: Lakshmi Yadlapati <lakshmiy@us.ibm.com>

### 描述


该驱动支持 ACBEL FSG032-00xG 电源。该驱动是核心 PMBus 驱动的一个客户端。

### 使用说明


该驱动不会自动探测设备。你必须显式地实例化这些设备。详见 Documentation/i2c/instantiating-devices.rst。

### Sysfs 属性


支持以下属性：

======================= ======================================================
curr1_crit              临界最大电流。
curr1_crit_alarm        输入电流临界告警。
curr1_input             测得的输出电流。
curr1_label             "iin"
curr1_max               最大输入电流。
curr1_max_alarm         最大输入电流高告警。
curr1_rated_max         最大额定输入电流。
curr2_crit              临界最大电流。
curr2_crit_alarm        输出电流临界告警。
curr2_input             测得的输出电流。
curr2_label             "iout1"
curr2_max               最大输出电流。
curr2_max_alarm         输出电流高告警。
curr2_rated_max         最大额定输出电流。


fan1_alarm              风扇 1 警告。
fan1_fault	       风扇 1 故障。
fan1_input	       风扇 1 转速（RPM）。
fan1_target             设置风扇转速参考值。

in1_alarm               输入电压欠压告警。
in1_input               测得的输入电压。
in1_label               "vin"
in1_rated_max           最大额定输入电压。
in1_rated_min           最小额定输入电压。
in2_crit                临界最大输出电压。
in2_crit_alarm          输出电压临界高告警。
in2_input               测得的输出电压。
in2_label               "vout1"
in2_lcrit               临界最小输出电压。
in2_lcrit_alarm         输出电压临界低告警。
in2_rated_max           最大额定输出电压。
in2_rated_min           最小额定输出电压。

power1_alarm            输入故障或告警。
power1_input            测得的输入功率。
power1_label            "pin"
power1_max              输入功率限制。
power1_rated_max        最大额定输入功率。
power2_crit             临界输出功率限制。
power2_crit_alarm       输出功率临界告警限制被超过。
power2_input            测得的输出功率。
power2_label            "pout"
power2_max              输出功率限制。
power2_max_alarm        输出功率高告警。
power2_rated_max        最大额定输出功率。

temp[1-3]_input         测得的温度。
temp[1-2]_max           最高温度。
temp[1-3]_rated_max     温度高告警。
======================= ======================================================
