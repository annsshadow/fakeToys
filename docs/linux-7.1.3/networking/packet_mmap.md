## Packet MMAP（Packet 内存映射

## 摘要


本文件记录了 PACKET 套接字接口所提供mmap() 机制。这类套接字用于
i) 使用 tcpdump 之类的工具捕获网络流量，
ii) 发送网络流量，或任何其他需要直接（raw    访问网络接口的场景
使用方法的详细介绍可在以下地址找到
    https://web.archive.org/web/20220404160947/https://sites.google.com/site/packetmmap/

请将您的意见发送给我们    - Ulisses Alonso Camaró <uaca@i.hate.spam.alumni.uv.es>
    - Johann Baudy

## 为什么要使用 PACKET_MMAP


不使PACKET_MMAP 的捕获过程（AF_PACKET）效率非常低。它使用非常有限
的缓冲区，并且每捕获一个数据包就需要一次系统调用；若还想获取数据包的时间戳
（libpcap 总是如此），则需要两次系统调用
相反，PACKET_MMAP 的效率很高。PACKET_MMAP 提供了一个大小可配置、映射到用户空间
的环形缓冲区（circular buffer），可用于接收或发送数据包。这样读取数据包只需等待
它们到来，大多数情况下不需要发出任何系统调用。在发送方面，可以通过一次系统调发送多个数据包以获得最高带宽。由于在内核与用户之间使用了共享缓冲区，还带来了
减少数据包拷贝次数的好处
使用 PACKET_MMAP 来提升捕获和发送过程的性能是合适的，但它并非全部。至少，如果你在
高速（相对CPU 速度而言）捕获，应当检查你的网卡（network interface card）设备驱动是支持某种中断负载缓解机制，或者（更好）是否支NAPI，并且确保它已启用。对于发送，
请检查你的网络设备所使用和支持的 MTU（Maximum Transmission Unit，最大传输单元）。将你的
网卡中断（IRQ）绑定到特定 CPU 也会带来好处
## 如何使用 mmap() 提升捕获过程


从用户的角度看，你应该使用更高层libpcap 库，它是事实上的标准，几乎可移植包括 Win32 在内的所有操作系统
Packet MMAP 的支持大约是1.3.0 版本时集成进 libpcap 的；TPACKET_V3 的支持在
1.5.0 版本中加入
## 如何直接使用 mmap() 提升捕获过程


从系统调用的角度看，PACKET_MMAP 的使用涉```

    [setup]     socket() -------> creation of the capture socket
		setsockopt() ---> allocation of the circular buffer (ring)
				  option: PACKET_RX_RING
		mmap() ---------> mapping of the allocated buffer to the
				  user process

    [capture]   poll() ---------> to wait for incoming packets

    [shutdown]  close() --------> destruction of the capture socket and
				  deallocation of all associated
				  resources.


```
套接字的创建与销毁都很直接，通过如下方式完成
```

 int fd = socket(PF_PACKET, mode, htons(ETH_P_ALL));

```
其中 mode SOCK_RAW 表示原始（raw）接口，可捕获链路层信息；或SOCK_DGRAM
表示“熟”（cooked）接口，其中不支持捕获链路层信息，而是由内核提供一个链路层
伪头部（pseudo-header）
套接字及所有相关资源的销毁通过简单地调用 close(fd) 来完成
与使PACKET_MMAP 无关，也可以用一个套接字同时进行捕获和发送。这只需用一mmap() 调用同时映射已分配的 RX TX 缓冲区环（ring）即可。参见“环形缓冲区（ring的映射与使用”
接下来我将描PACKET_MMAP 的设置及其约束，以及环形缓冲区在用户进程中的映射
和该缓冲区的使用
## 如何直接使用 mmap() 提升发送过```

    [setup]         socket() -------> creation of the transmission socket
		    setsockopt() ---> allocation of the circular buffer (ring)
				      option: PACKET_TX_RING
		    bind() ---------> bind transmission socket with a network interface
		    mmap() ---------> mapping of the allocated buffer to the
				      user process

    [transmission]  poll() ---------> wait for free packets (optional)
		    send() ---------> send all packets that are set as ready in
				      the ring
				      The flag MSG_DONTWAIT can be used to return
				      before end of transfer.

    [shutdown]      close() --------> destruction of the transmission socket and
				      deallocation of all associated resources.

```
套接字的创建与销毁同样很直接，通过如下方式完成
```

 int fd = socket(PF_PACKET, mode, 0);

```
如果我们只通过该套接字发送，协议（protocol）可以可选地设为 0，从而避免一次昂贵的
packet_rcv() 调用。这种情况下，你还需要将 TX_RING sll_protocol = 0 进行
bind(2) 绑定。否则，例如使用 htons(ETH_P_ALL) 或任何其他协议
将套接字绑定到你的网络接口是必须的（采用零拷贝时），以便获知环形缓冲区中使用帧头部大小```

    --------------------
    | struct tpacket_hdr | Header. It contains the status of
    |                    | of this frame
    |--------------------|
    | data buffer        |
    .                    .  Data that will be sent over the network interface.
    .                    .
    --------------------

 bind() associates the socket to your network interface thanks to
 sll_ifindex parameter of struct sockaddr_ll.

 Initialization example::

    struct sockaddr_ll my_addr;
    struct ifreq s_ifr;
    ...

    strscpy_pad (s_ifr.ifr_name, "eth0", sizeof(s_ifr.ifr_name));

    /* get interface index of eth0 */
    ioctl(this->socket, SIOCGIFINDEX, &s_ifr);

    /* fill sockaddr_ll struct to prepare binding */
    my_addr.sll_family = AF_PACKET;
    my_addr.sll_protocol = htons(ETH_P_ALL);
    my_addr.sll_ifindex =  s_ifr.ifr_ifindex;

    /* bind socket to eth0 */
    bind(this->socket, (struct sockaddr *)&my_addr, sizeof(struct sockaddr_ll));

 A complete tutorial is available at:
 https://web.archive.org/web/20220404160947/https://sites.google.com/site/packetmmap/

```
```

 frame base + TPACKET_HDRLEN - sizeof(struct sockaddr_ll)

```
因此，无论你为套接字模式选择什么（SOCK_DGRAM SOCK_RAW），
```

 frame base + TPACKET_ALIGN(sizeof(struct tpacket_hdr))

```
如果你想将用户数据放在距离帧起始处的自定义偏移位置（例如为了SOCK_RAW 模式下的
负载对齐），可以设置 tp_net（配SOCK_DGRAM）或 tp_mac（配SOCK_RAW）。为了使生效，必须事先通过 setsockopt() PACKET_TX_HAS_OFF 选项启用它
## PACKET_MMAP 设置


要在用户级代码中设置 PACKET_MMAP，是通过类似如下的调用来完成
```

     setsockopt(fd, SOL_PACKET, PACKET_RX_RING, (void *) &req, sizeof(req))

 - Transmission process::

     setsockopt(fd, SOL_PACKET, PACKET_TX_RING, (void *) &req, sizeof(req))

```
上述调用中最重要的参数是 req 参数```

    struct tpacket_req
    {
	unsigned int    tp_block_size;  /* Minimal size of contiguous block */
	unsigned int    tp_block_nr;    /* Number of blocks */
	unsigned int    tp_frame_size;  /* Size of frame */
	unsigned int    tp_frame_nr;    /* Total number of frames */
    };

```
该结构定义在 /usr/include/linux/if_packet.h 中，它建立了一个不可换出（unswappable内存的环形缓冲区（ring）
它被映射到捕获进程后，允许在不发出系统调用的情况下读取已捕获的帧以及时间戳等
相关元信息
帧被分组到块（block）中。每个块是一段物理连续的内存区域，包tp_block_size/tp_frame_size 个帧。总数量满```

    frames_per_block = tp_block_size/tp_frame_size

```
```

    frames_per_block * tp_block_nr == tp_frame_nr

```
```

     tp_block_size= 4096
     tp_frame_size= 2048
     tp_block_nr  = 4
     tp_frame_nr  = 8

```
```

	    block #1                 block #2
    +---------+---------+    +---------+---------+
    | frame 1 | frame 2 |    | frame 3 | frame 4 |
    +---------+---------+    +---------+---------+

	    block #3                 block #4
    +---------+---------+    +---------+---------+
    | frame 5 | frame 6 |    | frame 7 | frame 8 |
    +---------+---------+    +---------+---------+

```
一个帧可以是任意大小，唯一条件是它能放入一个块中。一个块只能容纳整数个帧，换句话说，
一个帧不能跨越两个块，因此在选择 frame_size 时有些细节需要注意。参见“环形缓冲区（ring的映射与使用”
## PACKET_MMAP 设置约束


在内核版2.4.26 之前（针2.4 分支）和 2.6.5 之前.6 分支），PACKET_MMAP 缓冲区在
32 位架构上最多只能容32768 个帧，在 64 位架构上最多只能容16384 个帧
### 块大小限

如前所述，每个块都是一段连续的物理内存区域。这些内存区域是通过调用
__get_free_pages() 函数分配的。顾名思义，该函数分配内存页，第二个参数是“order”，2 的幂次页数量，也就是（当 PAGE_SIZE == 4096 时）order=0 ==> 4096 字节，order=1 ==>
8192 字节，order=2 ==> 16384 字节，依此类推。由 __get_free_pages 分配的区域的最大大MAX_PAGE_ORDER 宏决定```

   PAGE_SIZE << MAX_PAGE_ORDER

   In a i386 architecture PAGE_SIZE is 4096 bytes
   In a 2.4/i386 kernel MAX_PAGE_ORDER is 10
   In a 2.6/i386 kernel MAX_PAGE_ORDER is 11

```
因此2.4/2.6 内核中，配合 i386 架构，get_free_pages 最多可分配 4MB 8MB
用户空间程序可以包含 /usr/include/sys/user.h /usr/include/linux/mmzone.h 来获PAGE_SIZE、MAX_PAGE_ORDER 的声明
页大小也可以通过 getpagesize (2) 系统调用动态确定
### 块数量限

为了理解 PACKET_MMAP 的约束，我们需要查看用于保存每个块指针的结构
目前，该结构是一个用 kmalloc 动态分配的向量
```

    +---+---+---+---+
    | x | x | x | x |
    +---+---+---+---+
      |   |   |   |
      |   |   |   v
      |   |   v  block #4
      |   v  block #3
      v  block #2
     block #1

```
kmalloc 从一组预先确定的大小的内存池中分配任意字节数的物理连续内存。该内存池由
slab 分配器维护，最终由它负责完成分配，因此也由它限制了 kmalloc 能分配的最大内存
2.4/2.6 内核i386 架构上，限制131072 字节。kmalloc 使用的预定大小可以在
/proc/slabinfo 的“size-<bytes>”条目中查看
32 位架构上，指针长度为 4 字节，因此总块数为
```

     131072/4 = 32768 blocks

```
## PACKET_MMAP 缓冲区大小计算器


定义
==============  ================================================================
<size-max>      is the maximum size of allocable with kmalloc
		(see /proc/slabinfo)
<pointer size>  depends on the architecture -- `sizeof(void *)`
<page size>     depends on the architecture -- PAGE_SIZE or getpagesize (2)
<max-order>     is the value defined with MAX_PAGE_ORDER
<frame size>    it's an upper bound of frame's capture size (more on this later)
==============  ================================================================

```

	<block number> = <size-max>/<pointer size>
	<block size> = <pagesize> << <max-order>

```
```

	<block number> * <block size>

```
```

	<block number> * <block size> / <frame size>

```
假设有以下参数，适用2.6 内核```

	<size-max> = 131072 bytes
	<pointer size> = 4 bytes
	<pagesize> = 4096 bytes
	<max-order> = 11

```
```

	<block number> = 131072/4 = 32768 blocks
	<block size> = 4096 << 11 = 8 MiB.

```
因此缓冲区将262144 MiB 大小。它可以容纳 262144 MiB / 2048 字节 = 134217728 个帧
实际上，这个缓冲区大小在 i386 架构上是不可能的。记住，内存是在内核空间分配的，对于
i386 内核，内存大小限制为 1GiB
所有内存分配都不会被释放，直到套接字关闭。内存分配以 GFP_KERNEL 优先级进行，这基本上
意味着分配可以等待并换出其他进程的内存以分配所需内存，因此通常可以达到上限
### 其他约束


如果你查看源代码，你会看到我这里画成“帧”的并不只是链路层帧。在每个帧的开头有一称为 struct tpacket_hdr 的头部，用于 PACKET_MMAP 中保存链路层帧的元信息，如时间戳所以我们这里画的“帧”实际上```

 /*
   Frame structure:

   - Start. Frame must be aligned to TPACKET_ALIGNMENT=16
   - struct tpacket_hdr
   - pad to TPACKET_ALIGNMENT=16
   - struct sockaddr_ll
   - Gap, chosen so that packet data (Start+tp_net) aligns to
     TPACKET_ALIGNMENT=16
   - Start+tp_mac: [ Optional MAC header ]
   - Start+tp_net: Packet data, aligned to TPACKET_ALIGNMENT=16.
   - Pad to align to TPACKET_ALIGNMENT=16
 */

```
以下packet_set_ring 中会检查的条件

   - tp_block_size must be a multiple of PAGE_SIZE (1)
   - tp_frame_size must be greater than TPACKET_HDRLEN (obvious)
   - tp_frame_size must be a multiple of TPACKET_ALIGNMENT
   - tp_frame_nr   must be exactly frames_per_block*tp_block_nr

注意 tp_block_size 应选择2 的幂，否则会浪费内存
### 环形缓冲区（ring）的映射与使

缓冲区在用户进程中的映射是通过常规mmap 函数完成的。即使环形缓冲区由若干物理上
不连续的内存块组成，它们在用户空间看来是连续的，因此
```

    mmap(0, size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);

```
如果 tp_frame_size tp_block_size 的约数，帧将tp_frame_size 字节间隔连续排列如果不是，则tp_block_size/tp_frame_size 个帧之间会有一个间隙（gap）。这是因为一帧不能跨越两个块
要在一个套接字上同时进行捕获和发送，对两者的映射```

    ...
    setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &foo, sizeof(foo));
    setsockopt(fd, SOL_PACKET, PACKET_TX_RING, &bar, sizeof(bar));
    ...
    rx_ring = mmap(0, size * 2, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    tx_ring = rx_ring + size;

```
RX 必须在前，因为内核紧接着 RX 之后映射 TX 环内存
在每个帧的开头有一个状态字段（参见 struct tpacket_hdr）。如果该字段0，表示该可供内核使用；否则，存在一个用户可读的帧，适用以下标志
##### 捕获过程
```

     #define TP_STATUS_COPY          (1 << 1)
     #define TP_STATUS_LOSING        (1 << 2)
     #define TP_STATUS_CSUMNOTREADY  (1 << 3)
     #define TP_STATUS_CSUM_VALID    (1 << 7)

```
======================  =======================================================
TP_STATUS_COPY		This flag indicates that the frame (and associated
			meta information) has been truncated because it's
			larger than tp_frame_size. This packet can be
			read entirely with recvfrom().

			In order to make this work it must to be
			enabled previously with setsockopt() and
			the PACKET_COPY_THRESH option.

			The number of frames that can be buffered to
			be read with recvfrom is limited like a normal socket.
			See the SO_RCVBUF option in the socket (7) man page.

TP_STATUS_LOSING	indicates there were packet drops from last time
			statistics where checked with getsockopt() and
			the PACKET_STATISTICS option.

TP_STATUS_CSUMNOTREADY	currently it's used for outgoing IP packets which
			its checksum will be done in hardware. So while
			reading the packet we should not try to check the
			checksum.

TP_STATUS_CSUM_VALID	This flag indicates that at least the transport
			header checksum of the packet has been already
			validated on the kernel side. If the flag is not set
			then we are free to check the checksum by ourselves
			provided that TP_STATUS_CSUMNOTREADY is also not set.
======================  =======================================================

```

     #define TP_STATUS_KERNEL        0
     #define TP_STATUS_USER          1

```
内核将所有帧初始化为 TP_STATUS_KERNEL，当内核接收到一个数据包时，它将其放入缓冲区并更新状态为至少包含 TP_STATUS_USER 标志。然后用户可以读取该数据包，读取完后用户必须
将状态字段清零，以便内核可以再次使用该帧缓冲区
用户可以使用 poll（其他变体也应适用）来检查是否有```

    struct pollfd pfd;

    pfd.fd = fd;
    pfd.revents = 0;
    pfd.events = POLLIN|POLLRDNORM|POLLERR;

    if (status == TP_STATUS_KERNEL)
	retval = poll(&pfd, 1, timeout);

```
先检查状态值再 poll 等待帧，并不会产生竞争条件
##### 发送过```

     #define TP_STATUS_AVAILABLE        0 // Frame is available
     #define TP_STATUS_SEND_REQUEST     1 // Frame will be sent on next send()
     #define TP_STATUS_SENDING          2 // Frame is currently in transmission
     #define TP_STATUS_WRONG_FORMAT     4 // Frame format is not correct

```
首先，内核将所有帧初始化为 TP_STATUS_AVAILABLE。要发送一个数据包，用户填充一可用帧的数据缓冲区，tp_len 设为当前数据缓冲区大小，并将其状态字段设TP_STATUS_SEND_REQUEST。这可以在多个帧上完成。一旦用户准备好发送，就调send()然后所有状态等TP_STATUS_SEND_REQUEST 的缓冲区被转发到网络设备。内核将每个已发帧的状态更新为 TP_STATUS_SENDING，直到传输结束
每次传输结束时，缓冲区状态恢复为 TP_STATUS_AVAILABLE```

    header->tp_len = in_i_size;
    header->tp_status = TP_STATUS_SEND_REQUEST;
    retval = send(this->socket, NULL, 0, 0);

```
用户也可以使poll() 来检查缓冲区是否可用
(status == TP_STATUS_SENDING)
```

    struct pollfd pfd;
    pfd.fd = fd;
    pfd.revents = 0;
    pfd.events = POLLOUT;
    retval = poll(&pfd, 1, timeout);

```
## 有哪TPACKET 版本可用，何时使用它们？
```

 int val = tpacket_version;
 setsockopt(fd, SOL_PACKET, PACKET_VERSION, &val, sizeof(val));
 getsockopt(fd, SOL_PACKET, PACKET_VERSION, &val, sizeof(val));

```
其中 'tpacket_version' 可以TPACKET_V1（默认）、TPACKET_V2、TPACKET_V3
TPACKET_V1 - 若未通过 setsockopt(2) 另行指定，则为默认版 - 提供 RX_RING、TX_RING

TPACKET_V1 --> TPACKET_V2 - 由于 TPACKET_V1 结构中使用了 unsigned long，改64 位干净4 bit clean），
	  因此也能64 位内+ 32 位用户空间等组合下工 - 时间戳分辨率由微秒改为纳 - 提供 RX_RING、TX_RING
 - 数据包的 VLAN 元信息可	  （TP_STATUS_VLAN_VALID、TP_STATUS_VLAN_TPID_VALID），
	  tpacket2_hdr 结构中：

  - tp_status 字段中设置了 TP_STATUS_VLAN_VALID 位，表示
		  tp_vlan_tci 字段含有有效VLAN TCI   - tp_status 字段中设置了 TP_STATUS_VLAN_TPID_VALID 位，表示
		  tp_vlan_tpid 字段含有有效VLAN TPID 
 - 如何切换TPACKET_V2
  1. struct tpacket2_hdr 替换 struct tpacket_hdr
  2. 查询并保存头部长  3. 将协议版本设2，照常建ring
  4. 获取 sockaddr_ll 时，
		   使用 `(void *)hdr + TPACKET_ALIGN(hdrlen)` 而非
		   `(void *)hdr + TPACKET_ALIGN(sizeof(struct tpacket_hdr))`

TPACKET_V2 --> TPACKET_V3 - RX_RING 的灵活缓冲区实现  1. 块可配置为非静态帧大小
  2. poll 在块级别（而非包级别）进行
  3. 增加poll 超时，避免用户空间在空闲链路上无限等  4. 增加了用户可配置的参数：

			4.1 block::timeout
			4.2 tpkt_hdr::sk_rxhash

 - 用户空间可获RX Hash 数据
 - TX_RING 语义在概念上类似TPACKET_V2	  使用 tpacket3_hdr 而非 tpacket2_hdr，以TPACKET3_HDRLEN
	  而非 TPACKET2_HDRLEN。在当前实现中，tpacket3_hdr 中的 tp_next_offset
	  字段必须设为零，表示 ring 不保存可变大小的帧。tp_next_offset 非零	  数据包将被丢弃
## AF_PACKET fanout 模式


AF_PACKET fanout 模式下，数据包接收可以在多个进程间进行负载均衡。这也可以与
packet 套接字上mmap(2) 结合使用
当前已实现的 fanout 策略有：

  - PACKET_FANOUT_HASH：按 skb 的数据包哈希调度到套接字
  - PACKET_FANOUT_LB：按轮询（round-robin）调度到套接  - PACKET_FANOUT_CPU：按数据包到达的 CPU 调度到套接字
  - PACKET_FANOUT_RND：按随机选择调度到套接字
  - PACKET_FANOUT_ROLLOVER：若某个套接字已满，则滚动到另一  - PACKET_FANOUT_QM：按 skb 记录queue_mapping 调度到套接字

David S. Miller 提供的最小示例代码（可以尝试 "./test eth0 hash" 之类）：
```

    #include <stddef.h>
    #include <stdlib.h>
    #include <stdio.h>
    #include <string.h>

    #include <sys/types.h>
    #include <sys/wait.h>
    #include <sys/socket.h>
    #include <sys/ioctl.h>

    #include <unistd.h>

    #include <linux/if_ether.h>
    #include <linux/if_packet.h>

    #include <net/if.h>

    static const char *device_name;
    static int fanout_type;
    static int fanout_id;

    #ifndef PACKET_FANOUT
    # define PACKET_FANOUT			18
    # define PACKET_FANOUT_HASH		0
    # define PACKET_FANOUT_LB		1
    #endif

    static int setup_socket(void)
    {
	    int err, fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_IP));
	    struct sockaddr_ll ll;
	    struct ifreq ifr;
	    int fanout_arg;

	    if (fd < 0) {
		    perror("socket");
		    return EXIT_FAILURE;
	    }

	    memset(&ifr, 0, sizeof(ifr));
	    strcpy(ifr.ifr_name, device_name);
	    err = ioctl(fd, SIOCGIFINDEX, &ifr);
	    if (err < 0) {
		    perror("SIOCGIFINDEX");
		    return EXIT_FAILURE;
	    }

	    memset(&ll, 0, sizeof(ll));
	    ll.sll_family = AF_PACKET;
	    ll.sll_ifindex = ifr.ifr_ifindex;
	    err = bind(fd, (struct sockaddr *) &ll, sizeof(ll));
	    if (err < 0) {
		    perror("bind");
		    return EXIT_FAILURE;
	    }

	    fanout_arg = (fanout_id | (fanout_type << 16));
	    err = setsockopt(fd, SOL_PACKET, PACKET_FANOUT,
			    &fanout_arg, sizeof(fanout_arg));
	    if (err) {
		    perror("setsockopt");
		    return EXIT_FAILURE;
	    }

	    return fd;
    }

    static void fanout_thread(void)
    {
	    int fd = setup_socket();
	    int limit = 10000;

	    if (fd < 0)
		    exit(fd);

	    while (limit-- > 0) {
		    char buf[1600];
		    int err;

		    err = read(fd, buf, sizeof(buf));
		    if (err < 0) {
			    perror("read");
			    exit(EXIT_FAILURE);
		    }
		    if ((limit % 10) == 0)
			    fprintf(stdout, "(%d) \n", getpid());
	    }

	    fprintf(stdout, "%d: Received 10000 packets\n", getpid());

	    close(fd);
	    exit(0);
    }

    int main(int argc, char **argp)
    {
	    int fd, err;
	    int i;

	    if (argc != 3) {
		    fprintf(stderr, "Usage: %s INTERFACE {hash|lb}\n", argp[0]);
		    return EXIT_FAILURE;
	    }

	    if (!strcmp(argp[2], "hash"))
		    fanout_type = PACKET_FANOUT_HASH;
	    else if (!strcmp(argp[2], "lb"))
		    fanout_type = PACKET_FANOUT_LB;
	    else {
		    fprintf(stderr, "Unknown fanout type [%s]\n", argp[2]);
		    exit(EXIT_FAILURE);
	    }

	    device_name = argp[1];
	    fanout_id = getpid() & 0xffff;

	    for (i = 0; i < 4; i++) {
		    pid_t pid = fork();

		    switch (pid) {
		    case 0:
			    fanout_thread();

		    case -1:
			    perror("fork");
			    exit(EXIT_FAILURE);
		    }
	    }

	    for (i = 0; i < 4; i++) {
		    int status;

		    wait(&status);
	    }

	    return 0;
    }

```
## AF_PACKET TPACKET_V3 示例


AF_PACKET TPACKET_V3 环形缓冲区可配置为使用非静态帧大小，通过它自身的内存管理
实现。它基于块（block）工作，轮询（polling）按每块进行，而非TPACKET_V2 及其前身
那样按每ring 进行
据说 TPACKET_V3 带来以下好处
 - CPU 使用率降低约 15% - 20%
 - 数据包捕获率提升20%
 - 数据包密度提升约 2  - 端口聚合分析
 - 非静态帧大小以捕获完整的数据包负
因此它似乎是配合 packet fanout 使用的良好候选
Daniel Borkmann 基于 Chetan Loke lolpcap 提供的最小示例代码（编译
```

    /* Written from scratch, but kernel-to-user space API usage
    * dissected from lolpcap:
    *  Copyright 2011, Chetan Loke <loke.chetan@gmail.com>
    *  License: GPL, version 2.0
    */

    #include <stdio.h>
    #include <stdlib.h>
    #include <stdint.h>
    #include <string.h>
    #include <assert.h>
    #include <net/if.h>
    #include <arpa/inet.h>
    #include <netdb.h>
    #include <poll.h>
    #include <unistd.h>
    #include <signal.h>
    #include <inttypes.h>
    #include <sys/socket.h>
    #include <sys/mman.h>
    #include <linux/if_packet.h>
    #include <linux/if_ether.h>
    #include <linux/ip.h>

    #ifndef likely
    # define likely(x)		__builtin_expect(!!(x), 1)
    #endif
    #ifndef unlikely
    # define unlikely(x)		__builtin_expect(!!(x), 0)
    #endif

    struct block_desc {
	    uint32_t version;
	    uint32_t offset_to_priv;
	    struct tpacket_hdr_v1 h1;
    };

    struct ring {
	    struct iovec *rd;
	    uint8_t *map;
	    struct tpacket_req3 req;
    };

    static unsigned long packets_total = 0, bytes_total = 0;
    static sig_atomic_t sigint = 0;

    static void sighandler(int num)
    {
	    sigint = 1;
    }

    static int setup_socket(struct ring *ring, char *netdev)
    {
	    int err, i, fd, v = TPACKET_V3;
	    struct sockaddr_ll ll;
	    unsigned int blocksiz = 1 << 22, framesiz = 1 << 11;
	    unsigned int blocknum = 64;

	    fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
	    if (fd < 0) {
		    perror("socket");
		    exit(1);
	    }

	    err = setsockopt(fd, SOL_PACKET, PACKET_VERSION, &v, sizeof(v));
	    if (err < 0) {
		    perror("setsockopt");
		    exit(1);
	    }

	    memset(&ring->req, 0, sizeof(ring->req));
	    ring->req.tp_block_size = blocksiz;
	    ring->req.tp_frame_size = framesiz;
	    ring->req.tp_block_nr = blocknum;
	    ring->req.tp_frame_nr = (blocksiz * blocknum) / framesiz;
	    ring->req.tp_retire_blk_tov = 60;
	    ring->req.tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;

	    err = setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &ring->req,
			    sizeof(ring->req));
	    if (err < 0) {
		    perror("setsockopt");
		    exit(1);
	    }

	    ring->map = mmap(NULL, ring->req.tp_block_size * ring->req.tp_block_nr,
			    PROT_READ | PROT_WRITE, MAP_SHARED | MAP_LOCKED, fd, 0);
	    if (ring->map == MAP_FAILED) {
		    perror("mmap");
		    exit(1);
	    }

	    ring->rd = malloc(ring->req.tp_block_nr * sizeof(*ring->rd));
	    assert(ring->rd);
	    for (i = 0; i < ring->req.tp_block_nr; ++i) {
		    ring->rd[i].iov_base = ring->map + (i * ring->req.tp_block_size);
		    ring->rd[i].iov_len = ring->req.tp_block_size;
	    }

	    memset(&ll, 0, sizeof(ll));
	    ll.sll_family = PF_PACKET;
	    ll.sll_protocol = htons(ETH_P_ALL);
	    ll.sll_ifindex = if_nametoindex(netdev);
	    ll.sll_hatype = 0;
	    ll.sll_pkttype = 0;
	    ll.sll_halen = 0;

	    err = bind(fd, (struct sockaddr *) &ll, sizeof(ll));
	    if (err < 0) {
		    perror("bind");
		    exit(1);
	    }

	    return fd;
    }

    static void display(struct tpacket3_hdr *ppd)
    {
	    struct ethhdr *eth = (struct ethhdr *) ((uint8_t *) ppd + ppd->tp_mac);
	    struct iphdr *ip = (struct iphdr *) ((uint8_t *) eth + ETH_HLEN);

	    if (eth->h_proto == htons(ETH_P_IP)) {
		    struct sockaddr_in ss, sd;
		    char sbuff[NI_MAXHOST], dbuff[NI_MAXHOST];

		    memset(&ss, 0, sizeof(ss));
		    ss.sin_family = PF_INET;
		    ss.sin_addr.s_addr = ip->saddr;
		    getnameinfo((struct sockaddr *) &ss, sizeof(ss),
				sbuff, sizeof(sbuff), NULL, 0, NI_NUMERICHOST);

		    memset(&sd, 0, sizeof(sd));
		    sd.sin_family = PF_INET;
		    sd.sin_addr.s_addr = ip->daddr;
		    getnameinfo((struct sockaddr *) &sd, sizeof(sd),
				dbuff, sizeof(dbuff), NULL, 0, NI_NUMERICHOST);

		    printf("%s -> %s, ", sbuff, dbuff);
	    }

	    printf("rxhash: 0x%x\n", ppd->hv1.tp_rxhash);
    }

    static void walk_block(struct block_desc *pbd, const int block_num)
    {
	    int num_pkts = pbd->h1.num_pkts, i;
	    unsigned long bytes = 0;
	    struct tpacket3_hdr *ppd;

	    ppd = (struct tpacket3_hdr *) ((uint8_t *) pbd +
					pbd->h1.offset_to_first_pkt);
	    for (i = 0; i < num_pkts; ++i) {
		    bytes += ppd->tp_snaplen;
		    display(ppd);

		    ppd = (struct tpacket3_hdr *) ((uint8_t *) ppd +
						ppd->tp_next_offset);
	    }

	    packets_total += num_pkts;
	    bytes_total += bytes;
    }

    static void flush_block(struct block_desc *pbd)
    {
	    pbd->h1.block_status = TP_STATUS_KERNEL;
    }

    static void teardown_socket(struct ring *ring, int fd)
    {
	    munmap(ring->map, ring->req.tp_block_size * ring->req.tp_block_nr);
	    free(ring->rd);
	    close(fd);
    }

    int main(int argc, char **argp)
    {
	    int fd, err;
	    socklen_t len;
	    struct ring ring;
	    struct pollfd pfd;
	    unsigned int block_num = 0, blocks = 64;
	    struct block_desc *pbd;
	    struct tpacket_stats_v3 stats;

	    if (argc != 2) {
		    fprintf(stderr, "Usage: %s INTERFACE\n", argp[0]);
		    return EXIT_FAILURE;
	    }

	    signal(SIGINT, sighandler);

	    memset(&ring, 0, sizeof(ring));
	    fd = setup_socket(&ring, argp[argc - 1]);
	    assert(fd > 0);

	    memset(&pfd, 0, sizeof(pfd));
	    pfd.fd = fd;
	    pfd.events = POLLIN | POLLERR;
	    pfd.revents = 0;

	    while (likely(!sigint)) {
		    pbd = (struct block_desc *) ring.rd[block_num].iov_base;

		    if ((pbd->h1.block_status & TP_STATUS_USER) == 0) {
			    poll(&pfd, 1, -1);
			    continue;
		    }

		    walk_block(pbd, block_num);
		    flush_block(pbd);
		    block_num = (block_num + 1) % blocks;
	    }

	    len = sizeof(stats);
	    err = getsockopt(fd, SOL_PACKET, PACKET_STATISTICS, &stats, &len);
	    if (err < 0) {
		    perror("getsockopt");
		    exit(1);
	    }

	    fflush(stdout);
	    printf("\nReceived %u packets, %lu bytes, %u dropped, freeze_q_cnt: %u\n",
		stats.tp_packets, bytes_total, stats.tp_drops,
		stats.tp_freeze_q_cnt);

	    teardown_socket(&ring, fd);
	    return 0;
    }

```
## PACKET_QDISC_BYPASS


如果有需求像 pktgen 那样用大量数据包灌满网络，你可以在创建套接字后设置如选项
```

    int one = 1;
    setsockopt(fd, SOL_PACKET, PACKET_QDISC_BYPASS, &one, sizeof(one));

```
这有一个副作用：通过 PF_PACKET 发送的数据包会绕过内核qdisc 层，被强制直接推驱动。也就是说，数据包不会被缓冲，tc 规则（disciplines）被忽略，可能会增加丢包，并这类数据包对其他 PF_PACKET 套接字也不再可见。因此，这里已提醒你；一般来说，这对
压力测试系统的各个组件会很有用
默认情况下，PACKET_QDISC_BYPASS 是禁用的，需要在 PF_PACKET 套接字上显式启用
## PACKET_TIMESTAMP


PACKET_TIMESTAMP 设置决定mmap(2) 映射RX_RING TX_RING 中数据包元信息的
时间戳来源。如果你NIC 能够在硬件中对数据包打时间戳，你可以请求使用这些硬件
时间戳。注意：你可能需要通过 SIOCSHWTSTAMP 启用硬件时间戳的生成（参Documentation/networking/timestamping.rst 中的相关信息）```

    int req = SOF_TIMESTAMPING_RAW_HARDWARE;
    setsockopt(fd, SOL_PACKET, PACKET_TIMESTAMP, (void *) &req, sizeof(req))

```
对于 mmap(2) 映射的环形缓冲区，这类时间戳存储`tpacket{,2,3}_hdr` 结构tp_sec
`tp_{n,u}sec` 成员中。要确定报告了哪种时间戳，tp_status 字段与以下可能的位进二进制或运算…
```

    TP_STATUS_TS_RAW_HARDWARE
    TP_STATUS_TS_SOFTWARE

```
……它们等价于`SOF_TIMESTAMPING_*` 对应的位。对RX_RING，如果两者都没有设置
（即未设PACKET_TIMESTAMP），则在 PF_PACKET 的处理代码内部调用了软件回退（精度较低）
获取 TX_RING 的时间戳过程如下：i) 填充 ring 帧，ii) 调用 sendto()，例如在阻塞模式下，
iii) 等待相关帧的状态被更新，即该帧被交回给应用程序，iv) 遍历各帧以取出各自的硬件/
软件时间戳
只有）在启用了发送时间戳时，这些位才会与 TP_STATUS_AVAILABLE 进行二进| 运算因此你必须在应用程序中检查它（例如先通过 !(tp_status & (TP_STATUS_SEND_REQUEST |
TP_STATUS_SENDING)) 判断该帧是否属于应用程序，然后在第二步从 tp_status 中提取时间戳类型）！

如果你不在乎它们，即保持禁用，那么检TP_STATUS_AVAILABLE TP_STATUS_WRONG_FORMAT
就足够了。如果在 TX_RING 部分只设置了 TP_STATUS_AVAILABLE，那tp_sec tp_{n,u}sec
成员不包含有效值。对TX_RING，默认不生成时间戳！

有关硬件时间戳的更多信息，请参见 include/linux/net_tstamp.h Documentation/networking/timestamping.rst
## 杂项


- Packet 套接字与 Linux socket 过滤器配合使用得很好，因此你可能也想看看
  Documentation/networking/filter.rst

## 致谢


   Jesse Brandeburg，感谢他修正了我的语拼写错误
