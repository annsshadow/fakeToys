## APM X-Gene SoC 鎬ц兘鐩戣鍗曞厓锛圥MU锛?

X-Gene SoC PMU 鐢卞涓浉浜掔嫭绔嬬殑绯荤粺璁惧 PMU 缁勬垚锛屼緥濡?L3 cache锛圠3 缂撳瓨锛夈€両/O bridge锛圛/O 妗ワ級銆佸唴瀛樻帶鍒跺櫒妗ワ紙memory controller bridge锛変互鍙婂唴瀛樻帶鍒跺櫒锛坢emory controller锛夈€傝繖浜?PMU 璁惧閲囩敤鏉炬暎鏋舵瀯锛岄伒寰笌 ARM 鏍稿績 PMU 鐩稿悓鐨勬ā鍨嬨€傝繖浜?PMU 鍏变韩鐩稿悓鐨勬渶楂樼骇涓柇鍜岀姸鎬?CSR 鍖哄煙銆?
### PMU锛坧erf锛夐┍鍔?

xgene-pmu 椹卞姩浼氭敞鍐屽涓?perf PMU 椹卞姩銆傛瘡涓?perf 椹卞姩閮藉湪 sysfs 涓彁渚涘叾鍙敤浜嬩欢涓庨厤缃€夐」鐨勬弿杩帮紝鍙傝 /sys/bus/event_source/devices/<l3cX/iobX/mcbX/mcX>/銆?
鈥渇ormat鈥濈洰褰曟弿杩?perf_event_attr 缁撴瀯鐨?config锛堜簨浠?ID锛夈€乧onfig1锛堜唬鐞?ID锛夊瓧娈电殑鏍煎紡銆傗€渆vents鈥濈洰褰曟彁渚涙墍鏈夊彈鏀寔浜嬩欢绫诲瀷鐨勯厤缃ā鏉匡紝鍙笌 perf 宸ュ叿涓€璧蜂娇鐢ㄣ€備緥濡傦紝鈥渓3c0/bank-fifo-full/鈥濈瓑浠蜂簬鈥渓3c0/config=0x0b/鈥濄€?
澶у鏁?SoC PMU 閮芥湁涓€浠界敤浜庣洃瑙嗙壒瀹氭暟鎹€氳矾鎬ц兘鐨勭壒瀹?agent ID 鍒楄〃銆備緥濡傦紝L3 缂撳瓨鐨?agent 鍙互鏄煇涓壒瀹?CPU 鎴栨煇涓?I/O 妗ャ€傛瘡涓?PMU 閮芥湁涓€缁?2 涓瘎瀛樺櫒锛岃兘澶熷睆钄借姹傛潵婧?agent銆傝嫢璁剧疆浜嗕笌鏌?agent 瀵瑰簲鐨勪綅鍙锋墍瀵瑰簲鐨勪綅锛屽垯浠呭綋璇ヤ簨浠剁敱鏉ヨ嚜璇?agent 鐨勮姹傚紩璧锋椂鎵嶄細璁℃暟銆傛瘡涓?agent ID 浣嶄笌鈥渃onfig1鈥濆瓧娈典腑鐨勭浉搴斾綅鍛堝弽鐩告槧灏勩€傞粯璁ゆ儏鍐典笅锛屼簨浠朵細瀵规墍鏈?agent 璇锋眰璁℃暟锛坈onfig1 = 0x0锛夈€傚悇 PMU 鍙楁敮鎸佺殑鎵€鏈?agent锛岃鍙傞槄 APM X-Gene 鐢ㄦ埛鎵嬪唽銆?
姣忎釜 perf 椹卞姩杩樻彁渚涒€渃pumask鈥漵ysfs 灞炴€э紝鍏朵腑鍖呭惈灏嗙敤浜庡鐞嗘墍鏈?PMU 浜嬩欢鐨勫崟涓?CPU ID銆?
```

 / # perf list | grep -e l3c -e iob -e mcb -e mc
   l3c0/ackq-full/                                    [Kernel PMU event]
 <...>
   mcb1/mcb-csw-stall/                                [Kernel PMU event]

 / # perf stat -a -e l3c0/read-miss/,mcb1/csw-write-request/ sleep 1

 / # perf stat -a -e l3c0/read-miss,config1=0xfffffffffffffffe/ sleep 1

```
璇ラ┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝も€減erf record鈥濇棤娉曞伐浣溿€備笉鏀寔鎸変换鍔★紙涓嶅甫鈥?a鈥濓級鐨?perf 浼氳瘽銆?