## 客户机停机轮询（Guest halt polling）


cpuidle_haltpoll 驱动配合 haltpoll 调度器（governor），允许客户机 vcpu 在
停机（halt）之前轮询一段指定的时间。

这给主机一侧的轮询带来了以下好处：

 1) 在轮询执行期间会设置 POLL 标志，使得远程 vCPU 在执行唤醒时可以避免
	   发送 IPI（以及处理该 IPI 的相关开销）。

 2) 可以避免 VM-exit 的开销。

客户机一侧轮询的缺点在于，即使主机上还有其它可运行任务，也会执行轮询。

基本逻辑如下：由一个全局值 guest_halt_poll_ns 由用户配置，表示允许轮询的
最长时间。该值是固定的。

每个 vcpu 都有一个可调整的 guest_halt_poll_ns（“每 cpu 的 guest_halt_poll_ns”），
由算法根据事件（如下所述）进行调整。

## 模块参数


haltpoll 调度器有 5 个可调整的模块参数：

1) guest_halt_poll_ns：

轮询在停机前执行的最长时间（单位纳秒）。

默认值：200000

2) guest_halt_poll_shrink：

当唤醒事件发生在全局 guest_halt_poll_ns 之后时，用于收缩每 cpu 的
guest_halt_poll_ns 的除法因子。

默认值：2

3) guest_halt_poll_grow：

当事件发生在每 cpu 的 guest_halt_poll_ns 之后、但在全局 guest_halt_poll_ns
之前时，用于增长每 cpu 的 guest_halt_poll_ns 的乘法因子。

默认值：2

4) guest_halt_poll_grow_start：

在空闲系统的情况下，每 cpu 的 guest_halt_poll_ns 最终会降到零。该值设置了
增长时的初始每 cpu 的 guest_halt_poll_ns。可以从 10000 起增大，以避免在
初始增长阶段出现遗漏：

10k、20k、40k、……（示例假设 guest_halt_poll_grow=2）。

默认值：50000

5) guest_halt_poll_allow_shrink：

允许收缩的布尔参数。设为 N 可避免收缩（一旦达到全局 guest_halt_poll_ns 值，
每 cpu 的 guest_halt_poll_ns 将保持较高）。

默认值：Y

```

	/sys/module/haltpoll/parameters/

```
## 进一步说明


- 设置 guest_halt_poll_ns 参数时应小心，因为较大的值有可能将一台本应几乎
  完全空闲的机器的 cpu 使用率推高到 100%。
