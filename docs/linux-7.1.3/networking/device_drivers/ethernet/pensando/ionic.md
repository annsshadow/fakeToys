## Pensando(R) 浠ュお缃戦€傞厤鍣ㄧ郴鍒?Linux 椹卞姩


Pensando Linux 浠ュお缃戦┍鍔ㄣ€?Copyright(c) 2019 Pensando Systems, Inc

## 鐩綍


- 璇嗗埆閫傞厤鍣?- 鍚敤椹卞姩
- 閰嶇疆椹卞姩
- 閫氳繃杈呭姪璁惧鐨?RDMA 鏀寔
- 缁熻淇℃伅
- 鏀寔

## 璇嗗埆閫傞厤鍣?

瑕佺‘瀹氱郴缁熶笂鏄惁瀹夎浜嗕竴涓垨澶氫釜 Pensando PCI 浠ュお缃戣澶囷紝鍙娇鐢?```

  $ lspci -d 1dd8:
  b5:00.0 Ethernet controller: Device 1dd8:1002
  b6:00.0 Ethernet controller: Device 1dd8:1002

```
濡傛灉鍒楀嚭浜嗗涓婃墍绀虹殑璁惧锛岄偅涔?`ionic.ko` 椹卞姩搴旇鑳芥壘鍒板苟閰嶇疆瀹冧滑浠ヤ緵浣跨敤銆傚唴鏍告棩蹇椾腑搴斿綋鏈夌浉鍏虫潯鐩?```

  $ dmesg | grep ionic
  ionic 0000:b5:00.0: 126.016 Gb/s available PCIe bandwidth (8.0 GT/s PCIe x16 link)
  ionic 0000:b5:00.0 enp181s0: renamed from eth0
  ionic 0000:b5:00.0 enp181s0: Link up - 100 Gbps
  ionic 0000:b6:00.0: 126.016 Gb/s available PCIe bandwidth (8.0 GT/s PCIe x16 link)
  ionic 0000:b6:00.0 enp182s0: renamed from eth0
  ionic 0000:b6:00.0 enp182s0: Link up - 100 Gbps

```
椹卞姩鍜屽浐浠剁増鏈俊鎭彲浠ラ€氳繃浠ヤ笅浠讳竴鍛戒护鑾峰彇
```

  $ ethtool -i enp181s0
  driver: ionic
  version: 5.7.0
  firmware-version: 1.8.0-28
  ...

  $ devlink dev info pci/0000:b5:00.0
  pci/0000:b5:00.0:
    driver ionic
    serial_number FLM18420073
    versions:
        fixed:
          asic.id 0x0
          asic.rev 0x0
        running:
          fw 1.8.0-28

```
鏈夊叧 devlink dev info 鏁版嵁鐨勬洿澶氫俊鎭紝璇峰弬闃?`Documentation/networking/devlink/ionic.rst`銆?
## 鍚敤椹卞姩


椹卞姩閫氳繃鏍囧噯鐨勫唴鏍搁厤缃郴缁熷惎鐢紝
```

  make oldconfig/menuconfig/etc.

```
璇ラ┍鍔ㄥ湪鑿滃崟缁撴瀯涓殑浣嶇疆涓猴細

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> Pensando devices
          -> Pensando Ethernet IONIC Support

## 閰嶇疆椹卞姩


### MTU


鏀寔宸ㄥ瀷甯э紙jumbo frame锛夛紝鏈€澶уぇ灏忎负 9194 瀛楄妭銆?
### 涓柇鑱氬悎锛圛nterrupt coalescing锛?

涓柇鑱氬悎鍙互閫氳繃浣跨敤 "ethtool -C" 鍛戒护鏇存敼 rx-usecs 鍊兼潵閰嶇疆銆俽x-usecs 鐨勫彇鍊艰寖鍥存槸 0-190銆倀x-usecs 鍊煎弽鏄犱簡 rx-usecs 鍊硷紝鍥犱负瀹冧滑缁戝畾鍦ㄥ悓涓€涓腑鏂笂銆?
### SR-IOV


鐩墠鎻愪緵鏈€鍩虹鐨?SR-IOV 鏀寔锛屽彲閫氳繃璁剧疆 sysfs 鐨?'sriov_numvfs' 鍊兼潵鍚敤锛堝鏋滀綘鐨勭壒瀹氬浐浠堕厤缃敮鎸侊級銆?
### XDP


瀵?XDP 鐨勬敮鎸佸寘鍚熀鏈姛鑳斤紝澶栧姞宸ㄥ瀷甯с€丷edirect 鍜?`ndo_xmit`銆傜洰鍓嶄笉鏀寔闆舵嫹璐濆鎺ュ瓧鎴栫‖浠跺嵏杞姐€?
## 閫氳繃杈呭姪璁惧鐨?RDMA 鏀寔


褰撳浐浠跺０鏄庢敮鎸佹椂锛宨onic 椹卞姩閫氳繃 Linux 杈呭姪璁惧妗嗘灦鏀寔 RDMA锛圧emote Direct Memory Access锛岃繙绋嬬洿鎺ュ唴瀛樿闂級鍔熻兘銆俁DMA 鑳藉姏鍦ㄨ澶囧垵濮嬪寲鏈熼棿琚娴嬪埌锛屽鏋滃彈鏀寔锛屼互澶綉椹卞姩灏嗗垱寤轰竴涓緟鍔╄澶囷紝鍏佽 RDMA 椹卞姩缁戝畾骞舵彁渚?InfiniBand/RoCE 鍔熻兘銆?
## 缁熻淇℃伅


### 鍩虹纭欢缁熻


鍛戒护 `netstat -i`銆乣ip -s link show` 鍜?`ifconfig` 鏄剧ず
```

  $ ip -s link show enp181s0
  7: enp181s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP mode DEFAULT group default qlen 1000
      link/ether 00:ae:cd:00:07:68 brd ff:ff:ff:ff:ff:ff
      RX: bytes  packets  errors  dropped overrun mcast
      414        5        0       0       0       0
      TX: bytes  packets  errors  dropped carrier collsns
      1384       18       0       0       0       0

```
### ethtool -S


`ethtool -S` 鍛戒护鏄剧ず鐨勭粺璁′俊鎭寘鍚┍鍔ㄨ鏁板櫒鍜屽浐浠惰鏁板櫒鐨勭粍鍚堬紝鍖呮嫭绔彛鍜岄槦鍒楃浉鍏崇殑鍏蜂綋鏁板€笺€傞┍鍔ㄦ暟鍊兼槸鐢遍┍鍔ㄨ绠楀緱鍒扮殑璁℃暟鍣紝鍥轰欢鏁板€肩敱鍥轰欢浠庣鍙ｇ‖浠堕噰闆嗗苟閫氳繃椹卞姩閫忎紶锛屼笉鍋氳繘涓€姝ヨВ閲娿€?
```

     tx_packets: 12
     tx_bytes: 964
     rx_packets: 5
     rx_bytes: 414
     tx_tso: 0
     tx_tso_bytes: 0
     tx_csum_none: 12
     tx_csum: 0
     rx_csum_none: 0
     rx_csum_complete: 3
     rx_csum_error: 0
     xdp_drop: 0
     xdp_aborted: 0
     xdp_pass: 0
     xdp_tx: 0
     xdp_redirect: 0
     xdp_frames: 0

```
```

     tx_0_pkts: 3
     tx_0_bytes: 294
     tx_0_clean: 3
     tx_0_dma_map_err: 0
     tx_0_linearize: 0
     tx_0_frags: 0
     tx_0_tso: 0
     tx_0_tso_bytes: 0
     tx_0_hwstamp_valid: 0
     tx_0_hwstamp_invalid: 0
     tx_0_csum_none: 3
     tx_0_csum: 0
     tx_0_vlan_inserted: 0
     tx_0_xdp_frames: 0
     rx_0_pkts: 2
     rx_0_bytes: 120
     rx_0_dma_map_err: 0
     rx_0_alloc_err: 0
     rx_0_csum_none: 0
     rx_0_csum_complete: 0
     rx_0_csum_error: 0
     rx_0_hwstamp_valid: 0
     rx_0_hwstamp_invalid: 0
     rx_0_dropped: 0
     rx_0_vlan_stripped: 0
     rx_0_xdp_drop: 0
     rx_0_xdp_aborted: 0
     rx_0_xdp_pass: 0
     rx_0_xdp_tx: 0
     rx_0_xdp_redirect: 0

```
```

     hw_tx_dropped: 0
     hw_rx_dropped: 0
     hw_rx_over_errors: 0
     hw_rx_missed_errors: 0
     hw_tx_aborted_errors: 0
     frames_rx_ok: 15
     frames_rx_all: 15
     frames_rx_bad_fcs: 0
     frames_rx_bad_all: 0
     octets_rx_ok: 1290
     octets_rx_all: 1290
     frames_rx_unicast: 10
     frames_rx_multicast: 5
     frames_rx_broadcast: 0
     frames_rx_pause: 0
     frames_rx_bad_length: 0
     frames_rx_undersized: 0
     frames_rx_oversized: 0
     frames_rx_fragments: 0
     frames_rx_jabber: 0
     frames_rx_pripause: 0
     frames_rx_stomped_crc: 0
     frames_rx_too_long: 0
     frames_rx_vlan_good: 3
     frames_rx_dropped: 0
     frames_rx_less_than_64b: 0
     frames_rx_64b: 4
     frames_rx_65b_127b: 11
     frames_rx_128b_255b: 0
     frames_rx_256b_511b: 0
     frames_rx_512b_1023b: 0
     frames_rx_1024b_1518b: 0
     frames_rx_1519b_2047b: 0
     frames_rx_2048b_4095b: 0
     frames_rx_4096b_8191b: 0
     frames_rx_8192b_9215b: 0
     frames_rx_other: 0
     frames_tx_ok: 31
     frames_tx_all: 31
     frames_tx_bad: 0
     octets_tx_ok: 2614
     octets_tx_total: 2614
     frames_tx_unicast: 8
     frames_tx_multicast: 21
     frames_tx_broadcast: 2
     frames_tx_pause: 0
     frames_tx_pripause: 0
     frames_tx_vlan: 0
     frames_tx_less_than_64b: 0
     frames_tx_64b: 4
     frames_tx_65b_127b: 27
     frames_tx_128b_255b: 0
     frames_tx_256b_511b: 0
     frames_tx_512b_1023b: 0
     frames_tx_1024b_1518b: 0
     frames_tx_1519b_2047b: 0
     frames_tx_2048b_4095b: 0
     frames_tx_4096b_8191b: 0
     frames_tx_8192b_9215b: 0
     frames_tx_other: 0
     frames_tx_pri_0: 0
     frames_tx_pri_1: 0
     frames_tx_pri_2: 0
     frames_tx_pri_3: 0
     frames_tx_pri_4: 0
     frames_tx_pri_5: 0
     frames_tx_pri_6: 0
     frames_tx_pri_7: 0
     frames_rx_pri_0: 0
     frames_rx_pri_1: 0
     frames_rx_pri_2: 0
     frames_rx_pri_3: 0
     frames_rx_pri_4: 0
     frames_rx_pri_5: 0
     frames_rx_pri_6: 0
     frames_rx_pri_7: 0
     tx_pripause_0_1us_count: 0
     tx_pripause_1_1us_count: 0
     tx_pripause_2_1us_count: 0
     tx_pripause_3_1us_count: 0
     tx_pripause_4_1us_count: 0
     tx_pripause_5_1us_count: 0
     tx_pripause_6_1us_count: 0
     tx_pripause_7_1us_count: 0
     rx_pripause_0_1us_count: 0
     rx_pripause_1_1us_count: 0
     rx_pripause_2_1us_count: 0
     rx_pripause_3_1us_count: 0
     rx_pripause_4_1us_count: 0
     rx_pripause_5_1us_count: 0
     rx_pripause_6_1us_count: 0
     rx_pripause_7_1us_count: 0
     rx_pause_1us_count: 0
     frames_tx_truncated: 0

```
## 鏀寔


鏈夊叧涓€鑸€х殑 Linux 缃戠粶鏀寔锛岃浣跨敤 netdev 閭欢鍒楄〃
```

  netdev@vger.kernel.org

```
濡傞渶鏇村叿浣撶殑鏀寔锛岃浣跨敤 Pensando 椹卞姩鏀寔閭
```

  drivers@pensando.io

```
