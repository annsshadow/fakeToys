
## SMC Sysctl


## /proc/sys/net/smc/* 变量


autocorking_size - INTEGER（整数）
	设置 SMC 自动 cork（打 cork）的大小：
	从应用程序的视角看，SMC 自动 corking 类似于 TCP 自动 corking。当应用程序
	进行连续的小 write()/sendmsg() 系统调用时，我们会尽可能多地合并这些小写入，
	以降低被发送的 CDC 和 RDMA Write 的总量。
	autocorking_size 限制在一次发送中可以发送到下层设备的最大 cork 字节数。
	如果设为 0，则禁用 SMC 自动 corking。
	应用程序在知道如何/何时"开塞"（uncork）其套接字时，仍可使用 TCP_CORK 以获得
	最优行为。

	Default（默认）：64K

smcr_buf_type - INTEGER（整数）
	控制之后新建的 SMC-R 链路组使用哪种类型的 sndbuf 和 RMB。仅适用于 SMC-R。

	Default（默认）：0（物理连续（physically contiguous）的 sndbuf 和 RMB）

	Possible values（可能的值）：

 - 0 - 使用物理连续的缓冲区
 - 1 - 使用虚拟连续的缓冲区
 - 2 - 混合使用两种类型。先尝试物理连续的缓冲区。
	  若不可用，再使用虚拟连续的缓冲区。

smcr_testlink_time - INTEGER（整数）
	SMC-R 链路在上一次连接活动之后，以何种频率发出 TEST_LINK LLC 消息来确认链路
	可用性。值为 0 表示禁用 TEST_LINK。

	Default（默认）：30 秒。

wmem - INTEGER（整数）
	SMC 套接字使用的发送缓冲区的初始大小。

	最小值为 16KiB，对最大值没有硬性限制，但 SMC-R 只允许 512KiB，SMC-D 只允许
	1MiB。

	Default（默认）：64KiB

rmem - INTEGER（整数）
	SMC 套接字使用的接收缓冲区（RMB）的初始大小。

	最小值为 16KiB，对最大值没有硬性限制，但 SMC-R 只允许 512KiB，SMC-D 只允许
	1MiB。

	Default（默认）：64KiB

smcr_max_links_per_lgr - INTEGER（整数）
	控制可添加到一个 SMC-R 链路组的最大链路数。注意实际添加到一个 SMC-R 链路组的
	链路数取决于系统中存在的 RDMA 设备数量。可接受的值范围为 1 到 2。仅适用于
	SMC-R v2.1 及以后。

	Default（默认）：2

smcr_max_conns_per_lgr - INTEGER（整数）
	控制可添加到一个 SMC-R 链路组的最大连接数。可接受的值范围为 16 到 255。仅
	适用于 SMC-R v2.1 及以后。

	Default（默认）：255

smcr_max_send_wr - INTEGER（整数）
	所谓的 work request 缓冲区是执行 RDMA 操作所需的 SMCR 链路（以及 RDMA 队列对）
	级资源。由于最多 255 个连接可以共享一个链路组，因而也共享一个链路，而 work
	request 缓冲区的数量是在链路被分配时决定的，根据具体工作负载，它可能成为瓶颈，
	即线程必须等待 work request 缓冲区变得可用。在引入此控制之前，发送路径上可用
	的 work request 缓冲区的最大数量被硬编码为 16。有了此控制后，它变得可配置。
	可接受的范围为 2 到 2048。

	请注意，所有缓冲区都需要被分配为一个物理上连续的数组，其中每个元素是一个单独的
	缓冲区，大小为 SMC_WR_BUF_SIZE（48）字节。如果分配失败，我们会以一半的缓冲区
	数量持续重试，直到成功，或者（不太可能）降到旧的硬编码值 16，这时我们就像引入
	此控制之前那样放弃。

	Default（默认）：16

smcr_max_recv_wr - INTEGER（整数）
	所谓的 work request 缓冲区是执行 RDMA 操作所需的 SMCR 链路（以及 RDMA 队列对）
	级资源。由于最多 255 个连接可以共享一个链路组，因而也共享一个链路，而 work
	request 缓冲区的数量是在链路被分配时决定的，根据具体工作负载，它可能成为瓶颈，
	即线程必须等待 work request 缓冲区变得可用。在引入此控制之前，接收路径上可用
	的 work request 缓冲区的最大数量被硬编码为 16。有了此控制后，它变得可配置。
	可接受的范围为 2 到 2048。

	请注意，所有缓冲区都需要被分配为一个物理上连续的数组，其中每个元素是一个单独的
	缓冲区，大小为 SMC_WR_BUF_SIZE（48）字节。如果分配失败，我们会以一半的缓冲区
	数量持续重试，直到成功，或者（不太可能）降到旧的硬编码值 16，这时我们就像引入
	此控制之前那样放弃。

	Default（默认）：48

limit_smc_hs - INTEGER（整数）
	是否限制新创建套接字的 SMC 握手。

	启用时，SMC 监听路径会根据握手工作线程拥塞情况以及排队的 SMC 握手负载来施加
	握手限制。

	Possible values（可能的值）：

 - 0 - 禁用握手限制
 - 1 - 启用握手限制

	Default（默认）：0（禁用）

hs_ctrl - STRING（字符串）
	按名称选择 SMC 握手控制配置文件。

	该字符串引用一个用户实现的、类型为 smc_hs_ctrl 的 BPF struct_ops 实例的名称。

	所选配置文件控制在 TCP SYN/SYN-ACK 握手期间是否通告 SMC 选项。

	仅在启用 CONFIG_SMC_HS_CTRL_BPF 时可用。写入空字符串以清除当前配置文件。

	Default（默认）：空字符串
