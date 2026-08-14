
## PCI vNTB 功能（Function）

本文介绍 PCI 端点（Endpoint）子系统下的虚拟 NTB 功能（vNTB Function），说明其与标准 PCI NTB 的区别、实现所用的关键结构（配置区、便签寄存器、门铃、内存窗口等）及其工作原理，供 PCIe 端点驱动开发者参考。


:Author: Frank Li <Frank.Li@nxp.com>

PCI NTB 功能与 PCI vNTB 功能的区别在于：

PCI NTB 功能需要两个端点（endpoint）实例，连接 HOST1 与 HOST2。

PCI vNTB 功能只使用一个主机（host）与一个端点（EP），使用 NTB 连接 EP 与 PCI 主机



  +------------+         +---------------------------------------+
  |            |         |                                       |
  +------------+         |                        +--------------+
  | NTB        |         |                        | NTB          |
  | NetDev     |         |                        | NetDev       |
  +------------+         |                        +--------------+
  | NTB        |         |                        | NTB          |
  | Transfer   |         |                        | Transfer     |
  +------------+         |                        +--------------+
  |            |         |                        |              |
  |  PCI NTB   |         |                        |              |
  |    EPF     |         |                        |              |
  |   Driver   |         |                        | PCI Virtual  |
  |            |         +---------------+        | NTB Driver   |
  |            |         | PCI EP NTB    |<------>|              |
  |            |         |  FN Driver    |        |              |
  +------------+         +---------------+        +--------------+
  |            |         |               |        |              |
  |  PCI BUS   | <-----> |  PCI EP BUS   |        |  Virtual PCI |
  |            |  PCI    |               |        |     BUS      |
  +------------+         +---------------+--------+--------------+
      PCI RC                        PCI EP

## 用于实现 vNTB 的结构（Constructs used for Implementing vNTB）


 1) 配置区（Config Region）
 2) 自身便签寄存器（Self Scratchpad Registers）
 3) 对端便签寄存器（Peer Scratchpad Registers）
 4) 门铃（Doorbell，DB）寄存器
 5) 内存窗口（Memory Window，MW）


### 配置区（Config Region）：


与 PCI NTB Function 驱动相同

### 便签寄存器（Scratchpad Registers）：


它附加在配置区之后。



  +--------------------------------------------------+ Base
  |                                                  |
  |                                                  |
  |                                                  |
  |          Common Config Register                  |
  |                                                  |
  |                                                  |
  |                                                  |
  +-----------------------+--------------------------+ Base + span_offset
  |                       |                          |
  |    Peer Span Space    |    Span Space            |
  |                       |                          |
  |                       |                          |
  +-----------------------+--------------------------+ Base + span_offset
  |                       |                          |      + span_count * 4
  |                       |                          |
  |     Span Space        |   Peer Span Space        |
  |                       |                          |
  +-----------------------+--------------------------+
        Virtual PCI             Pcie Endpoint
        NTB Driver               NTB Driver


### 门铃寄存器（Doorbell Registers）：


门铃寄存器由主机用来互相中断。

### 内存窗口（Memory Window）：


两个主机之间的实际数据传输将通过内存窗口进行。

## 建模结构（Modeling Constructs）：


32 位 BAR。

======  ===============
BAR NO  CONSTRUCTS USED
======  ===============
BAR0    配置区（Config Region）
BAR1    门铃（Doorbell）
BAR2    内存窗口 1（Memory Window 1）
BAR3    内存窗口 2（Memory Window 2）
BAR4    内存窗口 3（Memory Window 3）
BAR5    内存窗口 4（Memory Window 4）
======  ===============

64 位 BAR。

======  ===============================
BAR NO  CONSTRUCTS USED
======  ===============================
BAR0    配置区（Config Region） + 便签寄存器（Scratchpad）
BAR1
BAR2    门铃（Doorbell）
BAR3
BAR4    内存窗口 1（Memory Window 1）
BAR5
======  ===============================
