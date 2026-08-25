## Kbuild


## 输出文件


### modules.order

该文件记录了模块Makefile 中出现的顺序。modprobe 利用它来确定性地解析与多个模块匹配的别名
### modules.builtin

该文件列出了所有内建到内核中的模块。modprobe 借此在尝试加载某个内建模块时不会失败
### modules.builtin.modinfo

该文件包含内建到内核中的所有模块的 modinfo。与独立模块modinfo 不同，所有字段都以模块名作为前缀
### modules.builtin.ranges

该文件包含内建到内核中的所有模块（ELF 段）的地址偏移范围。结System.map，可用于将模块名与符号关联起来
## 环境变量


### KCPPFLAGS

传递给预处理阶段的额外选项。这些预处理选项会在 kbuild 进行预处理的所有场景中使用，包括编C 文件和汇编文件
### KAFLAGS

传递给汇编器的额外选项（用于内建代码和模块）
### AFLAGS_MODULE

模块专用的额外汇编器选项
### AFLAGS_KERNEL

内建代码专用的额外汇编器选项
### KCFLAGS

传递给 C 编译器的额外选项（用于内建代码和模块）
### KRUSTFLAGS

传递给 Rust 编译器的额外选项（用于内建代码和模块）
### CFLAGS_KERNEL

当使$(CC) 编译作为内建的代码时，为其提供的额外选项
### CFLAGS_MODULE

使用 $(CC) 时采用的、模块专用的额外选项
### RUSTFLAGS_KERNEL

当使$(RUSTC) 编译作为内建的代码时，为其提供的额外选项
### RUSTFLAGS_MODULE

使用 $(RUSTC) 时采用的、模块专用的额外选项
### LDFLAGS_MODULE

使用 $(LD) 链接模块时使用的额外选项
### HOSTCFLAGS

构建宿主程序时传递给 $(HOSTCC) 的额外标志
### HOSTCXXFLAGS

构建宿主程序时传递给 $(HOSTCXX) 的额外标志
### HOSTRUSTFLAGS

构建宿主程序时传递给 $(HOSTRUSTC) 的额外标志
### PROCMACROLDFLAGS

链接 Rust 过程宏时传递的标志。由于过程宏在构建时rustc 加载，它们必须以与所rustc 工具链兼容的方式链接
例如，当 rustc 使用C 库与用户希望用于宿主程序C 库不同时，这会很有用
若未设置，则默认使用链接宿主程序时传递的标志
### HOSTLDFLAGS

链接宿主程序时传递的额外标志
### HOSTLDLIBS

构建宿主程序时需链接的额外库

### USERCFLAGS

编译 userprogs 时用$(CC) 的额外选项
### USERLDFLAGS

链接 userprogs 时用$(LD) 的额外选项。userprogs 使用 CC 进行链接，因$(USERLDFLAGS) 应包含适用"-Wl," 前缀
### KBUILD_KCONFIG

将该环境变量的值设为顶Kconfig 文件。默认名称为 "Kconfig"
### KBUILD_VERBOSE

设置 kbuild 的详细程度。可赋予"V=..." 相同的值
完整列表参见 make help
设置 "V=..." 的优先级高于 KBUILD_VERBOSE
### KBUILD_EXTMOD

设置构建外部模块时查找内核源码的目录
设置 "M=..." 的优先级高于 KBUILD_EXTMOD
### KBUILD_OUTPUT

构建内核时指定输出目录
该变量也可用于在独立构建目录中针对预构建的内核构建外部模块时，指向内核输出目录。请注意，这并不指定外部模块自身的输出目录（为此请使KBUILD_EXTMOD_OUTPUT）
输出目录也可使用 "O=..." 指定
设置 "O=..." 的优先级高于 KBUILD_OUTPUT
### KBUILD_EXTMOD_OUTPUT

指定外部模块的输出目录
设置 "MO=..." 的优先级高于 KBUILD_EXTMOD_OUTPUT
### KBUILD_EXTRA_WARN

指定额外的构建检查。通过命令行传W=... 可赋予相同的值
支持的值列表参`make help`
设置 "W=..." 的优先级高于 KBUILD_EXTRA_WARN
### KBUILD_DEBARCH

对于 deb-pkg 目标，允许覆deb-pkg 所采用的常规启发式判断。通常 deb-pkg 会基UTS_MACHINE 变量，在某些架构上还会基于内核配置，来猜测正确的架构。KBUILD_DEBARCH 的值被假定（而非检查）为一个有效的 Debian 架构
### KDOCFLAGS

为构建过程中kernel-doc 检查指定额外的（警错误）标志，支持哪些标志参见 tools/docs/kernel-doc。注意这（目前）不适用于文档构建
### ARCH

ARCH 设为要构建的架构
在大多数情况下，架构名称arch/ 目录中的目录名相同
但某些架构（x86 sparc）有别名
- x862 位为 i3864 位为 x86_64
- parisc4 位为 parisc64
- sparc2 位为 sparc324 位为 sparc64

### CROSS_COMPILE

指定 binutils 文件名的固定部分（可选）。CROSS_COMPILE 可以是文件名的一部分，也可以是完整路径
在某些配置中，CROSS_COMPILE 也用ccache
### CF

sparse 的额外选项
```

    make CF=-Wbitwise C=2

```
### INSTALL_PATH

INSTALL_PATH 指定放置更新后的内核与系统映射镜像的位置。默认为 /boot，但也可设为其他值
### INSTALLKERNEL

使用 "make install" 时调用的安装脚本。默认名称为 "installkernel"
该脚本将使用以下参数调用
   - $1 - 内核版本
   - $2 - 内核镜像文件
   - $3 - 内核映射文件
   - $4 - 默认安装路径（若为空则使用根目录
"make install" 的实现是架构相关的，可能与上述不同
提供 INSTALLKERNEL 是为了能够在交叉编译内核时指定自定义安装器
### MODLIB

指定模块的安装位置```

     $(INSTALL_MOD_PATH)/lib/modules/$(KERNELRELEASE)

```
该值可被覆盖，此时默认值被忽略
### INSTALL_MOD_PATH

INSTALL_MOD_PATH MODLIB 指定一个前缀，用build root 所需的模块目录重定位。它makefile 中未定义，但如有需要可将此参数传递给 make
### INSTALL_MOD_STRIP

若定义了 INSTALL_MOD_STRIP，会导致模块在安装后strip。若 INSTALL_MOD_STRIP '1'，则使用默认选项 --strip-debug。否则，INSTALL_MOD_STRIP 的值将作为传递给 strip 命令的选项
### INSTALL_HDR_PATH

INSTALL_HDR_PATH 指定执行 "make headers_*" 时用户空间头文件的安装位置
```

    $(objtree)/usr

```
$(objtree) 是保存输出文件的目录
输出目录通常通过命令行上"O=..." 设置
该值可被覆盖，此时默认值被忽略
### INSTALL_DTBS_PATH

INSTALL_DTBS_PATH 指定设备blob 的安装位置，用于 build root 所需的重定位。它makefile 中未定义，但如有需要可将此参数传递给 make
### KBUILD_ABS_SRCTREE

在可能的情况下，Kbuild 使用相对路径来指向源码树。例如，在源码树中构建时，源码树路径'.'
设置此标志会要求 Kbuild 使用源码树的绝对路径。这在某些场景下很有用，例如生成带有绝对路径条目tag 文件等
### KBUILD_SIGN_PIN

当对内核模块签名且私钥需要口令或 PIN 时，该变量允许将口令PIN 传递给 sign-file 工具
### KBUILD_MODPOST_WARN

KBUILD_MODPOST_WARN 可设置为避免在最终模块链接阶段出现未定义符号时报错。它会将这些错误转为警告
### KBUILD_MODPOST_NOFINAL

KBUILD_MODPOST_NOFINAL 可设置为跳过模块的最终链接。这仅用于加速测试编译
### KBUILD_EXTRA_SYMBOLS

用于使用来自其他模块的符号的模块。更多细节参modules.rst
### ALLSOURCE_ARCHS

对于 tags/TAGS/cscope 目标，可以指定多个架```

    $ make ALLSOURCE_ARCHS="x86 mips arm" tags

```
```

    $ make ALLSOURCE_ARCHS=all tags

```
### IGNORE_DIRS

对于 tags/TAGS/cscope 目标，可以选择排除哪些目录
```

    $ make IGNORE_DIRS="drivers/gpu/drm/radeon tools" cscope

```
### KBUILD_BUILD_TIMESTAMP

将其设置为日期字符串，会覆盖 UTS_VERSION 定义中使用的（运行内核中 uname -v 所在的）时间戳。该值必```

    $ KBUILD_BUILD_TIMESTAMP="Mon Oct 13 00:00:00 UTC 2025" make

```
默认值为构建过程中某个时date 命令的输出。如果提供了该时间戳，它也将用于任何 initramfs 归档中的 mtime 字段。Initramfs mtime 32 位的，因1970 Unix 纪元之前2106-02-07 06:28:15 UTC 之后的日期会失败
### KBUILD_BUILD_USER, KBUILD_BUILD_HOST

这两个变量允许覆盖启动期间和 /proc/version 中显示的 user@host 字符串。默认值分别为 whoami host 命令的输出
### LLVM

如果将该变量设为 1，Kbuild 将使Clang LLVM 工具链（而非 GCC GNU binutils）来构建内核