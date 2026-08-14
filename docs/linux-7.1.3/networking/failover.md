## FAILOVER


## 概述


failover 模块为半虚拟化驱动提供了一个通用接口，用于向 failover 实例注册一个 netdev 和一组 ops。这些 ops 用作事件处理程序，在从属 pci 以太网设备（与 failover netdev 具有相同 mac 地址）上发生 netdev 注册/注销/链路变更/名称变更事件时被调用。

这使得半虚拟化驱动能够使用 VF 作为加速的低延迟数据路径。它还允许在 VF 被拔出时故障切换到半虚拟化数据路径，从而实现对直连 VF 的虚拟机（VM）的实时迁移。
