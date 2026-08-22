
## 用于 Intel(R) 以太网多主机控制器（Ethernet Multi-host Controller）的 Linux 基础驱动

2018 骞?8 鏈?20 鏃?Copyright(c) 2015-2018 Intel Corporation.

## 目录

- 识别你的适配- 额外配置
- 性能调优
- 已知问题
- 支持

## 识别你的适配
本发行版中的驱动兼容基于 Intel(R) 以太网多主机控制器的设备
关于如何识别你的适配器，以及获取最新的 Intel 网络驱动，请参阅 Intel 支持网站https://www.intel.com/support

### 流控制（Flow Control
Intel(R) 以太网交换主机接口（Ethernet Switch Host Interface）驱动不支持流控制它不会发送暂停（pause）帧。这可能导致丢帧
### 虚拟功能（Virtual Functions，VFs
使用 sysfs 来启VF有效范围-64

```

    echo $num_vf_enabled > /sys/class/net/$dev/device/sriov_numvfs //启用 VFs
    echo 0 > /sys/class/net/$dev/device/sriov_numvfs //禁用 VFs

```
注意：设备和驱动都不控制 VF 如何映射到配置空间。总线布局会因操作系统而异。在支持
的操作系统上，你可以检sysfs 来查找映射关系
注意：当 SR-IOV 模式启用时，硬件 VLAN 过滤以及 VLAN 标签剥离/插入将保持启用。请
绉婚櫎鏃х殑 VLAN 杩囨护鍣?```

    ip link set eth0 vf 0 vlan 100	// VF 0 设置 vlan 100
    ip link set eth0 vf 0 vlan 0	// 删除 vlan 100
    ip link set eth0 vf 0 vlan 200	// VF 0 设置一个新vlan 200


```
## 额外功能与配
### 巨帧（Jumbo Frames
通过把最大传输单元（MTU）改为大于默认1500 的值来启用巨帧支持
使用 ifconfig 命令来增MTU 大小。例如，输入
```

    ifconfig eth<x> mtu 9000 up

```
```

    ip link set mtu 9000 dev eth<x>
    ip link set up dev eth<x>

```
此设置不会在重启后保留。可以通过在以下文件中添加 'MTU=9000' 使设置永久生效：

- 对于 RHELetc/sysconfig/network-scripts/ifcfg-eth<x>
- 对于 SLESetc/sysconfig/network/<config_file>

注意：巨帧的最MTU 设置15342。该值与 15364 字节的最大巨帧大小一致
注意：该驱动会尝试使用多个页大小的缓冲区来接收每个巨帧数据包。这有助于在分配接收
数据包时避免缓冲区耗尽问题
### 通用接收卸载（Generic Receive Offload，即 GRO
该驱动支持内核内GRO 软件实现。GRO 表明，通过Rx 流量合并为更大的数据块，在大
Rx 负载下可以显著降CPU 使用率。GRO 是之前使用的 LRO 接口的演进。GRO 能够合并
除了 TCP 之外的其它协议。它也可以在LRO 有问题的配置（即桥接iSCSI）中安全使用
### 用于过滤的受支持 ethtool 命令与选项

-n --show-nfc
  获取接收网络流分类（receive network flow classification）配置
rx-flow-hash tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6
  获取指定网络流量类型的哈希选项
-N --config-nfc
  配置接收网络流分类
rx-flow-hash tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6 m|v|t|s|d|f|n|r
  配置指定网络流量类型的哈希选项
- udp4：基IPv4 UDP
- udp6：基IPv6 UDP
- f 基于接收数据包第 4 层（Layer 4）头的第 0 1 字节进行哈希- n 基于接收数据包第 4 层的2 3 字节进行哈希
## 已知问题/故障排查

### Linux KVM 下的 64 Microsoft Windows Server 2012/R2 客户机操作系统中启用 SR-IOV

KVM Hypervisor/VMM 支持PCIe 设备直接分配VM。这包括传统PCIe 设备，以及基Intel Ethernet Controller XL710 的具SR-IOV 能力的设备
## 支持

有关一般信息，请访Intel 支持网站https://www.intel.com/support/

如果在受支持的内核上使用受支持的适配器发现了已发布源代码中的问题，请将与该问题相关的
具体信息发送至 intel-wired-lan@lists.osuosl.org