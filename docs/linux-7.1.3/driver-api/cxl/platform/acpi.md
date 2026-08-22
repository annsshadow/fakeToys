
## ACPI 琛。

ACPI 是“高级配置与电源接口”（Advanced Configuration and Power Interface），是一定义平台与操作系统如何管理电源以及配置计算机硬件的标准。出于本操作理论的目的，提到“ACPI”时，我们通常指的是“ACPI 表”——平台（BIOS/EFI）向操作系统传递静态配信息的方式
以下 ACPI 表包含关CXL 设备*静*配置与性能数据
- [acpi/cedt.rst](acpi/cedt.rst)
- [acpi/srat.rst](acpi/srat.rst)
- [acpi/hmat.rst](acpi/hmat.rst)
- [acpi/slit.rst](acpi/slit.rst)
- [acpi/dsdt.rst](acpi/dsdt.rst)

SRAT 表也可能包含通用的端发起者（initiator）内容，旨在描述通用端口，但不包通往端点路径其余部分的信息
Linux 使用这些表来为静态配置（BIOS/EFI）的 CXL 设备配置内核资源，例如：

- NUMA 节点
- 内存分层（Memory Tiers- NUMA 抽象距离（Abstract Distances- SystemRAM 内存区域
- 加权交错节点权重（Weighted Interleave Node Weights
## ACPI 调试


`acpidump -b` 命令ACPI 表转储为二进制格式
`iasl -d` 命令将文件反汇编为人类可读的格式
```

   [000h 0000   4]   Signature : "CEDT"    [CXL Early Discovery Table]

```
### 常见问题


此处描述的大多数失败会导致驱动无法将内存作为 DAX 设备kmem 呈现
- CEDT CFMWS 目标列表 UID CEDT CHBS UID 不匹配- CEDT CFMWS 目标列表 UID DSDT CXL 主桥 UID 不匹配- CEDT CFMWS 限制位不正确- CEDT CFMWS 内存区域对齐不良- CEDT CFMWS 内存区域跨越了平台内存空洞- CEDT CHBS UID DSDT CXL 主桥 UID 不匹配- CEDT CHBS 规范版本不正确- SRAT 缺少 CEDT CFMWS 中描述的区域
  - 结果：无法为该区域创NUMA 节点，或者该区域被放入错误的节点
- HMAT 缺少 CEDT CFMWS 中描述的区域的数据
  - 结果：NUMA 节点被放入错误的内存分层
- SLIT 有错误数据
  - 结果：内核中许多性能机制会非常不满
所有这些问题在用户看来都像是驱动未能支CXL——而实际上它们都是平台未能正确配置
ACPI 表所导致的失败