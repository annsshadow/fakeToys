## 基于 InfiniBand 的 IP（IPoIB）


  ib_ipoib 驱动实现了在 InfiniBand 之上的 IP（IP over InfiniBand）协议，符合
  IETF ipoib 工作组发布的 RFC 4391 和 4392 规范。它是“原生”实现，即把接口类型设为
  ARPHRD_INFINIBAND、硬件地址长度设为 20（早期的私有实现是伪装成以太网接口向内核
  注册的）。

## 分区与 P_Key


  当 IPoIB 驱动被加载时，它会为每个端口使用索引 0 处的 P_Key 创建一个接口。要创建
  一个使用不同 P_Key 的接口，可将期望的 P_Key 写入主接口的
```

    echo 0x8001 > /sys/class/net/ib0/create_child

  这将创建一个名为 ib0.8001、P_Key 为 0x8001 的接口。要删除一个子接口，使用
  "delete_child" 文件::

    echo 0x8001 > /sys/class/net/ib0/delete_child

  P_Key 可通过 "pkey" 文件获取，子接口的主接口在 "parent" 中。

  子接口的创建/删除也可以使用 IPoIB 的 rtnl_link_ops 完成，两种方式创建的子接口
  行为一致。

```
## 数据报模式与连接模式


  IPoIB 驱动支持两种操作模式：数据报（datagram）和连接（connected）。模式通过接口
  的 /sys/class/net/<intf name>/mode 文件设置和读取。

  在数据报模式下，使用 IB UD（不可靠数据报）传输，因此接口 MTU 等于 IB L2 MTU
  减去 IPoIB 封装头（4 字节）。例如在典型的 2K MTU 的 IB 交换结构中，IPoIB MTU 为
  2048 - 4 = 2044 字节。

  在连接模式下，使用 IB RC（可靠连接）传输。连接模式利用了 IB 传输的面向连接特性，
  允许 MTU 最大达到 64K 的 IP 包大小，从而减少处理大型 UDP 数据报、TCP 段等所需的
  IP 包数量，并提升大消息的性能。

  在连接模式下，接口的 UD QP 仍用于组播和与不支持连接模式的对端通信。这种情况下，
  使用 ICMP PMTU 包的 RX 模拟来促使网络栈对这些邻居使用较小的 UD MTU。

## 无状态卸载


  如果 IB 硬件支持 IPoIB 无状态卸载，IPoIB 会向网络栈通告 TCP/IP 校验和和/或大发送
  （LSO）卸载能力。

  大接收（LRO）卸载也已实现，可通过 ethtool 调用开启/关闭。目前 LRO 仅支持具备
  校验和卸载能力的设备。

  无状态卸载仅在数据报模式下受支持。

## 中断 moderation


  如果底层 IB 设备支持 CQ 事件 moderation，可以使用 ethtool 设置中断缓解参数，从而
  减少处理中断带来的开销。IPoIB 的主代码路径不使用事件来做 TX 完成通知，因此只支持
  RX moderation。

## 调试信息


  通过将 CONFIG_INFINIBAND_IPOIB_DEBUG 编译选项设为 'y'，跟踪信息会被编译进驱动。
  通过将模块参数 debug_level 和 mcast_debug_level 设为 1 来开启。这些参数可以在运行时
  通过 /sys/module/ib_ipoib/ 下的文件进行控制。

  CONFIG_INFINIBAND_IPOIB_DEBUG 还会在 debugfs 中启用文件
```

    mount -t debugfs none /sys/kernel/debug

  这样就可以从 /sys/kernel/debug/ipoib/ib0_mcg 等文件获取关于组播组的统计信息。

  该选项的性能影响可忽略不计，因此对于正常操作，将 debug_level 设为 0 启用此选项是
  安全的。

  CONFIG_INFINIBAND_IPOIB_DEBUG_DATA 会在 data_debug_level 设为 1 时在数据路径上
  输出更多调试信息。然而，即使关闭了输出，启用该配置选项也会影响性能，因为它会向
  快速路径中添加判断。

```
## 参考资料


  Transmission of IP over InfiniBand (IPoIB) (RFC 4391)
    http://ietf.org/rfc/rfc4391.txt

  IP over InfiniBand (IPoIB) Architecture (RFC 4392)
    http://ietf.org/rfc/rfc4392.txt

  IP over InfiniBand: Connected Mode (RFC 4755)
    http://ietf.org/rfc/rfc4755.txt
