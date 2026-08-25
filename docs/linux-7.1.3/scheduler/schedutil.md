## Schedutil


   所有这一切都假设频率与工作能力之间存在线性关系，我们知道这有缺陷，但它是现有最可行的近似

## PELT（每实体负载跟踪，Per Entity Load Tracking


通过 PELT，我们跨各种调度实体跟踪一些指标，从单个任务、任务组切片CPU 运行队列。作为基础，我们使用指数加权移动平均（EWMA），每个周期024us）被衰减，使y^32 = 0.5。也就是说，最近的 32ms 贡献一半，而历史的其余部分贡献另一半

具体为：

  ewma_sum(u) := u_0 + u_1**y + u_2**y^2 + ...

  ewma(u) = ewma_sum(u) / ewma_sum(1)

由于这本质上是一个无穷等比数列的级数，结果是可组合的，即 ewma(A) + ewma(B) = ewma(A+B)。这一性质很关键，因为它使得在任务迁移时能够重新组合这些平均值

注意，被阻塞的任务仍然会对聚合值（任务组切片和 CPU 运行队列）产生贡献，这反映了它们在恢复运行时的预期贡献

利用这一点，我们跟踪两个关键指标running'（运行）'runnable'（可运行）running' 反映一个实体在 CPU 上花费的时间，'runnable' 反映一个实体在运行队列上花费的时间。当只有一个任务时，这两个指标相同；但一CPU 出现争用running' 会下降以反映每个任务CPU 上花费的时间比例，'runnable' 会上升以反映争用的程度

更多细节参见：kernel/sched/pelt.c


## 频率 / CPU 不变


由于1GHz 下占CPU 50% 不同于在 2GHz 下占CPU 50%，在 LITTLE CPU 上运50% 也不同于big CPU 上运50%，我们允许架构用两个比例对时间增量进行缩放：一个是动态电压与频率调整（DVFS）比例，一个是微架构比例

对于简单的 DVFS 架构（软件完全可控的情况），我们可以轻易
```

	    f_cur
  r_dvfs := -----
            f_max

```
对于硬件控制 DVFS 的更具动态性的系统，我们使用硬件计数器（Intel APERF/MPERF、ARMv8.4-AMU）来提供该比例
```

	   APERF
  f_cur := ----- * P0
	   MPERF

	     4C-turbo;	if available and turbo enabled
  f_max := { 1C-turbo;	if turbo enabled
	     P0;	otherwise

                    f_cur
  r_dvfs := min( 1, ----- )
                    f_max

```
我们选择 4C turbo 而非 1C turbo，以使其略微更具可持续性

r_cpu 被确定为当前 CPU 的最高性能级别与系统中任何其他 CPU 的最高性能级别之比

  r_tot = r_dvfs * r_cpu

结果是，上述'running' 'runnable' 指标变得DVFS CPU 类型无关。换言之，我们可以CPU 之间转移并比较它们

更多细节参见

 - kernel/sched/pelt.h:update_rq_clock_pelt()
 - arch/x86/kernel/smpboot.c:"APERF/MPERF frequency ratio computation."
 - Documentation/scheduler/sched-capacity.rst:"1. CPU Capacity + 2. Task utilization"


## UTIL_EST


由于周期性任务在睡眠时其平均值会衰减，即便运行时其预期利用率相同，它们也会在再次运行时遭受（DVFS）爬升

为缓解这一点（一个默认开启的选项），UTIL_EST 在出队时——当其最高时——以一'running' 值驱动一个无限冲激响应（IIR）EWMA。UTIL_EST 滤波器会立即增大，并且只在减小时衰减

进一步维护一个运行队列范围的、针对（可运行任务）的求和：

  util_est := \Sum_t max( t_running, t_util_est_ewma )

更多细节参见：kernel/sched/fair.c:util_est_dequeue()


## UCLAMP


可以为每CFS RT 任务设置有效u_min u_max 钳制值；运行队列为所有正在运行的任务维护这些钳制值的最大聚合

更多细节参见：include/uapi/linux/sched/types.h


## Schedutil / DVFS


每当调度器负载跟踪被更新（任务唤醒、任务迁移、时间推进）时，我们都会调用 schedutil 来更新硬DVFS 状态

其基础CPU 运行队列'running' 指标，根据上述内容，它是 CPU 的频率不变利用率估计。由此我们计
```

             max( running, util_est );	if UTIL_EST
  u_cfs := { running;			otherwise

               clamp( u_cfs + u_rt , u_min, u_max );	if UCLAMP_TASK
  u_clamp := { u_cfs + u_rt;				otherwise

  u := u_clamp + u_irq + u_dl;		[approx. see source for more detail]

  f_des := min( f_max, 1.25 u * f_max )

```
XXX IO-wait：当更新是由IO 完成导致的任务唤醒时，我们将上面'u' 提升

随后该频率被用来选择一P-state/OPP，或被直接转换为一个发送给硬件CPPC 风格请求

XXX：截止期限任务（Sporadic Task Model，突发任务模型）允许我们计算出满足该工作负载所需的硬 f_min

由于这些回调直接来自调度器，DVFS 硬件交互应当“快速”且非阻塞。Schedutil 支持DVFS 请求进行限流，以应对硬件交互缓慢且代价高昂的情况，这会降低有效性

更多信息参见：kernel/sched/cpufreq_schedutil.c


## 注意事项


 - 在低负载场景下，DVFS 最为重要，'running' 数值将密切反映利用率

 - 在饱和场景下，任务迁移会导致一些瞬时下降，假设我们有一个被 4 个任务饱和的 CPU，那么当我们将一个任务迁移到空闲 CPU 时，CPU 将具0.75 'running' 值，而新 CPU 将获0.25。这是不可避免的，时间推进会纠正这一点。XXX 由于不存在空闲时间，我们是否仍然保证 f_max

 - 上述大部分内容是关于避免 DVFS 下降，以及独立的 DVFS 域在负载转移时不得不重新学习/爬升
