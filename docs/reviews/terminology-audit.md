# 全局术语一致性复核报告

- 基准：`docs/系统文档/translations/zh_CN/glossary.md`

- 扫描范围：`docs/系统文档/` 全部已翻译 `.md`（3948 个文件）

- 生成时间：自动化复核（只读，未修改任何文件）


## 结论摘要

| 基准术语 | 严重度 | 基准出现次数 | 变体译法（命中数） |
| --- | --- | ---: | --- |
| 自旋锁 | wrong | 273 | （无） |
| 信号量 | wrong | 91 | （无） |
| 内存屏障 | wrong | 103 | **内存栅栏**×4 |
| 非统一内存访问 | wrong | 6 | **非一致内存访问**×2 |
| 巨页 | wrong | 122 | **大页**×386；**巨型页**×6 |
| 虚拟机超级管理器 | wrong | 3 | **虚拟机管理程序**×4；**虚拟机监控器**×63；**虚拟机监视器**×31 |
| 页表项 | variant | 79 | **页表条目**×20 |
| 互斥锁 | variant | 66 | **互斥体**×110 |
| 调度器 | variant | 565 | **调度程序**×4 |
| 调度域 | variant | 106 | （无） |
| 原子操作 | variant | 99 | （无） |
| 能耗感知调度 | variant | 2 | （无） |

## 决策建议（复核结论，未执行任何替换）

本次复核为**只读审计**，未对任一文件做改动。按扫描数据，术语一致性问题分为两类：

**A 类 — 与术语表明显冲突、且基准译法占绝对多数（建议二次精修阶段直接归一）：**
- 调度器 vs 调度程序：基准 565 处 vs 4 处，仅 4 处用 调度程序，可安全归一。
- 内存屏障 vs 内存栅栏：103 vs 4。
- 非统一内存访问 vs 非一致内存访问：6 vs 2。
- 页表项 vs 页表条目：79 vs 20（variant，建议统一为 页表项）。

**B 类 — 术语表基准译法实际极少使用，翻译正文以变体为主（建议先修订术语表，再决定是否反向归一）：**
- 巨页 vs 大页：基准 122 处 vs **386** 处，大页 为实际主流译法。
- 虚拟机超级管理器 vs 虚拟机监控器：基准 3 处 vs **63** 处（另 虚拟机监视器 31、虚拟机管理程序 4），虚拟机监控器 为主流。
- 互斥锁 vs 互斥体：基准 66 处 vs **110** 处，互斥体 为主流。

**结论**：A 类可在后续精修中安全归一；B 类不宜盲目将正文改为术语表译法（会改动数百处且违背社区主流习惯），应先将术语表更新为实际主流译法（大页 / 虚拟机监控器 / 互斥体），或统一沿用正文既有主流译法。**是否执行替换由用户决策，本脚本不自动替换。**


## 逐术语明细

### 自旋锁（severity=wrong，基准出现 273 次）

### 信号量（severity=wrong，基准出现 91 次）

### 内存屏障（severity=wrong，基准出现 103 次）

#### 变体：`内存栅栏`（4 处，涉及 1 个文件）

- `docs/系统文档/gpu/drm-vm-bind-async.md:20` — - `memory fence`（内存栅栏）：一种不同于 dma-fence 的同步对象。内存栅栏使用指定内存位置的值来确定已发信号状态。内存栅栏既可以由 GPU 也可以由 CPU 等待和发信号。内存栅栏有时被称为 user-fence、u
- `docs/系统文档/gpu/drm-vm-bind-async.md:46` — 由于异步 VM_BIND 操作可能使用嵌入在 out-syncobj 中的 dma-fence 以及在 KMD 内部用于发信号指示绑定完成的 dma-fence，任何作为 VM_BIND in-fence 给出的内存栅栏都需要在 VM_BI
- `docs/系统文档/gpu/drm-vm-bind-async.md:48` — 异步 VM_BIND 操作的目的是让用户模式驱动能够流水线化交错进行的 gpu_vm 修改和 exec 函数。对于长时间运行的工作负载，这种绑定操作的流水线化是不允许的，任何 in-fence 都需要被同步等待。这其中的原因有两方面。首先，
- `docs/系统文档/gpu/drm-vm-bind-async.md:52` — 同样，对于长时间运行 gpu_vm 的 VM_BIND，用户模式驱动通常应选择内存栅栏作为 out-fence，因为这为内核模式驱动在绑定/解绑操作中注入其他操作（例如向批处理缓冲区中插入断点）提供了更大的灵活性。然后，工作负载执行可以轻松

### 非统一内存访问（severity=wrong，基准出现 6 次）

#### 变体：`非一致内存访问`（2 处，涉及 2 个文件）

- `docs/系统文档/admin-guide/mm/concepts.md:84` — 许多多处理器机器是 NUMA —— 非一致内存访问（Non-Uniform Memory
- `docs/系统文档/admin-guide/mm/numa_memory_policy.md:7` — 在 Linux 内核中，“内存策略（memory policy）”决定了在 NUMA 系统或模拟的 NUMA 系统中，内核将从哪个节点分配内存。Linux 自 2.4.? 起就支持具有非一致内存访问（NUMA）架构的平台。当前的内存策略支持

### 巨页（severity=wrong，基准出现 122 次）

#### 变体：`大页`（386 处，涉及 42 个文件）

- `docs/系统文档/PROJECT.md:1120` — | MM_ID | def_bool | 透明大页允许内核在可能时透明地对应用使用大页与 huge tlb。该特性可通过……提升某些应用的计算性能 |
- `docs/系统文档/PROJECT.md:1144` — | NO_PAGE_MAPCOUNT | bool | 不为属于较大分配（如透明大页）的页维护每页 mapcount。启用此配置选项后，一些依赖此信息的接口将…… |
- `docs/系统文档/PROJECT.md:1317` — | THP_SWAP | def_bool | 以整体方式交换透明大页，无需拆分。XXX：目前，支撑透明大页的交换簇将在换出后被拆分。供具有合理 THP……的体系结构选择 |
- `docs/系统文档/PROJECT.md:1329` — | TRANSPARENT_HUGEPAGE_ALWAYS | bool | 总是启用透明大页，可能会增加应用的内存占用而没有保证的收益，但它会对所有应用自动生效。 |
- `docs/系统文档/PROJECT.md:1330` — | TRANSPARENT_HUGEPAGE_MADVISE | bool | 启用透明大页 madvise，只会为使用 madvise(MADV_HUGEPAGE) 的应用带来性能提升，但不会冒增加应用内存占用的风险…… |
- `docs/系统文档/PROJECT.md:1331` — | TRANSPARENT_HUGEPAGE_NEVER | bool | 默认禁用透明大页。仍可在运行时通过 sysfs 启用。 |
- `docs/系统文档/PROJECT.md:1332` — | TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | 仅当应用提供 madvise(MADV_HUGEPAGE) 提示时，才为 shmem 挂载启用大页分配。这确保大页仅在响应来自……的显式请求
- `docs/系统文档/PROJECT.md:1333` — | TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | 总是尝试为 shmem 挂载分配大页，可能会增加应用内存占用而没有保证的收益，但它会对所有应用自动生效。 |
- `docs/系统文档/_w2_test_copy.md:1120` — | MM_ID | def_bool | 透明大页允许内核在可能时透明地对应用程序使用大页与大页 TLB。该特性可通过...对某些应用程序提升计算性能。 |
- `docs/系统文档/admin-guide/cgroup-v2.md:1237` — 由透明大页（transparent hugepages）支持的匿名映射所使用的内存量。
- `docs/系统文档/admin-guide/numastat.md:6` — 所有单位均为页。大页（Hugepages）有独立的计数器。
- `docs/系统文档/admin-guide/cgroup-v1/hugetlb.md:32` — 对于支持三种大页大小（64k、32M 和 1G）的系统，控制
- `docs/系统文档/admin-guide/cgroup-v1/memory.md:463` — rss             匿名和交换缓存内存（包含透明大页）的字节数。
- `docs/系统文档/admin-guide/hw-vuln/multihit.md:44` — 修改分页结构，使得同一线性地址使用大页大小（2 MB、4 MB、1 GB）但具有不同的物理地址或
- `docs/系统文档/admin-guide/mm/concepts.md:43` — ## 大页（Huge Pages）
- …（其余 371 处略，详见 `.terminology_audit.json`）

#### 变体：`巨型页`（6 处，涉及 2 个文件）

- `docs/系统文档/admin-guide/sysctl/vm.md:450` — 该参数控制是否可从 ZONE_MOVABLE 分配巨型页（gigantic pages）。如果设为非零，则可从 ZONE_MOVABLE 分配巨型页。ZONE_MOVABLE 内存可通过内核启动参数 `kernelcore` 创建，或通过内
- `docs/系统文档/admin-guide/sysctl/vm.md:454` — 注意，使用 ZONE_MOVABLE 巨型页会使内存热移除（memory hotremove）不可靠。
- `docs/系统文档/admin-guide/sysctl/vm.md:456` — 内存热移除操作将无限期阻塞，直到管理员预留足够的巨型页来服务与内存下线过程相关的迁移请求。由于 HugeTLB 巨型页预留是一个手动过程（通过 `nodeN/hugepages/.../nr_hugepages` 接口），在仅尝试下线一个内
- `docs/系统文档/admin-guide/sysctl/vm.md:458` — 此外，由于单个块上可能预留多个巨型页，似乎巨型页可用于迁移，而实际上它们正在被移除的过程中。例如，如果 `memoryN` 包含两个巨型页，一个已预留、一个已分配，而管理员尝试下线该块，除非另一块 `memoryM` 上有另一个已预留的巨型
- `docs/系统文档/driver-api/cxl/allocation/hugepages.md:22` — 在 `ZONE_NORMAL` 中上线的 CXL 容量可用于 1GB 巨型页（Gigantic Page）分配。
- `docs/系统文档/driver-api/cxl/allocation/hugepages.md:24` — 在 `ZONE_MOVABLE` 中上线的 CXL 容量不能用于 1GB 巨型页分配。

### 虚拟机超级管理器（severity=wrong，基准出现 3 次）

#### 变体：`虚拟机管理程序`（4 处，涉及 3 个文件）

- `docs/系统文档/translations/zh_CN/dev-tools/gdb-kernel-debugging.md:9` — Kgdb内核调试器、QEMU等虚拟机管理程序或基于JTAG的硬件接口，支持在运行时使用gdb
- `docs/系统文档/translations/zh_CN/security/snp-tdx-threat-model.md:148` — 2. 防止宿主机特权升级到CoCo客户机Linux内核。虽然宿主机（及主机端虚拟机管理程序）
- `docs/系统文档/translations/zh_CN/security/snp-tdx-threat-model.md:164` — 直接内存访问（DMA）接口、访问PCI配置空间、特定于虚拟机管理程序（VMM）的超调用
- `docs/系统文档/virt/acrn/cpuid.md:5` — 在 ACRN 虚拟机管理程序上运行的来宾 VM 可以使用以下命令检查其某些功能

#### 变体：`虚拟机监控器`（63 处，涉及 23 个文件）

- `docs/系统文档/_w2_test_copy.md:756` — | COMPACT_UNEVICTABLE_DEFAULT | int | 空闲页报告允许从伙伴分配器增量获取空闲页，以便将这些页报告给另一个实体（例如虚拟机监控器），从而使内存... |
- `docs/系统文档/admin-guide/hw-vuln/attack_vector_controls.md:55` — 客体到主机的攻击向量涉及恶意 VM 试图将虚拟机监控器数据泄露到 VM 中。所涉及的数据可能是
- `docs/系统文档/admin-guide/hw-vuln/multihit.md:108` — KVM 虚拟机监控器将大页标记为不可执行的缓解机制，可以通过模块参数 “nx_huge_pages=” 控制。
- `docs/系统文档/admin-guide/hw-vuln/processor_mmio_stale_data.md:126` — 在受 MDS 影响的处理器上，内核已经在内核/用户空间、虚拟机监控器/guest 以及 C-state（空闲）
- `docs/系统文档/admin-guide/hw-vuln/vmscape.md:5` — 像 QEMU 这样的虚拟机监控器（hypervisor）。
- `docs/系统文档/admin-guide/hw-vuln/vmscape.md:7` — 即使某个虚拟机监控器可能没有任何敏感数据（如磁盘加密密钥），客户机用户态
- `docs/系统文档/admin-guide/hw-vuln/vmscape.md:8` — 也可能利用虚拟机监控器作为“被混淆的代理人”（confused deputy）来攻击
- `docs/系统文档/admin-guide/hw-vuln/vmscape.md:45` — 漏洞枚举与缓解措施不会在客户机内部应用。这是因为嵌套的虚拟机监控器应当
- `docs/系统文档/admin-guide/sysctl/index.md:67` — xen/		Xen 虚拟机监控器控制
- `docs/系统文档/arch/sparc/adi.md:22` — 平台上的 ADI 块大小由 hypervisor（虚拟机监控器）在机器描述表中提供给内核。
- `docs/系统文档/arch/x86/intel_txt.md:46` — Tboot 目前支持启动 Xen（自 v3.2 起支持 TXT 的开源 VMM/虚拟机监控器），以及现在的
- `docs/系统文档/arch/x86/mds.md:78` — 该缓解措施在内核/用户空间、虚拟机监控器/客户机以及 C-state（空闲）转换时被调用。
- `docs/系统文档/driver-api/generic_pt.md:114` — 刷新操作来最小化受影响的 VA 数量。如果处理 VA 的代价非常高，例如因为虚拟机监控器
- `docs/系统文档/mm/free_page_reporting.md:4` — 空闲页上报是一种 API，设备可以通过它注册以接收系统当前未使用的页列表。这在虚拟化场景下很有用，客户机随后能够利用这些数据通知虚拟机监控器它不再使用内存中的某些页。
- `docs/系统文档/networking/net_failover.md:12` — 半虚拟化驱动可利用它来启用一条低延迟的替代数据路径。它还支持在 VF 被拔出时故障转移到半虚拟化数据路径，从而实现由虚拟机监控器（hypervisor）控制的、对直连 VF 的 VM 进行热迁移。
- …（其余 48 处略，详见 `.terminology_audit.json`）

#### 变体：`虚拟机监视器`（31 处，涉及 5 个文件）

- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:63` — 如果处理器不支持扩展页表（Extended Page Tables），则只有当虚拟机监视器（hypervisor）未清理有效（影子）页表的内容时，攻击才可能成功。
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:118` — 为确保客户机无法攻击存在于 L1D 中的数据，虚拟机监视器在进入客户机之前刷新 L1D。
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:126` — 条件模式会避免在仅执行了经审计代码路径后才发生相应 VMENTER 的 VMEXIT 之后刷新 L1D。这些代码路径已经过验证，不会向攻击者暴露机密或其他有价值的数据，但它们可能泄露虚拟机监视器的地址空间布局信息。
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:144` — 当兄弟 SMT 线程之一运行在主机操作系统（虚拟机监视器）上下文中，而另一个运行在客户机上下文中时，主机内存可被攻击。来自主机操作系统上下文的有价值信息量取决于主机操作系统所执行的上下文，即中断、软中断与内核线程。若不对代码进行深入审查，无
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:223` — 为虚拟机禁用 EPT 可针对 L1TF 提供完全缓解，即使在启用 SMT 的情况下也是如此，因为客户机的有效页表由虚拟机监视器管理并清理。不过禁用 EPT 会带来显著的性能影响，尤其是在启用 Meltdown 缓解措施 KPTI 时。
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:225` — EPT 可在虚拟机监视器中通过 'kvm-intel.ept' 参数禁用。
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:236` — full		为 L1TF 漏洞提供所有可用的缓解措施。禁用 SMT 并启用虚拟机监视器
- `docs/系统文档/admin-guide/hw-vuln/l1tf.md:239` — SMT 控制与 L1D 刷新控制仍可在引导后通过 sysfs 接口进行。虚拟机监视器
- `docs/系统文档/arch/arm64/booting.md:9` — AArch64 异常模型由若干异常级别（EL0 - EL3）构成，其中 EL0、EL1 和 EL2 各自拥有一个安全与非安全副本。EL2 是 hypervisor（虚拟机监视器）级别，EL3 是最高优先级级别，且仅存在于安全模式。两者在架构
- `docs/系统文档/arch/arm64/mops.md:12` — ### 虚拟机监视器（Hypervisor）要求
- `docs/系统文档/security/tpm/xen-tpmfront.md:16` — Xen 中的虚拟机。vTPM 的每个主要组件都实现为一个独立的域，提供由虚拟机监视器保证的安全隔离。vTPM 域在
- `docs/系统文档/translations/zh_CN/security/snp-tdx-threat-model.md:49` — 全管理员的可信中介。宿主机侧的虚拟机监视器（VMM）通常由传统VMM功能的一个子集

### 页表项（severity=variant，基准出现 79 次）

#### 变体：`页表条目`（20 处，涉及 5 个文件）

- `docs/系统文档/mm/page_migration.md:46` — 4. 页表中对该页的所有引用都被转换为迁移条目。这会减少页的 mapcount。如果结果 mapcount 不为零，那么我们就不迁移该页。所有试图访问该页的用户空间进程现在将等待页锁，或等待迁移页表条目被移除。
- `docs/系统文档/mm/process_addrs.md:215` — 3. **清空/解除映射（Zapping/unmapping）** 页表条目 - 这是内核对仅在叶子级别清除页表映射的称呼，同时保留所有页表不变。这是内核中在文件截断、`!MADV_DONTNEED` 操作（经由 `!madvise`）等场
- `docs/系统文档/mm/process_addrs.md:226` — 当**安装** 页表条目时，必须持有 mmap 或 VMA 锁以保持 VMA 稳定。我们会在下面的页表锁细节小节中探讨其原因。
- `docs/系统文档/mm/process_addrs.md:358` — 是否小心地读取页表条目取决于架构，详见下面的原子性小节。
- `docs/系统文档/mm/process_addrs.md:365` — - 当修改一个页表条目时，**必须**持有该页表的页表锁，除非你能安全地假设没有人可以并发访问这些页表（例如在调用 `!free_pgtables` 时）。
- `docs/系统文档/mm/process_addrs.md:366` — - 对页表条目的读取和写入必须是**恰当**原子的。详见下面的原子性小节。
- `docs/系统文档/mm/process_addrs.md:388` — 如果正在执行写入，或者一次读取决定了是否发生写入（例如在安装页表条目时，例如 `!__pud_install`），则必须始终特别小心。在这些情况下，我们永远不能假设页表锁给了我们完全独占的访问，并且必须只获取一次页表条目。
- `docs/系统文档/mm/process_addrs.md:390` — 如果我们正在读取页表条目，那么我们只需确保编译器不会重排我们的加载。这通过 `!pXXp_get` 函数实现——`!pgdp_get`、`!p4dp_get`、`!pudp_get`、`!pmdp_get` 和 `!ptep_get`。
- `docs/系统文档/translations/zh_CN/arch/arm64/hugetlbpage.md:27` — 架构中转换页表条目(D4.5.3, ARM DDI 0487C.a)中提供一个连续
- `docs/系统文档/translations/zh_CN/mm/hmm.md:225` — 作特殊的“交换”页表条目，以便 CPU 进程在尝试访问已迁移到设备专用内存的页面时会发生异常。
- `docs/系统文档/virt/kvm/api.md:1955` — 创建一个页表条目。这仅对次要缺页（minor fault）有效，因此建议事先通过用户页表访问相关内存页。

### 互斥锁（severity=variant，基准出现 66 次）

#### 变体：`互斥体`（110 处，涉及 36 个文件）

- `docs/系统文档/PROJECT.md:901` — | DEBUG_LOCK_ALLOC | bool | 该特性将检查任何被持有的锁（自旋锁、rwlock、互斥体或 rwsem）是否被内核通过任一内存释放例程（kfree()、kmem_cache_free()、free_pages()……
- `docs/系统文档/PROJECT.md:905` — | DEBUG_MUTEXES | bool | 该特性允许检测并报告对互斥体语义的违反。 |
- `docs/系统文档/PROJECT.md:921` — | DEBUG_RT_MUTEXES | bool | 这允许自动检测并报告对 rt 互斥体语义的违反，以及 rt 互斥体相关的死锁（lockup）。 |
- `docs/系统文档/PROJECT.md:936` — | DEBUG_WW_MUTEX_SLOWPATH | bool | 该特性通过注入额外的 -EDEADLK 回退（wound/backoff）用例，为 w/w 互斥体使用者启用慢速路径测试。配合（CONFIG_PROVE_LOCKING）
- `docs/系统文档/PROJECT.md:986` — | FUTEX | bool | 禁用此选项将导致内核在构建时不包含对“fast userspace mutexes（快速用户空间互斥体）”的支持。生成的内核可能无法正确运行基于 glibc 的应用。 |
- `docs/系统文档/PROJECT.md:1219` — | SCHED_PROXY_EXEC | bool | 此选项启用代理执行（proxy execution），一种让持有互斥体的任务继承更高优先级等待者调度上下文的机制。 |
- `docs/系统文档/_TRANSLATE_PROTOCOL.md:14` — spinlock=自旋锁, mutex=互斥体, semaphore=信号量, page=页, interrupt=中断,
- `docs/系统文档/_w2_test_copy.md:986` — | FUTEX | bool | 禁用该选项将导致构建出的内核不支持"快速用户空间互斥体"。所得内核可能无法正确运行基于 glibc 的应用程序。 |
- `docs/系统文档/admin-guide/cgroup-v1/cgroups.md:366` — cgroup 系统使用一个全局互斥体 cgroup_mutex。任何想要修改 cgroup 的代码都应获取它。它也可以被获取以阻止 cgroups 被修改，但在那种情况下使用更具体的锁可能更合适。
- `docs/系统文档/core-api/kref.md:149` — 那违反了规则 3，因为你并没有已经持有一个有效指针。你必须添加一个互斥体（或其他锁）。
- `docs/系统文档/core-api/xarray.md:234` — 有时你需要用互斥体保护对 XArray 的访问，因为该锁在锁层次结构中位于另一个互斥体
- `docs/系统文档/driver-api/clk.md:250` — prepare 锁是一把互斥体（mutex），在对所有其他操作的调用期间持有。所有这些操作都允许
- `docs/系统文档/driver-api/pwm.md:149` — PWM 核心的列表操作受互斥体（mutex）保护，因此 pwm_get() 和 pwm_put() 不能从原子
- `docs/系统文档/driver-api/media/dtv-demux.md:18` — 每当 demux API 的函数修改共享数据时，都应考虑丢失更新和竞态条件问题，例如通过用互斥体（mutex）保护部分代码来解决。
- `docs/系统文档/driver-api/media/v4l2-dev.md:117` — 字段，它是一个指向互斥体的指针。如果你设置了该指针，那么 unlocked_ioctl
- …（其余 95 处略，详见 `.terminology_audit.json`）

### 调度器（severity=variant，基准出现 565 次）

#### 变体：`调度程序`（4 处，涉及 4 个文件）

- `docs/系统文档/translations/zh_CN/core-api/this_cpu_ops.md:210` — 这些操作不能保证并发中断或抢占。如果在中断上下文中不使用每CPU变量并且调度程序无法
- `docs/系统文档/translations/zh_CN/mm/page_migration.md:28` — 如果调度程序将一个进程重新安置到一个遥远的节点上的处理器，手动迁移是很有用的。批量调度程序
- `docs/系统文档/translations/zh_CN/process/3.Early-stage.md:30` — 调度程序。这个模块被实现并发到linux-kernel邮件列表，在那里它立即遇到了麻烦。
- `docs/系统文档/translations/zh_CN/security/credentials.md:407` — 作，并通知调度程序和其他组件有关更改的情况。

### 调度域（severity=variant，基准出现 106 次）

### 原子操作（severity=variant，基准出现 99 次）

### 能耗感知调度（severity=variant，基准出现 2 次）
