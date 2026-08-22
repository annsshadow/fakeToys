
## IOMMUFD


:Author: Jason Gunthorpe
:Author: Kevin Tian

## 概述


IOMMUFD 是用户API，用于控制系统中IOMMU 子系统，涉及使用文件描述符从用户空间管理
IO 页表。它旨在通用且可被任何希望向用户空间暴露 DMA 的驱动使用。这些驱动最终预期会弃用
它们可能已经/历史上实现的任何内部 IOMMU 逻辑（例vfio_iommu_type1.c）
至少 iommufd 为所IOMMU 提供管理 I/O 地址空间I/O 页表的通用支持，并在设计中留有空间
以添加非通用特性来迎合特定硬件功能
在此上下文中，大写字母（IOMMUFD）指子系统，而小写字母（iommufd）指通过 /dev/iommu 创建
供用户空间使用的文件描述符
## 关键概念


### 用户可见对象


以下 IOMMUFD 对象暴露给用户空间：

- IOMMUFD_OBJ_IOAS，代表一I/O 地址空间（IOAS），允许将用户空间内存映解除映射  I/O 虚拟地址（IOVA）的范围
  IOAS VFIO 容器的功能性替代，并且VFIO 容器一样，它将一IOVA 映射复制到其中包含的
  iommu_domain 列表中
- IOMMUFD_OBJ_DEVICE，代表一个由外部驱动绑定iommufd 的设备
- IOMMUFD_OBJ_HWPT_PAGING，代表一个由 iommu 驱动管理的实际硬I/O 页表（即单个 struct
  iommu_domain）PAGING" 主要表示这种类型HWPT 应该被链接到一IOAS。它还表示它  带有 __IOMMU_DOMAIN_PAGING 特性标志的 iommu_domain 支撑。这可以是在用户空间运行的设备的
  一UNMANAGED stage-1 域，或者是从客户机级物理地址到主机级物理地址映射的嵌套父 stage-2
  域
  IOAS 有一个共享相IOVA 映射HWPT_PAGING 列表，并且它会将其映射与每个成员 HWPT_PAGING
  同步
- IOMMUFD_OBJ_HWPT_NESTED，代表一个由用户空间（例如客户机 OS）管理的实际硬件 I/O 页表
  （即单个 struct iommu_domain）NESTED" 表示这种类型HWPT 应该被链接到一HWPT_PAGING  它还表示它由类型IOMMU_DOMAIN_NESTED iommu_domain 支撑。这必须是在用户空间运行的设备的
  stage-1 域（例如在启IOMMU 嵌套翻译特性的客户VM 中）。因此，它必须使用给定的嵌套  stage-2 域来创建以进行关联。这个由用户空间管理的嵌stage-1 页表通常具有从客户机I/O
  虚拟地址到客户机级物理地址的映射
- IOMMUFD_FAULT，代表一个用HWPT 通过 IOMMU HW PRI（页请求接口）报IO 页错误的软件
  队列。这个队列对象为用户空间提供一FD 来轮询页错误事件并响应这些事件。必须先创建一  FAULT 对象以获得一fault_id，然后可以通过IOMMU_HWPT_ALLOC 命令flags 字段中设  IOMMU_HWPT_FAULT_ID_VALID 位来分配一个支持故障的 HWPT
- IOMMUFD_OBJ_VIOMMU，代表物IOMMU 实例的一个切片，被传递给 VM 或与 VM 共享。它可能是一  HW 加速的虚拟化特性以及一VM 使用SW 资源。例如：

  - 客户机拥有的 ID 的安全命名空间，例如客户机控制的缓存标签
  - 非设备相关的事件报告，例如失效队列错  - 跨物IOMMU 访问可共享的嵌套父页  - 各种平台 ID 的虚拟化，例RID   - 半虚拟化失效的投  - 直接分配的失效队  - 直接分配的中
  这样vIOMMU 对象通常有权访问一个嵌套父页表，以支持一HW 加速的虚拟化特性。因此，必须
  给定一个嵌套父 HWPT_PAGING 对象来创vIOMMU 对象，然后它会封装该 HWPT_PAGING 对象。因此，
  vIOMMU 对象可以用来分配一HWPT_NESTED 对象，以取代被封装的 HWPT_PAGING
```

     The name "vIOMMU" isn't necessarily identical to a virtualized IOMMU in a
     VM. A VM can have one giant virtualized IOMMU running on a machine having
     multiple physical IOMMUs, in which case the VMM will dispatch the requests
     or configurations from this single virtualized IOMMU instance to multiple
     vIOMMU objects created for individual slices of different physical IOMMUs.
     In other words, a vIOMMU object is always a representation of one physical
     IOMMU, not necessarily of a virtualized IOMMU. For VMMs that want the full
     virtualization features from physical IOMMUs, it is suggested to build the
     same number of virtualized IOMMUs as the number of physical IOMMUs, so the
     passed-through devices would be connected to their own virtualized IOMMUs
     backed by corresponding vIOMMU objects, in which case a guest OS would do
     the "dispatch" naturally instead of VMM trappings.

```
- IOMMUFD_OBJ_VDEVICE，代表一IOMMUFD_OBJ_DEVICE 针对一IOMMUFD_OBJ_VIOMMU 的虚拟设备  这个虚拟设备持有设备的虚拟信息或属性（vIOMMU 相关）于一VM 中。一个直接的 vDATA 例子
  可以是设备在 vIOMMU 上的虚拟 ID，这VMM 为设备分配给 vIOMMU 的翻译通道/端口的唯一 ID  例如 ARM SMMUv3 vSID、AMD IOMMU vDeviceID，以Intel VT-d Context Table vRID  一些高级安全信息的潜在用例也可以通过此对象转发，例如机密计算架构中的安全级别realm
  信息。当 VMM 将设备连接到 vIOMMU 时，它应该创建一vDEVICE 对象来转VM 中的所有设备信息，
  这是一个单独的 ioctl 调用，不同于将同一设备附加vIOMMU 持有HWPT_PAGING
- IOMMUFD_OBJ_VEVENTQ，代表一个用vIOMMU 报告其事件的软件队列，例如发生在嵌套 stage-1   翻译故障（不包括应通过 IOMMUFD_OBJ_FAULT I/O 页错误）以及 HW 特定事件。这个队列对象为
  用户空间提供一FD 来轮读取 vIOMMU 事件。必须先创建一vIOMMU 对象以获得其 viommu_id  然后可用于分配一vEVENTQ。每vIOMMU 可以支持多种类型vEVENTS，但每种 vEVENTQ 类型限制
  为一vEVENTQ
- IOMMUFD_OBJ_HW_QUEUE，代表一个硬件加速队列，作为 IOMMU 虚拟化特性的一部分，供 IOMMU HW 直接
  读取或写入由客户OS 拥有的虚拟队列内存。这HW 加速特性可以允VM 直接IOMMU HW 协作
  而无需 VM 退出，从而减少来自超级调用的开销。连HW QUEUE 对象，iommufd 为用户空间提供一  mmap 接口，供 VMM 将物MMIO 区域从主机物理地址空间映射到客户机物理地址空间，允许客户机 OS
  直接控制已分配的 HW QUEUE。因此，当分配一HW QUEUE 时，VMM 必须请求一mmap 信息
  （offset/length）并精确地通过 offset length 参数传给一mmap 系统调用
所有用户可见对象都通过 IOMMU_DESTROY uAPI 销毁
下面的图表显示了用户可见对象与内核数据结构（iommufd 外部）之间的关系，数字指代操```

  _______________________________________________________________________
 |                      iommufd (HWPT_PAGING only)                       |
 |                                                                       |
 |        [1]                  [3]                                [2]    |
 |  ________________      _____________                        ________  |
 | |                |    |             |                      |        | |
 | |      IOAS      |<---| HWPT_PAGING |<---------------------| DEVICE | |
 | |________________|    |_____________|                      |________| |
 |         |                    |                                  |     |
 |_________|____________________|__________________________________|_____|
           |                    |                                  |
           |              ______v_____                          ___v__
           | PFN storage |  (paging)  |                        |struct|
           |------------>|iommu_domain|<-----------------------|device|
                         |____________|                        |______|

  _______________________________________________________________________
 |                      iommufd (with HWPT_NESTED)                       |
 |                                                                       |
 |        [1]                  [3]                [4]             [2]    |
 |  ________________      _____________      _____________     ________  |
 | |                |    |             |    |             |   |        | |
 | |      IOAS      |<---| HWPT_PAGING |<---| HWPT_NESTED |<--| DEVICE | |
 | |________________|    |_____________|    |_____________|   |________| |
 |         |                    |                  |               |     |
 |_________|____________________|__________________|_______________|_____|
           |                    |                  |               |
           |              ______v_____       ______v_____       ___v__
           | PFN storage |  (paging)  |     |  (nested)  |     |struct|
           |------------>|iommu_domain|<----|iommu_domain|<----|device|
                         |____________|     |____________|     |______|

  _______________________________________________________________________
 |                      iommufd (with vIOMMU/vDEVICE)                    |
 |                                                                       |
 |                             [5]                [6]                    |
 |                        _____________      _____________               |
 |                       |             |    |             |              |
 |      |----------------|    vIOMMU   |<---|   vDEVICE   |<----|        |
 |      |                |             |    |_____________|     |        |
 |      |                |             |                        |        |
 |      |      [1]       |             |          [4]           | [2]    |
 |      |     ______     |             |     _____________     _|______  |
 |      |    |      |    |     [3]     |    |             |   |        | |
 |      |    | IOAS |<---|(HWPT_PAGING)|<---| HWPT_NESTED |<--| DEVICE | |
 |      |    |______|    |_____________|    |_____________|   |________| |
 |      |        |              |                  |               |     |
 |______|________|______________|__________________|_______________|_____|
        |        |              |                  |               |
  ______v_____   |        ______v_____       ______v_____       ___v__
 |   struct   |  |  PFN  |  (paging)  |     |  (nested)  |     |struct|
 |iommu_device|  |------>|iommu_domain|<----|iommu_domain|<----|device|
 |____________|   storage|____________|     |____________|     |______|

```
1. IOMMUFD_OBJ_IOAS 通过 IOMMU_IOAS_ALLOC uAPI 创建。一iommufd 可以持有多个 IOAS 对象   IOAS 是最通用的对象，不暴露特定于单个 IOMMU 驱动的接口。对 IOAS 的所有操作必须在其内部的
   每个 iommu_domain 上平等地进行
2. IOMMUFD_OBJ_DEVICE 在外部驱动调IOMMUFD kAPI 将设备绑定到 iommufd 时创建。该驱动需要实   一ioctl 以允许用户空间发起绑定操作。此操作的成功完成建立了对该设备的期DMA 所有权   该驱动还必须设置 driver_managed_dma 标志，并且在操作成功之前不得触碰该设备
3. IOMMUFD_OBJ_HWPT_PAGING 可以通过两种方式创建
   - IOMMUFD_OBJ_HWPT_PAGING 在外部驱动调IOMMUFD kAPI 将绑定的设备附加IOAS 时自动创建     类似地，外部驱动 uAPI 允许用户空间发起附加操作。如IOAS HWPT_PAGING 列表中存在一     兼容的成HWPT_PAGING 对象，则它会被重用。否则将创建一个代表面向用户空间的
     iommu_domain 的新 HWPT_PAGING，然后添加到列表中。此操作的成功完成建立了 IOAS、设备和
     iommu_domain 之间的链接。一旦完成，设备就可以进DMA
   - IOMMUFD_OBJ_HWPT_PAGING 可以通过 IOMMU_HWPT_ALLOC uAPI 手动创建，通过 @pt_id 提供 ioas_id
     以将新的 HWPT_PAGING 关联到相应的 IOAS 对象。这种手动分配的好处是允许分配标志（定义     enum iommufd_hwpt_alloc_flags），例如，如果设置了 IOMMU_HWPT_ALLOC_NEST_PARENT 标志，它     分配一个嵌套父 HWPT_PAGING
4. IOMMUFD_OBJ_HWPT_NESTED 只能手动通过 IOMMU_HWPT_ALLOC uAPI 创建，通过 @pt_id 提供 hwpt_id    封装了嵌套父 HWPT_PAGING vIOMMU 对象viommu_id，以将新HWPT_NESTED 对象关联到相应的
   HWPT_PAGING 对象。关联的 HWPT_PAGING 对象必须是先前通过同一 uAPI 设置   IOMMU_HWPT_ALLOC_NEST_PARENT 标志手动分配的嵌套父对象，否则分配将失败。该分配将进一步由
   IOMMU 驱动验证，以确保被分配的嵌套父域和嵌套域是兼容的。此操作的成功完成建立了 IOAS、设   iommu_domain 之间的链接。一旦完成，设备就可以通过 2 级翻译（即嵌套翻译）进行 DMA。注意，
   多个 HWPT_NESTED 对象可以由（并随后关联到）同一个嵌套父对象分配
```

      Either a manual IOMMUFD_OBJ_HWPT_PAGING or an IOMMUFD_OBJ_HWPT_NESTED is
      created via the same IOMMU_HWPT_ALLOC uAPI. The difference is at the type
      of the object passed in via the @pt_id field of struct iommufd_hwpt_alloc.

```
5. IOMMUFD_OBJ_VIOMMU 只能手动通过 IOMMU_VIOMMU_ALLOC uAPI 创建，提供一dev_id（用于设备的
   物理 IOMMU 来支撑该 vIOMMU）和一hwpt_id（将 vIOMMU 关联到一个嵌套父 HWPT_PAGING）   iommufd 核心会将 vIOMMU 对象链接到该 struct device 背后struct iommu_device。并IOMMU
   驱动可以实现 viommu_alloc op 来分配它自己vIOMMU 数据结构，内嵌核心级结构 iommufd_viommu
   和一些驱动特定数据。如有必要，驱动还可以为vIOMMU（并因此VM）配置其 HW 虚拟化特性。此
   操作的成功完成建立了 vIOMMU 对象HWPT_PAGING 之间的链接，然后vIOMMU 对象可用作嵌套父
   对象来分配上面描述的 HWPT_NESTED 对象
6. IOMMUFD_OBJ_VDEVICE 只能手动通过 IOMMU_VDEVICE_ALLOC uAPI 创建，提供一iommufd_viommu 对象   viommu_id 和一iommufd_device 对象dev_id。vDEVICE 对象将是这两个父对象之间的绑定。另一   @virt_id 也将通过 uAPI 设置，为 iommufd 核心提供一个索引，以将 vDEVICE 对象存储到每vIOMMU
   vDEVICE 数组中。如有必要，IOMMU 驱动可以选择实现 vdevice_alloc op 来初始化HW 以用于与
   vDEVICE 相关的虚拟化特性。此操作的成功完成建立了 vIOMMU 和设备之间的链接
一个设备只能绑定到一iommufd，这是由DMA 所有权声明，并且最多附加到一IOAS 对象（尚支持 PASID）
### 内核数据结构


用户可见对象由以下数据结构支撑：

- iommufd_ioas 对应 IOMMUFD_OBJ_IOAS- iommufd_device 对应 IOMMUFD_OBJ_DEVICE- iommufd_hwpt_paging 对应 IOMMUFD_OBJ_HWPT_PAGING- iommufd_hwpt_nested 对应 IOMMUFD_OBJ_HWPT_NESTED- iommufd_fault 对应 IOMMUFD_OBJ_FAULT- iommufd_viommu 对应 IOMMUFD_OBJ_VIOMMU- iommufd_vdevice 对应 IOMMUFD_OBJ_VDEVICE- iommufd_veventq 对应 IOMMUFD_OBJ_VEVENTQ- iommufd_hw_queue 对应 IOMMUFD_OBJ_HW_QUEUE
看这些数据结构时的一些术语：

- 自动- 指在将设备附加到 IOAS 对象时自动创建的 iommu 域。这VFIO type1 的语义兼容
- 手动- 指用户指定的、作为设备要附加的目标页表的 iommu 域。虽然目前没uAPI 直接创建这样  域，但数据结构和算法已准备好处理该用例
- 内核内用- 指像 VFIO mdev 这样使用 IOMMUFD access 接口来访IOAS 的东西。这首先创建一  iommufd_access 对象，类似于物理设备绑定域所做的那样。然access 对象将允许将 IOVA 范围
  转换struct page * 列表，或IOVA 进行直接写
iommufd_ioas 作为元数据数据结构，用于管理 IOVA 范围如何映射到内存页，由以下组成
- struct io_pagetable 持有 IOVA 映射
- struct iopt_area 代表 IOVA 已填充的部分
- struct iopt_pages 代表 PFN 的存- struct iommu_domain 代表 IOMMU 中的 IO 页表
- struct iopt_pages_access 代表 PFN 的内核内用户
- struct xarray pinned_pfns 持有由内核内用户固定的页列表

每个 iopt_pages 代表一个完整的 PFN 的逻辑线性数组。PFN 最终通过 mm_struct 从用户空VA 派生一旦它们被固定，PFN 就被存储iommu_domain IOPTE 中，或者如果它们是iommufd_access 固定
的，则存储在 pinned_pfns xarray 中
PFN 必须在存储位置的所有组合之间复制，这取决于存在哪些域以及存在哪些类型的内核软件访问"
用户。该机制确保一个页只被固定一次
一io_pagetable 由指iopt_pages iopt_area 以及镜像 IOVA PFN 映射iommu_domain 列表
组成
多个 io_pagetable（通过iopt_area）可以共享一个单一iopt_pages，这避免了多重固定和页消耗的
重复记账
只要由不同子系统管理的设备绑定到同一iommufd，iommufd_ioas 就可以在子系统之间共享，例如
VFIO 鍜?VDPA銆?
## IOMMUFD 用户 API



## IOMMUFD 内核 API


IOMMUFD kAPI 是以设备为中心的，与组相关的技巧在幕后管理。这使得调用此类 kAPI 的外部驱动能实现一个简单的以设备为中心uAPI，用于将它的设备连接iommufd，而不是像 VFIO 那样在其 uAPI
中显式强加组语义
   :export:

   :export:

### VFIO 鍜?IOMMUFD


VFIO 设备连接iommufd 可以通过两种方式完成
第一种是 VFIO 兼容的方式，通过将这IOCTL 映射io_pagetable 操作来直接实/dev/vfio/vfio 容器 IOCTL。这样做允许通过/dev/vfio/vfio /dev/iommufd 之间建立符号链接或扩VFIO 使用 iommufd 而非容器 fd SET_CONTAINER，从而在遗留 VFIO 应用程序中使iommufd
第二种方式直接扩VFIO 以支持一组新的基于上IOMMUFD 内核 API 的以设备为中心的用户 API。它
需要用户空间更改，但与 IOMMUFD API 语义更匹配，并且与第一种方式相比更容易支持新的 iommufd
特性
目前两种方式仍在进行中
VFIO type1 相比仍有一些差距需要解决，iommufd_vfio_check_extension() 中所述
## 未来TODO


目前 IOMMUFD 仅支持内核管理的 I/O 页表，类似于 VFIO type1。雷达上的新特性包括：

 - iommu_domain 绑定PASID/SSID
 - 用户空间页表，针ARM、x86 S390
 - 内核旁路的用户页表失 - IOMMU 中复KVM 页表
 - IOMMU 中的脏页跟踪
 - IOPTE 大小的运行时增加/减少
 - 在用户空间解决故障的 PRI 支持
