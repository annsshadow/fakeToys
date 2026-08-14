
## UDP-Lite 协议（RFC 3828）


  UDP-Lite 是一个 IETF 标准轨道传输协议，其特征是可变长度的校验和。这对于通过无线网络传输多媒体（视频、VoIP）具有优势，因为部分损坏的数据包仍然可以送入编解码器，而不是因为校验和测试失败而被丢弃。

  本文件简要描述现有的内核支持以及套接字 API。深入了解信息，你可以查阅：

   - UDP-Lite 主页：
     http://web.archive.org/web/%2E/http://www.erg.abdn.ac.uk/users/gerrit/udp-lite/

     从这里你还可以下载一些示例应用程序源代码。

   - UDP-Lite HOWTO：
     http://web.archive.org/web/%2E/http://www.erg.abdn.ac.uk/users/gerrit/udp-lite/files/UDP-Lite-HOWTO.txt

   - Wireshark UDP-Lite WiKi（含抓包文件）：
     https://wiki.wireshark.org/Lightweight_User_Datagram_Protocol

   - 协议规范 RFC 3828：http://www.ietf.org/rfc/rfc3828.txt


## 1. 应用程序


  多个应用程序已成功移植到 UDP-Lite。Ethereal（现称 wireshark）默认支持 UDP-Litev4/v6。

  将应用程序移植到 UDP-Lite 很简单：只需更改套接字级别和 IPPROTO；发送方还需设置校验和覆盖长度（默认 = 头部长度 = 8）。细节在下一节。

## 2. 编程 API


  UDP-Lite 提供无连接、不可靠的数据报服务，因此使用与 UDP 相同的套接字类型。事实上，从 UDP 移植到 UDP-Lite 非常容易：只需将 `IPPROTO_UDPLITE` 作为最后一个参数加入
```

      s = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDPLITE);

  或者，分别对应地：

  ::

      s = socket(PF_INET6, SOCK_DGRAM, IPPROTO_UDPLITE);

```
  仅做上述更改，你就能够运行 UDP-Lite 服务或连接到 UDP-Lite 服务器。内核会假定你对使用部分校验和覆盖不感兴趣，从而模拟 UDP 模式（完全覆盖）。

  要使用部分校验和覆盖功能，需要设置一个单独的套接字选项，它接受一个指定覆盖长度的整数：

    * 发送方校验和覆盖：UDPLITE_SEND_CSCOV

      例如::

	int val = 20;
	setsockopt(s, SOL_UDPLITE, UDPLITE_SEND_CSCOV, &val, sizeof(int));

      将校验和覆盖长度设为 20 字节（12 字节数据 + 8 字节头部）。每个数据包中只有前 20 字节（加上伪头部）会被校验和。这对于具有 12 字节基头的 RTP 应用程序很有用。


    * 接收方校验和覆盖：UDPLITE_RECV_CSCOV

      此选项是接收方对应的部分。它是真正可选的，即并非启用部分校验和覆盖流量所必需。它的功能是作为流量过滤器：启用时，它指示内核丢弃所有覆盖长度_小于_此值的数据包。例如，如果要保护 RTP 和 UDP 头部，接收方可以强制只接收最小覆盖为 20 的数据包::

	int min = 20;
	setsockopt(s, SOL_UDPLITE, UDPLITE_RECV_CSCOV, &min, sizeof(int));

  getsockopt(2) 的调用与之类似。作为一个扩展而非独立协议，所有从 UDP 已知的套接字选项都可以以与以前完全相同的方式使用，例如 UDP_CORK 或 UDP_ENCAP。

  关于 UDP-Lite 校验和覆盖选项的详细讨论在第 IV 节。

```
## 3. 头文件


  套接字 API 需要通过 /usr/include 下的头文件获得支持：

    - /usr/include/netinet/in.h
      用于定义 IPPROTO_UDPLITE

    - /usr/include/netinet/udplite.h
      用于 UDP-Lite 头部字段和协议常量

```

    #define IPPROTO_UDPLITE       136
    #define SOL_UDPLITE           136
    #define UDPLITE_SEND_CSCOV     10
    #define UDPLITE_RECV_CSCOV     11

```
  各种发行版现成的头文件在 UDP-Lite tarball 中。

```
## 4. 内核关于各种套接字选项的行为


  要启用调试消息，需要将日志级别设为 8，因为大多数消息使用 KERN_DEBUG 级别（7）。

  1) 发送方套接字选项

  如果发送方指定覆盖长度为值 0，模块假定为完全覆盖，传输一个覆盖长度为 0 的数据包及相应的校验和。如果发送方指定的覆盖 < 8 且不为 0，内核假定 8 为默认值。最后，如果指定的覆盖长度超过数据包长度，则改用数据包长度作为覆盖长度。

  2) 接收方套接字选项

  接收方指定它愿意接受的最小覆盖长度值。此处值为 0 表示接收方总是希望整个数据包被覆盖。在这种情况下，所有部分覆盖的数据包都会被丢弃，并记录一个错误。

  不可能指定非法值（<0 和 <8）；在这些情况下假定默认为 8。

  所有覆盖值小于指定阈值到达的数据包都会被丢弃，这些事件也会被记录。

  3) 禁用校验和计算

  在发送方和接收方，校验和总是会被执行
```

	setsockopt(sockfd, SOL_SOCKET, SO_NO_CHECK,  ... );

```
  将总是被忽略，而::

	getsockopt(sockfd, SOL_SOCKET, SO_NO_CHECK, &value, ...);

```
  的值没有意义（如同在 TCP 中）。校验和字段为零的数据包是非法的（参见 RFC 3828 第 3.1 节），会被静默丢弃。

  4) 分片

  校验和计算同时考虑缓冲区大小和 MTU。UDP-Lite 数据包的大小由发送缓冲区的大小决定。发送缓冲区的最小大小为 2048（在 include/net/sock.h 中定义为 SOCK_MIN_SNDBUF），默认值可配置为 net.core.wmem_default，或通过设置 SO_SNDBUF socket(7) 选项。发送缓冲区的最大上限由 net.core.wmem_max 决定。

  给定大于发送缓冲区大小的负载大小，UDP-Lite 会将负载拆分为若干个独立的数据包，每种情况下填满发送缓冲区大小。

  确切的值还取决于接口 MTU。接口 MTU 反过来可能触发 IP 分片。在这种情况下，生成的 UDP-Lite 数据包被拆分为多个 IP 数据包，其中只有第一个包含 L4 头部。

  发送缓冲区大小对校验和覆盖长度有影响。考虑以下示例::

    Payload: 1536 bytes          Send Buffer:     1024 bytes
    MTU:     1500 bytes          Coverage Length:  856 bytes

```
  UDP-Lite 将把这 1536 字节分装在两个独立的数据包中::

    Packet 1: 1024 payload + 8 byte header + 20 byte IP header = 1052 bytes
    Packet 2:  512 payload + 8 byte header + 20 byte IP header =  540 bytes

```
  覆盖数据包覆盖第一个数据包中的 UDP-Lite 头部和 848 字节负载，第二个数据包被完全覆盖。注意对于第二个数据包，覆盖长度超过了数据包长度。内核在这种情况下总是将覆盖长度重新调整为数据包长度。

  作为一个 UDP-Lite 数据包被拆分为多个微小分片的例子，考虑以下示例::

    Payload: 1024 bytes            Send buffer size: 1024 bytes
    MTU:      300 bytes            Coverage length:   575 bytes

    +-+-----------+--------------+--------------+--------------+
    |8|    272    |      280     |     280      |     280      |
    +-+-----------+--------------+--------------+--------------+
		280            560            840           1032
					^
    *****checksum coverage*************

```
  UDP-Lite 模块生成一个 1032 字节的数据包（1024 + 8 字节头部）。根据接口 MTU，这些被拆分为 4 个 IP 数据包（280 字节 IP 负载 + 20 字节 IP 头部）。内核模块在对分片释放给 IP 模块之前，对前两个完整数据包的内容，加上最后一个数据包的 15 字节求和。

  要查看 IPv6 分片的类似情况，考虑链路 MTU 为 1280 字节、写缓冲区为 3356 字节。如果校验和覆盖小于 1232 字节（MTU 减去 IPv6/分片头部长度），只需考虑第一个分片。当使用更大的校验和覆盖长度时，每个符合条件的分片都需要被校验和。假设我们有一个 3062 的校验和覆盖。3356 字节的缓冲区将被拆分为以下分片::

    Fragment 1: 1280 bytes carrying  1232 bytes of UDP-Lite data
    Fragment 2: 1280 bytes carrying  1232 bytes of UDP-Lite data
    Fragment 3:  948 bytes carrying   900 bytes of UDP-Lite data

```
  前两个分片必须被完整校验和，最后一个分片中只有 598（= 3062 - 2*1232）字节被校验和。

  虽然正确处理此类情况很重要，但它们（令人讨厌地）罕见：UDP-Lite 设计用于优化通过无线（或一般噪声）链路的彩多媒体性能，因此更可能预期较小的覆盖长度。

```
## 5. UDP-Lite 运行时统计及其含义


  异常和错误条件以 KERN_DEBUG 级别记录到 syslog。关于 UDP-Lite 的实时统计可在 /proc/net/snmp 获取
```

			    netstat -svu

```
  这会显示 UDP-Lite 统计变量，其含义如下。

   ============     =====================================================
   InDatagrams      交付给用户的数据报总数。

   NoPorts          接收到一个未知端口的数据包数量。
		    这些情况被单独计数（不计入 InErrors）。

   InErrors         错误的 UDP-Lite 数据包数量。错误包括：

		      * 内部套接字队列接收错误
		      * 数据包太短（小于 8 字节，或声明的
			覆盖长度超过接收长度）
		      * xfrm4_policy_check() 返回错误
		      * 应用程序指定的最小覆盖长度大于
			入站数据包的覆盖长度
		      * 校验和覆盖被违反
		      * 错误的校验和

   OutDatagrams     已发送数据报的总数。
   ============     =====================================================

   这些统计来自 UDP MIB（RFC 2013）。

```
## 6. IPtables


  对 UDP-Lite 有数据包匹配支持，以及 LOG 目标的支持。
```

    udplite 136     UDP-Lite        # UDP-Lite [RFC 3828]

```
  然后::

	      iptables -A INPUT -p udplite -j LOG

```
  将产生输出到 syslog 的日志记录。丢弃和拒绝数据包也可工作。

```
## 7. 维护者地址


  UDP-Lite 补丁开发于

		    University of Aberdeen
		    Electronics Research Group
		    Department of Engineering
		    Fraser Noble Building
		    Aberdeen AB24 3UE; UK

```
  当前的维护者是 Gerrit Renker，<gerrit@erg.abdn.ac.uk>。初始代码由 William Stanislaus，<william@erg.abdn.ac.uk> 开发。
