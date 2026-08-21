## DAX 驱动操作


`Direct Access Device` 驱动最初设计用于为类内存块设备提供类内存的访问机制它被扩展以支CXL 内存设备，后者提供用户配置的内存设备
CXL 子系统依DAX 子系统来实现以下之一
- 通过 `/dev/daxN.Y` 生成面向用户空间的文件式接口，或
- 调用 memory-hotplug 接口CXL 内存加入页分配器
DAX 子系统通过 `cxl_dax_region` 驱动暴露此能力。`dax_region` 提供 CXL
`memory_region` `DAX Device` 之间的转换
## DAX 设备


`DAX Device` 是在 `/dev/daxN.Y` 中暴露的文件式接口。通过 DAX 设备暴露的内区域可由用户空间软件通过 `mmap()` 系统调用访问。结果是在任务的页表中直映射CXL 容量
希望手动处理 CXL 内存分配的用户应使用此接口
## kmem 转换


`dax_kmem` 驱动`DAX Device` 转换为由 `kernel/memory-hotplug.c` 管理的一
系列 `hotplug memory blocks`。此容量将在用户选择的内存区中暴露给内核页分配器
`memmap_on_memory` 设置（全局DAX 设备本地）决定了内核将从何处分配此内存的
`struct folio` 描述符。如果设置了 `memmap_on_memory`，内存热插拔将预留一部分
内存块容量来分配 folio。如果未设置，内存将通过正常`GFP_KERNEL` 分配——因很可能会落到执行热插拔操作的 CPU 的本NUMA 节点上