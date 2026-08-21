
## Nitro Enclaves（NE，Nitro 飞地

## 概述


Nitro Enclaves（NE）是 Amazon Elastic Compute Cloud（EC2）的一项新能力允许客户EC2 实例中切分出隔离的计算环[^1^]
例如，一个处理敏感数据并运行VM 中的应用程序，可以与运行在同一VM 中的其他应用程序分离开来。然后该应用程序运行在一个独立的 VM 中，
而不是主 VM，即一enclave（飞地）。它与生成它VM 并行运行。这种设符合低延迟应用程序的需求
上游 Linux 内核中可用的 NE 内核驱动当前支持的架构是 x86 ARM64
enclave 分配的资源，例如内存CPU，是从主 VM 中切分出来的。每enclave 映射到主 VM 中运行的一个进程，该进程通过 ioctl 接口NE 内核驱动
通信
从这个意义上说，有两个组件：

1. 一enclave 抽象进程——运行在VM 客户机中的用户空间进程，它使   NE 驱动提供ioctl 接口来生成一enclave VM（即下面2）
   有一个暴露给VM NE 仿真 PCI 设备。该PCI 设备的驱动包含在
   NE 驱动中
   ioctl 逻辑映射PCI 设备命令，例NE_START_ENCLAVE ioctl 映射   一enclave 启动 PCI 命令。然PCI 设备命令被转换为在管理程序一   采取的动作；即运行主 VM 所在主机上Nitro 管理程序。Nitro 管理程序
   基于核心 KVM 技术
2. enclave 本身——一个运行在与生成它的主 VM 相同主机上的 VM。内存和
   CPU 从主 VM 中切分出来，并专用于 enclave VM。enclave 没有附加的持久存储
从主 VM 中切分出来并enclave 的内存区域需要是对齐2 MiB / 1 GiB 物理
连续内存区域（或此大小的倍数，例8 MiB）。内存可以通过例如从用户空间使hugetlbfs 来分[^2^][^3^][^7^]。enclave 的内存大小至少需64 MiBenclave 的内存和 CPU 需要来自同一NUMA 节点
enclave 运行在专用核心上。CPU 0 及其 CPU 兄弟（sibling）需要保留给VM
可用。必须由具有管理员能力的用户NE 目的设置一CPU 池。有CPU 池格式，
请参见内核文[^4^] 中的 cpu 列表一节
enclave 通过本地通信通道使用 virtio-vsock [^5^] 与主 VM 通信。主 VM virtio-pci vsock 仿真设备，enclave VM virtio-mmio vsock 仿真设备vsock 设备使用 eventfd 进行信号通知。enclave VM 看到通常的接口——本APIC
IOAPIC——以virtio-vsock 设备获取中断。virtio-mmio 设备被放置在典型
4 GiB 以下的的内存中
enclave 中运行的应用程序需要与其将enclave VM 中运行的 OS（例如内核ramdisk、init）一起打包进一enclave 镜像中。enclave VM 有自己的内核并遵标准 Linux 启动协议 [^6^][^8^]
内核 bzImage、内核命令行、ramdisk(s) Enclave Image Format（EIF，enclave
镜像格式）的一部分；外加一EIF 头，包含诸如魔数、eif 版本、镜像大小和
CRC 等元数据
为整enclave 镜像（EIF）、内核和 ramdisk(s) 计算哈希值。这用于例如检加载enclave VM 中的 enclave 镜像就是预期要运行的那个
这些加密度量（crypto measurement）被包含在一个由 Nitro 管理程序生成的签证明文档中，并进一步用于证enclave 的身份；KMS NE 集成并会检查该证明
文档的服务的示例
enclave 镜像（EIF）被加载enclave 内存8 MiB 偏移处。enclave 中的 init
进程连接到主 VM vsock CID 和一个预定义端口—000——以发送一个心跳—xb7。该机制用于在主 VM 中检enclave 是否已启动。主 VM CID 3
如果 enclave VM 崩溃或优雅退出，NE 驱动会收到一个中断事件。该事件通过轮询
通知机制进一步发送给运行在主 VM 中的用户空间 enclave 进程。然后用户空enclave 进程可以退出
[^1^] https://aws.amazon.com/ec2/nitro/nitro-enclaves/
[^2^] https://www.kernel.org/doc/html/latest/admin-guide/mm/hugetlbpage.html
[^3^] https://lwn.net/Articles/807108/
[^4^] https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html
[^5^] https://man7.org/linux/man-pages/man7/vsock.7.html
[^6^] https://www.kernel.org/doc/html/latest/x86/boot.html
[^7^] https://www.kernel.org/doc/html/latest/arm64/hugetlbpage.html
[^8^] https://www.kernel.org/doc/html/latest/arm64/booting.html
