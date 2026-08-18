
## 鐢ㄤ簬 Synopsys(R) 浠ュお缃戞帶鍒跺櫒 "stmmac" 鐨?Linux 椹卞姩

浣滆€咃細Giuseppe Cavallaro <peppe.cavallaro@st.com>銆?Alexandre Torgue <alexandre.torgue@st.com>銆丣ose Abreu <joabreu@synopsys.com>

## 鐩綍

- In This Release锛堟湰鐗堟湰璇存槑锛?- Feature List锛堢壒鎬у垪琛級
- Kernel Configuration锛堝唴鏍搁厤缃級
- Command Line Parameters锛堝懡浠よ鍙傛暟锛?- Driver Information and Notes锛堥┍鍔ㄤ俊鎭笌璇存槑锛?- Debug Information锛堣皟璇曚俊鎭級
- Support锛堟敮鎸侊級

## 鏈増鏈鏄?
鏈枃浠舵弿杩颁簡鐢ㄤ簬鎵€鏈?Synopsys(R) 浠ュお缃戞帶鍒跺櫒鐨?stmmac Linux 椹卞姩銆?
鐩墠锛岃繖涓綉缁滆澶囬┍鍔ㄩ€傜敤浜庢墍鏈?STi 宓屽叆寮?MAC/GMAC锛堝嵆 7xxx/5xxx SoC锛夈€?SPEAr锛坅rm锛夈€丩oongson1B锛坢ips锛変互鍙?XILINX XC2V3000 FF1152AMT0221
D1215994A VIRTEX FPGA 鏉裤€係ynopsys Ethernet QoS 5.0 IPK 涔熷彈鏀寔銆?
寮€鍙戞椹卞姩鏃朵娇鐢ㄤ簡 DesignWare(R) Cores Ethernet MAC 10/100/1000 Universal
鐗堟湰 3.70a锛堝強鏇存棭鐗堟湰锛夊拰 DesignWare(R) Cores Ethernet Quality-of-Service
鐗堟湰 4.0锛堝強鏇撮珮鐗堟湰锛夛紝浠ュ強 DesignWare(R) Cores XGMAC - 10G Ethernet MAC 鍜?DesignWare(R) Cores Enterprise MAC - 100G Ethernet MAC銆?
姝ら┍鍔ㄥ悓鏃舵敮鎸?platform 鎬荤嚎鍜?PCI銆?
姝ら┍鍔ㄥ寘鍚浠ヤ笅 Synopsys(R) DesignWare(R) Cores 浠ュお缃戞帶鍒跺櫒浠ュ強瀵瑰簲鏈€灏忓拰
鏈€澶х増鏈殑鏀寔锛?
+-------------------------------+--------------+--------------+--------------+
| Controller Name               | Min. Version | Max. Version | Abbrev. Name |
+===============================+==============+==============+==============+
| Ethernet MAC Universal        | N/A          | 3.73a        | GMAC         |
+-------------------------------+--------------+--------------+--------------+
| Ethernet Quality-of-Service   | 4.00a        | N/A          | GMAC4+       |
+-------------------------------+--------------+--------------+--------------+
| XGMAC - 10G Ethernet MAC      | 2.10a        | N/A          | XGMAC2+      |
+-------------------------------+--------------+--------------+--------------+
| XLGMAC - 100G Ethernet MAC    | 2.00a        | N/A          | XLGMAC2+     |
+-------------------------------+--------------+--------------+--------------+

鏈夊叧纭欢瑕佹眰鐨勯棶棰橈紝璇峰弬鑰冮殢浣犵殑浠ュお缃戦€傞厤鍣ㄤ竴璧锋彁渚涚殑鏂囨。銆傚垪鍑虹殑鎵€鏈夌‖浠?瑕佹眰閮介€傜敤浜庡湪 Linux 涓嬩娇鐢ㄣ€?
## 鐗规€у垪琛?
姝ら┍鍔ㄦ彁渚涗互涓嬬壒鎬э細
 - GMII/MII/RGMII/SGMII/RMII/XGMII/XLGMII 鎺ュ彛
 - 鍗婂弻宸?/ 鍏ㄥ弻宸ユ搷浣? - 鑺傝兘浠ュお缃戯紙EEE锛? - IEEE 802.3x PAUSE 鍖咃紙娴佹帶鍒讹級
 - RMON/MIB 璁℃暟鍣? - IEEE 1588 鏃堕棿鎴筹紙PTP锛? - 姣忕鑴夊啿杈撳嚭锛圥PS锛? - MDIO Clause 22 / Clause 45 鎺ュ彛
 - MAC 鍥炵幆
 - ARP 鍗歌浇锛圤ffloading锛? - 鑷姩 CRC / PAD 鎻掑叆涓庢鏌? - 鎺ユ敹涓庡彂閫佹暟鎹寘鐨勬牎楠屽拰鍗歌浇
 - 鏍囧噯鎴栧法鍨嬶紙Jumbo锛変互澶綉鍖? - 婧愬湴鍧€鎻掑叆 / 鏇挎崲
 - VLAN TAG 鎻掑叆 / 鏇挎崲 / 鍒犻櫎 / 杩囨护锛圚ASH 鍜?PERFECT锛? - 鍙紪绋嬬殑 TX 鍜?RX 鐪嬮棬鐙椾笌鍚堝苟锛圕oalesce锛夎缃? - 鐩殑鍦板潃杩囨护锛圥ERFECT锛? - HASH 杩囨护锛堝鎾級
 - Layer 3 / Layer 4 杩囨护
 - 杩滅▼鍞ら啋妫€娴? - 鎺ユ敹渚х缉鏀撅紙RSS锛? - TX 鍜?RX 鐨勫抚鎶㈠崰锛團rame Preemption锛? - 鍙紪绋嬬獊鍙戦暱搴︺€侀槇鍊笺€侀槦鍒楀ぇ灏? - 澶氶槦鍒楋紙鏈€澶?8 涓級
 - 澶氱璋冨害绠楁硶锛圱X锛歐RR銆丏WRR銆乄FQ銆丼P銆丆BS銆丒ST銆乀BS锛汻X锛歐RR銆丼P锛? - 鐏垫椿鐨?RX 瑙ｆ瀽鍣? - TCP / UDP 鍒嗘鍗歌浇锛圱SO銆乁SO锛? - 鍒嗗壊澶撮儴锛圫PH锛? - 瀹夊叏鐗规€э紙ECC 淇濇姢銆佹暟鎹鍋朵繚鎶わ級
 - 浣跨敤 Ethtool 鐨勮嚜妫€娴嬭瘯

## 鍐呮牳閰嶇疆

鍐呮牳閰嶇疆閫夐」鏄?`CONFIG_STMMAC_ETH`锛? - `CONFIG_STMMAC_PLATFORM`锛氱敤浜庡惎鐢?platform 椹卞姩銆? - `CONFIG_STMMAC_PCI`锛氱敤浜庡惎鐢?pci 椹卞姩銆?
## 鍛戒护琛屽弬鏁?
濡傛灉椹卞姩琚瀯寤轰负妯″潡锛屽彲浠ヤ娇鐢ㄤ互涓嬪彲閫夊弬鏁帮紝閫氳繃 modprobe 鍛戒护鎶婂畠浠緭鍏ュ埌
鍛戒护琛屼腑锛屼娇鐢ㄥ涓嬪舰寮?```

    modprobe stmmac_pci [<option>=<VAL1>,<VAL2>,...]

```
```

    stmmaceth=watchdog:100,chain_mode=1

```
姣忎釜鍙傛暟鐨勯粯璁ゅ€奸€氬父灏辨槸鎺ㄨ崘璁剧疆锛岄櫎闈炲彟鏈夎鏄庛€?
### watchdog

:Valid Range: 5000-None
:Default Value: 5000

姝ゅ弬鏁拌鐩栦互姣涓哄崟浣嶇殑鍙戦€佽秴鏃躲€?
### debug

:Valid Range: 0-16 (0=none,...,16=all)
:Default Value: 0

姝ゅ弬鏁拌皟鏁存樉绀哄湪绯荤粺鏃ュ織涓殑璋冭瘯娑堟伅绾у埆銆?
### phyaddr

:Valid Range: 0-31
:Default Value: -1

姝ゅ弬鏁拌鐩?PHY 璁惧鐨勭墿鐞嗗湴鍧€銆?
### flow_ctrl

:Valid Range: 0-3 (0=off,1=rx,2=tx,3=rx/tx)
:Default Value: 3

姝ゅ弬鏁版敼鍙橀粯璁ょ殑娴佹帶鍒惰兘鍔涖€?
### pause

:Valid Range: 0-65535
:Default Value: 65535

姝ゅ弬鏁版敼鍙橀粯璁ょ殑娴佹帶鍒舵殏鍋滄椂闂淬€?
### tc

:Valid Range: 64-256
:Default Value: 64

姝ゅ弬鏁版敼鍙橀粯璁ょ殑 HW FIFO 闃堝€兼帶鍒跺€笺€?
### buf_sz

:Valid Range: 1536-16384
:Default Value: 1536

姝ゅ弬鏁版敼鍙橀粯璁ょ殑 RX DMA 鍖呯紦鍐插尯澶у皬銆?
### eee_timer

:Valid Range: 0-None
:Default Value: 1000

姝ゅ弬鏁版敼鍙橀粯璁ょ殑 LPI TX 杩囨湡鏃堕棿锛堟绉掞級銆?
### chain_mode

:Valid Range: 0-1 (0=off,1=on)
:Default Value: 0

姝ゅ弬鏁版妸榛樿鐨勬搷浣滄ā寮忎粠 Ring 妯″紡鏀逛负 Chain 妯″紡銆?
## 椹卞姩淇℃伅涓庤鏄?
### 鍙戦€佽繃绋?
褰撳唴鏍搁渶瑕佸彂閫佷竴涓寘鏃讹紝浼氳皟鐢?xmit 鏂规硶锛涘畠璁剧疆鐜紙ring锛変腑鐨勬弿杩扮锛屽苟
閫氱煡 DMA 寮曟搸鏈変竴涓寘宸插噯澶囧ソ鍙戦€併€?
榛樿鎯呭喌涓嬶紝椹卞姩鍦?`net_device` 缁撴瀯鐨?features 瀛楁涓缃?`NETIF_F_SG` 浣嶏紝
浠庤€屽惎鐢ㄦ暎鑱氾紙scatter-gather锛夌壒鎬с€傚湪閭ｄ簺鏍￠獙鍜屽彲浠ュ湪纭欢涓畬鎴愮殑鑺墖鍜?閰嶇疆涓婃槸濡傛銆?
涓€鏃︽帶鍒跺櫒瀹屾垚鍖呯殑鍙戦€侊紝灏变細璋冨害涓€涓畾鏃跺櫒鏉ラ噴鏀惧彂閫佽祫婧愩€?
### 鎺ユ敹杩囩▼

褰撲竴涓垨澶氫釜鍖呰鏀跺埌鏃讹紝浼氬彂鐢熶竴娆′腑鏂€備腑鏂笉浼氳鎺掗槦锛屽洜姝ら┍鍔ㄥ湪鎺ユ敹杩囩▼涓?蹇呴』鎵弿鐜腑鐨勬墍鏈夋弿杩扮銆?
杩欏熀浜?NAPI锛屽洜姝や腑鏂鐞嗙▼搴忓彧鍦ㄦ湁宸ヤ綔瑕佸仛鏃舵墠鍙戝嚭淇″彿锛岀劧鍚庨€€鍑恒€傛帴鐫€ poll
鏂规硶浼氬湪灏嗘潵鐨勬煇涓椂鍒昏璋冨害銆?
DMA 鎶婃敹鍒扮殑鍖呭瓨鍌ㄥ湪棰勫厛鍒嗛厤鐨?socket 缂撳啿鍖哄垪琛ㄤ腑锛屼互閬垮厤 memcpy锛堥浂鎷疯礉锛夈€?
### 涓柇缂撹В

椹卞姩鑳藉浣跨敤 NAPI 鏉ョ紦鍑忥紙mitigate锛夊叾 DMA 涓柇鐨勬暟閲忥紝鐢ㄤ簬 3.50 涔嬪墠鑺墖鐨?鎺ユ敹銆傛柊鑺墖鏈変竴涓敤浜庤繖绉嶇紦鍑忕殑 HW RX 鐪嬮棬鐙椼€?
缂撳噺鍙傛暟鍙互閫氳繃 ethtool 璋冩暣銆?
### WoL

閫氳繃 Magic 甯у拰 Unicast 甯у疄鐜扮殑灞€鍩熺綉鍞ら啋锛圵ake up on Lan锛夌壒鎬э紝鍦?GMAC銆?GMAC4/5 鍜?XGMAC 鏍稿績涓婂彈鏀寔銆?
### DMA 鎻忚堪绗?
椹卞姩澶勭悊鏅€氭弿杩扮鍜屽鐢ㄦ弿杩扮銆傚悗鑰呬粎鍦?DesignWare(R) Cores Ethernet MAC
Universal 鐗堟湰 3.41a 鍙婁互鍚庣殑鐗堟湰涓婅娴嬭瘯杩囥€?
stmmac 鏀寔 DMA 鎻忚堪绗﹀湪鍙岀紦鍐诧紙RING锛夊拰閾捐〃锛圕HAINED锛変袱绉嶆ā寮忎笅鎿嶄綔銆傚湪
RING 妯″紡涓嬶紝姣忎釜鎻忚堪绗︽寚鍚戜袱涓暟鎹紦鍐插尯鎸囬拡锛岃€屽湪 CHAINED 妯″紡涓嬪畠浠彧鎸囧悜
涓€涓暟鎹紦鍐插尯鎸囬拡銆俁ING 妯″紡鏄粯璁ゆā寮忋€?
鍦?CHAINED 妯″紡涓嬶紝姣忎釜鎻忚堪绗︿細鏈変竴涓寚鍚戜笅涓€涓弿杩扮鐨勬寚閽堬紝浠庤€屽湪鎻忚堪绗?鑷韩涓垱寤烘樉寮忕殑閾炬帴锛涜€岃繖鏍风殑鏄惧紡閾炬帴鍦?RING 妯″紡涓嬫槸涓嶅彲鑳界殑銆?
### 鎵╁睍鎻忚堪绗?
鎵╁睍鎻忚堪绗﹀湪鎵胯浇 PTP 鍖呮垨 IP 涓婄殑 TCP/UDP/ICMP 鏃讹紝缁欐垜浠彁渚涘叧浜庝互澶綉杞借嵎
鐨勪俊鎭€傚湪鏃╀簬 3.50 鐨?GMAC Synopsys(R) 鑺墖涓婅繖浜涗笉鍙敤銆傚湪鎺㈡祴锛坧robe锛夋椂
椹卞姩浼氬喅瀹氭槸鍚﹀彲浠ョ湡姝ｄ娇鐢ㄥ畠浠€傝繖涓€鏀寔瀵逛簬 PTPv2 涔熸槸寮哄埗闇€瑕佺殑锛屽洜涓洪澶?鐨勬弿杩扮琚敤鏉ヤ繚瀛樼‖浠舵椂闂存埑鍜屾墿灞曠姸鎬併€?
### Ethtool 鏀寔

鏀寔 Ethtool銆備緥濡傦紝椹卞姩缁熻淇℃伅锛堝寘鎷?RMON锛夛細
```

    ethtool -S ethX

```
Ethtool 鑷娴嬭瘯涔熷彈鏀寔銆傝繖鍏佽鍋氫竴浜涙棭鏈熺殑鍋ュ叏鎬ф鏌?```

    ethtool -t ethX

```
### 宸ㄥ瀷甯т笌鍒嗘鍗歌浇

宸ㄥ瀷甯у彈鏀寔锛屽苟涓旈拡瀵?GMAC 娴嬭瘯杩囥€侴SO 涔熻鍔犲叆锛屼絾瀹冩槸浠ヨ蒋浠舵柟寮忔墽琛岀殑銆?LRO 涓嶅彈鏀寔銆?
### TSO 鏀寔

TSO锛圱CP 鍒嗘鍗歌浇锛夌壒鎬у彈 GMAC > 4.x 鍜?XGMAC 鑺墖绯诲垪鏀寔銆傚綋涓€涓寘閫氳繃 TCP
鍗忚鍙戦€佹椂锛孴CP 鏍堢‘淇濇彁渚涚粰搴曞眰椹卞姩锛堝湪鎴戜滑鐨勪緥瀛愪腑鏄?stmmac锛夌殑 SKB 涓庢渶澶?甯ч暱鐩稿尮閰嶏紙IP 澶?+ TCP 澶?+ 杞借嵎 <= 1500 瀛楄妭锛堝浜庤缃负 1500 鐨?MTU锛夛級銆傝繖
鎰忓懗鐫€锛屽鏋滀娇鐢?TCP 鐨勫簲鐢ㄧ▼搴忔兂瑕佸彂閫佷竴涓湪鍔犱笂澶撮儴涔嬪悗闀垮害 > 1514 鐨勫寘锛岃
鍖呭皢琚媶鍒嗘垚澶氫釜 TCP 鍖咃細鏁版嵁杞借嵎琚媶鍒嗭紝鑰屽ご閮紙TCP/IP ..锛夎娣诲姞銆傝繖鏄敱杞欢
瀹屾垚鐨勩€?
褰撳惎鐢?TSO 鏃讹紝TCP 鏍堜笉鍏冲績鏈€澶у抚闀匡紝鑰屾槸鍘熸牱鎶?SKB 鍖呮彁渚涚粰 stmmac銆侴MAC IP
灏嗕笉寰椾笉鑷繁鎵ц鍒嗘锛屼互鍖归厤鏈€澶у抚闀裤€?
杩欎釜鐗规€у彲浠ラ€氳繃璁惧鏍戜腑鐨?`snps,tso` 椤规潵鍚敤銆?
### 鑺傝兘浠ュお缃?
鑺傝兘浠ュお缃戯紙EEE锛変娇 IEEE 802.3 MAC 瀛愬眰杩炲悓涓€绯诲垪鐗╃悊灞備竴璧峰湪浣庡姛鑰楃┖闂诧紙LPI锛?妯″紡涓嬭繍琛屻€侲EE 妯″紡鏀寔 IEEE 802.3 MAC 鍦?100Mbps銆?000Mbps 鍜?1Gbps 涓嬬殑
鎿嶄綔銆?
LPI 妯″紡閫氳繃鍦ㄦ病鏈夋暟鎹鏀跺彂鏃跺叧闂€氫俊璁惧鍔熻兘鐨勪竴閮ㄥ垎鏉ヨ妭鐪佸姛鑰椼€傞摼璺袱绔殑
绯荤粺閮藉彲浠ョ鐢ㄦ煇浜涘姛鑳斤紝骞跺湪浣庨摼璺埄鐢ㄧ巼鏈熼棿鑺傜渷鍔熻€椼€侻AC 鎺у埗鐫€绯荤粺鏄惁搴旇
杩涘叆鎴栭€€鍑?LPI 妯″紡锛屽苟鎶婅繖涓€鐐归€氱煡缁?PHY銆?
涓€鏃︽帴鍙ｈ鎵撳紑锛岄┍鍔ㄥ氨楠岃瘉 EEE 鏄惁鍙互琚敮鎸併€傝繖鏄€氳繃鏌ョ湅 DMA HW 鑳藉姏瀵勫瓨鍣?鍜?PHY 璁惧鐨?MCD 瀵勫瓨鍣ㄦ潵瀹屾垚鐨勩€?
涓轰簡杩涘叆 TX LPI 妯″紡锛岄┍鍔ㄩ渶瑕佷竴涓蒋浠跺畾鏃跺櫒锛屽湪娌℃湁涓滆タ瑕佸彂閫佹椂鍚敤鍜岀鐢?LPI 妯″紡銆?
### 绮剧‘鏃堕棿鍗忚锛圥TP锛?
椹卞姩鏀寔 IEEE 1588-2002 绮剧‘鏃堕棿鍗忚锛圥TP锛夛紝瀹冧娇寰楀湪浣跨敤璇稿缃戠粶閫氫俊绛夋妧鏈?瀹炵幇鐨勬祴閲忓拰鎺у埗绯荤粺涓紝鏃堕挓鑳藉琚簿纭悓姝ャ€?
闄や簡 IEEE 1588-2002 鏃堕棿鎴充腑鎻愬埌鐨勯偅浜涘熀纭€鏃堕棿鎴崇壒鎬у锛屾柊鐨?GMAC 鏍稿績鏀寔
楂樼骇鏃堕棿鎴崇壒鎬с€侷EEE 1588-2008 鍙互鍦ㄩ厤缃唴鏍告椂鍚敤銆?
### SGMII/RGMII 鏀寔

鏂扮殑 GMAC 璁惧鎻愪緵浜嗚嚜宸辩殑鏂瑰紡鏉ョ鐞嗙殑 RGMII/SGMII銆傝繖涓€淇℃伅鍦ㄨ繍琛屾椂閫氳繃鏌ョ湅
HW 鑳藉姏瀵勫瓨鍣ㄥ氨鍙互鑾峰緱銆傝繖鎰忓懗鐫€ stmmac 鍙互鏃犻渶浣跨敤 PHYLIB 鐨勯偅濂楁満鍒讹紝灏辫兘
绠＄悊鑷崗鍟嗗拰閾捐矾鐘舵€併€傚疄闄呬笂锛孒W 鎻愪緵浜嗕竴缁勬墿灞曞瘎瀛樺櫒鏉ラ噸鏂板惎鍔?ANE銆侀獙璇?鍏?鍗婂弻宸ユā寮忓拰閫熷害銆傚浜忎簡杩欎簺瀵勫瓨鍣紝鎵嶅緱浠ユ煡鐪嬭嚜鍗忓晢鐨勯摼璺紮浼磋兘鍔涖€?
### 鐗╃悊灞?
椹卞姩涓庣墿鐞嗘娊璞″眰锛圥hysical Abstraction Layer锛夊吋瀹癸紝浠ヨ繛鎺ュ埌 PHY 鍜?GPHY 璁惧銆?
### 骞冲彴淇℃伅

涓€浜涗俊鎭彲浠ラ€氳繃 platform 鍜岃澶囨爲浼犻€掋€?
```

    struct plat_stmmacenet_data {

```
```
        int bus_id;

```
2) PHY 鐗╃悊鍦板潃銆傚鏋滆涓?-1锛岄┍鍔ㄥ皢閫夋嫨瀹冩壘鍒扮殑绗竴涓?PHY
```
        int phy_addr;

```
```
        int interface;

```
```
        struct stmmac_mdio_bus_data *mdio_bus_data;

```
```
        struct stmmac_dma_cfg *dma_cfg;

```
```
        int clk_csr;

```
```
        int has_gmac;

```
```
        int enh_desc;

```
```
        int tx_coe;
        int rx_coe;

```
11) 涓€浜?HW 鐢变簬缂撳啿鍖哄ぇ灏忔湁闄愶紝鏃犳硶涓鸿秴澶у抚鍦?HW 涓墽琛?csum銆傝缃鏍囧織鍚庯紝
csum 灏嗗湪 SW 涓墽琛?```
        int bugged_jumbo;

```
```
        int pmt;

```
```
        int force_sf_dma_mode;
        int force_thresh_dma_mode;

```
```
        int riwt_off;

```
```
        int max_speed;
        int maxmtu;

```
```
        int multicast_filter_bins;
        int unicast_filter_entries;

```
```
        int tx_fifo_size;
        int rx_fifo_size;

```
```
        u32 rx_queues_to_use;
        u32 tx_queues_to_use;

```
```
        u8 rx_sched_algorithm;
        u8 tx_sched_algorithm;

```
```
        struct stmmac_rxq_cfg rx_queues_cfg[MTL_MAX_RX_QUEUES];
        struct stmmac_txq_cfg tx_queues_cfg[MTL_MAX_TX_QUEUES];

```
24) 姝ゅ洖璋冪敤浜庝慨鏀规煇浜?syscfg 瀵勫瓨鍣紙鍦?ST SoC 涓婏級
```
        void (*fix_mac_speed)(void *priv, unsigned int speed);

```
25) 鐢ㄤ簬璋冪敤鑷畾涔夊垵濮嬪寲鐨勫洖璋冿紱鍦ㄦ煇浜涘钩鍙帮紙渚嬪 ST 鏈洪《鐩掞級涓婅繖鏈夋椂鏄繀瑕佺殑锛?杩欎簺骞冲彴鐨?HW 闇€瑕佽缃竴浜?PIO 绾挎垨绯荤粺 cfg 瀵勫瓨鍣ㄣ€俰nit/exit 鍥炶皟涓嶅簲浣跨敤
```
        int (*init)(struct platform_device *pdev, void *priv);
        void (*exit)(struct platform_device *pdev, void *priv);

```
26) 鎵ц鎬荤嚎鐨?HW 璁剧疆銆備緥濡傦紝鍦ㄦ煇浜?ST 骞冲彴涓婅繖涓瓧娈?```
        struct mac_device_info *(*setup)(void *priv);
        void *bsp_priv;

```
```
        struct clk *stmmac_clk;
        struct clk *pclk;
        struct clk *clk_ptp_ref;
        unsigned int clk_ptp_rate;
        unsigned int clk_ref_rate;
        s32 ptp_max_adj;

```
```
        struct reset_control *stmmac_rst;

```
```
        struct stmmac_axi *axi;

```
```
        int has_gmac4;

```
```
        bool has_sun8i;

```
```
        bool tso_en;

```
```
        int rss_en;

```
```
        int mac_port_sel_speed;

```
```
        bool en_tx_lpi_clockgating;

```
```
        int has_xgmac;

```
```
    }

```
For MDIO bus data, we have:

```
    struct stmmac_mdio_bus_data {

```
```
        unsigned int phy_mask;

```
```
        int *irqs;

```
```
        int probed_phy_irq;

```
```
        bool needs_reset;

```
```
    }

```
For DMA engine configuration, we have:

```
    struct stmmac_dma_cfg {

```
```
        int pbl;

```
```
        int txpbl;
        int rxpbl;

```
```
        bool pblx8;

```
```
        int fixed_burst;
        int mixed_burst;

```
```
        bool aal;

```
```
        bool eame;

```
```
    }

```
For DMA AXI parameters, we have:

```
    struct stmmac_axi {

```
```
        bool axi_lpi_en;
        bool axi_xit_frm;

```
```
        u32 axi_wr_osr_lmt;
        u32 axi_rd_osr_lmt;

```
```
        bool axi_kbbe;

```
```
        u32 axi_blen[AXI_BLEN];

```
```
        bool axi_fb;
        bool axi_mb;

```
```
        bool axi_rb;

```
```
    }

```
For the RX Queues configuration, we have:

```
    struct stmmac_rxq_cfg {

```
```
        u8 mode_to_use;

```
```
        u32 chan;

```
```
        u8 pkt_route;

```
```
        bool use_prio;
        u32 prio;

```
```
    }

```
For the TX Queues configuration, we have:

```
    struct stmmac_txq_cfg {

```
```
        u32 weight;

```
```
        u8 mode_to_use;

```
```
        u32 send_slope;
        u32 idle_slope;
        u32 high_credit;
        u32 low_credit;

```
```
        bool use_prio;
        u32 prio;

```
```
    }

```
### 璁惧鏍戜俊鎭?
璇峰弬鑰冧互涓嬫枃妗ｏ細
Documentation/devicetree/bindings/net/snps,dwmac.yaml

### HW 鑳藉姏

娉ㄦ剰锛屼粠鍙敤 HW 鑳藉姏瀵勫瓨鍣ㄧ殑鏂拌姱鐗囧紑濮嬶紝璁稿閰嶇疆鏄湪杩愯鏃跺彂鐜扮殑锛屼緥濡傜敤浜?浜嗚В EEE銆丠W csum銆丳TP銆佸寮烘弿杩扮绛夋槸鍚︾湡姝ｅ彲鐢ㄣ€備綔涓烘椹卞姩鎵€閲囩敤鐨勭瓥鐣ワ紝鏉ヨ嚜
HW 鑳藉姏瀵勫瓨鍣ㄧ殑淇℃伅鍙互鍙栦唬浠庡钩鍙颁紶鏉ョ殑淇℃伅銆?
## 璋冭瘯淇℃伅

椹卞姩瀵煎嚭浜嗚澶氫俊鎭紝渚嬪鍐呴儴缁熻銆佽皟璇曚俊鎭€丮AC 鍜?DMA 瀵勫瓨鍣ㄧ瓑銆?
鏍规嵁瀹為檯鎵€闇€淇℃伅鐨勭被鍨嬶紝鍙互閫氳繃澶氱鏂瑰紡璇诲彇杩欎簺淇℃伅銆?
渚嬪锛岀敤鎴峰彲浠ヤ娇鐢?ethtool 鏀寔鏉ヨ幏鍙栫粺璁′俊鎭細渚嬪浣跨敤 `ethtool -S ethX`
锛堝鏋滄敮鎸佸垯鏄剧ず绠＄悊璁℃暟鍣紙MMC锛夛級锛屾垨鑰呮煡鐪?MAC/DMA 瀵勫瓨鍣細渚嬪浣跨敤
`ethtool -d ethX`銆?
鐢?`CONFIG_DEBUG_FS` 缂栬瘧鍐呮牳锛岄┍鍔ㄥ皢瀵煎嚭浠ヤ笅 debugfs 椤癸細

 - `descriptors_status`锛氱敤浜庢樉绀?DMA TX/RX 鎻忚堪绗︾幆
 - `dma_cap`锛氱敤浜庢樉绀?HW 鑳藉姏

寮€鍙戣€呬篃鍙互浣跨敤 `debug` 妯″潡鍙傛暟鏉ヨ幏鍙栬繘涓€姝ョ殑璋冭瘯淇℃伅锛堣鍙傞槄锛歂ETIF Msg
Level锛夈€?
## 鏀寔

濡傛灉鍦ㄥ彈鏀寔鐨勫唴鏍镐笂銆佷娇鐢ㄥ彈鏀寔鐨勯€傞厤鍣ㄣ€佸湪宸插彂甯冪殑婧愪唬鐮佷腑鍙戠幇闂锛岃鎶?涓庤闂鐩稿叧鐨勫叿浣撲俊鎭€氳繃鐢靛瓙閭欢鍙戦€佸埌 netdev@vger.kernel.org
