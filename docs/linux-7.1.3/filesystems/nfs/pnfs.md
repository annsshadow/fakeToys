## pnfs 中的引用计数


这里有几个相互关联的缓存。我们有布局（layout），一个布局可以引用多个设备（device），每个设备又可以引用多个数据服务器（data server）。每个数据服务器可以被多个设备引用。每个设备又可以被多个布局引用。为了让这一切保持清晰，我们需要进行引用计数

## struct pnfs_layout_hdr


线上命令 LAYOUTGET 对应struct pnfs_layout_segment，通常以变量名 lseg 来指代。每nfs_inode 可以nfsi->layout 中持有一个指向这些布局段缓存的指针，其类型struct pnfs_layout_hdr
我们为指向它inode 引用该头部，跨越每个引用它的未完RPC 调用（LAYOUTGET、LAYOUTRETURN、LAYOUTCOMMIT），并为其中包含的每lseg 引用它
每个头部（当非空时）还会被放入一个与 struct nfs_client（cl_layouts）关联的链表中。放入该链表并不会增加引用计数，因为布局由使其留在链表中lseg 维持着
## deviceid_cache


lseg 引用 device id，这id nfs_client 和布局驱动类型来解析。device id 保存在一RCU 缓存（struct nfs4_deviceid_cache）中。该缓存本身在每个挂载（mount）期间被引用。这些条目（struct nfs4_deviceid）本身在每个引用它们lseg 的生命周期内被持有
使用 RCU 是因deviceid 基本上是一个一次写入、多次读取（write once, read many）的数据结构2 个桶（bucket）的 hlist 大小需要更好的理由，但鉴于每个文件系统可以有多deviceid，而每nfs_client 又可以有多个文件系统，这似乎是合理的
哈希代码是从 nfsd 代码库复制过来的。关于哈希及该算法各种变体的讨论可以`这里<http://groups.google.com/group/comp.lang.c/browse_thread/thread/9522965e2b8d3809>`_ 找到

## 鏁版嵁鏈嶅姟鍣ㄧ紦瀛。

文件驱动（file driver）设备引用数据服务器，这些数据服务器保存在一个模块级缓存中。其引用在指向它deviceid 的生命周期内被持有
## lseg


lseg 维护一个与 NFS_LSEG_VALID 位对应的额外引用，该位使其留pnfs_layout_hdr 的链表中。当最后一lseg pnfs_layout_hdr 的链表中移除时，会设NFS_LAYOUT_DESTROYED 位，阻止再加入任何新lseg
## 布局驱动（layout drivers

PNFS 使用了所谓的布局驱动。STD 定义4 种基本布局类型files"objects"blocks" "flexfiles"。对于每种类型，都有一个布局驱动，带有一个由 nfs-client pnfs-core 调用的通用函数向量表，用于实现不同的布局类型
Files 布局驱动代码位于：fs/nfs/filelayout/.. 目录
Blocks 布局驱动代码位于：fs/nfs/blocklayout/.. 目录
Flexfiles 布局驱动代码位于：fs/nfs/flexfilelayout/.. 目录

## blocks 布局设置


TODO：记blocks 布局驱动的设置需