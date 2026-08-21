
## 回收（Reclaim

CXL 内存*间接**方式被利用的另一种途径是通过 `mm/vmscan.c` 中的回收系统。当系统内存容量因全局cgroup 局部的 `watermark` 设置而受到压力时，就会触发回收
本节我们不会讨论 `watermark` 配置，而只说明回收系统的各个部分是如何消费 CXL 内存的
## 降级（Demotion

默认情况下，回收系统在回收内存时会优先使swap（或 zswap）。启`kernel/mm/numa/demotion_enabled` 后，如果容量允许，vmscan opportunistic 地优先选择远端 NUMA 节点而非 swap zswap
降级会动`mm/memory_tier.c` 组件来确定下一个降级节点。下一个降级节点基`HMAT` `CDAT` 性能数据
### cpusets.mems_allowed 的怪异行为


Linux v6.15 及更早版本中，降级在迁移页面时不遵守 `cpusets.mems_allowed`。因此，如果启用了降级，vmscan 无法保证将容器的内存隔离mems_allowed 中未设置的节点之外
Linux v6.XX 及更高版本中，降级确实会尝试遵守 `cpusets.mems_allowed`；但是，某些由其cgroup 最初实例化的共享内存类别（例如公共库——如 libc）仍可能被降级。因此，mems_allowed 接口仍然无法提供与远端节点的完美隔离
## ZSwap 与节点偏

Linux v6.15 及更早版本中，ZSwap 为新压缩的页面从处理器的本地节点分配内存。由于被压缩的页面通常是冷页面，结果是冷页面被提升（promoted）——随后又随着其从 LRU 老化而被降级
Linux v6.XX 中，ZSwap 会尝试将正在被压缩的页面所属节点作为压缩页的分配目标。这有助于防止颠簸（thrashing）
## 结合 ZSwap 的降

当同时启用降级和 ZSwap 时，你会制造这样一种情况：默认情况ZSwap 会优先使用最慢的那一CXL 内存，直到该级内存耗尽