
## MPTCP Sysfs 变量


## /proc/sys/net/mptcp/* 变量


add_addr_timeout - INTEGER（秒）
	设置一个超时的最大值，超过之后，对于尚未确认先前
	ADD_ADDR 消息的 MPTCP 对端，将重新发送 ADD_ADDR 控制消息。
	如果此值低于最大值，则使用基于估计的连接往返时间
	动态估计的重传超时。

	如果设置为 0，则不重传。

	默认值与 TCP_RTO_MAX 匹配。这是一个按命名空间（per-namespace）
	的 sysctl。

	默认值：120

allow_join_initial_addr_port - BOOLEAN
	如果值为 1，则允许对端向初始子流（subflow）所使用的 IP 地址与
	端口号发送加入请求。这控制一个在连接时发送给对端的标志，以及
	此类加入请求是被接受还是被拒绝。

	对通过 ADD_ADDR 通告的地址的加入不受此值影响。

	这是一个按命名空间的 sysctl。

	默认值：1

available_path_managers - STRING
	显示已注册的可用路径管理器（path manager）选项。可能有更多
	路径管理器可用，但未被加载。

available_schedulers - STRING
	显示已注册的可用调度器（scheduler）选项。可能有更多数据包
	调度器可用，但未被加载。

blackhole_timeout - INTEGER（秒）
	当发生 MPTCP 防火墙黑洞问题时，禁用活跃 MPTCP 套接字上的 MPTCP
	的初始时间段（以秒计）。当 MPTCP 重新启用后紧接着检测到更多
	黑洞问题时，该时间段将呈指数增长，并在黑洞问题消失时重置为初始值。

	0 表示禁用黑洞检测。这是一个按命名空间的 sysctl。

	默认值：3600

checksum_enabled - BOOLEAN
	控制是否可启用 DSS 校验和。

	如果值非零，则可启用 DSS 校验和。这是一个
	按命名空间的 sysctl。

	默认值：0

close_timeout - INTEGER（秒）
	设置 make-after-break 超时：在没有出现任何 close 或
	shutdown 系统调用的情况下，MPTCP 套接字在移除最后一个子流之后、
	转到 TCP_CLOSE 之前，将在此时间段内保持状态不变。

	默认值与 TCP_TIMEWAIT_LEN 匹配。这是一个按命名空间的 sysctl。

	默认值：60

enabled - BOOLEAN
	控制是否可以创建 MPTCP 套接字。

	如果值为 1，则可以创建 MPTCP 套接字。这是一个
	按命名空间的 sysctl。

	默认值：1（启用）

path_manager - STRING
	设置用于每个新 MPTCP 套接字的默认路径管理器名称。内核态（in-kernel）
	路径管理将根据通过 MPTCP netlink API 配置的按命名空间值，
	控制子流连接与地址通告。用户态路径管理将每个 MPTCP 连接的子流
	连接决策与地址通告置于特权用户态程序的控制之下，代价是需要更多
	netlink 流量来传播所有相关事件与命令。

	这是一个按命名空间的 sysctl。

 - "kernel"          - 内核态路径管理器
 - "userspace"       - 用户态路径管理器

	默认值："kernel"

pm_type - INTEGER
	设置用于每个新 MPTCP 套接字的默认路径管理器类型。内核态路径管理
	将根据通过 MPTCP netlink API 配置的按命名空间值，控制子流
	连接与地址通告。用户态路径管理将每个 MPTCP 连接的子流
	连接决策与地址通告置于特权用户态程序的控制之下，代价是需要更多
	netlink 流量来传播所有相关事件与命令。

	这是一个按命名空间的 sysctl。

	自 v6.15 起已弃用，请改用 path_manager。

 - 0 - 内核态路径管理器
 - 1 - 用户态路径管理器

	默认值：0

scheduler - STRING
	选择你偏好的调度器。

	支持选择不同的调度器。这是一个按命名空间的 sysctl。

	默认值："default"

stale_loss_cnt - INTEGER
	判定某子流为陈旧（stale）所需的、在该子流上没有流量且存在
	待处理未完成数据的 MPTCP 层重传间隔次数。数据包调度器会忽略
	陈旧的子流。
	较低的 stale_loss_cnt 值允许快速的主备切换，较高的值可最大化
	边缘场景下的链路利用率，例如高误码率（BER）的有损链路，或
	对端暂停数据处理的情况。

	这是一个按命名空间的 sysctl。

	默认值：4

syn_retrans_before_tcp_fallback - INTEGER
	在回退到 TCP（即丢弃 MPTCP 选项）之前，SYN + MP_CAPABLE 重传的次数。
	换句话说，如果所有数据包都在途中被丢弃，将会出现：

 - 初始的带 MPTCP 支持的 SYN
 - 这个数量的带 MPTCP 支持的 SYN 重传
 - 后续的 SYN 重传将不带 MPTCP 支持

	0 表示第一次重传将在不带 MPTCP 选项的情况下进行。
	>= 128 表示所有 SYN 重传都将保留 MPTCP 选项。较低的数字可能会增加
	误报的 MPTCP 黑洞检测。这是一个按命名空间的 sysctl。

	默认值：2
