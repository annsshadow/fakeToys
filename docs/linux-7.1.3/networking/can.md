## SocketCAN - 控制器局域网（Controller Area Network）


## 概述 / 什么是 SocketCAN


socketcan 包是 Linux 上 CAN 协议（Controller Area Network）的一种实现。CAN 是一项在网络化技术，在自动化、嵌入式设备和汽车领域有广泛应用。虽然此前已有基于字符设备的其他 CAN 实现，但 SocketCAN 使用了 Berkeley 套接字 API、Linux 网络栈，并将 CAN 设备驱动实现为网络接口。CAN 套接字 API 的设计尽可能类似于 TCP/IP 协议，以便熟悉网络编程的程序员能够轻松学习如何使用 CAN 套接字。


## 动机 / 为什么使用套接字 API


在 SocketCAN 之前，Linux 上已经有过 CAN 实现，因此问题来了：我们为什么要启动另一个项目。大多数现有的实现是作为某些 CAN 硬件的设备驱动出现的，它们基于字符设备，并且提供相对较少的功能。通常，只有一个特定于硬件的设备驱动，提供字符设备接口来直接向/从控制器硬件发送和接收原始 CAN 帧。帧的排队以及 ISO-TP 等高层传输协议必须在用户空间应用程序中实现。此外，大多数字符设备实现只支持一个进程在某一时刻打开设备，类似于串行接口。更换 CAN 控制器需要换用另一个设备驱动，并且常常需要让应用程序的很大部分去适配新驱动的 API。

SocketCAN 的设计就是为了克服所有这些限制。我们实现了一个新的协议族，它为用户空间应用程序提供套接字接口，并构建在 Linux 网络层之上，从而能够使用所提供的全部排队功能。CAN 控制器硬件的设备驱动作为网络设备向 Linux 网络层注册，这样来自控制器的 CAN 帧就可以被上传到网络层，再传送到 CAN 协议族模块，反之亦然。此外，协议族模块为传输协议模块提供注册 API，从而可以动态地加载或卸载任意数量的传输协议。实际上，单独的 can 核心模块不提供任何协议，并且如果不加载至少一个额外的协议模块就无法使用。可以同时打开多个套接字，在不同的或相同的协议模块上，它们可以在不同的或相同的 CAN ID 上监听/发送帧。多个套接字在同一接口上监听具有相同 CAN ID 的帧时，都会被传入相同的匹配 CAN 帧。希望使用特定传输协议（例如 ISO-TP）进行通信的应用程序，只需在打开套接字时选择该协议，然后就可以读写应用程序数据流，而不必处理 CAN-ID、帧等。

类似的、从用户空间可见的功能也可以由字符设备提供，但这会因几个原因导致技术上不优雅的解决方案：

- **使用复杂：** 应用程序不必向 socket(2) 传递协议参数并使用 bind(2) 选择 CAN 接口和 CAN ID，而是必须使用 ioctl(2) 来完成所有这些操作。

- **代码重复：** 字符设备无法利用 Linux 的网络排队代码，因此所有那些代码都必须为 CAN 网络重复实现。

- **抽象：** 在大多数现有的字符设备实现中，CAN 控制器的特定于硬件的设备驱动直接为应用程序提供字符设备。至少在 Unix 系统中，对于字符设备和块设备来说，这都是非常不寻常的。例如，你不会为串行接口的某个特定 UART、计算机中的某个特定声卡、或提供对你硬盘或磁带流设备访问的 SCSI 或 IDE 控制器提供字符设备。相反，你有抽象层，一方面为应用程序提供统一的字符或块设备接口，另一方面为特定于硬件的设备驱动提供接口。这些抽象由诸如 tty 层、音频子系统或上述设备的 SCSI 和 IDE 子系统等子系统提供。

  实现 CAN 设备驱动最简单的方式是作为不带这种（完整）抽象层的字符设备，就像大多数现有驱动所做的那样。然而，正确的做法是添加这样一个层，提供诸如为特定 CAN ID 注册、支持多个打开的文件描述符以及它们之间 CAN 帧的（解）复用、（复杂的）CAN 帧排队，以及为设备驱动提供注册 API 等全部功能。但是，这样一来，使用 Linux 内核提供的网络框架就不再更困难，甚至可能更容易，而这正是 SocketCAN 所做的。

使用 Linux 内核的网络框架只是为 Linux 实现 CAN 最自然、最合适的方式。


## SocketCAN 概念


如 socketcan-motivation 所述，SocketCAN 的主要目标是提供一个构建在 Linux 网络层之上的、面向用户空间应用程序的套接字接口。与众所周知的 TCP/IP 和以太网网络相比，CAN 总线是一个仅广播（!）的介质，它没有像以太网那样的 MAC 层寻址。CAN 标识符（can_id）用于在 CAN 总线上进行仲裁。因此 CAN-ID 必须在总线上唯一选择。在设计 CAN-ECU 网络时，CAN-ID 被映射为由特定的 ECU 发送。因此，CAN-ID 最好被视为一种源地址。


### 接收列表


多个应用程序的网络透明访问导致这样一个问题：不同的应用程序可能对来自同一 CAN 网络接口的相同 CAN-ID 感兴趣。SocketCAN 核心模块——它实现协议族 CAN——为此提供了几个高效的接收列表。例如，如果用户空间应用程序打开一个 CAN RAW 套接字，raw 协议模块本身会向 SocketCAN 核心请求用户所请求的 CAN-ID（范围）。CAN-ID 的订阅和退订可以针对特定的 CAN 接口或针对所有（!）已知的 CAN 接口，使用 SocketCAN 核心提供给 CAN 协议模块的 can_rx_(un)register() 函数（见 socketcan-core-module）。为了在运行时优化 CPU 使用率，接收列表被拆分为每个设备的若干个特定列表，以匹配给定用例所请求的过滤复杂度。


### 已发送帧的本地回环


正如其他网络概念所知，交换数据的应用程序可以运行在相同或不同的节点上而无需任何改变（除了相应的寻址信息）：


	 ___   ___   ___                   _______   ___
	| _ | | _ | | _ |                 | _   _ | | _ |
	||A|| ||B|| ||C||                 ||A| |B|| ||C||
	|___| |___| |___|                 |_______| |___|
	  |     |     |                       |       |
	-----------------(1)- CAN bus -(2)---------------

为了确保在示例 (2) 中应用程序 A 接收到的信息与在示例 (1) 中接收到的信息相同，需要在相应节点上对已经发送的 CAN 帧进行某种本地回环。

Linux 网络设备（默认情况下）只能处理依赖于介质的帧的收发。由于 CAN 总线上的仲裁，低优先级的 CAN-ID 的发送可能会被高优先级 CAN 帧的接收所延迟。为了反映节点上正确的 [#f1]_ 流量，已发送数据的回环必须在成功发送之后立即执行。如果 CAN 网络接口由于某种原因无法执行回环，SocketCAN 核心可以作为回退方案执行此任务。详见 socketcan-local-loopback2（推荐）。

回环功能默认启用，以反映 CAN 应用程序的标准网络行为。由于 RT-SocketCAN 小组的一些请求，回环可选地可以针对每个单独的套接字禁用。参见 socketcan-raw-sockets 中 CAN RAW 套接字的 sockopts。


       以及（同一）节点上像 'candump' 或 'cansniffer' 这样的工具。


### 网络问题通知


CAN 总线的使用可能会在物理层与介质访问控制层上导致若干问题。检测和记录这些底层问题对于 CAN 用户识别物理收发器层上的硬件问题以及由不同 ECU 引起的仲裁问题和错误帧是至关重要的要求。所检测到错误的出现对于诊断很重要，必须与精确的时间戳一起记录。为此，CAN 接口驱动可以生成所谓的错误消息帧（Error Message Frames），它可以与别的 CAN 帧一样可选地传递给用户应用程序。每当在物理层或 MAC 层检测到错误（例如由 CAN 控制器检测到）时，驱动就会创建相应的错误消息帧。错误消息帧可以通过常用的 CAN 过滤机制由用户应用程序请求。在这个过滤定义中，可以选择（感兴趣的）错误类型。错误消息的接收默认是禁用的。CAN 错误消息帧的格式在 Linux 头文件 "include/uapi/linux/can/error.h" 中有简要描述。


## 如何使用 SocketCAN


与 TCP/IP 一样，你首先需要打开一个套接字以通过 CAN 网络进行通信。由于 SocketCAN 实现了新的协议族，你需要将 PF_CAN 作为第一个参数传递给 socket(2) 系统调用。目前有两种 CAN 协议可供选择：raw 套接字协议和广播管理器（BCM）。因此，要打开一个套接字：
```

    s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

```
```

    s = socket(PF_CAN, SOCK_DGRAM, CAN_BCM);

```
分别地。在成功创建套接字之后，你通常会使用 bind(2) 系统调用将套接字绑定到 CAN 接口（由于寻址方式不同，这与 TCP/IP 不同——见 socketcan-concept）。在绑定（CAN_RAW）或连接（CAN_BCM）套接字之后，你可以像往常一样对套接字进行 read(2) 和 write(2)，或使用 send(2)、sendto(2)、sendmsg(2) 及其 recv* 对应操作。下面还描述了 CAN 特定的套接字选项。

经典 CAN 帧结构（即 CAN 2.0B）、CAN FD 帧结构和 sockaddr 结构定义在 include/linux/can.h 中：


    struct can_frame {
            canid_t can_id;  /** 32 bit CAN_ID + EFF/RTR/ERR flags **/
            union {
                    /* CAN frame payload length in byte (0 .. CAN_MAX_DLEN)
                     - was previously named can_dlc so we need to carry that
                     - name for legacy support
                     */
                    __u8 len;
                    __u8 can_dlc; /** deprecated **/
            };
            __u8    __pad;   /** padding **/
            __u8    __res0;  /** reserved / padding **/
            __u8    len8_dlc; /** optional DLC for 8 byte payload length (9 .. 15) **/
            __u8    data[^8^] __attribute__((aligned(8)));
    };

备注：len 元素包含载荷长度（字节），应当使用它而非 can_dlc。已废弃的 can_dlc 命名具有误导性，因为它总是包含普通的载荷长度（字节），而不是所谓的“数据长度代码”（DLC）。

为了从/向经典 CAN 网络设备传递原始 DLC，当 len 元素为 8（所有大于等于 8 的 DLC 值所对应的真实载荷长度）时，len8_dlc 元素可以包含 9 到 15 的值。

（线性）载荷 data[] 向 64 位边界的对齐允许用户定义自己的结构体和联合体来方便地访问 CAN 载荷。CAN 总线上默认没有任何给定的字节序。对 CAN_RAW 套接字的 read(2) 系统调用会将一个 struct can_frame 传送到用户空间。

sockaddr_can 结构有一个像 PF_PACKET 套接字那样的接口索引，它也绑定到特定接口：


    struct sockaddr_can {
            sa_family_t can_family;
            int         can_ifindex;
            union {
                    /** transport protocol class address info (e.g. ISOTP) **/
                    struct { canid_t rx_id, tx_id; } tp;

                    /** J1939 address information **/
                    struct {
                            /** 8 byte name when using dynamic addressing **/
                            __u64 name;

                            /* pgn:
                             - 8 bit: PS in PDU2 case, else 0
                             - 8 bit: PF
                             - 1 bit: DP
                             - 1 bit: reserved
                             */
                            __u32 pgn;

                            /** 1 byte address **/
                            __u8 addr;
                    } j1939;

                    /** reserved for future CAN protocols address information **/
            } can_addr;
    };

为了确定接口索引，必须使用一个适当的 ioctl()（以 CAN_RAW 套接字为例，未做错误检查）：


    int s;
    struct sockaddr_can addr;
    struct ifreq ifr;

    s = socket(PF_CAN, SOCK_RAW, CAN_RAW);

    strcpy(ifr.ifr_name, "can0" );
    ioctl(s, SIOCGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    bind(s, (struct sockaddr *)&addr, sizeof(addr));

    (..)

要将套接字绑定到所有（!）CAN 接口，接口索引必须为 0（零）。在这种情况下，套接字从每个已启用的 CAN 接口接收 CAN 帧。要确定源 CAN 接口，可以使用系统调用 recvfrom(2) 而非 read(2)。要在绑定到 'any' 接口的套接字上发送，需要使用 sendto(2) 来指定出口接口。

从绑定的 CAN_RAW 套接字（见上）读取 CAN 帧包括读取一个 struct can_frame：


    struct can_frame frame;

    nbytes = read(s, &frame, sizeof(struct can_frame));

    if (nbytes < 0) {
            perror("can raw socket read");
            return 1;
    }

    /** paranoid check ... **/
    if (nbytes < sizeof(struct can_frame)) {
            fprintf(stderr, "read: incomplete CAN frame\n");
            return 1;
    }

    /** do something with the received CAN frame **/

```

    nbytes = write(s, &frame, sizeof(struct can_frame));

```
当 CAN 接口绑定到任何（'any'）已存在的 CAN 接口（addr.can_ifindex = 0）时，如果需要在意源 CAN 接口的信息，建议使用 recvfrom(2)：


    struct sockaddr_can addr;
    struct ifreq ifr;
    socklen_t len = sizeof(addr);
    struct can_frame frame;

    nbytes = recvfrom(s, &frame, sizeof(struct can_frame),
                      0, (struct sockaddr*)&addr, &len);

    /** get interface name of the received CAN frame **/
    ifr.ifr_ifindex = addr.can_ifindex;
    ioctl(s, SIOCGIFNAME, &ifr);
    printf("Received a CAN frame from interface %s", ifr.ifr_name);

要在绑定到 'any' CAN 接口的套接字上写入 CAN 帧，必须明确指定出口接口：


    strcpy(ifr.ifr_name, "can0");
    ioctl(s, SIOCGIFINDEX, &ifr);
    addr.can_ifindex = ifr.ifr_ifindex;
    addr.can_family  = AF_CAN;

    nbytes = sendto(s, &frame, sizeof(struct can_frame),
                    0, (struct sockaddr*)&addr, sizeof(addr));

在从套接字读取消息之后，可以通过 ioctl(2) 调用获取精确的时间戳：


    struct timeval tv;
    ioctl(s, SIOCGSTAMP, &tv);

该时间戳的分辨率为一微秒，并在接收到 CAN 帧时自动设置。

关于 CAN FD（灵活数据速率）支持的备注：

通常，CAN FD 的处理与前面描述的示例非常相似。新的支持 CAN FD 的 CAN 控制器为 CAN FD 帧的仲裁阶段和载荷阶段支持两种不同的比特率，以及最多 64 字节的载荷。这种扩展的载荷长度破坏了所有严重依赖固定八字节载荷的 CAN 帧（struct can_frame，如 CAN_RAW 套接字）的内核接口（ABI）。因此，例如 CAN_RAW 套接字支持一个新的套接字选项 CAN_RAW_FD_FRAMES，它将套接字切换到一个允许同时处理 CAN FD 帧和经典 CAN 帧的模式（见 socketcan-rawfd）。

struct canfd_frame 定义在 include/linux/can.h 中：


    struct canfd_frame {
            canid_t can_id;  /** 32 bit CAN_ID + EFF/RTR/ERR flags **/
            __u8    len;     /** frame payload length in byte (0 .. 64) **/
            __u8    flags;   /** additional flags for CAN FD **/
            __u8    __res0;  /** reserved / padding **/
            __u8    __res1;  /** reserved / padding **/
            __u8    data[^64^] __attribute__((aligned(8)));
    };

struct canfd_frame 和已有的 struct can_frame 在结构体内的相同偏移处拥有 can_id、载荷长度和载荷数据。这使得可以以非常相似的方式处理不同的结构。当 struct can_frame 的内容被复制到 struct canfd_frame 中时，所有结构元素都可以原样使用——只是 data[] 变长了。

在引入 struct canfd_frame 时发现，struct can_frame 的数据长度代码（DLC）被用作长度信息，因为在 0 到 8 的范围内长度与 DLC 是 1:1 映射的。为了保持长度信息处理的简便性，canfd_frame.len 元素包含一个从 0 到 64 的普通长度值。因此，canfd_frame.len 和 can_frame.len 都相等，并包含长度信息而非 DLC。关于 CAN 与支持 CAN FD 的设备的区别以及到与总线相关的数据长度代码（DLC）的映射，详见 socketcan-can-fd-driver。

这两种 CAN(FD) 帧结构的长度定义了 CAN(FD) 网络接口和 skbuff 数据长度的最大传输单元（MTU）。在 include/linux/can.h 中为 CAN 特定的 MTU 规定了两个定义：


  #define CAN_MTU   (sizeof(struct can_frame))   == 16  => 经典 CAN 帧
  #define CANFD_MTU (sizeof(struct canfd_frame)) == 72  => CAN FD 帧


### 返回的报文标志


在 RAW 或 BCM 套接字上使用系统调用 recvmsg(2) 时，msg->msg_flags 字段可能包含以下标志：

MSG_DONTROUTE：
	当接收到的帧是在本地主机上创建时设置。

MSG_CONFIRM：
	当帧通过接收到它的那个套接字发送时设置。当 CAN 驱动支持驱动层面的帧回显时，此标志可被解释为“发送确认”，见 socketcan-local-loopback1 和 socketcan-local-loopback2。（注意：为了在 RAW 套接字上接收此类消息，必须设置 CAN_RAW_RECV_OWN_MSGS。）


### 带 can_filters 的 RAW 协议套接字（SOCK_RAW）


使用 CAN_RAW 套接字在很大程度上可与众所周知的、对 CAN 字符设备的访问相媲美。为了满足多用户 SocketCAN 方案提供的新可能，一些合理的默认值在 RAW 套接字绑定时被设置：

- 过滤器被设置为恰好一个接收所有内容的过滤器
- 套接字只接收有效的数据帧（=> 无错误消息帧）
- 已发送 CAN 帧的回环被启用（见 socketcan-local-loopback2）
- 套接字不接收自身已发送的帧（在回环模式下）

这些默认设置可以在绑定套接字之前或之后更改。要使用 CAN_RAW 套接字相关的套接字选项的定义，请包含 <linux/can/raw.h>。


#### RAW 套接字选项 CAN_RAW_FILTER


使用 CAN_RAW 套接字接收 CAN 帧可以通过 CAN_RAW_FILTER 套接字选项定义 0 到 n 个过滤器来控制。

CAN 过滤结构定义在 include/linux/can.h 中：


    struct can_filter {
            canid_t can_id;
            canid_t can_mask;
    };

一个过滤器在以下情况下匹配：


    <received_can_id> & mask == can_id & mask

这类似于已知的 CAN 控制器硬件过滤语义。当 can_filter 结构的 can_id 元素中设置了 CAN_INV_FILTER 位时，该过滤器可以在此语义下被反转。与 CAN 控制器硬件过滤器相比，用户可以为每个打开的套接字分别设置 0 到 n 个接收过滤器：


    struct can_filter rfilter[^2^];

    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = CAN_SFF_MASK;
    rfilter[^1^].can_id   = 0x200;
    rfilter[^1^].can_mask = 0x700;

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));

要在所选的 CAN_RAW 套接字上禁用 CAN 帧的接收：


    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, NULL, 0);

将过滤器设置为 0 个过滤器已经相当过时了，因为不读取数据会导致 raw 套接字丢弃接收到的 CAN 帧。但有了这个“只发送”的用例，我们可以在内核中移除接收列表，以节省一点点（真的非常少！）CPU 使用率。

CAN 过滤器使用优化
.............................

CAN 过滤器在 CAN 帧接收时于每设备的过滤器列表中处理。为了减少遍历过滤器列表时需要执行的检查次数，当过滤订阅集中于单个 CAN ID 时，CAN 核心提供优化的过滤处理。

对于可能的 2048 个 SFF CAN 标识符，标识符被用作索引来访问相应的订阅列表，而无需任何进一步检查。对于 2^29 个可能的 EFF CAN 标识符，使用 10 位 XOR 折叠作为哈希函数来检索 EFF 表索引。

为了从针对单个 CAN 标识符的优化过滤器中获益，必须将 CAN_SFF_MASK 或 CAN_EFF_MASK 与所设置的 CAN_EFF_FLAG 和 CAN_RTR_FLAG 位一起设置到 can_filter.mask 中。can_filter.mask 中设置了的 CAN_EFF_FLAG 位清楚地表明，订阅的是 SFF 还是 EFF CAN ID 是有区别的。例如，在上面的示例中：


    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = CAN_SFF_MASK;

SFF 帧（CAN ID 0x123）和 EFF 帧（0xXXXXX123）都可以通过。

要仅过滤 0x123（SFF）和 0x12345678（EFF）CAN 标识符，必须以下列方式定义过滤器才能从优化的过滤器中受益：


    struct can_filter rfilter[^2^];

    rfilter[^0^].can_id   = 0x123;
    rfilter[^0^].can_mask = (CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_SFF_MASK);
    rfilter[^1^].can_id   = 0x12345678 | CAN_EFF_FLAG;
    rfilter[^1^].can_mask = (CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_EFF_MASK);

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_FILTER, &rfilter, sizeof(rfilter));


#### RAW 套接字选项 CAN_RAW_ERR_FILTER


如 socketcan-network-problem-notifications 所述，CAN 接口驱动可以生成所谓的错误消息帧，它可以与其他 CAN 帧一样可选地传递给用户应用程序。可能的错误被划分为不同的错误类，可以使用适当的错误掩码进行过滤。要注册每一种可能的错误条件，可以使用 CAN_ERR_MASK 作为错误掩码的值。错误掩码的值定义在 linux/can/error.h 中：


    can_err_mask_t err_mask = ( CAN_ERR_TX_TIMEOUT | CAN_ERR_BUSOFF );

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_ERR_FILTER,
               &err_mask, sizeof(err_mask));


#### RAW 套接字选项 CAN_RAW_LOOPBACK


为了满足多用户需求，本地回环默认是启用的（详见 socketcan-local-loopback1）。但在某些嵌入式用例中（例如当只有一个应用程序使用 CAN 总线时），这个回环功能可以被禁用（针对每个套接字分别设置）：


    int loopback = 0; /** 0 = 禁用, 1 = 启用 (默认) **/

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_LOOPBACK, &loopback, sizeof(loopback));


#### RAW 套接字选项 CAN_RAW_RECV_OWN_MSGS


当本地回环启用时，所有已发送的 CAN 帧都会被回环到那些在该给定接口上为这些 CAN 帧的 CAN-ID 注册了的已打开 CAN 套接字，以满足多用户需求。在同一个发送了 CAN 帧的套接字上接收该 CAN 帧被认为是不需要的，因此默认禁用。这个默认行为可以按需求更改：


    int recv_own_msgs = 1; /** 0 = 禁用 (默认), 1 = 启用 **/

    setsockopt(s, SOL_CAN_RAW, CAN_RAW_RECV_OWN_MSGS,
               &recv_own_msgs, sizeof(recv_own_msgs));

请注意，套接字自身 CAN 帧的接收与其他 CAN 帧一样受到相同的过滤（见 socketcan-rawfilter）。


#### RAW 套接字选项 CAN_RAW_FD_FRAMES


CAN_RAW 套接字中的 CAN FD 支持可以通过一个新的套接字选项 CAN_RAW_FD_FRAMES 启用，该选项默认关闭。当 CAN_RAW 套接字不支持该新套接字选项时（例如在较旧的内核上），切换 CAN_RAW_FD_FRAMES 选项会返回错误 -ENOPROTOOPT。

一旦启用了 CAN_RAW_FD_FRAMES，应用程序就可以发送 CAN 帧和 CAN FD 帧。另一方面，应用程序在从套接字读取时必须处理 CAN 和 CAN FD 帧：


    CAN_RAW_FD_FRAMES enabled:  CAN_MTU 和 CANFD_MTU 都允许
    CAN_RAW_FD_FRAMES disabled: 只允许 CAN_MTU (默认)

示例：


    [ 记得: CANFD_MTU == sizeof(struct canfd_frame) ]

    struct canfd_frame cfd;

    nbytes = read(s, &cfd, CANFD_MTU);

    if (nbytes == CANFD_MTU) {
            printf("got CAN FD frame with length %d\n", cfd.len);
            /** cfd.flags 包含有效数据 **/
    } else if (nbytes == CAN_MTU) {
            printf("got Classical CAN frame with length %d\n", cfd.len);
            /** cfd.flags 未定义 **/
    } else {
            fprintf(stderr, "read: invalid CAN(FD) frame\n");
            return 1;
    }

    /** 内容可以独立于接收到的 MTU 大小来处理 **/

    printf("can_id: %X data length: %d data: ", cfd.can_id, cfd.len);
    for (i = 0; i < cfd.len; i++)
            printf("%02X ", cfd.data[i]);

当以 CANFD_MTU 大小读取只返回从套接字接收到的 CAN_MTU 字节时，一个经典 CAN 帧已被读入所提供的 CAN FD 结构中。请注意，canfd_frame.flags 数据字段在 struct can_frame 中并未规定，因此它只在 CANFD_MTU 大小的 CAN FD 帧中有效。

新 CAN 应用程序的实现提示：

要构建感知 CAN FD 的应用程序，请使用 struct canfd_frame 作为基于 CAN_RAW 的应用程序的基本 CAN 数据结构。当应用程序在较旧的 Linux 内核上执行，并且切换 CAN_RAW_FD_FRAMES 套接字选项返回错误时：没有问题。你会得到经典 CAN 帧或 CAN FD 帧，并且可以用相同的方式处理它们。

在向 CAN 设备发送时，请确保该设备能够通过检查设备最大传输单元是否为 CANFD_MTU 来处理 CAN FD 帧。CAN 设备 MTU 可以通过例如 SIOCGIFMTU ioctl() 系统调用获取。


#### RAW 套接字选项 CAN_RAW_JOIN_FILTERS


CAN_RAW 套接字可以设置多个特定于 CAN 标识符的过滤器，这些过滤器在 af_can.c 的过滤处理中导致多个过滤器。这些过滤器彼此独立，在应用时导致逻辑“或”（OR）的过滤器（见 socketcan-rawfilter）。

这个套接字选项以这样的方式连接给定的 CAN 过滤器：只有匹配**所有**给定 CAN 过滤器的 CAN 帧才会被传递到用户空间。因此，所应用过滤器的语义被改变为逻辑“与”（AND）。

这在过滤器集合是过滤器组合、且其中设置了 CAN_INV_FILTER 标志以便从传入流量中滤除单个 CAN ID 或 CAN ID 范围时特别有用。


### 广播管理器协议套接字（SOCK_DGRAM）


广播管理器协议提供了一个基于命令的配置接口，用于在内核空间中过滤和发送（例如周期性）CAN 消息。

接收过滤器可用于对频繁的消息进行降采样；检测诸如消息内容改变、包长度改变等事件，并对接收到的消息进行超时监控。

可以创建 CAN 帧或 CAN 帧序列的周期性发送任务，并在运行时修改；消息内容和两种可能的发送间隔都可以更改。

BCM 套接字不打算用于使用已知来自 CAN_RAW 套接字的 struct can_frame 发送单个 CAN 帧。相反，定义了一个特殊的 BCM 配置消息。用于与广播管理器通信的基本 BCM 配置消息以及可用的操作定义在 linux/can/bcm.h 头文件中。BCM 消息由一个带命令（'opcode'）的消息头以及零个或多个 CAN 帧组成。广播管理器以相同的形式向用户空间发送响应：


    struct bcm_msg_head {
            __u32 opcode;                   /** command **/
            __u32 flags;                    /** special flags **/
            __u32 count;                    /** run 'count' times with ival1 **/
            struct timeval ival1, ival2;    /** count and subsequent interval **/
            canid_t can_id;                 /** unique can_id for task **/
            __u32 nframes;                  /** number of can_frames following **/
            struct can_frame frames[];
    };

对齐的载荷 'frames' 使用在 socketcan-rawfd 开头和 include/linux/can.h 头文件中定义的相同基本 CAN 帧结构。所有从用户空间发往广播管理器的消息都具有此结构。

请注意，CAN_BCM 套接字在创建后必须 connect 而非 bind（示例未做错误检查）：


    int s;
    struct sockaddr_can addr;
    struct ifreq ifr;

    s = socket(PF_CAN, SOCK_DGRAM, CAN_BCM);

    strcpy(ifr.ifr_name, "can0");
    ioctl(s, SIOCGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    connect(s, (struct sockaddr *)&addr, sizeof(addr));

    (..)

广播管理器套接字能够并发处理任意数量的在途发送或接收过滤器。不同的 RX/TX 作业通过每个 BCM 消息中唯一的 can_id 来区分。但建议使用额外的 CAN_BCM 套接字在多个 CAN 接口上通信。当广播管理器套接字绑定到 'any' CAN 接口（=> 接口索引设置为零）时，所配置的接收过滤器适用于任何 CAN 接口，除非使用 sendto() 系统调用来覆盖 'any' CAN 接口索引。当使用 recvfrom() 而非 read() 来检索 BCM 套接字消息时，源 CAN 接口在 can_ifindex 中提供。


#### 广播管理器操作


opcode 定义了广播管理器要执行的操作，或者详述了广播管理器对若干事件的响应，包括用户请求。

发送操作（用户空间到广播管理器）：

TX_SETUP：
	创建（周期性）发送任务。

TX_DELETE：
	移除（周期性）发送任务，只需要 can_id。

TX_READ：
	读取（周期性）发送任务的属性，针对 can_id。

TX_SEND：
	发送一个 CAN 帧。

发送响应（广播管理器到用户空间）：

TX_STATUS：
	对 TX_READ 请求的回复（发送任务配置）。

TX_EXPIRED：
	当计数器以初始间隔 'ival1' 完成发送时的通知。
	需要在 TX_SETUP 时设置 TX_COUNTEVT 标志。

接收操作（用户空间到广播管理器）：

RX_SETUP：
	创建 RX 内容过滤器订阅。

RX_DELETE：
	移除 RX 内容过滤器订阅，只需要 can_id。

RX_READ：
	读取针对 can_id 的 RX 内容过滤器订阅的属性。

接收响应（广播管理器到用户空间）：

RX_STATUS：
	对 RX_READ 请求的回复（过滤器任务配置）。

RX_TIMEOUT：
	检测到周期消息缺失（定时器 ival1 过期）。

RX_CHANGED：
	带有更新 CAN 帧的 BCM 消息（检测到内容改变）。
	在收到第一条消息或收到修订的 CAN 消息时发送。


#### 广播管理器消息标志


当向广播管理器发送消息时，'flags' 元素可以包含以下影响行为的标志定义：

SETTIMER：
	设置 ival1、ival2 和 count 的值。

STARTTIMER：
	以 ival1、ival2 和 count 的实际值启动定时器。
	启动定时器同时会导致发出一个 CAN 帧。

TX_COUNTEVT：
	当 count 过期时创建 TX_EXPIRED 消息。

TX_ANNOUNCE：
	进程对数据的改变会立即发出。

TX_CP_CAN_ID：
	将 can_id 从消息头复制到 frames 中的每个后续帧。这旨在简化使用。对于 TX 任务，消息头中唯一的 can_id 可能与为后续 struct can_frame(s) 中传输而存储的 can_id(s) 不同。

RX_FILTER_ID：
	仅按 can_id 过滤，不需要帧（nframes=0）。

RX_CHECK_DLC：
	DLC 的改变会导致 RX_CHANGED。

RX_NO_AUTOTIMER：
	阻止自动启动超时监控。

RX_ANNOUNCE_RESUME：
	如果在 RX_SETUP 时传入，且发生了接收超时，则（周期性）接收重启时会生成一条 RX_CHANGED 消息。

TX_RESET_MULTI_IDX：
	重置多帧传输的索引。

RX_RTR_FRAME：
	发送对 RTR 请求的回复（放在 op->frames[^0^] 中）。

CAN_FD_FRAME：
	bcm_msg_head 后面的 CAN 帧是 struct canfd_frame。

#### 广播管理器发送定时器


周期性发送配置最多可以使用两个间隔定时器。在这种情况下，BCM 以一个间隔 'ival1' 发送若干消息（'count'），然后以另一个给定间隔 'ival2' 继续发送。当只需要一个定时器时，'count' 设置为零，并且只使用 'ival2'。当设置了 SET_TIMER 和 START_TIMER 标志时，定时器被激活。当只设置了 SET_TIMER 时，定时器值可以在运行时更改。


#### 广播管理器消息序列发送


在周期性 TX 任务配置的情况下，最多 256 个 CAN 帧可以按序列发送。CAN 帧的数量在 BCM 消息头的 'nframes' 元素中提供。所定义的 CAN 帧数量作为数组添加到 TX_SETUP BCM 配置消息中：


    /** 创建一个用于设置四个 CAN 帧序列的结构 **/
    struct {
            struct bcm_msg_head msg_head;
            struct can_frame frame[^4^];
    } mytxmsg;

    (..)
    mytxmsg.msg_head.nframes = 4;
    (..)

    write(s, &mytxmsg, sizeof(mytxmsg));

每次发送时，CAN 帧数组中的索引会递增，并在索引溢出时重置为零。


#### 广播管理器接收过滤器定时器


定时器值 ival1 或 ival2 可以在 RX_SETUP 时设置为非零值。当设置了 SET_TIMER 标志时，定时器被启用：

ival1：
	当接收到的消息在给定时间内未再次收到时，发送 RX_TIMEOUT。如果在 RX_SETUP 时设置了 START_TIMER，则超时检测会直接激活——即使没有先前的 CAN 帧接收。

ival2：
	将接收到的消息速率限制到 ival2 的值。当 CAN 帧内的信号是无状态的、且 ival2 周期内的状态改变可能会丢失时，这对于减少应用程序的消息量很有用。

#### 广播管理器多路复用消息接收过滤器


为了过滤多路复用消息序列中的内容改变，可以在 RX_SETUP 配置消息中传入多于一个 CAN 帧的数组。第一个 CAN 帧的数据字节包含相关位的掩码，这些位必须在后续 CAN 帧中与接收到的 CAN 帧匹配。如果某个后续 CAN 帧匹配，则该帧数据中的位标记要与先前接收的内容进行比较的相关内容。最多 257 个 CAN 帧（多路复用过滤器位掩码 CAN 帧加 256 个 CAN 过滤器）可以作为数组添加到 TX_SETUP BCM 配置消息中：


    /** 通常用于清空 CAN 帧 data[] - 注意大小端问题! **/
    #define U64_DATA(p) (**(unsigned long long**)(p)->data)

    struct {
            struct bcm_msg_head msg_head;
            struct can_frame frame[^5^];
    } msg;

    msg.msg_head.opcode  = RX_SETUP;
    msg.msg_head.can_id  = 0x42;
    msg.msg_head.flags   = 0;
    msg.msg_head.nframes = 5;
    U64_DATA(&msg.frame[^0^]) = 0xFF00000000000000ULL; /** MUX mask **/
    U64_DATA(&msg.frame[^1^]) = 0x01000000000000FFULL; /** data mask (MUX 0x01) **/
    U64_DATA(&msg.frame[^2^]) = 0x0200FFFF000000FFULL; /** data mask (MUX 0x02) **/
    U64_DATA(&msg.frame[^3^]) = 0x330000FFFFFF0003ULL; /** data mask (MUX 0x33) **/
    U64_DATA(&msg.frame[^4^]) = 0x4F07FC0FF0000000ULL; /** data mask (MUX 0x4F) **/

    write(s, &msg, sizeof(msg));


#### 广播管理器 CAN FD 支持


CAN_BCM 的编程 API 依赖于 struct can_frame，它作为数组直接放在 bcm_msg_head 结构之后。为了对 CAN FD 帧遵循此模式，bcm_msg_head 标志中的一个新标志 'CAN_FD_FRAME' 指示 bcm_msg_head 后面连接的 CAN 帧结构被定义为 struct canfd_frame：


    struct {
            struct bcm_msg_head msg_head;
            struct canfd_frame frame[^5^];
    } msg;

    msg.msg_head.opcode  = RX_SETUP;
    msg.msg_head.can_id  = 0x42;
    msg.msg_head.flags   = CAN_FD_FRAME;
    msg.msg_head.nframes = 5;
    (..)

当使用 CAN FD 帧进行多路复用过滤时，MUX 掩码仍然期望在 struct canfd_frame 数据段的前 64 位中。


### 面向连接的传输协议（SOCK_SEQPACKET）


（待写）


### 无连接的传输协议（SOCK_DGRAM）


（待写）


## SocketCAN 核心模块


SocketCAN 核心模块实现了协议族 PF_CAN。CAN 协议模块在运行时由核心模块加载。核心模块为 CAN 协议模块提供了一个接口来订阅所需的 CAN ID（见 socketcan-receive-lists）。


### can.ko 模块参数


- **stats_timer**：
  为了计算 SocketCAN 核心统计信息（例如当前/最大每秒帧数），这个 1 秒定时器默认在 can.ko 模块启动时启动。可以通过在模块命令行上使用 stattimer=0 来禁用此定时器。

- **debug**：
  （自 SocketCAN SVN r546 起已移除）


### procfs 内容


如 socketcan-receive-lists 所述，SocketCAN 核心使用多个过滤器列表将接收到的 CAN 帧投递给 CAN 协议模块。这些接收列表、它们的过滤器以及过滤器匹配次数可以在相应的接收列表中查看。所有条目都包含
```

    foo@bar:~$ cat /proc/net/can/rcvlist_all

    receive list 'rx_all':
      (vcan3: no entry)
      (vcan2: no entry)
      (vcan1: no entry)
      device   can_id   can_mask  function  userdata   matches  ident
       vcan0     000    00000000  f88e6370  f6c6f400         0  raw
      (any: no entry)

```
```

    rcvlist_all - 未过滤条目的列表 (无过滤操作)
    rcvlist_eff - 单个扩展帧 (EFF) 条目的列表
    rcvlist_err - 错误消息帧掩码的列表
    rcvlist_fil - 掩码/值过滤器的列表
    rcvlist_inv - 掩码/值过滤器的列表 (逆语义)
    rcvlist_sff - 单个标准帧 (SFF) 条目的列表

```
```

    stats       - SocketCAN 核心统计信息 (rx/tx 帧, 匹配比率, ...)
    reset_stats - 手动统计重置
    version     - 打印 SocketCAN 核心和 ABI 版本 (在 Linux 5.10 中移除)

```
### 编写自己的 CAN 协议模块


要在协议族 PF_CAN 中实现新协议，必须在 include/linux/can.h 中定义新协议。用于使用 SocketCAN 核心的原型与定义可以通过包含 include/linux/can/core.h 来访问。除了注册 CAN 协议和 CAN 设备通知链的函数外，还有订阅 CAN 的函数
```

    can_rx_register   - 订阅来自特定接口的 CAN 帧
    can_rx_unregister - 退订来自特定接口的 CAN 帧
    can_send          - 发送一个 CAN 帧 (可选择带本地回环)

```
有关详情，请参见 net/can/af_can.c 中的 kerneldoc 文档，或 net/can/raw.c 与 net/can/bcm.c 的源代码。


## CAN 网络驱动


编写 CAN 网络设备驱动比编写 CAN 字符设备驱动要容易得多。与其他已知的网络设备驱动类似，你主要需要处理：

- TX：将 CAN 帧从套接字缓冲区送入 CAN 控制器。
- RX：将 CAN 帧从 CAN 控制器送入套接字缓冲区。

参见例如 Documentation/networking/netdevices.rst 。编写 CAN 网络设备驱动的不同之处描述如下：


### 通用设置


CAN 网络设备驱动可以使用 alloc_candev_mqs() 及其相关函数，而非 alloc_netdev_mqs()，以自动处理 CAN 特有的初始化工作：


    dev = alloc_candev_mqs(...);

struct can_frame 或 struct canfd_frame 是 PF_CAN 协议族中每个套接字缓冲区（skbuff）的载荷。


### 已发送帧的本地回环


如 socketcan-local-loopback1 所述，CAN 网络设备驱动应支持一种类似于本地回显（例如 tty 设备那样）的本地回环功能。在这种情况下，必须设置驱动标志 IFF_ECHO，以防止 PF_CAN 核心对已发送的帧进行本地回显。
```

    dev->flags = (IFF_NOARP | IFF_ECHO);


```
### CAN 控制器硬件过滤器


为了减少深度嵌入式系统上的中断负载，一些 CAN 控制器支持对 CAN ID 或 CAN ID 范围进行过滤。这些硬件过滤能力因控制器而异，并且必须认定为在多用户网络方案中是不可行的。使用高度控制器相关的硬件过滤器可能只在非常专用的用例中有意义，因为驱动层面的过滤器会影响多用户系统中的所有用户。PF_CAN 核心内部的高效过滤器集合允许为每个套接字分别设置多个不同的过滤器。因此，硬件过滤器的使用归入“深度嵌入式系统上的手工调优”这一类别。作者在一个 @133MHz 的 MPC603e 上，使用四个 SJA1000 CAN 控制器，自 2002 年起在重总线负载下运行而没有任何问题……


### 可切换的终端电阻


CAN 总线需要在差分对两端提供特定的阻抗，通常由总线最远端的节点上的两个 120Ohm 电阻提供。一些 CAN 控制器支持激活/停用终端电阻以提供正确的阻抗。

```

    $ ip -details link show can0
    ...
    termination 120 [ 0, 120 ]


```
```

    $ ip link set dev can0 type can termination 120


```
```

    $ ip link set dev can0 type can termination 0


```
要为 can 控制器启用终端电阻支持，要么
```

    termination_const
    termination_const_cnt
    do_set_termination


```
要么通过以下设备树条目添加 gpio 控制：
Documentation/devicetree/bindings/net/can/can-controller.yaml


### 虚拟 CAN 驱动（vcan）


与网络回环设备类似，vcan 提供了一个虚拟的本地 CAN 接口。CAN 上的一个完整限定地址由以下部分组成：

- 一个唯一的 CAN 标识符（CAN ID）
- 该 CAN ID 所传输到的 CAN 总线（例如 can0）

因此，在常见用例中需要不止一个虚拟 CAN 接口。

虚拟 CAN 接口允许在没有真实 CAN 控制器硬件的情况下收发 CAN 帧。虚拟 CAN 网络设备通常命名为 'vcanX'，例如 vcan0、vcan1、vcan2……当编译为模块时，虚拟 CAN 驱动模块名为 vcan.ko。

自 Linux 内核版本 2.6.24 起，vcan 驱动支持内核 netlink 接口来创建 vcan 网络设备。创建以及
```

  - 创建一个虚拟 CAN 网络接口：
       $ ip link add type vcan

  - 创建一个指定名称 'vcan42' 的虚拟 CAN 网络接口：
       $ ip link add dev vcan42 type vcan

  - 移除一个（虚拟 CAN）网络接口 'vcan42'：
       $ ip link del vcan42


```
### CAN 网络设备驱动接口


CAN 网络设备驱动接口提供了一个用于设置、配置和监控 CAN 网络设备的通用接口。然后用户可以配置 CAN 设备，例如通过 netlink 接口使用 "IPROUTE2" 工具套件中的 "ip" 程序来设置位时序参数。下面一章简要描述了如何使用它。此外，该接口使用一个通用的数据结构并导出一组公共函数，所有真实的 CAN 网络设备驱动都应当使用它们。请参考 SJA1000 或 MSCAN 驱动以了解如何使用它们。该模块名为 can-dev.ko。


#### 用于设置/获取设备属性的 Netlink 接口


CAN 设备必须通过 netlink 接口进行配置。支持的 netlink 消息类型在 "include/linux/can/netlink.h" 中定义并作了简要说明。IPROUTE2 工具套件中 "ip" 程序的 CAN 链路支持已经可用，其用法如下所示：

```

    $ ip link set can0 type can help
    Usage: ip link set DEVICE type can
        [ bitrate BITRATE [ sample-point SAMPLE-POINT] ] |
        [ tq TQ prop-seg PROP_SEG phase-seg1 PHASE-SEG1
          phase-seg2 PHASE-SEG2 [ sjw SJW ] ]

        [ dbitrate BITRATE [ dsample-point SAMPLE-POINT] ] |
        [ dtq TQ dprop-seg PROP_SEG dphase-seg1 PHASE-SEG1
          dphase-seg2 PHASE-SEG2 [ dsjw SJW ] ]

        [ loopback { on | off } ]
        [ listen-only { on | off } ]
        [ triple-sampling { on | off } ]
        [ one-shot { on | off } ]
        [ berr-reporting { on | off } ]
        [ fd { on | off } ]
        [ fd-non-iso { on | off } ]
        [ presume-ack { on | off } ]
        [ cc-len8-dlc { on | off } ]

        [ restart-ms TIME-MS ]
        [ restart ]

        Where: BITRATE       := { 1..1000000 }
               SAMPLE-POINT  := { 0.000..0.999 }
               TQ            := { NUMBER }
               PROP-SEG      := { 1..8 }
               PHASE-SEG1    := { 1..8 }
               PHASE-SEG2    := { 1..8 }
               SJW           := { 1..4 }
               RESTART-MS    := { 0 | NUMBER }


```
```

    $ ip -details -statistics link show can0
    2: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 16 qdisc pfifo_fast state UP qlen 10
      link/can
      can <TRIPLE-SAMPLING> state ERROR-ACTIVE restart-ms 100
      bitrate 125000 sample_point 0.875
      tq 125 prop-seg 6 phase-seg1 7 phase-seg2 2 sjw 1
      sja1000: tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1
      clock 8000000
      re-started bus-errors arbit-lost error-warn error-pass bus-off
      41         17457      0          41         42         41
      RX: bytes  packets  errors  dropped overrun mcast
      140859     17608    17457   0       0       0
      TX: bytes  packets  errors  dropped carrier collsns
      861        112      0       41      0       0


```
上述输出的更多信息：

"<TRIPLE-SAMPLING>"
	显示所选 CAN 控制器模式的列表：LOOPBACK、LISTEN-ONLY 或 TRIPLE-SAMPLING。

"state ERROR-ACTIVE"
	CAN 控制器的当前状态："ERROR-ACTIVE"、"ERROR-WARNING"、"ERROR-PASSIVE"、"BUS-OFF" 或 "STOPPED"。

"restart-ms 100"
	自动重启延迟时间。如果设置为非零值，则在发生总线关闭（bus-off）情况时，会在指定的延迟时间（毫秒）之后自动触发 CAN 控制器的重启。默认是关闭的。

"bitrate 125000 sample-point 0.875"
	显示真实的比特率（单位 bit/sec）和采样点（范围 0.000..0.999）。如果内核启用了位时序参数的计算（CONFIG_CAN_CALC_BITTIMING=y），则可以通过设置 "bitrate" 参数来定义位时序。可选地也可以指定 "sample-point"。默认是 0.000，假定采用 CIA 推荐的采样点。

"tq 125 prop-seg 6 phase-seg1 7 phase-seg2 2 sjw 1"
	显示时间量子（tq，单位 ns）、传播段、相位缓冲段 1 和 2，以及同步跳转宽度（单位也是 tq）。它们允许以与硬件无关的格式定义 CAN 位时序，正如 Bosch CAN 2.0 规范所建议的那样（见 http://www.semiconductors.bosch.de/pdf/can2spec.pdf 的第 8 章）。

"sja1000: tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1 clock 8000000"
	显示 CAN 控制器（此处为 "sja1000"）的位时序常量。包括时间段时间 1 和 2 的最小值和最大值、同步跳转宽度（单位 tq）、位率预分频器，以及 CAN 系统时钟频率（单位 Hz）。这些常量可用于用户空间中用户自定义的（非标准）位时序计算算法。

"re-started bus-errors arbit-lost error-warn error-pass bus-off"
	显示重启次数、总线错误与仲裁丢失错误次数，以及到错误警告、错误被动和总线关闭状态的转换次数。RX 溢出错误列在标准网络统计的 "overrun" 字段中。

#### 设置 CAN 位时序


CAN 位时序参数始终可以以与硬件无关的格式定义，正如 Bosch CAN 2.0 规范所建议的，通过指定 "tq"、"prop_seg"、"phase_seg1"、"phase_seg2" 参数：
```

    $ ip link set canX type can tq 125 prop-seg 6 \
				phase-seg1 7 phase-seg2 2 sjw 1

```
如果启用了内核选项 CONFIG_CAN_CALC_BITTIMING，则在设置比特率时会计算 CIA 推荐的 CAN 位时序参数：
```

    $ ip link set canX type can bitrate 125000

```
请注意，这对大多数带有标准比特率的常见 CAN 控制器都能正常工作，但对于特殊的比特率或 CAN 系统时钟频率可能会**失败**。禁用 CONFIG_CAN_CALC_BITTIMING 可以节省一些空间，并允许用户空间工具独立地确定和设置位时序参数。CAN 控制器特有的位时序常量可用于此目的。它们由以下命令列出：
```

    $ ip -details link show can0
    ...
      sja1000: clock 8000000 tseg1 1..16 tseg2 1..8 sjw 1..4 brp 1..64 brp-inc 1


```
#### 启动和停止 CAN 网络设备


CAN 网络设备的启动或停止与通常一样，使用命令 "ifconfig canX up/down" 或 "ip link set canX up/down"。请注意，对于真实的 CAN 设备，你**必须**定义正确的位时序参数：
```

    $ ip link set canX up type can bitrate 125000

```
如果 CAN 总线上发生了过多错误，设备可能会进入 "bus-off" 状态。此时不再收发任何消息。可以通过设置 "restart-ms" 来启用自动的总线关闭恢复：
```

    $ ip link set canX type can restart-ms 100

```
另外，应用程序也可以通过监控 CAN 错误消息帧来意识到 "bus-off" 状态，并在发生该状态时进行重启：
```

    $ ip link set canX type can restart

```
请注意，一次重启也会创建一个 CAN 错误消息帧（另见 socketcan-network-problem-notifications）。



### CAN FD（灵活数据速率）驱动支持


支持 CAN FD 的 CAN 控制器为 CAN FD 帧的仲裁阶段和载荷阶段支持两种不同的比特率。因此，必须指定第二个位时序才能启用 CAN FD 比特率。

此外，支持 CAN FD 的 CAN 控制器支持最多 64 字节的载荷。这种长度在 can_frame.len 和 canfd_frame.len 中，对于用户空间应用程序以及 Linux 网络层内部，是一个从 0 到 64 的普通数值，而不是范围从 0 到 8 的经典 CAN 长度。载荷长度到与总线相关的 DLC 的映射只在 CAN 驱动内部执行，最好使用辅助函数 can_fd_dlc2len() 和 can_fd_len2dlc()。

CAN 网络设备驱动的能力可以通过网络来区分：
```

  MTU = 16 (CAN_MTU)   => sizeof(struct can_frame)   => 经典 CAN 设备
  MTU = 72 (CANFD_MTU) => sizeof(struct canfd_frame) => CAN FD 能力设备

```
CAN 设备 MTU 可以通过例如 SIOCGIFMTU ioctl() 系统调用获取。注意：支持 CAN FD 的设备也可以处理和发送经典 CAN 帧。

在配置支持 CAN FD 的 CAN 控制器时，必须设置一个额外的 'data'（数据）比特率。CAN FD 帧数据阶段的这个比特率必须至少等于为仲裁阶段配置的比特率。这第二个比特率的指定方式与第一个类似，但 'data' 比特率的设置关键字以 'd' 开头，例如 dbitrate、dsample-point、dsjw 或 dtq 以及类似的设置。在配置过程中设置了数据比特率时，可以指定控制器选项 "fd on" 来在 CAN 控制器中启用 CAN FD 模式。该控制器选项同时会将设备 MTU 切换为 72（CANFD_MTU）。

当前的 CAN FD 规范在 2012 年国际 CAN 大会上以白皮书形式首次提出，出于数据完整性原因需要改进。因此，如今必须区分两种 CAN FD 实现：

- ISO 兼容：     ISO 11898-1:2015 的 CAN FD 实现（默认）
- 非 ISO 兼容： 遵循 2012 年白皮书的 CAN FD 实现

最终有三类 CAN FD 控制器：

1. ISO 兼容（固定）
2. 非 ISO 兼容（固定，例如 m_can.c 中的 M_CAN IP 核 v3.0.1）
3. ISO/非 ISO CAN FD 控制器（可切换，例如 PEAK PCAN-USB FD）

当前的 ISO/非 ISO 模式由 CAN 控制器驱动通过 netlink 公布，并由 'ip' 工具显示（控制器选项 FD-NON-ISO）。ISO/非 ISO 模式只能通过为可切换的 CAN FD 控制器设置 'fd-non-iso {on|off}' 来改变。

```

    $ ip link set can0 up type can bitrate 500000 sample-point 0.75 \
                                   dbitrate 4000000 dsample-point 0.8 fd on
    $ ip -details link show can0
    5: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 72 qdisc pfifo_fast state UNKNOWN \
             mode DEFAULT group default qlen 10
    link/can  promiscuity 0
    can <FD> state ERROR-ACTIVE (berr-counter tx 0 rx 0) restart-ms 0
          bitrate 500000 sample-point 0.750
          tq 50 prop-seg 14 phase-seg1 15 phase-seg2 10 sjw 1
          pcan_usb_pro_fd: tseg1 1..64 tseg2 1..16 sjw 1..16 brp 1..1024 \
          brp-inc 1
          dbitrate 4000000 dsample-point 0.800
          dtq 12 dprop-seg 7 dphase-seg1 8 dphase-seg2 4 dsjw 1
          pcan_usb_pro_fd: dtseg1 1..16 dtseg2 1..8 dsjw 1..4 dbrp 1..1024 \
          dbrp-inc 1
          clock 80000000


```
```

   can <FD,FD-NON-ISO> state ERROR-ACTIVE (berr-counter tx 0 rx 0) restart-ms 0


```
#### 发送器延迟补偿


在高位速率下，从收发器 TX 引脚到 RX 引脚的传播延迟可能变得大于实际位时间，从而导致测量错误：RX 引脚仍会在测量上一个位。

发送器延迟补偿（此后称 TDC）通过引入一个次级采样点（SSP）来解决此问题，该采样点等于从 TX 引脚上位时间开始到 RX 引脚上实际测量之间的、以最小时间量子为单位的距离。SSP 计算为两个可配置值之和：TDC 值（TDCV）和 TDC 偏移（TDCO）。

如果设备支持，TDC 可以与 CAN-FD 一起使用 ip 工具的 "tdc-mode" 参数进行配置，如下所示：

**omitted**
	当不提供 "tdc-mode" 选项时，内核将自动决定是否应打开 TDC，在这种情况下它将计算一个默认的 TDCO 并使用设备测得的 TDCV。这是使用 TDC 的推荐方法。

**"tdc-mode off"**
	TDC 被显式禁用。

**"tdc-mode auto"**
	用户必须提供 "tdco" 参数。TDCV 将由设备自动计算。此选项仅在设备支持 TDC-AUTO CAN 控制器模式时才可用。

**"tdc-mode manual"**
	用户必须同时提供 "tdco" 和 "tdcv" 参数。此选项仅在设备支持 TDC-MANUAL CAN 控制器模式时才可用。

请注意，某些设备可能提供额外的参数："tdcf"（TDC 滤波窗口）。如果您的设备支持，可以将其作为可选参数添加到 "tdc-mode auto" 或 "tdc-mode manual" 中。

配置 500 kbit/s 仲裁比特率、5 Mbit/s 数据比特率、TDCO 为 15 个最小时间量子以及自动测量的 TDCV 的示例：
```

    $ ip link set can0 up type can bitrate 500000 \
                                   fd on dbitrate 4000000 \
				   tdc-mode auto tdco 15
    $ ip -details link show can0
    5: can0: <NOARP,UP,LOWER_UP,ECHO> mtu 72 qdisc pfifo_fast state UP \
             mode DEFAULT group default qlen 10
        link/can  promiscuity 0 allmulti 0 minmtu 72 maxmtu 72
        can <FD,TDC-AUTO> state ERROR-ACTIVE restart-ms 0
          bitrate 500000 sample-point 0.875
          tq 12 prop-seg 69 phase-seg1 70 phase-seg2 20 sjw 10 brp 1
          ES582.1/ES584.1: tseg1 2..256 tseg2 2..128 sjw 1..128 brp 1..512 \
          brp_inc 1
          dbitrate 4000000 dsample-point 0.750
          dtq 12 dprop-seg 7 dphase-seg1 7 dphase-seg2 5 dsjw 2 dbrp 1
          tdco 15 tdcf 0
          ES582.1/ES584.1: dtseg1 2..32 dtseg2 1..16 dsjw 1..8 dbrp 1..32 \
          dbrp_inc 1
          tdco 0..127 tdcf 0..127
          clock 80000000


```
### 支持的 CAN 硬件


请检查 "drivers/net/can" 中的 "Kconfig" 文件以获取当前支持的 CAN 硬件列表。在 SocketCAN 项目网站（见 socketcan-resources）上可能有更多驱动可用，包括较旧的内核版本。


## SocketCAN 资源


Linux CAN / SocketCAN 项目资源（项目站点 / 邮件列表）在 Linux 源码树的 MAINTAINERS 文件中有引用。搜索 CAN NETWORK [LAYERS|DRIVERS]。

## 致谢


- Oliver Hartkopp（PF_CAN 核心、过滤器、驱动、bcm、SJA1000 驱动）
- Urs Thuermann（PF_CAN 核心、内核集成、套接字接口、raw、vcan）
- Jan Kizka（RT-SocketCAN 核心、Socket-API 协调）
- Wolfgang Grandegger（RT-SocketCAN 核心与驱动、Raw Socket-API 评审、CAN 设备驱动接口、MSCAN 驱动）
- Robert Schwebel（设计评审、PTXdist 集成）
- Marc Kleine-Budde（设计评审、Kernel 2.6 清理、驱动）
- Benedikt Spranger（评审）
- Thomas Gleixner（LKML 评审、代码风格、发布提示）
- Andrey Volkov（内核子树结构、ioctls、MSCAN 驱动）
- Matthias Brukner（首个 SJA1000 CAN 网络设备实现，2003 年第二季度）
- Klaus Hitschler（PEAK 驱动集成）
- Uwe Koppe（采用 PF_PACKET 方式的 CAN 网络设备）
- Michael Schulze（驱动层回环需求、RT CAN 驱动评审）
- Pavel Pisa（位时序计算）
- Sascha Hauer（SJA1000 平台驱动）
- Sebastian Haas（SJA1000 EMS PCI 驱动）
- Markus Plessing（SJA1000 EMS PCI 驱动）
- Per Dalen（SJA1000 Kvaser PCI 驱动）
- Sam Ravnborg（评审、代码风格、kbuild 帮助）
