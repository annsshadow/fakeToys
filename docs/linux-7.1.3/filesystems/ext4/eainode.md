### 大扩展属性值


为了使 ext4 能够存储无法放入 inode 或附加到 inode 的
单个扩展属性块的扩展属性值，EA_INODE 特性允许我们将该值存储于
一个常规文件 inode 的数据块中。此“EA inode”仅从扩展属性
名索引链接，且不能出现在目录项中。inode 的 i_atime 字段
用于存储 xattr 值的校验和；i_ctime/i_version 存储 64 位引用计数，
从而可在多个拥有者 inode 之间共享大 xattr 值。为与此特性的
旧版本保持向后兼容，i_mtime/i_generation **可能**存储对
（在 EA inode 未被多个 inode 引用的情况下）单个拥有者 inode 的
inode 编号和 i_generation 的反向引用，以验证被访问的 EA inode
是否为正确的那个。
