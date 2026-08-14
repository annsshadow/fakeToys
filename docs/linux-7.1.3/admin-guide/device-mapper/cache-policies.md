## 编写策略的指南


尽量将事务性（transactionality）排除在策略之外。核心会小心地避免询问任何正在迁移中的内容。这很麻烦，
但使编写策略更容易。

映射在构建时加载到策略中。

目标映射的每个 bio 都会被提交给策略。策略可以返回一个简单的 HIT 或 MISS，或者发起一次迁移。

目前策略无法发起后台工作，例如开始写回即将被淘汰的脏块。

因为我们映射的是 bio 而非请求，策略很容易被许多小的 bio 所欺骗。出于这个原因，核心目标会向策略发出
周期性的 tick。建议策略在每个 tick 内不要对同一个块的狀态（例如命中计数）更新超过一次。核心通过观察
bio 的完成来发出 tick，从而试图观察 io 调度器何时让 io 运行。


## 已提供的缓存替换策略概述


### multiqueue（mq）


此策略现在已是 smq 的别名（见下文）。

```

	'sequential_threshold <#nr_sequential_ios>'
	'random_threshold <#nr_random_ios>'
	'read_promote_adjustment <value>'
	'write_promote_adjustment <value>'
	'discard_promote_adjustment <value>'

```
### Stochastic multiqueue（smq，随机多队列）


此策略是默认策略。

随机多队列（smq）策略解决了多队列（mq）策略的一些问题。

smq 策略（相比 mq）有望带来更少的内存占用、更好的性能，以及在面对变化的工作负载时更强的适应性。smq
也没有任何繁琐的调优旋钮。

用户只需适当地重新加载一个使用 cache 目标的 DM 表，即可从 "mq" 切换到 "smq"。这样做会导致 mq 策略的
所有提示被丢弃。此外，在 smq 重新计算应被缓存的源设备热点之前，缓存的性能可能会略有下降。


##### 内存使用


mq 策略使用了大量内存；在 64 位机器上每个缓存块 88 字节。

smq 使用 28 位索引而非指针来实现其数据结构。它避免为每个块存储显式的命中计数。它有一个“热点”队列，
而不是预缓存（pre-cache），该队列使用四分之一的条目（每个热点块覆盖的区域比单个缓存块更大）。

所有这些都意味着 smq 每个缓存块使用约 25 字节。仍然是不少内存，但无论如何是实质性的改进。


##### 级别平衡


mq 根据命中计数（~ln(命中计数)）将条目放入多队列结构的不同级别。这意味着底层通常拥有最多的条目，
而顶层只有很少。像这样不平衡的级别降低了多队列的效力。

smq 不维护命中计数，而是用来自上一级最近最少使用（LRU）的条目来交换命中的条目。整体排序是这一随机过程
的副作用。通过此方案，我们可以决定每个多队列级别容纳多少条目，从而做出更好的提升/降级决策。

适应性：
mq 策略为每个缓存块维护一个命中计数。要让一个不同的块被提升到缓存中，其命中计数必须超过当前缓存中
最低的（命中计数）。这意味着缓存适应不同 IO 模式可能需要很长时间。

smq 不维护命中计数，因此许多此类问题就消失了。此外，它跟踪热点队列的性能，用于决定提升哪些块。如果热点
队列表现糟糕，则它会更快地在级别之间移动条目。这让它能非常快地适应新的 IO 模式。


##### 性能


对 smq 的测试显示出比 mq 好得多的性能。


### cleaner


cleaner 将所有脏块写回缓存以将其停用。


## 示例


```

	cache <metadata dev> <cache dev> <origin dev> <block size>
	<#feature_args> [<feature arg>]*
	<policy> <#policy_args> [<policy arg>]*

```
```

	dmsetup message <mapped device> 0 sequential_threshold 1024
	dmsetup message <mapped device> 0 random_threshold 8

```
```

	dmsetup create blah --table "0 268435456 cache /dev/sdb /dev/sdc \
	    /dev/sdd 512 0 mq 4 sequential_threshold 1024 random_threshold 8"
	创建一个名为 'blah'、大小为 128GB 的映射设备，其顺序阈值设为 1024，随机阈值设为 8。

```
