
## 时间

## 1. 控制接口


用于接收网络包时间戳的接口如下：

SO_TIMESTAMP
  为每个入站数据包生成时间戳，时间基于（不一定单调的）系统时间。通过 recvmsg() 在控制消息中以微秒（usec）分辨率报告时间戳。SO_TIMESTAMP 根据 libc 的架构类型和 time_t 表示，被定义SO_TIMESTAMP_NEW SO_TIMESTAMP_OLD。对SO_TIMESTAMP_OLD，控制消息格式为 struct __kernel_old_timeval；对SO_TIMESTAMP_NEW，则struct __kernel_sock_timeval
SO_TIMESTAMPNS
  SO_TIMESTAMP 相同的打时间戳机制，但以 struct timespec 形式、以纳秒（nsec）分辨率报告时间戳。SO_TIMESTAMPNS 根据 libc 的架构类型和 time_t 表示，被定义SO_TIMESTAMPNS_NEW SO_TIMESTAMPNS_OLD。对SO_TIMESTAMPNS_OLD，控制消息格式为 struct timespec；对SO_TIMESTAMPNS_NEW，则struct __kernel_timespec
IP_MULTICAST_LOOP + SO_TIMESTAMP[NS]
  仅用于多播：通过读取回环数据包的接收时间戳来获得近似的发送时间戳
SO_TIMESTAMPING
  在接收、发送或两者上生成时间戳。支持多种时间戳来源，包括硬件。支持为流套接字生成时间戳

### 1.1 SO_TIMESTAMP（以SO_TIMESTAMP_OLD SO_TIMESTAMP_NEW

该套接字选项在接收路径上启用对数据报的时间戳。因为目标套接字（如果存在）在网络栈中很晚才被知晓，所以该功能必须对所有数据包启用。所有早期的接收时间戳选项也是如此
有关接口细节，请参见 `man 7 socket`
始终使用 SO_TIMESTAMP_NEW 时间戳，以始终获struct __kernel_sock_timeval 格式的时间戳
32 位机器上，SO_TIMESTAMP_OLD 2038 年之后会返回错误的时间戳
### 1.2 SO_TIMESTAMPNS（以SO_TIMESTAMPNS_OLD SO_TIMESTAMPNS_NEW

该选项SO_TIMESTAMP 完全相同，只是返回的数据类型不同。其 struct timespec 允许SO_TIMESTAMP timeval（毫秒）具有更高的分辨率（纳秒）
始终使用 SO_TIMESTAMPNS_NEW 时间戳，以始终获struct __kernel_timespec 格式的时间戳
32 位机器上，SO_TIMESTAMPNS_OLD 2038 年之后会返回错误的时间戳
### 1.3 SO_TIMESTAMPING（以SO_TIMESTAMPING_OLD SO_TIMESTAMPING_NEW

支持多种类型的时间戳请求。因此，这需
```

  err = setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val, sizeof(val));

```

val 是一个整型，可设置以下任意比特位。设置其他比特位会返EINVAL，且不会改变当前状态
该套接字选项为各sk_buff.3.1）、向套接字错误队列报告时间戳.3.2）以及选项.3.3）配置时间戳生成。也可以利用 cmsg.3.4）为单个 sendmsg 调用启用时间戳生成

##### 1.3.1 时间戳生

某些比特位是请求协议栈尝试生成时间戳。它们的任意组合都是有效的。对这些比特位的更改适用于新创建的数据包，而不适用于已在栈中的数据包。因此，可以通过send() 调用嵌入到两setsockopt 调用之间（一个启用时间戳生成，一个禁用）来有选择地为一部分数据包请求时间戳（例如用于采样）。时间戳也可能因为特定套接字之外的其他原因而生成，例如前面所述在系统范围内启用了接收时间戳时
SOF_TIMESTAMPING_RX_HARDWARE:
  请求由网络适配器生成的 rx 时间戳
SOF_TIMESTAMPING_RX_SOFTWARE:
  当数据进入内核时请求 rx 时间戳。这些时间戳在设备驱动将数据包交给内核接收栈之后立即生成
SOF_TIMESTAMPING_TX_HARDWARE:
  请求由网络适配器生成的 tx 时间戳。该标志可通过套接字选项和控制消息两种方式来启用
SOF_TIMESTAMPING_TX_SOFTWARE:
  当数据离开内核时请tx 时间戳。这些时间戳在设备驱动中尽可能接近（但总是在之前）将数据包传递给网络接口时生成。因此，它们需要驱动支持，并非所有设备都可用。该标志可通过套接字选项和控制消息两种方式来启用
SOF_TIMESTAMPING_TX_SCHED:
  在进入数据包调度器之前请tx 时间戳。如果较长，内核发送延迟通常由排队延迟主导。该时间戳与SOF_TIMESTAMPING_TX_SOFTWARE 处获取的时间戳之间的差值，会暴露该延迟（与协议处理无关）。协议处理所产生的延迟（如果有的话）可以通过将本时间戳减send() 之前立即获取的用户空间时间戳来计算。在带有虚拟设备的机器上，发送的包会经过多个设备、从而经过多个数据包调度器，每一层都会生成一个时间戳。这允许对排队延迟进行细粒度测量。该标志可通过套接字选项和控制消息两种方式来启用
SOF_TIMESTAMPING_TX_ACK:
  当发送缓冲区中的所有数据都已被确认时请tx 时间戳。这仅对可靠协议有意义。目前仅针对 TCP 实现。对于该协议，它可能会高估测量结果，因为时间戳是send() 处缓冲区中直至（含）该位置的所有数据都被确认时生成的：即累积确认（cumulative acknowledgment）。该机制忽略 SACK FACK。该标志可通过套接字选项和控制消息两种方式来启用
SOF_TIMESTAMPING_TX_COMPLETION:
  在数据包发送完成时请求 tx 时间戳。完成时间戳由内核在收到硬件的发送完成报告时生成。硬件可能一次报告多个数据包，而完成时间戳反映的是报告的时间，而非实际的发送时间。该标志可通过套接字选项和控制消息两种方式来启用

##### 1.3.2 时间戳报

另外三个比特位控制哪些时间戳会在生成的控制消息中被报告。对这些比特位的更改会在栈中的时间戳报告位置立即生效。时间戳仅对同时设置了相关时间戳生成请求的数据包进行报告
SOF_TIMESTAMPING_SOFTWARE:
  在可用时报告任何软件时间戳
SOF_TIMESTAMPING_SYS_HARDWARE:
  该选项已弃用并被忽略
SOF_TIMESTAMPING_RAW_HARDWARE:
  在可用时报告SOF_TIMESTAMPING_TX_HARDWARE SOF_TIMESTAMPING_RX_HARDWARE 生成的硬件时间戳

##### 1.3.3 时间戳选项


该接口支持以下选项
SOF_TIMESTAMPING_OPT_ID:
  为每个数据包生成一个唯一标识符。一个进程可以有多个并发未完成的时间戳请求。数据包在发送路径上可能被重新排序，例如在数据包调度器中。在这种情况下，时间戳会按照与原send() 调用不同的顺序排队到错误队列中。此时，仅靠时间戳顺序或有效载荷检查，并不总是能够唯一地将时间戳与原始send() 调用对应起来
  该选项send() 时的每个数据包与一个唯一标识符关联，并连同时间戳一起返回该标识符。标识符派生自一个每套接u32 计数器（会回绕）。对于数据报套接字，计数器每发送一个数据包递增；对于流套接字，每发送一个字节递增。对于流套接字，还应设置 SOF_TIMESTAMPING_OPT_ID_TCP，见下文
  计数器从零开始。它在套接字选项首次启用时初始化。每次在禁用后重新启用该选项时，计数器都会重置。重置计数器不会改变系统中已有数据包的标识符
  该选项仅针对发送时间戳实现。在那里，时间戳总是struct sock_extended_err 一起回环。该选项修改 ee_data 字段，以传递一个在该套接字所有可能并发未完成的时间戳请求中唯一id
  进程可以选择性地覆盖默认生成ID，方法是使用控制消息 SCM_TS_OPT_ID 传递一个特定的 ID（不

```

    struct msghdr *msg;
    ...
    cmsg			 = CMSG_FIRSTHDR(msg);
    cmsg->cmsg_level		 = SOL_SOCKET;
    cmsg->cmsg_type		 = SCM_TS_OPT_ID;
    cmsg->cmsg_len		 = CMSG_LEN(sizeof(__u32));
    *((__u32 *) CMSG_DATA(cmsg)) = opt_id;
    err = sendmsg(fd, msg, 0);

```

SOF_TIMESTAMPING_OPT_ID_TCP:
  对于新的 TCP 时间戳应用，将此修饰符与 SOF_TIMESTAMPING_OPT_ID 一起传递。SOF_TIMESTAMPING_OPT_ID 定义了流套接字的计数器如何递增，但其起始点并非完全简单。该选项修正了这一点
  对于流套接字，如果设置了 SOF_TIMESTAMPING_OPT_ID，也应始终设置此选项。对于数据报套接字，该选项无效
  一个合理的预期是，计数器随系统调用重置为零，使得随后写N 字节write() 生成计数器为 N-1 的时间戳。SOF_TIMESTAMPING_OPT_ID_TCP 在所有情况下都实现了这一行为
  不带修饰符的 SOF_TIMESTAMPING_OPT_ID 通常报告相同的结果，尤其是当套接字选项在没有数据传输时设置。如果正在传输数据，它可能会偏差输出队列的长度（SIOCOUTQ）
  这一差异源于基于 snd_una 还是 write_seq。snd_una 是已被对端确认的流中的偏移量，它取决于进程控制之外的因素（例如网RTT）。write_seq 是进程写入的最后一个字节，该偏移量不受外部输入影响
  当在初始创建套接字时配置（此时没有数据排队或发送），这一差异很微妙，不太可能被注意到。但无论何时设置套接字选项，SOF_TIMESTAMPING_OPT_ID_TCP 的行为都更稳健
SOF_TIMESTAMPING_OPT_CMSG:
  为所有带时间戳的数据包支recv() cmsg。对于带接收时间戳的所有数据包以及带发送时间戳IPv6 数据包，控制消息已无条件支持。该选项将其扩展到带发送时间戳IPv4 数据包。一个用例是通过同时启用套接字选项 IP_PKTINFO，将数据包与其出口设备关联起来

SOF_TIMESTAMPING_OPT_TSONLY:
  仅适用于发送时间戳。使内核将时间戳作为 cmsg 与空数据包（而不是原始数据包）一起返回。这减少了计入套接字接收预算（SO_RCVBUF）的内存量，并且即使 sysctl net.core.tstamp_allow_data 0 也能交付时间戳。该选项会禁SOF_TIMESTAMPING_OPT_CMSG
SOF_TIMESTAMPING_OPT_STATS:
  与发送时间戳一起获取的可选统计信息。它必须SOF_TIMESTAMPING_OPT_TSONLY 一起使用。当发送时间戳可用时，统计信息以类型为 SCM_TIMESTAMPING_OPT_STATS 的独立控制消息形式提供，作为一TLV（struct nlattr）类型。这些统计信息允许应用程序将各种传输层统计与发送时间戳关联起来，例如某块数据被对端接收窗口限制的时间长度
SOF_TIMESTAMPING_OPT_PKTINFO:
  为带有硬件时间戳的入站数据包启用 SCM_TIMESTAMPING_PKTINFO 控制消息。该消息包含 struct scm_ts_pktinfo，提供接收该数据包的真实接口的索引及其在二层的长度。仅当启用了 CONFIG_NET_RX_BUSY_POLL 且驱动使NAPI 时，才会返回有效的（非零）接口索引。该结构体还包含两个其他字段，但它们是保留且未定义的
SOF_TIMESTAMPING_OPT_TX_SWHW:
  当同时启SOF_TIMESTAMPING_TX_HARDWARE SOF_TIMESTAMPING_TX_SOFTWARE 时，为出站数据包同时请求硬件和软件时间戳。如果两种时间戳都生成，将有两个独立的消息回环到套接字的错误队列，每个消息只包含一个时间戳
SOF_TIMESTAMPING_OPT_RX_FILTER:
  过滤掉虚假的接收时间戳：仅当启用了匹配的时间戳生成标志时才报告接收时间戳
  接收时间戳在入口路径的早期、在数据包的目标套接字已知之前生成。如果任何套接字启用了接收时间戳，所有套接字的数据包都会收到带时间戳的数据包。包括那些请求通过 SOF_TIMESTAMPING_SOFTWARE SOF_TIMESTAMPING_RAW_HARDWARE 报告时间戳、但不请求生成接收时间戳的套接字。这可能在仅请求发送时间戳时发生
  接收虚假时间戳通常无害。进程可以忽略意外的非零值。但这会使行为微妙地依赖于其他套接字。该标志隔离了套接字，以获得更具确定性的行为
鼓励新应用程序传SOF_TIMESTAMPING_OPT_ID 以消除时间戳的歧义，并传SOF_TIMESTAMPING_OPT_TSONLY 以在无论 sysctl net.core.tstamp_allow_data 如何设置的情况下都能工作
一种例外情况是当进程需要额外的 cmsg 数据时，例如使用 SOL_IP/IP_PKTINFO 来检测出口网络接口。此时应传递选项 SOF_TIMESTAMPING_OPT_CMSG。该选项依赖于能够访问原始数据包的内容，因此无法SOF_TIMESTAMPING_OPT_TSONLY 组合使用

##### 1.3.4. 通过控制消息启用时间

除了套接字选项外，还可以针对每次写入通过 cmsg 请求时间戳生成，但仅适用SOF_TIMESTAMPING_TX_*（见1.3.1 节）。使用该特性，应用程序可以针对每个 sendmsg() 采样时间戳，而无需承担通过

```

  struct msghdr *msg;
  ...
  cmsg			       = CMSG_FIRSTHDR(msg);
  cmsg->cmsg_level	       = SOL_SOCKET;
  cmsg->cmsg_type	       = SO_TIMESTAMPING;
  cmsg->cmsg_len	       = CMSG_LEN(sizeof(__u32));
  *((__u32 *) CMSG_DATA(cmsg)) = SOF_TIMESTAMPING_TX_SCHED |
				 SOF_TIMESTAMPING_TX_SOFTWARE |
				 SOF_TIMESTAMPING_TX_ACK;
  err = sendmsg(fd, msg, 0);

```

启用和禁用时间戳而带来的开销
通过 cmsg 设置SOF_TIMESTAMPING_TX_* 标志将覆盖通过 setsockopt 设置SOF_TIMESTAMPING_TX_* 标志
此外，应用程序仍必须通过

```

  __u32 val = SOF_TIMESTAMPING_SOFTWARE |
	      SOF_TIMESTAMPING_OPT_ID /* or any other flag */;
  err = setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val, sizeof(val));

```

启用时间戳报告
### 1.4 字节流时间戳


SO_TIMESTAMPING 接口支持对字节流中的字节打时间戳。每个请求被解释为：当缓冲区的全部内容通过某个时间戳点时记录一次。也就是说，对于流，选项 SOF_TIMESTAMPING_TX_SOFTWARE 会记录所有字节何时到达设备驱动，而不管数据被转换成了多少个数据包
一般而言，字节流没有自然的分界符，因此将时间戳与数据关联起来并非易事。一个字节范围可能被分割到多个段中，任何段都可能被合并（可能将之前与独立 send() 调用关联的已分段缓冲区区段合并）。段可能被重新排序，对于实现了重传的协议，同一字节范围可能同时存在于多个段中
所有时间戳都必须实现相同的语义，无论这些可能的转换如何，否则它们无法相互比较。以不同于简单情况（缓冲区到 skb 1:1 映射）的方式处理"罕见"的边界情况是不够的，因为性能调试往往需要关注此类离群值
在实践中，如果时间戳的语义和测量的时机都选择正确，时间戳就可以与字节流的段一致地关联起来。这一挑战与为 IP 分片决定策略并无不同。在 IP 分片中，定义是只有第一个分片被打时间戳。对于字节流，我们选择只在所有字节都通过某一点时才生成时间戳。所定义SOF_TIMESTAMPING_TX_ACK 易于实现和推理。一个需要考虑 SACK 的实现会更复杂，因为可能存在传输空洞和乱序到达
在主机上，由Nagle、cork、autocork、分段和 GSO，TCP 也可能破坏从缓冲区到 skbuff 的简1:1 映射。实现通过跟踪传递给 send() 的各个最后一个字节来确保各种情况下的正确性，即使它在 skbuff 扩展或合并操作之后不再是最后一个字节。它将相关的序列号存储在 skb_shinfo(skb)->tskey 中。由于一skbuff 只有一个这样的字段，因此只能生成一个时间戳
在极少数情况下，如果两个请求被合并到同一skb 上，可能会漏掉一个时间戳请求。进程可以通过启用 SOF_TIMESTAMPING_OPT_ID，并将发送时的字节偏移量与为每个时间戳返回的值进行比较，来检测这种情况。它可以通过在请求之间始终刷TCP 栈（例如启用 TCP_NODELAY 并禁TCP_CORK autocork）来防止这种情况。在 linux-4.7 之后，防止合并的更好方法是在 sendmsg() 时使MSG_EOR 标志
这些预防措施确保仅在所有字节都通过时间戳点时才生成时间戳——前提是网络栈本身不会对段重新排序。栈确实会尽量避免重新排序。唯一的例外在管理员控制之下：可以构造一个数据包调度器配置，以不同方式延迟来自同一流的段。这样的配置较为罕见

## 2 数据接口


时间戳通过 recvmsg() 的辅助数据（ancillary data）特性读取。有关该接口的详情请参见 `man 3 cmsg`。套接字手册页（`man 7 socket`）描述了如何获取SO_TIMESTAMP SO_TIMESTAMPNS 生成的时间戳记录

### 2.1 SCM_TIMESTAMPING 记录


这些时间戳在一条控制消息中返回，其 cmsg_level SOL_SOCKET，cmsg_type SCM_TIMESTAMPING，负载类型为

```

	struct scm_timestamping {
		struct timespec ts[3];
	};

```

```

	struct scm_timestamping64 {
		struct __kernel_timespec ts[3];

```

始终使用 SO_TIMESTAMPING_NEW 时间戳，以始终获struct scm_timestamping64 格式的时间戳
32 位机器上，SO_TIMESTAMPING_OLD 2038 年之后会返回错误的时间戳
该结构体最多可返回三个时间戳。这是一个遗留特性。任何时刻至少有一个字段非零。大多数时间戳通过 ts[^0^] 传递。硬件时间戳通过 ts[^2^] 传递
ts[^1^] 过去用于保存转换为系统时间的硬件时间戳。现在改为将 NIC 上的硬件时钟设备直接作为 HW PTP 时钟源暴露出来，以便在用户空间进行时间转换，并可选地通过 linuxptp 等用户空PTP 栈来同步系统时间。有PTP 时钟 API，请参见 Documentation/driver-api/ptp.rst
请注意，如果 SO_TIMESTAMP SO_TIMESTAMPNS 选项SO_TIMESTAMPING（使SOF_TIMESTAMPING_SOFTWARE）一起启用，那么在缺少真实软件时间戳时，recvmsg() 调用会生成一个虚假的软件时间戳，并通过 ts[^0^] 传递。这种情况在硬件发送时间戳上也会发生
##### 2.1.1 使用 MSG_ERRQUEUE 的发送时间戳


对于发送时间戳，出站数据包会连同发送时间戳一起回环到套接字的错误队列。进程通过调用设置MSG_ERRQUEUE 标志recvmsg()，并提供足够大以接收相关元数据结构的 msg_control 缓冲区，来接收这些时间戳。recvmsg 调用返回原始的出站数据包，并附带两条辅助消息
一cm_level SOL_IP(V6)、cm_type IP(V6)_RECVERR 的消息内嵌了一struct sock_extended_err。它定义了错误类型。对于时间戳，ee_errno 字段ENOMSG。另一条辅助消息的 cm_level SOL_SOCKET，cm_type SCM_TIMESTAMPING。它内嵌struct scm_timestamping

#### 2.1.1.2 时间戳类

这三struct timespec 的语义由扩展错误结构体中ee_info 字段定义。它包含一SCM_TSTAMP_* 类型的值，用于定义传入 scm_timestamping 的实际时间戳
SCM_TSTAMP_** 类型与前面讨论的 SOF_TIMESTAMPING_** 控制字段一一对应，只有一个例外。出于遗留原因，SCM_TSTAMP_SND 等于零，并且可以同时SOF_TIMESTAMPING_TX_HARDWARE SOF_TIMESTAMPING_TX_SOFTWARE 设置。如ts[^2^] 非零，则为前者；否则为后者，此时时间戳存储在 ts[^0^] 中

#### 2.1.1.3 分片


出站数据报的分片很少见，但有可能发生，例如通过显式禁用 PMTU 发现。如果出站数据包被分片，那么只有第一个分片会被打时间戳并返回给发送套接字

#### 2.1.1.4 数据包负

调用应用程序通常对接收它最初传给栈的完整数据包负载不感兴趣：套接字错误队列机制只是一种捎带（piggyback）时间戳的方法。在这种情况下，应用程序可以选择用更小的缓冲区（甚至可能长度0）读取数据报。负载会相应被截断。然而，在进程对错误队列调用 recvmsg() 之前，完整的数据包会被排队，占用 SO_RCVBUF 的预算

#### 2.1.1.5 阻塞读取


从错误队列读取始终是一个非阻塞操作。要阻塞等待时间戳，请使poll select。如果错误队列上有任何数据就绪，poll() 会在 pollfd.revents 中返POLLERR。无需pollfd.events 中传递该标志。该标志在请求时被忽略。另请参`man 2 poll`

##### 2.1.2 接收时间

在接收时，没有理由从套接字错误队列读取。SCM_TIMESTAMPING 辅助数据在正常的 recvmsg() 中随数据包一起发送。由于这不是套接字错误，它不附带 SOL_IP(V6)/IP(V6)_RECVERROR 消息。在这种情况下，struct scm_timestamping 中三个字段的含义是隐式定义的。ts[^0^] 在设置时保存软件时间戳，ts[^1^] 同样已弃用，ts[^2^] 在设置时保存硬件时间戳

## 3. 硬件时间戳配置：ETHTOOL_MSG_TSCONFIG_SET/GET


硬件时间打戳还必须为每个预期执行硬件时间打戳的设备驱动进行初始化。参数定义于

```

	struct hwtstamp_config {
		int flags;	/* no flags defined right now, must be zero */
		int tx_type;	/* HWTSTAMP_TX_* */
		int rx_filter;	/* HWTSTAMP_FILTER_* */
	};

```

期望的行为通过调用 tsconfig netlink 套接`ETHTOOL_MSG_TSCONFIG_SET` 传入内核和特定设备。随`ETHTOOL_A_TSCONFIG_TX_TYPES`、`ETHTOOL_A_TSCONFIG_RX_FILTERS` `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS` 这些 netlink 属性被用来相应地设struct hwtstamp_config
`ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` netlink 嵌套属性用于选择硬件时间打戳的来源。它由设备来源的索引和时间打戳类型的限定符组成
驱动可以自由地使用比所请求配置更宽松的配置。期望驱动只直接实现所能支持的最通用模式。例如，如果硬件支持 HWTSTAMP_FILTER_PTP_V2_EVENT，那么它通常应始终将 HWTSTAMP_FILTER_PTP_V2_L2_SYNC 等向上扩展为它，因为 HWTSTAMP_FILTER_PTP_V2_EVENT 更通用（对应用程序也更有用）
支持硬件时间打戳的驱动应使用实际的、可能更宽松的配置更新该结构体。如果所请求的数据包无法被打时间戳，则不应更改任何内容，并应返回 ERANGE（与 EINVAL 相对，EINVAL 表示根本不支SIOCSHWTSTAMP）
只有具有管理员权限的进程才能更改配置。用户空间负责确保多个进程互不干扰，并且设置会被重置
任何进程都可以通过请求 tsconfig netlink 套接`ETHTOOL_MSG_TSCONFIG_GET` 来读取实际配置
旧的配置方式是使ioctl(SIOCSHWTSTAMP)，并传入一个指struct ifreq 的指针，其中 ifr_data 指向 struct hwtstamp_config。tx_type rx_filter 是对驱动预期行为的提示。如果不支持所请求的入站数据包细粒度过滤，驱动可能会为超出所请求类型的数据包打时间戳。ioctl(SIOCGHWTSTAMP) 的使用方式与 ioctl(SIOCSHWTSTAMP) 相同。但是，并非所有驱动都实现了它
```

    /* possible values for hwtstamp_config->tx_type */
    enum {
	    /*
	    * no outgoing packet will need hardware time stamping;
	    * should a packet arrive which asks for it, no hardware
	    * time stamping will be done
	    */
	    HWTSTAMP_TX_OFF,

	    /*
	    * enables hardware time stamping for outgoing packets;
	    * the sender of the packet decides which are to be
	    * time stamped by setting SOF_TIMESTAMPING_TX_SOFTWARE
	    * before sending the packet
	    */
	    HWTSTAMP_TX_ON,
    };

    /* possible values for hwtstamp_config->rx_filter */
    enum {
	    /* time stamp no incoming packet at all */
	    HWTSTAMP_FILTER_NONE,

	    /* time stamp any incoming packet */
	    HWTSTAMP_FILTER_ALL,

	    /* return value: time stamp all packets requested plus some others */
	    HWTSTAMP_FILTER_SOME,

	    /* PTP v1, UDP, any kind of event packet */
	    HWTSTAMP_FILTER_PTP_V1_L4_EVENT,

	    /* for the complete list of values, please check
	    * the include file include/uapi/linux/net_tstamp.h
	    */
    };

```

### 3.1 硬件时间戳实现：设备驱动


支持硬件时间打戳的驱动必须支ndo_hwtstamp_set NDO，并按照 SIOCSHWTSTAMP 一节所述，用实际值更新所提供struct hwtstamp_config。它还应支持 ndo_hwtstamp_get NDO 来检索配置
接收数据包的时间戳必须存储在 skb 中。要获取 skb 的共享时间戳结构的指针，调用 skb_hwtstamps()。然
```

	struct skb_shared_hwtstamps {
	    /* hardware time stamp transformed into duration
	    * since arbitrary point in time
	    */
	    ktime_t	hwtstamp;
	};

```

出站数据包的时间戳应按如下方式生成：

- hard_start_xmit() 中，检(skb_shinfo(skb)->tx_flags & SKBTX_HW_TSTAMP) 是否被设置为非零。如果是，则驱动应当执行硬件时间打戳- 如果对于skb 可行且被请求，则通过设置标志

```

      skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS;

  You might want to keep a pointer to the associated skb for the next step
  and not free the skb. A driver not supporting hardware time stamping doesn't
  do that. A driver must never touch sk_buff::tstamp! It is used to store
  software generated time stamps by the network subsystem.
```

  声明驱动正在执行时间打戳- 驱动应尽可能在将 sk_buff 传递给硬件之前调用 skb_tx_timestamp()。如果请求了软件时间戳且无法进行硬件时间戳（未设SKBTX_IN_PROGRESS），skb_tx_timestamp() 会提供一个软件时间戳- 一旦驱动发送了数据包和/或为其获取了硬件时间戳，它就通过调用 skb_tstamp_tx()，传入原skb 和原始硬件时间戳，将时间戳传回。skb_tstamp_tx() 会克隆原skb 并添加时间戳，因此现在必须释放原skb。如果获取硬件时间戳由于某种原因失败，则驱动不应回退到软件时间打戳。其理由是：这会在处理流水线的更晚阶段发生，不同于其他软件时间打戳，因此可能导致时间戳之间出现意外的差值
### 3.2 堆叠 PTP 硬件时钟的特殊注意事

在某些情况下，一个数据包的数据路径中可能存在多个 PHC（PTP 硬件时钟）。内核没有显式机制允许用户选择哪个 PHC 用于对以太网帧打时间戳。相反，其假设是最外层PHC 总是最可取的，并且内核驱动会协作以实现这一目标。目前有 3 种堆PHC 的情况，详述如下
##### 3.2.1 DSA（分布式交换架构）交换机


这些是以太网交换机，其一个端口连接到（完全不知情的）主机以太网接口，并充当带有可选转发加速功能的端口倍增器。每DSA 交换机端口对用户而言表现为一个独立的（虚拟）网络接口，而其网络 I/O 在底层是间接地通过主机接口执行的（TX 时重定向到主机端口，RX 时拦截帧）
DSA 交换机连接到主机端口时，PTP 同步必然受到影响，因为交换机的可变排队延迟会在主机端口与PTP 伙伴之间引入路径延迟抖动。因此，一DSA 交换机自带时间戳时钟，并能够在自己的 MAC 上执行网络时间戳，从而使路径延迟仅测量线缆和 PHY 的传播延迟。Linux 支持带时间戳DSA 交换机，并暴露与任何其他网络接口相同ABI（除DSA 接口在网I/O 方面实际上是虚拟的这一点，它们确实拥有自己PHC）。DSA 交换机的所有接口共享同一PHC 是典型情况，但非强制
按设计，使用 DSA 交换机进PTP 时间戳不需要对其所连接的主机端口驱动做任何特殊处理。然而，当主机端口也支持 PTP 时间戳时，DSA 会负责拦截针对主机端口的 `.ndo_eth_ioctl` 调用，并阻止在其上启用硬件时间戳的尝试。这是因SO_TIMESTAMPING API 不允许为同一数据包交付多个硬件时间戳，因此必须阻DSA 交换机端口之外的任何其他方这样做
在通用层，DSA PTP 时间戳提供了以下基础设施
- `.port_txtstamp()`：在发送带有来自用户空间的硬件 TX 时间戳请求的数据包之前调用的钩子。这是两步（two-step）时间戳所必需的，因为硬件时间戳在实际 MAC 发送之后才可用，所以驱动必须准备好将该时间戳与原始数据包关联起来，以便将其重新入队到套接字的错误队列。为了在时间戳可用时保存数据包，驱动可以调用 `skb_clone_sk`，将克隆指针保存skb->cb 中，并将 tx skb 入队。通常，交换机会有一PTP TX 时间戳寄存器（有时是 FIFO），时间戳在那里可用。在 FIFO 的情况下，硬件可能存PTP 序列 ID/消息类型/域号与实际时间戳的键值对。为了在等待时间戳的队列中的数据包与实际时间戳之间正确关联，驱动可以使用 BPF 分类器（`ptp_classify_raw`）来识别 PTP 传输类型，使`ptp_parse_header` 来解PTP 头部字段。可能在该时间戳可用时触发一IRQ，或者驱动可能不得不在调`dev_queue_xmit()` 发往主机接口后进行轮询。一步（one-step）TX 时间戳不需要克隆数据包，因PTP 协议不需要后续消息（因为 TX 时间戳由 MAC 嵌入到数据包中），因此用户空间不期望TX 时间戳的数据包被重新入队到其套接字的错误队列
- `.port_rxtstamp()`：在 RX 时，DSA 运行 BPF 分类器来识别 PTP 事件消息（任何其他数据包，包PTP 通用消息，都不被打时间戳）。原始（也是唯一的）可打时间戳的 skb 被提供给驱动，以便它在时间戳立即可用时为其加上时间戳注解，或推迟到以后。在接收时，时间戳可能以带内方式可用（通过 DSA 头部中的元数据，或以其他方式附加到数据包上），或以带外方式可用（通过另一RX 时间FIFO）。在 RX 上推迟通常是必要的，当获取时间戳需要一个可休眠的上下文时。在这种情况下，DSA 驱动负责在刚打上时间戳的 skb 上调`netif_rx()`
##### 3.2.2 以太PHY


这些是通常在网络栈中扮演第 1 层角色的设备，因此它们不DSA 交换机那样拥有网络接口表示。然而，出于性能原因，PHY 可能能够检测并PTP 数据包打时间戳：尽可能靠近线缆获取的时间戳有可能产生更稳定、更精确的同步
支持 PTP 时间戳的 PHY 驱动必须创建一``struct mii_timestamper` 并在 `phydev->mii_ts`` 中添加指向它的指针。网络栈会检查该指针是否存在
由于 PHY 没有网络接口表示，它们的打时间戳ethtool ioctl 操作需要由其各自的 MAC 驱动来中介。因此，DSA 交换机不同，需要为每个单独MAC 驱动进行修改以支PHY 时间戳。这包括
- `.ndo_eth_ioctl` 中，检`phy_has_hwtstamp(netdev->phydev)` 是否为真。如果是，则 MAC 驱动不应处理该请求，而是使用 `phy_mii_ioctl()` 将其传递给 PHY
- RX 时，可能需要也可能不需要特殊干预，取决于用于将 skb 向上传递网络栈的函数。对于普通的 `netif_rx()` 及类似函数，MAC 驱动必须检`skb_defer_rx_timestamp(skb)` 是否必要——如果必要，则完全不要调`netif_rx()`。如果启用了 `CONFIG_NETWORK_PHY_TIMESTAMPING`，且 `skb->dev->phydev->mii_ts` 存在，则`.rxtstamp()` 钩子现在会被调用，以使用DSA 非常相似的逻辑来确定是否需要为 RX 时间戳推迟。同样像 DSA 一样，当时间戳可用时，PHY 驱动负责将数据包向上发送到栈
  对于其他 skb 接收函数，如 `napi_gro_receive` `netif_receive_skb`，栈会自动检`skb_defer_rx_timestamp()` 是否必要，因此驱动内部不需要此检查
- TX 上，同样，可能需要也可能不需要特殊干预。调`mii_ts->txtstamp()` 钩子的函数名`skb_clone_tx_timestamp()`。这个函数既可以直接调用（这种情况下确实需要有显式MAC 驱动支持），但该函数也会`skb_tx_timestamp()` 调用中捎带执行，而许MAC 驱动已经为了软件时间戳目的执行了 `skb_tx_timestamp()`。因此，如果 MAC 支持软件时间戳，在此阶段无需再做任何事情
##### 3.2.3 MII 总线嗅探设备


它们扮演与带时间戳的以太PHY 相同的角色，只是它们是分立设备，因此可以连同任何 PHY 一起使用，即使PHY 不支持时间戳。在 Linux 中，它们可通过设备树（Device Tree）被发现并附加到 `struct phy_device`，其余部分使用与那些相同mii_ts 基础设施。更多细节请参见 Documentation/devicetree/bindings/ptp/timestamper.txt
##### 3.2.4 MAC 驱动的其他注意事

堆叠 PHC 的使用可能会暴露出在没有它们时不可能触发MAC 驱动 bug。一个例子与这行代码有关
```

      skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS;

```

任何 TX 时间戳逻辑——无论是普通的 MAC 驱动、DSA 交换机驱动、PHY 驱动还是 MII 总线嗅探设备驱动——都应设置此标志。但是，一个不知道 PHC 堆叠MAC 驱动可能会因为除自身之外的其他方设置了此标志而陷入混乱，并交付重复的时间戳
例如，一个典型的 TX 时间戳驱动设计可能将发送部分拆分为 2 部分
1. "TX"：检PTP 时间戳是否之前已通过 `.ndo_eth_ioctl`（`priv->hwtstamp_tx_enabled == true`）启用，且当skb 需要一TX 时间戳（`skb_shinfo(skb)->tx_flags & SKBTX_HW_TSTAMP`）。如果为真，它设"`skb_shinfo(skb)->tx_flags |= SKBTX_IN_PROGRESS`" 标志。注意：如上所述，在堆PHC 系统的情况下，这个条件不应触发，因为MAC 肯定不是最外层PHC。但这不是典型问题所在。发送随该数据包继续
2. "TX confirmation"：发送已完成。驱动检查是否有必要为其收集任何 TX 时间戳。典型问题就出在这里：MAC 驱动走捷径，只检查是否设置了 "`skb_shinfo(skb)->tx_flags & SKBTX_IN_PROGRESS`"。在堆叠 PHC 系统中，这是不正确的，因为该 MAC 驱动并非 TX 数据路径中唯一可能首先启用 SKBTX_IN_PROGRESS 的实体
此问题的正确解决方案是，MAC 驱动在其 "TX confirmation" 部分进行复合检查，不仅要检"`skb_shinfo(skb)->tx_flags & SKBTX_IN_PROGRESS`"，还要检"`priv->hwtstamp_tx_enabled == true`"。由于系统的其余部分确保 PTP 时间戳不会对最外层 PHC 之外的任何对象启用，这一增强检查将避免向用户空间交付重复的 TX 时间戳