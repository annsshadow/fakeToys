
## VM_BIND 锁机

本文试图描述要使 VM_BIND 锁机制正确所需的内容，包括 userptr mmu_notifier 锁它还讨论了一些优化，以消除在最简单实现中所需的遍历所userptr 映射以及
外部/共享对象映射的开销。此外，还有一节描述了实现可恢复页错误（recoverable
pagefaults）所需VM_BIND 锁机制
## DRM GPUVM 辅助函数

对于实现 VM_BIND 的驱动，存在一组辅助函数，这组辅助函数实现了本文描述的锁机制中
的大部分（但并非全部）。特别是，它目前还缺userptr 的实现。本文无意详细描DRM GPUVM 的实现，但其内容已在 :ref:`其自身的文档 <drm_gpuvm>` 中涵盖。强烈建任何实现 VM_BIND 的驱动使DRM GPUVM 辅助函数，并在缺少通用功能时对其进行扩展
## 术语

- `gpu_vm`：带有元数据的虚GPU 地址空间的抽象。通常每个客户端（DRM 文件私有  或一个执行上下文对应一个- `gpu_vma`：gpu_vm 内一段带有相关元数据GPU 地址范围的抽象。gpu_vma 的后备存  可以是一GEM 对象，也可以是同时映射到该进CPU 地址空间的匿名或页缓  （page-cache）页- `gpu_vm_bo`：GEM 对象VM 之间关联的抽象。GEM 对象维护一gpu_vm_bos 列表  其中每个 gpu_vm_bo 又维护一gpu_vmas 列表- `userptr gpu_vma` 或简`userptr`：一gpu_vma，其后备存储是如上所述的匿名  页缓存页- `revalidating`（重新验证）：重新验证一gpu_vma 是指使后备存储的最新版本驻留，
  并确保该 gpu_vma 的页表项指向该后备存储- `dma_fence`：一个与 struct completion 类似struct dma_fence，用于跟GPU 活动  GPU 活动完成时，dma_fence 发出信号。请参阅
  [dma-buf doc </driver-api/dma-buf>](dma-buf doc </driver-api/dma-buf>) 中的
  `DMA Fences` 一节- `dma_resv`：一struct dma_resv（又reservation object），用于以多  dma_fences 的形式跟gpu_vm GEM 对象上的 GPU 活动。dma_resv 包含一  dma_fences 的数列表，以及一个在dma_resv 添加额外 dma_fences 时必须持有的
  锁。该锁的类型允许以任意顺序对多个 dma_resvs 进行无死锁的安全加锁。请参阅
  [dma-buf doc </driver-api/dma-buf>](dma-buf doc </driver-api/dma-buf>) 中的
  `Reservation Objects` 一节- `exec function`（执行函数）：一个重新验证所有受影响gpu_vmas、提交一GPU
  命令批次，并向所有受影响dma_resvs 注册代表GPU 命令活动dma_fence 的函数  为完整起见（尽管本文未涵盖），值得一提的是，exec function 也可能就是某些驱动在
  计算/长运行模式下使用的重新验worker- `local object`（本地对象）：仅映射在单VM 内的 GEM 对象。本GEM 对象共享
  gpu_vm dma_resv- `external object`（外部对象，又称 shared object）：可能被多gpu_vms 共享、且  后备存储可能与其他驱动共享的 GEM 对象
## 锁与加锁顺序


VM_BIND 的好处之一是，本地 GEM 对象共享 gpu_vm dma_resv 对象，从而也共享
dma_resv 锁。因此，即使有数量庞大的本地 GEM 对象，也只需一把锁即可exec
序列成为原子的
使用的锁与加锁顺序如下：

- `gpu_vm->lock`（可选为 rwsem）。保gpu_vm 中记gpu_vmas 的数据结构。它也可  保护 gpu_vm userptr gpu_vmas 列表。用 CPU mm 来类比的话，这相当于 mmap_lock  一rwsem 允许多个读者并发地遍历 VM 树，但这种并发带来的好处很可能因驱动而异- `userptr_seqlock`。该锁在 gpu_vm userptr 列表中的每个 userptr gpu_vma 上以
  读模式获取，并在 mmu notifier 失效（invalidation）期间以写模式获取。它并非真正  seqlock，而是`mm/mmu_notifier.c` 中被描述为“碰撞重试（Collision-retry）的
  读侧/写侧‘锁’，很像 seqcount。不过这允许多个写侧同时持有它……”。读侧临界区  ``mmu_interval_read_begin() / mmu_interval_read_retry()` 包裹，当写侧被持有时
  `mmu_interval_read_begin()`` 会休眠。写侧在内核调用 mmu interval 失效 notifier
  时由核心 mm 持有- `gpu_vm->resv` 锁。保gpu_vm 中需要重新绑定的 gpu_vmas 列表，以gpu_vm 所  本地 GEM 对象的驻留状态。此外，它通常还保gpu_vm 的已回收（evicted）和外部 GEM
  对象列表- `gpu_vm->userptr_notifier_lock`。这是一rwsem，在 exec 期间以读模式获取，在
  mmu notifier 失效期间以写模式获取。userptr notifier 锁是gpu_vm 的- `gem_object->gpuva_lock`。该锁保GEM 对象gpu_vm_bos 列表。它通常GEM 对象
  dma_resv 是同一把锁，但有些驱动以不同方式保护该列表，见下文- `gpu_vm 列表自旋锁（list spinlocks）`。在某些实现中，需要它们才能更gpu_vm   已回收和外部对象列表。对于那些实现，在处理列表时会获取这些自旋锁。然而，为了避免
  dma_resv 锁产生加锁顺序冲突，在遍历列表时需要一种特殊的方案

## gpu_vm_bos gpu_vmas 的保护与生命周期


GEM 对象gpu_vm_bos 列表，以gpu_vm_bo gpu_vmas 列表，由
`gem_object->gpuva_lock` 保护，该锁通常GEM 对象dma_resv 相同；但如果驱动需dma_fence 发信号（signalling）临界区内访问这些列表，它可以选择改用一把单独的锁，
该锁可以dma_fence 发信号临界区内被锁定。这类驱动随后需要额外注意：在遍gpu_vm_bo gpu_vma 列表的循环内部，需要获取哪些锁，以避免加锁顺序冲突
DRM GPUVM 辅助函数集会提供 lockdep 断言，表明在相关情形下此锁已被持有，并且还提一种让自身知晓实际使用了哪把锁的手段：`drm_gem_gpuva_set_lock`
每个 gpu_vm_bo 持有指向底层 GEM 对象的引用计数指针，每个 gpu_vma 持有指向
gpu_vm_bo 的引用计数指针。当遍历 GEM 对象gpu_vm_bos 列表以及 gpu_vm_bo gpu_vmas 列表时，不得释放 `gem_object->gpuva_lock`，否则，附加到某gpu_vm_bo 上的
gpu_vmas 可能会在毫无征兆的情况下消失，因为它们不是引用计数的。驱动可以实现自己的
方案来允许这样做，但这会以增加复杂性为代价，并且超出了本文的范围
DRM GPUVM 实现中，每个 gpu_vm_bo 和每gpu_vma 都持有对 gpu_vm 自身的引用计数因此，并且为了避免循环引用计数，gpu_vm gpu_vmas 的清理不得从 gpu_vm 的析构函数中
进行。驱动通常会实现一gpu_vm close 函数来进行此清理。gpu_vm close 函数会中使用VM GPU 执行、解除所gpu_vmas 的映射并释放页表内存
## 本地对象的重新验证与回收


请注意，下面给出的所有代码示例我们都使用了简化的伪代码。特别是，dma_resv 死锁避免
算法以及dma_resv fences 预留内存都被省略了
重新验证
____________
VM_BIND 下，GPU 使用 gpu_vm 执行时，所有本地对象都必须处于驻留状态，并且这些
对象需要建立指向它们的有效 gpu_vmas。因此，通常每次 GPU 命令缓冲区的提交之前都会
有一个重新验证（re-validation）区段：


   dma_resv_lock(gpu_vm->resv);

   // Validation section starts here.
   for_each_gpu_vm_bo_on_evict_list(&gpu_vm->evict_list, &gpu_vm_bo) {
           validate_gem_bo(&gpu_vm_bo->gem_bo);

           // The following list iteration needs the Gem object's
           // dma_resv to be held (it protects the gpu_vm_bo's list of
           // gpu_vmas, but since local gem objects share the gpu_vm's
           // dma_resv, it is already held at this point.
           for_each_gpu_vma_of_gpu_vm_bo(&gpu_vm_bo, &gpu_vma)
                  move_gpu_vma_to_rebind_list(&gpu_vma, &gpu_vm->rebind_list);
   }

   for_each_gpu_vma_on_rebind_list(&gpu vm->rebind_list, &gpu_vma) {
           rebind_gpu_vma(&gpu_vma);
           remove_gpu_vma_from_rebind_list(&gpu_vma);
   }
   // Validation section ends here, and job submission starts.

   add_dependencies(&gpu_job, &gpu_vm->resv);
   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);
   dma_resv_unlock(gpu_vm->resv);

之所以需要一个单独的 gpu_vm 重新绑定列表，是因为可能存在同样需要重新绑定的 userptr
gpu_vmas，而它们并未映射某个缓冲区对象
回收
________
其中一个本地对象的回收将类似于下面这样

   obj = get_object_from_lru();

   dma_resv_lock(obj->resv);
   for_each_gpu_vm_bo_of_obj(obj, &gpu_vm_bo);
           add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);

   add_dependencies(&eviction_job, &obj->resv);
   job_dma_fence = gpu_submit(&eviction_job);
   add_dma_fence(&obj->resv, job_dma_fence);

   dma_resv_unlock(&obj->resv);
   put_object(obj);

请注意，由于该对象是 gpu_vm 本地的，它将共享 gpu_vm dma_resv 锁，`obj->resv == gpu_vm->resv`。被标记为回收的 gpu_vm_bos 被放gpu_vm 的回收列表上该列表由 `gpu_vm->resv` 保护。在回收期间，所有本地对象的 dma_resv 都被锁定，并且由上述等式，保gpu_vm 回收列表gpu_vm dma_resv 也被锁定
VM_BIND 下，gpu_vmas 在回收之前无需解绑，因为驱动必须确保回收的 blit 或拷贝会等待
GPU 空闲或依赖于之前所有的 GPU 活动。此外，GPU 随后通过gpu_vma 访问已释放内存的
任何尝试，都会由一个带有重新验证区段的新的 exec function 先行，该区段会确保所gpu_vmas 都被重新绑定。回收代码在重新验证期间持有对象dma_resv，将确保新的 exec
function 不会与回收发生竞争
驱动可以这样实现：在每次 exec function 中，只选择一部分 vmas 进行重新绑定。在这种
情况下，所**被选中进行重新绑定vmas 必须exec function 工作负载提交之前
解绑
## 使用外部缓冲区对象时的加

由于外部缓冲区对象可能被多个 gpu_vms 共享，它们无法与单个 gpu_vm 共享reservation
对象。相反，它们需要拥有自己的 reservation 对象。使用一个或多个 gpu_vmas 绑定到某gpu_vm 的外部对象，因此被放到一个每 gpu_vm 的列表上，该列表gpu_vm dma_resv 或某gpu_vm 列表自旋锁保<Spinlock iteration>。一gpu_vm reservation 对象
被锁定，遍历外部对象列表并锁定所有外部对象的 dma_resvs 就是安全的。然而，如果改用列表
自旋锁，则需要使用一种更复杂的遍历方案
在回收时，外部对象所绑定*所* gpu_vms gpu_vm_bos 都需要被放到它们各自gpu_vm 的回收列表上。然而，当回收一个外部对象时，该对象所绑定gpu_vms dma_resvs
通常并未被持有。只有对象私有的 dma_resv 可以保证被持有。如果在回收时手头有一ww_acquire 上下文，我们可以获取那些 dma_resvs，但这可能导致代价高昂的 ww_mutex
回滚。一个简单的做法是：仅用 `evicted` 布尔值标记被回收gem 对象gpu_vm_bos，并下次需要遍历相应的 gpu_vm 回收列表之前检查该布尔值。例如，在遍历外部对象列表并锁定
它们时。此时，gpu_vm dma_resv 和对象的 dma_resv 都被持有，于是被标记为已回收gpu_vm_bo 就可以被添加gpu_vm 的已回收 gpu_vm_bos 列表中。该 `evicted` 布尔值在形式
上由对象dma_resv 保护
exec function 变为

   dma_resv_lock(gpu_vm->resv);

   // External object list is protected by the gpu_vm->resv lock.
   for_each_gpu_vm_bo_on_extobj_list(gpu_vm, &gpu_vm_bo) {
           dma_resv_lock(gpu_vm_bo.gem_obj->resv);
           if (gpu_vm_bo_marked_evicted(&gpu_vm_bo))
                   add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);
   }

   for_each_gpu_vm_bo_on_evict_list(&gpu_vm->evict_list, &gpu_vm_bo) {
           validate_gem_bo(&gpu_vm_bo->gem_bo);

           for_each_gpu_vma_of_gpu_vm_bo(&gpu_vm_bo, &gpu_vma)
                  move_gpu_vma_to_rebind_list(&gpu_vma, &gpu_vm->rebind_list);
   }

   for_each_gpu_vma_on_rebind_list(&gpu vm->rebind_list, &gpu_vma) {
           rebind_gpu_vma(&gpu_vma);
           remove_gpu_vma_from_rebind_list(&gpu_vma);
   }

   add_dependencies(&gpu_job, &gpu_vm->resv);
   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);
   for_each_external_obj(gpu_vm, &obj)
          add_dma_fence(job_dma_fence, &obj->resv);
   dma_resv_unlock_all_resv_locks();

与之对应的、可感知共享对象的回收看起来像这样：


   obj = get_object_from_lru();

   dma_resv_lock(obj->resv);
   for_each_gpu_vm_bo_of_obj(obj, &gpu_vm_bo)
           if (object_is_vm_local(obj))
                add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);
           else
                mark_gpu_vm_bo_evicted(&gpu_vm_bo);

   add_dependencies(&eviction_job, &obj->resv);
   job_dma_fence = gpu_submit(&eviction_job);
   add_dma_fence(&obj->resv, job_dma_fence);

   dma_resv_unlock(&obj->resv);
   put_object(obj);


## 在未持有 dma_resv 锁的情况下访gpu_vm 的列

有些驱动在访gpu_vm 的回收列表和外部对象列表时会持有 gpu_vm dma_resv 锁。然而，
也有些驱动需要在不持dma_resv 锁的情况下访问这些列表，例如由于来自 dma_fence 发信临界区内部的异步状态更新。在这种情况下，可以使用自旋锁来保护对列表的操纵。然而，由于遍历列表时需要对每个列表项获取更高级别的睡眠锁，已经遍历过的项需要被临时移动到一个私列表，并在处理每一项时释放自旋锁：


    struct list_head still_in_list;

    INIT_LIST_HEAD(&still_in_list);

    spin_lock(&gpu_vm->list_lock);
    do {
            struct list_head *entry = list_first_entry_or_null(&gpu_vm->list, head);

            if (!entry)
                    break;

            list_move_tail(&entry->head, &still_in_list);
            list_entry_get_unless_zero(entry);
            spin_unlock(&gpu_vm->list_lock);

            process(entry);

            spin_lock(&gpu_vm->list_lock);
            list_entry_put(entry);
    } while (true);

    list_splice_tail(&still_in_list, &gpu_vm->list);
    spin_unlock(&gpu_vm->list_lock);

由于额外的加锁和原子操作，那*能够**避免在该 dma_resv 锁之外访gpu_vm 列表的驱动，
可能也希望避免这种遍历方案。特别是，如果驱动预期列表项数量很大。对于那些预期列表项数量
较少、列表遍历不常发生、或者每次遍历有显著额外开销的情况，这类遍历所涉及的原子操作开销
很可能是可忽略的。请注意，如果使用此方案，必须确保该列表遍历由外层锁或信号量保护，因列表项在遍历时会被临时从列表上摘下；还值得一提的是，本地列表 `still_in_list` 也应被视`gpu_vm->list_lock` 保护，因此在列表遍历期间，项也可能从本地列表中被并发移除
请参:ref:`DRM GPUVM 加锁一<drm_gpuvm_locking>` 及其内部`get_next_vm_bo_from_list` 函数

## userptr gpu_vmas


userptr gpu_vma 是一gpu_vma，它不是将缓冲区对象映射到一GPU 虚拟地址范围，而是
直接映射一CPU mm 的匿名或文件页缓存页一种非常简单的方法是在绑定时用 pin_user_pages() 固定这些页，在解绑时取消固定，但这会
造成拒绝服务（Denial-Of-Service）隐患，因为单个用户空间进程就能够固定住系统的全部内存，
这是不可取的。（不过，对于特殊用例并且假设有恰当的记账，固定仍可能是一个可取的特性）在一般情况下，我们需要做的是：获取指向所需页的引用，确保在 CPU mm 解除映射这些页之通过 MMU notifier 得到通知，如果它们不是以只读方式映射GPU 则将其标记为脏，然后释放
该引用。当我们MMU notifier 通知 CPU mm 即将丢弃这些页时，我们需要通过在该 MMU
notifier 中等VM 空闲来停GPU 对这些页的访问，并确保在 GPU 下次尝试访问 CPU mm
范围内当前存在的任何内容之前，将旧的页从 GPU 页表中解除映射，并重复获取新页引用的过程（参见下文的 :ref:`notifier 示例 <Invalidation example>`）。请注意，当核心 mm 决定
回收（laundry）页时，我们会收到这样的解除映射 MMU 通知，并可以在下GPU 访问之前再次
将这些页标记为脏。我们还收到类似的用NUMA 记账MMU 通知，GPU 驱动其实无需关心这些但迄今为止，要将某些通知排除在外仍很困难
MMU notifier 用于设备 DMA（以及其他方法）pin_user_pages() 文档
<mmu-notifier-registration-case> 中有描述
现在，使get_user_pages() 获取 struct page 引用的方式，不幸的是无法dma_resv 锁下
使用，因为那会违dma_resv 锁与解决 CPU 页错误时获取mmap_lock 之间的加锁顺序。这
意味着 gpu_vm userptr gpu_vmas 列表需要由一把外层锁保护，在我们的下例中`gpu_vm->lock`
userptr gpu_vma MMU interval seqlock 按如下方式使用：


   // Exclusive locking mode here is strictly needed only if there are
   // invalidated userptr gpu_vmas present, to avoid concurrent userptr
   // revalidations of the same userptr gpu_vma.
   down_write(&gpu_vm->lock);
   retry:

   // Note: mmu_interval_read_begin() blocks until there is no
   // invalidation notifier running anymore.
   seq = mmu_interval_read_begin(&gpu_vma->userptr_interval);
   if (seq != gpu_vma->saved_seq) {
           obtain_new_page_pointers(&gpu_vma);
           dma_resv_lock(&gpu_vm->resv);
           add_gpu_vma_to_revalidate_list(&gpu_vma, &gpu_vm);
           dma_resv_unlock(&gpu_vm->resv);
           gpu_vma->saved_seq = seq;
   }

   // The usual revalidation goes here.

   // Final userptr sequence validation may not happen before the
   // submission dma_fence is added to the gpu_vm's resv, from the POW
   // of the MMU invalidation notifier. Hence the
   // userptr_notifier_lock that will make them appear atomic.

   add_dependencies(&gpu_job, &gpu_vm->resv);
   down_read(&gpu_vm->userptr_notifier_lock);
   if (mmu_interval_read_retry(&gpu_vma->userptr_interval, gpu_vma->saved_seq)) {
          up_read(&gpu_vm->userptr_notifier_lock);
          goto retry;
   }

   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);

   for_each_external_obj(gpu_vm, &obj)
          add_dma_fence(job_dma_fence, &obj->resv);

   dma_resv_unlock_all_resv_locks();
   up_read(&gpu_vm->userptr_notifier_lock);
   up_write(&gpu_vm->lock);

`mmu_interval_read_begin()` `mmu_interval_read_retry()` 之间的代码，标明了我们所
称的 `userptr_seqlock` 的读侧临界区。实际上，gpu_vm userptr gpu_vma 列表被遍历，
并且对它*所* userptr gpu_vmas 都进行了检查，尽管我们这里只展示了一个
userptr gpu_vma MMU 失效 notifier 可能从回收（reclaim）上下文中被调用，并且同样地为了避免加锁顺序冲突，我们不能在其中获取任何 dma_resv 锁或 gpu_vm->lock

  bool gpu_vma_userptr_invalidate(userptr_interval, cur_seq)
  {
          // Make sure the exec function either sees the new sequence
          // and backs off or we wait for the dma-fence:

          down_write(&gpu_vm->userptr_notifier_lock);
          mmu_interval_set_seq(userptr_interval, cur_seq);
          up_write(&gpu_vm->userptr_notifier_lock);

          // At this point, the exec function can't succeed in
          // submitting a new job, because cur_seq is an invalid
          // sequence number and will always cause a retry. When all
          // invalidation callbacks, the mmu notifier core will flip
          // the sequence number to a valid one. However we need to
          // stop gpu access to the old pages here.

          dma_resv_wait_timeout(&gpu_vm->resv, DMA_RESV_USAGE_BOOKKEEP,
                                false, MAX_SCHEDULE_TIMEOUT);
          return true;
  }

当此失效 notifier 返回时，GPU 不再能够访问 userptr gpu_vma 的旧页，并且需要在新的 GPU
提交成功之前重新进行页绑定
高效userptr gpu_vma exec_function 遍历
_________________________________________________
如果 gpu_vm userptr gpu_vmas 列表变得很大，在每次 exec function 中遍历完整的
userptrs 列表以检查每userptr gpu_vma 保存的序列号是否过期，效率就会很低。一种解方案是将所*已失*userptr gpu_vmas 放到一个单独的 gpu_vm 列表上，并且每次
exec function 只检查该列表上存在的 gpu_vmas。由于在mmu notifier 中（我们向列表添已失效的 gpu_vmas 的地方）无法获取任何`gpu_vm->lock` `gpu_vm->resv` 这样的外层锁该列表非常适合自旋锁迭代一<Spinlock iteration> 中描述的方案。请注意，`gpu_vm->lock`
在遍历时仍需要被持有，以确保列表的完整性，正如该节中也提到的那样
如果使用这样的已失效 userptr 列表，exec function 中的重试检查就会简单地变成检查已失效
列表是否为空
## 绑定与解绑时的加

在绑定时，假设是一个由 GEM 对象支撑gpu_vma，每gpu_vma 都需要与一gpu_vm_bo
关联，而该 gpu_vm_bo 又需要被添加GEM 对象gpu_vm_bo 列表，并可能添加gpu_vm 外部对象列表。这被称gpu_vma *链接（linking*，并且通常需要持`gpu_vm->lock`
`gem_object->gpuva_lock`。在解除一gpu_vma 的链接时，应持有相同的锁，这确保了当`gpu_vm->resv` GEM 对象dma_resv 下遍``gpu_vmas` 时，只要我们所遍历所依据的锁
未被释放，gpu_vmas 就会保持存活。对userptr gpu_vmas，类似地要求在销vma 期间持有
外层 `gpu_vm->lock`，否则当按照上一节所述遍历已失效userptr 列表时，没有任何东西让那userptr gpu_vmas 保持存活
## 可恢复页错误页表更新时的加锁


关于可恢复页错误（recoverable page-faults）的加锁，我们需要确保两件重要的事：

- 在我们将页归还给系统/分配器以供复用时，不应再有剩余的 GPU 映射，并且任GPU TLB
  都必须已被刷新- gpu_vma 的解映射与映射不得发生竞争
由于 GPU ptes 的解映射（或 zapping）通常发生在很难甚至不可能获取任何外层锁的地方，我要么引入一把在映射和解映射时都持有的新锁，要么查看我们在解映射时持有的锁，并确保它们在
映射时也被持有。对userptr gpu_vmas，在 zapping 发生mmu 失效 notifier 中，
`userptr_seqlock` 以写模式持有。因此，如果 `userptr_seqlock` 以及 `gpu_vm->userptr_notifier_lock`
在映射期间以读模式持有，它就不会zapping 发生竞争。对于由 GEM 对象支撑gpu_vmaszapping 会在 GEM 对象dma_resv 下进行，并且确保对于任何指向GEM 对象gpu_vma在填充其页表时也持有dma_resv，同样能确保我们是无竞争的
如果映射的任何部分是在这些锁被释放的情况下、在某个 dma-fence 下异步执行的，那zapping
将需要等待该 dma-fence 在相关锁下发出信号之后，才能开始修改页表
由于以释放页表内存的方式修改页表结构也可能需要外层锁，GPU ptes zapping 通常只聚焦于
将页表或页目录项清零并刷TLB，而将页表内存的释放推迟到解绑或重新绑定时进行