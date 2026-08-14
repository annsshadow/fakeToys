## 早期用户空间支持


Last update: 2004-12-20 tlh


"早期用户空间"（Early userspace）是一组库和程序，它们提供各种重要的功能，这些功能
在 Linux 内核启动过程中就需要可用，但不需要在内核本身内部运行。

它由几个主要的基础设施组件组成：

- gen_init_cpio，一个构建包含根文件系统映像的 cpio 格式归档的程序。该归档被压缩，
  压缩后的映像被链接进内核映像。
- initramfs，一段在内核引导过程中途解包压缩的 cpio 映像的代码。
- klibc，一个用户空间 C 库，目前单独打包，针对正确性和小体积进行了优化。

initramfs 使用的 cpio 文件格式是 "newc"（即 "cpio -H newc"）格式，并在文件
"buffer-format.txt" 中有文档说明。有两种方式添加早期用户空间映像：指定一个现有的
cpio 归档用作映像，或者让内核构建过程根据规格说明构建映像。

### CPIO 归档方法


你可以创建一个包含早期用户空间映像的 cpio 归档。你的 cpio 归档应在
CONFIG_INITRAMFS_SOURCE 中指定，并将被直接使用。CONFIG_INITRAMFS_SOURCE 中只能指定
单个 cpio 文件，并且目录和文件名不允许与 cpio 归档组合使用。

### 映像构建方法


内核构建过程也可以从源部件构建早期用户空间映像，而不是提供 cpio 归档。此方法提供了
一种创建具有 root 拥有文件映像的方式，即使该映像是由非特权用户构建的。

该映像在 CONFIG_INITRAMFS_SOURCE 中指定为一个或多个源。源可以是目录或文件——从源
构建时**不**允许使用 cpio 归档。

一个源目录将连同其所有内容一起被打包。指定的目录名将被映射到 '/'。在打包目录时，可以
执行有限的用户和组 ID 转换。INITRAMFS_ROOT_UID 可以设置为需要映射到用户 root（0）的
用户 ID。INITRAMFS_ROOT_GID 可以设置为需要映射到组 root（0）的组 ID。

一个源文件必须是 usr/gen_init_cpio 工具所需的格式指令（运行 'usr/gen_init_cpio -h'
获取文件格式）。文件中的指令将被直接传递给 usr/gen_init_cpio。

当指定目录和文件的组合时，initramfs 映像将是它们所有内容的聚合。通过这种方式，用户
可以创建一个 'root-image' 目录并将所有文件安装到其中。由于设备特殊文件无法由非特权
用户创建，特殊文件可以列在 'root-files' 文件中。'root-image' 和 'root-files' 都可以
列在 CONFIG_INITRAMFS_SOURCE 中，并且完整的早期用户空间映像可以由非特权用户构建。

作为一个技术说明，当指定目录和文件时，整个 CONFIG_INITRAMFS_SOURCE 被传递给
usr/gen_initramfs.sh。这意味着 CONFIG_INITRAMFS_SOURCE 实际上可以被解释为传给
gen_initramfs.sh 的任何合法参数。如果指定了一个目录作为参数，则扫描其内容，执行
uid/gid 转换，并输出 usr/gen_init_cpio 文件指令。如果指定了一个文件作为
usr/gen_initramfs.sh 的参数，则文件内容被简单地复制到输出。来自目录扫描和文件内容
复制的所有输出指令都由 usr/gen_init_cpio 处理。

另请参阅 'usr/gen_initramfs.sh -h'。

## 这一切将走向何方？


klibc 发行版包含一些使早期用户空间有用所需的必要软件。klibc 发行版目前与内核分开
维护。

你可以从 https://www.kernel.org/pub/linux/libs/klibc/ 获取不太频繁的 klibc 快照。

对于活跃用户，你最好使用 klibc 的 git 仓库，位于
https://git.kernel.org/?p=libs/klibc/klibc.git

除了 klibc 库之外，独立的 klibc 发行版目前提供三个组件：

- ipconfig，一个配置网络接口的程序。它可以静态配置它们，或者使用 DHCP 动态获取
  信息（即 "IP 自动配置"）。
- nfsmount，一个可以挂载 NFS 文件系统的程序。
- kinit，使用 ipconfig 和 nfsmount 来替换旧的 IP 自动配置支持、通过 NFS 挂载文件
  系统、并使用该文件系统作为根继续系统引导的"粘合剂"。

kinit 被构建为单个静态链接的二进制以节省空间。

最终，希望有更多的内核功能块移动到早期用户空间：

- 几乎全部的 init/do_mounts*（这部分的开头已经就位）
- ACPI 表解析
- 插入实际上不需要在内核空间的不易处理的子系统

如果 kinit 不能满足你当前的需求并且你有字节可挥霍，klibc 发行版包含一个小型的
Bourne 兼容 shell（ash）以及许多其他实用程序，因此你可以替换 kinit 并构建完全满足
你需求的自定义 initramfs 映像。

有关问题和帮助，你可以在 https://www.zytor.com/mailman/listinfo/klibc 注册早期用户空间
邮件列表。

## 它是如何工作的？


内核目前有 3 种方式来挂载根文件系统：

a) 所有必需的设备和文件系统驱动都编译进内核，没有 initrd。init/main.c:init() 将调用
   prepare_namespace() 来挂载最终的根文件系统，基于 root= 选项和可选的 init= 来运行
   不同于 init/main.c:init() 末尾列出的某个其他 init 二进制。

b) 一些设备和文件系统驱动构建为模块并存储在 initrd 中。initrd 必须包含一个二进制
   '/linuxrc'，它应该加载这些驱动模块。也可以通过 linuxrc 挂载最终的根文件系统并使用
   pivot_root 系统调用。initrd 通过 prepare_namespace() 挂载并执行。

c) 使用 initramfs。必须跳过对 prepare_namespace() 的调用。这意味着必须有一个二进制来做
   所有工作。该二进制可以通过修改 usr/gen_init_cpio.c 或通过新的 initrd 格式（一个 cpio
   归档）存储到 initramfs 中。它必须被命名为 "/init"。该二进制负责完成 prepare_namespace()
   会做的所有事情。

   为了保持向后兼容性，/init 二进制只有在它来自 initramfs cpio 归档时才会运行。如果不是
   这种情况，init/main.c:init() 将运行 prepare_namespace() 来挂载最终的根并执行预定义的
   init 二进制之一。

Bryan O'Sullivan <bos@serpentine.com>
