
## Hyper-V 网络驱动


## 兼容性

该驱动兼容 Windows Server 2012 R2、2016 以及 Windows 10。

## 功能


### 校验和卸载

  netvsc 驱动支持校验和卸载，前提是 Hyper-V 主机版本也支持。Windows Server 2016
  和 Azure 支持对 IPv4 和 IPv6 的 TCP 与 UDP 进行校验和卸载。Windows Server 2012
  仅支持对 TCP 进行校验和卸载。

### 接收端缩放（Receive Side Scaling）

  Hyper-V 支持接收端缩放。对于 TCP 和 UDP，可以根据 IP 地址和端口号将数据包
  分发到可用的队列中。

  对于 TCP 和 UDP，我们可以通过 ethtool 命令在 L3 和 L4 之间切换哈希级别。基于
  IPv4 和 IPv6 的 TCP/UDP 可以分别设置。默认的哈希级别是 L4。我们目前只允许在
  客户机内部切换 TX 哈希级别。

  在 Azure 上，分片的 UDP 数据包在使用 L4 哈希时丢包率较高。这种情况下建议使用
  L3 哈希。

  例如，对于 eth0 上基于 IPv4 的 UDP：

```

	ethtool -N eth0 rx-flow-hash udp4 sdfn

  To exclude UDP port numbers in hashing::

	ethtool -N eth0 rx-flow-hash udp4 sd

  To show UDP hash level::

	ethtool -n eth0 rx-flow-hash udp4

```
### 通用接收卸载（GRO）

  该驱动支持 GRO，并且默认启用。GRO 会将相似的数据包合并，从而在高 Rx 负载下
  显著降低 CPU 使用率。

### 大型接收卸载（LRO），或称接收端合并（Receive Side Coalescing，RSC）

  该驱动在 vSwitch 功能中支持 LRO/RSC。它通过尽可能将多个 TCP 段合并，来减少
  每个数据包的处理开销。该特性在运行于 Windows Server 2019 的 VM 上默认启用，
  以及
```

	ethtool -K eth0 lro on
	ethtool -K eth0 lro off

```
### SR-IOV 支持

  Hyper-V 支持 SR-IOV 作为硬件加速选项。如果在 vSwitch 和客户机配置中都启用了
  SR-IOV，则虚拟功能（Virtual Function，VF）设备会作为 PCI 设备透传给客户机。
  在这种情况下，客户机操作系统中同时可见一个合成（synthetic，netvsc）设备和 VF
  设备，并且两个 NIC 具有相同的 MAC 地址。

  VF 被 netvsc 设备接管（enslaved）。netvsc 驱动会在 VF 可用且已 up 时，透明地
  将数据路径切换到 VF。网络状态（地址、防火墙等）应当只应用于 netvsc 设备；在
  大多数情况下不应直接访问从设备（slave device）。例外情况是，如果需要某些特殊的
  队列规则或流方向，则应当直接应用到 VF 从设备上。

### 接收缓冲区

  数据包被接收到一个在设备被探测（probe）时创建的接收区域中。接收区域被拆分为
  MTU 大小的块，每个块可以包含一个或多个数据包。接收段的数量可以通过 ethtool 的
  Rx ring 参数来改变。

  有一个类似的发送缓冲区，用于聚合要发送的数据包。发送区域被拆分为块，通常每块
  6144 字节，每个段可以包含一个或多个数据包。小数据包通常通过复制到发送缓冲区
  来传输。不过，如果缓冲区暂时耗尽，或者要传输的数据包是 LSO 数据包，驱动会向
  主机提供来自 SKB 的数据指针。这试图在复制数据的开销与将 VM 内存重新映射为可被
  主机访问的影响之间取得平衡。

### XDP 支持

  XDP（eXpress Data Path）是一项在数据包到达 NIC 卡的早期阶段运行 eBPF 字节码
  的功能。其目标是提升数据包处理的性能，减少 SKB 分配以及其它上层网络协议栈的
  开销。

  hv_netvsc 支持原生（native）模式的 XDP，并且也会透明地在关联的 VF NIC 上设置
  XDP 程序。

  在合成 NIC（netvsc）上设置/取消 XDP 程序会自动传播到 VF NIC。不建议直接在 VF
  NIC 上设置/取消 XDP 程序，它也不会传播到合成 NIC，并且可能被合成 NIC 的设置
  覆盖。

  XDP 程序无法与启用的 LRO（RSC）一起运行，因此你需要禁用 LRO
```

	ethtool -K eth0 lro off

  XDP_REDIRECT 动作尚不支持。

```
