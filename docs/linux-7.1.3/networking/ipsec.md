
## IPsec


此处记录已知IPsec 边界情况，在真实生产环境中部署各IPsec 配置时需要牢记
1. IPcomp:
	   较小IP 报文在发送端不会被压缩，并在接收端的策略检查中失败
```

  2.2. 闈炴墿灞曠瓥鐣?
   If the total size of a compressed payload and the IPComp header, as
   defined in section 3, is not smaller than the size of the original
   payload, the IP datagram MUST be sent in the original non-compressed
   form.  To clarify: If an IP datagram is sent non-compressed, no

   IPComp header is added to the datagram.  This policy ensures saving
   the decompression processing cycles and avoiding incurring IP
   datagram fragmentation when the expanded datagram is larger than the
   MTU.

   Small IP datagrams are likely to expand as a result of compression.
   Therefore, a numeric threshold should be applied before compression,
   where IP datagrams of size smaller than the threshold are sent in the
   original form without attempting compression.  The numeric threshold
   is implementation dependent.

```
当前的实现确实严格遵循规范，但在实际中，当向对端发送未压缩报文时（无论报文长度是否小于阈值，或压缩后长度大于原始报文长度），该报文在策略检查处被丢弃，因为它匹配选择器但并非来自任何 XFRM 层，即没有安全路径。这种裸报文最终无法送达上层当用户使用不同载荷长ping 对端时，结果对用户来说更加诡异
一种变通方法是在观察到上述场景时，为每个策略尝试设置“level use”。这样做的结果是小报文（未压缩）将在接收端跳过策略检查