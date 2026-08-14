## slab 鍒嗛厤鍣ㄧ畝鏄庣敤鎴锋寚鍗?

slab 鍒嗛厤鍣ㄥ寘鍚畬鏁寸殑璋冭瘯鏀寔锛堝湪鏋勫缓鏃跺惎鐢?CONFIG_SLUB_DEBUG=y锛夛紝浣嗛粯璁ゅ叧闂紙闄ら潪
鏋勫缓鏃跺惎鐢ㄤ簡 CONFIG_SLUB_DEBUG_ON=y锛夈€備綘鍙互浠呬负閫夊畾鐨?slab 鍚敤璋冭瘯锛屼互閬垮厤瀵规暣浣撶郴缁?鎬ц兘閫犳垚褰卞搷锛屽惁鍒欏彲鑳戒娇 bug 鏇撮毦鍙戠幇銆?
瑕佸紑鍚皟璇曪紝鍙互鍚戝唴鏍稿懡浠よ娣诲姞 `slab_debug` 閫夐」銆傝繖灏嗕负鎵€鏈?slab 鍚敤瀹屾暣璋冭瘯銆?
閫氬父闅忓悗浼氫娇鐢?`slabinfo` 鍛戒护鑾峰彇缁熻鏁版嵁骞跺 slab 鎵ц鎿嶄綔銆傞粯璁ゆ儏鍐典笅 `slabinfo` 鍙?鍒楀嚭鍏朵腑鍖呭惈鏁版嵁鐨?slab銆傝繍琛屽懡浠ゆ椂璇峰弬瑙?"slabinfo -h" 浠ヤ簡瑙ｆ洿澶氶€夐」銆俙slabinfo` 鍙?閫氳繃浠ヤ笅鏂瑰紡缂栬瘧锛?
```

	gcc -o slabinfo tools/mm/slabinfo.c

```
`slabinfo` 鐨勬煇浜涙搷浣滄ā寮忚姹傚湪鍐呮牳鍛戒护琛屼笂鍚敤 slub 璋冭瘯銆備緥濡傦紝鍦ㄦ湭寮€鍚皟璇曟椂涓嶄細鏈?璺熻釜淇℃伅鍙敤锛屽苟涓斿鏋滄湭寮€鍚皟璇曪紝楠岃瘉鍙兘閮ㄥ垎鎵ц銆?
### slab_debug 鐨勪竴浜涙洿楂樼骇鐢ㄦ硶锛?

鍙互鍚?`slab_debug` 鎻愪緵鍙傛暟銆傚鏋滄湭鎸囧畾浠讳綍鍙傛暟锛屽垯鍚敤瀹屾暣璋冭瘯銆傛牸寮忥細

slab_debug=<Debug-Options>
	涓烘墍鏈?slab 鍚敤閫夐」

slab_debug=<Debug-Options>,<slab name1>,<slab name2>,...
	浠呬负閫夊畾鐨?slab 鍚敤閫夐」锛堥€楀彿鍚庢棤绌烘牸锛?
鍙互缁欏嚭閽堝鎵€鏈?slab 鎴栭€夊畾 slab 鐨勫涓€夐」鍧楋紝閫夐」鍧椾箣闂寸敤 ';' 鍒嗛殧銆傛渶鍚庝竴涓€滄墍鏈?slab鈥濆潡搴旂敤浜庨櫎鍖归厤鏌愪釜鈥滈€夊畾 slab鈥濆潡涔嬪鐨勬墍鏈?slab銆傚尮閰?slab 鍚嶇О鐨勭涓€涓€滈€夊畾 slab鈥?鍧楃殑閫夐」浼氳搴旂敤銆?
```

	F		Sanity checks on (enables SLAB_DEBUG_CONSISTENCY_CHECKS
			Sorry SLAB legacy issues)
	Z		Red zoning
	P		Poisoning (object and padding)
	U		User tracking (free and alloc)
	T		Trace (please only use on single slabs)
	A		Enable failslab filter mark for the cache
	O		Switch debugging off for caches that would have
			caused higher minimum slab orders
	-		Switch all debugging off (useful if the kernel is
			configured with CONFIG_SLUB_DEBUG_ON)

```
```

	slab_debug=FZ

```
```

	slab_debug=,dentry

```
浠ヤ粎鍦?dentry 缂撳瓨涓婂惎鐢ㄨ皟璇曘€備綘鍙互鍦?slab 鍚嶇О鏈熬浣跨敤鏄熷彿锛屼互瑕嗙洊鎵€鏈夊叿鏈夌浉鍚屽墠缂€鐨?slab銆備緥濡傦紝浠ヤ笅鏄浣曞 dentry 缂撳瓨浠ュ強鎵€鏈?kmalloc 杩涜姣掑寲

```

	slab_debug=P,kmalloc-*,dentry

```
Red zoning 鍜岃窡韪彲鑳戒細閲嶆柊瀵归綈 slab銆傛垜浠彲浠ュ彧搴旂敤鍋ュ叏鎬ф鏌?
```

	slab_debug=F,dentry

```
璋冭瘯閫夐」鍙兘浼氬洜涓哄瓨鍌ㄥ厓鏁版嵁锛堜緥濡傦紝瀵硅薄澶у皬涓?PAGE_SIZE 鐨勭紦瀛橈級鑰岃姹傛渶灏忓彲鑳界殑 slab
闃舵暟澧炲姞銆傝繖鍦ㄤ綆鍐呭瓨鎯呭喌鎴栧唴瀛橀珮搴︾鐗囧寲鏃舵洿鏈夊彲鑳藉鑷?slab 鍒嗛厤閿欒銆備负浜?
```

	slab_debug=O

```
浣犲彲浠ヤ娇鐢ㄩ€夐」鍧楀皢涓嶅悓閫夐」搴旂敤浜庝笉鍚岀殑 slab 鍚嶇О鍒楄〃銆傝繖灏嗕负 dentry 鍚敤 red zoning锛屽苟涓?
```

	slab_debug=Z,dentry;U,kmalloc-*

```
浣犱篃鍙互閫氳繃鎸囧畾鍏ㄥ眬璋冭瘯閫夐」鍚庤窡涓€涓?slab 鍚嶇О鍒楄〃锛屼负闄ゆ煇浜涜璁や负瀵规€ц兘杩囦簬鍏抽敭銆佷笉闇€瑕?璋冭瘯鐨勭紦瀛樹箣澶栫殑鎵€鏈夌紦瀛樺惎鐢ㄩ€夐」锛堜緥濡傚仴鍏ㄦ€ф鏌ュ拰姣掑寲锛?
```

	slab_debug=FZ;-,zs_handle,zspage

```
slab 姣忎釜璋冭瘯閫夐」鐨勭姸鎬佸彲浠ュ湪鐩稿簲鐨勬枃浠朵腑鎵惧埌

```

	/sys/kernel/slab/<slab name>/

```
濡傛灉鏂囦欢鍖呭惈 1锛屽垯璇ラ€夐」宸插惎鐢紝0 琛ㄧず宸茬鐢ㄣ€俤ebug

```

	F	sanity_checks
	Z	red_zone
	P	poison
	U	store_user
	T	trace
	A	failslab

```
failslab 鏂囦欢鏄彲鍐欑殑锛屽洜姝ゅ啓鍏?1 鎴?0 灏嗗湪杩愯鏃跺惎鐢ㄦ垨绂佺敤璇ラ€夐」銆傚鏋滅紦瀛樻槸鍒悕锛屽啓鍏?杩斿洖 -EINVAL銆備娇鐢ㄨ窡韪椂瑕佸皬蹇冿細瀹冨彲鑳戒細杈撳嚭澶ч噺淇℃伅锛屽鏋滃湪閿欒鐨?slab 涓婁娇鐢ㄥ垯姘歌繙涓嶄細
鍋滄銆?
## Slab 鍚堝苟


濡傛灉鏈寚瀹氳皟璇曢€夐」锛屽垯 SLUB 鍙兘浼氬皢鐩镐技鐨?slab 鍚堝苟鍦ㄤ竴璧凤紝浠ュ噺灏戝紑閿€骞舵彁楂樺璞＄殑缂撳瓨
鐑害銆俙slabinfo -a` 鏄剧ず鍝簺 slab 琚悎骞跺湪涓€璧枫€?
## Slab 楠岃瘉


濡傛灉鍐呮牳浠?slab_debug 鍚姩锛孲LUB 鍙互楠岃瘉鎵€鏈夊璞°€備负姝わ紝浣犲繀椤绘嫢鏈?`slabinfo` 宸ュ叿銆?鐒跺悗浣犲彲浠ユ墽琛?
```

	slabinfo -v

```
杩欏皢娴嬭瘯鎵€鏈夊璞°€傝緭鍑哄皢鐢熸垚鍒?syslog銆?
濡傛灉鍚姩鏃舵湭鍚敤 slab 璋冭瘯锛岃繖涔熶互鏇村彈闄愮殑鏂瑰紡宸ヤ綔銆傚湪杩欑鎯呭喌涓嬶紝`slabinfo -v` 鍙祴璇?鎵€鏈夊彲杈惧璞°€傞€氬父杩欎簺瀵硅薄浣嶄簬 cpu slab 鍜岄儴鍒?slab 涓€傚湪闈炶皟璇曟儏鍐典笅锛孲LUB 涓嶄細璺熻釜
瀹屾暣 slab銆?
## 鑾峰彇鏇撮珮鎬ц兘


鍦ㄦ煇绉嶇▼搴︿笂锛孲LUB 鐨勬€ц兘鍙楀埌闇€瑕佸伓灏旇幏鍙?list_lock 鏉ュ鐞嗛儴鍒?slab 鐨勯檺鍒躲€傝寮€閿€鐢辨瘡涓?slab 鐨勫垎閰嶉樁鏁板喅瀹氥€傚垎閰嶅彲浠ュ彈鍐呮牳鍙傛暟褰卞搷锛?
`slab_min_objects`
	鍏佽鎸囧畾涓轰簡浣垮垎閰嶉樁鏁板彲鎺ュ彈锛屼竴涓?slab 涓嚦灏戝繀椤诲绾冲灏戜釜瀵硅薄銆備竴鑸潵璇达紝slub
	灏嗚兘澶熷湪涓€涓?slab 涓婃墽琛屾鏁伴噺鐨勫垎閰嶏紝鑰屾棤闇€鍜ㄨ鍙兘鍙戠敓浜夌敤鐨勯泦涓紡璧勬簮
	锛坙ist_lock锛夈€?
`slab_min_order`
	鎸囧畾 slab 鐨勬渶灏忛樁鏁般€備笌 `slab_min_objects` 鏁堟灉绫讳技銆?
`slab_max_order`
	鎸囧畾涓嶅啀妫€鏌?`slab_min_objects` 鐨勯樁鏁般€傝繖鐢ㄤ簬閬垮厤 SLUB 灏濊瘯鐢熸垚瓒呭ぇ闃舵暟鐨勯〉闈紝灏?	鍏锋湁澶у璞″ぇ灏忕殑 slab 缂撳瓨鐨?`slab_min_objects` 濉炲叆涓€涓珮闃堕〉闈€傝缃唴鏍稿懡浠よ鍙傛暟
	`debug_guardpage_minorder=N`锛圢 > 0锛変細寮哄埗灏?`slab_max_order` 璁句负 0锛屼粠鑰屼娇 slab
	鍒嗛厤浣跨敤鏈€灏忓彲鑳界殑闃舵暟銆?
`slab_strict_numa`
        鍚敤鍦ㄦ瘡涓垎閰嶄笂搴旂敤鍐呭瓨绛栫暐銆傝繖浼氫娇瀵硅薄鏀剧疆鏇寸簿纭紝浠庤€屽彲鑳藉噺灏戝杩滅▼鑺傜偣鐨勮闂€?        榛樿鎯呭喌涓嬶紝浠呭綋鑾峰彇鏂?folio 鎴栦粠鍒楄〃涓彇鍥?folio 鏃讹紝鎵嶅湪 folio 绾у埆搴旂敤鍐呭瓨绛栫暐銆?        鍚敤姝ら€夐」浼氶檷浣?slab 鍒嗛厤鍣ㄧ殑蹇€熻矾寰勬€ц兘銆?
## SLUB 璋冭瘯杈撳嚭


```

 ====================================================================
 BUG kmalloc-8: Right Redzone overwritten
 --------------------------------------------------------------------

 INFO: 0xc90f6d28-0xc90f6d2b. First byte 0x00 instead of 0xcc
 INFO: Slab 0xc528c530 flags=0x400000c3 inuse=61 fp=0xc90f6d58
 INFO: Object 0xc90f6d20 @offset=3360 fp=0xc90f6d58
 INFO: Allocated in get_modalias+0x61/0xf5 age=53 cpu=1 pid=554

 Bytes b4 (0xc90f6d10): 00 00 00 00 00 00 00 00 5a 5a 5a 5a 5a 5a 5a 5a ........ZZZZZZZZ
 Object   (0xc90f6d20): 31 30 31 39 2e 30 30 35                         1019.005
 Redzone  (0xc90f6d28): 00 cc cc cc                                     .
 Padding  (0xc90f6d50): 5a 5a 5a 5a 5a 5a 5a 5a                         ZZZZZZZZ

   [<c010523d>] dump_trace+0x63/0x1eb
   [<c01053df>] show_trace_log_lvl+0x1a/0x2f
   [<c010601d>] show_trace+0x12/0x14
   [<c0106035>] dump_stack+0x16/0x18
   [<c017e0fa>] object_err+0x143/0x14b
   [<c017e2cc>] check_object+0x66/0x234
   [<c017eb43>] __slab_free+0x239/0x384
   [<c017f446>] kfree+0xa6/0xc6
   [<c02e2335>] get_modalias+0xb9/0xf5
   [<c02e23b7>] dmi_dev_uevent+0x27/0x3c
   [<c027866a>] dev_uevent+0x1ad/0x1da
   [<c0205024>] kobject_uevent_env+0x20a/0x45b
   [<c020527f>] kobject_uevent+0xa/0xf
   [<c02779f1>] store_uevent+0x4f/0x58
   [<c027758e>] dev_attr_store+0x29/0x2f
   [<c01bec4f>] sysfs_write_file+0x16e/0x19c
   [<c0183ba7>] vfs_write+0xd1/0x15a
   [<c01841d7>] sys_write+0x3d/0x72
   [<c0104112>] sysenter_past_esp+0x5f/0x99
   [<b7f7b410>] 0xb7f7b410
   =======================

 FIX kmalloc-8: Restoring Redzone 0xc90f6d28-0xc90f6d2b=0xcc

```
濡傛灉 SLUB 閬囧埌鎹熷潖鐨勫璞★紙瀹屾暣妫€娴嬮渶瑕佸唴鏍镐互 slab_debug 鍚姩锛夛紝鍒欎細鍚?syslog 杞偍浠ヤ笅
杈撳嚭锛?
1. 鎵€閬囧埌闂鐨勬弿杩?
```

     ===============================================
     BUG <slab cache affected>: <What went wrong>
     -----------------------------------------------

     INFO: <corruption start>-<corruption end> <more info>
     INFO: Slab <address> <slab information>
     INFO: Object <address> <object information>
     INFO: Allocated in <kernel function> age=<jiffies since alloc> cpu=<allocated by
	cpu> pid=<pid of the process>
     INFO: Freed in <kernel function> age=<jiffies since free> cpu=<freed by cpu>
	pid=<pid of the process>

   (Object allocation / free information is only available if SLAB_STORE_USER is
   set for the slab. slab_debug sets that option)

```
2. 濡傛灉娑夊強瀵硅薄锛屽垯鍖呮嫭瀵硅薄鍐呭銆?
   BUG SLUB 琛屼箣鍚庡彲鑳藉嚭鐜板悇绉嶇被鍨嬬殑琛岋細

   Bytes b4 <address> : <bytes>
	鏄剧ず鍦ㄦ娴嬪埌闂鐨勫璞′箣鍓嶇殑鍑犱釜瀛楄妭銆傚鏋滄崯鍧忓苟鏈湪瀵硅薄璧峰澶勫仠姝紝杩欎細寰堟湁鐢ㄣ€?
   Object <address> : <bytes>
	瀵硅薄鐨勫瓧鑺傘€傚鏋滃璞℃湭婵€娲伙紝鍒欏瓧鑺傞€氬父鍖呭惈姣掑寲鍊笺€備换浣曢潪姣掑寲鍊奸兘琛ㄦ槑瀛樺湪閲婃斁鍚庡啓鍏?	閫犳垚鐨勬崯鍧忋€?
   Redzone <address> : <bytes>
	瀵硅薄涔嬪悗鐨?Redzone銆俁edzone 鐢ㄤ簬妫€娴嬪璞′箣鍚庣殑鍐欏叆銆傛墍鏈夊瓧鑺傚簲濮嬬粓鍏锋湁鐩稿悓鐨勫€笺€傚鏋?	鏈変换浣曞亸宸紝鍒欐槸鐢卞璞¤竟鐣屼箣澶栫殑鍐欏叆閫犳垚鐨勩€?
	锛圧edzone 淇℃伅浠呭湪璁剧疆浜?SLAB_RED_ZONE 鏃跺彲鐢ㄣ€俿lab_debug 浼氳缃閫夐」锛?
   Padding <address> : <bytes>
	鐢ㄤ簬濉厖绌洪棿浠ヤ娇涓嬩竴涓璞℃纭榻愮殑鏈娇鐢ㄦ暟鎹€傚湪璋冭瘯鎯呭喌涓嬶紝鎴戜滑纭繚鑷冲皯鏈?4 瀛楄妭
	鐨勫～鍏呫€傝繖鍏佽妫€娴嬪璞′箣鍓嶇殑鍐欏叆銆?
3. 鏍堣浆鍌?
   鏍堣浆鍌ㄦ弿杩颁簡妫€娴嬪埌閿欒鐨勪綅缃€傞€氳繃鏌ョ湅鍒嗛厤鎴栭噴鏀捐瀵硅薄鐨勫嚱鏁帮紝鏇存湁鍙兘鎵惧埌鎹熷潖鐨勫師鍥犮€?
4. 鍏充簬濡備綍澶勭悊璇ラ棶棰樹互纭繚绯荤粺鎸佺画杩愯鐨勬姤鍛娿€?
```

	FIX <slab cache affected>: <corrective action taken>

   In the above sample SLUB found that the Redzone of an active object has
   been overwritten. Here a string of 8 characters was written into a slab that
   has the length of 8 characters. However, a 8 character string needs a
   terminating 0. That zero has overwritten the first byte of the Redzone field.
   After reporting the details of the issue encountered the FIX SLUB message
   tells us that SLUB has restored the Redzone to its proper value and then
   system operations continue.

```

## 绱ф€ユ搷浣?

```

	slab_debug=F

```
杩欓€氬父瓒充互鍚敤 slub 鐨勫脊鎬х壒鎬э紝鍗充娇鏈夌碂绯曠殑鍐呮牳缁勪欢涓嶆柇鎹熷潖瀵硅薄锛屼篃鑳戒繚鎸佺郴缁熻繍琛屻€傝繖
瀵逛簬鐢熶骇绯荤粺鍙兘寰堥噸瑕併€傛€ц兘浼氬彈鍒板仴鍏ㄦ€ф鏌ョ殑褰卞搷锛屽苟涓斾細鎸佺画鍚?syslog 杈撳嚭閿欒娑堟伅娴侊紝
浣嗕笉浼氫娇鐢ㄩ澶栫殑鍐呭瓨锛堜笌瀹屾暣璋冭瘯涓嶅悓锛夈€?
涓嶆彁渚涗换浣曚繚璇併€傚唴鏍哥粍浠朵粛鐒堕渶瑕佷慨澶嶃€傞€氳繃瀹氫綅鍙戠敓鎹熷潖鐨?slab 骞朵粎涓鸿缂撳瓨鍚敤璋冭瘯锛屽彲浠?杩涗竴姝ヤ紭鍖栨€ц兘

```

	slab_debug=F,dentry

```
濡傛灉鎹熷潖鏄€氳繃鍦ㄥ璞℃湯灏句箣鍚庡啓鍏ラ€犳垚鐨勶紝閭ｄ箞寤鸿鍚敤 Redzone 浠ラ伩鍏嶆崯鍧忓紑澶?
```

	slab_debug=FZ,dentry

```

## 鎵╁睍 slabinfo 妯″紡涓庣粯鍥?

`slabinfo` 宸ュ叿鏈変竴涓壒娈婄殑鈥滄墿灞曗€濓紙'-X'锛夋ā寮忥紝鍖呮嫭锛? - 缂撳瓨鎬昏
 - 鎸夊ぇ灏忔帓搴忕殑 slab锛堟渶澶?-N <num> 涓?slab锛岄粯璁?1锛? - 鎸夋崯鑰楁帓搴忕殑 slab锛堟渶澶?-N <num> 涓?slab锛岄粯璁?1锛?
姝ゅ锛屽湪姝ゆā寮忎笅 `slabinfo` 涓嶄細鍔ㄦ€佺缉鏀惧ぇ灏忥紙G/M/K锛夛紝鑰屾槸浠ュ瓧鑺備负鍗曚綅鎶ュ憡鎵€鏈夊唴瀹癸紙姝?鍔熻兘涔熷彲閫氳繃 '-B' 閫夐」鐢ㄤ簬鍏朵粬 slabinfo 妯″紡锛夛紝杩欎娇寰楁姤鍛婃洿绮剧‘銆傝€屼笖锛屽湪鏌愮鎰忎箟涓婏紝
`-X` 妯″紡涔熺畝鍖栦簡 slab 琛屼负鐨勫垎鏋愶紝鍥犱负鍏惰緭鍑哄彲浠ヤ娇鐢?`slabinfo-gnuplot.sh` 鑴氭湰缁樺埗鎴?鍥俱€傚洜姝ゅ畠灏嗗垎鏋愪粠鏌ョ湅鏁板瓧锛堝ぇ閲忔暟瀛楋級鎺ㄥ悜鏇磋交鏉剧殑鏂瑰紡鈥斺€斿彲瑙嗗寲鍒嗘瀽銆?
鐢熸垚缁樺浘锛?
```

	while [ 1 ]; do slabinfo -X >> FOO_STATS; sleep 1; done

```
```

	slabinfo-gnuplot.sh FOO_STATS [FOO_STATS2 .. FOO_STATSN]

   The ``slabinfo-gnuplot.sh`` script will pre-processes the collected records
   and generates 3 png files (and 3 pre-processing cache files) per STATS
   file:
   - Slabcache Totals: FOO_STATS-totals.png
   - Slabs sorted by size: FOO_STATS-slabs-by-size.png
   - Slabs sorted by loss: FOO_STATS-slabs-by-loss.png

```
`slabinfo-gnuplot.sh` 鏈夌敤鐨勫彟涓€涓敤渚嬫槸锛屽綋浣犻渶瑕佹瘮杈冩煇浜涗唬鐮佷慨鏀光€滀箣鍓嶁€濆拰鈥滀箣鍚庘€濈殑 slab
琛屼负鏃躲€備负姝わ紝`slabinfo-gnuplot.sh` 鑴氭湰鍙互鈥滃悎骞垛€濇潵鑷笉鍚屾祴閲忕殑 `Slabcache Totals`
閮ㄥ垎銆傝鍙鍖栨瘮杈?N 涓粯鍥撅細

```

	while [ 1 ]; do slabinfo -X >> STATS<X>; sleep 1; done

```
```

	slabinfo-gnuplot.sh STATS1 STATS2 .. STATSN

```
c) 鍦?'-t' 妯″紡涓嬫墽琛?`slabinfo-gnuplot.sh`锛屼紶鍏ユ墍鏈夌殑

```

	slabinfo-gnuplot.sh -t STATS1-totals STATS2-totals .. STATSN-totals

   This will produce a single plot (png file).

   Plots, expectedly, can be large so some fluctuations or small spikes
   can go unnoticed. To deal with that, ``slabinfo-gnuplot.sh`` has two
   options to 'zoom-in'/'zoom-out':

   a) ``-s %d,%d`` -- 瑕嗙洊榛樿鐨勫浘鍍忓搴﹀拰楂樺害
   b) ``-r %d,%d`` -- 鎸囧畾瑕佷娇鐢ㄧ殑鏍锋湰鑼冨洿锛堜緥濡傦紝鍦?``slabinfo -X >> FOO_STATS; sleep 1;``
      鐨勬儏鍐典笅锛屼娇鐢?``-r 40,60`` 鑼冨洿灏嗗彧缁樺埗鍦ㄧ 40 鍒扮 60 绉掍箣闂存敹闆嗙殑鏍锋湰锛夈€?

```

## SLUB 鐨?DebugFS 鏂囦欢


鏈夊叧鍚敤浜嗙敤鎴疯窡韪皟璇曢€夐」鐨?SLUB 缂撳瓨褰撳墠鐘舵€佺殑鏇村淇℃伅锛屽彲閫氳繃 debugfs 鏂囦欢鑾峰彇锛岄€氬父
浣嶄簬 /sys/kernel/debug/slab/<cache>/ 涓嬶紙浠呬负鍚敤浜嗙敤鎴疯窡韪殑缂撳瓨鍒涘缓锛夈€傝繖浜涙枃浠舵湁 2 绉?绫诲瀷锛屽寘鍚互涓嬭皟璇曚俊鎭細

```

    Prints information about unique allocation traces of the currently
    allocated objects. The output is sorted by frequency of each trace.

    Information in the output:
    Number of objects, allocating function, possible memory wastage of
    kmalloc objects(total/per-object), minimal/average/maximal jiffies
    since alloc, pid range of the allocating processes, cpu mask of
    allocating cpus, numa node mask of origins of memory, and stack trace.

    Example:::

    338 pci_alloc_dev+0x2c/0xa0 waste=521872/1544 age=290837/291891/293509 pid=1 cpus=106 nodes=0-1
        __kmem_cache_alloc_node+0x11f/0x4e0
        kmalloc_trace+0x26/0xa0
        pci_alloc_dev+0x2c/0xa0
        pci_scan_single_device+0xd2/0x150
        pci_scan_slot+0xf7/0x2d0
        pci_scan_child_bus_extend+0x4e/0x360
        acpi_pci_root_create+0x32e/0x3b0
        pci_acpi_scan_root+0x2b9/0x2d0
        acpi_pci_root_add.cold.11+0x110/0xb0a
        acpi_bus_attach+0x262/0x3f0
        device_for_each_child+0xb7/0x110
        acpi_dev_for_each_child+0x77/0xa0
        acpi_bus_attach+0x108/0x3f0
        device_for_each_child+0xb7/0x110
        acpi_dev_for_each_child+0x77/0xa0
        acpi_bus_attach+0x108/0x3f0

```
```

    Prints information about unique freeing traces of the currently allocated
    objects. The freeing traces thus come from the previous life-cycle of the
    objects and are reported as not available for objects allocated for the first
    time. The output is sorted by frequency of each trace.

    Information in the output:
    Number of objects, freeing function, minimal/average/maximal jiffies since free,
    pid range of the freeing processes, cpu mask of freeing cpus, and stack trace.

    Example:::

    1980 <not-available> age=4294912290 pid=0 cpus=0
    51 acpi_ut_update_ref_count+0x6a6/0x782 age=236886/237027/237772 pid=1 cpus=1
	kfree+0x2db/0x420
	acpi_ut_update_ref_count+0x6a6/0x782
	acpi_ut_update_object_reference+0x1ad/0x234
	acpi_ut_update_ref_count+0x6a6/0x782
	acpi_ut_remove_reference+0x7d/0x84
	acpi_rs_get_prt_method_data+0x97/0xd6
	acpi_get_irq_routing_table+0x82/0xc4
	acpi_pci_irq_find_prt_entry+0x8e/0x2e0
	acpi_pci_irq_lookup+0x3a/0x1e0
	acpi_pci_irq_enable+0x77/0x240
	pcibios_enable_device+0x39/0x40
	do_pci_enable_device.part.0+0x5d/0xe0
	pci_enable_device_flags+0xfc/0x120
	pci_enable_device+0x13/0x20
	virtio_pci_probe+0x9e/0x170
	local_pci_probe+0x48/0x80
	pci_device_probe+0x105/0x1c0

```
Christoph Lameter, May 30, 2007
Sergey Senozhatsky, October 23, 2015
