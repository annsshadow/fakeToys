## Cavium ThunderX2 SoC 鎬ц兘鐩戞帶鍗曞厓锛圥MU UNCORE锛?

ThunderX2 SoC PMU 鐢辩嫭绔嬬殑銆佺郴缁熻寖鍥寸殑銆佹瘡鎻掓Ы PMU 缁勬垚锛屼緥濡備笁绾х紦瀛?锛圠3C锛夈€丏DR4 鍐呭瓨鎺у埗鍣紙DMC锛変互鍙?Cavium 涓€鑷存€у鐞嗗櫒浜掕繛锛圕CPI2锛夈€?
DMC 鏈?8 涓氦閿欓€氶亾锛孡3C 鏈?16 涓氦閿欏垎鍧椼€備簨浠堕拡瀵归粯璁ら€氶亾锛堝嵆閫氶亾 0锛?璁℃暟锛屽苟鎸夐€氶亾/鍒嗗潡鎬绘暟鎸夋瘮渚嬪垎鎽娿€?
DMC 涓?L3C 鏈€澶氭敮鎸?4 涓鏁板櫒锛岃€?CCPI2 鏈€澶氭敮鎸?8 涓鏁板櫒銆傝鏁板櫒鍙嫭绔?缂栫▼涓轰笉鍚屼簨浠讹紝骞跺彲鍗曠嫭鍚姩鍜屽仠姝€傛病鏈変换浣曡鏁板櫒鏀寔婧㈠嚭涓柇銆侱MC 涓?L3C 璁℃暟鍣ㄤ负 32 浣嶏紝姣?2 绉掕鍙栦竴娆°€侰CPI2 璁℃暟鍣ㄤ负 64 浣嶏紝鍦ㄦ甯告搷浣滀腑
鍋囧畾涓嶄細婧㈠嚭銆?
PMU UNCORE锛坧erf锛夐┍鍔細

thunderx2_pmu 椹卞姩涓?DMC 涓?L3C 璁惧娉ㄥ唽姣忔彃妲界殑 perf PMU銆傛瘡涓?PMU 鍙敤浜?鍚屾椂璁℃暟鏈€澶?4 涓紙DMC/L3C锛夋垨鏈€澶?8 涓紙CCPI2锛変簨浠躲€傝繖浜?PMU 鍦?sysfs 涓?鎻愪緵鍏跺彲鐢ㄤ簨浠朵笌閰嶇疆閫夐」鐨勬弿杩帮紝瑙?/sys/bus/event_source/devices/uncore_<l3c_S/dmc_S/ccpi2_S/>锛汼 涓烘彃妲?id銆?
璇ラ┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝も€減erf record鈥濇棤娉曞伐浣溿€備篃涓嶆敮鎸佹瘡浠诲姟 perf 浼氳瘽銆?
```

  # perf stat -a -e uncore_dmc_0/cnt_cycles/ sleep 1

  # perf stat -a -e \
  uncore_dmc_0/cnt_cycles/,\
  uncore_dmc_0/data_transfers/,\
  uncore_dmc_0/read_txns/,\
  uncore_dmc_0/write_txns/ sleep 1

  # perf stat -a -e \
  uncore_l3c_0/read_request/,\
  uncore_l3c_0/read_hit/,\
  uncore_l3c_0/inv_request/,\
  uncore_l3c_0/inv_hit/ sleep 1

```
