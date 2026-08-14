
## x86 kvm 影子 mmu


mmu（位于 arch/x86/kvm，文件 mmu.[ch] 与 paging_tmpl.h）负责向 guest 呈现一个标准的 x86 mmu，同时将 guest 物理地址转换为 host 物理地址。

mmu 代码力求满足以下要求：

- 正确性（correctness）：
	       除了时序之外，guest 不应能够判断它运行在模拟的 mmu 之上（我们试图符合规范，而非模拟某个特定实现的特性，例如 tlb 大小）
- 安全性（security）：
	       guest 不得能够触碰未分配给它的 host 内存
- 性能（performance）：
               将 mmu 带来的性能开销降到最低
- 可扩展性（scaling）：
               需要能扩展到具有大内存和大 vcpu 的 guest
- 硬件（hardware）：
               支持全系列的 x86 虚拟化硬件
- 集成（integration）：
               Linux 内存管理代码必须掌控 guest 内存，以便交换（swapping）、页迁移、页合并、透明大页（transparent hugepages）及类似特性无需修改即可工作
- 脏页跟踪（dirty tracking）：
               报告对 guest 内存的写入，以启用实时迁移和基于帧缓冲的显示
- 内存占用（footprint）：
               保持固定的内核内存量较低（大部分内存应当是可回收的）
- 可靠性（reliability）：
               避免使用多页或 GFP_ATOMIC 分配

## 缩写

====  ====================================================================
pfn   host page frame number（宿主页帧号）
hpa   host physical address（宿主物理地址）
hva   host virtual address（宿主虚拟地址）
gfn   guest frame number（客户机页帧号）
gpa   guest physical address（客户机物理地址）
gva   guest virtual address（客户机虚拟地址）
ngpa  nested guest physical address（嵌套客户机物理地址）
ngva  nested guest virtual address（嵌套客户机虚拟地址）
pte   page table entry（页表项，也用于泛指地指代分页结构条目）
gpte  guest pte（指向 gfn）
spte  shadow pte（指向 pfn）
tdp   two dimensional paging（二维分页，NPT 与 EPT 的厂商中立术语）
====  ====================================================================

## 支持的虚拟与实际硬件

mmu 支持第一代 mmu 硬件，它允许在 guest 进入时原子地切换当前的 paging 模式与 cr3，同时也支持二维分页（AMD 的 NPT 与 Intel 的 EPT）。它所模拟的硬件是传统的 2/3/4 级 x86 mmu，支持全局页、pae、pse、pse36、cr0.wp 以及 1GB 页。模拟的硬件也能够在支持 NPT 的宿主上暴露支持 NPT 的硬件。

## 转换

mmu 的主要工作是对处理器的 mmu 进行编程，以为 guest 转换地址。在不同的时间需要不同的转换：

- 当 guest paging 被禁用时，我们将 guest 物理地址转换为 host 物理地址（gpa->hpa）
- 当 guest paging 被启用时，我们将 guest 虚拟地址转换为 guest 物理地址，再转换为 host 物理地址（gva->gpa->hpa）
- 当 guest 启动它自己的 guest 时，我们将嵌套 guest 虚拟地址转换为嵌套 guest 物理地址，再转换为 guest 物理地址，再转换为 host 物理地址（ngva->ngpa->gpa->hpa）

主要的挑战是将 1 到 3 种转换编码到只支持 1 种（传统）和 2 种（tdp）转换的硬件中。当所需转换的数量与硬件匹配时，mmu 以直接模式（direct mode）运行；否则以影子模式（shadow mode）运行（见下文）。

## 内存

guest 内存（gpa）是使用 kvm 的进程的用户地址空间的一部分。用户空间定义了 guest 地址与用户地址之间的转换（gpa->hva）；注意两个 gpa 可能别名到同一个 hva，但反之则不然。

这些 hva 可以使用宿主可用的任何方法作为后端：匿名内存、文件后端内存以及设备内存。内存可能随时被宿主换页。

## 事件

mmu 由事件驱动，部分来自 guest，部分来自 host。

来自 guest 的事件：

- 对控制寄存器的写入（尤其是 cr3）
- invlpg/invlpga 指令的执行
- 对缺失或受保护转换的访问

来自 host 的事件：

- gpa->hpa 转换的变化（通过 gpa->hva 变化或 hva->hpa 变化）
- 内存压力（shrinker）

## 影子页

主要的数据结构是影子页（shadow page），即 'struct kvm_mmu_page'。一个影子页包含 512 个 spte，可以是叶子（leaf）或非叶子（nonleaf）spte。一个影子页可以混合包含 leaf 和 nonleaf spte。

nonleaf spte 让硬件 mmu 能够到达 leaf 页，并不直接与某个转换相关。它指向其他影子页。

leaf spte 对应于编码到一个分页结构条目中的一到两个转换。它们始终是转换栈的最低层，可选的高层转换留给 NPT/EPT。leaf pte 指向 guest 页。

下表显示了由 leaf pte 编码的转换，高层转换在括号中：

```

  nonpaging:     gpa->hpa
  paging:        gva->gpa->hpa
  paging, tdp:   (gva->)gpa->hpa

 Nested guests::

  non-tdp:       ngva->gpa->hpa  (*)
  tdp:           (ngva->)ngpa->gpa->hpa

  (*) the guest hypervisor will encode the ngva->gpa translation into its page
      tables if npt is not present

```
影子页包含以下信息：
  role.level：
    此影子页所属的影子分页层级中的级别。
    1=4k spte，2=2M spte，3=1G spte，依此类推。
  role.direct：
    如果设置，则从此页可达的 leaf spte 对应于一个线性范围。
    示例包括实模式转换、由小而宿主页支持的大 guest 页，以及 NPT 或 EPT 活跃时的 gpa->hpa 转换。
    线性范围起始于 (gfn << PAGE_SHIFT)，其大小由 role.level 决定（第一级为 2MB，第二级为 1GB，第三级为 0.5TB，第四级为 256TB）
    如果清除，此页对应于由 gfn 字段表示的 guest 页表。
  role.quadrant：
    当 role.has_4_byte_gpte=1 时，guest 使用 32 位 gpte，而 host 使用 64 位 spte。这意味着 guest 页表包含的 pte 比 host 多，因此需要一个以上的影子页来影子一个 guest 页。
    对于第一级影子页，role.quadrant 可以为 0 或 1，表示 guest 页表中的第一或第二个 512-gpte 块。对于第二级页表，每个 32 位 gpte 被转换为两个 64 位 spte（因为每个第一级 guest 页由两个第一级影子页影子），因此 role.quadrant 取值在 0..3 范围内。每个象限映射 1GB 虚拟地址空间。
  role.access：
    以 uwx 形式从父 pte 继承的 guest 访问权限。注意执行权限是正向的，而非负向的。
  role.invalid：
    该页无效，不应被使用。它是一个当前被固定的根页（由指向它的某个 cpu 硬件寄存器）；一旦解除固定，它将被销毁。
  role.has_4_byte_gpte：
    反映该页所适用的 guest PTE 的大小，即如果使用直接映射或 64 位 gpte，则为 '0'，如果使用 32 位 gpte，则为 '1'。
  role.efer_nx：
    包含该页所适用的 efer.nx 的值。
  role.cr0_wp：
    包含该页所适用的 cr0.wp 的值。
  role.smep_andnot_wp：
    包含该页所适用的 cr4.smep && !cr0.wp 的值（此值为真的页与其他页不同；见下文对 cr0.wp=0 的处理）。
  role.smap_andnot_wp：
    包含该页所适用的 cr4.smap && !cr0.wp 的值（此值为真的页与其他页不同；见下文对 cr0.wp=0 的处理）。
  role.smm：
    如果该页在系统管理模式（system management mode）下有效，则为 1。此字段决定使用 kvm_memslots 数组中的哪一个来构建此影子页；它也用于通过 kvm_memslots_for_spte_role 宏与 __gfn_to_memslot 从一个 struct kvm_mmu_page 回到一个 memslot。
  role.ad_disabled：
    如果 MMU 实例不能使用 A/D 位，则为 1。EPT 在 Haswell 之前没有 A/D 位；如果 L1 hypervisor 未启用 A/D 位，影子 EPT 页表也不能使用 A/D 位。
  role.guest_mode：
    指示该影子页是为嵌套 guest 创建的。
  role.passthrough：
    该页不是由 guest 页表作为后端的，但其第一个条目指向一个 guest 页表。当 NPT 使用 5 级页表（host CR4.LA57=1）且正在影子 L1 的 4 级 NPT（L1 CR4.LA57=0）时设置此字段。
  mmu_valid_gen：
    该页的 MMU 代（generation），用于在不长时间阻塞 vCPU 的情况下快速清空（zap）一个 VM 内的所有 MMU 页。具体而言，KVM 更新每个 VM 的有效 MMU 代，这会导致每个 mmu 页的 mmu_valid_gen 失配。这使得所有现有的 MMU 页变得过时。过时的页不能被使用。因此，vCPU 必须在重新进入 guest 之前加载一个新的有效根。MMU 代只可能是 '0' 或 '1'。注意，TDP MMU 不使用此字段，因为非根的 TDP MMU 页只能从其所属的根到达。因此对于 TDP MMU，在根页中使用 role.invalid 来使所有 MMU 页失效就足够了。
  gfn：
    要么包含被此页影子的转换的 guest 页表，要么是线性转换的基页帧。参见 role.direct。
  spt：
    一整页的 64 位 spte，包含此页的转换。由 kvm 与硬件两者访问。
    spt 所指向的页将其 page->private 指回影子页结构。
    spt 中的 spte 要么指向 guest 页，要么指向更低层的影子页。
    具体而言，如果 sp1 和 sp2 是影子页，则 sp1->spt[n] 可能指向 __pa(sp2->spt)。sp2 将通过 parent_pte 指回 sp1。
    spt 数组构成一个以影子页为节点、以 guest 页为叶子的 DAG 结构。
  shadowed_translation：
    一个包含 512 个影子转换条目的数组，每个存在的 pte 对应一个。用于执行从 pte 到 gfn 的反向映射及其访问权限。当设置 role.direct 时，不会分配 shadow_translation 数组。这是因为此数组中任何元素的 gfn 在使用时都可以从 gfn 字段计算出来。此外，当设置 role.direct 时，KVM 不会跟踪每个 gfn 的访问权限。参见 role.direct 与 gfn。
  root_count / tdp_mmu_root_count：
     root_count 是 Shadow MMU 中根影子页的引用计数器。vCPU 在获取将被用作根页（即将被直接加载到硬件中的页，如 CR3、PDPTRs、nCR3 EPTP）的影子页时提升引用计数。根页在其引用计数非零时不能被销毁。参见 role.invalid。tdp_mmu_root_count 类似，但专用于 TDP MMU 中作为原子引用计数。
  parent_ptes：
    指向此页 spt 的 pte/ptes 的反向映射。如果 parent_ptes 的 bit 0 为零，则只有一个 spte 指向此页，并且 parent_ptes 指向这个单一的 spte；否则，存在多个指向此页的 spte，且 (parent_ptes & ~0x1) 指向一个包含父 spte 列表的数据结构。
  ptep：
    指向此影子页的 SPTE 的内核虚拟地址。专由 TDP MMU 使用，此字段与 parent_ptes 是联合体。
  unsync：
    如果为真，则此页中的转换可能与 guest 的转换不匹配。这等价于当 pte 被改变但尚未刷新 tlb 条目时 tlb 的状态。相应地，当 guest 执行 invlpg 或通过其他方式刷新其 tlb 时，unsync pte 会被同步。对 leaf 页有效。
  unsync_children：
    此页中有多少个 spte 指向 unsync（或具有未同步子节点）的页。
  unsync_child_bitmap：
    一个位图，指示 spt 中的哪些 spte（直接或间接）指向可能未同步的页。用于快速定位从给定页可达的所有未同步页。
  clear_spte_count：
    仅存在于 32 位宿主上，因为 64 位 spte 无法被原子写入。读者在脱离 MMU 锁运行时使用它来检测正在进行的更新，并在写入者完成写入之前重试。
  write_flooding_count：
    guest 可能多次写入一个页表，如果该页需要被写保护（见下文的“同步与未同步页”），将导致大量模拟（emulation）。leaf 页可以是未同步的，这样就不会触发频繁的模拟，但这对于非叶子页是不可能的。此字段统计自上次实际使用页表以来发生的模拟次数；如果在此页上触发模拟过于频繁，KVM 将取消映射该页，以避免将来的模拟。
  tdp_mmu_page：
    如果该影子页是 TDP MMU 页，则为 1。此变量用于在遍历可能包含来自 TDP MMU 和影子 MMU 两者的页的任何数据结构时，对 KVM 的控制流进行分叉。

## 反向映射

mmu 维护一个反向映射，由此给定其 gfn 可以到达映射该页的所有 pte。例如，这在换出一个页时使用。

## 同步与未同步页

guest 使用两个事件来同步其 tlb 与页表：tlb 刷新与页失效（invlpg）。

tlb 刷新意味着我们需要同步从 guest 的 cr3 可达的所有 spte。这开销很大，因此我们将所有 guest 页表保持写保护，并在写入 gpte 时同步 spte 到 gpte。

一种特殊情况是，当 guest 页表可从当前 guest cr3 到达时。在这种情况下，guest 有义务在使用该转换之前发出一条 invlpg 指令。我们利用这一点，移除对 guest 页的写保护，并允许 guest 自由修改它。当 guest 调用 invlpg 时，我们同步被修改的 gpte。这减少了当 guest 修改多个 gpte，或者当某个 guest 页不再用作页表而被用于随机 guest 数据时，我们必须进行的模拟量。

作为副作用，我们必须在 tlb 刷新时重新同步所有可达的未同步影子页。

## 对事件的响应

- guest 页错误（或 npt 页错误，或 ept violation）

这是最复杂的事件。页错误的原因可能是：

  - 真正的 guest 错误（guest 转换不允许此访问）(*)
  - 对缺失转换的访问
  - 对受保护转换的访问
    - 当记录脏页时，内存被写保护
    - 同步的影子页被写保护 (*)
  - 对不可转换内存（mmio）的访问

  (*) 在直接模式下不适用

处理页错误的方法如下：

 - 如果错误码的 RSV 位被设置，则页错误是由 guest 访问 MMIO 引起的，并且可用的缓存 MMIO 信息。

   - 遍历影子页表
   - 检查 spte 中有效的代编号（见下文的“MMIO spte 的快速失效”）
   - 将信息缓存到 vcpu->arch.mmio_gva、vcpu->arch.mmio_access 与 vcpu->arch.mmio_gfn，并调用模拟器（emulator）

 - 如果错误码的 P 位与 R/W 位都被设置，这可能可以作为“快速页错误”（fast page fault，无需获取 MMU 锁即可修复）来处理。参见 Documentation/virt/kvm/locking.rst 中的描述。

 - 必要时，遍历 guest 页表以确定 guest 转换（gva->gpa 或 ngpa->gpa）

   - 如果权限不足，将错误反射回 guest

 - 确定 host 页

   - 如果这是一个 mmio 请求，则没有 host 页；将信息缓存到 vcpu->arch.mmio_gva、vcpu->arch.mmio_access 与 vcpu->arch.mmio_gfn

 - 遍历影子页表以找到该转换的 spte，必要时实例化缺失的中间页表

   - 如果这是一个 mmio 请求，将 mmio 信息缓存到 spte 并设置该 spte 上的某个保留位（参见 kvm_mmu_set_mmio_spte_mask 的调用者）

 - 尝试使该页未同步（unsynchronize）

   - 如果成功，我们可以让 guest 继续并修改 gpte

 - 模拟该指令

   - 如果失败，取消影子化（unshadow）该页并让 guest 继续

 - 更新被该指令修改的任何转换

invlpg 处理：

  - 遍历影子页层级并丢弃受影响的转换
  - 尝试重新实例化所指示的转换，期望 guest 在不久的将来会使用它

guest 控制寄存器更新：

- mov to cr3

  - 查找新的影子根
  - 同步新可达的影子页

- mov to cr0/cr4/efer

  - 为新的 paging 模式建立 mmu 上下文
  - 查找新的影子根
  - 同步新可达的影子页

host 转换更新：

  - 以更新后的 hva 调用 mmu notifier
  - 通过反向映射查找受影响的 spte
  - 丢弃（或更新）转换

## 模拟 cr0.wp

如果未启用 tdp，host 必须保持 cr0.wp=1，以便页写保护对 guest 内核生效，而非 guest 用户空间。当 guest cr0.wp=1 时，这不会出现问題。然而当 guest cr0.wp=0 时，我们无法将 gpte.u=1、gpte.w=0 的权限映射到任何 spte（其语义要求允许任何 guest 内核访问加上用户读访问）。

我们通过根据错误类型将权限映射到两种可能的 spte 来处理：

- 内核写错误：spte.u=0，spte.w=1（允许完整的内核访问，禁止用户访问）
- 读错误：spte.u=1，spte.w=0（允许完整读访问，禁止内核写访问）

（用户写错误会产生 #PF）

在第一种情况下有两个额外的复杂之处：

- 如果启用了 CR4.SMEP：由于我们将该页变成了内核页，内核现在可能执行它。我们通过同时设置 spte.nx 来处理。如果我们遇到用户取指或读错误，我们会将 spte.u=1 和 spte.nx=gpte.nx 改回。为了使这生效，当使用影子分页时，KVM 强制 EFER.NX 为 1。
- 如果禁用了 CR4.SMAP：由于该页已被改为内核页，当 CR4.SMAP 启用时它不能被复用。我们将 CR4.SMAP && !CR0.WP 放入影子页的 role 中以避免这种情况。注意，这里我们不关心 CR4.SMAP 启用的情况，因为 KVM 会由于权限检查失败而直接向 guest 注入 #PF。

为了防止一个被转换为 cr0.wp=0 的内核页在 cr0.wp 变为 1 之后被内核写入，我们将 cr0.wp 的值作为页 role 的一部分。这意味着用某个 cr0.wp 值创建的 spte 不能在 cr0.wp 取不同值时使用——它将被影子页查找代码简单地忽略。当用 cr0.wp=0 和 cr4.smep=0 创建的 spte 在将 cr4.smep 改为 1 之后被使用时，存在类似的问题。为避免这种情况，!cr0.wp && cr4.smep 的值也被作为页 role 的一部分。

## 大页

mmu 支持大 guest 页与小 host 页、以及大 host 页与小 guest 页的所有组合。支持的页大小包括 4k、2M、4M 与 1G。由于 mmu 始终使用 PAE 分页，4M 页被视为两个独立的 2M 页，在 guest 与 host 上皆如此。

要实例化一个大的 spte，必须满足四个约束：

- spte 必须指向一个大的 host 页
- guest pte 必须至少是等效大小的大 pte（如果启用 tdp，则不存在 guest pte，此条件自然满足）
- 如果 spte 将是可写的，则大页帧不得与任何写保护页重叠
- guest 页必须完全包含在一个单独的内存槽（memory slot）中

为了检查后两个条件，mmu 为每个内存槽与每个大页大小维护一组 ->disallow_lpage 数组。每个写保护页都会使其 disallow_lpage 递增，从而阻止大 spte 的实例化。未对齐内存槽末尾的帧被人为地增大了 ->disallow_lpages，因此它们永远无法被实例化。

## MMIO spte 的快速失效

如上文“对事件的响应”中所述，kvm 会将 MMIO 信息缓存在 leaf spte 中。当新增一个 memslot 或更改一个现有的 memslot 时，此信息可能变得过时，需要失效。这还要求在所有影子页遍历时持有 MMU 锁，并通过类似的技术使其更具可扩展性。

MMIO spte 有几个空闲位，用于存储一个代编号（generation number）。全局代编号存储在 kvm_memslots(kvm)->generation 中，并在 guest 内存信息发生变化时递增。

当 KVM 找到一个 MMIO spte 时，它会检查该 spte 的代编号。如果 spte 的代编号不等于全局代编号，它将忽略缓存的 MMIO 信息，并通过慢速路径处理页错误。

由于 mmio spte 上只使用 18 位来存储代编号，发生溢出时会清空所有页。

遗憾的是，单次内存访问可能会多次访问 kvm_memslots(kvm)，最后一次发生在代编号被取出并存入 MMIO spte 时。因此，MMIO spte 可能基于过期的信息创建，但带有最新的代编号。

为避免这种情况，代编号在 synchronize_srcu 返回后再次递增；因此，kvm_memslots(kvm)->generation 的 bit 63 仅在 memslot 更新期间被设为 1，而某些 SRCU 读者可能正在使用旧副本。我们不希望使用以奇数代编号创建的 MMIO spte，而我们可以在不损失 MMIO spte 中一个位的情况下做到这一点。代的“更新进行中”位不存储在 MMIO spte 中，因此在从 spte 取出代时它隐式为零。如果 KVM 不走运，在更新进行期间创建了一个 MMIO spte，则下一次对该 spte 的访问将始终是缓存未命中。例如，在更新窗口期间的后续访问将由于进行中标志不同而缺失，而更新窗口关闭后的访问将具有更高的代编号（相对于 spte 而言）。

## 延伸阅读

- KVM Forum 2008 上的 NPT 演讲
  https://www.linux-kvm.org/images/c/c8/KvmForum2008%24kdf2008_21.pdf
