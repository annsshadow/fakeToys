
## 异步 VM_BIND

## 术语表（Nomenclature）：

- `VRAM`：设备上的内存。有时称为设备本地内存（device local memory）
- `gpu_vm`：一个虚GPU 地址空间。通常每个进程一个，但也可以由多个进程共享
- `VM_BIND`：一个用于通过 IOCTL 修改 gpu_vm 的操作或操作列表。这些操作包括映射和取消映射系统内存VRAM 内存
- `syncobj`：一个抽象了同步对象的容器。同步对象可以是通用的（dma-fence），也可以是驱动特定的。一syncobj 通常指示底层同步对象的类型
- `in-syncobj`：VM_BIND IOCTL 的参数，VM_BIND 操作在开始之前会等待这些对象
- `out-syncobj`：VM_BIND_IOCTL 的参数，当绑定操作完成时，VM_BIND 操作会向这些对象发信号
- `dma-fence`：一个跨驱动的同步对象。理解本文档需要基本的 dma-fence 知识。请参[dma-buf doc](dma-buf doc </driver-api/dma-buf>) 中的 “DMA Fences一节
- `memory fence`（内存栅栏）：一种不同于 dma-fence 的同步对象。内存栅栏使用指定内存位置的值来确定已发信号状态。内存栅栏既可以GPU 也可以由 CPU 等待和发信号。内存栅栏有时被称为 user-fence、userspace-fence gpu futex，并且不一定遵dma-fence 在“合理时间”内发信号的规则。因此内核应避免在持有锁的情况下等待内存栅栏
- `long-running workload`（长时间运行的工作负载）：一种可能需要超过当前规定的 dma-fence 最大发信号延迟才能完成的工作负载，因此需要将 gpu_vm GPU 执行上下文置于某种禁止完dma-fence 的模式
- `exec function`（执行函数）：一个重新验证所有受影响gpu_vma、提交一GPU 命令批次、并向所有受影响dma_resv 注册代表 GPU 命令活动dma_fence 的函数。为了完整性（尽管本文档未涵盖），值得一提的是，exec 函数也可能是某些驱动在计长时间运行模式下使用的重新验证工作线程
- `bind context`（绑定上下文）：用于 VM_BIND 操作的上下文标识符。使用相同绑定上下文VM_BIND 操作，在重要的地方，可以被假定按提交顺序完成。对于使用独立绑定上下文VM_BIND 操作，不能做这样的假设
- `UMD`：用户模式驱动（User-mode driver）
- `KMD`：内核模式驱动（Kernel-mode driver）

## 同步 / 异步 VM_BIND 操作

同步 VM_BIND
___________________
使用同步 VM_BIND，所VM_BIND 操作IOCTL 返回之前完成。同VM_BIND 既不接受 in-fence 也不接受 out-fence。同VM_BIND 可能会阻塞并等待 GPU 操作；例如换入（swap-in）或清零，甚至是先前的绑定
异步 VM_BIND
____________________
异步 VM_BIND 同时接受 in-syncobj out-syncobj。虽IOCTL 可能立即返回，但 VM_BIND 操作在修GPU 页表之前会等in-syncobj，并在修改完成后（即下一次等out-syncobj exec 函数将看到更改的意义上）out-syncobj 发信号。错误是同步报告的在低内存情况下，实现可能会阻塞，同步地执VM_BIND，因为可能没有完全足够的内存立即可用于准备异步操作
如果 VM_BIND IOCTL 接受一个操作列表或数组作为参数，in-syncobj 需要在第一个操作开始执行之前发信号，out-syncobj 在最后一个操作完成后发信号。在重要的地方，可以假定操作列表中的操作按顺序完成
由于异步 VM_BIND 操作可能使用嵌入out-syncobj 中的 dma-fence 以及KMD 内部用于发信号指示绑定完成的 dma-fence，任何作VM_BIND in-fence 给出的内存栅栏都需要在 VM_BIND ioctl 返回之前被同步等待，因为 dma-fence 要求在合理时间内发信号，绝不能依赖于没有此类限制的内存栅栏
异步 VM_BIND 操作的目的是让用户模式驱动能够流水线化交错进行的 gpu_vm 修改exec 函数。对于长时间运行的工作负载，这种绑定操作的流水线化是不允许的，任in-fence 都需要被同步等待。这其中的原因有两方面。首先，任何由长时间运行的工作负载门控、并用作 VM_BIND 操作in-syncobj 的内存栅栏无论如何都需要被同步等待（见上文）。其次，任何用作长时间运行工作负VM_BIND 操作in-syncobj dma-fence 无论如何都不允许流水线化，因为长时间运行的工作负载不允许dma-fence 用作 out-syncobj，所以虽然理论上可能，但使用它们是有疑问的，在没有有价值的用例之前应当被拒绝。注意，这不是由 dma-fence 规则施加的限制，而是KMD 实现为保持简单而施加的限制。它不影响将 dma-fence 用作长时间运行工作负载本身的依赖（这dma-fence 规则所允许的），而仅仅影VM_BIND 操作
一个异VM_BIND 操作可能需要大量时间来完成并向 out_fence 发信号。特别是当该操作在其VM_BIND 操作以及使用 exec 函数提交的工作负载之后被深度流水线化时。在这种情况下，如果没有显式依赖关系，UMD 可能希望避免后续VM_BIND 操作排队在第一个之后。为了规避这种排队，VM_BIND 实现可以允许创建 VM_BIND 上下文。对于每个上下文，VM_BIND 操作保证按它们被提交的顺序完成，但对于在独立 VM_BIND 上下文上执行VM_BIND 操作则不是这样。相反，KMD 会尝试并行执行此VM_BIND 操作，但不保证它们确实会并行执行。可能存在只KMD 知道的内部隐式依赖，例如页表结构的变化。一种尝试避免此类内部依赖的方法是让不同VM_BIND 上下文使VM 的不同区域
同样，对于长时间运行 gpu_vm VM_BIND，用户模式驱动通常应选择内存栅栏作为 out-fence，因为这为内核模式驱动在绑定/解绑操作中注入其他操作（例如向批处理缓冲区中插入断点）提供了更大的灵活性。然后，工作负载执行可以轻松地流水线化到绑定完成之后，使用内out-fence 作为 UMD 嵌入在工作负载中GPU 信号量的发信号条件
异步 VM_BIND 和同VM_BIND 在支持的操作或多操作支持方面没有区别
## 多操VM_BIND IOCTL 的错误处理与中断

VM_BIND IOCTL 操作可能由于各种原因而出错，例如由于完成所需的资源不足，以及由于等待被中断在这些情况下，UMD 最好在采取适当措施后重新启IOCTL如果 UMD 过度提交了内存资源，将返-ENOSPC 错误，然UMD 可以解绑当前未使用的资源并重新运IOCTL。对-EINTR，UMD 应简单地重新运行 IOCTL；对-ENOMEM，用户空间可以尝试释放已知的系统内存资源，或者失败。如UMD 由于错误返回而决定让某个绑定操作失败，则无需采取额外措施来清理失败的操作，VM 将保持在与失IOCTL 之前相同的状态解绑操作保证不会因资源限制而返回任何错误，但可能因例如无效参数gpu_vm 被封禁而返回错误如果在异步绑定过程中发生意外错误，gpu_vm 将被封禁，并且在封禁后尝试使用它将返-ENOENT
## 示例：Xe VM_BIND uAPI

VM_BIND 操作结构体开始，IOCTL 调用可以接受零个、一个或多个这样的操作。零个意味着只执IOCTL 的同步部分：异步 VM_BIND 更新 syncobject，而同VM_BIND 等待隐式依赖被满足

   struct drm_xe_vm_bind_op {
	/**
  - @obj: 要操作的对象，对MAP_USERPTR MBZ，对UNMAP MBZ
	 */
	__u32 obj;

	/** @pad: MBZ */
	__u32 pad;

	union {
		/**
   - @obj_offset: 用于 MAP 的对象的偏移		 */
		__u64 obj_offset;

		/** @userptr: 用于 MAP_USERPTR 的用户虚拟地址 */
		__u64 userptr;
	};

	/**
  - @range: 从对象绑定到 addr 的字节数，对UNMAP_ALL MBZ
	 */
	__u64 range;

	/** @addr: 要操作的地址，对UNMAP_ALL MBZ */
	__u64 addr;

	/**
  - @tile_mask: 为其创建绑定tile 掩码 == 所tile  - 仅适用于创建新VMA
	 */
	__u64 tile_mask;

       /* 将（对象的一部分）映射进 GPU 虚拟地址范围*/
    #define XE_VM_BIND_OP_MAP		0x0
        /** 取消映射一GPU 虚拟地址范围 **/
    #define XE_VM_BIND_OP_UNMAP		0x1
        /*
  - CPU 虚拟地址范围映射GPU 虚拟
  - 地址范围	 */
    #define XE_VM_BIND_OP_MAP_USERPTR	0x2
        /** VM 中解映射一gem 对象**/
    #define XE_VM_BIND_OP_UNMAP_ALL	0x3
        /*
  - 如果可能，使一个地址范围的后备内存常驻  - 注意这不会固定（pin）后备内存	 */
    #define XE_VM_BIND_OP_PREFETCH	0x4

        /** GPU 映射只读**/
    #define XE_VM_BIND_FLAG_READONLY	(0x1 << 16)
	/*
  - 仅在支持缺页VM 上有效，立即执行 MAP 操作
  - 而不是将 MAP 推迟到缺页处理程序	 */
    #define XE_VM_BIND_FLAG_IMMEDIATE	(0x1 << 17)
	/*
  - 当设置了 NULL 标志时，页表使用特殊位进行设置，
  - 该位指示写入被丢弃且所有读取返回零。在
  - 未来，NULL 标志将仅XE_VM_BIND_OP_MAP
  - 操作有效，BO 句柄MBZ，BO 偏移MBZ。此标志
  - 旨在实现 VK 稀疏绑定	 */
    #define XE_VM_BIND_FLAG_NULL	(0x1 << 18)
	/** @op: 要执行的操作（低 16 位）和标志（16 位） */
	__u32 op;

	/** @mem_region: 预取VMA 的内存区域，是实例而非掩码 */
	__u32 region;

	/** @reserved: 保留 */
	__u64 reserved[^2^];
   };


VM_BIND IOCTL 参数本身如下所示。注意，对于同步 VM_BIND，num_syncs syncs 字段必须为零。这里的 `exec_queue_id` 字段就是前面讨论过的 VM_BIND 上下文，用于促进乱序VM_BIND

    struct drm_xe_vm_bind {
	/** @extensions: 指向第一个扩展结构体的指针（如果有） */
	__u64 extensions;

	/** @vm_id: 要绑定的 VM ID */
	__u32 vm_id;

	/**
  - @exec_queue_id: exec_queue_id，必须是 DRM_XE_ENGINE_CLASS_VM_BIND 类，
  - 且执行队列必须具有相同的 vm_id。如果为零，则使用默认的 VM 绑定引擎	 */
	__u32 exec_queue_id;

	/** @num_binds: IOCTL 中绑定的数量 */
	__u32 num_binds;

        /** 如果设置，执行异VM_BIND；如果清除，执行同步 VM_BIND **/
    #define XE_VM_BIND_IOCTL_FLAG_ASYNC	(0x1 << 0)

	/** @flag: 控制ioctl 中所有操作的标志*/
	__u32 flags;

	union {
		/** @bind: num_binds == 1 时使*/
		struct drm_xe_vm_bind_op bind;

		/**
   - @vector_of_binds: 褰?num_binds > 1 鏃讹紝鎸囧悜 struct
   - drm_xe_vm_bind_op 数组userptr
		 */
		__u64 vector_of_binds;
	};

	/** @num_syncs: 要等待或在完成时发信号的同步对象数量*/
	__u32 num_syncs;

	/** @pad2: MBZ */
	__u32 pad2;

	/** @syncs: 指向 struct drm_xe_sync 数组的指*/
	__u64 syncs;

	/** @reserved: 保留 */
	__u64 reserved[^2^];
    };
