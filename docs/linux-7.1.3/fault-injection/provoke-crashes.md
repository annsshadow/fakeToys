
## 浣跨敤 Linux 鍐呮牳杞偍娴嬭瘯妯″潡锛圠KDTM锛夎Е鍙戝穿婧?

lkdtm 妯″潡鎻愪緵浜嗕竴涓帴鍙ｏ紝鐢ㄤ簬鍦ㄩ瀹氫箟鐨勪唬鐮佷綅缃腑鏂紙閫氬父瀵艰嚧宕╂簝锛夊唴鏍革紝浠ヨ瘎浼板唴鏍稿紓甯稿鐞嗭紙exception handling锛夌殑鍙潬鎬э紝骞舵祴璇曚娇鐢ㄤ笉鍚岃浆鍌ㄦ柟妗堣幏寰楃殑宕╂簝杞偍锛坈rash dump锛夈€傝妯″潡浣跨敤 KPROBE 鏉ユ彃妗╋紙instrument锛夎Е鍙戜綅缃紝浣嗕篃鍙互閫氳繃 debugfs 鍦ㄦ病鏈?KPROBE 鏀寔鐨勬儏鍐典笅鐩存帴瑙﹀彂鍐呮牳銆?
浣犲彲浠ラ€夋嫨瑙﹀彂鐨勪綅缃紙鈥滃穿婧冪偣鍚嶇О鈥濓紝crash point name锛夊拰鍔ㄤ綔绫诲瀷锛堚€滃穿婧冪偣绫诲瀷鈥濓紝crash point type锛夛紝鏃㈠彲浠ラ€氳繃鎻掑叆妯″潡鏃剁殑妯″潡鍙傛暟锛屼篃鍙互閫氳繃 debugfs 鎺ュ彛銆?
```

	insmod lkdtm.ko [recur_count={>0}] cpoint_name=<> cpoint_type=<>
			[cpoint_count={>0}]

```
recur_count
	鏍堟孩鍑烘祴璇曠殑閫掑綊灞傜骇銆傞粯璁ゆ儏鍐典笅鏍规嵁鍐呮牳閰嶇疆鍔ㄦ€佽绠楋紝鐩爣鏄垰濂藉ぇ鍒拌冻浠ヨ€楀敖鍐呮牳鏍堛€傝鍊煎彲鍦?`/sys/module/lkdtm/parameters/recur_count` 鏌ョ湅銆?
cpoint_name
	鍦ㄥ唴鏍镐腑鐨勪綍澶勮Е鍙戝姩浣溿€傚彲浠ユ槸 INT_HARDWARE_ENTRY銆両NT_HW_IRQ_EN銆両NT_TASKLET_ENTRY銆丗S_SUBMIT_BH銆丮EM_SWAPOUT銆乀IMERADD銆丼CSI_QUEUE_RQ 鎴?DIRECT 涔嬩竴銆?
cpoint_type
	鎸囩ず鍛戒腑宕╂簝鐐规椂瑕侀噰鍙栫殑鍔ㄤ綔銆傜绫诲緢澶氾紝鏈€濂界洿鎺ヤ粠 debugfs 鏌ヨ銆備竴浜涘父瑙佺殑鏄?PANIC銆丅UG銆丒XCEPTION銆丩OOP 鍜?OVERFLOW銆傚畬鏁村垪琛ㄥ弬瑙?`/sys/kernel/debug/provoke-crash/DIRECT` 鐨勫唴瀹广€?
cpoint_count
	鎸囩ず鍦ㄨЕ鍙戝姩浣滀箣鍓嶅穿婧冪偣闇€瑕佽鍛戒腑鐨勬鏁般€傞粯璁ゆ槸 10锛圖IRECT 闄ゅ锛屽畠鎬绘槸绔嬪嵆瑙﹀彂锛夈€?
浣犱篃鍙互閫氳繃鎸傝浇 debugfs 骞跺啓鍏ョ被鍨嬫潵寮曞彂鏁呴殰
```

  mount -t debugfs debugfs /sys/kernel/debug
  echo EXCEPTION > /sys/kernel/debug/provoke-crash/INT_HARDWARE_ENTRY

```
鐗规畩鏂囦欢 `DIRECT` 浼氬湪娌℃湁 KPROBE 鎻掓々鐨勬儏鍐典笅鐩存帴寮曞彂鍔ㄤ綔銆傚綋妯″潡浠ュ涓嬫柟寮忔瀯寤烘椂锛岃繖鏄敮涓€鍙敤鐨勬ā寮?```

  # 涓庡叾璁?BUG 鏉€鎺変綘鐨?shell锛屼笉濡傝瀹冩潃鎺?鈥渃at鈥濓細
  cat <(echo WRITE_RO) >/sys/kernel/debug/provoke-crash/DIRECT

```
