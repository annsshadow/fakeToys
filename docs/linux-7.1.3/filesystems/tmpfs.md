
## Tmpfs


Tmpfs 是一种将其所有文件保存在虚拟内存中的文件系统

从某种意义上说，tmpfs 中的一切都是临时的：不会在你的硬盘上创建任何文件。如果你卸载一tmpfs 实例，其中存储的所有内容都会丢失
tmpfs 将一切都放入内核内部缓存，并随之增长和收缩以适应它所包含的文件，并且如果为该 tmpfs 挂载启用swap，它还能够将不需要的页换出到交换空间。tmpfs 还支THP
tmpfs ramfs 的基础上扩展出了一些用户空间可配置的选项，如下文所列并进一步解释，其中一些可以通过文件系统的重新挂载（'mount -o remount ...'）动态地重新配置。tmpfs 文件系统可以被调整大小，但不能调整到小于其当前使用量。tmpfs 还支POSIX ACL，以trusted.**、security.** user.* 命名空间的扩展属性。ramfs 不使swap，你也无法修ramfs 文件系统的任何参数。ramfs 文件系统的大小限制就是你所拥有的可用内存大小，因此如果使用它，必须小心不要耗尽内存
tmpfs ramfs 的一种替代方案是使用 brd 来创RAM 磁盘dev/ram*），它允许你在物RAM 中模拟一个块设备磁盘。要写入数据，你只需在这ramdisk 之上创建一个常规的文件系统。与 ramfs 一样，brd ramdisk 不能换出。brd ramdisk 的大小也是在初始化时配置的，你无法动态调整它们的大小。与 brd ramdisk 相反，tmpfs 拥有自己的文件系统，它完全不依赖块层
由于 tmpfs 完全存在于页缓存中，并且可选地存在swap 中，所tmpfs 页都会在 /proc/meminfo 中显示为 "Shmem"，在 free(1) 中显示为 "Shared"。注意这些计数器也包含共享内存（shmem，参ipcs(1)）。获取计数的最可靠方式是使df(1) du(1)
tmpfs 有以下用途：

1) 总有一个内核内部的挂载，你根本看不到它。它用于共享匿名映射SYSV 共享内存
   这个挂载不依赖于 CONFIG_TMPFS。如果没有设CONFIG_TMPFS，tmpfs 的用户可见部分不会被构建。但内部机制始终存在
2) glibc 2.2 及以上版本期tmpfs 挂载/dev/shm 以用POSIX 共享内存（shm_open、shm_unlink）。添加以下内```

	tmpfs	/dev/shm	tmpfs	defaults	0 0

   如果需要，记得创建你打算将 tmpfs 挂载到的目录
   这个挂载对于 SYSV 共享内存不是必需的。内部挂载用于此目的。（2.3 内核版本中，有必要挂tmpfs 的前身（shm fs）来使用 SYSV 共享内存。）

```
3) 有些人（包括我）发现将其挂载在例/tmp /var/tmp 上并拥有一个大的交换分区非常方便。现tmpfs 文件的回环挂载确实可以工作，因此大多数发行版自带mkinitrd 应该能在 tmpfs /tmp 上成功
4) 可能还有更多我不知道的用:-)


tmpfs 有三个用于调整大小的挂载选项
=========  ============================================================
size       tmpfs 实例所分配的字节数的上限。默认是不含 swap 的物           RAM 的一半。如果你tmpfs 实例设置得过大，机器将发生死锁，
           因为 OOM 处理程序将无法释放那部分内存nr_blocks  size 相同，但PAGE_SIZE 大小的块为单位nr_inodes  该实例中 inode 的最大数量。默认是你的物理 RAM 页数量的一半，
           或者（在带highmem 的机器上）lowmem RAM 页的数量，取两           中较小的一个=========  ============================================================

这些参数接受后缀 k、m g，分别表kilo、mega giga，并且可以在重新挂载时更改。size 参数还接受后缀 %，将tmpfs 实例限制为物RAM 的该百分比：当既未指size 也未指定 nr_blocks 时，默认size=50%
如果 nr_blocks=0（或 size=0），则该实例中的块不受限制；如果 nr_inodes=0，则 inode 不受限制。以这样的选项挂载通常是不明智的，因为它允许任何有写访问权限的用户耗尽机器上的所有内存；但这增强了该实例在拥有许CPU、密集使用它的系统中的可扩展性
如果 nr_inodes 不为 0，那么用inode 的这块受限空间也会被扩展属性消耗："df -i" IUsed IUse% 会增加，IFree 会减少
tmpfs 的页可以在内存短缺时被换出。tmpfs 有一个挂载选项可以禁用其对 swap 的使用：

======  ===========================================================
noswap  禁用 swap。重新挂载必须尊重原始设置。默认情况下启用swap======  ===========================================================

tmpfs 还支持透明大页（Transparent Huge Pages），这需要一个配置了 CONFIG_TRANSPARENT_HUGEPAGE 的内核，并且你的系统支持 huge（has_transparent_hugepage()，这是架构相关的）。相关的挂载选项为：

================ ==============================================================
huge=never       不分配大页。这是默认值huge=always      每次需要新页时都尝试分配大页huge=within_size 仅当大页将完全位i_size 之内时才分配。同时尊madvise(2) 提示huge=advise      仅当通过 madvise(2) 请求时才分配大页================ ==============================================================

另请参阅 Documentation/admin-guide/mm/transhuge.rst，其中描述了 sysfs 文件 /sys/kernel/mm/transparent_hugepage/shmem_enabled：在紧急情况下它可以用于拒绝所tmpfs 挂载上的大页，或者为测试而在所tmpfs 挂载上强制使用大页
tmpfs 还支持配额，使用以下挂载选项
======================== =================================================
quota                    在挂载上启用用户和组配额记账与强制                         Tmpfs 使用在挂载时初始化的隐藏系统配额文件usrquota                 在挂载上启用用户配额记账与强制grpquota                 在挂载上启用组配额记账与强制usrquota_block_hardlimit 设置全局用户配额块硬限制usrquota_inode_hardlimit 设置全局用户配额 inode 硬限制grpquota_block_hardlimit 设置全局组配额块硬限制grpquota_inode_hardlimit 设置全局组配inode 硬限制======================== =================================================

任何与配额相关的挂载选项都不能在重新挂载时设置或更改
配额限制参数接受后缀 k、m g（表kilo、mega、giga），并且不能在重新挂载时更改。默认的全局配额限制会在用户/项目（root 除外）的配额条目首次被访问时生效——通常就是挂载后首次创建一个具有特id 所有权inode 时。换句话说，这些限制不是被初始化为零，而是被初始化为通过这些挂载选项提供的特定值。这些限制可以随时为任何用户/id 更改，就像通常可以做的那样
注意，tmpfs 配额不支持用户命名空间，因此如果在用户命名空间内启用了配额，不会进行任何 uid/gid 转换
tmpfs 有一个挂载选项，用于设置该实例中所有文件的 NUMA 内存分配策略（如果启用了 CONFIG_NUMA）——可以通过 'mount -o remount ...' 动态调整
======================== ==============================================
mpol=default             使用进程分配策略
                         （参set_mempolicy(2)mpol=prefer:Node         优先从给定的 Node 分配内存
mpol=bind:NodeList       仅从 NodeList 中的节点分配内存
mpol=interleave          轮流优先从每个节点分mpol=interleave:NodeList 轮流NodeList 的每个节点分mpol=local		 优先从本地节点分配内======================== ==============================================

NodeList 格式是以逗号分隔的十进制数字和范围的列表，一个范围是由两个以连字符分隔的十进制数字（该范围中最小和最大的节点号）组成。例如，mpol=bind:0-3,5,7,9-15

带有有效 NodeList 的内存策略会被保存（按指定的样子），供文件创建时使用。当一个任务在文件系统中分配一个文件时，挂载选项的内存策略将NodeList（如果有）一起应用，并由调用任务cpuset 约束 [参见 Documentation/admin-guide/cgroup-v1/cpusets.rst] 以及下面列出的任何可选标志修改。如果得到的结果 NodeList 是空集，则该文件的有效内存策略将回退"default" 策略
NUMA 内存分配策略有可与它们的模式结合使用的可选标志。这些可选标志可以在挂载 tmpfs 时，通过将它们附加到模式之前、NodeList 之前来指定。请参阅 Documentation/admin-guide/mm/numa_memory_policy.rst，了解所有可用的内存分配策略模式标志及其对内存策略的影响
```

	=static		is equivalent to	MPOL_F_STATIC_NODES
	=relative	is equivalent to	MPOL_F_RELATIVE_NODES

```
例如，mpol=bind=static:NodeList 等价于分配策MPOL_BIND | MPOL_F_STATIC_NODES
注意，如果正在运行的内核不支NUMA，则尝试mpol 选项挂载 tmpfs 将会失败；如果其 nodelist 指定了一个不在线的节点，也会失败。如果你的系统依赖于tmpfs 被挂载，但时不时会运行一个没NUMA 能力（也许是安全恢复内核）的内核，或者只有更少的节点在线，那么建议从自动挂载选项中省mpol 选项。稍后，tmpfs 已经挂载MountPoint 上时，可以通过 'mount -o remount,mpol=Policy:NodeList MountPoint' 添加它

要指定初始的根目录，可以使用以下挂载选项
====	==================================
mode	以八进制数表示的权限
uid	用户 id
gid	缁?id
====	==================================

这些选项对重新挂载没有任何效果。你可以在已挂载的文件系统上chmod(1)、chown(1) chgrp(1) 更改这些参数

tmpfs 有一个挂载选项，用于选择它在 32 位还64 inode 编号处回绕：

=======   ========================
inode64   使用 64 inode 编号
inode32   使用 32 inode 编号
=======   ========================

32 位内核上，inode32 是隐式的，inode64 在挂载时被拒绝。在 64 位内核上，CONFIG_TMPFS_INODE64 设置默认值。inode64 避免了在单个设备上多个文件具有相inode 编号的可能性；但当达到 33 inode 编号时，它可能导glibc EOVERFLOW 失败——如果长久存在的 tmpfs 被如此古老的 32 位应用程序访问，以至于打开大于 2GiB 的文件会EINVAL 失败
因此 'mount -t tmpfs -o size=10G,nr_inodes=10k,mode=700 tmpfs /mytmpfs' 会给你一个位/mytmpfs 上的 tmpfs 实例，它可以分配 10GB RAM/SWAP 10240 inode，并且只能被 root 访问
tmpfs 有以下用于大小写不敏感查找支持的挂载选项
================= ==============================================================
casefold          在此挂载点使用给定的参数作为编码标准启用 casefold 支持                  目前仅支UTF-8 编码。如果未使用参数，它将加载可用的最                  UTF-8 编码strict_encoding   在此挂载点启用严格编码（默认禁用）。在此模式下，文件系                  拒绝创建名称包含无效 UTF-8 字符的文件和目录================= ==============================================================

这个选项不会使整个文件系统大小写不敏感。还需要通过在一个空目录中翻+F 属性，逐个目录地设casefold 标志。不过，新目录会继承该属性。挂载点本身不能被设为大小写不敏感
```

    $ mount -t tmpfs -o casefold=utf8-12.1.0,strict_encoding fs_name /mytmpfs
    $ mount -t tmpfs -o casefold fs_name /mytmpfs


```
:Author:
   Christoph Rohland <cr@sap.com>, 1.12.01
:Updated:
   Hugh Dickins, 4 June 2007
:Updated:
   KOSAKI Motohiro, 16 Mar 2010
:Updated:
   Chris Down, 13 July 2020
:Updated:
   Andr茅 Almeida, 23 Aug 2024
