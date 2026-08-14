
## TCP 认证选项 Linux 实现（RFC5925）


TCP 认证选项（TCP-AO）提供了一个旨在验证可信对等体之间报文段的 TCP 扩展。它添加
了一个新的 TCP 头部选项，其中包含一个消息认证码（MAC）。MAC 是使用一个双方都知道
口令的哈希函数，从 TCP 报文段的内容计算得出的。TCP-AO 的意图是废弃 TCP-MD5，提供
更好的安全性、密钥轮换以及对多种哈希算法的支持。

## 1. 简介


| | TCP-MD5 | TCP-AO |
|---|---|---|
| 支持的哈希算法 | MD5（密码学上较弱） | 必须支持 HMAC-SHA1（选择前缀攻击）和 CMAC-AES-128（仅旁路攻击）。可能支持任意哈希算法。 |
| MAC 长度（字节） | 16 | 通常 12-16。允许其它能放入 TCP 头部的变体。 |
| 每个 TCP 连接的密钥数 | 1 | 多个 |
| 更改活动密钥的可能性 | 不实用（双方都必须在 MSL 期间更改） | 协议支持 |
| 针对 ICMP “硬错误” 的保护 | 否 | 是：默认在已建立连接上忽略它们 |
| 针对流量交叉攻击的保护 | 否 | 是：伪头部包含 TCP 端口 |
| 针对重放 TCP 段的保护 | 否 | 序列号扩展（SNE）和初始序列号（ISN） |
| 支持无连接重置 | 是 | 否。需要 ISN+SNE 才能正确签署 RST |
| 标准 | RFC 2385 | RFC 5925、RFC 5926 |


### 1.1 经常问到的问题（FAQ），并引用 RFC 5925


Q：对于相同的 4 元组（srcaddr、srcport、dstaddr、dstport），SendID 或 RecvID 是否
可能不唯一？

```

   >> The IDs of MKTs MUST NOT overlap where their TCP connection
   identifiers overlap.

```
Q：能否移除活动连接的主密钥元组（MKT）？

```

   It is presumed that an MKT affecting a particular connection cannot
   be destroyed during an active connection -- or, equivalently, that
   its parameters are copied to an area local to the connection (i.e.,
   instantiated) and so changes would affect only new connections.

```
Q：如果需要删除一个旧的 MKT，应该如何操作才能不把它从活动连接上移除？（因为它在
之后任何时刻仍可能在使用）

A：RFC 5925 未指定，这似乎是密钥管理的问题，以确保在尝试移除之前没有人使用该 MKT。

Q：一个旧的 MKT 能否永远存在并被另一个对等体使用？

```

   Deciding when to start using a key is a performance issue. Deciding
   when to remove an MKT is a security issue. Invalid MKTs are expected
   to be removed. TCP-AO provides no mechanism to coordinate their removal,
   as we consider this a key management operation.

```
```

   The only way to avoid reuse of previously used MKTs is to remove the MKT
   when it is no longer considered permitted.

```
Linux TCP-AO 会尽力阻止你移除正在使用的密钥，将其视为密钥管理失败。但由于保留过时
的密钥可能会成为安全问题，并且对等体可能通过始终将其设置为 RNextKeyID 而无意中阻止
移除旧密钥——因此提供了一种强制密钥移除机制，用户空间必须提供要使用的 KeyID 来替代
正在被移除的那个，内核会原子地删除旧密钥，即使对等体仍在请求它。强制删除没有任何
保证，因为对等体可能还没有新密钥——TCP 连接可能就此中断。或者，可以选择关闭套接字。

Q：当在一个新连接上接收到一个没有已知 MKT 的 RecvID 的报文段时，会发生什么？

A：RFC 5925 规定默认情况下是接受它并记录警告，但
```

   If the segment is a SYN, then this is the first segment of a new
   connection. Find the matching MKT for this segment, using the segment's
   socket pair and its TCP-AO KeyID, matched against the MKT's TCP connection
   identifier and the MKT's RecvID.

      i. If there is no matching MKT, remove TCP-AO from the segment.
         Proceed with further TCP handling of the segment.
         NOTE: this presumes that connections that do not match any MKT
         should be silently accepted, as noted in Section 7.3.

```
```

   >> A TCP-AO implementation MUST allow for configuration of the behavior
   of segments with TCP-AO but that do not match an MKT. The initial default
   of this configuration SHOULD be to silently accept such connections.
   If this is not the desired case, an MKT can be included to match such
   connections, or the connection can indicate that TCP-AO is required.
   Alternately, the configuration can be changed to discard segments with
   the AO option not matching an MKT.

```
```

   Connections not matching any MKT do not require TCP-AO. Further, incoming
   segments with TCP-AO are not discarded solely because they include
   the option, provided they do not match any MKT.

```
请注意，Linux TCP-AO 实现在这方面有所不同。目前，签名未知的 TCP-AO 报文段会被丢弃
并记录警告。

Q：RFC 是否以任何方式暗示集中的内核密钥管理？（即是否要求所有连接上的密钥必须同时
轮换？）

A：未指定。MKT 可以在用户空间管理，唯一相关的部分是
```

   >> All TCP segments MUST be checked against the set of MKTs for matching
   TCP connection identifiers.

```
Q：当对等体请求的 RNextKeyID 未知时会发生什么？是否应该重置连接？

```

   ii. If they differ, determine whether the RNextKeyID MKT is ready.

       1. If the MKT corresponding to the segment’s socket pair and RNextKeyID
       is not available, no action is required (RNextKeyID of a received
       segment needs to match the MKT’s SendID).

```
Q：current_key 是如何设置的，何时会变化？它是用户触发的更改，还是由远端对等体的
请求触发？是由用户显式设置，还是由匹配规则设置？

```

   Rnext_key is changed only by manual user intervention or MKT management
   protocol operation. It is not manipulated by TCP-AO. Current_key is updated
   by TCP-AO when processing received TCP segments as discussed in the segment
   processing description in Section 7.5. Note that the algorithm allows
   the current_key to change to a new MKT, then change back to a previously
   used MKT (known as "backing up"). This can occur during an MKT change when
   segments are received out of order, and is considered a feature of TCP-AO,
   because reordering does not result in drops.

```
```

   2. If the matching MKT corresponding to the segment’s socket pair and
   RNextKeyID is available:

      a. Set current_key to the RNextKeyID MKT.

```
Q：如果两个对等体都有多个匹配该连接套接字对的 MKT（具有不同的 KeyID），发送方/
接收方应如何选择要使用的 KeyID？

```

   Multiple MKTs may match a single outgoing segment, e.g., when MKTs
   are being changed. Those MKTs cannot have conflicting IDs (as noted
   elsewhere), and some mechanism must determine which MKT to use for each
   given outgoing segment.

   >> An outgoing TCP segment MUST match at most one desired MKT, indicated
   by the segment’s socket pair. The segment MAY match multiple MKTs, provided
   that exactly one MKT is indicated as desired. Other information in
   the segment MAY be used to determine the desired MKT when multiple MKTs
   match; such information MUST NOT include values in any TCP option fields.

```
Q：TCP-MD5 连接能否迁移到 TCP-AO（反之亦然）？

```

   TCP MD5-protected connections cannot be migrated to TCP-AO because TCP MD5
   does not support any changes to a connection’s security algorithm
   once established.

```
Q：如果从连接上移除了所有 MKT，它能否变成非 TCP-AO 签名的连接？

A：[7.5.2] 没有像 [7.5.1.i] 中 SYN 包处理那样的选项（该选项会允许接受无签名的
报文段，那将是不安全的）。虽然切换到非 TCP-AO 连接并未被直接禁止，但这似乎是 RFC
的意图。此外，TCP-AO 连接要求
```

   TCP-AO requires that every protected TCP segment match exactly one MKT.

```
```

   >> An incoming TCP segment including TCP-AO MUST match exactly one MKT,
   indicated solely by the segment’s socket pair and its TCP-AO KeyID.

```
```

   One or more MKTs. These are the MKTs that match this connection’s
   socket pair.

```
Q：非 TCP-AO 连接能否变成启用 TCP-AO 的连接？

A：不能：对于一个已经建立的、非 TCP-AO 连接，不可能切换到使用 TCP-AO，因为流量密钥
的生成需要初始序列号。换句话说，开始使用 TCP-AO 需要重新建立 TCP 连接。

## 2. 内核内 MKT 数据库与用户空间数据库对比


Linux TCP-AO 支持使用 `setsockopt()` 实现，方式与 TCP-MD5 类似。这意味着想要使用
TCP-AO 的用户空间应用程序应该在 TCP 套接字上执行 `setsockopt()` 来添加、移除或轮换
MKT。这种方法将密钥管理的责任以及对边界情况（例如，如果对等体不遵守 RNextKeyID 该
怎么办）的决策移到了用户空间，即将更多代码移到了用户空间，特别是负责策略决策的代码。
此外，它灵活且可良好扩展（与内核内数据库相比需要更少的锁）。还应记住，主要的目标用户
是 BGP 进程，而不是任何随机应用程序，这意味着与 IPsec 隧道相比，实际上不需要透明性，
而现代 BGP 守护进程已经有了用于 TCP-MD5 支持的 `setsockopt()`。

| | `setsockopt()` | 内核内数据库 |
|---|---|---|
| 可扩展性 | `setsockopt()` 命令应是可扩展的系统调用 | Netlink 消息简单且可扩展 |
| 所需的用户空间改动 | 想要 TCP-AO 的 BGP 或任何应用程序需要执行 `setsockopt()` 并进行密钥管理 | 可以像隧道一样透明，提供类似 `ip tcpao add key`（删除/显示/轮换）的功能 |
| MKT 的移除或添加 | 对用户空间更难 | 对内核更难 |
| 可转储性 | `getsockopt()` | Netlink .dump() 回调 |
| 内核资源/内存限制 | 相等 | 相等 |
| 可扩展性 | `TCP_LISTEN` 套接字上的争用 | 整个数据库上的争用 |
| 监控与警告 | `TCP_DIAG` | 相同的 Netlink 套接字 |
| MKT 匹配 | 半个问题：仅监听套接字 | 困难 |


## 3. uAPI


Linux 提供了一组 `setsockopt()` 和 `getsockopt()`，让用户空间能够在每个套接字的基础上
管理 TCP-AO。为了添加/删除 MKT，必须使用 `TCP_AO_ADD_KEY` 和 `TCP_AO_DEL_KEY` TCP
套接字选项。不允许在已建立的非 TCP-AO 连接上添加密钥，也不允许从 TCP-AO 连接上移除
最后一个密钥。

**`setsockopt(TCP_AO_DEL_KEY)` 命令可以指定 ``tcp_ao_del**
：current_key``
- `tcp_ao_del::set_current` 和/或 `tcp_ao_del::rnext`
- `tcp_ao_del::set_rnext`，它使此类删除成为 “强制” 的：它为用户空间提供了一种方式来
删除正在使用的密钥，并原子地设置一个替代密钥。这并非用于正常使用，只应在对等体忽略
RNextKeyID 并持续请求/使用旧密钥时使用。它提供了一种强制删除不受信任密钥的方法，但
可能会中断 TCP-AO 连接。

通常/正常的密钥轮换可以使用 `setsockopt(TCP_AO_INFO)` 执行。它还提供了一个 uAPI 来
更改每套接字的 TCP-AO 设置，例如忽略 ICMP，以及清除每套接字的 TCP-AO 报文计数器。
相应的 `getsockopt(TCP_AO_INFO)` 可用于获取这些每套接字的 TCP-AO 设置。

另一个有用的命令是 `getsockopt(TCP_AO_GET_KEYS)`。可以使用它列出 TCP 套接字上的所有
MKT，或者使用过滤器来获取特定对等体以及/或 sndid/rcvid、VRF L3 接口或 current_key/
rnext_key 的密钥。

为了修复 TCP-AO 连接，`setsockopt(TCP_AO_REPAIR)` 可用，前提是用户之前已经使用
`getsockopt(TCP_AO_REPAIR)` 对套接字进行了检查点/转储。

对于具有数千个 TCP-AO 密钥的、规模化的 TCP_LISTEN 套接字，一个建议是：在
`getsockopt(TCP_AO_GET_KEYS)` 中使用过滤器，并使用 `setsockopt(TCP_AO_DEL_KEY)` 进行
异步删除。

Linux TCP-AO 还提供了一组报文段计数器，有助于排查/调试问题。每个 MKT 都有 good/bad
计数器，反映有多少报文通过了/未通过验证。每个 TCP-AO 套接字具有以下计数器：
- 针对正常报文段（正确签名的）
- 针对错误报文段（TCP-AO 验证失败的）
- 针对使用未知密钥的报文段
- 针对期望有 AO 签名但未找到的报文段
- 针对被忽略的 ICMP 数量

TCP-AO 每套接字计数器也与每网络命名空间（netns）计数器一起重复，通过 SNMP 暴露。它们
是 `TCPAOGood`、`TCPAOBad`、`TCPAOKeyNotFound`、`TCPAORequired` 和 `TCPAODroppedIcmps`。

出于监控目的，有以下 TCP-AO 跟踪事件：`tcp_hash_bad_header`、`tcp_hash_ao_required`、
`tcp_ao_handshake_failure`、`tcp_ao_wrong_maclen`、`tcp_ao_wrong_maclen`、
`tcp_ao_key_not_found`、`tcp_ao_rnext_request`、`tcp_ao_synack_no_key`、
`tcp_ao_snd_sne_update`、`tcp_ao_rcv_sne_update`。可以单独启用它们中的任意一个，并
可以按网络命名空间、4 元组、族、L3 索引和 TCP 头部标志进行过滤。如果报文段带有
TCP-AO 头部，过滤器还可以包含 keyid、rnext 和 maclen。SNE 更新包含翻转的数字。

RFC 5925 非常宽松地规定了如何对 TCP 端口进行匹配
```

   TCP connection identifier. A TCP socket pair, i.e., a local IP
   address, a remote IP address, a TCP local port, and a TCP remote port.
   Values can be partially specified using ranges (e.g., 2-30), masks
   (e.g., 0xF0), wildcards (e.g., "*"), or any other suitable indication.

```
目前 Linux TCP-AO 实现不提供任何 TCP 端口匹配。也许端口范围对于 uAPI 来说最灵活，但
到目前为止尚未实现。

## 4. ``setsockopt()`` 与 ``accept()`` 的竞争


与只有一个密钥的已建立 TCP-MD5 连接不同，TCP-AO 连接可能有很多密钥，这意味着监听
套接字上被接受的连接也可能有任意数量的密钥。由于在一个首个正确签名的 SYN 上复制所有
这些密钥会使请求套接字变大，这是不期望的。目前，实现不会将密钥复制到请求套接字，而是
在 “父” 监听套接字上查找它们。

其结果是，当用户空间移除 TCP-AO 密钥时，可能会破坏请求套接字上尚未建立的连接，以及
不会从已经建立但尚未被 `accept()` 的连接（悬挂在 accept 队列中）上移除密钥。

反之亦然：如果用户空间在监听套接字上为某个对等体添加了一个新密钥，那么 accept 队列中
已建立的套接字将不会有这些新密钥。

目前，这两种竞争的解决方案：
`setsockopt(TCP_AO_ADD_KEY)` 与 `accept()` 之间的竞争，
以及 `setsockopt(TCP_AO_DEL_KEY)` 与 `accept()` 之间的竞争，被委托给用户空间。这意味着
期望用户空间检查由 `accept()` 返回的套接字上的 MKT，以验证监听套接字上发生的任何密钥
轮换是否反映在新建立的连接上。

这与内核侧对 TCP-MD5 的 “do-nothing”（不做任何事）方法类似，以后可能会通过为
`tcp_ao_add` 和 `tcp_ao_del` 引入新标志来改变。

请注意，这种竞争很少见，因为它需要新的 TCP 连接的 3 次握手期间发生 TCP-AO 密钥轮换。

## 5. 与 TCP-MD5 的交互


TCP 连接不能在 TCP-AO 和 TCP-MD5 选项之间迁移。已经建立了带有 AO 或 MD5 密钥的套接字
被限制为不能添加另一种选项的密钥。

对于监听套接字，情况则不同：BGP 服务器可能希望同时接收 TCP-AO 和（已弃用的）TCP-MD5
客户端。因此，两种类型的密钥都可以添加到 TCP_CLOSED 或 TCP_LISTEN 套接字上。不允许
为同一个对等体添加不同类型的密钥。

## 6. SNE 的 Linux 实现


RFC 5925 [6.2] 描述了如何用 SNE 扩展 TCP 序列号的算法。简而言之：TCP 必须跟踪先前的
序列号，并在当前 SEQ 号翻转时设置 sne_flag。当当前和先前的 SEQ 号都越过 0x7fff（即
32Kb）时，该标志被清除。

在 sne_flag 被置位的期间，算法将每个报文的 SEQ 与 0x7fff 比较，如果它高于 32Kb，则
假定该报文应该用递增之前的 SNE 来验证。结果，存在这个 [0; 32Kb] 的窗口，在此期间可以
接受带有（SNE - 1）的报文。

Linux 实现对此做了一些简化：由于网络栈已经跟踪了期望 ACK 的第一个 SEQ 字节（snd_una）
和期望的下一个 SEQ 字节（rcv_nxt）——这足以粗略估计发送方和接收方在 4GB SEQ 号空间中
的位置。当它们翻转到零时，相应的 SNE 会递增。

tcp_ao_compute_sne() 对每个 TCP-AO 报文段调用。它将报文中的 SEQ 号与 snd_una 或
rcv_nxt 比较，并将结果适配到它们周围 2GB 的窗口中，从而检测 SEQ 号的翻转。这大大简化
了代码，并且只需要在每个 TCP-AO 套接字上存储 SNE 号。

2GB 窗口乍看之下似乎比 RFC 5926 宽松得多。但它只用于在翻转之前/之后选择正确的 SNE。它
允许更多的 TCP 报文段重放，但在已验证的报文段上仍然会应用 tcp_sequence() 中的所有常规
TCP 检查。因此，它用对重放/重传报文段的稍宽松接受，换取了算法的简单性以及对大 TCP
窗口似乎更好的行为。

## 7. 链接


RFC 5925 The TCP Authentication Option
   https://www.rfc-editor.org/rfc/pdfrfc/rfc5925.txt.pdf

RFC 5926 Cryptographic Algorithms for the TCP Authentication Option (TCP-AO)
   https://www.rfc-editor.org/rfc/pdfrfc/rfc5926.txt.pdf

草案 “SHA-2 Algorithm for the TCP Authentication Option (TCP-AO)”
   https://datatracker.ietf.org/doc/html/draft-nayak-tcp-sha2-03

RFC 2385 Protection of BGP Sessions via the TCP MD5 Signature Option
   https://www.rfc-editor.org/rfc/pdfrfc/rfc2385.txt.pdf

:Author: Dmitry Safonov <dima@arista.com>
