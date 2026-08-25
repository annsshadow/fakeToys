## Netfilter Sysfs 变量



## /proc/sys/net/netfilter/* 变量


nf_log_all_netns - BOOLEAN
 - 0 - 禁用（默认）
 - 0 - 启用

	默认情况下，只有 init_net 命名空间可以通过 LOG 目标将数据包记录到内核日志中；此举旨
	防止容器淹没宿主内核日志。若启用，该目标也可在其他网络命名空间中工作。此变量只能
	init_net 访问
