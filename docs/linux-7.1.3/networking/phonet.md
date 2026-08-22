
## Linux Phonet 协议

### 简

Phonet 是诺基亚蜂窝调制解调器用IPC RPC 的数据包协议。通过 Linux Phonet
套接字族，Linux 主机进程可以从调制解调器或连接到调制解调器的任何其它外部设备
接收和发送消息。调制解调器负责路由
Phonet 数据包可以通过各种硬件连接交换，具体取决于设备，例如：

  - 带有 CDC Phonet 接口USB  - 红外  - 蓝牙  - RS232 串行端口（带有专用的 "FBUS" 线路规程），
  - 带有某些 TI OMAP 处理器的 SSI 总线

### 数据包格

```

  struct phonethdr {
    uint8_t  pn_media;  /* Media type (link-layer identifier) */
    uint8_t  pn_rdev;   /* Receiver device ID */
    uint8_t  pn_sdev;   /* Sender device ID */
    uint8_t  pn_res;    /* Resource ID or function */
    uint16_t pn_length; /* Big-endian message byte length (minus 6) */
    uint8_t  pn_robj;   /* Receiver object ID */
    uint8_t  pn_sobj;   /* Sender object ID */
  };

```
Linux 上，链路层头部包pn_media 字节（见下文）。接下来7 个字节是
网络层头部的一部分
设备 ID 被拆分：6 位构成设备地址，而低 2 位用于多路复用，8 位对象标识符
也是如此。因此，Phonet 可以被视为一个具6 位地址空间10 位传输协议（很像
IP 世界中的端口号）的网络层
调制解调器始终具有地址编号零。所有其它设备都有自己的 6 位地址

### 閾捐矾灞。

Phonet 链路始终是点对点链路。链路层头部由单Phonet 介质类型字节组成。从
调制解调器的角度来看，它唯一标识数据包所经由的链路。每Phonet 网络设备适当地前置并设置介质类型字节。为方便起见，提供了一个通用phonet_header_ops
链路层头部操作结构。它根据网络设备硬件地址设置介质类型
Linux Phonet 网络接口支持专用的链路层数据包类型（ETH_P_PHONET），它超以太网类型范围。它们只能发送和接收 Phonet 数据包
虚拟 TUN 隧道设备驱动程序也可用于 Phonet。这需IFF_TUN 模式，_不_IFF_NO_PI
标志。在这种情况下，没有链路层头部，因此没有 Phonet 介质类型字节
注意，Phonet 接口不允许对数据包重新排序，因此只能与（默认的）Linux FIFO qdisc
一起使用

### 缃戠粶灞。

```

  struct sockaddr_pn {
    sa_family_t spn_family;    /* AF_PHONET */
    uint8_t     spn_obj;       /* Object ID */
    uint8_t     spn_dev;       /* Device ID */
    uint8_t     spn_resource;  /* Resource or function */
    uint8_t     spn_zero[...]; /* Padding */
  };

```
resource 字段仅在发送和接收时使用；它在 bind() getsockname() 中被忽略

### 底层数据报协

应用程序可以使用来自 PF_PHONET 族的 Phonet 数据报套接字协议发Phonet 消息每个套接字绑定到可用2^10 个对ID 之一，并可以与任何其它对等方发送和接收
数据包
```

  struct sockaddr_pn addr = { .spn_family = AF_PHONET, };
  ssize_t len;
  socklen_t addrlen = sizeof(addr);
  int fd;

  fd = socket(PF_PHONET, SOCK_DGRAM, 0);
  bind(fd, (struct sockaddr *)&addr, sizeof(addr));
  /* ... */

  sendto(fd, msg, msglen, 0, (struct sockaddr *)&addr, sizeof(addr));
  len = recvfrom(fd, buf, sizeof(buf), 0,
		 (struct sockaddr *)&addr, &addrlen);

```
此协议遵SOCK_DGRAM 无连接语义。但是，不支connect() getpeername()因为它们Phonet 用法中似乎没有用处（可以轻松添加）

### 资源订阅


Phonet 数据报套接字可以订阅任意数量8 ```

  uint32_t res = 0xXX;
  ioctl(fd, SIOCPNADDRESOURCE, &res);

```
订阅同样使用 SIOCPNDELRESOURCE I/O 控制请求取消，或在套接字关闭时取消
注意，任何给定资源一次最多只能有一个套接字订阅。否则，ioctl() 将返EBUSY

### Phonet 管道协议


Phonet 管道协议是一种带有端到端拥塞控制的简单有序数据包协议。它使用被动监听
套接字范式。监听套接字绑定到一个唯一的空闲对ID。每个监听套接字最多可处理
255 个并发连接，每个 accept() 到的套接字一个
```

  int lfd, cfd;

  lfd = socket(PF_PHONET, SOCK_SEQPACKET, PN_PROTO_PIPE);
  listen (lfd, INT_MAX);

  /* ... */
  cfd = accept(lfd, NULL, NULL);
  for (;;)
  {
    char buf[...];
    ssize_t len = read(cfd, buf, sizeof(buf));

    /* ... */

    write(cfd, msg, msglen);
  }

```
连接传统上由"第三应用在两端之间建立。这意味着两端都是被动的

Linux 内核版本 2.6.39 起，也可以使用主动端connect() 直接连接两个端点这旨在支持较新的诺基亚无线调制解调器 API，如```

  struct sockaddr_spn spn;
  int fd;

  fd = socket(PF_PHONET, SOCK_SEQPACKET, PN_PROTO_PIPE);
  memset(&spn, 0, sizeof(spn));
  spn.spn_family = AF_PHONET;
  spn.spn_obj = ...;
  spn.spn_dev = ...;
  spn.spn_resource = 0xD9;
  connect(fd, (struct sockaddr *)&spn, sizeof(spn));
  /* normal I/O here ... */
  close(fd);


```
中找到的
   当轮询已连接的管道套接字以检查可写性时，存在一个内在的竞态条件，即可写性可   在轮询和系统调用写入之间丢失。在这种情况下，套接字将阻塞，直到写入再次变   可能，除非启用了非阻塞模式

管道协议SOL_PNPIPE 级别提供两个套接字选项
  PNPIPE_ENCAP 接受一个整数值（int）：

    PNPIPE_ENCAP_NONE      套接字正常运行（默认）
    PNPIPE_ENCAP_IP      套接字用作虚IP 接口的后端。这需CAP_NET_ADMIN 能力。诺基亚调制解调      上的 GPRS 数据支持可以使用此选项。注意，在此模式下不能可靠地对该套接字进      poll() read()
  PNPIPE_IFINDEX
      是一个只读整数值。它包含PNPIPE_ENCAP 创建的网络接口的接口索引      如果封装关闭则为 0
  PNPIPE_HANDLE
      是一个只读整数值。它包含管道的底层标识符pipe handle"）。这仅为
      已连接或正在连接的套接字描述符定义

### 作

Linux Phonet 最初由 Sakari Ailus 编写
其它贡献者包Mikä Liljeberg、Andras Domokos、Carlos Chinea Rémi Denis-Courmont
Copyright |copy| 2008 Nokia Corporation.
