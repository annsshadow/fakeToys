## 内核驱动 peci-cputemp


支持的芯片：
	下方所列的某款连接PECI 总线Intel 服务CPU  - Intel Xeon E5/E7 v3 服务器处理器
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
  - Intel Xeon 可扩展服务器处理			Intel Xeon D 系列
			Intel Xeon Bronze 系列
			Intel Xeon Silver 系列
			Intel Xeon Gold 系列
			Intel Xeon Platinum 系列

	Datasheet: Available from http://www.intel.com/design/literature.htm

Author: Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>

### 描述


该驱动实现了通用PECI hwmon 功能，提供可通过处理PECI 接口访问CPU
封装CPU 核心的数字热传感器（DTS）温度读数
所有温度值均以毫摄氏度给出，并且仅当目标 CPU 上电时才可测量
### Sysfs 接口


======================= =======================================================
temp1_label		"Die"
temp1_input		提供 CPU 封装的当前芯片温度temp1_max		提供 CPU 封装的热控制温度，也称为 Tcontroltemp1_crit		提供 CPU 封装的关机温度，也称为处理器最大结			Tjmax Tprochottemp1_crit_hyst		提供 CPU 封装的迟滞温度。返Tcontrol，即临界
			条件解除时的温度temp2_label		"DTS"
temp2_input		提供 CPU 封装的当前温度，已缩放以匹配 DTS 热曲线temp2_max		提供 CPU 封装的热控制温度，也称为 Tcontroltemp2_crit		提供 CPU 封装的关机温度，也称为处理器最大结			Tjmax Tprochottemp2_crit_hyst		提供 CPU 封装的迟滞温度。返Tcontrol，即临界
			条件解除时的温度temp3_label		"Tcontrol"
temp3_input		提供 CPU 封装的当Tcontrol 温度，也称为风扇温度目标			表示热监视器触发温度的相对值，达到该温度时应启动风扇temp3_crit		提供 CPU 封装Tcontrol 临界值，Tjmax 相同temp4_label		"Tthrottle"
temp4_input		提供 CPU 封装的当Tthrottle 温度。用于节流温度。若该			被允许且低于 Tjmax，则会发生节流，并在低于 Tjmax 时报告temp5_label		"Tjmax"
temp5_input		提供 CPU 封装的最大结Tjmaxtemp[6-N]_label		提供字符“Core X”，其中 X 为解析出的核心编号temp[6-N]_input		提供每个核心的当前温度======================= =======================================================
