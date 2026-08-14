## /proc/sys/net/ 文档


版权


Copyright (c) 1999

 - Terrehon Bowden <terrehon@pacbell.net>
 - Bodo Bauer <bb@ricochet.net>

Copyright (c) 2000

 - Jorge Nerin <comandante@zaralinux.com>

Copyright (c) 2009

 - Shen Feng <shen@cn.fujitsu.com>

有关一般信息与法律声明，请参阅 index.rst。

------------------------------------------------------------------------------

本文件包含 /proc/sys/net 中 sysctl 文件的文档。

内核网络部分的接口位于 /proc/sys/net。下表显示了所有可能的子目录。根据内核配置的不同，你也许只能看到其中一部分。


表：/proc/sys/net 中的子目录

 ========= =================== = ========== ===================
 Directory Content               Directory  Content
 ========= =================== = ========== ===================
 802       E802 protocol         mptcp      Multipath TCP
 appletalk Appletalk protocol    netfilter  Network Filter
 ax25      AX25                  netrom     NET/ROM
 bridge    Bridging              rose       X.25 PLP layer
 core      General parameter     tipc       TIPC
 ethernet  Ethernet protocol     unix       Unix domain sockets
 ipv4      IP version 4          vsock      VSOCK sockets
 ipv6      IP version 6          x25        X.25 protocol
 ========= =================== = ========== ===================

## 1. /proc/sys/net/core - 网络核心选项


### bpf_jit_enable


该功能启用 BPF 即时（Just in Time，JIT）编译器。BPF 是一种灵活且高效的基础设施，允许在各个钩子（hook）点执行字节码。它被用于若干 Linux 内核子系统，例如网络（如 XDP、tc）、追踪（如 kprobes、uprobes、tracepoints）和安全（如 seccomp）。LLVM 有一个 BPF 后端，可以将受限的 C 编译为一系列 BPF 指令。通过 bpf(2) 加载程序并经内核中的验证器（verifier）检查后，JIT 会将这些 BPF proglet 翻译为本机 CPU 指令。JIT 有两种类型，较新的 eBPF JIT 当前在以下架构上受支持：

  - x86_64
  - x86_32
  - arm64
  - arm32
  - ppc64
  - ppc32
  - sparc64
  - mips64
  - s390x
  - riscv64
  - riscv32
  - loongarch64
  - arc

较旧的 cBPF JIT 在以下架构上受支持：

  - mips
  - sparc

eBPF JIT 是 cBPF JIT 的超集，意味着内核会将 cBPF 指令迁移为 eBPF 指令，然后透明地对其做 JIT 编译。较旧的 cBPF JIT 只能翻译 tcpdump 过滤器、seccomp 规则等，而不能翻译上文提到的通过 bpf(2) 加载的 eBPF 程序。

取值：

 - 0 - 禁用 JIT（默认值）
 - 1 - 启用 JIT
 - 2 - 启用 JIT，并要求编译器在内核日志中发出跟踪信息（trace）。

### bpf_jit_harden


该功能对 BPF JIT 编译器启用加固（hardening）。受支持的是 eBPF JIT 后端。启用加固会以性能为代价，但可以缓解 JIT spraying 攻击。

取值：

 - 0 - 禁用 JIT 加固（默认值）
 - 1 - 仅对无特权用户启用 JIT 加固
 - 2 - 对所有用户启用 JIT 加固

其中“特权用户”在此上下文中指在其根用户命名空间（root user name space）中拥有 CAP_BPF 或 CAP_SYS_ADMIN 的进程。

### bpf_jit_kallsyms


当 BPF JIT 编译器启用后，编译出的映像对内核而言是未知地址，意味着它们既不会出现在跟踪信息中，也不会出现在 /proc/kallsyms 中。此功能导出这些地址，可用于调试/追踪。如果启用了 bpf_jit_harden，此功能将被禁用。

取值：

 - 0 - 禁用 JIT kallsyms 导出（默认值）
 - 1 - 仅对特权用户导出 JIT kallsyms

### bpf_jit_limit


该功能对 BPF JIT 编译器的内存分配强制执行一个全局上限，以便在其被超过后拒绝无特权的 JIT 请求。bpf_jit_limit 包含该全局上限的值（以字节为单位）。

### dev_weight


内核在单个 NAPI 中断中能够处理的包的最大数量，它是一个每 CPU 变量。对于支持 LRO 或 GRO_HW 的驱动，一个硬件聚合的包在此上下文中被计为一个包。

默认值：64

### dev_weight_rx_bias


RPS（如 RFS、aRFS）处理会与驱动注册的 NAPI poll 函数竞争每个软中断周期的 netdev_budget。此参数影响在 RX 软中断周期中，所配置的 netdev_budget 里有多少比例被用于基于 RPS 的包处理。它进一步旨在使当前的 dev_weight 能够适配网络栈 RX/TX 侧不对称 CPU 需求的情况。（参见 dev_weight_tx_bias）它在每 CPU 基础上生效。其确定基于 dev_weight，并按乘法计算（dev_weight * dev_weight_rx_bias）。

默认值：1

### dev_weight_tx_bias


缩放一个 TX 软中断周期内能够处理的包的最大数量。在每 CPU 基础上生效。允许根据不对称的网络栈处理需求缩放当前的 dev_weight。注意避免使 TX 软中断处理成为 CPU 消耗大户。

计算基于 dev_weight（dev_weight * dev_weight_tx_bias）。

默认值：1

### default_qdisc


用于网络设备的默认排队规则（queuing discipline）。这允许用另一种规则覆盖默认的 pfifo_fast。由于默认排队规则是在不附加额外参数的情况下创建的，因此最适合用于那些无需配置即可良好工作的排队规则，例如随机公平队列（sfq）、CoDel（codel）或公平队列 CoDel（fq_codel）。不要使用像分层令牌桶（Hierarchical Token Bucket）或赤字轮询（Deficit Round Robin）这样需要设置类别和带宽的排队规则。注意，物理多队列接口仍然使用 mq 作为根 qdisc，而 mq 又使用此默认值作为其叶子。虚拟设备（例如 lo 或 veth）会忽略此设置，转而默认使用 noqueue。

默认值：pfifo_fast

### busy_read


用于 socket 读取的低延迟忙轮询（busy poll）超时。（需要 CONFIG_NET_RX_BUSY_POLL）在设备队列上忙循环等待数据包的大致时间，以微秒为单位。它设置 SO_BUSY_POLL socket 选项的默认值。可以通过设置 socket 选项 SO_BUSY_POLL 来设置或覆盖每个 socket 的值，这也是推荐的启用方式。如果你需要通过 sysctl 全局启用该功能，建议使用值 50。

会增加功耗。

默认值：0（关闭）

### busy_poll


用于 poll 和 select 的低延迟忙轮询超时。（需要 CONFIG_NET_RX_BUSY_POLL）忙循环等待事件的大致时间，以微秒为单位。推荐值取决于你轮询的 socket 数量。对于若干 socket 用 50，对于数百个用 100。再多的话你可能想使用 epoll。注意只有设置了 SO_BUSY_POLL 的 socket 才会被忙轮询，因此你要么在这些 socket 上有选择地设置 SO_BUSY_POLL，要么全局设置 sysctl.net.busy_read。

会增加功耗。

默认值：0（关闭）

### mem_pcpu_rsv


每 CPU 预留的转发分配（forward alloc）缓存大小，以页为单位。默认每 CPU 1MB。

### bypass_prot_mem


跳过将 socket 缓冲区计入由 net.ipv4.tcp_mem、net.ipv4.udp_mem 等控制的全局每协议内存统计。

默认值：0（关闭）

### rmem_default


socket 接收缓冲区的默认设置（以字节为单位）。

### rmem_max


接收 socket 缓冲区的最大大小（以字节为单位）。

默认值：4194304

### rps_default_mask


在新创建的网络设备上使用的默认 RPS CPU 掩码。空掩码表示默认禁用 RPS。

### tstamp_allow_data


允许进程接收与原始包内容一起回环（loop）的发送时间戳。如果禁用，来自无特权进程的发送时间戳请求会被丢弃，除非设置了 socket 选项 SOF_TIMESTAMPING_OPT_TSONLY。

默认值：1（开启）


### wmem_default


socket 发送缓冲区的默认设置（以字节为单位）。

### wmem_max


发送 socket 缓冲区的最大大小（以字节为单位）。

默认值：4194304

### message_burst 与 message_cost


这些参数用于限制从网络代码写入内核日志的警告消息。它们强制实施一个速率限制，使得拒绝服务（denial-of-service）攻击无法得逞。较大的 message_cost 因子会导致写入的消息更少。message_burst 控制消息何时被丢弃。默认设置将警告消息限制为每五秒一条。

### warnings


此 sysctl 现已不再使用。

它曾用于控制来自网络栈的控制台消息，这些消息因网络问题（如重复地址或错误校验和）而产生。

这些消息现在以 KERN_DEBUG 级别发出，通常可以通过 dynamic_debug 设施启用和控制。

### netdev_budget


在一个轮询周期（NAPI poll）中从所有接口取走的包的最大数量。在一个轮询周期中，注册到轮询的接口以轮询（round-robin）方式被探测。此外，即便 netdev_budget 尚未耗尽，一个轮询周期也不得超过 netdev_budget_usecs 微秒。

### netdev_budget_usecs


一个 NAPI 轮询周期中的最大微秒数。当轮询周期中 netdev_budget_usecs 已流逝，或已处理的包数量达到 netdev_budget 时，轮询将退出。

### netdev_max_backlog


当接口接收数据包的速度快于内核处理它们的速度时，在输入侧排队的包的最大数量。

### qdisc_max_burst


在到达 qdisc 之前可以临时存储的包的最大数量。

默认值：1000

### netdev_rss_key


启用了 RSS（Receive Side Scaling）的驱动使用一个随机生成的主机密钥（host key）。

某些用户空间可能需要在驱动尚未提供 ethtool -x 支持的情况下获取其内容。

```

  myhost:~# cat /proc/sys/net/core/netdev_rss_key
  84:50:f4:00:a8:15:d1:a7:e9:7f:1d:60:35:c7:47:25:42:97:74:ca:56:bb:b6:a1:d8: ... (256 bytes total)

```
如果从来没有驱动调用过 netdev_rss_key_fill() 函数，文件包含全部为 nul 的字节。

注意：
  /proc/sys/net/core/netdev_rss_key 包含 256 字节的密钥，
  但许多驱动只使用其中的 40 或 52 字节。

```

  myhost:~# ethtool -x eth0
  RX flow hash indirection table for eth0 with 8 RX ring(s):
      0:    0     1     2     3     4     5     6     7
  RSS hash key:
  84:50:f4:00:a8:15:d1:a7:e9:7f:1d:60:35:c7:47:25:42:97:74:ca:56:bb:b6:a1:d8:43:e3:c9:0c:fd:17:55:c2:3a:4d:69:ed:f1:42:89

```
### netdev_tstamp_prequeue


如果设为 0，RX 包时间戳可以在 RPS 处理之后、由目标 CPU 处理包时采样。这可能在时间戳上引入一些延迟，但允许将负载分布到多个 CPU 上。

如果设为 1（默认值），时间戳会在排队之前尽快被采样。

### netdev_unregister_timeout_secs


注销网络设备超时时间（以秒为单位）。此选项控制在设备注销期间等待网络设备引用计数降为 0 时，发出警告所用的超时（秒）。较小的值在二分查错（bisection）时可能有助于更快地检测到泄漏的引用。较大的值可能有助于在缓慢/高负载系统上避免误报警告。默认值为 10，最小值为 1，最大值为 3600。

### skb_defer_max


由分配它们的 CPU 释放的、每 CPU 的 skb 列表的最大大小（以 skb 计）。

默认值：128

### optmem_max


每个 socket 允许的辅助缓冲（ancillary buffer）最大大小。辅助数据是一系列带附加数据的 struct cmsghdr 结构。TCP 发送零拷贝（tx zerocopy）也使用 optmem_max 作为其内部结构的上限。

默认值：128 KB

### fb_tunnels_only_for_init_net


控制是否自动创建回退隧道（如 tunl0、gre0、gretap0、erspan0、sit0、ip6tnl0、ip6gre0）。有 3 种可能：

(a) 值 = 0；在各个网络命名空间中加载模块时创建相应的回退隧道（向后兼容行为）。
(b) 值 = 1；[kcmd 值：initns] 相应的回退隧道仅在 init 网络命名空间中创建，其他所有网络命名空间都不会拥有它们。
(c) 值 = 2；[kcmd 值：none] 在任意网络命名空间中加载模块时都不会创建回退隧道。如果这些模块是内建的，启动后将值设为“2”没有意义，因此有一个内核命令行选项可以更改此默认值。更多细节请参阅 Documentation/admin-guide/kernel-parameters.txt。

不创建回退隧道，让用户空间能够只创建所需内容，并避免创建冗余的设备。

默认值：0（出于兼容性原因）

### devconf_inherit_init_net


控制一个新的网络命名空间是否应继承 /proc/sys/net/{ipv4,ipv6}/conf/{all,default}/ 下的所有当前设置。默认情况下，我们保持当前行为：对于 IPv4，我们从 init_net 继承所有当前设置；对于 IPv6，我们将所有设置重置为默认值。

如果设为 1，IPv4 和 IPv6 设置都被强制从 init_net 中的当前设置继承。如果设为 2，IPv4 和 IPv6 设置都被强制重置为各自的默认值。如果设为 3，IPv4 和 IPv6 设置都被强制从创建此新 netns 的那个 netns 中的当前设置继承。

默认值：0（出于兼容性原因）

### txrehash


控制当 SO_TXREHASH 选项被设为 SOCK_TXREHASH_DEFAULT（即未被 setsockopt 覆盖）时，socket 上的默认哈希重算（hash rethink）行为。

如果设为 1（默认值），会在监听 socket 上执行哈希重算。如果设为 0，则不执行哈希重算。

### txq_reselection_ms


控制一个繁忙的已连接流可以多频繁地（以毫秒为单位）选择另一个 tx 队列。

当用户线程已迁移且 XPS 会选择不同队列时，重新选择是可取的。即使没有 XPS，如果流哈希发生了变化，也可能发生同样的情况。

但切换 txq 可能引入乱序（reorder），尤其是在旧队列处于高压力下时。现代 TCP 栈若乱序发生得不频繁，能够很好地应对。

要禁用此功能，请将值设为 0。

默认值：1000

### gro_normal_batch


GRO 输出时批量合并的最大段数。当一个包退出 GRO 时（无论是作为合并后的超级帧（superframe），还是作为 GRO 决定不合并的原始包），它会被放入一个每 NAPI 的列表中。当段的数量达到 gro_normal_batch 上限时，该列表会被传递给网络栈。

### high_order_alloc_disable


默认情况下，页碎片（page frag）分配器尝试使用高阶页（在 x86 上为 order-3）。虽然默认行为在大多数情况下效果良好，但某些用户可能遇到页分配/释放中的争用。在较旧的内核（< 5.14）上当高阶页未存储在每 CPU 列表上时，这一点尤为明显。这允许选择改用 order-0 分配，但现在主要具有历史意义。

默认值：0

### 2. /proc/sys/net/unix - Unix 域套接字参数


此目录中只有一个文件。unix_dgram_qlen 限制 Unix 域 socket 缓冲区中排队的 datagram 的最大数量。除非指定了 PF_UNIX 标志，否则它不会生效。


### 3. /proc/sys/net/ipv4 - IPV4 设置


请参阅：Documentation/networking/ip-sysctl.rst 与 Documentation/admin-guide/sysctl/net.rst，了解这些条目的说明。


### 4. Appletalk


/proc/sys/net/appletalk 目录在 Appletalk 加载时保存其配置数据。可配置的参数有：

### aarp-expiry-time


在将一个 ARP 条目过期之前我们保留它的时间。用于淘汰（age out）旧的主机。

### aarp-resolve-time


我们将尝试解析一个 Appletalk 地址所花费的时间。

### aarp-retransmit-limit


在放弃之前我们将重传一次查询的次数。

### aarp-tick-time


控制检查过期（expire）的速率。

目录 /proc/net/appletalk 保存机器上活跃 Appletalk socket 的列表。

这些字段指示 DDP 类型、本地地址（network:node 格式）、远端地址、发送挂起队列的大小、接收队列的大小（等待应用程序读取的字节数）、状态以及拥有该 socket 的 uid。

/proc/net/atalk_iface 列出所有为 appletalk 配置的接口。它显示接口名称、其 Appletalk 地址、该地址上的网络范围（或 phase 1 网络中的网络号），以及接口的状态。

/proc/net/atalk_route 列出每个已知的网络路由。它列出路由所指向的目标（网络）、路由器（可能直接相连）、路由标志，以及该路由所使用的设备。

### 5. TIPC


### tipc_rmem


TIPC 协议现在有一个针对接收内存的可调参数，类似于 tcp_rmem——即一个包含 3 个 INTEGER 的向量：(min, default, max)

```

    # cat /proc/sys/net/tipc/tipc_rmem
    4252725 34021800        68043600
    #

```
max 值被设为 CONN_OVERLOAD_LIMIT，而 default 和 min 值是该同一值的缩放（移位）版本。注意 min 值目前在有意义的层面并未被使用，但保留这个三元组是为了与 tcp_rmem 等保持一致。

### named_timeout


TIPC 名称表更新在集群中是异步分发的，没有任何形式的事务处理。这意味着可能出现不同的竞态场景。其中一种情况是，一个节点发出的名称撤销（name withdrawal）被另一个节点接收时，可能晚于已经从一个第三个节点接受的、与之前重叠的名称发布（name publication），尽管这些冲突的更新最初可能是按正确的顺序发出的。如果 named_timeout 非零，失败拓扑更新会被放入一个延迟队列，直到另一个清除该错误的事件到达，或者直到超时到期。值以毫秒为单位。

### 6. /proc/sys/net/vsock - VSOCK 套接字


VSOCK 套接字（AF_VSOCK）提供虚拟机与其宿主机之间的通信。VSOCK 套接字在网络命名空间中的行为由该命名空间的模式（`global` 或 `local`）决定，该模式控制 CID（Context ID）如何分配，以及 socket 如何跨命名空间交互。

### ns_mode


只读。报告当前命名空间的模式，在命名空间创建时设定，此后不可变。

取值：

 - `global` - 该命名空间共享系统范围的 CID 分配，其 socket 可以到达任意全局命名空间中的任意 VM 或 socket。此命名空间中的 socket 无法到达 local 命名空间中的 socket。
 - `local` - 该命名空间拥有私有的 CID 分配，其 socket 只能连接到同一命名空间内的 VM 或 socket。

init_net 的模式始终为 `global`。

### child_ns_mode


控制新创建的子命名空间将继承何种模式。在命名空间创建时，`ns_mode` 从父命名空间的 `child_ns_mode` 继承。初始值与该命名空间自身的 `ns_mode` 相匹配。

取值：

 - `global` - 子命名空间将共享系统范围的 CID 分配，其 socket 将能够到达任意全局命名空间中的任意 VM 或 socket。
 - `local` - 子命名空间将拥有私有的 CID 分配，其 socket 将只能在其自身命名空间内连接。

对 `child_ns_mode` 的第一次写入会锁定其值。后续写入相同的值会成功，但写入不同的值会返回 `-EBUSY`。

更改 `child_ns_mode` 只影响更改之后创建的命名空间；它不会修改当前命名空间或任何已有的子命名空间。

`ns_mode` 设为 `local` 的命名空间无法将 `child_ns_mode` 改为 `global`（返回 `-EPERM`）。

### g2h_fallback


控制到不被宿主机到客户机（H2G）传输所拥有的 CID 的连接，是否自动回退（fall back）到客户机到宿主机（G2H）传输。

启用时，如果一次 connect 的目标是一个 H2G 传输（例如 vhost-vsock）不服务的 CID，或者根本没有加载任何 H2G 传输，则该连接会通过 G2H 传输（例如 virtio-vsock）路由。这使得同时运行嵌套 VM（通过 vhost-vsock）以及可通过管理程序（例如 Nitro Enclaves）到达的兄弟 VM 的主机，能够使用单一 CID 空间寻址二者，而无需应用程序设置 `VMADDR_FLAG_TO_HOST`。

当发生回退时，会在远端地址上自动设置 `VMADDR_FLAG_TO_HOST`，以便用户空间可以通过 `getpeername()` 确定路径。

注意：启用此 sysctl 后，试图与 H2G 传输未实现的客户机 CID 通信的用户空间会产生宿主机 vsock 流量。依赖仅 H2G 隔离的环境应将其设为 0。

取值：

 - 0 - 到 CID <= 2 或带有 VMADDR_FLAG_TO_HOST 的连接使用 G2H；所有其他连接使用 H2G（如果 H2G 未加载，则因 ENODEV 失败）。
 - 1 - 到 H2G 不拥有的 CID 的连接回退到 G2H。（默认值）
