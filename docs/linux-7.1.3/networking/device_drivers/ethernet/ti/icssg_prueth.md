
## 德州仪器 ICSSG PRUETH 以太网驱

:Version: 1.0

## ICSSG 固件


每个 ICSSG 核心有两个可编程实时单元（PRU）、两个辅助实时传输单元（RTU）以及两个发送实时传输单元（TX_PRU）。其中每一个都运行各自的固件。这些固件合起来被称ICSSG 固件
## 固件统计


ICSSG 固件维护某些统计信息，由驱动通过 `ethtool -S <interface>` 导出
这些统计信息如下
 - `FW_RTU_PKT_DROP`：诊断错误计数器，当 RTU 因端口被禁用或规则违例而丢弃本地注入的数据包时递增 - `FW_Q0_OVERFLOW`：队0 TX 溢出计数 - `FW_Q1_OVERFLOW`：队1 TX 溢出计数 - `FW_Q2_OVERFLOW`：队2 TX 溢出计数 - `FW_Q3_OVERFLOW`：队3 TX 溢出计数 - `FW_Q4_OVERFLOW`：队4 TX 溢出计数 - `FW_Q5_OVERFLOW`：队5 TX 溢出计数 - `FW_Q6_OVERFLOW`：队6 TX 溢出计数 - `FW_Q7_OVERFLOW`：队7 TX 溢出计数 - `FW_DROPPED_PKT`：当数据包因规则违例PRU 处被丢弃时，该计数器递增 - `FW_RX_ERROR`：若 PRU 处发CRC 错误或最最大帧错误则递增
 - `FW_RX_DS_INVALID`：当 RTU 检测到数据状态无效条件时递增
 - `FW_TX_DROPPED_PACKET`：经TX 端口丢弃的数据包计数 - `FW_TX_TS_DROPPED_PACKET`：带TS 标志、经TX 端口丢弃的数据包计数 - `FW_INF_PORT_DISABLED`：因端口被禁用而丢RX 帧时递增
 - `FW_INF_SAV`：因源地址违例而丢RX 帧时递增
 - `FW_INF_SA_DL`：因源地址位于拒绝列表中而丢RX 帧时递增
 - `FW_INF_PORT_BLOCKED`：因端口被阻塞且帧为特殊帧而丢RX 帧时递增
 - `FW_INF_DROP_TAGGED`：因带标签而丢RX 帧时递增
 - `FW_INF_DROP_PRIOTAGGED`：因带优先级标签而丢RX 帧时递增
 - `FW_INF_DROP_NOTAG`：因无标签而丢RX 帧时递增
 - `FW_INF_DROP_NOTMEMBER`：因端口不是 VLAN 成员而丢RX 帧时递增
 - `FW_RX_EOF_SHORT_FRMERR`：在未看RX_B1 的情况下调度帧结束（EOF）任务时递增
 - `FW_RX_B0_DROP_EARLY_EOF`：因提前 EOF 而丢弃帧时递增
 - `FW_TX_JUMBO_FRM_CUTOFF`：为防止数据包大小超2000 字节而对帧进行截断时递增
 - `FW_RX_EXP_FRAG_Q_DROP`：当快速帧在前一片段所在的同一队列中被接收时递增
 - `FW_RX_FIFO_OVERRUN`：RX fifo 溢出计数 - `FW_CUT_THR_PKT`：使用直通（Cut-Through）转发方法转发数据包时递增
 - `FW_HOST_RX_PKT_CNT`：Rx PRU 通过 PSI 发送给主机的有效数据包数量
 - `FW_HOST_TX_PKT_CNT`：RTU0 复制Tx 队列的有效数据包数量
 - `FW_HOST_EGRESS_Q_PRE_OVERFLOW`：主机出口队列（可抢占）溢出计数 - `FW_HOST_EGRESS_Q_EXP_OVERFLOW`：主机出口队列（可抢占）溢出计数