
### 索引节点（Index Nodes
在常规的 UNIX 文件系统中，inode 存储与文件相关的所有元数据（时间戳、块映射、扩展属性等），
而不存储目录项。要查找与文件关联的信息，必须遍历目录文件以找到与该文件关联的目录项然后加载 inode 以找到该文件的元数据。出于性能原因，ext4 似乎（略微）耍了点小聪明：它在目录项存储一份文件类型（通常存于 inode 中）的副本。（将这一切与 FAT 对比：FAT 把所有文件信息直接存目录项中，但不支持硬链接，并且由于其更简单的块分配器以及大量使用链表，通常ext4 更频繁地寻道。）

inode 表是一`struct ext4_inode` 的线性数组。该表的大小被设为足以存储至`sb.s_inode_size * sb.s_inodes_per_group` 字节。包含某inode 的块组编号可计算`(inode_number - 1) / sb.s_inodes_per_group`，而该组内表的偏移`(inode_number - 1) % sb.s_inodes_per_group`。不存在 inode 0
inode 校验和是针对 FS UUID、inode 编号以及 inode 结构本身计算的
inode 表项布局`struct ext4_inode` 中
   :widths: 8 8 24 40
   :header-rows: 1
   :class: longtable

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - i_mode
     - 文件模式。参见下文的 i_mode_ 表   - - 0x2
     - __le16
     - i_uid
     - 所有UID 的低 16 位   - - 0x4
     - __le32
     - i_size_lo
     - 大小（字节）的低 32 位   - - 0x8
     - __le32
     - i_atime
     - 上次访问时间，自纪元起的秒数。但是，若设置了 EA_INODE inode 标志，此 inode 存储一       扩展属性值，该字段包含此值的校验和   - - 0xC
     - __le32
     - i_ctime
     - 上次 inode 更改时间，自纪元起的秒数。但是，若设置了 EA_INODE inode 标志，此 inode 存储
       一个扩展属性值，该字段包含属性值引用计数的32 位   - - 0x10
     - __le32
     - i_mtime
     - 上次数据修改时间，自纪元起的秒数。但是，若设置了 EA_INODE inode 标志，此 inode 存储一       扩展属性值，该字段包含拥有该扩展属性的 inode 的编号   - - 0x14
     - __le32
     - i_dtime
     - 删除时间，自纪元起的秒数   - - 0x18
     - __le16
     - i_gid
     - GID 的低 16 位   - - 0x1A
     - __le16
     - i_links_count
     - 硬链接计数。通常，ext4 不允许一inode 拥有超过 65,000 个硬链接。这适用于文件和目录       意味着一个目录中不能有超64,998 个子目录（每个子目录'..' 项计为一个硬链接，目录自身的
       '.' 项也是如此）。启DIR_NLINK 特性后，ext4 通过将此字段设为 1 来表示硬链接数未知，
       从而支持超64,998 个子目录   - - 0x1C
     - __le32
     - i_blocks_lo
     - “块”计数的32 位。如果文件系统未设置 huge_file 特性标志，文件在磁盘上消       `i_blocks_lo` 512 字节块。如果设置了 huge_file `inode.i_flags` 中未设置
       EXT4_HUGE_FILE_FL，则文件在磁盘上消``i_blocks_lo + (i_blocks_hi << 32)``        512 字节块。如果设置了 huge_file `inode.i_flags` 中设置了 EXT4_HUGE_FILE_FL，则文件
       在磁盘上消(`i_blocks_lo + i_blocks_hi` << 32) 个文件系统块   - - 0x20
     - __le32
     - i_flags
     - Inode 标志。参见下文的 i_flags_ 表   - - 0x24
     - 4 bytes
     - i_osd1
     - 更多细节参见 i_osd1_ 表   - - 0x28
     - 60 bytes
     - i_block[EXT4_N_BLOCKS=15]
     - 块映射或范围树。参见“inode.i_block 的内容”小节   - - 0x64
     - __le32
     - i_generation
     - 文件版本（用NFS）   - - 0x68
     - __le32
     - i_file_acl_lo
     - 扩展属性块的低 32 位。ACL 当然是众多可能扩展属性之一；此字段的名称源于扩展属性最       用于 ACL   - - 0x6C
     - __le32
     - i_size_high / i_dir_acl
     - 文件/目录大小的高 32 位。在 ext2/3 中此字段名为 i_dir_acl，尽管通常设为零且从未使用   - - 0x70
     - __le32
     - i_obso_faddr
     - （已废弃）片段地址   - - 0x74
     - 12 bytes
     - i_osd2
     - 更多细节参见 i_osd2_ 表   - - 0x80
     - __le16
     - i_extra_isize
     - inode 大小减去 128。或者，原始 ext2 inode 之外的扩inode 字段大小（包括此字段）   - - 0x82
     - __le16
     - i_checksum_hi
     - inode 校验和的16 位   - - 0x84
     - __le32
     - i_ctime_extra
     - 额外的更改时间位。提供亚秒级精度。参Inode 时间戳小节   - - 0x88
     - __le32
     - i_mtime_extra
     - 额外的修改时间位。提供亚秒级精度   - - 0x8C
     - __le32
     - i_atime_extra
     - 额外的访问时间位。提供亚秒级精度   - - 0x90
     - __le32
     - i_crtime
     - 文件创建时间，自纪元起的秒数   - - 0x94
     - __le32
     - i_crtime_extra
     - 额外的文件创建时间位。提供亚秒级精度   - - 0x98
     - __le32
     - i_version_hi
     - 版本号的32 位   - - 0x9C
     - __le32
     - i_projid
     - 项目 ID

`i_mode` 值是以下标志的组合：

   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x1
     - S_IXOTH（其它用户可执行   - - 0x2
     - S_IWOTH（其它用户可写）
   - - 0x4
     - S_IROTH（其它用户可读）
   - - 0x8
     - S_IXGRP（组成员可执行）
   - - 0x10
     - S_IWGRP（组成员可写   - - 0x20
     - S_IRGRP（组成员可读   - - 0x40
     - S_IXUSR（所有者可执行   - - 0x80
     - S_IWUSR（所有者可写）
   - - 0x100
     - S_IRUSR（所有者可读）
   - - 0x200
     - S_ISVTX（粘滞位   - - 0x400
     - S_ISGID（设GID   - - 0x800
     - S_ISUID（设UID   - -
     - 以下是互斥的文件类型   - - 0x1000
     - S_IFIFO（FIFO   - - 0x2000
     - S_IFCHR（字符设备）
   - - 0x4000
     - S_IFDIR（目录）
   - - 0x6000
     - S_IFBLK（块设备   - - 0x8000
     - S_IFREG（普通文件）
   - - 0xA000
     - S_IFLNK（符号链接）
   - - 0xC000
     - S_IFSOCK（套接字

`i_flags` 字段是以下值的组合
   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x1
     - 此文件需要安全删除（EXT4_SECRM_FL）。（未实现）
   - - 0x2
     - 若希望反删除，应保留此文件（EXT4_UNRM_FL）。（未实现）
   - - 0x4
     - 文件已压缩（EXT4_COMPR_FL）。（并未真正实作   - - 0x8
     - 对文件的所有写入必须是同步的（EXT4_SYNC_FL）   - - 0x10
     - 文件不可变（EXT4_IMMUTABLE_FL）   - - 0x20
     - 文件只能追加（EXT4_APPEND_FL）   - - 0x40
     - dump(1) 工具不应转储此文件（EXT4_NODUMP_FL）   - - 0x80
     - 不更新访问时间（EXT4_NOATIME_FL）   - - 0x100
     - 脏的已压缩文件（EXT4_DIRTY_FL）。（未使用）
   - - 0x200
     - 文件具有一个或多个已压缩簇（EXT4_COMPRBLK_FL）。（未使用）
   - - 0x400
     - 不压缩文件（EXT4_NOCOMPR_FL）。（未使用）
   - - 0x800
     - 已加密的 inode（EXT4_ENCRYPT_FL）。此位值此前为 EXT4_ECOMPR_FL（压缩错误），从未使用   - - 0x1000
     - 目录具有哈希索引（EXT4_INDEX_FL）   - - 0x2000
     - AFS 魔法目录（EXT4_IMAGIC_FL）   - - 0x4000
     - 文件数据必须始终通过日志写入（EXT4_JOURNAL_DATA_FL）   - - 0x8000
     - 文件尾部不应合并（EXT4_NOTAIL_FL）。（ext4 未使用）
   - - 0x10000
     - 所有目录项数据应同步写入（参见 `dirsync`）（EXT4_DIRSYNC_FL）   - - 0x20000
     - 目录层级的顶端（EXT4_TOPDIR_FL）   - - 0x40000
     - 这是一个大文件（EXT4_HUGE_FILE_FL）   - - 0x80000
     - Inode 使用范围（extents）（EXT4_EXTENTS_FL）   - - 0x100000
     - 经过 verity 保护文件（EXT4_VERITY_FL）   - - 0x200000
     - Inode 在其数据块中存储一个大型扩展属性值（EXT4_EA_INODE_FL）   - - 0x400000
     - 此文件分配了越过 EOF 的块（EXT4_EOFBLOCKS_FL）。（已弃用）
   - - 0x01000000
     - Inode 是快照（`EXT4_SNAPFILE_FL`）。（不在主线中）
   - - 0x04000000
     - 快照正在被删除（`EXT4_SNAPFILE_DELETED_FL`）。（不在主线中）
   - - 0x08000000
     - 快照收缩已完成（`EXT4_SNAPFILE_SHRUNK_FL`）。（不在主线中）
   - - 0x10000000
     - Inode 具有内联数据（EXT4_INLINE_DATA_FL）   - - 0x20000000
     - 以相同的项目 ID 创建子项（EXT4_PROJINHERIT_FL）   - - 0x40000000
     - 对目录内容使用大小写不敏感的查找（EXT4_CASEFOLD_FL）   - - 0x80000000
     - 保留ext4 库（EXT4_RESERVED_FL）   - -
     - 聚合标志   - - 0x705BDFFF
     - 用户可见标志   - - 0x604BC0FF
     - 用户可修改标志。注意，虽然 EXT4_JOURNAL_DATA_FL EXT4_EXTENTS_FL 可以通过 setattr 设置       但它们不在内核的 EXT4_FL_USER_MODIFIABLE 掩码中，因为内核需要以特殊方式处理这些标志的设置，
       并且它们被从直接保存i_flags 的标志集合中屏蔽掉

`osd1` 字段根据创建者的不同有多个含义：

Linux锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - l_i_version
     - Inode 版本。但是，若设置了 EA_INODE inode 标志，此 inode 存储一个扩展属性值，该字段包       属性值引用计数的32 位
Hurd锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - h_i_translator
     - ??

Masix锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - m_i_reserved
     - ??


`osd2` 字段根据文件系统创建者的不同有多个含义：

Linux锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - l_i_blocks_high
     - 块计数的16 位。请参阅附于 i_blocks_lo 的说明   - - 0x2
     - __le16
     - l_i_file_acl_high
     - 扩展属性块的高 16 位（历史上指文件 ACL 位置）。参见下文的扩展属性小节   - - 0x4
     - __le16
     - l_i_uid_high
     - 所有UID 的高 16 位   - - 0x6
     - __le16
     - l_i_gid_high
     - GID 的高 16 位   - - 0x8
     - __le16
     - l_i_checksum_lo
     - inode 校验和的16 位   - - 0xA
     - __le16
     - l_i_reserved
     - 未使用
Hurd锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - h_i_reserved1
     - ??
   - - 0x2
     - __u16
     - h_i_mode_high
     - 文件模式的高 16 位   - - 0x4
     - __le16
     - h_i_uid_high
     - 所有UID 的高 16 位   - - 0x6
     - __le16
     - h_i_gid_high
     - GID 的高 16 位   - - 0x8
     - __u32
     - h_i_author
     - 作者代码？

Masix锛。
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le16
     - h_i_reserved1
     - ??
   - - 0x2
     - __u16
     - m_i_file_acl_high
     - 扩展属性块的高 16 位（历史上指文件 ACL 位置）   - - 0x4
     - __u32
     - m_i_reserved2[^2^]
     - ??

#### Inode 大小

ext2 ext3 中，inode 结构大小固定128 字节（`EXT2_GOOD_OLD_INODE_SIZE`），
每个 inode 的磁盘记录大小为 128 字节。从 ext4 开始，可以在格式化时为文件系统中所inode
分配一个更大的磁盘 inode，以提供超出原始 ext2 inode 末尾之外的空间。磁inode 记录大小记录超级块中`s_inode_size`。除原始 128 字节 ext2 inode 之外，`struct ext4_inode` 实际使用字节数记录于每个 inode `i_extra_isize` 字段中，这使`struct ext4_inode` 能够为新内核增长而无需升级所有磁盘上inode。对超出 EXT2_GOOD_OLD_INODE_SIZE 的字段的访问，应验证其位`i_extra_isize` 之内。默认情况下，ext4 inode 记录256 字节，且（截2019 8 月）inode 结构160 字节（`i_extra_isize = 32`）。inode 结构末尾inode 记录末尾之间的额外空间可用于存储
扩展属性。每inode 记录最大可达文件系统块大小，尽管这并非特别高效
#### 查找一Inode

每个块组包含 `sb->s_inodes_per_group` inode。由inode 0 被定义为不存在，可使用以下公找到某个 inode 所在的块组：`bg = (inode_num - 1) / sb->s_inodes_per_group`特定inode 可在块组inode 表中`index = (inode_num - 1) % sb->s_inodes_per_group`
处找到。要获取 inode 表中的字节地址，使`offset = index * sb->s_inode_size`
#### Inode 时间
四个时间戳记录于 inode 结构的低 128 字节中——inode 更改时间（ctime）、访问时间（atime）数据修改时间（mtime）以及删除时间（dtime）。这四个字段32 位有符号整数，表示自 Unix 纪元
970-01-01 00:00:00 GMT）起的秒数，这意味着这些字段将在 2038 1 月溢出。如果文件系统没orphan_file 特性，那些未从任何目录链接但仍处于打开状态的 inode（孤inode）会dtime 字段
重载用于孤儿列表。超级块字段 `s_last_orphan` 指向孤儿列表中的第一inode；dtime 随后是下一孤儿 inode 的编号，若没有更多孤儿则0
如果 inode 结构大小 `sb->s_inode_size` 大于 128 字节，且 `i_inode_extra` 字段足够大以容纳
相应`i_[cma]time_extra` 字段，则 ctime、atime mtime inode 字段被拓宽为 64 位。在这个
“额外”的 32 位字段中，低 2 位用于将 32 位秒字段扩展34 位宽；高 30 位用于提供纳秒级时间戳精度因此，时间戳2446 5 月之前不应溢出。dtime 未被拓宽。还有一个第五个时间戳用于记inode 创建时间
（crtime）；此字段为 64 位宽，并以与 64 [cma]time 相同的方式解码。crtime dtime 都无法通过
常规stat() 接口访问，不debugfs 会报告它们
我们使用 32 位有符号时间值加上（2^32 *（额外纪元位））。换言之：

   :widths: 20 20 20 20 20
   :header-rows: 1

   - - Extra epoch bits
     - MSB of 32-bit time
     - Adjustment for signed 32-bit to 64-bit tv_sec
     - Decoded 64-bit tv_sec
     - valid time range
   - - 0 0
     - 1
     - 0
     - `-0x80000000 - -0x00000001`
     - 1901-12-13 鑷?1969-12-31
   - - 0 0
     - 0
     - 0
     - `0x000000000 - 0x07fffffff`
     - 1970-01-01 鑷?2038-01-19
   - - 0 1
     - 1
     - 0x100000000
     - `0x080000000 - 0x0ffffffff`
     - 2038-01-19 鑷?2106-02-07
   - - 0 1
     - 0
     - 0x100000000
     - `0x100000000 - 0x17fffffff`
     - 2106-02-07 鑷?2174-02-25
   - - 1 0
     - 1
     - 0x200000000
     - `0x180000000 - 0x1ffffffff`
     - 2174-02-25 鑷?2242-03-16
   - - 1 0
     - 0
     - 0x200000000
     - `0x200000000 - 0x27fffffff`
     - 2242-03-16 鑷?2310-04-04
   - - 1 1
     - 1
     - 0x300000000
     - `0x280000000 - 0x2ffffffff`
     - 2310-04-04 鑷?2378-04-22
   - - 1 1
     - 0
     - 0x300000000
     - `0x300000000 - 0x37fffffff`
     - 2378-04-22 鑷?2446-05-10

这是一种有些奇怪的编码，因为正值的数量实际上是负值数量的七倍。对2038 年之后的日期，也长期存在
解码和编码的 bug，截至内3.12 e2fsprogs 1.42.8 似乎仍未修复4 位内核错误地使用额外纪元1,1 来处1901 1970 年之间的日期。内核终将被修复，e2fsck 也会修复此状况，前提是它2310 之前运行