
## XFRM 璁惧 - 鍗歌浇 IPsec 璁＄畻


Shannon Nelson <shannon.nelson@oracle.com>
Leon Romanovsky <leonro@nvidia.com>


## 姒傝堪


IPsec 鏄繚闅滅綉缁滄祦閲忓畨鍏ㄧ殑瀹炵敤鐗规€э紝浣嗚绠楁垚鏈緢楂橈細鏍规嵁娴侀噺鍜岄摼璺厤缃殑涓嶅悓锛屼竴鏉?10Gbps 鐨勯摼璺緢瀹规槗闄嶅埌 1Gbps 浠ヤ笅銆傚垢杩愮殑鏄紝鏈?NIC 鎻愪緵鍩轰簬纭欢鐨?IPsec 鍗歌浇锛屽彲浠?澶у箙鎻愰珮鍚炲悙閲忓苟闄嶄綆 CPU 鍒╃敤鐜囥€俋FRM 璁惧鎺ュ彛鍏佽 NIC 椹卞姩鍚戝崗璁爤鎻愪緵瀵圭‖浠跺嵏杞界殑
璁块棶銆?
鐩墠锛屽唴鏍告敮鎸佷袱绉嶇被鍨嬬殑纭欢鍗歌浇锛?
 - IPsec 鍔犲瘑鍗歌浇锛?
   - NIC 鎵ц鍔犲瘑/瑙ｅ瘑
   - 鍐呮牳澶勭悊鍏跺畠涓€鍒?
 - IPsec 鏁版嵁鍖呭嵏杞斤細

   - NIC 鎵ц鍔犲瘑/瑙ｅ瘑
   - NIC 鎵ц灏佽
   - 鍐呮牳鍜?NIC 鐨?SA 鍜岀瓥鐣ヤ繚鎸佸悓姝?   - NIC 澶勭悊 SA 鍜岀瓥鐣ョ姸鎬?   - 鍐呮牳涓庡瘑閽ョ鐞嗗櫒閫氫俊

鐢ㄦ埛绌洪棿瀵瑰嵏杞界殑璁块棶閫氬父閫氳繃璇稿 libreswan 鎴?KAME/raccoon 杩欐牱鐨勭郴缁燂紝浣嗗湪璇曢獙鏃讹紝
iproute2 鐨?'ip xfrm' 鍛戒护闆嗕細寰堟柟渚裤€備竴涓ず渚嬪懡浠ゅ彲鑳界湅璧锋潵鍍?```

  ip x s add proto esp dst 14.0.0.70 src 14.0.0.52 spi 0x07 mode transport \
     reqid 0x07 replay-window 32 \
     aead 'rfc4106(gcm(aes))' 0x44434241343332312423222114131211f4f3f2f1 128 \
     sel src 14.0.0.52/24 dst 14.0.0.70/24 proto tcp \
     offload dev eth4 dir in

```
```

  ip x s add proto esp dst 14.0.0.70 src 14.0.0.52 spi 0x07 mode transport \
     reqid 0x07 replay-window 32 \
     aead 'rfc4106(gcm(aes))' 0x44434241343332312423222114131211f4f3f2f1 128 \
     sel src 14.0.0.52/24 dst 14.0.0.70/24 proto tcp \
     offload packet dev eth4 dir in

  ip x p add src 14.0.0.70 dst 14.0.0.52 offload packet dev eth4 dir in
  tmpl src 14.0.0.70 dst 14.0.0.52 proto esp reqid 10000 mode transport

```
娌￠敊锛岃繖寰堥毦鐪嬶紝浣嗚繖灏辨槸 shell 鑴氭湰鍜?鎴?libreswan 鐨勭敤閫斻€?

## 闇€瑕佸疄鐜扮殑鍥炶皟


```

  /* from include/linux/netdevice.h */
  struct xfrmdev_ops {
        /* Crypto and Packet offload callbacks */
	int	(*xdo_dev_state_add)(struct net_device *dev,
                                     struct xfrm_state *x,
                                     struct netlink_ext_ack *extack);
	void	(*xdo_dev_state_delete)(struct net_device *dev,
                                        struct xfrm_state *x);
	void	(*xdo_dev_state_free)(struct net_device *dev,
                                      struct xfrm_state *x);
	bool	(*xdo_dev_offload_ok) (struct sk_buff *skb,
				       struct xfrm_state *x);
	void    (*xdo_dev_state_advance_esn) (struct xfrm_state *x);
	void    (*xdo_dev_state_update_stats) (struct xfrm_state *x);

        /* Solely packet offload callbacks */
	int	(*xdo_dev_policy_add) (struct xfrm_policy *x, struct netlink_ext_ack *extack);
	void	(*xdo_dev_policy_delete) (struct xfrm_policy *x);
	void	(*xdo_dev_policy_free) (struct xfrm_policy *x);
  };

```
鎻愪緵 ipsec 鍗歌浇鐨?NIC 椹卞姩闇€瑕佸疄鐜颁笌鎵€鏀寔鍗歌浇鐩稿叧鐨勫洖璋冿紝浠ヤ娇璇ュ嵏杞藉缃戠粶鍗忚鏍堢殑
XFRM 瀛愮郴缁熷彲鐢ㄣ€傛澶栵紝鐗规€т綅 NETIF_F_HW_ESP 鍜?NETIF_F_HW_ESP_TX_CSUM 灏嗚〃鏄庡嵏杞?鐨勫彲鐢ㄦ€с€?

## 娴佺▼


鍦ㄦ帰娴嬫椂浠ュ強璋冪敤 register_netdev() 涔嬪墠锛岄┍鍔ㄥ簲褰撹缃湰鍦版暟鎹粨鏋勫拰 XFRM 鍥炶皟锛屽苟
璁剧疆鐗规€т綅銆俋FRM 浠ｇ爜鐨勭洃鍚櫒灏嗗湪 NETDEV_REGISTER 涓婂畬鎴愯缃€?```

		adapter->netdev->xfrmdev_ops = &ixgbe_xfrmdev_ops;
		adapter->netdev->features |= NETIF_F_HW_ESP;
		adapter->netdev->hw_enc_features |= NETIF_F_HW_ESP;

```
褰撲负璇锋眰鈥滃嵏杞解€濈壒鎬х殑鏂?SA 寤虹珛鏃讹紝椹卞姩鐨?xdo_dev_state_add() 灏嗚幏寰楄琚嵏杞界殑鏂?SA 浠ュ強瀹冩槸鐢ㄤ簬 Rx 杩樻槸 Tx 鐨勬寚绀恒€傞┍鍔ㄥ簲褰?
 - 楠岃瘉绠楁硶鏀寔鍗歌浇
 - 瀛樺偍 SA 淇℃伅锛堝瘑閽ャ€乻alt銆佺洰鏍?IP銆佸崗璁瓑锛? - 鍚敤璇?SA 鐨勭‖浠跺嵏杞? - 杩斿洖鐘舵€佸€硷細

		===========   ===================================
		0             success
		-EOPNETSUPP   涓嶆敮鎸佸嵏杞斤紝灏濊瘯 SW IPsec锛?                              涓嶉€傜敤浜庢暟鎹寘鍗歌浇妯″紡
		other         浣胯姹傚け璐?		===========   ===================================

椹卞姩杩樺彲浠ュ湪 SA 涓缃竴涓?offload_handle锛屼竴涓笉閫忔槑鐨?void 鎸囬拡
```

		xs->xso.offload_handle = context;


```
褰撶綉缁滃崗璁爤涓哄凡璁剧疆鍗歌浇鐨?SA 鍑嗗涓€涓?IPsec 鏁版嵁鍖呮椂锛屽畠棣栧厛璋冪敤 xdo_dev_offload_ok()
锛屼紶鍏?skb 鍜岄鏈熺殑鍗歌浇鐘舵€侊紝璇㈤棶椹卞姩鍗歌浇鏄惁鍙敤銆傝繖鍙互妫€鏌ユ暟鎹寘淇℃伅浠ョ‘淇濆嵏杞?琚敮鎸侊紙渚嬪 IPv4 鎴?IPv6銆佹病鏈?IPv4 閫夐」绛夛級锛屽苟杩斿洖 true 鎴?false 浠ヨ〃鏄庡叾鏀寔銆?濡傛灉椹卞姩娌℃湁瀹炵幇姝ゅ洖璋冿紝鍗忚鏍堟彁渚涘悎鐞嗙殑榛樿鍊笺€?
鍔犲瘑鍗歌浇妯″紡锛?褰撳噯澶囧彂閫佹椂锛岄┍鍔ㄩ渶瑕佹鏌?Tx 鏁版嵁鍖呯殑鍗歌浇淇℃伅锛屽寘鎷笉閫忔槑鐨勪笂涓嬫枃锛屽苟璁剧疆鏁版嵁鍖?```

		xs = xfrm_input_state(skb);
		context = xs->xso.offload_handle;
		set up HW for send

```
鍗忚鏍堝凡缁忓湪鏁版嵁鍖呮暟鎹腑鎻掑叆浜嗛€傚綋鐨?IPsec 澶撮儴锛屽嵏杞藉彧闇€瑕佽繘琛屽姞瀵嗗苟淇澶撮儴鍊笺€?

褰撴敹鍒颁竴涓暟鎹寘骞朵笖 HW 鎸囩ず瀹冨嵏杞戒簡瑙ｅ瘑鏃讹紝椹卞姩闇€瑕佸悜鏁版嵁鍖呯殑 skb 娣诲姞涓€涓瑙ｇ爜鍚?SA 鐨勫紩鐢ㄣ€傛鏃舵暟鎹簲褰撳凡琚В瀵嗭紝浣?IPsec 澶撮儴浠嶅湪鏁版嵁鍖呮暟鎹腑锛涘畠浠◢鍚庝細鍦ㄥ崗璁爤
涓婂眰鐨?xfrm_input() 涓绉婚櫎銆?```

		/* get spi, protocol, and destination IP from packet headers */
		xs = find xs from (spi, protocol, dest_IP)
		xfrm_state_hold(xs);

```
```

		sp = secpath_set(skb);
		if (!sp) return;
		sp->xvec[sp->len++] = xs;
		sp->olen++;

```
```

		xo = xfrm_offload(skb);
		xo->flags = CRYPTO_DONE;
		xo->status = crypto_status;

```
4. 鍍忓線甯镐竴鏍峰皢鏁版嵁鍖呬氦缁?napi_gro_receive()銆?
鍦?ESN 妯″紡涓嬶紝浠?xfrm_replay_advance_esn()锛圧X锛夊拰 xfrm_replay_overflow_offload_esn
锛圱X锛夎皟鐢?xdo_dev_state_advance_esn()銆傞┍鍔ㄥ皢妫€鏌ユ暟鎹寘搴忓垪鍙凤紝骞跺湪闇€瑕佹椂鏇存柊 HW ESN
鐘舵€佹満銆?
鏁版嵁鍖呭嵏杞芥ā寮忥細
HW 娣诲姞鍜屽垹闄?XFRM 澶撮儴銆傚洜姝ゅ湪 RX 璺緞涓紝濡傛灉 HW 鎶ュ憡鎴愬姛锛孹FRM 鍗忚鏍堣缁曡繃銆傚湪
TX 璺緞涓紝鏁版嵁鍖呭湪娌℃湁棰濆澶撮儴涓旀湭鍔犲瘑鐨勬儏鍐典笅绂诲紑鍐呮牳锛孒W 璐熻矗鎵ц瀹冦€?
褰?SA 琚敤鎴风Щ闄ゆ椂锛屼細瑕佹眰椹卞姩鐨?xdo_dev_state_delete() 鍜?xdo_dev_policy_delete()
绂佺敤鍗歌浇銆備箣鍚庯紝鍦ㄦ墍鏈夊璇ョ姸鎬佸拰绛栫暐鐨勫紩鐢ㄨ鏁伴兘琚Щ闄ゃ€佸苟涓斾换浣曞墿浣欒祫婧愬彲浠ヤ负鍗歌浇
鐘舵€佹竻鐞嗕箣鍚庯紝xdo_dev_state_free() 鍜?xdo_dev_policy_free() 浠庝竴涓瀮鍦惧洖鏀朵緥绋嬩腑琚?璋冪敤銆傞┍鍔ㄥ浣曚娇鐢ㄨ繖浜涘彇鍐充簬鐗瑰畾鐨勭‖浠堕渶姹傘€?
褰?netdev 琚缃负 DOWN 鏃讹紝XFRM 鍗忚鏍堢殑 netdev 鐩戝惉鍣ㄤ細瀵逛换浣曞墿浣欑殑鍗歌浇鐘舵€佽皟鐢?xdo_dev_state_delete()銆亁do_dev_policy_delete()銆亁do_dev_state_free() 鍜?xdo_dev_policy_free()銆?
鐢变簬 HW 澶勭悊鏁版嵁鍖呯殑缁撴灉锛孹FRM 鏍稿績鏃犳硶璁℃暟纭檺鍒躲€佽蒋闄愬埗銆侶W/椹卞姩璐熻矗鎵ц瀹冿紝骞跺湪
璋冪敤 xdo_dev_state_update_stats() 鏃舵彁渚涘噯纭殑鏁版嵁銆傚鏋滃彂鐢熶簡杩欎簺闄愬埗涔嬩竴锛岄┍鍔ㄩ渶瑕?璋冪敤 xfrm_state_check_expire() 浠ョ‘淇?XFRM 鎵ц閲嶆柊瀵嗛挜搴忓垪銆?