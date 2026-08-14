## 缃戠粶鍒嗙被鍣?cgroup


缃戠粶鍒嗙被鍣紙Network classifier锛塩group 鎻愪緵浜嗕竴涓帴鍙ｏ紝鐢ㄤ簬涓虹綉缁滄暟鎹寘鎵撲笂绫诲埆鏍囪瘑绗︼紙classid锛夈€?
娴侀噺鎺у埗鍣紙Traffic Controller锛宼c锛夊彲鐢ㄤ簬涓烘潵鑷笉鍚?cgroup 鐨勬暟鎹寘鍒嗛厤涓嶅悓鐨勪紭鍏堢骇銆傛澶栵紝Netfilter锛坕ptables锛変篃鍙互鍒╃敤姝ゆ爣璁板杩欑被鏁版嵁鍖呮墽琛屾搷浣溿€?
鍒涘缓 net_cls cgroup 瀹炰緥浼氬垱寤轰竴涓?net_cls.classid 鏂囦欢銆傝 net_cls.classid 鍊煎垵濮嬪寲涓?0銆?
浣犲彲浠ュ悜 net_cls.classid 鍐欏叆鍗佸叚杩涘埗鍊硷紱杩欎簺鍊肩殑鏍煎紡涓?0xAAAABBBB锛屽叾涓?AAAA 鏄富鍙ユ焺鍙凤紙major handle number锛夛紝BBBB 鏄鍙ユ焺鍙凤紙minor handle number锛夈€傝鍙?net_cls.classid 寰楀埌鐨勬槸鍗佽繘鍒剁粨鏋溿€?
```

	mkdir /sys/fs/cgroup/net_cls
	mount -t cgroup -onet_cls net_cls /sys/fs/cgroup/net_cls
	mkdir /sys/fs/cgroup/net_cls/0
	echo 0x100001 >  /sys/fs/cgroup/net_cls/0/net_cls.classid

```
```

	cat /sys/fs/cgroup/net_cls/0/net_cls.classid
	1048577

```
```

	tc qdisc add dev eth0 root handle 10: htb
	tc class add dev eth0 parent 10: classid 10:1 htb rate 40mbit

```
```

	tc filter add dev eth0 parent 10: protocol ip prio 10 handle 1: cgroup

```
```

	iptables -A OUTPUT -m cgroup ! --cgroup 0x100001 -j DROP

```
