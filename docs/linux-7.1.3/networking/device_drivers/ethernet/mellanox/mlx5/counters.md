
## Ethtool 璁℃暟鍣?


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. 淇濈暀鎵€鏈夋潈鍒┿€?

## 鐩綍


- `Overview`_
- `Groups`_
- `Types`_
- `Descriptions`_

## 姒傝堪


瀛樺湪鑻ュ共涓鏁板櫒鍒嗙粍锛屽垎缁勪緷鎹槸璁℃暟鍣ㄨ缁熻鐨勪綅缃€傛澶栵紝姣忎竴缁勮鏁板櫒鍙兘鍏锋湁涓嶅悓鐨勮鏁板櫒绫诲瀷銆?

杩欎簺璁℃暟鍣ㄥ垎缁勫熀浜庣綉缁滆缃腑鐨勫摢涓粍浠讹紝
```

                                                  ----------------------------------------
                                                  |                                      |
    ----------------------------------------    ---------------------------------------- |
    |              Hypervisor              |    |                  VM                  | |
    |                                      |    |                                      | |
    | -------------------  --------------- |    | -------------------  --------------- | |
    | | Ethernet driver |  | RDMA driver | |    | | Ethernet driver |  | RDMA driver | | |
    | -------------------  --------------- |    | -------------------  --------------- | |
    |           |                 |        |    |           |                 |        | |
    |           -------------------        |    |           -------------------        | |
    |                   |                  |    |                   |                  |--
    ----------------------------------------    ----------------------------------------
                        |                                           |
            -------------               -----------------------------
            |                           |
         ------                      ------ ------ ------         ------      ------      ------
    -----| PF |----------------------| VF |-| VF |-| VF |-----  --| PF |--- --| PF |--- --| PF |---
    |    ------                      ------ ------ ------    |  | ------  | | ------  | | ------  |
    |                                                        |  |         | |         | |         |
    |                                                        |  |         | |         | |         |
    |                                                        |  |         | |         | |         |
    | eSwitch                                                |  | eSwitch | | eSwitch | | eSwitch |
    ----------------------------------------------------------  ----------- ----------- -----------
               -------------------------------------------------------------------------------
               |                                                                             |
               |                                                                             |
               | Uplink (no counters)                                                        |
               -------------------------------------------------------------------------------
                       ---------------------------------------------------------------
                       |                                                             |
                       |                                                             |
                       | MPFS (no counters)                                          |
                       ---------------------------------------------------------------
                                                     |
                                                     |
                                                     | Port

```
## 鍒嗙粍


Ring
  鐢遍┍鍔ㄦ爤濉厖鐨勮蒋浠惰鏁板櫒銆?

Netdev
  杞欢 Ring 璁℃暟鍣ㄧ殑鑱氬悎銆?

vPort counters
  鍥犳祦鎺э紙steering锛夋垨鏃犵紦鍐插尯瀵艰嚧娴侀噺璁℃暟涓庝涪鍖呫€傚彲鑳芥寚绀?NIC 瀛樺湪闂銆傝繖浜涜鏁板櫒鍖呭惈浠ュお缃戞祦閲忚鏁板櫒锛堝寘鎷?Raw Ethernet锛変互鍙?RDMA/RoCE 娴侀噺璁℃暟鍣ㄣ€?

Physical port counters
  鏀堕泦 PF 涓?VF 鐩稿叧缁熻淇℃伅鐨勮鏁板櫒銆傚彲鑳芥寚绀?NIC銆侀摼璺垨缃戠粶瀛樺湪闂銆傝娴嬮噺鐐逛繚瀛樹簡 IEEE 802.3銆丷FC2863銆丷FC 2819銆丷FC 3635 绛夋爣鍑嗗寲璁℃暟鍣紝浠ュ強娴佹帶銆丗EC 绛夐澶栬鏁板櫒鐨勪俊鎭€侾hysical port counters 涓嶄細鏆撮湶缁欒櫄鎷熸満銆?

Priority Port Counters
  涓€缁勭墿鐞嗙鍙ｈ鏁板櫒锛屾寜姣忎釜绔彛鐨勬瘡涓紭鍏堢骇鍒嗗埆缁熻銆?

## 绫诲瀷


璁℃暟鍣ㄥ垎涓轰笁绉嶇被鍨嬨€?

Traffic Informative Counters
  缁熻娴侀噺鐨勮鏁板櫒銆傝繖浜涜鏁板櫒鍙敤浜庤礋杞戒及绠楁垨涓€鑸皟璇曘€?

Traffic Acceleration Counters
  缁熻琚?Mellanox 椹卞姩鎴栫‖浠跺姞閫熻繃鐨勬祦閲忕殑璁℃暟鍣ㄣ€傝繖浜涜鏁板櫒鏄?informative 璁℃暟鍣ㄩ泦鍚堜箣涓婄殑涓€灞傦紝鍚屼竴浠芥祦閲忎細鍚屾椂琚?informative 涓?acceleration 璁℃暟鍣ㄧ粺璁°€?


Error Counters
  杩欎簺璁℃暟鍣ㄧ殑澧為暱鍙兘鎸囩ず闂銆傛瘡涓绫昏鏁板櫒閮介檮甯﹁鏄庝笌绾犳鎺柦銆?

缁熻淇℃伅鍙互閫氳繃 `ip link` 鎴?`ethtool` 鍛戒护鑾峰彇銆俙ethtool`
```

    ip 鈥搒 link show <if-name>
    ethtool -S <if-name>

```
## 鎻忚堪


XSK銆丳TP 涓?QoS 璁℃暟鍣ㄤ腑锛岃嫢涓庡厛鍓嶅凡瀹氫箟鐨勮鏁板櫒绫讳技锛屽皢涓嶅啀鍗曠嫭鍒楀嚭銆備緥濡?`ptp_tx[i]_packets` 涓嶄細琚樉寮忚鏄庯紝鍥犱负 `tx[i]_packets` 宸叉弿杩颁袱鑰呯殑琛屼负锛屽敮涓€鍖哄埆鏄?`ptp_tx[i]_packets` 浠呭湪浣跨敤浜嗙簿纭椂闂村崗璁紙precision time protocol锛夋椂鎵嶈鏁般€?

### Ring / Netdev 璁℃暟鍣?


浠ヤ笅璁℃暟鍣ㄥ湪姣忎釜 ring 鎴栬蒋浠剁鍙ｄ笂鍧囧彲鐢ㄣ€?

杩欎簺璁℃暟鍣ㄦ彁渚涘叧浜庤 NIC 鍔犻€熺殑娴侀噺澶у皬鐨勪俊鎭€傞櫎浜嗙粺璁¤娴侀噺鐨勬爣鍑嗚鏁板櫒澶栵紝杩欎簺璁℃暟鍣ㄤ細棰濆缁熻鍔犻€熸祦閲忥紙鍗冲姞閫熸祦閲忎細琚粺璁′袱娆★級銆?

涓嬭〃涓垪鍑虹殑璁℃暟鍣ㄥ悕绉板悓鏃舵寚浠?ring 涓庣鍙ｈ鏁板櫒銆俽ing 璁℃暟鍣ㄧ殑璁版硶鍖呭惈涓嶅甫鑺辨嫭鍙风殑 [i] 绱㈠紩锛涚鍙ｈ鏁板櫒鐨勮娉曞垯涓嶅寘鍚?[i]銆傝鏁板櫒鍚嶇О `rx[i]_packets` 鍦?ring 0 涓婁細鎵撳嵃涓?`rx0_packets`锛屽湪杞欢绔彛涓婁細鎵撳嵃涓?`rx_packets`銆?

   :widths: 2 3 1

   - - 璁℃暟鍣?
     - 鎻忚堪
     - 绫诲瀷

   - - `rx[i]_packets`
     - ring i 涓婃帴鏀剁殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx[i]_bytes`
     - ring i 涓婃帴鏀剁殑瀛楄妭鏁般€?
     - Informative

   - - `tx[i]_packets`
     - ring i 涓婂彂閫佺殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `tx[i]_bytes`
     - ring i 涓婂彂閫佺殑瀛楄妭鏁般€?
     - Informative

   - - `tx[i]_recover`
     - SQ 琚仮澶嶇殑娆℃暟銆?
     - Error

   - - `tx[i]_cqes`
     - ring i 涓?SQ 鍙戝嚭鐨?CQE 浜嬩欢鏁伴噺銆?
     - Informative

   - - `tx[i]_cqe_err`
     - ring i 涓?SQ 閬囧埌鐨勯敊璇?CQE 鏁伴噺銆?
     - Error

   - - `tx[i]_tso_packets`
     - ring i 涓婂彂閫佺殑 TSO 鏁版嵁鍖呮暟閲?[#accel]_銆?
     - Acceleration

   - - `tx[i]_tso_bytes`
     - ring i 涓婂彂閫佺殑 TSO 瀛楄妭鏁?[#accel]_銆?
     - Acceleration

   - - `tx[i]_tso_inner_packets`
     - ring i 涓婂彂閫佺殑銆佽鏍囪涓烘惡甯﹀唴閮ㄥ皝瑁呯殑 TSO 鏁版嵁鍖呮暟閲?[#accel]_銆?
     - Acceleration

   - - `tx[i]_tso_inner_bytes`
     - ring i 涓婂彂閫佺殑銆佽鏍囪涓烘惡甯﹀唴閮ㄥ皝瑁呯殑 TSO 瀛楄妭鏁?[#accel]_銆?
     - Acceleration

   - - `rx[i]_gro_packets`
     - 浣跨敤纭欢鍔犻€?GRO 澶勭悊鐨勬帴鏀舵暟鎹寘鏁伴噺锛屽嵆 ring i 涓婃帴鏀剁殑纭欢 GRO 鍗歌浇鏁版嵁鍖呮暟閲忋€備粎缁熻鐪熸鐨?GRO 鏁版嵁鍖咃細浠呯粺璁′綅浜?GRO 璁℃暟澶т簬 1 鐨?SKB 涓殑鏁版嵁鍖呫€?
     - Acceleration

   - - `rx[i]_gro_bytes`
     - 浣跨敤纭欢鍔犻€?GRO 澶勭悊鐨勬帴鏀跺瓧鑺傛暟锛屽嵆 ring i 涓婃帴鏀剁殑纭欢 GRO 鍗歌浇瀛楄妭鏁般€備粎缁熻鐪熸鐨?GRO 鏁版嵁鍖咃細浠呯粺璁′綅浜?GRO 璁℃暟澶т簬 1 鐨?SKB 涓殑鏁版嵁鍖呫€?
     - Acceleration

   - - `rx[i]_gro_skbs`
     - 鐢辩‖浠跺姞閫?GRO 鏋勫缓鐨?GRO SKB 鏁伴噺銆備粎缁熻 GRO 璁℃暟澶т簬 1 鐨?SKB銆?
     - Informative

   - - `rx[i]_gro_large_hds`
     - 浣跨敤纭欢鍔犻€?GRO 涓斿ご閮ㄨ緝澶с€侀渶瑕侀澶栧垎閰嶅唴瀛樼殑鎺ユ敹鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx[i]_hds_nodata_packets`
     - header/data split 妯″紡涓嬩粎鍚ご閮ㄧ殑鏁版嵁鍖呮暟閲?[#accel]_銆?
     - Informative

   - - `rx[i]_hds_nodata_bytes`
     - header/data split 妯″紡涓嬩粎鍚ご閮ㄧ殑鏁版嵁鍖呯殑瀛楄妭鏁?[#accel]_銆?
     - Informative
   - - `rx[i]_hds_nosplit_packets`
     - 鍦?header/data split 妯″紡涓嬫湭琚媶鍒嗙殑鏁版嵁鍖呮暟閲忋€傚綋纭欢涓嶆敮鎸佽鍗忚鎷嗗垎鏃讹紝鏁版嵁鍖呬笉浼氳鎷嗗垎锛屼緥濡傚崗璁?ICMPv4/v6銆傜洰鍓?header/data split 浠呮敮鎸?IPv4/IPv6 涓婄殑 TCP 涓?UDP [#accel]_銆?
     - Informative

   - - `rx[i]_hds_nosplit_bytes`
     - 鍦?header/data split 妯″紡涓嬫湭琚媶鍒嗙殑鏁版嵁鍖呯殑瀛楄妭鏁般€傚綋纭欢涓嶆敮鎸佽鍗忚鎷嗗垎鏃讹紝鏁版嵁鍖呬笉浼氳鎷嗗垎锛屼緥濡傚崗璁?ICMPv4/v6銆傜洰鍓?header/data split 浠呮敮鎸?IPv4/IPv6 涓婄殑 TCP 涓?UDP [#accel]_銆?
     - Informative

   - - `rx[i]_lro_packets`
     - ring i 涓婃帴鏀剁殑 LRO 鏁版嵁鍖呮暟閲?[#accel]_銆?
     - Acceleration

   - - `rx[i]_lro_bytes`
     - ring i 涓婃帴鏀剁殑 LRO 瀛楄妭鏁?[#accel]_銆?
     - Acceleration

   - - `rx[i]_ecn_mark`
     - 鎺ユ敹鏁版嵁鍖呬腑 ECN 鏍囪琚疆浣嶇殑鏁伴噺銆?
     - Informative

   - - `rx_oversize_pkts_buffer`
     - 鍥犻暱搴﹁秴鍑鸿澶囦负鍏ュ悜娴侀噺鍒嗛厤鐨勮蒋浠剁紦鍐插尯澶у皬銆佸埌杈?RQ 鍚庤涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝繖鍙兘鎰忓懗鐫€璁惧鐨?MTU 澶т簬杞欢缂撳啿鍖哄ぇ灏忋€?
     - Error

   - - `rx_oversize_pkts_sw_drop`
     - 鍥?CQE 鏁版嵁澶т簬 MTU 澶у皬鑰屽湪杞欢涓涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Error

   - - `rx[i]_csum_unnecessary`
     - ring i 涓婁互 `CHECKSUM_UNNECESSARY` 鎺ユ敹鐨勬暟鎹寘 [#accel]_銆?
     - Acceleration

   - - `rx[i]_csum_unnecessary_inner`
     - ring i 涓婂甫鏈夊唴閮ㄥ皝瑁呬笖浠?`CHECKSUM_UNNECESSARY` 鎺ユ敹鐨勬暟鎹寘 [#accel]_銆?
     - Acceleration

   - - `rx[i]_csum_none`
     - ring i 涓婁互 `CHECKSUM_NONE` 鎺ユ敹鐨勬暟鎹寘 [#accel]_銆?
     - Acceleration

   - - `rx[i]_csum_complete`
     - ring i 涓婁互 `CHECKSUM_COMPLETE` 鎺ユ敹鐨勬暟鎹寘 [#accel]_銆?
     - Acceleration

   - - `rx[i]_csum_complete_tail`
     - 宸茶繘琛屾牎楠屽拰璁＄畻锛堝彲鑳介渶瑕佸～鍏咃級涓旇兘澶熶娇鐢?`CHECKSUM_PARTIAL` 瀹屾垚璁＄畻鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx[i]_csum_complete_tail_slow`
     - 鏍￠獙鍜屾墍闇€鐨勫～鍏呭ぇ浜?8 瀛楄妭鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Informative

   - - `tx[i]_csum_partial`
     - ring i 涓婁互 `CHECKSUM_PARTIAL` 鍙戦€佺殑鏁版嵁鍖?[#accel]_銆?
     - Acceleration

   - - `tx[i]_csum_partial_inner`
     - ring i 涓婂甫鏈夊唴閮ㄥ皝瑁呬笖浠?`CHECKSUM_PARTIAL` 鍙戦€佺殑鏁版嵁鍖?[#accel]_銆?
     - Acceleration

   - - `tx[i]_csum_none`
     - ring i 涓婃湭浣跨敤纭欢鏍￠獙鍜屽姞閫熷彂閫佺殑鏁版嵁鍖呫€?
     - Informative

   - - `tx[i]_stopped` / `tx_queue_stopped` [#ring_global]_
     - ring i 涓?SQ 宸叉弧鐨勪簨浠躲€傝嫢璇ヨ鏁板櫒澧為暱锛岃妫€鏌ヤ负鍙戦€佸垎閰嶇殑缂撳啿鍖烘暟閲忋€?
     - Informative

   - - `tx[i]_wake` / `tx_queue_wake` [#ring_global]_
     - ring i 涓?SQ 鏇炬弧鍚庡張鍙樹负闈炴弧鐨勪簨浠躲€?
     - Informative

   - - `tx[i]_dropped` / `tx_queue_dropped` [#ring_global]_
     - ring i 涓婂洜 DMA 鏄犲皠澶辫触鑰岃涓㈠純鐨勫彂閫佹暟鎹寘銆傝嫢璇ヨ鏁板櫒澧為暱锛岃妫€鏌ヤ负鍙戦€佸垎閰嶇殑缂撳啿鍖烘暟閲忋€?
     - Error
   - - `tx[i]_nop`
     - 鐢变簬鍒拌揪寰幆缂撳啿鍖烘湯灏捐€屾彃鍏ュ埌 SQ锛堜笌 ring i 鐩稿叧锛夌殑 nop WQE锛堢┖ WQE锛夋暟閲忋€傚綋鎺ヨ繎寰幆缂撳啿鍖烘湯灏炬椂锛岄┍鍔ㄥ彲鑳戒細娣诲姞杩欎簺绌?WQE锛屼互閬垮厤鍑虹幇鏌愪釜 WQE 鍦ㄩ槦鍒楁湯灏惧紑濮嬨€佸湪闃熷垪寮€澶寸粨鏉熺殑鎯呭喌銆傝繖鏄甯哥幇璞°€?
     - Informative

   - - `tx[i]_timestamps`
     - 鍦ㄨ澶?DMA 灞傝鎵撲笂纭欢鏃堕棿鎴崇殑鍙戦€佹暟鎹寘銆?
     - Informative

   - - `tx[i]_added_vlan_packets`
     - vlan 鏍囩鎻掑叆琚嵏杞藉埌纭欢鐨勫彂閫佹暟鎹寘鏁伴噺銆?
     - Acceleration

   - - `rx[i]_removed_vlan_packets`
     - vlan 鏍囩鍓ョ琚嵏杞藉埌纭欢鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Acceleration

   - - `rx[i]_wqe_err`
     - ring i 涓婃帴鏀跺埌鐨勯敊璇搷浣滅爜鏁伴噺銆?
     - Error

   - - `rx[i]_mpwqe_frag`
     - 鍥犳棤娉曞垎閰嶅鍚堥〉鑰屽け璐ャ€佷粠鑰屾敼鐢ㄧ鐗囧寲 MPWQE锛圡ulti Packet WQE锛夌殑 WQE 鏁伴噺锛屽彂鐢熷湪 ring i 涓娿€傝嫢璇ヨ鏁板櫒涓婂崌锛屽彲鑳借〃鏄庢病鏈夎冻澶熺殑澶ч〉鍐呭瓨锛岄┍鍔ㄥ垎閰嶄簡纰庣墖鍖栭〉銆傝繖涓嶆槸寮傚父鐘跺喌銆?
     - Informative

   - - `rx[i]_mpwqe_filler_cqes`
     - ring i 涓婂彂鍑虹殑 filler CQE 浜嬩欢鏁伴噺銆?
     - Informative

   - - `rx[i]_mpwqe_filler_strides`
     - ring i 涓婅 filler CQE 娑堣€楃殑 stride 鏁伴噺銆?
     - Informative

   - - `tx[i]_mpwqe_blks`
     - 浠?Multi-Packet WQE锛坢pwqe锛夊鐞嗙殑鍙戦€佸潡鏁伴噺銆?
     - Informative

   - - `tx[i]_mpwqe_pkts`
     - 浠?Multi-Packet WQE锛坢pwqe锛夊鐞嗙殑鍙戦€佹暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx[i]_cqe_compress_blks`
     - ring i 涓婂甫鏈?CQE 鍘嬬缉鐨勬帴鏀跺潡鏁伴噺 [#accel]_銆?
     - Acceleration

   - - `rx[i]_cqe_compress_pkts`
     - ring i 涓婂甫鏈?CQE 鍘嬬缉鐨勬帴鏀舵暟鎹寘鏁伴噺 [#accel]_銆?
     - Acceleration

   - - `rx[i]_arfs_add`
     - 涓?ring i 涓婄殑鐩存帴 RQ 娴佸鍚戣€屾坊鍔犲埌璁惧鐨?aRFS 娴佽鍒欐暟閲?[#accel]_銆?
     - Acceleration

   - - `rx[i]_arfs_request_in`
     - 宸茶璇锋眰绉诲叆 ring i 浠ヨ繘琛岀洿鎺?RQ 娴佸鍚戠殑娴佽鍒欐暟閲?[#accel]_銆?
     - Acceleration

   - - `rx[i]_arfs_request_out`
     - 宸茶璇锋眰绉诲嚭 ring i 鐨勬祦瑙勫垯鏁伴噺 [#accel]_銆?
     - Acceleration

   - - `rx[i]_arfs_expired`
     - 宸茶繃鏈熷苟琚Щ闄ょ殑娴佽鍒欐暟閲?[#accel]_銆?
     - Acceleration

   - - `rx[i]_arfs_err`
     - 鏈兘鎴愬姛娣诲姞鍒版祦琛ㄧ殑娴佽鍒欐暟閲忋€?
     - Error

   - - `rx[i]_recover`
     - RQ 琚仮澶嶇殑娆℃暟銆?
     - Error

   - - `tx[i]_xmit_more`
     - 鍦?skbuff 涓婅缃簡 `xmit_more` 鎸囩ず锛堟棤闇€ doorbell锛夎€屽彂閫佺殑鏁版嵁鍖呮暟閲忋€?
     - Acceleration

   - - `ch[i]_poll`
     - 閫氶亾 i 涓?NAPI poll 鐨勮皟鐢ㄦ鏁般€?
     - Informative

   - - `ch[i]_arm`
     - 閫氶亾 i 涓?NAPI poll 鍑芥暟瀹屾垚骞朵负瀹屾垚闃熷垪鈥滃竷闃测€濓紙arm锛夌殑娆℃暟銆?
     - Informative

   - - `ch[i]_aff_change`
     - 閫氶亾 i 涓?NAPI poll 鍑芥暟鍥犱翰鍜屾€у彉鍖栬€屽湪鏌愪釜 CPU 涓婃樉寮忓仠姝㈡墽琛岀殑娆℃暟銆?
     - Informative
   - - `ch[i]_events`
     - 閫氶亾 i 鐨勫畬鎴愰槦鍒椾笂鍙戠敓鐨勭‖涓柇浜嬩欢鏁伴噺銆?
     - Informative

   - - `ch[i]_eq_rearm`
     - EQ 琚仮澶嶇殑娆℃暟銆?
     - Error

   - - `ch[i]_force_irq`
     - 閫氳繃鍚?ICOSQ 鎶曢€?NOP 鏉ョ敱 XSK 鍞ら啋瑙﹀彂 NAPI 鐨勬鏁般€?
     - Acceleration

   - - `rx[i]_congst_umr`
     - ring i 涓婂洜鎷ュ鑰屽欢杩熺殑鏈畬鎴?UMR 璇锋眰娆℃暟銆?
     - Informative

   - - `rx_pp_alloc_fast`
     - 鎴愬姛蹇€熻矾寰勫垎閰嶇殑娆℃暟銆?
     - Informative

   - - `rx_pp_alloc_slow`
     - 鎱㈤€熻矾寰?order-0 鍒嗛厤鐨勬鏁般€?
     - Informative

   - - `rx_pp_alloc_slow_high_order`
     - 鎱㈤€熻矾寰勯珮闃跺垎閰嶇殑娆℃暟銆?
     - Informative

   - - `rx_pp_alloc_empty`
     - 褰?ptr ring 涓虹┖銆佷粠鑰岃杩繘琛屾參閫熻矾寰勫垎閰嶆椂閫掑銆?
     - Informative

   - - `rx_pp_alloc_refill`
     - 褰撴煇娆″垎閰嶈Е鍙戜簡缂撳瓨琛ュ厖锛坮efill锛夋椂閫掑銆?
     - Informative

   - - `rx_pp_alloc_waive`
     - 褰撲粠 ptr ring 鑾峰彇鐨勯〉鍥?NUMA 涓嶅尮閰嶈€屾棤娉曞姞鍏ョ紦瀛樻椂閫掑銆?
     - Informative

   - - `rx_pp_recycle_cached`
     - 褰撳洖鏀跺皢椤垫斁鍏?page pool 缂撳瓨鏃堕€掑銆?
     - Informative

   - - `rx_pp_recycle_cache_full`
     - 褰?page pool 缂撳瓨宸叉弧鏃堕€掑銆?
     - Informative

   - - `rx_pp_recycle_ring`
     - 褰撻〉琚斁鍏?ptr ring 鏃堕€掑銆?
     - Informative

   - - `rx_pp_recycle_ring_full`
     - 褰撳洜 ptr ring 宸叉弧鑰屼粠 page pool 閲婃斁椤垫椂閫掑銆?
     - Informative

   - - `rx_pp_recycle_released_ref`
     - 褰撳洜 refcnt > 1 鑰岄噴鏀鹃〉锛堣€岄潪鍥炴敹锛夋椂閫掑銆?
     - Informative

   - - `rx[i]_xsk_buff_alloc_err`
     - 鍦?XSK RQ 涓婁笅鏂囦腑鍒嗛厤 skb 鎴?XSK buffer 澶辫触鐨勬鏁般€?
     - Error

   - - `rx[i]_xdp_tx_xmit`
     - 鍥?XDP 绋嬪簭鐨?`XDP_TX` 鍔ㄤ綔锛堝洖寮癸級鑰岃杞彂鍥炵鍙ｇ殑鏁版嵁鍖呮暟閲忋€傝繖浜涙暟鎹寘涓嶄細琚叾浠栬蒋浠惰鏁板櫒缁熻锛屼絾浼氳鐗╃悊绔彛涓?vPort 璁℃暟鍣ㄧ粺璁°€?
     - Informative

   - - `rx[i]_xdp_tx_mpwqe`
     - 鍦ㄧ綉缁滆澶囦笂涓嬫枃锛圧Q锛変腑鐢?netdev 鍙戦€併€佸苟琚?netdev 浠?`XDP_TX` 澶勭悊鐨勫涓暟鎹寘 WQE 鏁伴噺銆?
     - Acceleration

   - - `rx[i]_xdp_tx_inlnw`
     - 鏁版嵁鍙唴鑱斾簬 WQE 涓€佸苟鍦?RQ 涓婁笅鏂囦腑浠?`XDP_TX` 澶勭悊鐨勬暟鎹 WQE 鏁伴噺銆?
     - Acceleration

   - - `rx[i]_xdp_tx_nops`
     - 鎶曢€掑埌 XDP SQ 鐨?NOP WQEBB锛圵QE 鏋勫缓鍧楋級鏁伴噺銆?
     - Acceleration

   - - `rx[i]_xdp_tx_full`
     - 鏈簲鍥?`XDP_TX` 鍔ㄤ綔琚浆鍙戝洖绔彛銆佸嵈鍥犲彂閫侀槦鍒楀凡婊¤€岃涓㈠純鐨勬暟鎹寘鏁伴噺銆傝繖浜涙暟鎹寘涓嶄細琚叾浠栬蒋浠惰鏁板櫒缁熻锛屼絾浼氳鐗╃悊绔彛涓?vPort 璁℃暟鍣ㄧ粺璁°€傚彲浠ユ墦寮€鏇村 rx 闃熷垪骞跺皢 rx 娴侀噺鍒嗘暎鍒版墍鏈夐槦鍒楋紝鍜?鎴栧澶?rx ring 澶у皬銆?
     - Error

   - - `rx[i]_xdp_tx_err`
     - 鍦?RX ring 鐨?`XDP_TX` ring 涓婂彂鐢熺殑 `XDP_TX` 閿欒锛堝甯ц繃闀裤€佸抚杩囩煭锛夋鏁般€?
     - Error
   - - `rx[i]_xdp_tx_cqes` / `rx_xdp_tx_cqe` [#ring_global]_
     - 鍦?`XDP_TX` ring 鐨?CQ 涓婃敹鍒扮殑瀹屾垚鏁伴噺銆?
     - Informative

   - - `rx[i]_xdp_drop`
     - 鍥?XDP 绋嬪簭鐨?`XDP_DROP` 鍔ㄤ綔鑰岃涓㈠純鐨勬暟鎹寘鏁伴噺銆傝繖浜涙暟鎹寘涓嶄細琚叾浠栬蒋浠惰鏁板櫒缁熻锛屼絾浼氳鐗╃悊绔彛涓?vPort 璁℃暟鍣ㄧ粺璁°€?
     - Informative

   - - `rx[i]_xdp_redirect`
     - ring i 涓婅Е鍙?XDP redirect 鍔ㄤ綔鐨勬鏁般€?
     - Acceleration

   - - `tx[i]_xdp_xmit`
     - 琚噸瀹氬悜鍒版帴鍙ｏ紙鍥?XDP redirect锛夌殑鏁版嵁鍖呮暟閲忋€傝繖浜涙暟鎹寘涓嶄細琚叾浠栬蒋浠惰鏁板櫒缁熻锛屼絾浼氳鐗╃悊绔彛涓?vPort 璁℃暟鍣ㄧ粺璁°€?
     - Informative

   - - `tx[i]_xdp_full`
     - 琚噸瀹氬悜鍒版帴鍙ｏ紙鍥?XDP redirect锛夈€佸嵈鍥犲彂閫侀槦鍒楀凡婊¤€岃涓㈠純鐨勬暟鎹寘鏁伴噺銆傝繖浜涙暟鎹寘涓嶄細琚叾浠栬蒋浠惰鏁板櫒缁熻锛屽彲浠ュ澶?tx 闃熷垪銆?
     - Informative

   - - `tx[i]_xdp_mpwqe`
     - 浠庡叾浠?netdev 浠?`XDP_REDIRECT` 鏂瑰紡鍗歌浇鍒?NIC 鐨勫涓暟鎹寘 WQE 鏁伴噺銆?
     - Acceleration

   - - `tx[i]_xdp_inlnw`
     - 鏁版嵁鍙唴鑱斾簬 WQE 涓€佷笖鏁版嵁娈典粠鍏朵粬 netdev 浠?`XDP_REDIRECT` 鏂瑰紡鏉ョ殑 WQE 鏁版嵁娈垫暟閲忋€?
     - Acceleration

   - - `tx[i]_xdp_nops`
     - 鎶曢€掑埌 SQ銆佷笖浠庡叾浠?netdev 浠?`XDP_REDIRECT` 鏂瑰紡鏉ョ殑 NOP WQEBB锛圵QE 鏋勫缓鍧楋級鏁伴噺銆?
     - Acceleration

   - - `tx[i]_xdp_err`
     - 琚噸瀹氬悜鍒版帴鍙ｏ紙鍥?XDP redirect锛夈€佸嵈鍥犻敊璇紙濡傚抚杩囬暱銆佸抚杩囩煭锛夎€岃涓㈠純鐨勬暟鎹寘鏁伴噺銆?
     - Error

   - - `tx[i]_xdp_cqes`
     - 鍦?CQ 涓婇拡瀵归噸瀹氬悜鍒版帴鍙ｏ紙鍥?XDP redirect锛夌殑鏁版嵁鍖呮墍鏀跺埌鐨勫畬鎴愭暟閲忋€?
     - Informative

   - - `tx[i]_xsk_xmit`
     - 浣跨敤 XSK zerocopy 鍔熻兘鍙戦€佺殑鏁版嵁鍖呮暟閲忋€?
     - Acceleration

   - - `tx[i]_xsk_mpwqe`
     - 浠庡叾浠?netdev 浠?`XDP_REDIRECT` 鏂瑰紡鍗歌浇鍒?NIC 鐨勫涓暟鎹寘 WQE 鏁伴噺銆?
     - Acceleration

   - - `tx[i]_xsk_inlnw`
     - 鏁版嵁鍙唴鑱斾簬 WQE 涓€佷笖浣跨敤 XSK zerocopy 鍙戦€佺殑鏁版嵁娈?WQE 鏁伴噺銆?
     - Acceleration

   - - `tx[i]_xsk_full`
     - 鍦?XSK zerocopy 妯″紡涓?SQ 宸叉弧鏃跺搷閾冿紙doorbell锛夌殑娆℃暟銆?
     - Error

   - - `tx[i]_xsk_err`
     - 鍦?XSK zerocopy 妯″紡涓嬪彂鐢熺殑閿欒鏁伴噺锛屼緥濡傛暟鎹ぇ灏忓ぇ浜?MTU 澶у皬銆?
     - Error

   - - `tx[i]_xsk_cqes`
     - 鍦?XSK zerocopy 妯″紡涓嬪鐞嗙殑 CQE 鏁伴噺銆?
     - Acceleration

   - - `tx_tls_ctx`
     - 涓哄姞瀵嗚€屾坊鍔犲埌璁惧鐨?TLS TX HW 鍗歌浇涓婁笅鏂囨暟閲忋€?
     - Acceleration

   - - `tx_tls_del`
     - 浠庤澶囩Щ闄ょ殑 TLS TX HW 鍗歌浇涓婁笅鏂囨暟閲忥紙杩炴帴鍏抽棴锛夈€?
     - Acceleration

   - - `tx_tls_pool_alloc`
     - 鍦?TLS HW 鍗歌浇姹犱腑鎴愬姛鍒嗛厤涓€涓伐浣滃崟鍏冪殑娆℃暟銆?
     - Acceleration

   - - `tx_tls_pool_free`
     - 鍦?TLS HW 鍗歌浇姹犱腑閲婃斁涓€涓伐浣滃崟鍏冪殑娆℃暟銆?
     - Acceleration

   - - `rx_tls_ctx`
     - 涓鸿В瀵嗚€屾坊鍔犲埌璁惧鐨?TLS RX HW 鍗歌浇涓婁笅鏂囨暟閲忋€?
     - Acceleration
   - - `rx_tls_del`
     - 浠庤澶囧垹闄ょ殑 TLS RX HW 鍗歌浇涓婁笅鏂囨暟閲忥紙杩炴帴宸茬粨鏉燂級銆?
     - Acceleration

   - - `rx[i]_tls_decrypted_packets`
     - 灞炰簬 TLS 娴佷笖鎴愬姛瑙ｅ瘑鐨?RX 鏁版嵁鍖呮暟閲忋€?
     - Acceleration

   - - `rx[i]_tls_decrypted_bytes`
     - RX 鏁版嵁鍖呬腑鎴愬姛瑙ｅ瘑鐨?TLS 璐熻浇瀛楄妭鏁般€?
     - Acceleration

   - - `rx[i]_tls_resync_req_pkt`
     - 甯︽湁閲嶅悓姝ヨ姹傜殑鎺ユ敹 TLS 鏁版嵁鍖呮暟閲忋€?
     - Acceleration

   - - `rx[i]_tls_resync_req_start`
     - TLS 寮傛閲嶅悓姝ヨ姹傝鍚姩鐨勬鏁般€?
     - Acceleration

   - - `rx[i]_tls_resync_req_end`
     - TLS 寮傛閲嶅悓姝ヨ姹傛纭粨鏉熴€佸苟鎻愪緵浜?HW 璺熻釜鐨?tcp-seq 鐨勬鏁般€?
     - Acceleration

   - - `rx[i]_tls_resync_req_skip`
     - TLS 寮傛閲嶅悓姝ヨ姹傝繃绋嬭鍚姩鍗存湭姝ｇ‘缁撴潫鐨勬鏁般€?
     - Error

   - - `rx[i]_tls_resync_res_ok`
     - 瀵归┍鍔ㄧ殑 TLS 閲嶅悓姝ュ搷搴旇皟鐢ㄨ鎴愬姛澶勭悊鐨勬鏁般€?
     - Acceleration

   - - `rx[i]_tls_resync_res_retry`
     - 褰?ICOSQ 宸叉弧鏃讹紝瀵归┍鍔ㄧ殑 TLS 閲嶅悓姝ュ搷搴旇皟鐢ㄨ閲嶈瘯鐨勬鏁般€?
     - Error

   - - `rx[i]_tls_resync_res_skip`
     - 瀵归┍鍔ㄧ殑 TLS 閲嶅悓姝ュ搷搴旇皟鐢ㄦ湭鎴愬姛缁堟鐨勬鏁般€?
     - Error

   - - `rx[i]_tls_err`
     - CQE TLS 鍗歌浇鍑虹幇闂鐨勬鏁般€?
     - Error

   - - `tx[i]_tls_encrypted_packets`
     - 鐢卞唴鏍歌繘琛?TLS 鍔犲瘑鐨勫彂閫佹暟鎹寘鏁伴噺銆?
     - Acceleration

   - - `tx[i]_tls_encrypted_bytes`
     - 鐢卞唴鏍歌繘琛?TLS 鍔犲瘑鐨勫彂閫佸瓧鑺傛暟銆?
     - Acceleration

   - - `tx[i]_tls_ooo`
     - ring i 涓婂鐞嗙殑涔卞簭 TLS SQE 鍒嗙墖娆℃暟銆?
     - Acceleration

   - - `tx[i]_tls_dump_packets`
     - 閫氳繃 DMA 浠?NIC 鎷疯礉杩囨潵鐨?TLS 瑙ｅ瘑鏁版嵁鍖呮暟閲忋€?
     - Acceleration

   - - `tx[i]_tls_dump_bytes`
     - 閫氳繃 DMA 浠?NIC 鎷疯礉杩囨潵鐨?TLS 瑙ｅ瘑瀛楄妭鏁般€?
     - Acceleration

   - - `tx[i]_tls_resync_bytes`
     - 涓鸿В瀵嗚€岃姹傞噸鍚屾鐨?TLS 瀛楄妭鏁般€?
     - Acceleration

   - - `tx[i]_tls_skip_no_sync_data`
     - 鍙互瀹夊叏璺宠繃銆佹棤闇€瑙ｅ瘑鐨?TLS 鍙戦€佹暟鎹噺銆?
     - Acceleration

   - - `tx[i]_tls_drop_no_sync_data`
     - 鍥?TLS 鏁版嵁閲嶄紶鑰岃涓㈠純鐨?TLS 鍙戦€佹暟鎹噺銆?
     - Acceleration

   - - `ptp_cq[i]_abort`
     - 鍦ㄧ簿纭椂闂村崗璁紙precision time protocol锛変腑锛屽洜绔彛鏃堕棿鎴充笌 CQE 鏃堕棿鎴充箣闂寸殑鍋忓樊澶т簬 128 绉掕€屽繀椤昏烦杩囩殑 CQE 娆℃暟銆?
     - Error

   - - `ptp_cq[i]_abort_abs_diff_ns`
     - 鍦ㄧ簿纭椂闂村崗璁腑锛屽綋绔彛鏃堕棿鎴充笌 CQE 鏃堕棿鎴充箣宸ぇ浜?128 绉掓椂锛屼簩鑰呮椂闂村樊鍊肩殑绱Н銆?
     - Error

   - - `ptp_cq[i]_late_cqe`
     - 鍦?PTP 鏃堕棿鎴?CQ 涓婇€佽揪 CQE 鐨勬鏁帮紝鑰岃 CQE 鏈笉琚鏈熲€斺€斿洜涓哄凡杩囧幓涓€娈垫椂闂达紝璁惧閫氬父浼氱‘淇濅笉鎶曢€掕 CQE銆?
     - Error
   - - `ptp_cq[i]_lost_cqe`
     - 璁惧棰勬湡鍥犳椂闂村閲忔祦閫濊€屼笉浼氬湪 PTP 鏃堕棿鎴?CQE 涓婃姇閫?CQE 鐨勬鏁般€傚鏋滆繖鏍风殑 CQE 浠嶈鎶曢€掞紝鍒?`ptp_cq[i]_late_cqe` 浼氶€掑銆?
     - Error

                 鐩稿悓鍚嶇О锛堝嵆涓嶉伒寰€氱敤鍛藉悕鏂规锛夈€?

### vPort 璁℃暟鍣?


杩炴帴鍒?eSwitch 鐨?NIC 绔彛涓婄殑璁℃暟鍣ㄣ€?

   :widths: 2 3 1

   - - 璁℃暟鍣?
     - 鎻忚堪
     - 绫诲瀷

   - - `rx_vport_unicast_packets`
     - 鎺ユ敹鐨勫崟鎾暟鎹寘锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_unicast_bytes`
     - 鎺ユ敹鐨勫崟鎾瓧鑺傛暟锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_unicast_packets`
     - 鍙戦€佺殑鍗曟挱鏁版嵁鍖咃紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_unicast_bytes`
     - 鍙戦€佺殑鍗曟挱瀛楄妭鏁帮紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_multicast_packets`
     - 鎺ユ敹鐨勫鎾暟鎹寘锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_multicast_bytes`
     - 鎺ユ敹鐨勫鎾瓧鑺傛暟锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_multicast_packets`
     - 鍙戦€佺殑澶氭挱鏁版嵁鍖咃紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_multicast_bytes`
     - 鍙戦€佺殑澶氭挱瀛楄妭鏁帮紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_broadcast_packets`
     - 鎺ユ敹鐨勫箍鎾暟鎹寘锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_broadcast_bytes`
     - 鎺ユ敹鐨勫箍鎾瓧鑺傛暟锛岃瀵煎悜鍒版煇涓鍙ｏ紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_broadcast_packets`
     - 鍙戦€佺殑骞挎挱鏁版嵁鍖咃紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `tx_vport_broadcast_bytes`
     - 鍙戦€佺殑骞挎挱瀛楄妭鏁帮紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紝鍖呭惈 Raw Ethernet QP/DPDK 娴侀噺锛屼笉鍚?RDMA 娴侀噺銆?
     - Informative

   - - `rx_vport_rdma_unicast_packets`
     - 鎺ユ敹鐨?RDMA 鍗曟挱鏁版嵁鍖咃紝琚鍚戝埌鏌愪釜绔彛锛堣鏁板櫒缁熻 RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `rx_vport_rdma_unicast_bytes`
     - 鎺ユ敹鐨?RDMA 鍗曟挱瀛楄妭鏁帮紝琚鍚戝埌鏌愪釜绔彛锛堣鏁板櫒缁熻 RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `tx_vport_rdma_unicast_packets`
     - 鍙戦€佺殑 RDMA 鍗曟挱鏁版嵁鍖咃紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紙璁℃暟鍣ㄧ粺璁?RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `tx_vport_rdma_unicast_bytes`
     - 鍙戦€佺殑 RDMA 鍗曟挱瀛楄妭鏁帮紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紙璁℃暟鍣ㄧ粺璁?RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `rx_vport_rdma_multicast_packets`
     - 鎺ユ敹鐨?RDMA 澶氭挱鏁版嵁鍖咃紝琚鍚戝埌鏌愪釜绔彛锛堣鏁板櫒缁熻 RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration
   - - `rx_vport_rdma_multicast_bytes`
     - 鎺ユ敹鐨?RDMA 澶氭挱瀛楄妭鏁帮紝琚鍚戝埌鏌愪釜绔彛锛堣鏁板櫒缁熻 RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `tx_vport_rdma_multicast_packets`
     - 鍙戦€佺殑 RDMA 澶氭挱鏁版嵁鍖咃紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紙璁℃暟鍣ㄧ粺璁?RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `tx_vport_rdma_multicast_bytes`
     - 鍙戦€佺殑 RDMA 澶氭挱瀛楄妭鏁帮紝浠庢煇涓鍙ｅ鍚戝嚭鍘伙紙璁℃暟鍣ㄧ粺璁?RoCE/UD/RC 娴侀噺锛塠#accel]_銆?
     - Acceleration

   - - `vport_loopback_packets`
     - 琚幆鍥烇紙鎺ユ敹骞跺彂閫侊級鐨勫崟鎾€佸鎾拰骞挎挱鏁版嵁鍖咃紝IB/Eth [#accel]_銆?
     - Acceleration

   - - `vport_loopback_bytes`
     - 琚幆鍥烇紙鎺ユ敹骞跺彂閫侊級鐨勫崟鎾€佸鎾拰骞挎挱瀛楄妭鏁帮紝IB/Eth [#accel]_銆?
     - Acceleration

   - - `rx_steer_missed_packets`
     - NIC 鏀跺埌浣嗗洜涓嶅尮閰?NIC 娴佽〃涓换浣曟祦鑰岃涓㈠純鐨勬暟鎹寘鏁伴噺銆?
     - Error

   - - `rx_packets`
     - 浠?representor锛氱敱 hypervisor 澶勭悊鐨勬帴鏀舵暟鎹寘銆?
     - Informative

   - - `rx_bytes`
     - 浠?representor锛氱敱 hypervisor 澶勭悊鐨勬帴鏀跺瓧鑺傛暟銆?
     - Informative

   - - `tx_packets`
     - 浠?representor锛氱敱 hypervisor 澶勭悊鐨勫彂閫佹暟鎹寘銆?
     - Informative

   - - `tx_bytes`
     - 浠?representor锛氱敱 hypervisor 澶勭悊鐨勫彂閫佸瓧鑺傛暟銆?
     - Informative

   - - `dev_internal_queue_oob`
     - 鍥犲唴閮ㄨ澶?RQ 缂哄皯鎺ユ敹 WQE 鑰岃涓㈠純鐨勬暟鎹寘鏁伴噺銆?
     - Error

### 鐗╃悊绔彛璁℃暟鍣?


鐗╃悊绔彛璁℃暟鍣ㄦ槸杩炴帴閫傞厤鍣ㄤ笌缃戠粶鐨勫閮ㄧ鍙ｄ笂鐨勮鏁板櫒銆傝娴嬮噺鐐逛繚瀛樹簡 IEEE 802.3銆丷FC2863銆丷FC 2819銆丷FC 3635 绛夋爣鍑嗗寲璁℃暟鍣紝浠ュ強娴佹帶銆丗EC 绛夐澶栬鏁板櫒鐨勪俊鎭€?

   :widths: 2 3 1

   - - 璁℃暟鍣?
     - 鎻忚堪
     - 绫诲瀷

   - - `rx_packets_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑鏁版嵁鍖呮暟閲忋€傝璁℃暟鍣ㄤ笉鍖呭惈鍥?FCS銆佸抚澶у皬鍙婄被浼奸敊璇€岃涓㈠純鐨勬暟鎹寘銆?
     - Informative

   - - `tx_packets_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑瀛楄妭鏁帮紝鍖呭惈浠ュお缃戝ご閮ㄤ笌 FCS銆?
     - Informative

   - - `tx_bytes_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑瀛楄妭鏁般€?
     - Informative

   - - `rx_multicast_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶氭挱鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `tx_multicast_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑澶氭挱鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_broadcast_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑骞挎挱鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `tx_broadcast_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑骞挎挱鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_crc_errors_phy`
     - 鐗╃悊绔彛涓婂洜 FCS锛團rame Check Sequence锛屽抚鏍￠獙搴忓垪锛夐敊璇€岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒楂橀€熺巼澧為暱锛岃浣跨敤涓嬫柟鐨?`rx_symbol_error_phy` 涓?`rx_corrected_bits_phy` 璁℃暟鍣ㄦ鏌ラ摼璺川閲忋€?
     - Error
   - - `rx_in_range_len_errors_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂鐨勯暱搴?绫诲瀷閿欒鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Error

   - - `rx_out_of_range_len_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害瓒呭嚭鍏佽鍊艰€岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃杩炴帴鍒伴€傞厤鍣ㄧ殑瀵圭閰嶇疆浜嗘洿澶х殑 MTU銆備娇鐢ㄧ浉鍚岀殑 MTU 閰嶇疆鍗冲彲瑙ｅ喅姝ら棶棰樸€?
     - Error

   - - `rx_oversize_pkts_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害瓒呭嚭 MTU 澶у皬鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃杩炴帴鍒伴€傞厤鍣ㄧ殑瀵圭閰嶇疆浜嗘洿澶х殑 MTU銆備娇鐢ㄧ浉鍚岀殑 MTU 閰嶇疆鍗冲彲瑙ｅ喅姝ら棶棰樸€?
     - Error

   - - `rx_symbol_err_phy`
     - 鍥犵墿鐞嗙紪鐮侀敊璇紙绗﹀彿閿欒锛夎€岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺锛屽彂鐢熷湪鐗╃悊绔彛涓娿€?
     - Error

   - - `rx_mac_control_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑 MAC 鎺у埗鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `tx_mac_control_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑 MAC 鎺у埗鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_pause_ctrl_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑閾捐矾灞?pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃缃戠粶鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷€傞厤鍣ㄧ殑娴侀噺銆?
     - Informative

   - - `tx_pause_ctrl_phy`
     - 鐗╃悊绔彛涓婂彂閫佺殑閾捐矾灞?pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃 NIC 鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷綉缁滅殑娴侀噺銆?
     - Informative

   - - `rx_unsupported_op_phy`
     - 鐗╃悊绔彛涓婃帴鏀跺埌鐨勩€佸甫鏈変笉鏀寔鎿嶄綔鐮佺殑 MAC 鎺у埗鏁版嵁鍖呮暟閲忋€?
     - Error

   - - `rx_discards_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂缂撳啿鍖轰笉瓒宠€岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃閫傞厤鍣ㄥ彂鐢熸嫢濉烇紝鏃犳硶鍚告敹鏉ヨ嚜缃戠粶鐨勬祦閲忋€?
     - Error

   - - `tx_discards_phy`
     - 鍦ㄥ彂閫佹椂琚涪寮冪殑鏁版嵁鍖呮暟閲忥紝鍗充究鏈娴嬪埌閿欒銆備涪寮冨彲鑳界敱浜庨摼璺浜?down 鐘舵€併€侀槦澶撮樆濉烇紙head of line drop锛夈€佹潵鑷綉缁滅殑 pause 绛夊師鍥犲彂鐢熴€?
     - Error

   - - `tx_errors_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害瓒呭嚭 MTU 澶у皬鑰岃涓㈠純鐨勫彂閫佹暟鎹寘鏁伴噺銆?
     - Error

   - - `rx_undersize_pkts_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害鐭簬 64 瀛楄妭鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃杩炴帴鍒伴€傞厤鍣ㄧ殑瀵圭閰嶇疆浜嗛潪鏍囧噯 MTU锛屾垨鏈夌暩褰㈡暟鎹寘鍒拌揪銆?
     - Error

   - - `rx_fragments_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害鐭簬 64 瀛楄妭涓斿瓨鍦?FCS 閿欒鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃杩炴帴鍒伴€傞厤鍣ㄧ殑瀵圭閰嶇疆浜嗛潪鏍囧噯 MTU銆?
     - Error

   - - `rx_jabbers_phy`
     - 鍥犵墿鐞嗙鍙ｄ笂闀垮害闀夸簬 64 瀛楄妭涓斿瓨鍦?FCS 閿欒鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Error

   - - `rx_64_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?64 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative
   - - `rx_65_to_127_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?65 鍒?127 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_128_to_255_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?128 鍒?255 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_256_to_511_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?256 鍒?511 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_512_to_1023_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?512 鍒?1023 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_1024_to_1518_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?1024 鍒?1518 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_1519_to_2047_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?1519 鍒?2047 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_2048_to_4095_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?2048 鍒?4095 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_4096_to_8191_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?4096 鍒?8191 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_8192_to_10239_bytes_phy`
     - 鐗╃悊绔彛涓婃帴鏀剁殑澶у皬涓?8192 鍒?10239 瀛楄妭鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `link_down_events_phy`
     - 閾捐矾杩愯鐘舵€佸彉涓?down 鐨勬鏁般€傝嫢璇ヨ鏁板櫒澧為暱锛屽彲鑳芥剰鍛崇潃绔彛鎶栧姩锛坒lapping锛夛紝鍙兘闇€瑕佹洿鎹㈢嚎缂?鏀跺彂鍣ㄣ€?
     - Error

   - - `total_success_recovery_phy`
     - 绔彛澶嶄綅鍛ㄦ湡鍐呬换鎰忕被鍨嬬殑鎬绘垚鍔熸仮澶嶄簨浠舵鏁般€?
     - Error

   - - `rx_out_of_buffer`
     - 鎺ユ敹闃熷垪娌℃湁涓洪€傞厤鍣ㄥ叆鍚戞祦閲忓垎閰嶈蒋浠剁紦鍐插尯鐨勬鏁般€?
     - Error

   - - `module_bus_stuck`
     - 妫€娴嬪埌妯″潡 I\ `2`\C 鎬荤嚎锛堟暟鎹垨鏃堕挓锛夌煭璺殑娆℃暟銆傚彲鑳介渶瑕佹洿鎹㈢嚎缂?鏀跺彂鍣ㄣ€?
     - Error

   - - `module_high_temp`
     - 妯″潡娓╁害杩囬珮鍙戠敓鐨勬鏁般€傝嫢闂鎸佺画锛屽彲鑳介渶瑕佹鏌ョ幆澧冩俯搴︽垨鏇存崲绾跨紗/鏀跺彂鍣ㄦā鍧椼€?
     - Error

   - - `module_bad_shorted`
     - 妯″潡绾跨紗鐭矾鐨勬鏁般€傚彲鑳介渶瑕佹洿鎹㈢嚎缂?鏀跺彂鍣ㄦā鍧椼€?
     - Error

   - - `module_unplug`
     - 妯″潡琚脊鍑虹殑娆℃暟銆?
     - Informative

   - - `rx_buffer_passed_thres_phy`
     - 绔彛鎺ユ敹缂撳啿鍖鸿秴杩?85% 婊＄殑浜嬩欢鏁伴噺銆?
     - Informative

   - - `tx_pause_storm_warning_events`
     - 璁惧闀挎椂闂村彂閫?pause 鐨勬鏁般€?
     - Informative

   - - `tx_pause_storm_error_events`
     - 璁惧闀挎椂闂村彂閫?pause銆佹渶缁堣秴鏃跺苟绂佺敤 pause 甯у彂閫佺殑娆℃暟銆傚湪 pause 甯ц绂佺敤鐨勬湡闂达紝鍙兘鍙戠敓涓㈠寘銆?
     - Error

   - - `rx[i]_buff_alloc_err`
     - 鍦?ring i 涓婂垎閰嶆帴鏀舵暟鎹寘锛堟垨 SKB锛夌紦鍐插尯澶辫触銆?
     - Error
   - - `rx_bits_phy`
     - 璇ヨ鏁板櫒鎻愪緵鏈彲琚帴鏀剁殑娴侀噺鎬婚噺淇℃伅锛屽彲鐢ㄤ綔琛￠噺 `rx_pcs_symbol_err_phy` 涓?`rx_corrected_bits_phy` 涓敊璇祦閲忔瘮渚嬬殑鍙傝€冦€?
     - Informative

   - - `rx_pcs_symbol_err_phy`
     - 璇ヨ鏁板櫒缁熻鏈 FEC 绾犳绠楁硶绾犳銆佹垨璇ユ帴鍙ｄ笂 FEC 绠楁硶鏈縺娲荤殑绗﹀彿閿欒鏁伴噺銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃 NIC 涓庣綉缁滀箣闂寸殑閾捐矾瀛樺湪楂?BER锛堣鐮佺巼锛夛紝涓旀湁娴侀噺涓㈠け锛屽彲鑳介渶瑕佹洿鎹㈢嚎缂?鏀跺彂鍣ㄣ€傞敊璇巼涓虹壒瀹氭椂闂村抚鍐?`rx_pcs_symbol_err_phy` 鏁伴噺闄や互 `rx_bits_phy` 鏁伴噺銆?
     - Error

   - - `rx_corrected_bits_phy`
     - 鏍规嵁娲诲姩 FEC锛圧S/FC锛夊湪璇ョ鍙ｄ笂琚籂姝ｇ殑姣旂壒鏁般€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃 NIC 涓庣綉缁滀箣闂寸殑閾捐矾瀛樺湪楂?BER銆傜籂姝ｆ瘮鐗圭巼涓虹壒瀹氭椂闂村抚鍐?`rx_corrected_bits_phy` 鏁伴噺闄や互 `rx_bits_phy` 鏁伴噺銆?
     - Error

   - - `rx_err_lane_[l]_phy`
     - 璇ヨ鏁板櫒缁熻姣忎釜閫氶亾 l 绱㈠紩涓婄殑鐗╃悊鍘熷閿欒鏁伴噺锛岀粺璁″湪 FEC 绾犳涔嬪墠銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃 NIC 涓庣綉缁滀箣闂寸殑閾捐矾瀛樺湪楂?BER锛屼笖鍙兘鏈夋祦閲忎涪澶憋紝鍙兘闇€瑕佹洿鎹㈢嚎缂?鏀跺彂鍣ㄣ€傝缁撳悎 `rx_corrected_bits_phy` 涓€骞舵鏌ャ€?
     - Error

   - - `rx_global_pause`
     - 鐗╃悊绔彛涓婃帴鏀剁殑 pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃缃戠粶鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷€傞厤鍣ㄧ殑娴侀噺銆傛敞鎰忥細璇ヨ鏁板櫒浠呭湪鍚敤鍏ㄥ眬 pause 妯″紡鏃舵湁鏁堛€?
     - Informative

   - - `rx_global_pause_duration`
     - 鐗╃悊绔彛涓婃帴鏀?pause 鐨勬寔缁椂闂达紙寰锛夈€傝璁℃暟鍣ㄨ〃绀虹鍙ｆ湭鍙戦€佷换浣曟祦閲忕殑鏃堕棿銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃缃戠粶鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷€傞厤鍣ㄧ殑娴侀噺銆傛敞鎰忥細璇ヨ鏁板櫒浠呭湪鍚敤鍏ㄥ眬 pause 妯″紡鏃舵湁鏁堛€?
     - Informative

   - - `tx_global_pause`
     - 鐗╃悊绔彛涓婂彂閫佺殑 pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃閫傞厤鍣ㄥ彂鐢熸嫢濉烇紝鏃犳硶鍚告敹鏉ヨ嚜缃戠粶鐨勬祦閲忋€傛敞鎰忥細璇ヨ鏁板櫒浠呭湪鍚敤鍏ㄥ眬 pause 妯″紡鏃舵湁鏁堛€?
     - Informative

   - - `tx_global_pause_duration`
     - 鐗╃悊绔彛涓?pause 鍙戦€佸櫒鐨勬寔缁椂闂达紙寰锛夈€傛敞鎰忥細璇ヨ鏁板櫒浠呭湪鍚敤鍏ㄥ眬 pause 妯″紡鏃舵湁鏁堛€?
     - Informative

   - - `rx_global_pause_transition`
     - 鐗╃悊绔彛涓婁粠 Xoff 鍒?Xon 鐨勫垏鎹㈡鏁般€傛敞鎰忥細璇ヨ鏁板櫒浠呭湪鍚敤鍏ㄥ眬 pause 妯″紡鏃舵湁鏁堛€?
     - Informative

   - - `rx_if_down_packets`
     - 鍥犳帴鍙?down 鑰岃涓㈠純鐨勬帴鏀舵暟鎹寘鏁伴噺銆?
     - Informative

### 浼樺厛绾х鍙ｈ鏁板櫒


浠ヤ笅璁℃暟鍣ㄦ槸鎸?L2 浼樺厛绾э紙0-7锛夌粺璁＄殑鐗╃悊绔彛璁℃暟鍣ㄣ€?
**娉ㄦ剰锛?* 璁℃暟鍣ㄥ悕绉颁腑鐨?`p` 琛ㄧず浼樺厛绾с€?

   :widths: 2 3 1

   - - 璁℃暟鍣?
     - 鎻忚堪
     - 绫诲瀷

   - - `rx_prio[p]_bytes`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鎺ユ敹鐨勫瓧鑺傛暟銆?
     - Informative

   - - `rx_prio[p]_packets`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鎺ユ敹鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `tx_prio[p]_bytes`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鍙戦€佺殑瀛楄妭鏁般€?
     - Informative

   - - `tx_prio[p]_packets`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鍙戦€佺殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_prio[p]_pause`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鎺ユ敹鐨?pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃缃戠粶鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷€傞厤鍣ㄧ殑娴侀噺銆傛敞鎰忥細璇ヨ鏁板櫒浠呭湪浼樺厛绾?p 涓婂惎鐢ㄤ簡 PFC 鏃跺彲鐢ㄣ€?
     - Informative

   - - `rx_prio[p]_pause_duration`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鎺ユ敹 pause 鐨勬寔缁椂闂达紙寰锛夈€傝璁℃暟鍣ㄨ〃绀虹鍙ｅ湪璇ヤ紭鍏堢骇涓婃湭鍙戦€佷换浣曟祦閲忕殑鏃堕棿銆傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃缃戠粶鍙戠敓鎷ュ锛屾棤娉曞惛鏀舵潵鑷€傞厤鍣ㄧ殑娴侀噺銆傛敞鎰忥細璇ヨ鏁板櫒浠呭湪浼樺厛绾?p 涓婂惎鐢ㄤ簡 PFC 鏃跺彲鐢ㄣ€?
     - Informative

   - - `rx_prio[p]_pause_transition`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 浠?Xoff 鍒?Xon 鐨勫垏鎹㈡鏁般€傛敞鎰忥細璇ヨ鏁板櫒浠呭湪浼樺厛绾?p 涓婂惎鐢ㄤ簡 PFC 鏃跺彲鐢ㄣ€?
     - Informative

   - - `tx_prio[p]_pause`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鍙戦€佺殑 pause 鏁版嵁鍖呮暟閲忋€傝嫢璇ヨ鏁板櫒澧為暱锛屾剰鍛崇潃閫傞厤鍣ㄥ彂鐢熸嫢濉烇紝鏃犳硶鍚告敹鏉ヨ嚜缃戠粶鐨勬祦閲忋€傛敞鎰忥細璇ヨ鏁板櫒浠呭湪浼樺厛绾?p 涓婂惎鐢ㄤ簡 PFC 鏃跺彲鐢ㄣ€?
     - Informative

   - - `tx_prio[p]_pause_duration`
     - 鐗╃悊绔彛涓婁互浼樺厛绾?p 鐨?pause 鍙戦€佸櫒鎸佺画鏃堕棿锛堝井绉掞級銆傛敞鎰忥細璇ヨ鏁板櫒浠呭湪浼樺厛绾?p 涓婂惎鐢ㄤ簡 PFC 鏃跺彲鐢ㄣ€?
     - Informative

   - - `rx_prio[p]_buf_discard`
     - 璁惧鍥犵己灏戞瘡涓绘満鎺ユ敹缂撳啿鍖鸿€屼涪寮冪殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_prio[p]_cong_discard`
     - 璁惧鍥犳瘡涓绘満鎷ュ鑰屼涪寮冪殑鏁版嵁鍖呮暟閲忋€?
     - Informative

   - - `rx_prio[p]_marked`
     - 璁惧鍥犳瘡涓绘満鎷ュ鑰岃繘琛?ECN 鏍囪鐨勬暟鎹寘鏁伴噺銆?
     - Informative

   - - `rx_prio[p]_discards`
     - 璁惧鍥犵己灏戞帴鏀剁紦鍐插尯鑰屼涪寮冪殑鏁版嵁鍖呮暟閲忋€?
     - Informative

### 璁惧璁℃暟鍣?


   :widths: 2 3 1

   - - 璁℃暟鍣?
     - 鎻忚堪
     - 绫诲瀷

   - - `rx_pci_signal_integrity`
     - 缁熻鐗╃悊灞?PCIe 淇″彿瀹屾暣鎬ч敊璇紝浠ュ強鍥犲抚閿欒鍜?CRC锛坉lp 涓?tlp锛夎€岃繘鍏?recovery 鐨勬鏁般€傝嫢璇ヨ鏁板櫒涓婂崌锛屽皾璇曞皢閫傞厤鍣ㄥ崱鎹㈠埌鍙︿竴涓彃妲斤紝浠ユ帓闄?PCI 鎻掓Ы鏁呴殰銆傝纭浣犺繍琛岀殑鏄渶鏂扮殑鍙敤鍥轰欢涓庢渶鏂扮殑鏈嶅姟鍣?BIOS 鐗堟湰銆?
     - Error
   - - `tx_pci_signal_integrity`
     - 缁熻鐗╃悊灞?PCIe 淇″彿瀹屾暣鎬ч敊璇紝浠ュ強鐢卞绔彂璧风殑杩涘叆 recovery 鐨勬鏁帮紙鍥犳敹鍒?TS/EIEOS 鑰岃繘鍏?recovery锛夈€傝嫢璇ヨ鏁板櫒涓婂崌锛屽皾璇曞皢閫傞厤鍣ㄥ崱鎹㈠埌鍙︿竴涓彃妲斤紝浠ユ帓闄?PCI 鎻掓Ы鏁呴殰銆傝纭浣犺繍琛岀殑鏄渶鏂扮殑鍙敤鍥轰欢涓庢渶鏂扮殑鏈嶅姟鍣?BIOS 鐗堟湰銆?
     - Error

   - - `outbound_pci_buffer_overflow`
     - 鍥?PCI 缂撳啿鍖烘孩鍑鸿€岃涓㈠純鐨勬暟鎹寘鏁伴噺銆傝嫢璇ヨ鏁板櫒楂橀€熺巼涓婂崌锛屽彲鑳芥剰鍛崇潃鏌愪富鏈虹殑鎺ユ敹娴侀噺閫熺巼瓒呰繃浜?PCIe 鎬荤嚎锛屼粠鑰屽彂鐢熸嫢濉炪€?
     - Informative

   - - `outbound_pci_stalled_rd`
     - 鍦ㄨ繃鍘讳竴绉掑唴锛孨IC 鏈夊嚭绔欓潪 posted 璇昏姹備絾鍥?posted credit 涓嶈冻鑰屾棤娉曟墽琛岀殑鏃堕棿鍗犳瘮锛堣寖鍥?0...100锛夈€?
     - Informative

   - - `outbound_pci_stalled_wr`
     - 鍦ㄨ繃鍘讳竴绉掑唴锛孨IC 鏈夊嚭绔?posted 鍐欒姹備絾鍥?posted credit 涓嶈冻鑰屾棤娉曟墽琛岀殑鏃堕棿鍗犳瘮锛堣寖鍥?0...100锛夈€?
     - Informative

   - - `outbound_pci_stalled_rd_events`
     - `outbound_pci_stalled_rd` 楂樹簬 30% 鐨勭鏁般€?
     - Informative

   - - `outbound_pci_stalled_wr_events`
     - `outbound_pci_stalled_wr` 楂樹簬 30% 鐨勭鏁般€?
     - Informative

   - - `dev_out_of_buffer`
     - 璁惧鑷湁闃熷垪娌℃湁鍒嗛厤瓒冲缂撳啿鍖虹殑娆℃暟銆?
     - Error

   - - `pci_bw_inbound_high`
     - 璁惧瓒婅繃鍏ョ珯 PCIe 楂樺甫瀹介槇鍊肩殑娆℃暟銆傞渶涓?`pci_bw_inbound_low` 姣旇緝浠ュ垽鏂澶囨槸鍚﹀浜庢嫢濉炵姸鎬併€?
       鑻?`pci_bw_inbound_high` == `pci_bw_inbound_low`锛屽垯璁惧鏈嫢濉炪€?
       鑻?`pci_bw_inbound_high` > `pci_bw_inbound_low`锛屽垯璁惧宸叉嫢濉炪€?
     - Informative

   - - `pci_bw_inbound_low`
     - 璁惧瓒婅繃浣庡叆绔?PCIe 甯﹀闃堝€肩殑娆℃暟銆傞渶涓?`pci_bw_inbound_high` 姣旇緝浠ュ垽鏂澶囨槸鍚﹀浜庢嫢濉炵姸鎬併€?
       鑻?`pci_bw_inbound_high` == `pci_bw_inbound_low`锛屽垯璁惧鏈嫢濉炪€?
       鑻?`pci_bw_inbound_high` > `pci_bw_inbound_low`锛屽垯璁惧宸叉嫢濉炪€?
     - Informative

   - - `pci_bw_outbound_high`
     - 璁惧瓒婅繃鍑虹珯 PCIe 楂樺甫瀹介槇鍊肩殑娆℃暟銆傞渶涓?`pci_bw_outbound_low` 姣旇緝浠ュ垽鏂澶囨槸鍚﹀浜庢嫢濉炵姸鎬併€?
       鑻?`pci_bw_outbound_high` == `pci_bw_outbound_low`锛屽垯璁惧鏈嫢濉炪€?
       鑻?`pci_bw_outbound_high` > `pci_bw_outbound_low`锛屽垯璁惧宸叉嫢濉炪€?
     - Informative

   - - `pci_bw_outbound_low`
     - 璁惧瓒婅繃浣庡嚭绔?PCIe 甯﹀闃堝€肩殑娆℃暟銆傞渶涓?`pci_bw_outbound_high` 姣旇緝浠ュ垽鏂澶囨槸鍚﹀浜庢嫢濉炵姸鎬併€?
       鑻?`pci_bw_outbound_high` == `pci_bw_outbound_low`锛屽垯璁惧鏈嫢濉炪€?
       鑻?`pci_bw_outbound_high` > `pci_bw_outbound_low`锛屽垯璁惧宸叉嫢濉炪€?
     - Informative

   - - `pci_bw_stale_event`
     - 璁惧瑙﹀彂 PCIe 鎷ュ浜嬩欢銆佷絾鏌ヨ鏃跺彂鐜扮姸鎬佹棤鍙樺寲鐨勬鏁般€?
     - Informative