
## PHY 閾捐矾鎷撴墤锛圥HY link topology锛?

## 姒傝堪


缃戠粶鏍堜腑鐨?PHY 閾捐矾鎷撴墤琛ㄧず鏃ㄥ湪琛ㄧず浠讳綍缁欏畾浠ュお缃戦摼璺殑纭欢甯冨眬銆?
浠庣敤鎴风┖闂寸殑瑙掑害鏉ョ湅锛屼竴涓互澶綉鎺ュ彛涓嶈繃鏄竴涓?`struct net_device <net_device>`锛?瀹冮€氳繃浼犵粺鐨?ioctl 鍜?ethtool netlink 鍛戒护鏆撮湶閰嶇疆閫夐」銆傚熀鏈殑鍋囪
```

  +-----------------------+        +----------+      +--------------+
  | Ethernet Controller / |        | Ethernet |      | Connector /  |
  |       MAC             | ------ |   PHY    | ---- |    Port      | ---... to LP
  +-----------------------+        +----------+      +--------------+
  struct net_device               struct phy_device

```
闇€瑕侀厤缃?PHY 鐨勫懡浠ゅ皢閫氳繃 net_device.phydev 瀛楁鍒拌揪 PHY 骞舵墽琛岀浉鍏抽厤缃€?
褰撳嚭鐜版洿澶嶆潅鐨勬嫇鎵戞椂锛岃繖涓€鍋囪灏变細澶辨晥锛屼緥濡備娇鐢?SFP 鏀跺彂鍣ㄦ椂
锛堝敖绠″苟闈炲彧鏈夎繖涓€绉嶇壒瀹氭儏鍐碉級銆?
杩欓噷鎴戜滑鏈変袱绉嶅熀鏈満鏅€傝涔?MAC 鑳藉杈撳嚭涓茶鎺ュ彛锛屽彲浠ョ洿鎺ラ鍏?SFP 绗硷紙cage锛夛紝
渚嬪 SGMII銆?000BaseX銆?0GBaseR 绛夈€?
```

  +-----+  SGMII  +------------+
  | MAC | ------- | SFP Module |
  +-----+         +------------+

```
```

  +-----+  SGMII   +--------------+
  | MAC | -------- | PHY (on SFP) |
  +-----+          +--------------+

```
鍦ㄨ繖绉嶆儏鍐典笅锛孲FP PHY 鐢?phylib 澶勭悊锛屽苟閫氳繃鍏?SFP 涓婃父 ops 鐢?phylink 娉ㄥ唽銆?
鐜板湪涓€浜涗互澶綉鎺у埗鍣ㄦ棤娉曡緭鍑轰覆琛屾帴鍙ｏ紝鍥犳鎴戜滑涓嶈兘鐩存帴灏嗗畠浠繛鎺ュ埌 SFP 绗笺€?鐒惰€岋紝涓€浜?PHY 鍙互鐢ㄤ綔濯掍綋杞崲鍣紙media-converter锛夛紝灏嗛潪涓茶鐨?MAC MII 鎺ュ彛
杞崲涓?```

  +-----+  RGMII  +-----------------------+  SGMII  +--------------+
  | MAC | ------- | PHY (media converter) | ------- | PHY (on SFP) |
  +-----+         +-----------------------+         +--------------+

```
杩欐鏄崟涓€ net_device.phydev 鎸囬拡妯″瀷鏄鹃湶鍏跺眬闄愭€х殑鍦版柟锛屽洜涓虹幇鍦ㄩ摼璺笂
鏈変袱涓?PHY銆?
phy_link 鎷撴墤妗嗘灦鏃ㄥ湪鎻愪緵涓€绉嶆柟寮忔潵璺熻釜閾捐矾涓婄殑姣忎釜 PHY锛屼緵鍐呮牳椹卞姩鍜屽瓙绯荤粺
浣跨敤锛屽悓鏃朵篃鍚戠敤鎴风┖闂存姤鍛婃嫇鎵戯紝浠庤€屽厑璁稿湪閰嶇疆鍛戒护涓拡瀵瑰崟涓?PHY銆?
## API


`struct phy_link_topology <phy_link_topology>` 鏄竴涓?per-netdevice
璧勬簮锛屽湪缃戠粶璁惧鍒涘缓鏃跺垵濮嬪寲銆備竴鏃﹀垵濮嬪寲锛屽氨鍙互閫氳繃
`phy_link_topo_add_phy` 灏?PHY 娉ㄥ唽鍒版嫇鎵戜腑銆?
闄や簡灏?PHY 娉ㄥ唽鍒版嫇鎵戜箣澶栵紝璇ヨ皟鐢ㄨ繕浼氫负 PHY 鍒嗛厤涓€涓敮涓€绱㈠紩锛岃绱㈠紩闅忓悗
鍙互鎶ュ憡缁欑敤鎴风┖闂翠互寮曠敤姝?PHY锛堢被浼间簬 ifindex锛夈€傝绱㈠紩鏄竴涓?u32锛岃寖鍥翠粠
1 鍒?U32_MAX銆傚€?0 琚繚鐣欑敤浜庤〃绀?PHY 灏氫笉灞炰簬浠讳綍鎷撴墤銆?
鐒跺悗鍙互閫氳繃 `phy_link_topo_del_phy` 灏?PHY 浠庢嫇鎵戜腑绉婚櫎銆?
杩欎簺鍑芥暟宸茬粡鎸傛帴鍒?phylib 瀛愮郴缁熶腑锛屽洜姝ゆ墍鏈夐€氳繃 `phy_attach_direct` 閾炬帴鍒?net_device 鐨?PHY 灏嗚嚜鍔ㄥ姞鍏ヨ netdev 鐨勬嫇鎵戙€?
浣嶄簬 SFP 妯″潡涓婄殑 PHY 涔熶細鍦?SFP 涓婃父鏄?phylink锛堝嵆娌℃湁濯掍綋杞崲鍣級鏃?鑷姩娉ㄥ唽銆?
鍙敤浣?SFP 涓婃父鐨?PHY 椹卞姩闇€瑕佽皟鐢?`phy_sfp_attach_phy` 鍜?`phy_sfp_detach_phy`锛?瀹冧滑鍙互鐢ㄤ綔 `struct sfp_upstream_ops <sfp_upstream_ops>` 鐨?.attach_phy / .detach_phy 瀹炵幇銆?
## UAPI


瀛樺湪涓€缁?netlink 鍛戒护鐢ㄤ簬浠庣敤鎴风┖闂存煡璇㈤摼璺嫇鎵戯紝璇峰弬瑙?`Documentation/networking/ethtool-netlink.rst`銆?
鎷ユ湁鎷撴墤琛ㄧず鐨勫叏閮ㄦ剰涔夊湪浜庝负 `struct phy_device <phy_device>` 涓殑
phyindex 瀛楁璧嬪€笺€傝绱㈠紩浣跨敤 `ETHTOOL_MSG_PHY_GET` ethtnl 鍛戒护鎶ュ憡缁?鐢ㄦ埛绌洪棿銆傛墽琛?DUMP 鎿嶄綔灏嗗鑷村垪鍑烘墍鏈?net_device 鐨勬墍鏈?PHY銆侱UMP 鍛戒护
鎺ュ彈 `ETHTOOL_A_HEADER_DEV_INDEX` 鎴?`ETHTOOL_A_HEADER_DEV_NAME`
浣滀负璇锋眰涓紶鍏ョ殑鍙傛暟锛屼互灏?DUMP 杩囨护鍒板崟涓?net_device銆?
妫€绱㈠埌鐨勭储寮曢殢鍚庡彲浠ヤ綔涓鸿姹傚弬鏁颁娇鐢?`ETHTOOL_A_HEADER_PHY_INDEX` 瀛楁
浼犲叆浠ヤ笅 ethnl 鍛戒护锛?
- `ETHTOOL_MSG_STRSET_GET` 鐢ㄤ簬鑾峰彇缁欏畾 PHY 鐨勭粺璁″瓧绗︿覆闆?- `ETHTOOL_MSG_CABLE_TEST_ACT` 鍜?`ETHTOOL_MSG_CABLE_TEST_ACT`锛岀敤浜庡湪閾捐矾涓婄殑
  缁欏畾 PHY锛堟渶鍙兘鏄渶澶栧眰鐨?PHY锛変笂鎵ц鐢电紗娴嬭瘯
- `ETHTOOL_MSG_PSE_SET` 鍜?`ETHTOOL_MSG_PSE_GET` 鐢ㄤ簬 PHY 鎺у埗鐨?PoE 鍜?PSE 璁剧疆
- `ETHTOOL_MSG_PLCA_GET_CFG`銆乣ETHTOOL_MSG_PLCA_SET_CFG` 鍜?  `ETHTOOL_MSG_PLCA_GET_STATUS` 鐢ㄤ簬璁剧疆 PLCA锛堢墿鐞嗗眰鍐茬獊閬垮厤锛夊弬鏁?
娉ㄦ剰锛孭HY 绱㈠紩鍙互浼犻€掔粰鍏朵粬璇锋眰锛屽鏋滃瓨鍦ㄤ笖涓嶇浉鍏筹紝瀹冧滑浼氶潤榛樺拷鐣ュ畠銆?