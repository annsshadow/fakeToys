## 闃块噷宸村反 T-Head SoC Uncore 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?


Yitian 710 鐢遍樋閲屽反宸撮泦鍥㈣姱鐗囧紑鍙戜笟鍔?T-Head 瀹氬埗鎵撻€狅紝瀹炵幇浜嗙敤浜庢€ц兘涓庡姛鑳借皟璇曘€佷互鏂逛究绯荤粺缁存姢鐨?uncore PMU銆?

## DDR 瀛愮郴缁?Driveway锛圖RW锛塒MU 椹卞姩


Yitian 710 閲囩敤鍏釜 DDR5/4 閫氶亾锛屾瘡涓?die 涓婂洓涓€傛瘡涓?DDR5 閫氶亾鐩镐簰鐙珛锛屼互鏈嶅姟绯荤粺鍐呭瓨璇锋眰銆備竴涓?DDR5 閫氶亾琚媶鍒嗕负涓や釜鐙珛鐨勫瓙閫氶亾銆侱DR 瀛愮郴缁?Driveway 涓烘瘡涓瓙閫氶亾瀹炵幇鐙珛鐨?PMU锛屼互鐩戞帶鍚勭鎬ц兘鎸囨爣銆?

Driveway PMU 璁惧閫氳繃 ali_drw_<sys_base_addr> 鍛藉悕锛屽苟涓?perf 涓€璧蜂娇鐢ㄣ€備緥濡傦紝ali_drw_21000 鍜?ali_drw_21080 鏄?die 0 涓悓涓€閫氶亾鐨勪袱涓瓙閫氶亾瀵瑰簲鐨勪袱涓?PMU 璁惧銆傝€?die 1 鐨?PMU 璁惧浠?ali_drw_400XXXXX 涓哄墠缂€锛屼緥濡?ali_drw_40021000銆?

姣忎釜瀛愰€氶亾鍏辨湁 36 涓?PMU 璁℃暟鍣紝鍒嗕负鍥涚粍锛?

- 绗?0 缁勶細PMU 鍛ㄦ湡璁℃暟鍣ㄣ€傝缁勬湁涓€瀵硅鏁板櫒 pmu_cycle_cnt_low 鍜?pmu_cycle_cnt_high锛岀敤浣滃熀浜?DDRC 鏍稿績鏃堕挓鐨勫懆鏈熻鏁般€?

- 绗?1 缁勶細PMU 甯﹀璁℃暟鍣ㄣ€傝缁勬湁 8 涓鏁板櫒锛岀敤浜庣粺璁℃墍閫?rank 涓叓涓?bank 缁勶紝鎴栧墠 4 涓鏁板櫒涓洓涓?rank 鍒嗗埆鐨勬€昏闂鏁般€傚熀鏈紶杈撳崟鍏冧负 64B銆?

- 绗?2 缁勶細PMU 閲嶈瘯璁℃暟鍣ㄣ€傝缁勬湁 10 涓鏁板櫒锛岀敤浜庣粺璁℃瘡绫讳笉鍙籂姝ｉ敊璇殑鎬婚噸璇曟鏁般€?

- 绗?3 缁勶細PMU 閫氱敤璁℃暟鍣ㄣ€傝缁勬湁 16 涓鏁板櫒锛岀敤浜庣粺璁￠€氱敤浜嬩欢銆?

鐩墠锛孌riveway PMU 椹卞姩浠呬娇鐢ㄧ 0 缁勫拰绗?3 缁勪腑鐨勮鏁板櫒銆?

DDR 鎺у埗鍣紙DDRCTL锛変笌 DDR PHY 鍏卞悓鏋勬垚灏?SoC 搴旂敤鎬荤嚎杩炴帴鍒?DDR 鍐呭瓨璁惧鐨勫畬鏁存柟妗堛€侱DRCTL 鎺ユ敹鐢?Synopsys 鑷畾涔夊畾涔夌殑 Host Interface锛圚IF锛変簨鍔°€傝繖浜涗簨鍔″湪鍐呴儴鎺掗槦骞惰皟搴﹁闂紝鍚屾椂婊¤冻 SDRAM 鍗忚鏃跺簭瑕佹眰銆佷簨鍔′紭鍏堢骇浠ュ強浜嬪姟涔嬮棿鐨勪緷璧栧叧绯汇€侱DRCTL 杩涜€屽湪 DDR PHY Interface锛圖FI锛変笂鍚?PHY 妯″潡鍙戝嚭鍛戒护锛孭HY 妯″潡鍚?SDRAM 鍙戣捣骞舵崟鑾锋暟鎹€俤riveway PMU 鍏峰纭欢閫昏緫锛岀敤浜庨噰闆?HIF銆丏FI 绛変笂鐨勭粺璁′笌鎬ц兘璁板綍淇″彿銆?

閫氳繃瀵归€氳繃 HIF 鎺ュ彛鍙戝線 DDRC 鐨?READ銆乄RITE 鍜?RMW 鍛戒护杩涜璁℃暟锛屾垜浠彲浠ヨ绠楀嚭甯﹀銆傜粺璁″唴瀛樼殑璁℃暟绀轰緥鐢ㄦ硶
```

  perf stat \
    -e ali_drw_21000/hif_wr/ \
    -e ali_drw_21000/hif_rd/ \
    -e ali_drw_21000/hif_rmw/ \
    -e ali_drw_21000/cycle/ \
    -e ali_drw_21080/hif_wr/ \
    -e ali_drw_21080/hif_rd/ \
    -e ali_drw_21080/hif_rmw/ \
    -e ali_drw_21080/cycle/ \
    -e ali_drw_23000/hif_wr/ \
    -e ali_drw_23000/hif_rd/ \
    -e ali_drw_23000/hif_rmw/ \
    -e ali_drw_23000/cycle/ \
    -e ali_drw_23080/hif_wr/ \
    -e ali_drw_23080/hif_rd/ \
    -e ali_drw_23080/hif_rmw/ \
    -e ali_drw_23080/cycle/ \
    -e ali_drw_25000/hif_wr/ \
    -e ali_drw_25000/hif_rd/ \
    -e ali_drw_25000/hif_rmw/ \
    -e ali_drw_25000/cycle/ \
    -e ali_drw_25080/hif_wr/ \
    -e ali_drw_25080/hif_rd/ \
    -e ali_drw_25080/hif_rmw/ \
    -e ali_drw_25080/cycle/ \
    -e ali_drw_27000/hif_wr/ \
    -e ali_drw_27000/hif_rd/ \
    -e ali_drw_27000/hif_rmw/ \
    -e ali_drw_27000/cycle/ \
    -e ali_drw_27080/hif_wr/ \
    -e ali_drw_27080/hif_rd/ \
    -e ali_drw_27080/hif_rmw/ \
    -e ali_drw_27080/cycle/ -- sleep 10

```
```

  perf stat -M ddr_read_bandwidth.all -- sleep 10
  perf stat -M ddr_write_bandwidth.all -- sleep 10

```
骞冲潎 DRAM 甯﹀鍙寜濡備笅鏂瑰紡璁＄畻锛?

- 璇诲甫瀹?= perf_hif_rd ** DDRC_WIDTH ** DDRC_Freq / DDRC_Cycle
- 鍐欏甫瀹?= (perf_hif_wr + perf_hif_rmw) ** DDRC_WIDTH ** DDRC_Freq / DDRC_Cycle

鍏朵腑锛孌DRC_WIDTH = 64 瀛楄妭銆?

褰撳墠椹卞姩涓嶆敮鎸侀噰鏍枫€傚洜姝?"perf record" 涓嶅彈鏀寔銆傚悓鏍凤紝鐢变簬浜嬩欢鍧囦负 uncore 浜嬩欢锛岄檮鍔犲埌浠诲姟涔熶笉鍙楁敮鎸併€?
