
## IP动态地址 hack-port v0.03


这个东西允许通过以下方式建立拨打ONESHOT 连接
动态更改数据包源地址（以及套接字的本地进程）
它是针对 TCP 拨号盒连(1) IP_MASQuerading(2) 实现的

如果启用\ [#]_并且转发接口已更改：

1) 套接字（和数据包）源地址在重传时被重
处于 SYN_SENT 状态时（拨号框进程）
2) 出界 MASQueraded 源地址更改 ON OUTPUT（当
内部主机进行重传）直到来自外部的数据包被
由隧道接收

这对于自动拨号链(diaald) 特别有帮助，其中
`actual` 传出地址目前未知
上升。因此，**相同**（本地和伪装）连接请
建立链接就可以了


```

     # echo 1 > /proc/sys/net/ipv4/ip_dynaddr

  To enable verbose mode::

    # echo 2 > /proc/sys/net/ipv4/ip_dynaddr

  To disable (default)::

     # echo 0 > /proc/sys/net/ipv4/ip_dynaddr

```
享受

鑳″畨涔?<jjciarla@raiz.uncu.edu.ar>
