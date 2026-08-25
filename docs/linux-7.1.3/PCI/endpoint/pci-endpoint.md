
:作 Kishon Vijay Abraham I <kishon@ti.com>

本文档是关于如何使用 PCI Endpoint Framework（PCI 端点框架）的指南，以创建端点控制
驱动、端点功能驱动，并使configfs 接口将功能驱动绑定到控制器驱动

## Introduction


Linux 有一个全面的 PCI 子系统，用于支持Root Complex（根复合体）模式运行PCI 控制器
该子系统能够扫描 PCI 总线、分配内存资源和 IRQ 资源、加PCI 驱动（基于厂ID、设
ID），并支持热插拔、电源管理、高级错误报告和虚拟通道等其他服务

然而，集成在某SoC 中的 PCI 控制IP 既能Root Complex 模式也能Endpoint（端点）
模式下运行。PCI Endpoint Framework 将为 Linux 添加端点模式支持。这将有助于EP 系统
运行 Linux，其可用于从测试或验证、协处理器加速器等各种广泛的使用场景

## PCI Endpoint Core


PCI Endpoint Core（端点核心）层由 3 个部分组成：Endpoint Controller 库、Endpoint
Function 库，以及将端点功能与端点控制器绑定的 configfs 层

### PCI Endpoint Controller(EPC) Library


EPC 库提供了可在端点模式下运行的控制器使用的 API。它还提供了供功能驱库实现特定端
功能时使用的 API

#### APIs for the PCI controller Driver


本节列出 PCI Endpoint core 提供PCI 控制器驱动使用的 API

- devm_pci_epc_create()/pci_epc_create()

   PCI 控制器驱动应实现以下 ops

  - write_header：填充配置空间头部的 ops
  - set_bar：配BAR ops
  - clear_bar：复BAR ops
  - alloc_addr_space：在 PCI 控制器地址空间中分配的 ops
  - free_addr_space：释放已分配地址空间ops
  - raise_irq：触legacy、MSI MSI-X 中断ops
  - start：启PCI 链路ops
  - stop：停PCI 链路ops

   PCI 控制器驱动随后可以通过调用 devm_pci_epc_create()/pci_epc_create() 创建一个新
   EPC 设备

- pci_epc_destroy()

   PCI 控制器驱动可以使pci_epc_destroy() 销毁由 pci_epc_create() 创建EPC 设备

- pci_epc_linkup()

   为了通知所有功能设备，它们所链接EPC 设备已与主机建立链路，PCI 控制器驱动应调用
   pci_epc_linkup()銆。

- pci_epc_mem_init()

   初始化用于分EPC 地址空间pci_epc_mem 结构

- pci_epc_mem_exit()

   清理pci_epc_mem_init() 期间分配pci_epc_mem 结构


#### EPC APIs for the PCI Endpoint Function Driver


本节列出 PCI Endpoint core 提供PCI 端点功能驱动使用API

- pci_epc_write_header()

   PCI 端点功能驱动应使pci_epc_write_header() 将标准配置头部写入端点控制器

- pci_epc_set_bar()

   PCI 端点功能驱动应使pci_epc_set_bar() 配置基地址寄存器（Base Address Register），
   以便主机分配 PCI 地址空间。功能驱动的寄存器空间通常使用API 进行配置

   某些端点控制器也支持在为主机已编BAR 基地址后，再次为同一BAR 调用
   pci_epc_set_bar()（无需调用 pci_epc_clear_bar()）来更新入站地址转换。端点功能驱动可
   通过 dynamic_inbound_mapping EPC 特性位检查此能力

   pci_epf_bar.num_submap 非零时，端点功能驱动正在使用 pci_epf_bar.submap 请求 BAR 
   范围映射。这要求 EPC 通过 subrange_mapping EPC 特性位声明支持

   EPF 驱动想要使用入站子范围映射特性时，它要求 BAR 基地址已由主机在枚举期间编程。因此，
   它需要对同一BAR 调用两次 pci_epc_set_bar()（需dynamic_inbound_mapping）：第一次将
   num_submap 设为零并配置 BAR 大小，然后在 PCIe 链路建立且主机枚举端点并编程 BAR 基地址后，
   再次num_submap 设为非零值

   注意，在使用入站子范围映射特性时，EPF 驱动不得在两pci_epc_set_bar() 调用之间调用
   pci_epc_clear_bar()，因为清BAR 可能会清禁用端点上的 BAR 寄存器或 BAR 解码，而此
   主机仍期望已分配BAR 地址保持有效

- pci_epc_clear_bar()

   PCI 端点功能驱动应使pci_epc_clear_bar() 复位 BAR

- pci_epc_raise_irq()

   PCI 端点功能驱动应使pci_epc_raise_irq() 触发 Legacy 中断、MSI MSI-X 中断

- pci_epc_mem_alloc_addr()

   PCI 端点功能驱动应使pci_epc_mem_alloc_addr() EPC 地址空间分配内存地址，该地址用于
   访问 RC 的缓冲区

- pci_epc_mem_free_addr()

   PCI 端点功能驱动应使pci_epc_mem_free_addr() 释放使用 pci_epc_mem_alloc_addr() 分配
   内存空间

- pci_epc_map_addr()

   PCI 端点功能驱动应使pci_epc_map_addr() 将通过 pci_epc_mem_alloc_addr() 获取的本地内
   CPU 地址映射RC PCI 地址

- pci_epc_unmap_addr()

   PCI 端点功能驱动应使pci_epc_unmap_addr() 解除使用 pci_epc_map_addr() 映射RC 地址
   本地内存 CPU 地址的映射

- pci_epc_mem_map()

   PCI 端点控制器可能对可映射的 RC PCI 地址施加约束。函pci_epc_mem_map() 允许端点功能
   驱动在应对此类约束时分配并映射控制器内存。该函数将确定必须通过使用 pci_epc_mem_alloc_addr()
   分配的内存大小，以成功映射一RC PCI 地址范围。该函数还将指示实际映射PCI 地址范围大小
   （可能小于请求的大小），以及用于访问已映RC PCI 地址范围的已分配内存中的偏移量

- pci_epc_mem_unmap()

   PCI 端点功能驱动可以使用 pci_epc_mem_unmap() 解除并释放使pci_epc_mem_map() 分配和映射的
   控制器内存


#### Other EPC APIs


EPC 库还提供了其API。这些用于绑EPF 设备EPC 设备。pci-ep-cfs.c 可作为使用这
API 的参考

- pci_epc_get()

   基于控制器的设备名获取对 PCI 端点控制器的引用

- pci_epc_put()

   释放使用 pci_epc_get() 获得的对 PCI 端点控制器的引用

- pci_epc_add_epf()

   PCI 端点控制器添PCI 端点功能。根据规范，一PCIe 设备最多可8 个功能

- pci_epc_remove_epf()

   PCI 端点控制器移PCI 端点功能

- pci_epc_start()

   PCI 端点功能驱动在配置完端点功能并希望启PCI 链路时，应调pci_epc_start()

- pci_epc_stop()

   PCI 端点功能驱动应调pci_epc_stop() 停止 PCI 链路


### PCI Endpoint Function(EPF) Library


EPF 库提供了供功能驱动和 EPC 库使用的 API，以提供端点模式功能

#### EPF APIs for the PCI Endpoint Function Driver


本节列出 PCI Endpoint core 提供PCI 端点功能驱动使用API

- pci_epf_register_driver()

   PCI Endpoint Function 驱动应实现以ops
  - bind：当 EPC 设备已绑定到 EPF 设备时执行的操作
  - unbind：当 EPC 设备EPF 设备之间的绑定丢失时执行的操
  - add_cfs：可选的 ops，用于创建功能特定的 configfs 属

   PCI Function 驱动随后可以使用 pci_epf_register_driver() 注册 PCI EPF 驱动

- pci_epf_unregister_driver()

   PCI Function 驱动可以使用 pci_epf_unregister_driver() 注销 PCI EPF 驱动

- pci_epf_alloc_space()

   PCI Function 驱动可以使用 pci_epf_alloc_space() 为特BAR 分配空间

- pci_epf_free_space()

   PCI Function 驱动可以通过调用 pci_epf_free_space() 释放已分配的空间（使
   pci_epf_alloc_space 分配的）

#### APIs for the PCI Endpoint Controller Library


本节列出 PCI Endpoint core 提供PCI 端点控制器库使用API

- pci_epf_linkup()

   EPC 设备已与主机建立连接时，PCI 端点控制器库会调pci_epf_linkup()

#### Other EPF APIs


EPF 库还提供了其API。这些用于在 EPF 设备绑定EPC 设备时通知功能驱动。pci-ep-cfs.c
可作为使用这API 的参考

- pci_epf_create()

   通过传入 PCI EPF 设备的名称创建一个新PCI EPF 设备。该名称将用于将 EPF 设备绑定
   EPF 驱动

- pci_epf_destroy()

   销毁已创建PCI EPF 设备

- pci_epf_bind()

   EPF 设备已绑定到 EPC 设备时，应调pci_epf_bind()

- pci_epf_unbind()

   EPC 设备EPF 设备之间的绑定丢失时，应调用 pci_epf_unbind()
