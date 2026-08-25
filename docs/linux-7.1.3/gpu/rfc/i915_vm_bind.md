## I915 VM_BIND 特性设计与用例

## VM_BIND 特
DRM_I915_GEM_VM_BIND/UNBIND ioctl 允许用户态驱动（UMD）在指定的地址空间（VM）上、指定的 GPU
虚拟地址处绑解绑 GEM 缓冲区对象（BO）或 BO 的某个区段。这些映射（也称为持久映射）会在 UMD
发出的多GPU 提交（execbuf 调用）之间保持持久，而无需用户在每次提交时都提供所有所需映射列表（旧execbuf 模式要求如此）
VM_BIND/UNBIND 调用允许 UMD 请求一个时间线出站围栏（out fence），用于通知绑定/解绑操作的完成
VM_BIND 特性通过 I915_PARAM_VM_BIND_VERSION 向用户通告。用户必须在 VM 创建时通过
I915_VM_CREATE_FLAGS_USE_VM_BIND 扩展显式选择采用 VM_BIND 绑定模式
在不CPU 线程上并发执行的 VM_BIND/UNBIND ioctl 调用是无序的。此外，当指定了有效的出围栏时，VM_BIND/UNBIND 操作的部分可以异步完成
VM_BIND 特性包括：

- 多个虚拟地址（VA）映射可以映射到对象的同一物理页（别名化，aliasing）- VA 映射可以映射BO 的一个部分区段（部分绑定，partial binding）- 支持GPU 出错时将持久映射捕获到转储中- 支持 userptr gem 对象（这不需要特殊的 uapi）
### TLB 刷新考虑

i915 驱动会在每次提交时以及对象的页面被释放时刷新 TLB。VM_BIND/UNBIND 操作不会进行任何额外
TLB 刷新。所添加的任VM_BIND 映射都将成为VM 后续提交的工作集的一部分，而不会处当前正在运行的批处理的工作集中（那将需要额外的 TLB 刷新，而这是不支持的）
### VM_BIND 模式下的 Execbuf ioctl

处于 VM_BIND 模式VM 将不支持旧的 execbuf 绑定模式。VM_BIND 模式下的 execbuf ioctl 处理旧的 execbuf2 ioctl（见 struct drm_i915_gem_execbuffer2）有显著不同。因此，新增execbuf3
ioctl 以支VM_BIND 模式（见 struct drm_i915_gem_execbuffer3）。execbuf3 ioctl 不接受任execlist，因此不支持隐式同步。期望下述工作能够支持所有用例中对象依赖设置的需求：

"dma-buf: Add an API for exporting sync files"
(https://lwn.net/Articles/859290/)

新的 execbuf3 ioctl 仅在 VM_BIND 模式下工作，VM_BIND 模式也仅通过 execbuf3 ioctl 进行
提交。在 execbuf3 调用时该 VM 上所有被映射BO（通过 VM_BIND 调用）都被视为该提交所需
execbuf3 ioctl 直接指定批处理地址，而不是像 execbuf2 ioctl 那样使用对象句柄。execbuf3 ioctl
也不支持许多旧特性，如入/提交围栏、围栏数组、默gem 上下文等等（struct
drm_i915_gem_execbuffer3）
VM_BIND 模式下，VA 分配完全由用户管理，而非 i915 驱动。因此所有的 VA 分配、驱逐（evictionVM_BIND 模式下都不适用。此外，为了确定对象的活跃性（activeness），VM_BIND 模式不会使用
i915_vma 活跃引用跟踪，而是使用 dma-resv 对象（见 `VM_BIND dma_resv 用法`_）
因此，许多支execbuf2 ioctl 的现有代码，如重定位、VA 驱逐、vma 查找表、隐式同步、vma 活跃
引用跟踪等，都不适用execbuf3 ioctl。因此，所execbuf3 特有的处理应放在单独的文件中，只这些 ioctl 共用的功能才尽可能放在共享代码中
### VM_PRIVATE 对象

默认情况下，BO 可以映射到多VM 上，也可以被 dma-buf 导出。因此这BO 被称为共BO（Shared
BO）。在每次 execbuf 提交时，请求围栏必须被添加到VM 上所有被映射的共BO dma-resv 围栏
列表中
VM_BIND 特性引入了一项优化：用户可以BO 创建时通过 I915_GEM_CREATE_EXT_VM_PRIVATE 标志创建
专用于某个指VM BO。与共享 BO 不同，这VM 私有 BO 只能被映射到它们所私有的那VM 上，
并且不能dma-buf 导出。一VM 的所有私BO 共享同一dma-resv 对象。因此在每次 execbuf
提交时，它们只需更新一dma-resv 围栏列表。这样一来，在快速路径（所需映射已绑定）下提交的延迟
相对VM 私有 BO 的数量是 O(1) 的
### VM_BIND 加锁层级

此处的加锁设计支持旧的（基于 execlist 的）execbuf 模式、新VM_BIND 模式、带 GPU 缺页VM_BIND 模式以及未来可能的系统分配器支持（见 `共享虚拟内存（SVM）支持`_）。旧execbuf 模式不带缺页的新 VM_BIND 模式使用 dma_fence 管理后备存储（backing storage）的驻留。带缺页VM_BIND
模式和系统分配器支持则完全不使用任何 dma_fence
VM_BIND 加锁顺序如下
1) Lock-A：一vm_bind 互斥锁将保护 vm_bind 列表。该锁在 vm_bind/vm_unbind ioctl 调用中   execbuf 路径中以及释放映射时获取
   未来在支GPU 缺页时，我们可能会改rwsem，以便多个缺页处理程序可以获取读侧锁来查找映射，
   从而可以并行运行。旧execbuf 绑定模式不需要此锁
2) Lock-B：对象的 dma-resv 锁将保护 i915_vma 状态，在异步工作线程中绑定/解绑 vma 以及更新对象   dma-resv 围栏列表时需要持有。注意，一VM 的私BO 都会共享一dma-resv 对象
   未来的系统分配器支持将改HMM 规定的加锁
3) Lock-C：自旋锁，用于保VM 的一些列表，如被失效vma 列表（因驱逐和 userptr 失效等）
在支GPU 缺页时，execbuf 路径不会获取上述任何锁。在那里我们只需简单地将新的批处理缓冲地址塞入 ring，然后通知调度器运行它。加锁只发生在缺页处理程序中，在那里我们以读模式获取
lock-A、获取找到后备存储所需的任lock-B（gem 对象dma_resv 锁，以及系统分配器的
hmm/core mm）以及一些额外的锁（lock-D）来处理页表竞争。缺页模式不应需要操vm 列表，因此永不需lock-C
### VM_BIND LRU 处理

我们需要确VM_BIND 映射的对象被正确打上 LRU 标记，以避免性能下降。我们还需要支VM_BIND
对象的批LRU 移动，以避免execbuf 路径中产生额外延迟
页表页与 VM_BIND 映射的对象类似（`可驱逐的页表分配`_），它们VM 维护，并且在VM 激活时（即在该 VM 上的 execbuf 调用时）需要被锁定在内存中。因此也需要对页表页进行批LRU
移动
### VM_BIND dma_resv 用法

围栏需要被添加到所VM_BIND 映射的对象上。在每次 execbuf 提交时，它们DMA_RESV_USAGE_BOOKKEEP
用法添加，以防止过度同步（见 enum dma_resv_usage）。在显式设置对象依赖时，可以DMA_RESV_USAGE_READ DMA_RESV_USAGE_WRITE 用法覆盖它
注意，DRM_I915_GEM_WAIT DRM_I915_GEM_BUSY ioctl 不检DMA_RESV_USAGE_BOOKKEEP 用法，因不应被用于批处理结束检查。相反，应使execbuf3 出站围栏进行批处理结束检查（struct
drm_i915_gem_execbuffer3）
此外，在 VM_BIND 模式下，使用 dma-resv API 来确定对象的活跃性（dma_resv_test_signaled() dma_resv_wait_timeout()），而不要使用已废弃的旧 i915_vma 活跃引用跟踪。这应该更容易与当前TTM 后端协同工作
### Mesa 用例

VM_BIND 有可能降Mesa（包Vulkan Iris）中CPU 开销，从而提升受 CPU 限制的应用的性能它还允许我们实现 Vulkan 的稀疏资源（Sparse Resources）。随着 GPU 硬件性能的提升，降低 CPU 开销
变得更有意义
## 其他 VM_BIND 用例

### 长时运行的计算上下文

dma-fence 的使用期望它们在合理的时间内完成。而计算则可能是长时运行的。因此计算适合使用
用户/内存围栏（见 `用户/内存围栏`_），dma-fence 的使用必须仅限于内核内部消费
在不支持 GPU 缺页的情况下，内核驱动在缓冲区失效时会发起长时运行上下文的挂起（抢占），完成失效重新验证 BO，然后恢复计算上下文。这是通过每个上下文一个抢占围栏来实现的，当有人试图等待它该围栏被启用，并触发上下文抢占
#### 用户/内存围栏

用户/内存围栏是一<地址,  对。要发出用户围栏信号，指定的值会被写入指定的虚拟地址并唤等待的进程。用户围栏可以由 GPU 或内核异步工作线程（如绑定完成时）发出信号。用户可以通过一个新用户围栏等待 ioctl 等待用户围栏
这方面之前的工作如下https://patchwork.freedesktop.org/patch/349417/

#### 低延迟提
允许计算 UMD 直接提交 GPU 任务，而不是通过 execbuf ioctl。这之所以可能，是因VM_BIND 不与
execbuf 同步。VM_BIND 允许为直接提交的作业绑定/解绑所需的映射
### 璋冭瘯鍣。
通过调试事件接口，用户空间进程（调试器）能够跟踪并作用于另一个进程（被调试进程）创建并通过
vm_bind 接口附加GPU 的资源
### GPU 缺页

未来支持 GPU 缺页时，将仅VM_BIND 模式下受支持。虽然旧execbuf 模式和新VM_BIND 绑定
模式都需要使dma-fence 来确保驻留，但支GPU 缺页的模式将不使用任dma-fence，因为驻留纯通过安装和移失效页表项来管理
### 页级提示设置

VM_BIND 允许按映射而非BO 设置任何提示。可能的提示包括放置（placement）和原子性。在即将到来GPU 按需缺页支持下，BO 级的放置提示将更具意义
### 页级缓存/CLOS 设置

VM_BIND 允许按映射而非BO 设置缓存/CLOS
### 可驱逐的页表分配

使页表分配可驱逐，并像 VM_BIND 映射的对象一样管理它们。页表页类似VM 的持久映射（区别在于页表
页没i915_vma 结构，并且在换入页面后需要更新父页链接）
### 共享虚拟内存（SVM）支
VM_BIND 接口可用于使HMM 接口直接映射系统内存（无需 gem BO 抽象）。SVM 仅在启用 GPU 缺页受支持
## VM_BIND UAPI
