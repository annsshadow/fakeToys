## 缓冲区共享与同步（dma-buf

dma-buf 子系统提供了一个框架，用于跨多个设备驱动和子系统共享供硬件（DMA）访问的缓冲区，并对异步硬件访问进行同步
举例来说，DRM 子系统广泛地使用它，用于在进程之间、上下文之间、同一进程内的API 之间交换缓冲区，并且也与 V4L2 等其他子系统交换缓冲区
本文档描述了内核子系统可以使dma-buf 所提供的三个主要原语并与之交互的方式：

 - dma-buf，表示一sg_table，并作为文件描述符暴露给用户空间，以便能够在进程、子系统、设备等之间传递；
 - dma-fence，提供一种机制用于在异步硬件操作完成时发出信号；以及
 - dma-resv，它管理某个特定 dma-buf 的一dma-fence，从而允许对工作任务进行隐式（由内核排序的）同步，以维持一致访问的假象

### 用户空间 API 原则与使

关于如何为你的子系统设计用于 dma-buf API 的更多细节，请参Documentation/userspace-api/dma-buf-alloc-exchange.rst

### 共享DMA 缓冲

本文档作为面向设备驱动编写者的指南，介dma-buf 缓冲区共API 是什么，以及如何使用它来导出和使用共享缓冲区
任何希望成为 DMA 缓冲区共享一部分的设备驱动，既可以作为缓冲区的“导出者（exporter）”，也可以作为缓冲区的“使用者（user）”或“导入者（importer）”
假设驱动 A 想要使用由驱B 创建的缓冲区，那么我们称 B 为导出者，A 为缓冲区使用导入者
瀵煎嚭鑰。
 - :c:type:`struct dma_buf_ops <dma_buf_ops>` 中为缓冲区实现并管理操作 - 允许其他使用者通过 dma_buf 共享 API 来共享该缓冲区，
 - 管理缓冲区分配的细节，封装在一:c:type:`struct dma_buf <dma_buf>` 中，
 - 决定此次分配实际发生在哪里的后备存储（backing storage），
 - 并负责任scatterlist 的迁移——针对该缓冲区的所有（共享的）使用者
缓冲区使用
 - 是该缓冲区诸多（众多）共享使用者之一 - 无需关心缓冲区是如何分配的，或在哪里分配的 - 并且需要一种机制来获取构成该缓冲区在内存中scatterlist，并将其映射到自己的地址空间，从而能够访问同一片内存区域。该接口:c:type:`struct dma_buf_attachment <dma_buf_attachment>` 提供
任何 dma-buf 缓冲区共享框架的导出者或使用者都必须在各自的 Kconfig 中包'select DMA_SHARED_BUFFER'

#### 用户空间接口注意事项


在多数情况下，DMA 缓冲区文件描述符对用户空间而言只是一个不透明的对象，因此所暴露出的通用接口非常精简。不过仍有几点需要考虑
- 自内3.12 起，dma-buf 文件描述符支llseek 系统调用，但仅支offset=0 以及 whence=SEEK_END|SEEK_SET。支SEEK_SET 是为了允许常见的获取大小模式 size = SEEK_END(0); SEEK_SET(0)。所有其llseek 操作都将返回 -EINVAL
  如果 dma-buf 文件描述符不支持 llseek，内核在所有情况下都会返回 -ESPIPE。用户空间可以利用这一点来检测是否支持通过 llseek 发现 dma-buf 的大小
- 为了避免exec 时发fd 泄漏，必须在文件描述符上设置 FD_CLOEXEC 标志。这不仅是一个资源泄漏问题，更是一个潜在的安全漏洞。它可能会让exec 的应用程序通过泄漏fd 访问那些本不应被允许访问的缓冲区
  通过单独fcntl() 调用（相对于fd 创建时原子地完成）来做这件事的问题在于，在一个多线程应用中这本质上存在竞争[^3^]。当是库代码打开/创建文件描述符时，问题会更加严重，因为应用程序甚至可能意识不到这fd 的存在
  为避免此问题，用户空间必须有一种方式，能够在创dma-buf fd 时请求设O_CLOEXEC 标志。因此，任何由导出驱动提供的用于创建 dmabuf fd API 都必须提供一种方式，让用户空间控制传dma_buf_fd() O_CLOEXEC 标志的设置
- DMA 缓冲区内容的存储器映射也是受支持的。完整细节请参阅下文 `CPU Access to DMA Buffer Objects`_ 的讨论
- DMA 缓冲fd 也是可轮询（pollable）的，详情见下文 `Implicit Fence Poll Support`_
- DMA 缓冲fd 还支持一dma-buf 专用ioctl，详情见下文 `DMA Buffer ioctls`_

#### 基本操作与设DMA 访问


   :doc: dma buf device access


#### CPU DMA 缓冲区对象的访问


   :doc: cpu access


#### 隐式 Fence 轮询支持


   :doc: implicit fence polling


#### DMA 缓冲ioctls





#### DMA-BUF 加锁约定


   :doc: locking convention


#### 内核函数与结构参

   :export:

   :internal:


### 预留对象（Reservation Objects

   :doc: Reservation Object Overview

   :export:

   :internal:


### DMA Fence


   :doc: DMA fences overview


#### DMA Fence 跨驱动契

   :doc: fence cross-driver contract


#### DMA Fence 信号标注


   :doc: fence signalling annotation


#### DMA Fence 截止时间提示


   :doc: deadline hints


#### DMA Fence 函数参

   :export:

   :internal:


#### DMA Fence 数组


   :export:

   :internal:


#### DMA Fence 閾。

   :export:

   :internal:


#### DMA Fence 解包（unwrap

   :internal:


#### DMA Fence 同步文件


   :export:

   :internal:


#### DMA Fence 同步文件 uABI


   :internal:


#### 鏃犻檺鏈?DMA Fence


在某些时候，有人提出了带有“在 dma_fence_wait() 完成之前无限期时间”的 struct dma_fence。例子包括：

- Future fence，在 HWC1 中用于在一个缓冲区不再被显示屏使用时发出信号，并且随使该缓冲区可见的屏幕更新一起创建。该 fence 完成的时间完全由用户空间控制
- Proxy fence，被提议用于处理尚未设置 fence &drm_syncobj。用于异步地延迟命令提交
- 用户空间 fence gpu futex，即命令缓冲区中由用户空间用于跨引擎或与 CPU 进行同步的细粒度锁，随后被导入为 DMA fence，以集成到现有的 winsys 协议中
- 长时间运行的 compute 命令缓冲区，同时仍使用传统的批处理结束（end of batch）DMA fence 进行内存管理，而不是使用在 compute 任务被重新调度时会被重新附加的上下文抢占（context preemption）DMA fence
所有这些方案的共同点是，用户空间控制这fence 的依赖关系，并控制它们何时触发。将无限fence 与正常的、内核内DMA fence 混用是行不通的，即便包含一个用于防范恶意用户空间的回退超时也不行：

- 只有内核知道所有的 DMA fence 依赖关系，用户空间并不知道因内存管理或调度器决策而注入的依赖关系
- 只有用户空间知道无限fence 中的所有依赖关系以及它们确切的完成时间，内核对此没有可见性
此外，内核必须能够为了内存管理的需要而拖住用户空间的命令提交，这意味着我们必须支持“依赖于 DMA fence 的无限期 fence”。如果内核也像上述任何一种提案那样，在核心中DMA fence 一样支持无限期 fence，就有可能产生死锁
   :alt: Indefinite Fencing Dependency Cycle
   :caption: Indefinite Fencing Dependency Cycle

   digraph "Fencing Cycle" {
      node [shape=box bgcolor=grey style=filled]
      kernel [label="Kernel DMA Fences"]
      userspace [label="userspace controlled fences"]
      kernel -> userspace [label="memory management"]
      userspace -> kernel [label="Future fence, fence proxy, ..."]

      { rank=same; kernel userspace }
   }

这意味着内核可能会由于用户空间未意识到的内存管理依赖关系，而意外地制造出死锁，从而随机地挂起工作负载，直到超时生效。从用户空间的角度看，这些工作负载并不包含死锁。在这种混合fencing 架构中，没有任何单一实体掌握所有的依赖关系。因此，从内核内部防止此类死锁是不可能的
避免依赖环的唯一解决办法是不允许无限fence 进入内核。这意味着
- 不能future fence、proxy fence 或用户空fence 作为 DMA fence 导入，不论带不带超时
- 不能将那些“用户空间被允许使用用户空间 fencing 或长时间运行compute 工作负载”的命令提交中、标志着批处理结束的 DMA fence，因为这种情况下也不能进行隐fencing
#### 可恢复硬件页错误（Recoverable Hardware Page Faults）的影响


现代硬件支持可恢复的页错误，这对 DMA fence 有很多影响
首先，一个待处理的页错误显然会拖住加速器上正在运行的工作，而解决该错误通常需要一次内存分配。但是内存分配是不允许用来阻DMA fence 的完成的，这意味着任何使用可恢复页错误的工作负载都不能使用 DMA fence 进行同步。必须改用由用户空间控制的同fence
GPU 上，这带来一个问题，因为 Linux 上当前的桌面合成器（compositor）协议依DMA fence，这意味着如果没有一个完全建立在用户空间 fence 之上的全新用户空间栈，它们就无法从可恢复页错误中受益。具体来说，这意味着隐式同步将不可行。唯一的例外是当页错误仅被用作迁移提示，而从不用于按需填充内存请求时。目前这意味着 GPU 上的可恢复页错误仅限于纯计算工作负载
此外，GPU 通常3D 渲染compute 侧之间共享资源，例如计算单元或命令提交引擎。如果同时有一个带DMA fence 3D 工作负载和一个使用可恢复页错误的 compute 工作负载处于待处理状态，它们就可能死锁：

- 3D 工作负载可能需要等compute 任务先完成并释放硬件资源
- compute 工作负载可能卡在一个页错误中，因为内存分配正在等待 3D 工作负载DMA fence 完成
有几种防止该问题的选择，其中之一是驱动必须确保的
- compute 工作负载必须始终可以被抢占，即使页错误处于待处理且尚未修复时也是如此。并非所有硬件都支持这一点
- DMA fence 工作负载与需要进行页错误处理的负载拥有相互独立的硬件资源，以保证向前推进。这可以通过例如专用的引擎以及为 DMA fence 工作负载保留最小的计算单元来实现
- 预留方案还可以进一步细化，即仅DMA fence 工作负载处于在途（in-flight）状态时才为其预留硬件资源。这必须覆盖DMA fence 对其他线程可见，到通过 dma_fence_signal() 完成fence 的这段时间
- 作为最后手段，如果硬件不提供有用的预留机制，则在切换到需DMA fence 的任务与需要页错误处理的任务之间时，必须将所有工作负载从 GPU 中刷出：这意味着在插入一个需要页错误处理compute 任务到调度器队列之前，所DMA fence 必须已完成。反之亦然，DMA fence 能够在系统中的任何地方可见之前，所compute 工作负载必须被抢占，以保证所有待处理GPU 页错误都被刷出
- 只有一个相当理论性的选择，即在分配内存来修复硬件页错误时解开这些依赖关系，要么通过独立的内存块，要么通过运行时跟踪所DMA fence 的完整依赖图。这会对内核产生非常广泛的影响，因为CPU 侧解决页错误本身也可能涉及一次页错误。将处理硬件页错误的影响限制在特定的驱动范围内，要可行和健壮得多
注意，运行在独立硬件（如拷贝引擎或其GPU）上的工作负载没有任何影响。这允许我们在内核内部即使为了修复硬件页错误也继续使DMA fence，例如通过使用拷贝引擎来清空或复制解决页错误所需的内存
在某些方面，这个页错误问题是 `Infinite DMA Fences` 讨论的一个特例：来自 compute 工作负载的无fence 被允许依赖于 DMA fence，但反过来不行。而且页错误问题也并不新鲜，因为用户空间中的某个其CPU 线程可能会遇到一个页错误，从而拖住一个用户空fence——支GPU 上的页错误并没有带来任何根本性的新东西