
## Universal TUN/TAP device driver（通用 TUN/TAP 设备驱动）


Copyright |copy| 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>

  Linux、Solaris 驱动程序
  Copyright |copy| 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>

  FreeBSD TAP 驱动程序
  Copyright |copy| 1999-2000 Maksim Yevmenkin <m_evmenkin@yahoo.com>

  本文档于 2002 年由 Florian Thiel <florian.thiel@gmx.net> 修订

## 1. 说明（Description）


  TUN/TAP 为用户空间程序提供数据包的接收与发送能力。
  它可以被看作一个简单的点对点（Point-to-Point）设备或以太网（Ethernet）设备，
  不同之处在于：它不是从物理介质接收数据包，而是从用户空间程序接收；
  发送数据包时也不是经由物理介质，而是写入用户空间程序。

  要使用此驱动，程序必须打开 /dev/net/tun 并发出相应的 ioctl() 调用，
  向内核注册一个网络设备。根据所选择的选项，该网络设备会呈现为 tunXX 或 tapXX。
  当程序关闭该文件描述符时，该网络设备及其所有相关路由都会消失。

  根据所选设备的类型，用户空间程序必须读取/写入 IP 数据包（tun 设备）
  或以太网帧（tap 设备）。具体使用哪一个，取决于 ioctl() 调用时所传入的标志。

  http://vtun.sourceforge.net/tun 上的软件包包含了两个关于如何使用 tun 和 tap
  设备的简单示例。这两个程序都像是介于两个网络接口之间的桥。
  br_select.c - 基于 select 系统调用的桥。
  br_sigio.c  - 基于异步 IO 与 SIGIO 信号的桥。
  不过，最好的示例还是 VTun http://vtun.sourceforge.net :))

## 2. 配置（Configuration）


```

     mkdir /dev/net（如果尚不存在）
     mknod /dev/net/tun c 10 200

  设置权限::

     e.g. chmod 0666 /dev/net/tun

  允许非 root 用户访问该设备并无危害，因为创建网络设备或者连接到
  不属于该用户的网络设备都需要 CAP_NET_ADMIN 能力。如果你希望创建
  持久化设备并将其所有权交给非特权用户，那么就需要让这些用户能够
  使用 /dev/net/tun 设备。

  驱动模块自动加载

     请确保你的内核中启用了“Kernel module loader”——模块自动加载支持。
     内核应当在首次访问时自动加载它。

  手动加载

     手动插入模块::

	modprobe tun

  如果你采用后一种方式，那么每次需要时就得手动加载模块；如果采用前一种方式，
  那么在打开 /dev/net/tun 时会自动加载。

```
## 3. 程序接口（Program interface）


### 3.1 网络设备分配（Network device allocation）


`char *dev` 应为带格式字符串的设备名（例如
"tun%d"），不过（据我所知）它也可以是任意合法的网络设备名。
注意，该字符指针会被真实的设备名覆盖。
```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_alloc(char *dev)
  {
      struct ifreq ifr;
      int fd, err;

      if( (fd = open("/dev/net/tun", O_RDWR)) < 0 )
	 return tun_alloc_old(dev);

      memset(&ifr, 0, sizeof(ifr));

      /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
       *        IFF_TAP   - TAP device
       *
       *        IFF_NO_PI - Do not provide packet information
       */
      ifr.ifr_flags = IFF_TUN;
      if( *dev )
	 strscpy_pad(ifr.ifr_name, dev, IFNAMSIZ);

      if( (err = ioctl(fd, TUNSETIFF, (void *) &ifr)) < 0 ){
	 close(fd);
	 return err;
      }
      strcpy(dev, ifr.ifr_name);
      return fd;
  }

```
### 3.2 帧格式（Frame format）


```

     Flags [2 bytes]
     Proto [2 bytes]
     Raw protocol(IP, IPv6, etc) frame.

```
### 3.3 多队列 tuntap 接口（Multiqueue tuntap interface）


从 3.8 版本开始，Linux 支持多队列 tuntap，它可以使用多个文件描述符
（队列）来并行地发送或接收数据包。设备分配方式与此前相同；如果用户希望
创建多个队列，则必须使用相同的设备名多次调用带有 IFF_MULTI_QUEUE 标志的
TUNSETIFF。

`char *dev` 应为设备名，queues 是要创建的队列数量，fds 用于存储并向调用者
返回所创建的文件描述符（队列）。每个文件描述符都作为用户空间可访问的
一个队列的接口。

```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_alloc_mq(char *dev, int queues, int *fds)
  {
      struct ifreq ifr;
      int fd, err, i;

      if (!dev)
	  return -1;

      memset(&ifr, 0, sizeof(ifr));
      /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
       *        IFF_TAP   - TAP device
       *
       *        IFF_NO_PI - Do not provide packet information
       *        IFF_MULTI_QUEUE - Create a queue of multiqueue device
       */
      ifr.ifr_flags = IFF_TAP | IFF_NO_PI | IFF_MULTI_QUEUE;
      strcpy(ifr.ifr_name, dev);

      for (i = 0; i < queues; i++) {
	  if ((fd = open("/dev/net/tun", O_RDWR)) < 0)
	     goto err;
	  err = ioctl(fd, TUNSETIFF, (void *)&ifr);
	  if (err) {
	     close(fd);
	     goto err;
	  }
	  fds[i] = fd;
      }

      return 0;
  err:
      for (--i; i >= 0; i--)
	  close(fds[i]);
      return err;
  }

```
引入了一个新的 ioctl(TUNSETQUEUE) 用于启用或禁用某个队列。当以
IFF_DETACH_QUEUE 标志调用它时，该队列被禁用；当以 IFF_ATTACH_QUEUE 标志
调用时，该队列被启用。通过 TUNSETIFF 创建后，该队列默认处于启用状态。

fd 为我们想要启用或禁用的文件描述符（队列），当
```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_set_queue(int fd, int enable)
  {
      struct ifreq ifr;

      memset(&ifr, 0, sizeof(ifr));

      if (enable)
	 ifr.ifr_flags = IFF_ATTACH_QUEUE;
      else
	 ifr.ifr_flags = IFF_DETACH_QUEUE;

      return ioctl(fd, TUNSETQUEUE, (void *)&ifr);
  }

```
## Universal TUN/TAP device driver 常见问题（Frequently Asked Question）


1. TUN/TAP 驱动支持哪些平台？

目前该驱动已针对 3 种 Unix 系统编写：

  - Linux kernels 2.2.x, 2.4.x
  - FreeBSD 3.x, 4.x, 5.x
  - Solaris 2.6, 7.0, 8.0

2. TUN/TAP 驱动的用途是什么？

如上所述，TUN/TAP 驱动的主要用途是隧道（tunneling）。
它被 VTun（http://vtun.sourceforge.net）所使用。

另一个使用 TUN/TAP 的有趣应用是 pipsecd
（http://perso.enst.fr/~beyssac/pipsec/），这是一个用户空间 IPSec
实现，可以使用完整的内核路由（不同于 FreeS/WAN）。

3. 虚拟网络设备实际是如何工作的？

虚拟网络设备可以看作一个简单的点对点或以太网设备，它与普通设备的
不同之处在于：不是从物理介质接收数据包，而是从用户空间程序接收；
发送数据包时也不是经由物理介质，而是发送给用户空间程序。

假设你在 tap0 上配置了 IPv6，那么每当内核向 tap0 发送一个 IPv6 数据包时，
它就会被传递给应用程序（例如 VTun）。应用程序对该包进行加密、压缩，并通过
TCP 或 UDP 发送到对端。对端的应用程序对收到的数据进行解压缩和解密，然后
将数据包写入 TAP 设备，内核会像处理来自真实物理设备的数据包一样处理它。

4. TUN 驱动和 TAP 驱动有什么区别？

TUN 处理 IP 帧。TAP 处理以太网帧。

这意味着使用 tun 时必须读取/写入 IP 数据包，而使用 tap 时则读取/写入
以太网帧。

5. BPF 与 TUN/TAP 驱动有什么区别？

BPF 是一种高级数据包过滤器。它可以附加到已有的网络接口上，但
并不提供虚拟网络接口。TUN/TAP 驱动确实提供虚拟网络接口，并且可以
将 BPF 附加到该接口上。

6. TAP 驱动支持内核以太网桥接吗？

支持。Linux 和 FreeBSD 驱动都支持以太网桥接。
