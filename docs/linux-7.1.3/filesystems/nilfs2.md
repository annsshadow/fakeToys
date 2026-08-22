
## NILFS2

NILFS2 是一个支持连续快照的日志结构文件系统（LFS）。除了整个文件系统的版本控制能力外，用户
甚至还能恢复几秒前被误覆盖或误删除的文件。由NILFS2 能像传统LFS 一样保持一致，它能系统崩溃后快速恢复
NILFS2 每隔几秒或基于每次同步写入创建一个检查点（前提是有变更）。用户可以在连续创建检查点中选择重要的版本，并将其更改为快照，快照会一直保留直到被改回检查点
在卷被写满之前，快照数量没有限制。每个快照都可以与其可写挂载并发地作为一个只读文件系挂载，这一特性便于在线备份
用户空间工具包含nilfs-utils 软件包中，可从以下下载页面获取。至少需"mkfs.nilfs2"mount.nilfs2"umount.nilfs2" "nilfs_cleanerd"（即所谓的 cleaner 垃圾收集器）。关于这些工具的细节请参见该软件包中所含的手册页
:Project web page:    https://nilfs.sourceforge.io/
:Download page:       https://nilfs.sourceforge.io/en/download.html
:List info:           http://vger.kernel.org/vger-lists.html#linux-nilfs

## 注意事项

NILFS2 尚不支持的特性：

 - atime
 - extended attributes（扩展属性）
 - POSIX ACLs
 - quotas（配额）
 - fsck
 - defragmentation（碎片整理）

## 挂载选项

NILFS2 支持以下挂载选项(*) == 默认

======================= =======================================================
barrier(*)		启用/禁用写屏障（write barrier）的使用。这要求
nobarrier		一个能够支持屏障的 IO 栈；如果 nilfs 在屏障写入时
			遇到错误，它会再次禁用并给出警告errors=continue		文件系统出错时继续运行errors=remount-ro(*)	出错时将文件系统重新挂载为只读errors=panic		如果发生错误，触panic 并停机cp=n			指定要挂载的快照的检查点编号。检查点和快照由 lscp
			用户命令列出。只有标记为快照的检查点才能用此选项挂载			快照是只读的，因此必须同时指定只读挂载选项order=relaxed(*)	采用宽松的顺序语义，允许在没有元数据更新进行时，			已修改的数据块写入磁盘而无需创建检查点。除数据块的
			更新仍保持原子性外，此模式等价ext3 文件系统			有序数据（ordered data）模式。这将提升覆盖写入的
			同步写性能order=strict		采用严格的按序语义，保留包括覆盖写入数据块在内的
			所有文件操作的顺序。也就是说，保证崩溃后恢复的文件
			系统中不会发生事件的超越（overtaking）norecovery		禁用挂载时文件系统的恢复。对于只读挂载或快照，这			禁用对设备的一切写访问。对于不干净卷上的读写挂载，
			此选项会失败discard			启用/禁用 discard/TRIM 命令的使用nodiscard(*)		discard/TRIM 命令在块被释放时发送到底层块设备。这			SSD 设备和稀精简配置LUN 很有用======================= =======================================================

## Ioctls

NILFS2 有一些特定功能可由应用程序通过系统调用接口访问。所NILFS2 特定ioctl 列于下表中
NILFS2 特定 ioctl 表：

 ============================== ===============================================
 Ioctl			        描述
 ============================== ===============================================
 NILFS_IOCTL_CHANGE_CPMODE      在检查点和快照状态之间更改给定检查点的模式			        ioctl 用于 chcp mkcp 工具
 NILFS_IOCTL_DELETE_CHECKPOINT  NILFS2 文件系统中移除检查点。该 ioctl
			        用于 rmcp 工具
 NILFS_IOCTL_GET_CPINFO         返回所请求检查点的信息。该 ioctl 用于 lscp
			        工具以及 nilfs_cleanerd 守护进程
 NILFS_IOCTL_GET_CPSTAT         返回检查点统计信息。该 ioctl 用于 lscp、rmcp
			        工具以及 nilfs_cleanerd 守护进程
 NILFS_IOCTL_GET_SUINFO         返回所请求段的段使用情况信息。该 ioctl 用于
			        lssu、nilfs_resize 工具以及 nilfs_cleanerd
			        守护进程
 NILFS_IOCTL_SET_SUINFO         修改所请求段的使用情况信息。该 ioctl 			        nilfs_cleanerd 守护进程使用，以跳过不必要的			        清理操作，并减少因冗余移动在用块而导致的性能
			        损失或闪存设备磨损
 NILFS_IOCTL_GET_SUSTAT         返回段使用统计信息。该 ioctl 用于 lssu			        nilfs_resize 工具以及 nilfs_cleanerd 守护进程
 NILFS_IOCTL_GET_VINFO          返回关于虚拟块地址的信息。该 ioctl 			        nilfs_cleanerd 守护进程使用
 NILFS_IOCTL_GET_BDESCS         返回关于磁盘块号描述符的信息。该 ioctl 			        nilfs_cleanerd 守护进程使用
 NILFS_IOCTL_CLEAN_SEGMENTS     在来自用户空间的请求参数环境下执行垃圾收			        操作。该 ioctl nilfs_cleanerd 守护进程使用
 NILFS_IOCTL_SYNC               创建一个检查点。该 ioctl 用于 mkcp 工具
 NILFS_IOCTL_RESIZE             调整 NILFS2 卷的大小。该 ioctl 用于
			        nilfs_resize 工具
 NILFS_IOCTL_SET_ALLOC_RANGE    以字节定义段的下限和上限。该 ioctl 用于
			        nilfs_resize 工具 ============================== ===============================================

## NILFS2 用法

```
 # mkfs -t nilfs2 /dev/block_device
 # mount -t nilfs2 /dev/block_device /dir

```
这也会通过挂载辅助程序（mount.nilfs2）调cleaner
检查点和快照由以下命令管理。它们的手册页包含在上述 nilfs-utils 软件包中
  ====     ===========================================================
  lscp     列出检查点或快照  mkcp     创建一个检查点或快照  chcp     将已有的检查点更改为快照，或反之  rmcp     使指定的检查点失效  ====     ===========================================================

```
 # mount -t nilfs2 -r -o cp=<cno> /dev/block_device /snap_dir

```
其中 <cno> 是快照的检查点编号
```
 # umount /dir

```
然后，cleaner 守护进程会被 umount 辅助程序（umount.nilfs2）自动关闭
## 磁盘格式

除超级块（SB）和0 段外，一nilfs2 卷被等量划分为若干个段。段是日志的容器。每个日志由
摘要信息块、负载块组成
```
   ______________________________________________________
  | |SB| | Segment | Segment | Segment | ... | Segment | |
  |_|__|_|____0____|____1____|____2____|_____|____N____|_|
  0 +1K +4K       +8M       +16M      +24M  +(8MB x N)
       .             .            (Typical offsets for 4KB-block)
    .                  .
  .______________________.
  | log | log |... | log |
  |__1__|__2__|____|__m__|
        .       .
      .               .
    .                       .
  .______________________________.
  | Summary | Payload blocks  |SR|
  |_blocks__|_________________|__|

```
负载块按文件组织，每个文件由以下部分组成
```
    |<---       File-A        --->|<---       File-B        --->|
   _______________________________________________________________
    | Data blocks | B-tree blocks | Data blocks | B-tree blocks | ...
   _|_____________|_______________|_____________|_______________|_

```
由于只有被修改的块才会写入日志，日志中可能包含没有数据块B-tree 节点块的文件
块的组织方式记录在摘要信息块中，其中包含每个文件的头部结构（nilfs_segment_summary），例如
```
  _________________________________________________________________________
 | Summary | finfo | binfo | ... | binfo | finfo | binfo | ... | binfo |...
 |_blocks__|___A___|_(A,1)_|_____|(A,Na)_|___B___|_(B,1)_|_____|(B,Nb)_|___

```
日志包含常规文件、目录文件、符号链接文件以及若干元数据文件。元数据文件是用于维护文件系元数据的文件。当前版本的 NILFS2 使用
```
 1) Inode file (ifile)             -- 存储磁盘上的 inode
 2) Checkpoint file (cpfile)       -- 存储检查点
 3) Segment usage file (sufile)    -- 存储段的分配状 4) Data address translation file  -- 将虚拟块号映射到通常    (DAT)                             块号。该文件使磁盘上的块可重定位
```
```
  _________________________________________________________________________
 | Summary | regular file | file  | ... | ifile | cpfile | sufile | DAT |SR|
 |_blocks__|_or_directory_|_______|_____|_______|________|________|_____|__|

```
为了跨越段边界，这一文件序列可能被拆分为多个日志。应被逻辑上视为一个日志的日志序列，由摘要中标记的标志界定。nilfs2 的恢复代码会查看这一边界信息以确保更新的原子性
超级根块为每个检查点插入。它包含三个特殊inode：DAT、cpfile sufile inode。常规文件目录、符号链接和其他特殊文件inode 包含ifile 中。ifile 自身inode 包含cpfile 相应的检查点条目中。因此层级关系如下：

```
  Super block (SB)
       |
       v
  Super root block (the latest cno=xx)
       |-- DAT
       |-- sufile
       `-- cpfile
              |-- ifile (cno=c1)
              |-- ifile (cno=c2) ---- file (ino=i1)
              :        :          |-- file (ino=i2)
              `-- ifile (cno=xx)  |-- file (ino=i3)
                                  :        :
                                  `-- file (ino=yy)
                                    ( 常规文件、目录或符号链接 )

```
关于每个文件格式的详细信息，请参见位include/uapi/linux 目录下的 nilfs2_ondisk.h
关于 NILFS2 的设计，我们没有需要保护的专利或其他知识产权。允许复制该设计，期望其他操作系能够共享（挂载、读取、写入等）以该格式存储的数据