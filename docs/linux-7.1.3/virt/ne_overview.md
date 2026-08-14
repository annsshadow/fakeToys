
## Nitro Enclaves（NE，Nitro 飞地）


## 概述


Nitro Enclaves（NE）是 Amazon Elastic Compute Cloud（EC2）的一项新能力，
允许客户在 EC2 实例中切分出隔离的计算环境 [^1^]。

例如，一个处理敏感数据并运行在 VM 中的应用程序，可以与运行在同一个
VM 中的其他应用程序分离开来。然后该应用程序运行在一个独立的 VM 中，
而不是主 VM，即一个 enclave（飞地）。它与生成它的 VM 并行运行。这种设置
符合低延迟应用程序的需求。

上游 Linux 内核中可用的 NE 内核驱动当前支持的架构是 x86 和 ARM64。

为 enclave 分配的资源，例如内存和 CPU，是从主 VM 中切分出来的。每个
enclave 映射到主 VM 中运行的一个进程，该进程通过 ioctl 接口与 NE 内核驱动
通信。

从这个意义上说，有两个组件：

1. 一个 enclave 抽象进程——运行在主 VM 客户机中的用户空间进程，它使用
   NE 驱动提供的 ioctl 接口来生成一个 enclave VM（即下面的 2）。

   有一个暴露给主 VM 的 NE 仿真 PCI 设备。该新 PCI 设备的驱动包含在
   NE 驱动中。

   ioctl 逻辑映射到 PCI 设备命令，例如 NE_START_ENCLAVE ioctl 映射到
   一个 enclave 启动 PCI 命令。然后 PCI 设备命令被转换为在管理程序一侧
   采取的动作；即运行主 VM 所在主机上的 Nitro 管理程序。Nitro 管理程序
   基于核心 KVM 技术。

2. enclave 本身——一个运行在与生成它的主 VM 相同主机上的 VM。内存和
   CPU 从主 VM 中切分出来，并专用于 enclave VM。enclave 没有附加的持久存储。

从主 VM 中切分出来并给 enclave 的内存区域需要是对齐的 2 MiB / 1 GiB 物理
连续内存区域（或此大小的倍数，例如 8 MiB）。内存可以通过例如从用户空间使用
hugetlbfs 来分配 [^2^][^3^][^7^]。enclave 的内存大小至少需要 64 MiB。
enclave 的内存和 CPU 需要来自同一个 NUMA 节点。

enclave 运行在专用核心上。CPU 0 及其 CPU 兄弟（sibling）需要保留给主 VM
可用。必须由具有管理员能力的用户为 NE 目的设置一个 CPU 池。有关 CPU 池格式，
请参见内核文档 [^4^] 中的 cpu 列表一节。

enclave 通过本地通信通道使用 virtio-vsock [^5^] 与主 VM 通信。主 VM 有
virtio-pci vsock 仿真设备，而 enclave VM 有 virtio-mmio vsock 仿真设备。
vsock 设备使用 eventfd 进行信号通知。enclave VM 看到通常的接口——本地 APIC
和 IOAPIC——以从 virtio-vsock 设备获取中断。virtio-mmio 设备被放置在典型
4 GiB 以下的的内存中。

在 enclave 中运行的应用程序需要与其将在 enclave VM 中运行的 OS（例如内核、
ramdisk、init）一起打包进一个 enclave 镜像中。enclave VM 有自己的内核并遵循
标准 Linux 启动协议 [^6^][^8^]。

内核 bzImage、内核命令行、ramdisk(s) 是 Enclave Image Format（EIF，enclave
镜像格式）的一部分；外加一个 EIF 头，包含诸如魔数、eif 版本、镜像大小和
CRC 等元数据。

为整个 enclave 镜像（EIF）、内核和 ramdisk(s) 计算哈希值。这用于例如检查
加载到 enclave VM 中的 enclave 镜像就是预期要运行的那个。

这些加密度量（crypto measurement）被包含在一个由 Nitro 管理程序生成的签名
证明文档中，并进一步用于证明 enclave 的身份；KMS 是 NE 集成并会检查该证明
文档的服务的示例。

enclave 镜像（EIF）被加载到 enclave 内存的 8 MiB 偏移处。enclave 中的 init
进程连接到主 VM 的 vsock CID 和一个预定义端口——9000——以发送一个心跳值
——0xb7。该机制用于在主 VM 中检查 enclave 是否已启动。主 VM 的 CID 是 3。

如果 enclave VM 崩溃或优雅退出，NE 驱动会收到一个中断事件。该事件通过轮询
通知机制进一步发送给运行在主 VM 中的用户空间 enclave 进程。然后用户空间
enclave 进程可以退出。

[^1^] https://aws.amazon.com/ec2/nitro/nitro-enclaves/
[^2^] https://www.kernel.org/doc/html/latest/admin-guide/mm/hugetlbpage.html
[^3^] https://lwn.net/Articles/807108/
[^4^] https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html
[^5^] https://man7.org/linux/man-pages/man7/vsock.7.html
[^6^] https://www.kernel.org/doc/html/latest/x86/boot.html
[^7^] https://www.kernel.org/doc/html/latest/arm64/hugetlbpage.html
[^8^] https://www.kernel.org/doc/html/latest/arm64/booting.html
