## pNFS 块布局（block layout）服务器端用户指南


Linux NFS 服务器现在支持 pNFS 块布局扩展。在此情况下，NFS 服务器充当 pNFS 的
元数据服务器（MDS），除了处理对 NFS 导出（export）的所有元数据访问外，它还会
向客户端分发布局，让客户端直接访问与客户端共享的底层块设备。

要使用 pNFS 块布局配合 Linux NFS 服务器，被导出的文件系统需要支持 pNFS 块布局
（目前仅 XFS），并且该文件系统必须位于 MDS 之外客户端也能访问的共享存储上
（通常是 iSCSI）。截至目前，文件系统需要直接位于导出的卷上，尚不支持在 MDS 与
客户端上对卷进行条带化或拼接。

在服务器端，如果文件系统支持，pNFS 块卷支持会自动启用。在客户端，需确保内核
启用了 CONFIG_PNFS_BLOCK 选项，nfs-utils 中的 blkmapd 守护进程正在运行，并且
文件系统使用 NFSv4.1 协议版本挂载（mount -o vers=4.1）。

如果 nfsd 服务器需要对一个无响应的客户端进行隔离（fence），它会调用
/sbin/nfsd-recall-failed，第一个参数设为客户端的 IP 地址，第二个参数设为待隔离
文件系统的设备节点（不带 /dev 前缀）。下面是一个示例文件，展示了：

```
	cat > /sbin/nfsd-recall-failed << EOF

```

	#!/bin/sh

	CLIENT="$1"
	DEV="/dev/$2"
	EVPD=`sg_inq --page=0x80 ${DEV} | \
		grep "Unit serial number:" | \
		awk -F ': ' '{print $2}'`

	echo "fencing client ${CLIENT} serial ${EVPD}" >> /var/log/pnfsd-fence.log
	EOF

如果 nfsd 服务器需要对一个无响应的客户端进行隔离，而隔离操作失败，服务器会在
系统日志中记录一条格式如下的警告信息：

    FENCE failed client[IP_address] clid[#n] device[dev_name]

    其中：

    - IP_address：指受影响客户端的 IP 地址。
    - #n：表示唯一的客户端标识符。
    - dev_name：指与此次隔离尝试相关的块设备名称。

服务器会无限期地反复重试该操作。在此期间，所有其它客户端对该受影响文件的访问
都会受限。这是为了防止多个客户端同时访问同一文件时可能出现的数据损坏。

要让其它客户端恢复对该受影响文件的访问，管理员需要执行以下操作：

    - 关闭或断电被隔离的客户端。
```

        echo 'expire' > /proc/fs/nfsd/clients/clid/ctl

    其中：

      - clid：系统日志中显示的唯一客户端标识符。

```
