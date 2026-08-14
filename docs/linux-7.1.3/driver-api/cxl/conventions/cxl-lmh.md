
## Resolve conflict between CFMWS, Platform Memory Holes, and Endpoint Decoders


### Document


CXL Revision 3.2, Version 1.0

### License


SPDX-License Identifier: CC-BY-4.0

### Creator/Contributors


- Fabio M. De Francesco, Intel
- Dan J. Williams, Intel
- Mahesh Natu, Intel

### Summary of the Change


根据当前的 Compute Express Link（CXL）规范（Revision 3.2, Version 1.0），CXL 固定内存窗口结构（CFMWS）描述了与每个 CXL 主机桥相关联的零个或多个主机物理地址（HPA）窗口。每个窗口代表一个可能跨一个或多个目标（包括 CXL 主机桥）交织的连续 HPA 范围。每个窗口都有一组约束其使用的限制。由操作系统主导的配置与电源管理（OSPM）负责将每个窗口用于指定的用途。

当前 CXL 规范的表 9-22 指出，Window Size 字段包含该窗口描述的 HPA 连续字节总数。该值必须是交织路数（NIW）* 256 MB 的整数倍。

平台固件（BIOS）可能在 4 GB 以下保留物理地址，那里可能存在内存空洞（例如用于 PCIe MMIO 的低内存空洞）。在这种情况下，CFMWS 范围大小可能不遵循 NIW * 256 MB 规则。

HPA 代表 CXL 设备能够解码并响应的实际物理内存地址空间，而系统物理地址（SPA）是一个相关但不同的概念，它代表用户可以直接发起事务寻址的系统可见地址空间，因此排除了保留区域。

BIOS 发布 CFMWS 来传达活跃的 SPA 范围，在有 LMH 的平台上，这些范围映射到 HPA 的一个严格子集。SPA 范围裁掉了空洞，导致 Endpoints 中有一部分 HPA 范围与空洞相交却无对应 SPA 可映射，从而丢失容量。

例如，一个带两个 CFMWS 且 LMH 从 2 GB 开始的 x86 平台：

 +--------+------------+-------------------+------------------+-------------------+------+
 | Window | CFMWS Base |    CFMWS Size     | HDM Decoder Base |  HDM Decoder Size | Ways |
 +========+============+===================+==================+===================+======+
 |   0    |   0 GB     |       2 GB        |      0 GB        |       3 GB        |  12  |
 +--------+------------+-------------------+------------------+-------------------+------+
 |   1    |   4 GB     | NIW**256MB Aligned |      4 GB        | NIW**256MB Aligned |  12  |
 +--------+------------+-------------------+------------------+-------------------+------+

HDM decoder base 和 HDM decoder size 代表一个 12 路区域的全部 12 个 Endpoint Decoder 以及所有中间 Switch Decoder。它们由 BIOS 根据 NIW * 256MB 规则配置，产生 3GB 的 HPA 范围大小。而 CFMWS Base 和 CFMWS Size 用于配置 Root Decoder 的 HPA 范围，结果（2GB）比层次结构中 Switch 和 Endpoint Decoder 的范围（3GB）更小。

这会造成两个问题，导致无法构建区域（region）：

1) Root 与任何 HDM decoder 之间的区域大小不匹配。由于裁减，Root decoder 总是更小。

2) 裁减导致 root decoder 违反（NIW * 256MB）规则。

该改动允许基址为 0GB 的区域绕过这些检查，以便用被裁减的 root decoder 地址范围构建区域。

该改动不允许任何其他任意区域违反这些检查——它专门用于使将 CXL 内存映射到 4GB 以下的 x86 平台能够构建区域。

尽管 HDM decoder 覆盖了 PCIE 空洞的 HPA 区域，但预计平台永远不会把地址访问路由到 CXL 复合体，因为 root decoder 只覆盖被裁减的区域（即排除了该空洞）。这超出了 Linux 能够强制实施的能力范围。

在示例平台上，只有前 2GB 可能可用，但 Linux 为了遵循当前规范，无法构建 Region 并把 Endpoint 与中间 Switch Decoder 挂接到它们上面。

有多个失败点，原因在于人们期望 Root Decoder 的 HPA 大小（等于配置它的 CFMWS 大小）必须大于或等于匹配的 Switch 和 Endpoint HDM Decoder。

为了成功构建并挂接，Linux 必须用 Root Decoder 的 HPA 范围大小构建一个 Region，然后把属于该层次结构的所有中间 Switch Decoder 和 Endpoint Decoder 挂接到该 Region，而不论它们各自的范围大小。

### Benefits of the Change


如果不做此改动，OSPM 将无法把中间 Switch 和 Endpoint Decoder 与配置了不符合 NIW * 256MB 约束的 CFMWS HPA 大小的 Root Decoder 匹配起来，从而导致 memdev 容量丢失。

该改动使 OSPM 能够构建 Region 并把中间 Switch 和 Endpoint Decoder 挂接到它们，从而使内存设备总容量中可寻址的部分对用户可用。

### References


Compute Express Link Specification Revision 3.2, Version 1.0
<https://www.computeexpresslink.org/>

### Detailed Description of the Change


表 9-22 中 Window Size 字段的描述需要顾及存在低内存空洞（Low Memory Holes）的平台，那里 SPA 范围可能是 endpoints HPA 的子集。因此，它需要改为如下内容：

"该窗口所代表的 HPA 连续字节总数。该值应为 NIW * 256 MB 的整数倍。

在保留 4 GB 以下物理地址的平台（例如 x86 上用于 PCIe MMIO 的低内存空洞）上，Base HPA 范围为 0 的某个 CFMWS 实例，其大小可能不符合 NIW * 256 MB 约束。

注意，匹配的中间 Switch Decoder 和 Endpoint Decoder 的 HPA 范围大小仍须符合上述规则，但超出 CFMWS 窗口大小的那部分内存容量将不可访问。"。
