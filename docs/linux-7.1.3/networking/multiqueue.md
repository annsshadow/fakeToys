
## 多队列网络设备支持 HOWTO


## 第 1 节：实现多队列支持的底层驱动要求


### 简介：内核对多队列设备的支持


内核始终提供对多队列设备的支持。

底层驱动需要使用新的 alloc_etherdev_mq() 或 alloc_netdev_mq() 函数来为设备分配子队列。底层的 kernel API 将负责子队列内存的分配与释放，以及队列在内存中位于何处的 netdev 配置。

底层驱动还需要像今天管理全局 netdev->queue_lock 那样管理这些队列。因此，在设备仍处于运行状态时，底层驱动应使用 netif_{start|stop|wake}_subqueue() 函数来管理每个队列。netdev->queue_lock 仍用于设备上线或完全关闭时（unregister_netdev() 等）。

## 第 2 节：qdisc 对多队列设备的支持


目前有两个 qdisc 针对多队列设备进行了优化。第一个是默认的 pfifo_fast qdisc。该 qdisc 每个硬件队列支持一个 qdisc。一个新的轮询 qdisc，sch_multiq，也支持多个硬件队列。qdisc 负责分类 skb，然后根据 skb->queue_mapping 的值将 skb 导向对应的 band 与队列。在底层驱动中使用此字段来决定将 skb 发送到哪个队列。

sch_multiq 已为希望避免队头阻塞（head-of-line blocking）的硬件添加。它将在各个 band 间循环，并在出队一个数据包之前验证与该 band 关联的硬件队列未被停止。

在 qdisc 加载时，band 的数量基于硬件上的队列数量。一旦建立关联，任何设置了 skb->queue_mapping 的 skb 都将被排队到与硬件队列关联的 band。

## 第 3 节：使用 MULTIQ 处理多队列设备的简要 HOWTO


用户空间命令 'tc'（iproute2 软件包的一部分）用于配置 qdisc。要将 MULTIQ qdisc 添加到你的网络设备，假设设备
```

    # tc qdisc add dev eth0 root handle 1: multiq

```

qdisc 将分配与设备报告的队列数量相等的 band 数，并使 qdisc 上线。假设 eth0 有 4 个 Tx
```

    band 0 => queue 0
    band 1 => queue 1
    band 2 => queue 2
    band 3 => queue 3

```

流量将基于 simple_tx_hash 函数，或者如果你定义了 netdev->select_queue()，则基于它流经每个队列。

tc 过滤器的行为保持不变。不过新增了一个 tc 动作 skbedit。假设你想将所有到特定主机（例如 192.168.0.3）的流量通过特定队列路由，你可以使用
```

    tc filter add dev eth0 parent 1: protocol ip prio 1 u32 \
	    match ip dst 192.168.0.3 \
	    action skbedit queue_mapping 3

```

:Author: Alexander Duyck <alexander.h.duyck@intel.com>
:Original Author: Peter P. Waskiewicz Jr. <peter.p.waskiewicz.jr@intel.com>
