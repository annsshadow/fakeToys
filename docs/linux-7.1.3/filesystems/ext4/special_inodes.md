锘?### 鐗规畩 inode


ext4 为特殊功能保留了一inode，如下：

   :widths: 6 70
   :header-rows: 1

   - - inode 编号
     - 用   - - 0
     - 不存在；没有 inode 0   - - 1
     - 坏块列表   - - 2
     - 根目录   - - 3
     - 用户配额   - - 4
     - 组配额   - - 5
     - 引导加载程序   - - 6
     - 反删除目录   - - 7
     - 保留的组描述inode。（“resize inode”）
   - - 8
     - 日志 inode   - - 9
     - “excludeinode，用于快照（)   - - 10
     - 副本 inode，用于某些非上游特性？
   - - 11
     - 传统的第一个非保留 inode。通常这是 lost+found 目录。见超级块中s_first_ino
注意，还有从非保inode 编号分配的一inode，用于其他文件系统特性，
它们未从标准目录层次引用。这些通常由超级块引用。它们是
   :widths: 20 50
   :header-rows: 1

   - - 瓒呯骇鍧楀瓧娈?     - 鎻忚堪

   - - s_lpf_ino
     - lost+found 目录inode 编号   - - s_prj_quota_inum
     - 跟踪项目配额的配额文件的 inode 编号   - - s_orphan_file_inum
     - 跟踪孤立 inode 的文件的 inode 编号