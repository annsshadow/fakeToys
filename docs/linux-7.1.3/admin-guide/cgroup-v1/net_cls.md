## 网络分类cgroup


网络分类器（Network classifier）cgroup 提供了一个接口，用于为网络数据包打上类别标识符（classid）
流量控制器（Traffic Controller，tc）可用于为来自不cgroup 的数据包分配不同的优先级。此外，Netfilter（iptables）也可以利用此标记对这类数据包执行操作
创建 net_cls cgroup 实例会创建一net_cls.classid 文件。该 net_cls.classid 值初始化0
你可以向 net_cls.classid 写入十六进制值；这些值的格式0xAAAABBBB，其AAAA 是主句柄号（major handle number），BBBB 是次句柄号（minor handle number）。读net_cls.classid 得到的是十进制结果
```

	mkdir /sys/fs/cgroup/net_cls
	mount -t cgroup -onet_cls net_cls /sys/fs/cgroup/net_cls
	mkdir /sys/fs/cgroup/net_cls/0
	echo 0x100001 >  /sys/fs/cgroup/net_cls/0/net_cls.classid

```
```

	cat /sys/fs/cgroup/net_cls/0/net_cls.classid
	1048577

```
```

	tc qdisc add dev eth0 root handle 10: htb
	tc class add dev eth0 parent 10: classid 10:1 htb rate 40mbit

```
```

	tc filter add dev eth0 parent 10: protocol ip prio 10 handle 1: cgroup

```
```

	iptables -A OUTPUT -m cgroup ! --cgroup 0x100001 -j DROP

```
