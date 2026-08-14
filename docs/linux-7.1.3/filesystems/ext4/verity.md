### Verity 文件


ext4 支持 fs-verity，这是一种文件系统特性，为单个只读文件提供基于 Merkle 树的
哈希。fs-verity 的大部分内容对所有支持它的文件系统是通用的；有关 fs-verity
文档请参阅 Documentation/filesystems/fsverity.rst <fsverity>。但是，verity
元数据的磁盘布局是文件系统特定的。在 ext4 上，verity 元数据存储在文件数据
本身末尾之后，格式如下：

- 零填充到下一个 65536 字节边界。此填充实际上无需在磁盘上分配，即它可以是一个
  空洞。

- Merkle 树，如 :ref:`Documentation/filesystems/fsverity.rst
  <fsverity_merkle_tree>` 中所述，树的层级按从根到叶的顺序存储，每个层级内的
  树块按自然顺序存储。

- 零填充到下一个文件系统块边界。

- verity 描述符，如 Documentation/filesystems/fsverity.rst <fsverity_descriptor>
  中所述，可选择性地附加签名 blob。

- 零填充到文件系统块边界前 4 字节的下一个偏移处。

- verity 描述符的大小（以字节为单位），为一个 4 字节小端整数。

Verity inode 设置了 EXT4_VERITY_FL，并且它们必须使用 extent，即必须设置
EXT4_EXTENTS_FL 且必须清除 EXT4_INLINE_DATA_FL。它们可以设置 EXT4_ENCRYPT_FL，
此时 verity 元数据与数据本身一起被加密。

Verity 文件不能在 verity 元数据末尾之后分配块。

Verity 与 DAX 不兼容，试图在文件上同时设置这两个标志将失败。
