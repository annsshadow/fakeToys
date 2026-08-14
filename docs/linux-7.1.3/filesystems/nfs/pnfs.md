## pnfs 中的引用计数


这里有几个相互关联的缓存。我们有布局（layout），一个布局可以引用多个设备（device），每个设备又可以引用多个数据服务器（data server）。每个数据服务器可以被多个设备引用。每个设备又可以被多个布局引用。为了让这一切保持清晰，我们需要进行引用计数。


## struct pnfs_layout_hdr


线上命令 LAYOUTGET 对应于 struct pnfs_layout_segment，通常以变量名 lseg 来指代。每个 nfs_inode 可以在 nfsi->layout 中持有一个指向这些布局段缓存的指针，其类型为 struct pnfs_layout_hdr。

我们为指向它的 inode 引用该头部，跨越每个引用它的未完成 RPC 调用（LAYOUTGET、LAYOUTRETURN、LAYOUTCOMMIT），并为其中包含的每个 lseg 引用它。

每个头部（当非空时）还会被放入一个与 struct nfs_client（cl_layouts）关联的链表中。放入该链表并不会增加引用计数，因为布局由使其留在链表中的 lseg 维持着。

## deviceid_cache


lseg 引用 device id，这些 id 按 nfs_client 和布局驱动类型来解析。device id 保存在一个 RCU 缓存（struct nfs4_deviceid_cache）中。该缓存本身在每个挂载（mount）期间被引用。这些条目（struct nfs4_deviceid）本身在每个引用它们的 lseg 的生命周期内被持有。

使用 RCU 是因为 deviceid 基本上是一个一次写入、多次读取（write once, read many）的数据结构。32 个桶（bucket）的 hlist 大小需要更好的理由，但鉴于每个文件系统可以有多个 deviceid，而每个 nfs_client 又可以有多个文件系统，这似乎是合理的。

哈希代码是从 nfsd 代码库复制过来的。关于哈希及该算法各种变体的讨论可以在 `这里。
<http://groups.google.com/group/comp.lang.c/browse_thread/thread/9522965e2b8d3809>`_ 找到

## 数据服务器缓存


文件驱动（file driver）设备引用数据服务器，这些数据服务器保存在一个模块级缓存中。其引用在指向它的 deviceid 的生命周期内被持有。

## lseg


lseg 维护一个与 NFS_LSEG_VALID 位对应的额外引用，该位使其留在 pnfs_layout_hdr 的链表中。当最后一个 lseg 从 pnfs_layout_hdr 的链表中移除时，会设置 NFS_LAYOUT_DESTROYED 位，阻止再加入任何新的 lseg。

## 布局驱动（layout drivers）


PNFS 使用了所谓的布局驱动。STD 定义了 4 种基本布局类型："files"、"objects"、"blocks" 和 "flexfiles"。对于每种类型，都有一个布局驱动，带有一个由 nfs-client 的 pnfs-core 调用的通用函数向量表，用于实现不同的布局类型。

Files 布局驱动代码位于：fs/nfs/filelayout/.. 目录
Blocks 布局驱动代码位于：fs/nfs/blocklayout/.. 目录
Flexfiles 布局驱动代码位于：fs/nfs/flexfilelayout/.. 目录

## blocks 布局设置


TODO：记录 blocks 布局驱动的设置需求
