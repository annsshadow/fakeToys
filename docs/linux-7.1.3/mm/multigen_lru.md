
## Multi-Gen LRU

The multi-gen LRU 是 一个 alternative LRU implementation 该 optimizes
页 reclaim 和 improves 性能 在…下 内存 pressure. 页
reclaim decides the 内核's caching policy 和 ability 到 overcommit
内存. 它 directly impacts the kswapd CPU usage 和 RAM efficiency.

## Design overview

### Objectives

The design objectives 是:

- Good representation 的 access recency
- Try 到 profit 来自 spatial locality
- Fast paths 到 make obvious choices
- 简单 self-correcting heuristics

The representation 的 access recency 是 在 the 核心 的 全部 LRU
implementations. 在 the multi-gen LRU, 每个 generation represents 一个
group 的 页 与 similar access recency. Generations establish 一个
(time-based) 通用 帧 的 参考 和 因此 help make better
choices, e.g., 之间 不同 memcgs 在 一个 computer 或 不同
computers 在 一个 数据 center (用于 job scheduling).

Exploiting spatial locality improves efficiency 当 gathering the
accessed 位. 一个 rmap walk targets 一个 单个 页 和 执行 不 try 到
profit 来自 discovering 一个 young PTE. 一个 页 表 walk 可 sweep 全部
the young PTEs 在 一个 地址 space, 但 the 地址 space 可 为 too
sparse 到 make 一个 profit. The key 是 到 optimize 两者 方法 和 使用
them 在 combination.

Fast paths reduce code complexity 和 runtime overhead. Unmapped 页
执行 不 需要 TLB flushes; clean 页 执行 不 需要 writeback.
这些 facts 是 仅 helpful 当 其他 conditions, e.g., access
recency, 是 similar. 与 generations 作为 一个 通用 帧 的 参考,
额外 factors stand out. 但 obvious choices 可能 不 为 good
choices; 从而 self-correction 是 必要.

The benefits 的 简单 self-correcting heuristics 是 self-evident.
再次, 与 generations 作为 一个 通用 帧 的 参考, 此 becomes
attainable. Specifically, 页 在 the 相同 generation 可 为
categorized 基于 额外 factors, 和 一个 feedback loop 可
statistically compare the refault percentages across 那些 categories
和 infer 其 的 them 是 better choices.

### Assumptions

The protection 的 hot 页 和 the selection 的 cold 页 是 based
在 页 access channels 和 patterns. 存在 two access channels:

- Accesses through 页 表
- Accesses through 文件 描述符

The protection 的 the former channel 是 由 design stronger 因为:

1. The uncertainty 在 determining the access patterns 的 the former
   channel 是 higher 由于 the approximation 的 the accessed 位.
2. The cost 的 evicting the former channel 是 higher 由于 the TLB
   flushes 必需 和 the likelihood 的 encountering the dirty 位.
3. The penalty 的 underprotecting the former channel 是 higher 因为
   applications 通常 执行 不 prepare themselves 用于 主要 页
   faults 类似 它们 执行 用于 blocked I/O. E.g., GUI applications
   commonly 使用 dedicated I/O 线程 到 avoid blocking rendering
   线程.

存在 也 two access patterns:

- Accesses exhibiting temporal locality
- Accesses 不 exhibiting temporal locality

用于 the reasons listed 上文, the former channel 是 assumed 到 follow
the former pattern 除非 `VM_SEQ_READ` 或 `VM_RAND_READ` 是
present, 和 the latter channel 是 assumed 到 follow the latter
pattern 除非 outlying refaults 具有 已经 observed.

## Workflow overview

Evictable 页 是 divided 进入 多个 generations 用于 每个
`lruvec`. The youngest generation 数字 是 stored 在
`lrugen->max_seq` 用于 两者 anon 和 文件 types 作为 它们是 aged 在
一个 equal footing. The oldest generation numbers 是 stored 在
`lrugen->min_seq[]` separately 用于 anon 和 文件 types 作为 clean 文件
页 可 为 evicted regardless 的 swap constraints. 这些 three
variables 是 monotonically increasing.

Generation numbers 是 truncated 进入 `order_base_2(MAX_NR_GENS+1)`
位 为了 fit 进入 the gen counter 在 `folio->flags`. 每个
truncated generation 数字 是 一个 索引 到 `lrugen->folios[]`. The
sliding window technique 是 使用 到 track 至少 `MIN_NR_GENS` 和
至多 `MAX_NR_GENS` generations. The gen counter stores 一个 值
之内 `[1, MAX_NR_GENS]` 同时 一个 页 是 在 one 的
`lrugen->folios[]`; 否则 它 stores zero.

每个 generation 是 divided 进入 多个 tiers. 一个 页 accessed `N`
times through 文件 描述符 是 在 tier `order_base_2(N)`. Unlike
generations, tiers 执行 不 具有 dedicated `lrugen->folios[]`. 在
contrast 到 moving across generations, 其 需要 the LRU 锁,
moving across tiers 仅 involves 原子 操作 在
`folio->flags` 和 因此 具有 一个 negligible cost. 一个 feedback loop
modeled 之后 the PID 控制器 monitors refaults 在…上 全部 the tiers
来自 anon 和 文件 types 和 decides 其 tiers 来自 其 types 到
evict 或 protect. The desired effect 是 到 balance refault percentages
之间 anon 和 文件 types proportional 到 the swappiness level.

存在 two conceptually independent procedures: the aging 和 the
eviction. 它们 form 一个 closed-loop 系统, i.e., the 页 reclaim.

### Aging

The aging produces young generations. Given 一个 `lruvec`, 它
increments `max_seq` 当 `max_seq-min_seq+1` approaches
`MIN_NR_GENS`. The aging promotes hot 页 到 the youngest
generation 当 它 finds them accessed through 页 表; the
demotion 的 cold 页 happens consequently 当 它 increments
`max_seq`. The aging uses 页 表 walks 和 rmap walks 到 find
young PTEs. 用于 the former, 它 iterates `lruvec_memcg()->mm_list`
和 calls `walk_page_range()` 与 每个 `mm_struct` 在 此 列出
到 scan PTEs, 和 之后 每个 iteration, 它 increments `max_seq`. 用于
the latter, 当 the eviction walks the rmap 和 finds 一个 young PTE,
the aging scans the adjacent PTEs. 用于 两者, 在 finding 一个 young PTE,
the aging clears the accessed 位 和 updates the gen counter 的 the
页 mapped 由 此 PTE 到 `(max_seq%MAX_NR_GENS)+1`.

### Eviction

The eviction consumes 旧 generations. Given 一个 `lruvec`, 它
increments `min_seq` 当 `lrugen->folios[]` indexed 由
`min_seq%MAX_NR_GENS` becomes empty. 到 select 一个 类型 和 一个 tier 到
evict 来自, 它 第一 compares `min_seq[]` 到 select the older 类型.
若 两者 types 是 equally 旧, 它 selects the one whose 第一 tier 具有
一个 lower refault percentage. The 第一 tier 包含 single-use
unmapped clean 页, 其 是 the best bet. The eviction sorts 一个
页 根据 其 gen counter 若 the aging 具有 found 此 页
accessed through 页 表 和 updated 其 gen counter. 它 也
moves 一个 页 到 the 接下来 generation, i.e., `min_seq+1`, 若 此 页
曾是 accessed 多个 times through 文件 描述符 和 the feedback
loop 具有 detected outlying refaults 来自 the tier 此 页 是 在. 到
此 end, the feedback loop uses the 第一 tier 作为 the baseline, 用于
the reason stated 更早.

### Working set protection

每个 generation 是 timestamped 在 birth. 若 `lru_gen_min_ttl` 是
set, 一个 `lruvec` 是 protected 来自 the eviction 当 其 oldest
generation 曾是 born 之内 `lru_gen_min_ttl` milliseconds. 在 其他
words, 它 prevents the working set 的 `lru_gen_min_ttl` milliseconds
来自 getting evicted. The OOM killer 是 triggered 若 此 working set
cannot 为 kept 在 内存.

此 time-based approach 具有 the 以下 advantages:

1. 它是 easier 到 configure 因为 它是 agnostic 到 applications
   和 内存 sizes.
2. 它是 更多 reliable 因为 它是 directly wired 到 the OOM killer.

### ``mm_结构体`` 列出

一个 `mm_struct` 列出 是 maintained 用于 每个 memcg, 和 一个
`mm_struct` follows 其 owner task 到 the 新 memcg 当 此 task
是 migrated.

一个 页 表 walker iterates `lruvec_memcg()->mm_list` 和 calls
`walk_page_range()` 与 每个 `mm_struct` 在 此 列出 到 scan
PTEs. 当 多个 页 表 walkers iterate the 相同 列出, 每个 的
them gets 一个 unique `mm_struct`, 和 因此 它们 可 运行 在
并行.

页 表 walkers ignore 任何 misplaced 页, e.g., 若 一个
`mm_struct` 曾是 migrated, 页 left 在 the 前一个 memcg 将 为
ignored 当 the 电流 memcg 是 在…下 reclaim. Similarly, 页 表
walkers 将 ignore 页 来自 nodes 其他 比 the one 在…下 reclaim.

此 infrastructure 也 tracks the usage 的 `mm_struct` 之间
上下文 switches 因此 该 页 表 walkers 可 skip 进程 该
具有 已经 sleeping since the 最后 iteration.

### Rmap/PT walk feedback

Searching the rmap 用于 PTEs 映射 每个 页 在 一个 LRU 列出 (到 test
和 clear the accessed 位) 可 为 expensive 因为 页 来自
不同 VMAs (PA space) 是 不 缓存 friendly 到 the rmap (VA
space). 用于 workloads mostly 使用 mapped 页, searching the rmap
可 incur the highest CPU cost 在 the reclaim path.

`lru_gen_look_around()` exploits spatial locality 到 reduce the
trips 进入 the rmap. 它 scans the adjacent PTEs 的 一个 young PTE 和
promotes hot 页. 若 the scan 曾是 已完成 cacheline efficiently, 它
adds the PMD 条目 pointing 到 the PTE 表 到 the Bloom filter. 此
forms 一个 feedback loop 之间 the eviction 和 the aging.

### Bloom filters

Bloom filters 是 一个 space 和 内存 efficient 数据 结构体 用于 set
membership test, i.e., test 若 一个 element 是 不 在 the set 或 可 为
在 the set.

在 the eviction path, specifically, 在 `lru_gen_look_around()`, 若 一个
PMD 具有 一个 sufficient 数字 的 hot 页, 其 地址 是 placed 在 the
filter. 在 the aging path, set membership means 该 the PTE range
将 为 scanned 用于 young 页.

注意 该 Bloom filters 是 probabilistic 在 set membership. 若 一个 test
是 false positive, the cost 是 一个 额外 scan 的 一个 range 的 PTEs,
其 可 yield hot 页 anyway. 参数 的 the filter itself 可
control the false positive rate 在 the limit.

### PID 控制器

一个 feedback loop modeled 之后 the Proportional-Integral-Derivative
(PID) 控制器 monitors refaults 在…上 anon 和 文件 types 和
decides 其 类型 到 evict 当 两者 types 是 可用 来自 the
相同 generation.

The PID 控制器 uses generations rather 比 the wall clock 作为 the
time domain 因为 一个 CPU 可 scan 页 在 不同 rates 在…下
varying 内存 pressure. 它 calculates 一个 moving average 用于 每个 新
generation 到 avoid 正在 permanently locked 在 一个 suboptimal 状态.

### Memcg LRU

一个 memcg LRU 是 一个 per-node LRU 的 memcgs. 它是 也 一个 LRU 的 LRUs,
since 每个 node 和 memcg combination 具有 一个 LRU 的 folios (参见
`mem_cgroup_lruvec()`). 其 goal 是 到 improve the scalability 的
全局 reclaim, 其 是 critical 到 system-wide 内存 overcommit 在
数据 centers. 注意 该 memcg LRU 仅 applies 到 全局 reclaim.

The 基本 结构体 的 一个 memcg LRU 可 为 understood 由 一个 analogy 到
the active/inactive LRU (的 folios):

1. 它 具有 the young 和 the 旧 (generations), i.e., the counterparts
   到 the active 和 the inactive;
2. The increment 的 `max_seq` triggers promotion, i.e., the
   counterpart 到 activation;
3. 其他 事件 trigger similar 操作, e.g., offlining 一个 memcg
   triggers demotion, i.e., the counterpart 到 deactivation.

就…而言 全局 reclaim, 它 具有 two distinct 特性:

1. Sharding, 其 allows 每个 线程 到 启动 在 一个 random memcg (在
   the 旧 generation) 和 improves parallelism;
2. Eventual fairness, 其 allows direct reclaim 到 bail out 在 将
   和 reduces latency 无 affecting fairness 在…上 一些 time.

就…而言 traversing memcgs 期间 全局 reclaim, 它 improves the
best-case complexity 来自 O(n) 到 O(1) 和 执行 不 affect the
worst-case complexity O(n). 因此, 在 average, 它 具有 一个 sublinear
complexity.

### Summary

The multi-gen LRU (的 folios) 可 为 disassembled 进入 the 以下
parts:

- Generations
- Rmap walks
- 页 表 walks 通过 `mm_struct` 列出
- Bloom filters 用于 rmap/PT walk feedback
- PID 控制器 用于 refault feedback

The aging 和 the eviction form 一个 producer-consumer 型号;
specifically, the latter drives the former 由 the sliding window 在…上
generations. 之内 the aging, rmap walks drive 页 表 walks 由
inserting hot densely populated 页 表 到 the Bloom filters.
之内 the eviction, the PID 控制器 uses refaults 作为 the feedback
到 select types 到 evict 和 tiers 到 protect.
