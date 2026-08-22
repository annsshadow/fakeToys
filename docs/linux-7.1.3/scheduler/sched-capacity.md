## 感知容量的调度（Capacity Aware Scheduling

## 1. CPU 容量


### 1.1 简

传统的同SMP 平台由完全一致的 CPU 组成。另一方面，异构平台由具有不同性能特征CPU 组成 —此类平台上，并非所CPU 都可以被视为等同
CPU 容量是对一CPU 所能达到的性能的度量，以系统中性能最强的 CPU 为基准进行了归一化。异构系也被称为非对CPU 容量系统，因为它们包含容量不同的 CPU
可达到的最大性能（即最CPU 容量）的差异源于两个因素
- 并非所CPU 都可能具有相同的微架构（µarch）- 通过动态电压与频率调节（DVFS），并非所CPU 在物理上都能够达到较高的运行性能点（OPP）
Arm big.LITTLE 系统就是这两者的一个例子。big CPU LITTLE CPU 更偏向性能（更多的流水线级数、更大的
缓存、更智能的预测器等），并且通常能达到比 LITTLE CPU 更高OPP
CPU 性能通常用每秒百万条指令（MIPS）表示，也可以表示为在给定时间内可达到的一定数量指```

  capacity(cpu) = work_per_hz(cpu) * max_freq(cpu)

```
### 1.2 调度器术

调度器内部使用两个不同的容量值。一CPU `original capacity`（原始容量）是其可达到的最大容量，
即其可达到的最大性能水平。这个原始容量由函数 arch_scale_cpu_capacity() 返回。一CPU `capacity`
是其 ``original capacity`` 减去一部分可用性能的损耗（例如处理 IRQ 所花费的时间）
注意，CPU `capacity` 仅用CFS 类，`original capacity` 与具体类无关。为了简洁，本文档其余部将互换使`capacity` `original capacity`
### 1.3 平台示例


#### 1.3.1 相同OPP


考虑一个假设的 dual-core 非对CPU 容量系统，其
- work_per_hz(CPU0) = W
- work_per_hz(CPU1) = W/2
- 所CPU 都以相同的固定频率运
根据上述容量定义
- capacity(CPU0) = C
- capacity(CPU1) = C/2

为了Arm big.LITTLE 类比，CPU0 将是 big，CPU1 将是 LITTLE
对于周期性执行固定工作量负载，你将得```

 CPU0 work ^
           |     ____                ____                ____
           |    |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

 CPU1 work ^
           |     _________           _________           ____
           |    |         |         |         |         |
           +----+----+----+----+----+----+----+----+----+----+-> time

```
CPU0 在系统中具有最高容量（C），并在 T 单位时间内完成固定工作量 W。另一方面，CPU1 的容量是 CPU0 一半，因此T 内只完成 W/2
#### 1.3.2 不同的最OPP


通常，具有不同容量值的 CPU 也具有不同的最OPP。考虑与上面相同的 CPU（即相同work_per_hz()），
但：

- max_freq(CPU0) = F
- max_freq(CPU1) = 2/3 * F

这产生：

- capacity(CPU0) = C
- capacity(CPU1) = C/3

执行1.3.1 中描述的相同工作负载，每CPU 以其
```

 CPU0 work ^
           |     ____                ____                ____
           |    |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

                            workload on CPU1
 CPU1 work ^
           |     ______________      ______________      ____
           |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

```
### 1.4 表示上的注意事项


应该指出，使*单一**值来表示 CPU 性能差异在某种程度上是一个有争议的点。两个不µarch 之间的相性能差异可能是整数运算上 X%、浮点运算上 Y%、分支上 Z%，依此类推。尽管如此，使用这种简单方法的
结果到目前为止是令人满意的
## 2. 任务利用

### 2.1 简

感知容量的调度需要一种表达任务对 CPU 容量需求的方式。每个调度器类可以不同地表达这一点，而虽然任利用率（task utilization）是 CFS 特有的，但为了方便引入更通用的概念，在此处描述它是合适的
任务利用率是一个百分比，旨在表示吞吐量需```

  task_util(p) = duty_cycle(p)

```
在具有固定频率的 SMP 系统上，100% 利用率意味着该任务是一个忙循环（busy loop）。相反，10% 利用暗示它是一个小的周期性任务，花费在睡眠上的时间多于执行的时间。可变的 CPU 频率和异构的 CPU 容量这变得有些复杂；以下章节将展开讨论这些
### 2.2 频率不变

需要考虑的一个问题是，工作负载的占空比（duty cycle）直接受CPU 当前运行OPP 的影响。考虑运行一```

  CPU work ^
           |     ____                ____                ____
           |    |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

```
这产duty_cycle(p) == 25%
```

  CPU work ^
           |     _________           _________           ____
           |    |         |         |         |         |
           +----+----+----+----+----+----+----+----+----+----+-> time

```
这产duty_cycle(p) == 50%，尽管任务具有完全相同的行为（即执行相同数量的工作）于两次执行中
任务利用率信号可以通过下式变为频率不变
```

  task_util_freq_inv(p) = duty_cycle(p) * (curr_frequency(cpu) / max_frequency(cpu))

```
将此公式应用于上述两个示例，得到频率不变的任务利用率 25%
### 2.3 CPU 不变

CPU 容量对任务利用率有类似的影响，因为在具有不同容量值的 CPU 上运行相同的工作负载会产生不同的占空比
```

```
- capacity(CPU0) = C
- capacity(CPU1) = C/3

在每CPU 以其最大频率执行给定周期性工作负载时，会
```

 CPU0 work ^
           |     ____                ____                ____
           |    |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

 CPU1 work ^
           |     ______________      ______________      ____
           |    |              |    |              |    |
           +----+----+----+----+----+----+----+----+----+----+-> time

```
换言之，

- duty_cycle(p) == 25%（如p CPU0 以其最大频率上运行- duty_cycle(p) == 75%（如p CPU1 以其最大频率上运行
任务利用率信号可以通过下式变为 CPU 不变
```

  task_util_cpu_inv(p) = duty_cycle(p) * (capacity(cpu) / max_capacity)

```
其中 `max_capacity` 是系统中的最CPU 容量值。将此公式应用于上面的示例，得到 CPU 不变的任务利用率
25%銆。
### 2.4 不变的任务利用率


为了获得真正不变的信号，必须将频率不变性和 CPU 不变性都应用于任务利用率。对于一个给定的任务，既
CPU 不变又频率不变的利用率伪公式因此```

                                     curr_frequency(cpu)   capacity(cpu)
  task_util_inv(p) = duty_cycle(p) * ------------------- * -------------
                                     max_frequency(cpu)    max_capacity

```
换句话说，不变的任务利用率描述了一个任务的行为，就好像它运行在系统中容量最高的 CPU 上，并以最频率运行
以下章节中任何提到任务利用率的地方都隐含指其不变形式
### 2.5 利用率估

如果没有水晶球，任务行为（以及因此的任务利用率）在任务首次变为可运行的那一刻是无法准确预测的。CFS
类基于每实体负载跟踪（PELT）机制维护少CPU 和任务信号，其中之一产生一*平均**利用率（与瞬时相对）
这意味着，虽然感知容量的调度准则将考虑“真实”的任务利用率（使用水晶球），但实现将只能使用其估计值
## 3. 感知容量的调度需

### 3.1 CPU 容量


Linux 目前无法自行确定 CPU 容量，因此需要将此信息传递给它。架构必须为此定arch_scale_cpu_capacity()
arm、arm64 RISC-V 架构将此直接映射arch_topology 驱动CPU 缩放数据，该数据派生capacity-dmips-mhz CPU 绑定；参Documentation/devicetree/bindings/cpu/cpu-capacity.txt
### 3.2 频率不变

2.2 节所述，感知容量的调度需要频率不变的任务利用率。架构必须为此定arch_scale_freq_capacity(cpu)
实现此函数需要弄清每CPU 曾以何种频率运行。一种实现方式是利用增量速率CPU 当前频率缩放的硬计数器（x86 上的 APERF/MPERF、arm64 上的 AMU）。另一种方式是直接挂钩cpufreq 的频率切换，当内知道切换到的频率时（arm/arm64 也采用此方式）
## 4. 调度器拓

在构sched 域期间，调度器将弄清楚系统是否表现出非对称的 CPU 容量。如果是这样
- sched_asym_cpucapacity 静态键将被启用- SD_ASYM_CPUCAPACITY_FULL 标志将在跨越所有唯一 CPU 容量值的最sched_domain 级别上设置- SD_ASYM_CPUCAPACITY 标志将为任何跨越具有任意范围不对称性的 CPU sched_domain 设置
sched_asym_cpucapacity 静态键旨在保护那些服务于非对称 CPU 容量系统的代码段。但是请注意，该键是
```

  capacity    C/2          C
            ________    ________
           /        \  /        \
  CPUs     0  1  2  3  4  5  6  7
           \__/  \______________/
  cpusets   cs0         cs1

```
它可以通过以下方式创建

  mkdir /sys/fs/cgroup/cpuset/cs0
  echo 0-1 > /sys/fs/cgroup/cpuset/cs0/cpuset.cpus
  echo 0 > /sys/fs/cgroup/cpuset/cs0/cpuset.mems

  mkdir /sys/fs/cgroup/cpuset/cs1
  echo 2-7 > /sys/fs/cgroup/cpuset/cs1/cpuset.cpus
  echo 0 > /sys/fs/cgroup/cpuset/cs1/cpuset.mems

  echo 0 > /sys/fs/cgroup/cpuset/cpuset.sched_load_balance

由于系统*存在** CPU 容量不对称，sched_asym_cpucapacity 静态键将被启用。然而，CPU 0-1 sched_domain 层级跨越单一容量值：在该层级中不设置 SD_ASYM_CPUCAPACITY，它描述了一SMP 孤岛，应
作为此类对待
因此，保护服务于非对CPU 容量的代码路径的“规范”模式是
- 检sched_asym_cpucapacity 静态键
- 如果它已启用，则还要检sched_domain 层级中是否存SD_ASYM_CPUCAPACITY（如果相关，即该代码路径
  针对特定CPU 或其组）

## 5. 感知容量的调度实

### 5.1 CFS


#### 5.1.1 容量适配（capacity fitness

```

  task_util(p) < capacity(task_cpu(p))

```
这通常称为容量适配准则，即 CFS 必须确保任务“适配”于CPU。如果违反它，任务将需要完成超过其 CPU
所能提供的更多工作：它将成CPU 受限（CPU-bound）的
此外，uclamp 允许用户空间通过 sched_setattr() cgroup 接口（参Documentation/admin-guide/cgroup-v2.rst）为任务指定最小和最大利用率值。顾名思义，这可用于在前述
准则中钳task_util()
#### 5.1.2 唤醒 CPU 选择


CFS 任务唤醒 CPU 选择遵循上述的容量适配准则。在此基础上，uclamp 用于钳制任务利用率值，这让用户空间
CFS CPU 选择有更多控制力
```

  clamp(task_util(p), task_uclamp_min(p), task_uclamp_max(p)) < capacity(cpu)

```
通过使用 uclamp，用户空间可以例如通过给一个忙循环00% 利用率）一个较低的 uclamp.max 值来允许它在
任何 CPU 上运行。反之，它可以通过给一个小的周期性任务（例如 10% 利用率）一个较高的 uclamp.min 来强制其在最高性能CPU 上运行

  CFS 中的唤醒 CPU 选择可能被能量感知调度（EAS）所覆盖，EAS Documentation/scheduler/sched-energy.rst
  中描述
#### 5.1.3 负载均衡


唤醒 CPU 选择中的一个病态情况发生在任务很少
```

  w == wakeup event

  capacity(CPU0) = C
  capacity(CPU1) = C / 3

                           workload on CPU0
  CPU work ^
           |     _________           _________           ____
           |    |         |         |         |         |
           +----+----+----+----+----+----+----+----+----+----+-> time
                w                   w                   w

                           workload on CPU1
  CPU work ^
           |     ____________________________________________
           |    |
           +----+----+----+----+----+----+----+----+----+----+->
                w

```
此工作负载应该在 CPU0 上运行，但如果任务要么：

- 从一开始就被不正确地调度（不准确的初始利用率估计）
- 从一开始就被正确调度，但突然需要更多处理能
那么它可能会变成 CPU 受限，即 `task_util(p) > capacity(task_cpu(p))`；CPU 容量调度准则被违反，并且
可能没有任何更多的唤醒事件可以通过唤醒 CPU 选择来修复它
处于这种情况的任务被称为“misfit”（不匹配）任务，为处理这种情况而建立的机制共享相同的名称。Misfit
任务迁移利用 CFS 负载均衡器，更具体地说是主动负载均衡部分（它负责迁移当前正在运行的任务）。当发生
负载均衡时，如果一misfit 任务可以被迁移到比其当前 CPU 容量更高CPU 上，就会触发一misfit
主动负载均衡
### 5.2 RT


#### 5.2.1 唤醒 CPU 选择


```

  task_uclamp_min(p) <= capacity(task_cpu(cpu))

```
同时仍然遵循通常的优先级约束。如果没有候CPU 能够满足此容量准则，则遵循严格的基于优先级的调度，并
忽略 CPU 容量
### 5.3 DL


#### 5.3.1 唤醒 CPU 选择


```

  task_bandwidth(p) < capacity(task_cpu(p))

```
同时仍然遵守通常的带宽和截止时间约束。如果没有候CPU 能够满足此容量准则，则任务将保留在其当前 CPU 上