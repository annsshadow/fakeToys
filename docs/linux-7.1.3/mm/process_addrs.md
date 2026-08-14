
## 进程地址


用户态内存区间由内核通过虚拟内存区域（Virtual Memory Areas，简称 'VMA'）来跟踪，其类型为 :c`!struct vm_area_struct`。

每个 VMA 描述一段虚拟连续的、具有完全相同属性的内存区间，由一个个 :c`!struct vm_area_struct` 对象来描述。在 VMA 之外的用户态访问是无效的，除非相邻的栈 VMA 可以被扩展以包含被访问的地址。

所有 VMA 都包含在一个且唯一一个虚拟地址空间中，该地址空间由一个 :c`!struct mm_struct` 对象描述，所有共享该虚拟地址空间的任务（即线程）都引用它。我们称之为 :c`!mm`。

每个 mm 对象包含一个 maple tree 数据结构，用于描述虚拟地址空间内的所有 VMA。

          使用 :c`!vsyscall` 的架构，并且是一个全局静态对象，不属于任何特定的 mm。

### 锁机制


内核在设计上对 VMA **元数据（metadata）** 的并发读操作具有高度可扩展性，因此需要一套复杂的锁来确保不会发生内存损坏。

          它们所描述的，也不包括映射它们的页表。

### 术语


- **mmap 锁** - 每个 MM 都有一个读写信号量 :c`!mmap_lock`，它以进程地址空间为粒度加锁，可通过 `!mmap_read_lock`、`!mmap_write_lock` 及其变体获取。
- **VMA 锁** - VMA 锁的粒度为 VMA（理所当然），在实践中表现为一个读写信号量。VMA 读锁通过 `!lock_vma_under_rcu` 获取（并通过 `!vma_end_read` 释放），写锁通过 vma_start_write() 或 vma_start_write_killable() 获取（所有 VMA 写锁在 mmap 写锁释放时会自动释放）。要获取 VMA 写锁，你**必须**已经持有了 `!mmap_write_lock`。
- **rmap 锁** - 当尝试通过反向映射（reverse mapping）经由一个 :c`!struct address_space` 或 :c`!struct anon_vma` 对象（可由一个 folio 通过 :c`!folio->mapping` 到达）来访问 VMA 时。VMA 必须通过 `!anon_vma_[try]lock_read` 或 `!anon_vma_[try]lock_write`（用于匿名内存）以及 `!i_mmap_[try]lock_read` 或 `!i_mmap_[try]lock_write`（用于文件后备内存）来保持稳定。我们称这些锁为反向映射锁，或简称 'rmap 锁'。

我们将在下面专门的小节中单独讨论页表锁。

这些锁**任何**一个首先要实现的目标是让 VMA 在 MM 树中**稳定**下来。也就是说，保证 VMA 对象不会被在你看不见的情况下删除或修改（下面描述的一些特定字段除外）。

稳定一个 VMA 同时也保持了它所描述的地址空间的存在。

### 锁的使用


如果你想要**读取** VMA 元数据字段，或者只是想让 VMA 保持稳定，你必须做以下之一：

- 通过 `!mmap_read_lock`（或合适的变体）在 MM 粒度上获取一个 mmap 读锁，在你使用完 VMA 后通过相应的 `!mmap_read_unlock` 释放它，**或者**
- 尝试通过 `!lock_vma_under_rcu` 获取一个 VMA 读锁。它会尝试原子地获取该锁，因此可能失败，此时需要回退逻辑，在其返回 `!NULL` 的情况下改为获取一个 mmap 读锁，**或者**
- 在遍历被锁定的区间树（无论是匿名的还是文件后备的）以获取所需的 VMA 之前，先获取一个 rmap 锁。

如果你想要**写入** VMA 元数据字段，则情况因字段而异（我们将在下面详细探讨每个 VMA 字段）。对于大多数字段，你必须：

- 通过 `!mmap_write_lock`（或合适的变体）在 MM 粒度上获取一个 mmap 写锁，在你使用完 VMA 后通过相应的 `!mmap_write_unlock` 释放它，**并且**
- 通过 `!vma_start_write` 为你想要修改的每个 VMA 获取一个 VMA 写锁，它会在 `!mmap_write_unlock` 被调用时自动释放。
- 如果你想要能够写入**任何**字段，你还必须通过获取一个 **rmap 写锁** 将 VMA 从反向映射中隐藏起来。

VMA 锁的特殊之处在于，你必须**先**获取一个 mmap **写**锁，才能获取一个 VMA **写**锁。不过，VMA **读**锁可以在没有任何其他锁的情况下获取（`!lock_vma_under_rcu` 会获取再释放一个 RCU 锁来为你查找 VMA）。

这限制了写者对读者的影响，因为写者可以与一个 VMA 交互，而读者可以同时与另一个 VMA 交互。

          意味着在没有 VMA 写锁的情况下，缺页异常（page fault）将与你所做的任何操作并发运行。

考察所有有效的锁状态：

   ========= ======== ========= ======= ===== =========== ==========
   mmap lock VMA lock rmap lock Stable? Read? Write most? Write all?
   ========= ======== ========= ======= ===== =========== ==========
   \-        \-       \-        N       N     N           N
   \-        R        \-        Y       Y     N           N
   \-        \-       R/W       Y       Y     N           N
   R/W       \-/R     \-/R/W    Y       Y     N           N
   W         W        \-/R      Y       Y     Y           N
   W         W        W         Y       Y     Y           Y
   ========= ======== ========= ======= ===== =========== ==========

            尝试反向操作是无效的，因为这可能导致死锁——如果
            另一个任务已经持有 mmap 写锁并尝试获取一个 VMA
            写锁，它将在 VMA 读锁上发生死锁。

所有这些锁在实践中都表现为读写信号量，因此你可以为它们中的每一个获取读锁或写锁。

          允许多个并发读者。但写锁只有在所有读者都已离开临界区（并且待处理的读者被置为等待）时才能获取。

          这使得读写信号量上的读锁与其他读者并发，而写锁则排他于所有持有该信号量的其他方。

##### VMA 字段


我们可以根据 :c`!struct vm_area_struct` 字段的用途对其进行细分，这使得探索它们的锁特性更为容易：

          实际上是一个内部实现细节。


   ===================== ======================================== ===========
   Field                 Description                              Write lock
   ===================== ======================================== ===========
   :c`!vm_start` Inclusive start virtual address of range mmap write,
                         VMA describes.                           VMA write,
                                                                  rmap write.
   :c`!vm_end`   Exclusive end virtual address of range   mmap write,
                         VMA describes.                           VMA write,
                                                                  rmap write.
   :c`!vm_pgoff` Describes the page offset into the file, mmap write,
                         the original page offset within the      VMA write,
                         virtual address space (prior to any      rmap write.
                         `!mremap`), or PFN if a PFN map
                         and the architecture does not support
                         `!CONFIG_ARCH_HAS_PTE_SPECIAL`.
   ===================== ======================================== ===========

这些字段描述了 VMA 的大小、起始和结束地址，因此无法在不先将其从反向映射中隐藏的情况下被修改，因为这些字段用于在反向映射区间树中定位 VMA。


   ============================ ======================================== =========================
   Field                        Description                              Write lock
   ============================ ======================================== =========================
   :c`!vm_mm`           Containing mm_struct.                    None - written once on
                                                                         initial map.
   :c`!vm_page_prot`    Architecture-specific page table         mmap write, VMA write.
                                protection bits determined from VMA
                                flags.
   :c`!vm_flags`        Read-only access to VMA flags describing N/A
                                attributes of the VMA, in union with
                                private writable
                                :c`!__vm_flags`.
   :c`!__vm_flags`      Private, writable access to VMA flags    mmap write, VMA write.
                                field, updated by
                                `!vm_flags_*` functions.
   :c`!vm_file`         If the VMA is file-backed, points to a   None - written once on
                                struct file object describing the        initial map.
                                underlying file, if anonymous then
                                `!NULL`.
   :c`!vm_ops`          If the VMA is file-backed, then either   None - Written once on
                                the driver or file-system provides a     initial map by
                                :c`!struct vm_operations_struct` `!f_ops->mmap()`.
                                object describing callbacks to be
                                invoked on VMA lifetime events.
   :c`!vm_private_data` A :c`!void *` field for          Handled by driver.
                                driver-specific metadata.
   ============================ ======================================== =========================

这些是用于描述 VMA 所属的 MM 及其属性的核心字段。


   ================================= ===================== ======================================== ===============
   Field                             Configuration option  Description                              Write lock
   ================================= ===================== ======================================== ===============
   :c`!anon_name`            CONFIG_ANON_VMA_NAME  A field for storing a                    mmap write,
                                                           :c`!struct anon_vma_name`        VMA write.
                                                           object providing a name for anonymous
                                                           mappings, or `!NULL` if none
                                                           is set or the VMA is file-backed. The
							   underlying object is reference counted
							   and can be shared across multiple VMAs
							   for scalability.
   :c`!swap_readahead_info`  CONFIG_SWAP           Metadata used by the swap mechanism      mmap read,
                                                           to perform readahead. This field is      swap-specific
                                                           accessed atomically.                     lock.
   :c`!vm_policy`            CONFIG_NUMA           `!mempolicy` object which        mmap write,
                                                           describes the NUMA behaviour of the      VMA write.
                                                           VMA. The underlying object is reference
							   counted.
   :c`!numab_state`          CONFIG_NUMA_BALANCING `!vma_numab_state` object which  mmap read,
                                                           describes the current state of           numab-specific
                                                           NUMA balancing in relation to this VMA.  lock.
                                                           Updated under mmap read lock by
                                                           `!task_numa_work`.
   :c`!vm_userfaultfd_ctx`   CONFIG_USERFAULTFD    Userfaultfd context wrapper object of    mmap write,
                                                           type `!vm_userfaultfd_ctx`,      VMA write.
                                                           either of zero size if userfaultfd is
                                                           disabled, or containing a pointer
                                                           to an underlying
                                                           `!userfaultfd_ctx` object which
                                                           describes userfaultfd metadata.
   ================================= ===================== ======================================== ===============

这些字段是否存在，取决于相关的内核配置选项是否被设置。


   =================================== ========================================= ============================
   Field                               Description                               Write lock
   =================================== ========================================= ============================
   :c`!shared.rb`              A red/black tree node used, if the        mmap write, VMA write,
                                       mapping is file-backed, to place the VMA  i_mmap write.
                                       in the
                                       :c`!struct address_space->i_mmap`
                                       red/black interval tree.
   :c`!shared.rb_subtree_last` Metadata used for management of the       mmap write, VMA write,
                                       interval tree if the VMA is file-backed.  i_mmap write.
   :c`!anon_vma_chain`         List of pointers to both forked/CoW’d     mmap read, anon_vma write.
                                       `!anon_vma` objects and
                                       :c`!vma->anon_vma` if it is
                                       non-`!NULL`.
   :c`!anon_vma`               `!anon_vma` object used by        When `NULL` and
                                       anonymous folios mapped exclusively to    setting non-`NULL`:
                                       this VMA. Initially set by                mmap read, page_table_lock.
                                       `!anon_vma_prepare` serialised
                                       by the `!page_table_lock`. This  When non-`NULL` and
                                       is set as soon as any page is faulted in. setting `NULL`:
                                                                                 mmap write, VMA write,
                                                                                 anon_vma write.
   =================================== ========================================= ============================

这些字段既用于将 VMA 放置到反向映射中，也用于在匿名映射时访问相关的 :c`!struct anon_vma` 对象，以及那些被独占映射到本 VMA 的 folio 应当所在的 :c`!struct anon_vma`。

          那么它可能同时处于 `!anon_vma` 和 `!i_mmap`
          树中，因此所有这些字段可能同时被使用。

### 页表


我们不会详尽地讨论这个主题，但大体而言，页表通过一系列页表将虚拟地址映射到物理地址，其中每个页表都包含指向下一级页表物理地址的条目（以及标志位），在叶子级别则是底层物理数据页的物理地址，或者交换条目、迁移条目或其他特殊标记。这些页内的偏移量由虚拟地址本身提供。

在 Linux 中，这些被划分为五个级别——PGD、P4D、PUD、PMD 和 PTE。大页（Huge pages）可能会消除其中的一个或两个级别，但在这种情况下，我们通常仍将叶子级别称为 PTE 级别。

	  在内核很巧妙地对页表级别进行“折叠”，即把与被跳过级别相关的函数存根化（stubbing out）。这让我们在概念上可以像始终有五个级别一样操作，即使编译器在实践中可能会消除与缺失级别相关的任何代码。

页表上通常有四个关键操作：

1. **遍历（Traversing）** 页表 - 仅仅是读取页表以便遍历它们。这只要求 VMA 保持稳定，因此足以建立这种稳定性的锁即可用于遍历（也有无锁变体，连这一要求也消除了，例如 `!gup_fast`）。对于非 VMA 区域的页表遍历有一个特例，我们在下面单独考虑。
2. **安装（Installing）** 页表映射 - 无论是创建新映射还是以改变其标识的方式修改现有映射。这要求 VMA 通过 mmap 或 VMA 锁（明确不是 rmap 锁）保持稳定。
3. **清空/解除映射（Zapping/unmapping）** 页表条目 - 这是内核对仅在叶子级别清除页表映射的称呼，同时保留所有页表不变。这是内核中在文件截断、`!MADV_DONTNEED` 操作（经由 `!madvise`）等场景下执行的非常常见的操作。它由包括 `!unmap_mapping_range` 和 `!unmap_mapping_pages` 在内的若干函数执行。此操作只需要 VMA 保持稳定。
4. **释放（Freeing）** 页表 - 当内核最终从用户态进程移除页表时（通常通过 `!free_pgtables`），必须极度小心以确保安全地完成，因为该逻辑最终会释放指定范围内的所有页表，忽略现有的叶子条目（它假设调用者既已清空该范围，又阻止了其中任何进一步的缺页或修改）。

          锁，因为与清空一样，它根本上不会修改被映射对象的标识。

**遍历** 和 **清空** 范围可以持有上述术语小节中描述的任一锁来执行——即 mmap 锁、VMA 锁或任一个反向映射锁。

也就是说——只要你让相关的 VMA 保持**稳定**——你就可以放手对这些页表执行这些操作（不过在内部，执行写入的内核操作也会获取内部页表锁以进行串行化——详见页表实现细节小节）。

          改变上述关于清空的锁要求。

当**安装** 页表条目时，必须持有 mmap 或 VMA 锁以保持 VMA 稳定。我们会在下面的页表锁细节小节中探讨其原因。

**释放** 页表是一项完全内部的内存管理操作，具有特殊的要求（详见下面的页释放小节）。

            包含这些页表所映射范围的 VMA 可通过反向映射访问。

            `!free_pgtables` 函数将这些 VMA 从反向映射中移除，
            但不得允许任何其他 VMA 可访问并跨越指定范围。

### 遍历非 VMA 页表


我们上面关注的是属于 VMA 的页表的遍历。也有可能遍历不由 VMA 表示的页表。

内核页表映射本身通常由建立它们的内核的相应部分管理，前述锁定规则不适用——例如 vmalloc 有自己的一组锁，用于建立和拆除其页表。

不过，为方便起见，我们提供了 `!walk_kernel_page_table_range` 函数，它通过 `!init_mm` 这个 :c`!struct mm_struct` 元数据对象的内核实例上的 mmap 锁进行同步。

如果需要独占访问，则使用写锁，否则读锁就足够了——我们只断言至少已获取了一个读锁。

由于除 vmalloc 和内存热插拔外，内核页表并不经常被拆除——这通常就足够了，但是此功能的任何调用者都必须确保提前获取任何额外需要的锁。

我们还允许一个真正不寻常的情况，即在**用户态**范围内遍历非 VMA 页表，这由 `!walk_page_range_debug` 提供。

它只有一个使用者——通用的页表转储逻辑（实现于 `!mm/ptdump.c`）——其目的是暴露所有映射以供调试，即使它们非常不寻常（可能是架构相关的）且不由 VMA 后备。

在这种情况下我们必须格外小心，因为 `!munmap` 实现会在降级为 mmap 读锁的情况下，于 mmap 写锁下先分离 VMA，再拆除页表。

这意味着此类操作可能与之竞争，因此需要 mmap **写**锁。

### 锁顺序


由于内核中有多个锁可能会或不可能会与显式的 mm 或 VMA 锁同时获取，我们必须警惕锁反转（lock inversion），并且锁获取与释放的**顺序**变得非常重要。

            但这样做会无意中导致相互死锁。

            例如，考虑持有锁 A 并尝试获取锁 B 的线程 1，而持有锁 B 并尝试获取锁 A 的线程 2。

            现在两个线程相互死锁。然而，如果它们尝试以相同顺序获取锁，其中一个会等待另一个完成其工作，就不会发生死锁。

`!mm/rmap.c` 开头的注释详细描述了内存管理代码内部所需的锁顺序：

  inode->i_rwsem        (while writing or truncating, not reading or faulting)
    mm->mmap_lock
      mapping->invalidate_lock (in filemap_fault)
        folio_lock
          hugetlbfs_i_mmap_rwsem_key (in huge_pmd_share, see hugetlbfs below)
            vma_start_write
              mapping->i_mmap_rwsem
                anon_vma->rwsem
                  mm->page_table_lock or pte_lock
                    swap_lock (in swap_duplicate, swap_info_get)
                      mmlist_lock (in mmput, drain_mmlist and others)
                      mapping->private_lock (in block_dirty_folio)
                          i_pages lock (widely used)
                            lruvec->lru_lock (in folio_lruvec_lock_irq)
                      inode->i_lock (in set_page_dirty's __mark_inode_dirty)
                      bdi.wb->list_lock (in set_page_dirty's __mark_inode_dirty)
                        sb_lock (within inode_lock in fs/fs-writeback.c)
                        i_pages lock (widely used, in set_page_dirty,
                                  in arch-dependent flush_dcache_mmap_lock,
                                  within bdi.wb->list_lock in __sync_single_inode)

在 `!mm/filemap.c` 顶部还有一个文件系统特定的锁顺序注释：

  ->i_mmap_rwsem                        (truncate_pagecache)
    ->private_lock                      (__free_pte->block_dirty_folio)
      ->swap_lock                       (exclusive_swap_page, others)
        ->i_pages lock

  ->i_rwsem
    ->invalidate_lock                   (acquired by fs in truncate path)
      ->i_mmap_rwsem                    (truncate->unmap_mapping_range)

  ->mmap_lock
    ->i_mmap_rwsem
      ->page_table_lock or pte_lock     (various, mainly in memory.c)
        ->i_pages lock                  (arch-dependent flush_dcache_mmap_lock)

  ->mmap_lock
    ->invalidate_lock                   (filemap_fault)
      ->lock_page                       (filemap_fault, access_process_vm)

  ->i_rwsem                             (generic_perform_write)
    ->mmap_lock                         (fault_in_readable->do_page_fault)

  bdi->wb.list_lock
    sb_lock                             (fs/fs-writeback.c)
    ->i_pages lock                      (__sync_single_inode)

  ->i_mmap_rwsem
    ->anon_vma.lock                     (vma_merge)

  ->anon_vma.lock
    ->page_table_lock or pte_lock       (anon_vma_prepare and various)

  ->page_table_lock or pte_lock
    ->swap_lock                         (try_to_unmap_one)
    ->private_lock                      (try_to_unmap_one)
    ->i_pages lock                      (try_to_unmap_one)
    ->lruvec->lru_lock                  (follow_page_mask->mark_page_accessed)
    ->lruvec->lru_lock                  (check_pte_range->folio_isolate_lru)
    ->private_lock                      (folio_remove_rmap_pte->set_page_dirty)
    ->i_pages lock                      (folio_remove_rmap_pte->set_page_dirty)
    bdi.wb->list_lock                   (folio_remove_rmap_pte->set_page_dirty)
    ->inode->i_lock                     (folio_remove_rmap_pte->set_page_dirty)
    bdi.wb->list_lock                   (zap_pte_range->set_page_dirty)
    ->inode->i_lock                     (zap_pte_range->set_page_dirty)
    ->private_lock                      (zap_pte_range->block_dirty_folio)

请检查这些注释的当前状态，它们自本文档撰写之时起可能已发生变化。

### 锁实现细节


             其他级别页表的锁规则。

### 页表锁细节


          被一个 VMA 所包含。有关我们如何处理该情况的细节，请参见上面关于非 VMA 页表遍历的小节。

除了上述术语小节中描述的锁之外，我们还有专用于页表的额外锁：

- **更高级别的页表锁** - 更高级别的页表，即 PGD、P4D 和 PUD，在修改时各自使用以进程地址空间为粒度的 :c`!mm->page_table_lock` 锁。
- **细粒度页表锁** - PMD 和 PTE 各自拥有细粒度锁，这些锁要么保存在描述页表的 folio 中，要么在设置 `!ALLOC_SPLIT_PTLOCKS` 时单独分配并由 folio 指向。PMD 自旋锁通过 `!pmd_lock` 获取，而 PTE 被映射到高端内存（如果是 32 位系统）并通过 `!pte_offset_map_lock` 小心地加锁。

这些锁代表了与每个页表级别交互所需的最低要求，但还有进一步的要求。

重要的是，注意在页表**遍历**时，有时不会获取此类锁。然而，在 PTE 级别，至少必须防止并发的页表删除（使用 RCU），并且页表必须被映射到高端内存，见下文。

是否小心地读取页表条目取决于架构，详见下面的原子性小节。

##### 锁规则


我们在与页表交互时建立基本的锁规则：

- 当修改一个页表条目时，**必须**持有该页表的页表锁，除非你能安全地假设没有人可以并发访问这些页表（例如在调用 `!free_pgtables` 时）。
- 对页表条目的读取和写入必须是**恰当**原子的。详见下面的原子性小节。
- 填充先前为空的条目要求持有 mmap 或 VMA 锁（读或写），仅使用 rmap 锁来这样做是危险的（见下面的警告）。
- 如前所述，清空可以在仅仅保持 VMA 稳定的情况下执行，即持有 mmap、VMA 或 rmap 锁中的任意一个。

             `!vms_clear_ptes` 在清空（经由 `!unmap_vmas`）和释放页表（经由 `!free_pgtables`）之间有一个时间窗口，此时 VMA 在 rmap 树中仍然可见。`!free_pgtables` 假设清空已经执行，并无条件地移除 PTE（连同释放范围内所有其他页表），因此在此时安装新的 PTE 条目可能泄漏内存，并导致其他意外且危险的行为。

移动页表时还有额外的适用规则，我们在下面关于该主题的小节中讨论。

PTE 级别的页表不同于其他级别的页表，访问它们有额外要求：

- 在 32 位架构上，它们可能位于高端内存（意味着需要被映射到内核内存才能访问）。
- 当为空时，它们可以在持有 mmap 锁或 rmap 锁进行读取、并与 PTE 和 PMD 页表锁结合的情况下被解除链接并 RCU 释放。特别是，这在处理 `!MADV_COLLAPSE` 时的 `!retract_page_tables` 中发生。因此访问 PTE 级别页表至少要求持有一个 RCU 读锁；但这只适用于能够容忍与并发页表更新竞争的读者，即观察到一个空的 PTE（在实际上已被分离并标记为 RCU 释放的页表中），而另一个新的页表已安装在相同位置并填入了条目。写者通常需要获取 PTE 锁，并重新验证 PMD 条目仍然指向同一个 PTE 级别页表。如果写者不关心是否为同一个 PTE 级别页表，它可以获取 PMD 锁并重新验证 pmd 条目的内容仍然满足要求。特别是，这在处理 `!MADV_COLLAPSE` 时的 `!retract_page_tables` 中也会发生。

要访问 PTE 级别页表，可以根据稳定性要求使用 `!pte_offset_map_lock` 或 `!pte_offset_map` 之类的辅助函数。这些函数会在需要时把页表映射到内核内存，获取 RCU 锁，并根据变体可能还会查找或获取 PTE 锁。参见 `!pte_offset_map_lock` 上的注释。

##### 原子性


无论页表锁如何，MMU 硬件都会并发地更新访问位和脏位（可能更多，取决于架构）。此外，并行的页表遍历操作（尽管保持了 VMA 稳定）以及像 GUP-fast 这样的功能会无锁地遍历（即读取）页表，甚至完全不保持 VMA 稳定。

当执行页表遍历并保持 VMA 稳定时，读取是否必须只进行一次且仅一次，取决于架构（例如 x86-64 不需要任何特殊预防措施）。

如果正在执行写入，或者一次读取决定了是否发生写入（例如在安装页表条目时，例如 `!__pud_install`），则必须始终特别小心。在这些情况下，我们永远不能假设页表锁给了我们完全独占的访问，并且必须只获取一次页表条目。

如果我们正在读取页表条目，那么我们只需确保编译器不会重排我们的加载。这通过 `!pXXp_get` 函数实现——`!pgdp_get`、`!p4dp_get`、`!pudp_get`、`!pmdp_get` 和 `!ptep_get`。

它们中的每一个都使用 `!READ_ONCE` 来保证编译器只读取页表条目一次。

然而，如果我们想要操作一个现有的页表条目并关心先前存储的数据，我们必须更进一步，使用硬件原子操作，例如在 `!ptep_get_and_clear` 中。

同样，不依赖于持有稳定 VMA 的操作，例如 GUP-fast（参见 `!gup_fast` 及其各种页表级别处理程序如 `!gup_fast_pte_range`），必须非常小心地与页表条目交互，使用 `!ptep_get_lockless` 以及更高级别页表对应的等价函数。

对页表条目的写入也必须是恰当原子的，这由 `!set_pXX` 函数确立——`!set_pgd`、`!set_p4d`、`!set_pud`、`!set_pmd` 和 `!set_pte`。

同样，清除页表条目的函数也必须是恰当原子的，如 `!pXX_clear` 函数——`!pgd_clear`、`!p4d_clear`、`!pud_clear`、`!pmd_clear` 和 `!pte_clear`。

##### 页表安装


页表安装是通过 mmap 或 VMA 锁以读或写模式显式保持 VMA 稳定来执行的（有关原因的细节，请参见锁规则小节中的警告）。

当分配一个 P4D、PUD 或 PMD 并在上述 PGD、P4D 或 PUD 中设置相关条目时，必须持有 :c`!mm->page_table_lock`。这分别在 `!__p4d_alloc`、`!__pud_alloc` 和 `!__pmd_alloc` 中获取。

   `!pud_lockptr` 则亦然，不过在撰写本文时它最终引用的是 :c`!mm->page_table_lock`。

分配一个 PTE 将使用 :c`!mm->page_table_lock`，或者，如果定义了 `!USE_SPLIT_PMD_PTLOCKS`，则使用嵌入在 PMD 物理页元数据中的一个 :c`!struct ptdesc` 形式的锁，由从 `!pmd_lock` 调用的 `!pmd_ptdesc` 获取，并最终由 `!__pte_alloc` 获取。

最后，修改 PTE 的内容需要特殊处理，因为 PTE 页表锁必须在我们想要对 PTE 中包含的条目进行稳定且独占的访问时获取，尤其是当我们想要修改它们时。

这通过 `!pte_offset_map_lock` 执行，它会小心地检查以确保 PTE 没有在我们对之下发生变化，最终调用 `!pte_lockptr` 来获取包含在关联于物理 PTE 页的 :c`!struct ptdesc` 中的、以 PTE 为粒度的自旋锁。该锁必须通过 `!pte_unmap_unlock` 释放。

   `!pte_offset_map_rw_nolock`，当我们知道我们保持了 PTE 稳定时——但为了简洁我们不探讨它。有关更多细节，请参见 `!pte_offset_map_lock` 的注释。

当修改范围内的数据时，我们通常只希望按需分配更高级别的页表，使用这些锁来避免竞争或覆盖任何内容，并按需在 PTE 级别设置/清除数据（例如在缺页或清空时）。

遍历页表条目以安装新映射时采取的典型模式是，乐观地确定上一级页表中的页表条目是否为空，如果是，则仅此时获取页表锁并再次检查它是否在我们之下被分配。

这使得页表锁只在实际需要时才被获取。这方面的一个例子是 `!__pud_alloc`。

在叶子页表，即 PTE，我们不能完全依赖这种模式，因为我们有独立的 PMD 和 PTE 锁，而 THP 折叠（collapse）例如可能已经从我们之下消除了 PMD 条目以及 PTE。

这就是为什么 `!pte_offset_map_lock` 无锁地检索 PTE 的 PMD 条目，小心地检查它是否符合预期，然后再获取 PTE 特定的锁，然后**再次**检查 PMD 条目是否符合预期。

如果发生 THP 折叠（或类似情况），则两个页上的锁都将被获取，因此我们可以在持有 PTE 锁的同时防止这种情况发生。

以这种方式安装条目确保了写入的互斥。

##### 页表释放


拆除页表本身是一件需要极大小心的事情。绝不能有办法让被指定移除的页表被并发任务遍历或引用。

仅持有 mmap 写锁和 VMA 锁（这将阻止竞争的缺页和 rmap 操作）是不够的，因为文件后备映射可以在仅 :c`!struct address_space->i_mmap_rwsem` 之下被截断。

因此，任何可通过反向映射访问的 VMA（无论是通过 :c`!struct anon_vma->rb_root` 还是 :c:member:`!struct address_space->i_mmap` 区间树）都不能拆除其页表。

该操作通常通过 `!free_pgtables` 执行，它假设要么已经获取了 mmap 写锁（由其 :c`!mm_wr_locked` 参数指定），要么 VMA 已经不可达。

它小心地从所有反向映射中移除 VMA，但是重要的是，不能有任何新的反向映射与本 VMA 重叠，也不能保留任何允许访问正在拆除页表的范围内的地址的途径。

此外，它假设已经执行了一次清空，并且已经采取了措施确保在清空与调用 `!free_pgtables` 之间不能再安装任何页表条目。

由于假设所有此类步骤都已执行，页表条目在没有页表锁的情况下被清除（在 `!pgd_clear`、`!p4d_clear`、`!pud_clear` 和 `!pmd_clear` 函数中）。

          上面的页表，正如 `!retract_page_tables` 所做的那样，它在 i_mmap 读锁、PMD 和 PTE 页表锁下执行，没有这种级别的谨慎。

##### 页表移动


一些函数操作 PMD 以上的页表级别（即 PUD、P4D 和 PGD 页表）。其中最值得注意的是 `!mremap`，它能够移动更高级别的页表。

在这些情况下，要求获取**所有**锁，即 mmap 锁、VMA 锁和相关的 rmap 锁。

你可以在 `!mremap` 实现中的 `!take_rmap_locks` 和 `!drop_rmap_locks` 函数中观察到这一点，它们执行锁获取的 rmap 一侧，最终由 `!move_page_tables` 调用。

### VMA 锁内部机制


##### 概述


VMA 读锁完全是乐观的——如果锁存在竞争，或者一个竞争的写入已经开始，那么我们不会获取读锁。

VMA **读**锁通过 `!lock_vma_under_rcu` 获取，它首先调用 `!rcu_read_lock` 以确保在 RCU 临界区中查找 VMA，然后尝试通过 `!vma_start_read` 对 VMA 加锁，最后通过 `!rcu_read_unlock` 释放 RCU 锁。

在用户已经持有 mmap 读锁的情况下，可以使用 `!vma_start_read_locked` 和 `!vma_start_read_locked_nested`。这些函数不会因锁竞争而失败，但调用者仍应检查它们的返回值，以防它们因其他原因失败。

VMA 读锁在其持续期间递增 :c`!vma.vm_refcnt` 引用计数器，而 `!lock_vma_under_rcu` 的调用者必须通过 `!vma_end_read` 将其递减。

VMA **写**锁通过 `!vma_start_write` 在 VMA 即将被修改的实例中获取，与 `!vma_start_read` 不同，该锁总是被获取。mmap 写锁的持续时间必须覆盖 VMA 写锁，释放或降级 mmap 写锁也会释放 VMA 写锁，因此没有 `!vma_end_write` 函数。

注意，当对 VMA 锁进行
写锁时，:c`!vma.vm_refcnt` 被临时修改，以便读者能够检测到写者的存在。一旦用于串行化的 vma 序列号被更新，该引用计数器就会被恢复。

这确保了我们所需的语义——VMA 写锁提供对 VMA 的独占写访问。

##### 实现细节


VMA 锁机制旨在成为一种轻量级的手段，以避免使用竞争激烈的 mmap 锁。它通过组合使用属于包含的 :c`!struct mm_struct` 和 VMA 的引用计数器和序列号来实现。

读锁通过 `!vma_start_read` 获取，这是一个乐观操作，即它尝试获取读锁，但如果无法获取则返回 false。在读操作结束时，调用 `!vma_end_read` 来释放 VMA 读锁。

调用 `!vma_start_read` 要求先调用 `!rcu_read_lock`，建立在获取 VMA 读锁时我们处于 RCU 临界区之中。一旦获取，RCU 锁就可以被释放，因为它仅用于查找。这由 `!lock_vma_under_rcu` 抽象，它是用户应当使用的接口。

写入要求 mmap 被写锁锁定，并且 VMA 锁通过 `!vma_start_write` 获取，但写锁由 mmap 写锁的终止或降级来释放，因此不需要 `!vma_end_write`。

所有这一切都是通过每-mm 和每-VMA 的序列计数实现的，用于降低复杂性，尤其是对于那些一次性写锁多个 VMA 的操作。

如果 mm 序列计数 :c`!mm->mm_lock_seq` 等于 VMA 序列计数 :c`!vma->vm_lock_seq`，则 VMA 被写锁锁定。如果它们不同，则不是。

每次在 `!mmap_write_unlock` 或 `!mmap_write_downgrade` 中释放 mmap 写锁时，都会调用 `!vma_end_write_all`，它还会通过 `!mm_lock_seqcount_end` 递增 :c`!mm->mm_lock_seq`。

这样，我们确保无论 VMA 的序列号如何，都不会错误地指示写锁，并且当我们释放 mmap 写锁时，我们高效地同时释放了 mmap 内包含的**所有** VMA 写锁。

由于 mmap 写锁排他于持有它的其他方，在其释放时自动释放任何 VMA 锁是有意义的，因为你绝不会想要在完全独立的写操作之间保持 VMA 被锁。

它还有助于维持正确的锁顺序。

每次获取一个 VMA 读锁，我们递增 :c`!vma.vm_refcnt` 引用计数器，并检查 VMA 的序列计数是否与 mm 的不匹配。

如果不匹配，读锁失败并递减 :c`!vma.vm_refcnt`。如果匹配，我们保持引用计数器升高，排除写者，但允许其他读者，它们也可以在 RCU 下获取此锁。

重要的是，`!lock_vma_under_rcu` 中执行的 maple tree 操作也是 RCU 安全的，因此整个读锁操作保证正确运行。

在写的一侧，我们在 :c`!vma.vm_refcnt` 中设置一个读者无法修改的位，并等待所有读者递减它们的引用计数。一旦没有读者，VMA 的序列号被设置为与 mm 的相匹配。在整个操作期间持有 mmap 写锁。

这样，如果有任何读锁生效，`!vma_start_write` 将休眠直到它们完成，从而实现互斥。

在设置 VMA 的序列号之后，指示写者的 :c`!vma.vm_refcnt` 中的位被清除。从此时起，VMA 的序列号将指示 VMA 的写锁状态，直到 mmap 写锁被丢弃或降级。

这种引用计数器和序列号的巧妙组合，使得基于 RCU 的快速每-VMA 锁获取（尤其在缺页时，尽管也在其他地方使用）能够以最小的锁顺序复杂度实现。

### mmap 写锁降级


当持有一个 mmap 写锁时，你拥有对 mmap 内资源的独占访问（通常需要注意要求 VMA 写锁以避免与持有 VMA 读锁的任务竞争）。

然后可以通过 `!mmap_write_downgrade` 将写锁**降级**为读锁，它与 `!mmap_write_unlock` 类似，通过 `!vma_end_write_all` 隐式终止所有 VMA 写锁，但重要的是在降级时并不放弃 mmap 锁，因此保持被锁定的虚拟地址空间稳定。

由此产生一个有趣的结果：降级后的锁排他于任何其他持有降级锁的任务（因为一个竞争的任务必须先获取写锁才能将其降级，而降级锁会阻止在原始锁被释放之前获取新的写锁）。

为清晰起见，我们将读（R）/降级写（D）/写（W）锁相互对照，显示哪些锁排斥其他锁：

   :widths: 5 5 5 5
   :header-rows: 1
   :stub-columns: 1

   - -
     - R
     - D
     - W
   - - R
     - N
     - N
     - Y
   - - D
     - N
     - Y
     - Y
   - - W
     - Y
     - Y
     - Y

这里 Y 表示匹配行/列的锁是互斥的，N 表示它们不是。

### 栈扩展


栈扩展带来了额外的复杂性，因为我们不允许存在竞争的缺页，因此我们在 `!expand_downwards` 或 `!expand_upwards` 中调用 `!vma_start_write` 来防止这种情况。

### 函数与结构体
