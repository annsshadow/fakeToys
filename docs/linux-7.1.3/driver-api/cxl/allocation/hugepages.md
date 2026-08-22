## 大页（Huge Pages


## 连续内存分配器（Contiguous Memory Allocator


在早期启动阶段作SystemRAM 上线CXL 内存可用CMA，因为在 CMA 划分出连续容量时，承载该容量NUMA 节点处于 `Online` 状态

延迟CXL 驱动进行配置CXL 内存，其容量无法CMA 分配——因为在 CMA 划分出连续容量时（即 `__init` 时刻），承载该容量的 NUMA 节点处于 `Offline` 状态

## HugeTLB


不同的大页尺寸允许不同的内存配置

### 2MB 大页

无论配置时间或内存区域（zone）如何，所CXL 容量都可用于 2MB 大页

### 1GB 大页

`ZONE_NORMAL` 中上线的 CXL 容量可用1GB 巨型页（Gigantic Page）分配

`ZONE_MOVABLE` 中上线的 CXL 容量不能用于 1GB 巨型页分配
