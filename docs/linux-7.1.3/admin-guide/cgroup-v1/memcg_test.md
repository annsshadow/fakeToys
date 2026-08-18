## 鍐呭瓨璧勬簮鎺у埗鍣紙Memcg锛夊疄鐜板蹇樺綍


鏈€鍚庢洿鏂帮細2010/2

鍩哄噯鍐呮牳鐗堟湰锛氬熀浜?2.6.33-rc7-mm锛?4 鐨勫€欓€夌増鏈級銆?

鐢变簬 VM 姝ｅ彉寰楀鏉傦紙鍘熷洜涔嬩竴渚挎槸 memcg鈥︹€︼級锛宮emcg 鐨勮涓?
涔熷崄鍒嗗鏉傘€傛湰鏂囨。鎻忚堪 memcg 鐨勫唴閮ㄨ涓恒€?
璇锋敞鎰忥紝瀹炵幇缁嗚妭鍙兘浼氬彂鐢熷彉鍖栥€?

锛?锛夋湁鍏?API 鐨勪富棰樿鍙傝 `Documentation/admin-guide/cgroup-v1/memory.rst`

## 0. 濡備綍璁板綍鐢ㄩ噺锛?


   浣跨敤浜?2 涓璞°€?

   `page_cgroup`鈥︹€︽瘡涓〉瀵瑰簲涓€涓璞°€?

	鍦ㄥ惎鍔ㄦ垨鍐呭瓨鐑彃鎷旀椂鍒嗛厤锛屽湪鍐呭瓨鐑Щ闄ゆ椂閲婃斁銆?

   `swap_cgroup`鈥︹€︽瘡涓?`swp_entry` 瀵瑰簲涓€椤广€?

	鍦?swapon() 鏃跺垎閰嶏紝鍦?swapoff() 鏃堕噴鏀俱€?

   `page_cgroup` 甯︽湁 USED 浣嶏紝涓旀案杩滀笉浼氬鍚屼竴 `page_cgroup` 閲嶅璁℃暟銆?
   `swap_cgroup` 浠呭湪琚璐圭殑椤垫崲鍑猴紙swapped-out锛夋椂浣跨敤銆?

## 1. 璁¤垂锛圕harge锛?


   涓€涓〉 / `swp_entry` 鍙兘鍦ㄤ互涓嬩綅缃璁¤垂锛坲sage += PAGE_SIZE锛夛細

	mem_cgroup_try_charge()

## 2. 娉ㄩ攢璁¤垂锛圲ncharge锛?


   涓€涓〉 / `swp_entry` 鍙€氳繃浠ヤ笅鍑芥暟琚敞閿€璁¤垂锛坲sage -= PAGE_SIZE锛夛細

	mem_cgroup_uncharge()
	  鍦ㄩ〉鐨勫紩鐢ㄨ鏁伴檷涓?0 鏃惰皟鐢ㄣ€?

	mem_cgroup_uncharge_swap()
	  鍦?`swp_entry` 鐨勫紩鐢ㄨ鏁伴檷涓?0 鏃惰皟鐢ㄣ€傞拡瀵逛氦鎹㈠尯鐨勮璐归殢涔嬫秷澶便€?

## 3. 璁¤垂-鎻愪氦锛坈harge-commit锛?


	Memcg 椤电殑璁¤垂鍒嗕袱姝ヨ繘琛岋細

  - `mem_cgroup_try_charge()`
  - `commit_charge()`

	鍦?`try_charge()` 鏃讹紝灏氫笉瀛樺湪琛ㄧず鈥滄湰椤靛凡琚璐光€濈殑鏍囧織銆?
	姝ゆ椂 usage += PAGE_SIZE銆?

	鍦?`commit()` 鏃讹紝椤典笌 memcg 寤虹珛鍏宠仈銆?

鍦ㄤ笅闈㈢殑璇存槑涓紝鎴戜滑鍋囪 `CONFIG_SWAP=y`銆?

## 4. 鍖垮悕椤碉紙Anonymous锛?


	鍖垮悕椤靛湪浠ヤ笅鎯呭舰鏂板垎閰嶏細

    - 瀵?`MAP_ANONYMOUS` 鏄犲皠鍙戠敓缂洪〉锛坧age fault锛夈€?
    - 鍐欐椂澶嶅埗锛圕opy-On-Write锛夈€?

	4.1 鎹㈠叆锛圫wap-in锛夈€?
	鎹㈠叆鏃讹紝椤靛彇鑷?swap-cache銆傚瓨鍦ㄤ袱绉嶆儏鍐点€?

	(a) 鑻?`SwapCache` 鏄柊鍒嗛厤骞惰璇诲彇鐨勶紝鍒欏畠鏈璁¤垂銆?
	(b) 鑻?`SwapCache` 宸茶杩涚▼鏄犲皠锛屽垯瀹冨凡缁忚璁¤垂銆?

	4.2 鎹㈠嚭锛圫wap-out锛夈€?
	鎹㈠嚭鏃讹紝鍏稿瀷鐨勭姸鎬佽浆鎹㈠涓嬨€?

	(a) 鍔犲叆浜ゆ崲缂撳瓨锛堟爣璁颁负 `SwapCache`锛夈€?
	    `swp_entry` 鐨勫紩鐢ㄨ鏁?+= 1銆?
	(b) 瀹屽叏瑙ｉ櫎鏄犲皠銆?
	    `swp_entry` 鐨勫紩鐢ㄨ鏁?+= PTE 鐨勬暟閲忋€?
	(c) 鍐欏洖浜ゆ崲鍖恒€?
	(d) 浠庝氦鎹㈢紦瀛樺垹闄わ紙绉诲嚭 `SwapCache`锛夈€?
	    `swp_entry` 鐨勫紩鐢ㄨ鏁?-= 1銆?


	鏈€鍚庯紝鍦ㄤ换鍔￠€€鍑烘椂锛?
	(e) 璋冪敤 zap_pte()锛宍swp_entry` 鐨勫紩鐢ㄨ鏁?-= 1 鈫?0銆?

## 5. 椤电紦瀛橈紙Page Cache锛?


	椤电紦瀛橈紙Page Cache锛夊湪浠ヤ笅浣嶇疆琚璐癸細

 - `filemap_add_folio()`銆?

	閫昏緫闈炲父娓呮櫚銆傦紙鍏充簬杩佺Щ锛岃涓嬫枃锛?

	娉ㄦ剰锛?
	  `__filemap_remove_folio()` 鐢?`filemap_remove_folio()`
	  涓?`__remove_mapping()` 璋冪敤銆?

## 6. Shmem锛坱mpfs锛夐〉缂撳瓨


	鐞嗚В shmem 椤电姸鎬佽浆鎹㈢殑鏈€浣虫柟寮忔槸闃呰
	`mm/shmem.c`銆?

	浣嗗 memcg 鍥寸粫 shmem 鐨勮涓哄仛绠€瑕佽鏄庯紝鏈夊姪浜庣悊瑙ｅ叾閫昏緫銆?

	Shmem 鐨勯〉锛堜粎鍙跺瓙椤碉紝涓嶅惈鐩存帴/闂存帴鍧楋級鍙互浣嶄簬锛?

  - shmem inode 鐨?radix-tree锛堝熀鏁版爲锛夈€?
  - `SwapCache`銆?
  - 鍚屾椂浣嶄簬 radix-tree 涓?`SwapCache` 涓€傝繖鍙戠敓鍦ㄦ崲鍏ワ紙swap-in锛夋椂
		浠ュ強鎹㈠嚭锛坰wap-out锛夋椂銆?

	瀹冨湪浠ヤ笅鎯呭舰琚璐癸細

 - 涓€涓柊椤佃娣诲姞鍒?shmem 鐨?radix-tree 涓€?
 - 璇诲彇涓€涓?swp 椤点€傦紙灏嗚璐逛粠 `swap_cgroup` 杞Щ鍒?`page_cgroup`锛?

## 7. 椤佃縼绉伙紙Page Migration锛?


	mem_cgroup_migrate()

## 8. LRU


	姣忎釜 memcg 閮芥嫢鏈夎嚜宸辩殑涓€缁?LRU 鍚戦噺锛堥潪娲昏穬鍖垮悕銆佹椿璺冨尶鍚嶃€?
	闈炴椿璺冩枃浠躲€佹椿璺冩枃浠躲€佷笉鍙洖鏀讹級锛屽叾椤垫潵鑷悇涓妭鐐癸紱
	姣忎釜 LRU 鍦ㄨ memcg 涓庤妭鐐瑰搴旂殑鍗曚竴 `lru_lock` 涓嬪鐞嗐€?

## 9. 鍏稿瀷娴嬭瘯銆?


   閽堝绔炴€侊紙racy锛夋儏鍐电殑娴嬭瘯銆?

### 9.1 涓?memcg 璁剧疆杈冨皬闄愬埗銆?


	杩涜绔炴€佹祴璇曟椂锛屽皢 memcg 鐨勯檺鍒惰寰楀緢灏忥紙鑰岄潪 GB 绾э級鏄釜涓嶉敊鐨勬祴璇曘€?
	鍦?xKB 鎴?xxMB 绾у埆鐨勯檺鍒朵笅鑳藉彂鐜板ぇ閲忕珵鎬併€?

	锛堝唴瀛樺湪 GB 绾т笌 MB 绾т笅鐨勮涓鸿〃鐜板樊寮傚緢澶с€傦級

### 9.2 Shmem


	鍘嗗彶涓婏紝memcg 瀵?shmem 鐨勫鐞嗚緝宸紝鎴戜滑涔熷湪姝ら亣鍒拌繃涓€浜涢棶棰樸€?
	杩欐槸鍥犱负 shmem 鏃㈡槸椤电紦瀛橈紝鍙堝彲鑳芥槸 `SwapCache`銆備娇鐢?shmem/tmpfs
	杩涜娴嬭瘯濮嬬粓鏄釜濂介€夋嫨銆?

### 9.3 杩佺Щ锛圡igration锛?


	瀵逛簬 NUMA锛岃縼绉绘槸鍙︿竴涓壒渚嬨€備负渚夸簬娴嬭瘯锛屽彲浣跨敤 cpuset
```

		mount -t cgroup -o cpuset none /opt/cpuset

		mkdir /opt/cpuset/01
		echo 1 > /opt/cpuset/01/cpuset.cpus
		echo 0 > /opt/cpuset/01/cpuset.mems
		echo 1 > /opt/cpuset/01/cpuset.memory_migrate
		mkdir /opt/cpuset/02
		echo 1 > /opt/cpuset/02/cpuset.cpus
		echo 1 > /opt/cpuset/02/cpuset.mems
		echo 1 > /opt/cpuset/02/cpuset.memory_migrate

	In above set, when you moves a task from 01 to 02, page migration to
	node 0 to node 1 will occur. Following is a script to migrate all
	under cpuset.::

		--
		move_task()
		{
		for pid in $1
		do
			/bin/echo $pid >$2/tasks 2>/dev/null
			echo -n $pid
			echo -n " "
		done
		echo END
		}

		G1_TASK=`cat ${G1}/tasks`
		G2_TASK=`cat ${G2}/tasks`
		move_task "${G1_TASK}" ${G2} &
		--

```

### 9.4 鍐呭瓨鐑彃鎷旓紙Memory hotplug锛?


	memory hotplug 娴嬭瘯鏄竴绉嶄笉閿欑殑娴嬭瘯銆?
```

		# echo offline > /sys/devices/system/memory/memoryXXX/state

	(XXX is the place of memory)

	This is an easy way to test page migration, too.

```

### 9.5 宓屽 cgroup锛坣ested cgroups锛?


```

		mkdir /opt/cgroup/01/child_a
		mkdir /opt/cgroup/01/child_b

		set limit to 01.
		add limit to 01/child_b
		run jobs under child_a and child_b

	create/delete following groups at random while jobs are running::

		/opt/cgroup/01/child_a/child_aa
		/opt/cgroup/01/child_b/child_bb
		/opt/cgroup/01/child_c

	running new jobs in new group is also good.

```

### 9.6 涓庡叾浠栧瓙绯荤粺涓€璧锋寕杞?


	涓庡叾浠栧瓙绯荤粺涓€璧锋寕杞芥槸涓€涓笉閿欑殑娴嬭瘯锛屽洜涓轰笌鍏朵粬 cgroup 瀛愮郴缁?
	涔嬮棿瀛樺湪绔炴€佷笌閿佷緷璧栥€?
```

		# mount -t cgroup none /cgroup -o cpuset,memory,cpu,devices

	and do task move, mkdir, rmdir etc...under this.

```

### 9.7 swapoff


	闄や氦鎹㈠尯绠＄悊鏈韩鏄?memcg 涓緝澶嶆潅鐨勯儴鍒嗗锛宻wapoff 鏃剁殑鎹㈠叆璋冪敤璺緞
	涔熶笌閫氬父鐨勬崲鍏ヨ矾寰勪笉鍚岋紝鍊煎緱涓撻棬娴嬭瘯銆?

	渚嬪锛屼笅闈㈣繖鏍风殑娴嬭瘯鏄笉閿欑殑锛?
```

		# mount -t cgroup none /cgroup -o memory
		# mkdir /cgroup/test
		# echo 40M > /cgroup/test/memory.limit_in_bytes
		# echo 0 > /cgroup/test/tasks

	Run malloc(100M) program under this. You'll see 60M of swaps.

	(Shell-B)::

		# move all tasks in /cgroup/test to /cgroup
		# /sbin/swapoff -a
		# rmdir /cgroup/test
		# kill malloc task.

	Of course, tmpfs v.s. swapoff test should be tested, too.

```

### 9.8 OOM-Killer锛堝唴瀛樿€楀敖鏉€鎵嬶級


	鐢?memcg 闄愬埗寮曞彂鐨?Out-of-memory 浼氱粓姝㈣ memcg 涓嬬殑浠诲姟銆?
	浣跨敤灞傜骇锛坔ierarchy锛夋椂锛屽眰绾т笅鐨勪换鍔′細琚唴鏍哥粓姝€?

	鍦ㄨ繖绉嶆儏鍐典笅锛屼笉搴旇Е鍙?panic_on_oom锛屼篃涓嶅簲缁堟鍏朵粬缁勭殑浠诲姟銆?

	鍦?memcg 涓嬪紩鍙?OOM 骞朵笉鍥伴毦锛屽涓嬫墍绀恒€?
```

		#swapoff -a
		#echo 50M > /memory.limit_in_bytes

	run 51M of malloc

	Case B) when you use mem+swap limitation::

		#echo 50M > memory.limit_in_bytes
		#echo 50M > memory.memsw.limit_in_bytes

	run 51M of malloc

```

### 9.9 浠诲姟杩佺Щ鏃剁Щ鍔ㄨ璐癸紙Move charges锛?


	涓庝换鍔″叧鑱旂殑璁¤垂鍙殢浠诲姟杩佺Щ涓€璧风Щ鍔ㄣ€?
```

		#mkdir /cgroup/A
		#echo $$ >/cgroup/A/tasks

	run some programs which uses some amount of memory in /cgroup/A.

	(Shell-B)::

		#mkdir /cgroup/B
		#echo 1 >/cgroup/B/memory.move_charge_at_immigrate
		#echo "pid of the program running in group A" >/cgroup/B/tasks

	You can see charges have been moved by reading ``*.usage_in_bytes`` or
	memory.stat of both A and B.

	See 8.2 of Documentation/admin-guide/cgroup-v1/memory.rst to see what value should
	be written to move_charge_at_immigrate.

```

### 9.10 鍐呭瓨闃堝€硷紙Memory thresholds锛?


	鍐呭瓨鎺у埗鍣ㄤ娇鐢?cgroups 鐨勯€氱煡 API 瀹炵幇鍐呭瓨闃堝€笺€?
	浣犲彲浠ヤ娇鐢?tools/cgroup/cgroup_event_listener.c 鏉ユ祴璇曘€?
```

		# mkdir /cgroup/A
		# ./cgroup_event_listener /cgroup/A/memory.usage_in_bytes 5M

	(Shell-B) Add task to cgroup and try to allocate and free memory::

		# echo $$ >/cgroup/A/tasks
		# a="$(dd if=/dev/zero bs=1M count=10)"
		# a=

	You will see message from cgroup_event_listener every time you cross
	the thresholds.

	Use /cgroup/A/memory.memsw.usage_in_bytes to test memsw thresholds.

	It's good idea to test root cgroup as well.

```
