


## 内核的命令行参数


以下是内核参数的合并列表，这些参数由 __setup()、early_param()、core_param()
module_param() 宏实现，并按英文字典顺序排序（定义为忽略所有标点，并以
不区分大小写的方式将数字排在字母之前），并附有已知的描述
内核解析来自内核命令行的参数，直"`--`" 为止；如果它无法识别某个参数且该
参数不包'.'，则该参数会被传递给 init：带'=' 的参数进init 的环境，
其他的作为参数传递给 init。`"--"` 之后的所有内容都作为参数传递给 init
模块参数可以通过两种方式指定：通过内核命令```

	(kernel command line) usbcore.blinkenlights=1
	(modprobe command line) modprobe usbcore blinkenlights=1

```
对于内建到内核中的模块，其参数需要在内核命令行上指定。modprobe 会查内核命令行（/proc/cmdline）并在加载模块时收集模块参数，因此内核命令行
也可以用于可加载模块
本文档可能并不完全是最新和全面的。命"modinfo -p ${modulename}" 显示可加模块的所有参数的当前列表。可加载模块在加载到运行中的内核后，也会/sys/module/${modulename}/parameters/ 中显示其参数。其中一些参数可以通过命令
`echo -n ${value} > /sys/module/${modulename}/parameters/${parm}` 在运行时更改
### 特殊处理


```

	log_buf_len=1M print-fatal-signals=1

```
```

	log-buf-len=1M print_fatal_signals=1

```
```

	param="spaces in here"

```
#### CPU 列表


一些内核参数以 CPU 列表作为值，例如 isolcpus、nohz_full、irqaffinityrcu_nocbs。该列表的格式为
	<cpu number>,...,<cpu number>

鎴?
	<cpu number>-<cpu number>
	（必须是升序的正范围
或混合形
<cpu number>,...,<cpu number>-<cpu number>

注意，对于范围这种特殊情况，可以将范围拆分为大小相等的组，并对每个组使用
从该组开头起的一部分
	<cpu number>-<cpu number>:<used size>/<group size>

例如，可以向命令行添加以下参数：

	isolcpus=1,2,10-20,100-2000:2/25

其中最后一项表CPU 100,101,125,126,150,151,...

"N" 可用于表示系统上编号最后的 CPU，即 "foo_cpus=16-N" 32 核系统上
等价"16-31"
请记"N" 是动态的，因此如果系统变化导致位图宽度改变，例如 CPU 列表中的
核心更少，那N 以及任何使用 N 的范围也会改变。在小巧4 核系统上使用
相同的参数，"16-N" 会变"16-3"，现在相同的启动输入会被标记为无（start > end）
特殊的大小写无关组名 "all" 的含义是选择所CPU，因"nohz_full=all" 等价"nohz_full=0-N"
"N" "all" 的语义在 bitmap 层面上受支持，并适用bitmap_parselist() 所有用户
#### 度量后缀（Metric suffixes

[KMG] 后缀通常在许多内核参数值之后描述。允'K'M'G'T'P' 'E'
后缀。这些字母表_二进制_ 乘数 'Kilo'Mega'Giga'Tera'Peta' 'Exa'，分别等2^10^20^30^40^50 2^60 字节。此类字母后缀可以完全省略
### 内核构建选项


下面列出的参数只有在启用了某些内核构建选项且存在相应硬件时才有效。该列表
应保持字母顺序。每个描述开头方括号中的文本说明了该参数适用的限制条件
标有 BOOT 的参数实际上由引导加载程序解释，对内核本身没有直接意义除非有极端需要或<Documentation/arch/x86/boot.rst> 协调，否则不要修改引加载程序参数的语法
还有一些特定于架构的内核参数未在此处记录
注意，下面列出的所有内核参数都是区分大小写的，并且任何参数名末尾的 '='
表示该参数将作为环境变量输入，而没'=' 则表示它将作为内核参数出现，可由
系统启动后运行的程序通过 /proc/cmdline 读取
内核参数的数量不受限制，但完整命令行（包括空格等的参数）的长度被限制固定数量的字符。该限制取决于架构，介于 256 4096 个字符之间。它在文./include/uapi/asm-generic/setup.h 中定义为 COMMAND_LINE_SIZE
   :literal:
