## 面向架构特定代码的 CPU 调度器实现提示


	Nick Piggin, 2005

## 上下文切换

1. 运行队列（Runqueue）加锁
默认情况下，switch_to 架构函数是在运行队列加锁的情况下调用的。这通常不是问题，除非
switch_to 可能需要获取运行队列锁。这通常是由于上下文切换中的唤醒操作所致。

要请求调度器在未加锁运行队列的情况下调用 switch_to，你必须在头文件（通常是定义 switch_to
的文件）中 `#define __ARCH_WANT_UNLOCKED_CTXSW`。

未加锁的上下文切换在 CONFIG_SMP 情况下只会对核心调度器实现引入非常微小的性能开销。

## CPU 空闲

你的 cpu_idle 例程需要遵守以下规则：

1. 抢占（preempt）现在应在空闲例程期间保持禁用。只应在调用 schedule() 时启用，随后再次禁用。

2. need_resched/TIF_NEED_RESCHED 只会被设置，并且在运行任务调用 schedule() 之前永远不会
   被清除。空闲线程只需要查询 need_resched，而绝不应设置或清除它。

3. 当 cpu_idle 发现（need_resched() == 'true'）时，它应当调用 schedule()。在其他情况下
   不应调用 schedule()。

4. 检查 need_resched 时需要禁用中断的唯一时机，是当我们即将让处理器休眠直到下一次中断时
   （这并不提供对 need_resched 的任何保护，它防止丢失一个中断）：

```

	        local_irq_disable();
	        if (!need_resched()) {
	                local_irq_enable();
	                *** resched interrupt arrives here ***
	                __asm__("sleep until next interrupt");
	        }

```
5. TIF_POLLING_NRFLAG 可由空闲例程设置，当 need_resched 变高时它们不需要中断来唤醒。
   换句话说，它们必须周期性地轮询 need_resched，尽管进入较低的 CPU 优先级或做一些后台工作
   可能是合理的。

      - 5a. 如果设置了 TIF_POLLING_NRFLAG，并且我们确实决定进入中断休眠，则需要先清除它，
	然后发出一个内存屏障（随后在禁用中断的情况下测试 need_resched，如第 3 点所述）。

arch/x86/kernel/process.c 中包含轮询与休眠两种空闲函数的示例。


## 可能的架构问题


我发现的可能的架构问题（要么尝试修复，要么未修复）：

sparc - 此时中断已开启（?），将 local_irq_save 改为 _disable。
      - TODO：需要次要 CPU 禁用抢占（参见 #1）
