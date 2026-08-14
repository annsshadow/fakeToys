## Monitor wip（抢占式唤醒监视器）


- 名称：wip - 抢占式唤醒
- 类型：每 CPU 确定性自动机
- 作者：Daniel Bristot de Oliveira <bristot@kernel.org>

### 描述


抢占式唤醒（wip）监视器是一个示例性的每 CPU 监视器，用于验证唤醒事件是否始终在以下状态下发生：
```

                     |
                     |
                     v
                   #==================#
                   H    preemptive    H <+
                   #==================#  |
                     |                   |
                     | preempt_disable   | preempt_enable
                     v                   |
    sched_waking   +------------------+  |
  +--------------- |                  |  |
  |                |  non_preemptive  |  |
  +--------------> |                  | -+
                   +------------------+

```
由于调度器同步的原因，唤醒事件始终在抢占被禁用的情况下发生。然而，由于 preempt_count 及其 trace 事件相对于中断并非原子操作，某些
```
  preempt_disable() {
	__preempt_count_add(1)
	------->	smp_apic_timer_interrupt() {
				preempt_disable()
					do not trace (preempt count >= 1)

				wake up a thread

				preempt_enable()
					 do not trace (preempt count >= 1)
			}
	<------
	trace_preempt_disable();
  }
```
此问题在此处被报告并讨论：
  https://lore.kernel.org/r/cover.1559051152.git.bristot@redhat.com/

### 规格说明

Grapviz Dot 文件位于 tools/verification/models/wip.dot
