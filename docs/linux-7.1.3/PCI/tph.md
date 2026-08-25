
## TPH 支持


:Copyright: 2024 Advanced Micro Devices, Inc.
:Authors: - Eric van Tassell <eric.vantassell@amd.com>
          - Wei Huang <wei.huang2@amd.com>


## 概述


TPH（TLP Processing Hints，TLP 处理提示）是一PCIe 特性，它允许端点设备为指向内存空间请求提供优化提示。这些提示以一种称为转向标签（Steering Tags，STs）的格式嵌入到请求方TLP
头中，使系统硬件（如根复合体 Root Complex）能够更好地为这些请求管理平台资源
例如，在支持基于 TPH 的直接数据缓存注入的平台上，端点设备可以在其 DMA 流量中包含适当ST以指定数据应被写入哪个缓存。这使得 CPU 核心有更高的概率从缓存中获取数据，从而可能提升性能降低数据处理中的延迟

## 如何使用 TPH


TPH PCIe 中表现为一个可选的扩展能力。Linux 内核在启动时处理 TPH 的发现，但设备驱动若使用 TPH，则需自行请求启用 TPH。一旦启用，驱动使用提供API 获取目标内存的转向标签（Steering
Tag），并将ST 编程到设备的 ST 表中
### Linux 中启TPH 支持


要支TPH，内核必须启CONFIG_PCIE_TPH 选项来构建
### 管理 TPH


```

  int pcie_enable_tph(struct pci_dev *pdev, int mode);

```
此函数为设备启用具有特定 ST 模式TPH 支持。当前支持的模式包括
  - PCI_TPH_ST_NS_MODE - 鏃?ST 妯″紡
  - PCI_TPH_ST_IV_MODE - 中断向量模式
  - PCI_TPH_ST_DS_MODE - 设备特定模式

`pcie_enable_tph()` 在启用前会检查设备是否实际支持所请求的模式。设备驱动可以根`pcie_enable_tph()` 的返回值判断支持哪TPH 模式，并据此正确地启用
```

  void pcie_disable_tph(struct pci_dev *pdev);

```
### 管理 ST


转向标签（Steering Tags）是平台特定的。PCIe 规范并未规定 ST 来自何处。相反，PCI 固件规范
定义了一ACPI _DSM 方法（参`Revised _DSM for Cache Locality TPH Features ECN
<https://members.pcisig.com/wg/PCI-SIG/document/15470>`_），用于检索具有各种属性的目标内存ST。本实现支持的就是此方法
要检索与特定 CPU 关联的目标内存的转向标签，使```

  int pcie_tph_get_cpu_st(struct pci_dev *pdev, enum tph_mem_type type,
                          unsigned int cpu, u16 *tag);

```
`type` 参数用于指定目标内存的类型，可以是易失性（volatile）或持久性（persistent）`cpu` 参数指定内存所关联CPU
检索到 ST 值后，设备驱动可以使用以下函```

  int pcie_tph_set_st_entry(struct pci_dev *pdev, unsigned int index,
                            u16 tag);

```
`index` 参数ST 标签将被写入ST 表条目索引。`pcie_tph_set_st_entry()` 会确ST 表的
正确位置（无论是MSI-X 表中还是TPH 扩展能力空间中），并将转向标签写入由 `index` 参数
指向ST 条目
如何使用这些 TPH 函数完全由驱动决定。例如，网络设备的驱动可以在 RX/TX 队列的中断亲和发生改变时，使用上述 TPH API 来更新转向标签。下面是一个中断亲和性通知器的示例代码

    static void irq_affinity_notified(struct irq_affinity_notify *notify,
                                      const cpumask_t *mask)
    {
         struct drv_irq *irq;
         unsigned int cpu_id;
         u16 tag;

         irq = container_of(notify, struct drv_irq, affinity_notify);
         cpumask_copy(irq->cpu_mask, mask);

         /** 选择一个合适的 CPU 作为目标 - 这里仅作示例 **/
         cpu_id = cpumask_first(irq->cpu_mask);

         if (pcie_tph_get_cpu_st(irq->pdev, TPH_MEM_TYPE_VM, cpu_id,
                                 &tag))
             return;

         if (pcie_tph_set_st_entry(irq->pdev, irq->msix_nr, tag))
             return;
    }

### 系统范围内禁TPH


有一个可用的内核命令行选项来控TPH 特性：
    - "notph"：TPH 将对所有端点设备禁用