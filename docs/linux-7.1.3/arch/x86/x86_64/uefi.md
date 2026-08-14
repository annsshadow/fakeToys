
## 关于 [U]EFI x86_64 支持的通用说明


本文档中 EFI 与 UEFI 这两个术语可互换使用。

尽管构建内核并不需要使用下列工具，但下文列出了 x86_64 平台上
带有 EFI 固件与规范的引导加载程序支持及相关工具。

1. UEFI 规范：  http://www.uefi.org

2. 在 UEFI x86_64 平台上引导 Linux 内核，既可以使用
   <Documentation/admin-guide/efi-stub.rst>，也可以使用独立的
   引导加载程序。

3. 带有 EFI/UEFI 固件的 x86_64 平台。

### 机制


请参阅 <Documentation/admin-guide/efi-stub.rst> 了解如何使用 EFI stub。

以下是在 x86_64 平台上通用的 EFI 设置指南，无论你使用的是
EFI stub 还是独立的引导加载程序。

```

	CONFIG_FB_EFI=y
	CONFIG_FRAMEBUFFER_CONSOLE=y

  如果需要 EFI 运行时服务，应选择以下配置：

	CONFIG_EFI=y
	CONFIG_EFIVAR_FS=y or m		# 可选

```
- 在磁盘上创建一个带有 EFI System 标志的 VFAT 分区
    你可以使用 fdisk 通过以下命令完成：

        1. g - 初始化一个 GPT 分区表
        2. n - 创建一个新分区
        3. t - 将分区类型改为 “EFI System”（编号 1）
        4. w - 写入并保存更改

```

        mkfs.fat /dev/<your-partition>

```
- 将引导文件复制到 VFAT 分区：
    如果你使用 EFI stub 方式，内核同时也充当 EFI 可执行文件。

    你只需将 bzImage 复制到分区上的 EFI/boot/bootx64.efi 路径，
    它便会自动被引导；关于传递内核参数与 initramfs 的更多说明，
    请参阅 <Documentation/admin-guide/efi-stub.rst> 页面。

    如果你使用自定义引导加载程序，请参考相关文档以获得此部分的帮助。

- 如果部分或全部 EFI 运行时服务无法工作，你可以尝试使用以下
    内核命令行参数来关闭部分或全部 EFI 运行时服务。

	noefi
		关闭所有 EFI 运行时服务
	reboot_type=k
		关闭 EFI 重启运行时服务

- 如果 EFI 内存映射中包含 E820 映射里没有的额外条目，你可以
    使用以下内核命令行参数，将这些条目纳入内核可用物理 RAM 的
    内存映射中。

	add_efi_memmap
		纳入可用物理 RAM 的 EFI 内存映射
