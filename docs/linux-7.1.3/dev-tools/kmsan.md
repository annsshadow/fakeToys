
## Kernel Memory Sanitizer (KMSAN)


KMSAN 是一个动态错误检测器，旨在发现对未初始化值的使用。它基于编译器插桩实现，与用户空间的 `MemorySanitizer tool`_ 十分相似。

需要特别注意的是，KMSAN 并非为生产环境使用而设计，因为它会显著增加内核内存占用并拖慢整个系统。

## 用法


### 构建内核


为了构建带有 KMSAN 的内核，你需要一个较新的 Clang（14.0.6+）。有关如何构建 Clang 的说明，请参考 `LLVM documentation`_。

现在，在启用 `CONFIG_KMSAN` 的情况下配置并构建内核。

### 示例报告


```

  =====================================================
  BUG: KMSAN: uninit-value in test_uninit_kmsan_check_memory+0x1be/0x380 [kmsan_test]
   test_uninit_kmsan_check_memory+0x1be/0x380 mm/kmsan/kmsan_test.c:273
   kunit_run_case_internal lib/kunit/test.c:333
   kunit_try_run_case+0x206/0x420 lib/kunit/test.c:374
   kunit_generic_run_threadfn_adapter+0x6d/0xc0 lib/kunit/try-catch.c:28
   kthread+0x721/0x850 kernel/kthread.c:327
   ret_from_fork+0x1f/0x30 ??:?

  Uninit was stored to memory at:
   do_uninit_local_array+0xfa/0x110 mm/kmsan/kmsan_test.c:260
   test_uninit_kmsan_check_memory+0x1a2/0x380 mm/kmsan/kmsan_test.c:271
   kunit_run_case_internal lib/kunit/test.c:333
   kunit_try_run_case+0x206/0x420 lib/kunit/test.c:374
   kunit_generic_run_threadfn_adapter+0x6d/0xc0 lib/kunit/try-catch.c:28
   kthread+0x721/0x850 kernel/kthread.c:327
   ret_from_fork+0x1f/0x30 ??:?

  Local variable uninit created at:
   do_uninit_local_array+0x4a/0x110 mm/kmsan/kmsan_test.c:256
   test_uninit_kmsan_check_memory+0x1a2/0x380 mm/kmsan/kmsan_test.c:271

  Bytes 4-7 of 8 are uninitialized
  Memory access of size 8 starts at ffff888083fe3da0

  CPU: 0 PID: 6731 Comm: kunit_try_catch Tainted: G    B       E     5.16.0-rc3+ #104
  Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
  =====================================================

```
该报告表明，局部变量 `uninit` 在 `do_uninit_local_array()` 中以未初始化状态创建。第三条栈回溯对应该变量被创建的位置。

第一条栈回溯展示了未初始化值在何处被使用（在 `test_uninit_kmsan_check_memory()` 中）。该工具还显示了局部变量中哪些字节未被初始化，以及在使用前该值被复制到另一处内存位置的栈信息。

KMSAN 在以下情况下会报告对未初始化值 `v` 的使用：

 - 在条件判断中，例如 `if (v) { ... }`；
 - 在索引或指针解引用中，例如 `array[v]` 或 `*v`；
 - 当它被复制到用户空间或硬件时，例如 `copy_to_user(..., &v, ...)`；
 - 当它作为参数传递给函数，且
   `CONFIG_KMSAN_CHECK_PARAM_RETVAL` 已启用（见下文）。

所提及的情况（除了向用户空间或硬件复制数据这种情况，它属于安全问题）从 C11 标准的角度来看都被视为未定义行为。

### 禁用插桩


可以使用函数属性 `__no_kmsan_checks` 标记某个函数。这样做会使 KMSAN 忽略该函数中的未初始化值，并将其输出标记为已初始化。结果是，用户将不会再收到与该函相关的 KMSAN 报告。

KMSAN 支持的另一个函数属性是 `__no_sanitize_memory`。将该属性应用于某个函数会使 KMSAN 不对其插桩，如果我们不希望编译器干扰某些底层代码（例如被标记为 `noinstr` 的代码，它会隐式添加 `__no_sanitize_memory`），这会很有帮助。

然而这会带来代价：此类函数中的栈分配将具有不正确的 shadow/origin 值，很可能导致误报。从非插桩代码调用的函数也可能收到不正确的参数元数据。

作为经验法则，应避免显式使用 `__no_sanitize_memory`。

```

  KMSAN_SANITIZE_main.o := n

```
```
  KMSAN_SANITIZE := n

```
在 Makefile 中。可以将其视为对该文件或目录中的每个函数应用 `__no_sanitize_memory`。大多数用户不需要 `KMSAN_SANITIZE`，除非他们的代码被 KMSAN 破坏（例如在早期启动阶段运行）。

KMSAN 检查也可以使用 `kmsan_disable_current()` 和 `kmsan_enable_current()` 调用为当前任务临时禁用。每个 `kmsan_enable_current()` 调用之前必须有一个 `kmsan_disable_current()` 调用；这些调用对可以嵌套。使用这些调用时需要小心，保持禁用区域简短，并尽可能优先使用其他禁用插桩的方式。

## Support


为了让 KMSAN 正常工作，内核必须使用 Clang 构建，迄今为止 Clang 是唯一支持 KMSAN 的编译器。内核插桩过程基于用户空间的 `MemorySanitizer tool`_。

目前运行时库仅支持 x86_64。

## KMSAN 工作原理


### KMSAN 影子内存


KMSAN 为内核内存的每一个字节关联一个元数据字节（也称为 shadow 字节）。如果内核内存字节中的对应位未初始化，则 shadow 字节中的相应位被置位。将内存标记为未初始化（即把其 shadow 字节设为 `0xff`）称为 poisoning（污染），将其标记为已初始化（把 shadow 字节设为 `0x00`）称为 unpoisoning（去污染）。

当一个新的变量在栈上分配时，默认情况下会被编译器插入的插桩代码污染（除非它是一个立即被初始化的栈变量）。任何没有使用 `__GFP_ZERO` 的新堆分配也会被污染。

编译器插桩还会跟踪 shadow 值随代码使用的传播过程。在需要时，插桩代码会调用 `mm/kmsan/` 中的运行时库来持久化 shadow 值。

基本类型或复合类型的 shadow 值是一个与其等长的字节数组。当向内存写入常量值时，该内存被去污染。当从内存读取值时，也会获取其 shadow 内存，并将其传播到所有使用该值的操作中。对于每一个取一个或多个值的指令，编译器会生成代码，根据这些值及其 shadow 计算结果的 shadow。

```

  int a = 0xff;  // i.e. 0x000000ff
  int b;
  int c = a | b;

```
在这种情况下，`a` 的 shadow 是 `0`，`b` 的 shadow 是 `0xffffffff`，`c` 的 shadow 是 `0xffffff00`。这意味着 `c` 的高三个字节未初始化，而低字节已初始化。

### 来源跟踪


内核内存的每四个字节也映射了一个所谓的 origin（来源）。该 origin 描述了在程序执行过程中创建该未初始化值的位置。每个 origin 要么关联到完整的分配栈（对于堆分配的内存），要么关联到包含该未初始化变量的函数（对于局部变量）。

当未初始化变量在栈或堆上分配时，会创建一个新的 origin 值，并用该值填充该变量的 origin。当从内存读取值时，也会读取其 origin，并与 shadow 一起保存。对于每一个取一个或多个值的指令，结果的 origin 是对应于任一未初始化输入的 origin 之一。如果将污染值写入内存，其 origin 也会被写入对应的存储位置。

```

  int a = 42;
  int b;
  int c = a + b;

```
在这种情况下，`b` 的 origin 在函数入口时生成，并在加法结果写入内存之前存储到 `c` 的 origin 中。

如果多个变量存储在同一个四字节块中，它们可能会共享相同的 origin 地址。在这种情况下，对任意一个变量的每次写入都会更新所有这些变量的 origin。在这种情况下我们不得不牺牲精度，因为为单个位（甚至字节）存储 origin 代价过高。

```

  int combine(short a, short b) {
    union ret_t {
      int i;
      short s[2];
    } ret;
    ret.s[0] = a;
    ret.s[1] = b;
    return ret.i;
  }

```
如果 `a` 已初始化而 `b` 未初始化，则结果的 shadow 将是 `0xffff0000`，结果的 origin 将是 `b` 的 origin。`ret.s[^0^]` 将具有相同的 origin，但它永远不会被使用，因为该变量已初始化。

如果两个函数参数都未初始化，则只保留第二个参数的 origin。

#### 来源链


为了简化调试，KMSAN 会为每次将未初始化值存储到内存创建新的 origin。新的 origin 同时引用其创建栈以及该值先前拥有的 origin。这可能导致内存消耗增加，因此我们在运行时中限制了 origin 链的长度。

### Clang 插桩 API


Clang 插桩过程会向内核代码中插入对 `mm/kmsan/nstrumentation.c` 中定义的函数的调用。

#### 影子操作


对于每次内存访问，编译器都会发出一个调用，调用一个返回以下内容的函数：

```

  typedef struct {
    void *shadow, *origin;
  } shadow_origin_ptr_t

  shadow_origin_ptr_t __msan_metadata_ptr_for_load_{1,2,4,8}(void *addr)
  shadow_origin_ptr_t __msan_metadata_ptr_for_store_{1,2,4,8}(void *addr)
  shadow_origin_ptr_t __msan_metadata_ptr_for_load_n(void *addr, uintptr_t size)
  shadow_origin_ptr_t __msan_metadata_ptr_for_store_n(void *addr, uintptr_t size)

```
函数名取决于内存访问的大小。

编译器确保对于每一个被加载的值，其 shadow 和 origin 值都从内存中读取。当值被存储到内存时，其 shadow 和 origin 也会使用元数据指针一并存储。

#### 处理局部变量


使用特殊函数为局部变量创建新的 origin 值：

```
  void __msan_poison_alloca(void *addr, uintptr_t size, char *descr)

```
#### 对每个任务数据的访问


在每个被插桩函数的开头，KMSAN 会插入对以下函数的调用：

```
  kmsan_context_state *__msan_get_context_state(void)

```
```
  struct kmsan_context_state {
    char param_tls[KMSAN_PARAM_SIZE];
    char retval_tls[KMSAN_RETVAL_SIZE];
    char va_arg_tls[KMSAN_PARAM_SIZE];
    char va_arg_origin_tls[KMSAN_PARAM_SIZE];
    u64 va_arg_overflow_size_tls;
    char param_origin_tls[KMSAN_PARAM_SIZE];
    depot_stack_handle_t retval_origin_tls;
  };

```
该结构被 KMSAN 用来在被插桩的函数之间传递参数的 shadow 和 origin（除非参数被 `CONFIG_KMSAN_CHECK_PARAM_RETVAL` 立即检查）。

#### 将未初始化值传递给函数


Clang 的 MemorySanitizer 插桩有一个选项 `-fsanitize-memory-param-retval`，它使编译器检查按值传递的函数参数以及函数返回值。

该选项由 `CONFIG_KMSAN_CHECK_PARAM_RETVAL` 控制，默认启用，以使 KMSAN 能够更早地报告未初始化值。更多细节请参考 `LKML discussion`_。

由于这些检查在 LLVM 中的实现方式（它们仅应用于标记为 `noundef` 的参数），并非所有参数都能保证被检查，因此我们不能放弃 `kmsan_context_state` 中的元数据存储。

#### 字符串函数


编译器会将对 `memcpy()`/`memmove()`/`memset()` 的调用替换为以下函数。当数据结构被初始化或复制时也会调用这些函数，确保 shadow 和 origin 值随之一并被复制：

```
  void *__msan_memcpy(void *dst, void *src, uintptr_t n)
  void *__msan_memmove(void *dst, void *src, uintptr_t n)
  void *__msan_memset(void *dst, int c, uintptr_t n)

```
#### 错误报告


对于每次值的使用，编译器都会发出一个 shadow 检查，调用：

```
  void __msan_warning(u32 origin)

```
`__msan_warning()` 会使 KMSAN 运行时打印错误报告。

#### 内联汇编插桩


```
  void __msan_instrument_asm_store(void *addr, uintptr_t size)

```
，它会将该内存区域去污染。

这种方法可能会掩盖某些错误，但也有助于避免位运算、原子操作等场景中大量的误报。

有时传入内联汇编的指针并不指向有效内存。在这种情况下，它们会在运行时被忽略。


### 运行时库


代码位于 `mm/kmsan/`。

#### 每个任务的 KMSAN 状态


每个 task_struct 都关联一个 KMSAN 任务状态，用于保存 KMSAN：

```
  struct kmsan_context {
    ...
    unsigned int depth;
    struct kmsan_context_state cstate;
    ...
  }

  struct task_struct {
    ...
    struct kmsan_context kmsan;
    ...
  }

```
#### KMSAN 上下文


在运行于内核任务上下文时，KMSAN 使用 `current->kmsan.cstate` 来保存函数参数和返回值的元数据。

但在内核运行于中断、softirq 或 NMI 上下文的情况下：

```
  DEFINE_PER_CPU(struct kmsan_ctx, kmsan_percpu_ctx);

```
#### 元数据分配


内核中有几个用于存放元数据的地方。

1. 每个 `struct page` 实例都包含两个指向其 shadow 和 origin 的指针：

```
  struct page {
    ...
    struct page *shadow, *origin;
    ...
  };

```
在启动阶段，内核为每一个可用的内核页分配 shadow 和 origin 页。这一过程发生得相当晚，此时内核地址空间已经碎片化，因此普通数据页可能会与元数据页任意交错。

这意味着，通常对于两个连续的内存页，它们的 shadow/origin 页可能并不连续。因此，如果一次内存访问跨越了某个内存块的边界，对 shadow/origin 内存的访问可能会破坏其他页，或从这些页中读取不正确的值。

实际上，由同一次 `alloc_pages()` 调用返回的连续内存页将具有连续的元数据，而如果这些页属于两次不同的分配，它们的元数据页则可能碎片化。

对于内核数据（`.data`、`.bss` 等）和 percpu 内存区域，同样不保证元数据的连续性。

当 `__msan_metadata_ptr_for_XXX_YYY()` 命中两个分配之间的边界时：

```
  char dummy_load_page[PAGE_SIZE] __attribute__((aligned(PAGE_SIZE)));
  char dummy_store_page[PAGE_SIZE] __attribute__((aligned(PAGE_SIZE)));

```
`dummy_load_page` 被零初始化，因此从中读取总是得到零。`dummy_store_page` 的所有写入都被忽略。

2. 对于 vmalloc 内存和模块，内存区间、其 shadow 和 origin 之间存在直接映射。KMSAN 将 vmalloc 区域缩减 3/4，使得只有第一个四分之一可用于 `vmalloc()`。vmalloc 区域的第二个四分之一包含第一个四分之一的 shadow 内存，第三个四分之一保存 origin。第四个四分之一的一小部分包含内核模块的 shadow 和 origin。更多细节请参考 `arch/x86/include/asm/pgtable_64_types.h`。

当一组页被映射到连续的虚拟内存空间时，它们的 shadow 和 origin 页也会被类似地映射到连续区域。

## 参考资料


E. Stepanov, K. Serebryany. `MemorySanitizer：C++ 中未初始化内存使用的快速检测器
<https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/43308.pdf>`_.
In Proceedings of CGO 2015.
