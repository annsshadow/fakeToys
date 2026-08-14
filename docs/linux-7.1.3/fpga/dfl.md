## FPGA 设备特性列表（DFL）框架概述


作者（Authors）：

- Enno Luebbers <enno.luebbers@intel.com>
- Xiao Guangrong <guangrong.xiao@linux.intel.com>
- Wu Hao <hao.wu@intel.com>
- Xu Yilun <yilun.xu@intel.com>

DFL（Device Feature List，设备特性列表）FPGA 框架（以及依据此框架实现的驱动）隐藏了底层硬件的细节，并为用户空间提供统一的接口。应用程序可以使用这些接口，在实现了设备内存中 DFL 的平台上配置、枚举、打开并访问 FPGA 加速器。此外，DFL 框架还支持 FPGA 重配置等系统级管理功能。


## 设备特性列表（DFL）概述

设备特性列表（DFL）在设备 MMIO 空间中定义了一串特性头（feature header）的链表，用以提供一种可扩展的特性添加方式。软件可以遍历这些预定义的数据结构来枚举 FPGA 特性：FPGA 接口单元（FIU）、加速功能单元（AFU）以及私有特性（Private Features）。
```

    Header            Header            Header            Header
 +----------+  +-->+----------+  +-->+----------+  +-->+----------+
 |   Type   |  |   |  Type    |  |   |  Type    |  |   |  Type    |
 |   FIU    |  |   | Private  |  |   | Private  |  |   | Private  |
 +----------+  |   | Feature  |  |   | Feature  |  |   | Feature  |
 | Next_DFH |--+   +----------+  |   +----------+  |   +----------+
 +----------+      | Next_DFH |--+   | Next_DFH |--+   | Next_DFH |--> NULL
 |    ID    |      +----------+      +----------+      +----------+
 +----------+      |    ID    |      |    ID    |      |    ID    |
 | Next_AFU |--+   +----------+      +----------+      +----------+
 +----------+  |   | Feature  |      | Feature  |      | Feature  |
 |  Header  |  |   | Register |      | Register |      | Register |
 | Register |  |   |   Set    |      |   Set    |      |   Set    |
 |   Set    |  |   +----------+      +----------+      +----------+
 +----------+  |      Header
               +-->+----------+
                   |   Type   |
                   |   AFU    |
                   +----------+
                   | Next_DFH |--> NULL
                   +----------+
                   |   GUID   |
                   +----------+
                   |  Header  |
                   | Register |
                   |   Set    |
                   +----------+

```
FPGA 接口单元（FIU）表示用于与 FPGA 接口的独立功能单元，例如 FPGA 管理引擎（FME）和端口（Port）（有关 FME 和 Port 的更多描述见后文）。

加速功能单元（AFU）表示一个可编程的 FPGA 区域，并始终作为一个子节点连接到某个 FIU（例如一个 Port），如上图所示。

私有特性（Private Features）表示 FIU 和 AFU 的子特性。它们可以是具有不同 ID 的多种功能块，但所有属于同一 FIU 或 AFU 的私有特性，必须通过下一个设备特性头（Next_DFH）指针链接到同一个链表。

每个 FIU、AFU 和私有特性都可以实现自己的功能寄存器。FIU 和 AFU 的功能寄存器组称为头寄存器组（Header Register Set），例如 FME 头寄存器组（FME Header Register Set）；私有特性的功能寄存器组称为特性寄存器组（Feature Register Set），例如 FME 部分重配置特性寄存器组（FME Partial Reconfiguration Feature Register Set）。

该设备特性列表提供了一种将特性链接在一起的方式，软件可以通过遍历该列表方便地定位每个特性，并且可以在任何 FPGA 设备的寄存器区域中实现。


## 设备特性头 - 版本 0

版本 0（DFHv0）是设备特性头的原始版本。DFHv0 中所有多字节量均为小端序。
```

    +-----------------------------------------------------------------------+
    |63 Type 60|59 DFH VER 52|51 Rsvd 41|40 EOL|39 Next 16|15 REV 12|11 ID 0| 0x00
    +-----------------------------------------------------------------------+
    |63                                 GUID_L                             0| 0x08
    +-----------------------------------------------------------------------+
    |63                                 GUID_H                             0| 0x10
    +-----------------------------------------------------------------------+

```
- 偏移（Offset）0x00

  - Type - DFH 的类型（例如 FME、AFU 或私有特性）。
  - DFH VER - DFH 的版本。
  - Rsvd - 当前未使用。
  - EOL - 若 DFH 是设备特性列表（DFL）的末尾则置位。
  - Next - 从 DFH 起始处算起，DFL 中下一个 DFH 的字节偏移；且 DFH 的起始必须按 8 字节边界对齐。若 EOL 置位，则 Next 为列表中最后一个特性的 MMIO 大小。
  - REV - 与此头关联的特性版本。
  - ID - 若 Type 为私有特性，则为特性 ID。

- 偏移（Offset）0x08

  - GUID_L - 128 位全局唯一标识符（GUID）的最低有效 64 位（仅当 Type 为 FME 或 AFU 时存在）。

- 偏移（Offset）0x10

  - GUID_H - 128 位全局唯一标识符（GUID）的最高有效 64 位（仅当 Type 为 FME 或 AFU 时存在）。


## 设备特性头 - 版本 1

设备特性头的版本 1（DFHv1）增加了以下功能：

- 提供标准化的机制，使特性能够向软件描述参数/能力。
- 标准化所有 DFHv1 类型对 GUID 的使用。
- 将 DFH 的位置与特性自身的寄存器空间解耦。

DFHv1 中所有多字节量均为小端序。
```

    +-----------------------------------------------------------------------+
    |63 Type 60|59 DFH VER 52|51 Rsvd 41|40 EOL|39 Next 16|15 REV 12|11 ID 0| 0x00
    +-----------------------------------------------------------------------+
    |63                                 GUID_L                             0| 0x08
    +-----------------------------------------------------------------------+
    |63                                 GUID_H                             0| 0x10
    +-----------------------------------------------------------------------+
    |63                   Reg Address/Offset                      1|  Rel  0| 0x18
    +-----------------------------------------------------------------------+
    |63        Reg Size       32|Params 31|30 Group    16|15 Instance      0| 0x20
    +-----------------------------------------------------------------------+
    |63 Next    35|34RSV33|EOP32|31 Param Version 16|15 Param ID           0| 0x28
    +-----------------------------------------------------------------------+
    |63                 Parameter Data                                     0| 0x30
    +-----------------------------------------------------------------------+

                                  ...

    +-----------------------------------------------------------------------+
    |63 Next    35|34RSV33|EOP32|31 Param Version 16|15 Param ID           0|
    +-----------------------------------------------------------------------+
    |63                 Parameter Data                                     0|
    +-----------------------------------------------------------------------+

```
- 偏移（Offset）0x00

  - Type - DFH 的类型（例如 FME、AFU 或私有特性）。
  - DFH VER - DFH 的版本。
  - Rsvd - 当前未使用。
  - EOL - 若 DFH 是设备特性列表（DFL）的末尾则置位。
  - Next - 从 DFH 起始处算起，DFL 中下一个 DFH 的字节偏移；且 DFH 的起始必须按 8 字节边界对齐。若 EOL 置位，则 Next 为列表中最后一个特性的 MMIO 大小。
  - REV - 与此头关联的特性版本。
  - ID - 若 Type 为私有特性，则为特性 ID。

- 偏移（Offset）0x08

  - GUID_L - 128 位全局唯一标识符（GUID）的最低有效 64 位。

- 偏移（Offset）0x10

  - GUID_H - 128 位全局唯一标识符（GUID）的最高有效 64 位。

- 偏移（Offset）0x18

  - Reg Address/Offset - 若 Rel 位被置位，则该值为特性寄存器 16 字节对齐的绝对地址的高 63 位；否则该值为特性寄存器相对于 DFH 起始的偏移。

- 偏移（Offset）0x20

  - Reg Size - 特性寄存器组的大小（字节）。
  - Params - 若 DFH 具有参数块列表则置位。
  - Group - 若特性属于某个组，则为组 ID。
  - Instance - 组内特性实例的 ID。

- 偏移（Offset）0x28（若特性具有参数）

  - Next - 以 8 字节字为单位的、到下一个参数块的偏移。若 EOP 置位，则为最后一个参数的大小（以 8 字节字为单位）。
  - Param Version - Param ID 的版本。
  - Param ID - 参数 ID。

- 偏移（Offset）0x30

  - Parameter Data - 参数数据，其大小和格式由参数的版本和 ID 定义。


## FIU - FME（FPGA 管理引擎）

FPGA 管理引擎执行重配置及其他基础设施功能。每个 FPGA 设备只有一个 FME。

用户空间应用程序可以使用 open() 获取对 FME 的独占访问，并使用 close() 释放。

以下功能通过 ioctl 暴露：

- 获取驱动 API 版本（DFL_FPGA_GET_API_VERSION）
- 检查扩展（DFL_FPGA_CHECK_EXTENSION）
- 烧写比特流（DFL_FPGA_FME_PORT_PR）
- 将端口分配给 PF（DFL_FPGA_FME_PORT_ASSIGN）
- 从 PF 释放端口（DFL_FPGA_FME_PORT_RELEASE）
- 获取 FME 全局错误的 irq 数量（DFL_FPGA_FME_ERR_GET_IRQ_NUM）
- 设置 FME 错误的中断触发（DFL_FPGA_FME_ERR_SET_IRQ）

更多功能通过 sysfs 暴露
（/sys/class/fpga_region/regionX/dfl-fme.n/）：

 读取比特流 ID（bitstream_id）
     bitstream_id 表示静态 FPGA 区域的版本。

 读取比特流元数据（bitstream_metadata）
     bitstream_metadata 包含静态 FPGA 区域的详细信息，例如综合日期和种子。

 读取端口数量（ports_num）
     一个 FPGA 设备可能拥有多个端口，此 sysfs 接口表示 FPGA 设备有多少个端口。

 全局错误上报管理（errors/）
     错误上报 sysfs 接口允许用户在硬件检测到错误时读取错误，并清除已记录的错误。

 电源管理（dfl_fme_power hwmon）
     电源管理 hwmon sysfs 接口允许用户读取电源管理信息（功耗、阈值、阈值状态、限制等），并为不同的节流级别配置电源阈值。

 热管理（dfl_fme_thermal hwmon）
     热管理 hwmon sysfs 接口允许用户读取热管理信息（当前温度、阈值、阈值状态等）。

 性能上报
     性能计数器通过 perf PMU API 暴露。可以使用标准 perf 工具监控所有可用的 perf 事件。更详细信息请参阅下文的性能计数器小节。


## FIU - PORT（端口）

端口表示静态 FPGA 结构与包含 AFU 的部分可重配置区域之间的接口。它控制从软件到加速器的通信，并暴露复位和调试等特性。每个 FPGA 设备可能拥有多个端口，但每个端口始终只对应一个 AFU。


## AFU

AFU 连接到一个端口 FIU，并暴露一个固定长度的 MMIO 区域，用于加速器专用的控制寄存器。

用户空间应用程序可以通过对端口设备节点使用 open() 获取对连接到端口的 AFU 的独占访问，并使用 close() 释放。

以下功能通过 ioctl 暴露：

- 获取驱动 API 版本（DFL_FPGA_GET_API_VERSION）
- 检查扩展（DFL_FPGA_CHECK_EXTENSION）
- 获取端口信息（DFL_FPGA_PORT_GET_INFO）
- 获取 MMIO 区域信息（DFL_FPGA_PORT_GET_REGION_INFO）
- 映射 DMA 缓冲（DFL_FPGA_PORT_DMA_MAP）
- 解除映射 DMA 缓冲（DFL_FPGA_PORT_DMA_UNMAP）
- 复位 AFU（DFL_FPGA_PORT_RESET）
- 获取端口错误的 irq 数量（DFL_FPGA_PORT_ERR_GET_IRQ_NUM）
- 设置端口错误的中断触发（DFL_FPGA_PORT_ERR_SET_IRQ）
- 获取 UINT 的 irq 数量（DFL_FPGA_PORT_UINT_GET_IRQ_NUM）
- 设置 UINT 的中断触发（DFL_FPGA_PORT_UINT_SET_IRQ）

DFL_FPGA_PORT_RESET:
  复位 FPGA 端口及其 AFU。用户空间可以随时进行端口复位，例如在 DMA 或部分重配置期间。但它绝不应导致任何系统级问题，只会造成功能失败（例如 DMA 或 PR 操作失败），并且可以从失败中恢复。

用户空间应用程序也可以 mmap() 加速器的 MMIO 区域。

更多功能通过 sysfs 暴露：
（/sys/class/fpga_region/<regionX>/<dfl-port.m>/）：

 读取加速器 GUID（afu_id）
     afu_id 表示已编程到此 AFU 的 PR 比特流。

 错误上报（errors/）
     错误上报 sysfs 接口允许用户在硬件检测到端口/AFU 错误时读取错误，并清除已记录的错误。


## DFL 框架概述


```

         +----------+    +--------+ +--------+ +--------+
         |   FME    |    |  AFU   | |  AFU   | |  AFU   |
         |  Module  |    | Module | | Module | | Module |
         +----------+    +--------+ +--------+ +--------+
                 +-----------------------+
                 | FPGA Container Device |    Device Feature List
                 |  (FPGA Base Region)   |         Framework
                 +-----------------------+
  ------------------------------------------------------------------
               +----------------------------+
               |   FPGA DFL Device Module   |
               | (e.g. PCIE/Platform Device)|
               +----------------------------+
                 +------------------------+
                 |  FPGA Hardware Device  |
                 +------------------------+

```
内核中的 DFL 框架提供通用接口，用于创建容器设备（FPGA 基础区域）、从给定的设备特性列表中发现特性设备及其私有特性，并在容器设备下为特性设备（例如 FME、Port 和 AFU）创建带有相关资源的平台设备。它还抽象了私有特性的操作，并向特性设备驱动暴露通用操作。

FPGA DFL 设备可能是不同的硬件，例如 PCIe 设备、平台设备等。一旦系统创建了该设备，其驱动模块总是最先被加载。该驱动在驱动架构中扮演基础设施角色。它在设备内存中定位 DFL，并对其进行处理，将相关资源交给 DFL 框架的通用接口以进行枚举。（详细的枚举 API 请参阅 drivers/fpga/dfl.c。）

FPGA 管理引擎（FME）驱动是一个平台驱动，在 FME 平台设备由 DFL 设备模块创建后自动加载。它提供 FPGA 管理的关键特性，包括：

	a) 暴露静态 FPGA 区域信息，例如版本和元数据。用户可以通过 FME 驱动暴露的 sysfs 接口读取相关信息。

	b) 部分重配置。FME 驱动在 PR 子特性初始化期间创建 FPGA manager、FPGA bridge 和 FPGA region。一旦收到来自用户的 DFL_FPGA_FME_PORT_PR ioctl，它便调用 FPGA Region 的通用接口函数，完成将 PR 比特流部分重配置到指定端口。

与 FME 驱动类似，FPGA 加速功能单元（AFU）驱动在 AFU 平台设备创建后被探测加载。该模块的主要功能是提供一个接口，供用户空间应用程序访问各个加速器，包括端口上的基本复位控制、AFU MMIO 区域导出、DMA 缓冲映射服务函数。

在特性平台设备创建后，匹配的平台驱动会自动加载以处理不同功能。有关在此 DFL 框架下已实现的功能单元的详细信息，请参阅后续小节。


## 部分重配置

如上所述，加速器可以通过 PR 比特流文件的部分重配置进行重配置。PR 比特流文件必须针对 FPGA 的确切静态区域和目标可重配置区域（端口）生成，否则重配置操作将失败，并可能导致系统不稳定。可以通过将 PR 比特流文件头中标注的兼容性 ID 与目标 FPGA 区域暴露的 compat_id 进行比较来检查这种兼容性。该检查通常由用户空间在调用重配置 IOCTL 之前完成。


## FPGA 虚拟化 - PCIe SRIOV

本节描述基于 DFL 的 FPGA 设备上的虚拟化支持，以使运行在虚拟机（VM）中的应用程序能够访问加速器。本节仅描述带有 SRIOV 支持的基于 PCIe 的 FPGA 设备。

特定 FPGA 设备支持的特性通过设备特性列表暴露，如下所示：

```

    +-------------------------------+  +-------------+
    |              PF               |  |     VF      |
    +-------------------------------+  +-------------+
        ^            ^         ^              ^
        |            |         |              |
  +-----|------------|---------|--------------|-------+
  |     |            |         |              |       |
  |  +-----+     +-------+ +-------+      +-------+   |
  |  | FME |     | Port0 | | Port1 |      | Port2 |   |
  |  +-----+     +-------+ +-------+      +-------+   |
  |                  ^         ^              ^       |
  |                  |         |              |       |
  |              +-------+ +------+       +-------+   |
  |              |  AFU  | |  AFU |       |  AFU  |   |
  |              +-------+ +------+       +-------+   |
  |                                                   |
  |            DFL based FPGA PCIe Device             |
  +---------------------------------------------------+

```
FME 总是通过物理功能（PF）访问。

端口（及相关的 AFU）默认通过 PF 访问，但也可以通过 PCIe SRIOV 经虚拟功能（VF）设备暴露。每个 VF 仅包含 1 个端口和 1 个 AFU 以实现隔离。用户可以将通过 PCIe SRIOV 接口创建的各个 VF（加速器）分配给虚拟机。

虚拟化场景下的驱动组织结构如下图所示：
```

    +-------++------++------+             |
    | FME   || FME  || FME  |             |
    | FPGA  || FPGA || FPGA |             |
    |Manager||Bridge||Region|             |
    +-------++------++------+             |
    +-----------------------+  +--------+ |             +--------+
    |          FME          |  |  AFU   | |             |  AFU   |
    |         Module        |  | Module | |             | Module |
    +-----------------------+  +--------+ |             +--------+
          +-----------------------+       |       +-----------------------+
          | FPGA Container Device |       |       | FPGA Container Device |
          |  (FPGA Base Region)   |       |       |  (FPGA Base Region)   |
          +-----------------------+       |       +-----------------------+
            +------------------+          |         +------------------+
            | FPGA PCIE Module |          | Virtual | FPGA PCIE Module |
            +------------------+   Host   | Machine +------------------+
   -------------------------------------- | ------------------------------
             +---------------+            |          +---------------+
             | PCI PF Device |            |          | PCI VF Device |
             +---------------+            |          +---------------+

```
FPGA PCIe 设备驱动总是最先被加载，一旦检测到 FPGA PCIe PF 或 VF 设备即如此。它：

- 使用 DFL 框架的通用接口完成 FPGA PCIe PF 和 VF 设备的枚举。
- 支持 SRIOV。

FME 设备驱动在此驱动架构中扮演管理角色，它提供 ioctl 用于从 PF 释放端口以及将端口分配给 PF。从 PF 释放一个端口后，就可以通过 PCIe SRIOV 的 sysfs 接口将该端口安全地通过 VF 暴露。

为使运行在 VM 中的应用程序能够访问加速器，相应 AFU 的端口需要按以下步骤分配给 VF：

#. PF 默认拥有所有 AFU 端口。任何需要重新分配给 VF 的端口必须首先通过对 FME 设备的 DFL_FPGA_FME_PORT_RELEASE ioctl 释放。

#. 一旦从 PF 释放了 N 个端口，用户就可以使用以下命令启用 SRIOV 和 VF。每个 VF 仅拥有一个带有 AFU 的端口。

```

      echo N > $PCI_DEVICE_PATH/sriov_numvfs

```
#. 将 VF 直通给虚拟机。

#. VF 下的 AFU 可从 VM 中的应用程序访问（使用 VF 内部的同一驱动）。

注意 FME 不能被分配给 VF，因此 PR 及其他管理功能仅可通过 PF 使用。

## 设备枚举

本节介绍应用程序如何从 /sys/class/fpga_region 下的 sysfs 层级枚举 fpga 设备。

在下面的示例中，主机上安装了两个基于 DFL 的 FPGA 设备。每个 fpga 设备有一个 FME 和两个端口（AFU）。

```

	/sys/class/fpga_region/region0
	/sys/class/fpga_region/region1
	/sys/class/fpga_region/region2
	...

```
应用程序需要搜索每个 regionX 文件夹，若找到特性设备（例如找到 "dfl-port.n" 或 "dfl-fme.m"），则它就是代表该 FPGA 设备的基础 fpga 区域。

```

	/sys/class/fpga_region/region0/dfl-fme.0
	/sys/class/fpga_region/region0/dfl-port.0
	/sys/class/fpga_region/region0/dfl-port.1
	...

	/sys/class/fpga_region/region3/dfl-fme.1
	/sys/class/fpga_region/region3/dfl-port.2
	/sys/class/fpga_region/region3/dfl-port.3
	...

```
```

	/sys/class/fpga_region/<regionX>/<dfl-fme.n>/
	/sys/class/fpga_region/<regionX>/<dfl-port.m>/

```
其中 'n' 对所有 FME 连续编号，'m' 对所有端口连续编号。

```

	/sys/class/fpga_region/<regionX>/<dfl-fme.n>/dev
	/sys/class/fpga_region/<regionX>/<dfl-port.n>/dev


```
## 性能计数器

性能上报是 FME 中实现的一个私有特性。它支持在硬件中提供数个独立的、系统级的设备计数器组，用于监控和统计性能事件，包括 "basic"、"cache"、"fabric"、"vtd" 和 "vtd_sip" 计数器。用户可以使用标准 perf 工具监控 FPGA 缓存命中/未命中率、事务数量、AFU 接口时钟计数以及其他 FPGA 性能事件。

不同的 FPGA 设备可能具有不同的计数器组，取决于硬件实现。例如，某些独立 FPGA 卡没有任何缓存。用户可以使用 "perf list" 检查目标硬件支持哪些 perf 事件。

为了让用户能够使用标准 perf API 访问这些性能计数器，驱动创建一个 perf PMU，并在 /sys/bus/event_source/devices/dfl_fme* 下创建相关的 sysfs 接口，以描述可用的 perf 事件和配置选项。

"format" 目录描述 perf_event_attr 结构体 config 字段的格式。config 有 3 个位域："evtype" 定义 perf 事件所属的类型；"event" 是该事件在其类别中的标识；"portid" 用于决定计数器组是监控 FPGA 整体数据还是某个特定端口。

"events" 目录描述所有可用事件的配置模板，可直接配合 perf 工具使用。例如 fab_mmio_read 的配置为 "event=0x06,evtype=0x02,portid=0xff"，表明该事件属于 fabric 类型（0x02），其本地事件 ID 为 0x06，并且用于整体监控（portid=0xff）。

```

  $# perf list |grep dfl_fme

  dfl_fme0/fab_mmio_read/                              [Kernel PMU event]
  <...>
  dfl_fme0/fab_port_mmio_read,portid=?/                [Kernel PMU event]
  <...>

  $# perf stat -a -e dfl_fme0/fab_mmio_read/ <command>
  or
  $# perf stat -a -e dfl_fme0/event=0x06,evtype=0x02,portid=0xff/ <command>
  or
  $# perf stat -a -e dfl_fme0/config=0xff2006/ <command>

```
另一个例子，fab_port_mmio_read 监控特定端口的 mmio 读。因此其配置模板为 "event=0x06,evtype=0x01,portid=?"。portid 应当显式设置。

```

  $# perf stat -a -e dfl_fme0/fab_port_mmio_read,portid=0x0/ <command>
  or
  $# perf stat -a -e dfl_fme0/event=0x06,evtype=0x02,portid=0x0/ <command>
  or
  $# perf stat -a -e dfl_fme0/config=0x2006/ <command>

```
请注意，对于 fabric 计数器，整体 perf 事件（fab_*）和端口 perf 事件（fab_port_*）实际上在硬件中共享同一组计数器，因此无法同时监控两者。如果这组计数器被配置为监控
```

  $# perf stat -e dfl_fme0/fab_mmio_read/,dfl_fme0/fab_port_mmio_write,\
                                                    portid=0/ sleep 1

  Performance counter stats for 'system wide':

                 3      dfl_fme0/fab_mmio_read/
   <not supported>      dfl_fme0/fab_port_mmio_write,portid=0x0/

       1.001750904 seconds time elapsed

```
驱动还提供一个 "cpumask" sysfs 属性，其中仅包含一个用于访问这些 perf 事件的 CPU ID。不允许在多个 CPU 上计数，因为它们是 FPGA 设备上的系统级计数器。

当前驱动不支持采样，因此 "perf record" 不受支持。


## 中断支持

某些 FME 和 AFU 私有特性能够产生中断。如上所述，用户可以调用 ioctl（DFL_FPGA_*_GET_IRQ_NUM）来了解此私有特性是否支持中断或支持多少中断。驱动还实现了一种基于 eventfd 的中断处理机制，以便在中断发生时通知用户。用户可以通过 ioctl（DFL_FPGA_*_SET_IRQ）将 eventfd 设置给驱动，然后对这些 eventfd 进行 poll/select 以等待通知。在当前的 DFL 中，3 个子特性（Port 错误、FME 全局错误和 AFU 中断）支持中断。


## 添加新的 FIU 支持

开发者有可能在此 DFL 框架下制作一些新的功能块（FIU），此时需要为新特性设备（FIU）开发新的平台设备驱动，方法与该框架下已有的特性设备驱动（例如 FME 和 Port/AFU 平台设备驱动）相同。此外，还需要修改 DFL 框架的枚举代码，以检测新的 FIU 类型并创建相关的平台设备。


## 添加新的私有特性支持

在某些情况下，我们可能需要向已有的 FIU（例如 FME 或 Port）添加一些新的私有特性。开发者无需改动 DFL 框架中的枚举代码，因为每个私有特性会被自动解析，并且相关的 MMIO 资源可以在 DFL 框架创建的 FIU 平台设备下找到。开发者只需要提供一个具有匹配特性 ID 的子特性驱动。FME 部分重配置子特性驱动（参见 drivers/fpga/dfl-fme-pr.c）可作为参考。

请参阅以下链接获取已有特性 ID 表以及新特性 ID 申请指南。
https://github.com/OPAE/dfl-feature-id


## DFL 在 PCI 设备上的位置

在 PCI 设备上查找 DFL 的原始方法假定第一个 DFL 的起始位于 bar 0 的偏移 0 处。如果 DFL 的第一个节点是 FME，则端口中更靠后的 DFL 在 FME 头寄存器中指定。另外，也可以使用 PCIe 厂商特定能力（vendor specific capability）结构来指定设备上所有 DFL 的位置，从而灵活选择 DFL 的起始节点类型。Intel 为此保留了 VSEC ID 0x43。厂商特定数据以一个 4 字节的厂商特定寄存器（表示 DFL 数量）开始，随后是每个 DFL 的 4 字节 Offset/BIR 厂商特定寄存器。Offset/BIR 寄存器的位 2:0 表示 BAR，位 31:3 组成按 8 字节对齐的偏移，其中位 2:0 为零。
```

        +----------------------------+
        |31     Number of DFLS      0|
        +----------------------------+
        |31     Offset     3|2 BIR  0|
        +----------------------------+
                      . . .
        +----------------------------+
        |31     Offset     3|2 BIR  0|
        +----------------------------+

```
曾考虑过能够在每个 BAR 上指定多个 DFL，但认定该用例没有价值。每个 BAR 指定单个 DFL 简化了实现，并允许额外的错误检查。


## DFL 设备的用户空间驱动支持

FPGA 的目的是用新开发的硬件组件重新编程。新硬件可以在 DFL 中实例化一个新的私有特性，然后在系统中呈现一个 DFL 设备。在某些情况下，用户可能需要一个用于 DFL 设备的用户空间驱动：

- 用户可能需要对其硬件运行一些诊断测试。
- 用户可能在用户空间中原型化内核驱动。
- 某些硬件为特定用途设计，不适合归入某个标准内核子系统。

这需要从用户空间直接访问 MMIO 空间和中断处理。uio_dfl 模块为此暴露 UIO 设备接口。

当前 uio_dfl 驱动仅支持 Ether Group 子特性，该特性在硬件中没有 irq，因此该驱动未加入中断处理。

应选择 UIO_DFL 以启用 uio_dfl 模块驱动。要经由 UIO 直接访问支持一个新的 DFL 特性，应将其特性 ID 添加到驱动的 id_table 中。


## 开放讨论

FME 驱动目前向用户导出一个用于部分重配置的 ioctl（DFL_FPGA_FME_PORT_PR）。将来如果添加了统一的重配置用户接口，FME 驱动应当从 ioctl 接口切换到这些接口。
