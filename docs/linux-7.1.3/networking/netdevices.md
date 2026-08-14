## 网络设备和内核，以及你！


## 简介

以下是关于网络设备的一些零散文档集合。它面向驱动开发者。

## struct net_device 的生命周期规则

网络设备结构体即使在模块被卸载后也必须持续存在，并且必须使用 alloc_netdev_mqs() 及其相关函数进行分配。如果设备已成功注册，它将在最后一次使用时由 free_netdev() 释放。这是为了让那个极端情况（pathological case）能够被干净地处理（例如：`rmmod mydriver </sys/class/net/myeth/mtu`）。

alloc_netdev_mqs() / alloc_netdev() 会为驱动私有数据预留额外空间，该空间在网络设备被释放时一同释放。如果分配的独立数据被附加到网络设备（netdev_priv()）上，则由模块退出处理函数负责释放它。

注册 struct net_device 有两组 API。第一组可用于 `rtnl_lock` 尚未持有的普通上下文：register_netdev()、unregister_netdev()。第二组可用于 `rtnl_lock` 已经持有的情形：register_netdevice()、unregister_netdevice()、free_netdevice()。

### 简单驱动


大多数驱动（尤其是设备驱动）在 `rtnl_lock` 未被持有（例如驱动的 probe 和 remove 路径）的上下文中处理 struct net_device 的生命周期。

在这种情况下，struct net_device 的注册使用 register_netdev() 和 unregister_netdev() 函数完成：


  int probe()
  {
    struct my_device_priv *priv;
    int err;

    dev = alloc_netdev_mqs(...);
    if (!dev)
      return -ENOMEM;
    priv = netdev_priv(dev);

    /* ... 在调用 register_netdev() 之前完成所有设备设置 ...
     */

    err = register_netdev(dev);
    if (err)
      goto err_undo;

    /** net_device 对用户可见！ **/

  err_undo:
    /** ... 撤销设备设置 ... **/
    free_netdev(dev);
    return err;
  }

  void remove()
  {
    unregister_netdev(dev);
    free_netdev(dev);
  }

注意，调用 register_netdev() 之后，设备便在系统中可见。用户可以立即打开它并开始发送/接收流量，或运行任何其他回调，因此所有初始化都必须在注册之前完成。

unregister_netdev() 会关闭设备并等待所有用户使用完毕。struct net_device 自身的内存可能仍被 sysfs 引用，但对该设备的所有操作都会失败。

free_netdev() 可以在 unregister_netdev() 返回之后，或者 register_netdev() 失败时调用。

### 在 RTNL 下的设备管理


在已经持有 `rtnl_lock` 的上下文中注册 struct net_device 需要格外小心。在这些场景中，大多数驱动会希望利用 struct net_device 的 `needs_free_netdev` 和 `priv_destructor` 成员来释放状态。

在 `rtnl_lock` 下处理 netdev 的示例流程：


  static void my_setup(struct net_device *dev)
  {
    dev->needs_free_netdev = true;
  }

  static void my_destructor(struct net_device *dev)
  {
    some_obj_destroy(priv->obj);
    some_uninit(priv);
  }

  int create_link()
  {
    struct my_device_priv *priv;
    int err;

    ASSERT_RTNL();

    dev = alloc_netdev(sizeof(*priv), "net%d", NET_NAME_UNKNOWN, my_setup);
    if (!dev)
      return -ENOMEM;
    priv = netdev_priv(dev);

    /** 隐式构造函数 **/
    err = some_init(priv);
    if (err)
      goto err_free_dev;

    priv->obj = some_obj_create();
    if (!priv->obj) {
      err = -ENOMEM;
      goto err_some_uninit;
    }
    /** 构造函数结束，设置析构函数： **/
    dev->priv_destructor = my_destructor;

    err = register_netdevice(dev);
    if (err)
      /** register_netdevice() 会在失败时调用析构函数 **/
      goto err_free_dev;

    /* 如果此后有任何失败，unregister_netdevice()（或 unregister_netdev()）
     - 会负责调用 my_destructor 和 free_netdev()。
     */

    return 0;

  err_some_uninit:
    some_uninit(priv);
  err_free_dev:
    free_netdev(dev);
    return err;
  }

如果设置了 struct net_device.priv_destructor，核心代码会在 unregister_netdevice() 之后的某个时刻调用它，如果 register_netdevice() 失败它也会被调用。该回调可能在持有或未持有 `rtnl_lock` 的情况下被调用。

没有显式的构造函数回调，驱动在分配私有 netdev 状态之后、注册之前"构造"它。

设置 struct net_device.needs_free_netdev 会使核心代码在 unregister_netdevice() 之后、当对设备的所有引用都消失时，自动调用 free_netdevice()。它仅在成功调用 register_netdevice() 之后才生效，因此如果 register_netdevice() 失败，驱动负责调用 free_netdev()。

free_netdev() 在出错路径上、紧接 unregister_netdevice() 之后，或 register_netdevice() 失败时，都是安全可调用的。netdev 的（注销）注册过程的某些部分发生在 `rtnl_lock` 释放之后，因此在这些情况下 free_netdev() 会将其部分处理推迟到 `rtnl_lock` 释放之后进行。

从 struct rtnl_link_ops 派生出的设备绝不应直接释放 struct net_device。

#### .ndo_init 和 .ndo_uninit


`.ndo_init` 和 `.ndo_uninit` 回调在 net_device 注册和注销期间、在 `rtnl_lock` 下被调用。驱动可以在它们初始化过程的某些部分需要在 `rtnl_lock` 下运行时使用这些回调。

`.ndo_init` 在设备于系统中可见之前运行，`.ndo_uninit` 在设备关闭后的注销过程中运行，但其他子系统可能仍然持有对 netdev 的未决引用。

## MTU

每个网络设备都有一个最大传输单元（Maximum Transfer Unit，MTU）。MTU 不包含任何链路层协议开销。上层协议不得向设备传入一个数据量超过 mtu 的套接字缓冲区（skb）来传输。MTU 不包含链路层头部开销，例如标准 MTU 为 1500 字节的以太网，由于以太网头部的存在，实际 skb 最多会包含 1514 字节。设备还应当为 4 字节的 VLAN 头部留出空间。

分片卸载（Segmentation Offload，GSO、TSO）是此规则的一个例外。上层协议可以向设备的发送例程传入一个大的套接字缓冲区，设备会根据当前 MTU 将其拆分成独立的数据包。

MTU 是对称的，同时适用于接收和发送。设备必须能够接收至少 MTU 所允许的最大尺寸的数据包。网络设备可以将 MTU 用作调整接收缓冲区大小的机制，但设备应当允许带有 VLAN 头部的数据包。标准以太网 mtu 为 1500 字节时，设备应当允许最多 1518 字节的数据包（1500 + 14 头部 + 4 标签）。设备可以：丢弃、截断，或向上传递超大（oversize）数据包，但丢弃超大数据包是首选。


## struct net_device 同步规则

ndo_open:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形（shaper）API，还需 netdev 实例锁。
	上下文：进程

ndo_stop:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
	上下文：进程
	注意：netif_running() 保证为 false

ndo_do_ioctl:
	同步：rtnl_lock() 信号量。

	这仅由网络子系统在内部调用，而不是像 linux-5.14 之前那样由用户空间调用 ioctl 触发。

ndo_siocbond:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
        上下文：进程

	由 bonding 驱动用于 SIOCBOND 系列的 ioctl 命令。

ndo_siocwandev:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
	上下文：进程

	由 drivers/net/wan 框架用于配合 if_settings 结构体处理 SIOCWANDEV ioctl。

ndo_siocdevprivate:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
	上下文：进程

	这用于实现 SIOCDEVPRIVATE ioctl 辅助函数。不应将其添加到新驱动中，所以不要使用。

ndo_eth_ioctl:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
	上下文：进程

ndo_get_stats:
	同步：RCU（可以与统计信息更新路径并发调用）。
	上下文：原子（atomic，不能在 RCU 下睡眠）

ndo_start_xmit:
	同步：__netif_tx_lock 自旋锁。

	当驱动设置 dev->lltx 时，这将在不持有 netif_tx_lock 的情况下被调用。这种情况下驱动需要在需要时自行加锁。
	那里的加锁还应当正确防止与 set_rx_mode 之间的竞争。警告：使用 dev->lltx 已被弃用。不要在新驱动中使用它。

	上下文：BH 被禁用时的进程或 BH（定时器），netconsole 会在中断被禁用的情况下调用它。

	返回码：

 - NETDEV_TX_OK 一切正常。
 - NETDEV_TX_BUSY 无法发送数据包，稍后重试
	  通常是一个 bug，意味着驱动中的队列启动/停止流控被破坏。
	  注意：驱动不得将 skb 放入其 DMA 环中。

ndo_tx_timeout:
	同步：netif_tx_lock 自旋锁；所有 TX 队列被冻结。
	上下文：BH 被禁用
	注意：netif_queue_stopped() 保证为 true

ndo_set_rx_mode:
	同步：netif_addr_lock 自旋锁。
	上下文：BH 被禁用
	注意：已弃用，推荐使用在进程上下文中运行的 ndo_set_rx_mode_async。

ndo_set_rx_mode_async:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。
	上下文：进程（来自工作队列）
	注意：ndo_set_rx_mode 的异步版本，在进程上下文中运行。接收单播和组播地址列表的快照。

ndo_change_rx_flags:
	同步：rtnl_lock() 信号量。此外，如果驱动实现了队列管理或整形 API，还需 netdev 实例锁。

ndo_setup_tc:
	`TC_SETUP_BLOCK` 和 `TC_SETUP_FT` 运行在 NFT 锁下（即没有 `rtnl_lock`，也没有设备实例锁）。其余的 `tc_setup_type` 类型在驱动实现了队列管理或整形 API 时，运行在 netdev 实例锁下。

上面列表未指定的大多数 ndo 回调都运行在 `rtnl_lock` 下。此外，如果驱动实现了队列管理或整形 API，还会同时获取 netdev 实例锁。

## struct napi_struct 同步规则

napi->poll:
	同步：
		napi->state 中的 NAPI_STATE_SCHED 位。设备的 ndo_stop 方法会对所有 NAPI 实例调用 napi_disable()，它会针对 NAPI_STATE_SCHED napi->state 位进行睡眠式轮询，等待所有未决的 NAPI 活动停止。

	上下文：
		软中断（softirq）
		会被 netconsole 在中断被禁用的情况下调用。

## netdev 实例锁


历史上，所有网络控制操作都由一个称为 `rtnl_lock` 的单一全局锁保护。目前有一项持续的努力，要用每个网络命名空间独立的锁来取代这个全局锁。此外，单个 netdev 的属性越来越多地由 per-netdev 锁保护。

对于实现了整形或队列管理 API 的设备驱动，所有控制操作都将在 netdev 实例锁下进行。驱动也可以通过将 `request_ops_lock` 设为 true，显式请求在操作（ops）期间持有实例锁。代码注释和文档将操作在实例锁下被调用的驱动称为"ops locked"（锁定的操作）。另请参阅 struct net_device 的 `lock` 成员的文档。

还存在一种依次获取两个 per-netdev 锁的情况：当 netdev 队列被租借（lease）时，即虚拟设备和物理设备的 netdev 作用域锁都被获取。为防止死锁，虚拟设备的锁必须始终在物理设备的锁之前获取（参见 `netdev_nl_queue_create_doit`）。

将来，会有选项允许各个驱动选择不使用 `rtnl_lock`，而是直接在其 netdev 实例锁下执行控制操作。

鼓励设备驱动尽可能依赖实例锁。

对于需要与原核心栈交互的（主要是软件的）驱动，有两组接口：`dev_xxx`/`netdev_xxx` 和 `netif_xxx`（例如 `dev_set_mtu` 和 `netif_set_mtu`）。`dev_xxx`/`netdev_xxx` 函数自己负责获取实例锁，而 `netif_xxx` 函数假定驱动已经获取了实例锁。

### struct net_device_ops


对于大多数驱动，`ndos` 在不持有实例锁的情况下被调用。

对于"ops locked"驱动，大多数 `ndos` 会在实例锁下被调用。

### struct ethtool_ops


与 `ndos` 类似，实例锁仅对选定的驱动持有。对于"ops locked"驱动，所有 ethtool 操作无一例外都应在实例锁下调用。

### struct netdev_stat_ops


对于"ops locked"驱动，"qstat"操作在实例锁下被调用，而对于所有其他驱动则在 rtnl_lock 下调用。

### struct net_shaper_ops


所有网络整形（net shaper）回调在持有 netdev 实例锁时被调用。`rtnl_lock` 可能持有，也可能未持有。

注意，支持网络整形会自动启用"ops locking"（操作锁定）。

### struct netdev_queue_mgmt_ops


所有队列管理回调在持有 netdev 实例锁时被调用。`rtnl_lock` 可能持有，也可能未持有。

注意，支持 struct netdev_queue_mgmt_ops 会自动启用"ops locking"（操作锁定）。

### 通知链（Notifiers）与 netdev 实例锁


对于实现了整形或队列管理 API 的设备驱动，部分通知（`enum netdev_cmd`）运行在 netdev 实例锁下。

以下 netdev 通知链总是在实例锁下运行：
- `NETDEV_XDP_FEAT_CHANGE`

对于具有锁定操作的设备，目前只有以下通知链在锁下运行：
- `NETDEV_CHANGE`
- `NETDEV_REGISTER`
- `NETDEV_UP`

以下通知链在没有锁的情况下运行：
- `NETDEV_UNREGISTER`

对于其余通知链没有明确的预期。不在列表中的通知链可能带锁或不带锁运行，甚至可能从不同代码路径以带锁和不带锁两种方式调用同一类型的通知链。目标是最终确保所有（或大多数，除少数有文档说明的例外）通知链都在实例锁下运行。每当你对某个通知链下持有锁做出明确假设时，请扩展本文档。

## NETDEV_INTERNAL 符号命名空间


以 NETDEV_INTERNAL 导出的符号只能用于网络核心以及与主网络邮件列表和树（tree）直接对接的驱动。注意反之不成立，NETDEV_INTERNAL 之外的大多数符号也不应被 netdev 之外的随机代码使用。符号之所以缺少该标识，可能是因为它们早于命名空间的出现，或者仅仅是由于疏忽。

