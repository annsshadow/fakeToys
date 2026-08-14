
## 面向闪存的文件系统（Flash-Friendly File System，F2FS）


## 概述


基于 NAND 闪存的存储设备，如 SSD、eMMC 与 SD 卡，已被装备在从移动设备到服务器系统的各类系统上。由于它们已知具有不同于传统机械磁盘的特性，作为存储设备上层结构的文件系统，应当从设计层面起就适应这些变化。

F2FS 是一个利用基于 NAND 闪存的存储设备的文件系统，它基于日志结构文件系统（Log-structured File System，LFS）。其设计重点在于解决 LFS 中的根本问题，即游走树（wandering tree）的滚雪球效应与高昂的清理开销。

由于基于 NAND 闪存的存储设备会因其内部几何结构或闪存管理方案（即 FTL）的不同而表现出不同特性，F2FS 及其工具支持各种参数，不仅用于配置磁盘上的布局，也用于选择分配与清理算法。

以下 git 树提供了文件系统格式化工具（mkfs.f2fs）、一致性检查工具（fsck.f2fs）以及调试工具（dump.f2fs）：

- git://git.kernel.org/pub/scm/linux/kernel/git/jaegeuk/f2fs-tools.git

提交补丁请使用以下邮件列表：

- linux-f2fs-devel@lists.sourceforge.net

报告 bug 请使用以下 f2fs bug 跟踪链接：

- https://bugzilla.kernel.org/enter_bug.cgi?product=File%20System&component=f2fs

## 背景与设计问题


### 日志结构文件系统（LFS）


“日志结构文件系统以类日志的结构将所有修改顺序写入磁盘，从而同时加快文件写入与崩溃恢复。日志是磁盘上唯一的结构；它包含索引信息，以便文件能够高效地从日志中读回。为了维持磁盘上的大块空闲区域以加快写入，我们将日志划分为段，并使用段清理器（segment cleaner）将重度碎片化的段中的有效信息压缩。” 引自 Rosenblum, M. 与 Ousterhout, J. K., 1992，《日志结构文件系统的设计与实现》，ACM Trans. Computer Systems 10, 1, 26–52。

### 游走树问题


在 LFS 中，当文件数据被更新并写入日志末尾时，其直接指针块会因位置改变而更新。接着间接指针块也会因直接指针块的更新而更新。依此类推，上层索引结构如 inode、inode 映射与检查点块也会递归地更新。这个问题被称为游走树问题 [^1^]，为了提升性能，应尽可能消除或放松这种更新传播。

[^1^] Bityutskiy, A. 2005. JFFS3 design issues. http://www.linux-mtd.infradead.org/

### 清理开销


由于 LFS 基于异地写（out-of-place writes），它会产生大量散布在整个存储中的废弃块。为了提供新的空闲日志空间，它需要无缝地回收这些废弃块。这项工作被称为清理过程。

该过程由如下三个操作组成。

1. 通过引用段使用表选择一个受害者段。
2. 它加载受害者段中所有数据由段摘要块标识出的父索引结构。
3. 它检查数据与父索引结构之间的交叉引用。
4. 它有选择地移动有效数据。
这一清理工作可能导致意外的长延迟，因此最重要的目标是向用户隐藏这些延迟。当然，它还应减少需要移动的有效数据量，并快速地移动它们。

## 关键特性


### 闪存感知


- 扩大随机写入区域以获得更好的性能，同时提供较高的空间局部性
- 尽最大努力将文件系统数据结构对齐到 FTL 中的操作单元

### 游走树问题


- 使用一个术语“node（节点）”来表示 inode 以及各种指针块
- 引入包含所有“node”块位置的节点地址表（Node Address Table，NAT）；这将切断更新传播。

### 清理开销


- 支持后台清理过程
- 支持贪心（greedy）与成本收益（cost-benefit）算法用于受害者选择策略
- 支持多磁头日志用于静态/动态冷热数据分离
- 引入自适应日志（adaptive logging）以实现高效块分配

## 挂载选项
======================== ============================================================
background_gc=%s	 开启/关闭后台触发（即当 I/O 子系统空闲时）的清理操作，也就是垃圾回收（garbage collection）。若 background_gc=on，则开启垃圾回收；若 background_gc=off，则关闭垃圾回收。若 background_gc=sync，则开启在后台运行的同步垃圾回收。该选项的默认值为 on，因此默认开启垃圾回收。
gc_merge		 当 background_gc 开启时，可启用此选项，让后台 GC 线程处理前台 GC 请求，从而消除当 GC 由 I/O 与 CPU 资源受限的进程触发时，缓慢的前台 GC 操作导致的卡顿问题。
nogc_merge		 禁用 GC 合并特性。
disable_roll_forward	 禁用前滚（roll-forward）恢复流程。
norecovery		 禁用前滚恢复流程，以只读方式挂载（即 -o ro,disable_roll_forward）。
discard/nodiscard	 在 f2fs 中启用/禁用实时丢弃（discard）；若启用 discard，f2fs 会在清理一个段时发出 discard/TRIM 命令。
heap/no_heap		 已废弃。
nouser_xattr		 禁用扩展用户属性（Extended User Attributes）。注意：若选中了 CONFIG_F2FS_FS_XATTR，则 xattr 默认启用。
noacl			 禁用 POSIX 访问控制列表（Access Control List）。注意：若选中了 CONFIG_F2FS_FS_POSIX_ACL，则 acl 默认启用。
active_logs=%u		 支持配置活动日志的数量。在当前设计中，f2fs 仅支持 2、4 和 6 条日志。默认数量为 6。
disable_ext_identify	 禁用由 mkfs 配置的扩展名列表，这样 f2fs 就不会感知到诸如媒体文件之类的冷文件。
inline_xattr		 启用内联 xattr（inline xattrs）特性。
noinline_xattr		 禁用内联 xattr 特性。
inline_xattr_size=%u	 支持配置内联 xattr 大小，它依赖于灵活内联 xattr 特性。
inline_data		 启用内联数据（inline data）特性：新创建的较小（<~3.4k）文件可直接写入 inode 块。
inline_dentry		 启用内联目录（inline dir）特性：新建目录项中的数据可写入 inode 块。用于存储内联目录项的 inode 块空间上限约为 3.4k。
noinline_dentry		 禁用内联 dentry 特性。
flush_merge		 尽可能合并并发的 cache_flush 命令，以消除冗余命令的发出。如果底层设备处理 cache_flush 命令相对较慢，建议启用此选项。
nobarrier		 当底层存储保证其缓存数据应写入非易失性区域时，可使用此选项。若设置此选项，则不会发出 cache_flush 命令，但 f2fs 仍保证所有数据写入的写顺序。
barrier			 若设置此选项，则允许发出 cache_flush 命令。
fastboot		 当系统希望尽可能减少挂载时间、即使牺牲正常性能时，使用此选项。
extent_cache		 启用基于 rb-tree 的 extent 缓存，它可以为每个 inode 缓存尽可能多的、在连续逻辑地址与物理地址之间映射的 extent，从而提高缓存命中率。默认开启。
noextent_cache		 显式禁用基于 rb-tree 的 extent 缓存，参见上面的 extent_cache 挂载选项。
noinline_data		 禁用内联数据特性；内联数据特性默认是启用的。
data_flush		 在检查点之前启用数据刷新，以持久化普通文件与符号链接的数据。
reserve_root=%d	 支持配置保留空间，供具有指定 uid 或 gid 的特权用户进行分配，单位：4KB，默认上限为用户块的 12.5%。
reserve_node=%d	 支持配置保留节点，供具有指定 uid 或 gid 的特权用户进行分配，默认上限为所有节点的 12.5%。
resuid=%d		 可以使用保留块与节点的用户 ID。
resgid=%d		 可以使用保留块与节点的组 ID。
fault_injection=%d	 以指定的注入速率，在所有支持的类型中启用故障注入。
fault_type=%d		 支持配置故障注入类型，应与 fault_injection 选项一起启用；故障类型值如下所示，支持单一或组合类型。
			 .. code-block:: none
			     ===========================      ==========
			     Type_Name                        Type_Value
			     ===========================      ==========
			     FAULT_KMALLOC                    0x00000001
			     FAULT_KVMALLOC                   0x00000002
			     FAULT_PAGE_ALLOC                 0x00000004
			     FAULT_PAGE_GET                   0x00000008
			     FAULT_ALLOC_BIO                  0x00000010 (obsolete)
			     FAULT_ALLOC_NID                  0x00000020
			     FAULT_ORPHAN                     0x00000040
			     FAULT_BLOCK                       0x00000080
			     FAULT_DIR_DEPTH                  0x00000100
			     FAULT_EVICT_INODE                0x00000200
			     FAULT_TRUNCATE                   0x00000400
			     FAULT_READ_IO                    0x00000800
			     FAULT_CHECKPOINT                 0x00001000
			     FAULT_DISCARD                    0x00002000 (obsolete)
			     FAULT_WRITE_IO                   0x00004000
			     FAULT_SLAB_ALLOC                 0x00008000
			     FAULT_DQUOT_INIT                 0x00010000
			     FAULT_LOCK_OP                    0x00020000
			     FAULT_BLKADDR_VALIDITY           0x00040000
			     FAULT_BLKADDR_CONSISTENCE        0x00080000
			     FAULT_NO_SEGMENT                 0x00100000
			     FAULT_INCONSISTENT_FOOTER        0x00200000
			     FAULT_ATOMIC_TIMEOUT             0x00400000 (1000ms)
			     FAULT_VMALLOC                    0x00800000
			     FAULT_LOCK_TIMEOUT               0x01000000 (1000ms)
			     FAULT_SKIP_WRITE                 0x02000000
			     ===========================      ==========
mode=%s			 控制块分配模式，支持“adaptive”与“lfs”。在“lfs”模式下，不应有朝向主区域的随机写入。
			 “fragment:segment”与“fragment:block”是新加入的。这些是供实验模拟文件系统碎片化/GC 后情形本身的开发者选项。开发者使用这些模式来更好地理解文件系统碎片化/GC 后的状况，并最终获得更好的处理思路。
			 在“fragment:segment”模式下，f2fs 会在随机位置分配一个新段，借此可以模拟 GC 后的状况。
			 在“fragment:block”模式下，我们可以借助“max_fragment_chunk”与“max_fragment_hole” sysfs 节点打散块分配。我们对 chunk 与 hole 的大小都加入了一些随机性，使其接近真实的 I/O 模式。因此在此模式下，f2fs 会在一个 chunk 中分配 1..<max_fragment_chunk> 个块，并轮流转而制造长度为 1..<max_fragment_hole> 的空洞。这样，新分配的块将散布在整个分区中。注意，“fragment:block”会隐式启用“fragment:segment”选项以获得更多随机性。
			 请将这些选项用于你的实验，并且我们强烈建议在使用这些选项后重新格式化文件系统。
usrquota		 启用普通用户磁盘配额记账。
grpquota		 启用普通组磁盘配额记账。
prjquota		 启用普通项目配额记账。
usrjquota=<file>	 在挂载期间指定特定文件与类型，以便配额信息能在恢复流程中正确更新；
grpjquota=<file>	 <quota file> 必须位于根目录；
prjjquota=<file>	 jqfmt=<quota type>: <quota type>: [vfsold,vfsv0,vfsv1]。
usrjquota=		 关闭用户日志配额。
grpjquota=		 关闭组日志配额。
prjjquota=		 关闭项目日志配额。
quota			 启用普通用户磁盘配额记账。
noquota			 禁用所有普通磁盘配额选项。
alloc_mode=%s		 调整块分配策略，支持“reuse”与“default”。
fsync_mode=%s		 控制 fsync 的策略。目前支持“posix”、“strict”与“nobarrier”。在默认的“posix”模式下，fsync 会遵循 POSIX 语义，并进行轻量操作以提升文件系统性能。在“strict”模式下，fsync 会较重，行为向 xfs、ext4 与 btrfs 看齐，xfstest generic/342 会通过，但性能会退化。“nobarrier”基于“posix”，但不像“nobarrier”挂载选项那样为非原子文件发出刷新命令。
test_dummy_encryption
test_dummy_encryption=%s
			 启用虚拟加密（dummy encryption），提供伪造的 fscrypt 上下文。伪造的 fscrypt 上下文供 xfstests 使用。参数可以是“v1”或“v2”，以选择对应的 fscrypt 策略版本。
checkpoint=%s[:%u[%]]	 设为“disable”以关闭检查点；设为“enable”以重新启用检查点。默认启用。在禁用期间，任何卸载或意外关机都会使文件系统内容表现为挂载该选项时的样子。
			 在以 checkpoint=disable 挂载时，文件系统必须运行垃圾回收以确保所有可用空间都能被使用。如果这耗时过长，挂载可能返回 EAGAIN。你可以选择性地附加一个值，表示你愿意临时放弃多少磁盘空间以避免额外的垃圾回收。该值可给定为块数或百分比。例如，以 checkpoint=disable:100% 挂载总会成功，但可能隐藏多达全部剩余空闲空间。实际不可用的空间可在 /sys/fs/f2fs/<disk>/unusable 查看。一旦 checkpoint=enable，该空间即被回收。
checkpoint_merge	 当检查点启用时，此选项可用于创建一个内核守护进程，并使其尽可能合并并发的检查点请求，以消除冗余的检查点发出。此外，当检查点在具有较低 I/O 预算与 CPU 份额的 cgroup 的进程上下文中完成时，我们可以消除缓慢检查点操作导致的卡顿。为了让它表现更好，我们将该内核守护进程的默认 I/O 优先级设为“3”，使其优先级高于其他内核线程。这与为 ext4 文件系统的 jbd2 日志线程赋予 I/O 优先级的方式相同。
nocheckpoint_merge	 禁用检查点合并特性。
compress_algorithm=%s	 控制压缩算法，目前 f2fs 支持“lzo”、“lz4”、“zstd”与“lzo-rle”算法。
compress_algorithm=%s:%d 控制压缩算法及其压缩级别，目前仅
```

				 =========      ===========
				 algorithm      level range
				 =========      ===========
				 lz4            3 - 16
				 zstd           1 - 22
				 =========      ===========

```
compress_log_size=%u	 支持配置压缩簇大小。大小为 4KB * (1 << %u)。默认与最小大小均为 16KB。
compress_extension=%s	 支持添加指定扩展名，使 f2fs 能在对应的文件上启用压缩。例如，如果所有带“.ext”的文件压缩率很高，我们可以将“.ext”加入压缩扩展名列表，并默认对这些文件启用压缩，而无需通过 ioctl 启用。对于其他文件，我们仍可通过 ioctl 启用压缩。注意，有一个保留的特殊扩展名“*”，可将其设置以对所有文件启用压缩。
nocompress_extension=%s	 支持添加指定扩展名，使 f2fs 能在对应的文件上禁用压缩，恰恰与压缩扩展名相反。如果你确切知道哪些文件无法压缩，可以使用此项。同一个扩展名不能同时出现在 compress 与 nocompress 扩展名中。如果 compress 扩展名指定了所有文件，则 nocompress 扩展名指定的类型将被视为特例而不被压缩。不允许在 nocompress 扩展名中使用“*”来指定所有文件。添加 nocompress_extension 后，优先级应为：dir_flag < comp_extension,nocompress_extension < comp_file_flag,no_comp_file_flag。详见压缩章节。
compress_chksum		 支持校验压缩簇中原始数据的校验和（chksum）。
compress_mode=%s	 控制文件压缩模式。支持“fs”与“user”模式。在“fs”模式（默认）下，f2fs 会对启用压缩的文件进行自动压缩。在“user”模式下，f2fs 禁用自动压缩，并将选择目标文件与时机交由用户决定。用户可以使用 ioctl 对启用压缩的文件进行手动压缩/解压缩。
compress_cache		 支持使用文件系统管理的 inode 的地址空间来缓存压缩块，以提高随机读的缓存命中率。
inlinecrypt		 在可能时，使用 blk-crypto 框架而非文件系统层加密，对加密文件的内容进行加密/解密。这允许使用内联加密硬件。磁盘上的格式不受影响。更多细节见 Documentation/block/inline-encryption.rst。
atgc			 启用基于年龄阈值（age-threshold）的垃圾回收，它在后台 GC 上提供高有效性与高效率。
discard_unit=%s	 控制丢弃单元，参数可以是“block”、“segment”与“section”，发出的 discard 命令的偏移/大小将对齐到该单元。默认设置为“discard_unit=block”，从而启用小块丢弃功能。对于 blkzoned 设备，默认会设置为“discard_unit=section”，这有助于大型 SMR 或 ZNS 设备通过摆脱支持小块丢弃的 fs 元数据来降低内存开销。
memory=%s		 控制内存模式。支持“normal”与“low”模式。“low”模式是为支持低内存设备而引入的。由于低内存设备的特性，在此模式下 f2fs 有时会以牺牲性能为代价来节省内存。“normal”模式是默认模式，与之前相同。
age_extent_cache	 启用基于 rb-tree 的年龄 extent 缓存。它记录每个 inode 的 extent 数据块更新频率，以便为数据块分配提供更好的温度提示。
errors=%s		 指定 f2fs 在严重错误时的行为。支持的模式有：“panic”、“continue”与“remount-ro”，分别意为立即触发 panic、不做任何处理继续运行、以及以只读模式重新挂载分区。默认使用“continue”模式。

			 .. code-block:: none

			     ====================== =============== =============== ========
			     mode                   continue        remount-ro      panic
			     ====================== =============== =============== ========
			     access ops             normal          normal          N/A
			     syscall errors         -EIO            -EROFS          N/A
			     mount option           rw              ro              N/A
			     pending dir write      keep            keep            N/A
			     pending non-dir write  drop            keep            N/A
			     pending node write     drop            keep            N/A
			     pending meta write     keep            keep            N/A
			     ====================== =============== =============== ========
nat_bits		 启用 nat_bits 特性以增强对满/空 nat 块的访问，默认禁用。
lookup_mode=%s	 控制对大小写折叠（casefolded）目录的目录查找行为。该选项对未启用 casefold 特性的目录无效。

			 .. code-block:: none
			     ================== ========================================
			     Value              Description
			     ================== ========================================
			     perf               (Default) Enforces a hash-only lookup.
					        The linear search fallback is always
					        disabled, ignoring the on-disk flag.
			     compat             Enables the linear search fallback for
					        compatibility with directory entries
					        created by older kernel that used a
					        different case-folding algorithm.
					        This mode ignores the on-disk flag.
			     auto               F2FS determines the mode based on the
					        on-disk `SB_ENC_NO_COMPAT_FALLBACK_FL`
					        flag.
			     ================== ========================================
======================== ============================================================

## Debugfs 条目


/sys/kernel/debug/f2fs/ 包含有关所有以 f2fs 挂载的分区的信息。每个文件展示完整的 f2fs 信息。

/sys/kernel/debug/f2fs/status 包含：

 - f2fs 当前管理的主要文件系统信息
 - 关于整个段的 SIT 平均信息
 - f2fs 当前消耗的内存占用

## Sysfs 条目


有关已挂载 f2fs 文件系统的信息可在 /sys/fs/f2fs 中找到。每个已挂载的文件系统都会在 /sys/fs/f2fs 下根据其设备名拥有一个目录（例如 /sys/fs/f2fs/sda）。每个每设备目录下的文件如下表所示。

/sys/fs/f2fs/<devname> 下的文件
（另见 Documentation/ABI/testing/sysfs-fs-f2fs）

## 用法


1. 下载用户态工具并编译它们。

2. 如果 f2fs 已被静态编译进内核，则跳过此步。
```

	# insmod f2fs.ko

```
```

	# mkdir /mnt/f2fs

```
```

	# mkfs.f2fs -l label /dev/block_device
	# mount -t f2fs /dev/block_device /mnt/f2fs

```
### mkfs.f2fs


mkfs.f2fs 用于将分区格式化为 f2fs 文件系统，它会构建基本的磁盘布局。

快速选项包括：

===============    ===========================================================
`-l [label]`     指定卷标，最多 512 个 unicode 名称。
`-a [0 or 1]`    为每个区域的起始位置拆分，用于基于堆的分配。

                   默认设为 1，即执行此操作。
`-o [int]`       设置超额配置（overprovision）比例，以卷大小的百分比计。

                   默认设为 5。
`-s [int]`       设置每个 section 的段数量。

                   默认设为 1。
`-z [int]`       设置每个 zone 的 section 数量。

                   默认设为 1。
`-e [str]`       设置基本扩展名列表。例如 "mp3,gif,mov"。
`-t [0 or 1]`    是否禁用 discard 命令。

                   默认设为 1，即执行 discard。
===============    ===========================================================

注意：请参考 mkfs.f2fs(8) 的手册页获取完整选项列表。

### fsck.f2fs


fsck.f2fs 是用于检查 f2fs 格式化分区一致性的工具，它会检查文件系统元数据与用户数据是否被正确交叉引用。注意，该工具的早期版本不会修复任何不一致。
```

  -d debug level [default:0]

```
注意：请参考 fsck.f2fs(8) 的手册页获取完整选项列表。

### dump.f2fs


dump.f2fs 显示特定 inode 的信息，并将 SSA 与 SIT 转储到文件。每个文件分别为 dump_ssa 与 dump_sit。

dump.f2fs 用于调试 f2fs 文件系统的磁盘数据结构。它显示由给定 inode 号识别的磁盘 inode 信息，并能够将所有的 SSA 与 SIT 条目转储到预定义文件 ./dump_ssa 与 ./dump_sit 中。
```

  -d debug level [default:0]
  -i inode no (hex)
  -s [SIT dump segno from #1~#2 (decimal), for all 0~-1]
  -a [SSA dump segno from #1~#2 (decimal), for all 0~-1]

```
```

    # dump.f2fs -i [ino] /dev/sdx
    # dump.f2fs -s 0~-1 /dev/sdx (SIT dump)
    # dump.f2fs -a 0~-1 /dev/sdx (SSA dump)

```
注意：请参考 dump.f2fs(8) 的手册页获取完整选项列表。

### sload.f2fs


sload.f2fs 提供了一种在现有磁盘镜像中插入文件与目录的方式。该工具在基于已编译文件构建 f2fs 镜像时很有用。

注意：请参考 sload.f2fs(8) 的手册页获取完整选项列表。

### resize.f2fs


resize.f2fs 让用户可以调整 f2fs 格式化磁盘镜像的大小，同时保留镜像中存储的所有文件与目录。

注意：请参考 resize.f2fs(8) 的手册页获取完整选项列表。

### defrag.f2fs


defrag.f2fs 可用于对散布写入的数据以及跨磁盘的文件系统元数据进行碎片整理。通过提供更多的连续空闲空间，这可以提升写入速度。

注意：请参考 defrag.f2fs(8) 的手册页获取完整选项列表。

### f2fs_io


f2fs_io 是一个简单工具，用于发出各种文件系统 API 以及 f2fs 特定的 API，对 QA 测试非常有用。

注意：请参考 f2fs_io(8) 的手册页获取完整选项列表。

## 设计


### 磁盘布局


F2FS 将整个卷划分为若干个段（segment），每个段固定为 2MB 大小。一个 section 由连续的段组成，一个 zone 由一组 section 组成。默认情况下，section 与 zone 的大小都被设为与一个段大小相同，但用户可以通过 mkfs 轻松修改这些大小。

F2FS 将整个卷划分为六个区域，除超级块（superblock）外所有区域
```

                                            align with the zone size <-|
                 |-> align with the segment size
     _________________________________________________________________________
    |            |            |   Segment   |    Node     |   Segment  |      |
    | Superblock | Checkpoint |    Info.    |   Address   |   Summary  | Main |
    |    (SB)    |   (CP)     | Table (SIT) | Table (NAT) | Area (SSA) |      |
    |____________|_____2______|______N______|______N______|______N_____|__N___|
                                                                       .      .
                                                             .                .
                                                 .                            .
                                    ._________________________________________.
                                    |_Segment_|_..._|_Segment_|_..._|_Segment_|
                                    .           .
                                    ._________._________
                                    |_section_|__...__|_
                                    .            .
		                    .________.
	                            |__zone__|

```
- Superblock (SB)
   它位于分区的开头，存在两份副本以避免文件系统崩溃。它包含基本的分区信息以及 f2fs 的一些默认参数。

- Checkpoint (CP)
   它包含文件系统信息、有效 NAT/SIT 集合的位图、孤儿（orphan）inode 列表，以及当前活动段的摘要条目。

- Segment Information Table (SIT)
   它包含段信息，例如有效块计数，以及所有块有效性的位图。

- Node Address Table (NAT)
   它由存储于主区域中所有节点块的块地址表组成。

- Segment Summary Area (SSA)
   它包含摘要条目，这些条目保存了存储于主区域中所有数据与节点块的所有者信息。

- Main Area
   它包含文件与目录数据，包括它们的索引。

为了避免文件系统与基于闪存的存储之间出现未对齐，F2FS 将 CP 的起始块地址与段大小对齐。同时，它通过在 SSA 区域中保留一些段，将主区域的起始块地址与 zone 大小对齐。

更多技术细节请参考以下调查。
https://wiki.linaro.org/WorkingGroups/Kernel/Projects/FlashCardSurvey

### 文件系统元数据结构


F2FS 采用检查点（checkpointing）方案来维护文件系统一致性。在挂载时，F2FS 首先尝试通过扫描 CP 区域来找到最后一个有效的检查点数据。为了减少扫描时间，F2FS 只使用两份 CP 副本。其中一份始终指向最后一个有效数据，这被称为影子副本（shadow copy）机制。除了 CP 之外，NAT 与 SIT 也采用了影子副本机制。

为了保证文件系统一致性，每个 CP 指向哪些 NAT 与 SIT 副本是
```

  +--------+----------+---------+
  |   CP   |    SIT   |   NAT   |
  +--------+----------+---------+
  .         .          .          .
  .            .              .              .
  .               .                 .                 .
  +-------+-------+--------+--------+--------+--------+
  | CP #0 | CP #1 | SIT #0 | SIT #1 | NAT #0 | NAT #1 |
  +-------+-------+--------+--------+--------+--------+
     |             ^                          ^
     |             |                          |
     `----------------------------------------'

```
### 索引结构


管理数据位置的关键数据结构是“node（节点）”。与传统文件结构类似，F2FS 有三种类型的节点：inode、直接节点（direct node）、间接节点（indirect node）。F2FS 为 inode 块分配 4KB，其中包含 923 个数据块索引、两个直接节点指针、两个间接节点指针，以及一个双间接节点指针，如下所示。一个直接节点块包含 1018 个数据块，一个间接节点块也包含 1018 个节点块。因此，
```

  4KB * (923 + 2 * 1018 + 2 * 1018 * 1018 + 1018 * 1018 * 1018) := 3.94TB.

   Inode block (4KB)
     |- data (923)
     |- direct node (2)
     |          `- data (1018)
     |- indirect node (2)
     |            `- direct node (1018)
     |                       `- data (1018)
     `- double indirect node (1)
                         `- indirect node (1018)
			              `- direct node (1018)
	                                         `- data (1018)

```
注意，所有节点块都由 NAT 映射，这意味着每个节点的位置都通过 NAT 表进行转换。考虑到游走树问题，F2FS 能够切断由叶子数据写入引起的节点更新传播。

### 目录结构


一个目录项（directory entry）占用 11 字节，由以下属性组成。

- hash		文件名的哈希值
- ino		inode 号
- len		文件名的长度
- type		文件类型，如目录、符号链接等

一个 dentry 块由 214 个 dentry 槽与文件名组成。其中使用一个位图来表示每个 dentry 是否有效。一个 dentry 块占用 4KB，其组成如下。
```

  Dentry Block(4 K) = bitmap (27 bytes) + reserved (3 bytes) +
	              dentries(11 * 214 bytes) + file name (8 * 214 bytes)

                         [Bucket]
             +--------------------------------+
             |dentry block 1 | dentry block 2 |
             +--------------------------------+
             .               .
       .                             .
  .       [Dentry Block Structure: 4KB]       .
  +--------+----------+----------+------------+
  | bitmap | reserved | dentries | file names |
  +--------+----------+----------+------------+
  [Dentry Block: 4KB] .   .
		 .               .
            .                          .
            +------+------+-----+------+
            | hash | ino  | len | type |
            +------+------+-----+------+
            [Dentry Structure: 11 bytes]

```
F2FS 为目录结构实现了多级哈希表。每一级都有一个具有专门数量哈希桶（hash bucket）的哈希表，如下所示。注意，“A(2B)”表示一个桶包含 2 个数据块。
```

    ----------------------
    A : bucket
    B : block
    N : MAX_DIR_HASH_DEPTH
    ----------------------

    level #0   | A(2B)
	    |
    level #1   | A(2B) - A(2B)
	    |
    level #2   | A(2B) - A(2B) - A(2B) - A(2B)
	.     |   .       .       .       .
    level #N/2 | A(2B) - A(2B) - A(2B) - A(2B) - A(2B) - ... - A(2B)
	.     |   .       .       .       .
    level #N   | A(4B) - A(4B) - A(4B) - A(4B) - A(4B) - ... - A(4B)

```
```

                            ,- 2, if n < MAX_DIR_HASH_DEPTH / 2,
  # of blocks in level #n = |
                            `- 4, Otherwise

                             ,- 2^(n + dir_level),
			     |        if n + dir_level < MAX_DIR_HASH_DEPTH / 2,
  # of buckets in level #n = |
                             `- 2^((MAX_DIR_HASH_DEPTH / 2) - 1),
			              Otherwise

```
当 F2FS 在目录中查找文件名时，首先计算文件名的哈希值。然后，F2FS 扫描级别 #0 的哈希表，以查找由文件名及其 inode 号组成的 dentry。如果未找到，F2FS 会扫描级别 #1 的下一个哈希表。依此类推，F2FS 从 1 到 N 逐级递增地扫描哈希表。在每一级中，F2FS 只需扫描由下式确定的一个桶，其复杂度为 O(log(文件数))
```

  bucket number to scan in level #n = (hash value) % (# of buckets in level #n)

```
在创建文件时，F2FS 会查找覆盖该文件名的连续空槽。F2FS 以与查找操作相同的方式，从 1 到 N 在所有级别的哈希表中搜索空槽。
```

       --------------> Dir <--------------
       |                                 |
    child                             child

    child - child                     [hole] - child

    child - child - child             [hole] - [hole] - child

   Case 1:                           Case 2:
   Number of children = 6,           Number of children = 3,
   File size = 7                     File size = 7

```
### 默认块分配


在运行时，F2FS 在“Main”区域内管理六个活动日志：热/温/冷节点（Hot/Warm/Cold node）与热/温/冷数据（Hot/Warm/Cold data）。

- Hot node	包含目录的直接节点块。
- Warm node	包含除热节点块之外的直接节点块。
- Cold node	包含间接节点块。
- Hot data	包含 dentry 块。
- Warm data	包含除热数据与冷数据块之外的数据块。
- Cold data	包含多媒体数据或迁移的数据块。

LFS 有两种空闲空间管理方案：线程化日志（threaded log）与复制-压缩（copy-and-compaction）。被称为清理（cleaning）的复制-压缩方案非常适合顺序写入性能非常好的设备，因为始终有空闲段可用于写入新数据。然而，在高利用率下它会受到清理开销的困扰。相反，线程化日志方案会受随机写入之苦，但不需要清理过程。F2FS 采用混合方案：默认采用复制-压缩方案，但会根据文件系统状态动态切换到线程化日志方案。

为了使 F2FS 与底层基于闪存的存储对齐，F2FS 以 section 为单位分配段。F2FS 期望 section 大小与 FTL 中垃圾回收的单位大小相同。此外，关于 FTL 中的映射粒度，F2FS 尽可能从不同 zone 分配活动日志的每个 section，因为 FTL 可以按其映射粒度将活动日志中的数据写入一个分配单元。

### 清理过程


F2FS 既按需清理，也在后台清理。按需清理在没有足够的空闲段来服务 VFS 调用时触发。后台清理器由一个内核线程运行，并在系统空闲时触发清理工作。

F2FS 支持两种受害者选择策略：贪心（greedy）与成本收益（cost-benefit）算法。在贪心算法中，F2FS 选择有效块数量最少的受害者段。在成本收益算法中，F2FS 根据段年龄与有效块数量选择受害者段，以解决贪心算法中的日志块抖动（log block thrashing）问题。F2FS 对按需清理器采用贪心算法，而后台清理器采用成本收益算法。

为了识别受害者段中的数据是否有效，F2FS 管理一个位图。每一位代表一个块的有效性，该位图由覆盖主区域所有块的位流组成。

### 写提示（Write-hint）策略


F2FS 始终按照以下策略设置写提示（whint）。

===================== ======================== ===================
User                  F2FS                     Block
===================== ======================== ===================
N/A                   META                     WRITE_LIFE_NONE|REQ_META
N/A                   HOT_NODE                 WRITE_LIFE_NONE
N/A                   WARM_NODE                WRITE_LIFE_MEDIUM
N/A                   COLD_NODE                WRITE_LIFE_LONG
ioctl(COLD)           COLD_DATA                WRITE_LIFE_EXTREME
extension list        "                        "

### -- buffered io

N/A                   COLD_DATA                WRITE_LIFE_EXTREME
N/A                   HOT_DATA                 WRITE_LIFE_SHORT
N/A                   WARM_DATA                WRITE_LIFE_NOT_SET

### -- direct io
WRITE_LIFE_EXTREME    COLD_DATA                WRITE_LIFE_EXTREME
WRITE_LIFE_SHORT      HOT_DATA                 WRITE_LIFE_SHORT
WRITE_LIFE_NOT_SET    WARM_DATA                WRITE_LIFE_NOT_SET
WRITE_LIFE_NONE       "                        WRITE_LIFE_NONE
WRITE_LIFE_MEDIUM     "                        WRITE_LIFE_MEDIUM
WRITE_LIFE_LONG       "                        WRITE_LIFE_LONG
===================== ======================== ===================

### Fallocate(2) 策略


默认策略遵循以下 POSIX 规则。

分配磁盘空间
     fallocate() 的默认操作（即 mode 为零）在 offset 与 len 指定的范围内分配磁盘空间。如果 offset+len 大于文件大小，则文件大小（由 stat(2) 报告）会被改变。在调用前 offset 与 len 指定范围内原本不含数据的任何子区域，将被初始化为零。这一默认行为非常类似于 posix_fallocate(3) 库函数的行为，并被设计为最优实现该函数的方法。

然而，一旦 F2FS 在 fallocate(fd, DEFAULT_MODE) 之前收到 ioctl(fd, F2FS_IOC_SET_PIN_FILE)，它就会分配具有零或随机数据的磁盘块地址，这对于以下场景很有用：

 1. create(fd)
 2. ioctl(fd, F2FS_IOC_SET_PIN_FILE)
 3. fallocate(fd, 0, 0, size)
 4. address = fibmap(fd, offset)
 5. open(blkdev)
 6. write(blkdev, address)

### 压缩实现


- 新术语“cluster（簇）”被定义为压缩的基本单位，文件可以在逻辑上划分为多个簇。一个簇包含 4 << n（n >= 0）个逻辑页，压缩大小也就是簇大小，每个簇可以被压缩或不压缩。

- 在簇元数据布局中，使用一个特殊块地址来指示一个簇是压缩簇还是普通簇；对于压缩簇，其后的元数据将簇映射到 [1, 4 << n - 1] 个物理块，f2fs 在其中存储包含压缩头与压缩数据的数据。

- 为了在覆盖写入期间消除写放大，F2FS 仅支持对只写一次（write-once）的文件进行压缩；只有当簇中所有逻辑块都包含有效数据、且簇数据的压缩比低于指定阈值时，数据才能被压缩。

- 要对普通 inode 启用压缩，有四种方式：

  - chattr +c file
  - chattr +c dir; touch dir/file
  - mount w/ -o compress_extension=ext; touch file.ext
  - mount w/ -o compress_extension=*; touch any_file

- 要对普通 inode 禁用压缩，有两种方式：

  - chattr -c file
  - mount w/ -o nocompress_extension=ext; touch file.ext

- FS_COMPR_FL、FS_NOCOMP_FS 与扩展名之间的优先级：

  - compress_extension=so; nocompress_extension=zip; chattr +c dir; touch
    dir/foo.so; touch dir/bar.zip; touch dir/baz.txt; 则 foo.so 与 baz.txt
    应被压缩，bar.zip 应不被压缩。chattr +c dir/bar.zip 可在 bar.zip 上启用压缩。
  - compress_extension=so; nocompress_extension=zip; chattr -c dir; touch
    dir/foo.so; touch dir/bar.zip; touch dir/baz.txt; 则 foo.so 应被
    压缩，bar.zip 与 baz.txt 应不被压缩。
    chattr+c dir/bar.zip; chattr+c dir/baz.txt; 可在 bar.zip 与 baz.txt 上启用压缩。
- 此时，压缩特性不会直接向用户暴露压缩后的空间，以保证该空间后续潜在的数据更新。相反，其主要目标是尽可能减少写入闪存盘的数据，从而延长磁盘寿命并缓解 I/O 拥塞。此外，我们添加了 ioctl(F2FS_IOC_RELEASE_COMPRESS_BLOCKS) 接口，用于在为 inode 设置特殊标志后回收压缩空间并展示给用户。一旦压缩空间被释放，该标志将阻止向文件写入数据，直到通过 ioctl(F2FS_IOC_RESERVE_COMPRESS_BLOCKS) 预留压缩空间，或将文件大小截断为零。
```

				[Dnode Structure]
		+-----------------------------------------------+
		| cluster 1 | cluster 2 | ......... | cluster N |
		+-----------------------------------------------+
		.           .                       .           .
	  .                      .                .                      .
    .         Compressed Cluster       .        .        Normal Cluster            .
    +----------+---------+---------+---------+  +---------+---------+---------+---------+
    |compr flag| block 1 | block 2 | block 3 |  | block 1 | block 2 | block 3 | block 4 |
    +----------+---------+---------+---------+  +---------+---------+---------+---------+
	       .                             .
	    .                                           .
	.                                                           .
	+-------------+-------------+----------+----------------------------+
	| data length | data chksum | reserved |      compressed data       |
	+-------------+-------------+----------+----------------------------+

```
### 压缩模式


f2fs 通过“compression_mode”挂载选项支持“fs”与“user”两种压缩模式。使用该选项，f2fs 提供了选择如何压缩启用压缩的文件的方式（如何对普通 inode 启用压缩，请参考“Compression implementation”章节）。

1) compress_mode=fs

   这是默认选项。f2fs 在对启用压缩的文件执行回写（writeback）时自动进行压缩。

2) compress_mode=user

   这会禁用自动压缩，并将选择目标文件与时机的决定权交给用户。用户可以使用 F2FS_IOC_DECOMPRESS_FILE 与 F2FS_IOC_COMPRESS_FILE ioctl，对启用压缩的文件进行手动压缩/解压缩，如下所示。
```

  fd = open(filename, O_WRONLY, 0);
  ret = ioctl(fd, F2FS_IOC_DECOMPRESS_FILE);

```
```

  fd = open(filename, O_WRONLY, 0);
  ret = ioctl(fd, F2FS_IOC_COMPRESS_FILE);

```
### NVMe 分区命名空间（Zoned Namespace）设备


- ZNS 定义了一个每区域（per-zone）容量，它可以等于或小于区域大小（zone-size）。Zone-capacity 是该区域中可用块的数量。F2FS 会检查 zone-capacity 是否小于 zone-size，如果是，则在初始挂载时，任何起始位置在 zone-capacity 之后的段会在空闲段位图中被标记为不可用。这些段被标记为永久使用，因此不会被分配用于写入，也就无需进行垃圾回收。如果 zone-capacity 未与默认段大小（2MB）对齐，那么一个段可以在 zone-capacity 之前开始并跨越 zone-capacity 边界。这种跨边界的段也被视为可用段。这些段中位于 zone-capacity 之后的所有块都被视为不可用。

### 设备别名（device aliasing）特性


f2fs 可以利用一个称为“设备别名文件（device aliasing file）”的特殊文件。该文件允许用单个大 extent 映射整个存储设备，而不使用常规的 f2fs 节点结构。这块被映射的区域被固定（pinned），主要用于占据空间。

本质上，这一机制允许 f2fs 区域的一部分被临时保留并供另一个文件系统或其他用途使用。一旦该外部使用完成，设备别名文件即可被删除，将保留的空间释放回 F2FS 供其自身使用。


   # ls /dev/vd*
   /dev/vdb (32GB) /dev/vdc (32GB)
   # mkfs.ext4 /dev/vdc
   # mkfs.f2fs -c /dev/vdc@vdc.file /dev/vdb
   # mount /dev/vdb /mnt/f2fs
   # ls -l /mnt/f2fs
   vdc.file
   # df -h
   /dev/vdb                            64G   33G   32G  52% /mnt/f2fs

   # mount -o loop /dev/vdc /mnt/ext4
   # df -h
   /dev/vdb                            64G   33G   32G  52% /mnt/f2fs
   /dev/loop7                          32G   24K   30G   1% /mnt/ext4
   # umount /mnt/ext4

   # f2fs_io getflags /mnt/f2fs/vdc.file
   get a flag on /mnt/f2fs/vdc.file ret=0, flags=nocow(pinned),immutable
   # f2fs_io setflags noimmutable /mnt/f2fs/vdc.file
   get a flag on noimmutable ret=0, flags=800010
   set a flag on /mnt/f2fs/vdc.file ret=0, flags=noimmutable
   # rm /mnt/f2fs/vdc.file
   # df -h
   /dev/vdb                            64G  753M   64G   2% /mnt/f2fs

因此，其核心思想是：用户可以在 /dev/vdc 上执行任何文件操作，并在使用完后回收空间，而这些空间被计入 /data。这不需要修改分区大小与文件系统格式。

### 每文件只读大 Folio 支持


F2FS 在读取路径上实现了大 folio（large folio）支持，以利用高阶页分配获得显著的性能提升。为了最小化代码复杂度，该支持目前被排除在写入路径之外，因为写入路径需要处理压缩与块分配模式等复杂优化。

这一可选特性仅在文件的 immutable 位被设置时触发。因此，即使用户在清除该位后立即尝试以写权限打开一个已缓存的文件，F2FS 也会返回 EOPNOTSUPP。只有在已缓存的 inode 被丢弃后，写访问才会恢复。其使用流程如下所示：


   # f2fs_io setflags immutable /data/testfile_read_seq

   /** flush and reload the inode to enable the large folio **/
   # sync && echo 3 > /proc/sys/vm/drop_caches

   /** mmap(MAP_POPULATE) + mlock() **/
   # f2fs_io read 128 0 1024 mmap 1 0 /data/testfile_read_seq
   /** mmap() + fadvise(POSIX_FADV_WILLNEED) + mlock() **/
   # f2fs_io read 128 0 1024 fadvise 1 0 /data/testfile_read_seq

   /** mmap() + mlock2(MLOCK_ONFAULT) + madvise(MADV_POPULATE_READ) **/
   # f2fs_io read 128 0 1024 madvise 1 0 /data/testfile_read_seq

   # f2fs_io clearflags immutable /data/testfile_read_seq

   # f2fs_io write 1 0 1 zero buffered /data/testfile_read_seq
   Failed to open /mnt/test/test: Operation not supported

   /** flush and reload the inode to disable the large folio **/
   # sync && echo 3 > /proc/sys/vm/drop_caches

   # f2fs_io write 1 0 1 zero buffered /data/testfile_read_seq
   Written 4096 bytes with pattern = zero, total_time = 29 us, max_latency = 28 us

   # rm /data/testfile_read_seq