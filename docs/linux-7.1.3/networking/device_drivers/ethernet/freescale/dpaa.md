
## QorIQ DPAA 浠ュお缃戦┍鍔?

浣滆€咃細
- Madalin Bucur <madalin.bucur@nxp.com>
- Camelia Groza <camelia.groza@nxp.com>


 - DPAA 浠ュお缃戞杩? - DPAA 浠ュお缃戞敮鎸佺殑 SoC
 - 鍦ㄤ綘鐨勫唴鏍镐腑閰嶇疆 DPAA 浠ュお缃? - DPAA 浠ュお缃戝抚澶勭悊
 - DPAA 浠ュお缃戠壒鎬? - DPAA 涓柇浜插拰鎬т笌鎺ユ敹绔缉鏀撅紙RSS锛? - 璋冭瘯

## DPAA 浠ュお缃戞杩?

DPAA 浠ｈ〃 Data Path Acceleration Architecture锛堟暟鎹€氳矾鍔犻€熸灦鏋勶級锛屽畠鏄竴缁勭綉缁滃姞閫?IP锛屽湪 PowerPC 鍜?ARM64 澶氫釜浠ｉ檯鐨?SoC 涓婇兘鍙敤銆?
Freescale 鐨?DPAA 鏋舵瀯鐢变竴绯诲垪鏀寔浠ュお缃戣繛鎺ョ殑纭欢妯″潡缁勬垚銆傝浠ュお缃戦┍鍔ㄤ緷璧栦簬
Linux 鍐呮牳涓殑浠ヤ笅椹卞姩锛?
 - 澶栬璁块棶瀛樺偍鍗曞厓锛圥AMU锛夛紙*浠?PPC 骞冲彴闇€瑕侊級
    drivers/iommu/fsl_*
 - 甯х鐞嗗櫒锛團Man锛?    drivers/net/ethernet/freescale/fman
 - 闃熷垪绠＄悊鍣紙QMan锛夈€佺紦鍐插尯绠＄悊鍣紙BMan锛?    drivers/soc/fsl/qbman

```

  dpaa_eth       /eth0\     ...       /ethN\
  driver        |      |             |      |
  -------------   ----   -----------   ----   -------------
       -Ports  / Tx  Rx \    ...    / Tx  Rx \
  FMan        |          |         |          |
       -MACs  |   MAC0   |         |   MACN   |
	     /   dtsec0   \  ...  /   dtsecN   \ (or tgec)
	    /              \     /              \(or memac)
  ---------  --------------  ---  --------------  ---------
      FMan, FMan Port, FMan SP, FMan MURAM drivers
  ---------------------------------------------------------
      FMan HW blocks: MURAM, MACs, Ports, SP
  ---------------------------------------------------------

```
```

	      ________________________________
  dpaa_eth   /            eth0                \
  driver    /                                  \
  ---------   -^-   -^-   -^-   ---    ---------
  QMan driver / \   / \   / \  \   /  | BMan    |
	     |Rx | |Rx | |Tx | |Tx |  | driver  |
  ---------  |Dfl| |Err| |Cnf| |FQs|  |         |
  QMan HW    |FQ | |FQ | |FQs| |   |  |         |
	     /   \ /   \ /   \  \ /   |         |
  ---------   ---   ---   ---   -v-    ---------
	    |        FMan QMI         |         |
	    | FMan HW       FMan BMI  | BMan HW |
	      -----------------------   --------

```
鍏朵腑涓婇潰锛堜互鍙婁唬鐮佷腑锛変娇鐢ㄧ殑缂╁啓涓猴細

=============== ===========================================================
DPAA 		Data Path Acceleration Architecture锛堟暟鎹€氳矾鍔犻€熸灦鏋勶級
FMan 		DPAA 甯х鐞嗗櫒
QMan 		DPAA 闃熷垪绠＄悊鍣?BMan 		DPAA 缂撳啿鍖虹鐞嗗櫒
QMI 		FMan 涓殑 QMan 鎺ュ彛
BMI 		FMan 涓殑 BMan 鎺ュ彛
FMan SP 	FMan 瀛樺偍閰嶇疆鏂囦欢
MURAM 		FMan 涓殑澶氱敤鎴?RAM
FQ 		QMan 甯ч槦鍒?Rx Dfl FQ 	榛樿鎺ユ敹 FQ
Rx Err FQ 	Rx 閿欒甯?FQ
Tx Cnf FQ 	Tx 纭 FQ
Tx FQs 		鍙戦€佸抚闃熷垪
dtsec 		datapath 涓夐€熶互澶綉鎺у埗鍣紙10/100/1000 Mbps锛?tgec 		鍗佸崈鍏嗕互澶綉鎺у埗鍣紙10 Gbps锛?memac 		澶氶€熺巼浠ュお缃?MAC锛?0/100/1000/10000锛?=============== ===========================================================

## DPAA 浠ュお缃戞敮鎸佺殑 SoC


DPAA 椹卞姩鍚敤浜嗕互涓?SoC 涓婂瓨鍦ㄧ殑浠ュお缃戞帶鍒跺櫒锛?
PPC
- P1023
- P2041
- P3041
- P4080
- P5020
- P5040
- T1023
- T1024
- T1040
- T1042
- T2080
- T4240
- B4860

ARM
- LS1043A
- LS1046A

## 鍦ㄤ綘鐨勫唴鏍镐腑閰嶇疆 DPAA 浠ュお缃?

```

  # arch/arm64 鍜?arch/powerpc 骞冲彴閫氱敤
  CONFIG_FSL_DPAA=y
  CONFIG_FSL_FMAN=y
  CONFIG_FSL_DPAA_ETH=y
  CONFIG_FSL_XGMAC_MDIO=y

  # 浠?arch/powerpc
  CONFIG_FSL_PAMU=y

  # RDB 涓婃墍鐢?PHY 闇€瑕佺殑閫氱敤閫夐」
  CONFIG_VITESSE_PHY=y
  CONFIG_REALTEK_PHY=y
  CONFIG_AQUANTIA_PHY=y

```
## DPAA 浠ュお缃戝抚澶勭悊


鍦ㄦ帴鏀讹紙Rx锛変晶锛屼紶鍏ュ抚鐨勭紦鍐插尯鏄粠涓撶敤鎺ュ彛缂撳啿鍖烘睜涓殑缂撳啿鍖鸿幏鍙栫殑銆傞┍鍔ㄥ垵濮嬪寲骞?鐢ㄤ竴椤靛ぇ灏忕殑缂撳啿鍖哄～鍏呰繖浜涙睜銆?
鍦ㄥ彂閫侊紙Tx锛変晶锛屾墍鏈夎鍙戦€佺殑甯ч兘閫氳繃 Tx 纭甯ч槦鍒楄繑鍥炵粰椹卞姩銆傜劧鍚庨┍鍔ㄨ礋璐ｉ噴鏀捐繖浜?缂撳啿鍖恒€備负浜嗘纭湴鍋氬埌杩欎竴鐐癸紝鍦ㄥ彂閫佷箣鍓嶄細鍚戠紦鍐插尯娣诲姞涓€涓寚鍥?skb 鐨勫洖鎸囬拡銆傚綋缂撳啿
鍖哄湪纭 FQ 涓婅繑鍥炵粰椹卞姩鏃讹紝skb 灏辫兘琚纭秷璐广€?
## DPAA 浠ュお缃戠壒鎬?

鐩墠 DPAA 浠ュお缃戦┍鍔ㄥ惎鐢ㄤ簡 Linux 浠ュお缃戦┍鍔ㄦ墍闇€鐨勫熀鏈壒鎬с€傚楂樼骇鐗规€х殑鏀寔灏嗛€愭
娣诲姞銆?
璇ラ┍鍔ㄥ UDP 鍜?TCP 鍏锋湁 Rx 鍜?Tx 鏍￠獙鍜屽嵏杞姐€傜洰鍓?Rx 鏍￠獙鍜屽嵏杞界壒鎬ч粯璁ゅ惎鐢紝涓旀棤娉?閫氳繃 ethtool 鎺у埗銆傛澶栵紝杩樻坊鍔犱簡 rx-flow-hash 鍜?rx-hashing銆俁SS 鐨勫姞鍏ヤ负杞彂鍦烘櫙
甯︽潵浜嗗法澶х殑鎬ц兘鎻愬崌锛屽厑璁哥敱涓€涓帴鍙ｆ帴鏀剁殑涓嶅悓娴侀噺娴佽涓嶅悓鐨?CPU 骞惰澶勭悊銆?
璇ラ┍鍔ㄦ敮鎸佸涓甫浼樺厛绾х殑 Tx 娴侀噺绫诲埆銆備紭鍏堢骇鑼冨洿浠?0锛堟渶浣庯級鍒?3锛堟渶楂橈級銆傚畠浠
鏄犲皠鍒板叿鏈変弗鏍间紭鍏堢骇绾у埆鐨勭‖浠跺伐浣滈槦鍒椼€傛瘡涓祦閲忕被鍒寘鍚?NR_CPU 涓?Tx 闃熷垪銆傞粯璁?鎯呭喌涓嬶紝浠呭惎鐢ㄤ竴涓祦閲忕被鍒紝骞朵娇鐢ㄦ渶浣庝紭鍏堢骇鐨?Tx 闃熷垪銆傚彲浠ラ€氳繃 mqprio qdisc 鍚敤
鏇撮珮浼樺厛绾х殑娴侀噺绫诲埆銆備緥濡傦紝浣跨敤浠ヤ笅鍛戒护鍦ㄦ煇涓帴鍙ｄ笂鍚敤鍏ㄩ儴鍥涗釜娴侀噺绫诲埆銆傛澶栵紝
skb 浼樺厛绾х骇鍒埌娴侀噺绫诲埆鐨勬槧灏勫涓嬶細

 - 浼樺厛绾?0 鍒?3 - 娴侀噺绫诲埆 0锛堜綆浼樺厛绾э級
 - 浼樺厛绾?4 鍒?7 - 娴侀噺绫诲埆 1锛堜腑浣庝紭鍏堢骇锛? - 浼樺厛绾?8 鍒?11 - 娴侀噺绫诲埆 2锛堜腑楂樹紭鍏堢骇锛? - 浼樺厛绾?12 鍒?15 - 娴侀噺绫诲埆 3锛堥珮浼樺厛绾э級

```

  tc qdisc add dev <int> root handle 1: \
	 mqprio num_tc 4 map 0 0 0 0 1 1 1 1 2 2 2 2 3 3 3 3 hw 1

```
## DPAA 涓柇浜插拰鎬т笌鎺ユ敹绔缉鏀?

鍒拌揪 DPAA Rx 闃熷垪鎴?DPAA Tx 纭闃熷垪鐨勬祦閲忥紝鍦?CPU 鐪嬫潵鏄煇涓壒瀹?portal 涓婄殑鍏ュ彛
锛坕ngress锛夋祦閲忋€侱PAA QMan portal 涓柇鍚勮嚜浜插拰鍒版煇涓壒瀹?CPU銆傚悓涓€涓?portal 涓柇
鏈嶅姟浜庢墍鏈?QMan portal 娑堣垂鑰呫€?
榛樿鎯呭喌涓嬶紝DPAA 浠ュお缃戦┍鍔ㄥ惎鐢?RSS锛屽埄鐢?DPAA FMan 鐨?Parser 鍜?Keygen 妯″潡锛屽熀浜庢墍
鎺ユ敹甯т腑瀛樺湪鐨?IPv4/IPv6 婧愬拰鐩殑鍦板潃浠ュ強 L4 婧愬拰鐩殑绔彛鐨勫搱甯岋紝灏嗘祦閲忓垎甯冨埌 128 涓?纭欢甯ч槦鍒椾笂銆傚綋 RSS 琚鐢ㄦ椂锛屾煇涓壒瀹氭帴鍙ｆ帴鏀剁殑鎵€鏈夋祦閲忛兘鍦ㄩ粯璁?Rx 甯ч槦鍒椾笂鎺ユ敹銆?榛樿鐨?DPAA Rx 甯ч槦鍒楄閰嶇疆涓哄皢鎺ユ敹鍒扮殑娴侀噺鏀惧叆涓€涓睜閫氶亾锛坧ool channel锛夛紝鍏佽浠讳綍
鍙敤鐨?CPU portal 鍑洪槦璇ュ叆鍙ｆ祦閲忋€傞粯璁ゅ抚闃熷垪璁剧疆浜?HOLDACTIVE 閫夐」锛岀‘淇濇潵鑷煇涓槦鍒?鐨勬祦閲忕獊鍙戠敱鍚屼竴涓?CPU 鎻愪緵鏈嶅姟銆傝繖淇濊瘉浜嗘瀬浣庣殑甯т贡搴忕巼銆傚叾缂虹偣鏄紝鍦?RSS 鏈惎鐢ㄦ椂锛?鏌愪釜鐗瑰畾鎺ュ彛鎺ユ敹鍒扮殑娴侀噺涓€娆″彧鑳界敱涓€涓?CPU 鎻愪緵鏈嶅姟銆?
涓轰簡瀹炵幇 RSS锛孌PAA 浠ュお缃戦┍鍔ㄩ澶栧垎閰嶄竴缁?128 涓?Rx 甯ч槦鍒楋紝杩欎簺闃熷垪浠ヨ疆璇㈡柟寮忛厤缃埌
涓撶敤閫氶亾銆傚抚闃熷垪鍒?CPU 鐨勬槧灏勭幇鍦ㄦ槸纭紪鐮佺殑锛屾病鏈夐棿鎺ヨ〃鏉ュ皢鏌愪釜 FQ锛堝搱甯岀粨鏋滐級鐨勬祦閲?绉诲姩鍒板彟涓€涓?CPU銆傚埌杈捐繖浜涘抚闃熷垪涔嬩竴鐨勫叆鍙ｆ祦閲忓皢鍒拌揪鍚屼竴涓?portal锛屽苟鎬绘槸鐢卞悓涓€涓?CPU
澶勭悊銆傝繖淇濊瘉浜嗘祦鍐呴『搴忕殑淇濇寔浠ュ強澶氫釜娴侀噺娴佷箣闂寸殑宸ヤ綔璐熻浇鍒嗗竷銆?
```

	# ethtool -N fm1-mac9 rx-flow-hash tcp4 ""

```
```

	# ethtool -N fm1-mac9 rx-flow-hash udp4 sfdn

```
鏃犳硶瀵瑰悇涓崗璁繘琛岀嫭绔嬫帶鍒讹紝閽堝 tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6
涓换鎰忎竴涓繍琛岀殑鍛戒护锛岄兘浼氭帶鍒惰鎺ュ彛涓婃墍鏈夊崗璁殑 rx-flow-hashing銆?
闄や簡浣跨敤 FMan Keygen 璁＄畻鐨勫搱甯屽皢娴侀噺鍒嗘暎鍒?128 涓?Rx FQ 涔嬪锛孌PAA 浠ュお缃戦┍鍔ㄨ繕浼氬湪
NETIF_F_RXHASH 鐗规€у紑鍚紙榛樿婵€娲伙級鏃惰缃?skb 鍝堝笇鍊笺€傝繖鍙互閫氳繃浠ヤ笅鏂瑰紡鍏抽棴
```

	# ethtool -K fm1-mac9 rx-hashing off
	# ethtool -k fm1-mac9 | grep hash
	receive-hashing: off
	# ethtool -K fm1-mac9 rx-hashing on
	Actual changes:
	receive-hashing: on
	# ethtool -k fm1-mac9 | grep hash
	receive-hashing: on

```
璇锋敞鎰忥紝Rx 鍝堝笇渚濊禆浜庤鎺ュ彛鐨?rx-flow-hashing 澶勪簬寮€鍚姸鎬佲€斺€斿叧闂?rx-flow-hashing 涔?浼氱鐢?rx-hashing锛坋thtool 涓嶄細灏嗗叾鎶ュ憡涓?off锛屽洜涓鸿繖鍙栧喅浜?NETIF_F_RXHASH 鐗规€ф爣蹇楋級銆?
## 璋冭瘯


浠ヤ笅缁熻淇℃伅閫氳繃 ethtool 涓烘瘡涓帴鍙ｅ鍑猴細

 - 姣忎釜 CPU 鐨勪腑鏂鏁? - 姣忎釜 CPU 鐨?Rx 鏁版嵁鍖呰鏁? - 姣忎釜 CPU 鐨?Tx 鏁版嵁鍖呰鏁? - 姣忎釜 CPU 鐨?Tx 纭鏁版嵁鍖呰鏁? - 姣忎釜 CPU 鐨?Tx S/G 甯ц鏁? - 姣忎釜 CPU 鐨?Tx 閿欒璁℃暟
 - 姣忎釜 CPU 鐨?Rx 閿欒璁℃暟
 - 姣忎釜绫诲瀷鐨?Rx 閿欒璁℃暟
 - 涓庢嫢濉炵浉鍏崇殑缁熻锛?
  - 鎷ュ鐘舵€?  - 澶勪簬鎷ュ鐘舵€佺殑鏃堕棿
  - 璁惧杩涘叆鎷ュ鐘舵€佺殑娆℃暟
  - 鎸夊師鍥犵殑涓㈠寘璁℃暟

璇ラ┍鍔ㄨ繕浼氬湪 sysfs 涓鍑轰互涓嬩俊鎭細

 - 姣忕 FQ 绫诲瀷鐨?FQ ID
	  /sys/devices/platform/soc/<addr>.fman/<addr>.ethernet/dpaa-ethernet.<id>/net/fm<nr>-mac<nr>/fqids

 - 鎵€鐢ㄧ紦鍐插尯姹犵殑 ID
	  /sys/devices/platform/soc/<addr>.fman/<addr>.ethernet/dpaa-ethernet.<id>/net/fm<nr>-mac<nr>/bpids
