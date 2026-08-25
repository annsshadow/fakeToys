## ACPI 风扇性能状

当代表风扇的 ACPI 设备（例PNP0C0B INT3404）下存在可选的 _FPS 对象时，
ACPI 风扇驱动会在ACPI 设备sysfs 目录中创建额外的 “state*属性。这属性列出了风扇性能状态的属性
有关 _FPS 的更多信息，请参ACPI 规范
http://uefi.org/specifications

例如，INT3404 ACPI 设备 sysfs 目录的内
```
 $ ls -l /sys/bus/acpi/devices/INT3404:00/
 total 0
 ...
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state0
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state1
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state10
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state11
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state2
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state3
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state4
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state5
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state6
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state7
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state8
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state9
 -r--r--r-- 1 root root 4096 Dec 13 01:00 status
 ...

```
其中每个 “state*文件代表风扇的一个性能状态，并包含一个以冒号分隔5 整数（字段）的列
```
  control_percent:trip_point_index:speed_rpm:noise_level_mdb:power_mw

```
- `control_percent`：用于通过 _FSL 对象-100）将风扇速度设置为特定级别的
  百分比值
- `trip_point_index`：与此性能状态对应的主动冷却触发点编号（0-9）
- `speed_rpm`：风扇转速，单位为每分钟转数
- `noise_level_mdb`：此状态下风扇发出的可听噪声，单位为毫分贝（millidecibel）
- `power_mw`：此状态下风扇的功率消耗，单位为毫瓦
```
 $cat /sys/bus/acpi/devices/INT3404:00/state1
 25:0:3200:12500:1250

```
当给定字段未被填充，或其由平台固件提供的值无效时，会显示 “not-defined字符而非该值
## ACPI 风扇细粒度控

_FIF 对象指定支持细粒度控制时，可以通过 _FSL 对象将风扇速度0 设置100%（带有推荐的最小“步长”）。用户可以使用热 sysfs 冷却设备调整风扇速度
这里用户可参考风扇性能状态中的参考速度（speed_rpm），并通过更改冷却设备cur_state 来设置它。如果支持细粒度控制，用户还可以调整到性能状态中未定义的
其他速度
细粒度控制的支持通过 sysfs 属“fine_grain_control呈现。如果存在细粒度
控制，该属性显”，否则显示 ”
sysfs 属性与性能状态位于同一目录中
## ACPI 风扇性能反馈


可选的 _FST 对象提供风扇设备的状态信息。这包括一个字段，用于提供风扇当前
旋转转速（每分钟转数）
该速度通过属“fan_speed_rpmsysfs 中呈现，与性能状态位于同一目录