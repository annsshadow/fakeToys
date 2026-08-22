
## RxRPC 网络协议


RxRPC 协议驱动UDP 之上提供了一个可靠的两阶段传输，可用于执RxRPC 远程操作。这是通过
AF_RXRPC 族的套接字，使用 sendmsg() recvmsg() 配合控制数据来发送和接收数据、中止和错误
本文档内容：

 (#) 概述
 (#) RxRPC 协议摘要
 (#) AF_RXRPC 驱动模型
 (#) 控制消息
 (#) 套接字选项
 (#) 安全性
 (#) 示例客户端用法
 (#) 示例服务端用法
 (#) AF_RXRPC 内核接口
 (#) 可配置参数

## 概述


RxRPC 是一个两层协议。有一个会话层，它使用 UDP over IPv4（或 IPv6）作为传输层来提供可靠的
虚拟连接，但实现的是一个真正的网络协议；还有一个表示层，它使用 XDR 把结构化数据渲染二进制块，再转换回来
```
		+-------------+
		| Application |
		+-------------+
		|     XDR     |		Presentation
		+-------------+
		|    RxRPC    |		Session
		+-------------+
		|     UDP     |		Transport
		+-------------+
```

AF_RXRPC 提供
 (1) 一RxRPC 设施的一部分，供内核和应用程序同时使用，方法是把其中的会话部分做成一     Linux 网络协议（AF_RXRPC）
 (2) 一个两阶段协议。客户端发送一个二进制块（请求），然后接收一个二进制块（应答）；服务     接收请求，然后发送应答
 (3) 保留为一个调用所建立的传输系统中可复用的部分，以加速后续调用
 (4) 一个安全协议，使用 Linux 内核的密钥保留设施来在客户端管理安全。服务端端在协商安全     必须更为活跃
AF_RXRPC 不提XDR 编组/表示设施。那留给应用程序。AF_RXRPC 只处理二进制块。即便是操作 ID
也只是请求二进制块的前四个字节，因此超出了内核的关注范围

AF_RXRPC 族的套接字：

 (1) 以类SOCK_DGRAM 创建
 (2) 提供它们将要使用的底层传输类型的协议——目前只支持 PF_INET

Andrew 文件系统（AFS）是使用它并且同时具有内核（文件系统）和用户空间（工具）组件的应用程序的
例子

## RxRPC 协议摘要


RxRPC 协议概述
 (#) RxRPC 位于另一个网络协议之上（目前唯一选项UDP），并用它来提供网络传输。例如，UDP
     端口提供传输端点
 (#) RxRPC 支持来自任何给定传输端点的多个虚连接"，从而允许端点被共享，甚至共享到同一     远程端点
 (#) 每个连接都通向一个特定的"服务"。一个连接不能通向多个服务。一个服务可以被认为RxRPC
     对端口号的等价物。AF_RXRPC 允许多个服务共享一个端点
 (#) 客户端发起的数据包被标记，因此一个传输端点可以在客户端连接和服务端连接之间共享（连接
     有方向）
 (#) 在一个本地传输端点与某个远程端点上的一个服务之间，可以并发支持多达十亿个连接。一RxRPC

```
	Local address	}
	Local port	} Transport (UDP) address
	Remote address	}
	Remote port	}
	Direction
	Connection ID
	Service ID
```

 (#) 每个 RxRPC 操作都是一调用"（call）。一个连接最多可以进行四十亿次调用，但在任意时刻
     一个连接上最多只能有四次调用在进行
 (#) 调用是两阶段且非对称的：客户端发送其请求数据，由服务接收；然后服务发送应答数据，由客户端
     接收
 (#) 数据块的大小不定，一个阶段的结束由数据包中的一个标志标记。组成一个块的数据包数量不得超过
     四十亿，否则会导致序列号回绕
 (#) 请求数据的前四个字节是服务操ID
 (#) 安全是逐连接协商的。连接由抵达其上的第一个数据包发起。如果请求了安全，服务端随后发出一     "challenge"（质询），然后客户端用一"response"（响应）回复。如果响应成功，该安全为此连     的生存期设置，并且在该连接上进行的后续所有调用都使用同一安全。如果服务端在客户端之前让一     连接失效，则当客户端再次使用该连接时，安全将被重新协商
 (#) 调用使用 ACK 数据包来处理可靠性。数据数据包在每个调用内还被显式地排序
 (#) 有两种积极确认：hard-ACK（硬确认）和 soft-ACK（软确认）。hard-ACK 向对端表明，到某一点为     收到的所有数据都已被接收并处理；soft-ACK 表明数据已被接收，但可能仍会被丢弃并重新请求。发送方
     在数据包hard-ACK 之前不得丢弃任何可发送的数据包
 (#) 接收一个应答数据数据包会隐式地对组成请求的所有数据数据包进行 hard-ACK
 (#) 当一个调用已发送请求、已接收应答，并且应答最后一个数据包上的最hard-ACK 已到达服务端时，
     该调用完成
 (#) 一个调用在其完成之前的任何时候都可以被任一端中止

## AF_RXRPC 驱动模型


关于 AF_RXRPC 驱动
 (#) AF_RXRPC 协议透明地使用传输协议的内部套接字来表示传输端点
 (#) AF_RXRPC 套接字映射到 RxRPC 连接束。实际的 RxRPC 连接被透明地处理。一个客户端套接字可用于
     对同一服务进行多个并发调用。一个服务端套接字可处理来自许多客户端的调用
 (#) 将发起额外的并行客户端连接，以支持额外的并发调用，上限可调
 (#) 每个连接在最后一个正在使用它的调用完成之后，会被保留一段时[可调]，以防可以复用它的新
     调用出现
 (#) 每个内部 UDP 套接字在最后一个使用它的连接被丢弃之后，会被保[可调] 一段时[可调]，以     可以复用它的新连接出现
 (#) 一个客户端连接只有在调用具有描述其安全的相key 结构体时，才会在调用之间共享（并且假设这     调用本来也会共享该连接）。未加保护的调用也能够彼此共享连接
 (#) 一个服务端连接由客户端说可以共享时才共享
 (#) ACK（确认）由协议驱动自动处理，包括 ping 回复
 (#) SO_KEEPALIVE 自动 ping 另一端以保持连接存活 [TODO]
 (#) 如果收到一ICMP 错误，所有受该错误影响的调用将被中止，并通过 recvmsg() 传递一个适当的网     错误

RxRPC 套接字用户的交互
 (#) 一个套接字通过绑定一个具有非零服ID 的地址而成为服务端套接字
 (#) 在客户端，发送一个请求是通过一个或多个 sendmsg 完成的，随后通过一个或多个 recvmsg 接收应答
 (#) 从客户端发出的请求的第一sendmsg 包含一个标记（tag），用于与该调用关联的所有其sendmsg
     recvmsg。该标记携带在控制数据中
 (#) connect() 用于为客户端的套接字提供一个默认目标地址。这可以通过给调用的第一sendmsg() 提供
     一个备用地址（struct msghdr::msg_name）来覆盖
 (#) 如果在未绑定的客户端上调connect()，在操作发生前会绑定一个随机的本地端口
 (#) 一个服务端套接字也可用于进行客户端调用。为此，该调用的第一sendmsg() 必须指定目标地址     服务端的传输端点用于发送数据包
 (#) 一旦应用程序接收了与某个调用关联的最后一条消息，就保证不会再看到该标记，因此可以用它     固定客户端资源。然后可以用相同的标记发起一个新调用，而不必担心相互干扰
 (#) 在服务端，一个请求通过一个或多个 recvmsg 接收，然后应答通过一个或多个 sendmsg 发送，然后
     最终的 ACK 通过一个最后的 recvmsg 接收
 (#) 当为某个调用发送数据时，如果在该调用上还有更多数据要来，sendmsg 会被赋予 MSG_MORE
 (#) 当为某个调用接收数据时，如果在该调用上还有更多数据要来，recvmsg 会标MSG_MORE
 (#) 当为某个调用接收数据或消息时，recvmsg 会标MSG_EOR 以指示该调用的最终消息
 (#) 一个调用可以通过在控制数据中添加一个中止控制消息来中止。发出中止会终止内核对该调用标记的使用     任何在该调用的接收队列中等待的消息都将被丢弃
 (#) 中止、忙通知（busy notification）和质询数据包通过 recvmsg 传递，并且控制数据消息将被设置     指示上下文。接收一个中止或忙消息会终止内核对该调用标记的使用
 (#) msghdr 结构体的控制数据部分用于若干用途：

     (#) 目标或受影响的调用的标记
     (#) 发送或接收错误、中止和忙通知
     (#) 传入调用的通知
     (#) 发送调试请求和接收调试回复 [TODO]
 (#) 当内核接收并建立一个传入调用时，它会向服务端应用程序发送一条消息，让它知道有一个新调用在等     它的接受 [recvmsg 报告一个特殊的控制消息]。然后服务端应用程序使用 sendmsg 为新调用分配一个标记     一旦完成，请求数据的第一部分将由 recvmsg 传递
 (#) 服务端应用程序必须向服务端套接字提供一个密钥环（keyring），其中包含与其允许的安全类型对应的
     密钥。当建立一个安全连接时，内核在密钥环中查找适当的密钥，然后向客户端发送一个质询数据包     接收一个响应数据包。内核随后检查该数据包的授权，要么中止连接，要么建立安全
 (#) 客户端将用于保护其通信的密钥的名称由一个套接字选项指定

关于 sendmsg 的注意事项：

 (#) 可以设置 MSG_WAITALL，告sendmsg 忽略信号，只要对端在合理时间内取得进展、使我们得以把要
     发送的所有数据排入队列即可。这要求客户端在2*RTT 的时间段内至少接收一个数据包
     如果没有设置这个，sendmsg() 会立即返回，如果什么都没消费则返回 EINTR/ERESTARTSYS，否则返     已消费的数据量

关于 recvmsg 的注意事项：

 (#) 如果接收队列中有一系列属于某个特定调用的数据消息，那么 recvmsg 将持续处理它们，直到
     (a) 它遇到该调用已接收数据的末尾
     (b) 它遇到一个非数据消息
     (c) 它遇到属于另一个调用的消息，或

     (d) 它填满了用户缓冲区
     如果 recvmsg 在阻塞模式下被调用，它将持续睡眠，等待进一步数据的接收，直到上述四个条件之一
     被满足
 (2) MSG_PEEK 操作类似，但如果它已在缓冲区中放入了任何数据，它会立即返回，而不是一直睡眠直     能填满缓冲区
 (3) 如果一个数据消息在填满用户缓冲区时只被部分消费，那么该消息的剩余部分将留在队列前端供下一     接收者使用。永远不会标MSG_TRUNC
 (4) 如果一个调用还有数据可取（它尚未复制该阶段最后一个数据消息的最后一个字节），那么将标记
     MSG_MORE銆。

## 控制消息


AF_RXRPC 利用 sendmsg() recvmsg() 中的控制消息来多路复用调用、调用某些操作并报告某些状况它们是：

	=======================	=== ===========	===============================
	MESSAGE ID		SRT DATA	MEANING
	=======================	=== ===========	===============================
	RXRPC_USER_CALL_ID	sr- User ID	App's call specifier
	RXRPC_ABORT		srt Abort code	Abort code to issue/received
	RXRPC_ACK		-rt n/a		Final ACK received
	RXRPC_NET_ERROR		-rt error num	Network error on call
	RXRPC_BUSY		-rt n/a		Call rejected (server busy)
	RXRPC_LOCAL_ERROR	-rt error num	Local error encountered
	RXRPC_NEW_CALL		-r- n/a		New call received
	RXRPC_ACCEPT		s-- n/a		Accept new call
	RXRPC_EXCLUSIVE_CALL	s-- n/a		Make an exclusive client call
	RXRPC_UPGRADE_SERVICE	s-- n/a		Client call can be upgraded
	RXRPC_TX_LENGTH		s-- data len	Total length of Tx data
	=======================	=== ===========	===============================

	(SRT = usable in Sendmsg / delivered by Recvmsg / Terminal message)

 (#) RXRPC_USER_CALL_ID

     这用于指示应用程序的调用 ID。它是一个无符号长整型，由应用程序在客户端通过把它附加到第一     数据消息、或在服务端通过RXRPC_ACCEPT 消息的关联中传递它来指定。recvmsg() 在除
     RXRPC_NEW_CALL 消息之外的所有消息中传递它
 (#) RXRPC_ABORT

     这可被应用程序用来通过把它传给 sendmsg 来中止一个调用，或者可recvmsg 传递以指示收到了一     远程中止。无论哪种方式，它都必须RXRPC_USER_CALL_ID 关联以指定受影响的调用。如果要发送一     中止，但不存在具有该用户 ID 的调用，则将返回错误 EBADSLT
 (#) RXRPC_ACK

     这被传递给服务端应用程序，以指示从客户端收到了一个调用的最ACK。它将与 RXRPC_USER_CALL_ID
     关联，以指示现在已经完成的调用
 (#) RXRPC_NET_ERROR

     这被传递给应用程序，以指示在尝试与对端通信的过程中遇到ICMP 错误消息。控制消息数据中会包     一errno 类的整数值来指示问题，RXRPC_USER_CALL_ID 将指示受影响的调用
 (#) RXRPC_BUSY

     这被传递给客户端应用程序，以指示一个调用因服务端正忙而被拒绝。它将与 RXRPC_USER_CALL_ID 关联     以指示被拒绝的调用
 (#) RXRPC_LOCAL_ERROR

     这被传递给应用程序，以指示遇到了本地错误，并因此中止了一个调用。控制消息数据中会包含一errno
     类的整数值来指示问题，RXRPC_USER_CALL_ID 将指示受影响的调用
 (#) RXRPC_NEW_CALL

     这被传递以向服务端应用程序指示一个新的调用已经到达并正在等待接受。没有与之关联的用户 ID，因     之后必须通过执行 RXRPC_ACCEPT 来分配一个用ID
 (#) RXRPC_ACCEPT

     这被服务端应用程序用来尝试接受一个调用并为其分配一个用ID。它应当RXRPC_USER_CALL_ID 关联     以指示要分配的用ID。如果没有要接受的调用（它可能已超时、被中止等），则 sendmsg 将返回错     ENODATA。如果该用户 ID 已被另一个调用使用，则将返回错误 EBADSLT
 (#) RXRPC_EXCLUSIVE_CALL

     这用于指示一个客户端调用应当在一个一次性的连接上进行。该连接会在调用终止后被丢弃
 (#) RXRPC_UPGRADE_SERVICE

     这用于进行一个客户端调用，以探测指定的服ID 是否可被服务端升级。调用者必须检recvmsg() 返回
     msg_name 中实际使用的服务 ID。被探测的操作必须是在两个服务中都采用相同参数的那个
     一旦用这建立了服务端升级能力（或缺乏该能力），返回的服ID 应当用于未来到该服务端的所有通信     并且不应再设RXRPC_UPGRADE_SERVICE
 (#) RXRPC_TX_LENGTH

     这用于把一次调用（无论是客户端请求还是服务应答）将要传输的数据总量告知内核。如果给出，它允     内核直接从用户空间缓冲区加密到数据包缓冲区，而不是先复制到缓冲区再就地加密。这只能随为一次调     提供数据的第一sendmsg() 一起给出。如果实际给出的数据量不同，将产EMSGSIZE
     它接受一__s64 类型的参数，指示将要传输多少。该值不得小于零
符号 RXRPC__SUPPORTED 被定义为比所支持的最高控制消息类型大一。在运行时，这可以通过
RXRPC_SUPPORTED_CMSG 套接字选项（见下文）来查询

## 套接字选项


AF_RXRPC 套接字在 SOL_RXRPC 层级支持少数几个套接字选项
 (#) RXRPC_SECURITY_KEY

     这用于指定要使用的密钥的描述。该密钥通过 request_key() 从调用进程的密钥环中提取，并且应当是
     "rxrpc" 类型
     optval 指针指向描述字符串，optlen 指示字符串的长度（不NUL 终止符）
 (#) RXRPC_SECURITY_KEYRING

     与上面类似，但指定要使用的服务端密钥的密钥环（密钥类"keyring"）。参安全一节
 (#) RXRPC_EXCLUSIVE_CONNECTION

     这用于请求在本套接字上后续进行的每次调用都使用新连接。optval 应为 NULL optlen 0
 (#) RXRPC_MIN_SECURITY_LEVEL

     这用于指定本套接字上调用所需的最低安全级别。optval 必须指向一个包含下列值之一int
     (a) RXRPC_SECURITY_PLAIN

	 仅加密校验和
     (b) RXRPC_SECURITY_AUTH

	 加密校验和，外加数据包被填充且前八个字节被加密——其中包含实际的数据包长度
     (c) RXRPC_SECURITY_ENCRYPT

	 加密校验和，外加整个数据包被填充并加密，包括实际的数据包长度
 (#) RXRPC_UPGRADEABLE_SERVICE

     这用于指示一个具有两个绑定的服务套接字可以在客户端请求时把一个绑定的服务升级到另一个。optval
     必须指向一个包含两个无符号短整型的数组。第一个是要从中升级的服务 ID，第二个是要升级到的服务 ID
 (#) RXRPC_SUPPORTED_CMSG

     这是一个只读选项，它把一int 写入缓冲区，指示所支持的最高控制消息类型

## 瀹夊叏鎬。

目前，只实现kerberos 4 的等价协议（安全索引 2 - rxkad）。这需要加rxkad 模块，并且在客户需要从 AFS kaserver kerberos 服务器获取适当类型的票据（ticket）并安装"rxrpc" 类型的密钥这通常使用 klog 程序完成。一个简单的 klog 示例程序可以在下面找到：

	http://people.redhat.com/~dhowells/rxrpc/klog.c

提供给客户端 add_key() 的有效载荷应当采用以下结构：

```
	struct rxrpc_key_sec2_v1 {
		uint16_t	security_index;	/* 2 */
		uint16_t	ticket_length;	/* length of ticket[] */
		uint32_t	expiry;		/* time at which expires */
		uint8_t		kvno;		/* key version number */
		uint8_t		__pad[3];
		uint8_t		session_key[8];	/* DES session key */
		uint8_t		ticket[0];	/* the encrypted ticket */
	};
```

其中票据二进制块只是附加在上述结构之后

对于服务端，必须"rxrpc_s" 类型的密钥对服务端可用。它们的描述"<serviceID>:<securityIndex>"
（例如："52:2" 表示 AFS VL 服务rxkad 密钥）。当创建这样一个密钥时，应当把服务端的密钥作为
实例化数据提供给它（见下面的例子）
	add_key("rxrpc_s", "52:2", secret_key, 8, keyring);

一个密钥环通过在一sockopt 中命名它而被传给服务端套接字。然后当建立安全的传入连接时，服务端
套接字在这个密钥环中查找服务端密钥。这可以在一个示例程序中看到，该程序位于
	http://people.redhat.com/~dhowells/rxrpc/listen.c


## 示例客户端用

客户端会按下述方式发起一个操作：

```
	client = socket(AF_RXRPC, SOCK_DGRAM, PF_INET);

     Where the third parameter indicates the protocol family of the transport
     socket used - usually IPv4 but it can also be IPv6 [TODO].

 (2) A local address can optionally be bound::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= 0,  /* we're a client */
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7000), /* AFS callback */
		.transport.sin_address	= 0,  /* all local interfaces */
	};
	bind(client, &srx, sizeof(srx));

     This specifies the local UDP port to be used. If not given, a random
     non-privileged port will be used. A UDP port may be shared between
     several unrelated RxRPC sockets. Security is handled on a basis of
     per-RxRPC virtual connection.

 (3) The security is set::

	const char *key = "AFS:cambridge.redhat.com";
	setsockopt(client, SOL_RXRPC, RXRPC_SECURITY_KEY, key, strlen(key));

     This issues a request_key() to get the key representing the security
     context. The minimum security level can be set::

	unsigned int sec = RXRPC_SECURITY_ENCRYPT;
	setsockopt(client, SOL_RXRPC, RXRPC_MIN_SECURITY_LEVEL,
		   &sec, sizeof(sec));

 (4) The server to be contacted can then be specified (alternatively this can
     be done through sendmsg)::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= VL_SERVICE_ID,
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7005), /* AFS volume manager */
		.transport.sin_address	= ...,
	};
	connect(client, &srx, sizeof(srx));

 (5) The request data should then be posted to the server socket using a series
     of sendmsg() calls, each with the following control message attached:

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

     MSG_MORE should be set in msghdr::msg_flags on all but the last part of
     the request. Multiple requests may be made simultaneously.

     An RXRPC_TX_LENGTH control message can also be specified on the first
     sendmsg() call.

     If a call is intended to go to a destination other than the default
     specified through connect(), then msghdr::msg_name should be set on the
     first request message of that call.

 (6) The reply data will then be posted to the server socket for recvmsg() to
     pick up. MSG_MORE will be flagged by recvmsg() if there's more reply data
     for a particular call to be read. MSG_EOR will be set on the terminal
     read for a call.

     All data will be delivered with the following control message attached:

	RXRPC_USER_CALL_ID	- specifies the user ID for this call

     If an abort or error occurred, this will be returned in the control data
     buffer instead, and MSG_EOR will be flagged to indicate the end of that
     call.
```

客户端可以请求一个它已知的服ID，并通过在调用的第一sendmsg() 上提RXRPC_UPGRADE_SERVICE
来要求在有更好的服务可用时把它升级到更好的服务。客户端随后应当在收集结果时检查由 recvmsg() 填充msg_name 中的 srx_service。如果该升级请求被服务忽略了，srx_service 将持有与传给 sendmsg() 相同的值；
否则它会被改为指示服务端升级到的服务 ID。注意，升级后的服务 ID 由服务端选择。调用者必须等到它在应中看到该服务 ID 之后，才能发送任何更多的调用（在相同目的地上的进一步调用会被阻塞，直到探测结束）

## 示例服务端用

服务端会按下述方式建立以接受操作
```
	server = socket(AF_RXRPC, SOCK_DGRAM, PF_INET);

     Where the third parameter indicates the address type of the transport
     socket used - usually IPv4.

 (2) Security is set up if desired by giving the socket a keyring with server
     secret keys in it::

	keyring = add_key("keyring", "AFSkeys", NULL, 0,
			  KEY_SPEC_PROCESS_KEYRING);

	const char secret_key[8] = {
		0xa7, 0x83, 0x8a, 0xcb, 0xc7, 0x83, 0xec, 0x94 };
	add_key("rxrpc_s", "52:2", secret_key, 8, keyring);

	setsockopt(server, SOL_RXRPC, RXRPC_SECURITY_KEYRING, "AFSkeys", 7);

     The keyring can be manipulated after it has been given to the socket. This
     permits the server to add more keys, replace keys, etc. while it is live.

 (3) A local address must then be bound::

	struct sockaddr_rxrpc srx = {
		.srx_family	= AF_RXRPC,
		.srx_service	= VL_SERVICE_ID, /* RxRPC service ID */
		.transport_type	= SOCK_DGRAM,	/* type of transport socket */
		.transport.sin_family	= AF_INET,
		.transport.sin_port	= htons(7000), /* AFS callback */
		.transport.sin_address	= 0,  /* all local interfaces */
	};
	bind(server, &srx, sizeof(srx));

     More than one service ID may be bound to a socket, provided the transport
     parameters are the same. The limit is currently two. To do this, bind()
     should be called twice.

 (4) If service upgrading is required, first two service IDs must have been
     bound and then the following option must be set::

	unsigned short service_ids[2] = { from_ID, to_ID };
	setsockopt(server, SOL_RXRPC, RXRPC_UPGRADEABLE_SERVICE,
		   service_ids, sizeof(service_ids));

     This will automatically upgrade connections on service from_ID to service
     to_ID if they request it. This will be reflected in msg_name obtained
     through recvmsg() when the request data is delivered to userspace.

 (5) The server is then set to listen out for incoming calls::

	listen(server, 100);

 (6) The kernel notifies the server of pending incoming connections by sending
     it a message for each. This is received with recvmsg() on the server
     socket. It has no data, and has a single dataless control message
     attached::

	RXRPC_NEW_CALL

     The address that can be passed back by recvmsg() at this point should be
     ignored since the call for which the message was posted may have gone by
     the time it is accepted - in which case the first call still on the queue
     will be accepted.

 (7) The server then accepts the new call by issuing a sendmsg() with two
     pieces of control data and no actual data:

	==================	==============================
	RXRPC_ACCEPT		indicate connection acceptance
	RXRPC_USER_CALL_ID	specify user ID for this call
	==================	==============================

 (8) The first request data packet will then be posted to the server socket for
     recvmsg() to pick up. At that point, the RxRPC address for the call can
     be read from the address fields in the msghdr struct.

     Subsequent request data will be posted to the server socket for recvmsg()
     to collect as it arrives. All but the last piece of the request data will
     be delivered with MSG_MORE flagged.

     All data will be delivered with the following control message attached:


	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

 (9) The reply data should then be posted to the server socket using a series
     of sendmsg() calls, each with the following control messages attached:

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	==================	===================================

     MSG_MORE should be set in msghdr::msg_flags on all but the last message
     for a particular call.

```

(10) 客户端的最ACK 在被收到时将发布recvmsg() 获取。它将采取一个不带数据的消息的形式，并附     两个控制消息
	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	RXRPC_ACK		indicates final ACK (no data)
	==================	===================================

     MSG_EOR 会被标记以指示这是该调用的最终消息
(11) 直到应答数据的最后一个数据包被发送之前，该调用都可以通过调用带有一个不带数据的消息sendmsg()
     来中止，该消息附带以下控制消息：

	==================	===================================
	RXRPC_USER_CALL_ID	specifies the user ID for this call
	RXRPC_ABORT		indicates abort code (4 byte data)
	==================	===================================

     如果发出这个，任何在套接字接收队列中等待的数据包都将被丢弃
注意，某个特定服务的所有通信都通过那一个服务端套接字进行，使用 sendmsg() recvmsg() 上的控制
消息来确定受影响的调用

## AF_RXRPC 内核接口


AF_RXRPC 模块还为内核内实用程序（例如 AFS 文件系统）提供了一个接口。这允许这样的实用程序：

 (1) 在一个套接字上直接对各个客户端调用使用不同的密钥，而不必打开一大堆套接字、每个可能想用的
     密钥一个
 (2) 避免RxRPC 在发起调用或打开套接字的时刻调用 request_key()。而是由实用程序负责在适当     时刻请求密钥。例如，AFS 会在 open() unlink() VFS 操作期间这样做。然后在调用发起时把
     密钥传进去
 (3) 请求使用 GFP_KERNEL 之外的东西来分配内存
 (4) 避免 recvmsg() 调用的开销。RxRPC 消息可以在被放入套接Rx 队列之前被拦截，并直接操作套接字
     缓冲区
要使RxRPC 设施，一个内核实用程序仍然必须打开一AF_RXRPC 套接字，适当地绑定一个地址，并且如它是服务端套接字就监听，然后把它传给内核接口函数
内核接口函数如下
```
	struct rxrpc_call *
	rxrpc_kernel_begin_call(struct socket *sock,
				struct sockaddr_rxrpc *srx,
				struct key *key,
				unsigned long user_call_ID,
				s64 tx_total_len,
				gfp_t gfp,
				rxrpc_notify_rx_t notify_rx,
				bool upgrade,
				bool intr,
				unsigned int debug_id);

     This allocates the infrastructure to make a new RxRPC call and assigns
     call and connection numbers. The call will be made on the UDP port that
     the socket is bound to. The call will go to the destination address of a
     connected client socket unless an alternative is supplied (srx is
     non-NULL).

     If a key is supplied then this will be used to secure the call instead of
     the key bound to the socket with the RXRPC_SECURITY_KEY sockopt. Calls
     secured in this way will still share connections if at all possible.

     The user_call_ID is equivalent to that supplied to sendmsg() in the
     control data buffer. It is entirely feasible to use this to point to a
     kernel data structure.

     tx_total_len is the amount of data the caller is intending to transmit
     with this call (or -1 if unknown at this point). Setting the data size
     allows the kernel to encrypt directly to the packet buffers, thereby
     saving a copy. The value may not be less than -1.

     notify_rx is a pointer to a function to be called when events such as
     incoming data packets or remote aborts happen.

     upgrade should be set to true if a client operation should request that
     the server upgrade the service to a better one. The resultant service ID
     is returned by rxrpc_kernel_recv_data().

     intr should be set to true if the call should be interruptible. If this
     is not set, this function may not return until a channel has been
     allocated; if it is set, the function may return -ERESTARTSYS.

     debug_id is the call debugging ID to be used for tracing. This can be
     obtained by atomically incrementing rxrpc_debug_id.

     If this function is successful, an opaque reference to the RxRPC call is
     returned. The caller now holds a reference on this and it must be
     properly ended.

 (#) Shut down a client call::

	void rxrpc_kernel_shutdown_call(struct socket *sock,
					struct rxrpc_call *call);

     This is used to shut down a previously begun call. The user_call_ID is
     expunged from AF_RXRPC's knowledge and will not be seen again in
     association with the specified call.

 (#) Release the ref on a client call::

	void rxrpc_kernel_put_call(struct socket *sock,
				   struct rxrpc_call *call);

     This is used to release the caller's ref on an rxrpc call.

 (#) Send data through a call::

	typedef void (*rxrpc_notify_end_tx_t)(struct sock *sk,
					      unsigned long user_call_ID,
					      struct sk_buff *skb);

	int rxrpc_kernel_send_data(struct socket *sock,
				   struct rxrpc_call *call,
				   struct msghdr *msg,
				   size_t len,
				   rxrpc_notify_end_tx_t notify_end_rx);

     This is used to supply either the request part of a client call or the
     reply part of a server call. msg.msg_iovlen and msg.msg_iov specify the
     data buffers to be used. msg_iov may not be NULL and must point
     exclusively to in-kernel virtual addresses. msg.msg_flags may be given
     MSG_MORE if there will be subsequent data sends for this call.

     The msg must not specify a destination address, control data or any flags
     other than MSG_MORE. len is the total amount of data to transmit.

     notify_end_rx can be NULL or it can be used to specify a function to be
     called when the call changes state to end the Tx phase. This function is
     called with a spinlock held to prevent the last DATA packet from being
     transmitted until the function returns.

 (#) Receive data from a call::

	int rxrpc_kernel_recv_data(struct socket *sock,
				   struct rxrpc_call *call,
				   void *buf,
				   size_t size,
				   size_t *_offset,
				   bool want_more,
				   u32 *_abort,
				   u16 *_service)

      This is used to receive data from either the reply part of a client call
      or the request part of a service call. buf and size specify how much
      data is desired and where to store it. *_offset is added on to buf and
      subtracted from size internally; the amount copied into the buffer is
      added to *_offset before returning.

      want_more should be true if further data will be required after this is
      satisfied and false if this is the last item of the receive phase.

      There are three normal returns: 0 if the buffer was filled and want_more
      was true; 1 if the buffer was filled, the last DATA packet has been
      emptied and want_more was false; and -EAGAIN if the function needs to be
      called again.

      If the last DATA packet is processed but the buffer contains less than
      the amount requested, EBADMSG is returned. If want_more wasn't set, but
      more data was available, EMSGSIZE is returned.

      If a remote ABORT is detected, the abort code received will be stored in
      ``*_abort`` and ECONNABORTED will be returned.

      The service ID that the call ended up with is returned into *_service.
      This can be used to see if a call got a service upgrade.

 (#) Abort a call??

     ::

	void rxrpc_kernel_abort_call(struct socket *sock,
				     struct rxrpc_call *call,
				     u32 abort_code);

     This is used to abort a call if it's still in an abortable state. The
     abort code specified will be placed in the ABORT message sent.

 (#) Intercept received RxRPC messages::

	typedef void (*rxrpc_interceptor_t)(struct sock *sk,
					    unsigned long user_call_ID,
					    struct sk_buff *skb);

	void
	rxrpc_kernel_intercept_rx_messages(struct socket *sock,
					   rxrpc_interceptor_t interceptor);

     This installs an interceptor function on the specified AF_RXRPC socket.
     All messages that would otherwise wind up in the socket's Rx queue are
     then diverted to this function. Note that care must be taken to process
     the messages in the right order to maintain DATA message sequentiality.

     The interceptor function itself is provided with the address of the socket
     and handling the incoming message, the ID assigned by the kernel utility
     to the call and the socket buffer containing the message.

     The skb->mark field indicates the type of message:

	===============================	=======================================
	Mark				Meaning
	===============================	=======================================
	RXRPC_SKB_MARK_DATA		Data message
	RXRPC_SKB_MARK_FINAL_ACK	Final ACK received for an incoming call
	RXRPC_SKB_MARK_BUSY		Client call rejected as server busy
	RXRPC_SKB_MARK_REMOTE_ABORT	Call aborted by peer
	RXRPC_SKB_MARK_NET_ERROR	Network error detected
	RXRPC_SKB_MARK_LOCAL_ERROR	Local error encountered
	RXRPC_SKB_MARK_NEW_CALL		New incoming call awaiting acceptance
	===============================	=======================================

     The remote abort message can be probed with rxrpc_kernel_get_abort_code().
     The two error messages can be probed with rxrpc_kernel_get_error_number().
     A new call can be accepted with rxrpc_kernel_accept_call().

     Data messages can have their contents extracted with the usual bunch of
     socket buffer manipulation functions. A data message can be determined to
     be the last one in a sequence with rxrpc_kernel_is_data_last(). When a
     data message has been used up, rxrpc_kernel_data_consumed() should be
     called on it.

     Messages should be handled to rxrpc_kernel_free_skb() to dispose of. It
     is possible to get extra refs on all types of message for later freeing,
     but this may pin the state of a call until the message is finally freed.

 (#) Accept an incoming call::

	struct rxrpc_call *
	rxrpc_kernel_accept_call(struct socket *sock,
				 unsigned long user_call_ID);

     This is used to accept an incoming call and to assign it a call ID. This
     function is similar to rxrpc_kernel_begin_call() and calls accepted must
     be ended in the same way.

     If this function is successful, an opaque reference to the RxRPC call is
     returned. The caller now holds a reference on this and it must be
     properly ended.

 (#) Reject an incoming call::

	int rxrpc_kernel_reject_call(struct socket *sock);

     This is used to reject the first incoming call on the socket's queue with
     a BUSY message. -ENODATA is returned if there were no incoming calls.
     Other errors may be returned if the call had been aborted (-ECONNABORTED)
     or had timed out (-ETIME).

 (#) Allocate a null key for doing anonymous security::

	struct key *rxrpc_get_null_key(const char *keyname);

     This is used to allocate a null RxRPC key that can be used to indicate
     anonymous security for a particular domain.

 (#) Get the peer address of a call::

	void rxrpc_kernel_get_peer(struct socket *sock, struct rxrpc_call *call,
				   struct sockaddr_rxrpc *_srx);

     This is used to find the remote peer address of a call.

 (#) Set the total transmit data size on a call::

	void rxrpc_kernel_set_tx_length(struct socket *sock,
					struct rxrpc_call *call,
					s64 tx_total_len);

     This sets the amount of data that the caller is intending to transmit on a
     call. It's intended to be used for setting the reply size as the request
     size should be set when the call is begun. tx_total_len may not be less
     than zero.

 (#) Get call RTT::

	u64 rxrpc_kernel_get_rtt(struct socket *sock, struct rxrpc_call *call);

     Get the RTT time to the peer in use by a call. The value returned is in
     nanoseconds.

 (#) Check call still alive::

	bool rxrpc_kernel_check_life(struct socket *sock,
				     struct rxrpc_call *call,
				     u32 *_life);
	void rxrpc_kernel_probe_life(struct socket *sock,
				     struct rxrpc_call *call);

     The first function passes back in ``*_life`` a number that is updated when
     ACKs are received from the peer (notably including PING RESPONSE ACKs
     which we can elicit by sending PING ACKs to see if the call still exists
     on the server). The caller should compare the numbers of two calls to see
     if the call is still alive after waiting for a suitable interval. It also
     returns true as long as the call hasn't yet reached the completed state.

     This allows the caller to work out if the server is still contactable and
     if the call is still alive on the server while waiting for the server to
     process a client operation.

     The second function causes a ping ACK to be transmitted to try to provoke
     the peer into responding, which would then cause the value returned by the
     first function to change. Note that this must be called in TASK_RUNNING
     state.

 (#) Apply the RXRPC_MIN_SECURITY_LEVEL sockopt to a socket from within in the
     kernel::

       int rxrpc_sock_set_min_security_level(struct sock *sk,
					     unsigned int val);

     This specifies the minimum security level required for calls on this
     socket.
```

## 可配置参

RxRPC 协议驱动有一组可配置参数，可以通过 /proc/net/rxrpc/ 中的 sysctl 进行调整
 (#) req_ack_delay

     在收到一个设置了 request-ack 标志的数据包之后，到我们兑现该标志并实际发送所请求ack 之前     时间量（毫秒）
     通常对端在我们公布的接收窗口填满（最255 个数据包）之前不会停止发送数据包，因此延ACK 允许
     一次性对多个数据包进ACK
 (#) soft_ack_delay

     在收到一个新数据包之后，到我们生成一soft-ACK 来告诉发送方它无需重发之前的时间量（毫秒）
 (#) idle_ack_delay

     在当前接收队列中的所有数据包都已被消费之后，到我们生成一hard-ACK 来告诉发送方它可以释放其
     缓冲区之前的时间量（毫秒），前提是没有任何其他会让我们发ACK 的理由出现
 (#) resend_timeout

     在传输一个数据包之后，到我们在假设没有收到来自接收方的、告知其已收到的 ACK 之前重新传输它之前的
     时间量（毫秒）
 (#) max_call_lifetime

     一个调用在我们可以主动杀死它之前可以处于进行状态的最大时间量（秒）
 (#) dead_call_expiry

     在我们从调用列表中移除一个死调用之前的时间量（秒）。死调用会被保留一小段时间，以便重复发ACK
     ABORT 数据包
 (#) connection_expiry

     在一个连接最后一次被使用之后，到我们把它从连接列表中移除之前的时间量（秒）。在一个连接存在期间，
     它充当已协商安全的占位符；当它被删除时，安全必须重新协商
 (#) transport_expiry

     在一个传输最后一次被使用之后，到我们把它从传输列表中移除之前的时间量（秒）。在一个传输存在期间，
     它用于锚定对端数据并保持连接 ID 计数器
 (#) rxrpc_rx_window_size

     以数据包为单位的接收窗口大小。这是我们愿意为任何特定调用在内存中保留的未消费接收数据包的最大数量
 (#) rxrpc_rx_mtu

     我们愿意接收的最大数据包 MTU 大小（字节）。这向对端指示我们是否愿意接受巨型（jumbo）数据包
 (#) rxrpc_rx_jumbo_max

     我们愿意在一个巨型数据包中接受的包的最大数量。巨型数据包中的非终端数据包必须包含一个四字节     头部加上正好 1412 字节的数据。终端数据包必须包含一个四字节的头部加上任意数量的数据。无论如何，
     一个巨型数据包的大小不得超rxrpc_rx_mtu

## API 函数参