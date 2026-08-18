## 鍐呭姟澶勭悊锛圚ousekeeping锛?


CPU 闅旂浼氬皢鍘熸湰鍙兘杩愯鍦ㄤ换鎰?CPU 涓婄殑鍐呮牳宸ヤ綔绉昏蛋銆傚叾鐩稿叧鐗规€х殑鐩殑鏄噺灏戞煇浜涙瀬绔伐浣滆礋杞斤紙渚嬪閮ㄥ垎 DPDK 鐢ㄤ緥锛夋棤娉曞蹇嶇殑 OS 鎶栧姩銆?
CPU 闅旂绉昏蛋鐨勫唴鏍稿伐浣滈€氬父琚弿杩颁负鈥渉ousekeeping锛堝唴鍔″鐞嗭級鈥濓紝鍥犱负瀹冨寘鍚墽琛屾竻鐞嗐€佺粺璁′俊鎭殑缁存姢鍙婁緷璧栧畠浠殑琛屼负銆佸唴瀛橀噴鏀俱€佸悇绉嶅欢杩熸搷浣滅瓑鐨勫熀纭€鎬у伐浣溿€?
鏈夋椂 housekeeping 鍙槸涓€浜涙湭缁戝畾鐨勫伐浣滐紙鏈粦瀹氱殑宸ヤ綔闃熷垪銆佹湭缁戝畾鐨勫畾鏃跺櫒绛夛級锛屽畠浠緢瀹规槗琚垎閰嶅埌闈為殧绂荤殑 CPU銆備絾鏈夋椂 housekeeping 浼氱粦瀹氬埌鐗瑰畾 CPU锛岄渶瑕佺簿宸х殑鎶€宸ф墠鑳藉嵏杞藉埌闈為殧绂?CPU锛圧CU_NOCB銆佽繙绋嬭皟搴﹀櫒 tick 绛夛級銆?
鍥犳锛宧ousekeeping CPU 鍙互鐪嬩綔鏄殧绂?CPU 鐨勫弽闈€傚畠鍙槸涓€涓彲浠ユ墽琛?housekeeping 宸ヤ綔鐨?CPU銆備换浣曟椂鍒婚兘蹇呴』鑷冲皯鏈変竴涓湪绾跨殑 housekeeping CPU銆傛湭琚殧绂荤殑 CPU 浼氳嚜鍔ㄨ鎸囨淳涓?housekeeping銆?
Housekeeping 鐩墠鍒掑垎涓虹敱 `enum hk_type type` 鎻忚堪鐨勫洓涓壒鎬э細

1. HK_TYPE_DOMAIN 鍖归厤閫氳繃 `isolcpus=domain` 鍚姩鍙傛暟鎴?cgroup v2 涓殑闅旂 cpuset 鍒嗗尯鎵ц鐨勮皟搴﹀櫒鍩熼殧绂绘墍绉昏蛋鐨勫伐浣溿€傝繖鍖呮嫭璋冨害鍣ㄨ礋杞藉潎琛°€佹湭缁戝畾鐨勫伐浣滈槦鍒楀拰瀹氭椂鍣ㄣ€?
2. HK_TYPE_KERNEL_NOISE 鍖归厤閫氳繃 `nohz_full=` 鎴?`isolcpus=nohz` 鍚姩鍙傛暟鎵ц鐨?tick 闅旂鎵€绉昏蛋鐨勫伐浣溿€傝繖鍖呮嫭杩滅▼璋冨害鍣?tick銆乿mstat 鍜?lockup watchdog銆?
3. HK_TYPE_MANAGED_IRQ 鍖归厤閫氳繃 `isolcpus=managed_irq` 鎵ц鐨勫彈绠＄悊 IRQ 闅旂鎵€绉昏蛋鐨?IRQ 澶勭悊绋嬪簭銆?
4. HK_TYPE_DOMAIN_BOOT 鍖归厤浠呴€氳繃 `isolcpus=domain` 鎵ц鐨勮皟搴﹀櫒鍩熼殧绂绘墍绉昏蛋鐨勫伐浣溿€傚畠涓?HK_TYPE_DOMAIN 绫讳技锛屽尯鍒湪浜庡畠蹇界暐 cpuset 鎵ц鐨勯殧绂汇€?

## Housekeeping cpumask


Housekeeping cpumask 鍖呭惈浜嗗彲浠ユ墽琛岀敱鐩稿簲闅旂鐗规€хЩ璧扮殑宸ヤ綔鐨?CPU銆傝繖浜?cpumask 鐢变互涓嬪嚱鏁拌繑鍥?```

	const struct cpumask *housekeeping_cpumask(enum hk_type type)

```
榛樿鎯呭喌涓嬶紝濡傛灉鏃㈡湭浣跨敤 `nohz_full=`銆佷篃鏈娇鐢?`isolcpus`锛屼篃鏈娇鐢?cpuset 鐨勯殧绂诲垎鍖猴紙瑕嗙洊澶у鏁扮敤渚嬶級锛岃鍑芥暟杩斿洖 cpu_possible_mask銆?
鍚﹀垯璇ュ嚱鏁拌繑鍥為殧绂荤壒鎬х殑 cpumask 琛ラ泦銆備緥濡傦細

浣跨敤 isolcpus=domain,7 鏃讹紝浠ヤ笅璋冪敤灏嗚繑鍥炲寘鍚墍鏈夊彲鑳?```

	housekeeping_cpumask(HK_TYPE_DOMAIN)

```
绫讳技鍦帮紝浣跨敤 nohz_full=5,6 鏃讹紝浠ヤ笅璋冪敤灏嗚繑鍥炲寘鍚墍鏈?```

	housekeeping_cpumask(HK_TYPE_KERNEL_NOISE)


```
## 涓?cpusets 鐨勫悓姝?

Cpuset 鍙互鍦ㄥ垱寤恒€佷慨鏀规垨鍒犻櫎闅旂鍒嗗尯鏃朵慨鏀?HK_TYPE_DOMAIN housekeeping cpumask銆?
HK_TYPE_DOMAIN cpumask 鐨勪娇鐢ㄨ€呭繀椤荤‘淇濅笌 cpuset 姝ｇ‘鍚屾锛屼互淇濊瘉锛?
1. cpumask 蹇収淇濇寔涓€鑷存€с€?
2. 涓嶄細鍦ㄥ垰琚涓洪殧绂荤殑 CPU 涓婃帓闃熶换浣?housekeeping 宸ヤ綔銆?
3. 鎺掗槦鍒版煇涓潪闅旂 CPU锛堣 CPU 鍒氬垰閫氳繃 cpuset 鍙樹负闅旂锛夌殑寰呭鐞?housekeeping 宸ヤ綔锛屽繀椤诲湪鐩稿叧宸插垱寤?淇敼鐨勯殧绂诲垎鍖哄鐢ㄦ埛绌洪棿鍙敤涔嬪墠琚埛鏂般€?
璇ュ悓姝ョ敱鍩轰簬 RCU 鐨勬柟妗堢淮鎶ゃ€俢puset 鏇存柊渚у湪鏇存柊 HK_TYPE_DOMAIN cpumask 涔嬪悗銆佸埛鏂板緟澶勭悊宸ヤ綔涔嬪墠锛岀瓑寰呬竴涓?RCU 瀹介檺鏈燂紙grace period锛夈€傚湪璇讳晶锛屽繀椤诲皢 housekeeping 鐩爣鐨勯€夋嫨涓庡伐浣滃叆闃熸斁鍦ㄥ悓涓€涓?RCU 璇讳晶涓寸晫鍖哄唴銆?
鏇存柊渚х殑鍏稿瀷甯冨眬绀轰緥濡備笅
```

	rcu_assign_pointer(housekeeping_cpumasks[type], trial);
	synchronize_rcu();
	flush_workqueue(example_workqueue);

```
```

	rcu_read_lock();
	cpu = housekeeping_any_cpu(HK_TYPE_DOMAIN);
	queue_work_on(cpu, example_workqueue, work);
	rcu_read_unlock();

```
