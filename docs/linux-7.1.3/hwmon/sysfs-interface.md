## sysfs 文件的命名与数据格式标准


lib传感器（libsensors）库通过 sysfs 接口提供对原始传感器数据的访问
lm-sensors 3.0.0 起，libsensors 完全与具体芯片无关。它假定所有内核驱动都实现了本文档中描述的标准 sysfs 接口。这使得添加或更新对任何特定芯片的支持变得非常容易，因为 libsensors 以及使用它的应用程序都无需修改。相lm-sensors 2 这是一个重大改进

注意，主板与传感器芯片的连接方式千差万别。没有任何标准能保证，例如第二个温度传感器接CPU 上，或者第二个风扇CPU 上。此外，芯片报告的某些数值在能完全理解之前还需要一些计算。例如，大多数芯片只能测0 +4V 之间的电压。其他电压通过外部电阻被缩放回该范围内。由于这些电阻的阻值可能因主板而异，转换不能被硬编码进驱动，而必须在用户空间完成

因此，即便我们的目标是芯片无关的 libsensors，它仍然需要一个配置文件（例如 /etc/sensors.conf）来正确地进行数值转换、输入标注以及隐藏未使用的输入

一些程序使用的另一种方法是直接访问 sysfs 文件。本文档简要描述了驱动所遵循的标准，以便应用程序能以简单一致的方式扫描条目并访问这些数据。话虽如此，这类程序仍须自行实现输入的转换、标注与隐藏。因此，仍然不推荐绕过该库

每个芯片sysfs /sys/devices 树中拥有自己的目录。要查找所有传感器芯片，更简单的方式是顺着来自 `/sys/class/hwmon/hwmon*` 的设备软链接

lm-sensors 3.0.0 之前，libsensors 物理"设备目录中查找硬件监控属性。自 lm-sensors 3.0.1 起，hwmon "class" 设备目录中找到的属性也得到了支持。复杂的驱动（例如多功能芯片的驱动）可能希望利用这种可能性以避免命名空间污染。唯一的缺点是较旧版本libsensors 将无法支持该驱动

所sysfs 值都是定点数

与旧/proc 规范不同，每个文件只有一个值。文件命名的通用方案是：<type><number>_<item>。传感器芯片常用type "in"（电压）temp"（温度）"fan"（风扇）。常用的 item "input"（测量值）max"（高阈值）min"（低阈值）。编号通常1 开始，电压除外（从 0 开始，因为大多数数据手册都这样用）。对于任何可能出现多次的元素都总是使用数字编号，即便特定芯片上只有单个该类型的元素。其他文件不指向特定元素，因此使用简单名称，不带数字

告警（alarm）是从芯片读取的直接指示。驱动不会对读数与阈值做比较。这使得读数之间的违规能被捕获并告警。告警的确切定义（例如，是达到阈值还是必须超过阈值才会引发告警）取决于芯片

当设hwmon sysfs 属性的值时，必须写入所需值的字符串表示，注意非数字的字符串会被解释为 0！关于写入字符串如何被解释，详见本文档末尾的 "sysfs attribute writes interpretation" 小节

### 属性访问（Attribute access


硬件监控 sysfs 属性可被不受限制的用户空间应用程序读取。因此，所有标ABI 属性都应对所有人可读。可写的标准 ABI 属性应仅对特权用户可写

-------------------------------------------------------------------------

======= ===========================================
`[0-*]`	表示任意0 开始的正整
`[1-*]`	表示任意1 开始的正整
RO	只读
WO	只写
RW	读写
======= ===========================================

读写值对某些芯片可能是只读的，取决于硬件实现

所有条目（name 除外）都是可选的，只有在芯片具备该特性时才应由相应驱动创建

关于属性的完整描述，见 Documentation/ABI/testing/sysfs-class-hwmon

######## 全局属性（Global attributes


`name`
		芯片名称

`label`
		一个描述性标签，用于在系统内唯一标识一个设备

`update_interval`
		芯片更新读数的间隔


######## 电压（Voltages


`in[0-*]_min`
		电压最小值

`in[0-*]_lcrit`
		电压临界最小值

`in[0-*]_max`
		电压最大值

`in[0-*]_crit`
		电压临界最大值

`in[0-*]_input`
		电压输入值

`in[0-*]_average`
		平均电压

`in[0-*]_lowest`
		历史最低电

`in[0-*]_highest`
		历史最高电

`in[0-*]_reset_history`
		閲嶇疆 inX_lowest 鍜?inX_highest

`in_reset_history`
		为所有传感器重置 inX_lowest inX_highest

`in[0-*]_label`
		建议的电压通道标签

`in[0-*]_enable`
		启用或禁用传感器

`cpu[0-*]_vid`
		CPU 核心参考电压

`vrm`
		电压调节模块（VRM）版本号

`in[0-*]_rated_min`
		额定最小电压

`in[0-*]_rated_max`
		额定最大电压

另见告警（Alarms）小节中与电压相关的状态标志


######## 风扇（Fans


`fan[1-*]_min`
		风扇最小

`fan[1-*]_max`
		风扇最大

`fan[1-*]_input`
		风扇输入值

`fan[1-*]_div`
		风扇分频系数

`fan[1-*]_pulses`
		每转风扇的转速计脉冲数

`fan[1-*]_target`
		期望的风扇转

`fan[1-*]_label`
		建议的风扇通道标签

`fan[1-*]_enable`
		启用或禁用传感器

另见告警（Alarms）小节中与风扇相关的状态标志


######## PWM


`pwm[1-*]`
		脉宽调制风扇控制

`pwm[1-*]_enable`
		风扇转速控制方法

`pwm[1-*]_mode`
		直流或脉宽调制

`pwm[1-*]_freq`
		PWM 基准频率，单Hz

`pwm[1-*]_auto_channels_temp`
		选择自动模式下哪些温度通道影响PWM 输出

`pwm[1-**]_auto_point[1-**]_pwm` / `pwm[1-**]_auto_point[1-**]_temp` / `pwm[1-**]_auto_point[1-**]_temp_hyst`
		定义 PWM 与温度的曲线

`temp[1-**]_auto_point[1-**]_pwm` / `temp[1-**]_auto_point[1-**]_temp` / `temp[1-**]_auto_point[1-**]_temp_hyst`
		定义 PWM 与温度的曲线

还有第三种情况，即跳变点（trip point）同时关PWM 输出通道和温度通道：PWM 值关联到 PWM 输出通道，而温度值关联到温度通道。在这种情况下，结果由温度输入与 PWM 输出之间的映射决定。当多个温度输入映射到一个给定的 PWM 输出时，会产生多个候PWM 值。实际结果取决于芯片，但通常最高的候选值（最快的风扇转速）胜出


######## 温度（Temperatures


`temp[1-*]_type`
		传感器类型选择

`temp[1-*]_max`
		温度最大值

`temp[1-*]_min`
		温度最小值

`temp[1-*]_max_hyst`
		最大限值的温度迟滞值

`temp[1-*]_min_hyst`
		最小限值的温度迟滞值

`temp[1-*]_input`
		温度输入值

`temp[1-*]_crit`
		温度临界最大值，通常大于对应temp_max 值

`temp[1-*]_crit_hyst`
		临界限值的温度迟滞值

`temp[1-*]_emergency`
		温度紧急最大值，用于支持超过两个上限温度限制的芯片

`temp[1-*]_emergency_hyst`
		紧急限值的温度迟滞值

`temp[1-*]_lcrit`
		温度临界最小值，通常低于对应temp_min 值

`temp[1-*]_lcrit_hyst`
		临界最小限值的温度迟滞值

`temp[1-*]_offset`
		由芯片加到温度读数上的温度偏移

`temp[1-*]_label`
		建议的温度通道标签

`temp[1-*]_lowest`
		历史最低温

`temp[1-*]_highest`
		历史最高温

`temp[1-*]_reset_history`
		閲嶇疆 temp_lowest 鍜?temp_highest

`temp_reset_history`
		为所有传感器重置 temp_lowest temp_highest

`temp[1-*]_enable`
		启用或禁用传感器

`temp[1-*]_rated_min`
		额定最低温度

`temp[1-*]_rated_max`
		额定最高温度

某些芯片使用外部热敏电阻ADC 测量温度，并把温度测量值作为电压报告。把此电压转换回温度（或者反过来，对限值而言）需要内核中不可用的数学函数，因此转换必须在用户空间发生。对于这些芯片，上述所temp* 文件应包含以毫伏而非毫摄氏度表示的值。换句话说，此类温度通道由驱动当作电压通道处理

另见告警（Alarms）小节中与温度相关的状态标志


######## 电流（Currents


`curr[1-*]_max`
		电流最大值

`curr[1-*]_min`
		电流最小值

`curr[1-*]_lcrit`
		电流临界低

`curr[1-*]_crit`
		电流临界高值

`curr[1-*]_input`
		电流输入值

`curr[1-*]_average`
		平均电流使用

`curr[1-*]_lowest`
		历史最小电流

`curr[1-*]_highest`
		历史最大电流

`curr[1-*]_reset_history`
		閲嶇疆 currX_lowest 鍜?currX_highest

		WO

`curr_reset_history`
		为所有传感器重置 currX_lowest currX_highest

`curr[1-*]_enable`
		启用或禁用传感器

`curr[1-*]_rated_min`
		额定最小电流

`curr[1-*]_rated_max`
		额定最大电流

另见告警（Alarms）小节中与电流相关的状态标志

######## 功率（Power


`power[1-*]_average`
		平均功率使用

`power[1-*]_average_interval`
		功率使用平均间隔

`power[1-*]_average_interval_max`
		功率使用平均间隔最大值

`power[1-*]_average_interval_min`
		功率使用平均间隔最小值

`power[1-*]_average_highest`
		历史平均最大功率使

`power[1-*]_average_lowest`
		历史平均最小功率使

`power[1-*]_average_max`
		当功率使用超过此值时，向 `power[1-*]_average` 发送轮询通知

`power[1-*]_average_min`
		当功率使用低于此值时，向 `power[1-*]_average` 发送轮询通知

`power[1-*]_input`
		瞬时功率使用

`power[1-*]_input_highest`
		历史最大功率使

`power[1-*]_input_lowest`
		历史最小功率使用

`power[1-*]_reset_history`
		重置 input_highest、input_lowest、average_highest average_lowest

`power[1-*]_accuracy`
		功率计精度

`power[1-*]_cap`
		如果功率使用超过此限制，系统应采取措施降低功率使用

`power[1-*]_cap_hyst`
		在限值与通知周围建立的迟滞余量

`power[1-*]_cap_max`
		可设置的最大上限

`power[1-*]_cap_min`
		可设置的最小上限

`power[1-*]_max`
		最大功率

`power[1-*]_crit`
				临界最大功率

				如果功率上升到等于或超过此限制，系统应采取激烈措施来降低功耗，例如系统关机或强制关闭某些设备

				单位：微瓦（microWatt

				RW

`power[1-*]_enable`
				启用或禁用传感器

				当禁用时，传感器读取将返-ENODATA

    - 1: 启用
    - 0: 禁用

				RW

`power[1-*]_rated_min`
				额定最小功率

				单位：微瓦（microWatt

				RO

`power[1-*]_rated_max`
				额定最大功率

				单位：微瓦（microWatt

				RO

另见告警（Alarms）小节中与功率读数相关的状态标志

######## 能量（Energy


`energy[1-*]_input`
				累计能量使用

				单位：微焦耳（microJoule

				RO

`energy[1-*]_enable`
				启用或禁用传感器

				当禁用时，传感器读取将返-ENODATA

    - 1: 启用
    - 0: 禁用

				RW

######## 湿度（Humidity


`humidity[1-*]_input`
		湿度

`humidity[1-*]_enable`
		启用或禁用传感器

`humidity[1-*]_rated_min`
		额定最小湿度

`humidity[1-*]_rated_max`
		额定最大湿度

######## 告警（Alarms


每个通道或限值都可能有一个关联的告警文件，包含一个布尔值 表示存在告警条件 表示无告警

通常一个给定芯片要么使用通道相关的告警，要么使用限值相关的告警，不会两者都用。驱动应仅反映硬件实现

+-------------------------------+-----------------------+
| **`in[0-*]_alarm`,		| 通道告警		|
| `curr[1-*]_alarm`,		|			|
| `power[1-*]_alarm`,		|   - 0: 鏃犲憡璀?	|
| `fan[1-*]_alarm`,		|   - 1: 告警		|
| `temp[1-*]_alarm`**		|			|
|				|   RO			|
+-------------------------------+-----------------------+

**或（OR*

+-------------------------------+-----------------------+
| **`in[0-*]_min_alarm`,	| 限制告警		|
| `in[0-*]_max_alarm`,		|			|
| `in[0-*]_lcrit_alarm`,	|   - 0: 鏃犲憡璀?	|
| `in[0-*]_crit_alarm`,		|   - 1: 告警		|
| `curr[1-*]_min_alarm`,	|			|
| `curr[1-*]_max_alarm`,	| RO			|
| `curr[1-*]_lcrit_alarm`,	|			|
| `curr[1-*]_crit_alarm`,	|			|
| `power[1-*]_cap_alarm`,	|			|
| `power[1-*]_max_alarm`,	|			|
| `power[1-*]_crit_alarm`,	|			|
| `fan[1-*]_min_alarm`,		|			|
| `fan[1-*]_max_alarm`,		|			|
| `temp[1-*]_min_alarm`,	|			|
| `temp[1-*]_max_alarm`,	|			|
| `temp[1-*]_lcrit_alarm`,	|			|
| `temp[1-*]_crit_alarm`,	|			|
| `temp[1-*]_emergency_alarm`**	|			|
+-------------------------------+-----------------------+

每个输入通道可能有一个关联的故障（fault）文件。这可用于在硬件支持时通知开路二极管、未连接的风扇等。当此布尔值为 1 时，不应信任该通道的测量值

`fan[1-**]_fault` / `temp[1-**]_fault`
		输入故障条件

某些芯片还提供在告警发生时发出蜂鸣声（beep）的能力

`beep_enable`
		主蜂鸣使能

`in[0-**]_beep`, `curr[1-**]_beep`, `fan[1-**]_beep`, `temp[1-**]_beep`,
		通道蜂鸣

理论上，一个芯片可以提供逐限值的蜂鸣屏蔽，但迄今未见这样的芯片

旧驱动提供了一套不同的、非标准的告警与蜂鸣接口。这些接口文件已被弃用，但出于兼容性原因会被保留：

`alarms`
		告警位掩码

`beep_mask`
		蜂鸣位掩码


######## 入侵检测（Intrusion detection


`intrusion[0-*]_alarm`
		机箱入侵检测

`intrusion[0-*]_beep`
		机箱入侵蜂鸣

######## 平均采样配置（Average sample configuration


允许读取 {in,power,curr,temp}_average 值的设备可以导出用于控制计算平均值所用样本数的属性

+--------------+---------------------------------------------------------------+
| samples      | 为所有类型的测量设置平均样本数                             |
|	       |							       |
|	       | RW							       |
+--------------+---------------------------------------------------------------+
| in_samples   | 为特定类型的测量设置平均样本数                            |
| power_samples|							       |
| curr_samples |							       |
| temp_samples | 注意在某些设备上无法把它们都设为不同的值，因此更改其中一   |
|	       | 也可能改变另外一些                                         |
|	       |							       |
|	       | RW							       |
+--------------+---------------------------------------------------------------+

### sysfs 属性写入的解释（sysfs attribute writes interpretation


hwmon sysfs 属性总是包含数字，因此要做的第一件事是把输入转换为数字，根据是否
```

	unsigned long u = simple_strtoul(buf, NULL, 10);
	long s = simple_strtol(buf, NULL, 10);

```
buf 是内核传入的带有用户输入的缓冲区。注意我们没有使strto[u]l 的第二个参数，因此当返回 0 时，无法判断这真的是 0 还是由无效输入引起的。这是有意这样做的，因为在各处检查这一点会给内核增加大量代码

注意始终要把转换后的值存储为 unsigned long long，以便在任何进一步检查之前都不会发生回绕

把输入字符串转换为（无符号）长整型后，应检查该值是否可接受。在检查其有效性之前，对值做进一步的转换要小心，因为这些转换仍可能在检查之前造成回绕。例如不要对结果做乘法，只有在加减之前先除过，才进行加减

如果发现一个值无效，该如何处理取决于所设置sysfs 属性的类型。如果它是一个连续的设置，例tempX_max inX_max 属性，那么应使clamp_val(value, min_limit, max_limit) 把该值钳制到其限制范围内。如果它不是连续的，例如 tempX_type，那么当写入一个无效值时，应返回 -EINVAL

```

	long v = simple_strtol(buf, NULL, 10) / 1000;
	v = clamp_val(v, -128, 127);
	/* 灏?v 鍐欏叆瀵勫瓨鍣?*/

```
```

	unsigned long v = simple_strtoul(buf, NULL, 10);

	switch (v) {
	case 2: v = 1; break;
	case 4: v = 2; break;
	case 8: v = 3; break;
	default:
		return -EINVAL;
	}
	/* 灏?v 鍐欏叆瀵勫瓨鍣?*/

```
