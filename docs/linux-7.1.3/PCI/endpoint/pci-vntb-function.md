
## PCI vNTB 鍔熻兘锛團unction锛?

鏈枃浠嬬粛 PCI 绔偣锛圗ndpoint锛夊瓙绯荤粺涓嬬殑铏氭嫙 NTB 鍔熻兘锛坴NTB Function锛夛紝璇存槑鍏朵笌鏍囧噯 PCI NTB 鐨勫尯鍒€佸疄鐜版墍鐢ㄧ殑鍏抽敭缁撴瀯锛堥厤缃尯銆佷究绛惧瘎瀛樺櫒銆侀棬閾冦€佸唴瀛樼獥鍙ｇ瓑锛夊強鍏跺伐浣滃師鐞嗭紝渚?PCIe 绔偣椹卞姩寮€鍙戣€呭弬鑰冦€?


:Author: Frank Li <Frank.Li@nxp.com>

PCI NTB 鍔熻兘涓?PCI vNTB 鍔熻兘鐨勫尯鍒湪浜庯細

PCI NTB 鍔熻兘闇€瑕佷袱涓鐐癸紙endpoint锛夊疄渚嬶紝杩炴帴 HOST1 涓?HOST2銆?

PCI vNTB 鍔熻兘鍙娇鐢ㄤ竴涓富鏈猴紙host锛変笌涓€涓鐐癸紙EP锛夛紝浣跨敤 NTB 杩炴帴 EP 涓?PCI 涓绘満



  +------------+         +---------------------------------------+
  |            |         |                                       |
  +------------+         |                        +--------------+
  | NTB        |         |                        | NTB          |
  | NetDev     |         |                        | NetDev       |
  +------------+         |                        +--------------+
  | NTB        |         |                        | NTB          |
  | Transfer   |         |                        | Transfer     |
  +------------+         |                        +--------------+
  |            |         |                        |              |
  |  PCI NTB   |         |                        |              |
  |    EPF     |         |                        |              |
  |   Driver   |         |                        | PCI Virtual  |
  |            |         +---------------+        | NTB Driver   |
  |            |         | PCI EP NTB    |<------>|              |
  |            |         |  FN Driver    |        |              |
  +------------+         +---------------+        +--------------+
  |            |         |               |        |              |
  |  PCI BUS   | <-----> |  PCI EP BUS   |        |  Virtual PCI |
  |            |  PCI    |               |        |     BUS      |
  +------------+         +---------------+--------+--------------+
      PCI RC                        PCI EP

## 鐢ㄤ簬瀹炵幇 vNTB 鐨勭粨鏋勶紙Constructs used for Implementing vNTB锛?


 1) 閰嶇疆鍖猴紙Config Region锛?
 2) 鑷韩渚跨瀵勫瓨鍣紙Self Scratchpad Registers锛?
 3) 瀵圭渚跨瀵勫瓨鍣紙Peer Scratchpad Registers锛?
 4) 闂ㄩ搩锛圖oorbell锛孌B锛夊瘎瀛樺櫒
 5) 鍐呭瓨绐楀彛锛圡emory Window锛孧W锛?


### 閰嶇疆鍖猴紙Config Region锛夛細


涓?PCI NTB Function 椹卞姩鐩稿悓

### 渚跨瀵勫瓨鍣紙Scratchpad Registers锛夛細


瀹冮檮鍔犲湪閰嶇疆鍖轰箣鍚庛€?



  +--------------------------------------------------+ Base
  |                                                  |
  |                                                  |
  |                                                  |
  |          Common Config Register                  |
  |                                                  |
  |                                                  |
  |                                                  |
  +-----------------------+--------------------------+ Base + span_offset
  |                       |                          |
  |    Peer Span Space    |    Span Space            |
  |                       |                          |
  |                       |                          |
  +-----------------------+--------------------------+ Base + span_offset
  |                       |                          |      + span_count * 4
  |                       |                          |
  |     Span Space        |   Peer Span Space        |
  |                       |                          |
  +-----------------------+--------------------------+
        Virtual PCI             Pcie Endpoint
        NTB Driver               NTB Driver


### 闂ㄩ搩瀵勫瓨鍣紙Doorbell Registers锛夛細


闂ㄩ搩瀵勫瓨鍣ㄧ敱涓绘満鐢ㄦ潵浜掔浉涓柇銆?

### 鍐呭瓨绐楀彛锛圡emory Window锛夛細


涓や釜涓绘満涔嬮棿鐨勫疄闄呮暟鎹紶杈撳皢閫氳繃鍐呭瓨绐楀彛杩涜銆?

## 寤烘ā缁撴瀯锛圡odeling Constructs锛夛細


32 浣?BAR銆?

======  ===============
BAR NO  CONSTRUCTS USED
======  ===============
BAR0    閰嶇疆鍖猴紙Config Region锛?
BAR1    闂ㄩ搩锛圖oorbell锛?
BAR2    鍐呭瓨绐楀彛 1锛圡emory Window 1锛?
BAR3    鍐呭瓨绐楀彛 2锛圡emory Window 2锛?
BAR4    鍐呭瓨绐楀彛 3锛圡emory Window 3锛?
BAR5    鍐呭瓨绐楀彛 4锛圡emory Window 4锛?
======  ===============

64 浣?BAR銆?

======  ===============================
BAR NO  CONSTRUCTS USED
======  ===============================
BAR0    閰嶇疆鍖猴紙Config Region锛?+ 渚跨瀵勫瓨鍣紙Scratchpad锛?
BAR1
BAR2    闂ㄩ搩锛圖oorbell锛?
BAR3
BAR4    鍐呭瓨绐楀彛 1锛圡emory Window 1锛?
BAR5
======  ===============================
