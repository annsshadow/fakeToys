## Linux NVMe 澶氳矾寰。

本文档描述了 NVMe 多路径及其由 Linux NVMe 主机驱动支持的路径选择策略

## 简

Linux 中的 NVMe 多路径特性将具有相同标识符的命名空间（namespace）整合到单个块设备中使用多路径可增强 I/O 访问的可靠性与稳定性，同时提升带宽性能。当用户向该合并后的块设发I/O 时，多路径机制会根据所配置的策略选择其中一个底层块设备（路径）。不同的策略会导不同的路径选择

## 策略


所有策略都遵循 ANA（Asymmetric Namespace Access，非对称命名空间访问）机制，这意味着当存优化路径时，将优先选择它而非非优化路径。当NVMe 多路径策略包numa（默认）、round-robin
鍜?queue-depth銆。
要设置所需的策略（例如 round-robin），可使用以下方法之一   1. echo -n "round-robin" > /sys/module/nvme_core/parameters/iopolicy
   2. 或将 "nvme_core.iopolicy=round-robin" 添加cmdline

### NUMA


NUMA 策略I/O 分布选择距离当前 CPU NUMA 节点最近路径。该策略根据网络接口连接维护到每NUMA 节点的最近路径
何时使用 NUMA 策略  1. 多核系统：在多核和多处理器系统中优化内存访问，特别是NUMA 架构下  2. 高亲和性工作负载：I/O 处理绑定CPU，以减少跨节点的通信与数据传输延迟

### Round-Robin


round-robin 策略I/O 请求均匀分配到所有路径上，以提升吞吐量和资源利用率。每I/O 操作
按顺序发送到下一个路径
何时使用 round-robin 策略  1. 均衡工作负载：对于具有相I/O 大小和类型的均衡、可预测工作负载有效  2. 同质路径性能：当性能特征（例如延迟、带宽）相似时，高效利用所有路径

### Queue-Depth


queue-depth 策略根据每个路径的当前队列深度管I/O 请求，选择飞行中（in-flight）I/O 数量
最少的路径
何时使用 queue-depth 策略  1. 高负载且I/O：当负载较高、且 I/O 操作由相对固定大小的小请求组成时，有效地在各路径     平衡负载