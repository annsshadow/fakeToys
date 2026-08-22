## Building External Modules

本文档介绍如何构建一个树外（out-of-tree）内核模块
## 简
"kbuild" Linux 内核使用的构建系统。模块必须使kbuild，才能与构建基础设施的变化保持兼容，并获得传给编译器的正确标志。无论是树内（in-tree）还是树外（out-of-tree）的模块，构建功能都已提供。两者的构建方法相似，而且所有模块最初都是在树外开发和构建的
本文档面向那些有兴趣构建树外（或"external"，外部）模块的开发者。外部模块的作者应当提供一makefile，把大部分复杂性隐藏起来，这样只需输入 "make" 即可构建模块。这很容易做到，完整的示例将`Creating a Kbuild File for an External Module`_ 一节中给出
## 如何构建外部模块

要构建外部模块，你必须准备好一个已经预先构建好的内核，其中包含构建时所用到的配置与头文件。此外，该内核必须是在启用模块的情况下构建的。如果你使用的是发行版内核，你的发行版通常会提供与你正在运行的内核相对应的软件包
另一种做法是使用 "make" "modules_prepare" 目标。这会确保内核包含所需的信息。该目标存在的唯一目的，就是为构建外部模块准备内核源码树提供一种简单方式
注意modules_prepare" 即使设置CONFIG_MODVERSIONS，也不会构建 Module.symvers；因此，要使模块版本控制生效，需要执行一次完整的内核构建
### 命令语法

```

		$ make -C <path_to_kernel_dir> M=$PWD

	The kbuild system knows that an external module is being built
	due to the "M=<dir>" option given in the command.

	To build against the running kernel use::

		$ make -C /lib/modules/`uname -r`/build M=$PWD

	Then to install the module(s) just built, add the target
	"modules_install" to the command::

		$ make -C /lib/modules/`uname -r`/build M=$PWD modules_install

	Starting from Linux 6.13, you can use the -f option instead of -C. This
	will avoid unnecessary change of the working directory. The external
	module will be output to the directory where you invoke make.

		$ make -f /lib/modules/`uname -r`/build/Makefile M=$PWD

```

### 选项

	KDIR 指内核源码目录的路径，若内核是在单独的构建目录中构建的，则指内核输出目录的路径。）

	如果你想在单独的目录中构建模块，可以选择传入 MO= 选项
	make -C $KDIR M=$PWD [MO=$BUILD_DIR]

	-C $KDIR
		包含用于构建外部模块的内核及相关构建产物的目录		"make" 在执行时实际上会切换到指定的目录，并在结束后切回
	M=$PWD
		告知 kbuild 正在构建一个外部模块		传给 "M" 的值是外部模块（kbuild 文件）所在目录的绝对路径
	MO=$BUILD_DIR
		为外部模块指定一个独立的输出目录
### 目标

	构建外部模块时，只有 "make" 目标的一个子集可用
	make -C $KDIR M=$PWD [target]

	默认会构建位于当前目录中的模块，因此无需指定目标。所有输出文件也会在该目录中生成。不会尝试更新内核源码，并且前提是内核已经成功执行过一"make"
	modules
		外部模块的默认目标。其功能和未指定目标时相同。参见上面的说明
	modules_install
		安装外部模块。默认位置是
		/lib/modules/<kernel_release>/updates/，但可以通过 INSTALL_MOD_PATH 添加前缀（在 `Module Installation`_ 一节中讨论）
	clean
		仅删除模块目录中生成的所有文件
	help
		列出外部模块可用的目标
### 构建单独的文
	可以构建属于某个模块的单个文件	这对内核、模块、甚至外部模块同样适用
```

		make -C $KDIR M=$PWD bar.lst
		make -C $KDIR M=$PWD baz.o
		make -C $KDIR M=$PWD foo.ko
		make -C $KDIR M=$PWD ./

```

## 为外部模块创Kbuild 文件

在上一节中，我们看到了为正在运行的内核构建模块的命令。不过模块实际上并未被构建，因为还需要一个构建文件。该文件中将包含被构建模块的名称，以及所需的源文件列表

```

	obj-m := <module_name>.o

```

kbuild 系统会从 <module_name>.c 构建<module_name>.o，并在链接之后生成内核模<module_name>.ko。上面这行可以放"Kbuild" 文件"Makefile" 中。当模块由多个源文件构建时，还需要额外的一
```

	<module_name>-y := <src1>.o <src2>.o ...

```

注意：描kbuild 所用语法的进一步文档位Documentation/kbuild/makefiles.rst
下面的示例演示如何为以下文件创建构建文件

```

	8123_if.c
	8123_if.h
	8123_pci.c

```

### 共享 Makefile

	外部模块始终包含一个包装用makefile，它支持不带参数使用 "make" 来构建模块	这个目标并非kbuild 使用，仅为方便而设。也可以加入额外的功能（例如测试目标），但由于可能存在名称冲突，应当将其kbuild 中过滤掉
```

		--> filename: Makefile
		ifneq ($(KERNELRELEASE),)
		# kbuild part of makefile
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		else
		# normal makefile
		KDIR ?= /lib/modules/`uname -r`/build

		default:
			$(MAKE) -C $(KDIR) M=$$PWD

		endif

	The check for KERNELRELEASE is used to separate the two parts
	of the makefile. In the example, kbuild will only see the two
	assignments, whereas "make" will see everything except these
	two assignments. This is due to two passes made on the file:
	the first pass is by the "make" instance run on the command
	line; the second pass is by the kbuild system, which is
	initiated by the parameterized "make" in the default target.

```

### 独立Kbuild 文件Makefile

	Kbuild 会首先查找名"Kbuild" 的文件，若未找到，则再去查找 "Makefile"。利"Kbuild" 文件，我们可以将示例 1 中的 "Makefile" 拆分为两个文件：

```

		--> filename: Kbuild
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		--> filename: Makefile
		KDIR ?= /lib/modules/`uname -r`/build

		default:
			$(MAKE) -C $(KDIR) M=$$PWD

```

	示例 2 中的拆分由于每个文件都很简单而显得多余；不过，有些外部模块使用的 makefile 长达数百行，在这种情况下，将 kbuild 部分与其余部分分离确实大有裨益
	Linux 6.13 及更高版本支持另一种方式。外部模块的 Makefile 可以直接包含内核 Makefile，而不是调用子 Make
	Example 3::

```

		--> filename: Kbuild
		obj-m  := 8123.o
		8123-y := 8123_if.o 8123_pci.o

		--> filename: Makefile
		KDIR ?= /lib/modules/$(shell uname -r)/build
		export KBUILD_EXTMOD := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))
		include $(KDIR)/Makefile


```

### 构建多个模块

	kbuild 支持用单个构建文件构建多个模块。例如，如果你想构建两个模块 foo.ko

```

		obj-m := foo.o bar.o
		foo-y := <foo_srcs>
		bar-y := <bar_srcs>

	It is that simple!


```

## 包含文件

在内核中，头文件按照以下规则放置在标准位置：

 - 如果头文件只描述某个模块的内部接口，则该文件放在与源文件相同的目录中 - 如果头文件描述内核其他部分（位于不同目录中）所使用的接口，则该文件放在 include/linux/ 中
	  NOTE:
	      该规则有两个显著的例外：较大的子系统include/ 下有自己独立的目录，例如 include/scsi；而特定于架构的头文件位于 arch/$(SRCARCH)/include/ 下
### Kernel Includes

	要包含位include/linux/ 下的头文件，只需

```

		#include <linux/module.h>

	kbuild will add options to the compiler so the relevant directories
	are searched.

```

### Single Subdirectory

	外部模块倾向于把头文件放在其源码所在位置下一个独立的
	include/ 目录中，尽管这并非通常的内核风格。要告知 kbuild 该目录，可使ccflags-y CFLAGS_<filename>.o
	使用3 节的示例，如果我们将 8123_if.h 移动到一个名include 的子目录，则得到kbuild 文件将是

```

		--> filename: Kbuild
		obj-m := 8123.o

		ccflags-y := -I $(src)/include
		8123-y := 8123_if.o 8123_pci.o

```

### Several Subdirectories

	kbuild 可以处理分散在多个目录中的文件
```

		.
		|__ src
		|   |__ complex_main.c
		|   |__ hal
		|	|__ hardwareif.c
		|	|__ include
		|	    |__ hardwareif.h
		|__ include
			|__ complex.h

	To build the module complex.ko, we then need the following
	kbuild file::

		--> filename: Kbuild
		obj-m := complex.o
		complex-y := src/complex_main.o
		complex-y += src/hal/hardwareif.o

		ccflags-y := -I$(src)/include
		ccflags-y += -I$(src)/src/hal/include

	As you can see, kbuild knows how to handle object files located
	in other directories. The trick is to specify the directory
	relative to the kbuild file's location. That being said, this
	is NOT recommended practice.

	For the header files, kbuild must be explicitly told where to
	look. When kbuild executes, the current directory is always the
	root of the kernel tree (the argument to "-C") and therefore an
	absolute path is needed. $(src) provides the absolute path by
	pointing to the directory where the currently executing kbuild
	file is located.


```

## 模块安装

包含在内核中的模块会被安装到以下目录
	/lib/modules/$(KERNELRELEASE)/kernel/

而外部模块会被安装到
	/lib/modules/$(KERNELRELEASE)/updates/

### INSTALL_MOD_PATH

	上面是默认目录，但和通常一样，一定程度的定制是可能的。可以添加一个前缀
```

		$ make INSTALL_MOD_PATH=/frodo modules_install
		=> Install dir: /frodo/lib/modules/$(KERNELRELEASE)/kernel/

	INSTALL_MOD_PATH may be set as an ordinary shell variable or,
	as shown above, can be specified on the command line when
	calling "make." This has effect when installing both in-tree
	and out-of-tree modules.

```

### INSTALL_MOD_DIR

	默认情况下，外部模块被安装到 /lib/modules/$(KERNELRELEASE)/updates/ 下的某个目录中，但你可能希望将特定功能的模块放在一个独立的目录中。为此，可使INSTALL_MOD_DIR 来指定一
```

		$ make INSTALL_MOD_DIR=gandalf -C $KDIR \
		       M=$PWD modules_install
		=> Install dir: /lib/modules/$(KERNELRELEASE)/gandalf/


```

## 模块版本控制

模块版本控制CONFIG_MODVERSIONS 标记启用，用作一种简单的 ABI 一致性检查。会为导出的符号的完整原型创建一CRC 值。当模块被加使用时，内核中包含的 CRC 值会与模块中的类似值进行比较；若不相等，内核将拒绝加载该模块
Module.symvers 包含一个内核构建中所有已导出符号的列表
### 来自内核的符号（vmlinux + 模块
	在内核构建期间，会生成一个名Module.symvers 的文件。Module.symvers 包含内核与已编译模块中所有导出的符号。对于每个符号，其对应的 CRC 值也会被存储
```

		<CRC>       <Symbol>         <Module>                         <Export Type>     <Namespace>

		0xe1cc2a05  usb_stor_suspend drivers/usb/storage/usb-storage  EXPORT_SYMBOL_GPL USB_STORAGE

	The fields are separated by tabs and values may be empty (e.g.
	if no namespace is defined for an exported symbol).

	For a kernel build without CONFIG_MODVERSIONS enabled, the CRC
	would read 0x00000000.

	Module.symvers serves two purposes:

	1) It lists all exported symbols from vmlinux and all modules.
	2) It lists the CRC if CONFIG_MODVERSIONS is enabled.

```

### 版本信息格式

	导出的符号将其信息存储在 __ksymtab __kflagstab 段中。符号名与命名空间存储在 __ksymtab_strings 段中，使用的格式类似ELF 所用的字符串表。若启用CONFIG_MODVERSIONS，与导出符号对应CRC 会被添加__kcrctab 段中
	若启用了 CONFIG_BASIC_MODVERSIONS（CONFIG_MODVERSIONS 默认开启此项），导入符号的符号名与 CRC 会存储在导入模块__versions 段中。该模式仅支持长度不超过 64 字节的符号
	若启用了 CONFIG_EXTENDED_MODVERSIONS（需同时启用 CONFIG_MODVERSIONS CONFIG_RUST），导入符号的符号名会以一系列拼接起来的、以空字符结尾的字符串形式记录在 __version_ext_names 段中。这些符号的 CRC 会记录在 __version_ext_crcs 段中
### 符号与外部模
	构建外部模块时，构建系统需要访问内核中的符号，以检查所有外部符号是否都已定义。这一步在 MODPOST 阶段完成。modpost 通过读取内核源码树中Module.symvers 来获取符号。在 MODPOST 阶段，会写入一个新Module.symvers 文件，其中包含该外部模块导出的所有符号
### 来自另一个外部模块的符号

	有时，一个外部模块会使用另一个外部模块导出的符号。Kbuild 需要完全掌握所有符号，以避免发出关于未定义符号的警告。针对这种情况有两种解决方法
	注意：推荐使用带顶层 kbuild 文件的方法，但在某些情况下可能并不实用
	Use a top-level kbuild file
		如果你有两个模块 foo.ko bar.ko，其foo.ko 需要来bar.ko 的符号，你可以使用一个共用的顶层 kbuild 文件，使两个模块在同一个构建中编译。参考以
```

			./foo/ <= contains foo.ko
			./bar/ <= contains bar.ko

		The top-level kbuild file would then look like::

			#./Kbuild (or ./Makefile):
				obj-m := foo/ bar/

		And executing::

			$ make -C $KDIR M=$PWD

		will then do the expected and compile both modules with
		full knowledge of symbols from either module.

	Use "make" variable KBUILD_EXTRA_SYMBOLS
		If it is impractical to add a top-level kbuild file,
		you can assign a space separated list
		of files to KBUILD_EXTRA_SYMBOLS in your build file.
		These files will be loaded by modpost during the
		initialization of its symbol tables.


```

## 技巧与诀
### 测试 CONFIG_FOO_BAR

	模块常常需要检查某`CONFIG_` 选项，以决定模块是否包含某项特定功能。在 kbuild 中，这可以通过引用 `CONFIG_` 变量来实
```

		#fs/ext2/Makefile
		obj-$(CONFIG_EXT2_FS) += ext2.o

		ext2-y := balloc.o bitmap.o dir.o
		ext2-$(CONFIG_EXT2_FS_XATTR) += xattr.o

```
