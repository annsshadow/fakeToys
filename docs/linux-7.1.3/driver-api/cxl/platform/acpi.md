
## ACPI 琛?

ACPI 鏄€滈珮绾ч厤缃笌鐢垫簮鎺ュ彛鈥濓紙Advanced Configuration and Power Interface锛夛紝鏄竴椤?瀹氫箟骞冲彴涓庢搷浣滅郴缁熷浣曠鐞嗙數婧愪互鍙婇厤缃绠楁満纭欢鐨勬爣鍑嗐€傚嚭浜庢湰鎿嶄綔鐞嗚鐨勭洰鐨勶紝褰?鎻愬埌鈥淎CPI鈥濇椂锛屾垜浠€氬父鎸囩殑鏄€淎CPI 琛ㄢ€濃€斺€斿钩鍙帮紙BIOS/EFI锛夊悜鎿嶄綔绯荤粺浼犻€掗潤鎬侀厤缃?淇℃伅鐨勬柟寮忋€?
浠ヤ笅 ACPI 琛ㄥ寘鍚叧浜?CXL 璁惧鐨?*闈欐€?*閰嶇疆涓庢€ц兘鏁版嵁锛?
- [acpi/cedt.rst](acpi/cedt.rst)
- [acpi/srat.rst](acpi/srat.rst)
- [acpi/hmat.rst](acpi/hmat.rst)
- [acpi/slit.rst](acpi/slit.rst)
- [acpi/dsdt.rst](acpi/dsdt.rst)

SRAT 琛ㄤ篃鍙兘鍖呭惈閫氱敤鐨勭鍙?鍙戣捣鑰咃紙initiator锛夊唴瀹癸紝鏃ㄥ湪鎻忚堪閫氱敤绔彛锛屼絾涓嶅寘鍚?閫氬線绔偣璺緞鍏朵綑閮ㄥ垎鐨勪俊鎭€?
Linux 浣跨敤杩欎簺琛ㄦ潵涓洪潤鎬侀厤缃紙鐢?BIOS/EFI锛夌殑 CXL 璁惧閰嶇疆鍐呮牳璧勬簮锛屼緥濡傦細

- NUMA 鑺傜偣
- 鍐呭瓨鍒嗗眰锛圡emory Tiers锛?- NUMA 鎶借薄璺濈锛圓bstract Distances锛?- SystemRAM 鍐呭瓨鍖哄煙
- 鍔犳潈浜ら敊鑺傜偣鏉冮噸锛圵eighted Interleave Node Weights锛?
## ACPI 璋冭瘯


`acpidump -b` 鍛戒护灏?ACPI 琛ㄨ浆鍌ㄤ负浜岃繘鍒舵牸寮忋€?
`iasl -d` 鍛戒护灏嗘枃浠跺弽姹囩紪涓轰汉绫诲彲璇荤殑鏍煎紡銆?
```

   [000h 0000   4]   Signature : "CEDT"    [CXL Early Discovery Table]

```
### 甯歌闂


姝ゅ鎻忚堪鐨勫ぇ澶氭暟澶辫触浼氬鑷撮┍鍔ㄦ棤娉曞皢鍐呭瓨浣滀负 DAX 璁惧鍜?鎴?kmem 鍛堢幇銆?
- CEDT CFMWS 鐩爣鍒楄〃 UID 涓?CEDT CHBS UID 涓嶅尮閰嶃€?- CEDT CFMWS 鐩爣鍒楄〃 UID 涓?DSDT CXL 涓绘ˉ UID 涓嶅尮閰嶃€?- CEDT CFMWS 闄愬埗浣嶄笉姝ｇ‘銆?- CEDT CFMWS 鍐呭瓨鍖哄煙瀵归綈涓嶈壇銆?- CEDT CFMWS 鍐呭瓨鍖哄煙璺ㄨ秺浜嗗钩鍙板唴瀛樼┖娲炪€?- CEDT CHBS UID 涓?DSDT CXL 涓绘ˉ UID 涓嶅尮閰嶃€?- CEDT CHBS 瑙勮寖鐗堟湰涓嶆纭€?- SRAT 缂哄皯 CEDT CFMWS 涓弿杩扮殑鍖哄煙銆?
  - 缁撴灉锛氭棤娉曚负璇ュ尯鍩熷垱寤?NUMA 鑺傜偣锛屾垨鑰呰鍖哄煙琚斁鍏ラ敊璇殑鑺傜偣銆?
- HMAT 缂哄皯 CEDT CFMWS 涓弿杩扮殑鍖哄煙鐨勬暟鎹€?
  - 缁撴灉锛歂UMA 鑺傜偣琚斁鍏ラ敊璇殑鍐呭瓨鍒嗗眰銆?
- SLIT 鏈夐敊璇暟鎹€?
  - 缁撴灉锛氬唴鏍镐腑璁稿鎬ц兘鏈哄埗浼氶潪甯镐笉婊°€?
鎵€鏈夎繖浜涢棶棰樺湪鐢ㄦ埛鐪嬫潵閮藉儚鏄┍鍔ㄦ湭鑳芥敮鎸?CXL鈥斺€旇€屽疄闄呬笂瀹冧滑閮芥槸骞冲彴鏈兘姝ｇ‘閰嶇疆
ACPI 琛ㄦ墍瀵艰嚧鐨勫け璐ャ€?