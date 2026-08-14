
## 内存热插拔


将 CXL 内存呈现给内核页分配器的最后阶段，是由 `DAX` 驱动通过内存热插拔组件呈现一个 `Driver Managed` 内存区域。

需要考虑四种主要配置：

1) 默认上线行为（开/关与 zone）
2) 热插拔内存块大小
3) 内存映射资源位置
4) 驱动托管内存的指定

## 默认上线行为


热插拔内存的默认上线行为由以下各项按优先级顺序决定：

- `CONFIG_MHP_DEFAULT_ONLINE_TYPE` 构建配置
- `memhp_default_state` 引导参数
- `/sys/devices/system/memory/auto_online_blocks` 的值

这些决定了热插拔的内存块以三种状态之一到达：

1) 离线（Offline）
2) 在 `ZONE_NORMAL` 中上线
3) 在 `ZONE_MOVABLE` 中上线

`ZONE_NORMAL` 意味着该容量可用于几乎任何分配，而 `ZONE_MOVABLE` 意味着该容量只应用于可迁移的分配。

`ZONE_MOVABLE` 尝试保留内存块的热插拔能力，以便整个区域可以在稍后热拔插。任何上线到 `ZONE_NORMAL` 的容量应被视为永久附加到页分配器。

## 热插拔内存块大小


默认情况下，在大多数体系结构上，热插拔内存块大小是 128MB 或 256MB。在 x86 上，随着总内存容量超过 64GB，块大小会增加到最多 2GB。自 v6.15 起，Linux 在决定热插拔内存块大小时，未考虑 ACPI CEDT CFMWS 区域的大小与对齐（见 Early Boot 文档）。

## 内存映射


用于表示热插拔内存容量的 `struct folio` 分配位置由以下系统设置决定：

- `/sys_module/memory_hotplug/parameters/memmap_on_memory`
- `/sys/bus/dax/devices/daxN.Y/memmap_on_memory`

如果这两个参数都设置为 true，则用于该容量的 `struct folio` 将从正在上线的内存块中切分出来。如果内存延迟特别高且其 `struct folio` 变得激烈争用，这会带来性能影响。

如果任一参数设置为 false，则用于该容量的 `struct folio` 将从运行热插拔过程的本地处理器节点分配。由于这是一个 `GFP_KERNEL` 分配，该容量将从该节点上的 `ZONE_NORMAL` 分配。

具有极大量 `ZONE_MOVABLE` 内存（例如 CXL 内存池）的系统，必须确保有足够的本地 `ZONE_NORMAL` 容量来承载热插拔容量的内存映射。

## 驱动托管内存


DAX 驱动将此内存作为“Driver Managed”呈现给内存热插拔。这不是一个可配置的设置，但重要的是要注意，驱动托管内存在 kexec 期间被明确排除在使用之外。这是必需的，以确保 CXL 设备可能在功能性系统重启（如 probe 时重置）期间经受的任何复位或带外操作，不会导致 kexec 内核的部分被覆盖。
