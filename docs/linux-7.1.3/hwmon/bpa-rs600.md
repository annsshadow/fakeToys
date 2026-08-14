
## 内核驱动 bpa-rs600


支持的设备：

  - BPA-RS600-120

    Datasheet: Publicly available at the BluTek website
       http://blutekpower.com/wp-content/uploads/2019/01/BPA-RS600-120-07-19-2018.pdf

Authors:
      - Chris Packham <chris.packham@alliedtelesis.co.nz>

### 描述


BPA-RS600 是一款紧凑型 600W 可插拔电源模块。

### 使用说明


本驱动不会探测 PMBus 设备。你必须显式地实例化设备。

### Sysfs 属性


======================= ============================================
curr1_label             "iin"
curr1_input             测量的输入电流
curr1_max               最大输入电流
curr1_max_alarm         输入电流过高告警

curr2_label             "iout1"
curr2_input             测量的输出电流
curr2_max               最大输出电流
curr2_max_alarm         输出电流过高告警

fan1_input              测量的风扇转速
fan1_alarm              风扇警告
fan1_fault              风扇故障

in1_label               "vin"
in1_input               测量的输入电压
in1_max                 最大输入电压
in1_max_alarm           输入电压过高告警
in1_min                 最小输入电压
in1_min_alarm           输入电压过低告警

in2_label               "vout1"
in2_input               测量的输出电压
in2_max                 最大输出电压
in2_max_alarm           输出电压过高告警
in2_min                 最小输出电压
in2_min_alarm           输出电压过低告警

power1_label            "pin"
power1_input            测量的输入功率
power1_alarm            输入功率告警
power1_max              最大输入功率

power2_label            "pout1"
power2_input            测量的输出功率
power2_max              最大输出功率
power2_max_alarm        输出功率过高告警

temp1_input             输入连接器附近测得的温度
temp1_alarm             温度告警

temp2_input             输出连接器附近测得的温度
temp2_alarm             温度告警
======================= ============================================
