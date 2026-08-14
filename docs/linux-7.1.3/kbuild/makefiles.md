## Linux 内核 Makefiles


本文档描述了 Linux 内核的 Makefiles。

## Overview


```

	Makefile                    the top Makefile.
	.config                     the kernel configuration file.
	arch/$(SRCARCH)/Makefile    the arch Makefile.
	scripts/Makefile.*          common rules etc. for all kbuild Makefiles.
	kbuild Makefiles            exist in every subdirectory

```
顶层 Makefile 读取来自内核配置过程生成的 .config 文件。

顶层 Makefile 负责构建两个主要产物：vmlinux（常驻内核映像）与 modules（任意模块文件）。它通过递归进入内核源码树中的子目录来构建这些目标。

被访问的子目录列表取决于内核配置。顶层 Makefile 以文本方式包含一个名为 arch/$(SRCARCH)/Makefile 的架构 Makefile。该架构 Makefile 向顶层 Makefile 提供架构相关的信息。

每个子目录都有一个 kbuild Makefile，用于执行从上层传递下来的命令。kbuild Makefile 使用来自 .config 文件的信息，构造 kbuild 用于构建任意内置或模块化目标所需的各种文件列表。

scripts/Makefile.* 包含了基于 kbuild makefiles 构建内核所用的全部定义/规则等。

## Who does what


人们与内核 Makefiles 之间存在四种不同的关系。

**Users（用户）** 是构建内核的人。这些人输入诸如 `make menuconfig` 或 `make` 这样的命令。他们通常既不阅读也不编辑任何内核 Makefile（或任何其他源文件）。

**Normal developers（普通开发者）** 是从事设备驱动、文件系统和网络协议等特性开发的人。这些人需要维护其所工作子系统的 kbuild Makefile。为了高效地完成这项工作，他们需要一些关于内核 Makefiles 的整体知识，以及对 kbuild 公共接口的详细了解。

**Arch developers（架构开发者）** 是从事整个架构（如 sparc 或 x86）开发的人。架构开发者需要了解架构 Makefile 以及 kbuild Makefile。

**Kbuild developers（kbuild 开发者）** 是从事内核构建系统本身开发的人。这些人需要了解内核 Makefiles 的方方面面。

本文档面向普通开发者和架构开发者。


## The kbuild files


内核中的大多数 Makefile 都是使用 kbuild 基础设施的 kbuild Makefile。本章介绍 kbuild makefiles 中所使用的语法。

kbuild 文件的偏好名称是 `Makefile`，但也可以使用 `Kbuild`；若 `Makefile` 与 `Kbuild` 文件同时存在，则会使用 `Kbuild` 文件。

第 `Goal definitions`_ 节是一个快速入门；后续章节通过真实示例提供了更多细节。

### Goal definitions


目标定义是 kbuild Makefile 的主要部分（核心）。这些行定义了要构建的文件、任何特殊的编译选项，以及任何需要递归进入的子目录。

最简单的 kbuild makefile 只包含一行：

```

  obj-y += foo.o

```
这告诉 kbuild 该目录中存在一个名为 foo.o 的目标文件。foo.o 将由 foo.c 或 foo.S 构建。

如果 foo.o 要作为模块构建，则使用变量 obj-m。因此经常使用如下模式：

```

  obj-$(CONFIG_FOO) += foo.o

```
$(CONFIG_FOO) 求值为 y（内置）或 m（模块）。如果 CONFIG_FOO 既不是 y 也不是 m，那么该文件既不会被编译也不会被链接。

### Built-in object goals - obj-y


kbuild Makefile 在 $(obj-y) 列表中为 vmlinux 指定目标文件。这些列表取决于内核配置。

Kbuild 编译所有的 $(obj-y) 文件。然后它调用 `$(AR) rcSTP` 将这些文件合并为一个 built-in.a 文件。这是一个没有符号表的精简归档。它随后会被 scripts/link-vmlinux.sh 链接进 vmlinux。

$(obj-y) 中的文件顺序是有意义的。列表中允许重复：第一个实例会被链接进 built-in.a，后续实例将被忽略。

链接顺序是有意义的，因为某些函数（module_init() / __initcall）会按它们出现的顺序在启动期间被调用。因此请记住，改变链接顺序可能会改变 SCSI 控制器被检测的顺序，从而改变你的磁盘编号。

```

  #drivers/isdn/i4l/Makefile
  # Makefile for the kernel ISDN subsystem and device drivers.
  # Each configuration option enables a list of files.
  obj-$(CONFIG_ISDN_I4L)         += isdn.o
  obj-$(CONFIG_ISDN_PPP_BSDCOMP) += isdn_bsdcomp.o

```
### Loadable module goals - obj-m


$(obj-m) 指定作为可加载内核模块构建的目标文件。

一个模块可以由一个源文件或若干源文件构建。对于单个源文件的情况，kbuild makefile 只需将该文件加入 $(obj-m)。

```

  #drivers/isdn/i4l/Makefile
  obj-$(CONFIG_ISDN_PPP_BSDCOMP) += isdn_bsdcomp.o

```
注意：在本例中 $(CONFIG_ISDN_PPP_BSDCOMP) 求值为 "m"。

如果一个内核模块由多个源文件构建，你以与上述相同的方式指定要构建一个模块；然而，kbuild 需要知道你想用哪些目标文件来构建你的模块，因此你必须通过设置 $(<module_name>-y) 变量来告知它。

```

  #drivers/isdn/i4l/Makefile
  obj-$(CONFIG_ISDN_I4L) += isdn.o
  isdn-y := isdn_net_lib.o isdn_v110.o isdn_common.o

```
在本例中，模块名将为 isdn.o。Kbuild 将编译 $(isdn-y) 中列出的目标文件，然后对这些文件列表运行 `$(LD) -r` 来生成 isdn.o。

由于 kbuild 会识别用于复合目标的 $(<module_name>-y)，你可以使用 `CONFIG_` 符号的值有选择地将一个目标文件作为复合目标的一部分包含进来。

```

  #fs/ext2/Makefile
  obj-$(CONFIG_EXT2_FS) += ext2.o
  ext2-y := balloc.o dir.o file.o ialloc.o inode.o ioctl.o \
    namei.o super.o symlink.o
  ext2-$(CONFIG_EXT2_FS_XATTR) += xattr.o xattr_user.o \
    xattr_trusted.o

```
在本例中，只有当 $(CONFIG_EXT2_FS_XATTR) 求值为 "y" 时，xattr.o、xattr_user.o 和 xattr_trusted.o 才是复合目标 ext2.o 的一部分。

注意：当然，当你将目标构建进内核时，上述语法同样适用。因此，如果你设置了 CONFIG_EXT2_FS=y，kbuild 会像你预期的那样，从各个部分构建出一个 ext2.o 文件，然后将其链接进 built-in.a。

### Library file goals - lib-y


用 obj-* 列出的目标用于模块，或被合并进该特定目录的 built-in.a。还有一种可能，即列出将被包含进库 lib.a 的目标。所有用 lib-y 列出的目标都会被合并进该目录的单个库中。同时列在 obj-y 和 lib-y 中的目标不会被包含进库，因为它们无论如何都可被访问到。为保持一致，列在 lib-m 中的目标会被包含进 lib.a。

注意，同一个 kbuild makefile 可能既列出要内置的目标，也列出要作为库一部分的目标。因此同一个目录中可能同时包含 built-in.a 和 lib.a 文件。

```

  #arch/x86/lib/Makefile
  lib-y    := delay.o

```
这将基于 delay.o 创建一个库 lib.a。为了让 kbuild 真正识别出正在构建一个 lib.a，该目录必须列在 libs-y 中。

另见 `List directories to visit when descending`_。

lib-y 的使用通常限制在 `lib/` 和 `arch/*/lib`。

### Descending down in directories


Makefile 只负责构建其自身目录中的目标。子目录中的文件应由这些子目录中的 Makefile 负责。只要你让构建系统知道这些子目录，它会自动在子目录中递归调用 make。

为此，使用 obj-y 和 obj-m。ext2 位于一个独立的目录中，fs/ 中的 Makefile 通过以下赋值告诉 kbuild 进行递归下降。

```

  #fs/Makefile
  obj-$(CONFIG_EXT2_FS) += ext2/

```
如果 CONFIG_EXT2_FS 被设置为 "y"（内置）或 "m"（模块化），相应的 obj- 变量将被设置，kbuild 将递归下降进入 ext2 目录。

Kbuild 利用这些信息不仅决定是否需要访问该目录，还决定是否需要将该目录中的目标链接进 vmlinux。

当 Kbuild 以 "y" 下降进入目录时，该目录中的所有内置目标会被合并进 built-in.a，并最终被链接进 vmlinux。

相反，当 Kbuild 以 "m" 下降进入目录时，该目录中没有任何内容会被链接进 vmlinux。如果该目录中的 Makefile 指定了 obj-y，那些目标将被遗留为孤儿。这很可能是 Makefile 或 Kconfig 中依赖项的 bug。

Kbuild 还支持专用语法 subdir-y 和 subdir-m 用于下降到子目录。当你明确知道它们根本不包含内核空间目标时，它很合适。一个典型用途是让 Kbuild 下降进入子目录来构建工具。

```

  # scripts/Makefile
  subdir-$(CONFIG_GCC_PLUGINS) += gcc-plugins
  subdir-$(CONFIG_MODVERSIONS) += genksyms
  subdir-$(CONFIG_SECURITY_SELINUX) += selinux

```
与 obj-y/m 不同，subdir-y/m 不需要尾部斜杠，因为此语法始终用于目录。

在赋值时使用 `CONFIG_` 变量是良好实践。这样，如果相应的 `CONFIG_` 选项既不是 "y" 也不是 "m"，kbuild 可以完全跳过该目录。

### Non-builtin vmlinux targets - extra-y


extra-y 指定构建 vmlinux 所需、但未被合并进 built-in.a 的目标。

示例包括：

1) vmlinux 链接脚本

   vmlinux 的链接脚本位于 arch/$(SRCARCH)/kernel/vmlinux.lds

```

  # arch/x86/kernel/Makefile
  extra-y	+= vmlinux.lds

```
extra-y 现在已被弃用，因为它等价于：

  always-$(KBUILD_BUILTIN) += vmlinux.lds

$(extra-y) 应只包含在构建 vmlinux 时所需的目标。

当 vmlinux 显然不是一个最终目标时，Kbuild 会跳过 extra-y。（例如 `make modules`，或构建外部模块）

如果你打算无条件地构建目标，always-y（下一节解释）才是正确的语法。

### Always built goals - always-y


always-y 指定在 Kbuild 访问该 Makefile 时字面上总是被构建的目标。

```

  # ./Kbuild
  offsets-file := include/generated/asm-offsets.h
  always-y += $(offsets-file)

```
### Compilation flags


ccflags-y, asflags-y and ldflags-y
  这三个标志仅应用于它们被赋值的 kbuild makefile。它们用于递归构建期间所有正常的 cc、as 和 ld 调用。

  ccflags-y 指定使用 $(CC) 编译时的选项。

```

    # drivers/acpi/acpica/Makefile
    ccflags-y				:= -Os -D_LINUX -DBUILDING_ACPICA
    ccflags-$(CONFIG_ACPI_DEBUG)	+= -DACPI_DEBUG_OUTPUT

```
  该变量是必要的，因为顶层 Makefile 拥有变量 $(KBUILD_CFLAGS) 并将其用于整个源码树的编译标志。

  asflags-y 指定汇编器选项。

  Example::

```

    #arch/sparc/kernel/Makefile
    asflags-y := -ansi

```
  ldflags-y 指定使用 $(LD) 链接时的选项。

  Example::

```

    #arch/cris/boot/compressed/Makefile
    ldflags-y += -T $(src)/decompress_$(arch-y).lds

```
subdir-ccflags-y, subdir-asflags-y
  上面列出的两个标志类似于 ccflags-y 和 asflags-y。不同之处在于 subdir- 变体对它们所在的 kbuild 文件以及所有子目录都生效。使用 subdir-* 指定的选项会被加在非 subdir 变体指定的选项之前。

```

    subdir-ccflags-y := -Werror

```
ccflags-remove-y, asflags-remove-y
  这些标志用于移除编译器、汇编器调用中的特定标志。

```

    ccflags-remove-$(CONFIG_MCOUNT) += -pg

```
CFLAGS_$@, AFLAGS_$@
  CFLAGS_$@ 和 AFLAGS_$@ 仅应用于当前 kbuild makefile 中的命令。

  $(CFLAGS_$@) 为 $(CC) 指定每文件选项。$@ 部分是一个字面量值，指定它所针对的文件。

  CFLAGS_$@ 的优先级高于 ccflags-remove-y；CFLAGS_$@ 可以重新添加被 ccflags-remove-y 移除的编译器标志。

```

    # drivers/scsi/Makefile
    CFLAGS_aha152x.o =   -DAHA152X_STAT -DAUTOCONF

```
  此行指定了 aha152x.o 的编译标志。

  $(AFLAGS_$@) 是针对汇编语言源文件的类似特性。

  AFLAGS_$@ 的优先级高于 asflags-remove-y；AFLAGS_$@ 可以重新添加被 asflags-remove-y 移除的汇编器标志。

  Example::

```

    # arch/arm/kernel/Makefile
    AFLAGS_head.o        := -DTEXT_OFFSET=$(TEXT_OFFSET)
    AFLAGS_crunch-bits.o := -Wa,-mcpu=ep9312
    AFLAGS_iwmmxt.o      := -Wa,-mcpu=iwmmxt

```
### Dependency tracking


Kbuild 跟踪以下方面的依赖：

1) 所有先决条件文件（包括 `**.c` 和 `**.h`）
2) 所有先决条件文件中使用的 `CONFIG_` 选项
3) 用于编译目标的命令行

因此，如果你更改了 $(CC) 的某个选项，所有受影响的文件都将被重新编译。

### Custom Rules


当 kbuild 基础设施没有提供所需支持时使用自定义规则。一个典型示例是构建过程中生成的头文件。另一个示例是需要自定义规则来准备启动映像等的架构相关 Makefile。

自定义规则像普通 Make 规则一样编写。Kbuild 并不是在 Makefile 所在目录中执行，因此所有自定义规则应使用相对路径引用先决条件文件和目标文件。

定义自定义规则时使用两个变量：

$(src)
  $(src) 是 Makefile 所在目录。引用位于源码树中的文件时，应始终使用 $(src)。

$(obj)
  $(obj) 是保存目标的目录。引用生成的文件时，应始终使用 $(obj)。对于需要同时适用于生成文件和真实源文件的模式规则，使用 $(obj)（VPATH 不仅会在对象树中，也会在源码树中帮助查找先决条件）。

```

    #drivers/scsi/Makefile
    $(obj)/53c8xx_d.h: $(src)/53c7,8xx.scr $(src)/script_asm.pl
    $(CPP) -DCHIP=810 - < $< | ... $(src)/script_asm.pl

```
  这是一个自定义规则，遵循 make 所需的正常语法。

  目标文件依赖于两个先决条件文件。对目标文件的引用以 $(obj) 为前缀，对先决条件的引用以 $(src) 为前缀（因为它们不是生成的文件）。


$(srcroot)
  $(srcroot) 指你正在构建的源码根目录，它可以是内核源码，也可以是外部模块源码，取决于是否设置了 KBUILD_EXTMOD。它可以是相对路径或绝对路径，但如果设置了 KBUILD_ABS_SRCTREE=1，它始终是绝对路径。

$(srctree)
  $(srctree) 指内核源码树的根目录。构建内核时，它与 $(srcroot) 相同。

$(objtree)
  $(objtree) 指内核对象树的根目录。构建内核时它是 `.`，但构建外部模块时则不同。

$(kecho)
  在规则中向用户回显信息通常是一种良好实践，但在执行 `make -s` 时，除了警告/错误之外，不应期望看到任何输出。为了支持这一点，kbuild 定义了 $(kecho)，它会将 $(kecho) 后面的文本回显到 stdout，除非使用了 `make -s`。

```

    # arch/arm/Makefile
    $(BOOT_TARGETS): vmlinux
            $(Q)$(MAKE) $(build)=$(boot) MACHINE=$(MACHINE) $(boot)/$@
            @$(kecho) '  Kernel: $(boot)/$@ is ready'

```
  当 kbuild 在 KBUILD_VERBOSE 未设置的情况下执行时，通常只显示命令的简写形式。为了让自定义命令也具备这种行为，kbuild 要求设置两个变量：

    quiet_cmd_<command> - 应回显的内容
          cmd_<command> - 要执行的命令

  Example::

```

    # lib/Makefile
    quiet_cmd_crc32 = GEN     $@
          cmd_crc32 = $< > $@

    $(obj)/crc32table.h: $(obj)/gen_crc32table
            $(call cmd,crc32)

```
  更新 $(obj)/crc32table.h 目标时，以下行：

```

    GEN     lib/crc32table.h

```
  会随 ``make KBUILD_VERBOSE=`` 一起显示。

### Command change detection


当规则被求值时，会比较目标与其先决条件文件之间的时间戳。GNU Make 会在任一先决条件比目标更新时更新目标。

当命令行自上次调用以来发生变化时，目标也应被重新构建。Make 本身不支持这一点，因此 Kbuild 通过一种元编程来实现。

```

  quiet_cmd_<command> = ...
        cmd_<command> = ...

  <target>: <source(s)> FORCE
          $(call if_changed,<command>)

```
任何使用 if_changed 的目标必须列在 $(targets) 中，否则命令行检查将失败，该目标将总是被构建。

如果目标已经列在已知的语法中，如 obj-y/m、lib-y/m、extra-y/m、always-y/m、hostprogs、userprogs，Kbuild 会自动将其加入 $(targets)。否则，目标必须被显式加入 $(targets)。

对 $(targets) 的赋值不带 $(obj)/ 前缀。if_changed 可与 `Custom Rules`_ 中定义的自定义规则结合使用。

注意：忘记 FORCE 先决条件是一个典型错误。另一个常见的陷阱是空白有时是有意义的；对于

```

  target: source(s) FORCE

```
**WRONG!**	$(call if_changed, objcopy)

注意：
  if_changed 不应在同一目标上使用超过一次。它会将执行的命令存储在相应的 .cmd 文件中，多次调用会导致覆盖，并在目标是最新的、且只有命令变化的测试触发命令执行时产生不期望的结果。

### $(CC) support functions


内核可能使用多个不同版本的 $(CC) 构建，每个版本支持一组独特的特性和选项。kbuild 提供基本支持来检查 $(CC) 的有效选项。$(CC) 通常是 gcc 编译器，但也有其他替代方案可用。

as-option
  as-option 用于检查 $(CC) —— 当用于编译汇编器（`*.S`）文件时 —— 是否支持给定选项。如果第一个选项不被支持，可以指定可选的第二个选项。

```

    #arch/sh/Makefile
    cflags-y += $(call as-option,-Wa$(comma)-isa=$(isa-y),)

```
  在上面的示例中，如果 $(CC) 支持该选项，cflags-y 将被赋值为 -Wa$(comma)-isa=$(isa-y)。第二个参数是可选的，如果提供，将在第一个参数不被支持时使用。

as-instr
  as-instr 检查汇编器是否报告特定指令，然后输出 option1 或 option2。测试指令中支持 C 转义。注意：as-instr-option 使用 KBUILD_AFLAGS 作为汇编器选项。

cc-option
  cc-option 用于检查 $(CC) 是否支持给定选项，若不支持则使用可选的第二个选项。

```

    #arch/x86/Makefile
    cflags-y += $(call cc-option,-march=pentium-mmx,-march=i586)

```
  在上面的示例中，如果 $(CC) 支持该选项，cflags-y 将被赋值为 -march=pentium-mmx，否则为 -march=i586。cc-option 的第二个参数是可选的，如果省略，且第一个选项不被支持，cflags-y 将被赋为空值。注意：cc-option 使用 KBUILD_CFLAGS 作为 $(CC) 的选项。

cc-option-yn
  cc-option-yn 用于检查 $(CC) 是否支持给定选项，若支持则返回 "y"，否则返回 "n"。

```

    #arch/ppc/Makefile
    biarch := $(call cc-option-yn, -m32)
    aflags-$(biarch) += -a32
    cflags-$(biarch) += -m32

```
  在上面的示例中，如果 $(CC) 支持 -m32 选项，则 $(biarch) 被设为 y。当 $(biarch) 等于 "y" 时，展开后的变量 $(aflags-y) 和 $(cflags-y) 将分别被赋值为 -a32 和 -m32。

  注意：cc-option-yn 使用 KBUILD_CFLAGS 作为 $(CC) 的选项。

cc-disable-warning
  cc-disable-warning 检查 $(CC) 是否支持给定的警告，并返回用于禁用它的命令行开关。这个特殊函数是必需的，因为 gcc 4.4 及以后版本接受任何未知的 -Wno-* 选项，并仅当源文件中有其他警告时才对其发出警告。

```

    KBUILD_CFLAGS += $(call cc-disable-warning, unused-but-set-variable)

```
  在上面的示例中，-Wno-unused-but-set-variable 只会在 $(CC) 真正接受它时被加入 KBUILD_CFLAGS。

gcc-min-version
  gcc-min-version 测试 $(CONFIG_GCC_VERSION) 的值是否大于或等于所给值，若是则求值为 y。

```

    cflags-$(call gcc-min-version, 110100) := -foo

```
  在此示例中，如果 $(CC) 是 gcc 且 $(CONFIG_GCC_VERSION) >= 11.1，cflags-y 将被赋值为 -foo。

clang-min-version
  clang-min-version 测试 $(CONFIG_CLANG_VERSION) 的值是否大于或等于所给值，若是则求值为 y。

```

    cflags-$(call clang-min-version, 110000) := -foo

```
  在此示例中，如果 $(CC) 是 clang 且 $(CONFIG_CLANG_VERSION) >= 11.0.0，cflags-y 将被赋值为 -foo。

cc-cross-prefix
  cc-cross-prefix 用于检查 PATH 中是否存在带有所列前缀之一的 $(CC)。返回找到 prefix$(CC) 的第一个前缀 —— 如果未找到任何 prefix$(CC)，则返回空。

  在 cc-cross-prefix 的调用中，额外的前缀用单个空格分隔。

  这个功能对于试图将 CROSS_COMPILE 设置为已知值、但可能有多个值可供选择的架构 Makefile 很有用。

  仅建议在交叉构建（主机架构与目标架构不同）时尝试设置 CROSS_COMPILE。如果 CROSS_COMPILE 已被设置，则保留其旧值。

```

    #arch/m68k/Makefile
    ifneq ($(SUBARCH),$(ARCH))
            ifeq ($(CROSS_COMPILE),)
                    CROSS_COMPILE := $(call cc-cross-prefix, m68k-linux-gnu-)
            endif
    endif

```
### $(RUSTC) support functions


rustc-min-version
  rustc-min-version 测试 $(CONFIG_RUSTC_VERSION) 的值是否大于或等于所给值，若是则求值为 y。

```

    rustflags-$(call rustc-min-version, 108500) := -Cfoo

```
  在此示例中，如果 $(CONFIG_RUSTC_VERSION) >= 1.85.0，rustflags-y 将被赋值为 -Cfoo。

### $(LD) support functions


ld-option
  ld-option 用于检查 $(LD) 是否支持所提供的选项。ld-option 以两个选项作为参数。

  第二个参数是可选的，当 $(LD) 不支持第一个选项时可使用它。

```

    #Makefile
    LDFLAGS_vmlinux += $(call ld-option, -X)

```
### Script invocation


Make 规则可以调用脚本来构建内核。规则应始终提供适当的解释器来执行脚本。它们不应依赖执行位被设置，也不应直接调用脚本。为便于手动调用脚本（例如调用 ./scripts/checkpatch.pl），仍建议为脚本设置执行位。

Kbuild 提供变量 $(CONFIG_SHELL)、$(AWK)、$(PERL) 和 $(PYTHON3) 来引用相应脚本的解释器。

```

  #Makefile
  cmd_depmod = $(CONFIG_SHELL) $(srctree)/scripts/depmod.sh $(DEPMOD) \
          $(KERNELRELEASE)

```
## Host Program support


Kbuild 支持在主机上构建用于编译阶段的可执行文件。

使用主机可执行文件需要两步。

第一步是告诉 kbuild 存在一个主机程序。这是通过变量 `hostprogs` 来完成的。

第二步是向该可执行文件添加显式依赖。这可以通过两种方式完成：在规则中添加依赖，或使用变量 `always-y`。以下将描述这两种可能。

### Simple Host Program


在某些情况下，需要在运行构建的计算机上编译并运行一个程序。

以下行告诉 kbuild，程序 bin2hex 应在构建主机上构建。

```

  hostprogs := bin2hex

```
Kbuild 在上面的示例中假设 bin2hex 由位于与 Makefile 相同目录中的单个 C 源文件 bin2hex.c 构成。

### Composite Host Programs


主机程序可以基于复合目标构成。用于定义主机程序复合目标的语法与用于内核目标的语法类似。$(<executable>-objs) 列出用于链接最终可执行文件的所有目标。

```

  #scripts/lxdialog/Makefile
  hostprogs     := lxdialog
  lxdialog-objs := checklist.o lxdialog.o

```
扩展名为 .o 的目标由相应的 .c 文件编译而来。在上面的示例中，checklist.c 被编译为 checklist.o，lxdialog.c 被编译为 lxdialog.o。

最后，这两个 .o 文件被链接到可执行文件 lxdialog。注意：语法 <executable>-y 不允许用于主机程序。

### Using C++ for host programs


kbuild 提供对用 C++ 编写的主机程序的支持。这仅是为支持 kconfig 而引入的，不建议普遍使用。

```

  #scripts/kconfig/Makefile
  hostprogs     := qconf
  qconf-cxxobjs := qconf.o

```
在上面的示例中，可执行文件由 C++ 文件 qconf.cc 组成 —— 由 $(qconf-cxxobjs) 标识。

如果 qconf 由 .c 和 .cc 文件的混合组成，则可以使用额外的一行来标识这一点。

```

  #scripts/kconfig/Makefile
  hostprogs     := qconf
  qconf-cxxobjs := qconf.o
  qconf-objs    := check.o

```
### Using Rust for host programs


Kbuild 提供对用 Rust 编写的主机程序的支持。然而，由于 Rust 工具链并非内核编译所必需，它只能用于需要 Rust 可用的场景（例如启用了 `CONFIG_RUST` 时）。

```

  hostprogs     := target
  target-rust   := y

```
Kbuild 将使用位于与 `Makefile` 相同目录中的 `target.rs` 作为 crate 根来编译 `target`。该 crate 可能由多个源文件组成（见 `samples/rust/hostprogs`）。

### Controlling compiler options for host programs


编译主机程序时，可以设置特定标志。程序将始终使用 $(HOSTCC) 传入 $(KBUILD_HOSTCFLAGS) 中指定的选项进行编译。

要为在该 Makefile 中创建的所有主机程序设置生效的标志，使用变量 HOST_EXTRACFLAGS。

```

  #scripts/lxdialog/Makefile
  HOST_EXTRACFLAGS += -I/usr/include/ncurses

```
要为单个文件设置特定标志，使用以下构造：

```

  #arch/ppc64/boot/Makefile
  HOSTCFLAGS_piggyback.o := -DKERNELBASE=$(KERNELBASE)

```
也可以为链接器指定额外选项。

```

  #scripts/kconfig/Makefile
  HOSTLDLIBS_qconf := -L$(QTDIR)/lib

```
链接 qconf 时，将传入额外选项 `-L$(QTDIR)/lib`。

### When host programs are actually built


Kbuild 仅在主机程序被引用为先决条件时才会构建它。

这有两种方式：

(1) 在自定义规则中显式列出先决条件。

```

      #drivers/pci/Makefile
      hostprogs := gen-devlist
      $(obj)/devlist.h: $(src)/pci.ids $(obj)/gen-devlist
      ( cd $(obj); ./gen-devlist ) < $<

```
    目标 $(obj)/devlist.h 在 $(obj)/gen-devlist 更新之前不会被构建。注意，自定义规则中对主机程序的引用必须以 $(obj) 为前缀。

(2) Use always-y

    当不存在合适的自定义规则，且主机程序应在进入某个 makefile 时被构建时，应使用 always-y 变量。

```

      #scripts/lxdialog/Makefile
      hostprogs     := lxdialog
      always-y      := $(hostprogs)

```
    Kbuild 为此提供了以下简写形式：

      hostprogs-always-y := lxdialog

    这将告诉 kbuild 构建 lxdialog，即使它未被任何规则引用。

## Userspace Program support


与主机程序一样，Kbuild 也支持为目标架构（即你正在为之构建内核的相同架构）构建用户空间可执行文件。

语法非常相似。不同之处在于使用 `userprogs` 而非 `hostprogs`。

### Simple Userspace Program


以下行告诉 kbuild，程序 bpf-direct 应针对目标架构构建。

```

  userprogs := bpf-direct

```
Kbuild 在上面的示例中假设 bpf-direct 由位于与 Makefile 相同目录中的单个 C 源文件 bpf-direct.c 构成。

### Composite Userspace Programs


用户空间程序可以基于复合目标构成。用于定义用户空间程序复合目标的语法与用于内核目标的语法类似。$(<executable>-objs) 列出用于链接最终可执行文件的所有目标。

```

  #samples/seccomp/Makefile
  userprogs      := bpf-fancy
  bpf-fancy-objs := bpf-fancy.o bpf-helper.o

```
扩展名为 .o 的目标由相应的 .c 文件编译而来。在上面的示例中，bpf-fancy.c 被编译为 bpf-fancy.o，bpf-helper.c 被编译为 bpf-helper.o。

最后，这两个 .o 文件被链接到可执行文件 bpf-fancy。注意：语法 <executable>-y 不允许用于用户空间程序。

### Controlling compiler options for userspace programs


编译用户空间程序时，可以设置特定标志。程序将始终使用 $(CC) 传入 $(KBUILD_USERCFLAGS) 中指定的选项进行编译。

要为在该 Makefile 中创建的所有用户空间程序设置生效的标志，使用变量 userccflags。

```

  # samples/seccomp/Makefile
  userccflags += -I usr/include

```
要为单个文件设置特定标志，使用以下构造：

```

  bpf-helper-userccflags += -I user/include

```
也可以为链接器指定额外选项。

```

  # net/bpfilter/Makefile
  bpfilter_umh-userldflags += -static

```
要指定链接到用户空间程序的库，可以使用 `<executable>-userldlibs`。`userldlibs` 语法指定链接到当前 Makefile 中创建的所有用户空间程序的库。

链接 bpfilter_umh 时，将传入额外选项 -static。

从命令行，USERCFLAGS 和 USERLDFLAGS <userkbuildflags> 也会被使用。

### When userspace programs are actually built


Kbuild 仅在被告知时才构建用户空间程序。有两种方式可以做到这一点。

(1) 将其添加为另一文件的先决条件

```

      #net/bpfilter/Makefile
      userprogs := bpfilter_umh
      $(obj)/bpfilter_umh_blob.o: $(obj)/bpfilter_umh

    $(obj)/bpfilter_umh 在 $(obj)/bpfilter_umh_blob.o 之前被构建。

```
(2) Use always-y

```

      userprogs := binderfs_example
      always-y := $(userprogs)

```
    Kbuild 为此提供了以下简写形式：

      userprogs-always-y := binderfs_example

    这将告诉 Kbuild 在访问此 Makefile 时构建 binderfs_example。

## Kbuild clean infrastructure


`make clean` 删除编译内核的对象树中大多数生成的文件。这包括诸如主机程序之类的生成文件。Kbuild 知道列在 $(hostprogs)、$(always-y)、$(always-m)、$(always-)、$(extra-y)、$(extra-) 和 $(targets) 中的目标。它们在执行 `make clean` 时全部被删除。匹配 `**.[oas]`、`**.ko` 模式以及 kbuild 生成的一些附加文件，会在执行 `make clean` 时于整个内核源码树中被删除。

额外的文件或目录可以通过在 kbuild makefile 中使用 $(clean-files) 指定。

```

  #lib/Makefile
  clean-files := crc32table.h

```
执行 `make clean` 时，文件 `crc32table.h` 将被删除。Kbuild 会假设文件与 Makefile 处于相同的相对目录中。

要将某些文件或目录排除在 make clean 之外，使用 $(no-clean-files) 变量。

通常 kbuild 由于 `obj-* := dir/` 而下降进入子目录，但在 kbuild 基础设施不足的架构 makefile 中，有时需要显式指定。

```

  #arch/x86/boot/Makefile
  subdir- := compressed

```
上述赋值指示 kbuild 在执行 `make clean` 时下降进入 compressed/ 目录。

注意 1：arch/$(SRCARCH)/Makefile 不能使用 `subdir-`，因为该文件被包含在顶层 makefile 中。相反，arch/$(SRCARCH)/Kbuild 可以使用 `subdir-`。

注意 2：列在 core-y、libs-y、drivers-y 和 net-y 中的所有目录都会在 `make clean` 期间被访问。

## Architecture Makefiles


顶层 Makefile 在进行下降进入各个目录之前，完成环境搭建与准备工作。

顶层 makefile 包含通用部分，而 arch/$(SRCARCH)/Makefile 包含为所述架构搭建 kbuild 所需的内容。

为此，arch/$(SRCARCH)/Makefile 设置若干变量并定义少量目标。

当 kbuild 执行时，遵循以下步骤（大致）：

1) 内核配置 => 生成 .config

2) 将内核版本存入 include/linux/version.h

3) 更新目标 prepare 的所有其他先决条件：

   - 额外的先决条件在 arch/$(SRCARCH)/Makefile 中指定

4) 递归下降进入列在 init-** core** drivers-** net-** libs-* 中的所有目录并构建所有目标。

   - 上述变量的值在 arch/$(SRCARCH)/Makefile 中展开。

5) 所有目标文件随后被链接，生成的文件 vmlinux 位于对象树的根目录。最先被链接的目标列在 scripts/head-object-list.txt 中。

6) 最后，架构相关部分进行任何所需的后处理并构建最终的启动映像。

   - 这包括构建启动记录
   - 准备 initrd 映像等

### Set variables to tweak the build to the architecture


KBUILD_LDFLAGS
  通用的 $(LD) 选项

  用于链接器所有调用的标志。通常指定仿真就足够了。

```

    #arch/s390/Makefile
    KBUILD_LDFLAGS         := -m elf_s390

```
  注意：ldflags-y 可用于进一步定制所使用的标志。见 `Non-builtin vmlinux targets - extra-y`_。

LDFLAGS_vmlinux
  链接 vmlinux 时用于 $(LD) 的选项

  LDFLAGS_vmlinux 用于指定在链接最终 vmlinux 映像时传递给链接器的额外标志。

  LDFLAGS_vmlinux 使用 LDFLAGS_$@ 支持。

```

    #arch/x86/Makefile
    LDFLAGS_vmlinux := -e stext

```
OBJCOPYFLAGS
  objcopy 标志

  当使用 $(call if_changed,objcopy) 转换 .o 文件时，将使用 OBJCOPYFLAGS 中指定的标志。

  $(call if_changed,objcopy) 常用于在 vmlinux 上生成原始二进制文件。

```

    #arch/s390/Makefile
    OBJCOPYFLAGS := -O binary

    #arch/s390/boot/Makefile
    $(obj)/image: vmlinux FORCE
            $(call if_changed,objcopy)

```
  在此示例中，二进制 $(obj)/image 是 vmlinux 的二进制版本。$(call if_changed,xxx) 的用法将在后面描述。

KBUILD_AFLAGS
  汇编器标志

  默认值 —— 见顶层 Makefile。

  根据架构需要追加或修改。

```

    #arch/sparc64/Makefile
    KBUILD_AFLAGS += -m64 -mcpu=ultrasparc

```
KBUILD_CFLAGS
  $(CC) 编译器标志

  默认值 —— 见顶层 Makefile。

  根据架构需要追加或修改。

  通常，KBUILD_CFLAGS 变量取决于配置。

```

    #arch/x86/boot/compressed/Makefile
    cflags-$(CONFIG_X86_32) := -march=i386
    cflags-$(CONFIG_X86_64) := -mcmodel=small
    KBUILD_CFLAGS += $(cflags-y)

```
  许多架构 Makefile 会动态运行目标 C 编译器来探测受支持的选项：

```

    #arch/x86/Makefile

    ...
    cflags-$(CONFIG_MPENTIUMII)     += $(call cc-option,\
						-march=pentium2,-march=i686)
    ...
    # Disable unit-at-a-time mode ...
    KBUILD_CFLAGS += $(call cc-option,-fno-unit-at-a-time)
    ...


  第一个示例利用了配置选项在被选中时展开为 "y" 的技巧。

```
KBUILD_RUSTFLAGS
  $(RUSTC) 编译器标志

  默认值 —— 见顶层 Makefile。

  根据架构需要追加或修改。

  通常，KBUILD_RUSTFLAGS 变量取决于配置。

  注意，目标规范文件的生成（用于 `--target`）在 `scripts/generate_rust_target.rs` 中处理。

KBUILD_AFLAGS_KERNEL
  专用于内置的汇编器选项

  $(KBUILD_AFLAGS_KERNEL) 包含用于编译常驻内核代码的额外 C 编译器标志。

KBUILD_AFLAGS_MODULE
  专用于模块的汇编器选项

  $(KBUILD_AFLAGS_MODULE) 用于添加用于汇编器的架构相关选项。

  从命令行应使用 AFLAGS_MODULE（见 kbuild.rst）。

KBUILD_CFLAGS_KERNEL
  专用于内置的 $(CC) 选项

  $(KBUILD_CFLAGS_KERNEL) 包含用于编译常驻内核代码的额外 C 编译器标志。

KBUILD_CFLAGS_MODULE
  构建模块时用到的 $(CC) 选项

  $(KBUILD_CFLAGS_MODULE) 用于添加用于 $(CC) 的架构相关选项。

  从命令行应使用 CFLAGS_MODULE（见 kbuild.rst）。

KBUILD_RUSTFLAGS_KERNEL
  专用于内置的 $(RUSTC) 选项

  $(KBUILD_RUSTFLAGS_KERNEL) 包含用于编译常驻内核代码的额外 Rust 编译器标志。

KBUILD_RUSTFLAGS_MODULE
  构建模块时用到的 $(RUSTC) 选项

  $(KBUILD_RUSTFLAGS_MODULE) 用于添加用于 $(RUSTC) 的架构相关选项。

  从命令行应使用 RUSTFLAGS_MODULE（见 kbuild.rst）。

KBUILD_LDFLAGS_MODULE
  链接模块时用到的 $(LD) 选项

  $(KBUILD_LDFLAGS_MODULE) 用于添加用于链接模块时的架构相关选项。这通常是一个链接器脚本。

  从命令行应使用 LDFLAGS_MODULE（见 kbuild.rst）。

KBUILD_LDS
  带有完整路径的链接器脚本。由顶层 Makefile 赋值。

KBUILD_VMLINUX_OBJS
  vmlinux 的所有目标文件。它们以 KBUILD_VMLINUX_OBJS 中列出的相同顺序链接进 vmlinux。

  scripts/head-object-list.txt 中列出的目标为例外；它们被放置在其他目标之前。

KBUILD_VMLINUX_LIBS
  vmlinux 的所有 .a `lib` 文件。KBUILD_VMLINUX_OBJS 和 KBUILD_VMLINUX_LIBS 共同指定了用于链接 vmlinux 的所有目标文件。

### Add prerequisites to archheaders


archheaders: 规则用于生成可能由 `make headers_install` 安装到用户空间的头文件。

当在架构本身上运行时，它会在 `make archprepare` 之前运行。

### Add prerequisites to archprepare


archprepare: 规则用于列出在开始下降进入子目录之前需要构建的先决条件。

这通常用于包含汇编常量的头文件。

```

  #arch/arm/Makefile
  archprepare: maketools

```
在此示例中，文件目标 maketools 将在下降进入子目录之前被处理。

另见章节 XXX-TODO，它描述了 kbuild 如何支持生成偏移头文件。

### List directories to visit when descending


架构 Makefile 与顶层 Makefile 协作，定义指定如何构建 vmlinux 文件的变量。注意，模块没有相应的架构相关章节；模块的构建机制完全与架构无关。

core-y, libs-y, drivers-y
  $(libs-y) 列出可定位 lib.a 归档的目录。

  其余列出可定位 built-in.a 目标文件的目录。

  然后其余按以下顺序：

    $(core-y), $(libs-y), $(drivers-y)

  顶层 Makefile 为所有通用目录定义取值，而 arch/$(SRCARCH)/Makefile 只添加架构相关的目录。

```

    # arch/sparc/Makefile
    core-y                 += arch/sparc/

    libs-y                 += arch/sparc/prom/
    libs-y                 += arch/sparc/lib/

    drivers-$(CONFIG_PM) += arch/sparc/power/

```
### Architecture-specific boot images


架构 Makefile 指定将 vmlinux 文件压缩、用引导代码包裹并将结果文件复制到某处的目标。这包括各种安装命令。实际目标在各架构间并不标准化。

通常将任何额外处理放在 arch/$(SRCARCH)/ 下的 boot/ 目录中。

Kbuild 没有提供任何智能方式来支持构建 boot/ 中指定的目标。因此 arch/$(SRCARCH)/Makefile 应手动调用 make 来构建 boot/ 中的目标。

推荐的做法是在 arch/$(SRCARCH)/Makefile 中包含快捷方式，并在向下调用 arch/$(SRCARCH)/boot/Makefile 时使用完整路径。

```

  #arch/x86/Makefile
  boot := arch/x86/boot
  bzImage: vmlinux
          $(Q)$(MAKE) $(build)=$(boot) $(boot)/$@

```
`$(Q)$(MAKE) $(build)=<dir>` 是调用子目录中 make 的推荐方式。

对于架构相关目标的命名没有规则，但执行 `make help` 会列出所有相关目标。为了支持这一点，必须定义 $(archhelp)。

```

  #arch/x86/Makefile
  define archhelp
    echo  '* bzImage      - Compressed kernel image (arch/x86/boot/bzImage)'
  endif

```
当 make 不带参数执行时，遇到的第一个目标将被构建。在顶层 Makefile 中，存在的第一个目标是 all:。

架构在默认情况下应始终构建一个可启动映像。在 `make help` 中，默认目标以 `*` 高亮显示。

向 all: 添加一个新的先决条件以选择不同于 vmlinux 的默认目标。

```

  #arch/x86/Makefile
  all: bzImage

```
当不带参数执行 `make` 时，将构建 bzImage。

### Commands useful for building a boot image


Kbuild 提供了一些在构建启动映像时有用的宏。

ld
  链接目标。通常，LDFLAGS_$@ 用于为 ld 设置特定选项。

```

    #arch/x86/boot/Makefile
    LDFLAGS_bootsect := -Ttext 0x0 -s --oformat binary
    LDFLAGS_setup    := -Ttext 0x0 -s --oformat binary -e begtext

    targets += setup setup.o bootsect bootsect.o
    $(obj)/setup $(obj)/bootsect: %: %.o FORCE
            $(call if_changed,ld)

```
  在此示例中，有两个可能的目标，需要不同的链接器选项。链接器选项使用 LDFLAGS_$@ 语法指定 —— 每个潜在目标一个。

  $(targets) 被赋值为所有潜在目标，由此 kbuild 知道这些目标并将：

  1) 检查命令行的变化
  2) 在 make clean 期间删除目标

  ``: %: %.o`` 部分的先决条件是一个简写，使我们不必列出 setup.o 和 bootsect.o 文件。

  注意：
  忘记 ``targets :=`` 赋值是一个常见错误，会导致目标文件在没有明显原因的情况下被重新编译。

objcopy
  复制二进制文件。通常使用 arch/$(SRCARCH)/Makefile 中指定的 OBJCOPYFLAGS。

  OBJCOPYFLAGS_$@ 可用于设置额外选项。

gzip
  压缩目标。使用最大压缩来压缩目标。

```

    #arch/x86/boot/compressed/Makefile
    $(obj)/vmlinux.bin.gz: $(vmlinux.bin.all-y) FORCE
            $(call if_changed,gzip)

```
dtc
  创建扁平设备树 blob 对象，适合链接进 vmlinux。链接进 vmlinux 的设备树 blob 被放置在映像的一个 init 段中。平台代码 **必须** 在调用 unflatten_device_tree() 之前将该 blob 复制到非 init 内存。

  要使用此命令，只需将 `*.dtb` 加入 obj-y 或 targets，或让其他某个目标依赖于 `%.dtb`。

  存在一条中心规则用于从 `$(src)/%.dts` 创建 `$(obj)/%.dtb`；架构 Makefile 无需显式写出该规则。

```

    targets += $(dtb-y)
    DTC_FLAGS ?= -p 1024

```
### Preprocessing linker scripts


构建 vmlinux 映像时，使用链接脚本 arch/$(SRCARCH)/kernel/vmlinux.lds。

该脚本是同目录下文件 vmlinux.lds.S 的预处理变体。

kbuild 认识 .lds 文件并包含一条规则 `**lds.S` -> `**lds`。

```

  #arch/x86/kernel/Makefile
  extra-y := vmlinux.lds

```
对 extra-y 的赋值用于告诉 kbuild 构建目标 vmlinux.lds。

对 $(CPPFLAGS_vmlinux.lds) 的赋值告诉 kbuild 在构建目标 vmlinux.lds 时使用指定的选项。

```

  KBUILD_CPPFLAGS      : Set in top-level Makefile
  cppflags-y           : May be set in the kbuild makefile
  CPPFLAGS_$(@F)       : Target-specific flags.
                         Note that the full filename is used in this
                         assignment.

```
`*lds` 文件的 kbuild 基础设施在多个架构相关文件中被使用。

### Generic header files


目录 include/asm-generic 包含可在各个架构之间共享的头文件。

使用通用头文件的推荐方法是在 Kbuild 文件中列出该文件。

有关语法等的更多信息，见 `generic-y`_。

### Post-link pass


如果文件 arch/xxx/Makefile.postlink 存在，该 makefile 将被调用以对后链接对象（vmlinux 和 modules.ko）运行后链接处理，供架构使用。它还必须处理 clean 目标。

此过程在 kallsyms 生成之后运行。如果架构需要修改符号位置，而不是操作 kallsyms，那么为 .tmp_vmlinux? 目标添加另一个 postlink 目标、由 link-vmlinux.sh 调用可能更方便。

例如，powerpc 用它来检查链接后的 vmlinux 文件的重定位完整性。

## Kbuild syntax for exported headers


内核包含一组导出到用户空间的头文件。许多头文件可以原样导出，但其他头文件在可供用户空间使用之前需要最少的预处理。

预处理会：

- 丢弃内核特定的注解
- 丢弃对 compiler.h 的包含
- 丢弃所有内核内部（由 `ifdef __KERNEL__` 保护）的段

include/uapi/、include/generated/uapi/、arch/<arch>/include/uapi/ 和 arch/<arch>/include/generated/uapi/ 下的所有头文件都会被导出。

可以在 arch/<arch>/include/uapi/asm/ 和 arch/<arch>/include/asm/ 下定义 Kbuild 文件，以列出来自 asm-generic 的 asm 文件。

有关 Kbuild 文件的语法，见后续章节。

### no-export-headers


no-export-headers 本质上由 include/uapi/linux/Kbuild 使用，以避免在不支持某些头文件（例如 kvm.h）的架构上导出它们。应尽量避免使用它。

### generic-y


如果某个架构逐字使用来自 include/asm-generic 的一个头文件副本，则在文件 arch/$(SRCARCH)/include/asm/Kbuild 中按如下方式列出：

```

  #arch/x86/include/asm/Kbuild
  generic-y += termios.h
  generic-y += rtc.h

```
在构建的准备阶段会生成一个包装包含

```

  arch/$(SRCARCH)/include/generated/asm

```
当导出一个架构使用通用头的头文件时，会生成类似的包装作为

```

  usr/include/asm

```
在这两种情况下，生成的包装都如下所示：

```

  #include <asm-generic/termios.h>

```
### generated-y


如果某个架构在 generic-y 包装之外还生成其他头文件，generated-y 指定它们。

这可以阻止它们被当作过时的 asm-generic 包装而被删除。

```

  #arch/x86/include/asm/Kbuild
  generated-y += syscalls_32.h

```
### mandatory-y


mandatory-y 本质上由 include/(uapi/)asm-generic/Kbuild 使用，用于定义所有架构都必须具备的最小 ASM 头文件集合。

它类似于可选的 generic-y。如果 arch/$(SRCARCH)/include/(uapi/)/asm 中缺少某个强制头文件，Kbuild 将自动生成该 asm-generic 头文件的包装。

## Kbuild Variables


顶层 Makefile 导出以下变量：

VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION
  这些变量定义当前的内核版本。少数架构 Makefile 会直接使用这些值；它们应当改用 $(KERNELRELEASE)。

  $(VERSION)、$(PATCHLEVEL) 和 $(SUBLEVEL) 定义基本的三段版本号，例如 "2"、"4" 和 "0"。这三个值始终是数字。

  $(EXTRAVERSION) 为预补丁或附加补丁定义一个更小的子级别。它通常是某个非数字字符串，例如 "-pre4"，并且经常为空。

KERNELRELEASE
  $(KERNELRELEASE) 是一个单字符串，例如 "2.4.0-pre4"，适合用于构造安装目录名或显示在版本字符串中。一些架构 Makefile 将其用于此目的。

ARCH
  此变量定义目标架构，例如 "i386"、"arm" 或 "sparc"。一些 kbuild Makefile 会测试 $(ARCH) 以确定要编译哪些文件。

  默认情况下，顶层 Makefile 将 $(ARCH) 设置为与主机系统架构相同。对于交叉构建，用户可以

```

    make ARCH=m68k ...

```
SRCARCH
  此变量指定 arch/ 中要构建的目录。

  ARCH 和 SRCARCH 不一定匹配。有几个 arch 目录是双架构（biarch）的，即单个 `arch/*/` 目录同时支持 32 位和 64 位。

  例如，你可以传入 ARCH=i386、ARCH=x86_64 或 ARCH=x86。对它们全部而言，SRCARCH=x86，因为 arch/x86/ 同时支持 i386 和 x86_64。

INSTALL_PATH
  此变量为架构 Makefile 定义安装常驻内核映像和 System.map 文件的位置。将其用于架构相关的安装目标。

INSTALL_MOD_PATH, MODLIB
  $(INSTALL_MOD_PATH) 为模块安装的 $(MODLIB) 指定前缀。该变量未在 Makefile 中定义，但可按用户意愿从命令行传入。

  $(MODLIB) 指定模块安装目录。顶层 Makefile 将 $(MODLIB) 定义为 $(INSTALL_MOD_PATH)/lib/modules/$(KERNELRELEASE)。用户可按需从命令行覆盖此值。

INSTALL_MOD_STRIP
  如果指定了此变量，它将导致模块在安装后被 strip。如果 INSTALL_MOD_STRIP 为 "1"，则使用默认选项 --strip-debug。否则，INSTALL_MOD_STRIP 的值将作为 strip 命令的选项使用。

INSTALL_DTBS_PATH
  此变量为构建根所需的重定位指定前缀。它定义安装设备树 blob 的位置。与 INSTALL_MOD_PATH 类似，它未在 Makefile 中定义，但可按用户意愿传入。否则默认使用内核安装路径。

## Makefile language


内核 Makefile 被设计为使用 GNU Make 运行。Makefile 只使用 GNU Make 的文档化特性，但它们确实使用了许多 GNU 扩展。

GNU Make 支持基本的列表处理函数。内核 Makefile 使用一种新颖的列表构建与操作风格，几乎不使用 `if` 语句。

GNU Make 有两个赋值运算符，`:=` 和 `=`。`:=` 对右侧进行立即求值，并将一个实际的字符串存入左侧。`=` 类似于公式定义；它将右侧以未求值的形式存储，然后在每次使用左侧时对该形式进行求值。

在某些情况下 `=` 是合适的。不过，通常 `:=` 才是正确的选择。

## Credits


- 原始版本由 Michael Elizabeth Chastain 制作，<mailto:mec@shout.net>
- 由 Kai Germaschewski <kai@tp1.ruhr-uni-bochum.de> 更新
- 由 Sam Ravnborg <sam@ravnborg.org> 更新
- 语言质量检查由 Jan Engelhardt <jengelh@gmx.de> 完成

## TODO


- 生成偏移头文件。
- 向第 7 或第 9 章添加更多变量？
