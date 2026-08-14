
## 内核并发检查器（KCSAN）


内核并发检查器（KCSAN，Kernel Concurrency Sanitizer）是一个动态竞态检测器，它依赖编译期插桩，并使用基于观察点（watchpoint）的采样方法来检测竞态。KCSAN 的主要目的是检测 `data races`_（数据竞争）。

### 用法


KCSAN 同时受 GCC 与 Clang 支持。对于 GCC 我们需要 11 或更高版本，对于 Clang 也需要 11 或更高版本。

```
    CONFIG_KCSAN = y
```

KCSAN 还提供了若干其他配置选项以定制行为（更多信息请参阅 `lib/Kconfig.kcsan` 中相应的帮助文本）。

#### 错误报告


```
    ==================================================================
    BUG: KCSAN: data-race in test_kernel_read / test_kernel_write

    write to 0xffffffffc009a628 of 8 bytes by task 487 on cpu 0:
     test_kernel_write+0x1d/0x30
     access_thread+0x89/0xd0
     kthread+0x23e/0x260
     ret_from_fork+0x22/0x30

    read to 0xffffffffc009a628 of 8 bytes by task 488 on cpu 6:
     test_kernel_read+0x10/0x20
     access_thread+0x89/0xd0
     kthread+0x23e/0x260
     ret_from_fork+0x22/0x30

    value changed: 0x00000000000009a6 -> 0x00000000000009b2

    Reported by Kernel Concurrency Sanitizer on:
    CPU: 6 PID: 488 Comm: access_thread Not tainted 5.12.0-rc2+ #1
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================
```

报告的头部提供了参与竞态的函数的简短摘要。其后是对应的访问类型以及参与数据竞争的两个线程的栈回溯。如果 KCSAN 还观察到了值的变化，则观察到的旧值与新值会分别显示在 "value changed"（值已改变）一行中。

```
    ==================================================================
    BUG: KCSAN: data-race in test_kernel_rmw_array+0x71/0xd0

    race at unknown origin, with read to 0xffffffffc009bdb0 of 8 bytes by task 515 on cpu 2:
     test_kernel_rmw_array+0x71/0xd0
     access_thread+0x89/0xd0
     kthread+0x23e/0x260
     ret_from_fork+0x22/0x30

    value changed: 0x0000000000002328 -> 0x0000000000002329

    Reported by Kernel Concurrency Sanitizer on:
    CPU: 2 PID: 515 Comm: access_thread Not tainted 5.12.0-rc2+ #1
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
    ==================================================================
```

当无法确定另一个竞态线程，但由于被监视内存位置的数据值发生了变化而推断出存在竞态时，会生成此类报告。这类报告总会显示一行 "value changed"。这类报告常见的原因是竞态线程中缺少插桩，但也可能由例如 DMA 访问引起。此类报告仅在 `CONFIG_KCSAN_REPORT_RACE_UNKNOWN_ORIGIN=y` 时显示，该选项默认开启。

#### 选择性分析


可能希望对特定的访问、函数、编译单元或整个子系统禁用数据竞争检测。对于静态黑名单，可使用以下选项：

- KCSAN 理解 `data_race(expr)` 注解，它告诉 KCSAN：应忽略 `expr` 中访问所引起的任何数据竞争，并且遇到数据竞争时的结果行为被认为是安全的。更多信息请参阅 `"Marking Shared-Memory Accesses" in the LKMM`_。

- 与 `data_race(...)` 类似，类型限定符 `__data_racy` 可用于标注：对某变量的所有访问所导致的数据竞争都是有意为之的

```
    struct foo {
        ...
        int __data_racy stats_counter;
        ...
    };
```

- 禁用整个函数的数据竞争检测可通过

```
    __no_kcsan
    void foo(void) {
        ...
```

  要动态限制对哪些函数生成报告，请参阅 `DebugFS interface`_（DebugFS 接口）的黑名单/白名单功能。

```
```

- 要对特定编译单元禁用数据竞争检测，可添加

```
    KCSAN_SANITIZE_file.o := n
```

- 要对列出在某个文件中的所有编译单元禁用数据竞争检测，可使用

```
    KCSAN_SANITIZE := n
```

此外，还可以根据偏好告知 KCSAN 显示或隐藏整类数据竞争。这些可通过以下 Kconfig 选项更改：

- `CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY`：若启用，且通过 watchpoint 观察到一个冲突的写入，但内存位置的数据值经观察保持不变，则不报告该数据竞争。

- `CONFIG_KCSAN_ASSUME_PLAIN_WRITES_ATOMIC`：默认假设不超过字长的对齐普通写入是原子的。即假设此类写入不会受到导致数据竞争的不安全编译器优化影响。该选项会使 KCSAN 不报告那些唯一普通访问是不超过字长的对齐写入的冲突所导致的数据竞争。

- `CONFIG_KCSAN_PERMISSIVE`：启用额外的宽松规则，以忽略某些类别的常见数据竞争。与上述不同，这些规则更为复杂，涉及值变化模式、访问类型和地址。该选项依赖于 `CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY=y`。详情请参阅 `kernel/kcsan/permissive.h`。建议只关注特定子系统报告而非整个内核的测试者与维护者禁用此选项。

要使用尽可能严格的规则，请选择 `CONFIG_KCSAN_STRICT=y`，它将 KCSAN 配置为尽可能贴近地遵循 Linux 内核内存一致性模型（LKMM）。

#### DebugFS 接口


文件 `/sys/kernel/debug/kcsan` 提供以下接口：

- 读取 `/sys/kernel/debug/kcsan` 会返回各种运行时统计信息。

- 向 `/sys/kernel/debug/kcsan` 写入 `on` 或 `off` 可分别开启或关闭 KCSAN。

- 向 `/sys/kernel/debug/kcsan` 写入 `!some_func_name` 会将 `some_func_name` 加入报告过滤列表，该列表（默认情况下）将黑名单处理那些顶层栈帧为列表中函数的数据竞争报告。

- 向 `/sys/kernel/debug/kcsan` 写入 `blacklist` 或 `whitelist` 可更改报告过滤行为。例如，黑名单功能可用于抑制频繁出现的数据竞争；白名单功能有助于复现与测试修复。

#### 性能调优


影响 KCSAN 整体性能与 bug 检测能力的核心参数以内核命令行参数的形式暴露，其默认值也可通过相应的 Kconfig 选项更改。

- `kcsan.skip_watch`（`CONFIG_KCSAN_SKIP_WATCH`）：在设置下一个 watchpoint 之前要跳过的每 CPU 内存操作次数。更频繁地设置 watchpoint 会提高观察到竞态的可能性。该参数对整体系统性能与竞态检测能力影响最为显著。

- `kcsan.udelay_task`（`CONFIG_KCSAN_UDELAY_TASK`）：对于任务，在 watchpoint 设置后停滞执行的微秒延迟。值越大，我们可能观察到竞态的窗口就越大。

- `kcsan.udelay_interrupt`（`CONFIG_KCSAN_UDELAY_INTERRUPT`）：对于中断，在 watchpoint 设置后停滞执行的微秒延迟。中断对延迟要求更严格，其延迟通常应小于为任务选择的延迟。

它们可在运行时通过 `/sys/module/kcsan/parameters/` 调整。

### 数据竞争


在一次执行中，如果两次内存访问发生**冲突**、它们在不同的线程中并发发生、且其中至少一次是**普通访问**，那么它们就构成一个**数据竞争**；如果两者访问同一内存位置且至少一个是写入，则它们**冲突**。更详尽的讨论与定义，请参阅 `"Plain Accesses and Data Races" in the LKMM`_。

#### 与 Linux 内核内存一致性模型（LKMM）的关系


LKMM 定义了各种内存操作的传播与排序规则，使开发者能够推理并发代码。最终这可以确定并发代码可能的执行情况，以及该代码是否无数据竞争。

KCSAN 知晓**带标记的原子操作**（`READ_ONCE`、`WRITE_ONCE`、`atomic_*` 等），以及内存屏障所隐含的部分排序保证。在 `CONFIG_KCSAN_WEAK_MEMORY=y` 时，KCSAN 会对加载或存储缓冲建模，并能检测缺失的 `smp_mb()`、`smp_wmb()`、`smp_rmb()`、`smp_store_release()`，以及所有带等价隐含屏障的 `atomic_*` 操作。

注意，KCSAN 不会报告所有因缺失内存排序而导致的数据竞争，特别是那些需要内存屏障来防止后续内存操作重排到屏障之前的情况。因此，开发者应仔细考虑那些仍未被检查的内存排序要求。

### 超越数据竞争的竞态检测


对于具有复杂并发设计的代码，竞态条件 bug 未必总是表现为数据竞争。当并发执行的操作导致意外的系统行为时，就会发生竞态条件。另一方面，数据竞争是在 C 语言层面定义的。以下宏可用于检查那些 bug 不会表现为数据竞争的并发代码属性。

    :functions: ASSERT_EXCLUSIVE_WRITER ASSERT_EXCLUSIVE_WRITER_SCOPED
                ASSERT_EXCLUSIVE_ACCESS ASSERT_EXCLUSIVE_ACCESS_SCOPED
                ASSERT_EXCLUSIVE_BITS

### 实现细节


KCSAN 依赖于观察到两次访问是并发发生的。关键在于，我们要 (a) 提高观察到竞态的概率（尤其是那些很少出现的竞态），并且 (b) 能够真正观察到它们。我们可以通过注入各种延迟来完成 (a)，并通过使用地址 watchpoint（或断点）来完成 (b)。

如果我们刻意停滞一次内存访问，同时已为其地址设置好 watchpoint，然后观察到该 watchpoint 触发，那么这两次对同一地址的访问刚刚发生了竞态。使用硬件 watchpoint 正是 `DataCollider <http://usenix.org/legacy/events/osdi10/tech/full_papers/Erickson.pdf>`_ 所采取的方法。与 DataCollider 不同，KCSAN 不使用硬件 watchpoint，而是依赖编译器插桩与"软 watchpoint"。

在 KCSAN 中，watchpoint 使用一种高效的编码实现，将访问类型、大小和地址存储在一个 long 中；使用"软 watchpoint"的好处是可移植性与更高的灵活性。KCSAN 随后依赖编译器对普通访问进行插桩。对于每个被插桩的普通访问：

1. 检查是否存在匹配的 watchpoint；若存在，且至少一次访问是写入，那么我们就遇到了一次竞态访问。

2. 周期性地，如果不存在匹配的 watchpoint，则设置一个 watchpoint 并停滞一段较小的随机延迟。

3. 同时在延迟前检查数据值，在延迟后重新检查数据值；如果值不匹配，则推断出一个来源未知的竞态。

为了检测普通访问与带标记访问之间的数据竞争，KCSAN 也会对带标记的访问进行标注，但仅用于检查是否存在 watchpoint；即 KCSAN 从不在带标记的访问上设置 watchpoint。由于从不为带标记的操作设置 watchpoint，如果对某个被并发访问的变量的所有访问都正确做了标记，KCSAN 就永远不会触发 watchpoint，因此也永远不会报告这些访问。

#### 弱内存建模


KCSAN 检测因缺失内存屏障而导致数据竞争的方法，基于对访问重排序的建模（在 `CONFIG_KCSAN_WEAK_MEMORY=y` 时）。每个设置了 watchpoint 的普通内存访问，也会被选中在其函数作用域内模拟重排序（最多 1 个进行中的访问）。

一旦某个访问被选中重排序，它会在函数作用域结束前与每一个其他访问一起被检查。如果遇到适当的内存屏障，该访问将不再被考虑用于模拟重排序。

当内存操作的结果应由屏障来排序时，KCSAN 就能检测那些冲突仅因缺失以下内容而发生的数据竞争

```
    int x, flag;
    void T1(void)
    {
        x = 1;                  // data race!
        WRITE_ONCE(flag, 1);    // correct: smp_store_release(&flag, 1)
    }
    void T2(void)
    {
        while (!READ_ONCE(flag));   // correct: smp_load_acquire(&flag)
        ... = x;                    // data race!
    }
```

当启用弱内存建模时，KCSAN 可将 `T1` 中的 `x` 纳入模拟重排序。在写入 `flag` 之后，`x` 会再次被检查并发访问：由于 `T2` 能在 `flag` 写入后继续推进，因此检测到了数据竞争。若使用了正确的屏障，在正确释放 `flag` 之后，`x` 就不会被考虑重排序，也就不会检测到数据竞争。

刻意在复杂度上做出的权衡以及实际限制，意味着只能检测到因缺失内存屏障而导致的数据竞争的一个子集。在目前的编译器支持下，实现仅限于对"缓冲"（延迟访问）的效果建模，因为运行时无法"预取"访问。还需注意，watchpoint 仅为普通访问设置，且 KCSAN 仅对普通访问类型模拟重排序。这意味着带标记访问的重排序不会被建模。

上述做法的一个结果是：acquire 操作不需要屏障插桩（无预取）。此外，引入地址或控制依赖的带标记访问不需要特殊处理（带标记访问不可重排序，后续依赖访问也无法被预取）。

#### 关键特性


1. **内存开销：** 整体内存开销依据配置仅为几 MiB。当前实现使用一个小型 long 数组来编码 watchpoint 信息，开销可忽略。

2. **性能开销：** KCSAN 的运行时力求最小，使用高效的 watchpoint 编码，在快路径上不需要获取任何共享锁。对于在 8 CPU 系统上的内核启动：

   - 使用默认 KCSAN 配置时减速 5.0 倍；
   - 仅来自运行时快路径开销的减速为 2.8 倍（将 `KCSAN_SKIP_WATCH` 设得非常大并取消 `KCSAN_SKIP_WATCH_RANDOMIZE`）。

3. **注解开销：** KCSAN 运行时之外只需极少的注解。因此，随着内核演进，维护开销也很小。

4. **检测来自设备的竞态写入：** 由于在设置 watchpoint 时检查数据值，来自设备的竞态写入也能被检测到。

5. **内存排序：** KCSAN 仅知晓 LKMM 排序规则的一个子集；这可能导致漏报数据竞争（漏报，false negatives）。

6. **分析准确性：** 对于被观察到的执行，由于采用采样策略，该分析是**不完备的**（可能存在漏报），但力求做到完备（无误报）。

### 考虑过的替代方案


内核的一种替代数据竞争检测方案可见于 `Kernel Thread Sanitizer (KTSAN) <https://github.com/google/kernel-sanitizers/blob/master/KTSAN.md>`_。KTSAN 是一个 happens-before（先于发生）数据竞争检测器，它显式地建立内存操作之间的 happens-before 顺序，进而可用于确定 `Data Races`_（数据竞争）中所定义的数据竞争。

要构建正确的 happens-before 关系，KTSAN 必须知晓 LKMM 的所有排序规则与同步原语。遗憾的是，任何疏漏都会导致大量误报，这对于包含众多自定义同步机制的内核而言尤其有害。为了跟踪 happens-before 关系，KTSAN 的实现需要为每个内存位置（影子内存）维护元数据，每页对应 4 页影子内存，在大型系统上可能转化为数十 GiB 的开销。
