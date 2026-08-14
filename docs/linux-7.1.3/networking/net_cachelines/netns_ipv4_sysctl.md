如需进一步信息，请结合内核源码树中的对应实现与提交记录进行核对。

本页内容以结构化表格为主，表头与说明已译为中文，字段中的标识符与专有名称保留英文原文，以便与内核源码及文档交叉引用保持一致。

本页以表格形式列出网络命名空间（netns）中 IPv4 相关 sysctl 参数在结构体 netns_ipv4 内的字段布局与缓存行（cacheline）分布，便于理解各 sysctl 在内存中的热路径归属。字段名与标识符保留英文原文，说明文字已译为中文。


## netns_ipv4 结构体快速路径使用情况分解


=============================== ============================================ =================== =================== =================================================
类型                            Name                                         fastpath_tx_access  fastpath_rx_access  注释
=============================== ============================================ =================== =================== =================================================
结构体_inet_timewait_death_row  tcp_death_row
结构体_udp_表*               udp_表
结构体_ctl_表_header*        forw_hdr
结构体_ctl_表_header*        frags_hdr
结构体_ctl_表_header*        ipv4_hdr
结构体_ctl_表_header*        route_hdr
结构体_ctl_表_header*        xfrm4_hdr
结构体_ipv4_devconf*            devconf_全部
结构体_ipv4_devconf*            devconf_dflt
结构体_ip_ra_chain              ra_chain
结构体_互斥体                    ra_互斥体
结构体_fib_rules_ops*           rules_ops
结构体_fib_表                fib_主要
结构体_fib_表                fib_默认
unsigned_int                    fib_rules_需要_fldissect
bool                            fib_具有_custom_rules
bool                            fib_具有_custom_本地_routes
bool                            fib_offload_已禁用
原子_t                        fib_num_tclassid_users
结构体_hlist_head*              fib_表_hash
结构体_sock*                    fibnl
结构体_sock*                    mc_autojoin_sk
结构体_inet_peer_base*          peers
结构体_fqdir*                   fqdir
u8                              sysctl_icmp_echo_ignore_全部
u8                              sysctl_icmp_echo_启用_probe
u8                              sysctl_icmp_echo_ignore_broadcasts
u8                              sysctl_icmp_ignore_bogus_错误_responses
u8                              sysctl_icmp_错误_使用_inbound_ifaddr
int                             sysctl_icmp_ratelimit
int                             sysctl_icmp_ratemask
u32                             ip_rt_min_pmtu
int                             ip_rt_mtu_expires
int                             ip_rt_min_advmss
结构体_本地_ports              ip_本地_ports
u8                              sysctl_tcp_ecn
u8                              sysctl_tcp_ecn_fallback
u8                              sysctl_ip_默认_ttl                                                                ip4_dst_hoplimit/ip_select_ttl
u8                              sysctl_ip_无_pmtu_disc
u8                              sysctl_ip_fwd_使用_pmtu                       读取_mostly                             ip_dst_mtu_maybe_forward/ip_skb_dst_mtu
u8                              sysctl_ip_fwd_更新_优先级                                                        ip_forward
u8                              sysctl_ip_nonlocal_bind
u8                              sysctl_ip_autobind_reuse
u8                              sysctl_ip_dynaddr
u32                             sysctl_ip_本地_端口_step_width
u8                              sysctl_ip_early_demux                                            读取_mostly         ip(6)_rcv_finish_核心
u8                              sysctl_raw_l3mdev_accept
u8                              sysctl_tcp_early_demux                                           读取_mostly         ip(6)_rcv_finish_核心
u8                              sysctl_udp_early_demux
u8                              sysctl_nexthop_compat_模式
u8                              sysctl_fwmark_reflect
u8                              sysctl_tcp_fwmark_accept
u8                              sysctl_tcp_l3mdev_accept                                         读取_mostly         __inet6_lookup_established/inet_请求_bound_dev_若
u8                              sysctl_tcp_mtu_probing
int                             sysctl_tcp_mtu_probe_floor
int                             sysctl_tcp_base_mss
int                             sysctl_tcp_min_snd_mss                       读取_mostly                             __tcp_mtu_到_mss(tcp_写入_xmit)
int                             sysctl_tcp_probe_threshold                                                           tcp_mtu_probe(tcp_写入_xmit)
u32                             sysctl_tcp_probe_interval                                                            tcp_mtu_check_reprobe(tcp_写入_xmit)
int                             sysctl_tcp_keepalive_time
int                             sysctl_tcp_keepalive_intvl
u8                              sysctl_tcp_keepalive_probes
u8                              sysctl_tcp_syn_retries
u8                              sysctl_tcp_synack_retries
u8                              sysctl_tcp_syncookies                                                                generated_在_syn
u8                              sysctl_tcp_migrate_req                                                               reuseport
u8                              sysctl_tcp_comp_sack_nr                                                              __tcp_ack_snd_check
int                             sysctl_tcp_reordering                                            读取_mostly         tcp_可_raise_cwnd/tcp_cong_control
u8                              sysctl_tcp_retries1
u8                              sysctl_tcp_retries2
u8                              sysctl_tcp_orphan_retries
u8                              sysctl_tcp_tw_reuse                                                                  timewait_sock_ops
unsigned_int                    sysctl_tcp_tw_reuse_delay                                                            timewait_sock_ops
int                             sysctl_tcp_fin_超时                                                               TCP_最后_ACK/tcp_rcv_状态_进程
unsigned_int                    sysctl_tcp_notsent_lowat                     读取_mostly                             tcp_notsent_lowat/tcp_流_内存_free
u8                              sysctl_tcp_sack                                                                      tcp_syn_选项
u8                              sysctl_tcp_window_scaling                                                            tcp_syn_选项,tcp_parse_选项
u8                              sysctl_tcp_timestamps
u8                              sysctl_tcp_early_retrans                     读取_mostly                             tcp_schedule_loss_probe(tcp_写入_xmit)
u32                             sysctl_tcp_rto_max_ms
u8                              sysctl_tcp_recovery                                                                  tcp_fastretrans_alert
u8                              sysctl_tcp_thin_linear_timeouts                                                      tcp_retrans_timer(在_thin_streams)
u8                              sysctl_tcp_slow_启动_之后_idle                                                     unlikely(tcp_cwnd_validate-network-not-starved)
u8                              sysctl_tcp_retrans_collapse
u8                              sysctl_tcp_stdurg                                                                    unlikely(tcp_check_urg)
u8                              sysctl_tcp_rfc1337
u8                              sysctl_tcp_abort_在_overflow
u8                              sysctl_tcp_fack
int                             sysctl_tcp_max_reordering                                                            tcp_check_sack_reordering
int                             sysctl_tcp_adv_win_scale                                                             tcp_初始化_缓冲区_space
u8                              sysctl_tcp_dsack                                                                     partial_数据包_或_retrans_在_tcp_数据_队列
u8                              sysctl_tcp_app_win                                                                   tcp_win_来自_space
u8                              sysctl_tcp_frto                                                                      tcp_enter_loss
u8                              sysctl_tcp_nometrics_save                                                            TCP_最后_ACK/tcp_更新_metrics
u8                              sysctl_tcp_无_ssthresh_metrics_save                                                  TCP_最后_ACK/tcp_(更新/初始化)_metrics
u8                              sysctl_tcp_moderate_rcvbuf                                       读取_mostly         tcp_rcvbuf_grow()
u32                             sysctl_tcp_rcvbuf_low_rtt                                        读取_mostly         tcp_rcvbuf_grow()
u8                              sysctl_tcp_shrink_window                     读取_mostly         读取_mostly         __tcp_select_window()
u8                              sysctl_tcp_tso_win_divisor                   读取_mostly                             tcp_tso_应当_defer(tcp_写入_xmit)
u8                              sysctl_tcp_workaround_signed_windows                                                 tcp_select_window
int                             sysctl_tcp_limit_输出_bytes                读取_mostly                             tcp_small_队列_check(tcp_写入_xmit)
int                             sysctl_tcp_challenge_ack_limit
int                             sysctl_tcp_min_rtt_wlen                      读取_mostly                             tcp_ack_更新_rtt
u8                              sysctl_tcp_min_tso_segs                                                              unlikely(icsk_ca_ops-written)
u8                              sysctl_tcp_tso_rtt_log                       读取_mostly                             tcp_tso_autosize
u8                              sysctl_tcp_autocorking                       读取_mostly                             tcp_push/tcp_应当_autocork
u8                              sysctl_tcp_reflect_tos                                                               tcp_v(4/6)_send_synack
int                             sysctl_tcp_invalid_ratelimit
int                             sysctl_tcp_pacing_ss_ratio                                                           默认_cong_cont(tcp_更新_pacing_rate)
int                             sysctl_tcp_pacing_ca_ratio                                                           默认_cong_cont(tcp_更新_pacing_rate)
int                             sysctl_tcp_wmem[^3^]                           读取_mostly                             tcp_wmem_schedule(sendmsg/sendpage)
int                             sysctl_tcp_rmem[^3^]                                               读取_mostly         __tcp_grow_window(tx),tcp_rcv_space_adjust(rx)
unsigned_int                    sysctl_tcp_child_ehash_条目
unsigned_long                   sysctl_tcp_comp_sack_delay_ns                                                        __tcp_ack_snd_check
unsigned_long                   sysctl_tcp_comp_sack_slack_ns                                                        __tcp_ack_snd_check
int                             sysctl_max_syn_backlog
int                             sysctl_tcp_fastopen
结构体_tcp_congestion_ops       tcp_congestion_control                                                               初始化_cc
结构体_tcp_fastopen_上下文     tcp_fastopen_ctx
unsigned_int                    sysctl_tcp_fastopen_blackhole_超时
原子_t                        tfo_active_禁用_times
unsigned_long                   tfo_active_禁用_stamp
u32                             tcp_challenge_timestamp
u32                             tcp_challenge_count
u8                              sysctl_tcp_plb_已启用
u8                              sysctl_tcp_plb_idle_rehash_rounds
u8                              sysctl_tcp_plb_rehash_rounds
u8                              sysctl_tcp_plb_suspend_rto_sec
int                             sysctl_tcp_plb_cong_thresh
int                             sysctl_udp_wmem_min
int                             sysctl_udp_rmem_min
u8                              sysctl_fib_notify_在_标志_change
u8                              sysctl_udp_l3mdev_accept
u8                              sysctl_igmp_llm_reports
int                             sysctl_igmp_max_memberships
int                             sysctl_igmp_max_msf
int                             sysctl_igmp_qrv
结构体_ping_group_range         ping_group_range
原子_t                        dev_addr_genid
unsigned_int                    sysctl_udp_child_hash_条目
unsigned_long*                  sysctl_本地_reserved_ports
int                             sysctl_ip_prot_sock
结构体_mr_表*                mrt
结构体_列出_head                mr_表
结构体_fib_rules_ops*           mr_rules_ops
u32                             sysctl_fib_multipath_hash_字段
u8                              sysctl_fib_multipath_使用_neigh
u8                              sysctl_fib_multipath_hash_policy
结构体_fib_notifier_ops*        notifier_ops
unsigned_int                    fib_seq
结构体_fib_notifier_ops*        ipmr_notifier_ops
unsigned_int                    ipmr_seq
原子_t                        rt_genid
siphash_key_t                   ip_id_key
=============================== ============================================ =================== =================== =================================================
