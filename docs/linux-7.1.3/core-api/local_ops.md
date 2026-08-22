


## 本地原子操作的语义与行为


:Author: Mathieu Desnoyers


本文档解释了本地原子操作的目的、如何为任意给定架构实现它们，并说明如何正确
使用它们。它还强调了当内存写入顺序很重要时，CPU 读取这些本地变量必须采取
的预防措施

    注意，基`local_t` 的操作不建议用于通用的内核场景。除非确实有特殊用途，
    否则请改`this_cpu` 操作。内核中 `local_t` 的大部分用法都已`this_cpu`
    操作所取代。`this_cpu` 操作将重定位与类`local_t` 的语义合并到单条指令中，
    从而生成更紧凑、执行更快的代码

## 本地原子操作的目

本地原子操作旨在提供快速且高度可重入的CPU 计数器。它们通过去除通常用于CPU 同步LOCK 前缀和内存屏障，将标准原子操作的性能开销降到最低
在许多情况下，拥有快速的CPU 原子计数器很有价值：它不需要禁用中断来保护
中断处理程序，并且允许在 NMI 处理程序中保持一致的计数器。它对于跟踪目的以及
各种性能监视计数器特别有用
本地原子操作仅保证变量修改相对于拥有该数据的 CPU 是原子的。因此，必须小心
确保只有一CPU 写入 `local_t` 数据。这是通过使用CPU 数据并确保在可安抢占的上下文中修改它来实现的。但允许从任CPU 读取 `local_t` 数据：此时它表现为相对于所有CPU 的其他内存写入是乱序的

## 针对特定架构的实

这可以通过稍微修改标准原子操作来实现：只保留它们的 UP 变体。这通常意味着
移除 LOCK 前缀（在 i386 x86_64 上）以及任何 SMP 同步屏障。如果架构在 SMP
UP 之间没有不同行为，那么在您架构的 `local.h` 中包`asm-generic/local.h`
即可
`local_t` 类型通过将一`atomic_long_t` 嵌入结构中，被定义为一个不透明`signed long`。这样做是为了使从该类型
```
    typedef struct { atomic_long_t a; } local_t;

```
## 使用本地原子操作时应遵循的规

- 被本地操作访问的变量必须是每 CPU 变量- **只有**这些变量CPU 所有者才能写入它们- CPU 可以从任何上下文（进程、irq、softirq、nmi……）使用本地操作来更新其
  `local_t` 变量- 在进程上下文中使用本地操作时，必须禁用抢占（或中断），以确保进程在获取每
  CPU 变量到执行实际本地操作之间不会被迁移到不同的 CPU- 在中断上下文中使用本地操作时，在主线程内核上无需特别小心，因为它们会在本  CPU 上运行，且抢占已被禁用。不过，我仍建议显式禁用抢占，以确保其在 -rt 内核
  上仍能正常工作- 读取本地 CPU 变量将得到该变量的当前副本- 可以从任CPU 读取这些变量，因为对对齐"`long`" 变量的更新始终是原子的  由于写入 CPU 不进行内存同步，当读取某*其他** CPU 的变量时，可能读到一  过时的变量副本

## 如何使用本地原子操作


```
    #include <linux/percpu.h>
    #include <asm/local.h>

    static DEFINE_PER_CPU(local_t, counters) = LOCAL_INIT(0);

```
## 计数


计数针对有符号长整型的所有位进行
在可抢占上下文中，在本地原子操作前后使用 `get_cpu_var()` `put_cpu_var()`可确保写入周围的抢占被禁
```
    local_inc(&get_cpu_var(counters));
    put_cpu_var(counters);

```
如果您已经处于可安全抢占的上下文中，可以使用

```
    local_inc(this_cpu_ptr(&counters));



```
## 读取计数

可以从其CPU 读取这些本地计数器以对计数求和。请注意，跨 CPU local_read
所见的数据应被视为乱序
```
    long sum = 0;
    for_each_online_cpu(cpu)
            sum += local_read(&per_cpu(counters, cpu));

```
如果您想使用远程 local_read 来在 CPU 之间同步对资源的访问，则必须在写CPU
和读CPU 上分别使用显式的 `smp_wmb()` `smp_rmb()` 内存屏障。如果您`local_t` 变量用作缓冲区中已写入字节的计数器，就会是这样的情况：在缓冲区写与计数器递增之间应有一`smp_wmb()`，在计数器读取与缓冲区读取之间也应有一`smp_rmb()`

下面是一个示例模块，它使
```
    /* test-local.c
     *
     * Sample module for local.h usage.
     */

    #include <asm/local.h>
    #include <linux/module.h>
    #include <linux/timer.h>

    static DEFINE_PER_CPU(local_t, counters) = LOCAL_INIT(0);

    static struct timer_list test_timer;

    /* IPI called on each CPU. */
    static void test_each(void *info)
    {
            /* Increment the counter from a non preemptible context */
            printk("Increment on cpu %d\n", smp_processor_id());
            local_inc(this_cpu_ptr(&counters));

            /* This is what incrementing the variable would look like within a
             * preemptible context (it disables preemption) :
             *
             * local_inc(&get_cpu_var(counters));
             * put_cpu_var(counters);
             */
    }

    static void do_test_timer(unsigned long data)
    {
            int cpu;

            /* Increment the counters */
            on_each_cpu(test_each, NULL, 1);
            /* Read all the counters */
            printk("Counters read from CPU %d\n", smp_processor_id());
            for_each_online_cpu(cpu) {
                    printk("Read : CPU %d, count %ld\n", cpu,
                            local_read(&per_cpu(counters, cpu)));
            }
            mod_timer(&test_timer, jiffies + 1000);
    }

    static int __init test_init(void)
    {
            /* initialize the timer that will increment the counter */
            timer_setup(&test_timer, do_test_timer, 0);
            mod_timer(&test_timer, jiffies + 1);

            return 0;
    }

    static void __exit test_exit(void)
    {
            timer_shutdown_sync(&test_timer);
    }

    module_init(test_init);
    module_exit(test_exit);

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mathieu Desnoyers");
    MODULE_DESCRIPTION("Local Atomic Ops");

```
