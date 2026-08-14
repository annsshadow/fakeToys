## 鍐呮牳閿?torture 娴嬭瘯鎿嶄綔

## CONFIG_LOCK_TORTURE_TEST

CONFIG_LOCK_TORTURE_TEST 閰嶇疆閫夐」鎻愪緵浜嗕竴涓唴鏍告ā鍧楋紝瀹冧細瀵规牳蹇冨唴鏍哥殑閿佸師璇繍琛?torture 娴嬭瘯銆傚鏋滈渶瑕侊紝鍙互鍦ㄨ娴嬬殑姝ｅ湪杩愯鐨勫唴鏍镐笂浜嬪悗鏋勫缓鍚嶄负 'locktorture' 鐨勫唴鏍告ā鍧椼€傛祴璇曚細鍛ㄦ湡鎬у湴閫氳繃 printk() 杈撳嚭鐘舵€佹秷鎭紝鍙互閫氳繃 dmesg锛堜篃璁哥敤 grep "torture"锛夋煡鐪嬨€傛祴璇曞湪妯″潡鍔犺浇鏃跺惎鍔紝鍦ㄦā鍧楀嵏杞芥椂鍋滄銆傛湰绋嬪簭鍩轰簬 RCU 濡備綍琚?torture 鐨勬柟寮忥紝鍗抽€氳繃 rcutorture銆?
杩欎釜 torture 娴嬭瘯閫氳繃鍒涘缓鑻ュ共鍐呮牳绾跨▼鏉ユā鎷熶笉鍚岀殑涓寸晫鍖鸿涓猴紝杩欎簺绾跨▼鑾峰彇閿佸苟灏嗗叾鎸佹湁鐗瑰畾鐨勪竴娈垫椂闂淬€傞攣涓婄殑浜夌敤绋嬪害鍙互閫氳繃寤堕暱杩欎釜涓寸晫鍖虹殑鎸佹湁鏃堕棿鍜?鎴栧垱寤烘洿澶氱殑 kthread 鏉ユā鎷熴€?
## 妯″潡鍙傛暟

鏈ā鍧楀叿鏈変互涓嬪弬鏁帮細

### Locktorture 涓撶敤

nwriters_stress
		  鐢ㄤ簬瀵圭嫭鍗犻攣鎵€鏈夋潈锛堝啓鑰咃級鏂藉姞鍘嬪姏鐨勬牳绾跨▼鏁伴噺銆傞粯璁ゅ€兼槸鍦ㄧ嚎 CPU 鏁伴噺鐨勪袱鍊嶃€?
nreaders_stress
		  鐢ㄤ簬瀵瑰叡浜攣鎵€鏈夋潈锛堣鑰咃級鏂藉姞鍘嬪姏鐨勬牳绾跨▼鏁伴噺銆傞粯璁や笌鍐欒€呴攣鏁伴噺鐩稿悓銆傚鏋滅敤鎴锋湭鎸囧畾 nwriters_stress锛岄偅涔堣鑰呭拰鍐欒€呴兘涓哄湪绾?CPU 鐨勬暟閲忋€?
torture_type
		  瑕?torture 鐨勯攣绫诲瀷銆傞粯璁ゅ彧 torture 鑷棆閿併€傛湰妯″潡鍙互鐢ㄥ涓嬪瓧绗︿覆鍊?torture 浠ヤ笅閿侊細

       - "lock_busted":
				妯℃嫙涓€涓湁缂洪櫡鐨勯攣瀹炵幇銆?
       - "spin_lock":
				spin_lock() 涓?spin_unlock() 瀵广€?
       - "spin_lock_irq":
				spin_lock_irq() 涓?spin_unlock_irq() 瀵广€?
       - "rw_lock":
				read/write lock() 涓?unlock() rwlock 瀵广€?
       - "rw_lock_irq":
				read/write lock_irq() 涓?unlock_irq()
				rwlock 瀵广€?
       - "mutex_lock":
				mutex_lock() 涓?mutex_unlock() 瀵广€?
       - "rtmutex_lock":
				rtmutex_lock() 涓?rtmutex_unlock() 瀵广€?				鍐呮牳蹇呴』閰嶇疆 CONFIG_RT_MUTEXES=y銆?
       - "rwsem_lock":
				read/write down() 涓?up() 淇″彿閲忓銆?
### Torture 妗嗘灦锛圧CU + 閿侊級

shutdown_secs
		  鍦ㄧ粓姝㈡祴璇曞苟鍏抽棴绯荤粺涔嬪墠杩愯娴嬭瘯鐨勭鏁般€傞粯璁や负闆讹紝鍗崇鐢ㄦ祴璇曠粓姝笌绯荤粺鍏虫満銆傛鑳藉姏瀵硅嚜鍔ㄥ寲娴嬭瘯寰堟湁鐢ㄣ€?
onoff_interval
		  姣忔灏濊瘯鎵ц闅忔満閫夋嫨鐨?CPU 鐑彃鎷旀搷浣滀箣闂寸殑绉掓暟銆傞粯璁や负闆讹紝鍗崇鐢?CPU 鐑彃鎷斻€傚湪 CONFIG_HOTPLUG_CPU=n 鐨勫唴鏍镐腑锛屾棤璁轰负 onoff_interval 鎸囧畾浠€涔堝€硷紝locktorture 閮戒細闈欓粯鍦版嫆缁濇墽琛屼换浣?CPU 鐑彃鎷旀搷浣溿€?
onoff_holdoff
		  鍦ㄥ紑濮?CPU 鐑彃鎷旀搷浣滀箣鍓嶇瓑寰呯殑绉掓暟銆傝繖閫氬父鍙湪鍐呮牳鍐呯疆浜?locktorture 骞跺湪鍚姩鏃惰嚜鍔ㄥ惎鍔ㄦ椂鎵嶆湁鐢紝姝ゆ椂瀹冩湁鍔╀簬閬垮厤璁╁惎鍔ㄦ椂浠ｇ爜琚潵鏉ュ幓鍘荤殑 CPU 鎼炵硦娑傘€傛鍙傛暟浠呭湪鍚敤浜?CONFIG_HOTPLUG_CPU 鏃舵墠鏈夌敤銆?
stat_interval
		  缁熻鐩稿叧 printk() 涔嬮棿鐨勭鏁般€傞粯璁ゆ儏鍐典笅锛宭ocktorture 姣?60 绉掓姤鍛婁竴娆＄粺璁°€傛妸闂撮殧璁句负闆朵細瀵艰嚧缁熻鍙湪璇ユā鍧楀嵏杞芥椂鎵嶈鎵撳嵃銆?
stutter
		  杩愯娴嬭瘯鐒跺悗鍦ㄧ浉鍚岄暱搴︽椂闂村唴鏆傚仠鐨勬椂闀裤€傞粯璁や负 "stutter=5"锛屽嵆澶х害浠ヤ簲绉掔殑闂撮殧杩愯鍜屾殏鍋溿€傛寚瀹?"stutter=0" 浼氫娇娴嬭瘯鎸佺画杩愯鑰屼笉鏆傚仠銆?
shuffle_interval
		  灏嗘祴璇曠嚎绋嬩翰鍜屽埌鐗瑰畾 CPU 瀛愰泦淇濇寔鐨勭鏁帮紝榛樿涓?3 绉掋€備笌 test_no_idle_hz 閰嶅悎浣跨敤銆?
verbose
		  閫氳繃 printk() 鍚敤璇︾粏璋冭瘯鎵撳嵃銆傞粯璁ゅ惎鐢ㄣ€傝繖浜涢澶栦俊鎭ぇ澶氫笌鏉ヨ嚜涓?'torture' 妗嗘灦鐨勯珮灞傞敊璇拰鎶ュ憡鏈夊叧銆?
## 缁熻

```

  spin_lock-torture: Writes:  Total: 93746064  Max/Min: 0/0   Fail: 0
     (A)		    (B)		   (C)		  (D)	       (E)

  (A): Lock type that is being tortured -- torture_type parameter.

  (B): Number of writer lock acquisitions. If dealing with a read/write
       primitive a second "Reads" statistics line is printed.

  (C): Number of times the lock was acquired.

  (D): Min and max number of times threads failed to acquire the lock.

  (E): true/false values if there were errors acquiring the lock. This should
       -only- be positive if there is a bug in the locking primitive's
       implementation. Otherwise a lock should never fail (i.e., spin_lock()).
       Of course, the same applies for (C), above. A dummy example of this is
       the "lock_busted" type.

```
## 鐢ㄦ硶

```

	#!/bin/sh

	modprobe locktorture
	sleep 3600
	rmmod locktorture
	dmesg | grep torture:

```
杈撳嚭鍙互鎵嬪姩妫€鏌?"!!!" 鐨勯敊璇爣蹇椼€傚綋鐒讹紝涔熷彲浠ュ垱寤轰竴涓洿绮惧阀鐨勮剼鏈潵鑷姩妫€鏌ユ绫婚敊璇€?"rmmod" 鍛戒护浼氬己鍒?printk() 鎵撳嵃涓€涓?"SUCCESS"銆?FAILURE" 鎴?"RCU_HOTPLUG" 鎸囩ず銆傚墠涓や釜涓嶈█鑷槑锛岃€屾渶鍚庝竴涓〃绀鸿櫧鐒舵病鏈夐攣瀹氬け璐ワ紝浣嗘娴嬪埌浜?CPU 鐑彃鎷旈棶棰樸€?
鍙﹁锛欴ocumentation/RCU/torture.rst
