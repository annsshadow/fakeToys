
## batman-adv


Batman advanced 是一种不再基IP 的无线网络新方法。与 batman 守护进程使用 UDP 包交换信并设置路由表不同，batman-advanced 仅运行在 ISO/OSI 第二层，并使用、路由（或更准确地说桥接）以太网帧。它模拟所有参与节点的虚拟网络交换机。因此所有节点看起来都是链路本地的，
于是所有更高层的协议都不会受网络内部任何变化的影响。你几乎可以batman advanced 之上运行
任何协议，显著的例子有：IPv4、IPv6、DHCP、IPX
Batman advanced 被实现为 Linux 内核驱动，以将开销降到最低。它不依赖任何（其它）网络驱动，
可用wifi 以及以太LAN、VPN 等……（任何具有以太网风格第二层的介质）

## 配置


```

  $ insmod batman-adv.ko

```
模块现在正在等待激活。你必须添加一batman-adv 可以在其上运行的接口。batman-adv 网格接口
可以使用以下命令创建
```

  $ ip link add name bat0 type batadv

```
```

  $ ip link set dev eth0 master bat0

```
对所有希望添加的接口重复此步骤。现batman-adv 开始在这些接口上使广播
```

  $ ip link set dev eth0 nomaster

```
```

  batctl -m bat0 interface create
  batctl -m bat0 interface add -M eth0

```
```

  batctl -m bat0 interface del -M eth0
  batctl -m bat0 interface destroy

```
每个 batadv 网格接口、vlan hardif 都有额外的设置，可以使用 batctl 修改。关于此的详细信可在这份手册中找到
例如，你可以检查当前的源节点间隔（origination interval，以毫秒为单位的值，决定 batman-adv
发送其广播的频率）
```

  $ batctl -M bat0 orig_interval
  1000

```
```

  $ batctl -M bat0 orig_interval 3000

```
在高度移动的场景中，你可能希望把源节点间隔调低。这将使网格对拓扑变化更敏感，但也会增加开销
关于当前状态的信息可以通过 batadv 通用 netlink 系列访问。batctl 通过其调试表（debug tables子命令提供了一个人类可读的版本

## 使用


要使用你新创建的网格，batman advanced 提供了一个新的接"bat0"，从此你应该使用它。所有添加到
batman advanced 的接口都不再相关，因batman 会为你处理它们。基本上，人们通过使用 batman
接口来“交出”数据，batman 会确保它到达目的地
"bat0" 接口可以像任何其它常规接口一样使用。它需要一IP 地址，可以是静态配置，也可以是
动态获取（通过使用
```

  NodeA: ip link set up dev bat0
  NodeA: ip addr add 192.168.0.1/24 dev bat0

  NodeB: ip link set up dev bat0
  NodeB: ip addr add 192.168.0.2/24 dev bat0
  NodeB: ping 192.168.0.1

```
注意：为避免问题，请移除之前分配```

  $ ip addr flush dev eth0


```
## 日志/调试


所有错误消息、警告和信息消息都被发送到内核日志。根据你操作系统的发行版，可以通过多种方式读取。尝试使用这些命令：`dmesg`、`logread`，或查看文件 `/var/log/kern.log` `/var/log/syslog`。所batman-adv 消息
```

  $ dmesg | grep batman-adv

```
在研究网格网络的问题时，有时需要查看更详细的调试消息。这必须在编batman-adv 模块时启用当把 batman-adv 作为内核的一部分构建时，使用 "make menuconfig" 并启用选项
`B.A.T.M.A.N. debugging`（`CONFIG_BATMAN_ADV_DEBUG=y`）
```

  $ trace-cmd stream -e batadv:batadv_dbg

```
额外调试输出默认是关闭的。它可以```

  $ batctl -m bat0 loglevel routes tt

```
时启用，将为路由和转换表（translation table）条目变化时启用调试消息
进入和离开 batman-adv 的不同类型数据包的计数器
```

  $ ethtool --statistics bat0


```
## batctl


由于 batman advanced 运行在第二层，参与虚拟交换机的所有主机对所有第二层之上的协议完全透明因此常用的诊断工具无法按预期工作。为了克服这些问题，创建batctl。目batctl 包含 pingtraceroute、tcpdump 以及到内核模块设置的接口
更多信息请参阅手册页（`man batctl`）
batctl 可在 https://www.open-mesh.org/ 获取

## 联系方式


请向我们发送评论、经验、问题，任何内容都可:)

IRC:
  #batadv on ircs://irc.hackint.org/
Mailing-list:
  b.a.t.m.a.n@lists.open-mesh.org（可选订阅地址  https://lists.open-mesh.org/mailman3/postorius/lists/b.a.t.m.a.n.lists.open-mesh.org/
你也可以联系作者：

- Marek Lindner <marek.lindner@mailbox.org>
- Simon Wunderlich <sw@simonwunderlich.de>
