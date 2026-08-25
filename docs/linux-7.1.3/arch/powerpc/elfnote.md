## ELF Note PowerPC 命名空间


内核二进制文件中 ELF Note PowerPC 命名空间用于存储可供引导加载程序（bootloader）或用户态使用的功能与信息
### 类型与描述符


"PowerPC" 命名空间一起使用的类型定义[#f1]_ 中
 1) PPC_ELFNOTE_CAPABILITIES

定义内核支持/所需的功能。该类型使用位图（bitmap）作"descriptor" 字段。每一位如下所述：

- 支持 Ultravisor 的位（仅 PowerNV）

	#define PPCCAP_ULTRAVISOR_BIT (1 << 0)

表示 powerpc 内核二进制知道如何在启用ultravisor 的系统中运行
在启用了 ultravisor 的系统中，部分机器资源现在由 ultravisor 控制。如果内核不支持 ultravisor，但最终在带有 ultravisor 的机器上运行，内核在尝试访问 ultravisor 资源时可能会崩溃。例如，它可能在早期启动阶段尝试设置分区表项 0 时崩溃
在启用了 ultravisor 的系统中，如PowerPC ultravisor 能力不存在或未设置“支Ultravisor”位，引导加载程序可以警告用户或阻止内核运行
### 参考资