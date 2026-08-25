## 内核驱动 Lochnagar


支持的系统：
  - Cirrus Logic ：Lochnagar 2

作者：Lucas A. Tanure Alves

### 描述


Lochnagar 2 内置 Current Monitor（电流监视）电路，可对供minicard 的至八路供电电压轨同时测量电压和电流。该 Current Monitor 工作时不需任何硬件
修改或外部电路
电流与电压测量值通过访问 Lochnagar 板卡控制器的标准寄存器映射接口获得，
因此可由软件进行监视
### Sysfs 属

======================= =======================================================
temp1_input             Lochnagar 板卡温度（milliCelsiusin0_input               测量得到DBVDD1 电压（milliVoltsin0_label               "DBVDD1"
curr1_input             测量得到DBVDD1 电流（milliAmpscurr1_label             "DBVDD1"
power1_average          测量得到DBVDD1 平均功率（microWattspower1_average_interval 功率平均时间输入，有效范1 1708mS
power1_label            "DBVDD1"
in1_input               测量得到1V8 DSP 电压（milliVoltsin1_label               "1V8 DSP"
curr2_input             测量得到1V8 DSP 电流（milliAmpscurr2_label             "1V8 DSP"
power2_average          测量得到1V8 DSP 平均功率（microWattspower2_average_interval 功率平均时间输入，有效范1 1708mS
power2_label            "1V8 DSP"
in2_input               测量得到1V8 CDC 电压（milliVoltsin2_label               "1V8 CDC"
curr3_input             测量得到1V8 CDC 电流（milliAmpscurr3_label             "1V8 CDC"
power3_average          测量得到1V8 CDC 平均功率（microWattspower3_average_interval 功率平均时间输入，有效范1 1708mS
power3_label            "1V8 CDC"
in3_input               测量得到VDDCORE DSP 电压（milliVoltsin3_label               "VDDCORE DSP"
curr4_input             测量得到VDDCORE DSP 电流（milliAmpscurr4_label             "VDDCORE DSP"
power4_average          测量得到VDDCORE DSP 平均功率（microWattspower4_average_interval 功率平均时间输入，有效范1 1708mS
power4_label            "VDDCORE DSP"
in4_input               测量得到AVDD 1V8 电压（milliVoltsin4_label               "AVDD 1V8"
curr5_input             测量得到AVDD 1V8 电流（milliAmpscurr5_label             "AVDD 1V8"
power5_average          测量得到AVDD 1V8 平均功率（microWattspower5_average_interval 功率平均时间输入，有效范1 1708mS
power5_label            "AVDD 1V8"
curr6_input             测量得到SYSVDD 电流（milliAmpscurr6_label             "SYSVDD"
power6_average          测量得到SYSVDD 平均功率（microWattspower6_average_interval 功率平均时间输入，有效范1 1708mS
power6_label            "SYSVDD"
in6_input               测量得到VDDCORE CDC 电压（milliVoltsin6_label               "VDDCORE CDC"
curr7_input             测量得到VDDCORE CDC 电流（milliAmpscurr7_label             "VDDCORE CDC"
power7_average          测量得到VDDCORE CDC 平均功率（microWattspower7_average_interval 功率平均时间输入，有效范1 1708mS
power7_label            "VDDCORE CDC"
in7_input               测量得到MICVDD 电压（milliVoltsin7_label               "MICVDD"
curr8_input             测量得到MICVDD 电流（milliAmpscurr8_label             "MICVDD"
power8_average          测量得到MICVDD 平均功率（microWattspower8_average_interval 功率平均时间输入，有效范1 1708mS
power8_label            "MICVDD"
======================= =======================================================

注意    无法测量 SYSVDD 电压轨上的电压