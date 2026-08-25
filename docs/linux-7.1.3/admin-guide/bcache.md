## A block layer cache (bcache，块层缓


假设您有一个大而慢raid 6，以及一块或三块 ssd。如果能把它们用作缓存岂不美哉……于是有bcache

bcache wiki 位于
  https://bcache.evilpiepirate.org

这是 bcache-tools git 仓库
  https://git.kernel.org/pub/scm/linux/kernel/git/colyli/bcache-tools.git/

最新的 bcache 内核代码可在主线 Linux 内核中找到：
  https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/

它的设计围绕 SSD 的性能特征展开——它只在擦除块（erase block）大小的桶（bucket）中分配，并使用混合btree/日志来跟踪缓存的区段（extent）（区段大小可从单个扇区到桶大小不等）。它不遗余力地避免随机写

write-through（透写）和 writeback（回写）缓存都受支持。writeback 默认关闭，但可以在运行时任意开启或关闭。bcache 竭力保护您的数据——它能可靠地处理非正常关机。（它甚至没有“干净关机”的概念；bcache 只有在写入到达稳定存储后才会将写操作返回为已完成）

writeback 缓存可以使用大部分缓存来缓冲写操作——将脏数据写backing 设备始终是顺序进行的，从索引的起始扫描到末尾

由于随机 IO 正是 SSD 所擅长的，缓存大的顺序 IO 通常好处不大。bcache 检测顺IO 并跳过它；它还对每个任务IO 大小保持滚动平均，只要平均值高cutoff 就会跳过该任务的所IO——而不是在每次 seek 后缓存前 512k。因此备份和大文件复制应当完全绕过缓存

若闪存上发生数据 IO 错误，bcache 会尝试通过从磁盘读取或使缓存条目失效来恢复。对于不可恢复的错误（元数据或脏数据），缓存会自动禁用；若缓存中存在脏数据，它会先禁writeback 缓存并等待所有脏数据被刷出

Getting started（入门）
您将需要来bcache-tools 仓库bcache 工具。cache 设备
```
  bcache make -B /dev/sdb
  bcache make -C /dev/sdc
```
`bcache make` 能够同时格式化多个设备——如果您同时格式backing 设备cache 设备，就不会
```
  bcache make -B /dev/sda /dev/sdb -C /dev/sdc
```
如果您的 bcache-tools 未更新到最新版本且不具有统一`bcache` 工具，您可以使用旧的 `make-bcache` 工具，以相同-B -C 参数格式bcache 设备

bcache-tools 现在附带 udev 规则，bcache 设备为内核所
```
  echo /dev/sdb > /sys/fs/bcache/register
  echo /dev/sdc > /sys/fs/bcache/register
```
注册 backing 设备会使 bcache 设备出现/dev 中；您现在可以像平常一样格式化并使用它。但首次使用新的 bcache 设备时，在将attach 到缓存之前，它将运行passthrough（直通）模式。如果您打算稍后使用 bcache，建议将所有慢速设备都设为不带缓存bcache backing 设备，之后您可以选择添加缓存设备
参见下文的“ATTACHING”章节

```
  /dev/bcache<N>
```
```
  /dev/bcache/by-uuid/<uuid>
  /dev/bcache/by-label/<label>
```
```
  mkfs.ext4 /dev/bcache0
  mount /dev/bcache0 /mnt
```
您可以通过 sysfs /sys/block/bcache<N>/bcache 控制 bcache 设备。您也可以通过 /sys/fs//bcache/<cset-uuid>/ 控制它们

Cache 设备以集合（set）形式管理；每个集合目前还不支持多个缓存，但未来将允许元数据和脏数据的镜像。您的新缓存集合显示/sys/fs/bcache/<UUID>

### Attaching（附绑定


在您cache 设备backing 设备注册后，必须backing 设备 attach 到缓存集合以启用缓存。将 backing 设备 attach 到缓存集合的操作如下，使用缓存集合的 UUID 写入
```
  echo <CSET-UUID> > /sys/block/bcache0/bcache/attach
```
这只需做一次。下次重启时，只需重新注册您的所bcache 设备。如果某backing 设备在某个缓存中有数据，/dev/bcache<N> 设备要等到缓存出现后才会被创建——如果您开启了 writeback 缓存，这一点尤为重要

如果您在启动时缓存设备丢失且再也不会回来，您
```
  echo 1 > /sys/block/sdb/bcache/running
```
（您需要使/sys/block/sdb（或您的 backing 设备叫什么），而不/sys/block/bcache0，因bcache0 尚不存在。如果您使用的是分区，bcache 目录将位/sys/block/sdb/sdb2/bcache

backing 设备若将来出现仍会使用那个缓存集合，但所有缓存数据都会被失效。如果缓存中有脏数据，不要指望文件系统可恢复——您将面临大规模的文件系统损坏，尽管 ext4 fsck 确实能创造奇迹

### Error Handling（错误处理）


bcache 尝试透明地处理进出缓存设备的 IO 错误，而不影响正常操作；如果它看到过多错误（阈值是可配置的，默认为 0），它会关闭缓存设备并将所backing 设备切换passthrough 模式

 - 对于来自缓存的读，若出错，我们只是从 backing 设备重试该读

 - 对于 write-through 写，若对缓存的写出错，我们只是切换到使缓存中lba 的数据失效（即，与绕过缓存的写所做的相同）

 - 对于 writeback 写，我们目前将该错误传回给文件系用户空间。这可以得到改进——我们可以将其作为跳过缓存的写来重试，从而不必使该写出错

 - 当我detach 时，我们首先尝试刷出任何脏数据（如果我们运行writeback 模式）。不过，如果某些脏数据读取失败，它目前不会做任何智能处理

### Howto/cookbook（操作指秘籍


A) 使用缺失的缓存设备启bcache

如果注册 backing 设备没有帮助，说明它已经存在，您只需
```
	host:~# echo /dev/sdb1 > /sys/fs/bcache/register
	[  119.844831] bcache: register_bcache() error opening /dev/sdb1: device already registered

```
接下来，如果缓存设备存在，您尝试注册它。但如果它缺失，或因某种原因注册失败，您仍然可以
```
	host:/sys/block/sdb/sdb1/bcache# echo 1 > running

```
注意，如果您运行writeback 模式，这可能会导致数据丢失

```
	host:/sys/block/md5/bcache# echo 0226553a-37cf-41d5-b3ce-8b1e944543a8 > attach
	[ 1933.455082] bcache: bch_cached_dev_attach() Couldn't find uuid for md5 in set
	[ 1933.478179] bcache: __cached_dev_store() Can't attach 0226553a-37cf-41d5-b3ce-8b1e944543a8
	[ 1933.478179] : cache set not found

```
在这种情况下，缓存设备只是在启动时未注册
```
	host:/sys/block/md5/bcache# echo /dev/sdh2 > /sys/fs/bcache/register


```
C) 损坏bcache 在设备注册时导致内核崩溃

这绝不应该发生。如果确实发生了，那么您发现了一bug
请将其报告给 bcache 开发邮件列表：linux-bcache@vger.kernel.org

请务必提供尽可能多的信息，包括内dmesg 输出（如果可得），以便我们提供帮助


D) 在没bcache 的情况下恢复数据

如果内核中没bcache，backing 设备上的文件系统仍然位于 8KiB 偏移处可用。因此，可以通过--offset 8K 创建backing 设备loopdev，或者通过您最初用 `bcache make` 格式bcache 时由 --data-offset 定义的任何值来访问

```
	losetup -o 8192 /dev/loop0 /dev/your_bcache_backing_dev

```
这将/dev/loop0 中呈现您未修改的 backing 设备数据

如果您的缓存处于 write-through 模式，那么您可以安全地丢弃缓存设备而不丢失数据


E) 擦除缓存设备

```
	host:~# wipefs -a /dev/sdh2
	16 bytes were erased at offset 0x1018 (bcache)
	they were: c6 85 73 f6 4e 1a 45 ca 82 65 f5 7f 48 ba 6d 81

```
```
	host:~# bcache make -C /dev/sdh2
	UUID:                   7be7e175-8f4c-4f99-94b2-9c904d227045
	Set UUID:               5bc072a8-ab17-446d-9744-e247949913c1
	version:                0
	nbuckets:               106874
	block_size:             1
	bucket_size:            1024
	nr_in_set:              1
	nr_this_dev:            0
	first_bucket:           1
	[  650.511912] bcache: run_cache_set() invalidating existing data
	[  650.549228] bcache: register_cache() registered cache device sdh2

```
```
	host:/sys/block/md5/bcache# echo 1 > running

```
```
	host:/sys/block/md5/bcache# echo 5bc072a8-ab17-446d-9744-e247949913c1 > attach
	[  865.276616] bcache: bch_cached_dev_attach() Caching md5 as bcache0 on set 5bc072a8-ab17-446d-9744-e247949913c1


```
```
	host:/sys/block/sda/sda7/bcache# echo 1 > detach
	[  695.872542] bcache: cached_dev_detach_finish() Caching disabled for sda7

	host:~# wipefs -a /dev/nvme0n1p4
	wipefs: error: /dev/nvme0n1p4: probing initialization failed: Device or resource busy
	Ooops, it's disabled, but not unregistered, so it's still protected

```
```
	host:/sys/fs/bcache/b7ba27a1-2398-4649-8ae3-0959f57ba128# ls -l cache0
	lrwxrwxrwx 1 root root 0 Feb 25 18:33 cache0 -> ../../../devices/pci0000:00/0000:00:1d.0/0000:70:00.0/nvme/nvme0/nvme0n1/nvme0n1p4/bcache/
	host:/sys/fs/bcache/b7ba27a1-2398-4649-8ae3-0959f57ba128# echo 1 > stop
	kernel: [  917.041908] bcache: cache_set_free() Cache set b7ba27a1-2398-4649-8ae3-0959f57ba128 unregistered

```
```
	host:~# wipefs -a /dev/nvme0n1p4
	/dev/nvme0n1p4: 16 bytes were erased at offset 0x00001018 (bcache): c6 85 73 f6 4e 1a 45 ca 82 65 f5 7f 48 ba 6d 81


```
G) dm-crypt 涓?bcache

首先设置未加密的 bcache，然后在 /dev/bcache<N> 之上安装 dmcrypt。这比同dmcrypt 加密 backing caching 设备再在其上安装 bcache 要快。[benchmarks]


H) 停止/释放已注册的 bcache 以擦除和/或重建它

假设您需要释放所bcache 引用，以便运fdisk 并重新注册已更改的分区表，而只要上面还有任何活跃的 backing caching 设备，这就无法工作：

1) 它是否出现在 /dev/bcache* 中？（有时它不会

```
	host:/sys/block/bcache0/bcache# echo 1 > stop

```
```
	host:/sys/block/bcache0# cd bcache
	bash: cd: bcache: No such file or directory

   In this case, you may have to unregister the dmcrypt block device that
   references this bcache to free it up::

	host:~# dmsetup remove oldds1
	bcache: bcache_device_free() bcache0 stopped
	bcache: cache_set_free() Cache set 5bc072a8-ab17-446d-9744-e247949913c1 unregistered

   This causes the backing bcache to be removed from /sys/fs/bcache and
   then it can be reused.  This would be true of any block device stacking
   where bcache is a lower device.

```
```
	host:/sys/fs/bcache# ls -l */{cache?,bdev?}
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 0226553a-37cf-41d5-b3ce-8b1e944543a8/bdev1 -> ../../../devices/virtual/block/dm-1/bcache/
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 0226553a-37cf-41d5-b3ce-8b1e944543a8/cache0 -> ../../../devices/virtual/block/dm-4/bcache/
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 5bc072a8-ab17-446d-9744-e247949913c1/cache0 -> ../../../devices/pci0000:00/0000:00:01.0/0000:01:00.0/ata10/host9/target9:0:0/9:0:0:0/block/sdl/sdl2/bcache/

   The device names will show which UUID is relevant, cd in that directory
   and stop the cache::

	host:/sys/fs/bcache/5bc072a8-ab17-446d-9744-e247949913c1# echo 1 > stop

   This will free up bcache references and let you reuse the partition for
   other purposes.

```
### Troubleshooting performance（排查性能问题


Bcache 有一堆配置选项和可调参数。默认值旨在对典型的桌面和服务器工作负载合理，但在基准测试时想获得尽可能好的数字，它们并不是您想要的

 - Backing device alignment（backing 设备对齐

   bcache 中，默认的元数据大小8k。如果您backing 设备基于 RAID，那么务必使`bcache make --data-offset` stride 宽度的倍数对齐。如果您打算将来扩展磁盘阵列，则将一系列素数乘以您的 raid stripe 大小，以获得您想要的磁盘倍数

   例如：如果您64k stripe 大小，那么以下偏移量

```

	64k * 2*2*2*3*3*5*7 bytes = 161280k

   That space is wasted, but for only 157.5MB you can grow your RAID 5
   volume to the following data-spindle counts without re-aligning::

	3,4,5,6,7,8,9,10,12,14,15,18,20,21 ...

 - Bad write performance

   If write performance is not what you expected, you probably wanted to be
   running in writeback mode, which isn't the default (not due to a lack of
   maturity, but simply because in writeback mode you'll lose data if something
   happens to your SSD)::

	# echo writeback > /sys/block/bcache0/bcache/cache_mode

 - Bad performance, or traffic not going to the SSD that you'd expect

   By default, bcache doesn't cache everything. It tries to skip sequential IO -
   because you really want to be caching the random IO, and if you copy a 10
   gigabyte file you probably don't want that pushing 10 gigabytes of randomly
   accessed data out of your cache.

   But if you want to benchmark reads from cache, and you start out with fio
   writing an 8 gigabyte test file - so you want to disable that::

	# echo 0 > /sys/block/bcache0/bcache/sequential_cutoff

   To set it back to the default (4 mb), do::

	# echo 4M > /sys/block/bcache0/bcache/sequential_cutoff

 - Traffic's still going to the spindle/still getting cache misses

   In the real world, SSDs don't always keep up with disks - particularly with
   slower SSDs, many disks being cached by one SSD, or mostly sequential IO. So
   you want to avoid being bottlenecked by the SSD and having it slow everything
   down.

   To avoid that bcache tracks latency to the cache device, and gradually
   throttles traffic if the latency exceeds a threshold (it does this by
   cranking down the sequential bypass).

   You can disable this if you need to by setting the thresholds to 0::

	# echo 0 > /sys/fs/bcache/<cache set>/congested_read_threshold_us
	# echo 0 > /sys/fs/bcache/<cache set>/congested_write_threshold_us

   The default is 2000 us (2 milliseconds) for reads, and 20000 for writes.

 - Still getting cache misses, of the same data

   One last issue that sometimes trips people up is actually an old bug, due to
   the way cache coherency is handled for cache misses. If a btree node is full,
   a cache miss won't be able to insert a key for the new data and the data
   won't be written to the cache.

   In practice this isn't an issue because as soon as a write comes along it'll
   cause the btree node to be split, and you need almost no write traffic for
   this to not show up enough to be noticeable (especially since bcache's btree
   nodes are huge and index large regions of the device). But when you're
   benchmarking, if you're trying to warm the cache by reading a bunch of data
   and there's no other traffic - that can be a problem.

   Solution: warm the cache by doing writes, or use the testing branch (there's
   a fix for the issue there).


```

### Sysfs - backing device（Sysfs - backing 设备


位于 /sys/block/<bdev>/bcachesys/block/bcache*/bcache 以及（若attachsys/fs/bcache/<cset-uuid>/bdev*

attach
  将该缓存集合UUID 写入此文件以启用缓存

cache_mode
  可以writethrough、writeback、writearound none 之一

clear_stats
  写入此文件会重置累计统计（不是按小时/5 分钟的衰减版本）

detach
  写入此文件以从缓存集detach。如果缓存中有脏数据，会先将其刷出

dirty_data
  backing 设备在缓存中的脏数据量。与缓存集合的版本不同，它持续更新，但可能略有偏差

label
  底层设备的名称

readahead
  应执行的预读的字节数。默认为 0。若设为例如 1M，它会将缓存未命中的读向上取整到该大小，但不与现有缓存条目重叠

running
  如果 bcache 正在运行则为 1（即 /dev/bcache 设备是否存在，无论它处于 passthrough 模式还是缓存模式）

sequential_cutoff
  顺序 IO 一旦超过此阈值就会绕过缓存；会跟踪最128 IO，因此即使不是一次性完成的顺序 IO 也能被检测出来

sequential_merge
  若非零，bcache 保留最128 个请求的列表，与所有新请求比较，以确定哪些新请求是先前请求的顺序延续，从而决定顺cutoff。如果顺cutoff 值大于任何单个请求的最大可接受顺序大小，则这是必要的

state
  backing 设备可以处于以下四种状态之一

  no cache：从attach 到缓存集合

  clean：缓存集合的一部分，且没有缓存的脏数据

  dirty：缓存集合的一部分，且有缓存的脏数据

  inconsistent：当存在缓存的脏数据但缓存集合不可用时，用户强行运行backing 设备；backing 设备上的任何数据可能都已损坏

stop
  写入此文件以关闭 bcache 设备并关backing 设备

writeback_delay
  当脏数据写入缓存且其之前不包含任何脏数据时，会等待若干秒后再启动 writeback。默认为 30

writeback_percent
  若非零，bcache 尝试通过限制后台 writeback 并使PD 控制器平滑调整速率，将此百分比的缓存保持为脏

writeback_rate
  以每秒扇区数计的速率——若 writeback_percent 非零，后writeback 被限制到此速率。由 bcache 持续调整，但也可由用户设置

writeback_running
  若关闭，脏数据的 writeback 将完全不进行。脏数据仍会被加入缓存直到它几乎满；仅用于基准测试。默认为开启

#### Sysfs - backing device stats（Sysfs - backing 设备统计


存在带有这些数字的目录用于累计总数，以及过去一天、一小时5 分钟内衰减的版本；它们也在缓存集合目录中被聚合

bypassed
  绕过缓存IO 量（读和写都有）

cache_hits, cache_misses, cache_hit_ratio
  命中与未命中bcache 所见的每个独立 IO 计数；部分命中计为未命中

cache_bypass_hits, cache_bypass_misses
  针对本应跳过缓存IO 的命中与未命中仍会被计数，但在此单独列出

cache_miss_collisions
  计数数据本将从缓存未命中插入缓存，但与一次写竞争且数据已存在的情况（通常0，因为缓存未命中的同步已被重写）

#### Sysfs - cache set（Sysfs - 缓存集合


位于 /sys/fs/bcache/<cset-uuid>

average_key_size
  btree 中每个键的平均数据量

bdev<0..n>
  指向每个attach backing 设备的符号链接

block_size
  缓存设备的块大小

btree_cache_size
  btree 缓存当前使用的内存量

bucket_size
  桶的大小

cache<0..n>
  指向组成此缓存集合的每个缓存设备的符号链接

cache_available_percent
  不包含脏数据、可能用writeback 的缓存设备百分比。这并不意味此空间未被用于干净的缓存数据；未使用统计（priority_stats 中）通常低得多

clear_stats
  清除与此缓存相关的统

dirty_data
  缓存中的脏数据量（在垃圾回收运行时更新）

flash_vol_create
  将大小（以人类可读单k/M/G 回显）写入此文件，会创建一个由缓存集合支撑的精简配置卷

io_error_halflife, io_error_limit
  这些决定我们在禁用缓存之前接受多少错误。每个错误按半衰期（IO 数计）衰减。如果衰减计数达io_error_limit，脏数据会被写出且缓存被禁用

journal_delay_ms
  日志写会延迟至多这些毫秒，除非缓存刷新发生得更早。默认为 100

root_usage_percent
  btree 节点的使用百分比。如果过高，节点会拆分，增加树的深度

stop
  写入此文件以关闭缓存集合——等待所有已 attach backing 设备都被关闭

tree_depth
  btree 的深度（单节btree 深度0）

unregister
  分离所backing 设备并关闭缓存设备；如果存在脏数据，它会禁用 writeback 缓存并等待其被刷出

#### Sysfs - cache set internal（Sysfs - 缓存集合内部


此目录还暴露了许多内部操作的计时，分别有平均时长、平均频率、最近发生和最大时长的文件：垃圾回收、btree 读、btree 节点排序btree 拆分

active_journal_entries
  比索引更新的日志条目数

btree_nodes
  btree 中的节点总数

btree_used_percent
  btree 平均使用比例

bset_tree_stats
  关于辅助搜索树的统计

btree_cache_max_chain
  btree 节点缓存的哈希表中最长的

cache_read_races
  计数在从缓存读取数据期间，桶被重用并失效的情况——即读取完成后指针已失效。发生此情况时，数据会从 backing 设备重新读取

trigger_gc
  写入此文件会强制运行垃圾回收

#### Sysfs - Cache device（Sysfs - 缓存设备


位于 /sys/block/<cdev>/bcache

block_size
  写操作的最小粒度——应与硬件扇区大小匹配

btree_written
  所btree 写的总和，以（千/吉）字节

bucket_size
  桶的大小

cache_replacement_policy
  lru、fifo random 之一

freelist_percent
  空闲列表大小nbuckets 的百分比。可写入以增加空闲列表上保留的桶数，从而让您在运行时人为减小缓存大小。主要用于测试目的（即测试不同大小的缓存如何影响您的命中率）

io_errors
  已发生的错误数，io_error_halflife 衰减

metadata_written
  所有非数据写的总和（btree 写和所有其他元数据）

nbuckets
  此缓存中的桶总数

priority_stats
  关于缓存中数据最近被访问情况的统计。这可以揭示您的工作集大小。Unused 是不包含任何数据的缓存的百分比。Metadata bcache 的元数据开销。Average 是缓存桶的平均优先级。Next 是一个带有每个优先级阈值的分位数列表

written
  已写入缓存的所有数据的总和；与 btree_written 比较可得 bcache 中的写膨胀量

