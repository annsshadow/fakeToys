## 内核中的 CPU 热插

:Date: September, 2021
:Author: Sebastian Andrzej Siewior <bigeasy@linutronix.de>,
         Rusty Russell <rusty@rustcorp.com.au>,
         Srivatsa Vaddagiri <vatsa@in.ibm.com>,
         Ashok Raj <ashok.raj@intel.com>,
         Joel Schopp <jschopp@austin.ibm.com>,
	 Thomas Gleixner <tglx@kernel.org>

## 简

现代系统架构的进步在处理器中引入了先进的错误报告与纠正能力有一OEM 支持同样可热插拔NUMA 硬件，其中物理节点的插入与移除需CPU 热插拔的支持
此类进步要求内核可用CPU 出于资源调配原因，或出于 RAS 目的（使有问题的 CPU 远离系统执行路径）而被移除因此需要在 Linux 内核中支CPU 热插拔
CPU 热插拔支持一个更新颖的用途是今天SMP 的挂恢复支持中的使用双核与超线程（HT）支持使得即便是笔记本电脑也能运行原本不支持这些方法SMP 内核

## 命令行开

`maxcpus=n`
  将启动时CPU 限制**n**。例如，如果你有四颗 CPU，使  `maxcpus=2` 将只启动两颗。你可以选择稍后  其他 CPU 上线
`nr_cpus=n`
  限制内核将支持的 CPU 总数。如果此处提供的数字低于物理可用 CPU 的数量，那么
  这些 CPU 稍后也无法被上线
`possible_cpus=n`
  此选项`cpu_possible_mask` 中设`possible_cpus` 位
  此选项仅限X86 S390 架构
`cpu0_hotplug`
  允许关闭 CPU0
  此选项仅限X86 架构
## CPU 映射


`cpu_possible_mask`
  系统中曾经可用的可能 CPU 的位图。这用于在启动时为那些并非设计成CPU   可用或移除而增收缩per_cpu 变量分配一些启动期内存  一旦在启动期的发现阶段设置，该映射就是静态的，即任何时候都不会添加或移除位  为你的系统需求提前精确地裁剪它可以节省一些启动期内存
`cpu_online_mask`
  当前所有在CPU 的位图。它在一CPU 可用于内核调度并准备好接收来自设备的
  中断之后，于 `__cpu_up()` 中设置。当一CPU 通过
  `__cpu_disable()` 被关闭时，它被清除，在此之前的包括中断在内的所OS 服务
  都被迁移到另一个目CPU
`cpu_present_mask`
  当前系统中存在的 CPU 的位图。并非所  的它们都在线。当相关的子系统（例ACPI）处理物理热插拔时，会根据事件是热添热移  而改变，从映射中新增或移除一个位。目前没有任何锁定规则  典型用法是在启动时初始化拓扑，此时热插拔被禁用
你真的不需要去操纵任何系统 CPU 映射。对于大多数用途，它们应当是只读的在建per-cpu 资源时，几乎总是使用 `cpu_possible_mask` `for_each_possible_cpu()`
来迭代。宏 `for_each_cpu()` 可用于迭代一个自定义CPU 掩码
除了 `cpumask_t` 之外，绝不要使用任何其他东西来表CPU 的位图

## 使用 CPU 热插

需要启用内核选项 **CONFIG_HOTPLUG_CPU**。它目前在包ARM、MIPS、PowerPC X86 在内的多种架构上可用```
 $ ls -lh /sys/devices/system/cpu
 total 0
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu0
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu1
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu2
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu3
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu4
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu5
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu6
 drwxr-xr-x  9 root root    0 Dec 21 16:33 cpu7
 drwxr-xr-x  2 root root    0 Dec 21 16:33 hotplug
 -r--r--r--  1 root root 4.0K Dec 21 16:33 offline
 -r--r--r--  1 root root 4.0K Dec 21 16:33 online
 -r--r--r--  1 root root 4.0K Dec 21 16:33 possible
 -r--r--r--  1 root root 4.0K Dec 21 16:33 present
```
文件 **offline***online***possible***present** 代表 CPU 掩码每个 CPU 文件夹都包含一**online** 文件，它控制逻辑上的开）与
```
 $ echo 0 > /sys/devices/system/cpu/cpu4/online
  smpboot: CPU 4 is now offline
```
一CPU 被关闭，它将**/proc/interrupts****/proc/cpuinfo** 中移除，并且也不应再**top** 命令可见。要
```
 $ echo 1 > /sys/devices/system/cpu/cpu4/online
 smpboot: Booting Node 0 Processor 4 APIC 0x1
```
CPU 再次可用。这应当适用于所CPU，但 CPU0 通常比较特殊，被排除CPU 热插拔之外
## CPU 热插拔协

### 离线情形


一旦一CPU 被逻辑上关闭，已注册的
热插拔状态的拆除回调就会被调用，`CPUHP_ONLINE` 开始，到状`CPUHP_OFFLINE` 结束。这包括
- 如果由于挂起操作导致任务被冻结，**cpuhp_tasks_frozen**
  会被设为 true- 所有进程都从该离线CPU 迁移到新CPU  新的 CPU 从每个进程当前的 cpuset 中选择，它可能是在CPU 的一个子集- 所有以CPU 为目标的中断都被迁移到一个新CPU
- 定时器也被迁移到一个新CPU
- 一旦所有服务都被迁移，内核调用一arch 特定的例  `__cpu_disable()` 来执arch 特定的清理

## CPU 热插API


### CPU 热插拔状态机


CPU 热插拔使用一个从 CPUHP_OFFLINE CPUHP_ONLINE 的线性状态空间的简单状态机每个状态都有一个启动（startup）和一个拆除（teardown）回调
当一CPU 被上线时，启动回调被顺序调用，直到达到状CPUHP_ONLINE当某个状态的回调被建立，或者一个实例被添加到一个多实例状态时，它们也可以被调用
当一CPU 被离线时，拆除回调以相反的顺序被顺序调用，直到达到状CPUHP_OFFLINE当某个状态的回调被移除，或者一个实例从多实例状态被移除时，它们也可以被调用
如果一个使用点只需要在热插拔操作的一个方向（CPU 上线CPU 离线）上有一个回调，
那么另一个不需要的回调可以在建立该状态时设为 NULL
状态空间被划分为三个部分：

- PREPARE 部分

  PREPARE 部分覆盖CPUHP_OFFLINE   CPUHP_BRINGUP_CPU 的状态空间
  该部分中的启动回调在 CPU 上线操作期间、CPU 启动之前被调用  拆除回调CPU 离线操作期间、CPU 变得不可用之后被调用
  回调在一个控CPU 上被调用，因为它们显然无法运行在要么尚未启动、要么已经变得不可用的热插拔 CPU 上
  启动回调用于建立成功让一CPU 上线所需的资源。拆除回调用于在热插CPU 变得不可用之后释放资源，或者将待处理的工作移动到一个在线的 CPU
  启动回调允许失败。如果某个回调失败，CPU 上线操作被中止，并且CPU 再次被降到之前的状态（通常CPUHP_OFFLINE）
  该部分中的拆除回调不允许失败
- STARTING 部分

  STARTING 部分覆盖CPUHP_BRINGUP_CPU + 1
  CPUHP_AP_ONLINE 之间的状态空间
  该部分中的启动回调在 CPU 上线操作期间的早CPU 设置代码中，在中断禁用的情况下，于热插拔 CPU 上被调用  拆除回调CPU 离线操作期间、CPU 完全关闭前不久，在中断禁用的情况下，于热插拔 CPU 上被调用
  该部分中的回调不允许失败
  回调用于底层硬件的初始化/关闭以及核心子系统
- ONLINE 部分

  ONLINE 部分覆盖CPUHP_AP_ONLINE + 1   CPUHP_ONLINE 之间的状态空间
  该部分中的启动回调在 CPU 上线操作期间，于热插CPU 上被调用  拆除回调CPU 离线操作期间，于热插CPU 上被调用
  回调per CPU 热插拔线程的上下文中被调用，该线程被固定在热插拔 CPU 上  回调在中断与抢占均启用的情况下被调用
  回调允许失败。当某个回调失败时，热插拔操作被中止，并CPU 被带回之前的状态
### CPU 上线/离线操作


```
  [CPUHP_OFFLINE]
  [CPUHP_OFFLINE + 1]->startup()       -> success
  [CPUHP_OFFLINE + 2]->startup()       -> success
  [CPUHP_OFFLINE + 3]                  -> skipped because startup == NULL
  ...
  [CPUHP_BRINGUP_CPU]->startup()       -> success
  === End of PREPARE section
  [CPUHP_BRINGUP_CPU + 1]->startup()   -> success
  ...
  [CPUHP_AP_ONLINE]->startup()         -> success
  === End of STARTUP section
  [CPUHP_AP_ONLINE + 1]->startup()     -> success
  ...
  [CPUHP_ONLINE - 1]->startup()        -> success
  [CPUHP_ONLINE]
```
```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_AP_ONLINE + 1]->teardown()    -> success
  === Start of STARTUP section
  [CPUHP_AP_ONLINE]->teardown()        -> success
  ...
  [CPUHP_BRINGUP_ONLINE - 1]->teardown()
  ...
  === Start of PREPARE section
  [CPUHP_BRINGUP_CPU]->teardown()
  [CPUHP_OFFLINE + 3]->teardown()
  [CPUHP_OFFLINE + 2]                  -> skipped because teardown == NULL
  [CPUHP_OFFLINE + 1]->teardown()
  [CPUHP_OFFLINE]
```
```
  [CPUHP_OFFLINE]
  [CPUHP_OFFLINE + 1]->startup()       -> success
  [CPUHP_OFFLINE + 2]->startup()       -> success
  [CPUHP_OFFLINE + 3]                  -> skipped because startup == NULL
  ...
  [CPUHP_BRINGUP_CPU]->startup()       -> success
  === End of PREPARE section
  [CPUHP_BRINGUP_CPU + 1]->startup()   -> success
  ...
  [CPUHP_AP_ONLINE]->startup()         -> success
  === End of STARTUP section
  [CPUHP_AP_ONLINE + 1]->startup()     -> success
  ---
  [CPUHP_AP_ONLINE + N]->startup()     -> fail
  [CPUHP_AP_ONLINE + (N - 1)]->teardown()
  ...
  [CPUHP_AP_ONLINE + 1]->teardown()
  === Start of STARTUP section
  [CPUHP_AP_ONLINE]->teardown()
  ...
  [CPUHP_BRINGUP_ONLINE - 1]->teardown()
  ...
  === Start of PREPARE section
  [CPUHP_BRINGUP_CPU]->teardown()
  [CPUHP_OFFLINE + 3]->teardown()
  [CPUHP_OFFLINE + 2]                  -> skipped because teardown == NULL
  [CPUHP_OFFLINE + 1]->teardown()
  [CPUHP_OFFLINE]
```
```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()
  ...
  [CPUHP_ONLINE - 1]->startup()
  [CPUHP_ONLINE]
```
递归失败无法被明智地处理。请看以```
  [CPUHP_ONLINE]
  [CPUHP_ONLINE - 1]->teardown()       -> success
  ...
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()  -> success
  [CPUHP_ONLINE - (N - 2)]->startup()  -> fail
```
CPU 热插拔状态机会就停在这里，不再尝试回退
```
  [CPUHP_ONLINE - (N - 1)]->teardown() -> success
  [CPUHP_ONLINE - N]->teardown()       -> fail
  [CPUHP_ONLINE - (N - 1)]->startup()  -> success
  [CPUHP_ONLINE - (N - 2)]->startup()  -> fail
  [CPUHP_ONLINE - (N - 1)]->teardown() -> success
  [CPUHP_ONLINE - N]->teardown()       -> fail
```
```
  [CPUHP_ONLINE - (N - 1)]
```
这至少让系统能够取得进展，并给用户一个调试甚至解决该情况的机会
### 分配一个状

分配一CPU 热插拔状态有两种方式
- 静态分
  当子系统或驱动相对于其他 CPU 热插拔状态有排序要求时，必须使用静态分配  例如，PERF 核心的启动回调必须在 CPU 上线操作期间 PERF 驱动的启动回调之前被调用  CPU 离线操作期间，驱动的拆除回调必须在核心拆除回调之前被调用  静态分配的状态由 cpuhp_state 枚举中的常量描述，该枚举可在 include/linux/cpuhotplug.h 中找到
  将状态插入到枚举中的恰当位置，以满足排序要求。该状态常量必须用于状态的建立与移除
  当状态回调不是在运行时建立，而是 kernel/cpu.c CPU 热插拔状态数组的初始化器的一部分时，也需要静态分配
- 动态分
  当状态回调没有排序要求时，动态分配是首选方法。状态号由建立函数分配，并在成功时返回给调用者
  只有 PREPARE ONLINE 部分提供动态分配范围。STARTING 部分不提供，因为该部分中的大多数回调都有显式的排序要求
### 建立一CPU 热插拔状

核心代码提供以下函数来建立一个状态：

- cpuhp_setup_state(state, name, startup, teardown)
- cpuhp_setup_state_nocalls(state, name, startup, teardown)
- cpuhp_setup_state_cpuslocked(state, name, startup, teardown)
- cpuhp_setup_state_nocalls_cpuslocked(state, name, startup, teardown)

对于驱动或子系统有多个实例、并且相同的 CPU 热插拔状态回调需要对每个实例都调用的情况CPU 热插拔核心提供多实例支持。相对于驱动特定的实例列表，其优势在于实例相关函数完全针CPU 热插拔操作被串行化，并且提供在添加与移除时状态回调的自动调用要建立这样一个多实例状态，可使用以下函数：

- cpuhp_setup_state_multi(state, name, startup, teardown)

@state 参数要么是静态分配的状态，要么是动态分配状态的常量之一——CPUHP_BP_PREPARE_DYNCPUHP_AP_ONLINE_DYN——取决于应为其分配动态状态的那个状态部分（PREPARE、ONLINE）
@name 参数用于 sysfs 输出与插桩。命名约定是 "subsys:mode" "subsys/driver:mode"例如 "perf:mode" "perf/x86:mode"。常见的 mode 名称有：

======== =======================================================
prepare  For states in the PREPARE section

dead     For states in the PREPARE section which do not provide
         a startup callback

starting For states in the STARTING section

dying    For states in the STARTING section which do not provide
         a startup callback

online   For states in the ONLINE section

offline  For states in the ONLINE section which do not provide
         a startup callback
======== =======================================================

由于 @name 参数仅用sysfs 与插桩，如果其他 mode 描述符比常见的那些更能描述状态的性质，也可以使用它们
@name 参数的示例："perf/online"perf/x86:prepare""RCU/tree:dying"sched/waitempty"

@startup 参数是一个函数指针，指向那个应在 CPU 上线操作期间被调用的回调如果使用点不需要启动回调，将指针设NULL
@teardown 参数是一个函数指针，指向那个应在 CPU 离线操作期间被调用的回调如果使用点不需要拆除回调，将指针设NULL
这些函数在所安装的回调被对待的方式上有所不同
  - cpuhp_setup_state_nocalls()、cpuhp_setup_state_nocalls_cpuslocked()
    以及 cpuhp_setup_state_multi() 只安装回
  - cpuhp_setup_state() cpuhp_setup_state_cpuslocked() 安装回调，并针对当前状态号大于新安装状态的
    所有在CPU 调用 @startup 回调（如果非 NULL）。根据状态部分，该回调要么在当前 CPU（PREPARE 部分    上被调用，要么在每个在线 CPU（ONLINE 部分）上CPU 热插拔线程上下文中被调用
    如果某个回调CPU N 失败，则调用 CPU 0 .. N-1 的拆除回调来回滚该操作    状态建立失败，该状态的回调不被安装，并且对于动态分配，所分配的状态被释放
状态建立与回调调用相对CPU 热插拔操作被串行化如果建立函数必须从一CPU 热插拔读锁定区域调用，则必须使用 _cpuslocked() 变体这些函数不能CPU 热插拔回调内部使用
函数返回值：
  ======== ===================================================================
  0        Statically allocated state was successfully set up

  >0       Dynamically allocated state was successfully set up.

           The returned number is the state number which was allocated. If
           the state callbacks have to be removed later, e.g. module
           removal, then this number has to be saved by the caller and used
           as @state argument for the state remove function. For
           multi-instance states the dynamically allocated state number is
           also required as @state argument for the instance add/remove
           operations.

  <0	   Operation failed
  ======== ===================================================================

### 移除一CPU 热插拔状

要移除一个之前已建立的状态，提供以下函数
- cpuhp_remove_state(state)
- cpuhp_remove_state_nocalls(state)
- cpuhp_remove_state_nocalls_cpuslocked(state)
- cpuhp_remove_multi_state(state)

@state 参数要么是静态分配的状态，要么是由 cpuhp_setup_state*() 在动态范围内分配的状态号如果该状态在动态范围内，则该状态号被释放，并可再次用于动态分配
这些函数在所安装的回调被对待的方式上有所不同
  - cpuhp_remove_state_nocalls()、cpuhp_remove_state_nocalls_cpuslocked()
    以及 cpuhp_remove_multi_state() 只移除回调
  - cpuhp_remove_state() 移除回调，并针对当前状态号大于被移除状态的
    所有在CPU 调用拆除回调（如果非 NULL）。根据状态部分，该回调要么在当前 CPU（PREPARE 部分    上被调用，要么在每个在线 CPU（ONLINE 部分）上CPU 热插拔线程上下文中被调用
    为了完成移除，拆除回调不应失败
状态移除与回调调用相对CPU 热插拔操作被串行化如果移除函数必须从一CPU 热插拔读锁定区域调用，则必须使用 _cpuslocked() 变体这些函数不能CPU 热插拔回调内部使用
如果移除一个多实例状态，则调用者必须先移除所有实例
### 多实例状态实例管

一旦多实例状态被建立，就可以向该状态添加实例：

  - cpuhp_state_add_instance(state, node)
  - cpuhp_state_add_instance_nocalls(state, node)

@state 参数要么是静态分配的状态，要么是由 cpuhp_setup_state_multi() 在动态范围内分配的状态号
@node 参数是一个指hlist_node 的指针，hlist_node 被嵌入在实例的数据结构中该指针被交给多实例状态的回调，并且可被回调通过 container_of() 用来取回该实例
这些函数在所安装的回调被对待的方式上有所不同
  - cpuhp_state_add_instance_nocalls() 只将实例添加    多实例状态的节点列表
  - cpuhp_state_add_instance() 添加实例，并针对当前状态号大于 @state     所有在CPU 调用@state 关联的启动回调（如果NULL）。该回调只针对要添加的实例被调用    根据状态部分，该回调要么在当前 CPU（PREPARE 部分）上被调用，要么在每个在CPU（ONLINE 部分    上的 CPU 热插拔线程上下文中被调用
    如果某个回调CPU N 失败，则调用 CPU 0 .. N-1 的拆除回调来回滚该操作，该函数失败，并且
    该实例不被添加到多实例状态的节点列表
要从状态的节点列表移除一个实例，可使用这些函数：

  - cpuhp_state_remove_instance(state, node)
  - cpuhp_state_remove_instance_nocalls(state, node)

参数与上面的 cpuhp_state_add_instance*() 变体相同
这些函数在所安装的回调被对待的方式上有所不同
  - cpuhp_state_remove_instance_nocalls() 只从
    状态的节点列表移除该实例
  - cpuhp_state_remove_instance() 移除实例，并针对当前状态号大于 @state     所有在CPU 调用@state 关联的拆除回调（如果NULL）。该回调只针对要移除的实例被调用    根据状态部分，该回调要么在当前 CPU（PREPARE 部分）上被调用，要么在每个在CPU（ONLINE 部分    上的 CPU 热插拔线程上下文中被调用
    为了完成移除，拆除回调不应失败
节点列表的添移除操作与回调调用相对于 CPU 热插拔操作被串行化这些函数不能CPU 热插拔回调内部以CPU 热插拔读锁定区域中使用
### 示例


STARTING 部分中建立并拆除一个静态分配的状```
   ret = cpuhp_setup_state(CPUHP_SUBSYS_STARTING, "subsys:starting", subsys_cpu_starting, subsys_cpu_dying);
   if (ret < 0)
        return ret;
   ....
   cpuhp_remove_state(CPUHP_SUBSYS_STARTING);
```
ONLINE 部分中建立并拆除一个动态分配的状```
   state = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "subsys:offline", NULL, subsys_cpu_offline);
   if (state < 0)
       return state;
   ....
   cpuhp_remove_state(state);
```
ONLINE 部分中建立并拆除一个动态分配的状```
   state = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "subsys:online", subsys_cpu_online, NULL);
   if (state < 0)
       return state;
   ....
   cpuhp_remove_state_nocalls(state);
```
建立、使用并拆除一个动态分配的多实例状```
   state = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN, "subsys:online", subsys_cpu_online, subsys_cpu_offline);
   if (state < 0)
       return state;
   ....
   ret = cpuhp_state_add_instance(state, &inst1->node);
   if (ret)
        return ret;
   ....
   ret = cpuhp_state_add_instance(state, &inst2->node);
   if (ret)
        return ret;
   ....
   cpuhp_remove_instance(state, &inst1->node);
   ....
   cpuhp_remove_instance(state, &inst2->node);
   ....
   cpuhp_remove_multi_state(state);
```

## 热插拔状态的测试


验证一个自定义状态是否如预期工作的一种方式是关闭一CPU，然后再将其上线也可以将CPU 置于某个特定状态（例如 **CPUHP_AP_ONLINE**），然后回到
**CPUHP_ONLINE**。这会模拟在 **CPUHP_AP_ONLINE** 之后的一个状态出错，
从而导致回滚到在线状态
```
 $ tail /sys/devices/system/cpu/hotplug/states
 138: mm/vmscan:online
 139: mm/vmstat:online
 140: lib/percpu_cnt:online
 141: acpi/cpu-drv:online
 142: base/cacheinfo:online
 143: virtio/net:online
 144: x86/mce:online
 145: printk:online
 168: sched:active
 169: online
```
```
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  169
  $ echo 140 > /sys/devices/system/cpu/cpu4/hotplug/target
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  140
```
需要注意的是，状140 的拆除回调已```
  $ echo 169 > /sys/devices/system/cpu/cpu4/hotplug/target
  $ cat /sys/devices/system/cpu/cpu4/hotplug/state
  169
```
```
  #  TASK-PID   CPU#    TIMESTAMP  FUNCTION
  #     | |       |        |         |
      bash-394  [001]  22.976: cpuhp_enter: cpu: 0004 target: 140 step: 169 (cpuhp_kick_ap_work)
   cpuhp/4-31   [004]  22.977: cpuhp_enter: cpu: 0004 target: 140 step: 168 (sched_cpu_deactivate)
   cpuhp/4-31   [004]  22.990: cpuhp_exit:  cpu: 0004  state: 168 step: 168 ret: 0
   cpuhp/4-31   [004]  22.991: cpuhp_enter: cpu: 0004 target: 140 step: 144 (mce_cpu_pre_down)
   cpuhp/4-31   [004]  22.992: cpuhp_exit:  cpu: 0004  state: 144 step: 144 ret: 0
   cpuhp/4-31   [004]  22.993: cpuhp_multi_enter: cpu: 0004 target: 140 step: 143 (virtnet_cpu_down_prep)
   cpuhp/4-31   [004]  22.994: cpuhp_exit:  cpu: 0004  state: 143 step: 143 ret: 0
   cpuhp/4-31   [004]  22.995: cpuhp_enter: cpu: 0004 target: 140 step: 142 (cacheinfo_cpu_pre_down)
   cpuhp/4-31   [004]  22.996: cpuhp_exit:  cpu: 0004  state: 142 step: 142 ret: 0
      bash-394  [001]  22.997: cpuhp_exit:  cpu: 0004  state: 140 step: 169 ret: 0
      bash-394  [005]  95.540: cpuhp_enter: cpu: 0004 target: 169 step: 140 (cpuhp_kick_ap_work)
   cpuhp/4-31   [004]  95.541: cpuhp_enter: cpu: 0004 target: 169 step: 141 (acpi_soft_cpu_online)
   cpuhp/4-31   [004]  95.542: cpuhp_exit:  cpu: 0004  state: 141 step: 141 ret: 0
   cpuhp/4-31   [004]  95.543: cpuhp_enter: cpu: 0004 target: 169 step: 142 (cacheinfo_cpu_online)
   cpuhp/4-31   [004]  95.544: cpuhp_exit:  cpu: 0004  state: 142 step: 142 ret: 0
   cpuhp/4-31   [004]  95.545: cpuhp_multi_enter: cpu: 0004 target: 169 step: 143 (virtnet_cpu_online)
   cpuhp/4-31   [004]  95.546: cpuhp_exit:  cpu: 0004  state: 143 step: 143 ret: 0
   cpuhp/4-31   [004]  95.547: cpuhp_enter: cpu: 0004 target: 169 step: 144 (mce_cpu_online)
   cpuhp/4-31   [004]  95.548: cpuhp_exit:  cpu: 0004  state: 144 step: 144 ret: 0
   cpuhp/4-31   [004]  95.549: cpuhp_enter: cpu: 0004 target: 169 step: 145 (console_cpu_notify)
   cpuhp/4-31   [004]  95.550: cpuhp_exit:  cpu: 0004  state: 145 step: 145 ret: 0
   cpuhp/4-31   [004]  95.551: cpuhp_enter: cpu: 0004 target: 169 step: 168 (sched_cpu_activate)
   cpuhp/4-31   [004]  95.552: cpuhp_exit:  cpu: 0004  state: 168 step: 168 ret: 0
      bash-394  [005]  95.553: cpuhp_exit:  cpu: 0004  state: 169 step: 140 ret: 0
```
如所见，CPU4 一路下降到时间22.996，然后又一路上升到 95.552所有被调用的回调及其返回码trace 中都可见
## 架构要求


需要以下函数与配置
`CONFIG_HOTPLUG_CPU`
  此条目需要在 Kconfig 中启
`__cpu_up()`
  Arch 接口，用于启动一CPU

`__cpu_disable()`
  Arch 接口，用于关闭一CPU，在该例程返回之后内核不能再处理任何中断。这包括定时器的关闭
`__cpu_die()`
  这实际上是用来确CPU 的死亡。实际上请参考其他实现了 CPU 热插拔的 arch 中的示例代码  处理器从该特定架构的 `idle()` 循环中被取下。`__cpu_die()`
  通常等待某个 per_cpu 状态被设置，以确保处理器死亡例程被调用，从而确信其已死亡
## 用户空间通知


```
  SUBSYSTEM=="cpu", DRIVERS=="processor", DEVPATH=="/devices/system/cpu/*", RUN+="the_hotplug_receiver.sh"
```
```
  #!/bin/sh

  if [ "${ACTION}" = "offline" ]
  then
      echo "CPU ${DEVPATH##*/} offline"

  elif [ "${ACTION}" = "online" ]
  then
      echo "CPU ${DEVPATH##*/} online"

  fi
```
可以进一步处理该事件
当系统中发生CPU 的更改时，如果内核自行更kdump 捕获内核CPU 列表（通过 elfcorehdr 以及
其他相关kexec 段），则 sysfs 文件
/sys/devices/system/cpu/crash_hotplug 包含 '1'，如果用户空间必须更kdump 捕获内核CPU 列表，则包含 '0'
其可用性取决于 CONFIG_HOTPLUG_CPU 内核配置选项
为了跳过用户空间对用kdump CPU 热插拔下事件的处理（即先卸载再重载以获得当前 CPU 列表），
sysfs 文件可在 udev 规则中如下使用：

 SUBSYSTEM=="cpu", ATTRS{crash_hotplug}=="1", GOTO="kdump_reload_end"

对于 CPU 热插拔下事件，如果架构支持对 elfcorehdr（包CPU 列表）以及其他相kexec 段的
内核更新，那么该规则会跳kdump 捕获内核的卸重载
## 内核内联文档参