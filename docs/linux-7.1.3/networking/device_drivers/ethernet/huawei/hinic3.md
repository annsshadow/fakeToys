## 华为以太网设备驱动（hinic3）系Linux 内核驱动

## 概述

hinic3 是面向数据中心的网络接口卡（NIC）。它支持一系列不同链路速率的设备（10GE5GE00GE 等）。hinic3 设备可以有多种物理形态：LOM（主板集成局域网，Lan on Motherboard）NIC、PCIe 标准 NIC、OCP（开放计算项目，Open Compute Project）NIC 等
hinic3 驱动支持以下特性：
- IPv4/IPv6 TCP/UDP 校验和卸- TSO（TCP 分段卸载，TCP Segmentation Offload）、LRO（大接收卸载，Large Receive Offload- RSS（接收侧缩放，Receive Side Scaling- MSI-X 中断聚合配置与中断自适应
- SR-IOV（单I/O 虚拟化，Single Root I/O Virtualization
## 内容

- 受支持的 PCI 厂商 ID/设备 ID
- Hinic3 驱动源代码结- 管理接口

## 受支持的 PCI 厂商 ID/设备 ID

19e5:0222 - hinic3 PF/PPF
19e5:375F - hinic3 VF

主物理功能（PPF，Prime Physical Function）负责整NIC 卡的管理。例如，NIC 与主机之间的时钟同步。任PF 都可以充PPF。PPF 是动态选择的
## Hinic3 椹卞姩婧愪唬鐮佺粨鏋。
========================  ================================================
hinic3_pci_id_tbl.h       Supported device IDs
hinic3_hw_intf.h          Interface between HW and driver
hinic3_queue_common.[ch]  Common structures and methods for NIC queues
hinic3_common.[ch]        Encapsulation of memory operations in Linux
hinic3_csr.h              Register definitions in the BAR
hinic3_hwif.[ch]          Interface for BAR
hinic3_eqs.[ch]           Interface for AEQs and CEQs
hinic3_mbox.[ch]          Interface for mailbox
hinic3_mgmt.[ch]          Management interface based on mailbox and AEQ
hinic3_wq.[ch]            Work queue data structures and interface
hinic3_cmdq.[ch]          Command queue is used to post command to HW
hinic3_hwdev.[ch]         HW structures and methods abstractions
hinic3_lld.[ch]           Auxiliary driver adaptation layer
hinic3_hw_comm.[ch]       Interface for common HW operations
hinic3_mgmt_interface.h   Interface between firmware and driver
hinic3_hw_cfg.[ch]        Interface for HW configuration
hinic3_irq.c              Interrupt request
hinic3_netdev_ops.c       Operations registered to Linux kernel stack
hinic3_nic_dev.h          NIC structures and methods abstractions
hinic3_main.c             Main Linux kernel driver
hinic3_nic_cfg.[ch]       NIC service configuration
hinic3_nic_io.[ch]        Management plane interface for TX and RX
hinic3_rss.[ch]           Interface for Receive Side Scaling (RSS)
hinic3_rx.[ch]            Interface for transmit
hinic3_tx.[ch]            Interface for receive
hinic3_ethtool.c          Interface for ethtool operations (ops)
hinic3_filter.c           Interface for MAC address
========================  ================================================

## 管理接口

### 异步事件队列（AEQ
AEQ 通过一个描述符队列从硬件接收高优先级事件。每个描述符固定大小64 字节。AEQ 可以接收主动（solicited）或被动（unsolicited）事件。每个设备（VF PF）最多可以有 4 AEQ。每AEQ 关联一个专用的 IRQ。AEQ 可以接收多种类型的事件，但在实践hinic3 驱动忽略2 个邮箱相关事件之外的所有事件
### 邮箱（Mailbox
邮箱hinic3 驱动与硬件之间的一种通信机制。每个设备有一个独立的邮箱。驱动可以使用邮箱向管理平面发送请求。驱动通过 AEQ（使用事HINIC3_AEQ_FOR_MBOX）接收邮箱消息，例如对请求的响应。由于邮箱数据寄存器的大小有限，邮箱消息是分段发送的
每个设备都可以使用其邮箱向固件发送请求。邮箱也可用于在 PF 和它VFs 之间发送请求和响应
### 完成事件队列（CEQ
CEQ 的实现与 AEQ 相同。它通过一个固定大小2 位的描述符从硬件接收完成事件。每个设备最多可以有 32 CEQ。每CEQ 有一个专用的 IRQ。CEQ 只接收主动（solicited）事件，这些事件是对驱动请求的响应。CEQ 可以接收多种类型的事件，但在实践hinic3 驱动忽略HINIC3_CMDQ 之外的所有事件，HINIC3_CMDQ 表示先前cmdq 上发布的命令已完成
### 命令队列（cmdq
每个 cmdq 有一个专用的工作队列，命令发布在其上。工作队列上的命令是固定大小4 字节的描述符。命令的完成将通过承载该命令的描述符中ctrl 位来指示。命令完成的通知也会通过 CEQ 上的事件提供。每个设备有 4 个命令队列，它们作为一组（称为 cmdqs）初始化，每个队列有自己的类型。Hinic3 驱动只使用类HINIC3_CMDQ_SYNC
### 工作队列（WQ
工作队列是固定大WQE 的逻辑数组。该数组可以通过间接表分布到多个不连续的页上。工作队列被 I/O 队列和命令队列使用
### 全局功能 ID

每个功能（PF VF）在设备内有一个唯一的顺序标识。许多管理命令（mbox cmdq）都包含这个 ID，以便硬件将命令效果应用到正确的功能上
PF 被允许通过指定 VF ID 向从VF 发送管理命令。VF 必须提供它自己的 ID。硬件中的防欺骗机制会导致来VF 的命令在其包含错ID 时失败