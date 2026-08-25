## 内核驱动 power_meter


该驱动与 ACPI 4.0 功率计通信
支持的系统：

  - 任何较新的支ACPI 4.0 的系统
    Prefix: 'power_meter'

    Datasheet: https://uefi.org/specifications, section 10.4.

Author: Darrick J. Wong

### 描述


该驱动为 ACPI 4.0 规范（第 10.4 章）中暴露的功率计实现传感器读数支持。这设备具有一组简单的特性——一个功率计（返回可配置间隔内的平均功率使用）、一可选的封顶（capping）机制，以及几个触发点。sysfs 接口符合
Documentation/hwmon/sysfs-interface.rst “Power一节所述的规范
### 特殊功能


`power[1-*]_is_battery` 旋钮指示电源是否为电池。必须同时设`power[1-*]_average_{min,max}`，触发点才会生效。当二者都设置后，一ACPI
事件将在 ACPI netlink 套接字上广播，并且一poll 通知将发送到相应`power[1-*]_average` sysfs 文件
`power[1-*]_{model_number, serial_number, oem_info}` 字段显示 ACPI 随功率计
提供的任意字符串。measures/ 目录包含指向该功率计所测量设备的符号链接
某些计算机能够在硬件中强制实施功率上限。若如此，`power[1-*]_cap` 及相关的
sysfs 文件将出现。有关启用功率上限特性的信息，请参“Module Parameters一章中 “force_on_cap选项的描述。要正确使用功率上限特性，需要向
`power[1-*]_cap` sysfs 文件设置适当的值（单位微瓦）。该值必须位`power[1-]_cap_min` 处的最小值与 `power[1-]_cap_max` 处的最大值之间（两均以微瓦为单位）
当平均功率消耗超过上限时，一ACPI 事件将在 netlink 事件套接字上广播，并一poll 通知将发送到相应`power[1-*]_alarm` 文件，表示封顶已开始，硬件
已采取行动降低功率消耗。这很可能导致性能下降
固件还可以发送其他一ACPI 通知。在所有情况下，ACPI 事件都将ACPI netlink
事件套接字上广播，并作为 poll 通知发送到某个 sysfs 文件。事件如下：

`power[1-*]_cap` 将在固件更改功率上限时收到通知`power[1-*]_interval` 将在固件更改平均间隔时收到通知
### 模块参数


- force_cap_on: bool
                        强制启用功率封顶特性，以指定系统功率消耗的上限
                        默认情况下，驱动的功率封顶特性仅IBM 产品上启用                        因此，在其他支持功率封顶的系统上，你需要使用该选项
                        来启用它
                        注意：功率封顶是潜在不安全的特性                        在使用该选项之前，请检查平台规范以确认支持封顶