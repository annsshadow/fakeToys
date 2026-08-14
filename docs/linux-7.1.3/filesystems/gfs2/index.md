
## 全局文件系统 2（Global File System 2）


## 概述


GFS2 是一个集群文件系统。它允许一组计算机构成一个集群，同时使用一个在它们之间共享的块设备（通过 FC、iSCSI、NBD 等）。GFS2 像本地文件系统一样对该块设备进行读写，但同时使用一个锁模块来允许这些计算机协调它们的 I/O，从而保持文件系统的一致性。GFS2 的一个巧妙特性是完全一致性——在一台机器上对文件系统所做的更改会立即出现在集群中的所有其他机器上。

GFS2 使用可互换的节点间锁机制，当前受支持的机制有：

  lock_nolock
    - 允许 GFS2 用作本地文件系统

  lock_dlm
    - 使用分布式锁管理器（dlm）进行节点间加锁。
      dlm 位于 linux/fs/dlm/

lock_dlm 依赖于上面 URL 中给出的用户空间集群管理系统。

要将 GFS2 用作本地文件系统，不需要外部的集群系统
```

  $ mkfs -t gfs2 -p lock_nolock -j 1 /dev/block_device
  $ mount -t gfs2 /dev/block_device /dir

```
gfs2-utils 包在所有集群节点上都是必需的；而对于 lock_dlm，你还需要按照文档配置 dlm 和 corosync 用户空间工具。

gfs2-utils 可以在 https://pagure.io/gfs2-utils 找到

GFS2 与之前版本的 GFS 在磁盘上不兼容，但两者非常接近。

gfs2-utils 提供以下手册页：

  ============		=============================================
  fsck.gfs2		修复文件系统
  gfs2_grow		在线扩展文件系统
  gfs2_jadd		在线向文件系统添加日志
  tunegfs2		操作、检查和调整文件系统
  gfs2_convert		将 gfs 文件系统就地转换为 GFS2
  mkfs.gfs2		创建文件系统
  ============		=============================================

## 实现说明


- [glocks](glocks)
- [uevents](uevents)
