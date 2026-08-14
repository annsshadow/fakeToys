
## CPU 隔离

## 简介

“CPU 隔离”是指让一个 CPU 专用于给定的工作负载，而没有任何来自内核的不期望的代码干扰。

这些干扰通常被称为“噪声”，可能由异步事件（中断、定时器、由工作队列和内核线程引起的调度器
抢占……）或同步事件（系统调用和缺页）触发。

这种噪声通常不易被察觉。毕竟，同步事件是所请求的内核服务的组成部分。而异步事件要么作为任务
执行时被调度器充分均匀地分布，要么作为中断执行时足够快。定时器中断甚至每秒可以执行 1024 次，
而大多数时候不会产生显著且可测量的影响。

然而一些罕见且极端的工作负载可能对这类噪声相当敏感。例如高带宽网络处理（不能丢失单个数据包）
或极低延迟网络处理就是这种情况。这类用例通常涉及 DPDK，绕过内核网络栈并从用户空间直接访问
网络设备。

为了在没有或仅有有限内核噪声的情况下运行一个 CPU，相关的内务（housekeeping）工作要么需要被
关闭，要么被迁移，要么被卸载。

## 内务处理（Housekeeping）

在 CPU 隔离术语中，housekeeping 是内核为了维持其所有服务而需要处理的工作，通常是异步的。它
对应于上面列举的噪声，除非至少有一个 CPU 被隔离。当存在被隔离的 CPU 时，如果与 CPU 绑定的工作
必须被卸载，那么 housekeeping 可能会使用进一步的应对机制。

Housekeeping CPU 是那些非隔离的 CPU，内核噪声会从隔离的 CPU 上迁移到这些 CPU 上。

隔离可以根据噪声的性质以多种方式实现：

- 未绑定（unbound）工作，其中“未绑定”指不与任何 CPU 绑定，可以简单地从隔离的 CPU 迁移到
  housekeeping CPU。未绑定的工作队列、内核线程和定时器就是这种情况。

- 绑定（bound）工作，其中“绑定”指与特定 CPU 绑定，通常因其性质而无法原样移走。要么：

 - 该工作必须切换到一种加锁的实现。例如：配置了 CONFIG_RCU_NOCB_CPU 的 RCU 就是这种情况。

 - 相关特性必须被关闭，并视为与隔离的 CPU 不兼容。例如：Lockup 看门狗、不可靠的时钟源等。

 - 一种精细且重量级的应对机制作为替代。例如：在 nohz_full CPU 上定时器滴答被关闭，但约束是
   其上只能运行单个任务。内核进入/退出会增加显著的成本开销，并且残留的 1Hz 调度器滴答被
   卸载到 housekeeping CPU。

无论如何，housekeeping 工作都必须被处理，这就是为什么系统中必须至少有一个 housekeeping CPU，
如果机器运行大量 CPU，最好更多。例如在 NUMA 系统上每个节点一个。

此外，CPU 隔离通常意味着在无噪声的隔离 CPU 与 housekeeping CPU 上增加的开销之间进行权衡，
有时甚至包括进入内核的隔离 CPU。

## 隔离特性

可以在内核中配置不同级别的隔离，每种都有其自身的缺点和权衡。

### 调度域隔离

该特性将 CPU 从调度器拓扑中隔离出来。结果，目标不再参与负载均衡。任务也不会迁移到它或从它
迁移走，除非显式设置了亲和性。

作为副作用，该 CPU 也从未绑定的工作队列和未绑定的内核线程中隔离出来。

#### 需求

- 基于 cpusets 的接口需要 CONFIG_CPUSETS=y

#### 权衡

就其本质而言，由于一些 CPU 从全局负载均衡中被抽离，系统负载总体上分布得更少。

#### 接口

- 推荐 Documentation/admin-guide/cgroup-v2.rst 中的 cpuset 隔离分区，因为它们可以在运行时
  调整。

- 'isolcpus=' 内核启动参数带有 'domain' 标志，是一个灵活性较低的替代方案，不允许运行时重新
  配置。

### IRQ 隔离

尽可能隔离 IRQ，使其不在目标 CPU 上触发。

#### 接口

- 文件 /proc/irq/\*/smp_affinity，详见 Documentation/core-api/irq/irq-affinity.rst 页面。

- 用于默认设置的 "irqaffinity=" 内核启动参数。

- "isolcpus=" 内核启动参数中的 "managed_irq" 标志会对受管理的 IRQ 尽力进行亲和性覆盖。

### 完全动态 tick（Full Dynticks，即 nohz_full）

完全动态 tick 将 dynticks 空闲模式（CPU 空闲时停止 tick）扩展到运行单个用户空间任务的 CPU。
也就是说，如果环境允许，定时器 tick 会被停止。

全局定时器回调也从 nohz_full CPU 上隔离。

#### 需求

- CONFIG_NO_HZ_FULL=y

#### 约束

- 隔离的 CPU 必须只运行单个任务。多任务需要 tick 来维持抢占。这通常没问题，因为工作负载通常
  无法承受随机上下文切换的延迟。

- 隔离的 CPU 不得调用内核，否则有触发随机噪声的风险。

- 隔离的 CPU 上不得使用 POSIX CPU 定时器。

- 架构必须拥有稳定可靠的时钟源（没有需要看门狗的不可靠 TSC）。

#### 权衡

就成本而言，这是侵入性最强的隔离特性。假定在工作负载将大部分时间花在用户空间、并且除了准备
性工作外不依赖内核时使用，因为：

- RCU 由于加锁、卸载和线程化的回调处理（与 "rcu_nocbs" 启动参数所获得的效果相同）而增加
  了更多开销。

- 通过系统调用、异常和 IRQ 进入/退出内核代价更高，因为要将用户空间维持为 RCU 扩展静止状态而
  进行了完全有序的 RmW 操作。此外，CPU 时间是在内核边界上记账，而非通过 tick 周期性记账。

- Housekeeping CPU 必须代表隔离的 CPU 运行一个 1Hz 的残留远程调度器 tick。

## 检查清单

你已经设置了上述每种隔离特性，但仍然观察到抖动毁掉了你的工作负载？在进行之前，务必检查几个
要点。

其中一些检查清单项与实时工作负载类似：

- 使用 mlock() 防止你的页面被换出。缺页通常与对抖动敏感的工作负载不兼容。

- 避免 SMT，以防止你的硬件线程被另一个线程“抢占”。

- CPU 频率变化可能在工作负载中引发微妙的抖动。Cpufreq 应当谨慎使用和调优。

- 深度 C-state 可能在唤醒时导致延迟问题。如果这成为问题，可以通过 processor.max_cstate 或
  intel_idle.max_cstate 等内核启动参数限制 C-state。更细粒度的调优在
  Documentation/admin-guide/pm/cpuidle.rst 页面中描述。

- 你的系统可能会受到源自固件的中断影响——例如 x86 有系统管理中断（SMI）。检查你的系统 BIOS
  以禁用此类干扰，运气好的话你的供应商会提供针对低延迟操作的 BIOS 调优指南。

## 完全隔离示例

在本例中，系统有 8 个 CPU，第 8 个将被完全隔离。由于 CPU 从 0 开始编号，第 8 个 CPU 是 CPU 7。

### 内核参数

设置以下内核启动参数以禁用 SMT，并设置 tick 和 IRQ 隔离：

- 完全动态 tick：nohz_full=7

- IRQ 隔离：irqaffinity=0-6

- 受管理 IRQ 隔离：isolcpus=managed_irq,7

- 阻止 SMT：nosmt

完整的命令行如下：

  nohz_full=7 irqaffinity=0-6 isolcpus=managed_irq,7 nosmt

### CPUSET 配置（cgroup v2）

假设 cgroup v2 已挂载到 /sys/fs/cgroup，以下脚本将 CPU 7 从调度域中隔离。

```
  cd /sys/fs/cgroup
  # Activate the cpuset subsystem
  echo +cpuset > cgroup.subtree_control
  # Create partition to be isolated
  mkdir test
  cd test
  echo +cpuset > cgroup.subtree_control
  # Isolate CPU 7
  echo 7 > cpuset.cpus
  echo "isolated" > cpuset.cpus.partition

```
### 用户空间工作负载

模拟一个纯用户空间工作负载，下面的程序在隔离的 CPU 7 上运行一个空的用户空间循环。

```
  #include <stdio.h>
  #include <fcntl.h>
  #include <unistd.h>
  #include <errno.h>
  int main(void)
  {
      // Move the current task to the isolated cpuset (bind to CPU 7)
      int fd = open("/sys/fs/cgroup/test/cgroup.procs", O_WRONLY);
      if (fd < 0) {
          perror("Can't open cpuset file...\n");
          return 0;
      }

      write(fd, "0\n", 2);
      close(fd);

      // Run an endless dummy loop until the launcher kills us
      while (1)
      ;

      return 0;
  }

```
编译它并保存以备后续步骤使用：

```
  # gcc user_loop.c -o user_loop

```
### 启动器

下面的启动器运行上述程序 10 秒，并跟踪因抢占任务和 IRQ 而产生的噪声。

```
  TRACING=/sys/kernel/tracing/
  # Make sure tracing is off for now
  echo 0 > $TRACING/tracing_on
  # Flush previous traces
  echo > $TRACING/trace
  # Record disturbance from other tasks
  echo 1 > $TRACING/events/sched/sched_switch/enable
  # Record disturbance from interrupts
  echo 1 > $TRACING/events/irq_vectors/enable
  # Now we can start tracing
  echo 1 > $TRACING/tracing_on
  # Run the dummy user_loop for 10 seconds on CPU 7
  ./user_loop &
  USER_LOOP_PID=$!
  sleep 10
  kill $USER_LOOP_PID
  # Disable tracing and save traces from CPU 7 in a file
  echo 0 > $TRACING/tracing_on
  cat $TRACING/per_cpu/cpu7/trace > trace.7

```
如果没有出现特定问题，trace.7 的输出应如下所示：

```
  <idle>-0 [007] d..2. 1980.976624: sched_switch: prev_comm=swapper/7 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=user_loop next_pid=1553 next_prio=120
  user_loop-1553 [007] d.h.. 1990.946593: reschedule_entry: vector=253
  user_loop-1553 [007] d.h.. 1990.946593: reschedule_exit: vector=253

```
也就是说，在 user_loop 运行的 10 秒内，第一次跟踪和第二次跟踪之间没有触发特定的噪声。

## 调试

当然事情从来不会这么简单，尤其是在这个问题上。很可能在前述 trace.7 文件中观察到实际的噪声。

进一步调查的最佳方法是启用更细粒度的跟踪点，例如产生异步事件的子系统的跟踪点：workqueue、
timer、irq_vector 等。启用 tick_stop 事件来诊断为何在那种情况下 tick 被保留也很有意义。

一些工具对于更高层次的分析也很有用：

- Documentation/tools/rtla/rtla.rst 提供了一套用于分析系统中延迟和噪声的工具。例如
  Documentation/tools/rtla/rtla-osnoise.rst 运行一个内核跟踪器，分析并输出噪声的摘要。

- dynticks-testing 做的事情类似于 rtla-osnoise，但在用户空间进行。它位于
  git://git.kernel.org/pub/scm/linux/kernel/git/frederic/dynticks-testing.git
