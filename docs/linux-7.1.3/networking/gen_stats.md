
## 面向 netlink 用户的通用网络统计


统计计数器被分组到结构体中：

==================== ===================== =====================
Struct               TLV type              描述
==================== ===================== =====================
gnet_stats_basic     TCA_STATS_BASIC       基本统计
gnet_stats_rate_est  TCA_STATS_RATE_EST    速率估计器
gnet_stats_queue     TCA_STATS_QUEUE       队列统计
none                 TCA_STATS_APP         应用特定
==================== ===================== =====================


### 收集：


```

	struct mystruct {
		struct gnet_stats_basic	bstats;
		struct gnet_stats_queue	qstats;
		...
	};

```
```

	mystruct->tstats.packet++;
	mystruct->qstats.backlog += skb->pkt_len;


```
### 导出到用户空间（Dump）：


```

    my_dumping_routine(struct sk_buff *skb, ...)
    {
	    struct gnet_dump dump;

	    if (gnet_stats_start_copy(skb, TCA_STATS2, &mystruct->lock, &dump,
				    TCA_PAD) < 0)
		    goto rtattr_failure;

	    if (gnet_stats_copy_basic(&dump, &mystruct->bstats) < 0 ||
		gnet_stats_copy_queue(&dump, &mystruct->qstats) < 0 ||
		    gnet_stats_copy_app(&dump, &xstats, sizeof(xstats)) < 0)
		    goto rtattr_failure;

	    if (gnet_stats_finish_copy(&dump) < 0)
		    goto rtattr_failure;
	    ...
    }

```
### TCA_STATS/TCA_XSTATS 向后兼容性：


struct tc_stats 和 xstats 的早期使用者可以通过调用兼容包装函数来保持向后兼容性，以继续提供
```

    my_dumping_routine(struct sk_buff *skb, ...)
    {
	if (gnet_stats_start_copy_compat(skb, TCA_STATS2, TCA_STATS,
					TCA_XSTATS, &mystruct->lock, &dump,
					TCA_PAD) < 0)
		    goto rtattr_failure;
	    ...
    }

```
一个 struct tc_stats 将在 gnet_stats_copy_* 调用期间被填充并追加到 skb。如果调用了
gnet_stats_copy_app，则提供 TCA_XSTATS。


### 加锁：


在写入前获取锁，并在所有统计写入完成后释放。在发生错误的情况下锁也总是被释放。你有责任确保
锁已初始化。


### 速率估计器：


0) 准备一个估计器属性。这很可能在用户空间完成。此 TLV 的值应包含 tc_estimator 结构。像往常
   一样，这样的 TLV 需要 32 位对齐，因此长度需要适当设置等。估计器间隔和 ewma 对数需要转换为
   适当的值。建议使用 tc_estimator.c::tc_setup_estimator() 作为转换例程。它做了一些巧妙的事情。
   它接受一个以微秒为单位的时间间隔、同样以微秒为单位的时间常数，以及一个要填充的 struct
   tc_estimator。返回的 tc_estimator 可以被传输到内核。通过类型为 TCA_RATE 的 TLV 将该结构
   传输到你在内核中的代码。

在内核中设置时：

1) 确保你首先已设置基本统计和速率统计。
2) 确保你已初始化用于设置此类统计的 stats 锁。
```

    int ret = gen_new_estimator(my_basicstats,my_rate_est_stats,
	mystats_lock, attr_with_tcestimator_struct);

    if ret == 0
	success
    else
	failed

```
从现在起，每次你 dump my_rate_est_stats 时，它将包含最新的信息。

完成后，调用 gen_kill_estimator(my_basicstats, my_rate_est_stats)。确保在进行此调用时
my_basicstats 和 my_rate_est_stats 仍然有效（即仍然存在）。


### 作者：


- Thomas Graf <tgraf@suug.ch>
- Jamal Hadi Salim <hadi@cyberus.ca>
