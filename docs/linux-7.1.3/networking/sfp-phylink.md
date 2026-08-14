
## phylink


## 姒傝堪


phylink 鏄竴绉嶆敮鎸佺儹鎻掓嫈缃戠粶妯″潡鐩存帴杩炴帴鍒?MAC 鐨勬満鍒讹紝鏃犻渶鍦ㄧ儹鎻掓嫈浜嬩欢鏃?
閲嶆柊鍒濆鍖栭€傞厤鍣ㄣ€?

鐩墠 phylink 鏀寔浼犵粺鐨勫熀浜?phylib 鐨勮缃€佸浐瀹氶摼璺缃互鍙?SFP锛圫mall
Formfactor Pluggable锛屽皬灏哄鍙彃鎷旓級妯″潡銆?

## 鎿嶄綔妯″紡


phylink 鏈夊绉嶆搷浣滄ā寮忥紝鍙栧喅浜庡浐浠惰缃€?

1. PHY 妯″紡

   鍦?PHY 妯″紡涓嬶紝鎴戜滑浣跨敤 phylib 浠?PHY 璇诲彇褰撳墠閾捐矾璁剧疆锛屽苟灏嗗叾浼犻€掔粰 MAC
   椹卞姩銆傛垜浠湡鏈?MAC 椹卞姩绮剧‘鍦伴厤缃墍鎸囧畾鐨勬ā寮忥紝鑰岄摼璺笂涓嶅惎鐢ㄤ换浣曞崗鍟嗐€?

2. 鍥哄畾妯″紡

   灏?MAC 椹卞姩鑰岃█锛屽浐瀹氭ā寮忎笌 PHY 妯″紡鐩稿悓銆?

3. 甯﹀唴锛坕n-band锛夋ā寮?

   甯﹀唴妯″紡鐢ㄤ簬 802.3z銆丼GMII 浠ュ強绫讳技鐨勬帴鍙ｆā寮忥紝鎴戜滑鏈熸湜浣跨敤骞堕伒寰法 serdes
   閫氶亾鍙戦€佺殑甯﹀唴鍗忓晢鎴栨帶鍒跺瓧銆?

涓句緥鏉ヨ锛岃繖鎰忓懗鐫€锛?

  &eth {
    phy = <&phy>;
    phy-mode = "sgmii";
  };

涓嶄娇鐢ㄥ甫鍐?SGMII 淇″彿銆侾HY 搴斿綋涓ユ牸閬靛惊鍏?`mac_config` 鍑芥暟涓粰瀹氱殑璁剧疆銆?
閾捐矾搴斿湪 `mac_link_up` 涓?`mac_link_down` 鍑芥暟涓閫傚綋鍦板己鍒朵负 up 鎴?down銆?

  &eth {
    managed = "in-band-status";
    phy = <&phy>;
    phy-mode = "sgmii";
  };

浣跨敤甯﹀唴妯″紡锛孭HY 鍗忓晢鐨勭粨鏋滈€氳繃 SGMII 鎺у埗瀛椾紶閫掔粰 MAC锛屼笖 MAC 搴斿綋纭璇?
鎺у埗瀛椼€俙mac_link_up` 涓?`mac_link_down` 鍑芥暟涓嶅緱寮哄埗 MAC 渚х殑閾捐矾 up 鎴?down銆?

## 灏嗙綉缁滈┍鍔ㄨ浆鎹负 sfp/phylink 鐨勭矖鐣ユ寚鍗?


鏈寚鍗楃畝瑕佹弿杩板浣曞皢缃戠粶椹卞姩浠?phylib 杞崲涓?sfp/phylink 鏀寔銆傛杩庢彁浜よˉ涓?
鏉ユ敼杩涙湰鏂囨。銆?

1. 鍙€夊湴锛屽皢缃戠粶椹卞姩鐨?phylib 鏇存柊鍑芥暟鎷嗗垎涓哄鐞?link-down 涓?link-up 鐨?
   涓ら儴鍒嗐€傝繖鍙互浣滀负涓€涓嫭绔嬬殑鍑嗗鎻愪氦鏉ュ畬鎴愩€?

   杩欑鍑嗗鐨勪竴涓緝鏃╃ず渚嬪彲鍦?git 鎻愪氦 fc548b991fb0 涓壘鍒帮紝灏界褰撴椂鐨勬媶鍒嗘槸
   涓夐儴鍒嗭紱鑰?link-up 閮ㄥ垎鐜板湪宸插寘鍚负閾捐矾璁剧疆閰嶇疆 MAC銆傛洿澶氱浉鍏充俊鎭鍙傝
   `mac_link_up`銆?

```
	select FIXED_PHY
	select PHYLIB

   with::

	select PHYLINK

   in the driver's Kconfig stanza.
```

```
	#include <linux/phylink.h>

   to the driver's list of header files.
```

```
	struct phylink *phylink;
	struct phylink_config phylink_config;

   to the driver's private data structure.  We shall refer to the
   driver's private data pointer as ``priv`` below, and the driver's
   private data structure as ``struct foo_priv``.
```

5. 鏇挎崲浠ヤ笅鍑芥暟锛?

```
    :header-rows: 1
    :widths: 1 1
    :stub-columns: 0

    * - Original function
      - Replacement function
    * - phy_start(phydev)
      - phylink_start(priv->phylink)
    * - phy_stop(phydev)
      - phylink_stop(priv->phylink)
    * - phy_mii_ioctl(phydev, ifr, cmd)
      - phylink_mii_ioctl(priv->phylink, ifr, cmd)
    * - phy_ethtool_get_wol(phydev, wol)
      - phylink_ethtool_get_wol(priv->phylink, wol)
    * - phy_ethtool_set_wol(phydev, wol)
      - phylink_ethtool_set_wol(priv->phylink, wol)
    * - phy_disconnect(phydev)
      - phylink_disconnect_phy(priv->phylink)

   Please note that some of these functions must be called under the
   rtnl lock, and will warn if not. This will normally be the case,
   except if these are called from the driver suspend/resume paths.
```

6. 鐢ㄤ互涓嬫柟娉曟坊鍔?鏇挎崲 ksettings 鐨?get/set锛?

   .. code-block:: c

	static int foo_ethtool_set_link_ksettings(struct net_device *dev,
						  const struct ethtool_link_ksettings *cmd)
	{
		struct foo_priv *priv = netdev_priv(dev);

		return phylink_ethtool_ksettings_set(priv->phylink, cmd);
	}

	static int foo_ethtool_get_link_ksettings(struct net_device *dev,
						  struct ethtool_link_ksettings *cmd)
	{
		struct foo_priv *priv = netdev_priv(dev);

		return phylink_ethtool_ksettings_get(priv->phylink, cmd);
	}
	phy_dev = of_phy_connect(dev, node, link_func, flags, phy_interface);

   浠ュ強灏嗙浉鍏充唬鐮佹浛鎹负瀵逛互涓嬪嚱鏁扮殑璋冪敤锛?

	err = phylink_of_phy_connect(priv->phylink, node, flags);

   鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝``flags`` 鍙互涓洪浂锛涘鏋?DT 鑺傜偣 ``node`` 涓寚瀹氫簡 PHY锛岃繖浜?
   flags 浼氳浼犲叆璇ュ嚱鏁拌皟鐢ㄥ唴閮ㄧ殑 phy_attach_direct()銆?

   ``node`` 搴斿綋鏄寘鍚?network phy 灞炴€с€乫ixed link 灞炴€э紝骞朵笖涔熷皢鍖呭惈 sfp
   灞炴€х殑 DT 鑺傜偣銆?

   鍥哄畾閾捐矾鐨勮缃篃搴旇绉婚櫎锛涜繖浜涚敱 phylink 鍦ㄥ唴閮ㄥ鐞嗐€?

   of_phy_connect() 杩樹紶鍏ヤ簡涓€涓敤浜庨摼璺洿鏂扮殑鍑芥暟鎸囬拡銆傝鍑芥暟琚浛鎹负涓嬫枃
   (8) 涓弿杩扮殑涓€绉嶄笉鍚屽舰寮忕殑 MAC 鏇存柊銆?

   PHY 鐨?supported/advertised 鐨勬搷鎺у彂鐢熷湪 phylink 鍐呴儴锛屽熀浜?validate 鍥炶皟锛?
   瑙佷笅鏂?(8)銆?

   娉ㄦ剰锛岄┍鍔ㄤ笉鍐嶉渶瑕佸瓨鍌?``phy_interface``锛屽悓鏃朵篃瑕佹敞鎰?``phy_interface``
   鍙樻垚浜嗕竴涓姩鎬佸睘鎬э紝灏卞儚 speed銆乨uplex 绛夎缃竴鏍枫€?

   鏈€鍚庯紝娉ㄦ剰 MAC 椹卞姩涓嶅啀鑳界洿鎺ヨ闂?PHY锛涜繖鏄洜涓哄湪 phylink 妯″瀷涓紝PHY 鍙互鏄?
   鍔ㄦ€佺殑銆?

8. 鍚戦┍鍔ㄤ腑娣诲姞涓€涓?`struct phylink_mac_ops <phylink_mac_ops>` 瀹炰緥锛屽畠鏄竴涓?
   鍑芥暟鎸囬拡琛紝骞跺疄鐜拌繖浜涘嚱鏁般€傞拡瀵?`of_phy_connect` 鐨勬棫閾捐矾鏇存柊鍑芥暟鍙樻垚浜?
   涓変釜鏂规硶锛歚mac_link_up`銆乣mac_link_down` 鍜?`mac_config`銆傚鏋滄墽琛屼簡绗?1 姝ワ紝
   閭ｄ箞鐩稿叧鍔熻兘搴斿綋宸茬粡鍦ㄩ偅閲岃鎷嗗垎浜嗐€?

   閲嶈鐨勬槸锛屽鏋滀娇鐢ㄤ簡甯﹀唴鍗忓晢锛屽垯 `mac_link_up` 涓?`mac_link_down` 涓嶅緱闃绘
   甯﹀唴鍗忓晢瀹屾垚锛屽洜涓鸿繖浜涘嚱鏁版槸鍦ㄥ甫鍐呴摼璺姸鎬佹敼鍙樻椂琚皟鐢ㄧ殑鈥斺€斿惁鍒欓摼璺皢姘歌繙
   鏃犳硶寤虹珛銆?

   `mac_get_caps` 鏂规硶鏄彲閫夌殑锛屽鏋滄彁渚涳紝搴旇繑鍥炴墍浼犲叆 `interface` 妯″紡鎵€鏀寔鐨?
   phylink MAC capabilities銆備竴鑸潵璇达紝娌℃湁蹇呰瀹炵幇姝ゆ柟娉曘€侾hylink 浼氬皢杩欎簺
   capabilities 涓?`interface` 鐨勫厑璁?capabilities 缁撳悎锛屼互纭畾鍏佽鐨?ethtool
   閾捐矾妯″紡銆?

   `mac_link_state` 鏂规硶鐢ㄤ簬浠?MAC 璇诲彇閾捐矾鐘舵€侊紝骞跺洖鎶?MAC 褰撳墠姝ｅ湪浣跨敤鐨勮缃€?
   杩欏浜?1000base-X 涓?SGMII 绛夊甫鍐呭崗鍟嗘柟娉曞挨涓洪噸瑕併€?

   `mac_link_up` 鏂规硶鐢ㄤ簬閫氱煡 MAC 閾捐矾宸茬粡寤虹珛銆傝璋冪敤鍖呭惈鍗忓晢妯″紡涓庢帴鍙ｏ紝浠呬緵
   鍙傝€冦€傚悓鏃朵篃浼氭彁渚涙渶缁堢‘瀹氱殑閾捐矾鍙傛暟锛坰peed銆乨uplex 涓庢祦鎺у埗/pause 浣胯兘璁剧疆锛夛紝
   褰?MAC 涓?PCS 涓嶆槸绱у瘑闆嗘垚锛屾垨鑰呰缃笉鏄潵鑷甫鍐呭崗鍟嗘椂锛屽簲褰撶敤杩欎簺鍙傛暟鏉ラ厤缃?
   MAC銆?

   `mac_config` 鏂规硶鐢ㄤ簬浠ヨ姹傜殑鐘舵€佹洿鏂?MAC锛屽苟涓斿湪瀵?MAC 閰嶇疆鍋氭敼鍔ㄦ椂蹇呴』閬垮厤
   涓嶅繀瑕佸湴璁╅摼璺?down銆傝繖鎰忓懗鐫€璇ュ嚱鏁板簲褰撲慨鏀圭姸鎬侊紝骞朵笖浠呭湪缁濆蹇呴』鏀瑰彉 MAC
   閰嶇疆鏃舵墠璁╅摼璺?down銆傚叧浜庡浣曞仛鍒拌繖涓€鐐圭殑绀轰緥锛屽彲浠ュ弬瑙?
   `drivers/net/ethernet/marvell/mvneta.c` 涓殑 `mvneta_mac_config`銆?

   鍏充簬杩欎簺鏂规硶鐨勬洿澶氫俊鎭紝璇峰弬闃?`struct phylink_mac_ops <phylink_mac_ops>` 涓殑
   鍐呰仈鏂囨。銆?

9. 鐢ㄤ笌浣犵殑 `struct net_device <net_device>` 鍏宠仈鐨?`struct device <device>`
   寮曠敤濉厖 `struct phylink_config <phylink_config>` 鐨勫瓧娈碉細

   .. code-block:: c

	priv->phylink_config.dev = &dev.dev;
	priv->phylink_config.type = PHYLINK_NETDEV;

   濉厖浣犵殑 MAC 鑳藉澶勭悊鐨勯€熷害銆乸ause 涓?duplex 妯″紡锛?

   .. code-block:: c

        priv->phylink_config.mac_capabilities = MAC_SYM_PAUSE | MAC_10 | MAC_100 | MAC_1000FD;

10. 涓€浜涗互澶綉鎺у埗鍣ㄤ笌 PCS锛圥hysical Coding Sublayer锛岀墿鐞嗙紪鐮佸瓙灞傦級鍧楅厤瀵瑰伐浣滐紝
    PCS 闄ゅ叾浠栧杩樿兘澶勭悊缂栫爜/瑙ｇ爜銆侀摼璺缓绔嬫娴嬩笌鑷崗鍟嗐€傝櫧鐒舵煇浜?MAC 鍏锋湁鍐呴儴
    PCS 涓斿叾鎿嶄綔鏄€忔槑鐨勶紝浣嗗彟涓€浜涘垯闇€瑕佷笓闂ㄧ殑 PCS 閰嶇疆鎵嶈兘浣块摼璺甯稿伐浣溿€傚湪閭ｇ
    鎯呭喌涓嬶紝phylink 閫氳繃 `struct phylink_pcs <phylink_pcs>` 鎻愪緵浜嗕竴涓?PCS 鎶借薄銆?

    纭浣犵殑椹卞姩鏄惁鏈変竴涓垨澶氫釜鍐呴儴 PCS 鍧楋紝浠ュ強/鎴栬€呬綘鐨勬帶鍒跺櫒鏄惁鍙互浣跨敤鍙兘
    鍦ㄥ唴閮ㄨ繛鎺ュ埌浣犳帶鍒跺櫒鐨勫閮?PCS 鍧椼€?

    濡傛灉浣犵殑鎺у埗鍣ㄦ病鏈変换浣曞唴閮?PCS锛屽彲浠ヨ烦鍒版楠?11銆?

    濡傛灉浣犵殑浠ュお缃戞帶鍒跺櫒鍖呭惈涓€涓垨澶氫釜 PCS 鍧楋紝鍦ㄤ綘鐨勯┍鍔ㄧ鏈夋暟鎹粨鏋勪腑涓烘瘡涓?PCS
    鍧楀垱寤轰竴涓?`struct phylink_pcs <phylink_pcs>` 瀹炰緥锛?

    .. code-block:: c

        struct phylink_pcs pcs;

    濉厖鐩稿叧鐨?`struct phylink_pcs_ops <phylink_pcs_ops>` 鏉ラ厤缃綘鐨?PCS銆傚垱寤轰竴涓?
    `pcs_get_state` 鍑芥暟鏉ユ姤鍛婂甫鍐呴摼璺姸鎬併€佷竴涓?`pcs_config` 鍑芥暟鏉ユ牴鎹?phylink
    鎻愪緵鐨勫弬鏁伴厤缃綘鐨?PCS锛屼互鍙婁竴涓?`pcs_validate` 鍑芥暟鏉ュ悜 phylink 鎶ュ憡浣犵殑 PCS
    鎵€鑳芥帴鍙楃殑鎵€鏈夐厤缃弬鏁帮細

    .. code-block:: c

        struct phylink_pcs_ops foo_pcs_ops = {
                .pcs_validate = foo_pcs_validate,
                .pcs_get_state = foo_pcs_get_state,
                .pcs_config = foo_pcs_config,
        };

    瀹夋帓灏?PCS 閾捐矾鐘舵€佷腑鏂浆鍙戣繘 phylink锛屾柟娉曟槸锛?

    .. code-block:: c

        phylink_pcs_change(pcs, link_is_up);

    鍏朵腑 `link_is_up` 鍦ㄩ摼璺綋鍓嶄负 up 鏃朵负 true锛屽惁鍒欎负 false銆傚鏋滄煇涓?PCS 鏃犳硶
    鎻愪緵杩欎簺涓柇锛岄偅涔堝畠搴斿湪鍒涘缓 PCS 鏃惰缃?`pcs->pcs_poll = true;`銆?

11. 濡傛灉浣犵殑鎺у埗鍣ㄤ緷璧栨垨鎺ュ彈閫氳繃鑷韩椹卞姩鎺у埗鐨勫閮?PCS 鐨勫瓨鍦紝鍦ㄤ綘鐨勯┍鍔ㄧ鏈?
    鏁版嵁缁撴瀯涓坊鍔犱竴涓寚鍚?phylink_pcs 瀹炰緥鐨勬寚閽堬細

    .. code-block:: c

        struct phylink_pcs *pcs;

    鑾峰彇瀹為檯 PCS 瀹炰緥鐨勬柟寮忓彇鍐充簬骞冲彴锛屾煇浜?PCS 浣嶄簬 MDIO 鎬荤嚎涓婏紝閫氳繃浼犲叆鎸囧悜
    鐩稿簲 `struct mii_bus <mii_bus>` 鐨勬寚閽堜互鍙婅 PCS 鍦ㄨ鎬荤嚎涓婄殑鍦板潃鏉ュ彇寰椼€傚湪鏈?
    渚嬩腑锛屾垜浠亣璁炬帶鍒跺櫒杩炴帴鍒颁竴涓?Lynx PCS 瀹炰緥锛?

    .. code-block:: c

        priv->pcs = lynx_pcs_create_mdiodev(bus, 0);

    鏌愪簺 PCS 鍙互鍩轰簬鍥轰欢淇℃伅鍙栧緱锛?

    .. code-block:: c

        priv->pcs = lynx_pcs_create_fwnode(of_fwnode_handle(node));

12. 濉厖 `mac_select_pcs` 鍥炶皟锛屽苟灏嗗叾鍔犲叆浣犵殑 `struct phylink_mac_ops
    <phylink_mac_ops>` 鎿嶄綔闆嗐€傝鍑芥暟蹇呴』杩斿洖涓€涓寚鍚戝皢鐢ㄤ簬鎵€璇锋眰閾捐矾閰嶇疆鐨勭浉搴?
    `struct phylink_pcs <phylink_pcs>` 鐨勬寚閽堬細

    .. code-block:: c

        static struct phylink_pcs **foo_select_pcs(struct phylink_config **config,
                                                  phy_interface_t interface)
        {
                struct foo_priv *priv = container_of(config, struct foo_priv,
                                                     phylink_config);

                if ( /** 'interface' needs a PCS to function **/ )
                        return priv->pcs;

                return NULL;
        }

    鍙傝 `mvpp2_select_pcs` 浣滀负涓€涓嫢鏈夊涓唴閮?PCS 鐨勯┍鍔ㄧず渚嬨€?

13. 濉厖浣犵殑 MAC 鑳藉杈撳嚭鐨勬墍鏈?`phy_interface_t <phy_interface_t>`锛堝嵆鎵€鏈?MAC 鍒?
    PHY 鐨勯摼璺ā寮忥級銆備笅闈㈢殑绀轰緥灞曠ず浜嗛拡瀵逛竴涓兘澶熷鐞嗘墍鏈?RGMII 妯″紡銆丼GMII 鍜?
    1000BaseX 鐨?MAC 鐨勯厤缃€備綘蹇呴』鏍规嵁姝?MAC 浠ュ強鎵€鏈夊叧鑱?PCS 鐨勮兘鍔涜繘琛岃皟鏁达紝鑰?
    涓嶄粎浠呮槸浣犲笇鏈涗娇鐢ㄧ殑鎺ュ彛锛?

    .. code-block:: c

       phy_interface_set_rgmii(priv->phylink_config.supported_interfaces);
        __set_bit(PHY_INTERFACE_MODE_SGMII,
                  priv->phylink_config.supported_interfaces);
        __set_bit(PHY_INTERFACE_MODE_1000BASEX,
                  priv->phylink_config.supported_interfaces);

14. 浠?probe 鍑芥暟涓Щ闄ゅ PHY 鐨?of_parse_phandle()銆佸鍥哄畾閾捐矾鐨?
    of_phy_register_fixed_link() 绛夎皟鐢紝骞舵浛鎹负锛?

    .. code-block:: c

	struct phylink *phylink;

	phylink = phylink_create(&priv->phylink_config, node, phy_mode, &phylink_ops);
	if (IS_ERR(phylink)) {
		err = PTR_ERR(phylink);
		fail probe;
	}

	priv->phylink = phylink;

    骞堕€傚綋瀹夋帓閿€姣?phylink锛氬湪 probe 澶辫触璺緞浠ュ強绉婚櫎璺緞涓兘閫氳繃璋冪敤浠ヤ笅鍑芥暟鏉?
    閿€姣侊細

    .. code-block:: c

	phylink_destroy(priv->phylink);

15. 瀹夋帓灏?MAC 閾捐矾鐘舵€佷腑鏂浆鍙戣繘 phylink锛屾柟娉曟槸锛?

    .. code-block:: c

	phylink_mac_change(priv->phylink, link_is_up);

    鍏朵腑 `link_is_up` 鍦ㄩ摼璺綋鍓嶄负 up 鏃朵负 true锛屽惁鍒欎负 false銆?

```
	netif_carrier_on()
	netif_carrier_off()

    as these will interfere with phylink's tracking of the link state,
    and cause phylink to omit calls via the :c:func:`mac_link_up` and
    :c:func:`mac_link_down` methods.
```

缃戠粶椹卞姩搴旈€氳繃瀹冧滑鐨?suspend/resume 璺緞璋冪敤 phylink_stop() 涓?phylink_start()锛?
杩欑‘淇濅簡鍦ㄥ繀瑕佹椂璋冪敤鐩稿簲鐨?`struct phylink_mac_ops <phylink_mac_ops>` 鏂规硶銆?

鍏充簬鍦?DT 涓弿杩?SFP 绗硷紙cage锛夌殑淇℃伅锛岃鍙傞槄鍐呮牳婧愮爜鏍戜腑鐨勭粦瀹氭枃妗?
`Documentation/devicetree/bindings/net/sff,sfp.yaml`銆?
