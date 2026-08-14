
## 内核驱动 intel-m10-bmc-hwmon


支持的设备：

 - Intel MAX 10 BMC（用于 Intel PAC N3000）

   Prefix: 'n3000bmc-hwmon'

Author: Xu Yilun <yilun.xu@intel.com>


### 描述


该驱动为 Intel MAX 10 板管理控制器（BMC）芯片添加了温度、电压、电流和功耗的读取支持。该 BMC 芯片集成在一些 Intel 可编程加速卡（PAC）中。它连接到一组传感器芯片，以监控板上不同组件的传感器数据。BMC 固件负责在共享寄存器中采样和记录传感器数据。主机驱动从这些共享寄存器中读取传感器数据，并以 hwmon 接口的形式暴露给用户。

该 BMC 芯片使用 Intel MAX 10 CPLD 实现。它可以被重新编程为某些变体，以支持不同的 Intel PAC。该驱动设计上能够区分这些变体，但目前仅支持用于 Intel PAC N3000 的 BMC。


### Sysfs 属性


支持以下属性：

- Intel MAX 10 BMC（用于 Intel PAC N3000）：

======================= =======================================================
tempX_input             组件温度（由 tempX_label 指定）
tempX_max               组件温度最大设定点
tempX_crit              组件温度临界设定点
tempX_max_hyst          组件温度最大值的迟滞
tempX_crit_hyst         组件温度临界值的迟滞
temp1_label             "板载温度"
temp2_label             "FPGA 芯片温度"
temp3_label             "QSFP0 温度"
temp4_label             "QSFP1 温度"
temp5_label             "Retimer A 温度"
temp6_label             "Retimer A SerDes 温度"
temp7_label             "Retimer B 温度"
temp8_label             "Retimer B SerDes 温度"

inX_input               组件的测量电压（由 inX_label 指定）
in0_label               "QSFP0 供电电压"
in1_label               "QSFP1 供电电压"
in2_label               "FPGA 核心电压"
in3_label               "12V 背板电压"
in4_label               "1.2V 电压"
in5_label               "12V AUX 电压"
in6_label               "1.8V 电压"
in7_label               "3.3V 电压"

currX_input             组件的测量电流（由 currX_label 指定）
curr1_label             "FPGA 核心电流"
curr2_label             "12V 背板电流"
curr3_label             "12V AUX 电流"

powerX_input            组件的测量功耗（由 powerX_label 指定）
power1_label            "板载功耗"

======================= =======================================================

所有属性均为只读。
