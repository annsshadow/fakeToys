
## 6lowpan 接口的 netdev 私有数据区


所有支持 6lowpan 的网络设备，即所有具有 ARPHRD_6LOWPAN 的接口，必须在
netdev_priv 的开头放置 "struct lowpan_priv"。

```

 dev->priv_size = LOWPAN_PRIV_SIZE(LL_6LOWPAN_PRIV_DATA);

```
其中 LL_PRIV_6LOWPAN_DATA 是 sizeof linklayer 6lowpan 私有数据结构。
```

 lowpan_priv(dev)-priv;

```
指向你的 LL_6LOWPAN_PRIV_DATA 结构。

```

 lowpan_netdev_setup(dev, LOWPAN_LLTYPE_FOOBAR);

```
其中 LOWPAN_LLTYPE_FOOBAR 是你的 6LoWPAN 链路层类型的 enum lowpan_lltypes 中的
一个 define。

```

 static inline struct lowpan_priv_foobar *
 lowpan_foobar_priv(struct net_device *dev)
 {
	return (struct lowpan_priv_foobar *)lowpan_priv(dev)->priv;
 }

 switch (dev->type) {
 case ARPHRD_6LOWPAN:
	lowpan_priv = lowpan_priv(dev);
	/* 做一些与 ARPHRD_6LOWPAN 相关的很棒的事 */
	switch (lowpan_priv->lltype) {
	case LOWPAN_LLTYPE_FOOBAR:
		/* 在此处理 802.15.4 6LoWPAN */
		lowpan_foobar_priv(dev)->bar = foo;
		break;
	...
	}
	break;
 ...
 }

```
对于通用的 6lowpan 分支（"net/6lowpan"），你可以去掉对 ARPHRD_6LOWPAN 的检查，
因为你可以确定这些函数是由 ARPHRD_6LOWPAN 接口调用的。
