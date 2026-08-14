## SNMP 计数器


本文档解释 SNMP 计数器的含义。

## 通用 IPv4 计数器

所有第 4 层报文和 ICMP 报文都会改变这些计数器，但第 2 层报文（例如 STP）
或 ARP 报文不会改变它们。

- IpInReceives

定义于 `RFC1213 ipInReceives`_


IP 层收到的报文数量。它在 ip_rcv 函数开头处递增，始终与 IpExtInOctets
一起更新。即使报文后来被丢弃（例如由于 IP 头部无效或校验和错误等），
它仍会增加。它表示 GRO/LRO 之后聚合段的数量。

- IpInDelivers

定义于 `RFC1213 ipInDelivers`_


投递给上层协议的报文数量。例如 TCP、UDP、ICMP 等。如果没有人在 raw
socket 上监听，则只有内核支持的协议会被投递；如果有人在 raw socket 上
监听，所有合法的 IP 报文都会被投递。

- IpOutRequests

定义于 `RFC1213 ipOutRequests`_


经由 IP 层发送的报文数量，包括单播和多播报文，并且始终与 IpExtOutOctets
一起更新。

- IpExtInOctets 与 IpExtOutOctets

它们是 Linux 内核扩展，没有 RFC 定义。请注意，RFC1213 确实定义了
ifInOctets 和 ifOutOctets，但它们是不同的东西。ifInOctets 和 ifOutOctets
包含 MAC 层头部大小，而 IpExtInOctets 和 IpExtOutOctets 不包含，它们
只包含 IP 层头部和 IP 层数据。

- IpExtInNoECTPkts、IpExtInECT1Pkts、IpExtInECT0Pkts、IpExtInCEPkts

它们表示四种 ECN IP 报文的数量，更多细节请参考 `Explicit Congestion
Notification`_。


这 4 个计数器统计每种 ECN 状态下收到的报文数量。无论 LRO/GRO，它们都
统计真实的帧数。因此对于同一个报文，你可能会发现 IpInReceives 计数为 1，
而 IpExtInNoECTPkts 计数为 2 或更多。

- IpInHdrErrors

定义于 `RFC1213 ipInHdrErrors`_. 它表示报文因 IP 头部错误而被丢弃。它
可能发生在 IP 输入路径和 IP 转发路径中。


- IpInAddrErrors

定义于 `RFC1213 ipInAddrErrors`_. 它会在两种情况下增加：(1) IP 地址无效。
(2) 目的 IP 地址不是本地地址，且未启用 IP 转发。


- IpExtInNoRoutes

该计数器表示当 IP 协议栈收到一个报文、且无法从路由表中为其找到路由时，
该报文被丢弃。它可能发生在启用了 IP 转发、目的 IP 地址不是本地地址、且
不存在通往该目的 IP 地址的路由时。

- IpInUnknownProtos

定义于 `RFC1213 ipInUnknownProtos`_. 如果第 4 层协议不被内核支持，它就会
增加。如果应用程序正在使用 raw socket，内核总会将报文投递给 raw socket，
该计数器则不会增加。


- IpExtInTruncatedPkts

对于 IPv4 报文，它表示实际数据大小小于 IPv4 头部中的 "Total Length" 字段。

- IpInDiscards

定义于 `RFC1213 ipInDiscards`_. 它表示报文因内核内部原因（例如内存不足）
在 IP 接收路径中被丢弃。


- IpOutDiscards

定义于 `RFC1213 ipOutDiscards`_. 它表示报文因内核内部原因在 IP 发送路径中
被丢弃。


- IpOutNoRoutes

定义于 `RFC1213 ipOutNoRoutes`_. 它表示报文在 IP 发送路径中被丢弃，且
找不到通往它的路由。


## ICMP 计数器

- IcmpInMsgs 与 IcmpOutMsgs

由 `RFC1213 icmpInMsgs`_ 和 `RFC1213 icmpOutMsgs`_ 定义


如 RFC1213 所述，这两个计数器包含错误，即使 ICMP 报文类型无效它们也会
增加。ICMP 输出路径会检查 raw socket 的头部，因此即使 IP 头部由用户态
程序构造，IcmpOutMsgs 仍会被更新。

- ICMP 命名类型

| 这些计数器包含大多数常见的 ICMP 类型，它们是：
| IcmpInDestUnreachs: `RFC1213 icmpInDestUnreachs`_
| IcmpInTimeExcds: `RFC1213 icmpInTimeExcds`_
| IcmpInParmProbs: `RFC1213 icmpInParmProbs`_
| IcmpInSrcQuenchs: `RFC1213 icmpInSrcQuenchs`_
| IcmpInRedirects: `RFC1213 icmpInRedirects`_
| IcmpInEchos: `RFC1213 icmpInEchos`_
| IcmpInEchoReps: `RFC1213 icmpInEchoReps`_
| IcmpInTimestamps: `RFC1213 icmpInTimestamps`_
| IcmpInTimestampReps: `RFC1213 icmpInTimestampReps`_
| IcmpInAddrMasks: `RFC1213 icmpInAddrMasks`_
| IcmpInAddrMaskReps: `RFC1213 icmpInAddrMaskReps`_
| IcmpOutDestUnreachs: `RFC1213 icmpOutDestUnreachs`_
| IcmpOutTimeExcds: `RFC1213 icmpOutTimeExcds`_
| IcmpOutParmProbs: `RFC1213 icmpOutParmProbs`_
| IcmpOutSrcQuenchs: `RFC1213 icmpOutSrcQuenchs`_
| IcmpOutRedirects: `RFC1213 icmpOutRedirects`_
| IcmpOutEchos: `RFC1213 icmpOutEchos`_
| IcmpOutEchoReps: `RFC1213 icmpOutEchoReps`_
| IcmpOutTimestamps: `RFC1213 icmpOutTimestamps`_
| IcmpOutTimestampReps: `RFC1213 icmpOutTimestampReps`_
| IcmpOutAddrMasks: `RFC1213 icmpOutAddrMasks`_
| IcmpOutAddrMaskReps: `RFC1213 icmpOutAddrMaskReps`_



每种 ICMP 类型都有两个计数器：'In' 和 'Out'。例如，对于 ICMP Echo 报文，
它们是 IcmpInEchos 和 IcmpOutEchos。它们的含义很直观。'In' 计数器表示内核
收到了这样的报文，'Out' 计数器表示内核发送了这样的报文。

- ICMP 数字类型

它们是 IcmpMsgInType[N] 和 IcmpMsgOutType[N]，[N] 表示 ICMP 类型号。这些
计数器跟踪所有种类的 ICMP 报文。ICMP 类型号的定义可以在 `ICMP parameters`_
文档中找到。


例如，如果 Linux 内核发送一个 ICMP Echo 报文，IcmpMsgOutType8 会增加 1。
如果内核收到一个 ICMP Echo Reply 报文，IcmpMsgInType0 会增加 1。

- IcmpInCsumErrors

该计数器表示 ICMP 报文的校验和错误。内核会在更新 IcmpInMsgs 之后、更新
IcmpMsgInType[N] 之前校验校验和。如果报文校验和错误，IcmpInMsgs 会被更新，
但任何 IcmpMsgInType[N] 都不会被更新。

- IcmpInErrors 与 IcmpOutErrors

由 `RFC1213 icmpInErrors`_ 和 `RFC1213 icmpOutErrors`_ 定义


当 ICMP 报文处理路径中发生错误时，这两个计数器会被更新。接收报文路径
使用 IcmpInErrors，发送报文路径使用 IcmpOutErrors。当 IcmpInCsumErrors
增加时，IcmpInErrors 总会同时增加。

### ICMP 计数器之间的关系

IcmpMsgOutType[N] 的总和始终等于 IcmpOutMsgs，因为它们是同时更新的。
IcmpMsgInType[N] 的总和加上 IcmpInErrors 应当等于或大于 IcmpInMsgs。当
内核收到一个 ICMP 报文时，内核遵循以下逻辑：

1. 增加 IcmpInMsgs
2. 如果有任何错误，更新 IcmpInErrors 并结束处理
3. 更新 IcmpMsgOutType[N]
4. 根据类型处理报文，如果有任何错误，更新
   IcmpInErrors 并结束处理

因此，如果所有错误都发生在步骤 (2)，IcmpInMsgs 应等于 IcmpMsgOutType[N]
的总和加上 IcmpInErrors。如果所有错误都发生在步骤 (4)，IcmpInMsgs 应等于
IcmpMsgOutType[N] 的总和。如果错误同时发生在步骤 (2) 和步骤 (4)，IcmpInMsgs
应小于 IcmpMsgOutType[N] 的总和加上 IcmpInErrors。

## 通用 TCP 计数器

- TcpInSegs

定义于 `RFC1213 tcpInSegs`_


TCP 层收到的报文数量。如 RFC1213 所述，它包含接收时出错的报文，例如校验和
错误、TCP 头部无效等。只有一种错误不会被计入：如果第 2 层目的地址不是
NIC 的第 2 层地址。这可能发生在报文是组播或广播报文，或者 NIC 处于混杂
模式时。在这些情况下，报文会被投递给 TCP 层，但 TCP 层会在增加 TcpInSegs
之前丢弃这些报文。TcpInSegs 计数器不感知 GRO。因此如果有两个报文被 GRO
合并，TcpInSegs 计数器只会增加 1。

- TcpOutSegs

定义于 `RFC1213 tcpOutSegs`_


TCP 层发送的报文数量。如 RFC1213 所述，它不包含重传的报文，但包含 SYN、
ACK 和 RST 报文。与 TcpInSegs 不同，TcpOutSegs 感知 GSO，因此如果一个报文
被 GSO 拆分为 2 个，TcpOutSegs 会增加 2。

- TcpActiveOpens

定义于 `RFC1213 tcpActiveOpens`_


它表示 TCP 层发送一个 SYN，并进入 SYN-SENT 状态。每次 TcpActiveOpens 增加
1，TcpOutSegs 都应当同时增加 1。

- TcpPassiveOpens

定义于 `RFC1213 tcpPassiveOpens`_


它表示 TCP 层收到一个 SYN，回复 SYN+ACK，进入 SYN-RCVD 状态。

- TcpExtTCPRcvCoalesce

当 TCP 层收到报文且应用程序尚未读取时，TCP 层会尝试合并它们。该计数器
表示在这种情形下合并了多少个报文。如果启用了 GRO，大量报文会被 GRO 合并，
这些报文不会被计入 TcpExtTCPRcvCoalesce。

- TcpExtTCPAutoCorking

发送报文时，TCP 层会尝试将小报文合并为更大的报文。在这种情形下每合并一个
报文，该计数器增加 1。更多细节请参考 LWN 文章：
https://lwn.net/Articles/576263/

- TcpExtTCPOrigDataSent

该计数器由内核提交 f19c29e3e391 解释，我粘贴了
```

  TCPOrigDataSent: number of outgoing packets with original data (excluding
  retransmission but including data-in-SYN). This counter is different from
  TcpOutSegs because TcpOutSegs also tracks pure ACKs. TCPOrigDataSent is
  more useful to track the TCP retransmission rate.

```
- TCPSynRetrans

该计数器由内核提交 f19c29e3e391 解释，我粘贴了
```

  TCPSynRetrans: number of SYN and SYN/ACK retransmits to break down
  retransmissions into SYN, fast-retransmits, timeout retransmits, etc.

```
- TCPFastOpenActiveFail

该计数器由内核提交 f19c29e3e391 解释，我粘贴了
```

  TCPFastOpenActiveFail: Fast Open attempts (SYN/data) failed because
  the remote does not accept it or the attempts timed out.

```
- TcpExtListenOverflows 与 TcpExtListenDrops

当内核收到来自客户端的 SYN，且 TCP accept 队列已满时，内核会丢弃该 SYN
并向 TcpExtListenOverflows 加 1。同时，内核也会向 TcpExtListenDrops 加 1。
当一个 TCP socket 处于 LISTEN 状态时，只要内核需要丢弃一个报文，内核总会
向 TcpExtListenDrops 加 1。因此，TcpExtListenOverflows 的增加会同时导致
TcpExtListenDrops 增加，但 TcpExtListenDrops 也可能在 TcpExtListenOverflows
不增加的情况下增加，例如内存分配失败也会让 TcpExtListenDrops 增加。

注意：上述解释基于内核 4.10 或以上版本，在旧内核上，当 TCP accept 队列
已满时 TCP 协议栈的行为不同。在旧内核上，TCP 协议栈不会丢弃 SYN，而是会
完成三次握手。由于 accept 队列已满，TCP 协议栈会将该 socket 保留在 TCP
半开队列中。因为它处于半开队列中，TCP 协议栈会以指数退避定时器发送
SYN+ACK，在客户端回复 ACK 后，TCP 协议栈会检查 accept 队列是否仍然已满，
如果未满，则将该 socket 移入 accept 队列；如果已满，则继续将其保留在半开
队列中，待客户端下次回复 ACK 时，该 socket 会获得又一次移入 accept 队列的
机会。


## TCP Fast Open

- TcpEstabResets

定义于 `RFC1213 tcpEstabResets`_.


- TcpAttemptFails

定义于 `RFC1213 tcpAttemptFails`_.


- TcpOutRsts

定义于 `RFC1213 tcpOutRsts`_. RFC 指出该计数器表示“包含 RST 标志的段”，
但在 linux 内核中，该计数器表示的是内核尝试发送的段。发送过程可能因某些
错误（例如内存分配失败）而失败。


- TcpExtTCPSpuriousRtxHostQueues

当 TCP 协议栈想要重传一个报文，却发现该报文并非在网络中丢失，而是尚未
发送出去时，TCP 协议栈会放弃重传并更新该计数器。当报文在 qdisc 或驱动
队列中停留过久时可能会发生这种情况。

- TcpEstabResets

socket 在 Establish 或 CloseWait 状态收到了一个 RST 报文。

- TcpExtTCPKeepAlive

该计数器表示已发送了多少个 keepalive 报文。keepalive 默认不启用。用户态
程序可以通过设置 SO_KEEPALIVE socket 选项来启用它。

- TcpExtTCPSpuriousRTOs

由 `F-RTO`_ 算法检测到的虚假重传超时。


## TCP Fast Path

当内核收到一个 TCP 报文时，它有两条路径来处理该报文，一条是快速路径，
另一条是慢速路径。内核中的注释
```

  It is split into a fast path and a slow path. The fast path is
  disabled when:

  - A zero window was announced from us
  - zero window probing
    is only handled properly on the slow path.
  - Out of order segments arrived.
  - Urgent data is expected.
  - There is no buffer space left
  - Unexpected TCP flags/window values/header lengths are received
    (detected by checking the TCP header against pred_flags)
  - Data is sent in both directions. The fast path only supports pure senders
    or pure receivers (this means either the sequence number or the ack
    value must stay constant)
  - Unexpected TCP option.

```
除非满足上述任何条件，内核都会尝试使用快速路径。如果报文乱序，内核会以
慢速路径处理，这意味着性能可能不太好。如果使用了 "Delayed ack"，内核也会
进入慢速路径，因为使用 "Delayed ack" 时数据会在两个方向上发送。当未使用
TCP window scale 选项时，内核会在连接进入 established 状态时立即尝试启用
快速路径；但如果使用了 TCP window scale 选项，内核会先禁用快速路径，并在
收到报文后再尝试启用它。

- TcpExtTCPPureAcks 与 TcpExtTCPHPAcks

如果一个报文设置了 ACK 标志且没有数据，它就是纯 ACK 报文；如果内核以
快速路径处理它，TcpExtTCPHPAcks 增加 1；如果内核以慢速路径处理它，
TcpExtTCPPureAcks 增加 1。

- TcpExtTCPHPHits

如果一个 TCP 报文带有数据（即它不是纯 ACK 报文），并且该报文以快速路径
处理，TcpExtTCPHPHits 增加 1。


## TCP 中止（abort）

- TcpExtTCPAbortOnData

它表示 TCP 层有在途数据，但需要关闭连接。因此 TCP 层向对端发送一个 RST，
表明连接并非优雅地关闭。一种增加该计数器的简单方法是使用 SO_LINGER 选项。
请参考 `socket man page`_ 的 SO_LINGER 小节：


默认情况下，当应用程序关闭一个连接时，close 函数会立即返回，内核会尝试
异步发送在途数据。如果使用 SO_LINGER 选项，将 l_onoff 设为 1，并将 l_linger
设为一个正数，close 函数不会立即返回，而是等待在途数据被对端确认，最长
等待时间为 l_linger 秒。如果将 l_onoff 设为 1 并将 l_linger 设为 0，当
应用程序关闭连接时，内核会立即发送一个 RST，并增加 TcpExtTCPAbortOnData
计数器。

- TcpExtTCPAbortOnClose

该计数器表示当应用程序想要关闭 TCP 连接时，TCP 层中存在尚未被读取的数据。
在这种情况下，内核会向 TCP 连接的对端发送一个 RST。

- TcpExtTCPAbortOnMemory

当应用程序关闭一个 TCP 连接时，内核仍需要跟踪该连接，让其完成 TCP 断开
过程。例如，应用程序调用了 socket 的 close 方法，内核向连接对端发送 fin，
之后应用程序与该 socket 再无关联，但内核需要保留该 socket，这个 socket 变成
了孤儿 socket，内核等待对端的回复，最终会进入 TIME_WAIT 状态。当内核没有
足够的内存来保留该孤儿 socket 时，内核会向对端发送一个 RST 并删除该 socket，
在这种情况下，内核会让 TcpExtTCPAbortOnMemory 加 1。有两种情况会触发
TcpExtTCPAbortOnMemory：

1. TCP 协议使用的内存高于 tcp_mem 的第三个值。请参考 `TCP man page`_ 的
tcp_mem 小节：


2. 孤儿 socket 数量高于 net.ipv4.tcp_max_orphans


- TcpExtTCPAbortOnTimeout

当任何一个 TCP 定时器超时时，该计数器会增加。在这种情况下，内核不会发送
RST，只是放弃该连接。

- TcpExtTCPAbortOnLinger

当一个 TCP 连接进入 FIN_WAIT_2 状态时，内核可以不等待对端的 fin 报文，
而是立即发送一个 RST 并删除该 socket。这不是 Linux 内核 TCP 协议栈的默认
行为。通过配置 TCP_LINGER2 socket 选项，可以让内核采取这种行为。

- TcpExtTCPAbortFailed

如果满足 `RFC2525 2.17 section`_，内核 TCP 层会发送 RST。如果在此过程中
发生内部错误，TcpExtTCPAbortFailed 会增加。


## TCP 混合慢启动（Hybrid Slow Start）

混合慢启动算法是对传统 TCP 拥塞窗口慢启动算法的增强。它利用两类信息来
检测是否接近 TCP 路径的最大带宽。这两类信息是 ACK 列车（ACK train）长度
和报文延迟的增加。更多细节请参考 `Hybrid Slow Start paper`_。只要 ACK 列车
长度或报文延迟达到某个特定阈值，拥塞控制算法就会进入拥塞避免（Congestion
Avoidance）状态。直到 v4.20，有两个拥塞控制算法使用了混合慢启动，它们是
cubic（默认的拥塞控制算法）和 cdg。有四个 snmp 计数器与混合慢启动算法相关。


- TcpExtTCPHystartTrainDetect

检测到 ACK 列车长度阈值的次数

- TcpExtTCPHystartTrainCwnd

由 ACK 列车长度检测到的 CWND 之和。将该值除以 TcpExtTCPHystartTrainDetect
即为由 ACK 列车长度检测到的平均 CWND。

- TcpExtTCPHystartDelayDetect

检测到报文延迟阈值的次数。

- TcpExtTCPHystartDelayCwnd

由报文延迟检测到的 CWND 之和。将该值除以 TcpExtTCPHystartDelayDetect 即为
由报文延迟检测到的平均 CWND。

## TCP 重传与拥塞控制

TCP 协议有两种重传机制：SACK 和快速恢复（fast recovery）。它们彼此互斥。
当启用 SACK 时，内核 TCP 协议栈会使用 SACK，否则内核会使用快速恢复。SACK
是一个 TCP 选项，定义于 `RFC2018`_，快速恢复定义于 `RFC6582`_，也称为
'Reno'。

TCP 拥塞控制是一个庞大而复杂的主题。要理解相关的 snmp 计数器，我们需要
了解拥塞控制状态机的状态。共有 5 个状态：Open、Disorder、CWR、Recovery 和
Loss。关于这些状态的细节，请参考该文档的第 5 页和第 6 页：
https://pdfs.semanticscholar.org/0e9c/968d09ab2e53e24c4dca5b2d67c7f7140f8e.pdf


- TcpExtTCPRenoRecovery 与 TcpExtTCPSackRecovery

当拥塞控制进入 Recovery 状态时，如果使用了 sack，TcpExtTCPSackRecovery
增加 1；如果未使用 sack，TcpExtTCPRenoRecovery 增加 1。这两个计数器表示
TCP 协议栈开始重传丢失的报文。

- TcpExtTCPSACKReneging

一个报文已被 SACK 确认，但接收方丢弃了该报文，因此发送方需要重传该报文。
在这种情况下，发送方将 TcpExtTCPSACKReneging 加 1。接收方可能丢弃一个
已被 SACK 确认的报文，尽管这很不寻常，但 TCP 协议是允许的。发送方其实
并不知道接收方发生了什么。发送方只是等待该报文的 RTO 超时，然后假定该
报文已被接收方丢弃。

- TcpExtTCPRenoReorder

乱序报文由快速恢复检测。它仅在禁用 SACK 时使用。快速恢复算法通过重复 ACK
的数量来检测乱序。例如，如果触发了重传，而原本被重传的报文并未丢失，只是
乱序，接收方会进行多次确认，一次针对重传的报文，另一次针对原始乱序报文的
到达。因此发送方会发现收到的 ACK 多于预期，从而知道发生了乱序。

- TcpExtTCPTSReorder

当填补一个空隙（hole）时检测到乱序报文。例如，假设发送方发送了报文
1、2、3、4、5，而接收顺序是 1、2、4、5、3。当发送方收到报文 3 的 ACK（将
填补空隙）时，两种情况下会让 TcpExtTCPTSReorder 增加 1：(1) 如果报文 3
尚未被再次重传。(2) 如果报文 3 已被重传，但其 ACK 的时间戳早于重传的
时间戳。

- TcpExtTCPSACKReorder

由 SACK 检测到的乱序报文。SACK 有两种方法检测乱序：(1) 发送方收到 DSACK。
这表示发送方多次发送了同一个报文，唯一的原因是发送方认为一个乱序报文已
丢失，于是再次发送该报文。(2) 假设发送方发送了报文 1、2、3、4、5，且
发送方已收到报文 2 和 5 的 SACK，现在发送方收到报文 4 的 SACK，且发送方
尚未重传该报文，发送方就会知道报文 4 是乱序的。在上述两种情况下，内核的
TCP 协议栈都会增加 TcpExtTCPSACKReorder。

- TcpExtTCPSlowStartRetrans

TCP 协议栈想要重传一个报文，且拥塞控制状态为 'Loss'。

- TcpExtTCPFastRetrans

TCP 协议栈想要重传一个报文，且拥塞控制状态不为 'Loss'。

- TcpExtTCPLostRetransmit

一个 SACK 指出某个重传报文再次丢失。

- TcpExtTCPRetransFail

TCP 协议栈试图将一个重传报文交付给下层，但下层返回了错误。

- TcpExtTCPSynRetrans

TCP 协议栈重传一个 SYN 报文。

## DSACK

DSACK 定义于 `RFC2883`_。接收方使用 DSACK 向发送方报告重复的报文。存在两类
重复：(1) 一个已被确认的报文是重复的。(2) 一个乱序报文是重复的。TCP 协议栈
在接收方和发送方两侧都统计这两类重复。


- TcpExtTCPDSACKOldSent

TCP 协议栈收到一个已被确认的重复报文，于是向发送方发送一个 DSACK。

- TcpExtTCPDSACKOfoSent

TCP 协议栈收到一个乱序的重复报文，于是向发送方发送一个 DSACK。

- TcpExtTCPDSACKRecv

TCP 协议栈收到一个 DSACK，表示收到了一个已被确认的重复报文。

- TcpExtTCPDSACKOfoRecv

TCP 协议栈收到一个 DSACK，表示收到了一个乱序的重复报文。

## 无效的 SACK 与 DSACK

当一个 SACK（或 DSACK）块无效时，相应的计数器会被更新。校验方法基于 SACK
块的起始/结束序列号。更多细节请参考内核源码中函数 tcp_is_sackblock_valid
的注释。一个 SACK 选项最多可以有 4 个块，它们会被逐一检查。例如，如果
一个 SACK 有 3 个块无效，相应的计数器会被更新 3 次。提交 18f02545a9a1
（"[TCP] MIB: Add counters for discarded SACK blocks"）的注释有额外的解释：

- TcpExtTCPSACKDiscard

该计数器表示有多少个 SACK 块无效。如果无效的 SACK 块是由 ACK 记录（ACK
recording）引起的，TCP 协议栈只会忽略它，而不会更新该计数器。

- TcpExtTCPDSACKIgnoredOld 与 TcpExtTCPDSACKIgnoredNoUndo

当一个 DSACK 块无效时，这两个计数器之一会被更新。更新哪个计数器取决于
TCP socket 的 undo_marker 标志。如果 undo_marker 未设置，TCP 协议栈不太
可能重传任何报文，而我们仍然收到了一个无效的 DSACK 块，原因可能是报文在
网络中间被复制了。在这种情况下，TcpExtTCPDSACKIgnoredNoUndo 会被更新。如果
undo_marker 已设置，TcpExtTCPDSACKIgnoredOld 会被更新。正如其名称所暗示的，
它可能是一个旧的报文。

## SACK 移位（shift）

Linux 网络协议栈将数据存储在 sk_buff 结构体（简称 skb）中。如果一个 SACK
块跨越多个 skb，TCP 协议栈会尝试重新整理这些 skb 中的数据。例如，如果一个
SACK 块确认了 seq 10 到 15，skb1 拥有 seq 10 到 13，skb2 拥有 seq 14 到 20。
skb2 中的 seq 14 和 15 会被移动到 skb1。这个操作称为 'shift'（移位）。如果
一个 SACK 块确认了 seq 10 到 20，skb1 拥有 seq 10 到 13，skb2 拥有 seq 14
到 20。skb2 中的全部数据都会被移动到 skb1，并且 skb2 会被丢弃，这个操作
称为 'merge'（合并）。

- TcpExtTCPSackShifted

一个 skb 被移位

- TcpExtTCPSackMerged

一个 skb 被合并

- TcpExtTCPSackShiftFallback

一个 skb 本应被移位或合并，但 TCP 协议栈出于某些原因没有这么做。

## TCP 乱序（out of order）

- TcpExtTCPOFOQueue

TCP 层收到一个乱序报文，并且有充足的内存将其入队。

- TcpExtTCPOFODrop

TCP 层收到一个乱序报文，但没有足够的内存，于是将其丢弃。这类报文不会被
计入 TcpExtTCPOFOQueue。

- TcpExtTCPOFOMerge

收到的乱序报文与前一个报文存在重叠。重叠部分会被丢弃。所有 TcpExtTCPOFOMerge
报文也会被计入 TcpExtTCPOFOQueue。

## TCP PAWS

PAWS（Protection Against Wrapped Sequence numbers，防止序列号回绕）是一种
用于丢弃旧报文的算法。它依赖于 TCP 时间戳。更多细节请参考 `timestamp wiki`_
和 `RFC of PAWS`_。


- TcpExtPAWSActive

报文在 Syn-Sent 状态下被 PAWS 丢弃。

- TcpExtPAWSEstab

报文在除 Syn-Sent 之外的任何状态下被 PAWS 丢弃。

## TCP ACK 跳过

在某些场景下，内核会避免过于频繁地发送重复 ACK。更多细节请参考 `sysctl
document`_ 的 tcp_invalid_ratelimit 小节。当内核由于 tcp_invalid_ratelimit
决定跳过一个 ACK 时，内核会更新以下某个计数器，以表明该 ACK 是在哪种场景下
被跳过的。只有当收到的报文是 SYN 报文或不含数据时，ACK 才会被跳过。


- TcpExtTCPACKSkippedSynRecv

ACK 在 Syn-Recv 状态下被跳过。Syn-Recv 状态表示 TCP 协议栈收到了 SYN 并
回复了 SYN+ACK。此时 TCP 协议栈正在等待一个 ACK。通常，TCP 协议栈在 Syn-Recv
状态下不需要发送 ACK。但在几种场景下，TCP 协议栈需要发送 ACK。例如，TCP
协议栈重复收到相同的 SYN 报文、收到的报文未通过 PAWS 检查，或收到的报文
序列号超出窗口。在这些场景下，TCP 协议栈需要发送 ACK。如果发送 ACK 的频率
高于 tcp_invalid_ratelimit 所允许的值，TCP 协议栈会跳过发送 ACK，并增加
TcpExtTCPACKSkippedSynRecv。


- TcpExtTCPACKSkippedPAWS

ACK 因为 PAWS（Protect Against Wrapped Sequence numbers，防止序列号回绕）
检查失败而被跳过。如果 PAWS 检查在 Syn-Recv、Fin-Wait-2 或 Time-Wait 状态下
失败，被跳过的 ACK 会被计入 TcpExtTCPACKSkippedSynRecv、TcpExtTCPACKSkippedFinWait2
或 TcpExtTCPACKSkippedTimeWait。在所有其他状态下，被跳过的 ACK 会被计入
TcpExtTCPACKSkippedPAWS。

- TcpExtTCPACKSkippedSeq

序列号超出窗口，且时间戳通过了 PAWS 检查，且 TCP 状态不是 Syn-Recv、
Fin-Wait-2 和 Time-Wait。

- TcpExtTCPACKSkippedFinWait2

ACK 在 Fin-Wait-2 状态下被跳过，原因可能是 PAWS 检查失败或收到的序列号
超出窗口。

- TcpExtTCPACKSkippedTimeWait

ACK 在 Time-Wait 状态下被跳过，原因可能是 PAWS 检查失败或收到的序列号
超出窗口。

- TcpExtTCPACKSkippedChallenge

如果该 ACK 是一个挑战（challenge）ACK，则跳过它。RFC 5961 定义了 3 种
挑战 ACK，请参考 `RFC 5961 section 3.2`_、`RFC 5961 section 4.2`_ 和
`RFC 5961 section 5.2`_。除了这三种场景外，在某些 TCP 状态下，如果 ACK 号
位于第一个未确认号之前，Linux 的 TCP 协议栈也会发送挑战 ACK（比 `RFC 5961
section 5.2`_ 更严格）。

## TCP 接收窗口

- TcpExtTCPWantZeroWindowAdv

根据当前内存使用情况，TCP 协议栈尝试将接收窗口设为零。但接收窗口仍可能是
一个非零值。例如，如果之前的窗口大小为 10，而 TCP 协议栈收到了 3 字节，那么
当前窗口大小会是 7，即使按内存使用量计算出的窗口大小为零。

- TcpExtTCPToZeroWindowAdv

TCP 接收窗口从一个非零值被设为零。

- TcpExtTCPFromZeroWindowAdv

TCP 接收窗口从零被设为非零值。


## 延迟确认（Delayed ACK）

TCP 延迟确认是一种用于减少网络中报文数量的技术。更多细节请参考 `Delayed ACK
wiki`_


- TcpExtDelayedACKs

一个延迟确认定时器到期。TCP 协议栈会发送一个纯 ACK 报文并退出延迟确认模式。

- TcpExtDelayedACKLocked

一个延迟确认定时器到期，但由于 socket 被用户态程序锁定，TCP 协议栈无法
立即发送 ACK。TCP 协议栈会在稍后（在用户态程序解锁该 socket 之后）发送一个
纯 ACK。当 TCP 协议栈稍后发送该纯 ACK 时，它也会更新 TcpExtDelayedACKs 并
退出延迟确认模式。

- TcpExtDelayedACKLost

当 TCP 协议栈收到一个已被确认的报文时，它会被更新。延迟确认丢失可能会
导致此问题，但它也可能由其他原因触发，例如报文在网络中被复制。

## 尾部丢失探测（TLP，Tail Loss Probe）

TLP 是一种用于检测 TCP 报文丢失的算法。更多细节请参考 `TLP paper`_。


- TcpExtTCPLossProbes

发送了一个 TLP 探测报文。

- TcpExtTCPLossProbeRecovery

检测到一个报文丢失并由 TLP 恢复。

## TCP 快速打开（TCP Fast Open）说明

TCP 快速打开是一种允许在三次握手完成之前传输数据的技术。一般性介绍请参考
`TCP Fast Open wiki`_。


- TcpExtTCPFastOpenActive

当 TCP 协议栈在 SYN-SENT 状态下收到一个 ACK 报文，且该 ACK 报文确认了 SYN
报文中的数据时，TCP 协议栈便知道 TFO cookie 已被对端接受，于是更新该计数器。

- TcpExtTCPFastOpenActiveFail

该计数器表示 TCP 协议栈发起了一个 TCP 快速打开，但失败了。该计数器会在三种
场景下更新：(1) 对端没有确认 SYN 报文中的数据。(2) 带有 TFO cookie 的 SYN
报文至少超时了一次。(3) 三次握手之后，重传超时发生了 net.ipv4.tcp_retries1
次，因为某些中间设备可能会在握手后“黑洞”掉快速打开。

- TcpExtTCPFastOpenPassive

该计数器表示 TCP 协议栈接受快速打开请求的次数。

- TcpExtTCPFastOpenPassiveFail

该计数器表示 TCP 协议栈拒绝快速打开请求的次数。其原因要么是 TFO cookie
无效，要么是 TCP 协议栈在创建 socket 的过程中发现错误。

- TcpExtTCPFastOpenListenOverflow

当待处理的快速打开请求数量大于 fastopenq->max_qlen 时，TCP 协议栈会拒绝
该快速打开请求并更新该计数器。当该计数器被更新时，TCP 协议栈不会更新
TcpExtTCPFastOpenPassive 或 TcpExtTCPFastOpenPassiveFail。fastopenq->max_qlen
由 TCP_FASTOPEN socket 操作设置，且不能大于 net.core.somaxconn。例如：

setsockopt(sfd, SOL_TCP, TCP_FASTOPEN, &qlen, sizeof(qlen));

- TcpExtTCPFastOpenCookieReqd

该计数器表示客户端想要请求一个 TFO cookie 的次数。

## SYN cookie

SYN cookie 用于缓解 SYN flood 攻击，更多细节请参考 `SYN cookies wiki`_。


- TcpExtSyncookiesSent

表示发送了多少个 SYN cookie。

- TcpExtSyncookiesRecv

TCP 协议栈收到了多少个 SYN cookie 的回复报文。

- TcpExtSyncookiesFailed

从 SYN cookie 中解码出的 MSS 无效。当该计数器被更新时，收到的报文不会被
当作 SYN cookie 处理，TcpExtSyncookiesRecv 计数器也不会被更新。

## 挑战 ACK（Challenge ACK）

关于挑战 ACK 的细节，请参考 TcpExtTCPACKSkippedChallenge 的说明。

- TcpExtTCPChallengeACK

发送的挑战 ACK 的数量。

- TcpExtTCPSYNChallenge

为响应 SYN 报文而发送的挑战 ACK 的数量。更新该计数器后，TCP 协议栈可能会
发送一个挑战 ACK 并更新 TcpExtTCPChallengeACK 计数器，也可能跳过发送挑战
ACK 而更新 TcpExtTCPACKSkippedChallenge。

## 修剪（prune）

当 socket 处于内存压力下时，TCP 协议栈会尝试从接收队列和乱序队列中回收内存。
其中一种回收方法是 'collapse'（折叠），即分配一个大的 skb，将连续的 skb
复制到这个大的 skb 中，并释放这些连续的 skb。

- TcpExtPruneCalled

TCP 协议栈尝试为一个 socket 回收内存。更新该计数器后，TCP 协议栈会尝试折叠
乱序队列和接收队列。如果内存仍然不足，TCP 协议栈会尝试从乱序队列中丢弃报文
（并更新 TcpExtOfoPruned 计数器）。

- TcpExtOfoPruned

TCP 协议栈尝试从乱序队列中丢弃报文。

- TcpExtRcvPruned

经过 'collapse' 并从乱序队列中丢弃报文后，如果实际使用的内存仍然大于允许的
最大内存，该计数器会被更新。这意味着 'prune' 失败。

- TcpExtTCPRcvCollapsed

该计数器表示在 'collapse' 过程中释放了多少个 skb。

## 示例


### ping 测试
```

  nstatuser@nstat-a:~$ ping 8.8.8.8 -c 1
  PING 8.8.8.8 (8.8.8.8) 56(84) bytes of data.
  64 bytes from 8.8.8.8: icmp_seq=1 ttl=119 time=17.8 ms

  --- 8.8.8.8 ping statistics ---
  1 packets transmitted, 1 received, 0% packet loss, time 0ms
  rtt min/avg/max/mdev = 17.875/17.875/17.875/0.000 ms

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  IcmpInMsgs                      1                  0.0
  IcmpInEchoReps                  1                  0.0
  IcmpOutMsgs                     1                  0.0
  IcmpOutEchos                    1                  0.0
  IcmpMsgInType0                  1                  0.0
  IcmpMsgOutType8                 1                  0.0
  IpExtInOctets                   84                 0.0
  IpExtOutOctets                  84                 0.0
  IpExtInNoECTPkts                1                  0.0

```
Linux 服务器发送了一个 ICMP Echo 报文，因此 IpOutRequests、
IcmpOutMsgs、IcmpOutEchos 和 IcmpMsgOutType8 各增加 1。服务器从
8.8.8.8 收到 ICMP Echo Reply，因此 IpInReceives、IcmpInMsgs、
IcmpInEchoReps 和 IcmpMsgInType0 各增加 1。该 ICMP Echo Reply 经
IP 层传递给 ICMP 层，因此 IpInDelivers 增加 1。ping 的默认数据大小
为 48，因此一个 ICMP Echo 报文及其对应的 Echo Reply 报文由以下部分
构成：

- 14 字节 MAC 头部
- 20 字节 IP 头部
- 16 字节 ICMP 头部
- 48 字节数据（ping 命令的默认值）

因此 IpExtInOctets 和 IpExtOutOctets 均为 20+16+48=84。

### TCP 三次握手
```

  nstatuser@nstat-b:~$ nc -lknv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -nv 192.168.122.251 9000
  Connection to 192.168.122.251 9000 port [tcp/*] succeeded!

```
服务器监听 tcp 9000 端口，客户端连接到它，双方完成了三次握手。
```

  nstatuser@nstat-b:~$ nstat | grep -i tcp
  TcpPassiveOpens                 1                  0.0
  TcpInSegs                       2                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPPureAcks               1                  0.0

```
```

  nstatuser@nstat-a:~$ nstat | grep -i tcp
  TcpActiveOpens                  1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      2                  0.0

```
当服务器收到第一个 SYN 时，它回复 SYN+ACK，并进入 SYN-RCVD 状态，
因此 TcpPassiveOpens 增加 1。服务器收到 SYN、发送 SYN+ACK、收到
ACK，因此服务器发送 1 个报文、接收 2 个报文，TcpInSegs 增加 2，
TcpOutSegs 增加 1。三次握手的最后一个 ACK 是不带数据的纯 ACK，因此
TcpExtTCPPureAcks 增加 1。

当客户端发送 SYN 时，客户端进入 SYN-SENT 状态，因此 TcpActiveOpens
增加 1；客户端发送 SYN、收到 SYN+ACK、发送 ACK，因此客户端发送 2 个
报文、接收 1 个报文，TcpInSegs 增加 1，TcpOutSegs 增加 2。

### TCP 正常流量
```

  nstatuser@nstat-b:~$ nc -lkv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!
  hello

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPPureAcks               1                  0.0
  TcpExtTCPOrigDataSent           1                  0.0
  IpExtInOctets                   52                 0.0
  IpExtOutOctets                  58                 0.0
  IpExtInNoECTPkts                1                  0.0

```
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  IpExtInOctets                   58                 0.0
  IpExtOutOctets                  52                 0.0
  IpExtInNoECTPkts                1                  0.0

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!
  hello
  world

```
```

  nstatuser@nstat-a:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPHPAcks                 1                  0.0
  TcpExtTCPOrigDataSent           1                  0.0
  IpExtInOctets                   52                 0.0
  IpExtOutOctets                  58                 0.0
  IpExtInNoECTPkts                1                  0.0


```
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpInDelivers                    1                  0.0
  IpOutRequests                   1                  0.0
  TcpInSegs                       1                  0.0
  TcpOutSegs                      1                  0.0
  TcpExtTCPHPHits                 1                  0.0
  IpExtInOctets                   58                 0.0
  IpExtOutOctets                  52                 0.0
  IpExtInNoECTPkts                1                  0.0

```
对比第一次客户端 nstat 与第二次客户端 nstat，我们可以发现一个差异：
第一次有 'TcpExtTCPPureAcks'，而第二次有 'TcpExtTCPHPAcks'。第一次
服务器端 nstat 与第二次服务器端 nstat 也有差异：第二次服务器端 nstat
有 TcpExtTCPHPHits，而第一次服务器端 nstat 没有。网络流量模式完全
相同：客户端向服务器发送一个报文，服务器回复一个 ACK。但内核以不同的
方式处理它们。当未使用 TCP window scale 选项时，内核会在连接进入
established 状态时立即尝试启用快速路径；但如果使用了 TCP window scale
选项，内核会先禁用快速路径，并在收到报文后再尝试启用它。我们可以使用
'ss' 命令来验证是否使用了 window scale 选项。例如，在服务器或客户端
上运行以下命令
```

  nstatuser@nstat-a:~$ ss -o state established -i '( dport = :9000 or sport = :9000 )
  Netid    Recv-Q     Send-Q            Local Address:Port             Peer Address:Port
  tcp      0          0               192.168.122.250:40654         192.168.122.251:9000
             ts sack cubic wscale:7,7 rto:204 rtt:0.98/0.49 mss:1448 pmtu:1500 rcvmss:536 advmss:1448 cwnd:10 bytes_acked:1 segs_out:2 segs_in:1 send 118.2Mbps lastsnd:46572 lastrcv:46572 lastack:46572 pacing_rate 236.4Mbps rcv_space:29200 rcv_ssthresh:29200 minrtt:0.98

```
'wscale:7,7' 表示服务器和客户端都将 window scale 选项设为 7。现在我们可以
解释测试中 nstat 的输出：

在客户端第一次 nstat 输出中，客户端发送了一个报文，服务器回复了一个
ACK，当内核处理这个 ACK 时，快速路径尚未启用，因此该 ACK 被计入
'TcpExtTCPPureAcks'。

在客户端第二次 nstat 输出中，客户端再次发送了一个报文，并收到服务器
的另一个 ACK，此时快速路径已启用，且该 ACK 符合快速路径条件，因此
由快速路径处理，该 ACK 被计入 TcpExtTCPHPAcks。

在服务器端第一次 nstat 输出中，快速路径未启用，因此没有
'TcpExtTCPHPHits'。

在服务器端第二次 nstat 输出中，快速路径已启用，并且从客户端收到的
报文符合快速路径条件，因此它被计入 'TcpExtTCPHPHits'。

### TcpExtTCPAbortOnClose
```

  import socket
  import time

  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
该 python 脚本监听 9000 端口，但不会从连接中读取任何数据。
```

  nstatuser@nstat-a:~$ echo "hello" | nc nstat-b 9000

```
然后，我们回到服务器端，服务器已经收到了 "hello" 报文，并且 TCP 层
已经对该报文进行了确认（ack），但应用程序尚未读取它。我们输入
Ctrl-C 来终止服务器脚本。然后我们
```

  nstatuser@nstat-b:~$ nstat | grep -i abort
  TcpExtTCPAbortOnClose           1                  0.0

```
如果我们在服务器端运行 tcpdump，可以发现服务器在我们输入 Ctrl-C 后
发送了一个 RST。

### TcpExtTCPAbortOnMemory 与 TcpExtTCPAbortOnTimeout

下面是一个让孤儿 socket 数量超过 net.ipv4.tcp_max_orphans 的示例。
```

  sudo bash -c "echo 10 > /proc/sys/net/ipv4/tcp_max_orphans"

```
```

  nstatuser@nstat-a:~$ cat client_orphan.py
  import socket
  import time

  server = 'nstat-b' # server address
  port = 9000

  count = 64

  connection_list = []

  for i in range(64):
      s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
      s.connect((server, port))
      connection_list.append(s)
      print("connection_count: %d" % len(connection_list))

  while True:
      time.sleep(99999)

```
```

  nstatuser@nstat-b:~$ cat server_orphan.py
  import socket
  import time

  port = 9000
  count = 64

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(count)
  connection_list = []
  while True:
      sock, addr = s.accept()
      connection_list.append((sock, addr))
      print("connection_count: %d" % len(connection_list))

```
在服务器和客户端上运行 python 脚本。
```

  python3 server_orphan.py

```
```

  python3 client_orphan.py

```
```

  sudo iptables -A INPUT -i ens3 -p tcp --destination-port 9000 -j DROP

```
在客户端输入 Ctrl-C，停止 client_orphan.py。
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnMemory          54                 0.0

```
```

  nstatuser@nstat-a:~$ ss -s
  Total: 131 (kernel 0)
  TCP:   14 (estab 1, closed 0, orphaned 10, synrecv 0, timewait 0/0), ports 0

  Transport Total     IP        IPv6
  *         0         -         -
  RAW       1         0         1
  UDP       1         1         0
  TCP       14        13        1
  INET      16        14        2
  FRAG      0         0         0

```
该测试的解释：在运行 server_orphan.py 和 client_orphan.py 之后，我们在
服务器和客户端之间建立了 64 个连接。运行 iptables 命令后，服务器会丢弃
来自客户端的所有报文；在 client_orphan.py 上输入 Ctrl-C，客户端系统会
尝试关闭这些连接，在它们被正常关闭之前，这些连接变成了孤儿 socket。由于
服务器的 iptables 阻断了来自客户端的报文，服务器不会收到来自客户端的
fin，因此客户端上的所有连接都会卡在 FIN_WAIT_1 阶段，从而作为孤儿 socket
一直保持到超时。我们将 10 写入 /proc/sys/net/ipv4/tcp_max_orphans，因此
客户端系统只会保留 10 个孤儿 socket，对于其余所有孤儿 socket，客户端系统
会向它们发送 RST 并将其删除。我们建立了 64 个连接，因此 'ss -s' 命令显示
系统有 10 个孤儿 socket，而 TcpExtTCPAbortOnMemory 的值为 54。

关于孤儿 socket 数量的补充说明：你可以通过 'ss -s' 命令找到精确的孤儿
socket 数量，但当内核决定是否增加 TcpExtTCPAbortOnMemory 并发送 RST 时，
内核并不总是检查精确的孤儿 socket 数量。为了提高性能，内核会先检查一个
近似计数，如果近似计数大于 tcp_max_orphans，内核才会再次检查精确计数。
因此，如果近似计数小于 tcp_max_orphans，但精确计数大于 tcp_max_orphans，
你会发现 TcpExtTCPAbortOnMemory 根本不会增加。如果 tcp_max_orphans 足够
大，这种情况不会发生；但如果你像我们的测试那样把 tcp_max_orphans 调小，
就可能会遇到这个问题。所以在我们的测试中，尽管 tcp_max_orphans 是 10，
客户端仍建立了 64 个连接。如果客户端只建立 11 个连接，我们就观察不到
TcpExtTCPAbortOnMemory 的变化。

继续前面的测试，我们等待几分钟。由于服务器上的 iptables 阻断了流量，
服务器不会收到 fin，客户端的全部孤儿 socket 最终都会在 FIN_WAIT_1
状态超时。所以我们等待几分钟后，可以发现
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnTimeout         10                 0.0

```
### TcpExtTCPAbortOnLinger
```

  nstatuser@nstat-b:~$ cat server_linger.py
  import socket
  import time

  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
```

  nstatuser@nstat-a:~$ cat client_linger.py
  import socket
  import struct

  server = 'nstat-b' # server address
  port = 9000

  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack('ii', 1, 10))
  s.setsockopt(socket.SOL_TCP, socket.TCP_LINGER2, struct.pack('i', -1))
  s.connect((server, port))
  s.close()

```
```

  nstatuser@nstat-b:~$ python3 server_linger.py

```
```

  nstatuser@nstat-a:~$ python3 client_linger.py

```
```

  nstatuser@nstat-a:~$ nstat | grep -i abort
  TcpExtTCPAbortOnLinger          1                  0.0

```
### TcpExtTCPRcvCoalesce

在服务器端，我们运行一个监听 TCP 9000 端口的程序，但
```

  import socket
  import time
  port = 9000
  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(('0.0.0.0', port))
  s.listen(1)
  sock, addr = s.accept()
  while True:
      time.sleep(9999999)

```
```

  python3 server_coalesce.py

```
```

  import socket
  server = 'nstat-b'
  port = 9000
  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.connect((server, port))

```
```

  nstatuser@nstat-a:~$ python3 -i client_coalesce.py

```
```

  >>> s.send(b'foo')
  3

```
```

  >>> s.send(b'bar')
  3

```
```

  ubuntu@nstat-b:~$ nstat
  #kernel
  IpInReceives                    2                  0.0
  IpInDelivers                    2                  0.0
  IpOutRequests                   2                  0.0
  TcpInSegs                       2                  0.0
  TcpOutSegs                      2                  0.0
  TcpExtTCPRcvCoalesce            1                  0.0
  IpExtInOctets                   110                0.0
  IpExtOutOctets                  104                0.0
  IpExtInNoECTPkts                2                  0.0

```
客户端发送了两个报文，服务器没有读取任何数据。当第二个报文到达服务器时，
第一个报文仍在接收队列中。因此 TCP 层合并了这两个报文，我们可以看到
TcpExtTCPRcvCoalesce 增加了 1。

### TcpExtListenOverflows 与 TcpExtListenDrops
```

  nstatuser@nstat-b:~$ nc -lkv 0.0.0.0 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
nc 命令只接受 1 个连接，且 accept 队列长度为 1。在当前的 linux 实现中，
将队列长度设为 n 意味着实际队列长度为 n+1。现在我们创建 3 个连接，其中
1 个被 nc 接受，2 个在 accept 队列中，因此 accept 队列已满。
```

  nstatuser@nstat-b:~$ nstat -n

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000

```
如果 nc 服务器运行在内核 4.10 或更高版本上，你不会看到 "Connection to ... succeeded!"
字符串，因为当 accept 队列已满时内核会丢弃 SYN。如果 nc 客户端运行在旧内核上，
你会看到连接成功了，因为内核会完成三次握手并将 socket 保留在半开队列中。
我在内核 4.15 上进行的测试。下面是 nstat
```

  nstatuser@nstat-b:~$ nstat
  #kernel
  IpInReceives                    4                  0.0
  IpInDelivers                    4                  0.0
  TcpInSegs                       4                  0.0
  TcpExtListenOverflows           4                  0.0
  TcpExtListenDrops               4                  0.0
  IpExtInOctets                   240                0.0
  IpExtInNoECTPkts                4                  0.0

```
TcpExtListenOverflows 和 TcpExtListenDrops 均为 4。如果第 4 个 nc 与
nstat 之间的时间间隔更长，TcpExtListenOverflows 和 TcpExtListenDrops 的
值会更大，因为第 4 个 nc 的 SYN 被丢弃了，客户端正在重试。

### IpInAddrErrors、IpExtInNoRoutes 与 IpOutNoRoutes

server A IP address: 192.168.122.250
server B IP address: 192.168.122.251
```

  $ sudo ip route add 8.8.8.8/32 via 192.168.122.251

```
```

  $ sudo sysctl -w net.ipv4.conf.all.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.ens3.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.lo.send_redirects=0
  $ sudo sysctl -w net.ipv4.conf.default.send_redirects=0

```
我们希望让 server A 向 8.8.8.8 发送一个报文，并将该报文路由到 server B。
当 server B 收到这样的报文时，它可能会向 server A 发送一个 ICMP Redirect
消息，将 send_redirects 设为 0 可以禁用此行为。
```

  $ sudo sysctl -w net.ipv4.conf.all.forwarding=0

```
```

  $ nc -v 8.8.8.8 53

```
```

  $ nstat
  #kernel
  IpInReceives                    3                  0.0
  IpInAddrErrors                  3                  0.0
  IpExtInOctets                   180                0.0
  IpExtInNoECTPkts                3                  0.0

```
由于我们让 server A 将 8.8.8.8 路由到 server B，并且我们在 server B 上
禁用了 IP 转发，server A 向 server B 发送报文后，server B 会丢弃这些报文
并增加 IpInAddrErrors。由于 nc 命令如果未收到 SYN+ACK 会重新发送 SYN 报文，
我们可以发现多个 IpInAddrErrors。

其次，生成 IpExtInNoRoutes。在 server B 上，我们启用 IP
```

  $ sudo sysctl -w net.ipv4.conf.all.forwarding=1

```
```

  $ ip route show
  default via 192.168.122.1 dev ens3 proto static
  192.168.122.0/24 dev ens3 proto kernel scope link src 192.168.122.251
  $ sudo ip route delete default via 192.168.122.1 dev ens3 proto static

```
```

  $ nc -v 8.8.8.8 53
  nc: connect to 8.8.8.8 port 53 (tcp) failed: Network is unreachable

```
```

  $ nstat
  #kernel
  IpInReceives                    1                  0.0
  IpOutRequests                   1                  0.0
  IcmpOutMsgs                     1                  0.0
  IcmpOutDestUnreachs             1                  0.0
  IcmpMsgOutType3                 1                  0.0
  IpExtInNoRoutes                 1                  0.0
  IpExtInOctets                   60                 0.0
  IpExtOutOctets                  88                 0.0
  IpExtInNoECTPkts                1                  0.0

```
我们在 server B 上启用了 IP 转发，当 server B 收到目的 IP 地址为 8.8.8.8 的
报文时，server B 会尝试转发该报文。由于我们已经删除了默认路由，没有通往
8.8.8.8 的路由，因此 server B 增加 IpExtInNoRoutes，并向 server A 发送
"ICMP Destination Unreachable" 消息。
```

  $ ping -c 1 8.8.8.8
  connect: Network is unreachable

```
```

  $ nstat
  #kernel
  IpOutNoRoutes                   1                  0.0

```
我们已在 server B 上删除了默认路由。server B 找不到通往 8.8.8.8 的路由，
因此 server B 增加了 IpOutNoRoutes。

### TcpExtTCPACKSkippedSynRecv

在本测试中，我们从客户端向服务器发送 3 个相同的 SYN 报文。第一个 SYN
会让服务器创建一个 socket，将其置为 Syn-Recv 状态，并回复 SYN/ACK。第二个
SYN 会让服务器再次回复 SYN/ACK，并记录回复时间（重复 ACK 的回复时间）。
第三个 SYN 会让服务器检查之前重复 ACK 的回复时间，并决定跳过该重复 ACK，
然后增加 TcpExtTCPACKSkippedSynRecv 计数器。
```

  nstatuser@nstat-a:~$ sudo tcpdump -c 1 -w /tmp/syn.pcap port 9000
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
```

  nstatuser@nstat-a:~$ nc nstat-b 9000

```
由于 nstat-b 没有监听 9000 端口，它应当回复一个 RST，nc 命令随即退出。
这足以让 tcpdump 命令捕获到一个 SYN 报文。linux 服务器可能会对 TCP 校验和
使用硬件卸载（hardware offload），因此 /tmp/syn.pcap 中的校验和
```

  nstatuser@nstat-a:~$ tcprewrite --infile=/tmp/syn.pcap --outfile=/tmp/syn_fixcsum.pcap --fixcsum

```
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
在 nstat-a 上，我们阻断了来自 9000 端口的报文，否则 nstat-a 会发送
```

  nstatuser@nstat-a:~$ sudo iptables -A INPUT -p tcp --sport 9000 -j DROP

```
```

  nstatuser@nstat-a:~$ for i in {1..3}; do sudo tcpreplay -i ens3 /tmp/syn_fixcsum.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedSynRecv      1                  0.0

```
正如预期，TcpExtTCPACKSkippedSynRecv 为 1。

### TcpExtTCPACKSkippedPAWS

要触发 PAWS，我们可以发送一个旧的 SYN。
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

```
```

  nstatuser@nstat-a:~$ sudo tcpdump -w /tmp/paws_pre.pcap -c 1 port 9000
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

```
现在 tcpdump 已经捕获到 SYN 并退出。我们应当修复
```

  nstatuser@nstat-a:~$ tcprewrite --infile /tmp/paws_pre.pcap --outfile /tmp/paws.pcap --fixcsum

```
```

  nstatuser@nstat-a:~$ for i in {1..2}; do sudo tcpreplay -i ens3 /tmp/paws.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedPAWS         1                  0.0

```
我们通过 tcpreplay 发送了两个 SYN，它们都会使 PAWS 检查失败，nstat-b 为
第一个 SYN 回复了一个 ACK，跳过了第二个 SYN 的 ACK，并更新了
TcpExtTCPACKSkippedPAWS。

### TcpExtTCPACKSkippedSeq

要触发 TcpExtTCPACKSkippedSeq，我们发送带有有效时间戳（以通过 PAWS 检查）
但序列号超出窗口的报文。linux 的 TCP 协议栈会在报文带数据时避免跳过，
因此我们需要一个纯 ACK 报文。要生成这样的报文，我们可以创建两个 socket：
一个在 9000 端口，另一个在 9001 端口。然后在 9001 端口上捕获一个 ACK，
将源/目的端口号改为匹配 9000 端口的 socket。接着我们就可以通过该报文
触发 TcpExtTCPACKSkippedSeq。

在 nstat-b 上，打开两个终端，运行两个 nc 命令分别监听
```

  nstatuser@nstat-b:~$ nc -lkv 9000
  Listening on [0.0.0.0] (family 0, port 9000)

  nstatuser@nstat-b:~$ nc -lkv 9001
  Listening on [0.0.0.0] (family 0, port 9001)

```
```

  nstatuser@nstat-a:~$ nc -v nstat-b 9000
  Connection to nstat-b 9000 port [tcp/*] succeeded!

  nstatuser@nstat-a:~$ nc -v nstat-b 9001
  Connection to nstat-b 9001 port [tcp/*] succeeded!

```
```

  nstatuser@nstat-a:~$ sudo tcpdump -w /tmp/seq_pre.pcap -c 1 dst port 9001
  tcpdump: listening on ens3, link-type EN10MB (Ethernet), capture size 262144 bytes

```
在 nstat-b 上，通过 9001 端口的 socket 发送一个报文。例如我们发送了一个
```

  nstatuser@nstat-b:~$ nc -lkv 9001
  Listening on [0.0.0.0] (family 0, port 9001)
  Connection from nstat-a 42132 received!
  foo

```
在 nstat-a 上，tcpdump 应当已经捕获到该 ACK。我们应当检查
```

  nstatuser@nstat-a:~$ ss -ta '( dport = :9000 || dport = :9001 )' | tee
  State  Recv-Q   Send-Q         Local Address:Port           Peer Address:Port
  ESTAB  0        0            192.168.122.250:50208       192.168.122.251:9000
  ESTAB  0        0            192.168.122.250:42132       192.168.122.251:9001

```
运行 tcprewrite，将 9001 端口改为 9000 端口，将 42132 端口改为
```

  nstatuser@nstat-a:~$ tcprewrite --infile /tmp/seq_pre.pcap --outfile /tmp/seq.pcap -r 9001:9000 -r 42132:50208 --fixcsum

```
```

  nstatuser@nstat-a:~$ for i in {1..2}; do sudo tcpreplay -i ens3 /tmp/seq.pcap; done

```
```

  nstatuser@nstat-b:~$ nstat | grep -i skip
  TcpExtTCPACKSkippedSeq          1                  0.0

```