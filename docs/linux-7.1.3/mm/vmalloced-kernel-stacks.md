
## 虚拟映射内核栈支持


:Author: Shuah Khan <skhan@linuxfoundation.org>


### 概述


本资料汇集了引入 `Virtually Mapped Kernel Stacks feature
<https://lwn.net/Articles/694348/>`（虚拟映射内核栈特性）的代码与原始补丁系列
中的信息。

### 引言


内核栈溢出往往难以调试，并会使内核容易受到攻击。问题可能在较晚的时候才显现，
从而难以隔离和定位根因。

带有守护页（guard page）的虚拟映射内核栈能够立即捕获内核栈溢出，而不是导致难以
诊断的损坏。

HAVE_ARCH_VMAP_STACK 和 VMAP_STACK 配置选项启用对带有守护页的虚拟映射栈的支持。
该特性会在栈溢出时产生可靠的页错误。溢出后的栈回溯的可用性以及对溢出本身的
响应取决于具体架构。

        截至本文撰写时，arm64、powerpc、riscv、s390、um 和 x86 已支持 VMAP_STACK。

### HAVE_ARCH_VMAP_STACK


能够支持虚拟映射内核栈的架构应当启用这个布尔（bool）配置选项。其要求是：

- vmalloc 空间必须足够大以容纳众多内核栈。这可能会排除许多 32 位架构。
- 位于 vmalloc 空间中的栈需要可靠地工作。例如，如果 vmap 页表是按需创建的，则
  要么该机制在栈指向一个页表尚未填充的虚拟地址时仍能工作，要么架构代码
  （最可能是 switch_to() 和 switch_mm()）需要确保在可能尚未填充页表的栈上运行
  之前，栈的页表项已被填充。
- 如果栈溢出进入守护页，应当发生合理的事情。"合理"的定义较为灵活，但不经记录
  就立即重启是不友好的。

### VMAP_STACK


启用时，VMAP_STACK 布尔配置选项会分配虚拟映射的任务栈。该选项依赖于
HAVE_ARCH_VMAP_STACK。

- 若希望使用带有守护页的虚拟映射内核栈，请启用此选项。这会使内核栈溢出被立即
  捕获，而不是导致难以诊断的损坏。


        将此特性与 KASAN 一起使用需要架构支持用真实影子内存为虚拟映射提供
        后备，并且必须启用 KASAN_VMALLOC。


        启用 VMAP_STACK 后，无法对栈上分配的数据执行 DMA。

内核配置选项及其依赖关系不断变化。请参考最新的代码库：

`Kconfig <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/Kconfig>`

### 分配（Allocation）


当创建一个新的内核线程时，会从一个连续的虚拟内存页（来自页级分配器）分配线程栈。
这些页以 PAGE_KERNEL 保护属性被映射到连续的内核虚拟空间。

alloc_thread_stack_node() 调用 __vmalloc_node_range() 来分配带有 PAGE_KERNEL
保护属性的栈。

- 已分配的栈会被缓存，并被新线程后续复用，因此对 memcg 的记账是在将栈分配/释放
  给任务时手动进行的。因此，__vmalloc_node_range 是在未使用 __GFP_ACCOUNT 的
  情况下被调用的。
- vm_struct 被缓存，以便在中断上下文中发起线程释放时能够找到它。free_thread_stack()
  可以在中断上下文中被调用。
- 在 arm64 上，所有 VMAP 栈需要具有相同的对齐，以确保 VMAP 栈溢出检测正确工作。
  架构特定的 vmap 栈分配器负责处理这一细节。
- 这并未涉及中断栈——依据原始补丁。

线程栈的分配由 clone()、fork()、vfork()、kernel_thread() 经由 kernel_clone()
发起。以下是在代码库中搜索、以理解线程栈何时以及如何被分配的一些提示。

大部分代码位于：
`kernel/fork.c <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/kernel/fork.c>`。

task_struct 中的 stack_vm_area 指针用于跟踪虚拟分配的栈，一个非空的 stack_vm_area
指针可作为虚拟映射内核栈已启用的标志。

```

        struct vm_struct *stack_vm_area;

```
### 栈溢出处理


前导与尾部守护页有助于检测栈溢出。当栈溢出进入守护页时，处理程序必须小心不要再
次溢出栈。当处理程序被调用时，很可能只剩下极少的栈空间。

在 x86 上，这是通过在处理双重错误（double-fault）栈上、指示内核栈溢出的页错误
来完成的。

### 测试带守护页的 VMAP 分配


我们如何确保 VMAP_STACK 确实在分配带前导和尾部守护页的栈？以下 lkdtm 测试有助于
检测任何回归。

```

        void lkdtm_STACK_GUARD_PAGE_LEADING()
        void lkdtm_STACK_GUARD_PAGE_TRAILING()

```
### 结论


- 一个 vmalloced 栈的 percpu 缓存似乎比高阶栈分配略快，至少在缓存命中时如此。
- THREAD_INFO_IN_TASK 完全去掉了架构特定的 thread_info，而是简单地将
  thread_info（仅含标志）和 'int cpu' 内嵌进 task_struct。
- 线程栈可以在任务一死亡时就释放（无需等待 RCU），随后如果使用了 vmapped 栈，
  则可将整个栈缓存以便在同一 cpu 上复用。
