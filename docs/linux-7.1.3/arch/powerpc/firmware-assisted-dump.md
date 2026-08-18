## 鍥轰欢杈呭姪杞偍锛團irmware-Assisted Dump锛?
2011 骞?7 鏈?
鍥轰欢杈呭姪杞偍鐨勭洰鏍囷紝鏄湪涓€涓畬鍏ㄥ浣嶅悗鐨勭郴缁熶笂瀹炵幇瀵瑰穿婧冪郴缁熺殑杞偍锛?骞舵渶灏忓寲绯荤粺鎭㈠鐢熶骇浣跨敤鎵€闇€鐨勬€昏€楁椂銆?
- 鍥轰欢杈呭姪杞偍锛團ADump锛夊熀纭€璁炬柦鏃ㄥ湪鍙栦唬鐜版湁鐨?phyp 杈呭姪杞偍銆?- FADump 浣跨敤涓?phyp 杈呭姪杞偍鐩稿悓鐨勫浐浠舵帴鍙ｄ笌鍐呭瓨淇濈暀妯″瀷銆?- 涓?phyp dump 涓嶅悓锛孎ADump 閫氳繃 /proc/vmcore 浠?ELF 鏍煎紡瀵煎嚭鍐呭瓨杞偍锛?  鏂瑰紡涓?kdump 鐩稿悓銆傝繖鏈夊姪浜庢垜浠鐢?kdump 鍩虹璁炬柦鏉ヨ繘琛岃浆鍌ㄦ崟鑾蜂笌杩囨护銆?- 涓?phyp dump 涓嶅悓锛岀敤鎴风┖闂村伐鍏峰湪璇诲彇 /proc/vmcore 鏃舵棤闇€寮曠敤浠讳綍 sysfs 鎺ュ彛銆?- 涓?phyp dump 涓嶅悓锛孎ADump 鍏佽鐢ㄦ埛閫氳繃鍗曟鎿嶄綔 `echo 1 > /sys/kernel/fadump_release_mem`
  鏉ラ噴鏀句负杞偍淇濈暀鐨勬墍鏈夊唴瀛樸€?- 涓€鏃﹂€氳繃鍐呮牳鍚姩鍙傛暟鍚敤锛孎ADump 鍙€氳繃 /sys/kernel/fadump_registered 鎺ュ彛
  锛堝弬瑙佷笅鏂囩殑 sysfs 鏂囦欢灏忚妭锛夊惎鍔?鍋滄锛屽苟鍙交鏉惧湴涓?kdump 鏈嶅姟鐨勫惎鍔?鍋滄 init 鑴氭湰闆嗘垚銆?
涓?kdump 鎴栧叾瀹冪瓥鐣ョ浉姣旓紝鍥轰欢杈呭姪杞偍鎻愪緵浜嗚嫢骞插己澶т笖瀹炵敤鐨勪紭鍔匡細

- 涓?kdump 涓嶅悓锛岀郴缁熷凡琚浣嶏紝骞跺姞杞戒簡涓€浠藉叏鏂扮殑鍐呮牳鍓湰銆傜壒鍒湴锛?  PCI 鍜?I/O 璁惧宸茶閲嶆柊鍒濆鍖栵紝澶勪簬骞插噣銆佷竴鑷寸殑鐘舵€併€?- 涓€鏃﹁浆鍌ㄨ澶嶅埗鍑烘潵锛屾寔鏈夎浆鍌ㄧ殑鍐呭瓨绔嬪嵆瀵硅繍琛屼腑鐨勫唴鏍稿彲鐢ㄣ€傚洜姝わ紝涓?kdump 涓嶅悓锛?  FADump 涓嶉渶瑕佺浜屾閲嶅惎鏉ュ皢绯荤粺鎭㈠鍒扮敓浜ч厤缃€?
涓婅堪鐩爣鍙兘閫氳繃 Power 鍥轰欢鐨勫崗璋冧笌鍗忓姪鏉ュ疄鐜般€傚叾娴佺▼濡備笅锛?
- 绗竴涓唴鏍稿湪 OS 鍒濆鍖栨湡闂达紝鍚?Power 鍥轰欢娉ㄥ唽鐢ㄤ簬杞偍淇濈暀鐨勫唴瀛樻銆?  杩欎簺琚敞鍐岀殑鍐呭瓨娈电敱绗竴涓唴鏍稿湪鏃╂湡鍚姩鏈熼棿淇濈暀銆?
- 褰撶郴缁熷穿婧冩椂锛孭ower 鍥轰欢浼氬皢宸叉敞鍐岀殑浣庝綅鍐呭瓨鍖哄煙锛堝惎鍔ㄥ唴瀛橈級浠庢簮鍖哄鍒跺埌鐩爣鍖恒€?  瀹冭繕浼氫繚瀛樼‖浠?PTE銆?
  娉ㄦ剰锛?        鏈鈥渂oot memory锛堝惎鍔ㄥ唴瀛橈級鈥濇槸鎸囦竴涓唴鏍稿湪鍙楅檺鍐呭瓨涓嬫垚鍔熷惎鍔ㄦ墍闇€鐨?        浣庝綅鍐呭瓨鍧楀ぇ灏忋€傞粯璁ゆ儏鍐典笅锛屽惎鍔ㄥ唴瀛樺ぇ灏忎负绯荤粺 RAM 鐨?5% 涓?256MB 涓殑杈冨ぇ鑰呫€?        鎴栬€咃紝鐢ㄦ埛涔熷彲浠ラ€氳繃鍚姩鍙傛暟 'crashkernel=' 鎸囧畾鍚姩鍐呭瓨澶у皬锛屼互瑕嗙洊榛樿璁＄畻鍊笺€?        鑻ラ粯璁ゅ惎鍔ㄥ唴瀛樺ぇ灏忎笉瓒充互璁╃浜屼釜鍐呮牳鎴愬姛鍚姩锛岃浣跨敤姝ら€夐」銆傚叧浜?crashkernel=
        鍙傛暟鐨勮娉曪紝璇峰弬闃?Documentation/admin-guide/kdump/kdump.rst銆傚鏋滃湪 crashkernel=
        鍙傛暟涓彁渚涗簡浠讳綍鍋忕Щ锛屽畠灏嗚蹇界暐锛屽洜涓?FADump 浣跨敤棰勫畾涔夌殑鍋忕Щ鏉ヤ负鍚姩鍐呭瓨杞偍淇濈暀
        鍐呭瓨锛屼互搴斿宕╂簝鎯呭喌銆?
- 浣庝綅鍐呭瓨锛堝惎鍔ㄥ唴瀛橈級鍖哄煙琚繚瀛樺悗锛屽浐浠跺皢澶嶄綅 PCI 鍙婂叾瀹冪‖浠剁姸鎬併€傚畠**涓嶄細**娓呴櫎 RAM銆?  闅忓悗瀹冧細鍍忓钩甯镐竴鏍峰惎鍔ㄥ紩瀵煎姞杞界▼搴忋€?
- 鍏ㄦ柊鍚姩鐨勫唴鏍镐細娉ㄦ剰鍒拌澶囨爲涓湁涓€涓柊鑺傜偣锛坧Series 涓婁负 rtas/ibm,kernel-dump锛?  鎴栧湪 OPAL 骞冲彴涓婄殑 ibm,opal/dump/mpipl-boot锛夛紝琛ㄦ槑瀛樺湪鏉ヨ嚜涓婁竴娆″惎鍔ㄧ殑宕╂簝鏁版嵁銆?  鍦ㄦ棭鏈熷惎鍔ㄦ湡闂达紝OS 灏嗕繚鐣欏惎鍔ㄥ唴瀛樺ぇ灏忎互涓婄殑鍏朵綑鍐呭瓨锛屼粠鑰屾湁鏁堝湴浠ュ彈闄愬唴瀛樺ぇ灏忓惎鍔ㄣ€?  杩欏皢纭繚璇ュ唴鏍革紙涔熺О涓虹浜屼釜鍐呮牳鎴栨崟鑾峰唴鏍革級涓嶄細瑙︾浠讳綍杞偍鍐呭瓨鍖哄煙銆?
- 鐢ㄦ埛绌洪棿宸ュ叿灏嗚鍙?/proc/vmcore 浠ヨ幏鍙栧唴瀛樺唴瀹癸紝鍏朵腑浠ュ墠宕╂簝鍐呮牳鐨勮浆鍌ㄤ互 ELF 鏍煎紡淇濆瓨銆?  鐢ㄦ埛绌洪棿宸ュ叿鍙寜闇€灏嗘淇℃伅澶嶅埗鍒扮鐩樸€佺綉缁溿€乶as銆乻an銆乮scsi 绛夈€?
- 涓€鏃︾敤鎴风┖闂村伐鍏峰畬鎴愯浆鍌ㄤ繚瀛橈紝瀹冧細鍚?/sys/kernel/fadump_release_mem 鍐欏叆 '1'锛?  灏嗕繚鐣欑殑鍐呭瓨閲婃斁鍥炰竴鑸娇鐢紝淇濈暀涓嬩竴娆″浐浠惰緟鍔╄浆鍌ㄦ敞鍐屾墍闇€鐨勫唴瀛橀櫎澶栥€?
```

     # echo 1 > /sys/kernel/fadump_release_mem

```
璇锋敞鎰忥紝鍥轰欢杈呭姪杞偍鐗规€т粎鍦?pSeries锛圥owerVM锛夊钩鍙颁笂鐨?POWER6 鍙婃洿楂樼郴缁燂紝
浠ュ強 PowerNV锛圤PAL锛夊钩鍙颁笂 OP940 鎴栨洿楂樺浐浠剁増鏈殑 POWER9 鍙婃洿楂樼郴缁熶笂鍙敤銆?娉ㄦ剰锛屽綋 PowerNV 骞冲彴鏀寔 FADump 鏃讹紝OPAL 鍥轰欢浼氬鍑?ibm,opal/dump 鑺傜偣銆?
鍦ㄥ熀浜?OPAL 鐨勬満鍣ㄤ笂锛岀郴缁熶細鍦ㄥ惎鍔ㄥ埌鎹曡幏鍐呮牳涔嬪墠鍏堝惎鍔ㄤ竴涓腑闂村唴鏍?锛堢О涓?petitboot 鍐呮牳锛夈€傝鍐呮牳鍏锋湁鏈€灏忕殑鍐呮牳鍜?鎴栫敤鎴风┖闂存敮鎸佹潵澶勭悊宕╂簝鏁版嵁銆?杩欐牱鐨勫唴鏍搁渶瑕佷负鍚庣画鐨勬崟鑾峰唴鏍稿惎鍔ㄤ繚鐣欏厛鍓嶅穿婧冨唴鏍哥殑鍐呭瓨浠ュ鐞嗘宕╂簝鏁版嵁銆?蹇呴』鍦ㄦ绫诲唴鏍镐笂鍚敤鍐呮牳閰嶇疆閫夐」 CONFIG_PRESERVE_FA_DUMP锛屼互纭繚宕╂簝鏁版嵁琚繚鐣欎緵鍚庣画澶勭悊銆?
-- 鍦ㄥ熀浜?OPAL 鐨勬満鍣紙PowerNV锛変笂锛屽鏋滃唴鏍镐互 CONFIG_OPAL_CORE=y 鏋勫缓锛?宕╂簝鏃剁殑 OPAL 鍐呭瓨涔熶細浣滀负 /sys/firmware/opal/mpipl/core 鏂囦欢瀵煎嚭銆傛 procfs 鏂囦欢
鏈夊姪浜庣敤 GDB 璋冭瘯 OPAL 宕╂簝銆傜敤浜庡鍑烘 procfs 鏂囦欢鐨勫唴鏍稿唴瀛樺彲閫氳繃鍚?/sys/firmware/opal/mpipl/release_core 鑺傜偣鍐欏叆 '1' 鏉ラ噴鏀俱€?
   e.g.
     # echo 1 > /sys/firmware/opal/mpipl/release_core

-- Fadump 涓檮鍔犲唴鏍稿弬鏁扮殑鏀寔
   Fadump 鏈変竴椤圭壒鎬э紝鍏佽鍚?fadump 鍐呮牳浼犻€掗檮鍔犵殑鍐呮牳鍙傛暟銆傝鐗规€т富瑕佽璁＄敤浜?   绂佺敤 fadump 鍐呮牳涓嶉渶瑕佺殑鍐呮牳鍔熻兘锛屽苟鍦ㄦ敹闆嗚浆鍌ㄦ椂鍑忓皯鍏跺唴瀛樺崰鐢ㄣ€?
  鍚?Fadump 娣诲姞闄勫姞鍐呮牳鍙傛暟鐨勫懡浠わ細
  e.g.
  # echo "nr_cpus=16" > /sys/kernel/fadump/bootargs_append

  涓婅堪鍛戒护瓒充互鍚?fadump 娣诲姞闄勫姞鍙傛暟銆備笉闇€瑕佹樉寮忛噸鍚湇鍔°€?
  妫€绱㈤檮鍔?Fadump 鍙傛暟鐨勫懡浠わ細
  e.g.
  # cat /sys/kernel/fadump/bootargs_append

娉ㄦ剰锛氫娇鐢?HASH MMU 鐨?fadump 闄勫姞鍐呮牳鍙傛暟浠呭湪 RMA 澶у皬澶т簬 768 MB 鏃跺彈鏀寔銆?濡傛灉 RMA 澶у皬灏忎簬 768 MB锛屽唴鏍镐笉浼氬鍑?/sys/kernel/fadump/bootargs_append sysfs 鑺傜偣銆?
### 瀹炵幇缁嗚妭锛?
鍦ㄥ惎鍔ㄦ湡闂达紝浼氭鏌ュ浐浠舵槸鍚﹀湪璇ョ壒瀹氭満鍣ㄤ笂鏀寔姝ょ壒鎬с€傚鏋滄敮鎸侊紝鍒欐鏌ユ槸鍚︽湁
绛夊緟澶勭悊鐨勬椿璺冭浆鍌ㄣ€傚鏋滄湁锛屽垯鍦ㄦ棭鏈熷惎鍔ㄦ湡闂翠繚鐣欓櫎鍚姩鍐呭瓨澶у皬浠ュ鐨勫叏閮?RAM
锛堝弬瑙佸浘 2锛夈€備竴鏃︽垜浠畬鎴愪粠鐢ㄦ埛绌洪棿鑴氭湰锛堜緥濡?kdump 鑴氭湰锛夋敹闆嗚浆鍌紝璇ュ尯鍩熷嵆琚噴鏀俱€?濡傛灉鏈夎浆鍌ㄦ暟鎹紝鍒欎細鍒涘缓 /sys/kernel/fadump_release_mem 鏂囦欢锛屽苟鎸佹湁淇濈暀鐨勫唴瀛樸€?
濡傛灉娌℃湁绛夊緟澶勭悊鐨勮浆鍌ㄦ暟鎹紝鍒欓€氬父浠呭湪澶т簬鍚姩鍐呭瓨澶у皬鐨勫亸绉诲淇濈暀鐢ㄤ簬淇濆瓨 CPU 鐘舵€併€?HPTE 鍖哄煙銆佸惎鍔ㄥ唴瀛樿浆鍌ㄤ互鍙?FADump 澶寸殑鍐呭瓨锛堝弬瑙佸浘 1锛夈€傝鍖哄煙**涓嶄細**琚噴鏀撅細
姝ゅ尯鍩熷皢姘镐箙淇濈暀锛屼互渚垮湪涓嶅彂鐢熷穿婧冪殑姝ｅ父鎯呭喌涓嬶紝瀹冨彲浣滀负寮曞鍐呭瓨鍐呭鍓湰鐨勬帴鏀跺鍣紝
姝ゅ杩樺绾?CPU 鐘舵€佷笌 HPTE 鍖哄煙銆?
鐢变簬姝や繚鐣欏唴瀛樺尯鍩熶粎鍦ㄧ郴缁熷穿婧冨悗鎵嶈浣跨敤锛屽皢杩欎竴澶у潡鍐呭瓨浠庣敓浜у唴鏍镐腑闅旂鍑烘潵娌℃湁鎰忎箟銆?鍥犳锛屽鏋滃唴鏍搁厤缃簡 CMA锛屽疄鐜颁娇鐢?Linux 鍐呮牳鐨勮繛缁唴瀛樺垎閰嶅櫒锛圕MA锛夋潵杩涜鍐呭瓨淇濈暀銆?閫氳繃 CMA 淇濈暀锛屾鍐呭瓨鍙緵搴旂敤绋嬪簭浣跨敤锛屽悓鏃跺唴鏍歌闃绘浣跨敤瀹冦€傚€熷姪 FADump锛屼粛灏嗚兘澶?鎹曡幏鍏ㄩ儴鍐呮牳鍐呭瓨浠ュ強澶ч儴鍒嗙敤鎴风┖闂村唴瀛橈紝浣嗙敤鎴烽〉闄ゅ
```

  o Memory Reservation during first kernel

  Low memory                                                  Top of memory
  0    boot memory size   |<------ Reserved dump area ----->|     |
  |           |           |      Permanent Reservation      |     |
  V           V           |                                 |     V
  +-----------+-----/ /---+---+----+-----------+-------+----+-----+
  |           |           |///|////|    DUMP   |  HDR  |////|     |
  +-----------+-----/ /---+---+----+-----------+-------+----+-----+
        |                   ^    ^       ^         ^      ^
        |                   |    |       |         |      |
        \                  CPU  HPTE     /         |      |
         --------------------------------          |      |
      Boot memory content gets transferred         |      |
      to reserved area by firmware at the          |      |
      time of crash.                               |      |
                                           FADump Header  |
                                            (meta area)   |
                                                          |
                                                          |
                      Metadata: This area holds a metadata structure whose
                      address is registered with f/w and retrieved in the
                      second kernel after crash, on platforms that support
                      tags (OPAL). Having such structure with info needed
                      to process the crashdump eases dump capture process.

                   Fig. 1


  o Memory Reservation during second kernel after crash

  Low memory                                              Top of memory
  0      boot memory size                                      |
  |           |<------------ Crash preserved area ------------>|
  V           V           |<--- Reserved dump area --->|       |
  +----+---+--+-----/ /---+---+----+-------+-----+-----+-------+
  |    |ELF|  |           |///|////|  DUMP | HDR |/////|       |
  +----+---+--+-----/ /---+---+----+-------+-----+-----+-------+
       |   |  |                            |     |             |
       -----  ------------------------------     ---------------
         \              |                               |
           \            |                               |
             \          |                               |
               \        |    ----------------------------
                 \      |   /
                   \    |  /
                     \  | /
                  /proc/vmcore


        +---+
        |///| -> Regions (CPU, HPTE & Metadata) marked like this in the above
        +---+    figures are not always present. For example, OPAL platform
                 does not have CPU & HPTE regions while Metadata region is
                 not supported on pSeries currently.

        +---+
        |ELF| -> elfcorehdr, it is created in second kernel after crash.
        +---+

        Note: Memory from 0 to the boot memory size is used by second kernel

                   Fig. 2


```
褰撳墠锛岃浆鍌ㄥ皢鍦ㄧ敤鎴峰共棰勪笅浠?/proc/vmcore 澶嶅埗鍒版柊鏂囦欢銆傞€氳繃 /proc/vmcore 鍙敤鐨勮浆鍌ㄦ暟鎹?灏嗕负 ELF 鏍煎紡銆傚洜姝わ紝缁忚交寰慨鏀瑰悗锛岀幇鏈夌殑 kdump 鍩虹璁炬柦锛坘dump 鑴氭湰锛夌敤浜庝繚瀛樿浆鍌ㄥ嵆鍙甯稿伐浣溿€?涓绘祦鍙戣鐗堜笂鐨?KDump 鑴氭湰宸茶淇敼锛屼互鍦ㄥ皢 FADump 鐢ㄤ綔杞偍鏈哄埗锛堣€岄潪 KDump锛夋椂鏃犵紳宸ヤ綔
锛堜繚瀛樿浆鍌ㄦ棤闇€鐢ㄦ埛骞查锛夈€?
鐢ㄤ簬妫€鏌ヨ浆鍌ㄧ殑宸ュ叿灏嗕笌鐢ㄤ簬 kdump 鐨勭浉鍚屻€?
### 濡備綍鍚敤鍥轰欢杈呭姪杞偍锛團ADump锛夛細

1. 璁剧疆閰嶇疆閫夐」 CONFIG_FA_DUMP=y 骞舵瀯寤哄唴鏍搞€?2. 浠?'fadump=on' 鍐呮牳鍛戒护琛岄€夐」鍚姩杩涘叆 Linux 鍐呮牳銆?   榛樿鎯呭喌涓嬶紝FADump 淇濈暀鍐呭瓨灏嗚鍒濆鍖栦负 CMA 鍖哄煙銆?   鎴栬€咃紝鐢ㄦ埛鍙互浠?'fadump=nocma' 鍚姩 Linux 鍐呮牳锛屼互闃叉 FADump 浣跨敤 CMA銆?3. 鐢ㄦ埛杩樺彲浠ュ彲閫夊湴璁剧疆 'crashkernel=' 鍐呮牳鍛戒护琛岋紝浠ユ寚瀹氫负鍚姩鍐呭瓨杞偍淇濈暀鑰屼繚鐣欑殑鍐呭瓨澶у皬銆?
娉ㄦ剰锛?     1. 'fadump_reserve_mem=' 鍙傛暟宸茶寮冪敤銆傝鏀圭敤 'crashkernel=' 鎸囧畾涓哄惎鍔ㄥ唴瀛樿浆鍌ㄤ繚鐣欑殑
        鍐呭瓨澶у皬銆?     2. 濡傛灉鍥轰欢杈呭姪杞偍鏃犳硶淇濈暀鍐呭瓨锛岄偅涔堣嫢鍐呮牳鍛戒护琛岃缃簡 'crashkernel=' 閫夐」锛?        瀹冨皢鍥為€€鍒扮幇鏈夌殑 kdump 鏈哄埗銆?     3. 濡傛灉鐢ㄦ埛甯屾湜鎹曡幏鍏ㄩ儴鐢ㄦ埛绌洪棿鍐呭瓨锛屼笖鍙互鎺ュ彈淇濈暀鍐呭瓨瀵圭敓浜х郴缁熶笉鍙敤锛屽垯鍙互浣跨敤
        'fadump=nocma' 鍐呮牳鍙傛暟鍥為€€鍒版棫鐨勮涓恒€?
### sysfs/debugfs 鏂囦欢锛?
鍥轰欢杈呭姪杞偍鐗规€т娇鐢?sysfs 鏂囦欢绯荤粺淇濆瓨鎺у埗鏂囦欢锛屽苟浣跨敤 debugfs 鏂囦欢鏄剧ず淇濈暀鐨勫唴瀛樺尯鍩熴€?
浠ヤ笅鏄唴鏍?sysfs 涓嬬殑鏂囦欢鍒楄〃锛?
 /sys/kernel/fadump_enabled
    姝ゆ枃浠剁敤浜庢樉绀?FADump 鐘舵€併€?
    - 0 = FADump 宸茬鐢?    - 1 = FADump 宸插惎鐢?
    姝ゆ帴鍙ｅ彲琚?kdump init 鑴氭湰鐢ㄦ潵璇嗗埆鍐呮牳涓槸鍚﹀惎鐢ㄤ簡 FADump锛屽苟鎹閲囧彇琛屽姩銆?
 /sys/kernel/fadump_registered
    姝ゆ枃浠剁敤浜庢樉绀?FADump 娉ㄥ唽鐘舵€侊紝浠ュ強鎺у埗锛堝惎鍔?鍋滄锛塅ADump 娉ㄥ唽銆?
    - 0 = FADump 鏈敞鍐屻€?    - 1 = FADump 宸叉敞鍐岋紝骞跺噯澶囧ソ澶勭悊绯荤粺宕╂簝銆?
    瑕佹敞鍐?FADump锛屽啓鍏?echo 1 > /sys/kernel/fadump_registered锛涜娉ㄩ攢骞跺仠姝?FADump锛?    鍐欏叆 echo 0 > /sys/kernel/fadump_registered銆備竴鏃?FADump 琚敞閿€锛岀郴缁熷穿婧冨皢涓嶄細琚鐞嗭紝
    涔熶笉浼氭崟鑾?vmcore銆傛鎺ュ彛鍙交鏉句笌 kdump 鏈嶅姟鐨勫惎鍔?鍋滄闆嗘垚銆?
 /sys/kernel/fadump/mem_reserved

   姝ゆ枃浠剁敤浜庢樉绀?FADump 涓轰繚瀛樺穿婧冭浆鍌ㄨ€屼繚鐣欑殑鍐呭瓨銆?
 /sys/kernel/fadump_release_mem
    姝ゆ枃浠朵粎鍦ㄧ浜屼釜鍐呮牳鏈熼棿 FADump 澶勪簬娲昏穬鐘舵€佹椂鍙敤銆傚畠鐢ㄤ簬閲婃斁涓轰繚瀛樺穿婧冭浆鍌ㄨ€屾寔鏈夌殑
    淇濈暀鍐呭瓨鍖哄煙銆傝閲婃斁
```

	echo 1  > /sys/kernel/fadump_release_mem

    鍦?echo 1 涔嬪悗锛?sys/kernel/debug/powerpc/fadump_region 鏂囦欢鐨勫唴瀹瑰皢鏀瑰彉浠ュ弽鏄犳柊鐨?    鍐呭瓨淇濈暀銆?
    鐜版湁鐨勭敤鎴风┖闂村伐鍏凤紙kdump 鍩虹璁炬柦锛夊彲杞绘澗澧炲己锛屼互浣跨敤姝ゆ帴鍙ｉ噴鏀句负杞偍淇濈暀鐨勫唴瀛橈紝
    骞跺湪鏃犻渶绗簩娆￠噸鍚殑鎯呭喌涓嬬户缁€?
```
娉ㄦ剰锛?sys/kernel/fadump_release_opalcore sysfs 宸茬Щ鑷?      /sys/firmware/opal/mpipl/release_core

 /sys/firmware/opal/mpipl/release_core

    姝ゆ枃浠朵粎鍦ㄥ熀浜?OPAL 鐨勬満鍣ㄤ笂銆佹崟鑾峰唴鏍告湡闂?FADump 澶勪簬娲昏穬鐘舵€佹椂鍙敤銆傚畠鐢ㄤ簬閲婃斁
    鍐呮牳鐢ㄤ簬瀵煎嚭 /sys/firmware/opal/mpipl/core 鏂囦欢鐨勫唴瀛樸€傝閲婃斁姝ゅ唴瀛橈紝鍚戝叾鍐欏叆 '1'锛?
    echo 1  > /sys/firmware/opal/mpipl/release_core

娉ㄦ剰锛氫互涓?FADump sysfs 鏂囦欢宸茶寮冪敤銆?
+----------------------------------+--------------------------------+
| Deprecated                       | Alternative                    |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_enabled       | /sys/kernel/fadump/enabled     |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_registered    | /sys/kernel/fadump/registered  |
+----------------------------------+--------------------------------+
| /sys/kernel/fadump_release_mem   | /sys/kernel/fadump/release_mem |
+----------------------------------+--------------------------------+

浠ヤ笅鏄?powerpc debugfs 涓嬬殑鏂囦欢鍒楄〃锛?锛堝亣瀹?debugfs 鎸傝浇鍦?/sys/kernel/debug 鐩綍涓嬨€傦級

 /sys/kernel/debug/powerpc/fadump_region
    濡傛灉鍚敤浜?FADump锛屾鏂囦欢鏄剧ず淇濈暀鐨勫唴瀛樺尯鍩燂紝鍚﹀垯姝ゆ枃浠朵负绌恒€傝緭鍑烘牸寮?```

      <region>: [<start>-<end>] <reserved-size> bytes, Dumped: <dump-size>

    鑰屽唴鏍?DUMP 鍖哄煙鐨勬牸寮忎负锛?
    DUMP: Src: <src-addr>, Dest: <dest-addr>, Size: <size>, Dumped: # bytes

    e.g.
    Contents when FADump is registered during first kernel::

      # cat /sys/kernel/debug/powerpc/fadump_region
      CPU : [0x0000006ffb0000-0x0000006fff001f] 0x40020 bytes, Dumped: 0x0
      HPTE: [0x0000006fff0020-0x0000006fff101f] 0x1000 bytes, Dumped: 0x0
      DUMP: [0x0000006fff1020-0x0000007fff101f] 0x10000000 bytes, Dumped: 0x0

    Contents when FADump is active during second kernel::

      # cat /sys/kernel/debug/powerpc/fadump_region
      CPU : [0x0000006ffb0000-0x0000006fff001f] 0x40020 bytes, Dumped: 0x40020
      HPTE: [0x0000006fff0020-0x0000006fff101f] 0x1000 bytes, Dumped: 0x1000
      DUMP: [0x0000006fff1020-0x0000007fff101f] 0x10000000 bytes, Dumped: 0x10000000
          : [0x00000010000000-0x0000006ffaffff] 0x5ffb0000 bytes, Dumped: 0x5ffb0000


```
娉ㄦ剰锛?      鍏充簬濡備綍鎸傝浇 debugfs 鏂囦欢绯荤粺锛岃鍙傞槄 Documentation/filesystems/debugfs.rst銆?

### 寰呭姙锛?
 - 闇€瑕佹彁鍑烘洿濂界殑鏂规硶锛屼互鎵惧嚭鍦ㄥ彈闄愬唴瀛樹笅鎴愬姛鍚姩鍐呮牳鎵€闇€鐨勬洿鍑嗙‘鐨勫惎鍔ㄥ唴瀛樺ぇ灏忋€?
浣滆€咃細Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>

鏈枃妗ｅ熀浜?Linas Vepstas 鍜?Manish Ahuja 涓?phyp 杈呭姪杞偍鎵€鍐欑殑鍘熷鏂囨。銆?