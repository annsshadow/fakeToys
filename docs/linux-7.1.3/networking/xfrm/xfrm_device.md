
## XFRM 设备 - 卸载 IPsec 计算


Shannon Nelson <shannon.nelson@oracle.com>
Leon Romanovsky <leonro@nvidia.com>


## 概述


IPsec 是保障网络流量安全的实用特性，但计算成本很高：根据流量和链路配置的不同，一10Gbps 的链路很容易降到 1Gbps 以下。幸运的是，NIC 提供基于硬件IPsec 卸载，可大幅提高吞吐量并降低 CPU 利用率。XFRM 设备接口允许 NIC 驱动向协议栈提供对硬件卸载的
访问
目前，内核支持两种类型的硬件卸载
 - IPsec 加密卸载
   - NIC 执行加密/解密
   - 内核处理其它一
 - IPsec 数据包卸载：

   - NIC 执行加密/解密
   - NIC 执行封装
   - 内核NIC SA 和策略保持同   - NIC 处理 SA 和策略状   - 内核与密钥管理器通信

用户空间对卸载的访问通常通过诸如 libreswan KAME/raccoon 这样的系统，但在试验时，
iproute2 'ip xfrm' 命令集会很方便。一个示例命令可能看起来```

  ip x s add proto esp dst 14.0.0.70 src 14.0.0.52 spi 0x07 mode transport \
     reqid 0x07 replay-window 32 \
     aead 'rfc4106(gcm(aes))' 0x44434241343332312423222114131211f4f3f2f1 128 \
     sel src 14.0.0.52/24 dst 14.0.0.70/24 proto tcp \
     offload dev eth4 dir in

```
```

  ip x s add proto esp dst 14.0.0.70 src 14.0.0.52 spi 0x07 mode transport \
     reqid 0x07 replay-window 32 \
     aead 'rfc4106(gcm(aes))' 0x44434241343332312423222114131211f4f3f2f1 128 \
     sel src 14.0.0.52/24 dst 14.0.0.70/24 proto tcp \
     offload packet dev eth4 dir in

  ip x p add src 14.0.0.70 dst 14.0.0.52 offload packet dev eth4 dir in
  tmpl src 14.0.0.70 dst 14.0.0.52 proto esp reqid 10000 mode transport

```
没错，这很难看，但这就是 shell 脚本libreswan 的用途

## 需要实现的回调


```

  /* from include/linux/netdevice.h */
  struct xfrmdev_ops {
        /* Crypto and Packet offload callbacks */
	int	(*xdo_dev_state_add)(struct net_device *dev,
                                     struct xfrm_state *x,
                                     struct netlink_ext_ack *extack);
	void	(*xdo_dev_state_delete)(struct net_device *dev,
                                        struct xfrm_state *x);
	void	(*xdo_dev_state_free)(struct net_device *dev,
                                      struct xfrm_state *x);
	bool	(*xdo_dev_offload_ok) (struct sk_buff *skb,
				       struct xfrm_state *x);
	void    (*xdo_dev_state_advance_esn) (struct xfrm_state *x);
	void    (*xdo_dev_state_update_stats) (struct xfrm_state *x);

        /* Solely packet offload callbacks */
	int	(*xdo_dev_policy_add) (struct xfrm_policy *x, struct netlink_ext_ack *extack);
	void	(*xdo_dev_policy_delete) (struct xfrm_policy *x);
	void	(*xdo_dev_policy_free) (struct xfrm_policy *x);
  };

```
提供 ipsec 卸载NIC 驱动需要实现与所支持卸载相关的回调，以使该卸载对网络协议栈的
XFRM 子系统可用。此外，特性位 NETIF_F_HW_ESP NETIF_F_HW_ESP_TX_CSUM 将表明卸的可用性

## 流程


在探测时以及调用 register_netdev() 之前，驱动应当设置本地数据结构和 XFRM 回调，并
设置特性位。XFRM 代码的监听器将在 NETDEV_REGISTER 上完成设置```

		adapter->netdev->xfrmdev_ops = &ixgbe_xfrmdev_ops;
		adapter->netdev->features |= NETIF_F_HW_ESP;
		adapter->netdev->hw_enc_features |= NETIF_F_HW_ESP;

```
当为请求“卸载”特性的SA 建立时，驱动xdo_dev_state_add() 将获得要被卸载的SA 以及它是用于 Rx 还是 Tx 的指示。驱动应
 - 验证算法支持卸载
 - 存储 SA 信息（密钥、salt、目IP、协议等 - 启用SA 的硬件卸 - 返回状态值：

		===========   ===================================
		0             success
		-EOPNETSUPP   不支持卸载，尝试 SW IPsec                              不适用于数据包卸载模式
		other         使请求失		===========   ===================================

驱动还可以在 SA 中设置一offload_handle，一个不透明void 指针
```

		xs->xso.offload_handle = context;


```
当网络协议栈为已设置卸载SA 准备一IPsec 数据包时，它首先调用 xdo_dev_offload_ok()
，传skb 和预期的卸载状态，询问驱动卸载是否可用。这可以检查数据包信息以确保卸被支持（例如 IPv4 IPv6、没IPv4 选项等），并返回 true false 以表明其支持如果驱动没有实现此回调，协议栈提供合理的默认值
加密卸载模式当准备发送时，驱动需要检Tx 数据包的卸载信息，包括不透明的上下文，并设置数据```

		xs = xfrm_input_state(skb);
		context = xs->xso.offload_handle;
		set up HW for send

```
协议栈已经在数据包数据中插入了适当IPsec 头部，卸载只需要进行加密并修正头部值

当收到一个数据包并且 HW 指示它卸载了解密时，驱动需要向数据包的 skb 添加一个对解码SA 的引用。此时数据应当已被解密，IPsec 头部仍在数据包数据中；它们稍后会在协议栈
上层xfrm_input() 中被移除```

		/* get spi, protocol, and destination IP from packet headers */
		xs = find xs from (spi, protocol, dest_IP)
		xfrm_state_hold(xs);

```
```

		sp = secpath_set(skb);
		if (!sp) return;
		sp->xvec[sp->len++] = xs;
		sp->olen++;

```
```

		xo = xfrm_offload(skb);
		xo->flags = CRYPTO_DONE;
		xo->status = crypto_status;

```
4. 像往常一样将数据包交napi_gro_receive()
ESN 模式下，xfrm_replay_advance_esn()（RX）和 xfrm_replay_overflow_offload_esn
（TX）调xdo_dev_state_advance_esn()。驱动将检查数据包序列号，并在需要时更新 HW ESN
状态机
数据包卸载模式：
HW 添加和删XFRM 头部。因此在 RX 路径中，如果 HW 报告成功，XFRM 协议栈被绕过。在
TX 路径中，数据包在没有额外头部且未加密的情况下离开内核，HW 负责执行它
SA 被用户移除时，会要求驱动xdo_dev_state_delete() xdo_dev_policy_delete()
禁用卸载。之后，在所有对该状态和策略的引用计数都被移除、并且任何剩余资源可以为卸载
状态清理之后，xdo_dev_state_free() xdo_dev_policy_free() 从一个垃圾回收例程中调用。驱动如何使用这些取决于特定的硬件需求
netdev 被设置为 DOWN 时，XFRM 协议栈的 netdev 监听器会对任何剩余的卸载状态调xdo_dev_state_delete()、xdo_dev_policy_delete()、xdo_dev_state_free() xdo_dev_policy_free()
由于 HW 处理数据包的结果，XFRM 核心无法计数硬限制、软限制。HW/驱动负责执行它，并在
调用 xdo_dev_state_update_stats() 时提供准确的数据。如果发生了这些限制之一，驱动需调用 xfrm_state_check_expire() 以确XFRM 执行重新密钥序列