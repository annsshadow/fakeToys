## Kdump 鏂囨。鈥斺€斿熀浜?kexec 鐨勫穿婧冭浆鍌ㄨВ鍐虫柟妗?
鏈枃妗ｅ寘鍚杩般€佽缃€佸畨瑁呭拰鍒嗘瀽鐩稿叧淇℃伅銆?
## 姒傝堪

Kdump 浣跨敤 kexec 鍦ㄩ渶瑕佽幏鍙栫郴缁熷唴鏍稿唴瀛樿浆鍌ㄦ椂锛堜緥濡傦紝褰撶郴缁熷彂鐢?panic 鏃讹級蹇€?寮曞鍒颁竴涓浆鍌ㄦ崟鑾凤紙dump-capture锛夊唴鏍搞€傜郴缁熷唴鏍哥殑鍐呭瓨鏄犲儚鍦ㄩ噸鍚繃绋嬩腑琚繚鐣欙紝骞朵笖
瀵硅浆鍌ㄦ崟鑾峰唴鏍告槸鍙鐨勩€?
浣犲彲浠ヤ娇鐢ㄥ父瑙佸懡浠わ紙濡?cp銆乻cp 鎴?makedumpfile锛夊皢鍐呭瓨鏄犲儚澶嶅埗鍒版湰鍦扮鐩樹笂鐨勮浆鍌?鏂囦欢锛屾垨閫氳繃缃戠粶澶嶅埗鍒拌繙绋嬬郴缁熴€?
Kdump 鍜?kexec 鐩墠鏀寔 x86銆亁86_64銆乸pc64銆乻390x銆乤rm 鍜?arm64 鏋舵瀯銆?
褰撶郴缁熷唴鏍稿紩瀵兼椂锛屽畠浼氫负杞偍鎹曡幏鍐呮牳淇濈暀涓€灏忔鍐呭瓨銆傝繖纭繚浜嗘潵鑷郴缁熷唴鏍哥殑鎸佺画
鐩存帴鍐呭瓨璁块棶锛圖MA锛変笉浼氱牬鍧忚浆鍌ㄦ崟鑾峰唴鏍搞€俴exec -p 鍛戒护灏嗚浆鍌ㄦ崟鑾峰唴鏍稿姞杞藉埌杩欐
淇濈暀鍐呭瓨涓€?
鍦?x86 鏈哄櫒涓婏紝鏃犺鍐呮牳鍔犺浇鍦ㄥ摢閲岋紝鍚姩閮介渶瑕佺墿鐞嗗唴瀛樼殑鍓?640 KB銆備负浜嗙畝鍖栧鐞嗭紝
鏁翠釜浣?1M 琚繚鐣欙紝浠ラ伩鍏嶄换浣曞悗缁唴鏍告垨璁惧椹卞姩灏嗘暟鎹啓鍏ヨ鍖哄煙銆傝繖鏍凤紝浣?1M 鍙互琚?kdump 鍐呮牳澶嶇敤涓虹郴缁?RAM锛岃€屾棤闇€棰濆澶勭悊銆?
鍦?PPC64 鏈哄櫒涓婏紝鏃犺鍐呮牳鍔犺浇鍦ㄥ摢閲岋紝鍚姩閮介渶瑕佺墿鐞嗗唴瀛樼殑鍓?32KB锛屽苟涓斾负浜嗘敮鎸?64K
椤靛ぇ灏忥紝kexec 浼氬浠藉墠 64KB 鍐呭瓨銆?
瀵逛簬 s390x锛屽綋瑙﹀彂 kdump 鏃讹紝crashkernel 鍖哄煙涓?[0, crashkernel 鍖哄煙澶у皬] 鍖哄煙杩涜
浜ゆ崲锛岀劧鍚?kdump 鍐呮牳杩愯鍦?[0, crashkernel 鍖哄煙澶у皬] 涓€傚洜姝?s390x 涓嶉渶瑕佸彲閲嶅畾浣?鍐呮牳銆?
鍏充簬绯荤粺鍐呮牳鏍稿績鏄犲儚鐨勬墍鏈夊繀瑕佷俊鎭兘浠?ELF 鏍煎紡缂栫爜锛屽苟鍦ㄥ穿婧冧箣鍓嶅瓨鍌ㄥ湪涓€鍧椾繚鐣欑殑
鍐呭瓨鍖哄煙涓€侲LF 澶磋捣濮嬩綅缃殑鐗╃悊鍦板潃閫氳繃 elfcorehdr= 寮曞鍙傛暟浼犻€掔粰杞偍鎹曡幏鍐呮牳銆?鍙€夊湴锛屽綋浣跨敤 elfcorehdr=[size[KMG]@]offset[KMG] 璇硶鏃讹紝涔熷彲浠ヤ紶閫?ELF 澶寸殑澶у皬銆?
閫氳繃杞偍鎹曡幏鍐呮牳锛屼綘鍙互閫氳繃 /proc/vmcore 璁块棶鍐呭瓨鏄犲儚銆傚畠灏嗚浆鍌ㄥ鍑轰负涓€涓?ELF 鏍煎紡
鐨勬枃浠讹紝浣犲彲浠ヤ娇鐢?cp 鎴?scp 绛夋枃浠跺鍒跺懡浠ゅ皢鍏跺啓鍑恒€備綘涔熷彲浠ヤ娇鐢?makedumpfile 宸ュ叿
閫氳繃閫夐」鏉ュ垎鏋愬拰鍐欏嚭缁忚繃杩囨护鐨勫唴瀹癸紝渚嬪浣跨敤 '-d 31' 鏃跺畠灏嗗彧鍐欏嚭鍐呮牳鏁版嵁銆傛澶栵紝浣?鍙互浣跨敤 GNU 璋冭瘯鍣紙GDB锛夊拰 Crash 宸ュ叿绛夊垎鏋愬伐鍏锋潵璋冭瘯杞偍鏂囦欢銆傛鏂规硶纭繚杞偍椤?琚纭帓搴忋€?
## 璁剧疆涓庡畨瑁?
### 瀹夎 kexec-tools

1) 浠?root 鐢ㄦ埛鐧诲綍銆?
2) 浠庝互涓?URL 涓嬭浇 kexec-tools 鐢ㄦ埛绌洪棿鍖咃細

http://kernel.org/pub/linux/utils/kernel/kexec/kexec-tools.tar.gz

杩欐槸涓€涓寚鍚戞渶鏂扮増鏈殑绗﹀彿閾炬帴銆?
鏈€鏂扮殑 kexec-tools git 鏍戝彲鍦ㄤ互涓嬩綅缃幏鍙栵細

- git://git.kernel.org/pub/scm/utils/kernel/kexec/kexec-tools.git
- http://www.kernel.org/pub/scm/utils/kernel/kexec/kexec-tools.git

杩樻湁涓€涓?gitweb 鎺ュ彛鍙敤锛?http://www.kernel.org/git/?p=utils/kernel/kexec/kexec-tools.git

鍏充簬 kexec-tools 鐨勬洿澶氫俊鎭彲鍦ㄤ互涓嬩綅缃壘鍒帮細
http://horms.net/projects/kexec/

```
	tar xvpzf kexec-tools.tar.gz
```

```
	cd kexec-tools-VERSION
```

```
	./configure
```

```
	make
```

```
	make install
```

### 鏋勫缓绯荤粺鍜岃浆鍌ㄦ崟鑾峰唴鏍?
浣跨敤 Kdump 鏈変袱绉嶅彲鑳界殑鏂规硶銆?
1) 鏋勫缓涓€涓崟鐙殑瀹氬埗杞偍鎹曡幏鍐呮牳鏉ユ崟鑾峰唴鏍告牳蹇冭浆鍌ㄣ€?
2) 鎴栬€呬娇鐢ㄧ郴缁熷唴鏍镐簩杩涘埗鏈韩浣滀负杞偍鎹曡幏鍐呮牳锛岃€屾棤闇€鏋勫缓鍗曠嫭鐨勮浆鍌ㄦ崟鑾峰唴鏍搞€傝繖鍙?   鍦ㄦ敮鎸佸彲閲嶅畾浣嶅唴鏍哥殑鏋舵瀯涓婃墠鍙兘銆傛埅鑷崇洰鍓嶏紝i386銆亁86_64銆乸pc64銆乤rm 鍜?arm64
   鏋舵瀯鏀寔鍙噸瀹氫綅鍐呮牳銆?
鏋勫缓鍙噸瀹氫綅鍐呮牳鐨勪紭鍔垮湪浜庢棤闇€涓烘崟鑾疯浆鍌ㄨ€屾瀯寤虹浜屼釜鍐呮牳銆備絾鍚屾椂锛屼汉浠彲鑳藉笇鏈涙瀯寤?涓€涓€傚悎鑷繁闇€姹傜殑瀹氬埗杞偍鎹曡幏鍐呮牳銆?
浠ヤ笅鏄负绯荤粺鍜岃浆鍌ㄦ崟鑾峰唴鏍稿惎鐢?kdump 鏀寔鎵€闇€鐨勯厤缃缃€?
### 绯荤粺鍐呮牳閰嶇疆閫夐」

1) 鍦?"Processor type and features"锛堝鐞嗗櫒绫诲瀷鍜岀壒鎬э級涓惎鐢?"kexec system call"
   锛坘exec 绯荤粺璋冪敤锛夋垨 "kexec file based system call"锛堝熀浜庢枃浠剁殑 kexec 绯荤粺璋冪敤锛夛細

```
	CONFIG_KEXEC=y 鎴?CONFIG_KEXEC_FILE=y

   骞朵笖瀹冧滑涓よ€呴兘浼氶€夋嫨 KEXEC_CORE::

	CONFIG_KEXEC_CORE=y
```

2) 鍦?"Filesystem"锛堟枃浠剁郴缁燂級-> "Pseudo filesystems"锛堜吉鏂囦欢绯荤粺锛変腑鍚敤 "sysfs file
   system support"锛坰ysfs 鏂囦欢绯荤粺鏀寔锛夛細

```
	CONFIG_SYSFS=y

   娉ㄦ剰锛屽鏋?"General Setup"锛堥€氱敤璁剧疆锛変腑娌℃湁鍚敤 "Configure standard kernel
   features (expert users)"锛堥厤缃爣鍑嗗唴鏍哥壒鎬э紙涓撳鐢ㄦ埛锛夛級锛岄偅涔?"sysfs file system
   support" 鍙兘涓嶄細鍑虹幇鍦?"Pseudo filesystems" 鑿滃崟涓€傚湪杩欑鎯呭喌涓嬶紝璇风洿鎺ユ鏌?   .config 鏂囦欢鏈韩浠ョ‘淇?sysfs 琚墦寮€锛屽涓嬫墍绀?:

	grep 'CONFIG_SYSFS' .config
```

```
	CONFIG_DEBUG_INFO=Y

   杩欏皢浣垮唴鏍镐互璋冭瘯绗﹀彿鏋勫缓銆傝浆鍌ㄥ垎鏋愬伐鍏烽渶瑕佸甫鏈夎皟璇曠鍙风殑 vmlinux 鎵嶈兘璇诲彇鍜屽垎鏋?   杞偍鏂囦欢銆?```

### 杞偍鎹曡幏鍐呮牳閰嶇疆閫夐」锛堟灦鏋勬棤鍏筹級

1) 鍦?"Processor type and features"锛堝鐞嗗櫒绫诲瀷鍜岀壒鎬э級涓嬪惎鐢?"kernel crash dumps"
   锛堝唴鏍稿穿婧冭浆鍌級鏀寔锛?
```
	CONFIG_CRASH_DUMP=y

   骞朵笖杩欏皢閫夋嫨 VMCORE_INFO 鍜?CRASH_RESERVE::
	CONFIG_VMCORE_INFO=y
	CONFIG_CRASH_RESERVE=y
```

```
	CONFIG_PROC_VMCORE=y

   锛堝綋閫夋嫨 CONFIG_CRASH_DUMP 鏃讹紝CONFIG_PROC_VMCORE 榛樿琚缃€傦級
```

### 杞偍鎹曡幏鍐呮牳閰嶇疆閫夐」锛堟灦鏋勭浉鍏筹紝i386 鍜?x86_64锛?
1) 鍦?i386 涓婏紝鍦?"Processor type and features" 涓嬪惎鐢ㄩ珮绔唴瀛樻敮鎸侊細

```
	CONFIG_HIGHMEM4G
```

2) 鍦?CONFIG_SMP=y 鐨勬儏鍐典笅锛岄€氬父鍦ㄥ姞杞借浆鍌ㄦ崟鑾峰唴鏍告椂闇€瑕佸湪鍐呮牳鍛戒护琛屾寚瀹?nr_cpus=1锛?   鍥犱负瀵瑰ぇ澶氭暟绯荤粺鑰岃█锛宬dump 鍐呮牳鐢ㄤ竴涓?CPU 鏉ヨ浆鍌?vmcore 灏辫冻澶熶簡銆?
   浣嗘槸锛屼綘涔熷彲浠ユ寚瀹?nr_cpus=X 浠ュ湪 kdump 鍐呮牳涓惎鐢ㄥ涓鐞嗗櫒銆?
   鍦?CONFIG_SMP=n 鐨勬儏鍐典笅锛屼笂杩颁簨椤逛笌涔嬫棤鍏炽€?
3) 寤鸿榛樿鏋勫缓涓€涓彲閲嶅畾浣嶅唴鏍搞€傚鏋滃皻鏈瀯寤猴紝璇峰湪 "Processor type and features" 涓?   鍚敤 "Build a relocatable kernel"锛堟瀯寤哄彲閲嶅畾浣嶅唴鏍革級鏀寔锛?
```
	CONFIG_RELOCATABLE=y
```

4) 涓?"Physical address where the kernel is loaded"锛堝唴鏍稿姞杞界殑鐗╃悊鍦板潃锛夛紙鍦?   "Processor type and features" 涓嬶級浣跨敤涓€涓悎閫傜殑鍊笺€傝繖浠呭綋 "kernel crash dumps" 琚?   鍚敤鏃舵墠浼氬嚭鐜般€傚悎閫傜殑鍊煎彇鍐充簬鍐呮牳鏄惁鍙噸瀹氫綅銆?
   濡傛灉浣犱娇鐢ㄧ殑鏄彲閲嶅畾浣嶅唴鏍革紝浣跨敤 CONFIG_PHYSICAL_START=0x100000銆傝繖灏嗕负鐗╃悊鍦板潃
   1MB 缂栬瘧鍐呮牳锛屼絾閴翠簬鍐呮牳鏄彲閲嶅畾浣嶇殑锛屽畠鍙互浠庝换浣曠墿鐞嗗湴鍧€杩愯锛屽洜姝?kexec 寮曞
   鍔犺浇绋嬪簭浼氬皢鍏跺姞杞藉埌涓鸿浆鍌ㄦ崟鑾峰唴鏍镐繚鐣欑殑鍐呭瓨鍖哄煙涓€?
   鍚﹀垯锛屽畠搴旇鏄娇鐢ㄥ紩瀵煎弬鏁?"crashkernel=Y@X" 涓虹浜屼釜鍐呮牳淇濈暀鐨勫唴瀛樺尯鍩熺殑璧峰浣嶇疆銆?   杩欓噷 X 鏄负杞偍鎹曡幏鍐呮牳淇濈暀鐨勫唴瀛樺尯鍩熺殑璧峰浣嶇疆銆傞€氬父 X 鏄?16MB锛?x1000000锛夈€傛墍浠?   浣犲彲浠ヨ缃?CONFIG_PHYSICAL_START=0x1000000銆?
5) 鏋勫缓骞跺畨瑁呭唴鏍稿強鍏舵ā鍧椼€備笉瑕佸皢姝ゅ唴鏍告坊鍔犲埌寮曞鍔犺浇绋嬪簭鐨勯厤缃枃浠朵腑銆?
### 杞偍鎹曡幏鍐呮牳閰嶇疆閫夐」锛堟灦鏋勭浉鍏筹紝ppc64锛?
```
	CONFIG_CRASH_DUMP=y
```

```
	CONFIG_RELOCATABLE=y

   鏋勫缓骞跺畨瑁呭唴鏍稿強鍏舵ā鍧椼€?```

### 杞偍鎹曡幏鍐呮牳閰嶇疆閫夐」锛堟灦鏋勭浉鍏筹紝arm锛?
- 瑕佷娇鐢ㄥ彲閲嶅畾浣嶅唴鏍革細

```
	AUTO_ZRELADDR=y
```

### 杞偍鎹曡幏鍐呮牳閰嶇疆閫夐」锛堟灦鏋勭浉鍏筹紝arm64锛?
- 璇锋敞鎰忥紝鍗充娇鍦ㄩ潪 VHE 绯荤粺涓婇厤缃簡 dump-capture 鍐呮牳鐨?kvm锛屽畠涔熶笉浼氳鍚敤銆傝繖鏄?  鍥犱负 CPU 鍦?panic 鏃朵笉浼氳閲嶇疆鍒?EL2銆?
## crashkernel 璇硶

1) crashkernel=size@offset

   'size' 鎸囧畾涓鸿浆鍌ㄦ崟鑾峰唴鏍镐繚鐣欏灏戝唴瀛橈紝'offset' 鎸囧畾杩欐淇濈暀鍐呭瓨鐨勮捣濮嬩綅缃€備緥濡傦紝
   "crashkernel=64M@16M" 鍛婅瘔绯荤粺鍐呮牳浠庣墿鐞嗗湴鍧€ 0x01000000锛?6MB锛夊紑濮嬩负杞偍鎹曡幏鍐呮牳
   淇濈暀 64 MB 鍐呭瓨銆?
   宕╂簝鍐呮牳鍖哄煙鍙互鐢辩郴缁熷唴鏍稿湪杩愯鏃惰嚜鍔ㄦ斁缃€傝繖鏄€氳繃灏嗗熀鍧€鎸囧畾涓?0 鏉ュ畬鎴愮殑锛?
```
         crashkernel=256M@0
```

鎴栵細

```
         crashkernel=256M
```

   濡傛灉鎸囧畾浜嗚捣濮嬪湴鍧€锛岃娉ㄦ剰鍐呮牳鐨勮捣濮嬪湴鍧€浼氬榻愬埌涓€涓€硷紙璇ュ€间緷璧栦簬鏋舵瀯锛夛紝鎵€浠ュ鏋?   璧峰鍦板潃鏈榻愶紝閭ｄ箞瀵归綈鐐逛互涓嬬殑浠讳綍绌洪棿閮藉皢琚氮璐广€?
2) range1:size1[,range2:size2,...][@offset]

   铏界劧 "crashkernel=size[@offset]" 璇硶瀵瑰ぇ澶氭暟閰嶇疆鏉ヨ宸茬粡瓒冲锛屼絾鏈夋椂璁╀繚鐣欏唴瀛?   渚濊禆浜庣郴缁?RAM 鐨勫ぇ灏忎細寰堟柟渚库€斺€旇繖涓昏鏄负閭ｄ簺棰勫厛璁剧疆濂藉唴鏍稿懡浠よ浠ラ伩鍏嶅湪浠庢満鍣ㄤ腑
   绉婚櫎閮ㄥ垎鍐呭瓨鍚庣郴缁熸棤娉曞惎鍔ㄧ殑鍙戣鐗堝噯澶囩殑銆?
```
       crashkernel=<range1>:<size1>[,<range2>:<size2>,...][@offset]
       range=start-[end]

   渚嬪::

       crashkernel=512M-2G:64M,2G-:128M
```

   杩欐剰鍛崇潃锛?
       1) 濡傛灉 RAM 灏忎簬 512M锛屽垯涓嶄繚鐣欎换浣曞唴瀛橈紙杩欐槸"鏁戞彺"鎯呭喌锛?       2) 濡傛灉 RAM 澶у皬鍦?512M 鍜?2G 涔嬮棿锛堜笉鍚級锛屽垯淇濈暀 64M
       3) 濡傛灉 RAM 澶у皬澶т簬 2G锛屽垯淇濈暀 128M

3) crashkernel=size,high 鍜?crashkernel=size,low

   濡傛灉鍋忓ソ 4G 浠ヤ笂鐨勫唴瀛橈紝鍙互浣跨敤 crashkernel=size,high 鏉ユ弧瓒炽€備娇鐢ㄥ畠鏃讹紝鍏佽浠庨《绔?   鍒嗛厤鐗╃悊鍐呭瓨锛屽洜姝ゅ鏋滅郴缁熷畨瑁呬簡瓒呰繃 4G 鐨?RAM锛屽畠鍙兘鍦?4G 浠ヤ笂銆傚惁鍒欙紝濡傛灉鍙敤锛?   鍐呭瓨鍖哄煙灏嗗垎閰嶅湪 4G 浠ヤ笅銆?
   褰撲紶鍏?crashkernel=X,high 鏃讹紝鍐呮牳鍙兘鍒嗛厤 4G 浠ヤ笂鐨勭墿鐞嗗唴瀛樺尯鍩燂紝杩欑鎯呭喌涓嬮渶瑕?4G
   浠ヤ笅鐨勪綆鍐呭瓨銆傛湁涓夌鏂瑰紡鑾峰彇浣庡唴瀛橈細

      1) 濡傛灉鏈寚瀹?crashkernel=Y,low锛屽唴鏍镐細鑷姩鍦?4G 浠ヤ笅鍒嗛厤鑷冲皯 256M 鍐呭瓨銆?      2) 鏀逛负璁╃敤鎴锋寚瀹氫綆鍐呭瓨澶у皬銆?
```
            crashkernel=0,low
```

4) crashkernel=size,cma

	浠?CMA 涓繚鐣欓澶栫殑宕╂簝鍐呮牳鍐呭瓨銆傝繖娈典繚鐣欏唴瀛樺彲浠ヨ绗竴涓郴缁燂紙first system锛夌殑
	鐢ㄦ埛绌洪棿鍐呭瓨鍜屽唴鏍稿彲绉诲姩鍒嗛厤锛堝唴瀛樻皵鐞冦€亃swap锛変娇鐢ㄣ€備粠璇ュ唴瀛樿寖鍥村垎閰嶇殑椤典笉浼氳
	鍖呭惈鍦?vmcore 涓紝鍥犳濡傛灉鎵撶畻杞偍鐢ㄦ埛绌洪棿鍐呭瓨锛屽苟涓斿彲浠ラ鏈熸煇浜涘彲绉诲姩鍐呮牳椤靛彲鑳?	浼氫粠杞偍涓己澶憋紝鍒欎笉搴斾娇鐢ㄦ閫夐」銆?
	濡備笂鎵€杩帮紝浠嶇劧闇€瑕佷竴涓爣鍑嗙殑 crashkernel 淇濈暀锛屼互瀹圭撼宕╂簝鍐呮牳鍜?initrd銆?
	姝ら€夐」澧炲姞浜?kdump 澶辫触鐨勯闄╋細绗竴涓唴鏍搁厤缃殑 DMA 浼犺緭鏈€缁堝彲鑳界牬鍧忕浜屼釜鍐呮牳鐨?	鍐呭瓨銆?
	杩欑淇濈暀鏂规硶閫傜敤浜庨偅浜涙棤娉曚负鏍囧噯鐨?crashkernel 淇濈暀鐗虹壊瓒冲鍐呭瓨銆佷笖杈冧笉鍙潬銆佸彲鑳?	涓嶅畬鏁寸殑 kdump 涔熶紭浜庡畬鍏ㄦ病鏈?kdump 鐨勭郴缁熴€?
### 寮曞杩涘叆绯荤粺鍐呮牳

1) 鏍规嵁闇€瑕佹洿鏂板紩瀵煎姞杞界▼搴忥紙濡?grub銆亂aboot 鎴?lilo锛夌殑閰嶇疆鏂囦欢銆?
2) 浣跨敤寮曞鍙傛暟 "crashkernel=Y@X" 寮曞绯荤粺鍐呮牳銆?
   鍦?x86 鍜?x86_64 涓婏紝浣跨敤 "crashkernel=Y[@X]"銆傚ぇ澶氭暟鏃跺€欙紝璧峰鍦板潃 'X' 涓嶆槸蹇呴渶鐨勶紝
   鍐呮牳浼氭悳绱竴涓悎閫傜殑鍖哄煙銆傞櫎闈炴湡鏈涗竴涓樉寮忕殑璧峰鍦板潃銆?
   鍦?ppc64 涓婏紝浣跨敤 "crashkernel=128M@32M"銆?
   鍦?s390x 涓婏紝閫氬父浣跨敤 "crashkernel=xxM"銆倄x 鐨勫€煎彇鍐充簬 kdump 绯荤粺鐨勫唴瀛樻秷鑰椼€備竴鑸?   鏉ヨ锛岃繖涓嶄緷璧栦簬鐢熶骇绯荤粺鐨勫唴瀛樺ぇ灏忋€?
   鍦?arm 涓婏紝涓嶅啀闇€瑕?"crashkernel=Y@X"锛涘鏋滄湭缁欏畾 X锛屽唴鏍稿皢鑷姩鍦?RAM 鐨勫墠 512MB
   鍐呭畾浣嶅穿婧冨唴鏍告槧鍍忋€?
   鍦?arm64 涓婏紝浣跨敤 "crashkernel=Y[@X]"銆傛敞鎰忥紝濡傛灉鏄惧紡鎸囧畾锛屽唴鏍哥殑璧峰鍦板潃 X 蹇呴』瀵?   榻愬埌 2MiB锛?x200000锛夈€?
## 鍔犺浇杞偍鎹曡幏鍐呮牳

寮曞杩涘叆绯荤粺鍐呮牳鍚庯紝闇€瑕佸姞杞借浆鍌ㄦ崟鑾峰唴鏍搞€?
鍩轰簬鏋舵瀯鍜屾槧鍍忕被鍨嬶紙鏄惁鍙噸瀹氫綅锛夛紝鍙互閫夋嫨鍔犺浇杞偍鎹曡幏鍐呮牳鐨勬湭鍘嬬缉 vmlinux 鎴?鍘嬬缉 bzImage/vmlinuz銆備互涓嬫槸鎽樿銆?
瀵逛簬 i386 鍜?x86_64锛?
 - 濡傛灉鍐呮牳鍙噸瀹氫綅锛屼娇鐢?bzImage/vmlinuz銆? - 濡傛灉鍐呮牳涓嶅彲閲嶅畾浣嶏紝浣跨敤 vmlinux銆?
瀵逛簬 ppc64锛?
 - 浣跨敤 vmlinux

瀵逛簬 s390x锛?
 - 浣跨敤 image 鎴?bzImage

瀵逛簬 arm锛?
 - 浣跨敤 zImage

瀵逛簬 arm64锛?
 - 浣跨敤 vmlinux 鎴?Image

濡傛灉浣犱娇鐢ㄧ殑鏄湭鍘嬬缉鐨?vmlinux 鏄犲儚锛屽垯浣跨敤浠ヤ笅鍛戒护锛?
```
   kexec -p <dump-capture-kernel-vmlinux-image> \
   --initrd=<initrd-for-dump-capture-kernel> --args-linux \
   --append="root=<root-dev> <arch-specific-options>"
```

濡傛灉浣犱娇鐢ㄧ殑鏄帇缂╃殑 bzImage/vmlinuz锛屽垯浣跨敤浠ヤ笅鍛戒护锛?
```
   kexec -p <dump-capture-kernel-bzImage> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

濡傛灉浣犱娇鐢ㄧ殑鏄帇缂╃殑 zImage锛屽垯浣跨敤浠ヤ笅鍛戒护锛?
```
   kexec --type zImage -p <dump-capture-kernel-bzImage> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --dtb=<dtb-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

濡傛灉浣犱娇鐢ㄧ殑鏄湭鍘嬬缉鐨?Image锛屽垯浣跨敤浠ヤ笅鍛戒护锛?
```
   kexec -p <dump-capture-kernel-Image> \
   --initrd=<initrd-for-dump-capture-kernel> \
   --append="root=<root-dev> <arch-specific-options>"
```

浠ヤ笅鏄湪鍔犺浇杞偍鎹曡幏鍐呮牳鏃惰浣跨敤鐨勬灦鏋勭浉鍏冲懡浠よ閫夐」銆?
瀵逛簬 i386 鍜?x86_64锛?
	"1 irqpoll nr_cpus=1 reset_devices"

瀵逛簬 ppc64锛?
	"1 maxcpus=1 noirqdistrib reset_devices"

瀵逛簬 s390x锛?
	"1 nr_cpus=1 cgroup_disable=memory"

瀵逛簬 arm锛?
	"1 maxcpus=1 reset_devices"

瀵逛簬 arm64锛?
	"1 nr_cpus=1 reset_devices"

鍏充簬鍔犺浇杞偍鎹曡幏鍐呮牳鐨勬敞鎰忎簨椤癸細

- 榛樿鎯呭喌涓嬶紝ELF 澶翠互 ELF64 鏍煎紡瀛樺偍锛屼互鏀寔鍐呭瓨瓒呰繃 4GB 鐨勭郴缁熴€傚湪 i386 涓婏紝kexec
  浼氳嚜鍔ㄦ鏌ョ墿鐞?RAM 澶у皬鏄惁瓒呰繃 4 GB 闄愬埗锛屽鏋滄病鏈夛紝鍒欎娇鐢?ELF32銆傚洜姝わ紝鍦ㄩ潪 PAE
  绯荤粺涓婏紝濮嬬粓浣跨敤 ELF32銆?
  --elf32-core-headers 閫夐」鍙敤浜庡己鍒剁敓鎴?ELF32 澶淬€傝繖鏄繀瑕佺殑锛屽洜涓?GDB 鐩墠鍦?32 浣?  绯荤粺涓婃棤娉曟墦寮€甯︽湁 ELF64 澶寸殑 vmcore 鏂囦欢銆?
- "irqpoll" 寮曞鍙傛暟鍙噺灏戣浆鍌ㄦ崟鑾峰唴鏍镐腑鐢变簬鍏变韩涓柇瀵艰嚧鐨勯┍鍔ㄥ垵濮嬪寲澶辫触銆?
- 浣犲繀椤讳互涓?mount 鍛戒护杈撳嚭涓殑鏍硅澶囧悕鐩稿搴旂殑鏍煎紡鎸囧畾 <root-dev>銆?
- 寮曞鍙傛暟 "1" 灏嗚浆鍌ㄦ崟鑾峰唴鏍稿紩瀵煎埌鍗曠敤鎴锋ā寮忎笖涓嶅甫缃戠粶銆傚鏋滀綘鎯宠缃戠粶锛屼娇鐢?"3"銆?
- 鎴戜滑涓€鑸笉蹇呬粎浠呬负浜嗘崟鑾疯浆鍌ㄨ€屽惎鍔ㄤ竴涓?SMP 鍐呮牳銆傚洜姝わ紝閫氬父鏋勫缓 UP 杞偍鎹曡幏鍐呮牳鎴栧湪
  鍔犺浇杞偍鎹曡幏鍐呮牳鏃舵寚瀹?maxcpus=1 閫夐」鏄湁鐢ㄧ殑銆備笉杩囨敞鎰忥紝铏界劧 maxcpus 鎬绘槸鏈夋晥锛屽鏋?  褰撳墠 ARCH锛堝 x86锛夋敮鎸侊紝浣犳渶濂界敤 nr_cpus 鏇挎崲瀹冧互鑺傜渷鍐呭瓨銆?
- 濡傛灉浣犳墦绠楀湪鍏朵腑浣跨敤澶氱嚎绋嬬▼搴忥紙渚嬪 makedumpfile 鐨勫苟琛岃浆鍌ㄥ姛鑳斤級锛屼綘搴旇鍦ㄨ浆鍌?  鎹曡幏鍐呮牳涓惎鐢ㄥ CPU 鏀寔銆傚惁鍒欙紝澶氱嚎绋嬬▼搴忓彲鑳戒細鏈変弗閲嶇殑鎬ц兘涓嬮檷銆傝鍚敤澶?CPU 鏀寔锛?  浣犲簲璇ュ惎鍔ㄤ竴涓?SMP 杞偍鎹曡幏鍐呮牳锛屽苟鍦ㄥ姞杞藉畠鏃舵寚瀹?maxcpus/nr_cpus 閫夐」銆?
- 瀵逛簬 s390x 鏈変袱绉?kdump 妯″紡锛氬鏋滀娇鐢?elfcorehdr= 鍐呮牳鍙傛暟鎸囧畾浜?ELF 澶达紝鍒欏畠鍍忓湪
  鎵€鏈夊叾浠栨灦鏋勪笂涓€鏍疯 kdump 鍐呮牳浣跨敤銆傚鏋滄湭鎸囧畾 elfcorehdr= 鍐呮牳鍙傛暟锛宻390x kdump
  鍐呮牳浼氬姩鎬佸湴鍒涘缓璇ュご銆傜浜岀妯″紡鐨勪紭鍔垮湪浜庯紝瀵逛簬 CPU 鍜屽唴瀛樼儹鎻掓嫈锛屾棤闇€鐢?  kexec_load() 閲嶆柊鍔犺浇 kdump銆?
- 瀵逛簬甯︽湁璁稿闄勫睘璁惧鐨?s390x 绯荤粺锛宬dump 鍐呮牳搴旇浣跨敤 "cio_ignore" 鍐呮牳鍙傛暟锛屼互闃叉
  涓轰笌 kdump 鏃犲叧鐨勮澶囧垎閰嶅唴鏍稿唴瀛樸€傝繖鍚屾牱閫傜敤浜庝娇鐢?SCSI/FCP 璁惧鐨勭郴缁熴€傚湪杩欑
  鎯呭喌涓嬶紝鍦ㄥ皢 FCP 璁惧涓婄嚎涔嬪墠锛屽簲灏?"allow_lun_scan" zfcp 妯″潡鍙傛暟璁剧疆涓洪浂銆?
## 鍐呮牳 Panic

鍦ㄥ鍓嶆墍杩版垚鍔熷姞杞借浆鍌ㄦ崟鑾峰唴鏍镐箣鍚庯紝濡傛灉瑙﹀彂浜嗙郴缁熷穿婧冿紝绯荤粺灏嗛噸鍚繘鍏ヨ浆鍌ㄦ崟鑾峰唴鏍搞€?瑙﹀彂鐐逛綅浜?panic()銆乨ie()銆乨ie_nmi() 浠ュ強 sysrq 澶勭悊绋嬪簭锛圓LT-SysRq-c锛変腑銆?
浠ヤ笅鏉′欢浼氭墽琛屽穿婧冭Е鍙戠偣锛?
濡傛灉妫€娴嬪埌纭攣瀹氾紙hard lockup锛変笖閰嶇疆浜?"NMI watchdog"锛岀郴缁熷皢寮曞杩涘叆杞偍鎹曡幏鍐呮牳
锛坉ie_nmi()锛夈€?
濡傛灉璋冪敤浜?die()锛屽苟涓斿畠鎭板ソ鏄?pid 涓?0 鎴?1 鐨勭嚎绋嬶紝鎴栬€?die() 鍦ㄤ腑鏂笂涓嬫枃涓璋冪敤锛?鎴栬€呰皟鐢ㄤ簡 die() 涓旇缃簡 panic_on_oops锛岀郴缁熷皢寮曞杩涘叆杞偍鎹曡幏鍐呮牳銆?
鍦?powerpc 绯荤粺涓婏紝褰撶敓鎴愯蒋閲嶇疆锛坰oft-reset锛夋椂锛屾墍鏈?CPU 閮戒細璋冪敤 die()锛岀郴缁熷皢寮曞
杩涘叆杞偍鎹曡幏鍐呮牳銆?
鍑轰簬娴嬭瘯鐩殑锛屼綘鍙互浣跨敤 "ALT-SysRq-c"銆?echo c > /proc/sysrq-trigger" 鎴栫紪鍐欎竴涓?妯″潡鏉ュ己鍒?panic 浠ヨЕ鍙戝穿婧冦€?
## 鍐欏嚭杞偍鏂囦欢

杞偍鎹曡幏鍐呮牳寮曞鍚庯紝浣跨敤浠ヤ笅鍛戒护鍐欏嚭杞偍鏂囦欢锛?
```
   cp /proc/vmcore <dump-file>
```

```
   scp /proc/vmcore remote_username@remote_ip:<dump-file>
```

浣犱篃鍙互浣跨敤 makedumpfile 宸ュ叿鍐欏嚭杞偍鏂囦欢锛?
```
   makedumpfile -l --message-level 1 -d 31 /proc/vmcore <dump-file>
```

## 鍒嗘瀽

鍦ㄥ垎鏋愯浆鍌ㄦ槧鍍忎箣鍓嶏紝浣犲簲璇ラ噸鍚繘鍏ヤ竴涓ǔ瀹氱殑鍐呮牳銆?
浣犲彲浠ヤ娇鐢?GDB 瀵逛粠 /proc/vmcore 澶嶅埗鍑烘潵鐨勮浆鍌ㄦ枃浠跺仛鏈夐檺鐨勫垎鏋愩€備娇鐢ㄥ甫鏈?-g 鏋勫缓鐨?璋冭瘯 vmlinux 骞惰繍琛屼互涓嬪懡浠わ細

```
   gdb vmlinux <dump-file>
```

澶勭悊鍣?0 涓婁换鍔＄殑鏍堝洖婧€佸瘎瀛樺櫒鏄剧ず鍜屽唴瀛樻樉绀洪兘宸ヤ綔姝ｅ父銆?
娉ㄦ剰锛欸DB 鏃犳硶鍒嗘瀽 x86 涓婁互 ELF64 鏍煎紡鐢熸垚鐨勬牳蹇冩枃浠躲€傚湪鏈€澶?4GB 鍐呭瓨鐨勭郴缁熶笂锛屼綘
鍙互鍦ㄨ浆鍌ㄥ唴鏍镐笂浣跨敤 --elf32-core-headers 鍐呮牳閫夐」鐢熸垚 ELF32 鏍煎紡鐨勫ご銆?
浣犱篃鍙互浣跨敤 Crash 宸ュ叿鏉ュ垎鏋?Kdump 鏍煎紡鐨勮浆鍌ㄦ枃浠躲€侰rash 鍙湪浠ヤ笅 URL 鑾峰彇锛?
   https://github.com/crash-utility/crash

Crash 鏂囨。鍙湪浠ヤ笅浣嶇疆鎵惧埌锛?   https://crash-utility.github.io/

## 鍦?WARN() 涓婅Е鍙?Kdump

鍐呮牳鍙傛暟 panic_on_warn 浼氬湪鎵€鏈?WARN() 璺緞涓皟鐢?panic()銆傝繖灏嗗鑷村湪 panic() 璋冪敤
澶勫彂鐢?kdump銆傚湪鐢ㄦ埛鎯宠鍦ㄨ繍琛屾椂鎸囧畾姝よ涓虹殑鎯呭喌涓嬶紝鍙互灏?/proc/sys/kernel/panic_on_warn
璁剧疆涓?1 鏉ュ疄鐜扮浉鍚岀殑琛屼负銆?
## 鍦?add_taint() 涓婅Е鍙?Kdump

鍐呮牳鍙傛暟 panic_on_taint 渚夸簬鍦?add_taint() 鍐呴儴鏈夋潯浠跺湴璋冪敤 panic()锛屽彧瑕佹浣嶆帺鐮佷腑
璁剧疆鐨勫€间笌 add_taint() 姝ｅ湪璁剧疆鐨勪綅鏍囧織鐩稿尮閰嶃€傝繖灏嗗鑷村湪 add_taint()->panic() 璋冪敤
澶勫彂鐢?kdump銆?
## 灏嗚浆鍌ㄦ枃浠跺啓鍏ュ姞瀵嗙鐩樺嵎

鍙互鍚敤 CONFIG_CRASH_DM_CRYPT 浠ユ敮鎸佸皢杞偍鏂囦欢淇濆瓨鍒板姞瀵嗙鐩樺嵎锛堢洰鍓嶄粎鏀寔 x86_64锛夈€?鐢ㄦ埛绌洪棿鍙互閫氳繃 /sys/kernel/config/crash_dm_crypt_keys 杩涜浜や簰璁剧疆锛?
1. 鍛婅瘔绗竴涓唴鏍搁渶瑕佸摢浜?logon 瀵嗛挜鏉ヨВ閿佺鐩樺嵎锛?
    # 娣诲姞瀵嗛挜 #1
    mkdir /sys/kernel/config/crash_dm_crypt_keys/7d26b7b4-e342-4d2d-b660-7426b0996720
    # 娣诲姞瀵嗛挜 #1 鐨勬弿杩?    echo cryptsetup:7d26b7b4-e342-4d2d-b660-7426b0996720 > /sys/kernel/config/crash_dm_crypt_keys/description

    # 鎴戜滑鐜板湪鏈夊灏戜釜瀵嗛挜锛?    cat /sys/kernel/config/crash_dm_crypt_keys/count
    1

    # 浠ョ浉鍚屾柟寮忔坊鍔犲瘑閽?#2

    # 鎴戜滑鐜板湪鏈夊灏戜釜瀵嗛挜锛?    cat /sys/kernel/config/crash_dm_crypt_keys/count
    2

    # 涓烘敮鎸?CPU/鍐呭瓨鐑彃鎷旓紝澶嶇敤宸蹭繚瀛樺埌淇濈暀鍐呭瓨鐨勫瘑閽?    echo true > /sys/kernel/config/crash_dm_crypt_key/reuse

2. 鍔犺浇杞偍鎹曡幏鍐呮牳

3. 鍦ㄨ浆鍌ㄦ崟鑾峰唴鏍稿紩瀵间箣鍚庯紝灏嗗瘑閽ユ仮澶嶅埌鐢ㄦ埛瀵嗛挜鐜細
   echo yes > /sys/kernel/config/crash_dm_crypt_keys/restore

## 鑱旂郴鏂瑰紡

- kexec@lists.infradead.org

## GDB 瀹?
   :literal:
