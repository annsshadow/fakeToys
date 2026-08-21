## MPLS Sysfs 变量


## /proc/sys/net/mpls/* 变量


platform_labels - INTEGER
	平台标签表中的条目数量。无法为大于等于平台标签数量的标签
	配置转发

	平台标签表条目的密集使用是可能且预期的，因为平台标签是本
	分配的

	如果平台标签表条目数量设0，内核将不识别任何标签，
	mpls 转发将被禁用

	减小该值将移除所有不再适合该表的标签路由条目

	可选值：0 - 1048575

	默认值：0

ip_ttl_propagate - BOOL
	控制在压入标签时是否TTL IPv4/IPv6 头部传播
	MPLS 头部，以及在弹出最后一个标签时是否MPLS 头部传播
	IPv4/IPv6 头部

	如果禁用，MPLS 传输网络对过境流量将表现为单跳

 - 0 - 禁用 / RFC 3443 [Short] Pipe 模型
 - 1 - 启用 / RFC 3443 Uniform 模型（默认）

default_ttl - INTEGER
	用于 MPLS 数据包的默认 TTL 值，当无法从 IP 头部传播
	使用，原因可能是没有 IP 头部，或ip_ttl_propagate 已被禁用

	可选值：1 - 255

	默认值：255

conf/<interface>/input - BOOL
	控制是否可以在此接口上输入数据包

	如果禁用，数据包将被直接丢弃，不再进行后续处理

 - 0 - 禁用（默认）
 - 0 - 启用
