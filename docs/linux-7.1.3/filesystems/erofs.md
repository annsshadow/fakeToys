
## EROFS - 增强型只读文件系统（Enhanced Read-Only File System

## 概述


EROFS 文件系统代表 Enhanced Read-Only File System（增强型只读文件系统）。它的目标是形成一种通用的只读文件系统方案，适用于各种只读使用场景，而不是仅仅关注节省存储空间而忽略运行时性能的任何副作用
它被设计为满足灵活性、特性可扩展性以及对用户负载友好等需求。除此之外，与类似方案相比，它仍然保持为一个简单的、适合随机访问的高性能文件系统，以消除不必要的 I/O 放大和常驻内存开销
它被实现为以下场景的更好选择
 - 只读存储介质；或

 - 完全可信的只读方案的一部分，这意味着出于安全或其他考虑，它需要是不可变的，并且与其发布的官方黄金镜像逐位（bit-for-bit）一致；以及

 - 希望通过使用紧凑布局、透明文件压缩与直接访问，在保证端到端性能的前提下最小化额外存储空间，特别是针对内存有限的嵌入式设备以及拥有大量容器的高密度主机
以下EROFS 的主要特性：

 - 小端（little endian）的磁盘设计
 - 支持基于块的分布以及基于 fscache 的文件级分布
 - 支持多个设备引用外部 blob，可用于容器镜像
 - 每个设备 32 位块地址，因此在 4KiB 块大小下目前最16TiB 地址空间
 - 针对不同需求提供两inode 布局
   =====================  ============  ======================================
                          compact (v1)  extended (v2)
   =====================  ============  ======================================
   Inode metadata size    32 bytes      64 bytes
   Max file size          4 GiB         16 EiB (also limited by max. vol size)
   Max uids/gids          65536         4294967296
   Per-inode timestamp    no            yes (64 + 32-bit timestamp)
   Max hardlinks          65536         4294967296
   Metadata reserved      8 bytes       18 bytes
   =====================  ============  ======================================

 - 可选地支持扩展属性（extended attributes）；

 - 支持加速否定扩展属性查找的布隆过滤器（bloom filter）；

 - 通过使用扩展属性支POSIX.1e ACL
 - 可选地支持透明数据压缩   LZ4、MicroLZMA、DEFLATE Zstandard 算法可按文件粒度使用；此外，还支持原地（inplace）解压缩，以避免压缩缓冲区的弹跳（bounce）以及不必要的页缓存颠簸（thrashing）
 - 支持基于块的数据去重以及滚动哈希（rolling-hash）压缩数据去重；

 - 支持与按字节寻址的非对齐元数据或更小块大小方案相比的 tailpacking 内联
 - 支持将尾部数据合并为特殊 inode 作为片段（fragments）
 - 支持大页（large folios）以利用 THP（Transparent Hugepages，透明大页）；

 - 支持未压缩文件上的直I/O，以避免 loop 设备的双重缓存；

 - 支持未压缩镜像上FSDAX，用于安全容器与 ramdisk，以消除不必要的页缓存
 - 支持基于 Fscache 基础设施的文件级按需加载
以下 git 树提供了仍在开发中的文件系统用户空间工具，例如格式化工具（mkfs.erofs）、磁盘一致性与兼容性检查工具（fsck.erofs）以及调试工具（dump.erofs）：

- git://git.kernel.org/pub/scm/linux/kernel/git/xiang/erofs-utils.git

更多信息请参考文档站点：

- https://erofs.docs.kernel.org

欢迎报告缺陷与提交补丁，请帮助我们并将其发送到以下 linux-erofs 邮件列表
- linux-erofs mailing list   <linux-erofs@lists.ozlabs.org>

## 挂载选项


===================    =========================================================
(no)user_xattr         设置扩展用户属性。注意：若选中 CONFIG_EROFS_FS_XATTR，默认启xattr(no)acl                设置 POSIX 访问控制列表。注意：若选中 CONFIG_EROFS_FS_POSIX_ACL，默认启aclcache_strategy=%s      从此刻起选择缓存解压缩的策略
		       ==========  =============================================
                         disabled  仅进行原I/O 解压缩；
                        readahead  缓存最后一个不完整的压缩物理簇以供后续读取。其余压缩物理簇仍进行原I/O 解压缩；
                       readaround  缓存不完整压缩物理簇的两端以供后续读取。其余压缩物理簇仍进行原I/O 解压缩		       ==========  =============================================
dax={always,never}     使用直接访问（无页缓存）。参                       Documentation/filesystems/dax.rstdax                    一个遗留选项，是 `dax=always` 的别名device=%s              指定一个要一起使用的额外设备路径directio               （对于文件支持的挂载）使用直I/O 访问后端文件，若支持则启用异I/Ofsid=%s                Fscache 后端指定一个文件系统镜IDdomain_id=%s           fscache 模式指定一个可信域 ID，以便由 blob ID 标识、拥有相blob 的不同镜像可以在同一可信域内共享存储。也用于启用 inode 页共享的不同文件系统在同一可信域内共享页缓存fsoffset=%llu          为主设备指定块对齐的文件系统偏移inode_share            为本文件系统启用 inode 页共享。在同一ID 内内容相同的 inode 可以共享页缓存===================    =========================================================

## Sysfs 条目


关于已挂erofs 文件系统的信息可/sys/fs/erofs 中找到。每个已挂载的文件系统都会在 /sys/fs/erofs 下拥有一个基于其设备名（/sys/fs/erofs/sda）的目录（另Documentation/ABI/testing/sysfs-fs-erofs
## 磁盘细节


### 概要

不同于其它只读文件系统，EROFS 卷被设计
```
                                |-> aligned with the block size
   ____________________________________________________________
  | |SB| | ... | Metadata | ... | Data | Metadata | ... | Data |
  |_|__|_|_____|__________|_____|______|__________|_____|______|
  0 +1K
```

所有数据区域都应对齐到块大小，但元数据区域不一定。现在所有元数据可以在两个不同的空间（视图）中观察到
 1. Inode 元数据空
    每个有效 inode 应对齐到一inode 槽（slot），这是一个固定值（32 字节），设计上保持与紧凑 inode 大小一致
    每个 inode 都可以通过如下公式直接找到         inode offset = meta_blkaddr ** block_size + 32 ** nid

```
                                 |-> aligned with 8B
                                            |-> followed closely
     + meta_blkaddr blocks                                      |-> another slot
       _____________________________________________________________________
     |  ...   | inode |  xattrs  | extents  | data inline | ... | inode ...
     |________|_______|(optional)|(optional)|__(optional)_|_____|__________
              |-> aligned with the inode slot size
                   .                   .
                 .                         .
               .                              .
             .                                    .
           .                                         .
         .                                              .
       .____________________________________________________|-> aligned with 4B
       | xattr_ibody_header | shared xattrs | inline xattrs |
       |____________________|_______________|_______________|
       |->    12 bytes    <-|->x * 4 bytes<-|               .
                           .                .                 .
                     .                      .                   .
                .                           .                     .
            ._______________________________.______________________.
            | id | id | id | id |  ... | id | ent | ... | ent| ... |
            |____|____|____|____|______|____|_____|_____|____|_____|
                                            |-> aligned with 4B
                                                        |-> aligned with 4B
```

    Inode 可以32 64 字节，可以通过所inode 版本都拥有的公共字段 i_format 来区分：

```
        __________________               __________________
       |     i_format     |             |     i_format     |
       |__________________|             |__________________|
       |        ...       |             |        ...       |
       |                  |             |                  |
       |__________________| 32 bytes    |                  |
                                        |                  |
                                        |__________________| 64 bytes
```

    Xattrs、extents、data inline 在相inode 之后以适当对齐放置，并且对于不同的数据映射它们可能是可选的。目前总共支持 5 种数据布局
    ==  ====================================================================
     0  不含 data inline 的扁平文件数据（extent）；
     1  固定大小输出数据压缩（使用非紧凑索引）；
     2  tail packing data inline 的扁平文件数据（extent）；
     3  固定大小输出数据压缩（使用紧凑索引，v5.3+）；
     4  基于块的文件（v5.15+）    ==  ====================================================================

    可xattrs 的大小由 inode 头中i_xattr_count 指示。大 xattrs 或被许多不同文件共享xattrs 可以存储在共xattrs 元数据中，而不是紧inode 之后内联
 2. 共享 xattrs 元数据空
    共享 xattrs 空间与上面的 inode 空间类似，以一个由 xattr_blkaddr 指示的特定块开始，以适当对齐逐个组织
    每个共享 xattr 也可以通过如下公式直接找到         xattr offset = xattr_blkaddr * block_size + 4 * xattr_id

```
                           |-> aligned by  4 bytes
    + xattr_blkaddr blocks                     |-> aligned with 4 bytes
     _________________________________________________________________________
    |  ...   | xattr_entry |  xattr data | ... |  xattr_entry | xattr data  ...
    |________|_____________|_____________|_____|______________|_______________
```

### 目录

所有目录现在都以紧凑的磁盘格式组织。注意每个目录块被划分为索引区和名称区，以支持随机文件查找；并且所有目录项都_严格_按字母顺序记录，以支持改进的前缀二分查找算法（可参考相关源代码）
```
                  ___________________________
                 /                           |
                /              ______________|________________
               /              /              | nameoff1       | nameoffN-1
  ____________.______________._______________v________________v__________
 | dirent | dirent | ... | dirent | filename | filename | ... | filename |
 |___.0___|____1___|_____|___N-1__|____0_____|____1_____|_____|___N-1____|
      \                           ^
       \                          |                           * could have
        \                         |                             trailing '\0'
         \________________________| nameoff0
                             Directory block
```

注意，除了第一个文件名的偏移外，nameoff0 还指示了该块中目录项的总数，因为根本不需要引入另一个磁盘字段
### 基于块的文件

为了支持基于块的数据去重，自 Linux v5.15 起支持了一种新inode 数据布局：文件被拆分为等大小的数据块（chunk），inode 元数据的 `extents` 区域指示如何获取块数据：这些可以简单地4 字节块地址数组，或8 字节块索引形式（详见 erofs_fs.h 中的 struct erofs_inode_chunk_index）
顺带一提，目前所有基于块的文件都是未压缩的
### 长扩展属性名前缀

存在这样的使用场景：具有不同值的扩展属性可能只有少数几个共同前缀（例overlayfs xattrs）。预定义前缀在这种情况下对镜像大小和运行时性能都效率低下
引入xattr 名前缀特性来解决此问题。总体思路是，除了现有的预定义前缀外，xattr 项还可以引用用户指定的长 xattr 名前缀，例"trusted.overlay."
当引用一个长 xattr 名前缀时，erofs_xattr_entry.e_name_index 的最高位（bit 7）被置位，而低几位（bit 0-6）整体表示所引用长名前缀在所有长名前缀中的索引。因此，只有xattr 名前缀之外名称的尾部部分被存储erofs_xattr_entry.e_name 中；如果完整 xattr 名与xattr 名前缀完全匹配，则该尾部部分可以为空
所有长 xattr 前缀只要打包（packed）inode 有效，就逐个存储在打inode 中，否则存储在元（meta）inode 中。磁盘超级块中的 xattr_prefix_count 指示xattr 名前缀的总数，(xattr_prefix_start * 4) 指示长名前缀在打meta inode 中的起始偏移。注意，如果 xattr_prefix_count 0，则长扩展属性名前缀被禁用
每个长名前缀以如下格式存储：ALIGN({__le16 len, data}, 4)，其len 表示 data 部分的总大小。data 部分实际上由 'struct erofs_xattr_long_prefix' 表示，其base_index 表示预定xattr 名前缀的索引（例如 "trusted.overlay." 长名前缀对应 EROFS_XATTR_INDEX_TRUSTED），infix 字符串保留去掉短前缀后的字符串（例如上例中为 "overlay."）
### 数据压缩

EROFS 实现了固定大小输出压缩，它从可变大小输入生成固定大小的压缩数据块，这与其它现有的固定大小输入方案相反。使用固定大小输出压缩可以获得相对更高的压缩比，因为如今流行的数据压缩算法大多基LZ77，而这种固定大小输出方式可以从历史字典（即滑动窗口）中获益
具体来说，原始（未压缩）数据被转换为若干可变大小extent，同时被压缩进物理簇（pcluster）。为了记录每个可变大小的 extent，引入了逻辑簇（lcluster）作为压缩索引的基本单元，用于指示在该范围内是否生成了新extent（HEAD）或没有（NONHEAD）。Lcluster 现在

```
          |<-    variable-sized extent    ->|<-       VLE         ->|
        clusterofs                        clusterofs              clusterofs
          |                                 |                       |
  _________v_________________________________v_______________________v________
 ... |    .         |              |        .     |              |  .   ...
 ____|____._________|______________|________.___ _|______________|__.________
     |-> lcluster <-|-> lcluster <-|-> lcluster <-|-> lcluster <-|
          (HEAD)        (NONHEAD)       (HEAD)        (NONHEAD)    .
           .             CBLKCNT            .                    .
            .                               .                  .
             .                              .                .
       _______._____________________________.______________._________________
          ... |              |              |              | ...
       _______|______________|______________|______________|_________________
              |->      big pcluster       <-|-> pcluster <-|
```

物理簇可以看作包含压缩数据的物理压缩块容器。此前，仅支lcluster 大小KB）的 pcluster。在引入 big pcluster 特性（Linux v5.13 起可用）后，pcluster 可以lcluster 大小的倍数
对于每个 HEAD lcluster，记clusterofs 以指示新 extent 从何处开始，并使blkaddr 来定位压缩数据。对于每NONHEAD lcluster，可以使delta0 delta1 而非 blkaddr，以指示到其 HEAD lcluster 和下一HEAD lcluster 的距离。PLAIN lcluster 也是一HEAD lcluster，只是其数据未压缩。更多细节参erofs_fs.h "struct z_erofs_vle_decompressed_index" 周围的注释
如果启用big pcluster，也需要记录以 lcluster 计的 pcluster 大小。让第一NONHEAD lcluster delta0 以特殊标志存储压缩块计数，作为一个新称为 CBLKCNT NONHEAD lcluster。这很容
```
   __________________________________________________________
  | HEAD |  NONHEAD  | NONHEAD | ... | NONHEAD | HEAD | HEAD |
  |__:___|_(CBLKCNT)_|_________|_____|_________|__:___|____:_|
     |<----- a big pcluster (with CBLKCNT) ------>|<--  -->|
           a lcluster-sized pcluster (without CBLKCNT) ^
```

如果另一HEAD 跟随在某HEAD lcluster 之后，就没有空间记录 CBLKCNT，但很容易知道这pcluster 的大小同样是 1 lcluster
Linux v6.1 起，每个 pcluster 可用于多个可变大小的 extent，因此它可以用于压缩数据去重