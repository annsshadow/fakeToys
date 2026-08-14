
## tcp_sock 结构体快速路径使用情况分解

本文档分解 `tcp_sock` 结构体各字段在网络协议栈快速路径（发送与接收）上的访问模式，列出字段类型、名称、读写属性及所涉及的 TCP 核心函数，供内核网络开发者理解缓存行布局与性能热点。


下表按快速路径访问维度，分解 `tcp_sock` 结构体中各字段在发送（tx）与接收（rx）路径上的使用情况；字段名、函数名与标识符均保持原文以便检索。


============================= ======================= =================== =================== ==================================================================================================================================================================================================================
类型                          名称                    fastpath_tx_access  fastpath_rx_access  注释
============================= ======================= =================== =================== ==================================================================================================================================================================================================================
结构体 inet_连接_sock   inet_conn
u16                           tcp_header_len          读取_mostly         读取_mostly         tcp_bound_到_half_wnd,tcp_电流_mss(tx);tcp_rcv_established(rx)
u16                           gso_segs                读取_mostly                             tcp_xmit_大小_goal
__为32                        pred_标志              读取_写入          读取_mostly         tcp_select_window(tx);tcp_rcv_established(rx)
u64                           bytes_received                              读取_写入          tcp_rcv_nxt_更新(rx)
u32                           segs_在                                     读取_写入          tcp_v6_rcv(rx)
u32                           数据_segs_在                                读取_写入          tcp_v6_rcv(rx)
u32                           rcv_nxt                 读取_mostly         读取_写入          tcp_cleanup_rbuf,tcp_send_ack,tcp_inq_hint,tcp_transmit_skb,tcp_receive_window(tx);tcp_v6_执行_rcv,tcp_rcv_established,tcp_数据_队列,tcp_receive_window,tcp_rcv_nxt_更新(写入)(rx)
u32                           copied_seq                                  读取_mostly         tcp_cleanup_rbuf,tcp_rcv_space_adjust,tcp_inq_hint
u32                           rcv_wup                                     读取_写入          __tcp_cleanup_rbuf,tcp_receive_window,tcp_receive_established
u32                           snd_nxt                 读取_写入          读取_mostly         tcp_rate_check_app_limited,__tcp_transmit_skb,tcp_事件_新_数据_sent(写入)(tx);tcp_rcv_established,tcp_ack,tcp_clean_rtx_队列(rx)
u32                           segs_out                读取_写入                              __tcp_transmit_skb
u32                           数据_segs_out           读取_写入                              __tcp_transmit_skb,tcp_更新_skb_之后_send
u64                           bytes_sent              读取_写入                              __tcp_transmit_skb
u64                           bytes_acked                                 读取_写入          tcp_snd_una_更新/tcp_ack
u32                           dsack_dups
u32                           snd_una                 读取_mostly         读取_写入          tcp_wnd_end,tcp_urg_模式,tcp_minshall_check,tcp_cwnd_validate(tx);tcp_ack,tcp_可_更新_window,tcp_clean_rtx_队列(写入),tcp_ack_tstamp(rx)
u32                           snd_sml                 读取_写入                              tcp_minshall_check,tcp_minshall_更新
u32                           rcv_tstamp              读取_写入          读取_写入          tcp_ack
void *                        tcp_clean_acked         读取_mostly                             tcp_ack
u32                           lsndtime                读取_写入                              tcp_slow_启动_之后_idle_check,tcp_事件_数据_sent
u32                           最后_oow_ack_time
u32                           compressed_ack_rcv_nxt
u32                           tsoffset                读取_mostly         读取_mostly         tcp_established_选项(tx);tcp_fast_parse_选项(rx)
结构体 列出_head              tsq_node
结构体 列出_head              tsorted_sent_队列      读取_写入                              tcp_更新_skb_之后_send
u32                           snd_wl1                                     读取_mostly         tcp_可_更新_window
u32                           snd_wnd                 读取_mostly         读取_mostly         tcp_wnd_end,tcp_tso_应当_defer(tx);tcp_fast_path_在(rx)
u32                           max_window              读取_mostly                             tcp_bound_到_half_wnd,forced_push
u32                           mss_缓存               读取_mostly         读取_mostly         tcp_rate_check_app_limited,tcp_电流_mss,tcp_sync_mss,tcp_sndbuf_expand,tcp_tso_应当_defer(tx);tcp_更新_pacing_rate,tcp_clean_rtx_队列(rx)
u32                           window_clamp            读取_mostly         读取_写入          tcp_rcv_space_adjust,__tcp_select_window
u32                           rcv_ssthresh            读取_mostly                             __tcp_select_window
u8                            scaling_ratio           读取_mostly         读取_mostly         tcp_win_来自_space
结构体                        tcp_rack
u16                           advmss                                      读取_mostly         tcp_rcv_space_adjust
u8                            compressed_ack
u8:2                          dup_ack_counter
u8:1                          tlp_retrans
u8:1                          tcp_usec_ts             读取_mostly         读取_mostly
u32                           chrono_启动            读取_写入                              tcp_chrono_启动/停止(tcp_写入_xmit,tcp_cwnd_validate,tcp_send_syn_数据)
u32[^3^]                        chrono_stat             读取_写入                              tcp_chrono_启动/停止(tcp_写入_xmit,tcp_cwnd_validate,tcp_send_syn_数据)
u8:2                          chrono_类型             读取_写入                              tcp_chrono_启动/停止(tcp_写入_xmit,tcp_cwnd_validate,tcp_send_syn_数据)
u8:1                          rate_app_limited                            读取_写入          tcp_rate_gen
u8:1                          fastopen_connect
u8:1                          fastopen_无_cookie
u8:1                          是_sack_reneg                               读取_mostly         tcp_skb_entail,tcp_ack
u8:2                          fastopen_client_fail
u8:4                          nonagle                 读取_写入                              tcp_skb_entail,tcp_push_pending_frames
u8:1                          thin_lto
u8:1                          recvmsg_inq                                 读取_mostly         tcp_recvmsg
u8:1                          repair                  读取_mostly                             tcp_写入_xmit
u8:1                          frto
u8                            repair_队列
u8:2                          save_syn
u8:1                          syn_数据
u8:1                          syn_fastopen
u8:1                          syn_fastopen_exp
u8:1                          syn_fastopen_ch
u8:1                          syn_数据_acked
u8:1                          是_cwnd_limited         读取_mostly                             tcp_cwnd_validate,tcp_是_cwnd_limited
u32                           tlp_high_seq                                读取_mostly         tcp_ack
u32                           tcp_tx_delay
u64                           tcp_wstamp_ns           读取_写入                              tcp_pacing_check,tcp_tso_应当_defer,tcp_更新_skb_之后_send
u64                           tcp_clock_缓存         读取_写入          读取_写入          tcp_mstamp_refresh(tcp_写入_xmit/tcp_rcv_space_adjust),__tcp_transmit_skb,tcp_tso_应当_defer;timer
u64                           tcp_mstamp              读取_写入          读取_写入          tcp_mstamp_refresh(tcp_写入_xmit/tcp_rcv_space_adjust)(tx);tcp_rcv_space_adjust,tcp_rate_gen,tcp_clean_rtx_队列,tcp_ack_更新_rtt/tcp_time_stamp(rx);timer
u32                           srtt_us                 读取_mostly         读取_写入          tcp_tso_应当_defer(tx);tcp_更新_pacing_rate,__tcp_set_rto,tcp_rtt_estimator(rx)
u32                           mdev_us                 读取_写入                              tcp_rtt_estimator
u32                           mdev_max_us
u32                           rttvar_us                                   读取_mostly         __tcp_set_rto
u32                           rtt_seq                 读取_写入                              tcp_rtt_estimator
结构体 minmax                 rtt_min                                     读取_mostly         tcp_min_rtt/tcp_rate_gen,tcp_min_rtttcp_更新_rtt_min
u32                           packets_out             读取_写入          读取_写入          tcp_packets_在_flight(tx/rx);tcp_slow_启动_之后_idle_check,tcp_nagle_check,tcp_rate_skb_sent,tcp_事件_新_数据_sent,tcp_cwnd_validate,tcp_写入_xmit(tx);tcp_ack,tcp_clean_rtx_队列,tcp_更新_pacing_rate(rx)
u32                           retrans_out                                 读取_mostly         tcp_packets_在_flight,tcp_rate_check_app_limited
u32                           max_packets_out                             读取_写入          tcp_cwnd_validate
u32                           cwnd_usage_seq                              读取_写入          tcp_cwnd_validate
u16                           urg_数据                                    读取_mostly         tcp_fast_path_check
u8                            ecn_标志               读取_写入                              tcp_ecn_send
u8                            keepalive_probes
u32                           reordering              读取_mostly                             tcp_sndbuf_expand
u32                           reord_seen
u32                           snd_up                  读取_写入          读取_mostly         tcp_mark_urg,tcp_urg_模式,__tcp_transmit_skb(tx);tcp_clean_rtx_队列(rx)
结构体 tcp_选项_received   rx_opt                  读取_mostly         读取_写入          tcp_established_选项(tx);tcp_fast_path_在,tcp_ack_更新_window,tcp_是_sack,tcp_数据_队列,tcp_rcv_established,tcp_ack_更新_rtt(rx)
u32                           snd_ssthresh                                读取_mostly         tcp_更新_pacing_rate
u32                           snd_cwnd                读取_mostly         读取_mostly         tcp_snd_cwnd,tcp_rate_check_app_limited,tcp_tso_应当_defer(tx);tcp_更新_pacing_rate
u32                           snd_cwnd_cnt
u32                           snd_cwnd_clamp
u32                           snd_cwnd_使用
u32                           snd_cwnd_stamp
u32                           prior_cwnd
u32                           prr_delivered
u32                           prr_out                 读取_mostly         读取_mostly         tcp_rate_skb_sent,tcp_newly_delivered(tx);tcp_ack,tcp_rate_gen,tcp_clean_rtx_队列(rx)
u32                           delivered               读取_mostly         读取_写入          tcp_rate_skb_sent, tcp_newly_delivered(tx);tcp_ack, tcp_rate_gen, tcp_clean_rtx_队列 (rx)
u32                           delivered_ce            读取_mostly         读取_写入          tcp_rate_skb_sent(tx);tcp_rate_gen(rx)
u32                           received_ce             读取_mostly         读取_写入
u32[^3^]                        received_ecn_bytes      读取_mostly         读取_写入
u8:4                          received_ce_pending     读取_mostly         读取_写入
u32[^3^]                        delivered_ecn_bytes                         读取_写入
u16                           pkts_acked_ewma                             读取_写入
u8:2                          syn_ect_snt             写入_mostly        读取_写入
u8:2                          syn_ect_rcv             读取_mostly         读取_写入
u8:2                          accecn_minlen           写入_mostly        读取_写入
u8:2                          est_ecnfield                                读取_写入
u8:2                          accecn_opt_demand       读取_mostly         读取_写入
u8:2                          prev_ecnfield                               读取_写入
u64                           accecn_opt_tstamp       读取_写入
u8:4                          accecn_fail_模式
u32                           lost                                        读取_mostly         tcp_ack
u32                           app_limited             读取_写入          读取_mostly         tcp_rate_check_app_limited,tcp_rate_skb_sent(tx);tcp_rate_gen(rx)
u64                           第一_tx_mstamp         读取_写入                              tcp_rate_skb_sent
u64                           delivered_mstamp        读取_写入                              tcp_rate_skb_sent
u32                           rate_delivered                              读取_mostly         tcp_rate_gen
u32                           rate_interval_us                            读取_mostly         rate_delivered,rate_app_limited
u32                           rcv_wnd                 读取_写入          读取_mostly         tcp_select_window,tcp_receive_window,tcp_fast_path_check
u32                           rcv_mwnd_seq            读取_写入                              tcp_select_window
u32                           写入_seq               读取_写入                              tcp_rate_check_app_limited,tcp_写入_队列_empty,tcp_skb_entail,forced_push,tcp_mark_push
u32                           notsent_lowat           读取_mostly                             tcp_流_内存_free
u32                           pushed_seq              读取_写入                              tcp_mark_push,forced_push
u32                           lost_out                读取_mostly         读取_mostly         tcp_left_out(tx);tcp_packets_在_flight(tx/rx);tcp_rate_check_app_limited(rx)
u32                           sacked_out              读取_mostly         读取_mostly         tcp_left_out(tx);tcp_packets_在_flight(tx/rx);tcp_clean_rtx_队列(rx)
结构体 hrtimer                pacing_timer
结构体 hrtimer                compressed_ack_timer
结构体 sk_buff*               retransmit_skb_hint     读取_mostly                             tcp_clean_rtx_队列
结构体 rb_root                out_的_order_队列                          读取_mostly         tcp_数据_队列,tcp_fast_path_check
结构体 sk_buff*               ooo_最后_skb
结构体 tcp_sack_块[^1^]      duplicate_sack
结构体 tcp_sack_块[^4^]      selective_acks
结构体 tcp_sack_块[^4^]      recv_sack_缓存
结构体 sk_buff*               highest_sack            读取_写入                              tcp_事件_新_数据_sent
u32                           prior_ssthresh
u32                           high_seq
u32                           retrans_stamp
u32                           undo_marker
int                           undo_retrans
u64                           bytes_retrans
u32                           总计_retrans
u32                           rto_stamp
u16                           总计_rto
u16                           总计_rto_recoveries
u32                           总计_rto_time
u32                           urg_seq
unsigned_int                  keepalive_time
unsigned_int                  keepalive_intvl
int                           linger2
u8                            bpf_sock_ops_cb_标志
u8:1                          bpf_chg_cc_inprogress
u16                           超时_rehash
u32                           rcv_ooopack
u32                           rcv_rtt_最后_tsecr
结构体                        rcv_rtt_est                                 读取_写入          tcp_rcv_space_adjust,tcp_rcv_established
结构体                        rcvq_space                                  读取_写入          tcp_rcv_space_adjust
结构体                        mtu_probe
u32                           plb_rehash
u32                           mtu_info
bool                          是_mptcp
bool                          smc_hs_congested
bool                          syn_smc
结构体 tcp_sock_af_ops*       af_特定
结构体 tcp_md5sig_info*       md5sig_info
结构体 tcp_fastopen_请求*  fastopen_req
结构体 请求_sock*          fastopen_rsk
结构体 saved_syn*             saved_syn
============================= ======================= =================== =================== ==================================================================================================================================================================================================================
