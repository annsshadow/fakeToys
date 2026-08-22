
## 内核驱动 pli1209bc


支持的芯片：

  - Digital Supervisor PLI1209BC

    Prefix: 'pli1209bc'

    Addresses scanned: 0x50 - 0x5F

    Datasheet: https://www.vicorpower.com/documents/datasheets/ds-PLI1209BCxyzz-VICOR.pdf

Authors:
    - Marcello Sylvester Bauer <sylv@sylv.io>

### 描述


Vicor PLI1209BC 是一款隔离的数字电源系统监控器（supervisor），为主机处理器与一个总线转换模块（BCM）之间提供通信接口。PLI 通过一个隔离的 UART 接口PMBus 兼容接口与系统控制器通信。通过 PLI，主机处理器可以配置、设置保护限制并监视 BCM
### Sysfs 接口


======================= ========================================================
in1_label		"vin2"
in1_input		输入电压in1_rated_min		最小额定输入电压in1_rated_max		最大额定输入电压in1_max			最大输入电压in1_max_alarm		输入电压过高报警in1_crit		临界输入电压in1_crit_alarm		输入电压临界报警
in2_label		"vout2"
in2_input		输出电压in2_rated_min		最小额定输出电压in2_rated_max		最大额定输出电压in2_alarm		输出电压报警

curr1_label		"iin2"
curr1_input		输入电流curr1_max		最大输入电流curr1_max_alarm		最大输入电流过高报警curr1_crit		临界输入电流curr1_crit_alarm	输入电流临界报警
curr2_label		"iout2"
curr2_input		输出电流curr2_crit		临界输出电流curr2_crit_alarm	输出电流临界报警curr2_max		最大输出电流curr2_max_alarm		输出电流过高报警
power1_label		"pin2"
power1_input		输入功率power1_alarm		输入功率报警
power2_label		"pout2"
power2_input		输出功率power2_rated_max	最大额定输出功率
temp1_input		芯片（die）温度temp1_alarm		芯片温度报警temp1_max		最大芯片温度temp1_max_alarm		芯片温度过高报警temp1_crit		临界芯片温度temp1_crit_alarm	芯片温度临界报警======================= ========================================================
