## Microchip SparX-5 SoC


支持芯片：

  - VSC7546, VSC7549, VSC755, VSC7556, VSC7558（Sparx5 系列）

    Prefix: 'sparx5-temp'

    Addresses scanned: -

    Datasheet: 由 Microchip 应要求并在 NDA 下提供

Author: Lars Povlsen <lars.povlsen@microchip.com>

### 说明


Sparx5 SoC 包含一个基于 MR74060 Moortec IP 的温度传感器。

该传感器范围为 -40°C 到 +125°C，精度为 +/-5°C。

### Sysfs 条目


支持以下属性。

======================= ========================================================
temp1_input		芯片温度（单位为毫摄氏度。）
======================= ========================================================
