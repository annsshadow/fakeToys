
Minimal requirements to compile the Kernel
++++++++++++++++++++++++++++++++++++++++++

## 简

本文档旨在提供运行当前内核版本所需的最低软件级别列表
本文档最初基于我针对 2.0.x 内核所写的“Changes”文件，因此向与那份文件相同的人致谢（Jared Mauch、Axel Boldt、Alessandro Sigala，以及网络上无数其他用户）
######## 当前最低要

在认为你遇到了缺陷之前，*至少**升级到以下软件版本！如果你不确定当前运行的是哪个版本，建议的命令会告诉你。要列出系统中包含其版本的所有程序，请执./scripts/ver_linux

再次提醒，此列表假设你已经在正常运行一Linux 内核。此外，并非所有工具在所有系统上都是必需的；显然，例如如果你没有任何 PC Card 硬件，你可能不需要关pcmciautils
====================== ===============  ========================================
        程序            最低版             检查版本的命令
====================== ===============  ========================================
bash                   4.2              bash --version
bc                     1.06.95          bc --version
bindgen (可          0.71.1           bindgen --version
binutils               2.30             ld -v
bison                  2.0              bison --version
btrfs-progs            0.18             btrfs --version
Clang/LLVM (可       15.0.0           clang --version
e2fsprogs              1.41.4           e2fsck -V
flex                   2.5.35           flex --version
gdb                    7.2              gdb --version
GNU awk (可          5.1.0            gawk --version
GNU C                  8.1              gcc --version
GNU make               4.0              make --version
GNU tar                1.28             tar --version
GRUB                   0.93             grub --version || grub-install --version
gtags (可            6.6.5            gtags --version
iptables               1.4.2            iptables -V
jfsutils               1.1.3            fsck.jfs -V
kmod                   13               kmod -V
mcelog                 0.6              mcelog --version
mkimage (可          2017.01          mkimage --version
nfs-utils              1.0.5            showmount --version
openssl & libcrypto    1.0.0            openssl version
pahole                 1.22             pahole --version
pcmciautils            004              pccardctl -V
PPP                    2.4.0            pppd --version
procps                 3.2.0            ps --version
Python                 3.9.x            python3 --version
quota-tools            3.09             quota -V
Rust (可            1.85.0           rustc --version
Sphinx\ [#f1]_         3.4.3            sphinx-build --version
squashfs-tools         4.0              mksquashfs -version
udev                   081              udevadm --version
util-linux             2.10o            mount --version
xfsprogs               2.6.0            xfs_db -V
====================== ===============  ========================================


######## 内核编译


### GCC


gcc 的版本要求可能因你计算机CPU 的类型而异
### Clang/LLVM (可


clang LLVM 工具的最新正式发布版（根`releases.llvm.org <https://releases.llvm.org>`_）都受支持用于构建内核。较旧的发布版不保证可用，并且我们可能会从内核中移除用于支持旧版本的变通代码。请参阅关于使用 Clang/LLVM 构建 Linux 的额外文<kbuild_llvm>
### Rust (可


需要较新版本的 Rust 编译器
请参Documentation/rust/quick-start.rst 了解如何满足 Rust 支持的构建要求。特别是 `Makefile` 目标 `rustavailable` 对于排查 Rust 工具链未被检测到的原因很有用
### bindgen (可


`bindgen` 用于为内核的 C 侧生Rust 绑定。它依赖`libclang`
### Make


构建内核需GNU make 4.0 或更高版本
### Bash


内核构建中会使用一bash 脚本。需Bash 4.2 或更高版本
### Binutils


构建内核需Binutils 2.30 或更高版本
### pkg-config


4.18 起，构建系统需pkg-config 来检查已安装kconfig 工具，并确定用于 'make {g,x}config' 的标志设置。此pkg-config 虽被使用，但未被验证或记录
### Flex


Linux 4.16 起，构建系统在构建期间生成词法分析器。这需flex 2.5.35 或更高版本

### Bison


Linux 4.16 起，构建系统在构建期间生成解析器。这需bison 2.0 或更高版本
### pahole


Linux 5.2 起，如果选择CONFIG_DEBUG_INFO_BTF，构建系统会vmlinux 中的 DWARF 生成 BTF（BPF Type Format），稍后也会从内核模块生成。这需pahole v1.22 或更高版本
它可'dwarves' 'pahole' 发行版软件包中找到，或来https://fedorapeople.org/~acme/dwarves/
### Perl


**构建内核需perl 5 以及以下模块``Getopt**
: Long``,
**``Getopt**
: Std`、`File::Basename` `File::Find``
### Python


若干配置选项需要它：arm/arm64 的默认配置、CONFIG_LTO_CLANG、一些可选的 DRM 配置、kernel-doc 工具以及文档构建（Sphinx）等都需要它
### BC


构建 3.10 及更高版本的内核需bc

### OpenSSL


模块签名和外部证书处理使OpenSSL 程序和加密库来进行密钥创建和签名生成
如果启用了模块签名，构建 3.7 及更高版本的内核需openssl。构4.3 及更高版本的内核还需openssl 开发包
### Tar


如果想要通过 sysfs 启用对内核头文件的访问（CONFIG_IKHEADERS），则需GNU tar
### gtags / GNU GLOBAL (可


内核构建需要通过 `make gtags` 生成标签文件，这需GNU GLOBAL 6.6.5 或更高版本。这是因为它使用gtags `-C (--directory)` 标志
### mkimage


该工具在构建扁平镜像树（FIT，Flat Image Tree）时使用，常见于 ARM 平台。该工具可通过 `u-boot-tools` 软件包获取，也可U-Boot 源代码构建。请参阅 https://docs.u-boot.org/en/latest/build/tools.html#building-tools-for-linux 中的说明
### GNU AWK


如果希望内核构建为内置模块生成地址范围数据（CONFIG_BUILTIN_MODULE_RANGES），则需GNU AWK
######## 系统工具


### 架构相关变更


DevFS 已被弃用，改udev（https://www.kernel.org/pub/linux/utils/kernel/hotplug/）
32 UID 支持现已就位。尽情享用吧
内核函数的文档正逐步过渡到通过源代码中其定义附近、采用特殊格式编写的注释来进行内联文档。这些注释可以与 Documentation/ 目录中的 ReST 文件结合，生成富文档，随后可转换PostScript、HTML、LaTex、ePUB PDF 文件。为了从 ReST 格式转换为你选择的格式，你需Sphinx
### Util-linux


新版本的 util-linux 提供了对更大磁盘`fdisk` 支持、支mount 的新选项、识别更多受支持的分区类型，以及类似的好东西。你可能想要升级
### Ksymoops


如果发生了不可想象的事情，你的内核发生了 oops，你可能需ksymoops 工具来解码它，但在大多数情况下你不需要。通常更倾向于使`CONFIG_KALLSYMS` 构建内核，这样它会产生可直接使用的可读转储（这也会产生比 ksymoops 更好的输出）。如果出于某种原因你的内核未使用 `CONFIG_KALLSYMS` 构建，并且你无法重新构建并用该选项复现 Oops，那么你仍然可以ksymoops 解码Oops
### Mkinitrd


`/lib/modules` 文件树布局的这些变更也要求升级 mkinitrd
### E2fsprogs


最新版本的 `e2fsprogs` 修复fsck debugfs 中的若干缺陷。显然，升级是个好主意
### JFSutils


`jfsutils` 软件包包含该文件系统的工具。可用工具有
- `fsck.jfs` - 启动事务日志重放，并检查、修JFS 格式的分区
- `mkfs.jfs` - 创建 JFS 格式的分区
- 该软件包中还提供其他文件系统工具
### Xfsprogs


最新版本的 `xfsprogs` 包含 `mkfs.xfs`、`xfs_db` 以及 `xfs_repair` 工具等，用于 XFS 文件系统。它与架构无关，2.0.0 及之后的任何版本都应能与此版本的 XFS 内核代码正常工作（由于一些显著的改进，建议使2.6.0 或更高版本）
### PCMCIAutils


PCMCIAutils 取代`pcmcia-cs`。它在系统启动时正确设置 PCMCIA 插槽，并在内核被模块化并且使用了 hotplug 子系统时，为 16 PCMCIA 设备加载相应的模块
### Quota-tools


如果你想使用较新版本 2 的配额格式，则需要支32 uid gid。Quota-tools 3.07 及更高版本支持此功能。请使用上表中推荐或更高的版本
### Intel IA32 微码


添加了一个驱动，允许更新 Intel IA32 微码，它作为普通（misc）字符设备可访问。如果你没有使用
```
  mkdir /dev/cpu
  mknod /dev/cpu/microcode c 10 184
  chmod 0644 /dev/cpu/microcode

```

作为 root 才能使用它。你可能还想要获取用户空间的 microcode_ctl 工具来配合使用
### udev


`udev` 是一个用户空间应用程序，用于仅用实际存在的设备条目动态填`/dev`。`udev` 取代devfs 的基本功能，同时允许对设备使用持久化命名
### FUSE


需libfuse 2.4.0 或更高版本。绝对最低为 2.3.0，但挂载选项 `direct_io` `kernel_cache` 将不起作用
######## 网络


### 总体变更


如果你有高级的网络配置需求，你可能应该考虑使用 ip-route2 中的网络工具
### 包过/ NAT


包过滤和 NAT 代码使用与之2.4.x 内核系列相同的工具（iptables）。它仍然包含针对 2.2.x 风格 ipchains 2.0.x 风格 ipfwadm 的向后兼容模块
### PPP


PPP 驱动已被重构以支持多链路（multilink），并使其能够在多样化的媒体层上运行。如果你使用 PPP，请pppd 升级到至2.4.0
如果你没有使udev，则必须有设备文/dev/ppp
```
  mknod /dev/ppp c 108 0

```

作为 root
### NFS-utils


在古老的.4 及更早）内核中，nfs 服务器需要知道任何期望能够通过 NFS 访问文件的客户端。当客户端挂载文件系统时，这些信息会`mountd` 提供给内核，或者在系统启动时由 `exportfs` 提供。`exportfs` 会从 `/var/lib/nfs/rmtab` 获取关于活跃客户端的信息
这种方式相当脆弱，因为它依赖rmtab 的正确性，而这并不总是容易保证，特别是在尝试实现故障转移时。即使系统运行良好，`rmtab` 也会积累大量永远不会被删除的旧条目
在现代内核中，我们可以选择让内核在收到来自未知主机的请求时通知 mountd，mountd 可以向内核提供相应的导出信息。这消除了对 `rmtab` 的依赖，意味着内核只需要知道当前活跃的客户端
```
  mount -t nfsd nfsd /proc/fs/nfsd

```

在运exportfs mountd 之前。建议在可能的情况下，用防火墙将所NFS 服务与整个互联网隔离
### mcelog


x86 内核上，当启`CONFIG_X86_MCE` 时，需mcelog 工具来处理和记录机器检查（machine check）事件。机器检查事件是CPU 报告的错误。强烈建议对其进行处理
######## 内核文档


### Sphinx


有关 Sphinx 要求的详细信息，请参Documentation/doc-guide/sphinx.rst 中的 sphinx_install
### rustdoc


`rustdoc` 用于生成 Rust 代码的文档。更多信息请参阅 Documentation/rust/general-information.rst
## 获取更新的软

######## 内核编译


### gcc


- <ftp://ftp.gnu.org/gnu/gcc/>

### Clang/LLVM


- Getting LLVM <getting_llvm>銆。
### Rust


- Documentation/rust/quick-start.rst銆。
### bindgen


- Documentation/rust/quick-start.rst銆。
### Make


- <ftp://ftp.gnu.org/gnu/make/>

### Bash


- <ftp://ftp.gnu.org/gnu/bash/>

### Binutils


- <https://www.kernel.org/pub/linux/devel/binutils/>

### Flex


- <https://github.com/westes/flex/releases>

### Bison


- <ftp://ftp.gnu.org/gnu/bison/>

### OpenSSL


- <https://www.openssl.org/>

######## 系统工具


### Util-linux


- <https://www.kernel.org/pub/linux/utils/util-linux/>

### Kmod


- <https://www.kernel.org/pub/linux/utils/kernel/kmod/>
- <https://git.kernel.org/pub/scm/utils/kernel/kmod/kmod.git>

### Ksymoops


- <https://www.kernel.org/pub/linux/utils/kernel/ksymoops/v2.4/>

### Mkinitrd


- <https://code.launchpad.net/initrd-tools/main>

### E2fsprogs


- <https://www.kernel.org/pub/linux/kernel/people/tytso/e2fsprogs/>
- <https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git/>

### JFSutils


- <https://jfs.sourceforge.net/>

### Xfsprogs


- <https://git.kernel.org/pub/scm/fs/xfs/xfsprogs-dev.git>
- <https://www.kernel.org/pub/linux/utils/fs/xfs/xfsprogs/>

### Pcmciautils


- <https://www.kernel.org/pub/linux/utils/kernel/pcmcia/>

### Quota-tools


- <https://sourceforge.net/projects/linuxquota/>


### Intel P6 microcode


- <https://downloadcenter.intel.com/>

### udev


- <https://www.freedesktop.org/software/systemd/man/udev.html>

### FUSE


- <https://github.com/libfuse/libfuse/releases>

### mcelog


- <https://www.mcelog.org/>

######## 网络


### PPP


- <https://download.samba.org/pub/ppp/>
- <https://git.ozlabs.org/?p=ppp.git>
- <https://github.com/paulusmack/ppp/>

### NFS-utils


- <https://sourceforge.net/project/showfiles.php?group_id=14>
- <https://nfs.sourceforge.net/>

### Iptables


- <https://netfilter.org/projects/iptables/index.html>

### Ip-route2


- <https://www.kernel.org/pub/linux/utils/net/iproute2/>

### OProfile


- <https://oprofile.sf.net/download/>

######## 内核文档


### Sphinx


- <https://www.sphinx-doc.org/>
