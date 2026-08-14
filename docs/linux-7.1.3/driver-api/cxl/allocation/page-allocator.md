
## 页分配器（The Page Allocator）


内核页分配器处理所有通用的页分配请求，例如 `kmalloc`。CXL 配置步骤会根据所选的 `Memory Zone`（内存区）与容量所在的 `NUMA node`（NUMA 节点）影响页分配器的行为。

本节主要关注这些配置（截至 Linux v6.15）如何影响页分配器，而非页分配器的整体行为。

## NUMA 节点与 mempolicy


除非任务显式注册了 mempolicy，否则 Linux 内核的默认内存策略是优先从 `local NUMA node`（本地 NUMA 节点）分配内存，仅当本地节点受压时才回退到其他节点。

通常，我们期望在独立的 NUMA 节点上看到本地 DRAM 与 CXL 内存，其中 CXL 内存是非本地的。不过从技术上讲，计算节点有可能没有本地 DRAM，而 CXL 内存成为该计算节点的 `local`（本地）容量。

## 内存区（Memory Zones）


CXL 容量可以在 `ZONE_NORMAL` 或 `ZONE_MOVABLE` 中上线（online）。

截至 v6.15，页分配器优先尝试从本地节点上最高可用且兼容的 ZONE 进行分配。

`zone 不兼容`（zone incompatibility）的一个例子是尝试从 `ZONE_MOVABLE` 服务于标记为 `GFP_KERNEL` 的分配。内核分配通常是不可迁移的，因此只能从 `ZONE_NORMAL` 或更低的区域服务。

为简化这一点，默认情况下页分配器会优先选择 `ZONE_MOVABLE` 而非 `ZONE_NORMAL`，但如果 `ZONE_MOVABLE` 耗尽，它会回退到从 `ZONE_NORMAL` 分配。

## CGroups 与 CPUSets


最后，假设 CXL 内存可通过页分配到达（即已在 `ZONE_NORMAL` 中上线），则 `cpusets.mems_allowed` 可被容器用来限制该容器中任务对某些 NUMA 节点的可访问性。用户可能希望在多租户系统中利用这一点，其中某些任务不希望使用较慢的内存。

在回收（reclaim）一节中，我们将讨论此接口的一些限制，以防止共享数据被降级（demotion）到 CXL 内存（如果启用了降级）。
