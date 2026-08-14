
## Virtual Routing and Forwarding锛堣櫄鎷熻矾鐢变笌杞彂锛孷RF锛?


## The VRF Device锛圴RF 璁惧锛?


VRF 璁惧閰嶅悎 ip 瑙勫垯锛屽彲浠ュ湪 Linux 缃戠粶鏍堜腑鍒涘缓铏氭嫙璺敱涓庤浆鍙戝煙
锛堝叿浣撳嵆 VRF銆乂RF-lite锛夈€備竴涓吀鍨嬬殑浣跨敤鍦烘櫙鏄绉熸埛闂锛氭瘡涓鎴?
閮芥嫢鏈夊悇鑷嫭绔嬬殑璺敱琛紝骞朵笖鑷冲皯闇€瑕佷笉鍚岀殑榛樿缃戝叧銆?

杩涚▼鍙互閫氳繃灏嗗鎺ュ瓧缁戝畾鍒?VRF 璁惧锛屼粠鑰屽仛鍒扳€淰RF 鎰熺煡鈥濄€傞€氳繃璇ュ鎺ュ瓧
鏀跺彂鐨勬姤鏂囬殢鍚庝細浣跨敤涓?VRF 璁惧鐩稿叧鑱旂殑璺敱琛ㄣ€俈RF 璁惧瀹炵幇鐨勪竴涓噸瑕?
鐗规€ф槸瀹冨彧褰卞搷绗?3 灞傚強浠ヤ笂锛屽洜姝?L2 宸ュ叿锛堜緥濡?LLDP锛変笉浼氬彈鍒板奖鍝?
锛堝嵆鏃犻渶鍦ㄦ瘡涓?VRF 涓垎鍒繍琛岋級銆傝璁捐杩樺厑璁镐娇鐢ㄦ洿楂樹紭鍏堢骇鐨?ip 瑙勫垯
锛堝熀浜庣瓥鐣ョ殑璺敱锛孭BR锛夋潵浼樺厛浜?VRF 璁惧瑙勫垯锛屼粠鑰屾寜闇€寮曞鐗瑰畾娴侀噺銆?

姝ゅ锛孷RF 璁惧鏀寔灏?VRF 宓屽鍦ㄥ懡鍚嶇┖闂村唴銆備緥濡傦紝缃戠粶鍛藉悕绌洪棿鍦ㄨ澶囧眰
鎻愪緵缃戠粶鎺ュ彛鐨勯殧绂伙紝鍛藉悕绌洪棿鍐呮帴鍙ｄ笂鐨?VLAN 鎻愪緵 L2 闅旂锛岃€?VRF 璁惧
鍒欐彁渚?L3 闅旂銆?

### Design锛堣璁★級


VRF 璁惧鍒涘缓鏃朵細鍏宠仈涓€寮犺矾鐢辫〃銆傜綉缁滄帴鍙ｈ浠庡睘浜庢煇涓?VRF 璁惧鍚庯紝VRF
灏卞埄鐢ㄥ畠鏉ュ紩瀵煎叆鍚戜笌鍑哄悜鎶ユ枃锛?

```
	 +-----------------------------+
	 |           vrf-blue          |  ===> route table 10
	 +-----------------------------+
	    |        |            |
	 +------+ +------+     +-------------+
	 | eth1 | | eth2 | ... |    bond1    |
	 +------+ +------+     +-------------+
				  |       |
			      +------+ +------+
			      | eth8 | | eth9 |
			      +------+ +------+

```
鍦ㄨ浠庡睘浜?VRF 璁惧鐨勬帴鍙ｄ笂鏀跺埌鐨勬姤鏂囷紝浼氬湪 IPv4 涓?IPv6 鍗忚鏍堜腑琚垏鎹㈠埌
VRF 璁惧锛屼粠鑰岀粰浜轰竴绉嶆姤鏂囨祦缁?VRF 璁惧鐨勫嵃璞°€傜被浼煎湴锛屽湪鍑虹珯鏂瑰悜涓婏紝璺敱
瑙勫垯浼氬湪鎶ユ枃鐪熸鍙戝嚭鍓嶅皢鍏堕€佸線 VRF 璁惧椹卞姩銆傝繖浣垮緱鍦?VRF 璁惧涓婁娇鐢?
tcpdump 鍗冲彲鎹曡幏杩涘嚭鏁翠釜 VRF 鐨勬墍鏈夋姤鏂嘰 [^1^]_銆傚悓鏍峰湴锛屽彲浠ュ埄鐢?VRF 璁惧
搴旂敤 netfilter\ [^2^]_ 涓?tc 瑙勫垯锛屼粠鑰屾寚瀹氶€傜敤浜庢暣涓?VRF 鍩熺殑瑙勫垯銆?

       娉ㄦ剰锛歵cpdump 鐩墠鐪嬩笉鍒拌繖浜涙姤鏂囥€傝闄愬埗灏嗗湪鏈潵鐨勭増鏈腑浜堜互瑙ｅ喅銆?

       瀵逛簬鍏ュ悜锛孖NPUT 涓?PREROUTING 瑙勫垯鐨?skb->dev 琚涓?VRF 璁惧锛?
       瀵逛簬鍑哄悜锛孭OSTROUTING 涓?OUTPUT 瑙勫垯鍙互浣跨敤 VRF 璁惧鎴栫湡瀹炵殑
       鍑哄悜璁惧杩涜缂栧啓銆?

### Setup锛堥厤缃級


1. 鍒涘缓 VRF 璁惧锛屽苟鍏宠仈涓€寮?FIB 琛細

```
	ip link add vrf-blue type vrf table 10
	ip link set dev vrf-blue up

```
2. 涓€鏉?l3mdev FIB 瑙勫垯灏嗘煡鎵惧紩瀵煎埌涓庤璁惧鐩稿叧鑱旂殑琛ㄣ€傚崟涓?l3mdev 瑙勫垯
   瓒充互鏈嶅姟鎵€鏈?VRF銆傚綋棣栦釜璁惧鍒涘缓鏃讹紝VRF 璁惧浼氫负 IPv4 涓?IPv6 娣诲姞
   l3mdev 瑙勫垯锛岄粯璁や紭鍏堢骇涓?1000銆傜敤鎴峰闇€鍙垹闄よ瑙勫垯锛屽苟浠ヤ笉鍚屼紭鍏堢骇
   閲嶆柊娣诲姞锛屾垨瀹夎鎸?VRF 鍒掑垎鐨勮鍒欍€?

```
       ip ru add oif vrf-blue table 10
       ip ru add iif vrf-blue table 10

```
3. 涓?VRF 娣诲姞 IPv4 涓?IPv6 榛樿璺敱銆備緥濡傦紝浣跨敤榛樿涓嶅彲杈捐矾鐢变綔涓哄厹搴曪紝
   纭繚浠讳綍璺敱鍗忚閮借兘瑕嗙洊瀹冿細

```
       ip route add table 10 unreachable default metric 4278198272

```
璇ヨ緝楂樼殑 metric 鍊肩‘淇濋粯璁や笉鍙揪璺敱鍙璺敱鍗忚濂椾欢瑕嗙洊銆侳RRouting 灏?
鍐呮牳 metric 瑙ｉ噴涓虹粍鍚堢殑绠＄悊璺濈锛堥珮瀛楄妭锛変笌浼樺厛绾э紙浣?3 瀛楄妭锛夛紝鍥犳
涓婅堪 metric 绛変环浜?[255/8192]銆?

4. 灏嗙綉缁滄帴鍙ｄ粠灞炰簬 VRF 璁惧锛?

```
       ip link set dev eth1 master vrf-blue

```
浠庡睘浜?VRF 璁惧鐨勬湰鍦颁笌鐩磋繛璺敱浼氳嚜鍔ㄧЩ鍔ㄥ埌涓庤 VRF 璁惧鐩稿叧鑱旂殑琛ㄣ€?
浠讳綍渚濊禆浜庤浠庡睘璁惧鐨勯澶栬矾鐢变細琚涪寮冿紝闇€瑕佸湪浠庡睘鍏崇郴寤虹珛鍚庨噸鏂版彃鍏?
鍒?VRF FIB 琛ㄣ€?

5. IPv6 sysctl 閫夐」 keep_addr_on_down 鍙紑鍚紝浠ュ湪 VRF 浠庡睘鍏崇郴鍙樺寲鏃?
   淇濈暀 IPv6 鍏ㄥ眬鍦板潃锛?

```
       sysctl -w net.ipv6.conf.all.keep_addr_on_down=1

```
6. 鍚?VRF 琛ㄦ坊鍔犺矾鐢憋細

```
       ip route add table 10 ...

```
### Applications锛堝簲鐢級


瑕佸湪 VRF 鍐呭伐浣滅殑搴旂敤闇€瑕佸皢鍏跺鎺ュ瓧缁戝畾鍒?VRF 璁惧锛屽彲浣跨敤 setsockopt锛?

```
    setsockopt(sd, SOL_SOCKET, SO_BINDTODEVICE, dev, strlen(dev)+1);

```
鎴栦娇鐢?cmsg 涓?IP_PKTINFO 鏉ユ寚瀹氳緭鍑鸿澶囥€?

榛樿鎯呭喌涓嬶紝鏈粦瀹氬鎺ュ瓧鐨勭鍙ｇ粦瀹氳寖鍥翠粎闄愪簬榛樿 VRF銆備篃灏辨槸璇达紝鍒拌揪
浠庡睘浜?l3mdev 鐨勬帴鍙ｇ殑鎶ユ枃涓嶄細琚叾鍖归厤锛涜€岃繘绋嬭嫢缁戝畾鍒版煇涓?l3mdev锛屽垯
鍙互缁戝畾鍒板悓涓€绔彛銆?

杩愯鍦ㄩ粯璁?VRF 涓婁笅鏂囷紙鍗虫湭缁戝畾鍒颁换浣?VRF 璁惧锛変腑鐨?TCP 涓?UDP 鏈嶅姟锛?
鍙€氳繃寮€鍚互涓嬮€夐」鍦ㄦ墍鏈?VRF 鍩熶腑宸ヤ綔锛?

```
    sysctl -w net.ipv4.tcp_l3mdev_accept=1
    sysctl -w net.ipv4.udp_l3mdev_accept=1

```
杩欎簺閫夐」榛樿鍏抽棴锛屼娇寰?VRF 涓殑濂楁帴瀛楀彧琚€変腑澶勭悊璇?VRF 鍐呯殑鎶ユ枃銆俁AW
濂楁帴瀛楁湁绫讳技閫夐」锛屽嚭浜庡悜鍚庡吋瀹归粯璁ゅ紑鍚紝浠ヤ究鐢?cmsg 涓?IP_PKTINFO 鎸囧畾
杈撳嚭璁惧锛屼絾浣跨敤鐨勬槸鏈粦瀹氬埌瀵瑰簲 VRF 鐨勫鎺ュ瓧銆備緥濡傦紝杩欏厑璁歌€佸紡 ping
瀹炵幇鍦ㄦ寚瀹氳澶囩殑鎯呭喌涓嬭繍琛岋紝鑰屾棤闇€鍦?VRF 涓墽琛屻€傝閫夐」鍙叧闂紝浠庤€屼娇
鍦?VRF 涓婁笅鏂囦腑鏀跺埌鐨勬姤鏂囧彧鐢辩粦瀹氬埌 VRF 鐨?raw 濂楁帴瀛楀鐞嗭細

```
    sysctl -w net.ipv4.raw_l3mdev_accept=0

```
VRF 璁惧涓婄殑 netfilter 瑙勫垯涔熷彲鐢ㄤ簬闄愬埗瀵硅繍琛屽湪榛樿 VRF 涓婁笅鏂囦腑鐨?
鏈嶅姟鐨勮闂€?

浣跨敤 VRF 鎰熺煡搴旂敤锛堝嵆鍚屾椂鍒涘缓 VRF 鍐呭濂楁帴瀛楃殑搴旂敤锛夐厤鍚?
`net.ipv4.tcp_l3mdev_accept=1` 鏄彲琛岀殑锛屼絾鍦ㄦ煇浜涙儏鍐典笅鍙兘瀵艰嚧闂銆?
鍦ㄨ sysctl 鍙栧€间笅锛岀敱鍝釜鐩戝惉濂楁帴瀛楁潵澶勭悊 VRF 娴侀噺杩炴帴鏄笉纭畾鐨勶紱
涔熷氨鏄锛屾棦鍙兘浣跨敤缁戝畾鍒?VRF 鐨勫鎺ュ瓧锛屼篃鍙兘浣跨敤鏈粦瀹氱殑濂楁帴瀛楁潵
鎺ュ彈鏉ヨ嚜 VRF 鐨勬柊杩炴帴銆傚鏋滀负濂楁帴瀛楅厤缃簡棰濆閫夐」锛堜緥濡?TCP MD5 瀵嗛挜锛夛紝
骞舵湡鏈?VRF 娴侀噺鍙敱缁戝畾鍒?VRF 鐨勫鎺ュ瓧澶勭悊锛堝嵆 `net.ipv4.tcp_l3mdev_accept=0`
鐨勬儏褰級锛岃繖绉嶇暐鏄炬剰澶栫殑琛屼负灏卞彲鑳藉紩鍙戦棶棰樸€傛渶鍚庢彁閱掞紝鏃犺閫変腑鍝釜鐩戝惉
濂楁帴瀛楋紝宸插缓绔嬬殑濂楁帴瀛楅兘浼氬熀浜庡叆鍚戞帴鍙ｅ垱寤哄湪瀵瑰簲鐨?VRF 涓紝濡傚墠鏂囨墍杩般€?

--------------------------------------------------------------------------------

## Using iproute2 for VRFs锛堜娇鐢?iproute2 绠＄悊 VRF锛?


iproute2 鑷?v4.7 璧锋敮鎸?vrf 鍏抽敭瀛椼€傚嚭浜庡悜鍚庡吋瀹癸紝鏈妭鍦ㄥ悎閫傚鍒楀嚭涓ょ
鍛戒护鈥斺€斿甫 vrf 鍏抽敭瀛楃殑褰㈠紡涓庝笉甯﹀畠鐨勬棫寮忓啓娉曘€?

1. Create a VRF锛堝垱寤?VRF锛?

鍒涘缓 VRF 璁惧锛?

```
       $ ip link add dev NAME type vrf table ID

```
鑷?v4.8 璧凤紝鍐呮牳鏀寔 l3mdev FIB 瑙勫垯锛屽崟鏉¤鍒欏嵆鍙鐩栨墍鏈?VRF銆傝
l3mdev 瑙勫垯鍦ㄩ涓澶囧垱寤烘椂涓?IPv4 涓?IPv6 寤虹珛銆?

2. List VRFs锛堝垪鍑?VRF锛?

鍒楀嚭鎵€鏈?VRF 璁惧锛?

```
       $ ip [-d] link show type vrf
	 NOTE: 闇€瑕?-d 閫夐」鎵嶈兘鏄剧ず琛?id

```
渚嬪锛?

```
       $ ip -d link show type vrf
       11: mgmt: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether 72:b3:ba:91:e2:24 brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 1 addrgenmode eui64
       12: red: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether b6:6f:6e:f6:da:73 brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 10 addrgenmode eui64
       13: blue: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether 36:62:e8:7d:bb:8c brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 66 addrgenmode eui64
       14: green: <NOARP,MASTER,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000
	   link/ether e6:28:b8:63:70:bb brd ff:ff:ff:ff:ff:ff promiscuity 0
	   vrf table 81 addrgenmode eui64

```
鎴栦互绠€鐣ヨ緭鍑猴細

```
       $ ip -br link show type vrf
       mgmt         UP             72:b3:ba:91:e2:24 <NOARP,MASTER,UP,LOWER_UP>
       red          UP             b6:6f:6e:f6:da:73 <NOARP,MASTER,UP,LOWER_UP>
       blue         UP             36:62:e8:7d:bb:8c <NOARP,MASTER,UP,LOWER_UP>
       green        UP             e6:28:b8:63:70:bb <NOARP,MASTER,UP,LOWER_UP>

```
3. Assign a Network Interface to a VRF锛堝皢缃戠粶鎺ュ彛鍒嗛厤缁?VRF锛?

缃戠粶鎺ュ彛閫氳繃灏嗙綉缁滆澶囦粠灞炰簬 VRF 鏉ュ垎閰嶇粰 VRF锛?

```
       $ ip link set dev NAME master NAME

```
浠庡睘鏃讹紝鐩磋繛涓庢湰鍦拌矾鐢变細鑷姩绉诲姩鍒颁笌 VRF 璁惧鐩稿叧鑱旂殑琛ㄣ€?

渚嬪锛?

```
       $ ip link set dev eth0 master mgmt

```
4. Show Devices Assigned to a VRF锛堟樉绀哄垎閰嶇粰 VRF 鐨勮澶囷級

瑕佹樉绀哄凡鍒嗛厤缁欑壒瀹?VRF 鐨勮澶囷紝鍙湪 show 鍛戒护涓姞鍏?master 鍙傛暟锛?

```
       $ ip link show vrf NAME
       $ ip link show master NAME

```
渚嬪锛?

```
       $ ip link show vrf red
       3: eth1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:02 brd ff:ff:ff:ff:ff:ff
       4: eth2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:03 brd ff:ff:ff:ff:ff:ff
       7: eth5: <BROADCAST,MULTICAST> mtu 1500 qdisc noop master red state DOWN mode DEFAULT group default qlen 1000
	   link/ether 02:00:00:00:02:06 brd ff:ff:ff:ff:ff:ff

```
鎴栦娇鐢ㄧ畝鐣ヨ緭鍑猴細

```
       $ ip -br link show vrf red
       eth1             UP             02:00:00:00:02:02 <BROADCAST,MULTICAST,UP,LOWER_UP>
       eth2             UP             02:00:00:00:02:03 <BROADCAST,MULTICAST,UP,LOWER_UP>
       eth5             DOWN           02:00:00:00:02:06 <BROADCAST,MULTICAST>

```
5. Show Neighbor Entries for a VRF锛堟樉绀?VRF 鐨勯偦灞呰〃椤癸級

瑕佸垪鍑轰粠灞炰簬 VRF 璁惧鐨勮澶囩浉鍏宠仈鐨勯偦灞呰〃椤癸紝鍙娇鐢細

```
       $ ip [-6] neigh show vrf NAME
       $ ip [-6] neigh show master NAME

```
渚嬪锛?

```
       $  ip neigh show vrf red
       10.2.1.254 dev eth1 lladdr a6:d9:c7:4f:06:23 REACHABLE
       10.2.2.254 dev eth2 lladdr 5e:54:01:6a:ee:80 REACHABLE

       $ ip -6 neigh show vrf red
       2002:1::64 dev eth1 lladdr a6:d9:c7:4f:06:23 REACHABLE

```
6. Show Addresses for a VRF锛堟樉绀?VRF 鐨勫湴鍧€锛?

瑕佹樉绀轰笌 VRF 鐩稿叧鑱旂殑鎺ュ彛鍦板潃锛屽彲鍦?show 鍛戒护涓姞鍏?master 鍙傛暟锛?

```
       $ ip addr show vrf NAME
       $ ip addr show master NAME

```
渚嬪锛?

```
	$ ip addr show vrf red
	3: eth1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP group default qlen 1000
	    link/ether 02:00:00:00:02:02 brd ff:ff:ff:ff:ff:ff
	    inet 10.2.1.2/24 brd 10.2.1.255 scope global eth1
	       valid_lft forever preferred_lft forever
	    inet6 2002:1::2/120 scope global
	       valid_lft forever preferred_lft forever
	    inet6 fe80::ff:fe00:202/64 scope link
	       valid_lft forever preferred_lft forever
	4: eth2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast master red state UP group default qlen 1000
	    link/ether 02:00:00:00:02:03 brd ff:ff:ff:ff:ff:ff
	    inet 10.2.2.2/24 brd 10.2.2.255 scope global eth2
	       valid_lft forever preferred_lft forever
	    inet6 2002:2::2/120 scope global
	       valid_lft forever preferred_lft forever
	    inet6 fe80::ff:fe00:203/64 scope link
	       valid_lft forever preferred_lft forever
	7: eth5: <BROADCAST,MULTICAST> mtu 1500 qdisc noop master red state DOWN group default qlen 1000
	    link/ether 02:00:00:00:02:06 brd ff:ff:ff:ff:ff:ff

```
鎴栦互绠€鐣ユ牸寮忥細

```
	$ ip -br addr show vrf red
	eth1             UP             10.2.1.2/24 2002:1::2/120 fe80::ff:fe00:202/64
	eth2             UP             10.2.2.2/24 2002:2::2/120 fe80::ff:fe00:203/64
	eth5             DOWN

```
7. Show Routes for a VRF锛堟樉绀?VRF 鐨勮矾鐢憋級

瑕佹樉绀?VRF 鐨勮矾鐢憋紝浣跨敤 ip 鍛戒护鏄剧ず涓?VRF 鐩稿叧鑱旂殑琛細

```
       $ ip [-6] route show vrf NAME
       $ ip [-6] route show table ID

```
渚嬪锛?

```
	$ ip route show vrf red
	unreachable default  metric 4278198272
	broadcast 10.2.1.0 dev eth1  proto kernel  scope link  src 10.2.1.2
	10.2.1.0/24 dev eth1  proto kernel  scope link  src 10.2.1.2
	local 10.2.1.2 dev eth1  proto kernel  scope host  src 10.2.1.2
	broadcast 10.2.1.255 dev eth1  proto kernel  scope link  src 10.2.1.2
	broadcast 10.2.2.0 dev eth2  proto kernel  scope link  src 10.2.2.2
	10.2.2.0/24 dev eth2  proto kernel  scope link  src 10.2.2.2
	local 10.2.2.2 dev eth2  proto kernel  scope host  src 10.2.2.2
	broadcast 10.2.2.255 dev eth2  proto kernel  scope link  src 10.2.2.2

	$ ip -6 route show vrf red
	local 2002:1:: dev lo  proto none  metric 0  pref medium
	local 2002:1::2 dev lo  proto none  metric 0  pref medium
	2002:1::/120 dev eth1  proto kernel  metric 256  pref medium
	local 2002:2:: dev lo  proto none  metric 0  pref medium
	local 2002:2::2 dev lo  proto none  metric 0  pref medium
	2002:2::/120 dev eth2  proto kernel  metric 256  pref medium
	local fe80:: dev lo  proto none  metric 0  pref medium
	local fe80:: dev lo  proto none  metric 0  pref medium
	local fe80::ff:fe00:202 dev lo  proto none  metric 0  pref medium
	local fe80::ff:fe00:203 dev lo  proto none  metric 0  pref medium
	fe80::/64 dev eth1  proto kernel  metric 256  pref medium
	fe80::/64 dev eth2  proto kernel  metric 256  pref medium
	ff00::/8 dev red  metric 256  pref medium
	ff00::/8 dev eth1  metric 256  pref medium
	ff00::/8 dev eth2  metric 256  pref medium
	unreachable default dev lo  metric 4278198272  error -101 pref medium

```
8. Route Lookup for a VRF锛堟煡璇?VRF 鐨勮矾鐢憋級

鏌ヨ鏌愬湴鍧€鍦?VRF 涓殑璺敱锛?

```
       $ ip [-6] route get vrf NAME ADDRESS
       $ ip [-6] route get oif NAME ADDRESS

```
渚嬪锛?

```
	$ ip route get 10.2.1.40 vrf red
	10.2.1.40 dev eth1  table red  src 10.2.1.2
	    cache

	$ ip -6 route get 2002:1::32 vrf red
	2002:1::32 from :: dev eth1  table red  proto kernel  src 2002:1::2  metric 256  pref medium

```
9. Removing Network Interface from a VRF锛堜粠 VRF 绉婚櫎缃戠粶鎺ュ彛锛?

缃戠粶鎺ュ彛閫氳繃瑙ｉ櫎瀵?VRF 璁惧鐨勪粠灞炲叧绯绘潵浠?VRF 涓Щ闄わ細

```
       $ ip link set dev NAME nomaster

```
鐩磋繛璺敱浼氳绉诲洖榛樿琛紝鏈湴琛ㄩ」浼氳绉诲姩鍒版湰鍦拌〃銆?

渚嬪锛?

```
    $ ip link set dev eth0 nomaster

```
--------------------------------------------------------------------------------

浠ヤ笅鏄湪 /etc/iproute2/rt_tables.d/vrf.conf 涓畾涔夎嚜瀹氫箟琛紝骞剁敤鑴氭湰
鎵归噺鍒涘缓 VRF 鐨勭ず渚嬶細

```
     cat >> /etc/iproute2/rt_tables.d/vrf.conf <<EOF
     1  mgmt
     10 red
     66 blue
     81 green
     EOF

     function vrf_create
     {
	 VRF=$1
	 TBID=$2

	 # create VRF device
	 ip link add ${VRF} type vrf table ${TBID}

	 if [ "${VRF}" != "mgmt" ]; then
	     ip route add table ${TBID} unreachable default metric 4278198272
	 fi
	 ip link set dev ${VRF} up
     }

     vrf_create mgmt 1
     ip link set dev eth0 master mgmt

     vrf_create red 10
     ip link set dev eth1 master red
     ip link set dev eth2 master red
     ip link set dev eth5 master red

     vrf_create blue 66
     ip link set dev eth3 master blue

     vrf_create green 81
     ip link set dev eth4 master green


     Interface addresses from /etc/network/interfaces:
     auto eth0
     iface eth0 inet static
	   address 10.0.0.2
	   netmask 255.255.255.0
	   gateway 10.0.0.254

     iface eth0 inet6 static
	   address 2000:1::2
	   netmask 120

     auto eth1
     iface eth1 inet static
	   address 10.2.1.2
	   netmask 255.255.255.0

     iface eth1 inet6 static
	   address 2002:1::2
	   netmask 120

     auto eth2
     iface eth2 inet static
	   address 10.2.2.2
	   netmask 255.255.255.0

     iface eth2 inet6 static
	   address 2002:2::2
	   netmask 120

     auto eth3
     iface eth3 inet static
	   address 10.2.3.2
	   netmask 255.255.255.0

     iface eth3 inet6 static
	   address 2002:3::2
	   netmask 120

     auto eth4
     iface eth4 inet static
	   address 10.2.4.2
	   netmask 255.255.255.0

     iface eth4 inet6 static
	   address 2002:4::2
	   netmask 120

```
