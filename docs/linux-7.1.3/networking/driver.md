
## Softnet 椹卞姩闂


## 鎺㈡祴鍑嗗垯


### 鍦板潃鏍￠獙


浣犱负璁惧鑾峰彇鐨勪换浣曠‖浠跺眰鍦板潃閮藉簲缁忚繃鏍￠獙銆備緥濡傦紝瀵逛簬浠ュお缃戯紝鍙娇鐢?linux/etherdevice.h:is_valid_ether_addr() 杩涜鏍￠獙銆?
## 鍏抽棴/鍋滄鍑嗗垯


### 闈欓粯


鍦ㄨ皟鐢?ndo_stop 渚嬬▼涔嬪悗锛岀‖浠朵笉寰楀啀鎺ユ敹鎴栧彂閫佷换浣曟暟鎹€傛墍鏈夊湪閫旂殑鏁版嵁鍖呴兘蹇呴』琚腑姝€傚鏈夊繀瑕侊紝杞鎴栫瓑寰呬换浣曞浣嶅懡浠ゅ畬鎴愩€?
### 鑷姩鍏抽棴


濡傛灉璁惧浠嶅浜?UP 鐘舵€侊紝unregister_netdevice 灏嗚皟鐢?ndo_stop 渚嬬▼銆?
## 鍙戦€佽矾寰勫噯鍒?

### 鎻愬墠鍋滄闃熷垪


ndo_start_xmit 鏂规硶鍦ㄤ换浣曟甯告儏鍐典笅閮戒笉寰楄繑鍥?NETDEV_TX_BUSY銆傞櫎闈炰綘鐨勮澶囨棤娉曟彁鍓嶈幏鐭ュ叾鍙戦€佸姛鑳戒綍鏃朵細鍙樺緱绻佸繖锛屽惁鍒欒繖琚涓轰竴涓弗閲嶉敊璇€?
鐩稿弽锛屽畠蹇呴』姝ｇ‘鍦扮淮鎶ら槦鍒椼€備緥濡傦紝瀵逛簬瀹炵幇浜嗗垎鏁?鑱氶泦锛坰catter-gather锛夌殑椹卞姩鏉ヨ锛岃繖鎰忓懗鐫€锛?

	static u32 drv_tx_avail(struct drv_ring *dr)
	{
		u32 used = READ_ONCE(dr->prod) - READ_ONCE(dr->cons);

		return dr->tx_ring_size - (used & bp->tx_ring_mask);
	}

	static netdev_tx_t drv_hard_start_xmit(struct sk_buff *skb,
					       struct net_device *dev)
	{
		struct drv *dp = netdev_priv(dev);
		struct netdev_queue *txq;
		struct drv_ring *dr;
		int idx;

		idx = skb_get_queue_mapping(skb);
		dr = dp->tx_rings[idx];
		txq = netdev_get_tx_queue(dev, idx);

		//...
		/** This should be a very rare race - log it. **/
		if (drv_tx_avail(dr) <= skb_shinfo(skb)->nr_frags + 1) {
			netif_stop_queue(dev);
			netdev_warn(dev, "Tx Ring full when queue awake!\n");
			return NETDEV_TX_BUSY;
		}

		//... queue packet to card ...

		netdev_tx_sent_queue(txq, skb->len);

		//... update tx producer index using WRITE_ONCE() ...

		if (!netif_txq_maybe_stop(txq, drv_tx_avail(dr),
					  MAX_SKB_FRAGS + 1, 2 * MAX_SKB_FRAGS))
			dr->stats.stopped++;

		//...
		return NETDEV_TX_OK;
	}

鐒跺悗鍦ㄤ綘鐨?TX 鍥炴敹浜嬩欢澶勭悊缁撴潫鏃讹細


	//... update tx consumer index using WRITE_ONCE() ...

	netif_txq_completed_wake(txq, cmpl_pkts, cmpl_bytes,
				 drv_tx_avail(dr), 2 * MAX_SKB_FRAGS);

#### 鏃犻攣闃熷垪鍋滄/鍞ら啋杈呭姪瀹?

   :doc: Lockless queue stopping / waking helpers.

### 鏃犵嫭鍗犳墍鏈夋潈


ndo_start_xmit 鏂规硶涓嶅緱淇敼琚厠闅嗙殑 SKB 鐨勫叡浜儴鍒嗐€?
### 鍙婃椂瀹屾垚


涓嶈蹇樿锛屼竴鏃︿綘鐨?ndo_start_xmit 鏂规硶杩斿洖 NETDEV_TX_OK锛岄噴鏀捐 SKB 灏辨槸浣犵殑椹卞姩鐨勮矗浠伙紝骞朵笖蹇呴』鍦ㄦ湁闄愮殑鏃堕棿鍐呭畬鎴愩€?
渚嬪锛岃繖鎰忓懗鐫€濡傛灉浣犵殑 TX 缂撹В锛坢itigation锛夋柟妗堝湪娌℃湁浠讳綍鏂扮殑 TX 鏁版嵁鍖呭彂閫佹椂锛屼笉鍏佽璁?TX 鏁版嵁鍖呮案杩溾€滄粸鐣欌€濆湪 TX 鐜腑鑰屾湭琚洖鏀躲€傛閿欒鍙兘瀵艰嚧姝ｅ湪绛夊緟鍙戦€佺紦鍐插尯绌洪棿閲婃斁鐨勫鎺ュ瓧鍙戠敓姝婚攣銆?
濡傛灉浣犱粠 ndo_start_xmit 鏂规硶杩斿洖 NETDEV_TX_BUSY锛屼綘涓嶅緱淇濈暀瀵硅 SKB 鐨勪换浣曞紩鐢紝涔熶笉寰楀皾璇曢噴鏀惧畠銆?