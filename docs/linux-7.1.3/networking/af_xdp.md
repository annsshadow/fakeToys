
## AF_XDP


## 概述


AF_XDP 是一个为高性能数据包处理而优化的地址族。

本文档假定读者已经熟悉 BPF 和 XDP。如果不熟悉，Cilium 项目在
http://cilium.readthedocs.io/en/latest/bpf/ 提供了一份优秀的参考指南。

通过 XDP 程序中的 XDP_REDIRECT 动作，程序可以使用
bpf_redirect_map() 函数将入口帧（ingress frame）重定向到其他启用了 XDP 的
netdev，。AF_XDP 套接字使得 XDP 程序能够将帧重定向到用户空间应用程序中的一块内存缓冲区。

一个 AF_XDP 套接字（XSK）使用普通的 socket() 系统调用创建。每个 XSK 关联两个
环（ring）：RX 环和 TX 环。套接字可以在 RX 环上接收数据包，也可以在 TX 环上发送
数据包。这两个环分别通过 setsockopt XDP_RX_RING 和 XDP_TX_RING 注册并设定大小。
每个套接字必须至少拥有其中一个环。RX 或 TX 描述符环指向称为 UMEM 的内存区域中的
一个数据缓冲区。RX 和 TX 可以共享同一个 UMEM，从而数据包无需在 RX 和 TX 之间复制。
此外，如果某个数据包因为可能需要重传而要保留一段时间，指向该数据包的描述符可以
被改为指向另一个数据包，并立即被重用。这也避免了数据复制。

UMEM 由若干大小相等的块（chunk）组成。某个环中的描述符通过引用其 addr 来引用一个
帧（frame）。addr 只是整个 UMEM 区域内的一个偏移量。用户空间使用它认为最合适的方式
（malloc、mmap、大页等）为这个 UMEM 分配内存。然后这块内存区域通过新的 setsockopt
XDP_UMEM_REG 注册到内核。UMEM 还有两个环：FILL 环和 COMPLETION 环。FILL 环由
应用程序使用，向下传递 addr 供内核填充 RX 数据包数据。每个数据包被接收后，对这些帧的
引用就会出现在 RX 环中。另一方面，COMPLETION 环包含内核已经完整发送、现在可以被
用户空间再次用于 TX 或 RX 的帧 addr。因此，出现在 COMPLETION 环中的帧 addr 是之前
使用 TX 环发送的那些 addr。总之，RX 和 FILL 环用于 RX 路径，而 TX 和 COMPLETION
环用于 TX 路径。

套接字最终通过 bind() 调用绑定到某个设备以及该设备上的一个特定队列 id，而且只有在
bind 完成之后，流量才会开始流动。

如果需要，UMEM 可以在进程之间共享。如果某个进程想这样做，它只需跳过 UMEM 及其对应
两个环的注册，在 bind 调用中设置 XDP_SHARED_UMEM 标志，并提交它想要共享 UMEM 的
那个进程的 XSK 以及它自己新创建的 XSK 套接字。然后新进程会在它自己的 RX 环中收到指向
这个共享 UMEM 的帧 addr 引用。请注意，由于环结构出于性能原因是单消费者 / 单生产者
（single-consumer / single-producer）的，新进程必须创建它自己的带有 RX 和 TX 环的
套接字，因为它无法与该进程共享这部分。这也是为什么每个 UMEM 只有一组 FILL 和
COMPLETION 环的原因。处理 UMEM 是单个进程的责任。

那么数据包是如何从 XDP 程序分发到各个 XSK 的呢？有一个称为 XSKMAP（完整名称
BPF_MAP_TYPE_XSKMAP）的 BPF map。用户空间应用程序可以在这个 map 中任意位置放置一个
XSK。然后 XDP 程序可以将数据包重定向到该 map 中的特定索引，此时 XDP 会校验该 map 中的
XSK 确实绑定到了那个设备和环编号。如果没有，数据包会被丢弃。如果该索引处 map 为空，
数据包也会被丢弃。这也意味着，当前必须加载一个 XDP 程序（并且 XSKMAP 中至少有一个
XSK）才能通过 XSK 将任何流量送到用户空间。

AF_XDP 可以在两种不同的模式下运行：XDP_SKB 和 XDP_DRV。如果驱动不支持 XDP，或者在
加载 XDP 程序时显式选择了 XDP_SKB，则采用 XDP_SKB 模式，该模式使用 SKB 配合通用的
XDP 支持，并将数据复制到用户空间。这是一种适用于任何网络设备的回退模式。另一方面，
如果驱动支持 XDP，AF_XDP 代码将使用它来提供更好的性能，但数据仍然会被复制到用户空间。

## 概念


要使用 AF_XDP 套接字，需要建立若干相关联的对象。这些对象及其选项将在以下各节中说明。

关于 AF_XDP 工作原理的概览，你也可以参阅 2018 年 Linux Plumbers 会议上关于该主题的
文章：http://vger.kernel.org/lpc_net2018_talks/lpc18_paper_af_xdp_perf-v2.pdf。请勿
参考 2017 年关于“AF_PACKET v4”（AF_XDP 的首次尝试）的文章。自那以后几乎所有内容都
改变了。Jonathan Corbet 也在 LWN 上写了一篇优秀的文章“Accelerating networking with
AF_XDP”。可在 https://lwn.net/Articles/750845/ 找到。

### UMEM


UMEM 是一段虚拟连续内存区域，被划分为大小相等的帧（frame）。一个 UMEM 关联到一个
netdev 以及该 netdev 的一个特定队列 id。它是通过使用 XDP_UMEM_REG setsockopt 系统
调用来创建和配置（块大小、headroom、起始地址和大小）的。UMEM 通过 bind() 系统调用
绑定到 netdev 和队列 id。

一个 AF_XDP 套接字链接到单个 UMEM，但一个 UMEM 可以有多个 AF_XDP 套接字。要共享通过
某个套接字 A 创建的 UMEM，下一个套接字 B 可以通过在 struct sockaddr_xdp 成员
sxdp_flags 中设置 XDP_SHARED_UMEM 标志，并将 A 的文件描述符传给 struct sockaddr_xdp
成员 sxdp_shared_umem_fd 来实现。

UMEM 有两个单生产者 / 单消费者（single-producer / single-consumer）环，用于在内核和
用户空间应用程序之间转移 UMEM 帧的所有权。

### 环


共有四种不同的环：FILL、COMPLETION、RX 和 TX。所有环都是单生产者 / 单消费者的，因此
当多个进程 / 线程读写它们时，用户空间应用程序需要显式的同步。

UMEM 使用两个环：FILL 和 COMPLETION。与 UMEM 关联的每个套接字必须拥有 RX 队列、
TX 队列或两者皆有。例如，假设有一个包含四个套接字（都进行 TX 和 RX）的配置。那么将会
有一个 FILL 环、一个 COMPLETION 环、四个 TX 环和四个 RX 环。

这些环是基于 head（生产者）/ tail（消费者）的环。生产者在 struct xdp_ring 的
producer 成员所指向的索引处写入数据环，并递增生产者索引。消费者在 struct xdp_ring 的
consumer 成员所指向的索引处读取数据环，并递增消费者索引。

这些环通过 _RING setsockopt 系统调用进行配置和创建，并使用适当的 mmap() 偏移量映射到
用户空间（XDP_PGOFF_RX_RING、XDP_PGOFF_TX_RING、XDP_UMEM_PGOFF_FILL_RING 和
XDP_UMEM_PGOFF_COMPLETION_RING）。

环的大小必须是 2 的幂。

#### UMEM FILL 环


FILL 环用于将 UMEM 帧的所有权从用户空间转移到内核空间。UMEM addr 在该环中传递。例如，
如果 UMEM 是 64k，每个块是 4k，那么 UMEM 有 16 个块，可以传递 0 到 64k 之间的 addr。

传递给内核的帧用于入口路径（RX 环）。

用户应用程序向这个环生产（produce）UMEM addr。请注意，如果以对齐块模式运行应用程序，
内核会屏蔽传入的 addr。例如，对于 2k 的块大小，addr 的 log2(2048) 个最低有效位（LSB）
会被屏蔽，这意味着 2048、2050 和 3000 都指向同一个块。如果用户应用程序以非对齐块模式
运行，则传入的 addr 会保持原样。


#### UMEM COMPLETION 环


COMPLETION 环用于把 UMEM 帧的所有权从内核空间转移到用户空间。与 FILL 环一样，使用的是
UMEM 索引。

从内核传递给用户空间的帧是已经被发送（TX 环）且可以被用户空间再次使用的帧。

用户应用程序从这个环消费（consume）UMEM addr。


#### RX 环


RX 环是套接字的接收端。环中的每一项都是一个 struct xdp_desc 描述符。该描述符包含
UMEM 偏移（addr）以及数据长度（len）。

如果没有通过 FILL 环向内核传递任何帧，RX 环上就不会（也无法）出现任何描述符。

用户应用程序从这个环消费 struct xdp_desc 描述符。

#### TX 环


TX 环用于发送帧。struct xdp_desc 描述符被填充（索引、长度和偏移）后传入该环。

要启动传输，需要一次 sendmsg() 系统调用。这一点未来可能会放宽。

用户应用程序向这个环生产 struct xdp_desc 描述符。

## Libbpf


Libbpf 是一个用于 eBPF 和 XDP 的辅助库，使这些技术的使用简单很多。它还包含
tools/testing/selftests/bpf/xsk.h 中的特定辅助函数，便于使用 AF_XDP。它包含两种
函数：一种可用于简化 AF_XDP 套接字的建立，另一种可用于数据面（data plane）以安全快速
地访问这些环。

我们建议你使用这个库，除非你已经成为高级用户。它会使你的程序简单很多。

## XSKMAP / BPF_MAP_TYPE_XSKMAP


在 XDP 侧有一个 BPF map 类型 BPF_MAP_TYPE_XSKMAP（XSKMAP），它与 bpf_redirect_map()
配合使用，将入口帧传递给一个套接字。

用户应用程序通过 bpf() 系统调用将套接字插入该 map。

请注意，如果 XDP 程序试图重定向到一个与队列配置和 netdev 不匹配的套接字，该帧会被
丢弃。例如，一个 AF_XDP 套接字绑定到 netdev eth0 和队列 17。只有为 eth0 和队列 17
执行的 XDP 程序才能成功将数据传给该套接字。请参考示例应用程序（samples/bpf/）中的
例子。

## 配置标志与套接字选项


以下是可用于控制和监视 AF_XDP 套接字行为的各种配置标志。

### XDP_COPY 和 XDP_ZEROCOPY 绑定标志


当你绑定到一个套接字时，内核会首先尝试使用零拷贝（zero-copy）。如果不支持零拷贝，它会
回退到使用拷贝（copy）模式，即把所有数据包拷贝到用户空间。但如果你想强制使用某种模式，
可以使用以下标志。如果你在 bind 调用中传入 XDP_COPY 标志，内核会强制该套接字进入拷贝
模式。如果它无法使用拷贝模式，bind 调用将以错误失败。相反，XDP_ZEROCOPY 标志会强制
套接字进入零拷贝模式，否则失败。

### XDP_SHARED_UMEM 绑定标志


该标志使你能够将多个套接字绑定到同一个 UMEM。它适用于相同的队列 id 之间、不同队列 id
之间以及不同 netdev / 设备之间。在此模式下，每个套接字照常拥有自己的 RX 和 TX 环，但
你会拥有一组或多组 FILL 和 COMPLETION 环对。你必须为你绑定到的每个唯一的 netdev 和
队列 id 元组创建这样一组。

先从我们希望在绑定到相同 netdev 和队列 id 的套接字之间共享 UMEM 的情况说起。UMEM（绑定
到第一个创建的套接字）将只有一个 FILL 环和一个 COMPLETION 环，因为我们已经绑定的唯一
netdev、queue_id 元组只有一个。要使用此模式，创建第一个套接字并以常规方式绑定它。
创建第二个套接字并创建 RX 和 TX 环（或至少其中之一），但不要创建 FILL 或 COMPLETION
环，因为将使用第一个套接字的那些。在 bind 调用中，设置 XDP_SHARED_UMEM 选项，并在
sxdp_shared_umem_fd 字段中提供初始套接字的 fd。你可以以这种方式附加任意数量的额外
套接字。

那么数据包会到达哪个套接字呢？这由 XDP 程序决定。把所有套接字放入 XSK_MAP，并指明你想
把每个数据包发送到数组中的哪个索引。下面展示了一个简单的轮询（round-robin）分发数据包
示例：


   #include <linux/bpf.h>
   #include "bpf_helpers.h"

   #define MAX_SOCKS 16

   struct {
       __uint(type, BPF_MAP_TYPE_XSKMAP);
       __uint(max_entries, MAX_SOCKS);
       __uint(key_size, sizeof(int));
       __uint(value_size, sizeof(int));
   } xsks_map SEC(".maps");

   static unsigned int rr;

   SEC("xdp_sock") int xdp_sock_prog(struct xdp_md *ctx)
   {
       rr = (rr + 1) & (MAX_SOCKS - 1);

       return bpf_redirect_map(&xsks_map, rr, XDP_DROP);
   }

请注意，由于只有一组 FILL 和 COMPLETION 环，而且它们是单生产者、单消费者环，你需要
确保多个进程或线程不会并发使用这些环。libbpf 代码目前没有任何同步原语来保护多个用户。

如果你创建多个绑定到同一个 UMEM 的套接字，libbpf 会使用此模式。但请注意，你需要在
xsk_socket__create 调用中提供 XSK_LIBBPF_FLAGS__INHIBIT_PROG_LOAD libbpf_flag，并加载
你自己的 XDP 程序，因为 libbpf 中没有内置的、可为你路由流量的程序。

第二种情况是你在绑定到不同队列 id 和 / 或不同 netdev 的套接字之间共享 UMEM。在这种
情况下，你必须为每个唯一的 netdev、queue_id 对创建一个 FILL 环和一个 COMPLETION 环。
假设你想创建两个绑定到同一 netdev 上不同队列 id 的套接字。创建第一个套接字并以常规方式
绑定它。创建第二个套接字并创建 RX 和 TX 环（或至少其中之一），然后为这个套接字创建一个
FILL 和 COMPLETION 环。然后在 bind 调用中，设置 XDP_SHARED_UMEM 选项，并在
sxdp_shared_umem_fd 字段中提供初始套接字的 fd（因为你在该套接字上注册了 UMEM）。这两个
套接字现在将共享同一个 UMEM。

不需要像前面套接字绑定到相同队列 id 和设备的情形那样提供 XDP 程序。相反，使用 NIC 的
数据包导向（packet steering）能力将数据包导向正确的队列。在前面的例子中，套接字之间只
共享一个队列，所以 NIC 无法进行这种导向。它只能在队列之间做导向。

在 libbpf 中，你需要使用 xsk_socket__create_shared() API，因为它接受一个 FILL 环和
一个 COMPLETION 环的引用，这两个环会为你创建并绑定到共享 UMEM。你可以对创建的所有套接字
都使用这个函数，也可以只对第二个及之后的套接字使用它，而对第一个套接字使用
xsk_socket__create()。两种方法得到相同的结果。

请注意，UMEM 可以在相同队列 id 和设备的套接字之间共享，也可以同时在相同设备的不同队列
之间以及不同设备之间共享。

### XDP_USE_NEED_WAKEUP 绑定标志


该选项新增了对一个名为 need_wakeup 的新标志的支持，它存在于 FILL 环和 TX 环中（用户空间
作为生产者的那些环）。当在 bind 调用中设置此选项时，如果内核需要被系统调用显式唤醒才能
继续处理数据包，need_wakeup 标志会被置位。如果该标志为零，则不需要系统调用。

如果 FILL 环上设置了该标志，应用程序需要调用 poll() 才能继续在 RX 环上接收数据包。例如，
当内核检测到 FILL 环上已没有缓冲区、NIC 的 RX HW 环上也没有剩余缓冲区时，就会发生这种
情况。此时中断被关闭，因为 NIC 无法接收任何数据包（因为没有缓冲区可放入），于是设置
need_wakeup 标志，以便用户空间可以把缓冲区放到 FILL 环上，然后调用 poll()，让内核驱动把
这些缓冲区放到 HW 环上并开始接收数据包。

如果 TX 环上设置了该标志，意味着应用程序需要显式通知内核发送放到 TX 环上的任何数据包。
这可以通过 poll() 调用（如同 RX 路径那样）或调用 sendto() 来完成。

TX 路径中使用 libbpf 辅助函数的一个示例如下：


   if (xsk_ring_prod__needs_wakeup(&my_tx_ring))
       sendto(xsk_socket__fd(xsk_handle), NULL, 0, MSG_DONTWAIT, NULL, 0);

也就是说，仅当该标志被置位时才使用系统调用。

我们建议你始终启用此模式，因为它通常带来更好的性能，特别是在应用程序和驱动运行在同一个
核上时；即便应用程序和内核驱动使用不同的核，它也会减少 TX 路径所需的系统调用数量。

### XDP_{RX|TX|UMEM_FILL|UMEM_COMPLETION}_RING setsockopts


这些 setsockopt 分别设置 RX、TX、FILL 和 COMPLETION 环应当拥有的描述符数量。必须设置
RX 和 TX 环中至少一个的大小。如果两者都设置，你的应用程序将既能接收也能发送流量；但如果你
只想做其中之一，可以只建立其中一个来节省资源。FILL 环和 COMPLETION 环都是必需的，因为你
需要一个绑定到套接字的 UMEM。但如果使用了 XDP_SHARED_UMEM 标志，第一个之后的任何套接字
都没有 UMEM，这种情况下就不应该创建任何 FILL 或 COMPLETION 环，因为将使用来自共享 UMEM
的那些。请注意，这些环是单生产者单消费者的，所以不要尝试同时从多个进程访问它们。参见
XDP_SHARED_UMEM 一节。

在 libbpf 中，你可以分别将 NULL 传给 xsk_socket__create 函数的 rx 和 tx 参数，来创建
只接收（Rx-only）和只发送（Tx-only）的套接字。

如果你创建了只发送的套接字，我们建议你不要在 fill 环上放任何数据包。如果这样做，驱动可能
会认为你将接收某些东西（而实际上你不会），这会对性能产生负面影响。

### XDP_UMEM_REG setsockopt


该 setsockopt 将一个 UMEM 注册到套接字。这是包含所有可容纳数据包的缓冲区的区域。该调用
接受一个指向该区域起始位置的指针以及它的大小。此外，它还有一个名为 chunk_size 的参数，
表示 UMEM 被划分成的块大小。目前它只能是 2K 或 4K。如果你有一个 128K 的 UMEM 区域和 2K
的块大小，这意味着你的 UMEM 区域最多可容纳 128K / 2K = 64 个数据包，且你的最大数据包
大小可以是 2K。

还有一个选项可以设置 UMEM 中每个缓冲区的 headroom。如果你将其设为 N 字节，意味着数据包将
从缓冲区第 N 字节处开始，留下前 N 字节供应用程序使用。最后一个选项是 flags 字段，但将
针对每个 UMEM 标志在单独的章节中说明。

### SO_BINDTODEVICE setsockopt


这是一个通用的 SOL_SOCKET 选项，可用于将 AF_XDP 套接字绑定到特定的网络接口。当套接字由
特权进程创建并传递给非特权进程时，它很有用。一旦设置了该选项，内核将拒绝将该套接字绑定到
不同接口的尝试。更新该值需要 CAP_NET_RAW。

### XDP_MAX_TX_SKB_BUDGET setsockopt


该 setsockopt 设置在一次 send 系统调用中可以处理并传给驱动的描述符最大数量。它应用于
拷贝模式，用于让应用程序调优每个套接字的最大迭代次数，以获得更好的吞吐量并降低 send 系统
调用的频率。允许的范围是 [32, xs->tx->nentries]。

### XDP_STATISTICS getsockopt


获取套接字的丢弃统计信息，可用于调试目的。支持的统计信息如下所示：


   struct xdp_statistics {
       __u64 rx_dropped; /** Dropped for reasons other than invalid desc **/
       __u64 rx_invalid_descs; /** Dropped due to invalid descriptor **/
       __u64 tx_invalid_descs; /** Dropped due to invalid descriptor **/
   };

### XDP_OPTIONS getsockopt


从 XDP 套接字获取选项。目前唯一支持的是 XDP_OPTIONS_ZEROCOPY，它告诉你是否开启了零拷贝。

## 多缓冲区支持


借助多缓冲区支持，使用 AF_XDP 套接字的程序可以在拷贝模式和零拷贝模式下接收和发送由多个
缓冲区组成的数据包。例如，一个数据包可以由两个帧 / 缓冲区组成，一个包含头部、另一个包含
数据；或者一个 9K 的以太网巨型帧（jumbo frame）可以通过将三个 4K 帧链接起来构造。

一些定义：

- 一个数据包由一个或多个帧组成

- 某个 AF_XDP 环中的描述符总是引用单个帧。当数据包由单个帧组成时，该描述符引用整个
  数据包。

要为 AF_XDP 套接字启用多缓冲区支持，请使用新的绑定标志 XDP_USE_SG。如果不提供它，所有
多缓冲区数据包都会像以前一样被丢弃。请注意，加载的 XDP 程序也需要处于多缓冲区模式。这
可以通过把 "xdp.frags" 用作所用 XDP 程序的段（section）名来实现。

为了表示一个由多个帧组成的数据包，在 Rx 和 Tx 描述符的 options 字段中引入了一个名为
XDP_PKT_CONTD 的新标志。如果它为真（1），表示数据包延续到下一个描述符；如果为假（0），
表示这是数据包的最后一个描述符。为什么采用许多 NIC 中包结束（eop）标志的反向逻辑？仅仅
是为了与多缓冲区应用程序保持兼容——那些应用程序在 Rx 上把该位设为假，并在 Tx 上把 options
字段设为零，因为其他任何值都会被视为无效描述符。

以下是将由多个帧组成的数据包生产（produce）到 AF_XDP Tx 环时的语义：

- 当发现一个无效描述符时，该数据包的所有其他描述符 / 帧都会被标记为无效且不完成。下一个
  描述符会被当作一个新数据包的开始，即便这并非本意（因为我们无法猜测本意）。和以前一样，
  如果你的程序正在生产无效描述符，说明你有一个必须修复的 bug。

- 零长度描述符被视为无效描述符。

- 对于拷贝模式，一个数据包支持的帧最大数量等于 CONFIG_MAX_SKB_FRAGS + 1。如果超出，
  到目前为止累积的所有描述符都会被丢弃并视为无效。要编写一个可在任何系统上运行、不受该
  配置设置影响的应用程序，请将 frags 数量限制为 18，因为该配置的最小值是 17。

- 对于零拷贝模式，上限取决于 NIC 硬件支持的程度。在我们检查过的 NIC 上通常至少有五个。
  我们刻意选择不为零拷贝模式强制一个固定的上限（例如 CONFIG_MAX_SKB_FRAGS + 1），因为那
  会导致在底层进行拷贝以适应该 NIC 支持的上限。这有违零拷贝模式的目的。如何探测该上限将在
  “探测多缓冲区支持”一节中说明。

在拷贝模式的 Rx 路径上，xsk 核心会在需要时把 XDP 数据复制到多个描述符，并按前述设置
XDP_PKT_CONTD 标志。零拷贝模式工作方式相同，只是数据不被复制。当应用程序拿到一个
XDP_PKT_CONTD 标志设为 1 的描述符时，意味着该数据包由多个缓冲区组成，并延续到下一个
描述符中的下一个缓冲区。当收到一个 XDP_PKT_CONTD == 0 的描述符时，意味着这是该数据包的
最后一个缓冲区。AF_XDP 保证只把完整的数据包（数据包中的所有帧）发送给应用程序。如果
AF_XDP 的 Rx 环中没有足够空间，该数据包的所有帧都将被丢弃。

如果应用程序读取一批描述符（例如使用 libxdp 接口），不能保证这批描述符会以一个完整的数据包
结束。它可能在一个数据包的中间结束，该数据包的其余缓冲区会在下一批的开头到达，因为 libxdp
接口不会读取整个环（除非你有极大的批大小或极小的环大小）。

针对 Rx 和 Tx 多缓冲区支持的示例程序可在本文档后面找到。

### 用法


要使用 AF_XDP 套接字，需要两个部分：用户空间应用程序和 XDP 程序。关于完整的建立和使用
示例，请参考 xdp-project：
https://github.com/xdp-project/bpf-examples/tree/main/AF_XDP-example。

XDP 代码示例代码如下：


   SEC("xdp_sock") int xdp_sock_prog(struct xdp_md *ctx)
   {
       int index = ctx->rx_queue_index;

       // A set entry here means that the corresponding queue_id
       // has an active AF_XDP socket bound to it.
       if (bpf_map_lookup_elem(&xsks_map, &index))
           return bpf_redirect_map(&xsks_map, index, 0);

       return XDP_PASS;
   }

一个简单但性能并不高的环出队（dequeue）和入队（enqueue）可能如下所示：


    // struct xdp_rxtx_ring {
    //     __u32 *producer;
    //     __u32 *consumer;
    //     struct xdp_desc *desc;
    // };

    // struct xdp_umem_ring {
    //     __u32 *producer;
    //     __u32 *consumer;
    //     __u64 *desc;
    // };

    // typedef struct xdp_rxtx_ring RING;
    // typedef struct xdp_umem_ring RING;

    // typedef struct xdp_desc RING_TYPE;
    // typedef __u64 RING_TYPE;

    int dequeue_one(RING **ring, RING_TYPE **item)
    {
        __u32 entries = **ring->producer - **ring->consumer;

        if (entries == 0)
            return -1;

        // read-barrier!

        **item = ring->desc[**ring->consumer & (RING_SIZE - 1)];
        (*ring->consumer)++;
        return 0;
    }

    int enqueue_one(RING **ring, const RING_TYPE **item)
    {
        u32 free_entries = RING_SIZE - (**ring->producer - **ring->consumer);

        if (free_entries == 0)
            return -1;

        ring->desc[**ring->producer & (RING_SIZE - 1)] = **item;

        // write-barrier!

        (*ring->producer)++;
        return 0;
    }

但请使用 libbpf 的函数，因为它们经过优化且开箱即用。那会让你的生活更轻松。

### 多缓冲区 Rx 用法


下面是一个简单的 Rx 路径伪代码示例（为简洁起见使用了 libxdp 接口）。为保持简短，省略了
错误路径：


    void rx_packets(struct xsk_socket_info *xsk)
    {
        static bool new_packet = true;
        u32 idx_rx = 0, idx_fq = 0;
        static char *pkt;

        int rcvd = xsk_ring_cons__peek(&xsk->rx, opt_batch_size, &idx_rx);

        xsk_ring_prod__reserve(&xsk->umem->fq, rcvd, &idx_fq);

        for (int i = 0; i < rcvd; i++) {
            struct xdp_desc *desc = xsk_ring_cons__rx_desc(&xsk->rx, idx_rx++);
            char *frag = xsk_umem__get_data(xsk->umem->buffer, desc->addr);
            bool eop = !(desc->options & XDP_PKT_CONTD);

            if (new_packet)
                pkt = frag;
            else
                add_frag_to_pkt(pkt, frag);

            if (eop)
                process_pkt(pkt);

            new_packet = eop;

            *xsk_ring_prod__fill_addr(&xsk->umem->fq, idx_fq++) = desc->addr;
        }

        xsk_ring_prod__submit(&xsk->umem->fq, rcvd);
        xsk_ring_cons__release(&xsk->rx, rcvd);
    }

### 多缓冲区 Tx 用法


下面是一个 Tx 路径伪代码示例（为简洁起见使用了 libxdp 接口），忽略 umem 大小有限这一点，
以及我们最终会耗尽待发送数据包这一点。同时假设 pkts.addr 指向 umem 中的一个有效位置：


    void tx_packets(struct xsk_socket_info **xsk, struct pkt **pkts,
                    int batch_size)
    {
        u32 idx, i, pkt_nb = 0;

        xsk_ring_prod__reserve(&xsk->tx, batch_size, &idx);

        for (i = 0; i < batch_size;) {
            u64 addr = pkts[pkt_nb].addr;
            u32 len = pkts[pkt_nb].size;

            do {
                struct xdp_desc *tx_desc;

                tx_desc = xsk_ring_prod__tx_desc(&xsk->tx, idx + i++);
                tx_desc->addr = addr;

                if (len > xsk_frame_size) {
                    tx_desc->len = xsk_frame_size;
                    tx_desc->options = XDP_PKT_CONTD;
                } else {
                    tx_desc->len = len;
                    tx_desc->options = 0;
                    pkt_nb++;
                }
                len -= tx_desc->len;
                addr += xsk_frame_size;

                if (i == batch_size) {
                /* Remember len, addr, pkt_nb for next iteration.
                 - Skipped for simplicity.
                 */
                    break;
                }
            } while (len);
        }

        xsk_ring_prod__submit(&xsk->tx, i);
    }

### 探测多缓冲区支持


要发现某个驱动是否在 SKB 或 DRV 模式下支持多缓冲区 AF_XDP，可使用 linux/netdev.h 中
netlink 的 XDP_FEATURES 特性查询 NETDEV_XDP_ACT_RX_SG 支持。这与查询 XDP 多缓冲区支持
使用的是同一个标志。如果某个驱动中的 XDP 支持多缓冲区，那么 AF_XDP 在 SKB 和 DRV 模式下
也将支持它。

要发现某个驱动是否在零拷贝模式下支持多缓冲区 AF_XDP，可使用 XDP_FEATURES 并先检查
NETDEV_XDP_ACT_XSK_ZEROCOPY 标志。如果它被置位，意味着至少支持零拷贝，你应该去检查
linux/netdev.h 中的 netlink 属性 NETDEV_A_DEV_XDP_ZC_MAX_SEGS。将返回一个无符号整数值，
表示此设备在零拷贝模式下支持的最大 frags 数量。以下是可能的返回值：

1：该设备不支持零拷贝多缓冲区，因为最多支持一个 fragment 意味着无法进行多缓冲区。

>=2：该设备在零拷贝模式下支持多缓冲区。返回的数字表示支持的最大 frags 数量。

关于如何通过这些接口（通过 libbpf）使用的示例，请参考 tools/testing/selftests/bpf/
xskxceiver.c。

### 零拷贝驱动的多缓冲区支持


零拷贝驱动通常使用批处理 API 进行 Rx 和 Tx 处理。请注意，Tx 批处理 API 保证它会提供一批
以完整数据包结尾的 Tx 描述符。这是为了便于为零拷贝驱动扩展多缓冲区支持。

## 示例应用程序


有一个名为 xdpsock 的基准测试 / 测试应用程序，可在
https://github.com/xdp-project/bpf-examples/tree/main/AF_XDP-example 找到，它演示了如何
将 AF_XDP 套接字与私有 UMEM 一起使用。假设你想让来自端口 4242 的 UDP 流量最终进入队列
16，我们将在该队列上启用 AF_XDP。这里我们使用 ethtool：
```

      ethtool -N p3p2 rx-flow-hash udp4 fn
      ethtool -N p3p2 flow-type udp4 src-port 4242 dst-port 4242 \
          action 16

```
在 XDP_DRV 模式下运行 rxdrop 基准测试可以这样做：
```

      samples/bpf/xdpsock -i p3p2 -q 16 -r -N

```
对于 XDP_SKB 模式，使用开关 "-S" 代替 "-N"，所有选项都可以像往常一样用 "-h" 显示。

这个示例应用程序使用 libbpf 来简化 AF_XDP 的建立和使用。如果你想了解 AF_XDP 的原始 uapi
是如何真正用于实现更高级功能的，请查看 tools/testing/selftests/bpf/xsk.[ch] 中的
libbpf 代码。

## 常见问题


问：我在套接字上看不到任何流量。我做错了什么？

答：当物理 NIC 的 netdev 被初始化时，Linux 通常每个核分配一对 RX 和 TX 队列。因此在一个
8 核系统上，会分配队列 id 0 到 7，每个核一个。在 AF_XDP 的 bind 调用或 xsk_socket__create
libbpf 函数调用中，你指定一个要绑定的特定队列 id，而你在该套接字上只能收到流向该队列的
流量。所以在上面的例子中，如果你绑定到队列 0，你将看不到任何被分发到队列 1 到 7 的流量。
如果运气好，你会看到这些流量，但通常它们会落到某个你没有绑定的队列上。

有若干种方法可以解决把你想要的流量送到所绑定队列 id 的问题。如果你想看到所有流量，你可以
强制 netdev 只拥有 1 个队列，队列
```

     sudo ethtool -L <interface> combined 1

    If you want to only see part of the traffic, you can program the
    NIC through ethtool to filter out your traffic to a single queue id
    that you can bind your XDP socket to. Here is one example in which
    UDP traffic to and from port 4242 are sent to queue 2::

      sudo ethtool -N <interface> rx-flow-hash udp4 fn
      sudo ethtool -N <interface> flow-type udp4 src-port 4242 dst-port \
      4242 action 2

    A number of other ways are possible all up to the capabilities of
    the NIC you have.

```

问：我可以使用 XSKMAP 在拷贝模式下实现不同 umem 之间的切换吗？

答：简短的回答是不行，目前不支持。XSKMAP 只能用于把进入队列 id X 的流量切换到绑定到同一
队列 id X 的套接字。XSKMAP 可以包含绑定到不同队列 id（例如 X 和 Y）的套接字，但只有来自
队列 id Y 的流量才能被导向绑定到同一队列 id Y 的套接字。在零拷贝模式下，你应该在你的 NIC
中使用 switch 或其他分发机制，把流量导向正确的队列 id 和套接字。

问：我的数据包有时会损坏。哪里出错了？

答：必须小心不要把 UMEM 中的同一个缓冲区同时喂给多个环。例如，如果你把同一个缓冲区同时
喂给 FILL 环和 TX 环，NIC 可能会在缓冲区发送数据的同时接收数据到该缓冲区。这会导致某些
数据包损坏。把同一个缓冲区喂给属于不同队列 id 或由 XDP_SHARED_UMEM 标志绑定的不同 netdev
的 FILL 环也会有同样的问题。

## 致谢


- Björn Töpel (AF_XDP core)
- Magnus Karlsson (AF_XDP core)
- Alexander Duyck
- Alexei Starovoitov
- Daniel Borkmann
- Jesper Dangaard Brouer
- John Fastabend
- Jonathan Corbet (LWN coverage)
- Michael S. Tsirkin
- Qi Z Zhang
- Willem de Bruijn
