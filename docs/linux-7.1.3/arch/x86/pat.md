
## PAT锛堥〉灞炴€ц〃锛?

x86 鐨勯〉灞炴€ц〃锛圥age Attribute Table锛孭AT锛夊厑璁稿湪椤电矑搴︿笂璁剧疆鍐呭瓨灞炴€с€?
PAT 鏄 MTRR 璁剧疆鐨勮ˉ鍏咃紝MTRR 鐢ㄤ簬鍦ㄧ墿鐞嗗湴鍧€鑼冨洿涓婅缃唴瀛樼被鍨嬨€傜劧鑰岋紝
PAT 姣?MTRR 鏇寸伒娲伙紝鍥犱负瀹冭兘澶熷湪椤电骇鍒缃睘鎬э紝鑰屼笖姝ょ被灞炴€ц缃殑鏁伴噺
娌℃湁纭欢闄愬埗銆傚鍔犵殑鐏垫椿鎬т即闅忕潃涓€浜涘噯鍒欙細瀵逛簬鍚屼竴鐗╃悊鍐呭瓨鐨勫涓櫄鎷?
鍦板潃锛屼笉搴斿嚭鐜板唴瀛樼被鍨嬪埆鍚嶃€?

PAT 鍏佽浣跨敤涓嶅悓绫诲瀷鐨勫唴瀛樺睘鎬с€傚綋鍓嶅皢鏀寔鐨勬渶甯哥敤绫诲瀷濡備笅锛?

===  ==============
WB   鍐欏洖
UC   涓嶇紦瀛?
WC   鍐欏悎骞?
WT   鍐欑洿杈?
UC-  涓嶇紦瀛樺噺鍨?
===  ==============


## PAT API

鍐呮牳涓湁璁稿涓嶅悓鐨?API 鍏佽鍦ㄩ〉绾у埆璁剧疆鍐呭瓨灞炴€с€備负浜嗛伩鍏嶅埆鍚嶏紝搴斿綋
璋ㄦ厧浣跨敤杩欎簺鎺ュ彛銆備笅闈㈡槸涓€寮犲彲鐢ㄦ帴鍙ｈ〃锛屽垪鍑轰簡瀹冧滑鐨勯鏈熺敤閫斿強鍏跺唴瀛?
灞炴€у叧绯汇€傚湪鍐呴儴锛岃繖浜?API 鍦ㄧ墿鐞嗗湴鍧€鑼冨洿涓婁娇鐢?
reserve_memtype()/free_memtype() 鎺ュ彛鏉ラ伩鍏嶄换浣曞埆鍚嶃€?

+------------------------+----------+--------------+------------------+
| API                    |    RAM   |  ACPI,...    |  Reserved/Holes  |
+------------------------+----------+--------------+------------------+
| ioremap                |    --    |    UC-       |       UC-        |
+------------------------+----------+--------------+------------------+
| ioremap_cache          |    --    |    WB        |       WB         |
+------------------------+----------+--------------+------------------+
| ioremap_uc             |    --    |    UC        |       UC         |
+------------------------+----------+--------------+------------------+
| ioremap_wc             |    --    |    --        |       WC         |
+------------------------+----------+--------------+------------------+
| ioremap_wt             |    --    |    --        |       WT         |
+------------------------+----------+--------------+------------------+
| set_memory_uc,         |    UC-   |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| set_memory_wc,         |    WC    |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| set_memory_wt,         |    WT    |    --        |       --         |
| set_memory_wb          |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci sysfs resource     |    --    |    --        |       UC-        |
+------------------------+----------+--------------+------------------+
| pci sysfs resource_wc  |    --    |    --        |       WC         |
| is IORESOURCE_PREFETCH |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci proc               |    --    |    --        |       UC-        |
| !PCIIOC_WRITE_COMBINE  |          |              |                  |
+------------------------+----------+--------------+------------------+
| pci proc               |    --    |    --        |       WC         |
| PCIIOC_WRITE_COMBINE   |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |   WB/WC/UC-  |    WB/WC/UC-     |
| read-write             |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    UC-       |       UC-        |
| mmap SYNC flag         |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |   WB/WC/UC-  |  WB/WC/UC-       |
| mmap !SYNC flag        |          |              |                  |
| and                    |          |(from existing|  (from existing  |
| any alias to this area |          |alias)        |  alias)          |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    WB        |       WB         |
| mmap !SYNC flag        |          |              |                  |
| no alias to this area  |          |              |                  |
| and                    |          |              |                  |
| MTRR says WB           |          |              |                  |
+------------------------+----------+--------------+------------------+
| /dev/mem               |    --    |    --        |       UC-        |
| mmap !SYNC flag        |          |              |                  |
| no alias to this area  |          |              |                  |
| and                    |          |              |                  |
| MTRR says !WB          |          |              |                  |
+------------------------+----------+--------------+------------------+


## 闈㈠悜椹卞姩鐨勯珮绾?API


A. 浣跨敤 remap_pfn_range銆乮o_remap_pfn_range銆乿mf_insert_pfn 鍚戠敤鎴峰鍑洪〉銆?

甯屾湜鍚戠敤鎴风┖闂村鍑烘煇浜涢〉鐨勯┍鍔紝閫氳繃 mmap 鎺ュ彛浠ュ強浠ヤ笅缁勫悎鏉ュ疄鐜帮細

  1) pgprot_noncached()
  2) io_remap_pfn_range() 鎴?remap_pfn_range() 鎴?vmf_insert_pfn()

鍊熷姪 PAT 鏀寔锛屾鍦ㄦ柊澧炰竴涓?API pgprot_writecombine銆傚洜姝わ紝椹卞姩鍙互缁х画
浣跨敤涓婅堪搴忓垪锛屽湪绗?1 姝ヤ娇鐢?pgprot_noncached() 鎴?pgprot_writecombine()锛?
鐒跺悗鎵ц绗?2 姝ャ€?

姝ゅ锛岀 2 姝ヤ細鍦ㄥ唴閮ㄥ皢璇ュ尯鍩熶綔涓?UC 鎴?WC 鍦?memtype 鍒楄〃涓繘琛岃拷韪紝
浠ョ‘淇濅笉浼氬嚭鐜板啿绐佺殑鏄犲皠銆?

娉ㄦ剰锛岃繖缁?API 浠呴€傜敤浜?IO锛堥潪 RAM锛夊尯鍩熴€傚鏋滈┍鍔ㄦ兂瑕佸鍑?RAM 鍖哄煙锛?
鍒欏繀椤诲涓婇潰鐨勭 0 姝ラ偅鏍锋墽琛?set_memory_uc() 鎴?set_memory_wc()锛屽苟涓?
杩樿杩借釜杩欎簺椤电殑浣跨敤鎯呭喌锛屽苟鍦ㄨ椤甸噴鏀惧洖绌洪棽姹犱箣鍓嶄娇鐢?set_memory_wb()銆?

## MTRR 瀵?PAT / 闈?PAT 绯荤粺鐨勫奖鍝?


涓嬭〃璇存槑浜嗗湪 x86 涓婂悓鏃朵娇鐢?ioremap*() 璋冪敤鏃讹紝浣跨敤鍐欏悎骞跺瀷 MTRR 瀵?
闈?PAT 鍜?PAT 绯荤粺鐨勫奖鍝嶃€傜悊鎯虫儏鍐典笅锛宮trr_add() 鐨勪娇鐢ㄥ皢琚€愭娣樻卑锛?
杞€屼娇鐢?arch_phys_wc_add()锛屽悗鑰呭湪鍚敤 PAT 鐨勭郴缁熶笂涓虹┖鎿嶄綔銆傛墽琛?
arch_phys_wc_add() 鐨勫尯鍩熷簲褰撳凡缁忛€氳繃 WC 灞炴€ф垨 PAT 琛ㄩ」杩涜浜?ioremap锛?
杩欏彲浠ラ€氳繃浣跨敤 ioremap_wc() / set_memory_wc() 鏉ュ畬鎴愩€傚浜庡笇鏈涘皢闇€瑕佷繚鎸?
涓嶅彲缂撳瓨鐨?IO 鍐呭瓨鍖哄煙涓庨€傚悎鍐欏悎骞剁殑鍖哄煙缁勫悎鍦ㄤ竴璧风殑璁惧锛屽簲鑰冭檻浣跨敤
ioremap_uc() 鍚庢帴 set_memory_wc() 鏉ュ皢鏈夋晥鐨勫啓鍚堝苟鍖哄煙鍔犲叆鐧藉悕鍗曘€備笉杩囷紝
杩欑鐢ㄦ硶浠嶇劧涓嶈榧撳姳锛屽洜涓烘湁鏁堢殑鍐呭瓨绫诲瀷琚涓哄疄鐜板畾涔夌殑锛屼絾姝ょ瓥鐣ュ彲
浣滀负鏈€鍚庢墜娈电敤浜庨偅浜涚┖闂村彈闄愩€佸惁鍒?MTRR 鍐欏悎骞跺皢涓嶈捣浣滅敤鐨勮澶囥€?
```

  ====  =======  ===  =========================  =====================
  MTRR  Non-PAT  PAT  Linux ioremap value        Effective memory type
  ====  =======  ===  =========================  =====================
        PAT                                        Non-PAT |  PAT
        |PCD                                               |
        ||PWT                                              |
        |||                                                |
  WC    000      WB   _PAGE_CACHE_MODE_WB             WC   |   WC
  WC    001      WC   _PAGE_CACHE_MODE_WC             WC*  |   WC
  WC    010      UC-  _PAGE_CACHE_MODE_UC_MINUS       WC*  |   UC
  WC    011      UC   _PAGE_CACHE_MODE_UC             UC   |   UC
  ====  =======  ===  =========================  =====================

  (*) denotes implementation defined and is discouraged

```
  鍏朵腑鐨?-- 鐢卞唴鏍镐弗鏍兼墽琛屻€傚叾浠栦竴浜涚洰鍓嶅苟鏈湡姝ｅ己鍒舵墽琛岋紝浣嗗皢鏉?
  鍙兘浼氬己鍒舵墽琛屻€?

瀵逛簬 ioremap 浠ュ強閫氳繃 /sys 鎴?/proc 杩涜鐨?PCI 璁块棶鈥斺€斿湪鏌愪簺鎯呭喌涓嬶紝濡傛灉
璇ュ湴鍧€瀛樺湪浠讳綍宸叉湁鐨勫埆鍚嶏紝杩斿洖鐨勫疄闄呯被鍨嬪彲鑳戒細鏇翠弗鏍笺€備緥濡傦細濡傛灉宸插瓨鍦?
涓€涓笉鍙紦瀛樼殑鏄犲皠锛岄偅涔堟柊鐨?ioremap_wc 鍙兘浼氳繑鍥炰笉鍙紦瀛樼殑鏄犲皠锛岃€岄潪
鎵€璇锋眰鐨勫啓鍚堝苟銆?

set_memory_[uc|wc|wt] 鍜?set_memory_wb 搴斿綋鎴愬浣跨敤锛岄┍鍔ㄩ鍏堝皢鏌愬尯鍩?
璁句负 uc銆亀c 鎴?wt锛屼娇鐢ㄥ悗鍐嶅皢鍏跺垏鎹㈠洖 wb銆?

闅忕潃鏃堕棿鐨勬帹绉伙紝瀵?/proc/mtrr 鐨勫啓鍏ュ皢琚簾寮冿紝杞€屼娇鐢ㄥ熀浜?PAT 鐨勬帴鍙ｃ€?
寤鸿鍐欏叆 /proc/mtrr 鐨勭敤鎴蜂娇鐢ㄤ笂杩版帴鍙ｃ€?

椹卞姩搴斿綋浣跨敤 ioremap_[uc|wc] 鏉ヨ闂叿鏈?[uc|wc] 璁块棶绫诲瀷鐨?PCI BAR銆?

椹卞姩搴斿綋浣跨敤 set_memory_[uc|wc|wt] 鏉ヨ缃?RAM 鑼冨洿鐨勮闂被鍨嬨€?


## PAT 璋冭瘯


```

  # mount -t debugfs debugfs /sys/kernel/debug
  # cat /sys/kernel/debug/x86/pat_memtype_list
  PAT memtype list:
  uncached-minus @ 0x7fadf000-0x7fae0000
  uncached-minus @ 0x7fb19000-0x7fb1a000
  uncached-minus @ 0x7fb1a000-0x7fb1b000
  uncached-minus @ 0x7fb1b000-0x7fb1c000
  uncached-minus @ 0x7fb1c000-0x7fb1d000
  uncached-minus @ 0x7fb1d000-0x7fb1e000
  uncached-minus @ 0x7fb1e000-0x7fb25000
  uncached-minus @ 0x7fb25000-0x7fb26000
  uncached-minus @ 0x7fb26000-0x7fb27000
  uncached-minus @ 0x7fb27000-0x7fb28000
  uncached-minus @ 0x7fb28000-0x7fb2e000
  uncached-minus @ 0x7fb2e000-0x7fb2f000
  uncached-minus @ 0x7fb2f000-0x7fb30000
  uncached-minus @ 0x7fb31000-0x7fb32000
  uncached-minus @ 0x80000000-0x90000000

```
姝ゅ垪琛ㄦ樉绀轰簡鐗╃悊鍦板潃鑼冨洿浠ュ強鐢ㄤ簬璁块棶杩欎簺鐗╃悊鍦板潃鑼冨洿鐨勫悇绉?PAT 璁剧疆銆?

鍙︿竴绉嶆洿璇︾粏鐨勮幏鍙?PAT 鐩稿叧璋冭瘯娑堟伅鐨勬柟寮忔槸浣跨敤 "debugpat" 寮曞鍙傛暟銆?
浣跨敤璇ュ弬鏁板悗锛屽悇绉嶈皟璇曟秷鎭細琚墦鍗板埌 dmesg 鏃ュ織涓€?

## PAT 鍒濆鍖?


涓嬭〃鎻忚堪浜嗗湪鍚勭閰嶇疆涓?PAT 濡備綍琚垵濮嬪寲銆侾AT MSR 蹇呴』鐢?Linux 鏇存柊锛?
浠ユ敮鎸?WC 鍜?WT 灞炴€с€傚惁鍒欙紝PAT MSR 涓繚瀛樼殑鏄浐浠跺啓鍏ュ叾涓殑鍊笺€傛敞鎰忥紝
Xen 鍦ㄥ鎴锋満鐨?PAT MSR 涓惎鐢ㄤ簡 WC 灞炴€с€?

 ==== ===== ==========================  =========  =======
 MTRR PAT   Call Sequence               PAT State  PAT MSR
 ==== ===== ==========================  =========  =======
 E    E     MTRR -> PAT init            Enabled    OS
 E    D     MTRR -> PAT init            Disabled    -
 D    E     MTRR -> PAT disable         Disabled   BIOS
 D    D     MTRR -> PAT disable         Disabled    -
 - np/E  PAT  -> PAT disable         Disabled   BIOS
 - np/D  PAT  -> PAT disable         Disabled    -
 E    !P/E  MTRR -> PAT init            Disabled   BIOS
 D    !P/E  MTRR -> PAT disable         Disabled   BIOS
 !M   !P/E  MTRR stub -> PAT disable    Disabled   BIOS
 ==== ===== ==========================  =========  =======

  鍥句緥

 ========= =======================================
 E         CPU 涓惎鐢ㄧ殑鐗规€?
 D	       CPU 涓鐢?涓嶆敮鎸佺殑鐗规€?
 np	       鎸囧畾浜?"nopat" 寮曞閫夐」
 !P	       CONFIG_X86_PAT 閫夐」鏈缃?
 !M	       CONFIG_MTRR 閫夐」鏈缃?
 Enabled   PAT 鐘舵€佽涓哄凡鍚敤
 Disabled  PAT 鐘舵€佽涓哄凡绂佺敤
 OS        PAT 浣跨敤 OS 璁剧疆鍒濆鍖?PAT MSR
 BIOS      PAT 淇濇寔 PAT MSR 鐨?BIOS 璁剧疆
 ========= =======================================

