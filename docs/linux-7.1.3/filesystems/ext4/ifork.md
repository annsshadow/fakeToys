### inode.i_block 的内


根据 inode 所描述的文件类型，`inode.i_block` 中的 60 字节存储空间可以有不同的
用途。一般来说，普通文件和目录会将其用于文件块索引信息，而特殊文件则将其用于
特殊用途

#### 符号链接


如果符号链接的目标字符串长度小于 60 字节，则其目标将存储在该字段中。否则，
使用 extent（区）或块映射来分配数据块以存储链接目标

#### 直接/间接块寻址


ext2/3 中，文件块号通过（最多）三级1-1 块映射映射到逻辑块号。为了找
存储某个特定文件块的逻辑块，代码需要遍历这个逐级复杂的结构。请注意，这里既
没有魔数也没有校验和，无法提供任何程度的信心来确认该块不是一堆垃圾


   .. include:: blockmap.rst


   [表格省略，因LaTeX 不支持嵌套表格。]

请注意，采用这种块映射方案，即便对于一个很大的连续文件，也必须填写大量的映
数据！这种低效促使了下文将要讨论extent 映射方案的诞生

另请注意，使用这种映射方案的文件无法被放置在高于 2^32 个块的位置

#### Extent 鏍。


ext4 中，文件到逻辑块的映射已被 extent 树取代。在旧方案中，分配一段连续的
1000 个块需要用一个间接块来映射全1000 个条目；而使extent，映射被简化为
单个 `struct ext4_extent`，其 `ee_len = 1000`。如果启用了 flex_bg，则有可
用单extent 分配非常大的文件，从而显著减少元数据块的使用，并在一定程度上
提高磁盘效率。要使用此特性，inode 必须设置 extents 标志x80000）

Extent 被组织成一棵树。树的每个节点都`struct ext4_extent_header` 开头。如
该节点是内部节点（`eh.eh_depth` > 0），则表头之后跟`eh.eh_entries` 
`struct ext4_extent_idx` 实例；每个这样的索引条目指向一个包extent 树中更多
节点的块。如果该节点是叶子节点（`eh.eh_depth == 0`），则表头之后跟
`eh.eh_entries` `struct ext4_extent` 实例；这些实例指向文件的数据块。Extent
树的根节点存储在 `inode.i_block` 中，这使得前四个 extent 无需使用额外的元数据
块即可被记录

Extent 树表头记录在 `struct ext4_extent_header` 中，长度12 字节

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移
     - 大小
     - 名称
     - 描述
   - - 0x0
     - __le16
     - eh_magic
     - 魔数xF30A
   - - 0x2
     - __le16
     - eh_entries
     - 表头之后有效条目的数量
   - - 0x4
     - __le16
     - eh_max
     - 表头之后最多可以跟随的条目数量
   - - 0x6
     - __le16
     - eh_depth
     - extent 节点extent 树中的深度 = extent 节点指向数据块；
       否则，此 extent 节点指向其他 extent 节点。extent 树的深度最多为 5 层：
       一个逻辑块号最大为 `2^32`，而满`4*(((blocksize - 12)/12)^n) >= 2^32`
       的最`n` 5
   - - 0x8
     - __le32
     - eh_generation
     - 树的代（generation）。（Lustre 使用，但非标ext4。）

Extent 树的内部节点，也称为索引节点，记录为 `struct ext4_extent_idx`，长度为
12 字节

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移
     - 大小
     - 名称
     - 描述
   - - 0x0
     - __le32
     - ei_block
     - 此索引节点覆盖从 'block' 开始的文件中块
   - - 0x4
     - __le32
     - ei_leaf_lo
     - 树中下一extent 节点的块号的32 位。所指向的树节点可以是另一个内
       节点，也可以是如下所述的叶子节点
   - - 0x8
     - __le16
     - ei_leaf_hi
     - 上一字段的高 16 位
   - - 0xA
     - __u16
     - ei_unused
     -

Extent 树的叶子节点记录`struct ext4_extent`，长度同样为 12 字节

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移
     - 大小
     - 名称
     - 描述
   - - 0x0
     - __le32
     - ee_block
     - extent 覆盖的第一个文件块号
   - - 0x4
     - __le16
     - ee_len
     - extent 覆盖的块数量。如果该字段的<= 32768，则 extent 已初始化
       如果该字段的> 32768，则 extent 未初始化，实extent 长度`ee_len` -
       32768。因此，已初始化 extent 的最大长度为 32768 个块，未初始extent 
       最大长度为 32767 个块
   - - 0x6
     - __le16
     - ee_start_hi
     - extent 所指向块号的高 16 位
   - - 0x8
     - __le32
     - ee_start_lo
     - extent 所指向块号的低 32 位

在引入元数据校验和之前，extent 表头 + extent 条目在每extent 树数据块的末
总是至少留下 4 字节未分配的空间（因(2^x % 12) >= 4）。因此，32 位校验和会被
插入到这段空间中。inode 中的 4 extent 不需要校验和，因inode 本身已经进行
校验和。该校验和是针对文件系统 UUID、inode 号、inode 代（generation）以
整个 extent 块（直到校验和本身之前，但不包含校验和）计算的

`struct ext4_extent_tail` 长度4 字节

   :widths: 8 8 24 40
   :header-rows: 1

   - - 偏移
     - 大小
     - 名称
     - 描述
   - - 0x0
     - __le32
     - eb_checksum
     - extent 块的校验和，crc32c(uuid+inum+igeneration+extentblock)

#### 内联数据


如果文件系统启用了内联数据（inline data）特性，并且 inode 设置了相应标志，
文件数据的前 60 字节有可能存储在此处