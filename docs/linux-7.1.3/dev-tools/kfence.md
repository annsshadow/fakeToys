
## 内核 Electric-Fence (KFENCE)


Kernel Electric-Fence（KFENCE）是一种低开销、基于采样的记忆体安全错误检测器。KFENCE 可检测堆越界访问、释放后使用（use-after-free）以及无效释放（invalid-free）错误。

KFENCE 设计为可在生产内核中启用，并且性能开销接近于零。与 KASAN 相比，KFENCE 以精度换取性能。KFENCE 设计的主要动机在于：只要总运行时间足够长，KFENCE 就能检测出那些通常不会被非生产测试负载所执行到的代码路径中的缺陷。快速累积足够长总运行时间的一种方式是：将该工具部署到大规模机器集群中。

### 用法


```
    CONFIG_KFENCE=y

```

要构建带 KFENCE 支持但默认禁用的内核（要启用则设置
```
    CONFIG_KFENCE=y
    CONFIG_KFENCE_SAMPLE_INTERVAL=0

```

KFENCE 还提供若干其他配置选项用于定制行为（更多信息请参阅 `lib/Kconfig.kfence` 中相应的帮助文本）。

#### 调优性能


最重要的参数是 KFENCE 的采样间隔，它可以通过内核引导参数 `kfence.sample_interval`（单位为毫秒）来设置。采样间隔决定了堆分配被 KFENCE 保护起来的频率。默认值可通过 Kconfig 选项 `CONFIG_KFENCE_SAMPLE_INTERVAL` 配置。设置 `kfence.sample_interval=0` 将禁用 KFENCE。

采样间隔控制一个定时器，该定时器负责建立 KFENCE 分配。默认情况下，为了保持实际采样间隔的可预测性，普通定时器在系统完全空闲时也会唤醒 CPU。这在功耗受限的系统上可能并不理想。引导参数 `kfence.deferrable=1` 则会改用“可延迟（deferrable）”定时器，它不会在空闲系统上强制唤醒 CPU，但代价是采样间隔变得不可预测。默认值可通过 Kconfig 选项 `CONFIG_KFENCE_DEFERRABLE` 配置。

   KUnit 测试套件在使用可延迟定时器时极有可能失败，因为它目前会造成非常不可预测的采样间隔。

默认情况下，KFENCE 在每个采样间隔内只对 1 个堆分配进行采样。**突发模式（Burst mode）** 允许对连续的堆分配进行采样，其中内核引导参数 `kfence.burst` 可设为一个非零值，表示在一个采样间隔内的**额外**连续分配数；设置 `kfence.burst=N` 意味着每个采样间隔内会通过 KFENCE 尝试 `1 + N` 个连续分配。

KFENCE 内存池大小固定，如果内存池耗尽，则不再进行进一步的 KFENCE 分配。通过 `CONFIG_KFENCE_NUM_OBJECTS`（默认 255）可以控制可用受保护对象的数量。每个对象需要 2 个页，一个用于对象本身，另一个用作保护页（guard page）；对象页与保护页交错排列，因此每个对象页都被两个保护页所包围。

```
    ( #objects + 1 ) * 2 * PAGE_SIZE

```

使用默认配置，并假设页大小为 4 KiB，则 KFENCE 内存池占用 2 MiB。

注意：在支持大页（huge pages）的架构上，KFENCE 会确保内存池使用大小为 `PAGE_SIZE` 的页。这将导致分配额外的页表。

#### 错误报告


引导参数 `kfence.fault` 可用于控制检测到 KFENCE 错误时的行为：

- `kfence.fault=report`：打印错误报告并继续（默认）。
- `kfence.fault=oops`：打印错误报告并触发 oops。
- `kfence.fault=panic`：打印错误报告并触发 panic。

```
    ==================================================================
    BUG: KFENCE: out-of-bounds read in test_out_of_bounds_read+0xa6/0x234

    Out-of-bounds read at 0xffff8c3f2e291fff (1B left of kfence-#72):
     test_out_of_bounds_read+0xa6/0x234
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#72: 0xffff8c3f2e292000-0xffff8c3f2e29201f, size=32, cache=kmalloc-32

    allocated by task 484 on cpu 0 at 32.919330s:
     test_alloc+0xfe/0x738
     test_out_of_bounds_read+0x9b/0x234
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 0 PID: 484 Comm: kunit_try_catch Not tainted 5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

报告头部提供了所涉及访问函数的简要摘要。其后跟随关于该访问及其来源的更详细信息。注意，只有在使用了内核命令行选项 `no_hash_pointers` 时才会显示真实的内核地址。

```
    ==================================================================
    BUG: KFENCE: use-after-free read in test_use_after_free_read+0xb3/0x143

    Use-after-free read at 0xffff8c3f2e2a0000 (in kfence-#79):
     test_use_after_free_read+0xb3/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#79: 0xffff8c3f2e2a0000-0xffff8c3f2e2a001f, size=32, cache=kmalloc-32

    allocated by task 488 on cpu 2 at 33.871326s:
     test_alloc+0xfe/0x738
     test_use_after_free_read+0x76/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    freed by task 488 on cpu 2 at 33.871358s:
     test_use_after_free_read+0xa8/0x143
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 2 PID: 488 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

```
    ==================================================================
    BUG: KFENCE: invalid free in test_double_free+0xdc/0x171

    Invalid free of 0xffff8c3f2e2a4000 (in kfence-#81):
     test_double_free+0xdc/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#81: 0xffff8c3f2e2a4000-0xffff8c3f2e2a401f, size=32, cache=kmalloc-32

    allocated by task 490 on cpu 1 at 34.175321s:
     test_alloc+0xfe/0x738
     test_double_free+0x76/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    freed by task 490 on cpu 1 at 34.175348s:
     test_double_free+0xa8/0x171
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 1 PID: 490 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

KFENCE 还在对象保护页的另一侧使用基于模式的红区（redzone），以检测对象未受保护一侧的越界写入。
```
    ==================================================================
    BUG: KFENCE: memory corruption in test_kmalloc_aligned_oob_write+0xef/0x184

    Corrupted memory at 0xffff8c3f2e33aff9 [ 0xac . . . . . . ] (in kfence-#156):
     test_kmalloc_aligned_oob_write+0xef/0x184
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    kfence-#156: 0xffff8c3f2e33afb0-0xffff8c3f2e33aff8, size=73, cache=kmalloc-96

    allocated by task 502 on cpu 7 at 42.159302s:
     test_alloc+0xfe/0x738
     test_kmalloc_aligned_oob_write+0x57/0x184
     kunit_try_run_case+0x61/0xa0
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x176/0x1b0
     ret_from_fork+0x22/0x30

    CPU: 7 PID: 502 Comm: kunit_try_catch Tainted: G    B             5.13.0-rc3+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================

```

对于此类错误，会显示发生损坏的地址以及被无效写入的字节（相对于地址的偏移）；在该表示中，'.' 表示未被触碰的字节。在上面示例中 `0xac` 是写入偏移 0 处无效地址的值，其余的 '.' 表示后续字节未被触碰。注意，只有在内核以 `no_hash_pointers` 引导时才会显示真实值；否则为避免信息泄露，会使用 '!' 来表示被无效写入的字节。

最后，KFENCE 还可能报告对任何受保护页的无效访问，而此时无法确定关联的对象，例如当相邻
```
    ==================================================================
    BUG: KFENCE: invalid read in test_invalid_access+0x26/0xe0

    Invalid read at 0xffffffffb670b00a:
     test_invalid_access+0x26/0xe0
     kunit_try_run_case+0x51/0x85
     kunit_generic_run_threadfn_adapter+0x16/0x30
     kthread+0x137/0x160
     ret_from_fork+0x22/0x30

    CPU: 4 PID: 124 Comm: kunit_try_catch Tainted: G        W         5.8.0-rc6+ #7
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.13.0-1 04/01/2014
    ==================================================================

```

#### DebugFS 接口


一些调试信息通过 debugfs 暴露出来：

- 文件 `/sys/kernel/debug/kfence/stats` 提供运行时统计信息。

- 文件 `/sys/kernel/debug/kfence/objects` 提供通过 KFENCE 分配的对象列表，包括那些已释放但仍受保护的对象。

### 实现细节


受保护的分配基于采样间隔建立。采样间隔到期后，下一次通过主分配器（SLAB 或 SLUB）进行的分配会返回一个来自 KFENCE 对象池的受保护分配（支持最大到 PAGE_SIZE 的分配大小）。此时定时器被重置，并在该间隔到期后再建立下一次分配。

当使用 `CONFIG_KFENCE_STATIC_KEYS=y` 时，KFENCE 分配通过主分配器快速路径的静态分支（static branch），依赖静态键（static keys）基础设施进行“门控”。该静态分支会被切换，以将分配重定向到 KFENCE。根据采样间隔、目标工作负载以及系统架构的不同，这可能比简单的动态分支性能更好。建议进行仔细的基准测试。

每个 KFENCE 对象都驻留在一个专用页上，位于随机选择的左边界或右边界页处。对象页左右两侧的页是“保护页”，其属性被改为受保护状态，并对任何尝试的访问产生页错误。此类页错误随后被 KFENCE 拦截，KFENCE 通过报告一次越界访问来优雅地处理该错误，并将该页标记为可访问，以便引发错误的代码能够（错误地）继续执行（设置 `panic_on_warn` 则改为触发 panic）。

为了检测对象页本身内部的内存越界写入，KFENCE 还使用了基于模式的红区。对于每个对象页，会为所有非对象内存设置一个红区。对于典型的对齐方式，红区只需要在对象的未受保护一侧设置。由于 KFENCE 必须遵守缓存所请求的对齐方式，特殊的对齐可能导致对象任意一侧出现未受保护的间隙，所有这些间隙都会被设为红区。

```
    ---+-----------+-----------+-----------+-----------+-----------+---
       | xxxxxxxxx | O :       | xxxxxxxxx |       : O | xxxxxxxxx |
       | xxxxxxxxx | B :       | xxxxxxxxx |       : B | xxxxxxxxx |
       | x GUARD x | J : RED-  | x GUARD x | RED-  : J | x GUARD x |
       | xxxxxxxxx | E :  ZONE | xxxxxxxxx |  ZONE : E | xxxxxxxxx |
       | xxxxxxxxx | C :       | xxxxxxxxx |       : C | xxxxxxxxx |
       | xxxxxxxxx | T :       | xxxxxxxxx |       : T | xxxxxxxxx |
    ---+-----------+-----------+-----------+-----------+-----------+---

```

KFENCE 对象被释放时，该对象的页会再次被保护，并且对象被标记为已释放。对该对象的任何进一步访问都会引发错误，KFENCE 会报告一次释放后使用访问。已释放的对象被插入到 KFENCE 空闲链表的尾部，以便最近最少释放的对象被优先复用，从而增加检测到最近释放对象的释放后使用问题的概率。

如果内存池利用率达到 75%（默认）或以上，为降低内存池最终被已分配对象完全占满的风险，同时保证分配的多样化覆盖，KFENCE 会限制当前已覆盖的、来自同一来源的分配进一步填满内存池。一次分配的“来源”基于其部分分配栈回溯。一个副作用是，这也限制了来自同一来源的频繁长生命周期分配（例如页缓存）永久填满内存池，而这是导致内存池变满、采样分配率降为零的最常见风险。开始限制当前已覆盖分配的阈值可以通过引导参数 `kfence.skip_covered_thresh`（内存池使用率 %）进行配置。

### 接口


以下描述分配器以及页处理代码用于建立和处理 KFENCE 分配的函数。

   :functions: is_kfence_address
               kfence_shutdown_cache
               kfence_alloc kfence_free __kfence_free
               kfence_ksize kfence_object_start
               kfence_handle_page_fault

### 相关工具


在用户空间中，`GWP-ASan <http://llvm.org/docs/GwpAsan.html>`_ 采用了类似的方法。GWP-ASan 同样依赖保护页和采样策略来大规模检测内存不安全缺陷。KFENCE 的设计直接受到 GWP-ASan 的影响，可视为其内核版本兄弟。另一个类似但非采样、并且也启发了 “KFENCE” 这一名称的方法，可以在用户空间的 `Electric Fence Malloc Debugger <https://linux.die.net/man/3/efence>`_ 中找到。

在内核中，存在若干用于调试内存访问错误的工具，特别是 KASAN 能够检测出 KFENCE 所能检测的所有缺陷类别。虽然 KASAN 借助编译器插桩更为精确，但这会带来性能代价。

值得强调的是，KASAN 与 KFENCE 是互补的，面向不同的目标环境。例如，在存在测试用例或复现器的情况下，KASAN 是更好的调试辅助手段：由于 KFENCE 检测到错误的概率较低，使用 KFENCE 来调试需要更多的精力。然而，那些无法承担启用 KASAN 成本的大规模部署，将受益于使用 KFENCE 来发现那些未被测试用例或模糊测试执行到的代码路径中的缺陷。
