
## 内核连接多路复用器（Kernel Connection Multiplexor

内核连接多路复用器（KCM，Kernel Connection Multiplexor）是一种在 TCP 之上为通用
应用协议提供基于消息的接口的机制。借助 KCM，应用程序可以通过数据报（datagram套接字在 TCP 上高效地发送和接收应用协议消息
```
    +------------+   +------------+   +------------+   +------------+
    | KCM socket |   | KCM socket |   | KCM socket |   | KCM socket |
    +------------+   +------------+   +------------+   +------------+
	|                 |               |                |
	+-----------+     |               |     +----------+
		    |     |               |     |
		+----------------------------------+
		|           Multiplexor            |
		+----------------------------------+
		    |   |           |           |  |
	+---------+   |           |           |  ------------+
	|             |           |           |              |
    +----------+  +----------+  +----------+  +----------+ +----------+
    |  Psock   |  |  Psock   |  |  Psock   |  |  Psock   | |  Psock   |
    +----------+  +----------+  +----------+  +----------+ +----------+
	|              |           |            |             |
    +----------+  +----------+  +----------+  +----------+ +----------+
    | TCP sock |  | TCP sock |  | TCP sock |  | TCP sock | | TCP sock |
    +----------+  +----------+  +----------+  +----------+ +----------+
```

## KCM 濂楁帴瀛。

KCM 套接字提供了到多路复用器的用户接口。绑定到同一个多路复用器的所KCM 套接被认为具有等价的功能，并且不同套接字上的 I/O 操作可以并行执行，无需用户空间线程
之间进行同步
## 多路复用器（Multiplexor

多路复用器提供消息的导向（steering）。在发送路径上，写KCM 套接字上的消息会一个合适的 TCP 套接字上以原子方式发送。类似地，在接收路径上，消息在每TCP 套接（Psock）上被构造出来，完整的消息被导向到一KCM 套接字
## TCP 套接字与 Psock


TCP 套接字可以被绑定到一KCM 多路复用器。为每个被绑定的 TCP 套接字分配一Psock
结构体，该结构体保存用于在接收时构造消息的状态，以及其他针对 KCM 的连接特定信息
## 连接模式的语

每个多路复用器都假定所有附着TCP 连接都指向同一个目的地，并且可以在发送时使用
不同连接进行负载均衡。普通的 send recv 调用（包sendmmsg recvmmsg）可用于
KCM 套接字发送和接收消息
## 濂楁帴瀛楃被鍨。

KCM 支持 SOCK_DGRAM SOCK_SEQPACKET 套接字类型
### 消息定界（Message delineation

消息通过 TCP 流以某种应用协议消息格式发送，这种格式通常包含一个对消息进行分帧
（framing）的头部。接收到的消息的长度可以从应用协议头部推断出来（通常只是一简单的长度字段）
必须TCP 流进行解析以确定消息边界。这里使用伯克利包过滤器（BPF，Berkeley Packet
Filter）。当把一TCP 套接字附着到多路复用器时，必须指定一BPF 程序。该程序开始接收新消息时被调用，并得到一个包含到目前为止已接收字节的 skbuff。它解析消息
头部并返回消息的长度。有了这一信息，KCM 就会构造出指定长度的消息，并将其投递到一
KCM 套接字
### TCP 套接字管

当一TCP 套接字被附着KCM 多路复用器时，数据就绪（POLLIN）和写空间可用（POLLOUT事件由多路复用器处理。如果一TCP 套接字发生了状态变化（断开连接）或其他错误，会TCP 套接字上投递一个错误，从而引POLLERR 事件，KCM 停止使用该套接字。当应用
程序收到TCP 套接字的错误通知时，它应当将该套接字KCM 上解除附着，然后处理错状况（典型的响应是关闭该套接字，并在必要时创建一个新的连接）
KCM 将最大接收消息大小限制为被附着 TCP 套接字接收缓冲区的大小（套接字缓冲区大小可以
通过 SO_RCVBUF 设置）。如BPF 程序报告的新消息长度大于此限制，则会在该 TCP 套接上投递相应的错误（EMSGSIZE）。BPF 程序也可以强制实施一个最大消息大小，并在超出时报错
可以为接收套接字上的消息组装设置超时。超时取值取自被附着 TCP 套接字的接收超时（这
通过 SO_RCVTIMEO 设置）。如果定时器在组装完成之前到期，则会在该套接字上投递一个错（ETIMEDOUT）
## 用户接口


### 创建多路复用

```
  socket(AF_KCM, type, protocol)
```
- type SOCK_DGRAM SOCK_SEQPACKET 之一
- protocol 鏄?KCMPROTO_CONNECTED

### 鍏嬮殕 KCM 濂楁帴瀛。

在如上所述使socket 调用创建了第一KCM 套接字之后，可以通过克隆的方式为多路复用创建额外的套接字
```
  /* From linux/kcm.h */
  struct kcm_clone {
	int fd;
  };

  struct kcm_clone info;

  memset(&info, 0, sizeof(info));

  err = ioctl(kcmfd, SIOCKCMCLONE, &info);

  if (!err)
    newkcmfd = info.fd;
```

### 闄勭潃浼犺緭濂楁帴瀛。

把传输套接字附着到多路复用器是通过调用以下方式执行```
  /* From linux/kcm.h */
  struct kcm_attach {
	int fd;
	int bpf_fd;
  };

  struct kcm_attach info;

  memset(&info, 0, sizeof(info));

  info.fd = tcpfd;
  info.bpf_fd = bpf_prog_fd;

  ioctl(kcmfd, SIOCKCMATTACH, &info);
```
kcm_attach 结构体包含：

  - fd：被附着TCP 套接字的文件描述  - bpf_prog_fd：已下载的已编译 BPF 程序的文件描述符

### 解除附着传输套接

把一个传输套接字从多路复用器上解除附着很简单。一```
  /* From linux/kcm.h */
  struct kcm_unattach {
	int fd;
  };

  struct kcm_unattach info;

  memset(&info, 0, sizeof(info));

  info.fd = cfd;

  ioctl(fd, SIOCKCMUNATTACH, &info);
```

### 禁用 KCM 套接字上的接

一setsockopt 用于禁用或启KCM 套接字上的接收。当接收被禁用时，该套接字接缓冲区中的任何待处理消息都会被移动到其他套接字。如果一个应用线程知道它将对某个
请求做大量工作、从而无法为新的消息提供服务，那么这个特性很有用
```
  int val = 1;

  setsockopt(kcmfd, SOL_KCM, KCM_RECV_DISABLE, &val, sizeof(val))
```

### 用于消息定界BPF 程序


BPF 程序可以使用 BPF LLVM 后端编译。例如，
```
  #include "bpf.h" /* for __sk_buff */
  #include "bpf_helpers.h" /* for load_word intrinsic */

  SEC("socket_kcm")
  int bpf_prog1(struct __sk_buff *skb)
  {
       return load_word(skb, 0) + 4;
  }

  char _license[] SEC("license") = "GPL";
```

## 在应用程序中使用


KCM 加速了应用层协议。具体而言，它允许应用程序使用基于消息的接口来发送和接收消息内核提供了必要的保证，确保消息以原子方式发送和接收。这减轻了应用程序在把基于消息的
协议映射TCP 流方面的很大负担。KCM 还使应用层消息在内核中成为用于导向和调度工作单元，这反过来允许在多线程应用程序中采用更简单的网络模型
### 配置


Nx1 配置中，KCM 在逻辑上为同一TCP 连接提供多个套接字句柄。这允许在该 TCP 套接I/O 操作之间实现并行（例如数据的 copyin copyout 是并行的）。在应用程序中，可以
为每个处理线程打开一KCM 套接字，并将其插epoll（类似于使用 SO_REUSEPORT 来允同一端口上的多个监听套接字）
MxN 配置中，会向同一目的地建立多个连接。这些连接用于简单的负载均衡
### 消息批处理（Message batching

KCM 的主要目的在KCM 套接字之间、进而在典型用例中线程之间的负载均衡。完美的负载
均衡，即把每个接收到的消息导向不同的 KCM 套接字，或把每个发送的消息导向不同TCP
套接字，可能会对性能产生负面影响，因为这不允许建立亲和性（affinity）。基于消息组批次的均衡可能对性能有益
在发送时，应用程序有三种方式KCM 套接字上批处理（流水线化）消息
  1) 在单sendmmsg 中发送多个消息  2) 用每sendmsg 调用发送一组消息，其中除最后一个以外的所有消息在 sendmsg 调用     flags 中都带有 MSG_BATCH  3) 创建一个由多个消息组成的“超级消息”，并用单个 sendmsg 发送
在接收时，KCM 模块会尝试在每个 TCP 就绪回调期间，把在同一 KCM 套接字上接收到的消息
排入队列。目KCM 套接字在每个 KCM 套接字的接收就绪回调处发生变化。应用程序不需配置这一点
### 错误处理


应用程序应当包含一个线程来监控TCP 连接上引发的错误。通常，这通过将每个附着KCM
多路复用器的 TCP 套接字放epoll 集合中监POLLERR 事件来完成。如果一个被附着TCP
套接字上发生错误，KCM 会在该套接字上设置一EPIPE，从而唤醒应用程序线程。当应用程序
看到该错误（可能仅仅是一次断开）时，它应当将该套接字从 KCM 上解除附着，然后关闭它这里假定，一旦在 TCP 套接字上投递了错误，数据流就不可恢复了（即错误可能发生在接消息的中途）
### TCP 连接监控


KCM 中，没有办法把一个消息关联到用于发送或接收该消息的 TCP 套接字（只有一个被附着
TCP 套接字的情况除外）。不过，应用程序确实保留了对该套接字的打开文件描述符，因此
它能够从该套接字获取可以用于检测问题的统计信息（例如该套接字上的高重传率）