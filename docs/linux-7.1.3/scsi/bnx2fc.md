
## 使用 bnx2fc 运行 FCoE


通过 bnx2fc 实现的博通（Broadcom）FCoE 卸载是一种全状态硬件卸载，它与 Linux 生态系统中FC/FCoE SCSI 控制器提供的所有接口协同工作。因此，一旦启用，FCoE 功能在很大程度上是透明的。在 SAN 上发现的设备会自动向高层存储层注册和注销
尽管博通的 FCoE 卸载是完全卸载的，但它确实依赖于网络接口的运行状态。因此，FCoE 卸载启动器关联的网络接口（例eth0）必须处'up' 状态。建议将网络接口配置为在启动时自动启用
此外，博FCoE 卸载方案会创VLAN 接口，以支持FCoE 操作发现VLAN（例eth0.1001-fcoe）。不要删除或禁用这些接口，否FCoE 操作将中断
## 驱动使用模型

1. 确保已安fcoe-utils 软件包
2. 配置 bnx2fc 驱动需要运行的接口配置步骤如下
	a. cd /etc/fcoe
	b. 如果需要在 eth5 上启FCoE，将 cfg-ethx 复制cfg-eth5	c. 对所有需要启FCoE 的接口重复此操作	d. 编辑所cfg-eth 文件，将 DCB_REQUIRED** 字段设为 "no"，将
	   AUTO_VLAN 设为 "yes"	e. 其他配置参数保持默认即可
3. 确保 "bnx2fc" 位于 /etc/fcoe/config SUPPORTED_DRIVERS 列表中
4. 启动 fcoe 服务。（service fcoe start）。如果系统中存在博通设备，bnx2fc 驱动会自动占用这些接口，开vlan 发现并登录到目标
5. 'fcoeadm -i' 输出中的 "Symbolic Name" 会显bnx2fc 是否已占用该接口
```

 [root@bh2 ~]# fcoeadm -i
    Description:      NetXtreme II BCM57712 10 Gigabit Ethernet
    Revision:         01
    Manufacturer:     Broadcom Corporation
    Serial Number:    0010186FD558
    Driver:           bnx2x 1.70.00-0
    Number of Ports:  2

        Symbolic Name:     bnx2fc v1.0.5 over eth5.4
        OS Device Name:    host11
        Node Name:         0x10000010186FD559
        Port Name:         0x20000010186FD559
        FabricName:        0x2001000DECB3B681
        Speed:             10 Gbit
        Supported Speed:   10 Gbit
        MaxFrameSize:      2048
        FC-ID (Port ID):   0x0F0377
        State:             Online

```
6. 通过运行 ifconfig 并留意会自动创建<INTERFACE>.<VLAN>-fcoe 接口，验证是否已执行 vlan 发现
有关 fcoeadm 操作用来创建/销毁接口或显示 lun/目标信息的更多信息，请参fcoeadm 手册页
## 注意

** 支持博FCoE 的设备在芯片上实现了 DCBX/LLDP 客户端。每个接口只允许有一LLDP 客户端。为正常运行，必须禁用所有基于主机软件的 DCBX/LLDP 客户端（例如 lldpad）。要禁用 lldpad，请使用
```

	lldptool set-lldp -i <interface_name> adminStatus=disabled

```
