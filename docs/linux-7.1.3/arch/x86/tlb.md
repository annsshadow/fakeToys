
## TLB


当内核取消映射或修改变内存范围的属性时，它有两个选择
 1. 用两条指令的序列刷新整个 TLB。这是一个快速的操作，但它会造成附带损害：来自我们试    刷新的区域以外的 TLB 条目会被销毁，之后必须以一定代价重新填充 2. 使用 invlpg 指令一次使单个页失效。这潜在地可能耗费更多指令，但它是一个更精确的操作，
    不会对其TLB 条目造成附带损害
采用哪种方法取决于若干因素：

 1. 正在执行的刷新的大小。刷新整个地址空间显然更适合通过刷新整个 TLB 来完成，而不是做
    2^48/PAGE_SIZE 次单独的刷新 2. TLB 的内容。如TLB 为空，那么全局刷新不会造成附带损害，而所有单独的刷新都将沦为
    被浪费的工作 3. TLB 的大小。TLB 越大，完整刷新造成的附带损害越多。因此，TLB 越大，单独刷新看起来越有吸引力    数据与指令拥有各自独立的 TLB，不同的页大小也是如此 4. 微架构。在现代 CPU 上，TLB 已成为多级缓存，相对于单页刷新，全局刷新变得更加昂贵
显然内核无法知道所有这些事情，尤其是在给定刷新期间 TLB 的内容。刷新的大小也会随着工作负载
的不同而大不相同。基本上没有“正确”的选择点
如果你在性能剖析中看invlpg 指令（或其附近的指令）排名很高，说明你可能做了过多的单独失效如果你认为单独的失效
```

	/sys/kernel/debug/x86/tlb_single_page_flush_ceiling

```
正在进行中，这会使我们在更多情况下执行全局刷新。将其降低到 0 将禁用单独刷新的使用将其设为 1 是一种非常保守的设置，正常情况下绝不需要为 0
尽管x86 上，一次单独的刷新保证刷新整整 2MB [^1^]_，hugetlbfs 总是使用完整的刷新。THP 处理与普通内存完全相同
你可能会看到 flush_tlb_mm_range() 内部invlpg 出现在性能剖析中，或者你可以使用
trace_tlb_flush() 跟踪点来确定刷新操作耗时多久
本质上，你是在做 invlpg 所花费的周期与之后重新填充 TLB 所花费的周期之间进行权衡
你可以使用以下方式衡TLB 重新填充的代```

  perf stat -e
    cpu/event=0x8,umask=0x84,name=dtlb_load_misses_walk_duration/,
    cpu/event=0x8,umask=0x82,name=dtlb_load_misses_walk_completed/,
    cpu/event=0x49,umask=0x4,name=dtlb_store_misses_walk_duration/,
    cpu/event=0x49,umask=0x2,name=dtlb_store_misses_walk_completed/,
    cpu/event=0x85,umask=0x4,name=itlb_misses_walk_duration/,
    cpu/event=0x85,umask=0x2,name=itlb_misses_walk_completed/

```
这适用Ivy Bridge 时代CPU（i5-3320M）。不同的 CPU 可能有不同名称的计数器，但它们至应以某种形式存在。你可以使用 pmu-tools 'ocperf list'
（https://github.com/andikleen/pmu-tools）来查找给定 CPU 的正确计数器
   说明：“即使对于大4 KBytes 的页，执行一INVLPG 也足够了。