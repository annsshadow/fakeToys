
## 不可靠的 Linux 内核 Hacking 指南


:Author: Rusty Russell

## 引言


欢迎，温和的读者，来到 Rusty 的《极不可靠的 Linux 内核 Hacking 指南》。本文档描述了内核代码的常用例程与通用要求：其目标是作为有经验C 程序员进Linux 内核开发的入门读物。我回避实现细节：那是代码本身要做的事，而且我会忽略大段有用的例程

在你阅读本文之前，请理解：我从未想过要写这篇文档，因为我资历严重不足，但我一直想读它，而这是唯一的办法。我希望它能成长为一本汇集最佳实践、常见起点与零散信息的纲要

## 鍙備笌鑰。


系统中的每个 CPU 在任何时刻都可能处于以下状态之一

- 不与任何进程关联，正在处理硬件中断；

- 不与任何进程关联，正在处理软中断（softirq）或小任务（tasklet）；

- 运行在内核空间，与某个进程关联（用户上下文）

- 在用户空间运行某个进程

这些状态之间存在一种次序关系。最下面的两种可以互相抢占，但在这之上是严格的层级：每一层只能被其上层的状态抢占。例如，当某CPU 上正在运行软中断时，不会有其他软中断抢占它，但硬件中断可以。然而，系统中的任何其他 CPU 都是独立执行的

我们将看到用户上下文可以通过多种方式屏蔽中断，从而变得真正不可抢占

### 用户上下


用户上下文是指你从系统调用或其他陷阱进入内核时的状态：与用户空间类似，你可能会被更重要的任务以及中断所抢占。你可以通过调用 schedule() 进入睡眠


    在模块加载与卸载时，以及对块设备层的操作中，你始终处于用户上下文

在用户上下文中，`current` 指针（指向当前正在执行的任务）是有效的，in_interrupt()（`include/linux/preempt.h`）返false


    注意，如果你禁用了抢占或软中断（见下文），in_interrupt() 会返回误报（false positive）

### 硬件中断（Hard IRQ


定时器节拍、网卡和键盘都是会在任意时刻产生中断的真实硬件示例。内核会运行中断处理程序来为硬件提供服务。内核保证该处理程序绝不会被重入：如果同一个中断再次到来，它会被排队（或丢弃）。因为它会禁用中断，所以这个处理程序必须很快：通常它只是简单地确认中断、标记一个待执行的“软件中断”，然后退出

你可以通过 in_hardirq() 返回 true 来判断自己正处于硬件中断中


    注意，如果中断被禁用（见下文），该函数会返回误报（false positive）

### 软件中断上下文：软中断（Softirq）与小任务（Tasklet


每当系统调用即将返回用户空间，或硬件中断处理程序退出时，任何被标记为挂起（通常由硬件中断标记）的“软件中断”都会得到运行（`kernel/softirq.c`）

大量真正的中断处理工作是在这里完成的。在SMP 过渡的早期，只有“下半部”（bottom halves，BHs），它们无法利用多个 CPU。在我们抛弃了用火柴棍和鼻涕做的发条计算机之后不久，便放弃了这一限制，转而使用“软中断”（softirqs）

`include/linux/interrupt.h` 列出了不同的软中断。一个非常重要的软中断是定时器软中断（`include/linux/timer.h`）：你可以注册，让它在给定时间长度后为你调用函数

软中断往往很难处理，因为同一个软中断会同时在多个 CPU 上运行。因此，小任务（tasklets，`include/linux/interrupt.h`）使用得更频繁：它们是动态可注册的（意味着你可以拥有任意多个），并且它们还保证任意一个小任务在任意时刻只会在一CPU 上运行，尽管不同的小任务可以并行运行


    “tasklet”这个名字具有误导性：它们与“任务（tasks）”毫无关系

你可以通过 in_softirq() 宏（`include/linux/preempt.h`）判断自己是否处于软中断（或小任务）中


    注意，如果持有一bottom half lock <local_bh_disable>，该函数会返回误报（false positive）

## 一些基本规


没有内存保护
    如果你破坏了内存，无论是在用户上下文还是中断上下文中，整台机器都会崩溃。你确定你不能在用户空间完成你想做的事吗

不能使用浮点MMX
    FPU 上下文不会被保存；即使在用户上下文中，FPU 状态也可能与当前进程不对应：你会弄乱某个用户进程的 FPU 状态。如果你真的想这样做，就必须显式地保恢复完整FPU 状态（并且避免上下文切换）。这通常是个坏主意；请优先使用定点运算

严格的栈限制
    根据配置选项的不同，大多32 位架构的内核栈约3K 6K：大多数 64 位架构上约为 14K，而且常常与中断共享，因此你无法全部使用。避免在栈上进行深层递归以及声明巨大的局部数组（改为动态分配它们）

Linux 内核是可移植
    让我们保持这种状态。你的代码应当做64 位干净，并且与字节序无关。你还应当尽量减少与 CPU 相关的代码，例如内联汇编应当被清晰地封装起来，并尽量少用以便于移植。通常它应当被限制在内核源码树中与体系结构相关的部分

## ioctls：不要写新的系统调用


```
    asmlinkage long sys_mycall(int arg)
    {
            return 0;
    }


```
首先，在大多数情况下你并不想创建一个新的系统调用。你可以创建一个字符设备，并为它实现一个合适的 ioctl。这比系统调用灵活得多，不必写入每个架构`include/asm/unistd.h` `arch/kernel/entry.S` 文件，而且也更容易Linus 接受

如果你的例程只是读取或写入某个参数，不妨考虑改用 sysfs() 接口实现

ioctl 内部时你处于某个进程的用户上下文中。当发生错误时，你返回一个取反的 errno（参`include/uapi/asm-generic/errno-base.h`、`include/uapi/asm-generic/errno.h` `include/linux/errno.h`），否则返回 0

在你睡眠之后，应当检查是否有信号发生：Unix/Linux 处理信号的方式是`-ERESTARTSYS` 错误暂时退出系统调用。系统调用入口代码会切回用户上下文，执行信号处理程序，然后你的系统调用会被重新启动（除非用户禁用了该行为）。因此你应当准备好处理重启，例如当你正在操作某个数据结构的中途时


```
    if (signal_pending(current))
            return -ERESTARTSYS;


```
如果你要进行更长的计算：先考虑放到用户空间。如果你**真的**想在内核里做，就应当定期检查是否需要让CPU（请记住每个 CPU 上是协作式多任务）
```
    cond_resched(); /* Will sleep */


```
关于接口设计的一点说明：UNIX 系统调用的格言是“提供机制，而非策略”

## 死锁配方


你不能调用任何可能睡眠的例程，除非：

- 你处于用户上下文中

- 你没有持有任何自旋锁（spinlock）

- 你已启用中断（实际上，Andi Kleen 说调度代码会替你启用它们，但那大概不是你想要的）

注意，某些函数可能会隐式睡眠：常见的例子是用户空间访问函数（\*_user）以及不`GFP_ATOMIC` 的内存分配函数

你应当始终在内核中开`CONFIG_DEBUG_ATOMIC_SLEEP`，如果你违反了这些规则，它会给出警告。如果你**真的**违反了规则，你的机器最终会被锁死

真的

## 常用例程


### printk()


瀹氫箟浜?`include/linux/printk.h`

printk() 将内核消息送往控制台、dmesg 以及 syslog 守护进程。它对调试和报告错误很有用，并且可以在中断上下文中使用，但使用时务必小心：一台控制台printk 消息淹没的机器是无法使用的。它使用的格式字符串基本兼容 ANSI C printf，以C 字符
```
    printk(KERN_INFO "i = %u\n", i);


```
参见 `include/linux/kern_levels.h`，了解其`KERN_` 取值；这些取值会syslog 解释为级别。特殊情况：打印一IP 地址
```
    __be32 ipaddress;
    printk(KERN_INFO "my ip: %pI4\n", &ipaddress);


```
printk() 内部使用一1K 的缓冲区，并且不会检测溢出。请确保它足够用


    当你开始在自己的用户程序中printf 误打printk 时，你就知道自己已经是一名真正的内核黑客:)

    另一个旁注：最初的 Unix Version 6 源代码在printf 函数上方有一条注释：“Printf 不应被用于闲聊”。你应该遵从这条建议

### copy_to_user() / copy_from_user() / get_user() / put_user()


瀹氫箟浜?`include/linux/uaccess.h` / `asm/uaccess.h`

**[SLEEPS]**

put_user() get_user() 用于在用户空间与内核之间获取和存放单个值（例如一int、char long）。指向用户空间的指针绝不应该被直接解引用：数据应当通过这些例程来复制。两者都返回 `-EFAULT` 0

copy_to_user() copy_from_user() 则更为通用：它们可以在用户空间与内核之间复制任意数量的数据


    put_user() get_user() 不同，它们返回未复制的数据量（即 0 仍然表示成功）

[是的，这个令人不快的接口让我直起鸡皮疙瘩。关于它的争论大约每年都会冒出来一次。——RR.]

这些函数可能会隐式睡眠。绝不应在用户上下文之外、中断被禁用时、或持有自旋锁时调用它们

### kmalloc()/kfree()


瀹氫箟浜?`include/linux/slab.h`

**[MAY SLEEP: SEE BELOW]**

这些例程用于动态请求按指针对齐的内存块，就像用户空间中malloc free 那样，但 kmalloc() 额外带有一个标志字。重要的取值有

`GFP_KERNEL`
    可能会睡眠并通过交换来释放内存。只允许在用户上下文中使用，但这是分配内存最可靠的方式

`GFP_ATOMIC`
    不会睡眠。不`GFP_KERNEL` 可靠，但可以在中断上下文中调用。你**真的**应当准备一个良好的内存耗尽错误处理策略

`GFP_DMA`
    分配低于 16MB ISA DMA 内存。如果你不知道那是什么，那你不需要它。非常不可靠

如果你看到了“在无效上下文中调用了可能睡眠的函数”的警告信息，那么也许是你没有使`GFP_ATOMIC` 就在中断上下文中调用了可能睡眠的分配函数。你真的应该修复它。跑，别走

如果你要分配至少 `PAGE_SIZE`（`asm/page.h` `asm/page_types.h`）字节，可以考虑使用 __get_free_pages()（`include/linux/gfp.h`）。它接受一order 参数 表示一页大小，1 表示两页 表示四页，依此类推）以及和上面相同的内存优先级标志字

如果你要分配超过一页的字节数，可以使用 vmalloc()。它会在内核映射区中分配虚拟内存。这一块内存在物理内存中并不连续，MMU 会为你让它看起来是连续的（因此它只对 CPU 看起来是连续的，对外部设备驱动则不是）。如果你真的因为某些奇怪的设备而需要大块物理连续内存，那你就遇到麻烦了：Linux 对它的支持很差，因为在运行的内核中，经过一段时间后内存碎片会使得它变得困难。最好的办法是在启动过程的早期通过 alloc_bootmem() 例程来分配这块内存

在发明你自己的常用对象缓存之前，不妨考虑使用 `include/linux/slab.h` 中的 slab 缓存

### current


瀹氫箟浜?`include/asm/current.h`

这个全局变量（实际上是一个宏）包含一个指向当前任务结构体的指针，因此只在用户上下文中有效。例如，当一个进程发起系统调用时，它会指向调用进程的任务结构体。它在中断上下文*不是 NULL**

### mdelay()/udelay()


瀹氫箟浜?`include/asm/delay.h` / `include/linux/delay.h`

udelay() ndelay() 函数可用于短暂暂停。不要对它们使用很大的数值，否则有溢出的风险——这里的辅助函数 mdelay() 很有用，或者也可以考虑 msleep()

### cpu_to_be32()/be32_to_cpu()/cpu_to_le32()/le32_to_cpu()


瀹氫箟浜?`include/asm/byteorder.h`

cpu_to_be32() 系列（其中2”可以替换为 64 16，“be”可以替换为“le”）是内核中进行字节序转换的通用方式：它们返回转换后的值。所有变体也都提供反向转换：例如 be32_to_cpu() 等

这些函数有两大主要变体：指针变体，例cpu_to_be32p()，它接受一个指向给定类型的指针，并返回转换后的值。另一个变体是“就地（in-situ）”系列，例如 cpu_to_be32s()，它转换指针所指向的值，并返void

### local_irq_save()/local_irq_restore()


瀹氫箟浜?`include/linux/irqflags.h`

这些例程在本CPU 上禁用硬中断，并恢复它们。它们是可重入的；会把先前的状态保存在它们唯一`unsigned long flags` 参数中。如果你确定中断是开启的，可以直接使local_irq_disable() local_irq_enable()

### local_bh_disable()/local_bh_enable()


瀹氫箟浜?`include/linux/bottom_half.h`

这些例程在本CPU 上禁用软中断，并恢复它们。它们是可重入的；如果软中断在此之前已被禁用，那么在这对函数被调用之后它们仍然会被禁用。它们会阻止软中断和小任务在当前 CPU 上运行

### smp_processor_id()


瀹氫箟浜?`include/linux/smp.h`

get_cpu() 会禁用抢占（这样你就不会突然被移到另一CPU 上），并返回当前的处理器编号，介0 `NR_CPUS` 之间。注CPU 编号不一定是连续的。在你完成时，用 put_cpu() 把它重新归还

如果你知道自己不会被另一个任务抢占（即你处于中断上下文中，或者已经禁用了抢占），就可以使smp_processor_id()

### ``__init``/``__exit``/``__initdata``


瀹氫箟浜? `include/linux/init.h`

启动之后，内核会释放一个特殊的段；被标记为 `__init` 的函数和被标记为 `__initdata` 的数据结构会在启动完成后被丢弃：类似地，模块也会在初始化之后丢弃这部分内存。`__exit` 用于声明一个只在退出时才需要的函数：如果这个文件没有被编译为模块，该函数会被丢弃。具体用法请参阅头文件。注意，将一个标记为 `__init` 的函数用 EXPORT_SYMBOL() EXPORT_SYMBOL_GPL() 导出给模块是没有意义的——这会出问题

### __initcall()/module_init()


瀹氫箟浜? `include/linux/init.h` / `include/linux/module.h`

内核的许多部分都很适合做成模块（内核中可动态加载的部分）。使module_init() module_exit() 宏，可以很容易地写出不需#ifdef 的代码，它能够既作为模块运行，也可以内建到内核中

module_init() 宏定义了在模块插入时（如果这个文件被编译为模块）或在启动时（如果这个文件没有被编译为模块）要被调用的函数：如果文件没有被编译为模块，module_init() 宏就等同__initcall()，后者通过链接器的魔法确保该函数在启动时会被调用

该函数可以返回一个负的错误号，使模块加载失败（遗憾的是，如果模块被编译进内核，这不会起作用）。这个函数是在中断开启的用户上下文中被调用的，因此它可以睡眠

### module_exit()


瀹氫箟浜? `include/linux/module.h`

这个宏定义了在模块移除时（或者，对于被编译进内核的文件来说，永远不会）要被调用的函数。只有当模块的使用计数降为零时它才会被调用。这个函数也可以睡眠，但不能失败：在它返回之前一切都必须被清理干净

注意这个宏是可选的：如果它不存在，你的模块将无法被移除（除非使用“rmmod -f”）

### try_module_get()/module_put()


瀹氫箟浜?`include/linux/module.h`

这些函数操作模块的使用计数，以防止模块被移除（如果一个模块使用了另一个模块导出的某个符号，该模块同样无法被移除：见下文）。在调用进入模块代码之前，你应该对该模块调用 try_module_get()：如果它失败，说明该模块正在被移除，你应当表现得好像它不存在一样。否则，你可以安全地进入该模块，并在完成时调module_put()

大多数可注册的结构体都有一owner 字段，例`struct file_operations <file_operations>` 结构体中。将这个字段设为`THIS_MODULE`

## 等待队列 ``include/linux/wait.h``


**[SLEEPS]**

等待队列用于在某条件为真时等待有人来唤醒你。它们必须被小心使用，以确保不存在竞态条件。你声明一`wait_queue_head_t`，然后想要等待该条件的进程声明一个指向自身的 `wait_queue_entry_t`，并把它放入队列中

### 声明


你可以使DECLARE_WAIT_QUEUE_HEAD() 宏来声明一`wait_queue_head_t`，或者在你的初始化代码中使用 init_waitqueue_head() 例程

### 入队


把自己放入等待队列相当复杂，因为你必须在检查条件之前先把自己放入队列。有一个宏可以做这件事：wait_event_interruptible()（`include/linux/wait.h`）。第一个参数是等待队列头，第二个是一个会被求值的表达式；当该表达式为真时宏返0，如果收到信号则返回 `-ERESTARTSYS`。wait_event() 版本会忽略信号

### 唤醒队列中的任务


调用 wake_up()（`include/linux/wait.h`），它会唤醒队列中的每一个进程。例外情况是如果某个进程设置`TASK_EXCLUSIVE`，那么队列中剩余的进程将不会被唤醒。在同一个头文件中还可以找到这个基本函数的其他变体

## 原子操作


某些操作在所有平台上都保证是原子的。第一类操作作用于 `atomic_t`（`include/asm/atomic.h`）；它包含一个有符号整数（至32 位长），你必须使用这些函数来操作或读`atomic_t` 变量。atomic_read() atomic_set() 用于获取和设置计数器，atomic_add()、atomic_sub()、atomic_inc()、atomic_dec() 以及 atomic_dec_and_test()（如果递减到零则返true）

是的。如果原子变量为零，它返true（即 != 0）

注意这些函数比普通的算术运算要慢，因此不应被不必要地使用

第二类原子操作是对一`unsigned long` 进行的原子位操作，定义于 `include/linux/bitops.h`。这些操作通常接受一个指向位模式的指针，以及一个位号：0 是最低有效位。set_bit()、clear_bit() change_bit() 分别用于设置、清除和翻转给定的位。test_and_set_bit()、test_and_clear_bit() test_and_change_bit() 做同样的事情，不同之处在于如果指定位先前已被设置则返true；这些对于原子地设置标志位特别有用

可以用大`BITS_PER_LONG` 的位索引来调用这些操作。不过在大端（big-endian）平台上由此产生的行为很怪异，所以最好不要这样做

## 符号


在内核本体内部，适用普通的链接规则（即，除非一个符号通过 `static` 关键字被声明为文件作用域，否则它可以在内核的任何地方使用）。然而，对于模块，内核维护了一张特殊的导出符号表，它限制了进入内核本体的入口点。模块也可以导出符号

### EXPORT_SYMBOL()


瀹氫箟浜?`include/linux/export.h`

这是导出符号的经典方法：动态加载的模块将能够像平常一样使用该符号

### EXPORT_SYMBOL_GPL()


瀹氫箟浜?`include/linux/export.h`

EXPORT_SYMBOL() 类似，只是由 EXPORT_SYMBOL_GPL() 导出的符号只能被带有指定 GPLv2 兼容许可证的 MODULE_LICENSE() 的模块看到。这意味着该函数被视为一个内部实现细节，而非真正的接口。不过，某些维护者和开发者在添加任何API 或新功能时，可能会要求使EXPORT_SYMBOL_GPL()

### EXPORT_SYMBOL_NS()


瀹氫箟浜?`include/linux/export.h`

这是 EXPORT_SYMBOL() 的变体，允许指定一个符号命名空间。符号命名空间在 Documentation/core-api/symbol-namespaces.rst 中有文档说明

### EXPORT_SYMBOL_NS_GPL()


瀹氫箟浜?`include/linux/export.h`

这是 EXPORT_SYMBOL_GPL() 的变体，允许指定一个符号命名空间。符号命名空间在 Documentation/core-api/symbol-namespaces.rst 中有文档说明

## 例程与约


### 双向链表 ``include/linux/list.h``


内核头文件中曾经有三套链表例程，但这一套胜出了。如果你没有什么特别迫切的需要使用单链表，它是一个不错的选择

尤其list_for_each_entry() 很有用

### 返回约定


对于在用户上下文中被调用的代码，违背 C 语言惯例、以返回 0 表示成功、以负的错误号（例如 `-EFAULT`）表示失败是非常常见的。起初这可能不直观，但它在内核中相当普遍

使用 ERR_PTR()（`include/linux/err.h`）将一个负的错误号编码进指针，再用 IS_ERR() PTR_ERR() 把它取出来：这样可以避免为错误号单独使用一个指针参数。有点别扭，但别扭得好

### 破坏编译


Linus 和其他开发者有时会在开发中的内核里改动函数或结构体的名字；这样做不仅仅是为了让大家保持警惕：它反映了一个根本性的变化（例如，不再能在中断开启时被调用，或者会做额外的检查，又或者不再做此前会被捕获的检查）。通常这还会附带一封相当完整的说明发往相应的内核开发邮件列表；请搜索邮件归档。简单地对整个文件做全局替换通常只会让事*更糟**

### 初始化结构体成员


初始化结构体的首选方法是使用指定初始化器（designated initializer
```
    static struct block_device_operations opt_fops = {
            .open               = opt_open,
            .release            = opt_release,
            .ioctl              = opt_ioctl,
            .check_media_change = opt_media_change,
    };


```
这样便于grep 查找，也能清楚地看出设置了哪些结构体字段。你应该这样做，因为这看上去很酷

### GNU 扩展


GNU 扩展Linux 内核中被明确允许。注意其中一些较为复杂的扩展由于缺乏普遍使用而支持得不是很好，但以下这些被认为是标准的（更多细节请参GCC info 页“C Extensions”一节——是的，真的要看 info 页，man 页只info 中内容的简短摘要）

- 内联函数

- 语句表达式（({ }) 构造）

- 声明函数/变量/类型的属性（__attribute__

- typeof

- 零长度数

- 宏可变参数（varargs

- void 指针进行算术运算

- 非常量初始化

- 汇编指令（不能出现在 arch/ include/asm/ 之外

- 以字符串形式使用函数名（__func__）

- __builtin_constant_p()

在内核中使用 long long 时要小心，gcc 为它生成的代码糟糕透顶：除法和乘法i386 上无法工作，因为内核环境中缺少对应的 GCC 运行时函数

### C++


在内核中使用 C++ 通常是个坏主意，因为内核不提供必要的运行时环境，而且头文件也没有针对它进行测试。它仍然是可能的，但不推荐。如果你真的想这样做，至少忘掉异常吧

### #if


通常更干净的做法是在头文件（或 .c 文件顶部）中使用宏来抽象掉函数，而不是在整个源代码中到处使用 \`#if\` 预处理器语句

## 将你的代码并入内


为了让你的成果达到可被官方收录的形态，甚至是仅仅做出一个整洁的补丁，都有一些管理工作要做：

- 弄清楚谁是你所修改代码的拥有者。查看源文件顶部、内部的 `MAINTAINERS` 文件，最后才`CREDITS` 文件。你应当与这些人协调，确保你没有在重复劳动，或者试图做已经被拒绝的事情

   务必在你创建或大幅修改的任何文件顶部写上你的姓名和电子邮件地址。当人们发现一bug，或*他们**想要做改动时，这是他们第一眼会看的地方

- 通常你会想为你的内核 hack 提供一个配置选项。编辑相应目录下`Kconfig`。Config 语言通过复制粘贴即可轻松使用，完整的文档Documentation/kbuild/kconfig-language.rst 中

   在对该选项的描述中，务必同时顾及专家用户和对你的功能一无所知的用户。在此处提及不兼容性和问题*一定要**用“if in doubt, say N”（或者偶尔用 \`Y\`）来结束你的描述；这是给那些完全不知道你在说什么的人看的

- 编辑 `Makefile`：CONFIG 变量在这里被导出，所以你通常只需要加一“obj-$(CONFIG_xxx) += xxx.o”。其语法Documentation/kbuild/makefiles.rst 中有文档说明

- 如果你认为自己所做的东西值得一提，通常超出单个文件的范围（反正你的名字本就该在源文件顶部），就把你自己加入 `CREDITS`。`MAINTAINERS` 意味着当某个子系统发生改动时你希望被咨询，并得bug 的相关信息；它暗示你对代码的某些部分有着超出一时兴起的投入

- 最后，别忘了阅Documentation/process/submitting-patches.rst

## 内核咒语


一些在浏览源码时发现的珍品。欢迎往这个列表里添加

```
    #define ndelay(n) (__builtin_constant_p(n) ? \
            ((n) > 20000 ? __bad_ndelay() : __const_udelay((n) * 5ul)) : \
            __ndelay(n))


```
```
    /*
     * Kernel pointers have redundant information, so we can use a
     * scheme where we can return either an error code or a dentry
     * pointer with the same return value.
     *
     * This should be a per-architecture thing, to allow different
     * error and pointer decisions.
     */
     #define ERR_PTR(err)    ((void *)((long)(err)))
     #define PTR_ERR(ptr)    ((long)(ptr))
     #define IS_ERR(ptr)     ((unsigned long)(ptr) > (unsigned long)(-1000))


```
```
    #define copy_to_user(to,from,n)                         \
            (__builtin_constant_p(n) ?                      \
             __constant_copy_to_user((to),(from),(n)) :     \
             __generic_copy_to_user((to),(from),(n)))


```
```
    /*
     * Sun people can't spell worth damn. "compatability" indeed.
     * At least we *know* we can't spell, and use a spell-checker.
     */

    /* Uh, actually Linus it is I who cannot spell. Too much murky
     * Sparc assembly will do this to ya.
     */
    C_LABEL(cputypvar):
            .asciz "compatibility"

    /* Tested on SS-5, SS-10. Probably someone at Sun applied a spell-checker. */
            .align 4
    C_LABEL(cputypvar_sun4m):
            .asciz "compatible"


```
```
            /* Sun, you just can't beat me, you just can't.  Stop trying,
     * give up.  I'm serious, I am going to kick the living shit
     * out of you, game over, lights out.
     */


```

## 致谢


感谢 Andi Kleen 提出的想法、回答我的问题、修正我的错误、充实内容等。感Philipp Rumpf 做了更多拼写与清晰度修正，并提出了一些极佳的非显而易见的观点。感Werner Almesberger 为我精彩地总结disable_irq()，以Jes Sorensen Andrea Arcangeli 补充的注意事项。感Michael Elizabeth Chastain 检查并补充Configure 一节。感Telsa Gwynne 教我 DocBook
