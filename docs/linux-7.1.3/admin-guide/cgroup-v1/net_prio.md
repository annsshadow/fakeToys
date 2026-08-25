## 缃戠粶浼樺厛绾?cgroup


网络优先cgroup 提供一个接口，允许管理员动态地设置由各种应用程序生成的网络流量的优先级
名义上，应用程序会通过 SO_PRIORITY 套接字选项设置其流量的优先级。然而，这并非总是可能，因为：

1) 应用程序可能没有被编写成设置该2) 应用程序流量的优先级通常是一个站点特定的管理决策，而非应用程序定义的决策
cgroup 允许管理员将进程分配到定义一个给定接口上出口流量优先级的组。网络优先级组可```

	# mount -t cgroup -onet_prio none /sys/fs/cgroup/net_prio

```
完成上述步骤后，作为父记账组的初始组'/sys/fs/cgroup/net_prio' 处可见。该组包含系统中
所有的任务/sys/fs/cgroup/net_prio/tasks' 列出了此 cgroup 中的任务
每个 net_prio cgroup 包含两个子系统特定的文件
net_prio.prioidx
  该文件只读，仅提供信息。它包含一个唯一的整数值，内核将其用作cgroup 的内部表示
net_prio.ifpriomap
  该文件包含一个优先级映射，分配给从该组中的进程发起、并从各种接口离开系统的流量。它包含
  形如 <ifname priority> 的元组列表。此文件的内容可以通过将同样的元组字符echo 到该文件
  鏉ヤ慨鏀?```

	echo "eth0 5" > /sys/fs/cgroups/net_prio/iscsi/net_prio.ifpriomap

```
该命令将强制任何属于 iscsi net_prio cgroup 的进程发起、并eth0 接口离开的流量，其优先级
被设为5。父记账组也有一个可写的 'net_prio.ifpriomap' 文件，可用于设置系统默认优先级
优先级在将帧排队到设备排队规则（qdisc）之前立即设置，因此优先级会在硬件队列选择做出之前
分配
net_prio cgroup 的一种用途是mqprio qdisc 配合，将应用程序流量导向基于硬件/驱动的流量类这些映射随后可由管理员或其他网络协议（如 DCBX）管理
新的 net_prio cgroup 继承父组的配置