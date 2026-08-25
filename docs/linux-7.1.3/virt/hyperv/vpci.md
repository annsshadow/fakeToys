
## PCI 直通设

Hyper-V 客机 VM 中，PCI 直通设备（也称为虚PCI 设备，或 vPCI 设备）是被直接映射到 VM
物理地址空间中的物理 PCI 设备。客机设备驱动可以直接与硬件交互，而无需主机 hypervisor 中介。与hypervisor 虚拟化的设备相比，这种方法以更低的延迟提供了到设备的更高带宽访问。该
设备在客机中的表现应与在裸机上运行时完全一样，因此 Linux 设备驱动无需做任何修改
Hyper-V vPCI 设备的术语是 "Discrete Device Assignment"（DDA，离散设备分配）。Hyper-V DDA
的公开文档可在此处获取：`DDA`_


DDA 通常用于存储控制器（例如 NVMe）以GPU。用NIC 的类似机制称SR-IOV，通过允许客机
设备驱动直接与硬件交互而产生相同的好处。请参阅 Hyper-V 公开文档：`SR-IOV`_


此处vPCI 设备的讨论包DDA SR-IOV 设备
### 设备呈现


vPCI 设备运行时，Hyper-V 为其提供完整PCI 功能，因此只要该设备使用正确Linux 内核 API
来访PCI 配置空间以及Linux 集成，Linux 设备驱动就可以原样使用。但是，PCI 设备的初探测及其Linux PCI 子系统的集成必须使用 Hyper-V 特定的机制。因此，Hyper-V 上的 vPCI 设备
具有双重身份。它们最初通过标准VMBus "offer" 机制作为 VMBus 设备呈现Linux 客机，因此它具有 VMBus 身份，并出现/sys/bus/vmbus/devices 下。Linux 中位drivers/pci/controller/
pci-hyperv.c VMBus vPCI 驱动通过构造一PCI 总线拓扑并创建若 PCI 设备是在裸机系统上通过
ACPI 发现时本应存在的所有常PCI 设备数据结构，来处理一个新引入vPCI 设备。一旦这些数据结建立完成，该设备Linux 中也拥有了正常的 PCI 身份，该 vPCI 设备的常Linux 设备驱动就可像在裸机上的 Linux 中运行一样工作。由vPCI 设备是通过 VMBus offer 机制动态呈现的，它们不出现Linux 客机ACPI 表中。vPCI 设备可以VM 生命周期内的任意时刻被添加到 VM 或从 VM 移除，而不仅仅是在初始启动时
通过这种方法，vPCI 设备同时VMBus 设备PCI 设备。作为对 VMBus offer 消息的响应，hv_pci_probe()
函数运行并与 Hyper-V 主机上的 vPCI VSP 建立 VMBus 连接。该连接具有一个单一VMBus 通道。该
通道用于vPCI VSP 交换消息，以Linux 中配vPCI 设备。一旦该设备Linux 中作PCI 设备
被完全配置好，VMBus 通道仅在 Linux 更改要在客机中被中断vCPU，或在该 VM 运行vPCI 设备VM 中移除时才被使用。设备的持续运行直接发生在该设备Linux 设备驱动与硬件之间，VMBus VMBus 通道不起作用
### PCI 设备建立


PCI 设备建立遵循 Hyper-V 最初为 Windows 客机创建的顺序，由于 Linux PCI 子系统整体结构与 Windows
不同，它可能不太适合 Linux 客机。尽管如此，通过Linux Hyper-V 虚拟 PCI 驱动做一点修补，虚拟
PCI 设备Linux 中被建立，从而使通用Linux PCI 子系统代码和该设备的 Linux 驱动“直接可用”
每个 vPCI 设备Linux 中被建立为拥有自己的主机桥（host bridge）的 PCI 域（domain）。PCI
domainID 由分配给 VMBus vPCI 设备的实GUID 的第 4 和第 5 字节导出。Hyper-V 主机不保证这字节是唯一的，因此 hv_pci_probe() 有一种算法来解决冲突。该冲突解决旨在同一 VM 的多次重启之保持稳定，以PCI domainID 不会改变，因domainID 出现在某些设备的用户空间配置中
hv_pci_probe() 分配一个客MMIO 范围，用作该设备PCI 配置空间。这MMIO 范围作为告知主机
设备已准备好进入 d0 的一部分，通过 VMBus 通道传达Hyper-V。参hv_pci_enter_d0()。当客机
随后访问这个 MMIO 范围时，Hyper-V 主机会拦截这些访问，并将其映射到物理设备PCI 配置空间
hv_pci_probe() 还从 Hyper-V 主机获取该设备的 BAR 信息，并利用这些信息BAR 分配 MMIO 空间那个 MMIO 空间随后被设置为与主机桥关联，以便在 Linux 中通用 PCI 子系统代码处BAR 时能够工作
最后，hv_pci_probe() 创建PCI 总线。到此时，Hyper-V 虚拟 PCI 驱动的修补工作已经完成，扫描总线的常Linux PCI 机制开始工作，以探测设备、执行驱动匹配以及初始化驱动和设备
### PCI 设备移除


Hyper-V 主机可以VM 生命周期内的任意时刻发起从客VM 中移vPCI 设备。该移除由在 Hyper-V
主机上执行的管理操作触发，不受客OS 的控制
客机 VM 通过主机经与vPCI 设备关联VMBus 通道发送给客机的主动推送的 "Eject"（弹出）消息
获知移除。收到此类消息后，Linux 中的 Hyper-V 虚拟 PCI 驱动会异步调Linux 内核 PCI 子系调用来关闭并移除该设备。当这些调用完成时，一"Ejection Complete"（弹出完成）消息VMBus
通道发回Hyper-V，指示设备已被移除。此时，Hyper-V Linux 客机发送一VMBus rescind（撤销消息，Linux 中的 VMBus 驱动通过移除该设备的 VMBus 身份来处理它。一旦该处理完成，设备曾经存的所有痕迹都Linux 内核中消失了。rescind 消息还向客机表明 Hyper-V 已停止在客机中提供对 vPCI
设备的支持。如果客机尝试访问该设备MMIO 空间，那将是无效引用。影响该设备Hypercall 会返错误，并且在 VMBus 通道中发送的任何后续消息都将被忽略
在发Eject 消息之后，Hyper-V 允许客机 VM 60 秒时间来干净地关闭设备并Ejection Complete
响应，然后才发VMBus rescind 消息。如果由于任何原Eject 步骤未能在允许的 60 秒内完成Hyper-V 主机会强制执 rescind 步骤，这很可能会导致客机中出现级联错误，因为从该客机角度看设现已不再存在，访问该设备 MMIO 空间将失败
由于弹出是异步的，并且可能发生在客机 VM 生命周期中的任何时刻，Hyper-V 虚拟 PCI 驱动中的正确
同步非常棘手。甚至在新提供的 vPCI 设备尚未完全建立时就观察到过弹出。多年来，Hyper-V 虚拟 PCI
驱动已被多次更新，以修复在弹出发生在不合适时机时出现的竞态条件。修改此代码时必须小心，以防重新引入此类问题。参见代码中的注释
### 中断分配


Hyper-V 虚拟 PCI 驱动支持使用 MSI、多 MSI MSI-X vPCI 设备。为将接收特MSI MSI-X 消息
中断的客vCPU 进行分配是复杂的，因为这涉及Linux IRQ 的设置如何映射到 Hyper-V 接口对于MSI MSI-X 情况，Linux 调用 hv_compose_msi_msg() 两次，第一次调用包含伪 vCPU，第二次
调用包含真实vCPU。此外，最后调hv_irq_unmask()（在 x86 上）或设GICD 寄存器（arm64 上）
以再次指定真vCPU。这三次调用每一次都Hyper-V 交互，Hyper-V 必须在中断被转发到客VM 之前
决定哪个物理 CPU 应接收该中断。不幸的是，Hyper-V 的决策过程有些受限，可能导致物理中断集中在单CPU 上，从而造成性能瓶颈。关于如何解决这个问题，请参hv_compose_msi_req_get_cpu() 函数上方
详尽的注释
Hyper-V 虚拟 PCI 驱动irq_chip.irq_compose_msi_msg 函数实现hv_compose_msi_msg()。不幸的是，
Hyper-V 上，该实现需要向 Hyper-V 主机发送一VMBus 消息，并等待一条指示收到回复消息的中断由于 irq_chip.irq_compose_msi_msg 可以在持IRQ 锁的情况下被调用，执行正常的睡眠直到被中唤醒是行不通的。相反，hv_compose_msi_msg() 必须发VMBus 消息，然后轮询完成消息。更复杂的是vPCI 设备可能在轮询进行期间被弹出/撤销，因此也必须检测这种情况。关于这一非常棘手的区域，请参代码中的注释
Hyper-V 虚拟 PCI 驱动（pci-hyperv.c）中的大部分代码适用于运行在 x86 arm64 架构上的 Hyper-V
Linux 客机。但在中断分配的管理方式上存在差异。在 x86 上，客机中的 Hyper-V 虚拟 PCI 驱动必须
发起一hypercall 来告Hyper-V 哪个客机 vCPU 应被每个 MSI/MSI-X 中断打断，以x86_vector IRQ
域为该中断挑选的 x86 中断向量号。该 hypercall hv_arch_irq_unmask() 发出。在 arm64 上，Hyper-V
虚拟 PCI 驱动管理为每MSI/MSI-X 中断分配一SPI。Hyper-V 虚拟 PCI 驱动将分配的 SPI 存储架构相关GICD 寄存器中（Hyper-V 对其进行了模拟），因此与 x86 不同，不需hypercall。Hyper-V
不支持在 arm64 客机 VM 中将 LPI 用于 vPCI 设备，因为它不模GICv3 ITS
Linux 中的 Hyper-V 虚拟 PCI 驱动支持那些驱动创建受管理或未受管理 Linux IRQ vPCI 设备。如果对
一个未受管IRQ smp_affinity 通过 /proc/irq 接口被更新，Hyper-V 虚拟 PCI 驱动会被调用告诉 Hyper-V 主机更改中断目标，一切都能正常工作。然而，x86 上，如果 x86_vector IRQ 域由CPU 上向量耗尽而需要重新分配一个中断向量，则没有路径通知 Hyper-V 主机这一变更，于是就会出问题所幸，客机 VM 运行在受限的设备环境中，不会用尽 CPU 上的所有向量。由于这种问题只是理论上的关而非实际关切，因此一直未被处理
### DMA


默认情况下，Hyper-V 在创VM 时将客机 VM 的所有内存锁定在主机中，并将物理 IOMMU 编程为允VM
对其所有内存拥DMA 访问权限。因此，PCI 设备分配VM 并允许客机操作系统对 DMA 传输进行
编程是安全的。物IOMMU 防止恶意的客机发起指向属于主机或主机上其VM 的内存的 DMA。从 Linux
客机的角度看，此DMA 传输处于“直接”模式，因为 Hyper-V 不在客机中提供虚IOMMU
Hyper-V 假设物理 PCI 设备总是执行缓存一致的 DMA。在 x86 上运行时，这种行为是架构所要求的。在
arm64 上运行时，架构允许缓存一致和非缓存一致的设备，每个设备的行为ACPI DSDT 中指定。但PCI 设备被分配给客机 VM 时，该设备不会出现在 DSDT 中，因此 Hyper-V VMBus 驱动将缓存一致信息ACPI DSDT 中的 VMBus 节点传播到所VMBus 设备，包vPCI 设备（因为它们作VMBus 设备
PCI 设备具有双重身份）。参vmbus_dma_configure()。当Hyper-V 版本总是表明 VMBus 是缓一致的，因arm64 上的 vPCI 设备总是被标记为缓存一致，CPU 在执dma_map/unmap_*() 调用时不执行任何同步操作
### vPCI 协议版本


如前所述，vPCI 设备建立和拆除过程中，消息经 VMBus 通道Hyper-V 主机Linux 客机中的
Hyper-V vPCI 驱动之间传递。某些消息在较新版本Hyper-V 中已被修订，因此客机和主机必须就将要
使用vPCI 协议版本达成一致。该版本在通过 VMBus 通道建立通信时协商。参hv_pci_protocol_negotiation()。较新版本的协议扩展了对超过 64 vCPU VM 的支持，并提供关vPCI 设备的额外信息，例如它在底层硬件中最紧密关联的客机虚NUMA 节点
### 瀹㈡満 NUMA 鑺傜偣浜插拰鎬。

vPCI 协议版本提供时，vPCI 设备的客NUMA 节点亲和性会作为 Linux 设备信息的一部分被存储，
供后续由 Linux 驱动使用。参hv_pci_assign_numa_node()。如果协商的协议版本不支持主机提NUMA
亲和性信息，Linux 客机将该设备NUMA 节点默认0。但即使协商的协议版本包NUMA 亲和性信息，
主机提供此类信息的能力也取决于某些主机配置选项。如果客机收NUMA 节点"0"，它可能表示 NUMA
节点 0，也可能表示“无信息可用”。不幸的是，从客机侧无法区分这两种情况
### CoCo VM 中的 PCI 配置空间访问


Linux PCI 设备驱动使用 Linux PCI 子系统提供的一组标准函数来访问 PCI 配置空间。在 Hyper-V 客机
中，这些标准函数映射Hyper-V 虚拟 PCI 驱动中的 hv_pcifront_read_config() hv_pcifront_write_config() 函数。在普VM 中，这些 hv_pcifront_*() 函数直接访问 PCI 配置空间这些访问会陷入（trap）到 Hyper-V 进行处理。但CoCo VM 中，内存加密阻止 Hyper-V 读取客机指令
流来模拟该访问，因此 hv_pcifront_*() 函数必须发起带有显式参数hypercall，以描述要进行的访问
### 配置块后通道


Hyper-V 主机Linux 中的 Hyper-V 虚拟 PCI 驱动共同实现了一条主机与客机之间的非标准后通道
（back-channel）通信路径。该后通道路径使用经与vPCI 设备关联VMBus 通道发送的消息。函hyperv_read_cfg_blk() hyperv_write_cfg_blk() 是提供给 Linux 内核其他部分的主要接口。截至撰本文时，这些接口仅被 Mellanox mlx5 驱动用于在运行于 Azure 公有云的 Hyper-V 主机上传递诊断数据函数 hyperv_read_cfg_blk() hyperv_write_cfg_blk() 在一个独立模块（pci-hyperv-intf.c，位CONFIG_PCI_HYPERV_INTERFACE 下）中实现，在非 Hyper-V 环境中运行时有效地将它们置为空操作