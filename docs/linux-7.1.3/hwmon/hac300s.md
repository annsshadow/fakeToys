## 内核驱动 hac300s


支持的芯片：

   - HiTRON HAC300S

     Prefix: 'hac300s'

     Datasheet: 可在 HiTRON 官网公开获取。

Author:

  - Vasileios Amoiridis <vasileios.amoiridis@cern.ch>

### 说明


该驱动支持 HiTRON HAC300S 电源（PSU）。它是一款通用交流输入、谐波校正、交流-直流热插拔、CompactPCI Serial 双路输出（带 5V 待机）、312 瓦主动均流开关电源。

该设备输入为 90-264VAC，具有标称 12V 和 5V 两路输出电压，分别可提供高达 25A 和 2.5A 的电流。

### Sysfs 条目


======= ==========================================
curr1   输出电流
in1     输出电压
power1  输出功率
temp1   模块内部环境温度
temp2   内部次级元件温度
======= ==========================================
