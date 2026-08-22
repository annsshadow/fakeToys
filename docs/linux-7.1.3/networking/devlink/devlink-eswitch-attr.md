
## Devlink E-Switch 属


Devlink E-Switch 支持两种操作模式：legacy switchdev。Legacy 模式基于传统MAC/VLAN 导向规则运行。切换决策基MAC 地址、VLAN 等做出。将切换规则卸载到硬件的能力有限

另一方面，switchdev 模式允许E-Switch 更多地高级卸载能力交给硬件。在 switchdev 模式下，更多的切换规则与逻辑可以被卸载到硬件交换 ASIC 上。它启用了代表设备虚拟功能（VF）或可扩展功能（SF）慢速路径的 representor netdevices。有关更多信息，请参Documentation/networking/switchdev.rst <switchdev> Documentation/networking/representors.rst <representors>

此外，devlink E-Switch 还附带了下一节列出的其他属性

## 属性描


以下E-Switch 属性的列表

   :widths: 8 5 45

   - - 名称
     - 类型
     - 描述
   - - `mode`
     - enum
     - 设备的模式。模式可以是以下之一

       - `legacy` 基于传统 MAC/VLAN 导向规则运行
       - `switchdev` 允许E-Switch 更多地高级卸载能力交给硬件
       - `switchdev_inactive` switchdev 模式但启动时处于非激活状态，在显式激活前不允许流量通过。此模式对于希望switchdev 模式准备设备、但仅在所有配置完成后才激活它的编排器很有用
   - - `inline-mode`
     - enum
     - 某些硬件需VF 驱动将部分数据包头部放入 TX 描述符，以便 e-switch 能够进行正确的匹配与导向。switchdev 模式legacy 模式均支持

       - `none` 无
       - `link` L2 模式
       - `network` L3 模式
       - `transport` L4 模式
   - - `encap-mode`
     - enum
     - 设备的封装模式。switchdev 模式legacy 模式均支持。模式可以是以下之一

       - `none` 禁用封装支持
       - `basic` 启用封装支持

## 使用示例


    # 启用 switchdev 模式
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev

    # 设置 inline-mode encap-mode
    $ devlink dev eswitch set pci/0000:08:00.0 inline-mode none encap-mode basic

    # 显示 devlink 设备eswitch 属
    $ devlink dev eswitch show pci/0000:08:00.0
      pci/0000:08:00.0: mode switchdev inline-mode none encap-mode basic

    # legacy 模式下启encap-mode
    $ devlink dev eswitch set pci/0000:08:00.0 mode legacy inline-mode none encap-mode basic

    # 以非激活状态启switchdev 模式
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev_inactive

    # 配置 switchdev 的设置、representors、FDB 条目.
    ...

    # 激switchdev 模式以允许流量通过
    $ devlink dev eswitch set pci/0000:08:00.0 mode switchdev
