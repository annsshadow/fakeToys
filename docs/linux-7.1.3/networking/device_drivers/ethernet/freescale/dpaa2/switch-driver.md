
## DPAA2 浜ゆ崲鏈洪┍鍔?

:Copyright: |copy| 2021 NXP

DPAA2 浜ゆ崲鏈洪┍鍔ㄥ湪 Datapath Switch锛圖PSW锛夊璞′笂鎺㈡祴锛坧robe锛夛紝璇ュ璞″彲鍦ㄤ互涓?DPAA2 SoC 鍙婂叾鍙樹綋涓婂疄渚嬪寲锛歀S2088A 鍜?LX2160A銆?
椹卞姩浣跨敤浜ゆ崲鏈鸿澶囬┍鍔ㄦā鍨嬶紝骞舵妸姣忎釜浜ゆ崲鏈虹鍙ｄ綔涓轰竴涓綉缁滄帴鍙ｆ毚闇插嚭鏉ワ紝鏃㈠彲浠ヨ绾冲叆缃戞ˉ锛屼篃鍙互浣滀负鐙珛鎺ュ彛浣跨敤銆傜鍙ｄ箣闂翠氦鎹㈢殑娴侀噺浼氳鍗歌浇鍒扮‖浠朵腑銆?
DPSW 鍙互鏈夎繛鎺ュ埌 DPNI 鎴栬繛鎺ュ埌 DPMAC 浠ュ疄鐜板閮ㄨ闂殑绔彛銆?```

         [ethA]     [ethB]      [ethC]     [ethD]     [ethE]     [ethF]
            :          :          :          :          :          :
            :          :          :          :          :          :
       [dpaa2-eth]  [dpaa2-eth]  [              dpaa2-switch              ]
            :          :          :          :          :          :        kernel
       =============================================================================
            :          :          :          :          :          :        hardware
         [DPNI]      [DPNI]     [============= DPSW =================]
            |          |          |          |          |          |
            |           ----------           |       [DPMAC]    [DPMAC]
             -------------------------------            |          |
                                                        |          |
                                                      [PHY]      [PHY]

```
## 鍒涘缓涓€涓互澶綉浜ゆ崲鏈?

dpaa2-switch 椹卞姩鍦?fsl-mc 鎬荤嚎涓婂彂鐜扮殑 DPSW 璁惧涓婃帰娴嬨€傝繖浜涜澶囨棦鍙互閫氳繃鍚姩鏃堕厤缃枃浠垛€斺€擠ataPath Layout锛圖PL锛夆€斺€旈潤鎬佸垱寤猴紝涔熷彲浠ュ湪杩愯鏃朵娇鐢?DPAA2 瀵硅薄 API锛堝凡闆嗘垚鍒?restool 鐢ㄦ埛绌洪棿宸ュ叿涓級鍒涘缓銆?
鐩墠锛宒paa2-switch 椹卞姩瀵瑰畠瑕佹帰娴嬬殑 DPSW 瀵硅薄鏂藉姞浜嗕互涓嬮檺鍒讹細

 - FDB 鐨勬渶灏忔暟閲忓簲鑷冲皯绛変簬浜ゆ崲鏈烘帴鍙ｇ殑鏁伴噺銆傝繖鏄负浜嗗疄鐜颁氦鎹㈡満绔彛鐨勯殧绂绘墍蹇呴渶鐨勶紝鍗冲綋涓嶅湪缃戞ˉ涓嬫椂锛屾瘡涓氦鎹㈡満绔彛灏嗘嫢鏈夎嚜宸辩殑 FDB銆?```

        fsl_dpaa2_switch dpsw.0: The number of FDBs is lower than the number of ports, cannot probe

 * 骞挎挱鍜屾椽娉涳紙flooding锛夐厤缃兘搴旀槸姣忎釜 FDB 鐙珛鐨勩€傝繖浣垮緱椹卞姩鑳藉鏍规嵁鍏变韩璇?FDB 鐨勪氦鎹㈡満绔彛锛堝嵆澶勪簬鍚屼竴缃戞ˉ涓嬶級鏉ラ檺鍒舵瘡涓?FDB 鐨勫箍鎾拰娲硾鍩熴€?   ::

        fsl_dpaa2_switch dpsw.0: Flooding domain is not per FDB, cannot probe
        fsl_dpaa2_switch dpsw.0: Broadcast domain is not per FDB, cannot probe

 * 浜ゆ崲鏈虹殑鎺у埗鎺ュ彛涓嶅簲琚鐢紙鍒涘缓鏃堕€夐」涓嶅簲浼犲叆 DPSW_OPT_CTRL_IF_DIS锛夈€傛病鏈夋帶鍒舵帴鍙ｏ紝椹卞姩灏辨棤娉曞湪浜ゆ崲鏈虹鍙?netdevices 涓婃彁渚涙纭殑 Rx/Tx 娴侀噺鏀寔銆?   ::

        fsl_dpaa2_switch dpsw.0: Control Interface is disabled, cannot probe

```
闄や簡瀹為檯 DPSW 瀵硅薄鐨勯厤缃锛宒paa2-switch 椹卞姩杩橀渶瑕佷互涓?DPAA2 瀵硅薄锛?
 - 1 涓?DPMCP - 浠讳綍涓?MC 鍥轰欢鐨勪氦浜掗兘闇€瑕佷竴涓?Management Command Portal 瀵硅薄銆?
 - 1 涓?DPBP - 涓€涓?Buffer Pool 鐢ㄤ簬涓烘帶鍒舵帴鍙ｄ笂 Rx 璺緞鍑嗗鐨勭紦鍐插尯鎾銆?
 - 闇€瑕佽闂嚦灏戜竴涓?DPIO 瀵硅薄锛圫oftware Portal锛夋墠鑳藉鎺у埗鎺ュ彛闃熷垪鎵ц浠讳綍鍏ラ槦/鍑洪槦鎿嶄綔銆侱PIO 瀵硅薄灏嗚鍏变韩锛屾棤闇€绉佹湁鐨勩€?
## 浜ゆ崲鐗规€?

椹卞姩鏀寔鍦ㄧ‖浠朵腑閰嶇疆 L2 杞彂瑙勫垯锛岀敤浜庣鍙ｆˉ鎺ヤ互鍙婄嫭绔嬩氦鎹㈡満鎺ュ彛鐨勭嫭绔嬩娇鐢ㄣ€?
纭欢鍦?VLAN 鎰熺煡鏂归潰涓嶅彲閰嶇疆锛屽洜姝や换浣?DPAA2
```

        $ ip link add dev br0 type bridge vlan_filtering 1

        $ ip link add dev br1 type bridge
        $ ip link set dev ethX master br1
        Error: fsl_dpaa2_switch: Cannot join a VLAN-unaware bridge

```
褰撹缃?`stp_state 1` 鏃讹紝鏀寔閫氳繃 STP 杩涜鎷撴墤鍜岀幆璺娴?```

        $ ip link add dev br0 type bridge vlan_filtering 1 stp_state 1

```
鏀寔 L2 FDB 鎿嶄綔锛堟坊鍔?鍒犻櫎/杞偍锛夈€?
鍙互閫氳繃缃戞ˉ鍛戒护鍦ㄦ瘡涓氦鎹㈡満绔彛涓婄嫭绔嬮厤缃?HW FDB 瀛︿範銆傚綋绂佺敤 HW 瀛︿範鏃讹紝浼氳繍琛屼竴涓揩閫熻€佸寲锛坒ast age锛夎繃绋嬶紝浠讳綍鍏堝墠瀛︿範鍒扮殑鍦板潃閮戒細琚Щ闄ゃ€?```

        $ bridge link set dev ethX learning off
        $ bridge link set dev ethX learning on

```
鏀寔闄愬埗鏈煡鍗曟挱鍜岀粍鎾椽娉涘煙锛屼絾
```

        $ ip link set dev ethX type bridge_slave flood off mcast_flood off
        $ ip link set dev ethX type bridge_slave flood off mcast_flood on
        Error: fsl_dpaa2_switch: Cannot configure multicast flooding independently of unicast.

```
```

        $ echo 0 > /sys/bus/fsl-mc/devices/dpsw.Y/net/ethX/brport/broadcast_flood

```
## 鍗歌浇锛圤ffloads锛?

### 璺敱鍔ㄤ綔锛堥噸瀹氬悜銆乼rap銆乨rop锛?

DPAA2 浜ゆ崲鏈鸿兘澶熷埄鐢?ACL 琛ㄥ嵏杞藉熀浜庢祦鐨勫寘閲嶅畾鍚戙€傞€氳繃鍦ㄥ涓鍙ｉ棿鍏变韩鍗曚釜 ACL 琛ㄦ潵鏀寔鍏变韩杩囨护鍧椼€?
鏀寔浠ヤ笅娴佸叧閿瓧锛?
 - Ethernet锛歞st_mac/src_mac
 - IPv4锛歞st_ip/src_ip/ip_proto/tos
 - VLAN锛歷lan_id/vlan_prio/vlan_tpid/vlan_dei
 - L4锛歞st_port/src_port

姝ゅ锛宮atchall 杩囨护鍣ㄥ彲鐢ㄤ簬閲嶅畾鍚戠鍙ｄ笂鎺ユ敹鍒扮殑鍏ㄩ儴娴侀噺銆?
灏辨祦鍔ㄤ綔鑰岃█锛屾敮鎸佷互涓嬪姩浣滐細

 - drop
 - mirred egress redirect
 - trap

姣忎釜 ACL 琛ㄩ」锛堣繃婊ゅ櫒锛夊彧鑳介厤缃墍鍒楀姩浣滀腑鐨勪竴涓€?
绀轰緥 1锛氭妸 eth4 涓婃帴鏀跺埌鐨勩€丼A 涓?00:01:02:03:04:05 鐨勫抚鍙戦€佸埌
```

        $ tc qdisc add dev eth4 clsact
        $ tc filter add dev eth4 ingress flower src_mac 00:01:02:03:04:05 skip_sw action trap

```
```

        $ tc filter add dev eth4 ingress protocol 802.1q flower skip_sw vlan_id 100 vlan_prio 3 action drop

```
```

        $ tc filter add dev eth4 ingress matchall action mirred egress redirect dev eth1

```
```

        $ tc qdisc add dev eth5 ingress_block 1 clsact
        $ tc qdisc add dev eth6 ingress_block 1 clsact
        $ tc filter add block 1 ingress flower dst_mac 00:01:02:03:04:04 skip_sw \
                action trap
        $ tc filter add block 1 ingress protocol ipv4 flower src_ip 192.168.1.1 skip_sw \
                action mirred egress redirect dev eth3

```
#### 闀滃儚锛圡irroring锛?

DPAA2 浜ゆ崲鏈轰粎鏀寔姣忕鍙ｉ暅鍍忓拰姣?VLAN 闀滃儚銆備篃鏀寔鍦ㄥ叡浜潡涓坊鍔犻暅鍍忚繃婊ゅ櫒銆?
褰撲娇鐢ㄥ甫鏈?802.1q 鍗忚鐨?tc-flower 鍒嗙被鍣ㄦ椂锛屽彧鎺ュ彈 鈥樷€檝lan_id鈥樷€?鍏抽敭瀛椼€傚熀浜庝换浣曞叾浠栧瓧娈电殑闀滃儚
```

        $ tc qdisc add dev eth8 ingress_block 1 clsact
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_prio 3 action mirred egress mirror dev eth6
        Error: fsl_dpaa2_switch: Only matching on VLAN ID supported.
        We have an error talking to the kernel

```
濡傛灉鍦ㄧ鍙ｄ笂璇锋眰浜嗘煇涓?VLAN 鐨勯暅鍍忚繃婊ゅ櫒锛屽垯璇?VLAN 蹇呴』宸插畨瑁呭湪鐩稿叧浜ゆ崲鏈虹鍙ｄ笂锛屽彲浠ヤ娇鐢?鈥樷€檅ridge鈥樷€?鎴?```

        $ tc qdisc add dev eth8 ingress_block 1 clsact
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6
        Error: VLAN must be installed on the switch port.
        We have an error talking to the kernel

        $ bridge vlan add vid 200 dev eth8
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6

        $ ip link add link eth8 name eth8.200 type vlan id 200
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6

```
姝ゅ锛屽簲娉ㄦ剰闀滃儚娴侀噺灏嗗彈鍒颁笌鍏朵粬浠讳綍娴侀噺鐩稿悓鐨勫嚭鍙ｉ檺鍒躲€傝繖鎰忓懗鐫€褰撻暅鍍忔暟鎹寘鍒拌揪闀滃儚绔彛鏃讹紝濡傛灉鍖呬腑鍙戠幇鐨?VLAN 鏈畨瑁呭湪璇ョ鍙ｄ笂锛屽畠灏嗚涓㈠純銆?
DPAA2 浜ゆ崲鏈哄彧鏀寔鍗曚竴闀滃儚鐩殑鍦帮紝鍥犳澶氫釜
```

        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 100 action mirred egress mirror dev eth7
        Error: fsl_dpaa2_switch: Multiple mirror ports not supported.
        We have an error talking to the kernel

```
