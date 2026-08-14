## I/O 设备的运行时电源管理框架


(C) 2009-2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.

(C) 2010 Alan Stern <stern@rowland.harvard.edu>

(C) 2014 Intel Corp., Rafael J. Wysocki <rafael.j.wysocki@intel.com>

## 1. 引言


I/O 设备的运行时电源管理（runtime PM）支持由电源管理核心（PM core）在以下层面提供：

- 电源管理工作队列 pm_wq，总线类型和设备驱动可以在其中放入它们与 PM 相关的工作项。强烈建议 pm_wq 用于排队所有与运行时 PM 相关的工作项，因为这使得它们能够与系统范围的电源状态切换（挂起到 RAM、休眠以及从系统睡眠状态恢复）保持同步。pm_wq 在 include/linux/pm_runtime.h 中声明，在 kernel/power/main.c 中定义。

- 'struct device' 的 'power' 成员（其类型为 'struct dev_pm_info'，定义在 include/linux/pm.h 中）中的若干运行时 PM 字段，可用于把运行时 PM 操作彼此同步。

- 'struct dev_pm_ops'（定义在 include/linux/pm.h 中）中的三个设备运行时 PM 回调。

- 一组定义在 drivers/base/power/runtime.c 中的辅助函数，可用于执行运行时 PM 操作，而这些操作之间的同步由 PM core 负责。鼓励总线类型和设备驱动使用这些函数。

'struct dev_pm_ops' 中的运行时 PM 回调、'struct dev_pm_info' 的设备运行时 PM 字段以及为运行时 PM 提供的核心辅助函数将在下面描述。

## 2. 设备运行时 PM 回调


```
  struct dev_pm_ops {
	...
	int (*runtime_suspend)(struct device *dev);
	int (*runtime_resume)(struct device *dev);
	int (*runtime_idle)(struct device *dev);
	...
  };

```
PM core 为该设备的子系统执行 ->runtime_suspend()、->runtime_resume() 和 ->runtime_idle() 回调，子系统可能是以下之一：

  1. 设备的 PM 域（PM domain），如果设备的 PM 域对象 dev->pm_domain 存在。

  2. 设备的设备类型（device type），如果 dev->type 和 dev->type->pm 都存在。

  3. 设备的设备类（device class），如果 dev->class 和 dev->class->pm 都存在。

  4. 设备的总线类型（bus type），如果 dev->bus 和 dev->bus->pm 都存在。

如果应用上述规则所选择的子系统没有提供相关的回调，PM core 将直接调用存储在 dev->driver->pm 中对应的驱动回调（如果存在的话）。

PM core 总是按上面给出的顺序检查使用哪个回调，因此回调的优先级从高到低为：PM 域、设备类型、类和总线类型。此外，高优先级的回调总是优先于低优先级的回调。PM 域、总线类型、设备类型和类回调在下面被称为子系统级（subsystem-level）回调。

默认情况下，回调总是在中断使能的进程上下文中被调用。不过，pm_runtime_irq_safe() 辅助函数可用来告诉 PM core，在中断禁用、原子上下文中运行该给定设备的 ->runtime_suspend()、->runtime_resume() 和 ->runtime_idle() 回调是安全的。这意味着相关的回调例程不能阻塞或睡眠，但也意味着第 4 节末尾列出的同步辅助函数可用于该设备的中断处理程序中，或一般而言在原子上下文中使用。

子系统级挂起回调（如果存在）_完全_ _负责_ 以适当的方式处理设备的挂起，其中可以（但不需要）包括执行设备驱动自身的 ->runtime_suspend() 回调（从 PM core 的角度看，只要子系统级挂起回调知道如何处理该设备，就无需在设备驱动中实现 ->runtime_suspend() 回调）。

  - 一旦子系统级挂起回调（或者如果直接调用则为驱动挂起回调）为给定设备成功完成，PM core 就认为该设备已被挂起，这未必意味着它已被置于低功耗状态。不过，它应当意味着在为其执行适当的恢复回调之前，该设备将不处理数据、也不与 CPU 和 RAM 通信。挂起回调成功执行后，设备的运行时 PM 状态为 'suspended'。

  - 如果挂起回调返回 -EBUSY 或 -EAGAIN，设备的运行时 PM 状态保持为 'active'，这意味着之后该设备 _必须_ 完全可操作。

  - 如果挂起回调返回一个不同于 -EBUSY 和 -EAGAIN 的错误码，PM core 将此视为致命错误，并拒绝运行第 4 节描述的辅助函数，直到其状态被直接设置为 'active' 或 'suspended'（PM core 为此提供了特殊的辅助函数）。

特别地，如果驱动需要远程唤醒能力（即允许设备请求改变其电源状态的硬件机制，例如 PCI PME）才能正常工作，而 device_can_wakeup() 对该设备返回 'false'，那么 ->runtime_suspend() 应当返回 -EBUSY。另一方面，如果 device_can_wakeup() 对该设备返回 'true'，并且在执行挂起回调期间设备被置于低功耗状态，则预期将为该设备启用远程唤醒。一般而言，对于在运行时被置于低功耗状态的所有输入设备，都应启用远程唤醒。

子系统级恢复回调（如果存在）**完全负责** 以适当的方式处理设备的恢复，其中可以（但不需要）包括执行设备驱动自身的 ->runtime_resume() 回调（从 PM core 的角度看，只要子系统级恢复回调知道如何处理该设备，就无需在设备驱动中实现 ->runtime_resume() 回调）。

  - 一旦子系统级恢复回调（或者如果直接调用则为驱动恢复回调）成功完成，PM core 就认为该设备完全可操作，这意味着该设备 _必须_ 能够按需完成 I/O 操作。此时设备的运行时 PM 状态为 'active'。

  - 如果恢复回调返回错误码，PM core 将此视为致命错误，并拒绝运行第 4 节描述的辅助函数，直到其状态被直接设置为 'active' 或 'suspended'（通过 PM core 为此提供的特殊辅助函数）。

空闲（idle）回调（如果存在则为子系统级的，否则为驱动的）在设备看上去空闲时由 PM core 执行，这由两个计数器向 PM core 指示：设备的使用计数器（usage counter）和设备"active"子设备的计数器。

  - 如果使用 PM core 提供的辅助函数使其中任一计数器减小，且结果为零，则会检查另一个计数器。如果该计数器也等于零，PM core 就以该设备为参数执行空闲回调。

空闲回调执行的操作完全取决于相关的子系统（或驱动），但预期且推荐的操作是检查设备是否可以被挂起（即挂起该设备所需的全部条件是否都满足），并在这种情况下为设备排队一个挂起请求。如果没有空闲回调，或者回调返回 0，那么 PM core 将尝试对设备执行运行时挂起，同时也尊重配置为自动挂起（autosuspend）的设备。本质上这意味着调用 pm_runtime_autosuspend()。为防止这一点（例如，如果回调例程已经启动了一个延迟挂起），该例程必须返回一个非零值。负的错误返回码会被 PM core 忽略。

PM core 提供的辅助函数（在第 4 节描述）保证针对一个设备的运行时 PM 回调满足以下约束：

(1) 回调之间互斥（例如，禁止与 ->runtime_resume() 或同一设备的另一个 ->runtime_suspend() 实例并行执行 ->runtime_suspend()），唯一的例外是 ->runtime_suspend() 或 ->runtime_resume() 可以与 ->runtime_idle() 并行执行（尽管在为该同一设备执行任何其他回调时，不会启动 ->runtime_idle()）。

(2) ->runtime_idle() 和 ->runtime_suspend() 只能对 'active' 设备执行（即 PM core 只会为运行时 PM 状态为 'active' 的设备执行 ->runtime_idle() 或 ->runtime_suspend()）。

(3) ->runtime_idle() 和 ->runtime_suspend() 只能对使用计数器等于零 _并且_ 其"active"子设备计数器等于零、或其 'power.ignore_children' 标志被置位的设备执行。

(4) ->runtime_resume() 只能对 'suspended' 设备执行（即 PM core 只会为运行时 PM 状态为 'suspended' 的设备执行 ->runtime_resume()）。

此外，PM core 提供的辅助函数遵循以下规则：

  - 如果 ->runtime_suspend() 即将被执行，或者有一个待执行的请求要执行它，则不会为同一设备执行 ->runtime_idle()。

  - 一个执行或调度 ->runtime_suspend() 执行的请求，将取消同一设备的任何待执行的 ->runtime_idle() 执行请求。

  - 如果 ->runtime_resume() 即将被执行，或者有一个待执行的请求要执行它，则不会为同一设备执行其他回调。

  - 一个执行 ->runtime_resume() 的请求将取消同一设备的任何其他回调的待执行或已调度的请求，已调度的自动挂起除外。

## 3. 运行时 PM 设备字段


'struct dev_pm_info'（定义在 include/linux/pm.h 中）中存在以下设备运行时 PM 字段：

  `struct timer_list suspend_timer;`
    - 用于调度（延迟）挂起和自动挂起请求的定时器

  `unsigned long timer_expires;`
    - 定时器到期时间，以 jiffies 计（如果此值不同于零，则定时器正在运行，并将在该时刻到期，否则定时器未运行）

  `struct work_struct work;`
    - 用于排队请求（即 pm_wq 中的工作项）的工作结构

  `wait_queue_head_t wait_queue;`
    - 如果有任何辅助函数需要等待另一个完成时所使用的等待队列

  `spinlock_t lock;`
    - 用于同步的自旋锁

  `atomic_t usage_count;`
    - 设备的使用计数器

  `atomic_t child_count;`
    - 设备的 'active' 子设备计数

  `unsigned int ignore_children;`
    - 如果置位，则忽略 child_count 的值（但仍会更新）

  `unsigned int disable_depth;`
    - 用于禁用辅助函数（如果此值为零则它们正常工作）；其初始值为 1（即所有设备的运行时 PM 初始是禁用的）

  `int runtime_error;`
    - 如果置位，则发生过致命错误（某个回调返回了第 2 节描述的错误码），因此在清除此标志之前辅助函数不会工作；这是失败的回调返回的错误码

  `unsigned int idle_notification;`
    - 如果置位，则 ->runtime_idle() 正在执行

  `unsigned int request_pending;`
    - 如果置位，则有一个待处理的请求（即一个排队进入 pm_wq 的工作项）

  `enum rpm_request request;`
    - 待处理请求的类型（request_pending 置位时有效）

  `unsigned int deferred_resume;`
    - 如果在执行该设备的 ->runtime_suspend() 时 ->runtime_resume() 即将运行，且等待挂起完成不切实际，则置位；意为"一旦你挂起就启动恢复"

  `enum rpm_status runtime_status;`
    - 设备的运行时 PM 状态；此字段的初始值为 RPM_SUSPENDED，这意味着无论其真实硬件状态如何，每个设备在初始时都被 PM core 视为 'suspended'

  `enum rpm_status last_status;`
    - 在为设备禁用运行时 PM 之前捕获的设备最后一次运行时 PM 状态（在初始时以及 disable_depth 为 0 时无效）

  `unsigned int runtime_auto;`
    - 如果置位，表示用户空间已允许设备驱动通过 /sys/devices/.../power/control `interface;` 在运行时对设备进行电源管理；它只能借助 pm_runtime_allow() 和 pm_runtime_forbid() 辅助函数修改

  `unsigned int no_callbacks;`
    - 表示设备不使用运行时 PM 回调（见第 8 节）；它只能由 pm_runtime_no_callbacks() 辅助函数修改

  `unsigned int irq_safe;`
    - 表示 ->runtime_suspend() 和 ->runtime_resume() 回调将在持有自旋锁且中断禁用的情况下被调用

  `unsigned int use_autosuspend;`
    - 表示设备的驱动支持延迟自动挂起（见第 9 节）；它只能由 pm_runtime{_dont}_use_autosuspend() 辅助函数修改

  `unsigned int timer_autosuspends;`
    - 表示 PM core 应在定时器到期时尝试执行自动挂起，而不是普通挂起

  `int autosuspend_delay;`
    - 用于自动挂起的延迟时间（以毫秒计）

  `unsigned long last_busy;`
    - pm_runtime_mark_last_busy() 辅助函数最后一次为该设备被调用的时间（以 jiffies 计）；用于计算自动挂起的非活动时间段

以上所有字段都是 'struct device' 的 'power' 成员的成员。

## 4. 运行时 PM 设备辅助函数


以下运行时 PM 辅助函数定义在 drivers/base/power/runtime.c 和 include/linux/pm_runtime.h 中：

  `void pm_runtime_init(struct device *dev);`
    - 初始化 'struct dev_pm_info' 中的设备运行时 PM 字段

  `void pm_runtime_remove(struct device *dev);`
    - 确保在从设备层次结构中移除该设备后，该设备的运行时 PM 将被禁用

  `int pm_runtime_idle(struct device *dev);`
    - 执行该设备的子系统级空闲回调；失败时返回错误码，其中 -EINPROGRESS 表示 ->runtime_idle() 已在执行；如果没有回调或回调返回 0，则运行 pm_runtime_autosuspend(dev) 并返回其结果

  `int pm_runtime_suspend(struct device *dev);`
    - 执行该设备的子系统级挂起回调；成功时返回 0，如果设备的运行时 PM 状态已经是 'suspended' 则返回 1，失败时返回错误码，其中 -EAGAIN 或 -EBUSY 表示将来尝试挂起该设备是安全的，-EACCES 表示 'power.disable_depth' 不同于 0

  `int pm_runtime_autosuspend(struct device *dev);`
    - 与 pm_runtime_suspend() 相同，只不过会调用 pm_runtime_mark_last_busy() 并在适当时间调度一次自动挂起，并返回 0

  `int pm_runtime_resume(struct device *dev);`
    - 执行该设备的子系统级恢复回调；成功时返回 0，如果设备的运行时 PM 状态已经是 'active'（也包括 'power.disable_depth' 非零、但状态在从 0 变为 1 时是 'active' 的情况）则返回 1，失败时返回错误码，其中 -EAGAIN 表示将来尝试恢复该设备可能是安全的，但还应额外检查 'power.runtime_error'，-EACCES 表示因为 'power.disable_depth' 不同于 0 而无法运行该回调

  `int pm_runtime_resume_and_get(struct device *dev);`
    - 运行 pm_runtime_resume(dev)，如果成功则递增设备的使用计数器；成功时返回 0（无论设备的运行时 PM 状态是否已经是 'active'），失败时返回 pm_runtime_resume() 的错误码。

  `int pm_request_idle(struct device *dev);`
    - 提交一个执行该设备子系统级空闲回调的请求（该请求由 pm_wq 中的一个工作项表示）；成功时返回 0，如果请求未被排队则返回错误码

  `int pm_request_autosuspend(struct device *dev);`
    - 调用 pm_runtime_mark_last_busy()，并在自动挂起延迟到期时调度该设备子系统级挂起回调的执行

  `int pm_schedule_suspend(struct device *dev, unsigned int delay);`
    - 将来调度该设备子系统级挂起回调的执行，其中 'delay' 是在 pm_wq 中排队挂起工作项之前等待的时间，以毫秒计（如果 'delay' 为零，则立即排队工作项）；成功时返回 0，如果设备的 PM 运行时状态已经是 'suspended' 则返回 1，如果请求未被调度（或者在 'delay' 为 0 时未被排队）则返回错误码；如果 ->runtime_suspend() 的执行已经被调度且尚未到期，则 'delay' 的新值将用作等待时间

  `int pm_request_resume(struct device *dev);`
    - 提交一个执行该设备子系统级恢复回调的请求（该请求由 pm_wq 中的一个工作项表示）；成功时返回 0，如果设备的运行时 PM 状态已经是 'active' 则返回 1，如果请求未被排队则返回错误码

  `void pm_runtime_get_noresume(struct device *dev);`
    - 递增设备的使用计数器

  `int pm_runtime_get(struct device *dev);`
    - 递增设备的使用计数器，运行 pm_request_resume(dev) 并返回其结果

  `int pm_runtime_get_sync(struct device *dev);`
    - 递增设备的使用计数器，运行 pm_runtime_resume(dev) 并返回其结果；
      注意它在出错时不会递减设备的使用计数器，因此考虑使用 pm_runtime_resume_and_get() 代替它，特别是在其返回值被调用者检查的情况下，因为这很可能产生更清晰的代码。

  `int pm_runtime_get_if_in_use(struct device *dev);`
    - 如果 'power.disable_depth' 非零则返回 -EINVAL；否则，如果运行时 PM 状态为 RPM_ACTIVE 且运行时 PM 使用计数器非零，则递增计数器并返回 1；否则在不改变计数器的情况下返回 0

  `int pm_runtime_get_if_active(struct device *dev);`
    - 如果 'power.disable_depth' 非零则返回 -EINVAL；否则，如果运行时 PM 状态为 RPM_ACTIVE，则递增计数器并返回 1；否则在不改变计数器的情况下返回 0

  `void pm_runtime_put_noidle(struct device *dev);`
    - 递减设备的使用计数器

  `int pm_runtime_put(struct device *dev);`
    - 递减设备的使用计数器；如果结果为 0，则运行 pm_request_idle(dev) 并返回其结果

  `int pm_runtime_put_autosuspend(struct device *dev);`
    - 将 power.last_busy 字段设为当前时间并递减设备的使用计数器；如果结果为 0，则运行 pm_request_autosuspend(dev) 并返回其结果

  `int __pm_runtime_put_autosuspend(struct device *dev);`
    - 递减设备的使用计数器；如果结果为 0，则运行 pm_request_autosuspend(dev) 并返回其结果

  `int pm_runtime_put_sync(struct device *dev);`
    - 递减设备的使用计数器；如果结果为 0，则运行 pm_runtime_idle(dev) 并返回其结果

  `int pm_runtime_put_sync_suspend(struct device *dev);`
    - 递减设备的使用计数器；如果结果为 0，则运行 pm_runtime_suspend(dev) 并返回其结果

  `int pm_runtime_put_sync_autosuspend(struct device *dev);`
    - 将 power.last_busy 字段设为当前时间并递减设备的使用计数器；如果结果为 0，则运行 pm_runtime_autosuspend(dev) 并返回其结果

  `void pm_runtime_enable(struct device *dev);`
    - 递减设备的 'power.disable_depth' 字段；如果该字段等于零，则运行时 PM 辅助函数可以执行第 2 节描述的子系统级回调

  `int pm_runtime_disable(struct device *dev);`
    - 递增设备的 'power.disable_depth' 字段（如果该字段之前为零，这防止为该设备运行子系统级运行时 PM 回调），确保设备上所有待处理的运行时 PM 操作要么已完成要么已取消；如果有待处理的恢复请求且有必要执行该设备的子系统级恢复回调以满足该请求，则返回 1，否则返回 0

  `void pm_runtime_barrier(struct device *dev);`
    - 检查是否有待处理的恢复请求，并在这种情况下（同步地）恢复它，取消关于它的任何其他待处理运行时 PM 请求，并等待其上所有正在进行的运行时 PM 操作完成

  `void pm_suspend_ignore_children(struct device *dev, bool enable);`
    - 设置/清除设备的 power.ignore_children 标志

  `int pm_runtime_set_active(struct device *dev);`
    - 清除设备的 'power.runtime_error' 标志，将设备的运行时 PM 状态设为 'active'，并适当地更新其父设备的 'active' 子设备计数器（只有在 'power.runtime_error' 被置位或 'power.disable_depth' 大于零时才允许使用此函数）；如果设备有一个父设备不是 'active' 且其 'power.ignore_children' 标志未置位，则它将失败并返回错误码

  `void pm_runtime_set_suspended(struct device *dev);`
    - 清除设备的 'power.runtime_error' 标志，将设备的运行时 PM 状态设为 'suspended'，并适当地更新其父设备的 'active' 子设备计数器（只有在 'power.runtime_error' 被置位或 'power.disable_depth' 大于零时才允许使用此函数）

  `bool pm_runtime_active(struct device *dev);`
    - 如果设备的运行时 PM 状态为 'active' 或其 'power.disable_depth' 字段不等于零，则返回 true，否则返回 false

  `bool pm_runtime_suspended(struct device *dev);`
    - 如果设备的运行时 PM 状态为 'suspended' 且其 'power.disable_depth' 字段等于零，则返回 true，否则返回 false

  `bool pm_runtime_status_suspended(struct device *dev);`
    - 如果设备的运行时 PM 状态为 'suspended'，则返回 true

  `void pm_runtime_no_callbacks(struct device *dev);`
    - 为设备设置 power.no_callbacks 标志，并从 /sys/devices/.../power 移除运行时 PM 属性（或在设备注册时阻止它们被添加）

  `void pm_runtime_irq_safe(struct device *dev);`
    - 为设备设置 power.irq_safe 标志，使得运行时 PM 回调在中断禁用的情况下被调用

  `bool pm_runtime_is_irq_safe(struct device *dev);`
    - 如果为设备设置了 power.irq_safe 标志（使运行时 PM 回调在中断禁用的情况下被调用），则返回 true

  `void pm_runtime_mark_last_busy(struct device *dev);`
    - 将 power.last_busy 字段设为当前时间

  `void pm_runtime_use_autosuspend(struct device *dev);`
    - 设置 power.use_autosuspend 标志，启用自动挂起延迟；如果该标志之前被清除且 power.autosuspend_delay 为负，则调用 pm_runtime_get_sync

  `void pm_runtime_dont_use_autosuspend(struct device *dev);`
    - 清除 power.use_autosuspend 标志，禁用自动挂起延迟；如果该标志之前被置位且 power.autosuspend_delay 为负，则递减设备的使用计数器；调用 pm_runtime_idle

  `void pm_runtime_set_autosuspend_delay(struct device *dev, int delay);`
    - 将 power.autosuspend_delay 的值设为 'delay'（以毫秒表示）；如果 'delay' 为负则阻止运行时挂起；如果 power.use_autosuspend 被置位，则根据 power.autosuspend_delay 是否被改为负值或改为离开负值，调用 pm_runtime_get_sync 或递减设备的使用计数器并调用 pm_runtime_idle；如果 power.use_autosuspend 被清除，则调用 pm_runtime_idle

  `unsigned long pm_runtime_autosuspend_expiration(struct device *dev);`
    - 基于 power.last_busy 和 power.autosuspend_delay 计算当前自动挂起延迟时段到期的时间；如果延迟时间为 1000 ms 或更大，则到期时间向上取整到最近的秒；如果延迟时段已经到期或 power.use_autosuspend 未被设置，则返回 0，否则以 jiffies 返回到期时间

可以在中断上下文中安全执行以下辅助函数：

- pm_request_idle()
- pm_request_autosuspend()
- pm_schedule_suspend()
- pm_request_resume()
- pm_runtime_get_noresume()
- pm_runtime_get()
- pm_runtime_put_noidle()
- pm_runtime_put()
- pm_runtime_put_autosuspend()
- __pm_runtime_put_autosuspend()
- pm_runtime_enable()
- pm_suspend_ignore_children()
- pm_runtime_set_active()
- pm_runtime_set_suspended()
- pm_runtime_suspended()
- pm_runtime_mark_last_busy()
- pm_runtime_autosuspend_expiration()

如果已经为设备调用了 pm_runtime_irq_safe()，则以下辅助函数也可以在中断上下文中使用：

- pm_runtime_idle()
- pm_runtime_suspend()
- pm_runtime_autosuspend()
- pm_runtime_resume()
- pm_runtime_get_sync()
- pm_runtime_put_sync()
- pm_runtime_put_sync_suspend()
- pm_runtime_put_sync_autosuspend()

## 5. 运行时 PM 初始化、设备探测与移除


最初，所有设备的运行时 PM 都是禁用的，这意味着在第 4 节描述的大多数运行时 PM 辅助函数在为设备调用 pm_runtime_enable() 之前都将返回 -EAGAIN。

除此之外，所有设备的初始运行时 PM 状态为 'suspended'，但这未必反映设备的实际物理状态。因此，如果设备初始是活跃的（即它能够处理 I/O），则在其调用 pm_runtime_enable() 之前，必须借助 pm_runtime_set_active() 将其运行时 PM 状态改为 'active'。

然而，如果设备有父设备且父设备的运行时 PM 是启用的，则为设备调用 pm_runtime_set_active() 会影响父设备，除非父设备的 'power.ignore_children' 标志被置位。即在那种情况下，只要子设备的状态是 'active'，即使子设备的运行时 PM 仍被禁用（即尚未为子设备调用 pm_runtime_enable() 或已为其调用 pm_runtime_disable()），父设备也无法在运行时挂起（使用 PM core 的辅助函数）。出于这个原因，一旦为设备调用了 pm_runtime_set_active()，就应尽快合理地为其调用 pm_runtime_enable()，或者借助 pm_runtime_set_suspended() 将其运行时 PM 状态改回 'suspended'。

如果设备的默认初始运行时 PM 状态（即 'suspended'）反映了设备的实际状态，其总线类型或驱动的 ->probe() 回调很可能需要使用第 4 节描述的 PM core 的某个辅助函数来唤醒它。在那种情况下，应使用 pm_runtime_resume()。当然，为此目的，设备的运行时 PM 必须更早通过调用 pm_runtime_enable() 来启用。

注意，如果设备可能在探测（probe）期间执行 pm_runtime 调用（例如如果它注册到一个可能会回调的子系统），那么成对使用 pm_runtime_get_sync() 与 pm_runtime_put() 调用是合适的，以确保设备在探测期间不会被放回睡眠。这可能发生在诸如网络设备层这样的系统中。

在 ->probe() 完成后挂起设备可能是可取的。因此驱动核心使用异步的 pm_request_idle() 来提交一个在该时刻执行设备子系统级空闲回调的请求。利用了运行时自动挂起特性的驱动可能想在从 ->probe() 返回之前更新最后的 busy 标记。

此外，驱动核心防止运行时 PM 回调与 __device_release_driver() 中的总线通知（notifier）回调竞争，这是必要的，因为某些子系统使用该通知来执行影响运行时 PM 功能的操作。它通过在 driver_sysfs_remove() 和 BUS_NOTIFY_UNBIND_DRIVER 通知之前调用 pm_runtime_get_sync() 来实现。这会在设备处于挂起状态时恢复它，并防止在这些例程执行期间它被再次挂起。

为了允许总线类型和驱动通过从它们的 ->remove() 例程调用 pm_runtime_suspend() 将设备置于挂起状态，驱动核心在 __device_release_driver() 中运行 BUS_NOTIFY_UNBIND_DRIVER 通知之后执行 pm_runtime_put_sync()。这要求总线类型和驱动让它们的 ->remove() 回调直接避免与运行时 PM 竞争，但它也允许在移除驱动期间更灵活地处理设备。

驱动在 ->remove() 回调中应撤销在 ->probe() 中做的运行时 PM 更改。通常这意味着调用 pm_runtime_disable()、pm_runtime_dont_use_autosuspend() 等。

用户空间可以通过将设备 /sys/devices/.../power/control 属性的值改为 "on" 来有效禁止设备的驱动在运行时对其进行电源管理，这会导致调用 pm_runtime_forbid()。原则上，驱动也可以利用此机制在用户空间打开它之前有效关闭设备的运行时电源管理。即，在初始化期间驱动可以确保设备的运行时 PM 状态为 'active' 并调用 pm_runtime_forbid()。不过应当注意，如果用户空间已经有意将 /sys/devices/.../power/control 的值改为 "auto" 以允许驱动在运行时对设备进行电源管理，驱动以这种方式使用 pm_runtime_forbid() 可能会让用户空间困惑。

## 6. 运行时 PM 与系统睡眠


运行时 PM 与系统睡眠（即系统挂起和休眠，也称为挂起到 RAM 和挂起到磁盘）以几种方式相互交互。如果系统睡眠开始时设备是活跃的，则一切都很直接。但如果设备已经被挂起，会发生什么呢？

设备对运行时 PM 和系统睡眠可能有不同的唤醒设置。例如，远程唤醒可能对运行时挂起启用，但对系统睡眠禁止（device_may_wakeup(dev) 返回 'false'）。当这种情况发生时，子系统级系统挂起回调负责改变设备的唤醒设置（它可以把这件事留给设备驱动的系统挂起例程）。为此可能有必要先恢复设备再将其挂起。如果驱动对运行时挂起和系统睡眠使用不同的电源级别或其他设置，情况也是如此。

在系统恢复期间，最简单的方法是把所有设备都恢复到全功率，即使它们在系统睡眠开始之前就已经被挂起。这样做有几个原因，包括：

  - 设备可能需要切换电源级别、唤醒设置等。

  - 固件可能丢失了远程唤醒事件。

  - 设备的子设备可能需要设备处于全功率才能恢复它们自己。

  - 驱动对设备状态的认知可能与设备的物理状态不一致。这在从休眠恢复期间可能发生。

  - 设备可能需要被重置。

  - 即使设备已被挂起，如果其使用计数器 > 0，那么很可能不久之后它也需要一次运行时恢复。

如果设备在系统睡眠开始之前已被挂起，并在恢复期间被恢复到全功率，那么它的运行时 PM 状态将必须更新以反映系统睡眠后的实际状态。做法是：

  - pm_runtime_disable(dev);
  - pm_runtime_set_active(dev);
  - pm_runtime_enable(dev);

PM core 总是在调用 ->suspend() 回调之前递增运行时使用计数器，并在调用 ->resume() 回调之后递减它。因此像这样临时禁用运行时 PM 不会导致任何运行时挂起尝试被永久丢失。如果使用计数在 ->resume() 回调返回后变为零，->runtime_idle() 回调将照常被调用。

然而，在某些系统上，系统睡眠不是通过全局固件或硬件操作进入的。相反，所有硬件组件都由内核以协调的方式直接置于低功耗状态。然后，系统睡眠状态实际上源于硬件组件最终所处的状态，并且系统从该状态被硬件中断或类似机制（完全处于内核控制之下）唤醒。结果，内核从不交出控制权，并且恢复期间所有设备的状态它都精确知晓。如果是这种情况，并且上面列出的情形都不发生（特别是，如果系统不是从休眠唤醒），那么把在系统睡眠开始之前已被挂起的设备留在挂起状态中可能更高效。

为此，PM core 提供了一种机制，允许设备层次结构的不同层级之间进行某种协调。即，如果系统挂起的 .prepare() 回调为某设备返回一个正数，这表示向 PM core 表明该设备看上去处于运行时挂起状态且其状态良好，因此只要它的所有后代也留在运行时挂起状态，就可以让它留在运行时挂起。如果发生这种情况，PM core 将不会为所有这些设备执行任何系统挂起和恢复回调，除了 .complete() 回调，它随后完全负责以适当的方式处理该设备。这仅适用于与休眠无关的系统挂起转换（更多信息参见 Documentation/driver-api/pm/devices.rst）。

PM core 通过执行以下操作，尽最大努力降低运行时 PM 与系统挂起/恢复（以及休眠）回调之间竞争条件的可能性：

  - 在系统挂起期间，正好在执行某设备的子系统级 .prepare() 回调之前，对其调用 pm_runtime_get_noresume()，并且正好在执行其子系统级 .suspend() 回调之前，对其调用 pm_runtime_barrier()。除此之外，PM core 正好在执行其子系统级 .suspend_late() 回调之前，为每台设备禁用运行时 PM。

  - 在系统恢复期间，正好在执行其子系统级 .resume_early() 回调之后，对其调用 pm_runtime_enable()，并正好在执行其子系统级 .complete() 回调之后，对其调用 pm_runtime_put()。

## 7. 通用子系统回调


子系统可能希望通过使用 PM core 提供的一组通用电源管理回调来节省代码空间，这些回调定义在 driver/base/power/generic_ops.c 中：

  `int pm_generic_runtime_suspend(struct device *dev);`
    - 调用此设备的驱动提供的 ->runtime_suspend() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_runtime_resume(struct device *dev);`
    - 调用此设备的驱动提供的 ->runtime_resume() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_suspend(struct device *dev);`
    - 如果设备尚未在运行时被挂起，调用其驱动提供的 ->suspend() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_suspend_noirq(struct device *dev);`
    - 如果 pm_runtime_suspended(dev) 返回 "false"，调用设备驱动提供的 ->suspend_noirq() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_resume(struct device *dev);`
    - 调用此设备的驱动提供的 ->resume() 回调，如果成功，则将设备的运行时 PM 状态改为 'active'

  `int pm_generic_resume_noirq(struct device *dev);`
    - 调用此设备的驱动提供的 ->resume_noirq() 回调

  `int pm_generic_freeze(struct device *dev);`
    - 如果设备尚未在运行时被挂起，调用其驱动提供的 ->freeze() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_freeze_noirq(struct device *dev);`
    - 如果 pm_runtime_suspended(dev) 返回 "false"，调用设备驱动提供的 ->freeze_noirq() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_thaw(struct device *dev);`
    - 如果设备尚未在运行时被挂起，调用其驱动提供的 ->thaw() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_thaw_noirq(struct device *dev);`
    - 如果 pm_runtime_suspended(dev) 返回 "false"，调用设备驱动提供的 ->thaw_noirq() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_poweroff(struct device *dev);`
    - 如果设备尚未在运行时被挂起，调用其驱动提供的 ->poweroff() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_poweroff_noirq(struct device *dev);`
    - 如果 pm_runtime_suspended(dev) 返回 "false"，运行设备驱动提供的 ->poweroff_noirq() 回调并返回其结果，如果未定义则返回 0

  `int pm_generic_restore(struct device *dev);`
    - 调用此设备的驱动提供的 ->restore() 回调，如果成功，则将设备的运行时 PM 状态改为 'active'

  `int pm_generic_restore_noirq(struct device *dev);`
    - 调用此设备的驱动提供的 ->restore_noirq() 回调

这些函数是 PM core 在未提供自己的回调时使用默认值，用于子系统级 dev_pm_ops 结构中的 ->runtime_idle()、->runtime_suspend()、->runtime_resume()、->suspend()、->suspend_noirq()、->resume()、->resume_noirq()、->freeze()、->freeze_noirq()、->thaw()、->thaw_noirq()、->poweroff()、->poweroff_noirq()、->restore()、->restore_noirq()。

希望使用同一个函数作为系统挂起、冻结、断电和运行时挂起回调，类似地用于系统恢复、解冻、恢复和运行时恢复的的设备驱动，可以借助 include/linux/pm_runtime.h 中定义的 DEFINE_RUNTIME_DEV_PM_OPS()（可能将其最后一个参数设为 NULL）来实现类似的行为。

## 8. "无回调"设备


某些"设备"只是其父设备的逻辑子设备，无法自行进行电源管理。（典型的例子是 USB 接口。整个 USB 设备可以进入低功耗模式或发送唤醒请求，但对于单个接口这两者都不可能。）这些设备的驱动不需要运行时 PM 回调；如果回调存在，->runtime_suspend() 和 ->runtime_resume() 将总是返回 0 而不做任何其他事情，->runtime_idle() 将总是调用 pm_runtime_suspend()。

子系统可以通过调用 pm_runtime_no_callbacks() 来告知 PM core 这些设备。这应该在设备结构被初始化之后、注册之前完成（尽管在设备注册之后做也可以）。该例程将设置设备的 power.no_callbacks 标志，并阻止创建非调试用的运行时 PM sysfs 属性。

当 power.no_callbacks 被置位时，PM core 将不会调用 ->runtime_idle()、->runtime_suspend() 或 ->runtime_resume() 回调。相反，它将假定挂起和恢复总是成功，并且空闲设备应当被挂起。

因此，PM core 将永远不会直接通知设备的子系统或驱动有关运行时电源的变化。相反，设备的父设备的驱动必须负责在父设备的电源状态改变时通知设备的驱动。

注意，在某些情况下，子系统/驱动可能不希望为其设备调用 pm_runtime_no_callbacks()。这可能是因为需要实现运行时 PM 回调的一个子集、一个平台相关的 PM 域可能附加到该设备，或者该设备是通过供应者设备链接（supplier device link）进行电源管理。出于这些原因，并为了避免子系统/驱动中的样板代码，PM core 允许运行时 PM 回调不被赋值。更准确地说，如果某个回调指针为 NULL，PM core 将表现得好像存在一个回调并且它返回了 0。

## 9. 自动挂起，或自动延迟的挂起


改变设备的电源状态并非没有代价；它需要时间和能量。只有当有理由认为设备将在该状态停留相当长一段时间时，才应将其置于低功耗状态。一种常见的启发式方法认为，一段时间内未被使用的设备很可能保持未使用；遵循这一建议，驱动不应允许设备在运行时被挂起，直到它们已经非活动了某个最短时间段。即使该启发式方法最终不是最优的，它仍然能防止设备在低功耗和全功率状态之间"弹跳"得太快。

术语"autosuspend"（自动挂起）是一个历史遗留物。它并不意味着设备被自动挂起（子系统或驱动仍然必须调用适当的 PM 例程）；而是意味着运行时挂起将自动延迟，直到期望的非活动时段已经过去。

非活动是根据 power.last_busy 字段确定的。期望的非活动时段长度是一个策略问题。子系统可以通过调用 pm_runtime_set_autosuspend_delay() 初始设置这个长度，但在设备注册之后，该长度应由用户空间使用 /sys/devices/.../power/autosuspend_delay_ms 属性来控制。

为了使用自动挂起，子系统或驱动必须调用 pm_runtime_use_autosuspend()（最好在注册设备之前），此后它们应当使用各种 `*_autosuspend()` 辅助函数

```
	Instead of: pm_runtime_suspend    use: pm_runtime_autosuspend;
	Instead of: pm_schedule_suspend   use: pm_request_autosuspend;
	Instead of: pm_runtime_put        use: pm_runtime_put_autosuspend;
	Instead of: pm_runtime_put_sync   use: pm_runtime_put_sync_autosuspend.

```
驱动也可以继续使用非自动挂起的辅助函数；它们的行为将正常，这意味着有时会考虑自动挂起延迟（见 pm_runtime_idle）。这些函数的自动挂起变体也会调用 pm_runtime_mark_last_busy()。

在某些情况下，驱动或子系统可能想阻止设备立即自动挂起，即使使用计数器为零且自动挂起延迟时间已经到期。如果 ->runtime_suspend() 回调返回 -EAGAIN 或 -EBUSY，并且下一次自动挂起延迟到期时间在未来（就像回调调用了 pm_runtime_mark_last_busy() 时通常那样），PM core 将自动重新调度自动挂起。->runtime_suspend() 回调自身不能做这个重新调度，因为在设备挂起期间（即回调运行时）任何类型的挂起请求都不会被接受。

该实现非常适合在中断上下文中异步使用。然而这种使用不可避免地涉及竞争，因为 PM core 无法将 ->runtime_suspend() 回调与 I/O 请求的到达同步。这种同步必须由驱动使用其私有锁来处理。

```
	foo_read_or_write(struct foo_priv *foo, void *data)
	{
		lock(&foo->private_lock);
		add_request_to_io_queue(foo, data);
		if (foo->num_pending_requests++ == 0)
			pm_runtime_get(&foo->dev);
		if (!foo->is_suspended)
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
	}

	foo_io_completion(struct foo_priv *foo, void *req)
	{
		lock(&foo->private_lock);
		if (--foo->num_pending_requests == 0)
			pm_runtime_put_autosuspend(&foo->dev);
		else
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
		/* Send req result back to the user ... */
	}

	int foo_runtime_suspend(struct device *dev)
	{
		struct foo_priv foo = container_of(dev, ...);
		int ret = 0;

		lock(&foo->private_lock);
		if (foo->num_pending_requests > 0) {
			ret = -EBUSY;
		} else {
			/* ... suspend the device ... */
			foo->is_suspended = 1;
		}
		unlock(&foo->private_lock);
		return ret;
	}

	int foo_runtime_resume(struct device *dev)
	{
		struct foo_priv foo = container_of(dev, ...);

		lock(&foo->private_lock);
		/* ... resume the device ... */
		foo->is_suspended = 0;
		pm_runtime_mark_last_busy(&foo->dev);
		if (foo->num_pending_requests > 0)
			foo_process_next_request(foo);
		unlock(&foo->private_lock);
		return 0;
	}

```
要点是，在 foo_io_completion() 请求自动挂起之后，foo_runtime_suspend() 回调可能与 foo_read_or_write() 竞争。因此 foo_runtime_suspend() 必须在允许挂起继续进行之前（持有私有锁时）检查是否有任何待处理的 I/O 请求。

此外，power.autosuspend_delay 字段可以随时被用户空间改变。如果驱动关心这一点，它可以在持有自身私有锁的情况下，从 ->runtime_suspend() 回调内调用 pm_runtime_autosuspend_expiration()。如果函数返回一个非零值，则延迟尚未到期，该回调应当返回 -EAGAIN。
