
## Adding a new board to LinuxSH


               Paul Mundt <lethal@linux-sh.org>

本文档试图概述在新的 2.5 和 2.6 内核下，为 LinuxSH 移植版添加新板卡支持所需
的步骤。同时也试图说明 2.4 与 2.5/2.6 SH 后端之间一些显著的变化。

## 1. New Directory Structure


首先要注意的是新的目录结构。在 2.4 下，绝大多数板卡相关的代码（stboards 除外）
最终都直接放在 arch/sh/kernel/ 中，板卡相关的头文件则放在 include/asm-sh/ 中。
在新内核中，代码按板卡类型、配套芯片类型以及 CPU 类型拆分。从该目录层级的树状
视图来看，大致如下：

```

    .
    |-- arch
    |   `-- sh
    |       `-- boards
    |           |-- adx
    |           |   `-- board-specific files
    |           |-- bigsur
    |           |   `-- board-specific files
    |           |
    |           ... more boards here ...
    |
    `-- include
	`-- asm-sh
	    |-- adx
	    |   `-- board-specific headers
	    |-- bigsur
	    |   `-- board-specific headers
	    |
	    .. more boards here ...

```
```

    .
    `-- arch
	`-- sh
	    `-- cchips
		`-- hd6446x
		    `-- hd64461
			`-- cchip-specific files

```
……以此类推。配套芯片的头文件与板卡特定的头文件处理方式相同。因此，include/asm-sh/hd64461
存放了所有 hd64461 特定的头文件。

```

    .
    |-- arch
    |   `-- sh
    |       |-- kernel
    |       |   `-- cpu
    |       |       |-- sh2
    |       |       |   `-- SH-2 generic files
    |       |       |-- sh3
    |       |       |   `-- SH-3 generic files
    |       |       `-- sh4
    |       |           `-- SH-4 generic files
    |       `-- mm
    |           `-- This is also broken out per CPU family, so each family can
    |               have their own set of cache/tlb functions.
    |
    `-- include
	`-- asm-sh
	    |-- cpu-sh2
	    |   `-- SH-2 specific headers
	    |-- cpu-sh3
	    |   `-- SH-3 specific headers
	    `-- cpu-sh4
		`-- SH-4 specific headers

```
应当注意，CPU 子类型并_不_做抽象。因此，这些仍需由 CPU 系列相关的代码来处理。

## 2. Adding a New Board


首先要确定的是，你正在添加的板卡是独立的，还是属于一个板卡家族——该家族中
各成员差别很小，大多可以共享相同的板卡特定代码。

在第一种情况下，只需在 arch/sh/boards/ 下为你的板卡建一个目录，并添加规则将你的
板卡挂接到构建系统（下一节详述）。但对于板卡家族，更合理的做法是在 arch/sh/boards/
下建立一个公共的顶层目录，然后在该目录下为每个家族成员建立子目录。Solution Engine
和 hp6xx 板卡都是这种例子。

设置好新的 arch/sh/boards/ 目录后，请记住你还应在 include/asm-sh 下添加一个专属于
该板卡的目录（如果会有多个的话）。为了能与构建系统无缝协作，最好让该目录名与
arch/sh/boards/ 的目录名相同；不过如果你的板卡又属于某个家族，构建系统有办法处理
这种情况（通过 incdir-y 重载），你也可以自由地按家族成员本身来命名目录。

每个板卡在 arch/sh/boards 和 include/asm-sh/ 层级下都需要具备一些要素。为了更好地
说明，我们以添加一个虚拟板卡为例。对于初始化代码，我们至少必须提供 get_system_type()
和 platform_setup() 的定义。对于我们的虚拟板卡，这
```

    /*
    * arch/sh/boards/vapor/setup.c - Setup code for imaginary board
    */
    #include <linux/init.h>

    const char *get_system_type(void)
    {
	    return "FooTech Vaporboard";
    }

    int __init platform_setup(void)
    {
	    /*
	    * If our hardware actually existed, we would do real
	    * setup here. Though it's also sane to leave this empty
	    * if there's no real init work that has to be done for
	    * this board.
	    */

	    /* Start-up imaginary PCI ... */

	    /* And whatever else ... */

	    return 0;
    }

```
我们新的虚拟板卡还必须在 machvec 中挂接，才能发挥作用。

machvec 函数分为若干类：

 - 访问 IO 内存（inb 等）和 PCI/主内存（readb 等）的 I/O 函数。
 - I/O 映射函数（ioport_map、ioport_unmap 等）。
 - 一个“心跳”（heartbeat）函数。
 - PCI 和 IRQ 初始化例程。
 - 一致性分配器（consistent allocator，针对需要特殊分配器、尤其是要从某些板卡
   特定的 SRAM 中为 DMA handle 分配内存的板卡）。

machvec 函数会随时间不断增减，因此请务必查阅 include/asm-sh/machvec.h 以了解
machvec 的当前状态。

内核会在启动时自动为 machvec 中未定义的函数指针套用通用例程，因为 machvec 函数在
内核树的大部分地方都是无条件引用的。有些板卡的 machvec 极为精简（如 dreamcast 和
sh03），而另一些则必须定义几乎全部（rts7751r2d）。

添加一个新机器相当简单（以 vapor 为例）：

如果板卡特定的定义非常精简（绝大多数板卡都是这种情况），那么只需一个单独的板卡
特定头文件就足够了。

 - 添加一个新文件 include/asm-sh/vapor.h，其中包含以机器名作为前缀的、所有机器
   特定 IO 函数的原型，例如 vapor_inb。在填写机器向量（machine vector）时会用到它们。

   注意，这些原型通过设置
```

	#define __IO_PREFIX vapor
	#include <asm/io_generic.h>

   somewhere in the board-specific header. Any boards being ported that still
   have a legacy io.h should remove it entirely and switch to the new model.

 - Add machine vector definitions to the board's setup.c. At a bare minimum,
   this must be defined as something like::

	struct sh_machine_vector mv_vapor __initmv = {
		.mv_name = "vapor",
	};
	ALIAS_MV(vapor)

 - finally add a file arch/sh/boards/vapor/io.c, which contains definitions of
   the machine specific io functions (if there are enough to warrant it).

```
## 3. Hooking into the Build System


现在目录都已建立，所有板卡特定代码也已就位，是时候看看如何让这一团东西融入
构建系统了。

构建系统的很大部分现在是完全动态的，只需要在各处加入合适的条目即可完成工作。

首先要做的是在 arch/sh/Kconfig 的
```

    config SH_VAPOR
	    bool "Vapor"
	    help
	    select Vapor if configuring for a FooTech Vaporboard.

```
接下来，必须把它加入 arch/sh/Makefile。所有板卡都需要一个 machdir-y 条目才能被
构建。该条目必须是板卡目录在 arch/sh/boards 中出现的名称，即使它位于子目录中（在
那种情况下，arch/sh/boards/ 以下的所有父目录
```

    machdir-$(CONFIG_SH_VAPOR)	+= vapor

```
前提是我们已经把所有内容放在 arch/sh/boards/vapor/ 目录中。

接下来，构建系统假定你的 include/asm-sh 目录也会使用相同的名字。如果不是（例如
属于同一公共家族的多个板卡），则需要将该目录名隐式追加到 incdir-y。现有代码已经
为 Solution Engine 和 hp6xx 板卡处理了这一点，可参考这些例子。

处理好之后，就到了为 mach 类型添加条目的环节。这通过向 arch/sh/tools/mach-types
列表末尾添加条目来完成。做法不言自明，此处不再赘述。完成后，如果你在整个
```

	/* Make sure we're on the FooTech Vaporboard */
	if (!mach_is_vapor())
		return -ENODEV;

```
还要注意，mach_is_boardname() 检查会被隐式强制转为小写，尽管 mach-types 条目全部
是大写。如果你真的很在意可以读那个脚本，但它相当丑陋，所以你可能并不想这么做。

现在剩下的就是为你的新板卡提供一个 defconfig。这样，最终拿到这块板卡的其他人
就可以直接参考该配置，而不必去猜测应当使用哪些设置。

另外，一旦你为新板卡复制了一个示例 .config（假设为 arch/sh/configs/vapor_defconfig），
你也可以直接将它作为一个构建目标使用，它会被隐式地列在 help 文本中。

查看 'make help' 的输出，你现在应该会看到类似如下内容：

Architecture specific targets (sh)：

  =======================   =============================================
  zImage                    Compressed kernel image (arch/sh/boot/zImage)
  adx_defconfig             Build for adx
  cqreek_defconfig          Build for cqreek
  dreamcast_defconfig       Build for dreamcast
  ...
  vapor_defconfig           Build for vapor
  =======================   =============================================

```

    $ make ARCH=sh CROSS_COMPILE=sh4-linux- vapor_defconfig vmlinux

```
它会进而复制该板卡的 defconfig，用 oldconfig 跑一遍（自创建以来若有新选项会提示你
确认），然后带你踏上为新板卡构建一个可用内核的征程。
