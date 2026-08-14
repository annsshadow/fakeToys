
## Devlink 闄烽槺


## 鑳屾櫙


鑳藉鍗歌浇鍐呮牳鏁版嵁閫氳矾骞舵墽琛屾ˉ鎺ヤ笌璺敱绛夊姛鑳界殑璁惧锛岃繕蹇呴』鑳藉灏嗙壒瀹?鏁版嵁鍖呭彂閫佸埌鍐呮牳锛堝嵆 CPU锛夎繘琛屽鐞嗐€?
渚嬪锛屽厖褰撴敮鎸佺粍鎾殑妗ユ帴璁惧鐨勮澶囷紝蹇呴』鑳藉灏?IGMP 鎴愬憳鎶ュ憡鍙戦€佸埌
鍐呮牳锛岀敱妗ユ帴妯″潡澶勭悊銆傚鏋滀笉澶勭悊姝ょ被鏁版嵁鍖咃紝妗ユ帴妯″潡灏嗘案杩滄棤娉曞～鍏呭叾
MDB銆?
鍐嶄妇涓€涓緥瀛愶紝鑰冭檻涓€涓厖褰撹矾鐢卞櫒鐨勮澶囨敹鍒颁簡涓€涓?TTL 涓?1 鐨?IP 鏁版嵁鍖呫€?鍦ㄨ矾鐢辫鏁版嵁鍖呮椂锛岃澶囧繀椤诲皢鍏跺彂閫佸埌鍐呮牳锛屼互渚垮唴鏍镐篃瀵瑰叾杩涜璺敱骞剁敓鎴?ICMP Time Exceeded 閿欒鏁版嵁鎶ャ€傚鏋滀笉璁╁唴鏍歌嚜宸辫矾鐢辨绫绘暟鎹寘锛岃濡?`traceroute` 杩欐牱鐨勫伐鍏峰皢鏃犳硶宸ヤ綔銆?
灏嗘煇浜涙暟鎹寘鍙戦€佸埌鍐呮牳杩涜澶勭悊鐨勫熀鏈兘鍔涜绉颁负鈥減acket trapping鈥濓紙鏁版嵁鍖?闄烽槺锛夈€?
## 姒傝堪


`devlink-trap` 鏈哄埗鍏佽鍏峰鑳藉姏鐨勮澶囬┍鍔ㄥ悜 `devlink` 娉ㄥ唽鍏舵敮鎸佺殑鏁版嵁鍖?闄烽槺锛屽苟鍚?`devlink` 鎶ュ憡琚櫡闃辨崟鑾风殑鏁版嵁鍖呬互杩涜杩涗竴姝ュ垎鏋愩€?
鍦ㄦ帴鏀跺埌琚櫡闃辨崟鑾风殑鏁版嵁鍖呭悗锛宍devlink` 浼氭寜闄烽槺杩涜鏁版嵁鍖呬笌瀛楄妭璁℃暟锛?骞跺彲鑳介€氳繃 netlink 浜嬩欢灏嗘墍鏈夋彁渚涚殑鍏冩暟鎹紙濡傞櫡闃卞師鍥犮€佹椂闂存埑銆佽緭鍏ョ鍙ｏ級
涓€骞舵姤鍛婄粰鐢ㄦ埛绌洪棿銆傝繖瀵逛簬涓㈠純绫婚櫡闃憋紙see Trap-Types锛夊挨鍏舵湁鐢紝鍥犱负瀹冭
鐢ㄦ埛鑳藉杩涗竴姝ヤ簡瑙ｅ師鏈笉鍙鐨勪涪鍖呮儏鍐点€?
```

                                    Netlink event: Packet w/ metadata
                                                   Or a summary of recent drops
                                  ^
                                  |
         Userspace                |
        +---------------------------------------------------+
         Kernel                   |
                                  |
                          +-------+--------+
                          |                |
                          |  drop_monitor  |
                          |                |
                          +-------^--------+
                                  |
                                  | Non-control traps
                                  |
                             +----+----+
                             |         |      Kernel's Rx path
                             | devlink |      (non-drop traps)
                             |         |
                             +----^----+      ^
                                  |           |
                                  +-----------+
                                  |
                          +-------+-------+
                          |               |
                          | Device driver |
                          |               |
                          +-------^-------+
         Kernel                   |
        +---------------------------------------------------+
         Hardware                 |
                                  | Trapped packet
                                  |
                               +--+---+
                               |      |
                               | ASIC |
                               |      |
                               +------+

```

## 闄烽槺绫诲瀷


`devlink-trap` 鏈哄埗鏀寔浠ヤ笅鏁版嵁鍖呴櫡闃辩被鍨嬶細

  - `drop`锛氳闄烽槺鎹曡幏鐨勬暟鎹寘宸茶搴曞眰璁惧涓㈠純銆傛暟鎹寘浠呯敱 `devlink` 澶勭悊锛?    涓嶄細娉ㄥ叆鍒板唴鏍哥殑鎺ユ敹璺緞銆傞櫡闃卞姩浣滐紙see Trap-Actions锛夊彲浠ユ洿鏀广€?  - `exception`锛氳闄烽槺鎹曡幏鐨勬暟鎹寘鐢变簬寮傚父锛堝 TTL 閿欒銆佺己灏戦偦灞呰〃椤癸級鑰?    鏈搴曞眰璁惧鎸夐鏈熻浆鍙戯紝骞惰闄烽槺鎹曡幏鍒版帶鍒跺钩闈互杩涜瑙ｆ瀽銆傛暟鎹寘鐢?    `devlink` 澶勭悊骞舵敞鍏ュ埌鍐呮牳鐨勬帴鏀惰矾寰勩€備笉鍏佽鏇存敼姝ょ被闄烽槺鐨勫姩浣滐紝鍥犱负
    杩欏緢瀹规槗鐮村潖鎺у埗骞抽潰銆?  - `control`锛氳闄烽槺鎹曡幏鐨勬暟鎹寘琚澶囨崟鑾凤紝鍥犱负瀹冧滑鏄帶鍒跺钩闈㈡纭繍琛?    鎵€闇€鐨勬帶鍒舵暟鎹寘銆備緥濡?ARP 璇锋眰涓?IGMP 鏌ヨ鏁版嵁鍖呫€傛暟鎹寘琚敞鍏ュ埌
    鍐呮牳鐨勬帴鏀惰矾寰勶紝浣嗕笉浼氭姤鍛婄粰鍐呮牳鐨勪涪鍖呯洃瑙嗗櫒銆備笉鍏佽鏇存敼姝ょ被闄烽槺鐨?    鍔ㄤ綔锛屽洜涓鸿繖寰堝鏄撶牬鍧忔帶鍒跺钩闈€?

## 闄烽槺鍔ㄤ綔


`devlink-trap` 鏈哄埗鏀寔浠ヤ笅鏁版嵁鍖呴櫡闃卞姩浣滐細

  - `trap`锛氭暟鎹寘鐨勫敮涓€鍓湰琚彂閫佸埌 CPU銆?  - `drop`锛氭暟鎹寘琚簳灞傝澶囦涪寮冿紝涓斾笉鍙戦€佸壇鏈埌 CPU銆?  - `mirror`锛氭暟鎹寘琚簳灞傝澶囪浆鍙戯紝骞跺彂閫佷竴浠藉壇鏈埌 CPU銆?
## 閫氱敤鏁版嵁鍖呴櫡闃?

閫氱敤鏁版嵁鍖呴櫡闃辩敤浜庢弿杩版崟鑾峰畾涔夋槑纭殑鏁版嵁鍖呫€佹垨鍥犲畾涔夋槑纭殑鏉′欢锛堝 TTL
閿欒锛夎€岃鎹曡幏鐨勬暟鎹寘鐨勯櫡闃便€傛绫婚櫡闃卞彲鐢卞涓澶囬┍鍔ㄥ叡浜紝鍏舵弿杩板繀椤?娣诲姞鍒颁笅琛ㄤ腑锛?
   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `source_mac_is_multicast`
     - `drop`
     - 鎹曡幏璁惧鍥犵粍鎾簮 MAC 鑰屽喅瀹氫涪寮冪殑鍏ョ珯鏁版嵁鍖?   - - `vlan_tag_mismatch`
     - `drop`
     - 鎹曡幏璁惧鍥?VLAN 鏍囩涓嶅尮閰嶈€屽喅瀹氫涪寮冪殑鍏ョ珯鏁版嵁鍖咃細鍏ユˉ绔彛鏈厤缃?       PVID锛屼笖鏁版嵁鍖呮湭甯︽爣绛炬垨浠呭甫浼樺厛绾ф爣绛?   - - `ingress_vlan_filter`
     - `drop`
     - 鎹曡幏璁惧鍥犳暟鎹寘甯︽湁鍏ユˉ绔彛涓婃湭閰嶇疆鐨?VLAN 鑰屽喅瀹氫涪寮冪殑鍏ョ珯鏁版嵁鍖?   - - `ingress_spanning_tree_filter`
     - `drop`
     - 鎹曡幏璁惧鍥犲叆妗ョ鍙ｇ殑 STP 鐘舵€佷笉鏄?"forwarding" 鑰屽喅瀹氫涪寮冪殑鍏ョ珯鏁版嵁鍖?   - - `port_list_is_empty`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佹硾娲紙濡傛湭鐭ュ崟鎾€佹湭娉ㄥ唽缁勬挱锛変絾娌℃湁鍙硾娲殑绔彛鑰?       鍐冲畾涓㈠純鐨勬暟鎹寘
   - - `port_loopback_filter`
     - `drop`
     - 鎹曡幏璁惧鍥犱簩灞傝浆鍙戝悗鍞竴搴斿彂閫佸嚭鍘荤殑绔彛灏辨槸鎺ユ敹绔彛鑰屽喅瀹氫涪寮冪殑
       鏁版嵁鍖?   - - `blackhole_route`
     - `drop`
     - 鎹曡幏璁惧鍥犲懡涓粦娲炶矾鐢辫€屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `ttl_value_is_too_small`
     - `exception`
     - 鎹曡幏璁惧鏈簲杞彂銆佷絾 TTL 琚€掑噺鍒?0 鎴栨洿灏忕殑鍗曟挱鏁版嵁鍖?   - - `tail_drop`
     - `drop`
     - 鎹曡幏璁惧鍥犳棤娉曞叆闃熷埌宸叉弧鐨勫彂閫侀槦鍒楄€屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `non_ip`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佹墽琛屼笁灞傛煡鎵句絾涓嶆槸 IP 鎴?MPLS 鏁版嵁鍖呰€屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `uc_dip_over_mc_dmac`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖鐩殑 IP 涓哄崟鎾€岀洰鐨?MAC 涓虹粍鎾€屽喅瀹氫涪寮冪殑
       鏁版嵁鍖?   - - `dip_is_loopback_address`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖鐩殑 IP 涓虹幆鍥炲湴鍧€锛堝嵆 127.0.0.0/8 涓?::1/128锛?       鑰屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `sip_is_mc`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖婧?IP 涓虹粍鎾紙鍗?224.0.0.0/8 涓?ff::/8锛夎€屽喅瀹?       涓㈠純鐨勬暟鎹寘
   - - `sip_is_loopback_address`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖婧?IP 涓虹幆鍥炲湴鍧€锛堝嵆 127.0.0.0/8 涓?::1/128锛?       鑰屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `ip_header_corrupted`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖 IP 澶撮儴鎹熷潖锛堟牎楠屽拰閿欒銆両P 鐗堟湰閿欒鎴栬繃闀?       鐨?Internet Header Length锛圛HL锛夛級鑰屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `ipv4_sip_is_limited_bc`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖婧?IP 涓哄彈闄愬箍鎾紙鍗?255.255.255.255/32锛夎€?       鍐冲畾涓㈠純鐨勬暟鎹寘
   - - `ipv6_mc_dip_reserved_scope`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖 IPv6 缁勬挱鐩殑 IP 鍏锋湁淇濈暀鑼冨洿锛堝嵆 ffx0::/16锛?       鑰屽喅瀹氫涪寮冪殑 IPv6 鏁版嵁鍖?   - - `ipv6_mc_dip_interface_local_scope`
     - `drop`
     - 鎹曡幏璁惧鍥犻渶瑕佽矾鐢便€佷笖 IPv6 缁勬挱鐩殑 IP 鍏锋湁鎺ュ彛鏈湴鑼冨洿锛堝嵆
       ffx1::/16锛夎€屽喅瀹氫涪寮冪殑 IPv6 鏁版嵁鍖?   - - `mtu_value_is_too_small`
     - `exception`
     - 鎹曡幏鏈簲鐢辫澶囪矾鐢便€佷絾澶т簬鍑哄彛鎺ュ彛 MTU 鐨勬暟鎹寘
   - - `unresolved_neigh`
     - `exception`
     - 鎹曡幏璺敱鍚庢病鏈夊尮閰?IP 閭诲眳鐨勬暟鎹寘
   - - `mc_reverse_path_forwarding`
     - `exception`
     - 鎹曡幏缁勬挱璺敱涓湭閫氳繃鍙嶅悜璺緞杞彂锛圧PF锛夋鏌ョ殑缁勬挱 IP 鏁版嵁鍖?   - - `reject_route`
     - `exception`
     - 鎹曡幏鍛戒腑鎷掔粷璺敱锛堝嵆 "unreachable"銆?prohibit"锛夌殑鏁版嵁鍖?   - - `ipv4_lpm_miss`
     - `exception`
     - 鎹曡幏鏈尮閰嶄换浣曡矾鐢辩殑鍗曟挱 IPv4 鏁版嵁鍖?   - - `ipv6_lpm_miss`
     - `exception`
     - 鎹曡幏鏈尮閰嶄换浣曡矾鐢辩殑鍗曟挱 IPv6 鏁版嵁鍖?   - - `non_routable_packet`
     - `drop`
     - 鎹曡幏璁惧鍥犱笉搴旇璺敱鑰屽喅瀹氫涪寮冪殑鏁版嵁鍖呫€備緥濡傦紝IGMP 鏌ヨ鍙璁惧鍦?       浜屽眰娉涙椽骞跺埌杈捐矾鐢卞櫒锛屾绫绘暟鎹寘涓嶅簲琚矾鐢辫€屽簲褰撲涪寮?   - - `decap_error`
     - `exception`
     - 鎹曡幏璁惧鍥犺В灏佽澶辫触锛堝鏁版嵁鍖呰繃鐭€乂XLAN 澶撮儴涓缃簡淇濈暀姣旂壒锛夎€?       鍐冲畾涓㈠純鐨?NVE 涓?IPinIP 鏁版嵁鍖?   - - `overlay_smac_is_mc`
     - `drop`
     - 鎹曡幏璁惧鍥犲彔鍔犵綉缁滄簮 MAC 涓虹粍鎾€屽喅瀹氫涪寮冪殑 NVE 鏁版嵁鍖?   - - `ingress_flow_action_drop`
     - `drop`
     - 鎹曡幏鍦ㄥ鐞嗗叆鍚戞祦鍔ㄤ綔 drop 鏃朵涪寮冪殑鏁版嵁鍖?   - - `egress_flow_action_drop`
     - `drop`
     - 鎹曡幏鍦ㄥ鐞嗗嚭鍚戞祦鍔ㄤ綔 drop 鏃朵涪寮冪殑鏁版嵁鍖?   - - `stp`
     - `control`
     - 鎹曡幏 STP 鏁版嵁鍖?   - - `lacp`
     - `control`
     - 鎹曡幏 LACP 鏁版嵁鍖?   - - `lldp`
     - `control`
     - 鎹曡幏 LLDP 鏁版嵁鍖?   - - `igmp_query`
     - `control`
     - 鎹曡幏 IGMP 鎴愬憳鏌ヨ鏁版嵁鍖?   - - `igmp_v1_report`
     - `control`
     - 鎹曡幏 IGMP 鐗堟湰 1 鎴愬憳鎶ュ憡鏁版嵁鍖?   - - `igmp_v2_report`
     - `control`
     - 鎹曡幏 IGMP 鐗堟湰 2 鎴愬憳鎶ュ憡鏁版嵁鍖?   - - `igmp_v3_report`
     - `control`
     - 鎹曡幏 IGMP 鐗堟湰 3 鎴愬憳鎶ュ憡鏁版嵁鍖?   - - `igmp_v2_leave`
     - `control`
     - 鎹曡幏 IGMP 鐗堟湰 2 绂荤粍鏁版嵁鍖?   - - `mld_query`
     - `control`
     - 鎹曡幏 MLD 缁勬挱渚﹀惉鑰呮煡璇㈡暟鎹寘
   - - `mld_v1_report`
     - `control`
     - 鎹曡幏 MLD 鐗堟湰 1 缁勬挱渚﹀惉鑰呮姤鍛婃暟鎹寘
   - - `mld_v2_report`
     - `control`
     - 鎹曡幏 MLD 鐗堟湰 2 缁勬挱渚﹀惉鑰呮姤鍛婃暟鎹寘
   - - `mld_v1_done`
     - `control`
     - 鎹曡幏 MLD 鐗堟湰 1 缁勬挱渚﹀惉鑰呭畬鎴愭暟鎹寘
   - - `ipv4_dhcp`
     - `control`
     - 鎹曡幏 IPv4 DHCP 鏁版嵁鍖?   - - `ipv6_dhcp`
     - `control`
     - 鎹曡幏 IPv6 DHCP 鏁版嵁鍖?   - - `arp_request`
     - `control`
     - 鎹曡幏 ARP 璇锋眰鏁版嵁鍖?   - - `arp_response`
     - `control`
     - 鎹曡幏 ARP 搴旂瓟鏁版嵁鍖?   - - `arp_overlay`
     - `control`
     - 鎹曡幏鍒拌揪鍙犲姞缃戠粶銆佺粡 NVE 瑙ｅ皝瑁呯殑 ARP 鏁版嵁鍖呫€備緥濡傦紝褰撻渶瑕佽В鏋愮殑
       鍦板潃鏄湰鍦板湴鍧€鏃跺氨闇€瑕佹闄烽槺
   - - `ipv6_neigh_solicit`
     - `control`
     - 鎹曡幏 IPv6 閭诲眳璇锋眰鏁版嵁鍖?   - - `ipv6_neigh_advert`
     - `control`
     - 鎹曡幏 IPv6 閭诲眳閫氬憡鏁版嵁鍖?   - - `ipv4_bfd`
     - `control`
     - 鎹曡幏 IPv4 BFD 鏁版嵁鍖?   - - `ipv6_bfd`
     - `control`
     - 鎹曡幏 IPv6 BFD 鏁版嵁鍖?   - - `ipv4_ospf`
     - `control`
     - 鎹曡幏 IPv4 OSPF 鏁版嵁鍖?   - - `ipv6_ospf`
     - `control`
     - 鎹曡幏 IPv6 OSPF 鏁版嵁鍖?   - - `ipv4_bgp`
     - `control`
     - 鎹曡幏 IPv4 BGP 鏁版嵁鍖?   - - `ipv6_bgp`
     - `control`
     - 鎹曡幏 IPv6 BGP 鏁版嵁鍖?   - - `ipv4_vrrp`
     - `control`
     - 鎹曡幏 IPv4 VRRP 鏁版嵁鍖?   - - `ipv6_vrrp`
     - `control`
     - 鎹曡幏 IPv6 VRRP 鏁版嵁鍖?   - - `ipv4_pim`
     - `control`
     - 鎹曡幏 IPv4 PIM 鏁版嵁鍖?   - - `ipv6_pim`
     - `control`
     - 鎹曡幏 IPv6 PIM 鏁版嵁鍖?   - - `uc_loopback`
     - `control`
     - 鎹曡幏闇€瑕侀€氳繃鎺ユ敹璇ュ寘鐨勫悓涓€涓夊眰鎺ュ彛杩涜璺敱鐨勫崟鎾暟鎹寘銆傛绫绘暟鎹寘
       鐢卞唴鏍歌矾鐢憋紝浣嗕篃鍙兘浣垮叾鐢熸垚 ICMP 閲嶅畾鍚戞暟鎹寘
   - - `local_route`
     - `control`
     - 鎹曡幏鍛戒腑鏈湴璺敱銆侀渶瑕佹湰鍦版姇閫掔殑鍗曟挱鏁版嵁鍖?   - - `external_route`
     - `control`
     - 鎹曡幏搴旈€氳繃涓嶅睘浜庡悓涓€璁惧锛堝浜ゆ崲 ASIC锛夌殑澶栭儴鎺ュ彛锛堝绠＄悊鎺ュ彛锛夎繘琛?       璺敱鐨勬暟鎹寘
   - - `ipv6_uc_dip_link_local_scope`
     - `control`
     - 鎹曡幏闇€瑕佽矾鐢便€佷笖鐩殑 IP 鍦板潃鍏锋湁閾捐矾鏈湴鑼冨洿锛堝嵆 fe80::/10锛夌殑鍗曟挱
       IPv6 鏁版嵁鍖呫€傝闄烽槺鍏佽璁惧椹卞姩閬垮厤缂栫▼閾捐矾鏈湴璺敱锛屼絾浠嶈兘鎺ユ敹鐢ㄤ簬
       鏈湴鎶曢€掔殑鏁版嵁鍖?   - - `ipv6_dip_all_nodes`
     - `control`
     - 鎹曡幏鐩殑 IP 鍦板潃涓?"All Nodes Address"锛堝嵆 ff02::1锛夌殑 IPv6 鏁版嵁鍖?   - - `ipv6_dip_all_routers`
     - `control`
     - 鎹曡幏鐩殑 IP 鍦板潃涓?"All Routers Address"锛堝嵆 ff02::2锛夌殑 IPv6 鏁版嵁鍖?   - - `ipv6_router_solicit`
     - `control`
     - 鎹曡幏 IPv6 璺敱鍣ㄨ姹傛暟鎹寘
   - - `ipv6_router_advert`
     - `control`
     - 鎹曡幏 IPv6 璺敱鍣ㄩ€氬憡鏁版嵁鍖?   - - `ipv6_redirect`
     - `control`
     - 鎹曡幏 IPv6 閲嶅畾鍚戞秷鎭暟鎹寘
   - - `ipv4_router_alert`
     - `control`
     - 鎹曡幏闇€瑕佽矾鐢便€佷笖鍖呭惈 Router Alert 閫夐」鐨?IPv4 鏁版嵁鍖呫€傛绫绘暟鎹寘闇€瑕?       鏈湴鎶曢€掑埌璁剧疆浜?IP_ROUTER_ALERT socket 閫夐」鐨勫師濮嬪鎺ュ瓧
   - - `ipv6_router_alert`
     - `control`
     - 鎹曡幏闇€瑕佽矾鐢便€佷笖鍦ㄥ叾閫愯烦鎵╁睍澶撮儴涓寘鍚?Router Alert 閫夐」鐨?IPv6
       鏁版嵁鍖呫€傛绫绘暟鎹寘闇€瑕佹湰鍦版姇閫掑埌璁剧疆浜?IPV6_ROUTER_ALERT socket 閫夐」
       鐨勫師濮嬪鎺ュ瓧
   - - `ptp_event`
     - `control`
     - 鎹曡幏 PTP 鏃堕棿鍏抽敭浜嬩欢娑堟伅锛圫ync銆丏elay_req銆丳delay_Req 涓?Pdelay_Resp锛?   - - `ptp_general`
     - `control`
     - 鎹曡幏 PTP 閫氱敤娑堟伅锛圓nnounce銆丗ollow_Up銆丏elay_Resp銆?       Pdelay_Resp_Follow_Up銆佺鐞嗕笌淇′护锛?   - - `flow_action_sample`
     - `control`
     - 鎹曡幏鍦ㄥ鐞嗘祦鍔ㄤ綔 sample锛堝閫氳繃 tc 鐨?sample 鍔ㄤ綔锛夋椂閲囨牱鐨勬暟鎹寘
   - - `flow_action_trap`
     - `control`
     - 鎹曡幏鍦ㄥ鐞嗘祦鍔ㄤ綔 trap锛堝閫氳繃 tc 鐨?trap 鍔ㄤ綔锛夋椂璁板綍鐨勬暟鎹寘
   - - `early_drop`
     - `drop`
     - 鎹曡幏鍥?RED锛堥殢鏈烘棭鏈熸娴嬶級绠楁硶锛堝嵆鏃╂湡涓㈠純锛夎€屼涪寮冪殑鏁版嵁鍖?   - - `vxlan_parsing`
     - `drop`
     - 鎹曡幏鍥?VXLAN 澶撮儴瑙ｆ瀽閿欒锛堝彲鑳芥槸鏁版嵁鍖呮埅鏂垨 I 鏍囧織鏈缃級鑰屼涪寮?       鐨勬暟鎹寘
   - - `llc_snap_parsing`
     - `drop`
     - 鎹曡幏鍥?LLC+SNAP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `vlan_parsing`
     - `drop`
     - 鎹曡幏鍥?VLAN 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傚彲鑳藉寘鎷剰澶栫殑鏁版嵁鍖呮埅鏂?   - - `pppoe_ppp_parsing`
     - `drop`
     - 鎹曡幏鍥?PPPoE+PPP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傚彲鑳藉寘鎷彂鐜颁細璇?ID 涓?       0xFFFF锛堜繚鐣欎笉鍙敤锛夈€丳PPoE 闀垮害澶т簬鎺ユ敹鍒扮殑甯э紝鎴栨绫诲ご閮ㄤ笂鐨勪换浣?       甯歌閿欒
   - - `mpls_parsing`
     - `drop`
     - 鎹曡幏鍥?MPLS 澶撮儴瑙ｆ瀽閿欒锛堝彲鑳藉寘鎷剰澶栫殑澶撮儴鎴柇锛夎€屼涪寮冪殑鏁版嵁鍖?   - - `arp_parsing`
     - `drop`
     - 鎹曡幏鍥?ARP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `ip_1_parsing`
     - `drop`
     - 鎹曡幏鍥犵涓€涓?IP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傝鏁版嵁鍖呴櫡闃卞彲鑳藉寘鎷湭
       閫氳繃 IP 鏍￠獙鍜屾鏌ャ€佸ご閮ㄩ暱搴︽鏌ワ紙鏈€灏?20 瀛楄妭锛夈€佸彲鑳藉洜鏁版嵁鍖呮埅鏂鑷?       鎬婚暱搴﹀瓧娈佃秴杩囨帴鏀跺寘闀垮害绛夌殑鏁版嵁鍖?   - - `ip_n_parsing`
     - `drop`
     - 鎹曡幏鍥犳渶鍚庝竴涓?IP 澶撮儴锛圛P over IP 闅ч亾鎯呭喌涓嬬殑鍐呭眰澶撮儴锛夎В鏋愰敊璇€?       涓㈠純鐨勬暟鎹寘銆傛澶勬墽琛屼笌 ip_1_parsing 闄烽槺鐩稿悓鐨勫父瑙侀敊璇鏌?   - - `gre_parsing`
     - `drop`
     - 鎹曡幏鍥?GRE 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `udp_parsing`
     - `drop`
     - 鎹曡幏鍥?UDP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傝鏁版嵁鍖呴櫡闃卞彲鑳藉寘鎷牎楠屽拰
       閿欒銆佹娴嬪埌涓嶅悎閫傜殑 UDP 闀垮害锛堝皬浜?8 瀛楄妭锛夋垨妫€娴嬪埌澶撮儴鎴柇
   - - `tcp_parsing`
     - `drop`
     - 鎹曡幏鍥?TCP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傚彲鑳藉寘鎷?TCP 鏍￠獙鍜岄敊璇€丼YN銆?       FIN 鍜?鎴?RESET 鐨勪笉褰撶粍鍚堢瓑
   - - `ipsec_parsing`
     - `drop`
     - 鎹曡幏鍥?IPSEC 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `sctp_parsing`
     - `drop`
     - 鎹曡幏鍥?SCTP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖呫€傝繖鎰忓懗鐫€浣跨敤浜嗙鍙ｅ彿 0 鎴?       澶撮儴琚埅鏂?   - - `dccp_parsing`
     - `drop`
     - 鎹曡幏鍥?DCCP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `gtp_parsing`
     - `drop`
     - 鎹曡幏鍥?GTP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `esp_parsing`
     - `drop`
     - 鎹曡幏鍥?ESP 澶撮儴瑙ｆ瀽閿欒鑰屼涪寮冪殑鏁版嵁鍖?   - - `blackhole_nexthop`
     - `drop`
     - 鎹曡幏璁惧鍥犲懡涓粦娲炰笅涓€璺宠€屽喅瀹氫涪寮冪殑鏁版嵁鍖?   - - `dmac_filter`
     - `drop`
     - 鎹曡幏璁惧鍥犵洰鐨?MAC 鏈湪 MAC 琛ㄤ腑閰嶇疆銆佷笖鎺ュ彛涓嶅浜庢贩鏉傛ā寮忚€屽喅瀹?       涓㈠純鐨勫叆绔欐暟鎹寘
   - - `eapol`
     - `control`
     - 鎹曡幏 IEEE 802.1X 涓瀹氱殑 "Extensible Authentication Protocol over LAN"
       锛圗APOL锛夋暟鎹寘
   - - `locked_port`
     - `drop`
     - 鎹曡幏璁惧鍥犳湭閫氳繃閿佸畾妗ョ鍙ｆ鏌ヨ€屽喅瀹氫涪寮冪殑鏁版嵁鍖呫€傚嵆锛岄€氳繃閿佸畾绔彛
       鎺ユ敹銆佷笖鍏?{SMAC, VID} 涓嶅搴斾簬鎸囧悜璇ョ鍙ｇ殑 FDB 鏉＄洰鐨勬暟鎹寘

## 椹卞姩鐗瑰畾鐨勬暟鎹寘闄烽槺


璁惧椹卞姩鍙互娉ㄥ唽椹卞姩鐗瑰畾鐨勬暟鎹寘闄烽槺锛屼絾蹇呴』娓呮鍦拌褰曘€傛绫婚櫡闃卞彲瀵瑰簲浜?璁惧鐗瑰畾鐨勫紓甯革紝骞舵湁鍔╀簬璋冭瘯鐢辫繖浜涘紓甯稿紩璧风殑涓㈠寘銆備互涓嬪垪琛ㄥ寘鍚寚鍚戝悇绉?璁惧椹卞姩娉ㄥ唽鐨勯┍鍔ㄧ壒瀹氶櫡闃辨弿杩扮殑閾炬帴锛?
  - Documentation/networking/devlink/netdevsim.rst
  - Documentation/networking/devlink/mlxsw.rst
  - Documentation/networking/devlink/prestera.rst


## 閫氱敤鏁版嵁鍖呴櫡闃辩粍


閫氱敤鏁版嵁鍖呴櫡闃辩粍鐢ㄤ簬鑱氬悎閫昏緫涓婄浉鍏崇殑鏁版嵁鍖呴櫡闃便€傝繖浜涚粍鍏佽鐢ㄦ埛鎵归噺鎿嶄綔锛?渚嬪璁剧疆鎵€鏈夋垚鍛橀櫡闃辩殑闄烽槺鍔ㄤ綔銆傛澶栵紝鍦ㄥ悇闄烽槺缁熻杩囦簬鐙獎鐨勬儏鍐典笅锛?`devlink-trap` 鍙互鎶ュ憡鎸夌粍鑱氬悎鐨勬暟鎹寘涓庡瓧鑺傜粺璁°€傝繖浜涚粍鐨勬弿杩板繀椤绘坊鍔?鍒颁笅琛ㄤ腑锛?
   :widths: 10 90

   - - Name
     - Description
   - - `l2_drops`
     - 鍖呭惈璁惧鍦ㄤ簩灞傝浆鍙戯紙鍗虫ˉ鎺ワ級鏈熼棿涓㈠純鐨勬暟鎹寘鐨勯櫡闃?   - - `l3_drops`
     - 鍖呭惈璁惧鍦ㄤ笁灞傝浆鍙戞湡闂翠涪寮冪殑鏁版嵁鍖呯殑闄烽槺
   - - `l3_exceptions`
     - 鍖呭惈璁惧鍦ㄤ笁灞傝浆鍙戞湡闂村懡涓紓甯革紙濡?TTL 閿欒锛夌殑鏁版嵁鍖呯殑闄烽槺
   - - `buffer_drops`
     - 鍖呭惈璁惧鍥犲叆闃熷喅绛栬€屼涪寮冪殑鏁版嵁鍖呯殑闄烽槺
   - - `tunnel_drops`
     - 鍖呭惈璁惧鍦ㄩ毀閬撳皝瑁?瑙ｅ皝瑁呮湡闂翠涪寮冪殑鏁版嵁鍖呯殑闄烽槺
   - - `acl_drops`
     - 鍖呭惈璁惧鍦?ACL 澶勭悊鏈熼棿涓㈠純鐨勬暟鎹寘鐨勯櫡闃?   - - `stp`
     - 鍖呭惈 STP 鏁版嵁鍖呯殑闄烽槺
   - - `lacp`
     - 鍖呭惈 LACP 鏁版嵁鍖呯殑闄烽槺
   - - `lldp`
     - 鍖呭惈 LLDP 鏁版嵁鍖呯殑闄烽槺
   - - `mc_snooping`
     - 鍖呭惈缁勬挱渚﹀惉鎵€闇€鐨?IGMP 涓?MLD 鏁版嵁鍖呯殑闄烽槺
   - - `dhcp`
     - 鍖呭惈 DHCP 鏁版嵁鍖呯殑闄烽槺
   - - `neigh_discovery`
     - 鍖呭惈閭诲眳鍙戠幇鏁版嵁鍖咃紙濡?ARP銆両Pv6 ND锛夌殑闄烽槺
   - - `bfd`
     - 鍖呭惈 BFD 鏁版嵁鍖呯殑闄烽槺
   - - `ospf`
     - 鍖呭惈 OSPF 鏁版嵁鍖呯殑闄烽槺
   - - `bgp`
     - 鍖呭惈 BGP 鏁版嵁鍖呯殑闄烽槺
   - - `vrrp`
     - 鍖呭惈 VRRP 鏁版嵁鍖呯殑闄烽槺
   - - `pim`
     - 鍖呭惈 PIM 鏁版嵁鍖呯殑闄烽槺
   - - `uc_loopback`
     - 鍖呭惈鍗曟挱鐜洖鏁版嵁鍖咃紙鍗?`uc_loopback`锛夌殑闄烽槺銆傚皢璇ラ櫡闃卞崟鐙垪鍑猴紝鏄?       鍥犱负鍦ㄨ濡傚崟鑷傝矾鐢卞櫒鐨勬儏鍐典笅瀹冧細鎸佺画瑙﹀彂銆備负闄愬埗瀵?CPU 浣跨敤鐜囩殑
       褰卞搷锛屽彲浠ュ皢浣庨€熺巼鐨勬暟鎹寘闄烽槺闄愰€熷櫒缁戝畾鍒拌缁勶紝鑰屼笉褰卞搷鍏朵粬闄烽槺
   - - `local_delivery`
     - 鍖呭惈璺敱鍚庡簲鏈湴鎶曢€掋€佷絾涓嶅尮閰嶆洿鍏蜂綋鐨勬暟鎹寘闄烽槺锛堝 `ipv4_bgp`锛夌殑
       鏁版嵁鍖呯殑闄烽槺
   - - `external_delivery`
     - 鍖呭惈搴旈€氳繃涓嶅睘浜庡悓涓€璁惧锛堝浜ゆ崲 ASIC锛夌殑澶栭儴鎺ュ彛锛堝绠＄悊鎺ュ彛锛夎繘琛?       璺敱鐨勬暟鎹寘鐨勯櫡闃?   - - `ipv6`
     - 鍖呭惈鍚勭 IPv6 鎺у埗鏁版嵁鍖咃紙濡傝矾鐢卞櫒閫氬憡锛夌殑闄烽槺
   - - `ptp_event`
     - 鍖呭惈 PTP 鏃堕棿鍏抽敭浜嬩欢娑堟伅锛圫ync銆丏elay_req銆丳delay_Req 涓?Pdelay_Resp锛?       鐨勯櫡闃?   - - `ptp_general`
     - 鍖呭惈 PTP 閫氱敤娑堟伅锛圓nnounce銆丗ollow_Up銆丏elay_Resp銆?       Pdelay_Resp_Follow_Up銆佺鐞嗕笌淇′护锛夌殑闄烽槺
   - - `acl_sample`
     - 鍖呭惈璁惧鍦?ACL 澶勭悊鏈熼棿閲囨牱鐨勬暟鎹寘鐨勯櫡闃?   - - `acl_trap`
     - 鍖呭惈璁惧鍦?ACL 澶勭悊鏈熼棿琚櫡闃辨崟鑾凤紙璁板綍锛夌殑鏁版嵁鍖呯殑闄烽槺
   - - `parser_error_drops`
     - 鍖呭惈璁惧鍦ㄨВ鏋愭湡闂存爣璁颁负閿欒鐨勫寘鐨勯櫡闃?   - - `eapol`
     - 鍖呭惈 IEEE 802.1X 涓瀹氱殑 "Extensible Authentication Protocol over LAN"
       锛圗APOL锛夋暟鎹寘鐨勯櫡闃?
## 鏁版嵁鍖呴櫡闃遍檺閫熷櫒


濡傚墠鎵€杩帮紝搴曞眰璁惧鍙互灏嗘煇浜涙暟鎹寘闄烽槺鎹曡幏鍒?CPU 杩涜澶勭悊銆傚湪澶у鏁版儏鍐?涓嬶紝搴曞眰璁惧鑳藉澶勭悊鐨勬暟鎹寘閫熺巼姣?CPU 鑳藉鐞嗙殑閫熺巼楂樺嚑涓暟閲忕骇銆?
鍥犳锛屼负浜嗛槻姝㈠簳灞傝澶囧帇鍨?CPU锛岃澶囬€氬父鍖呭惈鏁版嵁鍖呴櫡闃遍檺閫熷櫒锛岃兘澶熷皢闄烽槺
鎹曡幏鐨勬暟鎹寘闄愬埗鍒?CPU 鍙鐞嗙殑閫熺巼銆?
`devlink-trap` 鏈哄埗鍏佽鍏峰鑳藉姏鐨勮澶囬┍鍔ㄥ悜 `devlink` 娉ㄥ唽鍏舵敮鎸佺殑鏁版嵁鍖?闄烽槺闄愰€熷櫒銆傝澶囬┍鍔ㄥ彲浠ュ湪鍒濆鍖栨湡闂撮€夋嫨灏嗚繖浜涢檺閫熷櫒涓庡彈鏀寔鐨勬暟鎹寘闄烽槺
缁勶紙see Generic-Packet-Trap-Groups锛夊叧鑱旓紝浠庤€屽皢鍏堕粯璁ゆ帶鍒跺钩闈㈢瓥鐣ユ毚闇茬粰
鐢ㄦ埛绌洪棿銆?
璁惧椹卞姩搴旈€氳繃瀹炵幇鐩稿叧鍥炶皟鍑芥暟锛屽厑璁哥敤鎴风┖闂存洿鏀归檺閫熷櫒鐨勫弬鏁帮紙濡傞€熺巼銆?绐佸彂澶у皬锛変互鍙婇檺閫熷櫒涓庨櫡闃辩粍涔嬮棿鐨勫叧鑱斻€?
濡傛灉鍙兘锛岃澶囬┍鍔ㄥ簲瀹炵幇涓€涓洖璋冨嚱鏁帮紝鍏佽鐢ㄦ埛绌洪棿鑾峰彇鍥犺繚鍙嶉厤缃殑闄愰€?绛栫暐鑰岃闄愰€熷櫒涓㈠純鐨勬暟鎹寘鏁伴噺銆?
## 娴嬭瘯


鏈夊叧瑕嗙洊鏍稿績鍩虹璁炬柦鐨勬祴璇曪紝璇峰弬瑙?`tools/testing/selftests/drivers/net/netdevsim/devlink_trap.sh`銆傚簲涓轰换浣?鏂板姛鑳芥坊鍔犳祴璇曠敤渚嬨€?
璁惧椹卞姩搴斿皢鍏舵祴璇曢噸鐐规斁鍦ㄨ澶囩壒瀹氱殑鍔熻兘涓婏紝渚嬪鎵€鏀寔鐨勬暟鎹寘闄烽槺鐨?瑙﹀彂銆?