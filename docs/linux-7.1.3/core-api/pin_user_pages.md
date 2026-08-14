
## pin_user_pages() 及相关调用

## 概述

```

 pin_user_pages()
 pin_user_pages_fast()
 pin_user_pages_remote()

```
## FOLL_PIN 基本描述

FOLL_PIN 和 FOLL_LONGTERM 是可以传递给 get_user_pages*()（“gup”）函数族的标志。FOLL_PIN 与 FOLL_LONGTERM 有着显著的交互和相互依赖关系，因此这里一并介绍。

FOLL_PIN 是 gup 内部的，意味着它不应出现在 gup 调用点。这使得相关的包装函数（pin_user_pages*() 及其他）能够设置这些标志的正确组合，并检查问题。

另一方面，FOLL_LONGTERM **可以**在 gup 调用点设置。这样做是为了避免创建大量包装函数来覆盖 get**()、pin**()、FOLL_LONGTERM 等的所有组合。此外，pin_user_pages**() API 与 get_user_pages**() API 明显不同，因此这是一个自然的划分线，也是进行单独包装调用的好切入点。换句话说，对 DMA-pinned 页使用 pin_user_pages*()，对其他情况使用 get_user_pages*()。本文档后面描述了五种情况，以进一步阐明这一概念。

对于给定的 gup 调用，FOLL_PIN 与 FOLL_GET 是互斥的。不过，多个线程和调用点可以通过 FOLL_PIN 和 FOLL_GET 自由地 pin 相同的 struct page。需要选择其中之一的是调用点，而不是 struct page。

FOLL_PIN 的实现与 FOLL_GET 几乎相同，只是 FOLL_PIN 使用了不同的引用计数技术。

FOLL_PIN 是 FOLL_LONGTERM 的先决条件。换句话说，FOLL_LONGTERM 是 FOLL_PIN 的一种更受限的特定情况。

## 每个包装函数设置了哪些标志

对于这些 pin_user_pages*() 函数，FOLL_PIN 与调用者提供的任何 gup 标志进行 OR 运算。调用者需要传入一个非空的 struct pages* 数组，然后函数通过为每个页增加一个特殊值：GUP_PIN_COUNTING_BIAS 来 pin 页。

对于大 folio（large folios），不使用 GUP_PIN_COUNTING_BIAS 方案。相反，使用 struct folio 中可用的额外空间来直接存储 pincount。

这种针对大 folio 的方法避免了下面讨论的计数上限问题。那些限制会被巨页（huge pages）严重加剧，因为每个尾页都会向头页添加一个引用计数。事实上，测试表明，在没有单独 pincount 字段的情况下，在某些巨页压力测试中观察到了引用计数溢出。

这也意味着巨页和大 folio 不会遭受
```

 Function
 --------
 pin_user_pages          FOLL_PIN 总是由此函数在内部设置。
 pin_user_pages_fast     FOLL_PIN 总是由此函数在内部设置。
 pin_user_pages_remote   FOLL_PIN 总是由此函数在内部设置。

```
对于这些 get_user_pages*() 函数，可能根本不会指定 FOLL_GET。行为比上面稍复杂一些。如果 **没有**指定 FOLL_GET，但调用者传入了非空的 struct pages* 数组，那么函数会为你设置 FOLL_GET，并继续通过增加引用计数来 pin 页
```

 Function
 --------
 get_user_pages           FOLL_GET 有时由此函数在内部设置。
 get_user_pages_fast      FOLL_GET 有时由此函数在内部设置。
 get_user_pages_remote    FOLL_GET 有时由此函数在内部设置。

```
## 跟踪 dma-pinned 页

跟踪 dma-pinned 页的一些关键设计约束与解决方案：

- 需要每个 struct page 一个实际的引用计数。这是因为多个进程可能会 pin 和 unpin 一个页。

- 假阳性（报告一个页被 dma-pinned，而实际上并非如此）是可以接受的，但假阴性不行。

- 为此不能增大 struct page 的大小，而且所有字段都已被使用。

- 鉴于以上，我们可以通过使用 page->_refcount 字段中“某种意义上的”高位来重载该字段，以作为 dma-pinned 计数。“某种意义上的”意味着，我们不将 page->_refcount 划分为位字段，而是简单地将一个中等大小的值（GUP_PIN_COUNTING_BIAS，最初选为 1024：10 位）加到 page->_refcount 上。这提供了模糊的行为：如果一个页被调用了 1024 次 get_page()，那么它将表现为具有单个 dma-pinned 计数。再次说明，这是可接受的。

这也带来了限制：只有 31-10==21 位可用于一个每次递增 10 位的计数器。

- 由于该限制，使用 FOLL_PIN 时对零页（zero pages）做了特殊处理。我们只是假装 pin 了一个零页——根本不改变其引用计数或 pincount（它是永久的，因此没有必要）。unpinning 函数对零页也不做任何操作。这对调用者是透明的。

- 调用者必须显式请求“页的 dma-pinned 跟踪”。换句话说，仅仅调用 get_user_pages() 是不够的；必须使用一组新函数，即 pin_user_page() 及其相关函数。

## FOLL_PIN、FOLL_GET、FOLL_LONGTERM：何时使用哪个标志

感谢 Jan Kara、Vlastimil Babka 以及其他几位 -mm 人员描述了这些类别：

### CASE 1: 直接 IO（DIO）

存在 GUP 引用，这些页作为 DIO 缓冲区。这些缓冲区需要的时间相对较短（因此它们不是“长期的”）。与 folio_mkclean() 或 munmap() 没有特殊的同步
```

    FOLL_PIN

```
……但与其直接设置 FOLL_PIN，调用点应当使用设置了 FOLL_PIN 的 pin_user_pages*() 例程之一。

### CASE 2: RDMA

存在 GUP 引用，这些页作为 DMA 缓冲区。这些缓冲区需要很长时间（“长期”）。没有提供与 folio_mkclean() 或 munmap() 的特殊同步。因此标志
```

    FOLL_PIN | FOLL_LONGTERM

```
注意：某些页，例如 DAX 页，无法被长期 pin。这是因为 DAX 页没有单独的页缓存，因此“pinning”意味着锁定文件系统块，而（目前）还不以支持这种方式。

### CASE 3: MMU notifier 注册，带或不带缺页硬件

设备驱动可以通过 get_user_pages*() pin 页，并为该内存范围注册 mmu notifier 回调。然后，在收到 notifier 的“invalidate range”回调时，停止设备使用该范围，并 unpin 这些页。可能还有其他可行的方案，例如显式地针对待处理的 IO 进行同步，以达到大致相同的效果。

或者，如果硬件支持可重放缺页（replayable page faults），那么设备驱动可以完全避免 pinning（这是理想的），如下所示：像上面一样注册 mmu notifier 回调，但不是在回调中停止设备并 unpin，而只是将该范围从设备的页表中移除。

无论哪种方式，只要驱动在 mmu notifier 回调时 unpin 这些页，就与文件系统和 mm（folio_mkclean()、munmap() 等）有了适当的同步。因此，不需要设置任何一个标志。

### CASE 4: 仅为 struct page 操作而 pinning

如果只影响 struct page 数据（与页所追踪的实际内存内容相对），那么普通的 GUP 调用就足够了，不需要设置任何一个标志。

### CASE 5: 为了写入页内数据而 pinning

即使不涉及 DMA 或直接 IO，仅仅是“pin、写入页数据、unpin”这样简单的情况也会造成问题。CASE 5 可以被视为 CASE 1 加上 CASE 2 再加上任何调用该模式的情况的超集。换句话说，如果代码既不是 CASE 1 也不是 CASE 2，它仍然可能需要 FOLL_PIN，对于如下这样的模式：

正确（使用 FOLL_PIN 调用）：
    pin_user_pages()
    写入这些页内的数据
    unpin_user_pages()

错误（使用 FOLL_GET 调用）：
    get_user_pages()
    写入这些页内的数据
    put_page()

## folio_maybe_dma_pinned()：pinning 的全部意义

将 folio 标记为“DMA-pinned”或“gup-pinned”的全部意义在于能够查询“这个 folio 是否被 DMA-pinned？”这使得诸如 folio_mkclean()（以及一般的文件系统回写代码）之类的代码能够在由于此类 pin 而无法解除映射某个 folio 时，对要做什么做出明智的决定。

在这些情况下该做什么，是长达数年的讨论与争论的主题（参见本文档末尾的参考文献）。这是一个 TODO 项：待该问题理清后补全细节。同时，可以肯定地说
```

        static inline bool folio_maybe_dma_pinned(struct folio *folio)

```
……是解决长期存在的 gup+DMA 问题的先决条件。

## 思考 FOLL_GET、FOLL_PIN 和 FOLL_LONGTERM 的另一种方式

思考这些标志的另一种方式是作为一系列逐步加强的限制：FOLL_GET 用于 struct page 操作，不影响 struct page 所引用的数据。FOLL_PIN 是 FOLL_GET 的**替代品**，用于对其数据**将被**访问的页进行短期 pin。因此，FOLL_PIN 是一种“更严格”的 pin 形式。最后，FOLL_LONGTERM 是一个限制更强的、以 FOLL_PIN 为先决条件的情况：这用于将被长期 pin 且其数据将被访问的页。

## 单元测试

```

 tools/testing/selftests/mm/gup_test.c

```
有以下新的调用用于演练新的 pin*() 包装函数：

- PIN_FAST_BENCHMARK (./gup_test -a)
- PIN_BASIC_TEST (./gup_test -b)

你可以监控已获取和已释放的 dma-pinned 页总数
```

    /proc/vmstat/nr_foll_pin_acquired
    /proc/vmstat/nr_foll_pin_released

```
在正常情况下，这两个值相等，除非存在任何长期 [R]DMA pin，或处于 pin/unpin 转换期间。

- nr_foll_pin_acquired：自系统上电以来已获取的 logical pins 数量。对于巨页，头页被 pin 一次（针对巨页中的每个页——头页和每个尾页）。这遵循与 get_user_pages() 用于巨页的相同行为：当 get_user_pages() 应用于巨页时，头页针对巨页中的每个尾页或头页被引用计数一次。

- nr_foll_pin_released：自系统上电以来已释放的 logical pins 数量。注意，页是以 PAGE_SIZE 粒度释放（unpin）的，即使最初的 pin 是应用于巨页。由于上面“nr_foll_pin_acquired”中描述的 pin 计数的行为，
```

    pin_user_pages(huge_page);
    for (each page in huge_page)
        unpin_user_page(page);

```
```

    nr_foll_pin_released == nr_foll_pin_acquired

```
（……除非由于已有的长期 RDMA pin 而已经失去平衡。）

## 其他诊断

dump_page() 已被略微增强以处理这些新的计数字段，并更好地报告大 folio。具体来说，对于大 folio，会报告精确的 pincount。

## 参考文献

- `Some slow progress on get_user_pages() (Apr 2, 2019) <https://lwn.net/Articles/784574/>`_
- `DMA and get_user_pages() (LPC: Dec 12, 2018) <https://lwn.net/Articles/774411/>`_
- `The trouble with get_user_pages() (Apr 30, 2018) <https://lwn.net/Articles/753027/>`_
- `LWN kernel index: get_user_pages() <https://lwn.net/Kernel/Index/#Memory_management-get_user_pages>`_

John Hubbard，2019 年 10 月
