## 内核驱动 peci-cputemp


支持的芯片：
	下方所列的某款连接到 PECI 总线的 Intel 服务器 CPU。
  - Intel Xeon E5/E7 v3 服务器处理器
			Intel Xeon E5-14xx v3 系列
			Intel Xeon E5-24xx v3 系列
			Intel Xeon E5-16xx v3 系列
			Intel Xeon E5-26xx v3 系列
			Intel Xeon E5-46xx v3 系列
			Intel Xeon E7-48xx v3 系列
			Intel Xeon E7-88xx v3 系列
  - Intel Xeon E5/E7 v4 服务器处理器
			Intel Xeon E5-16xx v4 系列
			Intel Xeon E5-26xx v4 系列
			Intel Xeon E5-46xx v4 系列
			Intel Xeon E7-48xx v4 系列
			Intel Xeon E7-88xx v4 系列
  - Intel Xeon 可扩展服务器处理器
			Intel Xeon D 系列
			Intel Xeon Bronze 系列
			Intel Xeon Silver 系列
			Intel Xeon Gold 系列
			Intel Xeon Platinum 系列

	Datasheet: Available from http://www.intel.com/design/literature.htm

Author: Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>

### 描述


该驱动实现了通用的 PECI hwmon 功能，提供可通过处理器 PECI 接口访问的 CPU
封装与 CPU 核心的数字热传感器（DTS）温度读数。

所有温度值均以毫摄氏度给出，并且仅当目标 CPU 上电时才可测量。

### Sysfs 接口


======================= =======================================================
temp1_label		"Die"
temp1_input		提供 CPU 封装的当前芯片温度。
temp1_max		提供 CPU 封装的热控制温度，也称为 Tcontrol。
temp1_crit		提供 CPU 封装的关机温度，也称为处理器最大结温
			Tjmax 或 Tprochot。
temp1_crit_hyst		提供 CPU 封装的迟滞温度。返回 Tcontrol，即临界
			条件解除时的温度。
temp2_label		"DTS"
temp2_input		提供 CPU 封装的当前温度，已缩放以匹配 DTS 热曲线。
temp2_max		提供 CPU 封装的热控制温度，也称为 Tcontrol。
temp2_crit		提供 CPU 封装的关机温度，也称为处理器最大结温
			Tjmax 或 Tprochot。
temp2_crit_hyst		提供 CPU 封装的迟滞温度。返回 Tcontrol，即临界
			条件解除时的温度。
temp3_label		"Tcontrol"
temp3_input		提供 CPU 封装的当前 Tcontrol 温度，也称为风扇温度目标。
			表示热监视器触发温度的相对值，达到该温度时应启动风扇。
temp3_crit		提供 CPU 封装的 Tcontrol 临界值，与 Tjmax 相同。
temp4_label		"Tthrottle"
temp4_input		提供 CPU 封装的当前 Tthrottle 温度。用于节流温度。若该值
			被允许且低于 Tjmax，则会发生节流，并在低于 Tjmax 时报告。
temp5_label		"Tjmax"
temp5_input		提供 CPU 封装的最大结温 Tjmax。
temp[6-N]_label		提供字符串 “Core X”，其中 X 为解析出的核心编号。
temp[6-N]_input		提供每个核心的当前温度。
======================= =======================================================
