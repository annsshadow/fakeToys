## VFIO 鈥，鈥淰irtual Function I/O鈥?[1]_


如今许多现代系统都提DMA 与中断重映射设施，以帮助确保 I/O 设备在其被分配的边界内运行。这包括带有 AMD-Vi Intel VT-d x86 硬件、带有可分区端点（Partitionable Endpoints，PEs）的 POWER 系统，以及嵌入式 PowerPC 系统（如 Freescale PAMU）。VFIO 驱动是一个与 IOMMU/设备无关的框架，用于在安全、受 IOMMU 保护的环境中向用户空间暴露直接的设备访问。换句话说，它允许安[^2]_、非特权的用户空间驱动
我们为什么需要它？虚拟机在配置为尽可能高I/O 性能时，通常会使用直接的设备访问（“设备直通”，device assignment）。从设备和主机的角度来看，这只不过是把虚拟机变成了用户空间驱动，其好处是显著更低的延迟、更高的带宽，以及直接使用裸金属设备驱动 [^3^]_
某些应用，尤其是高性能计算领域的应用，也能从用户空间的低开销直接设备访问中受益。例子包括网络适配器（通常非基TCP/IP）和计算加速器。在 VFIO 出现之前，这些驱动要么必须经过完整的开发周期才能成为合适的上游驱动，要么在树外维护，要么使UIO 框架——UIO 没有 IOMMU 保护的概念、中断支持有限，并且需root 权限才能访问 PCI 配置空间之类的内容
VFIO 驱动框架旨在统一这些方案，既取代 KVM PCI 特定的设备直通代码，又提供比 UIO 更安全、功能更丰富的用户空间驱动环境
### 组、设备与 IOMMU


设备是任I/O 驱动的主要目标。设备通常会创建一个由 I/O 访问、中断和 DMA 组成的编程接口。不深入讨论其中每一项，DMA 到目前为止是维护安全环境最关键的方面，因为允许设备对系统内存进行读写访问会对整个系统的完整性构成最大的风险
为了帮助缓解这一风险，许多现IOMMU 现在把隔离属性引入了本在许多情况下只是用于转换（即解决地址空间有限的设备的寻址问题）的接口中。有了它，设备现在可以被彼此隔离、也可以与任意内存访问隔离，从而允许诸如将设备安全地直接直通到虚拟机之类的事情
不过，这种隔离并不总是以单个设备的粒度进行。即IOMMU 具备这种能力，设备、互连以IOMMU 拓扑各自的属性都会削弱这种隔离。例如，单个设备可能是更大的多功能封装的一部分。虽IOMMU 可能能够区分封装内的设备，但封装可能并不要求设备间的交易到达 IOMMU。这方面的例子从功能之间存在后门的多功能 PCI 设备，到允许在不经过 IOMMU 的情况下进行重定向、不支持 PCI-ACS（Access Control Services，访问控制服务）的桥，不一而足。拓扑也可能在隐藏设备方面起作用。PCIe PCI 桥会掩盖其后的设备，使交易看起来像是从桥本身发出的。显然，IOMMU 的设计也是一个主要因素
因此，尽管在大多数情况下 IOMMU 可能具有设备级粒度，但任何系统都容易出现粒度降低的情况。因IOMMU API 支持 IOMMU 组（group）的概念。组是一组可以与系统中所有其他设备隔离的设备。因此，组是 VFIO 使用的所有权单位
虽然组是确保安全用户访问所必须使用的最小粒度，但它不一定是首选粒度。在使用页表IOMMU 中，可能可以在不同组之间共享一组页表，从而减少平台的开销（减TLB 抖动、减少重复的页表）和用户开销（只需编程一组转换）。为此，VFIO 使用了容器（container）类，它可以持有一个或多个组。容器只需打开 /dev/vfio/vfio 字符设备即可创建
容器本身提供的功能很少，除了少数版本和扩展查询接口外都被锁住。用户需要向容器中添加一个组才能获得下一级的功能。为此，用户首先需要确定与所需设备关联的组。这可以通过下面示例中描述的 sysfs 链接来完成。通过将设备从宿主机驱动解绑并将其绑定VFIO 驱动，会为该组出现一个新VFIO /dev/vfio/$GROUP，其$GROUP 是该设备所属的 IOMMU 组编号。如IOMMU 组包含多个设备，则每个设备都需要先绑定VFIO 驱动，才允许VFIO 组进行操作（如果没有 VFIO 驱动可用，仅将设备从宿主机驱动解绑也足够；这会让组可用，但那个特定设备不可用）。TBD——用于禁用驱动探锁定设备的接口
组准备好后，可以通过打开 VFIO 组字符设备（/dev/vfio/$GROUP）并使用 VFIO_GROUP_SET_CONTAINER ioctl、传入之前打开的容器文件的文件描述符，将其添加到容器中。如果需要，并且 IOMMU 驱动支持在组之间共享 IOMMU 上下文，则可以将多个组设置到同一个容器中。如果一个组无法设置到含有已有组的容器，则需要改用一个新的空容器
组（或若干组）附加到容器后，其余ioctl 就可用了，从而能够访VFIO IOMMU 接口。此外，现在可以通过VFIO 组文件描述符使用 ioctl 来获取组内每个设备的文件描述符
VFIO 设备 API 包含用于描述设备、I/O 区域及其在设备描述符上的 read/write/mmap 偏移量的 ioctl，以及用于描述和注册中断通知的机制
### VFIO 使用示例


```

	$ readlink /sys/bus/pci/devices/0000:06:0d.0/iommu_group
	../../../../kernel/iommu_groups/26

```
因此该设备位IOMMU 26。该设备pci 总线上，因此用户将使vfio-pci 来管
```

	# modprobe vfio-pci

```
将该设备绑定vfio-pci 驱动会创VFIO 
```

	$ lspci -n -s 0000:06:0d.0
	06:0d.0 0401: 1102:0002 (rev 08)
	# echo 0000:06:0d.0 > /sys/bus/pci/devices/0000:06:0d.0/driver/unbind
	# echo 1102 0002 > /sys/bus/pci/drivers/vfio-pci/new_id

```
现在我们需要查看组里还有哪些其他设备以释放

```

	$ ls -l /sys/bus/pci/devices/0000:06:0d.0/iommu_group/devices
	total 0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:00:1e.0 ->
		../../../../devices/pci0000:00/0000:00:1e.0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:06:0d.0 ->
		../../../../devices/pci0000:00/0000:00:1e.0/0000:06:0d.0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:06:0d.1 ->
		../../../../devices/pci0000:00/0000:00:1e.0/0000:06:0d.1

```
该设备位于一PCIe PCI [^4^]_ 之后，因此我们还需要把设备 0000:06:0d.1 按与上述相同的步骤加入组。设0000:00:1e.0 是一个当前没有宿主机驱动的桥，因此不要求将该设备绑定vfio-pci 驱动（vfio-pci 目前不支PCI 桥）
如果希望进行非特权操作，最后一步是赋予用户对该组的访问权限（注/dev/vfio/vfio 本身不提供任何能力，因此预期将其设置
```

	# chown user:user /dev/vfio/26

```
用户现在对此容器中的所有设备及iommu 拥有完全访问
```

	int container, group, device, i;
	struct vfio_group_status group_status =
					{ .argsz = sizeof(group_status) };
	struct vfio_iommu_type1_info iommu_info = { .argsz = sizeof(iommu_info) };
	struct vfio_iommu_type1_dma_map dma_map = { .argsz = sizeof(dma_map) };
	struct vfio_device_info device_info = { .argsz = sizeof(device_info) };

	/* Create a new container */
	container = open("/dev/vfio/vfio", O_RDWR);

	if (ioctl(container, VFIO_GET_API_VERSION) != VFIO_API_VERSION)
		/* Unknown API version */

	if (!ioctl(container, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU))
		/* Doesn't support the IOMMU driver we want. */

	/* Open the group */
	group = open("/dev/vfio/26", O_RDWR);

	/* Test the group is viable and available */
	ioctl(group, VFIO_GROUP_GET_STATUS, &group_status);

	if (!(group_status.flags & VFIO_GROUP_FLAGS_VIABLE))
		/* Group is not viable (ie, not all devices bound for vfio) */

	/* Add the group to the container */
	ioctl(group, VFIO_GROUP_SET_CONTAINER, &container);

	/* Enable the IOMMU model we want */
	ioctl(container, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU);

	/* Get addition IOMMU info */
	ioctl(container, VFIO_IOMMU_GET_INFO, &iommu_info);

	/* Allocate some space and setup a DMA mapping */
	dma_map.vaddr = mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
			     MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
	dma_map.size = 1024 * 1024;
	dma_map.iova = 0; /* 1MB starting at 0x0 from device view */
	dma_map.flags = VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE;

	ioctl(container, VFIO_IOMMU_MAP_DMA, &dma_map);

	/* Get a file descriptor for the device */
	device = ioctl(group, VFIO_GROUP_GET_DEVICE_FD, "0000:06:0d.0");

	/* Test and setup the device */
	ioctl(device, VFIO_DEVICE_GET_INFO, &device_info);

	for (i = 0; i < device_info.num_regions; i++) {
		struct vfio_region_info reg = { .argsz = sizeof(reg) };

		reg.index = i;

		ioctl(device, VFIO_DEVICE_GET_REGION_INFO, &reg);

		/* Setup mappings... read/write offsets, mmaps
		 * For PCI devices, config space is a region */
	}

	for (i = 0; i < device_info.num_irqs; i++) {
		struct vfio_irq_info irq = { .argsz = sizeof(irq) };

		irq.index = i;

		ioctl(device, VFIO_DEVICE_GET_IRQ_INFO, &irq);

		/* Setup IRQs... eventfds, VFIO_DEVICE_SET_IRQS */
	}

	/* Gratuitous device reset and go... */
	ioctl(device, VFIO_DEVICE_RESET);

```
### IOMMUFD 涓?vfio_iommu_type1


IOMMUFD 是用于从用户空间管理 I/O 页表的新用户 API。它旨在成为交付高级用户空间 DMA 特性（嵌套转换 [^5^]_、PASID [^6^]_ 等）的门户，同时为现有的 VFIO_TYPE1v2_IOMMU 用例提供向后兼容接口。最终，vfio_iommu_type1 驱动以及传统vfio 容器与组模型都计划被弃用
IOMMUFD 向后兼容接口可以通过两种方式启用。第一种方式，内核可以CONFIG_IOMMUFD_VFIO_CONTAINER 配置，在这种情况IOMMUFD 子系统透明地为 VFIO 容器IOMMU 后端接口提供完整的基础设施。如VFIO 容器接口（即 /dev/vfio/vfio）被简单地符号链接/dev/iommu，也可以访问兼容模式。请注意，在撰写本文时，相对VFIO_TYPE1v2_IOMMU（例DMA 映射 MMIO），兼容模式的功能尚不完整，并且不打算为 VFIO_SPAPR_TCE_IOMMU 接口提供兼容性。因此，目前一般不建议从原生的 VFIO 实现切换IOMMUFD 兼容接口
从长远来看，VFIO 用户应当迁移到通过下面描述cdev 接口进行设备访问，以及通过 IOMMUFD 提供的接口进行原生访问
### VFIO 设备 cdev


传统上，用户通过 VFIO 组中VFIO_GROUP_GET_DEVICE_FD 获取设备 fd
启用 CONFIG_VFIO_DEVICE_CDEV=y 后，用户现在可以通过直接打开字符设备 /dev/vfio/devices/vfioX 来获取设fd，其中“X”是 VFIO 为已注册设备唯一分配的数字。cdev 接口不支noiommu 设备，因此如果需noiommu，用户应使用传统的组接口
cdev 仅与 IOMMUFD 配合工作。VFIO 驱动和应用程序都必须适应新的 cdev 安全模型，该模型要求在实际开始使用设备之前使VFIO_DEVICE_BIND_IOMMUFD 来声DMA 所有权。一BIND 成功，VFIO 设备就能被用户完全访问
VFIO 设备 cdev 不依VFIO 容器/IOMMU 驱动。因此，在没有传VFIO 应用的环境中，那些模块可以被完全编译掉
迄今为止，SPAPR 尚不支持 IOMMUFD。因此它也不能支持设cdev
vfio 设备 cdev 访问仍然IOMMU 组语义约束，即一个组只能有一DMA 所有者。属于同一组的设备不能绑定到多iommufd_ctx，也不能在原生内核与 vfio 总线驱动或支driver_managed_dma 标志的其他驱动之间共享。违反此所有权要求会在 VFIO_DEVICE_BIND_IOMMUFD ioctl 处失败，ioctl 是完整设备访问的门槛
### 设备 cdev 示例


```

	$ ls /sys/bus/pci/devices/0000:6a:01.0/vfio-dev/
	vfio0

```
因此该设备表示为 vfio0。用户可以验
```

	$ ls -l /dev/vfio/devices/vfio0
	crw------- 1 root root 511, 0 Feb 16 01:22 /dev/vfio/devices/vfio0
	$ cat /sys/bus/pci/devices/0000:6a:01.0/vfio-dev/vfio0/dev
	511:0
	$ ls -l /dev/char/511\:0
	lrwxrwxrwx 1 root root 21 Feb 16 01:22 /dev/char/511:0 -> ../vfio/devices/vfio0

```
如果希望非特权访问，则赋予用户对该设备的访问权限

```

	$ chown user:user /dev/vfio/devices/vfio0

```
```

	cdev_fd = open("/dev/vfio/devices/vfio0", O_RDWR);

```
打开cdev_fd 不会赋予用户访问设备的任何权限，只能cdev_fd 绑定到一iommufd。在那之后，设备才被完全访问，包括将其附加到一
```

	struct vfio_device_bind_iommufd bind = {
		.argsz = sizeof(bind),
		.flags = 0,
	};
	struct iommu_ioas_alloc alloc_data  = {
		.size = sizeof(alloc_data),
		.flags = 0,
	};
	struct vfio_device_attach_iommufd_pt attach_data = {
		.argsz = sizeof(attach_data),
		.flags = 0,
	};
	struct iommu_ioas_map map = {
		.size = sizeof(map),
		.flags = IOMMU_IOAS_MAP_READABLE |
			 IOMMU_IOAS_MAP_WRITEABLE |
			 IOMMU_IOAS_MAP_FIXED_IOVA,
		.__reserved = 0,
	};

	iommufd = open("/dev/iommu", O_RDWR);

	bind.iommufd = iommufd;
	ioctl(cdev_fd, VFIO_DEVICE_BIND_IOMMUFD, &bind);

	ioctl(iommufd, IOMMU_IOAS_ALLOC, &alloc_data);
	attach_data.pt_id = alloc_data.out_ioas_id;
	ioctl(cdev_fd, VFIO_DEVICE_ATTACH_IOMMUFD_PT, &attach_data);

	/* Allocate some space and setup a DMA mapping */
	map.user_va = (int64_t)mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
				    MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
	map.iova = 0; /* 1MB starting at 0x0 from device view */
	map.length = 1024 * 1024;
	map.ioas_id = alloc_data.out_ioas_id;

	ioctl(iommufd, IOMMU_IOAS_MAP, &map);

	/* Other device operations as stated in "VFIO Usage Example" */

```
### VFIO 用户 API


完整API 文档请参include/uapi/linux/vfio.h
### VFIO 总线驱动 API


VFIO 总线驱动（如 vfio-pci）只使用少数几个进入 VFIO 核心的接口。当设备被绑定到驱动以及从驱动解绑时，在设备被绑定到以及

```

	int vfio_register_group_dev(struct vfio_device *device);
	int vfio_register_emulated_iommu_dev(struct vfio_device *device);
	void vfio_unregister_group_dev(struct vfio_device *device);

```
驱动应当vfio_device 嵌入到自己的结构体中，并使用 vfio_alloc_device() 来分配该结构体，还可以注@init/@release 回调来管理包裹该

```

	vfio_alloc_device(dev_struct, member, dev, ops);
	void vfio_put_device(struct vfio_device *device);

```
vfio_register_group_dev() 通知核心开始跟踪指dev iommu_group，并将该 dev 注册为由 VFIO 总线驱动拥有。一vfio_register_group_dev() 返回，用户空间就可以开始访问该驱动，因此驱动应当确保在调用它之前已完全准备就绪。驱动提供一个用于回调的 ops 结构
```

	struct vfio_device_ops {
		char	*name;
		int	(*init)(struct vfio_device *vdev);
		void	(*release)(struct vfio_device *vdev);
		int	(*bind_iommufd)(struct vfio_device *vdev,
					struct iommufd_ctx *ictx, u32 *out_device_id);
		void	(*unbind_iommufd)(struct vfio_device *vdev);
		int	(*attach_ioas)(struct vfio_device *vdev, u32 *pt_id);
		void	(*detach_ioas)(struct vfio_device *vdev);
		int	(*open_device)(struct vfio_device *vdev);
		void	(*close_device)(struct vfio_device *vdev);
		ssize_t	(*read)(struct vfio_device *vdev, char __user *buf,
				size_t count, loff_t *ppos);
		ssize_t	(*write)(struct vfio_device *vdev, const char __user *buf,
			 size_t count, loff_t *size);
		long	(*ioctl)(struct vfio_device *vdev, unsigned int cmd,
				 unsigned long arg);
		int	(*mmap)(struct vfio_device *vdev, struct vm_area_struct *vma);
		void	(*request)(struct vfio_device *vdev, unsigned int count);
		int	(*match)(struct vfio_device *vdev, char *buf);
		void	(*dma_unmap)(struct vfio_device *vdev, u64 iova, u64 length);
		int	(*device_feature)(struct vfio_device *device, u32 flags,
					  void __user *arg, size_t argsz);
	};

```
每个函数都会传入最初在上面vfio_register_group_dev() vfio_register_emulated_iommu_dev() 调用中注册的 vdev。这让总线驱动可以使用 container_of() 获取其私有数据
```

	- The init/release callbacks are issued when vfio_device is initialized
	  and released.

	- The open/close device callbacks are issued when the first
	  instance of a file descriptor for the device is created (eg.
	  via VFIO_GROUP_GET_DEVICE_FD) for a user session.

	- The ioctl callback provides a direct pass through for some VFIO_DEVICE_*
	  ioctls.

	- The [un]bind_iommufd callbacks are issued when the device is bound to
	  and unbound from iommufd.

	- The [de]attach_ioas callback is issued when the device is attached to
	  and detached from an IOAS managed by the bound iommufd. However, the
	  attached IOAS can also be automatically detached when the device is
	  unbound from iommufd.

	- The read/write/mmap callbacks implement the device region access defined
	  by the device's own VFIO_DEVICE_GET_REGION_INFO ioctl.

	- The request callback is issued when device is going to be unregistered,
	  such as when trying to unbind the device from the vfio bus driver.

	- The dma_unmap callback is issued when a range of iovas are unmapped
	  in the container or IOAS attached by the device. Drivers which make
	  use of the vfio page pinning interface must implement this callback in
	  order to unpin pages within the dma_unmap range. Drivers must tolerate
	  this callback even before calls to open_device().

```
### PPC64 sPAPR 实现说明


本实现有一些特定之处：

1) 在较旧的系统（带 P5IOC2/IODA1 POWER7）上，每个容器只支持一IOMMU 组，因为 IOMMU 表是在启动时分配的，每个 IOMMU 组（即可分区端点 PE）一张表（PE 通常是一PCI 域，但不一定）
   较新的系统（IODA2 POWER8）改进了硬件设计，可以消除这一限制，从而每VFIO 容器可以有多IOMMU 组
2) 硬件支持所谓的 DMA 窗口——即允许进行 DMA 传输PCI 地址范围，任何访问窗口外地址空间的尝试都会导致整PE 被隔离
3) PPC64 客户机是半虚拟化的，但不是完全模拟的。有一个用于为 DMA 映射/取消映射页的 API，通常每次调用映射 1..32 页，目前无法减少调用次数。为了让事情更快，映取消映射的处理已在实模式（real mode）中实现，提供了出色的性能，但也存在诸如无法实时进行锁定页记账之类的限制
4) 根据 sPAPR 规范，可分区端点（PE）是一I/O 子树，在分区和错误恢复时可被当作一个单元处理。PE 可以是单功能或多功能 IOA（IO 适配器）、多功能 IOA 的一个功能，或多IOA（可能包含多IOA 之上的交换机和桥结构）。PPC64 客户机通过 EEH RTAS 服务检测并PCI 错误中恢复，该服务基于额外的 ioctl 命令运作
   因此新增4 个额外的 ioctl
	VFIO_IOMMU_SPAPR_TCE_GET_INFO
		returns the size and the start of the DMA window on the PCI bus.

	VFIO_IOMMU_ENABLE
		enables the container. The locked pages accounting
		is done at this point. This lets user first to know what
		the DMA window is and adjust rlimit before doing any real job.

	VFIO_IOMMU_DISABLE
		disables the container.

	VFIO_EEH_PE_OP
		provides an API for EEH setup, error detection and recovery.

```

	struct vfio_eeh_pe_op pe_op = { .argsz = sizeof(pe_op), .flags = 0 };

	.....
	/* Add the group to the container */
	ioctl(group, VFIO_GROUP_SET_CONTAINER, &container);

	/* Enable the IOMMU model we want */
	ioctl(container, VFIO_SET_IOMMU, VFIO_SPAPR_TCE_IOMMU)

	/* Get addition sPAPR IOMMU info */
	vfio_iommu_spapr_tce_info spapr_iommu_info;
	ioctl(container, VFIO_IOMMU_SPAPR_TCE_GET_INFO, &spapr_iommu_info);

	if (ioctl(container, VFIO_IOMMU_ENABLE))
		/* Cannot enable container, may be low rlimit */

	/* Allocate some space and setup a DMA mapping */
	dma_map.vaddr = mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
			     MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);

	dma_map.size = 1024 * 1024;
	dma_map.iova = 0; /* 1MB starting at 0x0 from device view */
	dma_map.flags = VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE;

	/* Check here is .iova/.size are within DMA window from spapr_iommu_info */
	ioctl(container, VFIO_IOMMU_MAP_DMA, &dma_map);

	/* Get a file descriptor for the device */
	device = ioctl(group, VFIO_GROUP_GET_DEVICE_FD, "0000:06:0d.0");

	....

	/* Gratuitous device reset and go... */
	ioctl(device, VFIO_DEVICE_RESET);

	/* Make sure EEH is supported */
	ioctl(container, VFIO_CHECK_EXTENSION, VFIO_EEH);

	/* Enable the EEH functionality on the device */
	pe_op.op = VFIO_EEH_PE_ENABLE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* You're suggested to create additional data struct to represent
	 * PE, and put child devices belonging to same IOMMU group to the
	 * PE instance for later reference.
	 */

	/* Check the PE's state and make sure it's in functional state */
	pe_op.op = VFIO_EEH_PE_GET_STATE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Save device state using pci_save_state().
	 * EEH should be enabled on the specified device.
	 */

	....

	/* Inject EEH error, which is expected to be caused by 32-bits
	 * config load.
	 */
	pe_op.op = VFIO_EEH_PE_INJECT_ERR;
	pe_op.err.type = EEH_ERR_TYPE_32;
	pe_op.err.func = EEH_ERR_FUNC_LD_CFG_ADDR;
	pe_op.err.addr = 0ul;
	pe_op.err.mask = 0ul;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	....

	/* When 0xFF's returned from reading PCI config space or IO BARs
	 * of the PCI device. Check the PE's state to see if that has been
	 * frozen.
	 */
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Waiting for pending PCI transactions to be completed and don't
	 * produce any more PCI traffic from/to the affected PE until
	 * recovery is finished.
	 */

	/* Enable IO for the affected PE and collect logs. Usually, the
	 * standard part of PCI config space, AER registers are dumped
	 * as logs for further analysis.
	 */
	pe_op.op = VFIO_EEH_PE_UNFREEZE_IO;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/*
	 * Issue PE reset: hot or fundamental reset. Usually, hot reset
	 * is enough. However, the firmware of some PCI adapters would
	 * require fundamental reset.
	 */
	pe_op.op = VFIO_EEH_PE_RESET_HOT;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);
	pe_op.op = VFIO_EEH_PE_RESET_DEACTIVATE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Configure the PCI bridges for the affected PE */
	pe_op.op = VFIO_EEH_PE_CONFIGURE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Restored state we saved at initialization time. pci_restore_state()
	 * is good enough as an example.
	 */

	/* Hopefully, error is recovered successfully. Now, you can resume to
	 * start PCI traffic to/from the affected PE.
	 */

	....

```
5) SPAPR TCE IOMMU v2 版本。它弃用VFIO_IOMMU_ENABLE/VFIO_IOMMU_DISABLE，并实现2 个新ioctl：VFIO_IOMMU_SPAPR_REGISTER_MEMORY VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY（在 v1 IOMMU 中不受支持）
   PPC64 半虚拟化客户机会产生大量的映取消映射请求，这些请求的处理包含锁定/解锁页，并更mm::locked_vm 计数器以确保不超rlimit。v2 IOMMU 将记账与锁定拆分为独立的操作
   - VFIO_IOMMU_SPAPR_REGISTER_MEMORY/VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY ioctl 接收一个用户空间地址以及要被锁定块的大小。不支持二分（bisecting），并且期望 VFIO_IOMMU_UNREGISTER_MEMORY 使用注册该内存块时所用的确切地址和大小来调用。不期望用户空间频繁调用这些。这些范围存储在 VFIO 容器的链表中
   - VFIO_IOMMU_MAP_DMA/VFIO_IOMMU_UNMAP_DMA ioctl 只更新实际的 IOMMU 表，不进行锁定；相反，它们检查用户空间地址是否来自预先注册的范围
   这种分离有助于优化客户机DMA
6) sPAPR 规范允许客户机在 PCI 总线上拥有额外的 DMA 窗口（可变页大小）。为此新增了两个 ioctl：VFIO_IOMMU_SPAPR_TCE_CREATE VFIO_IOMMU_SPAPR_TCE_REMOVE。平台必须支持该功能，否则会向用户空间返回错误。现有硬件最多支2 DMA 窗口，一个是 2GB 长、使4K 页，称为“默32 位窗口（default 32bit window）”；另一个可以大到整RAM、使用不同的页大小，它是可选的——如果客户机驱动支持 64 DMA，客户机在运行时创建它们
   VFIO_IOMMU_SPAPR_TCE_CREATE 接收一个页偏移（page shift）、DMA 窗口大小以及 TCE 表级数（如果 TCE 表将足够大、而内核可能无法分配足够的物理连续内存）。它在可用的槽中创建一个新窗口，并返回新窗口开始的 bus 地址。受硬件限制，用户空间无法选择 DMA 窗口的位置
   VFIO_IOMMU_SPAPR_TCE_REMOVE 接收窗口的总线起始地址并将其移除
-------------------------------------------------------------------------------

   最初由 Tom Lyon Cisco 时实现。从那以后我们已经超出了这个缩写的本意，但它很上口
   多功能设备有可能在功能之间存在后门，甚至单功能设备也有可能通过 MMIO 寄存器获得对 PCI 配置空间之类的替代访问。为了防止前者，我们可以IOMMU 驱动中加入额外的预防措施，将多功PCI 设备分组在一起（iommu=group_mf）。后者我们无法防止，IOMMU 仍应提供隔离。对PCI，SR-IOV 虚拟功能（Virtual Functions）是“行为良好”的最佳指标，因为它们是为虚拟化使用模型设计的
   超出 VFIO 范围的设备分配（assignment）。预计未来的 IOMMU 技术会减少其中部分（但也许不是全部）的取舍
```

	-[0000:00]-+-1e.0-[06]--+-0d.0
				\-0d.1

	00:1e.0 PCI bridge: Intel Corporation 82801 PCI Bridge (rev 90)

```
   地址转换。这提高IOMMU 虚拟化中的地址转换效率
   Express。它是共享虚拟寻址（Shared Virtual Addressing，SVA）和可扩I/O 虚拟化（Scalable I/O Virtualization，Scalable IOV）的先决条件