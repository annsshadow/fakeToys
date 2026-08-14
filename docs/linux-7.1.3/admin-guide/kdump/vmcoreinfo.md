## VMCOREINFO


## 它是什么？

VMCOREINFO 是一个特殊的 ELF note 段。它包含来自内核的多种信息，例如结构大小、页大小、
符号值、字段偏移等。这些数据被打包进一个 ELF note 段，并被 crash、makedumpfile 等
用户空间工具用于分析内核的内存布局。

## 通用变量


### init_uts_ns.name.release


Linux 内核的版本号。用于找到构建该内核所对应的源代码。例如，crash 用它来查找对应的
vmlinux，以便处理 vmcore。

### PAGE_SIZE


页的大小。它是内存管理设施所使用的最小数据单位。其大小通常为 4096 字节，且页按 4096
字节对齐。用于计算页地址。

### init_uts_ns


UTS 命名空间，用于隔离系统中与 uname(2) 系统调用相关的两个特定元素。它以用于存储
uname(2) 系统调用所返回信息的数据结构命名。

用户空间工具可以从中获取内核名称、主机名、内核发布号、内核版本、架构名和 OS 类型。

### (uts_namespace, name)


name 成员的偏移量。Crash Utility 与 Makedumpfile 据此获取 init_uts_ns.name 的起始地址。

### node_online_map


数组 node_states[N_ONLINE]，表示系统中在线节点的集合，每个节点号对应一个比特位。
用于跟踪哪些节点在系统中且处于在线状态。

### swapper_pg_dir


内核的全局页目录指针。用于将虚拟地址转换为物理地址。

### _stext


定义 text 段的起始位置。通常，_stext 表示内核的起始地址。用于将来自内核直接映射的
虚拟地址转换为物理地址。

### VMALLOC_START


存储 vmalloc 区域的基地址。makedumpfile 需要获取该值，因为它对 vmalloc 转换是必要的。

### mem_map


物理地址通过将其作为 mem_map 数组的索引来转换为 struct page。将物理地址右移
PAGE_SHIFT 位即可将其转换为页帧号，也就是该 mem_map 数组的索引。

用于将地址映射到对应的 struct page。

### contig_page_data


Makedumpfile 从该符号获取 pglist_data 结构，该结构用于描述内存布局。

用户空间工具利用它来在转储内存时排除空闲页。

### mem_section|(mem_section, NR_SECTION_ROOTS)|(mem_section, section_mem_map)


mem_section 数组的地址、其长度、结构大小，以及 section_mem_map 的偏移量。

它存在于稀疏内存映射模型中，并且与 mem_map 变量有些类似，二者都用于转换地址。

### MAX_PHYSMEM_BITS


定义所支持的最大物理地址空间内存。

### page


page 结构的大小。struct page 是一个重要的数据结构，被广泛用于计算连续内存。

### pglist_data


pglist_data 结构的大小。该值用于检查 pglist_data 结构是否有效。它也用于检查内存类型。

### zone


zone 结构的大小。该值用于检查是否已找到 zone 结构。它也用于排除空闲页。

### free_area


free_area 结构的大小。它指示 free_area 结构是否有效。在排除空闲页时很有用。

### list_head


list_head 结构的大小。用于在后验分析会话中遍历链表。

### nodemask_t


nodemask_t 类型的大小。用于计算在线节点的数量。

### (page, flags|_refcount|mapping|lru|_mapcount|private|compound_order|compound_info)


用户空间工具基于这些变量的偏移量来计算它们的值。这些变量在排除不必要的页时使用。

### (pglist_data, node_zones|nr_zones|node_mem_map|node_start_pfn|node_spanned_pages|node_id)


在 NUMA 机器上，每个 NUMA 节点都有一个 pg_data_t 来描述其内存布局。在 UMA 机器上，
只有一个 pglist_data 用于描述整个内存。

这些值用于检查内存类型，并计算内存映射的虚拟地址。

### (zone, free_area|vm_stat|spanned_pages)


每个节点被划分为若干个称为 zone 的块，它们表示内存中的范围。一个 zone 由 zone 结构来描述。

用户空间工具基于这些变量的偏移量来计算所需的值。

### (free_area, free_list)


free_list 成员的偏移量。该值用于计算空闲页的数量。

每个 zone 都有一个名为 free_area[NR_PAGE_ORDERS] 的 free_area 结构数组。
free_list 表示空闲页块的链表。

### (list_head, next|prev)


list_head 各成员的偏移量。list_head 用于定义循环链表。用户空间工具需要它们以便遍历链表。

### (vmap_area, va_start|list)


vmap_area 各成员的偏移量。它们携带 vmalloc 特有的信息。Makedumpfile 据此获取 vmalloc
区域的起始地址。

### (zone.free_area, NR_PAGE_ORDERS)


空闲区域描述符。用户空间工具使用该值来遍历 free_area 范围。NR_PAGE_ORDERS 由 zone
伙伴分配器使用。

### prb


指向 printk 环形缓冲区（struct printk_ringbuffer）的指针。根据核心转储发生的时机，
它可能已经指向静态启动环形缓冲区，也可能指向动态分配的环形缓冲区。
由用户空间工具用于读取当前活跃的内核日志缓冲区。

### printk_rb_static


指向静态启动 printk 环形缓冲区的指针。如果 @prb 的值不同，这对于查看初始启动消息很有用，
那些消息可能已在动态分配的环形缓冲区中被覆盖。

### clear_seq


上一次 clear 命令之后的 printk() 记录序号。它表示上一次 SYSLOG_ACTION_CLEAR
（例如由 'dmesg -c' 发出的）之后的第一条记录。由用户空间工具用于转储 dmesg 日志的一个子集。

### printk_ringbuffer


printk_ringbuffer 结构的大小。该结构包含访问内核日志缓冲区各个组成部分所需的全部信息。

### (printk_ringbuffer, desc_ring|text_data_ring|dict_data_ring|fail)


printk 环形缓冲区各组成部分的偏移量。由用户空间工具用于在不要求声明该结构的情况下查看
内核日志缓冲区。

### prb_desc_ring


prb_desc_ring 结构的大小。该结构包含关于一组记录描述符的信息。

### (prb_desc_ring, count_bits|descs|head_id|tail_id)


描述一组记录描述符的各个字段的偏移量。由用户空间工具用于在不要求声明该结构的情况下
遍历这些描述符。

### prb_desc


prb_desc 结构的大小。该结构包含关于单个记录描述符的信息。

### (prb_desc, info|state_var|text_blk_lpos|dict_blk_lpos)


描述一个记录描述符的各个字段的偏移量。由用户空间工具用于在不要求声明该结构的情况下
读取这些描述符。

### prb_data_blk_lpos


prb_data_blk_lpos 结构的大小。该结构包含关于文本或字典数据（数据块）在相应数据环形
缓冲区中位置的信息。

### (prb_data_blk_lpos, begin|next)


描述一个数据块位置的各个字段的偏移量。由用户空间工具用于在不要求声明该结构的情况下
定位数据块。

### printk_info


printk_info 结构的大小。该结构包含一条记录的全部元数据。

### (printk_info, seq|ts_nsec|text_len|dict_len|caller_id)


提供一条记录元数据的各个字段的偏移量。由用户空间工具用于在不要求声明该结构的情况下
读取该信息。

### prb_data_ring


prb_data_ring 结构的大小。该结构包含关于一组数据块的信息。

### (prb_data_ring, size_bits|data|head_lpos|tail_lpos)


描述一组数据块的各个字段的偏移量。由用户空间工具用于在不要求声明该结构的情况下
访问这些数据块。

### atomic_long_t


atomic_long_t 结构的大小。由用户空间工具用于能够复制整个结构，而不论其架构相关的实现。

### (atomic_long_t, counter)


atomic_long_t 变量的长整型值的偏移量。由用户空间工具用于在不要求架构相关声明的情况下
访问该长整型值。

### (free_area.free_list, MIGRATE_TYPES)


页的迁移类型数量。free_list 由该数组描述。由工具用于计算空闲页的数量。

### NR_FREE_PAGES


在 linux-2.6.21 或更高版本中，空闲页数量位于 vm_stat[NR_FREE_PAGES]。用于获取空闲页
数量。

### PG_lru|PG_private|PG_swapcache|PG_swapbacked|PG_hwpoison|PG_head_mask


页属性。这些标志用于过滤各种转储时不需要的页。

### PAGE_SLAB_MAPCOUNT_VALUE|PAGE_BUDDY_MAPCOUNT_VALUE|PAGE_OFFLINE_MAPCOUNT_VALUE|PAGE_HUGETLB_MAPCOUNT_VALUE|PAGE_UNACCEPTED_MAPCOUNT_VALUE


更多页属性。这些标志用于过滤各种转储时不需要的页。


## x86_64


### phys_base


用于将导出的内核符号的虚拟地址转换为其对应的物理地址。

### init_top_pgt


用于遍历整个页表并将虚拟地址转换为物理地址。init_top_pgt 与 swapper_pg_dir 有些类似，
但仅在 x86_64 中使用。

### pgtable_l5_enabled


用户空间工具需要了解崩溃内核是否处于 5 级分页模式。

### node_data


这是一个 struct pglist_data 数组，存储所有 NUMA 节点的信息。Makedumpfile 从中获取
pglist_data 结构。

### (node_data, MAX_NUMNODES)


系统中节点的最大数量。

### KERNELOFFSET


内核随机化偏移量。用于计算页偏移。如果 KASLR 被禁用，则该值为零。

### KERNEL_IMAGE_SIZE


目前未被 Makedumpfile 使用。用于由 Crash 计算模块虚拟地址。

### sme_mask


AMD 特有，支持 SME：它表示安全内存加密掩码。Makedumpfile 工具需要了解崩溃内核是否
被加密。如果第一个内核启用了 SME，崩溃内核的页表项（pgd/pud/pmd/pte）中包含该内存
加密掩码。这用于去除 SME 掩码并获取真实的物理地址。

目前，sme_mask 存储 C 位的位置。如果需要，可以将额外的 SME 相关信息放入该变量中。

```

  [ misc	        ][ enc bit  ][ other misc SME info       ]
  0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_..._0000
  63   59   55   51   47   43   39   35   31   27   ... 3

```
## x86_32


### X86_PAE


表示是否启用了物理地址扩展。它会带来更高的页表查找开销，并且每个进程也消耗更多的页表
空间。用于在将虚拟地址转换为物理地址时检查崩溃内核是否启用了 PAE。

## ARM64


### VA_BITS


虚拟地址的最大位数。用于计算虚拟内存范围。

### kimage_voffset


内核虚拟映射与物理映射之间的偏移量。用于将虚拟地址转换为物理地址。

### PHYS_OFFSET


表示内存起始位置的物理地址。与 kimage_voffset 类似，后者用于将虚拟地址转换为物理地址。

### KERNELOFFSET


内核随机化偏移量。用于计算页偏移。如果 KASLR 被禁用，则该值为零。

### KERNELPACMASK


用于从内核虚拟地址中提取指针认证码（Pointer Authentication Code）的掩码。

### TCR_EL1.T1SZ


表示 TTBR1_EL1 所寻址的内存区域的大小偏移。该区域大小为 2^(64-T1SZ) 字节。

TTBR1_EL1 是由 ARMv8-A 架构规定的表基地址寄存器，用于查找较高 VA 范围中虚拟地址的
页表（更多细节请参阅 ARMv8 ARM 文档）。

### MODULES_VADDR|MODULES_END|VMALLOC_START|VMALLOC_END|VMEMMAP_START|VMEMMAP_END


用于获取正确的范围：
	MODULES_VADDR ~ MODULES_END-1 : 内核模块空间。
	VMALLOC_START ~ VMALLOC_END-1 : vmalloc() / ioremap() 空间。
	VMEMMAP_START ~ VMEMMAP_END-1 : vmemmap 区域，用于 struct page 数组。

## arm


### ARM_LPAE


它表示崩溃内核是否支持大物理地址扩展。用于将虚拟地址转换为物理地址。

## s390


### lowcore_ptr


一个指向每个 CPU 的 lowcore 的指针数组。用于打印 psw 以及所有寄存器的信息。

### high_memory


用于从 high_memory 符号获取 vmalloc_start 地址。

### (lowcore_ptr, NR_CPUS)


CPU 的最大数量。

## powerpc



### node_data|(node_data, MAX_NUMNODES)


参见上文。

### contig_page_data


参见上文。

### vmemmap_list


vmemmap_list 维护整个 vmemmap 物理映射。用于获取 vmemmap 列表计数以及已填充的 vmemmap
区域信息。如果 vmemmap 地址转换信息存储在崩溃内核中，则用于转换 vmemmap 内核虚拟地址。

### mmu_vmemmap_psize


页的大小。用于将虚拟地址转换为物理地址。

### mmu_psize_defs


页大小定义，即 4k、64k 或 16M。

用于进行 vtop 转换。

### vmemmap_backing|(vmemmap_backing, list)|(vmemmap_backing, phys)|(vmemmap_backing, virt_addr)


vmemmap 虚拟地址空间管理没有传统的页表来跟踪哪些虚拟 struct page 由物理映射支撑。
虚拟到物理的映射以一种简单的链表格式进行跟踪。

用户空间工具在计算 vmemmap 区域计数时需要了解 list、phys 和 virt_addr 的偏移量。

### mmu_psize_def|(mmu_psize_def, shift)


struct mmu_psize_def 的大小以及 mmu_psize_def 成员的偏移量。

用于 vtop 转换。

## sh


### node_data|(node_data, MAX_NUMNODES)


参见上文。

### X2TLB


表示崩溃内核是否启用了 SH 扩展模式。

## RISCV64


### VA_BITS


虚拟地址的最大位数。用于计算虚拟内存范围。

### PAGE_OFFSET


表示直接映射 RAM 区域的虚拟内核起始地址。

### phys_ram_base


表示物理 RAM 的起始地址。

### MODULES_VADDR|MODULES_END|VMALLOC_START|VMALLOC_END|VMEMMAP_START|VMEMMAP_END|KERNEL_LINK_ADDR


用于获取正确的范围：

  - MODULES_VADDR ~ MODULES_END : 内核模块空间。
  - VMALLOC_START ~ VMALLOC_END : vmalloc() / ioremap() 空间。
  - VMEMMAP_START ~ VMEMMAP_END : vmemmap 空间，用于 struct page 数组。
  - KERNEL_LINK_ADDR : 内核链接与 BPF 的起始地址

### va_kernel_pa_offset


表示内核虚拟映射与物理映射之间的偏移量。用于将虚拟地址转换为物理地址。
