## 何时应在内页表锁内进行通知？


在清除一个 pte/pmd 时，我们可以选择在内页表锁下通过该事件的通知（\*_clear_flush 调用的 notify 版本 mmu_notifier_invalidate_range）来通知该事件。但并非在所有情况下都需要该通知。

对于次级 TLB（非 CPU TLB），例如 IOMMU TLB 或设备 TLB（当设备使用类似 ATS/PASID 的机制让 IOMMU 遍历 CPU 页表以访问进程虚拟地址空间时），在清除 pte/pmd 时，只有两种情况需要在持有页表锁的同时通知这些次级 TLB：

  A) 在 mmu_notifier_invalidate_range_end() 之前释放页的备份地址
  B) 页表项被更新为指向一个新页（COW、对零页的写缺页、__replace_page() 等）

情况 A 很明显，你不会想冒设备写入某个现在可能已被完全不同任务使用的页的风险。

情况 B 则更为微妙。为保证正确性，需要发生以下序列：

  - 获取页表锁
  - 清除页表项并进行通知（[pmd/pte]p_huge_clear_flush_notify()）
  - 将页表项设置为指向新页

如果清除页表项之后、设置新的 pte/pmd 值之前没有紧随一次通知，那么你就可能破坏设备侧的内存模型（如 C11 或 C++11）。

考虑以下场景（设备使用了类似 ATS/PASID 的特性）：

两个地址 addrA 和 addrB，满足 \|addrA - addrB\| >= PAGE_SIZE，我们假设它们因 COW 而被写保护（情况 B 的其他情形同样适用）。

```

 [Time N] --------------------------------------------------------------------
 CPU-thread-0  {try to write to addrA}
 CPU-thread-1  {try to write to addrB}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {read addrA and populate device TLB}
 DEV-thread-2  {read addrB and populate device TLB}
 [Time N+1] ------------------------------------------------------------------
 CPU-thread-0  {COW_step0: {mmu_notifier_invalidate_range_start(addrA)}}
 CPU-thread-1  {COW_step0: {mmu_notifier_invalidate_range_start(addrB)}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+2] ------------------------------------------------------------------
 CPU-thread-0  {COW_step1: {update page table to point to new page for addrA}}
 CPU-thread-1  {COW_step1: {update page table to point to new page for addrB}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+3] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {preempted}
 CPU-thread-2  {write to addrA which is a write to new page}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+3] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {preempted}
 CPU-thread-2  {}
 CPU-thread-3  {write to addrB which is a write to new page}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+4] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {COW_step3: {mmu_notifier_invalidate_range_end(addrB)}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+5] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {read addrA from old page}
 DEV-thread-2  {read addrB from new page}

```
所以这里因为在时刻 N+2，清除页表项的操作没有与一次用于使次级 TLB 失效的通知配对，设备会在看到 addrA 的新值之前先看到 addrB 的新值。这破坏了设备侧的总体内存顺序。

当将一个 pte 改为写保护，或指向一个具有相同内容（KSM）的新写保护页时，将 mmu_notifier_invalidate_range 调用延迟到页表锁之外的 mmu_notifier_invalidate_range_end() 是可以的。即使在执行页表更新的线程于释放页表锁之后、调用 mmu_notifier_invalidate_range_end() 之前被抢占，也是如此。
