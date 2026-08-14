
## 管理组件传输协议（Management Component Transport Protocol，MCTP）


net/mctp/ 包含了 MCTP 的协议支持，其定义见 DMTF 标准 DSP0236。物理接口驱动（规范中称为“bindings”）由 drivers/net/mctp/ 提供。

核心代码通过 AF_MCTP、SOCK_DGRAM 套接字提供了一个基于套接字的接口，用于发送和接收 MCTP 消息。

## 结构：接口与网络


内核通过两个要素为本地 MCTP 拓扑建模：接口（interface）和网络（network）。

接口（或称“link”）是 MCTP 物理传输绑定（由 DSP0236 第 3.2.47 节定义）的一个实例，可能连接到某个特定的硬件设备。它表示为一个 `struct netdevice`。

网络通过端点 ID（endpoint-ID，由 DSP0236 第 3.2.31 节描述）为 MCTP 端点定义了一个唯一的地址空间。网络有一个对用户可见的标识符，以便从用户空间引用。路由定义特定于某一个网络。

接口与某一个网络关联。一个网络可以与一个或多个接口关联。

如果存在多个网络，每个网络都可能包含也出现在其他网络上的端点 ID（EID）。

## 套接字 API


### 协议定义


MCTP 使用 `AF_MCTP` / `PF_MCTP` 作为地址族和协议族。由于 MCTP 是基于消息的，因此仅支持 `SOCK_DGRAM` 套接字。

    int sd = socket(AF_MCTP, SOCK_DGRAM, 0);

`protocol` 参数当前唯一（且有效）的值是 0。

与所有套接字地址族一样，源地址和目的地址使用 `sockaddr` 类型指定，其中包含一个单字节的端点地址：

    typedef __u8		mctp_eid_t;

    struct mctp_addr {
            mctp_eid_t		s_addr;
    };

    struct sockaddr_mctp {
            __kernel_sa_family_t smctp_family;
            unsigned int         smctp_network;
            struct mctp_addr     smctp_addr;
            __u8                 smctp_type;
            __u8                 smctp_tag;
    };

    #define MCTP_NET_ANY	0x0
    #define MCTP_ADDR_ANY	0xff

### 系统调用行为


以下各节描述了标准套接字系统调用中与 MCTP 相关的行为。这些行为被选择为与现有的套接字 API 紧密对应。

##### ``bind()`` ：设置本地套接字地址


接收传入请求数据包的套接字将使用 `bind()` 系统调用来绑定到一个本地地址。

    struct sockaddr_mctp addr;

    addr.smctp_family = AF_MCTP;
    addr.smctp_network = MCTP_NET_ANY;
    addr.smctp_addr.s_addr = MCTP_ADDR_ANY;
    addr.smctp_type = MCTP_TYPE_PLDM;
    addr.smctp_tag = MCTP_TAG_OWNER;

    int rc = bind(sd, (struct sockaddr *)&addr, sizeof(addr));

这将建立套接字的本地地址。与此网络的网络号、地址和消息类型相匹配的传入 MCTP 消息将被该套接字接收。这里对“incoming”的引用很重要；绑定后的套接字只会接收设置了 TO 位的消息，表示这是一条传入的请求消息，而不是响应。

`smctp_tag` 的值将配置从该套接字远端接受的标签。根据以上说明，唯一有效的值是 `MCTP_TAG_OWNER`，这将使远端“拥有”的标签被路由到该套接字。由于设置了 `MCTP_TAG_OWNER`，`smctp_tag` 的最低 3 位不会被使用；调用者必须将它们设为零。

`smctp_network` 的值为 `MCTP_NET_ANY` 时，会将套接字配置为接收来自任何本地连接网络的传入数据包。指定某个网络值则会使套接字只接收来自该网络的传入消息。

`smctp_addr` 字段指定要绑定的本地地址。值为 `MCTP_ADDR_ANY` 时，将套接字配置为接收寻址到任何本地目的 EID 的消息。

`smctp_type` 字段指定要接收的消息类型。传入消息只匹配类型的低 7 位（即最高位的 IC 位不参与匹配）。这导致套接字既能接收带有消息完整性检查尾部的数据包，也能接收不带该尾部的数据包。

##### ``sendto()``、``sendmsg()``、``send()`` ：发送一条 MCTP 消息


一条 MCTP 消息可以使用 `sendto()`、`sendmsg()` 或 `send()` 系统调用之一发送。以 `sendto()` 作为主要示例：

    struct sockaddr_mctp addr;
    char buf[^14^];
    ssize_t len;

    /** set message destination **/
    addr.smctp_family = AF_MCTP;
    addr.smctp_network = 0;
    addr.smctp_addr.s_addr = 8;
    addr.smctp_tag = MCTP_TAG_OWNER;
    addr.smctp_type = MCTP_TYPE_ECHO;

    /** arbitrary message to send, with message-type header **/
    buf[^0^] = MCTP_TYPE_ECHO;
    memcpy(buf + 1, "hello, world!", sizeof(buf) - 1);

    len = sendto(sd, buf, sizeof(buf), 0,
                    (struct sockaddr_mctp *)&addr, sizeof(addr));

`addr` 的网络和地址字段定义了要发送到的远端地址。如果 `smctp_tag` 带有 `MCTP_TAG_OWNER`，内核将忽略 `MCTP_TAG_VALUE` 中设置的任何位，并为目的 EID 生成一个合适的标签值。如果未设置 `MCTP_TAG_OWNER`，消息将按指定的标签值发送。如果无法分配标签，系统调用将返回 `EAGAIN` 错误码。

应用程序必须将消息类型字节作为传入 `sendto()` 的消息缓冲区的第一个字节提供。如果要在发送的消息中包含消息完整性检查，也必须将其放在消息缓冲区中，并且消息类型字节的最高位必须为 1。

`sendmsg()` 系统调用允许更紧凑的参数接口，并允许将消息缓冲区指定为分散-聚集（scatter-gather）列表。目前没有定义任何辅助消息类型（用于传给 `sendmsg()` 的 `msg_control` 数据）。

在未连接的套接字上发送消息时若指定了 `MCTP_TAG_OWNER`，则如果尚未为该目的地址分配有效标签，将导致分配一个标签。（目的 EID，标签）元组充当隐式的本地套接字地址，使套接字能够接收此出站消息的响应。如果之前已经执行过分配（针对不同的远端 EID），则该分配将丢失。

套接字只会接收它们所发送请求（TO=1）的响应，并且只能对它们收到的请求（TO=0）作出响应。

##### ``recvfrom()``、``recvmsg()``、``recv()`` ：接收一条 MCTP 消息


应用程序可以使用 `recvfrom()`、`recvmsg()` 或 `recv()` 系统调用之一接收 MCTP 消息。以 `recvfrom()` 作为主要示例：

    struct sockaddr_mctp addr;
    socklen_t addrlen;
    char buf[^14^];
    ssize_t len;

    addrlen = sizeof(addr);

    len = recvfrom(sd, buf, sizeof(buf), 0,
                    (struct sockaddr_mctp *)&addr, &addrlen);

    /** We can expect addr to describe an MCTP address **/
    assert(addrlen >= sizeof(buf));
    assert(addr.smctp_family == AF_MCTP);

    printf("received %zd bytes from remote EID %d
", rc, addr.smctp_addr);

传给 `recvfrom` 和 `recvmsg` 的地址参数会被填入传入消息的远端地址，包括标签值（回复该消息时会需要它）。

消息缓冲区的第一个字节将包含消息类型字节。如果消息后面跟随有完整性检查，它也会被包含在接收到的缓冲区中。

`recv()` 系统调用的行为类似，但不会向应用程序提供远端地址。因此，只有在远端地址已经已知，或者消息不需要回复时，它才有用。

与发送调用一样，套接字只会接收它们所发送请求（TO=1）的响应，并且只能对它们收到的请求（TO=0）作出响应。

##### ``ioctl(SIOCMCTPALLOCTAG)`` 与 ``ioctl(SIOCMCTPDROPTAG)``


这些标签让应用程序能够对 MCTP 消息标签有更多的控制，方法是显式地分配（和释放）标签值，而不是由内核在 `sendmsg()` 时自动分配每消息标签。

一般来说，只有当您的 MCTP 协议不符合通常的请求/响应模型时，才需要使用这些 ioctl。例如，如果您需要在多个请求之间保持标签，或者一个请求可能产生多个响应。在这些情况下，ioctl 允许您将标签的分配（和释放）与单个消息的发送和接收操作解耦。

两个 ioctl 都传入一个指向 `struct mctp_ioc_tag_ctl` 的指针：

    struct mctp_ioc_tag_ctl {
        mctp_eid_t      peer_addr;
        __u8		tag;
        __u16   	flags;
    };

`SIOCMCTPALLOCTAG` 为一个特定的对等方分配一个标签，应用程序可以在将来的 `sendmsg()` 调用中使用它。应用程序用远端 EID 填充 `peer_addr` 成员。其他字段必须为零。

返回时，`tag` 成员将被填入已分配的标签值。已分配的标签将设置以下标签位：

 - `MCTP_TAG_OWNER`：只有当您是标签拥有者时，分配标签才有意义

 - `MCTP_TAG_PREALLOC`：用于向 `sendmsg()` 表明这是一个预分配的标签

 - ……以及实际的标签值，位于最低三位（`MCTP_TAG_MASK`）中。注意，零是一个有效的标签值。

该标签值应按原样用于 ``struct sockaddr_mctp`` 的 `smctp_tag` 成员。

`SIOCMCTPDROPTAG` 释放一个之前由 `SIOCMCTPALLOCTAG` ioctl 分配的标签。`peer_addr` 必须与分配时使用的相同，并且 `tag` 值必须精确匹配分配时返回的标签（包括 `MCTP_TAG_OWNER` 和 `MCTP_TAG_PREALLOC` 位）。`flags` 字段必须为零。

```

	sendmsg()
	 -> mctp_local_output()
	    : route lookup
	    -> rt->output() (== mctp_route_output)
	       -> dev_queue_xmit()

```
```

	sendmsg()
	-> mctp_local_output()
	    -> mctp_do_fragment_route()
	       : creates packet-sized skbs. For each new skb:
	       -> rt->output() (== mctp_route_output)
	          -> dev_queue_xmit()

```
```

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   -> sock_queue_rcv_skb()

```
```

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   : stores skb in struct sk_key->reasm_head

	mctp_pkttype_receive()
	: route lookup
	-> rt->output() (== mctp_route_input)
	   : sk_key lookup
	   : finds existing reassembly in sk_key->reasm_head
	   : appends new fragment
	   -> sock_queue_rcv_skb()

```
### 关键引用计数


 - 键的引用来自：

   - 一个 skb：在路由输出期间，存储在 `skb->cb` 中。

   - netns 和 sock 列表。

 - 键可以与一个设备关联，此时它们持有对该设备（dev）的引用（通过 `key->dev` 设置，通过 `dev->key_count` 计数）。多个键可以引用同一个设备。

