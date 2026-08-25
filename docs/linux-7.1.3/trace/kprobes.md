## 内核探针（Kprobes

:Author: Jim Keniston <jkenisto@us.ibm.com>
:Author: Prasanna S Panchamukhi <prasanna.panchamukhi@gmail.com>
:Author: Masami Hiramatsu <mhiramat@kernel.org>


  1. 概念：Kprobes Return Probes
  2. 支持的处理器架构
  3. 配置 Kprobes
  4. API 参  5. Kprobes 特性与限制
  6. 探针开销
  7. TODO
  8. Kprobes 示例
  9. Kretprobes 示例
  10. 已废弃特  Appendix A: kprobes debugfs 接口
  Appendix B: kprobes sysctl 接口
  Appendix C: 参考资
## 概念：Kprobes Return Probes


Kprobes 使你能够动态地插入到任意内核例程中，并以非侵入方式收集调试与性能信息你几乎可以在任何内核代码地址 [^1^]_ 处设置陷阱，并指定一个在命中断点时被调用的处理例程
       kprobes_blacklist)

目前有两种类型的探针：kprobes kretprobes（也return probes，返回探针）kprobe 可以插入到内核中几乎任意一条指令上。return probe 在指定函数返回时触发
在典型情况下，基Kprobes 的插桩被打包为一个内核模块模块init 函数安装注册"）一个或多个探针，exit 函数注销它们诸如 register_kprobe() 之类的注册函数指定探针插入的位置，以及探针命中时要调用的处理例程
还有 `register_/unregister_*probes()` 函数用于成批注册/注销一`*probes`当你需要一次性注销大量探针时，这些函数可以加快注销过程
接下来的四个小节解释了不同类型探针的工作方式，以及跳转优化是如何工作的它们解释了一些你为了最佳地使用 Kprobes 所需要了解的事情——例如，pre_handler post_handler 的区别，以及如何使用 kretprobe maxactive nmissed 字段但如果你急于开始使Kprobes，可以跳kprobes_archs_supported
### Kprobe 是如何工作的

当注册一kprobe 时，Kprobes 会复制被探测的指令，并用一条断点指令（例如 i386 x86_64 上的 int3）替换被探测指令的首字节（或多字节）
CPU 命中这条断点指令时，会发生一次陷阱，CPU 的寄存器被保存，控制权通过 notifier_call_chain 机制传递给 KprobesKprobes 执行与该 kprobe 关联"pre_handler"，并kprobe 结构以及保存的寄存器地址传给该处理例程
接着，Kprobes 单步执行它复制的那份被探测指令（原地单步执行实际指令本应更简单，但那Kprobes 就必须临时移除断点指令这样会打开一个很小的时间窗口，期间另一CPU 可能直接从探测点一掠而过。）

在指令被单步执行之后，Kprobes 执行与该 kprobe 关联"post_handler"（如果有的话）随后执行从探测点之后的指令继续
### 改变执行路径


由于 kprobes 可以探测运行中的内核代码，它能够改变寄存器集合，包括指令指针此操作需要极其小心，例如保持栈帧、恢复执行路径等。由于它作用于运行中的内核，并且需要深入的计算机体系结构与并发计算知识，你很容易搬起石头砸自己的脚
如果你在 pre_handler 中改变了指令指针（并设置了其他相关寄存器），你必须返!0，以kprobes 停止单步执行并直接返回到给定地址这也意味着不应再调post_handler
请注意，在某些使TOC（Table of Contents，目录表）进行函数调用的体系结构上，此操作可能更困难，因为你必须在你的模块中为你的函数建立一个新TOC，并在从中返回后恢复旧的 TOC
### Return Probes


##### Return Probe 是如何工作的

当你调用 register_kretprobe() 时，Kprobes 会在该函数的入口处建立一kprobe当被探测的函数被调用且命中该探针时，Kprobes 保存一份返回地址的副本，并将返回地址替换"trampoline"（蹦床）的地址trampoline 是一段任意代码——通常只是一nop 指令在启动阶段，Kprobes trampoline 处注册一kprobe
当被探测的函数执行其返回指令时，控制权传递给 trampoline，并命中该探针Kprobes trampoline 处理例程调用与该 kretprobe 关联的、用户指定的返回处理例程，然后将保存的指令指针设置为保存的返回地址，从陷阱返回时就在那里恢复执行
在被探测函数执行期间，其返回地址存储在一kretprobe_instance 类型的对象中在调register_kretprobe() 之前，用户设kretprobe 结构maxactive 字段，以指定该指定函数可以同时被探测的实例数量register_kretprobe() 预分配指定数量的 kretprobe_instance 对象
例如，如果函数是非递归的，并且是在持有自旋锁的情况下被调用，那maxactive = 1 就足够了如果函数是非递归的，并且永远不会放弃 CPU（例如，通过信号量或抢占），那么 NR_CPUS 就足够了如果 maxactive <= 0，则被设置为默认值：max(10, 2*NR_CPUS)
如果你把 maxactive 设置得太低，也不是什么灾难；你只是会错过一些探针kretprobe 结构中，nmissed 字段在注册返回探针时被设为零，并且每当被探测函数被进入但没有可用kretprobe_instance 对象来建立返回探针时递增
##### Kretprobe 入口处理例程（entry-handler

Kretprobes 还提供一个可选的、用户指定的处理例程，它在函数入口处运行该处理例程通过设置 kretprobe 结构entry_handler 字段来指定每当 kretprobe 放置在函数入口处kprobe 被命中时，就会调用用户定义的 entry_handler（如果有的话）如果 entry_handler 返回 0（成功），则保证在函数返回时会调用相应的返回处理例程如果 entry_handler 返回非零错误，则 Kprobes 保持返回地址不变，并且该特定函数实例kretprobe 不再产生进一步影响
多个入口与返回处理例程的调用通过与之关联的唯一 kretprobe_instance 对象进行匹配此外，用户还可以指定每个返回实例的私有数据，作为每个 kretprobe_instance 对象的一部分这在相应的用户入口与返回处理例程之间共享私有数据时特别有用每个私有数据对象的大小可以在注册 kretprobe 时通过设置 kretprobe 结构data_size 字段来指定这些数据可以通过每个 kretprobe_instance 对象data 字段访问
如果被探测的函数被进入但没有可用kretprobe_instance 对象，那么除了递增 nmissed 计数外，用户 entry_handler 的调用也会被跳过

### 跳转优化是如何工作的

如果你的内核CONFIG_OPTPROBES=y 构建（目前该标志x86/x86-64、非抢占式内核上自动设为 'y'），并且 "debug.kprobes_optimization" 内核参数被设1（参sysctl(8)），Kprobes 会尝试在每个探测点使用跳转指令取代断点指令，以降低探针命中开销
##### 初始化一Kprobe


当注册一个探针时，在尝试此优化之前，Kprobes 会在指定地址插入一个普通的、基于断点的 kprobe因此，即便无法优化这个特定的探测点，那里仍会有一个探针
##### 安全检

在优化一个探针之前，Kprobes 执行以下安全检查：

- Kprobes 验证将被跳转指令替换的区域（"优化区域"）完全位于一个函数内部  （跳转指令有多字节，因此可能覆盖多条指令。）

- Kprobes 分析整个函数，并验证没有跳转进入优化区域。具体而言
  - 函数不包含间接跳转；
  - 函数不包含会导致异常的指令（因为异常触发的修复代码可能跳回优化区域——Kprobes 会检查异常表来验证这一点）  - 没有到优化区域的近跳转（到首字节的跳转除外）
- 对于优化区域中的每条指令，Kprobes 验证该指令可out of line（异地）执行
##### 准备绕行缓冲区（Detour Buffer

接下来，Kprobes 准备一"detour"（绕行）缓冲区，其中包含以下指令序列
- 压入 CPU 寄存器的代码（模拟断点陷阱）
- trampoline 代码的调用，该代码调用用户的探针处理例程- 恢复寄存器的代码
- 来自优化区域的指- 跳回原始执行路径的指令
##### 预优

在准备好绕行缓冲区之后，Kprobes 验证不存在以下任何一种情况：

- 探针有一post_handler- 优化区域中的其他指令被探测- 探针被禁用
在上述任何情况下，Kprobes 都不会开始优化该探针由于这些都是临时情况，如果情况发生变化，Kprobes 会尝试再次开始优化它
如果 kprobe 可以被优化，Kprobes 将该 kprobe 排入一个优化列表，并踢kprobe-optimizer 工作队列来优化它如果待优化的探测点在被优化之前被命中，Kprobes 通过CPU 的指令指针设置为绕行缓冲区中复制的代码，将控制权返还给原始指令路径——从而至少避免了单步执行
##### 优化


Kprobe-optimizer 不会立即插入跳转指令相反，为了安全起见，它首先调synchronize_rcu()，因CPU 有可能在执行优化区域的中途被中断 [^3^]_如你所知，synchronize_rcu() 能够确保调用 synchronize_rcu() 时处于活动状态的所有中断都已完成，但只有在 CONFIG_PREEMPT=n 时才成立因此，这个版本的 kprobe 优化仅支CONFIG_PREEMPT=n 的内[^4^]_
之后，Kprobe-optimizer 调用 stop_machine()，使text_poke_smp() 将优化区域替换为一条指向绕行缓冲区的跳转指令
##### 取消优化


当一个被优化kprobe 被注销、禁用或被另一kprobe 阻塞时，它将被取消优化如果这种情况发生在优化完成之前，则该 kprobe 只是从优化列表中出队如果优化已经完成，则使用 text_poke_smp() 将跳转替换为原始代码（除了首字节中的 int3 断点）
   the optimizer replaces the 2nd instruction with the jump **address**
   while the interrupt handler is running. When the interrupt
   returns to original address, there is no valid instruction,
   and it causes an unexpected result.

   stop-machine method that ksplice uses for supporting a CONFIG_PREEMPT=y
   kernel.

极客须知跳转优化改变kprobe pre_handler 行为在没有优化的情况下，pre_handler 可以通过改变 regs->ip 并返1 来改变内核的执行路径然而，当探针被优化时，该修改会被忽略因此，如果你想要调整内核的执行路径，你需要使用以下任一技术来抑制优化
- kprobe post_handler 指定一个空函数
鎴。
- 执行 'sysctl -w debug.kprobes_optimization=n'


### 黑名单（Blacklist

Kprobes 可以探测除自身之外的大部分内核这意味着有一些函数是 kprobes 无法探测的。探测（陷阱）此类函数可能导致递归陷阱（例如双重故障），或者嵌套的探针处理例程可能永远不被调用Kprobes 将此类函数作为黑名单管理如果你想把一个函数加入黑名单，你只需要（1）包linux/kprobes.h，并且（2）使NOKPROBE_SYMBOL() 宏来指定一个被黑名单的函数Kprobes 会将给定的探针地址与黑名单比对，如果给定地址在黑名单中，则拒绝注册它

## 支持的处理器架构


Kprobes return probes 在以下体系结构上实现
- i386（支持跳转优化）
- x86_64（AMD-64, EM64T）（支持跳转优化- ppc64
- sparc64（尚未实Return probes。）
- arm
- ppc
- mips
- s390
- parisc
- loongarch
- riscv

## 配置 Kprobes


使用 make menuconfig/xconfig/oldconfig 配置内核时，确保 CONFIG_KPROBES 被设"y"，在 "General architecture-dependent options" 下查"Kprobes"
为了能够加载与卸载基Kprobes 的插桩模块，确保 "Loadable module support"（CONFIG_MODULES）与 "Module unloading"（CONFIG_MODULE_UNLOAD）被设为 "y"
同时，确CONFIG_KALLSYMS，甚CONFIG_KALLSYMS_ALL 也被设为 "y"，因为内核内kprobe 地址解析代码使用kallsyms_lookup_name()
如果你需要在函数中间插入一个探针，你可能会发现 "Compile the kernel with debug info"（CONFIG_DEBUG_INFO）很有用，这样你就可以使"objdump -d -l vmlinux" 来查看源码到目标代码的映射
## API 参

Kprobes API 为每种类型的探针各包含一"register"（注册）函数与一"unregister"（注销）函数API 还包"register_*probes" "unregister_*probes" 函数，用于（反）注册探针数组以下是对这些函数以及你将编写的关联探针处理例程的简明、迷你手册式规范有关示例，请参见 samples/kprobes/ 子目录中的文件
### register_kprobe


```
	#include <linux/kprobes.h>
	int register_kprobe(struct kprobe *kp);
```

在地址 kp->addr 处设置一个断点。当断点被命中时，Kprobes 调用 kp->pre_handler在被探测指令被单步执行之后，Kprobe 调用 kp->post_handler任何或全部处理例程都可以NULL。如果设置了 kp->flags KPROBE_FLAG_DISABLED，则kp 将被注册但处于禁用状态，因此在调enable_kprobe(kp) 之前不会命中它的处理例程

   1. 随着 "symbol_name" 字段被引入到 struct kprobe 中，探测点地址解析现在由内核负责```
	kp.symbol_name = "symbol_name";
```

      （诸如函数描述符之类64 powerpc 细节会被透明地处理）

   2. 如果你知道安装探测点的符号内偏移，可使用 struct kprobe "offset" 字段。该字段用于计算探测点
   3. 只能指定 kprobe "symbol_name" "addr" 二者之一。如果两者都指定，kprobe 注册将以 -EINVAL 失败
   4. 对于 CISC 体系结构（如 i386 x86_64），kprobes 代码不会验证 kprobe.addr 是否位于指令边界上      使用 "offset" 时请谨慎
```
register_kprobe() 在成功时返回 0，否则返回一个负errno```

```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	int pre_handler(struct kprobe *p, struct pt_regs *regs);
```

p 指向与断点关联的 kprobe，regs 指向保存断点命中时寄存器的结构除非你是一Kprobes 极客，否则在这里返回 0
```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	void post_handler(struct kprobe *p, struct pt_regs *regs,
			  unsigned long flags);
```

p regs 的描述与 pre_handler 相同。flags 似乎总是为零
### register_kretprobe


```
	#include <linux/kprobes.h>
	int register_kretprobe(struct kretprobe *rp);
```

为地址rp->kp.addr 的函数建立一个返回探针当该函数返回时，Kprobes 调用 rp->handler在调register_kretprobe() 之前，你必须适当地设rp->maxactive；详"Return Probe 是如何工作的
register_kretprobe() 在成功时返回 0，否则返回一个负errno

```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	int kretprobe_handler(struct kretprobe_instance *ri,
			      struct pt_regs *regs);
```

regs 的描述同 kprobe.pre_handler。ri 指向 kretprobe_instance 对象，其中以下字段可能令人感兴趣
- ret_addr：返回地址
- rp：指向对应的 kretprobe 对象
- task：指向对应的 task struct
- data：指向每个返回实例的私有数据；详"Kretprobe entry-handler"
regs_return_value(regs) 宏提供了一个简单的抽象，用于按照体系结ABI 的定义，从适当的寄存器中提取返回值
该处理例程的返回值目前被忽略
### unregister_*probe


```
	#include <linux/kprobes.h>
	void unregister_kprobe(struct kprobe *kp);
	void unregister_kretprobe(struct kretprobe *rp);
```

移除指定的探针。在探针注册之后的任何时刻都可以调用注销函数

   If the functions find an incorrect probe (ex. an unregistered probe),
   they clear the addr field of the probe.

### register_*probes


```
	#include <linux/kprobes.h>
	int register_kprobes(struct kprobe **kps, int num);
	int register_kretprobes(struct kretprobe **rps, int num);
```

注册指定数组中的 num 个探针。如果在注册过程中发生任何错误，在该错误探针之前的所有数组探针都会在 register_*probes 函数返回之前被安全地注销
- kps/rps：一个指`*probe` 数据结构的指针数- num：数组条目的数量

   You have to allocate(or define) an array of pointers and set all
   of the array entries before using these functions.

### unregister_*probes


```
	#include <linux/kprobes.h>
	void unregister_kprobes(struct kprobe **kps, int num);
	void unregister_kretprobes(struct kretprobe **rps, int num);
```

一次性移除指定数组中num 个探针

   If the functions find some incorrect probes (ex. unregistered
   probes) in the specified array, they clear the addr field of those
   incorrect probes. However, other probes in the array are
   unregistered correctly.

### disable_*probe


```
	#include <linux/kprobes.h>
	int disable_kprobe(struct kprobe *kp);
	int disable_kretprobe(struct kretprobe *rp);
```

临时禁用指定`*probe`。你可以通过 enable_*probe() 再次启用它。你必须指定已被注册的探针
### enable_*probe


```
	#include <linux/kprobes.h>
	int enable_kprobe(struct kprobe *kp);
	int enable_kretprobe(struct kretprobe *rp);
```

启用disable_**probe() 禁用掉的 `**probe`。你必须指定已被注册的探针
## Kprobes 特性与限制


Kprobes 允许在同一地址上有多个探针此外，带post_handler 的探测点无法被优化因此，如果你在一个已优化的探测点上安装一个带post_handler kprobe，该探测点将被自动取消优化
一般而言，你可以在内核中的任何位置安装探针特别是，你可以探测中断处理例程。本节讨论已知的例外情况
如果你试图在实现Kprobes 的代码（主要kernel/kprobes.c `arch/*/kernel/kprobes.c`，但也包do_page_fault notifier_call_chain 之类的函数）中安装探针，register_*probe 函数将返-EINVAL
如果你在一个可内联的函数中安装探针，Kprobes 不会试图追踪该函数所有的内联实例并在那里安装探针gcc 可能在未被请求的情况下内联一个函数，所以如果你没有看到预期的探针命中，请记住这一点
探针处理例程可以修改被探测函数的环境——例如，通过修改内核数据结构，或者修pt_regs 结构的内容（这些内容在从断点返回时会被恢复到寄存器）因此，Kprobes 可以被用来，例如，安装一bug 修复，或者注入故障用于测试当然，Kprobes 无法区分蓄意注入的故障与意外故障。请勿酒后探针
Kprobes 不会试图阻止探针处理例程互相踩踏——例如，探测 printk() 然后从探针处理例程中调用 printk()如果一个探针处理例程命中了一个探针，那么在该实例中第二个探针的处理例程将不会运行，并且第二个探针kprobe.nmissed 成员将被递增
Linux v2.6.15-rc1 起，多个处理例程（或同一处理例程的多个实例）可以在不同的 CPU 上并发运行
Kprobes 不使用互斥体，也不分配内存，除非在注册与注销期间
探针处理例程在抢占禁用或中断禁用的状态下运行，这取决于体系结构与优化状态。（例如，在 x86/x86-64 上，kretprobe 处理例程与优化后kprobe 处理例程是在中断未禁用的情况下运行的。）
无论如何，你的处理例程不应放CPU（例如，试图获取信号量，或等I/O）
由于返回探针是通过将返回地址替换trampoline 的地址来实现的，栈回溯与对 __builtin_return_address() 的调用，通常会为kretprobe 探测的函数给trampoline 的地址，而不是真实的返回地址（据我们所知，__builtin_return_address() 仅用于插桩与错误报告。）

如果一个函数的调用次数与返回次数不匹配，在该函数上注册返回探针可能会产生不良结果在这种情况下，会打印一行：
kretprobe BUG!: Processing kretprobe d000000000041aa8 @ c00000000004f48c
凭借这些信息，人们就能将引起问题的 kretprobe 的确切实例关联起来我们已经覆盖do_exit() 的情况。do_execve() do_fork() 不是问题我们尚不清楚其他可能产生此问题的具体情况
如果在一个函数进入或退出时，CPU 运行在当前任务以外的栈上，在该函数上注册返回探针可能会产生不良结果出于这个原因，Kprobes 不支持在 x86_64 版本__switch_to() 上使用返回探针（kprobes）；注册函数返回 -EINVAL
x86/x86-64 上，由于 Kprobes 的跳转优化会大范围地修改指令，优化存在一些限制为了解释它，我们引入一些术语。想象一个由两条 2 字节指令与一3 字节指令组成的三指令序列
```
		IA
		|
	[-2][-1][0][1][2][3][4][5][6][7]
		[ins1][ins2][  ins3 ]
		[<-     DCR       ->]
		[<- JTPR ->]

	ins1: 1st Instruction
	ins2: 2nd Instruction
	ins3: 3rd Instruction
	IA:  Insertion Address
	JTPR: Jump Target Prohibition Region
	DCR: Detoured Code Region
```

DCR 中的指令被复制到 kprobe out-of-line 缓冲区中，因DCR 中的字节被一5 字节跳转指令替换。因此存在若干限制
a) DCR 中的指令必须可重定位b) DCR 中的指令不得包含调用指令c) JTPR 不得作为任何跳转或调用指令的目标d) DCR 不得跨越函数之间的边界
无论如何，这些限制都由内核内的指令解码器检查，所以你无需为此担心
## 探针开销


2005 年使用的一款典CPU 上，一kprobe 命中需0.5 1.0 微秒来处理具体而言，一个重复命中同一探测点、每次触发一个简单处理例程的基准测试报告每秒 1-2 百万次命中，具体取决于体系结构返回探针命中通常kprobe 命中多花 50-75% 的时间当在一个函数上设置了返回探针时，在该函数入口处再添加一kprobe 基本上不会增加开销
```
  k = kprobe; r = return probe; kr = kprobe + return probe
  on same function

  i386: Intel Pentium M, 1495 MHz, 2957.31 bogomips
  k = 0.57 usec; r = 0.92; kr = 0.99

  x86_64: AMD Opteron 246, 1994 MHz, 3971.48 bogomips
  k = 0.49 usec; r = 0.80; kr = 0.82

  ppc64: POWER5 (gr), 1656 MHz (SMT disabled, 1 virtual CPU per physical CPU)
  k = 0.77 usec; r = 1.26; kr = 1.45
```

### 优化后的探针开销


通常，一次优化后kprobe 命中需0.07 0.1 微秒来处理```
  k = unoptimized kprobe, b = boosted (single-step skipped), o = optimized kprobe,
  r = unoptimized kretprobe, rb = boosted kretprobe, ro = optimized kretprobe.

  i386: Intel(R) Xeon(R) E5410, 2.33GHz, 4656.90 bogomips
  k = 0.80 usec; b = 0.33; o = 0.05; r = 1.10; rb = 0.61; ro = 0.33

  x86-64: Intel(R) Xeon(R) E5410, 2.33GHz, 4656.90 bogomips
  k = 0.99 usec; b = 0.43; o = 0.06; r = 1.24; rb = 0.68; ro = 0.30
```

## TODO


a. SystemTap (http://sourceware.org/systemtap)：为基于探针的插桩提供简化的编程接口。试一试b. sparc64 的内核返回探针c. 对其他体系结构的支持d. 用户空间探针e. 监视点探针（在数据引用时触发）
## Kprobes 示例


参见 samples/kprobes/kprobe_example.c

## Kretprobes 示例


参见 samples/kprobes/kretprobe_example.c

## 已废弃特

Jprobes 现在是一个已废弃的特性。依赖它的人应当迁移到其他追踪特性，或者使用更旧的内核。请考虑将你的工具迁移到以下选项之一
- 使用 trace-event 来追踪带有参数的目标函数
  trace-event 是一个低开销（关闭时几乎不可见开销）的静态定义事件接口  你可以定义新事件，并通过 ftrace 或任何其他追踪工具追踪它
  参见以下网址
    - https://lwn.net/Articles/379903/
    - https://lwn.net/Articles/381064/
    - https://lwn.net/Articles/383362/

- ftrace 动态事件（kprobe event）与 perf-probe 一起使用
  如果你以调试信息构建内核（CONFIG_DEBUG_INFO=y），你可以通过 perf-probe 找到哪个寄存栈被分配给哪个局部变量或参数，并建立新事件来追踪它
  参见以下文档
  - Documentation/trace/kprobetrace.rst
  - Documentation/trace/events.rst
  - tools/perf/Documentation/perf-probe.txt


## kprobes debugfs 接口


随着较新的内核（> 2.6.20），已注kprobes 的列表在 /sys/kernel/debug/kprobes/ 目录下可见（假定 debugfs 挂载//sys/kernel/debug）
```
	c015d71a  k  vfs_read+0x0
	c03dedc5  r  tcp_v4_rcv+0x0
```

第一列提供探针插入的内核地址第二列标识探针的类型（k - kprobe r - kretprobe），第三列指定探针的 symbol+offset如果被探测的函数属于某个模块，也会指定模块名。后续列显示探针状态。如果探针位于一个不再有效的虚拟地址上（模块 init 段、对应于已被卸载模块的模块虚拟地址），这样的探针被标记[GONE]。如果探针被临时禁用，这样的探针被标记为 [DISABLED]。如果探针被优化，它被标记为 [OPTIMIZED]。如果探针是基于 ftrace 的，它被标记[FTRACE]
/sys/kernel/debug/kprobes/enabled：强制开关闭 kprobes
提供一个旋钮，用于全局地、强制性地开启或关闭已注册的 kprobes默认情况下，所kprobes 都是启用的。通过向该文件回显 "0"，所有已注册的探针将被解除武装，直到向该文件回显 "1" 为止请注意，这个旋钮只是解除武装和武装所kprobes，并不改变每个探针的禁用状态。这意味着，如果你通过这个旋钮开启所kprobes，被禁用kprobes（标记为 [DISABLED]）不会被启用

## kprobes sysctl 接口


/proc/sys/debug/kprobes-optimization：开关闭 kprobes 优化
CONFIG_OPTPROBES=y 时，会出现这sysctl 接口，它提供一个旋钮，用于全局地、强制性地开启或关闭跳转优化（参kprobes_jump_optimization 小节）默认情况下，跳转优化是允许的（ON）。如果你向该文件回显 "0"，或者通过 sysctl "debug.kprobes_optimization" 设为 0，所有已优化的探针将被取消优化，并且此后注册的任何新探针将不会被优化
请注意，这个旋钮**改变**优化状态。这意味着已优化的探针（标记为 [OPTIMIZED]）将被取消优化（[OPTIMIZED] 标签将被移除）。如果该旋钮被打开，它们将被再次优化
## 参考资

有关 Kprobes 的更多信息，请参考以URL
- https://lwn.net/Articles/132196/
- https://www.kernel.org/doc/ols/2006/ols2006v2-pages-109-124.pdf
