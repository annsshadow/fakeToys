## 调度域（Scheduler Domains

每个 CPU 都有一基础"调度域（struct sched_domain）。域层级通过 ->parent 指针由这些基础域构建而成>parent MUST NULL 结尾，且域结构应为每 CPU 的，因为它们被无锁地更新
每个调度域跨越若CPU（存储在 ->span 字段中）。一个域span 必须是其子域 span 的超集（若有需要，此限制可放宽），并且 CPU i 的基础域必须至少跨i。每CPU 的顶层域通常会跨越系统中所CPU，尽管严格来说不必如此，但这可能导致某些 CPU 永远不会被分到任务运行，除非显式设置CPU 允许掩码（allowed mask）。调度域span 意为"在这CPU 之间平衡进程负载"
每个调度域必须有一个或多个 CPU 组（struct sched_group），它们通过 ->groups 指针组织成一个单向循环链表。这些组cpumask 的并集必须与域的 span 相同>groups 指针所指向的组必须包含该域所属的 CPU。组可以CPU 之间共享，因为在建立之后它们包含的是只读数据。任意两个组cpumask 交集可能非空。如果是这样，会在相应的调度域上设置 SD_OVERLAP 标志，且其组不能CPU 之间共享
调度域内的负载均衡发生在组之间。也就是说，每个组被当作一个实体。一个组的负载定义为该组各成CPU 负载之和，只有当某个组的负载变得不均衡时，才会在组之间移动任务
kernel/sched/core.c 中，sched_balance_trigger() 通过 sched_tick() 在每CPU 上周期性运行。它在当前运行队列（runqueue）的下一次定期调度再均衡事件到达后触发一个软中断（softirq）。真正的负载均衡工作核心 sched_balance_softirq()->sched_balance_domains() 随后在软中断上下文（SCHED_SOFTIRQ）中运行
后者函数接受两个参数：当前 CPU 的运行队列，以及发生 sched_tick() 时该 CPU 是否空闲，并从基础域开始沿 ->parent 链向上遍历当CPU 所在的所有调度域。在遍历过程中，它检查当前域是否已用尽其再均衡间隔。如果是，它就在该域上运sched_balance_rq()。然后它检查父调度域（如果存在），再检查父域的父域，依此类推
最初，sched_balance_rq() 找到当前调度域中最繁忙（busiest）的组。如果成功，它就在该组所CPU 的运行队列中寻找最繁忙的运行队列。如果找到了这样一个运行队列，它会锁定我们初始 CPU 的运行队列与新找到的最繁忙运行队列，并开始将任务从后者迁移到我们的运行队列。迁移的任务确切数量等于之前遍历该调度域各组时计算出的不平衡量
## 瀹炵幇璋冨害鍩。

"基础"域将"跨越"层级的第一级。在 SMT 的情形下，你会跨越物CPU 的所有兄弟（sibling）核，每个组是单个虚CPU
SMP 中，基础域的父域将跨越节点（node）内所有物CPU，每个组是单个物CPU。然后在 NUMA 下，SMP 域的父域将跨越整台机器，每个组拥有某个节点的 cpumask。或者，你也可以做多NUMA，例Opteron 可能只有一个覆盖其单个 NUMA 级别的域
实现者应当阅include/linux/sched/sd_flags.h 中的注释：SD_*，以了解调度域的 SD 标志的具体内容以及该如何调优
架构可以通过创建一sched_domain_topology_level 数组，并以该数组为参数调set_sched_topology()，来覆盖通用的域构建器以及给定拓扑级别的默认 SD 标志
sched-domains 调试基础设施可通过在内核命令行添加 'sched_verbose' 来启用。如果你忘了调整命令行，也可以翻/sys/kernel/debug/sched/verbose 开关。这会启用对调度域的错误检查解析，应能捕获大多数可能的错误（如上所述），同时也会以可视化格式打印出域结构