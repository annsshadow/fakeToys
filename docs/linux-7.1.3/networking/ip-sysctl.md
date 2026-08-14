
## IP Sysctl


## /proc/sys/net/ipv4/* 变量


ip_forward - BOOLEAN
	在接口之间转发数据包。

	该变量比较特殊，对其修改会将所有配置参数重置为默认状态
	（主机遵循 RFC1122，路由器遵循 RFC1812）。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

ip_default_ttl - INTEGER
	外出（非转发）IP 数据包中 TTL 字段（Time To Live，生存时间）的默认值。
	应在 1 到 255 之间（含边界）。
	默认值：64（依据 RFC1700 的建议）

ip_no_pmtu_disc - INTEGER
	禁用路径 MTU 发现（Path MTU Discovery）。如果在模式 1 下启用且收到
	"需要分片"类型的 ICMP 消息，则到该目的地的 PMTU 将被设为到该目的地的
	旧 MTU 与 min_pmtu（见下文）两者中的较小值。若希望避免本地产生分片，
	你需要手动将 min_pmtu 提升到系统上最小接口的 MTU。

	在模式 2 下，收到的路径 MTU 发现消息将被丢弃。外出帧的处理与模式 1
	相同，即在每个创建的套接字上隐式设置 IP_PMTUDISC_DONT。

	模式 3 是一种加固的 pmtu 发现模式。除了普通的套接字查找外，只有当
	底层协议能够验证时，内核才会接受"需要分片"错误。目前会处理 pmtu
	事件的协议为 TCP 和 SCTP，因为它们会验证例如序列号或关联。该模式不应
	全局启用，而仅用于保护例如命名空间中的名称服务器，其中 TCP 路径 mtu
	仍须工作，但其他协议的路径 MTU 信息应被丢弃。若全局启用，该模式可能
	破坏其他协议。

	可选值：0-3

	默认值：FALSE（否）

min_pmtu - INTEGER
	默认值 552 —— 最小路径 MTU。除非手动修改，否则每个缓存的 pmtu 都不会
	低于此设置。

ip_forward_use_pmtu - BOOLEAN
	默认情况下，我们在转发时不信任协议路径 MTU，因为它们很容易被伪造，
	并可能导致路由器产生非预期的分片。
	仅当你有用户态软件试图自行发现路径 mtu，并依赖内核遵循该信息时，才
	需要启用此项。通常并非如此。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

fwmark_reflect - BOOLEAN
	控制内核生成的、未关联任何套接字的 IPv4 应答数据包（例如 TCP RST 或
	ICMP 回显应答）的 fwmark。
	若禁用，这些数据包的 fwmark 为零。若启用，则它们具有所应答数据包的
	fwmark。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

fib_multipath_use_neigh - BOOLEAN
	在确定多路径路由的下一跳时，使用已有邻居表项的状态。若禁用，则不
	使用邻居信息，数据包可能被导向失效的下一跳。仅对启用了
	CONFIG_IP_ROUTE_MULTIPATH 的内核有效。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

fib_multipath_hash_policy - INTEGER
	控制多路径路由使用哪种哈希策略。仅对启用了
	CONFIG_IP_ROUTE_MULTIPATH 的内核有效。

	默认值：0（第 3 层）

	可选值：

 - 0 - 第 3 层
 - 1 - 第 4 层
 - 2 - 第 3 层，若存在内部第 3 层则使用内部第 3 层
 - 3 - 自定义多路径哈希。用于多路径哈希计算的字段由
	 fib_multipath_hash_fields sysctl 决定

fib_multipath_hash_fields - UNSIGNED INTEGER
	当 fib_multipath_hash_policy 设置为 3（自定义多路径哈希）时，用于
	多路径哈希计算的字段由此 sysctl 决定。

	该值是一个位掩码，用于启用多路径哈希计算所需的各个字段。

	可选字段如下：

	====== ============================
	0x0001 源 IP 地址
	0x0002 目的 IP 地址
	0x0004 IP 协议
	0x0008 未使用（流标签）
	0x0010 源端口
	0x0020 目的端口
	0x0040 内部源 IP 地址
	0x0080 内部目的 IP 地址
	0x0100 内部 IP 协议
	0x0200 内部流标签
	0x0400 内部源端口
	0x0800 内部目的端口
	====== ============================

	默认值：0x0007（源 IP、目的 IP 与 IP 协议）

fib_multipath_hash_seed - UNSIGNED INTEGER
	计算多路径路由哈希时使用的种子值。同时适用于 IPv4 和 IPv6 数据路径。
	仅对启用了 CONFIG_IP_ROUTE_MULTIPATH 的内核存在。

	设为 0 时，多路径路由使用的种子值默认为内核内部随机生成的一个值。

	实际的哈希算法未作规定 —— 不能保证由给定种子产生的下一跳分布在
	不同内核版本间保持稳定。

	默认值：0（随机）

fib_sync_mem - UNSIGNED INTEGER
	在强制 synchronize_rcu 之前，fib 表项可积压的脏内存量。

	默认值：512kB   最小值：64kB   最大值：64MB

ip_forward_update_priority - INTEGER
	在 IPv4 数据包被转发后，是否根据 IPv4 头部中的 "TOS" 字段更新 SKB
	优先级。新的 SKB 优先级依据 rt_tos2priority 表（例如参见 man tc-prio）
	由 TOS 字段值映射而来。

	默认值：1（更新优先级。）

	可选值：

 - 0 - 不更新优先级。
 - 1 - 更新优先级。

route/max_size - INTEGER
	内核允许的最大路由数量。在使用大量接口和/或路由时增大此值。

	从 linux 内核 3.6 开始，该值对 ipv4 已废弃，因为不再使用路由缓存。

	从 linux 内核 6.3 开始，该值对 ipv6 已废弃，因为垃圾回收会管理
	缓存的路由表项。

neigh/default/gc_thresh1 - INTEGER
	要保持的最小表项数。当表项数少于该值时，垃圾回收器不会清除表项。

	默认值：128

neigh/default/gc_thresh2 - INTEGER
	垃圾回收器开始更积极地清除表项的阈值。当超过该值时，早于 5 秒的
	表项将被清除。

	默认值：512

neigh/default/gc_thresh3 - INTEGER
	允许的最大非 PERMANENT 邻居表项数。在使用大量接口以及与大量直连
	对等体通信时增大此值。

	默认值：1024

neigh/default/gc_interval - INTEGER
	指定邻居表项的垃圾回收器应运行的频率。该值作用于整个表，而非
	单个表项。自内核 v2.6.8 起已不再使用。

	默认值：30 秒

neigh/default/gc_stale_time - INTEGER
	决定邻居表项在被视为陈旧、有资格被垃圾回收之前可以保持未使用的
	时长。超过此时长未被使用的表项将被垃圾回收器移除，除非它们具有
	活动引用、被标记为 PERMANENT，或带有 NTF_EXT_LEARNED 或
	NTF_EXT_VALIDATED 标志。陈旧的表项仅当表中至少有 gc_thresh1 个
	邻居时，才会被周期性 GC 移除。

	默认值：60 秒

neigh/default/unres_qlen_bytes - INTEGER
	其他网络层为每个未解析地址排队的数据包可以使用的最大字节数。
	（于 linux 3.3 加入）

	设置负值无意义并返回错误。

	默认值：SK_WMEM_DEFAULT，（与 net.core.wmem_default 相同）。

		具体取值取决于体系结构与内核选项，但应足以允许排队 256 个
		中等大小的数据包。

neigh/default/unres_qlen - INTEGER
	其他网络层为每个未解析地址可以排队的最大数据包数。

	（于 linux 3.3 废弃）：请改用 unres_qlen_bytes。

	在 linux 3.3 之前，默认值为 3，这可能导致非预期的数据包丢失。当前
	默认值依据 unres_qlen_bytes 的默认值以及数据包的真实大小计算。

	默认值：101

neigh/default/interval_probe_time_ms - INTEGER
	带有 NTF_MANAGED 标志的邻居表项的探测间隔，最小值为 1。

	默认值：5000

mtu_expires - INTEGER
	缓存的 PMTU 信息保留的秒数。

min_adv_mss - INTEGER
	通告的 MSS 取决于第一跳路由 MTU，但永远不会低于此设置。

fib_notify_on_flag_change - INTEGER
        是否在任何时候 RTM_F_OFFLOAD/RTM_F_TRAP/RTM_F_OFFLOAD_FAILED
        标志发生变化时发出 RTM_NEWROUTE 通知。

        将路由安装到内核后，用户态会收到一个确认，这意味着路由已安装到
        内核中，但不一定安装到了硬件中。
        已经安装到硬件中的路由也有可能改变其动作，从而改变其标志。例如，
        一个正在捕获数据包的主机路由，在安装了 IPinIP/VXLAN 隧道之后，
        可能被"提升"以执行解封装。
        这些通知将向用户态指示路由的状态。

        默认值：0（不发出通知。）

        可选值：

        - 0 - 不发出通知。
        - 1 - 发出通知。
        - 2 - 仅当 RTM_F_OFFLOAD_FAILED 标志变化时才发出通知。

IP Fragmentation:

ipfrag_high_thresh - LONG INTEGER
	用于重组 IP 分片的最大内存量。

ipfrag_low_thresh - LONG INTEGER
	（自 linux-4.17 起废弃）
	在尚未重组完成的碎片队列过多之前，用于重组 IP 分片的最大内存量。
	内核仍会接收新的分片进行重组。

ipfrag_time - INTEGER
	IP 分片在内存中保留的秒数。

ipfrag_max_dist - INTEGER
	ipfrag_max_dist 是一个非负整数值，定义了共享同一 IP 源地址的
	分片之间允许的最大"失序"程度。注意数据包的重排序并不罕见，但
	如果来自某个源 IP 地址的大量分片到达，而某个特定的分片队列仍然
	不完整，这通常表明该队列中一个或多个分片已丢失。当 ipfrag_max_dist
	为正值时，在将分片加入重组队列之前会做一次额外检查 —— 如果对使用
	该源地址的任何 IP 分片队列进行添加之间，已有 ipfrag_max_dist（或更
	多）个分片从该特定 IP 地址到达，则假定该队列中的一个或多个分片已
	丢失。现有的分片队列将被丢弃，并启动一个新的队列。ipfrag_max_dist
	值为零时将禁用此检查。

	使用非常小的值（例如 1 或 2）作为 ipfrag_max_dist，可能在发生
	正常的包重排序时导致不必要地丢弃分片队列，进而造成应用程序性能
	下降。使用非常大的值（例如 50000）则会增加错误地重组来自不同 IP
	数据报的 IP 分片的可能性，从而可能导致数据损坏。
	默认值：64

bc_forwarding - INTEGER
	bc_forwarding 启用 rfc1812#section-5.3.5.2 与 rfc2644 中描述的特性。
	它允许路由器转发定向广播。
	要启用此特性，'all' 项与输入接口项都应设为 1。
	默认值：0

## INET peer storage


inet_peer_threshold - INTEGER
	存储的大致大小。从该阈值开始，表项将被激进地丢弃。该阈值还决定了
	表项的时间-生存值以及垃圾回收之间的时间间隔。表项越多，生存时间越短，
	GC 间隔越小。

inet_peer_minttl - INTEGER
	表项的最小时间-生存值。应足以覆盖重组侧的分片时间-生存值。当池大小
	小于 inet_peer_threshold 时，保证此最小时间-生存值。
	以秒为单位。

inet_peer_maxttl - INTEGER
	表项的最大时间-生存值。当池上没有内存压力时（即池中表项数量很少），
	未使用的表项将在经过该时间段后过期。
	以秒为单位。

## TCP variables


somaxconn - INTEGER
	套接字 listen() 积压队列的限制，在用户态称为 SOMAXCONN。
	默认为 4096。（在 linux-5.4 之前为 128）
	另请参阅 tcp_max_syn_backlog 以对 TCP 套接字做进一步调优。

tcp_abort_on_overflow - BOOLEAN
	如果监听服务处理新连接过慢，则重置它们。默认状态为 FALSE（否）。
	这意味着如果因突发导致溢出，连接将恢复。仅当你确实确信监听守护
	进程无法被调优为更快地接受连接时，才启用此选项。启用此选项可能
	伤害你服务器的客户端。

tcp_adv_win_scale - INTEGER
	自 linux-6.6 起废弃
	以 bytes/2^tcp_adv_win_scale 计算缓冲开销（若 tcp_adv_win_scale > 0），
	或 bytes-bytes/2^(-tcp_adv_win_scale)（若其 <= 0）。

	可选值范围为 [-31, 31]（含边界）。

	默认值：1

tcp_allowed_congestion_control - STRING
	显示/设置非特权进程可用的拥塞控制选项。该列表是
	tcp_available_congestion_control 中所列选项的一个子集。

	默认值为 "reno" 及默认设置（tcp_congestion_control）。

tcp_app_win - INTEGER
	为应用程序缓冲区保留 max(window/2^tcp_app_win, mss) 的窗口。值 0
	是特殊的，表示不保留任何空间。

	可选值范围为 [0, 31]（含边界）。

	默认值：31

tcp_autocorking - BOOLEAN
	启用 TCP 自动塞入（auto corking）：
	当应用程序连续进行小的 write()/sendmsg() 系统调用时，我们会尽力将
	这些小写入尽可能合并，以降低发送数据包的总量。前提是该流至少有一个
	先前的数据包正在 Qdisc 队列或设备发送队列中等待。当应用程序知道
	如何/何时解除其套接字的塞入时，仍可使用 TCP_CORK 以获得最优行为。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_available_congestion_control - STRING
	显示已注册的可用拥塞控制选项。更多的拥塞控制算法可能作为模块
	可用，但尚未加载。

tcp_base_mss - INTEGER
	分组层路径 MTU 发现（MTU 探测）所使用的 search_low 初始值。若
	启用了 MTU 探测，则该值是连接使用的初始 MSS。

tcp_mtu_probe_floor - INTEGER
	如果启用了 MTU 探测，该值限制连接 search_low 所使用的最小 MSS。

	默认值：48

tcp_min_snd_mss - INTEGER
	TCP SYN 和 SYNACK 消息通常会通告一个 ADVMSS 选项，如 RFC 1122 与
	RFC 6691 所述。

	如果此 ADVMSS 选项小于 tcp_min_snd_mss，则会被静默地截断为
	tcp_min_snd_mss。

	默认值：48（每个分段至少 8 字节的有效载荷）

tcp_congestion_control - STRING
	设置用于新连接的拥塞控制算法。"reno" 算法始终可用，但依据内核配置
	可能提供其他选择。
	默认值作为内核配置的一部分设定。
	对于被动连接，将继承监听器的拥塞控制选择。

	[参见 setsockopt(listenfd, SOL_TCP, TCP_CONGESTION, "name" ...) ]

tcp_dsack - BOOLEAN
	允许 TCP 发送"重复的" SACK。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_early_retrans - INTEGER
	尾丢探测（TLP）将因尾丢导致的 RTO 转换为快速恢复（RFC8985）。注意
	TLP 需要 RACK 才能正常工作（见下文 tcp_recovery）

	可选值：

  - 0 禁用 TLP
  - 3 或 4 启用 TLP

	默认值：3

tcp_ecn - INTEGER
	控制 TCP 对显式拥塞通知（ECN）的使用。仅当 TCP 连接两端都表示支持
	时才会使用 ECN。该特性通过允许支持 ECN 的路由器在不得不丢包之前
	发出拥塞信号，有助于避免因拥塞造成丢包。支持 ECN 的主机既在 IP 层
	发送 ECN，也在 TCP 层反馈 ECN。双方对等体都支持的最高 ECN 反馈变体
	由 ECN 协商选定（Accurate ECN、ECN，或无 ECN）。

	传入连接请求的最高协商变体，以及外出连接尝试请求的最高变体：

	===== ==================== ====================
	值    传入连接             外出连接
	===== ==================== ====================
	0     无 ECN               无 ECN
	1     ECN                  ECN
	2     ECN                  无 ECN
	3     AccECN               AccECN
	4     AccECN               ECN
	5     AccECN               无 ECN
	===== ==================== ====================

	默认值：2

tcp_ecn_option - INTEGER
	当 AccECN 在握手期间成功协商后，控制 Accurate ECN（AccECN）选项的
	发送。当反向方向尚未看到 AccECN 选项时，无论此设置为何，发送逻辑
	都会抑制发送 AccECN 选项。

	可选值如下：

	= ============================================================
	0 从不发送 AccECN 选项。这也会禁用握手期间在 SYN/ACK 中发送 AccECN
	  选项。
	1 依据 draft-ietf-tcpm-accurate-ecn 中概述的最小选项规则，谨慎地
	  发送 AccECN 选项。
	2 只要能放入 TCP 选项空间，就为每个数据包发送 AccECN 选项，除非
	  触发了 AccECN 回退。
	3 只要能放入 TCP 选项空间，就为每个数据包发送 AccECN 选项，即使
	  触发了 AccECN 回退也是如此。
	= ============================================================

	默认值：2

tcp_ecn_option_beacon - INTEGER
	控制每个 RTT 内 Accurate ECN（AccECN）选项的发送频率，且仅当
	tcp_ecn_option 设为 2 时生效。

	默认值：3（AccECN 每个 RTT 至少发送 3 次）

tcp_ecn_fallback - BOOLEAN
	如果内核检测到 ECN 连接行为异常，则启用回退到非 ECN。目前，该旋钮
	实现了 RFC3168 第 6.1.1.1 节中的回退，但我们保留未来在该旋钮下
	实现其他检测机制的可能。如果 tcp_ecn 或每路由（或拥塞控制）的 ECN
	设置被禁用，则该值不被使用。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_fack - BOOLEAN
	这是一个遗留选项，已不再产生任何效果。

tcp_fin_timeout - INTEGER
	一个被孤立（不再被任何应用程序引用）的连接在本地被中止之前，保持在
	FIN_WAIT_2 状态的时长。对于未孤立的连接，这是一个完全有效的"仅接收"
	状态；而处于 FIN_WAIT_2 状态的孤立连接，否则可能会永远等待远端关闭
	其连接的一端。

	参见 tcp_max_orphans

	默认值：60 秒

tcp_frto - INTEGER
	启用 RFC5682 中定义的前向 RTO 恢复（F-RTO）。
	F-RTO 是 TCP 重传超时的一种增强恢复算法。它在 RTT 波动的网络（例如
	无线）中尤其有益。F-RTO 仅修改发送端，不需要对端提供任何支持。

	默认以非零值启用。0 禁用 F-RTO。

tcp_fwmark_accept - BOOLEAN
	如果启用，发往未设置套接字标记的监听套接字的传入连接，会将该接受
	套接字的标记设为传入 SYN 数据包的 fwmark。这将导致该连接上的所有
	数据包（从首个 SYNACK 开始）都以此 fwmark 发送。监听套接字的标记
	保持不变。已通过 setsockopt(SOL_SOCKET, SO_MARK, ...) 设置了 fwmark
	的监听套接字不受影响。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_invalid_ratelimit - INTEGER
	限制响应传入 TCP 数据包而发送重复确认的最大速率，这些数据包属于
	已存在的连接，但因以下任一原因而无效：

	  (a) 窗口外的序列号，
	  (b) 窗口外的确认号，或
	  (c) PAWS（防止序列号回绕，Protection Against Wrapped Sequence numbers）
	      检查失败

	这有助于缓解简单的"ack 循环"拒绝服务攻击，其中存在缺陷或恶意的
	中间盒或中间人（man-in-the-middle）会以某种方式改写 TCP 头部字段，
	导致每个端点都认为对方在发送无效的 TCP 分段，从而使每一方都发送
	一股无尽的重复确认流，以响应无效分段。

	使用 0 会禁用对无效分段重复确认（dupack）的速率限制；否则该值指定
	发送此类重复确认之间的最小间隔，以毫秒为单位。

	默认值：500（毫秒）。

tcp_keepalive_time - INTEGER
	启用保活时，TCP 发送保活消息的频率。
	默认值：2 小时。

tcp_keepalive_probes - INTEGER
	TCP 在判定连接已断开之前发送的保活探测次数。默认值：9。

tcp_keepalive_intvl - INTEGER
	探测发送的频繁程度。乘以 tcp_keepalive_probes 即为探测开始后在
	杀死无响应连接之前所经历的时间。默认值：75 秒，即连接将在约 11 分钟的
	重试后被中止。

tcp_l3mdev_accept - BOOLEAN
	允许子套接字继承 L3 主设备索引。启用此选项可让"全局"监听套接字跨
	L3 主域（例如 VRF）工作，由监听套接字派生的已连接套接字被绑定到
	数据包起源的 L3 域。仅当内核使用 CONFIG_NET_L3_MASTER_DEV 编译时
	有效。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_low_latency - BOOLEAN
	这是一个遗留选项，已不再产生任何效果。

tcp_max_orphans - INTEGER
	系统持有的、未绑定到任何用户文件句柄的 TCP 套接字的最大数量。如果
	超过此数量，孤立连接会立即被重置并打印警告。此限制的存在仅是为了
	防止简单的拒绝服务攻击，你_必须_不要依赖此限制或人为地降低它，而
	应（可能在增加已安装内存之后）增大它，如果网络条件要求的默认值
	更高，并调优网络服务以更积极地清理和终止此类状态。再次提醒：每个
	孤立连接最多消耗约 64K 不可交换的内存。

tcp_max_syn_backlog - INTEGER
	记住的、尚未收到连接客户端确认的连接请求（SYN_RECV）的最大数量。

	这是一个每监听器限制。

	最小值对于低内存机器为 128，并随机器内存成比例增长。

	如果服务器遭受过载，尝试增大此数值。

	记得同时检查 /proc/sys/net/core/somaxconn
	一个 SYN_RECV 请求套接字消耗约 304 字节内存。

tcp_max_tw_buckets - INTEGER
	系统同时持有的 timewait 套接字的最大数量。如果超过此数量，time-wait
	套接字会立即被销毁并打印警告。此限制的存在仅是为了防止简单的拒绝
	服务攻击，你_必须_不要人为地降低该限制，而应在网络条件要求高于
	默认值时增大它（可能在增加已安装内存之后）。

tcp_mem - 由 3 个 INTEGER 组成的向量：min、pressure、max
	min：低于此页数的 TCP 不会在意其内存占用。

	pressure：当 TCP 分配的内存量超过此页数时，TCP 会节制其内存消耗并
	进入内存压力模式，当内存消耗降到 "min" 以下时退出该模式。

	max：所有 TCP 套接字允许用于排队的页数。

	默认值在启动时依据可用内存量计算。

tcp_min_rtt_wlen - INTEGER
	用于跟踪最小 RTT 的窗口化最小值过滤器的窗口长度。较短的窗口让一个
	流在移动到更长路径（例如由于流量工程）时能更快地采用新的（更高的）
	最小 RTT。较长的窗口使过滤器更能抵抗 RTT 膨胀，例如瞬时拥塞。单位
	为秒。

	可选值：0 - 86400（1 天）

	默认值：300

tcp_moderate_rcvbuf - BOOLEAN
	如果启用，TCP 执行接收缓冲区自动调优，尝试自动调整缓冲区大小（不
	大于 tcp_rmem[^2^]）以匹配路径为达到全额吞吐所需的大小。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_rcvbuf_low_rtt - INTEGER
	rcvbuf 自动调优可能高估最终的套接字 rcvbuf，这会对高吞吐流造成缓存
	颠簸（cache trashing）。

	对于小 RTT 流（低于 tcp_rcvbuf_low_rtt 微秒），我们可以放宽 rcvbuf
	的增长：多花几毫秒达到最终（且更小）的 rcvbuf 是一个不错的折中。

	默认值：1000（1 毫秒）

tcp_mtu_probing - INTEGER
	控制 TCP 分组层路径 MTU 发现。取三个值：

 - 0 - 禁用
 - 1 - 默认禁用，在检测到 ICMP 黑洞时启用
 - 2 - 始终启用，使用 tcp_base_mss 的初始 MSS。

tcp_probe_interval - UNSIGNED INTEGER
	控制启动 TCP 分组层路径 MTU 发现重新探测的频率。默认按 RFC4821 为
	每 10 分钟重新探测一次。

tcp_probe_threshold - INTEGER
	控制 TCP 分组层路径 MTU 发现探测在搜索范围宽度（以字节计）方面何时
	停止。默认为 8 字节。

tcp_no_metrics_save - BOOLEAN
	默认情况下，TCP 在连接关闭时将各种连接度量保存到路由缓存中，以便
	不久的将来建立的连接可以使用这些数据来设置初始条件。通常这会提升
	整体性能，但有时可能导致性能下降。如果启用，TCP 在关闭连接时不会
	缓存度量。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_no_ssthresh_metrics_save - BOOLEAN
	控制 TCP 是否将 ssthresh 度量保存到路由缓存中。
	如果启用，ssthresh 度量被禁用。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_orphan_retries - INTEGER
	该值影响本地关闭的 TCP 连接在 RTO 重传一直未得到确认时的超时。
	更多细节参见 tcp_retries2。

	默认值为 8。

	如果你的机器是一个负载较重的 WEB 服务器，你应该考虑降低此值，因为
	此类套接字可能消耗大量资源。参见 tcp_max_orphans。

tcp_recovery - INTEGER
	该值是一个位图，用于启用各种实验性的丢包恢复特性。

	=========   =============================================================
	RACK: 0x1   启用 RACK 丢包检测，用于快速检测丢失的重传与尾丢，并
		    增强对重排序的韧性。目前将该位置 0 没有效果，因为 RACK
		    是唯一受支持的丢包检测算法。

	RACK: 0x2   使 RACK 的重排序窗口固定为（min_rtt/4）。

	RACK: 0x4   禁用 RACK 的 DUPACK 阈值启发式
	=========   =============================================================

	默认值：0x1

tcp_reflect_tos - BOOLEAN
	对于监听套接字，将初始 SYN 消息的 DSCP 值复用于外出数据包。这样可以
	让 TCP 流的双向都使用相同的 DSCP 值（假设 DSCP 在连接生存期内保持不变）。

	此选项同时影响 IPv4 与 IPv6。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_reordering - INTEGER
	TCP 流中数据包的初始重排序级别。
	TCP 栈随后可在该初始值与 tcp_max_reordering 之间动态调整流的重排序
	级别。

	默认值：3

tcp_max_reordering - INTEGER
	TCP 流中数据包的最大重排序级别。
	300 是一个相当保守的值，但如果路径使用了逐包负载均衡（例如 bonding
	rr 模式），你可能需要增大它。

	默认值：300

tcp_retrans_collapse - BOOLEAN
	与某些有缺陷的打印机进行"缺陷对缺陷"的兼容。在重传时尝试发送更大的
	数据包，以规避某些 TCP 栈中的缺陷。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_retries1 - INTEGER
	该值影响 TCP 在因未确认的 RTO 重传而判定出现问题的时长，并将此怀疑
	报告给网络层。
	更多细节参见 tcp_retries2。

	RFC 1122 建议至少 3 次重传，这也是默认值。

tcp_retries2 - INTEGER
	该值影响存活的 TCP 连接在 RTO 重传一直未得到确认时的超时。
	给定值 N，一个遵循指数退避、初始 RTO 为 TCP_RTO_MIN 的假定 TCP 连接，
	会在第 (N+1) 次 RTO 之前重传 N 次，然后杀死连接。

	默认值 15 产生一个假定的 924.6 秒超时，并且是有效超时的下界。
	TCP 将在首次超过假定超时的 RTO 处实际超时。
	如果降低了 tcp_rto_max_ms，建议同时更改 tcp_retries2。

	RFC 1122 建议超时至少 100 秒，这对应于至少 8 的值。

tcp_rfc1337 - BOOLEAN
	如果启用，TCP 栈的行为符合 RFC1337。如果未设置，我们不符合 RFC，但
	可防止 TCP TIME_WAIT 暗杀（assassination）。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_rmem - 由 3 个 INTEGER 组成的向量：min、default、max
	min：TCP 套接字使用的接收缓冲区最小大小。
	即使在中度内存压力下，也保证每个 TCP 套接字都能使用。

	默认值：4K

	default：TCP 套接字使用的接收缓冲区初始大小。
	该值覆盖其他协议使用的 net.core.rmem_default。
	默认值：131072 字节。
	该值导致初始窗口为 65535。

	max：TCP 套接字自动选择的接收缓冲区所允许的最大大小。
	调用 setsockopt() 并传入 SO_RCVBUF 会禁用该套接字接收缓冲区大小的
	自动调优，此时忽略此值。
	默认值：介于 131072 与 32MB 之间，取决于 RAM 大小。

tcp_sack - BOOLEAN
	启用选择确认（SACK）。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_comp_sack_rtt_percent - INTEGER
	用于压缩 SACK 特性的 SRTT 百分比。
	参见 tcp_comp_sack_nr、tcp_comp_sack_delay_ns、tcp_comp_sack_slack_ns。

	可选值：1 - 1000

	默认值：33 %

tcp_comp_sack_delay_ns - LONG INTEGER
	TCP 尝试减少发送的 SACK 数量，使用一个基于 tcp_comp_sack_rtt_percent
	的 SRTT、并以该 sysctl（以纳秒计）为上限的定时器。
	默认值为 1ms，基于 TSO 自动调整周期。

	默认值：1,000,000 ns（1 ms）

tcp_comp_sack_slack_ns - LONG INTEGER
	该 sysctl 控制 SACK 压缩所用定时器在设置时的松弛量。这为小 RTT 流
	提供了额外时间，并通过允许机会性地减少定时器中断来降低系统开销。
	过大的值可能降低有效吞吐（goodput）。

	默认值：10,000 ns（10 us）

tcp_comp_sack_nr - INTEGER
	可以被压缩的 SACK 的最大数量。
	使用 0 会禁用 SACK 压缩。

	默认值：44

tcp_backlog_ack_defer - BOOLEAN
	如果启用，处理套接字积压队列的用户线程会尝试为整个队列发送一个 ACK。
	这有助于避免 TCP 套接字系统调用结束时潜在的较长延迟。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_slow_start_after_idle - BOOLEAN
	如果启用，提供 RFC2861 行为，并在空闲一段时间后使拥塞窗口超时。空闲
	时间段定义为当前 RTO。如果未设置，拥塞窗口在空闲一段时间后不会被
	超时。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_stdurg - BOOLEAN
	使用 TCP 紧急指针字段的主机需求解释。大多数主机使用较旧的 BSD 解释，
	因此如果启用，Linux 可能无法与它们正确通信。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_synack_retries - INTEGER
	被动 TCP 连接尝试的 SYNACK 将被重传的次数。不应高于 255。默认值为 5，
	对应于在当前初始 RTO 为 1 秒的情况下，到最后一次重传为止 31 秒；
	据此，被动 TCP 连接的最终超时将在 63 秒后发生。

tcp_syncookies - INTEGER
	仅当内核使用 CONFIG_SYN_COOKIES 编译时有效。
	当套接字的 syn 积压队列溢出时发送 syncookie。这是为了防止常见的
	"SYN 泛洪攻击"（SYN flood attack）。
	默认值：1

	注意，syncookie 是回退机制。
	它绝不能用于帮助高负载服务器抵御合法的连接速率。如果你在日志中
	看到 SYN 泛洪警告，但调查显示它们是因为合法连接过载而发生的，你
	应该调优其他参数，直到该警告消失。
	参见：tcp_max_syn_backlog、tcp_synack_retries、tcp_abort_on_overflow。

	syncookie 严重违反 TCP 协议，不允许使用 TCP 扩展，可能导致某些
	服务（例如 SMTP 中继）严重降级，且这种降级你的客户端和中继可见，
	而不是你可见，因为它们在联系你。当你在日志中看到 SYN 泛洪警告但
	实际并未被泛洪时，你的服务器配置严重不当。

	如果你想测试 syncookie 对你的网络连接有何影响，可将该旋钮设为 2
	以无条件生成 syncookie。

tcp_migrate_req - BOOLEAN
	在三路握手期间收到初始 SYN 数据包时，传入连接被绑定到特定的监听
	套接字。当监听器关闭时，握手期间在途的请求套接字以及接受队列中的
	已建立套接字都会被中止。

	如果监听器启用了 SO_REUSEPORT，同一端口上的其他监听器本应能够
	接受此类连接。此选项使得在 close() 或 shutdown() 之后，可以将此类
	子套接字迁移到另一个监听器。

	通常应使用 BPF_SK_REUSEPORT_SELECT_OR_MIGRATE 类型的 eBPF 程序来
	定义选择存活监听器的策略。否则，仅当启用此选项时，内核才会随机
	选择一个存活的监听器。

	注意，在具有不同设置的监听器之间迁移可能使应用程序崩溃。假设迁移
	从监听器 A 发生到 B，且只有 B 启用了 TCP_SAVE_SYN。B 无法从迁移自 A
	的请求中读取 SYN 数据。为避免这种情况，可以返回 SK_DROP（在 eBPF
	程序的该类型中）来取消迁移，或禁用此选项。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_fastopen - INTEGER
	启用 TCP 快速打开（RFC7413），在 opening SYN 数据包中发送并接受数据。

	客户端支持由标志 0x1 启用（默认开启）。然后客户端必须使用 sendmsg()
	或 sendto() 并附带 MSG_FASTOPEN 标志，而不是 connect()，以在 SYN 中
	发送数据。

	服务器支持由标志 0x2 启用（默认关闭）。然后，要么使用另一个标志
	（0x400）为所有监听器启用，要么通过 TCP_FASTOPEN 套接字选项启用单个
	监听器，选项值为 syn-data 积压队列的长度。

	这些值（位图）为：

	=====  ======== ======================================================
	  0x1  （客户端）启用在客户端的 opening SYN 中发送数据。
	  0x2  （服务器）启用服务器支持，即允许在三次握手完成之前接受 SYN
			数据包中的数据并将其传递给应用程序。
	  0x4  （客户端）无论 cookie 是否可用、也不带 cookie 选项，在
			opening SYN 中发送数据。
	0x200  （服务器）在没有呈现任何 cookie 选项的情况下接受 SYN 中的
			数据。
	0x400  （服务器）默认情况下启用所有监听器支持快速打开，而无需
			显式的 TCP_FASTOPEN 套接字选项。
	=====  ======== ======================================================

	默认值：0x1

	注意，额外的客户端或服务器特性只有在相应基础支持（0x1 和 0x2）
	分别启用时才有效。

tcp_fastopen_blackhole_timeout_sec - INTEGER
	当发生 TFO 防火墙黑洞问题时，禁用主动 TCP 套接字上快速打开的初始
	时间段（秒）。当重新启用快速打开后再次检测到更多黑洞问题时，该时间
	段会呈指数增长；当黑洞问题消失时重置为初始值。0 表示禁用黑洞检测。

	默认情况下，它设为 0（特性被禁用）。

tcp_fastopen_key - 逗号分隔的 32 位十六进制 INTEGER 列表
	该列表由一个主密钥和一个可选的备份密钥组成。主密钥用于创建和验证
	cookie，而可选的备份密钥仅用于验证 cookie。备份密钥的目的是在密钥
	轮换时最大化 TFO 验证。

	如果 tcp_fastopen sysctl 设为 0x400（见上文），或设置了 TCP_FASTOPEN
	setsockopt() 的 optname 且此前未通过 sysctl 配置密钥，则内核可能
	选择一个随机的主密钥。如果通过 TCP_FASTOPEN_KEY optname 使用
	setsockopt() 配置密钥，则使用这些每套接字密钥，而不是通过 sysctl
	指定的任何密钥。

	密钥被指定为 4 个 8 位十六进制整数，以 '-' 分隔，格式为：
	xxxxxxxx-xxxxxxxx-xxxxxxxx-xxxxxxxx。前导零可省略。主密钥和备份密钥
	可通过逗号分隔来指定。如果只指定了一个密钥，它就成为主密钥，任何
	先前配置的备份密钥将被移除。

tcp_syn_retries - INTEGER
	主动 TCP 连接尝试的初始 SYN 将被重传的次数。不应高于 127。默认值为 6，
	对应于在当前初始 RTO 为 1 秒的情况下（tcp_syn_linear_timeouts = 4），
	到最后一次重传为止 67 秒。据此，主动 TCP 连接尝试的最终超时将在
	131 秒后发生。

tcp_timestamps - INTEGER
	启用 RFC1323 中定义的 timestamps。

 - 0：禁用。
 - 1：启用 RFC1323 中定义的 timestamps，并为每个连接使用随机偏移，而非
	 仅使用当前时间。
 - 2：类似 1，但不使用随机偏移。

	默认值：1

tcp_min_tso_segs - INTEGER
	每个 TSO 帧的最小分段数。

	自 linux-3.12 起，TCP 会根据流速率自动调整 TSO 帧大小，而不是填充
	64K 字节的数据包。对于特定用途，可以强制 TCP 构建大的 TSO 帧。注意
	如果可用窗口过小，TCP 栈可能会拆分过大的 TSO 数据包。

	默认值：2

tcp_tso_rtt_log - INTEGER
	基于 min_rtt 的 TSO 数据包大小调整

	自 linux-5.18 起，TCP 自动调整可针对具有小 RTT 的流进行微调。

	旧的自动调整是将 pacing 预算拆分为每秒发送 1024 个 TSO。

	tso_packet_size = sk->sk_pacing_rate / 1024;

	使用新机制，我们通过以下方式增大此 TSO 大小：

	distance = min_rtt_usec / (2^tcp_tso_rtt_log)
	tso_packet_size += gso_max_size >> distance;

	这意味着非常接近的主机之间的流可以使用更大的 TSO 数据包，降低其
	CPU 开销。

	如果想使用旧的自动调整，将此 sysctl 设为 0。

	默认值：9（2^9 = 512 微秒）

tcp_pacing_ss_ratio - INTEGER
	sk->sk_pacing_rate 由 TCP 栈使用应用于当前速率的比率设置。
	（current_rate = cwnd * mss / srtt）
	如果 TCP 处于慢启动，应用 tcp_pacing_ss_ratio 以让 TCP 探测更大的
	速度，假设 cwnd 可以每隔一个 RTT 翻倍。

	默认值：200

tcp_pacing_ca_ratio - INTEGER
	sk->sk_pacing_rate 由 TCP 栈使用应用于当前速率的比率设置。
	（current_rate = cwnd * mss / srtt）
	如果 TCP 处于拥塞避免阶段，应用 tcp_pacing_ca_ratio 以保守地探测
	更大的吞吐。

	默认值：120

tcp_syn_linear_timeouts - INTEGER
	在退回到指数退避超时之前，主动 TCP 连接以线性退避超时重传 SYN 的次数。
	这对被动 TCP 端的 SYNACK 没有效果。

	初始 RTO 为 1 且 tcp_syn_linear_timeouts = 4 时，我们预期 SYN RTO 为：
	1、1、1、1、1、2、4……（4 次线性超时，以及第一次使用 2^0 * initial_RTO
	的指数退避）。
	默认值：4

tcp_tso_win_divisor - INTEGER
	这允许控制单个 TSO 帧可以消耗拥塞窗口的百分比。
	该参数的设置是在突发性与构建更大的 TSO 帧之间做选择。

	默认值：3

tcp_tw_reuse - INTEGER
	在协议角度安全时，启用 TIME-WAIT 套接字对新连接的复用。

 - 0 - 禁用
 - 1 - 全局启用
 - 2 - 仅对环回流量启用

	未经技术专家的建议/请求，不应更改它。

	默认值：2

tcp_tw_reuse_delay - UNSIGNED INTEGER
        在 TIME-WAIT 套接字复用被启用的情况下，TIME-WAIT 套接字在被新连接
        复用之前所需的延迟（毫秒）。实际的复用阈值在 [N, N+1] 范围内，其中
        N 是请求的延迟（毫秒），以确保延迟间隔永远不会短于配置的值。

        此设置包含对对端 TCP 时间戳时钟嘀嗒间隔的假设。它不应被设为低于对端
        的时钟嘀嗒，以便 PAWS（防止序列号回绕）机制对被复用的连接正常工作。

        默认值：1000（毫秒）

tcp_window_scaling - BOOLEAN
	启用 RFC1323 中定义的窗口缩放。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

tcp_shrink_window - BOOLEAN
	这改变了 TCP 接收窗口的计算方式。

	RFC 7323 第 2.4 节指出，存在可撤回（retracted）窗口被提供的情况，并且
	TCP 实现必须确保它们能处理窗口收缩，如 RFC 1122 所规定。

	可选值：

 - 0（禁用） - 窗口永不收缩。
 - 1（启用）  - 在必要时为保持在自动调优设置的内存限制（sk_rcvbuf）之内，
	  窗口会被收缩。这仅在同时也存在非零的接收窗口缩放因子时才会发生。

	默认值：0（禁用）

tcp_wmem - 由 3 个 INTEGER 组成的向量：min、default、max
	min：为 TCP 套接字的发送缓冲区保留的内存量。
	每个 TCP 套接字因其诞生这一事实而有权使用它。

	默认值：4K

	default：TCP 套接字使用的发送缓冲区初始大小。该值覆盖其他协议使用的
		net.core.wmem_default。

	它通常低于 net.core.wmem_default。

	默认值：16K

	max：TCP 套接字自动调优的发送缓冲区所允许的最大内存量。该值不会覆盖
		net.core.wmem_max。调用 setsockopt() 并传入 SO_SNDBUF 会禁用该
		套接字发送缓冲区大小的自动调优，此时忽略此值。

	默认值：介于 64K 与 4MB 之间，取决于 RAM 大小。

tcp_notsent_lowat - UNSIGNED INTEGER
	TCP 套接字可以借助 TCP_NOTSENT_LOWAT 套接字选项控制其写队列中未发送
	字节的数量。如果未发送字节量低于每套接字值且写队列未满，则
	poll()/select()/epoll() 会报告 POLLOUT 事件。如果达到限制，sendmsg()
	也不会再添加新缓冲区。

	此全局变量控制未使用 TCP_NOTSENT_LOWAT 的套接字的未发送数据量。对于
	这些套接字，对全局变量的更改会立即生效。

	默认值：UINT_MAX（0xFFFFFFFF）

tcp_workaround_signed_windows - BOOLEAN
	如果启用，则假设未收到窗口缩放选项意味着远端 TCP 有缺陷，并将窗口
	视为带符号的量。如果禁用，则假设远端 TCP 没有缺陷，即使我们没有从
	它们那里收到窗口缩放选项。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_thin_linear_timeouts - BOOLEAN
	为稀疏流启用线性超时的动态触发。如果启用，会在因超时重传时执行一次
	检查，以确定流是否稀疏（在途数据包少于 4 个）。只要发现流稀疏，在
	启动指数退避模式之前，最多可执行 6 次线性超时。这改善了非激进型稀疏
	流的丢包重传延迟，这类流通常被发现是时间相关的。
	关于稀疏流的更多信息，参见
	Documentation/networking/tcp-thin.rst

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_limit_output_bytes - INTEGER
	控制每个 tcp 套接字的 TCP 小队列（Small Queue）限制。
	TCP 批量发送方倾向于增加在途数据包，直到收到丢包通知。配合 SNDBUF
	自动调优，这可能导致本地机器上排队大量数据包（例如：qdisc、CPU 积压
	队列，或设备），从而损害其他流的延迟，对于典型的 pfifo_fast qdisc
	尤其如此。tcp_limit_output_bytes 限制 qdisc 或设备上的字节数，以减少
	人为的 RTT/cwnd 并减轻缓冲膨胀（bufferbloat）。

	默认值：4194304（4 MB）

tcp_challenge_ack_limit - INTEGER
	限制每秒发送的 Challenge ACK 数量，如 RFC 5961（提升 TCP 对盲窗内攻击
	的鲁棒性）所建议。注意此每 netns 速率限制可能允许某些侧信道攻击，
	可能不应启用。TCP 栈无论如何都实现了每 TCP 套接字限制。
	默认值：INT_MAX（无限制）

tcp_ehash_entries - INTEGER
	显示当前网络命名空间中 TCP 套接字的哈希桶数量。

	负值意味着该网络命名空间不拥有自己的哈希桶，而是共享初始网络命名
	空间的哈希桶。

tcp_child_ehash_entries - INTEGER
	控制子网络命名空间中 TCP 套接字的哈希桶数量，必须在 clone() 或
	unshare() 之前设置。

	如果该值不为 0，内核使用向上取整到 2^n 的值作为实际哈希桶大小。0 是
	一个特殊值，意味着子网络命名空间将共享初始网络命名空间的哈希桶。

	注意，如果内核无法分配足够内存，子命名空间将使用全局的那个。此外，
	全局哈希桶分布在可用的 NUMA 节点上，但子哈希表的分配取决于当前进程
	的 NUMA 策略，这可能导致性能差异。

	另请注意，tcp_max_tw_buckets 与 tcp_max_syn_backlog 的默认值取决于
	哈希桶大小。

	可选值：0、2^n（n：0 - 24（16Mi））

	默认值：0

tcp_plb_enabled - BOOLEAN
	如果启用，且底层拥塞控制（例如 DCTCP）支持并启用了 PLB 特性，则启用
	TCP PLB（保护式负载均衡，Protective Load Balancing）。PLB 在以下论文
	中描述：https://doi.org/10.1145/3544216.3544226。基于 PLB 参数，在感知
	到持续拥塞时，TCP 会触发改变外出 IPv6 数据包的流标签字段。流标签字段
	的改变可能会改变使用 ECMP/WCMP 进行路由的交换机的外出数据包路径。

	PLB 改变套接字的 txhash，进而导致 IPv6 流标签字段改变，目前对 IPv4
	头部是无操作（no-op）。可以将 PLB 应用于 IPv4，使用其他网络头部字段
	（例如 TCP 或 IPv4 选项），或使用封装，其中外层头部被交换机用于确定
	下一跳。无论哪种情况，都需要主机和交换机侧的进一步更改。

	如果启用，PLB 假定拥塞信号（例如 ECN）是可用的，并被拥塞控制模块用于
	估计拥塞度量（例如 ce_ratio）。PLB 需要拥塞度量来做出重新路径决策。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

tcp_plb_idle_rehash_rounds - INTEGER
	在没有任何数据包在途的情况下，可以执行重哈希之前所看到的连续拥塞轮
	（RTT）数量。在 PLB 论文中称为 M：
	https://doi.org/10.1145/3544216.3544226。

	可选值：0 - 31

	默认值：3

tcp_plb_rehash_rounds - INTEGER
	在可以执行强制重哈希之前所看到的连续拥塞轮（RTT）数量。设置此参数时
	要小心，因为较小的值会增加重传的风险。在 PLB 论文中称为 N：
	https://doi.org/10.1145/3544216.3544226。

	可选值：0 - 31

	默认值：12

tcp_plb_suspend_rto_sec - INTEGER
	在 RTO 事件发生后暂停 PLB 的时间（秒）。为了避免让 PLB 重新路径到一个
	连通性"黑洞"，在 RTO 之后，TCP 连接会将 PLB 重新路径暂停一个介于此
	参数的 1 倍到 2 倍之间的随机时长。添加随机性是为了避免多个 TCP 连接
	并发重哈希。它应设置为修复故障链路所需的时间量。

	可选值：0 - 255

	默认值：60

tcp_plb_cong_thresh - INTEGER
	在一轮（RTT）中被标记为拥塞的数据包比例，以将该轮标记为拥塞。在 PLB
	论文中称为 K：https://doi.org/10.1145/3544216.3544226。

	0-1 的比例范围被映射到 0-256 范围，以避免浮点运算。例如，128 意味着
	如果一轮中至少 50% 的数据包被标记为拥塞，则该轮将被标记为拥塞。

	将阈值设为 0 意味着 PLB 每轮 RTT 都重新路径，无论是否有拥塞。这不是
	PLB 的预期行为，应仅用于实验目的。

	可选值：0 - 256

	默认值：128

tcp_pingpong_thresh - INTEGER
	在为估计的传入数据请求发送的估计数据应答数量，必须达到该数量后 TCP
	才认为连接是一个"乒乓"（请求-应答）连接，对此延迟确认可以提供好处。

	该阈值默认为 1，但某些应用程序可能需要更高的阈值才能获得最佳性能。

	可选值：1 - 255

	默认值：1

tcp_rto_min_us - INTEGER
	TCP 最小重传超时（微秒）。注意 rto_min 路由选项对此设置具有最高
	优先级，其次是 TCP_BPF_RTO_MIN 与 TCP_RTO_MIN_US 套接字选项，再次是
	此 tcp_rto_min_us sysctl。

	建议做法是使用小于或等于 200000 微秒的值。

	可选值：1 - INT_MAX

	默认值：200000

tcp_rto_max_ms - INTEGER
	TCP 最大重传超时（毫秒）。
	注意 TCP_RTO_MAX_MS 套接字选项具有更高优先级。

	更改 tcp_rto_max_ms 时，重要的是要理解 tcp_retries2 可能也需要更改。

	可选值：1000 - 120,000

	默认值：120,000

## UDP variables


udp_l3mdev_accept - BOOLEAN
	启用此选项可让"全局"绑定的套接字跨 L3 主域（例如 VRF）工作，数据包
	无论起源于哪个 L3 域都能被接收。仅当内核使用 CONFIG_NET_L3_MASTER_DEV
	编译时有效。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

udp_mem - 由 3 个 INTEGER 组成的向量：min、pressure、max
	所有 UDP 套接字允许用于排队的页数。

	min：所有 UDP 套接字允许用于排队的页数。

	pressure：引入此值是为了遵循 tcp_mem 的格式。

	max：引入此值是为了遵循 tcp_mem 的格式。

	默认值在启动时依据可用内存量计算。

udp_rmem_min - INTEGER
	UDP 套接字在适度情况下使用的接收缓冲区最小大小。每个 UDP 套接字都能
	使用该大小来接收数据，即使 UDP 套接字的总页数超过了 udp_mem 压力值。
	单位为字节。

	默认值：4K

udp_wmem_min - INTEGER
	UDP 没有发送内存记账，因此此可调参数没有效果。

udp_hash_entries - INTEGER
	显示当前网络命名空间中 UDP 套接字的哈希桶数量。

	负值意味着该网络命名空间不拥有自己的哈希桶，而是共享初始网络命名
	空间的哈希桶。

udp_child_hash_entries - INTEGER
	控制子网络命名空间中 UDP 套接字的哈希桶数量，必须在 clone() 或
	unshare() 之前设置。

	如果该值不为 0，内核使用向上取整到 2^n 的值作为实际哈希桶大小。0 是
	一个特殊值，意味着子网络命名空间将共享初始网络命名空间的哈希桶。

	注意，如果内核无法分配足够内存，子命名空间将使用全局的那个。此外，
	全局哈希桶分布在可用的 NUMA 节点上，但子哈希表的分配取决于当前进程
	的 NUMA 策略，这可能导致性能差异。

	可选值：0、2^n（n：7（128） - 16（64K））

	默认值：0


## RAW variables


raw_l3mdev_accept - BOOLEAN
	启用此选项可让"全局"绑定的套接字跨 L3 主域（例如 VRF）工作，数据包
	无论起源于哪个 L3 域都能被接收。仅当内核使用 CONFIG_NET_L3_MASTER_DEV
	编译时有效。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

## CIPSOv4 Variables


cipso_cache_enable - BOOLEAN
	如果启用，则启用对 CIPSO 标签映射缓存的添加与查找。如果禁用，则忽略
	添加，且查找总是未命中。然而，无论设置如何，缓存在需要时仍会被失效，
	这意味着你可以安全地打开和关闭它，缓存始终是"安全"的。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

cipso_cache_bucket_size - INTEGER
	CIPSO 标签缓存由一个固定大小的哈希表组成，每个哈希桶包含若干缓存
	表项。此变量限制每个哈希桶中的表项数量；值越大，可以缓存的 CIPSO
	标签映射就越多。当给定哈希桶中的表项数量达到此限制时，添加新表项
	会导致桶中最旧的表项被移除以腾出空间。

	默认值：10

cipso_rbm_optfmt - BOOLEAN
	启用 CIPSO 草案规范第 3.4.2.6 节（参见 Documentation/netlabel 了解细节）
	中定义的"优化的标签 1 格式（Optimized Tag 1 Format）"。这意味着设置
	后，CIPSO 标签将用空类别填充，以使数据包数据按 32 位对齐。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

cipso_rbm_strictvalid - BOOLEAN
	如果启用，则在调用 ip_options_compile() 时对 CIPSO 选项进行非常严格的
	检查。如果禁用，则放宽 ip_options_compile() 期间所做的检查。两种方式都
	是"安全"的，因为错误会在 CIPSO 处理代码的其他地方被捕获，但将此设为 0
	（False）应会减少工作量（即更快），但可能与要求严格检查的其他实现产生
	问题。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

## IP Variables


ip_local_port_range - 2 个 INTEGER
	定义 TCP 和 UDP 用于选择本地端口的本地端口范围。第一个数字是起始本地
	端口号，第二个是结束本地端口号。
	如果可能，最好这两个数字具有不同的奇偶性（一个偶数、一个奇数）。
	必须大于或等于 ip_unprivileged_port_start。
	默认值分别为 32768 和 60999。

ip_local_reserved_ports - 逗号分隔的范围列表
	指定为已知第三方应用程序保留的端口。这些端口不会被自动端口分配使用
	（例如调用 connect() 或 bind() 且端口号为 0 时）。显式的端口分配行为
	不变。

	输入和输出使用的格式都是逗号分隔的范围列表（例如端口 1、2、3、4 和
	10 为 "1,2-4,10-10"）。写入该文件将清除所有先前保留的端口，并用输入
	中给出的列表更新当前列表。

	注意 ip_local_port_range 与 ip_local_reserved_ports 设置是独立的，内核
	在确定哪些端口可用于自动端口分配时会同时考虑两者。

	你可以保留不在当前
```
    $ cat /proc/sys/net/ipv4/ip_local_port_range
    32000	60999
    $ cat /proc/sys/net/ipv4/ip_local_reserved_ports
    8080,9148

	although this is redundant. However such a setting is useful
	if later the port range is changed to a value that will
	include the reserved ports. Also keep in mind, that overlapping
	of these ranges may affect probability of selecting ephemeral
	ports which are right after block of reserved ports.

	Default: Empty

```
ip_local_port_step_width - INTEGER
        定义在临时端口范围内，当遇到不可用端口时，连续端口分配之间的数值
        最大增量。这可用于缓解在配置了保留端口时端口分布中的累积节点。请
        注意，在负载非常高的系统中，端口冲突可能更频繁。

        建议将此值严格设置为大于在 ip_local_reserved_ports 中配置的最大连续
        端口块。对于大的保留端口范围，建议将其设为最大块的 3 倍或 4 倍。使用
        等于或大于本地端口范围大小的值可完全解决端口分布不均的问题，但在
        端口耗尽的情况下会降低性能。

        默认值：0（禁用）

ip_unprivileged_port_start - INTEGER
	这是一个每命名空间 sysctl。它定义了网络命名空间中第一个非特权端口。
	特权端口需要 root 或 CAP_NET_BIND_SERVICE 才能绑定到它们。要禁用所有
	特权端口，将此设为 0。它们不得与 ip_local_port_range 重叠。

	默认值：1024

ip_nonlocal_bind - BOOLEAN
	如果启用，允许进程 bind() 到非本地 IP 地址，这可能相当有用 —— 但可能
	破坏某些应用程序。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

ip_autobind_reuse - BOOLEAN
	默认情况下，即使新套接字和绑定到该端口的所有套接字都具有
	SO_REUSEADDR，bind() 也不会自动选择端口。ip_autobind_reuse 允许 bind()
	复用该端口，这在使用 bind()+connect() 时很有用，但可能破坏某些应用程序。
	首选方案是使用 IP_BIND_ADDRESS_NO_PORT，此选项只应由专家设置。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

ip_dynaddr - INTEGER
	如果设为非零，启用对动态地址的支持。
	如果设为大于 1 的非零值，在发生动态地址重写时会打印内核日志消息。

	默认值：0

ip_early_demux - BOOLEAN
	将某些本地套接字的输入数据包处理优化为一次 demux。目前我们仅对已建立
	的 TCP 和已连接的 UDP 套接字执行此操作。

	对于纯路由工作负载，它可能会增加额外开销，降低整体吞吐，在这种情况下
	你应禁用它。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

ping_group_range - 2 个 INTEGER
	将 ICMP_PROTO 数据报套接字限制给组范围内的用户。
	默认是 "1 0"，意味着没有人（甚至 root 也不）可以创建 ping 套接字。
	将其设为 "100 100" 将授予单一组权限。"0 4294967294" 将为全世界启用它，
	"100 4294967294" 将为用户启用，但不包括守护进程。

tcp_early_demux - BOOLEAN
	为已建立的 TCP 套接字启用提前 demux。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

udp_early_demux - BOOLEAN
	为已连接的 UDP 套接字启用提前 demux。如果你的系统可能经历更多未连接
	负载，请禁用此项。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

icmp_echo_ignore_all - BOOLEAN
	如果启用，则内核将忽略发送给它的所有 ICMP ECHO 请求。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

icmp_echo_enable_probe - BOOLEAN
        如果启用，则内核将响应发送给它的 RFC 8335 PROBE 请求。

        可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

icmp_echo_ignore_broadcasts - BOOLEAN
	如果启用，则内核将忽略通过广播/组播发送给它的所有 ICMP ECHO 和
	TIMESTAMP 请求。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

icmp_ratelimit - INTEGER
	限制发送类型与 icmp_ratemask（见下文）匹配的 ICMP 数据包到特定目标的
	最大速率。
	0 表示禁用任何限制，否则为响应之间的最小间隔（毫秒）。
	注意另一个 sysctl，icmp_msgs_per_sec 限制发送到所有目标的 ICMP 数据包
	数量。

	默认值：1000

icmp_msgs_per_sec - INTEGER
	限制此主机每秒发送的 ICMP 数据包最大数量。仅类型与 icmp_ratemask（见
	下文）匹配的消息受此限制控制。出于安全原因，每秒精确的消息计数是
	随机化的。

	默认值：10000

icmp_msgs_burst - INTEGER
	icmp_msgs_per_sec 控制每秒发送的 ICMP 数据包数量，而 icmp_msgs_burst
	控制令牌桶大小。出于安全原因，精确的突发大小是随机化的。

	默认值：10000

icmp_ratemask - INTEGER
	由受速率限制的 ICMP 类型组成的掩码。

	有效位：IHGFEDCBA9876543210

	默认掩码：     0000001100000011000（6168）

	位定义（参见 include/linux/icmp.h）：

		= =========================
		0 回显应答（Echo Reply）
		3 目的地不可达（Destination Unreachable）[^1^]_
		4 源抑制（Source Quench）[^1^]_
		5 重定向（Redirect）
		8 回显请求（Echo Request）
		B 超时（Time Exceeded）[^1^]_
		C 参数问题（Parameter Problem）[^1^]_
		D 时间戳请求（Timestamp Request）
		E 时间戳应答（Timestamp Reply）
		F 信息请求（Info Request）
		G 信息应答（Info Reply）
		H 地址掩码请求（Address Mask Request）
		I 地址掩码应答（Address Mask Reply）
		= =========================

	.. [^1^] 这些默认受速率限制（见上文默认掩码）

icmp_ignore_bogus_error_responses - BOOLEAN
	某些路由器违反 RFC1122，向广播帧发送虚假应答。此类违规通常会通过内核
	警告记录。如果启用，内核将不给出此类警告，从而避免日志文件杂乱。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

icmp_errors_use_inbound_ifaddr - BOOLEAN

	如果禁用，icmp 错误消息随退出接口的主地址发送。

	如果启用，消息将随接收到导致 icmp 错误的数据包的接口的主地址发送。
	这是许多网络管理员期望从路由器获得的行为。并且它可以让调试复杂的
	网络布局容易得多。

	注意，如果所选接口不存在主地址，则无论此设置如何，都会使用第一个
	具有主地址的非环回接口的主地址。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

icmp_errors_extension_mask - UNSIGNED INTEGER
	追加到 ICMPv4 错误消息（"目的地不可达"、"超时"与"参数问题"）的 ICMP
	扩展的位掩码。原始数据报被截断/填充到 128 字节，以兼容不符合 RFC 4884
	的应用程序。

	可能的扩展如下：

	==== ==============================================================
	0x01 根据 RFC 5837 的传入 IP 接口信息。
	     扩展将包含接收到引发 ICMP 错误的数据报的 IP 接口的索引、IPv4
	     地址（若存在）、名称与 MTU。
	==== ==============================================================

	默认值：0x00（无扩展）

igmp_max_memberships - INTEGER
	更改我们可以订阅的多播组的最大数量。
	默认值：20

	理论最大值受必须单个数据报发送成员报告所限制（即报告不能跨多个数据报，
	否则可能混淆交换机并导致你非预期地离开组）。

	支持的组数量 'M' 受你能放入单个 65535 字节数据报中的组成员报告条目数
	所限制。

	M = 65536-sizeof (ip header)/(sizeof(Group record))

	组成员记录是变长的，最小为 12 字节。
	因此 net.ipv4.igmp_max_memberships 不应设置高于：

	(65536-24) / 12 = 5459

	值 5459 假设没有 IP 头部选项，因此实践中此数字可能更低。

igmp_max_msf - INTEGER
	多播组的源过滤列表中允许的地址最大数量。

	默认值：10

igmp_qrv - INTEGER
	控制 IGMP 查询鲁棒性变量（参见 RFC2236 8.1）。

	默认值：2（如 RFC2236 8.1 所规定）

	最小值：1（如 RFC6636 4.5 所规定）

force_igmp_version - INTEGER
 - 0 -（默认）不强制使用 IGMP 版本，允许 IGMPv1/v2 回退。
	  当所有 IGMPv1/v2 查询者存在计时器过期后，将回到 IGMPv3 模式。
 - 1 - 强制使用 IGMP 版本 1。如果收到 IGMPv2/v3 查询，也将回复 IGMPv1 报告。
 - 2 - 强制使用 IGMP 版本 2。如果收到 IGMPv1 查询消息，将回退到 IGMPv1。
	  如果收到 IGMPv3 查询，将回复报告。
 - 3 - 强制使用 IGMP 版本 3。与默认 0 反应相同。

```
	   this is not the same with force_mld_version because IGMPv3 RFC3376
	   Security Considerations does not have clear description that we could
	   ignore other version messages completely as MLDv2 RFC3810. So make
	   this value as default 0 is recommended.

```
`conf/interface/*`
	改变每接口的特殊设置（其中"interface"是你的网络接口的名称）

`conf/all/*`
	  是特殊的，改变所有接口的设置

log_martians - BOOLEAN
	将具有不可能地址的数据包记录到内核日志。
	如果 conf/{all,interface}/log_martians 中至少有一个设为 TRUE，则接口的
	log_martians 将被启用，否则将被禁用。

accept_redirects - BOOLEAN
	接受 ICMP 重定向消息。
	在以下情况下，接口的 accept_redirects 将被启用：

 - 在接口转发启用的情况下，conf/{all,interface}/accept_redirects 均为 TRUE

	或

 - 在接口转发禁用的情况下，conf/{all,interface}/accept_redirects 中至少
	  有一个为 TRUE

	否则接口的 accept_redirects 将被禁用。

	默认值：

  - TRUE（主机）
  - FALSE（路由器）

forwarding - BOOLEAN
	在此接口上启用 IP 转发。这控制是否可以在_此_接口上_接收_的数据包被
	转发。

mc_forwarding - BOOLEAN
	进行多播路由。内核需要使用 CONFIG_MROUTE 编译，并且需要一个多播路由
	守护进程。
	还必须将 conf/all/mc_forwarding 设为 TRUE，才能为接口启用多播路由。

medium_id - INTEGER
	用于通过其所连接的介质来区分设备的整数值。当两个设备仅在其中一个
	上接收广播数据包时，它们可以具有不同的 id 值。
	默认值 0 意味着该设备是其介质上唯一的接口，值 -1 意味着介质未知。

	目前，它用于改变 proxy_arp 行为：
	proxy_arp 特性对连接在挂载到不同介质的两个设备之间转发的数据包启用。

proxy_arp - BOOLEAN
	执行代理 arp。

	如果 conf/{all,interface}/proxy_arp 中至少有一个设为 TRUE，则接口的
	proxy_arp 将被启用，否则将被禁用。

proxy_arp_pvlan - BOOLEAN
	私有 VLAN 代理 arp。

	基本上允许代理 arp 应答回同一接口（即收到 ARP 请求/通告的接口）。

	这是为支持（以太网）交换机特性而做的，例如 RFC 3069，其中各个端口
	不允许相互通信，但允许与上游路由器通信。如 RFC 3069 所述，可以通过
	proxy_arp 让这些主机经由上游路由器通信。不需要与 proxy_arp 一起使用。

	这项技术有不同的名称：

 - 在 RFC 3069 中称为 VLAN 聚合（VLAN Aggregation）。
 - Cisco 和 Allied Telesyn 称为私有 VLAN（Private VLAN）。
 - Hewlett-Packard 称为源端口过滤（Source-Port filtering）或端口隔离
   （port-isolation）。
 - Ericsson 称为 MAC 强制转发（MAC-Forced Forwarding，RFC 草案）。

proxy_delay - INTEGER
	延迟代理应答。

	当启用 proxy_arp 或 proxy_ndp 时，延迟对邻居请求的应答。将选择一个
	介于 [0, proxy_delay) 之间的随机值，设为零表示无延迟应答。值以 jiffies
	计。默认为 80。

shared_media - BOOLEAN
	发送（路由器）或接受（主机）RFC1620 共享介质重定向。
	覆盖 secure_redirects。

	如果 conf/{all,interface}/shared_media 中至少有一个设为 TRUE，则接口的
	shared_media 将被启用，否则将被禁用。

	默认值 TRUE

secure_redirects - BOOLEAN
	仅接受重定向到接口当前网关列表中列出的网关的 ICMP 重定向消息。即使
	禁用，RFC1122 重定向规则仍然适用。

	被 shared_media 覆盖。

	如果 conf/{all,interface}/secure_redirects 中至少有一个设为 TRUE，则
	接口的 secure_redirects 将被启用，否则将被禁用。

	默认值 TRUE

send_redirects - BOOLEAN
	如果为路由器，则发送重定向。

	如果 conf/{all,interface}/send_redirects 中至少有一个设为 TRUE，则接口的
	send_redirects 将被启用，否则将被禁用。

	默认值：TRUE

bootp_relay - BOOLEAN
	接受源地址为 0.b.c.d、目标不是本机的数据包作为本地数据包。假定
	BOOTP 中继守护进程会捕获并转发此类数据包。
	还必须将 conf/all/bootp_relay 设为 TRUE，才能为接口启用 BOOTP 中继。

	默认值 FALSE

	尚未实现。

accept_source_route - BOOLEAN
	接受带有 SRR 选项的数据包。
	还必须将 conf/all/accept_source_route 设为 TRUE，才能在此接口上接受
	带有 SRR 选项的数据包。

	默认值

  - TRUE（路由器）
  - FALSE（主机）

accept_local - BOOLEAN
	接受带有本地源地址的数据包。结合适当的路由，这可用于在两个本地接口
	之间通过线缆引导数据包并使其被正确接受。
	默认值 FALSE

route_localnet - BOOLEAN
	在路由时不将环回地址视为火星（martian）源或目的。
	这使得可以使用 127/8 进行本地路由。

	默认值 FALSE

rp_filter - INTEGER
 - 0 - 不进行源验证。
 - 1 - RFC3704 严格反向路径（Strict Reverse Path）中定义的严格模式。
	  每个传入数据包都针对 FIB 进行测试，如果接口不是最佳反向路径，则
	  数据包检查将失败。默认情况下失败的数据包被丢弃。
 - 2 - RFC3704 松散反向路径（Loose Reverse Path）中定义的松散模式。
	  每个传入数据包的源地址也针对 FIB 测试，如果源地址无法通过任何
	  接口到达，则数据包检查将失败。

	RFC3704 中当前的推荐做法是启用严格模式以防止来自 DDos 攻击的 IP
	欺骗。如果使用非对称路由或其他复杂路由，则推荐松散模式。

	在 {interface} 上执行源验证时，使用 conf/{all,interface}/rp_filter 中的
	最大值。

	默认值为 0。注意某些发行版会在启动脚本中启用它。

src_valid_mark - BOOLEAN
 - 0 - 数据包的 fwmark 不包含在反向路径路由查找中。这允许仅在一个方向上
	  利用 fwmark 的非对称路由配置，例如透明代理。

 - 1 - 数据包的 fwmark 包含在反向路径路由查找中。这允许 rp_filter 在
	  fwmark 被用于双向路由流量时正常工作。

	此设置还影响在执行 ICMP 应答的源地址选择，或确定为 IPOPT_TS_TSANDADDR
	与 IPOPT_RR IP 选项存储的地址时 fmwark 的使用。

	使用 conf/{all,interface}/src_valid_mark 中的最大值。

	默认值为 0。

arp_filter - BOOLEAN
 - 1 - 允许你在同一子网拥有多个网络接口，并让每个接口的 ARP 根据该接口
	  是否会将来自被 ARP 的 IP 的数据包路由出去来应答（因此你必须使用
	  基于源的路由才能使其工作）。换句话说，它控制哪些网卡（通常为 1）
	  将响应 arp 请求。

 - 0 -（默认）内核可以用其他接口的地址响应 arp 请求。这看似错误，但通常
	  是有意义的，因为它增加了成功通信的机会。在 Linux 上，IP 地址由
	  完整的主机拥有，而非特定接口。仅对于更复杂的设置（如负载均衡），
	  此行为才会引起问题。

	如果 conf/{all,interface}/arp_filter 中至少有一个设为 TRUE，则接口的
	arp_filter 将被启用，否则将被禁用。

arp_announce - INTEGER
	定义在接口上发送的 ARP 请求中公布本地源 IP 地址的不同限制级别：

 - 0 -（默认）使用配置在任何接口上的任何本地地址
 - 1 - 尝试避免不是该接口目标子网的本地地址。当经由该接口可达的目标主机
	  要求 ARP 请求中的源 IP 地址是其接收接口上配置的逻辑网络的一部分时，
	  此模式很有用。当我们生成请求时，将检查所有包含目标 IP 的子网，并
	  如果该源地址来自此类子网则保留它。如果没有这样的子网，我们依据
	  级别 2 的规则选择源地址。
 - 2 - 始终使用此目标的最佳本地地址。
	  在此模式下，我们忽略 IP 数据包中的源地址，并尝试选择我们倾向于用于
	  与目标主机通信的本地地址。此类本地地址通过查找外出接口上所有包含
	  目标 IP 地址的子网上的主 IP 地址来选定。如果找不到合适的本地地址，
	  我们选择外出接口或所有其他接口上的第一个本地地址，期望我们能收到
	  对我们请求的应答，有时甚至不管我们公布的源 IP 地址是什么。

	使用 conf/{all,interface}/arp_announce 中的最大值。

	提高限制级别会获得更多从已解析目标收到应答的机会，而降低级别则会
	公布更多有效的发送者信息。

arp_ignore - INTEGER
	定义发送应答以响应接收到的、解析本地目标 IP 地址的 ARP 请求的不同
	模式：

 - 0 -（默认）：应答任何配置在任何接口上的本地目标 IP 地址
 - 1 - 仅当目标 IP 地址是配置在传入接口上的本地地址时才应答
 - 2 - 仅当目标 IP 地址是配置在传入接口上的本地地址，并且与发送者 IP
	  地址同属该接口上同一子网时才应答
 - 3 - 不应答以 host 作用域配置的本地地址，仅应答 global 和 link 地址的
	  解析
 - 4-7 - 保留
 - 8 - 不应答所有本地地址

	在 {interface} 上收到 ARP 请求时，使用 conf/{all,interface}/arp_ignore
	中的最大值。

arp_notify - BOOLEAN
	定义地址和设备变更的通知模式。

	 ==  ==========================================================
	  0  （默认）：不执行任何操作
	  1  在设备启用或硬件地址变更时生成免费 arp 请求
	 ==  ==========================================================

arp_accept - INTEGER
	定义接受来自尚不在 ARP 表中的设备的免费 ARP（garp）帧的行为：

 - 0 - 不在 ARP 表中创建新表项
 - 1 - 在 ARP 表中创建新表项
 - 2 - 仅当源 IP 地址与接收 garp 消息的接口上配置的地址处于同一子网时才
	  创建新表项

	如果启用此设置，免费 arp 的应答和请求类型都会触发 ARP 表更新。

	如果 ARP 表已包含免费 arp 帧的 IP 地址，无论此设置开启或关闭，ARP 表
	都会被更新。

arp_evict_nocarrier - BOOLEAN
	在 NOCARRIER 事件时清除 ARP 缓存。此选项对于无线设备很重要，在
	同一网络中的接入点之间漫游时不应清除 ARP 缓存。在大多数情况下，它
	应保持为默认值（1）。

	可选值：

 - 0（禁用） - 不在 NOCARRIER 事件时清除 ARP 缓存
 - 1（启用）  - 在 NOCARRIER 事件时清除 ARP 缓存

	默认值：1（启用）

mcast_solicit - INTEGER
	在 INCOMPLETE 状态下、关联硬件地址未知时的最大多播探测次数。默认为 3。

ucast_solicit - INTEGER
	在 PROBE 状态下、硬件地址被重新确认时的最大单播探测次数。默认为 3。

app_solicit - INTEGER
	通过 netlink 发送到用户态 ARP 守护进程、在退回到多播探测（参见
	mcast_resolicit）之前的最大探测次数。默认为 0。

mcast_resolicit - INTEGER
	在单播和 app 探测之后、PROBE 状态下的最大多播探测次数。默认为 0。

disable_policy - BOOLEAN
	对此接口禁用 IPSEC 策略（SPD）。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

disable_xfrm - BOOLEAN
	无论策略如何，在此接口上禁用 IPSEC 加密。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

igmpv2_unsolicited_report_interval - INTEGER
	下一次非请求的 IGMPv1 或 IGMPv2 报告重传将发生的时间间隔（毫秒）。

	默认值：10000（10 秒）

igmpv3_unsolicited_report_interval - INTEGER
	下一次非请求的 IGMPv3 报告重传将发生的时间间隔（毫秒）。

	默认值：1000（1 秒）

ignore_routes_with_linkdown - BOOLEAN
        在执行 FIB 查找时，忽略链路已 down 的路由。

        可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

promote_secondaries - BOOLEAN
	从此接口移除一个主 IP 地址时，提升一个对应的辅助 IP 地址，而不是
	移除所有对应的辅助 IP 地址。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

drop_unicast_in_l2_multicast - BOOLEAN
	丢弃在链路层多播（或广播）帧中接收到的任何单播 IP 数据包。

	此行为（针对多播）实际上是 RFC 1122 中的 SHOULD，但出于兼容性原因
	默认禁用。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

drop_gratuitous_arp - BOOLEAN
	丢弃所有免费 ARP 帧，例如当网络上存在一个已知的良好 ARP 代理且不需要
	使用此类帧时（或在 802.11 的情况下，必须不使用以防止攻击）。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）


tag - INTEGER
	允许你写入一个数字，可按需要使用。

	默认值为 0。

xfrm4_gc_thresh - INTEGER
	（自 linux-4.14 起废弃）
	我们将开始对 IPv4 目的缓存表项进行垃圾回收的阈值。在两倍于此值时，
	系统将拒绝新的分配。

igmp_link_local_mcast_reports - BOOLEAN
	为 224.0.0.X 范围内的链路本地多播组启用 IGMP 报告。

	默认值 TRUE

Alexey Kuznetsov.
kuznet@ms2.inr.ac.ru

Updated by:

- Andi Kleen
  ak@muc.de
- Nicolas Delon
  delon.nicolas@wanadoo.fr



## /proc/sys/net/ipv6/* 变量


IPv6 没有诸如 tcp_** 这样的全局变量。ipv4/ 下的 tcp_** 设置也适用于 IPv6
[XXX?]。

bindv6only - BOOLEAN
	IPV6_V6ONLY 套接字选项的默认值，该选项将 IPv6 套接字的使用限制为仅
	IPv6 通信。

	可选值：

 - 0（禁用） - 启用 IPv4 映射地址特性
 - 1（启用）  - 禁用 IPv4 映射地址特性

	默认值：0（禁用）

flowlabel_consistency - BOOLEAN
	保护流标签的一致性（与唯一性）。你必须禁用它才能在流标签管理器上
	使用 IPV6_FL_F_REFLECT 标志。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）

auto_flowlabels - INTEGER
	基于数据包的流哈希自动生成流标签。这允许中间设备（例如路由器）识别
	数据包流，以用于诸如等价多路径路由（Equal Cost Multipath Routing，参见
	RFC 6438）之类的机制。

	=  ===========================================================
	0  完全禁用自动流标签
	1  默认启用自动流标签，可以使用 IPV6_AUTOFLOWLABEL 套接字选项在每
	   套接字基础上禁用
	2  允许自动流标签，可以使用 IPV6_AUTOFLOWLABEL 套接字选项在每套接字
	   基础上启用
	3  启用并强制自动流标签，不能被套接字选项禁用
	=  ===========================================================

	默认值：1

flowlabel_state_ranges - BOOLEAN
	将流标签数字空间拆分为两个范围。0-0x7FFFF 保留给 IPv6 流管理器设施，
	0x80000-0xFFFFF 保留给 RFC6437 中描述的无状态流标签。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：1（启用）


flowlabel_reflect - INTEGER
	控制流标签反射。需要它才能让路径 MTU 发现在任播（anycast）环境中与
	等价多路径路由配合工作。参见 RFC 7690 与：
	https://tools.ietf.org/html/draft-wang-6man-flow-label-reflection-01

	这是一个位掩码。

 - 1：为已建立的流启用

	  注意这会阻止自动流标签变更，如"tcp: change IPv6 flow-label upon
	  receiving spurious retransmission"与"tcp: Change txhash on every
	  SYN and RTO retransmit"中所述

 - 2：为 TCP RESET 数据包（无活动监听器）启用
	  如果设置，在关闭端口上响应 SYN 数据包发送的 RST 数据包将反射传入的
	  流标签。

 - 4：为 ICMPv6 回显应答消息启用。

	默认值：0

fib_multipath_hash_policy - INTEGER
	控制多路径路由使用哪种哈希策略。

	默认值：0（第 3 层）

	可选值：

 - 0 - 第 3 层（源和目的地址加流标签）
 - 1 - 第 4 层（标准 5 元组）
 - 2 - 第 3 层，或若存在内部第 3 层则使用内部第 3 层
 - 3 - 自定义多路径哈希。用于多路径哈希计算的字段由 fib_multipath_hash_fields
	  sysctl 决定

fib_multipath_hash_fields - UNSIGNED INTEGER
	当 fib_multipath_hash_policy 设置为 3（自定义多路径哈希）时，用于
	多路径哈希计算的字段由此 sysctl 决定。

	该值是一个位掩码，用于启用多路径哈希计算所需的各个字段。

	可能的字段如下：

	====== ============================
	0x0001 源 IP 地址
	0x0002 目的 IP 地址
	0x0004 IP 协议
	0x0008 流标签（Flow Label）
	0x0010 源端口
	0x0020 目的端口
	0x0040 内部源 IP 地址
	0x0080 内部目的 IP 地址
	0x0100 内部 IP 协议
	0x0200 内部流标签（Flow Label）
	0x0400 内部源端口
	0x0800 内部目的端口
	====== ============================

	默认值：0x0007（源 IP、目的 IP 与 IP 协议）

anycast_src_echo_reply - BOOLEAN
	控制将任播地址用作 ICMPv6 回显应答的源地址。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）


idgen_delay - INTEGER
	在检测到 DAD 冲突后，重试隐私稳定地址生成的延迟（秒）。

	默认值：1（如 RFC7217 所规定）

idgen_retries - INTEGER
	在检测到 DAD 冲突后，生成稳定隐私地址的重试次数。

	默认值：3（如 RFC7217 所规定）

mld_qrv - INTEGER
	控制 MLD 查询鲁棒性变量（参见 RFC3810 9.1）。

	默认值：2（如 RFC3810 9.1 所规定）

	最小值：1（如 RFC6636 4.5 所规定）

max_dst_opts_number - INTEGER
	目标选项扩展头中允许的非填充 TLV 的最大数量。如果此值为负，则不允许
	未知选项，且允许的已知 TLV 数量为此数的绝对值。

	默认值：8

max_hbh_opts_number - INTEGER
	逐跳（Hop-by-Hop）选项扩展头中允许的非填充 TLV 的最大数量。如果此值为
	负，则不允许未知选项，且允许的已知 TLV 数量为此数的绝对值。

	默认值：8

max_dst_opts_length - INTEGER
	允许的目标选项扩展头的最大长度。

	默认值：INT_MAX（无限制）

max_hbh_length - INTEGER
	允许的逐跳选项扩展头的最大长度。

	默认值：INT_MAX（无限制）

skip_notify_on_dev_down - BOOLEAN
	控制当设备被停用或删除时，是否为被移除的路由生成 RTM_DELROUTE 消息。
	IPv4 不生成此消息；IPv6 默认生成。将此 sysctl 设为 true 会跳过该消息，
	使 IPv4 与 IPv6 在依赖用户态缓存来跟踪链路事件并驱逐路由方面保持一致。

	可选值：

 - 0（禁用） - 生成该消息
 - 1（启用）  - 跳过生成该消息

	默认值：0（禁用）

nexthop_compat_mode - BOOLEAN
	新的 nexthop API 提供了一种独立于前缀管理 nexthop 的手段。与旧路由
	格式的向后兼容性默认启用，这意味着路由转储和通知既包含新的 nexthop
	属性，也包含完整展开的 nexthop 定义。此外，对 nexthop 配置的更新或
	删除会为使用该 nexthop 的每个 fib 表项生成路由通知。一旦系统理解了
	新 API，就可以禁用此 sysctl，通过禁用 nexthop 展开和多余的通知来获得
	新 API 的全部性能收益。

	注意，作为向后兼容模式，现代特性的转储可能不完整或不正确。例如，弹性
	组（resilient groups）不会以其本来面目显示，而只是显示为下一跳列表。
	此外，不适合 8 位的权重也会显示不正确。

	默认值：true（向后兼容模式）

fib_notify_on_flag_change - INTEGER
        是否在任何时候 RTM_F_OFFLOAD/RTM_F_TRAP/RTM_F_OFFLOAD_FAILED 标志
        发生变化时发出 RTM_NEWROUTE 通知。

        将路由安装到内核后，用户态会收到一个确认，这意味着路由已安装到内核
        中，但不一定安装到了硬件中。
        已经安装到硬件中的路由也有可能改变其动作，从而改变其标志。例如，一个
        正在捕获数据包的主机路由，在安装了 IPinIP/VXLAN 隧道之后，可能被
        "提升"以执行解封装。
        这些通知将向用户态指示路由的状态。

        默认值：0（不发出通知。）

        可选值：

        - 0 - 不发出通知。
        - 1 - 发出通知。
        - 2 - 仅当 RTM_F_OFFLOAD_FAILED 标志变化时才发出通知。

ioam6_id - INTEGER
        定义此节点的 IOAM id。总共 32 位中仅使用 24 位。

        可选值范围：

        - 最小值：0
        - 最大值：0xFFFFFF

        默认值：0xFFFFFF

ioam6_id_wide - LONG INTEGER
        定义此节点的宽 IOAM id。总共 64 位中仅使用 56 位。可以与 ioam6_id
        不同。

        可选值范围：

        - 最小值：0
        - 最大值：0xFFFFFFFFFFFFFF

        默认值：0xFFFFFFFFFFFFFF

IPv6 Fragmentation:

ip6frag_high_thresh - INTEGER
	用于重组 IPv6 分片的最大内存量。当为此目的分配了 ip6frag_high_thresh
	字节的内存时，分片处理程序将丢弃数据包，直到达到 ip6frag_low_thresh。

ip6frag_low_thresh - INTEGER
	参见 ip6frag_high_thresh

ip6frag_time - INTEGER
	IPv6 分片在内存中保留的秒数。

`conf/default/*`:
	更改接口特定的默认设置。

	这些设置将在创建新接口时使用。


`conf/all/*`:
	更改所有接口特定的设置。

	[XXX：除了转发之外还有其他特殊特性吗？]

conf/all/disable_ipv6 - BOOLEAN
	更改此值与更改 `conf/default/disable_ipv6` 设置以及所有每接口
	`disable_ipv6` 设置为相同值的效果一样。

	读取此值没有任何特定含义。它不会说明 IPv6 支持是启用还是禁用。即使
	某些接口的 `disable_ipv6` 设为 0 且已配置 IPv6 地址，返回值也可能为 1。

conf/all/forwarding - BOOLEAN
	启用所有接口之间的全局 IPv6 转发。

	IPv4 和 IPv6 在此处的工作方式不同；必须使用 `force_forwarding` 标志
	来控制哪些接口可以转发数据包。

	这也会将所有接口的 Host/Router 设置 'forwarding' 设为指定值。详情
	见下文。

	这被称为全局转发。

proxy_ndp - BOOLEAN
	执行代理 ndp。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

force_forwarding - BOOLEAN
	仅在此接口上启用转发 —— 无论 `conf/all/forwarding` 的设置如何。当将
	`conf.all.forwarding` 设为 0 时，所有接口上的 `force_forwarding` 标志
	将被重置。

fwmark_reflect - BOOLEAN
	控制内核生成的、未关联任何套接字的 IPv6 应答数据包（例如 TCP RST 或
	ICMPv6 回显应答）的 fwmark。如果禁用，这些数据包的 fwmark 为零。如果
	启用，则它们具有所应答数据包的 fwmark。

	可选值：

 - 0（禁用）
 - 1（启用）

	默认值：0（禁用）

`conf/interface/*`:
	更改每接口的特殊设置。

	某些设置的功能行为取决于本地转发是否启用而不同。

accept_ra - INTEGER
	接受路由器通告（Router Advertisements）；使用它们自动配置。

	它还决定了是否传输路由器请求（Router Solicitations）。当且仅当功能
	设置为接受路由器通告时，才会传输路由器请求。

	可选值如下：

		==  ===========================================================
		 0  不接受路由器通告。
		 1  在转发禁用时接受路由器通告。
		 2  覆盖转发行为。即使转发启用也接受路由器通告。
		==  ===========================================================

	功能默认值：

  - 如果本地转发禁用，则启用。
  - 如果本地转发启用，则禁用。

accept_ra_defrtr - BOOLEAN
	在路由器通告中学习默认路由器。

	功能默认值：

  - 如果 accept_ra 启用，则启用。
  - 如果 accept_ra 禁用，则禁用。

ra_defrtr_metric - UNSIGNED INTEGER
	在路由器通告中学习到的默认路由的路由度量。该值将作为通过 IPv6 路由器
	通告学习到的默认路由的度量。仅当 accept_ra_defrtr 启用时才生效。

	可选值：
		1 到 0xFFFFFFFF

		默认值：IP6_RT_PRIO_USER，即 1024。

accept_ra_from_local - BOOLEAN
	接受源地址在本机找到的 RA（如果该 RA 在其他方面正常且可被接受）。

	默认是不接受这些，因为它可能是一个非预期的网络环回。

	功能默认值：

    - 如果在特定接口上 accept_ra_from_local 启用，则启用。
    - 如果在特定接口上 accept_ra_from_local 禁用，则禁用。

accept_ra_min_hop_limit - INTEGER
	路由器通告中的最小跳数限制（Hop limit）信息。

	小于此变量的路由器通告中的跳数限制信息将被忽略。

	默认值：1

accept_ra_min_lft - INTEGER
	路由器通告中可接受的最小生存时间值。

	生存时间小于此值的 RA 节将被忽略。零生存时间不受影响。

	默认值：0

accept_ra_pinfo - BOOLEAN
	在路由器通告中学习前缀信息（Prefix Information）。

	功能默认值：

  - 如果 accept_ra 启用，则启用。
  - 如果 accept_ra 禁用，则禁用。

ra_honor_pio_life - BOOLEAN
	是否使用 RFC4862 第 5.5.3e 节来确定与路由器通告前缀信息选项（Prefix
	Information Option）中发送的前缀匹配的地址的有效生存时间。

	可选值：

 - 0（禁用） - 使用 RFC4862 第 5.5.3e 节来确定地址的有效生存时间。
 - 1（启用）  - PIO 有效生存时间将始终被遵循。

	默认值：0（禁用）

ra_honor_pio_pflag - BOOLEAN
	前缀信息选项（Prefix Information Option）的 P 标志指示网络可以使用
	DHCPv6-PD 为每个客户端分配唯一的 IPv6 前缀。当运行用户态 DHCPv6-PD
	客户端时，可以启用此 sysctl 以使 P 标志生效：即 P 标志抑制同一 PIO 内
	A 标志的任何效果。对于给定的 PIO，P=1 且 A=1 被视为 A=0。

	可选值：

 - 0（禁用） - 忽略 P 标志。
 - 1（启用）  - P 标志将禁用给定前缀信息选项的 SLAAC 自动配置。

	默认值：0（禁用）

accept_ra_rt_info_min_plen - INTEGER
	RA 中路由信息（Route Information）的最小前缀长度。

	前缀小于此变量的路由信息将被忽略。

	功能默认值：

  - 如果 accept_ra_rtr_pref 启用，则为 0。
  - 如果 accept_ra_rtr_pref 禁用，则为 -1。

accept_ra_rt_info_max_plen - INTEGER
	RA 中路由信息的最大前缀长度。

	前缀大于此变量的路由信息将被忽略。

	功能默认值：

  - 如果 accept_ra_rtr_pref 启用，则为 0。
  - 如果 accept_ra_rtr_pref 禁用，则为 -1。

accept_ra_rtr_pref - BOOLEAN
	接受 RA 中的路由器偏好（Router Preference）。

	功能默认值：

  - 如果 accept_ra 启用，则启用。
  - 如果 accept_ra 禁用，则禁用。

accept_ra_mtu - BOOLEAN
	应用 RA 选项 5（RFC4861）中指定的 MTU 值。如果禁用，RA 中指定的 MTU
	将被忽略。

	功能默认值：

  - 如果 accept_ra 启用，则启用。
  - 如果 accept_ra 禁用，则禁用。

accept_redirects - BOOLEAN
	接受重定向（Redirects）。

	功能默认值：

  - 如果本地转发禁用，则启用。
  - 如果本地转发启用，则禁用。

accept_source_route - INTEGER
	接受源路由（路由扩展头）。

 - >= 0：仅接受路由头类型 2。
 - < 0：不接受路由头。

	默认值：0

autoconf - BOOLEAN
	使用路由器通告中的前缀信息自动配置地址。

	功能默认值：

  - 如果 accept_ra_pinfo 启用，则启用。
  - 如果 accept_ra_pinfo 禁用，则禁用。

dad_transmits - INTEGER
	发送的重复地址检测（Duplicate Address Detection）探测数量。

	默认值：1

forwarding - INTEGER
	配置接口特定的 Host/Router 行为。

```

	   It is recommended to have the same setting on all
	   interfaces; mixed router/host scenarios are rather uncommon.

	Possible values are:

		- 0 Forwarding disabled
		- 1 Forwarding enabled

	**FALSE (0)**:

	By default, Host behaviour is assumed.  This means:

	1. IsRouter flag is not set in Neighbour Advertisements.
	2. If accept_ra is TRUE (default), transmit Router
	   Solicitations.
	3. If accept_ra is TRUE (default), accept Router
	   Advertisements (and do autoconfiguration).
	4. If accept_redirects is TRUE (default), accept Redirects.

	**TRUE (1)**:

	If local forwarding is enabled, Router behaviour is assumed.
	This means exactly the reverse from the above:

	1. IsRouter flag is set in Neighbour Advertisements.
	2. Router Solicitations are not sent unless accept_ra is 2.
	3. Router Advertisements are ignored unless accept_ra is 2.
	4. Redirects are ignored.

	Default: 0 (disabled) if global forwarding is disabled (default),
	otherwise 1 (enabled).

```
hop_limit - INTEGER
	设置的默认跳数限制