## Qualcomm Datacenter Technologies L3 缓存性能监控单元（PMU）


该驱动支持 Qualcomm Datacenter Technologies Centriq SoC 中的 L3 缓存 PMU。
这些 SoC 上的 L3 缓存由多个切片组成，由插槽内的所有核心共享。每个切片作为
独立的非核 perf PMU 暴露，设备名为 l3cache_<socket>_<instance>。用户空间
负责跨切片聚合。

该驱动在 sysfs 中提供其可用事件与配置选项的描述，见
/sys/bus/event_source/devices/l3cache*。鉴于这些是非核 PMU，驱动还暴露一个
"cpumask" sysfs 属性，其中包含每个插槽一个 CPU 的掩码，将用于处理该插槽上的
所有 PMU 事件。

硬件实现 32 位事件计数器，并通过 "event" 格式属性暴露一个扁平的 8 位事件空间。
除了 32 位物理计数器外，驱动还通过使用硬件计数器链式连接支持虚拟 64 位硬件
计数器。该特性通过 "lc"（长计数器）格式暴露
```

  perf stat -e l3cache_0_0/read-miss,lc/

```
鉴于这些是非核 PMU，驱动不支持采样，因此 "perf record" 将无法工作。不支持
每任务的 perf 会话。
