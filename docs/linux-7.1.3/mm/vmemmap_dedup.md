

## 为 HugeTLB 与 Device DAX 削减 vmemmap

## HugeTLB

本节用于解释 HugeTLB Vmemmap 优化（HVO）的工作原理。

`struct page` 结构体用于描述一个物理页帧。默认情况下，页帧与其对应的 `struct page` 之间存在一对一映射。

HugeTLB 页由多个基本页大小的页组成，并得到许多体系结构的支持。更多细节参见 Documentation/admin-guide/mm/hugetlbpage.rst。在 x86-64 体系结构上，目前支持 2MB 和 1GB 大小的 HugeTLB 页。由于 x86 上的基本页大小为 4KB，一个 2MB 的 HugeTLB 页由 512 个基本页组成，而一个 1GB 的 HugeTLB 页由 262144 个基本页组成。对于每个基本页，都对应有一个 `struct page`。

在 HugeTLB 子系统内部，只有前 4 个 `struct page` 被用来存放关于某个 HugeTLB 页的唯一信息。`__NR_USED_SUBPAGE` 提供了这一上限。其余 `struct page` 中唯一“有用”的信息是 compound_info 字段，而该字段对所有尾页都是相同的。

通过移除 HugeTLB 页中冗余的 `struct page`，可以将内存归还给伙伴分配器以作他用。

不同的体系结构支持不同的 HugeTLB 页。例如，下表是 x86 和 arm64 体系结构所支持的 HugeTLB 页大小。由于 arm64 支持 4k、16k 和 64k 的基本页，并且支持连续表项（contiguous entries），因此它支持多种大小的 HugeTLB 页。

+--------------+-----------+-----------------------------------------------+
| Architecture | Page Size |                HugeTLB Page Size              |
+--------------+-----------+-----------+-----------+-----------+-----------+
|    x86-64    |    4KB    |    2MB    |    1GB    |           |           |
+--------------+-----------+-----------+-----------+-----------+-----------+
|              |    4KB    |   64KB    |    2MB    |    32MB   |    1GB    |
|              +-----------+-----------+-----------+-----------+-----------+
|    arm64     |   16KB    |    2MB    |   32MB    |     1GB   |           |
|              +-----------+-----------+-----------+-----------+-----------+
|              |   64KB    |    2MB    |  512MB    |    16GB   |           |
+--------------+-----------+-----------+-----------+-----------+-----------+

当系统启动时，每个 HugeTLB 页都拥有多于一个 `struct page`
```

   struct_size = HugeTLB_Size / PAGE_SIZE * sizeof(struct page) / PAGE_SIZE

```
其中 HugeTLB_Size 是 HugeTLB 页的大小。我们知道 HugeTLB 页的大小始终是 PAGE_SIZE 的 n 倍。因此可以得到下式
```

   HugeTLB_Size = n * PAGE_SIZE

```
```
   struct_size = n * PAGE_SIZE / PAGE_SIZE * sizeof(struct page) / PAGE_SIZE
               = n * sizeof(struct page) / PAGE_SIZE

```
我们可以对 HugeTLB 页在 pud/pmd 级别使用大页映射。
```

   struct_size = n * sizeof(struct page) / PAGE_SIZE
               = PAGE_SIZE / sizeof(pte_t) * sizeof(struct page) / PAGE_SIZE
               = sizeof(struct page) / sizeof(pte_t)
               = 64 / 8
               = 8 (pages)

```
其中 n 是一个页所能包含的 pte 表项数量。所以 n 的值为 (PAGE_SIZE / sizeof(pte_t))。

该优化仅支持 64 位系统，因此 sizeof(pte_t) 的值为 8。并且该优化仅在 `struct page` 的大小为 2 的幂时才适用。在大多数情况下，`struct page` 的大小为 64 字节（例如 x86-64 和 arm64）。因此，如果我们对某个 HugeTLB 页使用 pmd 级别的映射，其 `struct page` 结构体所占的大小为 8 个页帧，具体大小取决于基本页的大小。
```

   struct_size = PAGE_SIZE / sizeof(pmd_t) * struct_size(pmd)
               = PAGE_SIZE / 8 * 8 (pages)
               = PAGE_SIZE (pages)

```
其中 struct_size(pmd) 是采用 pmd 级别映射的 HugeTLB 页的 `struct page` 结构体的大小。

例如：x86_64 上一个 2MB 的 HugeTLB 页由 8 个页帧组成，而 1GB 的 HugeTLB 页由 4096 个页帧组成。

接下来，我们以 HugeTLB 页的 pmd 级别映射为例，展示该优化的内部实现。采用 pmd 映射的 HugeTLB 页关联有 8 个页的 `struct page` 结构体。
```

    HugeTLB                  struct pages(8 pages)         page frame(8 pages)
 +-----------+ ---virt_to_page---> +-----------+   mapping to   +-----------+
 |           |                     |     0     | -------------> |     0     |
 |           |                     +-----------+                +-----------+
 |           |                     |     1     | -------------> |     1     |
 |           |                     +-----------+                +-----------+
 |           |                     |     2     | -------------> |     2     |
 |           |                     +-----------+                +-----------+
 |           |                     |     3     | -------------> |     3     |
 |           |                     +-----------+                +-----------+
 |           |                     |     4     | -------------> |     4     |
 |    PMD    |                     +-----------+                +-----------+
 |   level   |                     |     5     | -------------> |     5     |
 |  mapping  |                     +-----------+                +-----------+
 |           |                     |     6     | -------------> |     6     |
 |           |                     +-----------+                +-----------+
 |           |                     |     7     | -------------> |     7     |
 |           |                     +-----------+                +-----------+
 |           |
 |           |
 |           |
 +-----------+

```
与 HugeTLB 页关联的第一个 `struct page`（页 0）包含描述该 HugeTLB 所必需的 4 个 `struct page`。其余的 `struct page`（页 1 到页 7）是尾页。

该优化仅当 struct page 的大小为 2 的幂时才应用。在这种情况下，所有相同阶数的尾页都是相同的。参见 compound_head()。这使我们能够将 vmemmap 的尾页重映射到一个共享的、只读的页。头页也被重映射到一个新页。这使得原始的 vmemmap 页得以释放。
```

    HugeTLB                  struct pages(8 pages)                 page frame (new)
 +-----------+ ---virt_to_page---> +-----------+   mapping to   +----------------+
 |           |                     |     0     | -------------> |       0        |
 |           |                     +-----------+                +----------------+
 |           |                     |     1     | ------┐
 |           |                     +-----------+       |
 |           |                     |     2     | ------┼        +----------------------------+
 |           |                     +-----------+       |        | A single, per-zone page    |
 |           |                     |     3     | ------┼------> | frame shared among all     |
 |           |                     +-----------+       |        | hugepages of the same size |
 |           |                     |     4     | ------┼        +----------------------------+
 |           |                     +-----------+       |
 |           |                     |     5     | ------┼
 |    PMD    |                     +-----------+       |
 |   level   |                     |     6     | ------┼
 |  mapping  |                     +-----------+       |
 |           |                     |     7     | ------┘
 |           |                     +-----------+
 |           |
 |           |
 |           |
 +-----------+

```
当某个 HugeTLB 被释放回伙伴系统时，我们应当分配 7 个页用于 vmemmap 页，并恢复先前的映射关系。

对于采用 pud 级别映射的 HugeTLB 页，情况与前文类似。我们同样可以用这种方法来释放 (PAGE_SIZE - 1) 个 vmemmap 页。

除了 pmd/pud 级别映射的 HugeTLB 页之外，某些体系结构（例如 aarch64）在转换表项（translation table entry）中提供了一个连续位（contiguous bit），用于向 MMU 提示：它是一组连续表项中的一个，这些表项可以被缓存到单个 TLB 表项中。

连续位用于在 pmd 和 pte（最后一级）级别增大映射尺寸。因此这类 HugeTLB 页仅当其 `struct page` 结构体的大小大于 **1** 个页时才能被优化。

## Device DAX

device-dax 接口使用了前一章所介绍的相同尾页去重技术，唯一的例外是它与设备中的 vmemmap（altmap）一起使用。

DAX 中支持以下页大小：PAGE_SIZE（x86_64 上为 4K）、PMD_SIZE（x86_64 上为 2M）以及 PUD_SIZE（x86_64 上为 1G）。关于 powerpc 上等效的细节，参见 Documentation/arch/powerpc/vmemmap_dedup.rst。

其与 HugeTLB 的差异相对较小。

它仅使用 3 个 `struct page` 来存储全部信息，而不是 HugeTLB 页所需的 4 个。

由于 device-dax 内存并非启动时初始化的 System RAM 范围的一部分，因此不存在 vmemmap 的重映射。于是尾页去重发生在我们填充段（sections）的较晚阶段。HugeTLB 复用所代表头部的 vmemmap 页，而 device-dax 复用尾部的 vmemmap 页。这导致其相比 HugeTLB 只能节省一半。

去重后的尾页不会被映射为只读。
```

 +-----------+ ---virt_to_page---> +-----------+   mapping to   +-----------+
 |           |                     |     0     | -------------> |     0     |
 |           |                     +-----------+                +-----------+
 |           |                     |     1     | -------------> |     1     |
 |           |                     +-----------+                +-----------+
 |           |                     |     2     | ----------------^ ^ ^ ^ ^ ^
 |           |                     +-----------+                   | | | | |
 |           |                     |     3     | ------------------+ | | | |
 |           |                     +-----------+                     | | | |
 |           |                     |     4     | --------------------+ | | |
 |    PMD    |                     +-----------+                       | | |
 |   level   |                     |     5     | ----------------------+ | |
 |  mapping  |                     +-----------+                         | |
 |           |                     |     6     | ------------------------+ |
 |           |                     +-----------+                           |
 |           |                     |     7     | --------------------------+
 |           |                     +-----------+
 |           |
 |           |
 |           |
 +-----------+

```
