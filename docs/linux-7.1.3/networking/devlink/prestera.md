
## prestera devlink 支持


本文档描述由 `prestera` 设备驱动实现devlink 特性
## 驱动特定Traps


   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `arp_bc`
     - `trap`
     - 捕获 ARP 广播包（请求与响应）
   - - `is_is`
     - `trap`
     - 捕获 IS-IS    - - `ospf`
     - `trap`
     - 捕获 OSPF    - - `ip_bc_mac`
     - `trap`
     - 捕获目的 MAC 地址为广播地址IPv4    - - `stp`
     - `trap`
     - 捕获 STP BPDU
   - - `lacp`
     - `trap`
     - 捕获 LACP    - - `lldp`
     - `trap`
     - 捕获 LLDP    - - `router_mc`
     - `trap`
     - 捕获组播   - - `vrrp`
     - `trap`
     - 捕获 VRRP    - - `dhcp`
     - `trap`
     - 捕获 DHCP    - - `mtu_error`
     - `trap`
     - 捕获超出端口 MTU 的（异常）包
   - - `mac_to_me`
     - `trap`
     - 捕获目的 MAC 地址为交换端口地址的包
   - - `ttl_error`
     - `trap`
     - 捕获 TTL 超时的（异常）IPv4    - - `ipv4_options`
     - `trap`
     - IPv4 头选项格式错误而捕获的（异常）   - - `ip_default_route`
     - `trap`
     - 捕获没有特定 IP 接口（IP to me）也没有转发前缀的包
   - - `local_route`
     - `trap`
     - 捕获发往某个交换 IP 接口地址的包
   - - `ipv4_icmp_redirect`
     - `trap`
     - 捕获（异常）IPv4 ICMP 重定向包
   - - `arp_response`
     - `trap`
     - 捕获目的 MAC 地址为交换端口地址ARP 应答   - - `acl_code_0`
     - `trap`
     - 捕获 ACL 优先级为 0（tc pref 0）的   - - `acl_code_1`
     - `trap`
     - 捕获 ACL 优先级为 1（tc pref 1）的   - - `acl_code_2`
     - `trap`
     - 捕获 ACL 优先级为 2（tc pref 2）的   - - `acl_code_3`
     - `trap`
     - 捕获 ACL 优先级为 3（tc pref 3）的   - - `acl_code_4`
     - `trap`
     - 捕获 ACL 优先级为 4（tc pref 4）的   - - `acl_code_5`
     - `trap`
     - 捕获 ACL 优先级为 5（tc pref 5）的   - - `acl_code_6`
     - `trap`
     - 捕获 ACL 优先级为 6（tc pref 6）的   - - `acl_code_7`
     - `trap`
     - 捕获 ACL 优先级为 7（tc pref 7）的   - - `ipv4_bgp`
     - `trap`
     - 捕获 IPv4 BGP    - - `ssh`
     - `trap`
     - 捕获 SSH    - - `telnet`
     - `trap`
     - 捕获 Telnet    - - `icmp`
     - `trap`
     - 捕获 ICMP    - - `rxdma_drop`
     - `drop`
     - 因缺少入口缓冲区等而丢弃包（RxDMA   - - `port_no_vlan`
     - `drop`
     - 因网络配置错误或内部 bug（配置问题）而丢弃包
   - - `local_port`
     - `drop`
     - 丢弃决策（FDB 表项）为将包桥接回入口端trunk 的包
   - - `invalid_sa`
     - `drop`
     - 丢弃MAC 地址为组播的   - - `illegal_ip_addr`
     - `drop`
     - 丢弃目的 IP 为非法组单播地址的包
   - - `illegal_ipv4_hdr`
     - `drop`
     - 丢弃 IPv4 头非法的   - - `ip_uc_dip_da_mismatch`
     - `drop`
     - 丢弃目的 MAC 为单播、但目的 IP 为组播的   - - `ip_sip_is_zero`
     - `drop`
     - 丢弃 IPv4 源地址0 的包
   - - `met_red`
     - `drop`
     - 丢弃不合规的包（被入口限速器丢弃，计量丢弃），例如包速率超出配置的带