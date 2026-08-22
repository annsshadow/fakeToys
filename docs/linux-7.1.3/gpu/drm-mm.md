## DRM 内存管理

现代 Linux 系统需要大量图形内存来存储帧缓冲、纹理、顶点以及其它图形相关数据鉴于其中许多数据的高度动态特性，高效地管理图形内存对图形栈而言至关重要并在 DRM 基础设施中扮演核心角色
DRM 核心包含两个内存管理器，即转换表管理器（Translation Table Manager，TTM和图形执行管理器（Graphics Execution Manager，GEM）。TTM 是首个被开发的 DRM 内存管理器，
试图成为“一刀切”的解决方案。它提供单一userspace API 以满足所有硬件的需求，
同时支持统一内存架构（Unified Memory Architecture，UMA）设备和带有专用显存
（即大多数独立显卡）的设备。这导致产生了一块庞大而复杂的代码，结果证明对驱动开发而言难以使用
GEM 是作Intel 赞助的项目，为应TTM 的复杂性而生。它的设计哲学完全不同：
GEM 没有为每一个图形内存相关问题提供解决方案，而是识别了驱动间的公共代码并创建了一支持库来共享它。GEM 的初始化与执行要求比 TTM 更简单，但没有显存管理能力，因此仅限UMA 设备
## 转换表管理器（TTM
   :doc: TTM

   :internal:

### TTM 设备对象引用

   :internal:

   :export:

### TTM 资源放置引用

   :internal:

### TTM 资源对象引用

   :internal:

   :export:

### TTM TT 对象引用

   :internal:

   :export:

### TTM 页池引用

   :internal:

   :export:

## 图形执行管理器（GEM
GEM 的设计方法导致了一个这样的内存管理器：它并未在userspace 或内API 提供对所有（甚至所有常见）用例的完整覆盖。GEM userspace 暴露一组标准的与内存相关的操作，并向驱动提供一组辅助函数，同时让驱动用其自身的私有 API
实现硬件特定的操作
GEM userspace API LWN 上的 `GEM - the Graphics Execution
Manager <http://lwn.net/Articles/283798/>`__ 一文中有所描述。尽管略微过时，
该文档仍很好地概述了 GEM API 的原则。作为公GEM API 一部分描述的缓冲分配以读写操作，目前使用驱动特定的 ioctl 实现
GEM 是与数据无关的。它管理抽象的缓冲对象，而无需知晓各个缓冲中包含什么内容因此需要了解缓冲内容或用途的 API（例如缓冲分配或同步原语）因而超出了 GEM 的范畴，
必须使用驱动特定ioctl 来实现
在基本层面上，GEM 涉及若干操作
- 内存分配与释- 命令执行
- 命令执行时的 aperture 管理

缓冲对象分配相对直接，主要由 Linux shmem 层提供，该层为每个对象提供后备内存
设备特定的操作，例如命令执行、固定（pinning）、缓冲读写、映射以及域所有权转移都留给驱动特定的 ioctl 处理
### GEM 初始
使用 GEM 的驱动必须在 struct
`struct drm_driver <drm_driver>` 鐨?driver_features
字段中设DRIVER_GEM 位。随DRM 核心将在调用 load 操作之前自动初始GEM 核心在幕后，这将创建一DRM 内存管理器对象，为对象分配提供一个地址空间池
KMS 配置中，若硬件要求，驱动需要在核心 GEM 初始化之后分配并初始化一个命令环形缓冲UMA 设备通常拥有一个所谓的“被盗（stolen）”内存区域，为初始帧缓冲以及设备所需大块连续内存区域提供空间。该空间通常不由 GEM 管理，必须单独初始化到其自身DRM MM 对象中
### GEM 对象创建

GEM GEM 对象的创建与为其提供后备的内存的分配拆分为两个独立的操作
GEM 对象struct :c:type:`struct
drm_gem_object <drm_gem_object>` 的一个实例表示。驱动通常需用私有信息扩GEM 对象，从而创建一个嵌入了 struct
`struct drm_gem_object <drm_gem_object>` 实例的驱动特GEM 对象结构类型
要创GEM 对象，驱动为其特GEM 对象类型的一个实例分配内存，并用drm_gem_object_init() 的调用来初始化所嵌入struct
`struct drm_gem_object <drm_gem_object>`。该函数接受指向 DRM 设备指向 GEM 对象以及缓冲对象大小（以字节计）的指针
GEM 使用 shmem 来分配匿名的、可分页内存drm_gem_object_init() 将创建所请求大小shmfs 文件，并将其存入 struct :c:type:`struct
drm_gem_object <drm_gem_object>` filp 字段。当图形硬件直接使用系统内存时，
该内存被用作对象的主存储，否则用作后备存储。驱动可以调drm_gem_huge_mnt_create()
来创建、挂载并使用一huge shmem 挂载点，以替代默认的shm_mnt'）。对于启用了
CONFIG_TRANSPARENT_HUGEPAGE 的构建，后续drm_gem_object_init() 的调用将shmem 在可能时分配巨页
驱动负责通过对每个页调用 shmem_read_mapping_page_gfp() 来进行实际的物理页分配注意它们可以决定在初始化 GEM 对象时分配页，也可以延迟分配直到内存被需（例如当用户空间内存访问导致页错误发生，或当驱动需要启动涉及该内存DMA 传输时）
匿名的、可分页内存分配并非总是理想的，例如当硬件要求物理连续的系统内存（在嵌入式设备中常常如此）。驱动可以通过用对 drm_gem_private_object_init() 的调（而非 drm_gem_object_init()）初始化，来创建没有 shmfs 后备（称为私GEM 对象）的
GEM 对象。私GEM 对象的存储必须由驱动管理
### GEM 对象生命周期

所GEM 对象都由 GEM 核心进行引用计数。引用可分别通过调用 drm_gem_object_get() drm_gem_object_put() 来获取和释放
当对 GEM 对象的最后一个引用被释放时，GEM 核心调用
`struct drm_gem_object_funcs <gem_object_funcs>` 鐨?free
操作。该操作对启GEM 的驱动是强制性的，必须释GEM 对象及所有关联资源
void (\**free) (struct drm_gem_object \**obj); 驱动负责
释放所GEM 对象资源。这包括GEM 核心创建的资源，需要用
drm_gem_object_release() 释放
### GEM 对象命名

用户空间与内核之间的通信使用本地句柄（handle）、全局名称（name）或更近期的文件描述来引GEM 对象。这些都32 位整数值；通常Linux 内核限制适用于文件描述符
GEM 句柄是本地于某个 DRM 文件的。应用程序通过驱动特定ioctl 获得 GEM 对象的句柄，
并可在其它标准或驱动特定ioctl 中用该句柄引GEM 对象。关闭一DRM 文件句柄会释其所GEM 句柄并解除对相关 GEM 对象的引用
要为 GEM 对象创建句柄，驱动调drm_gem_handle_create()。该函数接受指向 DRM 文件以及
GEM 对象的指针，并返回一个本地唯一的句柄。当句柄不再需要时，驱动通过调用
drm_gem_handle_delete() 删除它。最后，与一个句柄关联的 GEM 对象可通过调用
drm_gem_object_lookup() 取回
句柄并不拥有 GEM 对象的所有权，它们只是获取对该对象的一个引用，该引用会在句柄被销毁时丢弃为避免泄GEM 对象，驱动必须确保适当地丢弃它们所拥有的引用（例如对象创建时获取的初始引用），
而无需为句柄做任何特殊考虑。例如，dumb_create 操作实现GEM 对象与句柄联合创建的特定情况下，
驱动必须在返回句柄之前丢弃对 GEM 对象的初始引用
GEM 名称在用途上类似句柄，但并非本地DRM 文件。它们可以在进程间传递以全局引用 GEM 对象名称不能直接用于DRM API 中引用对象，应用程序必须使用 DRM_IOCTL_GEM_FLINK DRM_IOCTL_GEM_OPEN ioctl 分别将句柄转换为名称和将名称转换为句柄。该转换DRM 核心处理无需任何驱动特定支持
GEM 也通过 PRIME 支持基于 dma-buf 文件描述符的缓冲共享。基GEM 的驱动必须使用所提供辅助函数才能正确实现导出与导入。参。由于共享文件描述符本质上比易于猜测且全局可见GEM 名称更安全，因此它是首选的缓冲共享机制。通过 GEM 名称共享缓冲仅支持旧式用户空间此外，PRIME 也允许跨设备缓冲共享，因为它基于 dma-buf
### GEM 对象映射

由于映射操作相当重量级，GEM 倾向于通过驱动特定ioctl 实现的、类读写式的缓冲访问而非将缓冲映射到用户空间。然而，当需要随机访问缓冲时（例如执行软件渲染）直接访问对象可能更高效
mmap 系统调用不能直接用于映射 GEM 对象，因为它们没有自己的文件句柄。当前存在两种并存的方法
GEM 对象映射到用户空间。第一种方法使用驱动特定的 ioctl 来执行映射操作，在底层调do_mmap()。这通常被认为是可疑的，似乎对新的启GEM 的驱动不被鼓励，因此此处不作描述
第二种方法在 DRM 文件句柄上使mmap 系统调用。void
\**mmap(void \**addr, size_t length, int prot, int flags, int fd, off_t
offset); DRM 通过经由 mmap offset 参数传入的一个伪偏移（fake offset）来识别要映射的
GEM 对象。因此，在被映射之前，GEM 对象必须被关联一个伪偏移。要做到这一点，驱动必须对该对象
调用 drm_gem_create_mmap_offset()
一旦分配，伪偏移值必须以驱动特定的方式传递给应用程序，随后可用作 mmap offset 参数
GEM 核心提供一个辅助方drm_gem_mmap() 来处理对象映射。该方法可直接设mmap file 操作处理程序。它将基offset 值查GEM 对象，并VMA 操作设为 struct :c:type:`struct drm_driver
<drm_driver>` gem_vm_ops 字段。注drm_gem_mmap() 并不将内存映射到用户空间而是依赖驱动提供的缺页（fault）处理程序来逐页映射
要使drm_gem_mmap()，驱动必须用指向 VM 操作的指针填struct :c:type:`struct drm_driver
<drm_driver>` gem_vm_ops 字段
VM 操作是一`struct vm_operations_struct <vm_operations_struct>`，由若干字段组成，其更有意思的是：

	struct vm_operations_struct {
		void (**open)(struct vm_area_struct ** area);
		void (**close)(struct vm_area_struct ** area);
		vm_fault_t (**fault)(struct vm_fault **vmf);
	};

open close 操作必须更新 GEM 对象引用计数。驱动可以直接使drm_gem_vm_open() drm_gem_vm_close() 辅助函数作为 open close 处理程序
fault 操作处理程序负责在页错误发生时将页映射到用户空间。根据内存分配方案，驱动可以缺页时分配页，也可以决定在对象创建时为其分配内存
希望预先映射 GEM 对象而非处理页错误的驱动，可以实现它们自己的 mmap file 操作处理程序
为减少页表开销，若内部 shmem 挂载"shm_mnt" 被配置为使用透明巨页（对于启用了
CONFIG_TRANSPARENT_HUGEPAGE 的构建），且 shmem 后备存储成功为一个故障地址分配了巨页，
fault 处理程序会先尝试将该巨页插入 VMA，再回退到逐页插入。GEM 对象mmap() 用户地址对齐
通过提供一个转发到 shmem 后备存储的自定义 get_unmapped_area file 操作来处理。对于大多数
默认或通过模块参数未创huge 挂载点的驱动，透明巨页可通过设置 "transparent_hugepage_shmem"
内核参数"/sys/kernel/mm/transparent_hugepage/shmem_enabled" sysfs 旋钮来启用
对于没有 MMU 的平台，GEM 核心提供一个辅助方drm_gem_dma_get_unmapped_area()。mmap() 例程将调用它来获取映射的提议地址
要使drm_gem_dma_get_unmapped_area()，驱动必须用指向
drm_gem_dma_get_unmapped_area() 的指针填struct
`struct file_operations <file_operations>` get_unmapped_area 字段
关于 get_unmapped_area 的更详细信息可在
Documentation/admin-guide/mm/nommu-mmap.rst 中找
### 内存一致
当映射到设备或在命令缓冲中使用时，对象的后备页被刷入内存并标记为 write combined以便GPU 保持一致。同样，如果 CPU GPU 完成对对象的渲染后访问该对象，则必须使对CPU 的内存视图保持一致，通常涉及各类 GPU 缓存刷新。这一核心CPU<->GPU 一致性管由设备特定的 ioctl 提供，它评估对象的当前域，并执行任何必要的刷新或同步以将对象置入
期望的一致性域（注意对象可能处于忙碌状态，即一个活跃的渲染目标；这种情况下，设置域会阻客户端并等待渲染完成，再执行任何必要的刷新操作）
### 命令执行

也许 GEM GPU 设备最重要的功能，是向客户端提供命令执行接口。客户端程序构造包含对先前分配的内对象引用的命令缓冲，然后将它们提交给 GEM。此时，GEM 负责将所有对象绑定到 GTT、执行缓冲，
并在访问同一缓冲的客户端之间提供必要同步。这通常涉及GTT 驱逐某些对象并重新绑定其它对象
（一项相当昂贵的操作），并提供重定位（relocation）支持，从而向客户端隐藏固定的 GTT 偏移客户端必须注意不要提交引用了超过 GTT 所能容纳的对象数量的命令缓冲；否则 GEM 将拒绝它们，
且不会发生任何渲染。类似地，若缓冲中的若干对象需要分fence 寄存器以正确渲染
（例pre-965 芯片上的 2D blits），则必须注意不要让客户端所需 fence 寄存器超过可用的数量此类资源管理应在 libdrm 中对客户端抽象
### GEM 函数引用

   :internal:

   :export:

### GEM DMA 辅助函数引用

   :doc: dma helpers

   :internal:

   :export:

### GEM SHMEM 辅助函数引用

   :doc: overview

   :internal:

   :export:

### GEM VRAM 辅助函数引用

   :doc: overview

   :internal:

   :export:

### GEM TTM 辅助函数引用

   :doc: overview

   :export:

## VMA 鍋忕Щ绠＄悊鍣。
   :doc: vma offset manager

   :internal:

   :export:

## PRIME 缓冲共享

PRIME drm 中的跨设备缓冲共享框架，最初为 OPTIMUS 系列GPU 平台而创建。对用户空间而言PRIME 缓冲是基dma-buf 的文件描述符
### 概述与生命周期规
   :doc: overview and lifetime rules

### PRIME 辅助函数

   :doc: PRIME Helpers

### PRIME 函数引用

   :internal:

   :export:

## DRM MM 区间分配
### 概述

   :doc: Overview

### LRU 扫描/驱逐支
   :doc: lru scan roster

### DRM MM 区间分配器函数引
   :internal:

   :export:

## DRM GPUVM

### 概述

   :doc: Overview

### 拆分与合
   :doc: Split and Merge

### 锁定

   :doc: Locking

### 示例

   :doc: Examples

### DRM GPUVM 函数引用

   :internal:

   :export:

## DRM Buddy 分配
### Buddy 分配器函数引用（GPU buddy
   :export:

### DRM Buddy 特定日志函数引用

   :export:

## DRM 缓存处理与快WC memcpy()

   :export:

## DRM 同步对象

   :doc: Overview

   :internal:

   :export:

## DRM 执行上下
   :doc: Overview

   :internal:

   :export:

## GPU 璋冨害鍣。
### 概述

   :doc: Overview

### 流控

   :doc: Flow Control

### 调度器函数引
   :internal:

   :export:

   :export:
