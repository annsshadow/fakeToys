## CPU 鎷撴墤淇℃伅濡備綍閫氳繃 sysfs 瀵煎嚭


CPU 鎷撴墤淇℃伅閫氳繃 sysfs 瀵煎嚭銆傛潯鐩紙灞炴€э級绫讳技浜庢煇浜涙灦鏋勭殑 /proc/cpuinfo 杈撳嚭銆傚畠浠?浣嶄簬 /sys/devices/system/cpu/cpuX/topology/銆傝鍙傝€?ABI 鏂囦欢锛?Documentation/ABI/stable/sysfs-devices-system-cpu銆?
涓庢灦鏋勬棤鍏崇殑浠ｇ爜 drivers/base/topology.c 瀵煎嚭杩欎簺灞炴€с€備絾鏄紝涓?die銆乧luster銆乥ook 鍜?drawer 灞傜骇鐩稿叧鐨?sysfs 鏂囦欢锛屽彧鏈夊湪鏋舵瀯鎸夊涓嬫墍杩版彁渚涗簡鐩稿叧瀹忔椂鎵嶄細琚垱寤恒€?
瑕佹敮鎸佽鐗规€э紝鏋舵瀯蹇呴』瀹氫箟浠ヤ笅閮ㄥ垎瀹忥細
```

	#define topology_physical_package_id(cpu)
	#define topology_die_id(cpu)
	#define topology_cluster_id(cpu)
	#define topology_core_id(cpu)
	#define topology_book_id(cpu)
	#define topology_drawer_id(cpu)
	#define topology_sibling_cpumask(cpu)
	#define topology_core_cpumask(cpu)
	#define topology_cluster_cpumask(cpu)
	#define topology_die_cpumask(cpu)
	#define topology_book_cpumask(cpu)
	#define topology_drawer_cpumask(cpu)

```
`**_id` 瀹忕殑绫诲瀷涓?int銆?`**_cpumask` 瀹忕殑绫诲瀷涓?`(const) struct cpumask *`銆傚悗鑰呭搴旂浉搴旂殑 `**_siblings` sysfs
灞炴€э紙topology_sibling_cpumask() 闄ゅ锛屽畠瀵瑰簲 thread_siblings锛夈€?
涓哄湪鎵€鏈夋灦鏋勪笂淇濇寔涓€鑷达紝include/linux/topology.h 涓轰笂杩颁换浣曟湭琚?include/asm-XXX/topology.h 瀹氫箟鐨勫畯鎻愪緵榛樿瀹氫箟锛?
1) topology_physical_package_id: -1
2) topology_die_id: -1
3) topology_cluster_id: -1
4) topology_core_id: 0
5) topology_book_id: -1
6) topology_drawer_id: -1
7) topology_sibling_cpumask: 浠呯粰瀹氱殑 CPU
8) topology_core_cpumask: 浠呯粰瀹氱殑 CPU
9) topology_cluster_cpumask: 浠呯粰瀹氱殑 CPU
10) topology_die_cpumask: 浠呯粰瀹氱殑 CPU
11) topology_book_cpumask:  浠呯粰瀹氱殑 CPU
12) topology_drawer_cpumask: 浠呯粰瀹氱殑 CPU

姝ゅ锛孋PU 鎷撴墤淇℃伅鍦?/sys/devices/system/cpu 涓嬫彁渚涳紝骞跺寘鍚互涓嬫枃浠躲€傝緭鍑虹殑鍐呴儴鏉ユ簮
鍦ㄦ嫭鍙凤紙鈥淸]鈥濓級涓€?
    =========== ==========================================================
    kernel_max: 鍐呮牳閰嶇疆鍏佽鐨勬渶澶?CPU 绱㈠紩銆?		[NR_CPUS-1]

    offline:	鍥犲凡琚儹鎻掓嫈锛圚OTPLUGGED锛夊叧闂垨瓒呭嚭鍐呮牳閰嶇疆
		锛堜笂闈㈢殑 kernel_max锛夊厑璁哥殑 CPU 鏁伴噺闄愬埗鑰屼笉鍦ㄧ嚎鐨?CPU銆?		[~cpu_online_mask + cpus >= NR_CPUS]

    online:	鍦ㄧ嚎涓旀鍦ㄨ璋冨害鐨?CPU [cpu_online_mask]

    possible:	宸插垎閰嶈祫婧愩€佽嫢瀛樺湪鍒欏彲琚甫鍏ュ湪绾跨殑 CPU銆俒cpu_possible_mask]

    present:	宸茶璇嗗埆涓虹郴缁熶腑瀛樺湪鐨?CPU銆俒cpu_present_mask]
    =========== ==========================================================

涓婅堪杈撳嚭鐨勬牸寮忓吋瀹?cpulist_parse() [鍙傝 <linux/cpumask.h>]銆備笅闈㈢粰鍑轰竴浜涚ず渚嬨€?
鍦ㄦ绀轰緥涓紝绯荤粺涓湁 64 涓?CPU锛屼絾 cpu 32-63 瓒呭嚭浜嗗唴鏍告渶澶у€硷紝璇ユ渶澶у€肩敱 NR_CPUS
閰嶇疆閫夐」锛堜负 32锛夐檺鍒朵负 0..31銆傚彟璇锋敞鎰忥紝CPU 2 涓?4-31 涓嶅湪绾匡紝浣嗗彲浠ヨ
```

     kernel_max: 31
        offline: 2,4-31,32-63
         online: 0-1,3
       possible: 0-31
        present: 0-31

```
鍦ㄦ绀轰緥涓紝NR_CPUS 閰嶇疆閫夐」涓?128锛屼絾鍐呮牳浠?possible_cpus=144 鍚姩銆傜郴缁熶腑鏈?4 涓?CPU锛宑pu2 琚墜鍔ㄧ绾匡紙涓旀槸鍞竴鍙互琚甫鍏ュ湪绾跨殑 CPU锛?```

     kernel_max: 127
        offline: 2,4-127,128-143
         online: 0-1,3
       possible: 0-127
        present: 0-3

```
鍙傝 Documentation/core-api/cpu_hotplug.rst 浜嗚В possible_cpus=NUM 鍐呮牳鍚姩鍙傛暟浠ュ強
鍏充簬鍚勭 cpumask 鐨勬洿澶氫俊鎭€?