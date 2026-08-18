
## inet_connection_sock 缁撴瀯浣撳揩閫熻矾寰勪娇鐢ㄦ媶瑙?

鏈枃妗ｆ媶瑙?TCP 杩炴帴濂楁帴瀛楃粨鏋勪綋 inet_connection_sock 鍚勫瓧娈靛湪鍙戦€佷笌鎺ユ敹蹇€熻矾寰勭殑缂撳瓨琛屽竷灞€涓庤鍐欑壒鎬э紝骞跺垪鍑鸿闂繖浜涘瓧娈电殑涓昏鍐呮牳鍑芥暟鍙婂叾璇诲啓鏍囪锛屼緵缃戠粶鍗忚鏍堝叧閿暟鎹矾寰勭殑鎬ц兘涓庣紦瀛樹紭鍖栧弬鑰冦€?


鏈枃妗ｄ负鑷姩鐢熸垚鐨勭粨鏋勫寲鏉＄洰绱㈠紩锛屾潯鐩悕绉颁互鑻辨枃鏈淇濈暀浠ヤ究妫€绱€?


涓嬭〃鍒楀嚭`inet_connection_sock`缁撴瀯浣撲腑鍚勫瓧娈靛湪鍙戦€佷笌鎺ユ敹蹇€熻矾寰勶紙fastpath tx/rx锛?
璁块棶涓殑缂撳瓨琛屽竷灞€涓庤鍐欑壒鎬э紝`璇存槑`鍒楃粰鍑轰細璁块棶璇ュ瓧娈电殑閮ㄥ垎鍐呮牳鍑芥暟銆?


=================================== ====================== =================== =================== ========================================================================================================================================================
绫诲瀷 鍚嶇О fastpath_tx_access fastpath_rx_access 璇存槑
=================================== ====================== =================== =================== ========================================================================================================================================================
缁撴瀯浣?inet_sock icsk_inet read_mostly read_mostly tcp_init_buffer_space,tcp_init_transfer,tcp_finish_connect,tcp_connect,tcp_send_rcvq,tcp_send_syn_data
缁撴瀯浣?request_sock_queue icsk_accept_queue
缁撴瀯浣?inet_bind_bucket icsk_bind_hash read_mostly tcp_set_state
缁撴瀯浣?inet_bind2_bucket icsk_bind2_hash read_mostly tcp_set_state,inet_put_port
缁撴瀯浣?timer_list icsk_delack_timer read_mostly inet_csk_reset_xmit_timer,tcp_connect
缁撴瀯浣?timer_list icsk_keepalive_timer
u32 icsk_rto read_write tcp_cwnd_validate,tcp_schedule_loss_probe,tcp_connect_init,tcp_connect,tcp_write_xmit,tcp_push_one
u32 icsk_rto_min
u32 icsk_rto_max read_mostly tcp_reset_xmit_timer
u32 icsk_delack_max
u32 icsk_pmtu_cookie read_write tcp_sync_mss,tcp_current_mss,tcp_send_syn_data,tcp_connect_init,tcp_connect
缁撴瀯浣?tcp_congestion_ops icsk_ca_ops read_write tcp_cwnd_validate,tcp_tso_segs,tcp_ca_dst_init,tcp_connect_init,tcp_connect,tcp_write_xmit
缁撴瀯浣?inet_connection_sock_af_ops icsk_af_ops read_mostly tcp_finish_connect,tcp_send_syn_data,tcp_mtup_init,tcp_mtu_check_reprobe,tcp_mtu_probe,tcp_connect_init,tcp_connect,__tcp_transmit_skb
缁撴瀯浣?tcp_ulp_ops* icsk_ulp_ops
void* icsk_ulp_data
u8:5 icsk_ca_state read_write tcp_cwnd_application_limited,tcp_set_ca_state,tcp_enter_cwr,tcp_tso_should_defer,tcp_mtu_probe,tcp_schedule_loss_probe,tcp_write_xmit,__tcp_transmit_skb
u8:1 icsk_ca_initialized read_write tcp_init_transfer,tcp_init_congestion_control,tcp_init_transfer,tcp_finish_connect,tcp_connect
u8:1 icsk_ca_setsockopt
u8:1 icsk_ca_dst_locked write_mostly tcp_ca_dst_init,tcp_connect_init,tcp_connect
u8 icsk_retransmits write_mostly tcp_connect_init,tcp_connect
u8 icsk_pending read_write inet_csk_reset_xmit_timer,tcp_connect,tcp_check_probe_timer,__tcp_push_pending_frames,tcp_rearm_rto,tcp_event_new_data_sent,tcp_event_new_data_sent
u8 icsk_backoff write_mostly tcp_write_queue_purge,tcp_connect_init
u8 icsk_syn_retries
u8 icsk_probes_out
u16 icsk_ext_hdr_len read_mostly __tcp_mtu_to_mss,tcp_mtu_to_rss,tcp_mtu_probe,tcp_write_xmit,tcp_mtu_to_mss,
缁撴瀯浣?icsk_ack_u8 pending read_write read_write inet_csk_ack_scheduled,__tcp_cleanup_rbuf,tcp_cleanup_rbuf,inet_csk_clear_xmit_timer,tcp_event_ack-sent,inet_csk_reset_xmit_timer
缁撴瀯浣?icsk_ack_u8 quick read_write write_mostly tcp_dec_quickack_mode,tcp_event_ack_sent,__tcp_transmit_skb,__tcp_select_window,__tcp_cleanup_rbuf
缁撴瀯浣?icsk_ack_u8 pingpong
缁撴瀯浣?icsk_ack_u8 retry write_mostly read_write inet_csk_clear_xmit_timer,tcp_rearm_rto,tcp_event_new_data_sent,tcp_write_xmit,__tcp_send_ack,tcp_send_ack,
缁撴瀯浣?icsk_ack_u8 ato read_mostly write_mostly tcp_dec_quickack_mode,tcp_event_ack_sent,__tcp_transmit_skb,__tcp_send_ack,tcp_send_ack
缁撴瀯浣?icsk_ack_u32 lrcvtime read_write tcp_finish_connect,tcp_connect,tcp_event_data_sent,__tcp_transmit_skb
缁撴瀯浣?icsk_ack_u16 rcv_mss write_mostly read_mostly __tcp_select_window,__tcp_cleanup_rbuf,tcp_initialize_rcv_mss,tcp_connect_init
缁撴瀯浣?icsk_mtup_int search_high read_write tcp_mtup_init,tcp_sync_mss,tcp_connect_init,tcp_mtu_check_reprobe,tcp_write_xmit
缁撴瀯浣?icsk_mtup_int search_low read_write tcp_mtu_probe,tcp_mtu_check_reprobe,tcp_write_xmit,tcp_sync_mss,tcp_connect_init,tcp_mtup_init
缁撴瀯浣?icsk_mtup_u32:31 probe_size read_write tcp_mtup_init,tcp_connect_init,__tcp_transmit_skb
缁撴瀯浣?icsk_mtup_u32:1 鍚敤 read_write tcp_mtup_init,tcp_sync_mss,tcp_connect_init,tcp_mtu_probe,tcp_write_xmit
缁撴瀯浣?icsk_mtup_u32 probe_timestamp read_write tcp_mtup_init,tcp_connect_init,tcp_mtu_check_reprobe,tcp_mtu_probe
u32 icsk_probes_tstamp
u32 icsk_user_timeout
u64[104/sizeof(u64)] icsk_ca_priv
=================================== ====================== =================== =================== ========================================================================================================================================================
