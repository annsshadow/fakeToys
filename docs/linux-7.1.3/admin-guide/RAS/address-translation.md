## 地址转换


### x86 AMD


基于 Zen AMD 系统包含一个管理物理内存布局的数fabric（Data Fabric）。连接到 fabric 的设备（如内存控制器、I/O 等）可能不具备系统物理内存映射的完整视图。这些设备在报告内存错误时可能提供一个“规范化”（normalized，即设备物理）地址。规范化地址必须转换为系统物理地址，内核才能对该内存采取动作

AMD 地址转换库（CONFIG_AMD_ATL）为这种情况提供转换

基于 Zen 的系统地址转换中使用的缩略语表

- CCM               = 缓存一致性调节器（Cache Coherent Moderator
- COD               = 单芯片内多集群（Cluster-on-Die
- COH_ST            = 一致性站点（Coherent Station
- DF                = 数据 fabric（Data Fabric
