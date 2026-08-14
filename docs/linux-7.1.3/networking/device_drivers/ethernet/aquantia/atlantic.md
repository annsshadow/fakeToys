
## Marvell(Aquantia) AQtion 椹卞姩


閫傜敤浜?aQuantia 澶氬崈鍏嗕綅 PCI Express 绯诲垪浠ュお缃戦€傞厤鍣?

    - 璇嗗埆鎮ㄧ殑閫傞厤鍣?    - 閰嶇疆
    - 鍙楁敮鎸佺殑 ethtool 閫夐」
    - 鍛戒护琛屽弬鏁?    - 閰嶇疆鏂囦欢鍙傛暟
    - 鏀寔
    - 璁稿彲璇?
## 璇嗗埆鎮ㄧ殑閫傞厤鍣?

姝ょ増鏈┍鍔ㄤ笌鍩轰簬 AQC-100銆丄QC-107銆丄QC-108 鐨勪互澶綉閫傞厤鍣ㄥ吋瀹广€?

### SFP+ 璁惧锛堥€傜敤浜庡熀浜?AQC-100 鐨勯€傞厤鍣級


姝ょ増鏈娇鐢ㄦ棤婧愮洿杩為摐缂嗭紙DAC锛夊拰 SFP+/LC 鍏夋敹鍙戝櫒杩涜浜嗘祴璇曘€?
## 閰嶇疆


### 鏌ョ湅閾捐矾娑堟伅

  濡傛灉鍙戣鐗堥檺鍒朵簡绯荤粺娑堟伅锛岄摼璺秷鎭皢涓嶄細鏄剧ず鍒版帶鍒跺彴銆備负浜嗗湪鎺у埗鍙颁笂
  鐪嬪埌缃戠粶椹卞姩閾捐矾娑堟伅锛岃浣跨敤
```

       dmesg -n 8

  .. note::

     姝よ缃笉浼氬湪閲嶅惎鍚庝繚鐣欍€?
```
### Jumbo Frames锛堝法鍨嬪抚锛?
  璇ラ┍鍔ㄥ鎵€鏈夐€傞厤鍣ㄦ敮鎸?Jumbo Frames銆傞€氳繃灏?MTU 鏇存敼涓哄ぇ浜庨粯璁ゅ€?1500
  鐨勫€兼潵鍚敤 Jumbo Frames 鏀寔銆侻TU 鐨勬渶澶у€间负 16000銆備娇鐢?`ip` 鍛戒护锛?```

	ip link set mtu 16000 dev enp1s0

```
### ethtool

  椹卞姩鍒╃敤 ethtool 鎺ュ彛杩涜椹卞姩閰嶇疆鍜岃瘖鏂紝浠ュ強鏄剧ず缁熻淇℃伅銆傛鍔熻兘闇€瑕?  鏈€鏂扮増鏈殑 ethtool銆?
### NAPI

  atlantic 椹卞姩鏀寔 NAPI锛圧x 杞妯″紡锛夈€?
## 鍙楁敮鎸佺殑 ethtool 閫夐」


### 鏌ョ湅閫傞厤鍣ㄨ缃?

```

    ethtool <ethX>

 Output example::

  Settings for enp1s0:
    Supported ports: [ TP ]
    Supported link modes:   100baseT/Full
			    1000baseT/Full
			    10000baseT/Full
			    2500baseT/Full
			    5000baseT/Full
    Supported pause frame use: Symmetric
    Supports auto-negotiation: Yes
    Supported FEC modes: Not reported
    Advertised link modes:  100baseT/Full
			    1000baseT/Full
			    10000baseT/Full
			    2500baseT/Full
			    5000baseT/Full
    Advertised pause frame use: Symmetric
    Advertised auto-negotiation: Yes
    Advertised FEC modes: Not reported
    Speed: 10000Mb/s
    Duplex: Full
    Port: Twisted Pair
    PHYAD: 0
    Transceiver: internal
    Auto-negotiation: on
    MDI-X: Unknown
    Supports Wake-on: g
    Wake-on: d
    Link detected: yes


 .. note::

    AQrate 閫熺巼锛?.5/5 Gb/s锛変粎浼氬湪 linux 鍐呮牳 > 4.10 鏃舵樉绀恒€?    浣嗘偍浠嶅彲浣跨敤杩欎簺閫熺巼::

	ethtool -s eth0 autoneg off speed 2500

```
### 鏌ョ湅閫傞厤鍣ㄤ俊鎭?

```

  ethtool -i <ethX>

 Output example::

  driver: atlantic
  version: 5.2.0-050200rc5-generic-kern
  firmware-version: 3.1.78
  expansion-rom-version:
  bus-info: 0000:01:00.0
  supports-statistics: yes
  supports-test: no
  supports-eeprom-access: no
  supports-register-dump: yes
  supports-priv-flags: no


```
### 鏌ョ湅浠ュお缃戦€傞厤鍣ㄧ粺璁′俊鎭?

```

    ethtool -S <ethX>

 Output example::

  NIC statistics:
     InPackets: 13238607
     InUCast: 13293852
     InMCast: 52
     InBCast: 3
     InErrors: 0
     OutPackets: 23703019
     OutUCast: 23704941
     OutMCast: 67
     OutBCast: 11
     InUCastOctects: 213182760
     OutUCastOctects: 22698443
     InMCastOctects: 6600
     OutMCastOctects: 8776
     InBCastOctects: 192
     OutBCastOctects: 704
     InOctects: 2131839552
     OutOctects: 226938073
     InPacketsDma: 95532300
     OutPacketsDma: 59503397
     InOctetsDma: 1137102462
     OutOctetsDma: 2394339518
     InDroppedDma: 0
     Queue[0] InPackets: 23567131
     Queue[0] OutPackets: 20070028
     Queue[0] InJumboPackets: 0
     Queue[0] InLroPackets: 0
     Queue[0] InErrors: 0
     Queue[1] InPackets: 45428967
     Queue[1] OutPackets: 11306178
     Queue[1] InJumboPackets: 0
     Queue[1] InLroPackets: 0
     Queue[1] InErrors: 0
     Queue[2] InPackets: 3187011
     Queue[2] OutPackets: 13080381
     Queue[2] InJumboPackets: 0
     Queue[2] InLroPackets: 0
     Queue[2] InErrors: 0
     Queue[3] InPackets: 23349136
     Queue[3] OutPackets: 15046810
     Queue[3] InJumboPackets: 0
     Queue[3] InLroPackets: 0
     Queue[3] InErrors: 0

```
### 涓柇鍚堝苟鏀寔


```

    ethtool -c <ethX>

 and changed with::

    ethtool -C <ethX> tx-usecs <usecs> rx-usecs <usecs>

 To disable coalescing::

    ethtool -C <ethX> tx-usecs 0 rx-usecs 0 tx-max-frames 1 tx-max-frames 1

```
### Wake on LAN 鏀寔


```

    ethtool -s <ethX> wol g

 To disable WOL::

    ethtool -s <ethX> wol d

```
### 璁剧疆骞舵鏌ラ┍鍔ㄦ秷鎭骇鍒?

 璁剧疆娑堟伅绾у埆

```

    ethtool -s <ethX> msglvl <level>

 绾у埆鍊硷細

 ======   =============================
 0x0001   閫氱敤椹卞姩鐘舵€併€? 0x0002   纭欢鎺㈡祴銆? 0x0004   閾捐矾鐘舵€併€? 0x0008   鍛ㄦ湡鎬х姸鎬佹鏌ャ€? 0x0010   鎺ュ彛琚叧闂€? 0x0020   鎺ュ彛琚惎鐢ㄣ€? 0x0040   鎺ユ敹閿欒銆? 0x0080   鍙戦€侀敊璇€? 0x0200   涓柇澶勭悊銆? 0x0400   鍙戦€佸畬鎴愩€? 0x0800   鎺ユ敹瀹屾垚銆? 0x1000   鏁版嵁鍖呭唴瀹广€? 0x2000   纭欢鐘舵€併€? 0x4000   Wake-on-LAN 鐘舵€併€? ======   =============================

 榛樿鎯呭喌涓嬶紝璋冭瘯娑堟伅绾у埆璁句负 0x0001锛堥€氱敤椹卞姩鐘舵€侊級銆?
 妫€鏌ユ秷鎭骇鍒?
 ::

    ethtool <ethX> | grep "Current message level"

 濡傛灉鎮ㄦ兂绂佺敤娑堟伅杈撳嚭::

    ethtool -s <ethX> msglvl 0

```
### RX 娴佽鍒欙紙ntuple 杩囨护鍣級


 鏀寔浠ヤ笅鐙珛鐨勮鍒欙紝鎸夎椤哄簭搴旂敤锛?
 1. 16 鏉?VLAN ID 瑙勫垯
 2. 16 鏉?L2 EtherType 瑙勫垯
 3. 8 鏉?L3/L4 5 鍏冪粍瑙勫垯


 椹卞姩鍒╃敤 ethtool 鎺ュ彛閫氳繃 `ethtool -N <device> <filter>` 閰嶇疆 ntuple 杩囨护鍣ㄣ€?
```

    ethtool -K ethX ntuple <on|off>

 绂佺敤 ntuple 杩囨护鍣ㄦ椂锛屾墍鏈夌敤鎴风紪绋嬬殑杩囨护鍣ㄩ兘浼氫粠椹卞姩缂撳瓨鍜岀‖浠朵腑琚埛鏂般€? 閲嶆柊鍚敤 ntuple 鍚庡繀椤婚噸鏂版坊鍔犳墍鏈夐渶瑕佺殑杩囨护鍣ㄣ€?
 鐢变簬瑙勫垯鐨勫浐瀹氶『搴忥紝杩囨护鍣ㄧ殑浣嶇疆涔熸槸鍥哄畾鐨勶細

 - 浣嶇疆 0 - 15 鐢ㄤ簬 VLAN ID 杩囨护鍣? - 浣嶇疆 16 - 31 鐢ㄤ簬 L2 EtherType 杩囨护鍣? - 浣嶇疆 32 - 39 鐢ㄤ簬 L3/L4 5 鍏冪粍杩囨护鍣紙浣嶇疆 32銆?6 鐢ㄤ簬 IPv6锛?
 L3/L4 5 鍏冪粍锛堝崗璁€佹簮鍜岀洰鐨?IP 鍦板潃銆佹簮鍜岀洰鐨?TCP/UDP/SCTP 绔彛锛変笌 8 涓? 杩囨护鍣ㄨ繘琛屾瘮杈冦€傚浜?IPv4锛屾渶澶氬彲鍖归厤 8 涓簮鍦板潃鍜岀洰鐨勫湴鍧€銆傚浜?IPv6锛? 鏈€澶氭敮鎸?2 瀵瑰湴鍧€銆傛簮绔彛鍜岀洰鐨勭鍙ｄ粎瀵?TCP/UDP/SCTP 鏁版嵁鍖呰繘琛屾瘮杈冦€?
 瑕佹坊鍔犱竴鏉″皢鏁版嵁鍖呭鍚戦槦鍒?5 鐨勮繃婊ゅ櫒锛屼娇鐢? ``<-N|-U|--config-nfc|--config-ntuple>`` 寮€鍏?:

    ethtool -N <ethX> flow-type udp4 src-ip 10.0.0.1 dst-ip 10.0.0.2 src-port 2000 dst-port 2001 action 5 <loc 32>

 - action 涓洪槦鍒楀彿銆? - loc 涓鸿鍒欏彿銆?
 瀵逛簬 ``flow-type ip4|udp4|tcp4|sctp4|ip6|udp6|tcp6|sctp6``锛屽繀椤诲皢 loc 缂栧彿
 璁惧湪 32 - 39 涔嬮棿銆? 瀵逛簬 ``flow-type ip4|udp4|tcp4|sctp4|ip6|udp6|tcp6|sctp6``锛屾偍鍙互涓?IPv4
 娴侀噺璁剧疆 8 鏉¤鍒欙紝鎴栦负 IPv6 娴侀噺璁剧疆 2 鏉¤鍒欍€侷Pv6 娴侀噺鐨?loc 缂栧彿涓?32
 鍜?36銆? 鐩墠鎮ㄤ笉鑳藉悓鏃朵娇鐢?IPv4 鍜?IPv6 杩囨护鍣ㄣ€?
 IPv6 杩囨护娴侀噺鐨勭ず渚?:

    sudo ethtool -N <ethX> flow-type tcp6 src-ip 2001:db8:0:f101::1 dst-ip 2001:db8:0:f101::2 action 1 loc 32
    sudo ethtool -N <ethX> flow-type ip6 src-ip 2001:db8:0:f101::2 dst-ip 2001:db8:0:f101::5 action -1 loc 36

 IPv4 杩囨护娴侀噺鐨勭ず渚?:

    sudo ethtool -N <ethX> flow-type udp4 src-ip 10.0.0.4 dst-ip 10.0.0.7 src-port 2000 dst-port 2001 loc 32
    sudo ethtool -N <ethX> flow-type tcp4 src-ip 10.0.0.3 dst-ip 10.0.0.9 src-port 2000 dst-port 2001 loc 33
    sudo ethtool -N <ethX> flow-type ip4 src-ip 10.0.0.6 dst-ip 10.0.0.4 loc 34

 濡傛灉璁剧疆 action -1锛屽垯鎵€鏈夊尮閰嶈杩囨护鍣ㄧ殑娴侀噺閮戒細琚涪寮冦€?
 action 鐨勬渶澶у€间负 31銆?

 VLAN 杩囨护鍣紙VLAN id锛変笌 16 涓繃婊ゅ櫒杩涜姣旇緝銆? VLAN id 蹇呴』浼撮殢鎺╃爜 0xF000銆傝繖鏄负灏?VLAN 杩囨护鍣ㄤ笌甯︽湁 UserPriority 鐨? L2 EtherType 杩囨护鍣ㄥ尯鍒嗗紑锛屽洜涓?User Priority 鍜?VLAN ID 閮介€氳繃鍚屼竴涓? 'vlan' 鍙傛暟浼犲叆銆?
 瑕佹坊鍔犱竴鏉″皢鏉ヨ嚜 VLAN 2001 鐨勬暟鎹寘瀵煎悜闃熷垪 5 鐨勮繃婊ゅ櫒::

    ethtool -N <ethX> flow-type ip4 vlan 2001 m 0xF000 action 1 loc 0


 L2 EtherType 杩囨护鍣ㄥ厑璁告寜 EtherType 瀛楁锛屾垨鍚屾椂鎸?802.1Q 鐨?EtherType 鍜? User Priority锛圥CP锛夊瓧娈佃繃婊ゆ暟鎹寘銆? UserPriority锛坴lan锛夊弬鏁板繀椤讳即闅忔帺鐮?0x1FFF銆傝繖鏄负灏?VLAN 杩囨护鍣ㄤ笌甯︽湁
 UserPriority 鐨?L2 Ethertype 杩囨护鍣ㄥ尯鍒嗗紑锛屽洜涓?User Priority 鍜?VLAN ID
 閮介€氳繃鍚屼竴涓?'vlan' 鍙傛暟浼犲叆銆?
 瑕佹坊鍔犱竴鏉″皢浼樺厛绾?3 鐨?IP4 鏁版嵁鍖呭鍚戦槦鍒?3 鐨勮繃婊ゅ櫒::

    ethtool -N <ethX> flow-type ether proto 0x800 vlan 0x600 m 0x1FFF action 3 loc 16

 瑕佹煡鐪嬪綋鍓嶅瓨鍦ㄧ殑杩囨护鍣ㄥ垪琛?:

    ethtool <-u|-n|--show-nfc|--show-ntuple> <ethX>

 瑙勫垯鍙互浠庤〃鏈韩鍒犻櫎銆備娇鐢ㄥ涓嬪懡浠ゅ畬鎴?:

    sudo ethtool <-N|-U|--config-nfc|--config-ntuple> <ethX> delete <loc>

 - loc 涓鸿鍒犻櫎鐨勮鍒欏彿銆?
 Rx 杩囨护鍣ㄦ槸涓€涓皢杩囨护琛ㄥ姞杞界殑鎺ュ彛锛岄櫎闈炰娇鐢?鈥渁ction鈥?鎸囧畾鏇夸唬闃熷垪锛屽惁鍒? 瀹冨皢鎵€鏈夋祦姹囧叆闃熷垪 0銆傚湪杩欑鎯呭喌涓嬶紝浠讳綍鍖归厤杩囨护鍣ㄦ潯浠剁殑娴侀兘浼氳瀵煎悜鐩稿簲鐨? 闃熷垪銆俁X 杩囨护鍣ㄥ湪鎵€鏈?2.6.30 鍙婁互鍚庣増鏈殑鍐呮牳涓婂彈鏀寔銆?
```
### UDP 鐨?RSS

 鐩墠锛孨IC 涓嶆敮鎸佸鍒嗙墖 IP 鏁版嵁鍖呯殑 RSS锛岃繖浼氬鑷村鍒嗙墖 UDP 娴侀噺鐨?RSS
 宸ヤ綔涓嶆纭€傝绂佺敤 UDP 鐨?RSS锛屽彲浠ヤ娇鐢?RX Flow L3/L4 瑙勫垯銆?
```

    ethtool -N eth0 flow-type udp4 action 0 loc 32

```
### UDP GSO 纭欢鍗歌浇

 UDP GSO 閫氳繃灏?UDP 澶撮儴鍒嗛厤鍗歌浇鍒扮‖浠讹紝鏉ユ彁鍗?UDP 鍙戦€侀€熺巼銆備负姝ら渶瑕佺壒娈婄殑
 鐢ㄦ埛绌洪棿 socket 閫夐」锛?```

    udpgso_bench_tx -u -4 -D 10.0.1.1 -s 6300 -S 100

 灏嗗鑷翠粠鍗曚釜 6300 瀛楄妭鐨勭敤鎴风紦鍐插尯鍙戝嚭 100 瀛楄妭澶у皬鐨?UDP 鏁版嵁鍖呫€?
 UDP GSO 閫氳繃濡備笅鏂瑰紡閰嶇疆::

    ethtool -K eth0 tx-udp-segmentation on

```
### 绉佹湁鏍囧織锛堟祴璇曠敤锛?

```

	$ ethtool --show-priv-flags ethX

	Private flags for ethX:
	DMASystemLoopback  : off
	PKTSystemLoopback  : off
	DMANetworkLoopback : off
	PHYInternalLoopback: off
	PHYExternalLoopback: off

 Example::

	$ ethtool --set-priv-flags ethX DMASystemLoopback on

 DMASystemLoopback:   DMA 涓绘満鍥炵幆銆? PKTSystemLoopback:   鏁版嵁鍖呯紦鍐插尯涓绘満鍥炵幆銆? DMANetworkLoopback:  DMA 鍧椾笂鐨勭綉缁滀晶鍥炵幆銆? PHYInternalLoopback: Phy 涓婄殑鍐呴儴鍥炵幆銆? PHYExternalLoopback: Phy 涓婄殑澶栭儴鍥炵幆锛堜娇鐢ㄥ洖鐜互澶綉绾跨紗锛夈€?

```
## 鍛戒护琛屽弬鏁?
atlantic 椹卞姩鎻愪緵浠ヤ笅鍛戒护琛屽弬鏁帮細

### aq_itr - 涓柇鑺傛祦妯″紡

鍙帴鍙楀€硷細0, 1, 0xFFFF

榛樿鍊硷細0xFFFF

======   ==============================================================
0        绂佺敤涓柇鑺傛祦銆?1        鍚敤涓柇鑺傛祦骞朵娇鐢ㄦ寚瀹氱殑 tx 鍜?rx 閫熺巼銆?0xFFFF   鑷姩鑺傛祦妯″紡銆傞┍鍔ㄥ皢鏍规嵁閾捐矾閫熺巼閫夋嫨鏈€浣崇殑 RX 鍜?TX
	 涓柇鑺傛祦璁剧疆銆?======   ==============================================================

### aq_itr_tx - TX 涓柇鑺傛祦閫熺巼


鍙帴鍙楀€硷細0 - 0x1FF

榛樿鍊硷細0

浠ュ井绉掕鐨?TX 渚ц妭娴併€傞€傞厤鍣ㄤ細灏嗘渶澶т腑鏂欢杩熻缃负姝ゅ€笺€傛渶灏忎腑鏂欢杩熶负
姝ゅ€肩殑涓€鍗娿€?
### aq_itr_rx - RX 涓柇鑺傛祦閫熺巼


鍙帴鍙楀€硷細0 - 0x1FF

榛樿鍊硷細0

浠ュ井绉掕鐨?RX 渚ц妭娴併€傞€傞厤鍣ㄤ細灏嗘渶澶т腑鏂欢杩熻缃负姝ゅ€笺€傛渶灏忎腑鏂欢杩熶负
姝ゅ€肩殑涓€鍗娿€?

   ITR 璁剧疆鍙湪杩愯鏃堕€氳繃 ethtool -c 鏂瑰紡鏇存敼锛堣涓嬫枃锛?
## 閰嶇疆鏂囦欢鍙傛暟


涓轰簡涓€浜涘井璋冧笌鎬ц兘浼樺寲锛屾煇浜涘弬鏁板彲浠ュ湪 {source_dir}/aq_cfg.h 鏂囦欢涓洿鏀广€?
### AQ_CFG_RX_PAGEORDER


榛樿鍊硷細0

RX 椤甸樁瑕嗙洊銆傝繖鏄负姣忎釜鎻忚堪绗﹀垎閰嶇殑 RX 椤垫暟閲忕殑 2 鐨勫箓娆°€傛帴鏀舵弿杩扮澶у皬
浠嶅彈 AQ_CFG_RX_FRAME_MAX 闄愬埗銆?
澧炲ぇ椤甸樁鍙互鏀瑰杽椤靛鐢紙鍦ㄥ惎鐢?iommu 鐨勭郴缁熶笂灏や负鏄庢樉锛夈€?
### AQ_CFG_RX_REFILL_THRES


榛樿鍊硷細32

RX 濉厖闃堝€笺€俁X 璺緞鍦ㄨ瀵熷埌鎸囧畾鏁伴噺鐨勭┖闂叉弿杩扮涔嬪墠涓嶄細濉厖宸查噴鏀剧殑鎻忚堪绗︺€?杈冨ぇ鐨勫€煎彲鑳芥湁鍔╀簬鏇村ソ鍦板鐢ㄩ〉锛屼絾涔熷彲鑳藉鑷翠涪鍖呫€?
### AQ_CFG_VECS_DEF


闃熷垪鏁伴噺

鏈夋晥鑼冨洿锛? - 8锛堟渶澶у埌 AQ_CFG_VECS_MAX锛?
榛樿鍊硷細8

娉ㄦ剰姝ゅ€间細琚郴缁熶腑鍙敤鐨勬牳蹇冩暟鎵€闄愬埗銆?
### AQ_CFG_IS_RSS_DEF


鍚敤/绂佺敤 Receive Side Scaling锛堟帴鏀剁缂╂斁锛?
姝ょ壒鎬у厑璁搁€傞厤鍣ㄥ皢鎺ユ敹澶勭悊鍒嗗竷鍒板涓?CPU 鏍稿績涓婏紝浠ラ槻姝㈠崟涓?CPU 鏍稿績杩囪浇銆?
鏈夋晥鍊?
==  ========
0   绂佺敤
1   鍚敤
==  ========

榛樿鍊硷細1

### AQ_CFG_NUM_RSS_QUEUES_DEF


Receive Side Scaling 鐨勯槦鍒楁暟閲?
鏈夋晥鑼冨洿锛? - 8锛堟渶澶у埌 AQ_CFG_VECS_DEF锛?
榛樿鍊硷細AQ_CFG_VECS_DEF

### AQ_CFG_IS_LRO_DEF


鍚敤/绂佺敤 Large Receive Offload锛堝ぇ鍨嬫帴鏀跺嵏杞斤級

姝ゅ嵏杞戒娇閫傞厤鍣ㄨ兘澶熷皢澶氫釜 TCP 娈靛悎骞讹紝骞跺皢鍏朵綔涓哄崟涓悎骞跺崟鍏冩寚绀虹粰鎿嶄綔绯荤粺
缃戠粶瀛愮郴缁熴€?
绯荤粺娑堣€楁洿灏戠殑鑳介噺锛屼絾涔熷紩鍏ユ洿澶氱殑鏁版嵁鍖呭鐞嗗欢杩熴€?
鏈夋晥鍊?
==  ========
0   绂佺敤
1   鍚敤
==  ========

榛樿鍊硷細1

### AQ_CFG_TX_CLEAN_BUDGET


鍗曟 TX 娓呯悊鐨勬渶澶ф弿杩扮鏁伴噺銆?
榛樿鍊硷細256

淇敼 aq_cfg.h 鏂囦欢鍚庯紝蹇呴』閲嶆柊鏋勫缓椹卞姩鎵嶈兘鐢熸晥銆?
## 鏀寔


濡傛灉鍦ㄥ彈鏀寔鐨勫唴鏍镐笂浣跨敤鍙楁敮鎸佺殑閫傞厤鍣紝鍙戠幇宸插彂甯冩簮浠ｇ爜瀛樺湪闂锛岃灏?涓庤闂鐩稿叧鐨勫叿浣撲俊鎭€氳繃鐢靛瓙閭欢鍙戦€佽嚦 aqn_support@marvell.com

## 璁稿彲璇?

aQuantia Corporation 缃戠粶椹卞姩

Copyright |copy| 2014 - 2019 aQuantia Corporation.

鏈▼搴忔槸鑷敱杞欢锛涙偍鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇佺 2 鐗?鐨勬潯娆惧拰鏉′欢涓嬮噸鏂板垎鍙戝拰/鎴栦慨鏀瑰畠銆?