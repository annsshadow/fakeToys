## zswap


## Overview


Zswap 是一个用于交换页的轻量级压缩缓存。它接收正在被换出的页，并尝试将其压缩到一个
动态分配的、基于 RAM 的内存池中。zswap 本质上是用 CPU 周期来换取潜在减少的交换 I/O。
如果来自压缩缓存的读取比来自交换设备的读取更快，这种权衡也可能带来显著的性能提升。

一些潜在的好处：

- 内存容量有限的桌面/笔记本用户可以减轻交换带来的性能影响。
- 共享同一 I/O 资源的过度承诺（overcommitted）客户机可以显著减少其交换 I/O 压力，
  避免由 hypervisor 进行的粗暴 I/O 限流。这样可以在对客户机工作负载以及共享 I/O
  子系统的其他客户机影响更小的情况下完成更多工作。
- 使用 SSD 作为交换设备的用户可以通过大幅减少折寿写入来延长设备寿命。

当压缩池达到其大小上限时，zswap 会以 LRU 方式将页从压缩缓存淘汰到后端交换设备。
这一需求在先前社区讨论中已被确认。

zswap 在启动时是否启用，取决于 `CONFIG_ZSWAP_DEFAULT_ON` Kconfig 选项是否开启。
随后可以通过提供内核命令行 `zswap.enabled=` 选项来覆盖该设置，例如 `zswap.enabled=0`。
zswap 也可以在运行时通过 sysfs 接口启用和禁用。一个在运行时启用 zswap 的示例命令
（假设 sysfs 已挂载）：
```

	echo 1 > /sys/module/zswap/parameters/enabled

```
当 zswap 在运行时被禁用时，它将停止存储正在被换出的页。但是，它_不会_立即将压缩池中
存储的所有页写回或 fault 回内存。存储在 zswap 中的页将一直保留在压缩池中，直到它们
被失效或 fault 回内存。为了强制将所有页移出压缩池，对交换设备执行 swapoff 会将所有
已换出的页（包括压缩池中的那些）fault 回内存。

## Design


zswap 从交换子系统接收待压缩的页，并能够在压缩池满时将自身的压缩页以 LRU 方式淘汰，
写回到后端交换设备。

zswap 使用 zsmalloc 来管理压缩内存池。zsmalloc 中的每次分配都不能直接通过地址访问。
相反，分配例程会返回一个句柄（handle），该句柄必须先被映射才能被访问。压缩内存池按需
增长，并随压缩页被释放而收缩。该内存池不是预先分配的。

当交换页从 swapout 传递给 zswap 时，zswap 维护一个从交换条目（swap type 与 swap offset
的组合）到引用该压缩交换页的 zsmalloc 句柄的映射。这一映射通过每个 swap type 一个的
xarray 实现。swap offset 是 xarray 节点的搜索键。

在对一个作为交换条目的 PTE 发生 page fault 期间，swapin 代码调用 zswap 的 load 函数，
将该页解压到由 page fault 处理程序分配的页中。

一旦没有任何 PTE 引用存储在 zswap 中的某个交换页（即 swap_map 中的计数变为 0），交换
代码就会调用 zswap 的 invalidate 函数以释放该压缩条目。

zswap 力求在策略上保持简单。sysfs 属性允许一种由用户控制的策略：

- max_pool_percent - 压缩池可以占用的内存的最大百分比。

默认的压缩器在 `CONFIG_ZSWAP_COMPRESSOR_DEFAULT` Kconfig 选项中选定，但可以在启动时
通过设置 `compressor` 属性来覆盖，例如 `zswap.compressor=lzo`。
它也可以在运行时通过 sysfs 的 "compressor" 属性更改：
```

	echo lzo > /sys/module/zswap/parameters/compressor

```
当在运行时更改压缩器参数时，任何已有的压缩页都不会被修改；它们保留在各自的池中。当请求
一个旧池中的页时，会使用其原始的压缩器进行解压。一旦某个旧池中的所有页都被移除，该池
及其压缩器就会被释放。

zswap 中的部分页是等值填充页（即页的内容具有相同值或重复模式）。这些页包含全零填充页，
并且它们被区别对待。在存储操作中，页在被压缩之前会先被检查是否为等值填充页。如果是，
该页的压缩长度被设为零，并存储该模式或等值填充值。

为了防止在 zswap 已满且交换压力很高时 zswap 收缩内存池（这将导致页在 zswap 池中反复
换入换出而没有任何实际收益，却会让系统性能下降），引入了一个特殊参数来实现一种迟滞
（hysteresis）机制：在达到上限后，拒绝接收页进入 zswap 池，直到其拥有足够空间为止。
要设置在 zswap 变满后重新开始接收页的阈值，请使用 sysfs 的 `accept_threshold_percent`
```

	echo 80 > /sys/module/zswap/parameters/accept_threshold_percent

```
将此参数设为 100 将禁用该迟滞机制。

一些用户无法忍受 zswap 存储失败和 zswap 回写所带来的交换。可以完全禁用交换（而无需
禁用
```

	echo 0 > /sys/fs/cgroup/<cgroup-name>/memory.zswap.writeback

```
注意，如果存储失败反复出现（例如页面不可压缩），用户在禁用回写后可能会观察到回收
效率低下（因为相同的页可能会被反复拒绝）。

当 zswap 池中存在大量冷内存时，主动将这些冷页写入交换并回收内存供其他用例使用可能是
有利的。默认情况下，zswap 的 shrinker 是禁用的。
```

  echo Y > /sys/module/zswap/parameters/shrinker_enabled

```
如果选定了 `CONFIG_ZSWAP_SHRINKER_DEFAULT_ON`，这可以在启动时启用。

提供了一个 debugfs 接口，用于获取关于池大小、已存储页数量、等值填充页，以及页面被拒绝
各种原因的各种计数器的统计信息。
