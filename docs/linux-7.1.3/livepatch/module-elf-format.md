## Livepatch 模块 ELF 格式

本文档概述了 livepatch 模块所必须遵守的 ELF 格式要求。

## 1. 背景与动机

早先，livepatch 需要单独的架构相关代码来写入重定位项（relocations）。然而，用于写入重定位项的架构相关代码已经存在于模块加载器中，因此这种旧方法产生了冗余代码。于是，livepatch 不再重复造轮子、也不再重新实现模块加载器已经能做到的事情，而是借助模块加载器中已有的代码来完成所有架构相关的重定位工作。具体而言，livepatch 复用了模块加载器中的 apply_relocate_add() 函数来写入重定位项。本文档所描述的补丁模块 ELF 格式，使得 livepatch 能够这样做。我们希望借此让 livepatch 更容易移植到其他架构，并减少将 livepatch 移植到某个特定架构所需的架构相关代码量。

由于 apply_relocate_add() 需要访问模块的节头表、符号表以及重定位节索引，livepatch 模块的 ELF 信息会被保留（见第 5 节）。livepatch 管理它自己的重定位节和符号，这些将在本文档中描述。用于标记 livepatch 符号和重定位节的 ELF 常量，是根据 glibc 的定义从 OS 专用范围中挑选的。

### 为什么 livepatch 需要写入自己的重定位项？

一个典型的 livepatch 模块包含被补丁化函数的补丁版本，这些版本可能引用未导出的全局符号以及未包含进来的局部符号。引用这类符号的重定位项不能原样保留，因为内核模块加载器无法解析它们，从而会拒绝该 livepatch 模块。此外，我们无法对补丁模块加载时尚不存在的模块应用重定位（例如对某个尚未加载的驱动做补丁）。早先，livepatch 通过在生成的补丁模块 ELF 输出中嵌入特殊的 “dynrela”（动态 rela）节来解决这个问题。借助这些 dynrela 节，livepatch 可以在考虑符号作用域以及符号所属模块的前提下解析符号，然后手动应用这些动态重定位。然而这种方法要求 livepatch 提供架构相关代码来写入这些重定位项。在新的格式中，livepatch 用自身的 SHT_RELA 重定位节取代 dynrela 节，而 rela 所引用的符号是特殊的 livepatch 符号（见第 2、3 节）。架构相关的 livepatch 重定位代码被一次对 apply_relocate_add() 的调用所取代。

## 2. Livepatch modinfo 字段

livepatch 模块必须带有 “livepatch” modinfo 属性。关于如何做到这一点，请参见 samples/livepatch/ 中的示例 livepatch 模块。

用户可以使用 'modinfo' 命令、并通过查找是否存在 “livepatch” 字段来识别 livepatch 模块。该字段也被内核模块加载器用于识别 livepatch 模块。

### Example（示例）：


**Modinfo 输出：**

```

	% modinfo livepatch-meminfo.ko
	filename:		livepatch-meminfo.ko
	livepatch:		Y
	license:		GPL
	depends:
	vermagic:		4.3.0+ SMP mod_unload

```

## 3. Livepatch 重定位节

一个 livepatch 模块管理它自己的 ELF 重定位节，以便在恰当的时机将重定位项应用到模块以及内核（vmlinux）。例如，如果一个补丁模块对某个当前尚未加载的驱动打补丁，livepatch 会在该驱动加载时，将相应的 livepatch 重定位节应用到该驱动。

一个补丁模块中的每个 “对象”（例如 vmlinux，或一个模块）可能关联有多个 livepatch 重定位节（例如对同一对象内多个函数的补丁）。一个 livepatch 重定位节与它所应用的那个目标节（通常是某函数的 text 节）之间存在一一对应关系。一个 livepatch 模块也有可能没有任何 livepatch 重定位节，示例 livepatch 模块就是这种情况（见 samples/livepatch）。

由于 ELF 信息会在 livepatch 模块中保留（见第 5 节），一个 livepatch 重定位节只需把相应的节索引传给 apply_relocate_add() 即可被应用，后者随后用它来访问该重定位节并应用重定位项。

livepatch 重定位节中，每个被 rela 引用的符号都是一个 livepatch 符号。在 livepatch 调用 apply_relocate_add() 之前，必须先解析它们。更多信息见第 3 节。

## 3.1 Livepatch 重定位节格式

livepatch 重定位节必须用 SHF_RELA_LIVEPATCH 节标志标记。定义见 include/uapi/linux/elf.h。模块加载器识别这一标志，并会避免在补丁模块加载时应用这些重定位节。这些节还必须用 SHF_ALLOC 标记，以便模块加载器在加载模块时不丢弃它们（即它们会和其他 SHF_ALLOC 节一起被复制到内存中）。

livepatch 重定位节的名称必须符合以下格式
```

  .klp.rela.objname.section_name
  ^        ^^     ^ ^          ^
  |________||_____| |__________|
     [A]      [B]        [C]

```
[A]
  重定位节名称以字符串 ".klp.rela." 为前缀。

[B]
  该重定位节所属对象（即 "vmlinux" 或模块名）的名称紧跟在前缀之后。

[C]
  该重定位节所应用到的那个节的实际名称。

### Examples（示例）：


**Livepatch 重定位节名称：**

```

  .klp.rela.ext4.text.ext4_attr_store
  .klp.rela.vmlinux.text.cmdline_proc_show

```
**`readelf --sections` 输出，针对一个对 vmlinux 以及模块 9p、btrfs、ext4 打补丁的
补丁模块：**

```

  Section Headers:
  [Nr] Name                          Type                    Address          Off    Size   ES Flg Lk Inf Al
  [ snip ]
  [29] .klp.rela.9p.text.caches.show RELA                    0000000000000000 002d58 0000c0 18 AIo 64   9  8
  [30] .klp.rela.btrfs.text.btrfs.feature.attr.show RELA     0000000000000000 002e18 000060 18 AIo 64  11  8
  [ snip ]
  [34] .klp.rela.ext4.text.ext4.attr.store RELA              0000000000000000 002fd8 0000d8 18 AIo 64  13  8
  [35] .klp.rela.ext4.text.ext4.attr.show RELA               0000000000000000 0030b0 000150 18 AIo 64  15  8
  [36] .klp.rela.vmlinux.text.cmdline.proc.show RELA         0000000000000000 003200 000018 18 AIo 64  17  8
  [37] .klp.rela.vmlinux.text.meminfo.proc.show RELA         0000000000000000 003218 0000f0 18 AIo 64  19  8
  [ snip ]                                       ^                                             ^
                                                 |                                             |
                                                [*]                                           [*]

```
[*]
  Livepatch 重定位节是 SHT_RELA 节，但带有一些特殊特征。注意它们被标记为 SHF_ALLOC（"A"），这样当模块被加载进内存时不会被丢弃，同时它们也被标记为 SHF_RELA_LIVEPATCH 标志（"o" —— 表示 OS 专用）。

**`readelf --relocs` 输出，针对一个补丁模块：**

```

  Relocation section '.klp.rela.btrfs.text.btrfs_feature_attr_show' at offset 0x2ba0 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
  000000000000001f  0000005e00000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.printk,0 - 4
  0000000000000028  0000003d0000000b R_X86_64_32S           0000000000000000 .klp.sym.btrfs.btrfs_ktype,0 + 0
  0000000000000036  0000003b00000002 R_X86_64_PC32          0000000000000000 .klp.sym.btrfs.can_modify_feature.isra.3,0 - 4
  000000000000004c  0000004900000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.snprintf,0 - 4
  [ snip ]                                                                   ^
                                                                             |
                                                                            [*]

```
[*]
  重定位项所引用的每个符号都是一个 livepatch 符号。

## 4. Livepatch 符号

livepatch 符号是被 livepatch 重定位节所引用的符号。这些是来自补丁对象的新版本函数所访问的符号，其地址无法被模块加载器解析（因为它们是局部的或未导出的全局符号）。由于模块加载器只解析已导出的符号，而新补丁函数所引用的符号并非每一个都已导出，于是引入了 livepatch 符号。在补丁模块加载时我们无法立即得知某个符号地址的情况下，也会用到它们。例如，当 livepatch 对某个尚未加载的模块打补丁时就是这种情况。在这种情况下，相关的 livepatch 符号会在目标模块加载时简单地完成解析。无论如何，对于任何 livepatch 重定位节，该节所引用的所有 livepatch 符号都必须在 livepatch 能够对该重定位节调用 apply_relocate_add() 之前被解析。

livepatch 符号必须用 SHN_LIVEPATCH 标记，以便模块加载器能够识别并忽略它们。livepatch 模块将这些符号保留在它们的符号表中，而符号表通过 module->symtab 变得可访问。

## 4.1 一个 livepatch 模块的符号表

通常，模块符号表的一个精简副本（仅包含 “核心” 符号）会通过 module->symtab 提供（见 kernel/module/kallsyms.c 中的 layout_symtab()）。对于 livepatch 模块，在模块加载时复制到内存中的符号表必须与补丁模块编译时生成的符号表完全一致。这是因为每个 livepatch 重定位节中的重定位项都是通过各自的符号索引来引用相应符号的，而原始的符号索引（以及符号表的排序）必须被保留，以便 apply_relocate_add() 能找到正确的符号。

```

  Relocation section '.klp.rela.btrfs.text.btrfs_feature_attr_show' at offset 0x2ba0 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
  000000000000001f  0000005e00000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.printk,0 - 4

```
这个 rela 引用符号 '.klp.sym.vmlinux.printk,0'，符号索引编码在 'Info' 中。这里它的符号索引是 0x5e，即十进制的 94，指向符号索引 94。

而在该补丁模块对应的符号表中，符号索引 94 指向
```

  [ snip ]
  94: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.printk,0
  [ snip ]

```

## 4.2 Livepatch 符号格式

livepatch 符号的节索引必须标记为 SHN_LIVEPATCH，以便模块加载器能够识别它们、并不去尝试解析它们。实际定义见 include/uapi/linux/elf.h。

```

  .klp.sym.objname.symbol_name,sympos
  ^       ^^     ^ ^         ^ ^
  |_______||_____| |_________| |
     [A]     [B]       [C]    [D]

```
[A]
  符号名称以字符串 ".klp.sym." 为前缀。

[B]
  该符号所属对象（即 "vmlinux" 或模块名）的名称紧跟在前缀之后。

[C]
  符号的实际名称。

[D]
  符号在对象中的位置（按 kallsyms 计算）。这用于区分同一对象内的重复符号。符号位置以数字表示（0、1、2……）。唯一符号的符号位置为 0。

### Examples（示例）：


**Livepatch 符号名称：**

```

	.klp.sym.vmlinux.snprintf,0
	.klp.sym.vmlinux.printk,0
	.klp.sym.btrfs.btrfs_ktype,0

```
**`readelf --symbols` 输出，针对一个补丁模块：**

```

  Symbol table '.symtab' contains 127 entries:
     Num:    Value          Size Type    Bind   Vis     Ndx         Name
     [ snip ]
      73: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.snprintf,0
      74: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.capable,0
      75: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.find_next_bit,0
      76: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.si_swapinfo,0
    [ snip ]                                               ^
                                                           |
                                                          [*]

```
[*]
  注意这些符号的 'Ndx'（节索引）是 SHN_LIVEPATCH（0xff20）。
  "OS" 表示 OS 专用。

## 5. 符号表与 ELF 节访问

一个 livepatch 模块的符号表可通过 module->symtab 访问。

由于 apply_relocate_add() 需要访问模块的节头、符号表以及重定位节索引，livepatch 模块的 ELF 信息会被保留，并由模块加载器通过 module->klp_info（它是一个 `klp_modinfo` 结构体）提供访问。当一个 livepatch 模块加载时，该结构体由模块加载器填充。
