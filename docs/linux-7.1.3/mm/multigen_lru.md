
## Multi-Gen LRU

The multi-gen LRU 鏄?涓€涓?alternative LRU implementation 璇?optimizes
椤?reclaim 鍜?improves 鎬ц兘 鍦ㄢ€︿笅 鍐呭瓨 pressure. 椤?
reclaim decides the 鍐呮牳's caching policy 鍜?ability 鍒?overcommit
鍐呭瓨. 瀹?directly impacts the kswapd CPU usage 鍜?RAM efficiency.

## Design overview

### Objectives

The design objectives 鏄?

- Good representation 鐨?access recency
- Try 鍒?profit 鏉ヨ嚜 spatial locality
- Fast paths 鍒?make obvious choices
- 绠€鍗?self-correcting heuristics

The representation 鐨?access recency 鏄?鍦?the 鏍稿績 鐨?鍏ㄩ儴 LRU
implementations. 鍦?the multi-gen LRU, 姣忎釜 generation represents 涓€涓?
group 鐨?椤?涓?similar access recency. Generations establish 涓€涓?
(time-based) 閫氱敤 甯?鐨?鍙傝€?鍜?鍥犳 help make better
choices, e.g., 涔嬮棿 涓嶅悓 memcgs 鍦?涓€涓?computer 鎴?涓嶅悓
computers 鍦?涓€涓?鏁版嵁 center (鐢ㄤ簬 job scheduling).

Exploiting spatial locality improves efficiency 褰?gathering the
accessed 浣? 涓€涓?rmap walk targets 涓€涓?鍗曚釜 椤?鍜?鎵ц 涓?try 鍒?
profit 鏉ヨ嚜 discovering 涓€涓?young PTE. 涓€涓?椤?琛?walk 鍙?sweep 鍏ㄩ儴
the young PTEs 鍦?涓€涓?鍦板潃 space, 浣?the 鍦板潃 space 鍙?涓?too
sparse 鍒?make 涓€涓?profit. The key 鏄?鍒?optimize 涓よ€?鏂规硶 鍜?浣跨敤
them 鍦?combination.

Fast paths reduce code complexity 鍜?runtime overhead. Unmapped 椤?
鎵ц 涓?闇€瑕?TLB flushes; clean 椤?鎵ц 涓?闇€瑕?writeback.
杩欎簺 facts 鏄?浠?helpful 褰?鍏朵粬 conditions, e.g., access
recency, 鏄?similar. 涓?generations 浣滀负 涓€涓?閫氱敤 甯?鐨?鍙傝€?
棰濆 factors stand out. 浣?obvious choices 鍙兘 涓?涓?good
choices; 浠庤€?self-correction 鏄?蹇呰.

The benefits 鐨?绠€鍗?self-correcting heuristics 鏄?self-evident.
鍐嶆, 涓?generations 浣滀负 涓€涓?閫氱敤 甯?鐨?鍙傝€? 姝?becomes
attainable. Specifically, 椤?鍦?the 鐩稿悓 generation 鍙?涓?
categorized 鍩轰簬 棰濆 factors, 鍜?涓€涓?feedback loop 鍙?
statistically compare the refault percentages across 閭ｄ簺 categories
鍜?infer 鍏?鐨?them 鏄?better choices.

### Assumptions

The protection 鐨?hot 椤?鍜?the selection 鐨?cold 椤?鏄?based
鍦?椤?access channels 鍜?patterns. 瀛樺湪 two access channels:

- Accesses through 椤?琛?
- Accesses through 鏂囦欢 鎻忚堪绗?

The protection 鐨?the former channel 鏄?鐢?design stronger 鍥犱负:

1. The uncertainty 鍦?determining the access patterns 鐨?the former
   channel 鏄?higher 鐢变簬 the approximation 鐨?the accessed 浣?
2. The cost 鐨?evicting the former channel 鏄?higher 鐢变簬 the TLB
   flushes 蹇呴渶 鍜?the likelihood 鐨?encountering the dirty 浣?
3. The penalty 鐨?underprotecting the former channel 鏄?higher 鍥犱负
   applications 閫氬父 鎵ц 涓?prepare themselves 鐢ㄤ簬 涓昏 椤?
   faults 绫讳技 瀹冧滑 鎵ц 鐢ㄤ簬 blocked I/O. E.g., GUI applications
   commonly 浣跨敤 dedicated I/O 绾跨▼ 鍒?avoid blocking rendering
   绾跨▼.

瀛樺湪 涔?two access patterns:

- Accesses exhibiting temporal locality
- Accesses 涓?exhibiting temporal locality

鐢ㄤ簬 the reasons listed 涓婃枃, the former channel 鏄?assumed 鍒?follow
the former pattern 闄ら潪 `VM_SEQ_READ` 鎴?`VM_RAND_READ` 鏄?
present, 鍜?the latter channel 鏄?assumed 鍒?follow the latter
pattern 闄ら潪 outlying refaults 鍏锋湁 宸茬粡 observed.

## Workflow overview

Evictable 椤?鏄?divided 杩涘叆 澶氫釜 generations 鐢ㄤ簬 姣忎釜
`lruvec`. The youngest generation 鏁板瓧 鏄?stored 鍦?
`lrugen->max_seq` 鐢ㄤ簬 涓よ€?anon 鍜?鏂囦欢 types 浣滀负 瀹冧滑鏄?aged 鍦?
涓€涓?equal footing. The oldest generation numbers 鏄?stored 鍦?
`lrugen->min_seq[]` separately 鐢ㄤ簬 anon 鍜?鏂囦欢 types 浣滀负 clean 鏂囦欢
椤?鍙?涓?evicted regardless 鐨?swap constraints. 杩欎簺 three
variables 鏄?monotonically increasing.

Generation numbers 鏄?truncated 杩涘叆 `order_base_2(MAX_NR_GENS+1)`
浣?涓轰簡 fit 杩涘叆 the gen counter 鍦?`folio->flags`. 姣忎釜
truncated generation 鏁板瓧 鏄?涓€涓?绱㈠紩 鍒?`lrugen->folios[]`. The
sliding window technique 鏄?浣跨敤 鍒?track 鑷冲皯 `MIN_NR_GENS` 鍜?
鑷冲 `MAX_NR_GENS` generations. The gen counter stores 涓€涓?鍊?
涔嬪唴 `[1, MAX_NR_GENS]` 鍚屾椂 涓€涓?椤?鏄?鍦?one 鐨?
`lrugen->folios[]`; 鍚﹀垯 瀹?stores zero.

姣忎釜 generation 鏄?divided 杩涘叆 澶氫釜 tiers. 涓€涓?椤?accessed `N`
times through 鏂囦欢 鎻忚堪绗?鏄?鍦?tier `order_base_2(N)`. Unlike
generations, tiers 鎵ц 涓?鍏锋湁 dedicated `lrugen->folios[]`. 鍦?
contrast 鍒?moving across generations, 鍏?闇€瑕?the LRU 閿?
moving across tiers 浠?involves 鍘熷瓙 鎿嶄綔 鍦?
`folio->flags` 鍜?鍥犳 鍏锋湁 涓€涓?negligible cost. 涓€涓?feedback loop
modeled 涔嬪悗 the PID 鎺у埗鍣?monitors refaults 鍦ㄢ€︿笂 鍏ㄩ儴 the tiers
鏉ヨ嚜 anon 鍜?鏂囦欢 types 鍜?decides 鍏?tiers 鏉ヨ嚜 鍏?types 鍒?
evict 鎴?protect. The desired effect 鏄?鍒?balance refault percentages
涔嬮棿 anon 鍜?鏂囦欢 types proportional 鍒?the swappiness level.

瀛樺湪 two conceptually independent procedures: the aging 鍜?the
eviction. 瀹冧滑 form 涓€涓?closed-loop 绯荤粺, i.e., the 椤?reclaim.

### Aging

The aging produces young generations. Given 涓€涓?`lruvec`, 瀹?
increments `max_seq` 褰?`max_seq-min_seq+1` approaches
`MIN_NR_GENS`. The aging promotes hot 椤?鍒?the youngest
generation 褰?瀹?finds them accessed through 椤?琛? the
demotion 鐨?cold 椤?happens consequently 褰?瀹?increments
`max_seq`. The aging uses 椤?琛?walks 鍜?rmap walks 鍒?find
young PTEs. 鐢ㄤ簬 the former, 瀹?iterates `lruvec_memcg()->mm_list`
鍜?calls `walk_page_range()` 涓?姣忎釜 `mm_struct` 鍦?姝?鍒楀嚭
鍒?scan PTEs, 鍜?涔嬪悗 姣忎釜 iteration, 瀹?increments `max_seq`. 鐢ㄤ簬
the latter, 褰?the eviction walks the rmap 鍜?finds 涓€涓?young PTE,
the aging scans the adjacent PTEs. 鐢ㄤ簬 涓よ€? 鍦?finding 涓€涓?young PTE,
the aging clears the accessed 浣?鍜?updates the gen counter 鐨?the
椤?mapped 鐢?姝?PTE 鍒?`(max_seq%MAX_NR_GENS)+1`.

### Eviction

The eviction consumes 鏃?generations. Given 涓€涓?`lruvec`, 瀹?
increments `min_seq` 褰?`lrugen->folios[]` indexed 鐢?
`min_seq%MAX_NR_GENS` becomes empty. 鍒?select 涓€涓?绫诲瀷 鍜?涓€涓?tier 鍒?
evict 鏉ヨ嚜, 瀹?绗竴 compares `min_seq[]` 鍒?select the older 绫诲瀷.
鑻?涓よ€?types 鏄?equally 鏃? 瀹?selects the one whose 绗竴 tier 鍏锋湁
涓€涓?lower refault percentage. The 绗竴 tier 鍖呭惈 single-use
unmapped clean 椤? 鍏?鏄?the best bet. The eviction sorts 涓€涓?
椤?鏍规嵁 鍏?gen counter 鑻?the aging 鍏锋湁 found 姝?椤?
accessed through 椤?琛?鍜?updated 鍏?gen counter. 瀹?涔?
moves 涓€涓?椤?鍒?the 鎺ヤ笅鏉?generation, i.e., `min_seq+1`, 鑻?姝?椤?
鏇炬槸 accessed 澶氫釜 times through 鏂囦欢 鎻忚堪绗?鍜?the feedback
loop 鍏锋湁 detected outlying refaults 鏉ヨ嚜 the tier 姝?椤?鏄?鍦? 鍒?
姝?end, the feedback loop uses the 绗竴 tier 浣滀负 the baseline, 鐢ㄤ簬
the reason stated 鏇存棭.

### Working set protection

姣忎釜 generation 鏄?timestamped 鍦?birth. 鑻?`lru_gen_min_ttl` 鏄?
set, 涓€涓?`lruvec` 鏄?protected 鏉ヨ嚜 the eviction 褰?鍏?oldest
generation 鏇炬槸 born 涔嬪唴 `lru_gen_min_ttl` milliseconds. 鍦?鍏朵粬
words, 瀹?prevents the working set 鐨?`lru_gen_min_ttl` milliseconds
鏉ヨ嚜 getting evicted. The OOM killer 鏄?triggered 鑻?姝?working set
cannot 涓?kept 鍦?鍐呭瓨.

姝?time-based approach 鍏锋湁 the 浠ヤ笅 advantages:

1. 瀹冩槸 easier 鍒?configure 鍥犱负 瀹冩槸 agnostic 鍒?applications
   鍜?鍐呭瓨 sizes.
2. 瀹冩槸 鏇村 reliable 鍥犱负 瀹冩槸 directly wired 鍒?the OOM killer.

### ``mm_缁撴瀯浣揱` 鍒楀嚭

涓€涓?`mm_struct` 鍒楀嚭 鏄?maintained 鐢ㄤ簬 姣忎釜 memcg, 鍜?涓€涓?
`mm_struct` follows 鍏?owner task 鍒?the 鏂?memcg 褰?姝?task
鏄?migrated.

涓€涓?椤?琛?walker iterates `lruvec_memcg()->mm_list` 鍜?calls
`walk_page_range()` 涓?姣忎釜 `mm_struct` 鍦?姝?鍒楀嚭 鍒?scan
PTEs. 褰?澶氫釜 椤?琛?walkers iterate the 鐩稿悓 鍒楀嚭, 姣忎釜 鐨?
them gets 涓€涓?unique `mm_struct`, 鍜?鍥犳 瀹冧滑 鍙?杩愯 鍦?
骞惰.

椤?琛?walkers ignore 浠讳綍 misplaced 椤? e.g., 鑻?涓€涓?
`mm_struct` 鏇炬槸 migrated, 椤?left 鍦?the 鍓嶄竴涓?memcg 灏?涓?
ignored 褰?the 鐢垫祦 memcg 鏄?鍦ㄢ€︿笅 reclaim. Similarly, 椤?琛?
walkers 灏?ignore 椤?鏉ヨ嚜 nodes 鍏朵粬 姣?the one 鍦ㄢ€︿笅 reclaim.

姝?infrastructure 涔?tracks the usage 鐨?`mm_struct` 涔嬮棿
涓婁笅鏂?switches 鍥犳 璇?椤?琛?walkers 鍙?skip 杩涚▼ 璇?
鍏锋湁 宸茬粡 sleeping since the 鏈€鍚?iteration.

### Rmap/PT walk feedback

Searching the rmap 鐢ㄤ簬 PTEs 鏄犲皠 姣忎釜 椤?鍦?涓€涓?LRU 鍒楀嚭 (鍒?test
鍜?clear the accessed 浣? 鍙?涓?expensive 鍥犱负 椤?鏉ヨ嚜
涓嶅悓 VMAs (PA space) 鏄?涓?缂撳瓨 friendly 鍒?the rmap (VA
space). 鐢ㄤ簬 workloads mostly 浣跨敤 mapped 椤? searching the rmap
鍙?incur the highest CPU cost 鍦?the reclaim path.

`lru_gen_look_around()` exploits spatial locality 鍒?reduce the
trips 杩涘叆 the rmap. 瀹?scans the adjacent PTEs 鐨?涓€涓?young PTE 鍜?
promotes hot 椤? 鑻?the scan 鏇炬槸 宸插畬鎴?cacheline efficiently, 瀹?
adds the PMD 鏉＄洰 pointing 鍒?the PTE 琛?鍒?the Bloom filter. 姝?
forms 涓€涓?feedback loop 涔嬮棿 the eviction 鍜?the aging.

### Bloom filters

Bloom filters 鏄?涓€涓?space 鍜?鍐呭瓨 efficient 鏁版嵁 缁撴瀯浣?鐢ㄤ簬 set
membership test, i.e., test 鑻?涓€涓?element 鏄?涓?鍦?the set 鎴?鍙?涓?
鍦?the set.

鍦?the eviction path, specifically, 鍦?`lru_gen_look_around()`, 鑻?涓€涓?
PMD 鍏锋湁 涓€涓?sufficient 鏁板瓧 鐨?hot 椤? 鍏?鍦板潃 鏄?placed 鍦?the
filter. 鍦?the aging path, set membership means 璇?the PTE range
灏?涓?scanned 鐢ㄤ簬 young 椤?

娉ㄦ剰 璇?Bloom filters 鏄?probabilistic 鍦?set membership. 鑻?涓€涓?test
鏄?false positive, the cost 鏄?涓€涓?棰濆 scan 鐨?涓€涓?range 鐨?PTEs,
鍏?鍙?yield hot 椤?anyway. 鍙傛暟 鐨?the filter itself 鍙?
control the false positive rate 鍦?the limit.

### PID 鎺у埗鍣?

涓€涓?feedback loop modeled 涔嬪悗 the Proportional-Integral-Derivative
(PID) 鎺у埗鍣?monitors refaults 鍦ㄢ€︿笂 anon 鍜?鏂囦欢 types 鍜?
decides 鍏?绫诲瀷 鍒?evict 褰?涓よ€?types 鏄?鍙敤 鏉ヨ嚜 the
鐩稿悓 generation.

The PID 鎺у埗鍣?uses generations rather 姣?the wall clock 浣滀负 the
time domain 鍥犱负 涓€涓?CPU 鍙?scan 椤?鍦?涓嶅悓 rates 鍦ㄢ€︿笅
varying 鍐呭瓨 pressure. 瀹?calculates 涓€涓?moving average 鐢ㄤ簬 姣忎釜 鏂?
generation 鍒?avoid 姝ｅ湪 permanently locked 鍦?涓€涓?suboptimal 鐘舵€?

### Memcg LRU

涓€涓?memcg LRU 鏄?涓€涓?per-node LRU 鐨?memcgs. 瀹冩槸 涔?涓€涓?LRU 鐨?LRUs,
since 姣忎釜 node 鍜?memcg combination 鍏锋湁 涓€涓?LRU 鐨?folios (鍙傝
`mem_cgroup_lruvec()`). 鍏?goal 鏄?鍒?improve the scalability 鐨?
鍏ㄥ眬 reclaim, 鍏?鏄?critical 鍒?system-wide 鍐呭瓨 overcommit 鍦?
鏁版嵁 centers. 娉ㄦ剰 璇?memcg LRU 浠?applies 鍒?鍏ㄥ眬 reclaim.

The 鍩烘湰 缁撴瀯浣?鐨?涓€涓?memcg LRU 鍙?涓?understood 鐢?涓€涓?analogy 鍒?
the active/inactive LRU (鐨?folios):

1. 瀹?鍏锋湁 the young 鍜?the 鏃?(generations), i.e., the counterparts
   鍒?the active 鍜?the inactive;
2. The increment 鐨?`max_seq` triggers promotion, i.e., the
   counterpart 鍒?activation;
3. 鍏朵粬 浜嬩欢 trigger similar 鎿嶄綔, e.g., offlining 涓€涓?memcg
   triggers demotion, i.e., the counterpart 鍒?deactivation.

灏扁€﹁€岃█ 鍏ㄥ眬 reclaim, 瀹?鍏锋湁 two distinct 鐗规€?

1. Sharding, 鍏?allows 姣忎釜 绾跨▼ 鍒?鍚姩 鍦?涓€涓?random memcg (鍦?
   the 鏃?generation) 鍜?improves parallelism;
2. Eventual fairness, 鍏?allows direct reclaim 鍒?bail out 鍦?灏?
   鍜?reduces latency 鏃?affecting fairness 鍦ㄢ€︿笂 涓€浜?time.

灏扁€﹁€岃█ traversing memcgs 鏈熼棿 鍏ㄥ眬 reclaim, 瀹?improves the
best-case complexity 鏉ヨ嚜 O(n) 鍒?O(1) 鍜?鎵ц 涓?affect the
worst-case complexity O(n). 鍥犳, 鍦?average, 瀹?鍏锋湁 涓€涓?sublinear
complexity.

### Summary

The multi-gen LRU (鐨?folios) 鍙?涓?disassembled 杩涘叆 the 浠ヤ笅
parts:

- Generations
- Rmap walks
- 椤?琛?walks 閫氳繃 `mm_struct` 鍒楀嚭
- Bloom filters 鐢ㄤ簬 rmap/PT walk feedback
- PID 鎺у埗鍣?鐢ㄤ簬 refault feedback

The aging 鍜?the eviction form 涓€涓?producer-consumer 鍨嬪彿;
specifically, the latter drives the former 鐢?the sliding window 鍦ㄢ€︿笂
generations. 涔嬪唴 the aging, rmap walks drive 椤?琛?walks 鐢?
inserting hot densely populated 椤?琛?鍒?the Bloom filters.
涔嬪唴 the eviction, the PID 鎺у埗鍣?uses refaults 浣滀负 the feedback
鍒?select types 鍒?evict 鍜?tiers 鍒?protect.
