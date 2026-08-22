
## 挂起/休眠通知器（Notifiers


:Copyright: |copy| 2016 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


某些子系统或驱动可能希望在休挂起之前或恢唤醒之后执行一些操作，但它们要求系统完全可用，因此驱动的与子系统的 `->suspend()` `->resume()` 甚至 `->prepare()` `->complete()` 回调都不适合此目的

例如，设备驱动可能希望在唤醒/恢复之后向它们的设备上传固件，但它们无法`->resume()` `->complete()` 回调例程中调`request_firmware()`（此时用户态进程已被冻结）。解决方案可能是在进程被冻结之前将固件加载到内存中，并在 `->resume()` 例程中从那里上传。为此可以使用挂休眠通知器

有此类需求的子系统或驱动可以注册挂起通知器，它们将在以下事件时被 PM 核心调用

`PM_HIBERNATION_PREPARE`
	系统将要休眠，任务将立即被冻结。这与下面的 `PM_SUSPEND_PREPARE` 不同，因为在这种情况下，通知器与针对“冻结”转换的 PM 回调调用之间会完成额外的工作

`PM_POST_HIBERNATION`
	系统内存状态已从休眠镜像恢复，或在休眠期间发生了错误。设备恢复回调已执行，任务已解冻

`PM_RESTORE_PREPARE`
	系统将要恢复一个休眠镜像。如果一切顺利，恢复后的镜像内核将发`PM_POST_HIBERNATION` 通知

`PM_POST_RESTORE`
	从休眠恢复期间发生了错误。设备恢复回调已执行，任务已解冻

`PM_SUSPEND_PREPARE`
	系统正在准备挂起

`PM_POST_SUSPEND`
	系统刚刚唤醒，或在挂起期间发生了错误。设备唤醒回调已执行，任务已解冻

通常假定，通知器为 `PM_HIBERNATION_PREPARE` 所做的任何事情，都应在 `PM_POST_HIBERNATION` 中撤销。类似地，为 `PM_SUSPEND_PREPARE` 执行的操作应`PM_POST_SUSPEND` 中反向执行

此外，如果某个通知器在 `PM_HIBERNATION_PREPARE` `PM_SUSPEND_PREPARE` 事件上失败，那么已经为该事件成功过的通知器将分别被以 `PM_POST_HIBERNATION` `PM_POST_SUSPEND` 调用

休眠与挂起通知器在持有 :c:`pm_mutex` 的情况下被调用。它们以通常的方式定义，但它们的最后一个参数无意义（始终为 NULL）

要注册和/或注销挂起通知器，分别使用 `register_pm_notifier()` `unregister_pm_notifier()`（二者都定义`include/linux/suspend.h` 中）。如果你不需要注销通知器，也可以使`include/linux/suspend.h` 中定义的 `pm_notifier()` 宏
