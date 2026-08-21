
UBI 文件系统


简


UBIFS 文件系统代表 UBI File System。UBI 代表“Unsorted Block Images”（无序块映像）。UBIFS 是一种闪存文件系统，这意味着它专为配合闪存设备而设计。需要理解的是，UBIFS Linux 中任何传统文件系统（Ext2、XFS、JFS 等）完全不同。UBIFS 代表了与 MTD 设备（而非块设备）配合工作的一类文件系统。此类中的另一Linux 文件系统JFFS2

为了更清楚说明，下面将对 MTD 设备与块设备进行简单对比

1 MTD 设备代表闪存设备，由相当大的擦除块组成，通常约为 128KiB。块设备由较小的块组成，通常512 字节
2 MTD 设备支持 3 种主要操作——从擦除块内某个偏移读取、向擦除块内某个偏移写入，以及擦除整个擦除块。块设备支持 2 种主要操作——读取整个块和写入整个块
3 必须先擦除整个擦除块，然后才能重写其内容。块则可以直接重写
4 擦除块在经过一定次数的擦除循环后会磨损——对SLC NAND NOR 闪存通常10 万到 10 亿次，对MLC NAND 闪存1 千到 1 万次。块不具有磨损特性
5 擦除块可能变为坏块（仅发生在 NAND 闪存上），软件需要处理这种情况。硬盘上的块通常不会变坏，因为硬件具有替换坏块的机制，至少在现代 LBA 磁盘中是如此

为何 UBIFS 与传统文件系统大不相同应当相当明显

UBIFS 构建UBI 之上。UBI 是一个独立的软件层，可在 drivers/mtd/ubi 中找到。UBI 基本上是一个卷管理与磨损均衡层。它提供所谓的 UBI 卷，这是一种比 MTD 设备更高层的抽象。UBI 设备的编程模型与 MTD 设备非常相似——它们仍由较大的擦除块组成，具有擦除操作，但 UBI 设备不存在磨损和坏块等限制（即上述列表中的第 4 和第 5 项）

从某种意义上说，UBIFS JFFS2 文件系统的下一代，但它JFFS2 截然不同且互不兼容。主要区别如下

- JFFS2 构建MTD 设备之上，UBIFS 依赖 UBI 并工作于 UBI 卷之上
- JFFS2 没有介质上的索引，必须在挂载时构建，这需要完整扫描介质。UBIFS 将文件系统索引信息维护在闪存介质上，不需要完整扫描介质，因此挂载速度JFFS2 快很多倍
- JFFS2 是写透（write-through）文件系统，UBIFS 支持写回（write-back），这使UBIFS 在写入时快得多

JFFS2 类似，UBIFS 支持即时（on-the-fly）压缩，从而可以在闪存中容纳相当多的数据

JFFS2 类似，UBIFS 能够容忍不干净重启和掉电。它不需要像 fsck.ext2 那样的工具。UBIFS 会自动重放其日志并从崩溃中恢复，确保闪存上的数据结构保持一致

UBIFS 以对数方式扩展（它使用的大多数数据结构都是树），因此挂载时间和内存消耗不会像 JFFS2 那样随闪存大小线性增长。这是因UBIFS 将文件系统索引维护在闪存介质上。然而，UBIFS 依赖UBI 是线性扩展的，因UBI/UBIFS 整体栈是线性扩展的。尽管如此，UBI/UBIFS 的扩展性仍明显优于 JFFS2

UBIFS 的作者认为，有可能开发出同样以对数方式扩展的 UBI2。UBI2 将支持与 UBI 相同API，但UBI 二进制不兼容。因UBIFS 无需修改即可使用 UBI2


挂载选项


(*) == 默认值

====================	=======================================================
bulk_read		一次性读取更多数据，以利用顺序读取更快的闪存介质
no_bulk_read (*)	不进行批量读
no_chk_data_crc (*)	跳过数据节点上的 CRC 校验以提高读取性能
			仅当闪存介质高度可靠时才使用此选项。其副作用是
			文件内容的损坏可能不会被察觉
chk_data_crc		不跳过数据节点上CRC 校验
compr=none		覆盖默认压缩器并将其设置"none"
compr=lzo		覆盖默认压缩器并将其设置"lzo"
compr=zlib		覆盖默认压缩器并将其设置"zlib"
auth_key=		指定用于文件系统认证的密钥
			传入此选项会使认证变为强制要求
			所传入的密钥必须存在于内核 keyring 中，
			且类型必须为 'logon'
auth_hash_name=		用于认证的哈希算法，同时用于哈希计算与创HMAC
			典型取值包"sha256" "sha512"
====================	=======================================================


快速使用说


要挂载的 UBI 卷使"ubiX_Y" "ubiX:NAME" 语法指定，其"X" UBI 设备号，"Y" UBI 卷号NAME" UBI 卷名

```

    $ mount -t ubifs ubi0_0 /mnt/ubifs

```
UBI 设备 0 "rootfs" 卷挂载到 /mnt/ubifsrootfs" 为卷
```

    $ mount -t ubifs ubi0:rootfs /mnt/ubifs

```
以下是将 mtd0 附加UBI 并挂"rootfs" 卷的内核引导参数示例
ubi.mtd=0 root=ubi0:rootfs rootfstype=ubifs

参考文


MTD 网站上的 UBIFS 文档与常见问操作指南

- http://www.linux-mtd.infradead.org/doc/ubifs.html
- http://www.linux-mtd.infradead.org/faq/ubifs.html
