
## 如何使用 radiotap 头


### 指向 radiotap 头包含文件


Radiotap 头是变长且可扩展的，你可以从以下位置获取大部分
```

    ./include/net/ieee80211_radiotap.h

```
本文档给出一个概览，并就一些边界情况给出警示。


### 头的结构


开头有一个固定部分，其中包含一个 u32 位图，用于定义与每个位关联的可能参数是否存在。
因此，如果 ieee80211_radiotap_header 的 it_present 成员的 b0 被置位，就意味着参数区中
存在参数索引 0（IEEE80211_RADIOTAP_TSFT）的头。

```

   < 8-byte ieee80211_radiotap_header >
   [ <possible argument bitmap extensions ... > ]
   [ <argument> ... ]

```
目前只定义了 13 个可能的参数索引，但如果我们用完了 u32 it_present 成员中的空间，规定
b31 置位表示后面还有一个 u32 位图（如上所示“可能的参数位图扩展...”），并且每次参数
的起点向前移动 4 字节。

还要注意，it_len 成员 __le16 被设为 ieee80211_radiotap_header 及其后任何参数所覆盖的
总字节数。


### 对参数的要求


在头的固定部分之后，是那些在 ieee80211_radiotap_header 的 it_present 成员中对应位被
置位的每个参数索引的参数，依次跟随。

 - 所有参数都以小端（little-endian）方式存储！

 - 给定参数索引的参数载荷具有固定大小。因此 IEEE80211_RADIOTAP_TSFT 存在总是表示存在一个
   8 字节参数。参见 ./include/net/ieee80211_radiotap.h 中的注释，了解所有参数大小的
   清晰细分。

 - 参数必须使用填充（padding）对齐到参数大小所在边界。因此一个 u16 参数若尚未处于 u16
   边界上，则必须开始于下一个 u16 边界；u32 必须开始于下一个 u32 边界，依此类推。

 - “对齐”是相对于 ieee80211_radiotap_header 的起点（即 radiotap 头的第一个字节）而言的。
   该第一个字节的绝对对齐方式并未定义。因此即使整个 radiotap 头开始于例如地址 0x00000003，
   出于对齐目的，该 radiotap 头的第一个字节仍被视为 0。

 - 上面一点说明，在固定的 radiotap 头或参数区中，多字节实体可能没有任何绝对对齐，这意味着
   你在尝试访问这些多字节实体时必须采取特殊的规避措施。某些架构（如 Blackfin）无法处理
   对指向奇数地址的 u16 指针的解引用。相反，你必须使用内核 API get_unaligned() 来解引用该
   指针，在需要这样做的架构上它会逐字节地进行。

 - 给定参数索引的参数可以是多种类型组合在一起。例如 IEEE80211_RADIOTAP_CHANNEL 的参数载荷
   由两个 u16 组成，总长度为 4。当这种情况发生时，应用的是处理 u16 的填充规则，而不是处理
   4 字节单一实体的规则。


### 示例：有效的 radiotap 头


```

	0x00, 0x00, // <-- radiotap version + pad byte
	0x0b, 0x00, // <- radiotap header length
	0x04, 0x0c, 0x00, 0x00, // <-- bitmap
	0x6c, // <-- rate (in 500kHz units)
	0x0c, //<-- tx power
	0x01 //<-- antenna


```
### 使用 Radiotap 解析器


如果你需要解析一个 radiotap 结构，可以使用位于 net/wireless/radiotap.c 中的 radiotap
解析器大幅简化工作，它拥有
```

    #include <net/cfg80211.h>

    /* buf points to the start of the radiotap header part */

    int MyFunction(u8 * buf, int buflen)
    {
	    int pkt_rate_100kHz = 0, antenna = 0, pwr = 0;
	    struct ieee80211_radiotap_iterator iterator;
	    int ret = ieee80211_radiotap_iterator_init(&iterator, buf, buflen);

	    while (!ret) {

		    ret = ieee80211_radiotap_iterator_next(&iterator);

		    if (ret)
			    continue;

		    /* see if this argument is something we can use */

		    switch (iterator.this_arg_index) {
		    /*
		    * You must take care when dereferencing iterator.this_arg
		    * for multibyte types... the pointer is not aligned.  Use
		    * get_unaligned((type *)iterator.this_arg) to dereference
		    * iterator.this_arg for type "type" safely on all arches.
		    */
		    case IEEE80211_RADIOTAP_RATE:
			    /* radiotap "rate" u8 is in
			    * 500kbps units, eg, 0x02=1Mbps
			    */
			    pkt_rate_100kHz = (*iterator.this_arg) * 5;
			    break;

		    case IEEE80211_RADIOTAP_ANTENNA:
			    /* radiotap uses 0 for 1st ant */
			    antenna = *iterator.this_arg);
			    break;

		    case IEEE80211_RADIOTAP_DBM_TX_POWER:
			    pwr = *iterator.this_arg;
			    break;

		    default:
			    break;
		    }
	    }  /* while more rt headers */

	    if (ret != -ENOENT)
		    return TXRX_DROP;

	    /* discard the radiotap header part */
	    buf += iterator.max_length;
	    buflen -= iterator.max_length;

	    ...

    }

```
Andy Green <andy@warmcat.com>
