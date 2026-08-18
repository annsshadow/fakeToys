
## 寰峰窞浠櫒 ICSSG PRUETH 浠ュお缃戦┍鍔?

:Version: 1.0

## ICSSG 鍥轰欢


姣忎釜 ICSSG 鏍稿績鏈変袱涓彲缂栫▼瀹炴椂鍗曞厓锛圥RU锛夈€佷袱涓緟鍔╁疄鏃朵紶杈撳崟鍏冿紙RTU锛変互鍙婁袱涓彂閫佸疄鏃朵紶杈撳崟鍏冿紙TX_PRU锛夈€傚叾涓瘡涓€涓兘杩愯鍚勮嚜鐨勫浐浠躲€傝繖浜涘浐浠跺悎璧锋潵琚О涓?ICSSG 鍥轰欢銆?
## 鍥轰欢缁熻


ICSSG 鍥轰欢缁存姢鏌愪簺缁熻淇℃伅锛岀敱椹卞姩閫氳繃 `ethtool -S <interface>` 瀵煎嚭銆?
杩欎簺缁熻淇℃伅濡備笅锛?
 - `FW_RTU_PKT_DROP`锛氳瘖鏂敊璇鏁板櫒锛屽綋 RTU 鍥犵鍙ｈ绂佺敤鎴栬鍒欒繚渚嬭€屼涪寮冩湰鍦版敞鍏ョ殑鏁版嵁鍖呮椂閫掑銆? - `FW_Q0_OVERFLOW`锛氶槦鍒?0 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q1_OVERFLOW`锛氶槦鍒?1 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q2_OVERFLOW`锛氶槦鍒?2 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q3_OVERFLOW`锛氶槦鍒?3 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q4_OVERFLOW`锛氶槦鍒?4 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q5_OVERFLOW`锛氶槦鍒?5 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q6_OVERFLOW`锛氶槦鍒?6 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_Q7_OVERFLOW`锛氶槦鍒?7 鐨?TX 婧㈠嚭璁℃暟鍣? - `FW_DROPPED_PKT`锛氬綋鏁版嵁鍖呭洜瑙勫垯杩濅緥鍦?PRU 澶勮涓㈠純鏃讹紝璇ヨ鏁板櫒閫掑銆? - `FW_RX_ERROR`锛氳嫢 PRU 澶勫彂鐢?CRC 閿欒鎴栨渶灏?鏈€澶у抚閿欒鍒欓€掑
 - `FW_RX_DS_INVALID`锛氬綋 RTU 妫€娴嬪埌鏁版嵁鐘舵€佹棤鏁堟潯浠舵椂閫掑
 - `FW_TX_DROPPED_PACKET`锛氱粡鐢?TX 绔彛涓㈠純鐨勬暟鎹寘璁℃暟鍣? - `FW_TX_TS_DROPPED_PACKET`锛氬甫鏈?TS 鏍囧織銆佺粡鐢?TX 绔彛涓㈠純鐨勬暟鎹寘璁℃暟鍣? - `FW_INF_PORT_DISABLED`锛氬洜绔彛琚鐢ㄨ€屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_SAV`锛氬洜婧愬湴鍧€杩濅緥鑰屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_SA_DL`锛氬洜婧愬湴鍧€浣嶄簬鎷掔粷鍒楄〃涓€屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_PORT_BLOCKED`锛氬洜绔彛琚樆濉炰笖甯т负鐗规畩甯ц€屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_DROP_TAGGED`锛氬洜甯︽爣绛捐€屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_DROP_PRIOTAGGED`锛氬洜甯︿紭鍏堢骇鏍囩鑰屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_DROP_NOTAG`锛氬洜鏃犳爣绛捐€屼涪寮?RX 甯ф椂閫掑
 - `FW_INF_DROP_NOTMEMBER`锛氬洜绔彛涓嶆槸 VLAN 鎴愬憳鑰屼涪寮?RX 甯ф椂閫掑
 - `FW_RX_EOF_SHORT_FRMERR`锛氬湪鏈湅鍒?RX_B1 鐨勬儏鍐典笅璋冨害甯х粨鏉燂紙EOF锛変换鍔℃椂閫掑
 - `FW_RX_B0_DROP_EARLY_EOF`锛氬洜鎻愬墠 EOF 鑰屼涪寮冨抚鏃堕€掑
 - `FW_TX_JUMBO_FRM_CUTOFF`锛氫负闃叉鏁版嵁鍖呭ぇ灏忚秴杩?2000 瀛楄妭鑰屽甯ц繘琛屾埅鏂椂閫掑
 - `FW_RX_EXP_FRAG_Q_DROP`锛氬綋蹇€熷抚鍦ㄥ墠涓€鐗囨鎵€鍦ㄧ殑鍚屼竴闃熷垪涓鎺ユ敹鏃堕€掑
 - `FW_RX_FIFO_OVERRUN`锛歊X fifo 婧㈠嚭璁℃暟鍣? - `FW_CUT_THR_PKT`锛氫娇鐢ㄧ洿閫氾紙Cut-Through锛夎浆鍙戞柟娉曡浆鍙戞暟鎹寘鏃堕€掑
 - `FW_HOST_RX_PKT_CNT`锛歊x PRU 閫氳繃 PSI 鍙戦€佺粰涓绘満鐨勬湁鏁堟暟鎹寘鏁伴噺
 - `FW_HOST_TX_PKT_CNT`锛歊TU0 澶嶅埗鍒?Tx 闃熷垪鐨勬湁鏁堟暟鎹寘鏁伴噺
 - `FW_HOST_EGRESS_Q_PRE_OVERFLOW`锛氫富鏈哄嚭鍙ｉ槦鍒楋紙鍙姠鍗狅級婧㈠嚭璁℃暟鍣? - `FW_HOST_EGRESS_Q_EXP_OVERFLOW`锛氫富鏈哄嚭鍙ｉ槦鍒楋紙鍙姠鍗狅級婧㈠嚭璁℃暟鍣?