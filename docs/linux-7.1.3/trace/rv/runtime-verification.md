## 运行时验证（Runtime Verification

运行时验证（Runtime Verification，RV）是一种轻量（但严谨）的方法，它用一种对复杂系统更实用的方式补充了经典的穷尽式验证技术（模型检测（model checking*定理证明（theorem proving*）
RV 不依赖于系统的细粒度模型（例如在指令级重新实现），而是通过分析与系统行为的形式化规范（formal specification相对比的系统实际执行的迹（trace）来工作
其主要优势在于，RV 能够提供被监视系统运行时行为的精确信息，而避免了开发需要以建模语言重新实现整个系统模型所带来的陷阱。此外，给定一种高效的监视方法，就有可能执行系统的**在线（online*验证从而对意外事件作出**反应（reaction*，例如避免故障在安全关键（safety-critical）系统上的传播
## 运行时监视器与反应器


监视器（monitor）是系统运行时验证的核心部分。监视器处于期望（或不期望）行为的形式化规范与实际系统的迹之间
Linux 的术语来说，运行时验证监视器被封装在 **RV monitor** 抽象之中。一**RV monitor** 包含一个系统的
参考模型（reference model）、一组监视器实例（例如每 CPU 监视器、每任务监视器等等），以及通过
以下方式将监视器与系统粘合在一起的辅助函数
```
 Linux   +---- RV Monitor ----------------------------------+ Formal
  Realm  |                                                  |  Realm
  +-------------------+     +----------------+     +-----------------+
  |   Linux kernel    |     |     Monitor    |     |     Reference   |
  |     Tracing       |  -> |   Instance(s)  | <-  |       Model     |
  | (instrumentation) |     | (verification) |     | (specification) |
  +-------------------+     +----------------+     +-----------------+
         |                          |                       |
         |                          V                       |
         |                     +----------+                 |
         |                     | Reaction |                 |
         |                     +--+--+--+-+                 |
         |                        |  |  |                   |
         |                        |  |  +-> trace output ?  |
         +------------------------|--|----------------------+
                                  |  +----> panic ?
                                  +-------> <user-specified>
```
除了对系统进行验证和监视外，监视器还可以对意外事件作出反应。反应的形式可以多种多样，从记录
事件发生的日志，到强制正确行为，再到极端的关闭系统以避免故障传播
Linux 的术语来说，**reactor（反应器* 是一种供 **RV monitor** 使用的反应方法默认情况下，所有监视器都应提供其动作的 trace 输出，这本身已经是一种反应。此外，还会提供其他反应以便用户根据需要启用它们
关于运行时验证原理以及应用于 Linux RV 的更多信息：

  Bartocci, Ezio, et al. **Introduction to runtime verification.** In: Lectures on
  Runtime Verification. Springer, Cham, 2018. p. 1-33.

  Falcone, Ylies, et al. **A taxonomy for classifying runtime verification tools.**
  In: International Conference on Runtime Verification. Springer, Cham, 2018. p.
  241-262.

  De Oliveira, Daniel Bristot. *Automata-based formal analysis and
  verification of the real-time Linux kernel.* Ph.D. Thesis, 2020.

## 在线 RV 监视

监视器可分为 **offline（离线）** **online（在线）** 监视器*Offline**
监视器在事件发生后处理系统生成的迹，通常是从永久存储系统读取迹执行*Online** 监视在系统执行期间处理迹。如果在事件监视期间处理事件附加于系统执行、并在事件监视期间阻塞系统，
则在线监视器被称*同步（synchronous*。另一方面*异步（asynchronous* 监视器的执行与系统相分离每种类型的监视器都有一系列优点。例如，**offline** 监视器可以在不同的机器上执行，但需要将日志保存文件的操作。相比之下，**synchronous online（同步在线）** 方法可以在违规发生的确切时刻作出反应
关于监视器的另一个重要方面是与事件分析相关的开销。如果系统生成事件的频率高于监视器在同一系统处理它们的能力，则只**offline** 方法是可行的。另一方面，如果事件追踪带来的开销高于监视器对单个事件简单处理，那么 **synchronous online** 监视器将带来更低的开销
事实上，以下研究所呈现的内容：

  De Oliveira, Daniel Bristot; Cucinotta, Tommaso; De Oliveira, Romulo Silva.
  **Efficient formal verification for the Linux kernel.** In: International
  Conference on Software Engineering and Formal Methods. Springer, Cham, 2019.
  p. 315-332.

表明，对于确定性自动机（Deterministic Automata）模型，在内核中同步处理事件造成的开销低于将相同事保存到迹缓冲区，甚至还没算上为用户空间分析收集迹的开销。这推动了内核内接口（in-kernel interface）在线监视器的开发
关于使用自动机对 Linux 内核行为进行建模的更多信息，参见
  De Oliveira, Daniel B.; De Oliveira, Romulo S.; Cucinotta, Tommaso. *A thread
  synchronization model for the PREEMPT_RT Linux kernel.* Journal of Systems
  Architecture, 2020, 107: 101729.

## 用户接口


用户接口（是故意）类似于追踪（tracing）接口。它当前位于 "/sys/kernel/tracing/rv/"
当前可用的文文件夹如下：

**available_monitors**

- 读取它会逐行列出可用的监视器

```
   # cat available_monitors
   wip
   wwnr
```

**available_reactors**

- 读取它会逐行显示可用的反应器
```
   # cat available_reactors
   nop
   panic
   printk
```

**enabled_monitors**锛。
- 读取它会列出已启用的监视器，每行一- 写入它会启用给定的监视器
- 写入'!' 前缀的监视器名称会禁用它
- 截断该文件会禁用所有已启用的监视器

```
   # cat enabled_monitors
   # echo wip > enabled_monitors
   # echo wwnr >> enabled_monitors
   # cat enabled_monitors
   wip
   wwnr
   # echo '!wip' >> enabled_monitors
   # cat enabled_monitors
   wwnr
   # echo > enabled_monitors
   # cat enabled_monitors
   #
```

注意，可以同时启用多个监视器
**monitoring_on**

这是一个用于监视的开关式总开关。它类似trace 接口中的
"tracing_on" 开关
- 写入 "0" 会停止监- 写入 "1" 会继续监- 读取它会返回监视的当前状
注意，它不会禁用已启用的监视器，而是停止监视从系统接收事件的每实体（per-entity）监视器
**reacting_on**

- 写入 "0" 会阻止反应发- 写入 "1" 会启用反- 读取它会返回反应的当前状
**monitors/**

每个监视器在 "monitors/" 内会有自己的目录。那里会展示监视器特定的文件monitors/" 目录类似tracefs 上的 "events" 目录
```
   # cd monitors/wip/
   # ls
   desc  enable
   # cat desc
   wakeup in preemptive per-cpu testing monitor.
   # cat enable
   0
```

**monitors/MONITOR/desc**

- 读取它会显示监视**MONITOR** 的描
**monitors/MONITOR/enable**

- 写入 "0" 会禁**MONITOR**
- 写入 "1" 会启**MONITOR**
- 读取它会返回 **MONITOR** 的当前状
**monitors/MONITOR/reactors**

- 列出可用的反应器，给**MONITOR** 的所选反应位"[]" 内。默认的nop（无操作）反应器- 写入反应器的名称会将其启用到给定MONITOR
```
   # cat monitors/wip/reactors
   [nop]
   panic
   printk
   # echo panic > monitors/wip/reactors
   # cat monitors/wip/reactors
   nop
   [panic]
   printk
```
