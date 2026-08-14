
## Kernel Address Sanitizer (KASAN，内核地址消毒剂)


### Overview（概述）


Kernel Address Sanitizer (KASAN) 是一个动态内存安全错误检测器，旨在发现越界访问和释放后使用（use-after-free）缺陷。

KASAN 有三种模式：

1. Generic KASAN（通用 KASAN）
2. Software Tag-Based KASAN（基于软件标签的 KASAN）
3. Hardware Tag-Based KASAN（基于硬件标签的 KASAN）

Generic KASAN，通过 CONFIG_KASAN_GENERIC 启用，是面向调试的模式，类似于用户空间 ASan。该模式支持多种 CPU 架构，但具有显著的性能和内存开销。

Software Tag-Based KASAN（或称 SW_TAGS KASAN），通过 CONFIG_KASAN_SW_TAGS 启用，可用于调试和 dogfood 测试，类似于用户空间 HWASan。该模式仅支持 arm64，但其适度的内存开销允许在受内存限制的设备上以真实工作负载进行测试。

Hardware Tag-Based KASAN（或称 HW_TAGS KASAN），通过 CONFIG_KASAN_HW_TAGS 启用，是旨在用作现场内存缺陷检测器或安全缓解措施的的模式。该模式仅适用于支持 MTE（Memory Tagging Extension，内存标记扩展）的 arm64 CPU，但其内存和性能开销很低，因此可用于生产环境。

关于每种 KASAN 模式的内存与性能影响，详见相应 Kconfig 选项的描述。

Generic 和 Software Tag-Based 模式通常被称为软件模式。Software Tag-Based 和 Hardware Tag-Based 模式被称为基于标签的模式。

### Support（支持）


#### Architectures（架构）


Generic KASAN 支持 x86_64、arm、arm64、powerpc、riscv、s390、xtensa 和 loongarch，而基于标签的 KASAN 模式仅支持 arm64。

#### Compilers（编译器）


软件 KASAN 模式使用编译期插桩，在每次内存访问前插入有效性检查，因此需要提供支持该特性的编译器版本。基于硬件标签的模式依赖硬件执行这些检查，但仍需要支持内存标记指令的编译器版本。

Generic KASAN 需要 GCC 8.3.0 或更高版本，或内核支持的任何 Clang 版本。

Software Tag-Based KASAN 需要 GCC 11+ 或内核支持的任何 Clang 版本。

Hardware Tag-Based KASAN 需要 GCC 10+ 或 Clang 12+。

#### Memory types（内存类型）


Generic KASAN 支持在 slab、page_alloc、vmap、vmalloc、stack 和 global 内存中发现缺陷。

Software Tag-Based KASAN 支持 slab、page_alloc、vmalloc 和 stack 内存。

Hardware Tag-Based KASAN 支持 slab、page_alloc 和非可执行 vmalloc 内存。

### Usage（用法）


```
	  CONFIG_KASAN=y
```
并从 `CONFIG_KASAN_GENERIC`（启用 Generic KASAN）、`CONFIG_KASAN_SW_TAGS`（启用 Software Tag-Based KASAN）和 `CONFIG_KASAN_HW_TAGS`（启用 Hardware Tag-Based KASAN）中选择。

对于软件模式，还要在 `CONFIG_KASAN_OUTLINE` 和 `CONFIG_KASAN_INLINE` 之间选择。outline 和 inline 是编译器插桩类型。前者生成较小的二进制文件，而后者速度快至 2 倍。

要将受影响 slab 对象的分配与释放栈回溯纳入报告，启用 `CONFIG_STACKTRACE`。要包含受影响物理页的分配与释放栈回溯，启用 `CONFIG_PAGE_OWNER` 并以 `page_owner=on` 启动。

#### Boot parameters（启动参数）


KASAN 受通用的 `panic_on_warn` 命令行参数影响。当它启用时，KASAN 会在打印缺陷报告后使内核 panic。

默认情况下，KASAN 仅针对第一次无效内存访问打印缺陷报告。使用 `kasan_multi_shot` 时，KASAN 会在每次无效访问时打印报告。这实际上为 KASAN 报告禁用了 `panic_on_warn`。

或者，独立于 `panic_on_warn`，`kasan.fault=` 启动参数可用于控制 panic 和报告行为：

- `kasan.fault=report`、`=panic` 或 `=panic_on_write` 控制是仅打印 KASAN 报告、使内核 panic，还是仅在无效写访问时使内核 panic（默认：`report`）。即使启用了 `kasan_multi_shot`，也会发生 panic。注意，当使用 Hardware Tag-Based KASAN 的异步模式时，`kasan.fault=panic_on_write` 总是对异步检查的访问（包括读）触发 panic。

Software 和 Hardware Tag-Based KASAN 模式（见下文关于各种模式的章节）支持改变栈回溯收集行为：

- `kasan.stacktrace=off` 或 `=on` 禁用或启用分配与释放栈回溯的收集（默认：`on`）。
- `kasan.stack_ring_size=<number of entries>` 指定栈环（stack ring）中的条目数（默认：`32768`）。

Hardware Tag-Based KASAN 模式旨在用作生产环境中的安全缓解措施。因此，它支持额外的启动参数，允许完全禁用 KASAN 或控制其特性：

- `kasan=off` 或 `=on` 控制是否启用 KASAN（默认：`on`）。

- `kasan.mode=sync`、`=async` 或 `=asymm` 控制 KASAN 配置为同步、异步或非对称执行模式（默认：`sync`）。
  同步模式：当发生标签检查故障时，立即检测到错误访问。
  异步模式：错误访问的检测被延迟。当发生标签检查故障时，信息存储在硬件中（对于 arm64，存储在 TFSR_EL1 寄存器中）。内核定期检查硬件，仅在这些检查期间报告标签故障。
  非对称模式：错误访问在读时同步检测，在写时异步检测。

- `kasan.write_only=off` 或 `kasan.write_only=on` 控制 KASAN 是仅检查写（store）访问还是检查所有访问（默认：`off`）。

- `kasan.vmalloc=off` 或 `=on` 禁用或启用 vmalloc 分配的标记（默认：`on`）。

- `kasan.page_alloc.sample=<采样间隔>` 使 KASAN 仅对每第 N 个 order 等于或大于 `kasan.page_alloc.sample.order` 的 page_alloc 分配进行标记，其中 N 为 `sample` 参数的值（默认：`1`，即对每个此类分配都标记）。
  该参数旨在缓解 KASAN 引入的性能开销。
  注意，启用此参数会使 Hardware Tag-Based KASAN 跳过对采样所选分配的检核，从而漏掉对这些分配的坏访问。为准确检测缺陷，请使用默认值。

- `kasan.page_alloc.sample.order=<最小页 order>` 指定受采样影响的分配的最小 order（默认：`3`）。
  仅当 `kasan.page_alloc.sample` 设置为大于 `1` 的值时适用。
  该参数旨在仅允许对大型 page_alloc 分配进行采样，这类分配是性能开销的最大来源。

#### Error reports（错误报告）


```
    ==================================================================
    BUG: KASAN: slab-out-of-bounds in kmalloc_oob_right+0xa8/0xbc [kasan_test]
    Write of size 1 at addr ffff8801f44ec37b by task insmod/2760

    CPU: 1 PID: 2760 Comm: insmod Not tainted 4.19.0-rc3+ #698
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.10.2-1 04/01/2014
    Call Trace:
     dump_stack+0x94/0xd8
     print_address_description+0x73/0x280
     kasan_report+0x144/0x187
     __asan_report_store1_noabort+0x17/0x20
     kmalloc_oob_right+0xa8/0xbc [kasan_test]
     kmalloc_tests_init+0x16/0x700 [kasan_test]
     do_one_initcall+0xa5/0x3ae
     do_init_module+0x1b6/0x547
     load_module+0x75df/0x8070
     __do_sys_init_module+0x1c6/0x200
     __x64_sys_init_module+0x6e/0xb0
     do_syscall_64+0x9f/0x2c0
     entry_SYSCALL_64_after_hwframe+0x44/0xa9
    RIP: 0033:0x7f96443109da
    RSP: 002b:00007ffcf0b51b08 EFLAGS: 00000202 ORIG_RAX: 00000000000000af
    RAX: ffffffffffffffda RBX: 000055dc3ee521a0 RCX: 00007f96443109da
    RDX: 00007f96445cff88 RSI: 0000000000057a50 RDI: 00007f9644992000
    RBP: 000055dc3ee510b0 R08: 0000000000000003 R09: 0000000000000000
    R10: 00007f964430cd0a R11: 0000000000000202 R12: 00007f96445cff88
    R13: 000055dc3ee51090 R14: 0000000000000000 R15: 0000000000000000

    Allocated by task 2760:
     save_stack+0x43/0xd0
     kasan_kmalloc+0xa7/0xd0
     kmem_cache_alloc_trace+0xe1/0x1b0
     kmalloc_oob_right+0x56/0xbc [kasan_test]
     kmalloc_tests_init+0x16/0x700 [kasan_test]
     do_one_initcall+0xa5/0x3ae
     do_init_module+0x1b6/0x547
     load_module+0x75df/0x8070
     __do_sys_init_module+0x1c6/0x200
     __x64_sys_init_module+0x6e/0xb0
     do_syscall_64+0x9f/0x2c0
     entry_SYSCALL_64_after_hwframe+0x44/0xa9

    Freed by task 815:
     save_stack+0x43/0xd0
     __kasan_slab_free+0x135/0x190
     kasan_slab_free+0xe/0x10
     kfree+0x93/0x1a0
     umh_complete+0x6a/0xa0
     call_usermodehelper_exec_async+0x4c3/0x640
     ret_from_fork+0x35/0x40

    The buggy address belongs to the object at ffff8801f44ec300
     which belongs to the cache kmalloc-128 of size 128
    The buggy address is located 123 bytes inside of
     128-byte region [ffff8801f44ec300, ffff8801f44ec380)
    The buggy address belongs to the page:
    page:ffffea0007d13b00 count:1 mapcount:0 mapping:ffff8801f7001640 index:0x0
    flags: 0x200000000000100(slab)
    raw: 0200000000000100 ffffea0007d11dc0 0000001a0000001a ffff8801f7001640
    raw: 0000000000000000 0000000080150015 00000001ffffffff 0000000000000000
    page dumped because: kasan: bad access detected

    Memory state around the buggy address:
     ffff8801f44ec200: fc fc fc fc fc fc fc fc fb fb fb fb fb fb fb fb
     ffff8801f44ec280: fb fb fb fb fb fb fb fb fc fc fc fc fc fc fc fc
    >ffff8801f44ec300: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 03
                                                                    ^
     ffff8801f44ec380: fc fc fc fc fc fc fc fc fb fb fb fb fb fb fb fb
     ffff8801f44ec400: fb fb fb fb fb fb fb fb fc fc fc fc fc fc fc fc
    ==================================================================
```

报告头部概括了发生了何种缺陷以及由何种访问引起。其后跟随错误访问的栈回溯、被访问内存被分配位置的栈回溯（若访问的是 slab 对象），以及对象被释放位置的栈回溯（若是 use-after-free 缺陷报告）。接下来是所访问 slab 对象的描述以及被访问内存页的信息。

最后，报告展示被访问地址周围的内存状态。在内部，KASAN 对每个内存颗粒（memory granule）单独跟踪内存状态，该颗粒根据 KASAN 模式为 8 或 16 字节对齐。报告内存状态部分中的每个数字显示围绕被访问地址的某个内存颗粒的状态。

对于 Generic KASAN，每个内存颗粒的大小为 8。每个颗粒的状态编码在一个影子字节（shadow byte）中。这 8 个字节可以是可访问的、部分可访问的、已释放的，或是 redzone 的一部分。KASAN 对每个影子字节使用如下编码：00 表示对应内存区域的全部 8 个字节均可访问；数字 N（1 <= N <= 7）表示前 N 个字节可访问，其余（8 - N）个字节不可访问；任何负值表示整个 8 字节字不可访问。KASAN 使用不同的负值来区分不同类型不可访问内存，如 redzone 或已释放内存（见 mm/kasan/kasan.h）。

在上面的报告中，箭头指向影子字节 `03`，这意味着被访问地址是部分可访问的。

对于基于标签的 KASAN 模式，这最后一部分报告显示被访问地址周围的内存标签（见 `Implementation details`_ 章节）。

注意，KASAN 缺陷标题（如 `slab-out-of-bounds` 或 `use-after-free`）是尽力而为的：KASAN 根据其所拥有的有限信息打印最可能的缺陷类型。实际缺陷类型可能不同。

Generic KASAN 还会报告最多两条辅助调用栈回溯。这些栈回溯指向与对象交互但未直接出现在错误访问栈回溯中的代码位置。目前，这包括 call_rcu() 和 workqueue 排队。

#### CONFIG_KASAN_EXTRA_INFO


启用 CONFIG_KASAN_EXTRA_INFO 允许 KASAN 记录并报告更多信息。当前支持的额外信息是分配与释放时的 CPU 编号和时间戳。更多信息有助于找到缺陷原因并将错误与其他系统事件关联，代价是使用额外内存来记录更多信息（更多代价细节见 CONFIG_KASAN_EXTRA_INFO 的帮助文本）。

以下是启用 CONFIG_KASAN_EXTRA_INFO 后的报告（仅
```
    ==================================================================
    ...
    Allocated by task 134 on cpu 5 at 229.133855s:
    ...
    Freed by task 136 on cpu 3 at 230.199335s:
    ...
    ==================================================================
```

### Implementation details（实现细节）


#### Generic KASAN


软件 KASAN 模式使用影子内存来记录每个内存字节是否可安全访问，并使用编译期插桩在每次内存访问前插入影子内存检查。

Generic KASAN 将其影子内存占为内核内存的 1/8（在 x86_64 上为 16TB 以覆盖 128TB），并使用带比例和偏移的直映射将内存地址转换为其对应的影子地址。

以下是用于将地址转换为其对应影子地址的函数
```
    static inline void *kasan_mem_to_shadow(const void *addr)
    {
	return (void *)((unsigned long)addr >> KASAN_SHADOW_SCALE_SHIFT)
		+ KASAN_SHADOW_OFFSET;
    }
```
其中 `KASAN_SHADOW_SCALE_SHIFT = 3`。

编译期插桩用于插入内存访问检查。编译器在每次大小为 1、2、4、8 或 16 的内存访问前插入函数调用（`__asan_load**(addr)`、`__asan_store**(addr)`）。这些函数通过检查对应的影子内存来判断内存访问是否有效。

使用 inline 插桩时，编译器不直接进行函数调用，而是直接插入检查影子内存的代码。此选项显著增大内核体积，但相比 outline 插桩的内核带来 x1.1-x2 的性能提升。

Generic KASAN 是唯一通过隔离区（quarantine）延迟释放对象重用的模式（实现见 mm/kasan/quarantine.c）。

#### Software Tag-Based KASAN


Software Tag-Based KASAN 使用软件内存标记方法来检查访问有效性。目前仅针对 arm64 架构实现。

Software Tag-Based KASAN 使用 arm64 CPU 的 Top Byte Ignore (TBI) 特性，在内核指针的最高字节中存储指针标签。它使用影子内存存储与每个 16 字节内存单元关联的内存标签（因此，它占内核内存的 1/16 用于影子内存）。

在每次内存分配时，Software Tag-Based KASAN 生成一个随机标签，用此标签标记已分配内存，并将同一标签嵌入返回的指针中。

Software Tag-Based KASAN 使用编译期插桩在每次内存访问前插入检查。这些检查确保被访问内存的标签等于用于访问该内存的指针的标签。若发生标签不匹配，Software Tag-Based KASAN 打印缺陷报告。

Software Tag-Based KASAN 也有两种插桩模式（outline，发出回调以检查内存访问；以及 inline，内联执行影子内存检查）。在 outline 插桩模式下，缺陷报告由执行访问检查的函数打印。在 inline 插桩模式下，编译器发出 `brk` 指令，并使用专用的 `brk` 处理程序来打印缺陷报告。

Software Tag-Based KASAN 使用 0xFF 作为 match-all 指针标签（通过带有 0xFF 指针标签的指针进行的访问不被检查）。值 0xFE 当前保留用于标记已释放的内存区域。

#### Hardware Tag-Based KASAN


Hardware Tag-Based KASAN 在概念上类似于软件模式，但使用硬件内存标记支持，而非编译器插桩和影子内存。

Hardware Tag-Based KASAN 目前仅针对 arm64 架构实现，并基于 ARMv8.5 指令集架构引入的 arm64 Memory Tagging Extension (MTE) 以及 Top Byte Ignore (TBI)。

专用的 arm64 指令用于为每个分配分配内存标签。相同的标签被分配给指向这些分配的指针。在每次内存访问时，硬件确保被访问内存的标签等于用于访问该内存的指针的标签。若发生标签不匹配，则生成故障并打印报告。

Hardware Tag-Based KASAN 使用 0xFF 作为 match-all 指针标签（通过带有 0xFF 指针标签的指针进行的访问不被检查）。值 0xFE 当前保留用于标记已释放的内存区域。

若硬件不支持 MTE（ARMv8.5 之前），Hardware Tag-Based KASAN 将不会被启用。在这种情况下，所有 KASAN 启动参数均被忽略。

注意，启用 CONFIG_KASAN_HW_TAGS 总是会导致内核内 TBI 被启用。即使提供了 `kasan.mode=off`，或硬件不支持 MTE（但支持 TBI）。

Hardware Tag-Based KASAN 仅报告发现的第一个缺陷。此后，MTE 标签检查被禁用。

### Shadow memory（影子内存）


本节内容仅适用于软件 KASAN 模式。

内核在地址空间的多个不同部分映射内存。内核虚拟地址的范围很大：没有足够的物理内存来为内核可能访问的每个地址支持真实的影子区域。因此，KASAN 仅为地址空间的某些部分映射真实的影子。

#### Default behaviour（默认行为）


默认情况下，架构仅为线性映射（以及潜在的其他小部分区域）之上的影子区域映射真实内存。对于所有其他区域——如 vmalloc 和 vmemmap 空间——在影子区域之上映射单个只读页。这个只读影子页将所有内存访问声明为允许。

这给模块带来了问题：它们不位于线性映射中，而是位于专用的模块空间。通过挂接（hook）模块分配器，KASAN 临时映射真实影子内存来覆盖它们。例如，这允许检测对模块全局变量的无效访问。

这也造成了与 `VMAP_STACK` 的不兼容：若栈位于 vmalloc 空间中，它将被该只读页遮蔽，内核在尝试为栈变量建立影子数据时将出错。

#### CONFIG_KASAN_VMALLOC


通过 `CONFIG_KASAN_VMALLOC`，KASAN 可以以更大的内存使用为代价覆盖 vmalloc 空间。目前，这在 x86、arm64、riscv、s390 和 powerpc 上受支持。

其工作方式是通过挂接 vmalloc 和 vmap，并动态分配真实影子内存来支撑映射。

vmalloc 空间中的大多数映射都很小，需要的影子空间不足一整页。因此，为每个映射分配一整页影子页将是浪费的。此外，为确保不同的映射使用不同的影子页，映射必须与 `KASAN_GRANULE_SIZE * PAGE_SIZE` 对齐。

相反，KASAN 在多个映射之间共享支撑空间。当 vmalloc 空间中的映射使用影子区域的某个特定页时，它分配一个支撑页。该页之后可被其他 vmalloc 映射共享。

KASAN 挂接 vmap 基础设施，以惰性清理未使用的影子内存。

为避免围绕映射交换的困难，KASAN 期望覆盖 vmalloc 空间的影子区域部分不被早期影子页覆盖，而是保持未映射。这将需要架构相关代码的改动。

这允许在 x86 上支持 `VMAP_STACK`，并可简化对没有固定模块区域的架构的支持。

### For developers（面向开发者）


#### Ignoring accesses（忽略访问）


软件 KASAN 模式使用编译器插桩来插入有效性检查。此类插桩可能与内核的某些部分不兼容，因此需要被禁用。

内核的其他部分可能访问已分配对象的元数据。通常，KASAN 会检测并报告此类访问，但在某些情况下（例如，在内存分配器中），这些访问是有效的。

对于软件 KASAN 模式，要为特定文件或目录禁用插桩，请向相应的内核 Makefile 添加 `KASAN_SANITIZE` 注解：

```
    KASAN_SANITIZE_main.o := n
```
```
    KASAN_SANITIZE := n
```
对于软件 KASAN 模式，要以逐函数方式禁用插桩，使用 KASAN 特定的 `__no_sanitize_address` 函数属性或通用的 `noinstr` 属性。

注意，禁用编译器插桩（无论是按文件还是按函数）会使 KASAN 忽略该代码中直接发生的访问（针对软件 KASAN 模式）。当访问间接发生（通过对插桩函数的调用）或使用不使用编译器插桩的 Hardware Tag-Based KASAN 时，它无济于事。

对于软件 KASAN 模式，要针对当前任务在内核代码的一部分中禁用 KASAN 报告，请用 `kasan_disable_current()`/`kasan_enable_current()` 区段注解该部分代码。这也会禁用通过函数调用发生的间接访问的报告。

对于基于标签的 KASAN 模式，要禁用访问检查，使用 `kasan_reset_tag()` 或 `page_kasan_tag_reset()`。注意，通过 `page_kasan_tag_reset()` 临时禁用访问检查需要借助 `page_kasan_tag`/`page_kasan_tag_set` 保存并恢复每页的 KASAN 标签。

#### Tests（测试）


有一些 KASAN 测试可用于验证 KASAN 是否工作以及能否检测某些类型的内存损坏。

所有 KASAN 测试都与 KUnit Test Framework 集成，并可通过 `CONFIG_KASAN_KUNIT_TEST` 启用。测试可以以几种不同的方式自动运行和部分验证；见以下说明。

每个 KASAN 测试在检测到错误时打印多个 KASAN 报告之一。然后该测试打印其编号和状态。

```
        ok 28 - kmalloc_double_kzfree
```

```
        # kmalloc_large_oob_right: ASSERTION FAILED at mm/kasan/kasan_test.c:245
        Expected ptr is not null, but is
        not ok 5 - kmalloc_large_oob_right
```
```
        # kmalloc_double_kzfree: EXPECTATION FAILED at mm/kasan/kasan_test.c:709
        KASAN failure expected in "kfree_sensitive(ptr)", but none occurred
        not ok 28 - kmalloc_double_kzfree
```
```
        ok 1 - kasan
```
```
        not ok 1 - kasan
```

有几种运行 KASAN 测试的方式。

1. 可加载模块（Loadable module）

   启用 `CONFIG_KUNIT` 后，测试可构建为可加载模块，并通过用 `insmod` 或 `modprobe` 加载 `kasan_test.ko` 来运行。

2. 内建（Built-In）

   启用内建的 `CONFIG_KUNIT` 后，测试也可内建。

   在这种情况下，测试将在启动时作为 late-init 调用运行。

3. 使用 kunit_tool

   启用内建的 `CONFIG_KUNIT` 和 `CONFIG_KASAN_KUNIT_TEST` 时，也可以使用 `kunit_tool` 以更易读的方式查看 KUnit 测试的结果。这不会打印已通过测试的 KASAN 报告。有关 `kunit_tool` 的最新信息，参见 `KUnit 文档 <https://www.kernel.org/doc/html/latest/dev-tools/kunit/index.html>`_。

