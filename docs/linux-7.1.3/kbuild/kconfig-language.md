## Kconfig 语言


### 简介


配置数据库是一系列配置选项的集合
```

	+- Code maturity level options
	|  +- Prompt for development and/or incomplete code/drivers
	+- General setup
	|  +- Networking support
	|  +- System V IPC
	|  +- BSD Process Accounting
	|  +- Sysctl support
	+- Loadable module support
	|  +- Enable loadable module support
	|     +- Set version information on all module symbols
	|     +- Kernel module loader
	+- ...

```
每个条目都有自己的依赖关系。这些依赖关系用于确定条目的可见性。任何子条目只有在父条目也可见时才可见。

### 菜单条目


多数条目定义一个配置选项；其余条目用于组织
```

  config MODVERSIONS
	bool "Set version information on all module symbols"
	depends on MODULES
	help
	  Usually, modules have to be recompiled whenever you switch to a new
	  kernel.  ...

```
每一行都以一个关键字开头，其后可跟多个参数。"config" 开始一个新的配置条目。随后的行定义该配置选项的属性。属性可以是配置选项的类型、输入提示、依赖关系、帮助文本以及默认值。一个配置选项可以使用相同名称多次定义，但每个定义只能有一个输入提示，且类型不得冲突。

### 菜单属性


一个菜单条目可以包含若干属性。并非所有属性在任意位置都适用（参见语法）。

- 类型定义："bool"/"tristate"/"string"/"hex"/"int"

  每个配置选项都必须有一个类型。基本类型只有两种：tristate 和 string；其他类型都基于这两种。类型定义可选择性地接受一个输入提示，因此以下两个示例
```

	bool "Networking support"

  and::

	bool
	prompt "Networking support"

```
- 输入提示："prompt" <prompt> ["if" <expr>]

  每个菜单条目最多只能有一个提示，用于显示给用户。也可以仅针对该提示通过 "if" 添加依赖。若不存在提示，则该配置选项是一个不可见的符号，意味着其值无法由用户直接更改（例如在 `.config` 中修改该值），并且该选项不会出现在任何配置菜单中。其值只能通过 "default" 和 "select" 设置（见下文）。

- 默认值："default" <expr> ["if" <expr>]

  一个配置选项可以有任意数量的默认值。若多个默认值可见，只有第一个被定义的生效。默认值并不局限于定义它的菜单条目。这意味着默认值可以在其他地方定义，或者由更早的定义覆盖。
  只有当用户未设置其他值（通过上面的输入提示）时，默认值才会赋给配置符号。若输入提示可见，则默认值会呈现给用户，并可由用户覆盖。
  也可以仅针对该默认值通过 "if" 添加依赖。

 默认值刻意设为 'n'，以避免构建变得臃肿。除少数例外，新的配置选项不应改变这一点。其意图是让 "make oldconfig" 在不同版本之间尽量少地向配置中新增内容。

 注意：
	符合 "default y/m" 的情况包括：

	a) 某功能过去总是被构建，为其新增的 Kconfig 选项应设为 "default y"。

	b) 一个新的“把关”Kconfig 选项，用于隐藏/显示其他 Kconfig 选项（但其自身不生成任何代码），应设为 "default y"，以便用户能看到那些其他选项。

	c) 对于 "default n" 的驱动，其子驱动行为或类似选项。这允许你提供合理的默认值。

	d) 人人都预期存在的硬件或基础设施，例如 CONFIG_NET 或 CONFIG_BLOCK。这些属于罕见的例外。

```

	"def_bool"/"def_tristate" <expr> ["if" <expr>]

  This is a shorthand notation for a type definition plus a value.
  Optionally dependencies for this default value can be added with "if".

```
- 依赖关系："depends on" <expr> ["if" <expr>]

  这为当前菜单条目定义一个依赖。若定义了多个依赖，则它们以 '&&' 连接。依赖会作用于该菜单条目内的所有其他选项（同样也包括
```

	bool "foo" if BAR
	default y if BAR

  and::

	depends on BAR
	bool "foo"
	default y

  The dependency definition itself may be conditional by appending "if"
  followed by an expression. For example::

    config FOO
	tristate
	depends on BAR if BAZ

  meaning that FOO is constrained by the value of BAR only if BAZ is
  also set.

```
- 反向依赖："select" <symbol> ["if" <expr>]

  普通依赖会降低符号的上限（见下文），而反向依赖可用于强制另一个符号的下限。当前菜单符号的值被用作 <symbol> 可设置的最小值。若 <symbol> 被多次 select，则下限取最大的选择值。
  反向依赖只能用于布尔或三态符号。

  注意：
	select 应谨慎使用。select 会强制将一个符号设为某个值，而不会检查其依赖。
	滥用 select 时，即便 FOO 依赖的 BAR 未被设置，你也能选中符号 FOO。
	一般而言，select 仅用于不可见符号（任何地方都没有提示）以及没有依赖的符号。
	这会降低其可用性，但另一方面可避免到处出现的非法配置。

	若 "select" <symbol> 后跟 "if" <expr>，则 <symbol> 将由当前菜单符号的值与 <expr> 的逻辑与来选中。这意味着，由于存在 "if" <expr>，下限可能被降低。这种行为看似奇怪，但我们有赖于此。（该行为的未来走向尚未确定。）

- 弱反向依赖："imply" <symbol> ["if" <expr>]

  这与 "select" 类似，也会对另一个符号强制一个下限，但区别在于被 "imply" 的符号的值仍可被直接依赖或可见提示设为 n。

```

    config FOO
	tristate "foo"
	imply BAZ

    config BAZ
	tristate "baz"
	depends on BAR

  The following values are possible:

	===		===		=============	==============
	FOO		BAR		BAZ's default	choice for BAZ
	===		===		=============	==============
	n		y		n		N/m/y
	m		y		m		M/y/n
	y		y		y		Y/m/n
	n		m		n		N/m
	m		m		m		M/n
	y		m		m		M/n
	y		n		*		N
	===		===		=============	==============

  This is useful e.g. with multiple drivers that want to indicate their
  ability to hook into a secondary subsystem while allowing the user to
  configure that subsystem out without also having to unset these drivers.

  Note: If the feature provided by BAZ is highly desirable for FOO,
  FOO should imply not only BAZ, but also its dependency BAR::

    config FOO
	tristate "foo"
	imply BAR
	imply BAZ

  Note: If "imply" <symbol> is followed by "if" <expr>, the default of <symbol>
  will be the logical AND of the value of the current menu symbol and <expr>.
  (The future of this behavior is undecided.)

```
- 限制菜单显示："visible if" <expr>

  该属性仅适用于菜单块，若条件为假，则该菜单块不会显示给用户（不过其中包含的符号仍可被其他符号选中）。它类似于针对单个菜单条目的条件式 "prompt" 属性。"visible" 的默认值为真。

- 数值范围："range" <symbol> <symbol> ["if" <expr>]

  这用于限制 int 和 hex 符号可能输入的值的范围。用户只能输入大于等于第一个符号、且小于等于第二个符号的值。

- 帮助文本："help"

  这用于定义帮助文本。帮助文本的结束由缩进层级决定，即遇到第一行缩进小于帮助文本首行的那一行时结束。

- 模块属性："modules"
  这声明该符号用作 MODULES 符号，它为所有配置符号启用第三种模块状态。
  最多只能有一个符号设置 "modules" 选项。

- 过渡属性："transitional"
  这声明该符号为过渡性符号，意味着它应在配置期间被处理，但会被排除在新写入的 .config 文件之外。
  过渡性符号在配置选项迁移过程中对向后兼容很有用——它们允许 olddefconfig 处理已有的 .config 文件，同时确保旧选项不会出现在新配置中。

  过渡性符号：
  - 没有提示（在菜单中对用户不可见）
  - 在配置期间被正常处理（值会被读取和使用）
  - 可被其他符号的默认表达式引用
  - 不会被写入新的 .config 文件
  - 不能拥有任何其他属性（它是一个透传选项）

```

    config NEW_NAME
	bool "New option name"
	default OLD_NAME
	help
	  This replaces the old CONFIG_OLD_NAME option.

    config OLD_NAME
	bool
	transitional
	help
	  Transitional config for OLD_NAME to NEW_NAME migration.

  With this setup, existing .config files with "CONFIG_OLD_NAME=y" will
  result in "CONFIG_NEW_NAME=y" being set, while CONFIG_OLD_NAME will be
  omitted from newly written .config files.

```
### 菜单依赖


依赖关系定义了菜单条目的可见性，也能缩小三态符号的输入范围。表达式中使用的三态逻辑比普通布尔逻辑多一个状态，用以表达
```

  <expr> ::= <symbol>                           (1)
           <symbol> '=' <symbol>                (2)
           <symbol> '!=' <symbol>               (3)
           <symbol1> '<' <symbol2>              (4)
           <symbol1> '>' <symbol2>              (4)
           <symbol1> '<=' <symbol2>             (4)
           <symbol1> '>=' <symbol2>             (4)
           '(' <expr> ')'                       (5)
           '!' <expr>                           (6)
           <expr> '&&' <expr>                   (7)
           <expr> '||' <expr>                   (8)

```
表达式按优先级从高到低列出。

(1) 将符号转换为表达式。布尔和三态符号直接转换为相应的表达式值。所有其他符号类型结果为 'n'。
(2) 若两个符号的值相等，返回 'y'，否则返回 'n'。
(3) 若两个符号的值相等，返回 'n'，否则返回 'y'。
(4) 若 <symbol1> 的值分别小于、大于、小于等于或大于等于 <symbol2> 的值，返回 'y'，否则返回 'n'。
(5) 返回表达式的值。用于覆盖优先级。
(6) 返回 (2-/expr/) 的结果。
(7) 返回 min(/expr/, /expr/) 的结果。
(8) 返回 max(/expr/, /expr/) 的结果。

表达式的值可以是 'n'、'm' 或 'y'（计算时分别对应 0、1、2）。当菜单条目的表达式求值结果为 'm' 或 'y' 时，该条目变为可见。

符号有两种类型：常量符号和非常量符号。
非常量符号最为常见，由 'config' 语句定义。非常量符号完全由字母数字字符或下划线组成。
常量符号仅作为表达式的一部分存在。常量符号始终被单引号或双引号包围。在引号内允许出现任意其他字符，并且可以使用 '\' 对引号进行转义。

### 菜单结构


菜单条目在树中的位置由两种方式决定。首先
```

  menu "Network device support"
	depends on NET

  config NETDEVICES
	...

  endmenu

```
"menu" ... "endmenu" 块内的所有条目都会成为“网络设备支持”的子菜单。所有子条目继承该菜单条目的依赖，例如这意味着依赖 "NET" 会被加入配置选项 NETDEVICES 的依赖列表中。

生成菜单结构的另一种方式是通过分析依赖关系。若某个菜单条目在某种程度上依赖于前一个条目，则可将其设为前者的子菜单。首先，前一个（父级）符号必须是依赖列表的一部分，且以下两个条件之一必须成立：

- 若父级被设为 'n'，子条目必须变为不可见
```

    config MODULES
	bool "Enable loadable module support"

    config MODVERSIONS
	bool "Set version information on all module symbols"
	depends on MODULES

    comment "module support disabled"
	depends on !MODULES

```
MODVERSIONS 直接依赖于 MODULES，这意味着仅当 MODULES 不为 'n' 时才可见。另一方面，该注释仅当 MODULES 设为 'n' 时才可见。


### Kconfig 语法


配置文件描述一系列菜单条目，其中每一行都以关键字开头（帮助文本除外）。以下关键字会结束一个菜单条目：

- config
- menuconfig
- choice/endchoice
- comment
- menu/endmenu
- if/endif
- source

前五个关键字同时也开启一个菜单条目的定义。

```

	"config" <symbol>
	<config options>

```
这定义了一个配置符号 <symbol>，并接受上述任意属性作为选项。

```

	"menuconfig" <symbol>
	<config options>

```
这与上面的简单 config 条目类似，但它向前端给出提示：所有子选项应作为一个独立的选项列表显示。为确保所有子选项确实出现在 menuconfig 条目之下、而非其之外，<config options> 列表中的每一项都必须依赖于该 menuconfig 符号。
```

  (1):
  menuconfig M
  if M
      config C1
      config C2
  endif

  (2):
  menuconfig M
  config C1
      depends on M
  config C2
      depends on M

```
在下面的示例 (3) 和 (4) 中，C1 和 C2 仍然具有 M 依赖，但不再出现在 menuconfig M 之下，因为
```

  (3):
  menuconfig M
      config C0
  if M
      config C1
      config C2
  endif

  (4):
  menuconfig M
  config C0
  config C1
      depends on M
  config C2
      depends on M

```
```

	"choice"
	<choice options>
	<choice block>
	"endchoice"

```
这定义了一个 choice 组，并接受 "prompt"、"default"、"depends on" 和 "help" 属性作为选项。

一个 choice 只允许选中单个配置条目。

```

	"comment" <prompt>
	<comment options>

```
这定义了一个注释，在配置过程中显示给用户，同时也会被回显到输出文件中。唯一可能的选项是依赖。

```

	"menu" <prompt>
	<menu options>
	<menu block>
	"endmenu"

```
这定义了一个菜单块，详见上文“菜单结构”。唯一可能的选项是依赖和 "visible" 属性。

```

	"if" <expr>
	<if block>
	"endif"

```
这定义了一个 if 块。依赖表达式 <expr> 会被追加到所有被包含的菜单条目上。

```

	"source" <prompt>

```
这会读取指定的配置文件。该文件总是被解析。

```

	"mainmenu" <prompt>

```
若配置程序选择使用，这会设置其标题栏。它应放在配置的最顶部、任何其他语句之前。

'#' Kconfig 源文件注释：

在源文件行的任意位置，未加引号的 '#' 字符表示该源文件注释的开始。该行剩余部分即为注释。


### Kconfig 提示

这是一组 Kconfig 技巧，其中大部分乍看并不明显，且多数已成为多个 Kconfig 文件中的惯用法。

#### 添加通用特性并使用法可配置

实现某些特性/功能，这些特性仅与部分而非全部架构相关，这是一种常见惯用法。
推荐的做法是使用一个名为 HAVE_* 的配置变量，它在通用的 Kconfig 文件中定义，并由相关的架构选中。
通用 IOMAP 功能即是一例。

```

  # Generic IOMAP is used to ...
  config HAVE_GENERIC_IOMAP

  config GENERIC_IOMAP
	depends on HAVE_GENERIC_IOMAP && FOO

```
```

	obj-$(CONFIG_GENERIC_IOMAP) += iomap.o

```
```

  config X86
	select ...
	select HAVE_GENERIC_IOMAP
	select ...

```
注意：我们使用已有的配置选项，避免新建一个配置变量来选中 HAVE_GENERIC_IOMAP。

注意：这里使用了内部配置变量 HAVE_GENERIC_IOMAP，引入它是为了克服 select 的限制——select 会无视依赖而将配置选项强制设为 'y'。
依赖被移到了符号 GENERIC_IOMAP 上，从而避免了 select 将某个符号强制设为 'y' 的情况。

#### 添加需要编译器支持的特性


有若干特性需要编译器支持。描述对编译器特性的依赖的推荐方式是使用 "depends on"
```

  config STACKPROTECTOR
	bool "Stack Protector buffer overflow detection"
	depends on $(cc-option,-fstack-protector)
	...

```
若你需要向 makefile 和/或 C 源文件暴露编译器能力，
```

  config CC_HAS_FOO
	def_bool $(success,$(srctree)/scripts/cc-check-foo.sh $(CC))

```

#### 仅作为模块构建

要将某组件的构建限制为仅模块，可对其配置符号限定
```

  config FOO
	depends on BAR && m

```
这将 FOO 限制为模块（=m）或禁用（=n）。

#### 编译测试

若某个配置符号存在依赖，但由该配置符号控制的代码在依赖不满足时仍可编译，则建议通过在依赖中添加 "|| COMPILE_TEST" 子句来提高构建覆盖率。这对于较冷门硬件的驱动尤其有用，因为它允许持续集成系统在更常见的系统上对该代码进行编译测试，从而发现缺陷。
请注意，被编译测试的代码应避免在依赖不满足的系统上运行时崩溃。

#### 架构与平台依赖

由于存在桩函数（stub），现在大多数驱动都可以在大多数架构上编译。然而，这并不意味着在所有地方都提供所有驱动是合理的，因为实际硬件可能只存在于特定的架构和平台上。对于片上（on-SoC）IP 核尤其如此，它们可能仅限于特定的厂商或 SoC 系列。

为避免向用户询问那些无法用于其正在编译内核的目标系统的驱动，在合理的情况下，控制驱动编译的配置符号应包含适当的依赖，将该符号的可见性限制在驱动可运行的平台（的超集）上。依赖可以是一个架构依赖（例如 ARM）或平台依赖（例如 ARCH_OMAP4）。这不仅让发行版配置维护者更轻松，也让每一位配置内核的开发者或用户更轻松。

这种依赖可以通过与上面的编译测试规则结合而放宽，即：

  config FOO
	bool "Support for foo hardware"
	depends on ARCH_FOO_VENDOR || COMPILE_TEST

#### 可选依赖


某些驱动能够选择性地使用来自另一个模块的特性，或在禁用该模块时干净地构建，但在尝试从内建驱动使用该可加载模块时会导致链接失败。

在 Kconfig 逻辑中表达这种可选依赖的推荐方式是
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR if BAR

```
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR || !BAR

```
这意味着要么存在一个对 BAR 的依赖，禁止 FOO=y 与 BAR=m 组合，要么 BAR 被完全禁用。BAR 模块必须为 !BAR 的情况提供所有桩函数。

若存在多个具有此类依赖的驱动，可采用更形式化的方法
```

  config FOO
	tristate "Support for foo hardware"
	depends on BAR_OPTIONAL

  config BAR_OPTIONAL
	def_tristate BAR || !BAR

```
表达可选依赖较不推荐的方式是模块代码中的 IS_REACHABLE()，例如当模块 BAR 不提供
```

	foo_init()
	{
		if (IS_REACHABLE(CONFIG_BAR))
			bar_register(&foo);
		...
	}

```
一般不建议使用 IS_REACHABLE()，因为当 CONFIG_BAR=m 且该代码为内建时，代码会被静默丢弃。这并非用户在将 BAR 启用为模块时通常所期望的。

#### Kconfig 递归依赖的限制


如果你遇到了 Kconfig 错误：“recursive dependency detected”（检测到递归依赖），说明你碰到了 Kconfig 的递归依赖问题，递归依赖可概括为循环依赖。kconfig 工具需要确保 Kconfig 文件符合指定的配置要求。为此，kconfig 必须确定所有 Kconfig 符号可能取到的值，而当两个或多个 Kconfig 符号之间存在循环关系时，目前无法做到这一点。更多细节请参阅下文的“简单 Kconfig 递归问题”小节。Kconfig 不进行递归依赖解析；这对 Kconfig 文件编写者有几个影响。我们将先解释该问题为何存在，然后给出一个由此带给 Kconfig 开发者的技术性限制示例。希望尝试解决此限制的积极开发者应阅读下面的小节。

#### 简单 Kconfig 递归问题


参阅：Documentation/kbuild/Kconfig.recursion-issue-01

```

  make KBUILD_KCONFIG=Documentation/kbuild/Kconfig.recursion-issue-01 allnoconfig

```
#### 累积型 Kconfig 递归问题


参阅：Documentation/kbuild/Kconfig.recursion-issue-02

```

  make KBUILD_KCONFIG=Documentation/kbuild/Kconfig.recursion-issue-02 allnoconfig

```
#### Kconfig 递归问题的实用解决方案


遇到 Kconfig 递归问题的开发者有两个可选方案。我们在下文中记录它们，并提供一个通过这些不同方案解决的历史问题列表。

  a) 移除任何多余的 "select FOO" 或 "depends on FOO"
  b) 匹配依赖语义：

	b1) 将所有 "select FOO" 替换为 "depends on FOO"，或，

	b2) 将所有 "depends on FOO" 替换为 "select FOO"

方案 a) 的解决方式可以用示例 Kconfig 文件 Documentation/kbuild/Kconfig.recursion-issue-01 验证：从 CORE_BELL_A_ADVANCED 中移除 "select CORE"，因为由于 CORE_BELL_A 依赖于 CORE，这已经是隐含的。有时可能无法移除某些依赖条件，这种情况下可使用方案 b)。

方案 b) 的两种不同解决方式可在示例 Kconfig 文件 Documentation/kbuild/Kconfig.recursion-issue-02 中验证。

以下是此前针对此类递归问题的修复示例列表；所有错误似乎都涉及一个或多个 "select" 语句以及一个或多个 "depends on"。

============    ===================================
提交            修复
============    ===================================
06b718c01208    select A -> depends on A
c22eacfe82f9    depends on A -> depends on B
6a91e854442c    select A -> depends on A
118c565a8f2e    select A -> select B
f004e5594705    select A -> depends on A
c7861f37b4c6    depends on A -> (null)
80c69915e5fb    select A -> (null)              (1)
c2218e26c0d0    select A -> depends on A        (1)
d6ae99d04e1c    select A -> depends on A
95ca19cf8cbf    select A -> depends on A
8f057d7bca54    depends on A -> (null)
8f057d7bca54    depends on A -> select A
a0701f04846e    select A -> depends on A
0c8b92f7f259    depends on A -> (null)
e4e9e0540928    select A -> depends on A        (2)
7453ea886e87    depends on A > (null)           (1)
7b1fff7e4fdf    select A -> depends on A
86c747d2a4f0    select A -> depends on A
d9f9ab51e55e    select A -> depends on A
0c51a4d8abd6    depends on A -> select A        (3)
e98062ed6dc4    select A -> depends on A        (3)
91e5d284a7f1    select A -> (null)
============    ===================================

(1) 对错误的部分（或未）引用。
(2) 这似乎是该修复的要点。
(3) 同样的错误。

#### 未来的 kconfig 工作


欢迎在 kconfig 的两个方向上开展工作：厘清语义，以及评估使用完整的 SAT 求解器。完整的 SAT 求解器有助于支持更复杂的依赖映射和/或查询，例如 SAT 求解器的一个可能用途是处理当前已知的递归依赖问题。目前尚不清楚这是否能解决此类问题，但这样的评估是值得的。如果对完整 SAT 求解器的支持被证明过于复杂，或无法解决递归依赖问题，那么 Kconfig 至少应拥有清晰且定义良好的语义，并阐明和记录诸如递归依赖相关的限制或要求。

Kconfig 欢迎在这两个方向上的进一步工作。我们在接下来两个小节中分别详述。

#### Kconfig 的语义


Kconfig 的使用十分广泛，Linux 现在只是 Kconfig 的用户之一：一项研究已完成对 12 个项目中 Kconfig 用法的广泛分析 [^0^]_。
尽管 Kconfig 被广泛使用，且尽管本文档在记录基本 Kconfig 语法方面做得不错，但仍欢迎对 Kconfig 语义给出更精确的定义。有一个项目通过 xconfig 配置器推导出 Kconfig 语义 [^1^]_。应开展工作以确认推导出的语义是否符合我们预期的 Kconfig 设计目标。
另一个项目形式化了 Kconfig 语言核心子集的指称语义 [^10^]_。

拥有定义良好的语义，对用于实际评估依赖的工具很有帮助，例如有一项工作将推导出的 Kconfig 语义用布尔抽象表达，将 Kconfig 逻辑转换为布尔公式并在此上运行 SAT 求解器，以发现死代码/特性（始终不活跃），使用该方法在 Linux 中发现了 114 个死特性 [^1^]_（第 8 节：有效性威胁）。
基于 [^10^]_ 中语义的 kismet 工具，能发现对反向依赖的滥用，并已促成对 Linux Kconfig 文件的几十处已合入修复 [^11^]_。

确认这一点可能很有用，因为 Kconfig 是最重要的工业级变体建模语言之一 [^1^]_ [^2^]_。对其研究有助于评估此类语言的实际用途，过去它们的使用仅停留在理论层面，实际需求并未被充分理解。不过就目前而言，只有逆向工程技术被用于从 Kconfig 等变体建模语言中推导语义 [^3^]_。


#### 用于 Kconfig 的完整 SAT 求解器


尽管 SAT 求解器 [^4^]_ 尚未被 Kconfig 直接使用，但如上一小节所述，已有工作将推导出的 Kconfig 语义用布尔抽象表达，转换为布尔公式并在此上运行 SAT 求解器 [^5^]_。另一个已知的相关项目是 CADOS [^6^]_（前身 VAMOS [^7^]_）及其工具，主要是 undertaker [^8^]_，它最早在 [^9^]_ 中引入。undertaker 的基本思想是从 Kconfig 抽取变体模型，并将其与从 CPP #ifdef 和构建规则中抽取的命题公式一起放入 SAT 求解器，以发现死代码、死文件和死符号。若希望在 Kconfig 上使用 SAT 求解器，一种方法是评估如何将这些工作重新用于 Kconfig。现有项目的导师表现出足够兴趣，不仅愿意就如何将该工作合入上游提供建议，也愿意帮助长期维护它。感兴趣的开发者应访问：

https://kernelnewbies.org/KernelProjects/kconfig-sat
