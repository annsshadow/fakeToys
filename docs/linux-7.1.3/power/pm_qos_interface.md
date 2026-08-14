## PM 服务质量（QoS）接口


该接口提供了一个内核模式和用户模式接口，用于驱动、子系统以及用户空间应用程序就
某个参数注册性能期望。

有两种不同的 PM QoS 框架可用：
 - CPU 延迟 QoS。
 - 每设备 PM QoS 框架提供了用于管理每设备延迟约束和 PM QoS 标志的 API。

PM QoS 框架中使用的延迟单位是微秒（usec）。


## 1. PM QoS 框架


一个 CPU 延迟 QoS 请求的全局列表与聚合（有效）目标值一起被维护。聚合目标值会随着
请求列表或其元素的变更而更新。对于 CPU 延迟 QoS，聚合目标值就是列表中各元素所持有
请求值的最小值。

注意：聚合目标值被实现为一个原子变量，因此读取聚合值不需要任何加锁机制。

在内核空间，该接口的使用很简单：

void cpu_latency_qos_add_request(handle, target_value):
  将一个元素以目标值插入到 CPU 延迟 QoS 列表中。
  一旦该列表发生变化，就会重新计算新目标，并且仅当目标值发生改变时才会调用任何已
  注册的 notifier。
  PM QoS 的客户端需要保存返回的处理句柄，以便在其他 PM QoS API 函数中后续使用。

void cpu_latency_qos_update_request(handle, new_target_value):
  会使用该新目标值更新由句柄指向的列表元素，并重新计算新的聚合目标，如果目标发生
  变化则调用通知树。

void cpu_latency_qos_remove_request(handle):
  会移除该元素。移除之后，如果移除该请求导致目标发生变化，它会更新聚合目标并调用
  通知树。

int cpu_latency_qos_limit():
  返回 CPU 延迟 QoS 的聚合值。

int cpu_latency_qos_request_active(handle):
  返回该请求是否仍然处于活动状态，即它是否尚未从 CPU 延迟 QoS 列表中移除。


从用户空间：

该基础设施暴露两个独立的设备节点，/dev/cpu_dma_latency 用于 CPU 延迟 QoS，
/dev/cpu_wakeup_latency 用于 CPU 系统唤醒延迟 QoS。

只有进程可以注册一个 PM QoS 请求。为了支持进程的自动清理，该接口要求进程按如下
方式注册其参数请求。

要注册 CPU 延迟 QoS 的默认 PM QoS 目标，进程必须打开 /dev/cpu_dma_latency。要注册
一个 CPU 系统唤醒 QoS 限制，进程必须打开 /dev/cpu_wakeup_latency。

只要该设备节点保持打开，该进程就在这个参数上拥有一个已注册的请求。

要更改所请求的目标值，进程需要向打开的设备节点写入一个 s32 值。或者，它可以使用
10 个字符长的格式（例如 "0x12345678"）写入该值的十六进制字符串。

要移除针对某个目标值的用户模式请求，只需关闭该设备节点。


## 2. 每设备 PM QoS 延迟与标志框架


对于每个设备，有三个 PM QoS 请求列表。其中两个与恢复延迟（resume latency）和
活动状态延迟容忍度（active state latency tolerance，单位为微秒）的聚合目标一起被
维护，第三个用于 PM QoS 标志。这些值会随着请求列表的变化而更新。

恢复延迟和活动状态延迟容忍度的目标值，就是参数列表元素所持有请求值的最小值。PM QoS
标志的聚合值是所有列表元素值的聚集（按位 OR）。目前定义了一个设备 PM QoS 标志：
PM_QOS_FLAG_NO_POWER_OFF。

注意：聚合目标值的实现方式使得读取聚合值不需要任何加锁机制。


在内核模式，该接口的使用如下：

int dev_pm_qos_add_request(device, handle, type, value):
  将以目标值把一个元素插入到所标识设备的列表中。一旦该列表发生变化，就会重新计算
  新目标，并且仅当目标值发生改变时才会调用任何已注册的 notifier。dev_pm_qos 的
  客户端需要保存该句柄，以便在其他 dev_pm_qos API 函数中后续使用。

int dev_pm_qos_update_request(handle, new_value):
  会使用该新目标值更新由句柄指向的列表元素，并重新计算新的聚合目标，如果目标发生
  变化则调用通知树。

int dev_pm_qos_remove_request(handle):
  会移除该元素。移除之后，如果移除该请求导致目标发生变化，它会更新聚合目标并调用
  通知树。

s32 dev_pm_qos_read_value(device, type):
  返回给定设备约束列表的聚合值。

enum pm_qos_flags_status dev_pm_qos_flags(device, mask)
  根据给定标志掩码检查给定设备的 PM QoS 标志。返回值的含义如下：

	PM_QOS_FLAGS_ALL:
		掩码中的所有标志都已设置
	PM_QOS_FLAGS_SOME:
		掩码中的某些标志已设置
	PM_QOS_FLAGS_NONE:
		掩码中的标志均未设置
	PM_QOS_FLAGS_UNDEFINED:
		该设备的 PM QoS 结构尚未初始化，或请求列表为空。

int dev_pm_qos_add_ancestor_request(dev, handle, type, value)
  为给定设备的第一个直接祖先添加一个 PM QoS 请求，该祖先的 power.ignore_children
  标志未设置（对于 DEV_PM_QOS_RESUME_LATENCY 请求），或其
  power.set_latency_tolerance 回调指针不为 NULL（对于
  DEV_PM_QOS_LATENCY_TOLERANCE 请求）。

int dev_pm_qos_expose_latency_limit(device, value)
  向设备的恢复延迟约束 PM QoS 列表添加一个请求，并在设备 power 目录下创建 sysfs
  属性 pm_qos_resume_latency_us，允许用户空间操作该请求。

void dev_pm_qos_hide_latency_limit(device)
  从设备的恢复延迟约束 PM QoS 列表中移除由 dev_pm_qos_expose_latency_limit() 添加的
  请求，并从设备 power 目录中移除 sysfs 属性 pm_qos_resume_latency_us。

int dev_pm_qos_expose_flags(device, value)
  向设备的标志 PM QoS 列表添加一个请求，并在设备 power 目录下创建 sysfs 属性
  pm_qos_no_power_off，允许用户空间更改 PM_QOS_FLAG_NO_POWER_OFF 标志的值。

void dev_pm_qos_hide_flags(device)
  从设备的标志 PM QoS 列表中移除由 dev_pm_qos_expose_flags() 添加的请求，并从设备
  power 目录中移除 sysfs 属性 pm_qos_no_power_off。

通知机制：

每设备 PM QoS 框架有一个每设备的通知树。

int dev_pm_qos_add_notifier(device, notifier, type):
  为设备添加一个针对特定请求类型的通知回调函数。

  当设备约束列表的聚合值发生变化时会调用该回调。

int dev_pm_qos_remove_notifier(device, notifier, type):
  移除设备的通知回调函数。


##### 活动状态延迟容忍度


该设备 PM QoS 类型用于支持那些硬件可以动态切换到节能运行模式的系统。在这类系统中，
如果硬件所选择的运行模式以过度激进的方式节省能耗，可能会使软件可见的延迟过大，导致
其错过某些协议要求或目标帧率、采样率等。

如果软件可以使用给定设备的某个延迟容忍度控制机制，则应当填充该设备 dev_pm_info
结构中的 .set_latency_tolerance 回调。它所指向的例程应当实现将有效需求值传递给硬件
所需的任何操作。

每当设备的有效延迟容忍度发生变化时，其 .set_latency_tolerance() 回调就会被执行，
并将有效值传递给它。如果该值为负数，意味着该设备的延迟容忍度需求列表为空，则期望
该回调在可行时将底层硬件延迟容忍度控制机制切换到自主（autonomous）模式。反之，如果
该值为 PM_QOS_LATENCY_ANY，并且硬件支持一种特殊的"无需求"设置，则期望该回调使用
它。这样软件可以防止硬件在响应电源状态变化（例如从 D3cold 转换到 D0 期间）时自动
更新设备的延迟容忍度，而这通常是在自主延迟容忍度控制模式下完成的。

如果设备存在 .set_latency_tolerance()，则设备的 power 目录中会出现 sysfs 属性
pm_qos_latency_tolerance_us。然后，用户空间可以使用该属性来指定其对设备的延迟
容忍度需求（如果有）。向其写入 "any" 表示"无需求，但不要让硬件控制延迟容忍度"，
向其写入 "auto" 则允许在没有来自内核侧的其他需求时，将硬件切换到自主模式。

内核代码可以使用上述函数，配合 DEV_PM_QOS_LATENCY_TOLERANCE 设备 PM QoS 类型，来
为设备添加、移除和更新延迟容忍度需求。
