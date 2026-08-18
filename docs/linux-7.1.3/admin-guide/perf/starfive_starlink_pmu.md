## StarFive StarLink 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?

StarFive StarLink 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛変綅浜?StarLink 涓€鑷存€х墖涓婄綉缁滐紙CNoC锛変腑锛?璇ョ綉缁滃皢澶氫釜 CPU 闆嗙兢涓?L3 鍐呭瓨绯荤粺杩炴帴璧锋潵銆?
璇?uncore PMU 鏀寔婧㈠嚭涓柇銆佹渶澶?16 涓彲缂栫▼ 64bit 浜嬩欢璁℃暟鍣紝浠ュ強涓€涓?鐙珛鐨?64bit 鍛ㄦ湡璁℃暟鍣ㄣ€侾MU 鍙兘閫氳繃鍐呭瓨鏄犲皠 I/O锛圡MIO锛夎闂紝骞朵笖瀵?杩炴帴鍒板悓涓€ PMU 鐨勬牳蹇冩潵璇存槸鍏变韩鐨勩€?
```

  /sys/bus/event_source/devices/starfive_starlink_pmu/events/

```
椹卞姩鍦?sysfs 鐨勨€渃pumask鈥濈洰褰曚腑鏆撮湶鐢ㄤ簬澶勭悊 PMU 浜嬩欢鐨?cpu

```

  /sys/bus/event_source/devices/starfive_starlink_pmu/cpumask/

```
椹卞姩鍦?sysfs 鐨勨€渇ormat鈥濈洰褰曚腑鎻忚堪 config锛堜簨浠?ID锛夌殑鏍煎紡

```

  /sys/bus/event_source/devices/starfive_starlink_pmu/format/

```
```

	$ perf list

	starfive_starlink_pmu/cycles/                      [Kernel PMU event]
	starfive_starlink_pmu/read_hit/                    [Kernel PMU event]
	starfive_starlink_pmu/read_miss/                   [Kernel PMU event]
	starfive_starlink_pmu/read_request/                [Kernel PMU event]
	starfive_starlink_pmu/release_request/             [Kernel PMU event]
	starfive_starlink_pmu/write_hit/                   [Kernel PMU event]
	starfive_starlink_pmu/write_miss/                  [Kernel PMU event]
	starfive_starlink_pmu/write_request/               [Kernel PMU event]
	starfive_starlink_pmu/writeback/                   [Kernel PMU event]


	$ perf stat -a -e /starfive_starlink_pmu/cycles/ sleep 1

```
涓嶆敮鎸侀噰鏍枫€傚洜姝や笉鏀寔鈥減erf record鈥濄€備笉鏀寔闄勫姞鍒颁换鍔★紝浠呮敮鎸佺郴缁熻寖鍥寸殑璁℃暟銆?