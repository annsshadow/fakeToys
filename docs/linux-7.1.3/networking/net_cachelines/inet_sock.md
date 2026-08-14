
## inet_sock 结构体快路径使用分解

本页以表格形式分解网络子系统中 inet_sock 结构体的字段布局与缓存行（cacheline）分布，标注各字段在发送（tx）与接收（rx）快路径中的访问情况。供内核网络开发者理解结构体字段的内存热路径归属，辅助缓存友好性优化。


本文档为自动生成的结构化条目索引，条目名称以英文术语保留以便检索。


======================== ===================== =================== =================== ======================================================================================================
类型 名称 fastpath_tx_access fastpath_rx_access 说明
======================== ===================== =================== =================== ======================================================================================================
结构体 sock sk read_mostly read_mostly tcp_init_buffer_space,tcp_init_transfer,tcp_finish_connect,tcp_connect,tcp_send_rcvq,tcp_send_syn_data
结构体 ipv6_pinfo* pinet6
结构体 ipv6_fl_socklist* ipv6_fl_list read_mostly tcp_v6_connect,__ip6_datagram_connect,udpv6_sendmsg,rawv6_sendmsg
be16 inet_sport read_mostly __tcp_transmit_skb
be32 inet_daddr read_mostly ip_select_ident_segs
be32 inet_rcv_saddr
be16 inet_dport read_mostly __tcp_transmit_skb
u16 inet_num
be32 inet_saddr
s16 uc_ttl read_mostly __ip_queue_xmit/ip_select_ttl
u16 cmsg_flags
结构体 ip_options_rcu* inet_opt read_mostly __ip_queue_xmit
u16 inet_id read_mostly ip_select_ident_segs
u8 tos read_mostly ip_queue_xmit
u8 min_ttl
u8 mc_ttl
u8 pmtudisc
u8:1 recverr
u8:1 is_icsk
u8:1 freebind
u8:1 hdrincl
u8:1 mc_loop
u8:1 transparent
u8:1 mc_all
u8:1 nodefrag
u8:1 bind_address_no_port
u8:1 recverr_rfc4884
u8:1 defer_connect read_mostly tcp_sendmsg_fastopen
u8 rcv_tos
u8 convert_csum
int uc_index
int mc_index
be32 mc_addr
结构体 ip_mc_socklist* mc_list
结构体 inet_cork_full cork read_mostly __tcp_transmit_skb
结构体 local_port_range
======================== ===================== =================== =================== ======================================================================================================
