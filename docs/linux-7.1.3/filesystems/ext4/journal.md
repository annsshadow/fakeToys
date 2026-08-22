
### 日志（jbd2

日志（journal）在 ext3 中引入，ext4 文件系统利用它来防止在系统崩溃时出现元数据不一致。最多可在文件系统内预留 10,240,000 个文件系统块（关于日志大小限制的更多细节，请参见 man mke2fs(8)）作为位置，以尽可能快地将“重要”数据写入磁盘。一旦重要的数据事务被完整写入磁盘并从磁盘写缓存中刷新，被提交数据的记录也会被写入日志。在稍后的某个时刻，日志代码会将事务写入磁盘上的最终位置（这可能涉及大量寻道或大量小的擦除操作），然后才擦除提交记录。如果在第二次缓慢写入期间系统崩溃，日志可以被重放到最新的提交记录，从而保证通过日志写入磁盘的任何内容的原子性。其效果是保证文件系统不会在元数据更新过程中卡住
出于性能原因，ext4 默认只通过日志写入文件系统元数据。这意味着文件数据块在崩溃后不保证处于任何一致状态。如果这种默认保证级别（`data=ordered`）不能令人满意，可以使用挂载选项来控制日志行为。如`data=journal`，所有数据和元数据都通过日志写入磁盘。这较慢但最安全。如`data=writeback`，则在元数据通过日志写入磁盘之前，脏数据块不会被刷新到磁盘
`data=ordered` 模式下，Ext4 还支持快速提交（fast commit），可显著减少提交延迟。默认的 `data=ordered` 模式通过把元数据块记录到日志来工作。在快速提交模式下，Ext4 仅存储在共享JBD2 的快速提交空间中重建受影响元数据所需的最小增量（delta）。一旦快速提交区域填满，或快速提交不可行，或 JBD2 提交定时器到期，Ext4 就会执行传统的完整提交。一次完整提交会使之前发生的所有快速提交失效，从而清空快速提交区域以供后续快速提交使用。此特性需要在 mkfs 时启用
日志 inode 通常inode 8。日inode 的前 68 字节会在 ext4 超级块中复制。日志本身是位于文件系统内的普通（但隐藏）文件。该文件通常占用整个块组，尽mke2fs 尝试将其放在磁盘中部
jbd2 中的所有字段都以大端序写入磁盘。这ext4 相反
注意：ext4 ocfs2 都使jbd2
内嵌ext4 文件系统中的日志的最大大小为 2^32 个块。jbd2 本身似乎并不在意
#### 布局


一般来说，日志具有如下格式
   :widths: 16 48 16
   :header-rows: 1

   - - Superblock
     - descriptor_block (data_blocks or revocation_block) [more data or
       revocations] commmit_block
     - [more transactions...]
   - -
     - 一个事     -

注意，一个事务以描述符和一些数据，或一个块撤销（revocation）列表开始。已完成的事务总是以提交（commit）结束。如果没有提交记录（或校验和不匹配），该事务在重放时会被丢弃
#### 外部日志


可选地，ext4 文件系统可以使用外部日志设备创建（与使用保留 inode 的内部日志相对）。在这种情况下，在文件系统设备上，`s_journal_inum` 应为零，`s_journal_uuid` 应被设置。在日志设备上，通常位置会有一ext4 超级块，UUID 相匹配。日志超级块位于超级块之后的下一个完整块中
   :widths: 12 12 12 32 12
   :header-rows: 1

   - - 1024 字节的填     - ext4 超级     - 日志超级     - descriptor_block (data_blocks or revocation_block) [more data or
       revocations] commmit_block
     - [more transactions...]
   - -
#      -

     - 一个事     -

#### 块头


日志中的每个块都以一个通用12 字节头部 `struct journal_header_s` 开始：

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 说明
   - - 0x0
     - __be32
     - h_magic
     - jbd2 魔数xC03B3998   - - 0x4
     - __be32
     - h_blocktype
     - 描述该块所包含的内容。参见下文的 jbd2_blocktype_ 表   - - 0x8
     - __be32
     - h_sequence
     - 与此块关联的事务 ID

日志块类型可以是以下任意一种：

   :widths: 16 64
   :header-rows: 1

   - -      - 说明
   - - 1
     - 描述符（Descriptor）。该块位于事务期间通过日志写入的一系列数据块之前   - - 2
     - 块提交记录。该块表示事务的完成   - - 3
     - 日志超级v1   - - 4
     - 日志超级v2   - - 5
     - 块撤销记录。通过让日志跳过写入随后被重写的块，从而加速恢复
#### 瓒呯骇鍧。

日志的超级块ext4 的简单得多。其中保留的关键数据是日志的大小，以及在哪里找到事务日志的起始位置
日志超级块记录为 `struct journal_superblock_s`，长度为 1024 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 说明
#    * -

     -
     - 描述该日志的静态信息   - - 0x0
     - journal_header_t (12 bytes)
     - s_header
     - 标识此为超级块的通用头部   - - 0xC
     - __be32
     - s_blocksize
     - 日志设备块大小   - - 0x10
     - __be32
     - s_maxlen
     - 本日志中的块总数   - - 0x14
     - __be32
     - s_first
     - 日志信息的第一个块#    * -

     -
     - 描述日志当前状态的动态信息   - - 0x18
     - __be32
     - s_sequence
     - 日志中期望的第一个提ID   - - 0x1C
     - __be32
     - s_start
     - 日志起始位置的块号。与注释相反，该字段为零并不意味着日志是干净的！
   - - 0x20
     - __be32
     - s_errno
     - 错误值，jbd2_journal_abort() 设置#    * -

     -
     - 其余字段仅在 v2 超级块中有效   - - 0x24
     - __be32
     - s_feature_compat;
     - 兼容特性集。参见下文的 jbd2_compat_ 表   - - 0x28
     - __be32
     - s_feature_incompat
     - 不兼容特性集。参见下文的 jbd2_incompat_ 表   - - 0x2C
     - __be32
     - s_feature_ro_compat
     - 只读兼容特性集。目前没有任何此类特性   - - 0x30
     - __u8
     - s_uuid[^16^]
     - 日志128 UUID。在挂载时会ext4 超级块中的副本进行比对   - - 0x40
     - __be32
     - s_nr_users
     - 共享此日志的文件系统数量   - - 0x44
     - __be32
     - s_dynsuper
     - 动态超级块副本的位置。（未使用？   - - 0x48
     - __be32
     - s_max_transaction
     - 每个事务的日志块数限制。（未使用？   - - 0x4C
     - __be32
     - s_max_trans_data
     - 每个事务的数据块数限制。（未使用？   - - 0x50
     - __u8
     - s_checksum_type
     - 日志使用的校验和算法。更多信息见 jbd2_checksum_type_   - - 0x51
     - __u8[^3^]
     - s_padding2
     -
   - - 0x54
     - __be32
     - s_num_fc_blocks
     - 日志中快速提交块的数量   - - 0x58
     - __be32
     - s_head
     - 日志头部（第一个未使用块）的块号，仅在日志为空时是最新的   - - 0x5C
     - __u32
     - s_padding[^40^]
     -
   - - 0xFC
     - __be32
     - s_checksum
     - 整个超级块的校验和，计算时此字段设为零   - - 0x100
     - __u8
     - s_users[16*48]
     - 所有共享该日志的文件系统的 ID。e2fsprogs/Linux 不允许共享外部日志，但我猜想使用 jbd2 代码Lustre（或 ocfs2？）可能会

日志兼容特性是以下任意项的组合
   :widths: 16 64
   :header-rows: 1

   - -      - 说明
   - - 0x1
     - 日志对数据块维护校验和。（JBD2_FEATURE_COMPAT_CHECKSUM

日志不兼容特性是以下任意项的组合
   :widths: 16 64
   :header-rows: 1

   - -      - 说明
   - - 0x1
     - 日志拥有块撤销记录。（JBD2_FEATURE_INCOMPAT_REVOKE   - - 0x2
     - 日志可处64 位块号。（JBD2_FEATURE_INCOMPAT_64BIT   - - 0x4
     - 日志异步提交。（JBD2_FEATURE_INCOMPAT_ASYNC_COMMIT   - - 0x8
     - 该日志使用磁盘上校验和格式的 v2。每个日志元数据块都有自己的校验和，且描述符表中的块标签包含日志中每个数据块的校验和。（JBD2_FEATURE_INCOMPAT_CSUM_V2   - - 0x10
     - 该日志使用磁盘上校验和格式的 v3。这v2 相同，但日志块标签大小固定，与块号大小无关。（JBD2_FEATURE_INCOMPAT_CSUM_V3   - - 0x20
     - 日志拥有快速提交块。（JBD2_FEATURE_INCOMPAT_FAST_COMMIT

日志校验和类型代码为以下之一。crc32 crc32c 是最有可能的选择
   :widths: 16 64
   :header-rows: 1

   - -      - 说明
   - - 1
     - CRC32
   - - 2
     - MD5
   - - 3
     - SHA1
   - - 4
     - CRC32C

#### 描述符块


描述符块包含一个日志块标签数组，描述日志中后续数据块的最终位置。描述符块是开放编码（open-coded）的，并非完全由某个数据结构描述，但这里仍给出块结构。描述符块至少占36 字节，但使用一个完整的块：

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 鎻忚堪绗?   - - 0x0
     - journal_header_t
     - (open coded)
     - 通用块头   - - 0xC
     - struct journal_block_tag_s
     - open coded array[]
     - 足够数量的标签，以填满该块，或描述此描述符块之后的所有数据块
日志块标签具有以下任意一种格式，取决于设置了哪个日志特性和块标签标志
若设置了 JBD2_FEATURE_INCOMPAT_CSUM_V3，则日志块标签定义为 `struct journal_block_tag3_s`，如下所示。大小为 16 32 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 鎻忚堪绗?   - - 0x0
     - __be32
     - t_blocknr
     - 对应数据块在磁盘上最终位置的32 位   - - 0x4
     - __be32
     - t_flags
     - 与描述符一起使用的标志。更多信息见 jbd2_tag_flags_ 表   - - 0x8
     - __be32
     - t_blocknr_high
     - 对应数据块在磁盘上最终位置的32 位。若未启JBD2_FEATURE_INCOMPAT_64BIT，则为零   - - 0xC
     - __be32
     - t_checksum
     - 日志 UUID、序列号和数据块的校验和#    * -

     -
     - 该字段似乎是开放编码的。它总是出现在标签末尾、t_checksum 之后。如果设置了“same UUID”标志，则该字段不存在   - - 0x8 0xC
     - char
     - uuid[^16^]
     - 与该标签一起使用的 UUID。该字段似乎`struct journal_s` `j_uuid` 字段复制而来，但只有 tune2fs 会触及该字段

日志标签标志是以下任意项的组合：

   :widths: 16 64
   :header-rows: 1

   - -      - 说明
   - - 0x1
     - 磁盘上的块被转义（escaped）。数据块的前四个字节恰好jbd2 魔数匹配   - - 0x2
     - 该块与前一个块具有相同UUID，因此省UUID 字段   - - 0x4
     - 数据块已被事务删除。（未使用？   - - 0x8
     - 这是此描述符块中的最后一个标签
若未设置 JBD2_FEATURE_INCOMPAT_CSUM_V3，则日志块标签定义为 `struct journal_block_tag_s`，如下所示。大小为 824 28 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 鎻忚堪绗?   - - 0x0
     - __be32
     - t_blocknr
     - 对应数据块在磁盘上最终位置的32 位   - - 0x4
     - __be16
     - t_checksum
     - 日志 UUID、序列号和数据块的校验和。注意只存储16 位   - - 0x6
     - __be16
     - t_flags
     - 与描述符一起使用的标志。更多信息见 jbd2_tag_flags_ 表#    * -

     -
     - 下一个字段仅当超级块表示支持 64 位块号时才存在   - - 0x8
     - __be32
     - t_blocknr_high
     - 对应数据块在磁盘上最终位置的32 位#    * -

     -
     - 该字段似乎是开放编码的。它总是出现在标签末尾、t_flags t_blocknr_high 之后。如果设置了“same UUID”标志，则该字段不存在   - - 0x8 0xC
     - char
     - uuid[^16^]
     - 与该标签一起使用的 UUID。该字段似乎`struct journal_s` `j_uuid` 字段复制而来，但只有 tune2fs 会触及该字段
若设置了 JBD2_FEATURE_INCOMPAT_CSUM_V2 JBD2_FEATURE_INCOMPAT_CSUM_V3，则块的末尾是一`struct jbd2_journal_block_tail`，如下所示：

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 鎻忚堪绗?   - - 0x0
     - __be32
     - t_checksum
     - 日志 UUID 与描述符块的校验和，计算时此字段设为零
#### 鏁版嵁鍧。

一般来说，通过日志写入磁盘的数据块在描述符块之后逐字写入日志文件。但是，如果块的前四个字节与 jbd2 魔数匹配，那么这四个字节会被替换为零，并在描述符块标签中设置“escaped”标志
#### 鎾ら攢鍧。

撤销块用于防止重放更早事务中的某个块。它用于标记曾经被记录到日志、但不再被记录到日志的块。典型情况是：一个元数据块被释放并重新分配为文件数据块；此时，在文件块写入磁盘后进行的日志重放会导致损坏
**注意**：该机制不用于表示“此日志块被另一日志块取代”，正如作者（djwong）曾误以为的那样。任何被加入事务的块都会导致移除该块的所有现有撤销记录
撤销块由 `struct jbd2_journal_revoke_header_s` 描述，长度至少为 16 字节，但使用一个完整的块：

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 说明
   - - 0x0
     - journal_header_t
     - r_header
     - 通用块头   - - 0xC
     - __be32
     - r_count
     - 该块中已使用的字节数   - - 0x10
     - __be32 or __be64
     - blocks[^0^]
     - 要撤销的块
r_count 之后是一个块号线性数组，这些块号被此事务有效撤销。如果超级块声明支持 64 位块号，则每个块号大小为 8 字节，否则为 4 字节
若设置了 JBD2_FEATURE_INCOMPAT_CSUM_V2 JBD2_FEATURE_INCOMPAT_CSUM_V3，则撤销块的末尾是一`struct jbd2_journal_revoke_tail`，其格式如下
   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 说明
   - - 0x0
     - __be32
     - r_checksum
     - 日志 UUID 与撤销块的校验
#### 鎻愪氦鍧。

提交块是一个哨兵（sentry），表示事务已完整写入日志。一旦该提交块到达日志，与此事务一起存储的数据就可以被写入磁盘上的最终位置
提交块由 `struct commit_header` 描述，长度为 32 字节（但使用一个完整的块）
   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移     - 类型
     - 名称
     - 鎻忚堪绗?   - - 0x0
     - journal_header_s
     - (open coded)
     - 通用块头   - - 0xC
     - unsigned char
     - h_chksum_type
     - 用于验证事务中数据块完整性的校验和类型。更多信息见 jbd2_checksum_type_   - - 0xD
     - unsigned char
     - h_chksum_size
     - 校验和使用的字节数。很可能4   - - 0xE
     - unsigned char
     - h_padding[^2^]
     -
   - - 0x10
     - __be32
     - h_chksum[JBD2_CHECKSUM_BYTES]
     - 用于存储校验和的 32 字节空间。如果设置了 JBD2_FEATURE_INCOMPAT_CSUM_V2 JBD2_FEATURE_INCOMPAT_CSUM_V3，第一`__be32` 是日UUID 与整个提交块的校验和，计算时此字段设为零。如果设置了 JBD2_FEATURE_COMPAT_CHECKSUM，第一`__be32` 是已写入该事务的所有块crc32   - - 0x30
     - __be64
     - h_commit_sec
     - 事务被提交的时间，自纪元起的秒数   - - 0x38
     - __be32
     - h_commit_nsec
     - 上述时间戳的纳秒部分
#### 快速提

快速提交区域组织为标签长度值的日志。每TLV 开头有一`struct ext4_fc_tl`，存储标签和整个字段的长度。其后跟随可变长度的标签特定值。以下是受支持标签及其含义的列表
   :widths: 8 20 20 32
   :header-rows: 1

   - - 标签
     - 含义
     - 值结     - 说明
   - - EXT4_FC_TAG_HEAD
     - 快速提交区域头
     - `struct ext4_fc_head`
     - 存储应在其之后应用这些快速提交的事务TID   - - EXT4_FC_TAG_ADD_RANGE
     - inode 添加 extent
     - `struct ext4_fc_add_range`
     - 存储 inode 号以及要添加到该 inode extent
   - - EXT4_FC_TAG_DEL_RANGE
     - 移除 inode 的逻辑偏移范围
     - `struct ext4_fc_del_range`
     - 存储 inode 号以及需要移除的逻辑偏移范围
   - - EXT4_FC_TAG_CREAT
     - 为新建文件创建目录项
     - `struct ext4_fc_dentry_info`
     - 存储新建文件的父 inode 号、inode 号和目录   - - EXT4_FC_TAG_LINK
     - 将目录项链接到一inode
     - `struct ext4_fc_dentry_info`
     - 存储inode 号、inode 号和目录   - - EXT4_FC_TAG_UNLINK
     - 解除 inode 的某个目录项的链     - `struct ext4_fc_dentry_info`
     - 存储inode 号、inode 号和目录
   - - EXT4_FC_TAG_PAD
     - 填充（未使用区域     - None
     - 快速提交区域中未使用的字节
   - - EXT4_FC_TAG_TAIL
     - 标记快速提交的结束
     - `struct ext4_fc_tail`
     - 存储提交TID，以及本标签所代表结束的快速提交的 CRC

#### 快速提交重放的幂等

只要恢复代码遵循某些规则，快速提交标签本质上是幂等的。提交路径在提交时遵循的指导原则是：它存储特定操作的结果，而不是存储过程
让我们考虑这个重命名操作：'mv /a /b'。假设目录项 '/a' 关联的是 inode 10。在快速提交期间，我们不将此操作存储为一个过程“rename a to b”，而是将产生的文件系统状态存储为一系列结果
- 将目录项 b 链接inode 10
- 解除目录a 的链- inode 10 具有有效的引用计
现在当恢复代码运行时，它需要在文件系统上“强制实施”这一状态。这正是保证快速提交重放幂等性的原因
让我们以非幂等过程为例，看看快速提交如何使其变为幂等。考虑以下操作序列
1) 删除 A
2) B 重命名为 A
3) 读取 A

如果我们原样存储这一操作序列，那么重放就不是幂等的。假设在重放期间，我们在 (2) 之后崩溃。在第二次重放时，文A（实际上是由“mv B A”操作创建的）会被删除。因此，当我们尝试读A 时，名为 A 的文件将不存在。所以，这一操作序列不是幂等的。然而如上所述，快速提交存储的不是过程，而是每个过程的结果。因此，上述过程的快速提交日志如下：

（假设在重放前，目录A 链接inode 10，目录项 B 链接inode 11
1) 解除 A 的链2) A 链接inode 11
3) 解除 B 的链4) inode 11

如果我们(3) 之后崩溃，我们将得到链接inode 11 的文A。在第二次重放时，我们会移除文件 A（inode 11）。但我们会重新创建它并使其指inode 11。我们找不到 B，因此跳过那一步。此时，inode 11 的引用计数不可靠，但这会在最后一inode 11 标签的重放中被修正。因此，通过将非幂等过程转换为一系列幂等结果，快速提交保证了重放期间的幂等性
#### 日志检查点


对日志进行（checkpointing）可确保所有事务及其关联的缓冲区都被提交到磁盘。正在进行的的事务会被等待并包含在检查点中。检查点在文件系统关键更新期间内部使用，包括日志恢复、文件系统大小调整以journal_t 结构的释放
可以通过 ioctl EXT4_IOC_CHECKPOINT 从用户空间触发日志检查点。该 ioctl 接受一个单一u64 标志参数。目前支持三个标志。首先，EXT4_IOC_CHECKPOINT_FLAG_DRY_RUN 可用于验ioctl 的输入。如果存在任何无效输入，它会返回错误，否则在不执行任何检查点操作的情况下返回成功。这可用于检查系统上是否存在ioctl，并验证参数或标志没有问题。另外两个标志是 EXT4_IOC_CHECKPOINT_FLAG_DISCARD EXT4_IOC_CHECKPOINT_FLAG_ZEROOUT。这两个标志分别使日志块在检查点完成后被丢弃或填零。EXT4_IOC_CHECKPOINT_FLAG_DISCARD EXT4_IOC_CHECKPOINT_FLAG_ZEROOUT 不能同时设置。在系统快照或遵守内容删SLO 时，ioctl 可能很有用