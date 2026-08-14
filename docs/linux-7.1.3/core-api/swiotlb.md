
## DMA 与 swiotlb


swiotlb 是 Linux 内核 DMA 层使用的一块内存缓冲区分配器。当进行 DMA 的设备由于硬件
限制或其它要求而无法直接访问目标内存缓冲区时，通常会使用它。在这种情况下，DMA 层
调用 swiotlb 来分配一块符合这些限制的临时内存缓冲区。DMA 在该临时内存缓冲区与目标
内存缓冲区之间进行，CPU 在临时缓冲区和原始目标内存缓冲区之间复制数据。这种方式一般
称为 “bounce buffering（反弹缓冲）”，该临时内存缓冲区称为 “bounce buffer（反弹
缓冲区）”。

设备驱动不直接与 swiotlb 交互。相反，驱动向 DMA 层告知其管理的设备的 DMA 属性，并在
编程设备执行 DMA 时使用正常的 DMA 映射（map）、解除映射（unmap）和同步（sync）API。
这些 API 使用设备 DMA 属性和内核范围的设置来判断是否需要进行反弹缓冲。如果需要，DMA
层负责管理反弹缓冲区的分配、释放和同步。由于 DMA 属性是每设备的，系统中某些设备可能
使用反弹缓冲，而其它设备则不使用。

由于 CPU 在反弹缓冲区和原始目标内存缓冲区之间复制数据，进行反弹缓冲比直接对原始内存
缓冲区进行 DMA 更慢，并且会消耗更多的 CPU 资源。因此它仅在为了提供 DMA 功能而有必要
时才被使用。

### 使用场景


swiotlb 最初是为处理具有寻址限制的设备的 DMA 而创建的。随着物理内存大小增长到超过
4 GiB，某些设备只能提供 32 位 DMA 地址。通过在 4 GiB 线以下分配反弹缓冲区内存，这些
具有寻址限制的设备仍然可以工作并进行 DMA。

更近一些，机密计算（Confidential Computing, CoCo）虚拟机会默认对客户机（guest）VM 的
内存进行加密，并且该内存无法被宿主机（host）的 hypervisor 和 VMM 访问。为了让宿主机
代表客户机进行 I/O，I/O 必须被定向到未加密的客户机内存。CoCo 虚拟机设置一个内核范围的
选项，强制所有 DMA I/O 使用反弹缓冲区，并且反弹缓冲区内存被设置为未加密的。宿主机对
反弹缓冲区内存进行 DMA I/O，Linux 内核 DMA 层执行 “sync” 操作，使 CPU 将数据复制
到/自原始目标内存缓冲区。CPU 复制在未加密内存和加密内存之间架起桥梁。这种反弹缓冲的
使用使得设备驱动能够在 CoCo 虚拟机中 “开箱即用”，无需为处理内存加密的复杂性做任何
修改。

反弹缓冲区还会出现其它边缘场景。例如，当为来自/发往一个被视为 “不可信（untrusted）”
的设备的 DMA 操作设置 IOMMU 映射时，应当只给予该设备对包含所传输数据的那部分内存的
访问权。但如果该内存只占用了 IOMMU 颗粒（granule）的一部分，那么该颗粒的其它部分可能
包含无关的内核数据。由于 IOMMU 访问控制是每颗粒的，不可信设备可能获得对这些无关内核
数据的访问权。这个问题通过对 DMA 操作进行反弹缓冲、并确保反弹缓冲区的未使用部分不
包含任何无关内核数据来解决。

### 核心功能


swiotlb 的主要 API 是 swiotlb_tbl_map_single() 和 swiotlb_tbl_unmap_single()。“map”
API 分配指定字节大小的反弹缓冲区，并返回该缓冲区的物理地址。该缓冲区内存在物理上是
连续的。预期 DMA 层将物理内存地址映射为 DMA 地址，并将 DMA 地址返回给驱动以编程写入
设备。如果一个 DMA 操作指定了多个内存缓冲区段（segment），则必须为每个段分别分配一个
反弹缓冲区。swiotlb_tbl_map_single() 总是执行一次 “sync” 操作（即一次 CPU 复制），
以初始化反弹缓冲区，使其内容与原始缓冲区相匹配。

swiotlb_tbl_unmap_single() 执行相反的操作。如果 DMA 操作可能已经更新了反弹缓冲区内存
且未设置 DMA_ATTR_SKIP_CPU_SYNC，那么该解除映射会执行一次 “sync” 操作，使 CPU 将
数据从反弹缓冲区复制回原始缓冲区。然后释放反弹缓冲区内存。

swiotlb 还提供与 dma_sync_*() API 对应的 “sync” API，当缓冲区的控制权在 CPU 与设备
之间转移时，驱动可能会用到这些 API。swiotlb 的 “sync” API 会使 CPU 在原始缓冲区与
反弹缓冲区之间复制数据。与 dma_sync_*() API 一样，swiotlb 的 “sync” API 支持执行
部分同步（partial sync），即只将反弹缓冲区的一个子集复制到原始缓冲区或从原始缓冲区
复制出来。

### 核心功能约束


swiotlb 的 map/unmap/sync API 必须在不阻塞的情况下运行，因为它们由相应的 DMA API 调用，
而这些 DMA API 可能在无法阻塞的上下文中运行。因此 swiotlb 分配的默认内存池必须在启动
时预分配（但见下文的动态 swiotlb）。由于 swiotlb 分配在物理上必须是连续的，整个默认
内存池被作为一整块连续的内存块分配。

需要预分配默认 swiotlb 池带来了一个启动时的权衡。该池应当足够大，以确保反弹缓冲区
请求总是能够满足，因为不阻塞的要求意味着请求无法等待空间变得可用。但一个大的内存池
可能会浪费内存，因为这块预分配的内存无法用于系统中其他用途。在使用反弹缓冲区处理所有
DMA I/O 的 CoCo 虚拟机中，这种权衡尤为突出。这些虚拟机使用一个启发式方法将默认池大小
设置为内存的约 6%，最大为 1 GiB，这有可能非常浪费内存。反之，根据虚拟机中工作负载的
I/O 模式，该启发式方法可能产生不足的大小。下文描述的动态 swiotlb 特性可以提供帮助，
但存在限制。更好地管理 swiotlb 默认内存池大小仍然是一个未解决的问题。

swiotlb 的单个分配被限制为 IO_TLB_SIZE * IO_TLB_SEGSIZE 字节，按当前定义为 256 KiB。
当设备的 DMA 设置使得该设备可能使用 swiotlb 时，DMA 段的最大大小必须限制在该 256 KiB
以内。该值通过 dma_map_mapping_size() 和 swiotlb_max_mapping_size() 传递给更高级别的
内核代码。如果高级代码未能考虑此限制，它可能发出对于 swiotlb 来说过大的请求，并得到
“swiotlb full” 错误。

一个关键的设备 DMA 设置是 “min_align_mask”，它是 2 的幂减 1，因此会设置一定数量的
低阶位，或者它也可能为零。swiotlb 分配确保反弹缓冲区物理地址的 min_align_mask 这些位
与原始缓冲区地址中的相同位相匹配。当 min_align_mask 非零时，它可能在反弹缓冲区地址中
产生一个 “对齐偏移（alignment offset）”，从而略微减小分配的最大大小。这种潜在的
对齐偏移反映在 swiotlb_max_mapping_size() 返回的值中，它会出现在类似
/sys/block/<device>/queue/max_sectors_kb 这样的地方。例如，如果设备不使用 swiotlb，
max_sectors_kb 可能是 512 KiB 或更大。如果设备可能使用 swiotlb，max_sectors_kb 将是
256 KiB。当 min_align_mask 非零时，max_sectors_kb 可能更小，例如 252 KiB。

swiotlb_tbl_map_single() 还接受一个 “alloc_align_mask” 参数。该参数指定反弹缓冲区空间
的分配必须从一个 alloc_align_mask 位被设置为零的物理地址开始。但如果 min_align_mask
非零，实际的反弹缓冲区可能从一个更大的地址开始。因此可能存在在反弹缓冲区开始之前分配的
前导填充（pre-padding）空间。类似地，反弹缓冲区的末尾会被向上舍入到 alloc_align_mask
边界，可能导致尾部填充（post-padding）空间。任何前导或尾部填充空间都不会被 swiotlb
代码初始化。“alloc_align_mask” 参数由 IOMMU 代码在为非可信设备映射时使用。它被设置为
颗粒大小减 1，以使反弹缓冲区完全从不用于任何其它用途的颗粒中分配。

### 数据结构概念


用于 swiotlb 反弹缓冲区的内存从一个或多个 “池（pool）” 的整体系统内存中分配。默认池
在系统启动期间分配，默认大小为 64 MiB。默认池大小可以通过 “swiotlb=” 内核启动命令行
参数修改。默认大小也可能由于其它条件而调整，例如在 CoCo 虚拟机中运行，如前所述。如果
启用了 CONFIG_SWIOTLB_DYNAMIC，则可以在系统运行的后期分配额外的池。每个池必须是连续的
物理内存范围。默认池分配在 4 GiB 物理地址线以下，以便它适用于只能寻址 32 位物理内存
的设备（除非特定架构的代码提供了 SWIOTLB_ANY 标志）。在 CoCo 虚拟机中，该池内存必须在
swiotlb 使用之前被解密。

每个池被划分为大小为 IO_TLB_SIZE 的 “槽（slot）”，按当前定义为 2 KiB。IO_TLB_SEGSIZE
个连续的槽（128 个槽）构成所谓的一个 “槽集（slot set）”。当分配一个反弹缓冲区时，它
占用一个或多个连续的槽。一个槽永远不会被多个反弹缓冲区共享。此外，一个反弹缓冲区必须
从单个槽集中分配，这就导致反弹缓冲区的最大大小为 IO_TLB_SIZE * IO_TLB_SEGSIZE。如果
对齐和大小约束能够满足，多个较小的反弹缓冲区可以在一个槽集中共存。

槽还被分组为 “区域（area）”，约束是每个槽集完全存在于单个区域中。每个区域有它自己的
自旋锁（spin lock），操作该区域中的槽时必须持有它。划分成区域避免了在 swiotlb 被大量
使用（例如在 CoCo 虚拟机中）时争用单个全局自旋锁。区域数量默认为系统中的 CPU 数量以
获得最大并行度，但由于一个区域不能小于 IO_TLB_SEGSIZE 个槽，可能有必要将多个 CPU 分配
到同一个区域。区域数量也可以通过 “swiotlb=” 内核启动参数设置。

当分配一个反弹缓冲区时，如果与调用 CPU 关联的区域没有足够的空闲空间，则会按顺序尝试
与其它 CPU 关联的区域。对于尝试的每个区域，在尝试分配之前必须获取该区域的自旋锁，因此
如果 swiotlb 总体上相对繁忙，可能会发生争用。但除非所有区域都没有足够的空闲空间，否则
分配请求不会失败。

IO_TLB_SIZE、IO_TLB_SEGSIZE 和区域数量都必须是 2 的幂，因为代码使用移位和位掩码来执行
许多计算。区域数量在必要时会被向上舍入到 2 的幂以满足这一要求。

默认池以 PAGE_SIZE 对齐分配。如果 swiotlb_tbl_map_single() 的 alloc_align_mask 参数
指定了更大的对齐，则每个槽集中的一或多个初始槽可能不满足 alloc_align_mask 标准。由于
反弹缓冲区分配不能跨越槽集边界，消除那些初始槽会有效地减小反弹缓冲区的最大大小。目前
没有问题，因为 alloc_align_mask 是基于 IOMMU 颗粒大小设置的，而颗粒不可能大于
PAGE_SIZE。但如果将来这种情况发生改变，初始池分配可能需要以大于 PAGE_SIZE 的对齐来
进行。

### 动态 swiotlb


当启用了 CONFIG_SWIOTLB_DYNAMIC 时，swiotlb 可以按需扩展可用于分配反弹缓冲区的内存
数量。如果由于可用空间不足而导致反弹缓冲区请求失败，会启动一个异步后台任务，从通用系统
内存中分配内存并将其转变为一个 swiotlb 池。创建一个额外的池必须是异步的，因为内存分配
可能会阻塞，而如上所述，swiotlb 请求是不允许阻塞的。一旦后台任务被启动，反弹缓冲区请求
会创建一个 “瞬态池（transient pool）” 以避免返回 “swiotlb full” 错误。瞬态池具有反弹
缓冲区请求的大小，并在反弹缓冲区被释放时删除。该瞬态池的内存来自通用系统内存的原子池
（atomic pool），因此其创建不会阻塞。创建一个瞬态池的成本相对较高，特别是在必须将内存
解密的 CoCo 虚拟机中，因此它仅作为一个权宜之计，直到后台任务可以添加另一个非瞬态池。

添加一个动态池存在限制。与默认池一样，内存必须在物理上连续，因此大小被限制为
MAX_PAGE_ORDER 页（例如，在典型 x86 系统上为 4 MiB）。由于内存碎片，可能拿不到最大大小
的分配。动态池分配器会尝试更小的尺寸直到成功，但最小尺寸为 1 MiB。鉴于足够严重的系统
内存碎片，动态添加池可能根本不会成功。

动态池中的区域数量可能与默认池中的区域数量不同。因为新池的大小通常最多只有几 MiB，区域
数量可能会更小。例如，新池大小为 4 MiB 且最小区域大小为 256 KiB 时，只能创建 16 个区域。
如果系统有超过 16 个 CPU，多个 CPU 必须共享一个区域，从而产生更多的锁争用。

通过动态 swiotlb 添加的新池在一个线性列表中链接在一起。swiotlb 代码经常必须搜索包含
特定 swiotlb 物理地址的池，因此该搜索是线性的，在存在大量动态池时性能不佳。数据结构可以
改进以加快搜索。

总体而言，动态 swiotlb 在 CPU 相对较少的小型配置中效果最好。它允许默认 swiotlb 池更小，
从而不浪费内存，而在需要时可动态添加池来提供更多空间（只要碎片不是障碍）。它对大型 CoCo
虚拟机的用处较小。

### 数据结构细节


swiotlb 由四个主要数据结构管理：io_tlb_mem、io_tlb_pool、io_tlb_area 和 io_tlb_slot。
io_tlb_mem 描述一个 swiotlb 内存分配器，它包括默认内存池以及链接到它的任何动态或瞬态
池。关于 swiotlb 用量的有限统计信息按内存分配器保存，并存储在该数据结构中。当设置了
CONFIG_DEBUG_FS 时，这些统计信息在 /sys/kernel/debug/swiotlb 下可用。

io_tlb_pool 描述一个内存池，可以是默认池、动态池或瞬态池。该描述包括池中内存的起始和
结束地址、一个指向 io_tlb_area 结构数组的指针，以及一个指向与该池关联的 io_tlb_slot
结构数组的指针。

io_tlb_area 描述一个区域。主要字段是用于序列化访问该区域中槽的自旋锁。一个池的
io_tlb_area 数组对每一个区域都有一个条目，并使用从调用处理器 ID 派生的从零开始的
区域索引来访问。区域的存在纯粹是为了允许从多个 CPU 并行访问 swiotlb。

io_tlb_slot 描述池中的一个单独内存槽，大小为 IO_TLB_SIZE（当前为 2 KiB）。io_tlb_slot
数组由根据反弹缓冲区地址相对于池起始内存地址计算得出的槽索引来索引。struct io_tlb_slot
的大小为 24 字节，因此开销约为槽大小的 1%。

io_tlb_slot 数组的设计旨在满足几个要求。首先，DMA API 和相应的 swiotlb API 使用反弹
缓冲区地址作为反弹缓冲区的标识符。该地址由 swiotlb_tbl_map_single() 返回，然后作为参数
传递给 swiotlb_tbl_unmap_single() 和 swiotlb_sync_*() 函数。原始内存缓冲区地址显然必须
作为参数传递给 swiotlb_tbl_map_single()，但它不会被传递给其它 API。因此，swiotlb 数据
结构必须保存原始内存缓冲区地址，以便在执行同步操作时使用。这个原始地址保存在 io_tlb_slot
数组中。

其次，io_tlb_slot 数组必须处理部分同步请求。在这种情况下，swiotlb_sync_*() 的参数不是
反弹缓冲区起始处的地址，而是反弹缓冲区中间某处的地址，而反弹缓冲区起始处的地址是 swiotlb
代码所不知道的。但 swiotlb 代码必须能够计算对应的原始内存缓冲区地址，以执行 “sync” 所
要求的 CPU 复制。因此，一个调整后的原始内存缓冲区地址被填充到反弹缓冲区占用的每个
struct io_tlb_slot 中。一个调整后的反弹缓冲区 “alloc_size” 也被记录在每个 struct
io_tlb_slot 中，以便可以对 “sync” 操作的大小执行完整性检查（sanity check）。“alloc_size”
字段仅用于该完整性检查。

第三，io_tlb_slot 数组用于跟踪可用槽。struct io_tlb_slot 中的 “list” 字段记录从该槽
开始存在多少个连续的可用槽。0 表示该槽被占用。1 表示只有当前槽可用。2 表示当前槽和下一个
槽可用，依此类推。最大值为 IO_TLB_SEGSIZE，它可以出现在槽集中的第一个槽中，表示整个槽集
都可用。这些值在搜索可用于新反弹缓冲区的可用槽时使用。它们会在分配新反弹缓冲区和释放反弹
缓冲区时更新。在创建池时，“list” 字段被初始化为每个槽集中的槽从 IO_TLB_SEGSIZE 递减到 1。

第四，io_tlb_slot 数组跟踪为满足上述 alloc_align_mask 要求而分配的任何 “填充槽
（padding slots）”。当 swiotlb_tbl_map_single() 分配反弹缓冲区空间以满足 alloc_align_mask
要求时，它可能跨零个或多个槽分配前导填充空间。但当调用 swiotlb_tbl_unmap_single() 并带有
反弹缓冲区地址时，控制该分配（以及因此任何填充槽的分配）的 alloc_align_mask 值是未知的。
“pad_slots” 字段记录填充槽的数量，以便 swiotlb_tbl_unmap_single() 能够释放它们。“pad_slots”
值仅记录在分配给反弹缓冲区的第一个非填充槽中。

### 受限池（Restricted pools）


swiotlb 机制也被用于 “受限池（restricted pools）”，它们是独立于默认 swiotlb 池的内存池，
专用于特定设备进行 DMA 使用。受限池在具有有限硬件保护能力（例如缺少 IOMMU）的系统上提供
一定程度的 DMA 内存保护。这种用法由 DeviceTree 条目指定，并要求设置
CONFIG_DMA_RESTRICTED_POOL。每个受限池基于它自己的 io_tlb_mem 数据结构，独立于主 swiotlb
的 io_tlb_mem。

受限池添加了 swiotlb_alloc() 和 swiotlb_free() API，它们从 dma_alloc_**() 和
dma_free_**() API 调用。swiotlb_alloc/free() API 直接从受限池分配/释放槽，而不经过
swiotlb_tbl_map/unmap_single()。
