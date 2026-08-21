## 内核驱动 acbel-fsg032


支持的设备：

  - ACBEL FSG032-00xG 电源
Author: Lakshmi Yadlapati <lakshmiy@us.ibm.com>

### 描述


该驱动支ACBEL FSG032-00xG 电源。该驱动是核PMBus 驱动的一个客户端
### 使用说明


该驱动不会自动探测设备。你必须显式地实例化这些设备。详Documentation/i2c/instantiating-devices.rst
### Sysfs 属

支持以下属性：

======================= ======================================================
curr1_crit              临界最大电流curr1_crit_alarm        输入电流临界告警curr1_input             测得的输出电流curr1_label             "iin"
curr1_max               最大输入电流curr1_max_alarm         最大输入电流高告警curr1_rated_max         最大额定输入电流curr2_crit              临界最大电流curr2_crit_alarm        输出电流临界告警curr2_input             测得的输出电流curr2_label             "iout1"
curr2_max               最大输出电流curr2_max_alarm         输出电流高告警curr2_rated_max         最大额定输出电流

fan1_alarm              风扇 1 警告fan1_fault	       风扇 1 故障fan1_input	       风扇 1 转速（RPM）fan1_target             设置风扇转速参考值
in1_alarm               输入电压欠压告警in1_input               测得的输入电压in1_label               "vin"
in1_rated_max           最大额定输入电压in1_rated_min           最小额定输入电压in2_crit                临界最大输出电压in2_crit_alarm          输出电压临界高告警in2_input               测得的输出电压in2_label               "vout1"
in2_lcrit               临界最小输出电压in2_lcrit_alarm         输出电压临界低告警in2_rated_max           最大额定输出电压in2_rated_min           最小额定输出电压
power1_alarm            输入故障或告警power1_input            测得的输入功率power1_label            "pin"
power1_max              输入功率限制power1_rated_max        最大额定输入功率power2_crit             临界输出功率限制power2_crit_alarm       输出功率临界告警限制被超过power2_input            测得的输出功率power2_label            "pout"
power2_max              输出功率限制power2_max_alarm        输出功率高告警power2_rated_max        最大额定输出功率
temp[1-3]_input         测得的温度temp[1-2]_max           最高温度temp[1-3]_rated_max     温度高告警======================= ======================================================
