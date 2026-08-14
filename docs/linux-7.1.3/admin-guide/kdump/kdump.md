## Kdump 文档——基于 kexec 的崩溃转储解决方案

本文档包含概述、设置、安装和分析相关信息。

## 概述

Kdump 使用 kexec 在需要获取系统内核内存转储时（例如，当系统发生 panic 时）快速
引导到一个转储捕获（dump-capture）内核。系统内核的内存映像在重启过程中被保留，并且
对转储捕获内核是可见的。

你可以使用常见命令（如 cp、scp 或 makedumpfile）将内存映像复制到本地磁盘上的转储
文件，或通过网络复制到远程系统。

Kdump 和 kexec 目前支持 x86、x86_64、ppc64、s390x、arm 和 arm64 架构。

当系统内核引导时，它会为转储捕获内核保留一小段内存。这确保了来自系统内核的持续
直接内存访问（DMA）不会破坏转储捕获内核。kexec -p 命令将转储捕获内核加载到这段
保留内存中。

在 x86 机器上，无论内核加载在哪里，启动都需要物理内存的前 640 KB。为了简化处理，
整个低 1M 被保留，以避免任何后续内核或设备驱动将数据写入该区域。这样，低 1M 可以被
kdump 内核复用为系统 RAM，而无需额外处理。

在 PPC64 机器上，无论内核加载在哪里，启动都需要物理内存的前 32KB，并且为了支持 64K
页大小，kexec 会备份前 64KB 内存。

对于 s390x，当触发 kdump 时，crashkernel 区域与 [0, crashkernel 区域大小] 区域进行
交换，然后 kdump 内核运行在 [0, crashkernel 区域大小] 中。因此 s390x 不需要可重定位
内核。

关于系统内核核心映像的所有必要信息都以 ELF 格式编码，并在崩溃之前存储在一块保留的
内存区域中。ELF 头起始位置的物理地址通过 elfcorehdr= 引导参数传递给转储捕获内核。
可选地，当使用 elfcorehdr=[size[KMG]@]offset[KMG] 语法时，也可以传递 ELF 头的大小。

通过转储捕获内核，你可以通过 /proc/vmcore 访问内存映像。它将转储导出为一个 ELF 格式
的文件，你可以使用 cp 或 scp 等文件复制命令将其写出。你也可以使用 makedumpfile 工具
通过选项来分析和写出经过过滤的内容，例如使用 '-d 31' 时它将只写出内核数据。此外，你
可以使用 GNU 调试器（GDB）和 Crash 工具等分析工具来调试转储文件。此方法确保转储页
被正确排序。

## 设置与安装

### 安装 kexec-tools

1) 以 root 用户登录。

2) 从以下 URL 下载 kexec-tools 用户空间包：

http://kernel.org/pub/linux/utils/kernel/kexec/kexec-tools.tar.gz

这是一个指向最新版本的符号链接。

最新的 kexec-tools git 树可在以下位置获取：

- git://git.kernel.org/pub/scm/utils/kernel/kexec/kexec-tools.git
- http://www.kernel.org/pub/scm/utils/kernel/kexec/kexec-tools.git

还有一个 gitweb 接口可用：
http://www.kernel.org/git/?p=utils/kernel/kexec/kexec-tools.git

关于 kexec-tools 的更多信息可在以下位置找到：
http://horms.net/projects/kexec/

```
	tar xvpzf kexec-tools.tar.gz
```

```
	cd kexec-tools-VERSION
```

```
	./configure
```

```
	make
```

```
	make install
```

### 构建系统和转储捕获内核

使用 Kdump 有两种可能的方法。

1) 构建一个单独的定制转储捕获内核来捕获内核核心转储。

2) 或者使用系统内核二进制本身作为转储捕获内核，而无需构建单独的转储捕获内核。这只
   在支持可重定位内核的架构上才可能。截至目前，i386、x86_64、ppc64、arm 和 arm64
   架构支持可重定位内核。

构建可重定位内核的优势在于无需为捕获转储而构建第二个内核。但同时，人们可能希望构建
一个适合自己需求的定制转储捕获内核。

以下是为系统和转储捕获内核启用 kdump 支持所需的配置设置。

### 系统内核配置选项

1) 在 "Processor type and features"（处理器类型和特性）中启用 "kexec system call"
   （kexec 系统调用）或 "kexec file based system call"（基于文件的 kexec 系统调用）：

```
	CONFIG_KEXEC=y 或 CONFIG_KEXEC_FILE=y

   并且它们两者都会选择 KEXEC_CORE::

	CONFIG_KEXEC_CORE=y
```

2) 在 "Filesystem"（文件系统）-> "Pseudo filesystems"（伪文件系统）中启用 "sysfs file
   system support"（sysfs 文件系统支持）：

```
	CONFIG_SYSFS=y

   注意，如果 "General Setup"（通用设置）中没有启用 "Configure standard kernel
   features (expert users)"（配置标准内核特性（专家用户）），那么 "sysfs file system
   support" 可能不会出现在 "Pseudo filesystems" 菜单中。在这种情况下，请直接检查
   .config 文件本身以确保 sysfs 被打开，如下所示::

	grep 'CONFIG_SYSFS' .config
```

```
	CONFIG_DEBUG_INFO=Y

   这将使内核以调试符号构建。转储分析工具需要带有调试符号的 vmlinux 才能读取和分析
   转储文件。
```

### 转储捕获内核配置选项（架构无关）

1) 在 "Processor type and features"（处理器类型和特性）下启用 "kernel crash dumps"
   （内核崩溃转储）支持：

```
	CONFIG_CRASH_DUMP=y

   并且这将选择 VMCORE_INFO 和 CRASH_RESERVE::
	CONFIG_VMCORE_INFO=y
	CONFIG_CRASH_RESERVE=y
```

```
	CONFIG_PROC_VMCORE=y

   （当选择 CONFIG_CRASH_DUMP 时，CONFIG_PROC_VMCORE 默认被设置。）
```

### 转储捕获内核配置选项（架构相关，i386 和 x86_64）

1) 在 i386 上，在 "Processor type and features" 下启用高端内存支持：

```
	CONFIG_HIGHMEM4G
```

2) 在 CONFIG_SMP=y 的情况下，通常在加载转储捕获内核时需要在内核命令行指定 nr_cpus=1，
   因为对大多数系统而言，kdump 内核用一个 CPU 来转储 vmcore 就足够了。

   但是，你也可以指定 nr_cpus=X 以在 kdump 内核中启用多个处理器。

   在 CONFIG_SMP=n 的情况下，上述事项与之无关。

3) 建议默认构建一个可重定位内核。如果尚未构建，请在 "Processor type and features" 下
   启用 "Build a relocatable kernel"（构建可重定位内核）支持：

```
	CONFIG_RELOCATABLE=y
```

4) 为 "Physical address where the kernel is loaded"（内核加载的物理地址）（在
   "Processor type and features" 下）使用一个合适的值。这仅当 "kernel crash dumps" 被
   启用时才会出现。合适的值取决于内核是否可重定位。

   如果你使用的是可重定位内核，使用 CONFIG_PHYSICAL_START=0x100000。这将为物理地址
   1MB 编译内核，但鉴于内核是可重定位的，它可以从任何物理地址运行，因此 kexec 引导
   加载程序会将其加载到为转储捕获内核保留的内存区域中。

   否则，它应该是使用引导参数 "crashkernel=Y@X" 为第二个内核保留的内存区域的起始位置。
   这里 X 是为转储捕获内核保留的内存区域的起始位置。通常 X 是 16MB（0x1000000）。所以
   你可以设置 CONFIG_PHYSICAL_START=0x1000000。

5) 构建并安装内核及其模块。不要将此内核添加到引导加载程序的配置文件中。

### 转储捕获内核配置选项（架构相关，ppc64）

```
	CONFIG_CRASH_DUMP=y
```

```
	CONFIG_RELOCATABLE=y

   构建并安装内核及其模块。
```

### 转储捕获内核配置选项（架构相关，arm）

- 要使用可重定位内核：

```
	AUTO_ZRELADDR=y
```

### 转储捕获内核配置选项（架构相关，arm64）

- 请注意，即使在非 VHE 系统上配置了 dump-capture 内核的 kvm，它也不会被启用。这是
  因为 CPU 在 panic 时不会被重置到 EL2。

## crashkernel 语法

1) crashkernel=size@offset

   'size' 指定为转储捕获内核保留多少内存，'offset' 指定这段保留内存的起始位置。例如，
   "crashkernel=64M@16M" 告诉系统内核从物理地址 0x01000000（16MB）开始为转储捕获内核
   保留 64 MB 内存。

   崩溃内核区域可以由系统内核在运行时自动放置。这是通过将基址指定为 0 来完成的：

```
         crashkernel=256M@0
```

或：

```
         crashkernel=256M
```

   如果指定了起始地址，请注意内核的起始地址会对齐到一个值（该值依赖于架构），所以如果
   起始地址未对齐，那么对齐点以下的任何空间都将被浪费。

2) range1:size1[,range2:size2,...][@offset]

   虽然 "crashkernel=size[@offset]" 语法对大多数配置来说已经足够，但有时让保留内存
   依赖于系统 RAM 的大小会很方便——这主要是为那些预先设置好内核命令行以避免在从机器中
   移除部分内存后系统无法启动的发行版准备的。

```
       crashkernel=<range1>:<size1>[,<range2>:<size2>,...][@offset]
       range=start-[end]

   例如::

       crashkernel=512M-2G:64M,2G-:128M
```

   这意味着：

       1) 如果 RAM 小于 512M，则不保留任何内存（这是"救援"情况）
       2) 如果 RAM 大小在 512M 和 2G 之间（不含），则保留 64M
       3) 如果 RAM 大小大于 2G，则保留 128M

3) crashkernel=size,high 和 crashkernel=size,low

   如果偏好 4G 以上的内存，可以使用 crashkernel=size,high 来满足。使用它时，允许从顶端
   分配物理内存，因此如果系统安装了超过 4G 的 RAM，它可能在 4G 以上。否则，如果可用，
   内存区域将分配在 4G 以下。

   当传入 crashkernel=X,high 时，内核可能分配 4G 以上的物理内存区域，这种情况下需要 4G
   以下的低内存。有三种方式获取低内存：

      1) 如果未指定 crashkernel=Y,low，内核会自动在 4G 以下分配至少 256M 内存。
      2) 改为让用户指定低内存大小。

```
            crashkernel=0,low
```

4) crashkernel=size,cma

	从 CMA 中保留额外的崩溃内核内存。这段保留内存可以被第一个系统（first system）的
	用户空间内存和内核可移动分配（内存气球、zswap）使用。从该内存范围分配的页不会被
	包含在 vmcore 中，因此如果打算转储用户空间内存，并且可以预期某些可移动内核页可能
	会从转储中缺失，则不应使用此选项。

	如上所述，仍然需要一个标准的 crashkernel 保留，以容纳崩溃内核和 initrd。

	此选项增加了 kdump 失败的风险：第一个内核配置的 DMA 传输最终可能破坏第二个内核的
	内存。

	这种保留方法适用于那些无法为标准的 crashkernel 保留牺牲足够内存、且较不可靠、可能
	不完整的 kdump 也优于完全没有 kdump 的系统。

### 引导进入系统内核

1) 根据需要更新引导加载程序（如 grub、yaboot 或 lilo）的配置文件。

2) 使用引导参数 "crashkernel=Y@X" 引导系统内核。

   在 x86 和 x86_64 上，使用 "crashkernel=Y[@X]"。大多数时候，起始地址 'X' 不是必需的，
   内核会搜索一个合适的区域。除非期望一个显式的起始地址。

   在 ppc64 上，使用 "crashkernel=128M@32M"。

   在 s390x 上，通常使用 "crashkernel=xxM"。xx 的值取决于 kdump 系统的内存消耗。一般
   来说，这不依赖于生产系统的内存大小。

   在 arm 上，不再需要 "crashkernel=Y@X"；如果未给定 X，内核将自动在 RAM 的前 512MB
   内定位崩溃内核映像。

   在 arm64 上，使用 "crashkernel=Y[@X]"。注意，如果显式指定，内核的起始地址 X 必须对
   齐到 2MiB（0x200000）。

## 加载转储捕获内核

引导进入系统内核后，需要加载转储捕获内核。

基于架构和映像类型（是否可重定位），可以选择加载转储捕获内核的未压缩 vmlinux 或
压缩 bzImage/vmlinuz。以下是摘要。

对于 i386 和 x86_64：

 - 如果内核可重定位，使用 bzImage/vmlinuz。
 - 如果内核不可重定位，使用 vmlinux。

对于 ppc64：

 - 使用 vmlinux

对于 s390x：

 - 使用 image 或 bzImage

对于 arm：

 - 使用 zImage

对于 arm64：

 - 使用 vmlinux 或 Image

如果你使用的是未压缩的 vmlinux 映像，则使用以下命令：

```
   kexec -p <dump-capture-kernel-vmlinux-image> \
   --initrd=<initrd-for-dump-capture-kernel> --args-linux \
   --append="root=<root-dev> <arch-specific-options>"
```

如果你使用的是压缩的 bzImage/vmlinuz，则使用以下命令：

```
   kexec -p <dump-capture-kernel-bzImage> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

如果你使用的是压缩的 zImage，则使用以下命令：

```
   kexec --type zImage -p <dump-capture-kernel-bzImage> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --dtb=<dtb-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

如果你使用的是未压缩的 Image，则使用以下命令：

```
   kexec -p <dump-capture-kernel-Image> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

以下是在加载转储捕获内核时要使用的架构相关命令行选项。

对于 i386 和 x86_64：

	"1 irqpoll nr_cpus=1 reset_devices"

对于 ppc64：

	"1 maxcpus=1 noirqdistrib reset_devices"

对于 s390x：

	"1 nr_cpus=1 cgroup_disable=memory"

对于 arm：

	"1 maxcpus=1 reset_devices"

对于 arm64：

	"1 nr_cpus=1 reset_devices"

关于加载转储捕获内核的注意事项：

- 默认情况下，ELF 头以 ELF64 格式存储，以支持内存超过 4GB 的系统。在 i386 上，kexec
  会自动检查物理 RAM 大小是否超过 4 GB 限制，如果没有，则使用 ELF32。因此，在非 PAE
  系统上，始终使用 ELF32。

  --elf32-core-headers 选项可用于强制生成 ELF32 头。这是必要的，因为 GDB 目前在 32 位
  系统上无法打开带有 ELF64 头的 vmcore 文件。

- "irqpoll" 引导参数可减少转储捕获内核中由于共享中断导致的驱动初始化失败。

- 你必须以与 mount 命令输出中的根设备名相对应的格式指定 <root-dev>。

- 引导参数 "1" 将转储捕获内核引导到单用户模式且不带网络。如果你想要网络，使用 "3"。

- 我们一般不必仅仅为了捕获转储而启动一个 SMP 内核。因此，通常构建 UP 转储捕获内核或在
  加载转储捕获内核时指定 maxcpus=1 选项是有用的。不过注意，虽然 maxcpus 总是有效，如果
  当前 ARCH（如 x86）支持，你最好用 nr_cpus 替换它以节省内存。

- 如果你打算在其中使用多线程程序（例如 makedumpfile 的并行转储功能），你应该在转储
  捕获内核中启用多 CPU 支持。否则，多线程程序可能会有严重的性能下降。要启用多 CPU 支持，
  你应该启动一个 SMP 转储捕获内核，并在加载它时指定 maxcpus/nr_cpus 选项。

- 对于 s390x 有两种 kdump 模式：如果使用 elfcorehdr= 内核参数指定了 ELF 头，则它像在
  所有其他架构上一样被 kdump 内核使用。如果未指定 elfcorehdr= 内核参数，s390x kdump
  内核会动态地创建该头。第二种模式的优势在于，对于 CPU 和内存热插拔，无需用
  kexec_load() 重新加载 kdump。

- 对于带有许多附属设备的 s390x 系统，kdump 内核应该使用 "cio_ignore" 内核参数，以防止
  为与 kdump 无关的设备分配内核内存。这同样适用于使用 SCSI/FCP 设备的系统。在这种
  情况下，在将 FCP 设备上线之前，应将 "allow_lun_scan" zfcp 模块参数设置为零。

## 内核 Panic

在如前所述成功加载转储捕获内核之后，如果触发了系统崩溃，系统将重启进入转储捕获内核。
触发点位于 panic()、die()、die_nmi() 以及 sysrq 处理程序（ALT-SysRq-c）中。

以下条件会执行崩溃触发点：

如果检测到硬锁定（hard lockup）且配置了 "NMI watchdog"，系统将引导进入转储捕获内核
（die_nmi()）。

如果调用了 die()，并且它恰好是 pid 为 0 或 1 的线程，或者 die() 在中断上下文中被调用，
或者调用了 die() 且设置了 panic_on_oops，系统将引导进入转储捕获内核。

在 powerpc 系统上，当生成软重置（soft-reset）时，所有 CPU 都会调用 die()，系统将引导
进入转储捕获内核。

出于测试目的，你可以使用 "ALT-SysRq-c"、"echo c > /proc/sysrq-trigger" 或编写一个
模块来强制 panic 以触发崩溃。

## 写出转储文件

转储捕获内核引导后，使用以下命令写出转储文件：

```
   cp /proc/vmcore <dump-file>
```

```
   scp /proc/vmcore remote_username@remote_ip:<dump-file>
```

你也可以使用 makedumpfile 工具写出转储文件：

```
   makedumpfile -l --message-level 1 -d 31 /proc/vmcore <dump-file>
```

## 分析

在分析转储映像之前，你应该重启进入一个稳定的内核。

你可以使用 GDB 对从 /proc/vmcore 复制出来的转储文件做有限的分析。使用带有 -g 构建的
调试 vmlinux 并运行以下命令：

```
   gdb vmlinux <dump-file>
```

处理器 0 上任务的栈回溯、寄存器显示和内存显示都工作正常。

注意：GDB 无法分析 x86 上以 ELF64 格式生成的核心文件。在最多 4GB 内存的系统上，你
可以在转储内核上使用 --elf32-core-headers 内核选项生成 ELF32 格式的头。

你也可以使用 Crash 工具来分析 Kdump 格式的转储文件。Crash 可在以下 URL 获取：

   https://github.com/crash-utility/crash

Crash 文档可在以下位置找到：
   https://crash-utility.github.io/

## 在 WARN() 上触发 Kdump

内核参数 panic_on_warn 会在所有 WARN() 路径中调用 panic()。这将导致在 panic() 调用
处发生 kdump。在用户想要在运行时指定此行为的情况下，可以将 /proc/sys/kernel/panic_on_warn
设置为 1 来实现相同的行为。

## 在 add_taint() 上触发 Kdump

内核参数 panic_on_taint 便于在 add_taint() 内部有条件地调用 panic()，只要此位掩码中
设置的值与 add_taint() 正在设置的位标志相匹配。这将导致在 add_taint()->panic() 调用
处发生 kdump。

## 将转储文件写入加密磁盘卷

可以启用 CONFIG_CRASH_DM_CRYPT 以支持将转储文件保存到加密磁盘卷（目前仅支持 x86_64）。
用户空间可以通过 /sys/kernel/config/crash_dm_crypt_keys 进行交互设置：

1. 告诉第一个内核需要哪些 logon 密钥来解锁磁盘卷：

    # 添加密钥 #1
    mkdir /sys/kernel/config/crash_dm_crypt_keys/7d26b7b4-e342-4d2d-b660-7426b0996720
    # 添加密钥 #1 的描述
    echo cryptsetup:7d26b7b4-e342-4d2d-b660-7426b0996720 > /sys/kernel/config/crash_dm_crypt_keys/description

    # 我们现在有多少个密钥？
    cat /sys/kernel/config/crash_dm_crypt_keys/count
    1

    # 以相同方式添加密钥 #2

    # 我们现在有多少个密钥？
    cat /sys/kernel/config/crash_dm_crypt_keys/count
    2

    # 为支持 CPU/内存热插拔，复用已保存到保留内存的密钥
    echo true > /sys/kernel/config/crash_dm_crypt_key/reuse

2. 加载转储捕获内核

3. 在转储捕获内核引导之后，将密钥恢复到用户密钥环：
   echo yes > /sys/kernel/config/crash_dm_crypt_keys/restore

## 联系方式

- kexec@lists.infradead.org

## GDB 宏

   :literal:
