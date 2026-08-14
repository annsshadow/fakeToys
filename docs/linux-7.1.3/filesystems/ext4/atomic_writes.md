
### 原子块写入（Atomic Block Writes）


#### 简介


原子（untorn，不撕裂）块写入确保整个写入要么全部提交到磁盘，要么都不提交。这可以防止在断电或系统崩溃期间出现 “torn writes（撕裂写入）”。ext4 文件系统支持在带有 extents 的常规文件上进行原子写入（仅限 Direct I/O），前提是底层存储设备支持硬件原子写入。这通过以下两种方式支持：

1. **单 fsblock 原子写入**：
   EXT4 自 v6.13 起支持以单个文件系统块为单位的原子写入操作。在此情况下，原子写入单元的最小和最大尺寸都设置为文件系统块大小。
   例如，在页面大小为 64KB 的系统上，使用 16KB 文件系统块大小进行 16KB 的原子写入是可行的。

2. **使用 Bigalloc 的多 fsblock 原子写入**：
   EXT4 现在也支持使用名为 bigalloc 的特性跨越多个文件系统块的原子写入。原子写入单元的最小和最大尺寸由文件系统块大小和簇大小决定，基于底层设备支持的原子写入单元限制。

#### 要求


ext4 中原子写入的基本要求：

 1. 必须启用 extents 特性（ext4 默认开启）
 2. 底层块设备必须支持原子写入
 3. 对于单 fsblock 原子写入：

    1. 具有适当块大小（最大到页面大小）的文件系统
 4. 对于多 fsblock 原子写入：

    1. 必须启用 bigalloc 特性
    2. 必须适当配置簇大小

注意：EXT4 不支持基于软件或 COW 的原子写入，这意味着只有当底层存储设备支持时，ext4 上的原子写入才受支持。

#### 多 fsblock 实现细节


bigalloc 特性将 ext4 改为以多个文件系统块（也称为簇）为单位进行分配。使用 bigalloc 时，块位图中的每个位代表一个簇（2 的幂个块），而不是单个文件系统块。
EXT4 通过 bigalloc 支持多 fsblock 原子写入，但受以下约束。最小原子写入尺寸是 fs 块大小和最小硬件原子写入单元中较大的一个；最大原子写入尺寸是 bigalloc 簇大小和最大硬件原子写入单元中较小的一个。Bigalloc 确保所有分配都与簇大小对齐，如果分区/逻辑卷的起始本身正确对齐，这就满足了硬件设备的 LBA 对齐要求。

以下是 bigalloc 中原子写入的块分配策略：

 - 对于具有完全已映射 extents 的区域，不需要额外的工作
 - 对于追加写入，分配一个新的已映射 extent
 - 对于完全是空洞（hole）的区域，创建 unwritten extent
 - 对于较大的 unwritten extent，该 extent 被拆分为两个适当请求大小的 unwritten extent
 - 对于混合映射区域（hole、unwritten extent 或 mapped extent 的组合），将以循环方式调用 ext4_map_blocks()，并传入 EXT4_GET_BLOCKS_ZERO 标志，通过向该区域写入零并将其中的任何 unwritten extent 转换为 written（如果在该范围内找到），从而将该区域转换为单个连续的 mapped extent。

注意：在单个连续的底层 extent（无论是 mapped 还是 unwritten）上写入本身没有问题。但是，在执行原子写入时，必须避免写入混合映射区域（即包含 mapped 和 unwritten extent 组合的区域）。

原因是，通过带 RWF_ATOMIC 标志的 pwritev2() 发出的原子写入，要求要么写入全部数据，要么什么也不写。如果在写入操作期间发生系统崩溃或意外断电，受影响的区域（在后续读取时）必须反映完整的旧数据或完整的新数据，而绝不能是两者的混合。

为了强制执行这一保证，我们确保在写入任何数据之前，写入目标由单个连续的 extent 支持。这很关键，因为 ext4 将 unwritten extent 到 written extent 的转换推迟到 I/O 完成路径（通常在 ->end_io() 中）。如果允许写入在混合映射区域（带有 mapped 和 unwritten extent）上进行，并且写入中途发生故障，系统在重启后可能会观察到部分更新的区域，即在 mapped 区域上是新数据，而在从未标记为 written 的 unwritten extent 上是陈旧（旧）数据。这就违反了原子性和/或撕裂写入防护保证。

为了防止此类撕裂写入，ext4 通过 ext4_iomap_alloc 中的 ext4_map_blocks_atomic() 主动为整个请求区域分配单个连续 extent。如果分配是在混合映射上进行的，EXT4 还会强制提交当前日志事务。这确保了在执行实际写入 I/O 之前，此范围内的任何挂起元数据更新（如 unwritten 到 written extent 的转换）与文件数据块处于一致状态。如果提交失败，必须中止整个 I/O 以防止任何可能的撕裂写入。
只有在这一步之后，实际的数据写入操作才由 iomap 执行。

#### 处理跨越叶子块的分裂 Extent


存在一种特殊的边缘情况：我们在逻辑和物理上连续的 extent 被存储在磁盘 extent 树的不同叶子节点中。这是因为磁盘 extent 树仅在叶子块内部进行合并，除了一种情况——两级的树可以被完全合并并折叠进 inode。

如果这样的布局存在，并且在最坏情况下，extent 状态缓存条目由于内存压力被回收，ext4_map_blocks() 可能永远不会为这些分裂的叶子 extent 返回单个连续的 extent。

为了解决这一边缘情况，新增了一个 get block 标志 EXT4_GET_BLOCKS_QUERY_LEAF_BLOCKS flag 以增强 ext4_map_query_blocks() 的查找行为。

这个新的 get block 标志允许 ext4_map_blocks() 首先检查 extent 状态缓存中是否存在整个范围的条目。
如果不存在，它会使用 ext4_map_query_blocks() 查询磁盘 extent 树。
如果定位到的 extent 位于叶子节点的末尾，它会探测下一个逻辑块（lblk）以检测相邻叶子中的连续 extent。

目前只查询一个额外的叶子块以保持效率，因为原子写入通常受限于较小的尺寸
（例如 [blocksize, clustersize]）。


#### 处理日志事务


为了支持多 fsblock 原子写入，我们确保在以下时刻预留足够的日志额度（credits）：

 1. 在 ext4_iomap_alloc() 中的块分配时刻。我们首先查询底层请求范围是否可能存在混合映射。如果是，则预留最多 m_len 的额度，假设每个交替的块可以是后跟一个 hole 的 unwritten extent。

 2. 在 ->end_io() 调用期间，我们确保为进行 unwritten 到 written 的转换启动单个事务。转换循环主要只需用来处理跨越叶子块的分裂 extent。

#### 如何操作


##### 创建支持原子写入的文件系统


首先查看块设备支持的原子写入单元。
详见 atomic_write_bdev_support。

对于使用较大块大小的单 fsblock 原子写入
（在块大小 < 页面大小的系统上）：


    # 创建一个块大小为 16KB 的 ext4 文件系统
    # （要求页面大小 >= 16KB）
    mkfs.ext4 -b 16384 /dev/device

对于使用 bigalloc 的多 fsblock 原子写入：


    # 创建带有 bigalloc、簇大小为 64KB 的 ext4 文件系统
    mkfs.ext4 -F -O bigalloc -b 4096 -C 65536 /dev/device

其中 `-b` 指定块大小，`-C` 指定簇大小（字节），`-O bigalloc` 启用 bigalloc 特性。

##### 应用程序接口


应用程序可以使用带 RWF_ATOMIC 标志的 pwritev2() 系统调用来执行原子写入：


    pwritev2(fd, iov, iovcnt, offset, RWF_ATOMIC);

该写入必须与文件系统块大小对齐，且不得超过文件系统的最大原子写入单元尺寸。
详见 generic_atomic_write_valid()。

带 STATX_WRITE_ATOMIC 标志的 statx() 系统调用可以提供以下详情：

 - `stx_atomic_write_unit_min`：原子写入请求的最小尺寸。
 - `stx_atomic_write_unit_max`：原子写入请求的最大尺寸。
 - `stx_atomic_write_segments_max`：段的上限。可以聚集到一个写入操作中的独立内存缓冲区的数量（例如 IOV_ITER 的 iovcnt 参数）。目前始终设置为 1。

如果支持原子写入，则会设置 statx->attributes 中的 STATX_ATTR_WRITE_ATOMIC 标志。


#### 硬件支持


底层存储设备必须支持原子写入操作。
现代 NVMe 和 SCSI 设备通常提供此能力。
Linux 内核通过 sysfs 暴露此信息：

- `/sys/block/<device>/queue/atomic_write_unit_min` - 最小原子写入尺寸
- `/sys/block/<device>/queue/atomic_write_unit_max` - 最大原子写入尺寸

这些属性的非零值表示该设备支持原子写入。

#### 另请参阅


- [bigalloc](bigalloc) - 关于 bigalloc 特性的文档
- [allocators](allocators) - 关于 ext4 中块分配的文档
- 6.13 中对原子块写入的支持：
  https://lwn.net/Articles/1009298/
