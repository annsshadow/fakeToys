
## PCI Express 端口总线驱动指南 HOWTO


:Author: Tom L Nguyen tom.l.nguyen@intel.com 11/03/2004
:Copyright: |copy| 2004 Intel Corporation

## 关于本指

本指南介PCI Express 端口总线驱动的基础知识，并提供如何使各服务驱动PCI Express 端口总线驱动注册/注销的信息

## 什么是 PCI Express 端口总线驱动


PCI Express 端口是一种逻辑上的 PCI-PCI 桥结构。PCI Express 端口有两种类型：
Root Port（根端口）和 Switch Port（交换端口）。Root Port PCI Express
Root Complex 发起一PCI Express 链路，Switch Port PCI Express 链路
连接到内部逻辑 PCI 总线。Switch Port secondary 总线代表交换机的内部路由
逻辑，称为交换机Upstream Port（上游端口）。交换机Downstream Port
（下游端口）将交换机的内部路由总线桥接到代表来PCI Express 交换机的下游
PCI Express 链路的总线
一PCI Express 端口根据其端口类型，最多可提供四种不同的功能，本文档称
之为服务（services）。PCI Express 端口的服务包括原生热插拔支持（HP）、电管理事件支持（PME）、高级错误上报支持（AER）和虚拟通道支持（VC）。这些服可以由一个复杂的驱动统一处理，也可以分别分布并由相应的服务驱动处理
## 为何使用 PCI Express 端口总线驱动

在现有的 Linux 内核中，Linux 设备驱动模型（Linux Device Driver Model）允一个物理设备仅由单个驱动处理。PCI Express 端口是一个具有多个不同服务的
PCI-PCI 桥设备。为保持简洁的解决方案，每个服务可以有自己的软件服务驱动这种情况下，多个服务驱动会竞争同一PCI-PCI 桥设备。例如，如果 PCI Express
Root Port 的原生热插拔服务驱动先被加载，它就会占用一PCI-PCI Root Port因此内核不会为该 Root Port 加载其他服务驱动。换句话说，使用当前的驱动模型，
不可能让多个服务驱动同时加载并运行在同一PCI-PCI 桥设备上
要使多个服务驱动能够同时运行，就需要一PCI Express 端口总线驱动，它管理
所有已填充PCI Express 端口，并按需将提供的所有服务请求分发给相应的服驱动。使PCI Express 端口总线驱动的一些主要优点如下：

  - 允许PCI-PCI 桥端口设备上同时运行多个服务驱动
  - 允许以独立的分阶段方式实现服务驱动
  - 允许一个服务驱动运行在多个 PCI-PCI 桥端口设备上
  - PCI-PCI 桥端口设备的资源管理并分发给请求的服务驱动
## 配置 PCI Express 端口总线驱动与服务驱动的比较


### PCI Express 端口总线驱动支持编入内核


是否包含 PCI Express 端口总线驱动，取决于内核配置中是否包PCI Express
支持。当内核启用PCI Express 支持时，内核会自动将 PCI Express 端口总线
驱动作为内核驱动包含进来
### 启用服务驱动支持


PCI 设备驱动基于 Linux 设备驱动模型实现。所有服务驱动都PCI 设备驱动如上所述，一旦内核加载了 PCI Express 端口总线驱动，就不可能再加载任何服务
驱动。要符合 PCI Express 端口总线驱动模型，需要对现有服务驱动做一些最小的
改动，且这些改动不会影响现有服务驱动的功能
服务驱动需要使用下面所示的两个 API 将其服务注册PCI Express 端口总线驱动
（参5.2.1 5.2.2 节）。重要的是，服务驱动在调用这API 之前，必须先
初始pcie_port_service_driver 数据结构，该结构位于头文/include/linux/pcieport_if.h 中。若不这样做将导致身份不匹配，使 PCI Express
端口总线驱动无法加载该服务驱动
#### pcie_port_service_register


```

  int pcie_port_service_register(struct pcie_port_service_driver *new)

```
API 取代Linux 驱动模型pci_register_driver API。服务驱动应始终在模初始化（module init）时调用 pcie_port_service_register。注意，服务驱动加载后，
诸如 pci_enable_device(dev) pci_set_master(dev) 之类的调用不再必要，因为
这些调用PCI 端口总线驱动执行
#### pcie_port_service_unregister


```

  void pcie_port_service_unregister(struct pcie_port_service_driver *new)

```
pcie_port_service_unregister 取代 Linux 驱动模型pci_unregister_driver。它
在模块退出时总是由服务驱动调用
#### 示例代码


下面是用于初始化端口服务驱动数据结构的示例服务驱动代码
```

  static struct pcie_port_service_id service_id[] = { {
    .vendor = PCI_ANY_ID,
    .device = PCI_ANY_ID,
    .port_type = PCIE_RC_PORT,
    .service_type = PCIE_PORT_SERVICE_AER,
    }, { /* end: all zeroes */ }
  };

  static struct pcie_port_service_driver root_aerdrv = {
    .name		= (char *)device_name,
    .id_table	= service_id,

    .probe		= aerdrv_load,
    .remove		= aerdrv_unload,

    .suspend	= aerdrv_suspend,
    .resume		= aerdrv_resume,
  };

```

下面是注注销服务驱动的示例代码
```

  static int __init aerdrv_service_init(void)
  {
    int retval = 0;

    retval = pcie_port_service_register(&root_aerdrv);
    if (!retval) {
      /*
      * FIX ME
      */
    }
    return retval;
  }

  static void __exit aerdrv_service_exit(void)
  {
    pcie_port_service_unregister(&root_aerdrv);
  }

  module_init(aerdrv_service_init);
  module_exit(aerdrv_service_exit);

```

## 可能的资源冲

由于允许 PCI-PCI 桥端口设备的所有服务驱动同时运行，下面列出几种可能的资冲突及建议的解决方案
### MSI MSI-X 向量资源


一旦在设备上启用了 MSI MSI-X 中断，设备就会保持该模式，直到再次被禁用由于同一 PCI-PCI 桥端口的服务驱动共享同一个物理设备，如果某个服务驱动启用
或禁MSI/MSI-X 模式，可能会导致不可预期的行为
为避免这种情况，所有服务驱动都不允许在其设备上切换中断模式。PCI Express 端口
总线驱动负责确定中断模式，且这对服务驱动应是透明的。服务驱动只需了解分配struct pcie_device irq 字段的向IRQ，该字段PCI Express 端口总线驱动
探测每个服务驱动时传入。服务驱动应使用 (struct pcie_device*)dev->irq 来调request_irq/free_irq。此外，中断模式存储struct pcie_device interrupt_mode
字段中
### PCI 内存/IO 映射区域


用于 PCI Express 电源管理（PME）、高级错误上报（AER）、热插拔（HP）和虚拟
通道（VC）的服务驱动会访PCI Express 端口上的 PCI 配置空间。在所有情况下所访问的寄存器彼此独立。本文假设所有服务驱动都会表现良好，不会覆盖其他服务
驱动的配置设置
### PCI 閰嶇疆瀵勫瓨鍣。

每个服务驱动都在其自身的能力结构（capability structure）上执行 PCI 配置操作PCI Express 能力结构除外，它被包括服务驱动在内的许多驱动共享。RMW 能力
访问器（pcie_capability_clear_and_set_word()、pcie_capability_set_word() pcie_capability_clear_word()）会保护一组选定PCI Express 能力寄存器：

- Link Control Register
- Root Control Register
- Link Control 2 Register

对这些寄存器的任何更改都应使RMW 访问器进行，以避免因并发更新而产生问题有关受保护寄存器的最新列表，请参pcie_capability_clear_and_set_word()