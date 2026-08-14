
## 设备内存 TCP（Device Memory TCP）


## 简介


设备内存 TCP（devmem TCP）支持将数据直接接收到设备内存（dmabuf）中。该特性当前针对 TCP 套接字实现。


### 机会


大量的数据传输以设备内存作为源和/或目的地。加速器极大地增加了此类传输的普遍性。一些例子包括：

- 分布式训练，其中位于不同主机上的 ML 加速器（如 GPU）之间交换数据。

- 分布式裸块存储应用与远程 SSD 之间传输大量数据。其中大部分数据不需要主机进行处理。

通常，网络中的设备到设备数据传输被实现为以下低级操作：设备到主机拷贝、主机到主机网络传输，以及主机到设备拷贝。

涉及主机拷贝的数据流并非最优，特别是对于批量数据传输，并且会给系统资源（如主机内存带宽和 PCIe 带宽）带来显著压力。

Devmem TCP 通过实现套接字 API 来优化此用例，使用户能够将接收到的网络数据包直接放入设备内存。

数据包载荷直接从 NIC 进入设备内存。

数据包头部进入主机内存，并由 TCP/IP 协议栈正常处理。NIC 必须支持头部分离（header split）才能实现这一点。

优点：

- 与现有的"网络传输 + 设备拷贝"语义相比，缓解主机内存带宽压力。

- 通过将数据传输限制在 PCIe 树的最低层级，缓解 PCIe 带宽压力，而传统路径会将数据经由根复杂体（root complex）发送。


### 更多信息


  slides、视频
    https://netdevconf.org/0x17/sessions/talk/device-memory-tcp.html

  patchset
    [PATCH net-next v24 00/13] Device Memory TCP
    https://lore.kernel.org/netdev/20240831004313.3713467-1-almasrymina@google.com/


## RX 接口


### 示例


./tools/testing/selftests/drivers/net/hw/ncdevmem:do_server 展示了设置此 API 的 RX 路径的示例。


### NIC 设置


头部分离、流导向（flow steering）和 RSS 是 devmem TCP 所需的功能。

头部分离用于将传入的数据包拆分为位于主机内存中的头部缓冲区，以及位于设备内存中的载荷缓冲区。

流导向和 RSS 用于确保只有以 devmem 为目标的数据流才会落在绑定到 devmem 的 RX 队列上。

```

	# 启用头部分离
	ethtool -G eth1 tcp-data-split on


	# 启用流导向
	ethtool -K eth1 ntuple on

```
配置 RSS 以将所有流量从目标 RX 队列（queue 15）引开，方法是
```

	ethtool --set-rxfh-indir eth1 equal 15


```
用户必须使用以下方式将 dmabuf 绑定到给定 NIC 上的任意数量的 RX 队列
```

	/* 将 dmabuf 绑定到 NIC RX 队列 15 */
	struct netdev_queue *queues;
	queues = malloc(sizeof(*queues) * 1);

	queues[0]._present.type = 1;
	queues[0]._present.idx = 1;
	queues[0].type = NETDEV_RX_QUEUE_TYPE_RX;
	queues[0].idx = 15;

	*ys = ynl_sock_create(&ynl_netdev_family, &yerr);

	req = netdev_bind_rx_req_alloc();
	netdev_bind_rx_req_set_ifindex(req, 1 /* ifindex */);
	netdev_bind_rx_req_set_dmabuf_fd(req, dmabuf_fd);
	__netdev_bind_rx_req_set_queues(req, queues, n_queue_index);

	rsp = netdev_bind_rx(*ys, req);

	dmabuf_id = rsp->dmabuf_id;


```
netlink API 返回一个 dmabuf_id：一个引用此已绑定 dmabuf 的唯一 ID。

用户可以通过关闭建立绑定的 netlink 套接字来将 dmabuf 从网络设备解绑。我们这样做是为了即使 userspace 进程崩溃，绑定也会自动解除。

请注意，任何来自任意导出方（exporter）的行为良好的 dmabuf 都应该能与 devmem TCP 配合工作，即使该 dmabuf 实际上并非由设备内存支持。udmabuf 就是这样一个例子，它将用户内存（非设备内存）包装在 dmabuf 中。


### 套接字设置


```

	ethtool -N eth1 flow-type tcp4 ... queue 15


```
### 接收数据


用户应用程序必须向内核表明其能够接收
```

	ret = recvmsg(fd, &msg, MSG_SOCK_DEVMEM);

```
未指定 MSG_SOCK_DEVMEM 标志的应用程序在接收 devmem 数据时将收到 EFAULT。

Devmem 数据被直接接收到绑定到 NIC 的 dmabuf 中，位于"NIC
```

		for (cm = CMSG_FIRSTHDR(&msg); cm; cm = CMSG_NXTHDR(&msg, cm)) {
			if (cm->cmsg_level != SOL_SOCKET ||
				(cm->cmsg_type != SCM_DEVMEM_DMABUF &&
				 cm->cmsg_type != SCM_DEVMEM_LINEAR))
				continue;

			dmabuf_cmsg = (struct dmabuf_cmsg *)CMSG_DATA(cm);

			if (cm->cmsg_type == SCM_DEVMEM_DMABUF) {
				/* 分片落在 dmabuf 中。
				 *
				 * dmabuf_cmsg->dmabuf_id 是该分片
				 * 所落入的 dmabuf。
				 *
				 * dmabuf_cmsg->frag_offset 是该分片
				 * 在 dmabuf 中起始的偏移。
				 *
				 * dmabuf_cmsg->frag_size 是分片
				 * 的大小。
				 *
				 * dmabuf_cmsg->frag_token 是一个令牌，
				 * 用于稍后释放此分片时引用它。
				 */

				struct dmabuf_token token;
				token.token_start = dmabuf_cmsg->frag_token;
				token.token_count = 1;
				continue;
			}

			if (cm->cmsg_type == SCM_DEVMEM_LINEAR)
				/* 分片落在线性缓冲区中。
				 *
				 * dmabuf_cmsg->frag_size 是分片
				 * 的大小。
				 */
				continue;

		}

```
应用程序可能收到 2 个 cmsgs：

- SCM_DEVMEM_DMABUF：这表示分片落在由 dmabuf_id 指示的 dmabuf 中。

- SCM_DEVMEM_LINEAR：这表示分片落在线性缓冲区中。当 NIC 无法在头部边界处拆分数据包，导致部分（或全部）载荷落入主机内存时，通常会发生这种情况。

应用程序可能收不到任何 SO_DEVMEM_* cmsgs。这表示落在未绑定到 dmabuf 的 RX 队列上的非 devmem 常规 TCP 数据。


### 释放分片


通过 SCM_DEVMEM_DMABUF 接收的分片在用户处理该分片期间被内核锁定（pinned）。用户必须通过以下方式将分片返还给内核
```

	ret = setsockopt(client_fd, SOL_SOCKET, SO_DEVMEM_DONTNEED, &token,
			 sizeof(token));

```
用户必须确保及时将令牌返还给内核。否则将耗尽绑定到 RX 队列的有限 dmabuf，并导致丢包。

用户传递的令牌不得超过 128 个，且所有令牌的 token->token_count 合计不得超过 1024 个分片。如果用户提供的分片超过 1024 个，内核将释放最多 1024 个分片并提前返回。

内核返回实际释放的分片数量。在以下情况下，释放的分片数可能少于用户提供的令牌数量：

(a) 内核内部泄漏 bug。
(b) 用户传递了超过 1024 个分片。


## TX 接口


### 示例


./tools/testing/selftests/drivers/net/hw/ncdevmem:do_client 展示了设置此 API 的 TX 路径的示例。


### NIC 设置


```

        struct netdev_bind_tx_req *req = NULL;
        struct netdev_bind_tx_rsp *rsp = NULL;
        struct ynl_error yerr;

        *ys = ynl_sock_create(&ynl_netdev_family, &yerr);

        req = netdev_bind_tx_req_alloc();
        netdev_bind_tx_req_set_ifindex(req, ifindex);
        netdev_bind_tx_req_set_fd(req, dmabuf_fd);

        rsp = netdev_bind_tx(*ys, req);

        tx_dmabuf_id = rsp->id;


```
netlink API 返回一个 dmabuf_id：一个引用此已绑定 dmabuf 的唯一 ID。

用户可以通过关闭建立绑定的 netlink 套接字来将 dmabuf 从网络设备解绑。我们这样做是为了即使 userspace 进程崩溃，绑定也会自动解除。

请注意，任何来自任意导出方的行为良好的 dmabuf 都应该能与 devmem TCP 配合工作，即使该 dmabuf 实际上并非由设备内存支持。udmabuf 就是这样一个例子，它将用户内存（非设备内存）包装在 dmabuf 中。


### 套接字设置


用户在发送 devmem TCP 时必须使用 MSG_ZEROCOPY 标志。Devmem 无法被内核拷贝，因此 devmem TX 的语义类似于
```

	setsockopt(socket_fd, SOL_SOCKET, SO_ZEROCOPY, &opt, sizeof(opt));

```
还建议用户将 TX 套接字绑定到同一接口
```

	setsockopt(socket_fd, SOL_SOCKET, SO_BINDTODEVICE, ifname, strlen(ifname) + 1);


```
### 发送数据


Devmem 数据使用 SCM_DEVMEM_DMABUF cmsg 发送。

用户应创建一个 msghdr，其中：

- iov_base 设置为 dmabuf 中开始发送的偏移
- iov_len 设置为要从 dmabuf 发送的字节数

用户通过 dmabuf_tx_cmsg.dmabuf_id 传递要从中发送的 dma-buf id。

下面的示例从 dmabuf 的偏移 100 处发送 1024 字节，以及从偏移 2000 处发送 2048 字节
```

       char ctrl_data[CMSG_SPACE(sizeof(struct dmabuf_tx_cmsg))];
       struct dmabuf_tx_cmsg ddmabuf;
       struct msghdr msg = {};
       struct cmsghdr *cmsg;
       struct iovec iov[2];

       iov[0].iov_base = (void*)100;
       iov[0].iov_len = 1024;
       iov[1].iov_base = (void*)2000;
       iov[1].iov_len = 2048;

       msg.msg_iov = iov;
       msg.msg_iovlen = 2;

       msg.msg_control = ctrl_data;
       msg.msg_controllen = sizeof(ctrl_data);

       cmsg = CMSG_FIRSTHDR(&msg);
       cmsg->cmsg_level = SOL_SOCKET;
       cmsg->cmsg_type = SCM_DEVMEM_DMABUF;
       cmsg->cmsg_len = CMSG_LEN(sizeof(struct dmabuf_tx_cmsg));

       ddmabuf.dmabuf_id = tx_dmabuf_id;

       *((struct dmabuf_tx_cmsg *)CMSG_DATA(cmsg)) = ddmabuf;

       sendmsg(socket_fd, &msg, MSG_ZEROCOPY);


```
### 复用 TX dmabuf


与常规内存的 MSG_ZEROCOPY 类似，用户在发送操作进行期间不应修改 dma-buf 的内容。这是因为内核不会保留 dmabuf 内容的副本。相反，内核会锁定（pin）并发送 userspace 可用的缓冲区中的数据。

正如 MSG_ZEROCOPY 一样，内核通过以下方式通知 userspace 发送完成
```

        int64_t tstop = gettimeofday_ms() + waittime_ms;
        char control[CMSG_SPACE(100)] = {};
        struct sock_extended_err *serr;
        struct msghdr msg = {};
        struct cmsghdr *cm;
        int retries = 10;
        __u32 hi, lo;

        msg.msg_control = control;
        msg.msg_controllen = sizeof(control);

        while (gettimeofday_ms() < tstop) {
                if (!do_poll(fd)) continue;

                ret = recvmsg(fd, &msg, MSG_ERRQUEUE);

                for (cm = CMSG_FIRSTHDR(&msg); cm; cm = CMSG_NXTHDR(&msg, cm)) {
                        serr = (void *)CMSG_DATA(cm);

                        hi = serr->ee_data;
                        lo = serr->ee_info;

                        fprintf(stdout, "tx complete [%d,%d]\n", lo, hi);
                }
        }

```
相应的 sendmsg 完成后，dmabuf 即可被 userspace 复用。


## 实现与注意事项


### 不可读 skb


Devmem 载荷对处理数据包的内核是不可访问的。这导致 devmem skb 的载荷出现一些怪异行为：

- 回环（Loopback）功能不可用。回环依赖拷贝载荷，而这对 devmem skb 是不可能的。

- 软件校验和计算失败。

- TCP Dump 和 bpf 无法访问 devmem 数据包载荷。


## 测试


更真实的示例代码可以在内核源码中的 `tools/testing/selftests/drivers/net/hw/ncdevmem.c` 下找到。

ncdevmem 是一个 devmem TCP 版的 netcat。它的工作方式与 netcat 非常相似，但会将数据直接接收到 udmabuf 中。

要运行 ncdevmem，你需要在被测机器上的服务器上运行它，并且需要在对端运行 netcat 来提供 TX 数据。

ncdevmem 还有一个验证模式，期望接收重复模式的数据并据此进行验证。例如，你可以启动
```

	ncdevmem -s <server IP> -c <client IP> -f <ifname> -l -p 5201 -v 7

```
在客户端，使用常规 netcat 向 ncdevmem 进程发送 TX 数据
```

	yes $(echo -e \\x01\\x02\\x03\\x04\\x05\\x06) | \
		tr \\n \\0 | head -c 5G | nc <server IP> 5201 -p 5201

```
