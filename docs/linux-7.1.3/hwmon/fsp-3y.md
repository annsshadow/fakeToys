## 内核驱动 fsp3y


支持设备：
  - 3Y POWER YH-5151E
  - 3Y POWER YM-2151E

Author: Václav Kubernát <kubernat@cesnet.cz>

### 说明


该驱动对两款 3Y POWER 设备提供有限支持。

### Sysfs 条目


  - in1_input            输入电压
  - in2_input            12V 输出电压
  - in3_input            5V 输出电压
  - curr1_input          输入电流
  - curr2_input          12V 输出电流
  - curr3_input          5V 输出电流
  - fan1_input           风扇转速（rpm）
  - temp1_input          温度 1
  - temp2_input          温度 2
  - temp3_input          温度 3
  - power1_input         输入功率
  - power2_input         输出功率
