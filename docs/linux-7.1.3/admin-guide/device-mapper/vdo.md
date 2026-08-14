
## dm-vdo


dm-vdo（虚拟数据优化器）设备映射器目标提供块级去重、压缩和精简配置。作为设备映射器目标，它可以将这些特性添加到存储栈中，并与任何文件系统兼容。vdo 目标不防护数据损坏，而是依赖其下方存储的完整性保护。强烈建议使用 lvm 来管理 vdo 卷。参见 lvmvdo(7)。

## 用户空间组件


格式化一个 vdo 卷需要使用 'vdoformat' 工具，它可在以下位置获取：

https://github.com/dm-vdo/vdo/

在大多数情况下，vdo 目标会在下次启动时自动从崩溃中恢复。如果它遇到了不可恢复的错误（无论是在正常运行期间还是崩溃恢复期间），目标将进入或以只读模式启动。由于只读模式是数据丢失的迹象，必须采取主动操作才能让 vdo 退出只读模式。来自同一代码仓库的 'vdoforcerebuild' 工具用于将只读的 vdo 准备好以退出只读模式。运行此工具后，vdo 目标会在下次启动时重建其元数据。尽管可能会丢失一些数据，但重建后的 vdo 元数据在内部是一致的，并且目标将再次可写。

该代码仓库还包含额外的用户空间工具，可用于检查 vdo 目标的磁盘上元数据。幸运的是，除了 dm-vdo 开发者之外，很少需要这些工具。

## 元数据需求


每个 vdo 卷预留 3GB 空间用于元数据，或根据配置预留更多。检查去重和压缩所节省的空间不会被元数据需求抵消是有帮助的。对于特定数据集所节省空间的估计可以通过 vdo 估算工具计算，该工具位于：

https://github.com/dm-vdo/vdoestimator/

## 目标接口


### 表行（Table line）


```

	<offset> <logical device size> vdo V4 <storage device>
	<storage device size> <minimum I/O size> <block map cache size>
	<block map era length> [optional arguments]


```
必需参数：

	offset（偏移）:
		 vdo 卷逻辑空间开始的偏移，以扇区为单位。

	logical device size（逻辑设备大小）:
		vdo 卷所服务的设备大小，以扇区为单位。必须与 vdo
		卷的当前逻辑大小匹配。

	storage device（存储设备）:
		持有 vdo 卷数据和元数据的设备。

	storage device size（存储设备大小）:
		持有 vdo 卷的设备大小，以 4096 字节块的数量表示。
		必须与 vdo 卷的当前大小匹配。

	minimum I/O size（最小 I/O 大小）:
		此 vdo 卷接受的最小 I/O 大小，以字节为单位。
		有效值为 512 或 4096。推荐值为 4096。

	block map cache size（块映射缓存大小）:
		块映射缓存的大小，以 4096 字节块的数量表示。
		最小和推荐值为 32768 块。如果逻辑线程数非零，
		则缓存大小必须至少为每逻辑线程 4096 块。

	block map era length（块映射纪元长度）:
		块映射缓存写出已修改块映射页的速度。较小的纪元长度
		可能会减少重建所花费的时间，代价是在正常运行期间
		增加块映射写入。最大值和推荐值为 16380；最小值为 1。

### 可选参数（Optional parameters）:


这些参数中的部分或全部可以作为 <key> <value> 对来指定。

与线程相关的参数：

不同类别的工作被分配到独立的线程组，每个组中的线程数可以分别配置。

如果 <hash>、<logical> 和 <physical> 都设为 0，则这三类线程处理的工作都将由单个线程处理。如果其中任何值非零，则所有值都必须非零。

	ack（确认）:
		用于完成 bio 的线程数。由于完成 bio 会调用 vdo 卷
		外部的任意完成函数，此类线程允许 vdo 卷在 bio 完成
		较慢时继续处理请求。默认值为 1。

	bio:
		用于向底层存储发出 bio 的线程数。此类线程允许 vdo 卷
		在 bio 提交较慢时继续处理请求。默认值为 4。

	bioRotationInterval（bio 轮转间隔）:
		在切换到下一个 bio 线程之前，每个 bio 线程上入队的
		bio 数量。该值必须大于 0 且不超过 1024；默认值为 64。

	cpu:
		用于执行 CPU 密集型工作（如哈希和压缩）的线程数。
		默认值为 1。

	hash（哈希）:
		用于基于数据块的哈希值管理去重数据比较的线程数。
		默认值为 0。

	logical（逻辑）:
		用于基于传入 bio 的逻辑地址管理缓存和锁定的线程数。
		默认值为 0；最大值为 60。

	physical（物理）:
		用于管理底层存储设备管理的线程数。在格式化时，会为
		vdo 选择一个 slab 大小；vdo 存储设备必须足够大，
		以使每个物理线程至少拥有 1 个 slab。默认值为 0；
		最大值为 16。

杂项参数：

	maxDiscard（最大丢弃）:
		所接受的 discard bio 的最大大小，以 4096 字节块为单位。
		对 vdo 卷的 I/O 请求通常被拆分为 4096 字节块，并且
		一次最多处理 2048 个。然而，对 vdo 卷的 discard 请求
		可以自动拆分为更大的大小，在单个 bio 中最多达到
		<maxDiscard> 个 4096 字节块，并且每次限制为 1500 个。
		增大此值可能会提供更好的整体性能，代价是单个 discard
		请求的延迟增加。默认值和最小值为 1；最大值为
		UINT_MAX / 4096。

	deduplication（去重）:
		是否启用去重。默认值为 'on'；可接受的值为 'on' 和 'off'。

	compression（压缩）:
		是否启用压缩。默认值为 'off'；可接受的值为 'on' 和 'off'。

### 设备修改（Device modification）


可以将修改后的表加载到正在运行的、未挂起的 vdo 卷中。这些修改将在设备下次恢复时生效。可修改的参数是 <logical device size>、<physical device size>、<maxDiscard>、<compression> 和 <deduplication>。

如果逻辑设备大小或物理设备大小发生变化，在成功恢复后 vdo 将存储新值，并在未来的启动中要求这些值。这两个参数不能减小。逻辑设备大小不能超过 4 PB。物理设备大小如果增加，必须至少增加 32832 个 4096 字节块，并且不能超过底层存储设备的大小。此外，在格式化 vdo 设备时，会选择一个 slab 大小：物理设备大小永远不能超过提供 8192 个 slab 的大小，并且每次增加都必须足够大，以至少添加一个新的 slab。

示例：

用 1 GB 逻辑空间和 1 GB 物理空间启动一个先前已格式化的 vdo 卷，存储到拥有超过 1 GB 空间的 /dev/dm-1。

```

	dmsetup create vdo0 --table \
	"0 2097152 vdo V4 /dev/dm-1 262144 4096 32768 16380"

```
将逻辑大小增长到 4 GB。

```

	dmsetup reload vdo0 --table \
	"0 8388608 vdo V4 /dev/dm-1 262144 4096 32768 16380"
	dmsetup resume vdo0

```
将物理大小增长到 2 GB。

```

	dmsetup reload vdo0 --table \
	"0 8388608 vdo V4 /dev/dm-1 524288 4096 32768 16380"
	dmsetup resume vdo0

```
将物理大小再增加 1 GB 并提高最大丢弃扇区数。

```

	dmsetup reload vdo0 --table \
	"0 10485760 vdo V4 /dev/dm-1 786432 4096 32768 16380 maxDiscard 8"
	dmsetup resume vdo0

```
停止 vdo 卷。

```

	dmsetup remove vdo0

```
再次启动 vdo 卷。注意逻辑和物理设备大小仍必须匹配，但其他参数可以更改。

```

	dmsetup create vdo1 --table \
	"0 10485760 vdo V4 /dev/dm-1 786432 512 65550 5000 hash 1 logical 3 physical 2"

```
### 消息（Messages）


所有 vdo 设备都接受如下形式的消息：

```

        dmsetup message <target-name> 0 <message-name> <message-parameters>

```
这些消息是：

        stats（统计）:
		输出 vdo 统计信息的当前视图。主要由 vdostats 用户空间
		程序用于解释输出缓冲区。

	config（配置）:
		输出有用的 vdo 配置信息。主要由想要重建类似 VDO 卷
		并希望知道所用创建配置的用户使用。

	dump（转储）:
		将许多内部结构转储到系统日志。这并不总是安全运行，
		因此只应用于调试挂起的 vdo。指定要转储结构的
		可选参数有：

			viopool：传入 bio 的 I/O 请求池
			pools：'viopool' 的同义词
			vdo：管理磁盘上数据的大部分结构
			queues：关于每个 vdo 线程的基本信息
			threads：'queues' 的同义词
			default：等价于 'queues vdo'
			all：上述全部。

        dump-on-shutdown（关机时转储）:
		下次 vdo 关闭时执行一次默认转储。


### 状态（Status）


```

    <device> <operating mode> <in recovery> <index state>
    <compression state> <physical blocks used> <total physical blocks>

	device（设备）:
		vdo 卷的名称。

	operating mode（运行模式）:
		vdo 卷的当前运行模式；值可以是
		'normal'（正常）、'recovering'（恢复中，卷检测到其
		元数据有问题并正在尝试自我修复）和
		'read-only'（只读，发生了迫使 vdo 卷只支持读操作
		而不支持写操作的错误）。

	in recovery（恢复中）:
		vdo 卷当前是否处于恢复模式；
		值可以是 'recovering'（恢复中）或 '-'（表示未恢复）。

	index state（索引状态）:
		vdo 卷中去重索引的当前状态；值可以是
		'closed'（已关闭）、'closing'（关闭中）、'error'（错误）、
		'offline'（离线）、'online'（在线）、'opening'（打开中）
		和 'unknown'（未知）。

	compression state（压缩状态）:
		vdo 卷中压缩的当前状态；值可以是
		'offline'（离线）和 'online'（在线）。

	used physical blocks（已用物理块）:
		vdo 卷正在使用的物理块数量。

	total physical blocks（总物理块）:
		vdo 卷可以使用的物理块总数；
		此值与 <used physical blocks> 之间的差值就是 vdo 卷
		在变满之前还剩余的块数。

```
## 内存需求（Memory Requirements）


一个 vdo 目标需要固定的 38 MB RAM，加上以下随目标规模增长的部分：

- 每配置 1 MB 的块映射缓存大小需要 1.15 MB 的 RAM。块映射缓存最少需要 150 MB。
- 每 1 TB 逻辑空间需要 1.6 MB 的 RAM。
- 卷管理的每 1 TB 物理存储需要 268 MB 的 RAM。

去重索引需要额外随去重窗口大小增长的内存。对于密集索引，每 1 TB 窗口需要 1 GB 的 RAM。对于稀疏索引，每 10 TB 窗口需要 1 GB 的 RAM。索引配置在目标格式化时设定，且不能修改。

## 模块参数（Module Parameters）


vdo 驱动有一个数值参数 'log_level'，它控制驱动日志的详细程度。默认设置为 6
（LOGLEVEL_INFO 及更严重消息）。

## 运行时使用（Run-time Usage）


使用 dm-vdo 时，重要的是要了解其行为与其他存储目标不同的地方。

- 不保证覆盖现有块会成功。由于底层存储可能被多重引用，覆盖一个现有块通常需要 vdo 有一个可用的空闲块。

- 当块不再被使用时，为这些块发送一个 discard 请求可以让 vdo 释放这些块的引用。如果 vdo 是精简配置的，丢弃未使用的块对于防止目标耗尽空间至关重要。然而，由于重复块的共享，任何给定逻辑块的 discard 请求都不保证能回收空间。

- 假设底层存储正确实现了 flush 请求，vdo 对崩溃是有弹性的，但是未刷新的写入在崩溃后可能会或可能不会持久化。

- 对 vdo 目标的每次写入都涉及大量的处理。然而，大部分工作是可并行的。因此，vdo 目标在更高的 I/O 深度下能达到更好的吞吐量，并且最多可以并行支持 2048 个请求。

## 调优（Tuning）


vdo 设备有许多选项，在没有对工作负载的完美了解的情况下，很难做出最优选择。此外，大多数配置选项必须在 vdo 目标启动时设置，并且在不完全关闭它的情况下无法更改；配置不能在目标处于活动状态时更改。理想情况下，应在将 vdo 部署到生产环境之前，使用模拟的工作负载进行调优。

最重要的调整值是块映射缓存大小。为了服务于任何逻辑地址的请求，vdo 必须加载持有相关映射的那部分块映射。这些映射被缓存。当工作集无法放入缓存时，性能会受到影响。默认情况下，vdo 分配 128 MB 的元数据缓存到 RAM 中，以高效访问最多 100 GB 的逻辑空间。对于更大的工作集，应按比例增大它。

逻辑和物理线程数也应调整。一个逻辑线程控制块映射的一个不相交部分，因此额外的逻辑线程会增加并行度并可以提高吞吐量。物理线程控制数据块的一个不相交部分，因此额外的物理线程也可以提高吞吐量。然而，过多的线程会浪费资源并增加争用。

Bio 提交线程控制向底层存储发送 I/O 的并行度；线程越少，重新排序 I/O 请求以获得性能收益的机会就越多，但每个 I/O 请求在提交前也要等待更久。

Bio 确认线程用于完成 I/O 请求。这是在专用线程上完成的，因为执行一个 bio 的回调所需的工作量无法由 vdo 自身控制。通常一个线程就足够了，但额外的线程可能是有益的，特别是当 bio 带有 CPU 密集的回调时。

CPU 线程用于哈希和压缩；在启用了压缩的工作负载中，更多线程可能会带来更高的吞吐量。

哈希线程用于按哈希对活跃请求排序，并确定它们是否应该去重；这些线程执行的最耗 CPU 的操作是比较 4096 字节的数据块。在大多数情况下，单个哈希线程就足够了。
