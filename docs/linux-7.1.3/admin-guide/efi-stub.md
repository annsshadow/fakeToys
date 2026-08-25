## EFI 引导存根（EFI Boot Stub

x86 ARM 平台上，内核 zImage/bzImage 可以伪装PE/COFF 镜像，从而说EFI 固件
加载器将其作EFI 可执行文件加载。修bzImage 头部的代码，连同固件加载器跳转到EFI 特定入口点，统称为“EFI 引导存根（EFI boot stub）”，分别位于
arch/x86/boot/header.S drivers/firmware/efi/libstub/x86-stub.c。对ARM，EFI 存根
实现arch/arm/boot/compressed/efi-header.S drivers/firmware/efi/libstub/arm32-stub.c。各架构间共享的 EFI 存根代码位于
drivers/firmware/efi/libstub銆。
对于 arm64，没有压缩内核支持，因此 Image 自身伪装PE/COFF 镜像，EFI 存根被链接进
内核。arm64 EFI 存根位于 drivers/firmware/efi/libstub/arm64.c drivers/firmware/efi/libstub/arm64-stub.c
通过使用 EFI 引导存根，可以在不使用传EFI 引导加载器（grub elilo）的情况引导 Linux 内核。由EFI 引导存根承担了引导加载器的工作，在某种意义上*就是**引导
加载器
EFI 引导存根通过 CONFIG_EFI_STUB 内核选项启用

### 如何安装 bzImage.efi


位于 arch/x86/boot/bzImage bzImage 必须复制EFI 系统分区（ESP），并改名为扩展efi”。没有该扩展名，EFI 固件加载器会拒绝执行它。无法从常用Linux 文件系统执行
bzImage.efi，因EFI 固件不支持它们。对ARM，应arch/arm/boot/zImage 复制到系分区，可能不需要改名。类似地，对arm64，应复制 arch/arm64/boot/Image，但不一定要改名

### EFI shell 传递内核参

```

	fs0:> bzImage.efi console=ttyS0 root=/dev/sda4


```
### "initrd=" 选项


与大多数引导加载器一样，EFI 存根允许用户使用 "initrd=" 选项指定多个 initrd 文件。这唯一 EFI 存根特定的命令行参数，其余内容在内核引导时传给内核
initrd 文件的路径必须是ESP 起始的绝对路径，相对路径名不起作用。此外，该路径是 EFI
风格的路径，目录元素必须用以下分隔符分隔
```

  fs0:>
	Kernels\
			bzImage.efi
			initrd-large.img

	Ramdisks\
			initrd-small.img
			initrd-medium.img

```
要在当前工作目录下以 initrd-large.img 文件引导
```

	fs0:\Kernels> bzImage.efi initrd=\Kernels\initrd-large.img

```
注意，bzImage.efi 是如何可以用相对路径指定的。这是因为我们正在执行的镜像EFI shell
解释，EFI shell 理解相对路径，而命令行的其余部分则传给 bzImage.efi
   也可以在引导时使Linux 特定UEFI 协议提供 initrd。详pe-coff-entry-point
### "dtb=" 选项


对于 ARM arm64 架构，必须向内核提供设备树。通常固件应通过 EFI CONFIGURATION TABLE
提供设备树。然而，"dtb=" 命令行选项可用于覆盖固件提供的设备树，或在固件无法提供时提一个
请注意：固件在引导内核之前会向设备树添加运行时配置信息。如果使dtb= 覆盖设备树，
则固件提供的任何运行时数据都会丢失dtb=" 选项只应作为调试工具，或作为EFI
CONFIGURATION TABLE 中未提供设备树时的最后手段使用
"dtb=" 的处理方式与上述 "initrd=" 选项相同