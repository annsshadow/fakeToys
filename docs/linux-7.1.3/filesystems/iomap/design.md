
..
        Dumb style notes to maintain the author's sanity:
        Please try to start sentences on separate lines so that
        sentence changes don't bleed colors in diff.
        Heading decorations are documented in sphinx.rst.

## 库设计（Library Design


   :local:

## 简介（Introduction


iomap 是一个用于处理常见文件操作的内核文件系统库
该库分为两层

 1. 底层提供一个对文件偏移范围进行迭代的迭代器
    这一层试图从文件系统获取每个文件范围到存储的映射
    但存储信息并非必需

 2. 上层作用于底层迭代器所提供的空间映射

迭代可以涉及文件逻辑偏移范围到物理区段（extent）的映射
但存储层信息并非必需，例如遍历已缓存的文件信息时
该库导出了各种用于实现文件操作的 API，例如：

 - 页缓存（pagecache）的读和
 - 对页缓存folio 写时缺页
 - folio 的回
 - 直接 I/O 的读和写
 - fsdax I/O 的读、写、加载与存储
 - FIEMAP
 - lseek 鐨?`SEEK_DATA` 涓?`SEEK_HOLE`
 - 交换文件（swapfile）激

这个库起源于 XFS 曾经使用的文I/O 路径；如今已被扩展以涵盖若干其他操作

## 谁应当阅读本文？


本文档的目标读者是文件系统、存储以及页缓存的程序员和代码评审者

如果你在从事 PCI、机器架构或设备驱动方面的工作，你多半来错了地方

## 这好在哪里？


与经典的 Linux I/O 模型不同——后者把文件 I/O 拆成小单元（通常是内存页或块）并基于该单元查找空间映射——iomap 模型向文件系统请求它能为给定文件操作创建的最大空间映射，并在此基础上发起操作
这一策略提升了文件系统对正在执行的操作大小的可见性，使其能够在可能时通过更大的空间分配来对抗碎片
更大的空间映射通过在更大量的数据上分摊进入文件系统的映射函数调用开销，从而改善运行时性能

高层来看，一iomap 操作 `` `looks like this <https://lore.kernel.org/all/ZGbVaewzcCysclPt@dread.disaster.area/>`_ ``

1. 对操作范围内的每个字节…

   1. 通过 `->iomap_begin` 获取一个空间映

   2. 对每个子工作单元…

      1. 如有必要，重新校验映射并返回上面(1)
         目前为止只有页缓存操作需要做这个

      2. 执行工作

   3. 推进操作游标

   4. 如有必要，通过 `->iomap_end` 释放映射

每个 iomap 操作将在下文更详细地介绍
本库此前已有一`LWN article <https://lwn.net/Articles/935934/>`_ 
一`KernelNewbies page <https://kernelnewbies.org/KernelProjects/iomap>`_ 做过介绍

本文档的目标是简要讨iomap 的设计与能力，随后给iomap 所呈现接口的更详细目录
如果你改iomap，请更新本设计文档

## 文件范围迭代器（File Range Iterator


### 定义（Definitions


 - **buffer head**：旧缓冲缓存（buffer cache）的破碎残骸

 - `fsblock`：一个文件的块大小，也称`i_blocksize`

 - `i_rwsem`：VFS `struct inode` 读写信号量（rwsemaphore）
   进程以共享模式持有它来读取文件状态和内容
   某些文件系统可能允许以共享模式写入
   进程常以独占模式持有它来改变文件状态和内容

 - `invalidate_lock`：页缓存`struct address_space` 读写信号量，
   用于保护那些支持EOF 之下打掉 folio 的文件系统，防止 folio 的插入与移除
   希望插入 folio 的进程必须以共享模式持有此锁以防止移除，但允许并发插入
   希望移除 folio 的进程必须以独占模式持有此锁以防止插入
   并发移除是不允许的

 - `dax_read_lock`：dax 所取的 RCU 读锁，用于防止设备预关闭（pre-shutdown）钩子在其他线程释放资源之前返回

 - **filesystem mapping lock（文件系统映射锁*：这个同步原语是文件系统内部的，
   必须在对映射取样期间保护文件映射数据不被更新
   文件系统作者必须决定这种协调应如何发生；它不一定需要是一把真正的锁

 - **iomap internal operation lock（iomap 内部操作锁）**：这是一个通用术语
   iomap 函数在持有映射期间所取的同步原语
   一个具体例子是在读写页缓存时取 folio 锁

 - **pure overwrite（纯覆盖写）**：一种在提交或完成期间都无需任何元数据或清零操作的写操作
   这意味着文件系统必须已经把磁盘上的空间分配为 `IOMAP_MAPPED`
   并且文件系统不得IO 对齐或大小施加任何约束
   I/O 对齐的唯一约束是设备级的（最I/O 大小和对齐，通常是扇区大小）

### ``struct iomap``


文件系统通过下面的结构，把文件的字节范围到存储设备字节范围的映射告知 iomap 迭代器：


 struct iomap {
     u64                 addr;
     loff_t              offset;
     u64                 length;
     u16                 type;
     u16                 flags;
     struct block_device *bdev;
     struct dax_device   *dax_dev;
     void                *inline_data;
     void                *private;
     u64                 validity_cookie;
 };

各字段如下：

 - `offset` `length` 描述此映射所覆盖的文件偏移范围（以字节计）
   这些字段必须总是由文件系统设置

 - `type` 描述空间映射的类型：

   - **IOMAP_HOLE**：尚未分配存储
     此类型绝不可作为`IOMAP_WRITE` 操作的回应返回，因为写操作必须分配并映射空间，并返回该映射
     `addr` 字段必须设为 `IOMAP_NULL_ADDR`
     iomap 不支持向一个空洞写入（无论是通过页缓存还是直I/O）

   - **IOMAP_DELALLOC**：承诺在稍后分配空间延迟分配"）
     如果文件系统在此处返IOMAP_F_NEW 而写操作失败，`->iomap_end` 函数必须删除该预留
     `addr` 字段必须设为 `IOMAP_NULL_ADDR`

   - **IOMAP_MAPPED**：文件范围映射到存储设备上的特定空间
     设备通过 `bdev` `dax_dev` 返回
     设备地址（以字节计）通过 `addr` 返回

   - **IOMAP_UNWRITTEN**：文件范围映射到存储设备上的特定空间，但该空间尚未被初始化
     设备通过 `bdev` `dax_dev` 返回
     设备地址（以字节计）通过 `addr` 返回
     对此类映射的读操作将向调用者返回零
     对于写或回写操作，ioend 应当把映射更新为 MAPPED
     更多细节请参阅关ioend 的小节

   - **IOMAP_INLINE**：文件范围映射到 `inline_data` 所指定的内存缓冲区
     对于写操作，`->iomap_end` 函数大概负责把数据持久化
     `addr` 字段必须设为 `IOMAP_NULL_ADDR`

 - `flags` 描述空间映射的状态
   这些标志应由文件系统`->iomap_begin` 中设置：

   - **IOMAP_F_NEW**：映射之下的空间是新分配的
     不会被写入的区域必须被清零
     如果写失败且该映射是一个空间预留，则必须删除该预留

   - **IOMAP_F_DIRTY**：inode 将含有访问任何已写数据所需的未提交元数据
     需fdatasync 来把这些变更提交到持久存储
     这需要考虑I/O 完成*可能**发生的元数据变更，例如来自直I/O 的文件大小更新

   - **IOMAP_F_SHARED**：映射之下的空间是共享的
     需要写时复制（copy on write）以避免破坏其他文件数据

   - **IOMAP_F_BUFFER_HEAD**：此映射要求对页缓存操作使用 buffer head
     不要再增加对此的使用

   - **IOMAP_F_MERGED**：多个连续的块映射被合并为这一单个映射
     这仅FIEMAP 有用

   - **IOMAP_F_XATTR**：此映射用于扩展属性数据，而非常规文件数据
     这仅FIEMAP 有用

   - **IOMAP_F_BOUNDARY**：这表明 I/O 及其完成不得与任何其I/O 或完成合并。文件系统在向无法处理跨越某LBA I/O 的设备提I/O 时必须使用此标志（例ZNS 设备）。该标志仅适用于缓I/O 回写；所有其他函数都忽略它

   - **IOMAP_F_PRIVATE**：此标志保留给文件系统私有使用

   - **IOMAP_F_ANON_WRITE**：表示（写）I/O 尚未分配目标块，文件系统将在 bio 提交处理函数中进行分配，并按需拆分 I/O

   - **IOMAP_F_ATOMIC_BIO**：这表明I/O 必须以在 bio 中设置了 `REQ_ATOMIC` 标志的方式提交。文件系统需要设置此标志来告iomap，写 I/O 操作需要基于硬件卸载机制的撕裂写（torn-write）保护。它们还必须确保I/O 完成时对映射的更新必须以单次元数据更新完成

   这些标志可能iomap 自身在文件操作期间设置
   文件系统如果需要观察这些标志，应提供一`->iomap_end` 函数

   - **IOMAP_F_SIZE_CHANGED**：由于使用了此映射，文件大小已改变

   - **IOMAP_F_STALE**：该映射被发现已过期
     iomap 会对此映射调`->iomap_end`，然后调`->iomap_begin` 以获取一个新的映射

   目前，这些标志只由页缓存操作设置

 - `addr` 描述设备地址，以字节计

 - `bdev` 描述此映射的块设备
   这只需要在映射或未完成（unwritten）操作中设置

 - `dax_dev` 描述此映射的 DAX 设备
   这只需要在映射或未完成操作中设置，并且仅针fsdax 操作

 - `inline_data` 指向用于涉及 `IOMAP_INLINE` 映射I/O 的内存缓冲区
   对所有其他映射类型，此值被忽略

 - `private` 是一个指`filesystem-private information <https://lore.kernel.org/all/20180619164137.13720-7-hch@lst.de/>`_ 的指针
   此值会原样传给 `->iomap_end`

 - `validity_cookie` 是文件系统设置的一个魔数般新鲜值，应当用于检测过期映射
   对于页缓存操作，这对正确运行至关重要，因为可能发生缺页，这意味着文件系统锁不应在 `->iomap_begin` `->iomap_end` 之间持有
   具有完全静态映射的文件系统无需设置此值
   只有页缓存操作会重新校验映射；详见关`iomap_valid` 的小节

### ``struct iomap_ops``


每个 iomap 函数都要求文件系统传入一个操作结构，以获取映射并（可选地）释放映射：


 struct iomap_ops {
     int (**iomap_begin)(struct inode **inode, loff_t pos, loff_t length,
                        unsigned flags, struct iomap *iomap,
                        struct iomap *srcmap);

     int (**iomap_end)(struct inode **inode, loff_t pos, loff_t length,
                      ssize_t written, unsigned flags,
                      struct iomap *iomap);
 };

#### ``->iomap_begin``


iomap 操作调用 `->iomap_begin` 来为 `inode` 这个文件、由 `pos` `length` 指定的字节范围获取一个文件映射
此映射应通过 `iomap` 指针返回
该映射必须至少覆盖所提供的文件范围的第一个字节，但无需覆盖整个请求范围

每个 iomap 操作通过 `flags` 参数描述所请求的操作
`flags` 的确切取值将在下文各操作专属的小节中说明
这些标志至少在原则上可普遍适用iomap 操作

 - `IOMAP_DIRECT` 在调用者希望向块存储发起文I/O 时设置

 - `IOMAP_DAX` 在调用者希望向类内存存储发起文I/O 时设置

 - `IOMAP_NOWAIT` 在调用者希望尽最大努力避免任何会导致提交任务阻塞的操作时设置
   其意图类似于网络 API 中的 `O_NONBLOCK`——它用于异步应用程序，使其继续做其他工作，而非等待特定的不可用文件系统资源变为可用
   实现 `IOMAP_NOWAIT` 语义的文件系统需要使trylock 算法
   它们需要能够用单个 iomap 映射满足整个 I/O 请求范围
   它们需要避免同步地读或写元数据
   它们需要避免阻塞内存分配
   它们需要避免等待事务预留以允许修改发生
   它们大概不应分配新的空间
   等等
   如果文件系统开发者对任何特定`IOMAP_NOWAIT` 操作是否最终会阻塞有所怀疑，
   那么他们应尽早返`-EAGAIN`，而不是启动操作并迫使提交任务阻塞
   `IOMAP_NOWAIT` 通常是代`IOCB_NOWAIT` `RWF_NOWAIT` 设置的

 - `IOMAP_DONTCACHE` 在调用者希望执行缓冲文I/O，并希望内核I/O 完成后（如果尚未被另一线程使用）丢弃页缓存时设置

如果需要从设备上的 `different <https://lore.kernel.org/all/20191008071527.29304-9-hch@lst.de/>`_ 设备或地址范围读取现有文件内容，文件系统应通过 `srcmap` 返回该信息
只有页缓存和 fsdax 操作支持从一个映射读取并写入另一个映射

#### ``->iomap_end``


操作完成后，若存`->iomap_end` 函数，则被调用以表明 iomap 已结束对一个映射的使用
通常，实现会用此函数来拆除在 `->iomap_begin` 中建立的任何上下文
例如，一次写可能希望提交所操作字节的预留，并取消对未被操作字节的空间预留
如果没有字节被触及，`written` 可能为零
`flags` 将包含传`->iomap_begin` 的相同值
读的 iomap 操作大概不需要提供此函数

两个函数在出错时应返回负errno 码，成功时返回零

## 为文件操作做准备（Preparing for File Operations


iomap 只处理映射和 I/O
文件系统仍必须在发起 I/O 操作之前调用 VFS 来检查输入参数和文件状态
它不处理获取文件系统冻结保护、时间戳更新、去除特权或访问控制

## 锁层级（Locking Hierarchy


iomap 要求文件系统提供它们自己的锁模型
iomap 而言，同步原语分为三类：

 - **上层（upper*原语由文件系统提供，用于协调对不iomap 操作的访问
   具体的原语因文件系统和操作而异，但通常VFS inode、页缓存失效（invalidation）或 folio 锁
   例如，一个文件系统可能在调用 `iomap_file_buffered_write` `iomap_file_unshare` 之前`i_rwsem`，以防止这两个文件操作互相破坏
   页缓存回写可能会锁住一folio，以防止其他线程在回写进行期间访问该 folio

   - **下层（lower*原语由文件系统在 `->iomap_begin` `->iomap_end` 函数中取得，用于协调对文件空间映射信息的访问
     iomap 对象的字段应在持有此原语期间填好
     在获取下层同步原语时，上层同步原语（如果有）仍被持有
     例如，XFS 在取样映射时`ILOCK_EXCL`，ext4 `i_data_sem`
     具有不可变映射信息的文件系统在这里可能不需要同步

   - **操作（operation*原语由某iomap 操作取得，用于协调对其自身内部数据结构的访问
     在获取此原语时，上层同步原语（如果有）仍被持有
     在获取此原语时不持有下层原语
     例如，页缓存写操作会先获取一个文件映射，然后抓取并锁住一folio 以复制新内容
     它可能还会锁住一个内folio 状态对象来更新元数据

确切的锁需求因文件系统而异；对于某些操作，其中一些锁可被省略
所有进一步提及的锁都*建议**，而非强制
每个文件系统作者必须自己弄清楚锁的安排

## 缺陷与限制（Bugs and Limitations


 - 不支fscrypt
 - 不支持压缩
 - 尚不支持 fsverity
 - 强烈假定 I/O 应像XFS 上那样工作
 - iomap **真的**适用于非正则文件数据吗？

欢迎提供补丁
