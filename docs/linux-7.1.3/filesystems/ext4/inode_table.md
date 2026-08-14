### 索引节点表（Inode Table）


索引节点表在 mkfs 时静态分配。每个块组描述符指向表的起始位置，超级块记录每个组的
索引节点数量。有关索引节点表布局的更多信息，请参见
[inode documentation <inodes>](inode documentation <inodes>)。
