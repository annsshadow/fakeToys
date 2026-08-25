
## 伪共享（False Sharing

## 什么是伪共

伪共享与在多CPU 缓存中维护同一缓存行数据一致性的缓存机制有关；其学术定义[^1^]_。考虑一个带有如下结构体struct
```
	struct foo {
		refcount_t refcount;
		...
		char name[16];
	} ____cacheline_internodealigned_in_smp;
```

```
                +-----------+                     +-----------+
                |   CPU 0   |                     |   CPU 1   |
                +-----------+                     +-----------+
               /                                        |
              /                                         |
             V                                          V
         +----------------------+             +----------------------+
         | A      B             | Cache 0     | A       B            | Cache 1
         +----------------------+             +----------------------+
                             |                  |
  ---------------------------+------------------+-----------------------------
                             |                  |
                           +----------------------+
                           |                      |
                           +----------------------+
              Main Memory  | A       B            |
                           +----------------------+
```

“refcount”被频繁修改，但“name”在对象创建时设置一次后便再也不修改。当许多 CPU 同时访问“foo”，且“refcount”只被一CPU 频繁递增、“name”被其他 CPU 读取时，所有这些读取的 CPU 都不得不因为“共享”而一遍又一遍地重新加载整个缓存行，即便“name”从未改变
由伪共享导致性能回退的真实案例有很多。其中一个是 mm_struct 结构体内部的读写信号量“mmap_lock”，其缓存行布局变化引发过一次回退，Linus [^2^]_ 中进行了分析
有害的伪共享有两个关键因素：

- 被许CPU 访问（共享）的全局数据
- 在对数据的并发访问中，至少存在一个写操作：写/写或读情况
这种共享可能来自完全不相干的內核组件，也可能来自同一内核组件的不同代码路径
## 伪共享的陷阱


在过去，当一个平台只有一颗或少数几颗 CPU 时，热点数据成员会被有意地放在同一个缓存行中，以使它们保持缓存热态并节省缓存TLB，例如一把锁以及受它保护的数据。但对于近期拥有数百CPU 的大型系统而言，当锁竞争激烈时这可能不再奏效，因为锁的持有CPU 可能写入数据，而其CPU 正忙于自旋等待该锁
回顾过去的案例，伪共享有几种频繁出现的模式：

- 锁（spinlock/mutex/semaphore）与受其保护的数据被有意放在同一个缓存行中- 全局数据被放在一起放在同一个缓存行中。某些内核子系统有许多小尺寸 字节）的全局参数，很容易成组放在一起并装入同一个缓存行- 大数据结构的成员随机地挨在一起而未被注意（缓存行通常64 字节或更大），例如“mem_cgroup”结构体
“缓解措施”一节提供了真实世界的示例
除非特意去检查，否则伪共享很容易发生；对于性能关键型负载，运行专门工具来检测影响性能的伪共享情形并相应优化是很有价值的
## 如何检测和分析伪共

perf record/report/stat 被广泛用于性能调优，一旦检测到热点，可以进一步使用像“perf-c2c”和“pahole”这样的工具来检测和定位可能的伪共享数据结构。“addr2line”在存在多层内联函数时也很擅长解码指令指针
perf-c2c 可以捕获命中伪共享最多的缓存行，以及访问该缓存行的解码函数（文件行号），

```
  $ perf c2c record -ag sleep 3
  $ perf c2c report --call-graph none -k vmlinux
```

在测试期间运行上述命令，针对 will-it-scale tlb_flush1 用例
```
  Total records                     :    1658231
  Locked Load/Store Operations      :      89439
  Load Operations                   :     623219
  Load Local HITM                   :      92117
  Load Remote HITM                  :        139

  #----------------------------------------------------------------------
      4        0     2374        0        0        0  0xff1100088366d880
  #----------------------------------------------------------------------
    0.00%   42.29%    0.00%    0.00%    0.00%    0x8     1       1  0xffffffff81373b7b         0       231       129     5312        64  [k] __mod_lruvec_page_state    [kernel.vmlinux]  memcontrol.h:752   1
    0.00%   13.10%    0.00%    0.00%    0.00%    0x8     1       1  0xffffffff81374718         0       226        97     3551        64  [k] folio_lruvec_lock_irqsave  [kernel.vmlinux]  memcontrol.h:752   1
    0.00%   11.20%    0.00%    0.00%    0.00%    0x8     1       1  0xffffffff812c29bf         0       170       136      555        64  [k] lru_add_fn                 [kernel.vmlinux]  mm_inline.h:41     1
    0.00%    7.62%    0.00%    0.00%    0.00%    0x8     1       1  0xffffffff812c3ec5         0       175       108      632        64  [k] release_pages              [kernel.vmlinux]  mm_inline.h:41     1
    0.00%   23.29%    0.00%    0.00%    0.00%   0x10     1       1  0xffffffff81372d0a         0       234       279     1051        64  [k] __mod_memcg_lruvec_state   [kernel.vmlinux]  memcontrol.c:736   1
```

关于 perf-c2c 的一篇不错的介绍[^3^]_
“pahole”以缓存行为粒度解码数据结构布局。用户可以将 perf-c2c 输出中的偏移pahole 的解码结果相匹配，从而定位确切的数据成员。对于全局数据，用户可以在 System.map 中搜索该数据地址
## 可能的缓解措

伪共享并不总是需要缓解。伪共享的缓解应该在性能收益与复杂度及空间消耗之间取得平衡。有时，较低的性能是可以接受的，没有必要对每一个很少使用的数据结构或冷数据路径进行过度优化
随着核心数量增加，伪共享损害性能的情况出现得更加频繁。由于这些有害影响，跨多个子系统（如网络和内存管理）提出了许多补丁并已被合并。一些常见的缓解措施（附示例）如下：

- 将热点全局数据放在其自己专用的缓存行中，即使它只是一个“short”类型。缺点是会消耗更多内存、缓存行TLB 项
  - Commit 91b6d3256356（“net: cache align tcp_memory_allocated, tcp_sockets_allocated”）

- 重组数据结构，将相互干扰的成员分隔到不同的缓存行中。一个缺点是可能会引入其他成员之间新的伪共享
  - Commit 802f1d522d5f（“mm: page_counter: re-layout structure to reduce false sharing”）

- 在可能的情况下用“读”替换“写”，尤其是在循环中。例如对某些全局变量，使用比较（读）再写，而不
```
	if (!test_bit(XXX))
		set_bit(XXX);
```

  而不要直接“set_bit(XXX);”，对于 atomic_t 数据类似：：

```
	if (atomic_read(XXX) == AAA)
		atomic_set(XXX, BBB);
```

  - Commit 7b1002f7cfe5（“bcache: fixup bcache_dev_sectors_dirty_add() multithreaded CPU false sharing”）
  - Commit 292648ac5cf1（“mm: gup: allow FOLL_PIN to scale in SMP”）

- 在可能的情况下将热点全局数据转为“per-cpu 数据 + 全局数据”，或合理地提高per-cpu 数据同步到全局数据的阈值，以减少或推迟对该全局数据的“写”
  - Commit 520f897a3554（“ext4: use percpu_counters for extent_status cache hits/misses”）
  - Commit 56f3547bfa4d（“mm: adjust vm_committed_as_batch according to vm overcommit policy”）

当然，所有缓解措施都应经过仔细验证，确保不会产生副作用。为避免在编码时引入伪共享，最好做到：

- 注意缓存行边- 将几乎只读的字段归为一- 将同时写入的字段归为一- 将频繁读和频繁写的字段分隔在不同的缓存行上
并且最好添加一条注释说明关于伪共享的考量
有一点需要注意的是，有时即使检测到并解决了一个严重的伪共享，性能可能仍无明显改善，因为热点切换到了新的位置
## 杂项


一个未解决的问题是，内核有一个可选的数据结构随机化机制，它也会随机化数据成员之间缓存行共享的状况