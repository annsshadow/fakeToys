## 统一可扩展固件接口（UEFI）


UEFI，即统一可扩展固件接口（Unified Extensible Firmware Interface），是一份规范，规定了兼容固件接口的行为。它由 UEFI 论坛维护 - http://www.uefi.org/。

UEFI 是其前身 'EFI' 的演进，因此在本文档及相关的源代码中，EFI 和 UEFI 这两个术语在一定程度上可以互换使用。通常，任何新内容都使用 'UEFI'，而 'EFI' 指代遗留（legacy）代码或规范。

## Linux 中的 UEFI 支持


在带有符合 UEFI 规范的固件的平台上启动，使内核能够支持额外的特性：

- UEFI 运行时服务（Runtime Services）
- 通过 UEFI 配置表的标准化接口检索各种配置信息。（ACPI、SMBIOS 等）

要实际启用 [U]EFI 支持，请启用：

- CONFIG_EFI=y
- CONFIG_EFIVAR_FS=y 或 m

该实现依赖于在扁平设备树（Flattened Device Tree，FDT）中接收关于 UEFI 环境的信息——因此仅在 CONFIG_OF 下可用。

## UEFI stub


"stub" 是一项功能，它将 Image/zImage 扩展为一个有效的 UEFI PE/COFF 可执行文件，包含一个加载器应用程序，使得可以直接从 UEFI shell、启动菜单，或像 Gummiboot 或 rEFInd 这样的轻量级引导加载程序加载内核。

带有 stub 支持构建的内核镜像仍然是一个有效的内核镜像，可用于在非 UEFI 环境中启动。

## ARM 上的 UEFI 内核支持


ARM 架构（arm 和 arm64）上的 UEFI 内核支持仅在通过 stub 启动时可用。

在 UEFI 模式下启动时，stub 会从提供的 DT 中删除任何内存节点。相反，内核读取 UEFI 内存映射（memory map）。

stub 会用以下参数填充 FDT 的 /chosen 节点（内核也会扫描这些参数）：

==========================  ======   ===========================================
名称                       类型     描述
==========================  ======   ===========================================
linux,uefi-system-table     64-bit   UEFI 系统表（System Table）的物理地址。

linux,uefi-mmap-start       64-bit   UEFI 内存映射的物理地址，
                                     由 UEFI GetMemoryMap() 调用填充。

linux,uefi-mmap-size        32-bit   上一项所指的 UEFI 内存映射的大小（字节）。

linux,uefi-mmap-desc-size   32-bit   UEFI 内存映射中每个条目的大小（字节）。

linux,uefi-mmap-desc-ver    32-bit   mmap 描述符格式的版本。

kaslr-seed                  64-bit   用于随机化内核镜像基址位置的熵。

bootargs                    String   内核命令行
==========================  ======   ===========================================
