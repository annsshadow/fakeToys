## 面向 Intel(R) 以太网 10 千兆 PCI Express 适配器的 Linux 基础驱动程序


Intel 10 千兆 Linux 驱动程序。
Copyright(c) 1999-2018 Intel Corporation.
## 目录


- 识别您的适配器
- 命令行参数
- 附加配置
- 已知问题
- 支持
## 识别您的适配器


该驱动程序兼容基于以下器件的设备：

 - Intel(R) Ethernet Controller 82598
 - Intel(R) Ethernet Controller 82599
 - Intel(R) Ethernet Controller X520
 - Intel(R) Ethernet Controller X540
 - Intel(R) Ethernet Controller x550
 - Intel(R) Ethernet Controller X552
 - Intel(R) Ethernet Controller X553

有关如何识别您的适配器以及获取最新 Intel 网络驱动程序的信息，请参阅 Intel 支持网站：
https://www.intel.com/support
### 带可插拔光模块的 SFP+ 设备


#### 基于 82599 的适配器


注意：
- 如果您的基于 82599 的 Intel(R) 网络适配器随附 Intel 光模块，或是 Intel(R) Ethernet Server Adapter X520-2，则它仅支持下列 Intel 光模块和/或直接连接电缆。
- 当基于 82599 的 SFP+ 设备背靠背连接时，应通过 ethtool 将其设置为相同的 Speed 设置。若混用速率设置，结果可能有所不同。

+---------------+---------------------------------------+------------------+
| Supplier      | Type                                  | Part Numbers     |
+===============+=======================================+==================+
| SR Modules                                                               |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ SR (bailed)     | FTLX8571D3BCV-IT |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ SR (bailed)     | AFBR-703SDZ-IN2  |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ SR (bailed)     | AFBR-703SDDZ-IN1 |
+---------------+---------------------------------------+------------------+
| LR Modules                                                               |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ LR (bailed)     | FTLX1471D3BCV-IT |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ LR (bailed)     | AFCT-701SDZ-IN2  |
+---------------+---------------------------------------+------------------+
| Intel         | DUAL RATE 1G/10G SFP+ LR (bailed)     | AFCT-701SDDZ-IN1 |
+---------------+---------------------------------------+------------------+

以下是一份经过部分测试的第三方 SFP+ 模块列表。并非所有模块都适用于所有设备。

+---------------+---------------------------------------+------------------+
| Supplier      | Type                                  | Part Numbers     |
+===============+=======================================+==================+
| Finisar       | SFP+ SR bailed, 10g single rate       | FTLX8571D3BCL    |
+---------------+---------------------------------------+------------------+
| Avago         | SFP+ SR bailed, 10g single rate       | AFBR-700SDZ      |
+---------------+---------------------------------------+------------------+
| Finisar       | SFP+ LR bailed, 10g single rate       | FTLX1471D3BCL    |
+---------------+---------------------------------------+------------------+
| Finisar       | DUAL RATE 1G/10G SFP+ SR (No Bail)    | FTLX8571D3QCV-IT |
+---------------+---------------------------------------+------------------+
| Avago         | DUAL RATE 1G/10G SFP+ SR (No Bail)    | AFBR-703SDZ-IN1  |
+---------------+---------------------------------------+------------------+
| Finisar       | DUAL RATE 1G/10G SFP+ LR (No Bail)    | FTLX1471D3QCV-IT |
+---------------+---------------------------------------+------------------+
| Avago         | DUAL RATE 1G/10G SFP+ LR (No Bail)    | AFCT-701SDZ-IN1  |
+---------------+---------------------------------------+------------------+
| Finisar       | 1000BASE-T SFP                        | FCLF8522P2BTL    |
+---------------+---------------------------------------+------------------+
| Avago         | 1000BASE-T                            | ABCU-5710RZ      |
+---------------+---------------------------------------+------------------+
| HP            | 1000BASE-SX SFP                       | 453153-001       |
+---------------+---------------------------------------+------------------+

基于 82599 的适配器支持所有符合 SFF-8431 v4.1 和 SFF-8472 v10.4 规范的被动及有源限流直接连接电缆。
#### 执行 ifconfig ethX down 时 SFP+ 的激光关闭


"ifconfig ethX down" 会关闭基于 82599 的 SFP+ 光纤适配器的激光。
"ifconfig ethX up" 会开启激光。
或者，您也可以使用 "ip link set [down/up] dev ethX" 来关闭和开启激光。
#### 基于 82599 的 QSFP+ 适配器


注意：
- 如果您的基于 82599 的 Intel(R) 网络适配器随附 Intel 光模块，则它仅支持 Intel 光模块。
- 基于 82599 的 QSFP+ 适配器仅支持 4x10 Gbps 连接。不支持 1x40 Gbps 连接。QSFP+ 链路对端必须配置为 4x10 Gbps。
- 基于 82599 的 QSFP+ 适配器不支持自动链路速率检测。链路速率必须配置为 10 Gbps 或 1 Gbps，以匹配链路对端的速率能力。错误的速率配置将导致链路建立失败。
- Intel(R) Ethernet Converged Network Adapter X520-Q1 仅支持下列光模块和直接连接电缆。

+---------------+---------------------------------------+------------------+
| Supplier      | Type                                  | Part Numbers     |
+===============+=======================================+==================+
| Intel         | DUAL RATE 1G/10G QSFP+ SRL (bailed)   | E10GQSFPSR       |
+---------------+---------------------------------------+------------------+

基于 82599 的 QSFP+ 适配器支持所有符合 SFF-8436 v4.1 规范的被动及有源限流 QSFP+ 直接连接电缆。
#### 基于 82598 的适配器


注意：
- 支持可插拔光模块的 Intel(r) 以太网网络适配器仅支持其原始模块类型（例如，Intel(R) 10 Gigabit SR Dual Port Express Module 仅支持 SR 光模块）。如果您插入不同类型的模块，驱动程序将不会加载。
- 不支持光模块的热交换/热插拔。
- 仅支持单速率、10 千兆模块。
- 主板集成 LAN（LOM）可能支持 DA、SR 或 LR 模块。不支持其他模块类型。详情请参阅您的系统文档。

以下是一份经过部分测试的 SFP+ 模块和直接连接电缆列表。并非所有模块都适用于所有设备。

+---------------+---------------------------------------+------------------+
| Supplier      | Type                                  | Part Numbers     |
+===============+=======================================+==================+
| Finisar       | SFP+ SR bailed, 10g single rate       | FTLX8571D3BCL    |
+---------------+---------------------------------------+------------------+
| Avago         | SFP+ SR bailed, 10g single rate       | AFBR-700SDZ      |
+---------------+---------------------------------------+------------------+
| Finisar       | SFP+ LR bailed, 10g single rate       | FTLX1471D3BCL    |
+---------------+---------------------------------------+------------------+

基于 82598 的适配器支持所有符合 SFF-8431 v4.1 和 SFF-8472 v10.4 规范的被动直接连接电缆。不支持有源直接连接电缆。

上述第三方光模块和电缆仅出于说明第三方规范和潜在兼容性之目的而列出，并不代表 Intel 对任何第三方产品的推荐、认可或赞助。Intel 不认可或推广任何第三方制造的产品，提供第三方名称仅为分享与具有上述规格的某些光模块和电缆相关的信息。可能还有其他制造商或供应商生产或供应具有类似或相符描述的光模块和电缆。客户必须自行谨慎并勤勉地从任意所选第三方处采购光模块和电缆。客户全权负责评估产品及/或设备的适用性，以及为采购任何产品而选择供应商。上述光模块和电缆不享受 Intel 的保修或支持。Intel 不承担任何责任，并否认任何与此类第三方产品的销售和/或使用或客户对供应商的选择相关的明示或暗示保证。
## 命令行参数


### max_vfs

:Valid Range: 1-63

该参数增加了对 SR-IOV 的支持。它使驱动程序生成最多 max_vfs 个虚拟功能。如果该值大于 0，它还会强制 VMDq 参数为 1 或更大。

注意：该参数仅用于内核 3.7.x 及更早版本。在内核 3.8.x 及更高版本中，请使用 sysfs 启用 VF。此外，对于 Red Hat 发行版，该参数仅用于 6.6 及更旧版本。对于 6.7 及更新版本，请使用

```

  #echo $num_vf_enabled > /sys/class/net/$dev/device/sriov_numvfs // enable VFs
  #echo 0 > /sys/class/net/$dev/device/sriov_numvfs               //disable VFs

```
驱动程序的参数按位置引用。因此，如果您的系统中有双端口适配器或多个适配器，并且希望每个端口有 N 个虚拟功能，则必须为每个端口的每个参数指定一个数字

```

  modprobe ixgbe max_vfs=4

```
这将在第一个端口上生成 4 个 VF。


```

  modprobe ixgbe max_vfs=2,4

```
这将在第一个端口上生成 2 个 VF，在第二个端口上生成 4 个 VF。

注意：加载带有这些参数的驱动程序时必须谨慎。取决于您的系统配置、插槽数量等，无法在所有情况下预测命令行上的位置。

注意：设备和驱动程序都不控制 VF 如何映射到配置空间。总线布局因操作系统而异。在支持的操作系统上，您可以检查 sysfs 以查找映射。

注意：当启用 SR-IOV 模式或 VMDq 模式时，硬件 VLAN 过滤和 VLAN 标签剥离/插入将保持启用。在添加新 VLAN 过滤器之前，请先移除旧的 VLAN 过滤器。例如，


```

  ip link set eth0 vf 0 vlan 100 // set VLAN 100 for VF 0
  ip link set eth0 vf 0 vlan 0   // Delete VLAN 100
  ip link set eth0 vf 0 vlan 200 // set a new VLAN 200 for VF 0

```
在内核 3.6 中，驱动程序支持同时使用 max_vfs 和 DCB 特性，但须遵守下述约束。在内核 3.6 之前，驱动程序不支持 max_vfs 大于 0 与 DCB 特性（利用优先流控制和扩展传输选择的多个流量类别）同时运行。

启用 DCB 后，网络流量通过多个流量类别（NIC 中的数据包缓冲区）进行收发。流量根据优先级与特定类别关联，优先级的值为 0 到 7，用于 VLAN 标签中。未启用 SR-IOV 时，每个流量类别与一组接收/发送描述符队列对关联。给定流量类别的队列对数量取决于硬件配置。启用 SR-IOV 后，描述符队列对被分组为池。物理功能（PF）和每个虚拟功能（VF）被分配一组接收/发送描述符队列对。当配置了多个流量类别（例如启用了 DCB）时，每个池包含来自每个流量类别的一个队列对。当硬件中配置了单个流量类别时，池包含来自该单一流量类别的多个队列对。

可分配的 VF 数量取决于可启用的流量类别数量。每个已启用 VF 的可配置流量类别数量如下：
0 - 15 VFs = Up to 8 traffic classes, depending on device support
16 - 31 VFs = Up to 4 traffic classes
32 - 63 VFs = 1 traffic class

配置 VF 时，PF 也会被分配一个池。PF 支持 DCB 特性，但约束是每个流量类别仅使用单个队列对。当配置零个 VF 时，PF 可以为每个流量类别支持多个队列对。
### allow_unsupported_sfp


:Valid Range: 0,1
:Default Value: 0 (disabled)

只要驱动程序已知模块类型，该参数便允许在基于 82599 的适配器上使用不受支持且未经测试的 SFP+ 模块。
### debug


:Valid Range: 0-16 (0=none,...,16=all)
:Default Value: 0

该参数调整系统日志中显示的调试消息级别。
## 附加特性与配置


### 流控制


以太网流控制（IEEE 802.3x）可通过 ethtool 配置，以启用 ixgbe 的接收和发送暂停帧。启用发送后，当接收数据包缓冲区超过预定义阈值时会生成暂停帧。启用接收后，发送单元将在收到暂停帧时指定的延迟时间内停止。

注意：您必须拥有一个支持流控制的链路对端。

流控制默认启用。

使用 ethtool 更改流控制设置。要启用或禁用 Rx 或

```

  ethtool -A eth? rx <on|off> tx <on|off>

```
注意：仅当自动协商被禁用时，此命令才会启用或禁用流控制。如果启用了自动协商，此命令会更改与链路对端进行自动协商所用的参数。

```

  ethtool -s eth? autoneg <on|off>

```
注意：流控制自动协商是链路自动协商的一部分。取决于您的设备，您可能无法更改自动协商设置。

注意：对于进入 1 千兆模式的 82598 背板卡，流控制默认行为更改为关闭。这些设备在 1 千兆模式下的流控制可能导致发送挂起。
### Intel(R) Ethernet Flow Director


Intel 以太网流导向器执行以下任务：

- 根据流将接收数据包引导至不同的队列。
- 实现对平台中流路由的严格控制。
- 将流与 CPU 核心匹配以实现流亲和性。
- 支持多个参数以进行灵活的流分类和负载均衡（仅限 SFP 模式）。

注意：Intel 以太网流导向器的掩码工作方式与之相反：

```

  #ethtool -N eth11 flow-type ip4 src-ip 172.4.1.2 m 255.0.0.0 dst-ip \
  172.21.1.1 m 255.128.0.0 action 31

```
写入过滤器中的 src-ip 值将为 0.4.1.2，而不是可能预期的 172.0.0.0。类似地，写入过滤器的 dst-ip 值将为 0.21.1.1，而不是 172.0.0.0。

```

  # ethtool -K ethX ntuple <on|off>

```
禁用 ntuple 过滤器时，所有用户编程的过滤器都会从驱动程序缓存和硬件中刷新。重新启用 ntuple 后，必须重新添加所有需要的过滤器。

```

  # ethtool -N ethX flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 src-port 2000 dst-port 2001 action 2 [loc 1]

```

```

  # ethtool <-u|-n> ethX

```

### 边带完美过滤器


边带完美过滤器用于引导匹配指定特征的业务流。它们通过 ethtool 的 ntuple 接口启用。要添加一条

```

  ethtool -U <device> flow-type <type> src-ip <ip> dst-ip <ip> src-port <port> \
  dst-port <port> action <queue>

```
其中：
  <device> - 要配置的以太网设备
  <type> - 可以是 ip4、tcp4、udp4 或 sctp4
  <ip> - 要匹配的 IP 地址
  <port> - 要匹配的端口号
  <queue> - 业务流导向的队列（-1 丢弃匹配的业务流）

```

  ethtool -U <device> delete <N>

```
其中 <N> 是打印所有活动过滤器时显示的过滤器 id，也可以在添加过滤器时使用 "loc <N>" 指定。

以下示例匹配从 192.168.0.1、端口 5300 发送的 TCP 业务流，

```

  ethtool -U enp130s0 flow-type tcp4 src-ip 192.168.0.1 dst-ip 192.168.0.5 \
  src-port 5300 dst-port 80 action 7

```
对于每种流类型，已编程的过滤器必须具有相同的匹配

```

  ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.5 src-port 55 action 10

```
但是，发出接下来的两条命令是不可接受的，因为第一条

```

  ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  ethtool -U enp130s0 flow-type ip4 dst-ip 192.168.0.5 src-port 55 action 10

```
第二条命令将失败并报错。您可以使用相同的字段、不同的数值编程多个过滤器，但在一个设备上，您不能编程两个具有不同匹配字段的 TCP4 过滤器。

ixgbe 驱动程序不支持对字段的子部分进行匹配，因此不支持部分掩码字段。

要创建将业务流导向特定虚拟功能（Virtual Function）的过滤器，请使用 "user-def" 参数。将 user-def 指定为 64 位值，其中低 32 位表示队列号，接下来的 8 位表示哪个 VF。

```

  ... user-def 0x800000002 ...

```
表示将业务流导向虚拟功能 7（8 减 1）到该 VF 的队列 2 中。

请注意，这些过滤器不会破坏内部路由规则，也不会将原本不会发往指定虚拟功能的业务流路由过去。
### 巨帧


巨帧支持通过更改最大传输单元（Maximum Transmission Unit，MTU）为大于默认值 1500 的值来启用。

使用 ifconfig 命令增大 MTU 大小。例如，输入

```

  ifconfig eth<x> mtu 9000 up

```

```

  ip link set mtu 9000 dev eth<x>
  ip link set up dev eth<x>

```
此设置不会在重启后保留。可通过以下方式更改设置：

```

  /etc/sysconfig/network-scripts/ifcfg-eth<x> // for RHEL
  /etc/sysconfig/network/<config_file> // for SLES

```
注意：巨帧的最大 MTU 设置为 9710。该值与 9728 字节的最大巨帧大小一致。

注意：本驱动程序将尝试使用多个页大小的缓冲区来接收每个巨帧数据包。这有助于在分配接收数据包时避免缓冲区耗尽问题。

注意：对于基于 82599 的网络连接，如果在虚拟功能（VF）中启用巨帧，必须先在物理功能（PF）中启用巨帧。VF 的 MTU 设置不能大于 PF 的 MTU。
### NBASE-T 支持


ixgbe 驱动程序在某些设备上支持 NBASE-T。但是，为兼容无法处理已通告 NBASE-T 速率的故障交换机，NBASE-T 速率的通告默认被抑制。请使用 ethtool

```

  ethtool -s eth? advertise 0x1800000001028

```
在带有 INTERFACES(5) 的 Linux 系统上，可以将其指定为 pre-up 命令
写入 /etc/network/interfaces，以便接口始终以

```

  iface eth? inet dhcp
       pre-up ethtool -s eth? advertise 0x1800000001028 || true

```

### 通用接收卸载（Generic Receive Offload，简称 GRO）


该驱动程序支持内核内 GRO 的软件实现。GRO 已证明，通过将 Rx 流量合并为更大的数据块，在大 Rx 负载下可显著降低 CPU 使用率。GRO 是先前使用的 LRO 接口的演进。GRO 能够合并除 TCP 之外的其他协议。它也可安全地用于那些对 LRO 有问题的配置，即桥接和 iSCSI。
### 数据中心桥接（Data Center Bridging，DCB）


注意：
内核假定 TC0 可用，如果 TC0 不可用，将在设备上禁用优先流控制（PFC）。要解决此问题，请在交换机上配置 DCB 时确保启用 TC0。

DCB 是硬件中的一种服务质量（Quality of Service）配置实现。它使用 VLAN 优先级标签（802.1p）来过滤流量。这意味着流量可被过滤到 8 个不同的优先级。它还启用优先流控制（802.1Qbb），可在网络压力期间限制或消除丢包数量。带宽可分配给这些优先级中的每一个，并在硬件层面强制执行（802.1Qaz）。

适配器固件分别按照 802.1AB 和 802.1Qaz 实现 LLDP 和 DCBX 协议代理。基于固件的 DCBX 代理仅以 willing 模式运行，并可接受来自具备 DCBX 能力的对端的设置。不支持通过 dcbtool/lldptool 对 DCBX 参数进行软件配置。

ixgbe 驱动程序实现了 DCB netlink 接口层，以允许用户空间与驱动程序通信并查询端口的 DCB 配置。
### ethtool


该驱动程序利用 ethtool 接口进行驱动程序配置和诊断，以及显示统计信息。此功能需要最新版本的 ethtool。请于此下载：
https://www.kernel.org/pub/software/network/ethtool/
### FCoE


ixgbe 驱动程序支持基于以太网的光纤通道（Fiber Channel over Ethernet，FCoE）和数据中心桥接（DCB）。此代码对常规驱动程序操作没有默认影响。配置 DCB 和 FCoE 超出本 README 的范围。有关 FCoE 项目信息，请参阅 http://www.open-fcoe.org/；有关 DCB 信息，请联系 ixgbe-eedc@lists.sourceforge.net。
### MAC 和 VLAN 防欺骗特性


当恶意驱动程序试图发送欺骗数据包时，它会被硬件丢弃而不被发送。

会向 PF 驱动程序发送一个中断，通知其发生了欺骗尝试。当检测到欺骗数据包时，PF 驱动程序将向

```

  ixgbe ethX: ixgbe_spoof_check: n spoofed packets detected

```
其中 "x" 是 PF 接口号；"n" 是欺骗数据包的数量。

```

  ip link set <pf dev> vf <vf id> spoofchk {off|on}

```

### IPsec 卸载


ixgbe 驱动程序支持 IPsec 硬件卸载。使用 "ip xfrm ..." 创建安全关联（Security Association）时，可以使用 'offload' 标签选项将 IPsec SA 注册到驱动程序，以在安全通信中获得更高的吞吐量。

该卸载也支持 ixgbe 的 VF，但 VF 必须设置为

```

  ethtool --set-priv-flags eth<x> vf-ipsec on
  ip link set eth<x> vf <y> trust on


```

## 已知问题/故障排查


### 在 64 位 Microsoft Windows Server 2012/R2 客户机操作系统中启用 SR-IOV


Linux KVM Hypervisor/VMM 支持将 PCIe 设备直接分配给 VM。这包括传统 PCIe 设备，以及基于 Intel Ethernet Controller XL710 的支持 SR-IOV 的设备。
## 支持


有关一般信息，请访问 Intel 支持网站：
https://www.intel.com/support/

如果在受支持内核上使用受支持适配器时发现已发布源代码存在问题，请将与该问题相关的具体信息通过电子邮件发送至 intel-wired-lan@lists.osuosl.org。
