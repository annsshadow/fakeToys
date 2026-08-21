## 系统挂起与设备中

Copyright (C) 2014 Intel Corp.
Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


### 挂起与恢复设IRQ


设备中断请求线（IRQ）通常在系统挂起期间、设备挂起的“late”阶段之后被禁用（即，在所有设备的 ->prepare>suspend ->suspend_late 回调都已执行之后）。这是由 suspend_device_irqs() 完成的
这样做的理由是：在设备挂起的“late”阶段之后，已挂起设备的任何中断都没有正当的理由触发；而且如果有任何设备尚未正确挂起，无论如何最好阻止来自它们的中断。此外，过去我们曾遇到共IRQ 的中断处理程序问题——实现它们的设备驱动并没有为设备挂起后中断触发做好准备。在某些情况下，它们会尝试访问（例如）已挂起设备的内存地址空间，从而导致不可预测的行为。遗憾的是，这类问题非常难以调试，而引suspend_device_irqs() 以及设备挂起和恢复的“noirq”阶段，是缓解这些问题的唯一实用方法
设备 IRQ 在系统恢复期间被重新启用，就在设备恢复的“early”阶段之前（即，在开始执行设备的 ->resume_early 回调之前）。完成此操作的函数是 resume_device_irqs()

### IRQF_NO_SUSPEND 标志


存在可以在整个系统挂恢复周期中合法触发的中断，包括设备挂起和恢复的“noirq”阶段，以及非引导（nonboot）CPU 下线又上线的期间。这首先适用于定时器中断，但也适用IPI 和一些其他特殊用途中断
在请求特殊用途中断时，IRQF_NO_SUSPEND 标志用于IRQ 子系统表明这一点。它suspend_device_irqs() 保持相应IRQ 启用，从而允许该中断在挂恢复周期中按预期工作，但它不保证该中断会将系统从挂起状态唤醒——对于这种情况，有必要使enable_irq_wake()
注意，IRQF_NO_SUSPEND 标志影响的是整个 IRQ，而不仅仅是它的某一个使用者。因此，如果 IRQ 是共享的，即使其中某些使用者没有向 request_irq()（或等效函数）传IRQF_NO_SUSPEND 标志，安装在IRQ 上的所有中断处理程序在 suspend_device_irqs() 之后仍会照常执行。因此，应避免同时使IRQF_NO_SUSPEND IRQF_SHARED

### 系统唤醒中断、enable_irq_wake() disable_irq_wake()


系统唤醒中断通常需要被配置为从睡眠状态唤醒系统，特别是如果它们在工作状态中用于不同目的（如作为 I/O 中断）时
这可能涉及在平台（如 SoC）内开启特殊的信号处理逻辑，使得来自某条线的信号在系统睡眠期间以不同方式路由，以便在需要时触发系统唤醒。例如，平台可能包含一个专门用于处理系统唤醒事件的专用中断控制器。那么，如果某条中断线应当从睡眠状态唤醒系统，就需要启用该中断控制器相应的输入，以接收来自该线的信号。唤醒之后，通常最好禁用该输入，以防止专用控制器不必要地触发中断
IRQ 子系统提供了两个供设备驱动用于这些目的的辅助函数。即，enable_irq_wake() 开启平台将给定 IRQ 作为系统唤醒中断线处理的逻辑，disable_irq_wake() 关闭该逻辑
调用 enable_irq_wake() 会使 suspend_device_irqs() 以特殊方式处理给IRQ。即，IRQ 保持启用，但在第一次中断时它会被禁用，标记为挂起（pending）和“suspended”，以便它在随后的系统恢复中resume_device_irqs() 重新启用。同时，PM 核心会收到该事件的通知，导致正在进行的系统挂起被中止（这不必立即发生，而是在挂起线程查找挂起唤醒事件的某个检查点发生）
这样，来自唤醒中断源的每个中断要么导致当前正在进行的系统挂起被中止，要么在已经挂起时唤醒系统。然而，suspend_device_irqs() 之后，系统唤IRQ 的中断处理程序不会被执行。此时它们只IRQF_NO_SUSPEND IRQ 执行，但那些 IRQ 不应通过 enable_irq_wake() 配置为系统唤醒

### 中断与挂起至空闲（Suspend-to-Idle

挂起至空闲（Suspend-to-idle，也称为“freeze”睡眠状态）是一种相对较新的系统睡眠状态，其工作方式是在设备挂起的“noirq”阶段之后使所有处理器空闲并等待中断
当然，这意味着所有设置了 IRQF_NO_SUSPEND 标志的中断在该状态下会将 CPU 带出空闲，但它们不会导致 IRQ 子系统触发系统唤醒
相应地，系统唤醒中断会触发从挂起至空闲状态的唤醒，与它们在完整系统挂起情况下的行为类似。唯一的区别是，从挂起至空闲的唤醒使用通常的工作状态中断传递机制发出信号，不需要平台使用任何特殊的中断处理逻辑来使其工作

### IRQF_NO_SUSPEND 涓?enable_irq_wake()


在同一IRQ 上同时使enable_irq_wake() IRQF_NO_SUSPEND 标志的理由极少，并且绝不应该对同一个设备同时使用两者
首先，如IRQ 不共享，处理 IRQF_NO_SUSPEND 中断的规则（中断处理程序suspend_device_irqs() 之后被调用）与处理系统唤醒中断的规则（中断处理程序在 suspend_device_irqs() 之后不被调用）直接冲突
其次，enable_irq_wake() IRQF_NO_SUSPEND 都适用于整IRQ，而不是单个中断处理程序，因此在系统唤醒中断源IRQF_NO_SUSPEND 中断源之间共IRQ 通常没有意义
在极少数情况下，IRQ 可以在唤醒设备驱动和 IRQF_NO_SUSPEND 使用者之间共享。为了使其安全，唤醒设备驱动必须能够区分虚假 IRQ 与真正的唤醒事件（通过 pm_system_wakeup() 将后者通知核心），必须使用 enable_irq_wake() 确保IRQ 作为唤醒源工作，并且必须IRQF_COND_SUSPEND 请求IRQ，以告知核心它满足这些要求。如果不满足这些要求，则使用 IRQF_COND_SUSPEND 是无效的