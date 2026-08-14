## 浣跨敤 kgdb銆乲db 浠ュ強鍐呮牳璋冭瘯鍣ㄥ唴閮ㄦ満鍒?

:Author: Jason Wessel

## 绠€浠?

鍐呮牳鏈変袱涓笌璋冭瘯鏍稿績锛坉ebug core锛夋帴鍙ｇ殑涓嶅悓璋冭瘯鍣ㄥ墠绔紙kdb 鍜?kgdb锛夈€傚鏋滀綘鍦ㄧ紪璇戝拰杩愯鏃舵纭厤缃唴鏍革紝灏卞彲浠ヤ娇鐢ㄥ叾涓换涓€璋冭瘯鍣ㄥ墠绔紝骞跺湪瀹冧滑涔嬮棿鍔ㄦ€佸垏鎹€?
Kdb 鏄竴涓畝鍗曠殑 shell 椋庢牸鎺ュ彛锛屼綘鍙互鍦ㄥ甫鏈夐敭鐩樻垨涓茶鎺у埗鍙扮殑绯荤粺鎺у埗鍙颁笂浣跨敤瀹冦€備綘鍙互鐢ㄥ畠鏉ユ鏌ュ唴瀛樸€佸瘎瀛樺櫒銆佽繘绋嬪垪琛ㄣ€乨mesg锛岀敋鑷冲彲浠ヨ缃柇鐐逛互鍦ㄧ壒瀹氫綅缃仠姝€侹db 涓嶆槸婧愮爜绾ц皟璇曞櫒锛屽敖绠′綘鍙互璁剧疆鏂偣骞舵墽琛屼竴浜涘熀鏈殑鍐呮牳杩愯鎺у埗銆侹db 涓昏鏃ㄥ湪杩涜涓€浜涘垎鏋愶紝浠ヨ緟鍔╁紑鍙戞垨璇婃柇鍐呮牳闂銆傚鏋滀唬鐮佹槸鐢?`CONFIG_KALLSYMS` 鏋勫缓鐨勶紝浣犲彲浠ラ€氳繃鍚嶇О璁块棶鍐呮牳鍐呯疆鎴栧唴鏍告ā鍧椾腑鐨勪竴浜涚鍙枫€?
Kgdb 鏃ㄥ湪鐢ㄤ綔 Linux 鍐呮牳鐨勬簮鐮佺骇璋冭瘯鍣ㄣ€傚畠涓?gdb 閰嶅悎浣跨敤鏉ヨ皟璇?Linux 鍐呮牳銆傛湡鏈涙槸 gdb 鍙互鈥滈棷鍏モ€濆唴鏍革紝浠ユ鏌ュ唴瀛樸€佸彉閲忓苟鏌ョ湅璋冪敤鏍堜俊鎭紝绫讳技浜庡簲鐢ㄧ▼搴忓紑鍙戣€呬娇鐢?gdb 璋冭瘯搴旂敤绋嬪簭鐨勬柟寮忋€傚彲浠ュ湪鍐呮牳浠ｇ爜涓斁缃柇鐐瑰苟鎵ц涓€浜涙湁闄愮殑鍗曟鎵ц銆?
浣跨敤 kgdb 闇€瑕佷袱鍙版満鍣ㄣ€傚叾涓竴鍙版槸寮€鍙戞満锛屽彟涓€鍙版槸鐩爣鏈恒€傚緟璋冭瘯鐨勫唴鏍歌繍琛屽湪鐩爣鏈轰笂銆傚紑鍙戞満杩愯涓€涓拡瀵?vmlinux 鏂囦欢锛堝寘鍚鍙凤紝鑰岄潪 bzImage銆亃Image銆乽Image 绛夊紩瀵奸暅鍍忥級鐨?gdb 瀹炰緥銆傚湪 gdb 涓紝寮€鍙戣€呮寚瀹氳繛鎺ュ弬鏁板苟杩炴帴鍒?kgdb銆傚紑鍙戣€呯敤 gdb 寤虹珛鐨勮繛鎺ョ被鍨嬪彇鍐充簬娴嬭瘯鏈哄唴鏍镐腑鏄惁灏?kgdb I/O 妯″潡缂栬瘧涓哄唴缃垨鍙姞杞藉唴鏍告ā鍧椼€?
## 缂栬瘧鍐呮牳


- 涓轰簡鍚敤 kdb 鐨勭紪璇戯紝浣犲繀椤诲厛鍚敤 kgdb銆?
- kgdb 娴嬭瘯缂栬瘧閫夐」鍦?kgdb 娴嬭瘯濂椾欢涓€绔犱腑鎻忚堪銆?
### kgdb 鐨勫唴鏍搁厤缃€夐」


瑕佸惎鐢?`CONFIG_KGDB`锛屼綘搴斿湪 `Kernel hacking --> Kernel debugging` 涓嬫煡鎵惧苟閫夋嫨 `KGDB: kernel debugger`銆?
铏界劧骞朵笉寮哄埗瑕佹眰浣犵殑 vmlinux 鏂囦欢涓湁绗﹀彿锛屼絾娌℃湁绗﹀彿鏁版嵁 gdb 寰€寰€娌′粈涔堢敤澶勶紝鍥犳浣犱細鎯宠寮€鍚?`CONFIG_DEBUG_INFO`锛屽畠鍦ㄩ厤缃彍鍗曚腑绉颁负 `Compile the kernel with debug info`銆?
寤鸿锛堜絾闈炲繀闇€锛夊紑鍚?`CONFIG_FRAME_POINTER` 鍐呮牳閫夐」锛屽畠鍦ㄩ厤缃彍鍗曚腑绉颁负 :menuselection:`Compile the kernel with frame pointers`銆傝閫夐」鍚戠紪璇戝悗鐨勫彲鎵ц鏂囦欢涓彃鍏ヤ唬鐮侊紝鍦ㄤ笉鍚屼綅缃皢甯т俊鎭繚瀛樺埌瀵勫瓨鍣ㄦ垨鏍堜笂锛屼粠鑰屽厑璁歌濡?gdb 涔嬬被鐨勮皟璇曞櫒鍦ㄨ皟璇曞唴鏍告椂鏇村噯纭湴鏋勯€犳爤鍥炴函銆?
濡傛灉浣犱娇鐢ㄧ殑鏋舵瀯鏀寔鍐呮牳閫夐」 `CONFIG_STRICT_KERNEL_RWX`锛屼綘搴旇鑰冭檻灏嗗叾鍏抽棴銆傝閫夐」浼氬皢鍐呮牳鍐呭瓨绌洪棿鐨勬煇浜涘尯鍩熸爣璁颁负鍙锛屼粠鑰岄樆姝娇鐢ㄨ蒋浠舵柇鐐广€傚鏋滀綘浣跨敤鐨勬灦鏋勬敮鎸侊紝浣犲彲浠ュ湪寮€鍚?`CONFIG_STRICT_KERNEL_RWX` 閫夐」鐨勬儏鍐典笅浣跨敤纭欢鏂偣锛屽惁鍒欎綘闇€瑕佸叧闂閫夐」銆?
鎺ヤ笅鏉ワ紝浣犲簲閫夋嫨涓€涓垨澶氫釜 I/O 椹卞姩鏉ヨ繛鎺ヨ皟璇曚富鏈哄拰琚皟璇曠殑鐩爣銆傛棭鏈熷惎鍔ㄨ皟璇曢渶瑕佷竴涓敮鎸佹棭鏈熻皟璇曠殑 KGDB I/O 椹卞姩锛屼笖璇ラ┍鍔ㄥ繀椤荤洿鎺ョ紪璇戣繘鍐呮牳銆侹gdb I/O 椹卞姩鐨勯厤缃€氳繃鍐呮牳鎴栨ā鍧楀弬鏁拌繘琛岋紝浣犲彲浠ュ湪鎻忚堪 kgdboc 鍙傛暟鐨勭珷鑺備腑浜嗚В鏇村銆?
```

  # CONFIG_STRICT_KERNEL_RWX is not set
  CONFIG_FRAME_POINTER=y
  CONFIG_KGDB=y
  CONFIG_KGDB_SERIAL_CONSOLE=y

```
### kdb 鐨勫唴鏍搁厤缃€夐」


Kdb 姣斾綅浜庡唴鏍歌皟璇曟牳蹇冧箣涓婄殑绠€鍗?gdbstub 瑕佸鏉傚緱澶氥€侹db 蹇呴』瀹炵幇涓€涓?shell锛屽苟涓旇繕鍦ㄥ唴鏍哥殑鍏朵粬閮ㄥ垎娣诲姞涓€浜涜緟鍔╁嚱鏁帮紝璐熻矗鎵撳嵃鍑烘湁瓒ｇ殑鏁版嵁锛屼緥濡備綘杩愯 `lsmod` 鎴?`ps` 鏃朵細鐪嬪埌鐨勫唴瀹广€傝灏?kdb 鏋勫缓杩涘唴鏍革紝浣犻伒寰笌 kgdb 鐩稿悓鐨勬楠ゃ€?
kdb 鐨勪富瑕侀厤缃€夐」鏄?`CONFIG_KGDB_KDB`锛屽畠鍦ㄩ厤缃彍鍗曚腑绉颁负 `KGDB_KDB: include kdb frontend for kgdb`銆傜悊璁轰笂锛屽鏋滀綘鎵撶畻鍦ㄤ覆琛岀鍙ｄ笂浣跨敤 kdb锛岄偅涔堝湪浣犻厤缃?kgdb 鏃跺氨搴旇宸茬粡閫夋嫨浜嗚濡?`CONFIG_KGDB_SERIAL_CONSOLE` 鎺ュ彛杩欐牱鐨?I/O 椹卞姩銆?
濡傛灉浣犳兂鍍忎娇鐢?PS/2 椋庢牸鐨勯敭鐩樹笌 kdb 閰嶅悎锛屼綘搴旈€夋嫨 `CONFIG_KDB_KEYBOARD`锛屽畠鍦ㄩ厤缃彍鍗曚腑绉颁负 :menuselection:`KGDB_KDB: keyboard as input device`銆俙CONFIG_KDB_KEYBOARD` 閫夐」鍦?kgdb 鐨?gdb 鎺ュ彛涓病鏈変换浣曠敤閫斻€俙CONFIG_KDB_KEYBOARD` 閫夐」浠呬笌 kdb 閰嶅悎宸ヤ綔銆?
```

  # CONFIG_STRICT_KERNEL_RWX is not set
  CONFIG_FRAME_POINTER=y
  CONFIG_KGDB=y
  CONFIG_KGDB_SERIAL_CONSOLE=y
  CONFIG_KGDB_KDB=y
  CONFIG_KDB_KEYBOARD=y

```
## 鍐呮牳璋冭瘯鍣ㄥ紩瀵煎弬鏁?

鏈妭鎻忚堪褰卞搷鍐呮牳璋冭瘯鍣ㄩ厤缃殑鍚勭杩愯鏃跺唴鏍稿弬鏁般€備笅涓€绔犳兜鐩?kdb 鍜?kgdb 鐨勪娇鐢紝骞舵彁渚涗竴浜涢厤缃弬鏁扮殑绀轰緥銆?
### 鍐呮牳鍙傛暟锛歬gdboc


kgdboc 椹卞姩鏈€鍒濇槸涓€涓缉鍐欙紝鎰忎负鈥渒gdb over console锛堥€氳繃鎺у埗鍙颁娇鐢?kgdb锛夆€濄€傚浠婂畠鏄厤缃浣曚粠 gdb 涓?kgdb 閫氫俊锛屼互鍙婁綘鎯崇敤鏉ヤ笌 kdb shell 浜や簰鐨勮澶囩殑涓昏鏈哄埗銆?
瀵逛簬 kgdb/gdb锛宬gdboc 璁捐鐢ㄤ簬涓庡崟涓覆琛岀鍙ｄ竴璧峰伐浣溿€傚畠鏃ㄥ湪瑕嗙洊浣犳兂灏嗕覆琛屾帶鍒跺彴鐢ㄤ綔涓绘帶鍒跺彴骞剁敤瀹冩墽琛屽唴鏍歌皟璇曠殑鎯呭喌銆備篃鍙互鍦ㄦ湭琚寚瀹氫负绯荤粺鎺у埗鍙扮殑涓茶绔彛涓婁娇鐢?kgdb銆侹gdboc 鍙互閰嶇疆涓哄唴鏍稿唴缃垨鍐呮牳鍙姞杞芥ā鍧椼€傚彧鏈夊皢 kgdboc 浣滀负鍐呯疆缂栬瘧杩涘唴鏍革紝浣犳墠鑳戒娇鐢?`kgdbwait` 鍜屾棭鏈熻皟璇曘€?
鍙€夊湴锛屼綘鍙互閫夋嫨婵€娲?kms锛圞ernel Mode Setting锛屽唴鏍告ā寮忚缃級闆嗘垚銆傚綋浣犲皢 kms 涓?kgdboc 涓€璧蜂娇鐢紝骞朵笖浣犳湁涓€涓叿鏈夊師瀛愭ā寮忚缃挬瀛愮殑瑙嗛椹卞姩鏃讹紝灏卞彲浠ュ湪鍥惧舰鎺у埗鍙颁笂杩涘叆璋冭瘯鍣ㄣ€傚綋鍐呮牳鎵ц鎭㈠鏃讹紝鍏堝墠鐨勫浘褰㈡ā寮忓皢琚仮澶嶃€傝繖绉嶉泦鎴愬彲浠ヤ綔涓轰竴涓湁鐢ㄧ殑宸ュ叿锛屽湪鍏佽瀹屾暣鍥惧舰鎺у埗鍙板簲鐢ㄧ▼搴忚繍琛岀殑鍚屾椂锛岃緟鍔╄瘖鏂穿婧冩垨鐢?kdb 瀵瑰唴瀛樿繘琛屽垎鏋愩€?
#### kgdboc 鍙傛暟


```

	kgdboc=[kms][[,]kbd][[,]serial_device][,baud]

```
涓婇潰鍒楀嚭鐨勯『搴忓繀椤婚伒瀹堬紝濡傛灉浣犲悓鏃朵娇鐢ㄤ换浣曞彲閫夐厤缃殑璇濄€?
缂╁啓锛?
- kms = 鍐呮牳妯″紡璁剧疆锛圞ernel Mode Setting锛?
- kbd = 閿洏锛圞eyboard锛?
浣犲彲浠ユ牴鎹槸鍚︿娇鐢?kdb 鍜?鎴?kgdb锛屽湪浠ヤ笅鍦烘櫙涔嬩竴涓厤缃?kgdboc 浣跨敤閿洏鍜?鎴栦覆琛岃澶囥€傚鏋滀綘鍚屾椂浣跨敤涓婅堪浠讳綍鍙€夐厤缃紝蹇呴』閬靛畧涓婇潰鍒楀嚭鐨勯『搴忋€備娇鐢?kms + 浠?gdb 閫氬父涓嶆槸涓€涓湁鐢ㄧ殑缁勫悎銆?
##### 浣跨敤鍙姞杞芥ā鍧楁垨鍐呯疆


1. 浣滀负鍐呮牳鍐呯疆锛?
```

	kgdboc=<tty-device>,[baud]

```
2. 浣滀负鍐呮牳鍙姞杞芥ā鍧楋細

```

	modprobe kgdboc kgdboc=<tty-device>,[baud]

   Here are two examples of how you might format the kgdboc string. The
   first is for an x86 target using the first serial port. The second
   example is for the ARM Versatile AB using the second serial port.

   1. ``kgdboc=ttyS0,115200``

   2. ``kgdboc=ttyAMA1,115200``

```
##### 鐢?sysfs 鍦ㄨ繍琛屾椂閰嶇疆 kgdboc


鍦ㄨ繍琛屾椂锛屼綘鍙互閫氳繃鍚?sysfs 鍐欏叆鍙傛暟鏉ュ惎鐢ㄦ垨绂佺敤 kgdboc銆傝繖閲屾湁涓や釜绀轰緥锛?
```

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
```

	echo "" > /sys/module/kgdboc/parameters/kgdboc

```

   濡傛灉浣犳鍦ㄩ厤缃凡缁忛厤缃ソ鎴栧凡鎵撳紑鐨?tty 涓婄殑鎺у埗鍙帮紝鍒欐棤闇€鎸囧畾娉㈢壒鐜囥€?
##### 鏇村绀轰緥


浣犲彲浠ユ牴鎹槸鍚︿娇鐢?kdb 鍜?鎴?kgdb锛屽湪浠ヤ笅鍦烘櫙涔嬩竴涓厤缃?kgdboc 浣跨敤閿洏鍜?鎴栦覆琛岃澶囥€?
```

	kgdboc=<serial_device>[,baud]

   Example::

	kgdboc=ttyS0,115200

```
```

	kgdboc=kbd,<serial_device>[,baud]

   Example::

	kgdboc=kbd,ttyS0,115200

```
```

	kgdboc=kbd

```
```

	kgdboc=kms,kbd

```
```

	kgdboc=kms,kbd,ttyS0,115200

```

   Kgdboc 涓嶆敮鎸侀€氳繃 gdb 杩滅▼鍗忚涓柇鐩爣銆備綘蹇呴』鎵嬪姩鍙戦€?`SysRq-G`锛岄櫎闈炰綘鏈変竴涓皢鎺у埗鍙拌緭鍑哄垎娴佸埌缁堢绋嬪簭鐨勪唬鐞嗐€傛帶鍒跺彴浠ｇ悊涓鸿皟璇曞櫒鎻愪緵涓€涓嫭绔嬬殑 TCP 绔彛锛屼负鈥滀汉绫烩€濇帶鍒跺彴鎻愪緵鍙︿竴涓嫭绔嬬殑 TCP 绔彛銆傝浠ｇ悊鍙互鏇夸綘鍙戦€?`SysRq-G`銆?
褰撳湪娌℃湁璋冭瘯鍣ㄤ唬鐞嗙殑鎯呭喌涓嬩娇鐢?kgdboc 鏃讹紝浣犳渶缁堝彲鑳戒細鍦ㄤ袱涓叆鍙ｇ偣涔嬩竴杩炴帴璋冭瘯鍣ㄣ€傚鏋滃湪鍔犺浇 kgdboc 鍚庡彂鐢熷紓甯革紝鎺у埗鍙板簲鎵撳嵃涓€鏉℃秷鎭紝璇存槑瀹冩鍦ㄧ瓑寰呰皟璇曞櫒銆傚湪杩欑鎯呭喌涓嬶紝浣犳柇寮€缁堢绋嬪簭锛岀劧鍚庤繛鎺ヨ皟璇曞櫒鍙栬€屼唬涔嬨€傚鏋滀綘鎯充腑鏂洰鏍囩郴缁熷苟寮哄埗杩涘叆璋冭瘯浼氳瘽锛屼綘蹇呴』鍙戝嚭 `Sysrq` 搴忓垪锛岀劧鍚庨敭鍏ュ瓧姣?`g`銆傜劧鍚庝綘鏂紑缁堢浼氳瘽骞惰繛鎺?gdb銆傚鏋滀綘涓嶅枩娆㈣繖鏍凤紝浣犵殑閫夋嫨鏄慨鏀?gdb 璁╁畠鍦ㄥ垵濮嬭繛鎺ユ椂涔熸浛浣犲彂閫?`SysRq-G`锛屾垨鑰呬娇鐢ㄥ厑璁告湭淇敼鐨?gdb 杩涜璋冭瘯鐨勮皟璇曞櫒浠ｇ悊銆?
### 鍐呮牳鍙傛暟锛歚kgdboc_earlycon`


濡傛灉浣犳寚瀹氫簡鍐呮牳鍙傛暟 `kgdboc_earlycon`锛屽苟涓斾綘鐨勪覆琛岄┍鍔ㄦ敞鍐屼簡涓€涓敮鎸佽疆璇紙涓嶉渶瑕佷腑鏂苟瀹炵幇闈為樆濉?read() 鍑芥暟锛夌殑寮曞鎺у埗鍙帮紝kgdb 灏嗗皾璇曚娇鐢ㄥ紩瀵兼帶鍒跺彴宸ヤ綔锛岀洿鍒板畠鍙互鍒囨崲鍒?`kgdboc` 鍙傛暟鎸囧畾鐨勫父瑙?tty 椹卞姩銆?
閫氬父鍙湁涓€涓紩瀵兼帶鍒跺彴锛堝挨鍏舵槸瀹炵幇浜?read() 鍑芥暟鐨勯偅涓級锛屽洜姝や粎娣诲姞 `kgdboc_earlycon` 鏈韩灏辫冻浠ヤ娇鍏跺伐浣溿€傚鏋滀綘鏈夊涓紩瀵兼帶鍒跺彴锛屽彲浠ユ坊鍔犲紩瀵兼帶鍒跺彴鐨勫悕绉颁互鍖哄垎銆傛敞鎰忥紝閫氳繃寮曞鎺у埗鍙板眰鍜?tty 灞傛敞鍐岀殑鍚屼竴绔彛鐨勫悕绉板苟涓嶇浉鍚屻€?
```

   kgdboc_earlycon=qcom_geni kgdboc=ttyMSM0

```
```

   kgdboc_earlycon kgdboc=ttyMSM0

```
### 鍐呮牳鍙傛暟锛歚kgdbwait`


鍐呮牳鍛戒护琛岄€夐」 `kgdbwait` 浣?kgdb 鍦ㄥ唴鏍稿惎鍔ㄦ湡闂寸瓑寰呰皟璇曞櫒杩炴帴銆傚彧鏈夊綋浣犲皢 kgdb I/O 椹卞姩缂栬瘧杩涘唴鏍革紝骞跺皢璇?I/O 椹卞姩閰嶇疆鎸囧畾涓哄唴鏍稿懡浠よ閫夐」鏃讹紝鎵嶈兘浣跨敤姝ら€夐」銆俙kgdbwait` 鍙傛暟搴斿缁堜綅浜庡唴鏍稿懡浠よ涓?kgdb I/O 椹卞姩鐨勯厤缃弬鏁颁箣鍚庯紝鍚﹀垯鍦ㄨ姹傚唴鏍镐娇鐢ㄥ畠鏉ョ瓑寰呬箣鍓嶏紝璇?I/O 椹卞姩灏嗕笉浼氳閰嶇疆銆?
褰撲綘浣跨敤姝ら€夐」鏃讹紝鍐呮牳浼氬湪 I/O 椹卞姩鍜屾灦鏋勫厑璁哥殑鏈€鏃╂椂鏈哄仠姝㈠苟绛夊緟銆傚鏋滀綘灏?kgdb I/O 椹卞姩鏋勫缓涓哄彲鍔犺浇鍐呮牳妯″潡锛宍kgdbwait` 灏嗕笉璧蜂换浣曚綔鐢ㄣ€?
### 鍐呮牳鍙傛暟锛歚kgdbcon`


`kgdbcon` 鐗规€у厑璁镐綘鍦?gdb 杩炴帴鍒板唴鏍告椂锛屽湪 gdb 鍐呴儴鐪嬪埌 printk() 娑堟伅銆侹db 涓嶄娇鐢?kgdbcon 鐗规€с€?
Kgdb 鏀寔鍦ㄨ皟璇曞櫒宸茶繛鎺ュ苟杩愯鏃讹紝浣跨敤 gdb 涓茶鍗忚鍚戣皟璇曞櫒鍙戦€佹帶鍒跺彴娑堟伅銆傛湁涓ょ鏂瑰紡婵€娲绘鐗规€с€?
```

	kgdbcon

```
```

	echo 1 > /sys/module/debug_core/parameters/kgdb_use_con

```

   濡傛灉浣犲湪閰嶇疆 kgdb I/O 椹卞姩涔嬪悗鎵ц姝ゆ搷浣滐紝璇ヨ缃鍒颁笅涓€娆￠噸鏂伴厤缃?I/O 鏃舵墠浼氱敓鏁堛€?

   浣犱笉鑳藉湪浣滀负

```

	console=ttyS0,115200 kgdboc=ttyS0 kgdbcon

```
绯荤粺鎺у埗鍙扮殑 tty 涓婂悓鏃朵娇鐢?kgdboc + kgdbcon銆傚彲浠ュ皢姝ら€夐」涓?kgdboc 涓€璧风敤浜庝笉鏄郴缁熸帶鍒跺彴鐨?tty 涓娿€?
### 杩愯鏃跺弬鏁帮細`kgdbreboot`


kgdbreboot 鐗规€у厑璁镐綘鏇存敼璋冭瘯鍣ㄥ鐞嗛噸鍚€氱煡鐨勬柟寮忋€傝涓烘湁 3 绉嶉€夋嫨銆傞粯璁よ涓哄缁堣涓?0銆?

  :widths: 1 10 8

  - - 1
    - `echo -1 > /sys/module/debug_core/parameters/kgdbreboot`
    - 瀹屽叏蹇界暐閲嶅惎閫氱煡銆?
  - - 2
    - `echo 0 > /sys/module/debug_core/parameters/kgdbreboot`
    - 鍚戜换浣曞凡杩炴帴鐨勮皟璇曞櫒瀹㈡埛绔彂閫佸垎绂绘秷鎭€?
  - - 3
    - `echo 1 > /sys/module/debug_core/parameters/kgdbreboot`
    - 鍦ㄩ噸鍚€氱煡鏃惰繘鍏ヨ皟璇曞櫒銆?
### 鍐呮牳鍙傛暟锛歚nokaslr`


濡傛灉浣犱娇鐢ㄧ殑鏋舵瀯榛樿鍚敤浜?KASLR锛屼綘搴旇鑰冭檻灏嗗叾鍏抽棴銆侹ASLR 浼氶殢鏈哄寲鍐呮牳鏄犲儚鏄犲皠鐨勮櫄鎷熷湴鍧€锛屽苟浣夸粠 vmlinux 鐨勭鍙疯〃瑙ｆ瀽鍐呮牳绗﹀彿鍦板潃鐨?gdb 鎰熷埌鍥版儜銆?
### 鍐呮牳鍙傛暟锛歚rodata`


`CONFIG_STRICT_KERNEL_RWX` 榛樿寮€鍚紝骞朵笖鍦ㄤ竴浜涙灦鏋勶紙渚嬪 arm64锛変笂瀵?menuconfig 涓嶅彲瑙侊紝鍦ㄨ繖绉嶆儏鍐典笅浣犲彲浠ュ悜鍐呮牳浼犻€?`rodata=off`銆?
## 浣跨敤 kdb


### 涓茶绔彛涓?kdb 鐨勫揩閫熷叆闂?

杩欐槸涓€涓浣曚娇鐢?kdb 鐨勭畝鐭ず渚嬨€?
```

	console=ttyS0,115200 kgdboc=ttyS0,115200 nokaslr

   OR

   Configure kgdboc after the kernel has booted; assuming you are using
   a serial port console::

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
2. 鎵嬪姩杩涘叆鍐呮牳璋冭瘯鍣紝鎴栬€呯瓑寰?oops 鎴栨晠闅溿€傛湁鍑犵鏂瑰紡鍙互鎵嬪姩杩涘叆鍐呮牳璋冭瘯鍣紱瀹冧滑閮芥秹鍙婁娇鐢?`SysRq-G`锛岃繖鎰忓懗鐫€浣犲繀椤诲湪鍐呮牳閰嶇疆涓惎鐢ㄤ簡 `CONFIG_MAGIC_SYSRQ=y`銆?
```

	echo g > /proc/sysrq-trigger

   -  Example using minicom 2.2

      Press: `CTRL-A` `f` `g`

   -  When you have telneted to a terminal server that supports sending
      a remote break

      Press: `CTRL-]`

      Type in: ``send break``

      Press: `Enter` `g`

```
3. 鍦?kdb 鎻愮ず绗︿笅锛屼綘鍙互杩愯 `help` 鍛戒护鏉ユ煡鐪嬪彲鐢ㄥ懡浠ょ殑瀹屾暣鍒楄〃銆?
   kdb 涓竴浜涙湁鐢ㄧ殑鍛戒护鍖呮嫭锛?
   =========== =================================================================
   `lsmod`   鏄剧ず鍐呮牳妯″潡鍔犺浇鐨勪綅缃?   `ps`      浠呮樉绀烘椿鍔ㄨ繘绋?   `ps A`    鏄剧ず鎵€鏈夎繘绋?   `summary` 鏄剧ず鍐呮牳鐗堟湰淇℃伅鍜屽唴瀛樹娇鐢ㄦ儏鍐?   `bt`      浣跨敤 dump_stack() 鑾峰彇褰撳墠杩涚▼鐨勫洖婧?   `dmesg`   鏌ョ湅鍐呮牳 syslog 缂撳啿鍖?   `go`      缁х画绯荤粺杩愯
   =========== =================================================================

4. 褰撲綘浣跨敤瀹?kdb 鍚庯紝闇€瑕佽€冭檻閲嶅惎绯荤粺锛屾垨鑰呬娇鐢?`go` 鍛戒护鎭㈠姝ｅ父鐨勫唴鏍告墽琛屻€傚鏋滀綘璁╁唴鏍告殏鍋滀簡杈冮暱鏃堕棿锛屼緷璧栧強鏃惰仈缃戞垨浠讳綍涓庣湡瀹炲涓婃椂閽熸椂闂寸浉鍏崇殑浜嬪姟鐨勫簲鐢ㄧ▼搴忓彲鑳戒細鍙楀埌涓嶅埄褰卞搷锛屽洜姝ゅ湪浣跨敤鍐呮牳璋冭瘯鍣ㄦ椂浣犲簲鑰冭檻鍒拌繖涓€鐐广€?
### 浣跨敤杩炴帴閿洏鐨勬帶鍒跺彴鐨?kdb 蹇€熷叆闂?

杩欐槸涓€涓浣曚娇鐢ㄩ敭鐩橀厤鍚?kdb 鐨勭畝鐭ず渚嬨€?
```

	kgdboc=kbd

   OR

   Configure kgdboc after the kernel has booted::

	echo kbd > /sys/module/kgdboc/parameters/kgdboc

```
2. 鎵嬪姩杩涘叆鍐呮牳璋冭瘯鍣紝鎴栬€呯瓑寰?oops 鎴栨晠闅溿€傛湁鍑犵鏂瑰紡鍙互鎵嬪姩杩涘叆鍐呮牳璋冭瘯鍣紱瀹冧滑閮芥秹鍙婁娇鐢?`SysRq-G`锛岃繖鎰忓懗鐫€浣犲繀椤诲湪鍐呮牳閰嶇疆涓惎鐢ㄤ簡 `CONFIG_MAGIC_SYSRQ=y`銆?
```

	echo g > /proc/sysrq-trigger

   -  Example using a laptop keyboard:

      Press and hold down: `Alt`

      Press and hold down: `Fn`

      Press and release the key with the label: `SysRq`

      Release: `Fn`

      Press and release: `g`

      Release: `Alt`

   -  Example using a PS/2 101-key keyboard

      Press and hold down: `Alt`

      Press and release the key with the label: `SysRq`

      Press and release: `g`

      Release: `Alt`

```
3. 鐜板湪閿叆涓€涓?kdb 鍛戒护锛屼緥濡?`help`銆乣dmesg`銆乣bt` 鎴?`go` 鏉ョ户缁唴鏍告墽琛屻€?
## 浣跨敤 kgdb / gdb


涓轰簡浣跨敤 kgdb锛屼綘蹇呴』閫氳繃鍚戞煇涓?kgdb I/O 椹卞姩浼犻€掗厤缃俊鎭潵婵€娲诲畠銆傚鏋滀綘涓嶄紶閫掍换浣曢厤缃俊鎭紝kgdb 灏嗕粈涔堜篃涓嶅仛銆傚彧鏈夊綋 kgdb I/O 椹卞姩琚姞杞藉苟閰嶇疆鍚庯紝kgdb 鎵嶄細涓诲姩鎸傛帴鍒板唴鏍搁櫡闃遍挬瀛愪笂銆傚鏋滀綘鍙栨秷閰嶇疆鏌愪釜 kgdb I/O 椹卞姩锛宬gdb 灏嗘敞閿€鎵€鏈夊唴鏍搁挬瀛愮偣銆?
濡傛灉鍚敤浜?`CONFIG_SYSFS` 鍜?`CONFIG_MODULES`锛屾墍鏈?kgdb I/O 椹卞姩閮藉彲浠ュ湪杩愯鏃堕€氳繃鍚?`/sys/module/<driver>/parameter/<option>` echo 鏂扮殑閰嶇疆瀛楃涓叉潵閲嶆柊閰嶇疆銆傞€氳繃浼犻€掔┖瀛楃涓插彲浠ュ彇娑堥厤缃椹卞姩銆傚湪璋冭瘯鍣ㄨ繛鎺ユ椂涓嶈兘鏇存敼閰嶇疆銆傚湪灏濊瘯鍙栨秷閰嶇疆 kgdb I/O 椹卞姩涔嬪墠锛屽姟蹇呬娇鐢?`detach` 鍛戒护鍒嗙璋冭瘯鍣ㄣ€?
### 閫氳繃涓茶绔彛鐢?gdb 杩炴帴


1. 閰嶇疆 kgdboc

```

	kgdboc=ttyS0,115200

   OR

   Configure kgdboc after the kernel has booted::

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
2. 鍋滄鍐呮牳鎵ц锛堥棷鍏ヨ皟璇曞櫒锛?
   涓轰簡閫氳繃 kgdboc 杩炴帴鍒?gdb锛屽唴鏍稿繀椤诲厛琚仠姝€傛湁鍑犵鏂瑰紡鍙互鍋滄鍐呮牳锛屽寘鎷娇鐢?kgdbwait 浣滀负寮曞鍙傛暟銆侀€氳繃 `SysRq-G`锛屾垨鑰呰鍐呮牳涓€鐩磋繍琛岀洿鍒板畠鍙戠敓寮傚父骞跺湪璇ュ绛夊緟璋冭瘯鍣ㄨ繛鎺ャ€?
```

	echo g > /proc/sysrq-trigger

   -  Example using minicom 2.2

      Press: `CTRL-A` `f` `g`

   -  When you have telneted to a terminal server that supports sending
      a remote break

      Press: `CTRL-]`

      Type in: ``send break``

      Press: `Enter` `g`

```
3. 浠?gdb 杩炴帴

```

           % gdb ./vmlinux
           (gdb) set serial baud 115200
           (gdb) target remote /dev/ttyS0


   Example (kgdb to a terminal server on TCP port 2012)::

           % gdb ./vmlinux
           (gdb) target remote 192.168.2.2:2012


   Once connected, you can debug a kernel the way you would debug an
   application program.

   If you are having problems connecting or something is going seriously
   wrong while debugging, it will most often be the case that you want
   to enable gdb to be verbose about its target communications. You do
   this prior to issuing the ``target remote`` command by typing in::

	set debug remote 1

```
璁颁綇锛屽鏋滀綘鍦?gdb 涓户缁繍琛岋紝骞朵笖闇€瑕佸啀娆♀€滈棷鍏モ€濓紝浣犻渶瑕佸啀鍙戝嚭涓€涓?`SysRq-G`銆傚緢瀹规槗鍒涘缓涓€涓畝鍗曠殑鍏ュ彛鐐癸細鍦?`sys_sync` 澶勬斁缃竴涓柇鐐癸紝鐒跺悗浣犲彲浠ヤ粠 shell 鎴栬剼鏈繍琛?`sync` 鏉ラ棷鍏ヨ皟璇曞櫒銆?
## kgdb 涓?kdb 鐨勪簰鎿嶄綔鎬?

鍙互鍦?kdb 鍜?kgdb 涔嬮棿鍔ㄦ€佸垏鎹€傝皟璇曟牳蹇冧細璁颁綇浣犱笂娆′娇鐢ㄧ殑鏄摢涓€涓紝骞惰嚜鍔ㄤ互鐩稿悓妯″紡鍚姩銆?
### 鍦?kdb 鍜?kgdb 涔嬮棿鍒囨崲


#### 浠?kgdb 鍒囨崲鍒?kdb


鏈変袱绉嶆柟寮忓彲浠ヤ粠 kgdb 鍒囨崲鍒?kdb锛氫綘鍙互浣跨敤 gdb 鍙戝嚭涓€涓淮鎶ゅ寘锛坢aintenance packet锛夛紝鎴栬€呯洸鐩湴閿叆鍛戒护 `$3#33`銆傛瘡褰撳唴鏍歌皟璇曞櫒鍦?kgdb 妯″紡涓嬪仠姝㈡椂锛屽畠浼氭墦鍗版秷鎭?`KGDB or $3#33 for KDB`銆傞渶瑕佹敞鎰忕殑鏄紝浣犲繀椤讳竴娆℃€ф纭湴閿叆璇ュ簭鍒椼€備綘涓嶈兘閿叆閫€鏍兼垨鍒犻櫎锛屽洜涓?kgdb 浼氬皢鍏惰В閲婁负璋冭瘯娴佺殑涓€閮ㄥ垎銆?
```

	$3#33

```
```

	maintenance packet 3

   .. note::

     Now you must kill gdb. Typically you press `CTRL-Z` and issue
     the command::

	kill -9 %

```
#### 浠?kdb 鍒囨崲鍒?kgdb


鏈変袱绉嶆柟寮忓彲浠ヤ粠 kdb 鍒囨崲鍒?kgdb銆備綘鍙互浠?kdb shell 鎻愮ず绗﹀彂鍑?kgdb 鍛戒护鏉ユ墜鍔ㄨ繘鍏?kgdb 妯″紡锛屾垨鑰呭湪 kdb shell 鎻愮ず绗﹀浜庢椿鍔ㄧ姸鎬佹椂杩炴帴 gdb銆俴db shell 浼氭煡鎵?gdb 閫氳繃 gdb 杩滅▼鍗忚鍙戝嚭鐨勫吀鍨嬮鏉″懡浠わ紝濡傛灉瀹冪湅鍒板叾涓竴鏉″懡浠わ紝灏变細鑷姩鍒囨崲鍒?kgdb 妯″紡銆?
```

	kgdb

```
2. 鍦?kdb 鎻愮ず绗︿笅锛屾柇寮€缁堢绋嬪簭锛岀劧鍚庤繛鎺?gdb 鍙栬€屼唬涔嬨€?
### 浠?gdb 杩愯 kdb 鍛戒护


鍙互浣跨敤 gdb 鐨?monitor 鍛戒护锛屼粠 gdb 杩愯涓€缁勫彈闄愮殑 kdb 鍛戒护銆備綘涓嶅簲鎵ц浠讳綍杩愯鎺у埗鎴栨柇鐐规搷浣滐紝鍥犱负杩欎細鎵颁贡鍐呮牳璋冭瘯鍣ㄧ殑鐘舵€併€傚鏋滀綘宸茶繛鎺?gdb锛屽簲璇ヤ娇鐢?gdb 鏉ヨ繘琛屾柇鐐瑰拰杩愯鎺у埗鎿嶄綔銆傛洿鏈夌敤鐨勫懡浠ゆ槸璇稿 lsmod銆乨mesg銆乸s 鎴栧彲鑳界殑涓€浜涘唴瀛樹俊鎭懡浠ゃ€傝鏌ョ湅鎵€鏈夊彲杩愯鐨?kdb 鍛戒护锛屼綘鍙互杩愯 `monitor help`銆?
```

    (gdb) monitor ps
    1 idle process (state I) and
    27 sleeping system daemon (state M) processes suppressed,
    use 'ps A' to see all.
    Task Addr       Pid   Parent [*] cpu State Thread     Command

    0xc78291d0        1        0  0    0   S  0xc7829404  init
    0xc7954150      942        1  0    0   S  0xc7954384  dropbear
    0xc78789c0      944        1  0    0   S  0xc7878bf4  sh
    (gdb)

```
## kgdb 娴嬭瘯濂椾欢


褰撳湪鍐呮牳閰嶇疆涓惎鐢ㄤ簡 kgdb 鏃讹紝浣犱篃鍙互閫夋嫨鍚敤閰嶇疆鍙傛暟 `KGDB_TESTS`銆傛墦寮€瀹冧細鍚敤涓€涓壒娈婄殑 kgdb I/O 妯″潡锛岃妯″潡鏃ㄥ湪娴嬭瘯 kgdb 鐨勫唴閮ㄥ嚱鏁般€?
kgdb 娴嬭瘯涓昏闈㈠悜寮€鍙戣€咃紝鐢ㄤ簬娴嬭瘯 kgdb 鍐呴儴鏈哄埗锛屼互鍙婁綔涓哄紑鍙戞柊鐨?kgdb 鏋舵瀯鐗瑰畾瀹炵幇鐨勫伐鍏枫€傝繖浜涙祴璇曞苟涓嶆槸鐪熸缁?Linux 鍐呮牳鐨勭粓绔敤鎴风敤鐨勩€備富瑕佺殑鏂囨。鏉ユ簮鏄煡鐪?`drivers/misc/kgdbts.c` 鏂囦欢銆?
kgdb 娴嬭瘯濂椾欢涔熷彲浠ュ湪缂栬瘧鏃堕厤缃负杩愯鏍稿績娴嬭瘯闆嗭紝鏂规硶鏄缃唴鏍搁厤缃弬鏁?`KGDB_TESTS_ON_BOOT`銆傝繖涓壒瀹氶€夐」闈㈠悜鑷姩鍖栧洖褰掓祴璇曪紝涓嶉渶瑕佷慨鏀瑰唴鏍稿紩瀵奸厤缃弬鏁般€傚鏋滃紑鍚簡瀹冿紝鍙互閫氳繃鎸囧畾 `kgdbts=` 浣滀负鍐呮牳寮曞鍙傛暟鏉ョ鐢?kgdb 娴嬭瘯濂椾欢銆?
## 鍐呮牳璋冭瘯鍣ㄥ唴閮ㄦ満鍒?

### 鏋舵瀯鐩稿叧缁嗚妭


鍐呮牳璋冭瘯鍣ㄨ缁勭粐涓鸿嫢骞茬粍浠讹細

1. 璋冭瘯鏍稿績

   璋冭瘯鏍稿績浣嶄簬 `kernel/debugger/debug_core.c`銆傚畠鍖呭惈锛?
   - 涓€涓€氱敤鐨?OS 寮傚父澶勭悊绋嬪簭锛屽寘鎷湪澶?CPU 绯荤粺涓婂皢澶勭悊鍣ㄥ悓姝ュ埌鍋滄鐘舵€併€?
   - 涓?kgdb I/O 椹卞姩閫氫俊鐨?API

   - 璋冪敤鏋舵瀯鐗瑰畾鐨?kgdb 瀹炵幇鐨?API

   - 鍦ㄤ娇鐢ㄨ皟璇曞櫒鏃跺鍐呭瓨鎵ц瀹夊叏璇诲啓鐨勯€昏緫

   - 杞欢鏂偣鐨勫畬鏁村疄鐜帮紝闄ら潪琚灦鏋勮鐩?
   - 璋冪敤 kdb 鎴?kgdb 鍓嶇鍒拌皟璇曟牳蹇冪殑 API銆?
   - 鐢ㄤ簬鍘熷瓙鍐呮牳妯″紡璁剧疆鐨勭粨鏋勫拰鍥炶皟 API銆?
      .. note:: kgdboc 鏄皟鐢?kms 鍥炶皟鐨勫湴鏂广€?
2. kgdb 鏋舵瀯鐗瑰畾瀹炵幇

   璇ュ疄鐜伴€氬父浣嶄簬 `arch/*/kernel/kgdb.c`銆備緥濡傦紝`arch/x86/kernel/kgdb.c` 鍖呭惈浜嗗疄鐜扮‖浠舵柇鐐圭殑缁嗚妭锛屼互鍙婂湪鏈灦鏋勪笂鍔ㄦ€佹敞鍐屽拰娉ㄩ攢闄烽槺澶勭悊绋嬪簭鐨勫垵濮嬪寲銆傛灦鏋勭壒瀹氱殑閮ㄥ垎瀹炵幇浜嗭細

   - 鍖呭惈涓€涓灦鏋勭壒瀹氱殑闄烽槺鎹曡幏鍣紝瀹冭皟鐢?kgdb_handle_exception() 鏉ュ惎鍔?kgdb 宸ヤ綔

   - 鍦?gdb 鐗瑰畾鍖呮牸寮忎笌 struct pt_regs 涔嬮棿鐨勮浆鎹?
   - 鏋舵瀯鐗瑰畾闄烽槺閽╁瓙鐨勬敞鍐屽拰娉ㄩ攢

   - 浠讳綍鐗规畩鐨勫紓甯稿鐞嗗拰娓呯悊

   - NMI 寮傚父澶勭悊鍜屾竻鐞?
   - 锛堝彲閫夛級纭欢鏂偣

3. gdbstub 鍓嶇锛堝嵆 kgdb锛?
   gdbstub 浣嶄簬 `kernel/debug/gdbstub.c`銆傚畠鍖呭惈锛?
   - 瀹炵幇 gdb 涓茶鍗忚鐨勫叏閮ㄩ€昏緫

4. kdb 鍓嶇

   kdb 璋冭瘯鍣?shell 琚媶鍒嗕负鑻ュ共缁勪欢銆俴db 鏍稿績浣嶄簬 kernel/debug/kdb銆傚湪鍏朵粬涓€浜涘唴鏍哥粍浠朵腑鏈夎嫢骞茶緟鍔╁嚱鏁帮紝浣?kdb 鑳藉鍦ㄤ笉鑾峰彇鍙兘瀵艰嚧鍐呮牳姝婚攣鐨勯攣鐨勬儏鍐典笅妫€鏌ュ拰鎶ュ憡鍐呮牳淇℃伅銆俴db 鏍稿績瀹炵幇浜嗕互涓嬪姛鑳姐€?
   - 涓€涓畝鍗曠殑 shell

   - kdb 鏍稿績鍛戒护闆?
   - 鐢ㄤ簬娉ㄥ唽棰濆 kdb shell 鍛戒护鐨勬敞鍐?API銆?
      - 涓€涓嚜鍖呭惈 kdb 妯″潡鐨勫ソ渚嬪瓙鏄敤浜庤浆鍌?ftrace 缂撳啿鍖虹殑 `ftdump` 鍛戒护銆傚弬瑙侊細`kernel/trace/trace_kdb.c`

      - 鍏充簬濡備綍鍔ㄦ€佹敞鍐屾柊 kdb 鍛戒护鐨勭ず渚嬶紝浣犲彲浠ヤ粠 `samples/kdb/kdb_hello.c` 鏋勫缓 kdb_hello.ko 鍐呮牳妯″潡銆傝鏋勫缓姝ょず渚嬶紝浣犲彲浠ュ湪鍐呮牳閰嶇疆涓缃?`CONFIG_SAMPLES=y` 鍜?`CONFIG_SAMPLE_KDB=m`銆備箣鍚庤繍琛?`modprobe kdb_hello`锛屼笅娆¤繘鍏?kdb shell 鏃讹紝浣犲氨鍙互杩愯 `hello` 鍛戒护銆?
   - kdb_printf() 鐨勫疄鐜帮紝瀹冪洿鎺ュ皢娑堟伅鍙戦€佸埌 I/O 椹卞姩锛岀粫杩囧唴鏍告棩蹇椼€?
   - kdb shell 鐨勮蒋浠?纭欢鏂偣绠＄悊

5. kgdb I/O 椹卞姩

   姣忎釜 kgdb I/O 椹卞姩蹇呴』涓哄疄鐜颁互涓嬪唴瀹规彁渚涘疄鐜帮細

   - 閫氳繃鍐呯疆鎴栨ā鍧楄繘琛岄厤缃?
   - 鍔ㄦ€侀厤缃拰 kgdb 閽╁瓙娉ㄥ唽璋冪敤

   - 璇诲啓瀛楃鎺ュ彛

   - 鐢ㄤ簬浠?kgdb 鏍稿績鍙栨秷閰嶇疆鐨勬竻鐞嗗鐞嗙▼搴?
   - 锛堝彲閫夛級鏃╂湡璋冭瘯鏂规硶

   浠讳綍缁欏畾鐨?kgdb I/O 椹卞姩閮藉繀椤讳笌纭欢闈炲父绱у瘑鍦伴厤鍚堝伐浣滐紝骞朵笖蹇呴』浠ヤ笉鍚敤涓柇鎴栨敼鍙樼郴缁熶笂涓嬫枃鍏朵粬閮ㄥ垎鑰屼笉瀹屽叏鎭㈠瀹冧滑鐨勬柟寮忔潵杩涜銆俴gdb 鏍稿績鍦ㄩ渶瑕佽緭鍏ユ椂浼氬弽澶嶁€滆疆璇⑩€漦gdb I/O 椹卞姩浠ヨ幏鍙栧瓧绗︺€傚鏋滄病鏈夊彲鐢ㄦ暟鎹紝I/O 椹卞姩搴旂珛鍗宠繑鍥炪€傝繖鏍峰仛涓哄皢鏉ヤ互鏌愮鏂瑰紡鎺ヨЕ鐪嬮棬鐙楃‖浠舵彁渚涗簡鍙兘锛屼娇寰楀湪鍚敤杩欎簺纭欢鏃剁洰鏍囩郴缁熶笉浼氶噸缃€?
濡傛灉浣犳墦绠椾负鏂扮殑鏋舵瀯娣诲姞 kgdb 鏋舵瀯鐗瑰畾鏀寔锛岃鏋舵瀯搴斿湪鍏舵灦鏋勭壒瀹氱殑 Kconfig 鏂囦欢涓畾涔?`HAVE_ARCH_KGDB`銆傝繖灏嗕负璇ユ灦鏋勫惎鐢?kgdb锛屾鏃朵綘蹇呴』鍒涘缓涓€涓灦鏋勭壒瀹氱殑 kgdb 瀹炵幇銆?
鍦ㄦ瘡涓灦鏋勭殑 `asm/kgdb.h` 鏂囦欢涓繀椤昏缃竴浜涙爣蹇椼€傚畠浠槸锛?
- `NUMREGBYTES`锛?     鎵€鏈夊瘎瀛樺櫒鐨勫瓧鑺傚ぇ灏忥紝浠ヤ究鎴戜滑纭繚瀹冧滑閮借兘鏀惧叆涓€涓寘涓€?
- `BUFMAX`锛?     GDB 灏嗚鍏ョ殑缂撳啿鍖虹殑瀛楄妭澶у皬銆傚畠蹇呴』澶т簬 NUMREGBYTES銆?
- `CACHE_FLUSH_IS_SAFE`锛?     濡傛灉璋冪敤 flush_cache_range 鎴?flush_icache_range 濮嬬粓瀹夊叏锛屽垯璁句负 1銆傚湪鏌愪簺鏋舵瀯涓婏紝鐢变簬鎴戜滑灏嗗叾浠?CPU 淇濇寔鍦ㄧ瓑寰呯姸鎬侊紝杩欎簺鍑芥暟鍦?SMP 涓婅皟鐢ㄥ彲鑳戒笉瀹夊叏銆?
鍦?`kernel/kgdb.c` 涓繕鏈変簺鐢ㄤ簬鍏叡鍚庣鐨勪互涓嬪嚱鏁帮紝蹇呴』鐢辨灦鏋勭壒瀹氱殑鍚庣鎻愪緵锛岄櫎闈炴爣璁颁负锛堝彲閫夛級锛屽湪杩欑鎯呭喌涓嬶紝濡傛灉鏋舵瀯涓嶉渶瑕佹彁渚涚壒瀹氬疄鐜帮紝鍙互浣跨敤榛樿鍑芥暟銆?
   :internal:

### kgdboc 鍐呴儴鏈哄埗


#### kgdboc 涓?uart


kgdboc 椹卞姩瀹為檯涓婃槸涓€涓潪甯歌杽鐨勯┍鍔紝瀹冧緷璧栦簬搴曞眰鍒扮‖浠堕┍鍔ㄧ殑鈥滆疆璇㈤挬瀛愶紙polling hooks锛夆€濓紝tty 椹卞姩灏辨寕杞藉湪杩欎簺閽╁瓙涓娿€傚湪 kgdboc 鐨勬渶鍒濆疄鐜颁腑锛宻erial_core 琚慨鏀逛负鏆撮湶涓€涓綆绾?UART 閽╁瓙锛岀敤浜庡湪鍘熷瓙涓婁笅鏂囦腑浠ヨ疆璇㈡ā寮忚鍐欏崟涓瓧绗︺€傚綋 kgdb 鍚戣皟璇曞櫒鍙戝嚭 I/O 璇锋眰鏃讹紝kgdboc 璋冪敤 serial_core 涓殑鍥炶皟锛岃鍥炶皟杩涜€屼娇鐢?UART 椹卞姩涓殑鍥炶皟銆?
褰撳皢 kgdboc 涓?UART 閰嶅悎浣跨敤鏃讹紝UART 椹卞姩蹇呴』鍦?struct uart_ops 涓疄鐜颁袱涓洖璋冦€?
```

    #ifdef CONFIG_CONSOLE_POLL
        .poll_get_char = serial8250_get_poll_char,
        .poll_put_char = serial8250_put_poll_char,
    #endif


```
鍥寸粫鍒涘缓杞椹卞姩鐨勪换浣曞疄鐜扮粏鑺傞兘浣跨敤 `#ifdef CONFIG_CONSOLE_POLL`锛屽涓婃墍绀恒€傝璁颁綇锛岃疆璇㈤挬瀛愬繀椤讳互鍙互浠庡師瀛愪笂涓嬫枃璋冪敤锛屽苟鍦ㄨ繑鍥炴椂鎭㈠ UART 鑺墖鐘舵€佺殑鏂瑰紡瀹炵幇锛屼互渚跨郴缁熻兘鍦ㄨ皟璇曞櫒鍒嗙鏃舵仮澶嶆甯搞€傚浠讳綍浣犺€冭檻鐨勯攣閮借闈炲父灏忓績锛屽洜涓鸿繖閲岀殑澶辫触寰堝彲鑳芥剰鍛崇潃瑕佹寜涓嬪浣嶆寜閽€?
#### kgdboc 涓庨敭鐩?

kgdboc 椹卞姩鍖呭惈閰嶇疆涓庡凡杩炴帴閿洏閫氫俊鐨勯€昏緫銆傞敭鐩樺熀纭€璁炬柦鍙湁鍦ㄥ唴鏍搁厤缃腑璁剧疆浜?`CONFIG_KDB_KEYBOARD=y` 鏃舵墠浼氱紪璇戣繘鍐呮牳銆?
PS/2 绫诲瀷閿洏鐨勬牳蹇冭疆璇㈤敭鐩橀┍鍔ㄤ綅浜?`drivers/char/kdb_keyboard.c`銆傚綋 kgdboc 鍦ㄥ悕涓?:c`kdb_poll_funcs[]` 鐨勬暟缁勪腑濉厖鍥炶皟鏃讹紝璇ラ┍鍔ㄤ細琚寕鎺ュ埌璋冭瘯鏍稿績銆俴db_get_kbd_char() 鏄疆璇㈢‖浠朵互鑾峰彇鍗曚釜瀛楃杈撳叆鐨勯《灞傚嚱鏁般€?
#### kgdboc 涓?kms


kgdboc 椹卞姩鍖呭惈閫昏緫锛屽湪浣犱娇鐢?`kgdboc=kms,kbd` 鏃惰姹傚浘褰㈡樉绀哄垏鎹㈠埌鏂囨湰涓婁笅鏂囷紝鍓嶆彁鏄綘鏈変竴涓甫鏈夊抚缂撳啿鎺у埗鍙板拰鍘熷瓙鍐呮牳妯″紡璁剧疆鏀寔鐨勮棰戦┍鍔ㄣ€?
姣忔杩涘叆鍐呮牳璋冭瘯鍣ㄦ椂锛屽畠浼氳皟鐢?kgdboc_pre_exp_handler()锛岃鍑芥暟杩涜€岃皟鐢ㄨ櫄鎷熸帶鍒跺彴灞備腑鐨?con_debug_enter()銆傚湪鎭㈠鍐呮牳鎵ц鏃讹紝鍐呮牳璋冭瘯鍣ㄨ皟鐢?kgdboc_post_exp_handler()锛岃鍑芥暟杩涜€岃皟鐢?con_debug_leave()銆?

## 鑷磋阿


浠ヤ笅浜哄憳瀵规湰鏂囨。鍋氬嚭浜嗚础鐚細

1. Amit Kale <amitkale@linsyssoft.com>

2. Tom Rini <trini@kernel.crashing.org>

2008 骞?3 鏈堬紝鏈枃妗ｇ敱浠ヤ笅浜哄憳瀹屽叏閲嶅啓锛?
- Jason Wessel <jason.wessel@windriver.com>

2010 骞?1 鏈堬紝鏈枃妗ｆ洿鏂颁互鍖呭惈 kdb銆?
- Jason Wessel <jason.wessel@windriver.com>
