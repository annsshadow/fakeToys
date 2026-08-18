## Ampere SoC 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?


Ampere SoC PMU 鏄竴涓伒寰?Arm CoreSight PMU 鏋舵瀯鐨勯€氱敤 PMU IP銆傚洜姝わ紝璇ラ┍鍔ㄤ綔涓?arm_cspmu 椹卞姩鐨?submodule 瀹炵幇銆傚湪绗竴闃舵锛屽畠鐢ㄤ簬缁熻 AmpereOne 涓婄殑 MCU 浜嬩欢銆?


### MCU PMU 浜嬩欢


PMU 椹卞姩鏀寔涓?"rank"銆?bank" 鍜?"threshold" 璁剧疆杩囨护鍣ㄣ€傛敞鎰忥紝杩囨护鍣ㄦ槸鎸?PMU 瀹炰緥鑰岄潪鎸変簨浠惰缃殑銆?

```

  / # perf list ampere

    ampere_mcu_pmu_0/act_sent/                         [Kernel PMU event]
    <...>
    ampere_mcu_pmu_1/rd_sent/                          [Kernel PMU event]
    <...>

  / # perf stat -a -e ampere_mcu_pmu_0/act_sent,bank=5,rank=3,threshold=2/,ampere_mcu_pmu_1/rd_sent/ \
        sleep 1

```
