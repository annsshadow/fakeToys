
## L2TP


第二层隧道协议（L2TP）允许将 L2 帧通过 IP 网络进行隧道传输。

本文档涵盖内核的 L2TP 子系统。它为希望使用 L2TP 子系统的应用程序开发者记录内核 API，并提供一些关于内部实现的技术细节，这些可能对内核开发者和维护者有用。

## 概述


内核的 L2TP 子系统实现了 L2TPv2 和 L2TPv3 的数据路径。L2TPv2 承载于 UDP 之上。L2TPv3 承载于 UDP 之上或直接承载于 IP（协议 115）之上。

L2TP 的 RFC 定义了两种基本类型的 L2TP 数据包：控制数据包（"控制平面"）和数据包（"数据平面"）。内核只处理数据包。更复杂的控制数据包由用户空间处理。

一个 L2TP 隧道承载一个或多个 L2TP 会话。每个隧道关联一个套接字。每个 session 关联一个虚拟网络设备，例如 `pppN`、`l2tpethN`，数据帧通过它在 L2TP 之间传入/传出。L2TP 头中的字段标识隧道或 session，以及它是控制包还是数据包。当使用 Linux 内核 API 建立隧道和 session 时，我们只是在建立 L2TP 数据路径。控制协议的所有方面都由用户空间处理。

这种职责划分导致在建立隧道和 session 时有一个自然的操作序列。过程如下：

    1) 创建一个隧道套接字。通过该套接字与对端交换 L2TP 控制协议消息，以建立隧道。

    2) 使用通过控制协议消息从对端获得的信息，在内核中创建隧道上下文。

    3) 通过隧道套接字与对端交换 L2TP 控制协议消息，以建立 session。

    4) 使用通过控制协议消息从对端获得的信息，在内核中创建 session 上下文。

## L2TP API


本节记录 L2TP 子系统的每个用户空间 API。

### 隧道套接字


L2TPv2 始终使用 UDP。L2TPv3 可以使用 UDP 或 IP 封装。

要创建供 L2TP 使用的隧道套接字，使用标准 POSIX 套接字 API。

```

    int sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);

```
```

    int sockfd = socket(AF_INET6, SOCK_DGRAM, IPPROTO_L2TP);

```
UDP 套接字编程无需在此赘述。

IPPROTO_L2TP 是内核 L2TP 子系统实现的一种 IP 协议类型。L2TPIP 套接字地址定义于 struct
sockaddr_l2tpip 和 struct sockaddr_l2tpip6，位于
`include/uapi/linux/l2tp.h`_。该地址包含 L2TP 隧道（连接）id。要使用 L2TP IP 封装，L2TPv3 应用程序应使用本地分配的隧道 id 绑定 L2TPIP 套接字。当已知对端的隧道 id 和 IP 地址时，必须执行 connect。

如果 L2TP 应用程序需要处理来自使用 L2TPIP 的对端的 L2TPv3 隧道建立请求，它必须打开一个专用的 L2TPIP 套接字来监听这些请求，并使用隧道 id 0 绑定该套接字，因为隧道建立请求是寻址到隧道 id 0 的。

当隧道套接字关闭时，L2TP 隧道及其所有 session 会自动关闭。

### Netlink API


L2TP 应用程序使用 netlink 管理内核中的 L2TP 隧道和 session 实例。L2TP netlink API 定义于
`include/uapi/linux/l2tp.h`_。

L2TP 使用 `Generic Netlink`_（GENL）。定义了若干命令：
Create、Delete、Modify 和 Get，用于隧道和 session 实例，例如 `L2TP_CMD_TUNNEL_CREATE`。API 头列出了可与每个命令一起使用的 netlink 属性类型。

隧道和 session 实例由本地唯一的 32 位 id 标识。L2TP 隧道 id 由 `L2TP_ATTR_CONN_ID` 和
`L2TP_ATTR_PEER_CONN_ID` 属性给出，L2TP session id 由
`L2TP_ATTR_SESSION_ID` 和 `L2TP_ATTR_PEER_SESSION_ID`
属性给出。如果使用 netlink 管理 L2TPv2 隧道和 session 实例，L2TPv2 的 16 位隧道/session id 在这些属性中被强制转换为 32 位值。

在 `L2TP_CMD_TUNNEL_CREATE` 命令中，`L2TP_ATTR_FD` 告诉内核正在使用的隧道套接字 fd。如果未指定，内核使用在
`L2TP_ATTR_IP[^6^]_SADDR`、`L2TP_ATTR_IP[^6^]_DADDR`、
`L2TP_ATTR_UDP_SPORT`、`L2TP_ATTR_UDP_DPORT` 属性中设置的 IP 参数，为隧道创建一个内核套接字。内核套接字用于实现非托管的 L2TPv3 隧道（iproute2 的 "ip l2tp" 命令）。如果给出了 `L2TP_ATTR_FD`，它必须是已经绑定并连接的套接字 fd。本文档后面有更多关于非托管隧道的信息。

`L2TP_CMD_TUNNEL_CREATE` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        设置隧道（连接）id。
PEER_CONN_ID       Y        设置对端隧道（连接）id。
PROTO_VERSION      Y        协议版本。2 或 3。
ENCAP_TYPE         Y        封装类型：UDP 或 IP。
FD                 N        隧道套接字文件描述符。
UDP_CSUM           N        启用 IPv4 UDP 校验和。仅当未设置 FD 时使用。
UDP_ZERO_CSUM6_TX  N        发送时将 IPv6 UDP 校验和置零。仅当未设置 FD 时使用。
UDP_ZERO_CSUM6_RX  N        接收时将 IPv6 UDP 校验和置零。仅当未设置 FD 时使用。
IP_SADDR           N        IPv4 源地址。仅当未设置 FD 时使用。
IP_DADDR           N        IPv4 目的地址。仅当未设置 FD 时使用。
UDP_SPORT          N        UDP 源端口。仅当未设置 FD 时使用。
UDP_DPORT          N        UDP 目的端口。仅当未设置 FD 时使用。
IP6_SADDR          N        IPv6 源地址。仅当未设置 FD 时使用。
IP6_DADDR          N        IPv6 目的地址。仅当未设置 FD 时使用。
DEBUG              N        调试标志。
================== ======== ===

`L2TP_CMD_TUNNEL_DESTROY` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        标识要销毁的隧道 id。
================== ======== ===

`L2TP_CMD_TUNNEL_MODIFY` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        标识要修改的隧道 id。
DEBUG              N        调试标志。
================== ======== ===

`L2TP_CMD_TUNNEL_GET` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            N        标识要查询的隧道 id。
                            在 DUMP 请求中忽略。
================== ======== ===

`L2TP_CMD_SESSION_CREATE` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        父隧道 id。
SESSION_ID         Y        设置 session id。
PEER_SESSION_ID    Y        设置父 session id。
PW_TYPE            Y        设置伪线类型。
DEBUG              N        调试标志。
RECV_SEQ           N        启用 rx 数据序列号。
SEND_SEQ           N        启用 tx 数据序列号。
LNS_MODE           N        启用 LNS 模式（自动启用数据序列号）。
RECV_TIMEOUT       N        重排序接收数据包时的等待超时。
L2SPEC_TYPE        N        设置 layer2-specific-sublayer 类型（仅 L2TPv3）。
COOKIE             N        设置可选 cookie（仅 L2TPv3）。
PEER_COOKIE        N        设置可选对端 cookie（仅 L2TPv3）。
IFNAME             N        设置接口名称（仅 L2TPv3）。
================== ======== ===

对于以太网 session 类型，这将创建一个 l2tpeth 虚拟接口，随后可按需配置。对于 PPP session 类型，还必须打开并连接一个 PPPoL2TP 套接字，将其映射到新 session。这在后面的"PPPoL2TP 套接字"中介绍。

`L2TP_CMD_SESSION_DESTROY` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        标识要销毁的 session 的父隧道 id。
SESSION_ID         Y        标识要销毁的 session id。
IFNAME             N        通过接口名称标识 session。如果设置，将覆盖任何 CONN_ID 和 SESSION_ID 属性。目前仅支持 L2TPv3 以太网 session。
================== ======== ===

`L2TP_CMD_SESSION_MODIFY` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        标识要修改的 session 的父隧道 id。
SESSION_ID         Y        标识要修改的 session id。
IFNAME             N        通过接口名称标识 session。如果设置，将覆盖任何 CONN_ID 和 SESSION_ID 属性。目前仅支持 L2TPv3 以太网 session。
DEBUG              N        调试标志。
RECV_SEQ           N        启用 rx 数据序列号。
SEND_SEQ           N        启用 tx 数据序列号。
LNS_MODE           N        启用 LNS 模式（自动启用数据序列号）。
RECV_TIMEOUT       N        重排序接收数据包时的等待超时。
================== ======== ===

`L2TP_CMD_SESSION_GET` 属性：-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            N        标识要查询的隧道 id。
                            对于 DUMP 请求忽略。
SESSION_ID         N        标识要查询的 session id。
                            对于 DUMP 请求忽略。
IFNAME             N        通过接口名称标识 session。
                            如果设置，将覆盖任何 CONN_ID 和
                            SESSION_ID 属性。对于 DUMP 请求忽略。目前仅支持 L2TPv3
                            以太网 session。
================== ======== ===

应用程序开发者应参考 `include/uapi/linux/l2tp.h`_ 获取 netlink 命令和属性定义。

使用 libmnl_ 的示例用户空间代码：

```

        struct nl_sock *nl_sock;
        int l2tp_nl_family_id;

        nl_sock = nl_socket_alloc();
        genl_connect(nl_sock);
        genl_id = genl_ctrl_resolve(nl_sock, L2TP_GENL_NAME);

  - 创建一个隧道::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_TUNNEL_CREATE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_FD, tunl_sock_fd);
        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_CONN_ID, peer_tid);
        mnl_attr_put_u8(nlh, L2TP_ATTR_PROTO_VERSION, protocol_version);
        mnl_attr_put_u16(nlh, L2TP_ATTR_ENCAP_TYPE, encap);

  - 创建一个 session::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_SESSION_CREATE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_CONN_ID, peer_tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_SESSION_ID, sid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_SESSION_ID, peer_sid);
        mnl_attr_put_u16(nlh, L2TP_ATTR_PW_TYPE, pwtype);
        /* there are other session options which can be set using netlink
         * attributes during session creation -- see l2tp.h
         */

  - 删除一个 session::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_SESSION_DELETE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_SESSION_ID, sid);

  - 删除一个隧道及其所有 session（如果有）::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_TUNNEL_DELETE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);

```
### PPPoL2TP Session 套接字 API


对于 PPP session 类型，必须打开一个 PPPoL2TP 套接字并连接到 L2TP session。

创建 PPPoL2TP 套接字时，应用程序在套接字 connect() 调用中向内核提供关于隧道和 session 的信息。提供源和目的隧道及 session id，以及 UDP 或 L2TPIP 套接字的文件描述符。参见 struct
pppol2tp_addr，位于 `include/linux/if_pppol2tp.h`_。出于历史原因，L2TPv2/L2TPv3 IPv4/IPv6 隧道不幸有略微不同的地址结构，用户空间必须使用与隧道套接字类型相匹配的适当结构。

用户空间可以使用 PPPoX 套接字上的 setsockopt 和 ioctl 控制隧道或 session 的行为。支持以下套接字选项：-

=========   ===========================================================
DEBUG       调试消息类别。见下文。
SENDSEQ     - 0 => 不发送带序列号的数据包
            - 1 => 发送带序列号的数据包
RECVSEQ     - 0 => 接收数据包的序列号为可选
            - 1 => 丢弃不带序列号的接收数据包
LNSMODE     - 0 => 充当 LAC。
            - 1 => 充当 LNS。
REORDERTO   重排序超时（毫秒）。若为 0，则不尝试重排序。
=========   ===========================================================

除了标准 PPP ioctls 外，还提供了 PPPIOCGL2TPSTATS，用于使用相应隧道或 session 的 PPPoX 套接字从内核检索隧道和 session 统计信息。

示例用户空间代码：

```

        /* Input: the L2TP tunnel UDP socket `tunnel_fd`, which needs to be
         * bound already (both sockname and peername), otherwise it will not be
         * ready.
         */

        struct sockaddr_pppol2tp sax;
        int session_fd;
        int ret;

        session_fd = socket(AF_PPPOX, SOCK_DGRAM, PX_PROTO_OL2TP);
        if (session_fd < 0)
                return -errno;

        sax.sa_family = AF_PPPOX;
        sax.sa_protocol = PX_PROTO_OL2TP;
        sax.pppol2tp.fd = tunnel_fd;
        sax.pppol2tp.addr.sin_addr.s_addr = addr->sin_addr.s_addr;
        sax.pppol2tp.addr.sin_port = addr->sin_port;
        sax.pppol2tp.addr.sin_family = AF_INET;
        sax.pppol2tp.s_tunnel  = tunnel_id;
        sax.pppol2tp.s_session = session_id;
        sax.pppol2tp.d_tunnel  = peer_tunnel_id;
        sax.pppol2tp.d_session = peer_session_id;

        /* session_fd is the fd of the session's PPPoL2TP socket.
         * tunnel_fd is the fd of the tunnel UDP / L2TPIP socket.
         */
        ret = connect(session_fd, (struct sockaddr *)&sax, sizeof(sax));
        if (ret < 0 ) {
                close(session_fd);
                return -errno;
        }

        return session_fd;

```
L2TP 控制数据包在 `tunnel_fd` 上仍然可读。

```

        /* Input: the session PPPoX data socket `session_fd` which was created
         * as described above.
         */

        int ppp_chan_fd;
        int chindx;
        int ret;

        ret = ioctl(session_fd, PPPIOCGCHAN, &chindx);
        if (ret < 0)
                return -errno;

        ppp_chan_fd = open("/dev/ppp", O_RDWR);
        if (ppp_chan_fd < 0)
                return -errno;

        ret = ioctl(ppp_chan_fd, PPPIOCATTCHAN, &chindx);
        if (ret < 0) {
                close(ppp_chan_fd);
                return -errno;
        }

        return ppp_chan_fd;

```
LCP PPP 帧在 `ppp_chan_fd` 上可读。

```

        /* Input: the PPP channel `ppp_chan_fd` which was created as described
         * above.
         */

        int ifunit = -1;
        int ppp_if_fd;
        int ret;

        ppp_if_fd = open("/dev/ppp", O_RDWR);
        if (ppp_if_fd < 0)
                return -errno;

        ret = ioctl(ppp_if_fd, PPPIOCNEWUNIT, &ifunit);
        if (ret < 0) {
                close(ppp_if_fd);
                return -errno;
        }

        ret = ioctl(ppp_chan_fd, PPPIOCCONNECT, &ifunit);
        if (ret < 0) {
                close(ppp_if_fd);
                return -errno;
        }

        return ppp_if_fd;

```
IPCP/IPv6CP PPP 帧在 `ppp_if_fd` 上可读。

ppp<ifunit> 接口随后可以使用 netlink 的 RTM_NEWLINK、RTM_NEWADDR、RTM_NEWROUTE，或 ioctl 的 SIOCSIFMTU、SIOCSIFADDR、SIOCSIFDSTADDR、SIOCSIFNETMASK、SIOCSIFFLAGS，或使用 `ip` 命令进行常规配置。

  - 桥接具有 PPP 伪线类型的 L2TP session（这也称为
    L2TP 隧道交换或 L2TP 多跳）通过桥接 PPP
```

        /* Input: the session PPPoX data sockets `session_fd1` and `session_fd2`
         * which were created as described further above.
         */

        int ppp_chan_fd;
        int chindx1;
        int chindx2;
        int ret;

        ret = ioctl(session_fd1, PPPIOCGCHAN, &chindx1);
        if (ret < 0)
                return -errno;

        ret = ioctl(session_fd2, PPPIOCGCHAN, &chindx2);
        if (ret < 0)
                return -errno;

        ppp_chan_fd = open("/dev/ppp", O_RDWR);
        if (ppp_chan_fd < 0)
                return -errno;

        ret = ioctl(ppp_chan_fd, PPPIOCATTCHAN, &chindx1);
        if (ret < 0) {
                close(ppp_chan_fd);
                return -errno;
        }

        ret = ioctl(ppp_chan_fd, PPPIOCBRIDGECHAN, &chindx2);
        close(ppp_chan_fd);
        if (ret < 0)
                return -errno;

        return 0;

```
可以看出，桥接 PPP 通道时，PPP session 不在本地终结，也不会创建本地 PPP 接口。到达一个通道的 PPP 帧直接传递给另一个通道，反之亦然。

PPP 通道不需要保持打开。只需保持 session 的 PPPoX 数据套接字打开。

更一般地说，也可以以相同方式桥接例如 PPPoL2TP PPP 通道与其他类型的 PPP 通道，例如 PPPoE。

PPP 侧的更多细节参见 ppp_generic.rst。

### 旧版仅 L2TPv2 API


当 L2TP 在 2.6.23 中首次加入 Linux 内核时，它只实现了 L2TPv2，且不包含 netlink API。相反，内核中的隧道和 session 实例直接使用 PPPoL2TP 套接字管理。PPPoL2TP 套接字的使用如"PPPoL2TP Session 套接字 API"一节所述，但隧道和 session 实例是在套接字 connect() 时自动创建，而不是通过单独的 netlink 请求创建：

    - 隧道使用隧道管理套接字管理，这是一个专用的 PPPoL2TP 套接字，连接到（无效的）session id 0。当 PPPoL2TP 隧道管理套接字连接时创建 L2TP 隧道实例，并在套接字关闭时销毁。

    - 当 PPPoL2TP 套接字连接到非零 session id 时，在内核中创建 session 实例。session 参数使用 setsockopt 设置。当套接字关闭时销毁 L2TP session 实例。

此 API 仍受支持，但不鼓励使用。相反，新的 L2TPv2 应用程序应首先使用 netlink 创建隧道和 session，然后为 session 创建 PPPoL2TP 套接字。

### 非托管 L2TPv3 隧道


内核 L2TP 子系统还支持静态（非托管）L2TPv3 隧道。非托管隧道没有用户空间隧道套接字，且与对端不交换控制消息来建立隧道；隧道在隧道两端手动配置。所有配置都使用 netlink 完成。这种情况下不需要 L2TP 用户空间应用程序——隧道套接字由内核创建，并使用在 `L2TP_CMD_TUNNEL_CREATE` netlink 请求中发送的参数配置。`iproute2` 的 `ip` 工具具有管理静态 L2TPv3 隧道命令；执行 ``ip l2tp help`` 了解更多信息。

### 调试


L2TP 子系统通过 debugfs 文件系统提供一系列调试接口。

```

    # mount -t debugfs debugfs /debug

```
随后可以访问 l2tp 目录下的文件，提供当前隧道和 session 上下文存在情况的概要
```

    # cat /debug/l2tp/tunnels

```
调试文件系统文件不应被应用程序用于获取 L2TP 状态信息，因为文件格式可能会更改。它实现用于提供额外的调试信息以帮助诊断问题。应用程序应改用 netlink API。

此外，L2TP 子系统使用标准内核事件跟踪 API 实现跟踪点。可用的 L2TP 事件可查看为
```

    # find /debug/tracing/events/l2tp

```
最后，/proc/net/pppol2tp 也提供，用于与原始 pppol2tp 代码向后兼容。它只列出关于 L2TPv2 隧道和 session 的信息。不鼓励使用它。

## 内部实现


本节面向内核开发者和维护者。

### 套接字


UDP 套接字由网络核心实现。当使用 UDP 套接字创建 L2TP 隧道时，通过在 UDP 套接字上设置 encap_rcv 和 encap_destroy 回调，将该套接字设置为封装的 UDP 套接字。接收到该套接字上的数据包时调用 l2tp_udp_encap_recv。用户空间关闭套接字时调用 l2tp_udp_encap_destroy。

L2TPIP 套接字实现于 `net/l2tp/l2tp_ip.c`_ 和
`net/l2tp/l2tp_ip6.c`_。

### 隧道


内核为每个 L2TP 隧道保留一个 struct l2tp_tunnel 上下文。l2tp_tunnel 始终与一个 UDP 或 L2TP/IP 套接字关联，并保留隧道中的 session 列表。当隧道首次向 L2TP 核心注册时，套接字上的引用计数增加。这确保在其数据结构引用该套接字时，套接字不会被移除。

隧道由唯一的隧道 id 标识。该 id 在 L2TPv2 中为 16 位，在 L2TPv3 中为 32 位。内部以 32 位值存储。

隧道保存在按网络（per-net）的列表中，由隧道 id 索引。隧道 id 命名空间由 L2TPv2 和 L2TPv3 共享。

处理隧道套接字关闭也许是 L2TP 实现中最棘手的部分。如果用户空间关闭隧道套接字，L2TP 隧道及其所有 session 必须关闭并销毁。由于隧道上下文持有对隧道套接字的引用，在隧道 sock_put 其套接字之前，不会调用套接字的 sk_destruct。对于 UDP 套接字，当用户空间关闭隧道套接字时，会调用套接字的 encap_destroy 处理程序，L2TP 用它来启动隧道关闭动作。对于 L2TPIP 套接字，套接字的 close 处理程序启动相同的隧道关闭动作。首先关闭所有 session。每个 session 丢弃其对隧道的引用。当隧道引用归零时，隧道丢弃其对套接字的引用。

### Session


内核为每个 session 保留一个 struct l2tp_session 上下文。每个 session 都有私有数据，用于特定于 session 类型的数据。在 L2TPv2 中，session 总是承载 PPP 流量。在 L2TPv3 中，session 可以承载以太网帧（以太网伪线）或其他数据类型，如 PPP、ATM、HDLC 或帧中继。Linux 目前仅实现了以太网和 PPP session 类型。

某些 L2TP session 类型还有一个套接字（PPP 伪线），而其他则没有（以太网伪线）。

与隧道类似，L2TP session 由唯一的 session id 标识。与隧道 id 一样，session id 在 L2TPv2 中为 16 位，在 L2TPv3 中为 32 位。内部以 32 位值存储。

Session 持有对其父隧道的引用，以确保在有一个或多个 session 引用隧道时隧道仍然存在。

Session 保存在按网络（per-net）的列表中。L2TPv2 session 和 L2TPv3 session 存储在单独的列表中。L2TPv2 session 由 16 位隧道 ID 和 16 位 session ID 组成的 32 位键索引。L2TPv3 session 由 32 位 session ID 索引，因为 L2TPv3 session id 在所有隧道中唯一。

尽管 L2TPv3 RFC 规定 L2TPv3 session id 不受隧道限制，但 Linux 实现历来允许如此。这种 session id 冲突使用以 sk 和 session ID 为键的按网络（per-net）哈希表来支持。查找 L2TPv3 session 时，列表项可能链接到多个具有该 session ID 的 session，此时使用匹配给定 sk（隧道）的 session。

### PPP


`net/l2tp/l2tp_ppp.c`_ 实现了 PPPoL2TP 套接字族。每个 PPP session 都有一个 PPPoL2TP 套接字。

PPPoL2TP 套接字的 sk_user_data 引用 l2tp_session。

用户空间通过 PPPoL2TP 套接字发送和接收 PPP 数据包。只有 PPP 控制帧通过此套接字：PPP 数据包完全由内核处理，在内核 PPP 子系统的 PPP 通道接口之间，在 L2TP session 及其关联的 `pppN` 网络设备之间传递。

L2TP PPP 实现通过关闭其相应的 L2TP session 来处理 PPPoL2TP 套接字的关闭。这很复杂，因为它必须考虑与 netlink session 创建/销毁请求以及 pppol2tp_connect 尝试重新连接到正在关闭过程中的 session 的竞争。PPP session 持有对其关联套接字的引用，以便在 session 引用它时套接字仍然存在。

### 以太网


`net/l2tp/l2tp_eth.c`_ 实现 L2TPv3 以太网伪线。它为每个 session 管理一个 netdev。

L2TP 以太网 session 由 netlink 请求创建和销毁，或在隧道销毁时销毁。与 PPP session 不同，以太网 session 没有关联的套接字。

## 杂项


### RFCs


内核代码实现了以下 RFC 中规定的数据路径特性：

======= =============== ===================================
RFC2661 L2TPv2          https://tools.ietf.org/html/rfc2661
RFC3931 L2TPv3          https://tools.ietf.org/html/rfc3931
RFC4719 L2TPv3 Ethernet https://tools.ietf.org/html/rfc4719
======= =============== ===================================

### 实现


若干开源应用程序使用 L2TP 内核子系统：

============ ==============================================
iproute2     https://github.com/shemminger/iproute2
go-l2tp      https://github.com/katalix/go-l2tp
tunneldigger https://github.com/wlanslovenija/tunneldigger
xl2tpd       https://github.com/xelerance/xl2tpd
============ ==============================================

### 限制


当前实现有一些限制：

  1) 与 openvswitch 的接口尚未实现。将 OVS 以太网和 VLAN 端口映射到 L2TPv3 隧道可能有用。

  2) VLAN 伪线使用配置了 VLAN 子接口的 `l2tpethN` 接口实现。由于 L2TPv3 VLAN 伪线承载且仅承载一个 VLAN，使用单一 netdev 而非每个 VLAN session 使用 `l2tpethN` 和 `l2tpethN`:M 对可能更好。为此添加了 netlink 属性 `L2TP_ATTR_VLAN_ID`，但它从未被实现。

### 测试


非托管 L2TPv3 以太网特性由内核内置的自测试测试。参见 `tools/testing/selftests/net/l2tp.sh`_。

另一个测试套件 l2tp-ktest_ 覆盖了所有 L2TP API 和隧道/session 类型。未来可能会集成到内核内置的 L2TP 自测试中。
