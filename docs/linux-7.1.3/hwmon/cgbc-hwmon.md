
## 内核驱动 cgbc-hwmon


支持的芯片：

 - Congatec Board Controller銆。

   前缀: 'cgbc-hwmon'

Author: Thomas Richard <thomas.richard@bootlin.com>

### 描述


该驱动为 Congatec Board Controller 启用监控支持
该控制器内嵌Congatec x86 SoM 上

### Sysfs 条目


以下 sysfs 条目列表包含 Board Controller 中定义的所有传感器
sysfs 中可用的传感器取决于 SoM 
系统

============= ======================
Name          描述
============= ======================
temp1_input   CPU 温度
temp2_input   机箱温度
temp3_input   环境温度
temp4_input   板卡温度
temp5_input   载板温度
temp6_input   芯片组温
temp7_input   视频温度
temp8_input   其他温度
temp9_input    TOPDIM 温度
temp10_input  BOTTOMDIM 温度
in0_input     CPU 电压
in1_input     DC 运行电压
in2_input     DC 待机电压
in3_input     CMOS 电池电压
in4_input     电池电压
in5_input     AC 电压
in6_input     其他电压
in7_input     5V 电压
in8_input     5V 待机电压
in9_input     3V3 电压
in10_input    3V3 待机电压
in11_input    VCore A 电压
in12_input    VCore B 电压
in13_input    12V 电压
curr1_input   DC 电流
curr2_input   5V 电流
curr3_input   12V 电流
fan1_input    CPU 风扇
fan2_input    机箱风扇
fan3_input    环境风扇
fan4_input    芯片组风
fan5_input    视频风扇
fan6_input    其他风扇
============= ======================
