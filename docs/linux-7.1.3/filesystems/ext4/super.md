
### 超级块


超级块记录了所在文件系统的各种信息，例如块计数、inode 计数、支持的特性、维护信息等。

若设置了 sparse_super 特性标志，则超级块和块组描述符的冗余副本仅保存在组号为 0 或 3、5、7 的幂的块组中。若未设置该标志，冗余副本保存在所有块组中。

超级块的校验和针对超级块结构进行计算，该结构包含文件系统 UUID。

ext4 超级块的布局在 `struct ext4_super_block` 中如下：

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移量
     - 大小
     - 名称
     - 说明
   - - 0x0
     - __le32
     - s_inodes_count
     - 总 inode 数。
   - - 0x4
     - __le32
     - s_blocks_count_lo
     - 总块数。
   - - 0x8
     - __le32
     - s_r_blocks_count_lo
     - 这些数量的块只能由超级用户分配。
   - - 0xC
     - __le32
     - s_free_blocks_count_lo
     - 空闲块数。
   - - 0x10
     - __le32
     - s_free_inodes_count
     - 空闲 inode 数。
   - - 0x14
     - __le32
     - s_first_data_block
     - 第一个数据块。对于 1KiB 块的文件系统，该值至少为 1；对于其他所有块大小，通常为 0。
   - - 0x18
     - __le32
     - s_log_block_size
     - 块大小为 2 ^ (10 + s_log_block_size)。
   - - 0x1C
     - __le32
     - s_log_cluster_size
     - 若启用了 bigalloc，则簇大小为 2 ^ (10 + s_log_cluster_size) 个块。否则 s_log_cluster_size 必须等于 s_log_block_size。
   - - 0x20
     - __le32
     - s_blocks_per_group
     - 每组块数。
   - - 0x24
     - __le32
     - s_clusters_per_group
     - 每组簇数，若启用了 bigalloc。否则 s_clusters_per_group 必须等于 s_blocks_per_group。
   - - 0x28
     - __le32
     - s_inodes_per_group
     - 每组 inode 数。
   - - 0x2C
     - __le32
     - s_mtime
     - 挂载时间，自纪元起的秒数。
   - - 0x30
     - __le32
     - s_wtime
     - 写入时间，自纪元起的秒数。
   - - 0x34
     - __le16
     - s_mnt_count
     - 自上次 fsck 以来的挂载次数。
   - - 0x36
     - __le16
     - s_max_mnt_count
     - 超过该挂载次数就需要进行 fsck。
   - - 0x38
     - __le16
     - s_magic
     - 魔数签名，0xEF53
   - - 0x3A
     - __le16
     - s_state
     - 文件系统状态。更多信息见 super_state_。
   - - 0x3C
     - __le16
     - s_errors
     - 检测到错误时的行为。更多信息见 super_errors_。
   - - 0x3E
     - __le16
     - s_minor_rev_level
     - 次版本号。
   - - 0x40
     - __le32
     - s_lastcheck
     - 上次检查的时间，自纪元起的秒数。
   - - 0x44
     - __le32
     - s_checkinterval
     - 两次检查之间的最大时间间隔，单位为秒。
   - - 0x48
     - __le32
     - s_creator_os
     - 创建文件系统时的操作系统。更多信息见 super_creator_ 表。
   - - 0x4C
     - __le32
     - s_rev_level
     - 版本号。更多信息见 super_revision_ 表。
   - - 0x50
     - __le16
     - s_def_resuid
     - 保留块的默认 uid。
   - - 0x52
     - __le16
     - s_def_resgid
     - 保留块的默认 gid。
#    * -

     -
     - 这些字段仅适用于 EXT4_DYNAMIC_REV 超级块。

       注意：兼容特性集与不兼容特性集的区别在于，如果内核不认识不兼容特性集中的某个被置位的位，它应该拒绝挂载该文件系统。

       e2fsck 的要求更为严格；如果它不认识兼容或不兼容特性集中的某个特性，它必须中止，不去尝试改动它不理解的内容……
   - - 0x54
     - __le32
     - s_first_ino
     - 第一个非保留 inode。
   - - 0x58
     - __le16
     - s_inode_size
     - inode 结构的大小，单位为字节。
   - - 0x5A
     - __le16
     - s_block_group_nr
     - 本超级块所在的块组编号。
   - - 0x5C
     - __le32
     - s_feature_compat
     - 兼容特性集标志。即使内核不理解某个标志，仍可读写该文件系统；fsck 则不应如此。更多信息见 super_compat_ 表。
   - - 0x60
     - __le32
     - s_feature_incompat
     - 不兼容特性集。如果内核或 fsck 不理解其中某个位，应停止挂载。更多信息见 super_incompat_ 表。
   - - 0x64
     - __le32
     - s_feature_ro_compat
     - 只读兼容特性集。如果内核不理解其中某个位，仍可以只读方式挂载。更多信息见 super_rocompat_ 表。
   - - 0x68
     - __u8
     - s_uuid[^16^]
     - 卷的 128 位 UUID。
   - - 0x78
     - char
     - s_volume_name[^16^]
     - 卷标。
   - - 0x88
     - char
     - s_last_mounted[^64^]
     - 文件系统上次挂载的目录。
   - - 0xC8
     - __le32
     - s_algorithm_usage_bitmap
     - 用于压缩（e2fsprogs/Linux 中未使用）
#    * -

     -
     - 性能提示。只有当 EXT4_FEATURE_COMPAT_DIR_PREALLOC 标志开启时，才应进行目录预分配。
   - - 0xCC
     - __u8
     - s_prealloc_blocks
     - 尝试为……文件预分配的块数？（e2fsprogs/Linux 中未使用）
   - - 0xCD
     - __u8
     - s_prealloc_dir_blocks
     - 为目录预分配的块数。（e2fsprogs/Linux 中未使用）
   - - 0xCE
     - __le16
     - s_reserved_gdt_blocks
     - 为文件系统未来扩展保留的 GDT 条目数。
#    * -

     -
     - 日志支持仅在设置了 EXT4_FEATURE_COMPAT_HAS_JOURNAL 时有效。
   - - 0xD0
     - __u8
     - s_journal_uuid[^16^]
     - 日志超级块的 UUID
   - - 0xE0
     - __le32
     - s_journal_inum
     - 日志文件的 inode 号。
   - - 0xE4
     - __le32
     - s_journal_dev
     - 日志文件的设备号，若设置了外部日志特性标志。
   - - 0xE8
     - __le32
     - s_last_orphan
     - 待删除孤立 inode 列表的起始位置。
   - - 0xEC
     - __le32
     - s_hash_seed[^4^]
     - HTREE 哈希种子。
   - - 0xFC
     - __u8
     - s_def_hash_version
     - 用于目录哈希的默认哈希算法。更多信息见 super_def_hash_。
   - - 0xFD
     - __u8
     - s_jnl_backup_type
     - 若该值为 0 或 EXT3_JNL_BACKUP_BLOCKS (1)，则 `s_jnl_blocks` 字段包含该 inode 的 `i_block[]` 数组与 `i_size` 的副本。
   - - 0xFE
     - __le16
     - s_desc_size
     - 块组描述符的大小（字节），若启用了 64bit 不兼容特性标志。
   - - 0x100
     - __le32
     - s_default_mount_opts
     - 默认挂载选项。更多信息见 super_mountopts_ 表。
   - - 0x104
     - __le32
     - s_first_meta_bg
     - 第一个元块块组，若启用了 meta_bg 特性。
   - - 0x108
     - __le32
     - s_mkfs_time
     - 文件系统创建时间，自纪元起的秒数。
   - - 0x10C
     - __le32
     - s_jnl_blocks[^17^]
     - 日志 inode 的 `i_block[]` 数组的前 15 个元素备份，以及 i_size_high 和 i_size 分别位于第 16 和 17 个元素。
#    * -

     -
     - 64bit 支持仅在设置了 EXT4_FEATURE_COMPAT_64BIT 时有效。
   - - 0x150
     - __le32
     - s_blocks_count_hi
     - 块计数的高 32 位。
   - - 0x154
     - __le32
     - s_r_blocks_count_hi
     - 保留块计数的高 32 位。
   - - 0x158
     - __le32
     - s_free_blocks_count_hi
     - 空闲块计数的高 32 位。
   - - 0x15C
     - __le16
     - s_min_extra_isize
     - 所有 inode 至少拥有 # 字节。
   - - 0x15E
     - __le16
     - s_want_extra_isize
     - 新 inode 应预留 # 字节。
   - - 0x160
     - __le32
     - s_flags
     - 杂项标志。更多信息见 super_flags_ 表。
   - - 0x164
     - __le16
     - s_raid_stride
     - RAID stride（步长）。这是在切换到下一个磁盘之前从磁盘读出或写入磁盘的逻辑块数。这会影响文件系统元数据的布局，有望提升 RAID 存储速度。
   - - 0x166
     - __le16
     - s_mmp_interval
     - 多挂载防护（MMP）检查中等待的秒数。理论上，MMP 是一种在超级块中记录已挂载该文件系统的主机和设备的机制，以防止多次挂载。该特性似乎并未实现……
   - - 0x168
     - __le64
     - s_mmp_block
     - 多挂载防护数据的块号。
   - - 0x170
     - __le32
     - s_raid_stripe_width
     - RAID 条带宽度。这是在回到当前磁盘之前从磁盘读出或写入磁盘的逻辑块数。块分配器利用它来尽量减少 RAID5/6 中的读-修改-写操作次数。
   - - 0x174
     - __u8
     - s_log_groups_per_flex
     - 灵活块组的大小为 2 ^ `s_log_groups_per_flex`。
   - - 0x175
     - __u8
     - s_checksum_type
     - 元数据校验和算法类型。唯一有效的值为 1（crc32c）。
   - - 0x176
     - \_\_u8
     - s\_encryption\_level
     - 加密的版本级别。
   - - 0x177
     - \_\_u8
     - s\_reserved\_pad
     - 填充到下一个 32 位。
   - - 0x178
     - __le64
     - s_kbytes_written
     - 该文件系统生命周期内写入的 KiB 数。
   - - 0x180
     - __le32
     - s_snapshot_inum
     - 活动快照的 inode 号。（e2fsprogs/Linux 中未使用。）
   - - 0x184
     - __le32
     - s_snapshot_id
     - 活动快照的顺序 ID。（e2fsprogs/Linux 中未使用。）
   - - 0x188
     - __le64
     - s_snapshot_r_blocks_count
     - 为活动快照未来使用而保留的块数。（e2fsprogs/Linux 中未使用。）
   - - 0x190
     - __le32
     - s_snapshot_list
     - 磁盘上快照列表头部的 inode 号。（e2fsprogs/Linux 中未使用。）
   - - 0x194
     - __le32
     - s_error_count
     - 遇到的错误数。
   - - 0x198
     - __le32
     - s_first_error_time
     - 首次发生错误的时间，自纪元起的秒数。
   - - 0x19C
     - __le32
     - s_first_error_ino
     - 首次错误涉及的 inode。
   - - 0x1A0
     - __le64
     - s_first_error_block
     - 首次错误涉及的块号。
   - - 0x1A8
     - __u8
     - s_first_error_func[^32^]
     - 发生错误的函数名。
   - - 0x1C8
     - __le32
     - s_first_error_line
     - 发生错误的行号。
   - - 0x1CC
     - __le32
     - s_last_error_time
     - 最近一次错误的时间，自纪元起的秒数。
   - - 0x1D0
     - __le32
     - s_last_error_ino
     - 最近一次错误涉及的 inode。
   - - 0x1D4
     - __le32
     - s_last_error_line
     - 最近一次错误发生的行号。
   - - 0x1D8
     - __le64
     - s_last_error_block
     - 最近一次错误涉及的块号。
   - - 0x1E0
     - __u8
     - s_last_error_func[^32^]
     - 发生最近一次错误的函数名。
   - - 0x200
     - __u8
     - s_mount_opts[^64^]
     - 挂载选项的 ASCIIZ 字符串。
   - - 0x240
     - __le32
     - s_usr_quota_inum
     - 用户 `quota <quota>`__ 文件的 inode 号。
   - - 0x244
     - __le32
     - s_grp_quota_inum
     - 组 `quota <quota>`__ 文件的 inode 号。
   - - 0x248
     - __le32
     - s_overhead_blocks
     - 文件系统中的开销块/簇。（嗯？该字段始终为零，意味着内核会动态计算它。）
   - - 0x24C
     - __le32
     - s_backup_bgs[^2^]
     - 包含超级块备份的块组（若 sparse_super2）
   - - 0x254
     - __u8
     - s_encrypt_algos[^4^]
     - 正在使用的加密算法。任意时刻最多可使用四种算法；有效的算法代码见下文 super_encrypt_ 表。
   - - 0x258
     - __u8
     - s_encrypt_pw_salt[^16^]
     - 用于加密的 string2key 算法的盐值。
   - - 0x268
     - __le32
     - s_lpf_ino
     - lost+found 的 inode 号
   - - 0x26C
     - __le32
     - s_prj_quota_inum
     - 跟踪项目配额的 inode。
   - - 0x270
     - __le32
     - s_checksum_seed
     - 用于 metadata_csum 计算的校验和种子。该值为 crc32c(~0, $orig_fs_uuid)。
   - - 0x274
     - __u8
     - s_wtime_hi
     - s_wtime 字段的高 8 位。
   - - 0x275
     - __u8
     - s_mtime_hi
     - s_mtime 字段的高 8 位。
   - - 0x276
     - __u8
     - s_mkfs_time_hi
     - s_mkfs_time 字段的高 8 位。
   - - 0x277
     - __u8
     - s_lastcheck_hi
     - s_lastcheck 字段的高 8 位。
   - - 0x278
     - __u8
     - s_first_error_time_hi
     - s_first_error_time 字段的高 8 位。
   - - 0x279
     - __u8
     - s_last_error_time_hi
     - s_last_error_time 字段的高 8 位。
   - - 0x27A
     - \_\_u8
     - s\_first\_error\_errcode
     -
   - - 0x27B
     - \_\_u8
     - s\_last\_error\_errcode
     -
   - - 0x27C
     - __le16
     - s_encoding
     - 文件名字符集编码。
   - - 0x27E
     - __le16
     - s_encoding_flags
     - 文件名字符集编码标志。
   - - 0x280
     - __le32
     - s_orphan_file_inum
     - 孤儿文件 inode 号。
   - - 0x284
     - __le32
     - s_reserved[^94^]
     - 填充至块末尾。
   - - 0x3FC
     - __le32
     - s_checksum
     - 超级块校验和。


超级块状态是以下各项的某种组合：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0x0001
     - 已干净卸载
   - - 0x0002
     - 检测到错误
   - - 0x0004
     - 正在恢复孤儿（orphan）inode


超级块错误处理策略为以下之一：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 1
     - 继续
   - - 2
     - 以只读方式重新挂载
   - - 3
     - Panic（内核慌乱）


文件系统创建者（操作系统）为以下之一：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0
     - Linux
   - - 1
     - Hurd
   - - 2
     - Masix
   - - 3
     - FreeBSD
   - - 4
     - Lites


超级块版本为以下之一：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0
     - 原始格式
   - - 1
     - 带有动态 inode 大小的 v2 格式

注意 `EXT4_DYNAMIC_REV` 指的是版本 1 或更新的文件系统。


超级块兼容特性字段是以下任意项的组合：

   :widths: 16 64
   :header-rows: 1

   - - 值
     - 说明
   - - 0x1
     - 目录预分配（COMPAT_DIR_PREALLOC）。
   - - 0x2
     - “imagic inodes”。从代码中不清楚其作用（COMPAT_IMAGIC_INODES）。
   - - 0x4
     - 拥有日志（COMPAT_HAS_JOURNAL）。
   - - 0x8
     - 支持扩展属性（COMPAT_EXT_ATTR）。
   - - 0x10
     - 拥有用于文件系统扩展的保留 GDT 块（COMPAT_RESIZE_INODE）。需要 RO_COMPAT_SPARSE_SUPER。
   - - 0x20
     - 拥有目录索引（COMPAT_DIR_INDEX）。
   - - 0x40
     - “Lazy BG”。不在 Linux 内核中，似乎曾用于未初始化的块组？（COMPAT_LAZY_BG）
   - - 0x80
     - “Exclude inode”。未使用。（COMPAT_EXCLUDE_INODE）。
   - - 0x100
     - “Exclude bitmap”。似乎用于指示存在与快照相关的排除位图？内核中未定义，e2fsprogs 中也未使用（COMPAT_EXCLUDE_BITMAP）。
   - - 0x200
     - 稀疏超级块 v2。若设置该标志，超级块的 s_backup_bgs 字段指向包含备份超级块的两个块组（COMPAT_SPARSE_SUPER2）。
   - - 0x400
     - 支持快速提交（fast commit）。尽管快速提交块是向后不兼容的，但日志中并不总是包含快速提交块。若日志中存在快速提交块，则 JBD2 不兼容特性（JBD2_FEATURE_INCOMPAT_FAST_COMMIT）会被设置（COMPAT_FAST_COMMIT）。
   - - 0x1000
     - 已分配孤儿文件。这是用于更高效地跟踪已删除但仍打开的 inode 的特殊文件。当该文件可能存在任何条目时，我们还会设置相应的 rocompat 特性（RO_COMPAT_ORPHAN_PRESENT）。


超级块不兼容特性字段是以下任意项的组合：

   :widths: 16 64
   :header-rows: 1

   - - 值
     - 说明
   - - 0x1
     - 压缩（INCOMPAT_COMPRESSION）。
   - - 0x2
     - 目录项记录文件类型。见下文的 ext4_dir_entry_2（INCOMPAT_FILETYPE）。
   - - 0x4
     - 文件系统需要恢复（INCOMPAT_RECOVER）。
   - - 0x8
     - 文件系统拥有独立的日志设备（INCOMPAT_JOURNAL_DEV）。
   - - 0x10
     - 元块组。见前文对该特性的讨论（INCOMPAT_META_BG）。
   - - 0x40
     - 该文件系统中的文件使用 extents（INCOMPAT_EXTENTS）。
   - - 0x80
     - 启用 2^64 个块的文件系统大小（INCOMPAT_64BIT）。
   - - 0x100
     - 多挂载保护（INCOMPAT_MMP）。
   - - 0x200
     - 灵活块组。见前文对该特性的讨论（INCOMPAT_FLEX_BG）。
   - - 0x400
     - inode 可用于存储较大的扩展属性值（INCOMPAT_EA_INODE）。
   - - 0x1000
     - 目录项中的数据（INCOMPAT_DIRDATA）。（未实现？）
   - - 0x2000
     - 元数据校验和种子存储在超级块中。该特性允许管理员在文件系统挂载时更改 metadata_csum 文件系统的 UUID；没有它，校验和定义要求重写所有元数据块（INCOMPAT_CSUM_SEED）。
   - - 0x4000
     - 大目录 >2GB 或 3 级 htree（INCOMPAT_LARGEDIR）。在此特性之前，目录不能大于 4GiB，且 htree 深度不能超过 2 层。若启用该特性，目录可以大于 4GiB，且 htree 最大深度为 3。
   - - 0x8000
     - inode 中的数据（INCOMPAT_INLINE_DATA）。
   - - 0x10000
     - 可能存在加密 inode。（INCOMPAT_ENCRYPT）。
   - - 0x20000
     - 目录可被标记为不区分大小写。（INCOMPAT_CASEFOLD）。


超级块只读兼容特性字段是以下任意项的组合：

   :widths: 16 64
   :header-rows: 1

   - - 值
     - 说明
   - - 0x1
     - 稀疏超级块。见前文对该特性的讨论（RO_COMPAT_SPARSE_SUPER）。
   - - 0x2
     - 该文件系统曾用于存储大于 2GiB 的文件（RO_COMPAT_LARGE_FILE）。
   - - 0x4
     - 内核或 e2fsprogs 中未使用（RO_COMPAT_BTREE_DIR）。
   - - 0x8
     - 该文件系统中的文件大小以逻辑块为单位表示，而非 512 字节扇区。这意味着文件确实非常大！（RO_COMPAT_HUGE_FILE）
   - - 0x10
     - 块组描述符带有校验和。除了检测损坏外，这对带有未初始化组的惰性格式化也很有用（RO_COMPAT_GDT_CSUM）。
   - - 0x20
     - 表示旧的 ext3 三万二千子目录限制不再适用（RO_COMPAT_DIR_NLINK）。若目录的 i_links_count 超过 64,999，将被设为 1。
   - - 0x40
     - 表示该文件系统上存在较大的 inode（RO_COMPAT_EXTRA_ISIZE）。
   - - 0x80
     - 该文件系统拥有快照（RO_COMPAT_HAS_SNAPSHOT）。
   - - 0x100
     - `Quota <Quota>`__（RO_COMPAT_QUOTA）。
   - - 0x200
     - 该文件系统支持 “bigalloc”，即文件 extents 以簇（块的集合）为单位而非以块为单位进行跟踪（RO_COMPAT_BIGALLOC）。
   - - 0x400
     - 该文件系统支持元数据校验和。（RO_COMPAT_METADATA_CSUM；隐含 RO_COMPAT_GDT_CSUM，但 GDT_CSUM 不得设置）
   - - 0x800
     - 文件系统支持副本。该特性既不在内核中，也不在 e2fsprogs 中。（RO_COMPAT_REPLICA）
   - - 0x1000
     - 只读文件系统镜像；内核不会以读写方式挂载该镜像，且大多数工具会拒绝写入镜像。（RO_COMPAT_READONLY）
   - - 0x2000
     - 文件系统跟踪项目配额。（RO_COMPAT_PROJECT）
   - - 0x8000
     - 文件系统上可能存在 Verity inode。（RO_COMPAT_VERITY）
   - - 0x10000
     - 表示孤儿文件可能含有有效的孤儿条目，因此挂载文件系统时需要清理它们（RO_COMPAT_ORPHAN_PRESENT）。


`s_def_hash_version` 字段为以下之一：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0x0
     - 传统方式。
   - - 0x1
     - 半 MD4。
   - - 0x2
     - Tea。
   - - 0x3
     - 传统方式，无符号。
   - - 0x4
     - 半 MD4，无符号。
   - - 0x5
     - Tea，无符号。


`s_default_mount_opts` 字段是以下各项的任意组合：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0x0001
     - 在（重新）挂载时打印调试信息。（EXT4_DEFM_DEBUG）
   - - 0x0002
     - 新文件采用其所在目录的 gid（而非当前进程的 fsgid）。（EXT4_DEFM_BSDGROUPS）
   - - 0x0004
     - 支持用户空间提供的扩展属性。（EXT4_DEFM_XATTR_USER）
   - - 0x0008
     - 支持 POSIX 访问控制列表（ACL）。（EXT4_DEFM_ACL）
   - - 0x0010
     - 不支持 32 位 UID。（EXT4_DEFM_UID16）
   - - 0x0020
     - 所有数据和元数据都提交到日志。（EXT4_DEFM_JMODE_DATA）
   - - 0x0040
     - 在元数据提交到日志之前，所有数据都已刷新到磁盘。（EXT4_DEFM_JMODE_ORDERED）
   - - 0x0060
     - 不保留数据顺序；数据可能在元数据写入之后才写入。（EXT4_DEFM_JMODE_WBACK）
   - - 0x0100
     - 禁用写刷新。（EXT4_DEFM_NOBARRIER）
   - - 0x0200
     - 跟踪文件系统中哪些块是元数据，因此不应被用作数据块。该选项有望在 3.18 中默认启用。（EXT4_DEFM_BLOCK_VALIDITY）
   - - 0x0400
     - 启用 DISCARD 支持，即通知存储设备哪些块已变为未使用。（EXT4_DEFM_DISCARD）
   - - 0x0800
     - 禁用延迟分配。（EXT4_DEFM_NODELALLOC）


`s_flags` 字段是以下各项的任意组合：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0x0001
     - 使用带符号的目录哈希。
   - - 0x0002
     - 使用无符号的目录哈希。
   - - 0x0004
     - 用于测试开发代码。


`s_encrypt_algos` 列表可包含以下任意项：

   :widths: 8 72
   :header-rows: 1

   - - 值
     - 说明
   - - 0
     - 无效算法（ENCRYPTION_MODE_INVALID）。
   - - 1
     - XTS 模式下的 256 位 AES（ENCRYPTION_MODE_AES_256_XTS）。
   - - 2
     - GCM 模式下的 256 位 AES（ENCRYPTION_MODE_AES_256_GCM）。
   - - 3
     - CBC 模式下的 256 位 AES（ENCRYPTION_MODE_AES_256_CBC）。

超级块的总大小为 1024 字节。
