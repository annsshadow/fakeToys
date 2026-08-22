
## KVM 锁概

### 1. 加锁顺序（Acquisition Orders

互斥体（mutex）的加锁顺序如下
- cpus_read_lock() kvm_lock 之外获取

- kvm_usage_lock cpus_read_lock() 之外获取

- kvm->lock vcpu->mutex 之外获取

- kvm->lock kvm->slots_lock kvm->irq_lock 之外获取

- vcpu->mutex kvm->slots_lock kvm->slots_arch_lock 之外获取

- kvm->slots_lock kvm->irq_lock 之外获取，尽管同时获取它们的情况相当罕见
- kvm->mn_active_invalidate_count 确保成对invalidate_range_start()   invalidate_range_end() 回调使用同一memslots 数组。在修改 memslots 时，等待侧会获取
  kvm->slots_lock kvm->slots_arch_lock，因MMU 通知器（notifier）绝不能获取
  kvm->slots_lock 鎴?kvm->slots_arch_lock銆。
cpus_read_lock() kvm_lock 的关系：

- 尽管官方顺序规定kvm_lock 之外获取 cpus_read_lock()，但这样做是有问题的，因为很容易  持有 kvm_lock 时不知不觉地触发 cpus_read_lock()。遍vm_list 时要谨慎，例如尽可能避免
  复杂操作
对于 SRCU
- `synchronize_srcu(&kvm->srcu)` kvm->lock、vcpu->mutex kvm->slots_lock 的临界区内部
  调用。这些锁**不能**kvm->srcu 读侧临界区内部获取；也就是说```

      srcu_read_lock(&kvm->srcu);
      mutex_lock(&kvm->slots_lock);

```

- kvm->slots_arch_lock 反而在调用 `synchronize_srcu()` 之前被释放。因此它**可以**  kvm->srcu 读侧临界区内部获取，例如在处vmexit 时
x86 上：

- vcpu->mutex kvm->arch.hyperv.hv_lock kvm->arch.xen.xen_lock 之外获取

- kvm->arch.mmu_lock 是一rwlock；kvm->arch.tdp_mmu_pages_lock   kvm->arch.mmu_unsync_pages_lock 的临界区也必须获kvm->arch.mmu_lock

其他一切都是叶锁（leaf）：临界区内不获取其他锁
### 2. 例外（Exception

快速页错误（Fast page fault）：

快速页错误是在 x86 上于 mmu-lock 之外修复客户机页错误的快速路径。目前，在以下两种情况下
页错误可以是快速的
1. 访问追踪（Access Tracking）：SPTE 不存在，但被标记为访问追踪。这意味着我们需要恢复保存的
   R/X 位。这在下文中会更详细地描述
2. 写保护（Write-Protection）：SPTE 存在且错误由写保护引起。这意味着我们只需改变 spte    W 位
我们用来避免所有竞争的spte 上的 Host-writable 位和 MMU-writable 位：

- Host-writable 表示 gfn 在主机内核页表及KVM memslot 中可写- MMU-writable 表示 gfn 在客户机 mmu 中可写，且未被影子页写保护
在快速页错误路径上，如果 spte.HOST_WRITEABLE = 1 spte.WRITE_PROTECT = 1，我们将使用
cmpxchg 原子地设spte W 位，以恢复保存的 R/X 位（对于访问追踪spte），或两者都设置这是安全的，因为对这些位的任何改变都能被 cmpxchg 检测到
但我们需要仔细检查以下情况：

1) gfn pfn 的映
gfn pfn 的映射可能会改变，因为我们只能确保在 cmpxchg 期间 pfn 不被改变。这是一ABA 问题，例如下面的情况会发生：

+------------------------------------------------------------------------+
**| 开始时**
: |
|                                                                        |
|	gpte = gfn1                                                      |
|	gfn1 在主机上映射pfn1                                         |
|	spte 是与 gpte 对应的影子页表项，且                            |
|	spte = pfn1                                                      |
+------------------------------------------------------------------------+
| 在快速页错误路径上：                                                   |
+------------------------------------+-----------------------------------+
| CPU 0:                             | CPU 1:                            |
+------------------------------------+-----------------------------------+
**| **
: |                                   |
|                                    |                                   |
|   old_spte = *spte;                |                                   |
+------------------------------------+-----------------------------------+
**|                                    | pfn1 被换*
: |
|                                    |                                   |
|                                    |    spte = 0;                      |
|                                    |                                   |
|                                    | pfn1 被重新分配给 gfn2         |
|                                    |                                   |
|                                    | gpte 被客户机改为指向             |
**|                                    | gfn2**
: |
|                                    |                                   |
|                                    |    spte = pfn1;                   |
+------------------------------------+-----------------------------------+
**| **
: |
|                                                                        |
|   if (cmpxchg(spte, old_spte, old_spte+W)                              |
|	mark_page_dirty(vcpu->kvm, gfn1)                                 |
|            OOPS!!!                                                     |
+------------------------------------------------------------------------+

我们gfn1 做了脏日志（dirty-log），这意味着 gfn2 在脏位图（dirty-bitmap）中丢失了
对于直接 sp（direct sp），我们可以轻易避免它，因为直接 sp spte 固定绑定gfn。对于间sp（indirect sp），为了简单起见我们禁用了快速页错误
针对间接 sp 的一个解决办法是cmpxchg 之前固定（pin）gfn。固定之后：

- 我们持有pfn 的引用计数（refcount）；这意味着 pfn 不能被释放并被另一gfn 重用- pfn 是可写的，因此它不能KSM 在不gfn 之间共享
这样，我们就可以确保gfn 正确设置了脏位图
2) 脏位（Dirty bit）追
在原始代码中，如spte 是只读的Accessed 位已被设置，spte 可以被快速更新（非原子地），
因为 Accessed 位和 Dirty 位不会丢失
但在快速页错误之后这就不成立了，因为在读取 spte 和更spte 之间，spte 可能变成可写。如下面
的情况：

+-------------------------------------------------------------------------+
**| 开始时**
: |
|                                                                         |
|  spte.W = 0                                                             |
|  spte.Accessed = 1                                                      |
+-------------------------------------+-----------------------------------+
| CPU 0:                              | CPU 1:                            |
+-------------------------------------+-----------------------------------+
**| 鍦?mmu_spte_update() 涓?*
: |                                   |
|                                     |                                   |
|  old_spte = *spte;                  |                                   |
|                                     |                                   |
|                                     |                                   |
|  /** 'if' 条件被满足**/            |                                   |
|  if (old_spte.Accessed == 1 &&      |                                   |
|       old_spte.W == 0)              |                                   |
|     spte = new_spte;                |                                   |
+-------------------------------------+-----------------------------------+
**|                                     | 在快速页错误路径*
: |
|                                     |                                   |
|                                     |    spte.W = 1                     |
|                                     |                                   |
**|                                     | spte 的内存写*
: |
|                                     |                                   |
|                                     |    spte.Dirty = 1                 |
+-------------------------------------+-----------------------------------+
**|  **
: |                                   |
|                                     |                                   |
|   else                              |                                   |
|     old_spte = xchg(spte, new_spte);|                                   |
|   if (old_spte.Accessed &&          |                                   |
|       !new_spte.Accessed)           |                                   |
|     flush = true;                   |                                   |
|   if (old_spte.Dirty &&             |                                   |
|       !new_spte.Dirty)              |                                   |
|     flush = true;                   |                                   |
|     OOPS!!!                         |                                   |
+-------------------------------------+-----------------------------------+

在这种情况下 Dirty 位丢失了
为了避免这类问题，如spte 可以mmu-lock 之外更新，我们总是将其视为“volatile”（易变[spte_needs_atomic_update()]；这意味着在这种情况下 spte 总是被原子地更新
3) spte 更新而刷tlb

如果 spte 从可写更新为只读，我们应该刷新所TLB，否rmap_write_protect 会找到一个只读的
spte，即使该可写spte 可能仍缓存在某个 CPU TLB 中
如前所述，spte 在快速页错误路径上可以在 mmu-lock 之外被更新为可写。为了便于审计该路径，我们在
mmu_spte_update() 中查看是否需要因该原因刷TLB，因为这是一个更spte（present -> present的通用函数
由于 spte 在可以在 mmu-lock 之外更新时是“volatile”的，我们总是原子地更spte，从而可以避由快速页错误引起的竞争。参spte_needs_atomic_update() mmu_spte_update() 中的注释
无锁访问追踪（Lockless Access Tracking）：

这用于使EPT 但不支持 EPT A/D 位的 Intel CPU。在这种情况下，PTE 被标记为 A/D 禁用（使忽略位），当 KVM MMU 通知器被调用以追踪对某个页的访问（通过 kvm_mmu_notifier_clear_flush_young时，它通过清除 PTE 中的 RWX 位并将原始的 R & X 位存入更多未使用/忽略位，在硬件中PTE 标记不存在。当 VM 稍后尝试访问该页时，会产生一个错误，并使用上述快速页错误机制PTE 原子地恢复为
Present 状态。当 PTE 被标记为访问追踪时，W 位不会被保存；在恢复Present 状态时，W 位根据是是一次写访问来设置。如果不是，W 位将保持清零，直到发生一次写访问，届时它将使用上述脏位追机制被设置
### 3. 参考（Reference

##### ``kvm_lock``


:Type:		mutex
:Arch:		any
:Protects:	- vm_list

##### ``kvm_usage_lock``


:Type:		mutex
:Arch:		any
:Protects:	- kvm_usage_count
  - 硬件虚拟化的启用/禁用
:Comment:	存在该锁是为了允许在 kvm_usage_count 受保护时获取 cpus_read_lock()		这简化了虚拟化启用逻辑
##### ``kvm->mn_invalidate_lock``


:Type:          spinlock_t
:Arch:          any
:Protects:      mn_active_invalidate_count, mn_memslots_update_rcuwait

##### ``kvm_arch::tsc_write_lock``


:Type:		raw_spinlock_t
:Arch:		x86
**:Protects:	- kvm_arch**
: {last_tsc_write,last_tsc_nsec,last_tsc_offset}
  - vmcb 中的 tsc 偏移
:Comment:	'raw' 是因为更tsc 偏移时不可被抢占
##### ``kvm->mmu_lock``


:Type:		spinlock_t 鎴?rwlock_t
:Arch:		any
:Protects:	- 影子影子 tlb :Comment:	这是一个自旋锁，因为它用于 mmu 通知器中
##### ``kvm->srcu``


:Type:		srcu 閿?:Arch:		any
:Protects:	- kvm->memslots
  - kvm->buses
:Comment:		访问 memslots（例如使gfn_to_* 函数）以及访问内核MMIO/PIO
		地址到设备结构的映射（kvm->buses）时，必须持srcu 读锁		如果多个函数需要，srcu 索引可以存储在每 vcpu kvm_vcpu->srcu_idx 中
##### ``kvm->slots_arch_lock``


:Type:          mutex
:Arch:          any（尽管仅x86 上需要）
:Protects:      必须`kvm->srcu` 读侧临界区中修改memslots 的任何架构特定字段:Comment:       在读取指向当memslots 的指针之前必须持有，直到memslots 的所                修改完成之后
##### ``wakeup_vcpus_on_cpu_lock``


:Type:		spinlock_t
:Arch:		x86
:Protects:	wakeup_vcpus_on_cpu
:Comment:	这是一个每 CPU 锁，用于 VT-d 投递中断（posted-interrupts）。当支持 VT-d
		投递中断且 VM 分配了设备时，我们将被阻塞的 vCPU 放入blocked_vcpu_on_cpu_lock
		保护blocked_vcpu_on_cpu 列表中。当来自分配设备的外部中断导VT-d 硬件发出
		唤醒通知事件时，我们会在该列表中找到 vCPU 并将其唤醒
##### ``vendor_module_lock``


:Type:		mutex
:Arch:		x86
:Protects:	加载一个厂商模块（kvm_amd kvm_intel:Comment:	存在该锁是因为使kvm_lock 会导致死锁。kvm_lock 在通知器中被持有，例如
    __kvmclock_cpufreq_notifier()，而该通知器可能在持有 cpu_hotplug_lock（例如来    cpufreq_boost_trigger_state()）时被调用；并且许多操作在加载厂商模块时需要获    cpu_hotplug_lock，例如更新静态调用（static call）