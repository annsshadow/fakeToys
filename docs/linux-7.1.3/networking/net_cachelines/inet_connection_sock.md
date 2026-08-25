
## inet_connection_sock 结构体快速路径使用拆

本文档拆TCP 连接套接字结构体 inet_connection_sock 各字段在发送与接收快速路径的缓存行布局与读写特性，并列出访问这些字段的主要内核函数及其读写标记，供网络协议栈关键数据路径的性能与缓存优化参考


本文档为自动生成的结构化条目索引，条目名称以英文术语保留以便检索


下表列出`inet_connection_sock`结构体中各字段在发送与接收快速路径（fastpath tx/rx
访问中的缓存行布局与读写特性，`说明`列给出会访问该字段的部分内核函数


=================================== ====================== =================== =================== ========================================================================================================================================================
类型 名称 fastpath_tx_access fastpath_rx_access 说明
=================================== ====================== =================== =================== ========================================================================================================================================================
结构inet_sock icsk_inet read_mostly read_mostly tcp_init_buffer_space,tcp_init_transfer,tcp_finish_connect,tcp_connect,tcp_send_rcvq,tcp_send_syn_data
结构request_sock_queue icsk_accept_queue
结构inet_bind_bucket icsk_bind_hash read_mostly tcp_set_state
结构inet_bind2_bucket icsk_bind2_hash read_mostly tcp_set_state,inet_put_port
结构timer_list icsk_delack_timer read_mostly inet_csk_reset_xmit_timer,tcp_connect
结构timer_list icsk_keepalive_timer
u32 icsk_rto read_write tcp_cwnd_validate,tcp_schedule_loss_probe,tcp_connect_init,tcp_connect,tcp_write_xmit,tcp_push_one
u32 icsk_rto_min
u32 icsk_rto_max read_mostly tcp_reset_xmit_timer
u32 icsk_delack_max
u32 icsk_pmtu_cookie read_write tcp_sync_mss,tcp_current_mss,tcp_send_syn_data,tcp_connect_init,tcp_connect
结构tcp_congestion_ops icsk_ca_ops read_write tcp_cwnd_validate,tcp_tso_segs,tcp_ca_dst_init,tcp_connect_init,tcp_connect,tcp_write_xmit
结构inet_connection_sock_af_ops icsk_af_ops read_mostly tcp_finish_connect,tcp_send_syn_data,tcp_mtup_init,tcp_mtu_check_reprobe,tcp_mtu_probe,tcp_connect_init,tcp_connect,__tcp_transmit_skb
结构tcp_ulp_ops* icsk_ulp_ops
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
结构icsk_ack_u8 pending read_write read_write inet_csk_ack_scheduled,__tcp_cleanup_rbuf,tcp_cleanup_rbuf,inet_csk_clear_xmit_timer,tcp_event_ack-sent,inet_csk_reset_xmit_timer
结构icsk_ack_u8 quick read_write write_mostly tcp_dec_quickack_mode,tcp_event_ack_sent,__tcp_transmit_skb,__tcp_select_window,__tcp_cleanup_rbuf
结构icsk_ack_u8 pingpong
结构icsk_ack_u8 retry write_mostly read_write inet_csk_clear_xmit_timer,tcp_rearm_rto,tcp_event_new_data_sent,tcp_write_xmit,__tcp_send_ack,tcp_send_ack,
结构icsk_ack_u8 ato read_mostly write_mostly tcp_dec_quickack_mode,tcp_event_ack_sent,__tcp_transmit_skb,__tcp_send_ack,tcp_send_ack
结构icsk_ack_u32 lrcvtime read_write tcp_finish_connect,tcp_connect,tcp_event_data_sent,__tcp_transmit_skb
结构icsk_ack_u16 rcv_mss write_mostly read_mostly __tcp_select_window,__tcp_cleanup_rbuf,tcp_initialize_rcv_mss,tcp_connect_init
结构icsk_mtup_int search_high read_write tcp_mtup_init,tcp_sync_mss,tcp_connect_init,tcp_mtu_check_reprobe,tcp_write_xmit
结构icsk_mtup_int search_low read_write tcp_mtu_probe,tcp_mtu_check_reprobe,tcp_write_xmit,tcp_sync_mss,tcp_connect_init,tcp_mtup_init
结构icsk_mtup_u32:31 probe_size read_write tcp_mtup_init,tcp_connect_init,__tcp_transmit_skb
结构icsk_mtup_u32:1 启用 read_write tcp_mtup_init,tcp_sync_mss,tcp_connect_init,tcp_mtu_probe,tcp_write_xmit
结构icsk_mtup_u32 probe_timestamp read_write tcp_mtup_init,tcp_connect_init,tcp_mtu_check_reprobe,tcp_mtu_probe
u32 icsk_probes_tstamp
u32 icsk_user_timeout
u64[104/sizeof(u64)] icsk_ca_priv
=================================== ====================== =================== =================== ========================================================================================================================================================
