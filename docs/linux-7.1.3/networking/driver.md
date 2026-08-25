
## Softnet 驱动问题


## 探测准则


### 地址校验


你为设备获取的任何硬件层地址都应经过校验。例如，对于以太网，可使linux/etherdevice.h:is_valid_ether_addr() 进行校验
## 关闭/停止准则


### 静默


在调ndo_stop 例程之后，硬件不得再接收或发送任何数据。所有在途的数据包都必须被中止。如有必要，轮询或等待任何复位命令完成
### 自动关闭


如果设备仍处UP 状态，unregister_netdevice 将调ndo_stop 例程
## 发送路径准

### 提前停止队列


ndo_start_xmit 方法在任何正常情况下都不得返NETDEV_TX_BUSY。除非你的设备无法提前获知其发送功能何时会变得繁忙，否则这被视为一个严重错误
相反，它必须正确地维护队列。例如，对于实现了分聚集（scatter-gather）的驱动来说，这意味着

	static u32 drv_tx_avail(struct drv_ring *dr)
	{
		u32 used = READ_ONCE(dr->prod) - READ_ONCE(dr->cons);

		return dr->tx_ring_size - (used & bp->tx_ring_mask);
	}

	static netdev_tx_t drv_hard_start_xmit(struct sk_buff *skb,
					       struct net_device *dev)
	{
		struct drv *dp = netdev_priv(dev);
		struct netdev_queue *txq;
		struct drv_ring *dr;
		int idx;

		idx = skb_get_queue_mapping(skb);
		dr = dp->tx_rings[idx];
		txq = netdev_get_tx_queue(dev, idx);

		//...
		/** This should be a very rare race - log it. **/
		if (drv_tx_avail(dr) <= skb_shinfo(skb)->nr_frags + 1) {
			netif_stop_queue(dev);
			netdev_warn(dev, "Tx Ring full when queue awake!\n");
			return NETDEV_TX_BUSY;
		}

		//... queue packet to card ...

		netdev_tx_sent_queue(txq, skb->len);

		//... update tx producer index using WRITE_ONCE() ...

		if (!netif_txq_maybe_stop(txq, drv_tx_avail(dr),
					  MAX_SKB_FRAGS + 1, 2 * MAX_SKB_FRAGS))
			dr->stats.stopped++;

		//...
		return NETDEV_TX_OK;
	}

然后在你TX 回收事件处理结束时：


	//... update tx consumer index using WRITE_ONCE() ...

	netif_txq_completed_wake(txq, cmpl_pkts, cmpl_bytes,
				 drv_tx_avail(dr), 2 * MAX_SKB_FRAGS);

#### 无锁队列停止/唤醒辅助

   :doc: Lockless queue stopping / waking helpers.

### 无独占所有权


ndo_start_xmit 方法不得修改被克隆的 SKB 的共享部分
### 及时完成


不要忘记，一旦你ndo_start_xmit 方法返回 NETDEV_TX_OK，释放该 SKB 就是你的驱动的责任，并且必须在有限的时间内完成
例如，这意味着如果你的 TX 缓解（mitigation）方案在没有任何新的 TX 数据包发送时，不允许TX 数据包永远“滞留”在 TX 环中而未被回收。此错误可能导致正在等待发送缓冲区空间释放的套接字发生死锁
如果你从 ndo_start_xmit 方法返回 NETDEV_TX_BUSY，你不得保留对该 SKB 的任何引用，也不得尝试释放它