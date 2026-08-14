
### Block Group Descriptors


文件系统中的每个块组（block group）都关联有一个这样的描述符。正如上文"布局"一节所述，组描述符（如果存在）是块组中的第二项。标准配置下，每个块组都包含一份完整的块组描述符表副本，除非设置了 sparse_super 特性标志。

注意组描述符记录了两个位图以及 inode 表的位置（即它们可以浮动）。这意味着在块组内，具有固定位置的唯一数据结构是超级块（superblock）和组描述符表。flex_bg 机制利用这一特性将若干个块组归入一个 flex group，并把所有这些组的位图和 inode 表布局到 flex group 第一个组中的一段连续区域里。

如果设置了 meta_bg 特性标志，则若干个块组会被归入一个 meta group。需要注意的是，在 meta_bg 的情况下，较大的 meta group 中第一个和最后两个块组只包含该 meta group 内部各组的组描述符。

flex_bg 和 meta_bg 看起来并非互斥的特性。

在 ext2、ext3 以及 ext4（未启用 64bit 特性时）中，块组描述符只有 32 字节长，因此到 bg_checksum 处结束。在启用了 64bit 特性的 ext4 文件系统上，块组描述符至少扩展到下面描述的 64 字节；其大小存储在超级块中。

如果设置了 gdt_csum 而未设置 metadata_csum，则块组校验和是 FS UUID、组号以及组描述符结构的 crc16。如果设置了 metadata_csum，则块组校验和是 FS UUID、组号以及组描述符结构校验和的第 16 位。块位图和 inode 位图的校验和都是针对 FS UUID、组号以及整个位图计算的。

块组描述符以 `struct ext4_group_desc` 布局。

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移（Offset）
     - 大小（Size）
     - 名称（Name）
     - 描述（Description）
   - - 0x0
     - __le32
     - bg_block_bitmap_lo
     - 块位图位置的低 32 位。
   - - 0x4
     - __le32
     - bg_inode_bitmap_lo
     - inode 位图位置的低 32 位。
   - - 0x8
     - __le32
     - bg_inode_table_lo
     - inode 表位置的低 32 位。
   - - 0xC
     - __le16
     - bg_free_blocks_count_lo
     - 空闲块计数的低 16 位。
   - - 0xE
     - __le16
     - bg_free_inodes_count_lo
     - 空闲 inode 计数的低 16 位。
   - - 0x10
     - __le16
     - bg_used_dirs_count_lo
     - 目录计数的低 16 位。
   - - 0x12
     - __le16
     - bg_flags
     - 块组标志。参见下文的 bgflags_ 表。
   - - 0x14
     - __le32
     - bg_exclude_bitmap_lo
     - 快照排除位图位置的低 32 位。
   - - 0x18
     - __le16
     - bg_block_bitmap_csum_lo
     - 块位图校验和的低 16 位。
   - - 0x1A
     - __le16
     - bg_inode_bitmap_csum_lo
     - inode 位图校验和的低 16 位。
   - - 0x1C
     - __le16
     - bg_itable_unused_lo
     - 未使用 inode 计数的低 16 位。若设置，则无需扫描该组 inode 表中
       `(sb.s_inodes_per_group - gdt.bg_itable_unused)` 之后的条目。
   - - 0x1E
     - __le16
     - bg_checksum
     - 组描述符校验和；若设置了 RO_COMPAT_GDT_CSUM 特性，则为
       crc16(sb_uuid+group_num+bg_desc)，或者若设置了
       RO_COMPAT_METADATA_CSUM 特性，则为 crc32c(sb_uuid+group_num+bg_desc) & 0xFFFF。
       计算 crc16 校验和时会跳过 bg_desc 中的 bg_checksum 字段，
       若使用 crc32c 校验和则将其置为零。
#    * -

     -
     - 以下字段仅在启用 64bit 特性且 s_desc_size > 32 时存在。
   - - 0x20
     - __le32
     - bg_block_bitmap_hi
     - 块位图位置的高 32 位。
   - - 0x24
     - __le32
     - bg_inode_bitmap_hi
     - inode 位图位置的高 32 位。
   - - 0x28
     - __le32
     - bg_inode_table_hi
     - inode 表位置的高 32 位。
   - - 0x2C
     - __le16
     - bg_free_blocks_count_hi
     - 空闲块计数的高 16 位。
   - - 0x2E
     - __le16
     - bg_free_inodes_count_hi
     - 空闲 inode 计数的高 16 位。
   - - 0x30
     - __le16
     - bg_used_dirs_count_hi
     - 目录计数的高 16 位。
   - - 0x32
     - __le16
     - bg_itable_unused_hi
     - 未使用 inode 计数的高 16 位。
   - - 0x34
     - __le32
     - bg_exclude_bitmap_hi
     - 快照排除位图位置的高 32 位。
   - - 0x38
     - __le16
     - bg_block_bitmap_csum_hi
     - 块位图校验和的高 16 位。
   - - 0x3A
     - __le16
     - bg_inode_bitmap_csum_hi
     - inode 位图校验和的高 16 位。
   - - 0x3C
     - __u32
     - bg_reserved
     - 填充至 64 字节。


块组标志可以是以下任意组合：

   :widths: 16 64
   :header-rows: 1

   - - 值（Value）
     - 描述（Description）
   - - 0x1
     - inode 表和位图未初始化（EXT4_BG_INODE_UNINIT）。
   - - 0x2
     - 块位图未初始化（EXT4_BG_BLOCK_UNINIT）。
   - - 0x4
     - inode 表已清零（EXT4_BG_INODE_ZEROED）。
