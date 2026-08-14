
## 闈㈠悜 Qualcomm IPQ SoC 绯诲垪鐨?PPE 浠ュお缃戦┍鍔?


鐗堟潈鎵€鏈?(c) Qualcomm Technologies, Inc. 鍙婂叾瀛愬叕鍙搞€?

浣滆€咃細Lei Wei <quic_leiwei@quicinc.com>


## 鐩綍


- `PPE Overview`_
- `PPE Driver Overview`_
- `PPE Driver Supported SoCs`_
- `Enabling the Driver`_
- `Debugging`_


## PPE 姒傝堪


IPQ锛圦ualcomm Internet Processor锛塖oC锛圫ystem-on-Chip锛岀郴缁熺骇鑺墖锛夌郴鍒楁槸 Qualcomm 闈㈠悜
Wi-Fi 鎺ュ叆鐐圭殑缃戠粶 SoC銆侾PE锛圥acket Process Engine锛屽寘澶勭悊寮曟搸锛夋槸 IPQ SoC 涓殑浠ュお缃?
鍖呭鐞嗗紩鎿庛€?
```
涓嬮潰鏄竴涓?IPQ9574 SoC 鐨勭畝鍖栫‖浠舵鍥撅紝鍏朵腑鍖呭惈 PPE 寮曟搸浠ュ強
SoC 涓絾浣嶄簬 PPE 寮曟搸涔嬪鐨勫叾浠栨ā鍧椼€傝繖浜涙ā鍧楀崗鍚屽伐浣?
               |netdev| |netdev| |netdev| |netdev| |netdev|  |netdev|<------|PHYLINK|
               +------+ +------+ +------+ +------+ +------+  +------+ stop  +-+-+-+-+
                                             |                                | | ^
 +-------+     +-------------------------+--------+----------------------+    | | |
 | GCC   |     |                         |  EDMA  |                      |    | | |
 +---+---+     |  PPE                    +---+----+                      |    | | |
     | clk     |                             |                           |    | | |
     +-------->| +-----------------------+------+-----+---------------+  |    | | |
               | |   Switch Core         |Port0 |     |Port7(EIP FIFO)|  |    | | |
               | |                       +---+--+     +------+--------+  |    | | |
               | |                           |               |        |  |    | | |
 +-------+     | |                    +------+---------------+----+   |  |    | | |
 |CMN PLL|     | | +---+ +---+ +----+ | +--------+                |   |  |    | | |
 +---+---+     | | |BM | |QM | |SCH | | | L2/L3  |  .......       |   |  |    | | |
 |   |         | | +---+ +---+ +----+ | +--------+                |   |  |    | | |
 |   |         | |                    +------+--------------------+   |  |    | | |
 |   |         | |                           |                        |  |    | | |
 |   v         | | +-----+-+-----+-+-----+-+-+---+--+-----+-+-----+   |  |    | | |
 | +------+    | | |Port1| |Port2| |Port3| |Port4|  |Port5| |Port6|   |  | mac| | |
 | |NSSCC |    | | +-----+ +-----+ +-----+ +-----+  +-----+ +-----+   |  |<---+ | |
 | +-+-+--+    | | |MAC0 | |MAC1 | |MAC2 | |MAC3 |  |MAC4 | |MAC5 |   |  | ops  | |
 | ^ | |clk    | | +-----+-+-----+-+-----+-+-----+--+-----+-+-----+   |  |      | |
 | | | +------>| +----|------|-------|-------|---------|--------|-----+  |      | |
 | | |         +---------------------------------------------------------+      | |
 | | |                |      |       |       |         |        |               | |
 | | |   MII clk      |      QSGMII               USXGMII   USXGMII             | |
 | | +--------------->|      |       |       |         |        |               | |
 | |                +-------------------------+ +---------+ +---------+         | |
 | |125/312.5MHz clk|       (PCS0)            | | (PCS1)  | | (PCS2)  | pcs ops | |
 | +----------------+       UNIPHY0           | | UNIPHY1 | | UNIPHY2 |<--------+ |
 +----------------->|                         | |         | |         |           |
 | 31.25MHz ref clk +-------------------------+ +---------+ +---------+           |
 |                     |     |      |      |          |          |                |
 |                +-----------------------------------------------------+         |
 |25/50MHz ref clk| +-------------------------+    +------+   +------+  | link    |
 +--------------->| |      QUAD PHY           |    | PHY4 |   | PHY5 |  |---------+
                  | +-------------------------+    +------+   +------+  | change
                  |                                                     |
                  |                       MDIO bus                      |
                  +-----------------------------------------------------+

```
CMN锛圕ommon锛岄€氱敤锛塒LL銆丯SSCC锛圢etworking Sub System Clock Controller锛岀綉缁滃瓙绯荤粺鏃堕挓鎺у埗鍣級鍜?GCC锛圙lobal Clock Controller锛屽叏灞€鏃堕挓鎺у埗鍣級妯″潡浣嶄簬 SoC 涓紝鍏呭綋鏃堕挓鎻愪緵鑰呫€?

UNIPHY 妯″潡浣嶄簬 SoC 涓紝鎻愪緵 PCS锛圥hysical Coding Sublayer锛岀墿鐞嗙紪鐮佸瓙灞傦級鍜?XPCS锛?0-Gigabit Physical Coding Sublayer锛?0 鍗冨厗鐗╃悊缂栫爜瀛愬眰锛夊姛鑳斤紝浠ユ敮鎸?PPE MAC 涓庡閮?PHY 涔嬮棿鐨勪笉鍚屾帴鍙ｆā寮忋€?
CMN锛圕ommon锛塒LL銆丯SSCC锛圢etworking Sub System Clock Controller锛岀綉缁滃瓙绯荤粺鏃堕挓鎺у埗鍣級鍜?GCC锛圙lobal
Clock Controller锛屽叏灞€鏃堕挓鎺у埗鍣級妯″潡浣嶄簬 SoC 涓紝鍏呭綋鏃堕挓鎻愪緵鑰呫€?

UNIPHY 妯″潡浣嶄簬 SoC 涓紝鎻愪緵 PCS锛圥hysical Coding Sublayer锛岀墿鐞嗙紪鐮佸瓙灞傦級鍜?
XPCS锛?0-Gigabit Physical Coding Sublayer锛?0 鍗冨厗鐗╃悊缂栫爜瀛愬眰锛夊姛鑳斤紝浠ユ敮鎸?PPE MAC 涓庡閮?PHY 涔嬮棿鐨?
涓嶅悓鎺ュ彛妯″紡銆?

鏈枃妗ｉ噸鐐规弿杩?PPE 寮曟搸涓?PPE 椹卞姩銆?

PPE锛圥acket Process Engine锛変腑鐨勪互澶綉鍔熻兘鐢变笁涓?
閮ㄥ垎缁勬垚锛氫氦鎹㈡牳蹇冿紙switch core锛夈€佺鍙ｅ皝瑁咃紙port wrapper锛夊拰浠ュお缃?DMA銆?
涓嬮潰鍒楀嚭灏嗙敱鏈?PPE 椹卞姩椹卞姩鐨勪富瑕佹ā鍧楋細
IPQ9574 PPE 涓殑浜ゆ崲鏍稿績鏈€澶氬叿鏈?6 涓墠闈㈡澘绔彛鍜?2 涓?FIFO
鎺ュ彛銆備袱涓?FIFO 鎺ュ彛涓殑涓€涓敤浜庝互澶綉绔彛涓庝富鏈?CPU 涔嬮棿鐨?
閫氫俊锛堜娇鐢ㄤ互澶綉 DMA锛夈€傚彟涓€涓敤浜庝笌
EIP 寮曟搸閫氫俊锛岃寮曟搸鐢ㄤ簬 IPsec 鍗歌浇銆傚湪 IPQ9574 涓婏紝PPE 鍖呭惈 6 涓?GMAC/XGMAC
锛屽彲涓庡閮ㄤ互澶綉 PHY 杩炴帴銆備氦鎹㈡牳蹇冭繕鍖呮嫭 BM锛圔uffer
Management锛岀紦鍐插尯绠＄悊锛夈€丵M锛圦ueue Management锛岄槦鍒楃鐞嗭級鍜?SCH锛圫cheduler锛岃皟搴﹀櫒锛夋ā鍧楋紝鐢ㄤ簬鏀寔
鏁版嵁鍖呭鐞嗐€?
- L2
绔彛灏佽鎻愪緵浠?6 涓?GMAC/XGMAC 鍒?UNIPHY锛圥CS锛夌殑杩炴帴锛屾敮鎸?
SGMII/QSGMII/PSGMII/USXGMII/10G-BASER 绛夊绉嶆ā寮忋€侷PQ9574 涓婃敮鎸?3 涓?UNIPHY锛圥CS锛?
瀹炰緥銆?
- EDMA锛圗thernet DMA锛屼互澶綉 DMA锛?
浠ュお缃?DMA 鐢ㄤ簬鍦ㄤ互澶綉瀛愮郴缁?
涓?ARM 涓绘満 CPU 涔嬮棿鏀跺彂鏁版嵁鍖呫€?
鍦?PPE MAC 绔彛涓婃帴鏀跺埌鐨勬暟鎹寘鍙互琚浆鍙戝埌鍙︿竴涓?PPE MAC 绔彛銆傚畠涔熷彲浠ヨ杞彂鍒板唴閮ㄤ氦鎹㈢鍙?0锛屼粠鑰屽彲浠ヤ娇鐢ㄤ互澶綉 DMA锛圗DMA锛夊紩鎿庡皢鏁版嵁鍖呬紶閫佺粰 ARM 鏍搞€備互澶綉 DMA 椹卞姩浼氬皢鏁版嵁鍖呬紶閫佸埌鐩稿簲鐨?鈥渘etdevice鈥?鎺ュ彛銆?
涓嬮潰鍒楀嚭灏嗙敱鏈?
PPE 椹卞姩椹卞姩鐨?PPE 寮曟搸涓昏妯″潡锛?

- BM
## PPE 椹卞姩姒傝堪
- QM

- SCH

- L2

- Makefile
- ppe.c
- MAC
- ppe_config.c
- ppe_config.h
- ppe_debugfs.c
- ppe_debugfs.h
- ppe_regs.h
- EDMA锛堜互澶綉 DMA锛?
ppe.c 鏂囦欢鍖呭惈涓昏鐨?PPE 骞冲彴椹卞姩锛屽苟鎵挎媴 PPE 浜ゆ崲鏍稿績妯″潡锛堝 QM銆丅M 鍜?L2锛夌殑鍒濆鍖栥€傝繖浜涚‖浠舵ā鍧楃殑閰嶇疆 API 鍦?ppe_config.c 鏂囦欢涓彁渚涖€?

ppe.h 瀹氫箟浜?PPE 璁惧鏁版嵁缁撴瀯锛屼緵 PPE 椹卞姩鍑芥暟浣跨敤銆?
鍦?PPE MAC 绔彛涓婃帴鏀跺埌鐨勬暟鎹寘鍙互杞彂鍒板彟涓€涓?PPE MAC 绔彛銆傚畠涔熷彲浠?
杞彂鍒板唴閮ㄤ氦鎹㈢鍙?0锛屼互渚块€氳繃浠ュお缃?DMA锛圗DMA锛夊紩鎿庡皢鏁版嵁鍖?
閫佽揪 ARM 鏍稿績銆備互澶綉 DMA 椹卞姩浼氬皢
鏁版嵁鍖呮姇閫掑埌鐩稿簲鐨?'netdevice' 鎺ュ彛銆?
## PPE 椹卞姩鏀寔鐨?SoC
PPE MAC锛坣etdevice锛夈€丳CS 涓庡閮?PHY 鐨勮蒋浠跺疄渚嬩笌
Linux PHYLINK 妗嗘灦浜や簰锛屼互绠＄悊 PPE 绔彛涓?
鎵€杩炴帴 PHY 涔嬮棿鐨勮繛鎺ヤ互鍙婄鍙ｉ摼璺姸鎬併€備笂鍥句篃灞曠ず浜嗚繖涓€鐐广€?

- IPQ9574
## PPE 椹卞姩姒傝堪

PPE 椹卞姩鏄潰鍚?Qualcomm IPQ SoC 鐨勪互澶綉椹卞姩銆傚畠鏄竴涓崟涓€鐨勫钩鍙伴┍鍔紝
鍖呭惈 PPE 閮ㄥ垎鍜屼互澶綉 DMA 閮ㄥ垎銆侾PE 閮ㄥ垎鍒濆鍖栧苟椹卞姩
PPE 浜ゆ崲鏍稿績涓殑鍚勭妯″潡锛堝 BM/QM/L2 妯″潡鍜?PPE MAC锛夈€侲DMA 閮ㄥ垎
椹卞姩浠ュお缃?DMA锛岀敤浜庡湪 PPE 绔彛涓?ARM 鏍稿績涔嬮棿浼犺緭鏁版嵁鍖咃紝骞跺惎鐢?
闈㈠悜 PPE 绔彛鐨?netdevice 椹卞姩銆?
  -> Device Drivers
drivers/net/ethernet/qualcomm/ppe/ 涓嬬殑 PPE 椹卞姩鏂囦欢濡備笅鎵€鍒楋細
      -> Ethernet driver support
- Makefile
- ppe.c
- ppe.h
- ppe_config.c
- ppe_config.h
- ppe_debugfs.c
- ppe_debugfs.h
- ppe_regs.h

ppe.c 鏂囦欢鍖呭惈涓昏鐨?PPE 骞冲彴椹卞姩锛屽苟璐熻矗鍒濆鍖?
PPE 浜ゆ崲鏍稿績妯″潡锛堝 QM銆丅M 鍜?L2锛夈€傝繖浜涚‖浠舵ā鍧楃殑
閰嶇疆 API 鍦?ppe_config.c 鏂囦欢涓彁渚涖€?
PPE 纭欢璁℃暟鍣ㄥ彲浠ラ€氳繃 debugfs 鎺ュ彛浠?`/sys/kernel/debug/ppe/` 鐩綍璁块棶銆?
ppe.h 瀹氫箟浜?PPE 璁惧鏁版嵁缁撴瀯锛屼緵 PPE 椹卞姩鍑芥暟浣跨敤銆?