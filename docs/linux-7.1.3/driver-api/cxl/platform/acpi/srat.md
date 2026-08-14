
## SRAT - 闈欐€佽祫婧愪翰鍜屾€ц〃锛圫tatic Resource Affinity Table锛?

绯荤粺/闈欐€佽祫婧愪翰鍜屾€ц〃鎻忚堪璧勬簮锛圕PU銆佸唴瀛橈級鍒扳€滈偦杩戝煙锛圥roximity Domains锛夆€濈殑浜插拰鎬с€傝琛ㄥ湪鎶€鏈笂鏄彲閫夌殑锛屼絾瑕佽 Linux 鏋氫妇鍑烘€ц兘淇℃伅锛堝弬瑙?鈥淗MAT鈥濓級锛屽垯蹇呴』瀛樺湪璇ヨ〃銆?
CEDT 涓?SRAT 琛ㄤ箣闂达紝浠ュ強 NUMA 鑺傜偣濡備綍琚垱寤猴紝瀛樺湪鐫€寰鐨勯厤鍚堝叧绯汇€傚鏋滅粨鏋滀笌浣犵殑棰勬湡涓嶅お涓€鑷达紝璇锋鏌?SRAT 鐨勫唴瀛樹翰鍜屾€ц〃椤逛笌 CEDT CFMWS锛屼互纭畾浣犵殑骞冲彴鍦ㄧ伒娲绘嫇鎵戞柟闈㈠疄闄呮敮鎸佷粈涔堛€?
SRAT 鍙互闈欐€佸湴灏?CFMWS SPA 鑼冨洿鐨勪竴閮ㄥ垎鍒嗛厤缁欑壒瀹氱殑閭昏繎鍩熴€傛湁鍏宠繖鍦?NUMA 鎷撴墤涓浣曞憟鐜扮殑鏇村淇℃伅锛岃鍙傞槄 Linux NUMA 鍒涘缓鐩稿叧鍐呭銆?
## 閭昏繎鍩燂紙Proximity Domain锛?
閭昏繎鍩熷ぇ鑷寸浉褰撲簬鈥淣UMA 鑺傜偣锛圢UMA Node锛夆€濓紝浣嗗苟涓嶄繚璇佹槸涓€瀵逛竴鏄犲皠銆備緥濡傦紝瀛樺湪鈥滈偦杩戝煙 4鈥濇槧灏勫埌鈥淣UMA 鑺傜偣 3鈥濈殑鍦烘櫙銆傦紙鍙傝 鈥淣UMA 鑺傜偣鍒涘缓鈥濓級

## 鍐呭瓨浜插拰鎬э紙Memory Affinity锛?
涓€鑸潵璇达紝濡傛灉涓绘満鍦?BIOS 涓浠讳綍 CXL 缁撴瀯锛堣В鐮佸櫒锛夎繘琛屼簡缂栫▼锛岄偅涔堝氨闇€瑕佸瓨鍦ㄩ拡瀵硅鍐呭瓨鐨?SRAT 琛ㄩ」銆?
```

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000001          <- NUMA Node 1
             Reserved1 : 0000
          Base Address : 000000C050000000  <- Physical Memory Region
        Address Length : 0000003CA0000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
              Enabled : 1
        Hot Pluggable : 1
         Non-Volatile : 0


```
## 閫氱敤绔彛浜插拰鎬э紙Generic Port Affinity锛?
閫氱敤绔彛浜插拰鎬э紙Generic Port Affinity锛夊瓙琛ㄦ彁渚涗簡閭昏繎鍩熶笌浠ｈ〃閫氱敤绔彛锛堜緥濡?CXL 涓绘満妗ワ級鐨勮澶囧彞鏌勪箣闂寸殑鍏宠仈銆傚€熷姪璇ュ叧鑱旓紝鍙互浠?SRAT 涓绱?CPU锛堝彂璧锋柟锛変笌閫氱敤绔彛涔嬮棿璺緞鐨勫欢杩熶笌甯﹀鏁板€笺€傝繖鐢ㄤ簬涓虹儹鎻掓嫈鐨?CXL 璁惧鏋勫缓鎬ц兘鍧愭爣锛岃繖浜涜澶囨棤娉曞湪鍚姩鏃剁敱骞冲彴鍥轰欢鏋氫妇銆?
```

         Subtable Type : 06 [Generic Port Affinity]
                Length : 20               <- 32d, length of table
              Reserved : 00
    Device Handle Type : 00               <- 0 - ACPI, 1 - PCI
      Proximity Domain : 00000001
         Device Handle : ACPI0016:01
                 Flags : 00000001         <- Bit 0 (Enabled)
              Reserved : 00000000

```
閭昏繎鍩熶笌 [HMAT <hmat>](HMAT <hmat>) SSLBI 鐩爣閭昏繎鍩熷垪琛ㄧ浉鍖归厤锛屼互鑾峰彇鐩稿叧鐨勫欢杩熸垨甯﹀鏁板€笺€傝繖浜涙€ц兘鏁板€奸€氳繃璁惧鍙ユ焺鍏宠仈鍒版煇涓?CXL 涓绘満妗ャ€傞┍鍔ㄤ娇鐢ㄨ鍏宠仈鏉ユ绱㈤€氱敤绔彛鎬ц兘鏁板€硷紝鐢ㄤ簬鏁翠釜 CXL 璺緞璁块棶鍧愭爣鐨勮绠椼€?