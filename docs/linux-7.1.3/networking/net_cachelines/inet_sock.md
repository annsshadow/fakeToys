
## inet_sock 缁撴瀯浣撳揩璺緞浣跨敤鍒嗚В

鏈〉浠ヨ〃鏍煎舰寮忓垎瑙ｇ綉缁滃瓙绯荤粺涓?inet_sock 缁撴瀯浣撶殑瀛楁甯冨眬涓庣紦瀛樿锛坈acheline锛夊垎甯冿紝鏍囨敞鍚勫瓧娈靛湪鍙戦€侊紙tx锛変笌鎺ユ敹锛坮x锛夊揩璺緞涓殑璁块棶鎯呭喌銆備緵鍐呮牳缃戠粶寮€鍙戣€呯悊瑙ｇ粨鏋勪綋瀛楁鐨勫唴瀛樼儹璺緞褰掑睘锛岃緟鍔╃紦瀛樺弸濂芥€т紭鍖栥€?


鏈枃妗ｄ负鑷姩鐢熸垚鐨勭粨鏋勫寲鏉＄洰绱㈠紩锛屾潯鐩悕绉颁互鑻辨枃鏈淇濈暀浠ヤ究妫€绱€?


======================== ===================== =================== =================== ======================================================================================================
绫诲瀷 鍚嶇О fastpath_tx_access fastpath_rx_access 璇存槑
======================== ===================== =================== =================== ======================================================================================================
缁撴瀯浣?sock sk read_mostly read_mostly tcp_init_buffer_space,tcp_init_transfer,tcp_finish_connect,tcp_connect,tcp_send_rcvq,tcp_send_syn_data
缁撴瀯浣?ipv6_pinfo* pinet6
缁撴瀯浣?ipv6_fl_socklist* ipv6_fl_list read_mostly tcp_v6_connect,__ip6_datagram_connect,udpv6_sendmsg,rawv6_sendmsg
be16 inet_sport read_mostly __tcp_transmit_skb
be32 inet_daddr read_mostly ip_select_ident_segs
be32 inet_rcv_saddr
be16 inet_dport read_mostly __tcp_transmit_skb
u16 inet_num
be32 inet_saddr
s16 uc_ttl read_mostly __ip_queue_xmit/ip_select_ttl
u16 cmsg_flags
缁撴瀯浣?ip_options_rcu* inet_opt read_mostly __ip_queue_xmit
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
缁撴瀯浣?ip_mc_socklist* mc_list
缁撴瀯浣?inet_cork_full cork read_mostly __tcp_transmit_skb
缁撴瀯浣?local_port_range
======================== ===================== =================== =================== ======================================================================================================
