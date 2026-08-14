
## prestera devlink 鏀寔


鏈枃妗ｆ弿杩扮敱 `prestera` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 椹卞姩鐗瑰畾鐨?Traps


   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `arp_bc`
     - `trap`
     - 鎹曡幏 ARP 骞挎挱鍖咃紙璇锋眰涓庡搷搴旓級
   - - `is_is`
     - `trap`
     - 鎹曡幏 IS-IS 鍖?   - - `ospf`
     - `trap`
     - 鎹曡幏 OSPF 鍖?   - - `ip_bc_mac`
     - `trap`
     - 鎹曡幏鐩殑 MAC 鍦板潃涓哄箍鎾湴鍧€鐨?IPv4 鍖?   - - `stp`
     - `trap`
     - 鎹曡幏 STP BPDU
   - - `lacp`
     - `trap`
     - 鎹曡幏 LACP 鍖?   - - `lldp`
     - `trap`
     - 鎹曡幏 LLDP 鍖?   - - `router_mc`
     - `trap`
     - 鎹曡幏缁勬挱鍖?   - - `vrrp`
     - `trap`
     - 鎹曡幏 VRRP 鍖?   - - `dhcp`
     - `trap`
     - 鎹曡幏 DHCP 鍖?   - - `mtu_error`
     - `trap`
     - 鎹曡幏瓒呭嚭绔彛 MTU 鐨勶紙寮傚父锛夊寘
   - - `mac_to_me`
     - `trap`
     - 鎹曡幏鐩殑 MAC 鍦板潃涓轰氦鎹㈢鍙ｅ湴鍧€鐨勫寘
   - - `ttl_error`
     - `trap`
     - 鎹曡幏 TTL 瓒呮椂鐨勶紙寮傚父锛塈Pv4 鍖?   - - `ipv4_options`
     - `trap`
     - 鍥?IPv4 澶撮€夐」鏍煎紡閿欒鑰屾崟鑾风殑锛堝紓甯革級鍖?   - - `ip_default_route`
     - `trap`
     - 鎹曡幏娌℃湁鐗瑰畾 IP 鎺ュ彛锛圛P to me锛変篃娌℃湁杞彂鍓嶇紑鐨勫寘
   - - `local_route`
     - `trap`
     - 鎹曡幏鍙戝線鏌愪釜浜ゆ崲 IP 鎺ュ彛鍦板潃鐨勫寘
   - - `ipv4_icmp_redirect`
     - `trap`
     - 鎹曡幏锛堝紓甯革級IPv4 ICMP 閲嶅畾鍚戝寘
   - - `arp_response`
     - `trap`
     - 鎹曡幏鐩殑 MAC 鍦板潃涓轰氦鎹㈢鍙ｅ湴鍧€鐨?ARP 搴旂瓟鍖?   - - `acl_code_0`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 0锛坱c pref 0锛夌殑鍖?   - - `acl_code_1`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 1锛坱c pref 1锛夌殑鍖?   - - `acl_code_2`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 2锛坱c pref 2锛夌殑鍖?   - - `acl_code_3`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 3锛坱c pref 3锛夌殑鍖?   - - `acl_code_4`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 4锛坱c pref 4锛夌殑鍖?   - - `acl_code_5`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 5锛坱c pref 5锛夌殑鍖?   - - `acl_code_6`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 6锛坱c pref 6锛夌殑鍖?   - - `acl_code_7`
     - `trap`
     - 鎹曡幏 ACL 浼樺厛绾т负 7锛坱c pref 7锛夌殑鍖?   - - `ipv4_bgp`
     - `trap`
     - 鎹曡幏 IPv4 BGP 鍖?   - - `ssh`
     - `trap`
     - 鎹曡幏 SSH 鍖?   - - `telnet`
     - `trap`
     - 鎹曡幏 Telnet 鍖?   - - `icmp`
     - `trap`
     - 鎹曡幏 ICMP 鍖?   - - `rxdma_drop`
     - `drop`
     - 鍥犵己灏戝叆鍙ｇ紦鍐插尯绛夎€屼涪寮冨寘锛圧xDMA锛?   - - `port_no_vlan`
     - `drop`
     - 鍥犵綉缁滈厤缃敊璇垨鍐呴儴 bug锛堥厤缃棶棰橈級鑰屼涪寮冨寘
   - - `local_port`
     - `drop`
     - 涓㈠純鍐崇瓥锛團DB 琛ㄩ」锛変负灏嗗寘妗ユ帴鍥炲叆鍙ｇ鍙?trunk 鐨勫寘
   - - `invalid_sa`
     - `drop`
     - 涓㈠純婧?MAC 鍦板潃涓虹粍鎾殑鍖?   - - `illegal_ip_addr`
     - `drop`
     - 涓㈠純婧?鐩殑 IP 涓洪潪娉曠粍鎾?鍗曟挱鍦板潃鐨勫寘
   - - `illegal_ipv4_hdr`
     - `drop`
     - 涓㈠純 IPv4 澶撮潪娉曠殑鍖?   - - `ip_uc_dip_da_mismatch`
     - `drop`
     - 涓㈠純鐩殑 MAC 涓哄崟鎾€佷絾鐩殑 IP 涓虹粍鎾殑鍖?   - - `ip_sip_is_zero`
     - `drop`
     - 涓㈠純 IPv4 婧愬湴鍧€涓?0 鐨勫寘
   - - `met_red`
     - `drop`
     - 涓㈠純涓嶅悎瑙勭殑鍖咃紙琚叆鍙ｉ檺閫熷櫒涓㈠純锛岃閲忎涪寮冿級锛屼緥濡傚寘閫熺巼瓒呭嚭閰嶇疆鐨勫甫瀹?