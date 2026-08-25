
## 内核驱动 stef48h28


支持的芯片：

  - Analog Devices STEF48H28

    Prefix: 'stef48h28'

    Addresses scanned: -

    Datasheet: https://www.st.com/resource/en/data_brief/stef48h28.pdf

Author:

  - Charles Hsu <hsu.yungteng@gmail.com>


### 描述


STEF48H28 是一款面9-80 V DC 电源轨的 30 A 集成 e-fuse。它提供涌入电流控制（inrush control）、欠过压锁定（undervoltage/overvoltage lockout）以及使用自适应（I x t）方案的过流保护，该方案允许 CPU/GPU 负载典型的短时大电流脉冲
该器件提供模拟电流监视输出与片上温度监视信号用于系统监管。启动行为可通过插入延迟（insertion-delay）与软启动（soft-start）设置进行编程
附加特性包括电源良好（power-good）指示、自检（self-diagnostics）、热关断（thermal shutdown）以及用于遥测（telemetry）与状态报告的 PMBus 接口
### 平台数据支持


该驱动支持标准的 PMBus 驱动平台数据
### Sysfs 接口


======================  ========================================================
in1_label		"vin".
in1_input		测得的电压。来READ_VIN 寄存器in1_min			最小电压。来VIN_UV_WARN_LIMIT 寄存器in1_max			最大电压。来VIN_OV_WARN_LIMIT 寄存器
in2_label		"vout1".
in2_input		测得的电压。来READ_VOUT 寄存器in2_min			最小电压。来VOUT_UV_WARN_LIMIT 寄存器in2_max			最大电压。来VOUT_OV_WARN_LIMIT 寄存器
curr1_label "iin".      curr1_input 测得的电流。来READ_IIN 寄存器
curr2_label "iout1".    curr2_input 测得的电流。来READ_IOUT 寄存器
power1_label		"pin"
power1_input		测得的输入功率。来READ_PIN 寄存器
power2_label		"pout1"
power2_input		测得的输出功率。来READ_POUT 寄存器
temp1_input		测得的温度。来READ_TEMPERATURE_1 寄存器temp1_max		最大温度。来OT_WARN_LIMIT 寄存器temp1_crit		临界高温。来OT_FAULT_LIMIT 寄存器
temp2_input		测得的温度。来READ_TEMPERATURE_2 寄存器======================  ========================================================
