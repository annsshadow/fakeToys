
## 鎺ュ彛缁熻


## 姒傝堪


鏈枃妗ｆ槸 Linux 缃戠粶鎺ュ彛缁熻鐨勬寚鍗椼€?
Linux 涓湁涓変釜涓昏鐨勬帴鍙ｇ粺璁℃潵婧愶細

 - 鍩轰簬 `struct rtnl_link_stats64 <rtnl_link_stats64>` 鐨勬爣鍑嗘帴鍙ｇ粺璁★紱
 - 鍗忚鐗瑰畾鐨勭粺璁★紱浠ュ強
 - 閫氳繃 ethtool 鍙敤鐨勯┍鍔ㄥ畾涔夌粺璁°€?
### 鏍囧噯鎺ュ彛缁熻


鏈夊绉嶆帴鍙ｅ彲浠ヨ闂爣鍑嗙粺璁°€?```

  $ ip -s -s link show dev ens4u1u1
  6: ens4u1u1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 48:2a:e3:4c:b1:d1 brd ff:ff:ff:ff:ff:ff
    RX: bytes  packets  errors  dropped overrun mcast
    74327665117 69016965 0       0       0       0
    RX errors: length   crc     frame   fifo    missed
               0        0       0       0       0
    TX: bytes  packets  errors  dropped carrier collsns
    21405556176 44608960 0       0       0       0
    TX errors: aborted  fifo   window heartbeat transns
               0        0       0       0       128
    altname enp58s0u1u1

```
娉ㄦ剰锛宍-s` 琚寚瀹氫簡涓ゆ锛屼互鏌ョ湅 `struct rtnl_link_stats64 <rtnl_link_stats64>`
鐨勬墍鏈夋垚鍛樸€傚鏋滃彧鎸囧畾涓€娆?`-s`锛屽垯涓嶄細鏄剧ず璇︾粏鐨勯敊璇€?
`ip` 鏀寔閫氳繃 `-j` 閫夐」杩涜 JSON 鏍煎紡鍖栥€?
#### 闃熷垪缁熻


闃熷垪缁熻鍙互閫氳繃 netdev netlink 绯诲垪璁块棶銆?
鐩墠娌℃湁骞挎硾鍒嗗彂鐨?CLI 鏉ヨ闂繖浜涚粺璁°€傚唴鏍稿紑鍙戝伐鍏凤紙ynl锛夊彲鐢ㄤ簬璇曢獙瀹冧滑锛屽弬瑙?`Documentation/userspace-api/netlink/intro-specs.rst`銆?
### 鍗忚鐗瑰畾鐨勭粺璁?

鍗忚鐗瑰畾鐨勭粺璁￠€氳繃鐩稿叧鎺ュ彛鏆撮湶锛岃繖浜涙帴鍙ｄ笌鐢ㄤ簬閰嶇疆瀹冧滑鐨勬帴鍙ｇ浉鍚屻€?
#### ethtool


Ethtool 鏆撮湶甯歌鐨勫簳灞傜粺璁°€傛墍鏈夋爣鍑嗙粺璁￠兘棰勬湡鐢辫澶囷紙鑰岄潪椹卞姩锛夌淮鎶わ紙涓庝笅涓€鑺?鎻忚堪鐨勯┍鍔ㄥ畾涔夌粺璁′笉鍚岋紝鍚庤€呮贩鍚堜簡杞欢鍜岀‖浠剁粺璁★級銆傚浜庡寘鍚潪鎵樼浜ゆ崲鏈猴紙渚嬪
浼犵粺 SR-IOV 鎴栧涓绘満 NIC锛夌殑璁惧锛屾墍璁℃暟鐨勪簨浠跺彲鑳藉苟闈炰笓闂ㄥ搴斾簬鍙戝線鏈湴涓绘満
鎺ュ彛鐨勬暟鎹寘銆傛崲鍙ヨ瘽璇达紝浜嬩欢鍙兘鍦ㄧ綉缁滅鍙ｏ紙MAC/PHY 妯″潡锛夊琚鏁帮紝鑰屼笉鍖哄垎
涓嶅悓鐨勪富鏈轰晶锛圥CIe锛夎澶囥€傚綋鍐呴儴浜ゆ崲鏈虹敱 Linux 绠＄悊鏃讹紙鍗?NIC 鐨勬墍璋?switchdev
妯″紡锛夛紝涓嶅緱瀛樺湪杩欑姝т箟銆?
鏍囧噯 ethtool 缁熻鍙互閫氳繃鐢ㄤ簬閰嶇疆鐨勬帴鍙ｈ闂€備緥濡備娇鐢ㄧ殑 ethtool 鎺ュ彛
```

  $ ethtool --include-statistics -a eth0
  Pause parameters for eth0:
  Autonegotiate:	on
  RX:			on
  TX:			on
  Statistics:
    tx_pause_frames: 1
    rx_pause_frames: 1

```
涓庝换浣曠壒瀹氬姛鑳芥棤鍏崇殑閫氱敤浠ュお缃戠粺璁￠€氳繃 `ethtool -S $ifc` 鏆撮湶锛岄€氳繃鎸囧畾
```

  $ ethtool -S eth0 --groups eth-phy eth-mac eth-ctrl rmon
  Stats for eth0:
  eth-phy-SymbolErrorDuringCarrier: 0
  eth-mac-FramesTransmittedOK: 1
  eth-mac-FrameTooLongErrors: 1
  eth-ctrl-MACControlFramesTransmitted: 1
  eth-ctrl-MACControlFramesReceived: 0
  eth-ctrl-UnsupportedOpcodesReceived: 1
  rmon-etherStatsUndersizePkts: 1
  rmon-etherStatsJabbers: 0
  rmon-rx-etherStatsPkts64Octets: 1
  rmon-rx-etherStatsPkts65to127Octets: 0
  rmon-rx-etherStatsPkts128to255Octets: 0
  rmon-tx-etherStatsPkts64Octets: 2
  rmon-tx-etherStatsPkts65to127Octets: 3
  rmon-tx-etherStatsPkts128to255Octets: 0

```
### 椹卞姩瀹氫箟鐨勭粺璁?

```

  $ ethtool -S ens4u1u1
  NIC statistics:
     tx_single_collisions: 0
     tx_multi_collisions: 0

```
## uAPI


### procfs


鍘嗗彶鎬х殑 `/proc/net/dev` 鏂囨湰鎺ュ彛鎻愪緵浜嗗鎺ュ彛鍒楄〃鍙婂叾缁熻鐨勮闂€?
娉ㄦ剰锛屽嵆浣挎鎺ュ彛鍐呴儴浣跨敤 `struct rtnl_link_stats64 <rtnl_link_stats64>`锛屽畠涔?鍚堝苟浜嗗叾涓竴浜涘瓧娈点€?
### sysfs


sysfs 涓瘡涓澶囩洰褰曢兘鍖呭惈涓€涓?`statistics` 鐩綍锛堜緥濡?`/sys/class/net/lo/statistics/`锛夛紝鍏朵腑鐨勬枃浠跺搴斾簬 `struct rtnl_link_stats64
<rtnl_link_stats64>` 鐨勬垚鍛樸€?
杩欎釜绠€鍗曠殑鎺ュ彛鍦ㄦ病鏈夊伐鍏峰彲鐢ㄧ殑鍙楅檺/宓屽叆寮忕幆澧冧腑灏ゅ叾鏂逛究銆傜劧鑰岋紝褰撹鍙栧涓粺璁℃椂
瀹冩晥鐜囦綆涓嬶紝鍥犱负瀹冨唴閮ㄦ墽琛屼簡涓€娆?`struct rtnl_link_stats64 <rtnl_link_stats64>`
鐨勫畬鏁磋浆鍌紝骞跺彧鎶ュ憡涓庢墍璁块棶鏂囦欢瀵瑰簲鐨勭粺璁°€?
Sysfs 鏂囦欢璁板綍鍦?Documentation/ABI/testing/sysfs-class-net-statistics銆?

### netlink


`rtnetlink`锛坄NETLINK_ROUTE`锛夋槸璁块棶 `struct rtnl_link_stats64
<rtnl_link_stats64>` 缁熻鐨勯閫夋柟娉曘€?
缁熻鍦ㄩ摼璺俊鎭姹傦紙`RTM_GETLINK`锛夊拰缁熻璇锋眰锛坄RTM_GETSTATS`锛屽綋璇锋眰鐨?`.filter_mask` 涓缃簡 `IFLA_STATS_LINK_64` 浣嶆椂锛夌殑鍝嶅簲涓兘浼氳鎶ュ憡銆?
#### netdev锛坣etlink锛?

`netdev` 閫氱敤 netlink 绯诲垪鍏佽璁块棶椤垫睜鍜屾瘡闃熷垪缁熻銆?
### ethtool


Ethtool IOCTL 鎺ュ彛鍏佽椹卞姩鎶ュ憡瀹炵幇鐗瑰畾鐨勭粺璁°€傚巻鍙蹭笂瀹冧篃琚敤浜庢姤鍛婂叾瀹?API 涓嶅瓨鍦?鐨勭粺璁★紝渚嬪姣忚澶囬槦鍒楃粺璁★紝鎴栧熀浜庢爣鍑嗙殑缁熻锛堜緥濡?RFC 2863锛夈€?
缁熻鍙婂叾瀛楃涓叉爣璇嗙鏄垎鍒幏鍙栫殑銆傛爣璇嗙閫氳繃 `ETHTOOL_GSTRINGS`锛堝皢 `string_set`
璁句负 `ETH_SS_STATS`锛夎幏鍙栵紝鍊奸€氳繃 `ETHTOOL_GSTATS` 鑾峰彇銆傜敤鎴风┖闂村簲浣跨敤
`ETHTOOL_GDRVINFO` 妫€绱㈢粺璁＄殑鏁伴噺锛坄.n_stats`锛夈€?
### ethtool-netlink


Ethtool netlink 鏄杈冩棫 IOCTL 鎺ュ彛鐨勬浛浠ｃ€?
鍗忚鐩稿叧鐨勭粺璁″彲浠ュ湪 get 鍛戒护涓€氳繃璁剧疆 `ETHTOOL_A_HEADER_FLAGS` 涓殑
`ETHTOOL_FLAG_STATS` 鏍囧織鏉ヨ姹傘€傜洰鍓嶄互涓嬪懡浠ゆ敮鎸佺粺璁★細

  - `ETHTOOL_MSG_FEC_GET`
  - `ETHTOOL_MSG_LINKSTATE_GET`
  - `ETHTOOL_MSG_MM_GET`
  - `ETHTOOL_MSG_PAUSE_GET`
  - `ETHTOOL_MSG_TSINFO_GET`

### debugfs


涓€浜涢┍鍔ㄩ€氳繃 `debugfs` 鏆撮湶棰濆鐨勭粺璁°€?
## struct rtnl_link_stats64


    :identifiers: rtnl_link_stats64

## 缁欓┍鍔ㄤ綔鑰呯殑娉ㄦ剰浜嬮」


椹卞姩搴斿綋鎶ュ憡鎵€鏈夊湪 `struct rtnl_link_stats64 <rtnl_link_stats64>` 涓湁瀵瑰簲鎴愬憳鐨?缁熻锛屼笖鍙兘閫氳繃 `.ndo_get_stats64` 鎶ュ憡銆傞€氳繃 ethtool 鎴?debugfs 鎶ュ憡姝ょ被鏍囧噯
缁熻灏嗕笉琚帴鍙椼€?
椹卞姩蹇呴』纭繚涓?`struct rtnl_link_stats64 <rtnl_link_stats64>` 灏藉彲鑳藉吋瀹广€傝娉ㄦ剰
渚嬪锛岃缁嗙殑閿欒缁熻蹇呴』琚姞鍏ラ€氱敤鐨?`rx_error` / `tx_error` 璁℃暟鍣ㄤ腑銆?
`.ndo_get_stats64` 鍥炶皟涓嶈兘鐫＄湢锛屽洜涓轰細閫氳繃 `/proc/net/dev` 璁块棶銆傚鏋滈┍鍔ㄥ湪浠?璁惧妫€绱㈢粺璁℃椂鍙兘浼氱潯鐪狅紝瀹冨簲褰撳畾鏈熷紓姝ュ湴鎵ц锛屽苟涓斿彧浠?`.ndo_get_stats64` 杩斿洖
鏈€杩戠殑鍓湰銆傚鏈夐渶瑕侊紝ethtool 涓柇鑱氬悎鎺ュ彛鍏佽璁剧疆鍒锋柊缁熻鐨勯鐜囥€?
妫€绱?ethtool 缁熻鏄竴涓绯荤粺璋冪敤鐨勮繃绋嬶紝寤鸿椹卞姩淇濇寔缁熻鏁伴噺鎭掑畾锛屼互閬垮厤涓庤瘯鍥?璇诲彇瀹冧滑鐨勭敤鎴风┖闂村彂鐢熺珵鎬佹潯浠躲€?
缁熻蹇呴』璺ㄥ父瑙勬搷浣滐紙渚嬪灏嗘帴鍙ｅ叧闂啀寮€鍚級鎸佺画瀛樺湪銆?
### 鍐呮牳鍐呴儴鏁版嵁缁撴瀯


浠ヤ笅缁撴瀯鏄唴鏍稿唴閮ㄧ殑锛屽畠浠湪琚浆鍌ㄦ椂琚浆鎹负 netlink 灞炴€с€傞┍鍔ㄧ粷涓嶈兘鐢?0 瑕嗙洊
瀹冧滑鏈姤鍛婄殑缁熻銆?
- ethtool_pause_stats()
- ethtool_fec_stats()
