## 缃戠粶璁惧鍜屽唴鏍革紝浠ュ強浣狅紒


## 绠€浠?
浠ヤ笅鏄叧浜庣綉缁滆澶囩殑涓€浜涢浂鏁ｆ枃妗ｉ泦鍚堛€傚畠闈㈠悜椹卞姩寮€鍙戣€呫€?
## struct net_device 鐨勭敓鍛藉懆鏈熻鍒?
缃戠粶璁惧缁撴瀯浣撳嵆浣垮湪妯″潡琚嵏杞藉悗涔熷繀椤绘寔缁瓨鍦紝骞朵笖蹇呴』浣跨敤 alloc_netdev_mqs() 鍙婂叾鐩稿叧鍑芥暟杩涜鍒嗛厤銆傚鏋滆澶囧凡鎴愬姛娉ㄥ唽锛屽畠灏嗗湪鏈€鍚庝竴娆′娇鐢ㄦ椂鐢?free_netdev() 閲婃斁銆傝繖鏄负浜嗚閭ｄ釜鏋佺鎯呭喌锛坧athological case锛夎兘澶熻骞插噣鍦板鐞嗭紙渚嬪锛歚rmmod mydriver </sys/class/net/myeth/mtu`锛夈€?
alloc_netdev_mqs() / alloc_netdev() 浼氫负椹卞姩绉佹湁鏁版嵁棰勭暀棰濆绌洪棿锛岃绌洪棿鍦ㄧ綉缁滆澶囪閲婃斁鏃朵竴鍚岄噴鏀俱€傚鏋滃垎閰嶇殑鐙珛鏁版嵁琚檮鍔犲埌缃戠粶璁惧锛坣etdev_priv()锛変笂锛屽垯鐢辨ā鍧楅€€鍑哄鐞嗗嚱鏁拌礋璐ｉ噴鏀惧畠銆?
娉ㄥ唽 struct net_device 鏈変袱缁?API銆傜涓€缁勫彲鐢ㄤ簬 `rtnl_lock` 灏氭湭鎸佹湁鐨勬櫘閫氫笂涓嬫枃锛歳egister_netdev()銆乽nregister_netdev()銆傜浜岀粍鍙敤浜?`rtnl_lock` 宸茬粡鎸佹湁鐨勬儏褰細register_netdevice()銆乽nregister_netdevice()銆乫ree_netdevice()銆?
### 绠€鍗曢┍鍔?

澶у鏁伴┍鍔紙灏ゅ叾鏄澶囬┍鍔級鍦?`rtnl_lock` 鏈鎸佹湁锛堜緥濡傞┍鍔ㄧ殑 probe 鍜?remove 璺緞锛夌殑涓婁笅鏂囦腑澶勭悊 struct net_device 鐨勭敓鍛藉懆鏈熴€?
鍦ㄨ繖绉嶆儏鍐典笅锛宻truct net_device 鐨勬敞鍐屼娇鐢?register_netdev() 鍜?unregister_netdev() 鍑芥暟瀹屾垚锛?

  int probe()
  {
    struct my_device_priv *priv;
    int err;

    dev = alloc_netdev_mqs(...);
    if (!dev)
      return -ENOMEM;
    priv = netdev_priv(dev);

    /* ... 鍦ㄨ皟鐢?register_netdev() 涔嬪墠瀹屾垚鎵€鏈夎澶囪缃?...
     */

    err = register_netdev(dev);
    if (err)
      goto err_undo;

    /** net_device 瀵圭敤鎴峰彲瑙侊紒 **/

  err_undo:
    /** ... 鎾ら攢璁惧璁剧疆 ... **/
    free_netdev(dev);
    return err;
  }

  void remove()
  {
    unregister_netdev(dev);
    free_netdev(dev);
  }

娉ㄦ剰锛岃皟鐢?register_netdev() 涔嬪悗锛岃澶囦究鍦ㄧ郴缁熶腑鍙銆傜敤鎴峰彲浠ョ珛鍗虫墦寮€瀹冨苟寮€濮嬪彂閫?鎺ユ敹娴侀噺锛屾垨杩愯浠讳綍鍏朵粬鍥炶皟锛屽洜姝ゆ墍鏈夊垵濮嬪寲閮藉繀椤诲湪娉ㄥ唽涔嬪墠瀹屾垚銆?
unregister_netdev() 浼氬叧闂澶囧苟绛夊緟鎵€鏈夌敤鎴蜂娇鐢ㄥ畬姣曘€俿truct net_device 鑷韩鐨勫唴瀛樺彲鑳戒粛琚?sysfs 寮曠敤锛屼絾瀵硅璁惧鐨勬墍鏈夋搷浣滈兘浼氬け璐ャ€?
free_netdev() 鍙互鍦?unregister_netdev() 杩斿洖涔嬪悗锛屾垨鑰?register_netdev() 澶辫触鏃惰皟鐢ㄣ€?
### 鍦?RTNL 涓嬬殑璁惧绠＄悊


鍦ㄥ凡缁忔寔鏈?`rtnl_lock` 鐨勪笂涓嬫枃涓敞鍐?struct net_device 闇€瑕佹牸澶栧皬蹇冦€傚湪杩欎簺鍦烘櫙涓紝澶у鏁伴┍鍔ㄤ細甯屾湜鍒╃敤 struct net_device 鐨?`needs_free_netdev` 鍜?`priv_destructor` 鎴愬憳鏉ラ噴鏀剧姸鎬併€?
鍦?`rtnl_lock` 涓嬪鐞?netdev 鐨勭ず渚嬫祦绋嬶細


  static void my_setup(struct net_device *dev)
  {
    dev->needs_free_netdev = true;
  }

  static void my_destructor(struct net_device *dev)
  {
    some_obj_destroy(priv->obj);
    some_uninit(priv);
  }

  int create_link()
  {
    struct my_device_priv *priv;
    int err;

    ASSERT_RTNL();

    dev = alloc_netdev(sizeof(*priv), "net%d", NET_NAME_UNKNOWN, my_setup);
    if (!dev)
      return -ENOMEM;
    priv = netdev_priv(dev);

    /** 闅愬紡鏋勯€犲嚱鏁?**/
    err = some_init(priv);
    if (err)
      goto err_free_dev;

    priv->obj = some_obj_create();
    if (!priv->obj) {
      err = -ENOMEM;
      goto err_some_uninit;
    }
    /** 鏋勯€犲嚱鏁扮粨鏉燂紝璁剧疆鏋愭瀯鍑芥暟锛?**/
    dev->priv_destructor = my_destructor;

    err = register_netdevice(dev);
    if (err)
      /** register_netdevice() 浼氬湪澶辫触鏃惰皟鐢ㄦ瀽鏋勫嚱鏁?**/
      goto err_free_dev;

    /* 濡傛灉姝ゅ悗鏈変换浣曞け璐ワ紝unregister_netdevice()锛堟垨 unregister_netdev()锛?     - 浼氳礋璐ｈ皟鐢?my_destructor 鍜?free_netdev()銆?     */

    return 0;

  err_some_uninit:
    some_uninit(priv);
  err_free_dev:
    free_netdev(dev);
    return err;
  }

濡傛灉璁剧疆浜?struct net_device.priv_destructor锛屾牳蹇冧唬鐮佷細鍦?unregister_netdevice() 涔嬪悗鐨勬煇涓椂鍒昏皟鐢ㄥ畠锛屽鏋?register_netdevice() 澶辫触瀹冧篃浼氳璋冪敤銆傝鍥炶皟鍙兘鍦ㄦ寔鏈夋垨鏈寔鏈?`rtnl_lock` 鐨勬儏鍐典笅琚皟鐢ㄣ€?
娌℃湁鏄惧紡鐨勬瀯閫犲嚱鏁板洖璋冿紝椹卞姩鍦ㄥ垎閰嶇鏈?netdev 鐘舵€佷箣鍚庛€佹敞鍐屼箣鍓?鏋勯€?瀹冦€?
璁剧疆 struct net_device.needs_free_netdev 浼氫娇鏍稿績浠ｇ爜鍦?unregister_netdevice() 涔嬪悗銆佸綋瀵硅澶囩殑鎵€鏈夊紩鐢ㄩ兘娑堝け鏃讹紝鑷姩璋冪敤 free_netdevice()銆傚畠浠呭湪鎴愬姛璋冪敤 register_netdevice() 涔嬪悗鎵嶇敓鏁堬紝鍥犳濡傛灉 register_netdevice() 澶辫触锛岄┍鍔ㄨ礋璐ｈ皟鐢?free_netdev()銆?
free_netdev() 鍦ㄥ嚭閿欒矾寰勪笂銆佺揣鎺?unregister_netdevice() 涔嬪悗锛屾垨 register_netdevice() 澶辫触鏃讹紝閮芥槸瀹夊叏鍙皟鐢ㄧ殑銆俷etdev 鐨勶紙娉ㄩ攢锛夋敞鍐岃繃绋嬬殑鏌愪簺閮ㄥ垎鍙戠敓鍦?`rtnl_lock` 閲婃斁涔嬪悗锛屽洜姝ゅ湪杩欎簺鎯呭喌涓?free_netdev() 浼氬皢鍏堕儴鍒嗗鐞嗘帹杩熷埌 `rtnl_lock` 閲婃斁涔嬪悗杩涜銆?
浠?struct rtnl_link_ops 娲剧敓鍑虹殑璁惧缁濅笉搴旂洿鎺ラ噴鏀?struct net_device銆?
#### .ndo_init 鍜?.ndo_uninit


`.ndo_init` 鍜?`.ndo_uninit` 鍥炶皟鍦?net_device 娉ㄥ唽鍜屾敞閿€鏈熼棿銆佸湪 `rtnl_lock` 涓嬭璋冪敤銆傞┍鍔ㄥ彲浠ュ湪瀹冧滑鍒濆鍖栬繃绋嬬殑鏌愪簺閮ㄥ垎闇€瑕佸湪 `rtnl_lock` 涓嬭繍琛屾椂浣跨敤杩欎簺鍥炶皟銆?
`.ndo_init` 鍦ㄨ澶囦簬绯荤粺涓彲瑙佷箣鍓嶈繍琛岋紝`.ndo_uninit` 鍦ㄨ澶囧叧闂悗鐨勬敞閿€杩囩▼涓繍琛岋紝浣嗗叾浠栧瓙绯荤粺鍙兘浠嶇劧鎸佹湁瀵?netdev 鐨勬湭鍐冲紩鐢ㄣ€?
## MTU

姣忎釜缃戠粶璁惧閮芥湁涓€涓渶澶т紶杈撳崟鍏冿紙Maximum Transfer Unit锛孧TU锛夈€侻TU 涓嶅寘鍚换浣曢摼璺眰鍗忚寮€閿€銆備笂灞傚崗璁笉寰楀悜璁惧浼犲叆涓€涓暟鎹噺瓒呰繃 mtu 鐨勫鎺ュ瓧缂撳啿鍖猴紙skb锛夋潵浼犺緭銆侻TU 涓嶅寘鍚摼璺眰澶撮儴寮€閿€锛屼緥濡傛爣鍑?MTU 涓?1500 瀛楄妭鐨勪互澶綉锛岀敱浜庝互澶綉澶撮儴鐨勫瓨鍦紝瀹為檯 skb 鏈€澶氫細鍖呭惈 1514 瀛楄妭銆傝澶囪繕搴斿綋涓?4 瀛楄妭鐨?VLAN 澶撮儴鐣欏嚭绌洪棿銆?
鍒嗙墖鍗歌浇锛圫egmentation Offload锛孏SO銆乀SO锛夋槸姝よ鍒欑殑涓€涓緥澶栥€備笂灞傚崗璁彲浠ュ悜璁惧鐨勫彂閫佷緥绋嬩紶鍏ヤ竴涓ぇ鐨勫鎺ュ瓧缂撳啿鍖猴紝璁惧浼氭牴鎹綋鍓?MTU 灏嗗叾鎷嗗垎鎴愮嫭绔嬬殑鏁版嵁鍖呫€?
MTU 鏄绉扮殑锛屽悓鏃堕€傜敤浜庢帴鏀跺拰鍙戦€併€傝澶囧繀椤昏兘澶熸帴鏀惰嚦灏?MTU 鎵€鍏佽鐨勬渶澶у昂瀵哥殑鏁版嵁鍖呫€傜綉缁滆澶囧彲浠ュ皢 MTU 鐢ㄤ綔璋冩暣鎺ユ敹缂撳啿鍖哄ぇ灏忕殑鏈哄埗锛屼絾璁惧搴斿綋鍏佽甯︽湁 VLAN 澶撮儴鐨勬暟鎹寘銆傛爣鍑嗕互澶綉 mtu 涓?1500 瀛楄妭鏃讹紝璁惧搴斿綋鍏佽鏈€澶?1518 瀛楄妭鐨勬暟鎹寘锛?500 + 14 澶撮儴 + 4 鏍囩锛夈€傝澶囧彲浠ワ細涓㈠純銆佹埅鏂紝鎴栧悜涓婁紶閫掕秴澶э紙oversize锛夋暟鎹寘锛屼絾涓㈠純瓒呭ぇ鏁版嵁鍖呮槸棣栭€夈€?

## struct net_device 鍚屾瑙勫垯

ndo_open:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰锛坰haper锛堿PI锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼

ndo_stop:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼
	娉ㄦ剰锛歯etif_running() 淇濊瘉涓?false

ndo_do_ioctl:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€?
	杩欎粎鐢辩綉缁滃瓙绯荤粺鍦ㄥ唴閮ㄨ皟鐢紝鑰屼笉鏄儚 linux-5.14 涔嬪墠閭ｆ牱鐢辩敤鎴风┖闂磋皟鐢?ioctl 瑙﹀彂銆?
ndo_siocbond:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?        涓婁笅鏂囷細杩涚▼

	鐢?bonding 椹卞姩鐢ㄤ簬 SIOCBOND 绯诲垪鐨?ioctl 鍛戒护銆?
ndo_siocwandev:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼

	鐢?drivers/net/wan 妗嗘灦鐢ㄤ簬閰嶅悎 if_settings 缁撴瀯浣撳鐞?SIOCWANDEV ioctl銆?
ndo_siocdevprivate:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼

	杩欑敤浜庡疄鐜?SIOCDEVPRIVATE ioctl 杈呭姪鍑芥暟銆備笉搴斿皢鍏舵坊鍔犲埌鏂伴┍鍔ㄤ腑锛屾墍浠ヤ笉瑕佷娇鐢ㄣ€?
ndo_eth_ioctl:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼

ndo_get_stats:
	鍚屾锛歊CU锛堝彲浠ヤ笌缁熻淇℃伅鏇存柊璺緞骞跺彂璋冪敤锛夈€?	涓婁笅鏂囷細鍘熷瓙锛坅tomic锛屼笉鑳藉湪 RCU 涓嬬潯鐪狅級

ndo_start_xmit:
	鍚屾锛歘_netif_tx_lock 鑷棆閿併€?
	褰撻┍鍔ㄨ缃?dev->lltx 鏃讹紝杩欏皢鍦ㄤ笉鎸佹湁 netif_tx_lock 鐨勬儏鍐典笅琚皟鐢ㄣ€傝繖绉嶆儏鍐典笅椹卞姩闇€瑕佸湪闇€瑕佹椂鑷鍔犻攣銆?	閭ｉ噷鐨勫姞閿佽繕搴斿綋姝ｇ‘闃叉涓?set_rx_mode 涔嬮棿鐨勭珵浜夈€傝鍛婏細浣跨敤 dev->lltx 宸茶寮冪敤銆備笉瑕佸湪鏂伴┍鍔ㄤ腑浣跨敤瀹冦€?
	涓婁笅鏂囷細BH 琚鐢ㄦ椂鐨勮繘绋嬫垨 BH锛堝畾鏃跺櫒锛夛紝netconsole 浼氬湪涓柇琚鐢ㄧ殑鎯呭喌涓嬭皟鐢ㄥ畠銆?
	杩斿洖鐮侊細

 - NETDEV_TX_OK 涓€鍒囨甯搞€? - NETDEV_TX_BUSY 鏃犳硶鍙戦€佹暟鎹寘锛岀◢鍚庨噸璇?	  閫氬父鏄竴涓?bug锛屾剰鍛崇潃椹卞姩涓殑闃熷垪鍚姩/鍋滄娴佹帶琚牬鍧忋€?	  娉ㄦ剰锛氶┍鍔ㄤ笉寰楀皢 skb 鏀惧叆鍏?DMA 鐜腑銆?
ndo_tx_timeout:
	鍚屾锛歯etif_tx_lock 鑷棆閿侊紱鎵€鏈?TX 闃熷垪琚喕缁撱€?	涓婁笅鏂囷細BH 琚鐢?	娉ㄦ剰锛歯etif_queue_stopped() 淇濊瘉涓?true

ndo_set_rx_mode:
	鍚屾锛歯etif_addr_lock 鑷棆閿併€?	涓婁笅鏂囷細BH 琚鐢?	娉ㄦ剰锛氬凡寮冪敤锛屾帹鑽愪娇鐢ㄥ湪杩涚▼涓婁笅鏂囦腑杩愯鐨?ndo_set_rx_mode_async銆?
ndo_set_rx_mode_async:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?	涓婁笅鏂囷細杩涚▼锛堟潵鑷伐浣滈槦鍒楋級
	娉ㄦ剰锛歯do_set_rx_mode 鐨勫紓姝ョ増鏈紝鍦ㄨ繘绋嬩笂涓嬫枃涓繍琛屻€傛帴鏀跺崟鎾拰缁勬挱鍦板潃鍒楄〃鐨勫揩鐓с€?
ndo_change_rx_flags:
	鍚屾锛歳tnl_lock() 淇″彿閲忋€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕闇€ netdev 瀹炰緥閿併€?
ndo_setup_tc:
	`TC_SETUP_BLOCK` 鍜?`TC_SETUP_FT` 杩愯鍦?NFT 閿佷笅锛堝嵆娌℃湁 `rtnl_lock`锛屼篃娌℃湁璁惧瀹炰緥閿侊級銆傚叾浣欑殑 `tc_setup_type` 绫诲瀷鍦ㄩ┍鍔ㄥ疄鐜颁簡闃熷垪绠＄悊鎴栨暣褰?API 鏃讹紝杩愯鍦?netdev 瀹炰緥閿佷笅銆?
涓婇潰鍒楄〃鏈寚瀹氱殑澶у鏁?ndo 鍥炶皟閮借繍琛屽湪 `rtnl_lock` 涓嬨€傛澶栵紝濡傛灉椹卞姩瀹炵幇浜嗛槦鍒楃鐞嗘垨鏁村舰 API锛岃繕浼氬悓鏃惰幏鍙?netdev 瀹炰緥閿併€?
## struct napi_struct 鍚屾瑙勫垯

napi->poll:
	鍚屾锛?		napi->state 涓殑 NAPI_STATE_SCHED 浣嶃€傝澶囩殑 ndo_stop 鏂规硶浼氬鎵€鏈?NAPI 瀹炰緥璋冪敤 napi_disable()锛屽畠浼氶拡瀵?NAPI_STATE_SCHED napi->state 浣嶈繘琛岀潯鐪犲紡杞锛岀瓑寰呮墍鏈夋湭鍐崇殑 NAPI 娲诲姩鍋滄銆?
	涓婁笅鏂囷細
		杞腑鏂紙softirq锛?		浼氳 netconsole 鍦ㄤ腑鏂绂佺敤鐨勬儏鍐典笅璋冪敤銆?
## netdev 瀹炰緥閿?

鍘嗗彶涓婏紝鎵€鏈夌綉缁滄帶鍒舵搷浣滈兘鐢变竴涓О涓?`rtnl_lock` 鐨勫崟涓€鍏ㄥ眬閿佷繚鎶ゃ€傜洰鍓嶆湁涓€椤规寔缁殑鍔姏锛岃鐢ㄦ瘡涓綉缁滃懡鍚嶇┖闂寸嫭绔嬬殑閿佹潵鍙栦唬杩欎釜鍏ㄥ眬閿併€傛澶栵紝鍗曚釜 netdev 鐨勫睘鎬ц秺鏉ヨ秺澶氬湴鐢?per-netdev 閿佷繚鎶ゃ€?
瀵逛簬瀹炵幇浜嗘暣褰㈡垨闃熷垪绠＄悊 API 鐨勮澶囬┍鍔紝鎵€鏈夋帶鍒舵搷浣滈兘灏嗗湪 netdev 瀹炰緥閿佷笅杩涜銆傞┍鍔ㄤ篃鍙互閫氳繃灏?`request_ops_lock` 璁句负 true锛屾樉寮忚姹傚湪鎿嶄綔锛坥ps锛夋湡闂存寔鏈夊疄渚嬮攣銆備唬鐮佹敞閲婂拰鏂囨。灏嗘搷浣滃湪瀹炰緥閿佷笅琚皟鐢ㄧ殑椹卞姩绉颁负"ops locked"锛堥攣瀹氱殑鎿嶄綔锛夈€傚彟璇峰弬闃?struct net_device 鐨?`lock` 鎴愬憳鐨勬枃妗ｃ€?
杩樺瓨鍦ㄤ竴绉嶄緷娆¤幏鍙栦袱涓?per-netdev 閿佺殑鎯呭喌锛氬綋 netdev 闃熷垪琚鍊燂紙lease锛夋椂锛屽嵆铏氭嫙璁惧鍜岀墿鐞嗚澶囩殑 netdev 浣滅敤鍩熼攣閮借鑾峰彇銆備负闃叉姝婚攣锛岃櫄鎷熻澶囩殑閿佸繀椤诲缁堝湪鐗╃悊璁惧鐨勯攣涔嬪墠鑾峰彇锛堝弬瑙?`netdev_nl_queue_create_doit`锛夈€?
灏嗘潵锛屼細鏈夐€夐」鍏佽鍚勪釜椹卞姩閫夋嫨涓嶄娇鐢?`rtnl_lock`锛岃€屾槸鐩存帴鍦ㄥ叾 netdev 瀹炰緥閿佷笅鎵ц鎺у埗鎿嶄綔銆?
榧撳姳璁惧椹卞姩灏藉彲鑳戒緷璧栧疄渚嬮攣銆?
瀵逛簬闇€瑕佷笌鍘熸牳蹇冩爤浜や簰鐨勶紙涓昏鏄蒋浠剁殑锛夐┍鍔紝鏈変袱缁勬帴鍙ｏ細`dev_xxx`/`netdev_xxx` 鍜?`netif_xxx`锛堜緥濡?`dev_set_mtu` 鍜?`netif_set_mtu`锛夈€俙dev_xxx`/`netdev_xxx` 鍑芥暟鑷繁璐熻矗鑾峰彇瀹炰緥閿侊紝鑰?`netif_xxx` 鍑芥暟鍋囧畾椹卞姩宸茬粡鑾峰彇浜嗗疄渚嬮攣銆?
### struct net_device_ops


瀵逛簬澶у鏁伴┍鍔紝`ndos` 鍦ㄤ笉鎸佹湁瀹炰緥閿佺殑鎯呭喌涓嬭璋冪敤銆?
瀵逛簬"ops locked"椹卞姩锛屽ぇ澶氭暟 `ndos` 浼氬湪瀹炰緥閿佷笅琚皟鐢ㄣ€?
### struct ethtool_ops


涓?`ndos` 绫讳技锛屽疄渚嬮攣浠呭閫夊畾鐨勯┍鍔ㄦ寔鏈夈€傚浜?ops locked"椹卞姩锛屾墍鏈?ethtool 鎿嶄綔鏃犱竴渚嬪閮藉簲鍦ㄥ疄渚嬮攣涓嬭皟鐢ㄣ€?
### struct netdev_stat_ops


瀵逛簬"ops locked"椹卞姩锛?qstat"鎿嶄綔鍦ㄥ疄渚嬮攣涓嬭璋冪敤锛岃€屽浜庢墍鏈夊叾浠栭┍鍔ㄥ垯鍦?rtnl_lock 涓嬭皟鐢ㄣ€?
### struct net_shaper_ops


鎵€鏈夌綉缁滄暣褰紙net shaper锛夊洖璋冨湪鎸佹湁 netdev 瀹炰緥閿佹椂琚皟鐢ㄣ€俙rtnl_lock` 鍙兘鎸佹湁锛屼篃鍙兘鏈寔鏈夈€?
娉ㄦ剰锛屾敮鎸佺綉缁滄暣褰細鑷姩鍚敤"ops locking"锛堟搷浣滈攣瀹氾級銆?
### struct netdev_queue_mgmt_ops


鎵€鏈夐槦鍒楃鐞嗗洖璋冨湪鎸佹湁 netdev 瀹炰緥閿佹椂琚皟鐢ㄣ€俙rtnl_lock` 鍙兘鎸佹湁锛屼篃鍙兘鏈寔鏈夈€?
娉ㄦ剰锛屾敮鎸?struct netdev_queue_mgmt_ops 浼氳嚜鍔ㄥ惎鐢?ops locking"锛堟搷浣滈攣瀹氾級銆?
### 閫氱煡閾撅紙Notifiers锛変笌 netdev 瀹炰緥閿?

瀵逛簬瀹炵幇浜嗘暣褰㈡垨闃熷垪绠＄悊 API 鐨勮澶囬┍鍔紝閮ㄥ垎閫氱煡锛坄enum netdev_cmd`锛夎繍琛屽湪 netdev 瀹炰緥閿佷笅銆?
浠ヤ笅 netdev 閫氱煡閾炬€绘槸鍦ㄥ疄渚嬮攣涓嬭繍琛岋細
- `NETDEV_XDP_FEAT_CHANGE`

瀵逛簬鍏锋湁閿佸畾鎿嶄綔鐨勮澶囷紝鐩墠鍙湁浠ヤ笅閫氱煡閾惧湪閿佷笅杩愯锛?- `NETDEV_CHANGE`
- `NETDEV_REGISTER`
- `NETDEV_UP`

浠ヤ笅閫氱煡閾惧湪娌℃湁閿佺殑鎯呭喌涓嬭繍琛岋細
- `NETDEV_UNREGISTER`

瀵逛簬鍏朵綑閫氱煡閾炬病鏈夋槑纭殑棰勬湡銆備笉鍦ㄥ垪琛ㄤ腑鐨勯€氱煡閾惧彲鑳藉甫閿佹垨涓嶅甫閿佽繍琛岋紝鐢氳嚦鍙兘浠庝笉鍚屼唬鐮佽矾寰勪互甯﹂攣鍜屼笉甯﹂攣涓ょ鏂瑰紡璋冪敤鍚屼竴绫诲瀷鐨勯€氱煡閾俱€傜洰鏍囨槸鏈€缁堢‘淇濇墍鏈夛紙鎴栧ぇ澶氭暟锛岄櫎灏戞暟鏈夋枃妗ｈ鏄庣殑渚嬪锛夐€氱煡閾鹃兘鍦ㄥ疄渚嬮攣涓嬭繍琛屻€傛瘡褰撲綘瀵规煇涓€氱煡閾句笅鎸佹湁閿佸仛鍑烘槑纭亣璁炬椂锛岃鎵╁睍鏈枃妗ｃ€?
## NETDEV_INTERNAL 绗﹀彿鍛藉悕绌洪棿


浠?NETDEV_INTERNAL 瀵煎嚭鐨勭鍙峰彧鑳界敤浜庣綉缁滄牳蹇冧互鍙婁笌涓荤綉缁滈偖浠跺垪琛ㄥ拰鏍戯紙tree锛夌洿鎺ュ鎺ョ殑椹卞姩銆傛敞鎰忓弽涔嬩笉鎴愮珛锛孨ETDEV_INTERNAL 涔嬪鐨勫ぇ澶氭暟绗﹀彿涔熶笉搴旇 netdev 涔嬪鐨勯殢鏈轰唬鐮佷娇鐢ㄣ€傜鍙蜂箣鎵€浠ョ己灏戣鏍囪瘑锛屽彲鑳芥槸鍥犱负瀹冧滑鏃╀簬鍛藉悕绌洪棿鐨勫嚭鐜帮紝鎴栬€呬粎浠呮槸鐢变簬鐤忓拷銆?
