
## 亲和性受管中


IRQ 核心提供根据指定 CPU 亲和性来管理中断的支持。在正常操作中，一个中断与某个特定
CPU 关联。如果该 CPU 离线，中断会被迁移到另一个在CPU

具有大量中断向量的设备会占用可用的向量空间。例如，在至少有 128 CPU 的系统上，一
具有 128 I/O 队列NVMe 设备通常每个队列请求一个中断。因此两个这样的设备请求 256
个中断。在 x86 上，中断向量空间 notoriously 很低，每CPU 仅提256 个向量，且内核保
了其中一部分，进一步减少了可用于设备中断的数量。在实践中这不是问题，因为中断被分布
许多 CPU 上，因此每个 CPU 只接收少量向量

然而，在系统挂起期间，所有次CPU 都离线，所有中断都被迁移到唯一在线CPU。这可能
耗尽CPU 上可用的中断向量，并导致挂起操作失败

亲和性受管中断解决了这一限制。每个中断被赋一CPU 亲和性掩码，指定该中断可以被定向到的
CPU 集合。当掩码中的一CPU 离线时，中断被移动到掩码中的下一CPU。如果掩码中的最后一
CPU 离线，该中断被关闭。使用亲和性受管中断的驱动必须确保在中断被禁用之前相关的队列已静止
以免产生进一步的中断。当亲和性掩码中的一CPU 重新上线时，该中断被重新启用

### 实现


设备必须提供每实例中断，例如 NVMe 这类存储设备的每 I/O 队列中断。驱动使struct
irq_affinity 分配具有所需亲和性设置的中断向量。对MSI-X 设备，这是通过带有
PCI_IRQ_AFFINITY 标志pci_alloc_irq_vectors_affinity() 完成的

基于提供的亲和性信息，IRQ 核心尝试把中断均匀地散布到整个系统。亲和性掩码在这一分配步骤
计算，但最终的 IRQ 分配是在调用 request_irq() 时执行的

### 隔离CPU


受管中断的亲和性完全在内核中处理，无法通过 /proc 接口从用户空间修改。isolcpus 启动选项
managed_irq 子参数指定一CPU 掩码，受管中断应当尽量避免。这种隔离是尽力而为的，仅当自动
分配的中断掩码也包含被避开掩码之外的在CPU 时才适用。如果请求的掩码只包含隔离的 CPU
则该设置不起作用

列在避开掩码中的 CPU 仍然是中断亲和性掩码的一部分。这意味着如果所有非隔离CPU 离线
隔离CPU 仍然在线，该中断会被分配给其中一个隔离的 CPU

以下示例假设一个具8 CPU 的系统

- 一QEMU 实例"-device virtio-scsi-pci" 启动。该 MSI-X 设备暴露 11 个中断：3 
  "管理"中断8 "队列"中断。驱动请求这 8 个队列中断，每个都刚好亲和于一CPU
  如果CPU 离线，该中断被关闭

```

    /proc/irq/48/effective_affinity_list:7
    /proc/irq/48/smp_affinity_list:7

  This indicates that the interrupt is served only by CPU7. Shutting down CPU7
  does not migrate the interrupt to another CPU::

    /proc/irq/48/effective_affinity_list:0
    /proc/irq/48/smp_affinity_list:7

  This can be verified via the debugfs interface
  (/sys/kernel/debug/irq/irqs/48). The dstate field will include
  IRQD_IRQ_DISABLED, IRQD_IRQ_MASKED and IRQD_MANAGED_SHUTDOWN.

```
- 一QEMU 实例"-device virtio-scsi-pci,num_queues=2" 启动，并且内核命令行包含
  "irqaffinity=0,1 isolcpus=domain,2-7 isolcpus=managed_irq,1-3,5-7"。该 MSI-X 设备暴露
  5 个中断：3 个管理中断和 2 个队列中断。管理中断遵irqaffinity= 设置
```

    /proc/irq/47/effective_affinity_list:0
    /proc/irq/47/smp_affinity_list:0-3
    /proc/irq/48/effective_affinity_list:4
    /proc/irq/48/smp_affinity_list:4-7

  The two queue interrupts are evenly distributed. Interrupt 48 is placed on CPU4
  because the managed_irq mask avoids CPUs 5鈥? when possible.

  Replacing the managed_irq argument with "isolcpus=managed_irq,1-3,4-5,7"
  results in::

    /proc/irq/48/effective_affinity_list:6
    /proc/irq/48/smp_affinity_list:4-7

  Interrupt 48 is now served on CPU6 because the system avoids CPUs 4, 5 and
  7. If CPU6 is taken offline, the interrupt migrates to one of the "isolated"
  CPUs::

    /proc/irq/48/effective_affinity_list:7
    /proc/irq/48/smp_affinity_list:4-7

  The interrupt is shut down once all CPUs listed in its smp_affinity mask are
  offline.

```