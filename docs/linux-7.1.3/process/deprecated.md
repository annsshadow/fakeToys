

## 已废弃的接口、语言特性、属性与约定


在理想情况下，可以在一个开发周期内把某个已废弃 API 的所有用法都转换为新
API，并彻底移除旧 API。然而，由于内核规模庞大、维护者层级结构以及时间因素，
这类转换往往无法一次性完成。这意味着在新的实例混入内核的同时，旧的实例尚在
移除过程中，只会使移除该 API 的工作量不断增加。为了让开发者了解哪些内容已被
废弃以及废弃的原因，特此编制本清单，以便在有人提议将废弃内容纳入内核时作为
参考。

### __deprecated

虽然该属性确实能将一个接口标记为已废弃，但它 `在构建期间不再产生任何警告
<https://git.kernel.org/linus/771c035372a036f83353eef46dbb829780330234>`_
，因为内核的一个长期目标是无警告构建，而实际上并没有人在着手移除这些废弃接口。
虽然在头文件中使用 `__deprecated` 来标注旧的 API 不失为一种好的做法，但这并不是
完整的解决方案。这类接口要么必须彻底从内核中移除，要么加入本文件以劝阻他人
将来继续使用它们。

### BUG() 和 BUG_ON()

请改用 WARN() 和 WARN_ON()，并以尽可能优雅的方式处理“不可能发生”的错误
条件。虽然 BUG() 系列 API 最初被设计为充当“不可能发生的情况”断言，并“安全地”
杀死一个内核线程，但事实证明它们风险过高。（例如：“锁需要以何种顺序释放？
各种状态是否已恢复？”）非常常见的情况是，使用 BUG() 会破坏系统的稳定性，
甚至使其完全崩溃，从而导致无法调试，甚至无法获得有效的崩溃报告。Linus 对此
`表达了非常强烈的看法
<https://lore.kernel.org/lkml/CA+55aFy6jNLsywVYdGp83AMrXBo_P-pkjkphPGrO=82SPKCpLQ@mail.gmail.com/>`_
，并 `再次表达了看法
<https://lore.kernel.org/lkml/CAHk-=whDHsbK3HTOpTF=ue_o04onRwTEaK_ZoJp_fjbqq4+=Jw@mail.gmail.com/>`_。

请注意，WARN() 系列只应用于“预期不可达”的情况。如果你想就“可达但不希望
出现”的情况发出警告，请使用 pr_warn() 系列函数。系统所有者可能已经设置了
**panic_on_warn** sysctl，以确保其系统在遇到“不可达”条件时不会继续运行。
（例如，参见类似 `这样的提交
<https://git.kernel.org/linus/d4689846881d160a4d12a514e991a740bcb5d65a>`_。）

### 分配器参数中的开放编码算术运算

动态大小计算（尤其是乘法）不应在内存分配器（或类似）函数参数中执行，因为
存在溢出的风险。这可能导致数值回绕，从而分配出比调用者期望更小的内存。使用
这些分配结果可能导致堆内存的线性溢出以及其他异常行为。（唯一例外是字面量
值，编译器可以在其可能溢出时发出警告。但在这些情况下，首选方式是按照下文
建议重构代码，以避免开放编码的算术运算。）

```

	foo = kmalloc(count * size, GFP_KERNEL);

```
```

	foo = kmalloc_array(count, size, GFP_KERNEL);

```
具体来说，kmalloc() 可替换为 kmalloc_array()，而 kzalloc() 可替换为 kcalloc()。

如果没有两因子形式可用，则应使用溢出饱和辅助函数

```

	bar = dma_alloc_coherent(dev, array_size(count, size), &dma, GFP_KERNEL);

```
另一个应避免的常见情况是通过以下方式计算结构体的大小

```

	header = kzalloc(sizeof(*header) + count * sizeof(*header->item),
			 GFP_KERNEL);

```
```

	header = kzalloc(struct_size(header, item, count), GFP_KERNEL);

```
        或者使用一个单元素数组作为末尾数组成员，请重构此类数组用法并改用
        `柔性数组成员
        <#zero-length-and-one-element-arrays>`_。

对于其他计算，请组合使用 size_mul()、

```

	foo = krealloc(current_size + chunk_size * (count - 3), GFP_KERNEL);

```
```

	foo = krealloc(size_add(current_size,
				size_mul(chunk_size,
					 size_sub(count, 3))), GFP_KERNEL);

```
更多细节，另请参见 array3_size() 和 flex_array_size()，以及相关
的 check_mul_overflow()、check_add_overflow()、check_sub_overflow()、
check_shl_overflow() 系列函数。

### simple_strtol()、simple_strtoll()、simple_strtoul()、simple_strtoull()

simple_strtol()、simple_strtoll()、simple_strtoul() 和 simple_strtoull()
函数会显式忽略溢出，这可能导致调用者得到意外的结果。对应的 kstrtol()、
kstrtoll()、kstrtoul() 和 kstrtoull() 函数通常是正确的替代方案，但请注意，
这些函数要求字符串以 NUL 或换行符结尾。

### strcpy()

strcpy() 不会对目标缓冲区做边界检查。这可能导致缓冲区末尾之后的线性溢出，
从而引发各种异常行为。虽然 `CONFIG_FORTIFY_SOURCE=y` 和各种编译器标志有助于
降低使用此函数的风险，但没有理由再新增对该函数的使用。安全的替代方案是
strscpy()，但必须注意任何使用 strcpy() 返回值的情况，因为 strscpy() 返回的
不是指向目标的指针，而是已复制的非 NUL 字节计数（或在截断时返回负的 errno）。

### 用于 NUL 结尾字符串的 strncpy()

使用 strncpy() 不能保证目标缓冲区会被 NUL 结尾。这可能由于缺失结尾符而导致
各种线性读溢出以及其他异常行为。如果源内容短于目标缓冲区大小，它还会对目标
缓冲区进行 NUL 填充，这对于仅使用 NUL 结尾字符串的调用者来说可能是不必要的
性能损耗。

当目标必须是 NUL 结尾时，替代方案是 strscpy()，但必须注意任何使用 strncpy()
返回值的情况，因为 strscpy() 返回的同样不是指向目标的指针，而是已复制的非 NUL
字节计数（或在截断时返回负的 errno）。任何仍需要 NUL 填充的情况应改用
strscpy_pad()。

如果调用者使用的是非 NUL 结尾的字符串，应使用 strtomem()，并且目标应使用
`__nonstring <https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html>`_
属性进行标记，以避免将来出现编译器警告。对于仍需 NUL 填充的情况，可以使用
strtomem_pad()。

### strlcpy()

strlcpy() 会先读取整个源缓冲区（因为其返回值旨在与 strlen() 的返回值一致）。
该读取可能超出目标大小限制。这既低效，又可能在源字符串不是 NUL 结尾时导致
线性读溢出。安全的替代方案是 strscpy()，但必须注意任何使用 strlcpy() 返回值的
情况，因为 strscpy() 在截断时会返回负的 errno 值。

### %p 格式说明符

传统上，在格式字符串中使用 "%p" 会导致 dmesg、proc、sysfs 等中出现常规的
地址泄露缺陷。与其让这些缺陷可被利用，内核中所有 "%p" 的用法都被打印为哈希后
的值，使其无法用于定位地址。不应再向内核中新增 "%p" 的用法。对于文本地址，
使用 "%pS" 可能更好，因为它输出更有用的符号名。对于几乎其他所有情况，干脆
不要添加 "%p"。

转述 Linus 当前的 `指导原则
<https://lore.kernel.org/lkml/CA+55aFwQEd_d40g4mUCSsVRZzrFPUJt74vc6PPpb675hYNXcKw@mail.gmail.com/>`_：

- 如果哈希后的 "%p" 值毫无意义，请自问该指针本身是否重要。也许它应该被彻底
  移除？
- 如果你确实认为真实的指针值很重要，为何某些系统状态或用户特权级别被视为
  “特殊”？如果你认为你能足够好地在注释和提交日志中证明其合理性、经得起
  Linus 的审视，也许你可以使用 "%px"，同时确保你拥有合理的权限。

如果你正在调试某些因 "%p" 哈希而导致问题的情况，可以临时在启动时加上调试
标志 "`no_hash_pointers
<https://git.kernel.org/linus/5ead723a20e0447bc7db33dc3070b420e5f80aa6>`_"。

### 变长数组（VLA）

使用栈上的 VLA 会比静态大小的栈数组生成差得多的机器码。虽然这些不容忽视的
`性能问题
<https://git.kernel.org/linus/02361bc77888>`_ 本身就足以成为消除 VLA 的理由，
但它们同时也构成安全风险。栈数组的动态增长可能超出栈段中剩余的内存。这可能
导致崩溃、可能覆盖栈末尾的敏感内容（在未使用 `CONFIG_THREAD_INFO_IN_TASK=y`
构建时），或者覆盖栈相邻的内存（在未使用 `CONFIG_VMAP_STACK=y` 构建时）。

### 隐式的 switch case 贯穿（fall-through）

C 语言允许 switch 的 case 在末尾缺少 "break" 语句时贯穿到下一个 case。然而，
这会在代码中引入歧义，因为往往无法明确缺失的 break 是有意为之还是一处 bug。
例如，仅凭查看代码并不能清楚地知道 `STATE_ONE` 是否被有意设计为贯穿

```

	switch (value) {
	case STATE_ONE:
		do_something();
	case STATE_TWO:
		do_other();
		break;
	default:
		WARN("unknown state");
	}

```
由于存在一长串 `因缺失 "break" 语句而导致的缺陷
<https://cwe.mitre.org/data/definitions/484.html>`_，我们不再允许隐式贯穿。为了
识别有意为之的贯穿情况，我们采用了一个伪关键字宏 "fallthrough"，它会展开为
gcc 的扩展 `__attribute__((__fallthrough__))
<https://gcc.gnu.org/onlinedocs/gcc/Statement-Attributes.html>`_。
（当 C17/C18 的 `[[fallthrough]]` 语法被 C 编译器、静态分析器和 IDE 更广泛地
支持时，我们就可以切换为该语法来定义这个宏伪关键字。）

所有 switch/case 块必须以以下之一结尾：

- break;
- fallthrough;
- continue;
- goto <label>;
- return [expression];

### 零长度数组与单元素数组

内核中经常需要一种方式来声明在结构体中拥有动态大小的末尾元素集合。在这些
情况下，内核代码应始终使用 `"柔性数组成员" <https://en.wikipedia.org/wiki/Flexible_array_member>`_。
旧的那种单元素或零长度数组风格不应再被使用。

在较老的 C 代码中，动态大小的末尾元素是这样实现的：通过指定一个单元素数组

```

        struct something {
                size_t count;
                struct foo items[1];
        };

```
这导致了通过 sizeof() 进行脆弱的大小计算（需要减去单个末尾元素的大小才能得到
正确的“头部”大小）。随后 `引入了 GNU C 扩展
<https://gcc.gnu.org/onlinedocs/gcc/Zero-Length.html>`_ 以支持零长度数组，以避免
这类

```

        struct something {
                size_t count;
                struct foo items[0];
        };

```
但这种做法又带来了其他问题，并且没有解决两种风格共有的某些问题，例如无法
检测到此类数组在非结构体末尾处被意外使用的情况（这可能直接发生，也可能在
此类结构体位于联合体、结构体的结构体等中时发生）。

C99 引入了“柔性数组成员”，它不带数字大小

```

        struct something {
                size_t count;
                struct foo items[];
        };

```
这正是内核期望动态大小末尾元素的声明方式。它允许编译器在柔性数组未出现在
结构体末尾时生成错误，从而有助于防止某些 `未定义行为
<https://git.kernel.org/linus/76497732932f15e7323dc805e8ea8dc11bb587cf>`_
缺陷被无意中引入代码库。它还允许编译器正确分析数组大小（通过 sizeof()、
`CONFIG_FORTIFY_SOURCE` 和 `CONFIG_UBSAN_BOUNDS`）。例如，没有任何机制会警告
我们下面这种

```

        struct something {
                size_t count;
                struct foo items[0];
        };

        struct something *instance;

        instance = kmalloc(struct_size(instance, items, count), GFP_KERNEL);
        instance->count = count;

        size = sizeof(instance->items) * instance->count;
        memcpy(instance->items, source, size);

```
在上面的最后一行代码中，`size` 实际上是 `零`，而人们可能以为它代表最近为末尾
数组 `items` 动态分配的内存的总字节大小。以下是该问题的几个示例：`链接 1
<https://git.kernel.org/linus/f2cd32a443da694ac4e28fbf4ac6f9d5cc63a539>`_、
`链接 2
<https://git.kernel.org/linus/ab91c2a89f86be2898cee208d492816ec238b2cf>`_。
相反，`柔性数组成员具有不完整类型，因此 sizeof() 运算符不能作用于它
<https://gcc.gnu.org/onlinedocs/gcc/Zero-Length.html>`_，所以任何对此类运算符的
误用都会在构建时立刻被发现。

关于单元素数组，必须清楚地认识到 `此类数组至少占用该类型单个对象那么大的
空间 <https://gcc.gnu.org/onlinedocs/gcc/Zero-Length.html>`_，因此它们会贡献
封闭结构体的大小。每次人们想计算动态内存的总大小时都容易出错

```

        struct something {
                size_t count;
                struct foo items[1];
        };

        struct something *instance;

        instance = kmalloc(struct_size(instance, items, count - 1), GFP_KERNEL);
        instance->count = count;

        size = sizeof(instance->items) * instance->count;
        memcpy(instance->items, source, size);

```
在上面的示例中，使用 struct_size() 辅助函数时必须记得计算 `count - 1`，否则
我们会无意中为过多的 `items` 对象分配了内存。最简洁且最不易出错的实现方式是
使用 `柔性数组成员`，同时配合

```

        struct something {
                size_t count;
                struct foo items[];
        };

        struct something *instance;

        instance = kmalloc(struct_size(instance, items, count), GFP_KERNEL);
        instance->count = count;

        memcpy(instance->items, source, flex_array_size(instance, items, instance->count));

```
有两个需要使用 DECLARE_FLEX_ARRAY() 辅助函数的特殊替换情况。（注意，在 UAPI
头文件中使用时它被命名为 __DECLARE_FLEX_ARRAY()。）这些情况是：柔性数组要么
单独位于一个结构体中，要么是联合体的一部分。C99 规范禁止这些情况，但并无技术
上的理由（从这两处对此类数组的现有使用以及如下变通方法都可以看出

```

	struct something {
		...
		union {
			struct type1 one[0];
			struct type2 two[0];
		};
	};

```
```

	struct something {
		...
		union {
			DECLARE_FLEX_ARRAY(struct type1, one);
			DECLARE_FLEX_ARRAY(struct type2, two);
		};
	};

```
### 针对结构体对象的开放编码 kmalloc 赋值

执行开放编码的 kmalloc() 系列分配赋值会妨碍内核（以及编译器）检查所赋值变量的
类型，从而限制任何有助于对齐、回绕或额外加固的相关内省。kmalloc_obj() 系列宏
提供了这种内省能力，可用于单对象、数组和柔性对象的常见代码模式

```

	ptr = kmalloc(sizeof(*ptr), gfp);
	ptr = kzalloc(sizeof(*ptr), gfp);
	ptr = kmalloc_array(count, sizeof(*ptr), gfp);
	ptr = kcalloc(count, sizeof(*ptr), gfp);
	ptr = kmalloc(struct_size(ptr, flex_member, count), gfp);
	ptr = kmalloc(sizeof(struct foo, gfp);

```
```

	ptr = kmalloc_obj(*ptr, gfp);
	ptr = kzalloc_obj(*ptr, gfp);
	ptr = kmalloc_objs(*ptr, count, gfp);
	ptr = kzalloc_objs(*ptr, count, gfp);
	ptr = kmalloc_flex(*ptr, flex_member, count, gfp);
	__auto_type ptr = kmalloc_obj(struct foo, gfp);

```
如果 `ptr->flex_member` 被标注了 __counted_by()，则当 `count` 大于与 `flex_member`
关联的计数器成员所能表示的最大值时，该分配会自动失败。
