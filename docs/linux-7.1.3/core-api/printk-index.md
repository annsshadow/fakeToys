
## Printk 绱㈠紩锛圥rintk Index锛?

鏈夎澶氭柟寮忓彲浠ョ洃鎺х郴缁熺姸鎬併€備竴涓噸瑕佺殑淇℃伅鏉ユ簮鏄郴缁熸棩蹇椼€傚畠鎻愪緵浜嗗ぇ閲忎俊鎭紝
鍖呮嫭鎴栧鎴栧皯鐨勮鍛婁笌閿欒娑堟伅銆?
鏈変竴浜涚洃鎺у伐鍏蜂細鏍规嵁璁板綍鐨勬秷鎭繘琛岃繃婊ゅ苟閲囧彇琛屽姩銆?
鍐呮牳娑堟伅鏄殢浠ｇ爜涓€璧锋紨鍖栫殑銆傚洜姝わ紝鐗瑰畾鐨勫唴鏍告秷鎭笉鏄?KABI锛屼篃姘歌繙涓嶄細鏄紒

缁存姢绯荤粺鏃ュ織鐩戣鍣ㄦ槸涓€涓法澶х殑鎸戞垬銆傚畠瑕佹眰鐭ラ亾鍦ㄦ煇涓壒瀹氬唴鏍哥増鏈腑鍝簺娑堟伅琚?鏇存柊浜嗐€佷互鍙婁负浠€涔堛€傚湪婧愮爜涓壘鍒拌繖浜涘彉鍖栭渶瑕佺浉褰撳鏉傜殑瑙ｆ瀽鍣ㄣ€傝€屼笖瀹冭繕闇€瑕佸皢
婧愮爜涓庝簩杩涘埗鍐呮牳鍖归厤锛岃繖骞堕潪鎬绘槸鏄撲簨銆傚悇绉嶆洿鏀瑰彲鑳借鍙嶅悜绉绘锛坆ackport锛夈€備笉鍚岀殑
琚洃鎺х郴缁熶笂鍙兘浣跨敤涓嶅悓鐨勫唴鏍哥増鏈€?
杩欐鏄?printk 绱㈠紩鐗规€у彲鑳芥湁鐢ㄧ殑鍦版柟銆傚畠鎻愪緵浜嗕竴浠借繍琛屼腑绯荤粺涓婂唴鏍稿強妯″潡鎵€鐢ㄦ簮鐮?涓?printk 鏍煎紡鐨勮浆鍌ㄣ€傚畠鍙互閫氳繃 debugfs 鍦ㄨ繍琛屾椂璁块棶銆?
printk 绱㈠紩鏈夊姪浜庡彂鐜版秷鎭牸寮忎腑鐨勫彉鍖栥€傚悓鏃跺畠涔熸湁鍔╀簬灏嗗瓧绗︿覆鍥炴函鍒板唴鏍告簮鐮佸強
鐩稿叧鐨勬彁浜ゃ€?

## 鐢ㄦ埛鎺ュ彛


printk 鏍煎紡鐨勭储寮曡鎷嗗垎鍒扮嫭绔嬬殑鏂囦欢涓€傝繖浜涙枃浠舵牴鎹 printk 鏍煎紡鍐呯疆锛坆uilt-in锛?鎵€鍦ㄧ殑浜岃繘鍒舵枃浠舵潵鍛藉悕銆傛湁
```

   /sys/kernel/debug/printk/index/vmlinux
   /sys/kernel/debug/printk/index/ext4
   /sys/kernel/debug/printk/index/scsi_mod

```
娉ㄦ剰鍙樉绀哄凡鍔犺浇鐨勬ā鍧椼€傚綋鏌愪釜妯″潡琚唴缃椂锛屽畠鐨?printk 鏍煎紡涔熷彲鑳藉嚭鐜板湪
"vmlinux" 涓€?
```

   $> head -1 /sys/kernel/debug/printk/index/vmlinux; shuf -n 5 vmlinux
   # <level[,flags]> filename:line function "format"
   <5> block/blk-settings.c:661 disk_stack_limits "%s: Warning: Device %s is misaligned\n"
   <4> kernel/trace/trace.c:8296 trace_create_file "Could not create tracefs '%s' entry\n"
   <6> arch/x86/kernel/hpet.c:144 _hpet_print_config "hpet: %s(%d):\n"
   <6> init/do_mounts.c:605 prepare_namespace "Waiting for root device %s...\n"
   <6> drivers/acpi/osl.c:1410 acpi_no_auto_serialize_setup "ACPI: auto-serialization disabled\n"

```
锛屽叾鍚箟涓猴細

   - :level: 鏃ュ織绾у埆鍊硷細鐗瑰畾涓ラ噸绋嬪害鐨?0-7锛?1 涓洪粯璁わ紝'c' 涓烘病鏈夋槑纭棩蹇楃骇鍒殑
	杩炵画琛?   - :flags: 鍙€夋爣蹇楋細鐩墠鍙湁 'c' 琛ㄧず KERN_CONT
   - :filename\:line: 鐩稿叧 printk() 璋冪敤鐨勬簮鏂囦欢鍚嶅拰琛屽彿銆傛敞鎰忔湁璁稿鍖呰鍑芥暟锛?	渚嬪 pr_warn()銆乸r_warn_once()銆乨ev_warn()銆?   - :function: 浣跨敤 printk() 璋冪敤鐨勫嚱鏁板悕銆?   - :format: 鏍煎紡瀛楃涓?
杩欎簺棰濆淇℃伅浣垮緱鍦ㄤ笉鍚屽唴鏍镐箣闂存煡鎵惧樊寮傜◢寰洶闅句竴浜涖€傚挨鍏舵槸琛屽彿鍙兘缁忓父鍙樺寲銆?鍙︿竴鏂归潰锛屽畠闈炲父鏈夊姪浜庣‘璁ゆ槸鍚屼竴涓瓧绗︿覆锛屾垨鑰呮壘鍒拌礋璐ｆ渶缁堝彉鍖栫殑鎻愪氦銆?

## printk() 涓嶆槸绋冲畾鐨?KABI


涓€浜涘紑鍙戣€呮媴蹇冿紝灏嗚繖浜涘疄鐜扮粏鑺傚叏閮ㄥ鍑哄埌鐢ㄦ埛绌洪棿浼氭妸鐗瑰畾鐨?printk() 璋冪敤鍙樻垚
KABI銆?
浣嗕簨瀹炴伆鎭扮浉鍙嶃€俻rintk() 璋冪敤**缁濅笉**搴旇鏄?KABI銆傝€?printk 绱㈠紩甯姪鐢ㄦ埛绌洪棿
宸ュ叿搴斿杩欎竴鐐广€?

## 瀛愮郴缁熺壒瀹氱殑 printk 鍖呰鍑芥暟


printk 绱㈠紩鏄娇鐢ㄥ瓨鍌ㄥ湪涓撶敤 .elf 娈?".printk_index" 涓殑棰濆鍏冩暟鎹敓鎴愮殑銆傝繖鏄?閫氳繃瀹忓寘瑁呭嚱鏁颁笌鐪熸鐨?printk() 璋冪敤涓€璧锋墽琛?__printk_index_emit() 鏉ュ疄鐜扮殑銆?鍔ㄦ€佽皟璇曪紙dynamic debug锛夌壒鎬ф墍浣跨敤鐨勫厓鏁版嵁涔熼噰鐢ㄤ簡鐩稿悓鐨勬妧鏈€?
杩欎簺鍏冩暟鎹彧鏈夊湪浣跨敤杩欎簺鐗规畩鍖呰鍑芥暟鎵撳嵃鐗瑰畾娑堟伅鏃舵墠浼氳瀛樺偍銆傚畠閽堝甯哥敤鐨?printk() 璋冪敤瀹炵幇锛屽寘鎷緥濡?pr_warn() 鎴?pr_once()銆?
瀵逛簬閫氳繃鍚勭瀛愮郴缁熺壒瀹氱殑鍖呰鍑芥暟锛堝畠浠€氳繃鍏叡杈呭姪鍑芥暟璋冪敤鍘熷鐨?printk()锛夐渶瑕?鍋氶澶栫殑鏇存敼銆傝繖浜涢渶瑕佸畠浠嚜宸辩殑鍖呰鍑芥暟鏉ユ坊鍔?__printk_index_emit()銆?
鍒扮洰鍓嶄负姝㈠彧鏈夊皯鏁板瓙绯荤粺鐗瑰畾鐨勫寘瑁呭嚱鏁拌鏇存柊锛屼緥濡?dev_printk()銆傚洜姝わ紝鏌愪簺瀛?绯荤粺鐨?printk 鏍煎紡鍙兘浼氱己澶变簬 printk 绱㈠紩涓€?

## 瀛愮郴缁熺壒瀹氱殑鍓嶇紑


瀹?pr_fmt() 鍏佽瀹氫箟涓€涓墠缂€锛屽畠浼氳鎵撳嵃鍦ㄧ浉鍏?printk() 璋冪敤鐢熸垚鐨勫瓧绗︿覆涔嬪墠銆?
瀛愮郴缁熺壒瀹氱殑鍖呰鍑芥暟閫氬父浼氭坊鍔犳洿澶嶆潅鐨勫悗缂€銆?
杩欎簺鍓嶇紑鍙互閫氳繃 __printk_index_emit() 鐨勪竴涓彲閫夊弬鏁板瓨鍌ㄥ埌 printk 绱㈠紩鍏冩暟鎹腑銆?debugfs 鎺ュ彛闅忓悗鍙兘浼氭樉绀哄寘鍚繖浜涘墠缂€鐨?printk 鏍煎紡銆?```

  #define pr_fmt(fmt) "ACPI: OSL: " fmt

  static int __init acpi_no_auto_serialize_setup(char *str)
  {
	acpi_gbl_auto_serialize_methods = FALSE;
	pr_info("Auto-serialization disabled\n");

	return 1;
  }

```
```

  <6> drivers/acpi/osl.c:1410 acpi_no_auto_serialize_setup "ACPI: auto-serialization disabled\n"

```
瀹冩湁鍔╀簬灏嗙湡瀹炴棩蹇椾腑鐨勬秷鎭笌 printk 绱㈠紩鍖归厤銆傜劧鍚庢簮鏂囦欢鍚嶃€佽鍙峰拰鍑芥暟鍚嶅彲浠?鐢ㄦ潵灏嗗瓧绗︿覆涓庢簮浠ｇ爜鍖归厤銆?