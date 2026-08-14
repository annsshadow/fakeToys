## 通过 initrd 升级 ACPI 表


## 这是什么

如果 ACPI_TABLE_UPGRADE 编译选项为真，就可以通过用经过改造的、修改过的、更新的
版本替换 BIOS 提供的 ACPI 表，来升级由 ACPI 表定义的 ACPI 执行环境，或者安装全新的
ACPI 表。

当在内核与 initrd 合并到单一镜像中构建时，此功能还需将
ACPI_TABLE_OVERRIDE_VIA_BUILTIN_INITRD 选项也设为真才能工作。

关于可升级/安装的全部 ACPI 表，请查看 drivers/acpi/tables.c 中
`char *table_sigs[MAX_ACPI_SIGNATURE];` 的定义。

iasl（Intel 的 ACPI 编译器与反汇编器）认识的所有 ACPI 表都应可被覆盖，除了：

  - ACPI_SIG_RSDP（签名为 6 字节）
  - ACPI_SIG_FACS（没有普通的 ACPI 表头）

这两者将来也可能被实现。


## 用途

如果你发现一个严重到 Linux 内核无法接受变通方案的 bug，请向你的平台/BIOS 厂商投诉。
而此功能允许你在平台/BIOS 厂商发布升级后的 BIOS 二进制之前，先升级有 bug 的表。

平台/BIOS 厂商可以利用此功能在不修改底层平台固件的情况下，提供一个与 Linux 兼容的
环境。

此功能还提供了一个强大的特性，可以通过修改平台提供的旧 ACPI 表或插入新的 ACPI 表，
来轻松调试和测试 ACPI BIOS 表与 Linux 内核的兼容性。

它可以在任何内核中启用，因为对未经过改造的 initrd 来说没有任何功能变化。


## 工作原理

```

  # 提取本机的 ACPI 表：
  cd /tmp
  acpidump >acpidump
  acpixtract -a acpidump
  # 反汇编、修改并重新编译：
  iasl -d *.dat
  # 例如将如下语句加入 DSDT（PCI 路由表）函数的 _PRT 中：
  Store("HELLO WORLD", debug)
  # 并增大 OEM Revision。例如，修改前：
  DefinitionBlock ("DSDT.aml", "DSDT", 2, "INTEL ", "TEMPLATE", 0x00000000)
  # 修改后：
  DefinitionBlock ("DSDT.aml", "DSDT", 2, "INTEL ", "TEMPLATE", 0x00000001)
  iasl -sa dsdt.dsl
  # 将原始 ACPI 表加入一个未压缩的 cpio 归档。
  # 它们必须放在 cpio 归档内的 /kernel/firmware/acpi 目录下。注意，如果放在这里的表
  # 与平台表（相似的表签名、相似的 OEMID、相似的 OEM 表 ID）匹配且拥有更新的 OEM
  # Revision，平台表将被此表升级。如果放在这里的表与平台表不匹配（不同的表签名，或
  # 不同的 OEMID，或不同的 OEM 表 ID），此表将被追加。
  mkdir -p kernel/firmware/acpi
  cp dsdt.aml kernel/firmware/acpi
  # 目前最多允许 "NR_ACPI_INITRD_TABLES (64)" 个表（见 osl.c）：
  iasl -sa facp.dsl
  iasl -sa ssdt1.dsl
  cp facp.aml kernel/firmware/acpi
  cp ssdt1.aml kernel/firmware/acpi
  # 未压缩的 cpio 归档必须放在最前面。其它（通常是压缩的）cpio 归档必须拼接在其后。
  # 下面命令创建未压缩的 cpio 归档，并将原始 initrd 拼接其后：
  find kernel | cpio -H newc --create > /boot/instrumented_initrd
  cat /boot/initrd >>/boot/instrumented_initrd
  # 以增大的 acpi 调试级别重启，例如启动参数：
  acpi.debug_level=0x2 acpi.debug_layer=0xFFFFFFFF
  # 然后检查你的 syslog：
  [    1.268089] ACPI: PCI Interrupt Routing Table [\_SB_.PCI0._PRT]
  [    1.272091] [ACPI Debug]  String [0x0B] "HELLO WORLD"

```
iasl 能够反汇编并重新编译相当多种不同的静态 ACPI 表。


## 在哪里获取用户态工具

iasl 和 acpixtract 是 Intel ACPICA 项目的一部分：
https://acpica.org/

并且应当由发行版打包提供（例如在 SUSE 上的 acpica 包中）。

acpidump 可以在 Len Brown 的 pmtools 中找到：
ftp://kernel.org/pub/linux/kernel/people/lenb/acpi/utils/pmtools/acpidump

该工具在 SUSE 上也是 acpica 包的一部分。另外，在最新内核中可以通过 sysfs 获取已用的
ACPI 表：
/sys/firmware/acpi/tables
