
## 鐢ㄤ簬 Intel(R) 浠ュお缃戝涓绘満鎺у埗鍣紙Ethernet Multi-host Controller锛夌殑 Linux 鍩虹椹卞姩

2018 骞?8 鏈?20 鏃?Copyright(c) 2015-2018 Intel Corporation.

## 鐩綍

- 璇嗗埆浣犵殑閫傞厤鍣?- 棰濆閰嶇疆
- 鎬ц兘璋冧紭
- 宸茬煡闂
- 鏀寔

## 璇嗗埆浣犵殑閫傞厤鍣?
鏈彂琛岀増涓殑椹卞姩鍏煎鍩轰簬 Intel(R) 浠ュお缃戝涓绘満鎺у埗鍣ㄧ殑璁惧銆?
鍏充簬濡備綍璇嗗埆浣犵殑閫傞厤鍣紝浠ュ強鑾峰彇鏈€鏂扮殑 Intel 缃戠粶椹卞姩锛岃鍙傞槄 Intel 鏀寔缃戠珯锛?https://www.intel.com/support

### 娴佹帶鍒讹紙Flow Control锛?
Intel(R) 浠ュお缃戜氦鎹富鏈烘帴鍙ｏ紙Ethernet Switch Host Interface锛夐┍鍔ㄤ笉鏀寔娴佹帶鍒躲€?瀹冧笉浼氬彂閫佹殏鍋滐紙pause锛夊抚銆傝繖鍙兘瀵艰嚧涓㈠抚銆?
### 铏氭嫙鍔熻兘锛圴irtual Functions锛孷Fs锛?
浣跨敤 sysfs 鏉ュ惎鐢?VF銆?鏈夋晥鑼冨洿锛?-64

```

    echo $num_vf_enabled > /sys/class/net/$dev/device/sriov_numvfs //鍚敤 VFs
    echo 0 > /sys/class/net/$dev/device/sriov_numvfs //绂佺敤 VFs

```
娉ㄦ剰锛氳澶囧拰椹卞姩閮戒笉鎺у埗 VF 濡備綍鏄犲皠鍒伴厤缃┖闂淬€傛€荤嚎甯冨眬浼氬洜鎿嶄綔绯荤粺鑰屽紓銆傚湪鏀寔
鐨勬搷浣滅郴缁熶笂锛屼綘鍙互妫€鏌?sysfs 鏉ユ煡鎵炬槧灏勫叧绯汇€?
娉ㄦ剰锛氬綋 SR-IOV 妯″紡鍚敤鏃讹紝纭欢 VLAN 杩囨护浠ュ強 VLAN 鏍囩鍓ョ/鎻掑叆灏嗕繚鎸佸惎鐢ㄣ€傝
绉婚櫎鏃х殑 VLAN 杩囨护鍣?```

    ip link set eth0 vf 0 vlan 100	// 涓?VF 0 璁剧疆 vlan 100
    ip link set eth0 vf 0 vlan 0	// 鍒犻櫎 vlan 100
    ip link set eth0 vf 0 vlan 200	// 涓?VF 0 璁剧疆涓€涓柊鐨?vlan 200


```
## 棰濆鍔熻兘涓庨厤缃?
### 宸ㄥ抚锛圝umbo Frames锛?
閫氳繃鎶婃渶澶т紶杈撳崟鍏冿紙MTU锛夋敼涓哄ぇ浜庨粯璁ゅ€?1500 鐨勫€兼潵鍚敤宸ㄥ抚鏀寔銆?
浣跨敤 ifconfig 鍛戒护鏉ュ澶?MTU 澶у皬銆備緥濡傦紝杈撳叆
```

    ifconfig eth<x> mtu 9000 up

```
```

    ip link set mtu 9000 dev eth<x>
    ip link set up dev eth<x>

```
姝よ缃笉浼氬湪閲嶅惎鍚庝繚鐣欍€傚彲浠ラ€氳繃鍦ㄤ互涓嬫枃浠朵腑娣诲姞 'MTU=9000' 浣胯缃案涔呯敓鏁堬細

- 瀵逛簬 RHEL锛?etc/sysconfig/network-scripts/ifcfg-eth<x>
- 瀵逛簬 SLES锛?etc/sysconfig/network/<config_file>

娉ㄦ剰锛氬法甯х殑鏈€澶?MTU 璁剧疆涓?15342銆傝鍊间笌 15364 瀛楄妭鐨勬渶澶у法甯уぇ灏忎竴鑷淬€?
娉ㄦ剰锛氳椹卞姩浼氬皾璇曚娇鐢ㄥ涓〉澶у皬鐨勭紦鍐插尯鏉ユ帴鏀舵瘡涓法甯ф暟鎹寘銆傝繖鏈夊姪浜庡湪鍒嗛厤鎺ユ敹
鏁版嵁鍖呮椂閬垮厤缂撳啿鍖鸿€楀敖闂銆?
### 閫氱敤鎺ユ敹鍗歌浇锛圙eneric Receive Offload锛屽嵆 GRO锛?
璇ラ┍鍔ㄦ敮鎸佸唴鏍稿唴鐨?GRO 杞欢瀹炵幇銆侴RO 琛ㄦ槑锛岄€氳繃灏?Rx 娴侀噺鍚堝苟涓烘洿澶х殑鏁版嵁鍧楋紝鍦ㄥぇ
Rx 璐熻浇涓嬪彲浠ユ樉钁楅檷浣?CPU 浣跨敤鐜囥€侴RO 鏄箣鍓嶄娇鐢ㄧ殑 LRO 鎺ュ彛鐨勬紨杩涖€侴RO 鑳藉鍚堝苟
闄や簡 TCP 涔嬪鐨勫叾瀹冨崗璁€傚畠涔熷彲浠ュ湪涓?LRO 鏈夐棶棰樼殑閰嶇疆锛堝嵆妗ユ帴鍜?iSCSI锛変腑瀹夊叏浣跨敤銆?
### 鐢ㄤ簬杩囨护鐨勫彈鏀寔 ethtool 鍛戒护涓庨€夐」

-n --show-nfc
  鑾峰彇鎺ユ敹缃戠粶娴佸垎绫伙紙receive network flow classification锛夐厤缃€?
rx-flow-hash tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6
  鑾峰彇鎸囧畾缃戠粶娴侀噺绫诲瀷鐨勫搱甯岄€夐」銆?
-N --config-nfc
  閰嶇疆鎺ユ敹缃戠粶娴佸垎绫汇€?
rx-flow-hash tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6 m|v|t|s|d|f|n|r
  閰嶇疆鎸囧畾缃戠粶娴侀噺绫诲瀷鐨勫搱甯岄€夐」銆?
- udp4锛氬熀浜?IPv4 鐨?UDP
- udp6锛氬熀浜?IPv6 鐨?UDP
- f 鍩轰簬鎺ユ敹鏁版嵁鍖呯 4 灞傦紙Layer 4锛夊ご鐨勭 0 鍜?1 瀛楄妭杩涜鍝堝笇銆?- n 鍩轰簬鎺ユ敹鏁版嵁鍖呯 4 灞傜殑绗?2 鍜?3 瀛楄妭杩涜鍝堝笇銆?
## 宸茬煡闂/鏁呴殰鎺掓煡

### 鍦?Linux KVM 涓嬬殑 64 浣?Microsoft Windows Server 2012/R2 瀹㈡埛鏈烘搷浣滅郴缁熶腑鍚敤 SR-IOV

KVM Hypervisor/VMM 鏀寔灏?PCIe 璁惧鐩存帴鍒嗛厤缁?VM銆傝繖鍖呮嫭浼犵粺鐨?PCIe 璁惧锛屼互鍙婂熀浜?Intel Ethernet Controller XL710 鐨勫叿澶?SR-IOV 鑳藉姏鐨勮澶囥€?
## 鏀寔

鏈夊叧涓€鑸俊鎭紝璇疯闂?Intel 鏀寔缃戠珯锛?https://www.intel.com/support/

濡傛灉鍦ㄥ彈鏀寔鐨勫唴鏍镐笂浣跨敤鍙楁敮鎸佺殑閫傞厤鍣ㄥ彂鐜颁簡宸插彂甯冩簮浠ｇ爜涓殑闂锛岃灏嗕笌璇ラ棶棰樼浉鍏崇殑
鍏蜂綋淇℃伅鍙戦€佽嚦 intel-wired-lan@lists.osuosl.org銆?