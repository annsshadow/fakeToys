
## 透明代理支持


此特性为当前内核添加了类似 Linux 2.2 的透明代理支持。要使用该特性，请在你的内核配置中
启用 socket 匹配与 TPROXY 目标。你还需要策略路由，因此请务必也启用它。

从 Linux 4.18 起，nf_tables 中也提供了透明代理支持。

## 1. 让非本地套接字工作


其思路是：你通过策略路由识别目的地址匹配本地某个地址的数据包，从而让这些数据包
```

    # iptables -t mangle -N DIVERT
    # iptables -t mangle -A PREROUTING -p tcp -m socket --transparent -j DIVERT
    # iptables -t mangle -A DIVERT -j MARK --set-mark 1
    # iptables -t mangle -A DIVERT -j ACCEPT

```
```

    # nft add table filter
    # nft add chain filter divert "{ type filter hook prerouting priority -150; }"
    # nft add rule filter divert meta l4proto tcp socket transparent 1 meta mark set 1 accept

```
然后通过策略路由匹配该值，使那些数据包
```

    # ip rule add fwmark 1 lookup 100
    # ip route add local 0.0.0.0/0 dev lo table 100

```
由于 IPv4 路由输出代码的某些限制，你将不得不修改你的应用程序，以允许它_从_非本地 IP
地址发送数据报。你只需启用（SOL_IP, IP_TRANSPARENT）套接字选项
```

    fd = socket(AF_INET, SOCK_STREAM, 0);
    /* - 8< -*/
    int value = 1;
    setsockopt(fd, SOL_IP, IP_TRANSPARENT, &value, sizeof(value));
    /* - 8< -*/
    name.sin_family = AF_INET;
    name.sin_port = htons(0xCAFE);
    name.sin_addr.s_addr = htonl(0xDEADBEEF);
    bind(fd, &name, sizeof(name));

```
netcat 的一个简单补丁可在此处获取：
http://people.netfilter.org/hidden/tproxy/netcat-ip_transparent-support.patch

## 2. 重定向流量


透明代理通常涉及在路由器上“拦截”流量。这通常通过 iptables 的 REDIRECT 目标完成；然而，
该方法存在严重局限。其中一个主要问题是，它实际上会修改数据包以改变目的地址——这在某些
情况下可能不可接受。（例如想想代理 UDP：你将无法获知原始目的地址。即便对于 TCP，获取
原始目的地址也存在竞争条件。）

'TPROXY' 目标提供了类似的功能，且不依赖 NAT。只需
```

    # iptables -t mangle -A PREROUTING -p tcp --dport 80 -j TPROXY \
      --tproxy-mark 0x1/0x1 --on-port 50080

```
```

    # nft add rule filter divert tcp dport 80 tproxy to :50080 meta mark set 1 accept

```
注意，要使其工作，你必须修改代理，为监听套接字启用（SOL_IP, IP_TRANSPARENT）。

作为示例实现，tcprdr 可在此处获取：
https://git.breakpoint.cc/cgit/fw/tcprdr.git/
该工具由 Florian Westphal 编写，并在 nf_tables 实现期间用于测试。

## 3. Iptables 与 nf_tables 扩展


要使用 tproxy，你需要为 iptables 编译以下模块：

 - NETFILTER_XT_MATCH_SOCKET
 - NETFILTER_XT_TARGET_TPROXY

或为 nf_tables 编译以下模块：

 - NFT_SOCKET
 - NFT_TPROXY

## 4. 应用程序支持


### 4.1. Squid


Squid 3.HEAD 已内置支持。要使用它，请将 '--enable-linux-netfilter' 传给 configure，
并在你通过 TPROXY iptables 目标重定向流量到的 HTTP 监听器上设置 'tproxy' 选项。

更多信息请查阅 Squid wiki 上的以下页面：http://wiki.squid-cache.org/Features/Tproxy4
