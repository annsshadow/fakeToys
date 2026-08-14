
## Linux 内核编码风格


这是一篇描述 Linux 内核首选编码风格的简短文档。编码风格是非常个人化的，我不会
**强制**任何人接受我的观点，但这是任何我必须能够维护的东西所应当遵循的，而且对于
大多数其他东西我也更喜欢这样。请至少考虑一下这里提出的要点。

首先，我建议打印一份 GNU 编码标准，然后**不要**读它。把它们烧掉，这是一个很好的
象征性姿态。

总之，开始吧：

### 1) 缩进


制表符（Tab）是 8 个字符，因此缩进也是 8 个字符。有一些异端运动试图让缩进变成
4（甚至 2！）个字符深，这无异于试图把 PI 的值定义为 3。

理由：缩进背后的整个想法是清晰地定义一块控制代码从哪里开始、到哪里结束。特别是
当你已经盯着屏幕连续看了 20 个小时的时候，如果缩进足够大，你会发现更容易看清
缩进是如何工作的。

现在，有些人会声称 8 字符缩进让代码向右移动得太远，并且在 80 列的终端屏幕上难以
阅读。对此的回答是，如果你需要超过 3 层的缩进，那你无论如何都搞砸了，应该修复
你的程序。

简而言之，8 字符缩进让东西更易读，并且有一个额外的好处：当你的函数嵌套得太深时
警告你。留意那个警告。

在 switch 语句中缓解多层缩进的首选方式，是将 `switch` 和它的从属 `case` 标签对齐
到同一列，而不是对 `case` 标签进行 `双重缩进`。例如：


	switch (suffix) {
	case 'G':
	case 'g':
		mem <<= 30;
		break;
	case 'M':
	case 'm':
		mem <<= 20;
		break;
	case 'K':
	case 'k':
		mem <<= 10;
		fallthrough;
	default:
		break;
	}

除非你有东西要隐藏，否则不要把多个语句放在同一行上：


	if (condition) do_this;
	  do_something_everytime;

不要使用逗号来避免使用花括号：


	if (condition)
		do_this(), do_that();

对于多个语句，总是使用花括号：


	if (condition) {
		do_this();
		do_that();
	}

也不要把多个赋值放在同一行上。内核编码风格超级简单。避免复杂的表达式。

在注释、文档之外，并且除了在 Kconfig 中，空格从不用于缩进，上面的例子是故意
写错的。

使用一个像样的编辑器，并且不要留下行尾空白。

### 2) 断行与断字符串


编码风格全在于使用常见可用工具的可读性和可维护性。

单行长度的首选限制是 80 列。

超过 80 列的陈述应该被拆分成合理的块，除非超过 80 列能显著提高可读性并且不隐藏
信息。

后代行总是明显比父行短，并且明显放在右侧。一个非常常用的风格是把后代行对齐到
函数的左圆括号。

同样的规则也适用于带有长参数列表的函数头。

然而，永远不要打断用户可见的字符串，例如 printk 消息，因为那会破坏对它们进行
grep 的能力。

### 3) 花括号与空格的位置


C 语言风格中另一个总是出现的问题，是花括号的放置。与缩进大小不同，选择一个放置
策略而不是另一个几乎没有什么技术上的理由，但先知 Kernighan 和 Ritchie 展示给我们
的首选方式是：把左花括号放在行末，把右花括号放在行首，如下：


	if (x is true) {
		we do y
	}

这适用于所有非函数的语句块（if、switch、for、while、do）。例如：


	switch (action) {
	case KOBJ_ADD:
		return "add";
	case KOBJ_REMOVE:
		return "remove";
	case KOBJ_CHANGE:
		return "change";
	default:
		return NULL;
	}

然而，有一个特例，即函数：它们的左花括号在下一行的开头，因此：


	int function(int x)
	{
		body of function
	}

全世界的异端人士都声称这种不一致是……嗯……不一致的，但所有思想正确的人都知道
(a) K&R 是**对的**，并且 (b) K&R 是对的。此外，函数本来就是特殊的（你在 C 中
不能嵌套它们）。

注意，右花括号独占一行、为空，**除非**它后面跟着同一语句的延续，即 do 语句中的
`while`，或 if 语句中的 `else`，像这样：


	do {
		body of do-loop
	} while (condition);

和


	if (x == y) {
		..
	} else if (x > y) {
		...
	} else {
		....
	}

理由：K&R。

另外，请注意这种花括号放置方式也最小化了空（或几乎空）行的数量，而不会损失任何
可读性。因此，由于你屏幕上的新行供应不是可再生资源（想想 25 行的终端屏幕），你
就有更多的空行用来放注释。

当单个语句就足够时，不要不必要地使用花括号。


	if (condition)
		action();

和


	if (condition)
		do_this();
	else
		do_that();

如果条件语句只有一个分支是单个语句，这不适用；在后一种情况下，两个分支都使用
花括号：


	if (condition) {
		do_this();
		do_that();
	} else {
		otherwise();
	}

另外，当循环包含多个简单语句时，使用花括号：


	while (condition) {
		if (test)
			do_something();
	}

######## 3.1) 空格


Linux 内核关于空格使用的风格取决于（主要）函数与关键字的使用。在（大多数）关键字
后面使用空格。显著的例外是 sizeof、typeof、alignof 和 __attribute__，它们看起来
有点像函数（并且在 Linux 中通常与括号一起使用，尽管语言并不要求，例如：在声明
`struct fileinfo info;` 之后，`sizeof info`）。

```

	if, switch, case, for, do, while

```
但不要与 sizeof、typeof、alignof 或 __attribute__ 一起。例如，


	s = sizeof(struct file);

不要在带括号的表达式周围（内部）添加空格。这个例子是**坏的**：


	s = sizeof( struct file );

当声明指针数据或返回指针类型的函数时，`*` 的首选用法是紧贴数据名或函数名，而不是
紧贴类型名。例如：


	char *linux_banner;
	unsigned long long memparse(char *ptr, char **retptr);
	char **match_strdup(substring_t **s);

在大多数二元和三元运算符周围使用一个空格，
```

	=  +  -  <  >  *  /  %  |  &  ^  <=  >=  ==  !=  ?  :

```
```

	&  *  +  -  ~  !  sizeof  typeof  alignof  __attribute__  defined

```
```

	++  --

```
```

	++  --

```
而在 `.` 和 `->` 结构体成员运算符周围不加空格。

不要让行尾留下尾随空白。一些带有 `smart` 缩进的编辑器会在新行开头适当地插入空白，
这样你就可以马上开始输入下一行代码。然而，某些这样的编辑器如果你最终没有在那里
放一行代码（例如你留了一个空行），并不会移除那些空白。结果就是，你最终得到了包含
尾随空白的行。

Git 会警告你那些引入尾随空白的补丁，并且可以选择为你去掉尾随空白；然而，如果应用
一系列补丁，这可能会通过改变它们的上下文行而导致该系列中后面的补丁失败。

### 4) 命名


C 是一种简朴的语言，你的命名约定也应该与之相称。与 Modula-2 和 Pascal 程序员不同，
C 程序员不使用像 ThisVariableIsATemporaryCounter 这样可爱的名字。C 程序员会把它叫做
`tmp`，这写起来容易得多，而且理解起来一点也不更难。

然而，虽然混合大小写的名字不受待见，但全局变量必须有描述性的名字。把一个全局函数
叫做 `foo` 是一种该打的行为。

全局变量（仅当你**真的**需要时才使用）需要有描述性的名字，全局函数也一样。如果你
有一个统计活跃用户数量的函数，你应该叫它 `count_active_users()` 或类似的名字，你
**不应该**叫它 `cntusr()`。

把函数的类型编码进名字（所谓的匈牙利命名法）是愚蠢的——编译器反正知道类型并且
可以检查它们，这只会让程序员困惑。

局部变量名应该简短，并且切中要害。如果你有一个随机的整数循环计数器，它大概应该
叫做 `i`。叫它 `loop_counter` 是低效的，只要没有可能被误解的机会。类似地，`tmp`
几乎可以是任何用来保存临时值的变量类型。

如果你害怕混淆你的局部变量名，那你还有另一个问题，叫做函数生长激素失衡综合症。
参见第 6 章（函数）。

对于符号名和文档，避免引入 'master / slave'（或独立于 'master' 的 'slave'）以及
'blacklist / whitelist' 的新用法。

'master / slave' 的推荐替换是：
    '{primary,main} / {secondary,replica,subordinate}'
    '{initiator,requester} / {target,responder}'
    '{controller,host} / {device,worker,proxy}'
    'leader / follower'
    'director / performer'

'blacklist/whitelist' 的推荐替换是：
    'denylist / allowlist'
    'blocklist / passlist'

引入新用法的例外情况是为了维持一个用户空间 ABI/API，或者当为现有的（截至 2020 年）
硬件或协议规范更新代码、而该规范强制要求那些术语时。对于新规范，在可能的情况下将
规范中对术语的用法翻译为内核编码标准。

### 5) Typedefs


请不要使用像 `vps_t` 这样的东西。对结构体和指针使用 typedef 是一个**错误**。当你
在源代码中看到


	vps_t a;

这是什么意思？相反，如果它写的是


	struct virtual_container *a;

你实际上能说出 `a` 是什么。

许多人认为 typedef `有助于可读性`。并非如此。它们只在以下情况下有用：

 (a) 完全不透明的对象（typedef 被积极地用来**隐藏**对象是什么）。

     例子：`pte_t` 等不透明对象，你只能使用适当的访问器函数来访问它们。

```

       Opaqueness and ``accessor functions`` are not good in themselves.
       The reason we have them for things like pte_t etc. is that there
       really is absolutely **zero** portably accessible information there.

 (b) 清晰的整数类型，其中抽象**有助于**避免混淆它究竟是 ``int`` 还是 ``long``。

     u8/u16/u32 是非常好的 typedef，尽管它们更适合归入类别 (d) 而不是这里。

     .. note::

       Again - there needs to be a **reason** for this. If something is
       ``unsigned long``, then there's no reason to do

	typedef unsigned long myflags_t;

     but if there is a clear reason for why it under certain circumstances
     might be an ``unsigned int`` and under other configurations might be
     ``unsigned long``, then by all means go ahead and use a typedef.

 (c) 当你使用 sparse 从字面上创建一个**新**类型用于类型检查。

 (d) 与标准 C99 类型完全相同的、在特定例外情况下的新类型。

     尽管眼睛和大脑只需很短的时间就能习惯于像 ``uint32_t`` 这样的标准类型，但
     有些人仍然反对使用它们。

     因此，与标准类型完全相同的 Linux 特有的 ``u8/u16/u32/u64`` 类型及其有符号
     等价类型是允许的——尽管在你自己的新代码中它们不是强制的。

     当编辑已经使用其中一套或另一套类型的现有代码时，你应该遵循该代码中已有的
     选择。

 (e) 适合在用户空间使用的安全类型。

     在某些对用户空间可见的结构体中，我们不能要求 C99 类型，也不能使用上面的
     ``u32`` 形式。因此，在所有与用户空间共享的结构体中，我们使用 __u32 和类似
     的类型。

```
也许还有其他情况，但规则基本上应该是：除非你能清楚地匹配上述某条规则，否则永远
不要使用 typedef。

一般来说，一个指针，或者一个其元素可以被合理地直接访问的结构体，应该**永远**不是
typedef。

### 6) 函数


函数应该短小精悍，并且只做一件事。它们应该能放在一两屏文本内（ISO/ANSI 屏幕大小是
80x24，众所周知），做一件事，并且把它做好。

函数的最大长度与该函数的复杂度和缩进层级成反比。所以，如果你有一个概念上简单、只是
一个很长（但简单）的 case 语句的函数，你需要为许多不同的情况做许多小事情，那么
有一个较长的函数是可以的。

然而，如果你有一个复杂的函数，并且你怀疑一个资质平庸的高一学生可能连这个函数到底
是干什么的都不理解，你就更应该严格遵守最大限制。使用带有描述性名字的辅助函数（如果
你认为它性能关键，你可以让编译器把它们内联，而且它可能会比你做得更好）。

衡量函数的另一个标准是局部变量的数量。它们不应该超过 5-10 个，否则你就是做错了
什么。重新思考这个函数，把它拆成更小的块。人脑通常能轻松跟踪大约 7 件不同的事情，
再多就会困惑。你知道你很聪明，但也许你愿意理解你两周后做了什么。

在源文件中，用一空行分隔函数。如果函数被导出，它的 **EXPORT** 宏应该紧接在函数的
右花括号行之后。例如：


	int system_is_up(void)
	{
		return system_state == SYSTEM_RUNNING;
	}
	EXPORT_SYMBOL(system_is_up);

######## 6.1) 函数原型


在函数原型中，包含参数名及其数据类型。虽然 C 语言不要求这样，但 Linux 中更偏好这样，
因为这是一种为读者添加有价值信息的简单方式。

不要对函数声明使用 `extern` 关键字，因为这会让行变长，而且并非严格必要。

写函数原型时，请保持 `order of elements regular <https://lore.kernel.org/mm-commits/CAHk-=wiOCLRny5aifWNhr621kYrJwhfURsa0vFPeUEm8mF0ufg@mail.gmail.com/>`_。
```

 __init void * __must_check action(enum magic value, size_t size, u8 count,
				   char *fmt, ...) __printf(4, 5) __malloc;

```
函数原型的元素首选顺序是：

- 存储类（下面，`static __always_inline`，注意 `__always_inline` 在技术上是一个
  属性，但被当作 `inline` 对待）
- 存储类属性（这里，`__init`——即段声明，但也包括像 `__cold` 这样的东西）
- 返回类型（这里，`void *`）
- 返回类型属性（这里，`__must_check`）
- 函数名（这里，`action`）
- 函数参数（这里，`(enum magic value, size_t size, u8 count, char *fmt, ...)`，
  注意参数名应该总是包含在内）
- 函数参数属性（这里，`__printf(4, 5)`）
- 函数行为属性（这里，`__malloc`）

注意，对于函数的**定义**（即实际的函数体），编译器不允许在函数参数之后有函数参数
属性。在这种情况下，它们应该放在存储类属性之后（例如，注意 `__printf(4, 5)` 位置
的变化）
```

 static __always_inline __init __printf(4, 5) void * __must_check action(enum magic value,
		size_t size, u8 count, char *fmt, ...) __malloc
 {
	...
 }

```
### 7) 函数的集中式退出


尽管被某些人反对，但 goto 语句的等价物被编译器以无条件跳转指令的形式频繁使用。

当一个函数从多个位置退出，并且需要做一些公共工作（例如清理）时，goto 语句就派上
用场了。如果不需要清理，就直接返回。

选择能说明 goto 做什么或为何存在的标签名。一个好的名字的例子可以是 `out_free_buffer:`，
如果该 goto 释放 `buffer`。避免使用 GW-BASIC 风格的名字如 `err1:` 和 `err2:`，因为
如果你曾经添加或移除退出路径，你将不得不重新编号它们，而且它们无论如何都让正确性
难以验证。

使用 goto 的理由是：

- 无条件语句更容易理解和跟踪
- 减少了嵌套
- 防止了在修改时因没有更新各个退出点而产生的错误
- 节省了编译器优化掉冗余代码的工作 ;)

	int fun(int a)
	{
		int result = 0;
		char *buffer;

		buffer = kmalloc(SIZE, GFP_KERNEL);
		if (!buffer)
			return -ENOMEM;

		if (condition1) {
			while (loop1) {
				...
			}
			result = 1;
			goto out_free_buffer;
		}
		...
		out_free_buffer:
		kfree(buffer);
		return result;
	}

一个需要注意的常见类型缺陷是 `one err bugs`，它看起来像这样：


	err:
		kfree(foo->bar);
		kfree(foo);
		return ret;

这段代码中的缺陷是，在某些退出路径上 `foo` 是 NULL。通常对此的修复是把它拆分成两个
错误标签 `err_free_bar:` 和 `err_free_foo:`：


	err_free_bar:
		kfree(foo->bar);
	err_free_foo:
		kfree(foo);
		return ret;

理想情况下，你应该模拟错误来测试所有退出路径。

### 8) 注释


注释是好的，但过度注释也有危险。永远不要试图在注释中解释你的代码**如何**工作：把
代码写得让**工作**方式显而易见要好得多，解释写得糟糕的代码是在浪费时间。

一般来说，你希望你的注释告诉别人你的代码**做什么**，而不是**怎么做**。另外，尽量
避免把注释放在函数体内：如果函数复杂到你需要单独注释它的各个部分，你可能应该回到
第 6 章待一会儿。你可以写一些小注释来指出或警告某些特别巧妙（或特别丑陋）的地方，
但要避免过度。相反，把注释放在函数的头部，告诉人们它做什么，可能还有**为什么**做
它。

注释内核 API 函数时，请使用 kernel-doc 格式。细节请参阅 Documentation/doc-guide/ <doc_guide>
中的文件以及 `tools/docs/kernel-doc`。注意，过度注释的危险同样适用于 kernel-doc 注释。
不要添加仅仅重复从函数签名中就能明显看出的内容的样板 kernel-doc。

长（多行）注释的首选风格是：


	/*
  - This is the preferred style for multi-line
  - comments in the Linux kernel source code.
  - Please use it consistently.
	 *
  - Description:  A column of asterisks on the left side,
  - with beginning and ending almost-blank lines.
	 */

注释数据也很重要，无论它们是基本类型还是派生类型。为此，每行只做一个数据声明（不要
用逗号做多个数据声明）。这为你留出了空间，对每一项做一个小注释，解释它的用途。

### 9) 你把事情搞乱了


没关系，我们都会。你那位资深的 Unix 用户助手大概告诉过你，`GNU emacs` 会自动为
你格式化 C 源代码，而且你注意到是的，它确实会那样做，但它使用的默认值并不令人满意
（事实上，它们比随机打字还糟——无数只猴子在 GNU emacs 里打字永远也写不出一个好程序）。

所以，你要么摆脱 GNU emacs，要么改变它使用更明智的值。要做后者，你可以把以下内容
放进你的 .emacs 文件：


  (defun c-lineup-arglist-tabs-only (ignored)
    "Line up argument lists by tabs, not spaces"
    (let* ((anchor (c-langelem-pos c-syntactic-element))
           (column (c-langelem-2nd-pos c-syntactic-element))
           (offset (- (1+ column) anchor))
           (steps (floor offset c-basic-offset)))
      (* (max steps 1)
         c-basic-offset)))

  (dir-locals-set-class-variables
   'linux-kernel
   '((c-mode . (
          (c-basic-offset . 8)
          (c-label-minimum-indentation . 0)
          (c-offsets-alist . (
                  (arglist-close         . c-lineup-arglist-tabs-only)
                  (arglist-cont-nonempty .
                      (c-lineup-gcc-asm-reg c-lineup-arglist-tabs-only))
                  (arglist-intro         . +)
                  (brace-list-intro      . +)
                  (c                     . c-lineup-C-comments)
                  (case-label            . 0)
                  (comment-intro         . c-lineup-comment)
                  (cpp-define-intro      . +)
                  (cpp-macro             . -1000)
                  (cpp-macro-cont        . +)
                  (defun-block-intro     . +)
                  (else-clause           . 0)
                  (func-decl-cont        . +)
                  (inclass               . +)
                  (inher-cont            . c-lineup-multi-inher)
                  (knr-argdecl-intro     . 0)
                  (label                 . -1000)
                  (statement             . 0)
                  (statement-block-intro . +)
                  (statement-case-intro  . +)
                  (statement-cont        . +)
                  (substatement          . +)
                  ))
          (indent-tabs-mode . t)
          (show-trailing-whitespace . t)
          ))))

  (dir-locals-set-directory-class
   (expand-file-name "~/src/linux-trees")
   'linux-kernel)

这将使 emacs 更好地配合 `~/src/linux-trees` 下面的 C 文件的内核编码风格。

但即使你没能成功让 emacs 做明智的格式化，也不是一切都完了：使用 `indent`。

现在，再说一次，GNU indent 有着和 GNU emacs 一样脑残的设置，这就是为什么你需要给它
几个命令行选项。然而，这还不算太糟，因为即使是 GNU indent 的制作者也认可 K&R 的权威
（GNU 的人并不邪恶，他们只是在这件事上严重被误导了），所以你只需给 indent 选项
`-kr -i8`（代表 `K&R, 8 字符缩进`），或者使用 `scripts/Lindent`，它以最新风格缩进。

`indent` 有很多选项，特别是在注释重新格式化方面，你可能想看一下 man 页。但请记住：
`indent` 不是对糟糕编程的修复。

注意，你也可以使用 `clang-format` 工具来帮助你遵循这些规则，快速自动重新格式化你的
代码部分，并审查整个文件以发现编码风格错误、拼写错误和可能的改进。它在排序 `#includes`、
对齐变量/宏、重排文本以及其他类似任务方面也很方便。更多细节请参阅文件
Documentation/dev-tools/clang-format.rst <clangformat>。

一些基本的编辑器设置，如缩进和行尾，如果你使用的是兼容 EditorConfig 的编辑器，会
自动设置。更多信息请参阅官方 EditorConfig 网站：
https://editorconfig.org/

### 10) Kconfig 配置文件


对于整个源代码树中的所有 Kconfig* 配置文件，缩进有些不同。`config` 定义下的行用
一个制表符缩进，而帮助文本额外缩进两个
```

  config AUDIT
	bool "Auditing support"
	depends on NET
	help
	  Enable auditing infrastructure that can be used with another
	  kernel subsystem, such as SELinux (which requires this for
	  logging of avc messages output).  Does not do system-call
	  auditing without CONFIG_AUDITSYSCALL.

```
真正危险的特性（例如某些的写入支持
```

  config ADFS_FS_RW
	bool "ADFS write support (DANGEROUS)"
	depends on ADFS_FS
	...

```
关于配置文件的完整文档，请参阅文件 Documentation/kbuild/kconfig-language.rst。

### 11) 数据结构


那些在创建和销毁它们的单线程环境之外有可见性的数据结构，应该总是有引用计数。在内核中，
不存在垃圾回收（而且在内核之外垃圾回收既慢又低效），这意味着你绝对**必须**对你所有的
使用进行引用计数。

引用计数意味着你可以避免加锁，并允许多个用户并行访问该数据结构——而不必担心该结构
仅仅因为他们睡了一会儿或做了一会儿其他事情就突然从他们脚下消失。

注意，加锁**不是**引用计数的替代品。加锁用于保持数据结构的一致性，而引用计数是一种
内存管理技术。通常两者都需要，并且它们不应被混淆。

许多数据结构确实可以有两级引用计数，当存在不同 `class` 的用户时。子类计数统计子类
用户的数量，并且仅当子类计数归零时才将全局计数减一。

这种 `multi-level-reference-counting` 的例子可以在内存管理（`struct mm_struct`：
mm_users 和 mm_count）中，以及文件系统代码（`struct super_block`：s_count 和
s_active）中找到。

记住：如果另一个线程可以找到你的数据结构，而你没有对它的引用计数，那么你几乎肯定
有一个缺陷。

### 12) 宏、枚举和 RTL


定义常量的宏以及枚举中的标签名使用大写。

	#define CONSTANT 0x12345

在定义几个相关常量时，优先使用枚举。

大写宏名受人欢迎，但类似于函数的宏可以命名为小写。

一般来说，内联函数优于类似于函数的宏。

带有多个语句的宏应该包含在一个 do - while 块中：


	#define macrofun(a, b, c)			\
		do {					\
			if (a == 5)			\
				do_this(b, c);		\
		} while (0)

带有未使用参数的类函数宏应该被替换为静态内联函数，以避免未使用变量的问题：


	static inline void fun(struct foo *foo)
	{
	}

由于历史惯例，许多文件仍然采用“强制转换到 (void)”的方法来求值参数。然而，这种方法
不可取。内联函数解决了“带有副作用的表达式被求值多次”的问题，规避了未使用变量的
问题，并且出于某种原因通常比宏有更好的文档。

	/*
  - Avoid doing this whenever possible and instead opt for static
  - inline functions
	 */
	#define macrofun(foo) do { (void) (foo); } while (0)

使用宏时要避免的事情：

1) 影响控制流的宏：


	#define FOO(x)					\
		do {					\
			if (blah(x) < 0)		\
				return -EBUGGERED;	\
		} while (0)

这是一个**非常**糟糕的主意。它看起来像一次函数调用，却退出了 `调用` 它的函数；不要
破坏那些将要阅读代码的人的内部解析器。

2) 依赖具有神奇名字的局部变量的宏：


	#define FOO(val) bar(index, val)

可能看起来是个好东西，但当人们阅读代码时会困惑得要命，并且容易因为看似无害的改动而
出问题。

3) 参数被用作左值的宏：FOO(x) = y; 如果有人把 FOO 变成内联函数，会咬你一口。

4) 忘记了优先级：使用表达式定义常量的宏必须把表达式用括号括起来。对于使用参数的宏
也要警惕类似的问题。

	#define CONSTANT 0x4000
	#define CONSTEXP (CONSTANT | 3)

5) 在类似于函数的宏中定义局部变量时的命名空间冲突：


	#define FOO(x)				\
	({					\
		typeof(x) ret;			\
		ret = calc_ret(x);		\
		(ret);				\
	})

ret 是局部变量的一个常见名字——__foo_ret 不太可能与已有变量冲突。

cpp 手册详尽地处理了宏。gcc 内部手册也涵盖了 RTL，它在内核中经常与汇编语言一起使用。

### 13) 打印内核消息


内核开发者喜欢被认为是有文化的人。请注意内核消息的拼写，以留下好印象。不要使用不正确
的缩略词如 `dont`；改用 `do not` 或 `don't`。让消息简洁、清晰、无歧义。

内核消息不必以句号结尾。

用括号打印数字 (%d) 不增加任何价值，应该避免。

在 <linux/dev_printk.h> 中有许多驱动模型诊断宏，你应该使用它们来确保消息与正确的
设备和驱动匹配，并带有正确的级别标记：dev_err()、dev_warn()、dev_info() 等等。对于
不与特定设备关联的消息，<linux/printk.h> 定义了 pr_notice()、pr_info()、pr_warn()、
pr_err() 等。当驱动正常工作时它们是安静的，所以除非出了问题，否则优先使用
dev_dbg/pr_debug。

想出好的调试消息可能是个相当的挑战；而一旦你有了它们，它们对于远程排错会是非常大的
帮助。然而，调试消息的打印与其他非调试消息的打印处理方式不同。虽然其他 pr_XXX()
函数是无条件打印的，pr_debug() 却不是；默认情况下它被编译掉，除非定义了 DEBUG 或
设置了 CONFIG_DYNAMIC_DEBUG。dev_dbg() 也是如此，一个相关的约定使用 VERBOSE_DEBUG
来把 dev_vdbg() 消息添加到那些已经由 DEBUG 启用的消息中。

许多子系统有 Kconfig 调试选项，用来在相应的 Makefile 中打开 -DDEBUG；在其他情况下，
特定的文件 #define DEBUG。而当一条调试消息应该无条件打印时，例如它已经在一个与调试
相关的 #ifdef 段内，可以使用 printk(KERN_DEBUG ...)。

### 14) 分配内存


内核提供以下通用内存分配器：kmalloc()、kzalloc()、kmalloc_array()、kcalloc()、
vmalloc() 和 vzalloc()。有关它们的进一步信息，请参阅 API 文档。:ref:`Documentation/core-api/memory-allocation.rst
<memory_allocation>`

传递结构体大小的首选形式是：

	p = kmalloc(sizeof(*p), ...);

拼出结构体名的替代形式会损害可读性，并且当指针变量类型被改变、而传给内存分配器的
相应 sizeof 却没有改变时，引入了产生缺陷的机会。

强制转换返回值的 void 指针是多余的。从 void 指针到其他任何指针类型的转换由 C 编程
语言保证。

分配数组的首选形式是：

	p = kmalloc_array(n, sizeof(...), ...);

分配清零数组的首选形式是：

	p = kcalloc(n, sizeof(...), ...);

两种形式都会检查分配大小 n * sizeof(...) 的溢出，如果发生则返回 NULL。

这些通用分配函数在不带 __GFP_NOWARN 使用时，都会在失败时打印栈转储，因此在返回 NULL
时再打印一条额外的失败消息是没有用的。

### 15) 内联病


似乎存在一种普遍的误解，认为 gcc 有一个叫做 `inline` 的神奇“让我更快”的加速选项。
虽然内联的使用可能是适当的（例如作为替换宏的一种手段，见第 12 章），但它常常并非
如此。大量使用 inline 关键字会导致一个大得多的内核，这反过来又会由于 CPU 更大的
icache 占用，以及仅仅因为可用于页缓存的内存更少，而拖慢整个系统。只需想想：一次
页缓存未命中会导致一次磁盘寻道，这轻易就要花 5 毫秒。这 5 毫秒里可以塞进大量的
CPU 周期。

一个合理的经验法则是：不要把 inline 放在超过 3 行代码的函数上。这条规则的一个例外
是，当某个参数已知是编译期常量，并且由于这种常量性你**知道**编译器将能够在编译期
把你的函数大部分优化掉的情况。对于后一种情况的好例子，请参阅 kmalloc() 内联函数。

人们常常争辩说，给只使用一次的静态函数添加 inline 总是赢家，因为没有空间权衡。虽然
这在技术上是对的，但 gcc 能够自动内联它们而无需帮助，而且当第二个用户出现时移除
inline 的维护问题，超过了告诉 gcc 去做它本来就会做的事的这个提示的潜在价值。

### 16) 函数返回值与命名


函数可以返回许多不同类型的值，其中最常见的一种是指示函数成功还是失败的值。这样的
值可以表示为错误码整数（-Exxx = 失败，0 = 成功）或 `succeeded` 布尔值（0 = 失败，
非零 = 成功）。

混淆这两种表示是难以发现的缺陷的肥沃来源。如果 C 语言对整数和布尔值有强烈的区分，
那么编译器就会替我们发现这些错误……但它没有。为了帮助防止此类缺陷，总是遵循这个
```

	If the name of a function is an action or an imperative command,
	the function should return an error-code integer.  If the name
	is a predicate, the function should return a "succeeded" boolean.

```
例如，`add work` 是一条命令，add_work() 函数返回 0 表示成功，或 -EBUSY 表示失败。
同样地，`PCI device present` 是一个谓词，pci_dev_present() 函数如果成功找到匹配的设备
返回 1，否则返回 0。

所有被 EXPORT 的函数都必须遵守这个约定，所有公共函数也应该如此。私有（静态）函数不
必，但建议它们这样做。

返回值实际上是计算结果的、而不是指示计算是否成功的函数，不受此规则约束。一般来说，
它们通过返回一个超出范围的结果来指示失败。典型的例子是返回指针的函数；它们使用
NULL 或 ERR_PTR 机制来报告失败。

### 17) 使用 bool


Linux 内核的 bool 类型是 C99 _Bool 类型的别名。bool 值只能求值为 0 或 1，并且到 bool
的隐式或显式转换会自动把值转换为 true 或 false。使用 bool 类型时不需要 !! 构造，这
消除了一类缺陷。

使用 bool 值时，应该使用 true 和 false 定义，而不是 1 和 0。

bool 函数返回类型和栈变量在适当的时候总是可以放心使用。鼓励使用 bool 来提高可读性，
并且对于存储布尔值来说，它通常是比 'int' 更好的选择。

如果缓存行布局或值的大小很重要，不要使用 bool，因为它的大小和对齐根据所编译的体系
结构而变化。为对齐和大小优化了的结构体不应该使用 bool。

如果一个结构体有许多真/假值，考虑把它们合并成一个具有 1 位成员的位域，或者使用一个
适当的定宽类型，例如 u8。

类似地，对于函数参数，许多真/假值可以合并成单个按位 'flags' 参数，并且如果调用点
有裸露的真/假常量，'flags' 常常是更易读的选择。

否则，在结构体和参数中有限地使用 bool 可以提高可读性。

### 18) 不要重新发明内核宏


在 include/linux/ 中有许多头文件，包含许多你应该使用的宏，而不是显式地自己编写它们
的某个变体。例如，如果你需要计算一个数组的长度，可以利用这个宏


	#define ARRAY_SIZE(x) (sizeof(x) / sizeof((x)[^0^]))

它定义在 array_size.h 中。

类似地，如果你需要计算某个结构体成员的大小，使用


	#define sizeof_field(t, f) (sizeof(((t*)0)->f))

它定义在 stddef.h 中。

在 minmax.h 中还定义了进行严格类型检查的 min() 和 max() 宏，如果你需要的话。随意
浏览头文件，看看还有什么已经定义好的东西是你不应该在自己的代码中重现的。

### 19) 编辑器模式行和其他杂物


某些编辑器可以解释嵌入在源文件中的、用特殊标记指示的配置信息。例如，emacs 解释像
这样的行：


	-**- mode: c -**-

或者像这样：


	/*
	Local Variables:
	compile-command: "gcc -DMAGIC_DEBUG_FLAG foo.c"
	End:
	*/

Vim 解释像这样的标记：


	/** vim:set sw=8 noet **/

不要在源文件中包含任何这些。人们有他们自己的个人编辑器配置，你的源文件不应该覆盖
它们。这包括用于缩进和模式配置的标记。人们可能使用他们自己自定义的 mode，或者可能
有某种其他神奇的方法让缩进正确工作。

### 20) 内联汇编


在特定于体系结构的代码中，你可能需要使用内联汇编来与 CPU 或平台功能交互。在必要时
不要犹豫这样做。然而，当 C 能完成工作时，不要 gratuitously（无意义地）使用内联汇编。
在可能的时候，你可以并且应该从 C 中操作硬件。

考虑编写简单的辅助函数来包装常见的内联汇编片段，而不是反复以微小变化地编写它们。
请记住，内联汇编可以使用 C 参数。

大型、非平凡汇编函数应该放在 .S 文件中，并在 C 头文件中定义相应的 C 原型。汇编函数
的 C 原型应该使用 `asmlinkage`。

你可能需要把你的 asm 语句标记为 volatile，以防止 GCC 在没注意到任何副作用时把它移除。
不过你并不总是需要这样做，而不必要地这样做会限制优化。

当写一个包含多条指令的单一内联汇编语句时，把每条指令放在一个单独引用字符串的单独
一行上，并且除最后一条外每条字符串以 `\n\t` 结尾，以在汇编输出中正确地缩进下一条
指令：


	asm ("magic %reg1, #42\n\t"
	     "more_magic %reg2, %reg3"
	     : /** outputs **/ : /** inputs **/ : /** clobbers **/);

### 21) 条件编译


在可能的情况下，不要在 .c 文件中使用预处理器条件（#if、#ifdef）；这样做会让代码更难
阅读，逻辑更难跟踪。相反，在定义供那些 .c 文件使用的函数的头文件中使用这样的条件，
在 #else 情况下提供空操作（no-op）桩版本，然后从无条件的 .c 文件中调用那些函数。
编译器会避免为桩调用生成任何代码，产生相同的结果，但逻辑将仍然容易跟踪。

优先编译出整个函数，而不是函数的一部分或表达式的一部分。与其在表达式中放一个 ifdef，
不如把表达式的部分或全部提取到一个单独的辅助函数中，并对该函数应用条件。

如果你有一个函数或变量在某个特定配置中可能不被使用，并且编译器会警告它的定义未被
使用，就把该定义标记为 __maybe_unused，而不是把它包在预处理器条件中。（然而，如果
一个函数或变量**总是**不被使用，就删除它。）

在代码内，在可能的情况下，使用 IS_ENABLED 宏把一个 Kconfig 符号转换成一个 C 布尔
表达式，并在普通的 C 条件中使用它：


	if (IS_ENABLED(CONFIG_SOMETHING)) {
		...
	}

编译器会把条件常量折叠掉，并像 #ifdef 一样包含或排除这段代码块，所以这不会增加任何
运行时开销。然而，这种方法仍然允许 C 编译器看到块内的代码，并检查它的正确性（语法、
类型、符号引用等）。因此，如果块内的代码引用了在条件不满足时不会存在的符号，你仍然
需要使用 #ifdef。

在任何非平凡的 #if 或 #ifdef 块（超过几行）的末尾，在 #endif 同一行之后放一个注释，
注明所用的条件表达式。例如：


	#ifdef CONFIG_SOMETHING
	...
	#endif /** CONFIG_SOMETHING **/

### 22) 不要使内核崩溃


一般来说，使内核崩溃的决定属于用户，而不是内核开发者。

######## 避免使用 panic()


panic() 应该谨慎使用，并且主要只在系统启动期间。例如，当在启动期间内存耗尽并且无法
继续时，panic() 是可以接受的。

######## 使用 WARN() 而非 BUG()


不要添加使用任何 BUG() 变体（如 BUG()、BUG_ON() 或 VM_BUG_ON()）的新代码。相反，使用
WARN*() 变体，最好是 WARN_ON_ONCE()，并且可能带有恢复代码。如果没有合理的方式至少
部分恢复，则不需要恢复代码。

“我太懒了，不想做错误处理”不是使用 BUG() 的借口。没有任何继续方式的主要内部损坏可能
仍然使用 BUG()，但需要充分的理由。

######## 使用 WARN_ON_ONCE() 而非 WARN() 或 WARN_ON()


WARN_ON_ONCE() 通常优于 WARN() 或 WARN_ON()，因为对于给定的警告条件，如果它发生的话，
通常会发生多次，这很常见。这可能填满并回绕内核日志，甚至可能把系统拖慢到过度的日志
本身变成一个新的、额外的问题。

######## 不要轻率地使用 WARN


WARN*() 用于意外的、这绝不该发生的情况。WARN*() 宏不应用于正常操作期间预期会发生的
任何事情。例如，这些不是前置或后置条件断言。再说一遍：WARN*() 绝不能用于预期容易
触发的条件，例如由用户空间动作触发。如果你需要通知用户一个问题，pr_warn_once() 是
一个可能的替代。

######## 不必为 panic_on_warn 用户担心


关于 panic_on_warn 再多说几句：请记住 `panic_on_warn` 是一个可用的内核选项，并且许多
用户设置了这个选项。这就是为什么上面有一篇“不要轻率地使用 WARN”的说明。然而，
panic_on_warn 用户的存在并不是避免明智地使用 WARN*() 的正当理由。那是因为，无论谁
启用了 panic_on_warn，都明确要求内核在 WARN*() 触发时崩溃，并且这样的用户必须准备好
应对一个更可能崩溃的系统的后果。

######## 使用 BUILD_BUG_ON() 进行编译期断言


使用 BUILD_BUG_ON() 是可以接受并且受鼓励的，因为它是一个编译期断言，在运行时没有
效果。

### 附录 I) 参考资料


The C Programming Language, Second Edition
by Brian W. Kernighan and Dennis M. Ritchie.
Prentice Hall, Inc., 1988.
ISBN 0-13-110362-8 (paperback), 0-13-110370-9 (hardback).

The Practice of Programming
by Brian W. Kernighan and Rob Pike.
Addison-Wesley, Inc., 1999.
ISBN 0-201-61586-X.

GNU manuals - where in compliance with K&R and this text - for cpp, gcc,
gcc internals and indent, all available from https://www.gnu.org/manual/

WG14 is the international standardization working group for the programming
language C, URL: http://www.open-std.org/JTC1/SC22/WG14/

Kernel CodingStyle, by greg@kroah.com at OLS 2002:
http://www.kroah.com/linux/talks/ols_2002_kernel_codingstyle_talk/html/
