
## SRAT - 静态资源亲和性表（Static Resource Affinity Table）


系统/静态资源亲和性表描述资源（CPU、内存）到“邻近域（Proximity Domains）”的亲和性。该表在技术上是可选的，但要让 Linux 枚举出性能信息（参见 “HMAT”），则必须存在该表。

CEDT 与 SRAT 表之间，以及 NUMA 节点如何被创建，存在着微妙的配合关系。如果结果与你的预期不太一致，请检查 SRAT 的内存亲和性表项与 CEDT CFMWS，以确定你的平台在灵活拓扑方面实际支持什么。

SRAT 可以静态地将 CFMWS SPA 范围的一部分分配给特定的邻近域。有关这在 NUMA 拓扑中如何呈现的更多信息，请参阅 Linux NUMA 创建相关内容。

## 邻近域（Proximity Domain）

邻近域大致相当于“NUMA 节点（NUMA Node）”，但并不保证是一对一映射。例如，存在“邻近域 4”映射到“NUMA 节点 3”的场景。（参见 “NUMA 节点创建”）

## 内存亲和性（Memory Affinity）

一般来说，如果主机在 BIOS 中对任何 CXL 结构（解码器）进行了编程，那么就需要存在针对该内存的 SRAT 表项。

```

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000001          <- NUMA Node 1
             Reserved1 : 0000
          Base Address : 000000C050000000  <- Physical Memory Region
        Address Length : 0000003CA0000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
              Enabled : 1
        Hot Pluggable : 1
         Non-Volatile : 0


```
## 通用端口亲和性（Generic Port Affinity）

通用端口亲和性（Generic Port Affinity）子表提供了邻近域与代表通用端口（例如 CXL 主机桥）的设备句柄之间的关联。借助该关联，可以从 SRAT 中检索 CPU（发起方）与通用端口之间路径的延迟与带宽数值。这用于为热插拔的 CXL 设备构建性能坐标，这些设备无法在启动时由平台固件枚举。

```

         Subtable Type : 06 [Generic Port Affinity]
                Length : 20               <- 32d, length of table
              Reserved : 00
    Device Handle Type : 00               <- 0 - ACPI, 1 - PCI
      Proximity Domain : 00000001
         Device Handle : ACPI0016:01
                 Flags : 00000001         <- Bit 0 (Enabled)
              Reserved : 00000000

```
邻近域与 [HMAT <hmat>](HMAT <hmat>) SSLBI 目标邻近域列表相匹配，以获取相关的延迟或带宽数值。这些性能数值通过设备句柄关联到某个 CXL 主机桥。驱动使用该关联来检索通用端口性能数值，用于整个 CXL 路径访问坐标的计算。
