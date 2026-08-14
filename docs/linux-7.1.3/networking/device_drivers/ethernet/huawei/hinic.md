
## 华为智能网卡（HiNIC）系列 Linux 内核驱动


## 概述：


HiNIC 是面向数据中心领域的网络接口卡。

该驱动支持一系列不同链路速率的设备（10GbE、25GbE、40GbE 等）。该驱动还支持协商式、可扩展的特性集。

部分 HiNIC 设备支持 SR-IOV。本驱动用于物理功能（PF）。

HiNIC 设备为每个 Tx/Rx 队列支持 MSI-X 中断向量以及自适应中断调节。

HiNIC 设备还支持多种卸载特性，例如校验和卸载、TCP 发送分段卸载（TSO）、接收端缩放（RSS）以及 LRO（Large Receive Offload，大接收卸载）。


## 支持的 PCI 厂商 ID/设备 ID：


19e5:1822 - HiNIC PF


## 驱动架构与源代码：


hinic_dev - 实现一个与具体硬件（HW）数据结构格式细节无关的逻辑网络设备。

hinic_hwdev - 实现设备的硬件细节，并包含访问 PCI 网卡所需的组件。

## hinic_hwdev 包含以下组件：


## 硬件接口：


用于访问 pci 设备（DMA 内存与 PCI BAR）的接口。
（hinic_hw_if.c、hinic_hw_if.h）

描述配置与状态 BAR0 上硬件寄存器的配置状态寄存器（CSR）区域。（hinic_hw_csr.h）

## 管理（MGMT）组件：


异步事件队列（AEQ）- 用于接收来自网卡上 MGMT 模块消息的事件队列。（hinic_hw_eqs.c、hinic_hw_eqs.h）

应用程序可编程接口命令（API CMD）- 用于向网卡发送 MGMT 命令的接口。（hinic_hw_api_cmd.c、hinic_hw_api_cmd.h）

管理（MGMT）- PF 到 MGMT 的通道，使用 API CMD 向网卡发送 MGMT 命令，并通过 AEQ 接收来自网卡上 MGMT 模块的通知。同时设置硬件中 IO CMDQ 的地址。
（hinic_hw_mgmt.c、hinic_hw_mgmt.h）

## IO 组件：


完成事件队列（CEQ）- 描述已完成的 IO 任务的完成事件队列。（hinic_hw_eqs.c、hinic_hw_eqs.h）

工作队列（WQ）- 包含供 CMD 队列和队列对使用的内存与操作。WQ 是页中的一块内存。该块包含指向内存区（即工作队列元素 WQE 的内存）的指针。
（hinic_hw_wq.c、hinic_hw_wq.h）

命令队列（CMDQ）- 用于发送 IO 管理命令的队列，用于设置硬件中的 QP 地址。命令的完成事件累积在配置为接收 CMDQ 完成事件的 CEQ 上。
（hinic_hw_cmdq.c、hinic_hw_cmdq.h）

队列对（QP）- 用于接收和发送数据的硬件接收与发送队列。（hinic_hw_qp.c、hinic_hw_qp.h、hinic_hw_qp_ctxt.h）

IO - 构建/拆解所有 IO 组件。（hinic_hw_io.c、hinic_hw_io.h）

## 硬件设备：


硬件设备 - 在驱动初始化时构建/拆解硬件接口、MGMT 组件，并在接口 UP/DOWN 事件时构建/拆解 IO 组件。（hinic_hw_dev.c、hinic_hw_dev.h）


## hinic_dev 包含以下组件：


PCI ID 表 - 包含受支持的 PCI 厂商/设备 ID。
（hinic_pci_tbl.h）

端口命令 - 向硬件设备发送端口管理命令（MAC、Vlan、MTU 等）。（hinic_port.c、hinic_port.h）

Tx 队列 - 使用硬件发送队列进行发送的逻辑 Tx 队列。逻辑 Tx 队列不依赖于硬件发送队列的格式。
（hinic_tx.c、hinic_tx.h）

Rx 队列 - 使用硬件接收队列进行接收的逻辑 Rx 队列。逻辑 Rx 队列不依赖于硬件接收队列的格式。
（hinic_rx.c、hinic_rx.h）

hinic_dev - 构建/拆解逻辑 Tx 与 Rx 队列。
（hinic_main.c、hinic_dev.h）


## 杂项


硬件与逻辑设备共用的通用函数。
（hinic_common.c、hinic_common.h）


## 支持


如果发现所发布的源代码在受支持内核、配合受支持适配器时存在问题，请将与该问题相关的具体信息通过电子邮件发送至
aviad.krawczyk@huawei.com。
