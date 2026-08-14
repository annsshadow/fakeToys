## 优化的 MPEG 文件系统（OMFS）


## 概述


OMFS 是由 SonicBlue 创建的文件系统，用于 ReplayTV DVR 和 Rio Karma MP3 播放器。该文件系统是基于区段（extent）的，使用 2k 到 8k 的块大小，并采用基于哈希的目录。此文件系统驱动可用于读写来自这些设备的磁盘。

注意，不建议用此文件系统替代通用文件系统用于你自己的流媒体设备。原生的 Linux 文件系统可能会表现更好。

更多信息见：

    http://linux-karma.sf.net/

各种实用工具，包括 mkomfs 和 omfsck，随 omfsprogs 一起提供，可在以下地址获取：

    https://bobcopeland.com/karma/

其 README 中包含说明。

## 选项


OMFS 支持以下挂载时选项：

    ============   ========================================
    uid=n          使所有文件归指定用户所有
    gid=n          使所有文件归指定组所有
    umask=xxx      将权限 umask 设置为 xxx
    fmask=xxx      为文件将 umask 设置为 xxx
    dmask=xxx      为目录将 umask 设置为 xxx
    ============   ========================================

## 磁盘格式


OMFS 区分“sysblock”和普通数据块。sysblock 组由超级块信息、文件元数据、目录结构和区段组成。每个 sysblock 都有一个头部，包含整个 sysblock 的 CRC，并且可能在磁盘上的连续块中进行镜像。sysblock 的大小可能小于数据块，但由于二者都由相同的 64 位块号寻址，较小 sysblock 中的任何剩余空间都未被使用。

```

    struct omfs_header {
	    __be64 h_self;                  /* FS block where this is located */
	    __be32 h_body_size;             /* size of useful data after header */
	    __be16 h_crc;                   /* crc-ccitt of body_size bytes */
	    char h_fill1[2];
	    u8 h_version;                   /* version, always 1 */
	    char h_type;                    /* OMFS_INODE_X */
	    u8 h_magic;                     /* OMFS_IMAGIC */
	    u8 h_check_xor;                 /* XOR of header bytes before this */
	    __be32 h_fill2;
    };

```
```

    struct omfs_inode {
	    struct omfs_header i_head;      /* header */
	    __be64 i_parent;                /* parent containing this inode */
	    __be64 i_sibling;               /* next inode in hash bucket */
	    __be64 i_ctime;                 /* ctime, in milliseconds */
	    char i_fill1[35];
	    char i_type;                    /* OMFS_[DIR,FILE] */
	    __be32 i_fill2;
	    char i_fill3[64];
	    char i_name[OMFS_NAMELEN];      /* filename */
	    __be64 i_size;                  /* size of file, in bytes */
    };

```
OMFS 中的目录实现为一个大型哈希表。文件名被哈希后，从 OMFS_DIR_START 开始插入到桶列表中。查找需要哈希文件名，然后遍历 i_sibling 指针，直到在 i_name 上找到匹配。空桶由全为 1（~0）的块指针表示。

一个文件是一个 omfs_inode 结构，其后跟着一个从 ```

    struct omfs_extent_entry {
	    __be64 e_cluster;               /* start location of a set of blocks */
	    __be64 e_blocks;                /* number of blocks after e_cluster */
    };

    struct omfs_extent {
	    __be64 e_next;                  /* next extent table location */
	    __be32 e_extent_count;          /* total # extents in this table */
	    __be32 e_fill;
	    struct omfs_extent_entry e_entry;       /* start of extent entries */
    };

开始的区段表。每个区段保存块偏移，后跟分配给该区段的块数。每个表中的最后一个区段是一个终止符，其 e_cluster 为 ~0，e_blocks 为表中区块总数的反码。

如果该表溢出，会写入一个延续 inode，并由 e_next 指向。这些延续 inode 有头部，但缺少 inode 结构的其余部分。
