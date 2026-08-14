
## pNFS SCSI 布局服务器用户指南


本文档描述 Linux NFS 服务器中对 pNFS SCSI 布局的支持。借助 pNFS SCSI 布局，NFS 服务器充当 pNFS 的元数据服务器（MDS），除了处理对 NFS 导出（export）的所有元数据访问外，它还会向客户端分发布局，使客户端能够直接访问与客户端共享的底层 SCSI LUN。

要在 Linux NFS 服务器上使用 pNFS SCSI 布局，导出的文件系统需要支持 pNFS SCSI 布局（目前仅 XFS），并且该文件系统必须位于一个客户端与 MDS 都可访问的 SCSI LUN 上。截至目前，文件系统需要直接位于导出的 LUN 上，尚不支持在 MDS 与客户端上对 LUN 进行条带化（striping）或拼接（concatenation）。

在启用了 CONFIG_NFSD_SCSI 构建的服务器上，如果文件系统使用 "pnfs" 选项导出，且底层 SCSI 设备支持持久预留（persistent reservations），则 pNFS SCSI 卷支持会自动启用。在客户端上，确保内核启用了 CONFIG_PNFS_BLOCK 选项，并且文件系统使用 NFSv4.1 协议版本挂载（mount -o vers=4.1）。

如果 nfsd 服务器需要隔离（fence）一个无响应的客户端，而隔离操作失败，服务器会在系统日志中记录一条格式如下的警告消息：

    FENCE failed client[IP_address] clid[#n] device[dev_name]

    其中：

    - IP_address：指受影响客户端的 IP 地址。
    - #n：表示唯一的客户端标识符。
    - dev_name：指定与此次隔离尝试相关的块设备名称。

服务器会无限期地反复重试该操作。在此期间，对所有其他客户端访问受影响文件都受限。这是为了防止多个客户端同时访问同一文件时潜在的数据损坏。

要为其他客户端恢复对受影响文件的访问，管理员需要采取以下操作：

    - 关闭或断电被隔离的客户端。
```

        echo 'expire' > /proc/fs/nfsd/clients/clid/ctl

    其中：

      - clid：是系统日志中显示的唯一客户端标识符。


```
