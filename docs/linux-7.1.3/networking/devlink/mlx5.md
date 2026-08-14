
## mlx5 devlink 鏀寔


鏈枃妗ｆ弿杩颁簡 `mlx5` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


   - - 鍚嶇О
     - 妯″紡
     - 鏍￠獙
     - 璇存槑
   - - `enable_roce`
     - driverinit
     - 甯冨皵鍊?     - 鑻ヨ澶囨敮鎸佺鐢?RoCE锛屽垯 RoCE 鍚敤鐘舵€佹帶鍒惰澶囧 RoCE 鑳藉姏鐨勬敮鎸併€?       鍚﹀垯锛屾帶鍒跺彂鐢熷湪椹卞姩鏍堜腑銆傚綋鍦ㄩ┍鍔ㄥ眰闈㈢鐢?RoCE 鏃讹紝浠呮敮鎸佸師濮?       ethernet QP銆?   - - `io_eq_size`
     - driverinit
     - 鍙栧€艰寖鍥村湪 64 鍒?4096 涔嬮棿銆?     -
   - - `event_eq_size`
     - driverinit
     - 鍙栧€艰寖鍥村湪 64 鍒?4096 涔嬮棿銆?     -
   - - `max_macs`
     - driverinit
     - 鍙栧€艰寖鍥村湪 1 鍒?2^31 涔嬮棿銆備粎鏀寔 2 鐨勫箓鐨勫€笺€?     -
   - - `enable_sriov`
     - permanent
     - 甯冨皵鍊?     - 鑻ヨ澶囨敮鎸侊紝鍒欏垎鍒嫭绔嬪湴搴旂敤浜庢瘡涓墿鐞嗗姛鑳斤紙PF锛夈€傚惁鍒欙紝瀵圭О鍦?       搴旂敤浜庢墍鏈?PF銆?   - - `total_vfs`
     - permanent
     - 鍙栧€艰寖鍥村湪 1 鍒拌澶囩浉鍏崇殑鏈€澶у€间箣闂淬€?     - 鑻ヨ澶囨敮鎸侊紝鍒欏垎鍒嫭绔嬪湴搴旂敤浜庢瘡涓墿鐞嗗姛鑳斤紙PF锛夈€傚惁鍒欙紝瀵圭О鍦?       搴旂敤浜庢墍鏈?PF銆?
娉ㄦ剰锛氳濡?`enable_sriov` 鍜?`total_vfs` 杩欑被 permanent 鍙傛暟闇€瑕?FW reset 鎵嶈兘鐢熸晥


   # 璁剧疆鍙傛暟
   devlink dev param set pci/0000:01:00.0 name enable_sriov value true cmode permanent
   devlink dev param set pci/0000:01:00.0 name total_vfs value 8 cmode permanent

   # Fw reset
   devlink dev reload pci/0000:01:00.0 action fw_activate

   # 瀵逛簬 PCI 鐩稿叧閰嶇疆锛屼緥濡?sriov 闇€瑕?PCI reset/rescan锛?   echo 1 >/sys/bus/pci/devices/0000:01:00.0/remove
   echo 1 >/sys/bus/pci/rescan
   grep ^ /sys/bus/pci/devices/0000:01:00.0/sriov_*

   - - `num_doorbells`
     - driverinit
     - 璇ュ弬鏁版帶鍒?netdev 浣跨敤鐨勯€氶亾 doorbell 鏁伴噺銆傚湪鎵€鏈夋儏鍐典笅锛岄兘浼氶澶?       鍒嗛厤骞朵娇鐢ㄤ竴涓?doorbell 鐢ㄤ簬闈為€氶亾閫氫俊锛堜緥濡傜敤浜?PTP銆丠WS 绛夛級銆傛敮鎸佺殑
       鍙栧€间负锛?
       - 0锛氫笉浣跨敤閫氶亾鐗瑰畾鐨?doorbell锛屾墍鏈変簨鎯呴兘浣跨敤鍏ㄥ眬 doorbell銆?       - [1, max_num_channels]锛氬皢杩欎簺 netdev 閫氶亾鍧囨憡鍒拌繖浜?doorbell 涓娿€?
`mlx5` 椹卞姩杩樺疄鐜颁簡浠ヤ笅椹卞姩鐗瑰畾鐨勫弬鏁般€?
   :widths: 5 5 5 85

   - - 鍚嶇О
     - 绫诲瀷
     - 妯″紡
     - 鎻忚堪
   - - `flow_steering_mode`
     - string
     - runtime
     - 鎺у埗椹卞姩鐨勬祦瀵煎悜锛坒low steering锛夋ā寮?
       - `dmfs` 璁惧绠＄悊鐨勬祦瀵煎悜銆傚湪 DMFS 妯″紡涓嬶紝HW steering 瀹炰綋閫氳繃鍥轰欢
         鍒涘缓鍜岀鐞嗐€?       - `smfs` 杞欢绠＄悊鐨勬祦瀵煎悜銆傚湪 SMFS 妯″紡涓嬶紝HW steering 瀹炰綋鐢遍┍鍔ㄥ垱寤?         鍜岀鐞嗭紝鏃犻渶鍥轰欢浠嬪叆銆?       - `hmfs` 纭欢绠＄悊鐨勬祦瀵煎悜銆傚湪 HMFS 妯″紡涓嬶紝椹卞姩浣跨敤甯︽湁涓€绉嶇壒娈婄殑鏂板瀷
         WQE锛圵ork Queue Element锛夌殑 Work Queue 鐩存帴灏?steering 瑙勫垯閰嶇疆鍒?HW銆?
       涓庨粯璁ょ殑 DMFS 妯″紡鐩告瘮锛孲MFS 妯″紡鏇村揩锛屽苟鎻愪緵鏇村ソ鐨勮鍒欐彃鍏ラ€熺巼銆?   - - `fdb_large_groups`
     - u32
     - driverinit
     - 鎺у埗 FDB 琛ㄤ腑澶х粍锛堝ぇ灏?> 1锛夌殑鏁伴噺銆?
       - 榛樿鍊间负 15锛屽彇鍊艰寖鍥村湪 1 鍒?1024 涔嬮棿銆?   - - `esw_multiport`
     - 甯冨皵鍊?     - runtime
     - 鎺у埗 MultiPort E-Switch 鍏变韩 fdb 妯″紡銆?
       涓€绉嶅疄楠屾€фā寮忥紝浣跨敤鍗曚釜 E-Switch锛孨IC 涓婄殑鎵€鏈?vport 鍜岀墿鐞嗙鍙ｉ兘
       杩炴帴鍒板畠銆?
       渚嬪锛屽皢鍒涘缓鍦?PF0 涓婄殑 VF 鐨勬祦閲忓彂閫佸埌鍘熸湰涓?PF1 鐨?uplink 鍏宠仈鐨?       uplink銆?
       娉ㄦ剰锛氭湭鏉ョ殑璁惧锛孋onnectX-8 鍙婁箣鍚庯紝鏈€缁堜細灏嗗叾浣滀负榛樿鍊硷紝浠ュ厑璁稿湪
       鍗曚釜 E-switch 鐜涓墍鏈?NIC 绔彛涔嬮棿杞彂锛岃€屽弻 E-switch 妯″紡寰堝彲鑳戒細琚?       寮冪敤銆?
       榛樿鍊硷細绂佺敤
   - - `esw_port_metadata`
     - 甯冨皵鍊?     - runtime
     - 鍦ㄩ€傜敤鐨勬儏鍐典笅锛岀鐢?eswitch 鍏冩暟鎹彲鏍规嵁鐢ㄤ緥鍜屽寘澶у皬灏嗗寘閫熺巼鎻愬崌楂樿揪
       20%銆?
       Eswitch 绔彛鍏冩暟鎹姸鎬佹帶鍒舵槸鍚︾敤鍏冩暟鎹湪鍐呴儴鏍囪鍖呫€傚绔彛 RoCE銆?       representor 涔嬮棿鐨勬晠闅滆浆绉讳互鍙婂爢鍙犺澶囧繀椤诲惎鐢ㄥ厓鏁版嵁鏍囪銆傞粯璁ゆ儏鍐典笅锛?       鍦ㄥ彈鏀寔鐨?E-switch 璁惧涓婂厓鏁版嵁鏄惎鐢ㄧ殑銆傚厓鏁版嵁浠呴€傜敤浜?switchdev 妯″紡鐨?       E-switch锛屽綋鐢ㄦ埛涓嶄細浣跨敤浠ヤ笅浠讳綍鐢ㄤ緥鏃讹紝鍙互绂佺敤瀹冿細
       1. HCA 澶勪簬鍙?澶氱鍙?RoCE 妯″紡銆?       2. VF/SF representor bonding锛堥€氬父鐢ㄤ簬瀹炴椂杩佺Щ锛夈€?       3. 鍫嗗彔璁惧銆?
       褰撳厓鏁版嵁琚鐢ㄦ椂锛屽鏋滅敤鎴峰皾璇曞惎鐢ㄤ笂杩扮敤渚嬶紝瀹冧滑灏嗘棤娉曞垵濮嬪寲銆?
       娉ㄦ剰锛氳缃鍙傛暟涓嶄細绔嬪嵆鐢熸晥銆傝缃繀椤诲湪 legacy 妯″紡涓嬭繘琛岋紝eswitch 绔彛
       鍏冩暟鎹湪鍚敤 switchdev 妯″紡鍚庣敓鏁堛€?   - - `hairpin_num_queues`
     - u32
     - driverinit
     - 鎴戜滑绉版秹鍙婅浆鍙戠殑 TC NIC 瑙勫垯涓衡€渉airpin鈥濄€侶airpin 闃熷垪鏄?mlx5 閽堝姝ょ被
       鍖呯殑纭欢杞彂鐨勭壒瀹氱‖浠跺疄鐜般€?
       鎺у埗 hairpin 闃熷垪鐨勬暟閲忋€?   - - `hairpin_queue_size`
     - u32
     - driverinit
     - 鎺у埗 hairpin 闃熷垪鐨勫ぇ灏忥紙浠ュ寘涓哄崟浣嶏級銆?   - - `pcie_cong_inbound_high`
     - u16
     - driverinit
     - PCIe 鎷ュ浜嬩欢鐨勯珮闃堝€奸厤缃€傚綋璁惧渚у叆绔?PCIe 娴侀噺鍦ㄨ冻澶熼暱鐨勬椂闂村唴锛堣嚦灏?       200ms锛夎秴杩囬厤缃殑楂橀槇鍊兼椂锛屽浐浠跺皢鍙戦€佷竴涓簨浠躲€?
       鍙傝 pci_bw_inbound_high ethtool 缁熻銆?
       鍗曚綅涓?0.01 %銆傚彲鎺ュ彈鐨勫彇鍊艰寖鍥村湪 [0, 10000]銆?       pcie_cong_inbound_low < pcie_cong_inbound_high銆?       榛樿鍊硷細9000锛堝搴?90%锛夈€?   - - `pcie_cong_inbound_low`
     - u16
     - driverinit
     - PCIe 鎷ュ浜嬩欢鐨勪綆闃堝€奸厤缃€傚綋璁惧渚у叆绔?PCIe 娴侀噺鍦ㄥ厛鍓嶅凡澶勪簬鎷ュ鐘舵€佸悗
       闄嶅埌閰嶇疆鐨勪綆闃堝€间互涓嬫椂锛屽浐浠跺皢鍙戦€佷竴涓簨浠躲€?
       鍙傝 pci_bw_inbound_low ethtool 缁熻銆?
       鍗曚綅涓?0.01 %銆傚彲鎺ュ彈鐨勫彇鍊艰寖鍥村湪 [0, 10000]銆?       pcie_cong_inbound_low < pcie_cong_inbound_high銆?       榛樿鍊硷細7500銆?   - - `pcie_cong_outbound_high`
     - u16
     - driverinit
     - PCIe 鎷ュ浜嬩欢鐨勯珮闃堝€奸厤缃€傚綋璁惧渚у嚭绔?PCIe 娴侀噺鍦ㄨ冻澶熼暱鐨勬椂闂村唴锛堣嚦灏?       200ms锛夎秴杩囬厤缃殑楂橀槇鍊兼椂锛屽浐浠跺皢鍙戦€佷竴涓簨浠躲€?
       鍙傝 pci_bw_outbound_high ethtool 缁熻銆?
       鍗曚綅涓?0.01 %銆傚彲鎺ュ彈鐨勫彇鍊艰寖鍥村湪 [0, 10000]銆?       pcie_cong_outbound_low < pcie_cong_outbound_high銆?       榛樿鍊硷細9000锛堝搴?90%锛夈€?   - - `pcie_cong_outbound_low`
     - u16
     - driverinit
     - PCIe 鎷ュ浜嬩欢鐨勪綆闃堝€奸厤缃€傚綋璁惧渚у嚭绔?PCIe 娴侀噺鍦ㄥ厛鍓嶅凡澶勪簬鎷ュ鐘舵€佸悗
       闄嶅埌閰嶇疆鐨勪綆闃堝€间互涓嬫椂锛屽浐浠跺皢鍙戦€佷竴涓簨浠躲€?
       鍙傝 pci_bw_outbound_low ethtool 缁熻銆?
       鍗曚綅涓?0.01 %銆傚彲鎺ュ彈鐨勫彇鍊艰寖鍥村湪 [0, 10000]銆?       pcie_cong_outbound_low < pcie_cong_outbound_high銆?       榛樿鍊硷細7500銆?
   - - `cqe_compress_type`
     - string
     - permanent
     - 閰嶇疆 NIC 搴斾娇鐢ㄥ摢绉嶆満鍒?绠楁硶锛岃绠楁硶浼氭牴鎹?PCIe 鎬荤嚎鐘跺喌鍜屽叾浠栧唴閮?NIC
       鍥犵礌锛屽奖鍝嶅帇缂?CQE 鐨勯€熺巼锛堟縺杩涚▼搴︼級銆傛妯″紡褰卞搷鎵€鏈夊惎鐢ㄥ帇缂╃殑闃熷垪銆?       - `balanced`锛氬悎骞惰緝灏戠殑 CQE锛屽緱鍒颁腑绛夌殑鍘嬬缉姣旓紝浣嗗湪甯﹀鑺傜渷鍜屾€ц兘涔嬮棿
         淇濇寔骞宠　銆?       - `aggressive`锛氬皢鏇村 CQE 鍚堝苟涓哄崟涓潯鐩紝瀹炵幇鏇撮珮鐨勫帇缂╃巼骞舵渶澶у寲鎬ц兘锛?         灏ゅ叾鏄湪楂樻祦閲忚礋杞戒笅銆?
   - - `swp_l4_csum_mode`
     - string
     - permanent
     - 閰嶇疆璁惧鍦ㄤ娇鐢ㄨ蒋浠惰В鏋愬櫒锛圫WP锛夋彁绀烘潵瀹氫綅澶撮儴鏃跺浣曡绠?L4 鏍￠獙鍜屻€?
       - `default`锛氫娇鐢ㄨ澶囩殑榛樿鏍￠獙鍜岃绠楁ā寮忋€傞┍鍔ㄥ湪鍒濆鍖栨湡闂翠細鍙戠幇
         浣跨敤鐨勬槸 full_csum 杩樻槸 l4_only銆備笉鍏佽浠庣敤鎴风┖闂存樉寮忚缃鍊硷紝浣嗘煇浜?         鍥轰欢鐗堟湰鍦ㄨ鍙栧弬鏁版椂鍙兘杩斿洖姝ゅ€笺€?       - `full_csum`锛氳绠楀寘鍚吉澶寸殑瀹屾暣鏍￠獙鍜屻€?       - `l4_only`锛氫粎璁＄畻 L4 鏍￠獙鍜岋紝鎺掗櫎浼ご銆?
`mlx5` 椹卞姩鏀寔閫氳繃 `DEVLINK_CMD_RELOAD` 閲嶆柊鍔犺浇

## 淇℃伅鐗堟湰


`mlx5` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `fw.psid`
     - fixed
     - 鐢ㄤ簬琛ㄧず璁惧鐨勬澘鍗?id銆?   - - `fw.version`
     - stored, running
     - 涓変綅鏁板瓧 major.minor.subminor 鍥轰欢鐗堟湰鍙枫€?
## 鍋ュ悍鎶ュ憡鍣?

### tx 鎶ュ憡鍣?
tx 鎶ュ憡鍣ㄨ礋璐ｆ姤鍛婂拰鎭㈠浠ヤ笅涓夌閿欒鍦烘櫙锛?
- tx 瓒呮椂
    鍦ㄥ唴鏍告娴嬪埌 tx 瓒呮椂鏃舵姤鍛娿€?    閫氳繃鎼滅储涓㈠け鐨勪腑鏂潵鎭㈠銆?- tx 閿欒瀹屾垚
    鍦?tx 瀹屾垚鍑洪敊鏃舵姤鍛娿€?    閫氳繃鍒锋柊 tx 闃熷垪骞跺浣嶅畠鏉ユ仮澶嶃€?- tx PTP 绔彛鏃堕棿鎴?CQ 寮傚父
    鎶ュ憡绔彛 ts CQ 涓婁粠鏈姇閫掔殑 CQE 杩囧銆?    閫氳繃鍒锋柊骞堕噸寤烘墍鏈?PTP 閫氶亾鏉ユ仮澶嶃€?
tx 鎶ュ憡鍣ㄨ繕鏀寔鎸夐渶璇婃柇鍥炶皟锛岄€氳繃瀹冩彁渚涘叾鍙戦€侀槦鍒楃姸鎬佺殑瀹炴椂淇℃伅銆?
鐢ㄦ埛鍛戒护绀轰緥锛?
```

    $ devlink health diagnose pci/0000:82:00.0 reporter tx

```
   姝ゅ懡浠や粎鍦ㄦ帴鍙ｅ浜?up 鐘舵€佹椂鎵嶆湁鏈夋晥杈撳嚭锛屽惁鍒欏懡浠よ緭鍑轰负绌恒€?
- 鏄剧ず鎸囩ず鐨?tx 閿欒鏁伴噺銆佹垚鍔熺粨鏉熺殑鎭㈠娴佺▼鏁伴噺锛?```

    $ devlink health show pci/0000:82:00.0 reporter tx

```
### rx 鎶ュ憡鍣?
rx 鎶ュ憡鍣ㄨ礋璐ｆ姤鍛婂拰鎭㈠浠ヤ笅涓ょ閿欒鍦烘櫙锛?
- rx 闃熷垪鍒濆鍖栵紙濉厖锛夎秴鏃?    鐜舰缂撳啿鍖哄垵濮嬪寲鏃跺 rx 闃熷垪鎻忚堪绗︾殑濉厖鏄€氳繃瑙﹀彂涓€涓?irq 鍦?napi 涓婁笅鏂囦腑
    瀹屾垚鐨勩€傚鏋滄湭鑳借幏寰楁渶灏戞暟閲忕殑鎻忚堪绗︼紝灏变細鍙戠敓瓒呮椂锛屽苟涓斿彲浠ラ€氳繃杞 EQ
    锛圗vent Queue锛夋潵鎭㈠鎻忚堪绗︺€?- rx 甯﹂敊璇殑瀹屾垚锛堝湪涓柇涓婁笅鏂囩敱 HW 鎶ュ憡锛?    鍦?rx 瀹屾垚鍑洪敊鏃舵姤鍛娿€?    閫氳繃鍒锋柊鐩稿叧闃熷垪骞跺浣嶅畠鏉ユ仮澶嶏紙濡傛灉闇€瑕侊級銆?
rx 鎶ュ憡鍣ㄨ繕鏀寔鎸夐渶璇婃柇鍥炶皟锛岄€氳繃瀹冩彁渚涘叾鎺ユ敹闃熷垪鐘舵€佺殑瀹炴椂淇℃伅銆?
```

    $ devlink health diagnose pci/0000:82:00.0 reporter rx

```
   姝ゅ懡浠や粎鍦ㄦ帴鍙ｅ浜?up 鐘舵€佹椂鎵嶆湁鏈夋晥杈撳嚭銆傚惁鍒欙紝鍛戒护杈撳嚭涓虹┖銆?
- 鏄剧ず鎸囩ず鐨?rx 閿欒鏁伴噺銆佹垚鍔熺粨鏉熺殑鎭㈠娴佺▼鏁伴噺锛?```

    $ devlink health show pci/0000:82:00.0 reporter rx

```
### fw 鎶ュ憡鍣?
fw 鎶ュ憡鍣ㄥ疄鐜颁簡 `diagnose` 鍜?`dump` 鍥炶皟銆傚畠閫氳繃瑙﹀彂 fw core dump 骞跺皢鍏跺瓨鍏?dump 缂撳啿鍖猴紝鏉ヨ窡韪?fw 閿欒锛堜緥濡?fw syndrome锛夌殑鐥囩姸銆傜敤鎴峰彲浠ラ殢鏃惰Е鍙?fw 鎶ュ憡鍣ㄧ殑
璇婃柇鍛戒护锛屼互妫€鏌ュ綋鍓?fw 鐘舵€併€?
鐢ㄦ埛鍛戒护绀轰緥锛?
```

    $ devlink health diagnose pci/0000:82:00.0 reporter fw

```
```

    $ devlink health dump show pci/0000:82:00.0 reporter fw

```
   姝ゅ懡浠ゅ彧鑳借繍琛屽湪鎷ユ湁 fw tracer 鎵€鏈夋潈鐨?PF 涓婏紝鍦ㄥ叾浠?PF 鎴栦换浣?VF 涓婅繍琛岄兘浼?   杩斿洖鈥淥peration not permitted鈥濄€?
### fw fatal 鎶ュ憡鍣?
fw fatal 鎶ュ憡鍣ㄥ疄鐜颁簡 `dump` 鍜?`recover` 鍥炶皟銆傚畠閫氳繃 CR-space dump 鍜屾仮澶嶆祦绋?鏉ヨ窡韪嚧鍛介敊璇寚绀恒€侰R-space dump 浣跨敤 vsc 鎺ュ彛锛屽嵆浣垮湪 FW 鍛戒护鎺ュ彛涓嶅彲鐢ㄧ殑鎯呭喌涓?锛堝ぇ澶氭暟 FW 鑷村懡閿欒閮芥槸杩欑鎯呭喌锛変篃鏈夋晥銆俽ecover 鍑芥暟杩愯鎭㈠娴佺▼锛屽湪闇€瑕佹椂閲嶆柊鍔犺浇
椹卞姩骞惰Е鍙?fw reset銆傚湪鍥轰欢閿欒鏃讹紝鍋ュ悍缂撳啿鍖轰細琚?dump 鍒?dmesg銆傛棩蹇楃骇鍒簮鑷敊璇殑
涓ラ噸绋嬪害锛堝湪鍋ュ悍缂撳啿鍖轰腑缁欏嚭锛夈€?
鐢ㄦ埛鍛戒护绀轰緥锛?
```

    $ devlink health recover pci/0000:82:00.0 reporter fw_fatal

```
```

    $ devlink health dump show pci/0000:82:00.1 reporter fw_fatal

```
   姝ゅ懡浠ゅ彧鑳借繍琛屽湪 PF 涓娿€?
### vnic 鎶ュ憡鍣?
vnic 鎶ュ憡鍣ㄤ粎瀹炵幇浜?`diagnose` 鍥炶皟銆傚畠璐熻矗浠?fw 鏌ヨ vnic 璇婃柇璁℃暟鍣ㄥ苟瀹炴椂鏄剧ず
瀹冧滑銆?
vnic 璁℃暟鍣ㄧ殑鎻忚堪锛?
- total_error_queues
        鐢变簬寮傛閿欒鎴栧嚭閿欏懡浠よ€屽浜庨敊璇姸鎬佺殑闃熷垪鏁伴噺銆?- send_queue_priority_update_flow
        QP/SQ 浼樺厛绾?SL 鏇存柊浜嬩欢鐨勬暟閲忋€?- cq_overrun
        CQ 鐢变簬婧㈠嚭鑰岃繘鍏ラ敊璇姸鎬佺殑娆℃暟銆?- async_eq_overrun
        鏄犲皠鍒板紓姝ヤ簨浠剁殑 EQ 琚孩鍑虹殑娆℃暟銆?- comp_eq_overrun
        鏄犲皠鍒板畬鎴愪簨浠剁殑 EQ 琚孩鍑虹殑娆℃暟銆?- quota_exceeded_command
        鐢变簬瓒呰繃閰嶉鑰屽彂鍑哄苟澶辫触鐨勫懡浠ゆ暟閲忋€?- invalid_command
        鐢变簬閰嶉涔嬪鐨勪换浣曞叾浠栧師鍥犺€屽彂鍑哄苟澶辫触鐨勫懡浠ゆ暟閲忋€?- nic_receive_steering_discard
        瀹屾垚 RX 娴?steering 浣嗙敱浜庢祦琛ㄤ笉鍖归厤鑰岃涓㈠純鐨勫寘鏁伴噺銆?- generated_pkt_steering_fail
	鐢?VNIC 鐢熸垚骞剁粡鍘嗘剰澶?steering 澶辫触鐨勫寘鏁伴噺锛堝湪 steering 娴佺殑浠绘剰浣嶇疆锛夈€?- handled_pkt_steering_fail
	鐢?VNIC 澶勭悊骞剁粡鍘嗘剰澶?steering 澶辫触鐨勫寘鏁伴噺锛堝湪 VNIC 鎷ユ湁鐨?steering 娴佺殑
	浠绘剰浣嶇疆锛屽寘鎷?eswitch 鎵€鏈夎€呯殑 FDB锛夈€?- icm_consumption
        vnic 娑堣€楃殑浜掕繛涓绘満鍐呭瓨锛圛CM锛夐噺锛岀矑搴︿负 4KB銆侷CM 鏄?SW 鍦?HCA 璇锋眰鏃?        鍒嗛厤鐨勪富鏈哄唴瀛橈紝鐢ㄤ簬瀛樺偍鎺у埗 HCA 鎿嶄綔鐨勬暟鎹粨鏋勩€?- bar_uar_access
        瀵?PCIe BAR 涓?UAR 鐨?WRITE 鎴?READ 璁块棶鎿嶄綔娆℃暟銆?- odp_local_triggered_page_fault
        鐢?ODP 灞€閮ㄨЕ鍙戠殑缂洪〉鏁伴噺銆?- odp_remote_triggered_page_fault
        鐢?ODP 杩滅▼瑙﹀彂鐨勭己椤垫暟閲忋€?
鐢ㄦ埛鍛戒护绀轰緥锛?
```

        $ devlink health diagnose pci/0000:82:00.1 reporter vnic

```
- 璇婃柇 representor vnic 璁℃暟鍣紙閫氳繃鎻愪緵 devlink 绔彛鏉ユ墽
```

        $ devlink health diagnose pci/0000:82:00.1/65537 reporter vnic

```
   姝ゅ懡浠ゅ彲浠ヨ繍琛屽湪鎵€鏈夋帴鍙ｄ笂锛屼緥濡?PF/VF 鍜?representor 绔彛銆?