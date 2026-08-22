## 内务处理（Housekeeping


CPU 隔离会将原本可能运行在任CPU 上的内核工作移走。其相关特性的目的是减少某些极端工作负载（例如部分 DPDK 用例）无法容忍的 OS 抖动
CPU 隔离移走的内核工作通常被描述为“housekeeping（内务处理）”，因为它包含执行清理、统计信息的维护及依赖它们的行为、内存释放、各种延迟操作等的基础性工作
有时 housekeeping 只是一些未绑定的工作（未绑定的工作队列、未绑定的定时器等），它们很容易被分配到非隔离的 CPU。但有时 housekeeping 会绑定到特定 CPU，需要精巧的技巧才能卸载到非隔CPU（RCU_NOCB、远程调度器 tick 等）
因此，housekeeping CPU 可以看作是隔CPU 的反面。它只是一个可以执housekeeping 工作CPU。任何时刻都必须至少有一个在线的 housekeeping CPU。未被隔离的 CPU 会自动被指派housekeeping
Housekeeping 目前划分为由 `enum hk_type type` 描述的四个特性：

1. HK_TYPE_DOMAIN 匹配通过 `isolcpus=domain` 启动参数cgroup v2 中的隔离 cpuset 分区执行的调度器域隔离所移走的工作。这包括调度器负载均衡、未绑定的工作队列和定时器
2. HK_TYPE_KERNEL_NOISE 匹配通过 `nohz_full=` `isolcpus=nohz` 启动参数执行tick 隔离所移走的工作。这包括远程调度tick、vmstat lockup watchdog
3. HK_TYPE_MANAGED_IRQ 匹配通过 `isolcpus=managed_irq` 执行的受管理 IRQ 隔离所移走IRQ 处理程序
4. HK_TYPE_DOMAIN_BOOT 匹配仅通过 `isolcpus=domain` 执行的调度器域隔离所移走的工作。它HK_TYPE_DOMAIN 类似，区别在于它忽略 cpuset 执行的隔离

## Housekeeping cpumask


Housekeeping cpumask 包含了可以执行由相应隔离特性移走的工作CPU。这cpumask 由以下函数返```

	const struct cpumask *housekeeping_cpumask(enum hk_type type)

```
默认情况下，如果既未使用 `nohz_full=`、也未使`isolcpus`，也未使cpuset 的隔离分区（覆盖大多数用例），该函数返回 cpu_possible_mask
否则该函数返回隔离特性的 cpumask 补集。例如：

使用 isolcpus=domain,7 时，以下调用将返回包含所有可```

	housekeeping_cpumask(HK_TYPE_DOMAIN)

```
类似地，使用 nohz_full=5,6 时，以下调用将返回包含所```

	housekeeping_cpumask(HK_TYPE_KERNEL_NOISE)


```
## cpusets 的同

Cpuset 可以在创建、修改或删除隔离分区时修HK_TYPE_DOMAIN housekeeping cpumask
HK_TYPE_DOMAIN cpumask 的使用者必须确保与 cpuset 正确同步，以保证
1. cpumask 快照保持一致性
2. 不会在刚被设为隔离的 CPU 上排队任housekeeping 工作
3. 排队到某个非隔离 CPU（该 CPU 刚刚通过 cpuset 变为隔离）的待处housekeeping 工作，必须在相关已创修改的隔离分区对用户空间可用之前被刷新
该同步由基于 RCU 的方案维护。cpuset 更新侧在更新 HK_TYPE_DOMAIN cpumask 之后、刷新待处理工作之前，等待一RCU 宽限期（grace period）。在读侧，必须将 housekeeping 目标的选择与工作入队放在同一RCU 读侧临界区内
更新侧的典型布局示例如下
```

	rcu_assign_pointer(housekeeping_cpumasks[type], trial);
	synchronize_rcu();
	flush_workqueue(example_workqueue);

```
```

	rcu_read_lock();
	cpu = housekeeping_any_cpu(HK_TYPE_DOMAIN);
	queue_work_on(cpu, example_workqueue, work);
	rcu_read_unlock();

```
