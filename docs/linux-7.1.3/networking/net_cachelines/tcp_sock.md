
## tcp_sock 缁撴瀯浣撳揩閫熻矾寰勪娇鐢ㄦ儏鍐靛垎瑙?

鏈枃妗ｅ垎瑙?`tcp_sock` 缁撴瀯浣撳悇瀛楁鍦ㄧ綉缁滃崗璁爤蹇€熻矾寰勶紙鍙戦€佷笌鎺ユ敹锛変笂鐨勮闂ā寮忥紝鍒楀嚭瀛楁绫诲瀷銆佸悕绉般€佽鍐欏睘鎬у強鎵€娑夊強鐨?TCP 鏍稿績鍑芥暟锛屼緵鍐呮牳缃戠粶寮€鍙戣€呯悊瑙ｇ紦瀛樿甯冨眬涓庢€ц兘鐑偣銆?


涓嬭〃鎸夊揩閫熻矾寰勮闂淮搴︼紝鍒嗚В `tcp_sock` 缁撴瀯浣撲腑鍚勫瓧娈靛湪鍙戦€侊紙tx锛変笌鎺ユ敹锛坮x锛夎矾寰勪笂鐨勪娇鐢ㄦ儏鍐碉紱瀛楁鍚嶃€佸嚱鏁板悕涓庢爣璇嗙鍧囦繚鎸佸師鏂囦互渚挎绱€?


============================= ======================= =================== =================== ==================================================================================================================================================================================================================
绫诲瀷                          鍚嶇О                    fastpath_tx_access  fastpath_rx_access  娉ㄩ噴
============================= ======================= =================== =================== ==================================================================================================================================================================================================================
缁撴瀯浣?inet_杩炴帴_sock   inet_conn
u16                           tcp_header_len          璇诲彇_mostly         璇诲彇_mostly         tcp_bound_鍒癬half_wnd,tcp_鐢垫祦_mss(tx);tcp_rcv_established(rx)
u16                           gso_segs                璇诲彇_mostly                             tcp_xmit_澶у皬_goal
__涓?2                        pred_鏍囧織              璇诲彇_鍐欏叆          璇诲彇_mostly         tcp_select_window(tx);tcp_rcv_established(rx)
u64                           bytes_received                              璇诲彇_鍐欏叆          tcp_rcv_nxt_鏇存柊(rx)
u32                           segs_鍦?                                    璇诲彇_鍐欏叆          tcp_v6_rcv(rx)
u32                           鏁版嵁_segs_鍦?                               璇诲彇_鍐欏叆          tcp_v6_rcv(rx)
u32                           rcv_nxt                 璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_cleanup_rbuf,tcp_send_ack,tcp_inq_hint,tcp_transmit_skb,tcp_receive_window(tx);tcp_v6_鎵ц_rcv,tcp_rcv_established,tcp_鏁版嵁_闃熷垪,tcp_receive_window,tcp_rcv_nxt_鏇存柊(鍐欏叆)(rx)
u32                           copied_seq                                  璇诲彇_mostly         tcp_cleanup_rbuf,tcp_rcv_space_adjust,tcp_inq_hint
u32                           rcv_wup                                     璇诲彇_鍐欏叆          __tcp_cleanup_rbuf,tcp_receive_window,tcp_receive_established
u32                           snd_nxt                 璇诲彇_鍐欏叆          璇诲彇_mostly         tcp_rate_check_app_limited,__tcp_transmit_skb,tcp_浜嬩欢_鏂癬鏁版嵁_sent(鍐欏叆)(tx);tcp_rcv_established,tcp_ack,tcp_clean_rtx_闃熷垪(rx)
u32                           segs_out                璇诲彇_鍐欏叆                              __tcp_transmit_skb
u32                           鏁版嵁_segs_out           璇诲彇_鍐欏叆                              __tcp_transmit_skb,tcp_鏇存柊_skb_涔嬪悗_send
u64                           bytes_sent              璇诲彇_鍐欏叆                              __tcp_transmit_skb
u64                           bytes_acked                                 璇诲彇_鍐欏叆          tcp_snd_una_鏇存柊/tcp_ack
u32                           dsack_dups
u32                           snd_una                 璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_wnd_end,tcp_urg_妯″紡,tcp_minshall_check,tcp_cwnd_validate(tx);tcp_ack,tcp_鍙痏鏇存柊_window,tcp_clean_rtx_闃熷垪(鍐欏叆),tcp_ack_tstamp(rx)
u32                           snd_sml                 璇诲彇_鍐欏叆                              tcp_minshall_check,tcp_minshall_鏇存柊
u32                           rcv_tstamp              璇诲彇_鍐欏叆          璇诲彇_鍐欏叆          tcp_ack
void *                        tcp_clean_acked         璇诲彇_mostly                             tcp_ack
u32                           lsndtime                璇诲彇_鍐欏叆                              tcp_slow_鍚姩_涔嬪悗_idle_check,tcp_浜嬩欢_鏁版嵁_sent
u32                           鏈€鍚巁oow_ack_time
u32                           compressed_ack_rcv_nxt
u32                           tsoffset                璇诲彇_mostly         璇诲彇_mostly         tcp_established_閫夐」(tx);tcp_fast_parse_閫夐」(rx)
缁撴瀯浣?鍒楀嚭_head              tsq_node
缁撴瀯浣?鍒楀嚭_head              tsorted_sent_闃熷垪      璇诲彇_鍐欏叆                              tcp_鏇存柊_skb_涔嬪悗_send
u32                           snd_wl1                                     璇诲彇_mostly         tcp_鍙痏鏇存柊_window
u32                           snd_wnd                 璇诲彇_mostly         璇诲彇_mostly         tcp_wnd_end,tcp_tso_搴斿綋_defer(tx);tcp_fast_path_鍦?rx)
u32                           max_window              璇诲彇_mostly                             tcp_bound_鍒癬half_wnd,forced_push
u32                           mss_缂撳瓨               璇诲彇_mostly         璇诲彇_mostly         tcp_rate_check_app_limited,tcp_鐢垫祦_mss,tcp_sync_mss,tcp_sndbuf_expand,tcp_tso_搴斿綋_defer(tx);tcp_鏇存柊_pacing_rate,tcp_clean_rtx_闃熷垪(rx)
u32                           window_clamp            璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_rcv_space_adjust,__tcp_select_window
u32                           rcv_ssthresh            璇诲彇_mostly                             __tcp_select_window
u8                            scaling_ratio           璇诲彇_mostly         璇诲彇_mostly         tcp_win_鏉ヨ嚜_space
缁撴瀯浣?                       tcp_rack
u16                           advmss                                      璇诲彇_mostly         tcp_rcv_space_adjust
u8                            compressed_ack
u8:2                          dup_ack_counter
u8:1                          tlp_retrans
u8:1                          tcp_usec_ts             璇诲彇_mostly         璇诲彇_mostly
u32                           chrono_鍚姩            璇诲彇_鍐欏叆                              tcp_chrono_鍚姩/鍋滄(tcp_鍐欏叆_xmit,tcp_cwnd_validate,tcp_send_syn_鏁版嵁)
u32[^3^]                        chrono_stat             璇诲彇_鍐欏叆                              tcp_chrono_鍚姩/鍋滄(tcp_鍐欏叆_xmit,tcp_cwnd_validate,tcp_send_syn_鏁版嵁)
u8:2                          chrono_绫诲瀷             璇诲彇_鍐欏叆                              tcp_chrono_鍚姩/鍋滄(tcp_鍐欏叆_xmit,tcp_cwnd_validate,tcp_send_syn_鏁版嵁)
u8:1                          rate_app_limited                            璇诲彇_鍐欏叆          tcp_rate_gen
u8:1                          fastopen_connect
u8:1                          fastopen_鏃燺cookie
u8:1                          鏄痏sack_reneg                               璇诲彇_mostly         tcp_skb_entail,tcp_ack
u8:2                          fastopen_client_fail
u8:4                          nonagle                 璇诲彇_鍐欏叆                              tcp_skb_entail,tcp_push_pending_frames
u8:1                          thin_lto
u8:1                          recvmsg_inq                                 璇诲彇_mostly         tcp_recvmsg
u8:1                          repair                  璇诲彇_mostly                             tcp_鍐欏叆_xmit
u8:1                          frto
u8                            repair_闃熷垪
u8:2                          save_syn
u8:1                          syn_鏁版嵁
u8:1                          syn_fastopen
u8:1                          syn_fastopen_exp
u8:1                          syn_fastopen_ch
u8:1                          syn_鏁版嵁_acked
u8:1                          鏄痏cwnd_limited         璇诲彇_mostly                             tcp_cwnd_validate,tcp_鏄痏cwnd_limited
u32                           tlp_high_seq                                璇诲彇_mostly         tcp_ack
u32                           tcp_tx_delay
u64                           tcp_wstamp_ns           璇诲彇_鍐欏叆                              tcp_pacing_check,tcp_tso_搴斿綋_defer,tcp_鏇存柊_skb_涔嬪悗_send
u64                           tcp_clock_缂撳瓨         璇诲彇_鍐欏叆          璇诲彇_鍐欏叆          tcp_mstamp_refresh(tcp_鍐欏叆_xmit/tcp_rcv_space_adjust),__tcp_transmit_skb,tcp_tso_搴斿綋_defer;timer
u64                           tcp_mstamp              璇诲彇_鍐欏叆          璇诲彇_鍐欏叆          tcp_mstamp_refresh(tcp_鍐欏叆_xmit/tcp_rcv_space_adjust)(tx);tcp_rcv_space_adjust,tcp_rate_gen,tcp_clean_rtx_闃熷垪,tcp_ack_鏇存柊_rtt/tcp_time_stamp(rx);timer
u32                           srtt_us                 璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_tso_搴斿綋_defer(tx);tcp_鏇存柊_pacing_rate,__tcp_set_rto,tcp_rtt_estimator(rx)
u32                           mdev_us                 璇诲彇_鍐欏叆                              tcp_rtt_estimator
u32                           mdev_max_us
u32                           rttvar_us                                   璇诲彇_mostly         __tcp_set_rto
u32                           rtt_seq                 璇诲彇_鍐欏叆                              tcp_rtt_estimator
缁撴瀯浣?minmax                 rtt_min                                     璇诲彇_mostly         tcp_min_rtt/tcp_rate_gen,tcp_min_rtttcp_鏇存柊_rtt_min
u32                           packets_out             璇诲彇_鍐欏叆          璇诲彇_鍐欏叆          tcp_packets_鍦╛flight(tx/rx);tcp_slow_鍚姩_涔嬪悗_idle_check,tcp_nagle_check,tcp_rate_skb_sent,tcp_浜嬩欢_鏂癬鏁版嵁_sent,tcp_cwnd_validate,tcp_鍐欏叆_xmit(tx);tcp_ack,tcp_clean_rtx_闃熷垪,tcp_鏇存柊_pacing_rate(rx)
u32                           retrans_out                                 璇诲彇_mostly         tcp_packets_鍦╛flight,tcp_rate_check_app_limited
u32                           max_packets_out                             璇诲彇_鍐欏叆          tcp_cwnd_validate
u32                           cwnd_usage_seq                              璇诲彇_鍐欏叆          tcp_cwnd_validate
u16                           urg_鏁版嵁                                    璇诲彇_mostly         tcp_fast_path_check
u8                            ecn_鏍囧織               璇诲彇_鍐欏叆                              tcp_ecn_send
u8                            keepalive_probes
u32                           reordering              璇诲彇_mostly                             tcp_sndbuf_expand
u32                           reord_seen
u32                           snd_up                  璇诲彇_鍐欏叆          璇诲彇_mostly         tcp_mark_urg,tcp_urg_妯″紡,__tcp_transmit_skb(tx);tcp_clean_rtx_闃熷垪(rx)
缁撴瀯浣?tcp_閫夐」_received   rx_opt                  璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_established_閫夐」(tx);tcp_fast_path_鍦?tcp_ack_鏇存柊_window,tcp_鏄痏sack,tcp_鏁版嵁_闃熷垪,tcp_rcv_established,tcp_ack_鏇存柊_rtt(rx)
u32                           snd_ssthresh                                璇诲彇_mostly         tcp_鏇存柊_pacing_rate
u32                           snd_cwnd                璇诲彇_mostly         璇诲彇_mostly         tcp_snd_cwnd,tcp_rate_check_app_limited,tcp_tso_搴斿綋_defer(tx);tcp_鏇存柊_pacing_rate
u32                           snd_cwnd_cnt
u32                           snd_cwnd_clamp
u32                           snd_cwnd_浣跨敤
u32                           snd_cwnd_stamp
u32                           prior_cwnd
u32                           prr_delivered
u32                           prr_out                 璇诲彇_mostly         璇诲彇_mostly         tcp_rate_skb_sent,tcp_newly_delivered(tx);tcp_ack,tcp_rate_gen,tcp_clean_rtx_闃熷垪(rx)
u32                           delivered               璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_rate_skb_sent, tcp_newly_delivered(tx);tcp_ack, tcp_rate_gen, tcp_clean_rtx_闃熷垪 (rx)
u32                           delivered_ce            璇诲彇_mostly         璇诲彇_鍐欏叆          tcp_rate_skb_sent(tx);tcp_rate_gen(rx)
u32                           received_ce             璇诲彇_mostly         璇诲彇_鍐欏叆
u32[^3^]                        received_ecn_bytes      璇诲彇_mostly         璇诲彇_鍐欏叆
u8:4                          received_ce_pending     璇诲彇_mostly         璇诲彇_鍐欏叆
u32[^3^]                        delivered_ecn_bytes                         璇诲彇_鍐欏叆
u16                           pkts_acked_ewma                             璇诲彇_鍐欏叆
u8:2                          syn_ect_snt             鍐欏叆_mostly        璇诲彇_鍐欏叆
u8:2                          syn_ect_rcv             璇诲彇_mostly         璇诲彇_鍐欏叆
u8:2                          accecn_minlen           鍐欏叆_mostly        璇诲彇_鍐欏叆
u8:2                          est_ecnfield                                璇诲彇_鍐欏叆
u8:2                          accecn_opt_demand       璇诲彇_mostly         璇诲彇_鍐欏叆
u8:2                          prev_ecnfield                               璇诲彇_鍐欏叆
u64                           accecn_opt_tstamp       璇诲彇_鍐欏叆
u8:4                          accecn_fail_妯″紡
u32                           lost                                        璇诲彇_mostly         tcp_ack
u32                           app_limited             璇诲彇_鍐欏叆          璇诲彇_mostly         tcp_rate_check_app_limited,tcp_rate_skb_sent(tx);tcp_rate_gen(rx)
u64                           绗竴_tx_mstamp         璇诲彇_鍐欏叆                              tcp_rate_skb_sent
u64                           delivered_mstamp        璇诲彇_鍐欏叆                              tcp_rate_skb_sent
u32                           rate_delivered                              璇诲彇_mostly         tcp_rate_gen
u32                           rate_interval_us                            璇诲彇_mostly         rate_delivered,rate_app_limited
u32                           rcv_wnd                 璇诲彇_鍐欏叆          璇诲彇_mostly         tcp_select_window,tcp_receive_window,tcp_fast_path_check
u32                           rcv_mwnd_seq            璇诲彇_鍐欏叆                              tcp_select_window
u32                           鍐欏叆_seq               璇诲彇_鍐欏叆                              tcp_rate_check_app_limited,tcp_鍐欏叆_闃熷垪_empty,tcp_skb_entail,forced_push,tcp_mark_push
u32                           notsent_lowat           璇诲彇_mostly                             tcp_娴乢鍐呭瓨_free
u32                           pushed_seq              璇诲彇_鍐欏叆                              tcp_mark_push,forced_push
u32                           lost_out                璇诲彇_mostly         璇诲彇_mostly         tcp_left_out(tx);tcp_packets_鍦╛flight(tx/rx);tcp_rate_check_app_limited(rx)
u32                           sacked_out              璇诲彇_mostly         璇诲彇_mostly         tcp_left_out(tx);tcp_packets_鍦╛flight(tx/rx);tcp_clean_rtx_闃熷垪(rx)
缁撴瀯浣?hrtimer                pacing_timer
缁撴瀯浣?hrtimer                compressed_ack_timer
缁撴瀯浣?sk_buff*               retransmit_skb_hint     璇诲彇_mostly                             tcp_clean_rtx_闃熷垪
缁撴瀯浣?rb_root                out_鐨刜order_闃熷垪                          璇诲彇_mostly         tcp_鏁版嵁_闃熷垪,tcp_fast_path_check
缁撴瀯浣?sk_buff*               ooo_鏈€鍚巁skb
缁撴瀯浣?tcp_sack_鍧梉^1^]      duplicate_sack
缁撴瀯浣?tcp_sack_鍧梉^4^]      selective_acks
缁撴瀯浣?tcp_sack_鍧梉^4^]      recv_sack_缂撳瓨
缁撴瀯浣?sk_buff*               highest_sack            璇诲彇_鍐欏叆                              tcp_浜嬩欢_鏂癬鏁版嵁_sent
u32                           prior_ssthresh
u32                           high_seq
u32                           retrans_stamp
u32                           undo_marker
int                           undo_retrans
u64                           bytes_retrans
u32                           鎬昏_retrans
u32                           rto_stamp
u16                           鎬昏_rto
u16                           鎬昏_rto_recoveries
u32                           鎬昏_rto_time
u32                           urg_seq
unsigned_int                  keepalive_time
unsigned_int                  keepalive_intvl
int                           linger2
u8                            bpf_sock_ops_cb_鏍囧織
u8:1                          bpf_chg_cc_inprogress
u16                           瓒呮椂_rehash
u32                           rcv_ooopack
u32                           rcv_rtt_鏈€鍚巁tsecr
缁撴瀯浣?                       rcv_rtt_est                                 璇诲彇_鍐欏叆          tcp_rcv_space_adjust,tcp_rcv_established
缁撴瀯浣?                       rcvq_space                                  璇诲彇_鍐欏叆          tcp_rcv_space_adjust
缁撴瀯浣?                       mtu_probe
u32                           plb_rehash
u32                           mtu_info
bool                          鏄痏mptcp
bool                          smc_hs_congested
bool                          syn_smc
缁撴瀯浣?tcp_sock_af_ops*       af_鐗瑰畾
缁撴瀯浣?tcp_md5sig_info*       md5sig_info
缁撴瀯浣?tcp_fastopen_璇锋眰*  fastopen_req
缁撴瀯浣?璇锋眰_sock*          fastopen_rsk
缁撴瀯浣?saved_syn*             saved_syn
============================= ======================= =================== =================== ==================================================================================================================================================================================================================
