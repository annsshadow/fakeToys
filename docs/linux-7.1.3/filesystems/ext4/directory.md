
### 目录

ext4 文件系统中，目录或多或少是一个平面文件，它将一个任意的字节串（通常ASCII）映射到文件系统中的某个 inode 号。整个文件系统中可以有许多目录项引用同一inode 号——这被称为硬链接，这也是为什么硬链接不能引用其他文件系统上的文件。因此，目录项是通过读取与所需特定目录项相关联的目录文件的数据块来查找的
#### 线性（经典）目

默认情况下，每个目录将其项列在一几乎线的数组中。我几乎"是因为从内存意义上说它并非线性数组，因为目录项不会被跨文件系统块分割。因此，更准确的说法是：目录是一系列数据块，每个块包含一组线性的目录项数组。每个块内数组的结束以到达块的末尾来表示；块中的最后一个条目的记录长度会一直延伸到块的末尾。整个目录的结束当然以到达文件末尾来表示。未使用的目录项inode = 0 来表示。默认情况下，文件系统使`struct ext4_dir_entry_2` 作为目录项，除非未设filetype"特性标志，此时它使`struct ext4_dir_entry`
原始的目录项格式`struct ext4_dir_entry`，最263 字节，不过在磁盘上你需要参`dirent.rec_len` 才能确定
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - inode
     - 本目录项所指向inode 的编号   - - 0x4
     - __le16
     - rec_len
     - 本目录项的长度。必须是 4 的倍数   - - 0x6
     - __le16
     - name_len
     - 文件名的长度   - - 0x8
     - char
     - name[EXT4_NAME_LEN]
     - 文件名
由于文件名不能超255 字节，新的目录项格式缩短name_len 字段，并将腾出的空间用于文件类型标志，大概是为了避免在遍历目录树时不得不加载每个 inode。此格式`ext4_dir_entry_2`，最263 字节，不过在磁盘上你需要参`dirent.rec_len` 才能确定
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - inode
     - 本目录项所指向inode 的编号   - - 0x4
     - __le16
     - rec_len
     - 本目录项的长度   - - 0x6
     - __u8
     - name_len
     - 文件名的长度   - - 0x7
     - __u8
     - file_type
     - 文件类型代码，见下文ftype_ 表   - - 0x8
     - char
     - name[EXT4_NAME_LEN]
     - 文件名

目录文件类型是下列值之一
   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x0
     - 未知   - - 0x1
     - 普通文件   - - 0x2
     - 目录   - - 0x3
     - 字符设备文件   - - 0x4
     - 块设备文件   - - 0x5
     - FIFO銆?   - - 0x6
     - 套接字   - - 0x7
     - 符号链接
为了支持既加密又大小写折叠（casefolded）的目录，我们还必须在目录项中包含哈希信息。除dot dotdot 项（它们保持不变）之外，我们`ext4_extended_dir_entry_2` 附加`ext4_dir_entry_2` 之后。该结构紧跟`name` 之后，并包含`rec_len` 列出的大小中。如果一个目录项使用了此扩展，它最长可271 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - hash
     - 目录名的哈希   - - 0x4
     - __le32
     - minor_hash
     - 目录名的次哈希

为了给这些经典目录块添加校验和，在每个叶子块的末尾放置一个伪 `struct ext4_dir_entry` 来保存校验和。该目录项长 12 字节。inode 号和 name_len 字段被设置为零，以欺骗旧软件忽略一个看似空的目录项，校验和则存储在通常放置文件名的位置。该结构`struct ext4_dir_entry_tail`
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Size
     - Name
     - Description
   - - 0x0
     - __le32
     - det_reserved_zero1
     - inode 号，必须为零   - - 0x4
     - __le16
     - det_rec_len
     - 本目录项的长度，必须12   - - 0x6
     - __u8
     - det_reserved_zero2
     - 文件名的长度，必须为零   - - 0x7
     - __u8
     - det_reserved_ft
     - 文件类型，必须为 0xDE   - - 0x8
     - __le32
     - det_checksum
     - 目录叶子块校验和
叶子目录块校验和是根据文件系UUID（或者，如果为该文件系统启用了该特性，则为校验和种子）、目录的 inode 号、目录的 inode 生成号，以及整个目录项块（直到但不包括伪目录项）计算得出的
#### 哈希树目

目录项的线性数组对性能并不理想，因ext3 中加入了一个新特性，提供一个更快（但奇特）的平衡树，以目录项名称的哈希值为键。如inode 中设置了 EXT4_INDEX_FL (0x1000) 标志，则该目录使用哈btree（htree）来组织和查找目录项。为了与 ext2 向后只读兼容，内部树节点实际上被隐藏在目录文件内部，伪装成跨越整个块目录项。前面说过，inode 设为 0 的目录项被视为未使用项；这里（ab）利用这一点来欺骗旧的线性扫描算法跳过那些包含内部树节点数据的块
树的根始终位于目录的第一个数据块中。按ext2 的惯例，'.' '..' 项必须出现在该第一块的开头，因此它们作为两个 `struct ext4_dir_entry_2` 放在这里，而不存储在树中。根节点的其余部分包含关于该树的元数据，最后是一hash->block 映射，用于查htree 中更下层的节点。如`dx_root.info.indirect_levels` 非零，则 htree 具有那么多层，根节点映射所指向的块是内部节点。这些内部节点有一个清零的 `struct ext4_dir_entry_2`，其后跟着一hash->block 映射，用于查找下一层的节点。叶子节点看起来像经典的线性目录块，但其所有条目的哈希值都大于或等于父节点指示的哈希值
条目名称的实际哈希值只31 位，最低有效位设为 0。然而，如果目录项之间发生哈希冲突，在这两个（或更多）哈希冲突的条目无法放入一个叶子节点而必须跨多个节点拆分的情况下，最低有效位可能在内部节点上被设1
要在这样htree 中查找一个名称，代码计算所需文件名的哈希值，并用它来找到哈希值范围包含所计算哈希值的叶子节点（换句话说，查找的工作方式与以哈希值为键的 B 树基本相同），并且由于哈希冲突的可能，还要扫描其后的（按树顺序的）叶子节点
要将目录作为线性数组遍历（就像旧代码所做的那样），代码只需读取目录中的每个数据块。用htree 的块看起来没有任何条目（除了 '.' '..'），因此只有叶子节点看起来包含任何有趣的内容
htree 的根位于 `struct dx_root` 中，其长度为一个完整的数据块：

   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - dot.inode
     - 本目录的 inode 号   - - 0x4
     - __le16
     - dot.rec_len
     - 本记录的长度2   - - 0x6
     - u8
     - dot.name_len
     - 名称的长度，1   - - 0x7
     - u8
     - dot.file_type
     - 本条目的文件类型x2（目录）（如果设置了特性标志）   - - 0x8
     - char
     - dot.name[^4^]
     - 鈥?\0\0\0鈥?   - - 0xC
     - __le32
     - dotdot.inode
     - 鐖剁洰褰曠殑 inode 鍙枫€?   - - 0x10
     - __le16
     - dotdot.rec_len
     - block_size - 12。记录长度足够长以覆盖所htree 数据   - - 0x12
     - u8
     - dotdot.name_len
     - 名称的长度，2   - - 0x13
     - u8
     - dotdot.file_type
     - 本条目的文件类型x2（目录）（如果设置了特性标志）   - - 0x14
     - char
     - dotdot_name[^4^]
     - 鈥?.\0\0鈥?   - - 0x18
     - __le32
     - struct dx_root_info.reserved_zero
     - 零   - - 0x1C
     - u8
     - struct dx_root_info.hash_version
     - 哈希类型，见下文dirhash_ 表   - - 0x1D
     - u8
     - struct dx_root_info.info_length
     - 树信息的长度x8   - - 0x1E
     - u8
     - struct dx_root_info.indirect_levels
     - htree 的深度。如果设置了 INCOMPAT_LARGEDIR 特性，则不能超3；否则不能超2   - - 0x1F
     - u8
     - struct dx_root_info.unused_flags
     - -
   - - 0x20
     - __le16
     - limit
     - 可跟随此头部dx_entry 的最大数量，再加上头部本身的 1   - - 0x22
     - __le16
     - count
     - 实际跟随此头部的 dx_entry 的数量，再加上头部本身的 1   - - 0x24
     - __le32
     - block
     - （在目录文件内）通向最左侧叶子块的块号，即包含哈希值最低条目的叶子   - - 0x28
     - struct dx_entry
     - entries[^0^]
     - 适合放入数据块剩余部分的尽可能多8 字节 `struct dx_entry`
目录哈希是下列值之一
   :widths: 16 64
   :header-rows: 1

   - - Value
     - Description
   - - 0x0
     - 传统（Legacy）   - - 0x1
     - MD4（Half MD4）   - - 0x2
     - Tea銆?   - - 0x3
     - 传统，无符号   - - 0x4
     - MD4，无符号   - - 0x5
     - Tea，无符号   - - 0x6
     - Siphash銆。
htree 的内部节点记录为 `struct dx_node`，其长度也是一个完整的数据块：

   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - fake.inode
     - 零，使该条目看起来像未在使用   - - 0x4
     - __le16
     - fake.rec_len
     - 块的大小，以隐藏所dx_node 数据   - - 0x6
     - u8
     - name_len
     - 零。这未使的目录项没有名称   - - 0x7
     - u8
     - file_type
     - 零。这未使的目录项没有文件类型   - - 0x8
     - __le16
     - limit
     - 可跟随此头部dx_entry 的最大数量，再加上头部本身的 1   - - 0xA
     - __le16
     - count
     - 实际跟随此头部的 dx_entry 的数量，再加上头部本身的 1   - - 0xE
     - __le32
     - block
     - （在目录文件内）与该块最低哈希值对应的块号。该值存储在父块中   - - 0x12
     - struct dx_entry
     - entries[^0^]
     - 适合放入数据块剩余部分的尽可能多8 字节 `struct dx_entry`
同时存在`struct dx_root` `struct dx_node` 中的哈希映射记录`struct dx_entry`，其长度8 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - hash
     - 哈希码   - - 0x4
     - __le32
     - block
     - htree 中下一个节点的块号（在目录文件内，而非文件系统块）
（如果你觉得这一切都相当巧妙而奇特，作者也是这么想的。）

如果启用了元数据校验和，目录块的最8 个字节（正好是一dx_entry 的长度）被用来存储包含校验和`struct dx_tail`。dx_root/dx_node 结构中的 `limit` `count` 条目会进行必要调整，以将 dx_tail 放入块中。如果没有空间存dx_tail，会通知用户运行 e2fsck -D 来重建目录索引（这将确保有空间存放校验和）。dx_tail 结构8 字节，看起来如下
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - u32
     - dt_reserved
     - 未使用（但奇怪地仍是校验和的一部分）   - - 0x4
     - __le32
     - dt_checksum
     - htree 目录块的校验和
校验和是根据文件系统 UUID、htree 索引头部（dx_root dx_node）、所有在使用htree 索引（dx_entry），以及 dt_checksum 初始设为 0 的尾部块（dx_tail）计算得出的