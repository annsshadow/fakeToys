## 内核驱动：x86_pkg_temp_thermal


支持的芯片：

- x86：具有封装级热管理

（使用以下方式验证：CPUID.06H:EAX[bit 6] =1）

Authors: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 参考


Intel® 64 和 IA-32 架构软件开发手册（2013 年 1 月）：
第 14.6 章：封装级热管理（PACKAGE LEVEL THERMAL MANAGEMENT）

### 描述


该驱动将 CPU 数字温度封装级传感器注册为一个热区，最多可配置两个用户模式触发点。触发点的数量取决于封装的能力。一旦触发点被违反，用户模式可以通过热通知机制接收通知，并可以采取任何措施来控制温度。


### 阈值管理


每个封装将作为 /sys/class/thermal 下的一个热区注册。

```
	/sys/class/thermal/thermal_zone1
```
这包含两个触发点：

- trip_point_0_temp
- trip_point_1_temp

用户可以设置 0 到 TJ-Max 温度之间的任意温度。温度单位为毫摄氏度。有关热 sys-fs 的细节，请参阅 "Documentation/driver-api/thermal/sysfs-api.rst"。

这些触发点中除 0 以外的任何值都可以触发热通知。设置为 0 会停止发送热通知。

热通知：
要获取 kobject-uevent 通知，请将热区的策略设置为 "user_space"。

```
	echo -n "user_space" > policy
```
