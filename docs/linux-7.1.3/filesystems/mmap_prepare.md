
## mmap_prepare 回调使用指南


## 简介


`struct file->f_op->mmap()` 回调已被废弃，因为它既存在稳定性问题，也存在安全风险，并且不总是允许相邻映射的合并，从而导致不必要的内存碎片。

它已被 `file->f_op->mmap_prepare()` 回调取代，该回调解决了这些问题。

此钩子在函数映射建立的起始阶段被调用，重要的是它在任何相邻映射合并发生**之前**被调用。

若在映射时产生错误，错误可能在此回调被调用之后才出现，因此应将其视为实质上无状态的。

也就是说——不应分配任何资源，也不应更新任何状态来反映映射已经建立，因为映射可能在回调完成后被合并，或者映射失败。

### 已映射回调


如果需要为每个映射分配资源，或者需要操作诸如引用计数之类的状态，应当通过 `vm_ops->mapped` 钩子来完成，该钩子本身应由 mmap_prepare 钩子设置。

仅当一个新的映射建立且未与其他映射合并时，才会调用此回调；并且它会在映射建立之前不可能发生错误的时刻被调用。

你可以向该回调本身返回错误，这将导致映射被取消映射，并向 mmap() 调用者返回错误。这在需要分配资源、而分配可能失败的情况下很有用。

## 如何使用


在你的驱动的 struct file_operations 结构体中，指定一个 `mmap_prepare` 回调，而不是 `mmap` 回调，例如对于 ext4：


    const struct file_operations ext4_file_operations = {
        ...
        .mmap_prepare    = ext4_file_mmap_prepare,
    };

其签名为 `int (**mmap_prepare)(struct vm_area_desc **)`。

观察 struct vm_area_desc 类型：


    struct vm_area_desc {
        /** Immutable state. **/
        const struct mm_struct *const mm;
        struct file **const file; /** May vary from vm_file in stacked callers. */
        unsigned long start;
        unsigned long end;

        /** Mutable fields. Populated with initial state. **/
        pgoff_t pgoff;
        struct file *vm_file;
        vma_flags_t vma_flags;
        pgprot_t page_prot;

        /** Write-only fields. **/
        const struct vm_operations_struct *vm_ops;
        void *private_data;

        /** Take further action? **/
        struct mmap_action action;
    };

这很直接——你拥有设置映射所需的所有字段，并且可以更新可变的与可写的字段，例如：


    static int ext4_file_mmap_prepare(struct vm_area_desc *desc)
    {
        int ret;
        struct file *file = desc->file;
        struct inode *inode = file->f_mapping->host;

        ...

        file_accessed(file);
        if (IS_DAX(file_inode(file))) {
            desc->vm_ops = &ext4_dax_vm_ops;
            vma_desc_set_flags(desc, VMA_HUGEPAGE_BIT);
        } else {
            desc->vm_ops = &ext4_file_vm_ops;
        }
        return 0;
    }

重要的是，更新这些字段时你不再需要在引用计数或锁上绕来绕去——**你可以直接去修改它们**。

一切都由映射代码负责处理。

### VMA 标志


随着 `mmap_prepare`，VMA 标志也经历了一次大修。以前你会调用 vm_flags_init()、vm_flags_reset()、vm_flags_set()、vm_flags_clear() 和 vm_flags_mod() 中的一个来修改标志（并让锁被正确执行），现在这已不再必要。

此外，通过 `VM_READ`、`VM_WRITE` 等指定 VMA 标志的传统方式——即使用 `-VM_xxx` 宏——也发生了变化。

在实现 mmap_prepare() 时，通过位号来引用标志，定义为 `VMA_xxx_BIT` 宏，例如 `VMA_READ_BIT`、`VMA_WRITE_BIT` 等，并使用下列函数之一（其中 `desc` 是指向 struct vm_area_desc 的指针）：

- `vma_desc_test_any(desc, ...)` - 指定一个以逗号分隔的标志列表来测试（任意标志是否被设置），例如——``vma_desc_test_any(desc, VMA_WRITE_BIT, VMA_MAYWRITE_BIT)`` 如果任一标志被设置则返回 `true`，否则返回 `false`。
- `vma_desc_set_flags(desc, ...)` - 更新 VMA 描述符标志以设置由逗号分隔的列表所指定的附加标志，例如——`vma_desc_set_flags(desc, VMA_PFNMAP_BIT, VMA_IO_BIT)`。
- `vma_desc_clear_flags(desc, ...)` - 更新 VMA 描述符标志以清除由逗号分隔的列表所指定的标志，例如——``vma_desc_clear_flags(desc, VMA_WRITE_BIT, VMA_MAYWRITE_BIT)``。

## 操作


现在你可以非常容易地通过对 struct vm_area_desc 指针调用简单的辅助函数，在映射建立后对其执行操作。这些辅助函数包括：

- mmap_action_remap() - 对由特定大小的一组 PFN 组成、起始于某个虚拟地址和 PFN 编号的范围进行重映射。

- mmap_action_remap_full() - 与 mmap_action_remap() 相同，只是从 `start_pfn` 开始重映射整个映射。

- mmap_action_ioremap() - 与 mmap_action_remap() 相同，只是执行一次 I/O 重映射。

- mmap_action_ioremap_full() - 与 mmap_action_ioremap() 相同，只是从 `start_pfn` 开始重映射整个映射。

- mmap_action_simple_ioremap() - 从指定的物理地址开始、覆盖指定长度，建立一个 I/O 重映射。

- mmap_action_map_kernel_pages() - 在 VMA 中从特定偏移处映射一组指定的 `struct page` 指针。

- mmap_action_map_kernel_pages_full() - 在整段 VMA 上映射一组指定的 `struct page` 指针。调用者必须确保页数组中有足够的条目来覆盖所描述的 VMA 的整个范围。

**注意：** `action` 字段通常绝不应被直接操作，而应当使用这些辅助函数之一。
