## 显式的易失写回缓存控制


### 简介


许多存储设备，尤其是消费级市场的设备，带有易失（volatile）写回缓存。这意味着设备在数据
真正写入非易失存储之前，就向操作系统报告 I/O 完成。这种行为显然加速了各种工作负载，但
意味着操作系统在执行数据完整性操作（如 fsync、sync 或卸载）时，需要强制将数据写到非易失
存储上。

Linux 块层提供了两个简单机制，让文件系统能够控制存储设备的缓存行为。这两个机制分别是
强制缓存刷新，以及请求上的 Force Unit Access（FUA）标志。


### 显式缓存刷新


REQ_PREFLUSH 标志可以通过按位或（OR）加入到由文件系统提交的 bio 的读/写标志中，从而确保
存储设备的易失缓存在该实际 I/O 操作开始之前已被刷新。这明确保证了在带标志的 bio 启动之前，
先前已完成的写请求都已位于非易失存储上。此外，REQ_PREFLUSH 标志可以设置在一个原本为空的
bio 结构上，这只会引发一次显式的缓存刷新而没有任何依赖的 I/O。对于纯粹的缓存刷新，建议
使用 blkdev_issue_flush() 辅助函数。


### Forced Unit Access（强制单元访问）


REQ_FUA 标志可以通过按位或加入到由文件系统提交的 bio 的读/写标志中，从而确保只有在数据已被
提交到非易失存储后，才会报告该请求的 I/O 完成。


### 文件系统的实现细节


文件系统只需简单地设置 REQ_PREFLUSH 与 REQ_FUA 位即可，无需担心底层设备是否需要任何显式
缓存刷新，也无需关心 Forced Unit Access 是如何实现的。REQ_PREFLUSH 与 REQ_FUA 标志可以
同时设置在一个 bio 上。

### 块驱动的特性设置


对于不支持易失写缓存的设备，不需要任何驱动支持；块层在进入驱动之前完成空的 REQ_PREFLUSH
请求，并从带有有效载荷的请求中剥离 REQ_PREFLUSH 与 REQ_FUA 位。

对于带有易失写缓存的设备，驱动需要告诉块层它支持刷新缓存，方法是在 queue_limits 的 feature
字段中设置

   BLK_FEAT_WRITE_CACHE

标志。对于同时支持 FUA 位的设备，还需要通过设置 queue_limits 结构的 features 字段中的

   BLK_FEAT_FUA

标志，告知块层传递 REQ_FUA 位。

### 基于 bio 的块驱动的实现细节


对于基于 bio 的驱动，如果驱动设置了 BLK_FEAT_WRITE_CACHE 标志，则 REQ_PREFLUSH 与 REQ_FUA
位会被简单地传递给驱动，由驱动负责处理。

**注意**：即使未设置 BLK_FEAT_FUA 标志，REQ_FUA 位也会被传递。任何设置了 BLK_FEAT_WRITE_CACHE
的基于 bio 的驱动也必须处理 REQ_FUA。

对于重映射（remapping）驱动，REQ_FUA 位需要传播到下层设备，并且需要为带有 REQ_PREFLUSH 位的
bio 实现一次全局刷新。

### blk-mq 驱动的实现细节


当设置了 BLK_FEAT_WRITE_CACHE 标志时，带有有效载荷的 REQ_OP_WRITE | REQ_PREFLUSH 请求会被
块层自动转换为一个 REQ_OP_FLUSH 请求后跟实际写操作的序列。

当设置了 BLK_FEAT_FUA 标志时，REQ_FUA 位会被简单地传递给 REQ_OP_WRITE 请求；否则，对于设置了
REQ_FUA 位的 bio 提交，块层会在写请求完成后发送一个 REQ_OP_FLUSH 请求。
