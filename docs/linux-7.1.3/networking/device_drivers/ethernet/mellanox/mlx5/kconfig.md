
## Enabling the driver and kconfig options


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

| mlx5 core 是模块化的，大多数主要的 mlx5 core 驱动特性都可以在构建时通过内核 Kconfig 标志进行选择（编译进内核或排除）| 基本特性、以太网网络设备 rx/tx 卸载XDP，在使用最基本的标| CONFIG_MLX5_CORE=y/m CONFIG_MLX5_CORE_EN=y 时即可使用| 高级特性列表请见下文
**CONFIG_MLX5_BRIDGE=(y/n)**

|    启用以太网桥接（BRIDGE）卸载支<mlx5_bridge_offload>|    这将提供mlx5 uplink VF 端口的代表（representor）加Bridge|    以及为这些端口之间的流量卸载规则的能力|    支持 VLAN（trunk access 模式）

**CONFIG_MLX5_CORE=(y/m/n)** (module mlx5_core.ko)

|    可以通过在内核配置中选择 CONFIG_MLX5_CORE=y/m 来启用该驱动|    这将mlx5 ulp 提供用于接口mlx5 core 驱动（mlx5e、mlx5_ib）

**CONFIG_MLX5_CORE_EN=(y/n)**

|    选择此选项将提供具有所有标rx/tx 卸载的基本以太网络网络设备支持|    mlx5e 是提供网络设备内核接口的 mlx5 ulp 驱动，选中mlx5e 将内建到 mlx5_core.ko 中

**CONFIG_MLX5_CORE_EN_DCB=(y/n)**:

|    启用 `Data Center Bridging (DCB) Support <https://enterprise-support.nvidia.com/s/article/howto-auto-config-pfc-and-ets-on-connectx-4-via-lldp-dcbx>`_

**CONFIG_MLX5_CORE_IPOIB=(y/n)**

|    IPoIB 卸载与加速支持|    需CONFIG_MLX5_CORE_EN rdma IPoIB ulp 网络设备提供加速接口

**CONFIG_MLX5_CLS_ACT=(y/n)**

|    启用 TC 分类器动作（NET_CLS_ACT）的卸载支持|    在原NIC 模式Switchdev SRIOV 模式下均可用|    基于流的分类器（例如通过 `tc-flower(8)` 注册的那些）由设备处理，而非主机|    随后会覆盖匹配分类结果的动作由于卸载而即时生效

**CONFIG_MLX5_EN_ARFS=(y/n)**

|    启用硬件加速的接收流导向（arfs）支持，以及 ntuple 过滤|    https://enterprise-support.nvidia.com/s/article/howto-configure-arfs-on-connectx-4


**CONFIG_MLX5_EN_IPSEC=(y/n)**

|    启用 IPSec XFRM 加密卸载加<xfrm_device>

**CONFIG_MLX5_MACSEC=(y/n)**

|    构建NIC MACsec 加密卸载加速的支持

**CONFIG_MLX5_EN_RXNFC=(y/n)**

|    启用 ethtool 接收网络流分类，允许用户通过 ethtool set/get_rxnfc API
|    用自定义流规则把流量导向任意 rx 队列

**CONFIG_MLX5_EN_TLS=(y/n)**

|    TLS 加密卸载加速

**CONFIG_MLX5_ESWITCH=(y/n)**

|    ConnectX NIC 中的以太SRIOV E-Switch 支持。E-Switch 为启用的 VF PF 提供内部 SRIOV 数据包导向与交换，有两种可用模式|           1) `Legacy SRIOV mode (L2 mac vlan steering based) <https://enterprise-support.nvidia.com/s/article/HowTo-Configure-SR-IOV-for-ConnectX-4-ConnectX-5-ConnectX-6-with-KVM-Ethernet>`_|           2) Switchdev mode (eswitch offloads) <switchdev>

**CONFIG_MLX5_FPGA=(y/n)**

|    构建Mellanox Technologies Innova 系列网卡的支持|    Innova 网卡由一ConnectX 芯片和一FPGA 芯片组成|    如果选择此选项，mlx5_core 驱动将包Innova FPGA core，并允许构建特定于沙箱的客户端驱动

**CONFIG_MLX5_INFINIBAND=(y/n/m)** (module mlx5_ib.ko)

|    提供底层 InfiniBand/RDMA `RoCE <https://enterprise-support.nvidia.com/s/article/recommended-network-configuration-examples-for-roce-deployment>`_ 支持

**CONFIG_MLX5_MPFS=(y/n)**

|    ConnectX NIC 中的以太网多物理功能交换（MPFS）支持|    在启`Multi-Host <https://www.nvidia.com/en-us/networking/multi-host/>`_ 配置时需要使MPFs|    以允许把用户配置的单MAC 地址传递给请求PF

**CONFIG_MLX5_SF=(y/n)**

|    构建对子功能（subfunction）的支持|    子功能比 PCI SRIOV VF 更轻量。选择此选项将启用创建子功能设备的支持

**CONFIG_MLX5_SF_MANAGER=(y/n)**

|    构建NIC 中子功能端口的支持。Mellanox 子功能端口通过 devlink 管理。子功能支持 RDMA、网络设备和 vdpa 设备|    它类似于 SRIOV VF，但不需SRIOV 支持

**CONFIG_MLX5_SW_STEERING=(y/n)**

|    构建NIC 中软件管理导向（steering）的支持

**CONFIG_MLX5_HW_STEERING=(y/n)**

|    构建NIC 中硬件管理导向（steering）的支持

**CONFIG_MLX5_TC_CT=(y/n)**

|    支持通过 tc ct 动作卸载连接跟踪规则

**CONFIG_MLX5_TC_SAMPLE=(y/n)**

|    支持通过 tc sample 动作卸载采样规则

**CONFIG_MLX5_VDPA=(y/n)**

|    用于 Mellanox VDPA 驱动的支持库。提供所有类VDPA 驱动通用的代码|    计划包含以下驱动：net、block

**CONFIG_MLX5_VDPA_NET=(y/n)**

|    用于 ConnectX6 及更新版本的 VDPA 网络驱动。提供对 virtio net 数据路径的卸载，
|    使得放在环上的描述符将由硬件执行。它还根据所使用的实际设备和固件版本支持多种无状态卸载

**CONFIG_MLX5_VFIO_PCI=(y/n)**

|    这提供使VFIO 框架MLX5 设备迁移支持

**External options** ( 如果相应mlx5 特性是必需的则选择 )

- CONFIG_MLXFW: 选中后，将启mlx5 固件刷写支持（通过 devlink ethtool）- CONFIG_PTP_1588_CLOCK: 选中后，将启mlx5 ptp 支持
- CONFIG_VXLAN: 选中后，将启mlx5 vxlan 支持