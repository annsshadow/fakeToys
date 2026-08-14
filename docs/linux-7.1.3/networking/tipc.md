
## Linux 内核 TIPC


## 简介


TIPC（Transparent Inter Process Communication，透明进程间通信）是一种专门为集群内
通信而设计的协议。它被配置为可以通过 UDP 或直接在以太网上传输消息。消息传递保证
有序、无丢失且受流控。其延迟时间比任何已知协议都短，而最大吞吐量与 TCP 相当。

### TIPC 特性


- 集群范围的 IPC 服务

  您是否曾希望即使在集群节点之间传输数据时，也能拥有 Unix 域套接字那样的便利？即
  由您自己决定想要绑定并使用的地址？不必执行 DNS 查询，也不必担心 IP 地址？不必
  启动定时器来监视对端套接字是否持续存在？同时又能避免该套接字类型的缺点，例如
  残留 inode 的风险？

  欢迎使用透明进程间通信服务，简称 TIPC，它为您提供上述所有功能，甚至更多。

- 服务寻址

  TIPC 的一个基本概念是服务寻址（Service Addressing），它使程序员能够选择自己的
  地址，将其绑定到服务器套接字，并让客户端程序仅使用该地址来发送消息。

- 服务跟踪

  想要等待服务器可用的客户端，使用服务跟踪机制来订阅与相应服务地址的套接字的
  绑定与解绑/关闭事件。

  服务跟踪机制也可用于集群拓扑跟踪，即订阅集群节点的可用性/不可用性。

  同样，服务跟踪机制也可用于集群连通性跟踪，即订阅集群节点之间各个链路的
  启用/停用事件。

- 传输模式

  使用服务地址，客户端可以向服务器套接字发送数据报消息。

  使用相同类型的地址，它可以与接受连接的服务器套接字建立连接。

  它也可以使用服务地址创建并加入通信组（Communication Group），这是 TIPC 对
  无代理消息总线的体现。

  在数据报模式和通信组模式下都可获得具有极佳性能和可扩展性的多播。

- 节点间链路

  集群中任意两个节点之间的通信由一条或两条节点间链路（Inter Node Link）维持，
  它们既保证数据流量的完整性，又监视对端节点的可用性。

- 集群可扩展性

  通过在节点间链路上应用重叠环监视（Overlapping Ring Monitoring）算法，可以将
  TIPC 集群扩展到多达 1000 个节点，同时保持邻居故障发现时间为 1-2 秒。对于较小的
  集群，这个时间可以更短。

- 邻居发现

  集群中的邻居节点发现通过以太网广播或 UDP 多播完成（当这些服务可用时）。如果
  不可用，则可以使用配置的对端 IP 地址。

- 配置

  在单节点模式下运行 TIPC 时不需要任何配置。在集群模式下运行时，TIPC 至少必须
  被给定一个节点地址（Linux 4.17 之前），并告知要附加到哪个接口。"tipc" 配置工具
  使得添加和维护更多配置参数成为可能。

- 性能

  TIPC 消息传输的延迟时间优于任何已知协议。节点间连接的最大字节吞吐量仍然略低于
  TCP，而在同一主机上的节点内和容器间吞吐量方面则更胜一筹。

- 语言支持

  TIPC 的用户 API 支持 C、Python、Perl、Ruby、D 和 Go。

### 更多信息


- 如何搭建 TIPC：

  http://tipc.io/getting_started.html

- 如何使用 TIPC 编程：

  http://tipc.io/programming.html

- 如何为 TIPC 做贡献：

  http://tipc.io/contacts.html

- 关于 TIPC 规范的更多细节：

  http://tipc.io/protocol.html


## 实现


TIPC 实现为 net/tipc/ 目录下的一个内核模块。

### TIPC 基础类型


   :internal:

   :internal:

   :internal:

   :internal:

   :internal:

### TIPC Bearer 接口


   :internal:

   :internal:

### TIPC 加密接口


   :internal:

### TIPC 发现接口


   :internal:

### TIPC 链路接口


   :internal:

### TIPC 消息接口


   :internal:

### TIPC 名称接口


   :internal:

   :internal:

### TIPC 节点管理接口


   :internal:

### TIPC 套接字接口


   :internal:

### TIPC 网络拓扑接口


   :internal:

### TIPC 服务器接口


   :internal:

### TIPC 跟踪接口


   :internal:
