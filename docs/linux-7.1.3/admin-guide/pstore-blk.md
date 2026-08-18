## pstore block oops/panic 璁板綍鍣?

### 绠€浠?

pstore block锛坧store/blk锛夋槸涓€涓?oops/panic 璁板綍鍣紝瀹冨湪绯荤粺宕╂簝鍓嶅皢鍏舵棩蹇楀啓鍏?鍧楄澶囧拰闈炲潡璁惧銆備綘鍙互鑾峰彇

```
    mount -t pstore pstore /sys/fs/pstore
```


### pstore block 姒傚康


pstore/blk 涓?pstore/blk 鎻愪緵浜嗛珮鏁堢殑閰嶇疆鏂规硶锛屽畠灏嗘墍鏈夐厤缃垎涓轰袱閮ㄥ垎锛氱敤鎴烽厤缃拰
椹卞姩閰嶇疆銆?
鐢ㄦ埛閰嶇疆鍐冲畾浜?pstore/blk 鐨勫伐浣滄柟寮忥紝渚嬪 pmsg_size銆乲msg_size 绛夈€傚畠浠兘鍚屾椂鏀寔 Kconfig 鍜屾ā鍧楀弬鏁帮紝浣嗘ā鍧楀弬鏁颁紭鍏堜簬 Kconfig銆?
椹卞姩閰嶇疆鍏ㄩ儴鍏充簬鍧楄澶囧拰闈炲潡璁惧锛屼緥濡傚潡璁惧鐨勬€诲ぇ灏忥紙total_size锛変互鍙婅/鍐欐搷浣溿€?
### 鐢ㄦ埛閰嶇疆


鎵€鏈夎繖浜涢厤缃兘鍚屾椂鏀寔 Kconfig 鍜屾ā鍧楀弬鏁帮紝浣嗘ā鍧楀弬鏁颁紭鍏堜簬 Kconfig銆?
```
        pstore_blk.blkdev=/dev/mmcblk0p7 pstore_blk.kmsg_size=64 best_effort=y
```
姣忎釜閰嶇疆鐨勭粏鑺傚彲鑳戒細璁╀綘鎰熷叴瓒ｃ€?
#### blkdev


瑕佷娇鐢ㄧ殑鍧楄澶囥€傚ぇ澶氭暟鎯呭喌涓嬶紝瀹冩槸鍧楄澶囩殑涓€涓垎鍖恒€?pstore/blk 闇€瑕佸畠銆傚畠涔熻鐢ㄤ簬 MTD 璁惧銆?
褰?pstore/blk 琚瀯寤轰负妯″潡鏃讹紝鈥渂lkdev鈥?鎺ュ彈浠ヤ笅鍙樹綋锛?
1. /dev/<disk_name> 琛ㄧず纾佺洏鐨勮澶囧彿
#. /dev/<disk_name><decimal> 琛ㄧず鍒嗗尯鐨勮澶囧彿 鈥斺€?纾佺洏鐨?   璁惧鍙峰姞涓婂垎鍖哄彿
#. /dev/<disk_name>p<decimal> 鈥斺€?涓庝笂杩扮浉鍚岋紱褰撳垎鍖虹鐩樼殑纾佺洏鍚嶄互鏁板瓧缁撳熬鏃朵娇鐢ㄦ褰㈠紡銆?
褰?pstore/blk 琚瀯寤鸿繘鍐呮牳鏃讹紝鈥渂lkdev鈥?鎺ュ彈浠ヤ笅鍙樹綋锛?
#. <hex_major><hex_minor> 鍗佸叚杩涘埗琛ㄧず鐨勮澶囧彿锛屼笉甯﹀墠瀵?0x锛屼緥濡?b302銆?#. PARTUUID=00112233-4455-6677-8899-AABBCCDDEEFF 琛ㄧず鍒嗗尯鐨勫敮涓€ id锛堝鏋滃垎鍖鸿〃鎻愪緵瀹冿級銆傝 UUID 鍙互鏄?   EFI/GPT UUID锛屾垨浣跨敤鏍煎紡 SSSSSSSS-PP 寮曠敤 MSDOS 鍒嗗尯锛屽叾涓?SSSSSSSS 鏄?32 浣?   鈥淣T disk signature鈥?鐨勯浂濉厖鍗佸叚杩涘埗琛ㄧず锛孭P 鏄?1 鍩哄垎鍖哄彿鐨勯浂濉厖鍗佸叚杩涘埗琛ㄧず銆?#. PARTUUID=<UUID>/PARTNROFF=<int> 鐢ㄤ簬鐩稿浜庡叿鏈夊凡鐭ュ敮涓€ id 鐨勫垎鍖洪€夋嫨鍒嗗尯銆?#. <major>:<minor> 浠ュ啋鍙峰垎闅旂殑璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿銆?
瀹冩帴鍙椾互涓嬬敤浜?MTD 璁惧鐨勫彉浣擄細

1. <device name> MTD 璁惧鍚嶃€傛帹鑽愪娇鐢?鈥減store鈥濄€?#. <device number> MTD 璁惧鍙枫€?
#### kmsg_size


鐢ㄤ簬 oops/panic 鍓嶇锛坒ront-end锛夌殑鍧楀ぇ灏忥紙浠?KB 涓哄崟浣嶏級銆傚畠**蹇呴』**鏄?4 鐨勫€嶆暟銆?濡傛灉浣犱笉鍏冲績 oops/panic 鏃ュ織锛屽畠鏄彲閫夌殑銆?
鏍规嵁闄ゅ叾浠?pstore 鍓嶇澶栧墿浣欑殑绌洪棿锛宱ops/panic 鍓嶇鏈夊涓潡銆?
pstore/blk 浼氶€愪釜璁板綍鍒?oops/panic 鍧楋紝骞朵笖濡傛灉娌℃湁鏇村绌洪棽鍧楋紝鎬绘槸瑕嗙洊鏈€鏃х殑鍧椼€?
#### pmsg_size


鐢ㄤ簬 pmsg 鍓嶇锛坒ront-end锛夌殑鍧楀ぇ灏忥紙浠?KB 涓哄崟浣嶏級銆傚畠**蹇呴』**鏄?4 鐨勫€嶆暟銆?濡傛灉浣犱笉鍏冲績 pmsg 鏃ュ織锛屽畠鏄彲閫夌殑銆?
涓?oops/panic 鍓嶇涓嶅悓锛宲msg 鍓嶇鍙湁涓€涓潡銆?
Pmsg 鏄竴涓敤鎴风┖闂村彲璁块棶鐨?pstore 瀵硅薄銆傚 **/dev/pmsg0** 鐨勫啓鍏ヤ細琚拷鍔犲埌璇ュ潡銆傞噸鍚悗鍐呭鍦?**/sys/fs/pstore/pmsg-pstore-blk-0** 涓彲鐢ㄣ€?
#### console_size


鐢ㄤ簬 console 鍓嶇锛坒ront-end锛夌殑鍧楀ぇ灏忥紙浠?KB 涓哄崟浣嶏級銆傚畠**蹇呴』**鏄?4 鐨勫€嶆暟銆?濡傛灉浣犱笉鍏冲績 console 鏃ュ織锛屽畠鏄彲閫夌殑銆?
涓?pmsg 鍓嶇绫讳技锛宑onsole 鍓嶇鍙湁涓€涓潡銆?
console 鐨勬墍鏈夋棩蹇楀皢琚拷鍔犲埌璇ュ潡銆傞噸鍚悗鍐呭鍦?**/sys/fs/pstore/console-pstore-blk-0** 涓彲鐢ㄣ€?
#### ftrace_size


鐢ㄤ簬 ftrace 鍓嶇锛坒ront-end锛夌殑鍧楀ぇ灏忥紙浠?KB 涓哄崟浣嶏級銆傚畠**蹇呴』**鏄?4 鐨勫€嶆暟銆?濡傛灉浣犱笉鍏冲績 ftrace 鏃ュ織锛屽畠鏄彲閫夌殑銆?
涓?oops 鍓嶇绫讳技锛屾牴鎹?cpu 澶勭悊鍣ㄧ殑鏁伴噺锛宖trace 鍓嶇鏈夊涓潡銆傛瘡涓潡澶у皬绛変簬
ftrace_size / processors_count銆?
ftrace 鐨勬墍鏈夋棩蹇楀皢琚拷鍔犲埌璇ュ潡銆傞噸鍚悗鍐呭琚悎骞跺苟鍦?**/sys/fs/pstore/ftrace-pstore-blk-0** 涓彲鐢ㄣ€?
鎸佷箙鍑芥暟杩借釜锛圥ersistent function tracing锛夊彲鑳藉璋冭瘯杞欢鎴栫‖浠舵湁鐢?
```
 # mount -t pstore pstore /sys/fs/pstore
 # mount -t debugfs debugfs /sys/kernel/debug/
 # echo 1 > /sys/kernel/debug/pstore/record_ftrace
 # reboot -f
 [...]
 # mount -t pstore pstore /sys/fs/pstore
 # tail /sys/fs/pstore/ftrace-pstore-blk-0
 CPU:0 ts:5914676 c0063828  c0063b94  call_cpuidle <- cpu_startup_entry+0x1b8/0x1e0
 CPU:0 ts:5914678 c039ecdc  c006385c  cpuidle_enter_state <- call_cpuidle+0x44/0x48
 CPU:0 ts:5914680 c039e9a0  c039ecf0  cpuidle_enter_freeze <- cpuidle_enter_state+0x304/0x314
 CPU:0 ts:5914681 c0063870  c039ea30  sched_idle_set_state <- cpuidle_enter_state+0x44/0x314
 CPU:1 ts:5916720 c0160f59  c015ee04  kernfs_unmap_bin_file <- __kernfs_remove+0x140/0x204
 CPU:1 ts:5916721 c05ca625  c015ee0c  __mutex_lock_slowpath <- __kernfs_remove+0x148/0x204
 CPU:1 ts:5916723 c05c813d  c05ca630  yield_to <- __mutex_lock_slowpath+0x314/0x358
 CPU:1 ts:5916724 c05ca2d1  c05ca638  __ww_mutex_lock <- __mutex_lock_slowpath+0x31c/0x358
```
#### max_reason


闄愬埗瀛樺偍鍝簺绫诲瀷鐨?kmsg 杞偍鍙互閫氳繃 `max_reason` 鍊兼潵鎺у埗锛屽 include/linux/kmsg_dump.h 涓殑
`enum kmsg_dump_reason` 鎵€瀹氫箟銆備緥濡傦紝瑕佸悓鏃跺瓨鍌?Oops 鍜?Panic锛宍max_reason` 搴旇缃负 2锛圞MSG_DUMP_OOPS锛夛紱
瑕佷粎瀛樺偍 Panic锛宍max_reason` 搴旇缃负 1锛圞MSG_DUMP_PANIC锛夈€傚皢鍏惰缃负 0
锛圞MSG_DUMP_UNDEF锛夋剰鍛崇潃鍘熷洜杩囨护灏嗙敱 `printk.always_kmsg_dump` 鍚姩鍙傛暟鎺у埗锛氬鏋滄湭璁剧疆锛屽垯涓?KMSG_DUMP_OOPS锛?鍚﹀垯涓?KMSG_DUMP_MAX銆?
### 椹卞姩閰嶇疆


璁惧椹卞姩浣跨敤 `register_pstore_device` 涓?`struct pstore_device_info` 鍚?pstore/blk 娉ㄥ唽銆?
   :export:

### 鍘嬬缉涓庡ご閮?

鍧楄澶囧浜庢湭鍘嬬缉鐨?oops 鏁版嵁鏉ヨ瓒冲澶с€傚疄闄呬笂鎴戜滑涓嶅缓璁暟鎹帇缂╋紝鍥犱负 pstore/blk 浼氬悜鍏朵腑鎻掑叆涓€浜涗俊鎭細

```
        Panic: Total 16 times
```
杩欐剰鍛崇潃鑷娆″惎鍔ㄤ互鏉ワ紝杩欐槸绗?16 娆?OOPS|Panic銆?鏈夋椂锛岃嚜棣栨鍚姩浠ユ潵 oops|panic 鍙戠敓鐨勬鏁板鍒ゆ柇绯荤粺鏄惁绋冲畾寰堥噸瑕併€?
```
        Oops#2 Part1
```
杩欐剰鍛崇潃鍦ㄤ笂娆″惎鍔ㄦ椂锛岃繖鏄 2 娆?OOPS銆?
### 璇诲彇鏁版嵁


杞偍鏁版嵁鍙互浠?pstore 鏂囦欢绯荤粺璇诲彇銆傝繖浜涙枃浠剁殑鏍煎紡涓猴細oops/panic 鍓嶇鏄?`dmesg-pstore-blk-[N]`锛?pmsg 鍓嶇鏄?`pmsg-pstore-blk-0`锛屼緷姝ょ被鎺ㄣ€傝浆鍌ㄦ枃浠剁殑鏃堕棿鎴宠褰曚簡瑙﹀彂鏃堕棿銆傝浠庡潡璁惧
鍒犻櫎涓€涓瓨鍌ㄧ殑璁板綍锛屽彧闇€鍙栨秷閾炬帴锛坲nlink锛夌浉搴旂殑 pstore 鏂囦欢銆?
### panic 璇诲啓 API 涓殑娉ㄦ剰浜嬮」


濡傛灉鍦?panic 鏃讹紝鍐呮牳涓嶄細杩愯澶箙浜嗭紝浠诲姟灏嗕笉浼氳璋冨害锛屽ぇ澶氭暟鍐呮牳璧勬簮灏嗗仠姝㈡湇鍔°€傝繖
鐪嬭捣鏉ュ氨鍍忓湪鍗曟牳璁＄畻鏈轰笂杩愯鐨勫崟绾跨▼绋嬪簭銆?
panic 璇诲啓 API 闇€瑕佺壒鍒敞鎰忎互涓嬪嚑鐐癸細

1. **涓嶈兘**鍒嗛厤浠讳綍鍐呭瓨銆?   濡傛灉浣犻渶瑕佸唴瀛橈紝灏卞湪鍧楅┍鍔ㄥ垵濮嬪寲鏃跺垎閰嶏紝鑰屼笉鏄瓑鍒?panic 鏃躲€?#. 蹇呴』鏄疆璇紙polled锛夛紝**涓嶆槸**涓柇椹卞姩銆?   涓嶅啀鏈変换浣曚换鍔¤皟搴︺€傚潡椹卞姩搴斿欢杩熶互纭繚鍐欏叆鎴愬姛锛屼絾**涓嶈兘**鐫＄湢銆?#. **涓嶈兘**鑾峰彇浠讳綍閿併€?   娌℃湁鍏朵粬浠诲姟锛屼篃娌℃湁浠讳綍鍏变韩璧勬簮锛涗綘鍙互瀹夊叏鍦版墦鐮存墍鏈夐攣銆?#. 鍙敤 CPU 浼犺緭銆?   涓嶈浣跨敤 DMA 浼犺緭锛岄櫎闈炰綘纭畾 DMA 涓嶄細鎸佹湁閿併€?#. 鐩存帴鎺у埗瀵勫瓨鍣ㄣ€?   璇风洿鎺ユ帶鍒跺瘎瀛樺櫒锛岃€屼笉鏄娇鐢?Linux 鍐呮牳璧勬簮銆?   鍦ㄥ垵濮嬪寲鏃跺仛 I/O 鏄犲皠锛岃€屼笉鏄瓑鍒?panic 鍙戠敓銆?#. 濡傛湁蹇呰锛岄噸缃綘鐨勫潡璁惧鍜屾帶鍒跺櫒銆?   濡傛灉浣犱笉纭畾 panic 鍙戠敓鏃跺潡璁惧鍜屾帶鍒跺櫒鐨勭姸鎬侊紝鍙互鍋滄骞堕噸缃畠浠槸瀹夊叏鐨勩€?
pstore/blk 鏀寔 psblk_blkdev_info()锛屽畠瀹氫箟鍦?**linux/pstore_blk.h** 涓紝鐢ㄤ簬鑾峰彇浣跨敤鍧楄澶囩殑淇℃伅锛屼緥濡?璁惧鍙枫€佹墖鍖鸿鏁颁互鍙婃暣涓鐩樼殑璧峰鎵囧尯銆?
### pstore block 鍐呴儴


渚涘紑鍙戣€呭弬鑰冿紝浠ヤ笅鏄墍鏈夐噸瑕佺殑缁撴瀯鍜?API锛?
   :internal:

   :internal:

   :internal:
