
## RDS


## 概述


本自述文件试图提供关于 RDS 来龙去脉的一些背景，并希望帮助你熟悉代码。

此外，请参见这封关于 RDS 起源的邮件：
http://oss.oracle.com/pipermail/rds-devel/2007-November/000228.html

## RDS 架构


RDS 通过在集群中任意两个节点之间使用单一可靠连接，提供可靠、有序的数据报投递。这让
应用程序可以使用单个套接字与集群中的任何其他进程通信——因此在一个有 N 个进程的集群中，
你只需要 N 个套接字，而如果使用像 TCP 这样面向连接的套接字传输，则需要 N*N 个。

RDS 并非 Infiniband 专用；它被设计为支持不同的传输。当前的实现曾经同时支持 RDS over
TCP 和 IB。

从应用程序的角度看，RDS 的高层语义如下：

 - 寻址

	RDS 使用 IPv4 地址和 16 位端口号来标识连接的端点。所有涉及在内核与用户空间之间
	传递地址的套接字操作通常使用 struct sockaddr_in。

	使用 IPv4 地址并不意味着底层传输必须基于 IP。实际上，RDS over IB 使用的是可靠的
	IB 连接；IP 地址仅用于定位远程节点的 GID（通过对给定 IP 进行 ARP 查询）。

	端口空间完全独立于 UDP、TCP 或任何其他协议。

 - 套接字接口

	RDS 套接字**大多数情况下**的工作方式与你从 BSD 套接字所期望的一致。下一节将介绍
	细节。无论如何，所有 I/O 都通过标准 BSD 套接字 API 执行。像零拷贝支持这样的一些
	附加功能通过控制消息实现，而其他扩展则使用 getsockopt/setsockopt 调用。

	套接字必须先绑定，然后才能发送或接收数据。这是必需的，因为绑定还会选择一个传输并
	将其附加到套接字。一旦绑定，传输分配就不会改变。RDS 可以容忍 IP 的移动（例如在
	active-active HA 场景中），但前提是地址不移动到不同的传输。

 - sysctls

	RDS 在 /proc/sys/net/rds 中支持若干 sysctl。


## 套接字接口


  AF_RDS, PF_RDS, SOL_RDS
	AF_RDS 和 PF_RDS 是与 socket(2) 一起使用以创建 RDS 套接字的域类型。SOL_RDS 是
	与 setsockopt(2) 和 getsockopt(2) 一起用于 RDS 特定套接字选项的套接字级别。

  fd = socket(PF_RDS, SOCK_SEQPACKET, 0);
	这会创建一个新建的、未绑定的 RDS 套接字。

  setsockopt(SOL_SOCKET): send and receive buffer size
	RDS 遵循发送和接收缓冲区大小的套接字选项。不允许向套接字排队超过 SO_SNDSIZE
	字节。消息在调用 sendmsg 时入队，并在远程系统确认其到达时离队。

	SO_RCVSIZE 选项控制最大接收队列长度。这是一个软限制而非硬限制——RDS 将继续接受
	并排队传入的消息，即使这会使队列长度超过限制。不过，它也会将该端口标记为“拥塞”
	（congested）并向源节点发送拥塞更新。源节点应当抑制任何向此拥塞端口发送数据的进程。

  bind(fd, &sockaddr_in, ...)
	这将套接字绑定到一个本地 IP 地址和端口，以及一个传输（如果尚未通过
	SO_RDS_TRANSPORT 套接字选项选择的话）。

  sendmsg(fd, ...)
	向指定的接收者发送一条消息。如果底层可靠连接尚未建立，内核会透明地建立它。

	尝试发送超过 SO_SNDSIZE 的消息将返回 -EMSGSIZE。

	尝试发送会使排队字节总数超过 SO_SNDSIZE 阈值的消息将返回 EAGAIN。

	尝试向标记为“拥塞”的目标发送消息将返回 ENOBUFS。

  recvmsg(fd, ...)
	接收排队到此套接字的消息。套接字的接收队列计数会调整，如果队列长度降到 SO_SNDSIZE
	以下，该端口被标记为未拥塞，并向所有对等节点发送拥塞更新。

	应用程序可以要求 RDS 内核模块通过控制消息接收通知（例如，当拥塞更新到达时，或当
	RDMA 操作完成时会有通知）。这些通知通过 struct msghdr 的 msg.msg_control 缓冲区
	接收。消息的格式在手册页中描述。

  poll(fd)
	RDS 支持 poll 接口，以允许应用程序实现异步 I/O。

	POLLIN 的处理相当直接。当有传入消息排队到套接字，或有待处理通知时，我们会发出
	POLLIN 信号。

	POLLOUT 稍微难一些。由于你基本上可以向任何目标发送，只要发送队列还有空间（即排队的
	字节数小于发送缓冲区大小），RDS 就会一直发出 POLLOUT 信号。

	然而，内核会拒绝接受发往标记为拥塞的目标的消息——在这种情况下，如果你依赖 poll 来
	告诉你要做什么，就会陷入无限循环。这不是一个简单的问题，但应用程序可以通过使用拥塞
	通知，以及检查 sendmsg 返回的 ENOBUFS 错误来处理——

  setsockopt(SOL_RDS, RDS_CANCEL_SENT_TO, &sockaddr_in)
	这允许应用程序丢弃在此特定套接字上排队到某个特定目标的所有消息。

	这让应用程序在检测到超时时可以取消未完成的消息。例如，如果它尝试发送一条消息，而
	远程主机不可达，RDS 会一直重试。应用程序可能认为这不值得，于是取消该操作。在这种
	情况下，它会使用 RDS_CANCEL_SENT_TO 来清除任何待处理的消息。

  `setsockopt(fd, SOL_RDS, SO_RDS_TRANSPORT, (int **)&transport ..), getsockopt(fd, SOL_RDS, SO_RDS_TRANSPORT, (int **)&transport ..)`
	设置或读取一个整数，用于定义套接字上 RDS 数据包所使用的底层封装传输。设置该选项时，
	整数参数可以是 RDS_TRANS_TCP 或 RDS_TRANS_IB 之一。读取该值时，未绑定套接字将返回
	RDS_TRANS_NONE。此套接字选项在套接字上只能设置恰好一次，且必须在通过 bind(2) 系统
	调用绑定之前。对于传输之前已显式（通过 SO_RDS_TRANSPORT）或隐式（通过 bind(2)）
	附加的套接字，尝试设置 SO_RDS_TRANSPORT 将返回 EOPNOTSUPP 错误。尝试将
	SO_RDS_TRANSPORT 设置为 RDS_TRANS_NONE 将始终返回 EINVAL。

## RDS 的 RDMA


  see rds-rdma(7) manpage (available in rds-tools)


## 拥塞通知


  see rds(7) manpage


## RDS 协议


  消息头

    The message header is a 'struct rds_header' (see rds.h):

    字段：

      h_sequence:
	  每个数据包的序列号
      h_ack:
	  对收到的最后一个数据包的捎带确认
      h_len:
	  数据长度，不包括头
      h_sport:
	  源端口
      h_dport:
	  目的端口
      h_flags:
	  可以是：

	  =============  ==================================
	  CONG_BITMAP    这是拥塞更新位图
	  ACK_REQUIRED   接收者必须确认此数据包
	  RETRANSMITTED  数据包之前已发送过
	  =============  ==================================

      h_credit:
	  向连接的另一端指示它有更多可用的信用（即还有更多发送空间）
      h_padding[^4^]:
	  未使用，留作将来使用
      h_csum:
	  头校验和
      h_exthdr:
	  可选数据可在此传递。目前用于传递 RDMA 相关信息。

  ACK 与重传处理

      有人可能认为，有了可靠的 IB 连接，就不需要确认已收到的消息。问题在于 IB 硬件在将
      消息 DMA 到内存之前就会生成确认消息。如果在发送确认之后、消息被 DMA 并处理之前，
      HCA 因任何原因被禁用，就可能造成消息丢失。这只有在存在另一个可用于故障转移的 HCA
      时才是一个潜在问题。

      立即发送确认可以让发送方快速从其发送队列中释放已发送的消息，但可能导致用于确认的
      流量过多。RDS 将确认捎带在发送的数据包上。通过只允许同时有一个纯确认包在途，并且
      发送方仅在其发送缓冲区开始填满时才请求确认，可以减少纯确认包的数量。所有重传也都会
      被确认。

  流控

      RDS 的 IB 传输使用基于信用的机制来验证对端的接收缓冲区中是否有空间容纳更多数据。这
      消除了连接在硬件层面重试的需要。

  拥塞

      在接收套接字接收队列中等待的消息会计入套接字的 SO_RCVBUF 选项值。只计入消息中的
      有效载荷字节。如果排队的字节数等于或超过 rcvbuf，则套接字处于拥塞状态。所有发往该
      套接字地址的发送尝试应当阻塞或返回 -EWOULDBLOCK。

      应用程序应当进行合理调优，以使这种情况极少发生。遇到这种“背压”（back-pressure）的
      应用程序被视为存在 bug。

      这一机制的实现方式是：每个节点维护位图，指示绑定地址上哪些端口处于拥塞状态。当位图
      发生变化时，它会通过所有终止于发生变化位图本地地址的连接发送出去。

      位图在连接建立时分配。这避免了在中断处理路径（将消息排队到套接字）中分配内存。密集
      位图让传输在任何位图变化时都能相当高效地发送整个位图。这比某种更细粒度的每端口拥塞
      通信要容易实现得多。发送方通过一次非常廉价的位测试来判断它即将发往的端口是否拥塞。


## RDS 传输层


  As mentioned above, RDS is not IB-specific. Its code is divided
  into a general RDS layer and a transport layer.

  通用层处理套接字 API、拥塞处理、回环、统计、用户内存固定，以及连接状态机。

  The transport layer handles the details of the transport. The IB
  transport, for example, handles all the queue pairs, work requests,
  CM event handlers, and other Infiniband details.


## RDS 内核结构


  struct rds_message
    也称可能为 "rds_outgoing"，通用 RDS 层根据套接字 API 复制待发送的数据并按需设置头
    字段。随后它被排队给各个连接，并由连接的传输发送。

  struct rds_incoming
    一个通用结构，指代传入的数据，可以从传输层交给通用代码，并在唤醒套接字时由通用代码
    排队。随后它被传回传输层代码以处理实际的复制到用户空间操作。

  struct rds_socket
    每套接字信息

  struct rds_connection
    每连接信息

  struct rds_transport
    指向传输特定函数的指针

  struct rds_statistics
    与传输无关的统计数据

  struct rds_cong_map
    包装原始拥塞位图，包含 rbnode、waitq 等。


## 连接管理


  连接可能处于 UP、DOWN、CONNECTING、DISCONNECTING 和 ERROR 状态。

  当 RDS 套接字首次尝试向某个节点发送数据时，会分配并建立一个连接。该连接随后被永久
  维持——如果出现传输错误，连接将被丢弃并重新建立。

  在数据包排队时丢弃连接，会导致已排队或部分发送的数据报在连接重新建立时被重传。


## 发送路径


  rds_sendmsg()
    - 从传入数据构建 struct rds_message
    - 解析 CMSG（例如 RDMA 操作）
    - 如果尚未分配并连接传输连接，则分配并连接之
    - 将 rds_message 放入发送队列
    - 唤醒发送工作线程

  rds_send_worker()
    - 调用 rds_send_xmit() 直到队列为空

  rds_send_xmit()
    - 如果有待处理的拥塞映射则发送之
    - 可能设置 ACK_REQUIRED
    - 调用传输发送非 RDMA 或 RDMA 消息（RDMA 操作从不重传）

  rds_ib_xmit()
    - 从发送环分配工作请求
    - 向对端添加任何可用新发送信用（h_credits）
    - 映射 rds_message 的 sg 列表
    - 捎带确认
    - 填充工作请求
    - 向连接的队列对提交发送


## 接收路径


  rds_ib_recv_cq_comp_handler()
    - 查看写完成
    - 从设备取消映射接收缓冲区
    - 无错误，调用 rds_ib_process_recv()
    - 补充接收环

  rds_ib_process_recv()
    - 校验头校验和
    - 如果是新数据报的开始，则将头复制到 rds_ib_incoming 结构
    - 添加到 ibinc 的 fraglist
    - 如果数据报已完成：
  - 如果数据报是拥塞更新，则更新拥塞映射
  - 否则调用 rds_recv_incoming()
  - 注意是否需要确认

  rds_recv_incoming()
    - 丢弃重复数据包
    - 响应 ping
    - 查找与此数据报关联的套接字
    - 添加到套接字队列
    - 唤醒套接字
    - 进行一些拥塞计算
  rds_recvmsg
    - 将数据复制到用户 iovec
    - 处理 CMSG
    - 返回给应用程序


## 多路径 RDS（mprds）


  Mprds 即多路径 RDS，主要面向 RDS-over-TCP（尽管该概念可扩展到其他传输）。RDS-over-TCP
  的经典实现通过将任意两个端点（端点 == [IP 地址, 端口]）之间的多个 PF_RDS 套接字复用在
  一个涉及两个 IP 地址的单一 TCP 套接字之上来实现。其局限性在于它最终会将多个 RDS 流
  汇聚到单一 TCP 流上，因此它（a）受限于单一流的带宽上限，（b）会遭遇所有 RDS 套接字的
  队头阻塞。

  通过在每个 rds/tcp 连接上使用多个 TCP/IP 流，即多路径 RDS（mprds），可以获得更好的
  吞吐量（对于固定的小数据包大小、MTU）。每个这样的 TCP/IP 流构成 rds/tcp 连接的一条
  路径。RDS 套接字将基于某种哈希（例如本地地址和 RDS 端口号）附加到一条路径，并且该 RDS
  套接字的数据包将通过在附加路径上使用 TCP 来在该路径上分段/重组 RDS 数据报。

  多路径 RDS 的实现方式是将 struct rds_connection 拆分为一个公共（对所有路径）部分和每
  路径的 struct rds_conn_path。所有 I/O 工作队列和重连线程都由 rds_conn_path 驱动。像
  TCP 这样支持多路径的传输可以为每个 rds_conn_path 建立一个 TCP 套接字，这由传输通过传输
  私有的 cp_transport_data 指针管理。

  传输在注册到 rds 核心模块时通过设置 t_mp_capable 位来声明自己支持多路径。当传输支持
  多路径时，rds_sendmsg() 将 outgoing 流量在多条路径上进行哈希。出向哈希基于 PF_RDS
  套接字绑定的本地地址和端口计算。

  此外，即使传输支持 MP，我们也可能与某个不支持 mprds、或支持不同路径数的节点建立对等
  连接。因此，对等节点需要就连接要使用的路径数达成一致。这是通过在第一条数据包之前发送
  控制包交换来完成的。当传输支持多路径时，控制包交换必须在 rds_sendmsg() 中的出向哈希
  完成之前完成。

  控制包是一个 RDS ping 数据包（即发往 RDS 目的端口 0 的数据包），该 ping 数据包带有
  类型为 RDS_EXTHDR_NPATHS、长度为 2 字节的 RDS 扩展头选项，其值为发送方支持的路径数。
  “探测”ping 数据包将从某个保留端口 RDS_FLAG_PROBE_PORT（在 <linux/rds.h> 中）发出。
  因此，收到来自 RDS_FLAG_PROBE_PORT 的 ping 的接收方可以立即计算出
  min(sender_paths, rcvr_paths)。当接收方支持 mprds 时，作为对探测 ping 响应而发送的
  pong 应包含接收方的 npaths。

  如果接收方不支持 mprds，ping 中的 exthdr 将被忽略。在这种情况下，pong 不会带有任何
  exthdr，因此探测 ping 的发送方可以默认使用单路径 mprds。
