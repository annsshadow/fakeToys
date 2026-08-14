
## 如何编写 Linux PCI 驱动

:Authors: - Martin Mares <mj@ucw.cz>
          - Grant Grundler <grundler@parisc-linux.org>

PCI 的世界广袤无垠，且充满了（大多令人不快的）意外。由于每个 CPU 架构实现了不同的芯片组，且 PCI 设备有不同的需求（呃，“特性”），结果是 Linux 内核中的 PCI 支持并不像人们所希望的那样简单。这篇短文试图向所有潜在的驱动作者介绍用于 PCI 设备驱动的 Linux API。

更完整的资源是 Jonathan Corbet、Alessandro Rubini 和 Greg Kroah-Hartman 所著的《Linux Device Drivers》第三版。LDD3 可在以下地址免费获取（基于知识共享许可协议）：
https://lwn.net/Kernel/LDD3/。

不过请记住，所有文档都难免会“过时腐烂”（bit rot）。如果实际情况与本文描述不符，请以源代码为准。

请将关于 Linux PCI API 的问题/评论/补丁发送到“Linux PCI”<linux-pci@atrey.karlin.mff.cuni.cz> 邮件列表。

## PCI 驱动的结构

PCI 驱动通过 pci_register_driver()“发现”系统中的 PCI 设备。实际上，情况恰好相反。当 PCI 通用代码发现一个新设备时，具有匹配“描述”的驱动将被通知。细节见下文。

pci_register_driver() 将大部分设备探测工作留给 PCI 层，并支持设备的在线插入/移除[从而支持在单一驱动中支持热插拔 PCI、CardBus 和 Express-Card]。pci_register_driver() 调用需要传入一个函数指针表，从而决定了驱动的高层结构。

一旦驱动了解某个 PCI 设备并取得所有权，驱动通常需要执行以下初始化：

  - 使能设备
  - 请求 MMIO/IOP 资源
  - 设置 DMA 掩码大小（用于一致性 DMA 与流式 DMA 两者）
  - 分配并初始化共享控制数据（pci_allocate_coherent()）
  - 访问设备配置空间（如需要）
  - 注册 IRQ 处理程序（request_irq()）
  - 初始化非 PCI 部分（即芯片的 LAN/SCSI/等部分）
  - 使能 DMA/处理引擎

当使用完设备、且可能需要卸载模块时，驱动需要执行以下步骤：

  - 禁止设备产生 IRQ
  - 释放 IRQ（free_irq()）
  - 停止所有 DMA 活动
  - 释放 DMA 缓冲区（流式与一致性两者）
  - 从其他子系统注销（例如 scsi 或 netdev）
  - 释放 MMIO/IOP 资源
  - 禁用设备

这些主题大多在以下小节中涵盖。其余部分请参阅 LDD3 或 <linux/pci.h>。

如果未配置 PCI 子系统（未设置 CONFIG_PCI），下面描述的多数 PCI 函数被定义为内联函数，要么完全为空，要么仅返回适当的错误码，以避免驱动中出现大量 ifdef。

## pci_register_driver() 调用

PCI 设备驱动在其初始化期间调用 `pci_register_driver()`，并传入一个描述该驱动的结构体指针（`struct pci_driver`）：

   :functions: pci_driver

ID 表是一个以全零条目结尾的 `struct pci_device_id` 条目数组。通常推荐使用 static const 定义。

   :functions: pci_device_id

大多数驱动只需要 `PCI_DEVICE()` 或 `PCI_DEVICE_CLASS()` 来建立 pci_device_id 表。

新的 PCI ID 可以在运行时添加到设备驱动的 pci_ids 表中
```

  echo "vendor device subvendor subdevice class class_mask driver_data" > \
  /sys/bus/pci/drivers/{driver}/new_id

```
所有字段都以十六进制值传入（不带前导 0x）。vendor 和 device 字段是必填的，其余为可选。用户只需传入必要的可选字段数量：

  - subvendor 和 subdevice 字段默认为 PCI_ANY_ID（FFFFFFFF）
  - class 和 classmask 字段默认为 0
  - driver_data 默认为 0UL。
  - override_only 字段默认为 0。

注意，driver_data 必须与驱动中定义的任何 pci_device_id 条目所使用的值匹配。如果所有 pci_device_id 条目都具有非零的 driver_data 值，这将使 driver_data 字段成为必填项。

一旦添加，对于任何在其（新更新的）pci_ids 表中列出的、未被认领的 PCI 设备，都会调用驱动的 probe 例程。

当驱动退出时，它只需调用 pci_unregister_driver()，PCI 层会自动对该驱动处理的所有设备调用 remove 钩子。

### 驱动函数/数据的“属性”

请在适当的位置标记初始化与清理函数（相应的宏定义在 <linux/init.h> 中）：

	======		=================================================
	__init		初始化代码。在驱动初始化后被丢弃。
	__exit		退出代码。对非模块化驱动被忽略。
	======		=================================================

关于何时/何处使用上述属性的提示：
 - module_init()/module_exit() 函数（以及仅从这些函数调用的所有初始化函数）应标记为 __init/__exit。

 - 不要标记 struct pci_driver。

 - 如果不确定的标记用法，请勿标记函数。不标记函数也好过错误标记函数。

## 如何手动查找 PCI 设备

PCI 驱动不使用 pci_register_driver() 接口来搜索 PCI 设备，应当有非常充分的理由。PCI 设备由多个驱动控制的主要原因是，一个 PCI 设备实现了几种不同的硬件服务。例如组合了串口/并口/软盘控制器。

可以使用以下结构进行手动搜索：

```

	struct pci_dev *dev = NULL;
	while (dev = pci_get_device(VENDOR_ID, DEVICE_ID, dev))
		configure_device(dev);

```
```

	pci_get_class(CLASS_ID, dev)

```
```

	pci_get_subsys(VENDOR_ID,DEVICE_ID, SUBSYS_VENDOR_ID, SUBSYS_DEVICE_ID, dev).

```
你可以使用常量 PCI_ANY_ID 作为 VENDOR_ID 或 DEVICE_ID 的通配符替代。例如，这允许搜索某个特定 vendor 的任何设备。

这些函数是热插拔安全的。它们会递增所返回的 pci_dev 的引用计数。你最终（可能在模块卸载时）必须通过调用 pci_dev_put() 来递减这些设备的引用计数。

## 设备初始化步骤

如简介中所述，大多数 PCI 驱动需要以下步骤进行设备初始化：

  - 使能设备
  - 请求 MMIO/IOP 资源
  - 设置 DMA 掩码大小（用于一致性 DMA 与流式 DMA 两者）
  - 分配并初始化共享控制数据（pci_allocate_coherent()）
  - 访问设备配置空间（如需要）
  - 注册 IRQ 处理程序（request_irq()）
  - 初始化非 PCI 部分（即芯片的 LAN/SCSI/等部分）
  - 使能 DMA/处理引擎。

驱动可以在任何时间访问 PCI 配置空间寄存器。（嗯，几乎可以。运行 BIST 时，配置空间可能消失……但这只会导致 PCI 总线主设备中止（Bus Master Abort），配置读取将返回垃圾数据）。

### 使能 PCI 设备

在触碰任何设备寄存器之前，驱动需要通过调用 pci_enable_device() 来使能 PCI 设备。这将：

  - 如果设备处于挂起状态，将其唤醒，
  - 分配设备的 I/O 和内存区域（如果 BIOS 没有分配），
  - 分配一个 IRQ（如果 BIOS 没有分配）。

   pci_enable_device() 可能失败！请检查返回值。

   OS BUG：我们在使能这些资源之前不检查资源分配。如果我们能在调用 pci_enable_device() 之前调用 pci_request_resources()，顺序会更有意义。目前，当两个设备被分配了相同范围时，设备驱动无法检测到该 bug。这不是一个常见问题，也不太可能很快得到修复。

   这一点此前已经讨论过，但截至 2.6.19 尚未更改：
   https://lore.kernel.org/r/20060302180025.GC28895@flint.arm.linux.org.uk/

pci_set_master() 将通过设置 PCI_COMMAND 寄存器中的总线主控位来使能 DMA。如果 BIOS 将其设置为某个无效值，它还会修复延迟定时器（latency timer）值。pci_clear_master() 将通过清除总线主控位来禁用 DMA。

如果 PCI 设备可以使用 PCI Memory-Write-Invalidate 事务，调用 pci_set_mwi()。这将使能 Mem-Wr-Inval 的 PCI_COMMAND 位，并确保缓存行大小（cache line size）寄存器被正确设置。请检查 pci_set_mwi() 的返回值，因为并非所有架构或芯片组都支持 Memory-Write-Invalidate。或者，如果 Mem-Wr-Inval 有则更好但并非必需，调用 pci_try_set_mwi() 让系统尽最大努力去使能 Mem-Wr-Inval。

### 请求 MMIO/IOP 资源

内存（MMIO）和 I/O 端口地址不应直接从 PCI 设备配置空间读取。请使用 pci_dev 结构中的值，因为 PCI“总线地址”可能已被架构/芯片组特定的内核支持重映射为“主机物理”地址。

有关如何访问设备寄存器或设备内存，请参阅 Documentation/driver-api/io-mapping.rst。

设备驱动需要调用 pci_request_region() 以确认没有其他设备已经在使用相同的地址资源。反之，驱动应在调用 pci_disable_device() 之后调用 pci_release_region()。其意图是防止两个设备在同一地址范围上冲突。

   请参阅上面的 OS BUG 注释。当前（2.6.19），驱动只能在调用 pci_enable_device() 之后才能确定 MMIO 和 IO Port 资源的可用性。

pci_request_region() 的通用变体是 request_mem_region()（用于 MMIO 范围）和 request_region()（用于 IO Port 范围）。将这些用于未被“普通”PCI BAR 描述的地址资源。

另见下面的 pci_request_selected_regions()。

### 设置 DMA 掩码大小

   如果以下内容难以理解，请参阅 Documentation/core-api/dma-api.rst。本节只是提醒驱动需要指明设备的 DMA 能力，并非 DMA 接口的权威来源。

虽然所有驱动都应显式指明 PCI 总线主控的 DMA 能力（例如 32 位或 64 位），但对于具有超过 32 位总线主控流式数据能力的设备，驱动需要通过调用带有适当参数的 dma_set_mask() 来“注册”此能力。一般而言，这允许在系统 RAM 存在于 4G 以上_物理_地址的系统中进行更高效的 DMA。

所有 PCI-X 和 PCIe 兼容设备的驱动必须调用 dma_set_mask()，因为它们是 64 位 DMA 设备。

类似地，如果设备可以通过调用 dma_set_coherent_mask() 直接寻址位于 4G 物理地址以上的系统 RAM 中的“一致性内存”，驱动也必须“注册”此能力。同样，这包括所有 PCI-X 和 PCIe 兼容设备的驱动。许多 64 位“PCI”设备（PCI-X 之前）和一些 PCI-X 设备对于有效载荷（“流式”）数据具有 64 位 DMA 能力，但对于控制（“一致性”）数据则没有。

### 建立共享控制数据

一旦设置了 DMA 掩码，驱动就可以分配“一致性”（也称共享）内存。有关 DMA API 的完整描述，请参阅 Documentation/core-api/dma-api.rst。本节只是提醒需要在设备上使能 DMA 之前完成这一步。

### 初始化设备寄存器

某些驱动需要编程特定的“能力”字段，或初始化/重置其他“厂商特定”寄存器。例如清除挂起的中断。

### 注册 IRQ 处理程序

虽然调用 request_irq() 是此处描述的最后一步，但这通常只是初始化设备的另一个中间步骤。这一步通常可以推迟到设备被打开使用时。

所有 IRQ 线的中断处理程序都应使用 IRQF_SHARED 注册，并使用 devid 将 IRQ 映射到设备（请记住所有 PCI IRQ 线都可以共享）。

request_irq() 会将中断处理程序和设备句柄与中断号关联。历史上中断号代表从 PCI 设备运行到中断控制器的 IRQ 线。对于 MSI 和 MSI-X（详见下文），中断号是一个 CPU“向量”。

request_irq() 还会使能中断。在注册中断处理程序之前，请确保设备已静默（quiesced）且没有挂起的中断。

MSI 和 MSI-X 是 PCI 能力。两者都是“消息信号中断”（Message Signaled Interrupts），通过向 Local APIC 进行 DMA 写操作将中断投递给 CPU。MSI 与 MSI-X 的根本区别在于多个“向量”的分配方式。MSI 需要连续的向量块，而 MSI-X 可以分配多个独立的向量。

可以在调用 request_irq() 之前，通过带 PCI_IRQ_MSI 和/或 PCI_IRQ_MSIX 标志调用 pci_alloc_irq_vectors() 来使能 MSI 能力。这会导致 PCI 支持将 CPU 向量数据编程到 PCI 设备的能力寄存器中。许多架构、芯片组或 BIOS 不支持 MSI 或 MSI-X，因此仅带 PCI_IRQ_MSI 和 PCI_IRQ_MSIX 标志的 pci_alloc_irq_vectors 调用会失败，所以应尽量同时指定 PCI_IRQ_INTX。

对于 MSI/MSI-X 和传统 INTx 具有不同中断处理程序的驱动，应在调用 pci_alloc_irq_vectors 之后，根据 pci_dev 结构中的 msi_enabled 和 msix_enabled 标志选择正确的处理程序。

使用 MSI 至少有两个很好的理由：

1) 根据定义，MSI 是一个独占的中断向量。这意味着中断处理程序无需验证是其设备引发了中断。

2) MSI 避免了 DMA/IRQ 竞争条件。当 MSI 被投递时，到主机内存的 DMA 保证对主机 CPU 可见。这对于数据一致性和避免陈旧的控制数据都很重要。这一保证允许驱动省略用于刷新 DMA 流的 MMIO 读取。

有关 MSI/MSI-X 用法的示例，请参阅 drivers/infiniband/hw/mthca/ 或 drivers/net/tg3.c。

## PCI 设备关闭

当一个 PCI 设备驱动被卸载时，需要执行以下大部分步骤：

  - 禁止设备产生 IRQ
  - 释放 IRQ（free_irq()）
  - 停止所有 DMA 活动
  - 释放 DMA 缓冲区（流式与一致性两者）
  - 从其他子系统注销（例如 scsi 或 netdev）
  - 禁止设备响应 MMIO/IO Port 地址
  - 释放 MMIO/IO Port 资源

### 在设备上停止 IRQ

如何做到这一点是芯片/设备相关的。如果不这样做，则在（且仅在）IRQ 与另一个设备共享的情况下，会存在“尖叫中断”（screaming interrupt）的可能性。

当共享的 IRQ 处理程序被“解除挂钩”时，使用同一 IRQ 线的其余设备仍需要该 IRQ 保持使能。因此，如果“解除挂钩”的设备断言（assert）了 IRQ 线，系统会以为是其余某个设备断言了 IRQ 线而做出响应。由于其余设备都不会处理该 IRQ，系统将“挂起”直到它判定该 IRQ 不会被处理并屏蔽该 IRQ（在 100,000 次迭代之后）。一旦共享 IRQ 被屏蔽，其余设备将停止正常工作。这不是一个好的状况。

这是另一个在可用时使用 MSI 或 MSI-X 的理由。MSI 和 MSI-X 被定义为独占中断，因此不会受到“尖叫中断”问题的影响。

### 释放 IRQ

一旦设备被静默（不再有 IRQ），就可以调用 free_irq()。一旦任何挂起的 IRQ 被处理，此函数将返回控制权，“解除挂钩”驱动在该 IRQ 上的 IRQ 处理程序，并且在没有其他人使用它时最终释放该 IRQ。

### 停止所有 DMA 活动

在尝试释放 DMA 控制数据之前停止所有 DMA 操作极其重要。不这样做可能导致内存损坏、挂起，并在某些芯片组上导致硬崩溃。

在停止 IRQ 之后再停止 DMA 可以避免 IRQ 处理程序可能重启 DMA 引擎的竞争条件。

虽然这一步听起来显而易见且简单，但几个“成熟”的驱动过去并未正确处理这一步。

### 释放 DMA 缓冲区

一旦 DMA 停止，首先清理流式 DMA。即解除数据缓冲区的映射，并将其归还给“上游”所有者（如果存在）。

然后清理包含控制数据的“一致性”缓冲区。

有关解除映射接口的详细信息，请参阅 Documentation/core-api/dma-api.rst。

### 从其他子系统注销

大多数底层 PCI 设备驱动支持其他一些子系统，如 USB、ALSA、SCSI、NetDev、Infiniband 等。请确保你的驱动没有从该子系统中丢失资源。如果发生这种情况，典型的症状是当子系统尝试调用已卸载的驱动时产生 Oops（panic）。

### 禁止设备响应 MMIO/IO Port 地址

对 MMIO 或 IO Port 资源执行 io_unmap()，然后调用 pci_disable_device()。这是 pci_enable_device() 的对称相反操作。在调用 pci_disable_device() 之后不要访问设备寄存器。

### 释放 MMIO/IO Port 资源

调用 pci_release_region() 将 MMIO 或 IO Port 范围标记为可用。不这样做通常会导致无法重新加载驱动。

## 如何访问 PCI 配置空间

你可以使用 `pci_(read|write)_config_(byte|word|dword)` 来访问由 `struct pci_dev *` 表示的设备的配置空间。所有这些函数在成功时返回 0，或在失败时返回一个错误码（`PCIBIOS_...`），该错误码可通过 pcibios_strerror 转换为文本字符串。大多数驱动期望对有效 PCI 设备的访问不会失败。

如果没有可用的 struct pci_dev，你可以调用 `pci_bus_(read|write)_config_(byte|word|dword)` 来访问该总线上给定的设备和功能。

如果你访问配置头标准部分的字段，请使用 <linux/pci.h> 中声明的位置和位的符号名称。

如果你需要访问扩展 PCI 能力（Extended PCI Capability）寄存器，只需为特定能力调用 pci_find_capability()，它会为你找到相应的寄存器块。

## 其他有趣的函数

=============================	================================================
pci_get_domain_bus_and_slot()	查找对应于给定域、总线和槽位及编号的 pci_dev。
				如果找到设备，其引用计数会递增。
pci_set_power_state()		设置 PCI 电源管理状态（0=D0 ... 3=D3）
pci_find_capability()		在设备的能力列表中查找指定能力。
pci_resource_start()		返回给定 PCI 区域的 bus 起始地址
pci_resource_end()		返回给定 PCI 区域的 bus 结束地址
pci_resource_len()		返回 PCI 区域的字节长度
pci_set_drvdata()		设置 pci_dev 的私有驱动数据指针
pci_get_drvdata()		返回 pci_dev 的私有驱动数据指针
pci_set_mwi()			使能 Memory-Write-Invalidate 事务。
pci_clear_mwi()		禁用 Memory-Write-Invalidate 事务。
=============================	================================================

## 杂项提示

当向用户显示 PCI 设备名称时（例如当驱动想要告诉用户它找到了什么卡），请使用 pci_name(pci_dev)。

始终通过指向 pci_dev 结构的指针来引用 PCI 设备。所有 PCI 层函数都使用此标识，这也是唯一合理的标识方式。除非出于非常特殊的用途，否则不要使用 bus/slot/function 编号——在具有多个主总线的系统上，它们的语义可能相当复杂。

不要试图在你的驱动中开启 Fast Back to Back 写操作。总线上的所有设备都需要具备此能力，因此这应由平台和通用代码处理，而非各个驱动。

## 厂商与设备标识

除非设备/厂商 ID 在多个驱动之间共享，否则不要将新的设备或厂商 ID 添加到 include/linux/pci_ids.h。如果它们有用，你可以在自己的驱动中添加私有定义，或者直接使用普通的十六进制常量。

设备 ID 是任意的十六进制数字（由厂商控制），通常只用在单一位置，即 pci_device_id 表。

请将新的厂商/设备 ID 提交到 https://pci-ids.ucw.cz/。pci.ids 文件在 https://github.com/pciutils/pciids 上有镜像。

## 已废弃的函数

有几个函数在尝试将旧驱动移植到新 PCI 接口时可能会遇到。它们已不再存在于内核中，因为它们与热插拔、PCI 域或合理的加锁不兼容。

=================	===========================================
pci_find_device()	已被 pci_get_device() 取代
pci_find_subsys()	已被 pci_get_subsys() 取代
pci_find_slot()		已被 pci_get_domain_bus_and_slot() 取代
pci_get_slot()		已被 pci_get_domain_bus_and_slot() 取代
=================	===========================================

替代方案是遍历 PCI 设备列表的传统 PCI 设备驱动。这仍然是可能的，但不鼓励这样做。

## MMIO 空间与“写投递”

将驱动从使用 I/O Port 空间转换为使用 MMIO 空间通常需要一些额外的更改。具体而言，需要处理“写投递”（write posting）。许多驱动（例如 tg3、acenic、sym53c8xx_2）已经这样做了。I/O Port 空间保证写事务在 CPU 继续之前到达 PCI 设备。对 MMIO 空间的写操作允许 CPU 在事务到达 PCI 设备之前继续。硬件爱好者称之为“写投递”，因为写完成在事务到达其目的地之前就被“投递”给了 CPU。

因此，对时序敏感的代码应在 CPU 预期等待以进行其他工作的地方添加 readl()。经典的“位翻转”（bit banging）
```

       for (i = 8; --i; val >>= 1) {
               outb(val & 1, ioport_reg);      /* write bit */
               udelay(10);
       }

```
```

       for (i = 8; --i; val >>= 1) {
               writeb(val & 1, mmio_reg);      /* write bit */
               readb(safe_mmio_reg);           /* flush posted write */
               udelay(10);
       }

```
重要的是，“safe_mmio_reg” 不能有任何干扰设备正确运行的副作用。

另一个需要注意的情况是重置 PCI 设备时。使用 PCI 配置空间读取来刷新 writel()。如果预期 PCI 设备不会对 readl() 做出响应，这将优雅地处理所有平台上的 PCI 主设备中止。大多数 x86 平台允许 MMIO 读取主设备中止（即“Soft Fail”）并返回垃圾数据（例如 ~0）。但许多 RISC 平台会崩溃（即“Hard Fail”）。
