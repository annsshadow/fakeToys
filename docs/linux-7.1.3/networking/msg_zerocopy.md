
## MSG_ZEROCOPY


## 简

MSG_ZEROCOPY 标志socket send 调用启用了避免拷贝（copy avoidance）。该特性目前针TCP、UDP VSOCK（使virtio 传输）socket 实现

### 机会与注意事

在用户进程与内核之间拷贝大缓冲区可能代价高昂。Linux 支持各种避免拷贝的接口，sendfile splice。MSG_ZEROCOPY 标志将底层的避免拷贝机制扩展到普通的 socket send 调用
避免拷贝并非免费的午餐。在当前的实现中，通过页固定（page pinning），它用页记账和完成通知的开销替换了每字节的拷贝开销。因此，MSG_ZEROCOPY 通常只在大约 10 KB 以上的写入时才有效
页固定也改变了系统调用的语义。它在进程和网络栈之间临时共享缓冲区。与拷贝不同，进程不能在系统调用返回后立即覆盖缓冲区，否则可能修改正在传输中的数据。内核完整性不受影响，但有 bug 的程序可能会破坏自己的数据流
当修改数据是安全的时，内核会返回一个通知。因此，将现有应用程序转换为 MSG_ZEROCOPY 并不总是像只传入该标志那么简单

### 更多信息


本文档的大部分内容源自在 netdev 2.1 上发表的一篇较长的论文。有关更深入的信息，请参阅该论文和演讲、LWN.net 上出色的报道，或阅读原始代码
  paper, slides, video
    https://netdevconf.org/2.1/session.html?debruijn

  LWN article
    https://lwn.net/Articles/726917/

  patchset
    [PATCH net-next v4 0/9] socket sendmsg MSG_ZEROCOPY
    https://lore.kernel.org/netdev/20170803202945.70750-1-willemdebruijn.kernel@gmail.com


## 接口


传入 MSG_ZEROCOPY 标志是启用避免拷贝最明显的步骤，但不是唯一的一步
### Socket 设置


当应用程序向 send 系统调用传入未定义的标志时，内核是宽松的。默认情况下它只是忽略它们。为了避免为那些意外已经传入此标志的旧进程启用避免拷贝模式，进程必须首先通过设置 socket 选项来表明意图：

```

	if (setsockopt(fd, SOL_SOCKET, SO_ZEROCOPY, &one, sizeof(one)))
		error(1, errno, "setsockopt zerocopy");

```
### 发

send（或 sendto、sendmsg、sendmmsg）本身的更改是微不足道的。传入新标志
```

	ret = send(fd, buf, sizeof(buf), MSG_ZEROCOPY);

```
zerocopy 失败将返-1 errno ENOBUFS。如socket 超过optmem 限制，或用户超过其锁定页ulimit，就会发生这种情况

#### 混合避免拷贝与拷

许多工作负载混合有大缓冲区和小缓冲区。因为对于小数据包而言，避免拷贝比拷贝更昂贵，所以该特性被实现为一个标志。将带标志的调用与不带标志的调用混合使用是安全的

### 通知


当复用先前传入的缓冲区是安全的时，内核必须通知进程。它socket 错误队列上排队完成通知，类似于发送时间戳接口
通知本身是一个简单的标量值。每socket 维护一个内部的无符32 位计数器。每次成功发送数据的、带 MSG_ZEROCOPY send 调用都会使计数器递增。在失败或以零长度调用时，计数器不会递增。计数器统计的是系统调用的调用次数，而不是字节数。它UINT_MAX 次调用后回绕

#### 通知接收


以下片段演示API。在最简单的情况下，每次 send 系统调用之后都跟一个在错误队列上的 poll recvmsg
从错误队列读取始终是一个非阻塞操作。poll 调用用于阻塞，直到有错误挂起。它会在其输出标志中设置 POLLERR。该标志不必设置events 字段中。错误是无条件发出的
```

	pfd.fd = fd;
	pfd.events = 0;
	if (poll(&pfd, 1, -1) != 1 || pfd.revents & POLLERR == 0)
		error(1, errno, "poll");

	ret = recvmsg(fd, &msg, MSG_ERRQUEUE);
	if (ret == -1)
		error(1, errno, "recvmsg");

	read_notification(msg);

```
该示例仅用于演示目的。在实践中，不等待通知，而是每隔几次 send 调用就非阻塞地读取，效率更高
通知可以相对socket 上的其他操作乱序处理。一个错误排队的 socket 通常会阻塞其他操作，直到错误被读取。然而，Zerocopy 通知的错误码为零，因此不会阻send recv 调用

#### 通知批处

可以使用 recvmmsg 调用一次读取多个未完成的包。这通常不是必需的。在内核返回的每条消息中，不是单个值，而是一个范围。当错误队列上有一个通知正在等待接收时，它会合并连续的通知
当一个新的通知即将排队时，它检查新值是否扩展了队列尾部通知的范围。如果是，它会丢弃新的通知包，而是增加未完成通知的范围上限值
对于TCP 这样按序确认数据的协议，每个通知都可以被压缩进前一个通知，从而在任何时刻最多只有一个通知未完成
有序投递是常见情况，但不保证。在重传socket 拆除时，通知可能乱序到达

#### 通知解析


以下片段演示了如何解析控制消息：即前面片段中read_notification() 调用。通知以标准错误格sock_extended_err 编码
控制数据中的 level type 字段是特定于协议族的，IP_RECVERR IPV6_RECVERR（对TCP UDP socket）。对VSOCK socket，cmsg_level 将是 SOL_VSOCK，cmsg_type 将是 VSOCK_RECVERR
错误来源是新的类SO_EE_ORIGIN_ZEROCOPY。ee_errno 为零，如前所述，以避免阻socket 上的读和写系统调用
32 位通知范围编码[ee_info, ee_data]。该范围是包含的（inclusive）。结构体中的其他字段必须被视为未定义，ee_code 除外，如下所述
```

	struct sock_extended_err *serr;
	struct cmsghdr *cm;

	cm = CMSG_FIRSTHDR(msg);
	if (cm->cmsg_level != SOL_IP &&
	    cm->cmsg_type != IP_RECVERR)
		error(1, 0, "cmsg");

	serr = (void *) CMSG_DATA(cm);
	if (serr->ee_errno != 0 ||
	    serr->ee_origin != SO_EE_ORIGIN_ZEROCOPY)
		error(1, 0, "serr");

	printf("completed: %u..%u\n", serr->ee_info, serr->ee_data);


```
#### 延迟拷贝


传入 MSG_ZEROCOPY 标志是对内核应用避免拷贝的提示，并且是一项内核将排队完成通知的约定。它并不保证拷贝会被省略
避免拷贝并非总是可行。不支持分散/聚集（scatter-gather）I/O 的设备无法发送由内核生成的协议头加上 zerocopy 用户数据组成的包。一个包可能需要在协议栈深处转换为数据的私有拷贝，例如为了计算校验和
在所有这些情况下，内核在释放其对共享页的持有时返回一个完成通知。该通知可能在（被拷贝的）数据完全传输之前到达。因此，zerocopy 完成通知不是传输完成通知
延迟拷贝可能比在系统调用中立即拷贝更昂贵，如果数据在缓存中不再温热（warm）。进程还会毫无收益地承担通知处理成本。出于这个原因，内核通过在返回时ee_code 字段设置标志 SO_EE_CODE_ZEROCOPY_COPIED 来发出数据是否被拷贝完成的信号。进程可以使用此信号，在后续对同一 socket 的请求上停止传入 MSG_ZEROCOPY 标志

## 实现


### 回环（Loopback

对于 TCP UDP发送给本地 socket 的数据如果接收进程不读取socket，可能会被无限期排队。不可接受的未绑定通知延迟。出于这个原因，所有用 MSG_ZEROCOPY 生成并回环到本地 socket 的包都会产生延迟拷贝。这包括回环到包 socket（例tcpdump）和 tun 设备
对于 VSOCK发送给本地 socket 的数据路径与非本socket 相同

## 测试


更真实的示例代码可以在内核源码中tools/testing/selftests/net/msg_zerocopy.c 找到
请注意回环限制。该测试可以在一对主机之间运行。但如果在一对本地进程之间运行，例如当通过 msg_zerocopy.sh 在一对跨命名空间veth 之间运行时，测试不会显示任何改善。为了测试，可以通过skb_orphan_frags_rx 设为skb_orphan_frags 相同来临时放宽回环限制
对于 VSOCK 类型socket，示例可以在 tools/testing/vsock/vsock_test_zerocopy.c 找到