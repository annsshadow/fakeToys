
## 内核驱动 peci-dimmtemp


支持的设备：
	下述连接到 PECI 总线的 Intel 服务器 CPU 之一。
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
  - Intel Xeon Scalable 服务器处理器
			Intel Xeon D 系列
			Intel Xeon Bronze 系列
			Intel Xeon Silver 系列
			Intel Xeon Gold 系列
			Intel Xeon Platinum 系列

	Datasheet: Available from http://www.intel.com/design/literature.htm

Author: Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>

### 描述


本驱动实现了一个通用的 PECI hwmon 特性，提供可通过处理器 PECI 接口访问的 DIMM 温度传感读数。

所有温度值均以毫摄氏度给出，且仅在目标 CPU 上电时可测量。

### Sysfs 接口


======================= =======================================================

temp[N]_label		提供字符串 "DIMM CI"，其中 C 为 DIMM 通道，I 为已安装 DIMM 的索引。
temp[N]_input		提供已安装 DIMM 的当前温度。
temp[N]_max		提供 DIMM 的热控制温度。
temp[N]_crit		提供 DIMM 的关断温度。

======================= =======================================================

说明：
	DIMM 温度属性会在客户端 CPU 的 BIOS 完成内存训练与测试后出现。
