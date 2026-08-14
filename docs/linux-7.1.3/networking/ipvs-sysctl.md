
## IPVS sysctl

## /proc/sys/net/ipv4/vs/* 变量：

am_droprate - INTEGER
	default 10

	它设置了“always 模式”的丢弃速率，该速率用于 drop_rate 防御的模式 3 中。

amemthresh - INTEGER
	default 1024

	它设置可用内存阈值（以页为单位），用于防御的自动模式。当没有足够的可用内存时，相应的策略将被启用，并且该变量会被自动设置为 2；否则该策略被禁用，该变量被设置为 1。

backup_only - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	如果设置，则当服务器处于备份模式时禁用 director 功能，以避免 DR/TUN 方法的包环路。

conn_lfactor - INTEGER
	可选值：-8（更大的表）.. 8（更小的表）

	默认：-4

	根据负载因子（每个表桶的连接数）控制连接哈希表的规模：

		2^conn_lfactor = nodes / buckets

	结果，当负载上升时表会增长，当负载下降时表会收缩，范围为 2^8 - 2^conn_tab_bits（模块参数）。
	该值是一个移位计数，负值选择 buckets = (connection hash nodes << -value)，而正值选择
	buckets = (connection hash nodes >> value)。负值会减少冲突并缩短查找时间，但会增加表的大小。
	正值在使用较小表时允许超过 100% 的负载，代价是更多冲突。如果使用 NAT 连接，请考虑将该值
	减 1，因为它们会在哈希表中增加两个节点。

	示例：
	-4：当负载超过 6% 时增长（buckets = nodes * 16）
	2：当负载超过 400% 时增长（buckets = nodes / 4）

conn_reuse_mode - INTEGER
	1 - 默认

	控制 ipvs 如何处理被检测到端口重用的连接。它是一个位图，取值为：

	0：禁用端口重用的任何特殊处理。新连接将被投递到为前一个连接提供服务的同一个真实服务器。

	bit 1：在安全的时机启用新连接的重新调度。即，任何时候 expire_nodest_conn，以及对于 TCP 套接字，
	当连接处于 TIME_WAIT 状态（仅在使用 NAT 模式时才可能）时。

	bit 2：即为 bit 1 加上：对于 TCP 连接，当连接处于 FIN_WAIT 状态时，因为这是负载均衡器在
	直接路由（Direct Routing）模式下看到的最后状态。此位有助于向一个非常繁忙的集群添加新真实服务器。

conntrack - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	如果设置，则为 IPVS 处理的连接维护连接跟踪条目。

	如果要让 IPVS 处理的连接也接受有状态防火墙规则的处理，则应启用此选项。即，使用连接跟踪的
	iptables 规则。否则禁用此设置是一项性能优化。

	IPVS FTP 应用模块处理的连接将拥有连接跟踪条目，不受此设置影响。

	仅当 IPVS 编译时启用了 CONFIG_IP_VS_NFCT 时可用。

cache_bypass - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	如果启用，当没有可用的缓存服务器且目的地址不是本地（iph->daddr 为 RTN_UNICAST）时，直接将包
	转发到原始目的地。它主要用于透明 Web 缓存集群。

debug_level - INTEGER
 - 0          - 传输错误消息（默认）
 - 1          - 非致命错误消息
 - 2          - 配置
 - 3          - 目的垃圾回收
 - 4          - 丢弃条目
 - 5          - 服务查找
 - 6          - 调度
 - 7          - 连接新建/过期、查找和同步
 - 8          - 状态转换
 - 9          - 绑定目的、模板检查和应用
 - 10         - IPVS 包传输
 - 11         - IPVS 包处理（ip_vs_in/ip_vs_out）
 - 12 或更高 - 包遍历

	仅当 IPVS 编译时启用了 CONFIG_IP_VS_DEBUG 时可用。

	更高的调试级别会包含较低调试级别的消息，因此设置调试级别 2 会包含级别 0、1 和 2 的消息。
	因此，级别越高，日志越冗长。

drop_entry - INTEGER
 - 0  - 禁用（默认）

	drop_entry 防御是随机丢弃连接哈希表中的条目，以便为新连接回收一些内存。在当前代码中，
	drop_entry 过程可以每秒激活一次，然后它随机扫描整个表的 1/32 并丢弃处于 SYN-RECV/SYNACK
	状态的条目，这应当能有效抵御 syn-flooding 攻击。

	drop_entry 的有效值为 0 到 3，其中 0 表示该策略始终禁用，1 和 2 表示自动模式（当没有足够的
	可用内存时，策略被启用且变量被自动设置为 2，否则策略被禁用且变量被设置为 1），3 表示策略始终启用。

drop_packet - INTEGER
 - 0  - 禁用（默认）

	drop_packet 防御旨在在将包转发到真实服务器之前丢弃 1/rate 的包。如果 rate 为 1，则丢弃所有入包。

	该取值定义与 drop_entry 相同。在自动模式下，速率由以下公式确定：
	rate = amemthresh / (amemthresh - available_memory)，当可用内存小于可用内存阈值时。
	当设置为模式 3 时，always 模式的丢弃速率由 /proc/sys/net/ipv4/vs/am_droprate 控制。

est_cpulist - CPULIST
	estimation kthreads 所允许的 CPU

	语法：标准 cpulist 格式
	空列表 - 停止 kthread 任务与估算
	默认 - 系统为 kthreads 保留的 housekeeping CPU

	示例：
	"all"：所有可能的 CPU
	"0-N"：所有可能的 CPU，N 表示最后一个 CPU 编号
	"0,1-N:1/2"：第一个以及所有编号为奇数的 CPU
	""：空列表

est_nice - INTEGER
	default 0
	有效范围：-20（更优先）.. 19（较不优先）

	用于 estimation kthreads 的 nice 值（调度优先级）

expire_nodest_conn - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	默认值为 0，当目的服务器不可用时，负载均衡器会静默地丢弃包。这在用户空间监控程序因服务器过载或
	错误检测而删除目的服务器、稍后又重新添加该服务器，且到该服务器的连接可以继续时，可能会有用。

	如果启用此特性，当包到达且目的服务器不可用时，负载均衡器会立即令该连接过期，然后客户端程序会
	收到连接已关闭的通知。这等同于某些人要求的在目的不可用时刷新连接的功能。

expire_quiescent_template - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	当设置为非零值时，当目的服务器处于静默（quiescent）状态时，负载均衡器会令持久化模板过期。
	当用户通过将目的服务器的权重设为 0 来使其进入静默状态，且希望后续原本持久的其它连接被发送到
	不同的目的服务器时，这可能很有用。默认情况下，新的持久连接被允许发往静默的目的服务器。

	如果启用此特性，当要使用该持久化模板来调度一个新连接，且目的服务器处于静默状态时，负载均衡器
	会令该持久化模板过期。

ignore_tunneled - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	如果设置，ipvs 会为所有未识别协议类型的包设置 ipvs_property。这防止我们对 ipip 等隧道协议进行
	路由，从而有助于防止对已被隧道转发到 ipvs 主机（即防止 ipvs 在同时作为真实服务器时产生 ipvs
	路由环路）的包进行重新调度。

nat_icmp_send - BOOLEAN
 - 0 - 禁用（默认）
 - 非 0 - 启用

	它控制在 VS/NAT 中，当负载均衡器收到来自真实服务器但连接条目不存在的包时，是否发送 icmp 错误消息
	（ICMP_DEST_UNREACH）。

pmtu_disc - BOOLEAN
 - 0 - 禁用
 - 非 0 - 启用（默认）

	默认情况下，对所有超出 PMTU 的 DF 包以 FRAG_NEEDED 拒绝，不论采用何种转发方法。对于 TUN 方法，
	可以禁用该标志以对这些包进行分片。

secure_tcp - INTEGER
 - 0  - 禁用（默认）

	secure_tcp 防御使用一个更复杂的 TCP 状态转换表。对于 VS/NAT，它还会延迟进入 TCP ESTABLISHED 状态，
	直到三次握手完成。

	该取值定义与 drop_entry 和 drop_packet 相同。

svc_lfactor - INTEGER
	可选值：-8（更大的表）.. 8（更小的表）

	默认：-3

	根据负载因子（每个表桶的服务数）控制服务哈希表的规模。表的增长和收缩范围为 2^4 - 2^20。
	详见 conn_lfactor 的说明。

sync_threshold - 由两个 INTEGER 组成的向量：sync_threshold, sync_period
	default 3 50

	它设置同步阈值，即一个连接在被同步之前需要收到的最小入包数。每当其入包数模 sync_period 等于
	阈值时，该连接会被同步一次。阈值的范围为 0 到 sync_period。

	当 sync_period 和 sync_refresh_period 为 0 时，仅在状态变化或当 pkts 匹配 sync_threshold 时同步一次。

sync_refresh_period - UNSIGNED INTEGER
	default 0

	以秒为单位，触发新同步消息的所报告连接定时器的差值。如果连接状态自上次同步以来未发生变化，它可
	用于在指定时间段（或连接超时的半数，如果更小）内避免同步消息。

	这对于高流量的正常连接很有用，可降低同步速率。此外，会以 sync_refresh_period/8 的周期重试
	sync_retries 次。

sync_retries - INTEGER
	default 0

	定义以 sync_refresh_period/8 为周期的同步重试次数。用于防止同步消息丢失。sync_retries 的范围为 0 到 3。

sync_qlen_max - UNSIGNED LONG

	未发送队列中同步消息的硬上限。它默认为内存页的 1/32，但实际上表示消息的数量。当发送速率低于
	排队速率时，它可以防止我们分配大块内存。

sync_sock_size - INTEGER
	default 0

	SNDBUF（master）或 RCVBUF（slave）套接字限制的配置。默认值为 0（保留系统默认值）。

sync_ports - INTEGER
	default 1

	master 和 backup 服务器可用于同步流量的线程数。每个线程将使用单个 UDP 端口，线程 0 将使用默认端口
	8848，而最后一个线程将使用端口 8848+sync_ports-1。

snat_reroute - BOOLEAN
 - 0 - 禁用
 - 非 0 - 启用（默认）

	如果启用，则从 realserver 重新计算 SNATed 包的路由，使它们如同源自 director 一样被路由。否则它们
	如同由 director 转发一样被路由。

	如果策略路由（policy routing）生效，则源自 director 的包可能会被路由到与由 director 转发的包
	不同的路径。

	如果策略路由未生效，则重新计算的路由将始终与原始路由相同，因此禁用 snat_reroute 并避免重新计算
	是一项优化。

sync_persist_mode - INTEGER
	default 0

	控制使用持久化时连接的同步

	0：所有类型的连接都会被同步

	1：尝试根据连接类型减少同步流量。对于持久化服务，避免对正常连接进行同步，仅对持久化模板进行同步。
	在这种情况下，对于 TCP 和 SCTP，可能需要在 backup 服务器上启用 sloppy_tcp 和 sloppy_sctp 标志。
	对于非持久化服务，不应用此类优化，假定为模式 0。

sync_version - INTEGER
	default 1

	发送同步消息时所使用的同步协议版本。

	0 选择原始同步协议（版本 0）。当向仅理解原始同步协议的旧系统发送同步消息时应使用此选项。

	1 选择当前同步协议（版本 1）。应尽可能使用此选项。

	带有此 sync_version 条目的内核能够接收同步协议的版本 1 和版本 2 的消息。

run_estimation - BOOLEAN
	0 - 禁用
	非 0 - 启用（默认）

	如果禁用，估算将被挂起且 kthread 任务停止。

	你随时可以通过将值设为 1 重新启用估算。但要小心，重新启用后的第一次估算并不准确。
