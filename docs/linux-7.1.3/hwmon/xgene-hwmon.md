## 内核驱动 xgene-hwmon


支持芯片：

 - APM X-Gene SoC

### 说明


该驱动通过邮箱（mailbox）通信接口为 APM X-Gene SoC 添加硬件温度和功率读取支持。
对于设备树，使用的是标准 DT 邮箱。
对于 ACPI，使用的是 PCC 邮箱。

支持以下传感器：

  - 温度
      - SoC 片上温度，单位为毫摄氏度（milli-degree C）
      - 当发生高温/过热时产生告警

  - 功率
      - CPU 功率，单位为微瓦（uW）
      - IO 功率，单位为微瓦（uW）

### sysfs 接口


temp0_input
 - SoC 片上温度（毫摄氏度）
temp0_critical_alarm
 - 值为 1 表示片上温度超过阈值
power0_input
 - CPU 功率（uW）
power1_input
 - IO 功率（uW）
