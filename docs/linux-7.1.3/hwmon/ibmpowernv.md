## 内核驱动 IBMPOWERNV


支持的系统：

  - 任何基于 POWERNV 平台的近期 IBM P 服务器

作者：Neelesh Gupta

### 描述


该驱动实现对 'POWERNV' 平台的温度/风扇/电压/功率等平台传感器数据的读取。

该驱动使用平台设备基础设施。它在 __init 阶段探测设备树以寻找传感器设备，并将它们注册到 'hwmon'。'hwmon' 填充具有属性文件的 'sysfs' 树，每个文件对应一个给定的传感器类型及其属性数据。

DT 中的所有节点都出现在 "/ibm,opal/sensors" 下，DT 中的每个有效节点都映射到 'sysfs' 中的一个属性文件。该节点导出唯一的 'sensor-id'，驱动使用它向固件发起 OPAL 调用。

### 使用说明


该驱动通过启用配置 CONFIG_SENSORS_IBMPOWERNV 与内核静态构建。它也可以作为模块 'ibmpowernv' 构建。

### Sysfs 属性


======================= =======================================================
fanX_input		测量的 RPM 值。
fanX_min		生成报警的 RPM 阈值。
fanX_fault		- 0：无故障条件
   - 1：风扇故障

tempX_input		测量的环境温度。
tempX_max		生成报警的环境温度阈值。
tempX_highest		历史最高温度
tempX_lowest		历史最低温度
tempX_enable		启用/禁用属于该子组的所有温度传感器。在 POWER9 中，此属性对应于每个 OCC。使用此属性可要求每个 OCC 禁用/启用其所有温度传感器。

   - 1：启用
   - 0：禁用

inX_input		测量的电源电压（毫伏）
inX_fault		- 0：无故障条件。
   - 1：电源故障。
inX_highest		历史最高电压
inX_lowest		历史最低电压
inX_enable		启用/禁用属于该子组的所有电压传感器。在 POWER9 中，此属性对应于每个 OCC。使用此属性可要求每个 OCC 禁用/启用其所有电压传感器。

   - 1：启用
   - 0：禁用

powerX_input		功耗（微瓦）
powerX_input_highest	历史最大功率
powerX_input_lowest	历史最小功率
powerX_enable		启用/禁用属于该子组的所有功率传感器。在 POWER9 中，此属性对应于每个 OCC。使用此属性可要求每个 OCC 禁用/启用其所有功率传感器。

   - 1：启用
   - 0：禁用

currX_input		测量的电流（毫安）
currX_highest		历史最大电流
currX_lowest		历史最小电流
currX_enable		启用/禁用属于该子组的所有电流传感器。在 POWER9 中，此属性对应于每个 OCC。使用此属性可要求每个 OCC 禁用/启用其所有电流传感器。

   - 1：启用
   - 0：禁用

energyX_input		累积能量（微焦）
======================= =======================================================
