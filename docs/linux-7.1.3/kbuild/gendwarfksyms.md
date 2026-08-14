## 基于 DWARF 的模块版本控制


## 简介


当启用 CONFIG_MODVERSIONS 时，模块的符号版本通常使用 **genksyms** 工具从预处理后的源代码计算。然而，这对于 Rust 等语言不兼容，因为源代码缺乏关于最终 ABI 的足够信息。在选择了 CONFIG_GENDWARFKSYMS（以及 CONFIG_DEBUG_INFO）时，改为使用 **gendwarfksyms** 从 DWARF 调试信息计算符号版本，其中包含了关于最终模块 ABI 的必要细节。

### 依赖


gendwarfksyms 依赖于 libelf、libdw 与 zlib 库。

以下是安装这些依赖的几个示例：

```

	sudo pacman --needed -S libelf zlib

```
```

	sudo apt install libelf-dev libdw-dev zlib1g-dev

```
```

	sudo dnf install elfutils-libelf-devel elfutils-devel zlib-devel

```
```

	sudo zypper install libelf-devel libdw-devel zlib-devel

```
### 用法


gendwarfksyms 在命令行接受一个目标文件列表，用法如下：

```

	Usage: gendwarfksyms [options] elf-object-file ... < symbol-list

	Options:
	  -d, --debug          Print debugging information
	      --dump-dies      Dump DWARF DIE contents
	      --dump-die-map   Print debugging information about die_map changes
	      --dump-types     Dump type strings
	      --dump-versions  Dump expanded type strings used for symbol versions
	  -s, --stable         Support kABI stability features
	  -T, --symtypes file  Write a symtypes file
	  -h, --help           Print this message


```
## 类型信息的可用性


虽然符号通常在定义它们的同一翻译单元（TU）中导出，但 TU 导出外部符号也完全没问题。例如，在为独立汇编代码中的导出计算符号版本时就是这样做的。

为确保编译器在符号实际导出的 TU 中发出必要的 DWARF 类型信息，gendwarfksyms 使用以下方式在 `EXPORT_SYMBOL()` 宏中添加一个指向被导出符号的指针：

```

	#define __GENDWARFKSYMS_EXPORT(sym)				\
		static typeof(sym) *__gendwarfksyms_ptr_##sym __used	\
			__section(".discard.gendwarfksyms") = &sym;


```
当在 DWARF 中发现符号指针时，即使符号定义在其他地方，gendwarfksyms 也能使用其类型来计算符号版本。符号指针的名称应以 `__gendwarfksyms_ptr_` 开头，后跟被导出符号的名称。

## Symtypes 输出格式


与 genksyms 类似，gendwarfksyms 支持为每个被处理的目标写入一个 symtypes 文件，其中包含导出符号的类型以及计算符号版本时所用到的每个被引用类型。这些文件在试图确定构建之间符号版本变化的具体原因时很有用。要在内核构建期间生成 symtypes 文件，请设置 `KBUILD_SYMTYPES=1`。

与现有格式一致，每行的第一列包含类型引用或符号名。类型引用有一个单字母前缀，后跟 "#" 和类型名。共有四种类型：

```

	e#<type> = enum
	s#<type> = struct
	t#<type> = typedef
	u#<type> = union


```
```

	s#'core::result::Result<u8, core::num::error::ParseIntError>'

```
该行其余部分包含一个类型字符串。与生成 C 风格类型字符串的 genksyms 不同，gendwarfksyms 使用 **--dump-dies** 生成的相同简单解析 DWARF 格式，但使用类型引用而非完全展开的字符串。

## 维护稳定的 kABI


由于 LTS 更新或向后移植，发行版维护者常常需要能够对内核数据结构做出 ABI 兼容的修改。使用传统的 `#ifndef __GENKSYMS__` 来向符号版本控制隐藏这些修改，在处理目标文件时不起作用。为支持此用例，gendwarfksyms 提供了 kABI 稳定性特性，用于在计算版本时隐藏那些不会影响 ABI 的修改。这些特性都受 **--stable** 命令行标志控制，且不在主线内核中使用。要在内核构建期间使用稳定特性，请设置 `KBUILD_GENDWARFKSYMS_STABLE=1`。

使用这些特性的示例在 **scripts/gendwarfksyms/examples** 目录中提供，包括用于源代码标注的辅助宏。请注意，由于这些特性仅用于转换符号版本控制的输入，用户有责任确保其修改实际上不会破坏 ABI。

### kABI 规则


kABI 规则允许发行版微调 gendwarfksyms 输出的某些部分，从而控制符号版本的计算方式。这些规则定义在该目标文件的 `.discard.gendwarfksyms.kabi_rules` 节中，形式为以下以 NUL 结尾字段组成的字符串序列：

```

	version\0type\0target\0value\0

```
该字符串序列按需重复多次以表达所有规则。各字段如下：

- `version`：确保对未来结构修改的向后兼容性。当前预期为 "1"。
- `type`：指示所应用规则的类型。
- `target`：指定规则的目标，通常是 DWARF 调试信息条目（DIE）的完全限定名。
- `value`：提供规则特定的数据。

例如，以下辅助宏可用于指定规则：

```

	#define ___KABI_RULE(hint, target, value)                            \
		static const char __PASTE(__gendwarfksyms_rule_,             \
					  __COUNTER__)[] __used __aligned(1) \
			__section(".discard.gendwarfksyms.kabi_rules") =     \
				"1\0" #hint "\0" target "\0" value

	#define __KABI_RULE(hint, target, value) \
		___KABI_RULE(hint, #target, #value)


```
目前仅支持本节讨论的规则，但该格式具有足够的扩展性，可在需要时添加更多规则。

#### 管理定义可见性


当额外的 include 被引入翻译单元时，声明可能变成完整定义。这会改变任何引用该类型的符号的版本，即使 ABI 未改变。由于不破坏构建可能无法去掉 include，因此可以使用 `declonly` 规则将一个类型指定为仅声明，即使调试信息包含完整定义。

规则字段预期如下：

- `type`："declonly"
- `target`：目标数据结构的完全限定名（如 **--dump-dies** 输出所示）。
- `value`：此字段被忽略。

```

	#define KABI_DECLONLY(fqn) __KABI_RULE(declonly, fqn, )

```
```

	struct s {
		/* definition */
	};

	KABI_DECLONLY(s);

```
#### 添加枚举器


对于枚举，所有枚举器及其值都被纳入符号版本的计算，如果之后需要在不改变符号版本的情况下添加更多枚举器，这就会成为问题。`enumerator_ignore` 规则允许我们从输入中隐藏具名枚举器。

规则字段预期如下：

- `type`："enumerator_ignore"
- `target`：目标枚举的完全限定名（如 **--dump-dies** 输出所示）与枚举器字段名，以空格分隔。
- `value`：此字段被忽略。

```

	#define KABI_ENUMERATOR_IGNORE(fqn, field) \
		__KABI_RULE(enumerator_ignore, fqn field, )

```
```

	enum e {
		A, B, C, D,
	};

	KABI_ENUMERATOR_IGNORE(e, B);
	KABI_ENUMERATOR_IGNORE(e, C);

```
如果枚举还包含一个结束标记，且必须在中间添加新值，我们在计算版本时可能需要为最后一个枚举器使用旧值。`enumerator_value` 规则允许我们为版本计算覆盖枚举器的值：

- `type`："enumerator_value"
- `target`：目标枚举的完全限定名（如 **--dump-dies** 输出所示）与枚举器字段名，以空格分隔。
- `value`：用于该字段的整数值。

```

	#define KABI_ENUMERATOR_VALUE(fqn, field, value) \
		__KABI_RULE(enumerator_value, fqn field, value)

```
```

	enum e {
		A, B, C, LAST,
	};

	KABI_ENUMERATOR_IGNORE(e, C);
	KABI_ENUMERATOR_VALUE(e, LAST, 2);

```
#### 管理结构体大小变化


如果数据结构的内存分配由核心内核处理，而模块只需访问其中部分成员，那么该数据结构对模块可以是部分不透明的。在这种情况下，只要原有成员的布局保持不变，就可以向结构体中追加新成员而不破坏 ABI。

要追加新成员，我们可以按照“隐藏成员 <hiding_members>”一节所述将其从符号版本控制中隐藏，但我们无法隐藏结构体大小的增加。`byte_size` 规则允许我们覆盖用于符号版本控制的结构体大小。

规则字段预期如下：

- `type`："byte_size"
- `target`：目标数据结构的完全限定名（如 **--dump-dies** 输出所示）。
- `value`：指示结构体大小（字节）的正十进制数。

```

	#define KABI_BYTE_SIZE(fqn, value) \
		__KABI_RULE(byte_size, fqn, value)

```
```

	struct s {
		/* Unchanged original members */
		unsigned long a;
		void *p;

		/* Appended new members */
		KABI_IGNORE(0, unsigned long n);
	};

	KABI_BYTE_SIZE(s, 16);

```
#### 覆盖类型字符串


在极少数情况下，发行版必须对那些无意中被包含在已发布 ABI 中的、本应不透明的数据结构做出重大修改，此时使用更针对性的 kABI 规则来保持符号版本稳定会变得繁琐。`type_string` 规则允许我们覆盖类型或符号的完整类型字符串，甚至添加内核中已不再存在的、用于版本控制的类型。

规则字段预期如下：

- `type`："type_string"
- `target`：目标数据结构的完全限定名（如 **--dump-dies** 输出所示）或符号。
- `value`：一个有效的类型字符串（如 **--symtypes** 输出所示），用于替代真实类型。

```

	#define KABI_TYPE_STRING(type, str) \
		___KABI_RULE("type_string", type, str)

```
```

	/* Override type for a structure */
	KABI_TYPE_STRING("s#s",
		"structure_type s { "
			"member base_type int byte_size(4) "
				"encoding(5) n "
			"data_member_location(0) "
		"} byte_size(8)");

	/* Override type for a symbol */
	KABI_TYPE_STRING("my_symbol", "variable s#s");

```
`type_string` 规则应仅在其他手段无法合理维持稳定符号版本时才作为最后手段使用。覆盖类型字符串会增加实际 ABI 破坏被忽略的风险，因为它隐藏了对该类型的所有修改。

### 添加结构体成员


也许最常见的 ABI 兼容修改是向内核数据结构添加成员。当预期结构体会被修改时，发行版维护者可以预先在结构中保留空间，并在之后使用它而不破坏 ABI。如果需要对没有保留空间的数据结构进行修改，也可以改用已有的对齐空隙。虽然可以为这类修改添加 kABI 规则，但使用联合体通常是更自然的方法。本节描述 gendwarfksyms 对使用数据结构中的保留空间、以及隐藏在计算符号版本时不会改变 ABI 的成员的支持。

#### 预留空间与替换成员


空间通常通过在数据结构末尾追加整数类型或数组来为以后使用而预留，但任何类型都可以使用。每个保留成员需要唯一名称，但由于预留空间时通常不知道其实际用途，为方便起见，通常采用的命名如下：

```

	struct s {
		long a;
		long __kabi_reserved_0; /* reserved for future use */
	};

```
可以通过将成员包装在一个联合体中来使用预留空间：

```

	struct s {
		long a;
		union {
			long __kabi_reserved_0; /* original type */
			struct b b; /* replaced field */
		};
	};

```
如果在预留空间时使用了 `__kabi_` 命名方案，则联合体第一个成员的名称必须以 `__kabi_reserved` 开头。这确保在计算版本时使用原始类型，但名称再次被忽略。联合体的其余部分被忽略。

如果我们要替换的成员不遵循此命名约定，我们还需要保留原始名称以避免改变版本，为此可将联合体第一个成员的名称改为以 `__kabi_renamed` 开头，后跟原始名称。

示例中包含 `KABI_(RESERVE|USE|REPLACE)*` 宏，可帮助简化此过程，并确保替换成员正确对齐且其大小不会超过预留空间。


#### 隐藏成员


预测在支持周期内哪些结构需要修改并非总是可行，在这种情况下可能不得不求助于已有的对齐空隙。例如：

```

	struct s {
		int a;
		/* a 4-byte alignment hole */
		unsigned long b;
	};


```
虽然这不会改变数据结构的大小，但需要能够将添加的成员从符号版本控制中隐藏。与保留字段类似，这可以通过将添加的成员包装到一个联合体中实现，其中某个字段的名称以 `__kabi_` 开头：

```

	struct s {
		int a;
		union {
			char __kabi_ignored_0;
			int n;
		};
		unsigned long b;
	};

```
使用 **--stable** 时，两个版本产生相同的符号版本。示例中包含 `KABI_IGNORE` 宏以简化代码。
