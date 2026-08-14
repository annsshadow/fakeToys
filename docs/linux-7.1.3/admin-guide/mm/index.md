## 内存管理


顾名思义，Linux 内存管理子系统负责管理系统中的内存。这包括虚拟内存和需求分页的实现、为内核内部结构和用户空间程序分配内存、将文件映射到进程地址空间，以及许多其它很棒的功能。

Linux 内存管理是一个具有许多可配置设置的复杂系统。大多数设置可通过 `/proc` 文件系统获得，并可以使用 `sysctl` 查询和调整。这些 API 在 Documentation/admin-guide/sysctl/vm.rst 和 `man 5 proc`_ 中有描述。

Linux 内存管理有其自身的术语，如果你还不熟悉，建议阅读 Documentation/admin-guide/mm/concepts.rst。

这里我们将详细记录如何与 Linux 内存管理中的各种机制交互。

- [concepts](concepts)
- [cma_debugfs](cma_debugfs)
- [damon/index](damon/index)
- [hugetlbpage](hugetlbpage)
- [idle_page_tracking](idle_page_tracking)
- [ksm](ksm)
- [memory-hotplug](memory-hotplug)
- [multigen_lru](multigen_lru)
- [nommu-mmap](nommu-mmap)
- [numa_memory_policy](numa_memory_policy)
- [numaperf](numaperf)
- [pagemap](pagemap)
- [shrinker_debugfs](shrinker_debugfs)
- [slab](slab)
- [soft-dirty](soft-dirty)
- [transhuge](transhuge)
- [userfaultfd](userfaultfd)
- [zswap](zswap)
- [kho](kho)
