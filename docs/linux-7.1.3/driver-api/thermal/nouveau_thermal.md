## 内核驱动 nouveau


Supported chips:

- NV43+

Authors: Martin Peres (mupuf) <martin.peres@free.fr>

### 描述


本驱动允许读GPU 核心温度、驱GPU 风扇并设置温度报警
目前，由于内核中缺少访问 HWMON 驱动API，Nouveau 无法访问它可能发现的任何 i2c 外部监控芯片。如果你拥有此类芯片，那么通过 Nouveau HWMON 接口进行温度或风扇管理很可能无法工作。本文档可能因此无法完全覆盖你的情况
### 温度管理


温度以一个只读的 HWMON 属temp1_input 暴露
为保GPU 不过热，Nouveau 支持 4 个可配置的温度阈值：

 - Fan_boost（风扇加速）	达到该温度时风扇转速设100% - Downclock（降频）	GPU 将被降频以减少功耗；
 - Critical（临界）	GPU 被暂停以进一步降低功耗；
 - Shutdown（关机）	关闭计算机以保护你的 GPU
WARNING（警告）	根据芯片组不同，Nouveau 可能不会使用其中某些阈值
这些阈值的默认值来GPU vbios。这些阈值可通过以下 HWMON 属性配置：

 - Fan_boost：temp1_auto_point1_temp temp1_auto_point1_temp_hyst - Downclock：temp1_max temp1_max_hyst - Critical：temp1_crit temp1_crit_hyst - Shutdown：temp1_emergency temp1_emergency_hyst
NOTE（注意）：请记住，这些值以毫摄氏度（milli degrees Celsius）存储。别忘了换算
### 风扇管理


并非所有显卡都有可驱动的风扇。如果有，则以下 HWMON 属性应当可用：

 - pwm1_enable	当前风扇管理模式（NONE、MANUAL AUTO）；
 - pwm1	当前 PWM 值（功率百分比） - pwm1_min	允许的最PWM 转速；
 - pwm1_max	允许的最PWM 转速（命中 Fan_boost 时会被绕过）
你可能还拥有以下属性：

 - fan1_input	风扇转速（RPM）
你的风扇可以在不同模式下驱动
 - 0：风扇保持不动；
 - 1：风扇可手动驱动（使pwm1 改变转速） - 2：风扇根据温度自动驱动
NOTE（注意）  若想手动驱动风扇转速，请务必使用手动模式
NOTE2（注）：
  当在 vbios 定义[PWM_min, PWM_max] 范围之外以手动模式运行时，根据硬件不同，报告的风扇转速（RPM）可能不准确
### 缺陷报告


Nouveau 上的热管理属于新功能，可能并非在所有显卡上都能工作。如有疑问，请在 IRCnouveau，OFTC）上联系 mupuf
缺陷报告应提交到 Freedesktop bug 跟踪器。请访问
https://nouveau.freedesktop.org/wiki/Bugs
