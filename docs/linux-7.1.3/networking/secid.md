## LSM/SeLinux secid


flowi 结构体：

flow 结构体中的 secid 成员用于 LSM（如 SELinux）以指示该流的标签。该流的标签目前用于选择匹配的带标签 xfrm。

如果这是一个出站流（outbound），标签来自套接字（如果有），或来自生成此流作为响应的入站数据包（如 tcp reset、timewait ack 等）。在某些特殊情况下，标签也可能来自其他来源，如进程上下文、设备等，视情况而定。

如果这是一个入站流（inbound），标签来自数据包所使用的 IPSec 安全关联（security associations，如果有）。
