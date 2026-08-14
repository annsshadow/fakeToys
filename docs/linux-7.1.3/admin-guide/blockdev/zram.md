## zram：基于内存的压缩块设备


## 简介


zram 模块会创建名为 /dev/zram<id>（<id> = 0, 1, ...）的基于内存的块设备。
写入这些磁盘的页会被压缩并直接存储在内存中。这些磁盘具有非常快的 I/O
性能，并且压缩能够带来可观的内存节省。部分使用场景包括 /tmp 存储、用作
swap 磁盘、/var 下的各种缓存，以及可能更多的用途。:)

各个 zram 设备的统计信息通过 /sys/block/zram<id>/ 下的 sysfs 节点导出。

## 用法


配置和管理 zram 设备有以下几种方式：

a) 使用 zram 与 zram_control 的 sysfs 属性
b) 使用 util-linux 提供的 zramctl 工具（util-linux@vger.kernel.org）。

本文档仅描述“手动”配置 zram 的步骤，即 zram 与 zram_control 的 sysfs 属性。

若想进一步了解 zramctl，请查阅 util-linux 的文档、zramctl 手册页或
`zramctl --help`。请注意，zram 的维护者并不开发/维护 util-linux 或
zramctl，如有任何问题请联系 util-linux@vger.kernel.org。

下面展示使用 zram 的典型步骤序列。

## 警告


为简洁起见，下面的大多数示例省略了错误检查部分。但是，处理错误是你的
全部责任。

zram 的 sysfs 属性在出错时总是返回负值。可能的返回码列表如下：

========  =============================================================
-EBUSY	  试图修改设备初始化后无法再更改的属性。请先重置设备。
-ENOMEM	  zram 无法分配足够的内存来满足你的需求。
-EINVAL	  提供了无效的输入。
-EAGAIN	  稍后重试该操作（例如当试图同时执行 recompress 与 writeback 时）。
========  =============================================================

如果你使用 'echo'，返回值由 'echo' 工具设置，因此

```
	echo foo > /sys/block/zram0/comp_algorithm
	if [ $? -ne 0 ]; then
		handle_error
	fi

```
就足够了。

## 1) 加载模块


```
	modprobe zram num_devices=4
```

这会创建 4 个设备：/dev/zram{0,1,2,3}

num_devices 参数是可选的，用于告诉 zram 应预先创建多少设备。默认值：1。

## 2) 选择压缩算法


通过 comp_algorithm 设备属性，可以查看可用以及当前选中的（以方括号显示）
压缩算法，或者在设备初始化后更改所选的压缩算法（设备一旦初始化便无法再
更改压缩算法）。

```
	# 显示支持的压缩算法
	cat /sys/block/zram0/comp_algorithm
	lzo [lz4]

	# 选择 lzo 压缩算法
	echo lzo > /sys/block/zram0/comp_algorithm
```

目前，`comp_algorithm` 的内容仅显示 zram 所支持的压缩算法。

## 3) 设置压缩算法参数：可选


压缩算法可能支持针对特定数据集进行调整的特定参数。ZRAM 提供了一个
`algorithm_params` 设备属性，用于按算法进行参数配置。

例如，若干压缩算法支持 `level` 参数。此外，某些压缩算法支持预训练字典，
会显著改变算法的特性。为了让压缩算法使用外部的预训练字典，传入完整的

```
	# 传入预训练 zstd 字典的路径
	echo "algo=zstd dict=/etc/dictionary" > /sys/block/zram0/algorithm_params

	# 同样的方式，但使用算法优先级
	echo "priority=1 dict=/etc/dictionary" > \
		/sys/block/zram0/algorithm_params

	# 传入预训练 zstd 字典路径以及压缩级别
	echo "algo=zstd level=8 dict=/etc/dictionary" > \
		/sys/block/zram0/algorithm_params
```

参数是算法相关的：并非所有算法都支持预训练字典，也并非所有算法都支持
`level`。此外，对于某些算法，`level` 控制压缩级别（值越高压缩比越好，
某些算法甚至可以取负值）；对于另一些算法，`level` 是加速级别（值越高
压缩比越低）。

## 4) 设置磁盘大小


通过向 sysfs 节点 'disksize' 写入值来设置磁盘大小。该值可以是字节数，
也可以使用内存后缀。

```
	# 以 50MB 的磁盘大小初始化 /dev/zram0
	echo $((50*1024*1024)) > /sys/block/zram0/disksize

	# 使用内存后缀
	echo 256K > /sys/block/zram0/disksize
	echo 512M > /sys/block/zram0/disksize
	echo 1G > /sys/block/zram0/disksize
```

注意：
由于期望达到 2:1 的压缩比，创建大小超过内存两倍的 zram 意义不大。请注意，
zram 在未被使用时约占用磁盘大小的 0.1% 内存，因此过大的 zram 是浪费的。

## 5) 设置内存上限：可选


通过向 sysfs 节点 'mem_limit' 写入值来设置内存上限。该值可以是字节数，
也可以使用内存后缀。此外，你可以在运行时更改该值。

```
	# 限制 /dev/zram0 使用 50MB 内存
	echo $((50*1024*1024)) > /sys/block/zram0/mem_limit

	# 使用内存后缀
	echo 256K > /sys/block/zram0/mem_limit
	echo 512M > /sys/block/zram0/mem_limit
	echo 1G > /sys/block/zram0/mem_limit

	# 禁用内存上限
	echo 0 > /sys/block/zram0/mem_limit
```

## 6) 激活


```
	mkswap /dev/zram0
	swapon /dev/zram0

	mkfs.ext4 /dev/zram1
	mount /dev/zram1 /tmp
```

## 7) 添加/移除 zram 设备


zram 提供了一个控制接口，支持动态（按需）添加和移除设备。

要添加一个新的 /dev/zramX 设备，请对 hot_add 属性执行读操作。该操作会
返回新设备的设备 id（意味着你可以使用 /dev/zram<id>），或者返回一个错误码。

```
	cat /sys/class/zram-control/hot_add
	1
```

要移除已有的 /dev/zramX 设备（其中 X 为设备 id）

```
	echo X > /sys/class/zram-control/hot_remove
```

## 8) 统计信息


每个设备的统计信息作为 /sys/block/zram<id>/ 下的各种节点导出。

下面是已导出设备属性的简要说明。更多细节请阅读
Documentation/ABI/testing/sysfs-block-zram。

======================  ======  ===============================================
Name            	access            description
======================  ======  ===============================================
disksize          	RW	显示并设置设备的磁盘大小
initstate         	RO	显示设备的初始化状态
reset             	WO	触发设备重置
mem_used_max      	WO	重置 `mem_used_max` 计数器（见后文）
mem_limit         	WO	指定 ZRAM 可用于存储压缩数据的最大内存量
writeback_limit   	WO	指定 zram 可以写出到后端设备的最大写 IO
				量，以 4KB 为单位
writeback_limit_enable  RW	显示并设置 writeback_limit 功能
writeback_batch_size	RW	显示并设置最大的在途 writeback 操作数量
compressed_writeback	RW	显示并设置压缩 writeback 功能
comp_algorithm    	RW	显示并更改压缩算法
algorithm_params	WO	设置压缩算法参数
compact           	WO	触发内存规整
debug_stat        	RO	该文件用于 zram 调试目的
backing_dev	  	RW	为 zram 设置用于写出的后端存储
idle		  	WO	将已分配的槽位标记为 idle
======================  ======  ===============================================

建议用户空间使用以下文件来读取设备统计信息。

文件 /sys/block/zram<id>/stat

表示块层统计信息。细节请阅读 Documentation/block/stat.rst。

文件 /sys/block/zram<id>/io_stat

该 stat 文件表示未被块层统计、因而在 zram<id>/stat 文件中不可用的设备 I/O
统计信息。它由单行文本组成，包含以下以空白分隔的统计项：

 =============    =============================================================
 failed_reads     读取失败的次数
 failed_writes    写入失败的次数
 invalid_io       非页大小对齐的 I/O 请求数量
 notify_free      取决于设备使用场景，可能统计

                  a) 由于 swap 槽位释放通知而释放的页数

                  b) 由于 bio 发送的 REQ_OP_DISCARD 请求而释放的页数。前者在
                     释放 swap 槽位时发送给 swap 块设备，这意味着该磁盘正被
                     用作 swap 磁盘。

                  b) 后者由以 discard 选项挂载的文件系统在丢弃某些数据块时
                     发送。
 =============    =============================================================

文件 /sys/block/zram<id>/mm_stat

该 mm_stat 文件表示设备的 mm 统计信息。它由单行文本组成，包含以下以空白
分隔的统计项：

 ================ =============================================================
 orig_data_size   存储在该磁盘中的数据的未压缩大小。
                  单位：字节
 compr_data_size  存储在该磁盘中的数据的压缩后大小
 mem_used_total   为该磁盘分配的内存量。这包含为该磁盘分配的分配器碎片和
                  元数据开销。因此，可以使用 compr_data_size 和该项统计计算
                  分配器的空间效率。
                  单位：字节
 mem_limit         ZRAM 可用于存储压缩数据的最大内存量
 mem_used_max      zram 为存储数据所消耗的最大内存量
 same_pages        写入该磁盘的、被相同元素填充的页数量。
                   此类页不分配内存。
 pages_compacted   规整过程中释放的页数
 huge_pages	  不可压缩页的数量
 huge_pages_since  zram 建立以来不可压缩页的数量
 ================ =============================================================

文件 /sys/block/zram<id>/bd_stat

该 bd_stat 文件表示设备的后端设备统计信息。它由单行文本组成，包含以下以
空白分隔的统计项：

 ============== =============================================================
 bd_count	写入后端设备的数据大小。
		单位：4K 字节
 bd_reads	从后端设备读取的次数
		单位：4K 字节
 bd_writes	写入后端设备的次数
		单位：4K 字节
 ============== =============================================================

## 9) 停用


```
	swapoff /dev/zram0
	umount /dev/zram1
```

## 10) 重置


```
		echo 1 > /sys/block/zram0/reset
		echo 1 > /sys/block/zram1/reset

		这会释放为该设备分配的所有内存，并将磁盘大小重置为零。
		在重新使用该设备之前，你必须再次设置磁盘大小。
```

## 可选功能


### IDLE 页跟踪


zram 内置支持 idle 页跟踪（即已分配但未被使用的页）。该功能对例如 zram
writeback 等非常有用，可

```
	echo all > /sys/block/zramX/idle
```

这会将所有已分配的 zram 页标记为 idle。只有当该页（块）被访问（例如被
覆盖或释放）时，idle 标记才会被移除。此外，当启用 CONFIG_ZRAM_TRACK_ENTRY_ACTIME
时，可以根据距上次访问已过去的秒数将页标记为 idle：

```
	echo 86400 > /sys/block/zramX/idle
```

在本例中，所有超过 86400 秒（一天）未被访问的页将被标记为 idle。

### writeback


通过 CONFIG_ZRAM_WRITEBACK，zram 可以将 idle/不可压缩页写入后端存储，而
不是保留在内存中。

```
	echo /dev/sda5 > /sys/block/zramX/backing_dev
```

在设置 disksize 之前。目前它仅支持分区。

```
	echo huge > /sys/block/zramX/writeback
```

```
	echo idle > /sys/block/zramX/writeback
```

通过该命令，zram 会将内存中的 idle 页写回存储。

此外，如果用户选择只写回 huge 和 idle 页，

```
        echo huge_idle > /sys/block/zramX/writeback
```

如果用户选择只写回不可压缩页（即那些

```
	echo incompressible > /sys/block/zramX/writeback
```

如果管理员想把 zram 设备中的某个特定页写入后端设备，

```
	echo "page_index=1251" > /sys/block/zramX/writeback
```

在 Linux 6.16 中，该接口经历了一些重构。首先，该接口现在对其所有参数
支持 `key=value` 格式（`type=huge_idle` 等）。其次，引入了对 `page_indexes`
的支持，用于指定要写回的页的 `LOW-HIGH` 范围（或多个范围）。这减少了系统
调用的数量，但更重要的是，它使得最优的后处理成为可能：

```
	echo "type=idle" > /sys/block/zramX/writeback
	echo "page_indexes=1-100 page_indexes=200-300" > \
		/sys/block/zramX/writeback
```

我们现在还允许每次调用传入多个 page_index 参数，以及混合使用

```
	echo page_index=42 page_index=99 page_indexes=100-200 \
		page_indexes=500-700 > /sys/block/zramX/writeback
```

如果闪存设备上存在大量写 IO，则可能存在闪存磨损问题，因此管理员需要
设计写入限制，以保证整个产品生命周期内的存储健康。

为解决这个问题，zram 支持 "writeback_limit" 功能。"writeback_limit_enable"
的默认值为 0，因此不限制任何 writeback。也就是说，如果管理员想要应用
writeback 预算，他们应当

```
	$ echo 1 > /sys/block/zramX/writeback_limit_enable
```

一旦设置了 writeback_limit_enable，在管理员通过 /sys/block/zramX/writeback_limit
设置预算之前，zram 不允许任何 writeback。

（如果管理员没有启用 writeback_limit_enable，那么通过 /sys/block/zramX/writeback_limit
设置的 writeback_limit 值就没有意义。）

如果管理员想在预算耗尽后再次允许写入，

```
	$ echo $((400<<MB_SHIFT>>4K_SHIFT)) > \
		/sys/block/zram0/writeback_limit
```

如果管理员想要限制每天 400M 的 writeback，可以这样做

```
	$ MB_SHIFT=20
	$ 4K_SHIFT=12
	$ echo $((400<<MB_SHIFT>>4K_SHIFT)) > \
		/sys/block/zram0/writeback_limit.
	$ echo 1 > /sys/block/zram0/writeback_limit_enable
```

```
	$ cat /sys/block/zramX/writeback_limit
```

```
	$ echo 0 > /sys/block/zramX/writeback_limit_enable
```

writeback_limit 计数会在你重置 zram 时（例如系统重启、echo 1 > /sys/block/zramX/reset）
复位，因此记录重置 zram 之前发生了多少次 writeback，以便在下次设置时分配
额外的 writeback 预算，是用户的工作。

默认情况下，zram 以解压缩（原始）形式存储写回的页，这意味着 writeback
操作在写入后端设备之前需要对该页进行解压缩。该行为可以通过启用
`compressed_writeback` 功能来改变，该功能会让 zram 将压缩后的页写入后端
设备，从而避免解压缩开销。要启用它，

```
	$ echo yes > /sys/block/zramX/compressed_writeback
```

请注意，该功能应在 `zramX` 设备初始化之前配置。

根据后端设备的存储类型，writeback 操作可能受益于更多的在途写请求（批量
写入）。最大的在途 writeback 操作数量可以通过 `writeback_batch_size` 属性
配置。要更改默认值（为 32），

```
	$ echo 64 > /sys/block/zramX/writeback_batch_size
```

如果管理员想测量某个时间段内的 writeback 计数，可以通过
/sys/block/zram0/bd_stat 的第三列获知。

### recompression


通过 `CONFIG_ZRAM_MULTI_COMP`，zram 可以使用替代（secondary）压缩算法对
页进行重新压缩。其基本思想是，替代压缩算法可以以（潜在的）更慢的压缩/
解压缩速度为代价，提供更好的压缩比。例如，替代压缩算法可以更有效地压缩
huge 页（那些默认算法未能压缩的页）。另一个应用是 idle 页重新压缩——那些
冷数据并驻留在内存中的页可以使用更有效的算法重新压缩，从而减少 zsmalloc
的内存占用。

通过 `CONFIG_ZRAM_MULTI_COMP`，zram 最多支持 4 种压缩算法：1 个主算法和
最多 3 个次级算法。zram 主压缩器在“3) 选择压缩算法”中已说明，次级算法
通过 recomp_algorithm 设备属性配置。

```
	# 显示支持的重新压缩算法
	cat /sys/block/zramX/recomp_algorithm
	#1: lzo lzo-rle lz4 lz4hc [zstd]
	#2: lzo lzo-rle lz4 [lz4hc] zstd
```

替代压缩算法按优先级排序。在上例中，zstd 用作第一个替代算法，优先级为 1，
而 lz4hc 被配置为优先级 2 的压缩算法。替代压缩算法的优先级是在配置算法时
提供的：

```
	# 选择 zstd 重新压缩算法，优先级 1
	echo "algo=zstd priority=1" > /sys/block/zramX/recomp_algorithm

	# 选择 deflate 重新压缩算法，优先级 2
	echo "algo=deflate priority=2" > /sys/block/zramX/recomp_algorithm
```

`CONFIG_ZRAM_MULTI_COMP` 启用的另一个设备属性是 `recompress`，它控制
重新压缩。

```
	# IDLE 页重新压缩由 `idle` 模式激活
	echo "type=idle priority=1" > /sys/block/zramX/recompress

	# HUGE 页重新压缩由 `huge` 模式激活
	echo "type=huge priority=2" > /sys/block/zram0/recompress

	# HUGE_IDLE 页重新压缩由 `huge_idle` 模式激活
	echo "type=huge_idle priority=1" > /sys/block/zramX/recompress
```

idle 页的数量可能很大，因此用户空间可以向 recompress 旋钮传入一个大小
阈值（以字节为单位）：zram 将只重新压缩

```
	# 重新压缩大于 3000 字节的所有页
	echo "threshold=3000 priority=1" > /sys/block/zramX/recompress

	# 重新压缩大于 2000 字节的 idle 页
	echo "type=idle threshold=2000 priority=1" > \
		/sys/block/zramX/recompress
```

也可以限制 zram 重新压缩的页数：

```
	echo "type=huge_idle priority=1 max_pages=42" > \
		/sys/block/zramX/recompress
```

建议始终指定 `priority` 参数。虽然也可以指定 `algo` 参数，让 `zram` 通过
算法名称来确定优先级，但并不推荐这样做，因为当同一算法以不同优先级配置时
（例如不同参数）可能导致意想不到的结果。`priority` 是保证使用预期算法的
唯一方式。

## 内存跟踪


通过 CONFIG_ZRAM_MEMORY_TRACKING，用户可以了解 zram 块的信息。它对于通过
*pagemap 捕获进程的冷页或不可压缩页可能很有用。

如果启用该功能，你可以通过如下方式查看块状态

```
	  300    75.033841 .wh...
	  301    63.806904 s.....
	  302    63.806919 ..hi..
	  303    62.801919 ....r.
	  304   146.781902 ..hi.n
```

第一列
	zram 的块索引。
第二列
	自系统启动以来的访问时间
第三列
	块的状态：

	s:
		相同页
	w:
		已写入后端存储的页
	h:
		huge 页
	i:
		idle 页
	r:
		已重新压缩的页（次级压缩算法）
	n:
		没有任何（包括次级的）算法能够压缩它

上述示例的第一行表示第 300 个块在 75.033841 秒时被访问，且该块的状态为
huge，因此它被写回后端存储。这是一个调试功能，任何人都不要依赖它能正常
工作。

Nitin Gupta
ngupta@vflare.org
