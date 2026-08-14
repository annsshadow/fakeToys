
## 内核驱动 macsmc-hwmon


支持的硬件

    - Apple Silicon Macs（M1 及更高版本）

作者：James Calligeros <jcalligeros99@gmail.com>

### 描述


macsmc-hwmon 通过 hwmon 暴露 Apple 系统管理控制器的
温度、电压、电流和功率传感器，以及风扇转速与控制能力。

由于每款 Apple Silicon Mac 暴露的传感器集各不相同
（例如 MacBook 暴露了桌面版 Mac 没有的电池遥测数据），
任意给定机器上存在的传感器都通过 Devicetree 描述。该驱动
在探测时获取这些描述并向 hwmon 注册。

手动风扇转速通过 fan_control 模块参数支持。该参数默认
禁用并标记为不安全，因为无法证明在因使用手动风扇控制
导致过热时系统会安全失效。

### sysfs 接口


currX_input
    电流表数值

currX_label
    电流表标签

fanX_input
    当前风扇转速

fanX_label
    风扇标签

fanX_min
    最小可行风扇转速

fanX_max
    最大可行风扇转速

fanX_target
    当前设定值

inX_input
    电压表数值

inX_label
    电压表标签

powerX_input
    功率表数值

powerX_label
    功率表标签

tempX_input
    温度传感器数值

tempX_label
    温度传感器标签
