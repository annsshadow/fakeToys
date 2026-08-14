
## 启动 Linux/LoongArch


:Author: Yanteng Si <siyanteng@loongson.cn>
:Date:   18 Nov 2022

## 从引导加载程序传递给内核的信息


LoongArch 支持 ACPI 和 FDT。需要传递给内核的信息包括 memmap、initrd、命令行，以及可选的 ACPI/FDT 表等。

内核在 `kernel_entry` 处接收以下参数：

      - a0 = efi_boot：`efi_boot` 是一个标志，指示此引导环境是否完全符合 UEFI。

      - a1 = cmdline：`cmdline` 是指向内核命令行的指针。

      - a2 = systemtable：`systemtable` 指向 EFI 系统表。此阶段涉及的所有指针都是物理地址。

## Linux/LoongArch 内核映像头部


Linux/LoongArch 内核映像是 EFI 映像。作为 PE 文件，它们具有
```
	u32	MZ_MAGIC                /* "MZ"，MS-DOS 头部 */
	u32	res0 = 0                /* 保留 */
	u64	kernel_entry            /* 内核入口点 */
	u64	_end - _text            /* 内核映像有效大小 */
	u64	load_offset             /* 内核映像距 RAM 起始的加载偏移 */
	u64	res1 = 0                /* 保留 */
	u64	res2 = 0                /* 保留 */
	u64	res3 = 0                /* 保留 */
	u32	LINUX_PE_MAGIC          /* 魔数 */
	u32	pe_header - _head       /* 到 PE 头部的偏移 */
```
