
## Seg6 Sysfs 变量



## /proc/sys/net/conf/<iface>/seg6_* 变量：


seg6_enabled - BOOL
在此接口上接受或丢弃启用 SR 的 IPv6 数据包。

相关数据包是具有 SRH 且 DA = 本地的数据包。

 - 0 - 禁用（默认）
 - 非 0 - 启用

seg6_require_hmac - 整数
为该接口上启用 SR 的入口数据包定义 HMAC 策略。

 - -1 - 忽略 HMAC 字段
 - 0 - 接受不带 HMAC 的 SR 数据包，使用 HMAC 验证 SR 数据包
 - 1 - 丢弃没有 HMAC 的 SR 数据包，使用 HMAC 验证 SR 数据包

默认值为 0。

## /proc/sys/net/ipv6/seg6_* 变量：


seg6_flowlabel - 整数
控制计算外部流标签的行为
SR T.encaps 情况下的 IPv6 标头

	 == =======================================================
-1 将流标签设置为零。
0 在内部 IPv6 的情况下从内部数据包复制流标签
（如果是 IPv4/L2，则将 flowlabel 设置为 0）
1 使用 seg6_make_flowlabel() 计算 flowlabel
	 == =======================================================

默认值为 0。
