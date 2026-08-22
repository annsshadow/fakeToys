## 低功耗空闲表（Low Power Idle Table, LPIT

为了枚举平台的低功耗空闲状态，Intel 平台使用了“低功耗空闲表”（Low Power Idle Table，LPIT）。有关该表的更多细节可从以下地址下载https://www.uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf

每个低功耗状态的驻留时间（Residency）可以通过 FFH（Function fixed hardware，功能固定硬件）或内存映射（memory mapped）接口读取
在支S0ix 睡眠状态的平台上，可能存在两种类型的驻留时间：

  - CPU PKG C10（通过 FFH 接口读取  - 平台控制器中枢（Platform Controller Hub，PCH）SLP_S0（通过内存映射接口读取
以下属性会被动态地添加cpuidle 中：
```
  /sys/devices/system/cpu/cpuidle/low_power_idle_cpu_residency_us
  /sys/devices/system/cpu/cpuidle/low_power_idle_system_residency_us
```

"low_power_idle_cpu_residency_us" 属性显CPU 封装（package）处PKG C10 的时间
"low_power_idle_system_residency_us" 属性显SLP_S0 的驻留时间，SLP_S0# 信号被置位期间系统所花费的时间。这是可能的最低系统功耗状态，仅当 CPU 处于 PKG C10 PCH 中的所有功能模块都处于低功耗状态时才可实现