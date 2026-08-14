濡傞渶杩涗竴姝ヤ俊鎭紝璇风粨鍚堝唴鏍告簮鐮佹爲涓殑瀵瑰簲瀹炵幇涓庢彁浜よ褰曡繘琛屾牳瀵广€?

鏈〉鍐呭浠ョ粨鏋勫寲琛ㄦ牸涓轰富锛岃〃澶翠笌璇存槑宸茶瘧涓轰腑鏂囷紝瀛楁涓殑鏍囪瘑绗︿笌涓撴湁鍚嶇О淇濈暀鑻辨枃鍘熸枃锛屼互渚夸笌鍐呮牳婧愮爜鍙婃枃妗ｄ氦鍙夊紩鐢ㄤ繚鎸佷竴鑷淬€?

鏈〉浠ヨ〃鏍煎舰寮忓垪鍑虹綉缁滃懡鍚嶇┖闂达紙netns锛変腑 IPv4 鐩稿叧 sysctl 鍙傛暟鍦ㄧ粨鏋勪綋 netns_ipv4 鍐呯殑瀛楁甯冨眬涓庣紦瀛樿锛坈acheline锛夊垎甯冿紝渚夸簬鐞嗚В鍚?sysctl 鍦ㄥ唴瀛樹腑鐨勭儹璺緞褰掑睘銆傚瓧娈靛悕涓庢爣璇嗙淇濈暀鑻辨枃鍘熸枃锛岃鏄庢枃瀛楀凡璇戜负涓枃銆?


## netns_ipv4 缁撴瀯浣撳揩閫熻矾寰勪娇鐢ㄦ儏鍐靛垎瑙?


=============================== ============================================ =================== =================== =================================================
绫诲瀷                            Name                                         fastpath_tx_access  fastpath_rx_access  娉ㄩ噴
=============================== ============================================ =================== =================== =================================================
缁撴瀯浣揰inet_timewait_death_row  tcp_death_row
缁撴瀯浣揰udp_琛?               udp_琛?
缁撴瀯浣揰ctl_琛╛header*        forw_hdr
缁撴瀯浣揰ctl_琛╛header*        frags_hdr
缁撴瀯浣揰ctl_琛╛header*        ipv4_hdr
缁撴瀯浣揰ctl_琛╛header*        route_hdr
缁撴瀯浣揰ctl_琛╛header*        xfrm4_hdr
缁撴瀯浣揰ipv4_devconf*            devconf_鍏ㄩ儴
缁撴瀯浣揰ipv4_devconf*            devconf_dflt
缁撴瀯浣揰ip_ra_chain              ra_chain
缁撴瀯浣揰浜掓枼浣?                   ra_浜掓枼浣?
缁撴瀯浣揰fib_rules_ops*           rules_ops
缁撴瀯浣揰fib_琛?               fib_涓昏
缁撴瀯浣揰fib_琛?               fib_榛樿
unsigned_int                    fib_rules_闇€瑕乢fldissect
bool                            fib_鍏锋湁_custom_rules
bool                            fib_鍏锋湁_custom_鏈湴_routes
bool                            fib_offload_宸茬鐢?
鍘熷瓙_t                        fib_num_tclassid_users
缁撴瀯浣揰hlist_head*              fib_琛╛hash
缁撴瀯浣揰sock*                    fibnl
缁撴瀯浣揰sock*                    mc_autojoin_sk
缁撴瀯浣揰inet_peer_base*          peers
缁撴瀯浣揰fqdir*                   fqdir
u8                              sysctl_icmp_echo_ignore_鍏ㄩ儴
u8                              sysctl_icmp_echo_鍚敤_probe
u8                              sysctl_icmp_echo_ignore_broadcasts
u8                              sysctl_icmp_ignore_bogus_閿欒_responses
u8                              sysctl_icmp_閿欒_浣跨敤_inbound_ifaddr
int                             sysctl_icmp_ratelimit
int                             sysctl_icmp_ratemask
u32                             ip_rt_min_pmtu
int                             ip_rt_mtu_expires
int                             ip_rt_min_advmss
缁撴瀯浣揰鏈湴_ports              ip_鏈湴_ports
u8                              sysctl_tcp_ecn
u8                              sysctl_tcp_ecn_fallback
u8                              sysctl_ip_榛樿_ttl                                                                ip4_dst_hoplimit/ip_select_ttl
u8                              sysctl_ip_鏃燺pmtu_disc
u8                              sysctl_ip_fwd_浣跨敤_pmtu                       璇诲彇_mostly                             ip_dst_mtu_maybe_forward/ip_skb_dst_mtu
u8                              sysctl_ip_fwd_鏇存柊_浼樺厛绾?                                                       ip_forward
u8                              sysctl_ip_nonlocal_bind
u8                              sysctl_ip_autobind_reuse
u8                              sysctl_ip_dynaddr
u32                             sysctl_ip_鏈湴_绔彛_step_width
u8                              sysctl_ip_early_demux                                            璇诲彇_mostly         ip(6)_rcv_finish_鏍稿績
u8                              sysctl_raw_l3mdev_accept
u8                              sysctl_tcp_early_demux                                           璇诲彇_mostly         ip(6)_rcv_finish_鏍稿績
u8                              sysctl_udp_early_demux
u8                              sysctl_nexthop_compat_妯″紡
u8                              sysctl_fwmark_reflect
u8                              sysctl_tcp_fwmark_accept
u8                              sysctl_tcp_l3mdev_accept                                         璇诲彇_mostly         __inet6_lookup_established/inet_璇锋眰_bound_dev_鑻?
u8                              sysctl_tcp_mtu_probing
int                             sysctl_tcp_mtu_probe_floor
int                             sysctl_tcp_base_mss
int                             sysctl_tcp_min_snd_mss                       璇诲彇_mostly                             __tcp_mtu_鍒癬mss(tcp_鍐欏叆_xmit)
int                             sysctl_tcp_probe_threshold                                                           tcp_mtu_probe(tcp_鍐欏叆_xmit)
u32                             sysctl_tcp_probe_interval                                                            tcp_mtu_check_reprobe(tcp_鍐欏叆_xmit)
int                             sysctl_tcp_keepalive_time
int                             sysctl_tcp_keepalive_intvl
u8                              sysctl_tcp_keepalive_probes
u8                              sysctl_tcp_syn_retries
u8                              sysctl_tcp_synack_retries
u8                              sysctl_tcp_syncookies                                                                generated_鍦╛syn
u8                              sysctl_tcp_migrate_req                                                               reuseport
u8                              sysctl_tcp_comp_sack_nr                                                              __tcp_ack_snd_check
int                             sysctl_tcp_reordering                                            璇诲彇_mostly         tcp_鍙痏raise_cwnd/tcp_cong_control
u8                              sysctl_tcp_retries1
u8                              sysctl_tcp_retries2
u8                              sysctl_tcp_orphan_retries
u8                              sysctl_tcp_tw_reuse                                                                  timewait_sock_ops
unsigned_int                    sysctl_tcp_tw_reuse_delay                                                            timewait_sock_ops
int                             sysctl_tcp_fin_瓒呮椂                                                               TCP_鏈€鍚巁ACK/tcp_rcv_鐘舵€乢杩涚▼
unsigned_int                    sysctl_tcp_notsent_lowat                     璇诲彇_mostly                             tcp_notsent_lowat/tcp_娴乢鍐呭瓨_free
u8                              sysctl_tcp_sack                                                                      tcp_syn_閫夐」
u8                              sysctl_tcp_window_scaling                                                            tcp_syn_閫夐」,tcp_parse_閫夐」
u8                              sysctl_tcp_timestamps
u8                              sysctl_tcp_early_retrans                     璇诲彇_mostly                             tcp_schedule_loss_probe(tcp_鍐欏叆_xmit)
u32                             sysctl_tcp_rto_max_ms
u8                              sysctl_tcp_recovery                                                                  tcp_fastretrans_alert
u8                              sysctl_tcp_thin_linear_timeouts                                                      tcp_retrans_timer(鍦╛thin_streams)
u8                              sysctl_tcp_slow_鍚姩_涔嬪悗_idle                                                     unlikely(tcp_cwnd_validate-network-not-starved)
u8                              sysctl_tcp_retrans_collapse
u8                              sysctl_tcp_stdurg                                                                    unlikely(tcp_check_urg)
u8                              sysctl_tcp_rfc1337
u8                              sysctl_tcp_abort_鍦╛overflow
u8                              sysctl_tcp_fack
int                             sysctl_tcp_max_reordering                                                            tcp_check_sack_reordering
int                             sysctl_tcp_adv_win_scale                                                             tcp_鍒濆鍖朹缂撳啿鍖篲space
u8                              sysctl_tcp_dsack                                                                     partial_鏁版嵁鍖卂鎴朹retrans_鍦╛tcp_鏁版嵁_闃熷垪
u8                              sysctl_tcp_app_win                                                                   tcp_win_鏉ヨ嚜_space
u8                              sysctl_tcp_frto                                                                      tcp_enter_loss
u8                              sysctl_tcp_nometrics_save                                                            TCP_鏈€鍚巁ACK/tcp_鏇存柊_metrics
u8                              sysctl_tcp_鏃燺ssthresh_metrics_save                                                  TCP_鏈€鍚巁ACK/tcp_(鏇存柊/鍒濆鍖?_metrics
u8                              sysctl_tcp_moderate_rcvbuf                                       璇诲彇_mostly         tcp_rcvbuf_grow()
u32                             sysctl_tcp_rcvbuf_low_rtt                                        璇诲彇_mostly         tcp_rcvbuf_grow()
u8                              sysctl_tcp_shrink_window                     璇诲彇_mostly         璇诲彇_mostly         __tcp_select_window()
u8                              sysctl_tcp_tso_win_divisor                   璇诲彇_mostly                             tcp_tso_搴斿綋_defer(tcp_鍐欏叆_xmit)
u8                              sysctl_tcp_workaround_signed_windows                                                 tcp_select_window
int                             sysctl_tcp_limit_杈撳嚭_bytes                璇诲彇_mostly                             tcp_small_闃熷垪_check(tcp_鍐欏叆_xmit)
int                             sysctl_tcp_challenge_ack_limit
int                             sysctl_tcp_min_rtt_wlen                      璇诲彇_mostly                             tcp_ack_鏇存柊_rtt
u8                              sysctl_tcp_min_tso_segs                                                              unlikely(icsk_ca_ops-written)
u8                              sysctl_tcp_tso_rtt_log                       璇诲彇_mostly                             tcp_tso_autosize
u8                              sysctl_tcp_autocorking                       璇诲彇_mostly                             tcp_push/tcp_搴斿綋_autocork
u8                              sysctl_tcp_reflect_tos                                                               tcp_v(4/6)_send_synack
int                             sysctl_tcp_invalid_ratelimit
int                             sysctl_tcp_pacing_ss_ratio                                                           榛樿_cong_cont(tcp_鏇存柊_pacing_rate)
int                             sysctl_tcp_pacing_ca_ratio                                                           榛樿_cong_cont(tcp_鏇存柊_pacing_rate)
int                             sysctl_tcp_wmem[^3^]                           璇诲彇_mostly                             tcp_wmem_schedule(sendmsg/sendpage)
int                             sysctl_tcp_rmem[^3^]                                               璇诲彇_mostly         __tcp_grow_window(tx),tcp_rcv_space_adjust(rx)
unsigned_int                    sysctl_tcp_child_ehash_鏉＄洰
unsigned_long                   sysctl_tcp_comp_sack_delay_ns                                                        __tcp_ack_snd_check
unsigned_long                   sysctl_tcp_comp_sack_slack_ns                                                        __tcp_ack_snd_check
int                             sysctl_max_syn_backlog
int                             sysctl_tcp_fastopen
缁撴瀯浣揰tcp_congestion_ops       tcp_congestion_control                                                               鍒濆鍖朹cc
缁撴瀯浣揰tcp_fastopen_涓婁笅鏂?    tcp_fastopen_ctx
unsigned_int                    sysctl_tcp_fastopen_blackhole_瓒呮椂
鍘熷瓙_t                        tfo_active_绂佺敤_times
unsigned_long                   tfo_active_绂佺敤_stamp
u32                             tcp_challenge_timestamp
u32                             tcp_challenge_count
u8                              sysctl_tcp_plb_宸插惎鐢?
u8                              sysctl_tcp_plb_idle_rehash_rounds
u8                              sysctl_tcp_plb_rehash_rounds
u8                              sysctl_tcp_plb_suspend_rto_sec
int                             sysctl_tcp_plb_cong_thresh
int                             sysctl_udp_wmem_min
int                             sysctl_udp_rmem_min
u8                              sysctl_fib_notify_鍦╛鏍囧織_change
u8                              sysctl_udp_l3mdev_accept
u8                              sysctl_igmp_llm_reports
int                             sysctl_igmp_max_memberships
int                             sysctl_igmp_max_msf
int                             sysctl_igmp_qrv
缁撴瀯浣揰ping_group_range         ping_group_range
鍘熷瓙_t                        dev_addr_genid
unsigned_int                    sysctl_udp_child_hash_鏉＄洰
unsigned_long*                  sysctl_鏈湴_reserved_ports
int                             sysctl_ip_prot_sock
缁撴瀯浣揰mr_琛?                mrt
缁撴瀯浣揰鍒楀嚭_head                mr_琛?
缁撴瀯浣揰fib_rules_ops*           mr_rules_ops
u32                             sysctl_fib_multipath_hash_瀛楁
u8                              sysctl_fib_multipath_浣跨敤_neigh
u8                              sysctl_fib_multipath_hash_policy
缁撴瀯浣揰fib_notifier_ops*        notifier_ops
unsigned_int                    fib_seq
缁撴瀯浣揰fib_notifier_ops*        ipmr_notifier_ops
unsigned_int                    ipmr_seq
鍘熷瓙_t                        rt_genid
siphash_key_t                   ip_id_key
=============================== ============================================ =================== =================== =================================================
