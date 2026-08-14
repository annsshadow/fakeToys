
## 内存管理


## 采用 4 级页表的完整虚拟内存映射


 - 像 "-23 TB" 这样的负地址是绝对地址（以字节计），从 64 位地址空间的顶部向下计数。同时以绝对地址和距顶部的距离这两种方式来看布局，会更容易理解。

   例如 0xffffe90000000000 == -23 TB，它比 64 位地址空间的顶部（ffffffffffffffff）低 23 TB。

   注意，当我们越靠近地址空间顶部时，计数单位会从 TB 变为 GB，再到 MB/KB。

 - "16M TB" 乍一看可能有些奇怪，但它比 "16 EB" 更直观地表示大小——很少会有人一眼就认出 16 EB 是 16 艾字节（exabytes）。它也很好地展示了 64 位地址空间到底有多么巨大。

```

  ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
  ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 00007fffffffefff | ~128 TB | user-space virtual memory, different per mm
   00007ffffffff000 | ~128    TB | 00007fffffffffff |    4 kB | ... guard hole
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0000800000000000 | +128    TB | 7fffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -8 EB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM relaxes canonicallity check allowing to create aliases
                    |            |                  |         | for userspace memory here.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   8000000000000000 |   -8    EB | ffff7fffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -128 TB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM_SUP relaxes canonicallity check allowing to create
                    |            |                  |         | aliases for kernel memory here.
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ffff800000000000 | -128    TB | ffff87ffffffffff |    8 TB | ... guard hole, also reserved for hypervisor
   ffff880000000000 | -120    TB | ffff887fffffffff |  0.5 TB | LDT remap for PTI
   ffff888000000000 | -119.5  TB | ffffc87fffffffff |   64 TB | direct mapping of all physical memory (page_offset_base)
   ffffc88000000000 |  -55.5  TB | ffffc8ffffffffff |  0.5 TB | ... unused hole
   ffffc90000000000 |  -55    TB | ffffe8ffffffffff |   32 TB | vmalloc/ioremap space (vmalloc_base)
   ffffe90000000000 |  -23    TB | ffffe9ffffffffff |    1 TB | ... unused hole
   ffffea0000000000 |  -22    TB | ffffeaffffffffff |    1 TB | virtual memory map (vmemmap_base)
   ffffeb0000000000 |  -21    TB | ffffebffffffffff |    1 TB | ... unused hole
   ffffec0000000000 |  -20    TB | fffffbffffffffff |   16 TB | KASAN shadow memory
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 56-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   fffffc0000000000 |   -4    TB | fffffdffffffffff |    2 TB | ... unused hole
                    |            |                  |         | vaddr_end for KASLR
   fffffe0000000000 |   -2    TB | fffffe7fffffffff |  0.5 TB | cpu_entry_area mapping
   fffffe8000000000 |   -1.5  TB | fffffeffffffffff |  0.5 TB | ... unused hole
   ffffff0000000000 |   -1    TB | ffffff7fffffffff |  0.5 TB | %esp fixup stacks
   ffffff8000000000 | -512    GB | ffffffeeffffffff |  444 GB | ... unused hole
   ffffffef00000000 |  -68    GB | fffffffeffffffff |   64 GB | EFI region mapping space
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | ... unused hole
   ffffffff80000000 |   -2    GB | ffffffff9fffffff |  512 MB | kernel text mapping, mapped to physical address 0
   ffffffff80000000 |-2048    MB |                  |         |
   ffffffffa0000000 |-1536    MB | fffffffffeffffff | 1520 MB | module mapping space
   ffffffffff000000 |  -16    MB |                  |         |
      FIXADDR_START | ~-11    MB | ffffffffff5fffff | ~0.5 MB | kernel-internal fixmap range, variable size and offset
   ffffffffff600000 |  -10    MB | ffffffffff600fff |    4 kB | legacy vsyscall ABI
   ffffffffffe00000 |   -2    MB | ffffffffffffffff |    2 MB | ... unused hole
  __________________|____________|__________________|_________|___________________________________________________________


```
## 采用 5 级页表的完整虚拟内存映射


 - 对于 56 位地址，用户空间内存扩大了 512 倍，从 0.125 PB 增加到 64 PB。所有的内核映射都下移到了 -64 PB 的起始偏移处，并且许多区域也进行了扩展，以支持大得多的物理内存。

```

  ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
  ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 00fffffffffff000 |  ~64 PB | user-space virtual memory, different per mm
   00fffffffffff000 |  ~64    PB | 00ffffffffffffff |    4 kB | ... guard hole
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0100000000000000 |  +64    PB | 7fffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -8EB TB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM relaxes canonicallity check allowing to create aliases
                    |            |                  |         | for userspace memory here.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  ____________________________________________________________|___________________________________________________________
   8000000000000000 |   -8    EB | feffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -64 PB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM_SUP relaxes canonicallity check allowing to create
                    |            |                  |         | aliases for kernel memory here.
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ff00000000000000 |  -64    PB | ff0fffffffffffff |    4 PB | ... guard hole, also reserved for hypervisor
   ff10000000000000 |  -60    PB | ff10ffffffffffff | 0.25 PB | LDT remap for PTI
   ff11000000000000 |  -59.75 PB | ff90ffffffffffff |   32 PB | direct mapping of all physical memory (page_offset_base)
   ff91000000000000 |  -27.75 PB | ff9fffffffffffff | 3.75 PB | ... unused hole
   ffa0000000000000 |  -24    PB | ffd1ffffffffffff | 12.5 PB | vmalloc/ioremap space (vmalloc_base)
   ffd2000000000000 |  -11.5  PB | ffd3ffffffffffff |  0.5 PB | ... unused hole
   ffd4000000000000 |  -11    PB | ffd5ffffffffffff |  0.5 PB | virtual memory map (vmemmap_base)
   ffd6000000000000 |  -10.5  PB | ffdeffffffffffff | 2.25 PB | ... unused hole
   ffdf000000000000 |   -8.25 PB | fffffbffffffffff |   ~8 PB | KASAN shadow memory
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 47-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   fffffc0000000000 |   -4    TB | fffffdffffffffff |    2 TB | ... unused hole
                    |            |                  |         | vaddr_end for KASLR
   fffffe0000000000 |   -2    TB | fffffe7fffffffff |  0.5 TB | cpu_entry_area mapping
   fffffe8000000000 |   -1.5  TB | fffffeffffffffff |  0.5 TB | ... unused hole
   ffffff0000000000 |   -1    TB | ffffff7fffffffff |  0.5 TB | %esp fixup stacks
   ffffff8000000000 | -512    GB | ffffffeeffffffff |  444 GB | ... unused hole
   ffffffef00000000 |  -68    GB | fffffffeffffffff |   64 GB | EFI region mapping space
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | ... unused hole
   ffffffff80000000 |   -2    GB | ffffffff9fffffff |  512 MB | kernel text mapping, mapped to physical address 0
   ffffffff80000000 |-2048    MB |                  |         |
   ffffffffa0000000 |-1536    MB | fffffffffeffffff | 1520 MB | module mapping space
   ffffffffff000000 |  -16    MB |                  |         |
      FIXADDR_START | ~-11    MB | ffffffffff5fffff | ~0.5 MB | kernel-internal fixmap range, variable size and offset
   ffffffffff600000 |  -10    MB | ffffffffff600fff |    4 kB | legacy vsyscall ABI
   ffffffffffe00000 |   -2    MB | ffffffffffffffff |    2 MB | ... unused hole
  __________________|____________|__________________|_________|___________________________________________________________


```
该架构定义了一个 64 位虚拟地址。实现可以支持更少。目前支持的是 48 位和 57 位虚拟地址。从第 63 位到最高有效实现位之间的位进行符号扩展。如果你将它们解释为无符号数，这就在用户空间和内核地址之间造成了一个空洞。

直接映射覆盖了系统中直到最高内存地址的所有内存（这意味着在某些情况下它也可能包含 PCI 内存空洞）。

我们将 EFI 运行时服务映射在 'efi_pgd' PGD 中，位于一个 64GB 大小的虚拟内存窗口中（这个大小是任意的，如果将来需要可以调大）。这些映射不属于任何其它内核 PGD 的一部分，并且仅在 EFI 运行时调用期间可用。

注意，如果启用了 CONFIG_RANDOMIZE_MEMORY，则所有物理内存的直接映射、vmalloc/ioremap 空间以及虚拟内存映射都会被随机化。它们的顺序得以保留，但它们的基址会在启动早期被偏移。

在修改这里的任何内容时，要非常小心 KASLR。KASLR 地址范围不得与除 KASAN 影子区以外的任何东西重叠，这是正确的，因为 KASAN 会禁用 KASLR。

对于 4 级和 5 级两种布局，最后一个 2MB 空洞处的 KSTACK_ERASE_POISON 值为：ffffffffff4111
