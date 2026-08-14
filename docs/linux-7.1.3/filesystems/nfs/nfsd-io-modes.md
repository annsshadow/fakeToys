
## NFSD IO 模式


## 概述


NFSD 在处理 READ 和 WRITE 操作时，历史上一直使用缓冲 IO。BUFFERED 是 NFSD 的默认
IO 模式，但可以将该默认值覆盖为使用 DONTCACHE 或 DIRECT IO 模式。

提供了实验性的 NFSD debugfs 接口，允许独立配置用于 READ 和 WRITE 的 NFSD IO 模式。
请参见：

- /sys/kernel/debug/nfsd/io_cache_read
- /sys/kernel/debug/nfsd/io_cache_write

io_cache_read 和 io_cache_write 的默认值反映了 NFSD 的默认 IO 模式（即
NFSD_IO_BUFFERED=0）。

根据配置的设置，NFSD 的 IO 将为以下之一：

- 使用页缓存缓存（NFSD_IO_BUFFERED=0）
- 缓存但在完成时从页缓存移除（NFSD_IO_DONTCACHE=1）
- 不缓存 stable_how=NFS_UNSTABLE（NFSD_IO_DIRECT=2）

要设置 NFSD IO 模式，向
```

  echo 2 > /sys/kernel/debug/nfsd/io_cache_read
  echo 2 > /sys/kernel/debug/nfsd/io_cache_write

```
要检查 NFSD 对 READ 或 WRITE 正在使用哪个 IO 模式，只需读取
```

  cat /sys/kernel/debug/nfsd/io_cache_read
  cat /sys/kernel/debug/nfsd/io_cache_write

```
如果你在近期内核上试验 NFSD 的 IO 模式并得到了有趣的结果，请将其报告到
linux-nfs@vger.kernel.org

## NFSD DONTCACHE


DONTCACHE 提供了一种处理 IO 的混合方法，旨在提供使用 DIRECT IO 的好处，而不带来
DIRECT IO 所施加的任何严格对齐要求。为此，它使用缓冲 IO，但 IO 被标记为“落后丢弃”
（即相关联的页在 IO 完成时从页缓存中丢弃）。

DONTCACHE 旨在避免 Linux 内存管理子系统已被证明相当显著的一个限制：当/如果大量
数据被不频繁访问时（例如只读取一次_或_只写入一次、但很久之后才读取）。这类用例
尤其成问题，因为页缓存最终会成为服务新 IO 请求的瓶颈。

关于 DONTCACHE 的更多背景，请参阅这些 Linux 提交说明：

- 概述：  9ad6344568cc3 ("mm/filemap: change filemap_create_folio()
  to take a struct kiocb")
- 用于 READ：  8026e49bff9b1 ("mm/filemap: add read support for
  RWF_DONTCACHE")
- 用于 WRITE： 974c5e6139db3 ("xfs: flag as supporting FOP_DONTCACHE")

如果底层文件系统没有通过设置 FOP_DONTCACHE 来表明支持，NFSD_IO_DONTCACHE 将回退到
NFSD_IO_BUFFERED。

## NFSD DIRECT


DIRECT IO 不使用页缓存，因此它能够避免 Linux 内存管理的页回收（page reclaim）可扩展性
问题，而无需像 DONTCACHE 那样混合使用页缓存。

一些工作负载受益于 NFSD 避开页缓存，特别是那些工作集显著大于可用系统内存的负载。
NFSD DIRECT 被证明帮助最大的病态最坏情况工作负载是：NFS 客户端对一个大小为 NFS 服务器
可用系统内存 2-3 倍的文件发起大型顺序 IO。这种改进的原因在于 NFSD DIRECT 消除了内存
管理子系统原本需要执行的许多工作（例如页分配、脏页回写、页回收）。使用 NFSD DIRECT
时，kswapd 和 kcompactd 不再占据 CPU 时间去寻找足够的空闲页以推进 IO。

与 NFSD DIRECT 相关的性能提升此前在 linux-nfs 上讨论过，参见：
https://lore.kernel.org/linux-nfs/aEslwqa9iMeZjjlV@kernel.org/

总结如下：

- NFSD DIRECT 可以显著减少内存需求
- NFSD DIRECT 可以通过避免代价高昂的页回收工作来降低 CPU 负载
- NFSD DIRECT 可以提供更具确定性的 IO 性能

一如既往，效果因人而异，因此仔细考虑是否/何时使用 NFSD DIRECT 有益很重要。在评估你的
工作负载的相对性能时，请务必在测试期间记录相关的性能指标（例如内存使用、CPU 使用、
IO 性能）。使用 perf 收集 perf 数据，用于生成 Linux 为你的测试所必须执行的工作的
“火焰图”，是一种真正有意义的方式来比较系统的相对健康状况，以及切换 NFSD 的 IO 模式
如何改变所观察到的情况。

如果通过向 NFSD 的 debugfs 接口写入 2（或用于 WRITE 的 3 和 4）来指定 NFSD_IO_DIRECT，
理想情况下 IO 应相对于底层块设备的 logical_block_size 对齐。此外，用于存储 READ 或
WRITE 载荷的内存缓冲区必须相对于底层块设备的 dma_alignment 对齐。

但 NFSD DIRECT 在 O_DIRECT 的意义上会尽最大努力处理未对齐的 IO：

未对齐的 READ：
    如果使用 NFSD_IO_DIRECT，将任何未对齐的 READ 扩展到下一个 DIO 对齐的块（在 READ
    的两端）。扩展后的 READ 会校验具有正确的 offset/len（logical_block_size）以及
    dma_alignment 检查。

未对齐的 WRITE：
    如果使用 NFSD_IO_DIRECT，按需将任何未对齐的 WRITE 拆分为起始、中间和结尾。较大的
    中间段是 DIO 对齐的，而起始和/或结尾是未对齐的。对未对齐的段使用缓冲 IO，对中间
    DIO 对齐的段使用 O_DIRECT。未对齐的段_不_使用 DONTCACHE 缓冲 IO，因为使用普通缓冲
    IO 在处理流式未对齐 WRITE 时具有显著的 RMW 性能优势。

跟踪：
    nfsd_read_direct 跟踪事件展示了 NFSD 如何将任何未对齐的 READ 扩展到下一个 DIO 对齐
    的块（在原始 READ 的两端，按需）。

```

      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_vector/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_direct/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_read_io_done/enable
      echo 1 > /sys/kernel/tracing/events/xfs/xfs_file_direct_read/enable

    nfsd_write_direct 跟踪事件展示了 NFSD 如何将给定未对齐的 WRITE 拆分为一个 DIO 对齐
    的中间段。

    这一组合跟踪事件对 WRITE 很有用::

      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_opened/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_direct/enable
      echo 1 > /sys/kernel/tracing/events/nfsd/nfsd_write_io_done/enable
      echo 1 > /sys/kernel/tracing/events/xfs/xfs_file_direct_write/enable

```
