
## Virtual Routing and Forwarding（虚拟路由与转发，VRF）


## The VRF Device（VRF 设备）


VRF 设备配合 ip 规则，可以在 Linux 网络栈中创建虚拟路由与转发域
（具体即 VRF、VRF-lite）。一个典型的使用场景是多租户问题：每个租户
都拥有各自独立的路由表，并且至少需要不同的默认网关。

进程可以通过将套接字绑定到 VRF 设备，从而做到“VRF 感知”。通过该套接字
收发的报文随后会使用与 VRF 设备相关联的路由表。VRF 设备实现的一个重要
特性是它只影响第 3 层及以上，因此 L2 工具（例如 LLDP）不会受到影响
（即无需在每个 VRF 中分别运行）。该设计还允许使用更高优先级的 ip 规则
（基于策略的路由，PBR）来优先于 VRF 设备规则，从而按需引导特定流量。

此外，VRF 设备支持将 VRF 嵌套在命名空间内。例如，网络命名空间在设备层
提供网络接口的隔离，命名空间内接口上的 VLAN 提供 L2 隔离，而 VRF 设备
则提供 L3 隔离。

### Design（设计）


VRF 设备创建时会关联一张路由表。网络接口被从属于某个 VRF 设备后，VRF
就利用它来引导入向与出向报文：

```
	 +-----------------------------+
	 |           vrf-blue          |  ===> route table 10
	 +-----------------------------+
	    |        |            |
	 +------+ +------+     +-------------+
	 | eth1 | | eth2 | ... |    bond1    |
	 +------+ +------+     +-------------+
				  |       |
			      +------+ +------+
			      | eth8 | | eth9 |
			      +------+ +------+

```
在被从属于 VRF 设备的接口上收到的报文，会在 IPv4 与 IPv6 协议栈中被切换到
VRF 设备，从而给人一种报文流经 VRF 设备的印象。类似地，在出站方向上，路由
规则会在报文真正发出前将其送往 VRF 设备驱动。这使得在 VRF 设备上使用
tcpdump 即可捕获进出整个 VRF 的所有报文\ [^1^]_。同样地，可以利用 VRF 设备
应用 netfilter\ [^2^]_ 与 tc 规则，从而指定适用于整个 VRF 域的规则。

       注意：tcpdump 目前看不到这些报文。该限制将在未来的版本中予以解决。

       对于入向，INPUT 与 PREROUTING 规则的 skb->dev 被设为 VRF 设备；
       对于出向，POSTROUTING 与 OUTPUT 规则可以使用 VRF 设备或真实的
       出向设备进行编写。

### Setup（配置）


1. 创建 VRF 设备，并关联一张 FIB 表：

```
	ip link add vrf-blue type vrf table 10
	ip link set dev vrf-blue up

```
2. 一条 l3mdev FIB 规则将查找引导到与该设备相关联的表。单个 l3mdev 规则
   足以服务所有 VRF。当首个设备创建时，VRF 设备会为 IPv4 与 IPv6 添加
   l3mdev 规则，默认优先级为 1000。用户如需可删除该规则，并以不同优先级
   重新添加，或安装按 VRF 划分的规则。

```
       ip ru add oif vrf-blue table 10
       ip ru add iif vrf-blue table 10

```
3. 为 VRF 添加 IPv4 与 IPv6 默认路由。例如，使用默认不可达路由作为兜底，
   确保任何路由协议都能覆盖它：

```
       ip route add table 10 unreachable default metric 4278198272

```
该较高的 metric 值确保默认不可达路由可被路由协议套件覆盖。FRRouting 将
内核 metric 解释为组合的管理距离（高字节）与优先级（低 3 字节），因此
上述 metric 等价于 [255/8192]。

4. 将网络接口从属于 VRF 设备：

```
       ip link set dev eth1 master vrf-blue

```
从属于 VRF 设备的本地与直连路由会自动移动到与该 VRF 设备相关联的表。
任何依赖于该从属设备的额外路由会被丢弃，需要在从属关系建立后重新插入
到 VRF FIB 表。

5. IPv6 sysctl 选项 keep_addr_on_down 可开启，以在 VRF 从属关系变化时
   保留 IPv6 全局地址：

```
       sysctl -w net.ipv6.conf.all.keep_addr_on_down=1

```
6. 向 VRF 表添加路由：

```
       ip route add table 10 ...

```
### Applications（应用）


要在 VRF 内工作的应用需要将其套接字绑定到 VRF 设备，可使用 setsockopt：

```
    setsockopt(sd, SOL_SOCKET, SO_BINDTODEVICE, dev, strlen(dev)+1);

```
或使用 cmsg 与 IP_PKTINFO 来指定输出设备。

默认情况下，未绑定套接字的端口绑定范围仅限于默认 VRF。也就是说，到达
从属于 l3mdev 的接口的报文不会被其匹配；而进程若绑定到某个 l3mdev，则
可以绑定到同一端口。

运行在默认 VRF 上下文（即未绑定到任何 VRF 设备）中的 TCP 与 UDP 服务，
可通过开启以下选项在所有 VRF 域中工作：

```
    sysctl -w net.ipv4.tcp_l3mdev_accept=1
    sysctl -w net.ipv4.udp_l3mdev_accept=1

```
这些选项默认关闭，使得 VRF 中的套接字只被选中处理该 VRF 内的报文。RAW
套接字有类似选项，出于向后兼容默认开启，以便用 cmsg 与 IP_PKTINFO 指定
输出设备，但使用的是未绑定到对应 VRF 的套接字。例如，这允许老式 ping
实现在指定设备的情况下运行，而无需在 VRF 中执行。该选项可关闭，从而使
在 VRF 上下文中收到的报文只由绑定到 VRF 的 raw 套接字处理：

```
    sysctl -w net.ipv4.raw_l3mdev_accept=0

```
VRF 设备上的 netfilter 规则也可用于限制对运行在默认 VRF 上下文中的
服务的访问。

使用 VRF 感知应用（即同时创建 VRF 内外套接字的应用）配合
`net.ipv4.tcp_l3mdev_accept=1` 是可行的，但在某些情况下可能导致问题。
在该 sysctl 取值下，由哪个监听套接字来处理 VRF 流量连接是不确定的；
也就是说，既可能使用绑定到 VRF 的套接字，也可能使用未绑定的套接字来
接受来自 VRF 的新连接。如果为套接字配置了额外选项（例如 TCP MD5 密钥），
并期望 VRF 流量只由绑定到 VRF 的套接字处理（即 `net.ipv4.tcp_l3mdev_accept=0`
的情形），这种略显意外的行为就可能引发问题。最后提醒，无论选中哪个监听
套接字，已建立的套接字都会基于入向接口创建在对应的 VRF 中，如前文所述。

--------------------------------------------------------------------------------

## Using iproute2 for VRFs（使用 iproute2 管理 VRF）


iproute2 自 v4.7 起支持 vrf 关键字。出于向后兼容，本节在合适处列出两种
命令——带 vrf 关键字的形式与不带它的旧式写法。

1. Create a VRF（创建 VRF）

创建 VRF 设备：

```
       $ ip link add dev NAME type vrf table ID

```
自 v4.8 起，内核支持 l3mdev FIB 规则，单条规则即可覆盖所有 VRF。该
l3mdev 规则在首个设备创建时为 IPv4 与 IPv6 建立。

2. List VRFs（列出 VRF）

列出所有 VRF 设备：

```
       $ ip [-d] link show type vrf
	 NOTE: 需要 -d 选项才能显示表 id

```
例如：

```
       $ ip -d link show type vrf
       11: mgmt: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether 72:b3:ba:91:e2:24 brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 1 addrgenmode eui64
       12: red: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether b6:6f:6e:f6:da:73 brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 10 addrgenmode eui64
       13: blue: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether 36:62:e8:7d:bb:8c brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 66 addrgenmode eui64
       14: green: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether e6:28:b8:63:70:bb brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 81 addrgenmode eui64

```
或以简略输出：

```
       $ ip -br link show type vrf
       mgmt         UP             72:b3:ba:91:e2:24 <NOARP,MASTER,UP,LOWER_UP>
       red          UP             b6:6f:6e:f6:da:73 <NOARP,MASTER,UP,LOWER_UP>
       blue         UP             36:62:e8:7d:bb:8c <NOARP,MASTER,UP,LOWER_UP>
       green        UP             e6:28:b8:63:70:bb <NOARP,MASTER,UP,LOWER_UP>

```
3. Assign a Network Interface to a VRF（将网络接口分配给 VRF）

网络接口通过将网络设备从属于 VRF 来分配给 VRF：

```
       $ ip link set dev NAME master NAME

```
从属时，直连与本地路由会自动移动到与 VRF 设备相关联的表。

例如：

```
       $ ip link set dev eth0 master mgmt

```
4. Show Devices Assigned to a VRF（显示分配给 VRF 的设备）

要显示已分配给特定 VRF 的设备，可在 show 命令中加入 master 参数：

```
       $ ip link show vrf NAME
       $ ip link show master NAME

```
例如：

```
       $ ip link show vrf red
       3: eth1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:02 brd ff:ff:ff:ff:ff:ff
       4: eth2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:03 brd ff:ff:ff:ff:ff:ff
       7: eth5: <BROADCAST,MULTICAST> mtu 1500 qdisc noop master red state DOWN mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:06 brd ff:ff:ff:ff:ff:ff

```
或使用简略输出：

```
       $ ip -br link show vrf red
       eth1             UP             02:00:00:00:02:02 <BROADCAST,MULTICAST,UP,LOWER_UP>
       eth2             UP             02:00:00:00:02:03 <BROADCAST,MULTICAST,UP,LOWER_UP>
       eth5             DOWN           02:00:00:00:02:06 <BROADCAST,MULTICAST>

```
5. Show Neighbor Entries for a VRF（显示 VRF 的邻居表项）

要列出从属于 VRF 设备的设备相关联的邻居表项，可使用：

```
       $ ip [-6] neigh show vrf NAME
       $ ip [-6] neigh show master NAME

```
例如：

```
       $  ip neigh show vrf red
       10.2.1.254 dev eth1 lladdr a6:d9:c7:4f:06:23 REACHABLE
       10.2.2.254 dev eth2 lladdr 5e:54:01:6a:ee:80 REACHABLE

       $ ip -6 neigh show vrf red
       2002:1::64 dev eth1 lladdr a6:d9:c7:4f:06:23 REACHABLE

```
6. Show Addresses for a VRF（显示 VRF 的地址）

要显示与 VRF 相关联的接口地址，可在 show 命令中加入 master 参数：

```
       $ ip addr show vrf NAME
       $ ip addr show master NAME

```
例如：

```
	$ ip addr show vrf red
	3: eth1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP group default qlen 1000
	    link/ether 02:00:00:00:02:02 brd ff:ff:ff:ff:ff:ff
	    inet 10.2.1.2/24 brd 10.2.1.255 scope global eth1
	       valid_lft forever preferred_lft forever
	    inet6 2002:1::2/120 scope global
	       valid_lft forever preferred_lft forever
	    inet6 fe80::ff:fe00:202/64 scope link
	       valid_lft forever preferred_lft forever
	4: eth2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP group default qlen 1000
	    link/ether 02:00:00:00:02:03 brd ff:ff:ff:ff:ff:ff
	    inet 10.2.2.2/24 brd 10.2.2.255 scope global eth2
	       valid_lft forever preferred_lft forever
	    inet6 2002:2::2/120 scope global
	       valid_lft forever preferred_lft forever
	    inet6 fe80::ff:fe00:203/64 scope link
	       valid_lft forever preferred_lft forever
	7: eth5: <BROADCAST,MULTICAST> mtu 1500 qdisc noop master red state DOWN group default qlen 1000
	    link/ether 02:00:00:00:02:06 brd ff:ff:ff:ff:ff:ff

```
或以简略格式：

```
	$ ip -br addr show vrf red
	eth1             UP             10.2.1.2/24 2002:1::2/120 fe80::ff:fe00:202/64
	eth2             UP             10.2.2.2/24 2002:2::2/120 fe80::ff:fe00:203/64
	eth5             DOWN

```
7. Show Routes for a VRF（显示 VRF 的路由）

要显示 VRF 的路由，使用 ip 命令显示与 VRF 相关联的表：

```
       $ ip [-6] route show vrf NAME
       $ ip [-6] route show table ID

```
例如：

```
	$ ip route show vrf red
	unreachable default  metric 4278198272
	broadcast 10.2.1.0 dev eth1  proto kernel  scope link  src 10.2.1.2
	10.2.1.0/24 dev eth1  proto kernel  scope link  src 10.2.1.2
	local 10.2.1.2 dev eth1  proto kernel  scope host  src 10.2.1.2
	broadcast 10.2.1.255 dev eth1  proto kernel  scope link  src 10.2.1.2
	broadcast 10.2.2.0 dev eth2  proto kernel  scope link  src 10.2.2.2
	10.2.2.0/24 dev eth2  proto kernel  scope link  src 10.2.2.2
	local 10.2.2.2 dev eth2  proto kernel  scope host  src 10.2.2.2
	broadcast 10.2.2.255 dev eth2  proto kernel  scope link  src 10.2.2.2

	$ ip -6 route show vrf red
	local 2002:1:: dev lo  proto none  metric 0  pref medium
	local 2002:1::2 dev lo  proto none  metric 0  pref medium
	2002:1::/120 dev eth1  proto kernel  metric 256  pref medium
	local 2002:2:: dev lo  proto none  metric 0  pref medium
	local 2002:2::2 dev lo  proto none  metric 0  pref medium
	2002:2::/120 dev eth2  proto kernel  metric 256  pref medium
	local fe80:: dev lo  proto none  metric 0  pref medium
	local fe80:: dev lo  proto none  metric 0  pref medium
	local fe80::ff:fe00:202 dev lo  proto none  metric 0  pref medium
	local fe80::ff:fe00:203 dev lo  proto none  metric 0  pref medium
	fe80::/64 dev eth1  proto kernel  metric 256  pref medium
	fe80::/64 dev eth2  proto kernel  metric 256  pref medium
	ff00::/8 dev red  metric 256  pref medium
	ff00::/8 dev eth1  metric 256  pref medium
	ff00::/8 dev eth2  metric 256  pref medium
	unreachable default dev lo  metric 4278198272  error -101 pref medium

```
8. Route Lookup for a VRF（查询 VRF 的路由）

查询某地址在 VRF 中的路由：

```
       $ ip [-6] route get vrf NAME ADDRESS
       $ ip [-6] route get oif NAME ADDRESS

```
例如：

```
	$ ip route get 10.2.1.40 vrf red
	10.2.1.40 dev eth1  table red  src 10.2.1.2
	    cache

	$ ip -6 route get 2002:1::32 vrf red
	2002:1::32 from :: dev eth1  table red  proto kernel  src 2002:1::2  metric 256  pref medium

```
9. Removing Network Interface from a VRF（从 VRF 移除网络接口）

网络接口通过解除对 VRF 设备的从属关系来从 VRF 中移除：

```
       $ ip link set dev NAME nomaster

```
直连路由会被移回默认表，本地表项会被移动到本地表。

例如：

```
    $ ip link set dev eth0 nomaster

```
--------------------------------------------------------------------------------

以下是在 /etc/iproute2/rt_tables.d/vrf.conf 中定义自定义表，并用脚本
批量创建 VRF 的示例：

```
     cat >> /etc/iproute2/rt_tables.d/vrf.conf <<EOF
     1  mgmt
     10 red
     66 blue
     81 green
     EOF

     function vrf_create
     {
	 VRF=$1
	 TBID=$2

	 # create VRF device
	 ip link add ${VRF} type vrf table ${TBID}

	 if [ "${VRF}" != "mgmt" ]; then
	     ip route add table ${TBID} unreachable default metric 4278198272
	 fi
	 ip link set dev ${VRF} up
     }

     vrf_create mgmt 1
     ip link set dev eth0 master mgmt

     vrf_create red 10
     ip link set dev eth1 master red
     ip link set dev eth2 master red
     ip link set dev eth5 master red

     vrf_create blue 66
     ip link set dev eth3 master blue

     vrf_create green 81
     ip link set dev eth4 master green


     Interface addresses from /etc/network/interfaces:
     auto eth0
     iface eth0 inet static
	   address 10.0.0.2
	   netmask 255.255.255.0
	   gateway 10.0.0.254

     iface eth0 inet6 static
	   address 2000:1::2
	   netmask 120

     auto eth1
     iface eth1 inet static
	   address 10.2.1.2
	   netmask 255.255.255.0

     iface eth1 inet6 static
	   address 2002:1::2
	   netmask 120

     auto eth2
     iface eth2 inet static
	   address 10.2.2.2
	   netmask 255.255.255.0

     iface eth2 inet6 static
	   address 2002:2::2
	   netmask 120

     auto eth3
     iface eth3 inet static
	   address 10.2.3.2
	   netmask 255.255.255.0

     iface eth3 inet6 static
	   address 2002:3::2
	   netmask 120

     auto eth4
     iface eth4 inet static
	   address 10.2.4.2
	   netmask 255.255.255.0

     iface eth4 inet6 static
	   address 2002:4::2
	   netmask 120

```
