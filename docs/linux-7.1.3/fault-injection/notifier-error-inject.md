## 通知器错误注入


通知器错误注入提供了向指定的通知器链回调中注入人为错误的能力。它对于测试通知器调用链失败（这种情况很少被执行）的错误处理非常有用。有一些内核模块可用于测试以下通知器。

 - PM 通知器
 - 内存热插拔通知器
 - powerpc pSeries reconfig 通知器
 - 网络设备（Netdevice）通知器

### PM 通知器错误注入模块

该特性通过 debugfs 接口控制

  /sys/kernel/debug/notifier-error-inject/pm/actions/<notifier event>/error

可能被置为失败的 PM 通知器事件有：

 - PM_HIBERNATION_PREPARE
 - PM_SUSPEND_PREPARE
 - PM_RESTORE_PREPARE

```

	# cd /sys/kernel/debug/notifier-error-inject/pm/
	# echo -12 > actions/PM_SUSPEND_PREPARE/error
	# echo mem > /sys/power/state
	bash: echo: write error: Cannot allocate memory

```
### 内存热插拔通知器错误注入模块

该特性通过 debugfs 接口控制

  /sys/kernel/debug/notifier-error-inject/memory/actions/<notifier event>/error

可能被置为失败的内存通知器事件有：

 - MEM_GOING_ONLINE
 - MEM_GOING_OFFLINE

```

	# cd /sys/kernel/debug/notifier-error-inject/memory
	# echo -12 > actions/MEM_GOING_OFFLINE/error
	# echo offline > /sys/devices/system/memory/memoryXXX/state
	bash: echo: write error: Cannot allocate memory

```
### powerpc pSeries reconfig 通知器错误注入模块

该特性通过 debugfs 接口控制

  /sys/kernel/debug/notifier-error-inject/pSeries-reconfig/actions/<notifier event>/error

可能被置为失败的 pSeries reconfig 通知器事件有：

 - PSERIES_RECONFIG_ADD
 - PSERIES_RECONFIG_REMOVE
 - PSERIES_DRCONF_MEM_ADD
 - PSERIES_DRCONF_MEM_REMOVE

### 网络设备通知器错误注入模块

该特性通过 debugfs 接口控制

  /sys/kernel/debug/notifier-error-inject/netdev/actions/<notifier event>/error

可被置为失败的网络设备通知器事件有：

 - NETDEV_REGISTER
 - NETDEV_CHANGEMTU
 - NETDEV_CHANGENAME
 - NETDEV_PRE_UP
 - NETDEV_PRE_TYPE_CHANGE
 - NETDEV_POST_INIT
 - NETDEV_PRECHANGEMTU
 - NETDEV_PRECHANGEUPPER
 - NETDEV_CHANGEUPPER

```

	# cd /sys/kernel/debug/notifier-error-inject/netdev
	# echo -22 > actions/NETDEV_CHANGEMTU/error
	# ip link set eth0 mtu 1024
	RTNETLINK answers: Invalid argument

```
### 更多使用示例

有一些 tools/testing/selftests 使用了通知器错误注入特性来测试 CPU 和内存通知器。

 - tools/testing/selftests/cpu-hotplug/cpu-on-off-test.sh
 - tools/testing/selftests/memory-hotplug/mem-on-off-test.sh

这些脚本首先进行简单的上线与下线测试，然后在通知器错误注入模块可用时进行故障注入测试。
