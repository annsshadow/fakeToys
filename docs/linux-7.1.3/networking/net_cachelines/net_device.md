
## net_设备 结构体快速路径使用拆

本页以缓存行（cacheline）为维度，拆`struct net_device` 各字段在发送与接收快速路径中的访问方式，供性能分析与缓存热点优化参考。表格从左到右依次列出字段的类型、名称，以及该字段在发送（tx）与接收（rx）快速路径中被访问的位置（多`read_mostly` 或具体的内核函数），最后一列为简要说明


=================================== =========================== =================== =================== ===================================================================================
类型                               名称                        发送快速路径访    接收快速路径访   说明
=================================== =========================== =================== =================== ===================================================================================
unsigned_long:32                    priv_标志                  读取_mostly                             __dev_队列_xmit(tx)
unsigned_long:1                     lltx                        读取_mostly                             HARD_TX_HARD_TX_TRYLOCK,HARD_TX_解锁(tx)
unsigned long:1                     netmem_tx:1;                读取_mostly
char                                name[^16^]
结构netdev_name_node*            name_node
结构dev_ifalias*                 ifalias
unsigned_long                       mem_end
unsigned_long                       mem_启动
unsigned_long                       base_addr
unsigned_long                       状                      读取_mostly         读取_mostly         netif_运行dev)
结构列出_head                    dev_列出
结构列出_head                    napi_列出
结构列出_head                    unreg_列出
结构列出_head                    关闭_列出
结构列出_head                    ptype_全部                   读取_mostly                             dev_nit_active(tx)
结构列出_head                    ptype_特定                                  读取_mostly         deliver_ptype_列出_skb/__netif_receive_skb_核心(rx)
结构                             adj_列出
unsigned_int                        标志                       读取_mostly         读取_mostly         __dev_队列_xmit,__dev_xmit_skb,ip6_输出,__ip6_finish_输出(tx);ip6_rcv_核心(rx)
xdp_特性_t                      xdp_特
结构net_设备_ops*              netdev_ops                  读取_mostly                             netdev_核心_pick_tx,netdev_启动_xmit(tx)
结构xdp_metadata_ops*            xdp_metadata_ops
int                                 ifindex                                         读取_mostly         ip6_rcv_核心
unsigned_short                      gflags
unsigned_short                      hard_header_len             读取_mostly         读取_mostly         ip6_xmit(tx);gro_列出_prepare(rx)
unsigned_int                        mtu                         读取_mostly                             ip_finish_输出2
unsigned_short                      needed_headroom             读取_mostly                             LL_RESERVED_SPACE/ip_finish_输出2
unsigned_short                      needed_tailroom
netdev_特性_t                   特                   读取_mostly         读取_mostly         HARD_TX_netif_skb_特sk_setup_caps(tx);netif_elide_gro(rx)
netdev_特性_t                   hw_特
netdev_特性_t                   wanted_特
netdev_特性_t                   vlan_特
netdev_特性_t                   hw_enc_特                                                    netif_skb_特
netdev_特性_t                   mpls_特
netdev_特性_t                   gso_partial_特       读取_mostly                             gso_特性_check
unsigned_int                        min_mtu
unsigned_int                        max_mtu
unsigned_short                      类型
unsigned_char                       min_header_len
unsigned_char                       name_assign_类型
int                                 group
结构net_设备_stats             stats
结构net_设备_核心_stats*       核心_stats
原子_t                            carrier_up_count
原子_t                            carrier_down_count
结构iw_处理程序_def*              无线_handlers
结构ethtool_ops*                 ethtool_ops
结构l3mdev_ops*                  l3mdev_ops
结构ndisc_ops*                   ndisc_ops
结构xfrmdev_ops*                 xfrmdev_ops
结构tlsdev_ops*                  tlsdev_ops
结构header_ops*                  header_ops                  读取_mostly                             ip_finish_输出2,ip6_finish_输出2(tx)
unsigned_char                       operstate
unsigned_char                       link_模式
unsigned_char                       若_端口
unsigned_char                       dma
unsigned_char                       perm_addr[^32^]
unsigned_char                       addr_assign_类型
unsigned_char                       addr_len
unsigned_char                       upper_level
unsigned_char                       lower_level
u8                                  threaded                                                            napi_poll(napi_启用,netif_set_threaded)
unsigned_short                      neigh_priv_len
unsigned_short                      padded
unsigned_short                      dev_id
unsigned_short                      dev_端口
自旋锁_t                          addr_列出_
int                                 irq
结构netdev_hw_addr_列出          uc
结构netdev_hw_addr_列出          mc
结构netdev_hw_addr_列出          dev_addrs
结构kset*                        queues_kset
结构列出_head                    unlink_列出
unsigned_int                        promiscuity
unsigned_int                        allmulti
bool                                uc_promisc
unsigned_char                       nested_level
结构在_设备*                   ip_ptr                      读取_mostly         读取_mostly         __在_dev_get
结构hlist_head                   fib_nh_head
结构inet6_dev*                   ip6_ptr                     读取_mostly         读取_mostly         ___dev_get
结构vlan_info*                   vlan_info
结构dsa_端口*                    dsa_ptr
结构tipc_bearer*                 tipc_ptr
void*                               atalk_ptr
void*                               ax25_ptr
结构无线_dev*                ieee80211_ptr
结构wpan_dev*                    ieee802154_ptr
结构mpls_dev*                    mpls_ptr
结构mctp_dev*                    mctp_ptr
unsigned_char*                      dev_addr
结构netdev_队列*                _rx                         读取_mostly                             netdev_get_rx_队列(rx)
unsigned_int                        num_rx_queues
unsigned_int                        real_num_rx_queues                              读取_mostly         get_rps_CPU
结构bpf_prog*                    xdp_prog                                        读取_mostly         netif_elide_gro()
unsigned_long                       gro_flush_超时                               读取_mostly         napi_complete_已完
u32                                 napi_defer_hard_irqs                            读取_mostly         napi_complete_已完
unsigned_int                        gro_max_大小                                    读取_mostly         skb_gro_receive
unsigned_int                        gro_ipv4_max_大小                               读取_mostly         skb_gro_receive
rx_处理程序_func_t*                  rx_处理程序                  读取_mostly                             __netif_receive_skb_核心
void*                               rx_处理程序_数据             读取_mostly
结构netdev_队列*                ingress_队列               读取_mostly
结构bpf_mprog_条目              tcx_ingress                                     读取_mostly         sch_handle_ingress
结构nf_hook_条目*             nf_hooks_ingress
unsigned_char                       broadcast[^32^]
结构CPU_rmap*                    rx_CPU_rmap
结构hlist_node                   索引_hlist
结构netdev_队列*                _tx                         读取_mostly                             netdev_get_tx_队列(tx)
unsigned_int                        num_tx_queues
unsigned_int                        real_num_tx_queues          读取_mostly                             skb_tx_hash,netdev_核心_pick_tx(tx)
unsigned_int                        tx_队列_len
自旋锁_t                          tx_全局_
结构xdp_dev_bulk_队列__percpu*  xdp_bulkq
结构xps_dev_maps*                xps_maps[^2^]                 读取_mostly                             __netif_set_xps_队列
结构bpf_mprog_条目              tcx_egress                  读取_mostly                             sch_handle_egress
结构nf_hook_条目*             nf_hooks_egress             读取_mostly
结构hlist_head                   qdisc_hash[^16^]
结构timer_列出                   watchdog_timer
int                                 watchdog_timeo
u32                                 proto_down_reason
结构列出_head                    todo_列出
int__percpu*                        pcpu_refcnt
refcount_t                          dev_refcnt
结构ref_tracker_dir              refcnt_tracker
结构列出_head                    link_watch_列出
enum:8                              reg_状
bool                                dismantle
bool                                rtnl_link_initilizing
bool                                needs_free_netdev
void*priv_destructor                结构net_设备
结构netpoll_info*                npinfo                                          读取_mostly         napi_poll/napi_poll_
可能_net_t                      nd_net                                          读取_mostly         (dev_net)napi_busy_loop,tcp_v(4/6)_rcv,ip(v6)_rcv,ip(6)_输入,ip(6)_输入_finish
void*                               ml_priv
enum_netdev_ml_priv_类型            ml_priv_类型
结构pcpu_lstats__percpu*         lstats                      读取_mostly                             dev_lstats_add()
结构pcpu_sw_netstats__percpu*    tstats                      读取_mostly                             dev_sw_netstats_tx_add()
结构pcpu_dstats__percpu*         dstats
结构garp_端口*                   garp_端口
结构mrp_端口*                    mrp_端口
结构dm_hw_stat_delta*            dm_私有
结构设备                       dev
结构attribute_group*             sysfs_groups[^4^]
结构attribute_group*             sysfs_rx_队列_group
结构rtnl_link_ops*               rtnl_link_ops
unsigned_int                        gso_max_大小                读取_mostly                             sk_dst_gso_max_大小
unsigned_int                        tso_max_大小
u16                                 gso_max_segs                读取_mostly                             gso_max_segs
u16                                 tso_max_segs
unsigned_int                        gso_ipv4_max_大小           读取_mostly                             sk_dst_gso_max_大小
结构dcbnl_rtnl_ops*              dcbnl_ops
s16                                 num_tc                      读取_mostly                             skb_tx_hash
结构netdev_tc_txq                tc_到_txq[^16^]               读取_mostly                             skb_tx_hash
u8                                  prio_tc_map[^16^]
unsigned_int                        fcoe_ddp_xid
结构netprio_map*                 priomap
结构phy_设备*                  phydev
结构sfp_总线*                     sfp_总线
结构锁_类_key*              qdisc_tx_busylock
bool                                proto_down
unsigned:1                          wol_已启
unsigned_long:1                     参见_全部_hwtstamp_requests
unsigned_long:1                     change_proto_down
unsigned_long:1                     netns_immutable
unsigned_long:1                     fcoe_mtu
结构列出_head                    net_notifier_列出
结构macsec_ops*                  macsec_ops
结构udp_tunnel_nic_info*         udp_tunnel_nic_info
结构udp_tunnel_nic*              udp_tunnel_nic
unsigned_int                        xdp_zc_max_segs
结构bpf_xdp_entity               xdp_状态[^3^]
u8                                  dev_addr_shadow[^32^]
netdevice_tracker                   linkwatch_dev_tracker
netdevice_tracker                   watchdog_dev_tracker
netdevice_tracker                   dev_registered_tracker
结构rtnl_hw_stats64*             offload_xstats_l3
结构devlink_端口*                devlink_端口
结构dpll_pin*                    dpll_pin
结构hlist_head                   页_pools
结构dim_irq_moder*               irq_moder
u64                                 max_pacing_offload_horizon
结构体_napi_配置*                 napi_配置
unsigned_long                       gro_flush_超时
u32                                 napi_defer_hard_irqs
结构hlist_head                   neighbours[^2^]
=================================== =========================== =================== =================== ===================================================================================
