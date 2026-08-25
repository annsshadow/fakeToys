
## FWSEC（固件安/ Firmware Security

本文档从概念上简要描FWSEC（Firmware Security，固件安全）镜像及其GPU 启动序列中的作用。因此，这些信息将来可能会发生变化，且仅仅是截至安培（Ampere）GPU 系列时的情况。不过，希望其中描述的概念能帮助理解内核中处理它的相关代码。所有信息均来自公开可用的资料，例如公开的驱动和文档
FWSEC 的作用是提供一个安全启动过程。它运行在“Heavy-secure（高安全）”模式下，并GPU 复位后、将各种 ucode（微码）镜像加载到其GPU 微控制器（如 PMU GSP）之前，执行固件验证
FWSEC 本身是一个存储在 VBIOS ROM ROM FWSEC 分区里的应用程序（详vbios.rst）。它包含不同的命令，FRTS（Firmware Runtime Services，固件运行时服务）和 SB（Secure Booting，复位后安全启动其他微控制器并为它们加载FWSEC 的其ucode）。内核驱动只需要执FRTS，因为安全启动（SB）在驱动加载时已经完成
FRTS 命令划分WPR2 区域（写保护区域），其中包含电源管理所需的数据。一旦设置完成，只有 HS（High Secure，高安全）模式的 ucode 才能访问它（特权级别详见 falcon.rst）
FWSEC 镜像位于 VBIOS ROM 中包含各ucode 镜像（也称为应用程序）的分区中——其中之一便是 FWSEC。关于它如何被提取，请参vbios.rst vbios.rs 源代码
每个 ucode 镜像（包FWSEC 镜像）的 Falcon 数据由头部、数据段（DMEM）和指令代码段（IMEM）组合而成。所有这ucode 镜像都存储在同一ROM 分区中，并通过 PMU 表根据其应用 ID（application ID）来查找要加载的应用程序（详vbios.rs）
对于 nova-core 驱动，FWSEC 包含一个名DMEMMAPPER 的“应用程序接口”（application interface）。该接口除了其他用途外，还用于执行“FWSEC-FRTS”命令。对于安培架构，FWSEC 运行GSP 上的 Heavy-secure 模式并执FRTS
### FWSEC 内存布局

```

   +---------------------------------------------------------------+
   |                         FWSEC ROM image (type 0xE0)           |
   |                                                               |
   |  +---------------------------------+                          |
   |  |     PMU Falcon Ucode Table      |                          |
   |  |     (PmuLookupTable)            |                          |
   |  |  +-------------------------+    |                          |
   |  |  | Table Header            |    |                          |
   |  |  | - version: 0x01         |    |                          |
   |  |  | - header_size: 6        |    |                          |
   |  |  | - entry_size: 6         |    |                          |
   |  |  | - entry_count: N        |    |                          |
   |  |  | - desc_version:3(unused)|    |                          |
   |  |  +-------------------------+    |                          |
   |  |         ...                     |                          |
   |  |  +-------------------------+    |                          |
   |  |  | Entry for FWSEC (0x85)  |    |                          |
   |  |  | (PmuLookupTableEntry)   |    |                          |
   |  |  | - app_id: 0x85 (FWSEC)  |----|----+                     |
   |  |  | - target_id: 0x01 (PMU) |    |    |                     |
   |  |  | - data: offset ---------|----|----|---+ look up FWSEC   |
   |  |  +-------------------------+    |    |   |                 |
   |  +---------------------------------+    |   |                 |
   |                                         |   |                 |
   |                                         |   |                 |
   |  +---------------------------------+    |   |                 |
   |  |     FWSEC Ucode Component       |<---+   |                 |
   |  |     (aka Falcon data)           |        |                 |
   |  |  +-------------------------+    |        |                 |
   |  |  | FalconUCodeDescV3       |<---|--------+                 |
   |  |  | - hdr                   |    |                          |
   |  |  | - stored_size           |    |                          |
   |  |  | - pkc_data_offset       |    |                          |
   |  |  | - interface_offset -----|----|----------------+         |
   |  |  | - imem_phys_base        |    |                |         |
   |  |  | - imem_load_size        |    |                |         |
   |  |  | - imem_virt_base        |    |                |         |
   |  |  | - dmem_phys_base        |    |                |         |
   |  |  | - dmem_load_size        |    |                |         |
   |  |  | - engine_id_mask        |    |                |         |
   |  |  | - ucode_id              |    |                |         |
   |  |  | - signature_count       |    |    look up sig |         |
   |  |  | - signature_versions --------------+          |         |
   |  |  +-------------------------+    |     |          |         |
   |  |         (no gap)                |     |          |         |
   |  |  +-------------------------+    |     |          |         |
   |  |  | Signatures Section      |<---|-----+          |         |
   |  |  | (384 bytes per sig)     |    |                |         |
   |  |  | - RSA-3K Signature 1    |    |                |         |
   |  |  | - RSA-3K Signature 2    |    |                |         |
   |  |  |   ...                   |    |                |         |
   |  |  +-------------------------+    |                |         |
   |  |                                 |                |         |
   |  |  +-------------------------+    |                |         |
   |  |  | IMEM Section (Code)     |    |                |         |
   |  |  |                         |    |                |         |
   |  |  | Contains instruction    |    |                |         |
   |  |  | code etc.               |    |                |         |
   |  |  +-------------------------+    |                |         |
   |  |                                 |                |         |
   |  |  +-------------------------+    |                |         |
   |  |  | DMEM Section (Data)     |    |                |         |
   |  |  |                         |    |                |         |
   |  |  | +---------------------+ |    |                |         |
   |  |  | | Application         | |<---|----------------+         |
   |  |  | | Interface Table     | |    |                          |
   |  |  | | (FalconAppifHdrV1)  | |    |                          |
   |  |  | | Header:             | |    |                          |
   |  |  | | - version: 0x01     | |    |                          |
   |  |  | | - header_size: 4    | |    |                          |
   |  |  | | - entry_size: 8     | |    |                          |
   |  |  | | - entry_count: N    | |    |                          |
   |  |  | |                     | |    |                          |
   |  |  | | Entries:            | |    |                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | | DEVINIT (ID 1)  | | |    |                          |
   |  |  | | | - id: 0x01      | | |    |                          |
   |  |  | | | - dmemOffset X -|-|-|----+                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | +-----------------+ | |    |                          |
   |  |  | | | DMEMMAPPER(ID 4)| | |    |                          |
   |  |  | | | - id: 0x04      | | |    | Used only for DevInit    |
   |  |  | | |  (NVFW_FALCON_  | | |    | application (not FWSEC)  |
   |  |  | | |   APPIF_ID_DMEMMAPPER)   |                          |
   |  |  | | - dmemOffset Y -|-|-|----|-----+                    |
   |  |  | +-----------------+ | |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |                         |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |  | DEVINIT Engine      |<|----+     | Used by FWSEC      |
   |  |  | Interface           | |    |     |         app.       |
   |  |  +---------------------+ |    |     |                    |
   |  |                         |    |     |                    |
   |  |  +---------------------+ |    |     |                    |
   |  |  | DMEM Mapper (ID 4)  |<|----+-----+                    |
   |  |  | (FalconAppifDmemmapperV3)  |                          |
   |  |  | - signature: "DMAP" | |    |                          |
   |  |  | - version: 0x0003   | |    |                          |
   |  |  | - Size: 64 bytes    | |    |                          |
   |  |  | - cmd_in_buffer_off | |----|------------+             |
   |  |  | - cmd_in_buffer_size| |    |            |             |
   |  |  | - cmd_out_buffer_off| |----|------------|-----+       |
   |  |  | - cmd_out_buffer_sz | |    |            |     |       |
   |  |  | - init_cmd          | |    |            |     |       |
   |  |  | - features          | |    |            |     |       |
   |  |  | - cmd_mask0/1       | |    |            |     |       |
   |  |  +---------------------+ |    |            |     |       |
   |  |                         |    |            |     |       |
   |  |  +---------------------+ |    |            |     |       |
   |  |  | Command Input Buffer|<|----|------------+     |       |
   |  |  | - Command data      | |    |                  |       |
   |  |  | - Arguments         | |    |                  |       |
   |  |  +---------------------+ |    |                  |       |
   |  |                         |    |                  |       |
   |  |  +---------------------+ |    |                  |       |
   |  |  | Command Output      |<|----|------------------+       |
   |  |  | Buffer              | |    |                          |
   |  |  | - Results           | |    |                          |
   |  |  | - Status            | |    |                          |
   |  |  +---------------------+ |    |                          |
   |  +-------------------------+    |                          |
   |  +---------------------------------+                          |
   |                                                               |
   +---------------------------------------------------------------+

```
   以上GA-102 安培 GPU 为例，未来的 GPU 可能会有所不同
   FWSEC 镜像还在内存擦除（ECC 初始化）VPR（Video Protected Region，视频保护区）初始化中发挥作用。在 nova-core 驱动加载之前，FWSEC 镜像就已经运行在 GSP 上的 heavy-secure 模式。devinit 序列完成后，它会进行 VRAM 内存擦除（ECC 初始化）。在消费GPU 上，它只擦除部分内存，然后发起“异步擦除”（async scrubbing）。在该异步擦除完成之前，未擦除的 VRAM 不能用于分配（因DRM 内存分配器需要等待该擦除完成）