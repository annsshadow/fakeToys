
## net_璁惧 缁撴瀯浣撳揩閫熻矾寰勪娇鐢ㄦ媶瑙?

鏈〉浠ョ紦瀛樿锛坈acheline锛変负缁村害锛屾媶瑙?`struct net_device` 鍚勫瓧娈靛湪鍙戦€佷笌鎺ユ敹蹇€熻矾寰勪腑鐨勮闂柟寮忥紝渚涙€ц兘鍒嗘瀽涓庣紦瀛樼儹鐐逛紭鍖栧弬鑰冦€傝〃鏍间粠宸﹀埌鍙充緷娆″垪鍑哄瓧娈电殑绫诲瀷銆佸悕绉帮紝浠ュ強璇ュ瓧娈靛湪鍙戦€侊紙tx锛変笌鎺ユ敹锛坮x锛夊揩閫熻矾寰勪腑琚闂殑浣嶇疆锛堝涓?`read_mostly` 鎴栧叿浣撶殑鍐呮牳鍑芥暟锛夛紝鏈€鍚庝竴鍒椾负绠€瑕佽鏄庛€?


=================================== =========================== =================== =================== ===================================================================================
绫诲瀷                               鍚嶇О                        鍙戦€佸揩閫熻矾寰勮闂?    鎺ユ敹蹇€熻矾寰勮闂?   璇存槑
=================================== =========================== =================== =================== ===================================================================================
unsigned_long:32                    priv_鏍囧織                  璇诲彇_mostly                             __dev_闃熷垪_xmit(tx)
unsigned_long:1                     lltx                        璇诲彇_mostly                             HARD_TX_閿?HARD_TX_TRYLOCK,HARD_TX_瑙ｉ攣(tx)
unsigned long:1                     netmem_tx:1;                璇诲彇_mostly
char                                name[^16^]
缁撴瀯浣?netdev_name_node*            name_node
缁撴瀯浣?dev_ifalias*                 ifalias
unsigned_long                       mem_end
unsigned_long                       mem_鍚姩
unsigned_long                       base_addr
unsigned_long                       鐘舵€?                      璇诲彇_mostly         璇诲彇_mostly         netif_杩愯涓?dev)
缁撴瀯浣?鍒楀嚭_head                    dev_鍒楀嚭
缁撴瀯浣?鍒楀嚭_head                    napi_鍒楀嚭
缁撴瀯浣?鍒楀嚭_head                    unreg_鍒楀嚭
缁撴瀯浣?鍒楀嚭_head                    鍏抽棴_鍒楀嚭
缁撴瀯浣?鍒楀嚭_head                    ptype_鍏ㄩ儴                   璇诲彇_mostly                             dev_nit_active(tx)
缁撴瀯浣?鍒楀嚭_head                    ptype_鐗瑰畾                                  璇诲彇_mostly         deliver_ptype_鍒楀嚭_skb/__netif_receive_skb_鏍稿績(rx)
缁撴瀯浣?                             adj_鍒楀嚭
unsigned_int                        鏍囧織                       璇诲彇_mostly         璇诲彇_mostly         __dev_闃熷垪_xmit,__dev_xmit_skb,ip6_杈撳嚭,__ip6_finish_杈撳嚭(tx);ip6_rcv_鏍稿績(rx)
xdp_鐗规€t                      xdp_鐗规€?
缁撴瀯浣?net_璁惧_ops*              netdev_ops                  璇诲彇_mostly                             netdev_鏍稿績_pick_tx,netdev_鍚姩_xmit(tx)
缁撴瀯浣?xdp_metadata_ops*            xdp_metadata_ops
int                                 ifindex                                         璇诲彇_mostly         ip6_rcv_鏍稿績
unsigned_short                      gflags
unsigned_short                      hard_header_len             璇诲彇_mostly         璇诲彇_mostly         ip6_xmit(tx);gro_鍒楀嚭_prepare(rx)
unsigned_int                        mtu                         璇诲彇_mostly                             ip_finish_杈撳嚭2
unsigned_short                      needed_headroom             璇诲彇_mostly                             LL_RESERVED_SPACE/ip_finish_杈撳嚭2
unsigned_short                      needed_tailroom
netdev_鐗规€t                   鐗规€?                   璇诲彇_mostly         璇诲彇_mostly         HARD_TX_閿?netif_skb_鐗规€?sk_setup_caps(tx);netif_elide_gro(rx)
netdev_鐗规€t                   hw_鐗规€?
netdev_鐗规€t                   wanted_鐗规€?
netdev_鐗规€t                   vlan_鐗规€?
netdev_鐗规€t                   hw_enc_鐗规€?                                                    netif_skb_鐗规€?
netdev_鐗规€t                   mpls_鐗规€?
netdev_鐗规€t                   gso_partial_鐗规€?       璇诲彇_mostly                             gso_鐗规€check
unsigned_int                        min_mtu
unsigned_int                        max_mtu
unsigned_short                      绫诲瀷
unsigned_char                       min_header_len
unsigned_char                       name_assign_绫诲瀷
int                                 group
缁撴瀯浣?net_璁惧_stats             stats
缁撴瀯浣?net_璁惧_鏍稿績_stats*       鏍稿績_stats
鍘熷瓙_t                            carrier_up_count
鍘熷瓙_t                            carrier_down_count
缁撴瀯浣?iw_澶勭悊绋嬪簭_def*              鏃犵嚎_handlers
缁撴瀯浣?ethtool_ops*                 ethtool_ops
缁撴瀯浣?l3mdev_ops*                  l3mdev_ops
缁撴瀯浣?ndisc_ops*                   ndisc_ops
缁撴瀯浣?xfrmdev_ops*                 xfrmdev_ops
缁撴瀯浣?tlsdev_ops*                  tlsdev_ops
缁撴瀯浣?header_ops*                  header_ops                  璇诲彇_mostly                             ip_finish_杈撳嚭2,ip6_finish_杈撳嚭2(tx)
unsigned_char                       operstate
unsigned_char                       link_妯″紡
unsigned_char                       鑻绔彛
unsigned_char                       dma
unsigned_char                       perm_addr[^32^]
unsigned_char                       addr_assign_绫诲瀷
unsigned_char                       addr_len
unsigned_char                       upper_level
unsigned_char                       lower_level
u8                                  threaded                                                            napi_poll(napi_鍚敤,netif_set_threaded)
unsigned_short                      neigh_priv_len
unsigned_short                      padded
unsigned_short                      dev_id
unsigned_short                      dev_绔彛
鑷棆閿乢t                          addr_鍒楀嚭_閿?
int                                 irq
缁撴瀯浣?netdev_hw_addr_鍒楀嚭          uc
缁撴瀯浣?netdev_hw_addr_鍒楀嚭          mc
缁撴瀯浣?netdev_hw_addr_鍒楀嚭          dev_addrs
缁撴瀯浣?kset*                        queues_kset
缁撴瀯浣?鍒楀嚭_head                    unlink_鍒楀嚭
unsigned_int                        promiscuity
unsigned_int                        allmulti
bool                                uc_promisc
unsigned_char                       nested_level
缁撴瀯浣?鍦╛璁惧*                   ip_ptr                      璇诲彇_mostly         璇诲彇_mostly         __鍦╛dev_get
缁撴瀯浣?hlist_head                   fib_nh_head
缁撴瀯浣?inet6_dev*                   ip6_ptr                     璇诲彇_mostly         璇诲彇_mostly         __鍦?_dev_get
缁撴瀯浣?vlan_info*                   vlan_info
缁撴瀯浣?dsa_绔彛*                    dsa_ptr
缁撴瀯浣?tipc_bearer*                 tipc_ptr
void*                               atalk_ptr
void*                               ax25_ptr
缁撴瀯浣?鏃犵嚎_dev*                ieee80211_ptr
缁撴瀯浣?wpan_dev*                    ieee802154_ptr
缁撴瀯浣?mpls_dev*                    mpls_ptr
缁撴瀯浣?mctp_dev*                    mctp_ptr
unsigned_char*                      dev_addr
缁撴瀯浣?netdev_闃熷垪*                _rx                         璇诲彇_mostly                             netdev_get_rx_闃熷垪(rx)
unsigned_int                        num_rx_queues
unsigned_int                        real_num_rx_queues                              璇诲彇_mostly         get_rps_CPU
缁撴瀯浣?bpf_prog*                    xdp_prog                                        璇诲彇_mostly         netif_elide_gro()
unsigned_long                       gro_flush_瓒呮椂                               璇诲彇_mostly         napi_complete_宸插畬鎴?
u32                                 napi_defer_hard_irqs                            璇诲彇_mostly         napi_complete_宸插畬鎴?
unsigned_int                        gro_max_澶у皬                                    璇诲彇_mostly         skb_gro_receive
unsigned_int                        gro_ipv4_max_澶у皬                               璇诲彇_mostly         skb_gro_receive
rx_澶勭悊绋嬪簭_func_t*                  rx_澶勭悊绋嬪簭                  璇诲彇_mostly                             __netif_receive_skb_鏍稿績
void*                               rx_澶勭悊绋嬪簭_鏁版嵁             璇诲彇_mostly
缁撴瀯浣?netdev_闃熷垪*                ingress_闃熷垪               璇诲彇_mostly
缁撴瀯浣?bpf_mprog_鏉＄洰              tcx_ingress                                     璇诲彇_mostly         sch_handle_ingress
缁撴瀯浣?nf_hook_鏉＄洰*             nf_hooks_ingress
unsigned_char                       broadcast[^32^]
缁撴瀯浣?CPU_rmap*                    rx_CPU_rmap
缁撴瀯浣?hlist_node                   绱㈠紩_hlist
缁撴瀯浣?netdev_闃熷垪*                _tx                         璇诲彇_mostly                             netdev_get_tx_闃熷垪(tx)
unsigned_int                        num_tx_queues
unsigned_int                        real_num_tx_queues          璇诲彇_mostly                             skb_tx_hash,netdev_鏍稿績_pick_tx(tx)
unsigned_int                        tx_闃熷垪_len
鑷棆閿乢t                          tx_鍏ㄥ眬_閿?
缁撴瀯浣?xdp_dev_bulk_闃熷垪__percpu*  xdp_bulkq
缁撴瀯浣?xps_dev_maps*                xps_maps[^2^]                 璇诲彇_mostly                             __netif_set_xps_闃熷垪
缁撴瀯浣?bpf_mprog_鏉＄洰              tcx_egress                  璇诲彇_mostly                             sch_handle_egress
缁撴瀯浣?nf_hook_鏉＄洰*             nf_hooks_egress             璇诲彇_mostly
缁撴瀯浣?hlist_head                   qdisc_hash[^16^]
缁撴瀯浣?timer_鍒楀嚭                   watchdog_timer
int                                 watchdog_timeo
u32                                 proto_down_reason
缁撴瀯浣?鍒楀嚭_head                    todo_鍒楀嚭
int__percpu*                        pcpu_refcnt
refcount_t                          dev_refcnt
缁撴瀯浣?ref_tracker_dir              refcnt_tracker
缁撴瀯浣?鍒楀嚭_head                    link_watch_鍒楀嚭
enum:8                              reg_鐘舵€?
bool                                dismantle
bool                                rtnl_link_initilizing
bool                                needs_free_netdev
void*priv_destructor                缁撴瀯浣?net_璁惧
缁撴瀯浣?netpoll_info*                npinfo                                          璇诲彇_mostly         napi_poll/napi_poll_閿?
鍙兘_net_t                      nd_net                                          璇诲彇_mostly         (dev_net)napi_busy_loop,tcp_v(4/6)_rcv,ip(v6)_rcv,ip(6)_杈撳叆,ip(6)_杈撳叆_finish
void*                               ml_priv
enum_netdev_ml_priv_绫诲瀷            ml_priv_绫诲瀷
缁撴瀯浣?pcpu_lstats__percpu*         lstats                      璇诲彇_mostly                             dev_lstats_add()
缁撴瀯浣?pcpu_sw_netstats__percpu*    tstats                      璇诲彇_mostly                             dev_sw_netstats_tx_add()
缁撴瀯浣?pcpu_dstats__percpu*         dstats
缁撴瀯浣?garp_绔彛*                   garp_绔彛
缁撴瀯浣?mrp_绔彛*                    mrp_绔彛
缁撴瀯浣?dm_hw_stat_delta*            dm_绉佹湁
缁撴瀯浣?璁惧                       dev
缁撴瀯浣?attribute_group*             sysfs_groups[^4^]
缁撴瀯浣?attribute_group*             sysfs_rx_闃熷垪_group
缁撴瀯浣?rtnl_link_ops*               rtnl_link_ops
unsigned_int                        gso_max_澶у皬                璇诲彇_mostly                             sk_dst_gso_max_澶у皬
unsigned_int                        tso_max_澶у皬
u16                                 gso_max_segs                璇诲彇_mostly                             gso_max_segs
u16                                 tso_max_segs
unsigned_int                        gso_ipv4_max_澶у皬           璇诲彇_mostly                             sk_dst_gso_max_澶у皬
缁撴瀯浣?dcbnl_rtnl_ops*              dcbnl_ops
s16                                 num_tc                      璇诲彇_mostly                             skb_tx_hash
缁撴瀯浣?netdev_tc_txq                tc_鍒癬txq[^16^]               璇诲彇_mostly                             skb_tx_hash
u8                                  prio_tc_map[^16^]
unsigned_int                        fcoe_ddp_xid
缁撴瀯浣?netprio_map*                 priomap
缁撴瀯浣?phy_璁惧*                  phydev
缁撴瀯浣?sfp_鎬荤嚎*                     sfp_鎬荤嚎
缁撴瀯浣?閿乢绫籣key*              qdisc_tx_busylock
bool                                proto_down
unsigned:1                          wol_宸插惎鐢?
unsigned_long:1                     鍙傝_鍏ㄩ儴_hwtstamp_requests
unsigned_long:1                     change_proto_down
unsigned_long:1                     netns_immutable
unsigned_long:1                     fcoe_mtu
缁撴瀯浣?鍒楀嚭_head                    net_notifier_鍒楀嚭
缁撴瀯浣?macsec_ops*                  macsec_ops
缁撴瀯浣?udp_tunnel_nic_info*         udp_tunnel_nic_info
缁撴瀯浣?udp_tunnel_nic*              udp_tunnel_nic
unsigned_int                        xdp_zc_max_segs
缁撴瀯浣?bpf_xdp_entity               xdp_鐘舵€乕^3^]
u8                                  dev_addr_shadow[^32^]
netdevice_tracker                   linkwatch_dev_tracker
netdevice_tracker                   watchdog_dev_tracker
netdevice_tracker                   dev_registered_tracker
缁撴瀯浣?rtnl_hw_stats64*             offload_xstats_l3
缁撴瀯浣?devlink_绔彛*                devlink_绔彛
缁撴瀯浣?dpll_pin*                    dpll_pin
缁撴瀯浣?hlist_head                   椤礯pools
缁撴瀯浣?dim_irq_moder*               irq_moder
u64                                 max_pacing_offload_horizon
缁撴瀯浣揰napi_閰嶇疆*                 napi_閰嶇疆
unsigned_long                       gro_flush_瓒呮椂
u32                                 napi_defer_hard_irqs
缁撴瀯浣?hlist_head                   neighbours[^2^]
=================================== =========================== =================== =================== ===================================================================================
