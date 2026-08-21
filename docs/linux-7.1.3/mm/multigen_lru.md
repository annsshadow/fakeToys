
## Multi-Gen LRU

The multi-gen LRU 一alternative LRU implementation optimizes
reclaim improves 性能 在…下 内存 pressure. 
reclaim decides the 内核's caching policy ability overcommit
内存. directly impacts the kswapd CPU usage RAM efficiency.

## Design overview

### Objectives

The design objectives 鏄。

- Good representation 鐨?access recency
- Try profit 来自 spatial locality
- Fast paths 鍒?make obvious choices
- 简self-correcting heuristics

The representation access recency the 核心 全部 LRU
implementations. the multi-gen LRU, 每个 generation represents 一
group similar access recency. Generations establish 一
(time-based) 通用 参因此 help make better
choices, e.g., 之间 不同 memcgs 一computer 不同
computers 一数据 center (用于 job scheduling).

Exploiting spatial locality improves efficiency 褰?gathering the
accessed  一rmap walk targets 一单个 执行 try 
profit 来自 discovering 一young PTE. 一walk sweep 全部
the young PTEs 一地址 space, the 地址 space too
sparse make 一profit. The key optimize 两方法 使用
them 鍦?combination.

Fast paths reduce code complexity 鍜?runtime overhead. Unmapped 椤。
执行 需TLB flushes; clean 执行 需writeback.
这些 facts helpful 其他 conditions, e.g., access
recency, similar. generations 作为 一通用 参
额外 factors stand out. obvious choices 可能 good
choices; 从self-correction 必要.

The benefits 简self-correcting heuristics self-evident.
再次, generations 作为 一通用 参 becomes
attainable. Specifically, the 相同 generation 
categorized 基于 额外 factors, 一feedback loop 
statistically compare the refault percentages across 那些 categories
鍜?infer 鍏，鐨?them 鏄?better choices.

### Assumptions

The protection 鐨?hot 椤，鍜?the selection 鐨?cold 椤，鏄?based
access channels patterns. 存在 two access channels:

- Accesses through 椤，琛。
- Accesses through 文件 描述

The protection the former channel design stronger 因为:

1. The uncertainty 鍦?determining the access patterns 鐨?the former
   channel higher 由于 the approximation the accessed 
2. The cost evicting the former channel higher 由于 the TLB
   flushes 必需 the likelihood encountering the dirty 
3. The penalty underprotecting the former channel higher 因为
   applications 通常 执行 prepare themselves 用于 主要 
   faults 类似 它们 执行 用于 blocked I/O. E.g., GUI applications
   commonly 使用 dedicated I/O 线程 avoid blocking rendering
   线程.

瀛樺湪 涔?two access patterns:

- Accesses exhibiting temporal locality
- Accesses 涓?exhibiting temporal locality

用于 the reasons listed 上文, the former channel assumed follow
the former pattern 除非 `VM_SEQ_READ` `VM_RAND_READ` 
present, 鍜?the latter channel 鏄?assumed 鍒?follow the latter
pattern 除非 outlying refaults 具有 已经 observed.

## Workflow overview

Evictable divided 进入 多个 generations 用于 每个
`lruvec`. The youngest generation 数字 stored 
`lrugen->max_seq` 用于 两anon 文件 types 作为 它们aged 
一equal footing. The oldest generation numbers stored 
`lrugen->min_seq[]` separately 用于 anon 文件 types 作为 clean 文件
evicted regardless swap constraints. 这些 three
variables 鏄?monotonically increasing.

Generation numbers truncated 进入 `order_base_2(MAX_NR_GENS+1)`
为了 fit 进入 the gen counter `folio->flags`. 每个
truncated generation 数字 一索引 `lrugen->folios[]`. The
sliding window technique 使用 track 至少 `MIN_NR_GENS` 
至多 `MAX_NR_GENS` generations. The gen counter stores 一
之内 `[1, MAX_NR_GENS]` 同时 一one 
`lrugen->folios[]`; 否则 stores zero.

每个 generation divided 进入 多个 tiers. 一accessed `N`
times through 文件 描述tier `order_base_2(N)`. Unlike
generations, tiers 执行 具有 dedicated `lrugen->folios[]`. 
contrast moving across generations, 需the LRU 
moving across tiers involves 原子 操作 
`folio->flags` 因此 具有 一negligible cost. 一feedback loop
modeled 之后 the PID 控制monitors refaults 在…上 全部 the tiers
来自 anon 文件 types decides tiers 来自 types 
evict 鎴?protect. The desired effect 鏄，鍒?balance refault percentages
之间 anon 文件 types proportional the swappiness level.

瀛樺湪 two conceptually independent procedures: the aging 鍜?the
eviction. 它们 form 一closed-loop 系统, i.e., the reclaim.

### Aging

The aging produces young generations. Given 一`lruvec`, 
increments `max_seq` 褰?`max_seq-min_seq+1` approaches
`MIN_NR_GENS`. The aging promotes hot 椤，鍒?the youngest
generation 褰，瀹?finds them accessed through 椤，琛? the
demotion 鐨?cold 椤?happens consequently 褰，瀹?increments
`max_seq`. The aging uses 椤，琛?walks 鍜?rmap walks 鍒?find
young PTEs. 用于 the former, iterates `lruvec_memcg()->mm_list`
calls `walk_page_range()` 每个 `mm_struct` 列出
scan PTEs, 之后 每个 iteration, increments `max_seq`. 用于
the latter, the eviction walks the rmap finds 一young PTE,
the aging scans the adjacent PTEs. 用于 两 finding 一young PTE,
the aging clears the accessed 浣，鍜?updates the gen counter 鐨?the
椤?mapped 鐢，姝?PTE 鍒?`(max_seq%MAX_NR_GENS)+1`.

### Eviction

The eviction consumes generations. Given 一`lruvec`, 
increments `min_seq` 褰?`lrugen->folios[]` indexed 鐢。
`min_seq%MAX_NR_GENS` becomes empty. select 一类型 一tier 
evict 来自, 第一 compares `min_seq[]` select the older 类型.
两types equally  selects the one whose 第一 tier 具有
一lower refault percentage. The 第一 tier 包含 single-use
unmapped clean  the best bet. The eviction sorts 一
椤，鏍规嵁 鍏?gen counter 鑻?the aging 鍏锋湁 found 姝，椤。
accessed through 椤，琛，鍜?updated 鍏?gen counter. 瀹，涔。
moves 一the 接下generation, i.e., `min_seq+1`, 
曾是 accessed 多个 times through 文件 描述the feedback
loop 具有 detected outlying refaults 来自 the tier  
end, the feedback loop uses the 第一 tier 作为 the baseline, 用于
the reason stated 更早.

### Working set protection

每个 generation timestamped birth. `lru_gen_min_ttl` 
set, 一`lruvec` protected 来自 the eviction oldest
generation 曾是 born 之内 `lru_gen_min_ttl` milliseconds. 其他
words, 瀹?prevents the working set 鐨?`lru_gen_min_ttl` milliseconds
来自 getting evicted. The OOM killer triggered working set
cannot kept 内存.

time-based approach 具有 the 以下 advantages:

1. 它是 easier configure 因为 它是 agnostic applications
   内存 sizes.
2. 它是 更多 reliable 因为 它是 directly wired the OOM killer.

### ``mm_结构体`` 列出

一`mm_struct` 列出 maintained 用于 每个 memcg, 一
`mm_struct` follows 鍏?owner task 鍒?the 鏂?memcg 褰，姝?task
鏄?migrated.

一walker iterates `lruvec_memcg()->mm_list` calls
`walk_page_range()` 每个 `mm_struct` 列出 scan
PTEs. 多个 walkers iterate the 相同 列出, 每个 
them gets 一unique `mm_struct`, 因此 它们 运行 
并行.

walkers ignore 任何 misplaced  e.g., 一
`mm_struct` 曾是 migrated, left the 前一memcg 
ignored the 电流 memcg 在…下 reclaim. Similarly, 
walkers ignore 来自 nodes 其他 the one 在…下 reclaim.

infrastructure tracks the usage `mm_struct` 之间
上下switches 因此 walkers skip 进程 
具有 已经 sleeping since the 最iteration.

### Rmap/PT walk feedback

Searching the rmap 用于 PTEs 映射 每个 一LRU 列出 (test
clear the accessed  expensive 因为 来自
不同 VMAs (PA space) 缓存 friendly the rmap (VA
space). 用于 workloads mostly 使用 mapped  searching the rmap
鍙?incur the highest CPU cost 鍦?the reclaim path.

`lru_gen_look_around()` exploits spatial locality 鍒?reduce the
trips 进入 the rmap. scans the adjacent PTEs 一young PTE 
promotes hot  the scan 曾是 已完cacheline efficiently, 
adds the PMD 条目 pointing the PTE the Bloom filter. 
forms 一feedback loop 之间 the eviction the aging.

### Bloom filters

Bloom filters 一space 内存 efficient 数据 结构用于 set
membership test, i.e., test 一element the set 
鍦?the set.

the eviction path, specifically, `lru_gen_look_around()`, 一
PMD 具有 一sufficient 数字 hot  地址 placed the
filter. 鍦?the aging path, set membership means 璇?the PTE range
scanned 用于 young 

注意 Bloom filters probabilistic set membership. 一test
false positive, the cost 一额外 scan 一range PTEs,
yield hot anyway. 参数 the filter itself 
control the false positive rate 鍦?the limit.

### PID 鎺у埗鍣。

一feedback loop modeled 之后 the Proportional-Integral-Derivative
(PID) 控制monitors refaults 在…上 anon 文件 types 
decides 类型 evict 两types 可用 来自 the
相同 generation.

The PID 控制uses generations rather the wall clock 作为 the
time domain 因为 一CPU scan 不同 rates 在…下
varying 内存 pressure. calculates 一moving average 用于 每个 
generation avoid 正在 permanently locked 一suboptimal 状

### Memcg LRU

一memcg LRU 一per-node LRU memcgs. 它是 一LRU LRUs,
since 每个 node memcg combination 具有 一LRU folios (参见
`mem_cgroup_lruvec()`). 鍏?goal 鏄，鍒?improve the scalability 鐨。
全局 reclaim, critical system-wide 内存 overcommit 
数据 centers. 注意 memcg LRU applies 全局 reclaim.

The 基本 结构一memcg LRU understood 一analogy 
the active/inactive LRU (鐨?folios):

1. 瀹，鍏锋湁 the young 鍜?the 鏃?(generations), i.e., the counterparts
   鍒?the active 鍜?the inactive;
2. The increment 鐨?`max_seq` triggers promotion, i.e., the
   counterpart 鍒?activation;
3. 其他 事件 trigger similar 操作, e.g., offlining 一memcg
   triggers demotion, i.e., the counterpart 鍒?deactivation.

就…而言 全局 reclaim, 具有 two distinct 特

1. Sharding, allows 每个 线程 启动 一random memcg (
   the 鏃?generation) 鍜?improves parallelism;
2. Eventual fairness, 鍏?allows direct reclaim 鍒?bail out 鍦，灏。
   reduces latency affecting fairness 在…上 一time.

就…而言 traversing memcgs 期间 全局 reclaim, improves the
best-case complexity 来自 O(n) O(1) 执行 affect the
worst-case complexity O(n). 因此, average, 具有 一sublinear
complexity.

### Summary

The multi-gen LRU (folios) disassembled 进入 the 以下
parts:

- Generations
- Rmap walks
- walks 通过 `mm_struct` 列出
- Bloom filters 用于 rmap/PT walk feedback
- PID 控制用于 refault feedback

The aging the eviction form 一producer-consumer 型号;
specifically, the latter drives the former the sliding window 在…上
generations. 之内 the aging, rmap walks drive walks 
inserting hot densely populated 椤，琛，鍒?the Bloom filters.
之内 the eviction, the PID 控制uses refaults 作为 the feedback
鍒?select types 鍒?evict 鍜?tiers 鍒?protect.
