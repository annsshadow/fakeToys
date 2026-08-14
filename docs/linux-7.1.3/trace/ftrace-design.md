## 函数跟踪器（Function Tracer）设计


:Author: Mike Frysinger

	本文档已经过时。下面描述的某些内容已经与当前的实现不符。

### 简介


这里我们介绍公共函数跟踪代码赖以正常工作的架构相关部分。内容按复杂度递增
组织，以便你可以从简单入手，至少获得基本功能。

注意，本文档只关注架构实现细节。如果你希望从公共代码角度了解某个功能的
更多说明，请查阅 ftrace.txt 文件。

理想情况下，任何希望在支持跟踪的同时保持性能的内核，都应该一路做到支持
动态 ftrace。

### 先决条件


ftrace 依赖于以下特性的实现：
  - STACKTRACE_SUPPORT - 实现 save_stack_trace()
  - TRACE_IRQFLAGS_SUPPORT - 实现 include/asm/irqflags.h

### HAVE_FUNCTION_TRACER


你需要实现 mcount 和 ftrace_stub 函数。

具体的 mcount 符号名取决于你的工具链。有的叫 “mcount”、“_mcount”，甚至
“__mcount”。你大概可以通过下面的方式查出来：
```
	$ echo 'main(){}' | gcc -x c -S -o - - -pg | grep mcount
	        call    mcount
```
为了下方示例简洁清晰，我们假定符号名为 “mcount”。

请记住，mcount 函数内部生效的 ABI 是 **高度** 依赖于架构/工具链的。这方面
我们帮不了你，抱歉。请翻出一些老文档，或者找个比你更熟悉的人一起探讨。通常
情况下，寄存器的使用（参数/临时/等等）在这一点上是主要问题，尤其是与 mcount
调用位置（在函数序言之前/之后）相关时。你还可能想看看 glibc 是如何为你的
架构实现 mcount 函数的，或许（半）相关。

mcount 函数应检查函数指针 ftrace_trace_function，看它是否被设置为
ftrace_stub。如果是，那你无事可做，直接返回即可。如果不是，则像 mcount 函数
通常调用 __mcount_internal 那样调用该函数——第一个参数是 “frompc”，第二个
参数是 “selfpc”（已调整以去除内嵌于函数中的 mcount 调用的大小）。

例如，若函数 foo() 调用 bar()，当 bar() 函数调用 mcount() 时，mcount() 将
传递给跟踪器的参数为：

  - “frompc” - bar() 用来返回 foo() 的地址
  - “selfpc” - bar() 的地址（已做 mcount() 大小调整）

还要记住，这个 mcount 函数会被 **频繁** 调用，因此针对无跟踪器的默认情况进行
优化，将有助于在禁用跟踪时系统的平稳运行。所以 mcount 函数的开头通常只做
最少量的检查便返回。这也意味着代码流程通常应保持线性（即在 nop 情况下不
分支）。这当然是一种优化而非硬性要求。

下面是一些应该有帮助的伪代码（这些函数实际上应当
```
	void ftrace_stub(void)
	{
		return;
	}

	void mcount(void)
	{
		/* save any bare state needed in order to do initial checking */

		extern void (*ftrace_trace_function)(unsigned long, unsigned long);
		if (ftrace_trace_function != ftrace_stub)
			goto do_trace;

		/* restore any bare state */

		return;

	do_trace:

		/* save all state needed by the ABI (see paragraph above) */

		unsigned long frompc = ...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;
		ftrace_trace_function(frompc, selfpc);

		/* restore all state needed by the ABI */
	}
```
别忘了为模块导出 mcount！
```
	extern void mcount(void);
	EXPORT_SYMBOL(mcount);

```
### HAVE_FUNCTION_GRAPH_TRACER


深吸一口气……是时候干点真活了。这里你需要更新 mcount 函数以检查 ftrace 图
函数指针，并实现一些函数来保存（劫持）与恢复返回地址。

mcount 函数应检查函数指针 ftrace_graph_return（与 ftrace_stub 比较）和
ftrace_graph_entry（与 ftrace_graph_entry_stub 比较）。如果其中任意一个未被
设为相应的 stub 函数，则调用架构相关的函数 ftrace_graph_caller，后者进而
调用架构相关的函数 prepare_ftrace_return。这两个函数名都不是硬性要求的，但你
仍应使用它们，以在不同架构移植之间保持一致性——便于比较和对照。

prepare_ftrace_return 的参数与传给 ftrace_trace_function 的略有不同。第二个
参数 “selfpc” 相同，但第一个参数应是指向 “frompc” 的指针。通常它位于栈上。
这使得该函数可以临时劫持返回地址，使其指向架构相关的函数 return_to_handler。
该函数只需调用公共的 ftrace_return_to_handler 函数，它将返回原始的返回地址，
据此你可以返回到原始的调用点。
```
	void mcount(void)
	{
	...
		if (ftrace_trace_function != ftrace_stub)
			goto do_trace;

	+#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	+	extern void (*ftrace_graph_return)(...);
	+	extern void (*ftrace_graph_entry)(...);
	+	if (ftrace_graph_return != ftrace_stub ||
	+	    ftrace_graph_entry != ftrace_graph_entry_stub)
	+		ftrace_graph_caller();
	+#endif

		/* restore any bare state */
	...
```
```
	#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	void ftrace_graph_caller(void)
	{
		/* save all state needed by the ABI */

		unsigned long *frompc = &...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;
		/* passing frame pointer up is optional -- see below */
		prepare_ftrace_return(frompc, selfpc, frame_pointer);

		/* restore all state needed by the ABI */
	}
	#endif
```
关于如何实现 prepare_ftrace_return()，只需查看 x86 版本即可（frame pointer
的传递是可选的；详见下一节）。其中唯一架构相关的部分是错误恢复表（即
asm(...) 代码）的搭建。其余部分在各架构间应当相同。

下面是新的 return_to_handler 汇编函数的伪代码。注意，这里适用的 ABI 与 mcount
代码适用的不同。由于你是从一个函数返回（在尾声之后），你可能可以省去部分
保存/恢复的状态（通常只是用于传递返回值的寄存器）。
```
	#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	void return_to_handler(void)
	{
		/* save all state needed by the ABI (see paragraph above) */

		void (*original_return_point)(void) = ftrace_return_to_handler();

		/* restore all state needed by the ABI */

		/* this is usually either a return or a jump */
		original_return_point();
	}
	#endif

```
### HAVE_FUNCTION_GRAPH_FP_TEST


一个架构可以向函数的进入与退出传入一个唯一的值（frame pointer）。在退出时，
该值会被比较，如果不匹配，则会让内核 panic。这主要是对 gcc 错误代码生成的一
种健全性检查。如果你的移植版本在 gcc 不同优化级别下能合理地更新 frame pointer，
那么可以忽略此选项。

不过，为其添加支持并不太难。在你调用 prepare_ftrace_return() 的汇编代码中，
将 frame pointer 作为第 3 个参数传入。然后在那个函数的 C 版本中，像 x86 移植
那样，将其传递给 ftrace_push_return_trace()，而不是传入 stub 值 0。

类似地，当你调用 ftrace_return_to_handler() 时，将 frame pointer 传给它。

### HAVE_SYSCALL_TRACEPOINTS


你只需要很少的东西就能在某个架构上获得系统调用跟踪。

  - 支持 HAVE_ARCH_TRACEHOOK（见 arch/Kconfig）。
  - 在 <asm/unistd.h> 中有一个 NR_syscalls 变量，提供该架构支持的系统调用
    数量。
  - 支持 TIF_SYSCALL_TRACEPOINT 线程标志。
  - 在 ptrace 的系统调用跟踪路径中，从 ptrace 调用 trace_sys_enter() 和
    trace_sys_exit() tracepoint。
  - 如果该架构上的系统调用表比一个地址的简单数组更复杂，则实现一个
    arch_syscall_addr 以返回给定系统调用的地址。
  - 如果该架构上系统调用的符号名与函数名不匹配，则在 asm/ftrace.h 中定义
    ARCH_HAS_SYSCALL_MATCH_SYM_NAME 并实现 arch_syscall_match_sym_name，加入
    适当的逻辑：若功能名与符号名对应则返回 true。
  - 将该架构标记为 HAVE_SYSCALL_TRACEPOINTS。

### HAVE_DYNAMIC_FTRACE


详见 scripts/recordmcount.pl。只需填写架构相关细节，说明如何通过 objdump 定位
mcount 调用点的地址。不实现动态 ftrace 的话，此选项意义不大。

你首先需要 HAVE_FUNCTION_TRACER，所以如果你过于心急，请把阅读器往回滚。

一旦这些就绪，你需要实现：
 - asm/ftrace.h:
  - MCOUNT_ADDR
  - ftrace_call_adjust()
  - struct dyn_arch_ftrace{}
 - asm 代码:
  - mcount()（新 stub）
  - ftrace_caller()
  - ftrace_call()
  - ftrace_stub()
 - C 代码:
  - ftrace_dyn_arch_init()
  - ftrace_make_nop()
  - ftrace_make_call()
  - ftrace_update_ftrace_func()

首先你需要在 asm/ftrace.h 中填写一些架构细节。
```
	#define MCOUNT_ADDR ((unsigned long)mcount)
```
```
	extern void mcount(void);
```
你还需要辅助函数 ftrace_call_adjust()。大多数人
```
	static inline unsigned long ftrace_call_adjust(unsigned long addr)
	{
		return addr;
	}
```
<details to be filled>

最后，你需要自定义的 dyn_arch_ftrace 结构体。如果在运行时给任意调用点打补丁时
需要一些额外状态，这就是
```
	struct dyn_arch_ftrace {
		/* No extra data needed */
	};
```
头文件处理完后，我们可以填写汇编代码。虽然前面我们已经创建了 mcount() 函数，
但动态 ftrace 只需要一个 stub 函数。这是因为 mcount() 只会在启动期间使用，之后
所有对它的引用都会被打补丁替换掉，永不返回。取而代之的是，旧 mcount() 的核心
将被用来创建一个新的 ftrace_caller() 函数。由于二者难以合并，最省事的办法大概
是用 #ifdef 分成两个独立的定义。ftrace_stub() 也是如此，因为它现在将被内联进
ftrace_caller()。

在更困惑之前，我们先看一些伪代码，以便你
```
	void mcount(void)
	{
		return;
	}

	void ftrace_caller(void)
	{
		/* save all state needed by the ABI (see paragraph above) */

		unsigned long frompc = ...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;

	ftrace_call:
		ftrace_stub(frompc, selfpc);

		/* restore all state needed by the ABI */

	ftrace_stub:
		return;
	}
```
这乍看可能有点奇怪，但请记住我们将在运行时打补丁多处。首先，只有我们真正想
跟踪的函数才会被打补丁以调用 ftrace_caller()。其次，由于我们同一时间只激活一个
跟踪器，我们会给 ftrace_caller() 函数本身打补丁，以调用相关的那个跟踪器。这正
是 ftrace_call 标签的用途。

鉴于此，让我们继续看真正执行运行时打补丁的 C 代码。要度过下一节，你需要对
自己架构的操作码有一点了解。

每个架构都有一个 init 回调函数。如果你需要尽早做些初始化状态的工作，这就是
时机。否则，这个简单的
```
	int __init ftrace_dyn_arch_init(void)
	{
		return 0;
	}
```
有两个函数用于对任意函数进行运行时打补丁。第一个用于把 mcount 调用点变成 nop
（这正有助于我们在不跟踪时保持运行时性能）。第二个用于把 mcount 调用点变成
对某个任意位置的调用（但通常那是 ftracer_caller()）。参见
```
	ftrace_make_nop()
	ftrace_make_call()
```
rec->ip 值是在构建期由 scripts/recordmcount.pl 收集的 mcount 调用点地址。

最后一个函数用于对活动的跟踪器进行运行时打补丁。它将修改 ftrace_caller()
函数内 ftrace_call 符号所在位置的汇编代码。因此你应在该位置保留足够的填充
（padding）以支持将要插入的新函数调用。有人会用 “call” 类指令，也有人会用
```
	ftrace_update_ftrace_func()


```
### HAVE_DYNAMIC_FTRACE + HAVE_FUNCTION_GRAPH_TRACER


函数跟踪图器（function grapher）需要一些微调才能与动态 ftrace 配合工作。基本
上，你需要：

 - 更新：
  - ftrace_caller()
  - ftrace_graph_call()
  - ftrace_graph_caller()
 - 实现：
  - ftrace_enable_ftrace_graph_caller()
  - ftrace_disable_ftrace_graph_caller()

<details to be filled>

简要说明：

 - 在 ftrace_call 位置之后添加一个名为 ftrace_graph_call 的 nop stub；
	  该 stub 需要足够大，以支持对 ftrace_graph_caller() 的调用
 - 更新 ftrace_graph_caller() 以配合被新的 ftrace_caller() 调用，因为部分语义
	  可能已改变
 - ftrace_enable_ftrace_graph_caller() 会在运行时将 ftrace_graph_call 位置
	  打补丁为对 ftrace_graph_caller() 的调用
 - ftrace_disable_ftrace_graph_caller() 会在运行时将 ftrace_graph_call 位置
	  打补丁为 nops
