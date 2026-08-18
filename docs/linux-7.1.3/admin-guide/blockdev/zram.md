## zram锛氬熀浜庡唴瀛樼殑鍘嬬缉鍧楄澶?


## 绠€浠?


zram 妯″潡浼氬垱寤哄悕涓?/dev/zram<id>锛?id> = 0, 1, ...锛夌殑鍩轰簬鍐呭瓨鐨勫潡璁惧銆?
鍐欏叆杩欎簺纾佺洏鐨勯〉浼氳鍘嬬缉骞剁洿鎺ュ瓨鍌ㄥ湪鍐呭瓨涓€傝繖浜涚鐩樺叿鏈夐潪甯稿揩鐨?I/O
鎬ц兘锛屽苟涓斿帇缂╄兘澶熷甫鏉ュ彲瑙傜殑鍐呭瓨鑺傜渷銆傞儴鍒嗕娇鐢ㄥ満鏅寘鎷?/tmp 瀛樺偍銆佺敤浣?
swap 纾佺洏銆?var 涓嬬殑鍚勭缂撳瓨锛屼互鍙婂彲鑳芥洿澶氱殑鐢ㄩ€斻€?)

鍚勪釜 zram 璁惧鐨勭粺璁′俊鎭€氳繃 /sys/block/zram<id>/ 涓嬬殑 sysfs 鑺傜偣瀵煎嚭銆?

## 鐢ㄦ硶


閰嶇疆鍜岀鐞?zram 璁惧鏈変互涓嬪嚑绉嶆柟寮忥細

a) 浣跨敤 zram 涓?zram_control 鐨?sysfs 灞炴€?
b) 浣跨敤 util-linux 鎻愪緵鐨?zramctl 宸ュ叿锛坲til-linux@vger.kernel.org锛夈€?

鏈枃妗ｄ粎鎻忚堪鈥滄墜鍔ㄢ€濋厤缃?zram 鐨勬楠わ紝鍗?zram 涓?zram_control 鐨?sysfs 灞炴€с€?

鑻ユ兂杩涗竴姝ヤ簡瑙?zramctl锛岃鏌ラ槄 util-linux 鐨勬枃妗ｃ€亃ramctl 鎵嬪唽椤垫垨
`zramctl --help`銆傝娉ㄦ剰锛寊ram 鐨勭淮鎶よ€呭苟涓嶅紑鍙?缁存姢 util-linux 鎴?
zramctl锛屽鏈変换浣曢棶棰樿鑱旂郴 util-linux@vger.kernel.org銆?

涓嬮潰灞曠ず浣跨敤 zram 鐨勫吀鍨嬫楠ゅ簭鍒椼€?

## 璀﹀憡


涓虹畝娲佽捣瑙侊紝涓嬮潰鐨勫ぇ澶氭暟绀轰緥鐪佺暐浜嗛敊璇鏌ラ儴鍒嗐€備絾鏄紝澶勭悊閿欒鏄綘鐨?
鍏ㄩ儴璐ｄ换銆?

zram 鐨?sysfs 灞炴€у湪鍑洪敊鏃舵€绘槸杩斿洖璐熷€笺€傚彲鑳界殑杩斿洖鐮佸垪琛ㄥ涓嬶細

========  =============================================================
-EBUSY	  璇曞浘淇敼璁惧鍒濆鍖栧悗鏃犳硶鍐嶆洿鏀圭殑灞炴€с€傝鍏堥噸缃澶囥€?
-ENOMEM	  zram 鏃犳硶鍒嗛厤瓒冲鐨勫唴瀛樻潵婊¤冻浣犵殑闇€姹傘€?
-EINVAL	  鎻愪緵浜嗘棤鏁堢殑杈撳叆銆?
-EAGAIN	  绋嶅悗閲嶈瘯璇ユ搷浣滐紙渚嬪褰撹瘯鍥惧悓鏃舵墽琛?recompress 涓?writeback 鏃讹級銆?
========  =============================================================

濡傛灉浣犱娇鐢?'echo'锛岃繑鍥炲€肩敱 'echo' 宸ュ叿璁剧疆锛屽洜姝?

```
	echo foo > /sys/block/zram0/comp_algorithm
	if [ $? -ne 0 ]; then
		handle_error
	fi

```
灏辫冻澶熶簡銆?

## 1) 鍔犺浇妯″潡


```
	modprobe zram num_devices=4
```

杩欎細鍒涘缓 4 涓澶囷細/dev/zram{0,1,2,3}

num_devices 鍙傛暟鏄彲閫夌殑锛岀敤浜庡憡璇?zram 搴旈鍏堝垱寤哄灏戣澶囥€傞粯璁ゅ€硷細1銆?

## 2) 閫夋嫨鍘嬬缉绠楁硶


閫氳繃 comp_algorithm 璁惧灞炴€э紝鍙互鏌ョ湅鍙敤浠ュ強褰撳墠閫変腑鐨勶紙浠ユ柟鎷彿鏄剧ず锛?
鍘嬬缉绠楁硶锛屾垨鑰呭湪璁惧鍒濆鍖栧悗鏇存敼鎵€閫夌殑鍘嬬缉绠楁硶锛堣澶囦竴鏃﹀垵濮嬪寲渚挎棤娉曞啀
鏇存敼鍘嬬缉绠楁硶锛夈€?

```
	# 鏄剧ず鏀寔鐨勫帇缂╃畻娉?
	cat /sys/block/zram0/comp_algorithm
	lzo [lz4]

	# 閫夋嫨 lzo 鍘嬬缉绠楁硶
	echo lzo > /sys/block/zram0/comp_algorithm
```

鐩墠锛宍comp_algorithm` 鐨勫唴瀹逛粎鏄剧ず zram 鎵€鏀寔鐨勫帇缂╃畻娉曘€?

## 3) 璁剧疆鍘嬬缉绠楁硶鍙傛暟锛氬彲閫?


鍘嬬缉绠楁硶鍙兘鏀寔閽堝鐗瑰畾鏁版嵁闆嗚繘琛岃皟鏁寸殑鐗瑰畾鍙傛暟銆俍RAM 鎻愪緵浜嗕竴涓?
`algorithm_params` 璁惧灞炴€э紝鐢ㄤ簬鎸夌畻娉曡繘琛屽弬鏁伴厤缃€?

渚嬪锛岃嫢骞插帇缂╃畻娉曟敮鎸?`level` 鍙傛暟銆傛澶栵紝鏌愪簺鍘嬬缉绠楁硶鏀寔棰勮缁冨瓧鍏革紝
浼氭樉钁楁敼鍙樼畻娉曠殑鐗规€с€備负浜嗚鍘嬬缉绠楁硶浣跨敤澶栭儴鐨勯璁粌瀛楀吀锛屼紶鍏ュ畬鏁寸殑

```
	# 浼犲叆棰勮缁?zstd 瀛楀吀鐨勮矾寰?
	echo "algo=zstd dict=/etc/dictionary" > /sys/block/zram0/algorithm_params

	# 鍚屾牱鐨勬柟寮忥紝浣嗕娇鐢ㄧ畻娉曚紭鍏堢骇
	echo "priority=1 dict=/etc/dictionary" > \
		/sys/block/zram0/algorithm_params

	# 浼犲叆棰勮缁?zstd 瀛楀吀璺緞浠ュ強鍘嬬缉绾у埆
	echo "algo=zstd level=8 dict=/etc/dictionary" > \
		/sys/block/zram0/algorithm_params
```

鍙傛暟鏄畻娉曠浉鍏崇殑锛氬苟闈炴墍鏈夌畻娉曢兘鏀寔棰勮缁冨瓧鍏革紝涔熷苟闈炴墍鏈夌畻娉曢兘鏀寔
`level`銆傛澶栵紝瀵逛簬鏌愪簺绠楁硶锛宍level` 鎺у埗鍘嬬缉绾у埆锛堝€艰秺楂樺帇缂╂瘮瓒婂ソ锛?
鏌愪簺绠楁硶鐢氳嚦鍙互鍙栬礋鍊硷級锛涘浜庡彟涓€浜涚畻娉曪紝`level` 鏄姞閫熺骇鍒紙鍊艰秺楂?
鍘嬬缉姣旇秺浣庯級銆?

## 4) 璁剧疆纾佺洏澶у皬


閫氳繃鍚?sysfs 鑺傜偣 'disksize' 鍐欏叆鍊兼潵璁剧疆纾佺洏澶у皬銆傝鍊煎彲浠ユ槸瀛楄妭鏁帮紝
涔熷彲浠ヤ娇鐢ㄥ唴瀛樺悗缂€銆?

```
	# 浠?50MB 鐨勭鐩樺ぇ灏忓垵濮嬪寲 /dev/zram0
	echo $((50*1024*1024)) > /sys/block/zram0/disksize

	# 浣跨敤鍐呭瓨鍚庣紑
	echo 256K > /sys/block/zram0/disksize
	echo 512M > /sys/block/zram0/disksize
	echo 1G > /sys/block/zram0/disksize
```

娉ㄦ剰锛?
鐢变簬鏈熸湜杈惧埌 2:1 鐨勫帇缂╂瘮锛屽垱寤哄ぇ灏忚秴杩囧唴瀛樹袱鍊嶇殑 zram 鎰忎箟涓嶅ぇ銆傝娉ㄦ剰锛?
zram 鍦ㄦ湭琚娇鐢ㄦ椂绾﹀崰鐢ㄧ鐩樺ぇ灏忕殑 0.1% 鍐呭瓨锛屽洜姝よ繃澶х殑 zram 鏄氮璐圭殑銆?

## 5) 璁剧疆鍐呭瓨涓婇檺锛氬彲閫?


閫氳繃鍚?sysfs 鑺傜偣 'mem_limit' 鍐欏叆鍊兼潵璁剧疆鍐呭瓨涓婇檺銆傝鍊煎彲浠ユ槸瀛楄妭鏁帮紝
涔熷彲浠ヤ娇鐢ㄥ唴瀛樺悗缂€銆傛澶栵紝浣犲彲浠ュ湪杩愯鏃舵洿鏀硅鍊笺€?

```
	# 闄愬埗 /dev/zram0 浣跨敤 50MB 鍐呭瓨
	echo $((50*1024*1024)) > /sys/block/zram0/mem_limit

	# 浣跨敤鍐呭瓨鍚庣紑
	echo 256K > /sys/block/zram0/mem_limit
	echo 512M > /sys/block/zram0/mem_limit
	echo 1G > /sys/block/zram0/mem_limit

	# 绂佺敤鍐呭瓨涓婇檺
	echo 0 > /sys/block/zram0/mem_limit
```

## 6) 婵€娲?


```
	mkswap /dev/zram0
	swapon /dev/zram0

	mkfs.ext4 /dev/zram1
	mount /dev/zram1 /tmp
```

## 7) 娣诲姞/绉婚櫎 zram 璁惧


zram 鎻愪緵浜嗕竴涓帶鍒舵帴鍙ｏ紝鏀寔鍔ㄦ€侊紙鎸夐渶锛夋坊鍔犲拰绉婚櫎璁惧銆?

瑕佹坊鍔犱竴涓柊鐨?/dev/zramX 璁惧锛岃瀵?hot_add 灞炴€ф墽琛岃鎿嶄綔銆傝鎿嶄綔浼?
杩斿洖鏂拌澶囩殑璁惧 id锛堟剰鍛崇潃浣犲彲浠ヤ娇鐢?/dev/zram<id>锛夛紝鎴栬€呰繑鍥炰竴涓敊璇爜銆?

```
	cat /sys/class/zram-control/hot_add
	1
```

瑕佺Щ闄ゅ凡鏈夌殑 /dev/zramX 璁惧锛堝叾涓?X 涓鸿澶?id锛?

```
	echo X > /sys/class/zram-control/hot_remove
```

## 8) 缁熻淇℃伅


姣忎釜璁惧鐨勭粺璁′俊鎭綔涓?/sys/block/zram<id>/ 涓嬬殑鍚勭鑺傜偣瀵煎嚭銆?

涓嬮潰鏄凡瀵煎嚭璁惧灞炴€х殑绠€瑕佽鏄庛€傛洿澶氱粏鑺傝闃呰
Documentation/ABI/testing/sysfs-block-zram銆?

======================  ======  ===============================================
Name            	access            description
======================  ======  ===============================================
disksize          	RW	鏄剧ず骞惰缃澶囩殑纾佺洏澶у皬
initstate         	RO	鏄剧ず璁惧鐨勫垵濮嬪寲鐘舵€?
reset             	WO	瑙﹀彂璁惧閲嶇疆
mem_used_max      	WO	閲嶇疆 `mem_used_max` 璁℃暟鍣紙瑙佸悗鏂囷級
mem_limit         	WO	鎸囧畾 ZRAM 鍙敤浜庡瓨鍌ㄥ帇缂╂暟鎹殑鏈€澶у唴瀛橀噺
writeback_limit   	WO	鎸囧畾 zram 鍙互鍐欏嚭鍒板悗绔澶囩殑鏈€澶у啓 IO
				閲忥紝浠?4KB 涓哄崟浣?
writeback_limit_enable  RW	鏄剧ず骞惰缃?writeback_limit 鍔熻兘
writeback_batch_size	RW	鏄剧ず骞惰缃渶澶х殑鍦ㄩ€?writeback 鎿嶄綔鏁伴噺
compressed_writeback	RW	鏄剧ず骞惰缃帇缂?writeback 鍔熻兘
comp_algorithm    	RW	鏄剧ず骞舵洿鏀瑰帇缂╃畻娉?
algorithm_params	WO	璁剧疆鍘嬬缉绠楁硶鍙傛暟
compact           	WO	瑙﹀彂鍐呭瓨瑙勬暣
debug_stat        	RO	璇ユ枃浠剁敤浜?zram 璋冭瘯鐩殑
backing_dev	  	RW	涓?zram 璁剧疆鐢ㄤ簬鍐欏嚭鐨勫悗绔瓨鍌?
idle		  	WO	灏嗗凡鍒嗛厤鐨勬Ы浣嶆爣璁颁负 idle
======================  ======  ===============================================

寤鸿鐢ㄦ埛绌洪棿浣跨敤浠ヤ笅鏂囦欢鏉ヨ鍙栬澶囩粺璁′俊鎭€?

鏂囦欢 /sys/block/zram<id>/stat

琛ㄧず鍧楀眰缁熻淇℃伅銆傜粏鑺傝闃呰 Documentation/block/stat.rst銆?

鏂囦欢 /sys/block/zram<id>/io_stat

璇?stat 鏂囦欢琛ㄧず鏈鍧楀眰缁熻銆佸洜鑰屽湪 zram<id>/stat 鏂囦欢涓笉鍙敤鐨勮澶?I/O
缁熻淇℃伅銆傚畠鐢卞崟琛屾枃鏈粍鎴愶紝鍖呭惈浠ヤ笅浠ョ┖鐧藉垎闅旂殑缁熻椤癸細

 =============    =============================================================
 failed_reads     璇诲彇澶辫触鐨勬鏁?
 failed_writes    鍐欏叆澶辫触鐨勬鏁?
 invalid_io       闈為〉澶у皬瀵归綈鐨?I/O 璇锋眰鏁伴噺
 notify_free      鍙栧喅浜庤澶囦娇鐢ㄥ満鏅紝鍙兘缁熻

                  a) 鐢变簬 swap 妲戒綅閲婃斁閫氱煡鑰岄噴鏀剧殑椤垫暟

                  b) 鐢变簬 bio 鍙戦€佺殑 REQ_OP_DISCARD 璇锋眰鑰岄噴鏀剧殑椤垫暟銆傚墠鑰呭湪
                     閲婃斁 swap 妲戒綅鏃跺彂閫佺粰 swap 鍧楄澶囷紝杩欐剰鍛崇潃璇ョ鐩樻琚?
                     鐢ㄤ綔 swap 纾佺洏銆?

                  b) 鍚庤€呯敱浠?discard 閫夐」鎸傝浇鐨勬枃浠剁郴缁熷湪涓㈠純鏌愪簺鏁版嵁鍧楁椂
                     鍙戦€併€?
 =============    =============================================================

鏂囦欢 /sys/block/zram<id>/mm_stat

璇?mm_stat 鏂囦欢琛ㄧず璁惧鐨?mm 缁熻淇℃伅銆傚畠鐢卞崟琛屾枃鏈粍鎴愶紝鍖呭惈浠ヤ笅浠ョ┖鐧?
鍒嗛殧鐨勭粺璁￠」锛?

 ================ =============================================================
 orig_data_size   瀛樺偍鍦ㄨ纾佺洏涓殑鏁版嵁鐨勬湭鍘嬬缉澶у皬銆?
                  鍗曚綅锛氬瓧鑺?
 compr_data_size  瀛樺偍鍦ㄨ纾佺洏涓殑鏁版嵁鐨勫帇缂╁悗澶у皬
 mem_used_total   涓鸿纾佺洏鍒嗛厤鐨勫唴瀛橀噺銆傝繖鍖呭惈涓鸿纾佺洏鍒嗛厤鐨勫垎閰嶅櫒纰庣墖鍜?
                  鍏冩暟鎹紑閿€銆傚洜姝わ紝鍙互浣跨敤 compr_data_size 鍜岃椤圭粺璁¤绠?
                  鍒嗛厤鍣ㄧ殑绌洪棿鏁堢巼銆?
                  鍗曚綅锛氬瓧鑺?
 mem_limit         ZRAM 鍙敤浜庡瓨鍌ㄥ帇缂╂暟鎹殑鏈€澶у唴瀛橀噺
 mem_used_max      zram 涓哄瓨鍌ㄦ暟鎹墍娑堣€楃殑鏈€澶у唴瀛橀噺
 same_pages        鍐欏叆璇ョ鐩樼殑銆佽鐩稿悓鍏冪礌濉厖鐨勯〉鏁伴噺銆?
                   姝ょ被椤典笉鍒嗛厤鍐呭瓨銆?
 pages_compacted   瑙勬暣杩囩▼涓噴鏀剧殑椤垫暟
 huge_pages	  涓嶅彲鍘嬬缉椤电殑鏁伴噺
 huge_pages_since  zram 寤虹珛浠ユ潵涓嶅彲鍘嬬缉椤电殑鏁伴噺
 ================ =============================================================

鏂囦欢 /sys/block/zram<id>/bd_stat

璇?bd_stat 鏂囦欢琛ㄧず璁惧鐨勫悗绔澶囩粺璁′俊鎭€傚畠鐢卞崟琛屾枃鏈粍鎴愶紝鍖呭惈浠ヤ笅浠?
绌虹櫧鍒嗛殧鐨勭粺璁￠」锛?

 ============== =============================================================
 bd_count	鍐欏叆鍚庣璁惧鐨勬暟鎹ぇ灏忋€?
		鍗曚綅锛?K 瀛楄妭
 bd_reads	浠庡悗绔澶囪鍙栫殑娆℃暟
		鍗曚綅锛?K 瀛楄妭
 bd_writes	鍐欏叆鍚庣璁惧鐨勬鏁?
		鍗曚綅锛?K 瀛楄妭
 ============== =============================================================

## 9) 鍋滅敤


```
	swapoff /dev/zram0
	umount /dev/zram1
```

## 10) 閲嶇疆


```
		echo 1 > /sys/block/zram0/reset
		echo 1 > /sys/block/zram1/reset

		杩欎細閲婃斁涓鸿璁惧鍒嗛厤鐨勬墍鏈夊唴瀛橈紝骞跺皢纾佺洏澶у皬閲嶇疆涓洪浂銆?
		鍦ㄩ噸鏂颁娇鐢ㄨ璁惧涔嬪墠锛屼綘蹇呴』鍐嶆璁剧疆纾佺洏澶у皬銆?
```

## 鍙€夊姛鑳?


### IDLE 椤佃窡韪?


zram 鍐呯疆鏀寔 idle 椤佃窡韪紙鍗冲凡鍒嗛厤浣嗘湭琚娇鐢ㄧ殑椤碉級銆傝鍔熻兘瀵逛緥濡?zram
writeback 绛夐潪甯告湁鐢紝鍙?

```
	echo all > /sys/block/zramX/idle
```

杩欎細灏嗘墍鏈夊凡鍒嗛厤鐨?zram 椤垫爣璁颁负 idle銆傚彧鏈夊綋璇ラ〉锛堝潡锛夎璁块棶锛堜緥濡傝
瑕嗙洊鎴栭噴鏀撅級鏃讹紝idle 鏍囪鎵嶄細琚Щ闄ゃ€傛澶栵紝褰撳惎鐢?CONFIG_ZRAM_TRACK_ENTRY_ACTIME
鏃讹紝鍙互鏍规嵁璺濅笂娆¤闂凡杩囧幓鐨勭鏁板皢椤垫爣璁颁负 idle锛?

```
	echo 86400 > /sys/block/zramX/idle
```

鍦ㄦ湰渚嬩腑锛屾墍鏈夎秴杩?86400 绉掞紙涓€澶╋級鏈璁块棶鐨勯〉灏嗚鏍囪涓?idle銆?

### writeback


閫氳繃 CONFIG_ZRAM_WRITEBACK锛寊ram 鍙互灏?idle/涓嶅彲鍘嬬缉椤靛啓鍏ュ悗绔瓨鍌紝鑰?
涓嶆槸淇濈暀鍦ㄥ唴瀛樹腑銆?

```
	echo /dev/sda5 > /sys/block/zramX/backing_dev
```

鍦ㄨ缃?disksize 涔嬪墠銆傜洰鍓嶅畠浠呮敮鎸佸垎鍖恒€?

```
	echo huge > /sys/block/zramX/writeback
```

```
	echo idle > /sys/block/zramX/writeback
```

閫氳繃璇ュ懡浠わ紝zram 浼氬皢鍐呭瓨涓殑 idle 椤靛啓鍥炲瓨鍌ㄣ€?

姝ゅ锛屽鏋滅敤鎴烽€夋嫨鍙啓鍥?huge 鍜?idle 椤碉紝

```
        echo huge_idle > /sys/block/zramX/writeback
```

濡傛灉鐢ㄦ埛閫夋嫨鍙啓鍥炰笉鍙帇缂╅〉锛堝嵆閭ｄ簺

```
	echo incompressible > /sys/block/zramX/writeback
```

濡傛灉绠＄悊鍛樻兂鎶?zram 璁惧涓殑鏌愪釜鐗瑰畾椤靛啓鍏ュ悗绔澶囷紝

```
	echo "page_index=1251" > /sys/block/zramX/writeback
```

鍦?Linux 6.16 涓紝璇ユ帴鍙ｇ粡鍘嗕簡涓€浜涢噸鏋勩€傞鍏堬紝璇ユ帴鍙ｇ幇鍦ㄥ鍏舵墍鏈夊弬鏁?
鏀寔 `key=value` 鏍煎紡锛坄type=huge_idle` 绛夛級銆傚叾娆★紝寮曞叆浜嗗 `page_indexes`
鐨勬敮鎸侊紝鐢ㄤ簬鎸囧畾瑕佸啓鍥炵殑椤电殑 `LOW-HIGH` 鑼冨洿锛堟垨澶氫釜鑼冨洿锛夈€傝繖鍑忓皯浜嗙郴缁?
璋冪敤鐨勬暟閲忥紝浣嗘洿閲嶈鐨勬槸锛屽畠浣垮緱鏈€浼樼殑鍚庡鐞嗘垚涓哄彲鑳斤細

```
	echo "type=idle" > /sys/block/zramX/writeback
	echo "page_indexes=1-100 page_indexes=200-300" > \
		/sys/block/zramX/writeback
```

鎴戜滑鐜板湪杩樺厑璁告瘡娆¤皟鐢ㄤ紶鍏ュ涓?page_index 鍙傛暟锛屼互鍙婃贩鍚堜娇鐢?

```
	echo page_index=42 page_index=99 page_indexes=100-200 \
		page_indexes=500-700 > /sys/block/zramX/writeback
```

濡傛灉闂瓨璁惧涓婂瓨鍦ㄥぇ閲忓啓 IO锛屽垯鍙兘瀛樺湪闂瓨纾ㄦ崯闂锛屽洜姝ょ鐞嗗憳闇€瑕?
璁捐鍐欏叆闄愬埗锛屼互淇濊瘉鏁翠釜浜у搧鐢熷懡鍛ㄦ湡鍐呯殑瀛樺偍鍋ュ悍銆?

涓鸿В鍐宠繖涓棶棰橈紝zram 鏀寔 "writeback_limit" 鍔熻兘銆?writeback_limit_enable"
鐨勯粯璁ゅ€间负 0锛屽洜姝や笉闄愬埗浠讳綍 writeback銆備篃灏辨槸璇达紝濡傛灉绠＄悊鍛樻兂瑕佸簲鐢?
writeback 棰勭畻锛屼粬浠簲褰?

```
	$ echo 1 > /sys/block/zramX/writeback_limit_enable
```

涓€鏃﹁缃簡 writeback_limit_enable锛屽湪绠＄悊鍛橀€氳繃 /sys/block/zramX/writeback_limit
璁剧疆棰勭畻涔嬪墠锛寊ram 涓嶅厑璁镐换浣?writeback銆?

锛堝鏋滅鐞嗗憳娌℃湁鍚敤 writeback_limit_enable锛岄偅涔堥€氳繃 /sys/block/zramX/writeback_limit
璁剧疆鐨?writeback_limit 鍊煎氨娌℃湁鎰忎箟銆傦級

濡傛灉绠＄悊鍛樻兂鍦ㄩ绠楄€楀敖鍚庡啀娆″厑璁稿啓鍏ワ紝

```
	$ echo $((400<<MB_SHIFT>>4K_SHIFT)) > \
		/sys/block/zram0/writeback_limit
```

濡傛灉绠＄悊鍛樻兂瑕侀檺鍒舵瘡澶?400M 鐨?writeback锛屽彲浠ヨ繖鏍峰仛

```
	$ MB_SHIFT=20
	$ 4K_SHIFT=12
	$ echo $((400<<MB_SHIFT>>4K_SHIFT)) > \
		/sys/block/zram0/writeback_limit.
	$ echo 1 > /sys/block/zram0/writeback_limit_enable
```

```
	$ cat /sys/block/zramX/writeback_limit
```

```
	$ echo 0 > /sys/block/zramX/writeback_limit_enable
```

writeback_limit 璁℃暟浼氬湪浣犻噸缃?zram 鏃讹紙渚嬪绯荤粺閲嶅惎銆乪cho 1 > /sys/block/zramX/reset锛?
澶嶄綅锛屽洜姝よ褰曢噸缃?zram 涔嬪墠鍙戠敓浜嗗灏戞 writeback锛屼互渚垮湪涓嬫璁剧疆鏃跺垎閰?
棰濆鐨?writeback 棰勭畻锛屾槸鐢ㄦ埛鐨勫伐浣溿€?

榛樿鎯呭喌涓嬶紝zram 浠ヨВ鍘嬬缉锛堝師濮嬶級褰㈠紡瀛樺偍鍐欏洖鐨勯〉锛岃繖鎰忓懗鐫€ writeback
鎿嶄綔鍦ㄥ啓鍏ュ悗绔澶囦箣鍓嶉渶瑕佸璇ラ〉杩涜瑙ｅ帇缂┿€傝琛屼负鍙互閫氳繃鍚敤
`compressed_writeback` 鍔熻兘鏉ユ敼鍙橈紝璇ュ姛鑳戒細璁?zram 灏嗗帇缂╁悗鐨勯〉鍐欏叆鍚庣
璁惧锛屼粠鑰岄伩鍏嶈В鍘嬬缉寮€閿€銆傝鍚敤瀹冿紝

```
	$ echo yes > /sys/block/zramX/compressed_writeback
```

璇锋敞鎰忥紝璇ュ姛鑳藉簲鍦?`zramX` 璁惧鍒濆鍖栦箣鍓嶉厤缃€?

鏍规嵁鍚庣璁惧鐨勫瓨鍌ㄧ被鍨嬶紝writeback 鎿嶄綔鍙兘鍙楃泭浜庢洿澶氱殑鍦ㄩ€斿啓璇锋眰锛堟壒閲?
鍐欏叆锛夈€傛渶澶х殑鍦ㄩ€?writeback 鎿嶄綔鏁伴噺鍙互閫氳繃 `writeback_batch_size` 灞炴€?
閰嶇疆銆傝鏇存敼榛樿鍊硷紙涓?32锛夛紝

```
	$ echo 64 > /sys/block/zramX/writeback_batch_size
```

濡傛灉绠＄悊鍛樻兂娴嬮噺鏌愪釜鏃堕棿娈靛唴鐨?writeback 璁℃暟锛屽彲浠ラ€氳繃
/sys/block/zram0/bd_stat 鐨勭涓夊垪鑾风煡銆?

### recompression


閫氳繃 `CONFIG_ZRAM_MULTI_COMP`锛寊ram 鍙互浣跨敤鏇夸唬锛坰econdary锛夊帇缂╃畻娉曞
椤佃繘琛岄噸鏂板帇缂┿€傚叾鍩烘湰鎬濇兂鏄紝鏇夸唬鍘嬬缉绠楁硶鍙互浠ワ紙娼滃湪鐨勶級鏇存參鐨勫帇缂?
瑙ｅ帇缂╅€熷害涓轰唬浠凤紝鎻愪緵鏇村ソ鐨勫帇缂╂瘮銆備緥濡傦紝鏇夸唬鍘嬬缉绠楁硶鍙互鏇存湁鏁堝湴鍘嬬缉
huge 椤碉紙閭ｄ簺榛樿绠楁硶鏈兘鍘嬬缉鐨勯〉锛夈€傚彟涓€涓簲鐢ㄦ槸 idle 椤甸噸鏂板帇缂┾€斺€旈偅浜?
鍐锋暟鎹苟椹荤暀鍦ㄥ唴瀛樹腑鐨勯〉鍙互浣跨敤鏇存湁鏁堢殑绠楁硶閲嶆柊鍘嬬缉锛屼粠鑰屽噺灏?zsmalloc
鐨勫唴瀛樺崰鐢ㄣ€?

閫氳繃 `CONFIG_ZRAM_MULTI_COMP`锛寊ram 鏈€澶氭敮鎸?4 绉嶅帇缂╃畻娉曪細1 涓富绠楁硶鍜?
鏈€澶?3 涓绾х畻娉曘€倆ram 涓诲帇缂╁櫒鍦ㄢ€?) 閫夋嫨鍘嬬缉绠楁硶鈥濅腑宸茶鏄庯紝娆＄骇绠楁硶
閫氳繃 recomp_algorithm 璁惧灞炴€ч厤缃€?

```
	# 鏄剧ず鏀寔鐨勯噸鏂板帇缂╃畻娉?
	cat /sys/block/zramX/recomp_algorithm
	#1: lzo lzo-rle lz4 lz4hc [zstd]
	#2: lzo lzo-rle lz4 [lz4hc] zstd
```

鏇夸唬鍘嬬缉绠楁硶鎸変紭鍏堢骇鎺掑簭銆傚湪涓婁緥涓紝zstd 鐢ㄤ綔绗竴涓浛浠ｇ畻娉曪紝浼樺厛绾т负 1锛?
鑰?lz4hc 琚厤缃负浼樺厛绾?2 鐨勫帇缂╃畻娉曘€傛浛浠ｅ帇缂╃畻娉曠殑浼樺厛绾ф槸鍦ㄩ厤缃畻娉曟椂
鎻愪緵鐨勶細

```
	# 閫夋嫨 zstd 閲嶆柊鍘嬬缉绠楁硶锛屼紭鍏堢骇 1
	echo "algo=zstd priority=1" > /sys/block/zramX/recomp_algorithm

	# 閫夋嫨 deflate 閲嶆柊鍘嬬缉绠楁硶锛屼紭鍏堢骇 2
	echo "algo=deflate priority=2" > /sys/block/zramX/recomp_algorithm
```

`CONFIG_ZRAM_MULTI_COMP` 鍚敤鐨勫彟涓€涓澶囧睘鎬ф槸 `recompress`锛屽畠鎺у埗
閲嶆柊鍘嬬缉銆?

```
	# IDLE 椤甸噸鏂板帇缂╃敱 `idle` 妯″紡婵€娲?
	echo "type=idle priority=1" > /sys/block/zramX/recompress

	# HUGE 椤甸噸鏂板帇缂╃敱 `huge` 妯″紡婵€娲?
	echo "type=huge priority=2" > /sys/block/zram0/recompress

	# HUGE_IDLE 椤甸噸鏂板帇缂╃敱 `huge_idle` 妯″紡婵€娲?
	echo "type=huge_idle priority=1" > /sys/block/zramX/recompress
```

idle 椤电殑鏁伴噺鍙兘寰堝ぇ锛屽洜姝ょ敤鎴风┖闂村彲浠ュ悜 recompress 鏃嬮挳浼犲叆涓€涓ぇ灏?
闃堝€硷紙浠ュ瓧鑺備负鍗曚綅锛夛細zram 灏嗗彧閲嶆柊鍘嬬缉

```
	# 閲嶆柊鍘嬬缉澶т簬 3000 瀛楄妭鐨勬墍鏈夐〉
	echo "threshold=3000 priority=1" > /sys/block/zramX/recompress

	# 閲嶆柊鍘嬬缉澶т簬 2000 瀛楄妭鐨?idle 椤?
	echo "type=idle threshold=2000 priority=1" > \
		/sys/block/zramX/recompress
```

涔熷彲浠ラ檺鍒?zram 閲嶆柊鍘嬬缉鐨勯〉鏁帮細

```
	echo "type=huge_idle priority=1 max_pages=42" > \
		/sys/block/zramX/recompress
```

寤鸿濮嬬粓鎸囧畾 `priority` 鍙傛暟銆傝櫧鐒朵篃鍙互鎸囧畾 `algo` 鍙傛暟锛岃 `zram` 閫氳繃
绠楁硶鍚嶇О鏉ョ‘瀹氫紭鍏堢骇锛屼絾骞朵笉鎺ㄨ崘杩欐牱鍋氾紝鍥犱负褰撳悓涓€绠楁硶浠ヤ笉鍚屼紭鍏堢骇閰嶇疆鏃?
锛堜緥濡備笉鍚屽弬鏁帮級鍙兘瀵艰嚧鎰忔兂涓嶅埌鐨勭粨鏋溿€俙priority` 鏄繚璇佷娇鐢ㄩ鏈熺畻娉曠殑
鍞竴鏂瑰紡銆?

## 鍐呭瓨璺熻釜


閫氳繃 CONFIG_ZRAM_MEMORY_TRACKING锛岀敤鎴峰彲浠ヤ簡瑙?zram 鍧楃殑淇℃伅銆傚畠瀵逛簬閫氳繃
*pagemap 鎹曡幏杩涚▼鐨勫喎椤垫垨涓嶅彲鍘嬬缉椤靛彲鑳藉緢鏈夌敤銆?

濡傛灉鍚敤璇ュ姛鑳斤紝浣犲彲浠ラ€氳繃濡備笅鏂瑰紡鏌ョ湅鍧楃姸鎬?

```
	  300    75.033841 .wh...
	  301    63.806904 s.....
	  302    63.806919 ..hi..
	  303    62.801919 ....r.
	  304   146.781902 ..hi.n
```

绗竴鍒?
	zram 鐨勫潡绱㈠紩銆?
绗簩鍒?
	鑷郴缁熷惎鍔ㄤ互鏉ョ殑璁块棶鏃堕棿
绗笁鍒?
	鍧楃殑鐘舵€侊細

	s:
		鐩稿悓椤?
	w:
		宸插啓鍏ュ悗绔瓨鍌ㄧ殑椤?
	h:
		huge 椤?
	i:
		idle 椤?
	r:
		宸查噸鏂板帇缂╃殑椤碉紙娆＄骇鍘嬬缉绠楁硶锛?
	n:
		娌℃湁浠讳綍锛堝寘鎷绾х殑锛夌畻娉曡兘澶熷帇缂╁畠

涓婅堪绀轰緥鐨勭涓€琛岃〃绀虹 300 涓潡鍦?75.033841 绉掓椂琚闂紝涓旇鍧楃殑鐘舵€佷负
huge锛屽洜姝ゅ畠琚啓鍥炲悗绔瓨鍌ㄣ€傝繖鏄竴涓皟璇曞姛鑳斤紝浠讳綍浜洪兘涓嶈渚濊禆瀹冭兘姝ｅ父
宸ヤ綔銆?

Nitin Gupta
ngupta@vflare.org
