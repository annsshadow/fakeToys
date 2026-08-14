## 鍗庝负浠ュお缃戣澶囬┍鍔紙hinic3锛夌郴鍒?Linux 鍐呮牳椹卞姩

## 姒傝堪

hinic3 鏄潰鍚戞暟鎹腑蹇冪殑缃戠粶鎺ュ彛鍗★紙NIC锛夈€傚畠鏀寔涓€绯诲垪涓嶅悓閾捐矾閫熺巼鐨勮澶囷紙10GE銆?5GE銆?00GE 绛夛級銆俬inic3 璁惧鍙互鏈夊绉嶇墿鐞嗗舰鎬侊細LOM锛堜富鏉块泦鎴愬眬鍩熺綉锛孡an on Motherboard锛塏IC銆丳CIe 鏍囧噯 NIC銆丱CP锛堝紑鏀捐绠楅」鐩紝Open Compute Project锛塏IC 绛夈€?
hinic3 椹卞姩鏀寔浠ヤ笅鐗规€э細
- IPv4/IPv6 TCP/UDP 鏍￠獙鍜屽嵏杞?- TSO锛圱CP 鍒嗘鍗歌浇锛孴CP Segmentation Offload锛夈€丩RO锛堝ぇ鎺ユ敹鍗歌浇锛孡arge Receive Offload锛?- RSS锛堟帴鏀朵晶缂╂斁锛孯eceive Side Scaling锛?- MSI-X 涓柇鑱氬悎閰嶇疆涓庝腑鏂嚜閫傚簲
- SR-IOV锛堝崟鏍?I/O 铏氭嫙鍖栵紝Single Root I/O Virtualization锛?
## 鍐呭

- 鍙楁敮鎸佺殑 PCI 鍘傚晢 ID/璁惧 ID
- Hinic3 椹卞姩婧愪唬鐮佺粨鏋?- 绠＄悊鎺ュ彛

## 鍙楁敮鎸佺殑 PCI 鍘傚晢 ID/璁惧 ID

19e5:0222 - hinic3 PF/PPF
19e5:375F - hinic3 VF

涓荤墿鐞嗗姛鑳斤紙PPF锛孭rime Physical Function锛夎礋璐ｆ暣寮?NIC 鍗＄殑绠＄悊銆備緥濡傦紝NIC 涓庝富鏈轰箣闂寸殑鏃堕挓鍚屾銆備换鎰?PF 閮藉彲浠ュ厖褰?PPF銆侾PF 鏄姩鎬侀€夋嫨鐨勩€?
## Hinic3 椹卞姩婧愪唬鐮佺粨鏋?
========================  ================================================
hinic3_pci_id_tbl.h       Supported device IDs
hinic3_hw_intf.h          Interface between HW and driver
hinic3_queue_common.[ch]  Common structures and methods for NIC queues
hinic3_common.[ch]        Encapsulation of memory operations in Linux
hinic3_csr.h              Register definitions in the BAR
hinic3_hwif.[ch]          Interface for BAR
hinic3_eqs.[ch]           Interface for AEQs and CEQs
hinic3_mbox.[ch]          Interface for mailbox
hinic3_mgmt.[ch]          Management interface based on mailbox and AEQ
hinic3_wq.[ch]            Work queue data structures and interface
hinic3_cmdq.[ch]          Command queue is used to post command to HW
hinic3_hwdev.[ch]         HW structures and methods abstractions
hinic3_lld.[ch]           Auxiliary driver adaptation layer
hinic3_hw_comm.[ch]       Interface for common HW operations
hinic3_mgmt_interface.h   Interface between firmware and driver
hinic3_hw_cfg.[ch]        Interface for HW configuration
hinic3_irq.c              Interrupt request
hinic3_netdev_ops.c       Operations registered to Linux kernel stack
hinic3_nic_dev.h          NIC structures and methods abstractions
hinic3_main.c             Main Linux kernel driver
hinic3_nic_cfg.[ch]       NIC service configuration
hinic3_nic_io.[ch]        Management plane interface for TX and RX
hinic3_rss.[ch]           Interface for Receive Side Scaling (RSS)
hinic3_rx.[ch]            Interface for transmit
hinic3_tx.[ch]            Interface for receive
hinic3_ethtool.c          Interface for ethtool operations (ops)
hinic3_filter.c           Interface for MAC address
========================  ================================================

## 绠＄悊鎺ュ彛

### 寮傛浜嬩欢闃熷垪锛圓EQ锛?
AEQ 閫氳繃涓€涓弿杩扮闃熷垪浠庣‖浠舵帴鏀堕珮浼樺厛绾т簨浠躲€傛瘡涓弿杩扮鍥哄畾澶у皬涓?64 瀛楄妭銆侫EQ 鍙互鎺ユ敹涓诲姩锛坰olicited锛夋垨琚姩锛坲nsolicited锛変簨浠躲€傛瘡涓澶囷紙VF 鎴?PF锛夋渶澶氬彲浠ユ湁 4 涓?AEQ銆傛瘡涓?AEQ 鍏宠仈涓€涓笓鐢ㄧ殑 IRQ銆侫EQ 鍙互鎺ユ敹澶氱绫诲瀷鐨勪簨浠讹紝浣嗗湪瀹炶返涓?hinic3 椹卞姩蹇界暐闄?2 涓偖绠辩浉鍏充簨浠朵箣澶栫殑鎵€鏈変簨浠躲€?
### 閭锛圡ailbox锛?
閭鏄?hinic3 椹卞姩涓庣‖浠朵箣闂寸殑涓€绉嶉€氫俊鏈哄埗銆傛瘡涓澶囨湁涓€涓嫭绔嬬殑閭銆傞┍鍔ㄥ彲浠ヤ娇鐢ㄩ偖绠卞悜绠＄悊骞抽潰鍙戦€佽姹傘€傞┍鍔ㄩ€氳繃 AEQ锛堜娇鐢ㄤ簨浠?HINIC3_AEQ_FOR_MBOX锛夋帴鏀堕偖绠辨秷鎭紝渚嬪瀵硅姹傜殑鍝嶅簲銆傜敱浜庨偖绠辨暟鎹瘎瀛樺櫒鐨勫ぇ灏忔湁闄愶紝閭娑堟伅鏄垎娈靛彂閫佺殑銆?
姣忎釜璁惧閮藉彲浠ヤ娇鐢ㄥ叾閭鍚戝浐浠跺彂閫佽姹傘€傞偖绠变篃鍙敤浜庡湪 PF 鍜屽畠鐨?VFs 涔嬮棿鍙戦€佽姹傚拰鍝嶅簲銆?
### 瀹屾垚浜嬩欢闃熷垪锛圕EQ锛?
CEQ 鐨勫疄鐜颁笌 AEQ 鐩稿悓銆傚畠閫氳繃涓€涓浐瀹氬ぇ灏忋€?2 浣嶇殑鎻忚堪绗︿粠纭欢鎺ユ敹瀹屾垚浜嬩欢銆傛瘡涓澶囨渶澶氬彲浠ユ湁 32 涓?CEQ銆傛瘡涓?CEQ 鏈変竴涓笓鐢ㄧ殑 IRQ銆侰EQ 鍙帴鏀朵富鍔紙solicited锛変簨浠讹紝杩欎簺浜嬩欢鏄椹卞姩璇锋眰鐨勫搷搴斻€侰EQ 鍙互鎺ユ敹澶氱绫诲瀷鐨勪簨浠讹紝浣嗗湪瀹炶返涓?hinic3 椹卞姩蹇界暐闄?HINIC3_CMDQ 涔嬪鐨勬墍鏈変簨浠讹紝HINIC3_CMDQ 琛ㄧず鍏堝墠鍦?cmdq 涓婂彂甯冪殑鍛戒护宸插畬鎴愩€?
### 鍛戒护闃熷垪锛坈mdq锛?
姣忎釜 cmdq 鏈変竴涓笓鐢ㄧ殑宸ヤ綔闃熷垪锛屽懡浠ゅ彂甯冨湪鍏朵笂銆傚伐浣滈槦鍒椾笂鐨勫懡浠ゆ槸鍥哄畾澶у皬銆?4 瀛楄妭鐨勬弿杩扮銆傚懡浠ょ殑瀹屾垚灏嗛€氳繃鎵胯浇璇ュ懡浠ょ殑鎻忚堪绗︿腑鐨?ctrl 浣嶆潵鎸囩ず銆傚懡浠ゅ畬鎴愮殑閫氱煡涔熶細閫氳繃 CEQ 涓婄殑浜嬩欢鎻愪緵銆傛瘡涓澶囨湁 4 涓懡浠ら槦鍒楋紝瀹冧滑浣滀负涓€缁勶紙绉颁负 cmdqs锛夊垵濮嬪寲锛屾瘡涓槦鍒楁湁鑷繁鐨勭被鍨嬨€侶inic3 椹卞姩鍙娇鐢ㄧ被鍨?HINIC3_CMDQ_SYNC銆?
### 宸ヤ綔闃熷垪锛圵Q锛?
宸ヤ綔闃熷垪鏄浐瀹氬ぇ灏?WQE 鐨勯€昏緫鏁扮粍銆傝鏁扮粍鍙互閫氳繃闂存帴琛ㄥ垎甯冨埌澶氫釜涓嶈繛缁殑椤典笂銆傚伐浣滈槦鍒楄 I/O 闃熷垪鍜屽懡浠ら槦鍒椾娇鐢ㄣ€?
### 鍏ㄥ眬鍔熻兘 ID

姣忎釜鍔熻兘锛圥F 鎴?VF锛夊湪璁惧鍐呮湁涓€涓敮涓€鐨勯『搴忔爣璇嗐€傝澶氱鐞嗗懡浠わ紙mbox 鎴?cmdq锛夐兘鍖呭惈杩欎釜 ID锛屼互渚跨‖浠跺皢鍛戒护鏁堟灉搴旂敤鍒版纭殑鍔熻兘涓娿€?
PF 琚厑璁搁€氳繃鎸囧畾 VF 鐨?ID 鍚戜粠灞?VF 鍙戦€佺鐞嗗懡浠ゃ€俈F 蹇呴』鎻愪緵瀹冭嚜宸辩殑 ID銆傜‖浠朵腑鐨勯槻娆洪獥鏈哄埗浼氬鑷存潵鑷?VF 鐨勫懡浠ゅ湪鍏跺寘鍚敊璇?ID 鏃跺け璐ャ€?