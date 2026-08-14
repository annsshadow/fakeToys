## 符号命名空间


以下文档描述了如何使用符号命名空间（Symbol Namespaces）来组织通过
EXPORT_SYMBOL() 宏系列导出的内核符号的导出界面。

## 简介


符号命名空间被引入，作为组织内核内 API 导出界面的一种手段。它允许子系统维护者将
其导出的符号划分到独立的命名空间中。这对于文档目的（想想 SUBSYSTEM_DEBUG 命名空间）
以及限制一组符号在内核其他部分中的可用性都很有用。截至目前，使用导出到命名空间中的
符号的模块，必须导入该命名空间。否则内核将根据配置拒绝加载模块或警告缺少导入。

此外，还可以将符号放入模块命名空间，严格限制允许使用这些符号的模块。

## 如何定义符号命名空间


可以使用不同的方法将符号导出到命名空间。它们都改变了 EXPORT_SYMBOL 及其同类宏
被插桩以创建 ksymtab 条目的方式。

### 使用 EXPORT_SYMBOL 宏


除了 EXPORT_SYMBOL() 和 EXPORT_SYMBOL_GPL() 这两个允许将内核符号导出到内核符号
表的宏之外，还提供了它们的变体用于将符号导出到特定命名空间：EXPORT_SYMBOL_NS() 和
EXPORT_SYMBOL_NS_GPL()。它们多接受一个参数：作为字符串常量的命名空间。注意此字符串
不能包含空格。
例如，要将符号 `usb_stor_suspend` 导出到
```

	EXPORT_SYMBOL_NS(usb_stor_suspend, "USB_STORAGE");

```
相应的 ksymtab 条目结构体 `kernel_symbol` 将相应地设置其 `namespace` 成员。没有
命名空间导出的符号将引用 `NULL`。如果没有定义，则没有默认命名空间。`modpost` 和
kernel/module/main.c 分别在构建时或模块加载时使用该命名空间。

### 使用 DEFAULT_SYMBOL_NAMESPACE 定义


为子系统的所有符号定义命名空间可能非常冗长，并且难以维护。因此提供了一个默认定义
（DEFAULT_SYMBOL_NAMESPACE），如果设置它，将成为所有未指定命名空间的 EXPORT_SYMBOL()
和 EXPORT_SYMBOL_GPL() 宏展开式的默认值。

有多种方式指定此定义，具体取决于子系统和维护者的偏好。第一种选择是在子系统的
`Makefile` 中定义默认命名空间。例如，要将 usb-common 中定义的所有符号导出到
USB_COMMON 命名空间，添加
```

	ccflags-y += -DDEFAULT_SYMBOL_NAMESPACE='"USB_COMMON"'

```
这将影响所有 EXPORT_SYMBOL() 和 EXPORT_SYMBOL_GPL() 语句。在此定义存在时，使用
EXPORT_SYMBOL_NS() 导出的符号仍将被导出到作为命名空间参数传入的命名空间，因为该
参数优先于默认符号命名空间。

第二种定义默认命名空间的方式是直接在编译单元中，在 <linux/export.h> 的 #include
之前
```

	#define DEFAULT_SYMBOL_NAMESPACE "USB_COMMON"

```
通常在第一个 #include 语句之前放置。

### 使用 EXPORT_SYMBOL_FOR_MODULES() 宏


使用此宏导出的符号被放入模块命名空间。该命名空间无法被导入。这些导出仅限 GPL，因为
它们仅面向树内模块。

该宏接受一个以逗号分隔的模块名列表，仅允许这些模块访问此符号。支持简单的尾部通配符。

```

  EXPORT_SYMBOL_FOR_MODULES(preempt_notifier_inc, "kvm,kvm-*")

```
将把该符号的使用限制为名称匹配给定模式的模块。

## 如何使用命名空间中导出的符号


为了使用导出到命名空间中的符号，内核模块需要显式导入这些命名空间。否则内核可能拒绝
加载该模块。模块代码需要对其使用的符号所在的命名空间使用 MODULE_IMPORT_NS 宏。
例如，使用上面 `usb_stor_suspend` 符号的模块，需要导入 USB_STORAGE 命名空间
```

	MODULE_IMPORT_NS("USB_STORAGE");

```
这将为每个导入的命名空间在模块中创建一个 `modinfo` 标签。这有一个副作用，即模块的
导入命名空间可通过以下方式查看
```

	$ modinfo drivers/usb/storage/ums-karma.ko
	[...]
	import_ns:      USB_STORAGE
	[...]

```
对于当前已加载的模块，导入的命名空间也可通过以下方式查看
```

	$ cat /sys/module/ums_karma/import_ns
	USB_STORAGE

```
建议将 MODULE_IMPORT_NS() 语句放在靠近其他模块元数据定义（如 MODULE_AUTHOR() 或
MODULE_LICENSE()）的位置。

## 加载使用命名空间符号的模块


在模块加载时（例如 `insmod`），内核会检查模块引用的每个符号是否可用，以及它可能
导出到的命名空间是否已被模块导入。内核的默认行为是拒绝加载没有指定足够导入的模块。
将记录一个错误，并以 EINVAL 失败加载。为了允许加载不满足此前置条件的模块，提供了一个
配置选项：设置 MODULE_ALLOW_MISSING_NAMESPACE_IMPORTS=y 将无论如何允许加载，但会
发出警告。

## 自动创建 MODULE_IMPORT_NS 语句


缺失的命名空间导入很容易在构建时被检测到。事实上，如果模块使用了来自某个命名空间的
符号而未导入它，modpost 会发出警告。MODULE_IMPORT_NS() 语句通常会被添加到确定的位置
（与其他模块元数据一起）。为了让模块作者（和子系统维护者）的生活更轻松，提供了一个
脚本和 make 目标来修复
```

	$ make nsdeps

```
```

	- 编写依赖于来自未导入命名空间的符号的代码
	- ``make``
	- 注意 modpost 关于缺少导入的警告
	- 运行 ``make nsdeps`` 将导入添加到正确的代码位置

```
对于引入命名空间的子系统维护者，步骤非常相似。同样，`make nsdeps` 最终会添加缺失的
命名空间导入
```

	- 将符号移动或添加到命名空间（例如使用 EXPORT_SYMBOL_NS()）
	- ``make``（最好使用 allmodconfig 以覆盖所有内核内模块）
	- 注意 modpost 关于缺少导入的警告
	- 运行 ``make nsdeps`` 将导入添加到正确的代码位置

```
```

	$ make -C <path_to_kernel_src> M=$PWD nsdeps

```
注意：它会很乐意地为模块命名空间生成导入语句；但这不会起作用，并会产生构建和运行时
失败。
