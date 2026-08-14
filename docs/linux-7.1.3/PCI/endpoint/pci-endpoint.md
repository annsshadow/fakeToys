
:作者: Kishon Vijay Abraham I <kishon@ti.com>

本文档是关于如何使用 PCI Endpoint Framework（PCI 端点框架）的指南，以创建端点控制器
驱动、端点功能驱动，并使用 configfs 接口将功能驱动绑定到控制器驱动。

## Introduction


Linux 有一个全面的 PCI 子系统，用于支持以 Root Complex（根复合体）模式运行的 PCI 控制器。
该子系统能够扫描 PCI 总线、分配内存资源和 IRQ 资源、加载 PCI 驱动（基于厂商 ID、设备
ID），并支持热插拔、电源管理、高级错误报告和虚拟通道等其他服务。

然而，集成在某些 SoC 中的 PCI 控制器 IP 既能在 Root Complex 模式也能在 Endpoint（端点）
模式下运行。PCI Endpoint Framework 将为 Linux 添加端点模式支持。这将有助于在 EP 系统上
运行 Linux，其可用于从测试或验证、协处理器加速器等各种广泛的使用场景。

## PCI Endpoint Core


PCI Endpoint Core（端点核心）层由 3 个部分组成：Endpoint Controller 库、Endpoint
Function 库，以及将端点功能与端点控制器绑定的 configfs 层。

### PCI Endpoint Controller(EPC) Library


EPC 库提供了可在端点模式下运行的控制器使用的 API。它还提供了供功能驱动/库实现特定端点
功能时使用的 API。

#### APIs for the PCI controller Driver


本节列出 PCI Endpoint core 提供给 PCI 控制器驱动使用的 API。

- devm_pci_epc_create()/pci_epc_create()

   PCI 控制器驱动应实现以下 ops：

  - write_header：填充配置空间头部的 ops
  - set_bar：配置 BAR 的 ops
  - clear_bar：复位 BAR 的 ops
  - alloc_addr_space：在 PCI 控制器地址空间中分配的 ops
  - free_addr_space：释放已分配地址空间的 ops
  - raise_irq：触发 legacy、MSI 或 MSI-X 中断的 ops
  - start：启动 PCI 链路的 ops
  - stop：停止 PCI 链路的 ops

   PCI 控制器驱动随后可以通过调用 devm_pci_epc_create()/pci_epc_create() 创建一个新的
   EPC 设备。

- pci_epc_destroy()

   PCI 控制器驱动可以使用 pci_epc_destroy() 销毁由 pci_epc_create() 创建的 EPC 设备。

- pci_epc_linkup()

   为了通知所有功能设备，它们所链接的 EPC 设备已与主机建立链路，PCI 控制器驱动应调用
   pci_epc_linkup()。

- pci_epc_mem_init()

   初始化用于分配 EPC 地址空间的 pci_epc_mem 结构。

- pci_epc_mem_exit()

   清理在 pci_epc_mem_init() 期间分配的 pci_epc_mem 结构。


#### EPC APIs for the PCI Endpoint Function Driver


本节列出 PCI Endpoint core 提供给 PCI 端点功能驱动使用的 API。

- pci_epc_write_header()

   PCI 端点功能驱动应使用 pci_epc_write_header() 将标准配置头部写入端点控制器。

- pci_epc_set_bar()

   PCI 端点功能驱动应使用 pci_epc_set_bar() 配置基地址寄存器（Base Address Register），
   以便主机分配 PCI 地址空间。功能驱动的寄存器空间通常使用此 API 进行配置。

   某些端点控制器也支持在为主机已编程 BAR 基地址后，再次为同一个 BAR 调用
   pci_epc_set_bar()（无需调用 pci_epc_clear_bar()）来更新入站地址转换。端点功能驱动可以
   通过 dynamic_inbound_mapping EPC 特性位检查此能力。

   当 pci_epf_bar.num_submap 非零时，端点功能驱动正在使用 pci_epf_bar.submap 请求 BAR 子
   范围映射。这要求 EPC 通过 subrange_mapping EPC 特性位声明支持。

   当 EPF 驱动想要使用入站子范围映射特性时，它要求 BAR 基地址已由主机在枚举期间编程。因此，
   它需要对同一个 BAR 调用两次 pci_epc_set_bar()（需要 dynamic_inbound_mapping）：第一次将
   num_submap 设为零并配置 BAR 大小，然后在 PCIe 链路建立且主机枚举端点并编程 BAR 基地址后，
   再次将 num_submap 设为非零值。

   注意，在使用入站子范围映射特性时，EPF 驱动不得在两次 pci_epc_set_bar() 调用之间调用
   pci_epc_clear_bar()，因为清除 BAR 可能会清除/禁用端点上的 BAR 寄存器或 BAR 解码，而此时
   主机仍期望已分配的 BAR 地址保持有效。

- pci_epc_clear_bar()

   PCI 端点功能驱动应使用 pci_epc_clear_bar() 复位 BAR。

- pci_epc_raise_irq()

   PCI 端点功能驱动应使用 pci_epc_raise_irq() 触发 Legacy 中断、MSI 或 MSI-X 中断。

- pci_epc_mem_alloc_addr()

   PCI 端点功能驱动应使用 pci_epc_mem_alloc_addr() 从 EPC 地址空间分配内存地址，该地址用于
   访问 RC 的缓冲区。

- pci_epc_mem_free_addr()

   PCI 端点功能驱动应使用 pci_epc_mem_free_addr() 释放使用 pci_epc_mem_alloc_addr() 分配的
   内存空间。

- pci_epc_map_addr()

   PCI 端点功能驱动应使用 pci_epc_map_addr() 将通过 pci_epc_mem_alloc_addr() 获取的本地内存
   CPU 地址映射到 RC 的 PCI 地址。

- pci_epc_unmap_addr()

   PCI 端点功能驱动应使用 pci_epc_unmap_addr() 解除使用 pci_epc_map_addr() 映射到 RC 地址的
   本地内存 CPU 地址的映射。

- pci_epc_mem_map()

   PCI 端点控制器可能对可映射的 RC PCI 地址施加约束。函数 pci_epc_mem_map() 允许端点功能
   驱动在应对此类约束时分配并映射控制器内存。该函数将确定必须通过使用 pci_epc_mem_alloc_addr()
   分配的内存大小，以成功映射一个 RC PCI 地址范围。该函数还将指示实际映射的 PCI 地址范围大小
   （可能小于请求的大小），以及用于访问已映射 RC PCI 地址范围的已分配内存中的偏移量。

- pci_epc_mem_unmap()

   PCI 端点功能驱动可以使用 pci_epc_mem_unmap() 解除并释放使用 pci_epc_mem_map() 分配和映射的
   控制器内存。


#### Other EPC APIs


EPC 库还提供了其他 API。这些用于绑定 EPF 设备与 EPC 设备。pci-ep-cfs.c 可作为使用这些
API 的参考。

- pci_epc_get()

   基于控制器的设备名获取对 PCI 端点控制器的引用。

- pci_epc_put()

   释放使用 pci_epc_get() 获得的对 PCI 端点控制器的引用。

- pci_epc_add_epf()

   向 PCI 端点控制器添加 PCI 端点功能。根据规范，一个 PCIe 设备最多可有 8 个功能。

- pci_epc_remove_epf()

   从 PCI 端点控制器移除 PCI 端点功能。

- pci_epc_start()

   PCI 端点功能驱动在配置完端点功能并希望启动 PCI 链路时，应调用 pci_epc_start()。

- pci_epc_stop()

   PCI 端点功能驱动应调用 pci_epc_stop() 停止 PCI 链路。


### PCI Endpoint Function(EPF) Library


EPF 库提供了供功能驱动和 EPC 库使用的 API，以提供端点模式功能。

#### EPF APIs for the PCI Endpoint Function Driver


本节列出 PCI Endpoint core 提供给 PCI 端点功能驱动使用的 API。

- pci_epf_register_driver()

   PCI Endpoint Function 驱动应实现以下 ops：
  - bind：当 EPC 设备已绑定到 EPF 设备时执行的操作
  - unbind：当 EPC 设备与 EPF 设备之间的绑定丢失时执行的操作
  - add_cfs：可选的 ops，用于创建功能特定的 configfs 属性

   PCI Function 驱动随后可以使用 pci_epf_register_driver() 注册 PCI EPF 驱动。

- pci_epf_unregister_driver()

   PCI Function 驱动可以使用 pci_epf_unregister_driver() 注销 PCI EPF 驱动。

- pci_epf_alloc_space()

   PCI Function 驱动可以使用 pci_epf_alloc_space() 为特定 BAR 分配空间。

- pci_epf_free_space()

   PCI Function 驱动可以通过调用 pci_epf_free_space() 释放已分配的空间（使用
   pci_epf_alloc_space 分配的）。

#### APIs for the PCI Endpoint Controller Library


本节列出 PCI Endpoint core 提供给 PCI 端点控制器库使用的 API。

- pci_epf_linkup()

   当 EPC 设备已与主机建立连接时，PCI 端点控制器库会调用 pci_epf_linkup()。

#### Other EPF APIs


EPF 库还提供了其他 API。这些用于在 EPF 设备绑定到 EPC 设备时通知功能驱动。pci-ep-cfs.c
可作为使用这些 API 的参考。

- pci_epf_create()

   通过传入 PCI EPF 设备的名称创建一个新的 PCI EPF 设备。该名称将用于将 EPF 设备绑定到
   EPF 驱动。

- pci_epf_destroy()

   销毁已创建的 PCI EPF 设备。

- pci_epf_bind()

   当 EPF 设备已绑定到 EPC 设备时，应调用 pci_epf_bind()。

- pci_epf_unbind()

   当 EPC 设备与 EPF 设备之间的绑定丢失时，应调用 pci_epf_unbind()。
