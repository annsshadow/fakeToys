## Marvell Odyssey LLC-TAD 鎬ц兘鐩戞帶鍗曞厓锛圥MU UNCORE锛?

姣忎釜 TAD 鎻愪緵鍏釜 64 浣嶈鏁板櫒鐢ㄤ簬鐩戞帶缂撳瓨琛屼负銆傞┍鍔ㄥ缁堜负鎵€鏈?TAD 閰嶇疆鐩稿悓鐨勮鏁板櫒銆傜敤鎴锋渶缁堜細鏈夋晥鍦板湪姣忎釜 TAD 涓繚鐣欏叓涓鏁板櫒涔嬩竴锛屼互渚胯法鎵€鏈?TAD 杩涜瑙傚療銆?浜嬩欢鐨勫彂鐢熸鏁颁細琚仛鍚堬紝骞跺湪宸ヤ綔璐熻浇杩愯缁撴潫鏃跺憟鐜扮粰鐢ㄦ埛銆傞┍鍔ㄦ病鏈夋彁渚涜鐢ㄦ埛瀵?TAD 杩涜鍒嗗尯浠ヤ究涓嶅悓 TAD 鐢ㄤ簬涓嶅悓搴旂敤绋嬪簭鐨勬柟娉曘€?
鎬ц兘浜嬩欢鍙嶆槧浜嗗悇绉嶅唴閮ㄦ垨鎺ュ彛娲诲姩銆傞€氳繃缁勫悎澶氫釜鎬ц兘璁℃暟鍣ㄧ殑鍊硷紝鍙互浠ョ紦瀛樼己澶辩巼銆佺紦瀛樺垎閰嶃€佹帴鍙ｉ噸璇曠巼銆佸唴閮ㄨ祫婧愬崰鐢ㄧ巼绛夋柟寮忔潵琛￠噺缂撳瓨鎬ц兘锛岀瓑绛夈€?
```

        /sys/bus/event_source/devices/tad/events/
        /sys/bus/event_source/devices/tad/format/

```
```

   $ perf list | grep tad
        tad/tad_alloc_any/                                 [Kernel PMU event]
        tad/tad_alloc_dtg/                                 [Kernel PMU event]
        tad/tad_alloc_ltg/                                 [Kernel PMU event]
        tad/tad_hit_any/                                   [Kernel PMU event]
        tad/tad_hit_dtg/                                   [Kernel PMU event]
        tad/tad_hit_ltg/                                   [Kernel PMU event]
        tad/tad_req_msh_in_exlmn/                          [Kernel PMU event]
        tad/tad_tag_rd/                                    [Kernel PMU event]
        tad/tad_tot_cycle/                                 [Kernel PMU event]

   $ perf stat -e tad_alloc_dtg,tad_alloc_ltg,tad_alloc_any,tad_hit_dtg,tad_hit_ltg,tad_hit_any,tad_tag_rd <workload>

```
