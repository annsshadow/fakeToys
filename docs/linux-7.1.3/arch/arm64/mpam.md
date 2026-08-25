
## MPAM


## 什么是 MPAM


MPAM（Memory Partitioning and Monitoring，内存分区与监控）是 CPU 与内存系统组件（如缓存或内存控制器）中的一项特性，它允许对内存流量进行标记、分区和监控

流量CPU 根据当前任务通过 resctrl 分配到的控制或监控组进行标记。分区策略可以使resctrl 中的 schemata 文件设置，监控值通过 resctrl 读取。更多细节请参阅 Documentation/filesystems/resctrl.rst

这使得共享内存系统资源（如缓存）的任务能够根据分区策略（即所谓的“吵闹邻居”）彼此隔离

## 支持的平


使用该特性需CPU 支持、内存系统组件中的支持，以及固件MPAM 设备控制所MMIO 地址空间位置的描述（例如 'MPAM' ACPI 表）

为内存系统组件提MPAM 控制/监控MMIO 设备称为内存系统组件（MSC）

由于 MPAM 的用户接口是通过 resctrl 提供的，因此只有resctrl 兼容MPAM 特性才能暴露给用户空间

MSC 基于拓扑结构被视为一个组。与 L3 缓存对应MSC 被归在一起，不能L2 L3 之间混合 MSC 来“覆盖”一resctrl schema

支持的特性如下：

- L2 L3 缓存上的缓存部分位图控制（CPOR）。要L2 L3 暴露 CPOR，每CPU 必须拥有相应级别且同样支持该特性的 CPU 缓存。大小核不匹配的平台不受支持，因为此resctrl 的控制也会依赖于任务的放置

- L3 缓存上或之后的内存带宽最大控制（MBW_MAX）。resctrl 使用 L3 缓存 id 来标识内存带宽控制的应用位置。因此平台必须拥有由固件提供的带cache-id L3 缓存。（它不需要支MPAM。）

  要作'MB' schema 导出，所MSC 组的拓扑必须L3 缓存的拓扑匹配，以便可以重绘 cache-id。例如：在无CPU NUMA 节点上带有内存带宽最大控制的平台无法'MB' schema 暴露resctrl，因为这些节点没有对应的 L3 缓存。如果内存带宽控制位于内存而非 L3 上，则必须有一个单一的全局 L3，否则无法确定流量来自哪L3。在 L3 与内存之间必须没有缓存，这样路径两端才有等价的流量

  MPAM 驱动发现多个可用'MB' schema MSC 组时，它会优先选择最接近 L3 缓存的组

- 缓存存储用量（CSU）计数器可以暴露 'llc_occupancy'，前提是构成 L3 组的每个 MSC 上至少有一CSU 监控器。暴露来自其他缓存或设备CSU 计数器不受支持

## 报告缺陷


如果你没有看到预期的计数器或控制，请分享在启用动态调试并以如下参数引导时产生的调试信息：
dyndbg="file mpam_resctrl.c +pl"
