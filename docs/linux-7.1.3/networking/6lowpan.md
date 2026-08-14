
## 6lowpan 鎺ュ彛鐨?netdev 绉佹湁鏁版嵁鍖?

鎵€鏈夋敮鎸?6lowpan 鐨勭綉缁滆澶囷紝鍗虫墍鏈夊叿鏈?ARPHRD_6LOWPAN 鐨勬帴鍙ｏ紝蹇呴』鍦?netdev_priv 鐨勫紑澶存斁缃?"struct lowpan_priv"銆?
```

 dev->priv_size = LOWPAN_PRIV_SIZE(LL_6LOWPAN_PRIV_DATA);

```
鍏朵腑 LL_PRIV_6LOWPAN_DATA 鏄?sizeof linklayer 6lowpan 绉佹湁鏁版嵁缁撴瀯銆?```

 lowpan_priv(dev)-priv;

```
鎸囧悜浣犵殑 LL_6LOWPAN_PRIV_DATA 缁撴瀯銆?
```

 lowpan_netdev_setup(dev, LOWPAN_LLTYPE_FOOBAR);

```
鍏朵腑 LOWPAN_LLTYPE_FOOBAR 鏄綘鐨?6LoWPAN 閾捐矾灞傜被鍨嬬殑 enum lowpan_lltypes 涓殑
涓€涓?define銆?
```

 static inline struct lowpan_priv_foobar *
 lowpan_foobar_priv(struct net_device *dev)
 {
	return (struct lowpan_priv_foobar *)lowpan_priv(dev)->priv;
 }

 switch (dev->type) {
 case ARPHRD_6LOWPAN:
	lowpan_priv = lowpan_priv(dev);
	/* 鍋氫竴浜涗笌 ARPHRD_6LOWPAN 鐩稿叧鐨勫緢妫掔殑浜?*/
	switch (lowpan_priv->lltype) {
	case LOWPAN_LLTYPE_FOOBAR:
		/* 鍦ㄦ澶勭悊 802.15.4 6LoWPAN */
		lowpan_foobar_priv(dev)->bar = foo;
		break;
	...
	}
	break;
 ...
 }

```
瀵逛簬閫氱敤鐨?6lowpan 鍒嗘敮锛?net/6lowpan"锛夛紝浣犲彲浠ュ幓鎺夊 ARPHRD_6LOWPAN 鐨勬鏌ワ紝
鍥犱负浣犲彲浠ョ‘瀹氳繖浜涘嚱鏁版槸鐢?ARPHRD_6LOWPAN 鎺ュ彛璋冪敤鐨勩€?