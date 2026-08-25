## 通用时钟框架（Common Clk Framework
:Author: Mike Turquette <mturquette@ti.com>

本文档力求解释通用时钟（common clk）框架的细节，以及如何将一个平台移植到该框架上它目前还不是include/linux/clk.h 中时API 的详细解释，但也许将来会包含这些信息
## 简介与接口划分

通用时钟框架是一个用于控制当今各种设备上可用时钟节点的接口。它可以表现为时钟门控速率调节、多路复用或其他操作。该框架通过 CONFIG_COMMON_CLK 选项启用
接口本身被分为两半，各自屏蔽了另一半的实现细节。首先是 struct clk 的公共定义，它统一
了框架层面的记账（accounting）与基础设施——这些在传统上被各种不同的平台重复实现。其clk.h API 的公共实现，定义drivers/clk/clk.c。最后是 struct clk_ops，其操作时钟 API 的实现来调用
接口的后半部分由注册struct clk_ops 的硬件相关回调函数，以及为建模特定时钟所需的相硬件相关结构组成。在本文档的余下部分中，任何struct clk_ops 中回调（例如 .enable .set_rate）的引用，都指该代码的硬件相关实现。类似地，对 struct clk_foo 的引用只是对
假想的“foo”硬件的硬件相关部分实现的一种简便简写
将接口的两半联系在一起的struct clk_hw，它定义struct clk_foo 中，并被 struct
clk_core 中的指针所指向。这样可以方便地在通用时钟接口两个独立的一半之间进行导航
## 公共数据结构API

下面是来include/linux/clk-provider.h 的通用 struct clk_core 定义
```
	struct clk_core {
		const char		*name;
		const struct clk_ops	*ops;
		struct clk_hw		*hw;
		struct module		*owner;
		struct clk_core		*parent;
		const char		**parent_names;
		struct clk_core		**parents;
		u8			num_parents;
		u8			new_parent_index;
		...
	};

```
上述成员构成了时钟树拓扑的核心。时API 本身定义了多个面向驱动的函数，这些函数操struct clk。该 API include/linux/clk.h 中有文档说明
使用通用 struct clk_core 的平台和设备，利struct clk_core 中的 struct clk_ops 指针
来执行硬件相关的部分，例如：

```
	struct clk_ops {
		int		(*prepare)(struct clk_hw *hw);
		void		(*unprepare)(struct clk_hw *hw);
		int		(*is_prepared)(struct clk_hw *hw);
		void		(*unprepare_unused)(struct clk_hw *hw);
		int		(*enable)(struct clk_hw *hw);
		void		(*disable)(struct clk_hw *hw);
		int		(*is_enabled)(struct clk_hw *hw);
		void		(*disable_unused)(struct clk_hw *hw);
		unsigned long	(*recalc_rate)(struct clk_hw *hw,
						unsigned long parent_rate);
		int		(*determine_rate)(struct clk_hw *hw,
						  struct clk_rate_request *req);
		int		(*set_parent)(struct clk_hw *hw, u8 index);
		u8		(*get_parent)(struct clk_hw *hw);
		int		(*set_rate)(struct clk_hw *hw,
					    unsigned long rate,
					    unsigned long parent_rate);
		int		(*set_rate_and_parent)(struct clk_hw *hw,
					    unsigned long rate,
					    unsigned long parent_rate,
					    u8 index);
		unsigned long	(*recalc_accuracy)(struct clk_hw *hw,
						unsigned long parent_accuracy);
		int		(*get_phase)(struct clk_hw *hw);
		int		(*set_phase)(struct clk_hw *hw, int degrees);
		void		(*init)(struct clk_hw *hw);
		void		(*debug_init)(struct clk_hw *hw,
					      struct dentry *dentry);
	};

```
## 硬件时钟实现

通用 struct clk_core 的强大之处在于它.ops .hw 指针，它们将 struct clk 的细节与
硬件相关部分相互抽象开来，反之亦然。为了说明这一点，请考虑如下简单的可门控时钟实现：

```
	struct clk_gate {
		struct clk_hw	hw;
		void __iomem    *reg;
		u8              bit_idx;
		...
	};

```
struct clk_gate 包含 struct clk_hw hw，以及关于哪个寄存器和哪一位控制该时钟门控的硬相关知识。这里不需要任何关于时钟拓扑或记账（如 enable_count notifier_count）的信息这些全部由通用框架代码struct clk_core 处理
```
	struct clk *clk;
	clk = clk_get(NULL, "my_gateable_clk");

	clk_prepare(clk);
	clk_enable(clk);

```
```
	clk_enable(clk);
		clk->ops->enable(clk->hw);
		[resolves to...]
			clk_gate_enable(hw);
			[resolves struct clk gate with to_clk_gate(hw)]
				clk_gate_set_bit(gate);

```
```
	static void clk_gate_set_bit(struct clk_gate *gate)
	{
		u32 reg;

		reg = __raw_readl(gate->reg);
		reg |= BIT(gate->bit_idx);
		writel(reg, gate->reg);
	}

```
```
	#define to_clk_gate(_hw) container_of(_hw, struct clk_gate, hw)

```
这种抽象模式被用于每一种时钟硬件的表示
## 支持你自己的时钟硬件

当为新型时钟实现支持时，只需引入
```
	#include <linux/clk-provider.h>

```
要为你的平台构造一个时钟硬件结构，你必须定义：

```
	struct clk_foo {
		struct clk_hw hw;
		... hardware specific data goes here ...
	};

```
为了利用你的数据，你需要支持有效的操作
```
	struct clk_ops clk_foo_ops = {
		.enable		= &clk_foo_enable,
		.disable	= &clk_foo_disable,
	};

```
```
	#define to_clk_foo(_hw) container_of(_hw, struct clk_foo, hw)

	int clk_foo_enable(struct clk_hw *hw)
	{
		struct clk_foo *foo;

		foo = to_clk_foo(hw);

		... perform magic on foo ...

		return 0;
	};

```
下面是一张矩阵，详细说明根据时钟的硬件能力哪clk_ops 是必需的。标记为“y”的单元表示必需；标记为“n”的单元格表示该回调要么无效，要么不需要包含。空白单元格表示可选，
或必须根据具体情况评估
   +----------------+------+-------------+---------------+-------------+------+
   |                | gate | change rate | single parent | multiplexer | root |
   +================+======+=============+===============+=============+======+
   |.prepare        |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.unprepare      |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.enable         | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.disable        | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.is_enabled     | y    |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.recalc_rate    |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.determine_rate |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   |.set_rate       |      | y           |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.set_parent     |      |             | n             | y           | n    |
   +----------------+------+-------------+---------------+-------------+------+
   |.get_parent     |      |             | n             | y           | n    |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.recalc_accuracy|      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+
   +----------------+------+-------------+---------------+-------------+------+
   |.init           |      |             |               |             |      |
   +----------------+------+-------------+---------------+-------------+------+

最后，使用硬件相关的注册函数在运行时注册你的时钟。该函数只是填充 struct clk_foo 数据，然后将通用 struct clk 参数传递给框架，例如：

```
	clk_register(...)

```
相关示例请参`drivers/clk/clk-*.c` 中的基本时钟类型
## 禁止对未使用时钟进行门控

在开发过程中，有时能够绕过默认对未使用时钟的禁用会很有用。例如，如果驱动没有正确启用时钟，而是依赖它们bootloader 起就处于开启状态，那么绕过禁用意味着在该问题解决之前驱动仍能正常工作
你可以通过在内核启动时使用以下参数来查看哪些时钟已被禁用：

```
 tp_printk trace_event=clk:clk_disable

```
要绕过这种禁用，请在传给内核bootargs 中包"clk_ignore_unused"
## 閿。
通用时钟框架使用两把全局锁：prepare 锁和 enable 锁
enable 锁是一把自旋锁，在.enabledisable 操作的调用期间持有。因此这些操作不允许
睡眠，并且对 clk_enable()、clk_disable() API 函数的调用允许在原子上下文中进行
对于 clk_is_enabled() API，它同样被设计为允许在原子上下文中使用。然而，在框架核心中
持有 enable 锁其实并没有太大意义，除非你想在持有该锁的同时利用启用状态的信息做其事情。否则，查看某个时钟是否启用只是对启用状态的一次性读取，而在函数返回后该状态很可能
立刻就会改变（因为锁已被释放）。因此，API 的用户需要自行将该状态的读取与其用途进同步，以确保启用状态在此期间不会发生变化
prepare 锁是一把互斥体（mutex），在对所有其他操作的调用期间持有。所有这些操作都允许
睡眠，因此对相应 API 函数的调用不允许在原子上下文中进行
从加锁的角度看，这实际上将操作分成了两组
驱动不需要手动保护一组操作内部共享的资源，无论这些资源是否被多个时钟共享。然而，对于
被两组操作共享的资源的访问，需要由驱动来保护。此类资源的一个例子是同时控制时钟速率时钟启用/禁用状态的寄存器
时钟框架是可重入的，即驱动允许在其时钟操作的实现内部调用时钟框架函数。例如，这可能导一个时钟的 .set_rate 操作在另一个时钟的 .set_rate 操作内部被调用。驱动实现中必须考虑
这种情况，不过此时的代码流通常由驱动控制
请注意，当通用时钟框架之外的代码需要访问时钟操作所使用的资源时，也必须考虑加锁问题这被视为超出本文档范围