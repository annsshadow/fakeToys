## 内核驱动 ibmaem


该驱动与 IBM Systems Director Active Energy Manager 通信，以下简称 AEM。

支持的系统：

  - 任何近期支持 AEM 的 IBM System X 服务器。

    包括 x3350、x3550、x3650、x3655、x3755、x3850 M2、x3950 M2，以及某些 HC10/HS2x/LS2x/QS2x 刀片。

    IPMI 主机接口驱动（"ipmi-si"）需要被加载，该驱动才能工作。

    Prefix: 'ibmaem'

    Datasheet: 不可用

Author: Darrick J. Wong

### 描述


该驱动实现了通过 BMC 读取各种 IBM System X 硬件上可用能量和功率计的支持。所有传感器组都将作为平台设备导出；该驱动可以与 v1 和 v2 接口通信。该驱动与较旧的 ibmpex 驱动完全独立。

v1 AEM 接口具有一组简单的特性来监视能量使用。有一个寄存器显示自上次 BMC 复位以来的原始能量消耗估计值，以及一个返回可配置时间间隔内平均功率使用的功率传感器。

v2 AEM 接口更复杂一些，能够呈现更广泛的能量和功率使用寄存器、由 AEM 软件设置的功率上限，以及温度传感器。

### 特殊特性


"power_cap" 值显示当前系统功率上限，由 AEM 软件设置。目前不支持从主机设置功率上限。
