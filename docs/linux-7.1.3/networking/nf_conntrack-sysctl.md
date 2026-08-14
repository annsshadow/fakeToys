
## Netfilter 连接跟踪 Sysfs 变量


## /proc/sys/net/netfilter/nf_conntrack_* 变量：


nf_conntrack_acct - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	启用连接跟踪流记账。会为每个流添加 64 位字节与包计数器。

nf_conntrack_buckets - INTEGER
	哈希表的大小。如果在模块加载时未作为参数指定，默认大小通过将总内存除以 16384
	来确定桶的数量。哈希表永远不会少于 1024 个桶，也永远不会多于 262144 个桶。
	该 sysctl 仅在初始网络命名空间中可写。

nf_conntrack_checksum - BOOLEAN
 - 0 - 禁用
 - 非 0 - 启用（默认）

	校验入包校验和。校验和错误的包处于 INVALID 状态。如果启用此选项，此类包将不被
	考虑用于连接跟踪。

nf_conntrack_count - INTEGER（只读）
	当前已分配的流条目数量。

nf_conntrack_events - BOOLEAN
 - 0 - 禁用
 - 1 - 启用
 - 2 - 自动（默认）

	如果启用此选项，连接跟踪代码将通过 ctnetlink 向用户空间提供连接跟踪事件。默认
	情况下，如果有用户空间程序正在监听 ctnetlink 事件，则分配该扩展。

nf_conntrack_expect_max - INTEGER
	期望（expectation）表的最大大小。默认值为 nf_conntrack_buckets / 256。最小值为 1。

nf_conntrack_frag6_high_thresh - INTEGER
	default 262144

	用于重组 IPv6 分片的最大内存。当为上述目的分配了 nf_conntrack_frag6_high_thresh
	字节的内存时，分片处理程序将丢弃包，直到达到 nf_conntrack_frag6_low_thresh。

nf_conntrack_frag6_low_thresh - INTEGER
	default 196608

	参见 nf_conntrack_frag6_low_thresh

nf_conntrack_frag6_timeout - INTEGER（秒）
	default 60

	在内存中保留 IPv6 分片的时长。

nf_conntrack_generic_timeout - INTEGER（秒）
	default 600

	通用超时的默认值。这指的是第 4 层未知/不支持的协议。

nf_conntrack_icmp_timeout - INTEGER（秒）
	default 30

	ICMP 超时的默认值。

nf_conntrack_icmpv6_timeout - INTEGER（秒）
	default 30

	ICMP6 超时的默认值。

nf_conntrack_log_invalid - INTEGER
 - 0   - 禁用（默认）
 - 1   - 记录 ICMP 包
 - 6   - 记录 TCP 包
 - 17  - 记录 UDP 包
 - 41  - 记录 ICMPv6 包
 - 136 - 记录 UDPLITE 包
 - 255 - 记录任意协议的包

	记录由值指定的类型的无效包。

nf_conntrack_max - INTEGER
        允许的连跟踪条目的最大数量。默认情况下该值设为 nf_conntrack_buckets。注意，
        连接跟踪条目会被加入表中两次——一次用于原始方向，一次用于回复方向（即地址
        反转）。这意味着默认设置下，表满时的平均哈希链长度为 2，而不是 1。

nf_conntrack_tcp_be_liberal - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	严于律己，宽以待人（在你要做的事上保守，在从他人处接受的东西上宽松）。如果非零，
	我们只将窗口外的 RST 段标记为 INVALID。

nf_conntrack_tcp_ignore_invalid_rst - BOOLEAN
 - 0 - 禁用（默认）
 - 1 - 启用

	如果为 1，我们不将窗口外的 RST 段标记为 INVALID。

nf_conntrack_tcp_loose - BOOLEAN
 - 0 - 禁用
 - 非 0 - 启用（默认）

	如果设为 0，我们将禁用拾取（pick up）已建立的连接。

nf_conntrack_tcp_max_retrans - INTEGER
	default 3

	在未收到来自目的地的（可接受的）ACK 的情况下可以重传的最大包数。如果达到此数，
	将启动一个更短的定时器。

nf_conntrack_tcp_timeout_close - INTEGER（秒）
	default 10

nf_conntrack_tcp_timeout_close_wait - INTEGER（秒）
	default 60

nf_conntrack_tcp_timeout_established - INTEGER（秒）
	default 432000（5 天）

nf_conntrack_tcp_timeout_fin_wait - INTEGER（秒）
	default 120

nf_conntrack_tcp_timeout_last_ack - INTEGER（秒）
	default 30

nf_conntrack_tcp_timeout_max_retrans - INTEGER（秒）
	default 300

nf_conntrack_tcp_timeout_syn_recv - INTEGER（秒）
	default 60

nf_conntrack_tcp_timeout_syn_sent - INTEGER（秒）
	default 120

nf_conntrack_tcp_timeout_time_wait - INTEGER（秒）
	default 120

nf_conntrack_tcp_timeout_unacknowledged - INTEGER（秒）
	default 300

nf_conntrack_timestamp - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	启用连接跟踪流时间戳。

nf_conntrack_sctp_timeout_closed - INTEGER（秒）
	default 10

nf_conntrack_sctp_timeout_cookie_wait - INTEGER（秒）
	default 3

nf_conntrack_sctp_timeout_cookie_echoed - INTEGER（秒）
	default 3

nf_conntrack_sctp_timeout_established - INTEGER（秒）
	default 210

	默认值设为 (hb_interval * path_max_retrans + rto_max)

nf_conntrack_sctp_timeout_shutdown_sent - INTEGER（秒）
	default 3

nf_conntrack_sctp_timeout_shutdown_recd - INTEGER（秒）
	default 3

nf_conntrack_sctp_timeout_shutdown_ack_sent - INTEGER（秒）
	default 3

nf_conntrack_sctp_timeout_heartbeat_sent - INTEGER（秒）
	default 30

	该超时用于在辅助路径上建立连接跟踪条目。默认值设为 hb_interval。

nf_conntrack_udp_timeout - INTEGER（秒）
	default 30

nf_conntrack_udp_timeout_stream - INTEGER（秒）
	default 120

	在检测到 UDP 流的情况下将使用此扩展超时。

nf_conntrack_gre_timeout - INTEGER（秒）
	default 30

nf_conntrack_gre_timeout_stream - INTEGER（秒）
	default 180

	在检测到 GRE 流的情况下将使用此扩展超时。

nf_hooks_lwtunnel - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	如果启用此选项，轻量级隧道（lightweight tunnel）netfilter 钩子被启用。一旦启用，
	此选项无法被禁用。

nf_flowtable_tcp_timeout - INTEGER（秒）
        default 30

        控制 TCP 连接的卸载超时。TCP 连接可以从 nf conntrack 卸载到 nf flow table。
        一旦老化，连接将返回到 nf conntrack。

nf_flowtable_udp_timeout - INTEGER（秒）
        default 30

        控制 UDP 连接的卸载超时。UDP 连接可以从 nf conntrack 卸载到 nf flow table。
        一旦老化，连接将返回到 nf conntrack。
