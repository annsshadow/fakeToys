
## Linux HP WMI 浼犳劅鍣ㄩ┍鍔。

:Copyright: |copy| 2023 James Seo <james@equiv.tech>

## 描述


惠普（以及部HP Compaq）商务级计算机通过 Windows Management Instrumentation（WMI）报告硬件监控信息。该驱动将这些信息暴露给 Linux hwmon 子系统，允许`sensors` 这样的用户空间工具收集数值传感器读数
## sysfs 接口


当驱动加载时，它会发现系统上可用的传感器，并在必要时`/sys/class/hwmon/hwmon[X]` 中创建以sysfs 属性：

（`[X]` 是取决于其他系统组件的某个数字。）

======================= ======= ===================================
Name                    Perm    描述
======================= ======= ===================================
`curr[X]_input`       RO      电流，单位毫安（mA）`curr[X]_label`       RO      电流传感器标签`fan[X]_input`        RO      风扇转速，单位 RPM`fan[X]_label`        RO      风扇传感器标签`fan[X]_fault`        RO      风扇传感器故障指示器`fan[X]_alarm`        RO      风扇传感器报警指示器`in[X]_input`         RO      电压，单位毫伏（mV）`in[X]_label`         RO      电压传感器标签`temp[X]_input`       RO      温度，单位毫摄氏                               （m\ |deg|\ C）`temp[X]_label`       RO      温度传感器标签`temp[X]_fault`       RO      温度传感器故障指示器`temp[X]_alarm`       RO      温度传感器报警指示器`intrusion[X]_alarm`  RW      机箱入侵报警指示器======================= ======= ===================================

`fault` 属  读取`fault` 属性的值为 `1` 而非 `0`，表示该传感器在运行过程中遇到了某些问题，因此其测量值不应被信任。如果处于故障状态的传感器后来恢复，再次读取该属性将重新返回 `0`
`alarm` 属  读取`alarm` 属性的值为 `1` 而非 `0`，表示根据其类型，发生了以下之一
  - `fan`：风扇在运转时已停转或断开连接  - `temp`：传感器读数已达到临界阈值。具体的阈值取决于系统  - `intrusion`：系统机箱被打开
  读取`alarm` 属性的 `1` 后，该属性会自行复位，并在后续读取时返回 `0`。作为例外，`intrusion[X]_alarm` 只能通过向它写入 `0` 来手动复位
## debugfs 接口


             并且仅在内核编译时定义了 `CONFIG_DEBUG_FS` 时才可用
sysfs 中的标准 hwmon 接口暴露了在驱动初始化时连接的几种常见类型的传感器。然而，WMI 中通常还有其他不符合这些条件的传感器。此外，可能还存在一些系统相关的、用`alarm` 属性的“平台事件对象（platform events objects）”。因此提供了一debugfs 接口，用于只读访问所有可用的 HP WMI 传感器和平台事件对象
`/sys/kernel/debug/hp-wmi-sensors-[X]/sensor`
为每个传感器包含一个带编号的条目，具有以下属性：

=============================== =======================================
Name                            Example
=============================== =======================================
`name`                        `CPU0 Fan`
`description`                 `Reports CPU0 fan speed`
`sensor_type`                 `12`
`other_sensor_type`           （空字符串）
`operational_status`          `2`
`possible_states`             `Normal,Caution,Critical,Not Present`
`current_state`               `Normal`
`base_units`                  `19`
`unit_modifier`               `0`
`current_reading`             `1008`
`rate_units`                  `0`（仅存在于某些系统上=============================== =======================================

如果平台事件对象可用`/sys/kernel/debug/hp-wmi-sensors-[X]/platform_events`
为每个对象包含一个带编号的条目，具有以下属性：

=============================== ====================
Name                            Example
=============================== ====================
`name`                        `CPU0 Fan Stall`
`description`                 `CPU0 Fan Speed`
`source_namespace`            `root\wmi`
`source_class`                `HPBIOS_BIOSEvent`
`category`                    `3`
`possible_severity`           `25`
`possible_status`             `5`
=============================== ====================

这些代表了底`HPBIOS_BIOSNumericSensor` `HPBIOS_PlatformEvents` WMI 对象的属性，它们在不同系统之间有所差异更多细节和托管对象格式（MOF）定义请参见 [#]_
## 已知问题与限

- 如果针对非商务级 HP 系统的现hp-wmi 驱动已经加载，那么即使在不支持这些属性的系统上，`alarm` 属性也将不可用。这是因为该驱动用于 `alarm` 属性的同一WMI 事件 GUID 在这些系统上被用于例如笔记本热键- 已观察到可疑的传感器硬件和不一致的 BIOS WMI 实现会导致不准确的读数和异常行为，例如报警不发生或每次启动只发生一次- 迄今为止在现实中只见过温度、风扇转速和入侵这几种传感器类型。因此对电压和电流传感器的支持是暂定的- 尽管 HP WMI 传感器可能声称是任何类型，但 hwmon 不认识的任何奇怪传感器类型将不受支持
## 参考资

       “HP Client Management Interface Technical White Paper”，2005[Online].
       Available: https://h20331.www2.hp.com/hpsub/downloads/cmi_whitepaper.pdf
