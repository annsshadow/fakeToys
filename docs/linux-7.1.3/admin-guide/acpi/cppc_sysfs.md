
## 鍗忎綔寮忓鐞嗗櫒鎬ц兘鎺у埗锛圕PPC锛?


## CPPC


ACPI 瑙勮寖涓畾涔夌殑 CPPC 鎻忚堪浜嗕竴绉嶆満鍒讹紝渚涙搷浣滅郴缁熷湪杩炵画涓旀娊璞＄殑鎬ц兘鍒诲害涓婄鐞嗛€昏緫澶勭悊鍣ㄧ殑鎬ц兘銆侰PPC 鏆撮湶涓€缁勫瘎瀛樺櫒鏉ユ弿杩版娊璞℃€ц兘鍒诲害銆佽姹傛€ц兘绾у埆浠ュ強娴嬮噺姣?CPU 鐨勪氦浠樻€ц兘銆?
鏈夊叧 CPPC 鐨勬洿澶氱粏鑺傦紝璇峰弬闃?ACPI 瑙勮寖锛?
http://uefi.org/specifications

```

  /sys/devices/system/cpu/cpuX/acpi_cppc/

```
```

  $ ls -lR  /sys/devices/system/cpu/cpu0/acpi_cppc/
  /sys/devices/system/cpu/cpu0/acpi_cppc/:
  total 0
  -r--r--r-- 1 root root 65536 Mar  5 19:38 feedback_ctrs
  -r--r--r-- 1 root root 65536 Mar  5 19:38 highest_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_freq
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_nonlinear_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 lowest_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 nominal_freq
  -r--r--r-- 1 root root 65536 Mar  5 19:38 nominal_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 reference_perf
  -r--r--r-- 1 root root 65536 Mar  5 19:38 wraparound_time

```
- highest_perf锛氭湰澶勭悊鍣ㄧ殑鏈€楂樻€ц兘锛堟娊璞″埢搴︼級銆?- nominal_perf锛氭湰澶勭悊鍣ㄧ殑鏈€楂樻寔缁€ц兘锛堟娊璞″埢搴︼級銆?- lowest_nonlinear_perf锛氭湰澶勭悊鍣ㄥ湪闈炵嚎鎬ц妭鑳戒笅鐨勬渶浣庢€ц兘锛堟娊璞″埢搴︼級銆?- lowest_perf锛氭湰澶勭悊鍣ㄧ殑鏈€浣庢€ц兘锛堟娊璞″埢搴︼級銆?
- lowest_freq锛氬搴?lowest_perf 鐨?CPU 棰戠巼锛堝崟浣?MHz锛夈€?- nominal_freq锛氬搴?nominal_perf 鐨?CPU 棰戠巼锛堝崟浣?MHz锛夈€?  涓婅堪棰戠巼浠呭簲鐢ㄤ簬浠ラ鐜囪€岄潪鎶借薄鍒诲害鏉ユ姤鍛婂鐞嗗櫒鎬ц兘锛屼笉搴斿皢鍏剁敤浜庝换浣曞姛鑳芥€у喅绛栥€?
- feedback_ctrs锛氬寘鍚弬鑰冩€ц兘璁℃暟鍣ㄤ笌浜や粯鎬ц兘璁℃暟鍣ㄣ€?  鍙傝€冭鏁板櫒闅忓鐞嗗櫒鍙傝€冩€ц兘鎴愭瘮渚嬮€掑銆?  浜や粯璁℃暟鍣ㄩ殢澶勭悊鍣ㄤ氦浠樻€ц兘鎴愭瘮渚嬮€掑銆?- wraparound_time锛氬弽棣堣鏁板櫒鍥炵粫鎵€闇€鐨勬渶鐭椂闂达紙鍗曚綅绉掞級銆?- reference_perf锛氬弬鑰冩€ц兘璁℃暟鍣ㄧ疮鍔犳椂鐨勬€ц兘绾у埆锛堟娊璞″埢搴︼級銆?

## 璁＄畻骞冲潎浜や粯鎬ц兘


涓嬮潰鎻忚堪閫氳繃鍦ㄦ椂闂?T1 鍜?T2 涓ゆ鑾峰彇鍙嶉璁℃暟鍣ㄥ揩鐓ф潵璁＄畻骞冲潎浜や粯鎬ц兘鐨勬楠ゃ€?
  T1: 灏?feedback_ctrs 璇诲彇涓?fbc_t1
      绛夊緟鎴栬繍琛屾煇浜涘伐浣滆礋杞?
  T2: 灏?feedback_ctrs 璇诲彇涓?fbc_t2

```

  delivered_counter_delta = fbc_t2[del] - fbc_t1[del]
  reference_counter_delta = fbc_t2[ref] - fbc_t1[ref]

  delivered_perf = (reference_perf x delivered_counter_delta) / reference_counter_delta

```
