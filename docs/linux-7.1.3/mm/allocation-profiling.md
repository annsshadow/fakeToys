
## 内存分配性能分析（MEMORY ALLOCATION PROFILING）


对所有内存分配进行低开销（适用于生产环境）的记账，按文件与行号跟踪。

用法：
kconfig 选项：
- CONFIG_MEM_ALLOC_PROFILING

- CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT

- CONFIG_MEM_ALLOC_PROFILING_DEBUG
  为那些因缺少注解而未被记账的分配增加警告

启动参数：
  sysctl.vm.mem_profiling={0|1|never}[,compressed]

  当设置为 "never" 时，内存分配性能分析的开销被最小化，并且无法在运行时启用（sysctl 变为只读）。
  当 CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT=y 时，默认值为 "1"。
  当 CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT=n 时，默认值为 "never"。
  "compressed" 可选参数会尝试以紧凑格式存储页标记引用，避免使用页扩展。这会改善性能与内存占用，
  但可能会因系统配置而失败。如果压缩失败，会发出警告并禁用内存分配性能分析。

sysctl：
  /proc/sys/vm/mem_profiling

  1：启用内存性能分析。

  0：禁用内存性能分析。

  默认值取决于 CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT。

  当 CONFIG_MEM_ALLOC_PROFILING_DEBUG=y 时，此控件为只读，以避免在性能分析被禁用时进行的分配、
  以及在它被启用时释放所产生的警告。

运行时信息：
  /proc/allocinfo

```

  root@moria-kvm:~# sort -g /proc/allocinfo|tail|numfmt --to=iec
        2.8M    22648 fs/kernfs/dir.c:615 func:__kernfs_new_node
        3.8M      953 mm/memory.c:4214 func:alloc_anon_folio
        4.0M     1010 drivers/staging/ctagmod/ctagmod.c:20 [ctagmod] func:ctagmod_start
        4.1M        4 net/netfilter/nf_conntrack_core.c:2567 func:nf_ct_alloc_hashtable
        6.0M     1532 mm/filemap.c:1919 func:__filemap_get_folio
        8.8M     2785 kernel/fork.c:307 func:alloc_thread_stack_node
         13M      234 block/blk-mq.c:3421 func:blk_mq_alloc_rqs
         14M     3520 mm/mm_init.c:2530 func:alloc_large_system_hash
         15M     3656 mm/readahead.c:247 func:page_cache_ra_unbounded
         55M     4887 mm/slub.c:2259 func:alloc_slab_page
        122M    31168 mm/page_ext.c:270 func:alloc_page_ext

```
## 工作原理


内存分配性能分析建立在代码标记（code tagging）之上，代码标记是一个用于声明静态结构体（通常以某种方式
描述文件与行号，因此称为代码标记）、并在运行时查找并操作它们的库——例如遍历它们以在 debugfs/procfs 中打印。

要为一次分配调用增加记账，我们将其替换为一个宏调用 alloc_hooks()，该宏：
- 声明一个代码标记
- 在其 task_struct 中暂存一个指向它的指针
- 调用真正的分配函数
- 最后，将 task_struct 的分配标记指针恢复为其先前的值。

这使得 alloc_hooks() 调用可以嵌套，以最近的一次生效。这对于 mm/ 代码内部、不属于外层分配上下文、应当
单独计数的分配很重要：例如，slab 对象扩展向量，或者当 slab 从页分配器分配页时。

因此，正确的用法需要确定分配调用栈中的哪个函数应当被打标记。有许多辅助函数本质上只是封装了例如 kmalloc()
并多做了一点工作，然后在多处被调用；我们通常希望记账发生在这些辅助函数的调用者中，而不是在辅助函数自身中。

要修复某个给定的辅助函数，例如 foo()，请执行以下操作：
- 将其分配调用切换为 _noprof() 版本，例如 kmalloc_noprof()

- 将其重命名为 foo_noprof()

- 定义一个 foo() 的宏版本，如下所示：

  #define foo(...) alloc_hooks(foo_noprof(__VA_ARGS__))

也可以在你自己的数据结构中暂存一个指向分配标记的指针。

当你正在实现一个“代表”其他某些代码进行分配的通用数据结构时——例如 rhashtable 代码——就这样做。这样，
我们就不必在 /proc/allocinfo 中看到 rhashtable.c 的一大行，而是可以按 rhashtable 类型拆分它。

为此：
- 像其他任何分配函数一样，挂接你的数据结构的 init 函数。

- 在你的 init 函数内部，使用便捷宏 alloc_tag_record() 在你的数据结构中记录分配标记。

- 然后，对你的分配使用以下形式：
  alloc_hooks_tag(ht->your_saved_tag, kmalloc_noprof(...))
