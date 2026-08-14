
## ext4 通用信息


Ext4 是 ext3 文件系统的高级版本，它结合了可扩展性和可靠性增强，以支持大文件系统（64 位），从而顺应不断增长的磁盘容量和最先进的特性需求。

邮件列表：	linux-ext4@vger.kernel.org
网站：	http://ext4.wiki.kernel.org


## 快速使用说明


注意：关于 ext4 入门的更详尽信息，可在 ext4 wiki 站点找到，其 URL 为：
http://ext4.wiki.kernel.org/index.php/Ext4_Howto

  - e2fsprogs 的最新版本可在以下位置找到：

    https://www.kernel.org/pub/linux/kernel/people/tytso/e2fsprogs/

	或

    http://sourceforge.net/project/showfiles.php?group_id=2406

	或从以下位置获取最新的 git 仓库：

   https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git

  - 使用 ext4 文件系统类型创建一个新文件系统：

        # mke2fs -t ext4 /dev/hda1

    或者，将已有的 ext3 文件系统配置为支持 extent（区）：

	# tune2fs -O extents /dev/hda1

    如果文件系统是以 128 字节 inode 创建的，可以通过如下方式转换为使用 256 字节以提高效率：

        # tune2fs -I 256 /dev/hda1

  - 挂载：

	# mount -t ext4 /dev/hda1 /wherever

  - 将性能与其他文件系统比较时，尝试多种工作负载始终很重要；往往工作负载参数的细微变化会彻底改变各文件系统之间表现优劣的排名。与 ext3 比较时，请注意 ext4 默认启用写入屏障（write barrier），而 ext3 默认不启用写入屏障。因此，为了公平比较，对 ext3 和 ext4 文件系统都通过 '-o barriers=[0|1]' 挂载选项显式指定是否启用屏障是很有用的。在为最佳基准测试数据调优 ext3 时，尝试更改数据日志模式通常值得一做；对某些工作负载而言 '-o data=writeback' 可能更快。（但请注意，以 data=writeback 挂载运行，在意外断电的情况下可能使最近写入文件中的陈旧数据暴露出来，在某些情形下这会成为安全隐患。）为文件系统配置一个大的日志，对元数据密集型工作负载也有帮助。

## 特性


### 当前可用


- 能够使用 > 16TB 的文件系统（e2fsprogs 支持尚不可用）
- extent 格式减少了元数据开销（RAM、访问 IO、事务）
- extent 格式由于存在魔数（magic）和树内部的冗余，在面对磁盘损坏时更加健壮
- 改进的文件分配（多块分配，multi-block alloc）
- 解除了由 i_links_count[^1^] 施加的 32000 子目录限制
- mtime、atime、ctime、创建时间的纳秒级时间戳
- 磁盘上的 inode 版本字段（NFSv4、Lustre）
- 通过 uninit_bg 特性缩短 e2fsck 时间
- 日志校验和，用于健壮性和性能
- 持久的文件预分配（例如用于流媒体、数据库）
- 通过 flex_bg 特性将位图和 inode 表打包到更大的虚拟组中的能力
- 大文件支持
- 通过 flex_bg 使用大虚拟块组进行 inode 分配
- 延迟分配（delayed allocation）
- 大块（最大可达页大小）支持
- JBD2 和 ext4 中高效的新有序模式（避免使用缓冲头来强制顺序）
- 大小写不敏感的文件名查找
- 基于文件的加密支持（fscrypt）
- 基于文件的完整性（verity）支持（fsverity）

[^1^] 块大小为 1k 的文件系统可能会受到目录哈希树最大深度为二这一限制的影响。

## 大小写不敏感的文件名查找


大小写不敏感的文件名查找特性以每个目录为基础提供支持，允许用户在同一个文件系统中混合使用大小写不敏感和大小写敏感的目录。它通过翻转一个空目录的 +F inode 属性来启用。大小写不敏感的字符串匹配操作，仅在我们已知文本如何编码为一个字节序列时才有定义。因此，为了启用大小写不敏感的目录，文件系统必须具备 casefold 特性，该特性存储了文件系统范围内使用的编码模型。默认采用的字符集是 Unicode 的最新版本（撰写本文时为 12.1.0），以 UTF-8 形式编码。比较算法通过将字符串规范化为 Unicode 定义的规范分解（Canonical decomposition）形式来实现，随后进行逐字节比较。

大小写感知在磁盘上是保留名称的，这意味着用户空间提供的文件名与实际写入磁盘的文件名逐字节匹配。因此，内核使用的 Unicode 规范化格式是一种内部表示，不会暴露给用户空间或磁盘，唯一的例外是用于带有 DX 特性的大型大小写不敏感目录的磁盘哈希。在 DX 目录上，哈希必须使用文件名经过 casefold 后的版本来计算，这意味着实际使用的规范化格式确实会影响目录项存储的位置。

当我们将文件名从视为不透明的字节序列转变为视为编码字符串时，需要解决当某个程序试图创建一个带有无效名称的文件时会发生什么。内核内的 Unicode 子系统将这种情况下如何处理的决定权留给文件系统，文件系统通过启用/禁用严格（strict）模式来选择其偏好的行为。当 Ext4 遇到这类字符串、而文件系统并未要求严格模式时，它会回退到将整个字符串视为不透明的字节序列，这仍允许用户操作该文件，但大小写不敏感的查找将不起作用。

## 挂载选项


挂载 ext4 文件系统时，接受以下选项：
(*) == 默认值

  ro
        以只读方式挂载文件系统。注意，即使以“只读”方式挂载，ext4 仍会重放日志（从而写入该分区）。可以使用挂载选项 "ro,noload" 来阻止对文件系统的写入。

  journal_checksum
        启用日志事务的校验和。这将允许 e2fsck 和内核中的恢复代码检测到内核中的损坏。这是一个兼容的改动，较旧的内核会忽略它。

  journal_async_commit
        提交块可以在不等待描述符块的情况下写入磁盘。如果启用，较旧的内核将无法挂载该设备。这会在内部启用 'journal_checksum'。

  journal_path=path, journal_dev=devnum
        当外部日志设备的主/次设备号发生变化时，这些选项允许用户指定新的日志位置。日志设备通过 devnum 中编码的新的主/次设备号，或通过指向该设备的路径来标识。

  norecovery, noload
        挂载时不加载日志。注意，如果文件系统不是干净卸载的，跳过日志重放将导致文件系统包含不一致，从而引发各种各样的问题。

  data=journal
        所有数据在写入主文件系统之前都会提交到日志中。启用此模式将禁用延迟分配和 O_DIRECT 支持。

  data=ordered	(*)
        所有数据在其元数据提交到日志之前，被强制直接写出到主文件系统中。

  data=writeback
        不保留数据顺序，数据可能在其元数据已提交到日志之后才写入主文件系统。

  commit=nrsec	(*)
        此设置将运行中的事务的最大存在时间限制为 'nrsec' 秒。默认值为 5 秒。这意味着如果你断电，你最多会丢失最近 5 秒的元数据更改（不过由于日志的存在，你的文件系统不会损坏）。这个默认值（或任何较低的值）会损害性能，但对数据安全有好处。将其设为 0 与保持默认值（5 秒）效果相同。将其设为非常大的值将提升性能。请注意，由于延迟分配，即使更旧的数据也可能在断电时丢失，因为这些数据的回写直到 /proc/sys/vm/dirty_expire_centisecs 中设置的时间之后才开始。

  barrier=<0|1(**)>, barrier(**), nobarrier
        这启用/禁用 jbd 代码中使用写入屏障。barrier=0 禁用，barrier=1 启用。这还需要一个能够支持屏障的 IO 栈，如果 jbd 在屏障写入时遇到错误，它将再次禁用并给出警告。写入屏障强制日志提交在磁盘上有正确的顺序，使得易失性磁盘写缓存可以安全使用，代价是一定的性能损失。如果你的磁盘以某种方式由电池后备，禁用屏障可能会安全地提升性能。挂载选项 "barrier" 和 "nobarrier" 也可用于启用或禁用屏障，以与其他 ext4 挂载选项保持一致。

  inode_readahead_blks=n
        此调优参数控制 ext4 的 inode 表预读算法预读入缓冲缓存的 inode 表块的最大数量。默认值为 32 个块。

  bsddf	(*)
        让 'df' 的行为像 BSD。

  minixdf
        让 'df' 的行为像 Minix。

  debug
        将额外的调试信息发送到 syslog。

  abort
        出于调试目的，模拟调用 ext4_abort() 的效果。这通常用于在重新挂载一个已经挂载的文件系统时使用。

  errors=remount-ro
        出错时将文件系统重新挂载为只读。

  errors=continue
        遇到文件系统错误时继续运行。

  errors=panic
        如果发生错误则触发 panic 并停止机器。（这些挂载选项会覆盖超级块中指定的错误行为，后者可使用 tune2fs 配置）

  data_err=ignore(*)
        如果文件数据缓冲区中发生错误，仅打印一条错误消息。

  data_err=abort
        如果文件数据缓冲区中发生错误，则中止日志。

  grpid | bsdgroups
        新对象的组 ID 为其父对象的组 ID。

  nogrpid (*) | sysvgroups
        新对象的组 ID 为其创建者的组 ID。

  resgid=n
        可以使用保留块的组 ID。

  resuid=n
        可以使用保留块的用户 ID。

  sb=
        在此位置使用备用的超级块。

  quota, noquota, grpquota, usrquota
        这些选项被文件系统忽略。它们仅被配额（quota）工具用来识别应当开启配额的卷。更多细节请参阅 quota-tools 包中的文档（http://sourceforge.net/projects/linuxquota）。

  jqfmt=<quota type>, usrjquota=<file>, grpjquota=<file>
        这些选项向文件系统提供有关配额的细节，以便在日志重放期间正确更新配额信息。它们取代上面的配额选项。更多细节请参阅 quota-tools 包中的文档（http://sourceforge.net/projects/linuxquota）。

  stripe=n
        mballoc 尝试用于分配大小和对齐的文件系统块数量。对于 RAID5/6 系统，这应当是数据磁盘数量 * 文件系统块中的 RAID 条带大小。

  delalloc	(*)
        将块分配推迟到 ext4 即将写出相关块之前。这使 ext4 能够更高效地做出分配决策。

  nodelalloc
        禁用延迟分配。当数据从用户空间复制到页缓存时分配块，无论是通过 write(2) 系统调用，还是在之前未分配的 mmap'ed 页面首次被写入时。

  max_batch_time=usec
        ext4 应当等待将额外文件系统操作与一次同步写操作批处理在一起的最长时间。由于一次同步写操作将强制一次提交并随后等待 I/O 完成，这代价不大，并且可能带来巨大的吞吐量提升，因此我们会等待一小段时间，看看是否有其他事务能搭上这次同步写的便车。所使用的算法旨在通过测量（平均而言）完成一次事务提交所需的时间，来为磁盘速度自动调优。将此时间称为“提交时间”。如果事务已经运行的时间小于提交时间，ext4 将尝试休眠一个提交时间，看看是否有其他操作会加入该事务。提交时间受 max_batch_time 上限限制，默认为 15000us（15ms）。通过将 max_batch_time 设为 0，可以完全关闭此优化。

  min_batch_time=usec
        此参数将提交时间（如上所述）设为至少为 min_batch_time。它默认为零微秒。提高此参数可能会提升在极快磁盘上多线程同步工作负载的吞吐量，代价是增加延迟。

  journal_ioprio=prio
        kjournald2 在提交操作期间提交的 I/O 操作应使用的 I/O 优先级（从 0 到 7，其中 0 为最高优先级）。默认为 3，略高于默认 I/O 优先级。

  auto_da_alloc(*), noauto_da_alloc
        许多有缺陷的应用在通过诸如 fd = open("foo.new")/write(fd,..)/close(fd)/rename("foo.new", "foo") 这样的模式替换现有文件时，并不使用 fsync()，更糟的是，fd = open("foo", O_TRUNC)/write(fd,..)/close(fd)。如果启用了 auto_da_alloc，ext4 将检测到通过 rename 替换和通过 truncate 替换这两种模式，并强制在任何延迟分配块被这样分配：在下次日志提交时，在默认的 data=ordered 模式下，新文件的数据块会在 rename() 操作提交之前被强制写入磁盘。这提供了大致与 ext3 同等级别的保证，并避免了在延迟分配块被强制写入磁盘之前系统崩溃时可能出现的“零长度”问题。

  noinit_itable
        不在后台初始化任何未初始化的 inode 表块。此特性可被安装光盘使用，以便安装过程能尽快完成；inode 表初始化过程随后会推迟到下次卸载该文件系统时进行。

  init_itable=n
        惰性 itable 初始化代码将等待 n 倍于清零上一个块组的 inode 表所花费的毫秒数。这将在文件系统的 inode 表被初始化时，将对系统性能的影响降到最低。

  discard, nodiscard(*)
        控制在块被释放时，ext4 是否向底层块设备发出 discard/TRIM 命令。这对 SSD 设备和稀疏/精简配置的 LUN 很有用，但在完成充分测试之前默认是关闭的。

  nouid32
        禁用 32 位 UID 和 GID。这是为了与只存储并期望 16 位值的较旧内核互操作。

  block_validity(*), noblock_validity
        这些选项启用或禁用内核内用于在内部分数据结构中跟踪文件系统元数据块的功能。这允许多块分配器和其他例程注意到导致分配的块与文件系统元数据块重叠的错误或损坏的分配位图。

  dioread_lock, dioread_nolock
        控制 ext4 是否应使用 DIO 读锁。如果指定了 dioread_nolock 选项，ext4 会在缓冲写入之前分配未初始化的 extent，并在 IO 完成后将该 extent 转换为已初始化。这种方式让 ext4 代码避免使用 inode 互斥锁，从而提高了高速存储上的可扩展性。然而这不适用于数据日志，并且 dioread_nolock 选项会被忽略并给出内核警告。注意，dioread_nolock 代码路径仅用于基于 extent 的文件。由于此选项带来的限制，它默认是关闭的（例如使用 dioread_lock）。

  max_dir_size_kb=n
        这限制了目录的大小，使任何将其扩展到超过指定千字节限制之外的尝试都会导致 ENOSPC 错误。这在内存受限的环境中很有用，因为非常大的目录会导致严重的性能问题，甚至触发内存耗尽（Out Of Memory）杀手。（例如，如果只有 512mb 内存可用，一个 176mb 的目录可能会严重拖累系统的运行。）

  i_version
        启用 64 位 inode 版本支持。此选项默认关闭。

  dax
        使用直接访问（无页缓存）。请参阅 Documentation/filesystems/dax.rst。注意，此选项与 data=journal 不兼容。

  inlinecrypt
        在可能的情况下，使用 blk-crypto 框架而非文件系统层加密来对加密文件的内容进行加/解密。这允许使用内联加密硬件。磁盘上的格式不受影响。更多细节请参阅 Documentation/block/inline-encryption.rst。

## 数据模式


有三种不同的数据模式：

- writeback 模式

  在 data=writeback 模式下，ext4 完全不日志数据。此模式提供与 XFS 和 JFS 在其默认模式（元数据日志）下相同级别的日志。崩溃+恢复可能导致在崩溃前不久写入的文件出现不正确的数据。此模式通常提供最佳的 ext4 性能。

- ordered 模式

  在 data=ordered 模式下，ext4 只正式日志元数据，但它将与被更改数据相关的元数据信息，与数据块在逻辑上组合成一个称为事务（transaction）的单元。当需要将新元数据写出到磁盘时，相关的数据块先被写入。一般而言，此模式性能略慢于 writeback，但显著快于 journal 模式。

- journal 模式

  data=journal 模式提供完整的数据和元数据日志。所有新数据先写入日志，然后再写到其最终位置。在发生崩溃时，可以重放日志，将数据与元数据都带入一致状态。此模式是最慢的，除非数据需要同时被从磁盘读取和写入磁盘，此时它的表现优于所有其他模式。启用此模式将禁用延迟分配和 O_DIRECT 支持。

## /proc 条目


关于已挂载 ext4 文件系统的信息可以在 /proc/fs/ext4 中找到。每个已挂载的文件系统都会在 /proc/fs/ext4 中有一个以其设备名命名的目录（即 /proc/fs/ext4/hdc 或 /proc/fs/ext4/dm-0）。每个每设备目录中的文件如下表所示。

/proc/fs/ext4/<devname> 中的文件

  mb_groups
        多块分配器空闲块的伙伴（buddy）缓存的细节

## /sys 条目


关于已挂载 ext4 文件系统的信息可以在 /sys/fs/ext4 中找到。每个已挂载的文件系统都会在 /sys/fs/ext4 中有一个以其设备名命名的目录（即 /sys/fs/ext4/hdc 或 /sys/fs/ext4/dm-0）。每个每设备目录中的文件如下表所示。

/sys/fs/ext4/<devname> 中的文件：

（另请参阅 Documentation/ABI/testing/sysfs-fs-ext4）

  delayed_allocation_blocks
        此文件为只读，显示页缓存中尚未分配其在文件系统中位置的脏块数量。

  inode_goal
        调优参数，如果非零，则控制 inode 分配器优先使用的目标 inode，优先于所有其他分配启发式。这仅用于调试，在生产系统上应为 0。

  inode_readahead_blks
        调优参数，控制 ext4 的 inode 表预读算法预读入缓冲缓存的 inode 表块的最大数量。

  lifetime_write_kbytes
        此文件为只读，显示自该文件系统创建以来写入其中的数据千字节数。

  max_writeback_mb_bump
        回写代码在转去处理另一个 inode 之前，尝试写出的兆字节数的最大值。

  mb_group_prealloc
        如果 ext4 超级块中未设置条带大小，多块分配器会将分配请求向上取整到此调优参数的倍数。

  mb_max_to_scan
        多块分配器为找到最佳 extent 而将搜索的最大 extent 数量。

  mb_min_to_scan
        多块分配器为找到最佳 extent 而将搜索的最小 extent 数量。

  mb_order2_req
        控制使用伙伴缓存的请求的最小大小（以 2 的幂计）的调优参数。

  mb_stats
        控制多块分配器是否应收集统计信息，这些统计信息在卸载时显示。1 表示收集统计信息，0 表示不收集。

  mb_stream_req
        块数少于此可调参数的文件，其块将从特定于块组的预分配池中进行分配，从而让小文件紧密地打包在一起。每个大文件将从其自身独立的预分配池中进行块分配。

  session_write_kbytes
        此文件为只读，显示自该文件系统挂载以来写入其中的数据千字节数。

  reserved_clusters
        这是一个 RW 文件，包含文件系统中保留的簇的数量，这些簇将在特定情况下用于避免代价高昂的零化（zeroout）、意外的 ENOSPC，或可能的数据丢失。默认值为 2% 或 4096 个簇中的较小者，并且可以更改，但它绝不能超过文件系统中的簇数量。如果在挂载时没有足够的空间用于保留空间，文件挂载将*不会*失败。

## Ioctls


Ext4 实现了各种 ioctl，应用程序可使用它们来访问 ext4 特定的功能。这些 ioctl 的不完整列表如下表所示。此列表既包含真正 ext4 特定的 ioctl（`EXT4_IOC_*`），也包含最初可能是 ext4 特定、但现在也被其他某些文件系统支持的 ioctl（`FS_IOC_*`）。

Ext4 ioctl 表

  FS_IOC_GETFLAGS
        获取与 inode 关联的附加属性。ioctl 参数是一个整型位域，各位的值在 ext4.h 中描述。

  FS_IOC_SETFLAGS
        设置与 inode 关联的附加属性。ioctl 参数是一个整型位域，各位的值在 ext4.h 中描述。

  EXT4_IOC_GETVERSION, EXT4_IOC_GETVERSION_OLD
        获取为每个 inode 存储的 inode i_generation 号。i_generation 号通常仅在创建新 inode 时更改，它对网络文件系统特别有用。此 ioctl 的 '_OLD' 版本是 FS_IOC_GETVERSION 的别名。

  EXT4_IOC_SETVERSION, EXT4_IOC_SETVERSION_OLD
        设置为每个 inode 存储的 inode i_generation 号。此 ioctl 的 '_OLD' 版本是 FS_IOC_SETVERSION 的别名。

  EXT4_IOC_GROUP_EXTEND
        此 ioctl 与 resize 挂载选项用途相同。它允许将文件系统大小调整为到最后一个现有块组的末尾，进一步的调整必须通过 resize2fs 完成，无论是在线还是离线。参数指向一个无符号长整数，表示文件系统新的块计数。

  EXT4_IOC_MOVE_EXT
        将块 extent 从 orig_fd（本 ioctl 所指向的）移动到 donor_fd（作为本 ioctl 参数传入的 move_extent 结构中所指定的）。然后，在 orig_fd 和 donor_fd 之间交换 inode 元数据。这对在线碎片整理特别有用，因为分配器有机会更好地分配被移动的块，理想情况下将它们合并为一个连续的 extent。

  EXT4_IOC_GROUP_ADD
        向现有或新的组描述符块添加一个新的组描述符。新的组描述符由 ext4_new_group_input 结构描述，作为本 ioctl 的参数传入。这与 EXT4_IOC_GROUP_EXTEND 结合使用时特别有用，后者允许将文件系统在线调整到最后一个现有块组的末尾。这两个 ioctl 结合用于用户空间在线调整大小工具（例如 resize2fs）。

  EXT4_IOC_MIGRATE
        此 ioctl 作用于文件系统本身。它通过遍历原始 inode 的间接块映射，将连续的块范围转换为临时 inode 的 ext4 extent，从而将 ext3 间接块映射的 inode 转换（迁移）为 ext4 extent 映射的 inode。然后交换 inode。从 ext3 迁移到 ext4 文件系统时，此 ioctl 可能有帮助，不过建议创建全新的 ext4 文件系统并从备份复制数据。注意，此 ioctl 要工作，文件系统必须支持 extent。

  EXT4_IOC_ALLOC_DA_BLKS
        强制分配所有延迟分配的块，以保留应用所期望的 ext3 行为。注意，这也将开始触发数据块的写入，但此行为未来可能改变，因为它并非必要，而只是出于简单性才这样做。

  EXT4_IOC_RESIZE_FS
        将文件系统大小调整为新的大小。调整大小后的文件系统的块数通过一个 64 位整数参数传入。内核分配位图和 inode 表，因此用户空间工具只需传入新的块数。

  EXT4_IOC_SWAP_BOOT
        将指定 inode 的 i_blocks 及关联属性（如 i_blocks、i_size、i_flags 等）与 inode EXT4_BOOT_LOADER_INO（#5）交换。这通常用于将引导加载程序存储在文件系统的安全部分，使其不会被普通用户意外更改。之前引导加载程序的数据块将与给定 inode 关联。

## 参考资料


内核源码：	<file:fs/ext4/>
		<file:fs/jbd2/>

程序：	http://e2fsprogs.sourceforge.net/

有用链接：	https://fedoraproject.org/wiki/ext3-devel
		http://www.bullopensource.org/ext4/
		http://ext4.wiki.kernel.org/index.php/Main_Page
		https://fedoraproject.org/wiki/Features/Ext4
