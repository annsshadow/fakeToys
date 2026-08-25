
### Extended Attributes


扩展属性（xattrs）通常存储在磁盘上独立的数据块中，并通过 `inode.i_file_acl*` inode 引用。扩展属性的首次使用似乎是用于存储文ACL 和其他安全数据（selinux）。借助 `user_xattr` 挂载选项，用户可以存储扩展属性，只要所有属性名都以 “user开头；这一限制Linux 3.0 之后似乎已消失
扩展属性有两个存放位置。第一个位置在每个 inode 条目末尾与下一inode 条目开头之间。例如，inode.i_extra_isize = 28 sb.inode_size = 256，则256 - (128 + 28) = 100 字节可用inode 内的扩展属性存储。扩展属性可找到的第二个位置是由 `inode.i_file_acl` 指向的块中。从 Linux 3.11 起，该块无法包含指向第二个扩展属性块（甚至一个簇的剩余块）的指针。理论上，每个属性的值可以存储在一个独立的数据块中，但Linux 3.11 起代码不允许这样做
键（key）通常被假定为 ASCIIZ 字符串，而值可以是字符串或二进制数据
扩展属性在 inode 之后存储时，有一4 字节长的`ext4_xattr_ibody_header`
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - h_magic
     - 用于标识的幻数，0xEA020000。该值由 Linux 驱动设置，不e2fsprogs 似乎不检查它（？
扩展属性块的起始处`struct ext4_xattr_header`，长 32 字节
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __le32
     - h_magic
     - 用于标识的幻数，0xEA020000   - - 0x4
     - __le32
     - h_refcount
     - 引用计数   - - 0x8
     - __le32
     - h_blocks
     - 使用的磁盘块数   - - 0xC
     - __le32
     - h_hash
     - 所有属性的哈希值   - - 0x10
     - __le32
     - h_checksum
     - 扩展属性块的校验和   - - 0x14
     - __u32
     - h_reserved[^3^]
     - 零
校验和是针对 FS UUID、扩展属性块64 位块号以及整个块（头 + 条目）计算的
`struct ext4_xattr_header` `struct ext4_xattr_ibody_header` 之后是一`struct ext4_xattr_entry` 数组；每个条目至少长 16 字节。当存储在外部块中时，`struct ext4_xattr_entry` 条目必须按排序顺序存储。排序顺序为 `e_name_index`，然后是 `e_name_len`，最后是 `e_name`。存储在 inode 内的属性不需要按排序顺序存储
   :widths: 8 8 24 40
   :header-rows: 1

   - - Offset
     - Type
     - Name
     - Description
   - - 0x0
     - __u8
     - e_name_len
     - 名称长度   - - 0x1
     - __u8
     - e_name_index
     - 属性名索引。下文有相关讨论   - - 0x2
     - __le16
     - e_value_offs
     - 该属性的值在存储它的磁盘块上的位置。多个属性可以共享同一个值。对inode 属性，该值相对于第一个条目的起始；对于块，该值相对于块的起始（即头）   - - 0x4
     - __le32
     - e_value_inum
     - 值所存储inode。零表示该值与本条目在同一块中。该字段仅在启用 INCOMPAT_EA_INODE 特性时使用   - - 0x8
     - __le32
     - e_value_size
     - 属性值的长度   - - 0xC
     - __le32
     - e_hash
     - 属性名与属性值的哈希值。内核不会为 inode 内属性更新哈希，因此对于这种情况该值必须为零，因为 e2fsck 会校验任何非零哈希，无论 xattr 位于何处   - - 0x10
     - char
     - e_name[e_name_len]
     - 属性名。不包含结尾NULL
属性值可以跟在条目表末尾之后。似乎要求它们按 4 字节边界对齐。这些值从块的末尾开始存储，并向 xattr_header/xattr_entry 表方向增长。当两者相撞时，溢出部分被放入一个独立的磁盘块。如果磁盘块填满，文件系统返-ENOSPC
`ext4_xattr_entry` 的前四个字段被置零以标记键列表的结束
#### Attribute Name Indices


从逻辑上讲，扩展属性是一系列 key=value 对。键被假定为NULL 结尾的字符串。为了减少键在磁盘上占用的空间，键字符串的开头会与属性名索引进行匹配。如果找到匹配，则设置属性名索引字段，并从键名中去掉匹配到的字符串。下面是名称索引值到键前缀的映射：

   :widths: 16 64
   :header-rows: 1

   - - Name Index
     - Key Prefix
   - - 0
     - （无前缀   - - 1
     - 鈥渦ser.鈥?   - - 2
     - 鈥渟ystem.posix_acl_access鈥?   - - 3
     - 鈥渟ystem.posix_acl_default鈥?   - - 4
     - 鈥渢rusted.鈥?   - - 6
     - 鈥渟ecurity.鈥?   - - 7
     - “system.”（inline_data？）
   - - 8
     - “system.richacl”（SuSE 内核？）

例如，如果属性键“user.fubar”，则属性名索引被设1，并“fubar名称被记录到磁盘上
#### POSIX ACLs


POSIX ACL Linux 内核（及 libacl）内ACL 格式的精简版本存储。关键区别在于版本号不同），并且 `e_id` 字段仅为具名用户和组 ACL 存储