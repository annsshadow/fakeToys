## slab 分配器简明用户指南


slab 分配器包含完整的调试支持（在构建时启用 CONFIG_SLUB_DEBUG=y），但默认关闭（除非
构建时启用了 CONFIG_SLUB_DEBUG_ON=y）。你可以仅为选定的 slab 启用调试，以避免对整体系统
性能造成影响，否则可能使 bug 更难发现。

要开启调试，可以向内核命令行添加 `slab_debug` 选项。这将为所有 slab 启用完整调试。

通常随后会使用 `slabinfo` 命令获取统计数据并对 slab 执行操作。默认情况下 `slabinfo` 只
列出其中包含数据的 slab。运行命令时请参见 "slabinfo -h" 以了解更多选项。`slabinfo` 可
通过以下方式编译：

```

	gcc -o slabinfo tools/mm/slabinfo.c

```
`slabinfo` 的某些操作模式要求在内核命令行上启用 slub 调试。例如，在未开启调试时不会有
跟踪信息可用，并且如果未开启调试，验证只能部分执行。

### slab_debug 的一些更高级用法：


可以向 `slab_debug` 提供参数。如果未指定任何参数，则启用完整调试。格式：

slab_debug=<Debug-Options>
	为所有 slab 启用选项

slab_debug=<Debug-Options>,<slab name1>,<slab name2>,...
	仅为选定的 slab 启用选项（逗号后无空格）

可以给出针对所有 slab 或选定 slab 的多个选项块，选项块之间用 ';' 分隔。最后一个“所有
slab”块应用于除匹配某个“选定 slab”块之外的所有 slab。匹配 slab 名称的第一个“选定 slab”
块的选项会被应用。

```

	F		Sanity checks on (enables SLAB_DEBUG_CONSISTENCY_CHECKS
			Sorry SLAB legacy issues)
	Z		Red zoning
	P		Poisoning (object and padding)
	U		User tracking (free and alloc)
	T		Trace (please only use on single slabs)
	A		Enable failslab filter mark for the cache
	O		Switch debugging off for caches that would have
			caused higher minimum slab orders
	-		Switch all debugging off (useful if the kernel is
			configured with CONFIG_SLUB_DEBUG_ON)

```
```

	slab_debug=FZ

```
```

	slab_debug=,dentry

```
以仅在 dentry 缓存上启用调试。你可以在 slab 名称末尾使用星号，以覆盖所有具有相同前缀的
slab。例如，以下是如何对 dentry 缓存以及所有 kmalloc 进行毒化

```

	slab_debug=P,kmalloc-*,dentry

```
Red zoning 和跟踪可能会重新对齐 slab。我们可以只应用健全性检查

```

	slab_debug=F,dentry

```
调试选项可能会因为存储元数据（例如，对象大小为 PAGE_SIZE 的缓存）而要求最小可能的 slab
阶数增加。这在低内存情况或内存高度碎片化时更有可能导致 slab 分配错误。为了

```

	slab_debug=O

```
你可以使用选项块将不同选项应用于不同的 slab 名称列表。这将为 dentry 启用 red zoning，并为

```

	slab_debug=Z,dentry;U,kmalloc-*

```
你也可以通过指定全局调试选项后跟一个 slab 名称列表，为除某些被认为对性能过于关键、不需要
调试的缓存之外的所有缓存启用选项（例如健全性检查和毒化）

```

	slab_debug=FZ;-,zs_handle,zspage

```
slab 每个调试选项的状态可以在相应的文件中找到

```

	/sys/kernel/slab/<slab name>/

```
如果文件包含 1，则该选项已启用，0 表示已禁用。debug

```

	F	sanity_checks
	Z	red_zone
	P	poison
	U	store_user
	T	trace
	A	failslab

```
failslab 文件是可写的，因此写入 1 或 0 将在运行时启用或禁用该选项。如果缓存是别名，写入
返回 -EINVAL。使用跟踪时要小心：它可能会输出大量信息，如果在错误的 slab 上使用则永远不会
停止。

## Slab 合并


如果未指定调试选项，则 SLUB 可能会将相似的 slab 合并在一起，以减少开销并提高对象的缓存
热度。`slabinfo -a` 显示哪些 slab 被合并在一起。

## Slab 验证


如果内核以 slab_debug 启动，SLUB 可以验证所有对象。为此，你必须拥有 `slabinfo` 工具。
然后你可以执行

```

	slabinfo -v

```
这将测试所有对象。输出将生成到 syslog。

如果启动时未启用 slab 调试，这也以更受限的方式工作。在这种情况下，`slabinfo -v` 只测试
所有可达对象。通常这些对象位于 cpu slab 和部分 slab 中。在非调试情况下，SLUB 不会跟踪
完整 slab。

## 获取更高性能


在某种程度上，SLUB 的性能受到需要偶尔获取 list_lock 来处理部分 slab 的限制。该开销由每个
slab 的分配阶数决定。分配可以受内核参数影响：

`slab_min_objects`
	允许指定为了使分配阶数可接受，一个 slab 中至少必须容纳多少个对象。一般来说，slub
	将能够在一个 slab 上执行此数量的分配，而无需咨询可能发生争用的集中式资源
	（list_lock）。

`slab_min_order`
	指定 slab 的最小阶数。与 `slab_min_objects` 效果类似。

`slab_max_order`
	指定不再检查 `slab_min_objects` 的阶数。这用于避免 SLUB 尝试生成超大阶数的页面，将
	具有大对象大小的 slab 缓存的 `slab_min_objects` 塞入一个高阶页面。设置内核命令行参数
	`debug_guardpage_minorder=N`（N > 0）会强制将 `slab_max_order` 设为 0，从而使 slab
	分配使用最小可能的阶数。

`slab_strict_numa`
        启用在每个分配上应用内存策略。这会使对象放置更精确，从而可能减少对远程节点的访问。
        默认情况下，仅当获取新 folio 或从列表中取回 folio 时，才在 folio 级别应用内存策略。
        启用此选项会降低 slab 分配器的快速路径性能。

## SLUB 调试输出


```

 ====================================================================
 BUG kmalloc-8: Right Redzone overwritten
 --------------------------------------------------------------------

 INFO: 0xc90f6d28-0xc90f6d2b. First byte 0x00 instead of 0xcc
 INFO: Slab 0xc528c530 flags=0x400000c3 inuse=61 fp=0xc90f6d58
 INFO: Object 0xc90f6d20 @offset=3360 fp=0xc90f6d58
 INFO: Allocated in get_modalias+0x61/0xf5 age=53 cpu=1 pid=554

 Bytes b4 (0xc90f6d10): 00 00 00 00 00 00 00 00 5a 5a 5a 5a 5a 5a 5a 5a ........ZZZZZZZZ
 Object   (0xc90f6d20): 31 30 31 39 2e 30 30 35                         1019.005
 Redzone  (0xc90f6d28): 00 cc cc cc                                     .
 Padding  (0xc90f6d50): 5a 5a 5a 5a 5a 5a 5a 5a                         ZZZZZZZZ

   [<c010523d>] dump_trace+0x63/0x1eb
   [<c01053df>] show_trace_log_lvl+0x1a/0x2f
   [<c010601d>] show_trace+0x12/0x14
   [<c0106035>] dump_stack+0x16/0x18
   [<c017e0fa>] object_err+0x143/0x14b
   [<c017e2cc>] check_object+0x66/0x234
   [<c017eb43>] __slab_free+0x239/0x384
   [<c017f446>] kfree+0xa6/0xc6
   [<c02e2335>] get_modalias+0xb9/0xf5
   [<c02e23b7>] dmi_dev_uevent+0x27/0x3c
   [<c027866a>] dev_uevent+0x1ad/0x1da
   [<c0205024>] kobject_uevent_env+0x20a/0x45b
   [<c020527f>] kobject_uevent+0xa/0xf
   [<c02779f1>] store_uevent+0x4f/0x58
   [<c027758e>] dev_attr_store+0x29/0x2f
   [<c01bec4f>] sysfs_write_file+0x16e/0x19c
   [<c0183ba7>] vfs_write+0xd1/0x15a
   [<c01841d7>] sys_write+0x3d/0x72
   [<c0104112>] sysenter_past_esp+0x5f/0x99
   [<b7f7b410>] 0xb7f7b410
   =======================

 FIX kmalloc-8: Restoring Redzone 0xc90f6d28-0xc90f6d2b=0xcc

```
如果 SLUB 遇到损坏的对象（完整检测需要内核以 slab_debug 启动），则会向 syslog 转储以下
输出：

1. 所遇到问题的描述

```

     ===============================================
     BUG <slab cache affected>: <What went wrong>
     -----------------------------------------------

     INFO: <corruption start>-<corruption end> <more info>
     INFO: Slab <address> <slab information>
     INFO: Object <address> <object information>
     INFO: Allocated in <kernel function> age=<jiffies since alloc> cpu=<allocated by
	cpu> pid=<pid of the process>
     INFO: Freed in <kernel function> age=<jiffies since free> cpu=<freed by cpu>
	pid=<pid of the process>

   (Object allocation / free information is only available if SLAB_STORE_USER is
   set for the slab. slab_debug sets that option)

```
2. 如果涉及对象，则包括对象内容。

   BUG SLUB 行之后可能出现各种类型的行：

   Bytes b4 <address> : <bytes>
	显示在检测到问题的对象之前的几个字节。如果损坏并未在对象起始处停止，这会很有用。

   Object <address> : <bytes>
	对象的字节。如果对象未激活，则字节通常包含毒化值。任何非毒化值都表明存在释放后写入
	造成的损坏。

   Redzone <address> : <bytes>
	对象之后的 Redzone。Redzone 用于检测对象之后的写入。所有字节应始终具有相同的值。如果
	有任何偏差，则是由对象边界之外的写入造成的。

	（Redzone 信息仅在设置了 SLAB_RED_ZONE 时可用。slab_debug 会设置该选项）

   Padding <address> : <bytes>
	用于填充空间以使下一个对象正确对齐的未使用数据。在调试情况下，我们确保至少有 4 字节
	的填充。这允许检测对象之前的写入。

3. 栈转储

   栈转储描述了检测到错误的位置。通过查看分配或释放该对象的函数，更有可能找到损坏的原因。

4. 关于如何处理该问题以确保系统持续运行的报告。

```

	FIX <slab cache affected>: <corrective action taken>

   In the above sample SLUB found that the Redzone of an active object has
   been overwritten. Here a string of 8 characters was written into a slab that
   has the length of 8 characters. However, a 8 character string needs a
   terminating 0. That zero has overwritten the first byte of the Redzone field.
   After reporting the details of the issue encountered the FIX SLUB message
   tells us that SLUB has restored the Redzone to its proper value and then
   system operations continue.

```

## 紧急操作


```

	slab_debug=F

```
这通常足以启用 slub 的弹性特性，即使有糟糕的内核组件不断损坏对象，也能保持系统运行。这
对于生产系统可能很重要。性能会受到健全性检查的影响，并且会持续向 syslog 输出错误消息流，
但不会使用额外的内存（与完整调试不同）。

不提供任何保证。内核组件仍然需要修复。通过定位发生损坏的 slab 并仅为该缓存启用调试，可以
进一步优化性能

```

	slab_debug=F,dentry

```
如果损坏是通过在对象末尾之后写入造成的，那么建议启用 Redzone 以避免损坏开头

```

	slab_debug=FZ,dentry

```

## 扩展 slabinfo 模式与绘图


`slabinfo` 工具有一个特殊的“扩展”（'-X'）模式，包括：
 - 缓存总计
 - 按大小排序的 slab（最多 -N <num> 个 slab，默认 1）
 - 按损耗排序的 slab（最多 -N <num> 个 slab，默认 1）

此外，在此模式下 `slabinfo` 不会动态缩放大小（G/M/K），而是以字节为单位报告所有内容（此
功能也可通过 '-B' 选项用于其他 slabinfo 模式），这使得报告更精确。而且，在某种意义上，
`-X` 模式也简化了 slab 行为的分析，因为其输出可以使用 `slabinfo-gnuplot.sh` 脚本绘制成
图。因此它将分析从查看数字（大量数字）推向更轻松的方式——可视化分析。

生成绘图：

```

	while [ 1 ]; do slabinfo -X >> FOO_STATS; sleep 1; done

```
```

	slabinfo-gnuplot.sh FOO_STATS [FOO_STATS2 .. FOO_STATSN]

   The ``slabinfo-gnuplot.sh`` script will pre-processes the collected records
   and generates 3 png files (and 3 pre-processing cache files) per STATS
   file:
   - Slabcache Totals: FOO_STATS-totals.png
   - Slabs sorted by size: FOO_STATS-slabs-by-size.png
   - Slabs sorted by loss: FOO_STATS-slabs-by-loss.png

```
`slabinfo-gnuplot.sh` 有用的另一个用例是，当你需要比较某些代码修改“之前”和“之后”的 slab
行为时。为此，`slabinfo-gnuplot.sh` 脚本可以“合并”来自不同测量的 `Slabcache Totals`
部分。要可视化比较 N 个绘图：

```

	while [ 1 ]; do slabinfo -X >> STATS<X>; sleep 1; done

```
```

	slabinfo-gnuplot.sh STATS1 STATS2 .. STATSN

```
c) 在 '-t' 模式下执行 `slabinfo-gnuplot.sh`，传入所有的

```

	slabinfo-gnuplot.sh -t STATS1-totals STATS2-totals .. STATSN-totals

   This will produce a single plot (png file).

   Plots, expectedly, can be large so some fluctuations or small spikes
   can go unnoticed. To deal with that, ``slabinfo-gnuplot.sh`` has two
   options to 'zoom-in'/'zoom-out':

   a) ``-s %d,%d`` -- 覆盖默认的图像宽度和高度
   b) ``-r %d,%d`` -- 指定要使用的样本范围（例如，在 ``slabinfo -X >> FOO_STATS; sleep 1;``
      的情况下，使用 ``-r 40,60`` 范围将只绘制在第 40 到第 60 秒之间收集的样本）。


```

## SLUB 的 DebugFS 文件


有关启用了用户跟踪调试选项的 SLUB 缓存当前状态的更多信息，可通过 debugfs 文件获取，通常
位于 /sys/kernel/debug/slab/<cache>/ 下（仅为启用了用户跟踪的缓存创建）。这些文件有 2 种
类型，包含以下调试信息：

```

    Prints information about unique allocation traces of the currently
    allocated objects. The output is sorted by frequency of each trace.

    Information in the output:
    Number of objects, allocating function, possible memory wastage of
    kmalloc objects(total/per-object), minimal/average/maximal jiffies
    since alloc, pid range of the allocating processes, cpu mask of
    allocating cpus, numa node mask of origins of memory, and stack trace.

    Example:::

    338 pci_alloc_dev+0x2c/0xa0 waste=521872/1544 age=290837/291891/293509 pid=1 cpus=106 nodes=0-1
        __kmem_cache_alloc_node+0x11f/0x4e0
        kmalloc_trace+0x26/0xa0
        pci_alloc_dev+0x2c/0xa0
        pci_scan_single_device+0xd2/0x150
        pci_scan_slot+0xf7/0x2d0
        pci_scan_child_bus_extend+0x4e/0x360
        acpi_pci_root_create+0x32e/0x3b0
        pci_acpi_scan_root+0x2b9/0x2d0
        acpi_pci_root_add.cold.11+0x110/0xb0a
        acpi_bus_attach+0x262/0x3f0
        device_for_each_child+0xb7/0x110
        acpi_dev_for_each_child+0x77/0xa0
        acpi_bus_attach+0x108/0x3f0
        device_for_each_child+0xb7/0x110
        acpi_dev_for_each_child+0x77/0xa0
        acpi_bus_attach+0x108/0x3f0

```
```

    Prints information about unique freeing traces of the currently allocated
    objects. The freeing traces thus come from the previous life-cycle of the
    objects and are reported as not available for objects allocated for the first
    time. The output is sorted by frequency of each trace.

    Information in the output:
    Number of objects, freeing function, minimal/average/maximal jiffies since free,
    pid range of the freeing processes, cpu mask of freeing cpus, and stack trace.

    Example:::

    1980 <not-available> age=4294912290 pid=0 cpus=0
    51 acpi_ut_update_ref_count+0x6a6/0x782 age=236886/237027/237772 pid=1 cpus=1
	kfree+0x2db/0x420
	acpi_ut_update_ref_count+0x6a6/0x782
	acpi_ut_update_object_reference+0x1ad/0x234
	acpi_ut_update_ref_count+0x6a6/0x782
	acpi_ut_remove_reference+0x7d/0x84
	acpi_rs_get_prt_method_data+0x97/0xd6
	acpi_get_irq_routing_table+0x82/0xc4
	acpi_pci_irq_find_prt_entry+0x8e/0x2e0
	acpi_pci_irq_lookup+0x3a/0x1e0
	acpi_pci_irq_enable+0x77/0x240
	pcibios_enable_device+0x39/0x40
	do_pci_enable_device.part.0+0x5d/0xe0
	pci_enable_device_flags+0xfc/0x120
	pci_enable_device+0x13/0x20
	virtio_pci_probe+0x9e/0x170
	local_pci_probe+0x48/0x80
	pci_device_probe+0x105/0x1c0

```
Christoph Lameter, May 30, 2007
Sergey Senozhatsky, October 23, 2015
