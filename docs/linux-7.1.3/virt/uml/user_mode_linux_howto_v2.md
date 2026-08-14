
# UML HowTo


######## Introduction


欢迎使用 User Mode Linux（用户态 Linux）

User Mode Linux 是第一个开源虚拟化平台（首个发布日期为 1991 年），也是
x86 PC 上的第二个虚拟化平台。

## UML 与使用虚拟化软件包 X 的虚拟机有何不同？


我们往往默认虚拟化也意味着某种程度的硬件仿真。实际上并非如此。只要
虚拟化软件包为操作系统提供操作系统能够识别并拥有驱动程序的设备，这些
设备就不需要仿真真实硬件。如今大多数操作系统都内置支持一些仅在虚拟化
下使用的"伪"设备。
User Mode Linux 将这个概念推向了极致——眼前看不到任何真实的设备。它是
100% 人工的，或者用正确的术语说，是 100% 半虚拟（paravirtual）的。所有
UML 设备都是抽象概念，它们映射到宿主机提供的某种东西——文件、套接字、
管道等。

UML 与各种虚拟化软件包的另一个主要区别在于，UML 内核与 UML 程序的
运作方式存在明显差别。
UML 内核只是运行在 Linux 上的一个进程——与任何其他程序一样。它可以由
非特权用户运行，并且不需要任何特殊的 CPU 特性。
然而，UML 用户空间略有不同。宿主机上的 Linux 内核会协助 UML 拦截运行在
UML 实例上的程序试图执行的一切操作，并让 UML 内核处理其所有请求。
这与其他不区分客户机内核与客户机程序的虚拟化软件包不同。这种差异使得
UML 相比（例如）QEMU 具有一系列优缺点，我们将在本文档后续部分介绍。


## 我为什么需要 User Mode Linux？


- 如果 User Mode Linux 内核崩溃，你的宿主机内核仍然完好。它没有任何加速
  （vhost、kvm 等），也不试图直接访问任何设备。事实上，它就是一个与其
  他进程无异的进程。

- 你可以以非 root 用户身份运行用户态内核（你可能需要为某些设备安排适当
  的权限）。

- 你可以运行一个占用极小、针对特定任务的最小虚拟机（例如 32M 或更小）。

- 对于"内核特定任务"（如转发、防火墙等），你可以在仍与宿主机内核隔离的
  情况下获得极高的性能。

- 你可以在不破坏系统的情况下试验内核概念。

- 你不受"仿真"硬件的约束，因此可以尝试一些在仿真真实硬件（如时间旅行，
  或让系统时钟依赖于 UML 的行为——对测试等非常有用）时很难支持的奇特
  而美妙的概念。

- 它很有趣。

## 为何不运行 UML


- UML 使用的系统调用拦截技术使得它对任何用户空间应用程序而言天生更慢。
  虽然它在内核任务上可以与大多数其他虚拟化软件包并驾齐驱，但其用户空间
  是**慢**的。根本原因在于 UML 创建新进程和线程的代价非常高（这是大多数
  Unix/Linux 应用程序认为理所当然的事情）。

- UML 目前严格是单处理器的。如果你想运行一个需要多个 CPU 才能运作的
  应用程序，它显然是错误的选择。


######## 构建 UML 实例


任何发行版中都没有 UML 安装程序。虽然你可以使用现成的安装介质通过某个
虚拟化软件包安装到一个空白虚拟机中，但没有对应的 UML 方式。你必须使用
宿主机上的适当工具来构建一个可用的文件系统映像。

在 Debian 上这极其简单——你可以用 debootstrap 完成。在 OpenWRT 上也
很简单——构建过程可以构建 UML 映像。所有其他发行版——视情况而定
（YMMV）。

## 创建映像


```
   # dd if=/dev/zero of=disk_image_name bs=1 count=1 seek=16G
```

这将创建一个 16G 的磁盘映像。操作系统最初只分配一个块，并会在 UML 写入
时分配更多块。从内核版本 4.19 起，UML 完全支持 TRIM（通常用于闪存盘）。
通过在 UML 映像内指定 discard 作为挂载选项，或运行
`tune2fs -o discard /dev/ubdXX`，将请求 UML 把任何未使用的块归还给操作系统。

```
   # mkfs.ext4 ./disk_image_name && mount ./disk_image_name /mnt
```

此示例使用 ext4，任何其他文件系统如 ext3、btrfs、xfs、jfs 等也同样适用。

```
   # debootstrap buster /mnt http://deb.debian.org/debian
```

debootstrap 不会设置 root 密码、fstab、主机名或与网络相关的任何内容。
这要由用户来完成。

设置 root 密码——最简单的方法是 chroot 进入该映像：

```
   # chroot /mnt
   # passwd
   # exit
```

## 编辑关键系统文件


UML 块设备被称为 ubd。debootstrap 创建的 fstab：

```
   /dev/ubd0   ext4    discard,errors=remount-ro  0       1
```

映像的主机名将被设置为你创建其映像所在宿主机的主机名。最好改掉它，以免
出现"哎呀，我重启了错误的机器"。

UML 支持高性能网络设备 vector I/O，它支持一些标准虚拟网络封装，如
Ethernet over GRE 和 Ethernet over L2TPv3。这些被称为 vecX。

当使用 vector 网络设备时，`/etc/network/interfaces`：

```
   # vector UML network devices
   auto vec0
   iface vec0 inet dhcp
```

现在我们有一个几乎可以运行的 UML 映像，我们所需要的只是一个 UML 内核和
它的模块。

大多数发行版都有 UML 软件包。即使你打算使用自己的内核，用现成的内核
测试映像总是一个好的开始。这些软件包附带一组应复制到目标文件系统的模块。
其位置依赖于发行版。对于 Debian，它们位于 /usr/lib/uml/modules。递归
复制此目录的内容：

```
   # cp -rax /usr/lib/uml/modules /mnt/lib/modules
```

如果你编译了自己的内核，你需要使用通常的 "install"：

```
  # make INSTALL_MOD_PATH=/mnt/lib/modules modules_install
```

这会将模块安装到 /mnt/lib/modules/$(KERNELRELEASE)。

```
  # make MODLIB=/mnt/lib/modules modules_install
```

至此，映像已准备好被启动。


######## 设置 UML 网络


UML 网络旨在仿真以太网连接。此连接可以是点对点（类似于使用背靠背线缆
连接机器）或连接到交换机。UML 支持多种方式来建立到以下所有目标的连接：
本地机器、远程机器、本地和远程 UML 以及其他 VM 实例。


+-----------+--------+------------------------------------+------------+
| Transport |  Type  |        Capabilities                | Throughput |
+===========+========+====================================+============+
| tap       | vector | checksum, tso                      | > 8Gbit    |
+-----------+--------+------------------------------------+------------+
| hybrid    | vector | checksum, tso, multipacket rx      | > 6GBit    |
+-----------+--------+------------------------------------+------------+
| raw       | vector | checksum, tso, multipacket rx, tx" | > 6GBit    |
+-----------+--------+------------------------------------+------------+
| EoGRE     | vector | multipacket rx, tx                 | > 3Gbit    |
+-----------+--------+------------------------------------+------------+
| Eol2tpv3  | vector | multipacket rx, tx                 | > 3Gbit    |
+-----------+--------+------------------------------------+------------+
| bess      | vector | multipacket rx, tx                 | > 3Gbit    |
+-----------+--------+------------------------------------+------------+
| fd        | vector | dependent on fd type               | varies     |
+-----------+--------+------------------------------------+------------+
| vde       | vector | dep. on VDE VPN: Virt.Net Locator  | varies     |
+-----------+--------+------------------------------------+------------+

- 所有具有 tso 和 checksum 卸载的传输都可以在 TCP 流上提供接近 10G 的速度。

- 所有具有多包 rx 和/或 tx 的传输可以提供高达 1Mpps 或更高的包速率。

- GRE 和 L2TPv3 允许连接到以下所有目标：本地机器、远程机器、远程网络设备
  和远程 UML 实例。


## 网络配置权限


大多数受支持的网络模式需要 `root` 权限。例如，对于 vector 传输，需要
`root` 权限来发起 ioctl 以建立 tun 接口和/或在需要时使用的原始套接字。

这可以通过授予用户特定的 capability 来实现，而无需以 root 运行 UML。在
vector 传输的情况下，用户可以向 uml 二进制添加 capability `CAP_NET_ADMIN`
或 `CAP_NET_RAW`。此后，UML 可以以普通用户权限运行，同时拥有完整的网络
功能。

```
   # sudo setcap cap_net_raw,cap_net_admin+ep linux
```

## 配置 vector 传输


所有 vector 传输都支持类似的语法：

如果 X 是接口编号，如 vec0、vec1、vec2 等，通用语法为：

```
   vecX:transport="Transport Name",option=value,option=value,...,option=value
```

### 通用选项


这些选项对所有传输通用：

- `depth=int` - 设置 vector IO 的队列深度。这是 UML 将尝试在单次系统调用
  中读写的包数量。默认数值为 64，通常足以满足大多数需要 2-4 Gbit 吞吐量的
  应用。更高的速度可能需要更大的数值。

- `mac=XX:XX:XX:XX:XX` - 设置接口 MAC 地址值。

- `gro=[0,1]` - 设置 GRO 关闭或开启。启用接收/发送卸载。此选项的效果
  取决于所配置传输在宿主机侧的支持。在大多数情况下，它将启用 TCP 分片
  和 RX/TX 校验和卸载。宿主机侧和 UML 侧的设置必须一致。如果不一致，UML
  内核会产生警告。例如，GRO 在本地机器接口（如 veth 对、桥等）上默认开启，
  因此应在 UML 中相应的 UML 传输（raw、tap、hybrid）中启用，以便网络正常
  工作。

- `mtu=int` - 设置接口 MTU

- `headroom=int` - 调整默认的 headroom（32 字节），以防一个包需要被重新
  封装（例如到 VXLAN）。

- `vec=0` - 禁用多包 IO 并回退到一次一个包的模式

### 共享选项


- `ifname=str` 绑定到本地网络接口的传输有一个共享选项——要绑定的接口名称。

- `src, dst, src_port, dst_port` - 所有使用套接字（具有源和目的以及/或
  源端口和目的端口概念）的传输都使用这些来指定它们。

- `v6=[0,1]` 用于指定所有基于 IP 运行的传输是否需要 v6 连接。此外，对于
  在 v4 和 v6 上运作方式有所不同的传输（例如 EoL2TPv3），设置正确的运作
  模式。在没有此选项的情况下，套接字类型由 src 和 dst 参数解析/解析得到的
  结果决定。

### tap 传输


```
   vecX:transport=tap,ifname=tap0,depth=128,gro=1
```

这将把 vec0 连接到宿主机上的 tap0。tap0 必须已经存在（例如使用 tunctl
创建）并且处于 UP 状态。

tap0 可以配置为点对点接口并给定一个 IP 地址，以便 UML 可以与宿主机通信。
或者，也可以将 UML 连接到一个连接到桥的 tap 接口。

虽然 tap 依赖 vector 基础设施，但此刻它并不是一个真正的 vector 传输，因为
Linux 不支持在像 UML 这样的普通用户空间应用程序的 tap 文件描述符上进行
多包 IO。这一特权仅提供给能够通过 vhost-net 等专门接口在内核层面挂接的
东西。为 UML 提供类似 vhost-net 的辅助程序计划在将来的某个时刻实现。

所需权限：tap 传输需要以下之一：

- tap 接口已存在，并由 UML 用户使用 tunctl 创建为持久且归其所有。示例
  `tunctl -u uml-user -t tap0`

- 二进制具有 `CAP_NET_ADMIN` 特权

### hybrid 传输


```
   vecX:transport=hybrid,ifname=tap0,depth=128,gro=1
```

这是一个实验性/演示传输，它将 tap 用于发送，将原始套接字用于接收。原始
套接字允许多包接收，从而产生比普通 tap 高得多的包速率。

所需权限：hybrid 需要 UML 用户具有 `CAP_NET_RAW` 能力，以及 tap 传输的
要求。

### raw 套接字传输


```
   vecX:transport=raw,ifname=p-veth0,depth=128,gro=1
```

此传输在原始套接字上使用 vector IO。虽然你可以绑定到任何接口（包括物理
接口），但最常见的用法是绑定到 veth 对的"对端"一侧，另一侧配置在宿主机上。

Debian 的示例宿主机配置：

```
   auto veth0
   iface veth0 inet static
	address 192.168.4.1
	netmask 255.255.255.252
	broadcast 192.168.4.3
	pre-up ip link add veth0 type veth peer name p-veth0 && \
          ifconfig p-veth0 up
```

```
   vec0:transport=raw,ifname=p-veth0,depth=128,gro=1
```

如果 UML 客户机配置为 192.168.4.2、子网掩码 255.255.255.0，它就可以与
宿主机 192.168.4.1 通信。

raw 传输还提供将部分过滤卸载到宿主机的支持。控制它的两个选项是：

- `bpffile=str` 要作为套接字过滤器加载的原始 bpf 代码的文件名

- `bpfflash=int` 0/1 允许从 User Mode Linux 内部加载 bpf。此选项允许使用
  ethtool load firmware 命令来加载 bpf 代码。

在这两种情况下，bpf 代码都被加载到宿主机内核中。虽然目前这仅限于传统 bpf
语法（非 ebpf），但它仍然是一个安全风险。除非 User Mode Linux 实例被视为
可信，否则不建议允许这样做。

所需权限：raw 套接字传输需要 `CAP_NET_RAW` 能力。

### GRE 套接字传输


```
   vecX:transport=gre,src=$src_host,dst=$dst_host
```

这将配置一个 Ethernet over `GRE`（又称 `GRETAP` 或 `GREIRB`）隧道，它将把
UML 实例连接到位于宿主机 dst_host 的 `GRE` 端点。`GRE` 支持以下附加选项：

- `rx_key=int` - 用于 rx 包的 GRE 32 位整数密钥，如果设置，则 `txkey`
  也必须设置

- `tx_key=int` - 用于 tx 包的 GRE 32 位整数密钥，如果设置，则 `rx_key`
  也必须设置

- `sequence=[0,1]` - 启用 GRE 序列

- `pin_sequence=[0,1]` - 假装序列在每个包上都重置（需要与某些确实实现得很
  糟糕的实现互通时使用）

- `v6=[0,1]` - 分别强制使用 IPv4 或 IPv6 套接字

- GRE 校验和目前不支持

GRE 有一些注意事项：

- 每个 IP 地址你只能使用一个 GRE 连接。无法复用连接，因为每个 GRE 隧道
  都直接在 UML 实例上终结。

- 密钥并不是一个真正的安全特性。虽然它被设计成这样，但它的"安全性"很
  可笑。然而，它是一个有用的特性，用于确保隧道没有被错误配置。

一个本地地址为 192.168.128.1 的 Linux 宿主机连接到位于 192.168.129.1 的
UML 实例的示例配置：

```
   auto gt0
   iface gt0 inet static
    address 10.0.0.1
    netmask 255.255.255.0
    broadcast 10.0.0.255
    mtu 1500
    pre-up ip link add gt0 type gretap local 192.168.128.1 \
           remote 192.168.129.1 || true
    down ip link del gt0 || true
```

此外，GRE 已经针对各种网络设备进行了测试。

所需权限：GRE 需要 `CAP_NET_RAW`

### l2tpv3 套接字传输


_警告_。L2TPv3 有一个"bug"。这个"bug"就是"选项比 GNU ls 还多"。虽然
它有一些优点，但通常有更简单（也更简洁）的方式将 UML 实例连接到某处。
例如，大多数支持 L2TPv3 的设备也支持 GRE。

```
    vec0:transport=l2tpv3,udp=1,src=$src_host,dst=$dst_host,srcport=$src_port,dstport=$dst_port,depth=128,rx_session=0xffffffff,tx_session=0xffff
```

这将配置一个 Ethernet over L2TPv3 固定隧道，它将使用 L2TPv3 UDP 风格和
UDP 目的端口 $dst_port，把 UML 实例连接到位于宿主机 $dst_host 的 L2TPv3
端点。

L2TPv3 总是需要以下附加选项：

- `rx_session=int` - 用于 rx 包的 l2tpv3 32 位整数会话

- `tx_session=int` - 用于 tx 包的 l2tpv3 32 位整数会话

由于隧道是固定的，这些不会被协商，而是在两端预先配置。

此外，L2TPv3 支持以下可选参数。

- `rx_cookie=int` - 用于 rx 包的 l2tpv3 32 位整数 cookie——与 GRE 密钥
  功能相同，更多是为了防止错误配置而非提供实际安全性

- `tx_cookie=int` - 用于 tx 包的 l2tpv3 32 位整数 cookie

- `cookie64=[0,1]` - 使用 64 位 cookie 而非 32 位。

- `counter=[0,1]` - 启用 l2tpv3 计数器

- `pin_counter=[0,1]` - 假装计数器在每个包上都重置（需要与某些确实实现得
  很糟糕的实现互通时使用）

- `v6=[0,1]` - 强制使用 v6 套接字

- `udp=[0,1]` - 使用原始套接字（0）或 UDP（1）版本的协议

L2TPv3 有一些注意事项：

- 在 raw 模式下，每个 IP 地址只能使用一个连接。无法复用连接，因为每个
  L2TPv3 隧道都直接在 UML 实例上终结。UDP 模式可以使用不同的端口来达到此
  目的。

下面是配置一个 Linux 宿主机通过 L2TPv3 连接到 UML 的示例：

```
   auto l2tp1
   iface l2tp1 inet static
    address 192.168.126.1
    netmask 255.255.255.0
    broadcast 192.168.126.255
    mtu 1500
    pre-up ip l2tp add tunnel remote 127.0.0.1 \
           local 127.0.0.1 encap udp tunnel_id 2 \
           peer_tunnel_id 2 udp_sport 1706 udp_dport 1707 && \
           ip l2tp add session name l2tp1 tunnel_id 2 \
           session_id 0xffffffff peer_session_id 0xffffffff
    down ip l2tp del session tunnel_id 2 session_id 0xffffffff && \
           ip l2tp del tunnel tunnel_id 2
```

所需权限：L2TPv3 在 raw IP 模式下需要 `CAP_NET_RAW`，而 UDP 模式不需要
特殊权限。

### BESS 套接字传输


BESS 是一个高性能模块化网络交换机。

https://github.com/NetSys/bess

它支持一种简单的顺序包套接字模式，在较新版本中使用 vector IO 以获得高
性能。

```
   vecX:transport=bess,src=$unix_src,dst=$unix_dst
```

这将配置一个 BESS 传输，使用 unix_src Unix 域套接字地址作为源，unix_dst
套接字地址作为目的。

有关 BESS 配置以及如何分配 BESS Unix 域套接字端口，请参阅 BESS 文档。

https://github.com/NetSys/bess/wiki/Built-In-Modules-and-Ports

BESS 传输不需要任何特殊权限。

### VDE vector 传输


Virtual Distributed Ethernet（VDE）是一个主要目标是为虚拟网络提供高度
灵活支持的项目。

http://wiki.virtualsquare.org/#/tutorials/vdebasics

VDE 的常见用途包括快速原型设计和教学。

示例：

   `vecX:transport=vde,vnl=tap://tap0`

使用 tap0

   `vecX:transport=vde,vnl=slirp://`

使用 slirp

   `vec0:transport=vde,vnl=vde:///tmp/switch`

连接到一个 vde 交换机

   `vecX:transport=\"vde,vnl=cmd://ssh remote.host //tmp/sshlirp\"`

连接到一个远程 slirp（即时 VPN：将 ssh 转换为 VPN，它使用 sshlirp）
https://github.com/virtualsquare/sshlirp

   `vec0:transport=vde,vnl=vxvde://234.0.0.1`

连接到一个局域网云（所有使用相同组播地址、运行在相同组播域（LAN）中主机
上的 UML 节点将被自动连接到一起，构成一个虚拟 LAN）。


######## 运行 UML


本节假设宿主机上已经安装了来自发行版的用户态 Linux 软件包或自定义构建的
内核。

这些会向系统添加一个名为 linux 的可执行文件。这就是 UML 内核。它可以
像任何其他可执行文件一样运行。它将接受大多数普通的 linux 内核参数作为
命令行参数。此外，为了做一些有用的事情，它还需要一些 UML 特定的参数。

## 参数


### 必需参数：


- `mem=int[K,M,G]` - 内存大小。默认以字节为单位。它也接受 K、M 或 G
  限定符。

- `ubdX[s,d,c,t]=` 虚拟磁盘规格。这并非真正必需，但在几乎所有情况下
  都可能需要，这样我们才能指定一个根文件系统。最简单的映像规格就是
  文件系统映像的文件名（使用 `Creating an image`_ 中描述的方法之一创建）。

  - UBD 设备支持写时复制（COW）。变更保存在一个单独的文件中，可以丢弃该
    文件以回滚到原始纯净映像。如果需要 COW，UBD 映像指定为：
    `cow_file,master_image`。
    示例：`ubd0=Filesystem.cow,Filesystem.img`

  - UBD 设备可以设置为使用同步 IO。任何写入都会立即刷新到磁盘。这是通过在
    `ubdX` 规格后面添加 `s` 来实现的。

  - UBD 会对指定为单一文件名的设备做一些启发式检查，以确保没有把 COW 文件
    指定为映像。要关闭它们，请在 `ubdX` 后使用 `d` 标志。

  - UBD 支持 TRIM——请求宿主操作系统回收映像中任何未使用的块。要关闭它，
    请在 `ubdX` 后指定 `t` 标志。

- `root=` 根设备——最可能是 `/dev/ubd0`（这是一个 Linux 文件系统映像）

### 重要可选参数


如果 UML 作为 "linux" 运行且没有额外参数，它将尝试为映像内配置的每个
控制台（在大多数 Linux 发行版中最多 6 个）启动一个 xterm。每个控制台都在
一个 xterm 内启动。这使得在有 GUI 的宿主机上使用 UML 变得简单方便。然而，
如果 UML 要作为一个测试工具或在纯文本环境中运行，这是错误的方法。

为了改变这种行为，我们需要指定一个替代控制台并将其连接到受支持的"线路"
通道之一。为此，我们需要将一个控制台映射为使用不同于默认 xterm 的东西。

```
   con1=fd:0,fd:1
```

UML 支持各种各样的串行线路通道，使用以下语法指定：

   conX=channel_type:options[,channel_type:options]

如果通道规格包含由逗号分隔的两部分，则第一部分是输入，第二部分是输出。

- null 通道——丢弃所有输入或输出。示例 `con=null` 会默认将所有控制台
  设为 null。

- fd 通道——使用文件描述符编号进行输入/输出。示例：`con1=fd:0,fd:1.`

- port 通道——在 TCP 端口号上启动一个 telnet 服务器。示例：`con1=port:4321`。
  宿主机必须具有 /usr/sbin/in.telnetd（通常是 telnetd 软件包的一部分）以及
  UML 工具中的 port-helper（参见下面 xterm 通道的信息）。在客户端连接之前，
  UML 不会启动。

- pty 和 pts 通道——使用系统 pty/pts。

- tty 通道——绑定到一个现有的系统 tty。示例：`con1=/dev/tty8` 将使 UML
  使用宿主机的第 8 个控制台（通常未使用）。

- xterm 通道——这是默认通道——在该通道上启动一个 xterm 并将 IO 导向它。
  注意，为了让 xterm 工作，宿主机必须安装 UML 发行版软件包。它通常包含
  UML 与 xterm 通信所需的 port-helper 和其他工具。或者，需要从源代码编译
  并安装这些工具。所有适用于控制台的选项也适用于 UML 串行线路，后者在 UML
  内呈现为 ttyS。

## 启动 UML


我们现在可以运行 UML 了。

```
   # linux mem=2048M umid=TEST \
    ubd0=Filesystem.img \
    vec0:transport=tap,ifname=tap0,depth=128,gro=1 \
    root=/dev/ubda con=null con0=null,fd:2 con1=fd:0,fd:1
```

这将运行一个具有 `2048M RAM` 的实例，并尝试使用名为 `Filesystem.img` 的
映像文件作为根。它将使用 tap0 连接到宿主机。除 `con1` 外的所有控制台都
将被禁用，而控制台 1 将使用标准输入/输出，使其出现在启动它的同一终端中。

## 登录


如果你在生成映像时没有设置密码，你将不得不关闭 UML 实例、挂载映像、chroot
进入其中并设置密码——如"生成映像"一节所述。如果密码已经设置，你可以直接
登录。

## UML 管理控制台


除了使用常规系统管理工具从"内部"管理映像外，还可以使用 UML 管理控制台
执行许多底层操作。UML 管理控制台是到运行中的 UML 实例上内核的底层接口，
有点像 i386 的 SysRq 接口。由于 UML 之下有一个完整的操作系统，因此比起
SysRq 机制，可能具有更大的灵活性。

你可以用 mconsole 接口做很多事情：

- 获取内核版本
- 添加和移除设备
- 停止或重启机器
- 发送 SysRq 命令
- 暂停和恢复 UML
- 检查运行在 UML 内部的进程
- 检查 UML 内部的 /proc 状态

你需要 mconsole 客户端（uml\_mconsole），它是大多数 Linux 发行版中 UML
工具包的一部分。

你还需要在 UML 中启用 `CONFIG_MCONSOLE`（位于 'General Setup' 下）：

```
   mconsole initialized on /home/jdike/.uml/umlNJ32yL/mconsole
```

如果你在 UML 命令行上指定了唯一的机器 id，即：

```
   mconsole initialized on /home/jdike/.uml/debian/mconsole
```

该文件就是 uml_mconsole 用来与内核通信的套接字：

```
   # uml_mconsole debian
```

或

   # uml_mconsole /home/jdike/.uml/debian/mconsole

你将得到一个提示符，可以在其中运行以下命令之一：

- version
- help
- halt
- reboot
- config
- remove
- sysrq
- help
- cad
- stop
- go
- proc
- stack

### version


```
   (mconsole)  version
   OK Linux OpenWrt 4.14.106 #0 Tue Mar 19 08:19:41 2019 x86_64
```

这有几个实际用途。它是一个简单的空操作，可用于检查 UML 是否正在运行。它
也是向 UML 发送设备中断的一种方式。UML mconsole 在内部被视为一个 UML 设备。

### help


此命令不带参数。它会打印一个简短的帮助屏幕，列出受支持的 mconsole 命令。


### halt 和 reboot


这些命令不带参数。它们会立即关闭机器，不同步磁盘，也不干净地关闭用户空间。
因此，它们是：

```
   (mconsole)  halt
   OK
```

### config


"config" 向虚拟机添加一个新设备。大多数 UML 设备驱动都支持此功能。它接受
一个参数，即：

```
   (mconsole) config ubd3=/home/jdike/incoming/roots/root_fs_debian22
```

### remove


"remove" 从系统中删除一个设备。它的参数就是待移除设备的名称。该设备必须在
驱动认为必要的任何意义上处于空闲状态。对于 ubd 驱动，被移除的块设备不能
被挂载、用作交换区，或以其他方式：

```
   (mconsole)  remove ubd3
```

### sysrq


此命令接受一个参数，即单个字母。它调用通用内核的 SysRq 驱动，由该参数
决定执行什么操作。请参阅你喜欢的某个内核树中
Documentation/admin-guide/sysrq.rst 下的 SysRq 文档，了解哪些字母有效以及
它们的作用。

### cad


这会调用运行中的映像里的 `Ctl-Alt-Del` 动作。具体会做什么取决于 init、
systemd 等。通常，它会重启机器。

### stop


这将使 UML 进入一个循环，读取 mconsole 请求，直到收到 'go' mconsole 命令。
作为一个调试/快照工具，这非常有用。

### go


这会在被 'stop' 命令暂停后恢复 UML。注意，当 UML 恢复后，TCP 连接可能已
超时；如果 UML 被暂停了很长一段时间，crond 可能会发疯，运行它之前没有做的
所有任务。

### proc


这接受一个参数——/proc 中一个文件的名称，该文件会被打印到 mconsole 标准
输出

### stack


这接受一个参数——一个进程的 pid 编号。它的栈会被打印到标准输出。


######## 高级 UML 主题


## 在虚拟机之间共享文件系统


不要试图简单地通过从同一个文件启动两个 UML 来共享文件系统。这与从共享
磁盘启动两台物理机器是一回事。它将导致文件系统损坏。

### 使用分层块设备


在两个虚拟机之间共享文件系统的方法是使用 ubd 块驱动的写时复制（COW）分层
能力。任何更改过的块都存储在私有的 COW 文件中，而读取则来自任一设备——
如果请求的块在私有设备中有效，则来自私有设备，否则来自共享设备。使用这种
方案，绝大多数未更改的数据在任意数量的虚拟机之间共享，每个虚拟机都有一个小
得多的文件包含它所做的更改。对于从大型根文件系统启动大量 UML 的情况，这
可以节省巨大的磁盘空间。

共享文件系统数据也有助于性能，因为宿主机将能够使用少得多的内存来缓存共享
数据，因此 UML 的磁盘请求将从宿主机的内存而非其磁盘提供服务。在多路 NUMA
机器上这样做有一个重大注意事项。在此类硬件上，运行许多共享主映像和 COW
更改的 UML 实例可能会因过多的跨路（socket）流量而引发问题（如 NMIs）。

如果你在这样的高端硬件上运行 UML，请确保使用 `taskset` 命令将 UML 绑定到
位于同一路（socket）上的一组逻辑 CPU，或查看"调优"一节。

要向现有的块设备文件添加写时复制层，只需：

```
   ubd0=root_fs_cow,root_fs_debian_22
```

其中 `root_fs_cow` 是私有的 COW 文件，`root_fs_debian_22` 是现有的共享
文件系统。COW 文件不需要存在。如果不存在，驱动将创建并初始化它。

### 磁盘使用


UML 具有 TRIM 支持，它将把其磁盘映像文件中任何未使用的空间归还给底层
操作系统。使用 ls -ls 或 du 来验证实际文件大小很重要。

### COW 的有效性。


对主映像的任何更改都会使所有 COW 文件失效。如果发生这种情况，UML 将**不会**
自动删除任何 COW 文件，并且会拒绝启动。在这种情况下，唯一的解决方法是恢复
旧映像（包括其最后修改的时间戳），或者删除所有 COW 文件（这将导致它们被
重新创建）。COW 文件中的任何更改都会丢失。

### Cows can moo - uml_moo：将 COW 文件与其后端文件合并


根据你使用 UML 和 COW 设备的方式，可能建议不时将 COW 文件中的更改合并到
后端文件中。

```
   uml_moo COW_file new_backing_file
```

无需指定后端文件，因为该信息已经在 COW 文件头中。如果你很谨慎，可以启动
新的合并文件，如果你满意，就把它移到旧的备份文件之上。

`uml_moo` 默认会创建一个新的后端文件作为安全措施。它还有一个破坏性合并
选项，会将 COW 文件直接合并到其当前的后端文件中。这真正只在后端文件只
关联一个 COW 文件时才可用。如果有一个后端文件关联了多个 COW，对其中的一个
做 -d 合并会使所有其他 COW 失效。不过，如果你磁盘空间紧张，这很方便，而且
它也应该比非破坏性合并明显更快。

`uml_moo` 随 UML 发行版软件包一起安装，并作为 UML 工具的一部分提供。

## 宿主机文件访问


如果你想从 UML 内部访问宿主机上的文件，可以将其视为一台独立的机器，要么
从宿主机 nfs 挂载目录，要么用 scp 将文件复制到虚拟机中。然而，由于 UML
运行在宿主机上，它可以像任何其他进程一样访问那些文件，并使它们在虚拟机内
可用，而无需使用网络。这可以通过 hostfs 虚拟文件系统实现。使用它，你可以
将宿主机目录挂载到 UML 文件系统中，并像在宿主机上一样访问其中包含的文件。

**安全警告**

对 UML 映像不带任何参数地使用 Hostfs 将允许该映像挂载宿主文件系统任何部分
并写入其中。运行 UML 时，始终将 hostfs 限制在一个特定的"无害"目录（例如
`/var/tmp`）内。如果 UML 以 root 身份运行，这一点尤为重要。

### 使用 hostfs


首先，确保 hostfs 在虚拟内部可用：

```
   # cat /proc/filesystems
```

`hostfs` 应当被列出。如果没有，要么重新构建内核将 hostfs 编译进去，要么
确保 hostfs 被构建为模块并在虚拟机内部可用，然后 insmod 它。


```
   # mount none /mnt/host -t hostfs
```

将把宿主机的 `/` 挂载到虚拟机的 `/mnt/host` 上。如果你不想挂载宿主机根
目录，那么你可以：

```
   # mount none /mnt/home -t hostfs -o /home
```

将把宿主机的 /home 挂载到虚拟机的 /mnt/home 上。

### hostfs 作为根文件系统


可以使用 hostfs 从宿主机上的一个目录层次结构启动，而不是使用文件中的标准
文件系统。首先，你需要那个层次结构。最简单的方法是回环挂载：

```
   #  mount root_fs uml_root_dir -o loop
```

你需要将 `etc/fstab` 中 `/` 的文件系统类型改为：

```
   /dev/ubd/0       /        hostfs      defaults          1   1
```

然后你需要将该目录中所有文件的所有者改为你自己：

```
   #  find . -uid 0 -exec chown jdike {} \;
```

接下来，确保你的 UML 内核将 hostfs 编译进内核，而非作为：

```
   ubd0=/path/to/uml/root/directory
```

之后 UML 应当像平常一样启动。

### Hostfs 注意事项


Hostfs 不支持跟踪宿主机（UML 之外）上宿主机文件系统的更改。结果，如果一个
文件在 UML 不知情的情况下被更改，UML 将不会知道，它自己内存中的文件缓存
可能已损坏。虽然有可能修复这一点，但目前这不是正在着手处理的工作。

## 调优 UML


UML 目前严格是单处理器的。不过它会启动若干线程来处理各种功能。

UBD 驱动、SIGIO 和 MMU 仿真就是这样做的。如果系统空闲，这些线程将被迁移
到 SMP 宿主机上的其他处理器。遗憾的是，由于核心之间所有的缓存/内存同步
流量，这通常会导致更低的性能。因此，UML 通常受益于被固定到单个 CPU 上，
尤其是在大型系统上。在某些基准测试上，这可能导致 5 倍或更高的性能差异。

类似地，在大型多节点 NUMA 系统上，如果 UML 的所有内存都从它将运行的同一
NUMA 节点分配，它将受益。操作系统默认**不会**这样做。为了做到这一点，系统
管理员需要创建一个绑定到特定节点的合适 tmpfs ramdisk，并通过在 TMP 或 TEMP
环境变量中指定它，将其用作 UML RAM 分配的源。UML 会查看 `TMPDIR`、`TMP`
或 `TEMP` 的值。如果失败，它会查找挂载在 `/dev/shm` 下的 shmfs。如果一切都
失败，则使用：

```
   mount -t tmpfs -ompol=bind:X none /mnt/tmpfs-nodeX
   TEMP=/mnt/tmpfs-nodeX taskset -cX linux options options options..
```

######## 为 UML 做贡献以及使用 UML 开发


UML 是开发新 Linux 内核概念——文件系统、设备、虚拟化等的绝佳平台。它提供
了无与伦比的机会来创建和测试它们，而不受仿真特定硬件的约束。

示例——想试试 Linux 在拥有 4096 个"正规的"网络设备时会如何工作？

对 UML 来说不是问题。同时，这是其他虚拟化软件包难以做到的——它们受到
所尝试仿真的硬件总线上允许的设备数量的约束（例如 qemu 中 PCI 总线上 16 个）。

如果你有可以贡献的东西，例如补丁、bug 修复、新功能，请将其发送到
`linux-um@lists.infradead.org`。

请遵循所有标准的 Linux 补丁指南，例如抄送相关维护者，并对你的补丁运行
`./scripts/checkpatch.pl`。更多细节请参阅
`Documentation/process/submitting-patches.rst`

注意——该邮件列表不接受 HTML 或附件，所有邮件必须格式化为纯文本。

开发总是与调试携手并进。首先，你总是可以在 gdb 下运行 UML，后面会有一整节
介绍如何做到这一点。然而，这并不是调试 Linux 内核的唯一方法。通常，添加
跟踪语句和/或使用 UML 特定的方法（例如 ptrace 跟踪 UML 内核进程）信息量
要大得多。

## 跟踪 UML


运行时，UML 由一个主内核线程和若干辅助线程组成。对跟踪而言感兴趣的并非
那些已经由 UML 作为其 MMU 仿真的一部分 ptrace 过的线程。

这些通常是 ps 显示中前三个可见的线程。PID 编号最低且使用 CPU 最多的通常
是内核线程。其他线程是磁盘（ubd）设备辅助线程和 SIGIO 辅助线程。

```
   host$ strace -p 16566
   --- SIGIO {si_signo=SIGIO, si_code=POLL_IN, si_band=65} ---
   epoll_wait(4, [{EPOLLIN, {u32=3721159424, u64=3721159424}}], 64, 0) = 1
   epoll_wait(4, [], 64, 0)                = 0
   rt_sigreturn({mask=[PIPE]})             = 16967
   ptrace(PTRACE_GETREGS, 16967, NULL, 0xd5f34f38) = 0
   ptrace(PTRACE_GETREGSET, 16967, NT_X86_XSTATE, [{iov_base=0xd5f35010, iov_len=832}]) = 0
   ptrace(PTRACE_GETSIGINFO, 16967, NULL, {si_signo=SIGTRAP, si_code=0x85, si_pid=16967, si_uid=0}) = 0
   ptrace(PTRACE_SETREGS, 16967, NULL, 0xd5f34f38) = 0
   ptrace(PTRACE_SETREGSET, 16967, NT_X86_XSTATE, [{iov_base=0xd5f35010, iov_len=2696}]) = 0
   ptrace(PTRACE_SYSEMU, 16967, NULL, 0)   = 0
   --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_TRAPPED, si_pid=16967, si_uid=0, si_status=SIGTRAP, si_utime=65, si_stime=89} ---
   wait4(16967, [{WIFSTOPPED(s) && WSTOPSIG(s) == SIGTRAP | 0x80}], WSTOPPED|__WALL, NULL) = 16967
   ptrace(PTRACE_GETREGS, 16967, NULL, 0xd5f34f38) = 0
   ptrace(PTRACE_GETREGSET, 16967, NT_X86_XSTATE, [{iov_base=0xd5f35010, iov_len=832}]) = 0
   ptrace(PTRACE_GETSIGINFO, 16967, NULL, {si_signo=SIGTRAP, si_code=0x85, si_pid=16967, si_uid=0}) = 0
   timer_settime(0, 0, {it_interval={tv_sec=0, tv_nsec=0}, it_value={tv_sec=0, tv_nsec=2830912}}, NULL) = 0
   getpid()                                = 16566
   clock_nanosleep(CLOCK_MONOTONIC, 0, {tv_sec=1, tv_nsec=0}, NULL) = ? ERESTART_RESTARTBLOCK (Interrupted by signal)
   --- SIGALRM {si_signo=SIGALRM, si_code=SI_TIMER, si_timerid=0, si_overrun=0, si_value={int=1631716592, ptr=0x614204f0}} ---
   rt_sigreturn({mask=[PIPE]})             = -1 EINTR (Interrupted system call)
```

这是一个大多空闲的 UML 实例的典型画面。

- UML 中断控制器使用 epoll——这是 UML 在等待 IO 中断：

   epoll_wait(4, [{EPOLLIN, {u32=3721159424, u64=3721159424}}], 64, 0) = 1

- 一系列 ptrace 调用是 MMU 仿真和运行 UML 用户空间的一部分。
- `timer_settime` 是 UML 高分辨率定时器子系统的一部分，它将 UML 内部的
  定时器请求映射到宿主机的的分辨率定时器上。
- `clock_nanosleep` 是 UML 进入空闲（类似于 PC 执行 ACPI 空闲的方式）。

如你所见，即使空闲时 UML 也会产生相当多的输出。在观察 IO 时，这些输出可能
非常有用。它显示了实际的 IO 调用、它们的参数和返回值。

## 内核调试


你现在可以在 gdb 下运行 UML，尽管它不一定会乐意被在 gdb 下启动。如果你在
追踪一个运行时 bug，将 gdb 附加到一个正在运行的 UML 实例并让 UML 继续运行
要好得多。

```
   # gdb -p 16566
```

这将停止 UML 实例，因此你必须在 GDB 命令行中输入 `cont` 以请求它继续。把
这个做成一个 gdb 脚本并将其作为参数传给 gdb 可能是个好主意。

## 开发设备驱动


几乎所有 UML 驱动都是单体的。虽然可以将 UML 驱动构建为内核模块，但这会将
可能的功能限制为仅在内核内且非 UML 特定的。原因在于，为了真正利用 UML，
需要编写一段用户空间代码，将驱动概念映射到实际的用户空间宿主机调用上。

这构成了驱动所谓的"user"部分。虽然它可以重用很多内核概念，但它通常只是
另一段用户空间代码。这部分需要一些匹配的"kernel"代码，它驻留在 UML 映像
内部并实现 Linux 内核部分。

**注意："kernel"和"user"交互的方式几乎没有什么限制**。

UML 没有严格定义的内核到宿主机的 API。它不试图仿真特定的架构或总线。UML 的
"kernel"和"user"可以共享内存、代码，并按软件开发者设想的任何设计进行交互。
唯一的限制纯粹是技术性的。由于许多函数和变量具有相同的名称，开发者应该
小心他们所引用的 include 和库。

结果，许多用户空间代码由简单的包装器组成。例如 `os_close_file()` 只是
`close()` 的一个包装器，它确保用户空间函数 close 不会与内核部分中同名的
函数冲突。

## 使用 UML 作为测试平台


UML 是设备驱动开发的绝佳测试平台。与大多数事情一样，UML 是"可能需要用户
自己动手组装"。构建其仿真环境由用户负责。UML 目前只提供内核基础设施。

该基础设施的一部分是加载和解析 fdt 设备树 blob 的能力，如在 Arm 或 Open
Firmware 平台上使用的那样。它们作为内核命令行的可选额外参数提供：

```
    dtb=filename
```

设备树在启动时加载和解析，并可供查询它的驱动访问。目前这一功能仅用于
开发目的。UML 自身的设备不查询设备树。

### 安全考虑


驱动或任何新功能默认应当不接受任意文件名、bpf 代码或其他能够影响宿主机的
参数（从 UML 实例内部）。例如，在 UML 命令行上指定驱动与宿主机之间用于 IPC
通信的套接字在安全性上是可以的。将其作为可加载模块参数是不行的。

如果某个特定应用需要此类功能（例如为 raw 套接字网络传输加载 BPF"固件"），
它默认应当关闭，并应在启动时作为命令行参数显式开启。

即便如此，UML 与宿主机之间的隔离级别相对较弱。如果允许 UML 用户空间加载
任意内核驱动，攻击者可以利用这一点逃出 UML。因此，如果在生产应用中使用
UML，建议所有模块都在启动时加载，之后禁用内核模块加载。
