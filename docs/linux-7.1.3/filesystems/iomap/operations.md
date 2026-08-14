..
        为保持作者理智的笨拙风格说明：
        请尽量在单独的行上开始句子，以便句子变更不会在 diff 中串色。
        标题装饰在 sphinx.rst 中有说明。

## 支持的文件操作


   :local:

下面讨论 iomap 实现的高层文件操作。

## 缓冲 I/O


缓冲 I/O 是 Linux 中默认的文件 I/O 路径。
文件内容被缓存在内存中（"pagecache"）以响应读和写。
脏缓存会在某个时刻写回磁盘，也可通过 `fsync` 及其变体强制写回。

iomap 实现了文件系统在传统 I/O 模型下必须自行实现的几乎所有 folio 与 pagecache 管理工作。
这意味着文件系统无需了解分配、映射、管理 uptodate 与 dirty 状态，或 pagecache folio 的回写等细节。
在传统 I/O 模型下，这些都是用 buffer head 链表来低效管理的，而非 iomap 所使用的 per-folio 位图。
除非文件系统显式选择使用 buffer head，否则不会使用它们，这使得缓冲 I/O 高效得多，也让 pagecache 维护者开心得多。

### ``struct address_space_operations``


下列 iomap 函数可直接从地址空间操作结构中引用：

 - `iomap_dirty_folio`
 - `iomap_release_folio`
 - `iomap_invalidate_folio`
 - `iomap_is_partially_uptodate`

下列地址空间操作可轻松封装：

 - `read_folio`
 - `readahead`
 - `writepages`
 - `bmap`
 - `swap_activate`

### ``struct iomap_write_ops``


 struct iomap_write_ops {
     struct folio **(**get_folio)(struct iomap_iter *iter, loff_t pos,
                                unsigned len);
     void (**put_folio)(struct inode **inode, loff_t pos, unsigned copied,
                       struct folio *folio);
     bool (**iomap_valid)(struct inode **inode, const struct iomap *iomap);
     int (**read_folio_range)(const struct iomap_iter **iter,
     			struct folio *folio, loff_t pos, size_t len);
 };

iomap 调用以下函数：

  - `get_folio`：在开始写之前调用，用于分配并返回一个已锁定 folio 的活动引用。
    若未提供此函数，iomap 将调用 `iomap_get_folio`。
    这可用于 `为一次写设置 per-folio 文件系统状态
    <https://lore.kernel.org/all/20190429220934.10415-5-agruenba@redhat.com/>`_。

  - `put_folio`：在 pagecache 操作完成后调用，用于解锁并释放一个 folio。
    若未提供此函数，iomap 将自行执行 `folio_unlock` 与 `folio_put`。
    这可用于 `提交由 ->get_folio 设置的 per-folio 文件系统状态
    <https://lore.kernel.org/all/20180619164137.13720-6-hch@lst.de/>`_。

  - `iomap_valid`：文件系统不能在 `->iomap_begin` 与 `->iomap_end` 之间持有锁，因为 pagecache 操作可能获取 folio 锁、对用户空间页产生缺页、为内存回收发起回写，或进行其他耗时操作。
    如果文件的空间映射数据是可变的，那么某个 pagecache folio 的映射有可能在分配、安放并锁定该 folio 的这段时间内发生变化。

    对于 pagecache，如果回写不获取 `i_rwsem` 或 `invalidate_lock` 并更新映射信息，就可能发生竞争。
    如果文件系统允许并发写，也可能发生竞争。
    对于此类文件，必须在获取 folio 锁之后重新校验映射，以便 iomap 能正确管理该 folio。

    fsdax 不需要这种重新校验，因为它没有回写，也不支持 unwritten extent。

    受此类竞争影响的文件系统必须提供 `->iomap_valid` 函数来决定映射是否仍然有效。
    如果映射无效，将重新采样映射。

    为了支持有效性判定，文件系统的 `->iomap_begin` 函数在填充其他 iomap 字段的同时，可以设置 `struct iomap::validity_cookie`。
    一个简单的校验 cookie 实现是序列计数器。
    如果文件系统在每次修改 inode 的 extent map 时都递增序列计数器，就可以在 `->iomap_begin` 期间将其放入 ``struct iomap::validity_cookie`` 中。
    如果回传给 `->iomap_valid` 时 cookie 中的值被发现与文件系统持有的值不同，那么应认为该 iomap 已过期，校验失败。

  - `read_folio_range`：调用以同步读入将要写入的范围。若未提供此函数，iomap 将默认提交一个 bio 读请求。

这些 `struct kiocb` 标志对 iomap 的缓冲 I/O 很重要：

 - `IOCB_NOWAIT`：开启 `IOMAP_NOWAIT`。

 - `IOCB_DONTCACHE`：开启 `IOMAP_DONTCACHE`。

### ``struct iomap_read_ops``


 struct iomap_read_ops {
     int (**read_folio_range)(const struct iomap_iter **iter,
                             struct iomap_read_folio_ctx *ctx, size_t len);
     void (**submit_read)(struct iomap_read_folio_ctx **ctx);
 };

iomap 调用以下函数：

  - `read_folio_range`：调用以读入该范围。调用者必须提供此函数。若成功，无论读成功与否，在读入该范围后都必须调用 iomap_finish_folio_read()。

  - `submit_read`：提交任何挂起的读请求。此函数为可选。

### 每 Folio 内部状态


如果 fsblock 大小与 pagecache folio 大小一致，则假定所有磁盘 I/O 操作都作用于整个 folio。
对于这种情况，仅需 folio 的 uptodate（内存内容至少与磁盘上一样新）和 dirty（内存内容比磁盘上更新）状态即可。

如果 fsblock 大小小于 pagecache folio 大小，iomap 自行跟踪每个 fsblock 的 uptodate 与 dirty 状态。
这使得 iomap 既能处理 "bs < ps" `文件系统
<https://lore.kernel.org/all/20230725122932.144426-1-ritesh.list@gmail.com/>`_，也能处理 pagecache 中的大 folio。

iomap 在内部为每个 fsblock 跟踪两个状态位：

 - `uptodate`：iomap 会尽量保持 folio 完全是最新的。
   如果存在读（预读）错误，那些 fsblock 不会被标记为 uptodate。
   当 folio 内所有 fsblock 都是 uptodate 时，folio 本身会被标记为 uptodate。

 - `dirty`：当程序写入文件时，iomap 会设置 per-block 的 dirty 状态。
   当 folio 内任意 fsblock 为 dirty 时，folio 本身会被标记为 dirty。

iomap 还跟踪正在进行的读写磁盘 I/O 数量。
该结构比 `struct buffer_head` 轻量得多，因为每个 folio 只有一个，且 per-fsblock 开销是两个位对比 104 字节。

希望在 pagecache 中开启大 folio 的文件系统，应在初始化 incore inode 时调用 `mapping_set_large_folios`。

### 缓冲预读与读取


`iomap_readahead` 函数向 pagecache 发起预读。
`iomap_read_folio` 函数将一份 folio 大小的数据读入 pagecache。
传给 `->iomap_begin` 的 `flags` 参数将被设为零。
pagecache 在调用文件系统之前会获取所需的任何锁。

`iomap_readahead` 与 `iomap_read_folio` 都传入一个 ``struct iomap_read_folio_ctx``：


 struct iomap_read_folio_ctx {
    const struct iomap_read_ops *ops;
    struct folio *cur_folio;
    struct readahead_control *rac;
    void *read_ctx;
 };

`iomap_readahead` 必须设置：
 - `ops->read_folio_range()` 与 `rac`

`iomap_read_folio` 必须设置：
 - `ops->read_folio_range()` 与 `cur_folio`

`ops->submit_read()` 与 `read_ctx` 为可选。`read_ctx` 用于在 ops 回调中传递调用者需要访问的自定义数据，以满足读取需求。

### 缓冲写


`iomap_file_buffered_write` 函数将一个 `iocb` 写入 pagecache。
`IOMAP_WRITE` 或 `IOMAP_WRITE` | `IOMAP_NOWAIT` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以共享或独占模式获取 `i_rwsem`。

#### mmap 写缺页


`iomap_page_mkwrite` 函数处理对 pagecache 中某 folio 的写缺页。
`IOMAP_WRITE | IOMAP_FAULT` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以共享或独占模式获取 mmap 的 `invalidate_lock`。

#### 缓冲写失败


对 pagecache 的短写之后，未被写入的区域不会被标记为 dirty。
文件系统必须安排 `取消
<https://lore.kernel.org/all/20221123055812.747923-6-david@fromorbit.com/>`_ 这类 `预留
<https://lore.kernel.org/linux-xfs/20220817093627.GZ3600936@dread.disaster.area/>`_，因为回写不会消耗该预留。
`iomap_write_delalloc_release` 可从 `->iomap_end` 函数调用，以查找缓存了全新（`IOMAP_F_NEW`）delalloc 映射的 folio 的所有干净区域。
它获取 `invalidate_lock`。

文件系统必须提供一个 `punch` 函数，对处于此状态的每个文件范围调用。
此函数必须**仅**移除延迟分配预留，以防与当前线程竞争的另一个线程成功写入同一区域并触发回写将脏数据刷到磁盘。

#### 文件操作的零填充


文件系统可以调用 `iomap_zero_range` 来对未与 fsblock 大小对齐的非截断文件操作执行 pagecache 的零填充。
`IOMAP_ZERO` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以独占模式持有 `i_rwsem` 与 `invalidate_lock`。

#### 取消共享 Reflinked 文件数据


文件系统可以调用 `iomap_file_unshare` 强制一个与另一文件共享存储的文件，预先将共享数据复制到新分配的存储中。
`IOMAP_WRITE | IOMAP_UNSHARE` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以独占模式持有 `i_rwsem` 与 `invalidate_lock`。

### 截断


文件系统可以调用 `iomap_truncate_page` 在文件截断操作期间，将 pagecache 中从 EOF 到 fsblock 末尾的字节清零。
`truncate_setsize` 或 `truncate_pagecache` 将处理 EOF 块之后的所有内容。
`IOMAP_ZERO` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以独占模式持有 `i_rwsem` 与 `invalidate_lock`。

### Pagecache 回写


文件系统可以调用 `iomap_writepages` 来响应将脏 pagecache folio 写回磁盘的请求。
`mapping` 与 `wbc` 参数应原样传递。
`wpc` 指针应由文件系统分配，且必须初始化为零。

pagecache 在尝试调度某个 folio 进行回写之前会锁定它。
它不会锁定 `i_rwsem` 或 `invalidate_lock`。

即使回写失败，经过下述 `->writeback_range` 机制的 folio 的 dirty 位也会被清除。
这是为了防止存储设备故障时出现脏 folio 结块；会记录一个 `-EIO` 供用户空间通过 `fsync` 收集。

`ops` 结构必须指定，如下所示：

#### ``struct iomap_writeback_ops``


 struct iomap_writeback_ops {
    int (**writeback_range)(struct iomap_writepage_ctx **wpc,
        struct folio *folio, u64 pos, unsigned int len, u64 end_pos);
    int (**writeback_submit)(struct iomap_writepage_ctx **wpc, int error);
 };

字段如下：

  - `writeback_range`：将 `wpc->iomap` 设置为由 `offset` 与 `len` 给出的文件范围（字节）的空间映射。
    iomap 对每个脏 folio 中的每个脏 fs 块调用此函数，不过对于 folio 内连续脏 fsblock 的运行会 `复用映射
    <https://lore.kernel.org/all/20231207072710.176093-15-hch@lst.de/>`_。
    不要在此返回 `IOMAP_INLINE` 映射；`->iomap_end` 函数必须处理已写数据的持久化。
    不要在此返回 `IOMAP_DELALLOC` 映射；iomap 当前要求映射到已分配的空间。
    如果映射未改变，文件系统可以跳过可能昂贵的映射查找。
    这种重新校验必须由文件系统显式编码实现；尚不清楚 `iomap::validity_cookie` 能否复用于此目的。

    如果该方法未能为某个脏 folio 的任何部分调度 I/O，它应丢弃可能为该写所做的任何预留。
    folio 将被标记为干净，并在 pagecache 中记录一个 `-EIO`。
    文件系统可以使用此回调 `移除
    <https://lore.kernel.org/all/20201029163313.1766967-1-bfoster@redhat.com/>`_ delalloc 预留，以避免为干净的 pagecache 保留 delalloc 预留。
    此函数必须由文件系统提供。
    如果成功，无论回写成功与否，在范围回写完成后都必须调用一次 iomap_finish_folio_write()。

  - `writeback_submit`：提交之前构建的回写上下文。
    基于块的文件系统应使用 iomap_ioend_writeback_submit 辅助函数，其他文件系统可实现自己的。
    文件系统可以选择性地挂接到回写 bio 提交。
    这可能包括写前的空间记账更新，或为内部目的安装自定义的 `->bi_end_io` 函数，例如将 ioend 完成延迟到 workqueue，以便在提交 bio 之前从进程上下文运行元数据更新事务。
    此函数必须由文件系统提供。

#### Pagecache 回写完成


为了处理回写磁盘 I/O 完成后必须进行的簿记，iomap 创建了 `struct iomap_ioend` 对象链，这些对象封装了用于将 pagecache 数据写入磁盘的 `bio`。
默认情况下，iomap 通过清除附加到 `ioend` 的 folio 上的 writeback 位来完成回写 ioend。
如果写失败，它还会在 folio 和地址空间上设置错误位。
这可能发生在中断或进程上下文中，取决于存储设备。
需要更新内部簿记（例如 unwritten extent 转换）的文件系统应在 `->submit_writeback` 提交的 bio 上设置自己的 bi_end_io。
此函数应在完成自身工作（例如 unwritten extent 转换）后调用 `iomap_finish_ioends`。

某些文件系统可能希望 `分摊运行元数据事务
<https://lore.kernel.org/all/20220120034733.221737-1-david@fromorbit.com/>`_ 的成本，以对写后更新进行批处理。
它们可能还要求事务从进程上下文运行，这意味着将批次推送到 workqueue。
iomap ioend 包含一个 `list_head` 以支持批处理。

给定一批 ioend，iomap 有几个辅助函数协助分摊：

 - `iomap_sort_ioends`：按文件偏移对列表中的所有 ioend 排序。

 - `iomap_ioend_try_merge`：给定一个不在任何列表中的 ioend 以及另一个已排序的 ioend 列表，将列表中尽可能多的 ioend 从头部合并到给定 ioend 中。
   只有当文件范围和存储地址连续、unwritten 与 shared 状态相同、且写 I/O 结果相同时，ioend 才能合并。
   合并后的 ioend 自成一体成为一个列表。

 - `iomap_finish_ioends`：完成一个可能链接了其他 ioend 的 ioend。

## 直接 I/O


在 Linux 中，直接 I/O 定义为直接发往存储、绕过 pagecache 的文件 I/O。
`iomap_dio_rw` 函数实现了文件的 O_DIRECT（直接 I/O）读和写。


 ssize_t iomap_dio_rw(struct kiocb **iocb, struct iov_iter **iter,
                      const struct iomap_ops *ops,
                      const struct iomap_dio_ops *dops,
                      unsigned int dio_flags, void *private,
                      size_t done_before);

文件系统可以提供 `dops` 参数，如果它需要在 I/O 发往存储前后执行额外工作。
`done_before` 参数告知已经传输了多少请求。
它用于在 `请求的一部分
<https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=c03098d4b9ad76bca2966a8769dcfe59f7f85103>`_ 已经同步完成时，异步地继续一个请求。

如果调用前已为 `iocb` 启动了写，则应设置 `done_before` 参数。
I/O 的方向由传入的 `iocb` 决定。

`dio_flags` 参数可设置为下列值的任意组合：

 - `IOMAP_DIO_FORCE_WAIT`：即使 kiocb 不是同步的，也等待 I/O 完成。

 - `IOMAP_DIO_OVERWRITE_ONLY`：对此范围执行纯覆盖写，否则以 `-EAGAIN` 失败。
   这可被具有复杂未对齐 I/O 写路径的文件系统用来为未对齐写提供优化的快速路径。
   如果能执行纯覆盖写，则无需针对同一文件系统块的其他 I/O 进行串行化，因为没有暴露陈旧数据或数据丢失的风险。
   如果无法执行纯覆盖写，则文件系统可以执行所需的串行化步骤，以提供对未对齐 I/O 范围的独占访问，从而安全地执行分配和子块零填充。
   文件系统可使用此标志尝试减少锁竞争，但要 `正确
   <https://lore.kernel.org/linux-ext4/20230314130759.642710-1-bfoster@redhat.com/>`_ 做到需要大量 `细致检查
   <https://lore.kernel.org/linux-ext4/20230810165559.946222-1-bfoster@redhat.com/>`_。

 - `IOMAP_DIO_PARTIAL`：如果发生缺页，返回已完成的任何进度。
   调用者可以处理缺页并重试该操作。
   如果调用者决定重试该操作，应将之前所有调用的累计返回值作为 `done_before` 参数传给下一次调用。

这些 `struct kiocb` 标志对 iomap 的直接 I/O 很重要：

 - `IOCB_NOWAIT`：开启 `IOMAP_NOWAIT`。

 - `IOCB_SYNC`：确保在完成调用之前设备已将持久化数据写入磁盘。
   在纯覆盖写的情况下，I/O 可能以启用 FUA 的方式发出。

 - `IOCB_HIPRI`：轮询 I/O 完成，而不是等待中断。
   仅对异步 I/O 有意义，且仅当整个 I/O 可以作为单个 `struct bio` 发出时。

文件系统应从 `->read_iter` 和 `->write_iter` 调用 `iomap_dio_rw`，并在文件的 `->open` 函数中设置 `FMODE_CAN_ODIRECT`。
它们不应设置 `->direct_IO`，该字段已废弃。

如果文件系统希望在直接 I/O 完成前执行自身工作，它应调用 `__iomap_dio_rw`。
如果其返回值不是错误指针或 NULL 指针，文件系统应在完成内部工作后将返回值传给 `iomap_dio_complete`。

### 返回值


`iomap_dio_rw` 可返回以下之一：

 - 一个非负字节数，表示已传输的字节。

 - `-ENOTBLK`：回退到缓冲 I/O。
   如果 iomap 无法在将 I/O 发往存储前使 page cache 失效，它自身会返回此值。
   `->iomap_begin` 或 `->iomap_end` 函数也可能返回此值。

 - `-EIOCBQUEUED`：异步直接 I/O 请求已入队，将单独完成。

 - 任何其他负错误码。

### 直接读


直接 I/O 读发起从存储设备到调用者缓冲区的读 I/O。
在发起读 I/O 之前，pagecache 的脏部分会被刷回存储。
`->iomap_begin` 的 `flags` 值将是 `IOMAP_DIRECT`，可附加下列增强的组合：

 - `IOMAP_NOWAIT`，如前所述。

调用者通常在调用此函数前以共享模式持有 `i_rwsem`。

### 直接写


直接 I/O 写发起从调用者缓冲区到存储设备的写 I/O。
在发起写 I/O 之前，pagecache 的脏部分会被刷回存储。
在写 I/O 前后都会使 pagecache 失效。
`->iomap_begin` 的 `flags` 值将是 ``IOMAP_DIRECT | IOMAP_WRITE``，可附加下列增强的组合：

 - `IOMAP_NOWAIT`，如前所述。

 - `IOMAP_OVERWRITE_ONLY`：不允许分配块和零填充部分块。
   整个文件范围必须映射到单个已写或 unwritten extent。
   如果映射是 unwritten 的，且文件系统无法在不暴露陈旧内容的情况下处理未对齐区域的零填充，则文件 I/O 范围必须对齐到文件系统块大小。

 - `IOMAP_ATOMIC`：此写带有撕裂写保护。
   撕裂写保护可基于硬件卸载提供，或由文件系统提供的软件机制提供。

   对于基于硬件卸载的支持，写只能创建一个 bio，且写不得拆分为多个 I/O 请求，即必须设置 REQ_ATOMIC 标志。
   要写入的文件范围必须对齐，以满足文件系统和底层块设备原子提交能力的要求。
   如果需要文件系统元数据更新（例如 unwritten extent 转换或写时复制），整个文件范围的所有更新也必须原子提交。
   无损写可能比单个文件块更长。在所有情况下，映射起始的磁盘块必须至少与写偏移具有相同的对齐。
   文件系统必须设置 IOMAP_F_ATOMIC_BIO 以告知 iomap 核心基于硬件卸载的无损写。

   对于基于文件系统提供的软件机制的无损写，适用于基于硬件卸载的无损写的磁盘块对齐和单 bio 限制均不适用。
   该机制通常用作基于硬件卸载的无损写可能无法发出时的回退，例如写入范围覆盖多个 extent，意味着无法发出单个 bio。
   整个文件范围的所有文件系统元数据更新也必须原子提交。

调用者通常在调用此函数前以共享或独占模式持有 `i_rwsem`。

### ``struct iomap_dio_ops:``


 struct iomap_dio_ops {
     void (**submit_io)(const struct iomap_iter **iter, struct bio *bio,
                       loff_t file_offset);
     int (**end_io)(struct kiocb **iocb, ssize_t size, int error,
                   unsigned flags);
     struct bio_set *bio_set;
 };

此结构的字段如下：

  - `submit_io`：当 iomap 构造好所请求 I/O 的 `struct bio` 对象并希望将其提交给块设备时，会调用此函数。
   如果未提供函数，`submit_bio` 将被直接调用。
   希望在之前执行额外工作（例如 btrfs 的数据复制）的文件系统应实现此函数。

  - `end_io`：在 `struct bio` 完成后调用。
   此函数应执行 unwritten extent 映射的写后转换、处理写失败等。
   `flags` 参数可设置为下列组合：

    - `IOMAP_DIO_UNWRITTEN`：映射是 unwritten 的，因此 ioend 应将 extent 标记为已写。

    - `IOMAP_DIO_COW`：写入映射中的空间需要写时复制操作，因此 ioend 应切换映射。

  - `bio_set`：这允许文件系统提供自定义的 bio_set 用于分配直接 I/O 的 bio。
    这使得文件系统能够 `存放额外的 per-bio 信息
    <https://lore.kernel.org/all/20220505201115.937837-3-hch@lst.de/>`_ 供私有使用。
    如果此字段为 NULL，将使用通用的 `struct bio` 对象。

希望在 I/O 完成后执行额外工作的文件系统应通过 `->submit_io` 设置自定义的 `->bi_end_io` 函数。
之后，自定义的 endio 函数必须调用 `iomap_dio_bio_end_io` 来完成直接 I/O。

## DAX I/O


某些存储设备可直接映射为内存。
这些设备支持一种称为 "fsdax" 的新访问模式，允许通过 CPU 和内存控制器进行加载和存储。

### fsdax 读


fsdax 读执行从存储设备到调用者缓冲区的 memcpy。
`->iomap_begin` 的 `flags` 值将是 `IOMAP_DAX`，可附加下列增强的组合：

 - `IOMAP_NOWAIT`，如前所述。

调用者通常在调用此函数前以共享模式持有 `i_rwsem`。

### fsdax 写


fsdax 写发起从调用者缓冲区到存储设备的 memcpy。
`->iomap_begin` 的 `flags` 值将是 ``IOMAP_DAX | IOMAP_WRITE``，可附加下列增强的组合：

 - `IOMAP_NOWAIT`，如前所述。

 - `IOMAP_OVERWRITE_ONLY`：调用者要求从此映射执行纯覆盖写。
   这要求文件系统 extent 映射已经以 `IOMAP_MAPPED` 类型存在，并跨越整个写 I/O 请求的范围。
   如果文件系统无法以允许 iomap 基础设施执行纯覆盖写的方式映射此请求，则必须以 `-EAGAIN` 使映射操作失败。

调用者通常在调用此函数前以独占模式持有 `i_rwsem`。

#### fsdax mmap 缺页


`dax_iomap_fault` 函数处理对 fsdax 存储的读和写缺页。
对于读缺页，`IOMAP_DAX | IOMAP_FAULT` 将作为 `flags` 参数传给 `->iomap_begin`。
对于写缺页，`IOMAP_DAX | IOMAP_FAULT | IOMAP_WRITE` 将作为 `flags` 参数传给 `->iomap_begin`。

调用者通常持有与其调用 iomap pagecache 对应函数相同的锁。

### fsdax 截断、fallocate 与取消共享


对于 fsdax 文件，提供以下函数以替换其 iomap pagecache I/O 对应函数。
传给 `->iomap_begin` 的 `flags` 参数与 pagecache 对应函数相同，只是增加了 `IOMAP_DAX`。

 - `dax_file_unshare`
 - `dax_zero_range`
 - `dax_truncate_page`

调用者通常持有与其调用 iomap pagecache 对应函数相同的锁。

### fsdax 去重


实现 `FIDEDUPERANGE` ioctl 的文件系统必须用其自身的 iomap 读 ops 调用 `dax_remap_file_range_prep` 函数。

## 文件定位


iomap 实现了 `llseek` 系统调用的两种迭代 whence 模式。

### SEEK_DATA


`iomap_seek_data` 函数实现了 llseek 的 SEEK_DATA "whence" 值。
`IOMAP_REPORT` 将作为 `flags` 参数传给 `->iomap_begin`。

对于 unwritten 映射，将搜索 pagecache。
pagecache 中映射了 folio 且这些 folio 内有 uptodate fsblock 的区域将被报告为数据区域。

调用者通常在调用此函数前以共享模式持有 `i_rwsem`。

### SEEK_HOLE


`iomap_seek_hole` 函数实现了 llseek 的 SEEK_HOLE "whence" 值。
`IOMAP_REPORT` 将作为 `flags` 参数传给 `->iomap_begin`。

对于 unwritten 映射，将搜索 pagecache。
pagecache 中没有映射 folio，或 folio 内有非 uptodate fsblock 的区域将被报告为稀疏空洞区域。

调用者通常在调用此函数前以共享模式持有 `i_rwsem`。

## 交换文件激活


`iomap_swapfile_activate` 函数查找文件中的所有按基页对齐的区域，并将其设置为交换空间。
文件在激活前会被 `fsync()`。
`IOMAP_REPORT` 将作为 `flags` 参数传给 `->iomap_begin`。
所有映射必须是已映射或 unwritten 的；不能是 dirty 或 shared 的，且不能跨越多个块设备。
调用者必须以独占模式持有 `i_rwsem`；这已由 `swapon` 提供。

## 文件空间映射报告


iomap 实现了两个文件空间映射系统调用。

### FS_IOC_FIEMAP


`iomap_fiemap` 函数以 `FS_IOC_FIEMAP` ioctl 指定的格式将文件 extent 映射导出到用户空间。
`IOMAP_REPORT` 将作为 `flags` 参数传给 `->iomap_begin`。
调用者通常在调用此函数前以共享模式持有 `i_rwsem`。

### FIBMAP（已废弃）


`iomap_bmap` 实现 FIBMAP。
调用约定与 FIEMAP 相同。
此函数仅为与转换前已实现 FIBMAP 的文件系统保持兼容而提供。
此 ioctl 已废弃；不要为没有它的文件系统添加 FIBMAP 实现。
调用者可能在调用此函数前应持有 `i_rwsem` 的共享模式，但这并不明确。
