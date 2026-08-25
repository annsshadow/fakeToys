
## 高层设计


ext4 文件系统被拆分为一系列块组。为了减少因碎片而导致的性能问题，块分配会尽力将每个文件的块保留在同一组中，从而减少寻道时间。块组的大小`sb.s_blocks_per_group` 个块中指定，但也可以计算8 *
`block_size_in_bytes`。使用默认的 4KiB 块大小，每个组将包含 32,768 个块长度128MiB。块组的数量等于设备大小除以块组大小
ext4 中的所有字段都以小端序写入磁盘。但是，jbd2（日志）中的所有字段都大端序写入磁盘
- [blocks](blocks)
- [blockgroup](blockgroup)
- [special_inodes](special_inodes)
- [allocators](allocators)
- [checksums](checksums)
- [bigalloc](bigalloc)
- [inlinedata](inlinedata)
- [eainode](eainode)
- [verity](verity)
- [atomic_writes](atomic_writes)
