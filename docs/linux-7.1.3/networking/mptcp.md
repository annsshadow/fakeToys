
## 多路TCP（MPTCP


## 简


多路TCP（Multipath TCP，简MPTCP）是标准 TCP 的一个扩展，`RFC 8684 (MPTCPv1) <https://www.rfc-editor.org/rfc/rfc8684.html>`_ 中进行了描述。它允许设备同时利用多个接口，在单个 MPTCP 连接上发送和接收 TCP 数据包。MPTCP 可以聚合多个接口的带宽，或者优先选择延迟最低的那个。当某条路径中断时，它还允许故障转移，流量会无缝地重新注入其它路径

有关 Linux 内核中多路径 TCP 的更多细节，请参阅官方网站：`mptcp.dev <https://www.mptcp.dev>`_


## 使用场景


得益MPTCP，与 TCP 相比，能够并行或同时使用多条路径带来了新的使用场景：

- 无缝切换：在保持已建立连接的同时从一条路径切换到另一条路径，例如可用于智能手机等移动场景
- 最佳网络选择：根据某些条件（如延迟、丢包、成本、带宽等）使用“最佳”可用路径
- 网络聚合：同时使用多条路径以获得更高的吞吐量，例如组合固定网络与移动网络以更快地发送文件


## 概念


从技术上讲，当使`IPPROTO_MPTCP` 协议（Linux 特有）创建一个新套接字时，会创建一*子流（subflow*（或**路径**）。该**子流**由一个常规的 TCP 连接组成，用于通过一个接口传输数据。后续可以在主机之间协商建立额外*子流**。为了让远端主机能够检测到 MPTCP 的使用，到底TCP **子流**TCP **选项（option*字段中添加了一个新字段。除其它内容外，该字段包含一`MP_CAPABLE` 选项，告诉对端若其支持则使用 MPTCP。如果对端主机或中间的任何中间盒（middlebox）不支持，返回的 `SYN+ACK` 数据包的 TCP **选项**字段中将不包MPTCP 选项。在这种情况下，连接将被“降级”为普TCP，并继续以单条路径运行

这种行为由两个内部组件实现：路径管理器（path manager）和数据包调度器（packet scheduler）

### 路径管理


路径管理器负*子流**的创建与删除，以及地址通告。通常，由客户端发起子流，由服务端通过 `ADD_ADDR` `REMOVE_ADDR` 选项通告额外的地址

路径管理器由 `net.mptcp.path_manager` sysctl 旋钮控制——参mptcp-sysctl.rst。有两种类型：内核态的（`kernel`），对所有连接应用相同规则（参见：`ip mptcp`）；以及用户态的（`userspace`），由用户态守护进程（`mptcpd <https://mptcpd.mptcp.dev/>`_）控制，可以对每个连接应用不同的规则。路径管理器可通过 Netlink API 控制；参../netlink/specs/mptcp_pm.rst

为了能够在主机上使用多个 IP 地址来创建多*子流**（路径），默认的内核MPTCP 路径管理器需要知道哪IP 地址可以使用。这可以通过 `ip mptcp endpoint` 等命令进行配置

### 数据包调度器


数据包调度器负责选择使用哪个可用*子流（subflow*来发送下一个数据包。它可以决定最大限度地利用可用带宽，或者只选择延迟最低的路径，或者根据配置采用任何其它策略

数据包调度器`net.mptcp.scheduler` sysctl 旋钮控制——参mptcp-sysctl.rst


## 濂楁帴瀛?API


### 创建 MPTCP 套接


Linux 上，可以在创`socket` 时选择 MPTCP 而非 TCP 来使MPTCP


    int sd = socket(AF_INET(6), SOCK_STREAM, IPPROTO_MPTCP);

注意 `IPPROTO_MPTCP` 被定义为 `262`

如果 MPTCP 不受支持，`errno` 将被设置为：

- `EINVAL`：（**无效参数**）：< 5.6 的内核上 MPTCP 不可用
- `EPROTONOSUPPORT`*协议不支*）：>= v5.6 的内核上 MPTCP 未被编译进内核
- `ENOPROTOOPT`*协议不可*）：MPTCP 已通过 `net.mptcp.enabled` sysctl 旋钮被禁用；参见 mptcp-sysctl.rst

MPTCP 因此是选择启用的（opt-in）：应用程序需要显式请求它。注意，应用程序可以通过不同的技术被强制使用 MPTCP，例`LD_PRELOAD`（参`mptcpize`）、eBPF（参`mptcpify`）、SystemTAP、`GODEBUG`（`GODEBUG=multipathtcp=1`）等

切换`IPPROTO_MPTCP` 而非 `IPPROTO_TCP` 对用户空间应用程序应当尽可能透明

### 套接字选项


MPTCP 支持 TCP 处理的大多数套接字选项。可能某些不太常见的选项不受支持，但欢迎贡献

通常，相同的值会传播到所有子流，包括在调`setsockopt()` 之后创建的子流。eBPF 可用于为每个子流设置不同的值

`SOL_MPTCP`84）层级有一MPTCP 特有的套接字选项用于获取信息。它们填`getsockopt()` 系统调用`optval` 缓冲区：

- `MPTCP_INFO`：使`struct mptcp_info`
- `MPTCP_TCPINFO`：使`struct mptcp_subflow_data`，后接一`struct tcp_info` 数组
- `MPTCP_SUBFLOW_ADDRS`：使`struct mptcp_subflow_data`，后接一`mptcp_subflow_addrs` 数组
- `MPTCP_FULL_INFO`：使`struct mptcp_full_info`，其中一个指针指`struct mptcp_subflow_info` 数组（包`struct mptcp_subflow_addrs`），另一个指针指`struct tcp_info` 数组，后`struct mptcp_info` 的内容

注意，在 TCP 层级，`TCP_IS_MPTCP` 套接字选项可用于获知当前是否正在使MPTCP：如果是，该值将被设1


## 设计选择


为用户空facing 套接字新增了一MPTCP 套接字类型。内核负责创建子流套接字：它们是 TCP 套接字，其行为通过 TCP-ULP 进行修改

如果来自客户端的连接请求没有要求 MPTCP，MPTCP 监听套接字将创建“普通”的**已接受（accepted* TCP 套接字，从而在默认启用 MPTCP 时将性能影响降到最低
