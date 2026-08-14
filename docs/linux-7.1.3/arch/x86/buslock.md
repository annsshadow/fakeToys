


## 总线锁检测与处理


:Copyright: |copy| 2021 Intel Corporation
:Authors: - Fenghua Yu <fenghua.yu@intel.com>
          - Tony Luck <tony.luck@intel.com>

## 问题


拆分锁（split lock）是指操作数跨越两个缓存行的任何原子操作。由于操作数跨越两个
缓存行，且该操作必须是原子的，因此在 CPU 访问这两个缓存行的同时，系统会锁定总线。

总线锁可以通过对回写（WB）内存的拆分锁定访问，或对非 WB 内存的任何锁定访问来获取。
这通常比缓存行内的原子操作慢数千个周期。它还会破坏其他核心上的性能，并使整个系统
陷入瘫痪。

## 检测


Intel 处理器可能支持以下一种或两种硬件机制来检测拆分锁和总线锁。一些 AMD 处理器
也支持总线锁检测。

### 用于拆分锁检测的 #AC 异常


从 Tremont Atom CPU 开始，当尝试进行拆分锁操作时，拆分锁操作可能会引发对齐检查
（#AC）异常。

### 用于总线锁检测的 #DB 异常


某些 CPU 能够在用户指令获取总线锁并执行后，通过 #DB 陷阱通知内核。这使内核能够
终止应用程序或强制执行限流。

## 软件处理


内核的 #AC 和 #DB 处理程序根据内核参数 "split_lock_detect" 来处理总线锁。以下是
不同选项的摘要：

+------------------+----------------------------+-----------------------+
|split_lock_detect=|#AC for split lock		|#DB for bus lock	|
+------------------+----------------------------+-----------------------+
|off	  	   |Do nothing			|Do nothing		|
+------------------+----------------------------+-----------------------+
|warn		   |Kernel OOPs			|Warn once per task and |
|(default)	   |Warn once per task, add a	|and continues to run.  |
|		   |delay, add synchronization	|			|
|		   |to prevent more than one	|			|
|		   |core from executing a	|			|
|		   |split lock in parallel.	|			|
|		   |sysctl split_lock_mitigate	|			|
|		   |can be used to avoid the	|			|
|		   |delay and synchronization	|			|
|		   |When both features are	|			|
|		   |supported, warn in #AC	|			|
+------------------+----------------------------+-----------------------+
|fatal		   |Kernel OOPs			|Send SIGBUS to user.	|
|		   |Send SIGBUS to user		|			|
|		   |When both features are	|			|
|		   |supported, fatal in #AC	|			|
+------------------+----------------------------+-----------------------+
|ratelimit:N	   |Do nothing			|Limit bus lock rate to	|
|(0 < N <= 1000)   |				|N bus locks per second	|
|		   |				|system wide and warn on|
|		   |				|bus locks.		|
+------------------+----------------------------+-----------------------+

## 用途


检测和处理总线锁可以在多个领域找到用途：

对于构建整合式实时系统的实时系统设计者而言，这至关重要。这些系统在某些核心上运行
硬实时代码，并在其他核心上运行“不受信任”的用户进程。硬实时无法承受来自不受信任
进程的任意总线锁损害实时性能。迄今为止，设计者一直无法部署这些解决方案，因为他们
无法阻止“不受信任”的用户代码生成拆分锁和总线锁，从而在总线锁定期间阻塞硬实时代码
访问内存。

它对于一般计算也很有用，可防止客户机或用户应用程序通过执行带总线锁的指令拖慢整个
系统。


## 指南

### off


禁用对拆分锁和总线锁的检查。如果存在以较低频率触发这些事件的遗留应用程序，从而
不需要缓解措施，则此选项可能有用。

### warn


检测到总线锁时会发出警告，从而可以识别有问题的应用程序。这是默认行为。

### fatal


在这种情况下，不允许总线锁，进程会被杀死。

### ratelimit


指定一个系统范围的总线锁速率限制 N，其中 0 < N <= 1000。这允许高达每秒 N 个总线锁
的速率。当总线锁速率超过该限制时，任何通过总线锁 #DB 异常捕获的任务都会被强制休眠
进行限流，直到速率再次降到限制以下。

在可以容忍最小影响、但必须防止最终拒绝服务攻击的情况下，这是一种有效的缓解措施。
它可以识别有问题的进程，并分析它们是恶意的还是仅仅编写得很差。

选择 1000 的速率限制允许总线每秒被锁定多达约七百万个周期（假设每个总线锁 7000
个周期）。在 2 GHz 处理器上，这大约是 0.35% 的系统减速。
