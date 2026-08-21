
## Uniwill 笔记本附加功

Uniwill 制造的笔记本（无论是直接制造还是作ODM）上，`uniwill-laptop` 驱动
处理各种平台相关的特性
### 模块加载


`uniwill-laptop` 驱动依赖一DMI 表，以在受支持的设备上自动加载。当使用 `force`
模块参数时，会省略该 DMI 检查，从而允许出于测试目的在不受支持的设备上加载该驱动
### 热键（Hotkeys

通常 FN 键在没有特殊驱动的情况下就能工作。然而一旦加载了 `uniwill-laptop` 驱动FN 键就需要手动处理。这由驱动自身自动完成
### 键盘设置


`uniwill-laptop` 驱动允许用户启用/禁用
 - 集成键盘FN lock super  - 集成触摸板的触摸板开关功
详见 Documentation/ABI/testing/sysfs-driver-uniwill-laptop
### Hwmon 接口


`uniwill-laptop` 驱动支持读取 CPU GPU 温度，并支持最多两个风扇。用户态应用可通过 hwmon sysfs 接口访问传感器读数
### 平台配置档（Platform profile

目前尚未实现更改平台性能模式的支持
### 电池充电控制


             在部分设备上，驱动启用对该接口的访问可能会损坏电[^1^]_。因此，即使
             使用 `force` 模块参数，驱动也不会启用该功能
`uniwill-laptop` 驱动支持控制电池充电上限。这是通过标准`charge_control_end_threshold` 电源 sysfs 属性进行的。支1 100 之间的所百分比值
此外，驱动通过标准`health` 电源 sysfs 属性来提示存在电池充电问题
它还允许你设USB-C 电源应优先为电池充电还是CPU 提供即时供电。详Documentation/ABI/testing/sysfs-driver-uniwill-laptop
### 光条（Lightbar

`uniwill-laptop` 驱动将部分型号上带有的光条作为标准的多色 LED 类设备暴露出来LED 类设备的默认名称`uniwill:multicolor:status`
有关如何控制光条各种动画模式的细节，请参Documentation/ABI/testing/sysfs-driver-uniwill-laptop
### 可配TGP


`uniwill-laptop` 驱动允许为支持此功能的、带 NVIDIA GPU 的设备设置可配置 TGP
详见 Documentation/ABI/testing/sysfs-driver-uniwill-laptop
## 参考资
