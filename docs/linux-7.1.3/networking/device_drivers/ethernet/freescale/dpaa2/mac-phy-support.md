
## DPAA2 MAC / PHY 鏀寔


:Copyright: |copy| 2019 NXP

### 姒傝堪


DPAA2 MAC / PHY 鏀寔鐢变竴缁?API 缁勬垚锛岃繖浜?API 甯姪 DPAA2 缃戠粶椹卞姩锛坉paa2-eth銆?dpaa2-ethsw锛変笌 PHY 搴撲氦浜掋€?
### DPAA2 杞欢鏋舵瀯


鍦ㄥ叾瀹?DPAA2 瀵硅薄涔嬩腑锛宖sl-mc 鎬荤嚎瀵煎嚭 DPNI 瀵硅薄锛堟娊璞＄綉缁滄帴鍙ｏ級鍜?DPMAC 瀵硅薄
锛堟娊璞?MAC锛夈€俤paa2-eth 椹卞姩鍦?DPNI 瀵硅薄涓婃帰娴嬶紝骞跺€熷姪 phylink 杩炴帴骞堕厤缃竴涓?DPMAC 瀵硅薄銆?
鍙互鍦?DPNI 涓?DPMAC 涔嬮棿锛屾垨涓や釜 DPNI 涔嬮棿寤虹珛鏁版嵁杩炴帴銆傛牴鎹繛鎺ョ被鍨嬬殑涓嶅悓锛?netif_carrier_[on/off] 鐢?dpaa2-eth 椹卞姩鎴?phylink 鐩存帴澶勭悊銆?

  鐢?MC 鍥轰欢鍛堢幇鐨勬娊璞￠摼璺姸鎬佷俊鎭殑鏉ユ簮

                                               +--------------------------------------+
  +------------+                  +---------+  |                           xgmac_mdio |
  | net_device |                  | phylink |--|  +-----+  +-----+  +-----+  +-----+  |
  +------------+                  +---------+  |  | PHY |  | PHY |  | PHY |  | PHY |  |
        |                             |        |  +-----+  +-----+  +-----+  +-----+  |
      +------------------------------------+   |                    External MDIO bus |
      |            dpaa2-eth               |   +--------------------------------------+
      +------------------------------------+
#         |                             |                                           Linux

        |                             |                                     MC firmware
        |              /|             V
  +----------+        / |       +----------+
  |          |       /  |       |          |
  |          |       |  |       |          |
  |   DPNI   |<------|  |<------|   DPMAC  |
  |          |       |  |       |          |
  |          |       \  |<---+  |          |
  +----------+        \ |    |  +----------+
                       \|    |
                             |
           +--------------------------------------+
           | MC firmware polling MAC PCS for link |
           |  +-----+  +-----+  +-----+  +-----+  |
           |  | PCS |  | PCS |  | PCS |  | PCS |  |
           |  +-----+  +-----+  +-----+  +-----+  |
           |                    Internal MDIO bus |
           +--------------------------------------+


鏍规嵁 MC 鍥轰欢閰嶇疆璁剧疆鐨勪笉鍚岋紝姣忎釜 MAC 鍙兘澶勪簬涓ょ妯″紡涔嬩竴锛?
- DPMAC_LINK_TYPE_FIXED锛氶摼璺姸鎬佺鐞嗗畬鍏ㄧ敱 MC 鍥轰欢閫氳繃杞 MAC PCS 鏉ュ鐞嗐€傛棤闇€
  娉ㄥ唽 phylink 瀹炰緥锛宒paa2-eth 椹卞姩鏍规湰涓嶄細缁戝畾鍒版墍杩炴帴鐨?dpmac 瀵硅薄銆?
- DPMAC_LINK_TYPE_PHY锛歁C 鍥轰欢澶勪簬绛夊緟閾捐矾鐘舵€佹洿鏂颁簨浠剁殑鐘舵€侊紝浣嗚繖浜涗簨浠跺疄闄呬笂
  涓ユ牸鍦?dpaa2-mac锛堝熀浜?phylink锛変笌鍏舵墍杩炴帴鐨?net_device 椹卞姩锛坉paa2-eth銆?  dpaa2-ethsw锛変箣闂翠紶閫掞紝鏈夋晥鍦扮粫杩囦簡鍥轰欢銆?
### 瀹炵幇


鍦ㄦ帰娴嬫椂鎴栧綋 DPNI 鐨勭鐐硅鍔ㄦ€佹洿鏀规椂锛宒paa2-eth 璐熻矗鏌ユ槑瀵圭瀵硅薄鏄惁涓?DPMAC锛?濡傛灉鏄紝鍒欎娇鐢?dpaa2_mac_connect() API 灏嗗叾涓?PHYLINK 闆嗘垚锛岃 API 灏嗘墽琛屼互涓?鎿嶄綔锛?
 - 鍦ㄨ澶囨爲涓煡鎵句笌 PHYLINK 鍏煎鐨勭粦瀹氾紙phy-handle锛? - 灏嗗垱寤轰竴涓笌鎵€鎺ユ敹 net_device 鍏宠仈鐨?PHYLINK 瀹炰緥
 - 浣跨敤 phylink_of_phy_connect() 杩炴帴鍒?PHY

瀹炵幇浜嗕互涓?phylink_mac_ops 鍥炶皟锛?
 - .validate() 灏嗙敤 MAC 鑳藉姏濉厖鍙楁敮鎸佺殑閾捐矾妯″紡锛屼粎褰?phy_interface_t 涓?   RGMII_* 鏃讹紙鐩墠锛岃繖鏄┍鍔ㄦ敮鎸佺殑鍞竴绉嶉摼璺被鍨嬶級銆?
 - .mac_config() 灏嗕娇鐢?dpmac_set_link_state() MC 鍥轰欢 API 浠ユ柊閰嶇疆閰嶇疆 MAC銆?
 - .mac_link_up() / .mac_link_down() 灏嗕娇鐢ㄤ笂杩扮浉鍚岀殑 API 鏇存柊 MAC 閾捐矾銆?
鍦ㄩ┍鍔?unbind() 鏃舵垨褰?DPNI 瀵硅薄涓?DPMAC 鏂紑杩炴帴鏃讹紝dpaa2-eth 椹卞姩璋冪敤
dpaa2_mac_disconnect()锛屽悗鑰呭弽杩囨潵浼氭柇寮€涓?PHY 鐨勮繛鎺ュ苟閿€姣?PHYLINK 瀹炰緥銆?
鍦?DPNI-DPMAC 杩炴帴鐨勬儏鍐典笅锛?ip link set dev eth0 up' 灏嗗惎鍔ㄤ互涓嬫搷浣滃簭鍒楋細

(1) 浠?.dev_open() 璋冪敤 phylink_start()銆?(2) .mac_config() 鍜?.mac_link_up() 鍥炶皟鐢?PHYLINK 璋冪敤銆?(3) 涓轰簡閰嶇疆纭欢 MAC锛岃皟鐢?MC 鍥轰欢 API dpmac_set_link_state()銆?(4) 鍥轰欢鏈€缁堜細灏嗙‖浠?MAC 璁剧疆涓烘柊閰嶇疆銆?(5) 鐩存帴浠?PHYLINK 鍦ㄥ叧鑱旂殑 net_device 涓婅皟鐢?netif_carrier_on()銆?(6) dpaa2-eth 椹卞姩澶勭悊 LINK_STATE_CHANGE 涓柇锛屼互鏍规嵁鏆傚仠甯ц缃惎鐢?绂佺敤 Rx
    taildrop銆?

  +---------+               +---------+
  | PHYLINK |-------------->|  eth0   |
  +---------+           (5) +---------+
  (1) ^  |
      |  |
      |  v (2)
  +-----------------------------------+
  |             dpaa2-eth             |
  +-----------------------------------+
         |                    ^ (6)
         |                    |
         v (3)                |
  +---------+---------------+---------+
  |  DPMAC  |               |  DPNI   |
  +---------+               +---------+
  |            MC Firmware            |
  +-----------------------------------+
         |
         |
         v (4)
  +-----------------------------------+
  |             HW MAC                |
  +-----------------------------------+

鍦?DPNI-DPNI 杩炴帴鐨勬儏鍐典笅锛岄€氬父鐨勬搷浣滃簭鍒楀涓嬫墍绀猴細

(1) ip link set dev eth0 up
(2) 鍦ㄦ墍鍏宠仈鐨?fsl_mc_device 涓婅皟鐢?dpni_enable() MC API銆?(3) ip link set dev eth1 up
(4) 鍦ㄦ墍鍏宠仈鐨?fsl_mc_device 涓婅皟鐢?dpni_enable() MC API銆?(5) LINK_STATE_CHANGED 涓柇琚?dpaa2-eth 椹卞姩鐨勪袱涓疄渚嬫帴鏀讹紝鍥犱负鐜板湪鎿嶄綔閾捐矾鐘舵€?    涓?up銆?(6) 浠?link_state_update() 鍦ㄥ鍑虹殑 net_device 涓婅皟鐢?netif_carrier_on()銆?

  +---------+               +---------+
  |  eth0   |               |  eth1   |
  +---------+               +---------+
      |  ^                     ^  |
      |  |                     |  |
  (1) v  | (6)             (6) |  v (3)
  +---------+               +---------+
  |dpaa2-eth|               |dpaa2-eth|
  +---------+               +---------+
      |  ^                     ^  |
      |  |                     |  |
  (2) v  | (5)             (5) |  v (4)
  +---------+---------------+---------+
  |  DPNI   |               |  DPNI   |
  +---------+               +---------+
  |            MC Firmware            |
  +-----------------------------------+


### 瀵煎嚭鐨?API


浠讳綍椹卞姩 DPMAC 瀵硅薄绔偣鐨?DPAA2 椹卞姩閮藉簲褰撳鐞嗗叾 _EVENT_ENDPOINT_CHANGED 涓柇锛屽苟
涓庡叧鑱旂殑 DPMAC 杩炴帴/鏂紑
```

 - int dpaa2_mac_connect(struct dpaa2_mac *mac);
 - void dpaa2_mac_disconnect(struct dpaa2_mac *mac);

```
鍙湁褰撳绔?DPMAC 涓嶆槸 `TYPE_FIXED` 鏃讹紝鎵嶉渶瑕?phylink 闆嗘垚銆傝繖鎰忓懗鐫€瀹冭涔堟槸
`TYPE_PHY`锛岃涔堟槸 `TYPE_BACKPLANE`锛堜簩鑰呯殑鍖哄埆鍦ㄤ簬锛屽湪 `TYPE_BACKPLANE` 妯″紡涓嬶紝
MC 鍥轰欢涓嶈闂?PCS 瀵勫瓨鍣級銆傚彲浠ユ鏌?```

 - static inline bool dpaa2_mac_is_type_phy(struct dpaa2_mac *mac);

```
鍦ㄨ繛鎺ュ埌 MAC 涔嬪墠锛岃皟鐢ㄨ€呭繀椤诲垎閰嶅苟鐢ㄥ叧鑱旂殑 net_device銆佽浣跨敤鐨?MC portal 鎸囬拡
浠ュ強 DPMAC 鐨勫疄闄?fsl_mc_device 缁撴瀯濉厖 dpaa2_mac 缁撴瀯銆?