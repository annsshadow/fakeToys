
## VBIOS

本文档描述了 VBIOS 映像的布局，它是 GPU 的 ROM 中一系列拼接在一起的映像。VBIOS 被镜像到
BAR 0 空间，既由 GPU 上的 Boot ROM 固件（也称为 IFR 或 init-from-rom 固件）读取，以在驱动加载
前引导各种微控制器（PMU、SEC、GSP）完成关键的初始化，也由内核中的 nova-core 驱动读取以引导
GSP。

ROM 中映像的格式遵循 PCI 规范中的"BIOS 规范"部分，并带有 Nvidia 特有的扩展。类型为 FwSec 的
ROM 映像包含 Falcon ucode，也是我们主要寻找的内容。

举例来说，以下是被 nova-core 驱动支持的 Ampere GA102 GPU 的 VBIOS 中可以找到的不同映像类型：

- PciAt 映像（类型 0x00）——这是标准的 PCI BIOS 映像，其名称很可能来自 "IBM PC/AT" 架构。

- EFI 映像（类型 0x03）——这是 EFI BIOS 映像。它包含用于显示 UEFI 图形输出的 UEFI GOP 驱动。

- 第一个 FwSec 映像（类型 0xE0）——第一个 FwSec 映像（Secure Firmware，安全固件）

- 第二个 FwSec 映像（类型 0xE0）——第二个 FwSec 映像（安全固件）包含各种微码（也称为 application），
  它们执行一系列不同的功能。FWSEC ucode 以 heavy-secure 模式运行，通常直接在 GSP 上运行（在未来的
  代次中它也可能运行在不同的指定处理器上，但在 Ampere 上它是 GSP）。该固件随后在 GPU 复位之后、
  驱动加载之前，把其它固件 ucode 加载到 PMU 和 SEC2 微控制器上以进行 gfw 初始化（参见 devinit.rst）。
  DEVINIT ucode 本身是存储在该 ROM 分区中的另一个 ucode。

一旦定位到，Falcon ucode 在其数据内存（DMEM）中具有"应用程序接口"（Application Interfaces）。
对于 FWSEC，我们用于 FWSEC 的应用程序接口是"DMEM mapper"接口，它被配置为运行 "FRTS" 命令。该
命令在 VRAM 中划分出 WPR2（Write-Protected Region，写保护区域）。然后它把称为 'FRTS' 的重要
电源管理数据放入该区域。WPR2 区域只能被 heavy-secure ucode 访问。

   FwSec 为何在 ROM 中有 2 个不同的分区尚不清楚，但它们都是类型 0xE0，并且可以据此识别。在未来的
   代次中这可能会发生变化。

### VBIOS ROM 布局（VBIOS ROM Layout）

```

    +----------------------------------------------------------------------------+
    | VBIOS (Starting at ROM_OFFSET: 0x300000)                                   |
    +----------------------------------------------------------------------------+
    | +-----------------------------------------------+                          |
    | | PciAt Image (Type 0x00)                       |                          |
    | +-----------------------------------------------+                          |
    | | +-------------------+                         |                          |
    | | | ROM Header        |                         |                          |
    | | | (Signature 0xAA55)|                         |                          |
    | | +-------------------+                         |                          |
    | |         | rom header's pci_data_struct_offset |                          |
    | |         | points to the PCIR structure        |                          |
    | |         V                                     |                          |
    | | +-------------------+                         |                          |
    | | | PCIR Structure    |                         |                          |
    | | | (Signature "PCIR")|                         |                          |
    | | | last_image: 0x80  |                         |                          |
    | | | image_len: size   |                         |                          |
    | | | in 512-byte units |                         |                          |
    | | +-------------------+                         |                          |
    | |         |                                     |                          |
    | |         | NPDE immediately follows PCIR       |                          |
    | |         V                                     |                          |
    | | +-------------------+                         |                          |
    | | | NPDE Structure    |                         |                          |
    | | | (Signature "NPDE")|                         |                          |
    | | | last_image: 0x00  |                         |                          |
    | | +-------------------+                         |                          |
    | |                                               |                          |
    | | +-------------------+                         |                          |
    | | | BIT Header        | (Signature scanning     |                          |
    | | | (Signature "BIT") |  provides the location  |                          |
    | | +-------------------+  of the BIT table)      |                          |
    | |         | header is                           |                          |
    | |         | followed by a table of tokens       |                          |
    | |         V one of which is for falcon data.    |                          |
    | | +-------------------+                         |                          |
    | | | BIT Tokens        |                         |                          |
    | | |  ______________   |                         |                          |
    | | | | Falcon Data |   |                         |                          |
    | | | | Token (0x70)|---+------------>------------+--+                       |
    | | | +-------------+   |  falcon_data_ptr()      |  |                       |
    | | +-------------------+                         |  V                       |
    | +-----------------------------------------------+  |                       |
    |              (no gap between images)               |                       |
    | +-----------------------------------------------+  |                       |
    | | EFI Image (Type 0x03)                         |  |                       |
    | +-----------------------------------------------+  |                       |
    | | Contains the UEFI GOP driver (Graphics Output)|  |                       |
    | | +-------------------+                         |  |                       |
    | | | ROM Header        |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | PCIR Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | NPDE Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | Image data        |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | +-----------------------------------------------+  |                       |
    |              (no gap between images)               |                       |
    | +-----------------------------------------------+  |                       |
    | | First FwSec Image (Type 0xE0)                 |  |                       |
    | +-----------------------------------------------+  |                       |
    | | +-------------------+                         |  |                       |
    | | | ROM Header        |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | PCIR Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | NPDE Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | Image data        |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | +-----------------------------------------------+  |                       |
    |              (no gap between images)               |                       |
    | +-----------------------------------------------+  |                       |
    | | Second FwSec Image (Type 0xE0)                |  |                       |
    | +-----------------------------------------------+  |                       |
    | | +-------------------+                         |  |                       |
    | | | ROM Header        |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | PCIR Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | | | NPDE Structure    |                         |  |                       |
    | | +-------------------+                         |  |                       |
    | |                                               |  |                       |
    | | +-------------------+                         |  |                       |
    | | | PMU Lookup Table  | <- falcon_data_offset <----+                       |
    | | | +-------------+   |    pmu_lookup_table     |                          |
    | | | | Entry 0x85  |   |                         |                          |
    | | | | FWSEC_PROD  |   |                         |                          |
    | | | +-------------+   |                         |                          |
    | | +-------------------+                         |                          |
    | |         |                                     |                          |
    | |         | points to                           |                          |
    | |         V                                     |                          |
    | | +-------------------+                         |                          |
    | | | FalconUCodeDescV3 | <- falcon_ucode_offset  |                          |
    | | | (FWSEC Firmware)  |    fwsec_header()       |                          |
    | | +-------------------+                         |                          |
    | |         |   immediately followed  by...       |                          |
    | |         V                                     |                          |
    | | +----------------------------+                |                          |
    | | | Signatures + FWSEC Ucode   |                |                          |
    | | | fwsec_sigs(), fwsec_ucode()|                |                          |
    | | +----------------------------+                |                          |
    | +-----------------------------------------------+                          |
    |                                                                            |
    +----------------------------------------------------------------------------+

```
   该图以 GA-102 Ampere GPU 为例创建，对于未来的或其它 GPU 可能会有所不同。

   关于缩略语的更多解释，请参见 `vbios.rs` 中的详细描述。

### Falcon 数据查找（Falcon data Lookup）

VBIOS 提取代码（vbios.rs）的一个关键部分，是找到 VBIOS 中包含了 PMU 查找表的 Falcon data 的
位置。该查找表用于根据应用程序 ID 查找所需的 Falcon ucode。

PMU 查找表的位置是通过扫描 BIT（`BIOS Information Table`_）token 来找到的，目标是寻找 id 为
`BIT_TOKEN_ID_FALCON_DATA`（0x70）的 token，它表示该表相对于 VBIOS 映像起始位置的偏移量。
遗憾的是，该偏移量没有考虑位于 PciAt 与 FwSec 映像之间的 EFI 映像。`vbios.rs` 代码通过相应的
算术运算对此进行了补偿。
