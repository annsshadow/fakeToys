
## 如何mac80211 中使用包注入


mac80211 现在允许从用户空间向任意 Monitor Mode（监听模式）接口注入任意数据包。你注入数据包需要按如下方式组织
```
 [ radiotap header  ]
 [ ieee80211 header ]
 [ payload ]

```
radiotap 格式./Documentation/networking/radiotap-headers.rst
中有讨论
尽管目前定义了众radiotap 参数，但其中大部分只在接收的数据包上才有意义。以下信息会radiotap 头部解析出来，并用于控制注入过程
 - IEEE80211_RADIOTAP_FLAGS

   =========================  ===========================================
   IEEE80211_RADIOTAP_F_FCS   FCS 将被移除并重新计   IEEE80211_RADIOTAP_F_WEP   如果存在密钥，将对帧进行加密
   IEEE80211_RADIOTAP_F_FRAG  如果帧长于当前的分片阈值，将对其进行分   =========================  ===========================================

 - IEEE80211_RADIOTAP_TX_FLAGS

   =============================  ========================================
   IEEE80211_RADIOTAP_F_TX_NOACK  即使是非广播的单播帧，发送时也不等待 ACK
   =============================  ========================================

 - IEEE80211_RADIOTAP_RATE

   传输所用的传统速率（仅适用于没有自身速率控制的设备）

 - IEEE80211_RADIOTAP_MCS

   传输所用的 HT 速率（仅适用于没有自身速率控制的设备）。同时也会解析部分标
   ============================  ========================
   IEEE80211_RADIOTAP_MCS_SGI    使用短保护间   IEEE80211_RADIOTAP_MCS_BW_40  HT40 模式发   ============================  ========================

 - IEEE80211_RADIOTAP_DATA_RETRIES

   当使用了 IEEE80211_RADIOTAP_RATE IEEE80211_RADIOTAP_MCS 时的重试次数

 - IEEE80211_RADIOTAP_VHT

   传输所用的 VHT mcs 以及流的数量（仅适用于没有自身速率控制的设备）。同时也会解析其他字
   flags 字段
	IEEE80211_RADIOTAP_VHT_FLAG_SGI：使用短保护间隔

   bandwidth 字段
 - 1：以 40MHz 信道宽度发 - 4：以 80MHz 信道宽度发 - 11：以 160MHz 信道宽度发
注入代码还可以跳过所有其它当前已定义radiotap 字段，从而便于直接重放所捕获radiotap
头部
```

	0x00, 0x00, // <-- radiotap 版本
	0x0b, 0x00, // <- radiotap 头部长度
	0x04, 0x0c, 0x00, 0x00, // <-- 位图
	0x6c, // <-- 速率
	0x0c, //<-- 发射功率
	0x01 //<-- 天线

```
随后紧跟着 ieee80211 头部，例如：

```

	0x08, 0x01, 0x00, 0x00,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0x13, 0x22, 0x33, 0x44, 0x55, 0x66,
	0x13, 0x22, 0x33, 0x44, 0x55, 0x66,
	0x10, 0x86

```
最后是载荷
在构造好数据包内容后，通过 send() 将其发送到处于 Monitor 模式mac80211 逻辑接口即可也可以使Libpcap（这比自行将套接字绑定到正确接口要容易）
```

	ppcap = pcap_open_live(szInterfaceName, 800, 1, 20, szErrbuf);
	...
	r = pcap_inject(ppcap, u8aSendBuffer, nLength);

```
你还可以在此处找到一个完整注入应用的链接
https://wireless.wiki.kernel.org/en/users/Documentation/packetspammer

Andy Green <andy@warmcat.com>
