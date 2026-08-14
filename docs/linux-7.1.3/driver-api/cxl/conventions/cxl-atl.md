
## ACPI PRM CXL Address Translation（ACPI PRM CXL 地址转换）


### Document（文档）


CXL Revision 3.2, Version 1.0

### License（许可证）


SPDX-License Identifier: CC-BY-4.0

### Creator/Contributors（创建者/贡献者）


- Robert Richter, AMD et al.

### Summary of the Change（变更摘要）


CXL 固定内存窗口结构（CFMWS）描述了与一个或多个 CXL 主机桥相关联的零个或多个
主机物理地址（HPA）窗口。CXL 主机桥的每个 HPA 范围由一个 CFMWS 表项表示。一个
HPA 范围可以包含当前分配给 CXL.mem 设备的地址，或者操作系统也可以将一个地址
窗口中的范围分配给某个设备。

主机管理型设备内存（Host-managed Device Memory）是映射到系统一致性地址空间、
且主机可以使用标准回写（write-back）语义访问的设备附加内存。被管理的地址范围
配置在设备的 CXL HDM Decoder 寄存器中。设备中的 HDM Decoder 负责通过剥离特定的
地址位，将 HPA 转换为 DPA。

CXL 设备与 CXL 桥使用相同的 HPA 空间。它在属于同一主机域的所有组件之间是通用
的。在主机与设备之间的 CXL.mem 路径上，地址区域的视图必须保持一致。

这一点在 **CXL 3.2 规范**（表 1-1、3.3.1、8.2.4.20、9.13.1、9.18.1.3）中
有所描述。 [#cxl-spec-3.2]_

取决于平台的互联架构，挂接到主机的组件可能不共享相同的主机物理地址空间。那些
平台需要地址转换，以在主机与所挂接的组件（例如 CXL 设备）之间转换 HPA。转换
机制是主机特定的，且依赖于具体实现。

例如，x86 AMD 平台使用数据 fabric（Data Fabric）来管理对物理内存的访问。设备
拥有自己的内存空间，并可被配置为使用与系统物理地址（SPA）不同的“归一化地址
（Normalized addresses）”。因此就需要地址转换。详情参见
[x86 AMD Address Translation </admin-guide/RAS/address-translation>](x86 AMD Address Translation </admin-guide/RAS/address-translation>)。

那些 AMD 平台在固件中提供 PRM [#prm-spec]_ 处理程序，以执行各种类型的地址转换，
包括针对 CXL 端点。AMD Zen5 系统实现了 ACPI PRM CXL 地址转换固件调用。ACPI PRM
处理程序有一个特定的 GUID，用于唯一标识支持归一化地址的平台。这在 **ACPI v6.5
移植指南**（Address Translation - CXL DPA to System Physical Address）中有记载。
[#amd-ppr-58088]_

在归一化地址模式下，HDM 解码器的地址范围必须以不同的方式配置和处理。端点 HDM
解码器配置中使用的硬件地址不是 SPA，需要从其地址范围转换到 CXL 主机桥的地址
范围。这对于在 CFMWS 中查找端点相关联的 CXL 主机桥与 HPA 窗口尤为重要。此外，
交错（interleave）解码由数据 fabric 完成，端点在将 HPA 转换为 DPA 时并不执行
解码。相反，端点的交错被关闭（1-way）。最后，地址转换在检查端点的硬件地址时
也可能被需要，例如在性能剖析、跟踪或错误处理期间。

```
                          -------------------------------
                          | Root Decoder (CFMWS)        |
                          | SPA Range: 0x850000000      |
                          | Size: 0x8000000000 (512 GB) |
                          | Interleave Ways: 1          |
                          -------------------------------
                                        |
                                        v
                          -------------------------------
                          | Host Bridge Decoder (HDM)   |
                          | SPA Range: 0x850000000      |
                          | Size: 0x8000000000 (512 GB) |
                          | Interleave Ways: 4          |
                          | Targets: endpoint5,8,11,13  |
                          | Granularity: 256            |
                          -------------------------------
                                        |
           -----------------------------+------------------------------
           |                  |                   |                   |
           v                  v                   v                   v
 ------------------- ------------------- ------------------- -------------------
 | endpoint5       | | endpoint8       | | endpoint11      | | endpoint13      |
 | decoder5.0      | | decoder8.0      | | decoder11.0     | | decoder13.0     |
 | PCIe:           | | PCIe:           | | PCIe:           | | PCIe:           |
 |   0000:e2:00.0  | |   0000:e3:00.0  | |   0000:e4:00.0  | |   0000:e1:00.0  |
 | DPA:            | | DPA:            | | DPA:            | | DPA:            |
 |   Start: 0x0    | |   Start: 0x0    | |   Start: 0x0    | |   Start: 0x0    |
 |   Size:         | |   Size:         | |   Size:         | |   Size:         |
 |    0x2000000000 | |    0x2000000000 | |    0x2000000000 | |    0x2000000000 |
 |    (128 GB)     | |    (128 GB)     | |    (128 GB)     | |    (128 GB)     |
 | Interleaving:   | | Interleaving:   | | Interleaving:   | | Interleaving:   |
 |   Ways: 1       | |   Ways: 1       | |   Ways: 1       | |   Ways: 1       |
 |   Gran: 256     | |   Gran: 256     | |   Gran: 256     | |   Gran: 256     |
 ------------------- ------------------- ------------------- -------------------
          |                   |                   |                   |
          v                   v                   v                   v
         DPA                 DPA                 DPA                 DPA

```
这展示了在 sysfs 中的表示：


 /sys/bus/cxl/devices/endpoint5/decoder5.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint5/decoder5.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint5/decoder5.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint5/decoder5.0/start:0x0
 /sys/bus/cxl/devices/endpoint8/decoder8.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint8/decoder8.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint8/decoder8.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint8/decoder8.0/start:0x0
 /sys/bus/cxl/devices/endpoint11/decoder11.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint11/decoder11.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint11/decoder11.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint11/decoder11.0/start:0x0
 /sys/bus/cxl/devices/endpoint13/decoder13.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint13/decoder13.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint13/decoder13.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint13/decoder13.0/start:0x0

注意，端点交错配置使用直接映射（1-way）。

借助 PRM 调用，内核可以确定以下映射：


 cxl decoder5.0: address mapping found for 0000:e2:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder8.0: address mapping found for 0000:e3:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder11.0: address mapping found for 0000:e4:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder13.0: address mapping found for 0000:e1:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256

相应的 CXL 主机桥（HDM）解码器与根解码器（CFMWS）匹配上面所示的计算出的端点
映射：


 /sys/bus/cxl/devices/port1/decoder1.0/interleave_granularity:256
 /sys/bus/cxl/devices/port1/decoder1.0/interleave_ways:4
 /sys/bus/cxl/devices/port1/decoder1.0/size:0x8000000000
 /sys/bus/cxl/devices/port1/decoder1.0/start:0x850000000
 /sys/bus/cxl/devices/port1/decoder1.0/target_list:0,1,2,3
 /sys/bus/cxl/devices/port1/decoder1.0/target_type:expander
 /sys/bus/cxl/devices/root0/decoder0.0/interleave_granularity:256
 /sys/bus/cxl/devices/root0/decoder0.0/interleave_ways:1
 /sys/bus/cxl/devices/root0/decoder0.0/size:0x8000000000
 /sys/bus/cxl/devices/root0/decoder0.0/start:0x850000000
 /sys/bus/cxl/devices/root0/decoder0.0/target_list:7

需要对规范进行以下变更：

- 允许 CXL 设备处于主机地址空间之外的 HPA 空间中。

- 允许平台在主机与设备之间 CXL.mem 路径上跨越内存域时使用特定于实现的地址
  转换。

- 定义一种将设备地址转换为 SPA 的 PRM 处理程序方法。

- 规定平台应向操作系统提供 PRM 处理程序方法，以检测归一化地址，并确定端点
  SPA 范围与交错配置。

- 添加对以下文档的引用：

  | 平台运行时机制规范，版本 1.1 – 2020 年 11 月
  | https://uefi.org/sites/default/files/resources/PRM_Platform_Runtime_Mechanism_1_1_release_candidate.pdf

### Benefits of the Change（变更的好处）


如果不做此变更，操作系统可能无法确定端点的内存区域与根解码器，以及其对应的
HDM 解码器。区域创建会失败。具有不同互联架构的平台将无法建立并使用 CXL。

### References（参考资料）


   https://www.computeexpresslink.org/

   ACPI v6.5 移植指南，出版物编号 # 58088，
   https://www.amd.com/en/search/documentation/hub.html

   https://uefi.org/sites/default/files/resources/PRM_Platform_Runtime_Mechanism_1_1_release_candidate.pdf

### Detailed Description of the Change（变更的详细描述）


以下描述了对 **CXL 3.2 规范** [#cxl-spec-3.2]_ 所需的变更：

向表中添加以下引用：

Table 1-2. Reference Documents（参考文档）

+----------------------------+-------------------+---------------------------+
| Document（文档）           | Chapter Reference | Document No./Location     |
|                            | （章节引用）      | （文档编号/位置）         |
+============================+===================+===========================+
| Platform Runtime Mechanism | Chapter 8, 9      | https://www.uefi.org/acpi |
| Version: 1.1               |                   |                           |
+----------------------------+-------------------+---------------------------+

在章节末尾添加以下段落：

**8.2.4.20 CXL HDM 解码器能力结构**

“一个设备可以使用与其主机域其他组件不通用的 HPA 空间。平台负责在跨越 HPA 空间
时进行地址转换。操作系统必须确定交错配置，并在需要时执行到 HDM 解码器 HPA 范围
的地址转换。转换机制是主机特定的，且依赖于具体实现。

平台通过提供一个平台运行时机制（PRM）处理程序来表明对独立 HPA 空间的支持以及
对地址转换的需要。操作系统应使用该处理程序执行从 DPA 空间到 HPA 空间所需的转换。
该处理程序在 9.18.4 节 *PRM Handler for CXL DPA 到系统物理地址转换* 中定义。”

添加以下章节与小节（含表格）：

**9.18.4 用于 CXL DPA 到系统物理地址转换的 PRM 处理程序**

“一个平台可被配置为使用‘归一化地址’。主机物理地址（HPA）空间是组件特定的，
并且不同于系统物理地址（SPA）。端点拥有自己的物理地址空间。呈现给设备的所有
请求已经使用设备物理地址（DPA）。CXL 端点解码器关闭交错（1-way 交错），并且
设备不执行 HPA 解码来确定 DPA。

平台提供一个用于 CXL DPA 到系统物理地址转换的 PRM 处理程序。该 PRM 处理程序将
指定 CXL 端点的设备物理地址（DPA）转换为系统物理地址（SPA）。在主机的地址空间中，
SPA 与 HPA 是等价的，操作系统应使用该处理程序来确定与设备地址对应的 HPA，例如
在配置了归一化地址的平台上配置 HDM 解码器时。处理程序的 GUID 与参数缓冲区格式
在 9.18.4.1 节中规定。如果操作系统识别出该 PRM 处理程序，则说明平台支持归一化
地址，且操作系统必须在需要时执行 DPA 地址转换。”

**9.18.4.1 PRM 处理程序调用**

“操作系统使用直接调用机制来调用 CXL DPA 到系统物理地址转换的 PRM 处理程序。
调用 PRM 处理程序的细节在平台运行时机制（PRM）规范中描述。

该 PRM 处理程序由以下 GUID 标识：

 EE41B397-25D4-452C-AD54-48C6E3480B94

调用者分配并准备一个参数缓冲区，然后传入 PRM 处理程序 GUID 与指向参数缓冲区的
指针来调用该处理程序。参数缓冲区在表 9-32 中描述。”

**表 9-32. 用于 CXL DPA 到系统物理地址转换的 PRM 参数缓冲区**
（用于 CXL DPA 到系统物理地址转换的 PRM 参数缓冲区）

+-------------+-----------+------------------------------------------------------------------------+
| Byte Offset | Length in | Description                                                            |
| （字节偏移）|   Bytes   | （描述）                                                               |
+=============+===========+========================================================================+
| 00h         | 8         | **CXL Device Physical Address (DPA)**：CXL DPA（例如来自             |
|             |           | CXL Component Event Log）                                              |
+-------------+-----------+------------------------------------------------------------------------+
| 08h         | 4         | **CXL Endpoint SBDF**：                                                 |
|             |           |                                                                        |
|             |           | - Byte 3 - PCIe Segment（PCIe 段）                                      |
|             |           | - Byte 2 - Bus Number（总线号）                                         |
|             |           | - Byte 1:                                                              |
|             |           |          - Device Number Bits[7:3]（设备号位）                          |
|             |           |          - Function Number Bits[2:0]（功能号位）                        |
|             |           | - Byte 0 - RESERVED (MBZ)（保留）                                       |
|             |           |                                                                        |
+-------------+-----------+------------------------------------------------------------------------+
| 0Ch         | 8         | **Output Buffer**：指向缓冲区的虚拟地址指针，                          |
|             |           | 如 Table 9-33 所定义。                                                  |
+-------------+-----------+------------------------------------------------------------------------+

**表 9-33. 用于 CXL DPA 到系统物理地址转换的 PRM 输出缓冲区**
（用于 CXL DPA 到系统物理地址转换的 PRM 输出缓冲区）

+-------------+-----------+------------------------------------------------------------------------+
| Byte Offset | Length in | Description                                                            |
| （字节偏移）|   Bytes   | （描述）                                                               |
+=============+===========+========================================================================+
| 00h         | 8         | **System Physical Address (SPA)**：从 CXL DPA 转换而来的 SPA。        |
|             |           |                                                                        |
+-------------+-----------+------------------------------------------------------------------------+
