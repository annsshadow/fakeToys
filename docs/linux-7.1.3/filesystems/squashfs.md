
## Squashfs 4.0 文件系统


Squashfs 是 Linux 的一种压缩的只读文件系统。

它使用 zlib、lz4、lzo、xz 或 zstd 压缩来压缩文件、inode 与目录。系统中的 inode
非常小，并且所有块都被打包以最小化数据开销。支持大于 4K 的块大小，最大可达 1
兆字节（默认块大小 128K）。

Squashfs 旨在用于通用的只读文件系统、归档用途（即原本可能使用 .tar.gz 文件的
场景），以及需要低开销的受限制块设备/内存系统（例如嵌入式系统）。

邮件列表（内核代码）：linux-fsdevel@vger.kernel.org
网站：github.com/plougher/squashfs-tools

### 1. 文件系统特性


Squashfs 文件系统特性与 Cramfs 的对比：

============================== 	=========		==========
				Squashfs		Cramfs
============================== 	=========		==========
Max filesystem size		2^64			256 MiB
Max file size			~ 2 TiB			16 MiB
Max files			unlimited		unlimited
Max directories			unlimited		unlimited
Max entries per directory	unlimited		unlimited
Max block size			1 MiB			4 KiB
Metadata compression		yes			no
Directory indexes		yes			no
Sparse file support		yes			no
Tail-end packing (fragments)	yes			no
Exportable (NFS etc.)		yes			no
Hard link support		yes			no
"." and ".." in readdir		yes			no
Real inode numbers		yes			no
32-bit uids/gids		yes			no
File creation time		yes			no
Xattr support			yes			no
ACL support			no			no
============================== 	=========		==========

Squashfs 压缩数据、inode 与目录。此外，inode 与目录数据被高度压缩，并按字节边界
打包。每个被压缩的 inode 平均长度为 8 字节（确切长度随文件类型而变，即常规文件、
目录、符号链接以及块/字符设备 inode 的大小各不相同）。

### 2. 使用 Squashfs


由于 squashfs 是只读文件系统，必须使用 mksquashfs 程序来创建已填充内容的
squashfs 文件系统。该程序及其他 squashfs 工具很可能已被你的 Linux 发行版打包
（名为 squashfs-tools）。源代码可从 github.com/plougher/squashfs-tools 获取。
使用说明也可从同一站点获取。

### 2.1 挂载选项


===================    =========================================================
errors=%s              Specify whether squashfs errors trigger a kernel panic
                       or not

		       ==========  =============================================
                         continue  errors don't trigger a panic (default)
                            panic  trigger a panic when errors are encountered,
                                   similar to several other filesystems (e.g.
                                   btrfs, ext4, f2fs, GFS2, jfs, ntfs, ubifs)

                                   This allows a kernel dump to be saved,
                                   useful for analyzing and debugging the
                                   corruption.
                       ==========  =============================================
threads=%s             Select the decompression mode or the number of threads

                       If SQUASHFS_CHOICE_DECOMP_BY_MOUNT is set:

		       ==========  =============================================
                           single  use single-threaded decompression (default)

                                   Only one block (data or metadata) can be
                                   decompressed at any one time. This limits
                                   CPU and memory usage to a minimum, but it
                                   also gives poor performance on parallel I/O
                                   workloads when using multiple CPU machines
                                   due to waiting on decompressor availability.
                            multi  use up to two parallel decompressors per core

                                   If you have a parallel I/O workload and your
                                   system has enough memory, using this option
                                   may improve overall I/O performance. It
                                   dynamically allocates decompressors on a
                                   demand basis.
                           percpu  use a maximum of one decompressor per core

                                   It uses percpu variables to ensure
                                   decompression is load-balanced across the
                                   cores.
                        1|2|3|...  configure the number of threads used for
                                   decompression

                                   The upper limit is num_online_cpus() * 2.
                       ==========  =============================================

                       If SQUASHFS_CHOICE_DECOMP_BY_MOUNT is **not** set and
                       SQUASHFS_DECOMP_MULTI, SQUASHFS_MOUNT_DECOMP_THREADS are
                       both set:

		       ==========  =============================================
                          2|3|...  configure the number of threads used for
                                   decompression

                                   The upper limit is num_online_cpus() * 2.
                       ==========  =============================================

===================    =========================================================

### 3. Squashfs 文件系统设计


一个 squashfs 文件系统最多由九个部分组成，一起打包在

```
	 ---------------
	|  superblock 	|
	|---------------|
	|  compression  |
	|    options    |
	|---------------|
	|  datablocks   |
	|  & fragments  |
	|---------------|
	|  inode table	|
	|---------------|
	|   directory	|
	|     table     |
	|---------------|
	|   fragment	|
	|    table      |
	|---------------|
	|    export     |
	|    table      |
	|---------------|
	|    uid/gid	|
	|  lookup table	|
	|---------------|
	|     xattr     |
	|     table	|
	 ---------------
```

压缩数据块在从源目录读取文件时被写入文件系统，并检查重复项。一旦所有文件数据
写入完毕，就会写入已完成的 inode、目录、fragment、export、uid/gid 查找以及 xattr
表。

### 3.1 压缩选项


压缩器可以选择性地支持特定于压缩的选项（例如字典大小）。如果使用了非默认的
压缩选项，则这些选项存储于此。

### 3.2 Inodes


元数据（inode 与目录）以 8K 字节块为单位压缩。每个压缩块前面有一个两字节的长度，
如果该块未被压缩则最高位置位。如果设置了 -noI 选项，或者压缩后的块大于未压缩的
块，则该块不会被压缩。

inode 被打包进元数据块中，并且不与块边界对齐，因此 inode 会重叠在压缩块上。inode
由一个 48 位数字标识，该数字编码了包含该 inode 的压缩元数据块的位置，以及该
inode 在该块中的字节偏移（<block, offset>）。

为了最大化压缩，针对每种文件类型（常规文件、目录、设备等）有不同的 inode，其
内容与长度随类型而变。

为了进一步最大化压缩，定义了两类常规文件 inode 和目录 inode：针对频繁出现的
常规文件和目录优化的 inode，以及需要存储额外信息的扩展类型。

### 3.3 目录


与 inode 类似，目录被打包进压缩的元数据块中，存储在目录表里。目录通过包含该
目录的 metablock 的起始地址以及进入解压后块的偏移来访问（<block, offset>）。

目录的组织方式略微复杂，并非简单的文件名列表。这种组织方式利用了以下事实：
（在大多数情况下）文件的 inode 会位于同一个压缩元数据块中，因此可共享起始块。
于是目录以两级列表组织：一个目录头包含共享的起始块值，后跟一系列目录项，每项
共享该起始块。一旦 inode 起始块发生变化，就会写入一个新的目录头。目录头/目录项
列表按需重复多次。

目录是有序的，并且可以包含目录索引以加速文件查找。目录索引为每个 metablock 存储
一个条目，每个条目存储该元数据块中第一个目录头的索引/文件名映射。目录按字母顺序
排序，查找时线性扫描索引，寻找第一个字母顺序大于被查找文件名的文件名。此时就
找到了文件名所在元数据块的位置。索引的总体思路是：无论目录多长，查找都只需解压
一个元数据块。该方案的优势在于不需要额外的内存开销，也不需要磁盘上过多的额外
存储。

### 3.4 文件数据


常规文件由一串连续的压缩块和/或一个压缩的 fragment 块（尾部打包块）组成。每个
数据块的压缩大小存储在文件 inode 内的块列表中。

为了在读取“大”文件（256 兆字节或更大）时加速对数据块的访问，代码实现了一个索引
缓存，缓存从块索引到磁盘上数据块位置的映射。

该索引缓存使 Squashfs 能够处理大文件（最大 1.75 TiB），同时在磁盘上保留简单且
节省空间的块列表。缓存被划分为多个槽位，最多可缓存 8 个 224 GiB 的文件（128 KiB
块）。更大的文件使用多个槽位，1.75 TiB 的文件会使用全部 8 个槽位。索引缓存被设计
为内存高效，默认使用 16 KiB。

### 3.5 Fragment 查找表


常规文件可以包含一个 fragment 索引，该索引通过 fragment 查找表映射到磁盘上的
fragment 位置与压缩大小。该 fragment 查找表本身以压缩形式存储在元数据块中。使用
第二个索引表来定位它们。出于访问速度（且因其较小）考虑，这第二个索引表在挂载时
被读取并缓存在内存中。

### 3.6 Uid/gid 查找表


为了节省空间，常规文件存储 uid 和 gid 索引，这些索引通过一个 id 查找表转换为
32 位 uid/gid。该表以压缩形式存储在元数据块中。使用第二个索引表来定位它们。出于
访问速度（且因其较小）考虑，这第二个索引表在挂载时被读取并缓存在内存中。

### 3.7 Export 表


为了使 Squashfs 文件系统可导出（通过 NFS 等），文件系统可以可选地（通过 -no-exports
Mksquashfs 选项禁用）包含一个 inode 号到 inode 磁盘位置的查找表。这是为了使
Squashfs 能够将文件句柄中传入的 inode 号映射到磁盘上的 inode 位置，而这在导出代码
重新实例化过期/被刷出的 inode 时是必需的。

该表以压缩形式存储在元数据块中。使用第二个索引表来定位它们。出于访问速度（且因其
较小）考虑，这第二个索引表在挂载时被读取并缓存在内存中。

### 3.8 Xattr 表


xattr 表包含每个 inode 的扩展属性。每个 inode 的 xattr 存储在一个列表中，每个列表
条目包含类型、名称和值字段。类型字段编码了 xattr 前缀（“user.”、“trusted.” 等），
同时也编码了名称/值字段应如何解释。目前该类型指示值是内联存储的（此时值字段包含
xattr 值），还是外联存储的（此时值字段存储对实际值存储位置的引用）。这使得大值可以
外联存储，从而提升扫描与查找性能，并且也允许值被去重——值只存储一次，所有其他出现
处持有对该值的外联引用。

xattr 列表被打包进压缩的 8K 元数据块中。为了减少 inode 中的开销，inode 内并不存储
xattr 列表的磁盘位置，而是存储一个 32 位的 xattr id。该 xattr id 通过第二个 xattr id
查找表映射到 xattr 列表的位置。

### 4. TODO 与未决问题


### 4.1 TODO 列表


实现 ACL 支持。

### 4.2 Squashfs 内部缓存


Squashfs 中的块是压缩的。为了避免反复解压最近访问的数据，Squashfs 使用了两个小型
的元数据与 fragment 缓存。

该缓存不用于文件数据块，文件数据块以常规方式解压并缓存在页缓存（page-cache）中。
该缓存用于临时缓存由于元数据（即 inode 或目录）或 fragment 访问而被读取的 fragment
与元数据块。由于元数据与 fragment 被打包在一起形成块（以获得更高的压缩率），读取
某一特定元数据或 fragment 时会一并取回与之打包在一起的其它元数据/fragment，而基于
局部性原理，这些可能在不久的将来被读取。临时缓存它们确保其在近期访问时可用，而
无需额外的读取与解压。

未来这个内部缓存可能会被使用内核页缓存的实现所取代。由于页缓存以页大小为操作单位，
这可能会在锁机制及相关竞态条件方面引入额外的复杂性。
